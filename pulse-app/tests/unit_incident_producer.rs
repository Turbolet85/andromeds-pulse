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

use std::sync::{Arc, Mutex};

use interpretation::schema::{Decision, L4Output, Severity as L4Severity};
use pulse_app::inference_runtime::create_incident_from_l4_output;
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, InMemoryIncidentRegistry,
    Incident, IncidentError, IncidentPersistence, IncidentRegistry, IncidentStatus, PriorityTier,
    Severity as IncidentSeverity,
};

const WORKSPACE: &str = "/home/dev/example";

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
/// `(kind, scope, scope_id)` incident identity from it).
fn digest_with_cue(kind: CueKind, scope: CueScope, scope_id: Option<&str>) -> Digest {
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
    let digest = digest_with_cue(
        CueKind::ErrorRateSpike,
        CueScope::Service,
        Some("auth-service"),
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
    assert_eq!(inc.fingerprint, "incident-fp");
    assert_eq!(inc.evidence_refs.fingerprint_hashes, vec!["fp-1", "fp-2"]);
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
    assert!(
        persisted.title.starts_with("[redacted:"),
        "a title embedding PII must collapse to a category marker; got: {}",
        persisted.title,
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

    let (subscriber, events) = CapturingSubscriber::new();
    tracing::subscriber::with_default(subscriber, || {
        create_incident_from_l4_output(
            registry.as_ref(),
            persistence.as_ref(),
            &digest,
            &output,
            5_000,
        );
    });

    let captured = events.lock().expect("lock").clone();
    let created_evt = captured
        .iter()
        .find(|(t, _)| t == "interpretation.incident.created")
        .expect("producer outcome event present");
    assert!(created_evt.1.contains("created=true"));
    assert!(created_evt.1.contains("severity=error"));
    assert!(created_evt.1.contains("priority_tier=autonomous"));
    assert!(
        captured
            .iter()
            .any(|(t, _)| t == "metric.pipeline.l4.incidents_created_total"),
        "producer counter metric emitted",
    );
    for (target, fields) in &captured {
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

/// One captured event: `(target, concatenated-fields)`.
type Captured = Arc<Mutex<Vec<(String, String)>>>;

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
        self.events
            .lock()
            .expect("lock")
            .push((event.metadata().target().to_string(), fields));
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}
