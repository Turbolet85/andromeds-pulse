// Migrated 2026-08-30 from `pulse-app/src/connection_router.rs::tests` —
// that crate sets `[lib] test = false` (the WebView2 workaround), so a
// src-level `mod tests` compiles, passes clippy, and NEVER RUNS.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use ingest::connection::{ConnectionState, ReceiverBindStatus, Severity};
use ingest::state::IngestState;
use ui_bridge::health::{BindStatus, HeartbeatState};

use pulse_app::connection_router::{ConnectionApi, ConnectionApiImpl, HeartbeatBindStatus};

struct StubBindStatus(AtomicBool);
impl ReceiverBindStatus for StubBindStatus {
    fn any_receiver_failed(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

fn make_impl(failed: bool) -> ConnectionApiImpl {
    let ingest = Arc::new(IngestState::new());
    let bind: Arc<dyn ReceiverBindStatus> = Arc::new(StubBindStatus(AtomicBool::new(failed)));
    ConnectionApiImpl::new(ingest, bind)
}

#[tokio::test]
async fn current_state_returns_listening_when_no_ingest_and_bind_ok() {
    let api = make_impl(false);
    let payload = api.current_state().await.expect("returns Ok");
    assert_eq!(payload.state, ConnectionState::Listening);
    assert_eq!(payload.severity, Severity::Info);
    assert_eq!(payload.last_span_ago_ms, 0);
    assert!(payload.message.is_none());
    assert!(payload.reason.is_none());
}

#[tokio::test]
async fn current_state_returns_receiver_failed_when_bind_failed() {
    let api = make_impl(true);
    let payload = api.current_state().await.expect("returns Ok");
    assert_eq!(payload.state, ConnectionState::ReceiverFailed);
    assert_eq!(payload.severity, Severity::Critical);
    assert!(payload.message.is_some());
    assert_eq!(
        payload.reason,
        Some(ingest::connection::ReceiverFailureReason::BindFailed)
    );
}

#[test]
fn heartbeat_bind_status_reports_failed_when_either_receiver_failed() {
    let hb = Arc::new(HeartbeatState::new());
    let adapter = HeartbeatBindStatus::new(Arc::clone(&hb));
    assert!(
        !adapter.any_receiver_failed(),
        "fresh HeartbeatState has no recorded bind status"
    );
    hb.record_otlp_grpc_bind(BindStatus::Failed("test".to_string()));
    assert!(adapter.any_receiver_failed());
}

#[test]
fn heartbeat_bind_status_ok_reports_not_failed() {
    let hb = Arc::new(HeartbeatState::new());
    hb.record_otlp_grpc_bind(BindStatus::Ok);
    hb.record_otlp_http_bind(BindStatus::Ok);
    let adapter = HeartbeatBindStatus::new(hb);
    assert!(!adapter.any_receiver_failed());
}
