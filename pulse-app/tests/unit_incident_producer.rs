//! Chunk #92 unit tests for the L4Output → Incident producer
//! (`pulse_app::inference_runtime::create_incident_from_l4_output`). Covers
//! the creation predicate (Surface → Active, Watch → Curious,
//! Dismiss / None / resolution-summary / no-cue → skip), the L4 → triage
//! severity + priority-tier mapping, scope_id threading, per-service
//! re-emission dedup, the scrub-before-persist PII negative-canary, and the
//! aggregate-only producer observability.
//!
//! Lives in pulse-app/tests/ per CLAUDE.md testing.md 2026-05-20: source-level
//! `mod tests` in pulse-app never run (`[lib] test = false`).

use std::pin::Pin;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::ThreadId;

use interpretation::contract::{
    InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelStatus, ModelTier,
};
use interpretation::degraded_mode::{BackoffSnapshot, DegradedModeStatus};
use interpretation::schema::{Decision, L4Output, Severity as L4Severity};
use pulse_app::deterministic_inference::CANNED_L4_OUTPUT_JSON;
use pulse_app::inference_runtime::{
    attach_resolution_summary_to_incident, create_incident_from_l4_output, process_digest,
};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, GenerationDamper,
    InMemoryIncidentRegistry, Incident, IncidentError, IncidentPersistence, IncidentRegistry,
    IncidentStatus, PriorityTier, ResolutionTrigger, Severity as IncidentSeverity,
};

const WORKSPACE: &str = "/home/dev/example";

/// Full-width (16-byte) lowercase-hex L1 exception fingerprints, the shape
/// `hex_lower` produces from Q3 `span_events.fingerprint` bytes. Deliberately
/// NOT all-digit: the PII scrubber's credit-card pattern matches 13-19 digit
/// runs, so an all-digit fixture would exercise a redaction path real blake3
/// output effectively never hits.
const FINGERPRINT_A: &str = "a3f91c0b7e2d4568a3f91c0b7e2d4568";
const FINGERPRINT_B: &str = "bd07e4a2915c3f6ebd07e4a2915c3f6e";

/// Recording `IncidentPersistence` mock: assigns incrementing rowids and
/// captures every saved incident + status update + event for assertion.
#[derive(Default)]
struct RecordingPersistence {
    saved: Mutex<Vec<Incident>>,
    updates: Mutex<Vec<(i64, Incident)>>,
    events: Mutex<Vec<(i64, String, i64)>>,
    next_id: Mutex<i64>,
}

impl RecordingPersistence {
    fn saved(&self) -> Vec<Incident> {
        self.saved.lock().expect("lock").clone()
    }
    fn save_count(&self) -> usize {
        self.saved.lock().expect("lock").len()
    }
    fn update_count(&self) -> usize {
        self.updates.lock().expect("lock").len()
    }
    fn event_kinds(&self) -> Vec<String> {
        self.events
            .lock()
            .expect("lock")
            .iter()
            .map(|(_, k, _)| k.clone())
            .collect()
    }
}

impl IncidentPersistence for RecordingPersistence {
    fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError> {
        let mut next = self.next_id.lock().expect("lock");
        *next += 1;
        let id = *next;
        self.saved.lock().expect("lock").push(incident.clone());
        Ok(id)
    }
    fn update_incident_status(
        &self,
        id: i64,
        payload: &Incident,
    ) -> Result<triage::contract::IncidentWriteOutcome, IncidentError> {
        self.updates
            .lock()
            .expect("lock")
            .push((id, payload.clone()));
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
        incident_id: i64,
        event_kind: &str,
        occurred_at: i64,
    ) -> Result<(), IncidentError> {
        self.events
            .lock()
            .expect("lock")
            .push((incident_id, event_kind.to_string(), occurred_at));
        Ok(())
    }
}

/// Build a digest carrying a single triggering cue (the producer derives the
/// `(kind, scope, scope_id)` incident identity from it), with no cue-borne
/// fingerprint — the baseline-family shape.
fn digest_with_cue(kind: CueKind, scope: CueScope, scope_id: Option<&str>) -> Digest {
    digest_with_fingerprinted_cue(kind, scope, scope_id, None)
}

