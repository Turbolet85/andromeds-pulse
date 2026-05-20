//! Service registry + state machine evaluator — chunk #67 + chunk #71
//! corpus persistence.
//!
//! `ServiceRegistry` trait + `InMemoryServiceRegistry` backed by a
//! `DashMap<String, ServiceRegistryEntry>`. State derives at heartbeat
//! tick from chunk #61 `BaselineState` activity-floor snapshots (chunk
//! #64 `ServiceSilenceSnapshot` shape).
//!
//! Corpus persistence implemented in chunk #71 via `LifecyclePersistence`
//! trait at `crates/triage/src/lifecycle/persistence.rs`;
//! `set_state_on_corpus_restore` emits synthetic CorpusRestore lifecycle
//! events on boot for downstream constellation observers.

use std::fmt::Debug;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

use crate::baseline::{BaselineState, BootstrapState, ServiceSilenceSnapshot};

use super::broadcast::ServiceLifecycleEvent;
use super::state_machine::{ServiceLifecycleState, TransitionTrigger, is_valid_transition};

/// Activity-floor quiet-duration cutoff for the Active → Quiet natural
/// transition. 60 seconds matches the chunk #64 bucket interval — services
/// inactive for one full bucket are considered Quiet.
pub const ACTIVE_TO_QUIET_THRESHOLD_SECONDS: u64 = 60;

/// Fallback Quiet → Silent threshold used when a service has no
/// `p95_historical_quiet_duration_seconds` yet (insufficient observation
/// history). 300s ≈ 5 chunk #64 buckets.
pub const QUIET_TO_SILENT_FALLBACK_SECONDS: u64 = 300;

/// Per-service registry entry — lifecycle state plus the timestamps and
/// manual override pin. Identifier-class fields only (service.name is the
/// DashMap key; no per-span / per-attribute payload stored).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRegistryEntry {
    pub state: ServiceLifecycleState,
    pub first_seen_unix_nano: i64,
    pub last_seen_unix_nano: i64,
    pub last_transition_unix_nano: i64,
    pub manual_override: Option<ServiceLifecycleState>,
}

/// Item returned by `ServiceRegistry::list_all` for the
/// `services.list_with_states` TauRPC resolver. Service name is bounded
/// telemetry identifier; manual_override is the operator pin (None when
/// natural state).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceListItem {
    pub service: String,
    pub state: ServiceLifecycleState,
    pub last_seen_unix_nano: i64,
    pub manual_override: Option<ServiceLifecycleState>,
}

/// Service registry — per-service lifecycle state holder + tick evaluator.
/// Implementations MUST be `Send + Sync + Debug` so callers can hold
/// `Arc<dyn ServiceRegistry>` and thread it through TauRPC resolvers.
pub trait ServiceRegistry: Send + Sync + Debug {
    /// Returns the effective state for a service (manual override wins
    /// over natural state). `None` if the service is not tracked.
    fn current_state(&self, service: &str) -> Option<ServiceLifecycleState>;

    /// Snapshot of all tracked services for the `services.list_with_states`
    /// resolver.
    fn list_all(&self) -> Vec<ServiceListItem>;

    /// Pin a service to a state regardless of natural progression. Pass
    /// `None` to clear the override. Returns the emitted lifecycle event
    /// when the effective state changes; `None` otherwise.
    fn set_manual_override(
        &self,
        service: &str,
        override_state: Option<ServiceLifecycleState>,
        now_unix_nano: i64,
    ) -> Option<ServiceLifecycleEvent>;

    /// Force a service to `Bootstrapping` on restart-event observation
    /// (any-state bypass — spec edges do not constrain restart-driven
    /// transitions). Creates the entry if missing. Returns the emitted
    /// event when the effective state changes; `None` otherwise.
    fn set_state_on_restart(
        &self,
        service: &str,
        now_unix_nano: i64,
    ) -> Option<ServiceLifecycleEvent>;

