//! The persist-vs-resolve write race, reproduced deterministically.
//!
//! `run_incident_persist_cycle` snapshots `list_active` then writes each row;
//! a resolution landing inside that write span used to be overwritten with the
//! snapshot's stale state, and because the cycle lists actives only, the
//! reverted row never re-entered a snapshot — a permanent zombie-active.
//!
//! Determinism comes from a seam, not from timing: the cycle is a synchronous
//! `pub fn` and `AutoResolveObserver::run_one_tick` takes its instant as an
//! argument, so an interleaving persistence wrapper can fire the resolution
//! precisely inside the cycle's write span. No `sleep`, no interval, no clock.

use std::sync::{Arc, Mutex};

use corpus::contract::{
    Corpus, CorpusWriter, FakeKeychainBackend, IncidentWriteOutcome as CorpusWriteOutcome,
    KeychainBackend,
};
use pulse_app::incident_observer::AutoResolveObserver;
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, INCIDENT_PERSISTENCE_KIND, InMemoryIncidentRegistry, Incident,
    IncidentError, IncidentLifecycleBroadcast, IncidentPersistence, IncidentRegistry,
    IncidentStatus, IncidentWriteOutcome, PriorityTier, ResolutionTrigger, Severity,
    run_incident_persist_cycle,
};

const WS: &str = "ws-write-guard";

fn build_corpus() -> Arc<dyn CorpusWriter> {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    Arc::new(corpus)
}

