use thiserror::Error;

use crate::channel::IngestSender;
use crate::grpc::DEFAULT_GRPC_PORT;
use crate::http::DEFAULT_HTTP_PORT;
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

/// Validated OTLP receiver port. Wraps a `u16` that is either spec-fixed
/// (`:4317` / `:4318`) or non-privileged (≥ 1024). Per arch §Conventions
/// Configuration units + §Established Decisions Validation Library:
/// ports MUST go through `TryFrom<u16>` validating non-privileged-or-
/// explicitly-allowed ranges (no validation library).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OtlpPort(u16);

impl OtlpPort {
    pub const fn value(&self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for OtlpPort {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == DEFAULT_GRPC_PORT || value == DEFAULT_HTTP_PORT {
            return Ok(OtlpPort(value));
        }
        if value < 1024 {
            return Err(Error::InvalidPort {
                value: value.to_string(),
            });
        }
        Ok(OtlpPort(value))
    }
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
    use rstest::rstest;

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(80)]
    #[case(443)]
    #[case(1023)]
    fn otlp_port_rejects_privileged_or_zero(#[case] port: u16) {
        let result = OtlpPort::try_from(port);
        assert!(matches!(result, Err(Error::InvalidPort { .. })));
        if let Err(Error::InvalidPort { value }) = result {
            assert_eq!(value, port.to_string());
        }
    }

    #[rstest]
    #[case(1024)]
    #[case(4317)]
    #[case(4318)]
    #[case(9000)]
    #[case(65535)]
    fn otlp_port_accepts_spec_defaults_and_non_privileged(#[case] port: u16) {
        let result =
            OtlpPort::try_from(port).expect("non-privileged or spec-default port must pass");
        assert_eq!(result.value(), port);
    }

    #[test]
    fn invalid_port_error_message_carries_rejected_value() {
        let result = OtlpPort::try_from(80_u16);
        let err = result.expect_err("port 80 must be rejected");
        let s = format!("{err}");
        assert!(s.contains("80"));
    }

    #[test]
    fn otlp_port_value_round_trips() {
        let p = OtlpPort::try_from(4317_u16).expect("4317 valid");
        assert_eq!(p.value(), 4317);
    }

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
