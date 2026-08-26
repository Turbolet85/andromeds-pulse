use std::collections::HashSet;
use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::broadcast::error::TryRecvError;

use crate::baseline::{BaselineState, BootstrapState};
use crate::contract::{AttentionCue, CueKind, CueScope, PriorityTier};
use crate::cue::broadcast::{AttentionCueBroadcast, CadenceTriggerChannel};
use crate::cue::classify::{cue_kind_label, priority_tier_label};
use crate::cue::evaluate::{evaluate_service_went_silent, evaluate_thresholds};
use crate::cue::thresholds::Thresholds;
use crate::cue::{
    TARGET_CUE_EMIT, TARGET_CUE_EVALUATE, TARGET_CUE_SUPPRESSION_BYPASS,
    TARGET_CUE_SUPPRESSION_CHECK, TARGET_CUE_TICK, TARGET_METRIC_BOOTSTRAP_STATE,
    TARGET_METRIC_CUE_EMIT_COUNT, TARGET_SERVICE_WENT_SILENT_EVALUATE,
};
use crate::pattern::{
    RestartEventBroadcast, SuppressionParams, SuppressionState, TARGET_METRIC_MAGNITUDE_BYPASS,
    evaluate_with_suppression,
};

/// Refractory interval for the cue re-emission latch — a condition that stays
/// live re-asserts at most this often.
///
/// Matched to the 60s tier-3 cadence baseline so a latched condition can never
/// go quieter than the ticker that runs anyway.
pub const CUE_LATCH_REFRACTORY_NANOS: i64 = 60 * 1_000_000_000;

/// Re-emission latch keyed on the cue-identity tuple `(kind, scope_id)`.
///
/// Cue evaluators re-derive from current state, so a condition that persists
/// yields an identical cue every tick. BOTH cadence entry points are fed by
/// emission — Suggested cues through `CadenceTriggerChannel`, Autonomous cues
/// through the general cue broadcast — so an unlatched emitter turns one live
/// condition into a cadence cycle per tick on each path, and each cycle drives
/// ten `spawn_blocking` L1a queries. Measured 2026-08-25: five silent services
/// produced 4,590 cues, 2,671 immediate cadence cycles and 26,821 L1a queries
/// in 16 minutes, against 220 in a matched healthy control.
///
/// The key is the tuple incidents already coalesce on (architecture.md
/// §Established Decisions [Fault Identity]), so what the latch refuses is what
/// the registry would have merged downstream anyway — the dedup moves ahead of
/// the expensive work rather than after it.
pub struct CueLatch {
    entries: DashMap<(CueKind, String), LatchEntry>,
    refractory_nanos: i64,
}

#[derive(Clone, Copy)]
struct LatchEntry {
    admitted_at_nanos: i64,
    tier_rank: u8,
}

/// One cycle's partition into cues that reach emission and those refused.
#[derive(Debug, Default)]
pub struct LatchOutcome {
    pub admitted: Vec<AttentionCue>,
    pub latched: usize,
    /// Live conditions the latch is tracking after this cycle's eviction pass.
    pub tracked: usize,
}

/// Escalation ordering — a cue only re-asserts early when it gets *worse*.
fn tier_rank(tier: PriorityTier) -> u8 {
    match tier {
        PriorityTier::Curious => 0,
        PriorityTier::Suggested => 1,
        PriorityTier::Autonomous => 2,
    }
}

fn latch_key(cue: &AttentionCue) -> (CueKind, String) {
    (cue.kind, cue.scope_id.clone().unwrap_or_default())
}

impl CueLatch {
    pub fn new() -> Self {
        Self::with_refractory_nanos(CUE_LATCH_REFRACTORY_NANOS)
    }

    /// Tests inject the refractory window; production uses [`CueLatch::new`].
    pub fn with_refractory_nanos(refractory_nanos: i64) -> Self {
        Self {
            entries: DashMap::new(),
            refractory_nanos,
        }
    }

    /// Partition one cycle's cues into those that reach emission and those the
    /// latch refuses.
    ///
    /// A cue is admitted when its condition is newly observed, when its
    /// priority tier has escalated since the last admission, or when the
    /// refractory window has elapsed. Conditions absent from `cues` have
    /// cleared and are evicted first, so a genuine recurrence admits
    /// immediately instead of waiting out the window.
    pub fn admit_cycle(&self, cues: Vec<AttentionCue>, now_nanos: i64) -> LatchOutcome {
        let live: HashSet<(CueKind, String)> = cues.iter().map(latch_key).collect();
        self.entries.retain(|key, _| live.contains(key));

        let mut admitted = Vec::with_capacity(cues.len());
        let mut latched = 0_usize;
        for cue in cues {
            if self.admit(latch_key(&cue), cue.priority_tier, now_nanos) {
                admitted.push(cue);
            } else {
                latched += 1;
            }
        }

        LatchOutcome {
            admitted,
            latched,
            tracked: self.entries.len(),
        }
    }

    fn admit(&self, key: (CueKind, String), tier: PriorityTier, now_nanos: i64) -> bool {
        let rank = tier_rank(tier);
        // Bind before matching so no DashMap read guard is held across insert.
        let existing = self.entries.get(&key).map(|entry| *entry);
        if let Some(entry) = existing {
            let within_refractory =
                now_nanos.saturating_sub(entry.admitted_at_nanos) < self.refractory_nanos;
            if rank <= entry.tier_rank && within_refractory {
                return false;
            }
        }
        self.entries.insert(
            key,
            LatchEntry {
                admitted_at_nanos: now_nanos,
                tier_rank: rank,
            },
        );
        true
    }
}

