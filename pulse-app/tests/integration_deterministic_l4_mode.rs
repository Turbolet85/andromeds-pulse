//! Integration test for the deterministic env-gated L4 mode (P-073):
//! a cue-bearing digest driven through the REAL L4 handler + incident producer
//! with the `DeterministicInferenceRunner` yields a reproducible red-dot
//! incident — no GPU/model. Proves the digest -> L4 -> incident chain completes
//! reproducibly (verification-matrix.json#P-073).
//!
//! In pulse-app/tests/ per CLAUDE.md testing.md 2026-05-20.

use std::sync::{Arc, Mutex};

use interpretation::contract::ModelTier;
use interpretation::schema::{Decision, Severity as L4Severity};
use pulse_app::deterministic_inference::DeterministicInferenceRunner;
use pulse_app::inference_runtime::{
    L4DigestOutcome, create_incident_from_l4_output, handle_digest_outcome,
};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, InMemoryIncidentRegistry,
    Incident, IncidentError, IncidentPersistence, IncidentRegistry, IncidentStatus, PriorityTier,
    Severity as IncidentSeverity,
};

const WORKSPACE: &str = "/home/dev/example";

/// Recording `IncidentPersistence` double — assigns rowids, captures saves.
#[derive(Default)]
struct RecordingPersistence {
    saved: Mutex<Vec<Incident>>,
    next_id: Mutex<i64>,
}

impl IncidentPersistence for RecordingPersistence {
    fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError> {
        let mut next = self.next_id.lock().expect("lock");
        *next += 1;
        self.saved.lock().expect("lock").push(incident.clone());
        Ok(*next)
    }
    fn update_incident_status(&self, _id: i64, _payload: &Incident) -> Result<(), IncidentError> {
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

/// Cue-bearing digest — the producer derives the incident identity from the
/// triggering cue (a non-cue digest creates no incident, by design).
fn digest_with_cue() -> Digest {
    Digest {
        kind: DigestKind::CadenceTier3,
        token_count: 512,
        payload_summary: "WINDOW ... SERVICES ...".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000,
        workspace: WORKSPACE.to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![DigestCueRef {
            kind: CueKind::RetryStorm,
            priority_tier: PriorityTier::Autonomous,
            summary: "retry storm".to_string(),
            scope: CueScope::Global,
            fingerprint: None,
            scope_id: None,
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

#[tokio::test]
async fn deterministic_mode_yields_reproducible_red_dot_incident() {
    let runner = DeterministicInferenceRunner::new(ModelTier::Primary);
    let digest = digest_with_cue();

    // Drive the real L4 handler twice — identical deterministic output.
    let p1 = match handle_digest_outcome(&runner, &digest).await {
        L4DigestOutcome::Success(p) => p,
        other => panic!("expected Success, got {other:?}"),
    };
    let p2 = match handle_digest_outcome(&runner, &digest).await {
        L4DigestOutcome::Success(p) => p,
        other => panic!("expected Success, got {other:?}"),
    };
    assert_eq!(p1, p2, "deterministic: identical L4Output across runs");
    assert_eq!(p1.decision, Decision::Surface);
    assert_eq!(p1.severity, L4Severity::Autonomous);

    // The parsed output drives the existing chunk #92 producer → one incident.
    let registry = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());
    create_incident_from_l4_output(registry.as_ref(), persistence.as_ref(), &digest, &p1, 5_000);

    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1, "deterministic L4 → exactly one incident");
    assert_eq!(active[0].status, IncidentStatus::Active);
    assert_eq!(
        active[0].severity,
        IncidentSeverity::Error,
        "autonomous → Error (the red-dot severity)",
    );
    assert_eq!(persistence.saved.lock().expect("lock").len(), 1);

    // The single production join: L4Output.evidence_refs -> the incident's
    // EvidenceRefs.fingerprint_hashes. This is the only path by which the
    // deterministic fixture reaches the MCP retrieve_telemetry_slice response
    // and the Report Evidence section, so an empty vector here is what made
    // every assertion against those surfaces vacuous.
    assert_eq!(
        active[0].evidence_refs.fingerprint_hashes, p1.evidence_refs,
        "the producer carries L4Output.evidence_refs into fingerprint_hashes verbatim",
    );
    assert!(
        !active[0].evidence_refs.fingerprint_hashes.is_empty(),
        "populated, not empty — the whole point of the fixture repair",
    );
    assert_eq!(
        active[0].evidence_refs.fingerprint_hashes.len(),
        3,
        "exactly the three fixture refs reach the incident",
    );
    // The persisted copy carries them too — the sidecar reads the BLOB, not
    // the in-memory registry.
    assert_eq!(
        persistence.saved.lock().expect("lock")[0]
            .evidence_refs
            .fingerprint_hashes,
        p1.evidence_refs,
        "the persisted incident carries the refs the cross-process read-back will decode",
    );
}
