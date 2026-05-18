use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{env, fs};

use buffer::{BroadcastSenders, BufferState, create_schema, run_consumer, run_retention};
use duckdb::Connection;
use ingest::channel::{IngestSender, build_channel};
use ingest::connection::{self, ConnectionBroadcast, ReceiverBindStatus};
use ingest::contract::{Error as IngestError, OtlpPort};
use ingest::observer::SpanObserver;
use ingest::state::IngestState;
use tauri::Manager;
use tracing_error::SpanTrace;
use triage::contract::{
    AttentionCueBroadcast, CadenceTriggerChannel, DEFAULT_AUTONOMOUS_THRESHOLD,
    DEFAULT_DETECTION_SUB_WINDOW_SECONDS, DEFAULT_HEARTBEAT_INTERVAL,
    DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL, DEFAULT_MAX_SIZE_BYTES, DEFAULT_PERSIST_INTERVAL_NANOS,
    DEFAULT_SERVICE_COUNT_CAP, DEFAULT_STORM_WINDOW_SECONDS, DEFAULT_SUGGESTED_THRESHOLD,
    InMemoryServiceRegistry, RestartDetector, RestartEventBroadcast, RetryStormDetector,
    ServiceLifecycleBroadcast, ServiceRegistry, SuppressionState, Thresholds, bootstrap_state,
    resolve_corpus_path, run_persist_loop, start_emitter, start_lifecycle_heartbeat,
    start_restart_detector, start_storm_detector,
};
use ui_bridge::Settings;
use ui_bridge::health::{
    BindStatus, BufferConnectionStatus, HeartbeatState, IngestChannelStatus, IntrospectionApi,
    IntrospectionApiImpl, record_start, register_heartbeat_state,
};
use ui_bridge::telemetry::{TelemetryApi, TelemetryApiImpl};
use ui_bridge::workspace_ipc::{WorkspaceApi, WorkspaceApiImpl};
use viz::VizState;

use pulse_app::taurpc_export_config;
use pulse_app::{heartbeat, observability, tray, window};

use pulse_app::baseline_observer::BaselineObserverAdapter;
use pulse_app::connection_router::{ConnectionApi, ConnectionApiImpl, HeartbeatBindStatus};
#[cfg(feature = "mcp-server")]
use pulse_app::mcp_router::{McpApi, McpApiImpl};
use pulse_app::plugins_router::{PluginsApi, PluginsApiImpl};
use pulse_app::restart_observer::{CompositeSpanObserver, RestartObserverAdapter};
use pulse_app::services_router::{ServicesApi, ServicesApiImpl};
use pulse_app::snapshot_runtime::{SnapshotApi, SnapshotApiImpl};
use pulse_app::storage_router::{StorageApi, StorageApiImpl};
use pulse_app::storm_observer::StormObserverAdapter;
use pulse_app::streams::{StreamsApi, StreamsApiImpl};
use pulse_app::viz_routers::{
    LogsApi, LogsApiImpl, MetricsApi, MetricsApiImpl, TracesApi, TracesApiImpl,
};

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

