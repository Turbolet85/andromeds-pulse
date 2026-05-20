//! E2E coverage for chunk #72 PII scrubber wire across the four corpus
//! persistence adapters: drain template tree, ServiceRegistry rows,
//! RetryStormState, BaselineState.
//!
//! Each test seeds a PII-shaped value (email-shaped canary chosen as the
//! canonical P-047 category since email is recognizable + non-overlapping
//! with other regex set entries), drives it through the adapter's `save`
//! path, then verifies (a) the raw canary substring is absent from the
//! persisted corpus bytes and (b) the `[REDACTED:email]` marker is present
//! (where checkable via bincode round-trip or query).
//!
//! Why integration test instead of co-located source test: `pulse-app`
//! declares `[lib] test = false` in Cargo.toml (per session-learnings
//! 2026-05-13 — Windows WebView2 DLL load failure on the auto-generated
//! lib test binary). Integration tests under `pulse-app/tests/` are the
//! canonical place for pulse-app tests that exercise the lib surface.

use std::sync::Arc;

use buffer::{DrainConfig, DrainMiner, DrainPersistence};
use corpus::contract::{Corpus, CorpusWriter, FakeKeychainBackend, KeychainBackend};
use pulse_app::baseline_persistence::{
    BASELINE_PERSISTENCE_LAYER, BASELINE_STATE_METRIC_NAME, CorpusBaselinePersistence,
};
use pulse_app::drain_persistence::{
    CorpusDrainPersistence, DRAIN_PERSISTENCE_LAYER, DRAIN_TEMPLATE_METRIC_NAME,
};
use pulse_app::lifecycle_persistence::CorpusLifecyclePersistence;
use pulse_app::storm_persistence::{
    CorpusStormPersistence, STORM_PERSISTENCE_LAYER, STORM_STATE_METRIC_NAME,
};
use triage::contract::{
    BaselinePersistence, BaselineState, LifecyclePersistence, RetryStormDetector,
    ServiceLifecycleState, ServiceRegistryEntry, StormPersistence,
};

fn make_writer() -> Arc<dyn CorpusWriter> {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    Arc::new(corpus) as Arc<dyn CorpusWriter>
}

#[test]
fn drain_persistence_pii_canary_does_not_leak_into_corpus_bytes() {
    let writer = make_writer();
    let adapter = CorpusDrainPersistence::new(Arc::clone(&writer));
    let miner = DrainMiner::new(DrainConfig::default_config(), None);
    let canary = "canary-drain-adapter-email@example.com";
    miner.assign(&format!(
        "user logged in from {canary} on alpha-tier with status 200"
    ));
    let state = miner.snapshot_state().expect("snapshot");
    adapter.save(&state).expect("save");

    let bytes = writer
        .load_pipeline_metric(DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER)
        .expect("load via writer")
        .expect("Some bytes");
    let canary_bytes = canary.as_bytes();
    let leak_position = bytes
        .windows(canary_bytes.len())
        .position(|w| w == canary_bytes);
    assert!(
        leak_position.is_none(),
        "raw email canary leaked into corpus bincode bytes at offset {leak_position:?}"
    );
    let marker = b"[REDACTED:email]";
    let marker_position = bytes.windows(marker.len()).position(|w| w == marker);
    assert!(
        marker_position.is_some(),
        "expected [REDACTED:email] marker in corpus bincode bytes (snapshot_state scrubs upstream); \
         not found in {} bytes",
        bytes.len()
    );
}

#[test]
fn lifecycle_persistence_pii_shaped_service_name_redacted_via_scrubber() {
    let writer = make_writer();
    let adapter = CorpusLifecyclePersistence::new(writer);
    let pii_service = "canary-lifecycle-email@example.com";
    let entry = ServiceRegistryEntry {
        state: ServiceLifecycleState::Active,
        first_seen_unix_nano: 1_000,
        last_seen_unix_nano: 1_000,
        last_transition_unix_nano: 1_000,
        manual_override: None,
    };
    adapter
        .save_all(&[(pii_service.to_string(), entry)])
        .expect("save");
    let loaded = adapter.load_all().expect("load").expect("Some");
    assert_eq!(loaded.len(), 1);
    let (loaded_name, _) = &loaded[0];
    assert_ne!(
        loaded_name, pii_service,
        "scrubber should have replaced PII-shaped service_name with marker"
    );
    assert_eq!(
        loaded_name, "[REDACTED:email]",
        "expected [REDACTED:email] marker, got {loaded_name:?}"
    );
}

