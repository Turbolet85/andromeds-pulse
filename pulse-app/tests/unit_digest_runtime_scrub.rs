// Migrated 2026-08-30 from `pulse-app/src/digest_runtime.rs::tests` — that
// crate sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS.

use pulse_app::digest_runtime::pii_scrub_closure;

#[test]
fn pii_scrub_closure_redacts_jwt() {
    let scrub = pii_scrub_closure();
    let input = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.signaturedata123456";
    let out = scrub(input);
    assert!(out.starts_with("[redacted:"), "expected redaction: {out}");
}

#[test]
fn pii_scrub_closure_allows_clean_text() {
    let scrub = pii_scrub_closure();
    let out = scrub("hello world");
    assert_eq!(out, "hello world");
}

const DIGEST_WITH_DATE_STAMPED_PROJECT: &str = "WINDOW: 300s, tier1 cadence\n\
PROJECT: rm-20260923-093840 (vcs=git)\n\
OVERALL: anomalous (0 active incident(s); 1 cue(s))\n\
SERVICES (rate, error%, p99 vs baselines):\n  checkout-service     12.5/s | 0.0% | 41ms\n";

/// The fixture's OVERALL line is the line the real renderer writes for a
/// one-cue Tier1 digest, so the scrub pins above exercise the shipped shape.
#[test]
fn pii_scrub_fixture_overall_line_matches_the_render() {
    use std::time::Duration;
    use triage::contract::{
        CueKind, CueScope, DigestCueRef, DigestProjectContext, PriorityTier, render_payload,
    };

    let project = DigestProjectContext {
        workspace_canonical_path: "/ws/project".to_string(),
        project_name: Some("rm-20260923-093840".to_string()),
        vcs_type: Some("git"),
        recent_commits: Vec::new(),
        framework_signals: Vec::new(),
    };
    let cue = DigestCueRef {
        kind: CueKind::RetryStorm,
        priority_tier: PriorityTier::Autonomous,
        summary: "retry_storm scope_id=checkout-service".to_string(),
        scope: CueScope::Service,
        fingerprint: None,
        scope_id: Some("checkout-service".to_string()),
    };
    let rendered = render_payload(
        Duration::from_secs(300),
        "tier1",
        &project,
        &[],
        &[cue],
        &[],
        &[],
        false,
    );
    let line_of = |text: &str| -> String {
        text.lines()
            .find(|l| l.starts_with("OVERALL: "))
            .expect("an OVERALL line")
            .to_string()
    };
    assert_eq!(
        line_of(DIGEST_WITH_DATE_STAMPED_PROJECT),
        line_of(&rendered)
    );
    assert!(
        rendered
            .starts_with("WINDOW: 300s, tier1 cadence\nPROJECT: rm-20260923-093840 (vcs=git)\n")
    );
}

#[test]
fn pii_scrub_closure_keeps_a_digest_whose_project_line_carries_a_date_stamp() {
    let scrub = pii_scrub_closure();
    let out = scrub(DIGEST_WITH_DATE_STAMPED_PROJECT);
    assert_eq!(out, DIGEST_WITH_DATE_STAMPED_PROJECT);
}

#[test]
fn pii_scrub_closure_span_masks_only_the_card_in_a_digest() {
    let scrub = pii_scrub_closure();
    let payload = format!("{DIGEST_WITH_DATE_STAMPED_PROJECT}  note: card 4111 1111 1111 1111\n");
    let out = scrub(&payload);
    assert_eq!(
        out,
        format!("{DIGEST_WITH_DATE_STAMPED_PROJECT}  note: card [redacted:credit_card]\n")
    );
}

#[test]
fn pii_scrub_closure_span_masks_a_card_line_and_a_keyed_line_independently() {
    let scrub = pii_scrub_closure();
    let payload = format!(
        "{DIGEST_WITH_DATE_STAMPED_PROJECT}  note: card 4111 1111 1111 1111\n  \
         cue: retry with password=hunter2 failed\n  tail: checkout recovered\n"
    );
    let out = scrub(&payload);
    assert_eq!(
        out,
        format!(
            "{DIGEST_WITH_DATE_STAMPED_PROJECT}  note: card [redacted:credit_card]\n  \
             cue: retry with [redacted:secret_kv]\n  tail: checkout recovered\n"
        )
    );
}
