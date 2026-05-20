//! Unit tests for `CorpusLifecyclePersistence` adapter. Lives in `tests/`
//! per session-learnings 2026-05-13 (Windows WebView2 DLL load workaround
//! in `pulse-app/Cargo.toml [lib] test = false`).

use std::sync::Arc;

use corpus::contract::{
    Corpus, CorpusWriter, Error as CorpusError, FakeKeychainBackend, KeychainBackend,
};
use pulse_app::lifecycle_persistence::{
    CorpusLifecyclePersistence, corpus_error_to_lifecycle_error, parse_state,
};
use tempfile::TempDir;
use triage::contract::{
    LifecycleError, LifecyclePersistence, SCHEMA_VERSION, ServiceLifecycleState,
    ServiceRegistryEntry, state_label,
};

fn make_writer() -> Arc<dyn CorpusWriter> {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    Arc::new(corpus) as Arc<dyn CorpusWriter>
}

fn entry(state: ServiceLifecycleState, ts: i64) -> ServiceRegistryEntry {
    ServiceRegistryEntry {
        state,
        first_seen_unix_nano: ts,
        last_seen_unix_nano: ts,
        last_transition_unix_nano: ts,
        manual_override: None,
    }
}

#[test]
fn save_then_load_round_trips_three_services() {
    let writer = make_writer();
    let adapter = CorpusLifecyclePersistence::new(writer);
    let entries = vec![
        (
            "svc-a".to_string(),
            entry(ServiceLifecycleState::Active, 1_000),
        ),
        (
            "svc-b".to_string(),
            entry(ServiceLifecycleState::Quiet, 2_000),
        ),
        (
            "svc-c".to_string(),
            entry(ServiceLifecycleState::Silent, 3_000),
        ),
    ];
    adapter.save_all(&entries).expect("save");
    let loaded = adapter.load_all().expect("load ok").expect("Some");
    assert_eq!(loaded.len(), 3);
    let svc_a = loaded.iter().find(|(n, _)| n == "svc-a").expect("svc-a");
    assert_eq!(svc_a.1.state, ServiceLifecycleState::Active);
    let svc_b = loaded.iter().find(|(n, _)| n == "svc-b").expect("svc-b");
    assert_eq!(svc_b.1.state, ServiceLifecycleState::Quiet);
    let svc_c = loaded.iter().find(|(n, _)| n == "svc-c").expect("svc-c");
    assert_eq!(svc_c.1.state, ServiceLifecycleState::Silent);
}

#[test]
fn load_returns_none_when_corpus_empty() {
    let writer = make_writer();
    let adapter = CorpusLifecyclePersistence::new(writer);
    let loaded = adapter.load_all().expect("infallible on empty");
    assert!(loaded.is_none());
}

#[test]
fn save_persists_manual_override_field() {
    let writer = make_writer();
    let adapter = CorpusLifecyclePersistence::new(writer);
    let entries = vec![(
        "svc-pinned".to_string(),
        ServiceRegistryEntry {
            state: ServiceLifecycleState::Active,
            first_seen_unix_nano: 1_000,
            last_seen_unix_nano: 1_000,
            last_transition_unix_nano: 1_000,
            manual_override: Some(ServiceLifecycleState::Archived),
        },
    )];
    adapter.save_all(&entries).expect("save");
    let loaded = adapter.load_all().expect("load").expect("Some");
    assert_eq!(loaded.len(), 1);
    assert_eq!(
        loaded[0].1.manual_override,
        Some(ServiceLifecycleState::Archived)
    );
}

#[test]
fn save_upsert_overwrites_same_service_state() {
    let writer = make_writer();
    let adapter = CorpusLifecyclePersistence::new(writer);
    adapter
        .save_all(&[(
            "svc-x".to_string(),
            entry(ServiceLifecycleState::Active, 1_000),
        )])
        .expect("save initial");
    adapter
        .save_all(&[(
            "svc-x".to_string(),
            entry(ServiceLifecycleState::Quiet, 2_000),
        )])
        .expect("save update");
    let loaded = adapter.load_all().expect("load").expect("Some");
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].1.state, ServiceLifecycleState::Quiet);
}

