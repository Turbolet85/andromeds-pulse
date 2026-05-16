use std::time::Instant;

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
// budget; Applied = phase B/C truncation occurred с reported drop counts.
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
}
