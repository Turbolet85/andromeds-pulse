//! Storm observer adapter — chunk #66.
//!
//! Bridges `buffer::fingerprint::FingerprintObserver` (trait declared in
//! the lower buffer crate) to `triage::pattern::storm::observe_and_dispatch_storm`
//! (chunk #66 retry-storm dispatch helper). Lives at the pulse-app binary
//! boundary per arch §Cross-cutting Patterns Module dependency direction —
//! preserves the DAG flow (deps flow toward pulse-app; library crates
//! don't depend on siblings). Mirrors `RestartObserverAdapter` chunk #63
//! + `BaselineObserverAdapter` chunk #62 precedents.

use std::sync::Arc;

use buffer::fingerprint::{ExceptionFingerprint, FingerprintObserver};
use triage::contract::{AttentionCueBroadcast, RetryStormDetector, observe_and_dispatch_storm};

/// Adapter wrapping `Arc<RetryStormDetector>` + `Arc<AttentionCueBroadcast>`
/// to satisfy `buffer::fingerprint::FingerprintObserver`. Cheap to clone —
/// both inner handles are Arc shares; the detector's `record_occurrence`
/// is lock-free per DashMap shard.
#[derive(Debug, Clone)]
pub struct StormObserverAdapter {
    detector: Arc<RetryStormDetector>,
    broadcast: Arc<AttentionCueBroadcast>,
}

impl StormObserverAdapter {
    pub fn new(detector: Arc<RetryStormDetector>, broadcast: Arc<AttentionCueBroadcast>) -> Self {
        Self {
            detector,
            broadcast,
        }
    }
}

impl FingerprintObserver for StormObserverAdapter {
    fn on_fingerprint(
        &self,
        fingerprint: ExceptionFingerprint,
        service_name: &str,
        ts_unix_nano: i64,
    ) {
        observe_and_dispatch_storm(
            &self.detector,
            &self.broadcast,
            fingerprint,
            service_name,
            ts_unix_nano,
        );
    }
}

// Tests migrated to `pulse-app/tests/unit_span_observers.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
