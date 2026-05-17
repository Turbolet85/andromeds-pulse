//! Per-service activity-floor tracker for the chunk #64 L1b distillation
//! layer (capabilities P-013 service activity floor learning + P-014 service
//! went silent detection per `docs/v0_2_0/pulse-v0_2_0-route.md` §64).
//!
//! Two parallel structures cohabit one tracker because they answer different
//! questions (research.md Open question 1 path a — preferred interpretation).
//! The 288-bucket activity-density histogram captures the typical
//! observations-per-5-min-slot pattern for diagnostic surfacing; the
//! TDigest of inter-observation gap durations powers the p95 quiet-duration
//! gate that `evaluate_service_went_silent` consults.
//!
//! Cardinality / OOM defenses live one layer up at `BaselineState::observe_span`
//! (chunk #64 Step 2 service-name cap enforcement); this module assumes its
//! caller has already bounded the number of `ActivityFloor` instances per
//! process.

use serde::{Deserialize, Serialize};
use tdigest::TDigest;

/// Number of 5-minute bucket slots covering a full 24-hour rolling window
/// (`288 = 24 * 60 / 5`). Per pulse-v0_2_0-route §64 spec.
pub const BUCKET_COUNT: usize = 288;

/// Per-bucket time interval in seconds.
pub const BUCKET_INTERVAL_SECONDS: u64 = 300;

/// Total window the histogram covers (24h).
pub const WINDOW_DURATION_SECONDS: u64 = 86_400;

/// Per-service cold-start suppression window (1h) during which
/// `ServiceWentSilent` emission is universally suppressed regardless of
/// quiet duration. Per pulse-v0_2_0-route §64 spec.
pub const BOOTSTRAP_WINDOW_SECONDS: u64 = 3_600;

/// Percentile used to gate `ServiceWentSilent` emission against the learned
/// historical quiet-duration distribution. Per pulse-v0_2_0-route §64 spec.
pub const DEFAULT_QUIET_DURATION_PERCENTILE: f64 = 0.95;

/// TDigest compression parameter for the per-service quiet-duration digest;
/// matches the chunk #61 `TDIGEST_COMPRESSION` precedent
/// (`crates/triage/src/baseline/tdigest_pair.rs:4`).
const TDIGEST_COMPRESSION: usize = 100;

/// Batch insert threshold for the quiet-duration buffer; flushes when at or
/// above this many pending samples. Mirrors the chunk #61 TDigestPair
/// `FLUSH_THRESHOLD` to keep per-observe allocation amortized.
const QUIET_DIGEST_FLUSH_THRESHOLD: usize = 100;

// Module-level compile-time sanity asserts per testing.md Session Additions
// 2026-05-11 (clippy::assertions_on_constants forbidden inside `#[test]` fn).
const _: () = {
    assert!(BUCKET_COUNT > 0);
    assert!(BUCKET_INTERVAL_SECONDS > 0);
    assert!(WINDOW_DURATION_SECONDS == (BUCKET_COUNT as u64) * BUCKET_INTERVAL_SECONDS);
    assert!(BOOTSTRAP_WINDOW_SECONDS > 0);
    assert!(BOOTSTRAP_WINDOW_SECONDS < WINDOW_DURATION_SECONDS);
    assert!(DEFAULT_QUIET_DURATION_PERCENTILE > 0.0);
    assert!(DEFAULT_QUIET_DURATION_PERCENTILE < 1.0);
    assert!(TDIGEST_COMPRESSION > 0);
    assert!(QUIET_DIGEST_FLUSH_THRESHOLD > 0);
};

/// Lifecycle state of a per-service activity-floor tracker. `Learning` is
/// returned during the bootstrap window (also returned when the tracker has
/// never observed a span); `Ready` is returned once `now - first_observed`
/// exceeds the bootstrap window. Variants serialize as snake_case strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapState {
    Learning,
    Ready,
}

