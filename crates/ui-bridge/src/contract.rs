use serde::{Deserialize, Serialize};
use thiserror::Error;

use buffer::Error as BufferError;
use ingest::contract::Error as IngestError;
use mcp_server::contract::Error as McpServerError;
use plugins::contract::Error as PluginsError;
use snapshot::contract::Error as SnapshotError;
use viz::Error as VizError;
use workspace_detector::contract::Error as WorkspaceDetectorError;

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
        let (message, source_kind) = match e {
            IngestError::BindFailed { .. } => ("OTLP receiver bind failed", "bind_failed"),
            IngestError::ServeFailed { .. } => ("OTLP server stopped", "serve_failed"),
            IngestError::InvalidPort { .. } => ("invalid OTLP port configuration", "invalid_port"),
            IngestError::InvariantViolation { .. } => {
                ("invalid OTLP payload", "invariant_violation")
            }
            IngestError::ChannelFull => ("ingest channel saturated", "channel_full"),
        };
        tracing::warn!(
            target: "ui-bridge.error.ingest",
            error_category = "ingest",
            source_kind = source_kind,
            source_crate = "ingest",
            "{}",
            message
        );
        AppError::Ingest {
            message: message.to_string(),
        }
    }
}

impl From<BufferError> for AppError {
    fn from(e: BufferError) -> Self {
        let (message, source_kind) = match e {
            BufferError::Init { .. } => ("buffer init failed", "init"),
            BufferError::SchemaCreate { .. } => ("buffer schema create failed", "schema_create"),
            BufferError::Append { .. } => ("buffer Arrow append failed", "append"),
            BufferError::ConnectionLost => ("buffer connection lost", "connection_lost"),
            BufferError::InvalidBatch { .. } => ("buffer received invalid batch", "invalid_batch"),
            BufferError::Retention { .. } => ("buffer retention sweep failed", "retention"),
            BufferError::BroadcastEncode { .. } => {
                ("buffer broadcast encode failed", "broadcast_encode")
            }
            BufferError::BroadcastSizeCapExceeded { .. } => (
                "buffer broadcast payload exceeded size cap",
                "broadcast_size_cap_exceeded",
            ),
        };
        tracing::warn!(
            target: "ui-bridge.error.storage",
            error_category = "storage",
            source_kind = source_kind,
            source_crate = "buffer",
            "{}",
            message
        );
        AppError::Storage {
            message: message.to_string(),
        }
    }
}

impl From<VizError> for AppError {
    fn from(e: VizError) -> Self {
        match e {
            VizError::QueryFailed { .. } => {
                tracing::warn!(
                    target: "ui-bridge.error.storage",
                    error_category = "storage",
                    source_kind = "query_failed",
                    source_crate = "viz",
                    "viz query failed"
                );
                AppError::Storage {
                    message: "viz query failed".to_string(),
                }
            }
            VizError::InvalidArgument { field, .. } => {
                tracing::warn!(
                    target: "ui-bridge.error.validation",
                    error_category = "validation",
                    source_kind = "invalid_argument",
                    source_crate = "viz",
                    field = %field,
                    "invalid query argument"
                );
                AppError::Validation {
                    field,
                    reason: "invalid query argument".to_string(),
                }
            }
            VizError::ConnectionLost => {
                tracing::warn!(
                    target: "ui-bridge.error.storage",
                    error_category = "storage",
                    source_kind = "connection_lost",
                    source_crate = "viz",
                    "viz connection lost"
                );
                AppError::Storage {
                    message: "viz connection lost".to_string(),
                }
            }
            VizError::Decode { .. } => {
                tracing::warn!(
                    target: "ui-bridge.error.storage",
                    error_category = "storage",
                    source_kind = "decode",
                    source_crate = "viz",
                    "viz row decode failed"
                );
                AppError::Storage {
                    message: "viz row decode failed".to_string(),
                }
            }
        }
    }
}

