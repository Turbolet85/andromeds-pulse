//! Unit tests for the deterministic env-gated L4 runner (P-073).
//! In pulse-app/tests/ per CLAUDE.md testing.md 2026-05-20 ([lib] test = false:
//! source-level `mod tests` compile but never run).

use interpretation::contract::{LlmInferenceRunner, ModelStatus, ModelTier};
use interpretation::schema::{self, Confidence, Decision, L4Output, Severity as L4Severity};
use pulse_app::deterministic_inference::{
    CANNED_L4_OUTPUT_JSON, DeterministicInferenceRunner, deterministic_mode_enabled_for,
};

#[test]
fn canned_output_is_schema_valid_and_surface_autonomous() {
    let parsed: L4Output =
        serde_json::from_str(CANNED_L4_OUTPUT_JSON).expect("canned output parses");
    schema::validate(&parsed).expect("canned output passes defense-in-depth validation");
    assert_eq!(
        parsed.decision,
        Decision::Surface,
        "surface decision → the producer creates an incident",
    );
    assert_eq!(
        parsed.severity,
        L4Severity::Autonomous,
        "autonomous severity → IncidentSeverity::Error (red-dot)",
    );
    assert!(!parsed.is_resolution_summary);
    assert_eq!(parsed.schema_version, "2.0");
}

#[test]
fn canned_output_carries_no_pii() {
    // First-party constants only — no email / bearer-token shapes.
    assert!(!CANNED_L4_OUTPUT_JSON.contains('@'), "no email-shaped PII");
    assert!(
        !CANNED_L4_OUTPUT_JSON.to_lowercase().contains("bearer "),
        "no bearer-token shape",
    );
}

/// The graded arrays are POPULATED. Asserted by exact VALUE and COUNT, never
/// by shape: a `!is_empty()` check cannot discriminate a correct fixture from
/// a wrong one, which is the vacuity class this fixture exists to remove.
#[test]
fn canned_output_populates_the_graded_evidence_refs() {
    let parsed: L4Output =
        serde_json::from_str(CANNED_L4_OUTPUT_JSON).expect("canned output parses");
    assert_eq!(
        parsed.evidence_refs,
        vec![
            "det-span-9f2c4a7e1b6d0358".to_string(),
            "det-template-0007".to_string(),
            "det-fingerprint-4a7f2b91c6e05d3849b1e7a2c5f08d63".to_string(),
        ],
        "evidence_refs carries the exact fixture set — the value the MCP \
         retrieve_telemetry_slice response and the Report Evidence section grade on",
    );
}

#[test]
fn canned_output_populates_hypotheses_and_investigation_steps() {
    let parsed: L4Output =
        serde_json::from_str(CANNED_L4_OUTPUT_JSON).expect("canned output parses");
    assert_eq!(parsed.hypotheses.len(), 2, "two hypotheses render");
    assert_eq!(
        parsed.investigation_steps.len(),
        2,
        "two investigation steps render",
    );
    assert_eq!(parsed.hypotheses[0].confidence, Confidence::High);
    assert_eq!(parsed.hypotheses[1].confidence, Confidence::Low);
    assert!(
        parsed
            .hypotheses
            .iter()
            .all(|h| !h.statement.is_empty() && !h.justification.is_empty()),
        "no empty hypothesis field — an empty one renders as absent",
    );
    assert!(
        parsed
            .investigation_steps
            .iter()
            .all(|s| !s.step.is_empty() && !s.expected_yield.is_empty()),
        "no empty step field",
    );
}

/// The fixture must survive the scrubber intact. `scrub_string` runs over
/// every evidence ref twice on the way to the rendered Report, so a value
/// tripping a scrubber category would arrive as `[redacted: …]` — a
/// differently-vacuous surface that still passes a shape-only assertion.
#[test]
fn canned_evidence_refs_survive_the_scrubber_unredacted() {
    let parsed: L4Output =
        serde_json::from_str(CANNED_L4_OUTPUT_JSON).expect("canned output parses");
    for r in &parsed.evidence_refs {
        match security::scrubber::scrub_attribute(r) {
            security::scrubber::ScrubbedValue::Allowed(v) => assert_eq!(
                v, *r,
                "evidence ref must pass the scrubber byte-identical: {r}",
            ),
            security::scrubber::ScrubbedValue::Redacted { category } => panic!(
                "evidence ref {r} tripped scrubber category {category:?} — pick a value with no \
                 secret-KV label and no digit run of 13+",
            ),
        }
    }
}

/// Bounds are the real validator's, not this test's guesses — assert the
/// fixture sits inside them so a future edit cannot silently exceed one.
#[test]
fn canned_arrays_sit_inside_the_schema_bounds() {
    let parsed: L4Output =
        serde_json::from_str(CANNED_L4_OUTPUT_JSON).expect("canned output parses");
    assert!(parsed.evidence_refs.len() <= schema::EVIDENCE_REFS_MAX);
    assert!(parsed.hypotheses.len() <= schema::HYPOTHESES_MAX);
    assert!(parsed.investigation_steps.len() <= schema::INVESTIGATION_STEPS_MAX);
    assert!(
        parsed
            .evidence_refs
            .iter()
            .all(|r| r.len() <= schema::EVIDENCE_REF_MAX_LEN),
    );
}

#[tokio::test]
async fn runner_is_loaded_and_returns_canned_output() {
    let runner = DeterministicInferenceRunner::new(ModelTier::Primary);
    assert_eq!(runner.current_status(), ModelStatus::Loaded);
    assert_eq!(runner.tier(), ModelTier::Primary);
    assert_eq!(
        runner.identity().expect("identity present").semantic_name,
        "deterministic-stub",
    );
    let out = runner
        .generate_constrained("any prompt", "any schema")
        .await
        .expect("deterministic runner never errors");
    assert_eq!(
        out, CANNED_L4_OUTPUT_JSON,
        "returns the canned output verbatim regardless of prompt/schema",
    );
}

#[test]
fn env_gate_truthy_parse_table() {
    for v in ["1", "true", "TRUE", "yes", " true ", "Yes"] {
        assert!(deterministic_mode_enabled_for(Some(v)), "{v:?} → enabled");
    }
    for v in ["0", "false", "no", "", "  ", "on", "enable"] {
        assert!(!deterministic_mode_enabled_for(Some(v)), "{v:?} → disabled");
    }
    assert!(
        !deterministic_mode_enabled_for(None),
        "unset → disabled (real mode is the default)",
    );
}
