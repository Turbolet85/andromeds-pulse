//! Storage TauRPC router — chunk #68.
//!
//! `storage.inspect()` (read-only corpus metadata; no arguments —
//! read-only-by-design per capability P-051) + `storage.path()`
//! (structured envelope with corpus path + size + schema version).
//!
//! Mirrors `services_router.rs` shape — library crate (`crates/corpus`)
//! stays Tauri-free; the Tauri-aware router lives here at the binary
//! boundary so taurpc + specta deps don't leak into the workspace crate
//! per arch §Cross-cutting Patterns Module dependency direction. The
//! `From<corpus::Error> for AppError` impl ALSO lives here (binary
//! boundary; avoids `ui-bridge → corpus` reverse dep edge).

use std::sync::Arc;

use corpus::contract::{CorpusReader, Error as CorpusError, InspectionMetadata};
use serde::{Deserialize, Serialize};
use ui_bridge::contract::AppError;

/// Per-table record counts + on-disk byte size + schema version. Cross-bridge
/// envelope for the `storage.inspect` resolver.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct StorageInspectPayload {
    pub record_counts: Vec<TableRecordCount>,
    pub total_bytes_on_disk: u64,
    pub schema_version: u32,
}

/// Single table → record count entry. Vec<TableRecordCount> in the
/// payload (rather than a map) keeps the TS binding shape stable
/// across specta releases.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct TableRecordCount {
    pub table: String,
    pub count: u64,
}

/// Corpus DB filesystem path + size + schema version envelope. Path is
/// the canonicalized string surfaced across the bridge; the matching
/// `tracing` event in `storage.path.request` emits basename only per
/// obs-plan §1 Vector 6.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct StoragePathPayload {
    pub path: String,
    pub size_bytes: u64,
    pub schema_version: u32,
}

#[taurpc::procedures(path = "storage")]
pub trait StorageApi {
    async fn inspect() -> Result<StorageInspectPayload, AppError>;
    async fn path() -> Result<StoragePathPayload, AppError>;
}

#[derive(Clone)]
pub struct StorageApiImpl {
    reader: Arc<dyn CorpusReader>,
}

impl StorageApiImpl {
    pub fn new(reader: Arc<dyn CorpusReader>) -> Self {
        Self { reader }
    }
}

#[taurpc::resolvers]
impl StorageApi for StorageApiImpl {
    #[tracing::instrument(skip_all, fields(
        record_count = tracing::field::Empty,
        bytes_on_disk = tracing::field::Empty,
        schema_version = tracing::field::Empty,
    ))]
    async fn inspect(self) -> Result<StorageInspectPayload, AppError> {
        let meta = self.reader.inspect().map_err(corpus_error_to_app_error)?;
        let payload = inspect_metadata_to_payload(&meta);
        let total_records: u64 = payload.record_counts.iter().map(|r| r.count).sum();

        let span = tracing::Span::current();
        span.record("record_count", total_records);
        span.record("bytes_on_disk", payload.total_bytes_on_disk);
        span.record("schema_version", payload.schema_version);

        tracing::info!(
            target: "storage.inspect.request",
            record_count = total_records,
            bytes_on_disk = payload.total_bytes_on_disk,
            schema_version = payload.schema_version,
            "storage.inspect returned",
        );

        Ok(payload)
    }

    #[tracing::instrument(skip_all, fields(
        corpus_path_basename = tracing::field::Empty,
        schema_version = tracing::field::Empty,
    ))]
    async fn path(self) -> Result<StoragePathPayload, AppError> {
        let path = self.reader.path();
        let basename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(":memory:")
            .to_string();
        let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let meta = self.reader.inspect().map_err(corpus_error_to_app_error)?;
        let payload = StoragePathPayload {
            path: path.to_string_lossy().to_string(),
            size_bytes,
            schema_version: meta.schema_version,
        };

        let span = tracing::Span::current();
        span.record("corpus_path_basename", basename.as_str());
        span.record("schema_version", payload.schema_version);

        tracing::info!(
            target: "storage.path.request",
            corpus_path_basename = basename.as_str(),
            schema_version = payload.schema_version,
            "storage.path returned",
        );

        Ok(payload)
    }
}

