use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct IngestState {
    span_count: AtomicU64,
    log_record_count: AtomicU64,
    metric_data_point_count: AtomicU64,
    broadcast_subscribers: AtomicU32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IngestStateSnapshot {
    pub span_count: u64,
    pub log_record_count: u64,
    pub metric_data_point_count: u64,
    pub broadcast_subscribers: u32,
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
        }
    }

    pub fn record_spans(&self, n: u64) {
        self.span_count.fetch_add(n, Ordering::Relaxed);
    }

    pub fn record_log_records(&self, n: u64) {
        self.log_record_count.fetch_add(n, Ordering::Relaxed);
    }

    pub fn record_metric_data_points(&self, n: u64) {
        self.metric_data_point_count.fetch_add(n, Ordering::Relaxed);
    }

    pub fn set_broadcast_subscribers(&self, n: u32) {
        self.broadcast_subscribers.store(n, Ordering::Relaxed);
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
}
