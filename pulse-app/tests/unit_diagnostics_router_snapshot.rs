//! Integration tests for `pulse-app/src/diagnostics_router.rs`
//! `diagnostics.snapshot()` + `diagnostics.history()` TauRPC procedures
//! (chunk #97 — Settings → Diagnostics view, capability P-058).
//!
//! Hybrid-render scope: `snapshot()` aggregates the live point-in-time L6
//! state that already exists (model tier/profile/load-status + backoff +
//! Drain template count + hardware profile); sub-fields with no producer
//! (inference success rate, queue depth, per-layer L0-L5 numerics) are
//! `None`/`false` — never fabricated. `history()` is a validated stub (no
//! numeric-metric-history producer exists yet) returning an empty series.

use std::sync::Arc;

use buffer::{DrainConfig, DrainMiner};
use interpretation::contract::LlmInferenceRunner;
use pulse_app::degraded_mode_runtime::LocalDegradedModeStatus;
use pulse_app::diagnostics_router::{
    DIAGNOSTICS_HISTORY_MAX_WINDOW_SECONDS, DiagnosticsApi, DiagnosticsApiImpl,
    DiagnosticsHistoryPayload, DiagnosticsSnapshotPayload,
};
use pulse_app::llamacli_inference::LlamaCliInference;
use pulse_app::reevaluation::{LiveReevaluator, RecentWindowReevaluator};
use ui_bridge::contract::AppError;

fn test_runner() -> Arc<dyn LlmInferenceRunner> {
    // Env-unset LlamaCliInference: tier Primary, status Error, identity None.
    Arc::new(LlamaCliInference::new(
        interpretation::contract::ModelTier::Primary,
        triage::contract::HardwareProfile::Unknown,
        interpretation::broadcast::ModelStatusBroadcast::new(),
    ))
}

fn test_reevaluator() -> Arc<dyn RecentWindowReevaluator> {
    let registry: Arc<dyn triage::contract::ServiceRegistry> =
        Arc::new(triage::contract::InMemoryServiceRegistry::new());
    let broadcast = Arc::new(triage::contract::ServiceLifecycleBroadcast::new());
    let baseline = Arc::new(triage::contract::BaselineState::new());
    let (_tx, rx) = tokio::sync::watch::channel(triage::contract::LifecycleThresholds {
        dormant_after_secs: 3_600,
        archived_after_secs: 86_400,
    });
    Arc::new(LiveReevaluator::new(registry, broadcast, baseline, rx))
}

fn make_impl_with(miner: Arc<DrainMiner>) -> DiagnosticsApiImpl {
    DiagnosticsApiImpl::new(
        miner,
        Arc::new(LocalDegradedModeStatus::new()),
        test_reevaluator(),
        test_runner(),
        Arc::new(triage::contract::UnknownHardwareProfile),
    )
}

fn make_impl() -> DiagnosticsApiImpl {
    make_impl_with(Arc::new(DrainMiner::new(
        DrainConfig::default_config(),
        None,
    )))
}

#[tokio::test]
async fn snapshot_returns_live_state_with_degraded_markers_for_unproduced_fields() {
    let api = make_impl();
    let payload: DiagnosticsSnapshotPayload = api.snapshot().await.expect("infallible");

    // Model section — env-unset runner + fresh degraded-mode FSM.
    assert_eq!(payload.model.tier_label, "primary");
    assert_eq!(payload.model.load_status, "error");
    assert_eq!(payload.model.model_identity_name, None);
    assert_eq!(payload.model.backoff_state_label, "active");
    assert_eq!(payload.model.consecutive_failures, 0);
    assert_eq!(payload.model.backoff_remaining_seconds, 0);

    // Hybrid-render: unproduced sub-fields are None, NEVER fabricated.
    assert_eq!(payload.model.inference_success_rate_basis_points, None);
    assert_eq!(payload.model.queue_depth, None);

    // Hardware section.
    assert_eq!(payload.hardware.profile_label, "unknown");
    assert_eq!(payload.hardware.detection_detail, None);

    // Pipeline section — per-layer numerics not yet recorded.
    assert!(!payload.pipeline.per_layer_recorded);
    assert!(payload.captured_unix_nano > 0);
}

#[tokio::test]
async fn snapshot_reflects_drain_template_count_after_ingest() {
    let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
    miner.assign("event alpha occurred at startup");
    miner.assign("event beta different message entirely");
    let api = make_impl_with(miner);
    let payload = api.snapshot().await.expect("infallible");
    assert!(payload.pipeline.drain_template_count >= 1);
}

#[tokio::test]
async fn snapshot_returns_zero_template_count_for_fresh_miner() {
    let api = make_impl();
    let payload = api.snapshot().await.expect("infallible");
    assert_eq!(payload.pipeline.drain_template_count, 0);
}

#[tokio::test]
async fn history_returns_stub_for_known_metric() {
    let api = make_impl();
    let payload: DiagnosticsHistoryPayload = api
        .history("drain_template_count".to_string(), 3_600)
        .await
        .expect("known metric returns Ok");
    assert_eq!(payload.metric_name, "drain_template_count");
    assert!(!payload.recorded);
    assert!(payload.points.is_empty());
    assert!(!payload.notice.is_empty());
}

#[tokio::test]
async fn history_rejects_unknown_metric_name() {
    let api = make_impl();
    let err = api
        .history("bogus_metric".to_string(), 3_600)
        .await
        .expect_err("unknown metric rejected");
    match err {
        AppError::Validation { field, .. } => assert_eq!(field, "metric_name"),
        other => panic!("expected AppError::Validation, got {other:?}"),
    }
}

#[tokio::test]
async fn history_clamps_oversized_window_without_error() {
    let api = make_impl();
    let payload = api
        .history(
            "queue_depth".to_string(),
            DIAGNOSTICS_HISTORY_MAX_WINDOW_SECONDS.saturating_mul(10),
        )
        .await
        .expect("oversized window clamps, does not error");
    assert!(!payload.recorded);
    assert!(payload.points.is_empty());
}

#[test]
fn snapshot_payload_serializes_and_excludes_pii_fields() {
    let payload = DiagnosticsSnapshotPayload {
        model: pulse_app::diagnostics_router::ModelSectionPayload {
            tier_label: "primary".to_string(),
            profile_label: "unknown".to_string(),
            load_status: "error".to_string(),
            model_identity_name: None,
            backoff_state_label: "active".to_string(),
            backoff_remaining_seconds: 0,
            consecutive_failures: 0,
            inference_success_rate_basis_points: None,
            queue_depth: None,
        },
        hardware: pulse_app::diagnostics_router::HardwareSectionPayload {
            profile_label: "unknown".to_string(),
            detection_detail: None,
        },
        pipeline: pulse_app::diagnostics_router::PipelineSectionPayload {
            drain_template_count: 0,
            per_layer_recorded: false,
        },
        captured_unix_nano: 1_700_000_000_000,
    };
    let json = serde_json::to_string(&payload).expect("serialize");
    let roundtrip: DiagnosticsSnapshotPayload = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, payload);
    for banned in [
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
    ] {
        assert!(
            !json.contains(banned),
            "snapshot payload must not carry `{banned}`"
        );
    }
}

#[test]
fn history_payload_serializes_round_trip() {
    let payload = DiagnosticsHistoryPayload {
        metric_name: "queue_depth".to_string(),
        points: Vec::new(),
        recorded: false,
        notice: "metric history recording not yet available".to_string(),
    };
    let json = serde_json::to_string(&payload).expect("serialize");
    let roundtrip: DiagnosticsHistoryPayload = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, payload);
}
