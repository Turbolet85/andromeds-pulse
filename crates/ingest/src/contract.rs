use thiserror::Error;

use crate::state::IngestState;

#[derive(Debug, Error)]
pub enum Error {
    #[error("OTLP receiver bind failed: {reason}")]
    BindFailed { reason: String },
    #[error("OTLP gRPC server stopped: {reason}")]
    ServeFailed { reason: String },
    #[error("invalid OTLP gRPC port value: {value}")]
    InvalidPort { value: String },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IngestHeartbeat {
    pub span_count: u64,
    pub buffer_capacity_pct: f64,
    pub broadcast_subscribers: u32,
}

pub fn heartbeat_payload(state: &IngestState) -> IngestHeartbeat {
    let snap = state.snapshot();
    IngestHeartbeat {
        span_count: snap.span_count,
        // Buffer capacity tracking lands in chunk #18 (mpsc backpressure observation).
        // Receiver-only chunk #16 reports null-equivalent 0.0 placeholder per
        // obs-plan §3 Heartbeat ticks "placeholder is acceptable, the tick MUST still fire".
        buffer_capacity_pct: 0.0,
        broadcast_subscribers: snap.broadcast_subscribers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_reflects_recorded_spans() {
        let state = IngestState::new();
        state.record_spans(42);
        let h = heartbeat_payload(&state);
        assert_eq!(h.span_count, 42);
        assert_eq!(h.buffer_capacity_pct, 0.0);
        assert_eq!(h.broadcast_subscribers, 0);
    }

    #[test]
    fn heartbeat_payload_default_state_is_all_zero() {
        let state = IngestState::new();
        let h = heartbeat_payload(&state);
        assert_eq!(h.span_count, 0);
        assert_eq!(h.broadcast_subscribers, 0);
    }
}