    /// Restore a service to a specific state from corpus persistence at
    /// boot (chunk #71). Inserts or overwrites the entry in the registry
    /// with the restored state + sets first_seen / last_seen /
    /// last_transition = `restored_at_unix_nano`. Emits a synthetic
    /// `ServiceLifecycleEvent { from_state, to_state, trigger:
    /// CorpusRestore }` where `from_state == to_state == restored_state`
    /// — boot-path restore is conceptually a no-transition-but-mark for
    /// downstream constellation observer cascade.
    ///
    /// Bypasses the `is_valid_transition` runtime gate (which rejects
    /// self-loops at `state_machine.rs:187`) by NOT invoking the gate at
    /// restore time. Empty `service` is dropped per chunk #61 service
    /// identity discipline (returns `None`).
    ///
    /// Services restored at `Bootstrapping` are NOT force-promoted; the
    /// next baseline tick will re-evaluate the activity floor and
    /// transition naturally (preserves persisted state verbatim per
    /// chunk #71 plan §Lifecycle restore Bootstrapping handling).
    fn set_state_on_corpus_restore(
        &self,
        service: &str,
        restored_state: ServiceLifecycleState,
        restored_at_unix_nano: i64,
    ) -> Option<ServiceLifecycleEvent>;

    /// Evaluate every service against the activity-floor snapshot + thresholds.
    /// Returns the list of state-transition events to emit on broadcast.
    /// Pure function w.r.t. `now_unix_nano` (no `SystemTime::now()` reads
    /// — tests inject deterministic time).
    fn tick_all(
        &self,
        now_unix_nano: i64,
        baseline: &BaselineState,
        dormant_after_secs: u64,
        archived_after_secs: u64,
    ) -> Vec<ServiceLifecycleEvent>;

    /// Total service count.
    fn count(&self) -> usize;

    /// Count of services per state. Order:
    /// `[Unknown, Bootstrapping, Active, Quiet, Silent, Dormant, Archived]`.
    fn count_by_state(&self) -> [usize; 7];
}

/// In-memory `ServiceRegistry` implementation backed by `DashMap`. Mirrors
/// chunk #61 `BaselineState` lock-free shard pattern.
#[derive(Debug, Default)]
pub struct InMemoryServiceRegistry {
    entries: DashMap<String, ServiceRegistryEntry>,
}

impl InMemoryServiceRegistry {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    /// Construct an `InMemoryServiceRegistry` from previously-persisted
    /// entries restored from corpus at boot (chunk #71). Used by
    /// `pulse-app/src/main.rs` boot wiring after `LifecyclePersistence::
    /// load_all` returns `Ok(Some(entries))`. Subsequent
    /// `set_state_on_corpus_restore` invocations on the populated
    /// registry produce per-service CorpusRestore lifecycle events for
    /// downstream broadcast.
    pub fn from_entries(entries: Vec<(String, ServiceRegistryEntry)>) -> Self {
        let map = DashMap::with_capacity(entries.len());
        for (service, entry) in entries {
            map.insert(service, entry);
        }
        Self { entries: map }
    }
}

impl ServiceRegistry for InMemoryServiceRegistry {
    fn current_state(&self, service: &str) -> Option<ServiceLifecycleState> {
        self.entries
            .get(service)
            .map(|e| e.manual_override.unwrap_or(e.state))
    }

    fn list_all(&self) -> Vec<ServiceListItem> {
        self.entries
            .iter()
            .map(|entry| ServiceListItem {
                service: entry.key().clone(),
                state: entry.manual_override.unwrap_or(entry.state),
                last_seen_unix_nano: entry.last_seen_unix_nano,
                manual_override: entry.manual_override,
            })
            .collect()
    }

    fn set_manual_override(
        &self,
        service: &str,
        override_state: Option<ServiceLifecycleState>,
        now_unix_nano: i64,
    ) -> Option<ServiceLifecycleEvent> {
        let mut entry = self
            .entries
            .entry(service.to_string())
            .or_insert(ServiceRegistryEntry {
                state: ServiceLifecycleState::Unknown,
                first_seen_unix_nano: now_unix_nano,
                last_seen_unix_nano: now_unix_nano,
                last_transition_unix_nano: now_unix_nano,
                manual_override: None,
            });
        let prior_effective = entry.manual_override.unwrap_or(entry.state);
        entry.manual_override = override_state;
        let new_effective = entry.manual_override.unwrap_or(entry.state);
        if prior_effective == new_effective {
            return None;
        }
        entry.last_transition_unix_nano = now_unix_nano;
        Some(ServiceLifecycleEvent {
            service: service.to_string(),
            from_state: prior_effective,
            to_state: new_effective,
            transitioned_at_unix_nano: now_unix_nano,
            trigger: TransitionTrigger::ManualOverride,
        })
    }

