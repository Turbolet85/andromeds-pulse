//! The one engine boot both programs call.
//!
//! `init_process` starts what a process keeps of itself (log sink, at-exit
//! hook, signal listener, start instant, pid file). `start` builds the engine
//! state and spawns its tasks: the two loopback OTLP receivers, the ring
//! buffer, the detectors, the cadence and digest runtime, the corpus adapters
//! and the engine's heartbeat ticks. Neither entry point wires an engine part
//! itself, and this file names no window framework — both pinned by
//! `pulse-app/tests/unit_engine_boot_seam.rs`.
//!
//! Both functions need an entered tokio runtime: the signal listener returns
//! silently without one, and `start` spawns with `tokio::spawn`.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};
use std::{env, fs};

use buffer::{
    BroadcastSenders, BufferState, DrainConfig, DrainMiner, create_schema, run_consumer,
    run_retention,
};
use corpus::contract::{Corpus, CorpusReader, CorpusWriter, KeychainBackend, OsKeychainBackend};
use duckdb::Connection;
use ingest::channel::{IngestSender, build_channel};
use ingest::connection::{self, ConnectionBroadcast, ReceiverBindStatus};
use ingest::contract::{Error as IngestError, OtlpPort};
use ingest::observer::SpanObserver;
use ingest::state::IngestState;
use interpretation::contract::LlmInferenceRunner;
use interpretation::degraded_mode::DegradedModeStatus;
use tracing_error::SpanTrace;
use triage::contract::{
    AttentionCueBroadcast, BaselinePersistence, CadenceConfig, CadenceEventBroadcast,
    CadenceTriggerChannel, DEFAULT_AUTONOMOUS_THRESHOLD, DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
    DEFAULT_HEARTBEAT_INTERVAL, DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS,
    DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL, DEFAULT_LIFECYCLE_PERSIST_INTERVAL_SECS,
    DEFAULT_PERSIST_INTERVAL_NANOS, DEFAULT_SERVICE_COUNT_CAP, DEFAULT_STORM_PERSIST_INTERVAL_SECS,
    DEFAULT_STORM_WINDOW_SECONDS, DEFAULT_SUGGESTED_THRESHOLD, DigestBroadcast,
    DigestTriggerBroadcast, DurableActiveIncidents, GenerationDamper, HardwareProfileSource,
    InMemoryIncidentRegistry, InMemoryServiceRegistry, Incident, IncidentLifecycleBroadcast,
    IncidentPersistence, IncidentRegistry, LifecyclePersistence, LwwQueue, RestartDetector,
    RestartEventBroadcast, RetryStormDetector, ServiceLifecycleBroadcast, ServiceLifecycleState,
    ServiceRegistry, SqlQueryRunner, StormPersistence, SuppressionState,
    TARGET_BASELINE_PERSIST_ERROR, TARGET_INCIDENT_PERSIST_ERROR, TARGET_LIFECYCLE_CORPUS_RESTORE,
    TARGET_LIFECYCLE_PERSIST_ERROR, TARGET_PATTERN_STORM_CORPUS_RESTORE,
    TARGET_PATTERN_STORM_PERSIST_ERROR, Thresholds, TriageSqlState, bootstrap_state,
    run_incident_persist_loop, run_lifecycle_persist_loop, run_persist_loop,
    run_storm_persist_loop, start_cadence_coordinator, start_emitter, start_lifecycle_heartbeat,
    start_restart_detector, start_storm_detector,
};
use ui_bridge::Settings;
use ui_bridge::health::{
    BindStatus, BufferConnectionStatus, HeartbeatState, IngestChannelStatus, record_start,
    register_heartbeat_state,
};

use crate::baseline_observer::BaselineObserverAdapter;
use crate::baseline_persistence::{CorpusBaselinePersistence, migrate_legacy_baseline_if_present};
use crate::cadence_runner::CadenceSqlRunner;
use crate::config_router;
use crate::connection_router::HeartbeatBindStatus;
use crate::corpus_retrieval::{CorpusBackedIncidentSource, run_pipeline_metrics_purge};
use crate::degraded_mode_runtime::LocalDegradedModeStatus;
use crate::digest_runtime;
use crate::discovery_observer::DiscoveryObserverAdapter;
use crate::drain_persistence::CorpusDrainPersistence;
use crate::incident_observer::{AutoResolveObserver, run_auto_resolution_loop};
use crate::incident_persistence::CorpusIncidentPersistence;
use crate::inference_runtime;
use crate::lifecycle_persistence::CorpusLifecyclePersistence;
use crate::reevaluation::{LiveReevaluator, RecentWindowReevaluator};
use crate::restart_observer::{CompositeSpanObserver, RestartObserverAdapter};
use crate::storm_observer::StormObserverAdapter;
use crate::storm_persistence::CorpusStormPersistence;
use crate::{heartbeat, observability};

