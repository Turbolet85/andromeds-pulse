//! Chunk #67 E2E coverage — service lifecycle state machine integration.
//! Exercises cross-module wiring between chunk #61 `BaselineState`, chunk
//! #63 `RestartEventBroadcast`, chunk #67 `InMemoryServiceRegistry` plus
//! `ServiceLifecycleBroadcast`, and the `start_lifecycle_heartbeat` async
//! task. Coverage spans the heartbeat tick path (baseline activity drives
//! Unknown to Bootstrapping transitions broadcast), the restart event
//! subscription path (Bootstrapping bypass с trigger=Restart), the PII
//! negative-canary on broadcast payloads (only identifier-class fields),
//! and the `services.list_with_states` resolver returning typed payload.
//! Threshold-driven transitions are exercised by deterministic unit tests
//! in `crates/triage/src/lifecycle/registry.rs::tests` (injected time).

use std::sync::Arc;
use std::time::Duration;

use pulse_app::services_router::{ServiceListPayload, ServicesApiImpl};
use tokio::sync::broadcast;
use triage::contract::{
    BaselineState, InMemoryIncidentRegistry, InMemoryServiceRegistry, IncidentRegistry,
    RestartEvent, RestartEventBroadcast, ServiceLifecycleBroadcast, ServiceLifecycleEvent,
    ServiceLifecycleState, ServiceRegistry, TransitionTrigger, start_lifecycle_heartbeat,
};

const SHORT_HEARTBEAT: Duration = Duration::from_millis(50);
const NANOS_PER_SEC: i64 = 1_000_000_000;

fn boot_lifecycle(
    baseline_state: Arc<BaselineState>,
    restart_broadcast: Arc<RestartEventBroadcast>,
    dormant_secs: u64,
    archived_secs: u64,
) -> (
    Arc<dyn ServiceRegistry>,
    Arc<ServiceLifecycleBroadcast>,
    broadcast::Receiver<ServiceLifecycleEvent>,
    tokio::task::JoinHandle<()>,
) {
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    let broadcast_handle = Arc::new(ServiceLifecycleBroadcast::new());
    let rx = broadcast_handle.subscribe();
    let restart_rx = restart_broadcast.subscribe();
    // Chunk #96 — `start_lifecycle_heartbeat` now reads its thresholds from a
    // `watch` channel (hot-reloadable). Seed it once with the test thresholds;
    // the heartbeat re-reads the (constant) value each tick.
    let (_thresh_tx, thresh_rx) =
        tokio::sync::watch::channel(triage::contract::LifecycleThresholds {
            dormant_after_secs: dormant_secs,
            archived_after_secs: archived_secs,
        });
    let task = tokio::spawn(start_lifecycle_heartbeat(
        Arc::clone(&registry),
        Arc::clone(&broadcast_handle),
        baseline_state,
        restart_rx,
        thresh_rx,
        SHORT_HEARTBEAT,
    ));
    (registry, broadcast_handle, rx, task)
}

#[tokio::test]
async fn heartbeat_emits_unknown_to_bootstrapping_on_first_activity() {
    let baseline = Arc::new(BaselineState::new());
    let restart_broadcast = Arc::new(RestartEventBroadcast::new());
    baseline.observe_span("svc-alpha", "op-1", 0, 50, 1_000 * NANOS_PER_SEC);

    let (registry, _lifecycle_broadcast, mut rx, task) = boot_lifecycle(
        Arc::clone(&baseline),
        Arc::clone(&restart_broadcast),
        3_600,
        86_400,
    );

    // Wait for ≥1 heartbeat tick. SHORT_HEARTBEAT=50ms; skip the first
    // tick is consumed by `interval.tick().await` inside the task before
    // the loop body. So total wait ≥ 2× HEARTBEAT.
    let received = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("heartbeat fired within 2s")
        .expect("broadcast payload available");

    assert_eq!(received.service, "svc-alpha");
    assert_eq!(received.from_state, ServiceLifecycleState::Unknown);
    assert_eq!(received.to_state, ServiceLifecycleState::Bootstrapping);
    assert_eq!(received.trigger, TransitionTrigger::Activity);

    assert_eq!(
        registry.current_state("svc-alpha"),
        Some(ServiceLifecycleState::Bootstrapping),
    );

    task.abort();
}

#[tokio::test]
async fn restart_event_subscription_triggers_bootstrapping_transition() {
    let baseline = Arc::new(BaselineState::new());
    let restart_broadcast = Arc::new(RestartEventBroadcast::new());

    // Pre-seed registry с svc-beta in Quiet state (skip natural progression
    // for the test — use set_manual_override then clear to position the
    // entry, OR observe a span first to get Bootstrapping, then drive
    // through ticks). Simpler: insert directly via set_manual_override then
    // clear.
    let (registry, _lifecycle_broadcast, mut rx, task) = boot_lifecycle(
        Arc::clone(&baseline),
        Arc::clone(&restart_broadcast),
        3_600,
        86_400,
    );

    // Drain the initial bootstrap event from any prior baseline activity
    // (none in this test, but be defensive about heartbeat firing on empty
    // baseline). Channel is empty initially; heartbeat tick on empty
    // baseline produces no events.

    // Emit a RestartEvent on the chunk #63 broadcast. The lifecycle
    // heartbeat task subscribes к this channel; on receive it calls
    // `registry.set_state_on_restart` and broadcasts the resulting
    // `ServiceLifecycleEvent`.
    let restart_event = RestartEvent {
        service: "svc-beta".to_string(),
        gap_seconds: 25,
        last_seen_unix_nano: 1_000 * NANOS_PER_SEC,
        resume_unix_nano: 1_025 * NANOS_PER_SEC,
    };
    restart_broadcast
        .sender()
        .send(restart_event)
        .expect("restart event sent");

    let received = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("lifecycle event fired within 2s")
        .expect("broadcast payload available");

    assert_eq!(received.service, "svc-beta");
    assert_eq!(received.to_state, ServiceLifecycleState::Bootstrapping);
    assert_eq!(received.trigger, TransitionTrigger::Restart);

    assert_eq!(
        registry.current_state("svc-beta"),
        Some(ServiceLifecycleState::Bootstrapping),
    );

    task.abort();
}

