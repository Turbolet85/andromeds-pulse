use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use tracing::instrument;

use crate::contract::{AnomalyKind, AnomalyMarker, SpanRecord};

// Z-score threshold for latency-outlier detection. 3.0 ≈ p99.7 under
// normal distribution; common rule-of-thumb in ops dashboards.
pub(crate) const LATENCY_OUTLIER_Z_THRESHOLD: f64 = 3.0;

// Minimum errors per service before flagging an error cluster. Three errors
// in one service is the threshold above which clustering becomes signal
// rather than noise from one-off failures.
pub(crate) const ERROR_CLUSTER_MIN_ERRORS: usize = 3;

// Cardinality spike multiplier vs median unique-span-name count across
// services. A service with 2× the median is flagged as anomalous span
// keyspace explosion.
pub(crate) const CARDINALITY_SPIKE_MULTIPLIER: f64 = 2.0;

#[instrument(
    skip_all,
    fields(
        input_row_count = spans.len(),
        latency_outlier_count = tracing::field::Empty,
        error_cluster_count = tracing::field::Empty,
        cardinality_spike_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ),
)]
pub fn detect_anomalies(spans: &[SpanRecord]) -> Vec<AnomalyMarker> {
    let started = Instant::now();

    let latency = detect_latency_outliers(spans, LATENCY_OUTLIER_Z_THRESHOLD);
    let errors = detect_error_correlation(spans);
    let cardinality = detect_cardinality_spikes(spans);

    let span = tracing::Span::current();
    span.record("latency_outlier_count", latency.len() as u64);
    span.record("error_cluster_count", errors.len() as u64);
    span.record("cardinality_spike_count", cardinality.len() as u64);

    let mut all = Vec::with_capacity(latency.len() + errors.len() + cardinality.len());
    all.extend(latency);
    all.extend(errors);
    all.extend(cardinality);

    all.sort_by_key(|m| {
        (
            Reverse(m.severity),
            kind_ordinal(m.kind),
            m.affected_span_ids.first().copied().unwrap_or([0; 8]),
        )
    });

    span.record("duration_ms", started.elapsed().as_millis() as u64);
    all
}

fn kind_ordinal(kind: AnomalyKind) -> u8 {
    match kind {
        AnomalyKind::LatencyOutlier => 0,
        AnomalyKind::ErrorCorrelation => 1,
        AnomalyKind::CardinalitySpike => 2,
    }
}

pub(crate) fn detect_latency_outliers(
    spans: &[SpanRecord],
    z_threshold: f64,
) -> Vec<AnomalyMarker> {
    if spans.len() < 3 {
        return Vec::new();
    }
    let durations_ms: Vec<f64> = spans.iter().map(|s| span_duration_ms(s) as f64).collect();
    let n = durations_ms.len() as f64;
    let mean = durations_ms.iter().sum::<f64>() / n;
    let variance = durations_ms.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();
    if std_dev == 0.0 {
        return Vec::new();
    }

    let mut markers: Vec<AnomalyMarker> = spans
        .iter()
        .zip(durations_ms.iter().copied())
        .filter_map(|(span, duration)| {
            let z = (duration - mean) / std_dev;
            if z > z_threshold {
                let severity = ((z * 25.0).clamp(0.0, 100.0)) as u8;
                Some(AnomalyMarker {
                    kind: AnomalyKind::LatencyOutlier,
                    severity,
                    rationale: format!("Latency outlier: {z:.1}σ above mean"),
                    affected_span_ids: vec![span.span_id],
                })
            } else {
                None
            }
        })
        .collect();
    markers.sort_by_key(|m| {
        (
            Reverse(m.severity),
            m.affected_span_ids.first().copied().unwrap_or([0; 8]),
        )
    });
    markers
}