    fn set_state_on_restart(
        &self,
        service: &str,
        now_unix_nano: i64,
    ) -> Option<ServiceLifecycleEvent> {
        if service.is_empty() {
            return None;
        }
        let mut entry = self
            .entries
            .entry(service.to_string())
            .or_insert(ServiceRegistryEntry {
                state: ServiceLifecycleState::Unknown,
                first_seen_unix_nano: now_unix_nano,
                last_seen_unix_nano: now_unix_nano,
                last_transition_unix_nano: now_unix_nano,
                manual_override: None,
            });
        if entry.manual_override.is_some() {
            return None;
        }
        let from = entry.state;
        if from == ServiceLifecycleState::Bootstrapping {
            return None;
        }
        entry.state = ServiceLifecycleState::Bootstrapping;
        entry.last_seen_unix_nano = now_unix_nano;
        entry.last_transition_unix_nano = now_unix_nano;
        Some(ServiceLifecycleEvent {
            service: service.to_string(),
            from_state: from,
            to_state: ServiceLifecycleState::Bootstrapping,
            transitioned_at_unix_nano: now_unix_nano,
            trigger: TransitionTrigger::Restart,
        })
    }

    fn set_state_on_corpus_restore(
        &self,
        service: &str,
        restored_state: ServiceLifecycleState,
        restored_at_unix_nano: i64,
    ) -> Option<ServiceLifecycleEvent> {
        if service.is_empty() {
            return None;
        }
        self.entries.insert(
            service.to_string(),
            ServiceRegistryEntry {
                state: restored_state,
                first_seen_unix_nano: restored_at_unix_nano,
                last_seen_unix_nano: restored_at_unix_nano,
                last_transition_unix_nano: restored_at_unix_nano,
                manual_override: None,
            },
        );
        // Self-loop event SHAPE: from_state == to_state == restored_state.
        // Bypasses `is_valid_transition` gate intentionally — boot-path
        // restore is a no-transition-but-mark for downstream constellation
        // observer cascade.
        Some(ServiceLifecycleEvent {
            service: service.to_string(),
            from_state: restored_state,
            to_state: restored_state,
            transitioned_at_unix_nano: restored_at_unix_nano,
            trigger: TransitionTrigger::CorpusRestore,
        })
    }

