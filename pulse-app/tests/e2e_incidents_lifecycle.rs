//! E2E tests for incident lifecycle resolver + auto-resolution observer
//! — chunk #78. Exercises the full Active → Acknowledged → Resolved
//! lifecycle with broadcast assertions + 5-min cool-down + 120s
//! auto-resolution per capability spec P-022 + P-023.

use std::sync::Arc;

use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use pulse_app::incident_observer::AutoResolveObserver;
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use pulse_app::incidents_router::{IncidentsApi, IncidentsApiImpl};
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, InMemoryIncidentRegistry, Incident,
    IncidentLifecycleBroadcast, IncidentPersistence, IncidentRegistry, IncidentStatus,
    PriorityTier, ResolutionTrigger, Severity,
};
use ui_bridge::contract::AppError;

const WS: &str = "ws-e2e";

fn build_stack() -> (
    Arc<dyn IncidentRegistry>,
    Arc<dyn IncidentPersistence>,
    Arc<IncidentLifecycleBroadcast>,
    IncidentsApiImpl,
) {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let persistence: Arc<dyn IncidentPersistence> =
        Arc::new(CorpusIncidentPersistence::new(writer));
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let broadcast = Arc::new(IncidentLifecycleBroadcast::new());
    let resolver = IncidentsApiImpl::new(
        Arc::clone(&registry),
        Arc::clone(&broadcast),
        Arc::clone(&persistence),
        WS.to_string(),
    );
    (registry, persistence, broadcast, resolver)
}

fn seed_incident(
    persistence: &dyn IncidentPersistence,
    registry: &dyn IncidentRegistry,
    kind: CueKind,
    ts: i64,
) -> i64 {
    let mut inc = Incident {
        id: 0,
        workspace: WS.to_string(),
        fingerprint: format!("fp-{kind:?}"),
        title: "[redacted] e2e incident".to_string(),
        detail: "[redacted] e2e detail".to_string(),
        kind,
        scope: CueScope::Service,
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
    };
    let id = persistence.save_new_incident(&inc).expect("save");
    inc.id = id;
    registry.insert(inc);
    id
}

#[tokio::test]
async fn create_to_resolved_via_resolver_returns_empty_active_list() {
    let (registry, persistence, _broadcast, resolver) = build_stack();
    let id = seed_incident(
        persistence.as_ref(),
        registry.as_ref(),
        CueKind::ErrorRateSpike,
        1_000,
    );
    // list_active returns 1
    let list = resolver.clone().list_active().await.expect("list");
    assert_eq!(list.total, 1);
    assert_eq!(list.items[0].id, id);
    // acknowledge succeeds
    resolver.clone().acknowledge(id).await.expect("ack");
    let after_ack = resolver.clone().list_active().await.expect("list");
    assert_eq!(after_ack.total, 1, "acknowledged still в active list");
    assert_eq!(after_ack.items[0].status, IncidentStatus::Acknowledged);
    // mark_resolved succeeds
    resolver.clone().mark_resolved(id).await.expect("resolve");
    let after_resolve = resolver.clone().list_active().await.expect("list");
    assert_eq!(after_resolve.total, 0, "Resolved excluded from active list");
}

#[tokio::test]
async fn acknowledge_not_found_returns_app_error_notfound() {
    let (_registry, _persistence, _broadcast, resolver) = build_stack();
    let result = resolver.acknowledge(99999).await;
    match result {
        Err(AppError::NotFound { resource }) => {
            assert_eq!(resource, "incident");
        }
        other => panic!("expected NotFound; got {other:?}"),
    }
}

#[tokio::test]
async fn mark_resolved_not_found_returns_app_error_notfound() {
    let (_registry, _persistence, _broadcast, resolver) = build_stack();
    let result = resolver.mark_resolved(99999).await;
    match result {
        Err(AppError::NotFound { resource }) => {
            assert_eq!(resource, "incident");
        }
        other => panic!("expected NotFound; got {other:?}"),
    }
}

#[tokio::test]
async fn broadcast_emits_bounded_payload_no_pii() {
    let (registry, persistence, broadcast, resolver) = build_stack();
    let mut rx = broadcast.subscribe();
    let id = seed_incident(
        persistence.as_ref(),
        registry.as_ref(),
        CueKind::LatencyRegression,
        1_000,
    );
    resolver.acknowledge(id).await.expect("ack");
    let event = rx.recv().await.expect("event");
    let json = serde_json::to_string(&event).expect("serialize");
    // Required fields present
    for required in [
        "\"incident_id\":",
        "\"kind\":",
        "\"scope\":",
        "\"from_state\":",
        "\"to_state\":",
        "\"transitioned_at_unix_nano\":",
    ] {
        assert!(
            json.contains(required),
            "missing required field: {required}"
        );
    }
    // PII-bearing substrings ABSENT
    for banned in [
        "title",
        "detail",
        "evidence_refs",
        "workspace",
        "fingerprint",
        "trace_id",
        "span_id",
        "attribute",
        "exception_message",
    ] {
        assert!(
            !json.contains(banned),
            "PII-bearing substring `{banned}` MUST NOT appear in broadcast payload: {json}"
        );
    }
}

