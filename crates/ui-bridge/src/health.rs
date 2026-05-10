use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "taurpc-runtime")]
use crate::contract::{AppError, AppInfo, ReadyChecks, ReadyEnvelope, Settings};

static APP_START: OnceLock<Instant> = OnceLock::new();
static HEARTBEAT_STATE: OnceLock<Arc<HeartbeatState>> = OnceLock::new();

pub fn record_start() {
    let _ = APP_START.set(Instant::now());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindStatus {
    Ok,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IngestChannelStatus {
    Ok { capacity_pct: f64 },
    Saturated { capacity_pct: f64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferConnectionStatus {
    Ok,
    InitFailed(String),
    RetentionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VizQueryStatus {
    Ok,
    QueryFailed(String),
}

#[derive(Debug, Default)]
pub struct HeartbeatState {
    ingest: Mutex<Option<DateTime<Utc>>>,
    buffer: Mutex<Option<DateTime<Utc>>>,
    viz: Mutex<Option<DateTime<Utc>>>,
    plugins: Mutex<Option<DateTime<Utc>>>,
    otlp_grpc_bind: Mutex<Option<BindStatus>>,
    otlp_http_bind: Mutex<Option<BindStatus>>,
    ingest_channel: Mutex<Option<IngestChannelStatus>>,
    buffer_connection: Mutex<Option<BufferConnectionStatus>>,
    viz_query: Mutex<Option<VizQueryStatus>>,
}

impl HeartbeatState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_ingest(&self, ts: DateTime<Utc>) {
        *self.ingest.lock().unwrap() = Some(ts);
    }
    pub fn record_buffer(&self, ts: DateTime<Utc>) {
        *self.buffer.lock().unwrap() = Some(ts);
    }
    pub fn record_viz(&self, ts: DateTime<Utc>) {
        *self.viz.lock().unwrap() = Some(ts);
    }
    pub fn record_plugins(&self, ts: DateTime<Utc>) {
        *self.plugins.lock().unwrap() = Some(ts);
    }
    pub fn record_otlp_grpc_bind(&self, status: BindStatus) {
        *self.otlp_grpc_bind.lock().unwrap() = Some(status);
    }
    pub fn record_otlp_http_bind(&self, status: BindStatus) {
        *self.otlp_http_bind.lock().unwrap() = Some(status);
    }
    pub fn record_ingest_channel(&self, status: IngestChannelStatus) {
        *self.ingest_channel.lock().unwrap() = Some(status);
    }
    pub fn record_buffer_connection(&self, status: BufferConnectionStatus) {
        *self.buffer_connection.lock().unwrap() = Some(status);
    }
    pub fn record_viz_query(&self, status: VizQueryStatus) {
        *self.viz_query.lock().unwrap() = Some(status);
    }

    pub fn last_ingest(&self) -> Option<DateTime<Utc>> {
        *self.ingest.lock().unwrap()
    }
    pub fn last_buffer(&self) -> Option<DateTime<Utc>> {
        *self.buffer.lock().unwrap()
    }
    pub fn last_viz(&self) -> Option<DateTime<Utc>> {
        *self.viz.lock().unwrap()
    }
    pub fn last_plugins(&self) -> Option<DateTime<Utc>> {
        *self.plugins.lock().unwrap()
    }
    pub fn otlp_grpc_bind(&self) -> Option<BindStatus> {
        self.otlp_grpc_bind.lock().unwrap().clone()
    }
    pub fn otlp_http_bind(&self) -> Option<BindStatus> {
        self.otlp_http_bind.lock().unwrap().clone()
    }
    pub fn ingest_channel_status(&self) -> Option<IngestChannelStatus> {
        self.ingest_channel.lock().unwrap().clone()
    }
    pub fn buffer_connection(&self) -> Option<BufferConnectionStatus> {
        self.buffer_connection.lock().unwrap().clone()
    }
    pub fn viz_query(&self) -> Option<VizQueryStatus> {
        self.viz_query.lock().unwrap().clone()
    }
}

pub fn register_heartbeat_state(state: Arc<HeartbeatState>) {
    let _ = HEARTBEAT_STATE.set(state);
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Ok,
    Degraded,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemStatus {
    pub status: String,
    pub error_msg: Option<String>,
    pub last_tick_at: Option<DateTime<Utc>>,
}

impl SubsystemStatus {
    pub fn initialized() -> Self {
        Self {
            status: "initialized".to_string(),
            error_msg: None,
            last_tick_at: None,
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemStatuses {
    pub otlp_grpc_receiver: SubsystemStatus,
    pub otlp_http_receiver: SubsystemStatus,
    pub buffer: SubsystemStatus,
    pub ingest_channel: SubsystemStatus,
    pub viz: SubsystemStatus,
    pub plugins: SubsystemStatus,
}

impl SubsystemStatuses {
    pub fn placeholders() -> Self {
        Self {
            otlp_grpc_receiver: SubsystemStatus::initialized(),
            otlp_http_receiver: SubsystemStatus::initialized(),
            buffer: SubsystemStatus::initialized(),
            ingest_channel: SubsystemStatus::initialized(),
            viz: SubsystemStatus::initialized(),
            plugins: SubsystemStatus::initialized(),
        }
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEnvelope {
    pub status: HealthStatus,
    pub checked_at: DateTime<Utc>,
    pub subsystems: SubsystemStatuses,
    pub pid: u32,
    pub uptime_ms: u64,
}

pub fn current_health() -> HealthEnvelope {
    let uptime_ms = APP_START
        .get()
        .map(|start| start.elapsed().as_millis() as u64)
        .unwrap_or(0);

    let mut subsystems = SubsystemStatuses::placeholders();
    let mut overall_ok = true;
    if let Some(state) = HEARTBEAT_STATE.get() {
        subsystems.ingest_channel.last_tick_at = state.last_ingest();
        subsystems.buffer.last_tick_at = state.last_buffer();
        subsystems.viz.last_tick_at = state.last_viz();
        subsystems.plugins.last_tick_at = state.last_plugins();
        match state.otlp_grpc_bind() {
            Some(BindStatus::Ok) => {
                subsystems.otlp_grpc_receiver.status = "ok".to_string();
                subsystems.otlp_grpc_receiver.last_tick_at = state.last_ingest();
            }
            Some(BindStatus::Failed(reason)) => {
                subsystems.otlp_grpc_receiver.status = "bind_failed".to_string();
                subsystems.otlp_grpc_receiver.error_msg = Some(reason);
                overall_ok = false;
            }
            None => {}
        }
        match state.otlp_http_bind() {
            Some(BindStatus::Ok) => {
                subsystems.otlp_http_receiver.status = "ok".to_string();
                subsystems.otlp_http_receiver.last_tick_at = state.last_ingest();
            }
            Some(BindStatus::Failed(reason)) => {
                subsystems.otlp_http_receiver.status = "bind_failed".to_string();
                subsystems.otlp_http_receiver.error_msg = Some(reason);
                overall_ok = false;
            }
            None => {}
        }
        match state.ingest_channel_status() {
            Some(IngestChannelStatus::Ok { .. }) => {
                subsystems.ingest_channel.status = "ok".to_string();
                subsystems.ingest_channel.last_tick_at = state.last_ingest();
            }
            Some(IngestChannelStatus::Saturated { capacity_pct }) => {
                subsystems.ingest_channel.status = "saturated".to_string();
                subsystems.ingest_channel.error_msg =
                    Some(format!("capacity {} %", capacity_pct as u64));
                subsystems.ingest_channel.last_tick_at = state.last_ingest();
                overall_ok = false;
            }
            None => {}
        }
        match state.buffer_connection() {
            Some(BufferConnectionStatus::Ok) => {
                subsystems.buffer.status = "ok".to_string();
            }
            Some(BufferConnectionStatus::InitFailed(reason)) => {
                subsystems.buffer.status = "init_failed".to_string();
                subsystems.buffer.error_msg = Some(reason);
                overall_ok = false;
            }
            Some(BufferConnectionStatus::RetentionFailed(reason)) => {
                subsystems.buffer.status = "retention_failed".to_string();
                subsystems.buffer.error_msg = Some(reason);
                overall_ok = false;
            }
            None => {}
        }
        match state.viz_query() {
            Some(VizQueryStatus::Ok) => {
                subsystems.viz.status = "ok".to_string();
            }
            Some(VizQueryStatus::QueryFailed(reason)) => {
                subsystems.viz.status = "query_failed".to_string();
                subsystems.viz.error_msg = Some(reason);
                overall_ok = false;
            }
            None => {}
        }
    }

    HealthEnvelope {
        status: if overall_ok {
            HealthStatus::Ok
        } else {
            HealthStatus::Degraded
        },
        checked_at: Utc::now(),
        subsystems,
        pid: std::process::id(),
        uptime_ms,
    }
}

#[cfg(feature = "taurpc-runtime")]
mod runtime {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    // export_to: emission triggered when Router::into_handler() is called in
    // dev mode (`!cfg!(feature = "custom-protocol")`); a single merged TS file
    // covering ALL routers (IntrospectionApi + TracesApi + MetricsApi +
    // LogsApi + StreamsApi) is written to this path. Path is relative to
    // runtime cwd (`pulse-app/` for `cargo nextest -p pulse-app` and
    // `cargo tauri dev`).
    //
    // Top-level procedures (no `path = "..."`): `app_info`, `health`,
    // `ready`, `get_settings`, `update_settings` — the cross-cutting envelope
    // per arch §Conventions "Endpoint naming" + §Occupied Resources Tauri
    // IPC routes.
    #[taurpc::procedures(export_to = "ui/src/bindings/index.ts")]
    pub trait IntrospectionApi {
        async fn app_info() -> Result<AppInfo, AppError>;
        async fn health() -> Result<HealthEnvelope, AppError>;
        async fn ready() -> Result<ReadyEnvelope, AppError>;
        async fn get_settings() -> Result<Settings, AppError>;
        async fn update_settings(settings: Settings) -> Result<(), AppError>;
    }

    // Settings persistence path: data_dir/config.toml per arch §Occupied
    // Resources Filesystem locations. Resolution belongs to the binary
    // (`pulse-app/src/main.rs::resolve_data_dir()`); ui-bridge receives the
    // resolved path via constructor to avoid taking an env-resolution dep.
    const CONFIG_FILE: &str = "config.toml";

    #[derive(Clone)]
    pub struct IntrospectionApiImpl {
        data_dir: PathBuf,
        features: Vec<String>,
        broadcast_senders: Option<std::sync::Arc<buffer::BroadcastSenders>>,
    }

    impl IntrospectionApiImpl {
        pub fn new(
            data_dir: PathBuf,
            features: Vec<String>,
            broadcast_senders: Option<std::sync::Arc<buffer::BroadcastSenders>>,
        ) -> Self {
            Self {
                data_dir,
                features,
                broadcast_senders,
            }
        }

        fn config_path(&self) -> PathBuf {
            self.data_dir.join(CONFIG_FILE)
        }

        fn broadcast_subscribers_count(&self) -> u32 {
            match &self.broadcast_senders {
                Some(s) => {
                    (s.spans.receiver_count()
                        + s.metrics.receiver_count()
                        + s.logs.receiver_count()) as u32
                }
                None => 0,
            }
        }
    }

    #[taurpc::resolvers]
    impl IntrospectionApi for IntrospectionApiImpl {
        async fn app_info(self) -> Result<AppInfo, AppError> {
            tracing::info!(
                target: "ui-bridge.app_info",
                method_name = "app_info",
                result_type = "AppInfo",
                "introspection invoked",
            );
            Ok(AppInfo {
                name: "andromeda-pulse".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                rust_version: env!("CARGO_PKG_RUST_VERSION").to_string(),
                // Keep in sync with workspace tauri dep version (Cargo.toml line 35).
                tauri_version: "2.11".to_string(),
                features: self.features.clone(),
                build_profile: if cfg!(debug_assertions) {
                    "debug".to_string()
                } else {
                    "release".to_string()
                },
            })
        }

        async fn health(self) -> Result<HealthEnvelope, AppError> {
            tracing::info!(
                target: "ui-bridge.health",
                method_name = "health",
                result_type = "HealthEnvelope",
                "introspection invoked",
            );
            Ok(current_health())
        }

        async fn ready(self) -> Result<ReadyEnvelope, AppError> {
            tracing::info!(
                target: "ui-bridge.ready",
                method_name = "ready",
                result_type = "ReadyEnvelope",
                "introspection invoked",
            );
            let envelope = current_health();
            // checks fields mirror obs-plan §3 heartbeat tick contract so passive
            // ticks + active probe stay schema-identical. Buffer connection
            // collapses to a sanitized one-liner string per arch §Standard
            // Contracts ready envelope examples.
            let duckdb_connection = match envelope.subsystems.buffer.status.as_str() {
                "ok" => "ok".to_string(),
                "init_failed" => "init_failed".to_string(),
                "retention_failed" => "retention_failed".to_string(),
                _ => "init_in_progress".to_string(),
            };
            let ingest_mpsc_capacity_pct = HEARTBEAT_STATE
                .get()
                .and_then(|s| s.ingest_channel_status())
                .map(|s| match s {
                    IngestChannelStatus::Ok { capacity_pct } => capacity_pct as u32,
                    IngestChannelStatus::Saturated { capacity_pct } => capacity_pct as u32,
                })
                .unwrap_or(0);
            let mcp_server_enabled = self.features.iter().any(|f| f == "mcp-server");
            let checks = ReadyChecks {
                duckdb_connection,
                ingest_mpsc_capacity_pct,
                broadcast_subscribers: self.broadcast_subscribers_count(),
                // plugins_loaded stays 0 until plugins crate ships the real
                // host-loaded count surface (Epoch 7).
                plugins_loaded: 0,
                mcp_server_enabled,
            };
            let ready =
                matches!(envelope.status, HealthStatus::Ok) && checks.duckdb_connection == "ok";
            Ok(ReadyEnvelope {
                ready,
                checked_at: envelope.checked_at,
                checks,
            })
        }

        async fn get_settings(self) -> Result<Settings, AppError> {
            tracing::info!(
                target: "ui-bridge.get_settings",
                method_name = "get_settings",
                result_type = "Settings",
                "introspection invoked",
            );
            let path = self.config_path();
            match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<Settings>(&content) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(AppError::Storage {
                        message: "settings file unparseable".to_string(),
                    }),
                },
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Settings::default()),
                Err(_) => Err(AppError::Storage {
                    message: "settings read failed".to_string(),
                }),
            }
        }

        async fn update_settings(self, settings: Settings) -> Result<(), AppError> {
            // Per obs-plan §8 default-deny + Vector 6 path sanitizer: only
            // key names emit on the structured event, never values. Allowed
            // keys are bounded by the Settings struct shape (no
            // user-controllable arbitrary keys).
            tracing::info!(
                target: "ui-bridge.update_settings",
                method_name = "update_settings",
                result_type = "()",
                setting_keys_changed = "theme,widget_position,retention_seconds,mcp_server_enabled,notifications_enabled,always_on_top",
                "introspection invoked",
            );
            settings.validate()?;
            let path = self.config_path();
            let parent = path.parent().ok_or_else(|| AppError::Storage {
                message: "settings path has no parent".to_string(),
            })?;
            if fs::create_dir_all(parent).is_err() {
                return Err(AppError::Storage {
                    message: "settings dir create failed".to_string(),
                });
            }
            let serialized = match toml::to_string(&settings) {
                Ok(s) => s,
                Err(_) => {
                    return Err(AppError::Storage {
                        message: "settings serialize failed".to_string(),
                    });
                }
            };
            // Atomic write: tmp + rename to avoid partial-write windows.
            let tmp = path.with_extension("toml.tmp");
            if fs::write(&tmp, &serialized).is_err() {
                return Err(AppError::Storage {
                    message: "settings write failed".to_string(),
                });
            }
            if fs::rename(&tmp, &path).is_err() {
                return Err(AppError::Storage {
                    message: "settings rename failed".to_string(),
                });
            }
            Ok(())
        }
    }
}

#[cfg(feature = "taurpc-runtime")]
pub use runtime::{IntrospectionApi, IntrospectionApiImpl};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsystem_status_initialized_has_no_tick() {
        let s = SubsystemStatus::initialized();
        assert_eq!(s.status, "initialized");
        assert!(s.error_msg.is_none());
        assert!(s.last_tick_at.is_none());
    }

    #[test]
    fn placeholders_includes_six_subsystems_with_no_ticks() {
        let p = SubsystemStatuses::placeholders();
        assert!(p.otlp_grpc_receiver.last_tick_at.is_none());
        assert!(p.otlp_http_receiver.last_tick_at.is_none());
        assert!(p.buffer.last_tick_at.is_none());
        assert!(p.ingest_channel.last_tick_at.is_none());
        assert!(p.viz.last_tick_at.is_none());
        assert!(p.plugins.last_tick_at.is_none());
    }

    #[test]
    fn heartbeat_state_records_and_reads_per_module() {
        let state = HeartbeatState::new();
        assert!(state.last_ingest().is_none());
        assert!(state.last_buffer().is_none());
        assert!(state.last_viz().is_none());
        assert!(state.last_plugins().is_none());

        let now = Utc::now();
        state.record_ingest(now);
        state.record_buffer(now);
        state.record_viz(now);
        state.record_plugins(now);

        assert_eq!(state.last_ingest(), Some(now));
        assert_eq!(state.last_buffer(), Some(now));
        assert_eq!(state.last_viz(), Some(now));
        assert_eq!(state.last_plugins(), Some(now));
    }

    #[test]
    fn current_health_includes_six_subsystems_when_no_state_registered() {
        let envelope = current_health();
        assert!(
            envelope
                .subsystems
                .otlp_grpc_receiver
                .last_tick_at
                .is_none()
        );
        assert!(
            envelope
                .subsystems
                .otlp_http_receiver
                .last_tick_at
                .is_none()
        );
        assert!(envelope.subsystems.buffer.last_tick_at.is_none());
        assert!(envelope.subsystems.ingest_channel.last_tick_at.is_none());
        assert!(envelope.subsystems.viz.last_tick_at.is_none());
        assert!(envelope.subsystems.plugins.last_tick_at.is_none());
    }

    #[test]
    fn heartbeat_state_otlp_bind_slot_starts_empty() {
        let state = HeartbeatState::new();
        assert_eq!(state.otlp_grpc_bind(), None);
    }

    #[test]
    fn heartbeat_state_records_otlp_bind_ok_and_failed() {
        let state = HeartbeatState::new();
        state.record_otlp_grpc_bind(BindStatus::Ok);
        assert_eq!(state.otlp_grpc_bind(), Some(BindStatus::Ok));
        state.record_otlp_grpc_bind(BindStatus::Failed("address in use".to_string()));
        assert_eq!(
            state.otlp_grpc_bind(),
            Some(BindStatus::Failed("address in use".to_string()))
        );
    }

    #[test]
    fn heartbeat_state_otlp_http_bind_slot_starts_empty() {
        let state = HeartbeatState::new();
        assert_eq!(state.otlp_http_bind(), None);
    }

    #[test]
    fn heartbeat_state_records_otlp_http_bind_ok_and_failed() {
        let state = HeartbeatState::new();
        state.record_otlp_http_bind(BindStatus::Ok);
        assert_eq!(state.otlp_http_bind(), Some(BindStatus::Ok));
        state.record_otlp_http_bind(BindStatus::Failed("address in use".to_string()));
        assert_eq!(
            state.otlp_http_bind(),
            Some(BindStatus::Failed("address in use".to_string()))
        );
    }

    #[test]
    fn current_health_degrades_on_http_bind_failure() {
        let state = Arc::new(HeartbeatState::new());
        state.record_otlp_grpc_bind(BindStatus::Ok);
        state.record_otlp_http_bind(BindStatus::Failed("port in use".to_string()));
        register_heartbeat_state(state);
        let envelope = current_health();
        assert!(matches!(envelope.status, HealthStatus::Degraded));
        assert_eq!(envelope.subsystems.otlp_http_receiver.status, "bind_failed");
        assert_eq!(
            envelope.subsystems.otlp_http_receiver.error_msg,
            Some("port in use".to_string())
        );
    }

    #[test]
    fn heartbeat_state_ingest_channel_slot_starts_empty() {
        let state = HeartbeatState::new();
        assert_eq!(state.ingest_channel_status(), None);
    }

    #[test]
    fn heartbeat_state_records_ingest_channel_ok_and_saturated() {
        let state = HeartbeatState::new();
        state.record_ingest_channel(IngestChannelStatus::Ok { capacity_pct: 12.5 });
        assert_eq!(
            state.ingest_channel_status(),
            Some(IngestChannelStatus::Ok { capacity_pct: 12.5 })
        );
        state.record_ingest_channel(IngestChannelStatus::Saturated {
            capacity_pct: 100.0,
        });
        assert_eq!(
            state.ingest_channel_status(),
            Some(IngestChannelStatus::Saturated {
                capacity_pct: 100.0
            })
        );
    }

    #[test]
    fn heartbeat_state_buffer_connection_slot_starts_empty() {
        let state = HeartbeatState::new();
        assert_eq!(state.buffer_connection(), None);
    }

    #[test]
    fn heartbeat_state_records_buffer_connection_ok_and_failed() {
        let state = HeartbeatState::new();
        state.record_buffer_connection(BufferConnectionStatus::Ok);
        assert_eq!(state.buffer_connection(), Some(BufferConnectionStatus::Ok));
        state.record_buffer_connection(BufferConnectionStatus::InitFailed(
            "init: open_in_memory".into(),
        ));
        assert_eq!(
            state.buffer_connection(),
            Some(BufferConnectionStatus::InitFailed(
                "init: open_in_memory".into()
            ))
        );
    }

    #[test]
    fn heartbeat_state_records_buffer_connection_retention_failed() {
        let state = HeartbeatState::new();
        state.record_buffer_connection(BufferConnectionStatus::RetentionFailed(
            "retention sweep prepare failed".into(),
        ));
        assert_eq!(
            state.buffer_connection(),
            Some(BufferConnectionStatus::RetentionFailed(
                "retention sweep prepare failed".into()
            ))
        );
    }

    #[test]
    fn current_health_degrades_on_buffer_init_failure() {
        let state = Arc::new(HeartbeatState::new());
        state.record_otlp_grpc_bind(BindStatus::Ok);
        state.record_otlp_http_bind(BindStatus::Ok);
        state.record_buffer_connection(BufferConnectionStatus::InitFailed(
            "schema_create_failed".to_string(),
        ));
        register_heartbeat_state(state);
        let envelope = current_health();
        assert!(matches!(envelope.status, HealthStatus::Degraded));
        assert_eq!(envelope.subsystems.buffer.status, "init_failed");
        assert_eq!(
            envelope.subsystems.buffer.error_msg,
            Some("schema_create_failed".to_string())
        );
    }

    #[test]
    fn current_health_degrades_on_buffer_retention_failure() {
        let state = Arc::new(HeartbeatState::new());
        state.record_otlp_grpc_bind(BindStatus::Ok);
        state.record_otlp_http_bind(BindStatus::Ok);
        state.record_buffer_connection(BufferConnectionStatus::RetentionFailed(
            "buffer retention sweep failed".to_string(),
        ));
        register_heartbeat_state(state);
        let envelope = current_health();
        assert!(matches!(envelope.status, HealthStatus::Degraded));
        assert_eq!(envelope.subsystems.buffer.status, "retention_failed");
        assert_eq!(
            envelope.subsystems.buffer.error_msg,
            Some("buffer retention sweep failed".to_string())
        );
    }

    #[test]
    fn heartbeat_state_viz_query_slot_starts_empty() {
        let state = HeartbeatState::new();
        assert_eq!(state.viz_query(), None);
    }

    #[test]
    fn heartbeat_state_records_viz_query_ok_and_failed() {
        let state = HeartbeatState::new();
        state.record_viz_query(VizQueryStatus::Ok);
        assert_eq!(state.viz_query(), Some(VizQueryStatus::Ok));
        state.record_viz_query(VizQueryStatus::QueryFailed(
            "duckdb prepare error".to_string(),
        ));
        assert_eq!(
            state.viz_query(),
            Some(VizQueryStatus::QueryFailed(
                "duckdb prepare error".to_string()
            ))
        );
    }

    #[test]
    fn current_health_degrades_on_viz_query_failure() {
        let state = Arc::new(HeartbeatState::new());
        state.record_otlp_grpc_bind(BindStatus::Ok);
        state.record_otlp_http_bind(BindStatus::Ok);
        state.record_viz_query(VizQueryStatus::QueryFailed("viz query failed".to_string()));
        register_heartbeat_state(state);
        let envelope = current_health();
        assert!(matches!(envelope.status, HealthStatus::Degraded));
        assert_eq!(envelope.subsystems.viz.status, "query_failed");
        assert_eq!(
            envelope.subsystems.viz.error_msg,
            Some("viz query failed".to_string())
        );
    }

    #[test]
    fn current_health_degrades_on_ingest_channel_saturated() {
        let state = Arc::new(HeartbeatState::new());
        state.record_otlp_grpc_bind(BindStatus::Ok);
        state.record_otlp_http_bind(BindStatus::Ok);
        state.record_ingest_channel(IngestChannelStatus::Saturated {
            capacity_pct: 100.0,
        });
        register_heartbeat_state(state);
        let envelope = current_health();
        assert!(matches!(envelope.status, HealthStatus::Degraded));
        assert_eq!(envelope.subsystems.ingest_channel.status, "saturated");
        assert!(
            envelope
                .subsystems
                .ingest_channel
                .error_msg
                .as_deref()
                .unwrap_or("")
                .contains("100")
        );
    }
}

#[cfg(test)]
#[cfg(feature = "taurpc-runtime")]
mod introspection_tests {
    use super::*;
    use crate::contract::{Settings, SnapshotFormat, SnapshotPreset, Theme, WidgetPosition};
    use std::path::PathBuf;

    fn make_impl(data_dir: PathBuf, features: Vec<String>) -> IntrospectionApiImpl {
        IntrospectionApiImpl::new(data_dir, features, None)
    }

    #[tokio::test]
    async fn app_info_returns_andromeda_pulse_name_and_compile_time_versions() {
        let dir = tempfile::tempdir().expect("tempdir");
        let api = make_impl(dir.path().to_path_buf(), vec!["mcp-server".to_string()]);
        let info = api.app_info().await.expect("app_info ok");
        assert_eq!(info.name, "andromeda-pulse");
        assert!(!info.version.is_empty());
        assert!(!info.rust_version.is_empty());
        assert!(!info.tauri_version.is_empty());
        assert!(
            info.features.iter().any(|f| f == "mcp-server"),
            "features should contain mcp-server when constructor passes it"
        );
    }

    #[tokio::test]
    async fn health_returns_envelope_with_six_subsystems() {
        let dir = tempfile::tempdir().expect("tempdir");
        let api = make_impl(dir.path().to_path_buf(), vec![]);
        let env = api.health().await.expect("health ok");
        let v = serde_json::to_value(&env).expect("serializes");
        assert!(v["status"].is_string());
        assert!(v["checked_at"].is_string());
        for subsystem in [
            "otlp_grpc_receiver",
            "otlp_http_receiver",
            "buffer",
            "ingest_channel",
            "viz",
            "plugins",
        ] {
            assert!(
                v["subsystems"][subsystem].is_object(),
                "health envelope missing subsystem {subsystem}"
            );
        }
    }

    #[tokio::test]
    async fn ready_returns_envelope_with_required_check_keys() {
        let dir = tempfile::tempdir().expect("tempdir");
        let api = make_impl(dir.path().to_path_buf(), vec![]);
        let env = api.ready().await.expect("ready ok");
        let v = serde_json::to_value(&env).expect("serializes");
        assert!(v["ready"].is_boolean());
        assert!(v["checked_at"].is_string());
        for key in [
            "duckdb_connection",
            "ingest_mpsc_capacity_pct",
            "broadcast_subscribers",
            "plugins_loaded",
            "mcp_server_enabled",
        ] {
            assert!(
                v["checks"].get(key).is_some(),
                "ready envelope checks missing key {key}"
            );
        }
    }

    #[tokio::test]
    async fn ready_reports_mcp_server_enabled_from_features() {
        let dir = tempfile::tempdir().expect("tempdir");
        let api = make_impl(dir.path().to_path_buf(), vec!["mcp-server".to_string()]);
        let env = api.ready().await.expect("ready ok");
        assert!(env.checks.mcp_server_enabled);
    }

    #[tokio::test]
    async fn get_settings_returns_default_when_config_file_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let api = make_impl(dir.path().to_path_buf(), vec![]);
        let s = api.get_settings().await.expect("get_settings ok");
        assert_eq!(s, Settings::default());
    }

    #[tokio::test]
    async fn update_settings_persists_to_config_toml_and_get_returns_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let new_settings = Settings {
            theme: Theme::Light,
            widget_position: WidgetPosition::BottomLeft,
            retention_seconds: 300,
            mcp_server_enabled: true,
            notifications_enabled: false,
            always_on_top: false,
            snapshot_preset: SnapshotPreset::Detailed,
            snapshot_format: SnapshotFormat::Json,
        };

        let api1 = make_impl(dir.path().to_path_buf(), vec![]);
        api1.update_settings(new_settings.clone())
            .await
            .expect("update_settings ok");

        // Confirm config.toml exists at expected path under data_dir.
        assert!(dir.path().join("config.toml").exists());

        let api2 = make_impl(dir.path().to_path_buf(), vec![]);
        let read = api2.get_settings().await.expect("get_settings ok");
        assert_eq!(read, new_settings);
    }

    #[tokio::test]
    async fn update_settings_rejects_below_min_retention_with_validation_error() {
        let dir = tempfile::tempdir().expect("tempdir");
        let api = make_impl(dir.path().to_path_buf(), vec![]);
        let bad = Settings {
            retention_seconds: 30,
            ..Settings::default()
        };
        let result = api.update_settings(bad).await;
        match result {
            Err(crate::contract::AppError::Validation { field, .. }) => {
                assert_eq!(field, "retention_seconds");
            }
            other => panic!("expected Validation error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn update_settings_rejects_above_max_retention() {
        let dir = tempfile::tempdir().expect("tempdir");
        let api = make_impl(dir.path().to_path_buf(), vec![]);
        let bad = Settings {
            retention_seconds: 1_000_000,
            ..Settings::default()
        };
        let result = api.update_settings(bad).await;
        assert!(matches!(
            result,
            Err(crate::contract::AppError::Validation { .. })
        ));
    }

    #[tokio::test]
    async fn get_settings_returns_storage_error_when_config_unparseable() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("config.toml"),
            "this is = not valid toml }}}}}",
        )
        .expect("write garbage");
        let api = make_impl(dir.path().to_path_buf(), vec![]);
        let result = api.get_settings().await;
        match result {
            Err(crate::contract::AppError::Storage { message }) => {
                assert!(message.contains("settings"));
                assert!(!message.contains('/'), "no path leak in error message");
            }
            other => panic!("expected Storage error, got {other:?}"),
        }
    }
}