    fn tick_all(
        &self,
        now_unix_nano: i64,
        baseline: &BaselineState,
        dormant_after_secs: u64,
        archived_after_secs: u64,
    ) -> Vec<ServiceLifecycleEvent> {
        let snapshots = baseline.iter_service_silence_snapshots(now_unix_nano);
        let mut events = Vec::new();

        for snapshot in snapshots {
            if snapshot.service_name.is_empty() {
                continue;
            }
            let service = snapshot.service_name.clone();
            let mut entry = self
                .entries
                .entry(service.clone())
                .or_insert(ServiceRegistryEntry {
                    state: ServiceLifecycleState::Unknown,
                    first_seen_unix_nano: now_unix_nano,
                    last_seen_unix_nano: now_unix_nano,
                    last_transition_unix_nano: now_unix_nano,
                    manual_override: None,
                });

            if snapshot.current_quiet_duration_seconds == 0 {
                entry.last_seen_unix_nano = now_unix_nano;
            }

            // Manual override pins the effective state; underlying natural
            // state still evolves so that clearing the override resumes
            // observed reality. No event emitted while pinned.
            if entry.manual_override.is_some() {
                entry.state = next_natural_state(
                    entry.state,
                    &snapshot,
                    dormant_after_secs,
                    archived_after_secs,
                );
                continue;
            }

            let from = entry.state;
            let to = next_natural_state(from, &snapshot, dormant_after_secs, archived_after_secs);
            if from == to {
                continue;
            }
            if !is_valid_transition(from, to) {
                // Defensive: natural-state transitions must always be a
                // single spec-valid hop. If a future change to
                // `next_natural_state` produces an invalid edge, skip
                // emission (state stays put) rather than emit a malformed
                // event. Self-loops are filtered by the `from == to` guard.
                continue;
            }
            entry.state = to;
            entry.last_transition_unix_nano = now_unix_nano;
            let trigger = match (from, to) {
                (ServiceLifecycleState::Unknown, _) => TransitionTrigger::Activity,
                (ServiceLifecycleState::Bootstrapping, _) => TransitionTrigger::Activity,
                (
                    _,
                    ServiceLifecycleState::Silent
                    | ServiceLifecycleState::Dormant
                    | ServiceLifecycleState::Archived,
                ) => TransitionTrigger::ThresholdExpiry,
                _ => TransitionTrigger::Activity,
            };
            events.push(ServiceLifecycleEvent {
                service: service.clone(),
                from_state: from,
                to_state: to,
                transitioned_at_unix_nano: now_unix_nano,
                trigger,
            });
        }
        events
    }

    fn count(&self) -> usize {
        self.entries.len()
    }

    fn count_by_state(&self) -> [usize; 7] {
        let mut counts = [0usize; 7];
        for entry in self.entries.iter() {
            let s = entry.manual_override.unwrap_or(entry.state);
            counts[state_index(s)] += 1;
        }
        counts
    }
}

/// Single-step natural transition. Spec edges only — manual override and
/// restart-driven transitions are handled by the registry methods directly.
fn next_natural_state(
    current: ServiceLifecycleState,
    snapshot: &ServiceSilenceSnapshot,
    dormant_after_secs: u64,
    archived_after_secs: u64,
) -> ServiceLifecycleState {
    use ServiceLifecycleState::*;

    let quiet = snapshot.current_quiet_duration_seconds;
    let p95 = snapshot
        .p95_historical_quiet_duration_seconds
        .unwrap_or(QUIET_TO_SILENT_FALLBACK_SECONDS);
    let quiet_to_silent_threshold = p95.max(QUIET_TO_SILENT_FALLBACK_SECONDS);

    match current {
        Unknown => Bootstrapping,
        Bootstrapping => match snapshot.bootstrap_state {
            BootstrapState::Ready => Active,
            BootstrapState::Learning => Bootstrapping,
        },
        Active => {
            if quiet >= ACTIVE_TO_QUIET_THRESHOLD_SECONDS {
                Quiet
            } else {
                Active
            }
        }
        Quiet => {
            if quiet >= quiet_to_silent_threshold {
                Silent
            } else if quiet == 0 {
                Active
            } else {
                Quiet
            }
        }
        Silent => {
            if quiet >= dormant_after_secs {
                Dormant
            } else if quiet < quiet_to_silent_threshold {
                Quiet
            } else {
                Silent
            }
        }
        Dormant => {
            if quiet >= archived_after_secs {
                Archived
            } else {
                Dormant
            }
        }
        Archived => Archived,
    }
}

