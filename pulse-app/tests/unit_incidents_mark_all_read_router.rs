//! Integration tests для `pulse-app/src/incidents_router.rs`
//! `incidents.mark_all_read()` TauRPC procedure (chunk #87).
//!
//! Tests the resolver-level contract:
//! - Empty workspace returns affected_count=0
//! - Multi-incident bulk: mark_all_read transitions every unread Active /
//!   Acknowledged incident в the workspace, returns affected_count=N
//! - Idempotent re-invocation returns 0 (all already read)
//! - Serde round-trip of MarkAllReadPayload (specta::Type)
//! - PII negative-canary grep on serialized payload
//! - Persists changes through `IncidentPersistence::update_incident_status`

use std::sync::Arc;

use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use pulse_app::incidents_router::{IncidentsApi, IncidentsApiImpl, MarkAllReadPayload};
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, InMemoryIncidentRegistry, Incident,
    IncidentLifecycleBroadcast, IncidentPersistence, IncidentRegistry, IncidentStatus,
    PriorityTier, Severity,
};

const WORKSPACE: &str = "ws-test";

fn sample_incident(id: i64, workspace: &str, kind: CueKind, ts: i64) -> Incident {
    Incident {
        id,
        workspace: workspace.to_string(),
        fingerprint: format!("fp-{kind:?}"),
        title: "[redacted] sample incident".to_string(),
        detail: "[redacted] sample detail".to_string(),
        kind,
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

fn make_impl(
    registry: Arc<dyn IncidentRegistry>,
    persistence: Arc<dyn IncidentPersistence>,
) -> IncidentsApiImpl {
    IncidentsApiImpl::new(
        registry,
        Arc::new(IncidentLifecycleBroadcast::new()),
        persistence,
        WORKSPACE.to_string(),
    )
}

fn make_persistence_and_registry() -> (Arc<dyn IncidentPersistence>, Arc<dyn IncidentRegistry>) {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus) as Arc<dyn CorpusWriter>;
    let persistence: Arc<dyn IncidentPersistence> =
        Arc::new(CorpusIncidentPersistence::new(writer));
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    (persistence, registry)
}

#[tokio::test]
async fn mark_all_read_on_empty_workspace_returns_zero_affected() {
    let (persistence, registry) = make_persistence_and_registry();
    let api = make_impl(registry, persistence);
    let payload = api.mark_all_read().await.expect("infallible");
    assert_eq!(payload.affected_count, 0);
    assert!(payload.marked_at_unix_nano > 0);
}

#[tokio::test]
async fn mark_all_read_marks_all_unread_and_returns_affected_count() {
    let (persistence, registry) = make_persistence_and_registry();
    for id in 1..=3 {
        registry.insert(sample_incident(
            id,
            WORKSPACE,
            CueKind::ErrorRateSpike,
            1_000,
        ));
    }
    let api = make_impl(Arc::clone(&registry), persistence);
    let payload = api.mark_all_read().await.expect("infallible");
    assert_eq!(payload.affected_count, 3);
    for id in 1..=3 {
        let inc = registry.get(id).expect("present");
        assert!(inc.read_at_unix_nano.is_some());
    }
}

#[tokio::test]
async fn mark_all_read_is_idempotent_when_all_already_read() {
    let (persistence, registry) = make_persistence_and_registry();
    for id in 1..=2 {
        registry.insert(sample_incident(
            id,
            WORKSPACE,
            CueKind::ErrorRateSpike,
            1_000,
        ));
    }
    let api = make_impl(Arc::clone(&registry), persistence);
    let first = api.clone().mark_all_read().await.expect("first");
    assert_eq!(first.affected_count, 2);
    let second = api.mark_all_read().await.expect("second");
    assert_eq!(
        second.affected_count, 0,
        "all already read; nothing to mark"
    );
}

#[tokio::test]
async fn mark_all_read_filters_by_workspace() {
    let (persistence, registry) = make_persistence_and_registry();
    registry.insert(sample_incident(
        1,
        WORKSPACE,
        CueKind::ErrorRateSpike,
        1_000,
    ));
    registry.insert(sample_incident(
        2,
        "other-ws",
        CueKind::ErrorRateSpike,
        1_000,
    ));
    let api = make_impl(Arc::clone(&registry), persistence);
    let payload = api.mark_all_read().await.expect("infallible");
    assert_eq!(payload.affected_count, 1, "only WORKSPACE rows touched");
    let other = registry.get(2).expect("present");
    assert!(
        other.read_at_unix_nano.is_none(),
        "other workspace untouched"
    );
}

#[tokio::test]
async fn mark_all_read_skips_resolved_incidents() {
    let (persistence, registry) = make_persistence_and_registry();
    let mut resolved = sample_incident(1, WORKSPACE, CueKind::ErrorRateSpike, 1_000);
    resolved.status = IncidentStatus::Resolved;
    resolved.resolved_at_unix_nano = Some(2_000);
    registry.insert(resolved);
    registry.insert(sample_incident(
        2,
        WORKSPACE,
        CueKind::LatencyRegression,
        1_000,
    ));
    let api = make_impl(Arc::clone(&registry), persistence);
    let payload = api.mark_all_read().await.expect("infallible");
    assert_eq!(payload.affected_count, 1, "Resolved excluded from bulk");
    let resolved_loaded = registry.get(1).expect("present");
    assert!(resolved_loaded.read_at_unix_nano.is_none());
}

#[tokio::test]
async fn mark_all_read_persists_read_state_to_corpus() {
    let (persistence, registry) = make_persistence_and_registry();
    for id in 1..=3 {
        let inc = sample_incident(id, WORKSPACE, CueKind::ErrorRateSpike, 1_000);
        let row_id = persistence.save_new_incident(&inc).expect("seed save");
        let mut hydrated = inc.clone();
        hydrated.id = row_id;
        registry.insert(hydrated);
    }
    let api = make_impl(Arc::clone(&registry), Arc::clone(&persistence));
    let payload = api.mark_all_read().await.expect("infallible");
    assert_eq!(payload.affected_count, 3);
    let count_after = persistence
        .count_active_unread(WORKSPACE)
        .expect("count_active_unread");
    assert_eq!(
        count_after, 0,
        "P-045 SQL counts unread rows; all 3 marked read"
    );
}

#[test]
fn mark_all_read_payload_serializes_through_serde_json() {
    let payload = MarkAllReadPayload {
        affected_count: 5,
        marked_at_unix_nano: 1_700_000_000_000_000_000,
    };
    let json = serde_json::to_string(&payload).expect("serialize");
    let roundtrip: MarkAllReadPayload = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, payload);
}

#[test]
fn mark_all_read_payload_excludes_pii_fields_in_serialization() {
    let payload = MarkAllReadPayload {
        affected_count: 5,
        marked_at_unix_nano: 1_700_000_000_000_000_000,
    };
    let json = serde_json::to_string(&payload).expect("serialize");
    for banned in [
        "incident_id",
        "incident_ids",
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "workspace",
        "title",
        "detail",
        "fingerprint",
    ] {
        assert!(
            !json.contains(banned),
            "MarkAllReadPayload MUST NOT carry PII field substring `{banned}` (got: {json})",
        );
    }
}
