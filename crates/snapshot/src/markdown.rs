use std::collections::HashSet;
use std::fmt::Write as _;
use std::time::Instant;

use tracing::instrument;

use crate::contract::{
    AnomalyKind, AnomalyMarker, CriticalPathStep, CurationOutput, FormatError,
    MAX_ATTRIBUTE_VALUE_BYTES, MarkdownReport, ServicePercentiles, SpanRecord, TokenBudget,
    TruncationState,
};

#[derive(Default, Clone, Copy)]
struct AttributeCounts {
    dropped_span_count: usize,
    dropped_attribute_count: usize,
}

/// Render `CurationOutput` as a hierarchical observation-log markdown sized
/// to `budget`. Smart truncation drops verbose attribute detail before low-
/// priority spans before failing closed; anomaly evidence + critical-path
/// always survive (Phase A). Per design-system §Brand Identity Signature
/// element + arch §Established Decisions [Snapshot Curation Default].
#[instrument(
    target = "snapshot.render.markdown",
    skip_all,
    fields(
        markdown_size_bytes = tracing::field::Empty,
        section_count = tracing::field::Empty,
        token_count_actual = tracing::field::Empty,
        budget_token_limit = tracing::field::Empty,
        anomaly_marker_count = tracing::field::Empty,
        critical_path_span_count = tracing::field::Empty,
        truncated_span_count = tracing::field::Empty,
        truncated_attribute_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ),
)]
pub fn format_markdown(
    curated: &CurationOutput,
    budget: TokenBudget,
) -> Result<MarkdownReport, FormatError> {
    let started = Instant::now();
    let span = tracing::Span::current();
    let budget_count = budget.as_token_count();

    let mut buf = String::new();
    render_h1_title(&mut buf);
    render_section_anomalies(&mut buf, &curated.anomaly_markers, &curated.unique_spans);
    render_section_critical_path(&mut buf, &curated.critical_path);
    let phase_a_tokens = count_tokens(&buf);

    if phase_a_tokens > budget_count {
        return finish_phase_d(started, &span, budget_count, phase_a_tokens, curated);
    }

    render_section_aggregation(&mut buf, curated);

    let high_priority_ids = collect_high_priority_span_ids(curated);
    let (high_spans, low_spans) =
        partition_spans_by_priority(&curated.unique_spans, &high_priority_ids);

    let attempted_full = clone_with_section_unique_spans(&buf, &high_spans, &low_spans, true);
    if count_tokens(&attempted_full) <= budget_count {
        return finish_ok(
            started,
            &span,
            budget,
            budget_count,
            attempted_full,
            curated,
            AttributeCounts::default(),
        );
    }

    let total_low_attrs = total_attribute_count(&low_spans);
    let attempted_attrs_dropped =
        clone_with_section_unique_spans(&buf, &high_spans, &low_spans, false);
    if count_tokens(&attempted_attrs_dropped) <= budget_count {
        let mut out = attempted_attrs_dropped;
        render_truncation_marker_attributes(&mut out, budget, total_low_attrs);
        return finish_ok(
            started,
            &span,
            budget,
            budget_count,
            out,
            curated,
            AttributeCounts {
                dropped_span_count: 0,
                dropped_attribute_count: total_low_attrs,
            },
        );
    }

    let attempted_high_only = clone_with_section_unique_spans(&buf, &high_spans, &[], true);
    if count_tokens(&attempted_high_only) <= budget_count {
        let dropped_span_count = low_spans.len();
        let mut out = attempted_high_only;
        render_truncation_marker_spans(&mut out, budget, dropped_span_count);
        return finish_ok(
            started,
            &span,
            budget,
            budget_count,
            out,
            curated,
            AttributeCounts {
                dropped_span_count,
                dropped_attribute_count: total_low_attrs,
            },
        );
    }

    let dropped_span_count = curated.unique_spans.len();
    let mut out = buf;
    render_truncation_marker_spans(&mut out, budget, dropped_span_count);
    finish_ok(
        started,
        &span,
        budget,
        budget_count,
        out,
        curated,
        AttributeCounts {
            dropped_span_count,
            dropped_attribute_count: total_low_attrs,
        },
    )
}

