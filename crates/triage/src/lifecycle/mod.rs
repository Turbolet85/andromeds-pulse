//! Service lifecycle state machine — chunk #67.
//!
//! L1b service identity formal lifecycle layer per capability spec P-027
//! (Service Constellation Auto-Discovery, formal lifecycle). Seven-state
//! per-service FSM (Unknown → Bootstrapping → Active → Quiet → Silent →
//! Dormant → Archived) with configurable thresholds via Settings struct
//! extension (chunk #67 plan §Step 10) and corpus history lookup on
//! Archived→Active deferred to chunk #69 corpus SQLite scaffold.
//!
//! State derives at heartbeat tick (15s default) from chunk #61 baseline
//! activity-floor snapshots (chunk #64 `ServiceSilenceSnapshot`). Restart
//! events feed via subscription to chunk #63
//! `pulse://stream/restart-events` broadcast — services transitioning to
//! Bootstrapping bypass natural progression on restart-observed gap.
//!
//! Per arch §Cross-cutting Patterns Module dependency direction, this
//! module lives inside the `triage` crate; cross-crate state delivery
//! happens via the `ServiceRegistry` trait + `pulse-app` boundary adapter
//! (mirrors chunk #59 `ReceiverBindStatus` precedent).

mod broadcast;
mod persistence;
mod registry;
mod state_machine;

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast::Receiver;
use tokio::sync::watch;

use crate::baseline::BaselineState;
use crate::pattern::RestartEvent;

pub use broadcast::{
    STREAM_NAME_SERVICE_LIFECYCLE, ServiceLifecycleBroadcast, ServiceLifecycleEvent,
};
pub use persistence::{
    DEFAULT_LIFECYCLE_PERSIST_INTERVAL_SECS, LIFECYCLE_PERSISTENCE_KIND, LifecycleError,
    LifecyclePersistence, TARGET_LIFECYCLE_PERSIST, TARGET_LIFECYCLE_PERSIST_ERROR,
    persist_lifecycle_on_shutdown, run_lifecycle_persist_cycle, run_lifecycle_persist_loop,
};
pub use registry::{
    ACTIVE_TO_QUIET_THRESHOLD_SECONDS, InMemoryServiceRegistry, QUIET_TO_SILENT_FALLBACK_SECONDS,
    ServiceListItem, ServiceRegistry, ServiceRegistryEntry, state_index,
};
pub use state_machine::{ServiceLifecycleState, TransitionTrigger, is_valid_transition};

/// Default heartbeat tick cadence (15s) — mirrors chunk #63
/// `pattern::DEFAULT_HEARTBEAT_INTERVAL` and `.claude/rules/observability.md`
/// §Heartbeat ticks rule.
pub const DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);

/// Hot-reloadable lifecycle thresholds (chunk #96) delivered to the heartbeat
/// via a `watch` channel. `Copy` so the heartbeat re-reads the latest value
/// cheaply each tick. Prospective application: a threshold edit affects the
/// next tick's evaluation, not a retroactive replay (the explicit retrospective
/// pass is `reevaluate_now`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleThresholds {
    pub dormant_after_secs: u64,
    pub archived_after_secs: u64,
}

pub const TARGET_LIFECYCLE_TICK: &str = "triage.lifecycle.tick";
pub const TARGET_LIFECYCLE_TRANSITION: &str = "triage.lifecycle.transition";
pub const TARGET_LIFECYCLE_CORPUS_RESTORE: &str = "triage.lifecycle.corpus_restore";
pub const TARGET_PIPELINE_L1B_TRACKED_SERVICES_TOTAL: &str = "pipeline.l1b.tracked_services_total";
pub const TARGET_METRIC_LIFECYCLE_STATE_DISTRIBUTION: &str =
    "metric.triage.lifecycle.state_distribution";

