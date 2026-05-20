//! Corpus-backed RetryStormState persistence adapter — chunk #71.
//!
//! Wires `triage::contract::StormPersistence` to
//! `corpus::contract::CorpusWriter` at the pulse-app binary boundary,
//! preserving the arch §Module dependency direction DAG (triage stays
//! corpus-free; corpus stays triage-free; pulse-app owns the wire-up).
//! Mirrors chunk #70 `CorpusBaselinePersistence` precedent.
//!
//! Persistence cell shape per chunk #71 schema decision (Option A —
//! mirrors chunk #70 baseline + chunk #69 drain): re-uses the existing
//! `pipeline_metrics` SQLite table (chunk #68 schema, no version bump)
//! with `metric_name = "storm_state"`, `layer = "l2"`,
//! `payload = bincode-serialized StormStateSnapshot`. Cell-level
//! AES-256-GCM encryption is applied by the corpus crate transparently
//! to this adapter; serialization → encryption → INSERT is the save
//! path; SELECT → decryption → deserialization is the load path.
//!
//! Boot non-fatal: if corpus is unavailable at boot, the storm detector
//! runs in-memory-only for the session. Storm fingerprint bytes (16-byte
//! hashes of exception.type + normalized stack) are non-PII by
//! construction; FingerprintState.service field IS PII-bearing but
//! corpus-wide encryption at rest covers it (chunk #72 scrubber
//! extension is the producer-side defense-in-depth).

use std::sync::Arc;

use corpus::contract::{CorpusWriter, Error as CorpusError};
use triage::contract::{SCHEMA_VERSION, StormError, StormPersistence, StormStateSnapshot};

/// Stable `metric_name` value in `pipeline_metrics` for StormState.
/// Changing this string breaks in-place persistence reads against
/// existing on-disk corpora — schema-equivalent renaming would require
/// a migration path.
pub const STORM_STATE_METRIC_NAME: &str = "storm_state";

/// Stable `layer` tag in `pipeline_metrics`. Per pulse v0.2.0
/// distillation architecture, `l2` is the storm-detection layer.
pub const STORM_PERSISTENCE_LAYER: &str = "l2";

/// Adapter implementing `triage::StormPersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusStormPersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusStormPersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl StormPersistence for CorpusStormPersistence {
    fn load(&self) -> Result<Option<StormStateSnapshot>, StormError> {
        let bytes_opt = self
            .writer
            .load_pipeline_metric(STORM_STATE_METRIC_NAME, STORM_PERSISTENCE_LAYER)
            .map_err(corpus_error_to_storm_error)?;
        match bytes_opt {
            Some(bytes) => {
                let snapshot: StormStateSnapshot =
                    bincode::deserialize(&bytes).map_err(|_| StormError::Deserialize)?;
                Ok(Some(snapshot))
            }
            None => Ok(None),
        }
    }

    fn save(&self, snapshot: &StormStateSnapshot) -> Result<(), StormError> {
        let bytes = bincode::serialize(snapshot).map_err(|_| StormError::Serialize)?;
        self.writer
            .save_pipeline_metric(STORM_STATE_METRIC_NAME, STORM_PERSISTENCE_LAYER, &bytes)
            .map_err(corpus_error_to_storm_error)
    }
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the StormError surfaced upward. Mirrors
/// chunk #70 `corpus_error_to_baseline_error` precedent.
///
/// Free function (not `From` impl) — orphan rule forbids
/// `impl From<corpus::Error> for triage::StormError` here (both types
/// foreign to pulse-app). Per session-learnings 2026-05-18.
fn corpus_error_to_storm_error(err: CorpusError) -> StormError {
    match err {
        CorpusError::KeyringUnavailable => StormError::Io {
            kind: std::io::ErrorKind::PermissionDenied,
        },
        CorpusError::MigrationFailed => StormError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::EncryptionFailed => StormError::Serialize,
        CorpusError::DecryptionFailed => StormError::Deserialize,
        CorpusError::QueryFailed => StormError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::PathTraversal => StormError::PathTraversal,
        CorpusError::SizeCapExceeded => StormError::SizeCapExceeded { actual: 0, max: 0 },
        CorpusError::SchemaVersionMismatch => StormError::SchemaVersionMismatch {
            expected: SCHEMA_VERSION,
            got: 0,
        },
        CorpusError::Io { kind } => StormError::Io { kind },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus::contract::{Corpus, FakeKeychainBackend, KeychainBackend};
    use tempfile::TempDir;
    use triage::contract::RetryStormDetector;

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
        // Seed dedup state by recording 10 occurrences within window —
        // crosses Autonomous threshold so the detector tracks it.
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

        // Drop the detector + recreate from corpus.
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
        let roundtrip: StormStateSnapshot =
            bincode::deserialize(&bytes).expect("bincode round-trip");
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
    fn pii_canary_not_present_in_corpus_db_raw_bytes() {
        // FingerprintState.service field carries service.name which is
        // PII-bearing. Corpus-wide AES-256-GCM cell-level encryption
        // covers the pipeline_metrics.payload BLOB (chunk #68 substrate).
        // This test verifies encryption blocks the canary at-rest.
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
        // Seed detector with canary in service.name.
        for i in 0..5 {
            let _ = triage::contract::record_occurrence(
                &detector,
                [0x11; 16],
                canary,
                i * 1_000_000_000,
            );
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
}
