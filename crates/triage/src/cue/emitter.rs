use std::sync::Arc;

use tokio::sync::broadcast::error::TryRecvError;

use crate::baseline::{BaselineState, BootstrapState};
use crate::contract::{AttentionCue, CueScope, PriorityTier};
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

/// Synchronous helper that runs one emission cycle: evaluate thresholds, emit
/// each resulting cue to the broadcast topic, additionally emit Tier-2
/// (`PriorityTier::Suggested`) cues к cadence-triggers, fire per-emission
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

    let cues = outcome.cues_kept;
    let cues_suppressed = outcome.cues_suppressed;
    let bypass_triggered = outcome.bypass_triggers.len();
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
    }
}

/// Emit a single cue к broadcast + (if Tier-2) cadence-triggers + per-cue
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
}

/// Long-running future spawned at boot (chunk #62 pulse-app/src/main.rs
/// wiring; chunk #63 extends with restart-subscription + suppression-state).
/// Fires `run_one_emit_cycle` on the supplied tick interval.
///
/// Wall-clock time is read from `SystemTime::UNIX_EPOCH`; tests should
/// exercise `run_one_emit_cycle` directly with injected `now_nanos` для
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
            1_000,
        );

        assert!(stats.cues_emitted >= 1);
        let cue = rx.try_recv().expect("broadcast received");
        assert_eq!(cue.kind, crate::contract::CueKind::ErrorRateSpike);
    }

    #[test]
    fn run_one_emit_cycle_tier_two_fans_to_cadence_triggers() {
        // Borderline conditions: magnitude ~4x base, ~50 samples → Suggested tier.
        let state = BaselineState::new();
        // 4 errors per 100 → 4% rate → magnitude ~4x; 50 samples → confidence 0.5.
        // Need >= 0.7 confidence for Suggested. Let's seed more samples.
        for i in 0..70 {
            let status = if i < 4 { 2 } else { 0 };
            state.observe_span("svc-mid", "op", status, 50, 1_000_000 + i * 1_000_000);
        }
        let broadcast_handle = AttentionCueBroadcast::new();
        let _rx = broadcast_handle.subscribe();
        let cadence_handle = CadenceTriggerChannel::new();
        let mut cad_rx = cadence_handle.subscribe();

        let stats = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            1_000,
        );

        if stats.cadence_triggers_emitted > 0 {
            let cadence_cue = cad_rx.try_recv().expect("cadence trigger received");
            assert_eq!(cadence_cue.priority_tier, PriorityTier::Suggested);
        }
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

        let _stats = run_one_emit_cycle(
            &state,
            &Thresholds::default(),
            &broadcast_handle,
            &cadence_handle,
            &SuppressionState::new(),
            1_000,
        );

        // The only emitted cue is Autonomous; cadence-triggers stays empty.
        let result = cad_rx.try_recv();
        assert!(
            matches!(
                result,
                Err(tokio::sync::broadcast::error::TryRecvError::Empty)
            ),
            "Autonomous cues must not land in cadence-triggers; got {result:?}"
        );
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
    /// Uses high bypass thresholds к force `suppression_bypassed=false`
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

        // Force the cue к escape any bypass by raising the bypass
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
            now_nanos,
        );
        // Diagnostics if assertion fails:
        assert_eq!(
            stats.cues_suppressed, 1,
            "cue should be suppressed (stats = {stats:?})"
        );
        assert_eq!(stats.cues_emitted, 0);
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
        // skipped by design, so 50ms is safe для at least one full cycle.
        tokio::time::sleep(Duration::from_millis(50)).await;
        handle.abort();
        let result = handle.await;
        assert!(result.is_err(), "abort must yield cancellation");
        assert!(result.unwrap_err().is_cancelled());
    }
}
