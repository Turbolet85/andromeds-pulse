use std::collections::BTreeMap;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::instrument;

use crate::aggregation::aggregate_metrics;
use crate::anomaly::detect_anomalies;
use crate::attribute_filter::filter_attributes;
use crate::critical_path::extract_critical_path;
use crate::dedupe::dedupe_spans;

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

/// OTLP-native span identity input shape consumed by curation primitives.
/// trace_id / span_id are raw bytes per OTLP spec (16 / 8 length); primitives
/// never reconstruct hex strings or surrogate keys internally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpanRecord {
    pub trace_id: [u8; 16],
    pub span_id: [u8; 8],
    pub parent_span_id: Option<[u8; 8]>,
    pub service_name: String,
    pub name: String,
    pub start_time_unix_nano: i64,
    pub end_time_unix_nano: i64,
    /// OTLP Status.code: 0=Unset, 1=Ok, 2=Error per OTLP Trace v1 spec.
    pub status_code: u8,
    // Defaulting via serde keeps chunk #39 fixtures + downstream callers
    // without attribute data round-trip-compatible.
    #[serde(default)]
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyKind {
    LatencyOutlier,
    ErrorCorrelation,
    CardinalitySpike,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnomalyMarker {
    pub kind: AnomalyKind,
    pub severity: u8,
    pub rationale: String,
    pub affected_span_ids: Vec<[u8; 8]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalPathStep {
    pub span_id: [u8; 8],
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CurationOutput {
    pub unique_spans: Vec<SpanRecord>,
    pub dedup_count: usize,
    pub anomaly_markers: Vec<AnomalyMarker>,
    pub critical_path: Vec<CriticalPathStep>,
    pub input_row_count: usize,
    pub output_row_count: usize,
    // chunk #40 — aggregate latency percentiles + attribute-filter counts.
    // Defaulted via serde so chunk #39 round-trip fixtures remain compatible.
    #[serde(default)]
    pub aggregation: AggregationResult,
    #[serde(default)]
    pub kept_attribute_count: usize,
    #[serde(default)]
    pub dropped_attribute_count: usize,
}

// Per-bucket latency percentiles. p50/p95/p99/max are spec-locked per arch
// §Established Decisions [Snapshot Curation Default]; sample_count carries
// how many spans fed the percentile so degenerate buckets (0-2 samples)
// are detectable downstream without re-walking the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ServicePercentiles {
    pub p50_ms: u64,
    pub p95_ms: u64,
    pub p99_ms: u64,
    pub max_ms: u64,
    pub sample_count: usize,
}

// Per-service + global latency aggregation output. BTreeMap chosen so
// iteration order is deterministic by service name (chunk #39 precedent
// for ordering invariants).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AggregationResult {
    pub per_service: BTreeMap<String, ServicePercentiles>,
    pub global: ServicePercentiles,
}

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
    fn anomaly_kind_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&AnomalyKind::LatencyOutlier).unwrap(),
            "\"latency_outlier\""
        );
        assert_eq!(
            serde_json::to_string(&AnomalyKind::ErrorCorrelation).unwrap(),
            "\"error_correlation\""
        );
        assert_eq!(
            serde_json::to_string(&AnomalyKind::CardinalitySpike).unwrap(),
            "\"cardinality_spike\""
        );
    }

    #[test]
    fn anomaly_kind_round_trips_through_serde() {
        for kind in [
            AnomalyKind::LatencyOutlier,
            AnomalyKind::ErrorCorrelation,
            AnomalyKind::CardinalitySpike,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let parsed: AnomalyKind = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, kind);
        }
    }

    #[test]
    fn span_record_round_trips_through_serde() {
        let s = span_for_test(0xAA, 1);
        let json = serde_json::to_string(&s).unwrap();
        let parsed: SpanRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, s);
    }

    #[test]
    fn anomaly_marker_round_trips_through_serde() {
        let m = AnomalyMarker {
            kind: AnomalyKind::LatencyOutlier,
            severity: 75,
            rationale: "p99 exceeded baseline by 3.2σ".to_string(),
            affected_span_ids: vec![[1; 8], [2; 8]],
        };
        let json = serde_json::to_string(&m).unwrap();
        let parsed: AnomalyMarker = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, m);
    }

    #[test]
    fn critical_path_step_round_trips_through_serde() {
        let step = CriticalPathStep {
            span_id: [42; 8],
            duration_ms: 123,
        };
        let json = serde_json::to_string(&step).unwrap();
        let parsed: CriticalPathStep = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, step);
    }

    #[test]
    fn curation_output_round_trips_through_serde() {
        let out = CurationOutput {
            unique_spans: vec![span_for_test(0xAA, 1)],
            dedup_count: 0,
            anomaly_markers: Vec::new(),
            critical_path: vec![CriticalPathStep {
                span_id: [0xAA; 8],
                duration_ms: 10,
            }],
            input_row_count: 1,
            output_row_count: 1,
            aggregation: AggregationResult::default(),
            kept_attribute_count: 0,
            dropped_attribute_count: 0,
        };
        let json = serde_json::to_string(&out).unwrap();
        let parsed: CurationOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, out);
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
}