/// Per-service activity-floor tracker. Cheap to clone; designed for storage
/// behind a `DashMap` shard alongside the chunk #61 baseline trackers
/// (`ServiceBaseline` aggregates EWMA + RollingWindow + this struct).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityFloor {
    buckets: Vec<u32>,
    bucket_epochs: Vec<u64>,
    last_observed_unix_nanos: i64,
    first_observed_unix_nanos: i64,
    quiet_duration_buffer: Vec<f64>,
    quiet_duration_digest: TDigest,
}

impl Default for ActivityFloor {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl ActivityFloor {
    pub fn new() -> Self {
        Self {
            buckets: vec![0u32; BUCKET_COUNT],
            bucket_epochs: vec![0u64; BUCKET_COUNT],
            last_observed_unix_nanos: 0,
            first_observed_unix_nanos: 0,
            quiet_duration_buffer: Vec::new(),
            quiet_duration_digest: TDigest::new_with_size(TDIGEST_COMPRESSION),
        }
    }

    /// Record an observation at `now_nanos`. Bootstraps `first_observed` on
    /// the very first call; records the prior gap into the quiet-duration
    /// digest on subsequent calls; increments the time-of-day bucket and
    /// resets the bucket when its stored epoch (`now / 24h`) is stale,
    /// preserving the 24h rolling-window invariant under modulo bucket
    /// indexing.
    pub fn observe(&mut self, now_nanos: i64) {
        if self.first_observed_unix_nanos == 0 {
            self.first_observed_unix_nanos = now_nanos;
        }

        if self.last_observed_unix_nanos > 0 {
            let gap_nanos = now_nanos.saturating_sub(self.last_observed_unix_nanos);
            let gap_seconds = (gap_nanos.max(0) as f64) / 1_000_000_000.0;
            self.quiet_duration_buffer.push(gap_seconds);
            if self.quiet_duration_buffer.len() >= QUIET_DIGEST_FLUSH_THRESHOLD {
                self.flush_buffer();
            }
        }

        let bucket_idx = bucket_index_for(now_nanos);
        let current_epoch = epoch_for(now_nanos);
        if self.bucket_epochs[bucket_idx] != current_epoch {
            self.buckets[bucket_idx] = 0;
            self.bucket_epochs[bucket_idx] = current_epoch;
        }
        self.buckets[bucket_idx] = self.buckets[bucket_idx].saturating_add(1);

        self.last_observed_unix_nanos = now_nanos;
    }

    /// Current quiet duration in seconds (wall-clock time elapsed since the
    /// last observation). Returns 0 when no observation has occurred yet.
    pub fn current_quiet_duration_seconds(&self, now_nanos: i64) -> u64 {
        if self.last_observed_unix_nanos == 0 {
            return 0;
        }
        let elapsed_nanos = now_nanos.saturating_sub(self.last_observed_unix_nanos);
        (elapsed_nanos.max(0) / 1_000_000_000) as u64
    }

    /// Estimated p95 quiet-duration in seconds from the historical
    /// distribution. Returns `None` when fewer than 2 observations have
    /// occurred (no gap samples yet).
    pub fn p95_historical_quiet_duration_seconds(&self) -> Option<u64> {
        if self.quiet_duration_buffer.is_empty() && self.quiet_duration_digest.count() == 0.0 {
            return None;
        }
        let merged = if self.quiet_duration_buffer.is_empty() {
            self.quiet_duration_digest.clone()
        } else {
            self.quiet_duration_digest
                .clone()
                .merge_unsorted(self.quiet_duration_buffer.clone())
        };
        if merged.count() == 0.0 {
            None
        } else {
            let p95_seconds = merged.estimate_quantile(DEFAULT_QUIET_DURATION_PERCENTILE);
            Some(p95_seconds.max(0.0) as u64)
        }
    }