#[test]
fn lifecycle_persistence_pii_shaped_service_name_absent_from_at_rest_bytes() {
    let tmp = tempfile::TempDir::new().expect("tmp");
    let db_path = tmp.path().join("lifecycle-pii-at-rest.db");
    let backend_key = [0x99u8; 32];
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
        "corpus-key",
        backend_key,
    ));
    let corpus = Corpus::open(db_path.clone(), backend).expect("open");
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let adapter = CorpusLifecyclePersistence::new(writer);
    let pii_service = "canary-at-rest-pii@example.com";
    let entry = ServiceRegistryEntry {
        state: ServiceLifecycleState::Active,
        first_seen_unix_nano: 1_000,
        last_seen_unix_nano: 1_000,
        last_transition_unix_nano: 1_000,
        manual_override: None,
    };
    adapter
        .save_all(&[(pii_service.to_string(), entry)])
        .expect("save");
    let raw = std::fs::read(&db_path).expect("read db");
    let leak_position = raw
        .windows(pii_service.len())
        .position(|w| w == pii_service.as_bytes());
    assert!(
        leak_position.is_none(),
        "raw PII-shaped service_name leaked at-rest at byte offset {leak_position:?}"
    );
}

#[test]
fn storm_persistence_pii_shaped_fingerprint_service_redacted_in_bincode_payload() {
    let writer = make_writer();
    let adapter = CorpusStormPersistence::new(Arc::clone(&writer));
    let detector = RetryStormDetector::new(60, 30, 5, 10);
    let pii_service = "canary-storm-fingerprint-email@example.com";
    for i in 0..3 {
        let _ = triage::contract::record_occurrence(
            &detector,
            [0x22; 16],
            pii_service,
            i * 1_000_000_000,
        );
    }
    adapter.save(&detector.snapshot()).expect("save");
    let bytes = writer
        .load_pipeline_metric(STORM_STATE_METRIC_NAME, STORM_PERSISTENCE_LAYER)
        .expect("load via writer")
        .expect("Some");
    let pii_bytes = pii_service.as_bytes();
    let leak_position = bytes.windows(pii_bytes.len()).position(|w| w == pii_bytes);
    assert!(
        leak_position.is_none(),
        "raw PII-shaped service.name leaked into corpus bincode bytes at offset {leak_position:?}"
    );
    let marker = b"[REDACTED:email]";
    let marker_position = bytes.windows(marker.len()).position(|w| w == marker);
    assert!(
        marker_position.is_some(),
        "expected [REDACTED:email] marker in corpus bincode bytes (scrubber wired at producer side); \
         not found in {} bytes",
        bytes.len()
    );
}

#[test]
fn baseline_persistence_pii_shaped_service_key_redacted_in_bincode_payload() {
    let writer = make_writer();
    let adapter = CorpusBaselinePersistence::new(Arc::clone(&writer));
    let pii_service = "canary-baseline-email@example.com";
    let state = BaselineState::new();
    for i in 0..5 {
        state.observe_span(pii_service, "op-canary", 0, 100, (i + 1) * 1_000_000);
    }
    adapter.save(&state).expect("save");
    let bytes = writer
        .load_pipeline_metric(BASELINE_STATE_METRIC_NAME, BASELINE_PERSISTENCE_LAYER)
        .expect("load via writer")
        .expect("Some");
    let pii_bytes = pii_service.as_bytes();
    let leak_position = bytes.windows(pii_bytes.len()).position(|w| w == pii_bytes);
    assert!(
        leak_position.is_none(),
        "raw PII-shaped service key leaked into corpus bincode bytes at offset {leak_position:?}"
    );
    let marker = b"[REDACTED:email]";
    let marker_position = bytes.windows(marker.len()).position(|w| w == marker);
    assert!(
        marker_position.is_some(),
        "expected [REDACTED:email] marker in corpus bincode bytes (scrubber wired at producer side); \
         not found in {} bytes",
        bytes.len()
    );
}
