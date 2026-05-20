//! Unit tests for `CorpusBaselinePersistence` adapter + legacy bincode
//! migration helper. Lives in `tests/` per session-learnings 2026-05-13
//! (Windows WebView2 DLL load workaround in `pulse-app/Cargo.toml
//! [lib] test = false`).

use std::path::Path;
use std::sync::{Arc, Mutex};

use corpus::contract::{
    Corpus, CorpusWriter, Error as CorpusError, FakeKeychainBackend, KeychainBackend,
};
use pulse_app::baseline_persistence::{
    BASELINE_PERSISTENCE_LAYER, BASELINE_STATE_METRIC_NAME, CorpusBaselinePersistence,
    LEGACY_BASELINE_BASENAME, LEGACY_BASELINE_SUBDIR, MigrationOutcome,
    corpus_error_to_baseline_error, migrate_legacy_baseline_if_present,
};
use tempfile::TempDir;
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use triage::contract::{BaselineError, BaselinePersistence, BaselineState};

fn make_writer() -> Arc<dyn CorpusWriter> {
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
    let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
    Arc::new(corpus) as Arc<dyn CorpusWriter>
}

#[test]
fn save_then_load_round_trips_baseline_state_through_corpus() {
    let writer = make_writer();
    let adapter = CorpusBaselinePersistence::new(writer);

    let state_a = BaselineState::new();
    for i in 0..30 {
        state_a.observe_span("svc-a", "op-1", 0, 100, i * 1_000_000);
    }
    state_a.set_persisted_at_unix_nanos(5_000);
    adapter.save(&state_a).expect("save");

    let loaded = adapter.load().expect("load ok").expect("Some(state)");
    assert_eq!(loaded.service_count(), state_a.service_count());
    assert_eq!(loaded.persisted_at_unix_nanos(), 5_000);
}

#[test]
fn load_returns_none_when_corpus_empty() {
    let writer = make_writer();
    let adapter = CorpusBaselinePersistence::new(writer);
    let loaded = adapter.load().expect("infallible on empty");
    assert!(loaded.is_none());
}

#[test]
fn save_writes_through_corpus_writer_save_pipeline_metric() {
    let writer = make_writer();
    let adapter = CorpusBaselinePersistence::new(Arc::clone(&writer));
    let state = BaselineState::new();
    state.observe_span("svc-canary", "op-1", 0, 100, 1_000);
    adapter.save(&state).expect("save");

    let bytes = writer
        .load_pipeline_metric(BASELINE_STATE_METRIC_NAME, BASELINE_PERSISTENCE_LAYER)
        .expect("load via writer")
        .expect("Some bytes");
    assert!(!bytes.is_empty());
    let roundtrip: BaselineState =
        bincode::deserialize(&bytes).expect("bincode round-trip via writer");
    assert_eq!(roundtrip.service_count(), 1);
}

#[test]
fn corpus_error_keyring_unavailable_maps_to_sanitized_baseline_io() {
    let err = corpus_error_to_baseline_error(CorpusError::KeyringUnavailable);
    match err {
        BaselineError::Io { kind } => {
            assert_eq!(kind, std::io::ErrorKind::PermissionDenied);
        }
        other => panic!("expected BaselineError::Io, got {other:?}"),
    }
}

#[test]
fn corpus_error_query_failed_maps_to_sanitized_baseline_io() {
    let err = corpus_error_to_baseline_error(CorpusError::QueryFailed);
    match err {
        BaselineError::Io { kind } => {
            assert_eq!(kind, std::io::ErrorKind::Other);
        }
        other => panic!("expected BaselineError::Io, got {other:?}"),
    }
}

#[test]
fn corpus_error_decryption_failed_maps_to_baseline_deserialize() {
    let err = corpus_error_to_baseline_error(CorpusError::DecryptionFailed);
    assert!(matches!(err, BaselineError::Deserialize));
}

#[test]
fn corpus_error_encryption_failed_maps_to_baseline_serialize() {
    let err = corpus_error_to_baseline_error(CorpusError::EncryptionFailed);
    assert!(matches!(err, BaselineError::Serialize));
}