impl Default for CueLatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Synchronous helper that runs one emission cycle: evaluate thresholds, emit
/// each resulting cue to the broadcast topic, additionally emit Tier-2
/// (`PriorityTier::Suggested`) cues to cadence-triggers, fire per-emission
/// metric event. Returns the cues emitted so callers can assert on outcome.
///
/// Designed for direct unit-test invocation independent of the
/// `tokio::time::interval`-driven outer loop (per chunk #62 plan
/// Implementation Steps step 6 + chunk #20-#21 retention.rs precedent for
/// testable async-helper extraction).
pub fn run_one_emit_cycle(
    state: &BaselineState,
    thresholds: &Thresholds,
    broadcast_handle: &AttentionCueBroadcast,
    cadence_handle: &CadenceTriggerChannel,
    suppression_state: &SuppressionState,
    latch: &CueLatch,
    now_nanos: i64,
) -> EmitCycleStats {
    let services_tracked = state.service_count();
    let operations_tracked = state.operation_count();

    // Chunk #73 P-012: rotate the short-window (~30s effective) t-digest
    // pairs before evaluation so percentile queries see freshest data.
    // Internal age-check in `swap_on_tick` no-ops if elapsed <
    // DEFAULT_SHORT_SWAP_INTERVAL_NANOS (15s), making the 1Hz call safe.
    state.swap_short_tdigest_pairs_on_tick(now_nanos);

    tracing::info!(
        target: TARGET_CUE_EVALUATE,
        services_tracked = services_tracked as u64,
        operations_tracked = operations_tracked as u64,
        error_rate_multiplier = thresholds.error_rate_multiplier,
        latency_multiplier = thresholds.latency_multiplier,
        source = "default",
        "cue evaluation cycle",
    );

    let mut raw_cues = evaluate_thresholds(state, thresholds, now_nanos);
    let silence_cues = evaluate_service_went_silent(state, thresholds, now_nanos);
    let silence_cues_count = silence_cues.len();
    raw_cues.extend(silence_cues);

    // Aggregate bootstrap-state counts for chunk #64 observability — single
    // pass over the snapshots, fields are bounded-cardinality counts only
    // (no service.name per chunk #62/#63 PII discipline).
    let silence_snapshots = state.iter_service_silence_snapshots(now_nanos);
    let mut services_in_bootstrap = 0_u64;
    let mut services_ready = 0_u64;
    for snapshot in &silence_snapshots {
        match snapshot.bootstrap_state {
            BootstrapState::Learning => services_in_bootstrap += 1,
            BootstrapState::Ready => services_ready += 1,
        }
    }
    tracing::info!(
        target: TARGET_SERVICE_WENT_SILENT_EVALUATE,
        services_tracked = services_tracked as u64,
        services_in_bootstrap = services_in_bootstrap,
        services_ready = services_ready,
        silence_cues_emitted = silence_cues_count as u64,
        "service-went-silent evaluation cycle",
    );
    tracing::info!(
        target: TARGET_METRIC_BOOTSTRAP_STATE,
        value = services_ready,
        services_in_bootstrap = services_in_bootstrap,
        services_ready = services_ready,
        services_tracked = services_tracked as u64,
        "activity floor bootstrap state",
    );

    let cues_evaluated = services_tracked + operations_tracked + silence_snapshots.len();

    // Per-cue suppression decision logging — emit `triage.cue.suppression_check`
    // for each evaluated cue so the surgical-suppression posture (chunk #63
    // P-016) is observable. Decision = (in active restart window AND ErrorRateSpike
    // AND persistence < cutoff AND NOT suppression_bypassed → drop).
    for cue in &raw_cues {
        let service = cue.scope_id.as_deref().unwrap_or("");
        let restart_window_active =
            !service.is_empty() && suppression_state.is_active(service, now_nanos);
        tracing::info!(
            target: TARGET_CUE_SUPPRESSION_CHECK,
            cue_kind = cue_kind_label(cue.kind),
            persistence_seconds = cue.persistence_seconds,
            restart_window_active = restart_window_active,
            suppression_bypassed = cue.suppression_bypassed,
            bypass_reason = "none",
        );
    }

    let params = SuppressionParams {
        persistence_cutoff_seconds: thresholds.suppression_persistence_cutoff_seconds,
        magnitude_bypass_multiplier: thresholds.magnitude_bypass_multiplier,
        absolute_bypass_error_rate: thresholds.absolute_bypass_error_rate,
        absolute_bypass_latency_ms: thresholds.absolute_bypass_latency_ms,
    };
    let outcome = evaluate_with_suppression(raw_cues, suppression_state, &params, now_nanos);

    // Emit per-bypass-trigger events (P-057 dual-condition magnitude bypass).
    for trigger in &outcome.bypass_triggers {
        let kind_label = cue_kind_label(trigger.cue_kind);
        let reason_label = trigger.reason.label();
        tracing::info!(
            target: TARGET_CUE_SUPPRESSION_BYPASS,
            cue_kind = kind_label,
            bypass_reason = reason_label,
            "suppression bypassed by dual-condition",
        );
        tracing::info!(
            target: TARGET_METRIC_MAGNITUDE_BYPASS,
            value = 1_u64,
            reason = reason_label,
            cue_kind = kind_label,
            "magnitude bypass triggered",
        );
    }

    let cues_suppressed = outcome.cues_suppressed;
    let bypass_triggered = outcome.bypass_triggers.len();

    let latch_outcome = latch.admit_cycle(outcome.cues_kept, now_nanos);
    let cues = latch_outcome.admitted;
    let cues_latched = latch_outcome.latched;
    let latch_tracked = latch_outcome.tracked;
    let cues_emitted = cues.len();
    let mut cadence_emitted = 0_usize;

    for cue in &cues {
        emit_cue(cue, broadcast_handle, cadence_handle, &mut cadence_emitted);
    }

    tracing::info!(
        target: TARGET_CUE_TICK,
        cues_evaluated = cues_evaluated as u64,
        cues_emitted = cues_emitted as u64,
        cadence_triggers_emitted = cadence_emitted as u64,
        services_tracked = services_tracked as u64,
        operations_tracked = operations_tracked as u64,
        cues_suppressed = cues_suppressed as u64,
        bypass_triggered = bypass_triggered as u64,
        cues_latched = cues_latched as u64,
        latch_tracked = latch_tracked as u64,
        "heartbeat",
    );

    EmitCycleStats {
        cues_evaluated,
        cues_emitted,
        cadence_triggers_emitted: cadence_emitted,
        services_tracked,
        operations_tracked,
        cues_suppressed,
        bypass_triggered,
        cues_latched,
        latch_tracked,
    }
}

