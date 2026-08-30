// Migrated 2026-08-30 from the `mod tests` blocks of
// `pulse-app/src/{baseline_observer,restart_observer,storm_observer}.rs` —
// that crate sets `[lib] test = false` (the WebView2 workaround), so a
// src-level `mod tests` compiles, passes clippy, and NEVER RUNS. The three
// adapter modules cohere as one target: the composite fan-out test already
// crosses baseline + restart.

use std::sync::Arc;

use buffer::fingerprint::FingerprintObserver;
use ingest::observer::SpanObserver;
use triage::contract::{
    AttentionCueBroadcast, BaselineState, CueKind, PriorityTier, RestartDetector,
    RestartEventBroadcast, RetryStormDetector,
};

use pulse_app::baseline_observer::BaselineObserverAdapter;
use pulse_app::restart_observer::{CompositeSpanObserver, RestartObserverAdapter};
use pulse_app::storm_observer::StormObserverAdapter;

// ── baseline_observer.rs ─────────────────────────────────────────────

#[test]
fn adapter_delegates_observe_span_to_baseline_state() {
    let state = Arc::new(BaselineState::new());
    let adapter = BaselineObserverAdapter::new(Arc::clone(&state));

    assert_eq!(state.service_count(), 0);
    adapter.observe_span("svc-a", "op-x", 0, 100, 1_000_000);
    assert_eq!(state.service_count(), 1);
}

#[test]
fn adapter_can_be_held_as_dyn_span_observer() {
    let state = Arc::new(BaselineState::new());
    let observer: Arc<dyn SpanObserver> = Arc::new(BaselineObserverAdapter::new(state));
    observer.observe_span("svc-a", "op", 0, 100, 1_000_000);
}

// ── restart_observer.rs ──────────────────────────────────────────────

#[test]
fn restart_adapter_dispatches_observe_span_to_detector() {
    let detector = Arc::new(RestartDetector::new(20));
    let broadcast = Arc::new(RestartEventBroadcast::new());
    let adapter = RestartObserverAdapter::new(Arc::clone(&detector), Arc::clone(&broadcast));

    adapter.observe_span("svc-a", "op-x", 0, 50, 1_000_000_000);
    adapter.observe_span("svc-a", "op-x", 0, 50, 25_000_000_000);

    assert_eq!(detector.restart_events_emitted_total(), 1);
}

#[test]
fn restart_adapter_can_be_held_as_dyn_span_observer() {
    let detector = Arc::new(RestartDetector::new(20));
    let broadcast = Arc::new(RestartEventBroadcast::new());
    let observer: Arc<dyn SpanObserver> =
        Arc::new(RestartObserverAdapter::new(detector, broadcast));
    observer.observe_span("svc-a", "op", 0, 100, 1_000_000);
}

#[test]
fn composite_observer_fans_out_to_all_inner_observers() {
    // Compose baseline + restart adapter; verify both side-effects fire.
    let baseline_state = Arc::new(BaselineState::new());
    let detector = Arc::new(RestartDetector::new(20));
    let broadcast = Arc::new(RestartEventBroadcast::new());
    let baseline_adapter: Arc<dyn SpanObserver> =
        Arc::new(BaselineObserverAdapter::new(Arc::clone(&baseline_state)));
    let restart_adapter: Arc<dyn SpanObserver> = Arc::new(RestartObserverAdapter::new(
        Arc::clone(&detector),
        Arc::clone(&broadcast),
    ));
    let composite = CompositeSpanObserver::new(vec![baseline_adapter, restart_adapter]);

    composite.observe_span("svc-a", "op", 0, 100, 1_000_000_000);
    composite.observe_span("svc-a", "op", 0, 100, 25_000_000_000);

    assert_eq!(baseline_state.service_count(), 1, "baseline tracked");
    assert_eq!(
        detector.restart_events_emitted_total(),
        1,
        "restart detected"
    );
}

#[test]
fn composite_observer_with_empty_inner_is_noop() {
    let composite = CompositeSpanObserver::new(Vec::new());
    composite.observe_span("svc-a", "op", 0, 100, 1_000_000_000);
    // No panic, no side effect — empty composite is a no-op.
}

// ── storm_observer.rs ────────────────────────────────────────────────

const NANOS_PER_SEC: i64 = 1_000_000_000;
const FP: [u8; 16] = [0xCC; 16];

fn fresh_detector() -> Arc<RetryStormDetector> {
    Arc::new(RetryStormDetector::new(60, 30, 5, 10))
}

#[test]
fn storm_adapter_dispatches_observe_fingerprint_to_detector() {
    let detector = fresh_detector();
    let broadcast = Arc::new(AttentionCueBroadcast::new());
    let mut rx = broadcast.subscribe();
    let adapter = StormObserverAdapter::new(Arc::clone(&detector), Arc::clone(&broadcast));

    for i in 0..5 {
        let ts = (1_000 + i) * NANOS_PER_SEC;
        adapter.on_fingerprint(FP, "svc", ts);
    }

    let cue = rx.try_recv().expect("Suggested cue broadcast");
    assert_eq!(cue.kind, CueKind::RetryStorm);
    assert_eq!(cue.priority_tier, PriorityTier::Suggested);
    assert_eq!(detector.storms_detected_total(), 1);
}

#[test]
fn storm_adapter_can_be_held_as_dyn_fingerprint_observer() {
    let detector = fresh_detector();
    let broadcast = Arc::new(AttentionCueBroadcast::new());
    let observer: Arc<dyn FingerprintObserver> =
        Arc::new(StormObserverAdapter::new(detector, broadcast));
    observer.on_fingerprint(FP, "svc", 1_000 * NANOS_PER_SEC);
}