#[test]
fn acknowledge_cooldown_rejects_second_attempt_via_registry() {
    // Cool-down behavior tested through the registry directly (which
    // accepts explicit timestamps) rather than the resolver (which
    // reads SystemTime::now and requires start_paused от tokio test-util
    // feature per testing.md Session Addition 2026-05-03). The
    // resolver simply delegates к registry.acknowledge с current
    // timestamp + DEFAULT_INCIDENT_ACK_COOLDOWN_SECS; correctness is
    // verified at the registry layer.
    let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let inc_a = Incident {
        id: 1,
        workspace: WS.to_string(),
        fingerprint: "fp-a".to_string(),
        title: "[r]".to_string(),
        detail: "[r]".to_string(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        status: IncidentStatus::Active,
        severity: Severity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: vec![],
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: 0,
        updated_at_unix_nano: 0,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
    };
    let mut inc_b = inc_a.clone();
    inc_b.id = 2;
    registry.insert(inc_a);
    registry.insert(inc_b);
    let t0: i64 = 1_000_000_000_000_000_000;
    registry.acknowledge(1, t0, 300).expect("first ack");
    // 60s later — still within 5-min (300s) cool-down.
    let t1 = t0 + 60_000_000_000;
    let result = registry.acknowledge(2, t1, 300);
    assert!(
        matches!(
            result,
            Err(triage::contract::IncidentRegistryError::CooldownActive { .. })
        ),
        "expected CooldownActive; got {result:?}"
    );
}

#[tokio::test]
async fn auto_resolution_resolves_active_past_window() {
    let (registry, persistence, broadcast, _resolver) = build_stack();
    let observer = AutoResolveObserver::new(
        Arc::clone(&registry),
        Arc::clone(&persistence),
        Arc::clone(&broadcast),
    );
    let mut rx = broadcast.subscribe();
    // Seed at t=0
    let _id = seed_incident(
        persistence.as_ref(),
        registry.as_ref(),
        CueKind::ErrorRateSpike,
        0,
    );
    // Evaluate at t = 130s (past 120s window).
    let now_nanos: i64 = 130_000_000_000;
    let (evaluated, resolved) = observer.run_one_tick(now_nanos);
    assert_eq!(evaluated, 1, "1 incident evaluated");
    assert_eq!(resolved, 1, "1 incident resolved");
    // Broadcast event received с to_state = Resolved.
    let event = rx.recv().await.expect("event");
    assert_eq!(event.to_state, IncidentStatus::Resolved);
}

#[tokio::test]
async fn auto_resolution_skips_within_window() {
    let (registry, persistence, broadcast, _resolver) = build_stack();
    let observer = AutoResolveObserver::new(
        Arc::clone(&registry),
        Arc::clone(&persistence),
        Arc::clone(&broadcast),
    );
    seed_incident(
        persistence.as_ref(),
        registry.as_ref(),
        CueKind::ErrorRateSpike,
        0,
    );
    // Evaluate at t = 60s (within 120s window).
    let now_nanos: i64 = 60_000_000_000;
    let (evaluated, resolved) = observer.run_one_tick(now_nanos);
    assert_eq!(evaluated, 0);
    assert_eq!(resolved, 0);
}

#[tokio::test]
async fn auto_resolution_persists_resolved_state_in_corpus() {
    let (registry, persistence, broadcast, _resolver) = build_stack();
    let observer = AutoResolveObserver::new(
        Arc::clone(&registry),
        Arc::clone(&persistence),
        Arc::clone(&broadcast),
    );
    seed_incident(
        persistence.as_ref(),
        registry.as_ref(),
        CueKind::ErrorRateSpike,
        0,
    );
    // Trigger auto-resolution.
    let now_nanos: i64 = 130_000_000_000;
    observer.run_one_tick(now_nanos);
    // Corpus reflects resolution: load_active returns empty (Resolved
    // status excluded by SQL `WHERE status != 'resolved'`).
    let actives = persistence.load_active_incidents(WS).expect("load");
    assert!(
        actives.is_empty(),
        "auto-resolve persisted; load returns empty"
    );
}

#[tokio::test]
async fn explicit_resolve_then_re_resolve_returns_validation_error() {
    let (registry, persistence, _broadcast, resolver) = build_stack();
    let id = seed_incident(
        persistence.as_ref(),
        registry.as_ref(),
        CueKind::ErrorRateSpike,
        1_000,
    );
    resolver
        .clone()
        .mark_resolved(id)
        .await
        .expect("first resolve");
    let second = resolver.mark_resolved(id).await;
    match second {
        Err(AppError::Validation { field, reason }) => {
            assert_eq!(field, "id");
            assert!(reason.contains("invalid"), "reason: {reason}");
        }
        other => panic!("expected Validation error on double-resolve; got {other:?}"),
    }
}

// Suppress unused-import warning for ResolutionTrigger which is consumed
// transitively through the observer's run_one_tick call.
#[allow(dead_code)]
fn _force_resolution_trigger_referenced() -> ResolutionTrigger {
    ResolutionTrigger::AutoResolve
}
