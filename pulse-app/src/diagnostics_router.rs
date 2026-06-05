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
use interpretation::contract::{LlmInferenceRunner, model_status_label, model_tier_label};
use interpretation::degraded_mode::{DegradedModeStatus, degraded_mode_state_label};
use interpretation::hardware::profile_label;
use serde::{Deserialize, Serialize};
use triage::contract::HardwareProfileSource;
use ui_bridge::contract::AppError;

use crate::reevaluation::RecentWindowReevaluator;

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

/// Chunk #96 — `reevaluate_recent_window` payload (capability P-056). Counts
/// services re-classified + transitions emitted by the opt-in retrospective
/// pass; aggregate-only (no per-service identifiers).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ReevaluateWindowPayload {
    pub services_reclassified: u64,
    pub transitions_emitted: u64,
    pub cadence_prospective: bool,
}

/// Chunk #97 — `diagnostics.snapshot()` payload (capability P-058). A
/// point-in-time aggregate of the L6 self-observability state that already
/// exists in-process. Sub-fields with no production producer yet (inference
/// success rate, queue depth, per-layer L0-L5 numerics) are `None` / `false`
/// and rendered as "not yet recorded" by the webview — NEVER fabricated
/// (hybrid-render scope decision). The Connection + Templates sections reuse
/// the existing `connection.current_state` + `diagnostics.template_distribution`
/// resolvers webview-side, so this payload carries only the Model / Hardware /
/// Pipeline sections that have no dedicated resolver.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct DiagnosticsSnapshotPayload {
    pub model: ModelSectionPayload,
    pub hardware: HardwareSectionPayload,
    pub pipeline: PipelineSectionPayload,
    pub captured_unix_nano: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ModelSectionPayload {
    pub tier_label: String,
    pub profile_label: String,
    pub load_status: String,
    pub model_identity_name: Option<String>,
    pub backoff_state_label: String,
    pub backoff_remaining_seconds: u64,
    pub consecutive_failures: u32,
    /// Inference success rate in basis points (10000 = 100.00%). `None`
    /// until a numeric-metric-history producer lands; basis-points keeps
    /// the payload `Eq` (no `f64`).
    pub inference_success_rate_basis_points: Option<u32>,
    /// L4 inference queue depth. `None` until a producer records it.
    pub queue_depth: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct HardwareSectionPayload {
    pub profile_label: String,
    pub detection_detail: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct PipelineSectionPayload {
    pub drain_template_count: u64,
    /// `false` until per-layer L0-L5 numeric metrics are produced; the
    /// webview renders a "not yet recorded" notice when false.
    pub per_layer_recorded: bool,
}

/// Chunk #97 — `diagnostics.history(metric_name, window_seconds)` payload
/// (capability P-058). STUB this chunk: no numeric-metric-history producer
/// exists (the corpus `pipeline_metrics` table stores opaque latest-only
/// state-snapshot blobs, not per-metric series), so `points` is empty +
/// `recorded` is false + `notice` explains. The procedure still validates
/// `metric_name` + bounds the window so the contract is stable for a future
/// producer chunk.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct DiagnosticsHistoryPayload {
    pub metric_name: String,
    pub points: Vec<MetricHistoryPoint>,
    pub recorded: bool,
    pub notice: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct MetricHistoryPoint {
    pub snapshot_unix_nano: i64,
    pub value_basis_points: u32,
}

/// Bounded allowlist of metric names `diagnostics.history` accepts. Gates
/// the `metric_name` arg so it can never be used as an unbounded key (per
/// security plan §Input Validation). Extend when a producer adds a series.
pub const DIAGNOSTICS_HISTORY_METRICS: &[&str] = &[
    "inference_success_rate",
    "queue_depth",
    "drain_template_count",
    "connection_last_span_ago_ms",
];

/// Upper bound on the `window_seconds` arg (30 days), matching the corpus
/// 30-day default retention. Larger requests clamp to this.
pub const DIAGNOSTICS_HISTORY_MAX_WINDOW_SECONDS: u64 = 30 * 24 * 60 * 60;

// Chunk #86 — `retry_interpretation` is the manual override for the L4
// degraded-mode FSM (resets the consecutive-failure counter to 0 and
// transitions Degraded → Active immediately). Chunk #96 —
// `reevaluate_recent_window` is the opt-in retrospective re-evaluation
// (re-classify every tracked service against the freshly hot-reloaded
// lifecycle thresholds). Chunk #97 — `snapshot` is the point-in-time L6
// self-observability aggregate; `history` is the (currently stubbed)
// per-metric time-series accessor. Doc lives above the macro (taurpc 0.7
// rejects multi-line /// attributes inside the trait body).
#[taurpc::procedures(path = "diagnostics")]
pub trait DiagnosticsApi {
    async fn template_distribution() -> Result<TemplateDistributionPayload, AppError>;
    async fn retry_interpretation() -> Result<RetryInterpretationPayload, AppError>;
    async fn reevaluate_recent_window() -> Result<ReevaluateWindowPayload, AppError>;
    async fn snapshot() -> Result<DiagnosticsSnapshotPayload, AppError>;
    async fn history(
        metric_name: String,
        window_seconds: u64,
    ) -> Result<DiagnosticsHistoryPayload, AppError>;
}

#[derive(Clone)]
pub struct DiagnosticsApiImpl {
    miner: Arc<DrainMiner>,
    degraded_mode: Arc<dyn DegradedModeStatus>,
    reevaluator: Arc<dyn RecentWindowReevaluator>,
    runner: Arc<dyn LlmInferenceRunner>,
    profile_source: Arc<dyn HardwareProfileSource>,
}

impl DiagnosticsApiImpl {
    pub fn new(
        miner: Arc<DrainMiner>,
        degraded_mode: Arc<dyn DegradedModeStatus>,
        reevaluator: Arc<dyn RecentWindowReevaluator>,
        runner: Arc<dyn LlmInferenceRunner>,
        profile_source: Arc<dyn HardwareProfileSource>,
    ) -> Self {
        Self {
            miner,
            degraded_mode,
            reevaluator,
            runner,
            profile_source,
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
        services_reclassified = tracing::field::Empty,
        transitions_emitted = tracing::field::Empty,
    ))]
    async fn reevaluate_recent_window(self) -> Result<ReevaluateWindowPayload, AppError> {
        let summary = self.reevaluator.reevaluate();
        let payload = ReevaluateWindowPayload {
            services_reclassified: summary.services_reclassified,
            transitions_emitted: summary.transitions_emitted,
            cadence_prospective: summary.cadence_prospective,
        };
        let span = tracing::Span::current();
        span.record("services_reclassified", payload.services_reclassified);
        span.record("transitions_emitted", payload.transitions_emitted);
        tracing::info!(
            target: "diagnostics.reevaluate_recent_window.request",
            services_reclassified = payload.services_reclassified,
            transitions_emitted = payload.transitions_emitted,
            "diagnostics.reevaluate_recent_window returned",
        );
        Ok(payload)
    }

    #[tracing::instrument(skip_all, fields(
        tier = tracing::field::Empty,
        profile = tracing::field::Empty,
        load_status = tracing::field::Empty,
        backoff_remaining_seconds = tracing::field::Empty,
        consecutive_failures = tracing::field::Empty,
        drain_template_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ))]
    async fn snapshot(self) -> Result<DiagnosticsSnapshotPayload, AppError> {
        let start = std::time::Instant::now();
        let now = current_unix_nanos();

        let profile = self.profile_source.current_profile();
        let tier = self.runner.tier();
        let status = self.runner.current_status();
        let identity = self.runner.identity();
        let dm = self.degraded_mode.current_snapshot(now);
        let drain_template_count = self.miner.template_count();

        let model = ModelSectionPayload {
            tier_label: model_tier_label(tier).to_string(),
            profile_label: profile_label(profile).to_string(),
            load_status: model_status_label(status).to_string(),
            model_identity_name: identity.map(|i| i.semantic_name),
            backoff_state_label: degraded_mode_state_label(dm.state).to_string(),
            backoff_remaining_seconds: dm.backoff_seconds_remaining,
            consecutive_failures: dm.consecutive_failures,
            inference_success_rate_basis_points: None,
            queue_depth: None,
        };
        let hardware = HardwareSectionPayload {
            profile_label: profile_label(profile).to_string(),
            detection_detail: None,
        };
        let pipeline = PipelineSectionPayload {
            drain_template_count,
            per_layer_recorded: false,
        };
        let payload = DiagnosticsSnapshotPayload {
            model,
            hardware,
            pipeline,
            captured_unix_nano: now,
        };

        let duration_ms: u64 = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let span = tracing::Span::current();
        span.record("tier", payload.model.tier_label.as_str());
        span.record("profile", payload.hardware.profile_label.as_str());
        span.record("load_status", payload.model.load_status.as_str());
        span.record(
            "backoff_remaining_seconds",
            payload.model.backoff_remaining_seconds,
        );
        span.record("consecutive_failures", payload.model.consecutive_failures);
        span.record(
            "drain_template_count",
            payload.pipeline.drain_template_count,
        );
        span.record("duration_ms", duration_ms);

        tracing::info!(
            target: "diagnostics.snapshot.request",
            tier = payload.model.tier_label.as_str(),
            profile = payload.hardware.profile_label.as_str(),
            load_status = payload.model.load_status.as_str(),
            backoff_remaining_seconds = payload.model.backoff_remaining_seconds,
            consecutive_failures = payload.model.consecutive_failures,
            drain_template_count = payload.pipeline.drain_template_count,
            duration_ms = duration_ms,
            "diagnostics.snapshot returned",
        );

        Ok(payload)
    }

    #[tracing::instrument(skip_all, fields(
        metric_name = tracing::field::Empty,
        recorded = tracing::field::Empty,
        point_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ))]
    async fn history(
        self,
        metric_name: String,
        window_seconds: u64,
    ) -> Result<DiagnosticsHistoryPayload, AppError> {
        let start = std::time::Instant::now();

        if !DIAGNOSTICS_HISTORY_METRICS.contains(&metric_name.as_str()) {
            return Err(AppError::Validation {
                field: "metric_name".to_string(),
                reason: "unknown diagnostics metric".to_string(),
            });
        }
        // Window bounded к the 30-day corpus retention; clamp rather than
        // reject so the contract is forgiving for a future producer chunk.
        let _bounded_window = window_seconds.min(DIAGNOSTICS_HISTORY_MAX_WINDOW_SECONDS);

        // Hybrid-render scope (chunk #97): no numeric-metric-history producer
        // exists yet — `pipeline_metrics` stores opaque latest-only state
        // blobs, not per-metric series. Return an empty series + notice; the
        // contract stays stable for a future producer chunk. No corpus query
        // runs (so the prepared-statement / query-anonymizer disciplines are
        // N/A-by-construction this chunk).
        let payload = DiagnosticsHistoryPayload {
            metric_name: metric_name.clone(),
            points: Vec::new(),
            recorded: false,
            notice: "metric history recording not yet available".to_string(),
        };

        let duration_ms: u64 = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let point_count: u64 = u64::try_from(payload.points.len()).unwrap_or(u64::MAX);
        let span = tracing::Span::current();
        span.record("metric_name", metric_name.as_str());
        span.record("recorded", payload.recorded);
        span.record("point_count", point_count);
        span.record("duration_ms", duration_ms);

        tracing::info!(
            target: "diagnostics.history.request",
            metric_name = metric_name.as_str(),
            recorded = payload.recorded,
            point_count = point_count,
            duration_ms = duration_ms,
            "diagnostics.history returned",
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

    fn noop_reevaluator() -> Arc<dyn RecentWindowReevaluator> {
        struct Noop;
        impl RecentWindowReevaluator for Noop {
            fn reevaluate(&self) -> crate::reevaluation::ReevaluationSummary {
                crate::reevaluation::ReevaluationSummary {
                    services_reclassified: 0,
                    transitions_emitted: 0,
                    cadence_prospective: true,
                }
            }
        }
        Arc::new(Noop)
    }

    fn test_runner() -> Arc<dyn LlmInferenceRunner> {
        Arc::new(crate::llamacli_inference::LlamaCliInference::new(
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
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
            Arc::new(crate::degraded_mode_runtime::LocalDegradedModeStatus::new()),
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