#[test]
fn corpus_error_schema_version_mismatch_maps_with_expected_constant() {
    let err = corpus_error_to_baseline_error(CorpusError::SchemaVersionMismatch);
    match err {
        BaselineError::SchemaVersionMismatch { expected, got } => {
            assert_eq!(expected, triage::contract::SCHEMA_VERSION);
            assert_eq!(got, 0);
        }
        other => panic!("expected SchemaVersionMismatch, got {other:?}"),
    }
}

#[test]
fn metric_name_and_layer_constants_match_plan_spec() {
    assert_eq!(BASELINE_STATE_METRIC_NAME, "baseline_state");
    assert_eq!(BASELINE_PERSISTENCE_LAYER, "l1b");
}

// ===== Migration helper tests =====

type CapturedFields = Vec<(String, String)>;
type CapturedEvent = (String, Level, CapturedFields);
type CapturedEvents = Arc<Mutex<Vec<CapturedEvent>>>;

#[derive(Default)]
struct CapturingSubscriber {
    events: CapturedEvents,
}

impl CapturingSubscriber {
    fn new() -> (Self, CapturedEvents) {
        let events: CapturedEvents = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                events: Arc::clone(&events),
            },
            events,
        )
    }
}

struct FieldCollector(Vec<(String, String)>);

impl Visit for FieldCollector {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0
            .push((field.name().to_string(), format!("{value:?}")));
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.push((field.name().to_string(), value.to_string()));
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.push((field.name().to_string(), value.to_string()));
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.push((field.name().to_string(), value.to_string()));
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.push((field.name().to_string(), value.to_string()));
    }
}

impl Subscriber for CapturingSubscriber {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::Id {
        tracing::Id::from_u64(1)
    }
    fn record(&self, _: &tracing::Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &tracing::Id, _: &tracing::Id) {}
    fn event(&self, event: &Event<'_>) {
        let target = event.metadata().target().to_string();
        let level = *event.metadata().level();
        let mut collector = FieldCollector(Vec::new());
        event.record(&mut collector);
        self.events
            .lock()
            .unwrap()
            .push((target, level, collector.0));
    }
    fn enter(&self, _: &tracing::Id) {}
    fn exit(&self, _: &tracing::Id) {}
}

fn stage_legacy_file(tmp_data_dir: &Path, state: &BaselineState) {
    let triage_dir = tmp_data_dir.join(LEGACY_BASELINE_SUBDIR);
    std::fs::create_dir_all(&triage_dir).expect("mkdir triage");
    let path = triage_dir.join(LEGACY_BASELINE_BASENAME);
    let bytes = bincode::serialize(state).expect("bincode legacy");
    std::fs::write(&path, &bytes).expect("write legacy");
}

fn make_adapter() -> CorpusBaselinePersistence {
    let writer = make_writer();
    CorpusBaselinePersistence::new(writer)
}

#[test]
fn migrate_legacy_returns_noop_when_no_legacy_file() {
    let tmp = TempDir::new().expect("tmp data dir");
    let adapter = make_adapter();
    let outcome = migrate_legacy_baseline_if_present(tmp.path(), &adapter);
    assert_eq!(outcome, MigrationOutcome::Noop);
}

#[test]
fn migrate_legacy_completed_deletes_legacy_file_and_persists_to_corpus() {
    let tmp = TempDir::new().expect("tmp data dir");
    let staged = BaselineState::new();
    for i in 0..5 {
        staged.observe_span("svc-canary", "op-x", 0, 100, i * 1_000_000);
    }
    staged.set_persisted_at_unix_nanos(7_000);
    stage_legacy_file(tmp.path(), &staged);

    let adapter = make_adapter();
    let outcome = migrate_legacy_baseline_if_present(tmp.path(), &adapter);

    match outcome {
        MigrationOutcome::Completed {
            legacy_file_deleted,
            bytes,
            service_count,
        } => {
            assert!(legacy_file_deleted);
            assert!(bytes > 0);
            assert_eq!(service_count, 1);
        }
        other => panic!("expected Completed, got {other:?}"),
    }

    let legacy_path = tmp
        .path()
        .join(LEGACY_BASELINE_SUBDIR)
        .join(LEGACY_BASELINE_BASENAME);
    assert!(
        !legacy_path.exists(),
        "legacy file must be deleted after migration"
    );

    let loaded = adapter.load().expect("load ok").expect("Some(state)");
    assert_eq!(loaded.service_count(), 1);
    assert_eq!(loaded.persisted_at_unix_nanos(), 7_000);
}

