//! Corpus-backed Drain persistence adapter — chunk #69 Phase B Session 4.
//!
//! Wires `buffer::DrainPersistence` к `corpus::contract::CorpusWriter`
//! at the pulse-app binary boundary, preserving the arch §Module
//! dependency direction DAG (buffer stays corpus-free; corpus stays
//! buffer-free; pulse-app owns the wire-up). Mirrors chunk #59's
//! `HeartbeatBindStatus` trait-in-lower-crate + adapter-in-pulse-app
//! pattern (per session-learnings 2026-05-16).
//!
//! Persistence cell shape per chunk #69 plan Open Question Q1 option (a):
//! re-uses the existing `pipeline_metrics` SQLite table (chunk #68
//! schema, no version bump) with `metric_name = "drain_template_tree"`,
//! `layer = "l1c"`, `payload = bincode-serialized DrainState`.
//! Cell-level AES-256-GCM encryption is applied by the corpus crate
//! transparently to this adapter; serialization → encryption →
//! INSERT is the save path; SELECT → decryption → deserialization is
//! the load path.
//!
//! PII discipline: the DrainState payload is built from already-PII-
//! scrubbed template content (the appender / DuckDB write path calls
//! `security::scrubber::scrub_attribute` before storing a template;
//! the buffer-side `DrainMiner` operates on those scrubbed strings).
//! Encryption is defense-in-depth для the at-rest threat model, not a
//! substitute for that producer-side scrubbing.

use std::sync::Arc;

use buffer::{DrainPersistence, DrainState, Error as BufferError};
use corpus::contract::{CorpusWriter, Error as CorpusError};

/// Stable `metric_name` value in `pipeline_metrics` for the Drain
/// template tree. Changing this string breaks in-place persistence
/// reads against existing on-disk corpora — schema-equivalent renaming
/// would require a migration path.
pub const DRAIN_TEMPLATE_METRIC_NAME: &str = "drain_template_tree";

/// Stable `layer` tag in `pipeline_metrics`. Per pulse v0.2.0
/// distillation architecture, `l1c` is the log-template-mining layer
/// (DrainMiner output sits there).
pub const DRAIN_PERSISTENCE_LAYER: &str = "l1c";