fn inspect_metadata_to_payload(meta: &InspectionMetadata) -> StorageInspectPayload {
    let record_counts = meta
        .record_counts
        .iter()
        .map(|(table, count)| TableRecordCount {
            table: table.clone(),
            count: *count,
        })
        .collect();
    StorageInspectPayload {
        record_counts,
        total_bytes_on_disk: meta.total_bytes_on_disk,
        schema_version: meta.schema_version,
    }
}

/// Sanitized boundary conversion. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions / schema column names appear in the message
/// surfaced to the webview.
///
/// Free function (not `From` impl) — orphan rules forbid implementing
/// `From<external>` for `external` here; both `CorpusError` and
/// `AppError` are foreign types. Pulse-app owns the conversion at the
/// binary boundary so neither corpus nor ui-bridge takes a dep on the
/// other.
pub fn corpus_error_to_app_error(err: CorpusError) -> AppError {
    let message = match err {
        CorpusError::KeyringUnavailable => "keychain unavailable",
        CorpusError::MigrationFailed => "corpus schema migration failed",
        CorpusError::EncryptionFailed => "corpus encryption failed",
        CorpusError::DecryptionFailed => "corpus decryption failed",
        CorpusError::QueryFailed => "corpus query failed",
        CorpusError::PathTraversal => "corpus path rejected",
        CorpusError::SizeCapExceeded => "corpus file too large",
        CorpusError::SchemaVersionMismatch => "corpus schema version mismatch",
        CorpusError::Io { .. } => "corpus io error",
    };
    AppError::Storage {
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus::contract::{Corpus, FakeKeychainBackend, KeychainBackend};

    fn make_impl() -> StorageApiImpl {
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
        let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
        let reader: Arc<dyn CorpusReader> = Arc::new(corpus);
        StorageApiImpl::new(reader)
    }

    #[tokio::test]
    async fn inspect_returns_zero_records_for_fresh_corpus() {
        let api = make_impl();
        let payload = api.inspect().await.expect("inspect ok");
        assert_eq!(payload.schema_version, 1);
        assert!(payload.record_counts.iter().all(|r| r.count == 0));
        assert_eq!(payload.record_counts.len(), 6);
    }

    #[tokio::test]
    async fn inspect_payload_includes_all_six_tables() {
        let api = make_impl();
        let payload = api.inspect().await.expect("inspect ok");
        let tables: Vec<&str> = payload
            .record_counts
            .iter()
            .map(|r| r.table.as_str())
            .collect();
        for expected in [
            "baseline_state",
            "service_registry",
            "pipeline_metrics",
            "incidents",
            "incident_events",
            "digest_archive",
        ] {
            assert!(tables.contains(&expected), "missing table {expected}");
        }
    }

    #[tokio::test]
    async fn path_returns_memory_marker_for_in_memory_corpus() {
        let api = make_impl();
        let payload = api.path().await.expect("path ok");
        assert_eq!(payload.path, ":memory:");
        assert_eq!(payload.size_bytes, 0);
        assert_eq!(payload.schema_version, 1);
    }

    #[test]
    fn corpus_error_keyring_unavailable_maps_to_app_error_storage() {
        let err: AppError = corpus_error_to_app_error(CorpusError::KeyringUnavailable);
        match err {
            AppError::Storage { message } => {
                assert_eq!(message, "keychain unavailable");
                assert!(!message.contains("/"));
                assert!(!message.contains("rusqlite"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn corpus_error_query_failed_sanitized_message() {
        let err: AppError = corpus_error_to_app_error(CorpusError::QueryFailed);
        match err {
            AppError::Storage { message } => {
                assert_eq!(message, "corpus query failed");
                // No SQL fragments, no library version, no file path.
                assert!(!message.contains("SELECT"));
                assert!(!message.contains("0."));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn corpus_error_schema_version_mismatch_sanitized() {
        let err: AppError = corpus_error_to_app_error(CorpusError::SchemaVersionMismatch);
        match err {
            AppError::Storage { message } => {
                assert_eq!(message, "corpus schema version mismatch");
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }

    #[test]
    fn corpus_error_io_sanitized_no_kind_detail_in_message() {
        let err: AppError = corpus_error_to_app_error(CorpusError::Io {
            kind: std::io::ErrorKind::NotFound,
        });
        match err {
            AppError::Storage { message } => {
                assert_eq!(message, "corpus io error");
                assert!(!message.contains("NotFound"));
            }
            other => panic!("expected AppError::Storage, got {other:?}"),
        }
    }
}