/// Map ServiceLifecycleState to a stable 0-6 index for the
/// `count_by_state` array slot. Order matches the type docstring on
/// `ServiceRegistry::count_by_state`.
pub fn state_index(state: ServiceLifecycleState) -> usize {
    use ServiceLifecycleState::*;
    match state {
        Unknown => 0,
        Bootstrapping => 1,
        Active => 2,
        Quiet => 3,
        Silent => 4,
        Dormant => 5,
        Archived => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline::BaselineState;

    const NANOS_PER_SEC: i64 = 1_000_000_000;

    fn fresh_registry() -> InMemoryServiceRegistry {
        InMemoryServiceRegistry::new()
    }

    #[test]
    fn registry_starts_empty() {
        let r = fresh_registry();
        assert_eq!(r.count(), 0);
        assert!(r.current_state("svc-a").is_none());
        assert!(r.list_all().is_empty());
        assert_eq!(r.count_by_state(), [0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn tick_all_returns_empty_when_baseline_empty() {
        let r = fresh_registry();
        let baseline = BaselineState::new();
        let events = r.tick_all(1_000 * NANOS_PER_SEC, &baseline, 3600, 86_400);
        assert!(events.is_empty());
        assert_eq!(r.count(), 0);
    }

    #[test]
    fn tick_all_first_observation_creates_bootstrapping_entry() {
        let r = fresh_registry();
        let baseline = BaselineState::new();
        baseline.observe_span("svc-a", "op-1", 0, 50, 1_000 * NANOS_PER_SEC);
        let events = r.tick_all(1_001 * NANOS_PER_SEC, &baseline, 3600, 86_400);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].from_state, ServiceLifecycleState::Unknown);
        assert_eq!(events[0].to_state, ServiceLifecycleState::Bootstrapping);
        assert_eq!(events[0].trigger, TransitionTrigger::Activity);
        assert_eq!(events[0].service, "svc-a");
        assert_eq!(r.count(), 1);
        assert_eq!(
            r.current_state("svc-a"),
            Some(ServiceLifecycleState::Bootstrapping),
        );
    }

    #[test]
    fn tick_all_advances_bootstrapping_to_active_when_bootstrap_ready() {
        let r = fresh_registry();
        let baseline = BaselineState::new();
        // Seed observations to populate the activity floor; advance time enough
        // to push BootstrapState from Learning → Ready (chunk #64
        // BOOTSTRAP_WINDOW_SECONDS).
        baseline.observe_span("svc-a", "op-1", 0, 50, 1_000 * NANOS_PER_SEC);
        let _ = r.tick_all(1_001 * NANOS_PER_SEC, &baseline, 3600, 86_400);
        // Continue observing after the bootstrap window.
        let later = (1_000 + 60 * 60) * NANOS_PER_SEC;
        baseline.observe_span("svc-a", "op-1", 0, 50, later);
        let events = r.tick_all(later, &baseline, 3600, 86_400);
        assert!(
            events
                .iter()
                .any(|e| e.from_state == ServiceLifecycleState::Bootstrapping
                    && e.to_state == ServiceLifecycleState::Active),
            "expected Bootstrapping → Active transition; got {events:?}"
        );
        assert_eq!(
            r.current_state("svc-a"),
            Some(ServiceLifecycleState::Active),
        );
    }

    #[test]
    fn tick_all_emits_one_event_per_transition() {
        let r = fresh_registry();
        let baseline = BaselineState::new();
        baseline.observe_span("svc-a", "op", 0, 50, 1_000 * NANOS_PER_SEC);
        baseline.observe_span("svc-b", "op", 0, 50, 1_000 * NANOS_PER_SEC);
        let events = r.tick_all(1_001 * NANOS_PER_SEC, &baseline, 3600, 86_400);
        assert_eq!(events.len(), 2);
        let services: Vec<&str> = events.iter().map(|e| e.service.as_str()).collect();
        assert!(services.contains(&"svc-a"));
        assert!(services.contains(&"svc-b"));
        // Idempotent: re-running tick with same baseline state produces no
        // new transitions (already at Bootstrapping; bootstrap_state still
        // Learning).
        let events2 = r.tick_all(1_002 * NANOS_PER_SEC, &baseline, 3600, 86_400);
        assert!(
            events2.is_empty(),
            "re-tick should be no-op; got {events2:?}"
        );
    }

    #[test]
    fn set_manual_override_forces_state_regardless_of_activity() {
        let r = fresh_registry();
        let event = r
            .set_manual_override(
                "svc-a",
                Some(ServiceLifecycleState::Dormant),
                1_000 * NANOS_PER_SEC,
            )
            .expect("override emits event");
        assert_eq!(event.from_state, ServiceLifecycleState::Unknown);
        assert_eq!(event.to_state, ServiceLifecycleState::Dormant);
        assert_eq!(event.trigger, TransitionTrigger::ManualOverride);
        assert_eq!(
            r.current_state("svc-a"),
            Some(ServiceLifecycleState::Dormant),
        );

        // Subsequent activity should NOT alter the pinned state.
        let baseline = BaselineState::new();
        baseline.observe_span("svc-a", "op", 0, 50, 1_000 * NANOS_PER_SEC);
        let events = r.tick_all(1_001 * NANOS_PER_SEC, &baseline, 3600, 86_400);
        assert!(
            events.is_empty(),
            "pinned service should not emit; got {events:?}"
        );
        assert_eq!(
            r.current_state("svc-a"),
            Some(ServiceLifecycleState::Dormant),
            "pinned state survives ticks",
        );
    }

    #[test]
    fn set_manual_override_clearing_emits_event_when_natural_differs() {
        let r = fresh_registry();
        r.set_manual_override(
            "svc-a",
            Some(ServiceLifecycleState::Dormant),
            1_000 * NANOS_PER_SEC,
        );
        // Underlying natural state is Unknown; clearing override transitions
        // effective state Dormant → Unknown.
        let event = r
            .set_manual_override("svc-a", None, 2_000 * NANOS_PER_SEC)
            .expect("clear emits event");
        assert_eq!(event.from_state, ServiceLifecycleState::Dormant);
        assert_eq!(event.to_state, ServiceLifecycleState::Unknown);
        assert_eq!(event.trigger, TransitionTrigger::ManualOverride);
    }

    #[test]
    fn set_manual_override_returns_none_when_effective_state_unchanged() {
        let r = fresh_registry();
        r.set_manual_override(
            "svc-a",
            Some(ServiceLifecycleState::Unknown),
            1_000 * NANOS_PER_SEC,
        );
        // Re-pinning to the same effective state should be a no-op.
        assert!(
            r.set_manual_override(
                "svc-a",
                Some(ServiceLifecycleState::Unknown),
                2_000 * NANOS_PER_SEC,
            )
            .is_none()
        );
    }

    #[test]
    fn set_state_on_restart_creates_bootstrapping_entry() {
        let r = fresh_registry();
        let event = r
            .set_state_on_restart("svc-a", 1_000 * NANOS_PER_SEC)
            .expect("restart emits event");
        assert_eq!(event.from_state, ServiceLifecycleState::Unknown);
        assert_eq!(event.to_state, ServiceLifecycleState::Bootstrapping);
        assert_eq!(event.trigger, TransitionTrigger::Restart);
        assert_eq!(
            r.current_state("svc-a"),
            Some(ServiceLifecycleState::Bootstrapping),
        );
    }

    #[test]
    fn set_state_on_restart_bypasses_archived() {
        let r = fresh_registry();
        // Force the service to Archived via manual override, then clear,
        // then set natural state Archived via direct entry mutation.
        r.entries.insert(
            "svc-a".to_string(),
            ServiceRegistryEntry {
                state: ServiceLifecycleState::Archived,
                first_seen_unix_nano: 0,
                last_seen_unix_nano: 0,
                last_transition_unix_nano: 0,
                manual_override: None,
            },
        );
        let event = r
            .set_state_on_restart("svc-a", 1_000 * NANOS_PER_SEC)
            .expect("restart event emitted");
        assert_eq!(event.from_state, ServiceLifecycleState::Archived);
        assert_eq!(event.to_state, ServiceLifecycleState::Bootstrapping);
    }

    #[test]
    fn set_state_on_restart_skips_when_already_bootstrapping() {
        let r = fresh_registry();
        r.set_state_on_restart("svc-a", 1_000 * NANOS_PER_SEC);
        // Second restart while still Bootstrapping is a no-op (avoids re-emit).
        assert!(
            r.set_state_on_restart("svc-a", 2_000 * NANOS_PER_SEC)
                .is_none()
        );
    }

    #[test]
    fn set_state_on_restart_drops_empty_service() {
        let r = fresh_registry();
        assert!(r.set_state_on_restart("", 1_000 * NANOS_PER_SEC).is_none());
        assert_eq!(r.count(), 0);
    }

    #[test]
    fn set_state_on_restart_respects_manual_override() {
        let r = fresh_registry();
        r.set_manual_override(
            "svc-a",
            Some(ServiceLifecycleState::Dormant),
            1_000 * NANOS_PER_SEC,
        );
        assert!(
            r.set_state_on_restart("svc-a", 2_000 * NANOS_PER_SEC)
                .is_none()
        );
        assert_eq!(
            r.current_state("svc-a"),
            Some(ServiceLifecycleState::Dormant),
        );
    }

    #[test]
    fn count_by_state_orders_variants_consistently() {
        let r = fresh_registry();
        r.entries.insert(
            "svc-a".to_string(),
            ServiceRegistryEntry {
                state: ServiceLifecycleState::Active,
                first_seen_unix_nano: 0,
                last_seen_unix_nano: 0,
                last_transition_unix_nano: 0,
                manual_override: None,
            },
        );
        r.entries.insert(
            "svc-b".to_string(),
            ServiceRegistryEntry {
                state: ServiceLifecycleState::Quiet,
                first_seen_unix_nano: 0,
                last_seen_unix_nano: 0,
                last_transition_unix_nano: 0,
                manual_override: Some(ServiceLifecycleState::Archived),
            },
        );
        let counts = r.count_by_state();
        // [Unknown=0, Bootstrapping=0, Active=1, Quiet=0, Silent=0, Dormant=0, Archived=1]
        assert_eq!(counts, [0, 0, 1, 0, 0, 0, 1]);
    }

    #[test]
    fn list_all_returns_per_service_items_with_effective_state() {
        let r = fresh_registry();
        r.set_manual_override(
            "svc-a",
            Some(ServiceLifecycleState::Dormant),
            1_000 * NANOS_PER_SEC,
        );
        let items = r.list_all();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].service, "svc-a");
        assert_eq!(items[0].state, ServiceLifecycleState::Dormant);
        assert_eq!(
            items[0].manual_override,
            Some(ServiceLifecycleState::Dormant)
        );
    }

    #[test]
    fn state_index_is_total_and_distinct() {
        let mut seen = [false; 7];
        for s in [
            ServiceLifecycleState::Unknown,
            ServiceLifecycleState::Bootstrapping,
            ServiceLifecycleState::Active,
            ServiceLifecycleState::Quiet,
            ServiceLifecycleState::Silent,
            ServiceLifecycleState::Dormant,
            ServiceLifecycleState::Archived,
        ] {
            let idx = state_index(s);
            assert!(idx < 7);
            assert!(!seen[idx], "duplicate index {idx} for {s:?}");
            seen[idx] = true;
        }
        assert!(seen.iter().all(|&x| x));
    }

    // Threshold matrix tests using injected ServiceSilenceSnapshot directly,
    // bypassing BaselineState. Mirrors chunk #64 / #66 deterministic-time
    // pattern (no tokio::time::pause()).

    fn snapshot_with(
        quiet_secs: u64,
        p95: Option<u64>,
        bs: BootstrapState,
    ) -> ServiceSilenceSnapshot {
        ServiceSilenceSnapshot {
            service_name: "svc-test".to_string(),
            current_quiet_duration_seconds: quiet_secs,
            p95_historical_quiet_duration_seconds: p95,
            bootstrap_state: bs,
        }
    }

    #[test]
    fn next_natural_state_unknown_to_bootstrapping() {
        let s = snapshot_with(0, None, BootstrapState::Learning);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Unknown, &s, 3600, 86_400),
            ServiceLifecycleState::Bootstrapping,
        );
    }

    #[test]
    fn next_natural_state_bootstrapping_to_active_when_ready() {
        let s = snapshot_with(0, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Bootstrapping, &s, 3600, 86_400),
            ServiceLifecycleState::Active,
        );
    }

    #[test]
    fn next_natural_state_bootstrapping_stays_when_learning() {
        let s = snapshot_with(0, None, BootstrapState::Learning);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Bootstrapping, &s, 3600, 86_400),
            ServiceLifecycleState::Bootstrapping,
        );
    }

    #[test]
    fn next_natural_state_active_to_quiet_at_threshold() {
        let s = snapshot_with(
            ACTIVE_TO_QUIET_THRESHOLD_SECONDS,
            None,
            BootstrapState::Ready,
        );
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Active, &s, 3600, 86_400),
            ServiceLifecycleState::Quiet,
        );

        let s_just_under = snapshot_with(
            ACTIVE_TO_QUIET_THRESHOLD_SECONDS - 1,
            None,
            BootstrapState::Ready,
        );
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Active, &s_just_under, 3600, 86_400),
            ServiceLifecycleState::Active,
        );
    }

    #[test]
    fn next_natural_state_quiet_to_active_on_zero_quiet() {
        let s = snapshot_with(0, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Quiet, &s, 3600, 86_400),
            ServiceLifecycleState::Active,
        );
    }

    #[test]
    fn next_natural_state_quiet_to_silent_at_p95() {
        // p95 = 200s; threshold is max(200, 300) = 300.
        let s = snapshot_with(300, Some(200), BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Quiet, &s, 3600, 86_400),
            ServiceLifecycleState::Silent,
        );

        // p95 = 500s; threshold is 500.
        let s_higher = snapshot_with(499, Some(500), BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Quiet, &s_higher, 3600, 86_400),
            ServiceLifecycleState::Quiet,
        );
        let s_at = snapshot_with(500, Some(500), BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Quiet, &s_at, 3600, 86_400),
            ServiceLifecycleState::Silent,
        );
    }

    #[test]
    fn next_natural_state_quiet_uses_fallback_when_no_p95() {
        // No p95: threshold falls back to QUIET_TO_SILENT_FALLBACK_SECONDS (300).
        let s = snapshot_with(300, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Quiet, &s, 3600, 86_400),
            ServiceLifecycleState::Silent,
        );
    }

    #[test]
    fn next_natural_state_silent_to_quiet_when_activity_returns() {
        // p95 = 200s → threshold max(200, 300) = 300. Quiet < 300 → Quiet.
        let s = snapshot_with(100, Some(200), BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Silent, &s, 3600, 86_400),
            ServiceLifecycleState::Quiet,
        );
    }

    #[test]
    fn next_natural_state_silent_to_dormant_at_dormant_threshold() {
        let s = snapshot_with(3600, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Silent, &s, 3600, 86_400),
            ServiceLifecycleState::Dormant,
        );

        let s_just_under = snapshot_with(3599, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Silent, &s_just_under, 3600, 86_400),
            ServiceLifecycleState::Silent,
        );
    }

    #[test]
    fn next_natural_state_dormant_to_archived_at_archived_threshold() {
        let s = snapshot_with(86_400, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Dormant, &s, 3600, 86_400),
            ServiceLifecycleState::Archived,
        );

        let s_just_under = snapshot_with(86_399, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Dormant, &s_just_under, 3600, 86_400),
            ServiceLifecycleState::Dormant,
        );
    }

    #[test]
    fn next_natural_state_archived_is_absorbing() {
        let s_active = snapshot_with(0, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Archived, &s_active, 3600, 86_400),
            ServiceLifecycleState::Archived,
        );
        let s_inactive = snapshot_with(100_000, None, BootstrapState::Ready);
        assert_eq!(
            next_natural_state(ServiceLifecycleState::Archived, &s_inactive, 3600, 86_400),
            ServiceLifecycleState::Archived,
        );
    }

    #[test]
    fn tick_all_skips_empty_service_name_snapshots() {
        // Guard: even if a baseline snapshot somehow has empty service_name,
        // tick_all skips rather than creating an empty-key entry.
        let r = fresh_registry();
        let baseline = BaselineState::new();
        // BaselineState rejects empty-name spans, so iter_service_silence_snapshots
        // produces no empty-name entries naturally. Verify the registry stays
        // empty for an empty baseline.
        baseline.observe_span("", "op", 0, 50, 1_000 * NANOS_PER_SEC);
        let events = r.tick_all(1_001 * NANOS_PER_SEC, &baseline, 3600, 86_400);
        assert!(events.is_empty());
        assert_eq!(r.count(), 0);
    }
}
