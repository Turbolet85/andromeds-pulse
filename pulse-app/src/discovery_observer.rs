//! First-sighting observer adapter.
//!
//! Bridges `ingest::observer::SpanObserver` to
//! `triage::ServiceRegistry::register_first_sighting`, so a service is listed
//! for the constellation at its first span instead of at the lifecycle
//! heartbeat's first tick (P-027 discovery bound). Lives at the pulse-app
//! binary boundary per arch §Cross-cutting Patterns Module dependency
//! direction, beside `BaselineObserverAdapter` / `RestartObserverAdapter`.
//!
//! Composed AFTER the baseline adapter: the cap check reads what the
//! baseline admitted in the same fan-out. The service name is the scrubbed
//! identity the buffer tap passes; nothing is traced per span (the tick's
//! transition aggregate carries the count).

use std::sync::Arc;

use ingest::observer::SpanObserver;
use triage::contract::{BaselineState, ServiceLifecycleBroadcast, ServiceRegistry};

#[derive(Debug, Clone)]
pub struct DiscoveryObserverAdapter {
    registry: Arc<dyn ServiceRegistry>,
    baseline: Arc<BaselineState>,
    broadcast: Arc<ServiceLifecycleBroadcast>,
}

impl DiscoveryObserverAdapter {
    pub fn new(
        registry: Arc<dyn ServiceRegistry>,
        baseline: Arc<BaselineState>,
        broadcast: Arc<ServiceLifecycleBroadcast>,
    ) -> Self {
        Self {
            registry,
            baseline,
            broadcast,
        }
    }
}

impl SpanObserver for DiscoveryObserverAdapter {
    fn observe_span(
        &self,
        service_name: &str,
        _operation_name: &str,
        _status_code: u8,
        _latency_ms: u64,
        now_nanos: i64,
    ) {
        if service_name.is_empty()
            || self.registry.current_state(service_name).is_some()
            || !self.baseline.tracks_service(service_name)
        {
            return;
        }
        if let Some(event) = self
            .registry
            .register_first_sighting(service_name, now_nanos)
        {
            let _ = self.broadcast.sender().send(event);
        }
    }
}

// Tests live in `pulse-app/tests/unit_span_observers.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
