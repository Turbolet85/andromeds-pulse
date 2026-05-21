use serde::{Deserialize, Serialize};
use tdigest::TDigest;

const TDIGEST_COMPRESSION: usize = 100;
const FLUSH_THRESHOLD: usize = 100;

/// Per-operation streaming p50/p95/p99 estimator using a pair of t-digests
/// (current + previous) with swap-on-tick rotation (capability spec P-011).
/// Inserts buffer into `current_buffer` for batch-efficient `merge_unsorted`;
/// `swap_on_tick(now_nanos)` rotates the current digest into previous and
/// resets current when the swap window has elapsed.
///
/// `percentile(q)` queries the union of current + previous so callers see a
/// stable estimate that does not regress to zero at rotation boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDigestPair {
    current_buffer: Vec<f64>,
    current: TDigest,
    previous: TDigest,
    last_swap_nanos: i64,
    samples_current: u64,
    samples_previous: u64,
}

// Chunk #61 deliverable: callable + testable primitives for chunk #62.
// `samples_current` is exercised via swap-rotation tests; `centroid_count`
// fed into `BaselineState::total_centroid_count` for observability metric.
#[allow(dead_code)]
impl TDigestPair {
    pub fn new() -> Self {
        Self {
            current_buffer: Vec::new(),
            current: TDigest::new_with_size(TDIGEST_COMPRESSION),
            previous: TDigest::new_with_size(TDIGEST_COMPRESSION),
            last_swap_nanos: 0,
            samples_current: 0,
            samples_previous: 0,
        }
    }

    pub fn insert(&mut self, value: f64) {
        self.current_buffer.push(value);
        self.samples_current = self.samples_current.saturating_add(1);
        if self.current_buffer.len() >= FLUSH_THRESHOLD {
            self.flush_buffer();
        }
    }

    fn flush_buffer(&mut self) {
        if self.current_buffer.is_empty() {
            return;
        }
        let drained = std::mem::take(&mut self.current_buffer);
        self.current = self.current.merge_unsorted(drained);
    }

    pub fn swap_on_tick(&mut self, now_nanos: i64, swap_window_nanos: i64) -> bool {
        let age = now_nanos.saturating_sub(self.last_swap_nanos);
        if self.last_swap_nanos == 0 || age >= swap_window_nanos {
            self.flush_buffer();
            self.previous = std::mem::replace(
                &mut self.current,
                TDigest::new_with_size(TDIGEST_COMPRESSION),
            );
            self.samples_previous = self.samples_current;
            self.samples_current = 0;
            self.last_swap_nanos = now_nanos;
            true
        } else {
            false
        }
    }

    /// Query estimated percentile (q ∈ [0.0, 1.0]). Returns None when no
    /// samples have been observed across either window.
    pub fn percentile(&self, q: f64) -> Option<f64> {
        if self.samples_current == 0 && self.samples_previous == 0 {
            return None;
        }
        let current_eff = if self.current_buffer.is_empty() {
            self.current.clone()
        } else {
            self.current.merge_unsorted(self.current_buffer.clone())
        };
        let merged = TDigest::merge_digests(vec![current_eff, self.previous.clone()]);
        if merged.count() == 0.0 {
            None
        } else {
            Some(merged.estimate_quantile(q))
        }
    }

    /// Query the CURRENT window's percentile only (excluding `previous`)
    /// per chunk #73 P-012. Used by the short-window t-digest snapshot
    /// so the "recent observations" signal is not diluted by the prior
    /// rotation cycle. Returns None when current is empty.
    pub fn percentile_current_only(&self, q: f64) -> Option<f64> {
        if self.samples_current == 0 && self.current_buffer.is_empty() {
            return None;
        }
        let current_eff = if self.current_buffer.is_empty() {
            self.current.clone()
        } else {
            self.current.merge_unsorted(self.current_buffer.clone())
        };
        if current_eff.count() == 0.0 {
            None
        } else {
            Some(current_eff.estimate_quantile(q))
        }
    }

