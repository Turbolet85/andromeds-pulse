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
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferStateSnapshot {
    pub rows_ingested: u64,
    pub eviction_count: u64,
    pub memory_bytes: u64,
    pub first_append_at_nanos: u64,
    pub retention_window_active: bool,
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
            retention_window_active: self.retention_window_active.load(Ordering::Relaxed),
        }
    }

    pub fn record_rows_appended(&self, n: u64) {
        self.rows_ingested.fetch_add(n, Ordering::Relaxed);
        // Anchor the first-append wall-clock once; the load-guard skips the
        // now() cost on every append after the first has landed.
        if self.first_append_at_nanos.load(Ordering::Relaxed) == 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
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
}