#[test]
fn migrate_legacy_idempotent_on_second_call() {
    let tmp = TempDir::new().expect("tmp data dir");
    let staged = BaselineState::new();
    staged.observe_span("svc", "op", 0, 100, 1_000);
    stage_legacy_file(tmp.path(), &staged);

    let adapter = make_adapter();
    let first = migrate_legacy_baseline_if_present(tmp.path(), &adapter);
    assert!(matches!(first, MigrationOutcome::Completed { .. }));

    let second = migrate_legacy_baseline_if_present(tmp.path(), &adapter);
    assert_eq!(
        second,
        MigrationOutcome::Noop,
        "second boot must skip migration"
    );
}

#[test]
fn migrate_legacy_failed_on_corrupt_bytes_preserves_legacy_file() {
    let tmp = TempDir::new().expect("tmp data dir");
    let triage_dir = tmp.path().join(LEGACY_BASELINE_SUBDIR);
    std::fs::create_dir_all(&triage_dir).expect("mkdir triage");
    let legacy_path = triage_dir.join(LEGACY_BASELINE_BASENAME);
    // Corrupt bytes with a valid-looking 8-byte usize length prefix —
    // first 8 bytes decode to ~8.9e18 entries. Plain `bincode::deserialize`
    // (bincode 1.3 default Config, no size limit) would pre-allocate a
    // Vec of that capacity и trigger an OOM process abort. The chunk #72
    // follow-up `pulse_app::bincode_bounded::deserialize` helper bounds
    // length prefixes by `DEFAULT_MAX_SIZE_BYTES`, returning a graceful
    // `Err::SizeLimit` instead — this test verifies that protection.
    std::fs::write(&legacy_path, b"\x00\x01\x02 garbage bytes").expect("write");

    let adapter = make_adapter();
    let outcome = migrate_legacy_baseline_if_present(tmp.path(), &adapter);

    match outcome {
        MigrationOutcome::Failed { error_category } => {
            assert_eq!(error_category, "deserialize");
        }
        other => panic!("expected Failed, got {other:?}"),
    }

    assert!(
        legacy_path.exists(),
        "legacy file must be preserved on failure for retry"
    );
}

// Chunk #72 follow-up — additional negative cases for the bounded
// deserialize. Exercises the SizeLimit error path с several crafted
// prefixes to catch any future regression of the bounded-deserialize
// helper (e.g., accidental removal of `.with_limit(...)` from
// `bincode_bounded::deserialize` would re-OOM here).
#[test]
fn migrate_legacy_rejects_oversized_length_prefix_without_oom() {
    let tmp = TempDir::new().expect("tmp data dir");
    let triage_dir = tmp.path().join(LEGACY_BASELINE_SUBDIR);
    std::fs::create_dir_all(&triage_dir).expect("mkdir triage");
    let legacy_path = triage_dir.join(LEGACY_BASELINE_BASENAME);
    // Schema_version u32 prefix (4 bytes) + a u64 length prefix at the
    // services-map slot pointing at 2^40 entries. Without the chunk #72
    // follow-up `baseline_bytes_prefix_plausible` validator, bincode 1.3.3
    // would pre-allocate а DashMap с that capacity и OOM-abort the test
    // process. With the validator, this rejects gracefully as `Failed`.
    let mut bytes = vec![0u8, 0, 0, 0]; // schema_version
    let huge_len: u64 = 1u64 << 40;
    bytes.extend_from_slice(&huge_len.to_le_bytes());
    bytes.push(0xFFu8); // trailing garbage
    assert_eq!(bytes.len(), 13);
    std::fs::write(&legacy_path, &bytes).expect("write");

    let adapter = make_adapter();
    let outcome = migrate_legacy_baseline_if_present(tmp.path(), &adapter);

    match outcome {
        MigrationOutcome::Failed { error_category } => {
            assert_eq!(error_category, "deserialize");
        }
        other => panic!("expected Failed, got {other:?}"),
    }
    assert!(
        legacy_path.exists(),
        "legacy file must be preserved on failure for retry"
    );
}

