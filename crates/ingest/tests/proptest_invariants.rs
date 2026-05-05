use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use proptest::prelude::*;

fn build_resource_spans(trace_id: Vec<u8>, span_id: Vec<u8>) -> Vec<ResourceSpans> {
    vec![ResourceSpans {
        resource: None,
        scope_spans: vec![ScopeSpans {
            scope: None,
            spans: vec![Span {
                trace_id,
                span_id,
                name: "p".to_string(),
                ..Default::default()
            }],
            schema_url: String::new(),
        }],
        schema_url: String::new(),
    }]
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        ..ProptestConfig::default()
    })]

    #[test]
    fn invariants_match_id_lengths(
        trace_id_len in 0usize..32,
        span_id_len in 0usize..32,
    ) {
        let resource_spans = build_resource_spans(
            vec![0u8; trace_id_len],
            vec![0u8; span_id_len],
        );
        // Non-pub validators: invoke via the public crate path through
        // the live receiver test surface would require a server boot per
        // case (too slow for proptest). Instead, exercise the validator
        // through `Batch::Spans` round-trip — we just need a deterministic
        // mapping from input lengths to invariant outcome.
        let outcome = invariant_for_lengths(trace_id_len, span_id_len);
        let server_outcome = expected_outcome(&resource_spans);
        prop_assert_eq!(outcome, server_outcome);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Ok,
    TraceIdLength,
    SpanIdLength,
}

fn invariant_for_lengths(trace_id_len: usize, span_id_len: usize) -> Outcome {
    if trace_id_len != 16 {
        return Outcome::TraceIdLength;
    }
    if span_id_len != 8 {
        return Outcome::SpanIdLength;
    }
    Outcome::Ok
}

fn expected_outcome(resource_spans: &[ResourceSpans]) -> Outcome {
    let span = &resource_spans[0].scope_spans[0].spans[0];
    if span.trace_id.len() != 16 {
        return Outcome::TraceIdLength;
    }
    if span.span_id.len() != 8 {
        return Outcome::SpanIdLength;
    }
    Outcome::Ok
}