#[doc(hidden)]
pub const ENV_OTLP_GRPC_PORT: &str = "ANDROMEDA_PULSE_OTLP_GRPC_PORT";
#[doc(hidden)]
pub const ENV_OTLP_HTTP_PORT: &str = "ANDROMEDA_PULSE_OTLP_HTTP_PORT";
#[doc(hidden)]
pub const ENV_RETENTION_SECONDS: &str = "ANDROMEDA_PULSE_RETENTION_SECONDS";

// Retention bounds per security plan §Input Validation row "Configuration values":
// reject out-of-range rather than silently clamping. Default fallback per arch
// §Inherited Defaults (300–600s default range).
#[doc(hidden)]
pub const RETENTION_SECONDS_MIN: u64 = 60;
#[doc(hidden)]
pub const RETENTION_SECONDS_MAX: u64 = 86_400;
#[doc(hidden)]
pub const RETENTION_SECONDS_DEFAULT: u64 = 600;

const CORPUS_KEY_SERVICE: &str = "com.andromeda.pulse";

#[doc(hidden)]
pub const TARGET_BOOT_ENGINE: &str = "app.boot.engine";

#[doc(hidden)]
pub fn resolve_retention_seconds() -> u64 {
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

#[doc(hidden)]
pub fn resolve_grpc_port() -> Result<OtlpPort, IngestError> {
    resolve_port(ENV_OTLP_GRPC_PORT, ingest::grpc::DEFAULT_GRPC_PORT)
}

#[doc(hidden)]
pub fn resolve_http_port() -> Result<OtlpPort, IngestError> {
    resolve_port(ENV_OTLP_HTTP_PORT, ingest::http::DEFAULT_HTTP_PORT)
}

#[doc(hidden)]
pub fn resolve_port(env_var_name: &'static str, default: u16) -> Result<OtlpPort, IngestError> {
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

#[doc(hidden)]
pub fn resolve_data_dir() -> PathBuf {
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

#[doc(hidden)]
pub fn write_pid_file(data_dir: &Path) {
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
            run_dir_basename = observability::log_basename(&canonical_run).unwrap_or("unknown"),
            data_dir_basename = observability::log_basename(&canonical_data).unwrap_or("unknown"),
            "run dir escaped data dir; refusing to write PID",
        );
        return;
    }
    let pid = std::process::id();
    if let Err(e) = fs::write(&pid_path, pid.to_string()) {
        tracing::warn!(target: "app.boot.pid", error = %e, "failed to write PID file");
        return;
    }
    tracing::info!(
        target: "app.boot.pid",
        pid = pid,
        path_basename = observability::log_basename(&pid_path).unwrap_or("unknown"),
        "PID file written",
    );
}

/// Publish the resolved incident workspace key so the MCP stdio sidecar —
/// a separate process that shares only the data dir — filters incidents by
/// the identity this process stamps them with. Non-fatal: on failure the
/// sidecar falls back to `data_dir`, which is the pre-publication behaviour.
///
/// Basename only in the log line (obs-plan §5 Vector 5).
#[doc(hidden)]
pub fn publish_workspace_key_for_sidecar(data_dir: &Path, key: &str) {
    match workspace_detector::contract::publish_workspace_key(data_dir, key) {
        Ok(()) => tracing::info!(
            target: "app.boot.workspace_key",
            workspace_root_basename = Path::new(key)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown"),
            key_bytes = key.len(),
            "workspace key published for sidecar",
        ),
        Err(e) => tracing::warn!(
            target: "app.boot.workspace_key",
            error_category = "publish_failed",
            error_detail = %e,
            "workspace key publish failed; sidecar will fall back to data dir",
        ),
    }
}

#[doc(hidden)]
pub fn init_buffer(heartbeat_state: &Arc<HeartbeatState>) -> Option<Arc<Mutex<Connection>>> {
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

/// What a process keeps of itself, in this order: the log sink and its guard,
/// the at-exit hook, the Unix signal listener, the start instant, the pid
/// file. Called with the tokio runtime entered.
pub fn init_process(data_dir: &Path) {
    observability::hold_log_guard(observability::init(data_dir));
    observability::install_exit_hook();
    #[cfg(unix)]
    observability::install_signal_listener();
    record_start();
    write_pid_file(data_dir);
}

/// The corpus key backend both programs pass: the OS credential store.
pub fn os_key_backend() -> Arc<dyn KeychainBackend> {
    Arc::new(OsKeychainBackend::new(CORPUS_KEY_SERVICE))
}

/// Which program boots the engine. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Program {
    Window,
    Console,
}

impl Program {
    pub fn label(self) -> &'static str {
        match self {
            Program::Window => "window",
            Program::Console => "console",
        }
    }
}

/// What kind of runner sits in the interpretation seat. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatKind {
    Model,
    Deterministic,
}

