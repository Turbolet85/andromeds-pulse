use std::collections::HashMap;
use std::time::Instant;

use tracing::instrument;

use crate::contract::{CriticalPathStep, SpanRecord};

#[instrument(
    skip_all,
    fields(
        input_row_count = spans.len(),
        critical_path_span_count = tracing::field::Empty,
        total_span_count = tracing::field::Empty,
        orphan_parent_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ),
)]
pub fn extract_critical_path(spans: &[SpanRecord]) -> Vec<CriticalPathStep> {
    let started = Instant::now();
    let total_span_count = spans.len();
    let span = tracing::Span::current();
    if spans.is_empty() {
        span.record("critical_path_span_count", 0_u64);
        span.record("total_span_count", 0_u64);
        span.record("orphan_parent_count", 0_u64);
        span.record("duration_ms", started.elapsed().as_millis() as u64);
        return Vec::new();
    }

    let by_id: HashMap<[u8; 8], &SpanRecord> = spans.iter().map(|s| (s.span_id, s)).collect();
    let mut children_of: HashMap<[u8; 8], Vec<&SpanRecord>> = HashMap::new();
    let mut orphan_parent_count: u64 = 0;
    let mut roots: Vec<&SpanRecord> = Vec::new();

    for s in spans {
        match s.parent_span_id {
            None => roots.push(s),
            Some(parent_id) => {
                if by_id.contains_key(&parent_id) {
                    children_of.entry(parent_id).or_default().push(s);
                } else {
                    // Orphan parent reference: include the span itself as a
                    // synthetic root so its sub-tree can still surface, but
                    // count for observability.
                    orphan_parent_count += 1;
                    roots.push(s);
                }
            }
        }
    }

    for children in children_of.values_mut() {
        children.sort_by_key(|s| s.span_id);
    }
    roots.sort_by_key(|s| s.span_id);

    let mut best: Option<Vec<CriticalPathStep>> = None;
    for root in &roots {
        let candidate = longest_path_from(root, &children_of);
        best = Some(prefer_longer_or_lex(best, candidate));
    }
    let path = best.unwrap_or_default();

    span.record("critical_path_span_count", path.len() as u64);
    span.record("total_span_count", total_span_count as u64);
    span.record("orphan_parent_count", orphan_parent_count);
    span.record("duration_ms", started.elapsed().as_millis() as u64);
    path
}

fn longest_path_from<'a>(
    span: &'a SpanRecord,
    children_of: &HashMap<[u8; 8], Vec<&'a SpanRecord>>,
) -> Vec<CriticalPathStep> {
    let self_step = CriticalPathStep {
        span_id: span.span_id,
        duration_ms: span_duration_ms(span),
    };
    let children = match children_of.get(&span.span_id) {
        Some(c) if !c.is_empty() => c,
        _ => return vec![self_step],
    };

    let mut best: Option<Vec<CriticalPathStep>> = None;
    for child in children {
        let sub = longest_path_from(child, children_of);
        best = Some(prefer_longer_or_lex(best, sub));
    }

    let mut path = vec![self_step];
    if let Some(sub) = best {
        path.extend(sub);
    }
    path
}

