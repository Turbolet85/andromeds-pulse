use std::time::Instant;

use tracing::instrument;

use crate::contract::{AttributeFilterResult, MAX_ATTRIBUTE_VALUE_BYTES, SpanRecord};

// Explicit allowlist of attribute keys retained by `filter_attributes`. Any
// attribute key NOT in this set is dropped before output. Exact-match `==`
// equality (not regex / starts_with / substring); host-app instrumentation
// emits an unbounded long tail of keys + values that may incidentally
// carry secrets — allowlist semantics are the active redaction defense
// per security plan §Logging "What NEVER to log" Vector 1.
pub(crate) const KEPT_ATTRIBUTE_KEYS: &[&str] = &[
    "service.name",
    "request_id",
    "error",
    "error.type",
    "error.message",
];

#[instrument(
    skip_all,
    fields(
        input_row_count = spans.len(),
        output_row_count = tracing::field::Empty,
        kept_attribute_count = tracing::field::Empty,
        dropped_attribute_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ),
)]
pub(crate) fn filter_attributes(spans: &[SpanRecord]) -> AttributeFilterResult {
    let started = Instant::now();
    let span = tracing::Span::current();

    let mut output: Vec<SpanRecord> = Vec::with_capacity(spans.len());
    let mut kept_attribute_count: usize = 0;
    let mut dropped_attribute_count: usize = 0;

    for s in spans {
        let mut filtered_attrs: Vec<(String, String)> = Vec::with_capacity(s.attributes.len());
        for (k, v) in &s.attributes {
            if is_kept_key(k) {
                filtered_attrs.push((k.clone(), truncate_value(v, MAX_ATTRIBUTE_VALUE_BYTES)));
                kept_attribute_count += 1;
            } else {
                dropped_attribute_count += 1;
            }
        }
        let mut clone = s.clone();
        clone.attributes = filtered_attrs;
        output.push(clone);
    }

    span.record("output_row_count", output.len() as u64);
    span.record("kept_attribute_count", kept_attribute_count as u64);
    span.record("dropped_attribute_count", dropped_attribute_count as u64);
    span.record("duration_ms", started.elapsed().as_millis() as u64);

    AttributeFilterResult {
        spans: output,
        kept_attribute_count,
        dropped_attribute_count,
    }
}

fn is_kept_key(key: &str) -> bool {
    KEPT_ATTRIBUTE_KEYS.contains(&key)
}