// Resolves the path to the andromeda-pulse-mcp sidecar binary. Sibling of
// the current executable (same dir, with .exe on Windows). Falls back to а
// non-resolving placeholder if the current binary path cannot be read —
// `mcp.start` then surfaces AppError::Internal at spawn time. Chunk #49.
#[cfg(feature = "mcp-server")]
fn resolve_mcp_sidecar_binary_path() -> std::path::PathBuf {
    let binary_name = if cfg!(target_os = "windows") {
        "andromeda-pulse-mcp.exe"
    } else {
        "andromeda-pulse-mcp"
    };
    match env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
    {
        Some(dir) => dir.join(binary_name),
        None => std::path::PathBuf::from(binary_name),
    }
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
    // taurpc's `Router::into_handler()` spawns a background handler-manager
    // task during binding emission and requires a tokio runtime in scope, but
    // Tauri's Builder doesn't establish one until `.run()` (which happens
    // after the router is constructed). Build + enter our own runtime first
    // and hand it to Tauri via `async_runtime::set` so taurpc's pre-`run()`
    // spawns and Tauri's setup-closure spawns share a single runtime.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    let _enter = runtime.enter();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let data_dir = resolve_data_dir();
    let _guard = observability::init(&data_dir);
    record_start();
    write_pid_file(&data_dir);
    window::emit_boot_spans();

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

    // Chunk #59 — connection state machine substrate. The poller (spawned in
    // setup closure below) reads `ingest_state.last_ingest_at_nanos()` and
    // bind status from `heartbeat_state` via the `HeartbeatBindStatus`
    // adapter; transitions broadcast on `pulse://stream/connection-state`.
    // The TauRPC procedure `connection.current_state` reads the same atomic
    // for point queries.
    let connection_broadcast = Arc::new(ConnectionBroadcast::new());
    let bind_status: Arc<dyn ReceiverBindStatus> =
        Arc::new(HeartbeatBindStatus::new(Arc::clone(&heartbeat_state)));
    let connection_impl =
        ConnectionApiImpl::new(Arc::clone(&ingest_state), Arc::clone(&bind_status));

    // Chunk #62 — attention cue emitter substrate. BaselineState bootstraps
    // from disk corpus if present (chunk #61 deferred this wiring); the
    // BaselineObserverAdapter wraps it as `ingest::observer::SpanObserver`
    // for buffer's consumer tap. AttentionCueBroadcast + CadenceTriggerChannel
    // are the emit surfaces (chunk #62); Thresholds carries hardcoded
    // defaults this chunk (hot-reload lands in #86).
    let corpus_path = resolve_corpus_path(&data_dir).unwrap_or_else(|_| {
        // Fallback: a non-canonicalize-able path means persist will fail; the
        // emitter still operates on the in-memory state for the session.
        data_dir.join("triage").join("baseline-corpus.bin")
    });
    let baseline_now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let baseline_state = Arc::new(bootstrap_state(
        &corpus_path,
        DEFAULT_MAX_SIZE_BYTES,
        DEFAULT_SERVICE_COUNT_CAP,
        baseline_now,
    ));
    let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
    let cadence_channel = Arc::new(CadenceTriggerChannel::new());
    let thresholds = Arc::new(Thresholds::default());

    // Chunk #63 — restart event detector + dual-condition bypass substrate.
    // RestartDetector tracks per-service last-seen timestamps; gap > threshold
    // triggers emission on `pulse://stream/restart-events`. SuppressionState
    // holds per-service post-restart suppression windows consumed by the cue
    // emitter's surgical-suppression filter. RestartObserverAdapter wraps the
    // detector + broadcast for the span-observer hot path; CompositeSpanObserver
    // fan-outs each ingested span to BOTH baseline + restart adapters.
    let restart_broadcast = Arc::new(RestartEventBroadcast::new());
    let restart_detector = Arc::new(RestartDetector::new(
        thresholds.restart_gap_threshold_seconds,
    ));
    let suppression_state = Arc::new(SuppressionState::new());

    let baseline_adapter: Arc<dyn SpanObserver> =
        Arc::new(BaselineObserverAdapter::new(Arc::clone(&baseline_state)));
    let restart_adapter: Arc<dyn SpanObserver> = Arc::new(RestartObserverAdapter::new(
        Arc::clone(&restart_detector),
        Arc::clone(&restart_broadcast),
    ));
    let span_observer: Arc<dyn SpanObserver> = Arc::new(CompositeSpanObserver::new(vec![
        baseline_adapter,
        restart_adapter,
    ]));

    // Chunk #66 — retry storm detector. Tracks per-fingerprint occurrences
    // in a 60s rolling window; ≥5/30s → Suggested cue, ≥10/30s → Autonomous
    // cue, emitted through the existing chunk #62 attention-cues broadcast
    // channel. State в-memory only (corpus persistence deferred к chunk #69
    // per arch §[Telemetry Retention Surface]). StormObserverAdapter wraps
    // the detector + broadcast for the buffer-side FingerprintObserver hot
    // path; trait declaration lives в the lower buffer crate per arch
    // §Cross-cutting Patterns Module dependency direction.
    let storm_detector = Arc::new(RetryStormDetector::new(
        DEFAULT_STORM_WINDOW_SECONDS,
        DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
        DEFAULT_SUGGESTED_THRESHOLD,
        DEFAULT_AUTONOMOUS_THRESHOLD,
    ));
    let fingerprint_observer: Option<Arc<dyn buffer::fingerprint::FingerprintObserver>> =
        Some(Arc::new(StormObserverAdapter::new(
            Arc::clone(&storm_detector),
            Arc::clone(&cue_broadcast),
        )));

    // Chunk #67 — service lifecycle state machine + registry. In-memory
    // DashMap-backed registry per arch §[Telemetry Retention Surface]
    // guardrail (corpus persistence deferred to chunk #69 SQLite scaffold).
    // Heartbeat task spawned in setup closure below; subscribes to chunk #63
    // `pulse://stream/restart-events` to trigger Bootstrapping transitions
    // on any-state restart-detector observation. State derives at tick time
    // from chunk #61 `BaselineState` activity-floor snapshots; thresholds
    // (`dormant_after_secs` / `archived_after_secs`) flow through Settings.
    let lifecycle_registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    let lifecycle_broadcast = Arc::new(ServiceLifecycleBroadcast::new());
    let services_impl = ServicesApiImpl::new(
        Arc::clone(&lifecycle_registry),
        Arc::clone(&lifecycle_broadcast),
    );

    // Chunk #68 — persistent incident corpus scaffold (NEW `crates/corpus/`).
    // OS keychain backend fetches (or creates on first launch) the 32-byte
    // AES-256-GCM key per capability P-049. Corpus opens at the resolved
    // data dir's `corpus/corpus.db` subpath; first-launch creates the file
    // + runs schema migrations idempotently. Boot non-fatal: if keychain
    // unavailable OR corpus open fails, log structured error + continue
    // with corpus reader absent (storage.inspect / storage.path return
    // AppError::Storage at IPC time).
    let keychain_backend: Arc<dyn corpus::contract::KeychainBackend> = Arc::new(
        corpus::contract::OsKeychainBackend::new("com.andromeda.pulse"),
    );
    let corpus_db_path = data_dir.join("corpus").join("corpus.db");
    let corpus_reader: Option<Arc<dyn corpus::contract::CorpusReader>> =
        match corpus::contract::Corpus::open(corpus_db_path.clone(), Arc::clone(&keychain_backend))
        {
            Ok(c) => Some(Arc::new(c)),
            Err(e) => {
                tracing::error!(
                    target: "corpus.open.error",
                    error_kind = ?e,
                    "corpus open failed at boot; storage.* IPC will return error until corpus available",
                );
                None
            }
        };
    let storage_impl = corpus_reader
        .as_ref()
        .map(|r| StorageApiImpl::new(Arc::clone(r)));

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

    // features array surfaces in `app_info.features`; populated from compile-time
    // cfg!() checks. mcp-server is the only known opt-in feature at chunk #27.
    #[cfg(feature = "mcp-server")]
    let features: Vec<String> = vec!["mcp-server".to_string()];
    #[cfg(not(feature = "mcp-server"))]
    let features: Vec<String> = Vec::new();
    let introspection_impl = IntrospectionApiImpl::new(
        data_dir.clone(),
        features,
        Some(Arc::clone(&broadcast_senders)),
    );

    // Chunk #44: SnapshotApiImpl constructed BEFORE the router build so the
    // sibling clone (snapshot_impl_for_setup) can survive into the setup
    // closure to populate AppHandle. The Arc<OnceLock<AppHandle<Wry>>> is
    // shared between clones; setup populates once, resolver reads on every
    // IPC call.
    let snapshot_impl = SnapshotApiImpl::new(buffer_conn.clone(), data_dir.clone());
    let snapshot_impl_for_setup = snapshot_impl.clone();

    // Chunk #47: plugin host wiring. Engine is constructed at boot (shared
    // across all plugin operations); plugin dir is resolved + canonicalized
    // (missing-dir is non-fatal — boot proceeds with empty registry); initial
    // discovery populates the registry. The registry Mutex is shared with
    // the heartbeat task via Arc::clone.
    let plugin_engine = Arc::new(plugins::engine::build_engine().expect("plugin engine builds"));
    let plugin_dir_raw = plugins::loader::resolve_plugin_dir();
    let canonical_plugin_dir = match plugins::loader::canonicalize_plugin_dir(&plugin_dir_raw) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(
                target: "plugin.load.boot",
                error_msg = %e,
                "plugin dir canonicalization failed; plugin host disabled this boot",
            );
            plugin_dir_raw.clone()
        }
    };
    let canonical_plugin_dir = Arc::new(canonical_plugin_dir);
    let initial_plugins =
        plugins::loader::discover_plugins(&plugin_engine, canonical_plugin_dir.as_path())
            .unwrap_or_else(|e| {
                tracing::warn!(
                    target: "plugin.load.boot",
                    error_msg = %e,
                    "plugin discovery failed at boot; registry starts empty",
                );
                Vec::new()
            });
    let mut boot_registry = plugins::loader::PluginRegistry::empty();
    boot_registry.replace_plugins(initial_plugins);
    let plugins_registry = Arc::new(Mutex::new(boot_registry));
    let plugins_impl = PluginsApiImpl::new(
        Arc::clone(&plugin_engine),
        Arc::clone(&plugins_registry),
        Arc::clone(&canonical_plugin_dir),
    );

    // Chunk #49: mcp.* router wiring. The sidecar binary path is resolved
    // by joining the current binary's parent directory with the sidecar
    // executable name (with .exe extension on Windows). If the current
    // binary's path cannot be resolved, fall back to a relative name that
    // will fail at spawn time (mcp.start surfaces AppError::Internal then).
    #[cfg(feature = "mcp-server")]
    let mcp_impl = {
        let mcp_sidecar_binary_path = Arc::new(resolve_mcp_sidecar_binary_path());
        McpApiImpl::new(Arc::clone(&mcp_sidecar_binary_path))
    };

    let invoke_router = match buffer_conn.as_ref() {
        Some(conn) => {
            let base = taurpc::Router::new()
                .export_config(taurpc_export_config())
                .merge(introspection_impl.clone().into_handler())
                .merge(TracesApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
                .merge(MetricsApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
                .merge(LogsApiImpl::new(Arc::clone(conn), Arc::clone(&viz_state)).into_handler())
                .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler())
                .merge(TelemetryApiImpl::new().into_handler())
                .merge(snapshot_impl.clone().into_handler())
                .merge(WorkspaceApiImpl::new().into_handler())
                .merge(plugins_impl.clone().into_handler())
                .merge(connection_impl.clone().into_handler())
                .merge(services_impl.clone().into_handler());
            let base = match storage_impl.as_ref() {
                Some(s) => base.merge(s.clone().into_handler()),
                None => base,
            };
            #[cfg(feature = "mcp-server")]
            let base = base.merge(mcp_impl.clone().into_handler());
            base
        }
        None => {
            let base = taurpc::Router::new()
                .export_config(taurpc_export_config())
                .merge(introspection_impl.clone().into_handler())
                .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler())
                .merge(TelemetryApiImpl::new().into_handler())
                .merge(snapshot_impl.clone().into_handler())
                .merge(WorkspaceApiImpl::new().into_handler())
                .merge(plugins_impl.clone().into_handler())
                .merge(connection_impl.clone().into_handler())
                .merge(services_impl.clone().into_handler());
            let base = match storage_impl.as_ref() {
                Some(s) => base.merge(s.clone().into_handler()),
                None => base,
            };
            #[cfg(feature = "mcp-server")]
            let base = base.merge(mcp_impl.clone().into_handler());
            base
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .on_window_event(window::on_window_event)
        .invoke_handler(invoke_router.into_handler())
        .setup(move |app| {
            // Chunk #44: populate the deferred AppHandle into SnapshotApiImpl.
            // Setup runs after invoke_handler is locked, but the Arc<OnceLock>
            // shared between snapshot_impl (in router) and snapshot_impl_for_setup
            // (this closure) means the resolver sees the handle on every
            // subsequent IPC call.
            snapshot_impl_for_setup.set_app_handle(app.handle().clone());

            window::show_compact_widget(app);
            let settings = Settings::load_from_data_dir(&data_dir);
            window::apply_widget_settings(app, &settings);
            let tray_icon = tray::setup_tray(app.handle(), Arc::clone(&broadcast_senders))?;
            app.manage(tray_icon);
            match buffer_conn {
                Some(conn) => {
                    tauri::async_runtime::spawn(run_consumer(
                        ingest_receiver,
                        Arc::clone(&conn),
                        Arc::clone(&buffer_state),
                        Arc::clone(&broadcast_senders),
                        Arc::clone(&span_observer),
                        fingerprint_observer.clone(),
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
            // Chunk #59 — connection-state FSM detector loop. 1-second tick
            // reading IngestState atomic + bind status; broadcasts payload on
            // pulse://stream/connection-state ONLY when state changes. Separate
            // concern from the 15s heartbeat tick task below (which emits the
            // periodic `connection.tick` event regardless of state change).
            tauri::async_runtime::spawn(connection::start_poller(
                Arc::clone(&ingest_state),
                Arc::clone(&bind_status),
                Arc::clone(&connection_broadcast),
            ));
            // Chunk #62 — baseline corpus periodic persist (60s + on-shutdown
            // deferred to next chunk per plan Implementation note 5) +
            // attention cue emitter background tick (1s cadence reading all
            // BaselineState trackers, evaluating thresholds, emitting cues
            // to broadcast + cadence-triggers channel).
            tauri::async_runtime::spawn(run_persist_loop(
                Arc::clone(&baseline_state),
                corpus_path.clone(),
                std::time::Duration::from_nanos(DEFAULT_PERSIST_INTERVAL_NANOS as u64),
            ));
            tauri::async_runtime::spawn(start_emitter(
                Arc::clone(&baseline_state),
                Arc::clone(&cue_broadcast),
                Arc::clone(&cadence_channel),
                Arc::clone(&thresholds),
                Arc::clone(&restart_broadcast),
                Arc::clone(&suppression_state),
            ));
            // Chunk #63 — restart detector heartbeat tick (15s default
            // cadence per `.claude/rules/observability.md` heartbeat-ticks
            // rule); detection itself happens inline at observe_span time
            // via the RestartObserverAdapter hot-path hook.
            tauri::async_runtime::spawn(start_restart_detector(
                Arc::clone(&restart_detector),
                DEFAULT_HEARTBEAT_INTERVAL,
            ));
            // Chunk #66 — retry storm detector heartbeat tick (15s default;
            // shares cadence с chunk #63 restart detector). Storm detection
            // itself happens inline at fingerprint-observation time via the
            // StormObserverAdapter buffer-side hot-path hook.
            tauri::async_runtime::spawn(start_storm_detector(
                Arc::clone(&storm_detector),
                DEFAULT_HEARTBEAT_INTERVAL,
            ));
            // Chunk #67 — service lifecycle heartbeat tick (15s default
            // sibling cadence). Reads BaselineState activity snapshots,
            // emits ServiceLifecycleEvent transitions on the broadcast,
            // and subscribes к pulse://stream/restart-events to trigger
            // Bootstrapping transitions on any-state restart-observed gap.
            // Thresholds source from persisted Settings (loaded above).
            let lifecycle_restart_rx = restart_broadcast.subscribe();
            tauri::async_runtime::spawn(start_lifecycle_heartbeat(
                Arc::clone(&lifecycle_registry),
                Arc::clone(&lifecycle_broadcast),
                Arc::clone(&baseline_state),
                lifecycle_restart_rx,
                settings.lifecycle_dormant_after_secs,
                settings.lifecycle_archived_after_secs,
                DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL,
            ));
            let _heartbeat_handles = heartbeat::spawn(
                heartbeat_state,
                Arc::clone(&ingest_state),
                Arc::clone(&ingest_sender),
                Arc::clone(&buffer_state),
                retention_seconds,
                Arc::clone(&viz_state),
                Arc::clone(&broadcast_senders),
                Arc::clone(&plugins_registry),
                Arc::clone(&bind_status),
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

    // Bindings emission test (chunk #25). taurpc 0.7 emits TS bindings at
    // `Router::into_handler()` call time when `tauri::is_dev()` returns true
    // (which is `!cfg!(feature = "custom-protocol")`, true in test builds).
    // The emission target path on the HealthApi `#[taurpc::procedures]` macro
    // is `ui/src/bindings/index.ts` — relative to the test runtime cwd, which
    // is `pulse-app/` for `cargo nextest -p pulse-app`. The merged router
    // here mirrors the production wiring in `main()` (when buffer init OK).
    // `#[tokio::test]` provides the runtime context taurpc::TauRpcHandler::spawn()
    // requires.
    #[tokio::test]
    async fn emit_taurpc_bindings() {
        let conn = Arc::new(Mutex::new(
            Connection::open_in_memory().expect("in-memory DuckDB"),
        ));
        let viz_state = Arc::new(VizState::new());
        let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());
        let data_dir = std::env::temp_dir().join("andromeda-pulse-bindings-test");
        let introspection_impl = IntrospectionApiImpl::new(
            data_dir.clone(),
            vec![],
            Some(Arc::clone(&broadcast_senders)),
        );

        let snapshot_impl = SnapshotApiImpl::new(Some(Arc::clone(&conn)), data_dir.clone());

        // Chunk #47: PluginsApiImpl participates in the emit so the bindings.ts
        // ARGS_MAP includes plugins.list / reload / invoke; xtask capability-drift
        // depends on this emission to verify EXPECTED_PROCEDURES sync.
        let plugin_engine = Arc::new(
            plugins::engine::build_engine().expect("plugin engine builds for bindings test"),
        );
        let plugins_registry = Arc::new(Mutex::new(plugins::loader::PluginRegistry::empty()));
        let canonical_plugin_dir =
            Arc::new(std::env::temp_dir().join("andromeda-pulse-plugins-bindings-test"));
        let plugins_impl = PluginsApiImpl::new(
            Arc::clone(&plugin_engine),
            Arc::clone(&plugins_registry),
            Arc::clone(&canonical_plugin_dir),
        );

        // Chunk #59: ConnectionApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes connection.current_state (quadruple-binding 4th slot
        // per .claude/rules/security.md Session Additions 2026-05-12).
        let connection_ingest = Arc::new(IngestState::new());
        struct TestBindStatus;
        impl ingest::connection::ReceiverBindStatus for TestBindStatus {
            fn any_receiver_failed(&self) -> bool {
                false
            }
        }
        let connection_bind: Arc<dyn ingest::connection::ReceiverBindStatus> =
            Arc::new(TestBindStatus);
        let connection_impl = ConnectionApiImpl::new(connection_ingest, connection_bind);

        // Chunk #67: ServicesApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes services.list_with_states (quadruple-binding 4th slot
        // per .claude/rules/security.md Session Additions 2026-05-12).
        let services_registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
        let services_broadcast = Arc::new(ServiceLifecycleBroadcast::new());
        let services_impl = ServicesApiImpl::new(services_registry, services_broadcast);

        // Chunk #68: StorageApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes storage.inspect / storage.path (quadruple-binding
        // 4th slot per .claude/rules/security.md Session Additions 2026-05-12).
        // In-memory corpus backed by FakeKeychainBackend keeps the test
        // hermetic — no real OS keychain or on-disk SQLite file.
        let storage_keychain: Arc<dyn corpus::contract::KeychainBackend> =
            Arc::new(corpus::contract::FakeKeychainBackend::new());
        let storage_corpus = corpus::contract::Corpus::open_in_memory(storage_keychain)
            .expect("in-memory corpus opens with fake keychain");
        let storage_reader: Arc<dyn corpus::contract::CorpusReader> = Arc::new(storage_corpus);
        let storage_impl = StorageApiImpl::new(storage_reader);

        // Chunk #49: McpApiImpl participates in the emit so the bindings.ts
        // ARGS_MAP includes mcp.status / mcp.start / mcp.stop (4th binding
        // per .claude/rules/security.md Session Additions 2026-05-12).
        #[cfg(feature = "mcp-server")]
        let mcp_impl = {
            let mcp_sidecar_path =
                Arc::new(std::env::temp_dir().join("andromeda-pulse-mcp-bindings-test"));
            McpApiImpl::new(mcp_sidecar_path)
        };

        let router: taurpc::Router<tauri::Wry> = {
            let base = taurpc::Router::new()
                .export_config(taurpc_export_config())
                .merge(introspection_impl.into_handler())
                .merge(TracesApiImpl::new(Arc::clone(&conn), Arc::clone(&viz_state)).into_handler())
                .merge(
                    MetricsApiImpl::new(Arc::clone(&conn), Arc::clone(&viz_state)).into_handler(),
                )
                .merge(LogsApiImpl::new(Arc::clone(&conn), Arc::clone(&viz_state)).into_handler())
                .merge(StreamsApiImpl::new(Arc::clone(&broadcast_senders)).into_handler())
                .merge(TelemetryApiImpl::new().into_handler())
                .merge(snapshot_impl.into_handler())
                .merge(WorkspaceApiImpl::new().into_handler())
                .merge(plugins_impl.into_handler())
                .merge(connection_impl.into_handler())
                .merge(services_impl.into_handler())
                .merge(storage_impl.into_handler());
            #[cfg(feature = "mcp-server")]
            let base = base.merge(mcp_impl.into_handler());
            base
        };

        // into_handler() triggers export_types() in dev mode.
        let _handler = router.into_handler();

        let bindings_path = std::path::Path::new("ui/src/bindings/index.ts");
        assert!(
            bindings_path.exists(),
            "bindings file not emitted at {bindings_path:?} (cwd={:?})",
            std::env::current_dir().ok()
        );
    }
}