fn sample_incident(ts: i64) -> Incident {
    Incident {
        id: 0,
        workspace: WS.to_string(),
        fingerprint: "fp-write-guard".to_string(),
        title: "[redacted] write-guard incident".to_string(),
        detail: "[redacted] write-guard detail".to_string(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: None,
        status: IncidentStatus::Active,
        severity: Severity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: vec![],
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: ts,
        updated_at_unix_nano: ts,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

/// Wraps the real adapter and, on the FIRST `update_incident_status` call,
/// runs the auto-resolve observer BEFORE delegating — placing the resolution
/// inside the persist cycle's write span, which is the race verbatim.
struct InterleavingPersistence {
    inner: Arc<dyn IncidentPersistence>,
    observer: Mutex<Option<AutoResolveObserver>>,
    resolve_at_unix_nano: i64,
}

impl IncidentPersistence for InterleavingPersistence {
    fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError> {
        self.inner.save_new_incident(incident)
    }

    fn update_incident_status(
        &self,
        id: i64,
        payload: &Incident,
    ) -> Result<IncidentWriteOutcome, IncidentError> {
        if let Some(observer) = self.observer.lock().expect("lock").take() {
            observer.run_one_tick(self.resolve_at_unix_nano);
        }
        self.inner.update_incident_status(id, payload)
    }

    fn mark_read(&self, id: i64, read_unix_nano: i64) -> Result<(), IncidentError> {
        self.inner.mark_read(id, read_unix_nano)
    }

    fn load_active_incidents(&self, workspace: &str) -> Result<Vec<Incident>, IncidentError> {
        self.inner.load_active_incidents(workspace)
    }

    fn load_incidents_for_workspace_since(
        &self,
        workspace: &str,
        since_unix_nano: i64,
    ) -> Result<Vec<Incident>, IncidentError> {
        self.inner
            .load_incidents_for_workspace_since(workspace, since_unix_nano)
    }

    fn count_active_unread(&self, workspace: &str) -> Result<u64, IncidentError> {
        self.inner.count_active_unread(workspace)
    }

    fn save_incident_event(
        &self,
        incident_id: i64,
        event_kind: &str,
        occurred_at: i64,
    ) -> Result<(), IncidentError> {
        self.inner
            .save_incident_event(incident_id, event_kind, occurred_at)
    }
}

#[test]
fn a_resolution_landing_inside_the_persist_write_span_survives() {
    let writer = build_corpus();
    let inner: Arc<dyn IncidentPersistence> =
        Arc::new(CorpusIncidentPersistence::new(Arc::clone(&writer)));
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let broadcast = Arc::new(IncidentLifecycleBroadcast::new());

    let opened_at = 1_000_000_000_i64;
    let mut incident = sample_incident(opened_at);
    let id = inner.save_new_incident(&incident).expect("seed");
    incident.id = id;
    registry.insert(incident);

    // Far enough past the 120s no-reemission window that the observer resolves.
    let resolve_at = opened_at + 600_000_000_000;
    let interleaving = InterleavingPersistence {
        inner: Arc::clone(&inner),
        observer: Mutex::new(Some(AutoResolveObserver::new(
            Arc::clone(&registry),
            Arc::clone(&inner),
            Arc::clone(&broadcast),
        ))),
        resolve_at_unix_nano: resolve_at,
    };

    run_incident_persist_cycle(
        registry.as_ref(),
        &interleaving,
        INCIDENT_PERSISTENCE_KIND,
        &[WS.to_string()],
    )
    .expect("cycle ok");

    let row = writer
        .load_incident_by_id(id)
        .expect("load")
        .expect("row present");
    assert_eq!(
        row.status, "resolved",
        "the stale snapshot must not revert a resolution written inside its write span"
    );
    assert_eq!(row.resolved_unix_nano, Some(resolve_at));

    // And the row must not be a zombie for the readers that filter on status.
    let actives = writer.load_active_incidents(WS).expect("load actives");
    assert!(
        actives.is_empty(),
        "a resolved incident must not read back as active"
    );
}

#[test]
fn an_externally_resolved_row_is_not_reverted_by_the_next_persist_cycle() {
    // The MCP sidecar's shape: it resolves the row in the corpus directly and
    // the app's registry never learns of it (the registry is only hydrated at
    // boot), so the very next persist cycle replays the row as Active.
    let writer = build_corpus();
    let persistence: Arc<dyn IncidentPersistence> =
        Arc::new(CorpusIncidentPersistence::new(Arc::clone(&writer)));
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());

    let opened_at = 2_000_000_000_i64;
    let mut incident = sample_incident(opened_at);
    let id = persistence.save_new_incident(&incident).expect("seed");
    incident.id = id;
    registry.insert(incident);

    let resolved_at = opened_at + 60_000_000_000;
    let outcome = writer
        .update_incident_status(id, "resolved", resolved_at, Some(resolved_at), b"external")
        .expect("external resolve");
    assert_eq!(outcome, CorpusWriteOutcome::Applied);

    run_incident_persist_cycle(
        registry.as_ref(),
        persistence.as_ref(),
        INCIDENT_PERSISTENCE_KIND,
        &[WS.to_string()],
    )
    .expect("cycle ok");

    let row = writer
        .load_incident_by_id(id)
        .expect("load")
        .expect("row present");
    assert_eq!(
        row.status, "resolved",
        "an externally-resolved row must survive the app's next persist cycle"
    );
}

#[test]
fn a_fresh_write_still_applies_through_the_adapter() {
    // The accept arm: the guard must not brick ordinary progress.
    let writer = build_corpus();
    let persistence: Arc<dyn IncidentPersistence> =
        Arc::new(CorpusIncidentPersistence::new(Arc::clone(&writer)));
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());

    let opened_at = 3_000_000_000_i64;
    let mut incident = sample_incident(opened_at);
    let id = persistence.save_new_incident(&incident).expect("seed");
    incident.id = id;
    registry.insert(incident);

    let updated = registry
        .mark_resolved(
            id,
            opened_at + 120_000_000_000,
            ResolutionTrigger::ExplicitResolve,
        )
        .expect("resolve");
    let outcome = persistence
        .update_incident_status(id, &updated)
        .expect("write");

    assert_eq!(outcome, IncidentWriteOutcome::Applied);
}

#[test]
fn a_stale_write_declines_rather_than_erroring() {
    // A decline is a value, not a fault — the five call sites that inspect only
    // the `Err` arm depend on this, or every decline would log as an error.
    let writer = build_corpus();
    let persistence: Arc<dyn IncidentPersistence> =
        Arc::new(CorpusIncidentPersistence::new(Arc::clone(&writer)));

    let opened_at = 4_000_000_000_i64;
    let mut incident = sample_incident(opened_at);
    let id = persistence.save_new_incident(&incident).expect("seed");
    incident.id = id;

    writer
        .update_incident_status(
            id,
            "resolved",
            opened_at + 500_000_000_000,
            Some(opened_at + 500_000_000_000),
            b"newer",
        )
        .expect("newer write");

    // `incident` still carries the ORIGINAL `updated_at_unix_nano` — the stale
    // snapshot the persist cycle would replay.
    let outcome = persistence
        .update_incident_status(id, &incident)
        .expect("stale write must not error");

    assert_eq!(outcome, IncidentWriteOutcome::DeclinedStale);
}