fn finish_ok(
    started: Instant,
    span: &tracing::Span,
    budget: TokenBudget,
    budget_count: usize,
    markdown: String,
    curated: &CurationOutput,
    counts: AttributeCounts,
) -> Result<MarkdownReport, FormatError> {
    let truncation_state = if counts.dropped_span_count == 0 && counts.dropped_attribute_count == 0
    {
        TruncationState::None
    } else {
        TruncationState::Applied {
            dropped_span_count: counts.dropped_span_count,
            dropped_attribute_count: counts.dropped_attribute_count,
        }
    };

    let token_count = count_tokens(&markdown);
    let duration_ms = started.elapsed().as_millis() as u64;
    let section_count = section_h2_count(&markdown);

    span.record("markdown_size_bytes", markdown.len() as u64);
    span.record("section_count", section_count as u64);
    span.record("token_count_actual", token_count as u64);
    span.record("budget_token_limit", budget_count as u64);
    span.record("anomaly_marker_count", curated.anomaly_markers.len() as u64);
    span.record(
        "critical_path_span_count",
        curated.critical_path.len() as u64,
    );
    span.record("truncated_span_count", counts.dropped_span_count as u64);
    span.record(
        "truncated_attribute_count",
        counts.dropped_attribute_count as u64,
    );
    span.record("duration_ms", duration_ms);

    Ok(MarkdownReport {
        markdown,
        token_count,
        budget,
        truncation_state,
    })
}

fn finish_phase_d(
    started: Instant,
    span: &tracing::Span,
    budget_count: usize,
    actual_tokens: usize,
    curated: &CurationOutput,
) -> Result<MarkdownReport, FormatError> {
    let duration_ms = started.elapsed().as_millis() as u64;

    span.record("markdown_size_bytes", 0_u64);
    span.record("section_count", 0_u64);
    span.record("token_count_actual", actual_tokens as u64);
    span.record("budget_token_limit", budget_count as u64);
    span.record("anomaly_marker_count", curated.anomaly_markers.len() as u64);
    span.record(
        "critical_path_span_count",
        curated.critical_path.len() as u64,
    );
    span.record("truncated_span_count", 0_u64);
    span.record("truncated_attribute_count", 0_u64);
    span.record("duration_ms", duration_ms);

    tracing::error!(
        target: "snapshot.token.count.validate",
        token_count_actual = actual_tokens,
        token_budget_limit = budget_count,
        budget_exceeded = true,
        duration_ms,
        "snapshot token budget exceeded by anomaly + critical-path content",
    );

    Err(FormatError::BudgetExceeded {
        budget_tokens: budget_count,
        actual_tokens,
    })
}

// Heuristic: 1 token per ~4 bytes of UTF-8 input. Approximates GPT-style
// tokenization without pulling a tokenizer crate (chunk #40 zero-new-deps
// precedent). Future chunk MAY swap to a real tokenizer; the proptest
// invariant `report.token_count <= budget.as_token_count()` is independent
// of algorithm correctness.
fn count_tokens(s: &str) -> usize {
    s.len().div_ceil(4)
}

// Backslash-escape markdown structural chars that an OTLP-derived attribute
// value could otherwise use to break document structure (link injection,
// fake heading injection). Per security plan §Anti-patterns "Input"
// `format!`-style ban — same lesson, different output channel.
fn escape_attribute_value(s: &str) -> String {
    let truncated = truncate_to_byte_cap(s, MAX_ATTRIBUTE_VALUE_BYTES);
    let mut out = String::with_capacity(truncated.len() + 8);
    for ch in truncated.chars() {
        match ch {
            '[' | ']' | '(' | ')' | '`' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            '\n' | '\r' => out.push(' '),
            other => out.push(other),
        }
    }
    out
}

fn truncate_to_byte_cap(s: &str, cap: usize) -> String {
    if s.len() <= cap {
        return s.to_string();
    }
    let mut bytes_kept = 0_usize;
    for ch in s.chars() {
        let ch_len = ch.len_utf8();
        if bytes_kept + ch_len > cap {
            break;
        }
        bytes_kept += ch_len;
    }
    let mut out = String::with_capacity(bytes_kept + 3);
    out.push_str(&s[..bytes_kept]);
    out.push('\u{2026}');
    out
}

