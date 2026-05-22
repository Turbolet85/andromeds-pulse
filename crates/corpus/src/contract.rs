//! Public corpus contract — chunk #68.
//!
//! Defines the [`Corpus`] handle struct, [`CorpusReader`] trait for
//! cross-crate state delivery into TauRPC resolvers (per session-learnings
//! 2026-05-16 trait-in-lower-crate pattern), [`InspectionMetadata`]
//! envelope, and re-exports of [`Error`] + [`KeychainBackend`] for
//! crate-level convenience.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

pub use crate::error::Error;
pub use crate::keychain::{
    BackendKind, FakeKeychainBackend, KeychainBackend, KeychainError, OsKeychainBackend,
};

use crate::db;
use crate::encryption::{EncryptionKey, cell_decrypt, cell_encrypt};
use crate::schema::{SCHEMA_VERSION, TABLE_NAMES};

/// Per-table record count + on-disk byte size + schema version. Returned
/// by `Corpus::inspect()` + the `storage.inspect` TauRPC procedure.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionMetadata {
    pub record_counts: BTreeMap<String, u64>,
    pub total_bytes_on_disk: u64,
    pub schema_version: u32,
}

/// Raw row envelope for `service_registry` table reads (chunk #71). The
/// lifecycle persistence adapter at `pulse-app/src/lifecycle_persistence.rs`
/// parses TEXT `state` + `manual_override` columns into
/// `ServiceLifecycleState` enum variants at the binary boundary;
/// corpus crate stays domain-agnostic (no `triage` dep edge).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRegistryRowRaw {
    pub service_name: String,
    pub state: String,
    pub first_seen_unix_nano: i64,
    pub last_seen_unix_nano: i64,
    pub last_transition_unix_nano: i64,
    pub manual_override: Option<String>,
}

/// Raw row envelope for `incidents` table reads (chunk #78). The incident
/// persistence adapter at `pulse-app/src/incident_persistence.rs`
/// decrypts the BLOB column + bincode-deserializes into
/// `triage::contract::Incident` at the binary boundary; corpus crate
/// stays domain-agnostic (no `triage` dep edge).
///
/// The `id` field is the SQLite auto-rowid assigned по `INSERT INTO
/// incidents (...)` AND served as the external incident identifier
/// over the TauRPC bridge (incidents.acknowledge / mark_resolved take
/// the rowid string). The `payload` field carries the encrypted-then-
/// decrypted Incident BLOB (AES-256-GCM cell-level encryption per
/// chunk #68 substrate); `payload` is the source-of-truth for fields
/// not captured in the typed metadata columns (`kind`, `scope`, `severity`,
/// `acknowledged_at`, `priority_tier`, etc.).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentRowRaw {
    pub id: i64,
    pub workspace: String,
    pub status: String,
    pub created_unix_nano: i64,
    pub updated_unix_nano: i64,
    pub resolved_unix_nano: Option<i64>,
    pub read_unix_nano: Option<i64>,
    pub payload: Vec<u8>,
}

/// Corpus handle — wraps the rusqlite Connection + tracks the resolved
/// on-disk path + the loaded encryption key. Construct via
/// [`Corpus::open`] (on-disk) or [`Corpus::open_in_memory`] (tests).
pub struct Corpus {
    conn: Arc<Mutex<Connection>>,
    path: PathBuf,
    schema_version: u32,
    #[allow(dead_code)]
    key: EncryptionKey,
}

impl Debug for Corpus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Corpus")
            .field("path_basename", &basename(&self.path))
            .field("schema_version", &self.schema_version)
            .finish()
    }
}

