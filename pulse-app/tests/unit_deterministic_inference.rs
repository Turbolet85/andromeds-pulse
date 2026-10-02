//! Unit tests for the deterministic env-gated L4 runner (P-073).
//! In pulse-app/tests/ per CLAUDE.md testing.md 2026-05-20 ([lib] test = false:
//! source-level `mod tests` compile but never run).

use interpretation::contract::{LlmInferenceRunner, ModelStatus, ModelTier};
use interpretation::degraded_mode::{BackoffSnapshot, DegradedModeStatus};
use interpretation::schema::{self, Confidence, Decision, L4Output, Severity as L4Severity};
use pulse_app::deterministic_inference::{
    CANNED_L4_OUTPUT_JSON, DeterministicInferenceRunner, deterministic_mode_enabled_for,
};
use pulse_app::inference_runtime::{L4DigestOutcome, process_digest};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, GenerationDamper,
    InMemoryIncidentRegistry, Incident, IncidentError, IncidentPersistence, IncidentRegistry,
    PriorityTier,
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

// ---------------------------------------------------------------------------
// Branch-level blind-spot pin: the attach-vs-create seam under det-L4
// (`pulse_app::inference_runtime::process_digest`). The fixture-flag half is
// `canned_output_is_schema_valid_and_surface_autonomous` above.
// ---------------------------------------------------------------------------

struct NeverBackoff;

impl DegradedModeStatus for NeverBackoff {
    fn record_failure(&self, _now_unix_nano: i64) -> BackoffSnapshot {
        BackoffSnapshot::fresh_active()
    }
    fn record_success(&self, _now_unix_nano: i64) -> BackoffSnapshot {
        BackoffSnapshot::fresh_active()
    }
    fn current_snapshot(&self, _now_unix_nano: i64) -> BackoffSnapshot {
        BackoffSnapshot::fresh_active()
    }
    fn is_in_backoff(&self, _now_unix_nano: i64) -> bool {
        false
    }
    fn trigger_manual_retry(&self, _now_unix_nano: i64) -> BackoffSnapshot {
        BackoffSnapshot::fresh_active()
    }
}

struct NoopPersistence;

impl IncidentPersistence for NoopPersistence {
    fn save_new_incident(&self, _incident: &Incident) -> Result<i64, IncidentError> {
        Ok(1)
    }
    fn update_incident_status(
        &self,
        _id: i64,
        _payload: &Incident,
    ) -> Result<triage::contract::IncidentWriteOutcome, IncidentError> {
        Ok(triage::contract::IncidentWriteOutcome::Applied)
    }
    fn mark_read(&self, _id: i64, _read_unix_nano: i64) -> Result<(), IncidentError> {
        Ok(())
    }
    fn load_active_incidents(&self, _workspace: &str) -> Result<Vec<Incident>, IncidentError> {
        Ok(vec![])
    }
    fn load_incidents_for_workspace_since(
        &self,
        _workspace: &str,
        _since_unix_nano: i64,
    ) -> Result<Vec<Incident>, IncidentError> {
        Ok(vec![])
    }
    fn count_active_unread(&self, _workspace: &str) -> Result<u64, IncidentError> {
        Ok(0)
    }
    fn save_incident_event(
        &self,
        _incident_id: i64,
        _event_kind: &str,
        _occurred_at: i64,
    ) -> Result<(), IncidentError> {
        Ok(())
    }
}

const NON_RESOLUTION_KINDS: [DigestKind; 8] = [
    DigestKind::Snapshot,
    DigestKind::IncidentSummary,
    DigestKind::BaselineState,
    DigestKind::AttentionCueDigest,
    DigestKind::CadenceTier1,
    DigestKind::CadenceTier2,
    DigestKind::CadenceTier3,
    DigestKind::Reflection,
];

/// Cue-bearing digest of the given kind. The cue supplies the incident
/// identity for non-Reflection kinds; Reflection derives its synthetic
/// workspace-global identity and ignores it.
fn digest_of_kind(kind: DigestKind) -> Digest {
    Digest {
        kind,
        token_count: 512,
        payload_summary: "WINDOW: 60s ...".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000,
        workspace: "/home/dev/example".to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![DigestCueRef {
            kind: CueKind::ErrorRateSpike,
            priority_tier: PriorityTier::Suggested,
            summary: "cue summary".to_string(),
            scope: CueScope::Service,
            fingerprint: Some("a3f91c0b7e2d4568a3f91c0b7e2d4568".to_string()),
            scope_id: Some("svc-under-pin".to_string()),
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

/// Blind-spot pin, branch half: under the deterministic runner the
/// resolution-summary ATTACH path is structurally COLD — the canned output
/// pins `is_resolution_summary: false`, so every non-ResolutionSummary digest
/// kind routes `process_digest`'s attach-vs-create branch to CREATE. An
/// absence check over the resolution-summary surfaces is therefore vacuous
/// in this mode (arch §Occupied Resources → `ANDROMEDA_PULSE_L4_DETERMINISTIC`).
/// A fixture flipping the flag, or a predicate change routing these kinds to
/// attach, reddens this pin.
#[tokio::test]
async fn canned_output_routes_every_non_resolution_digest_kind_to_create() {
    for kind in NON_RESOLUTION_KINDS {
        let runner = DeterministicInferenceRunner::new(ModelTier::Primary);
        let registry = InMemoryIncidentRegistry::new();
        let damper = GenerationDamper::new();
        let outcome = process_digest(
            &runner,
            &NeverBackoff,
            &registry,
            &NoopPersistence,
            &damper,
            &digest_of_kind(kind),
            5_000,
        )
        .await;
        assert!(
            matches!(outcome, Some(L4DigestOutcome::Success(_))),
            "{kind:?}: generation must run and parse cleanly",
        );
        assert_eq!(
            registry.count(),
            1,
            "{kind:?}: the CREATE branch must run — the resolution-attach \
             branch is cold under det-L4",
        );
    }
}

/// The discriminating control: a ResolutionSummary-KIND digest takes the
/// attach branch (no incident created), proving the branch is live rather
/// than every kind trivially creating.
#[tokio::test]
async fn resolution_summary_kind_digest_takes_the_attach_branch() {
    let runner = DeterministicInferenceRunner::new(ModelTier::Primary);
    let registry = InMemoryIncidentRegistry::new();
    let damper = GenerationDamper::new();
    let outcome = process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &NoopPersistence,
        &damper,
        &digest_of_kind(DigestKind::ResolutionSummary),
        5_000,
    )
    .await;
    assert!(matches!(outcome, Some(L4DigestOutcome::Success(_))));
    assert_eq!(
        registry.count(),
        0,
        "attach branch: no incident is created for a ResolutionSummary digest",
    );
}
