use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{env, fs};

use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer, run_retention};
use duckdb::Connection;
use ingest::channel::{IngestSender, build_channel};
use ingest::contract::{Error as IngestError, OtlpPort};
use ingest::state::IngestState;
use tracing_error::SpanTrace;
use ui_bridge::health::{
    BindStatus, BufferConnectionStatus, HealthApi, HealthApiImpl, HeartbeatState,
    IngestChannelStatus, record_start, register_heartbeat_state,
};
use viz::VizState;

mod heartbeat;
mod observability;
mod streams;
mod viz_routers;

use streams::{StreamsApi, StreamsApiImpl};
use viz_routers::{LogsApi, LogsApiImpl, MetricsApi, MetricsApiImpl, TracesApi, TracesApiImpl};

const ENV_OTLP_GRPC_PORT: &str = "ANDROMEDA_PULSE_OTLP_GRPC_PORT";
const ENV_OTLP_HTTP_PORT: &str = "ANDROMEDA_PULSE_OTLP_HTTP_PORT";
const ENV_RETENTION_SECONDS: &str = "ANDROMEDA_PULSE_RETENTION_SECONDS";

// Retention bounds per security plan §Input Validation row "Configuration values":
// reject out-of-range rather than silently clamping. Default fallback per arch
// §Inherited Defaults (300–600s default range) — chunk #21 picks 600 (upper).
const RETENTION_SECONDS_MIN: u64 = 60;
const RETENTION_SECONDS_MAX: u64 = 86_400;
const RETENTION_SECONDS_DEFAULT: u64 = 600;

fn resolve_retention_seconds() -> u64 {
    let raw = match env::var(ENV_RETENTION_SECONDS) {
        Ok(r) => r,
        Err(_) => return RETENTION_SECONDS_DEFAULT,
    };
    let parsed: u64 = match raw.parse::<u64>() {
        Ok(p) => p,
        Err(_) => {
            tracing::warn!(
                target: "config.load.retention_seconds",
                env_var_name = ENV_RETENTION_SECONDS,
                reject_reason = "unparseable",
                "retention seconds env var rejected; falling back to default",
            );
            return RETENTION_SECONDS_DEFAULT;
        }
    };
    if !(RETENTION_SECONDS_MIN..=RETENTION_SECONDS_MAX).contains(&parsed) {
        tracing::warn!(
            target: "config.load.retention_seconds",
            env_var_name = ENV_RETENTION_SECONDS,
            reject_reason = "out_of_range",
            "retention seconds env var rejected; falling back to default",
        );
        return RETENTION_SECONDS_DEFAULT;
    }
    parsed
}

fn resolve_grpc_port() -> Result<OtlpPort, IngestError> {
    resolve_port(ENV_OTLP_GRPC_PORT, ingest::grpc::DEFAULT_GRPC_PORT)
}

fn resolve_http_port() -> Result<OtlpPort, IngestError> {
    resolve_port(ENV_OTLP_HTTP_PORT, ingest::http::DEFAULT_HTTP_PORT)
}

fn resolve_port(env_var_name: &'static str, default: u16) -> Result<OtlpPort, IngestError> {
    let raw = match env::var(env_var_name) {
        Ok(r) => r,
        Err(_) => {
            return OtlpPort::try_from(default);
        }
    };
    let parsed: u16 = match raw.parse::<u16>() {
        Ok(p) => p,
        Err(_) => {
            tracing::warn!(
                target: "config.load.port_validation",
                env_var_name = env_var_name,
                reject_reason = "unparseable",
                "OTLP port env var rejected; receiver will not start",
            );
            return Err(IngestError::InvalidPort { value: raw });
        }
    };
    match OtlpPort::try_from(parsed) {
        Ok(p) => Ok(p),
        Err(e) => {
            tracing::warn!(
                target: "config.load.port_validation",
                env_var_name = env_var_name,
                reject_reason = "out_of_range",
                "OTLP port env var rejected; receiver will not start",
            );
            Err(e)
        }
    }
}

fn resolve_data_dir() -> PathBuf {
    if let Ok(p) = env::var("ANDROMEDA_PULSE_DATA_DIR") {
        return PathBuf::from(p);
    }
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata).join("andromeda-pulse");
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("com.andromeda.pulse");
        }
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("andromeda-pulse");
    } else if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".andromeda-pulse");
    }
    env::temp_dir().join("andromeda-pulse")
}