impl Corpus {
    /// Open (or create) the corpus at `path`. Fetches the encryption
    /// key from the keychain backend on first call; subsequent opens с
    /// the same backend reuse the same key. Runs first-launch schema
    /// migration if the file is new.
    pub fn open(path: PathBuf, keychain: Arc<dyn KeychainBackend>) -> Result<Self, Error> {
        let key_bytes = keychain
            .fetch_or_create_key("corpus-key")
            .map_err(|_| Error::KeyringUnavailable)?;
        let key = EncryptionKey::new(key_bytes);
        let conn = db::open_at_path(&path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            path,
            schema_version: SCHEMA_VERSION,
            key,
        })
    }

    /// Open an in-memory corpus с the given keychain backend. Used by
    /// tests + the bindings emission flow.
    pub fn open_in_memory(keychain: Arc<dyn KeychainBackend>) -> Result<Self, Error> {
        let key_bytes = keychain
            .fetch_or_create_key("corpus-key")
            .map_err(|_| Error::KeyringUnavailable)?;
        let key = EncryptionKey::new(key_bytes);
        let conn = db::open_in_memory()?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            path: PathBuf::from(":memory:"),
            schema_version: SCHEMA_VERSION,
            key,
        })
    }

    /// Path returned by [`CorpusReader::path`]. `:memory:` for the
    /// in-memory variant.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Schema version currently applied. Always [`SCHEMA_VERSION`] for
    /// a freshly-opened corpus; surfaces in [`InspectionMetadata`].
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Encryption key accessor for crate-internal write/read paths
    /// (chunk #68 scaffold defers write/read to chunks #70+).
    #[allow(dead_code)]
    pub(crate) fn key(&self) -> &EncryptionKey {
        &self.key
    }

    /// Shared connection handle for crate-internal queries. Not exposed
    /// publicly — callers go through `CorpusReader` instead.
    pub(crate) fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }
}

/// Read-only corpus operations consumed by the TauRPC resolver layer.
/// Trait-in-lower-crate pattern per session-learnings 2026-05-16 —
/// resolvers в `pulse-app` hold `Arc<dyn CorpusReader>`.
pub trait CorpusReader: Send + Sync + Debug {
    /// Per-table record counts + total file size + schema version.
    fn inspect(&self) -> Result<InspectionMetadata, Error>;

    /// Returns the corpus DB filesystem path.
    fn path(&self) -> PathBuf;
}

impl CorpusReader for Corpus {
    fn inspect(&self) -> Result<InspectionMetadata, Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let mut counts = BTreeMap::new();
        for table in TABLE_NAMES {
            let n = db::count_table(&guard, table)?;
            counts.insert((*table).to_string(), n);
        }
        Ok(InspectionMetadata {
            record_counts: counts,
            total_bytes_on_disk: db::file_size_bytes(&self.path),
            schema_version: self.schema_version,
        })
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// Write-side corpus operations for the pipeline-metric persistence
/// surface (chunk #69 Phase B Drain template tree persistence; future
/// chunks #71+ digest archive writes). Parallel trait to [`CorpusReader`]:
/// keeps the read-only-by-design posture of P-051 — write capability is
/// only exposed to consumers that explicitly bind `Arc<dyn CorpusWriter>`.
///
/// Payloads MUST be pre-encrypted-free plaintext at this boundary; the
/// impl wraps each payload в AES-256-GCM before the SQLite INSERT (per
/// chunk #68 cell-level encryption discipline). Caller's plaintext
/// payload MUST already have been PII-scrubbed via
/// `security::scrubber::scrub_attribute` at the producer side (chunk #72);
/// encryption is defense-in-depth for the at-rest threat model, not a
/// substitute for PII scrubbing at the producer (per security plan
/// §Logging NEVER-log discipline + obs-plan §8 default-deny posture).
///
/// The pre-scrub contract is enforced at the producer side via per-adapter
/// PII negative-canary tests covering each call site:
/// - `crates/buffer/src/appender.rs::extract_log_body` (log_records.body
///   write path)
/// - `crates/buffer/src/appender.rs::build_span_events_record_batch`
///   (span_events.exception_message + span_events.exception_stacktrace
///   write paths)
/// - `crates/buffer/src/drain.rs::DrainMiner::snapshot_state` (Drain
///   template tokens; scrubbed upstream of `CorpusDrainPersistence::save`)
/// - `pulse-app/src/lifecycle_persistence.rs::save_all`
///   (service_registry.service_name column)
/// - `pulse-app/src/baseline_persistence.rs::save` (BaselineState
///   per-service map keys + operation_key service prefix)
/// - `pulse-app/src/storm_persistence.rs::save` (StormStateSnapshot
///   FingerprintState.service fields)
///
/// No defensive scrub inside the corpus impl — bincode payload bytes
/// can't be inspected without decoding (layer-separation violation).
/// Per-row typed columns (`service_registry.service_name`) likewise
/// rely on producer-side scrub rather than corpus-side double-scrub.
pub trait CorpusWriter: Send + Sync {
    /// Persist a pipeline-metric snapshot. `metric_name` + `layer`
    /// together identify the metric series (e.g.,
    /// `("drain_template_tree", "l1c")` for chunk #69 Drain persistence).
    /// `payload` is the plaintext-bytes to encrypt + store; the impl
    /// stamps `snapshot_unix_nano` from system time.
    ///
    /// Implementation appends a new row each call; the latest row для
    /// the given `(metric_name, layer)` pair is what
    /// [`Self::load_pipeline_metric`] returns. A bounded-history sweep
    /// is out of scope for chunk #69 (deferred к а follow-on retention
    /// chunk).
    fn save_pipeline_metric(
        &self,
        metric_name: &str,
        layer: &str,
        payload: &[u8],
    ) -> Result<(), Error>;

