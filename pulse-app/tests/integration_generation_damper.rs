//! Integration pins for the generation-damper gate at the L4 subscriber
//! seam (`pulse_app::inference_runtime::process_digest`).
//!
//! A counting stub runner proves the gate short-circuits BEFORE the
//! runner: an unchanged digest re-emission costs zero runner calls,
//! while any input change generates. The stub returns a CONSTANT output
//! — exactly the deterministic-runner property (`ANDROMEDA_PULSE_L4_DETERMINISTIC`
//! canned `L4Output`) — so these pins also prove the damper keys on
//! INPUT state and does not degenerate to always/never-suppress in the
//! reproducible-verification mode (arch §Established Decisions
//! [Fault Identity], REJECTED clause).

use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};

use interpretation::contract::{
    InferenceError, InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelStatus, ModelTier,
};
use interpretation::degraded_mode::{BackoffSnapshot, DegradedModeStatus};
use interpretation::schema::{
    Confidence, Decision, Hypothesis, InvestigationStep, L4Output, SCHEMA_VERSION,
    Severity as L4Severity,
};
use pulse_app::inference_runtime::{L4DigestOutcome, process_digest};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, GenerationDamper,
    InMemoryIncidentRegistry, Incident, IncidentPersistence, PriorityTier,
};

/// Constant-output runner counting `generate_constrained` invocations.
struct CountingRunner {
    calls: AtomicU64,
    canned_json: String,
}

impl CountingRunner {
    fn new() -> Self {
        Self {
            calls: AtomicU64::new(0),
            canned_json: serde_json::to_string(&canned_l4_output()).expect("canned serializes"),
        }
    }

    fn calls(&self) -> u64 {
        self.calls.load(Ordering::Relaxed)
    }
}

impl LlmInferenceRunner for CountingRunner {
    fn current_status(&self) -> ModelStatus {
        ModelStatus::Loaded
    }
    fn identity(&self) -> Option<ModelIdentity> {
        None
    }
    fn tier(&self) -> ModelTier {
        ModelTier::Primary
    }
    fn generate_constrained<'a>(
        &'a self,
        _prompt: &'a str,
        _schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        let out = self.canned_json.clone();
        Pin::from(Box::new(async move { Ok(out) }))
    }
}

/// Runner whose every generation fails — the reflection shape.
struct FailingCountingRunner {
    calls: AtomicU64,
}

impl LlmInferenceRunner for FailingCountingRunner {
    fn current_status(&self) -> ModelStatus {
        ModelStatus::Loaded
    }
    fn identity(&self) -> Option<ModelIdentity> {
        None
    }
    fn tier(&self) -> ModelTier {
        ModelTier::Primary
    }
    fn generate_constrained<'a>(
        &'a self,
        _prompt: &'a str,
        _schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        Pin::from(Box::new(async move {
            Err(InferenceError::InferenceFailed {
                reason: "stub failure".into(),
            })
        }))
    }
}

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

struct AlwaysBackoff;

impl DegradedModeStatus for AlwaysBackoff {
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
        true
    }
    fn trigger_manual_retry(&self, _now_unix_nano: i64) -> BackoffSnapshot {
        BackoffSnapshot::fresh_active()
    }
}

#[derive(Default)]
struct NoopPersistence;

impl IncidentPersistence for NoopPersistence {
    fn save_new_incident(
        &self,
        _incident: &Incident,
    ) -> Result<i64, triage::contract::IncidentError> {
        Ok(7)
    }
    fn update_incident_status(
        &self,
        _id: i64,
        _payload: &Incident,
    ) -> Result<triage::contract::IncidentWriteOutcome, triage::contract::IncidentError> {
        Ok(triage::contract::IncidentWriteOutcome::Applied)
    }
    fn mark_read(
        &self,
        _id: i64,
        _read_unix_nano: i64,
    ) -> Result<(), triage::contract::IncidentError> {
        Ok(())
    }
    fn load_active_incidents(
        &self,
        _workspace: &str,
    ) -> Result<Vec<Incident>, triage::contract::IncidentError> {
        Ok(vec![])
    }
    fn load_incidents_for_workspace_since(
        &self,
        _workspace: &str,
        _since_unix_nano: i64,
    ) -> Result<Vec<Incident>, triage::contract::IncidentError> {
        Ok(vec![])
    }
    fn count_active_unread(
        &self,
        _workspace: &str,
    ) -> Result<u64, triage::contract::IncidentError> {
        Ok(0)
    }
    fn save_incident_event(
        &self,
        _incident_id: i64,
        _event_kind: &str,
        _occurred_at: i64,
    ) -> Result<(), triage::contract::IncidentError> {
        Ok(())
    }
}

fn canned_l4_output() -> L4Output {
    L4Output {
        schema_version: SCHEMA_VERSION.into(),
        prompt_version: "v2.3".into(),
        decision: Decision::Watch,
        severity: L4Severity::None,
        title: "Quiet window".into(),
        symptom: "No anomalous behavior in the window.".into(),
        timeline: "Steady state.".into(),
        hypotheses: vec![Hypothesis {
            statement: "Nothing actionable.".into(),
            confidence: Confidence::Medium,
            justification: "All signals nominal.".into(),
        }],
        investigation_steps: vec![InvestigationStep {
            step: "None required.".into(),
            expected_yield: "n/a".into(),
        }],
        evidence_refs: vec![],
        fingerprint: "constant-under-deterministic-runner".into(),
        model_tier: "primary".into(),
        hardware_profile: "gpu_primary".into(),
        is_resolution_summary: false,
    }
}