impl From<SnapshotError> for AppError {
    fn from(e: SnapshotError) -> Self {
        let message = match e {
            SnapshotError::Placeholder => "snapshot: placeholder error",
        };
        tracing::warn!(
            target: "ui-bridge.error.internal",
            error_category = "internal",
            source_kind = "placeholder",
            source_crate = "snapshot",
            "{}",
            message
        );
        AppError::Internal {
            message: message.to_string(),
        }
    }
}

impl From<PluginsError> for AppError {
    fn from(e: PluginsError) -> Self {
        let message = match e {
            PluginsError::Placeholder => "plugins: placeholder error",
        };
        tracing::warn!(
            target: "ui-bridge.error.internal",
            error_category = "internal",
            source_kind = "placeholder",
            source_crate = "plugins",
            "{}",
            message
        );
        AppError::Internal {
            message: message.to_string(),
        }
    }
}

impl From<WorkspaceDetectorError> for AppError {
    fn from(e: WorkspaceDetectorError) -> Self {
        let message = match e {
            WorkspaceDetectorError::Placeholder => "workspace-detector: placeholder error",
        };
        tracing::warn!(
            target: "ui-bridge.error.internal",
            error_category = "internal",
            source_kind = "placeholder",
            source_crate = "workspace-detector",
            "{}",
            message
        );
        AppError::Internal {
            message: message.to_string(),
        }
    }
}

