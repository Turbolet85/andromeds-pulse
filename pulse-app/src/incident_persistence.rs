//! Corpus-backed incident persistence adapter — chunk #78.
//!
//! Wires `triage::contract::IncidentPersistence` to
//! `corpus::contract::CorpusWriter` at the pulse-app binary boundary,
//! preserving the arch §Module dependency direction DAG (triage stays
//! corpus-free; corpus stays triage-free; pulse-app owns the wire-up).
//! Mirrors chunk #70 `CorpusBaselinePersistence` + chunk #71
//! `CorpusLifecyclePersistence` + chunk #71 `CorpusStormPersistence`
//! precedents.
//!
//! Schema choice: per-row INSERT/UPDATE on the pre-allocated `incidents`
//! SQLite table (chunk #68 schema, no version bump). Payload BLOB carries
//! the bincode-encoded `Incident` struct (encrypted via AES-256-GCM at
//! the corpus cell layer); metadata columns duplicate workspace + status
//! + timestamps for fast SQL filtering (P-045 counter SQL).
//!
//! PII discipline (chunk #72 uniform coverage): the producer side (cue
//! emitter at chunk #62 + future interpretation layer) is responsible
//! for pre-scrubbing OTLP-derived attribute values in `Incident.title` /
//! `Incident.detail` / `evidence_refs.fingerprint_hashes` BEFORE invoking
//! `save_new_incident` / `update_incident_status`. The adapter does NOT
//! double-scrub the bincode payload (layer-separation per corpus
//! `CorpusWriter` trait docstring).

use std::io;
use std::sync::Arc;

use corpus::contract::{CorpusWriter, Error as CorpusError, IncidentRowRaw};
use triage::contract::{Incident, IncidentError, IncidentPersistence, incident_status_label};

