use std::collections::HashMap;
use std::time::Instant;

use tracing::instrument;

use crate::contract::SpanRecord;

// Duration bucket granularity for "logically identical" collapse: spans with
// the same (service_name, name) whose durations fall in the same 100ms bucket
// are treated as duplicates. Coarser than literal span_id matching but
// preserves variation across order-of-magnitude latency differences.
const DURATION_BUCKET_NS: i64 = 100_000_000;

pub struct DedupResult {
    pub unique_spans: Vec<SpanRecord>,
    pub dedup_count: usize,
    pub input_row_count: usize,
    pub output_row_count: usize,
}

#[instrument(
    skip_all,
    fields(
        input_row_count = spans.len(),
        output_row_count = tracing::field::Empty,
        dedup_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ),
)]
pub fn dedupe_spans(spans: &[SpanRecord]) -> DedupResult {
    let started = Instant::now();
    let input_row_count = spans.len();

    let mut seen: HashMap<(String, String, i64), SpanRecord> = HashMap::with_capacity(spans.len());

    for span in spans {
        let duration_ns = span
            .end_time_unix_nano
            .saturating_sub(span.start_time_unix_nano);
        let bucket = duration_ns / DURATION_BUCKET_NS;
        let key = (span.service_name.clone(), span.name.clone(), bucket);
        seen.entry(key).or_insert_with(|| span.clone());
    }

    let mut unique_spans: Vec<SpanRecord> = seen.into_values().collect();
    unique_spans.sort_by(|a, b| {
        a.service_name
            .cmp(&b.service_name)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.span_id.cmp(&b.span_id))
    });

    let output_row_count = unique_spans.len();
    let dedup_count = input_row_count.saturating_sub(output_row_count);

    let span = tracing::Span::current();
    span.record("output_row_count", output_row_count as u64);
    span.record("dedup_count", dedup_count as u64);
    span.record("duration_ms", started.elapsed().as_millis() as u64);

    DedupResult {
        unique_spans,
        dedup_count,
        input_row_count,
        output_row_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(span_id_byte: u8, service: &str, name: &str, duration_ns: i64) -> SpanRecord {
        SpanRecord {
            trace_id: [1; 16],
            span_id: [span_id_byte; 8],
            parent_span_id: None,
            service_name: service.to_string(),
            name: name.to_string(),
            start_time_unix_nano: 1_000_000_000,
            end_time_unix_nano: 1_000_000_000 + duration_ns,
            status_code: 1,
            attributes: Vec::new(),
        }
    }

    #[test]
    fn empty_input_returns_empty_result() {
        let r = dedupe_spans(&[]);
        assert!(r.unique_spans.is_empty());
        assert_eq!(r.dedup_count, 0);
        assert_eq!(r.input_row_count, 0);
        assert_eq!(r.output_row_count, 0);
    }

    #[test]
    fn single_span_passes_through_unchanged() {
        let s = span(0xAA, "svc", "op", 50_000_000);
        let r = dedupe_spans(std::slice::from_ref(&s));
        assert_eq!(r.unique_spans.len(), 1);
        assert_eq!(r.dedup_count, 0);
        assert_eq!(r.input_row_count, 1);
        assert_eq!(r.output_row_count, 1);
    }

    #[test]
    fn two_identical_spans_collapse_to_one_unique_one_dedup() {
        let a = span(0xAA, "svc", "op", 50_000_000);
        let b = span(0xBB, "svc", "op", 50_000_000);
        let r = dedupe_spans(&[a, b]);
        assert_eq!(r.unique_spans.len(), 1);
        assert_eq!(r.dedup_count, 1);
        assert_eq!(r.input_row_count, 2);
        assert_eq!(r.output_row_count, 1);
    }

    #[test]
    fn spans_differing_by_service_name_do_not_collapse() {
        let a = span(0xAA, "svc-1", "op", 50_000_000);
        let b = span(0xBB, "svc-2", "op", 50_000_000);
        let r = dedupe_spans(&[a, b]);
        assert_eq!(r.unique_spans.len(), 2);
        assert_eq!(r.dedup_count, 0);
    }

    #[test]
    fn spans_differing_by_name_do_not_collapse() {
        let a = span(0xAA, "svc", "op-a", 50_000_000);
        let b = span(0xBB, "svc", "op-b", 50_000_000);
        let r = dedupe_spans(&[a, b]);
        assert_eq!(r.unique_spans.len(), 2);
    }

    #[test]
    fn spans_differing_by_duration_bucket_do_not_collapse() {
        let a = span(0xAA, "svc", "op", 50_000_000);
        let b = span(0xBB, "svc", "op", 250_000_000);
        let r = dedupe_spans(&[a, b]);
        assert_eq!(r.unique_spans.len(), 2);
    }

    #[test]
    fn spans_within_same_duration_bucket_collapse() {
        let a = span(0xAA, "svc", "op", 50_000_000);
        let b = span(0xBB, "svc", "op", 90_000_000);
        let r = dedupe_spans(&[a, b]);
        assert_eq!(r.unique_spans.len(), 1);
        assert_eq!(r.dedup_count, 1);
    }

    #[test]
    fn count_invariant_holds_input_equals_unique_plus_dedup() {
        let inputs = vec![
            span(1, "svc", "op", 50_000_000),
            span(2, "svc", "op", 50_000_000),
            span(3, "svc", "op", 50_000_000),
            span(4, "svc", "op-other", 50_000_000),
        ];
        let r = dedupe_spans(&inputs);
        assert_eq!(r.input_row_count, r.output_row_count + r.dedup_count);
    }

    #[test]
    fn output_ordering_is_deterministic_across_invocations() {
        let inputs = vec![
            span(3, "svc-c", "op", 10_000_000),
            span(1, "svc-a", "op", 10_000_000),
            span(2, "svc-b", "op", 10_000_000),
        ];
        let r1 = dedupe_spans(&inputs);
        let r2 = dedupe_spans(&inputs);
        let ids1: Vec<[u8; 8]> = r1.unique_spans.iter().map(|s| s.span_id).collect();
        let ids2: Vec<[u8; 8]> = r2.unique_spans.iter().map(|s| s.span_id).collect();
        assert_eq!(ids1, ids2);
        assert_eq!(
            r1.unique_spans
                .iter()
                .map(|s| s.service_name.as_str())
                .collect::<Vec<_>>(),
            vec!["svc-a", "svc-b", "svc-c"],
        );
    }
}