impl From<McpServerError> for AppError {
    fn from(e: McpServerError) -> Self {
        let message = match e {
            McpServerError::Placeholder => "mcp-server: placeholder error",
        };
        tracing::warn!(
            target: "ui-bridge.error.internal",
            error_category = "internal",
            source_kind = "placeholder",
            source_crate = "mcp-server",
            "{}",
            message
        );
        AppError::Internal {
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::sync::{Arc, Mutex};
    use tracing::span::{Attributes, Id, Record};
    use tracing::{Event, Metadata, Subscriber};

    // In-process subscriber that captures emitted events into a thread-shared
    // Vec for test assertion. Used to verify each `From<E> for AppError` impl
    // emits a `tracing::warn!` event at the conversion boundary per obs-plan
    // §10 module-boundary error logging (Standard+ tier requirement). The
    // alternative `tracing-test` crate would add a workspace dep; this manual
    // Subscriber stays self-contained at ~25 lines and uses only the existing
    // `tracing` dep already required for emission.
    struct CapturingSubscriber {
        events: Arc<Mutex<Vec<(String, tracing::Level)>>>,
    }

    impl Subscriber for CapturingSubscriber {
        fn enabled(&self, _: &Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &Attributes<'_>) -> Id {
            Id::from_u64(1)
        }
        fn record(&self, _: &Id, _: &Record<'_>) {}
        fn record_follows_from(&self, _: &Id, _: &Id) {}
        fn event(&self, event: &Event<'_>) {
            let metadata = event.metadata();
            self.events
                .lock()
                .expect("event lock not poisoned")
                .push((metadata.target().to_string(), *metadata.level()));
        }
        fn enter(&self, _: &Id) {}
        fn exit(&self, _: &Id) {}
    }

    fn capture<F: FnOnce()>(f: F) -> Vec<(String, tracing::Level)> {
        let events: Arc<Mutex<Vec<(String, tracing::Level)>>> = Arc::new(Mutex::new(Vec::new()));
        let subscriber = CapturingSubscriber {
            events: events.clone(),
        };
        tracing::subscriber::with_default(subscriber, f);
        events.lock().expect("event lock not poisoned").clone()
    }

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
    fn from_buffer_broadcast_encode_collapses_to_constant_message_no_reason_leak() {
        let e = AppError::from(BufferError::BroadcastEncode {
            reason: "ArrowError::IpcError at /tmp/secret 0xdeadbeef".to_string(),
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "buffer broadcast encode failed");
                assert!(!message.contains("ArrowError"));
                assert!(!message.contains("/tmp/"));
                assert!(!message.contains("0xdeadbeef"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_buffer_broadcast_size_cap_collapses_to_constant_message_no_byte_count_leak() {
        let e = AppError::from(BufferError::BroadcastSizeCapExceeded {
            payload_bytes: 9_999_999,
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "buffer broadcast payload exceeded size cap");
                assert!(!message.contains("9999999"));
                assert!(!message.contains("9_999_999"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_buffer_retention_collapses_to_constant_message_no_struct_name_leak() {
        let e = AppError::from(BufferError::Retention {
            reason: "execute: internal duckdb 0xdeadbeef at /tmp/secret 1.10502".to_string(),
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "buffer retention sweep failed");
                assert!(!message.contains("0xdeadbeef"));
                assert!(!message.contains("/tmp/"));
                assert!(!message.contains("1.10502"));
                assert!(!message.contains("execute:"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_viz_query_failed_collapses_to_constant_message_no_leak() {
        let e = AppError::from(VizError::QueryFailed {
            reason: "duckdb internal error 0xdeadbeef at /tmp/secret 1.10502".to_string(),
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "viz query failed");
                assert!(!message.contains("duckdb"));
                assert!(!message.contains("0xdeadbeef"));
                assert!(!message.contains("/tmp/"));
                assert!(!message.contains("1.10502"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_viz_invalid_argument_preserves_field_strips_reason_internals() {
        let e = AppError::from(VizError::InvalidArgument {
            field: "limit".to_string(),
            reason: "above max 1000 see /usr/local/lib/duckdb 1.10502".to_string(),
        });
        match e {
            AppError::Validation { field, reason } => {
                assert_eq!(field, "limit");
                assert_eq!(reason, "invalid query argument");
                assert!(!reason.contains("/usr/local"));
                assert!(!reason.contains("1.10502"));
            }
            other => panic!("expected AppError::Validation, got {other:?}"),
        }
    }

    #[test]
    fn from_viz_connection_lost_collapses_to_constant_message() {
        let e = AppError::from(VizError::ConnectionLost);
        match e {
            AppError::Storage { message } => assert_eq!(message, "viz connection lost"),
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_viz_decode_collapses_to_constant_message_no_struct_name_leak() {
        let e = AppError::from(VizError::Decode {
            reason: "row::get failed at /home/user/code/viz.rs:42 0xdeadbeef".to_string(),
        });
        match e {
            AppError::Storage { message } => {
                assert_eq!(message, "viz row decode failed");
                assert!(!message.contains("/home/"));
                assert!(!message.contains(".rs:"));
                assert!(!message.contains("0xdeadbeef"));
                assert!(!message.contains("Error::"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn from_viz_query_failed_serializes_to_stable_sanitized_shape() {
        let e = AppError::from(VizError::QueryFailed {
            reason: "raw duckdb error".to_string(),
        });
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "storage");
        assert_eq!(v["message"], "viz query failed");
        assert!(!s.contains("duckdb"));
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

    // ===== Round-trip tests for all 6 standalone variants =====

    #[test]
    fn app_error_validation_round_trips_through_serde() {
        let e = AppError::Validation {
            field: "port".to_string(),
            reason: "out of range".to_string(),
        };
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "validation");
        assert_eq!(v["field"], "port");
        assert_eq!(v["reason"], "out of range");
    }

    #[test]
    fn app_error_not_found_round_trips_through_serde() {
        let e = AppError::NotFound {
            resource: "trace_id_xxx".to_string(),
        };
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "not_found");
        assert_eq!(v["resource"], "trace_id_xxx");
    }

    #[test]
    fn app_error_internal_round_trips_through_serde() {
        let e = AppError::Internal {
            message: "something internal".to_string(),
        };
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "internal");
        assert_eq!(v["message"], "something internal");
    }

    #[test]
    fn app_error_plugin_round_trips_through_serde() {
        let e = AppError::Plugin {
            plugin_id: "echo".to_string(),
            message: "load failed".to_string(),
        };
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "plugin");
        assert_eq!(v["plugin_id"], "echo");
        assert_eq!(v["message"], "load failed");
    }

    #[test]
    fn app_error_storage_round_trips_through_serde() {
        let e = AppError::Storage {
            message: "disk full".to_string(),
        };
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "storage");
        assert_eq!(v["message"], "disk full");
    }

    #[test]
    fn app_error_ingest_round_trips_through_serde() {
        let e = AppError::Ingest {
            message: "channel full".to_string(),
        };
        let s = serde_json::to_string(&e).expect("serializes");
        let v: serde_json::Value = serde_json::from_str(&s).expect("parses back");
        assert_eq!(v["kind"], "ingest");
        assert_eq!(v["message"], "channel full");
    }

    // ===== AppError::internal(...) helper =====

    #[test]
    fn app_error_internal_helper_constructs_internal_variant() {
        let e = AppError::internal("hello world");
        match e {
            AppError::Internal { message } => assert_eq!(message, "hello world"),
            other => panic!("expected AppError::Internal, got {other:?}"),
        }
    }

    // ===== Placeholder From-impl tests =====

    #[test]
    fn from_snapshot_placeholder_collapses_to_constant_message_no_leak() {
        let e = AppError::from(SnapshotError::Placeholder);
        match e {
            AppError::Internal { message } => {
                assert_eq!(message, "snapshot: placeholder error");
                assert!(!message.contains('/'));
                assert!(!message.contains("::"));
            }
            other => panic!("expected AppError::Internal, got {other:?}"),
        }
    }

    #[test]
    fn from_plugins_placeholder_collapses_to_constant_message_no_leak() {
        let e = AppError::from(PluginsError::Placeholder);
        match e {
            AppError::Internal { message } => {
                assert_eq!(message, "plugins: placeholder error");
                assert!(!message.contains('/'));
                assert!(!message.contains("::"));
            }
            other => panic!("expected AppError::Internal, got {other:?}"),
        }
    }

    #[test]
    fn from_workspace_detector_placeholder_collapses_to_constant_message_no_leak() {
        let e = AppError::from(WorkspaceDetectorError::Placeholder);
        match e {
            AppError::Internal { message } => {
                assert_eq!(message, "workspace-detector: placeholder error");
                assert!(!message.contains('/'));
                assert!(!message.contains("::"));
            }
            other => panic!("expected AppError::Internal, got {other:?}"),
        }
    }

    #[test]
    fn from_mcp_server_placeholder_collapses_to_constant_message_no_leak() {
        let e = AppError::from(McpServerError::Placeholder);
        match e {
            AppError::Internal { message } => {
                assert_eq!(message, "mcp-server: placeholder error");
                assert!(!message.contains('/'));
                assert!(!message.contains("::"));
            }
            other => panic!("expected AppError::Internal, got {other:?}"),
        }
    }

    // ===== Plain-language register tests =====
    // Per a11y-plan §11 phase-1 baseline: AppError variant strings MUST NOT
    // contain Rust panic boilerplate, kernel/stdlib symbols, RUST_BACKTRACE,
    // or module path syntax — those would surface verbatim to screen-reader
    // output if a downstream UI chunk renders them in toasts/banners.

    const PLAIN_LANGUAGE_FORBIDDEN: &[&str] = &[
        "panicked at",
        "RUST_BACKTRACE",
        "core::result",
        "::ErrorKind",
        "alloc::",
        "std::io::Error",
    ];

    #[rstest]
    #[case(IngestError::BindFailed {
        reason: "thread 'main' panicked at /home/user/code.rs:42 RUST_BACKTRACE=1".to_string(),
    })]
    #[case(IngestError::ServeFailed {
        reason: "core::result::Result::Err(::ErrorKind::ConnectionRefused) std::io::Error".to_string(),
    })]
    fn from_ingest_strips_plain_language_anti_patterns(#[case] e: IngestError) {
        let app = AppError::from(e);
        let s = serde_json::to_string(&app).expect("serializes");
        for forbidden in PLAIN_LANGUAGE_FORBIDDEN {
            assert!(
                !s.contains(forbidden),
                "wire JSON must not leak `{forbidden}`: {s}"
            );
        }
    }

    #[rstest]
    #[case(BufferError::Init {
        reason: "thread 'main' panicked at /tmp/duck.rs alloc::vec::Vec".to_string(),
    })]
    #[case(BufferError::Append {
        reason: "core::result::Result::Err RUST_BACKTRACE=full ::ErrorKind".to_string(),
    })]
    fn from_buffer_strips_plain_language_anti_patterns(#[case] e: BufferError) {
        let app = AppError::from(e);
        let s = serde_json::to_string(&app).expect("serializes");
        for forbidden in PLAIN_LANGUAGE_FORBIDDEN {
            assert!(
                !s.contains(forbidden),
                "wire JSON must not leak `{forbidden}`: {s}"
            );
        }
    }

    #[rstest]
    #[case(VizError::QueryFailed {
        reason: "thread 'main' panicked at std::io::Error of kind ::ErrorKind::Other".to_string(),
    })]
    fn from_viz_strips_plain_language_anti_patterns(#[case] e: VizError) {
        let app = AppError::from(e);
        let s = serde_json::to_string(&app).expect("serializes");
        for forbidden in PLAIN_LANGUAGE_FORBIDDEN {
            assert!(
                !s.contains(forbidden),
                "wire JSON must not leak `{forbidden}`: {s}"
            );
        }
    }

    // ===== Tracing-event presence assertions (in-process subscriber) =====
    // Verifies the obs-plan §10 module-boundary error logging requirement:
    // every From<E> for AppError emits a tracing::warn! at the conversion
    // boundary BEFORE returning the lossy variant.

    #[test]
    fn from_ingest_emits_tracing_warn_at_correct_target() {
        let events = capture(|| {
            let _ = AppError::from(IngestError::ChannelFull);
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.ingest"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.ingest; got {events:?}"
        );
    }

    #[test]
    fn from_buffer_emits_tracing_warn_at_correct_target() {
        let events = capture(|| {
            let _ = AppError::from(BufferError::ConnectionLost);
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.storage"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.storage; got {events:?}"
        );
    }

    #[test]
    fn from_viz_invalid_argument_emits_tracing_warn_at_validation_target() {
        let events = capture(|| {
            let _ = AppError::from(VizError::InvalidArgument {
                field: "limit".to_string(),
                reason: "above max".to_string(),
            });
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.validation"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.validation; got {events:?}"
        );
    }

    #[test]
    fn from_viz_query_failed_emits_tracing_warn_at_storage_target() {
        let events = capture(|| {
            let _ = AppError::from(VizError::QueryFailed {
                reason: "duck".to_string(),
            });
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.storage"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.storage; got {events:?}"
        );
    }

    #[test]
    fn from_snapshot_placeholder_emits_tracing_warn_at_internal_target() {
        let events = capture(|| {
            let _ = AppError::from(SnapshotError::Placeholder);
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.internal"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.internal; got {events:?}"
        );
    }

    #[test]
    fn from_plugins_placeholder_emits_tracing_warn_at_internal_target() {
        let events = capture(|| {
            let _ = AppError::from(PluginsError::Placeholder);
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.internal"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.internal; got {events:?}"
        );
    }

    #[test]
    fn from_workspace_detector_placeholder_emits_tracing_warn_at_internal_target() {
        let events = capture(|| {
            let _ = AppError::from(WorkspaceDetectorError::Placeholder);
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.internal"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.internal; got {events:?}"
        );
    }

    #[test]
    fn from_mcp_server_placeholder_emits_tracing_warn_at_internal_target() {
        let events = capture(|| {
            let _ = AppError::from(McpServerError::Placeholder);
        });
        assert!(
            events
                .iter()
                .any(|(target, level)| target == "ui-bridge.error.internal"
                    && *level == tracing::Level::WARN),
            "expected WARN at ui-bridge.error.internal; got {events:?}"
        );
    }
}