    /// Load the most recent pipeline-metric snapshot for the given
    /// `(metric_name, layer)` pair. Returns `Ok(None)` when no prior
    /// snapshot exists; `Err` on decryption failure or SQL error.
    fn load_pipeline_metric(
        &self,
        metric_name: &str,
        layer: &str,
    ) -> Result<Option<Vec<u8>>, Error>;

    /// UPSERT a `service_registry` row (chunk #71). Used by the
    /// lifecycle persistence adapter at `pulse-app/src/lifecycle_persistence.rs`
    /// to persist the in-memory DashMap-backed `InMemoryServiceRegistry`
    /// state across restarts (capability P-027 closure). Prepared
    /// statement with `?` placeholders per security plan §Input Validation
    /// + the rusqlite analogue of the 2026 DuckDB CVE cluster.
    ///
    /// Column-level encryption does NOT apply to `service_registry` rows
    /// (unlike `pipeline_metrics.payload` BLOB) because the schema columns
    /// are query-friendly typed columns rather than opaque blobs. The
    /// `service_name` column is user-content classification per arch
    /// §Threat Model; PII scrubber wired at the
    /// `CorpusLifecyclePersistence` producer-side adapter per chunk #72
    /// (corpus impl does not double-scrub — see trait docstring above).
    fn save_service_registry_row(
        &self,
        service_name: &str,
        state: &str,
        first_seen_unix_nano: i64,
        last_seen_unix_nano: i64,
        last_transition_unix_nano: i64,
        manual_override: Option<&str>,
    ) -> Result<(), Error>;

    /// Load all `service_registry` rows ordered by row id (insertion
    /// order — first observation first; mirrors broadcast ordering for
    /// downstream constellation cascade determinism). Returns
    /// `Ok(vec![])` when the table is empty (cold-start path); the
    /// lifecycle adapter caller maps `vec![]` → `Ok(None)` per the
    /// LifecyclePersistence trait semantics.
    fn load_all_service_registry_rows(&self) -> Result<Vec<ServiceRegistryRowRaw>, Error>;

