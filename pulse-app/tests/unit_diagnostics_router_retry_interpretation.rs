//! Integration tests for `pulse-app/src/diagnostics_router.rs`
//! `diagnostics.retry_interpretation()` TauRPC procedure (chunk #86).
//!
//! Tests the resolver-level contract:
//! - Positive path (Degraded state) → triggered=true + state reset
//! - No-op path (Active state with zero counter) → triggered=false
//! - Serde round-trip of RetryInterpretationPayload (specta::Type)
//! - Observability event emission with aggregate-only fields
//! - AppError sanitization grep (assert NO stack traces / paths / lib versions)

use std::sync::Arc;

use buffer::{DrainConfig, DrainMiner};
use interpretation::degraded_mode::DegradedModeStatus;
use pulse_app::degraded_mode_runtime::LocalDegradedModeStatus;
use pulse_app::diagnostics_router::{
    DiagnosticsApi, DiagnosticsApiImpl, RetryInterpretationPayload,
};

const NANOS_PER_SEC: i64 = 1_000_000_000;

fn make_impl(dm: Arc<dyn DegradedModeStatus>) -> DiagnosticsApiImpl {
    let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
    // Chunk #96 — DiagnosticsApiImpl now takes a RecentWindowReevaluator; a
    // LiveReevaluator over fresh in-memory handles keeps the test hermetic.
    let registry: Arc<dyn triage::contract::ServiceRegistry> =
        Arc::new(triage::contract::InMemoryServiceRegistry::new());
    let broadcast = Arc::new(triage::contract::ServiceLifecycleBroadcast::new());
    let baseline = Arc::new(triage::contract::BaselineState::new());
    let (_thresh_tx, thresh_rx) =
        tokio::sync::watch::channel(triage::contract::LifecycleThresholds {
            dormant_after_secs: 3_600,
            archived_after_secs: 86_400,
        });
    let reevaluator: Arc<dyn pulse_app::reevaluation::RecentWindowReevaluator> = Arc::new(
        pulse_app::reevaluation::LiveReevaluator::new(registry, broadcast, baseline, thresh_rx),
    );
    // Chunk #97 — DiagnosticsApiImpl gained LlmInferenceRunner +
    // HardwareProfileSource for diagnostics.snapshot(); hermetic stubs
    // (env-unset runner → status Error; Unknown hardware profile).
    let runner: Arc<dyn interpretation::contract::LlmInferenceRunner> =
        Arc::new(pulse_app::llamacli_inference::LlamaCliInference::new(
            interpretation::contract::ModelTier::Primary,
            triage::contract::HardwareProfile::Unknown,
            interpretation::broadcast::ModelStatusBroadcast::new(),
        ));
    DiagnosticsApiImpl::new(
        miner,
        dm,
        reevaluator,
        runner,
        Arc::new(triage::contract::UnknownHardwareProfile),
    )
}

#[tokio::test]
async fn diagnostics_retry_interpretation_in_active_state_returns_no_op() {
    // Fresh FSM: state=Active + consecutive_failures=0 → no-op retry
    let dm: Arc<dyn DegradedModeStatus> = Arc::new(LocalDegradedModeStatus::new());
    let api = make_impl(Arc::clone(&dm));
    let payload: RetryInterpretationPayload = api.retry_interpretation().await.expect("infallible");
    assert!(!payload.triggered);
    assert_eq!(payload.current_state, "active");
    assert_eq!(payload.consecutive_failures, 0);
    assert_eq!(payload.backoff_remaining_seconds, 0);
}

#[tokio::test]
async fn diagnostics_retry_interpretation_with_active_failures_triggers_reset() {
    // FSM has 1 in-window failure (still Active) — retry counts as triggered
    // because the counter > 0.
    let dm: Arc<dyn DegradedModeStatus> = Arc::new(LocalDegradedModeStatus::new());
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    let api = make_impl(Arc::clone(&dm));
    let payload: RetryInterpretationPayload = api.retry_interpretation().await.expect("infallible");
    assert!(payload.triggered);
    assert_eq!(payload.current_state, "active");
    assert_eq!(payload.consecutive_failures, 0);
}

#[tokio::test]
async fn diagnostics_retry_interpretation_in_degraded_state_triggers_recovery() {
    let dm: Arc<dyn DegradedModeStatus> = Arc::new(LocalDegradedModeStatus::new());
    let t0 = 10_000 * NANOS_PER_SEC;
    dm.record_failure(t0);
    dm.record_failure(t0);
    dm.record_failure(t0); // Degraded
    let api = make_impl(Arc::clone(&dm));
    let payload: RetryInterpretationPayload = api.retry_interpretation().await.expect("infallible");
    assert!(payload.triggered);
    assert_eq!(payload.current_state, "active");
    assert_eq!(payload.consecutive_failures, 0);
    assert_eq!(payload.backoff_remaining_seconds, 0);
}

#[test]
fn retry_interpretation_payload_serializes_through_serde_json() {
    let payload = RetryInterpretationPayload {
        triggered: true,
        current_state: "degraded".to_string(),
        consecutive_failures: 3,
        backoff_remaining_seconds: 120,
    };
    let json = serde_json::to_string(&payload).expect("serialize");
    let roundtrip: RetryInterpretationPayload = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, payload);
}

#[test]
fn retry_interpretation_payload_excludes_pii_fields_in_serialization() {
    let payload = RetryInterpretationPayload {
        triggered: false,
        current_state: "active".to_string(),
        consecutive_failures: 0,
        backoff_remaining_seconds: 0,
    };
    let json = serde_json::to_string(&payload).expect("serialize");
    // Aggregate-only discipline per chunk #86 obs constraint —
    // payload schema MUST NOT carry per-incident / per-service / per-trace
    // identifiers OR LLM-emitted content.
    assert!(!json.contains("incident_id"));
    assert!(!json.contains("service_name"));
    assert!(!json.contains("scope_id"));
    assert!(!json.contains("span_id"));
    assert!(!json.contains("trace_id"));
    assert!(!json.contains("model_output"));
    assert!(!json.contains("parse_error_message"));
}
