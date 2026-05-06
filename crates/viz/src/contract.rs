use thiserror::Error;

use crate::state::VizState;

#[derive(Debug, Error)]
pub enum Error {
    #[error("viz query failed: {reason}")]
    QueryFailed { reason: String },
    #[error("viz invalid argument `{field}`: {reason}")]
    InvalidArgument { field: String, reason: String },
    #[error("viz connection lost")]
    ConnectionLost,
    #[error("viz row decode failed: {reason}")]
    Decode { reason: String },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VizHeartbeat {
    pub query_latency_ms: f64,
    pub subscribers_active: u32,
    pub query_count: u64,
}

pub fn heartbeat_payload(state: &VizState) -> VizHeartbeat {
    let snap = state.snapshot();
    VizHeartbeat {
        query_latency_ms: snap.query_latency_ms_avg,
        subscribers_active: snap.subscribers_active,
        query_count: snap.query_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_default_state_is_all_zero() {
        let state = VizState::new();
        let h = heartbeat_payload(&state);
        assert_eq!(h.query_latency_ms, 0.0);
        assert_eq!(h.subscribers_active, 0);
        assert_eq!(h.query_count, 0);
    }

    #[test]
    fn heartbeat_payload_reflects_recorded_query_latency_avg() {
        let state = VizState::new();
        state.record_query_latency_ms(50);
        state.record_query_latency_ms(150);
        let h = heartbeat_payload(&state);
        assert_eq!(h.query_count, 2);
        assert_eq!(h.query_latency_ms, 100.0);
    }

    #[test]
    fn heartbeat_payload_reflects_subscribers_active_count() {
        let state = VizState::new();
        state.inc_subscribers();
        state.inc_subscribers();
        let h = heartbeat_payload(&state);
        assert_eq!(h.subscribers_active, 2);
    }

    #[test]
    fn error_query_failed_displays_reason() {
        let e = Error::QueryFailed {
            reason: "prepare error".to_string(),
        };
        assert!(format!("{e}").contains("prepare error"));
        assert!(format!("{e}").contains("query failed"));
    }

    #[test]
    fn error_invalid_argument_carries_field_and_reason() {
        let e = Error::InvalidArgument {
            field: "limit".into(),
            reason: "above max".into(),
        };
        let s = format!("{e}");
        assert!(s.contains("limit"));
        assert!(s.contains("above max"));
    }

    #[test]
    fn error_connection_lost_displays_constant() {
        let e = Error::ConnectionLost;
        assert!(format!("{e}").contains("connection lost"));
    }

    #[test]
    fn error_decode_displays_reason() {
        let e = Error::Decode {
            reason: "bad blob length".into(),
        };
        assert!(format!("{e}").contains("bad blob length"));
        assert!(format!("{e}").contains("decode failed"));
    }
}
