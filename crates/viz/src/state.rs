use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct VizState {
    active_subscribers: AtomicU32,
    total_query_latency_ms_sum: AtomicU64,
    query_count: AtomicU64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VizStateSnapshot {
    pub subscribers_active: u32,
    pub query_latency_ms_avg: f64,
    pub query_count: u64,
}

impl VizState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn snapshot(&self) -> VizStateSnapshot {
        let count = self.query_count.load(Ordering::Relaxed);
        let sum = self.total_query_latency_ms_sum.load(Ordering::Relaxed);
        let avg = if count == 0 {
            0.0
        } else {
            sum as f64 / count as f64
        };
        VizStateSnapshot {
            subscribers_active: self.active_subscribers.load(Ordering::Relaxed),
            query_latency_ms_avg: avg,
            query_count: count,
        }
    }

    pub fn record_query_latency_ms(&self, ms: u64) {
        self.total_query_latency_ms_sum
            .fetch_add(ms, Ordering::Relaxed);
        self.query_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_subscribers(&self) {
        self.active_subscribers.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec_subscribers(&self) {
        self.active_subscribers.fetch_sub(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn snapshot_is_zero_at_init() {
        let s = VizState::new();
        let snap = s.snapshot();
        assert_eq!(snap.subscribers_active, 0);
        assert_eq!(snap.query_latency_ms_avg, 0.0);
        assert_eq!(snap.query_count, 0);
    }

    #[test]
    fn record_query_latency_accumulates_avg() {
        let s = VizState::new();
        s.record_query_latency_ms(50);
        s.record_query_latency_ms(150);
        let snap = s.snapshot();
        assert_eq!(snap.query_count, 2);
        assert_eq!(snap.query_latency_ms_avg, 100.0);
    }

    #[test]
    fn inc_dec_subscribers_tracks_count() {
        let s = VizState::new();
        s.inc_subscribers();
        s.inc_subscribers();
        s.inc_subscribers();
        s.dec_subscribers();
        assert_eq!(s.snapshot().subscribers_active, 2);
    }

    #[test]
    fn snapshot_avg_zero_when_no_queries_recorded() {
        let s = VizState::new();
        s.inc_subscribers();
        let snap = s.snapshot();
        assert_eq!(snap.subscribers_active, 1);
        assert_eq!(snap.query_latency_ms_avg, 0.0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_record_query_latency_accumulates_correctly() {
        let s = Arc::new(VizState::new());
        let mut handles = Vec::new();
        for _ in 0..8 {
            let s = Arc::clone(&s);
            handles.push(tokio::spawn(async move {
                for _ in 0..100 {
                    s.record_query_latency_ms(10);
                }
            }));
        }
        for h in handles {
            h.await.unwrap();
        }
        let snap = s.snapshot();
        assert_eq!(snap.query_count, 800);
        assert_eq!(snap.query_latency_ms_avg, 10.0);
    }
}
