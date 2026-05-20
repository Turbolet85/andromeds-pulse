//! Unit tests for `CorpusStormPersistence` adapter. Lives in `tests/`
//! per session-learnings 2026-05-13 (Windows WebView2 DLL load workaround
//! in `pulse-app/Cargo.toml [lib] test = false`).

use std::sync::Arc;

use corpus::contract::{
    Corpus, CorpusWriter, Error as CorpusError, FakeKeychainBackend, KeychainBackend,
};
use pulse_app::storm_persistence::{
    CorpusStormPersistence, STORM_PERSISTENCE_LAYER, STORM_STATE_METRIC_NAME,
    corpus_error_to_storm_error,
};
use tempfile::TempDir;
use triage::contract::{
    RetryStormDetector, SCHEMA_VERSION, StormError, StormPersistence, StormStateSnapshot,
};

fn make_writer() -> Arc<dyn CorpusWriter> {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    Arc::new(corpus) as Arc<dyn CorpusWriter>
}

#[test]
fn save_then_load_round_trips_snapshot() {
    let writer = make_writer();
    let adapter = CorpusStormPersistence::new(writer);
    let detector = RetryStormDetector::new(60, 30, 5, 10);
    let snapshot = detector.snapshot();
    adapter.save(&snapshot).expect("save");
    let loaded = adapter.load().expect("load ok").expect("Some");
    assert_eq!(loaded.window_seconds, 60);
    assert_eq!(loaded.detection_window_seconds, 30);
    assert_eq!(loaded.suggested_threshold, 5);
    assert_eq!(loaded.autonomous_threshold, 10);
}

#[test]
fn load_returns_none_when_corpus_empty() {
    let writer = make_writer();
    let adapter = CorpusStormPersistence::new(writer);
    let loaded = adapter.load().expect("infallible on empty");
    assert!(loaded.is_none());
}

#[test]
fn save_then_restore_preserves_dedup_fingerprints() {
    let writer = make_writer();
    let adapter = CorpusStormPersistence::new(Arc::clone(&writer));
    let detector = RetryStormDetector::new(60, 30, 5, 10);
    let fingerprint: [u8; 16] = [0xAB; 16];
    for i in 0..10 {
        let _ = triage::contract::record_occurrence(
            &detector,
            fingerprint,
            "svc-storm",
            i * 1_000_000_000,
        );
    }
    assert!(detector.fingerprints_tracked() >= 1);
    let snapshot = detector.snapshot();
    adapter.save(&snapshot).expect("save");

    drop(detector);
    let loaded = adapter.load().expect("load").expect("Some");
    let restored = RetryStormDetector::restore_from_snapshot(loaded);
    assert_eq!(restored.fingerprints_tracked(), 1);
}

#[test]
fn save_writes_through_corpus_writer_pipeline_metric_slot() {
    let writer = make_writer();
    let adapter = CorpusStormPersistence::new(Arc::clone(&writer));
    let detector = RetryStormDetector::new(60, 30, 5, 10);
    let snapshot = detector.snapshot();
    adapter.save(&snapshot).expect("save");
    let bytes = writer
        .load_pipeline_metric(STORM_STATE_METRIC_NAME, STORM_PERSISTENCE_LAYER)
        .expect("load via writer")
        .expect("Some");
    assert!(!bytes.is_empty());
    let roundtrip: StormStateSnapshot = bincode::deserialize(&bytes).expect("bincode round-trip");
    assert_eq!(roundtrip.window_seconds, 60);
}

#[test]
fn metric_name_and_layer_constants_match_plan_spec() {
    assert_eq!(STORM_STATE_METRIC_NAME, "storm_state");
    assert_eq!(STORM_PERSISTENCE_LAYER, "l2");
}

#[test]
fn corpus_error_keyring_unavailable_maps_to_sanitized_io() {
    let err = corpus_error_to_storm_error(CorpusError::KeyringUnavailable);
    match err {
        StormError::Io { kind } => {
            assert_eq!(kind, std::io::ErrorKind::PermissionDenied);
        }
        other => panic!("expected Io, got {other:?}"),
    }
}

#[test]
fn corpus_error_query_failed_maps_to_sanitized_io() {
    let err = corpus_error_to_storm_error(CorpusError::QueryFailed);
    assert!(matches!(err, StormError::Io { .. }));
}

#[test]
fn corpus_error_decryption_failed_maps_to_deserialize() {
    let err = corpus_error_to_storm_error(CorpusError::DecryptionFailed);
    assert!(matches!(err, StormError::Deserialize));
}

#[test]
fn corpus_error_encryption_failed_maps_to_serialize() {
    let err = corpus_error_to_storm_error(CorpusError::EncryptionFailed);
    assert!(matches!(err, StormError::Serialize));
}

#[test]
fn corpus_error_schema_mismatch_maps_to_typed_variant() {
    let err = corpus_error_to_storm_error(CorpusError::SchemaVersionMismatch);
    match err {
        StormError::SchemaVersionMismatch { expected, got } => {
            assert_eq!(expected, SCHEMA_VERSION);
            assert_eq!(got, 0);
        }
        other => panic!("expected SchemaVersionMismatch, got {other:?}"),
    }
}

#[test]
fn non_pii_canary_in_fingerprint_service_redacted_by_encryption_at_rest() {
    // FingerprintState.service field carries service.name which is
    // PII-bearing. Two layers of defense apply at-rest:
    // (1) Chunk #72 producer-side `StormStateSnapshot::scrubbed_clone`
    //     replaces PII-matched service strings with `[REDACTED:*]` markers
    //     before bincode (only the marker bytes reach the bincode payload
    //     for matched categories).
    // (2) Corpus-wide AES-256-GCM cell-level encryption (chunk #68) covers
    //     the pipeline_metrics.payload BLOB regardless of scrubber outcome.
    // This test uses a non-PII canary (no P-047 category match) so the
    // scrubber passes the bytes through unchanged and the at-rest protection
    // comes from layer (2). The PII-shaped category coverage lives in
    // `tests/e2e_pii_scrubber_persistence_coverage.rs`.
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("storm-canary.db");
    let backend_key = [0x55u8; 32];
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
        "corpus-key",
        backend_key,
    ));
    let corpus = Corpus::open(db_path.clone(), backend).expect("open");
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let adapter = CorpusStormPersistence::new(writer);
    let detector = RetryStormDetector::new(60, 30, 5, 10);
    let canary = "EXCEPTION_CANARY_b8d3f7";
    for i in 0..5 {
        let _ =
            triage::contract::record_occurrence(&detector, [0x11; 16], canary, i * 1_000_000_000);
    }
    adapter.save(&detector.snapshot()).expect("save");
    let raw = std::fs::read(&db_path).expect("read db");
    let canary_position = raw
        .windows(canary.len())
        .position(|w| w == canary.as_bytes());
    assert!(
        canary_position.is_none(),
        "canary leaked at-rest at byte offset {canary_position:?}"
    );
}
