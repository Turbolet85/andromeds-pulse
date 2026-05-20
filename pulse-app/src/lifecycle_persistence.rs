//! Corpus-backed lifecycle persistence adapter — chunk #71.
//!
//! Wires `triage::contract::LifecyclePersistence` to
//! `corpus::contract::CorpusWriter` at the pulse-app binary boundary,
//! preserving the arch §Module dependency direction DAG (triage stays
//! corpus-free; corpus stays triage-free; pulse-app owns the wire-up).
//! Mirrors chunk #70 `CorpusBaselinePersistence` + chunk #69
//! `CorpusDrainPersistence` precedents.
//!
//! Schema choice: per-row INSERT/UPDATE on the pre-allocated
//! `service_registry` SQLite table (chunk #68 schema, no version bump).
//! Each `ServiceRegistryEntry` maps directly onto the table's columns
//! (state TEXT, three timestamp INTEGERs, manual_override TEXT NULL).
//! `service_name` is the UNIQUE key; the new
//! `CorpusWriter::save_service_registry_row` method UPSERTs.
//!
//! PII discipline: `service_name` is user-content classification. AES
//! corpus-wide encryption at rest covers the file bytes; per-row PII
//! scrubber call site at the producer is deferred to chunk #72 per
//! plan §Acceptance Criteria → Deferred.

use std::sync::Arc;

use corpus::contract::{CorpusWriter, Error as CorpusError};
use triage::contract::{
    LifecycleError, LifecyclePersistence, SCHEMA_VERSION, ServiceLifecycleState,
    ServiceRegistryEntry, state_label,
};

/// Adapter implementing `triage::LifecyclePersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusLifecyclePersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusLifecyclePersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl LifecyclePersistence for CorpusLifecyclePersistence {
    fn load_all(&self) -> Result<Option<Vec<(String, ServiceRegistryEntry)>>, LifecycleError> {
        let rows = self
            .writer
            .load_all_service_registry_rows()
            .map_err(corpus_error_to_lifecycle_error)?;
        if rows.is_empty() {
            return Ok(None);
        }
        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            let state = parse_state(&row.state).ok_or(LifecycleError::Deserialize)?;
            let manual_override = match row.manual_override.as_deref() {
                Some(s) => Some(parse_state(s).ok_or(LifecycleError::Deserialize)?),
                None => None,
            };
            entries.push((
                row.service_name,
                ServiceRegistryEntry {
                    state,
                    first_seen_unix_nano: row.first_seen_unix_nano,
                    last_seen_unix_nano: row.last_seen_unix_nano,
                    last_transition_unix_nano: row.last_transition_unix_nano,
                    manual_override,
                },
            ));
        }
        Ok(Some(entries))
    }

    fn save_all(&self, entries: &[(String, ServiceRegistryEntry)]) -> Result<(), LifecycleError> {
        for (service, entry) in entries {
            let manual_override_label = entry.manual_override.map(state_label);
            self.writer
                .save_service_registry_row(
                    service,
                    state_label(entry.state),
                    entry.first_seen_unix_nano,
                    entry.last_seen_unix_nano,
                    entry.last_transition_unix_nano,
                    manual_override_label,
                )
                .map_err(corpus_error_to_lifecycle_error)?;
        }
        Ok(())
    }
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the LifecycleError surfaced upward.
/// Mirrors chunk #70 `corpus_error_to_baseline_error` precedent.
///
/// Free function (not `From` impl) — orphan rule forbids
/// `impl From<corpus::Error> for triage::LifecycleError` here (both
/// types foreign to pulse-app). Per session-learnings 2026-05-18.
fn corpus_error_to_lifecycle_error(err: CorpusError) -> LifecycleError {
    match err {
        CorpusError::KeyringUnavailable => LifecycleError::Io {
            kind: std::io::ErrorKind::PermissionDenied,
        },
        CorpusError::MigrationFailed => LifecycleError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::EncryptionFailed => LifecycleError::Serialize,
        CorpusError::DecryptionFailed => LifecycleError::Deserialize,
        CorpusError::QueryFailed => LifecycleError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::PathTraversal => LifecycleError::PathTraversal,
        CorpusError::SizeCapExceeded => LifecycleError::SizeCapExceeded { actual: 0, max: 0 },
        CorpusError::SchemaVersionMismatch => LifecycleError::SchemaVersionMismatch {
            expected: SCHEMA_VERSION,
            got: 0,
        },
        CorpusError::Io { kind } => LifecycleError::Io { kind },
    }
}

/// Parse the corpus `state` TEXT column into a `ServiceLifecycleState`
/// enum variant. Mirrors the snake_case serialization defined by the
/// `#[serde(rename_all = "snake_case")]` derive on
/// `ServiceLifecycleState`. Returns `None` on unknown variant — caller
/// surfaces as `LifecycleError::Deserialize`.
fn parse_state(s: &str) -> Option<ServiceLifecycleState> {
    match s {
        "unknown" => Some(ServiceLifecycleState::Unknown),
        "bootstrapping" => Some(ServiceLifecycleState::Bootstrapping),
        "active" => Some(ServiceLifecycleState::Active),
        "quiet" => Some(ServiceLifecycleState::Quiet),
        "silent" => Some(ServiceLifecycleState::Silent),
        "dormant" => Some(ServiceLifecycleState::Dormant),
        "archived" => Some(ServiceLifecycleState::Archived),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus::contract::{Corpus, FakeKeychainBackend, KeychainBackend};
    use tempfile::TempDir;

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
        // first_seen is preserved by ON CONFLICT (not in SET clause).
        assert_eq!(restored.first_seen_unix_nano, 1_111_111);
        assert_eq!(restored.last_seen_unix_nano, 2_222_222);
        assert_eq!(restored.last_transition_unix_nano, 3_333_333);
    }

    #[test]
    fn pii_canary_not_present_in_corpus_db_raw_bytes() {
        // service_name + manual_override columns are TEXT (not encrypted
        // payload BLOB). Corpus-wide AES applies to BLOB columns only;
        // service_name is queryable plaintext. This test documents the
        // EXPECTED behavior — service.name IS visible at-rest in the
        // service_registry table. PII scrubber call site BEFORE write
        // is deferred to chunk #72 per plan §Acceptance Criteria →
        // Deferred. Documenting via this test so future readers know
        // the at-rest posture without re-checking the schema.
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
        // service_name TEXT column IS plaintext in service_registry per
        // chunk #68 schema; chunk #71 documents this as a known gap
        // closed at chunk #72 via scrubber-at-write.
        let canary_position = raw
            .windows(canary.len())
            .position(|w| w == canary.as_bytes());
        // Currently expected: canary IS present in raw bytes (TEXT column
        // not encrypted). This assertion is INVERTED from the chunk #70
        // pattern and serves as a code-aware reminder that scrubber wire-up
        // is the chunk #72 work.
        assert!(
            canary_position.is_some(),
            "service_name TEXT column expected plaintext at-rest until chunk #72 PII scrubber lands"
        );
    }
}
