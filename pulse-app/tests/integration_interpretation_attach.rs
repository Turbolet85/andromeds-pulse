//! Integration pins for the latest-interpretation attach seam
//! (chunk 2026-08-26-interpretation-brief-completeness).
//!
//! Contract under pin: a cleanly-parsed L4Output becomes retrievable on the
//! incident — attached at creation and refreshed on every deduped
//! re-generation — so `assemble_report` renders the full six-section brief
//! for a live incident instead of the false-degraded "Interpretation
//! pending" notice. RED at HEAD: the producer writes
//! `resolution_summary_text: None` at creation, the dedupe branch never
//! touches it, and the Resolved-only attach path has no production producer.
//!
//! Lives in pulse-app/tests/ per testing.md 2026-05-20: source-level
//! `mod tests` in pulse-app never run (`[lib] test = false`).

use std::sync::{Arc, Mutex};

use interpretation::markdown::assemble_report;
use interpretation::schema::{
    Confidence, Decision, Hypothesis, InvestigationStep, L4Output, SCHEMA_VERSION,
    Severity as L4Severity,
};
use pulse_app::inference_runtime::create_incident_from_l4_output;
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, InMemoryIncidentRegistry,
    Incident, IncidentError, IncidentPersistence, IncidentRegistry, PriorityTier,
};

const WORKSPACE: &str = "/home/dev/example";
const FINGERPRINT_A: &str = "a3f91c0b7e2d4568a3f91c0b7e2d4568";

#[derive(Default)]
struct RecordingPersistence {
    saved: Mutex<Vec<Incident>>,
    updates: Mutex<Vec<(i64, Incident)>>,
    next_id: Mutex<i64>,
}

impl RecordingPersistence {
    fn last_update(&self) -> Option<(i64, Incident)> {
        self.updates.lock().expect("lock").last().cloned()
    }
}