pub struct InterpretationSeat {
    pub runner: Arc<dyn LlmInferenceRunner>,
    pub kind: SeatKind,
}

/// A receiver port is `None` when its variable was rejected; the receiver then
/// does not start. The bind address is always loopback and is not configurable.
pub struct EngineConfig {
    pub data_dir: PathBuf,
    pub grpc_port: Option<OtlpPort>,
    pub http_port: Option<OtlpPort>,
    pub retention_seconds: u64,
}

impl EngineConfig {
    /// Reads the two port variables and the retention variable. Call after
    /// `init_process`: a rejected value is reported through the log sink.
    pub fn from_env(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            grpc_port: resolve_grpc_port().ok(),
            http_port: resolve_http_port().ok(),
            retention_seconds: resolve_retention_seconds(),
        }
    }
}

pub struct EngineInputs {
    pub program: Program,
    pub key_backend: Arc<dyn KeychainBackend>,
    pub hardware_profile: Arc<dyn HardwareProfileSource>,
    /// `None` seats nothing: cues and digests are produced, no incident forms.
    pub interpretation: Option<InterpretationSeat>,
}

/// The engine state a caller's own surfaces read. The engine's tasks hold
/// their own references; dropping this stops nothing.
pub struct EngineHandles {
    pub heartbeat_state: Arc<HeartbeatState>,
    pub ingest_state: Arc<IngestState>,
    pub bind_status: Arc<dyn ReceiverBindStatus>,
    pub buffer_conn: Option<Arc<Mutex<Connection>>>,
    pub buffer_state: Arc<BufferState>,
    pub broadcast_senders: Arc<BroadcastSenders>,
    pub retention_seconds: u64,
    pub corpus_reader: Option<Arc<dyn CorpusReader>>,
    pub corpus_writer: Option<Arc<dyn CorpusWriter>>,
    pub lifecycle_registry: Arc<dyn ServiceRegistry>,
    pub lifecycle_broadcast: Arc<ServiceLifecycleBroadcast>,
    pub incident_registry: Arc<dyn IncidentRegistry>,
    pub incident_broadcast: Arc<IncidentLifecycleBroadcast>,
    pub incident_persistence: Option<Arc<dyn IncidentPersistence>>,
    pub incident_workspace_key: String,
    pub drain_miner: Arc<DrainMiner>,
    pub degraded_mode: Arc<dyn DegradedModeStatus>,
    pub reevaluator: Arc<dyn RecentWindowReevaluator>,
    pub config_handle_slot: Arc<OnceLock<config_watcher::ConfigWatchHandle>>,
    pub config_status: Arc<Mutex<config_watcher::ConfigStatus>>,
    #[doc(hidden)]
    pub digest_broadcast: Arc<DigestBroadcast>,
}

/// The engine's one boot record. WARN when nothing is seated, INFO otherwise.
#[doc(hidden)]
pub fn emit_boot_record(program: Program, seat: Option<SeatKind>) {
    let program = program.label();
    match seat {
        Some(SeatKind::Model) => tracing::info!(
            target: TARGET_BOOT_ENGINE,
            program,
            interpretation = "model",
            reason = "window_runs_model",
            "engine boot",
        ),
        Some(SeatKind::Deterministic) => tracing::info!(
            target: TARGET_BOOT_ENGINE,
            program,
            interpretation = "deterministic",
            reason = "deterministic_gate_set",
            "engine boot",
        ),
        None => tracing::warn!(
            target: TARGET_BOOT_ENGINE,
            program,
            interpretation = "none",
            reason = "deterministic_gate_unset",
            "engine boot with no interpretation seated: no incident forms until the engine keeps its own incident record (ANDROMEDA_PULSE_L4_DETERMINISTIC is unset)",
        ),
    }
}

fn loopback(port: OtlpPort) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port.value()))
}

fn unix_nanos_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

fn fresh_storm_detector() -> RetryStormDetector {
    RetryStormDetector::new(
        DEFAULT_STORM_WINDOW_SECONDS,
        DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
        DEFAULT_SUGGESTED_THRESHOLD,
        DEFAULT_AUTONOMOUS_THRESHOLD,
    )
}

