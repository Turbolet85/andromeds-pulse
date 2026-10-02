//! Incident auto-resolution observer — chunk #78.
//!
//! `AutoResolveObserver` + `run_auto_resolution_loop` ticker. Every 30s
//! evaluates Active + Acknowledged incidents against the 120s
//! no-reemission window per capability spec P-022. Each transitioning
//! incident runs registry → mark_resolved then persistence →
//! update_incident_status then broadcast → IncidentLifecycleEvent
//! (to_state = Resolved). Aggregate tick event emitted per cycle per
//! CLAUDE.md observability rule 2026-05-17 session 84 aggregate-only
//! AllowList convention.

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use triage::contract::{
    DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS, IncidentLifecycleBroadcast, IncidentLifecycleEvent,
    IncidentPersistence, IncidentRegistry, IncidentStatus, ResolutionTrigger,
};

/// Default tick cadence — 30s. Auto-resolution evaluates each tick;
/// individual incidents resolve when their `now - updated_at >= window`
/// per chunk #78 plan. Sweep cadence intentionally shorter than the
/// 120s window to keep resolution latency bounded (~30s p99).
pub const DEFAULT_AUTO_RESOLVE_TICK_INTERVAL: Duration = Duration::from_secs(30);

/// Tracing target for aggregate per-tick observability. Aggregate-only
/// fields (evaluated_count + resolved_count + duration_ms) per CLAUDE.md
/// observability.md Session Addition 2026-05-17 session 84.
pub const TARGET_INCIDENT_AUTO_RESOLVE_TICK: &str = "triage.incident.auto_resolve.tick";

/// Auto-resolution observer holding the live registry + persistence +
/// broadcast handle. Cheap to clone (single Arc each).
#[derive(Clone)]
pub struct AutoResolveObserver {
    registry: Arc<dyn IncidentRegistry>,
    persistence: Arc<dyn IncidentPersistence>,
    broadcast: Arc<IncidentLifecycleBroadcast>,
    window_secs: u64,
}

impl AutoResolveObserver {
    pub fn new(
        registry: Arc<dyn IncidentRegistry>,
        persistence: Arc<dyn IncidentPersistence>,
        broadcast: Arc<IncidentLifecycleBroadcast>,
    ) -> Self {
        Self {
            registry,
            persistence,
            broadcast,
            window_secs: DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS,
        }
    }

    /// Single tick: evaluate, transition, persist, broadcast. Returns
    /// `(evaluated_count, resolved_count)`. Pure-function w.r.t.
    /// `now_unix_nano` — tests inject deterministic time.
    pub fn run_one_tick(&self, now_unix_nano: i64) -> (u64, u64) {
        let to_resolve = self
            .registry
            .evaluate_auto_resolution(now_unix_nano, self.window_secs);
        let evaluated = to_resolve.len() as u64;
        let mut resolved: u64 = 0;
        for id in to_resolve {
            let result =
                self.registry
                    .mark_resolved(id, now_unix_nano, ResolutionTrigger::AutoResolve);
            if let Ok(updated) = result {
                let from_state = if updated.acknowledged_at_unix_nano.is_some() {
                    IncidentStatus::Acknowledged
                } else {
                    IncidentStatus::Active
                };
                if let Err(err) = self.persistence.update_incident_status(id, &updated) {
                    tracing::warn!(
                        target: "triage.incident.persist.error",
                        error_category = err.error_category(),
                        persist_kind = "incident_auto_resolve",
                        "auto-resolve persist failed",
                    );
                }
                let _ = self.broadcast.sender().send(IncidentLifecycleEvent {
                    incident_id: id,
                    kind: updated.kind,
                    scope: updated.scope,
                    from_state,
                    to_state: IncidentStatus::Resolved,
                    transitioned_at_unix_nano: now_unix_nano,
                });
                resolved += 1;
            }
        }
        (evaluated, resolved)
    }
}

/// Long-running auto-resolution ticker task. Spawned at boot in
/// `pulse-app/src/main.rs`. Tick cadence per
/// `DEFAULT_AUTO_RESOLVE_TICK_INTERVAL` (30s default).
pub async fn run_auto_resolution_loop(observer: AutoResolveObserver) {
    let mut interval = tokio::time::interval(DEFAULT_AUTO_RESOLVE_TICK_INTERVAL);
    // Skip immediate first tick to mirror chunk #62/#63/#67 convention.
    interval.tick().await;
    loop {
        interval.tick().await;
        let tick_start = std::time::Instant::now();
        let now_nanos = current_unix_nanos();
        let (evaluated, resolved) = observer.run_one_tick(now_nanos);
        let duration_ms = tick_start.elapsed().as_millis() as u64;
        tracing::info!(
            target: TARGET_INCIDENT_AUTO_RESOLVE_TICK,
            evaluated_count = evaluated,
            resolved_count = resolved,
            duration_ms = duration_ms,
            "incident auto-resolution tick",
        );
    }
}

fn current_unix_nanos() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

// Tests live at `pulse-app/tests/e2e_incidents_lifecycle.rs` — integration
// test crate exercises the full lifecycle including auto-resolution with
// `tokio::time::pause()` + `tokio::time::advance(Duration::from_secs(125))`.
