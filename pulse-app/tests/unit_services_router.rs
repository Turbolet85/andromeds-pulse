//! Integration tests for `pulse-app/src/services_router.rs`
//! `services.list_with_states()` TauRPC procedure (chunk #67 + chunk #91
//! per-service severity join).
//!
//! Tests the resolver-level contract:
//! - Empty registry returns empty payload
//! - Manual override reflected in state; priority_tier None with no incidents
//! - Per-service severity join: max active service-scoped incident tier
//!   surfaces on the matching ServiceListItem; services without an active
//!   incident report None; non-service-scoped incidents are excluded
//!
//! Lives here (not colocated in `services_router.rs`) because `pulse-app`
//! sets `[lib] test = false` — colocated `#[cfg(test)]` blocks never run
//! (.claude/rules/testing.md Session Additions 2026-05-20).

use std::sync::Arc;

use pulse_app::services_router::{ServicesApi, ServicesApiImpl};
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, InMemoryIncidentRegistry, InMemoryServiceRegistry, Incident,
    IncidentRegistry, IncidentStatus, PriorityTier, ServiceLifecycleBroadcast,
    ServiceLifecycleState, ServiceRegistry, Severity,
};

const WORKSPACE: &str = "ws-test";

fn incident(id: i64, scope: CueScope, scope_id: Option<&str>, tier: PriorityTier) -> Incident {
    Incident {
        id,
        workspace: WORKSPACE.to_string(),
        fingerprint: format!("fp-{id}"),
        title: "[r]".to_string(),
        detail: "[r]".to_string(),
        kind: CueKind::ErrorRateSpike,
        scope,
        scope_id: scope_id.map(str::to_string),
        status: IncidentStatus::Active,
        severity: Severity::Warn,
        priority_tier: tier,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: vec![],
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: 1_000,
        updated_at_unix_nano: 1_000,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

fn make_impl(
    registry: Arc<dyn ServiceRegistry>,
    incidents: Arc<dyn IncidentRegistry>,
) -> ServicesApiImpl {
    ServicesApiImpl::new(
        registry,
        incidents,
        WORKSPACE.to_string(),
        Arc::new(ServiceLifecycleBroadcast::new()),
    )
}

#[tokio::test]
async fn list_with_states_returns_empty_for_fresh_registry() {
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    let incidents: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let api = make_impl(registry, incidents);
    let payload = api.list_with_states().await.expect("returns Ok");
    assert_eq!(payload.total, 0);
    assert!(payload.items.is_empty());
    assert!(payload.next_cursor.is_none());
}

#[tokio::test]
async fn list_with_states_reflects_manual_override_with_no_severity() {
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    registry.set_manual_override("svc-a", Some(ServiceLifecycleState::Dormant), 1_000_000_000);
    let incidents: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let api = make_impl(registry, incidents);
    let payload = api.list_with_states().await.expect("returns Ok");
    assert_eq!(payload.total, 1);
    assert_eq!(payload.items[0].service, "svc-a");
    assert_eq!(payload.items[0].state, ServiceLifecycleState::Dormant);
    assert_eq!(
        payload.items[0].manual_override,
        Some(ServiceLifecycleState::Dormant),
    );
    assert!(payload.items[0].priority_tier.is_none());
}

#[tokio::test]
async fn list_with_states_enriches_priority_tier_from_active_service_incidents() {
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    registry.set_manual_override("svc-a", Some(ServiceLifecycleState::Active), 1_000_000_000);
    registry.set_manual_override("svc-b", Some(ServiceLifecycleState::Active), 1_000_000_000);
    let incidents: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    // svc-a: two active incidents; the max tier (Autonomous) wins.
    incidents.insert(incident(
        1,
        CueScope::Service,
        Some("svc-a"),
        PriorityTier::Suggested,
    ));
    incidents.insert(incident(
        2,
        CueScope::Service,
        Some("svc-a"),
        PriorityTier::Autonomous,
    ));
    // A global-scoped incident must NOT be attributed to any service dot.
    incidents.insert(incident(
        3,
        CueScope::Global,
        None,
        PriorityTier::Autonomous,
    ));
    let api = make_impl(registry, incidents);

    let payload = api.list_with_states().await.expect("returns Ok");
    let svc_a = payload
        .items
        .iter()
        .find(|i| i.service == "svc-a")
        .expect("svc-a present");
    let svc_b = payload
        .items
        .iter()
        .find(|i| i.service == "svc-b")
        .expect("svc-b present");
    assert_eq!(svc_a.priority_tier, Some(PriorityTier::Autonomous));
    assert!(
        svc_b.priority_tier.is_none(),
        "svc-b has no active service-scoped incident",
    );
}

fn service_incident(id: i64, service: &str, tier: PriorityTier, opened_at: i64) -> Incident {
    let mut inc = incident(id, CueScope::Service, Some(service), tier);
    inc.opened_at_unix_nano = opened_at;
    inc.updated_at_unix_nano = opened_at;
    inc
}

async fn item_for(api: ServicesApiImpl, service: &str) -> triage::contract::ServiceListItem {
    api.list_with_states()
        .await
        .expect("returns Ok")
        .items
        .into_iter()
        .find(|i| i.service == service)
        .expect("service present")
}

fn two_live_services() -> Arc<dyn ServiceRegistry> {
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    registry.set_manual_override("svc-a", Some(ServiceLifecycleState::Active), 1_000_000_000);
    registry.set_manual_override("svc-b", Some(ServiceLifecycleState::Active), 1_000_000_000);
    registry
}

#[tokio::test]
async fn tier_effective_at_is_the_opening_of_the_raising_incident() {
    let incidents: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    incidents.insert(service_incident(1, "svc-a", PriorityTier::Curious, 5_000));
    incidents.insert(service_incident(
        2,
        "svc-a",
        PriorityTier::Autonomous,
        9_000,
    ));
    let item = item_for(make_impl(two_live_services(), incidents), "svc-a").await;
    assert_eq!(item.priority_tier, Some(PriorityTier::Autonomous));
    assert_eq!(item.tier_effective_at_unix_nano, Some(9_000));
}

#[tokio::test]
async fn tier_effective_at_is_the_resolution_of_the_last_max_holder_on_a_fall() {
    let incidents: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    incidents.insert(service_incident(1, "svc-a", PriorityTier::Suggested, 5_000));
    incidents
        .mark_resolved(1, 70_000, triage::contract::ResolutionTrigger::AutoResolve)
        .expect("resolve");
    let item = item_for(make_impl(two_live_services(), incidents), "svc-a").await;
    assert_eq!(item.priority_tier, None, "no active incident remains");
    assert_eq!(item.tier_effective_at_unix_nano, Some(70_000));
}

#[tokio::test]
async fn an_acknowledged_incident_keeps_its_tier_and_its_instant() {
    let incidents: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    incidents.insert(service_incident(1, "svc-a", PriorityTier::Suggested, 5_000));
    incidents.acknowledge(1, 40_000, 0).expect("ack");
    let item = item_for(make_impl(two_live_services(), incidents), "svc-a").await;
    assert_eq!(item.priority_tier, Some(PriorityTier::Suggested));
    assert_eq!(item.tier_effective_at_unix_nano, Some(5_000));
}

#[tokio::test]
async fn an_unaffected_service_carries_no_tier_instant() {
    let incidents: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    incidents.insert(service_incident(
        1,
        "svc-a",
        PriorityTier::Autonomous,
        5_000,
    ));
    let item = item_for(make_impl(two_live_services(), incidents), "svc-b").await;
    assert_eq!(item.priority_tier, None);
    assert_eq!(item.tier_effective_at_unix_nano, None);
}