fn tier1_digest(scope_id: &str, fingerprint: Option<&str>) -> Digest {
    const T0: i64 = 1_700_000_000_000_000_000;
    Digest {
        kind: DigestKind::CadenceTier1,
        token_count: 41,
        payload_summary: "payload".to_string(),
        incident_refs: Vec::new(),
        generated_at_unix_nano: T0,
        workspace: "D:/ws/project".to_string(),
        window_start_unix_nano: T0 - 60_000_000_000,
        window_end_unix_nano: T0,
        services: Vec::new(),
        attention_cues: vec![DigestCueRef {
            kind: CueKind::ServiceWentSilent,
            priority_tier: PriorityTier::Autonomous,
            summary: format!("service_went_silent scope_id={scope_id}"),
            scope: CueScope::Service,
            fingerprint: fingerprint.map(str::to_string),
            scope_id: Some(scope_id.to_string()),
        }],
        corpus_matches: Vec::new(),
        lww_mode: DigestLwwMode::Tier1NeverLww,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

const SEC: i64 = 1_000_000_000;
const NOW: i64 = 1_700_000_000_000_000_000;

#[tokio::test]
async fn unchanged_digest_reemission_costs_zero_runner_calls() {
    let runner = CountingRunner::new();
    let damper = GenerationDamper::new();
    let registry = InMemoryIncidentRegistry::new();
    let persistence = NoopPersistence;
    let digest = tier1_digest("svc-a", None);

    let first = process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &persistence,
        &damper,
        &digest,
        NOW,
    )
    .await;
    assert!(matches!(first, Some(L4DigestOutcome::Success(_))));
    assert_eq!(runner.calls(), 1);

    let second = process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &persistence,
        &damper,
        &digest,
        NOW + 60 * SEC,
    )
    .await;
    assert!(second.is_none(), "unchanged re-emission must be suppressed");
    assert_eq!(
        runner.calls(),
        1,
        "the runner must not be invoked for an unchanged digest"
    );
    assert_eq!(damper.generations_suppressed_total(), 1);
}

#[tokio::test]
async fn changed_input_generates_under_a_constant_output_runner() {
    // The runner output is CONSTANT (the deterministic-runner property):
    // discrimination below is therefore proven to key on INPUT state.
    let runner = CountingRunner::new();
    let damper = GenerationDamper::new();
    let registry = InMemoryIncidentRegistry::new();
    let persistence = NoopPersistence;

    let unchanged = tier1_digest("svc-a", None);
    process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &persistence,
        &damper,
        &unchanged,
        NOW,
    )
    .await;
    let suppressed = process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &persistence,
        &damper,
        &unchanged,
        NOW + 60 * SEC,
    )
    .await;
    assert!(suppressed.is_none());

    let changed = tier1_digest("svc-a", Some("deadbeef"));
    let third = process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &persistence,
        &damper,
        &changed,
        NOW + 120 * SEC,
    )
    .await;
    assert!(matches!(third, Some(L4DigestOutcome::Success(_))));
    assert_eq!(
        runner.calls(),
        2,
        "changed input must generate even though the output is constant"
    );
}

#[tokio::test]
async fn failed_generation_is_retried_never_suppressed() {
    let runner = FailingCountingRunner {
        calls: AtomicU64::new(0),
    };
    let damper = GenerationDamper::new();
    let registry = InMemoryIncidentRegistry::new();
    let persistence = NoopPersistence;
    let digest = tier1_digest("svc-a", None);

    let first = process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &persistence,
        &damper,
        &digest,
        NOW,
    )
    .await;
    assert!(matches!(first, Some(L4DigestOutcome::RuntimeError)));

    let second = process_digest(
        &runner,
        &NeverBackoff,
        &registry,
        &persistence,
        &damper,
        &digest,
        NOW + 60 * SEC,
    )
    .await;
    assert!(
        matches!(second, Some(L4DigestOutcome::RuntimeError)),
        "a failed generation must not mark its content analyzed"
    );
    assert_eq!(runner.calls.load(Ordering::Relaxed), 2);
    assert_eq!(damper.generations_suppressed_total(), 0);
}

#[tokio::test]
async fn backoff_skips_before_the_damper_sees_the_digest() {
    let runner = CountingRunner::new();
    let damper = GenerationDamper::new();
    let registry = InMemoryIncidentRegistry::new();
    let persistence = NoopPersistence;
    let digest = tier1_digest("svc-a", None);

    let outcome = process_digest(
        &runner,
        &AlwaysBackoff,
        &registry,
        &persistence,
        &damper,
        &digest,
        NOW,
    )
    .await;
    assert!(outcome.is_none());
    assert_eq!(runner.calls(), 0);
    assert_eq!(
        damper.generations_run_total() + damper.generations_suppressed_total(),
        0,
        "the damper must not account a digest the backoff gate already skipped"
    );
}