/// As [`digest_with_cue`], but with the cue carrying an L1 exception
/// fingerprint — the storm-detector shape.
fn digest_with_fingerprinted_cue(
    kind: CueKind,
    scope: CueScope,
    scope_id: Option<&str>,
    fingerprint: Option<&str>,
) -> Digest {
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
            kind,
            priority_tier: PriorityTier::Suggested,
            summary: "cue summary".to_string(),
            scope,
            fingerprint: fingerprint.map(|f| f.to_string()),
            scope_id: scope_id.map(|s| s.to_string()),
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

fn digest_without_cue() -> Digest {
    let mut d = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    d.attention_cues = vec![];
    d
}

fn l4_output(decision: Decision, severity: L4Severity, title: &str) -> L4Output {
    L4Output {
        schema_version: "2.0".into(),
        prompt_version: "v2.1".into(),
        decision,
        severity,
        title: title.into(),
        symptom: "symptom text".into(),
        timeline: "timeline text".into(),
        hypotheses: vec![],
        investigation_steps: vec![],
        evidence_refs: vec!["fp-1".into(), "fp-2".into()],
        fingerprint: "incident-fp".into(),
        model_tier: "primary".into(),
        hardware_profile: "cpu_primary".into(),
        is_resolution_summary: false,
    }
}

fn fresh() -> (Arc<InMemoryIncidentRegistry>, Arc<RecordingPersistence>) {
    (
        Arc::new(InMemoryIncidentRegistry::new()),
        Arc::new(RecordingPersistence::default()),
    )
}

#[test]
fn surface_decision_creates_active_incident() {
    let (registry, persistence) = fresh();
    let digest = digest_with_fingerprinted_cue(
        CueKind::ErrorRateSpike,
        CueScope::Service,
        Some("auth-service"),
        Some(FINGERPRINT_A),
    );
    let output = l4_output(
        Decision::Surface,
        L4Severity::Suggested,
        "auth latency spike",
    );

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    assert_eq!(
        persistence.save_count(),
        1,
        "exactly one incident persisted"
    );
    assert_eq!(
        persistence.event_kinds(),
        vec!["created".to_string()],
        "a `created` audit event is recorded",
    );
    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1);
    let inc = &active[0];
    assert_eq!(inc.status, IncidentStatus::Active);
    assert_eq!(inc.severity, IncidentSeverity::Warn, "Suggested → Warn");
    assert_eq!(inc.priority_tier, PriorityTier::Suggested);
    assert_eq!(inc.kind, CueKind::ErrorRateSpike);
    assert_eq!(inc.scope, CueScope::Service);
    assert_eq!(
        inc.scope_id.as_deref(),
        Some("auth-service"),
        "scope_id threaded from the triggering cue",
    );
    assert_eq!(
        inc.fingerprint, FINGERPRINT_A,
        "fingerprint threaded from the triggering cue, NOT from L4Output",
    );
    assert_eq!(
        inc.evidence_refs.fingerprint_hashes,
        vec![
            "fp-1".to_string(),
            "fp-2".to_string(),
            FINGERPRINT_A.to_string()
        ],
        "grounded union: model refs first, then the cue's real fingerprint",
    );
    assert!(inc.id > 0, "rowid assigned post-INSERT");
}

#[test]
fn dismiss_decision_creates_no_incident() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let output = l4_output(Decision::Dismiss, L4Severity::Suggested, "noise");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(persistence.save_count(), 0);
    assert_eq!(registry.count(), 0);
}

#[test]
fn severity_none_creates_no_incident() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let output = l4_output(Decision::Surface, L4Severity::None, "ambiguous");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 0);
}

#[test]
fn watch_decision_creates_curious_incident() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::LatencyRegression, CueScope::Service, Some("api"));
    let output = l4_output(Decision::Watch, L4Severity::Curious, "worth recording");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].priority_tier, PriorityTier::Curious);
    assert_eq!(active[0].severity, IncidentSeverity::Info, "Curious → Info");
    assert_eq!(active[0].status, IncidentStatus::Active);
}

#[test]
fn autonomous_severity_maps_to_error_and_autonomous_tier() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::RetryStorm, CueScope::Global, None);
    let output = l4_output(Decision::Surface, L4Severity::Autonomous, "storm");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1);
    assert_eq!(
        active[0].severity,
        IncidentSeverity::Error,
        "Autonomous → Error",
    );
    assert_eq!(active[0].priority_tier, PriorityTier::Autonomous);
    assert_eq!(active[0].scope_id, None, "Global cue → no scope_id");
}

#[test]
fn no_triggering_cue_creates_no_incident() {
    let (registry, persistence) = fresh();
    let digest = digest_without_cue();
    let output = l4_output(Decision::Surface, L4Severity::Autonomous, "no cue");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 0, "incidents are strictly cue-derived");
}

#[test]
fn resolution_summary_output_creates_no_incident() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let mut output = l4_output(Decision::Surface, L4Severity::Suggested, "resolved");
    output.is_resolution_summary = true;
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 0);
}

