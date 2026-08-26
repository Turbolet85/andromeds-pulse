use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default)]
pub struct BufferState {
    rows_ingested: AtomicU64,
    eviction_count: AtomicU64,
    memory_bytes: AtomicU64,
    // First-append wall-clock anchor (nanos), set once on the first append
    // (0 until then). Backs ReadyChecks.buffer_used_seconds — the honest
    // data-span for the P-070 status line, capped at retention downstream.
    first_append_at_nanos: AtomicU64,
    // Latching sentinel — flipped to true after the first successful retention
    // sweep completes; never resets. Distinguishes "buffer has been alive
    // long enough for retention to fire at least once" from "first-tick state".
    retention_window_active: AtomicBool,
    // Fingerprint-feed throughput. Without these, "nothing reached the storm
    // detector" is only ever inferable from the ABSENCE of downstream signal,
    // which is what let a dead feed go unnoticed. Accumulated per batch (not
    // per event) to keep the appender's per-event loop free of atomics.
    span_events_seen: AtomicU64,
    fingerprints_computed: AtomicU64,
    observer_invocations: AtomicU64,
    // PII redactions applied on the OTLP persistence path, so scrubber recall
    // is gradeable from outside the process instead of only by reading stored
    // rows. Spans all four record-batch builders — NOT the drain/template
    // path, which carries its own tick fields. Counts PERSISTED cells only:
    // the builders return their tally and the append site folds it after the
    // table's own append succeeds, so a batch rejected at `flush()` adds
    // nothing. Accumulated per batch, never per field.
    redactions_applied: AtomicU64,
    // Wall-clock (nanos) of the MOST RECENT append, refreshed on every batch —
    // distinct from `first_append_at_nanos`, which latches once. Without it a
    // stalled consumer is indistinguishable from an idle producer: both leave
    // `rows_ingested` static, and the heartbeat carries no notion of when the
    // last row actually landed.
    last_append_at_nanos: AtomicU64,
    // Monotonic allocators for the `log_records.seq` / `metrics_points.seq`
    // primary-key ordinals. Separate per table: one shared counter would
    // couple two tables' allocation for no gain. Not observables: never
    // folded into BufferStateSnapshot or `buffer.tick`.
    log_seq: AtomicU64,
    metric_seq: AtomicU64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferStateSnapshot {
    pub rows_ingested: u64,
    pub eviction_count: u64,
    pub memory_bytes: u64,
    pub first_append_at_nanos: u64,
    pub last_append_at_nanos: u64,
    pub retention_window_active: bool,
    pub span_events_seen: u64,
    pub fingerprints_computed: u64,
    pub observer_invocations: u64,
    pub redactions_applied: u64,
}

impl BufferState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn snapshot(&self) -> BufferStateSnapshot {
        BufferStateSnapshot {
            rows_ingested: self.rows_ingested.load(Ordering::Relaxed),
            eviction_count: self.eviction_count.load(Ordering::Relaxed),
            memory_bytes: self.memory_bytes.load(Ordering::Relaxed),
            first_append_at_nanos: self.first_append_at_nanos.load(Ordering::Relaxed),
            last_append_at_nanos: self.last_append_at_nanos.load(Ordering::Relaxed),
            retention_window_active: self.retention_window_active.load(Ordering::Relaxed),
            span_events_seen: self.span_events_seen.load(Ordering::Relaxed),
            fingerprints_computed: self.fingerprints_computed.load(Ordering::Relaxed),
            observer_invocations: self.observer_invocations.load(Ordering::Relaxed),
            redactions_applied: self.redactions_applied.load(Ordering::Relaxed),
        }
    }

