//! Span-level masking at the pulse-app consumers of the P-047 scrubber.
//!
//! Integration-test crate per `[lib] test = false` (CLAUDE.md testing
//! 2026-05-13/2026-05-20): the persisted L4 interpretation stays parseable,
//! the investigate egress keeps the sentence around a masked token, and the
//! three persistence adapters leave a key that already passed the
//! `extract_service_name` choke point unchanged.

use interpretation::schema::{Decision, L4Output, Severity as L4Severity};
use pulse_app::baseline_persistence::scrub_service_key;
use pulse_app::inference_runtime::scrubbed_l4_json;
use pulse_app::investigate_router::scrub;
use pulse_app::lifecycle_persistence::scrub_service_name;
use pulse_app::storm_persistence::scrub_fingerprint_service;

fn l4_output_with_symptom(symptom: &str) -> L4Output {
    L4Output {
        schema_version: "2.0".into(),
        prompt_version: "v2.1".into(),
        decision: Decision::Surface,
        severity: L4Severity::Suggested,
        title: "checkout errors rising".into(),
        symptom: symptom.into(),
        timeline: "started after the 10:02 deploy".into(),
        hypotheses: vec![],
        investigation_steps: vec![],
        evidence_refs: vec!["fp-1".into()],
        fingerprint: "incident-fp".into(),
        model_tier: "primary".into(),
        hardware_profile: "cpu_primary".into(),
        is_resolution_summary: false,
    }
}

#[test]
fn span_mask_l4_summary_with_a_keyed_secret_stays_parseable() {
    let output = l4_output_with_symptom("auth rejected token=abc123 then more");

    let persisted = scrubbed_l4_json(&output).expect("serializes");
    assert!(
        !persisted.contains("abc123"),
        "the token value must not persist"
    );

    let parsed: L4Output =
        serde_json::from_str(&persisted).expect("the masked summary parses back into L4Output");
    assert_eq!(parsed.symptom, "auth rejected [redacted: secret_kv]");

    let expected = L4Output {
        symptom: "auth rejected [redacted: secret_kv]".into(),
        ..output
    };
    assert_eq!(
        parsed, expected,
        "only the field carrying the secret changes"
    );
}

#[test]
fn span_mask_l4_summary_without_a_secret_round_trips_unchanged() {
    let output = l4_output_with_symptom("p99 latency doubled on checkout");
    let persisted = scrubbed_l4_json(&output).expect("serializes");
    let parsed: L4Output = serde_json::from_str(&persisted).expect("parses back");
    assert_eq!(parsed, output);
}

#[test]
fn span_mask_investigate_scrub_keeps_the_sentence_around_a_token() {
    assert_eq!(
        scrub("look at the sk_live_51NotARealKeyOnlyForPulseTests00 rotation first"), // gitleaks:allow
        "look at the [redacted: provider_key] rotation first"
    );
}

#[test]
fn span_mask_persistence_adapters_leave_a_choke_point_masked_key_unchanged() {
    let masked = "checkout [REDACTED:email]";
    assert_eq!(scrub_service_key(masked), masked);
    assert_eq!(scrub_service_name(masked), masked);
    assert_eq!(scrub_fingerprint_service(masked), masked);
}

#[test]
fn span_mask_persistence_adapters_mask_a_raw_key_like_the_choke_point() {
    let raw = "checkout owner@example.com";
    let expected = "checkout [REDACTED:email]";
    assert_eq!(scrub_service_key(raw), expected);
    assert_eq!(scrub_service_name(raw), expected);
    assert_eq!(scrub_fingerprint_service(raw), expected);
}
