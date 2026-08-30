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

// Tests migrated to `pulse-app/tests/unit_span_observers.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
