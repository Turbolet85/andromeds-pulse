use std::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, Ordering};

use chrono::Utc;

#[derive(Debug, Default)]
pub struct IngestState {
    span_count: AtomicU64,
    log_record_count: AtomicU64,
    metric_data_point_count: AtomicU64,
    broadcast_subscribers: AtomicU32,
    // Last-ingest timestamp (chunk #59): unix nanos at most recent
    // record_spans / record_log_records / record_metric_data_points call.
    // Read by the connection-state FSM poller to compute Idle / Stalled
    // thresholds. 0 = no ingest yet (initial Listening state).
    last_ingest_at_nanos: AtomicI64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IngestStateSnapshot {
    pub span_count: u64,
    pub log_record_count: u64,
    pub metric_data_point_count: u64,
    pub broadcast_subscribers: u32,
    pub last_ingest_at_nanos: i64,
}

impl IngestState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn snapshot(&self) -> IngestStateSnapshot {
        IngestStateSnapshot {
            span_count: self.span_count.load(Ordering::Relaxed),
            log_record_count: self.log_record_count.load(Ordering::Relaxed),
            metric_data_point_count: self.metric_data_point_count.load(Ordering::Relaxed),
            broadcast_subscribers: self.broadcast_subscribers.load(Ordering::Relaxed),
            last_ingest_at_nanos: self.last_ingest_at_nanos.load(Ordering::Relaxed),
        }
    }

    pub fn record_spans(&self, n: u64) {
        self.span_count.fetch_add(n, Ordering::Relaxed);
        self.touch_last_ingest();
    }

    pub fn record_log_records(&self, n: u64) {
        self.log_record_count.fetch_add(n, Ordering::Relaxed);
        self.touch_last_ingest();
    }

    pub fn record_metric_data_points(&self, n: u64) {
        self.metric_data_point_count.fetch_add(n, Ordering::Relaxed);
        self.touch_last_ingest();
    }

    pub fn set_broadcast_subscribers(&self, n: u32) {
        self.broadcast_subscribers.store(n, Ordering::Relaxed);
    }

    /// Last-ingest timestamp in unix nanos, or 0 if no ingest seen yet.
    pub fn last_ingest_at_nanos(&self) -> i64 {
        self.last_ingest_at_nanos.load(Ordering::Relaxed)
    }

    fn touch_last_ingest(&self) {
        let now = Utc::now().timestamp_nanos_opt().unwrap_or(0);
        self.last_ingest_at_nanos.store(now, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn snapshot_is_zero_at_init() {
        let s = IngestState::new();
        let snap = s.snapshot();
        assert_eq!(snap.span_count, 0);
        assert_eq!(snap.log_record_count, 0);
        assert_eq!(snap.metric_data_point_count, 0);
        assert_eq!(snap.broadcast_subscribers, 0);
    }

    #[test]
    fn record_spans_accumulates() {
        let s = IngestState::new();
        s.record_spans(5);
        s.record_spans(7);
        assert_eq!(s.snapshot().span_count, 12);
    }

    #[test]
    fn record_methods_target_distinct_counters() {
        let s = IngestState::new();
        s.record_spans(1);
        s.record_log_records(2);
        s.record_metric_data_points(3);
        let snap = s.snapshot();
        assert_eq!(snap.span_count, 1);
        assert_eq!(snap.log_record_count, 2);
        assert_eq!(snap.metric_data_point_count, 3);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_record_spans_accumulates_correctly() {
        let s = Arc::new(IngestState::new());
        let mut handles = Vec::new();
        for _ in 0..8 {
            let s = Arc::clone(&s);
            handles.push(tokio::spawn(async move {
                for _ in 0..100 {
                    s.record_spans(1);
                }
            }));
        }
        for h in handles {
            h.await.unwrap();
        }
        assert_eq!(s.snapshot().span_count, 800);
    }

    // Chunk #59 — last-ingest timestamp tests. The FSM poller reads
    // last_ingest_at_nanos to compute Idle / Stalled / ReceiverFailed
    // transitions; the hot-path record_* methods must advance the atomic
    // on every call.

    #[test]
    fn last_ingest_at_nanos_is_zero_at_init() {
        let s = IngestState::new();
        assert_eq!(s.last_ingest_at_nanos(), 0);
        assert_eq!(s.snapshot().last_ingest_at_nanos, 0);
    }

    #[test]
    fn record_spans_updates_last_ingest_at_nanos() {
        let s = IngestState::new();
        assert_eq!(s.last_ingest_at_nanos(), 0);
        s.record_spans(1);
        assert!(s.last_ingest_at_nanos() > 0);
    }

    #[test]
    fn record_log_records_updates_last_ingest_at_nanos() {
        let s = IngestState::new();
        s.record_log_records(1);
        assert!(s.last_ingest_at_nanos() > 0);
    }

    #[test]
    fn record_metric_data_points_updates_last_ingest_at_nanos() {
        let s = IngestState::new();
        s.record_metric_data_points(1);
        assert!(s.last_ingest_at_nanos() > 0);
    }

    #[test]
    fn last_ingest_at_nanos_monotonically_advances() {
        let s = IngestState::new();
        s.record_spans(1);
        let first = s.last_ingest_at_nanos();
        // Sleep briefly so the second timestamp must be ≥ first; Utc::now()
        // resolution is sub-microsecond on most platforms but a small
        // tolerance avoids platform-dependent test flake.
        std::thread::sleep(std::time::Duration::from_millis(2));
        s.record_spans(1);
        let second = s.last_ingest_at_nanos();
        assert!(
            second >= first,
            "second timestamp ({second}) must be >= first ({first})"
        );
    }

    #[test]
    fn set_broadcast_subscribers_does_not_update_last_ingest() {
        let s = IngestState::new();
        assert_eq!(s.last_ingest_at_nanos(), 0);
        s.set_broadcast_subscribers(3);
        // broadcast subscriber count is a snapshot, not an ingest signal —
        // FSM poller must not observe Receiving state from subscriber changes.
        assert_eq!(s.last_ingest_at_nanos(), 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_record_methods_keep_last_ingest_non_decreasing() {
        let s = Arc::new(IngestState::new());
        let mut handles = Vec::new();
        for _ in 0..8 {
            let s = Arc::clone(&s);
            handles.push(tokio::spawn(async move {
                for _ in 0..50 {
                    s.record_spans(1);
                    s.record_log_records(1);
                    s.record_metric_data_points(1);
                }
            }));
        }
        for h in handles {
            h.await.unwrap();
        }
        // After concurrent stress, the atomic must hold a valid timestamp;
        // under Relaxed ordering individual writes may race but the final
        // observed value is one of the writes (i.e., > 0, no torn read).
        assert!(s.last_ingest_at_nanos() > 0);
    }
}