#[test]
fn reemission_dedup_does_not_create_duplicate() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(
        CueKind::ErrorRateSpike,
        CueScope::Service,
        Some("auth-service"),
    );
    let output = l4_output(Decision::Surface, L4Severity::Suggested, "spike");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    // Second digest, same (kind, scope, scope_id) identity, later timestamp.
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        9_000,
    );

    assert_eq!(
        registry.count(),
        1,
        "same service identity dedups to one incident",
    );
    assert_eq!(persistence.save_count(), 1, "only the first call INSERTs");
    assert!(
        persistence.update_count() >= 1,
        "reemission drives a persistence update",
    );
    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        inc.updated_at_unix_nano, 9_000,
        "reemission bumped updated_at",
    );
}

/// DECIDED SEMANTIC (coalesce-per-cue-identity): a storm carrying a DIFFERENT
/// fingerprint on a service that already has an open incident is absorbed into
/// it. Pinned so the decision cannot silently drift into per-fingerprint
/// identity — which would additionally be a no-op under the deterministic
/// runner, whose `fingerprint` is a constant.
#[test]
fn distinct_fingerprint_same_service_still_coalesces_to_one_incident() {
    let (registry, persistence) = fresh();
    // The distinctness must live on the CUE, which is what now reaches
    // `Incident.fingerprint`. Varying `L4Output.fingerprint` instead would make
    // this test vacuous — that field no longer flows into the incident.
    let digest_a = digest_with_fingerprinted_cue(
        CueKind::RetryStorm,
        CueScope::Service,
        Some("checkout-service"),
        Some(FINGERPRINT_A),
    );
    let digest_b = digest_with_fingerprinted_cue(
        CueKind::RetryStorm,
        CueScope::Service,
        Some("checkout-service"),
        Some(FINGERPRINT_B),
    );
    assert_ne!(
        FINGERPRINT_A, FINGERPRINT_B,
        "fixture must carry genuinely distinct fingerprints or the test is vacuous",
    );

    let first = l4_output(Decision::Surface, L4Severity::Suggested, "storm A");
    let second = l4_output(Decision::Surface, L4Severity::Suggested, "storm B");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest_a,
        &first,
        5_000,
    );
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest_b,
        &second,
        9_000,
    );

    assert_eq!(
        registry.count(),
        1,
        "a distinct-fingerprint storm on the same service coalesces by decision",
    );
    assert_eq!(
        persistence.save_count(),
        1,
        "the second storm re-emits rather than INSERTing",
    );
    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        inc.title, "Retry storm: storm A",
        "the FIRST incident survives; the second storm is absorbed into it",
    );
    assert_eq!(
        inc.fingerprint, FINGERPRINT_A,
        "the surviving incident keeps the FIRST fault's fingerprint",
    );
    assert_eq!(
        inc.updated_at_unix_nano, 9_000,
        "re-emission bumped updated_at"
    );
}

/// The producer writes the cue-borne L1 fingerprint — the anonymized grouping
/// hash `Incident.fingerprint` is contracted as, and the value the corpus-
/// retrieval `fingerprint_match` arm compares against — not the model-authored
/// `L4Output.fingerprint`.
#[test]
fn producer_writes_cue_fingerprint_not_the_model_authored_one() {
    let (registry, persistence) = fresh();
    let digest = digest_with_fingerprinted_cue(
        CueKind::RetryStorm,
        CueScope::Service,
        Some("checkout-service"),
        Some(FINGERPRINT_A),
    );
    let mut output = l4_output(Decision::Surface, L4Severity::Suggested, "storm");
    output.fingerprint = "model-authored-string".into();

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(inc.fingerprint, FINGERPRINT_A);
    assert_ne!(
        inc.fingerprint, output.fingerprint,
        "the model-authored string must not reach the field",
    );
    assert_eq!(
        inc.fingerprint.len(),
        32,
        "must be the full-width hash the assembler's hex_lower produces",
    );
    assert_eq!(
        persistence.saved().remove(0).fingerprint,
        FINGERPRINT_A,
        "the persisted row carries it too, not just the in-memory registry",
    );
}

/// Evidence grounding (chunk 2026-08-26 interpretation-brief-completeness):
/// `fingerprint_hashes` is the order-preserving union of the model's refs
/// and the cue's REAL fingerprint — the incident record carries a real id
/// regardless of model behavior, with parsed refs FIRST (the deterministic
/// P-073 contains-pins ride them).
#[test]
fn fingerprint_hashes_union_appends_cue_fingerprint_to_model_refs() {
    let (registry, persistence) = fresh();
    let digest = digest_with_fingerprinted_cue(
        CueKind::RetryStorm,
        CueScope::Service,
        Some("checkout-service"),
        Some(FINGERPRINT_A),
    );
    let output = l4_output(Decision::Surface, L4Severity::Suggested, "storm");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        inc.evidence_refs.fingerprint_hashes,
        vec![
            "fp-1".to_string(),
            "fp-2".to_string(),
            FINGERPRINT_A.to_string()
        ],
        "union appends the cue's real fingerprint after the model's refs",
    );
}