impl IncidentPersistence for RecordingPersistence {
    fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError> {
        let mut next = self.next_id.lock().expect("lock");
        *next += 1;
        self.saved.lock().expect("lock").push(incident.clone());
        Ok(*next)
    }
    fn update_incident_status(&self, id: i64, payload: &Incident) -> Result<(), IncidentError> {
        self.updates
            .lock()
            .expect("lock")
            .push((id, payload.clone()));
        Ok(())
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

fn storm_digest() -> Digest {
    Digest {
        kind: DigestKind::CadenceTier3,
        token_count: 512,
        payload_summary: "WINDOW: 60s ... SERVICES: payment-service ...".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000,
        workspace: WORKSPACE.to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![DigestCueRef {
            kind: CueKind::ErrorRateSpike,
            priority_tier: PriorityTier::Suggested,
            summary: "[redacted] error storm on payment-service".to_string(),
            scope: CueScope::Service,
            fingerprint: Some(FINGERPRINT_A.to_string()),
            scope_id: Some("payment-service".to_string()),
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Tier1NeverLww,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

fn interpretation_output(timeline: &str) -> L4Output {
    L4Output {
        schema_version: SCHEMA_VERSION.into(),
        prompt_version: "v2.1".into(),
        decision: Decision::Surface,
        severity: L4Severity::Suggested,
        title: "Error rate spike in payment-service".into(),
        symptom: "payment-service error rate at 100% over the window.".into(),
        timeline: timeline.into(),
        hypotheses: vec![Hypothesis {
            statement: "Downstream dependency outage.".into(),
            confidence: Confidence::High,
            justification: "All requests fail with the same exception fingerprint.".into(),
        }],
        investigation_steps: vec![InvestigationStep {
            step: "Inspect payment-service dependency health.".into(),
            expected_yield: "Confirm the failing downstream call.".into(),
        }],
        evidence_refs: vec![FINGERPRINT_A.to_string()],
        fingerprint: "model-authored".into(),
        model_tier: "primary".into(),
        hardware_profile: "gpu_primary".into(),
        is_resolution_summary: false,
    }
}

/// Deliverable A, creation half: the parsed interpretation is attached to the
/// incident AT CREATION, retrievable as parseable L4Output JSON.
#[tokio::test]
async fn creation_attaches_latest_interpretation_to_incident() {
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());
    let digest = storm_digest();
    let parsed = interpretation_output("Errors began 45s ago and persist.");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref() as &dyn IncidentPersistence,
        &digest,
        &parsed,
        1_700_000_001_000_000_000,
    );

    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1, "one incident created");
    let text = active[0]
        .resolution_summary_text
        .as_deref()
        .expect("interpretation attached at creation");
    let round_trip: L4Output =
        serde_json::from_str(text).expect("attached text parses back as L4Output");
    assert_eq!(round_trip.timeline, "Errors began 45s ago and persist.");
    assert!(!round_trip.hypotheses.is_empty());
}

/// Deliverable A, dedupe half: a second generation for the same cue identity
/// REFRESHES the attached interpretation on the existing incident, and the
/// refreshed state reaches persistence via the existing BLOB rewrite.
#[tokio::test]
async fn dedupe_refreshes_latest_interpretation() {
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());
    let digest = storm_digest();

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref() as &dyn IncidentPersistence,
        &digest,
        &interpretation_output("First interpretation."),
        1_700_000_001_000_000_000,
    );
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref() as &dyn IncidentPersistence,
        &digest,
        &interpretation_output("Second interpretation, refreshed."),
        1_700_000_002_000_000_000,
    );

    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1, "dedupe kept one incident");
    let text = active[0]
        .resolution_summary_text
        .as_deref()
        .expect("interpretation present after dedupe");
    let round_trip: L4Output = serde_json::from_str(text).expect("parses back");
    assert_eq!(round_trip.timeline, "Second interpretation, refreshed.");

    let (updated_id, updated_payload) = persistence
        .last_update()
        .expect("dedupe persisted the refreshed incident");
    assert_eq!(updated_id, active[0].id);
    let persisted_text = updated_payload
        .resolution_summary_text
        .as_deref()
        .expect("persisted payload carries the refreshed interpretation");
    assert!(persisted_text.contains("Second interpretation, refreshed."));
}

/// Deliverable A, report half: with the interpretation attached, the report
/// projection for a live (non-Resolved) incident renders the FULL brief —
/// `degraded_mode == false`, populated hypotheses/timeline — via the same
/// `assemble_report` both the in-app resolver and the MCP sidecar use.
#[tokio::test]
async fn report_renders_full_brief_for_live_incident_with_attached_interpretation() {
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref() as &dyn IncidentPersistence,
        &storm_digest(),
        &interpretation_output("Live timeline narrative."),
        1_700_000_001_000_000_000,
    );

    let incident = registry.list_active(WORKSPACE).pop().expect("incident");
    let parsed: Option<L4Output> = incident
        .resolution_summary_text
        .as_deref()
        .and_then(|t| serde_json::from_str(t).ok());
    let report = assemble_report(&incident, parsed.as_ref(), vec![]);

    assert!(
        !report.degraded_mode,
        "a live incident with a cleanly-parsed interpretation must not render degraded"
    );
    assert_eq!(report.timeline, "Live timeline narrative.");
    assert!(!report.hypotheses.is_empty());
    assert!(!report.investigation_steps.is_empty());
}

/// Honest degradation preserved (the negative half of the pair): an incident
/// with NO parsed interpretation still renders the degraded notice branch.
#[tokio::test]
async fn report_stays_degraded_when_no_interpretation_attached() {
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref() as &dyn IncidentPersistence,
        &storm_digest(),
        &interpretation_output("irrelevant"),
        1_700_000_001_000_000_000,
    );
    let mut incident = registry.list_active(WORKSPACE).pop().expect("incident");
    incident.resolution_summary_text = None;

    let report = assemble_report(&incident, None, vec![]);
    assert!(report.degraded_mode);
    assert!(report.hypotheses.is_empty());
}