#[test]
fn migrate_legacy_emits_completed_warn_log() {
    let (sub, events) = CapturingSubscriber::new();
    let tmp = TempDir::new().expect("tmp data dir");
    let staged = BaselineState::new();
    staged.observe_span("svc", "op", 0, 100, 1_000);
    stage_legacy_file(tmp.path(), &staged);

    let adapter = make_adapter();
    tracing::subscriber::with_default(sub, || {
        let _ = migrate_legacy_baseline_if_present(tmp.path(), &adapter);
    });

    let captured = events.lock().unwrap();
    let migrate_events: Vec<&CapturedEvent> = captured
        .iter()
        .filter(|(t, _, _)| t == triage::contract::TARGET_BASELINE_MIGRATE)
        .collect();
    assert_eq!(migrate_events.len(), 1, "expected exactly 1 migrate event");
    let (target, level, fields) = migrate_events[0];
    assert_eq!(target, triage::contract::TARGET_BASELINE_MIGRATE);
    assert_eq!(*level, Level::INFO);
    let outcome_field = fields
        .iter()
        .find(|(k, _)| k == "migration_outcome")
        .map(|(_, v)| v.clone());
    assert_eq!(outcome_field, Some("completed".to_string()));
    let banned = [
        "legacy_path",
        "service_name",
        "scope_id",
        "span_id",
        "trace_id",
        "operation_name",
    ];
    for (k, _) in fields {
        assert!(
            !banned.contains(&k.as_str()),
            "field {k:?} forbidden in aggregate-only migrate event"
        );
    }
}

#[test]
fn migrate_legacy_emits_failed_warn_on_corrupt_bytes() {
    let (sub, events) = CapturingSubscriber::new();
    let tmp = TempDir::new().expect("tmp data dir");
    let triage_dir = tmp.path().join(LEGACY_BASELINE_SUBDIR);
    std::fs::create_dir_all(&triage_dir).expect("mkdir triage");
    let legacy_path = triage_dir.join(LEGACY_BASELINE_BASENAME);
    std::fs::write(&legacy_path, b"corrupt").expect("write");

    let adapter = make_adapter();
    tracing::subscriber::with_default(sub, || {
        let _ = migrate_legacy_baseline_if_present(tmp.path(), &adapter);
    });

    let captured = events.lock().unwrap();
    let failed: Vec<&CapturedEvent> = captured
        .iter()
        .filter(|(t, _, _)| t == triage::contract::TARGET_BASELINE_MIGRATE_FAILED)
        .collect();
    assert_eq!(failed.len(), 1);
    let (_, level, fields) = failed[0];
    assert_eq!(*level, Level::WARN);
    let cat = fields
        .iter()
        .find(|(k, _)| k == "error_category")
        .map(|(_, v)| v.clone());
    assert_eq!(cat, Some("deserialize".to_string()));
}

#[test]
fn pii_canary_not_present_in_corpus_db_raw_bytes() {
    // AES-256-GCM cell-level encryption (chunk #68) covers the
    // pipeline_metrics.payload BLOB; raw bincode bytes never appear at-rest.
    // Chunk #72 producer-side scrubber additionally redacts PII-matched
    // service names BEFORE bincode (defense-in-depth). The non-PII canary
    // here exercises only the encryption layer.
    let tmp = TempDir::new().expect("tmp");
    let db_path = tmp.path().join("pii-canary.db");
    let backend_key = [0x21u8; 32];
    let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
        "corpus-key",
        backend_key,
    ));
    let corpus = Corpus::open(db_path.clone(), backend).expect("open");
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let adapter = CorpusBaselinePersistence::new(writer);

    let state = BaselineState::new();
    let canary = "DISTINCTIVE-PII-CANARY-MUST-NOT-LEAK-AT-REST-42";
    state.observe_span(canary, "op-canary", 0, 100, 1_000);
    state.observe_span("svc-normal", canary, 0, 100, 2_000);
    adapter.save(&state).expect("save");

    let raw = std::fs::read(&db_path).expect("read db");
    let pos = raw
        .windows(canary.len())
        .position(|w| w == canary.as_bytes());
    assert!(
        pos.is_none(),
        "canary leaked at-rest at byte offset {pos:?}"
    );
    for pattern in ["span_id", "trace_id", "attribute_value", "operation_name"] {
        assert!(
            !raw.windows(pattern.len()).any(|w| w == pattern.as_bytes()),
            "PII pattern {pattern:?} leaked at-rest"
        );
    }
}