fn write_pid_file(data_dir: &Path) {
    let run_dir = data_dir.join("run");
    if let Err(e) = fs::create_dir_all(&run_dir) {
        tracing::warn!(target: "app.boot.pid", error = %e, "failed to create run dir");
        return;
    }
    let pid_path = run_dir.join("andromeda-pulse.pid");
    let canonical_data = match fs::canonicalize(data_dir) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(target: "app.boot.pid", error = %e, "data dir canonicalize failed");
            return;
        }
    };
    let canonical_run = match fs::canonicalize(&run_dir) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(target: "app.boot.pid", error = %e, "run dir canonicalize failed");
            return;
        }
    };
    if !canonical_run.starts_with(&canonical_data) {
        tracing::error!(
            target: "app.boot.pid",
            run_dir = ?canonical_run,
            data_dir = ?canonical_data,
            "run dir escaped data dir; refusing to write PID",
        );
        return;
    }
    let pid = std::process::id();
    if let Err(e) = fs::write(&pid_path, pid.to_string()) {
        tracing::warn!(target: "app.boot.pid", error = %e, "failed to write PID file");
        return;
    }
    tracing::info!(target: "app.boot.pid", pid = pid, path = ?pid_path, "PID file written");
}

fn main() {
    let data_dir = resolve_data_dir();
    let _guard = observability::init(&data_dir);
    record_start();
    write_pid_file(&data_dir);

    let heartbeat_state = Arc::new(HeartbeatState::new());
    register_heartbeat_state(heartbeat_state.clone());

    let ingest_state = Arc::new(IngestState::new());
    let (ingest_sender, ingest_receiver) = build_channel();
    let ingest_sender: Arc<IngestSender> = Arc::new(ingest_sender);
    heartbeat_state.record_ingest_channel(IngestChannelStatus::Ok { capacity_pct: 0.0 });

    let buffer_state = Arc::new(BufferState::new());
    let buffer_conn = init_buffer(&heartbeat_state);
    let retention_seconds = resolve_retention_seconds();
    let viz_state = Arc::new(VizState::new());
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());

    let grpc_addr = match resolve_grpc_port() {
        Ok(p) => Some(SocketAddr::from(([127, 0, 0, 1], p.value()))),
        Err(_) => {
            heartbeat_state.record_otlp_grpc_bind(BindStatus::Failed("invalid_port".to_string()));
            tracing::error!(
                target: "app.boot.otlp.grpc.bind",
                reason = "invalid_port",
                "OTLP gRPC port validation rejected env-var override; receiver will not start"
            );
            None
        }
    };
    let http_addr = match resolve_http_port() {
        Ok(p) => Some(SocketAddr::from(([127, 0, 0, 1], p.value()))),
        Err(_) => {
            heartbeat_state.record_otlp_http_bind(BindStatus::Failed("invalid_port".to_string()));
            tracing::error!(
                target: "app.boot.otlp.http.bind",
                reason = "invalid_port",
                "OTLP HTTP port validation rejected env-var override; receiver will not start"
            );
            None
        }
    };

    let invoke_router = match buffer_conn.as_ref() {
        Some(conn) => taurpc::Router::new()
            .merge(HealthApiImpl.into_handler())
            .merge(TracesApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
            .merge(MetricsApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
            .merge(LogsApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
            .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler()),
        None => taurpc::Router::new()
            .merge(HealthApiImpl.into_handler())
            .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(invoke_router.into_handler())
        .setup(move |_app| {
            match buffer_conn {
                Some(conn) => {
                    tauri::async_runtime::spawn(run_consumer(
                        ingest_receiver,
                        Arc::clone(&conn),
                        Arc::clone(&buffer_state),
                        Arc::clone(&broadcast_senders),
                    ));
                    tauri::async_runtime::spawn(run_retention(
                        Arc::clone(&conn),
                        Arc::clone(&buffer_state),
                        retention_seconds,
                    ));
                }
                None => {
                    // Buffer init failed; the health envelope is already degraded
                    // (BufferConnectionStatus::InitFailed recorded in init_buffer).
                    // Drain the receiver to keep the OTLP ingest path live so
                    // chunk #16/#17 receivers don't cascade into channel saturation.
                    // Retention task is also skipped — no connection to sweep.
                    tauri::async_runtime::spawn(async move {
                        let mut rx = ingest_receiver;
                        while rx.recv().await.is_some() {}
                    });
                }
            }
            if let Some(grpc_addr) = grpc_addr {
                let grpc_announcer = Arc::clone(&heartbeat_state);
                let grpc_state = Arc::clone(&ingest_state);
                let grpc_sender = Arc::clone(&ingest_sender);
                tauri::async_runtime::spawn(async move {
                    match ingest::grpc::try_bind(grpc_addr).await {
                        Ok(listener) => {
                            grpc_announcer.record_otlp_grpc_bind(BindStatus::Ok);
                            tracing::info!(
                                target: "app.boot.otlp.grpc.bind",
                                bind_address = %grpc_addr,
                                "OTLP gRPC receiver bound"
                            );
                            if let Err(e) =
                                ingest::grpc::serve_on(listener, grpc_state, grpc_sender).await
                            {
                                let reason = format!("{}", e);
                                grpc_announcer
                                    .record_otlp_grpc_bind(BindStatus::Failed(reason.clone()));
                                tracing::error!(
                                    target: "app.boot.otlp.grpc.bind",
                                    reason = %reason,
                                    bind_address = %grpc_addr,
                                    "OTLP gRPC server stopped"
                                );
                            }
                        }
                        Err(e) => {
                            let reason = format!("{}", e);
                            grpc_announcer
                                .record_otlp_grpc_bind(BindStatus::Failed(reason.clone()));
                            tracing::error!(
                                target: "app.boot.otlp.grpc.bind",
                                reason = %reason,
                                bind_address = %grpc_addr,
                                "bind failed"
                            );
                        }
                    }
                });
            }
            if let Some(http_addr) = http_addr {
                let http_announcer = Arc::clone(&heartbeat_state);
                let http_state = Arc::clone(&ingest_state);
                let http_sender = Arc::clone(&ingest_sender);
                tauri::async_runtime::spawn(async move {
                    match ingest::http::try_bind(http_addr).await {
                        Ok(listener) => {
                            http_announcer.record_otlp_http_bind(BindStatus::Ok);
                            tracing::info!(
                                target: "app.boot.otlp.http.bind",
                                bind_address = %http_addr,
                                "OTLP HTTP receiver bound"
                            );
                            if let Err(e) =
                                ingest::http::serve_on(listener, http_state, http_sender).await
                            {
                                let reason = format!("{}", e);
                                http_announcer
                                    .record_otlp_http_bind(BindStatus::Failed(reason.clone()));
                                tracing::error!(
                                    target: "app.boot.otlp.http.bind",
                                    reason = %reason,
                                    bind_address = %http_addr,
                                    "OTLP HTTP server stopped"
                                );
                            }
                        }
                        Err(e) => {
                            let reason = format!("{}", e);
                            http_announcer
                                .record_otlp_http_bind(BindStatus::Failed(reason.clone()));
                            tracing::error!(
                                target: "app.boot.otlp.http.bind",
                                reason = %reason,
                                bind_address = %http_addr,
                                "bind failed"
                            );
                        }
                    }
                });
            }
            let _heartbeat_handles = heartbeat::spawn(
                heartbeat_state,
                ingest_state,
                Arc::clone(&ingest_sender),
                Arc::clone(&buffer_state),
                retention_seconds,
                Arc::clone(&viz_state),
                Arc::clone(&broadcast_senders),
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_buffer(heartbeat_state: &Arc<HeartbeatState>) -> Option<Arc<Mutex<Connection>>> {
    let start = Instant::now();
    let conn = match Connection::open_in_memory() {
        Ok(c) => c,
        Err(_) => {
            heartbeat_state.record_buffer_connection(BufferConnectionStatus::InitFailed(
                "open_in_memory".into(),
            ));
            tracing::error!(
                target: "buffer.schema.init.error",
                error_type = "open_in_memory_failed",
                spantrace = ?SpanTrace::capture(),
                "DuckDB :memory: connection open failed",
            );
            return None;
        }
    };
    if create_schema(&conn).is_err() {
        heartbeat_state.record_buffer_connection(BufferConnectionStatus::InitFailed(
            "schema_create_failed".into(),
        ));
        tracing::error!(
            target: "buffer.schema.init.error",
            error_type = "schema_create",
            spantrace = ?SpanTrace::capture(),
            "DuckDB schema creation failed",
        );
        return None;
    }
    heartbeat_state.record_buffer_connection(BufferConnectionStatus::Ok);
    let duration_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        target: "buffer.schema.init",
        table_count = 7_u64,
        duration_ms = duration_ms,
        "ring buffer schema created",
    );
    Some(Arc::new(Mutex::new(conn)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // SAFETY: env::set_var / remove_var are unsafe in Rust 2024 edition because
    // they race with concurrent threads' env reads. cargo-nextest gives us
    // per-process test isolation (per .claude/rules/testing.md §Framework), and
    // each test uses a unique env-var name so there is no overlap with sibling
    // tests sharing the same process. Calls are scoped narrowly and the env
    // var is removed at end-of-test.

    #[test]
    fn resolve_port_unset_env_returns_default() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_UNSET";
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        let result = resolve_port(TEST_ENV, 4317);
        let port = result.expect("unset env returns default port");
        assert_eq!(port.value(), 4317);
    }

    #[test]
    fn resolve_port_unparseable_env_returns_invalid_port_err() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_UNPARSEABLE";
        unsafe {
            std::env::set_var(TEST_ENV, "not-a-number");
        }
        let result = resolve_port(TEST_ENV, 4317);
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
    }

    #[test]
    fn resolve_port_overflow_env_returns_invalid_port_err() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_OVERFLOW";
        unsafe {
            std::env::set_var(TEST_ENV, "99999");
        }
        let result = resolve_port(TEST_ENV, 4317);
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
    }

    #[test]
    fn resolve_port_privileged_env_returns_invalid_port_err() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_PRIVILEGED";
        unsafe {
            std::env::set_var(TEST_ENV, "80");
        }
        let result = resolve_port(TEST_ENV, 4317);
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
    }

    #[test]
    fn resolve_port_zero_env_returns_invalid_port_err() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_ZERO";
        unsafe {
            std::env::set_var(TEST_ENV, "0");
        }
        let result = resolve_port(TEST_ENV, 4317);
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert!(matches!(result, Err(IngestError::InvalidPort { .. })));
    }

    #[test]
    fn resolve_port_valid_non_privileged_returns_ok() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_VALID";
        unsafe {
            std::env::set_var(TEST_ENV, "9000");
        }
        let result = resolve_port(TEST_ENV, 4317);
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        let port = result.expect("non-privileged port must pass");
        assert_eq!(port.value(), 9000);
    }

    #[test]
    fn resolve_port_spec_default_via_env_returns_ok() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_TEST_PORT_SPEC_DEFAULT";
        unsafe {
            std::env::set_var(TEST_ENV, "4318");
        }
        let result = resolve_port(TEST_ENV, 4317);
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        let port = result.expect("spec-default 4318 via env must pass");
        assert_eq!(port.value(), 4318);
    }

    // resolve_retention_seconds tests (chunk #21). Same `unsafe { std::env::set_var }`
    // discipline as resolve_port tests above — cargo-nextest gives per-process
    // isolation, each test uses a distinct fixture by removing/setting the
    // same env var (ANDROMEDA_PULSE_RETENTION_SECONDS) within a narrow scope.

    #[test]
    fn resolve_retention_seconds_unset_returns_default() {
        unsafe {
            std::env::remove_var(ENV_RETENTION_SECONDS);
        }
        assert_eq!(resolve_retention_seconds(), RETENTION_SECONDS_DEFAULT);
    }

    #[test]
    fn resolve_retention_seconds_unparseable_falls_back_to_default() {
        unsafe {
            std::env::set_var(ENV_RETENTION_SECONDS, "not-a-number");
        }
        let result = resolve_retention_seconds();
        unsafe {
            std::env::remove_var(ENV_RETENTION_SECONDS);
        }
        assert_eq!(result, RETENTION_SECONDS_DEFAULT);
    }

    #[test]
    fn resolve_retention_seconds_below_min_falls_back_to_default() {
        unsafe {
            std::env::set_var(ENV_RETENTION_SECONDS, "30");
        }
        let result = resolve_retention_seconds();
        unsafe {
            std::env::remove_var(ENV_RETENTION_SECONDS);
        }
        assert_eq!(result, RETENTION_SECONDS_DEFAULT);
    }

    #[test]
    fn resolve_retention_seconds_above_max_falls_back_to_default() {
        unsafe {
            std::env::set_var(ENV_RETENTION_SECONDS, "999999");
        }
        let result = resolve_retention_seconds();
        unsafe {
            std::env::remove_var(ENV_RETENTION_SECONDS);
        }
        assert_eq!(result, RETENTION_SECONDS_DEFAULT);
    }

    #[test]
    fn resolve_retention_seconds_in_range_returns_parsed_value() {
        unsafe {
            std::env::set_var(ENV_RETENTION_SECONDS, "300");
        }
        let result = resolve_retention_seconds();
        unsafe {
            std::env::remove_var(ENV_RETENTION_SECONDS);
        }
        assert_eq!(result, 300);
    }

    #[test]
    fn resolve_retention_seconds_at_min_boundary_returns_min() {
        unsafe {
            std::env::set_var(ENV_RETENTION_SECONDS, "60");
        }
        let result = resolve_retention_seconds();
        unsafe {
            std::env::remove_var(ENV_RETENTION_SECONDS);
        }
        assert_eq!(result, RETENTION_SECONDS_MIN);
    }

    #[test]
    fn resolve_retention_seconds_at_max_boundary_returns_max() {
        unsafe {
            std::env::set_var(ENV_RETENTION_SECONDS, "86400");
        }
        let result = resolve_retention_seconds();
        unsafe {
            std::env::remove_var(ENV_RETENTION_SECONDS);
        }
        assert_eq!(result, RETENTION_SECONDS_MAX);
    }
}
