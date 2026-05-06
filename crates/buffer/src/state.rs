use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct BufferState {
    rows_ingested: AtomicU64,
    eviction_count: AtomicU64,
    memory_bytes: AtomicU64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferStateSnapshot {
    pub rows_ingested: u64,
    pub eviction_count: u64,
    pub memory_bytes: u64,
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
        }
    }

    pub fn record_rows_appended(&self, n: u64) {
        self.rows_ingested.fetch_add(n, Ordering::Relaxed);
    }

    pub fn record_eviction(&self, n: u64) {
        self.eviction_count.fetch_add(n, Ordering::Relaxed);
    }

    pub fn set_memory_bytes(&self, n: u64) {
        self.memory_bytes.store(n, Ordering::Relaxed);
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
    }

    #[test]
    fn record_rows_appended_accumulates() {
        let s = BufferState::new();
        s.record_rows_appended(5);
        s.record_rows_appended(7);
        assert_eq!(s.snapshot().rows_ingested, 12);
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
}
