//! Storm observer adapter — chunk #66.
//!
//! Bridges `buffer::fingerprint::FingerprintObserver` (trait declared in
//! the lower buffer crate) к `triage::pattern::storm::observe_and_dispatch_storm`
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

#[cfg(test)]
mod tests {
    use super::*;
    use triage::contract::{AttentionCueBroadcast, CueKind, PriorityTier, RetryStormDetector};

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
}
