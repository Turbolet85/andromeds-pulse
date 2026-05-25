//! Diagnostics TauRPC router — chunk #69 Phase B Session 3.
//!
//! `diagnostics.template_distribution()` exposes the in-memory Drain
//! template tree to the webview Settings → Diagnostics panel. Read-only
//! by design (mirrors `storage_router.rs` chunk #68 P-051 read-only-by-
//! design posture); no config-writing or tree-clearing procedures land
//! here.
//!
//! The buffer crate stays Tauri-free + specta-free; the cross-bridge
//! payload types defined locally (per arch §Cross-cutting Patterns
//! Module dependency direction). The free-function
//! `drain_error_to_app_error` parallels `storage_router.rs::corpus_error_to_app_error`
//! (forward-looking for Session 4 persistence error paths; the read-only
//! Session 3 surface itself is infallible at the in-memory miner level).

use std::sync::Arc;

use buffer::{DrainMiner, DriftIndicator as BufferDriftIndicator, TemplateDistEntry, TemplateId};
use interpretation::degraded_mode::{DegradedModeStatus, degraded_mode_state_label};
use serde::{Deserialize, Serialize};
use ui_bridge::contract::AppError;

/// Maximum templates returned per `diagnostics.template_distribution()`
/// call. Caller cannot exceed this cap (per layouts AC-L5 top-50 row
/// limit; same value enforced server-side as a defensive bound).
pub const TEMPLATE_DISTRIBUTION_TOP_N: usize = 50;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub enum DriftIndicatorPayload {
    Healthy,
    OverGeneralized,
    UnderClustered,
}

impl From<BufferDriftIndicator> for DriftIndicatorPayload {
    fn from(value: BufferDriftIndicator) -> Self {
        match value {
            BufferDriftIndicator::Healthy => Self::Healthy,
            BufferDriftIndicator::OverGeneralized => Self::OverGeneralized,
            BufferDriftIndicator::UnderClustered => Self::UnderClustered,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct TemplateDistEntryPayload {
    pub id: TemplateId,
    pub content: String,
    pub occurrence_count: u64,
    pub drift_indicator: DriftIndicatorPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct TemplateDistributionPayload {
    pub templates: Vec<TemplateDistEntryPayload>,
    pub total_template_count: u64,
    pub last_updated_unix_nano: i64,
}

/// Chunk #86 — Settings → Diagnostics "Retry interpretation now" payload.
/// Returned by `diagnostics.retry_interpretation()` after invoking the
/// manual-override path on the degraded-mode FSM.
///
/// Bounded к scalar / string-label fields per chunk #86 obs constraint
/// aggregate-only discipline; carries NO LLM-emitted content / incident
/// identifiers / per-trace IDs. The `current_state` label is the snake-
/// case bounded enum from `interpretation::degraded_mode::DegradedModeState`
/// ("active" | "degraded") routed through `degraded_mode_state_label`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct RetryInterpretationPayload {
    /// True if the manual-retry invocation transitioned FSM state OR
    /// reset а live failure counter; false if FSM was already в clean
    /// Active state с zero consecutive failures (no-op retry).
    pub triggered: bool,
    /// Post-invocation FSM state label ("active" | "degraded").
    pub current_state: String,
    pub consecutive_failures: u32,
    pub backoff_remaining_seconds: u64,
}

// Chunk #86 — `retry_interpretation` is the manual override for the L4
// degraded-mode FSM. Resets the consecutive-failure counter к 0 and
// transitions Degraded → Active immediately, regardless of current
// backoff window position. The next L4 inference invocation proceeds
// normally; если it fails the failure counter restarts from 1.
#[taurpc::procedures(path = "diagnostics")]
pub trait DiagnosticsApi {
    async fn template_distribution() -> Result<TemplateDistributionPayload, AppError>;
    async fn retry_interpretation() -> Result<RetryInterpretationPayload, AppError>;
}

#[derive(Clone)]
pub struct DiagnosticsApiImpl {
    miner: Arc<DrainMiner>,
    degraded_mode: Arc<dyn DegradedModeStatus>,
}

impl DiagnosticsApiImpl {
    pub fn new(miner: Arc<DrainMiner>, degraded_mode: Arc<dyn DegradedModeStatus>) -> Self {
        Self {
            miner,
            degraded_mode,
        }
    }
}

#[taurpc::resolvers]
impl DiagnosticsApi for DiagnosticsApiImpl {
    #[tracing::instrument(skip_all, fields(
        triggered = tracing::field::Empty,
        current_state = tracing::field::Empty,
        consecutive_failures = tracing::field::Empty,
        backoff_remaining_seconds = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ))]
    async fn retry_interpretation(self) -> Result<RetryInterpretationPayload, AppError> {
        let start = std::time::Instant::now();
        let now = current_unix_nanos();
        let pre = self.degraded_mode.current_snapshot(now);
        let triggered = pre.consecutive_failures > 0
            || matches!(
                pre.state,
                interpretation::degraded_mode::DegradedModeState::Degraded
            );
        let post = self.degraded_mode.trigger_manual_retry(now);
        let payload = RetryInterpretationPayload {
            triggered,
            current_state: degraded_mode_state_label(post.state).to_string(),
            consecutive_failures: post.consecutive_failures,
            backoff_remaining_seconds: post.backoff_seconds_remaining,
        };
        let duration_ms: u64 = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        let span = tracing::Span::current();
        span.record("triggered", triggered);
        span.record("current_state", payload.current_state.as_str());
        span.record("consecutive_failures", payload.consecutive_failures);
        span.record(
            "backoff_remaining_seconds",
            payload.backoff_remaining_seconds,
        );
        span.record("duration_ms", duration_ms);

        tracing::info!(
            target: "diagnostics.retry_interpretation.request",
            triggered = triggered,
            current_state = payload.current_state.as_str(),
            consecutive_failures = payload.consecutive_failures,
            backoff_remaining_seconds = payload.backoff_remaining_seconds,
            duration_ms = duration_ms,
            "diagnostics.retry_interpretation returned",
        );

        Ok(payload)
    }