pub(crate) fn detect_error_correlation(spans: &[SpanRecord]) -> Vec<AnomalyMarker> {
    let mut errors_per_service: BTreeMap<&str, Vec<&SpanRecord>> = BTreeMap::new();
    let mut total_per_service: BTreeMap<&str, usize> = BTreeMap::new();
    for span in spans {
        *total_per_service
            .entry(span.service_name.as_str())
            .or_insert(0) += 1;
        if span.status_code == 2 {
            errors_per_service
                .entry(span.service_name.as_str())
                .or_default()
                .push(span);
        }
    }

    let mut markers: Vec<AnomalyMarker> = errors_per_service
        .into_iter()
        .filter_map(|(service, errs)| {
            if errs.len() < ERROR_CLUSTER_MIN_ERRORS {
                return None;
            }
            let total = *total_per_service.get(service).unwrap_or(&errs.len());
            let error_rate = (errs.len() as f64 / total.max(1) as f64) * 100.0;
            let severity = error_rate.clamp(0.0, 100.0) as u8;
            let mut ids: Vec<[u8; 8]> = errs.iter().map(|s| s.span_id).collect();
            ids.sort();
            Some(AnomalyMarker {
                kind: AnomalyKind::ErrorCorrelation,
                severity,
                rationale: format!(
                    "Error cluster: {} errors across {} spans ({:.1}%)",
                    errs.len(),
                    total,
                    error_rate,
                ),
                affected_span_ids: ids,
            })
        })
        .collect();
    markers.sort_by_key(|m| {
        (
            Reverse(m.severity),
            m.affected_span_ids.first().copied().unwrap_or([0; 8]),
        )
    });
    markers
}

pub(crate) fn detect_cardinality_spikes(spans: &[SpanRecord]) -> Vec<AnomalyMarker> {
    if spans.is_empty() {
        return Vec::new();
    }
    let mut names_per_service: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    let mut spans_per_service: BTreeMap<&str, Vec<&SpanRecord>> = BTreeMap::new();
    for span in spans {
        names_per_service
            .entry(span.service_name.as_str())
            .or_default()
            .insert(span.name.as_str());
        spans_per_service
            .entry(span.service_name.as_str())
            .or_default()
            .push(span);
    }
    if names_per_service.len() < 2 {
        return Vec::new();
    }

    let mut counts: Vec<usize> = names_per_service.values().map(|s| s.len()).collect();
    counts.sort();
    let median = counts[counts.len() / 2] as f64;
    if median == 0.0 {
        return Vec::new();
    }
    let threshold = median * CARDINALITY_SPIKE_MULTIPLIER;

    let mut markers: Vec<AnomalyMarker> = names_per_service
        .iter()
        .filter_map(|(service, names)| {
            let unique_count = names.len() as f64;
            if unique_count < threshold {
                return None;
            }
            let spike_factor = unique_count / median;
            let severity = ((spike_factor - 1.0) * 25.0).clamp(0.0, 100.0) as u8;
            let svc_spans = spans_per_service.get(service).cloned().unwrap_or_default();
            let mut ids: Vec<[u8; 8]> = svc_spans.iter().map(|s| s.span_id).collect();
            ids.sort();
            Some(AnomalyMarker {
                kind: AnomalyKind::CardinalitySpike,
                severity,
                rationale: format!(
                    "Cardinality spike: {} unique names vs {:.0} median ({:.1}x)",
                    unique_count as u64, median, spike_factor,
                ),
                affected_span_ids: ids,
            })
        })
        .collect();
    markers.sort_by_key(|m| {
        (
            Reverse(m.severity),
            m.affected_span_ids.first().copied().unwrap_or([0; 8]),
        )
    });
    markers
}