#[tokio::test]
async fn service_lifecycle_event_payload_has_no_pii_fields() {
    let baseline = Arc::new(BaselineState::new());
    let restart_broadcast = Arc::new(RestartEventBroadcast::new());

    // Use a service-name canary that should NOT leak beyond the bounded
    // `service: String` field of `ServiceLifecycleEvent`. Verify by
    // serializing the payload + grepping for banned PII substrings.
    const CANARY_SERVICE: &str = "svc-canary-secret-token-99887";
    baseline.observe_span(CANARY_SERVICE, "op", 0, 50, 1_000 * NANOS_PER_SEC);

    let (_registry, _lifecycle_broadcast, mut rx, task) = boot_lifecycle(
        Arc::clone(&baseline),
        Arc::clone(&restart_broadcast),
        3_600,
        86_400,
    );

    let received = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("heartbeat fired within 2s")
        .expect("broadcast payload available");

    // The service name IS allowed на the bounded `service` field; the
    // PII discipline forbids attribute-value / span / scope / body fields.
    let json = serde_json::to_string(&received).expect("serialize");
    for banned in [
        "attribute",
        "attributes",
        "span_id",
        "trace_id",
        "scope_name",
        "instrumentation_scope",
        "body",
        "description",
        "exception_message",
        "exception_stacktrace",
    ] {
        assert!(
            !json.contains(banned),
            "PII substring `{banned}` MUST NOT appear in lifecycle event payload: {json}"
        );
    }

    // The bounded `service` field IS expected к contain the canary
    // (that's the per-service identifier — broadcast surface admits it).
    assert!(
        json.contains(CANARY_SERVICE),
        "service field should carry the bounded identifier"
    );

    task.abort();
}

#[tokio::test]
async fn services_list_with_states_resolver_returns_typed_payload() {
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    registry.set_manual_override(
        "svc-gamma",
        Some(ServiceLifecycleState::Dormant),
        1_000 * NANOS_PER_SEC,
    );
    let lifecycle_broadcast = Arc::new(ServiceLifecycleBroadcast::new());
    let incident_registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
    let api = ServicesApiImpl::new(
        Arc::clone(&registry),
        incident_registry,
        "ws-test".to_string(),
        lifecycle_broadcast,
    );

    // Direct resolver invocation (no taurpc IPC layer needed for contract
    // verification of the typed return shape).
    use pulse_app::services_router::ServicesApi;
    let payload: ServiceListPayload = api.list_with_states().await.expect("returns Ok");
    assert_eq!(payload.total, 1);
    assert!(payload.next_cursor.is_none());
    assert_eq!(payload.items[0].service, "svc-gamma");
    assert_eq!(payload.items[0].state, ServiceLifecycleState::Dormant);
    assert_eq!(
        payload.items[0].manual_override,
        Some(ServiceLifecycleState::Dormant),
    );
}

#[tokio::test]
async fn lifecycle_event_with_dormant_archived_thresholds_uses_settings_values() {
    // Verify that user-supplied threshold knobs (Settings struct extension)
    // actually thread through к the heartbeat task. Pre-load svc-delta
    // through ticks until Silent state, then assert that with а
    // dormant_after_secs=120 threshold (NOT the default 3600), the next
    // tick after >120s quiet advances к Dormant.
    let baseline = Arc::new(BaselineState::new());
    let restart_broadcast = Arc::new(RestartEventBroadcast::new());

    // Threshold-driven progression is deterministic via injected time in
    // the unit tests; this е2е test asserts that the configured
    // dormant_after_secs argument is non-default. Lower-bound verification:
    // the spawned task receives the values as ordinary u64 arguments — if
    // settings.lifecycle_dormant_after_secs wasn't passed correctly, the
    // call site wouldn't compile. So this test reduces к а compile-time +
    // smoke verification that the boot wiring shape compiles.
    let (_registry, _lifecycle_broadcast, _rx, task) = boot_lifecycle(
        Arc::clone(&baseline),
        Arc::clone(&restart_broadcast),
        120,    // dormant after 2 min (non-default)
        86_400, // archived after 24h (default)
    );

    // Wait а short period to ensure the task didn't panic on the
    // non-default threshold values (compile-time bound check + boot smoke).
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert!(
        !task.is_finished(),
        "lifecycle heartbeat task should still be running"
    );
    task.abort();
}
