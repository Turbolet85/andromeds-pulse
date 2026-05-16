use std::collections::BTreeMap;
use std::time::Instant;

use tracing::instrument;

use crate::contract::{AggregationResult, ServicePercentiles, SpanRecord};

#[instrument(
    skip_all,
    fields(
        metric_input_count = spans.len(),
        metric_output_count = tracing::field::Empty,
        service_count = tracing::field::Empty,
        p50_ms = tracing::field::Empty,
        p95_ms = tracing::field::Empty,
        p99_ms = tracing::field::Empty,
        max_ms = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ),
)]
pub fn aggregate_metrics(spans: &[SpanRecord]) -> AggregationResult {
    let started = Instant::now();
    let span = tracing::Span::current();

    if spans.is_empty() {
        span.record("metric_output_count", 0_u64);
        span.record("service_count", 0_u64);
        span.record("p50_ms", 0_u64);
        span.record("p95_ms", 0_u64);
        span.record("p99_ms", 0_u64);
        span.record("max_ms", 0_u64);
        span.record("duration_ms", started.elapsed().as_millis() as u64);
        return AggregationResult::default();
    }

    let mut per_service_durations: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    let mut global_durations: Vec<u64> = Vec::with_capacity(spans.len());
    for s in spans {
        let dur_ms = span_duration_ms(s);
        global_durations.push(dur_ms);
        per_service_durations
            .entry(s.service_name.clone())
            .or_default()
            .push(dur_ms);
    }

    let per_service: BTreeMap<String, ServicePercentiles> = per_service_durations
        .into_iter()
        .map(|(svc, mut durs)| {
            durs.sort_unstable();
            (svc, percentiles(&durs))
        })
        .collect();

    global_durations.sort_unstable();
    let global = percentiles(&global_durations);

    span.record("metric_output_count", spans.len() as u64);
    span.record("service_count", per_service.len() as u64);
    span.record("p50_ms", global.p50_ms);
    span.record("p95_ms", global.p95_ms);
    span.record("p99_ms", global.p99_ms);
    span.record("max_ms", global.max_ms);
    span.record("duration_ms", started.elapsed().as_millis() as u64);

    AggregationResult {
        per_service,
        global,
    }
}

// Saturating subtraction defends against malformed end_time < start_time
// inputs (clamps to 0 ms rather than panicking on overflow).
fn span_duration_ms(s: &SpanRecord) -> u64 {
    let dur_ns = s
        .end_time_unix_nano
        .saturating_sub(s.start_time_unix_nano)
        .max(0) as u64;
    dur_ns / 1_000_000
}

