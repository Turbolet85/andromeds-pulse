use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{env, fs};

use ingest::state::IngestState;
use ui_bridge::health::{
    BindStatus, HealthApi, HealthApiImpl, HeartbeatState, record_start, register_heartbeat_state,
};

mod heartbeat;
mod observability;

const ENV_OTLP_GRPC_PORT: &str = "ANDROMEDA_PULSE_OTLP_GRPC_PORT";
const ENV_OTLP_HTTP_PORT: &str = "ANDROMEDA_PULSE_OTLP_HTTP_PORT";

fn resolve_grpc_port() -> u16 {
    match env::var(ENV_OTLP_GRPC_PORT) {
        Ok(raw) => match raw.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                tracing::warn!(
                    target: "app.boot.otlp.grpc.port",
                    raw_len = raw.len(),
                    "invalid port value; falling back to default"
                );
                ingest::grpc::DEFAULT_GRPC_PORT
            }
        },
        Err(_) => ingest::grpc::DEFAULT_GRPC_PORT,
    }
}

fn resolve_http_port() -> u16 {
    match env::var(ENV_OTLP_HTTP_PORT) {
        Ok(raw) => match raw.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                tracing::warn!(
                    target: "app.boot.otlp.http.port",
                    raw_len = raw.len(),
                    "invalid port value; falling back to default"
                );
                ingest::http::DEFAULT_HTTP_PORT
            }
        },
        Err(_) => ingest::http::DEFAULT_HTTP_PORT,
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
    let grpc_port = resolve_grpc_port();
    let grpc_addr = SocketAddr::from(([127, 0, 0, 1], grpc_port));
    let http_port = resolve_http_port();
    let http_addr = SocketAddr::from(([127, 0, 0, 1], http_port));

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(taurpc::create_ipc_handler(HealthApiImpl.into_handler()))
        .setup(move |_app| {
            let grpc_announcer = Arc::clone(&heartbeat_state);
            let grpc_state = Arc::clone(&ingest_state);
            tauri::async_runtime::spawn(async move {
                match ingest::grpc::try_bind(grpc_addr).await {
                    Ok(listener) => {
                        grpc_announcer.record_otlp_grpc_bind(BindStatus::Ok);
                        tracing::info!(
                            target: "app.boot.otlp.grpc.bind",
                            bind_address = %grpc_addr,
                            "OTLP gRPC receiver bound"
                        );
                        if let Err(e) = ingest::grpc::serve_on(listener, grpc_state).await {
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
                        grpc_announcer.record_otlp_grpc_bind(BindStatus::Failed(reason.clone()));
                        tracing::error!(
                            target: "app.boot.otlp.grpc.bind",
                            reason = %reason,
                            bind_address = %grpc_addr,
                            "bind failed"
                        );
                    }
                }
            });
            let http_announcer = Arc::clone(&heartbeat_state);
            let http_state = Arc::clone(&ingest_state);
            tauri::async_runtime::spawn(async move {
                match ingest::http::try_bind(http_addr).await {
                    Ok(listener) => {
                        http_announcer.record_otlp_http_bind(BindStatus::Ok);
                        tracing::info!(
                            target: "app.boot.otlp.http.bind",
                            bind_address = %http_addr,
                            "OTLP HTTP receiver bound"
                        );
                        if let Err(e) = ingest::http::serve_on(listener, http_state).await {
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
                        http_announcer.record_otlp_http_bind(BindStatus::Failed(reason.clone()));
                        tracing::error!(
                            target: "app.boot.otlp.http.bind",
                            reason = %reason,
                            bind_address = %http_addr,
                            "bind failed"
                        );
                    }
                }
            });
            let _heartbeat_handles = heartbeat::spawn(heartbeat_state, ingest_state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
