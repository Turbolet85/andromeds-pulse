use chrono::{DateTime, Utc};
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

// ===== Introspection contracts (chunk #27) =====
//
// `app_info`/`health`/`ready`/`get_settings`/`update_settings` per arch §Standard
// Contracts. JSON envelopes carry only token-free semantic fields per
// design-system §Self-Validation Protocol "Token Test"; values map to design
// tokens at render-time, not at wire-time.

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub rust_version: String,
    pub tauri_version: String,
    pub features: Vec<String>,
    pub build_profile: String,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    // Locked default per design-system §Decisions Log 2026-05-02 "Color World locked".
    #[default]
    Dark,
    Light,
    Auto,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WidgetPosition {
    TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
}

// Token-budget preset per arch §Established Decisions [Snapshot Curation Default]:
// Balanced (25k tokens) is the default; Conservative (10k) and Detailed (50k)
// flank it. Budget is derived from preset enum at snapshot.generate time;
// no separate u32 field — Custom budget can be added when a concrete need
// emerges (likely epoch 6 chunks #39-#43 once snapshot.generate lands).
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotPreset {
    Conservative,
    #[default]
    Balanced,
    Detailed,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotFormat {
    #[default]
    Markdown,
    Json,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Settings {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub widget_position: WidgetPosition,
    #[serde(default = "default_retention_seconds")]
    pub retention_seconds: u64,
    #[serde(default)]
    pub mcp_server_enabled: bool,
    #[serde(default = "default_notifications_enabled")]
    pub notifications_enabled: bool,
    #[serde(default = "default_always_on_top")]
    pub always_on_top: bool,
    #[serde(default)]
    pub snapshot_preset: SnapshotPreset,
    #[serde(default)]
    pub snapshot_format: SnapshotFormat,
}

fn default_retention_seconds() -> u64 {
    600
}

fn default_notifications_enabled() -> bool {
    true
}

// Mirrors tauri.conf.json compact-widget `alwaysOnTop: true` boot-default
// per chunk #2 scaffold; runtime apply_widget_settings honors persisted
// override in pulse-app/src/window.rs after window show.
fn default_always_on_top() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            widget_position: WidgetPosition::default(),
            retention_seconds: default_retention_seconds(),
            mcp_server_enabled: false,
            notifications_enabled: default_notifications_enabled(),
            always_on_top: default_always_on_top(),
            snapshot_preset: SnapshotPreset::default(),
            snapshot_format: SnapshotFormat::default(),
        }
    }
}

// retention_seconds bounds match pulse-app/src/main.rs::resolve_retention_seconds
// (60..=86400) per arch §Inherited Defaults retention range + security plan
// §Input Validation row "Configuration values".
pub const RETENTION_SECONDS_MIN: u64 = 60;
pub const RETENTION_SECONDS_MAX: u64 = 86_400;

impl Settings {
    pub fn validate(&self) -> Result<(), AppError> {
        if !(RETENTION_SECONDS_MIN..=RETENTION_SECONDS_MAX).contains(&self.retention_seconds) {
            return Err(AppError::Validation {
                field: "retention_seconds".to_string(),
                reason: "out of range".to_string(),
            });
        }
        Ok(())
    }

    // Boot-time settings load: silent fallback to default on file-not-found
    // OR parse error so boot does not abort. IPC path
    // (IntrospectionApiImpl::get_settings) surfaces parse errors as
    // Storage AppError to the caller; this path is for the binary's own
    // startup sequence where there is no caller to surface to.
    pub fn load_from_data_dir(data_dir: &std::path::Path) -> Self {
        let path = data_dir.join("config.toml");
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|content| toml::from_str::<Settings>(&content).ok())
            .unwrap_or_default()
    }
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyChecks {
    pub duckdb_connection: String,
    pub ingest_mpsc_capacity_pct: u32,
    pub broadcast_subscribers: u32,
    pub plugins_loaded: u32,
    pub mcp_server_enabled: bool,
}