fn span_duration_ms(span: &SpanRecord) -> u64 {
    let ns = span
        .end_time_unix_nano
        .saturating_sub(span.start_time_unix_nano);
    if ns < 0 { 0 } else { (ns / 1_000_000) as u64 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(
        span_id_byte: u8,
        service: &str,
        name: &str,
        duration_ns: i64,
        status: u8,
    ) -> SpanRecord {
        SpanRecord {
            trace_id: [1; 16],
            span_id: [span_id_byte; 8],
            parent_span_id: None,
            service_name: service.to_string(),
            name: name.to_string(),
            start_time_unix_nano: 1_000_000_000,
            end_time_unix_nano: 1_000_000_000 + duration_ns,
            status_code: status,
            attributes: Vec::new(),
        }
    }

    // ===== detect_anomalies orchestrator =====

    #[test]
    fn detect_anomalies_empty_input_returns_empty() {
        let r = detect_anomalies(&[]);
        assert!(r.is_empty());
    }

    #[test]
    fn detect_anomalies_single_span_returns_empty() {
        let s = span(0xAA, "svc", "op", 50_000_000, 1);
        let r = detect_anomalies(std::slice::from_ref(&s));
        assert!(r.is_empty());
    }

    #[test]
    fn detect_anomalies_orders_severity_descending() {
        let mut spans = Vec::new();
        // Many normal-latency spans so std_dev is meaningful.
        for i in 0..20 {
            spans.push(span(i as u8, "svc", "op", 50_000_000, 1));
        }
        // One extreme outlier.
        spans.push(span(0xFF, "svc", "op", 5_000_000_000, 1));
        // 3 errors in another service for error-cluster.
        for i in 0..5 {
            spans.push(span(0xC0 + i as u8, "svc-err", "err-op", 30_000_000, 2));
        }

        let r = detect_anomalies(&spans);
        assert!(
            r.len() >= 2,
            "expected at least latency + error markers, got {r:?}"
        );
        for window in r.windows(2) {
            assert!(
                window[0].severity >= window[1].severity,
                "severity not monotonically decreasing: {} then {}",
                window[0].severity,
                window[1].severity,
            );
        }
    }

    #[test]
    fn detect_anomalies_deterministic_across_invocations() {
        let mut spans = Vec::new();
        for i in 0..10 {
            spans.push(span(i as u8, "svc", "op", 50_000_000, 1));
        }
        spans.push(span(0xFF, "svc", "op", 5_000_000_000, 1));
        let r1 = detect_anomalies(&spans);
        let r2 = detect_anomalies(&spans);
        assert_eq!(r1, r2);
    }

    // ===== detect_latency_outliers =====

    #[test]
    fn latency_outliers_empty_input_returns_empty() {
        assert!(detect_latency_outliers(&[], 3.0).is_empty());
    }

    #[test]
    fn latency_outliers_uniform_durations_no_outlier() {
        let spans: Vec<SpanRecord> = (0..10)
            .map(|i| span(i as u8, "svc", "op", 50_000_000, 1))
            .collect();
        assert!(detect_latency_outliers(&spans, 3.0).is_empty());
    }

    #[test]
    fn latency_outliers_extreme_outlier_flagged() {
        let mut spans: Vec<SpanRecord> = (0..20)
            .map(|i| span(i as u8, "svc", "op", 50_000_000, 1))
            .collect();
        spans.push(span(0xFF, "svc", "op", 5_000_000_000, 1));
        let r = detect_latency_outliers(&spans, 3.0);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].kind, AnomalyKind::LatencyOutlier);
        assert_eq!(r[0].affected_span_ids, vec![[0xFF; 8]]);
        assert!(r[0].severity >= 75);
    }

    #[test]
    fn latency_outliers_rationale_starts_capital_contains_sigma() {
        let mut spans: Vec<SpanRecord> = (0..20)
            .map(|i| span(i as u8, "svc", "op", 50_000_000, 1))
            .collect();
        spans.push(span(0xFF, "svc", "op", 5_000_000_000, 1));
        let r = detect_latency_outliers(&spans, 3.0);
        let m = &r[0];
        assert!(m.rationale.starts_with(|c: char| c.is_ascii_uppercase()));
        assert!(m.rationale.contains('σ'));
    }

    // ===== detect_error_correlation =====

    #[test]
    fn error_correlation_no_errors_returns_empty() {
        let spans = vec![span(1, "svc", "op", 50_000_000, 1)];
        assert!(detect_error_correlation(&spans).is_empty());
    }

    #[test]
    fn error_correlation_below_threshold_returns_empty() {
        let spans = vec![
            span(1, "svc", "op", 50_000_000, 2),
            span(2, "svc", "op", 50_000_000, 2),
            // Two errors are below ERROR_CLUSTER_MIN_ERRORS = 3.
        ];
        assert!(detect_error_correlation(&spans).is_empty());
    }

    #[test]
    fn error_correlation_three_errors_per_service_flagged() {
        let spans = vec![
            span(1, "svc", "op", 50_000_000, 2),
            span(2, "svc", "op", 50_000_000, 2),
            span(3, "svc", "op", 50_000_000, 2),
            span(4, "svc", "op", 50_000_000, 1),
        ];
        let r = detect_error_correlation(&spans);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].kind, AnomalyKind::ErrorCorrelation);
        assert_eq!(r[0].affected_span_ids.len(), 3);
        assert!(r[0].rationale.contains("Error cluster"));
        assert!(r[0].rationale.contains('%'));
    }

    #[test]
    fn error_correlation_separates_services() {
        let spans = vec![
            span(1, "svc-a", "op", 50_000_000, 2),
            span(2, "svc-a", "op", 50_000_000, 2),
            span(3, "svc-a", "op", 50_000_000, 2),
            span(4, "svc-b", "op", 50_000_000, 2),
            span(5, "svc-b", "op", 50_000_000, 2),
            span(6, "svc-b", "op", 50_000_000, 2),
        ];
        let r = detect_error_correlation(&spans);
        assert_eq!(r.len(), 2);
    }

    // ===== detect_cardinality_spikes =====

    #[test]
    fn cardinality_spike_empty_input_returns_empty() {
        assert!(detect_cardinality_spikes(&[]).is_empty());
    }

    #[test]
    fn cardinality_spike_single_service_returns_empty() {
        let spans = vec![
            span(1, "svc", "op-a", 50_000_000, 1),
            span(2, "svc", "op-b", 50_000_000, 1),
        ];
        assert!(detect_cardinality_spikes(&spans).is_empty());
    }

    #[test]
    fn cardinality_spike_one_service_2x_median_flagged() {
        // svc-baseline-a + svc-baseline-b each have 2 unique names.
        let mut spans = vec![
            span(1, "svc-baseline-a", "op-1", 50_000_000, 1),
            span(2, "svc-baseline-a", "op-2", 50_000_000, 1),
            span(3, "svc-baseline-b", "op-1", 50_000_000, 1),
            span(4, "svc-baseline-b", "op-2", 50_000_000, 1),
        ];
        // svc-spike has 6 unique names — 3x median of 2.
        for i in 0..6 {
            spans.push(span(
                10 + i as u8,
                "svc-spike",
                &format!("op-{i}"),
                50_000_000,
                1,
            ));
        }
        let r = detect_cardinality_spikes(&spans);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].kind, AnomalyKind::CardinalitySpike);
        assert!(r[0].rationale.contains("Cardinality spike"));
        assert!(r[0].rationale.contains('x'));
    }

    // ===== rationale format checks per design factual-form regex =====

    #[test]
    fn rationale_strings_factual_form_no_exclamation_or_interpretive() {
        let mut spans: Vec<SpanRecord> = (0..15)
            .map(|i| span(i as u8, "svc", "op", 50_000_000, 1))
            .collect();
        spans.push(span(0xFF, "svc", "op", 5_000_000_000, 1));
        spans.push(span(0xC0, "svc-err", "x", 30_000_000, 2));
        spans.push(span(0xC1, "svc-err", "x", 30_000_000, 2));
        spans.push(span(0xC2, "svc-err", "x", 30_000_000, 2));

        let markers = detect_anomalies(&spans);
        for m in &markers {
            assert!(
                m.rationale.starts_with(|c: char| c.is_ascii_uppercase()),
                "rationale must start uppercase: {:?}",
                m.rationale
            );
            assert!(
                !m.rationale.contains('!'),
                "rationale must be factual, not exclamatory: {:?}",
                m.rationale
            );
            for forbidden in ["Yikes", "Whoa", "Wow", "Oh no"] {
                assert!(
                    !m.rationale.contains(forbidden),
                    "rationale must not contain interpretive phrasing: {forbidden:?}"
                );
            }
        }
    }
}
