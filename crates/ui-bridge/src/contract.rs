use serde::{Deserialize, Serialize};
use thiserror::Error;

use buffer::Error as BufferError;
use ingest::contract::Error as IngestError;

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AppError {
    #[error("validation failed for `{field}`: {reason}")]
    Validation { field: String, reason: String },

    #[error("not found: {resource}")]
    NotFound { resource: String },

    #[error("internal error: {message}")]
    Internal { message: String },

    #[error("plugin `{plugin_id}` error: {message}")]
    Plugin { plugin_id: String, message: String },

    #[error("storage error: {message}")]
    Storage { message: String },

    #[error("ingest error: {message}")]
    Ingest { message: String },
}

impl AppError {
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
}

impl From<IngestError> for AppError {
    fn from(e: IngestError) -> Self {
        let message = match e {
            IngestError::BindFailed { .. } => "OTLP receiver bind failed",
            IngestError::ServeFailed { .. } => "OTLP server stopped",
            IngestError::InvalidPort { .. } => "invalid OTLP port configuration",
            IngestError::InvariantViolation { .. } => "invalid OTLP payload",
            IngestError::ChannelFull => "ingest channel saturated",
        };
        AppError::Ingest {
            message: message.to_string(),
        }
    }
}

impl From<BufferError> for AppError {
    fn from(e: BufferError) -> Self {
        let message = match e {
            BufferError::Init { .. } => "buffer init failed",
            BufferError::SchemaCreate { .. } => "buffer schema create failed",
            BufferError::Append { .. } => "buffer Arrow append failed",
            BufferError::ConnectionLost => "buffer connection lost",
            BufferError::InvalidBatch { .. } => "buffer received invalid batch",
        };
        AppError::Storage {
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_error_ingest_serializes_to_stable_sanitized_shape() {
        let e = AppError::Ingest {
            message: "invalid OTLP payload".to_string(),
        };
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "ingest");
        assert_eq!(v["message"], "invalid OTLP payload");
    }

    #[test]
    fn from_invariant_violation_collapses_to_constant_message() {
        let e = AppError::from(IngestError::InvariantViolation {
            kind: "trace_id_length",
            expected: 16,
            actual: 8,
        });
        match e {
            AppError::Ingest { message } => assert_eq!(message, "invalid OTLP payload"),
            other => panic!("expected AppError::Ingest, got {other:?}"),
        }
    }

    #[test]
    fn from_channel_full_collapses_to_constant_message() {
        let e = AppError::from(IngestError::ChannelFull);
        match e {
            AppError::Ingest { message } => assert_eq!(message, "ingest channel saturated"),
            other => panic!("expected AppError::Ingest, got {other:?}"),
        }
    }

    #[test]
    fn from_bind_failed_collapses_to_constant_message_no_struct_name_leak() {
        let e = AppError::from(IngestError::BindFailed {
            reason: "address in use at /private/var/secret/path 0.14.5".to_string(),
        });
        match e {
            AppError::Ingest { message } => {
                assert_eq!(message, "OTLP receiver bind failed");
                assert!(!message.contains("/private/"));
                assert!(!message.contains("0.14.5"));
            }
            other => panic!("expected AppError::Ingest, got {other:?}"),
        }
    }

    #[test]
    fn from_serve_failed_collapses_to_constant_message() {
        let e = AppError::from(IngestError::ServeFailed {
            reason: "listener closed".to_string(),
        });
        match e {
            AppError::Ingest { message } => assert_eq!(message, "OTLP server stopped"),
            other => panic!("expected AppError::Ingest, got {other:?}"),
        }
    }

    #[test]
    fn from_invalid_port_collapses_to_constant_message() {
        let e = AppError::from(IngestError::InvalidPort {
            value: "999999".to_string(),
        });
        match e {
            AppError::Ingest { message } => assert_eq!(message, "invalid OTLP port configuration"),
            other => panic!("expected AppError::Ingest, got {other:?}"),
        }
    }

    #[test]
    fn from_buffer_init_collapses_to_constant_message_no_struct_name_leak() {
        let e = AppError::from(BufferError::Init {
            reason: "open_in_memory error at /tmp/secret 1.10502".to_string(),
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "buffer init failed");
                assert!(!message.contains("/tmp/"));
                assert!(!message.contains("1.10502"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_buffer_schema_create_collapses_to_constant_message() {
        let e = AppError::from(BufferError::SchemaCreate {
            reason: "DDL parse failed at line 3".to_string(),
        });
        match e {
            AppError::Storage { message } => assert_eq!(message, "buffer schema create failed"),
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_buffer_append_collapses_to_constant_message() {
        let e = AppError::from(BufferError::Append {
            reason: "duckdb internal error 0xdeadbeef".to_string(),
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "buffer Arrow append failed");
                assert!(!message.contains("duckdb"));
                assert!(!message.contains("0xdeadbeef"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_buffer_connection_lost_collapses_to_constant_message() {
        let e = AppError::from(BufferError::ConnectionLost);
        match e {
            AppError::Storage { message } => assert_eq!(message, "buffer connection lost"),
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_buffer_invalid_batch_collapses_to_constant_message() {
        let e = AppError::from(BufferError::InvalidBatch {
            kind: "spans_empty",
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "buffer received invalid batch");
                assert!(!message.contains("spans_empty"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_invariant_message_does_not_leak_kind_or_lengths() {
        let e = AppError::from(IngestError::InvariantViolation {
            kind: "trace_id_length",
            expected: 16,
            actual: 8,
        });
        let s = serde_json::to_string(&e).expect("serializes");
        assert!(
            !s.contains("trace_id_length"),
            "wire message must not leak invariant kind: {s}"
        );
        assert!(
            !s.contains('8'),
            "wire message must not leak actual length: {s}"
        );
    }
}