/// Emit a single cue to broadcast + (if Tier-2) cadence-triggers + per-cue
/// metric event. Bounded-cardinality fields only — `kind` + `priority` enum
/// labels + structural numeric values; NEVER `scope_id` content per security
/// plan §Anti-Patterns § Logging row 1 (cue scope_id may carry user-
/// controlled service.name).
fn emit_cue(
    cue: &AttentionCue,
    broadcast_handle: &AttentionCueBroadcast,
    cadence_handle: &CadenceTriggerChannel,
    cadence_emitted: &mut usize,
) {
    let kind_label = cue_kind_label(cue.kind);
    let priority_label = priority_tier_label(cue.priority_tier);
    let scope_label = cue_scope_label(cue.scope);

    // Best-effort send: ignore SendError (no subscribers is benign).
    let _ = broadcast_handle.sender().send(cue.clone());

    if matches!(cue.priority_tier, PriorityTier::Suggested) {
        let _ = cadence_handle.sender().send(cue.clone());
        *cadence_emitted += 1;
    }

    tracing::info!(
        target: TARGET_CUE_EMIT,
        kind = kind_label,
        priority = priority_label,
        scope = scope_label,
        magnitude = cue.magnitude,
        absolute_value = cue.absolute_value,
        persistence_seconds = cue.persistence_seconds,
        confidence = cue.confidence,
        suppression_bypassed = cue.suppression_bypassed,
        "attention cue emitted",
    );

    tracing::info!(
        target: TARGET_METRIC_CUE_EMIT_COUNT,
        value = 1_u64,
        kind = kind_label,
        priority = priority_label,
        scope = scope_label,
        "cue emit count",
    );
}

fn cue_scope_label(scope: CueScope) -> &'static str {
    match scope {
        CueScope::Service => "service",
        CueScope::Operation => "operation",
        CueScope::Global => "global",
    }
}

/// Per-cycle counters surfaced for tests + heartbeat fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EmitCycleStats {
    pub cues_evaluated: usize,
    pub cues_emitted: usize,
    pub cadence_triggers_emitted: usize,
    pub services_tracked: usize,
    pub operations_tracked: usize,
    /// Number of cues dropped by surgical-suppression filter (chunk #63).
    pub cues_suppressed: usize,
    /// Number of cues that survived suppression via dual-condition bypass
    /// (P-057; chunk #63). Each entry corresponds to one
    /// `metric.pipeline.l2.magnitude_bypass_triggered_total` event.
    pub bypass_triggered: usize,
    /// Cues refused by [`CueLatch`] because their condition was already
    /// signalled, had not escalated, and was inside the refractory window.
    pub cues_latched: usize,
    /// Live conditions the latch tracks — the cardinality that bounds how
    /// many cadence cycles one tick can drive.
    pub latch_tracked: usize,
}

/// Long-running future spawned at boot (chunk #62 pulse-app/src/main.rs
/// wiring; chunk #63 extends with restart-subscription + suppression-state).
/// Fires `run_one_emit_cycle` on the supplied tick interval.
///
/// Wall-clock time is read from `SystemTime::UNIX_EPOCH`; tests should
/// exercise `run_one_emit_cycle` directly with injected `now_nanos` for
/// deterministic timing (per chunk #61 + #20 testable-helper-extraction
/// precedent).
///
/// Chunk #63 additions: each tick drains pending `RestartEvent`s from the
/// `restart_broadcast` subscription into `suppression_state` (recording
/// suppression windows), then `run_one_emit_cycle` applies surgical
/// suppression to the evaluated cues before emit.
pub async fn start_emitter(
    state: Arc<BaselineState>,
    broadcast_handle: Arc<AttentionCueBroadcast>,
    cadence_handle: Arc<CadenceTriggerChannel>,
    thresholds: Arc<Thresholds>,
    restart_broadcast: Arc<RestartEventBroadcast>,
    suppression_state: Arc<SuppressionState>,
) {
    let mut interval = tokio::time::interval(thresholds.tick_interval);
    let mut restart_rx = restart_broadcast.subscribe();
    let latch = CueLatch::new();
    // Skip immediate first tick to avoid sweeping at startup with no data —
    // mirrors chunk #20 retention loop + chunk #61 persist loop precedent.
    interval.tick().await;
    loop {
        interval.tick().await;
        drain_restart_events(
            &mut restart_rx,
            &restart_broadcast,
            &suppression_state,
            thresholds.restart_suppression_window_seconds,
        );
        let now = current_unix_nanos();
        let _ = run_one_emit_cycle(
            &state,
            &thresholds,
            &broadcast_handle,
            &cadence_handle,
            &suppression_state,
            &latch,
            now,
        );
    }
}

/// Drain pending `RestartEvent`s from the broadcast subscription into
/// `SuppressionState`. Non-blocking — uses `try_recv()` so no events
/// arriving this tick is benign. On `RecvError::Lagged`, re-subscribe to
/// drop the backlog + log a warning (matches tokio broadcast best
/// practice when the receiver falls behind the broadcast capacity).
fn drain_restart_events(
    rx: &mut tokio::sync::broadcast::Receiver<crate::pattern::RestartEvent>,
    broadcast: &Arc<RestartEventBroadcast>,
    suppression_state: &SuppressionState,
    window_seconds: u64,
) {
    loop {
        match rx.try_recv() {
            Ok(event) => {
                suppression_state.record_restart(&event, window_seconds);
            }
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Closed) => return,
            Err(TryRecvError::Lagged(skipped)) => {
                tracing::warn!(
                    target: "triage.cue.suppression_check",
                    skipped_events = skipped,
                    "restart event subscription lagged; backlog dropped",
                );
                *rx = broadcast.subscribe();
                return;
            }
        }
    }
}

