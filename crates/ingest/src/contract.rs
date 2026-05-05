use thiserror::Error;

use crate::channel::IngestSender;
use crate::state::IngestState;

#[derive(Debug, Error)]
pub enum Error {
    #[error("OTLP receiver bind failed: {reason}")]
    BindFailed { reason: String },
    #[error("OTLP gRPC server stopped: {reason}")]
    ServeFailed { reason: String },
    #[error("invalid OTLP gRPC port value: {value}")]
    InvalidPort { value: String },
    #[error("OTLP invariant violation: {kind}")]
    InvariantViolation {
        kind: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("ingest channel saturated")]
    ChannelFull,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IngestHeartbeat {
    pub span_count: u64,
    pub buffer_capacity_pct: f64,
    pub broadcast_subscribers: u32,
}

pub fn heartbeat_payload(state: &IngestState, sender: &IngestSender) -> IngestHeartbeat {
    let snap = state.snapshot();
    IngestHeartbeat {
        span_count: snap.span_count,
        buffer_capacity_pct: sender.capacity_pct(),
        broadcast_subscribers: snap.broadcast_subscribers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::build_channel;

    #[test]
    fn heartbeat_payload_reflects_recorded_spans() {
        let state = IngestState::new();
        state.record_spans(42);
        let (sender, _rx) = build_channel();
        let h = heartbeat_payload(&state, &sender);
        assert_eq!(h.span_count, 42);
        assert_eq!(h.buffer_capacity_pct, 0.0);
        assert_eq!(h.broadcast_subscribers, 0);
    }

    #[test]
    fn heartbeat_payload_default_state_is_all_zero() {
        let state = IngestState::new();
        let (sender, _rx) = build_channel();
        let h = heartbeat_payload(&state, &sender);
        assert_eq!(h.span_count, 0);
        assert_eq!(h.broadcast_subscribers, 0);
    }

    #[test]
    fn heartbeat_payload_reflects_live_channel_capacity() {
        let state = IngestState::new();
        let (sender, mut rx) = crate::channel::build_channel_with_capacity(4);
        // Fill 2 of 4 slots; receiver hasn't drained.
        sender
            .try_send(crate::channel::Batch::Spans(Vec::new()))
            .expect("send within capacity");
        sender
            .try_send(crate::channel::Batch::Spans(Vec::new()))
            .expect("send within capacity");
        let h = heartbeat_payload(&state, &sender);
        assert!(h.buffer_capacity_pct > 0.0);
        assert!(h.buffer_capacity_pct <= 100.0);
        // Drain to keep rx alive across assertion (would otherwise drop).
        while rx.try_recv().is_ok() {}
    }

    #[test]
    fn invariant_violation_error_carries_kind_and_lengths() {
        let e = Error::InvariantViolation {
            kind: "trace_id_length",
            expected: 16,
            actual: 8,
        };
        let s = format!("{e}");
        assert!(s.contains("trace_id_length"));
    }

    #[test]
    fn channel_full_error_displays_constant_string() {
        let e = Error::ChannelFull;
        assert_eq!(format!("{e}"), "ingest channel saturated");
    }
}