/// The dedup half of the union: a model that COPIED the cue fingerprint
/// (the intended post-fix behavior) must not produce a duplicate entry.
#[test]
fn fingerprint_hashes_union_does_not_duplicate_a_copied_fingerprint() {
    let (registry, persistence) = fresh();
    let digest = digest_with_fingerprinted_cue(
        CueKind::RetryStorm,
        CueScope::Service,
        Some("checkout-service"),
        Some(FINGERPRINT_A),
    );
    let mut output = l4_output(Decision::Surface, L4Severity::Suggested, "storm");
    output.evidence_refs = vec![FINGERPRINT_A.to_string()];

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        inc.evidence_refs.fingerprint_hashes,
        vec![FINGERPRINT_A.to_string()],
        "a copied fingerprint appears exactly once",
    );
}

/// The negative half of the pair: with NO cue fingerprint the union adds
/// nothing — the model's refs land unchanged, no phantom entry.
#[test]
fn fingerprint_hashes_carry_only_model_refs_without_a_cue_fingerprint() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc-a"));
    let output = l4_output(Decision::Surface, L4Severity::Suggested, "spike");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        inc.evidence_refs.fingerprint_hashes,
        vec!["fp-1".to_string(), "fp-2".to_string()],
    );
}

/// Incidents with no cue-borne fingerprint (reflection cadence; baseline
/// families) carry an EMPTY one, which both retrieval selectors' `is_empty`
/// guards drop to scope-only matching. Under the deterministic runner this is
/// what stops every incident matching every other as "previously seen".
#[test]
fn incidents_without_a_cue_fingerprint_carry_an_empty_one() {
    let (registry, persistence) = fresh();
    let output = l4_output(Decision::Surface, L4Severity::Suggested, "trend");

    let baseline = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc-a"));
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &baseline,
        &output,
        5_000,
    );

    let reflection = reflection_digest();
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &reflection,
        &output,
        6_000,
    );

    for inc in registry.list_active(WORKSPACE) {
        assert!(
            inc.fingerprint.is_empty(),
            "{:?} has no cue fingerprint, so the field must be empty rather than \
             the model-authored constant, or every such incident matches every other",
            inc.kind,
        );
    }
}

#[test]
fn distinct_services_create_distinct_incidents() {
    let (registry, persistence) = fresh();
    let d_a = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc-a"));
    let d_b = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc-b"));
    let output = l4_output(Decision::Surface, L4Severity::Suggested, "spike");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &d_a,
        &output,
        5_000,
    );
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &d_b,
        &output,
        6_000,
    );

    assert_eq!(
        registry.count(),
        2,
        "distinct scope_id → distinct incidents (per-service attribution)",
    );
    assert_eq!(persistence.save_count(), 2);
}

#[test]
fn pii_canary_in_l4_text_is_scrubbed_before_persist() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    // Email canary: scrub_attribute redacts the P-047 email category.
    let canary = "attacker@evil.example";
    let output = l4_output(
        Decision::Surface,
        L4Severity::Suggested,
        &format!("error referencing {canary} in title"),
    );
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let saved = persistence.saved();
    assert_eq!(saved.len(), 1);
    let persisted = &saved[0];
    assert!(
        !persisted.title.contains(canary),
        "email canary MUST be scrubbed before the corpus write; got title: {}",
        persisted.title,
    );
    assert_eq!(
        persisted.title, "Error-rate spike: error referencing [redacted: email] in title",
        "a title embedding PII must mask the secret in place and keep its words",
    );
    // The registry copy is the post-scrub incident (insert happens post-scrub).
    let active = registry.list_active(WORKSPACE);
    assert!(!active[0].title.contains(canary));
}

