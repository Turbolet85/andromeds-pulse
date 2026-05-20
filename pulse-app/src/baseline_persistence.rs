//! Corpus-backed BaselineState persistence adapter — chunk #70.
//!
//! Wires `triage::contract::BaselinePersistence` to
//! `corpus::contract::CorpusWriter` at the pulse-app binary boundary,
//! preserving the arch §Module dependency direction DAG (triage stays
//! corpus-free; corpus stays triage-free; pulse-app owns the wire-up).
//! Mirrors chunk #69's `CorpusDrainPersistence` precedent + chunk #59
//! `HeartbeatBindStatus` trait-in-lower-crate + adapter-in-pulse-app
//! pattern (per session-learnings 2026-05-16 + 2026-05-19).
//!
//! Persistence cell shape per chunk #70 schema decision Option A
//! (mirrors chunk #69): re-uses the existing `pipeline_metrics` SQLite
//! table (chunk #68 schema, no version bump) with
//! `metric_name = "baseline_state"`, `layer = "l1b"`, `payload =
//! bincode-serialized BaselineState`. Cell-level AES-256-GCM encryption
//! is applied by the corpus crate transparently to this adapter;
//! serialization → encryption → INSERT is the save path; SELECT →
//! decryption → deserialization is the load path.
//!
//! Legacy bincode migration: at boot, `migrate_legacy_baseline_if_present`
//! checks for `<data_dir>/triage/baseline-corpus.bin` (chunk #61
//! substrate). If present + valid, the contents are read once,
//! re-persisted through the trait, and the legacy file is deleted.
//! Failures preserve the legacy file for retry on next boot.

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use corpus::contract::{CorpusWriter, Error as CorpusError};
use triage::contract::{BaselineError, BaselinePersistence, BaselineState, DEFAULT_MAX_SIZE_BYTES};

/// Stable `metric_name` value in `pipeline_metrics` for BaselineState.
/// Changing this string breaks in-place persistence reads against
/// existing on-disk corpora — schema-equivalent renaming would require
/// a migration path.
pub const BASELINE_STATE_METRIC_NAME: &str = "baseline_state";

/// Stable `layer` tag in `pipeline_metrics`. Per pulse v0.2.0
/// distillation architecture, `l1b` is the statistical-baseline layer.
pub const BASELINE_PERSISTENCE_LAYER: &str = "l1b";

/// Legacy bincode subpath under the resolved data dir (chunk #61
/// substrate; eliminated by chunk #70). Migration helper looks for this
/// file at boot.
const LEGACY_BASELINE_BASENAME: &str = "baseline-corpus.bin";
const LEGACY_BASELINE_SUBDIR: &str = "triage";

