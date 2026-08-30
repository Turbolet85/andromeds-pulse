// Migrated 2026-08-30 from `pulse-app/src/diagnostics_router.rs::tests` —
// that crate sets `[lib] test = false` (the WebView2 workaround), so a
// src-level `mod tests` compiles, passes clippy, and NEVER RUNS. Covers the
// `diagnostics.template_distribution` resolver + `drain_error_to_app_error`;
// the sibling `unit_diagnostics_router_{retry_interpretation,snapshot}.rs`
// suites cover the router's other procedures.

use std::sync::Arc;

use buffer::{DrainConfig, DrainMiner, DriftIndicator as BufferDriftIndicator};
use triage::contract::HardwareProfileSource;
use ui_bridge::contract::AppError;

use pulse_app::diagnostics_router::{
    DiagnosticsApi, DiagnosticsApiImpl, DriftIndicatorPayload, TEMPLATE_DISTRIBUTION_TOP_N,
    TemplateDistEntryPayload, TemplateDistributionPayload, drain_error_to_app_error,
};
use pulse_app::reevaluation::{RecentWindowReevaluator, ReevaluationSummary};

use interpretation::contract::LlmInferenceRunner;

fn noop_reevaluator() -> Arc<dyn RecentWindowReevaluator> {
    struct Noop;
    impl RecentWindowReevaluator for Noop {
        fn reevaluate(&self) -> ReevaluationSummary {
            ReevaluationSummary {
                services_reclassified: 0,
                transitions_emitted: 0,
                cadence_prospective: true,
            }
        }
    }
    Arc::new(Noop)
}

fn test_runner() -> Arc<dyn LlmInferenceRunner> {
    Arc::new(pulse_app::llamacli_inference::LlamaCliInference::new(
        interpretation::contract::ModelTier::Primary,
        triage::contract::HardwareProfile::Unknown,
        interpretation::broadcast::ModelStatusBroadcast::new(),
    ))
}

fn test_profile_source() -> Arc<dyn HardwareProfileSource> {
    Arc::new(triage::contract::UnknownHardwareProfile)
}

fn make_impl() -> DiagnosticsApiImpl {
    let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
    DiagnosticsApiImpl::new(
        miner,
        Arc::new(pulse_app::degraded_mode_runtime::LocalDegradedModeStatus::new()),
        noop_reevaluator(),
        test_runner(),
        test_profile_source(),
    )
}

#[tokio::test]
async fn template_distribution_returns_empty_payload_for_fresh_miner() {
    let api = make_impl();
    let payload = api.template_distribution().await.expect("infallible");
    assert!(payload.templates.is_empty());
    assert_eq!(payload.total_template_count, 0);
    assert!(payload.last_updated_unix_nano > 0);
}

#[tokio::test]
async fn template_distribution_returns_top_templates_after_ingest() {
    let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
    miner.assign("event alpha occurred at startup");
    miner.assign("event alpha occurred at startup");
    miner.assign("event alpha occurred at startup");
    miner.assign("event beta different message entirely");
    let api = DiagnosticsApiImpl::new(
        miner,
        Arc::new(pulse_app::degraded_mode_runtime::LocalDegradedModeStatus::new()),
        noop_reevaluator(),
        test_runner(),
        test_profile_source(),
    );
    let payload = api.template_distribution().await.expect("infallible");
    assert!(!payload.templates.is_empty());
    assert!(payload.total_template_count >= 1);
    let first = &payload.templates[0];
    assert!(first.occurrence_count >= 1);
    for window in payload.templates.windows(2) {
        assert!(window[0].occurrence_count >= window[1].occurrence_count);
    }
}

#[tokio::test]
async fn template_distribution_caps_at_top_n_constant() {
    let miner = Arc::new(DrainMiner::new(
        DrainConfig {
            depth: 4,
            similarity: 0.99,
            max_clusters: 1000,
            masking_patterns: Vec::new(),
        },
        None,
    ));
    for i in 0..(TEMPLATE_DISTRIBUTION_TOP_N + 30) {
        miner.assign(&format!("distinct_event_x{i} occurred at phase tail"));
    }
    let api = DiagnosticsApiImpl::new(
        miner,
        Arc::new(pulse_app::degraded_mode_runtime::LocalDegradedModeStatus::new()),
        noop_reevaluator(),
        test_runner(),
        test_profile_source(),
    );
    let payload = api.template_distribution().await.expect("infallible");
    assert!(payload.templates.len() <= TEMPLATE_DISTRIBUTION_TOP_N);
}