    pub fn record_rows_appended(&self, n: u64) {
        self.rows_ingested.fetch_add(n, Ordering::Relaxed);
        // One now() per batch serves both anchors — the append itself is
        // DuckDB I/O, so a single clock read here is not a hot-path cost.
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        self.last_append_at_nanos.store(now, Ordering::Relaxed);
        if self.first_append_at_nanos.load(Ordering::Relaxed) == 0 {
            let _ = self.first_append_at_nanos.compare_exchange(
                0,
                now,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
        }
    }

    pub fn record_eviction(&self, n: u64) {
        self.eviction_count.fetch_add(n, Ordering::Relaxed);
    }

    /// Fold one batch's fingerprint-feed tallies in. Called once per span
    /// batch so the per-event loop stays atomic-free.
    pub fn record_feed_counts(
        &self,
        span_events_seen: u64,
        fingerprints_computed: u64,
        observer_invocations: u64,
    ) {
        self.span_events_seen
            .fetch_add(span_events_seen, Ordering::Relaxed);
        self.fingerprints_computed
            .fetch_add(fingerprints_computed, Ordering::Relaxed);
        self.observer_invocations
            .fetch_add(observer_invocations, Ordering::Relaxed);
    }

    /// Fold one batch's PII-redaction tally in. Separate from
    /// [`Self::record_feed_counts`] because the log-record path applies
    /// redactions without producing any fingerprint-feed counts.
    pub fn record_redactions(&self, n: u64) {
        if n > 0 {
            self.redactions_applied.fetch_add(n, Ordering::Relaxed);
        }
    }

    /// Reserve `n` contiguous `log_records.seq` values and return the first.
    ///
    /// An OTLP LogRecord carries no spec-defined unique id, and its OTLP-native
    /// columns do not separate two records emitted by one resource in the same
    /// nanosecond at the same severity — so the primary key needs an ordinal.
    /// A per-batch ordinal would not do: it resets, so records colliding across
    /// two export batches would both take index 0. This counter is buffer-global.
    ///
    /// Deliberately absent from [`BufferStateSnapshot`] — it is an allocator,
    /// not an observable, and `buffer.tick` must not carry it.
    pub fn reserve_log_seq_block(&self, n: u64) -> u64 {
        self.log_seq.fetch_add(n, Ordering::Relaxed)
    }

    /// Reserve `n` contiguous `metrics_points.seq` values and return the first.
    ///
    /// `metrics_points` stores no attributes column, so two data points of one
    /// metric differing only by label set are identical on every OTLP-native key
    /// column; since the metric name is scrubbed at ingestion, two distinct
    /// credential-shaped names also redact to one placeholder and collide. Both
    /// need an ordinal. A per-batch ordinal would not do — `metrics_points` has
    /// no unique parent scoping it, so points colliding across two export
    /// batches would both take index 0. This counter is buffer-global.
    ///
    /// Deliberately absent from [`BufferStateSnapshot`] — it is an allocator,
    /// not an observable, and `buffer.tick` must not carry it.
    pub fn reserve_metric_seq_block(&self, n: u64) -> u64 {
        self.metric_seq.fetch_add(n, Ordering::Relaxed)
    }

    pub fn set_memory_bytes(&self, n: u64) {
        self.memory_bytes.store(n, Ordering::Relaxed);
    }

    pub fn mark_retention_active(&self) {
        self.retention_window_active.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn snapshot_is_zero_at_init() {
        let s = BufferState::new();
        let snap = s.snapshot();
        assert_eq!(snap.rows_ingested, 0);
        assert_eq!(snap.eviction_count, 0);
        assert_eq!(snap.memory_bytes, 0);
        assert!(!snap.retention_window_active);
    }

    #[test]
    fn record_rows_appended_accumulates() {
        let s = BufferState::new();
        s.record_rows_appended(5);
        s.record_rows_appended(7);
        assert_eq!(s.snapshot().rows_ingested, 12);
    }

    #[test]
    fn last_append_is_zero_until_first_record() {
        let s = BufferState::new();
        assert_eq!(s.snapshot().last_append_at_nanos, 0);
    }

    #[test]
    fn last_append_refreshes_while_first_append_latches() {
        // The two anchors must NOT be the same semantic: `first` latches once
        // (it dates the buffer), `last` refreshes on every batch (it dates the
        // most recent row). A `last` that latched would report an ever-growing
        // age on a perfectly healthy buffer.
        let s = BufferState::new();
        s.record_rows_appended(1);
        let after_first = s.snapshot();
        assert_ne!(after_first.last_append_at_nanos, 0);
        assert_eq!(
            after_first.first_append_at_nanos,
            after_first.last_append_at_nanos,
        );

        s.record_rows_appended(1);
        let after_second = s.snapshot();
        assert_eq!(
            after_second.first_append_at_nanos, after_first.first_append_at_nanos,
            "first_append_at_nanos must latch on the first append only",
        );
        assert!(
            after_second.last_append_at_nanos >= after_first.last_append_at_nanos,
            "last_append_at_nanos must be refreshed by every append",
        );
    }

    #[test]
    fn first_append_is_zero_until_first_record_then_set_once() {
        let s = BufferState::new();
        assert_eq!(s.snapshot().first_append_at_nanos, 0);
        s.record_rows_appended(1);
        let first = s.snapshot().first_append_at_nanos;
        assert_ne!(first, 0, "first append timestamp set on first record");
        s.record_rows_appended(1);
        assert_eq!(
            s.snapshot().first_append_at_nanos,
            first,
            "set-once: later records do not move the anchor"
        );
    }

    #[test]
    fn record_methods_target_distinct_counters() {
        let s = BufferState::new();
        s.record_rows_appended(1);
        s.record_eviction(2);
        s.set_memory_bytes(3);
        let snap = s.snapshot();
        assert_eq!(snap.rows_ingested, 1);
        assert_eq!(snap.eviction_count, 2);
        assert_eq!(snap.memory_bytes, 3);
    }

    #[test]
    fn retention_window_active_starts_false() {
        let s = BufferState::new();
        assert!(!s.snapshot().retention_window_active);
    }

    #[test]
    fn mark_retention_active_flips_to_true() {
        let s = BufferState::new();
        s.mark_retention_active();
        assert!(s.snapshot().retention_window_active);
    }

    #[test]
    fn mark_retention_active_is_idempotent() {
        let s = BufferState::new();
        s.mark_retention_active();
        s.mark_retention_active();
        s.mark_retention_active();
        assert!(s.snapshot().retention_window_active);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_record_rows_appended_accumulates_correctly() {
        let s = Arc::new(BufferState::new());
        let mut handles = Vec::new();
        for _ in 0..8 {
            let s = Arc::clone(&s);
            handles.push(tokio::spawn(async move {
                for _ in 0..100 {
                    s.record_rows_appended(1);
                }
            }));
        }
        for h in handles {
            h.await.unwrap();
        }
        assert_eq!(s.snapshot().rows_ingested, 800);
    }

    // Both ordinal allocators shipped proven only through their one caller, so a
    // caller-side change could silently retire their only coverage. These assert
    // the allocator contract directly: block width, monotonicity, and that the
    // two counters are independent.
    #[test]
    fn reserve_seq_blocks_are_contiguous_and_monotonic() {
        let s = BufferState::new();

        assert_eq!(s.reserve_log_seq_block(3), 0, "first block starts at 0");
        assert_eq!(s.reserve_log_seq_block(2), 3, "next block starts past it");
        assert_eq!(s.reserve_log_seq_block(1), 5);

        assert_eq!(s.reserve_metric_seq_block(4), 0);
        assert_eq!(s.reserve_metric_seq_block(1), 4);
    }

    #[test]
    fn reserve_seq_block_of_zero_does_not_advance() {
        let s = BufferState::new();
        assert_eq!(s.reserve_log_seq_block(0), 0);
        assert_eq!(
            s.reserve_log_seq_block(1),
            0,
            "an empty batch consumes none"
        );
        assert_eq!(s.reserve_metric_seq_block(0), 0);
        assert_eq!(s.reserve_metric_seq_block(1), 0);
    }

    // A shared counter would couple two tables' allocation; keep them separate.
    #[test]
    fn log_and_metric_seq_allocators_are_independent() {
        let s = BufferState::new();
        s.reserve_log_seq_block(10);
        assert_eq!(
            s.reserve_metric_seq_block(1),
            0,
            "metric ordinals must not advance with log ordinals"
        );
        s.reserve_metric_seq_block(10);
        assert_eq!(s.reserve_log_seq_block(1), 10);
    }

    // Neither allocator may become an observable.
    #[test]
    fn seq_allocators_are_absent_from_the_snapshot() {
        let s = BufferState::new();
        s.reserve_log_seq_block(7);
        s.reserve_metric_seq_block(7);
        let snap = s.snapshot();
        assert_eq!(snap.rows_ingested, 0);
        assert_eq!(snap.redactions_applied, 0);
        assert_eq!(snap.span_events_seen, 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_metric_seq_reservations_never_overlap() {
        let s = std::sync::Arc::new(BufferState::new());
        let mut handles = Vec::new();
        for _ in 0..8 {
            let s = s.clone();
            handles.push(tokio::spawn(async move {
                let mut bases = Vec::new();
                for _ in 0..50 {
                    bases.push(s.reserve_metric_seq_block(2));
                }
                bases
            }));
        }
        let mut all = Vec::new();
        for h in handles {
            all.extend(h.await.unwrap());
        }
        all.sort_unstable();
        // 8 tasks x 50 reservations of width 2 = 400 disjoint blocks.
        assert_eq!(all.len(), 400);
        let expected: Vec<u64> = (0..400).map(|i| i * 2).collect();
        assert_eq!(all, expected, "every reserved block must be disjoint");
    }
}
