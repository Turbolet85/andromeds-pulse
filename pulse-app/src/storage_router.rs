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
use std::time::Instant;

use corpus::contract::{CorpusReader, CorpusWriter, Error as CorpusError, InspectionMetadata};
use serde::{Deserialize, Serialize};
use ui_bridge::contract::AppError;

use crate::training_export::{
    ExportPreviewPayload, assemble_records, build_preview_summary, count_redactions,
    resolve_default_target, serialize_jsonl, validate_export_target,
};

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
    // Anonymized JSONL corpus export for community training (chunk #95,
    // capability P-046). `confirm = false` returns the preview summary
    // WITHOUT writing a file; `confirm = true` writes the JSONL to
    // `target_path` (or the default `~/Downloads/...` when None) + returns
    // the summary with `written = true`. No auto-submission — the user
    // manually shares the file. Regular `//` comment (not `///`) so the
    // taurpc::procedures macro does not reject a `#[doc]` attribute on the
    // trait method (per CLAUDE.md testing 2026-05-25).
    async fn export_for_training(
        target_path: Option<String>,
        confirm: bool,
    ) -> Result<ExportPreviewPayload, AppError>;
}

#[derive(Clone)]
pub struct StorageApiImpl {
    reader: Arc<dyn CorpusReader>,
    // CorpusWriter view from the SAME underlying Arc<Corpus> as `reader`
    // (N-trait-from-single-Arc per session-learnings 2026-05-19). The
    // incident read used by `export_for_training` (`load_all_incidents`)
    // lives on CorpusWriter alongside the other incident reads.
    writer: Arc<dyn CorpusWriter>,
}

impl StorageApiImpl {
    pub fn new(reader: Arc<dyn CorpusReader>, writer: Arc<dyn CorpusWriter>) -> Self {
        Self { reader, writer }
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

    #[tracing::instrument(skip_all, fields(
        record_count = tracing::field::Empty,
        redacted_field_count = tracing::field::Empty,
        target_path_basename = tracing::field::Empty,
        written = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ))]
    async fn export_for_training(
        self,
        target_path: Option<String>,
        confirm: bool,
    ) -> Result<ExportPreviewPayload, AppError> {
        let start = Instant::now();
        let rows = self
            .writer
            .load_all_incidents()
            .map_err(corpus_error_to_app_error)?;
        let records = assemble_records(&rows)?;
        let redacted_field_count = count_redactions(&records);

        let (summary, basename) = if confirm {
            let target = match target_path {
                Some(ref s) if !s.trim().is_empty() => std::path::PathBuf::from(s),
                _ => resolve_default_target(now_unix_nanos())?,
            };
            let validated = validate_export_target(&target).inspect_err(|_| {
                tracing::warn!(
                    target: "storage.export_for_training.request",
                    error_category = "target_validation_rejected",
                    written = false,
                    "storage.export_for_training target rejected",
                );
            })?;
            let jsonl = serialize_jsonl(&records)?;
            std::fs::write(&validated, jsonl).map_err(|_| {
                tracing::warn!(
                    target: "storage.export_for_training.request",
                    error_category = "write_failed",
                    written = false,
                    "storage.export_for_training write failed",
                );
                AppError::Storage {
                    message: "export file write failed".to_string(),
                }
            })?;
            let basename = validated
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("pulse-corpus-export.jsonl")
                .to_string();
            (
                build_preview_summary(&records, true, Some(basename.clone())),
                basename,
            )
        } else {
            (build_preview_summary(&records, false, None), String::new())
        };

        let duration_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let span = tracing::Span::current();
        span.record("record_count", summary.total_records);
        span.record("redacted_field_count", redacted_field_count);
        span.record("target_path_basename", basename.as_str());
        span.record("written", summary.written);
        span.record("duration_ms", duration_ms);
        tracing::info!(
            target: "storage.export_for_training.request",
            record_count = summary.total_records,
            redacted_field_count = redacted_field_count,
            target_path_basename = basename.as_str(),
            written = summary.written,
            duration_ms = duration_ms,
            "storage.export_for_training returned",
        );

        Ok(summary)
    }
}

fn now_unix_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
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

// Tests migrated to `pulse-app/tests/unit_storage_router.rs` — a src-level `mod tests`
// compiles but never runs under `[lib] test = false` (2026-05-20 precedent).
