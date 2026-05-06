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
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferHeartbeat {
    pub rows_ingested: u64,
    pub retention_window_active: bool,
    pub eviction_count: u64,
    pub memory_bytes: u64,
    pub retention_window_seconds: u64,
    pub eviction_count_since_last_tick: u64,
}

pub fn heartbeat_payload(
    state: &BufferState,
    retention_window_seconds: u64,
    eviction_count_since_last_tick: u64,
) -> BufferHeartbeat {
    let snap = state.snapshot();
    BufferHeartbeat {
        rows_ingested: snap.rows_ingested,
        retention_window_active: snap.retention_window_active,
        eviction_count: snap.eviction_count,
        memory_bytes: snap.memory_bytes,
        retention_window_seconds,
        eviction_count_since_last_tick,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_default_state_is_all_zero() {
        let state = BufferState::new();
        let h = heartbeat_payload(&state, 0, 0);
        assert_eq!(h.rows_ingested, 0);
        assert!(!h.retention_window_active);
        assert_eq!(h.eviction_count, 0);
        assert_eq!(h.memory_bytes, 0);
        assert_eq!(h.retention_window_seconds, 0);
        assert_eq!(h.eviction_count_since_last_tick, 0);
    }

    #[test]
    fn heartbeat_payload_reflects_recorded_rows_ingested() {
        let state = BufferState::new();
        state.record_rows_appended(42);
        let h = heartbeat_payload(&state, 600, 0);
        assert_eq!(h.rows_ingested, 42);
        assert_eq!(h.retention_window_seconds, 600);
    }

    #[test]
    fn heartbeat_payload_reflects_eviction_count() {
        let state = BufferState::new();
        state.record_eviction(7);
        let h = heartbeat_payload(&state, 600, 7);
        assert_eq!(h.eviction_count, 7);
        assert_eq!(h.eviction_count_since_last_tick, 7);
    }

    #[test]
    fn heartbeat_payload_reflects_retention_active_flip() {
        let state = BufferState::new();
        let h_pre = heartbeat_payload(&state, 600, 0);
        assert!(!h_pre.retention_window_active);
        state.mark_retention_active();
        let h_post = heartbeat_payload(&state, 600, 0);
        assert!(h_post.retention_window_active);
    }

    #[test]
    fn heartbeat_payload_reflects_memory_bytes() {
        let state = BufferState::new();
        state.set_memory_bytes(2048);
        let h = heartbeat_payload(&state, 600, 0);
        assert_eq!(h.memory_bytes, 2048);
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