    #[tracing::instrument(skip_all, fields(
        top_n = tracing::field::Empty,
        result_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ))]
    async fn template_distribution(self) -> Result<TemplateDistributionPayload, AppError> {
        let start = std::time::Instant::now();
        let top_n = TEMPLATE_DISTRIBUTION_TOP_N;

        let entries = self.miner.template_distribution(top_n);
        let total = self.miner.template_count();
        let now = current_unix_nanos();

        let templates: Vec<TemplateDistEntryPayload> =
            entries.into_iter().map(template_entry_to_payload).collect();
        let result_count = templates.len();

        let payload = TemplateDistributionPayload {
            templates,
            total_template_count: total,
            last_updated_unix_nano: now,
        };

        let duration_ms: u64 = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        let span = tracing::Span::current();
        span.record("top_n", top_n);
        span.record("result_count", result_count);
        span.record("duration_ms", duration_ms);

        tracing::info!(
            target: "diagnostics.template_distribution.request",
            top_n = top_n,
            result_count = result_count,
            duration_ms = duration_ms,
            "diagnostics.template_distribution returned",
        );

        Ok(payload)
    }
}

fn template_entry_to_payload(entry: TemplateDistEntry) -> TemplateDistEntryPayload {
    TemplateDistEntryPayload {
        id: entry.id,
        content: entry.content,
        occurrence_count: entry.occurrence_count,
        drift_indicator: entry.drift_indicator.into(),
    }
}

fn current_unix_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

/// Sanitized boundary conversion. Per arch §Established Decisions
/// [Error Handling Pattern]: no stack traces, file paths, library
/// versions, or template body strings appear in the message surfaced
/// to the webview (the reason field carried inside `buffer::Error::Drain`
/// can include implementation-detail noise that MUST NOT cross the
/// bridge per security plan §Logging NEVER-log discipline).
///
/// Free function (not `From` impl) for symmetry with
/// `storage_router::corpus_error_to_app_error` (chunk #68) + as the
/// forward slot for Session 4 persistence error paths. The
/// `From<buffer::Error> for AppError` impl in `ui-bridge::contract`
/// also exists for `?`-propagation in non-resolver call sites; this
/// free function gives the resolver an explicit, route-local sanitizer
/// when the default From impl's message is wrong or too detailed for
/// the bridge surface.
pub fn drain_error_to_app_error(err: buffer::Error) -> AppError {
    use buffer::Error as B;
    let message = match err {
        B::Init { .. } => "buffer init failed",
        B::SchemaCreate { .. } => "buffer schema creation failed",
        B::Append { .. } => "buffer append failed",
        B::ConnectionLost => "buffer connection lost",
        B::InvalidBatch { .. } => "buffer rejected invalid batch",
        B::Retention { .. } => "buffer retention sweep failed",
        B::BroadcastEncode { .. } => "buffer broadcast encode failed",
        B::BroadcastSizeCapExceeded { .. } => "buffer broadcast payload too large",
        B::Drain { .. } => "drain template miner failed",
    };
    AppError::Storage {
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffer::{DrainConfig, DrainMiner};

    fn make_impl() -> DiagnosticsApiImpl {
        let miner = Arc::new(DrainMiner::new(DrainConfig::default_config(), None));
        DiagnosticsApiImpl::new(
            miner,
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
            let roundtrip: DriftIndicatorPayload =
                serde_json::from_str(&json).expect("deserialize");
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
        let roundtrip: TemplateDistributionPayload =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(roundtrip, payload);
    }
}
