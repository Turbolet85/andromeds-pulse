//! Retrospective re-evaluation engine (chunk #96 — capability P-056).
//!
//! The opt-in `diagnostics.reevaluate_recent_window()` action performs the
//! FULL retrospective pass: it re-classifies every tracked service against the
//! CURRENT (freshly hot-reloaded) lifecycle thresholds, broadcasting the
//! implied lifecycle transitions, instead of waiting for the next heartbeat
//! tick. Cadence is a scheduler — its config applies prospectively on the next
//! tick, so there is no retrospective cadence pass; the summary reports this.
//!
//! Trait-injected into `DiagnosticsApiImpl` per arch §Cross-cutting Module
//! dependency direction (the concrete `LiveReevaluator` holds the triage-side
//! `Arc<dyn ServiceRegistry>` + `Arc<BaselineState>` + the lifecycle threshold
//! watch receiver).

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::watch;
use triage::contract::{
    BaselineState, LifecycleThresholds, ServiceLifecycleBroadcast, ServiceRegistry, reevaluate_now,
};

/// Aggregate outcome of a retrospective re-evaluation pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReevaluationSummary {
    /// Services re-evaluated against the current thresholds (the tracked set).
    pub services_reclassified: u64,
    /// Lifecycle transition events emitted by the re-classification.
    pub transitions_emitted: u64,
    /// Always true — cadence config applies prospectively (no retrospective
    /// scheduler concept); surfaced so callers can render the distinction.
    pub cadence_prospective: bool,
}

/// Injected at the diagnostics-router boundary so the resolver stays free of
/// the triage registry / baseline handles.
pub trait RecentWindowReevaluator: Send + Sync {
    fn reevaluate(&self) -> ReevaluationSummary;
}

/// Production reevaluator over the live lifecycle registry + baseline state.
pub struct LiveReevaluator {
    registry: Arc<dyn ServiceRegistry>,
    broadcast: Arc<ServiceLifecycleBroadcast>,
    baseline: Arc<BaselineState>,
    thresholds_rx: watch::Receiver<LifecycleThresholds>,
}

impl LiveReevaluator {
    pub fn new(
        registry: Arc<dyn ServiceRegistry>,
        broadcast: Arc<ServiceLifecycleBroadcast>,
        baseline: Arc<BaselineState>,
        thresholds_rx: watch::Receiver<LifecycleThresholds>,
    ) -> Self {
        Self {
            registry,
            broadcast,
            baseline,
            thresholds_rx,
        }
    }
}

impl RecentWindowReevaluator for LiveReevaluator {
    fn reevaluate(&self) -> ReevaluationSummary {
        let now = now_unix_nano();
        let thresholds = *self.thresholds_rx.borrow();
        let transitions = reevaluate_now(
            self.registry.as_ref(),
            &self.broadcast,
            &self.baseline,
            thresholds,
            now,
        ) as u64;
        // Count after the pass so freshly-tracked services are reflected.
        let services_reclassified = self.registry.count() as u64;
        ReevaluationSummary {
            services_reclassified,
            transitions_emitted: transitions,
            cadence_prospective: true,
        }
    }
}

fn now_unix_nano() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}
