//! Diagnostic Report markdown serializer (chunk #88 — Epoch 9 Foundation v0.2.0).
//!
//! Composes the six-section Report structure per capability P-031
//! (symptom / timeline / hypotheses / investigation steps / evidence /
//! project context) plus optional resolution-summary + "Previously seen"
//! cross-incident subsection per P-036. Pure-function serializer
//! producing markdown identical к the future MCP delivery (#92) per
//! P-038 single-source-of-truth discipline.
//!
//! Pre-scrub contract: caller MUST have routed every user-facing text
//! field through `security::scrubber::scrub_attribute` BEFORE constructing
//! the `Report` struct. This module is pure transform — no scrubbing
//! happens here per arch §Module dependency direction (interpretation
//! crate has no security dep edge).
//!
//! Hybrid render contract (chunk #88 Phase 1 user-approved scope):
//! - Resolved incidents с `resolution_summary_text.is_some()` →
//!   caller parses the JSON-encoded L4Output payload and threads its
//!   six fields into the Report → full six-section markdown.
//! - Active / Acknowledged incidents → caller constructs Report с
//!   `degraded_mode = true`, empty hypotheses + investigation_steps;
//!   serializer surfaces explicit "interpretation pending" notice in
//!   place of those sections per chunk #86 degraded-mode UX pattern.

use serde::{Deserialize, Serialize};

/// Confidence label for а ranked hypothesis. Mirrors
/// [`crate::schema::Confidence`] but stays decoupled at this surface so
/// the markdown serializer doesn't import from `schema` (keeps `Report`
/// usable for callers that synthesize hypotheses outside the L4Output
/// JSON parse path).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HypothesisView {
    pub statement: String,
    pub confidence_label: String,
    pub justification: String,
}

/// Single investigation step with expected yield description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestigationStepView {
    pub step: String,
    pub expected_yield: String,
}

/// Cross-incident match surfaced under the "Previously seen" subsection
/// per P-036. Chunk #88 reserves the slot; corpus query path is а
/// follow-up — current impl always passes empty `Vec`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviouslySeenMatch {
    pub incident_id: i64,
    pub opened_at_unix_nano: i64,
    pub title: String,
    pub workspace: String,
}

/// Six-section Report content payload. All text fields are pre-scrubbed
/// at construction; the serializer does NOT re-scrub. `degraded_mode`
/// signals chunk #86 FSM degraded state OR absence of attached L4 output
/// for Active/Acknowledged incidents — both render с explicit notice in
/// place of hypotheses + investigation_steps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub incident_id: i64,
    pub title: String,
    pub workspace: String,
    pub opened_at_unix_nano: i64,
    pub status_label: String,
    pub severity_label: String,
    pub symptom: String,
    pub timeline: String,
    pub hypotheses: Vec<HypothesisView>,
    pub investigation_steps: Vec<InvestigationStepView>,
    pub evidence_refs: Vec<String>,
    pub project_context: String,
    pub degraded_mode: bool,
    pub resolution_summary: Option<String>,
    pub previously_seen: Vec<PreviouslySeenMatch>,
}

/// Explicit notice text when degraded_mode is true. Stable string —
/// integration tests grep for а distinguishing substring к assert the
/// degraded path fired.
pub const DEGRADED_NOTICE: &str = "_Interpretation pending — hypotheses and investigation steps will populate as soon as the next L4 inference cycle completes for this incident. See diagnostics for retry options._";