    /// INSERT a new row into `incidents` table (chunk #78). `payload` is
    /// the plaintext-bytes encoding (bincode-serialized
    /// `triage::contract::Incident`) — encrypted via AES-256-GCM по the
    /// cell-level discipline before write. Returns the auto-assigned
    /// SQLite rowid as the new external incident identifier. Prepared
    /// statement с `?` placeholders per security plan §Input Validation.
    /// Workspace + status + timestamp columns store metadata redundantly
    /// for fast SQL filtering (P-045 counter SQL); payload BLOB is the
    /// authoritative source of full struct state.
    ///
    /// Producer-side PII scrubbing rule (chunk #72 uniform coverage):
    /// the caller (incident persistence adapter в pulse-app) MUST have
    /// pre-scrubbed any OTLP-derived attribute values в `incident.title`
    /// / `incident.detail` / `evidence_refs.fingerprint_hashes` BEFORE
    /// passing к this method. Corpus impl does NOT double-scrub the BLOB
    /// payload — see trait docstring above.
    #[allow(clippy::too_many_arguments)]
    fn save_incident(
        &self,
        workspace: &str,
        status: &str,
        created_unix_nano: i64,
        updated_unix_nano: i64,
        resolved_unix_nano: Option<i64>,
        read_unix_nano: Option<i64>,
        payload: &[u8],
    ) -> Result<i64, Error>;

    /// UPDATE an existing `incidents` row's status + timestamps + payload
    /// (chunk #78). Updates the metadata columns + replaces the encrypted
    /// payload BLOB к keep BLOB-state в sync с column-state. Used по
    /// `incidents.acknowledge(id)` + `incidents.mark_resolved(id)` +
    /// auto-resolution observer tick. Returns `Error::QueryFailed` when
    /// `id` does not match а row (caller maps к `IncidentError::NotFound`
    /// or `AppError::NotFound` at the binary boundary).
    fn update_incident_status(
        &self,
        id: i64,
        status: &str,
        updated_unix_nano: i64,
        resolved_unix_nano: Option<i64>,
        payload: &[u8],
    ) -> Result<(), Error>;

    /// UPDATE only the `read_unix_nano` column for an incident (chunk #78).
    /// Used по Report-opening event (chunk #87+ wires the UI trigger;
    /// chunk #78 ships the schema + write path).
    fn mark_incident_read(&self, id: i64, read_unix_nano: i64) -> Result<(), Error>;

    /// SELECT all active (non-Resolved) incidents для а workspace, ordered
    /// по rowid ascending (creation order). Returns decrypted `payload`
    /// bytes per row. Empty Vec when the workspace has no active
    /// incidents; caller hydrates the in-memory registry from this set
    /// at boot.
    fn load_active_incidents(&self, workspace: &str) -> Result<Vec<IncidentRowRaw>, Error>;

    /// P-045 counter SQL: returns the count of active + unread incidents
    /// для а workspace (`status = 'active' AND read_unix_nano IS NULL`).
    /// SQL-only path; does NOT decrypt payloads. Fast counter для
    /// findings dropdown display.
    fn count_active_unread(&self, workspace: &str) -> Result<u64, Error>;

    /// INSERT а row into `incident_events` table (chunk #78). Audit-trail
    /// lifecycle events; `payload` is the encrypted bincode of event-
    /// specific metadata (currently empty Vec is acceptable; chunk #78+
    /// may extend per-event payload shape).
    fn save_incident_event(
        &self,
        incident_id: i64,
        event_kind: &str,
        occurred_unix_nano: i64,
        payload: &[u8],
    ) -> Result<(), Error>;
}

impl CorpusWriter for Corpus {
    fn save_pipeline_metric(
        &self,
        metric_name: &str,
        layer: &str,
        payload: &[u8],
    ) -> Result<(), Error> {
        let encrypted = cell_encrypt(self.key(), payload)?;
        let snapshot_unix_nano = current_unix_nanos();
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        guard
            .execute(
                "INSERT INTO pipeline_metrics (metric_name, layer, snapshot_unix_nano, payload) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![metric_name, layer, snapshot_unix_nano, &encrypted[..]],
            )
            .map_err(|_| Error::QueryFailed)?;
        Ok(())
    }

