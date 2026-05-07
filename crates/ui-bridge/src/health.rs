use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "taurpc-runtime")]
use crate::contract::AppError;

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

    // export_to: emission triggered when Router::into_handler() is called in
    // dev mode (`!cfg!(feature = "custom-protocol")`); a single merged TS file
    // covering ALL routers (HealthApi + TracesApi + MetricsApi + LogsApi +
    // StreamsApi) is written to this path. Path is relative to runtime cwd
    // (`pulse-app/` for `cargo nextest -p pulse-app` and `cargo tauri dev`;
    // bindings emit via the `emit_bindings` test below).
    #[taurpc::procedures(path = "health", export_to = "ui/src/bindings/index.ts")]
    pub trait HealthApi {
        async fn check() -> Result<HealthEnvelope, AppError>;
    }

    #[derive(Clone)]
    pub struct HealthApiImpl;

    #[taurpc::resolvers]
    impl HealthApi for HealthApiImpl {
        async fn check(self) -> Result<HealthEnvelope, AppError> {
            Ok(current_health())
        }
    }
}

#[cfg(feature = "taurpc-runtime")]
pub use runtime::{HealthApi, HealthApiImpl};

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
