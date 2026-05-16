use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use crate::aggregation::aggregate_metrics;
pub use crate::anomaly::detect_anomalies;
pub use crate::critical_path::extract_critical_path;
pub use crate::dedupe::{DedupResult, dedupe_spans};

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
}