fn current_unix_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::time::Duration;
    use tracing::field::{Field, Visit};
    use tracing::{Event, Level, Subscriber};

    fn seed_error_spike_service(state: &BaselineState, service: &str) {
        // Chunk #73 P-010 baseline-relative semantics: long-term baseline
        // at 0% errors (first 50 obs), then short-term spike at 100% errors
        // (last 50 obs). Short EWMA (α≈0.0333) converges quickly toward
        // the spike value; long EWMA (α≈0.00333) barely moves; ratio
        // short/long > 3.0× triggers ErrorRateSpike cue.
        for i in 0..100 {
            let status = if i >= 50 { 2 } else { 0 };
            state.observe_span(service, "op-x", status, 50, 1_000_000 + i * 1_000_000);
        }
    }

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
            let target = event.metadata().target().to_string();
            let level = *event.metadata().level();
            let mut collector = FieldCollector(Vec::new());
            event.record(&mut collector);
            self.events
                .lock()
                .unwrap()
                .push((target, level, collector.0));
        }
        fn enter(&self, _: &tracing::Id) {}
        fn exit(&self, _: &tracing::Id) {}
    }

    #[test]
    fn run_one_emit_cycle_emits_tick_event_with_zero_cues_on_empty_state() {
        let (sub, events) = CapturingSubscriber::new();
        let state = BaselineState::new();
        let broadcast_handle = AttentionCueBroadcast::new();
        let cadence_handle = CadenceTriggerChannel::new();

        let stats = tracing::subscriber::with_default(sub, || {
            run_one_emit_cycle(
                &state,
                &Thresholds::default(),
                &broadcast_handle,
                &cadence_handle,
                &SuppressionState::new(),
                &CueLatch::new(),
                1_000,
            )
        });

        assert_eq!(stats.cues_emitted, 0);
        let captured = events.lock().unwrap();
        let tick_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_CUE_TICK)
            .collect();
        assert_eq!(tick_events.len(), 1, "expected exactly 1 tick event");
    }

    #[test]
    fn run_one_emit_cycle_emits_metric_per_cue() {
        let (sub, events) = CapturingSubscriber::new();
        let state = BaselineState::new();
        seed_error_spike_service(&state, "svc-checkout");
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe(); // keep receiver alive
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        let stats = tracing::subscriber::with_default(sub, || {
            run_one_emit_cycle(
                &state,
                &Thresholds::default(),
                &broadcast_handle,
                &cadence_handle,
                &SuppressionState::new(),
                &CueLatch::new(),
                1_000,
            )
        });

        assert!(stats.cues_emitted >= 1);
        let captured = events.lock().unwrap();
        let metric_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_METRIC_CUE_EMIT_COUNT)
            .collect();
        assert!(
            !metric_events.is_empty(),
            "expected at least one metric emission, got {metric_events:?}"
        );
        // Verify bounded-cardinality labels.
        for (_, _, fields) in metric_events.iter() {
            assert!(
                fields.iter().any(|(k, _)| k == "kind"),
                "metric event missing kind field"
            );
            assert!(
                fields.iter().any(|(k, _)| k == "priority"),
                "metric event missing priority field"
            );
        }
    }

    #[test]
    fn run_one_emit_cycle_emits_cue_to_broadcast_topic() {
        let state = BaselineState::new();
        seed_error_spike_service(&state, "svc-checkout");
        let broadcast_handle = AttentionCueBroadcast::new();
        let mut rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        let stats = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            &CueLatch::new(),
            1_000,
        );

        assert!(stats.cues_emitted >= 1);
        let cue = rx.try_recv().expect("broadcast received");
        assert_eq!(cue.kind, crate::contract::CueKind::ErrorRateSpike);
    }

    #[test]
    fn run_one_emit_cycle_tier_two_fans_to_cadence_triggers() {
        // Clean baseline, then a spike long enough for the long EWMA to
        // partly catch up: short/long lands in [3.0, 5.0) → Suggested rather
        // than the Autonomous tier a 50-observation spike produces.
        //
        // The prior seeding placed its errors FIRST and then 66 clean spans,
        // which is a recovery, not a spike — short/long fell below 1.0, no cue
        // was produced, and this test's `if cadence_triggers_emitted > 0` guard
        // never ran. Measured at 2026-08-26-cadence-runaway-blocking-pool.
        let state = BaselineState::new();
        for i in 0..50_i64 {
            state.observe_span("svc-mid", "op", 0, 50, 1_000_000 + i * 1_000_000);
        }
        for i in 0..100_i64 {
            state.observe_span("svc-mid", "op", 2, 50, 51_000_000 + i * 1_000_000);
        }
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let mut cad_rx = cadence_handle.subscribe();

        let latch = CueLatch::new();
        let first = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            &latch,
            1_000,
        );

        assert!(
            first.cues_emitted >= 1,
            "first observation must emit; got {first:?}"
        );
        assert_eq!(
            first.cadence_triggers_emitted, 1,
            "a Suggested cue must fan to cadence-triggers; got {first:?}"
        );
        let cadence_cue = cad_rx.try_recv().expect("cadence trigger received");
        assert_eq!(cadence_cue.priority_tier, PriorityTier::Suggested);

        // Re-pointed to the bound: the SAME condition at the SAME tier inside
        // the refractory window is refused, so a persistent fault drives one
        // cadence cycle rather than one per tick.
        let second = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            &latch,
            2_000,
        );

        assert_eq!(second.cues_emitted, 0, "repeat must not re-emit");
        assert_eq!(second.cues_latched, first.cues_emitted);
        assert_eq!(second.cadence_triggers_emitted, 0);
    }

    #[test]
    fn run_one_emit_cycle_autonomous_tier_does_not_fan_to_cadence_triggers() {
        let state = BaselineState::new();
        // 50% errors × 100 samples → Autonomous tier (high magnitude + high confidence + persistence).
        seed_error_spike_service(&state, "svc-severe");
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let mut cad_rx = cadence_handle.subscribe();

        let latch = CueLatch::new();
        let first = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            &latch,
            1_000,
        );

        // The only emitted cue is Autonomous; cadence-triggers stays empty.
        assert!(first.cues_emitted >= 1, "first observation must emit");
        let result = cad_rx.try_recv();
        assert!(
            matches!(
                result,
                Err(tokio::sync::broadcast::error::TryRecvError::Empty)
            ),
            "Autonomous cues must not land in cadence-triggers; got {result:?}"
        );

        // Re-pointed to the bound: Autonomous cues reach the coordinator over
        // the general cue broadcast (the tier-1 path that produced 2,065 of
        // the wedge's 2,683 triggers), so the latch must refuse the repeat
        // there too — a cadence-only bound would have missed this path.
        let second = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            &latch,
            2_000,
        );

        assert_eq!(
            second.cues_emitted, 0,
            "repeat Autonomous cue must not reach the broadcast again"
        );
        assert_eq!(second.cues_latched, first.cues_emitted);
    }

    #[test]
    fn run_one_emit_cycle_pii_canary_does_not_leak_into_tracing_fields() {
        let canary = "secret-canary-API-key-12345";
        let (sub, events) = CapturingSubscriber::new();
        let state = BaselineState::new();
        // Synthesize spans whose service.name contains the canary.
        for i in 0..100 {
            let status = if i < 50 { 2 } else { 0 };
            state.observe_span(canary, "op", status, 50, 1_000_000 + i * 1_000_000);
        }
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        tracing::subscriber::with_default(sub, || {
            run_one_emit_cycle(
                &state,
                &Thresholds::default(),
                &broadcast_handle,
                &cadence_handle,
                &SuppressionState::new(),
                &CueLatch::new(),
                1_000,
            )
        });

        let captured = events.lock().unwrap();
        for (target, _level, fields) in captured.iter() {
            // The cue payload broadcast on pulse://stream/attention-cues IS expected
            // to contain scope_id (which is the canary here) — but the BROADCAST is
            // product-feature surface, not self-observation. The self-observation
            // tracing events (this captured set) must NOT carry the canary.
            assert!(
                !target.contains(canary),
                "canary leaked into target: {target}"
            );
            for (k, v) in fields {
                assert!(
                    !v.contains(canary),
                    "canary leaked into field value at target={target} key={k}: {v}"
                );
                assert!(
                    !k.contains(canary),
                    "canary leaked into field name at target={target}: {k}"
                );
            }
        }
    }

    #[test]
    fn run_one_emit_cycle_emits_evaluation_span_with_threshold_fields() {
        let (sub, events) = CapturingSubscriber::new();
        let state = BaselineState::new();
        let broadcast_handle = AttentionCueBroadcast::new();
        let cadence_handle = CadenceTriggerChannel::new();

        tracing::subscriber::with_default(sub, || {
            run_one_emit_cycle(
                &state,
                &Thresholds::default(),
                &broadcast_handle,
                &cadence_handle,
                &SuppressionState::new(),
                &CueLatch::new(),
                1_000,
            )
        });

        let captured = events.lock().unwrap();
        let evaluate_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_CUE_EVALUATE)
            .collect();
        assert_eq!(evaluate_events.len(), 1);
        let (_, _, fields) = evaluate_events[0];
        let multiplier_field = fields.iter().find(|(k, _)| k == "error_rate_multiplier");
        assert!(
            multiplier_field.is_some(),
            "missing error_rate_multiplier field"
        );
        let source = fields.iter().find(|(k, _)| k == "source");
        assert_eq!(source.map(|(_, v)| v.as_str()), Some("default"));
    }

    /// Integration: with an active restart suppression window for the
    /// affected service, a short-persistence ErrorRateSpike cue is dropped
    /// from the emit cycle (chunk #63 P-016 surgical suppression).
    /// Uses high bypass thresholds to force `suppression_bypassed=false`
    /// so the surgical-drop branch fires.
    #[test]
    fn run_one_emit_cycle_drops_short_persistence_error_spike_in_active_window() {
        let state = BaselineState::new();
        // Chunk #73 P-010 baseline-relative: seed 12 zeros (baseline) then
        // 13 errors (recent spike) so short EWMA > long EWMA × multiplier.
        // 25 samples < `suppression_persistence_cutoff_seconds` (30) → cue
        // is suppression-eligible during the active restart window.
        for i in 0..25 {
            let status = if i >= 12 { 2 } else { 0 };
            state.observe_span("svc-restarted", "op", status, 50, 1_000_000 + i * 1_000_000);
        }
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        // Force the cue to escape any bypass by raising the bypass
        // thresholds far above the cue's expected magnitude+absolute.
        let high_thresholds = Thresholds {
            magnitude_bypass_multiplier: 1_000.0,
            absolute_bypass_error_rate: 0.99,
            ..Thresholds::default()
        };
        let suppression = SuppressionState::new();
        let now_nanos = 1_000_000_000_000_i64;
        let restart_event = crate::pattern::RestartEvent {
            service: "svc-restarted".to_string(),
            gap_seconds: 25,
            last_seen_unix_nano: now_nanos - 25_000_000_000,
            resume_unix_nano: now_nanos,
        };
        suppression.record_restart(&restart_event, 60);
        assert!(
            suppression.is_active("svc-restarted", now_nanos),
            "window should be active at resume time"
        );

        let stats = run_one_emit_cycle(
            &state,
            &high_thresholds,
            &broadcast_handle,
            &cadence_handle,
            &suppression,
            &CueLatch::new(),
            now_nanos,
        );
        // Diagnostics if assertion fails:
        assert_eq!(
            stats.cues_suppressed, 1,
            "cue should be suppressed (stats = {stats:?})"
        );
        assert_eq!(stats.cues_emitted, 0);
    }

    /// Integration: the P-016 POSITIVE case — a SUSTAINED (>30s
    /// persistence) ErrorRateSpike SURFACES inside the active 60s restart
    /// window. Suppression only drops short-persistence cues
    /// (`persistence_seconds < suppression_persistence_cutoff_seconds`);
    /// long-persistence cues are real signals, not restart-induced noise.
    /// Bypass thresholds are raised so survival is attributable to
    /// persistence alone (not the P-057 magnitude bypass).
    #[test]
    fn run_one_emit_cycle_surfaces_sustained_spike_inside_active_restart_window() {
        let state = BaselineState::new();
        // 40 samples (12 baseline zeros + 28 tail errors) at 1s spacing →
        // persistence_seconds = 40 ≥ cutoff (30) → suppression-INELIGIBLE.
        for i in 0..40 {
            let status = if i >= 12 { 2 } else { 0 };
            state.observe_span("svc-restarted", "op", status, 50, 1_000_000 + i * 1_000_000);
        }
        let broadcast_handle = AttentionCueBroadcast::new();
        let mut rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        let high_thresholds = Thresholds {
            magnitude_bypass_multiplier: 1_000.0,
            absolute_bypass_error_rate: 0.99,
            ..Thresholds::default()
        };
        let suppression = SuppressionState::new();
        let now_nanos = 1_000_000_000_000_i64;
        let restart_event = crate::pattern::RestartEvent {
            service: "svc-restarted".to_string(),
            gap_seconds: 25,
            last_seen_unix_nano: now_nanos - 25_000_000_000,
            resume_unix_nano: now_nanos,
        };
        suppression.record_restart(&restart_event, 60);
        assert!(
            suppression.is_active("svc-restarted", now_nanos),
            "window should be active at resume time"
        );

        let stats = run_one_emit_cycle(
            &state,
            &high_thresholds,
            &broadcast_handle,
            &cadence_handle,
            &suppression,
            &CueLatch::new(),
            now_nanos,
        );
        assert_eq!(
            stats.cues_suppressed, 0,
            "sustained >30s spike must NOT be suppressed (stats = {stats:?})"
        );
        assert!(
            stats.cues_emitted >= 1,
            "sustained spike must SURFACE inside the active window (stats = {stats:?})"
        );
        let cue = rx.try_recv().expect("broadcast carries the surfaced cue");
        assert_eq!(cue.kind, crate::contract::CueKind::ErrorRateSpike);
        assert_eq!(cue.scope_id.as_deref(), Some("svc-restarted"));
        assert!(
            cue.persistence_seconds >= 30,
            "persistence must be at/above the suppression cutoff; got {}",
            cue.persistence_seconds
        );
        assert!(
            !cue.suppression_bypassed,
            "survival must be via persistence, not the P-057 bypass"
        );
    }

    /// Integration: a cue with `suppression_bypassed: true` survives the
    /// suppression filter during an active window AND fires the
    /// `metric.pipeline.l2.magnitude_bypass_triggered_total` event.
    #[test]
    fn run_one_emit_cycle_keeps_high_magnitude_spike_in_active_window_via_bypass() {
        let (sub, events) = CapturingSubscriber::new();
        let state = BaselineState::new();
        // 50% errors × 100 samples → Autonomous tier with very high
        // magnitude → suppression_bypassed=true via Relative bypass at
        // default Thresholds (magnitude>>10).
        seed_error_spike_service(&state, "svc-severe");
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();
        let suppression = SuppressionState::new();
        let restart_event = crate::pattern::RestartEvent {
            service: "svc-severe".to_string(),
            gap_seconds: 25,
            last_seen_unix_nano: 975 * 1_000_000_000,
            resume_unix_nano: 1_000 * 1_000_000_000,
        };
        suppression.record_restart(&restart_event, 60);

        // Note: persistence > 30s cutoff (=100), so suppression-eligibility
        // is FALSE — the cue would survive even without bypass. Force
        // short persistence by seeding fewer samples.
        // Chunk #73 P-010 baseline-relative: zeros first then errors so
        // short EWMA converges to the recent spike + long stays low.
        let state_short = BaselineState::new();
        for i in 0..25 {
            let status = if i >= 5 { 2 } else { 0 };
            state_short.observe_span("svc-severe", "op", status, 50, 1_000_000 + i * 1_000_000);
        }

        let stats = tracing::subscriber::with_default(sub, || {
            run_one_emit_cycle(
                &state_short,
                &Thresholds::default(),
                &broadcast_handle,
                &cadence_handle,
                &suppression,
                &CueLatch::new(),
                1_000 * 1_000_000_000,
            )
        });

        assert!(stats.bypass_triggered >= 1, "bypass should trigger");
        assert!(stats.cues_emitted >= 1, "bypassed cue should emit");

        let captured = events.lock().unwrap();
        let bypass_metrics: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == "metric.pipeline.l2.magnitude_bypass_triggered_total")
            .collect();
        assert!(
            !bypass_metrics.is_empty(),
            "magnitude_bypass_triggered_total metric should fire"
        );
        let (_, _, fields) = bypass_metrics[0];
        assert!(fields.iter().any(|(k, _)| k == "reason"));
        assert!(fields.iter().any(|(k, _)| k == "cue_kind"));
        assert!(fields.iter().any(|(k, _)| k == "value"));
    }

    /// Integration: emit cycle fires `triage.cue.suppression_check` per
    /// evaluated cue with the required field set per AllowList entry.
    #[test]
    fn run_one_emit_cycle_emits_suppression_check_per_cue() {
        let (sub, events) = CapturingSubscriber::new();
        let state = BaselineState::new();
        seed_error_spike_service(&state, "svc-a");
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        tracing::subscriber::with_default(sub, || {
            run_one_emit_cycle(
                &state,
                &Thresholds::default(),
                &broadcast_handle,
                &cadence_handle,
                &SuppressionState::new(),
                &CueLatch::new(),
                1_000,
            )
        });

        let captured = events.lock().unwrap();
        let check_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == "triage.cue.suppression_check")
            .collect();
        assert!(
            !check_events.is_empty(),
            "suppression_check must fire per cue"
        );
        let (_, _, fields) = check_events[0];
        for required in [
            "cue_kind",
            "persistence_seconds",
            "restart_window_active",
            "suppression_bypassed",
            "bypass_reason",
        ] {
            assert!(
                fields.iter().any(|(k, _)| k == required),
                "suppression_check missing field `{required}`"
            );
        }
    }

    /// Integration: a service that has passed its bootstrap window and gone
    /// quiet beyond its learned p95 quiet duration produces a ServiceWentSilent
    /// cue through `run_one_emit_cycle` end-to-end (chunk #64).
    #[test]
    fn run_one_emit_cycle_emits_service_went_silent_cue_post_bootstrap() {
        const NANOS_PER_SEC: i64 = 1_000_000_000;
        let state = BaselineState::new();
        // Seed 70 observations at 60s intervals → first=0, last=4140s,
        // p95~60s. Bootstrap requires >3600s elapsed since first.
        for i in 0..70_i64 {
            state.observe_span("svc-quiet", "op", 0, 50, (i * 60) * NANOS_PER_SEC);
        }
        let last = 69 * 60 * NANOS_PER_SEC;
        // Advance now to 600s past last → current_quiet ~600s >> p95 ~60s.
        let now = last + 600 * NANOS_PER_SEC;
        let broadcast_handle = AttentionCueBroadcast::new();
        let mut rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        let stats = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            &CueLatch::new(),
            now,
        );
        assert!(stats.cues_emitted >= 1);
        let mut saw_silent = false;
        while let Ok(cue) = rx.try_recv() {
            if cue.kind == crate::contract::CueKind::ServiceWentSilent {
                saw_silent = true;
                assert_eq!(cue.scope_id.as_deref(), Some("svc-quiet"));
                assert!(!cue.suppression_bypassed);
                break;
            }
        }
        assert!(
            saw_silent,
            "expected at least one ServiceWentSilent cue on broadcast"
        );
    }

    /// Integration: bootstrap suppression holds end-to-end through the emit
    /// cycle (chunk #64) — services still inside their 1h bootstrap window
    /// do not produce ServiceWentSilent cues regardless of quiet duration.
    #[test]
    fn run_one_emit_cycle_does_not_emit_service_went_silent_during_bootstrap() {
        const NANOS_PER_SEC: i64 = 1_000_000_000;
        let state = BaselineState::new();
        let first = 1_000 * NANOS_PER_SEC;
        for i in 0..20_i64 {
            state.observe_span("svc-fresh", "op", 0, 50, first + (i * 60) * NANOS_PER_SEC);
        }
        // now still inside bootstrap window (2000s elapsed vs 3600s required).
        let now = first + 2_000 * NANOS_PER_SEC;
        let broadcast_handle = AttentionCueBroadcast::new();
        let mut rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let _cad_rx = cadence_handle.subscribe();

        run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            &CueLatch::new(),
            now,
        );
        while let Ok(cue) = rx.try_recv() {
            assert_ne!(
                cue.kind,
                crate::contract::CueKind::ServiceWentSilent,
                "no ServiceWentSilent cue during bootstrap window"
            );
        }
    }

    /// Integration: the chunk #64 aggregate evaluation event fires per tick
    /// with bounded-cardinality count fields (no per-service identifiers per
    /// chunk #62/#63 PII discipline).
    #[test]
    fn run_one_emit_cycle_emits_service_went_silent_evaluate_aggregate_event() {
        let (sub, events) = CapturingSubscriber::new();
        let state = BaselineState::new();
        let broadcast_handle = AttentionCueBroadcast::new();
        let cadence_handle = CadenceTriggerChannel::new();

        tracing::subscriber::with_default(sub, || {
            run_one_emit_cycle(
                &state,
                &Thresholds::default(),
                &broadcast_handle,
                &cadence_handle,
                &SuppressionState::new(),
                &CueLatch::new(),
                1_000,
            )
        });

        let captured = events.lock().unwrap();
        let aggregate_events: Vec<&CapturedEvent> = captured
            .iter()
            .filter(|(t, _, _)| t == TARGET_SERVICE_WENT_SILENT_EVALUATE)
            .collect();
        assert_eq!(aggregate_events.len(), 1);
        let (_, _, fields) = aggregate_events[0];
        for required in [
            "services_tracked",
            "services_in_bootstrap",
            "services_ready",
            "silence_cues_emitted",
        ] {
            assert!(
                fields.iter().any(|(k, _)| k == required),
                "evaluate aggregate event missing field `{required}`"
            );
        }
        // PII guard: no service.name / scope_id / span_id / trace_id / operation_name
        // in either keys or values.
        let banned = [
            "service_name",
            "scope_id",
            "span_id",
            "trace_id",
            "operation_name",
        ];
        for (k, v) in fields {
            assert!(
                !banned.contains(&k.as_str()),
                "PII key {k:?} leaked into aggregate event"
            );
            assert!(!v.contains("svc-"), "PII value leaked: {v:?}");
        }
    }

    #[tokio::test]
    async fn start_emitter_spawnable_and_abortable() {
        // Short real-time interval (no tokio test-util dep in triage; per
        // testing.md Session Additions 2026-05-03 — drop start_paused when
        // not strictly needed).
        let state = Arc::new(BaselineState::new());
        let broadcast_handle = Arc::new(AttentionCueBroadcast::new());
        let cadence_handle = Arc::new(CadenceTriggerChannel::new());
        let thresholds = Arc::new(Thresholds {
            tick_interval: Duration::from_millis(10),
            ..Thresholds::default()
        });

        let restart_broadcast = Arc::new(RestartEventBroadcast::new());
        let suppression_state = Arc::new(SuppressionState::new());
        let handle = tokio::spawn(start_emitter(
            Arc::clone(&state),
            Arc::clone(&broadcast_handle),
            Arc::clone(&cadence_handle),
            Arc::clone(&thresholds),
            Arc::clone(&restart_broadcast),
            Arc::clone(&suppression_state),
        ));
        // Real-time short wait — emitter ticks every 10ms; first tick is
        // skipped by design, so 50ms is safe for at least one full cycle.
        tokio::time::sleep(Duration::from_millis(50)).await;
        handle.abort();
        let result = handle.await;
        assert!(result.is_err(), "abort must yield cancellation");
        assert!(result.unwrap_err().is_cancelled());
    }

    const TEST_REFRACTORY_NANOS: i64 = 1_000;

    fn latch_cue(kind: CueKind, service: &str, tier: PriorityTier) -> AttentionCue {
        AttentionCue {
            kind,
            scope: CueScope::Service,
            scope_id: Some(service.to_string()),
            magnitude: 2.0,
            absolute_value: 1.0,
            persistence_seconds: 30,
            confidence: 1.0,
            priority_tier: tier,
            suppression_bypassed: false,
            fingerprint: None,
        }
    }

    fn silent(service: &str) -> AttentionCue {
        latch_cue(CueKind::ServiceWentSilent, service, PriorityTier::Suggested)
    }

    #[test]
    fn cue_latch_admits_a_newly_observed_condition() {
        let latch = CueLatch::with_refractory_nanos(TEST_REFRACTORY_NANOS);

        let out = latch.admit_cycle(vec![silent("svc-a")], 10_000);

        assert_eq!(out.admitted.len(), 1);
        assert_eq!(out.latched, 0);
        assert_eq!(out.tracked, 1);
    }

    #[test]
    fn cue_latch_refuses_an_unchanged_repeat_inside_the_refractory_window() {
        let latch = CueLatch::with_refractory_nanos(TEST_REFRACTORY_NANOS);
        assert_eq!(
            latch
                .admit_cycle(vec![silent("svc-a")], 10_000)
                .admitted
                .len(),
            1
        );

        let out = latch.admit_cycle(vec![silent("svc-a")], 10_500);

        assert!(
            out.admitted.is_empty(),
            "repeat inside the window is refused"
        );
        assert_eq!(out.latched, 1);
        assert_eq!(out.tracked, 1);
    }

    #[test]
    fn cue_latch_admits_immediately_when_the_tier_escalates() {
        // The acceleration guard: a worsening condition reaches the
        // coordinator on the tick it worsens, never after the window.
        let latch = CueLatch::with_refractory_nanos(TEST_REFRACTORY_NANOS);
        latch.admit_cycle(
            vec![latch_cue(
                CueKind::ErrorRateSpike,
                "svc-a",
                PriorityTier::Curious,
            )],
            10_000,
        );

        let escalated = latch.admit_cycle(
            vec![latch_cue(
                CueKind::ErrorRateSpike,
                "svc-a",
                PriorityTier::Suggested,
            )],
            10_001,
        );
        let autonomous = latch.admit_cycle(
            vec![latch_cue(
                CueKind::ErrorRateSpike,
                "svc-a",
                PriorityTier::Autonomous,
            )],
            10_002,
        );

        assert_eq!(escalated.admitted.len(), 1, "Suggested escalation admits");
        assert_eq!(autonomous.admitted.len(), 1, "Autonomous escalation admits");
    }

    #[test]
    fn cue_latch_refuses_a_de_escalation_inside_the_window() {
        let latch = CueLatch::with_refractory_nanos(TEST_REFRACTORY_NANOS);
        latch.admit_cycle(
            vec![latch_cue(
                CueKind::ErrorRateSpike,
                "svc-a",
                PriorityTier::Autonomous,
            )],
            10_000,
        );

        let out = latch.admit_cycle(
            vec![latch_cue(
                CueKind::ErrorRateSpike,
                "svc-a",
                PriorityTier::Curious,
            )],
            10_100,
        );

        assert!(out.admitted.is_empty(), "a calmer repeat is not news");
        assert_eq!(out.latched, 1);
    }

    #[test]
    fn cue_latch_admits_again_after_the_refractory_window_elapses() {
        let latch = CueLatch::with_refractory_nanos(TEST_REFRACTORY_NANOS);
        latch.admit_cycle(vec![silent("svc-a")], 10_000);

        let inside = latch.admit_cycle(vec![silent("svc-a")], 10_000 + TEST_REFRACTORY_NANOS - 1);
        let elapsed = latch.admit_cycle(vec![silent("svc-a")], 10_000 + TEST_REFRACTORY_NANOS);

        assert!(inside.admitted.is_empty());
        assert_eq!(elapsed.admitted.len(), 1, "a live condition re-asserts");
    }

    #[test]
    fn cue_latch_evicts_a_cleared_condition_so_recurrence_admits_immediately() {
        let latch = CueLatch::with_refractory_nanos(TEST_REFRACTORY_NANOS);
        latch.admit_cycle(vec![silent("svc-a")], 10_000);

        let cleared = latch.admit_cycle(Vec::new(), 10_100);
        let recurrence = latch.admit_cycle(vec![silent("svc-a")], 10_200);

        assert_eq!(cleared.tracked, 0, "a cleared condition is evicted");
        assert_eq!(
            recurrence.admitted.len(),
            1,
            "recurrence is a new event, not a repeat"
        );
    }

    #[test]
    fn cue_latch_keys_on_kind_and_scope_so_distinct_conditions_stay_independent() {
        let latch = CueLatch::with_refractory_nanos(TEST_REFRACTORY_NANOS);
        let cycle = || {
            vec![
                silent("svc-a"),
                silent("svc-b"),
                latch_cue(CueKind::ErrorRateSpike, "svc-a", PriorityTier::Suggested),
            ]
        };

        let first = latch.admit_cycle(cycle(), 10_000);
        let repeat = latch.admit_cycle(cycle(), 10_100);

        assert_eq!(first.admitted.len(), 3, "distinct keys are independent");
        assert_eq!(first.tracked, 3);
        assert_eq!(repeat.latched, 3);
    }

    #[test]
    fn cue_latch_bounds_the_measured_wedge_shape() {
        // 2026-08-25: five services silent, one cue each per 1s tick. Unlatched
        // that is 300 cues/min driving 300 cadence cycles; latched it is one
        // admission per condition per refractory window.
        let latch = CueLatch::with_refractory_nanos(CUE_LATCH_REFRACTORY_NANOS);
        let services = ["svc-a", "svc-b", "svc-c", "svc-d", "svc-e"];
        let cycle = || services.iter().map(|s| silent(s)).collect::<Vec<_>>();

        let mut admitted_total = 0_usize;
        for tick in 0..60_i64 {
            admitted_total += latch
                .admit_cycle(cycle(), tick * 1_000_000_000)
                .admitted
                .len();
        }

        assert_eq!(
            admitted_total, 5,
            "one admission per condition per window, not per tick"
        );
    }
}