/// Builds the engine state and spawns its tasks. Called once per process, with
/// the tokio runtime entered.
pub fn start(config: EngineConfig, inputs: EngineInputs) -> EngineHandles {
    let EngineConfig {
        data_dir,
        grpc_port,
        http_port,
        retention_seconds,
    } = config;
    let EngineInputs {
        program,
        key_backend,
        hardware_profile,
        interpretation,
    } = inputs;

    emit_boot_record(program, interpretation.as_ref().map(|seat| seat.kind));

    let heartbeat_state = Arc::new(HeartbeatState::new());
    register_heartbeat_state(heartbeat_state.clone());

    let ingest_state = Arc::new(IngestState::new());
    let (ingest_sender, ingest_receiver) = build_channel();
    let ingest_sender: Arc<IngestSender> = Arc::new(ingest_sender);
    heartbeat_state.record_ingest_channel(IngestChannelStatus::Ok { capacity_pct: 0.0 });

    let buffer_state = Arc::new(BufferState::new());
    let buffer_conn = init_buffer(&heartbeat_state);
    let broadcast_senders: Arc<BroadcastSenders> = Arc::new(buffer::broadcast::create());

    // Connection state machine: the poller reads `ingest_state` and the bind
    // status and broadcasts transitions on `pulse://stream/connection-state`.
    let connection_broadcast = Arc::new(ConnectionBroadcast::new());
    let bind_status: Arc<dyn ReceiverBindStatus> =
        Arc::new(HeartbeatBindStatus::new(Arc::clone(&heartbeat_state)));

    // The corpus opens at `<data_dir>/corpus/corpus.db`. Non-fatal: with no key
    // or a failed open the engine continues with the corpus absent, and every
    // adapter below runs in memory only.
    let corpus_db_path = data_dir.join("corpus").join("corpus.db");
    let corpus_arc: Option<Arc<Corpus>> = match Corpus::open(corpus_db_path.clone(), key_backend) {
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
    // One `Arc<Corpus>`, two role-separated views sharing one connection.
    let corpus_reader: Option<Arc<dyn CorpusReader>> = corpus_arc
        .as_ref()
        .map(|c| Arc::clone(c) as Arc<dyn CorpusReader>);
    let corpus_writer: Option<Arc<dyn CorpusWriter>> = corpus_arc
        .as_ref()
        .map(|c| Arc::clone(c) as Arc<dyn CorpusWriter>);

    // Retire content orphaned by an earlier key: inventory first, then purge.
    // Runs before every other corpus consumer because one undecryptable row
    // fails an entire query, so the reads below depend on it. Idempotent — a
    // corpus with nothing orphaned is a no-op.
    if let Some(c) = corpus_arc.as_ref() {
        let outcome = corpus::disposition::dispose_orphaned_content(
            c.as_ref(),
            &data_dir.join("corpus"),
            unix_nanos_now(),
        );
        corpus::disposition::emit_disposition_outcome(&outcome);
    }

    // P-041 pipeline-metrics 30-day retention purge, once per boot. The newest
    // row per (metric_name, layer) series survives, so baseline / Drain / storm
    // snapshots outlive idle gaps longer than the window. Non-fatal on failure.
    if let Some(w) = corpus_writer.as_ref() {
        run_pipeline_metrics_purge(w.as_ref(), unix_nanos_now());
    }

    let baseline_persistence: Option<Arc<dyn BaselinePersistence>> =
        corpus_writer.as_ref().map(|w| {
            Arc::new(CorpusBaselinePersistence::new(Arc::clone(w))) as Arc<dyn BaselinePersistence>
        });

    // One-shot legacy bincode migration from `<data_dir>/triage/`. Idempotent;
    // a failure preserves the legacy file for the next boot's retry.
    if let Some(persistence) = baseline_persistence.as_ref() {
        let _ = migrate_legacy_baseline_if_present(&data_dir, persistence.as_ref());
    } else {
        tracing::warn!(
            target: TARGET_BASELINE_PERSIST_ERROR,
            error_category = "corpus_unavailable_at_boot",
            duration_ms = 0_u64,
            "baseline persistence unavailable; in-memory only this session",
        );
    }

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

    let baseline_now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    // Resolved before the baseline state it configures: `Thresholds` carries
    // the cold-start window, and the state receives that same value, so the
    // silence evaluator, the emitter counters and the lifecycle registry all
    // gate on one bound.
    let thresholds = Arc::new(Thresholds::from_env());
    let baseline_state = Arc::new(bootstrap_state(
        baseline_persistence.as_deref(),
        DEFAULT_SERVICE_COUNT_CAP,
        baseline_now,
        thresholds.bootstrap_window_seconds,
    ));
    let cue_broadcast = Arc::new(AttentionCueBroadcast::new());
    let cadence_channel = Arc::new(CadenceTriggerChannel::new());

    let cadence_event_broadcast = Arc::new(CadenceEventBroadcast::new());
    // P-074 — the non-L6 digest-assembly trigger conveys the full triggering
    // cue (incl. scope_id) from the coordinator to the digest assembler, so a
    // cue-driven storm digest reaches L4 with its `attention_cues` populated and
    // the producer can create + dedup the incident. Kept separate from the
    // PII-free `pulse://stream/cadence-events` topic above.
    let cadence_digest_trigger = Arc::new(DigestTriggerBroadcast::new());
    let cadence_sql_runner: Option<Arc<dyn SqlQueryRunner>> = buffer_conn.as_ref().map(|conn| {
        let state = Arc::new(TriageSqlState::new(Arc::clone(conn)));
        Arc::new(CadenceSqlRunner::new(state)) as Arc<dyn SqlQueryRunner>
    });

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

    // Retry storm detector, restored from the corpus so the dedup window
    // survives a restart (P-018); a missing or failed snapshot starts fresh.
    let storm_detector = {
        let restore_start = Instant::now();
        let (detector, restored_fingerprint_count) = match storm_persistence.as_ref() {
            Some(p) => match p.load() {
                Ok(Some(snapshot)) => {
                    let count = snapshot.entries.len() as u64;
                    (RetryStormDetector::restore_from_snapshot(snapshot), count)
                }
                Ok(None) => (fresh_storm_detector(), 0),
                Err(e) => {
                    tracing::warn!(
                        target: TARGET_PATTERN_STORM_PERSIST_ERROR,
                        error_category = e.error_category(),
                        duration_ms = restore_start.elapsed().as_millis() as u64,
                        "storm corpus restore failed; fresh detector",
                    );
                    (fresh_storm_detector(), 0)
                }
            },
            None => (fresh_storm_detector(), 0),
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

    // Service lifecycle registry, restored from the corpus. Each restored
    // service emits a synthetic CorpusRestore event on
    // `pulse://stream/service-lifecycle` so downstream observers cascade.
    let lifecycle_broadcast = Arc::new(ServiceLifecycleBroadcast::new());
    let lifecycle_registry: Arc<dyn ServiceRegistry> = {
        let restore_start = Instant::now();
        let restored_at = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        let (registry, restored_service_count) = match lifecycle_persistence.as_ref() {
            Some(p) => match p.load_all() {
                Ok(Some(entries)) => {
                    let count = entries.len() as u64;
                    let mut event_pairs: Vec<(String, ServiceLifecycleState)> =
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
    // P-027 discovery bound: the first-sighting adapter lists a service at its
    // first span rather than at the heartbeat's first 15 s tick. It is LAST so
    // the baseline has admitted the service (cap check) in the same fan-out.
    let discovery_adapter: Arc<dyn SpanObserver> = Arc::new(DiscoveryObserverAdapter::new(
        Arc::clone(&lifecycle_registry),
        Arc::clone(&baseline_state),
        Arc::clone(&lifecycle_broadcast),
    ));
    let span_observer: Arc<dyn SpanObserver> = Arc::new(CompositeSpanObserver::new(vec![
        baseline_adapter,
        restart_adapter,
        discovery_adapter,
    ]));

    // The workspace identity is single-sourced via workspace-detector so the
    // FILTER key (the services and incidents readers, persistence) equals the
    // value incidents are STAMPED with (the producer's digest.workspace); the
    // data_dir-vs-detected-root mismatch was the P-079 defect.
    let (incident_workspace_key, digest_project_context) = {
        let detected = env::current_dir()
            .ok()
            .and_then(|cwd| workspace_detector::detect::detect(&cwd).ok());
        digest_runtime::resolve_workspace_for_incidents(detected.as_ref(), &data_dir)
    };
    publish_workspace_key_for_sidecar(&data_dir, &incident_workspace_key);
    let incident_broadcast = Arc::new(IncidentLifecycleBroadcast::new());
    // One adapter instance, two trait views (write + the reconciliation read
    // port) — the concrete Arc is the intermediate so both views share the
    // single underlying corpus connection.
    let incident_adapter: Option<Arc<CorpusIncidentPersistence>> = corpus_writer
        .as_ref()
        .map(|w| Arc::new(CorpusIncidentPersistence::new(Arc::clone(w))));
    let incident_persistence: Option<Arc<dyn IncidentPersistence>> = incident_adapter
        .as_ref()
        .map(|a| Arc::clone(a) as Arc<dyn IncidentPersistence>);
    let incident_durable: Option<Arc<dyn DurableActiveIncidents>> = incident_adapter
        .as_ref()
        .map(|a| Arc::clone(a) as Arc<dyn DurableActiveIncidents>);
    let restored_incidents: Vec<Incident> = incident_persistence
        .as_ref()
        .and_then(|p| match p.load_active_incidents(&incident_workspace_key) {
            Ok(v) => Some(v),
            Err(e) => {
                tracing::warn!(
                    target: TARGET_INCIDENT_PERSIST_ERROR,
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

    // Drain miner with corpus-backed persistence. Its knobs come from the
    // settings read at boot; a later settings change does not mutate the live
    // miner (restart required, P-055). `similarity_x100` is the percent-scaled
    // integer storage form (50 ↔ 0.50).
    let drain_persistence: Option<Arc<dyn buffer::DrainPersistence>> =
        corpus_writer.as_ref().map(|writer| {
            Arc::new(CorpusDrainPersistence::new(Arc::clone(writer)))
                as Arc<dyn buffer::DrainPersistence>
        });
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
        Ok(false) => {}
        Err(e) => {
            tracing::warn!(
                target: "drain.persistence.unavailable",
                reason = %e,
                "Drain persistence unavailable; in-memory-only template tree this boot",
            );
        }
    }

    // One shared degraded-mode FSM: the interpretation subscriber, the backoff
    // heartbeat and the caller's manual-override surface all read it.
    let degraded_mode: Arc<dyn DegradedModeStatus> = Arc::new(LocalDegradedModeStatus::new());

    // Configuration hot-reload: the watcher publishes a validated `Settings` on
    // a watch channel; a fan-out maps it to the per-consumer channels (cadence,
    // lifecycle) so consumers re-read prospectively.
    let config_event_broadcast = Arc::new(config_watcher::ConfigEventBroadcast::new());
    let config_status = Arc::new(Mutex::new(config_watcher::ConfigStatus::default()));
    let config_handle_slot: Arc<OnceLock<config_watcher::ConfigWatchHandle>> =
        Arc::new(OnceLock::new());
    let (config_settings_tx, config_settings_rx) =
        tokio::sync::watch::channel(boot_settings.clone());
    let (cadence_cfg_tx, cadence_cfg_rx) =
        tokio::sync::watch::channel(config_router::settings_to_cadence_config(&boot_settings));
    let (lifecycle_thresh_tx, lifecycle_thresh_rx) = tokio::sync::watch::channel(
        config_router::settings_to_lifecycle_thresholds(&boot_settings),
    );
    let reevaluator: Arc<dyn RecentWindowReevaluator> = Arc::new(LiveReevaluator::new(
        Arc::clone(&lifecycle_registry),
        Arc::clone(&lifecycle_broadcast),
        Arc::clone(&baseline_state),
        lifecycle_thresh_rx.clone(),
    ));

    let grpc_addr = match grpc_port {
        Some(p) => Some(loopback(p)),
        None => {
            heartbeat_state.record_otlp_grpc_bind(BindStatus::Failed("invalid_port".to_string()));
            tracing::error!(
                target: "app.boot.otlp.grpc.bind",
                reason = "invalid_port",
                "OTLP gRPC port validation rejected env-var override; receiver will not start"
            );
            None
        }
    };
    let http_addr = match http_port {
        Some(p) => Some(loopback(p)),
        None => {
            heartbeat_state.record_otlp_http_bind(BindStatus::Failed("invalid_port".to_string()));
            tracing::error!(
                target: "app.boot.otlp.http.bind",
                reason = "invalid_port",
                "OTLP HTTP port validation rejected env-var override; receiver will not start"
            );
            None
        }
    };

    match config_watcher::start_config_watcher(
        &data_dir,
        config_settings_tx,
        config_event_broadcast,
        Arc::clone(&config_status),
    ) {
        Ok((config_handle, config_task)) => {
            let _ = config_handle_slot.set(config_handle);
            tokio::spawn(config_task.run());
            let mut settings_rx = config_settings_rx;
            tokio::spawn(async move {
                while settings_rx.changed().await.is_ok() {
                    let s = settings_rx.borrow_and_update().clone();
                    let _ = cadence_cfg_tx.send(config_router::settings_to_cadence_config(&s));
                    let _ = lifecycle_thresh_tx
                        .send(config_router::settings_to_lifecycle_thresholds(&s));
                }
            });
            tracing::info!(
                target: "config.watcher.boot",
                "config hot-reload watcher started",
            );
        }
        Err(e) => {
            tracing::warn!(
                target: "config.watcher.boot",
                error_category = config_router::config_watch_error_category(&e),
                "config hot-reload watcher disabled this boot",
            );
        }
    }

    match buffer_conn.as_ref() {
        Some(conn) => {
            tokio::spawn(run_consumer(
                ingest_receiver,
                Arc::clone(conn),
                Arc::clone(&buffer_state),
                Arc::clone(&broadcast_senders),
                Arc::clone(&span_observer),
                fingerprint_observer.clone(),
                Some(Arc::clone(&drain_miner)),
            ));
            tokio::spawn(run_retention(
                Arc::clone(conn),
                Arc::clone(&buffer_state),
                retention_seconds,
            ));
        }
        None => {
            // Buffer init failed; the health envelope is already degraded. The
            // receiver is drained so the OTLP path does not saturate.
            //
            // Announce it: draining is indistinguishable from a healthy
            // path in ingest's own counters (they count at receipt), so
            // without this line a dead fingerprint feed and absent
            // DuckDB appends can only be inferred from missing signal.
            tracing::warn!(
                target: "app.boot.buffer.degraded",
                reason = "buffer_conn_absent",
                consequence = "duckdb_appends_and_fingerprint_feed_inert",
                "buffer connection absent; ingest batches are drained and discarded this boot",
            );
            tokio::spawn(async move {
                let mut rx = ingest_receiver;
                while rx.recv().await.is_some() {}
            });
        }
    }
    if let Some(grpc_addr) = grpc_addr {
        let grpc_announcer = Arc::clone(&heartbeat_state);
        let grpc_state = Arc::clone(&ingest_state);
        let grpc_sender = Arc::clone(&ingest_sender);
        tokio::spawn(async move {
            match ingest::grpc::try_bind(grpc_addr).await {
                Ok(listener) => {
                    grpc_announcer.record_otlp_grpc_bind(BindStatus::Ok);
                    tracing::info!(
                        target: "app.boot.otlp.grpc.bind",
                        bind_address = %grpc_addr,
                        "OTLP gRPC receiver bound"
                    );
                    if let Err(e) = ingest::grpc::serve_on(listener, grpc_state, grpc_sender).await
                    {
                        let reason = format!("{}", e);
                        grpc_announcer.record_otlp_grpc_bind(BindStatus::Failed(reason.clone()));
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
    }
    if let Some(http_addr) = http_addr {
        let http_announcer = Arc::clone(&heartbeat_state);
        let http_state = Arc::clone(&ingest_state);
        let http_sender = Arc::clone(&ingest_sender);
        tokio::spawn(async move {
            match ingest::http::try_bind(http_addr).await {
                Ok(listener) => {
                    http_announcer.record_otlp_http_bind(BindStatus::Ok);
                    tracing::info!(
                        target: "app.boot.otlp.http.bind",
                        bind_address = %http_addr,
                        "OTLP HTTP receiver bound"
                    );
                    if let Err(e) = ingest::http::serve_on(listener, http_state, http_sender).await
                    {
                        let reason = format!("{}", e);
                        http_announcer.record_otlp_http_bind(BindStatus::Failed(reason.clone()));
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
    }
    // Connection-state detector loop: broadcasts on
    // `pulse://stream/connection-state` only when the state changes. The 15 s
    // `connection.tick` below is a separate concern.
    tokio::spawn(connection::start_poller(
        Arc::clone(&ingest_state),
        Arc::clone(&bind_status),
        Arc::clone(&connection_broadcast),
    ));
    if let Some(persistence) = baseline_persistence.as_ref() {
        tokio::spawn(run_persist_loop(
            Arc::clone(&baseline_state),
            Arc::clone(persistence),
            Duration::from_nanos(DEFAULT_PERSIST_INTERVAL_NANOS as u64),
        ));
    }
    tokio::spawn(start_emitter(
        Arc::clone(&baseline_state),
        Arc::clone(&cue_broadcast),
        Arc::clone(&cadence_channel),
        Arc::clone(&thresholds),
        Arc::clone(&restart_broadcast),
        Arc::clone(&suppression_state),
    ));
    // The two detector ticks; detection itself happens inline on the span and
    // fingerprint observer hot paths.
    tokio::spawn(start_restart_detector(
        Arc::clone(&restart_detector),
        DEFAULT_HEARTBEAT_INTERVAL,
    ));
    tokio::spawn(start_storm_detector(
        Arc::clone(&storm_detector),
        DEFAULT_HEARTBEAT_INTERVAL,
    ));
    let cadence_config = Arc::new(
        CadenceConfig::try_new(
            boot_settings.cadence_baseline_seconds,
            boot_settings.cadence_accelerated_seconds,
            boot_settings.cadence_reflection_seconds,
            boot_settings.cadence_tier2_acceleration_enabled,
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
        tokio::spawn(start_cadence_coordinator(
            Arc::clone(sql_runner),
            Arc::clone(&cadence_event_broadcast),
            Arc::clone(&cadence_digest_trigger),
            Arc::clone(&cue_broadcast),
            Arc::clone(&cadence_channel),
            hardware_profile,
            Arc::clone(&cadence_config),
            cadence_cfg_rx,
        ));
    } else {
        tracing::warn!(
            target: "cadence.config.safety_floor",
            "cadence coordinator disabled — buffer connection unavailable",
        );
    }
    // The digest assembler needs both the SQL runner and the corpus. Its
    // cadence subscriber and persister run whatever sits in the interpretation
    // seat; the interpretation subscriber and its two heartbeats run only
    // behind a seated runner.
    let digest_broadcast: Arc<DigestBroadcast> = Arc::new(DigestBroadcast::new());
    let digest_queue: Arc<Mutex<LwwQueue>> = Arc::new(Mutex::new(LwwQueue::new()));
    if let (Some(sql_runner), Some(corpus_writer_handle)) =
        (cadence_sql_runner.as_ref(), corpus_writer.as_ref())
    {
        match digest_runtime::build_assembler(
            Arc::clone(sql_runner),
            Arc::clone(&incident_registry),
            Arc::clone(&digest_broadcast),
            Arc::clone(&digest_queue),
            Arc::new(CorpusBackedIncidentSource::new(Arc::clone(
                corpus_writer_handle,
            ))),
        ) {
            Ok(assembler) => {
                digest_runtime::spawn_cadence_subscriber(
                    Arc::clone(&cadence_digest_trigger),
                    Arc::clone(&assembler),
                    digest_project_context,
                );
                digest_runtime::spawn_digest_persister(
                    Arc::clone(&digest_broadcast),
                    Arc::clone(corpus_writer_handle),
                );
                match interpretation {
                    Some(seat) => {
                        // Shared by the subscriber (the unchanged-input gate)
                        // and the backoff heartbeat (cumulative counters).
                        let generation_damper = Arc::new(GenerationDamper::new());
                        if let Some(persistence) = incident_persistence.as_ref() {
                            inference_runtime::spawn_l4_inference_subscriber(
                                Arc::clone(&digest_broadcast),
                                seat.runner,
                                Arc::clone(&degraded_mode),
                                Arc::clone(&incident_registry),
                                Arc::clone(persistence),
                                Arc::clone(&generation_damper),
                            );
                        } else {
                            tracing::warn!(
                                target: "digest.runtime.boot",
                                "L4 inference subscriber disabled — corpus unavailable; resolution-summary attachment path inactive this boot",
                            );
                        }
                        inference_runtime::spawn_l4_queue_depth_heartbeat(|| 0);
                        inference_runtime::spawn_l4_backoff_remaining_heartbeat(
                            Arc::clone(&degraded_mode),
                            Arc::clone(&generation_damper),
                        );
                        tracing::info!(
                            target: "digest.runtime.boot",
                            "digest assembler spawned (cadence subscriber + persister + L4 inference subscriber + degraded-mode heartbeat)",
                        );
                    }
                    None => {
                        tracing::info!(
                            target: "digest.runtime.boot",
                            "digest assembler spawned (cadence subscriber + persister); no interpretation seated",
                        );
                    }
                }
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
    let lifecycle_restart_rx = restart_broadcast.subscribe();
    tokio::spawn(start_lifecycle_heartbeat(
        Arc::clone(&lifecycle_registry),
        Arc::clone(&lifecycle_broadcast),
        Arc::clone(&baseline_state),
        lifecycle_restart_rx,
        lifecycle_thresh_rx,
        DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL,
    ));
    let corpus_basename: String = corpus_db_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("corpus.db")
        .to_string();
    if let Some(persistence) = lifecycle_persistence.as_ref() {
        tokio::spawn(run_lifecycle_persist_loop(
            Arc::clone(&lifecycle_registry),
            Arc::clone(persistence),
            Duration::from_secs(DEFAULT_LIFECYCLE_PERSIST_INTERVAL_SECS),
            corpus_basename.clone(),
        ));
    }
    if let Some(persistence) = storm_persistence.as_ref() {
        tokio::spawn(run_storm_persist_loop(
            Arc::clone(&storm_detector),
            Arc::clone(persistence),
            Duration::from_secs(DEFAULT_STORM_PERSIST_INTERVAL_SECS),
            corpus_basename,
        ));
    }
    // The persist loop catches the corpus up with the in-memory registry; the
    // auto-resolution observer evaluates Active and Acknowledged incidents for
    // the no-reemission window (P-022).
    if let Some(persistence) = incident_persistence.as_ref() {
        if let Some(durable) = incident_durable.as_ref() {
            tokio::spawn(run_incident_persist_loop(
                Arc::clone(&incident_registry),
                Arc::clone(persistence),
                Arc::clone(durable),
                vec![incident_workspace_key.clone()],
                DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS,
            ));
        }
        let observer = AutoResolveObserver::new(
            Arc::clone(&incident_registry),
            Arc::clone(persistence),
            Arc::clone(&incident_broadcast),
        );
        tokio::spawn(run_auto_resolution_loop(observer));
    }
    let _engine_tick_handles = heartbeat::spawn_engine_ticks(
        Arc::clone(&heartbeat_state),
        Arc::clone(&ingest_state),
        Arc::clone(&ingest_sender),
        Arc::clone(&buffer_state),
        retention_seconds,
        Arc::clone(&broadcast_senders),
        Arc::clone(&bind_status),
        Some(Arc::clone(&drain_miner)),
    );

    EngineHandles {
        heartbeat_state,
        ingest_state,
        bind_status,
        buffer_conn,
        buffer_state,
        broadcast_senders,
        retention_seconds,
        corpus_reader,
        corpus_writer,
        lifecycle_registry,
        lifecycle_broadcast,
        incident_registry,
        incident_broadcast,
        incident_persistence,
        incident_workspace_key,
        drain_miner,
        degraded_mode,
        reevaluator,
        config_handle_slot,
        config_status,
        digest_broadcast,
    }
}