/// Adapter implementing `triage::BaselinePersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusBaselinePersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusBaselinePersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl BaselinePersistence for CorpusBaselinePersistence {
    fn load(&self) -> Result<Option<BaselineState>, BaselineError> {
        let bytes_opt = self
            .writer
            .load_pipeline_metric(BASELINE_STATE_METRIC_NAME, BASELINE_PERSISTENCE_LAYER)
            .map_err(corpus_error_to_baseline_error)?;
        match bytes_opt {
            Some(bytes) => {
                let state: BaselineState =
                    bincode::deserialize(&bytes).map_err(|_| BaselineError::Deserialize)?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    fn save(&self, state: &BaselineState) -> Result<(), BaselineError> {
        let bytes = bincode::serialize(state).map_err(|_| BaselineError::Serialize)?;
        self.writer
            .save_pipeline_metric(
                BASELINE_STATE_METRIC_NAME,
                BASELINE_PERSISTENCE_LAYER,
                &bytes,
            )
            .map_err(corpus_error_to_baseline_error)
    }
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the BaselineError surfaced upward. Mirrors
/// chunk #69 `corpus_error_to_buffer_error` precedent.
///
/// Free function (not `From` impl) — orphan rule forbids
/// `impl From<corpus::Error> for triage::BaselineError` here (both types
/// foreign to pulse-app). Per session-learnings 2026-05-18.
fn corpus_error_to_baseline_error(err: CorpusError) -> BaselineError {
    use triage::contract::SCHEMA_VERSION;
    match err {
        CorpusError::KeyringUnavailable => BaselineError::Io {
            kind: std::io::ErrorKind::PermissionDenied,
        },
        CorpusError::MigrationFailed => BaselineError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::EncryptionFailed => BaselineError::Serialize,
        CorpusError::DecryptionFailed => BaselineError::Deserialize,
        CorpusError::QueryFailed => BaselineError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::PathTraversal => BaselineError::PathTraversal,
        CorpusError::SizeCapExceeded => BaselineError::SizeCapExceeded { actual: 0, max: 0 },
        CorpusError::SchemaVersionMismatch => BaselineError::SchemaVersionMismatch {
            expected: SCHEMA_VERSION,
            got: 0,
        },
        CorpusError::Io { kind } => BaselineError::Io { kind },
    }
}

/// Outcome of a legacy-baseline migration attempt. Returned by
/// `migrate_legacy_baseline_if_present`; tracing events for each outcome
/// fire inside that function before return.
#[derive(Debug, PartialEq, Eq)]
pub enum MigrationOutcome {
    /// Legacy file not present — no-op (cold-start path).
    Noop,
    /// Legacy file read + re-persisted through trait + cleanup attempted.
    Completed {
        legacy_file_deleted: bool,
        bytes: u64,
        service_count: u64,
    },
    /// Migration failed at the indicated stage. Legacy file is preserved
    /// for retry on next boot (no auto-delete on partial migration).
    Failed { error_category: &'static str },
}

/// Attempt one-shot migration of the legacy `<data_dir>/triage/
/// baseline-corpus.bin` flat-file (chunk #61 substrate) into the corpus
/// SQLite-backed `BaselinePersistence`. Idempotent: once the legacy file
/// is deleted (post-completion), subsequent boots return `Noop`.
///
/// PII discipline: emitted tracing events carry aggregate-only fields per
/// `.claude/rules/observability.md` Session Addition 2026-05-17 session
/// 84 — `legacy_state_size_bytes` / `migrated_service_count` /
/// `duration_ms` / `migration_outcome` enum. No raw path, no service
/// names, no error chain.
pub fn migrate_legacy_baseline_if_present(
    data_dir: &Path,
    persistence: &dyn BaselinePersistence,
) -> MigrationOutcome {
    let triage_dir = data_dir.join(LEGACY_BASELINE_SUBDIR);
    let legacy_path = triage_dir.join(LEGACY_BASELINE_BASENAME);

    if !legacy_path.exists() {
        return MigrationOutcome::Noop;
    }

    let start = Instant::now();
    let outcome = migrate_legacy_inner(&legacy_path, persistence);
    let duration_ms = start.elapsed().as_millis() as u64;

    match &outcome {
        MigrationOutcome::Noop => {}
        MigrationOutcome::Completed {
            legacy_file_deleted,
            bytes,
            service_count,
        } => {
            let outcome_tag = if *legacy_file_deleted {
                "completed"
            } else {
                "completed_legacy_kept"
            };
            tracing::info!(
                target: triage::contract::TARGET_BASELINE_MIGRATE,
                legacy_state_size_bytes = *bytes,
                migrated_service_count = *service_count,
                duration_ms = duration_ms,
                migration_outcome = outcome_tag,
                "migrated legacy baseline-corpus.bin to corpus",
            );
        }
        MigrationOutcome::Failed { error_category } => {
            tracing::warn!(
                target: triage::contract::TARGET_BASELINE_MIGRATE_FAILED,
                error_category = *error_category,
                duration_ms = duration_ms,
                "legacy baseline-corpus.bin migration failed; preserving legacy file for retry",
            );
        }
    }

    outcome
}

fn migrate_legacy_inner(
    legacy_path: &Path,
    persistence: &dyn BaselinePersistence,
) -> MigrationOutcome {
    // Canonicalize to defeat symlink-based path-traversal attempts. We
    // do NOT assert resolved-path-under-data-dir here because the data
    // dir itself was already canonicalized at the boot site; a symlink
    // inside the triage subdir pointing outside the data root would
    // surface as a canonicalize error against the symlink target.
    let canonical = match std::fs::canonicalize(legacy_path) {
        Ok(p) => p,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "io",
            };
        }
    };

    let metadata = match std::fs::metadata(&canonical) {
        Ok(m) => m,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "io",
            };
        }
    };

    if metadata.len() > DEFAULT_MAX_SIZE_BYTES {
        return MigrationOutcome::Failed {
            error_category: "size_exceeded",
        };
    }

    let bytes = match std::fs::read(&canonical) {
        Ok(b) => b,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "io",
            };
        }
    };

    let state: BaselineState = match bincode::deserialize(&bytes) {
        Ok(s) => s,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "deserialize",
            };
        }
    };

    let service_count = state.service_count() as u64;
    let bytes_len = bytes.len() as u64;

    if let Err(e) = persistence.save(&state) {
        return MigrationOutcome::Failed {
            error_category: e.error_category(),
        };
    }

    let legacy_file_deleted = std::fs::remove_file(&canonical).is_ok();

    MigrationOutcome::Completed {
        legacy_file_deleted,
        bytes: bytes_len,
        service_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus::contract::{Corpus, FakeKeychainBackend, KeychainBackend};
    use std::sync::Mutex;
    use tempfile::TempDir;
    use tracing::field::{Field, Visit};
    use tracing::{Event, Level, Subscriber};

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

        // Fresh load via the same adapter → restores state.
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

        // Reading via the CorpusWriter directly with the same
        // metric_name + layer should yield bytes.
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
        // PII discipline: no legacy path / service_name / per-record content.
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
        // Open on-disk corpus (not in-memory) so we can read raw bytes
        // back from the SQLite file post-save + verify AES encryption hid
        // the seeded canary substring.
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
        // Seed canary in BOTH service name + operation name positions
        // to exercise multiple field paths.
        let canary = "DISTINCTIVE-PII-CANARY-MUST-NOT-LEAK-AT-REST-42";
        state.observe_span(canary, "op-canary", 0, 100, 1_000);
        state.observe_span("svc-normal", canary, 0, 100, 2_000);
        adapter.save(&state).expect("save");

        let raw = std::fs::read(&db_path).expect("read db");
        // AES-256-GCM encrypts the bincode payload before SQLite INSERT.
        // The raw .db bytes MUST NOT contain the canary plaintext.
        let pos = raw
            .windows(canary.len())
            .position(|w| w == canary.as_bytes());
        assert!(
            pos.is_none(),
            "canary leaked at-rest at byte offset {pos:?}"
        );
        // Same for the OTLP-derived attribute names that the chunk #61
        // PII canary test guarded.
        for pattern in ["span_id", "trace_id", "attribute_value", "operation_name"] {
            assert!(
                !raw.windows(pattern.len()).any(|w| w == pattern.as_bytes()),
                "PII pattern {pattern:?} leaked at-rest"
            );
        }
    }
}
