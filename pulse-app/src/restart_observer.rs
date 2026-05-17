//! Restart observer adapter — chunk #63.
//!
//! Bridges `ingest::observer::SpanObserver` (trait declared in lower-level
//! ingest crate) to `triage::pattern::observe_and_dispatch` (chunk #63
//! restart-detection hot-path helper). Lives at the pulse-app binary
//! boundary per arch §Cross-cutting Patterns Module dependency direction
//! — preserves the DAG flow (deps flow toward pulse-app; library crates
//! don't depend on siblings). Mirrors `BaselineObserverAdapter` chunk #62
//! precedent at the sibling `baseline_observer.rs`.
//!
//! `CompositeSpanObserver` composes multiple `SpanObserver`s so the buffer
//! consumer's single-observer hook fan-outs to both baseline + restart
//! observers per chunk #63 plan Step 11.

use std::sync::Arc;

use ingest::observer::SpanObserver;
use triage::contract::{RestartDetector, RestartEventBroadcast, observe_and_dispatch};

/// Adapter wrapping `Arc<RestartDetector>` + `Arc<RestartEventBroadcast>`
/// to satisfy `ingest::observer::SpanObserver`. Cheap to clone — both
/// inner handles are Arc shares; the detector's `observe_span` is
/// lock-free via DashMap per-shard.
///
/// `operation_name`, `status_code`, `latency_ms` are ignored: the restart
/// detector reasons over per-service timestamps only. Empty `service_name`
/// is dropped inside `observe_and_dispatch` per chunk #61 service identity
/// discipline.
#[derive(Debug, Clone)]
pub struct RestartObserverAdapter {
    detector: Arc<RestartDetector>,
    broadcast: Arc<RestartEventBroadcast>,
}

impl RestartObserverAdapter {
    pub fn new(detector: Arc<RestartDetector>, broadcast: Arc<RestartEventBroadcast>) -> Self {
        Self {
            detector,
            broadcast,
        }
    }
}

impl SpanObserver for RestartObserverAdapter {
    fn observe_span(
        &self,
        service_name: &str,
        _operation_name: &str,
        _status_code: u8,
        _latency_ms: u64,
        now_nanos: i64,
    ) {
        observe_and_dispatch(&self.detector, &self.broadcast, service_name, now_nanos);
    }
}

/// Multi-observer composite implementing `SpanObserver` by fanning out
/// to a `Vec<Arc<dyn SpanObserver>>`. Used at boot to wire both baseline
/// and restart observers into the single buffer-consumer span-observer
/// hook; composition happens at the pulse-app boundary per arch
/// §Cross-cutting Patterns Module dependency direction.
pub struct CompositeSpanObserver {
    inner: Vec<Arc<dyn SpanObserver>>,
}

impl std::fmt::Debug for CompositeSpanObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompositeSpanObserver")
            .field("inner_count", &self.inner.len())
            .finish()
    }
}

impl CompositeSpanObserver {
    pub fn new(inner: Vec<Arc<dyn SpanObserver>>) -> Self {
        Self { inner }
    }
}

impl SpanObserver for CompositeSpanObserver {
    fn observe_span(
        &self,
        service_name: &str,
        operation_name: &str,
        status_code: u8,
        latency_ms: u64,
        now_nanos: i64,
    ) {
        for observer in &self.inner {
            observer.observe_span(
                service_name,
                operation_name,
                status_code,
                latency_ms,
                now_nanos,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use triage::contract::{BaselineState, RestartDetector, RestartEventBroadcast};

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
        use crate::baseline_observer::BaselineObserverAdapter;

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
}
