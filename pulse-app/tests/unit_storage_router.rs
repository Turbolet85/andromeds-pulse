// Migrated 2026-08-30 from `pulse-app/src/storage_router.rs::tests` — that
// crate sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS. The three inspect/path
// expectations were fixed-as-stale on first real execution: they pinned the
// retired 6-table / schema-v1 corpus, while `Corpus::inspect_summary` derives
// counts + version from the LIVE corpus (SCHEMA_VERSION 2, five tables —
// `baseline_state` DROPPED at chunk 2026-08-30-diagnostics-un-muting-
// harness-truth-sweep).

use std::sync::Arc;

use corpus::contract::{
    Corpus, CorpusReader, CorpusWriter, Error as CorpusError, FakeKeychainBackend, KeychainBackend,
};
use ui_bridge::contract::AppError;

use pulse_app::storage_router::{StorageApi, StorageApiImpl, corpus_error_to_app_error};

fn make_impl() -> StorageApiImpl {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    let corpus_arc = Arc::new(corpus);
    let reader: Arc<dyn CorpusReader> = Arc::clone(&corpus_arc) as Arc<dyn CorpusReader>;
    let writer: Arc<dyn CorpusWriter> = Arc::clone(&corpus_arc) as Arc<dyn CorpusWriter>;
    StorageApiImpl::new(reader, writer)
}

#[tokio::test]
async fn inspect_returns_zero_records_for_fresh_corpus() {
    let api = make_impl();
    let payload = api.inspect().await.expect("inspect ok");
    assert_eq!(payload.schema_version, 2);
    assert!(payload.record_counts.iter().all(|r| r.count == 0));
    assert_eq!(payload.record_counts.len(), 5);
}

#[tokio::test]
async fn inspect_payload_includes_all_five_tables() {
    let api = make_impl();
    let payload = api.inspect().await.expect("inspect ok");
    let tables: Vec<&str> = payload
        .record_counts
        .iter()
        .map(|r| r.table.as_str())
        .collect();
    for expected in [
        "service_registry",
        "pipeline_metrics",
        "incidents",
        "incident_events",
        "digest_archive",
    ] {
        assert!(tables.contains(&expected), "missing table {expected}");
    }
    assert!(
        !tables.contains(&"baseline_state"),
        "baseline_state was DROPPED at SCHEMA_VERSION 2 and must not reappear"
    );
}

#[tokio::test]
async fn path_returns_memory_marker_for_in_memory_corpus() {
    let api = make_impl();
    let payload = api.path().await.expect("path ok");
    assert_eq!(payload.path, ":memory:");
    assert_eq!(payload.size_bytes, 0);
    assert_eq!(payload.schema_version, 2);
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