#[test]
fn producer_observability_is_aggregate_only() {
    let (registry, persistence) = fresh();
    let canary = "leak@evil.example";
    let digest = digest_with_cue(
        CueKind::ErrorRateSpike,
        CueScope::Service,
        Some("secret-svc"),
    );
    let output = l4_output(
        Decision::Surface,
        L4Severity::Autonomous,
        &format!("title with {canary}"),
    );

    let events = global_capture();
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let captured = events.lock().expect("lock").clone();
    // A process-global subscriber can also capture sibling tests' events
    // under parallel libtest, so assert THIS test's emission by its full
    // signature rather than find-first.
    assert!(
        captured.iter().any(|(t, fields, _)| {
            t == "interpretation.incident.created"
                && fields.contains("created=true")
                && fields.contains("severity=error")
                && fields.contains("priority_tier=autonomous")
        }),
        "producer outcome event present with the aggregate-only shape",
    );
    assert!(
        captured
            .iter()
            .any(|(t, _, _)| t == "metric.pipeline.l4.incidents_created_total"),
        "producer counter metric emitted",
    );
    for (target, fields, _) in &captured {
        assert!(
            !fields.contains(canary),
            "title canary leaked into self-observation target {target}: {fields}",
        );
        assert!(
            !fields.contains("secret-svc"),
            "scope_id leaked into self-observation target {target}: {fields}",
        );
    }
}

// ---- Reflection-incident producer coverage (chunk #98) ----

/// Build a reflection-cadence digest (30-minute window, no triggering cue).
/// The producer derives the synthetic workspace-global ReflectionTrend
/// identity for these instead of requiring a cue.
fn reflection_digest() -> Digest {
    let mut d = digest_without_cue();
    d.kind = DigestKind::Reflection;
    d
}

#[test]
fn reflection_surface_creates_workspace_global_incident() {
    let (registry, persistence) = fresh();
    let digest = reflection_digest();
    let output = l4_output(
        Decision::Surface,
        L4Severity::Curious,
        "cumulative latency drift across the window",
    );

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let active = registry.list_active(WORKSPACE);
    assert_eq!(
        active.len(),
        1,
        "a reflection digest with a surface decision creates one incident",
    );
    let inc = &active[0];
    assert_eq!(inc.kind, CueKind::ReflectionTrend);
    assert_eq!(inc.scope, CueScope::Global);
    assert_eq!(
        inc.scope_id, None,
        "reflection incidents are workspace-global, not service-attributed",
    );
    assert_eq!(
        inc.priority_tier,
        PriorityTier::Curious,
        "default-curious reflection incident",
    );
    assert_eq!(inc.severity, IncidentSeverity::Info, "Curious → Info hue");
    assert_eq!(inc.status, IncidentStatus::Active);
}

#[test]
fn reflection_high_confidence_surfaces_suggested() {
    // "unless the model identifies a high-confidence pattern warranting
    // Suggested or higher" — when the L4 output is Suggested, the producer
    // maps it through (the default-curious bias is prompt-enforced upstream,
    // not a producer-side clamp; symmetric with the acute path).
    let (registry, persistence) = fresh();
    let digest = reflection_digest();
    let output = l4_output(
        Decision::Surface,
        L4Severity::Suggested,
        "recurring saturation pattern",
    );
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].priority_tier, PriorityTier::Suggested);
    assert_eq!(active[0].kind, CueKind::ReflectionTrend);
}

#[test]
fn reflection_dedups_one_per_workspace() {
    let (registry, persistence) = fresh();
    let digest = reflection_digest();
    let output = l4_output(Decision::Surface, L4Severity::Curious, "trend");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        9_000,
    );
    assert_eq!(
        registry.count(),
        1,
        "reflection trends dedup one-per-workspace on (ReflectionTrend, Global, None)",
    );
    assert_eq!(persistence.save_count(), 1, "only the first call INSERTs");
    assert!(
        persistence.update_count() >= 1,
        "the second reflection digest drives a reemission update",
    );
    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        inc.updated_at_unix_nano, 9_000,
        "reemission bumped updated_at"
    );
}

#[test]
fn reflection_dismiss_creates_no_incident() {
    let (registry, persistence) = fresh();
    let digest = reflection_digest();
    let output = l4_output(Decision::Dismiss, L4Severity::None, "no trend");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(
        registry.count(),
        0,
        "a dismissed reflection digest creates no incident",
    );
}

/// One captured event: `(target, concatenated-fields, emitting thread)`.
type Captured = Arc<Mutex<Vec<(String, String, ThreadId)>>>;

/// The process-global capture, installed once. Process-global, NOT the
/// thread-local `with_default`: sibling tests exercise the same tracing
/// callsites in parallel, and under parallel libtest the thread-local form
/// races the callsite interest cache — the capture comes back empty on
/// exactly the event under assertion (testing.md 2026-06-28). Each event
/// carries its thread so a test can count only its own emissions.
fn global_capture() -> Captured {
    static CAPTURE: OnceLock<Captured> = OnceLock::new();
    Arc::clone(CAPTURE.get_or_init(|| {
        let (subscriber, events) = CapturingSubscriber::new();
        tracing::subscriber::set_global_default(subscriber)
            .expect("the capture is the binary's only global subscriber");
        events
    }))
}

