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
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BufferHeartbeat {
    pub rows_ingested: u64,
    pub retention_window_active: bool,
    pub eviction_count: u64,
}

pub fn heartbeat_payload(state: &BufferState) -> BufferHeartbeat {
    let snap = state.snapshot();
    BufferHeartbeat {
        rows_ingested: snap.rows_ingested,
        // Set true once chunk #21 wires the retention DELETE task; chunk #20
        // creates the cutoff column shape but does not run the retention loop.
        retention_window_active: false,
        eviction_count: snap.eviction_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_default_state_is_all_zero() {
        let state = BufferState::new();
        let h = heartbeat_payload(&state);
        assert_eq!(h.rows_ingested, 0);
        assert!(!h.retention_window_active);
        assert_eq!(h.eviction_count, 0);
    }

    #[test]
    fn heartbeat_payload_reflects_recorded_rows_ingested() {
        let state = BufferState::new();
        state.record_rows_appended(42);
        let h = heartbeat_payload(&state);
        assert_eq!(h.rows_ingested, 42);
    }

    #[test]
    fn heartbeat_payload_reflects_eviction_count() {
        let state = BufferState::new();
        state.record_eviction(7);
        let h = heartbeat_payload(&state);
        assert_eq!(h.eviction_count, 7);
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
}