    fn load_pipeline_metric(
        &self,
        metric_name: &str,
        layer: &str,
    ) -> Result<Option<Vec<u8>>, Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let mut stmt = guard
            .prepare(
                "SELECT payload FROM pipeline_metrics WHERE metric_name = ?1 AND layer = ?2 ORDER BY snapshot_unix_nano DESC LIMIT 1",
            )
            .map_err(|_| Error::QueryFailed)?;
        let encrypted_opt: Option<Vec<u8>> = stmt
            .query_row(rusqlite::params![metric_name, layer], |row| row.get(0))
            .optional()
            .map_err(|_| Error::QueryFailed)?;
        match encrypted_opt {
            Some(encrypted) => {
                let plaintext = cell_decrypt(self.key(), &encrypted)?;
                Ok(Some(plaintext))
            }
            None => Ok(None),
        }
    }

    fn save_service_registry_row(
        &self,
        service_name: &str,
        state: &str,
        first_seen_unix_nano: i64,
        last_seen_unix_nano: i64,
        last_transition_unix_nano: i64,
        manual_override: Option<&str>,
    ) -> Result<(), Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        guard
            .execute(
                "INSERT INTO service_registry (service_name, state, first_seen_unix_nano, last_seen_unix_nano, last_transition_unix_nano, manual_override) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6) \
                 ON CONFLICT(service_name) DO UPDATE SET \
                   state = excluded.state, \
                   last_seen_unix_nano = excluded.last_seen_unix_nano, \
                   last_transition_unix_nano = excluded.last_transition_unix_nano, \
                   manual_override = excluded.manual_override",
                rusqlite::params![
                    service_name,
                    state,
                    first_seen_unix_nano,
                    last_seen_unix_nano,
                    last_transition_unix_nano,
                    manual_override,
                ],
            )
            .map_err(|_| Error::QueryFailed)?;
        Ok(())
    }

    fn load_all_service_registry_rows(&self) -> Result<Vec<ServiceRegistryRowRaw>, Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let mut stmt = guard
            .prepare(
                "SELECT service_name, state, first_seen_unix_nano, last_seen_unix_nano, last_transition_unix_nano, manual_override \
                 FROM service_registry ORDER BY id",
            )
            .map_err(|_| Error::QueryFailed)?;
        let rows = stmt
            .query_map([], |row| {
                Ok(ServiceRegistryRowRaw {
                    service_name: row.get(0)?,
                    state: row.get(1)?,
                    first_seen_unix_nano: row.get(2)?,
                    last_seen_unix_nano: row.get(3)?,
                    last_transition_unix_nano: row.get(4)?,
                    manual_override: row.get(5)?,
                })
            })
            .map_err(|_| Error::QueryFailed)?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|_| Error::QueryFailed)?);
        }
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn save_incident(
        &self,
        workspace: &str,
        status: &str,
        created_unix_nano: i64,
        updated_unix_nano: i64,
        resolved_unix_nano: Option<i64>,
        read_unix_nano: Option<i64>,
        payload: &[u8],
    ) -> Result<i64, Error> {
        let encrypted = cell_encrypt(self.key(), payload)?;
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        guard
            .execute(
                "INSERT INTO incidents (workspace, status, created_unix_nano, updated_unix_nano, resolved_unix_nano, read_unix_nano, payload) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![
                    workspace,
                    status,
                    created_unix_nano,
                    updated_unix_nano,
                    resolved_unix_nano,
                    read_unix_nano,
                    &encrypted[..],
                ],
            )
            .map_err(|_| Error::QueryFailed)?;
        Ok(guard.last_insert_rowid())
    }

    fn update_incident_status(
        &self,
        id: i64,
        status: &str,
        updated_unix_nano: i64,
        resolved_unix_nano: Option<i64>,
        payload: &[u8],
    ) -> Result<(), Error> {
        let encrypted = cell_encrypt(self.key(), payload)?;
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let rows = guard
            .execute(
                "UPDATE incidents SET status = ?1, updated_unix_nano = ?2, resolved_unix_nano = ?3, payload = ?4 WHERE id = ?5",
                rusqlite::params![status, updated_unix_nano, resolved_unix_nano, &encrypted[..], id],
            )
            .map_err(|_| Error::QueryFailed)?;
        if rows == 0 {
            return Err(Error::QueryFailed);
        }
        Ok(())
    }

    fn mark_incident_read(&self, id: i64, read_unix_nano: i64) -> Result<(), Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let rows = guard
            .execute(
                "UPDATE incidents SET read_unix_nano = ?1 WHERE id = ?2",
                rusqlite::params![read_unix_nano, id],
            )
            .map_err(|_| Error::QueryFailed)?;
        if rows == 0 {
            return Err(Error::QueryFailed);
        }
        Ok(())
    }

    fn load_active_incidents(&self, workspace: &str) -> Result<Vec<IncidentRowRaw>, Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let mut stmt = guard
            .prepare(
                "SELECT id, workspace, status, created_unix_nano, updated_unix_nano, resolved_unix_nano, read_unix_nano, payload \
                 FROM incidents WHERE workspace = ?1 AND status != 'resolved' ORDER BY id",
            )
            .map_err(|_| Error::QueryFailed)?;
        let rows = stmt
            .query_map(rusqlite::params![workspace], |row| {
                let encrypted: Vec<u8> = row.get(7)?;
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, Option<i64>>(5)?,
                    row.get::<_, Option<i64>>(6)?,
                    encrypted,
                ))
            })
            .map_err(|_| Error::QueryFailed)?;
        let mut result = Vec::new();
        for row in rows {
            let (id, workspace, status, created, updated, resolved, read, encrypted) =
                row.map_err(|_| Error::QueryFailed)?;
            let payload = cell_decrypt(self.key(), &encrypted)?;
            result.push(IncidentRowRaw {
                id,
                workspace,
                status,
                created_unix_nano: created,
                updated_unix_nano: updated,
                resolved_unix_nano: resolved,
                read_unix_nano: read,
                payload,
            });
        }
        Ok(result)
    }

    fn count_active_unread(&self, workspace: &str) -> Result<u64, Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let n: i64 = guard
            .query_row(
                "SELECT COUNT(*) FROM incidents WHERE workspace = ?1 AND read_unix_nano IS NULL AND status = 'active'",
                rusqlite::params![workspace],
                |row| row.get(0),
            )
            .map_err(|_| Error::QueryFailed)?;
        Ok(n.max(0) as u64)
    }

    fn save_incident_event(
        &self,
        incident_id: i64,
        event_kind: &str,
        occurred_unix_nano: i64,
        payload: &[u8],
    ) -> Result<(), Error> {
        let encrypted = cell_encrypt(self.key(), payload)?;
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        guard
            .execute(
                "INSERT INTO incident_events (incident_id, event_kind, occurred_unix_nano, payload) \
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![incident_id, event_kind, occurred_unix_nano, &encrypted[..]],
            )
            .map_err(|_| Error::QueryFailed)?;
        Ok(())
    }
}