/// Adapter implementing `triage::IncidentPersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusIncidentPersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusIncidentPersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl IncidentPersistence for CorpusIncidentPersistence {
    fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError> {
        let payload = bincode::serialize(incident).map_err(|_| IncidentError::Serialize)?;
        let id = self
            .writer
            .save_incident(
                &incident.workspace,
                incident_status_label(incident.status),
                incident.opened_at_unix_nano,
                incident.updated_at_unix_nano,
                incident.resolved_at_unix_nano,
                incident.read_at_unix_nano,
                &payload,
            )
            .map_err(corpus_error_to_incident_error)?;
        Ok(id)
    }

    fn update_incident_status(&self, id: i64, payload: &Incident) -> Result<(), IncidentError> {
        let bytes = bincode::serialize(payload).map_err(|_| IncidentError::Serialize)?;
        self.writer
            .update_incident_status(
                id,
                incident_status_label(payload.status),
                payload.updated_at_unix_nano,
                payload.resolved_at_unix_nano,
                &bytes,
            )
            .map_err(|err| match err {
                CorpusError::QueryFailed => IncidentError::NotFound,
                other => corpus_error_to_incident_error(other),
            })
    }

    fn mark_read(&self, id: i64, read_unix_nano: i64) -> Result<(), IncidentError> {
        self.writer
            .mark_incident_read(id, read_unix_nano)
            .map_err(|err| match err {
                CorpusError::QueryFailed => IncidentError::NotFound,
                other => corpus_error_to_incident_error(other),
            })
    }

    fn load_active_incidents(&self, workspace: &str) -> Result<Vec<Incident>, IncidentError> {
        let rows = self
            .writer
            .load_active_incidents(workspace)
            .map_err(corpus_error_to_incident_error)?;
        let mut incidents = Vec::with_capacity(rows.len());
        for row in rows {
            let mut incident: Incident = bincode::deserialize::<Incident>(&row.payload)
                .map_err(|_| IncidentError::Deserialize)?;
            // Re-stamp authoritative SQL-column metadata onto the loaded
            // struct (corpus columns are source-of-truth for fields they
            // hold; payload BLOB carries the rest of the fields like
            // `kind` / `scope` / `severity` / `priority_tier`).
            incident.id = row.id;
            incident.workspace = row.workspace;
            incident.opened_at_unix_nano = row.created_unix_nano;
            incident.updated_at_unix_nano = row.updated_unix_nano;
            incident.resolved_at_unix_nano = row.resolved_unix_nano;
            incident.read_at_unix_nano = row.read_unix_nano;
            incidents.push(incident);
        }
        Ok(incidents)
    }

    fn load_incidents_for_workspace_since(
        &self,
        workspace: &str,
        since_unix_nano: i64,
    ) -> Result<Vec<Incident>, IncidentError> {
        let rows = self
            .writer
            .load_incidents_for_workspace_since(workspace, since_unix_nano)
            .map_err(corpus_error_to_incident_error)?;
        let mut incidents = Vec::with_capacity(rows.len());
        for row in rows {
            let mut incident: Incident = bincode::deserialize::<Incident>(&row.payload)
                .map_err(|_| IncidentError::Deserialize)?;
            incident.id = row.id;
            incident.workspace = row.workspace;
            incident.opened_at_unix_nano = row.created_unix_nano;
            incident.updated_at_unix_nano = row.updated_unix_nano;
            incident.resolved_at_unix_nano = row.resolved_unix_nano;
            incident.read_at_unix_nano = row.read_unix_nano;
            incidents.push(incident);
        }
        Ok(incidents)
    }

    fn count_active_unread(&self, workspace: &str) -> Result<u64, IncidentError> {
        self.writer
            .count_active_unread(workspace)
            .map_err(corpus_error_to_incident_error)
    }

    fn save_incident_event(
        &self,
        incident_id: i64,
        event_kind: &str,
        occurred_at: i64,
    ) -> Result<(), IncidentError> {
        // chunk #78 ships with empty payload bytes; future chunks may
        // attach per-event metadata (e.g., LLM interpretation results).
        self.writer
            .save_incident_event(incident_id, event_kind, occurred_at, &[])
            .map_err(corpus_error_to_incident_error)
    }
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the IncidentError surfaced upward. Mirrors
/// chunk #70 `corpus_error_to_baseline_error` + chunk #71
/// `corpus_error_to_lifecycle_error` precedents.
///
/// Free function (not `From` impl) — orphan rule forbids
/// `impl From<corpus::Error> for triage::IncidentError` here (both types
/// foreign to pulse-app). Per session-learnings 2026-05-18.
#[doc(hidden)]
pub fn corpus_error_to_incident_error(err: CorpusError) -> IncidentError {
    match err {
        CorpusError::KeyringUnavailable => IncidentError::Io {
            kind: io::ErrorKind::PermissionDenied,
        },
        CorpusError::MigrationFailed => IncidentError::Io {
            kind: io::ErrorKind::Other,
        },
        CorpusError::EncryptionFailed => IncidentError::Serialize,
        CorpusError::DecryptionFailed => IncidentError::Deserialize,
        CorpusError::QueryFailed => IncidentError::Io {
            kind: io::ErrorKind::Other,
        },
        CorpusError::PathTraversal => IncidentError::PathTraversal,
        CorpusError::SizeCapExceeded => IncidentError::SizeCapExceeded { actual: 0, max: 0 },
        CorpusError::SchemaVersionMismatch => IncidentError::SchemaVersionMismatch {
            expected: 1,
            got: 0,
        },
        CorpusError::Io { kind } => IncidentError::Io { kind },
    }
}

// Unit tests live at `pulse-app/tests/unit_incident_persistence.rs`
// (integration test crate) per session-learnings 2026-05-13 — Cargo.toml
// `[lib] test = false` disables the lib auto-generated test binary on
// Windows due to a WebView2 DLL load failure, so source-level
// `#[cfg(test)] mod tests` would compile but never run.
//
// `IncidentRowRaw` import preserved through this comment to keep the
// audit-trail visible even if rustc later prunes the use as unused
// (currently inlined into `load_active_incidents` via destructuring).
#[allow(dead_code)]
fn _force_incident_row_raw_referenced(row: &IncidentRowRaw) -> i64 {
    row.id
}