fn span_id_hex(span_id: &[u8; 8]) -> String {
    let mut out = String::with_capacity(16);
    for byte in span_id {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn trace_id_hex(trace_id: &[u8; 16]) -> String {
    let mut out = String::with_capacity(32);
    for byte in trace_id {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn render_h1_title(buf: &mut String) {
    buf.push_str("# andromeda-pulse — Observation log\n\n");
}

fn render_section_anomalies(buf: &mut String, markers: &[AnomalyMarker], spans: &[SpanRecord]) {
    buf.push_str("## Anomaly observations\n\n");
    if markers.is_empty() {
        buf.push_str("_(no anomalies detected in this observation window)_\n\n");
        return;
    }

    let order = [
        AnomalyKind::LatencyOutlier,
        AnomalyKind::ErrorCorrelation,
        AnomalyKind::CardinalitySpike,
    ];
    for kind in order {
        for marker in markers.iter().filter(|m| m.kind == kind) {
            render_anomaly_subsection(buf, marker, spans);
        }
    }
}

fn render_anomaly_subsection(buf: &mut String, marker: &AnomalyMarker, spans: &[SpanRecord]) {
    let kind_label = match marker.kind {
        AnomalyKind::LatencyOutlier => "Latency outlier",
        AnomalyKind::ErrorCorrelation => "Error correlation",
        AnomalyKind::CardinalitySpike => "Cardinality spike",
    };
    let _ = writeln!(
        buf,
        "### {kind_label} (severity {sev})",
        sev = marker.severity
    );
    let _ = writeln!(buf, "{rationale}", rationale = marker.rationale);
    if !marker.affected_span_ids.is_empty() {
        buf.push_str("Affected spans: ");
        for (idx, id) in marker.affected_span_ids.iter().enumerate() {
            if idx > 0 {
                buf.push_str(", ");
            }
            let _ = write!(buf, "`{hex}`", hex = span_id_hex(id));
        }
        buf.push('\n');
    }
    let services: HashSet<&str> = spans
        .iter()
        .filter(|s| marker.affected_span_ids.contains(&s.span_id))
        .map(|s| s.service_name.as_str())
        .collect();
    if !services.is_empty() {
        let mut svc_list: Vec<&str> = services.into_iter().collect();
        svc_list.sort();
        buf.push_str("Services: ");
        for (idx, svc) in svc_list.iter().enumerate() {
            if idx > 0 {
                buf.push_str(", ");
            }
            let _ = write!(buf, "`{svc}`");
        }
        buf.push('\n');
    }
    buf.push('\n');
}

fn render_section_critical_path(buf: &mut String, path: &[CriticalPathStep]) {
    buf.push_str("## Critical path\n\n");
    if path.is_empty() {
        buf.push_str("_(no critical path extracted)_\n\n");
        return;
    }
    for (idx, step) in path.iter().enumerate() {
        let _ = writeln!(
            buf,
            "{n}. `{hex}` — {dur} ms",
            n = idx + 1,
            hex = span_id_hex(&step.span_id),
            dur = step.duration_ms
        );
    }
    buf.push('\n');
}

fn render_section_aggregation(buf: &mut String, curated: &CurationOutput) {
    buf.push_str("## Service constellation summary\n\n");
    if curated.aggregation.per_service.is_empty() {
        buf.push_str("_(no per-service aggregation available)_\n\n");
        return;
    }

    let alert_services = classify_alert_services(curated);
    let mut alert: Vec<(&String, &ServicePercentiles)> = Vec::new();
    let mut nominal: Vec<(&String, &ServicePercentiles)> = Vec::new();
    for (name, percentiles) in &curated.aggregation.per_service {
        if alert_services.contains(name) {
            alert.push((name, percentiles));
        } else {
            nominal.push((name, percentiles));
        }
    }

    for (name, p) in alert {
        render_service_block(buf, name, p, "alert");
    }
    for (name, p) in nominal {
        render_service_block(buf, name, p, "nominal");
    }
    let global = &curated.aggregation.global;
    let _ = writeln!(
        buf,
        "Global: p50_ms=`{p50}`, p95_ms=`{p95}`, p99_ms=`{p99}`, max_ms=`{max}` (n={n})",
        p50 = global.p50_ms,
        p95 = global.p95_ms,
        p99 = global.p99_ms,
        max = global.max_ms,
        n = global.sample_count
    );
    buf.push('\n');
}

fn render_service_block(
    buf: &mut String,
    name: &str,
    percentiles: &ServicePercentiles,
    band: &str,
) {
    let _ = writeln!(buf, "### `{name}` ({band})");
    let _ = writeln!(buf, "- p50: `{p50}` ms", p50 = percentiles.p50_ms);
    let _ = writeln!(buf, "- p95: `{p95}` ms", p95 = percentiles.p95_ms);
    let _ = writeln!(buf, "- p99: `{p99}` ms", p99 = percentiles.p99_ms);
    let _ = writeln!(buf, "- max: `{max}` ms", max = percentiles.max_ms);
    let _ = writeln!(buf, "- sample count: {n}", n = percentiles.sample_count);
    buf.push('\n');
}

fn render_section_unique_spans(
    buf: &mut String,
    high_spans: &[SpanRecord],
    low_spans: &[SpanRecord],
    keep_low_attributes: bool,
) {
    if high_spans.is_empty() && low_spans.is_empty() {
        return;
    }
    buf.push_str("## Span detail\n\n");
    for s in high_spans {
        render_span_block(buf, s, true);
    }
    for s in low_spans {
        render_span_block(buf, s, keep_low_attributes);
    }
}

fn render_span_block(buf: &mut String, s: &SpanRecord, render_attributes: bool) {
    let _ = writeln!(
        buf,
        "### Span `{hex}` (`{svc}`/`{name}`)",
        hex = span_id_hex(&s.span_id),
        svc = s.service_name,
        name = escape_attribute_value(&s.name)
    );
    let _ = writeln!(buf, "- trace: `{tr}`", tr = trace_id_hex(&s.trace_id));
    let duration_ns = s.end_time_unix_nano.saturating_sub(s.start_time_unix_nano);
    let duration_ms = (duration_ns / 1_000_000).max(0);
    let _ = writeln!(buf, "- duration: `{ms}` ms", ms = duration_ms);
    let status_label = match s.status_code {
        0 => "unset",
        1 => "ok",
        2 => "error",
        _ => "unknown",
    };
    let _ = writeln!(
        buf,
        "- status: {label} ({code})",
        label = status_label,
        code = s.status_code
    );
    if render_attributes && !s.attributes.is_empty() {
        buf.push_str("- attributes:\n");
        for (k, v) in &s.attributes {
            let _ = writeln!(
                buf,
                "  - `{k}`: {v}",
                k = escape_attribute_value(k),
                v = escape_attribute_value(v)
            );
        }
    }
    buf.push('\n');
}

fn clone_with_section_unique_spans(
    base: &str,
    high_spans: &[SpanRecord],
    low_spans: &[SpanRecord],
    keep_low_attributes: bool,
) -> String {
    let mut buf = String::with_capacity(base.len() + 1024);
    buf.push_str(base);
    render_section_unique_spans(&mut buf, high_spans, low_spans, keep_low_attributes);
    buf
}

fn render_truncation_marker_attributes(buf: &mut String, budget: TokenBudget, dropped: usize) {
    if dropped == 0 {
        return;
    }
    let _ = writeln!(
        buf,
        "_({dropped} attribute(s) elided to honor {label} token budget; anomaly-priority preserved)_",
        dropped = dropped,
        label = budget.label()
    );
    buf.push('\n');
}

fn render_truncation_marker_spans(buf: &mut String, budget: TokenBudget, dropped: usize) {
    if dropped == 0 {
        return;
    }
    let _ = writeln!(
        buf,
        "_({dropped} additional span(s) elided to honor {label} token budget; anomaly-priority preserved)_",
        dropped = dropped,
        label = budget.label()
    );
    buf.push('\n');
}

fn classify_alert_services(curated: &CurationOutput) -> HashSet<String> {
    let mut alert: HashSet<String> = HashSet::new();
    for s in &curated.unique_spans {
        if s.status_code == 2 {
            alert.insert(s.service_name.clone());
        }
    }
    let anomaly_affected: HashSet<&[u8; 8]> = curated
        .anomaly_markers
        .iter()
        .flat_map(|m| m.affected_span_ids.iter())
        .collect();
    for s in &curated.unique_spans {
        if anomaly_affected.contains(&s.span_id) {
            alert.insert(s.service_name.clone());
        }
    }
    alert
}

fn collect_high_priority_span_ids(curated: &CurationOutput) -> HashSet<[u8; 8]> {
    let mut ids: HashSet<[u8; 8]> = HashSet::new();
    for marker in &curated.anomaly_markers {
        for id in &marker.affected_span_ids {
            ids.insert(*id);
        }
    }
    for step in &curated.critical_path {
        ids.insert(step.span_id);
    }
    ids
}

fn partition_spans_by_priority(
    spans: &[SpanRecord],
    high_priority_ids: &HashSet<[u8; 8]>,
) -> (Vec<SpanRecord>, Vec<SpanRecord>) {
    let mut high: Vec<SpanRecord> = Vec::new();
    let mut low: Vec<SpanRecord> = Vec::new();
    for s in spans {
        if high_priority_ids.contains(&s.span_id) {
            high.push(s.clone());
        } else {
            low.push(s.clone());
        }
    }
    (high, low)
}

fn total_attribute_count(spans: &[SpanRecord]) -> usize {
    spans.iter().map(|s| s.attributes.len()).sum()
}

fn section_h2_count(buf: &str) -> usize {
    buf.lines().filter(|l| l.starts_with("## ")).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::AggregationResult;
    use proptest::prelude::*;
    use std::collections::BTreeMap;

    fn span_for_test(span_id_byte: u8, status_code: u8) -> SpanRecord {
        SpanRecord {
            trace_id: [1; 16],
            span_id: [span_id_byte; 8],
            parent_span_id: None,
            service_name: "checkout".to_string(),
            name: "POST /pay".to_string(),
            start_time_unix_nano: 1_000_000_000,
            end_time_unix_nano: 1_010_000_000,
            status_code,
            attributes: Vec::new(),
        }
    }

    fn anomaly_heavy_curation_output() -> CurationOutput {
        let spans: Vec<SpanRecord> = (0..10)
            .map(|i| span_for_test(i, if i < 3 { 2 } else { 1 }))
            .collect();
        let critical_path: Vec<CriticalPathStep> = (0..4)
            .map(|i| CriticalPathStep {
                span_id: [i as u8; 8],
                duration_ms: 100 - (i as u64) * 10,
            })
            .collect();
        let anomaly_markers = vec![
            AnomalyMarker {
                kind: AnomalyKind::LatencyOutlier,
                severity: 75,
                rationale: "p99 exceeded baseline by 3.2 sigma".to_string(),
                affected_span_ids: vec![[0; 8], [1; 8]],
            },
            AnomalyMarker {
                kind: AnomalyKind::LatencyOutlier,
                severity: 60,
                rationale: "p95 doubled in last window".to_string(),
                affected_span_ids: vec![[2; 8]],
            },
            AnomalyMarker {
                kind: AnomalyKind::ErrorCorrelation,
                severity: 80,
                rationale: "checkout error rate jumped 4x".to_string(),
                affected_span_ids: vec![[0; 8], [3; 8]],
            },
        ];
        let mut per_service: BTreeMap<String, ServicePercentiles> = BTreeMap::new();
        per_service.insert(
            "checkout".to_string(),
            ServicePercentiles {
                p50_ms: 50,
                p95_ms: 100,
                p99_ms: 200,
                max_ms: 500,
                sample_count: 10,
            },
        );
        CurationOutput {
            unique_spans: spans,
            dedup_count: 2,
            anomaly_markers,
            critical_path,
            input_row_count: 12,
            output_row_count: 10,
            aggregation: AggregationResult {
                per_service,
                global: ServicePercentiles {
                    p50_ms: 50,
                    p95_ms: 100,
                    p99_ms: 200,
                    max_ms: 500,
                    sample_count: 10,
                },
            },
            kept_attribute_count: 2,
            dropped_attribute_count: 5,
        }
    }

    fn many_span_curation(span_count: usize) -> CurationOutput {
        let spans: Vec<SpanRecord> = (0..span_count)
            .map(|i| {
                let mut s = span_for_test((i % 256) as u8, if i % 50 == 0 { 2 } else { 1 });
                s.service_name = format!("svc_{}", i % 5);
                s.attributes = vec![
                    ("service.name".to_string(), format!("svc_value_{}", i % 5)),
                    ("error".to_string(), "false".to_string()),
                ];
                s
            })
            .collect();
        let critical_path: Vec<CriticalPathStep> = (0..3)
            .map(|i| CriticalPathStep {
                span_id: [i as u8; 8],
                duration_ms: 90 - (i as u64) * 5,
            })
            .collect();
        let anomaly_markers = vec![AnomalyMarker {
            kind: AnomalyKind::LatencyOutlier,
            severity: 70,
            rationale: "tail latency drift".to_string(),
            affected_span_ids: vec![[0; 8], [1; 8]],
        }];
        let mut per_service: BTreeMap<String, ServicePercentiles> = BTreeMap::new();
        for i in 0..5 {
            per_service.insert(
                format!("svc_{i}"),
                ServicePercentiles {
                    p50_ms: 10,
                    p95_ms: 20,
                    p99_ms: 30,
                    max_ms: 50,
                    sample_count: span_count / 5,
                },
            );
        }
        CurationOutput {
            unique_spans: spans,
            dedup_count: 0,
            anomaly_markers,
            critical_path,
            input_row_count: span_count,
            output_row_count: span_count,
            aggregation: AggregationResult {
                per_service,
                global: ServicePercentiles {
                    p50_ms: 10,
                    p95_ms: 20,
                    p99_ms: 30,
                    max_ms: 50,
                    sample_count: span_count,
                },
            },
            kept_attribute_count: span_count * 2,
            dropped_attribute_count: 0,
        }
    }

    #[test]
    fn format_markdown_emits_h1_h2_h3_hierarchy() {
        let out = anomaly_heavy_curation_output();
        let report = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        let h1_count = report
            .markdown
            .lines()
            .filter(|l| l.starts_with("# "))
            .count();
        assert_eq!(h1_count, 1, "exactly one H1 expected");
        let h2_count = report
            .markdown
            .lines()
            .filter(|l| l.starts_with("## "))
            .count();
        assert!(
            h2_count >= 3,
            "at least 3 H2 sections expected, got {h2_count}"
        );
        let h3_count = report
            .markdown
            .lines()
            .filter(|l| l.starts_with("### "))
            .count();
        assert!(
            h3_count >= 1,
            "at least one H3 expected for anomaly subsection"
        );
    }

    #[test]
    fn format_markdown_anomaly_section_appears_before_critical_path_before_aggregation() {
        let out = anomaly_heavy_curation_output();
        let report = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        let anom = report
            .markdown
            .find("## Anomaly observations")
            .expect("anomaly section present");
        let crit = report
            .markdown
            .find("## Critical path")
            .expect("critical path section present");
        let agg = report
            .markdown
            .find("## Service constellation summary")
            .expect("aggregation section present");
        assert!(anom < crit, "anomaly before critical-path");
        assert!(crit < agg, "critical-path before aggregation");
    }

    #[test]
    fn format_markdown_with_balanced_budget_500_spans_under_budget() {
        let out = many_span_curation(500);
        let report = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        assert!(
            report.token_count <= TokenBudget::Balanced.as_token_count(),
            "token_count {} exceeds Balanced budget",
            report.token_count
        );
    }

    #[test]
    fn format_markdown_with_conservative_budget_500_spans_truncates_attributes_or_spans() {
        let out = many_span_curation(500);
        let report = format_markdown(&out, TokenBudget::Conservative).expect("ok");
        assert!(
            report.token_count <= TokenBudget::Conservative.as_token_count(),
            "token_count {} exceeds Conservative budget",
            report.token_count
        );
        match report.truncation_state {
            TruncationState::Applied {
                dropped_span_count,
                dropped_attribute_count,
            } => {
                assert!(
                    dropped_attribute_count > 0 || dropped_span_count > 0,
                    "expected some truncation under Conservative + 500 spans"
                );
            }
            TruncationState::None => {
                panic!(
                    "expected truncation under Conservative + 500 spans, got TruncationState::None"
                );
            }
        }
    }

    #[test]
    fn format_markdown_anomaly_evidence_survives_under_budget_pressure() {
        let out = anomaly_heavy_curation_output();
        let report = format_markdown(&out, TokenBudget::Conservative).expect("ok");
        assert!(report.markdown.contains("Latency outlier"));
        assert!(report.markdown.contains("Error correlation"));
        for marker in &out.anomaly_markers {
            for id in &marker.affected_span_ids {
                let hex = span_id_hex(id);
                assert!(
                    report.markdown.contains(&hex),
                    "expected anomaly span {hex} in output"
                );
            }
        }
    }

    #[test]
    fn format_markdown_h1_h2_anchors_stable_across_budget_presets() {
        let out = anomaly_heavy_curation_output();
        let extract_h12 = |md: &str| -> Vec<String> {
            md.lines()
                .filter(|l| l.starts_with("# ") || l.starts_with("## "))
                .map(|l| l.to_string())
                .collect()
        };
        let r_c = format_markdown(&out, TokenBudget::Conservative).expect("ok");
        let r_b = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        let r_d = format_markdown(&out, TokenBudget::Detailed).expect("ok");
        assert_eq!(extract_h12(&r_c.markdown), extract_h12(&r_b.markdown));
        assert_eq!(extract_h12(&r_b.markdown), extract_h12(&r_d.markdown));
    }

    #[test]
    fn format_markdown_no_orphan_anchors() {
        let out = anomaly_heavy_curation_output();
        let report = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        for line in report.markdown.lines() {
            let mut start = 0;
            while let Some(open) = line[start..].find("](#") {
                let abs = start + open + 3;
                let close = line[abs..]
                    .find(')')
                    .expect("anchor opens but never closes");
                let anchor = &line[abs..abs + close];
                assert!(
                    !anchor.is_empty(),
                    "anchor reference must not be empty: {line}"
                );
                start = abs + close + 1;
            }
        }
    }

    #[test]
    fn format_markdown_voice_audit_no_banned_strings() {
        let out = anomaly_heavy_curation_output();
        let report = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        let banned = ["Great news", "Here's what", "\u{1F389}", "\u{2728}"];
        for needle in banned {
            assert!(
                !report.markdown.contains(needle),
                "banned voice marker `{needle}` present in output"
            );
        }
        for line in report.markdown.lines() {
            let trimmed = line.trim_start_matches('#').trim();
            assert!(trimmed != "Report", "banned heading `Report`");
            assert!(trimmed != "Export", "banned heading `Export`");
            assert!(trimmed != "Dump", "banned heading `Dump`");
        }
    }

    #[test]
    fn format_markdown_monospace_split_trace_ids_in_backticks() {
        let mut out = anomaly_heavy_curation_output();
        for s in out.unique_spans.iter_mut() {
            s.trace_id = [0xAB; 16];
        }
        let report = format_markdown(&out, TokenBudget::Detailed).expect("ok");
        let trace_hex = "abababababababababababababababab";
        let total_occurrences = report.markdown.matches(trace_hex).count();
        let backticked_occurrences = report.markdown.matches(&format!("`{trace_hex}`")).count();
        assert!(total_occurrences > 0, "trace_id hex must appear in output");
        assert_eq!(
            total_occurrences, backticked_occurrences,
            "every trace_id occurrence must be backtick-fenced"
        );
    }

    #[test]
    fn format_markdown_attribute_value_truncated_with_ellipsis() {
        let mut out = anomaly_heavy_curation_output();
        let big_value = "X".repeat(1024);
        out.unique_spans[0]
            .attributes
            .push(("error.message".to_string(), big_value));
        let report = format_markdown(&out, TokenBudget::Detailed).expect("ok");
        assert!(
            report.markdown.contains('\u{2026}'),
            "expected truncation ellipsis in output for 1KB attribute value"
        );
        assert!(
            !report
                .markdown
                .contains(&"X".repeat(MAX_ATTRIBUTE_VALUE_BYTES + 1)),
            "raw value beyond byte cap must NOT appear in output"
        );
    }

    #[test]
    fn format_markdown_attribute_value_escapes_markdown_structural_chars() {
        let mut out = anomaly_heavy_curation_output();
        let injection = "](javascript:alert(1))\n# Fake Section";
        out.unique_spans[0]
            .attributes
            .push(("error".to_string(), injection.to_string()));
        let report = format_markdown(&out, TokenBudget::Detailed).expect("ok");
        assert!(
            !report.markdown.contains("](javascript:"),
            "raw `](javascript:` injection survived escaping"
        );
        assert!(
            !report.markdown.contains("\n# Fake Section"),
            "raw newline-then-hash injection survived escaping"
        );
    }

    #[test]
    fn format_markdown_phase_d_returns_err_when_anomalies_alone_exceed_budget() {
        let huge_rationale = "X".repeat(50_000);
        let markers: Vec<AnomalyMarker> = (0..10)
            .map(|i| AnomalyMarker {
                kind: AnomalyKind::LatencyOutlier,
                severity: 80,
                rationale: huge_rationale.clone(),
                affected_span_ids: vec![[i as u8; 8]],
            })
            .collect();
        let out = CurationOutput {
            unique_spans: Vec::new(),
            dedup_count: 0,
            anomaly_markers: markers,
            critical_path: Vec::new(),
            input_row_count: 0,
            output_row_count: 0,
            aggregation: AggregationResult::default(),
            kept_attribute_count: 0,
            dropped_attribute_count: 0,
        };
        let result = format_markdown(&out, TokenBudget::Conservative);
        match result {
            Err(FormatError::BudgetExceeded {
                budget_tokens,
                actual_tokens,
            }) => {
                assert_eq!(budget_tokens, 10_000);
                assert!(actual_tokens > 10_000);
            }
            other => panic!("expected BudgetExceeded err, got {other:?}"),
        }
    }

    #[test]
    fn format_markdown_empty_input_produces_minimal_output() {
        let out = CurationOutput::default();
        let report = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        assert!(report.markdown.contains("# andromeda-pulse"));
        assert!(report.markdown.contains("## Anomaly observations"));
        assert!(report.markdown.contains("no anomalies detected"));
        assert_eq!(report.truncation_state, TruncationState::None);
    }

    #[test]
    fn format_markdown_global_aggregation_emitted_in_constellation_section() {
        let out = anomaly_heavy_curation_output();
        let report = format_markdown(&out, TokenBudget::Balanced).expect("ok");
        assert!(report.markdown.contains("p50_ms"));
        assert!(report.markdown.contains("p95_ms"));
        assert!(report.markdown.contains("p99_ms"));
        assert!(report.markdown.contains("max_ms"));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]

        #[test]
        fn format_markdown_token_count_invariant(
            span_count in 0_usize..200,
            preset in 0_usize..3,
        ) {
            let budget = match preset {
                0 => TokenBudget::Conservative,
                1 => TokenBudget::Balanced,
                _ => TokenBudget::Detailed,
            };
            let out = if span_count == 0 {
                CurationOutput::default()
            } else {
                many_span_curation(span_count)
            };
            match format_markdown(&out, budget) {
                Ok(report) => {
                    prop_assert!(
                        report.token_count <= budget.as_token_count(),
                        "Ok path must respect budget: {} > {}",
                        report.token_count,
                        budget.as_token_count()
                    );
                }
                Err(FormatError::BudgetExceeded { budget_tokens, actual_tokens }) => {
                    prop_assert!(actual_tokens > budget_tokens);
                }
                Err(other) => prop_assert!(false, "unexpected err {other:?}"),
            }
        }
    }
}