/// Minimal field-capturing `tracing::Subscriber` (sync; driven via
/// `with_default`). Mirrors the CLAUDE.md testing.md 2026-05-11
/// FieldCollector pattern without adding a `tracing-test` dep.
struct CapturingSubscriber {
    events: Captured,
}

impl CapturingSubscriber {
    fn new() -> (Self, Captured) {
        let events: Captured = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                events: Arc::clone(&events),
            },
            events,
        )
    }
}

impl tracing::Subscriber for CapturingSubscriber {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }
    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
    fn event(&self, event: &tracing::Event<'_>) {
        let mut fields = String::new();
        struct V<'a>(&'a mut String);
        impl<'a> tracing::field::Visit for V<'a> {
            fn record_debug(&mut self, f: &tracing::field::Field, v: &dyn std::fmt::Debug) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={:?}", f.name(), v);
            }
            fn record_str(&mut self, f: &tracing::field::Field, v: &str) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", f.name(), v);
            }
            fn record_bool(&mut self, f: &tracing::field::Field, v: bool) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", f.name(), v);
            }
            fn record_u64(&mut self, f: &tracing::field::Field, v: u64) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", f.name(), v);
            }
            fn record_i64(&mut self, f: &tracing::field::Field, v: i64) {
                use std::fmt::Write;
                let _ = write!(self.0, " {}={}", f.name(), v);
            }
        }
        let mut visitor = V(&mut fields);
        event.record(&mut visitor);
        self.events.lock().expect("lock").push((
            event.metadata().target().to_string(),
            fields,
            std::thread::current().id(),
        ));
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}

/// Det-L4 blind-spot pin (b): the producer hardcodes the three
/// non-fingerprint `EvidenceRefs` fields empty in EVERY mode, so the MCP
/// slice's `span_refs` / `timestamps_unix_nano` are permanently empty in
/// production and an absence check over them passes for the wrong reason
/// (arch §Occupied Resources → `ANDROMEDA_PULSE_L4_DETERMINISTIC`). This pin
/// records that emptiness as BY CONSTRUCTION: the chunk that populates the
/// producer must flip it and rewrite those vacuous absence checks. The
/// populated `fingerprint_hashes` union is the in-test selectivity control
/// proving the pin reads the produced incident.
#[test]
fn producer_evidence_trace_span_and_timestamps_are_empty_by_construction() {
    let (registry, persistence) = fresh();
    let digest = digest_with_fingerprinted_cue(
        CueKind::ErrorRateSpike,
        CueScope::Service,
        Some("payment-service"),
        Some(FINGERPRINT_A),
    );
    let parsed: L4Output =
        serde_json::from_str(CANNED_L4_OUTPUT_JSON).expect("canned deterministic output parses");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &parsed,
        5_000,
    );

    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        inc.evidence_refs.trace_id, None,
        "no trace-id source exists at this seam in any mode",
    );
    assert_eq!(
        inc.evidence_refs.span_ids,
        Vec::<[u8; 8]>::new(),
        "span ids are not threaded by any mode",
    );
    assert_eq!(
        inc.evidence_refs.timestamps_unix_nano,
        Vec::<i64>::new(),
        "timestamps are not threaded by any mode",
    );
    assert_eq!(
        inc.evidence_refs.fingerprint_hashes,
        vec![
            "det-span-9f2c4a7e1b6d0358".to_string(),
            "det-template-0007".to_string(),
            "det-fingerprint-4a7f2b91c6e05d3849b1e7a2c5f08d63".to_string(),
            FINGERPRINT_A.to_string(),
        ],
        "selectivity control: the canned refs land FIRST, then the cue's real \
         fingerprint — the populated union proves this pin is live",
    );
}

// ---- No-incident outcome observability ----

const SKIPPED: &str = "interpretation.incident.skipped";

/// This thread's `interpretation.incident.skipped` records, fields only.
fn own_skip_records(events: &Captured) -> Vec<String> {
    let me = std::thread::current().id();
    events
        .lock()
        .expect("lock")
        .iter()
        .filter(|(t, _, thread)| t == SKIPPED && *thread == me)
        .map(|(_, fields, _)| fields.clone())
        .collect()
}

fn assert_one_skip(events: &Captured, reason: &str, decision: &str, severity: &str, kind: &str) {
    let records = own_skip_records(events);
    assert_eq!(
        records.len(),
        1,
        "exactly one skip record per no-incident generation: {records:?}",
    );
    let fields = &records[0];
    for expected in [
        format!("skip_reason={reason}"),
        format!("decision={decision}"),
        format!("severity={severity}"),
        format!("digest_kind={kind}"),
    ] {
        assert!(
            fields.contains(&expected),
            "skip record lacks `{expected}`: {fields}"
        );
    }
}