#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyEnvelope {
    pub ready: bool,
    pub checked_at: DateTime<Utc>,
    pub checks: ReadyChecks,
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

    // ===== Chunk #27 introspection contracts =====

    #[test]
    fn settings_default_theme_is_dark() {
        let s = Settings::default();
        assert_eq!(s.theme, Theme::Dark);
    }

    #[test]
    fn settings_default_widget_position_is_top_right() {
        let s = Settings::default();
        assert_eq!(s.widget_position, WidgetPosition::TopRight);
    }

    #[test]
    fn settings_default_retention_seconds_is_600() {
        let s = Settings::default();
        assert_eq!(s.retention_seconds, 600);
    }

    #[test]
    fn settings_default_notifications_enabled_true_mcp_disabled() {
        let s = Settings::default();
        assert!(s.notifications_enabled);
        assert!(!s.mcp_server_enabled);
    }

    #[test]
    fn settings_round_trips_through_serde() {
        let s = Settings {
            theme: Theme::Auto,
            widget_position: WidgetPosition::BottomLeft,
            retention_seconds: 300,
            mcp_server_enabled: true,
            notifications_enabled: false,
            always_on_top: false,
            snapshot_preset: SnapshotPreset::Detailed,
            snapshot_format: SnapshotFormat::Json,
        };
        let json = serde_json::to_string(&s).expect("serializes");
        let parsed: Settings = serde_json::from_str(&json).expect("parses back");
        assert_eq!(parsed, s);
    }

    #[test]
    fn settings_default_snapshot_preset_is_balanced() {
        let s = Settings::default();
        assert_eq!(s.snapshot_preset, SnapshotPreset::Balanced);
    }

    #[test]
    fn settings_default_snapshot_format_is_markdown() {
        let s = Settings::default();
        assert_eq!(s.snapshot_format, SnapshotFormat::Markdown);
    }

    #[test]
    fn snapshot_preset_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&SnapshotPreset::Conservative).unwrap(),
            "\"conservative\""
        );
        assert_eq!(
            serde_json::to_string(&SnapshotPreset::Balanced).unwrap(),
            "\"balanced\""
        );
        assert_eq!(
            serde_json::to_string(&SnapshotPreset::Detailed).unwrap(),
            "\"detailed\""
        );
    }

    #[test]
    fn snapshot_format_serializes_lowercase() {
        assert_eq!(
            serde_json::to_string(&SnapshotFormat::Markdown).unwrap(),
            "\"markdown\""
        );
        assert_eq!(
            serde_json::to_string(&SnapshotFormat::Json).unwrap(),
            "\"json\""
        );
    }

    #[test]
    fn settings_partial_deserialize_uses_defaults_for_missing_snapshot_fields() {
        let json = r#"{"theme":"light"}"#;
        let s: Settings = serde_json::from_str(json).expect("parses");
        assert_eq!(s.snapshot_preset, SnapshotPreset::Balanced);
        assert_eq!(s.snapshot_format, SnapshotFormat::Markdown);
    }

    #[test]
    fn settings_default_always_on_top_is_true() {
        let s = Settings::default();
        assert!(s.always_on_top);
    }

    #[test]
    fn theme_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&Theme::Dark).unwrap(), "\"dark\"");
        assert_eq!(serde_json::to_string(&Theme::Light).unwrap(), "\"light\"");
        assert_eq!(serde_json::to_string(&Theme::Auto).unwrap(), "\"auto\"");
    }

    #[test]
    fn widget_position_serializes_kebab_case() {
        assert_eq!(
            serde_json::to_string(&WidgetPosition::TopLeft).unwrap(),
            "\"top-left\""
        );
        assert_eq!(
            serde_json::to_string(&WidgetPosition::BottomRight).unwrap(),
            "\"bottom-right\""
        );
    }

    #[test]
    fn settings_validate_accepts_in_range_retention() {
        let s = Settings {
            retention_seconds: 600,
            ..Settings::default()
        };
        assert!(s.validate().is_ok());
    }

    #[test]
    fn settings_validate_rejects_below_min_retention() {
        let s = Settings {
            retention_seconds: 30,
            ..Settings::default()
        };
        match s.validate() {
            Err(AppError::Validation { field, reason }) => {
                assert_eq!(field, "retention_seconds");
                assert_eq!(reason, "out of range");
            }
            other => panic!("expected Validation error, got {other:?}"),
        }
    }

    #[test]
    fn settings_validate_rejects_above_max_retention() {
        let s = Settings {
            retention_seconds: 1_000_000,
            ..Settings::default()
        };
        match s.validate() {
            Err(AppError::Validation { field, .. }) => {
                assert_eq!(field, "retention_seconds");
            }
            other => panic!("expected Validation error, got {other:?}"),
        }
    }

    #[test]
    fn settings_partial_deserialize_uses_defaults_for_missing_fields() {
        let json = r#"{"theme":"light"}"#;
        let s: Settings = serde_json::from_str(json).expect("parses");
        assert_eq!(s.theme, Theme::Light);
        assert_eq!(s.widget_position, WidgetPosition::TopRight);
        assert_eq!(s.retention_seconds, 600);
        assert!(s.notifications_enabled);
        assert!(s.always_on_top);
    }

    #[test]
    fn app_info_serializes_with_required_fields() {
        let info = AppInfo {
            name: "andromeda-pulse".to_string(),
            version: "0.1.0".to_string(),
            rust_version: "1.85".to_string(),
            tauri_version: "2.11".to_string(),
            features: vec!["mcp-server".to_string()],
            build_profile: "release".to_string(),
        };
        let v: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&info).unwrap()).unwrap();
        for key in [
            "name",
            "version",
            "rust_version",
            "tauri_version",
            "features",
            "build_profile",
        ] {
            assert!(v.get(key).is_some(), "AppInfo JSON missing key {key}");
        }
        assert_eq!(v["name"], "andromeda-pulse");
    }

    #[test]
    fn ready_envelope_serializes_with_required_fields() {
        let now = Utc::now();
        let env = ReadyEnvelope {
            ready: true,
            checked_at: now,
            checks: ReadyChecks {
                duckdb_connection: "ok".to_string(),
                ingest_mpsc_capacity_pct: 5,
                broadcast_subscribers: 2,
                plugins_loaded: 0,
                mcp_server_enabled: false,
            },
        };
        let v: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&env).unwrap()).unwrap();
        assert_eq!(v["ready"], true);
        for key in [
            "duckdb_connection",
            "ingest_mpsc_capacity_pct",
            "broadcast_subscribers",
            "plugins_loaded",
            "mcp_server_enabled",
        ] {
            assert!(
                v["checks"].get(key).is_some(),
                "ReadyEnvelope.checks JSON missing key {key}"
            );
        }
    }
}
