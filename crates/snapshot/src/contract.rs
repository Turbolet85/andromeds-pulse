use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::instrument;

// chunk #58 — curation primitives + their shared types moved to crates/curation.
// snapshot::contract re-exports them so external callers depending on path
// `snapshot::contract::{SpanRecord, AnomalyMarker, ...}` continue to compile
// unchanged. Per arch §Cross-cutting Patterns "Module dependency direction":
// snapshot now depends on curation; curation has no dependency on snapshot.
pub use curation::contract::{
    AggregationResult, AnomalyKind, AnomalyMarker, CriticalPathStep, CurationOutput,
    ServicePercentiles, SpanRecord, aggregate_metrics, dedupe_spans, detect_anomalies,
    extract_critical_path,
};

use crate::attribute_filter::filter_attributes;
pub use crate::markdown::format_markdown;
pub use crate::token_budget::TokenBudget;

// Attribute value byte cap — any kept attribute value longer than this is
// truncated with U+2026 ellipsis sentinel before emission. Defends against
// runaway values (e.g., 1 MB stack-trace embedded in an `error` attribute)
// inflating snapshot output. Per security plan §Input Validation
// (post-prost row: attribute keys/values bounded).
pub const MAX_ATTRIBUTE_VALUE_BYTES: usize = 256;

// Per-span attribute count cap — informational bound documenting the
// expected upper limit. Filter does not enforce; callers MAY clamp at
// construction time. Per security plan §Input Validation.
pub const MAX_ATTRIBUTES_PER_SPAN: usize = 32;

// Attribute filter output: the spans with only allowlisted keys retained
// (values capped at MAX_ATTRIBUTE_VALUE_BYTES) plus aggregate counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AttributeFilterResult {
    pub spans: Vec<SpanRecord>,
    pub kept_attribute_count: usize,
    pub dropped_attribute_count: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("input span vector is empty")]
    EmptyInput,
    #[error("invalid span record: {kind}")]
    InvalidSpanRecord { kind: &'static str },
    #[error("orphan parent_span_id reference")]
    OrphanParentSpan { parent: [u8; 8] },
    #[error("latency distribution degenerate: {reason}")]
    LatencyDistributionDegenerate { reason: &'static str },
}

// chunk #41 — markdown formatter truncation tracking. None = full input fit
// budget; Applied = phase B/C truncation occurred with reported drop counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TruncationState {
    #[default]
    None,
    Applied {
        dropped_span_count: usize,
        dropped_attribute_count: usize,
    },
}

// chunk #41 — markdown formatter output envelope. Carries the rendered
// markdown body + size + budget + truncation state so downstream consumers
// (chunk #43 snapshot.generate IPC, chunk #46 MCP generate_snapshot) can
// route truncation-state to UI aria-live regions per a11y plan §7.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MarkdownReport {
    pub markdown: String,
    pub token_count: usize,
    pub budget: TokenBudget,
    pub truncation_state: TruncationState,
}

// chunk #41 — formatter-internal error enum. Distinct from snapshot::Error
// (which covers curate() input validity) so the existing exhaustive match
// in `From<SnapshotError> for AppError` at ui-bridge stays untouched until
// chunk #43 IPC wiring lands.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FormatError {
    #[error("token budget exceeded: actual={actual_tokens}, budget={budget_tokens}")]
    BudgetExceeded {
        budget_tokens: usize,
        actual_tokens: usize,
    },
    #[error("anchor encoding failed: {reason}")]
    AnchorEncodingFailed { reason: &'static str },
}