#[tokio::test]
async fn template_distribution_payload_drift_indicator_round_trips() {
    let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
    miner.assign("singleton message word four");
    let api = DiagnosticsApiImpl::new(
        miner,
        Arc::new(pulse_app::degraded_mode_runtime::LocalDegradedModeStatus::new()),
        noop_reevaluator(),
        test_runner(),
        test_profile_source(),
    );
    let payload = api.template_distribution().await.expect("infallible");
    assert_eq!(payload.templates.len(), 1);
    assert_eq!(
        payload.templates[0].drift_indicator,
        DriftIndicatorPayload::UnderClustered
    );
}

#[test]
fn drift_indicator_payload_from_buffer_variant_round_trip() {
    for (buffer_variant, expected_payload) in [
        (
            BufferDriftIndicator::Healthy,
            DriftIndicatorPayload::Healthy,
        ),
        (
            BufferDriftIndicator::OverGeneralized,
            DriftIndicatorPayload::OverGeneralized,
        ),
        (
            BufferDriftIndicator::UnderClustered,
            DriftIndicatorPayload::UnderClustered,
        ),
    ] {
        let payload: DriftIndicatorPayload = buffer_variant.into();
        assert_eq!(payload, expected_payload);
        let json = serde_json::to_string(&payload).expect("serialize");
        let roundtrip: DriftIndicatorPayload = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(roundtrip, expected_payload);
    }
}

#[test]
fn drain_error_to_app_error_drain_variant_sanitized() {
    let err = drain_error_to_app_error(buffer::Error::Drain {
        reason: "internal mutex poisoned at /home/user/secret/path.rs line 42 (SELECT *)"
            .to_string(),
    });
    match err {
        AppError::Storage { message } => {
            assert_eq!(message, "drain template miner failed");
            assert!(!message.contains("/"));
            assert!(!message.contains("SELECT"));
            assert!(!message.contains("mutex"));
            assert!(!message.contains("path.rs"));
        }
        other => panic!("expected AppError::Storage, got {other:?}"),
    }
}

#[test]
fn drain_error_to_app_error_init_variant_sanitized() {
    let err = drain_error_to_app_error(buffer::Error::Init {
        reason: "open_in_memory failed: duckdb 1.5.0 returned ENOMEM".to_string(),
    });
    match err {
        AppError::Storage { message } => {
            assert_eq!(message, "buffer init failed");
            assert!(!message.contains("duckdb"));
            assert!(!message.contains("ENOMEM"));
            assert!(!message.contains("1.5.0"));
        }
        other => panic!("expected AppError::Storage, got {other:?}"),
    }
}

#[test]
fn drain_error_to_app_error_schema_create_variant_sanitized() {
    let err = drain_error_to_app_error(buffer::Error::SchemaCreate {
        reason: "CREATE TABLE log_templates failed at line 12".to_string(),
    });
    match err {
        AppError::Storage { message } => {
            assert_eq!(message, "buffer schema creation failed");
            assert!(!message.contains("CREATE"));
            assert!(!message.contains("log_templates"));
        }
        other => panic!("expected AppError::Storage, got {other:?}"),
    }
}

#[test]
fn drain_error_to_app_error_invalid_batch_variant_sanitized() {
    let err = drain_error_to_app_error(buffer::Error::InvalidBatch {
        kind: "spans_empty",
    });
    match err {
        AppError::Storage { message } => {
            assert_eq!(message, "buffer rejected invalid batch");
        }
        other => panic!("expected AppError::Storage, got {other:?}"),
    }
}

#[test]
fn drain_error_to_app_error_broadcast_size_cap_variant_sanitized() {
    let err = drain_error_to_app_error(buffer::Error::BroadcastSizeCapExceeded {
        payload_bytes: 16 * 1024 * 1024,
    });
    match err {
        AppError::Storage { message } => {
            assert_eq!(message, "buffer broadcast payload too large");
        }
        other => panic!("expected AppError::Storage, got {other:?}"),
    }
}

#[test]
fn template_distribution_payload_serializes_through_serde_json() {
    let payload = TemplateDistributionPayload {
        templates: vec![TemplateDistEntryPayload {
            id: 1,
            content: "test template".to_string(),
            occurrence_count: 5,
            drift_indicator: DriftIndicatorPayload::Healthy,
        }],
        total_template_count: 1,
        last_updated_unix_nano: 1_700_000_000_000_000_000,
    };
    let json = serde_json::to_string(&payload).expect("serialize");
    let roundtrip: TemplateDistributionPayload = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, payload);
}