    /// Bootstrap state derived purely from internal clock — `now_nanos` minus
    /// `first_observed_unix_nanos` versus `BOOTSTRAP_WINDOW_SECONDS`. Per
    /// security extract: attacker-controlled OTLP attributes MUST NOT bypass
    /// this gate, so the derivation depends only on internal-clock state.
    pub fn bootstrap_state(&self, now_nanos: i64) -> BootstrapState {
        if self.first_observed_unix_nanos == 0 {
            return BootstrapState::Learning;
        }
        let elapsed_nanos = now_nanos.saturating_sub(self.first_observed_unix_nanos);
        let bootstrap_nanos = (BOOTSTRAP_WINDOW_SECONDS as i64).saturating_mul(1_000_000_000);
        if elapsed_nanos < bootstrap_nanos {
            BootstrapState::Learning
        } else {
            BootstrapState::Ready
        }
    }

    /// Number of populated bucket slots; the underlying Vec is always
    /// `BUCKET_COUNT` in length post-construction.
    pub fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    /// Sum of all bucket counts across the histogram (current 24h cycle's
    /// staleness-evicted total). Useful for observability metrics.
    pub fn total_observations_current_cycle(&self) -> u64 {
        self.buckets.iter().map(|&c| c as u64).sum()
    }

    pub fn first_observed_unix_nanos(&self) -> i64 {
        self.first_observed_unix_nanos
    }

    pub fn last_observed_unix_nanos(&self) -> i64 {
        self.last_observed_unix_nanos
    }