/// Long-running heartbeat task spawned at boot. Runs two concurrent loops
/// via `tokio::select!`:
///
/// - **Tick loop** (15s default): evaluates `registry.tick_all(...)` against
///   the chunk #61 `BaselineState` snapshot + configurable thresholds;
///   broadcasts each emitted `ServiceLifecycleEvent` on
///   `pulse://stream/service-lifecycle`; emits aggregate tick metric +
///   per-(from_state, to_state) transition counts.
/// - **Restart loop**: subscribes to chunk #63 `pulse://stream/restart-events`;
///   each `RestartEvent` triggers `registry.set_state_on_restart(...)` which
///   may emit a `ServiceLifecycleEvent` (any-state → Bootstrapping bypass).
///
/// Skips the immediate first tick to mirror chunk #62/#63/#66 convention.
/// Pure runtime task; deterministic unit tests live in `registry.rs` (no
/// `tokio::time::pause()` dependency on test-util feature).
pub async fn start_lifecycle_heartbeat(
    registry: Arc<dyn ServiceRegistry>,
    broadcast: Arc<ServiceLifecycleBroadcast>,
    baseline_state: Arc<BaselineState>,
    mut restart_rx: Receiver<RestartEvent>,
    thresholds_rx: watch::Receiver<LifecycleThresholds>,
    heartbeat_interval: Duration,
) {
    let mut interval = tokio::time::interval(heartbeat_interval);
    interval.tick().await;
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let now_nanos = current_time_unix_nano();
                // Re-read the latest hot-reloadable thresholds each tick
                // (chunk #96, prospective: a threshold edit applies from the
                // next tick forward; no retroactive re-classification).
                let t = *thresholds_rx.borrow();
                let events = registry.tick_all(
                    now_nanos,
                    &baseline_state,
                    t.dormant_after_secs,
                    t.archived_after_secs,
                );
                for event in &events {
                    let _ = broadcast.sender().send(event.clone());
                }
                emit_tick_observability(registry.as_ref(), &events);
            }
            Ok(restart) = restart_rx.recv() => {
                let now_nanos = current_time_unix_nano();
                if let Some(event) = registry.set_state_on_restart(&restart.service, now_nanos) {
                    let _ = broadcast.sender().send(event);
                }
            }
        }
    }
}

/// Opt-in retrospective re-evaluation (chunk #96 — capability P-056).
/// Immediately re-classifies every tracked service against the CURRENT
/// thresholds (rather than waiting for the next heartbeat tick), broadcasting
/// the implied lifecycle transitions. Returns the number of transition events
/// emitted. Invoked by `diagnostics.reevaluate_recent_window()` so a user can
/// apply freshly-hot-reloaded thresholds retrospectively.
pub fn reevaluate_now(
    registry: &dyn ServiceRegistry,
    broadcast: &ServiceLifecycleBroadcast,
    baseline_state: &BaselineState,
    thresholds: LifecycleThresholds,
    now_unix_nano: i64,
) -> usize {
    let events = registry.tick_all(
        now_unix_nano,
        baseline_state,
        thresholds.dormant_after_secs,
        thresholds.archived_after_secs,
    );
    for event in &events {
        let _ = broadcast.sender().send(event.clone());
    }
    emit_tick_observability(registry, &events);
    events.len()
}