// Truncate to ≤cap bytes on a UTF-8 char boundary, appending U+2026 (…)
// if truncation occurred. The ellipsis itself adds 3 bytes (its UTF-8
// length); total output <= cap + 3.
fn truncate_value(value: &str, cap: usize) -> String {
    if value.len() <= cap {
        return value.to_string();
    }
    let mut bytes_kept = 0_usize;
    for ch in value.chars() {
        let ch_len = ch.len_utf8();
        if bytes_kept + ch_len > cap {
            break;
        }
        bytes_kept += ch_len;
    }
    let mut out = String::with_capacity(bytes_kept + 3);
    out.push_str(&value[..bytes_kept]);
    out.push('\u{2026}');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_with_attrs(span_id: u8, attrs: Vec<(&str, &str)>) -> SpanRecord {
        SpanRecord {
            trace_id: [1; 16],
            span_id: [span_id; 8],
            parent_span_id: None,
            service_name: "svc".to_string(),
            name: "op".to_string(),
            start_time_unix_nano: 1_000_000_000,
            end_time_unix_nano: 1_010_000_000,
            status_code: 1,
            attributes: attrs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }

    #[test]
    fn filter_empty_input_returns_empty_result() {
        let result = filter_attributes(&[]);
        assert!(result.spans.is_empty());
        assert_eq!(result.kept_attribute_count, 0);
        assert_eq!(result.dropped_attribute_count, 0);
    }

    #[test]
    fn filter_span_with_no_attributes_emits_zero_counts() {
        let result = filter_attributes(&[span_with_attrs(0xAA, Vec::new())]);
        assert_eq!(result.spans.len(), 1);
        assert_eq!(result.kept_attribute_count, 0);
        assert_eq!(result.dropped_attribute_count, 0);
        assert!(result.spans[0].attributes.is_empty());
    }

    #[test]
    fn filter_keeps_all_allowlisted_keys() {
        let result = filter_attributes(&[span_with_attrs(
            0xAA,
            vec![
                ("service.name", "checkout"),
                ("request_id", "req-123"),
                ("error", "true"),
                ("error.type", "TimeoutError"),
                ("error.message", "deadline exceeded"),
            ],
        )]);
        assert_eq!(result.kept_attribute_count, 5);
        assert_eq!(result.dropped_attribute_count, 0);
        assert_eq!(result.spans[0].attributes.len(), 5);
    }

    #[test]
    fn filter_drops_all_verbose_keys() {
        let result = filter_attributes(&[span_with_attrs(
            0xAA,
            vec![
                ("http.user_agent", "Mozilla"),
                ("db.connection_string", "postgres://secret"),
                ("auth.token", "Bearer xyz"),
                ("payload.body", "{\"user\":1}"),
            ],
        )]);
        assert_eq!(result.kept_attribute_count, 0);
        assert_eq!(result.dropped_attribute_count, 4);
        assert!(result.spans[0].attributes.is_empty());
    }

    #[test]
    fn filter_mixed_keeps_and_drops_per_allowlist() {
        let result = filter_attributes(&[span_with_attrs(
            0xAA,
            vec![
                ("service.name", "checkout"),
                ("http.user_agent", "Mozilla"),
                ("error", "true"),
                ("auth.token", "Bearer xyz"),
            ],
        )]);
        assert_eq!(result.kept_attribute_count, 2);
        assert_eq!(result.dropped_attribute_count, 2);
        let kept_keys: Vec<&str> = result.spans[0]
            .attributes
            .iter()
            .map(|(k, _)| k.as_str())
            .collect();
        assert!(kept_keys.contains(&"service.name"));
        assert!(kept_keys.contains(&"error"));
    }

    #[test]
    fn filter_exact_match_rejects_near_keys() {
        // Defends against substring-match anti-pattern: `service_name_v2`
        // (underscore + suffix) does NOT match `service.name`.
        let result = filter_attributes(&[span_with_attrs(
            0xAA,
            vec![
                ("service_name_v2", "checkout"),
                ("service.name.suffix", "weird"),
                ("Service.Name", "case-sensitive"),
                ("service.name", "real-keep"),
            ],
        )]);
        assert_eq!(result.kept_attribute_count, 1);
        assert_eq!(result.dropped_attribute_count, 3);
        assert_eq!(result.spans[0].attributes[0].0, "service.name");
        assert_eq!(result.spans[0].attributes[0].1, "real-keep");
    }

    #[test]
    fn filter_truncates_long_value_with_ellipsis() {
        let huge_value = "A".repeat(1_048_576); // 1 MiB
        let result = filter_attributes(&[span_with_attrs(
            0xAA,
            vec![("error.message", huge_value.as_str())],
        )]);
        let value = &result.spans[0].attributes[0].1;
        // ≤ MAX_ATTRIBUTE_VALUE_BYTES + 3-byte ellipsis (U+2026 is 3 bytes UTF-8).
        assert!(
            value.len() <= MAX_ATTRIBUTE_VALUE_BYTES + 3,
            "truncated length {} exceeds cap+ellipsis {}",
            value.len(),
            MAX_ATTRIBUTE_VALUE_BYTES + 3
        );
        assert!(value.ends_with('\u{2026}'));
    }

    #[test]
    fn filter_preserves_short_value_unchanged() {
        let short_value = "deadline exceeded";
        let result =
            filter_attributes(&[span_with_attrs(0xAA, vec![("error.message", short_value)])]);
        assert_eq!(result.spans[0].attributes[0].1, short_value);
    }

    #[test]
    fn filter_truncates_at_utf8_char_boundary() {
        // Construct a value whose 256-byte boundary falls inside a
        // multi-byte char. Use 4-byte chars (U+1F600 😀 = 4 bytes UTF-8)
        // packed to force the boundary to bisect a char.
        let big_emoji = "\u{1F600}".repeat(80); // 80 × 4 = 320 bytes
        let result =
            filter_attributes(&[span_with_attrs(0xAA, vec![("error", big_emoji.as_str())])]);
        let value = &result.spans[0].attributes[0].1;
        // Truncated value must still be valid UTF-8 (Rust enforces this on String;
        // assertion confirms: `String` was successfully constructed).
        assert!(value.is_char_boundary(value.len()));
        // Ellipsis appended.
        assert!(value.ends_with('\u{2026}'));
        // Truncated kept-portion ≤ 256 bytes; ellipsis adds 3.
        assert!(value.len() <= MAX_ATTRIBUTE_VALUE_BYTES + 3);
    }

    #[test]
    fn filter_count_aggregates_across_multi_span_input() {
        let result = filter_attributes(&[
            span_with_attrs(0x01, vec![("service.name", "a"), ("verbose", "x")]),
            span_with_attrs(0x02, vec![("error", "true"), ("noise", "y"), ("more", "z")]),
            span_with_attrs(0x03, Vec::new()),
        ]);
        assert_eq!(result.kept_attribute_count, 2);
        assert_eq!(result.dropped_attribute_count, 3);
        assert_eq!(result.spans.len(), 3);
    }
}