    fn flush_buffer(&mut self) {
        if self.quiet_duration_buffer.is_empty() {
            return;
        }
        let drained = std::mem::take(&mut self.quiet_duration_buffer);
        self.quiet_duration_digest = self.quiet_duration_digest.clone().merge_unsorted(drained);
    }
}

fn bucket_index_for(now_nanos: i64) -> usize {
    let secs = (now_nanos.max(0) / 1_000_000_000) as u64;
    let bucket_seconds = secs / BUCKET_INTERVAL_SECONDS;
    (bucket_seconds % BUCKET_COUNT as u64) as usize
}

fn epoch_for(now_nanos: i64) -> u64 {
    let secs = (now_nanos.max(0) / 1_000_000_000) as u64;
    secs / WINDOW_DURATION_SECONDS
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const NANOS_PER_SEC: i64 = 1_000_000_000;
    const BUCKET_NANOS: i64 = (BUCKET_INTERVAL_SECONDS as i64) * NANOS_PER_SEC;

    #[test]
    fn new_initializes_empty_state() {
        let af = ActivityFloor::new();
        assert_eq!(af.bucket_count(), BUCKET_COUNT);
        assert_eq!(af.total_observations_current_cycle(), 0);
        assert_eq!(af.last_observed_unix_nanos(), 0);
        assert_eq!(af.first_observed_unix_nanos(), 0);
        assert!(af.p95_historical_quiet_duration_seconds().is_none());
    }

    #[test]
    fn observe_advances_last_observed_and_current_bucket() {
        let mut af = ActivityFloor::new();
        let now = 1_000 * NANOS_PER_SEC;
        af.observe(now);
        assert_eq!(af.last_observed_unix_nanos(), now);
        assert_eq!(af.first_observed_unix_nanos(), now);
        assert_eq!(af.total_observations_current_cycle(), 1);
    }

    #[test]
    fn observe_within_same_bucket_does_not_advance_bucket_index() {
        let mut af = ActivityFloor::new();
        let base = 1_000 * NANOS_PER_SEC;
        af.observe(base);
        af.observe(base + 60 * NANOS_PER_SEC);
        af.observe(base + 120 * NANOS_PER_SEC);
        assert_eq!(af.total_observations_current_cycle(), 3);
        let bucket_idx = bucket_index_for(base);
        assert_eq!(af.buckets[bucket_idx], 3);
    }

    #[test]
    fn observe_crossing_bucket_boundary_advances_bucket_index() {
        let mut af = ActivityFloor::new();
        let bucket_a_start = 0_i64;
        let bucket_b_start = BUCKET_NANOS;
        af.observe(bucket_a_start);
        af.observe(bucket_b_start);
        let idx_a = bucket_index_for(bucket_a_start);
        let idx_b = bucket_index_for(bucket_b_start);
        assert_ne!(idx_a, idx_b);
        assert_eq!(af.buckets[idx_a], 1);
        assert_eq!(af.buckets[idx_b], 1);
    }

    #[test]
    fn observe_within_same_24h_cycle_does_not_reset_buckets() {
        let mut af = ActivityFloor::new();
        let base = 1_000 * NANOS_PER_SEC;
        af.observe(base);
        af.observe(base + 60 * NANOS_PER_SEC);
        let bucket_idx = bucket_index_for(base);
        assert_eq!(af.buckets[bucket_idx], 2);
    }

    #[test]
    fn observe_crossing_24h_boundary_resets_stale_bucket() {
        let mut af = ActivityFloor::new();
        let base = 0_i64;
        af.observe(base);
        let bucket_idx = bucket_index_for(base);
        assert_eq!(af.buckets[bucket_idx], 1);
        let next_cycle = ((WINDOW_DURATION_SECONDS as i64) + 30) * NANOS_PER_SEC;
        let next_idx = bucket_index_for(next_cycle);
        assert_eq!(next_idx, bucket_idx);
        af.observe(next_cycle);
        assert_eq!(af.buckets[bucket_idx], 1);
    }

    #[test]
    fn bootstrap_state_returns_learning_when_never_observed() {
        let af = ActivityFloor::new();
        let now = 100 * NANOS_PER_SEC;
        assert_eq!(af.bootstrap_state(now), BootstrapState::Learning);
    }

    #[test]
    fn bootstrap_state_returns_learning_for_first_hour() {
        let mut af = ActivityFloor::new();
        let first = 1_000 * NANOS_PER_SEC;
        af.observe(first);
        let still_learning = first + (BOOTSTRAP_WINDOW_SECONDS as i64 - 1) * NANOS_PER_SEC;
        assert_eq!(af.bootstrap_state(still_learning), BootstrapState::Learning);
    }

    #[test]
    fn bootstrap_state_returns_ready_after_first_hour() {
        let mut af = ActivityFloor::new();
        let first = 1_000 * NANOS_PER_SEC;
        af.observe(first);
        let post_bootstrap = first + (BOOTSTRAP_WINDOW_SECONDS as i64 + 1) * NANOS_PER_SEC;
        assert_eq!(af.bootstrap_state(post_bootstrap), BootstrapState::Ready);
    }

    #[test]
    fn p95_historical_quiet_duration_returns_none_before_two_observations() {
        let mut af = ActivityFloor::new();
        assert!(af.p95_historical_quiet_duration_seconds().is_none());
        af.observe(1_000 * NANOS_PER_SEC);
        assert!(af.p95_historical_quiet_duration_seconds().is_none());
    }

    #[test]
    fn p95_historical_quiet_duration_returns_some_after_multiple_observations() {
        let mut af = ActivityFloor::new();
        let base = 1_000 * NANOS_PER_SEC;
        for i in 0..120 {
            af.observe(base + (i * 60) * NANOS_PER_SEC);
        }
        let p95 = af
            .p95_historical_quiet_duration_seconds()
            .expect("p95 populated after many gap samples");
        assert!(p95 > 0, "p95 should reflect ~60s gap pattern, got {p95}");
        assert!(p95 < 600, "p95 should not exceed ~10x base gap, got {p95}");
    }

    #[test]
    fn current_quiet_duration_seconds_returns_zero_before_first_observation() {
        let af = ActivityFloor::new();
        assert_eq!(af.current_quiet_duration_seconds(1_000 * NANOS_PER_SEC), 0);
    }

    #[test]
    fn current_quiet_duration_seconds_increases_with_time_since_last_observe() {
        let mut af = ActivityFloor::new();
        let first = 1_000 * NANOS_PER_SEC;
        af.observe(first);
        assert_eq!(af.current_quiet_duration_seconds(first), 0);
        assert_eq!(
            af.current_quiet_duration_seconds(first + 30 * NANOS_PER_SEC),
            30
        );
        assert_eq!(
            af.current_quiet_duration_seconds(first + 120 * NANOS_PER_SEC),
            120
        );
    }

    #[test]
    fn serde_round_trip_preserves_buckets_and_digest() {
        let mut af = ActivityFloor::new();
        let base = 1_000 * NANOS_PER_SEC;
        for i in 0..50 {
            af.observe(base + (i * 60) * NANOS_PER_SEC);
        }
        let bytes = bincode::serialize(&af).expect("serialize");
        let restored: ActivityFloor = bincode::deserialize(&bytes).expect("deserialize");
        assert_eq!(restored.buckets, af.buckets);
        assert_eq!(restored.bucket_epochs, af.bucket_epochs);
        assert_eq!(
            restored.last_observed_unix_nanos,
            af.last_observed_unix_nanos
        );
        assert_eq!(
            restored.first_observed_unix_nanos,
            af.first_observed_unix_nanos
        );
        assert_eq!(
            restored.total_observations_current_cycle(),
            af.total_observations_current_cycle()
        );
        let p95_before = af.p95_historical_quiet_duration_seconds();
        let p95_after = restored.p95_historical_quiet_duration_seconds();
        assert_eq!(p95_before.is_some(), p95_after.is_some());
    }

    #[test]
    fn bucket_index_for_wraps_modulo_bucket_count() {
        for slot in [
            0_i64,
            1,
            BUCKET_COUNT as i64 - 1,
            BUCKET_COUNT as i64,
            BUCKET_COUNT as i64 + 5,
        ] {
            let now = slot * BUCKET_NANOS;
            let idx = bucket_index_for(now);
            assert!(idx < BUCKET_COUNT, "idx {idx} should be < {BUCKET_COUNT}");
        }
    }

    #[test]
    fn bucket_index_for_handles_negative_timestamps_safely() {
        let idx = bucket_index_for(-1_000_000_000);
        assert!(idx < BUCKET_COUNT);
    }

    proptest! {
        #[test]
        fn prop_24h_rolling_invariant(
            timestamps in prop::collection::vec(0i64..(7 * (WINDOW_DURATION_SECONDS as i64) * NANOS_PER_SEC), 0..500),
        ) {
            let mut af = ActivityFloor::new();
            let mut sorted = timestamps;
            sorted.sort_unstable();
            for ts in &sorted {
                af.observe(*ts);
            }
            prop_assert_eq!(af.bucket_count(), BUCKET_COUNT);
            let total: u64 = af.total_observations_current_cycle();
            prop_assert!(total <= sorted.len() as u64);
            for &count in &af.buckets {
                prop_assert!(count <= sorted.len() as u32);
            }
        }

        #[test]
        fn prop_observe_never_panics_under_arbitrary_timestamps(
            timestamps in prop::collection::vec(any::<i64>(), 0..100),
        ) {
            let mut af = ActivityFloor::new();
            for ts in &timestamps {
                af.observe(*ts);
            }
            let now = timestamps.iter().copied().max().unwrap_or(0).saturating_add(NANOS_PER_SEC);
            let _ = af.bootstrap_state(now);
            let _ = af.current_quiet_duration_seconds(now);
            let _ = af.p95_historical_quiet_duration_seconds();
        }

        #[test]
        fn prop_bucket_index_always_in_range(now_nanos in any::<i64>()) {
            let idx = bucket_index_for(now_nanos);
            prop_assert!(idx < BUCKET_COUNT);
        }
    }
}