    pub fn samples_current(&self) -> u64 {
        self.samples_current
    }

    pub fn centroid_count(&self) -> usize {
        // Approximation: t-digest internal centroid count is not exposed via
        // the public API; total samples is the upper-bound proxy for the
        // observability `tdigest_centroid_count` metric field.
        (self.samples_current + self.samples_previous) as usize
    }
}

impl Default for TDigestPair {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const ONE_MINUTE_NANOS: i64 = 60_000_000_000;

    #[test]
    fn percentile_empty_returns_none() {
        let pair = TDigestPair::new();
        assert!(pair.percentile(0.5).is_none());
    }

    #[test]
    fn percentile_after_inserts_returns_some() {
        let mut pair = TDigestPair::new();
        for i in 0..1000 {
            pair.insert(i as f64);
        }
        let p50 = pair
            .percentile(0.5)
            .expect("p50 should exist after 1000 inserts");
        assert!(
            p50 > 100.0 && p50 < 900.0,
            "p50={p50} unreasonable for [0..1000)"
        );
    }

    #[test]
    fn percentile_monotonic_p50_le_p95_le_p99() {
        let mut pair = TDigestPair::new();
        for i in 0..1000 {
            pair.insert(i as f64);
        }
        let p50 = pair.percentile(0.5).expect("p50");
        let p95 = pair.percentile(0.95).expect("p95");
        let p99 = pair.percentile(0.99).expect("p99");
        assert!(p50 <= p95, "p50={p50} > p95={p95}");
        assert!(p95 <= p99, "p95={p95} > p99={p99}");
    }

    #[test]
    fn swap_rotates_current_to_previous_after_window() {
        let mut pair = TDigestPair::new();
        for i in 0..200 {
            pair.insert(i as f64);
        }
        assert_eq!(pair.samples_current(), 200);
        let swapped = pair.swap_on_tick(ONE_MINUTE_NANOS, ONE_MINUTE_NANOS);
        assert!(swapped);
        assert_eq!(pair.samples_current(), 0);
        // Querying after swap should still return a value (uses previous window)
        assert!(pair.percentile(0.5).is_some());
    }

    #[test]
    fn swap_no_op_before_window_elapses() {
        let mut pair = TDigestPair::new();
        pair.insert(10.0);
        let swapped_first = pair.swap_on_tick(ONE_MINUTE_NANOS, ONE_MINUTE_NANOS);
        assert!(swapped_first, "initial swap should fire");
        let swapped_second = pair.swap_on_tick(ONE_MINUTE_NANOS + 1_000, ONE_MINUTE_NANOS);
        assert!(!swapped_second, "second swap within window should not fire");
    }

    #[test]
    fn serde_round_trip_preserves_state() {
        let mut pair = TDigestPair::new();
        for i in 0..500 {
            pair.insert(i as f64);
        }
        let bytes = bincode::serialize(&pair).expect("serialize");
        let restored: TDigestPair = bincode::deserialize(&bytes).expect("deserialize");
        let original_p50 = pair.percentile(0.5).expect("p50 original");
        let restored_p50 = restored.percentile(0.5).expect("p50 restored");
        let diff = (original_p50 - restored_p50).abs();
        assert!(
            diff < 1.0,
            "p50 drift after round-trip: {original_p50} → {restored_p50}"
        );
    }

    proptest! {
        #[test]
        fn percentile_monotonic_property(values in prop::collection::vec(0.0f64..10_000.0, 100..500)) {
            let mut pair = TDigestPair::new();
            for v in &values {
                pair.insert(*v);
            }
            let p50 = pair.percentile(0.5).expect("p50");
            let p95 = pair.percentile(0.95).expect("p95");
            let p99 = pair.percentile(0.99).expect("p99");
            prop_assert!(p50 <= p95 + 1e-6, "p50={} > p95={}", p50, p95);
            prop_assert!(p95 <= p99 + 1e-6, "p95={} > p99={}", p95, p99);
        }
    }
}
