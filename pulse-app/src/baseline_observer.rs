//! Baseline observer adapter — chunk #62.
//!
//! Bridges `ingest::observer::SpanObserver` (trait declared in lower-level
//! ingest crate) to `triage::BaselineState::observe_span` (chunk #61
//! streaming baseline tracker). Lives at the pulse-app binary boundary
//! per arch §Cross-cutting Patterns Module dependency direction —
//! preserves the DAG flow (deps flow toward pulse-app; library crates
//! don't depend on siblings). Mirrors chunk #59 `HeartbeatBindStatus`
//! adapter precedent.

use std::sync::Arc;

use ingest::observer::SpanObserver;
use triage::contract::BaselineState;

/// Adapter wrapping `Arc<triage::BaselineState>` to satisfy
/// `ingest::observer::SpanObserver`. Cheap to clone (Arc share) — the
/// underlying state's `observe_span` is lock-free via DashMap per-shard.
#[derive(Debug, Clone)]
pub struct BaselineObserverAdapter {
    state: Arc<BaselineState>,
}

impl BaselineObserverAdapter {
    pub fn new(state: Arc<BaselineState>) -> Self {
        Self { state }
    }
}

impl SpanObserver for BaselineObserverAdapter {
    fn observe_span(
        &self,
        service_name: &str,
        operation_name: &str,
        status_code: u8,
        latency_ms: u64,
        now_nanos: i64,
    ) {
        self.state.observe_span(
            service_name,
            operation_name,
            status_code,
            latency_ms,
            now_nanos,
        );
    }
}

// Tests migrated to `pulse-app/tests/unit_span_observers.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
