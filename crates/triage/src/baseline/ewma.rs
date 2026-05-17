use serde::{Deserialize, Serialize};

/// Per-service exponential weighted moving average tracker for streaming
/// baselines (capability spec P-009). Alpha is calibrated for a 5-minute
/// effective window assuming roughly 1 observation per second; see
/// `DEFAULT_ALPHA_5MIN_WINDOW` in the parent module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EwmaTracker {
    current_value: f64,
    last_update_nanos: i64,
    alpha: f64,
    samples: u64,
}

// Chunk #61 deliverable: callable + testable primitives for chunk #62.
// Accessor methods (samples / last_update_nanos / alpha) exercised via
// tests; allow(dead_code) signals future API surface для emitter +
// percentile-snapshot consumers.
#[allow(dead_code)]
impl EwmaTracker {
    pub fn new(alpha: f64) -> Self {
        Self {
            current_value: 0.0,
            last_update_nanos: 0,
            alpha,
            samples: 0,
        }
    }

    pub fn observe(&mut self, value: f64, now_nanos: i64) -> f64 {
        if self.samples == 0 {
            self.current_value = value;
        } else {
            self.current_value = self.alpha * value + (1.0 - self.alpha) * self.current_value;
        }
        self.last_update_nanos = now_nanos;
        self.samples = self.samples.saturating_add(1);
        self.current_value
    }

    pub fn value(&self) -> f64 {
        self.current_value
    }

    pub fn samples(&self) -> u64 {
        self.samples
    }

    pub fn last_update_nanos(&self) -> i64 {
        self.last_update_nanos
    }

    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}

impl Default for EwmaTracker {
    fn default() -> Self {
        Self::new(super::DEFAULT_ALPHA_5MIN_WINDOW)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn ewma_first_observation_returns_value_directly() {
        let mut tracker = EwmaTracker::new(0.1);
        let result = tracker.observe(42.0, 1_000);
        assert_eq!(result, 42.0);
        assert_eq!(tracker.value(), 42.0);
        assert_eq!(tracker.samples(), 1);
        assert_eq!(tracker.last_update_nanos(), 1_000);
    }

    #[test]
    fn ewma_converges_toward_constant_input() {
        let mut tracker = EwmaTracker::new(0.1);
        for i in 0..1000 {
            tracker.observe(10.0, i);
        }
        let diff = (tracker.value() - 10.0).abs();
        assert!(
            diff < 0.01,
            "expected convergence to 10.0; got {}",
            tracker.value()
        );
    }

    #[test]
    fn ewma_alpha_zero_keeps_initial_value() {
        let mut tracker = EwmaTracker::new(0.0);
        tracker.observe(5.0, 1_000);
        tracker.observe(100.0, 2_000);
        assert_eq!(tracker.value(), 5.0);
    }

    #[test]
    fn ewma_alpha_one_replaces_value() {
        let mut tracker = EwmaTracker::new(1.0);
        tracker.observe(5.0, 1_000);
        tracker.observe(100.0, 2_000);
        assert_eq!(tracker.value(), 100.0);
    }

    #[test]
    fn ewma_serde_round_trip_preserves_state() {
        let mut tracker = EwmaTracker::new(0.1);
        tracker.observe(50.0, 1_000);
        tracker.observe(60.0, 2_000);
        let bytes = bincode::serialize(&tracker).expect("serialize");
        let restored: EwmaTracker = bincode::deserialize(&bytes).expect("deserialize");
        assert_eq!(restored.value(), tracker.value());
        assert_eq!(restored.samples(), tracker.samples());
        assert_eq!(restored.last_update_nanos(), tracker.last_update_nanos());
        assert_eq!(restored.alpha(), tracker.alpha());
    }

    proptest! {
        #[test]
        fn ewma_bounded_within_observed_min_max(values in prop::collection::vec(0.0f64..1000.0, 10..100)) {
            let mut tracker = EwmaTracker::new(0.1);
            let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            for (i, v) in values.iter().enumerate() {
                tracker.observe(*v, i as i64 * 1_000_000_000);
            }
            let v = tracker.value();
            prop_assert!(v >= min - 1e-9 && v <= max + 1e-9,
                "ewma {} outside observed range [{}, {}]", v, min, max);
        }
    }
}