/// Emit the per-tick observability events. Aggregate-only per
/// `.claude/rules/observability.md` Session Addition 2026-05-17 session 84
/// — no per-service identifiers; counts grouped by enum tag.
pub fn emit_tick_observability(
    registry: &dyn ServiceRegistry,
    events_this_tick: &[ServiceLifecycleEvent],
) {
    let counts = registry.count_by_state();
    let total: usize = counts.iter().sum();
    tracing::info!(
        target: TARGET_LIFECYCLE_TICK,
        tracked_services_total = total as u64,
        services_unknown = counts[0] as u64,
        services_bootstrapping = counts[1] as u64,
        services_active = counts[2] as u64,
        services_quiet = counts[3] as u64,
        services_silent = counts[4] as u64,
        services_dormant = counts[5] as u64,
        services_archived = counts[6] as u64,
        "lifecycle heartbeat tick",
    );
    tracing::info!(
        target: TARGET_PIPELINE_L1B_TRACKED_SERVICES_TOTAL,
        value = total as u64,
        "tracked services total",
    );
    // State distribution metric emitted once per tick with the full count
    // array; per-state values are bounded to the 7 enum tags. Field-name
    // discipline matches `.claude/rules/observability.md` Session Addition
    // 2026-05-03 plural-vs-singular: emit value=total + state="<enum-tag>"
    // in separate events would inflate event count. Keep a single event
    // carrying all counts as fields per chunk #61 baseline.tick precedent.
    tracing::info!(
        target: TARGET_METRIC_LIFECYCLE_STATE_DISTRIBUTION,
        value = total as u64,
        services_unknown = counts[0] as u64,
        services_bootstrapping = counts[1] as u64,
        services_active = counts[2] as u64,
        services_quiet = counts[3] as u64,
        services_silent = counts[4] as u64,
        services_dormant = counts[5] as u64,
        services_archived = counts[6] as u64,
        "state distribution",
    );
    // Aggregate transitions by (from_state, to_state) bucket. NO per-service
    // identifiers on this self-observation surface — per-service detail
    // rides `pulse://stream/service-lifecycle` broadcast only.
    // HashMap (vs BTreeMap) avoids requiring `Ord` on `ServiceLifecycleState`;
    // ordering is not load-bearing for emission.
    let mut buckets: std::collections::HashMap<
        (ServiceLifecycleState, ServiceLifecycleState),
        u64,
    > = std::collections::HashMap::new();
    for event in events_this_tick {
        *buckets
            .entry((event.from_state, event.to_state))
            .or_insert(0) += 1;
    }
    for ((from, to), count) in buckets {
        tracing::info!(
            target: TARGET_LIFECYCLE_TRANSITION,
            from_state = state_label(from),
            to_state = state_label(to),
            count = count,
            "lifecycle transitions this tick",
        );
    }
}

/// Stable snake_case label for a `ServiceLifecycleState` — used in
/// tracing field VALUES for aggregate transition events. Matches the
/// `#[serde(rename_all = "snake_case")]` serialization on the enum so
/// log emissions are consistent with broadcast payloads + TypeScript bindings.
pub fn state_label(state: ServiceLifecycleState) -> &'static str {
    match state {
        ServiceLifecycleState::Unknown => "unknown",
        ServiceLifecycleState::Bootstrapping => "bootstrapping",
        ServiceLifecycleState::Active => "active",
        ServiceLifecycleState::Quiet => "quiet",
        ServiceLifecycleState::Silent => "silent",
        ServiceLifecycleState::Dormant => "dormant",
        ServiceLifecycleState::Archived => "archived",
    }
}