fn prefer_longer_or_lex(
    current: Option<Vec<CriticalPathStep>>,
    candidate: Vec<CriticalPathStep>,
) -> Vec<CriticalPathStep> {
    let cur = match current {
        Some(c) => c,
        None => return candidate,
    };
    let cur_total: u64 = cur.iter().map(|s| s.duration_ms).sum();
    let candidate_total: u64 = candidate.iter().map(|s| s.duration_ms).sum();
    let cur_first = cur.first().map(|s| s.span_id).unwrap_or([0; 8]);
    let candidate_first = candidate.first().map(|s| s.span_id).unwrap_or([0; 8]);
    if candidate_total > cur_total || (candidate_total == cur_total && candidate_first < cur_first)
    {
        candidate
    } else {
        cur
    }
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

    fn span(span_id_byte: u8, parent: Option<u8>, duration_ns: i64) -> SpanRecord {
        SpanRecord {
            trace_id: [1; 16],
            span_id: [span_id_byte; 8],
            parent_span_id: parent.map(|b| [b; 8]),
            service_name: "svc".to_string(),
            name: format!("op-{span_id_byte:02x}"),
            start_time_unix_nano: 1_000_000_000,
            end_time_unix_nano: 1_000_000_000 + duration_ns,
            status_code: 1,
            attributes: Vec::new(),
        }
    }

    #[test]
    fn empty_input_returns_empty_path() {
        assert!(extract_critical_path(&[]).is_empty());
    }

    #[test]
    fn single_span_returns_single_step() {
        let s = span(0xAA, None, 50_000_000);
        let r = extract_critical_path(std::slice::from_ref(&s));
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].span_id, s.span_id);
        assert_eq!(r[0].duration_ms, 50);
    }

    #[test]
    fn linear_chain_returns_full_chain_in_order() {
        let r = span(0x01, None, 100_000_000);
        let m = span(0x02, Some(0x01), 50_000_000);
        let l = span(0x03, Some(0x02), 25_000_000);
        let path = extract_critical_path(&[r.clone(), m.clone(), l.clone()]);
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].span_id, r.span_id);
        assert_eq!(path[1].span_id, m.span_id);
        assert_eq!(path[2].span_id, l.span_id);
    }

    #[test]
    fn binary_tree_picks_longer_branch() {
        let r = span(0x01, None, 100_000_000);
        let short_a = span(0x02, Some(0x01), 10_000_000);
        let long_b = span(0x03, Some(0x01), 200_000_000);
        let path = extract_critical_path(&[r.clone(), short_a, long_b.clone()]);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].span_id, r.span_id);
        assert_eq!(path[1].span_id, long_b.span_id);
    }

    #[test]
    fn equal_branches_break_tie_by_lower_span_id() {
        let r = span(0x01, None, 100_000_000);
        let aa = span(0xAA, Some(0x01), 50_000_000);
        let bb = span(0xBB, Some(0x01), 50_000_000);
        let path = extract_critical_path(&[r.clone(), aa.clone(), bb]);
        assert_eq!(path.len(), 2);
        assert_eq!(path[1].span_id, aa.span_id);
    }

    #[test]
    fn two_disjoint_trees_returns_longest_chain() {
        let trace_a_root = span(0x01, None, 100_000_000);
        let trace_a_leaf = span(0x02, Some(0x01), 200_000_000);
        let trace_b_root = span(0x10, None, 50_000_000);
        let trace_b_leaf = span(0x11, Some(0x10), 25_000_000);
        let path = extract_critical_path(&[
            trace_a_root.clone(),
            trace_a_leaf.clone(),
            trace_b_root,
            trace_b_leaf,
        ]);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].span_id, trace_a_root.span_id);
        assert_eq!(path[1].span_id, trace_a_leaf.span_id);
    }

    #[test]
    fn orphan_parent_reference_treated_as_synthetic_root_no_panic() {
        let real_root = span(0x01, None, 100_000_000);
        let orphan = span(0x02, Some(0xEE), 200_000_000);
        let path = extract_critical_path(&[real_root, orphan.clone()]);
        // Orphan span has bigger duration so its sub-path wins.
        assert_eq!(path.len(), 1);
        assert_eq!(path[0].span_id, orphan.span_id);
    }

    #[test]
    fn deterministic_across_invocations() {
        let inputs = vec![
            span(0x05, Some(0x01), 30_000_000),
            span(0x01, None, 100_000_000),
            span(0x03, Some(0x01), 30_000_000),
            span(0x02, Some(0x01), 30_000_000),
        ];
        let r1 = extract_critical_path(&inputs);
        let r2 = extract_critical_path(&inputs);
        assert_eq!(r1, r2);
    }

    #[test]
    fn negative_duration_clamped_to_zero_no_panic() {
        let weird = SpanRecord {
            trace_id: [1; 16],
            span_id: [0xAA; 8],
            parent_span_id: None,
            service_name: "svc".to_string(),
            name: "op".to_string(),
            start_time_unix_nano: 2_000_000_000,
            end_time_unix_nano: 1_000_000_000,
            status_code: 1,
            attributes: Vec::new(),
        };
        let path = extract_critical_path(std::slice::from_ref(&weird));
        assert_eq!(path.len(), 1);
        assert_eq!(path[0].duration_ms, 0);
    }
}