/// Adapter implementing `buffer::DrainPersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap к clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusDrainPersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusDrainPersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl DrainPersistence for CorpusDrainPersistence {
    fn load(&self) -> Result<Option<DrainState>, BufferError> {
        let bytes_opt = self
            .writer
            .load_pipeline_metric(DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER)
            .map_err(corpus_error_to_buffer_error)?;
        match bytes_opt {
            Some(bytes) => {
                let state: DrainState =
                    bincode::deserialize(&bytes).map_err(|_| BufferError::Drain {
                        reason: "drain state bincode decode failed".to_string(),
                    })?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    fn save(&self, state: &DrainState) -> Result<(), BufferError> {
        let bytes = bincode::serialize(state).map_err(|_| BufferError::Drain {
            reason: "drain state bincode encode failed".to_string(),
        })?;
        self.writer
            .save_pipeline_metric(DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER, &bytes)
            .map_err(corpus_error_to_buffer_error)
    }
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the message surfaced upward to the buffer
/// layer (the buffer Error::Drain.reason field IS visible через
/// `From<BufferError> for AppError` IPC propagation, so noise is risk).
fn corpus_error_to_buffer_error(err: CorpusError) -> BufferError {
    let reason = match err {
        CorpusError::KeyringUnavailable => "corpus keychain unavailable",
        CorpusError::MigrationFailed => "corpus schema migration failed",
        CorpusError::EncryptionFailed => "corpus encryption failed",
        CorpusError::DecryptionFailed => "corpus decryption failed",
        CorpusError::QueryFailed => "corpus persistence query failed",
        CorpusError::PathTraversal => "corpus path rejected",
        CorpusError::SizeCapExceeded => "corpus file too large",
        CorpusError::SchemaVersionMismatch => "corpus schema version mismatch",
        CorpusError::Io { .. } => "corpus io error",
    };
    BufferError::Drain {
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffer::{DrainConfig, DrainMiner};
    use corpus::contract::{Corpus, FakeKeychainBackend, KeychainBackend};

    fn make_writer() -> Arc<dyn CorpusWriter> {
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
        let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
        Arc::new(corpus) as Arc<dyn CorpusWriter>
    }

    #[test]
    fn save_then_load_round_trips_drain_state_through_corpus() {
        let writer = make_writer();
        let adapter = CorpusDrainPersistence::new(writer);

        let miner_a = DrainMiner::new(
            DrainConfig::default_config(),
            Some(Arc::new(adapter.clone()) as Arc<dyn DrainPersistence>),
        );
        miner_a.assign("alpha event happened at startup");
        miner_a.assign("alpha event happened at startup");
        miner_a.assign("beta different message entirely");
        let template_count_before = miner_a.template_count();

        miner_a.persist().expect("persist via adapter");

        // Fresh miner, same persistence adapter → load_from_persistence
        // rehydrates the prior tree.
        let miner_b = DrainMiner::new(
            DrainConfig::default_config(),
            Some(Arc::new(adapter) as Arc<dyn DrainPersistence>),
        );
        assert_eq!(miner_b.template_count(), 0);
        let loaded = miner_b.load_from_persistence().expect("load");
        assert!(loaded);
        assert_eq!(miner_b.template_count(), template_count_before);
    }

    #[test]
    fn load_returns_none_when_corpus_empty() {
        let writer = make_writer();
        let adapter = CorpusDrainPersistence::new(writer);
        let loaded = adapter.load().expect("infallible on empty");
        assert!(loaded.is_none());
    }

    #[test]
    fn save_writes_through_corpus_writer_save_pipeline_metric() {
        let writer = make_writer();
        let adapter = CorpusDrainPersistence::new(Arc::clone(&writer));
        let miner = DrainMiner::new(DrainConfig::default_config(), None);
        miner.assign("save canary message one");
        let state = miner.snapshot_state().expect("snapshot");
        adapter.save(&state).expect("save");

        // Reading via the CorpusWriter directly with the same
        // metric_name + layer should yield bytes.
        let bytes = writer
            .load_pipeline_metric(DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER)
            .expect("load via writer")
            .expect("Some bytes");
        assert!(!bytes.is_empty());
        // Bytes should be bincode-deserializable back to DrainState.
        let roundtrip: DrainState =
            bincode::deserialize(&bytes).expect("bincode round-trip via writer");
        assert_eq!(roundtrip.schema_version, state.schema_version);
    }

    #[test]
    fn corpus_error_keyring_unavailable_maps_to_sanitized_buffer_drain() {
        let err = corpus_error_to_buffer_error(CorpusError::KeyringUnavailable);
        match err {
            BufferError::Drain { reason } => {
                assert_eq!(reason, "corpus keychain unavailable");
                assert!(!reason.contains("/"));
                assert!(!reason.contains("rusqlite"));
                assert!(!reason.contains("SELECT"));
            }
            other => panic!("expected BufferError::Drain, got {other:?}"),
        }
    }

    #[test]
    fn corpus_error_query_failed_maps_to_sanitized_message() {
        let err = corpus_error_to_buffer_error(CorpusError::QueryFailed);
        match err {
            BufferError::Drain { reason } => {
                assert_eq!(reason, "corpus persistence query failed");
            }
            other => panic!("expected BufferError::Drain, got {other:?}"),
        }
    }

    #[test]
    fn corpus_error_decryption_failed_maps_to_sanitized_message() {
        let err = corpus_error_to_buffer_error(CorpusError::DecryptionFailed);
        match err {
            BufferError::Drain { reason } => {
                assert_eq!(reason, "corpus decryption failed");
                assert!(!reason.contains("0x"));
            }
            other => panic!("expected BufferError::Drain, got {other:?}"),
        }
    }

    #[test]
    fn metric_name_and_layer_constants_match_plan_spec() {
        assert_eq!(DRAIN_TEMPLATE_METRIC_NAME, "drain_template_tree");
        assert_eq!(DRAIN_PERSISTENCE_LAYER, "l1c");
    }
}