// Nearest-rank percentile over a pre-sorted ascending slice. p50/p95/p99
// indices clamp at n-1 so single-element / two-element inputs produce
// the locked p50 ≤ p95 ≤ p99 ≤ max invariant.
fn percentiles(sorted: &[u64]) -> ServicePercentiles {
    if sorted.is_empty() {
        return ServicePercentiles::default();
    }
    let n = sorted.len();
    let pick = |q: f64| -> u64 {
        let idx = ((q * n as f64).floor() as usize).min(n - 1);
        sorted[idx]
    };
    ServicePercentiles {
        p50_ms: pick(0.50),
        p95_ms: pick(0.95),
        p99_ms: pick(0.99),
        max_ms: sorted[n - 1],
        sample_count: n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn span_with(span_id: u8, service: &str, duration_ms: u64) -> SpanRecord {
        SpanRecord {
            trace_id: [1; 16],
            span_id: [span_id; 8],
            parent_span_id: None,
            service_name: service.to_string(),
            name: "op".to_string(),
            start_time_unix_nano: 1_000_000_000,
            end_time_unix_nano: 1_000_000_000 + (duration_ms as i64) * 1_000_000,
            status_code: 1,
            attributes: Vec::new(),
        }
    }

    #[test]
    fn aggregate_empty_input_returns_zero_values() {
        let result = aggregate_metrics(&[]);
        assert_eq!(result.global.p50_ms, 0);
        assert_eq!(result.global.p95_ms, 0);
        assert_eq!(result.global.p99_ms, 0);
        assert_eq!(result.global.max_ms, 0);
        assert_eq!(result.global.sample_count, 0);
        assert!(result.per_service.is_empty());
    }

    #[test]
    fn aggregate_single_span_returns_uniform_percentiles() {
        let result = aggregate_metrics(&[span_with(0xAA, "svc", 42)]);
        assert_eq!(result.global.p50_ms, 42);
        assert_eq!(result.global.p95_ms, 42);
        assert_eq!(result.global.p99_ms, 42);
        assert_eq!(result.global.max_ms, 42);
        assert_eq!(result.global.sample_count, 1);
        assert_eq!(result.per_service.len(), 1);
        let svc = result.per_service.get("svc").expect("svc bucket");
        assert_eq!(svc.p50_ms, 42);
        assert_eq!(svc.max_ms, 42);
    }

    #[test]
    fn aggregate_two_spans_p50_takes_higher_via_nearest_rank() {
        let result = aggregate_metrics(&[span_with(0xAA, "svc", 10), span_with(0xBB, "svc", 100)]);
        // Nearest-rank: floor(0.5 * 2) = 1, sorted[1] = 100.
        assert_eq!(result.global.p50_ms, 100);
        assert_eq!(result.global.p95_ms, 100);
        assert_eq!(result.global.p99_ms, 100);
        assert_eq!(result.global.max_ms, 100);
        assert_eq!(result.global.sample_count, 2);
    }

    #[test]
    fn aggregate_uniform_distribution_all_percentiles_equal() {
        let spans: Vec<SpanRecord> = (0..10).map(|i| span_with(i as u8, "svc", 50)).collect();
        let result = aggregate_metrics(&spans);
        assert_eq!(result.global.p50_ms, 50);
        assert_eq!(result.global.p95_ms, 50);
        assert_eq!(result.global.p99_ms, 50);
        assert_eq!(result.global.max_ms, 50);
        assert_eq!(result.global.sample_count, 10);
    }

    #[test]
    fn aggregate_skewed_distribution_p99_isolates_outlier() {
        let mut spans: Vec<SpanRecord> = (0..99).map(|i| span_with(i as u8, "svc", 10)).collect();
        spans.push(span_with(0xFE, "svc", 10_000));
        let result = aggregate_metrics(&spans);
        // 100 elements: floor(0.5 * 100) = 50 → sorted[50] = 10
        // floor(0.99 * 100) = 99 → sorted[99] = 10000 (outlier)
        assert_eq!(result.global.p50_ms, 10);
        assert_eq!(result.global.p95_ms, 10);
        assert_eq!(result.global.p99_ms, 10_000);
        assert_eq!(result.global.max_ms, 10_000);
    }

    #[test]
    fn aggregate_100_element_known_indices() {
        // Durations 1..=100 ms; sorted index k → value k+1 (1-indexed dur).
        let spans: Vec<SpanRecord> = (1..=100u64).map(|d| span_with(d as u8, "svc", d)).collect();
        let result = aggregate_metrics(&spans);
        // floor(0.5 * 100) = 50 → sorted[50] = 51
        // floor(0.95 * 100) = 95 → sorted[95] = 96
        // floor(0.99 * 100) = 99 → sorted[99] = 100
        assert_eq!(result.global.p50_ms, 51);
        assert_eq!(result.global.p95_ms, 96);
        assert_eq!(result.global.p99_ms, 100);
        assert_eq!(result.global.max_ms, 100);
        assert_eq!(result.global.sample_count, 100);
    }

    #[test]
    fn aggregate_per_service_breakdown_isolates_buckets() {
        let result = aggregate_metrics(&[
            span_with(0x01, "svc-a", 5),
            span_with(0x02, "svc-a", 15),
            span_with(0x03, "svc-b", 200),
            span_with(0x04, "svc-b", 800),
        ]);
        assert_eq!(result.per_service.len(), 2);
        let a = result.per_service.get("svc-a").expect("svc-a");
        assert_eq!(a.max_ms, 15);
        assert_eq!(a.sample_count, 2);
        let b = result.per_service.get("svc-b").expect("svc-b");
        assert_eq!(b.max_ms, 800);
        assert_eq!(b.sample_count, 2);
        assert_eq!(result.global.sample_count, 4);
        assert_eq!(result.global.max_ms, 800);
    }

    #[test]
    fn aggregate_per_service_iteration_is_alphabetical() {
        let result = aggregate_metrics(&[
            span_with(0x01, "zoo", 1),
            span_with(0x02, "alpha", 2),
            span_with(0x03, "mid", 3),
        ]);
        let names: Vec<&String> = result.per_service.keys().collect();
        assert_eq!(
            names,
            vec![&"alpha".to_string(), &"mid".to_string(), &"zoo".to_string()]
        );
    }

    #[test]
    fn aggregate_negative_duration_clamps_to_zero() {
        let mut s = span_with(0xCC, "svc", 0);
        s.start_time_unix_nano = 2_000_000_000;
        s.end_time_unix_nano = 1_000_000_000;
        let result = aggregate_metrics(&[s]);
        assert_eq!(result.global.max_ms, 0);
    }

    proptest! {
        // Percentile monotonicity holds over arbitrary valid distributions:
        // p50 ≤ p95 ≤ p99 ≤ max. Nearest-rank indexing into sorted array
        // guarantees this by construction; property test confirms across
        // ≥1000 random non-empty inputs (proptest default cases).
        #[test]
        fn aggregate_percentile_monotonicity_invariant(
            durations in prop::collection::vec(0u64..10_000_000, 1..200)
        ) {
            let spans: Vec<SpanRecord> = durations
                .iter()
                .enumerate()
                .map(|(i, &d)| span_with(i as u8, "svc", d))
                .collect();
            let result = aggregate_metrics(&spans);
            prop_assert!(result.global.p50_ms <= result.global.p95_ms);
            prop_assert!(result.global.p95_ms <= result.global.p99_ms);
            prop_assert!(result.global.p99_ms <= result.global.max_ms);
            prop_assert_eq!(result.global.sample_count, spans.len());
        }
    }
}