/// Run filter_attributes → dedupe → aggregate → anomaly detection →
/// critical-path extraction in deterministic order over the input span set.
/// Pure function — no I/O, no system clock for logic; reproducible
/// byte-identical output for identical input.
#[instrument(
    skip_all,
    fields(
        input_row_count = spans.len(),
        output_row_count = tracing::field::Empty,
        dedup_count = tracing::field::Empty,
        anomaly_markers_count = tracing::field::Empty,
        critical_path_span_count = tracing::field::Empty,
        kept_attribute_count = tracing::field::Empty,
        dropped_attribute_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ),
)]
pub fn curate(spans: &[SpanRecord]) -> Result<CurationOutput, Error> {
    let started = Instant::now();
    let span = tracing::Span::current();

    if spans.is_empty() {
        span.record("output_row_count", 0_u64);
        span.record("dedup_count", 0_u64);
        span.record("anomaly_markers_count", 0_u64);
        span.record("critical_path_span_count", 0_u64);
        span.record("kept_attribute_count", 0_u64);
        span.record("dropped_attribute_count", 0_u64);
        span.record("duration_ms", started.elapsed().as_millis() as u64);
        return Ok(CurationOutput::default());
    }

    let filter = filter_attributes(spans);
    let dedup = dedupe_spans(&filter.spans);
    let aggregation = aggregate_metrics(&dedup.unique_spans);
    let anomaly_markers = detect_anomalies(&dedup.unique_spans);
    let critical_path = extract_critical_path(&dedup.unique_spans);

    span.record("output_row_count", dedup.output_row_count as u64);
    span.record("dedup_count", dedup.dedup_count as u64);
    span.record("anomaly_markers_count", anomaly_markers.len() as u64);
    span.record("critical_path_span_count", critical_path.len() as u64);
    span.record("kept_attribute_count", filter.kept_attribute_count as u64);
    span.record(
        "dropped_attribute_count",
        filter.dropped_attribute_count as u64,
    );
    span.record("duration_ms", started.elapsed().as_millis() as u64);

    Ok(CurationOutput {
        unique_spans: dedup.unique_spans,
        dedup_count: dedup.dedup_count,
        anomaly_markers,
        critical_path,
        input_row_count: dedup.input_row_count,
        output_row_count: dedup.output_row_count,
        aggregation,
        kept_attribute_count: filter.kept_attribute_count,
        dropped_attribute_count: filter.dropped_attribute_count,
    })
}

// The `metric.snapshot.token_count_ms` sample spans one whole generation —
// span load → curate → format — which is what the obs-plan §10 snapshot budget
// bounds (§4 P2 chain). Each orchestrator starts the timer before its span load
// and finishes it once formatting returns; `format_markdown` itself emits no
// sample. A generation that fails before formatting emits nothing.
#[derive(Debug, Clone, Copy)]
pub struct GenerationTimer {
    started: Instant,
}

impl GenerationTimer {
    pub fn start() -> Self {
        Self {
            started: Instant::now(),
        }
    }

    pub fn finish(
        self,
        formatted: &Result<MarkdownReport, FormatError>,
        curated: &CurationOutput,
        budget: TokenBudget,
    ) {
        if let Some(sample) =
            GenerationSample::new(self.started.elapsed(), formatted, curated, budget)
        {
            sample.emit();
        }
    }
}

// One generation's metric fields, split from the clock so a test can build it
// from a synthetic `Duration`.
#[derive(Debug, Clone, PartialEq)]
pub struct GenerationSample {
    pub duration_ms: f64,
    pub token_budget: &'static str,
    pub token_count_actual: usize,
    pub dedup_count: usize,
    pub budget_exceeded: bool,
}

impl GenerationSample {
    pub fn new(
        elapsed: Duration,
        formatted: &Result<MarkdownReport, FormatError>,
        curated: &CurationOutput,
        budget: TokenBudget,
    ) -> Option<Self> {
        let (token_count_actual, budget_exceeded) = match formatted {
            Ok(report) => (report.token_count, false),
            Err(FormatError::BudgetExceeded { actual_tokens, .. }) => (*actual_tokens, true),
            Err(FormatError::AnchorEncodingFailed { .. }) => return None,
        };
        Some(Self {
            duration_ms: elapsed.as_secs_f64() * 1000.0,
            token_budget: budget.label(),
            token_count_actual,
            dedup_count: curated.dedup_count,
            budget_exceeded,
        })
    }

