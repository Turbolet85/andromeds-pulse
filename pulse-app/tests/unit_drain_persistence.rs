//! Unit tests for `CorpusDrainPersistence` adapter. Lives in `tests/`
//! (integration test crate) because `pulse-app` declares
//! `[lib] test = false` in Cargo.toml (Windows WebView2 DLL load
//! workaround per session-learnings 2026-05-13); source-level
//! `#[cfg(test)] mod tests` would compile but never run.

use std::sync::Arc;

use buffer::{DrainConfig, DrainMiner, DrainPersistence, DrainState, Error as BufferError};
use corpus::contract::{
    Corpus, CorpusWriter, Error as CorpusError, FakeKeychainBackend, KeychainBackend,
};
use pulse_app::drain_persistence::{
    CorpusDrainPersistence, DRAIN_PERSISTENCE_LAYER, DRAIN_TEMPLATE_METRIC_NAME,
    corpus_error_to_buffer_error,
};

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

    let bytes = writer
        .load_pipeline_metric(DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER)
        .expect("load via writer")
        .expect("Some bytes");
    assert!(!bytes.is_empty());
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
