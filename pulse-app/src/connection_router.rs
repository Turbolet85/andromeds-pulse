//! Connection TauRPC router — chunk #59.
//!
//! `connection.current_state` resolver returning the live FSM state on
//! demand. Mirrors the `mcp_router.rs` pattern: library crate
//! (`crates/ingest`) stays Tauri-free; the Tauri-aware router lives here at
//! the binary boundary so taurpc + specta deps don't leak into the workspace
//! crate per arch §Cross-cutting Patterns Module dependency direction.
//!
//! Receiver bind status is sourced from `ui_bridge::health::HeartbeatState`
//! via the `HeartbeatBindStatus` adapter wrapping it for the
//! `ingest::connection::ReceiverBindStatus` trait — keeps the connection
//! module workspace-boundary-clean.

use std::sync::Arc;

use ingest::connection::{
    ConnectionState, ConnectionStatePayload, ReceiverBindStatus, Severity, compute_state,
    last_span_ago_ms, severity_label, state_label,
};
use ingest::state::IngestState;
use ui_bridge::contract::AppError;
use ui_bridge::health::{BindStatus, HeartbeatState};

/// Adapter implementing the ingest crate's `ReceiverBindStatus` trait over
/// `HeartbeatState`. Lives at the binary boundary; ui-bridge stays
/// unaware of the connection FSM concern.
#[derive(Debug)]
pub struct HeartbeatBindStatus {
    inner: Arc<HeartbeatState>,
}

impl HeartbeatBindStatus {
    pub fn new(inner: Arc<HeartbeatState>) -> Self {
        Self { inner }
    }
}

impl ReceiverBindStatus for HeartbeatBindStatus {
    fn any_receiver_failed(&self) -> bool {
        let grpc_failed = matches!(self.inner.otlp_grpc_bind(), Some(BindStatus::Failed(_)));
        let http_failed = matches!(self.inner.otlp_http_bind(), Some(BindStatus::Failed(_)));
        grpc_failed || http_failed
    }

    fn panic_signaled(&self) -> bool {
        crate::observability::panic_signaled()
    }
}

#[taurpc::procedures(path = "connection")]
pub trait ConnectionApi {
    async fn current_state() -> Result<ConnectionStatePayload, AppError>;
}

#[derive(Clone)]
pub struct ConnectionApiImpl {
    ingest_state: Arc<IngestState>,
    bind_status: Arc<dyn ReceiverBindStatus>,
}

impl ConnectionApiImpl {
    pub fn new(ingest_state: Arc<IngestState>, bind_status: Arc<dyn ReceiverBindStatus>) -> Self {
        Self {
            ingest_state,
            bind_status,
        }
    }
}

#[taurpc::resolvers]
impl ConnectionApi for ConnectionApiImpl {
    #[tracing::instrument(skip_all, fields(
        state = tracing::field::Empty,
        last_span_ago_ms = tracing::field::Empty,
        severity = tracing::field::Empty,
    ))]
    async fn current_state(self) -> Result<ConnectionStatePayload, AppError> {
        let now_nanos = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        let last = self.ingest_state.last_ingest_at_nanos();
        let failed = self.bind_status.any_receiver_failed();
        let panicked = self.bind_status.panic_signaled();
        let state = compute_state(last, now_nanos, failed, panicked);
        let lag = last_span_ago_ms(last, now_nanos);
        let reason = derive_reason_for_response(state, failed, panicked);
        let payload = ConnectionStatePayload::from_parts(state, lag, reason);

        let span = tracing::Span::current();
        span.record("state", state_label(state));
        span.record("last_span_ago_ms", lag);
        span.record("severity", severity_label(severity_of(state)));

        tracing::info!(
            target: "connection.current_state.request",
            state = state_label(state),
            last_span_ago_ms = lag,
            severity = severity_label(severity_of(state)),
            "connection.current_state returned",
        );

        Ok(payload)
    }
}

fn derive_reason_for_response(
    state: ConnectionState,
    any_receiver_failed: bool,
    receiver_panicked: bool,
) -> Option<ingest::connection::ReceiverFailureReason> {
    if matches!(state, ConnectionState::ReceiverFailed) {
        // Precedence: panic > bind failure > stale heartbeat. Mirrors the
        // `crates/ingest::connection::derive_reason` precedence used by
        // the FSM poller; the TauRPC `current_state` resolver should agree
        // with the broadcast topic emission for any given state.
        Some(if receiver_panicked {
            ingest::connection::ReceiverFailureReason::ReceiverPanicked
        } else if any_receiver_failed {
            ingest::connection::ReceiverFailureReason::BindFailed
        } else {
            ingest::connection::ReceiverFailureReason::StaleHeartbeat
        })
    } else {
        None
    }
}

fn severity_of(state: ConnectionState) -> Severity {
    match state {
        ConnectionState::Listening | ConnectionState::Receiving | ConnectionState::Idle => {
            Severity::Info
        }
        ConnectionState::Stalled => Severity::Warning,
        ConnectionState::ReceiverFailed => Severity::Critical,
    }
}

// Tests migrated to `pulse-app/tests/unit_connection_router.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