#[test]
fn incident_skip_records_a_dismiss_decision() {
    let events = global_capture();
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let output = l4_output(Decision::Dismiss, L4Severity::Suggested, "noise");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 0);
    assert_one_skip(
        &events,
        "decision_dismiss",
        "dismiss",
        "suggested",
        "cadence_tier3",
    );
}

#[test]
fn incident_skip_records_a_none_severity() {
    let events = global_capture();
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let output = l4_output(Decision::Surface, L4Severity::None, "ambiguous");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 0);
    assert_one_skip(&events, "severity_none", "surface", "none", "cadence_tier3");
}

#[test]
fn incident_skip_records_a_model_resolution_summary_at_the_predicate() {
    let events = global_capture();
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let mut output = l4_output(Decision::Surface, L4Severity::Autonomous, "resolved");
    output.is_resolution_summary = true;
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 0);
    assert_one_skip(
        &events,
        "model_resolution_summary",
        "surface",
        "autonomous",
        "cadence_tier3",
    );
}

#[test]
fn incident_skip_records_a_cue_less_digest() {
    let events = global_capture();
    let (registry, persistence) = fresh();
    let output = l4_output(Decision::Surface, L4Severity::Autonomous, "no cue");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest_without_cue(),
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 0);
    assert_one_skip(&events, "no_cue", "surface", "autonomous", "cadence_tier3");
}

#[test]
fn incident_skip_is_absent_when_an_incident_is_created() {
    let events = global_capture();
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let output = l4_output(Decision::Surface, L4Severity::Autonomous, "real");
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );
    assert_eq!(registry.count(), 1);
    assert!(
        own_skip_records(&events).is_empty(),
        "a created incident must leave no skip record",
    );
}

/// Constant-output runner for the `process_digest` routing seam.
struct CannedRunner {
    json: String,
}

impl LlmInferenceRunner for CannedRunner {
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
        let out = self.json.clone();
        Pin::from(Box::new(async move { Ok(out) }))
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

fn resolution_summary_runner() -> CannedRunner {
    let mut parsed: L4Output =
        serde_json::from_str(CANNED_L4_OUTPUT_JSON).expect("canned deterministic output parses");
    parsed.is_resolution_summary = true;
    CannedRunner {
        json: serde_json::to_string(&parsed).expect("serializes"),
    }
}

#[tokio::test]
async fn incident_skip_records_a_model_resolution_summary_on_a_storm_digest() {
    let events = global_capture();
    let (registry, persistence) = fresh();
    let mut digest = digest_with_cue(CueKind::RetryStorm, CueScope::Service, Some("svc"));
    digest.kind = DigestKind::CadenceTier1;
    let outcome = process_digest(
        &resolution_summary_runner(),
        &NeverBackoff,
        registry.as_ref(),
        persistence.as_ref(),
        &GenerationDamper::new(),
        &digest,
        5_000,
    )
    .await;
    assert!(matches!(
        outcome,
        Some(pulse_app::inference_runtime::L4DigestOutcome::Success(_))
    ));
    assert_eq!(registry.count(), 0);
    let records = own_skip_records(&events);
    assert_eq!(records.len(), 1, "one skip record: {records:?}");
    assert!(
        records[0].contains("skip_reason=model_resolution_summary")
            && records[0].contains("digest_kind=cadence_tier1"),
        "the routing seam names the model-set flag: {}",
        records[0],
    );
}

#[tokio::test]
async fn incident_skip_is_absent_for_a_resolution_summary_digest() {
    let events = global_capture();
    let (registry, persistence) = fresh();
    let mut digest = digest_with_cue(CueKind::RetryStorm, CueScope::Service, Some("svc"));
    digest.kind = DigestKind::ResolutionSummary;
    let _ = process_digest(
        &resolution_summary_runner(),
        &NeverBackoff,
        registry.as_ref(),
        persistence.as_ref(),
        &GenerationDamper::new(),
        &digest,
        5_000,
    )
    .await;
    assert!(
        own_skip_records(&events).is_empty(),
        "a resolution-summary digest attaching its summary is not a skipped incident",
    );
}

// ---- Cue-grounded incident title (the cause names itself) ----

/// The `title` inside the incident's persisted L4 JSON — what the report
/// header renders.
fn json_title(incident: &Incident) -> String {
    let text = incident
        .resolution_summary_text
        .as_deref()
        .expect("interpretation JSON attached");
    serde_json::from_str::<L4Output>(text)
        .expect("attached JSON parses back as L4Output")
        .title
}

#[test]
fn incident_title_names_its_cause_for_a_retry_storm() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::RetryStorm, CueScope::Service, Some("conductor"));
    let output = l4_output(
        Decision::Surface,
        L4Severity::Autonomous,
        "Error Rate Spike in ws",
    );

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let expected = "Retry storm: Error Rate Spike in ws";
    let saved = persistence.saved();
    assert_eq!(saved.len(), 1);
    assert_eq!(saved[0].title, expected, "persisted title names the cause");
    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(inc.title, expected, "registry title names the cause");
    assert_eq!(
        json_title(&inc),
        expected,
        "the report's L4 JSON title names the cause",
    );
}

