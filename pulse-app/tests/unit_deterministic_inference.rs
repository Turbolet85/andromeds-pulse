//! Unit tests for the deterministic env-gated L4 runner (P-073).
//! In pulse-app/tests/ per CLAUDE.md testing.md 2026-05-20 ([lib] test = false:
//! source-level `mod tests` compile but never run).

use interpretation::contract::{LlmInferenceRunner, ModelStatus, ModelTier};
use interpretation::schema::{self, Decision, L4Output, Severity as L4Severity};
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