fn current_time_unix_nano() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tracing::field::{Field, Visit};
    use tracing::{Event, Level, Subscriber};

    type CapturedFields = Vec<(String, String)>;
    type CapturedEvent = (String, Level, CapturedFields);
    type CapturedEvents = Arc<Mutex<Vec<CapturedEvent>>>;

    #[derive(Default)]
    struct CapturingSubscriber {
        events: CapturedEvents,
    }

    impl CapturingSubscriber {
        fn new() -> (Self, CapturedEvents) {
            let events: CapturedEvents = Arc::new(Mutex::new(Vec::new()));
            (
                Self {
                    events: Arc::clone(&events),
                },
                events,
            )
        }
    }

    struct FieldCollector(Vec<(String, String)>);

    impl Visit for FieldCollector {
        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            self.0
                .push((field.name().to_string(), format!("{value:?}")));
        }
        fn record_str(&mut self, field: &Field, value: &str) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
        fn record_u64(&mut self, field: &Field, value: u64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
        fn record_i64(&mut self, field: &Field, value: i64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
        fn record_bool(&mut self, field: &Field, value: bool) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
        fn record_f64(&mut self, field: &Field, value: f64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
    }

    impl Subscriber for CapturingSubscriber {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::Id {
            tracing::Id::from_u64(1)
        }
        fn record(&self, _: &tracing::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::Id, _: &tracing::Id) {}
        fn event(&self, event: &Event<'_>) {
            let mut fields = FieldCollector(Vec::new());
            event.record(&mut fields);
            let metadata = event.metadata();
            self.events.lock().expect("lock").push((
                metadata.target().to_string(),
                *metadata.level(),
                fields.0,
            ));
        }
        fn enter(&self, _: &tracing::Id) {}
        fn exit(&self, _: &tracing::Id) {}
    }

    #[test]
    fn state_label_matches_snake_case_serde() {
        assert_eq!(state_label(ServiceLifecycleState::Unknown), "unknown");
        assert_eq!(
            state_label(ServiceLifecycleState::Bootstrapping),
            "bootstrapping"
        );
        assert_eq!(state_label(ServiceLifecycleState::Active), "active");
        assert_eq!(state_label(ServiceLifecycleState::Quiet), "quiet");
        assert_eq!(state_label(ServiceLifecycleState::Silent), "silent");
        assert_eq!(state_label(ServiceLifecycleState::Dormant), "dormant");
        assert_eq!(state_label(ServiceLifecycleState::Archived), "archived");
    }

    #[test]
    fn default_lifecycle_heartbeat_interval_matches_obs_rule() {
        assert_eq!(
            DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL,
            Duration::from_secs(15)
        );
    }

    #[test]
    fn emit_tick_observability_emits_tick_and_metrics_on_empty_registry() {
        let (sub, events) = CapturingSubscriber::new();
        let registry = InMemoryServiceRegistry::new();
        tracing::subscriber::with_default(sub, || {
            emit_tick_observability(&registry, &[]);
        });
        let captured = events.lock().expect("lock");
        let targets: Vec<&str> = captured.iter().map(|(t, _, _)| t.as_str()).collect();
        assert!(
            targets.contains(&TARGET_LIFECYCLE_TICK),
            "missing tick event"
        );
        assert!(
            targets.contains(&TARGET_PIPELINE_L1B_TRACKED_SERVICES_TOTAL),
            "missing pipeline.l1b.tracked_services_total"
        );
        assert!(
            targets.contains(&TARGET_METRIC_LIFECYCLE_STATE_DISTRIBUTION),
            "missing state distribution metric"
        );
        // No transitions on empty registry.
        assert!(!targets.contains(&TARGET_LIFECYCLE_TRANSITION));
    }

    #[test]
    fn emit_tick_observability_emits_aggregate_transition_events() {
        let (sub, events) = CapturingSubscriber::new();
        let registry = InMemoryServiceRegistry::new();
        let transitions = [
            ServiceLifecycleEvent {
                service: "svc-a".to_string(),
                from_state: ServiceLifecycleState::Unknown,
                to_state: ServiceLifecycleState::Bootstrapping,
                transitioned_at_unix_nano: 1_000,
                trigger: TransitionTrigger::Activity,
            },
            ServiceLifecycleEvent {
                service: "svc-b".to_string(),
                from_state: ServiceLifecycleState::Unknown,
                to_state: ServiceLifecycleState::Bootstrapping,
                transitioned_at_unix_nano: 1_000,
                trigger: TransitionTrigger::Activity,
            },
            ServiceLifecycleEvent {
                service: "svc-c".to_string(),
                from_state: ServiceLifecycleState::Bootstrapping,
                to_state: ServiceLifecycleState::Active,
                transitioned_at_unix_nano: 1_000,
                trigger: TransitionTrigger::Activity,
            },
        ];
        tracing::subscriber::with_default(sub, || {
            emit_tick_observability(&registry, &transitions);
        });
        let captured = events.lock().expect("lock");
        let transition_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_LIFECYCLE_TRANSITION)
            .collect();
        // 2 distinct (from, to) buckets: (Unknown,Bootstrapping)×2, (Bootstrapping,Active)×1.
        assert_eq!(transition_events.len(), 2);
        for (target, level, fields) in &transition_events {
            assert_eq!(*target, TARGET_LIFECYCLE_TRANSITION);
            assert_eq!(*level, Level::INFO);
            let from = fields
                .iter()
                .find(|(k, _)| k == "from_state")
                .map(|(_, v)| v.clone());
            let to = fields
                .iter()
                .find(|(k, _)| k == "to_state")
                .map(|(_, v)| v.clone());
            let count = fields
                .iter()
                .find(|(k, _)| k == "count")
                .map(|(_, v)| v.clone());
            assert!(from.is_some() && to.is_some() && count.is_some());
            // Aggregate-only — no per-service identifier should appear.
            for (k, _) in fields.iter() {
                assert_ne!(
                    k.as_str(),
                    "service",
                    "service field MUST NOT appear on aggregate transition event"
                );
                assert_ne!(
                    k.as_str(),
                    "service_name",
                    "service_name field MUST NOT appear on aggregate transition event"
                );
            }
        }
    }

    #[test]
    fn emit_tick_observability_excludes_per_service_pii_fields() {
        // PII negative-canary: even with a canary substring in a transition
        // event's service field, the canary MUST NOT reach the aggregate
        // tracing emission. Per-service detail lives on broadcast only.
        let (sub, events) = CapturingSubscriber::new();
        let registry = InMemoryServiceRegistry::new();
        const CANARY: &str = "service-secret-canary-token-99887";
        let transitions = [ServiceLifecycleEvent {
            service: CANARY.to_string(),
            from_state: ServiceLifecycleState::Unknown,
            to_state: ServiceLifecycleState::Bootstrapping,
            transitioned_at_unix_nano: 1_000,
            trigger: TransitionTrigger::Activity,
        }];
        tracing::subscriber::with_default(sub, || {
            emit_tick_observability(&registry, &transitions);
        });
        let captured = events.lock().expect("lock");
        for (target, _level, fields) in captured.iter() {
            for (name, value) in fields {
                assert!(
                    !name.contains("secret-canary-token-99887"),
                    "CANARY in field NAME ({target}.{name})"
                );
                assert!(
                    !value.contains("secret-canary-token-99887"),
                    "CANARY in field VALUE ({target}.{name}={value})"
                );
            }
        }
    }

    #[test]
    fn emit_tick_observability_field_set_excludes_banned_pii_keys() {
        // Parametric negative-canary mirroring chunk #66
        // `tracing_emission_uses_identifier_class_fields_only`.
        let (sub, events) = CapturingSubscriber::new();
        let registry = InMemoryServiceRegistry::new();
        let transitions = [ServiceLifecycleEvent {
            service: "svc-a".to_string(),
            from_state: ServiceLifecycleState::Active,
            to_state: ServiceLifecycleState::Quiet,
            transitioned_at_unix_nano: 1_000,
            trigger: TransitionTrigger::ThresholdExpiry,
        }];
        tracing::subscriber::with_default(sub, || {
            emit_tick_observability(&registry, &transitions);
        });
        let captured = events.lock().expect("lock");
        let banned_keys = [
            "service_name",
            "service_id",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
            "attribute",
            "instrumentation_scope",
            "body",
            "exception_message",
            "exception_stacktrace",
        ];
        for (target, _level, fields) in captured.iter() {
            for (name, _value) in fields {
                for banned in banned_keys {
                    assert!(
                        !name.contains(banned),
                        "banned key `{banned}` appears in field name ({target}.{name})"
                    );
                }
            }
        }
    }
}
