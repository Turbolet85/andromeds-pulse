use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// Fixed-capacity circular buffer for streaming activity tracking
/// (capability spec P-009 activity baseline). Oldest-evicting push: when
/// at capacity, push removes the oldest entry before adding the new one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingWindow<T> {
    inner: VecDeque<T>,
    capacity: usize,
}

// Chunk #61 deliverable: callable + testable primitives for chunk #62
// attention cue emitter. `#[allow(dead_code)]` reflects that accessor
// methods (len / capacity / iter) are exercised only via tests in this
// chunk; the producer (BaselineState) calls only `with_capacity` + `push`.
#[allow(dead_code)]
impl<T> RollingWindow<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(cap),
            capacity: cap.max(1),
        }
    }

    pub fn push(&mut self, value: T) {
        while self.inner.len() >= self.capacity {
            self.inner.pop_front();
        }
        self.inner.push_back(value);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

impl<T> Default for RollingWindow<T> {
    fn default() -> Self {
        Self::with_capacity(super::DEFAULT_ACTIVITY_WINDOW_CAPACITY)
    }
}

#[allow(dead_code)]
impl RollingWindow<u32> {
    pub fn sum(&self) -> u64 {
        self.inner.iter().map(|&x| x as u64).sum()
    }

    pub fn mean(&self) -> f64 {
        if self.inner.is_empty() {
            0.0
        } else {
            self.sum() as f64 / self.inner.len() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn rolling_window_respects_capacity() {
        let mut w: RollingWindow<u32> = RollingWindow::with_capacity(3);
        w.push(1);
        w.push(2);
        w.push(3);
        w.push(4);
        assert_eq!(w.len(), 3);
        let v: Vec<u32> = w.iter().copied().collect();
        assert_eq!(v, vec![2, 3, 4]);
    }

    #[test]
    fn rolling_window_sum_and_mean() {
        let mut w: RollingWindow<u32> = RollingWindow::with_capacity(4);
        w.push(10);
        w.push(20);
        w.push(30);
        w.push(40);
        assert_eq!(w.sum(), 100);
        assert_eq!(w.mean(), 25.0);
    }

    #[test]
    fn rolling_window_empty_mean_is_zero() {
        let w: RollingWindow<u32> = RollingWindow::with_capacity(4);
        assert_eq!(w.mean(), 0.0);
    }

    #[test]
    fn rolling_window_serde_round_trip() {
        let mut w: RollingWindow<u32> = RollingWindow::with_capacity(5);
        for n in 0..7 {
            w.push(n);
        }
        let bytes = bincode::serialize(&w).expect("serialize");
        let restored: RollingWindow<u32> = bincode::deserialize(&bytes).expect("deserialize");
        assert_eq!(restored.len(), w.len());
        assert_eq!(restored.capacity(), w.capacity());
        assert_eq!(restored.sum(), w.sum());
        // Sanity-cover the remaining accessors so `iter()` is exercised here too.
        let restored_count = restored.iter().count();
        assert_eq!(restored_count, restored.len());
    }

    proptest! {
        #[test]
        fn rolling_window_length_never_exceeds_capacity(
            cap in 1usize..16,
            values in prop::collection::vec(0u32..1_000, 0..200)
        ) {
            let mut w: RollingWindow<u32> = RollingWindow::with_capacity(cap);
            for v in &values {
                w.push(*v);
                prop_assert!(w.len() <= cap, "len {} exceeded capacity {}", w.len(), cap);
            }
            prop_assert_eq!(w.len(), values.len().min(cap));
        }
    }
}