    pub fn emit(&self) {
        tracing::info!(
            target: "metric.snapshot.token_count_ms",
            value = self.duration_ms,
            duration_ms = self.duration_ms,
            token_budget = self.token_budget,
            time_range_minutes = 0_u64,
            token_count_actual = self.token_count_actual,
            dedup_count = self.dedup_count,
            budget_exceeded = self.budget_exceeded,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_for_test(span_id_byte: u8, status: u8) -> SpanRecord {
        SpanRecord {
            trace_id: [1; 16],
            span_id: [span_id_byte; 8],
            parent_span_id: None,
            service_name: "checkout".to_string(),
            name: "POST /pay".to_string(),
            start_time_unix_nano: 1_000_000_000,
            end_time_unix_nano: 1_010_000_000,
            status_code: status,
            attributes: Vec::new(),
        }
    }

    #[test]
    fn error_display_includes_kind_and_reason() {
        let e = Error::InvalidSpanRecord {
            kind: "negative_duration",
        };
        assert!(format!("{e}").contains("negative_duration"));
        let e = Error::LatencyDistributionDegenerate {
            reason: "all_durations_equal",
        };
        assert!(format!("{e}").contains("all_durations_equal"));
        assert_eq!(
            format!("{}", Error::OrphanParentSpan { parent: [0; 8] }),
            "orphan parent_span_id reference"
        );
        assert_eq!(
            format!("{}", Error::EmptyInput),
            "input span vector is empty"
        );
    }

    #[test]
    fn curate_empty_input_returns_empty_output() {
        let out = curate(&[]).expect("curate ok on empty");
        assert_eq!(out.unique_spans, Vec::<SpanRecord>::new());
        assert_eq!(out.dedup_count, 0);
        assert!(out.anomaly_markers.is_empty());
        assert!(out.critical_path.is_empty());
        assert_eq!(out.input_row_count, 0);
        assert_eq!(out.output_row_count, 0);
    }

    #[test]
    fn curate_single_span_returns_trivial_output() {
        let s = span_for_test(0xAA, 1);
        let out = curate(std::slice::from_ref(&s)).expect("curate ok");
        assert_eq!(out.unique_spans.len(), 1);
        assert_eq!(out.dedup_count, 0);
        assert_eq!(out.input_row_count, 1);
        assert_eq!(out.output_row_count, 1);
        assert_eq!(out.critical_path.len(), 1);
        assert_eq!(out.critical_path[0].span_id, s.span_id);
    }

    #[test]
    fn curate_two_identical_spans_collapses_dedup_count() {
        let s = span_for_test(0xAA, 1);
        let out = curate(&[s.clone(), s]).expect("curate ok");
        assert_eq!(out.unique_spans.len(), 1);
        assert!(out.dedup_count >= 1);
        assert_eq!(out.input_row_count, 2);
        assert_eq!(out.output_row_count, 1);
    }

    #[test]
    fn truncation_state_default_is_none() {
        assert_eq!(TruncationState::default(), TruncationState::None);
    }

    #[test]
    fn truncation_state_round_trips_through_serde() {
        for state in [
            TruncationState::None,
            TruncationState::Applied {
                dropped_span_count: 5,
                dropped_attribute_count: 12,
            },
        ] {
            let json = serde_json::to_string(&state).expect("serializes");
            let parsed: TruncationState = serde_json::from_str(&json).expect("parses");
            assert_eq!(parsed, state);
        }
    }

    #[test]
    fn markdown_report_round_trips_through_serde() {
        let report = MarkdownReport {
            markdown: "# title\n## section\nbody\n".to_string(),
            token_count: 8,
            budget: TokenBudget::Balanced,
            truncation_state: TruncationState::Applied {
                dropped_span_count: 5,
                dropped_attribute_count: 12,
            },
        };
        let json = serde_json::to_string(&report).expect("serializes");
        let parsed: MarkdownReport = serde_json::from_str(&json).expect("parses");
        assert_eq!(parsed, report);
    }

    #[test]
    fn markdown_report_default_uses_balanced_budget_and_none_truncation() {
        let r = MarkdownReport::default();
        assert_eq!(r.budget, TokenBudget::Balanced);
        assert_eq!(r.truncation_state, TruncationState::None);
        assert!(r.markdown.is_empty());
        assert_eq!(r.token_count, 0);
    }

    #[test]
    fn format_error_budget_exceeded_display_contains_actual_and_budget() {
        let e = FormatError::BudgetExceeded {
            budget_tokens: 10_000,
            actual_tokens: 12_345,
        };
        let s = format!("{e}");
        assert!(s.contains("10000"));
        assert!(s.contains("12345"));
    }

    fn curated_with_dedup(dedup_count: usize) -> CurationOutput {
        CurationOutput {
            dedup_count,
            ..CurationOutput::default()
        }
    }

    #[test]
    fn generation_timer_sample_carries_fractional_ms_from_the_elapsed_duration() {
        let formatted = Ok(MarkdownReport::default());
        let sample = GenerationSample::new(
            Duration::from_micros(61_250),
            &formatted,
            &curated_with_dedup(0),
            TokenBudget::Balanced,
        )
        .expect("a formatted generation yields a sample");
        assert_eq!(sample.duration_ms, 61.25);
    }

    #[test]
    fn generation_timer_ok_arm_reads_the_report_token_count_and_is_not_exceeded() {
        let formatted = Ok(MarkdownReport {
            token_count: 812,
            ..MarkdownReport::default()
        });
        let sample = GenerationSample::new(
            Duration::from_millis(3),
            &formatted,
            &curated_with_dedup(7),
            TokenBudget::Detailed,
        )
        .expect("sample");
        assert_eq!(
            sample,
            GenerationSample {
                duration_ms: 3.0,
                token_budget: TokenBudget::Detailed.label(),
                token_count_actual: 812,
                dedup_count: 7,
                budget_exceeded: false,
            }
        );
    }

    #[test]
    fn generation_timer_budget_exceeded_arm_sets_the_flag_and_the_actual_tokens() {
        let formatted = Err(FormatError::BudgetExceeded {
            budget_tokens: 10_000,
            actual_tokens: 12_345,
        });
        let sample = GenerationSample::new(
            Duration::from_millis(40),
            &formatted,
            &curated_with_dedup(2),
            TokenBudget::Conservative,
        )
        .expect("a budget-exceeded generation still yields a sample");
        assert!(sample.budget_exceeded);
        assert_eq!(sample.token_count_actual, 12_345);
        assert_eq!(sample.token_budget, TokenBudget::Conservative.label());
    }

    #[test]
    fn generation_timer_anchor_encoding_failure_yields_no_sample() {
        let formatted = Err(FormatError::AnchorEncodingFailed { reason: "test" });
        assert_eq!(
            GenerationSample::new(
                Duration::from_millis(1),
                &formatted,
                &curated_with_dedup(0),
                TokenBudget::Balanced,
            ),
            None
        );
    }

    struct TargetCounter {
        target: &'static str,
        hits: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    }

    impl tracing::Subscriber for TargetCounter {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }
        fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
        fn event(&self, event: &tracing::Event<'_>) {
            if event.metadata().target() == self.target {
                self.hits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        fn enter(&self, _: &tracing::span::Id) {}
        fn exit(&self, _: &tracing::span::Id) {}
    }

    // Both halves in one scope: a formatter that still emitted would read 2, and a
    // capture that saw nothing would read 0 — so the 1 cannot pass vacuously.
    #[test]
    fn generation_timer_is_the_only_metric_emitter_across_a_whole_generation() {
        let hits = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let subscriber = TargetCounter {
            target: "metric.snapshot.token_count_ms",
            hits: std::sync::Arc::clone(&hits),
        };
        tracing::subscriber::with_default(subscriber, || {
            let timer = GenerationTimer::start();
            let curated = curate(&[span_for_test(0xAA, 1)]).expect("curate ok");
            let formatted = format_markdown(&curated, TokenBudget::Balanced);
            assert!(formatted.is_ok());
            timer.finish(&formatted, &curated, TokenBudget::Balanced);
        });
        assert_eq!(hits.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
