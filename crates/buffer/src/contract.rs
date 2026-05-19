use thiserror::Error;

use crate::state::BufferState;

#[derive(Debug, Error)]
pub enum Error {
    #[error("buffer init failed: {reason}")]
    Init { reason: String },
    #[error("buffer schema create failed: {reason}")]
    SchemaCreate { reason: String },
    #[error("buffer Arrow append failed: {reason}")]
    Append { reason: String },
    #[error("buffer connection lost")]
    ConnectionLost,
    #[error("buffer received invalid batch: {kind}")]
    InvalidBatch { kind: &'static str },
    #[error("buffer retention sweep failed: {reason}")]
    Retention { reason: String },
    #[error("buffer broadcast encode failed: {reason}")]
    BroadcastEncode { reason: String },
    #[error("buffer broadcast payload exceeded size cap: {payload_bytes} bytes")]
    BroadcastSizeCapExceeded { payload_bytes: usize },
    // Chunk #69 Phase B — Drain template-mining errors. Sanitized message strings
    // only; no log content / template body strings (per security plan §Logging
    // NEVER-log discipline + obs-plan §8 default-deny posture).
    #[error("buffer Drain operation failed: {reason}")]
    Drain { reason: String },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferHeartbeat {
    pub rows_ingested: u64,
    pub retention_window_active: bool,
    pub eviction_count: u64,
    pub memory_bytes: u64,
    pub retention_window_seconds: u64,
    pub eviction_count_since_last_tick: u64,
    // Chunk #69 Phase B Session 7+ — Drain heartbeat fields. Pre-registered
    // in the `buffer` AllowList entry at observability.rs:159-160 (Session
    // 6); emission lands in pulse-app/src/heartbeat.rs::emit_buffer_tick
    // when a DrainMiner ref is threaded in (Some) — otherwise both default
    // to 0 (matches the all-zero default for the Default impl baseline).
    pub drain_template_count: u64,
    pub drain_lru_evictions_since_tick: u64,
}

pub fn heartbeat_payload(
    state: &BufferState,
    retention_window_seconds: u64,
    eviction_count_since_last_tick: u64,
    drain_template_count: u64,
    drain_lru_evictions_since_tick: u64,
) -> BufferHeartbeat {
    let snap = state.snapshot();
    BufferHeartbeat {
        rows_ingested: snap.rows_ingested,
        retention_window_active: snap.retention_window_active,
        eviction_count: snap.eviction_count,
        memory_bytes: snap.memory_bytes,
        retention_window_seconds,
        eviction_count_since_last_tick,
        drain_template_count,
        drain_lru_evictions_since_tick,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_default_state_is_all_zero() {
        let state = BufferState::new();
        let h = heartbeat_payload(&state, 0, 0, 0, 0);
        assert_eq!(h.rows_ingested, 0);
        assert!(!h.retention_window_active);
        assert_eq!(h.eviction_count, 0);
        assert_eq!(h.memory_bytes, 0);
        assert_eq!(h.retention_window_seconds, 0);
        assert_eq!(h.eviction_count_since_last_tick, 0);
        assert_eq!(h.drain_template_count, 0);
        assert_eq!(h.drain_lru_evictions_since_tick, 0);
    }

    #[test]
    fn heartbeat_payload_reflects_recorded_rows_ingested() {
        let state = BufferState::new();
        state.record_rows_appended(42);
        let h = heartbeat_payload(&state, 600, 0, 0, 0);
        assert_eq!(h.rows_ingested, 42);
        assert_eq!(h.retention_window_seconds, 600);
    }

    #[test]
    fn heartbeat_payload_reflects_eviction_count() {
        let state = BufferState::new();
        state.record_eviction(7);
        let h = heartbeat_payload(&state, 600, 7, 0, 0);
        assert_eq!(h.eviction_count, 7);
        assert_eq!(h.eviction_count_since_last_tick, 7);
    }

    #[test]
    fn heartbeat_payload_reflects_retention_active_flip() {
        let state = BufferState::new();
        let h_pre = heartbeat_payload(&state, 600, 0, 0, 0);
        assert!(!h_pre.retention_window_active);
        state.mark_retention_active();
        let h_post = heartbeat_payload(&state, 600, 0, 0, 0);
        assert!(h_post.retention_window_active);
    }

    #[test]
    fn heartbeat_payload_reflects_memory_bytes() {
        let state = BufferState::new();
        state.set_memory_bytes(2048);
        let h = heartbeat_payload(&state, 600, 0, 0, 0);
        assert_eq!(h.memory_bytes, 2048);
    }

    #[test]
    fn heartbeat_payload_carries_drain_fields_through() {
        // chunk #69 Phase B Session 7+: drain_template_count +
        // drain_lru_evictions_since_tick flow through without alteration.
        let state = BufferState::new();
        let h = heartbeat_payload(&state, 600, 0, 42, 7);
        assert_eq!(h.drain_template_count, 42);
        assert_eq!(h.drain_lru_evictions_since_tick, 7);
    }

    #[test]
    fn error_init_displays_reason() {
        let e = Error::Init {
            reason: "open_in_memory".to_string(),
        };
        assert!(format!("{e}").contains("open_in_memory"));
    }

    #[test]
    fn error_invalid_batch_carries_kind() {
        let e = Error::InvalidBatch {
            kind: "spans_empty",
        };
        assert!(format!("{e}").contains("spans_empty"));
    }

    #[test]
    fn error_retention_displays_reason() {
        let e = Error::Retention {
            reason: "prepare: connection lost".to_string(),
        };
        assert!(format!("{e}").contains("prepare: connection lost"));
        assert!(format!("{e}").contains("retention sweep failed"));
    }
}
