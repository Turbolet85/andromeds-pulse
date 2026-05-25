use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{env, fs};

use buffer::{
    BroadcastSenders, BufferState, DrainConfig, DrainMiner, create_schema, run_consumer,
    run_retention,
};
use duckdb::Connection;
use ingest::channel::{IngestSender, build_channel};
use ingest::connection::{self, ConnectionBroadcast, ReceiverBindStatus};
use ingest::contract::{Error as IngestError, OtlpPort};
use ingest::observer::SpanObserver;
use ingest::state::IngestState;
use tauri::Manager;
use tracing_error::SpanTrace;
use triage::contract::{
    AttentionCueBroadcast, BaselinePersistence, CadenceConfig, CadenceEventBroadcast,
    CadenceTriggerChannel, DEFAULT_AUTONOMOUS_THRESHOLD, DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
    DEFAULT_HEARTBEAT_INTERVAL, DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS,
    DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL, DEFAULT_LIFECYCLE_PERSIST_INTERVAL_SECS,
    DEFAULT_PERSIST_INTERVAL_NANOS, DEFAULT_SERVICE_COUNT_CAP, DEFAULT_STORM_PERSIST_INTERVAL_SECS,
    DEFAULT_STORM_WINDOW_SECONDS, DEFAULT_SUGGESTED_THRESHOLD, DigestBroadcast,
    HardwareProfileSource, InMemoryIncidentRegistry, InMemoryServiceRegistry,
    IncidentLifecycleBroadcast, IncidentPersistence, IncidentRegistry, LifecyclePersistence,
    LwwQueue, RestartDetector, RestartEventBroadcast, RetryStormDetector,
    ServiceLifecycleBroadcast, ServiceRegistry, SqlQueryRunner, StormPersistence, SuppressionState,
    TARGET_LIFECYCLE_CORPUS_RESTORE, TARGET_LIFECYCLE_PERSIST_ERROR,
    TARGET_PATTERN_STORM_CORPUS_RESTORE, TARGET_PATTERN_STORM_PERSIST_ERROR, Thresholds,
    TriageSqlState, bootstrap_state, run_incident_persist_loop, run_lifecycle_persist_loop,
    run_persist_loop, run_storm_persist_loop, start_cadence_coordinator, start_emitter,
    start_lifecycle_heartbeat, start_restart_detector, start_storm_detector,
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
use pulse_app::baseline_persistence::{
    CorpusBaselinePersistence, migrate_legacy_baseline_if_present,
};
use pulse_app::cadence_runner::CadenceSqlRunner;
use pulse_app::connection_router::{ConnectionApi, ConnectionApiImpl, HeartbeatBindStatus};
use pulse_app::diagnostics_router::{DiagnosticsApi, DiagnosticsApiImpl};
use pulse_app::drain_persistence::CorpusDrainPersistence;
use pulse_app::incident_observer::{AutoResolveObserver, run_auto_resolution_loop};
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use pulse_app::incidents_router::{IncidentsApi, IncidentsApiImpl};
use pulse_app::lifecycle_persistence::CorpusLifecyclePersistence;
use pulse_app::llamacli_inference::LlamaCliInference;
#[cfg(feature = "mcp-server")]
use pulse_app::mcp_router::{McpApi, McpApiImpl};
use pulse_app::model_router::{ModelApi, ModelApiImpl, tier_for_profile};
use pulse_app::plugins_router::{PluginsApi, PluginsApiImpl};
use pulse_app::restart_observer::{CompositeSpanObserver, RestartObserverAdapter};
use pulse_app::services_router::{ServicesApi, ServicesApiImpl};
use pulse_app::snapshot_runtime::{SnapshotApi, SnapshotApiImpl};
use pulse_app::storage_router::{StorageApi, StorageApiImpl};
use pulse_app::storm_observer::StormObserverAdapter;
use pulse_app::storm_persistence::CorpusStormPersistence;
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

    // Chunk #68 — persistent incident corpus scaffold (NEW `crates/corpus/`).
    // OS keychain backend fetches (or creates on first launch) the 32-byte
    // AES-256-GCM key per capability P-049. Corpus opens at the resolved
    // data dir's `corpus/corpus.db` subpath; first-launch creates the file
    // + runs schema migrations idempotently. Boot non-fatal: if keychain
    // unavailable OR corpus open fails, log structured error + continue
    // with corpus reader absent (storage.inspect / storage.path return
    // AppError::Storage at IPC time).
    //
    // Chunk #70 promoted this block above the baseline_state bootstrap so
    // BaselinePersistence can be derived from corpus_writer before
    // bootstrap_state is called.
    let keychain_backend: Arc<dyn corpus::contract::KeychainBackend> = Arc::new(
        corpus::contract::OsKeychainBackend::new("com.andromeda.pulse"),
    );
    let corpus_db_path = data_dir.join("corpus").join("corpus.db");
    let corpus_arc: Option<Arc<corpus::contract::Corpus>> = match corpus::contract::Corpus::open(
        corpus_db_path.clone(),
        Arc::clone(&keychain_backend),
    ) {
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
    let corpus_reader: Option<Arc<dyn corpus::contract::CorpusReader>> = corpus_arc
        .as_ref()
        .map(|c| Arc::clone(c) as Arc<dyn corpus::contract::CorpusReader>);
    // Chunk #69 Phase B Session 4 — CorpusWriter trait view от the same
    // underlying Arc<Corpus>. Both reader + writer share one rusqlite
    // connection mutex; CorpusReader stays read-only-by-design per P-051
    // while CorpusWriter is the additive write surface для pipeline-metric
    // persistence (drain template tree this chunk; future chunks #71+
    // digest archive writes).
    let corpus_writer: Option<Arc<dyn corpus::contract::CorpusWriter>> = corpus_arc
        .as_ref()
        .map(|c| Arc::clone(c) as Arc<dyn corpus::contract::CorpusWriter>);
    let storage_impl = corpus_reader
        .as_ref()
        .map(|r| StorageApiImpl::new(Arc::clone(r)));

    // Chunk #70 — BaselineState corpus persistence adapter. Derives a
    // third trait view from the same Arc<Corpus> (alongside reader +
    // drain-writer); BaselineState moves from the chunk #61 flat-file
    // bincode at `<data_dir>/triage/baseline-corpus.bin` к corpus SQLite
    // via Schema Option A (mirrors chunk #69 Drain — reuses the
    // `pipeline_metrics` blob slot with metric_name="baseline_state",
    // layer="l1b"). None ⇒ corpus unavailable at boot (keychain failure
    // / SQLite open failure); baseline_state runs in-memory-only for
    // the session, a one-time warn-log fires below.
    let baseline_persistence: Option<Arc<dyn BaselinePersistence>> =
        corpus_writer.as_ref().map(|w| {
            Arc::new(CorpusBaselinePersistence::new(Arc::clone(w))) as Arc<dyn BaselinePersistence>
        });

    // Chunk #70 — one-shot legacy bincode migration. Reads
    // `<data_dir>/triage/baseline-corpus.bin` (chunk #61 substrate); if
    // present + valid, re-persists through the trait + deletes the
    // legacy file. Idempotent on subsequent boots. Failures preserve
    // the legacy file for retry; aggregate-only tracing emits per
    // CLAUDE.md 2026-05-17 session 84 triage AllowList convention.
    if let Some(persistence) = baseline_persistence.as_ref() {
        let _ = migrate_legacy_baseline_if_present(&data_dir, persistence.as_ref());
    } else {
        tracing::warn!(
            target: triage::contract::TARGET_BASELINE_PERSIST_ERROR,
            error_category = "corpus_unavailable_at_boot",
            duration_ms = 0_u64,
            "baseline persistence unavailable; in-memory only this session",
        );
    }

    // Chunk #71 — lifecycle + storm corpus persistence adapters. Derive
    // 3rd + 4th trait views from the same `Arc<Corpus>` (alongside reader
    // + writer + baseline_persistence). None ⇒ corpus unavailable at
    // boot; registry + storm detector run in-memory-only this session
    // with one-time warn-once-at-boot emission below.
    let lifecycle_persistence: Option<Arc<dyn LifecyclePersistence>> =
        corpus_writer.as_ref().map(|w| {
            Arc::new(CorpusLifecyclePersistence::new(Arc::clone(w)))
                as Arc<dyn LifecyclePersistence>
        });
    let storm_persistence: Option<Arc<dyn StormPersistence>> = corpus_writer
        .as_ref()
        .map(|w| Arc::new(CorpusStormPersistence::new(Arc::clone(w))) as Arc<dyn StormPersistence>);
    if lifecycle_persistence.is_none() {
        tracing::warn!(
            target: TARGET_LIFECYCLE_PERSIST_ERROR,
            error_category = "corpus_unavailable_at_boot",
            duration_ms = 0_u64,
            "lifecycle persistence unavailable; in-memory only this session",
        );
    }
    if storm_persistence.is_none() {
        tracing::warn!(
            target: TARGET_PATTERN_STORM_PERSIST_ERROR,
            error_category = "corpus_unavailable_at_boot",
            duration_ms = 0_u64,
            "storm persistence unavailable; in-memory only this session",
        );
    }

    // Chunk #62 — attention cue emitter substrate. BaselineState now
    // bootstraps via the chunk #70 BaselinePersistence trait (None ⇒
    // cold-start fresh state); the BaselineObserverAdapter wraps it as
    // `ingest::observer::SpanObserver` for buffer's consumer tap.
    // AttentionCueBroadcast + CadenceTriggerChannel are the emit
    // surfaces (chunk #62); Thresholds carries hardcoded defaults this
    // chunk (hot-reload lands in #86).
    let baseline_now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let baseline_state = Arc::new(bootstrap_state(
        baseline_persistence.as_deref(),
        DEFAULT_SERVICE_COUNT_CAP,
        baseline_now,
    ));
    let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
    let cadence_channel = Arc::new(CadenceTriggerChannel::new());
    let thresholds = Arc::new(Thresholds::default());

    // Chunk #80 — cadence coordinator substrate. CadenceEventBroadcast is
    // the L6-visibility emission topic (`pulse://stream/cadence-events`).
    // CadenceConfig loads from persisted Settings; coordinator reads once
    // at spawn (hot-reload deferred к chunk #94 per pulse-v0_2_0-route §80).
    // SqlQueryRunner is wired via CadenceSqlRunner adapter ONLY when buffer_conn
    // is Some — else coordinator is disabled (warn-logged below). Hardware
    // profile defaults к Unknown (Tier-2-enabled posture) until chunk #82
    // delivers а real detector.
    let cadence_event_broadcast = Arc::new(CadenceEventBroadcast::new());
    // Chunk #82 — replace the chunk #80 boot stub (UnknownHardwareProfile)
    // with the real `HardwareProfileDetector`. Detector probes GPU presence
    // + CPU core count at construction; cached for subsequent reads.
    // Env var `ANDROMEDA_PULSE_HARDWARE_PROFILE` overrides detection for
    // tests + degraded-environment validation.
    let hardware_profile: Arc<dyn HardwareProfileSource> =
        Arc::new(pulse_app::hardware_profile::HardwareProfileDetector::new());
    let detected_profile = hardware_profile.current_profile();
    let model_tier = tier_for_profile(detected_profile);
    let model_status_broadcast = interpretation::broadcast::ModelStatusBroadcast::new();
    let llamacli_inference = Arc::new(LlamaCliInference::new(
        model_tier,
        detected_profile,
        model_status_broadcast.clone(),
    ));
    let llm_runner: Arc<dyn interpretation::contract::LlmInferenceRunner> =
        Arc::clone(&llamacli_inference) as Arc<dyn interpretation::contract::LlmInferenceRunner>;
    let model_impl = ModelApiImpl::new(Arc::clone(&llm_runner), Arc::clone(&hardware_profile));

    // Chunk #84 — boot-time llama-cli readiness check. Fire-and-forget: if
    // `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH` or `ANDROMEDA_PULSE_MODEL_PATH`
    // are unset OR resolve to invalid paths, the runner stays в `ModelStatus::Error`
    // and `generate_constrained` returns `ModelNotConfigured` — the L4 subscriber
    // catches it as а runtime_error and skips. App boots cleanly in graceful-
    // degraded mode either way. Subprocess D1 means no actual model load happens
    // here (load happens per-generation inside llama-cli).
    {
        let runner_for_load = Arc::clone(&llamacli_inference);
        tokio::spawn(async move {
            let _ = runner_for_load.load_from_env_if_configured().await;
        });
    }
    let cadence_sql_runner: Option<Arc<dyn SqlQueryRunner>> = buffer_conn.as_ref().map(|conn| {
        let state = Arc::new(TriageSqlState::new(Arc::clone(conn)));
        Arc::new(CadenceSqlRunner::new(state)) as Arc<dyn SqlQueryRunner>
    });
    // CadenceConfig itself is constructed inside the setup closure where
    // `settings` is in scope (mirrors the chunk #67 lifecycle thresholds
    // pattern of reading settings at spawn-time rather than at substrate-
    // construction time).

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

    // Chunk #66 + chunk #71 corpus persistence — retry storm detector.
    // Tracks per-fingerprint occurrences in a 60s rolling window; ≥5/30s
    // → Suggested cue, ≥10/30s → Autonomous cue, emitted through the
    // existing chunk #62 attention-cues broadcast channel.
    // StormObserverAdapter wraps the detector + broadcast for the
    // buffer-side FingerprintObserver hot path; trait declaration lives
    // в the lower buffer crate per arch §Cross-cutting Patterns Module
    // dependency direction.
    //
    // Chunk #71 — restore state from corpus at boot. Ok(Some(snapshot))
    // → preserved dedup window per capability P-018; Ok(None) or Err(_)
    // → fresh fallback per error/cold-start path. Emits aggregate-only
    // tracing event + boot-restore counter metric.
    let storm_detector = {
        let restore_start = Instant::now();
        let (detector, restored_fingerprint_count) = match storm_persistence.as_ref() {
            Some(p) => match p.load() {
                Ok(Some(snapshot)) => {
                    let count = snapshot.entries.len() as u64;
                    (RetryStormDetector::restore_from_snapshot(snapshot), count)
                }
                Ok(None) => (
                    RetryStormDetector::new(
                        DEFAULT_STORM_WINDOW_SECONDS,
                        DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
                        DEFAULT_SUGGESTED_THRESHOLD,
                        DEFAULT_AUTONOMOUS_THRESHOLD,
                    ),
                    0,
                ),
                Err(e) => {
                    tracing::warn!(
                        target: TARGET_PATTERN_STORM_PERSIST_ERROR,
                        error_category = e.error_category(),
                        duration_ms = restore_start.elapsed().as_millis() as u64,
                        "storm corpus restore failed; fresh detector",
                    );
                    (
                        RetryStormDetector::new(
                            DEFAULT_STORM_WINDOW_SECONDS,
                            DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
                            DEFAULT_SUGGESTED_THRESHOLD,
                            DEFAULT_AUTONOMOUS_THRESHOLD,
                        ),
                        0,
                    )
                }
            },
            None => (
                RetryStormDetector::new(
                    DEFAULT_STORM_WINDOW_SECONDS,
                    DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
                    DEFAULT_SUGGESTED_THRESHOLD,
                    DEFAULT_AUTONOMOUS_THRESHOLD,
                ),
                0,
            ),
        };
        let restore_duration_ms = restore_start.elapsed().as_millis() as u64;
        tracing::info!(
            target: TARGET_PATTERN_STORM_CORPUS_RESTORE,
            restored_fingerprint_count = restored_fingerprint_count,
            duration_ms = restore_duration_ms,
            kind = "storm",
            "storm corpus restore",
        );
        tracing::info!(
            target: "metric.triage.pattern.storm.corpus_restore_count_total",
            value = restored_fingerprint_count,
            kind = "storm",
            "storm restore counter",
        );
        Arc::new(detector)
    };
    let fingerprint_observer: Option<Arc<dyn buffer::fingerprint::FingerprintObserver>> =
        Some(Arc::new(StormObserverAdapter::new(
            Arc::clone(&storm_detector),
            Arc::clone(&cue_broadcast),
        )));

    // Chunk #67 + chunk #71 corpus persistence — service lifecycle state
    // machine + registry. Heartbeat task spawned in setup closure below;
    // subscribes to chunk #63 `pulse://stream/restart-events` to trigger
    // Bootstrapping transitions on any-state restart-detector observation.
    // State derives at tick time from chunk #61 `BaselineState`
    // activity-floor snapshots; thresholds (`dormant_after_secs` /
    // `archived_after_secs`) flow through Settings.
    //
    // Chunk #71 — restore registry from corpus at boot. For each
    // restored service, emit a synthetic CorpusRestore lifecycle event
    // on `pulse://stream/service-lifecycle` so downstream constellation
    // observers cascade. P-027 closure: "Restart Pulse; verify dot
    // positions match prior session" runtime-true.
    let lifecycle_broadcast = Arc::new(ServiceLifecycleBroadcast::new());
    let lifecycle_registry: Arc<dyn ServiceRegistry> = {
        let restore_start = Instant::now();
        let restored_at = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        let (registry, restored_service_count) = match lifecycle_persistence.as_ref() {
            Some(p) => match p.load_all() {
                Ok(Some(entries)) => {
                    let count = entries.len() as u64;
                    let mut event_pairs: Vec<(String, triage::contract::ServiceLifecycleState)> =
                        entries.iter().map(|(s, e)| (s.clone(), e.state)).collect();
                    let reg = InMemoryServiceRegistry::from_entries(entries);
                    for (service, state) in event_pairs.drain(..) {
                        if let Some(event) =
                            reg.set_state_on_corpus_restore(&service, state, restored_at)
                        {
                            let _ = lifecycle_broadcast.sender().send(event);
                        }
                    }
                    (Arc::new(reg) as Arc<dyn ServiceRegistry>, count)
                }
                Ok(None) => (
                    Arc::new(InMemoryServiceRegistry::new()) as Arc<dyn ServiceRegistry>,
                    0,
                ),
                Err(e) => {
                    tracing::warn!(
                        target: TARGET_LIFECYCLE_PERSIST_ERROR,
                        error_category = e.error_category(),
                        duration_ms = restore_start.elapsed().as_millis() as u64,
                        "lifecycle corpus restore failed; fresh registry",
                    );
                    (
                        Arc::new(InMemoryServiceRegistry::new()) as Arc<dyn ServiceRegistry>,
                        0,
                    )
                }
            },
            None => (
                Arc::new(InMemoryServiceRegistry::new()) as Arc<dyn ServiceRegistry>,
                0,
            ),
        };
        let restore_duration_ms = restore_start.elapsed().as_millis() as u64;
        tracing::info!(
            target: TARGET_LIFECYCLE_CORPUS_RESTORE,
            kind = "lifecycle",
            count = restored_service_count,
            restored_service_count = restored_service_count,
            duration_ms = restore_duration_ms,
            "lifecycle corpus restore",
        );
        tracing::info!(
            target: "metric.triage.lifecycle.corpus_restore_count_total",
            value = restored_service_count,
            kind = "lifecycle",
            "lifecycle restore counter",
        );
        registry
    };
    let services_impl = ServicesApiImpl::new(
        Arc::clone(&lifecycle_registry),
        Arc::clone(&lifecycle_broadcast),
    );

    // Chunk #78 — incident records + lifecycle persistence. Derive а 5th
    // CorpusWriter trait view (alongside baseline + lifecycle + storm +
    // drain) from the same Arc<Corpus>. None ⇒ corpus unavailable at
    // boot; incident registry runs in-memory-only this session.
    // Hydrate the registry from corpus active-incidents on boot (P-042
    // cross-session continuity). Workspace attribution uses data_dir as
    // the workspace key for chunk #78 backend persistence; future chunks
    // integrate workspace-detector for proper per-project keying.
    let incident_workspace_key: String = data_dir.to_string_lossy().to_string();
    let incident_broadcast = Arc::new(IncidentLifecycleBroadcast::new());
    let incident_persistence: Option<Arc<dyn IncidentPersistence>> =
        corpus_writer.as_ref().map(|w| {
            Arc::new(CorpusIncidentPersistence::new(Arc::clone(w))) as Arc<dyn IncidentPersistence>
        });
    let restored_incidents: Vec<triage::contract::Incident> = incident_persistence
        .as_ref()
        .and_then(|p| match p.load_active_incidents(&incident_workspace_key) {
            Ok(v) => Some(v),
            Err(e) => {
                tracing::warn!(
                    target: triage::contract::TARGET_INCIDENT_PERSIST_ERROR,
                    error_category = e.error_category(),
                    persist_kind = "incident_boot_restore",
                    "incident corpus restore failed at boot; starting with empty registry",
                );
                None
            }
        })
        .unwrap_or_default();
    let restored_incident_count = restored_incidents.len() as u64;
    let incident_registry: Arc<dyn IncidentRegistry> =
        Arc::new(InMemoryIncidentRegistry::from_persisted(restored_incidents));
    tracing::info!(
        target: "triage.incident.corpus_restore",
        kind = "incident",
        restored_incident_count = restored_incident_count,
        "incident corpus restore",
    );
    let incidents_impl = incident_persistence.as_ref().map(|p| {
        IncidentsApiImpl::new(
            Arc::clone(&incident_registry),
            Arc::clone(&incident_broadcast),
            Arc::clone(p),
            incident_workspace_key.clone(),
        )
    });

    // Chunk #69 Phase B Session 4 — Drain miner construction с corpus-backed
    // persistence. `CorpusDrainPersistence` wraps the writer trait object
    // via trait-in-lower-crate pattern (per session-learnings 2026-05-16);
    // serializes DrainState via bincode → AES-256-GCM cell encrypt →
    // `pipeline_metrics(metric_name="drain_template_tree", layer="l1c")`.
    // Boot is non-fatal: if corpus_writer is None the miner runs
    // in-memory-only; if `load_from_persistence` fails, we log + proceed
    // (template tree resets к empty). Settings-driven config (Step 14)
    // lands в Session 5 alongside the SettingsModalForm Drain UI.
    let drain_persistence: Option<Arc<dyn buffer::DrainPersistence>> =
        corpus_writer.as_ref().map(|writer| {
            Arc::new(CorpusDrainPersistence::new(Arc::clone(writer)))
                as Arc<dyn buffer::DrainPersistence>
        });
    // Chunk #69 Phase B Session 5 — Settings-driven Drain knobs. Load
    // Settings from disk; if file missing OR parse-error, silent fallback
    // к defaults via `Settings::load_from_data_dir` per chunk #30 boot-
    // load precedent. Settings.drain_* fields shape DrainConfig before
    // DrainMiner construction; runtime config changes (via Settings UI)
    // persist но do NOT mutate the live miner — restart required (P-055).
    // similarity_x100 is the percent-scaled integer storage form (50 ↔ 0.50).
    let boot_settings = Settings::load_from_data_dir(&data_dir);
    let mut drain_config = DrainConfig::default_config();
    drain_config.depth = boot_settings.drain_depth;
    drain_config.similarity = boot_settings.drain_similarity_x100 as f32 / 100.0;
    drain_config.max_clusters = boot_settings.drain_max_clusters as usize;
    let drain_miner = Arc::new(DrainMiner::new(drain_config, drain_persistence));
    match drain_miner.load_from_persistence() {
        Ok(true) => {
            tracing::info!(
                target: "drain.persistence.load.ok",
                template_count = drain_miner.template_count(),
                "drain template tree rehydrated from corpus",
            );
        }
        Ok(false) => {
            // No prior snapshot OR no persistence configured — fresh tree.
        }
        Err(e) => {
            tracing::warn!(
                target: "drain.persistence.unavailable",
                reason = %e,
                "Drain persistence unavailable; in-memory-only template tree this boot",
            );
        }
    }
    let diagnostics_impl = DiagnosticsApiImpl::new(Arc::clone(&drain_miner));

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
                .merge(services_impl.clone().into_handler())
                .merge(diagnostics_impl.clone().into_handler())
                .merge(model_impl.clone().into_handler());
            let base = match storage_impl.as_ref() {
                Some(s) => base.merge(s.clone().into_handler()),
                None => base,
            };
            let base = match incidents_impl.as_ref() {
                Some(i) => base.merge(i.clone().into_handler()),
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
                .merge(services_impl.clone().into_handler())
                .merge(diagnostics_impl.clone().into_handler())
                .merge(model_impl.clone().into_handler());
            let base = match storage_impl.as_ref() {
                Some(s) => base.merge(s.clone().into_handler()),
                None => base,
            };
            let base = match incidents_impl.as_ref() {
                Some(i) => base.merge(i.clone().into_handler()),
                None => base,
            };
            #[cfg(feature = "mcp-server")]
            let base = base.merge(mcp_impl.clone().into_handler());
            base
        }
    };

    // Chunk #70: clone baseline_persistence option for use inside the
    // `.setup(move ...)` closure (the persist loop spawn site). The outer
    // binding is no longer needed after this point.
    let baseline_persistence_for_persist = baseline_persistence.clone();

    // Chunk #71: same pattern for lifecycle + storm persistence.
    let lifecycle_persistence_for_persist = lifecycle_persistence.clone();
    let storm_persistence_for_persist = storm_persistence.clone();
    let corpus_basename_for_persist: String = corpus_db_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("corpus.db")
        .to_string();

    // Chunk #78: capture incident persistence + registry + broadcast +
    // workspace key for the setup closure spawn site (persist loop + auto-
    // resolution observer loop). Cloning Option<Arc<...>> is cheap (Arc).
    let incident_persistence_for_persist = incident_persistence.clone();
    let incident_registry_for_persist = Arc::clone(&incident_registry);
    let incident_broadcast_for_observe = Arc::clone(&incident_broadcast);
    let incident_workspace_for_persist = incident_workspace_key.clone();

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
                        // Chunk #69 Phase B Session 3 — Drain miner threaded
                        // into the buffer consumer hot path. The same Arc is
                        // shared with `diagnostics_impl` so TauRPC reads see
                        // the live tree. Persistence is None this session;
                        // Session 4 wires `CorpusDrainPersistence` through
                        // `DrainMiner::new`'s persistence slot via the
                        // trait-in-lower-crate `DrainPersistence` adapter.
                        Some(Arc::clone(&drain_miner)),
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
            // Chunk #62 + #70 — baseline corpus periodic persist (60s default)
            // + attention cue emitter background tick (1s cadence reading
            // all BaselineState trackers, evaluating thresholds, emitting
            // cues to broadcast + cadence-triggers channel).
            //
            // Chunk #70: spawn the persist loop ONLY when baseline_persistence
            // is Some (corpus available at boot). None ⇒ in-memory-only this
            // session per the boot-warn emitted above; no persist task needed.
            if let Some(persistence) = baseline_persistence_for_persist.as_ref() {
                tauri::async_runtime::spawn(run_persist_loop(
                    Arc::clone(&baseline_state),
                    Arc::clone(persistence),
                    std::time::Duration::from_nanos(DEFAULT_PERSIST_INTERVAL_NANOS as u64),
                ));
            }
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
            // Chunk #80 — cadence coordinator + three-tier triggering.
            // CadenceConfig reads from persisted Settings (validated by
            // Settings::validate at load time); coordinator reads once at
            // spawn per pulse-v0_2_0-route §80 (hot-reload deferred к chunk
            // #94). Spawn ONLY when buffer_conn is Some (cadence_sql_runner
            // has а real DuckDB handle); else log warn + skip.
            let cadence_config = Arc::new(
                CadenceConfig::try_new(
                    settings.cadence_baseline_seconds,
                    settings.cadence_accelerated_seconds,
                    settings.cadence_reflection_seconds,
                    settings.cadence_tier2_acceleration_enabled,
                )
                .expect("cadence config validated by Settings::validate"),
            );
            tracing::info!(
                target: "cadence.config.load",
                baseline_seconds = cadence_config.baseline_seconds as u64,
                accelerated_seconds = cadence_config.accelerated_seconds as u64,
                reflection_seconds = cadence_config.reflection_seconds as u64,
                tier2_acceleration_enabled = cadence_config.tier2_acceleration_enabled,
                "cadence config loaded at boot",
            );
            if let Some(sql_runner) = cadence_sql_runner.as_ref() {
                tauri::async_runtime::spawn(start_cadence_coordinator(
                    Arc::clone(sql_runner),
                    Arc::clone(&cadence_event_broadcast),
                    Arc::clone(&cue_broadcast),
                    Arc::clone(&cadence_channel),
                    Arc::clone(&hardware_profile),
                    Arc::clone(&cadence_config),
                ));
            } else {
                tracing::warn!(
                    target: "cadence.config.safety_floor",
                    "cadence coordinator disabled — buffer connection unavailable",
                );
            }
            // Chunk #81 — L3 digest assembler runtime. Constructs the
            // assembler if both sql_runner + corpus_writer are available
            // (degrades gracefully when buffer or corpus unavailable per
            // chunk #80 / chunk #68 substrates). Spawns two tasks:
            // - cadence subscriber: invokes assembler.assemble() per
            //   CadenceEvent
            // - digest persister: writes each assembled digest к
            //   corpus.digest_archive via CorpusWriter::save_digest
            let digest_broadcast: Arc<DigestBroadcast> = Arc::new(DigestBroadcast::new());
            let digest_queue: Arc<std::sync::Mutex<LwwQueue>> =
                Arc::new(std::sync::Mutex::new(LwwQueue::new()));
            if let (Some(sql_runner), Some(corpus_writer_handle)) =
                (cadence_sql_runner.as_ref(), corpus_writer.as_ref())
            {
                match pulse_app::digest_runtime::build_assembler(
                    Arc::clone(sql_runner),
                    Arc::clone(&incident_registry_for_persist),
                    Arc::clone(&digest_broadcast),
                    Arc::clone(&digest_queue),
                ) {
                    Ok(assembler) => {
                        let project_context = std::env::current_dir()
                            .ok()
                            .and_then(|cwd| workspace_detector::detect::detect(&cwd).ok())
                            .as_ref()
                            .map(pulse_app::digest_runtime::workspace_to_digest_context)
                            .unwrap_or_default();
                        pulse_app::digest_runtime::spawn_cadence_subscriber(
                            Arc::clone(&cadence_event_broadcast),
                            Arc::clone(&assembler),
                            project_context,
                        );
                        pulse_app::digest_runtime::spawn_digest_persister(
                            Arc::clone(&digest_broadcast),
                            Arc::clone(corpus_writer_handle),
                        );
                        // Chunk #83 — L4 inference subscriber. Subscribes
                        // to `pulse://stream/digests` (same source as the
                        // persister above) and invokes LlmInferenceRunner
                        // per digest. With the chunk #82 stub runner this
                        // returns ModelNotConfigured + counts а
                        // runtime_error metric per inference; future
                        // chunks с actual mistralrs binding produce real
                        // L4 outputs that downstream chunks (#86+) wire
                        // into incident records.
                        pulse_app::inference_runtime::spawn_l4_inference_subscriber(
                            Arc::clone(&digest_broadcast),
                            Arc::clone(&llm_runner),
                        );
                        pulse_app::inference_runtime::spawn_l4_queue_depth_heartbeat(|| 0);
                        tracing::info!(
                            target: "digest.runtime.boot",
                            "digest assembler spawned (cadence subscriber + persister + L4 inference subscriber)",
                        );
                    }
                    Err(e) => {
                        tracing::warn!(
                            target: "digest.runtime.boot",
                            error_message = %e,
                            "digest assembler init failed; L3 disabled",
                        );
                    }
                }
            } else {
                tracing::warn!(
                    target: "digest.runtime.boot",
                    "digest assembler disabled — buffer connection or corpus unavailable",
                );
            }
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
            // Chunk #71 — lifecycle + storm persistence periodic loops
            // (60s default, mirrors baseline cadence). Spawn ONLY when
            // corpus available at boot; None ⇒ in-memory-only this
            // session per the boot-warn emitted above; no persist task
            // needed. Skip-first-tick + 60s actual cadence preserves
            // chunk #62/#63/#66 heartbeat convention.
            if let Some(persistence) = lifecycle_persistence_for_persist.as_ref() {
                tauri::async_runtime::spawn(run_lifecycle_persist_loop(
                    Arc::clone(&lifecycle_registry),
                    Arc::clone(persistence),
                    std::time::Duration::from_secs(DEFAULT_LIFECYCLE_PERSIST_INTERVAL_SECS),
                    corpus_basename_for_persist.clone(),
                ));
            }
            if let Some(persistence) = storm_persistence_for_persist.as_ref() {
                tauri::async_runtime::spawn(run_storm_persist_loop(
                    Arc::clone(&storm_detector),
                    Arc::clone(persistence),
                    std::time::Duration::from_secs(DEFAULT_STORM_PERSIST_INTERVAL_SECS),
                    corpus_basename_for_persist.clone(),
                ));
            }
            // Chunk #78 — incident persistence + auto-resolution observer.
            // Spawn ONLY when corpus available at boot; None ⇒ in-memory-only
            // this session per the boot-restore-failed warn already emitted.
            // The persist loop (60s default cadence) catches up corpus state
            // with the in-memory registry; the auto-resolution observer
            // (30s tick) evaluates Active+Acknowledged incidents for the
            // 120s no-reemission window (capability P-022).
            if let Some(persistence) = incident_persistence_for_persist.as_ref() {
                tauri::async_runtime::spawn(run_incident_persist_loop(
                    Arc::clone(&incident_registry_for_persist),
                    Arc::clone(persistence),
                    vec![incident_workspace_for_persist.clone()],
                    DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS,
                ));
                let observer = AutoResolveObserver::new(
                    Arc::clone(&incident_registry_for_persist),
                    Arc::clone(persistence),
                    Arc::clone(&incident_broadcast_for_observe),
                );
                tauri::async_runtime::spawn(run_auto_resolution_loop(observer));
            }
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
                // chunk #69 Phase B Session 7+: thread the DrainMiner ref
                // into the buffer.tick heartbeat task so drain_template_count
                // + drain_lru_evictions_since_tick fields surface AND the
                // `metric.pipeline.l1c.drain_template_count_total` event
                // emits per tick (15s sibling cadence).
                Some(Arc::clone(&drain_miner)),
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
    use triage::contract::UnknownHardwareProfile;

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

        // Chunk #69 Phase B Session 3: DiagnosticsApiImpl participates in
        // the emit so bindings.ts ARGS_MAP includes diagnostics.template_distribution
        // (quadruple-binding 4th slot per .claude/rules/security.md Session
        // Additions 2026-05-12). In-memory miner with no persistence keeps
        // the test hermetic.
        let diagnostics_miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
        let diagnostics_impl = DiagnosticsApiImpl::new(diagnostics_miner);

        // Chunk #78: IncidentsApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes incidents.list_active / acknowledge / mark_resolved
        // (quadruple-binding 4th slot per .claude/rules/security.md Session
        // Additions 2026-05-12). In-memory corpus + fresh registry + broadcast
        // keep the test hermetic.
        let incident_keychain: Arc<dyn corpus::contract::KeychainBackend> =
            Arc::new(corpus::contract::FakeKeychainBackend::new());
        let incident_corpus = corpus::contract::Corpus::open_in_memory(incident_keychain)
            .expect("in-memory incident corpus opens with fake keychain");
        let incident_corpus_arc = Arc::new(incident_corpus);
        let incident_writer: Arc<dyn corpus::contract::CorpusWriter> =
            Arc::clone(&incident_corpus_arc) as Arc<dyn corpus::contract::CorpusWriter>;
        let incident_persistence_test: Arc<dyn IncidentPersistence> =
            Arc::new(CorpusIncidentPersistence::new(incident_writer));
        let incident_registry_test: Arc<dyn IncidentRegistry> =
            Arc::new(InMemoryIncidentRegistry::new());
        let incident_broadcast_test = Arc::new(IncidentLifecycleBroadcast::new());
        let incidents_impl = IncidentsApiImpl::new(
            incident_registry_test,
            incident_broadcast_test,
            incident_persistence_test,
            "bindings-test-workspace".to_string(),
        );

        // Chunk #82: ModelApiImpl participates in the emit so bindings.ts
        // ARGS_MAP includes model.current_profile (5-place binding 5th slot
        // per .claude/rules/security.md Session Additions 2026-05-12).
        // Test uses UnknownHardwareProfile (deterministic) + а stub
        // LlamaCliInference (status: Error, identity: None — env vars unset
        // in test process so binary_path + model_path both resolve to None).
        let model_hardware: Arc<dyn HardwareProfileSource> = Arc::new(UnknownHardwareProfile);
        let model_status_bcast_test = interpretation::broadcast::ModelStatusBroadcast::new();
        let model_runner_test: Arc<dyn interpretation::contract::LlmInferenceRunner> =
            Arc::new(LlamaCliInference::new(
                interpretation::contract::ModelTier::Primary,
                model_hardware.current_profile(),
                model_status_bcast_test,
            ));
        let model_impl = ModelApiImpl::new(model_runner_test, model_hardware);

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
                .merge(storage_impl.into_handler())
                .merge(diagnostics_impl.into_handler())
                .merge(incidents_impl.into_handler())
                .merge(model_impl.into_handler());
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