#[test]
fn incident_title_names_its_cause_without_retry_for_an_error_rate_spike() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::ErrorRateSpike, CueScope::Service, Some("svc"));
    let output = l4_output(Decision::Surface, L4Severity::Suggested, "Errors climbing");

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let inc = registry.list_active(WORKSPACE).remove(0);
    for title in [inc.title.clone(), json_title(&inc)] {
        assert_eq!(title, "Error-rate spike: Errors climbing");
        assert!(
            !title.to_lowercase().contains("retry"),
            "a non-retry cause never names the retry: {title}",
        );
    }
}

#[test]
fn incident_title_names_its_cause_on_dedupe_refresh() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::RetryStorm, CueScope::Service, Some("conductor"));

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &l4_output(Decision::Surface, L4Severity::Suggested, "first take"),
        5_000,
    );
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &l4_output(Decision::Surface, L4Severity::Suggested, "second take"),
        9_000,
    );

    assert_eq!(registry.count(), 1, "the second generation dedups");
    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(
        json_title(&inc),
        "Retry storm: second take",
        "the refreshed JSON carries the grounded SECOND model title",
    );
    assert!(
        inc.title.ends_with("first take") && !inc.title.contains("second take"),
        "a deduped incident keeps its creation title: {}",
        inc.title,
    );
}

#[test]
fn incident_title_names_its_cause_in_the_resolution_summary() {
    let (registry, persistence) = fresh();
    let digest = digest_with_cue(CueKind::RetryStorm, CueScope::Service, Some("conductor"));
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &l4_output(Decision::Surface, L4Severity::Suggested, "live take"),
        5_000,
    );
    let id = registry.list_active(WORKSPACE).remove(0).id;
    registry
        .mark_resolved(id, 7_000, ResolutionTrigger::AutoResolve)
        .expect("resolve");

    let mut resolution = l4_output(Decision::Surface, L4Severity::Suggested, "storm subsided");
    resolution.is_resolution_summary = true;
    attach_resolution_summary_to_incident(
        registry.as_ref(),
        persistence.as_ref(),
        &[id.to_string()],
        &resolution,
        8_000,
    );

    let inc = registry.get(id).expect("incident");
    assert_eq!(
        json_title(&inc),
        "Retry storm: storm subsided",
        "the resolution final write keeps the cause in the report header",
    );
}

#[test]
fn incident_title_names_its_cause_for_a_reflection_digest() {
    let (registry, persistence) = fresh();
    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &reflection_digest(),
        &l4_output(Decision::Surface, L4Severity::Curious, "slow drift"),
        5_000,
    );

    let inc = registry.list_active(WORKSPACE).remove(0);
    assert_eq!(inc.title, "Reflection trend: slow drift");
    assert_eq!(json_title(&inc), "Reflection trend: slow drift");
}

#[test]
fn incident_title_names_its_cause_and_still_masks_secrets() {
    let (registry, persistence) = fresh();
    let bare = "sk_live_51NotARealKeyOnlyForPulseTests00"; // gitleaks:allow
    let keyed = "hunter2";
    let digest = digest_with_cue(CueKind::RetryStorm, CueScope::Service, Some("conductor"));
    let output = l4_output(
        Decision::Surface,
        L4Severity::Suggested,
        &format!("rotate {bare} after password={keyed}"),
    );

    create_incident_from_l4_output(
        registry.as_ref(),
        persistence.as_ref(),
        &digest,
        &output,
        5_000,
    );

    let saved = persistence.saved();
    assert_eq!(saved.len(), 1);
    let inc = registry.list_active(WORKSPACE).remove(0);
    for title in [saved[0].title.clone(), inc.title.clone(), json_title(&inc)] {
        assert!(
            title.starts_with("Retry storm: "),
            "the cause label survives the scrub: {title}",
        );
        assert!(!title.contains(bare), "bare canary masked: {title}");
        assert!(!title.contains(keyed), "keyed canary masked: {title}");
    }
    let raw_json = inc.resolution_summary_text.as_deref().expect("json");
    assert!(!raw_json.contains(bare) && !raw_json.contains(keyed));
}