/// Serialize а `Report` к markdown. Pure function; deterministic output
/// for identical input (regression-protected via а test asserting
/// `assert_eq!(serialize_report(&r), serialize_report(&r))`). Output
/// format is byte-identical к the future MCP delivery payload per P-038.
pub fn serialize_report(report: &Report) -> String {
    let mut out = String::with_capacity(2048);
    out.push_str("# Diagnostic Report: ");
    out.push_str(&report.title);
    out.push_str("\n\n");

    out.push_str("- **Incident ID:** ");
    out.push_str(&report.incident_id.to_string());
    out.push('\n');
    out.push_str("- **Workspace:** ");
    out.push_str(&report.workspace);
    out.push('\n');
    out.push_str("- **Opened (unix-nano):** ");
    out.push_str(&report.opened_at_unix_nano.to_string());
    out.push('\n');
    out.push_str("- **Status:** ");
    out.push_str(&report.status_label);
    out.push('\n');
    out.push_str("- **Severity:** ");
    out.push_str(&report.severity_label);
    out.push_str("\n\n");

    out.push_str("## Symptom\n\n");
    if report.symptom.is_empty() {
        out.push_str("_No symptom narrative available._\n\n");
    } else {
        out.push_str(&report.symptom);
        out.push_str("\n\n");
    }

    out.push_str("## Timeline\n\n");
    if report.timeline.is_empty() {
        out.push_str("_No timeline narrative available._\n\n");
    } else {
        out.push_str(&report.timeline);
        out.push_str("\n\n");
    }

    out.push_str("## Hypotheses\n\n");
    if report.degraded_mode {
        out.push_str(DEGRADED_NOTICE);
        out.push_str("\n\n");
    } else if report.hypotheses.is_empty() {
        out.push_str("_No ranked hypotheses produced._\n\n");
    } else {
        for h in &report.hypotheses {
            out.push_str("- **(");
            out.push_str(&h.confidence_label);
            out.push_str(")** ");
            out.push_str(&h.statement);
            if !h.justification.is_empty() {
                out.push_str("\n  - ");
                out.push_str(&h.justification);
            }
            out.push('\n');
        }
        out.push('\n');
    }

    out.push_str("## Investigation Steps\n\n");
    if report.degraded_mode {
        out.push_str(DEGRADED_NOTICE);
        out.push_str("\n\n");
    } else if report.investigation_steps.is_empty() {
        out.push_str("_No suggested investigation steps produced._\n\n");
    } else {
        for (idx, s) in report.investigation_steps.iter().enumerate() {
            out.push_str(&(idx + 1).to_string());
            out.push_str(". ");
            out.push_str(&s.step);
            if !s.expected_yield.is_empty() {
                out.push_str("\n   - _Expected yield:_ ");
                out.push_str(&s.expected_yield);
            }
            out.push('\n');
        }
        out.push('\n');
    }

    out.push_str("## Evidence\n\n");
    if report.evidence_refs.is_empty() {
        out.push_str("_No evidence references attached._\n\n");
    } else {
        for r in &report.evidence_refs {
            out.push_str("- `");
            out.push_str(r);
            out.push_str("`\n");
        }
        out.push('\n');
    }

    out.push_str("## Project Context\n\n");
    if report.project_context.is_empty() {
        out.push_str("_No project context attached._\n\n");
    } else {
        out.push_str(&report.project_context);
        out.push_str("\n\n");
    }

    if let Some(summary) = &report.resolution_summary {
        out.push_str("## Resolution Summary\n\n");
        out.push_str(summary);
        out.push_str("\n\n");
    }

    if !report.previously_seen.is_empty() {
        out.push_str("## Previously Seen\n\n");
        for m in &report.previously_seen {
            out.push_str("- incident #");
            out.push_str(&m.incident_id.to_string());
            out.push_str(" @ ");
            out.push_str(&m.opened_at_unix_nano.to_string());
            out.push_str(" — ");
            out.push_str(&m.title);
            out.push_str(" (");
            out.push_str(&m.workspace);
            out.push_str(")\n");
        }
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_resolved_report() -> Report {
        Report {
            incident_id: 42,
            title: "Database connection saturation".into(),
            workspace: "ws-a".into(),
            opened_at_unix_nano: 1_700_000_000_000,
            status_label: "resolved".into(),
            severity_label: "suggested".into(),
            symptom: "Connection pool exhaustion observed in service-a".into(),
            timeline: "Saturation began at 12:34Z; recovered at 12:40Z".into(),
            hypotheses: vec![HypothesisView {
                statement: "Pool size too small for current load".into(),
                confidence_label: "high".into(),
                justification: "Pool size 10 vs observed 40 req/sec".into(),
            }],
            investigation_steps: vec![InvestigationStepView {
                step: "Inspect pool config".into(),
                expected_yield: "Confirm max_connections setting".into(),
            }],
            evidence_refs: vec!["span:abc123".into(), "fp:db-sat".into()],
            project_context: "workspace=ws-a; services=service-a, service-b".into(),
            degraded_mode: false,
            resolution_summary: Some("Auto-resolved after 120s of no re-emission".into()),
            previously_seen: vec![PreviouslySeenMatch {
                incident_id: 7,
                opened_at_unix_nano: 1_500_000_000_000,
                title: "Earlier saturation event".into(),
                workspace: "ws-a".into(),
            }],
        }
    }

    fn sample_degraded_report() -> Report {
        Report {
            incident_id: 43,
            title: "Active incident pending interpretation".into(),
            workspace: "ws-a".into(),
            opened_at_unix_nano: 1_700_000_000_500,
            status_label: "active".into(),
            severity_label: "warn".into(),
            symptom: "Error rate elevated on service-b".into(),
            timeline: String::new(),
            hypotheses: Vec::new(),
            investigation_steps: Vec::new(),
            evidence_refs: vec!["fp:err-spike".into()],
            project_context: "workspace=ws-a".into(),
            degraded_mode: true,
            resolution_summary: None,
            previously_seen: Vec::new(),
        }
    }

    #[test]
    fn serialize_includes_all_six_section_headings_for_resolved() {
        let md = serialize_report(&sample_resolved_report());
        for section in [
            "## Symptom",
            "## Timeline",
            "## Hypotheses",
            "## Investigation Steps",
            "## Evidence",
            "## Project Context",
        ] {
            assert!(
                md.contains(section),
                "expected section heading {section} in output:\n{md}"
            );
        }
    }

    #[test]
    fn serialize_includes_title_and_metadata_block() {
        let md = serialize_report(&sample_resolved_report());
        assert!(md.starts_with("# Diagnostic Report: Database connection saturation"));
        assert!(md.contains("- **Incident ID:** 42"));
        assert!(md.contains("- **Workspace:** ws-a"));
        assert!(md.contains("- **Status:** resolved"));
        assert!(md.contains("- **Severity:** suggested"));
    }

    #[test]
    fn serialize_renders_resolution_summary_section_when_present() {
        let md = serialize_report(&sample_resolved_report());
        assert!(md.contains("## Resolution Summary"));
        assert!(md.contains("Auto-resolved after 120s of no re-emission"));
    }

    #[test]
    fn serialize_omits_resolution_summary_section_when_none() {
        let mut report = sample_resolved_report();
        report.resolution_summary = None;
        let md = serialize_report(&report);
        assert!(!md.contains("## Resolution Summary"));
    }

    #[test]
    fn serialize_renders_previously_seen_section_when_non_empty() {
        let md = serialize_report(&sample_resolved_report());
        assert!(md.contains("## Previously Seen"));
        assert!(md.contains("incident #7"));
        assert!(md.contains("Earlier saturation event"));
    }

    #[test]
    fn serialize_omits_previously_seen_section_when_empty() {
        let mut report = sample_resolved_report();
        report.previously_seen.clear();
        let md = serialize_report(&report);
        assert!(!md.contains("## Previously Seen"));
    }

    #[test]
    fn degraded_mode_replaces_hypotheses_with_explicit_notice() {
        let md = serialize_report(&sample_degraded_report());
        assert!(md.contains("## Hypotheses"));
        assert!(md.contains("Interpretation pending"));
    }

    #[test]
    fn degraded_mode_replaces_investigation_steps_with_explicit_notice() {
        let md = serialize_report(&sample_degraded_report());
        assert!(md.contains("## Investigation Steps"));
        // Notice appears twice (hypotheses + investigation_steps); both are
        // explicit replacements per chunk #88 spec.
        let notice_count = md.matches("Interpretation pending").count();
        assert_eq!(notice_count, 2);
    }

    #[test]
    fn degraded_mode_still_renders_symptom_and_evidence() {
        let md = serialize_report(&sample_degraded_report());
        assert!(md.contains("Error rate elevated on service-b"));
        assert!(md.contains("fp:err-spike"));
    }

    #[test]
    fn serialize_deterministic_byte_identical_for_identical_input() {
        let report = sample_resolved_report();
        let a = serialize_report(&report);
        let b = serialize_report(&report);
        assert_eq!(a, b);
    }

    #[test]
    fn serialize_pii_pass_through_preserves_already_scrubbed_input_verbatim() {
        let mut report = sample_resolved_report();
        report.symptom = "[redacted: bearer_token]".into();
        report.timeline = "[redacted: email]".into();
        let md = serialize_report(&report);
        assert!(md.contains("[redacted: bearer_token]"));
        assert!(md.contains("[redacted: email]"));
        assert!(!md.contains("Bearer "));
        assert!(!md.contains("@example.com"));
    }

    #[test]
    fn serialize_handles_empty_symptom_with_placeholder() {
        let mut report = sample_resolved_report();
        report.symptom = String::new();
        let md = serialize_report(&report);
        assert!(md.contains("## Symptom"));
        assert!(md.contains("_No symptom narrative available._"));
    }

    #[test]
    fn serialize_handles_no_hypotheses_non_degraded_with_placeholder() {
        let mut report = sample_resolved_report();
        report.hypotheses.clear();
        let md = serialize_report(&report);
        assert!(md.contains("_No ranked hypotheses produced._"));
        // Should NOT contain the degraded-mode notice when degraded_mode = false
        assert!(!md.contains("Interpretation pending"));
    }

    #[test]
    fn serialize_handles_no_investigation_steps_non_degraded_with_placeholder() {
        let mut report = sample_resolved_report();
        report.investigation_steps.clear();
        let md = serialize_report(&report);
        assert!(md.contains("_No suggested investigation steps produced._"));
    }

    #[test]
    fn serialize_handles_no_evidence_with_placeholder() {
        let mut report = sample_resolved_report();
        report.evidence_refs.clear();
        let md = serialize_report(&report);
        assert!(md.contains("_No evidence references attached._"));
    }

    #[test]
    fn serialize_includes_hypothesis_confidence_label() {
        let md = serialize_report(&sample_resolved_report());
        assert!(md.contains("**(high)** Pool size too small for current load"));
        assert!(md.contains("Pool size 10 vs observed 40 req/sec"));
    }

    #[test]
    fn serialize_includes_numbered_investigation_steps() {
        let md = serialize_report(&sample_resolved_report());
        assert!(md.contains("1. Inspect pool config"));
        assert!(md.contains("_Expected yield:_ Confirm max_connections setting"));
    }

    #[test]
    fn serialize_evidence_refs_use_inline_code_formatting() {
        let md = serialize_report(&sample_resolved_report());
        assert!(md.contains("- `span:abc123`"));
        assert!(md.contains("- `fp:db-sat`"));
    }

    #[test]
    fn serialize_section_order_is_stable() {
        let md = serialize_report(&sample_resolved_report());
        let symptom_pos = md.find("## Symptom").expect("Symptom present");
        let timeline_pos = md.find("## Timeline").expect("Timeline present");
        let hypotheses_pos = md.find("## Hypotheses").expect("Hypotheses present");
        let steps_pos = md
            .find("## Investigation Steps")
            .expect("Investigation Steps present");
        let evidence_pos = md.find("## Evidence").expect("Evidence present");
        let context_pos = md
            .find("## Project Context")
            .expect("Project Context present");
        assert!(symptom_pos < timeline_pos);
        assert!(timeline_pos < hypotheses_pos);
        assert!(hypotheses_pos < steps_pos);
        assert!(steps_pos < evidence_pos);
        assert!(evidence_pos < context_pos);
    }

    #[test]
    fn report_serialize_round_trips_through_serde() {
        let report = sample_resolved_report();
        let json = serde_json::to_string(&report).expect("serialize");
        let parsed: Report = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, report);
    }
}
