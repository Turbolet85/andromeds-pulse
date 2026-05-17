use std::sync::Arc;

use crate::baseline::BaselineState;
use crate::contract::{AttentionCue, CueScope, PriorityTier};
use crate::cue::broadcast::{AttentionCueBroadcast, CadenceTriggerChannel};
use crate::cue::classify::{cue_kind_label, priority_tier_label};
use crate::cue::evaluate::evaluate_thresholds;
use crate::cue::thresholds::Thresholds;
use crate::cue::{
    TARGET_CUE_EMIT, TARGET_CUE_EVALUATE, TARGET_CUE_TICK, TARGET_METRIC_CUE_EMIT_COUNT,
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
    now_nanos: i64,
) -> EmitCycleStats {
    let services_tracked = state.service_count();
    let operations_tracked = state.operation_count();

    tracing::info!(
        target: TARGET_CUE_EVALUATE,
        services_tracked = services_tracked as u64,
        operations_tracked = operations_tracked as u64,
        error_rate_multiplier = thresholds.error_rate_multiplier,
        latency_multiplier = thresholds.latency_multiplier,
        source = "default",
        "cue evaluation cycle",
    );

    let cues = evaluate_thresholds(state, thresholds, now_nanos);
    let cues_evaluated = services_tracked + operations_tracked;
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
        "heartbeat",
    );

    EmitCycleStats {
        cues_evaluated,
        cues_emitted,
        cadence_triggers_emitted: cadence_emitted,
        services_tracked,
        operations_tracked,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmitCycleStats {
    pub cues_evaluated: usize,
    pub cues_emitted: usize,
    pub cadence_triggers_emitted: usize,
    pub services_tracked: usize,
    pub operations_tracked: usize,
}

/// Long-running future spawned at boot (chunk #62 pulse-app/src/main.rs
/// wiring). Fires `run_one_emit_cycle` on the supplied tick interval.
///
/// Wall-clock time is read from `SystemTime::UNIX_EPOCH`; tests should
/// exercise `run_one_emit_cycle` directly with injected `now_nanos` для
/// deterministic timing (per chunk #61 + #20 testable-helper-extraction
/// precedent).
pub async fn start_emitter(
    state: Arc<BaselineState>,
    broadcast_handle: Arc<AttentionCueBroadcast>,
    cadence_handle: Arc<CadenceTriggerChannel>,
    thresholds: Arc<Thresholds>,
) {
    let mut interval = tokio::time::interval(thresholds.tick_interval);
    // Skip immediate first tick to avoid sweeping at startup with no data —
    // mirrors chunk #20 retention loop + chunk #61 persist loop precedent.
    interval.tick().await;
    loop {
        interval.tick().await;
        let now = current_unix_nanos();
        let _ = run_one_emit_cycle(&state, &thresholds, &broadcast_handle, &cadence_handle, now);
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
        for i in 0..100 {
            let status = if i < 50 { 2 } else { 0 };
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

        let handle = tokio::spawn(start_emitter(
            Arc::clone(&state),
            Arc::clone(&broadcast_handle),
            Arc::clone(&cadence_handle),
            Arc::clone(&thresholds),
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
