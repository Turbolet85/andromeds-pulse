//! Acceptance test for Tier1 incident-path reliability (P-074):
//! a sustained identical-fingerprint storm drives the (cue->cadence->)digest->
//! deterministic-L4->incident path to EXACTLY ONE incident, reproducibly.
//!
//! The chunk's fix threads the triggering cue cadence->digest so the storm
//! digest carries `attention_cues`; the existing storm-detector one-shot +
//! incident dedup then coalesce a sustained storm to exactly one incident.
//! This test exercises the producer end of that path (cue-bearing digests ->
//! `handle_digest_outcome` -> `create_incident_from_l4_output`) under the
//! deterministic L4 runner (P-073) — no GPU/model — and proves the
//! coalescing. `cueless_digest_creates_no_incident` proves WHY the cue
//! passthrough is the fix. (verification-matrix.json#P-074)
//!
//! In pulse-app/tests/ per CLAUDE.md testing.md 2026-05-20.

use std::sync::{Arc, Mutex};

use interpretation::contract::ModelTier;
use pulse_app::deterministic_inference::DeterministicInferenceRunner;
use pulse_app::inference_runtime::{
    L4DigestOutcome, create_incident_from_l4_output, handle_digest_outcome,
};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, InMemoryIncidentRegistry,
    Incident, IncidentError, IncidentPersistence, IncidentRegistry, PriorityTier,
    Severity as IncidentSeverity,
};

const WORKSPACE: &str = "/home/dev/payments";
const STORM_SERVICE: &str = "payment-service";
const STORM_EVENTS: usize = 150;

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

/// One storm digest carrying the threaded RetryStorm cue — the shape the
/// P-074 cue-passthrough now produces for a cadence Tier-1 storm digest. Each
/// event of an identical-fingerprint storm shares the `(kind, scope, scope_id)`
/// identity, so the producer creates one and dedups the rest.
fn storm_digest(seq: usize) -> Digest {
    Digest {
        kind: DigestKind::CadenceTier1,
        token_count: 512,
        payload_summary: "WINDOW 60s, tier1 cadence\nSERVICES payment-service".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000 + seq as i64,
        workspace: WORKSPACE.to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![DigestCueRef {
            kind: CueKind::RetryStorm,
            priority_tier: PriorityTier::Autonomous,
            summary: "retry storm".to_string(),
            scope: CueScope::Service,
            fingerprint: None,
            scope_id: Some(STORM_SERVICE.to_string()),
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Tier1NeverLww,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

/// Drive a STORM_EVENTS-long identical-fingerprint storm through the real L4
/// handler + incident producer under the deterministic runner.
async fn run_storm() -> (Arc<InMemoryIncidentRegistry>, Arc<RecordingPersistence>) {
    let runner = DeterministicInferenceRunner::new(ModelTier::Primary);
    let registry = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());
    for seq in 0..STORM_EVENTS {
        let digest = storm_digest(seq);
        let parsed = match handle_digest_outcome(&runner, &digest).await {
            L4DigestOutcome::Success(p) => p,
            other => panic!("expected Success, got {other:?}"),
        };
        create_incident_from_l4_output(
            registry.as_ref(),
            persistence.as_ref(),
            &digest,
            &parsed,
            5_000 + seq as i64,
        );
    }
    (registry, persistence)
}

#[tokio::test]
async fn sustained_identical_fingerprint_storm_yields_exactly_one_incident() {
    let (registry, persistence) = run_storm().await;

    let active = registry.list_active(WORKSPACE);
    assert_eq!(
        active.len(),
        1,
        "a {STORM_EVENTS}-event identical-fingerprint storm must coalesce to exactly ONE incident",
    );
    assert_eq!(
        active[0].scope_id.as_deref(),
        Some(STORM_SERVICE),
        "incident attributed to the storm's service via the threaded cue",
    );
    assert_eq!(active[0].kind, CueKind::RetryStorm);
    assert_eq!(
        active[0].severity,
        IncidentSeverity::Error,
        "autonomous → Error (the red-dot severity)",
    );
    assert_eq!(
        persistence.saved.lock().expect("lock").len(),
        1,
        "exactly one durable create; the rest dedup",
    );
}

#[tokio::test]
async fn storm_to_one_incident_is_reproducible() {
    let (r1, _) = run_storm().await;
    let (r2, _) = run_storm().await;
    assert_eq!(r1.list_active(WORKSPACE).len(), 1, "run 1 → one incident");
    assert_eq!(r2.list_active(WORKSPACE).len(), 1, "run 2 → one incident");
}

#[tokio::test]
async fn cueless_digest_creates_no_incident() {
    // Without the threaded cue (empty attention_cues, non-reflection) the
    // producer creates NO incident — this is the gap the P-074 cue-passthrough
    // closes, and the reason a real storm previously yielded zero incidents.
    let runner = DeterministicInferenceRunner::new(ModelTier::Primary);
    let registry = Arc::new(InMemoryIncidentRegistry::new());
    let persistence = Arc::new(RecordingPersistence::default());

    let mut digest = storm_digest(0);
    digest.attention_cues.clear();
    let parsed = match handle_digest_outcome(&runner, &digest).await {
        L4DigestOutcome::Success(p) => p,
        other => panic!("expected Success, got {other:?}"),
    };
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &parsed,
        5_000,
    );

    assert!(
        registry.list_active(WORKSPACE).is_empty(),
        "a cue-less digest must not create an incident (incidents are cue-derived)",
    );
}