fn current_unix_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

fn basename(path: &Path) -> String {
    path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(":memory:")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_corpus() -> Corpus {
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
        Corpus::open_in_memory(backend).expect("in-memory corpus")
    }

    #[test]
    fn open_in_memory_returns_corpus_with_default_schema_version() {
        let corpus = test_corpus();
        assert_eq!(corpus.schema_version(), SCHEMA_VERSION);
        assert_eq!(corpus.path(), Path::new(":memory:"));
    }

    #[test]
    fn inspect_returns_zero_counts_for_fresh_corpus() {
        let corpus = test_corpus();
        let meta = corpus.inspect().expect("inspect");
        assert_eq!(meta.schema_version, SCHEMA_VERSION);
        for table in TABLE_NAMES {
            assert_eq!(meta.record_counts.get(*table), Some(&0));
        }
    }

    #[test]
    fn inspect_includes_all_six_tables() {
        let corpus = test_corpus();
        let meta = corpus.inspect().expect("inspect");
        assert_eq!(meta.record_counts.len(), TABLE_NAMES.len());
        for table in TABLE_NAMES {
            assert!(meta.record_counts.contains_key(*table));
        }
    }

    #[test]
    fn inspect_reflects_inserted_rows() {
        let corpus = test_corpus();
        {
            let conn = corpus.connection();
            let guard = conn.lock().expect("lock");
            guard
                .execute(
                    "INSERT INTO incidents (workspace, status, created_unix_nano, updated_unix_nano, payload) VALUES (?, ?, ?, ?, ?)",
                    rusqlite::params!["ws", "active", 1_000i64, 1_000i64, &b"x"[..]],
                )
                .expect("insert");
        }
        let meta = corpus.inspect().expect("inspect");
        assert_eq!(meta.record_counts.get("incidents"), Some(&1));
    }

    #[test]
    fn open_on_disk_persists_across_close_and_reopen() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("persist-test.db");
        let backend_key = [0x77u8; 32];
        {
            let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
                "corpus-key",
                backend_key,
            ));
            let corpus = Corpus::open(path.clone(), backend).expect("open");
            let conn = corpus.connection();
            let guard = conn.lock().expect("lock");
            guard
                .execute(
                    "INSERT INTO incidents (workspace, status, created_unix_nano, updated_unix_nano, payload) VALUES (?, ?, ?, ?, ?)",
                    rusqlite::params!["ws-a", "active", 2_000i64, 2_000i64, &b"x"[..]],
                )
                .expect("insert");
        }
        // Reopen with the SAME seeded key → row persists.
        let backend2: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
            "corpus-key",
            backend_key,
        ));
        let corpus2 = Corpus::open(path.clone(), backend2).expect("reopen");
        let meta = corpus2.inspect().expect("inspect");
        assert_eq!(meta.record_counts.get("incidents"), Some(&1));
        assert!(meta.total_bytes_on_disk > 0);
    }

    #[test]
    fn debug_does_not_leak_full_path() {
        let tmp = TempDir::new().expect("tmp");
        let secret_segment = "extremely-distinctive-segment-do-not-leak";
        let path = tmp.path().join(secret_segment).with_extension("db");
        let parent = path.parent().expect("parent");
        std::fs::create_dir_all(parent).expect("mkdir");
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
        let corpus = Corpus::open(path.clone(), backend).expect("open");
        let dbg = format!("{corpus:?}");
        // Full canonicalized path MUST NOT appear; only basename (which
        // contains the segment) is acceptable. Verify path-parent
        // components don't leak.
        if let Some(parent_str) = parent.to_str() {
            assert!(
                !dbg.contains(parent_str),
                "Debug output leaked parent path: {dbg}"
            );
        }
    }

    #[test]
    fn corpus_reader_trait_dispatch_works() {
        let corpus = test_corpus();
        let reader: Arc<dyn CorpusReader> = Arc::new(corpus);
        let meta = reader.inspect().expect("inspect via trait");
        assert_eq!(meta.schema_version, SCHEMA_VERSION);
    }

    // ===== CorpusWriter trait tests (chunk #69 Phase B Session 4) =====

    #[test]
    fn corpus_writer_save_then_load_round_trips_plaintext_bytes() {
        let corpus = test_corpus();
        let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
        let payload = b"drain-state-bincode-bytes-go-here-but-this-is-a-fixture";
        writer
            .save_pipeline_metric("drain_template_tree", "l1c", payload)
            .expect("save");
        let loaded = writer
            .load_pipeline_metric("drain_template_tree", "l1c")
            .expect("load")
            .expect("Some(bytes)");
        assert_eq!(loaded, payload);
    }

    #[test]
    fn corpus_writer_load_returns_none_when_no_prior_save() {
        let corpus = test_corpus();
        let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
        let loaded = writer
            .load_pipeline_metric("never_saved", "l1c")
            .expect("load");
        assert!(loaded.is_none());
    }

    #[test]
    fn corpus_writer_load_returns_latest_snapshot_when_multiple_saved() {
        let corpus = test_corpus();
        let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
        writer
            .save_pipeline_metric("drain_template_tree", "l1c", b"v1-older")
            .expect("save v1");
        // Small sleep to ensure distinct snapshot_unix_nano values; nanosecond
        // precision is generally enough but be explicit.
        std::thread::sleep(std::time::Duration::from_millis(2));
        writer
            .save_pipeline_metric("drain_template_tree", "l1c", b"v2-newer")
            .expect("save v2");
        let loaded = writer
            .load_pipeline_metric("drain_template_tree", "l1c")
            .expect("load")
            .expect("Some(bytes)");
        assert_eq!(loaded, b"v2-newer");
    }

    #[test]
    fn corpus_writer_distinct_layer_or_name_persists_separately() {
        let corpus = test_corpus();
        let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
        writer
            .save_pipeline_metric("drain_template_tree", "l1c", b"drain-payload")
            .expect("save drain");
        writer
            .save_pipeline_metric("baseline_state", "l1b", b"baseline-payload")
            .expect("save baseline");
        let drain_loaded = writer
            .load_pipeline_metric("drain_template_tree", "l1c")
            .expect("load drain")
            .expect("Some");
        let baseline_loaded = writer
            .load_pipeline_metric("baseline_state", "l1b")
            .expect("load baseline")
            .expect("Some");
        assert_eq!(drain_loaded, b"drain-payload");
        assert_eq!(baseline_loaded, b"baseline-payload");
    }

    #[test]
    fn corpus_writer_empty_payload_round_trips() {
        let corpus = test_corpus();
        let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
        writer
            .save_pipeline_metric("empty_metric", "l0", b"")
            .expect("save empty");
        let loaded = writer
            .load_pipeline_metric("empty_metric", "l0")
            .expect("load empty")
            .expect("Some");
        assert!(loaded.is_empty());
    }

    #[test]
    fn corpus_writer_persists_across_corpus_reopen() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("writer-persist-test.db");
        let backend_key = [0x5Au8; 32];
        {
            let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
                "corpus-key",
                backend_key,
            ));
            let corpus = Corpus::open(path.clone(), backend).expect("open");
            let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
            writer
                .save_pipeline_metric(
                    "drain_template_tree",
                    "l1c",
                    b"persisted-across-reopen-payload",
                )
                .expect("save");
        }
        let backend2: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
            "corpus-key",
            backend_key,
        ));
        let corpus2 = Corpus::open(path.clone(), backend2).expect("reopen");
        let writer2: Arc<dyn CorpusWriter> = Arc::new(corpus2);
        let loaded = writer2
            .load_pipeline_metric("drain_template_tree", "l1c")
            .expect("load")
            .expect("Some after reopen");
        assert_eq!(loaded, b"persisted-across-reopen-payload");
    }

    #[test]
    fn corpus_writer_save_encrypts_payload_on_disk() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("encryption-canary.db");
        let backend_key = [0x33u8; 32];
        let canary = b"DISTINCTIVE-PLAINTEXT-CANARY-MUST-NOT-APPEAR-RAW";
        {
            let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
                "corpus-key",
                backend_key,
            ));
            let corpus = Corpus::open(path.clone(), backend).expect("open");
            let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
            writer
                .save_pipeline_metric("canary_metric", "l1c", canary)
                .expect("save");
        }
        // Read raw file bytes; assert the canary plaintext is NOT present
        // (encryption discipline). This is the at-rest defense test
        // mirroring chunk #68 encryption.rs round-trip discipline.
        let raw = std::fs::read(&path).expect("read raw db");
        let canary_position = raw
            .windows(canary.len())
            .position(|window| window == canary);
        assert!(
            canary_position.is_none(),
            "plaintext canary leaked at-rest at byte offset {canary_position:?}"
        );
    }

    #[test]
    fn corpus_writer_trait_dispatch_works() {
        let corpus = test_corpus();
        let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
        writer
            .save_pipeline_metric("dispatch_test", "l1c", b"x")
            .expect("save via trait");
        let loaded = writer
            .load_pipeline_metric("dispatch_test", "l1c")
            .expect("load via trait");
        assert!(loaded.is_some());
    }
}