#[test]
fn parse_state_round_trips_all_seven_variants() {
    for state in [
        ServiceLifecycleState::Unknown,
        ServiceLifecycleState::Bootstrapping,
        ServiceLifecycleState::Active,
        ServiceLifecycleState::Quiet,
        ServiceLifecycleState::Silent,
        ServiceLifecycleState::Dormant,
        ServiceLifecycleState::Archived,
    ] {
        let label = state_label(state);
        let parsed = parse_state(label).expect("round-trip");
        assert_eq!(parsed, state);
    }
}

#[test]
fn parse_state_returns_none_for_unknown_text() {
    assert!(parse_state("not_a_state").is_none());
}

#[test]
fn corpus_error_keyring_unavailable_maps_to_sanitized_io() {
    let err = corpus_error_to_lifecycle_error(CorpusError::KeyringUnavailable);
    match err {
        LifecycleError::Io { kind } => {
            assert_eq!(kind, std::io::ErrorKind::PermissionDenied);
        }
        other => panic!("expected Io, got {other:?}"),
    }
}

#[test]
fn corpus_error_query_failed_maps_to_sanitized_io() {
    let err = corpus_error_to_lifecycle_error(CorpusError::QueryFailed);
    assert!(matches!(err, LifecycleError::Io { .. }));
}

#[test]
fn corpus_error_decryption_failed_maps_to_deserialize() {
    let err = corpus_error_to_lifecycle_error(CorpusError::DecryptionFailed);
    assert!(matches!(err, LifecycleError::Deserialize));
}

#[test]
fn corpus_error_encryption_failed_maps_to_serialize() {
    let err = corpus_error_to_lifecycle_error(CorpusError::EncryptionFailed);
    assert!(matches!(err, LifecycleError::Serialize));
}

#[test]
fn corpus_error_schema_mismatch_maps_to_typed_variant() {
    let err = corpus_error_to_lifecycle_error(CorpusError::SchemaVersionMismatch);
    match err {
        LifecycleError::SchemaVersionMismatch { expected, got } => {
            assert_eq!(expected, SCHEMA_VERSION);
            assert_eq!(got, 0);
        }
        other => panic!("expected SchemaVersionMismatch, got {other:?}"),
    }
}

#[test]
fn save_load_preserves_per_service_timestamps() {
    let writer = make_writer();
    let adapter = CorpusLifecyclePersistence::new(writer);
    let entry = ServiceRegistryEntry {
        state: ServiceLifecycleState::Active,
        first_seen_unix_nano: 1_111_111,
        last_seen_unix_nano: 2_222_222,
        last_transition_unix_nano: 3_333_333,
        manual_override: None,
    };
    adapter
        .save_all(&[("svc-ts".to_string(), entry)])
        .expect("save");
    let loaded = adapter.load_all().expect("load").expect("Some");
    let (_, restored) = &loaded[0];
    assert_eq!(restored.first_seen_unix_nano, 1_111_111);
    assert_eq!(restored.last_seen_unix_nano, 2_222_222);
    assert_eq!(restored.last_transition_unix_nano, 3_333_333);
}

#[test]
fn non_pii_service_name_passes_through_to_at_rest_text_column() {
    // service_name TEXT column is queryable plaintext (corpus-wide AES
    // covers payload BLOBs only). Chunk #72 wires the PII scrubber at the
    // producer-side `save_all` path — non-PII-shaped service names flow
    // through scrubber-Allowed and remain plaintext. The PII-shaped path
    // is in `tests/e2e_pii_scrubber_persistence_coverage.rs`.
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("lifecycle-canary.db");
    let backend_key = [0x42u8; 32];
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
        "corpus-key",
        backend_key,
    ));
    let corpus = Corpus::open(db_path.clone(), backend).expect("open");
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let adapter = CorpusLifecyclePersistence::new(writer);
    let canary = "SERVICE_CANARY_77f8d3";
    adapter
        .save_all(&[(
            canary.to_string(),
            entry(ServiceLifecycleState::Active, 1_000),
        )])
        .expect("save");
    let raw = std::fs::read(&db_path).expect("read db");
    let canary_position = raw
        .windows(canary.len())
        .position(|w| w == canary.as_bytes());
    assert!(
        canary_position.is_some(),
        "non-PII-shaped service_name expected plaintext at-rest after chunk #72 scrubber wire"
    );
}
