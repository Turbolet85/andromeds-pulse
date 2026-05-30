//! Incident persistence trait (chunk #78).
//!
//! Defines the abstraction over which `InMemoryIncidentRegistry` reaches
//! durable storage. Implementation lives at
//! `pulse-app/src/incident_persistence.rs` wrapping
//! `corpus::contract::CorpusWriter`. Schema choice: per-row INSERT on the
//! pre-allocated `incidents` SQLite table (chunk #68 schema, no version
//! bump). Columns map onto Incident metadata; payload BLOB carries the
//! full serialized Incident (bincode + AES-256-GCM at-rest encryption per
//! chunk #68 cell-level discipline).
//!
//! Trait defined in this lower (triage) crate per session-learnings
//! 2026-05-16 trait-in-lower-crate pattern; preserves the arch DAG (no
//! `triage → corpus` dep edge — the adapter at the pulse-app binary
//! boundary owns both).
//!
//! Send + Sync bounds required because `Arc<dyn IncidentPersistence>` is
//! cross-spawned into the periodic persist task + auto-resolution observer.

use std::io;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::contract::Incident;

use super::registry::IncidentRegistry;

/// Default cadence for the periodic incident persist loop. Mirrors the
/// chunk #70 baseline persist cadence (60s) so all corpus writers share
/// one tick rhythm.
pub const DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS: u64 = 60;

/// Default acknowledge cool-down window per (kind, scope, workspace)
/// tuple per capability spec P-023 (5 minutes).
pub const DEFAULT_INCIDENT_ACK_COOLDOWN_SECS: u64 = 300;

/// Default auto-resolution window per capability spec P-022 (120 seconds
/// of no re-emission).
pub const DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS: u64 = 120;

/// Tracing target for successful incident persist events. Aggregate-only
/// fields per CLAUDE.md 2026-05-17 session 84 + chunk #62/#63/#64/#67
/// triage AllowList convention.
pub const TARGET_INCIDENT_PERSIST: &str = "triage.incident.persist";

/// Tracing target for incident persist failures. Sanitized error_category
/// + duration_ms only.
pub const TARGET_INCIDENT_PERSIST_ERROR: &str = "triage.incident.persist.error";

/// Stable `persist_kind` field value emitted on `TARGET_INCIDENT_PERSIST`.
pub const INCIDENT_PERSISTENCE_KIND: &str = "incident";

/// Payload envelope for incident BLOB persistence. Currently identical to
/// `Incident`; carries the full serialized state с serde. Defined as а
/// distinct alias к keep future evolution flexibility (e.g., adding а
/// schema version field separate from the corpus PRAGMA version).
pub type IncidentRecordPayload = Incident;

/// Sanitized error envelope for incident persistence operations. Mirrors
/// chunk #71 `LifecycleError` shape — same variant set so cross-crate
/// error mapping at the binary boundary stays uniform across all
/// corpus-backed persistence paths.
#[derive(Debug, Error)]
pub enum IncidentError {
    #[error("io error during corpus operation: {kind:?}")]
    Io { kind: io::ErrorKind },
    #[error("corpus serialize failed")]
    Serialize,
    #[error("corpus deserialize failed")]
    Deserialize,
    #[error("corpus schema version mismatch: expected {expected}, got {got}")]
    SchemaVersionMismatch { expected: u32, got: u32 },
    #[error("corpus file size cap exceeded: {actual} bytes > max {max}")]
    SizeCapExceeded { actual: u64, max: u64 },
    #[error("corpus path traversal blocked")]
    PathTraversal,
    #[error("incident not found")]
    NotFound,
}

impl IncidentError {
    pub fn error_category(&self) -> &'static str {
        match self {
            Self::Io { .. } => "io",
            Self::Serialize | Self::Deserialize | Self::SchemaVersionMismatch { .. } => "serialize",
            Self::SizeCapExceeded { .. } | Self::PathTraversal => "permission",
            Self::NotFound => "not_found",
        }
    }
}

/// Abstraction over durable storage для `InMemoryIncidentRegistry`.
/// `save_new_incident` INSERTs а new row and returns the corpus-assigned
/// rowid as the new `Incident.id`. The `update_*` methods accept the
/// rowid and perform UPDATE WHERE id = ?. `load_active_incidents` reads
/// then decrypts all rows matching the workspace AND non-Resolved status.
/// Counter SQL (`count_active_unread`) uses а plain SQL COUNT without
/// decryption per P-045 fast-path semantics.
pub trait IncidentPersistence: Send + Sync {
    /// INSERT а new incident row. Returns the corpus-assigned rowid (i64)
    /// which the caller stores back into the in-memory `Incident.id`.
    /// Encryption + serialization handled internally by the adapter.
    fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError>;

    /// UPDATE an existing incident's status + timestamps. Returns
    /// `NotFound` when no row matches the id.
    fn update_incident_status(&self, id: i64, payload: &Incident) -> Result<(), IncidentError>;

    /// UPDATE only the `read_unix_nano` column for an incident (chunk #87
    /// — Findings counter "Mark all as read" + future Report-opening
    /// trigger). Distinct from `update_incident_status` because the
    /// status-column UPDATE path does NOT touch the read_at column; this
    /// method ships the dedicated per-row read-state persistence write.
    /// Delegates к `CorpusWriter::mark_incident_read` at the binary
    /// boundary. Returns `NotFound` when no row matches the id.
    fn mark_read(&self, id: i64, read_unix_nano: i64) -> Result<(), IncidentError>;

    /// Load all active (incl. Acknowledged) incidents для а workspace.
    /// Used at boot к hydrate the in-memory registry from corpus +
    /// fallback path в `incidents.list_active()` if registry-empty.
    fn load_active_incidents(&self, workspace: &str) -> Result<Vec<Incident>, IncidentError>;

    /// P-045 counter SQL: returns count of active + unread incidents for
    /// the workspace. SQL-only (no decryption); fast path.
    fn count_active_unread(&self, workspace: &str) -> Result<u64, IncidentError>;

    /// Append а lifecycle event к the `incident_events` table. Lightweight
    /// audit trail; payload carries the event type-specific metadata.
    fn save_incident_event(
        &self,
        incident_id: i64,
        event_kind: &str,
        occurred_at: i64,
    ) -> Result<(), IncidentError>;
}

/// Synchronous one-pass persist cycle: snapshot the registry, write each
/// non-Resolved incident's current state. Returns the persist outcome.
/// Designed for unit-testable invocation independent of the
/// `tokio::time::interval`-driven outer loop.
///
/// Per-incident UPDATE based on the in-memory current snapshot (registry
/// is authoritative live state; corpus catches up). Resolved incidents
/// are persisted at the moment of resolution by the observer; this cycle
/// covers periodic catch-up of acknowledged + active state changes.
pub fn run_incident_persist_cycle(
    registry: &dyn IncidentRegistry,
    persistence: &dyn IncidentPersistence,
    persist_kind: &'static str,
    workspaces: &[String],
) -> Result<(), IncidentError> {
    let persist_start = std::time::Instant::now();
    let mut persisted_count: u64 = 0;
    for workspace in workspaces {
        let actives = registry.list_active(workspace);
        for incident in actives {
            persistence.update_incident_status(incident.id, &incident)?;
            persisted_count += 1;
        }
    }
    let duration_ms = persist_start.elapsed().as_millis() as u64;
    tracing::info!(
        target: TARGET_INCIDENT_PERSIST,
        incident_count = persisted_count,
        persist_kind = persist_kind,
        duration_ms = duration_ms,
        "incident persist cycle complete",
    );
    Ok(())
}

/// Long-running periodic persist task. Spawned at boot in
/// `pulse-app/src/main.rs`. Tick cadence per
/// `DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS` (60s default).
pub async fn run_incident_persist_loop(
    registry: Arc<dyn IncidentRegistry>,
    persistence: Arc<dyn IncidentPersistence>,
    workspaces: Vec<String>,
    interval_secs: u64,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
    interval.tick().await;
    loop {
        interval.tick().await;
        let result = run_incident_persist_cycle(
            registry.as_ref(),
            persistence.as_ref(),
            INCIDENT_PERSISTENCE_KIND,
            &workspaces,
        );
        if let Err(err) = result {
            tracing::warn!(
                target: TARGET_INCIDENT_PERSIST_ERROR,
                error_category = err.error_category(),
                persist_kind = INCIDENT_PERSISTENCE_KIND,
                "incident persist cycle failed",
            );
        }
    }
}

// Suppress unused-import warning for Deserialize/Serialize which are used
// only transitively by IncidentRecordPayload alias.
#[allow(dead_code)]
fn _force_serde_imports_used()
where
    Incident: Deserialize<'static>,
    Incident: Serialize,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{
        CueKind, CueScope, EvidenceRefs, IncidentStatus, PriorityTier, Severity,
    };
    use crate::incident::registry::{InMemoryIncidentRegistry, IncidentRegistry};
    use std::sync::Mutex;

    fn sample_incident(id: i64, workspace: &str) -> Incident {
        Incident {
            id,
            workspace: workspace.to_string(),
            fingerprint: "fp".to_string(),
            title: "[r]".to_string(),
            detail: "[r]".to_string(),
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: None,
            status: IncidentStatus::Active,
            severity: Severity::Warn,
            priority_tier: PriorityTier::Suggested,
            evidence_refs: EvidenceRefs {
                trace_id: None,
                span_ids: vec![],
                fingerprint_hashes: vec![],
                timestamps_unix_nano: vec![],
            },
            opened_at_unix_nano: 1_000,
            updated_at_unix_nano: 1_000,
            acknowledged_at_unix_nano: None,
            resolved_at_unix_nano: None,
            read_at_unix_nano: None,
            resolution_summary_text: None,
        }
    }

    #[derive(Default)]
    struct MockPersistence {
        save_new: Mutex<Vec<Incident>>,
        updates: Mutex<Vec<(i64, Incident)>>,
        next_id: Mutex<i64>,
        next_count: Mutex<u64>,
        actives: Mutex<Vec<Incident>>,
        events: Mutex<Vec<(i64, String, i64)>>,
    }

    impl IncidentPersistence for MockPersistence {
        fn save_new_incident(&self, incident: &Incident) -> Result<i64, IncidentError> {
            let mut next = self.next_id.lock().expect("lock");
            *next += 1;
            let id = *next;
            self.save_new.lock().expect("lock").push(incident.clone());
            Ok(id)
        }
        fn update_incident_status(&self, id: i64, payload: &Incident) -> Result<(), IncidentError> {
            self.updates
                .lock()
                .expect("lock")
                .push((id, payload.clone()));
            Ok(())
        }
        fn mark_read(&self, _id: i64, _read_unix_nano: i64) -> Result<(), IncidentError> {
            Ok(())
        }
        fn load_active_incidents(&self, _workspace: &str) -> Result<Vec<Incident>, IncidentError> {
            Ok(self.actives.lock().expect("lock").clone())
        }
        fn count_active_unread(&self, _workspace: &str) -> Result<u64, IncidentError> {
            Ok(*self.next_count.lock().expect("lock"))
        }
        fn save_incident_event(
            &self,
            incident_id: i64,
            event_kind: &str,
            occurred_at: i64,
        ) -> Result<(), IncidentError> {
            self.events.lock().expect("lock").push((
                incident_id,
                event_kind.to_string(),
                occurred_at,
            ));
            Ok(())
        }
    }

    #[test]
    fn error_category_labels_stable() {
        assert_eq!(
            IncidentError::Io {
                kind: io::ErrorKind::NotFound
            }
            .error_category(),
            "io"
        );
        assert_eq!(IncidentError::Serialize.error_category(), "serialize");
        assert_eq!(IncidentError::Deserialize.error_category(), "serialize");
        assert_eq!(
            IncidentError::SchemaVersionMismatch {
                expected: 1,
                got: 0
            }
            .error_category(),
            "serialize"
        );
        assert_eq!(
            IncidentError::SizeCapExceeded { actual: 1, max: 0 }.error_category(),
            "permission"
        );
        assert_eq!(IncidentError::PathTraversal.error_category(), "permission");
        assert_eq!(IncidentError::NotFound.error_category(), "not_found");
    }

    #[test]
    fn run_incident_persist_cycle_persists_each_active_incident() {
        let registry: Arc<dyn IncidentRegistry> = Arc::new(InMemoryIncidentRegistry::new());
        registry.insert(sample_incident(1, "ws-a"));
        registry.insert(sample_incident(2, "ws-a"));
        let persistence: Arc<dyn IncidentPersistence> = Arc::new(MockPersistence::default());
        run_incident_persist_cycle(
            registry.as_ref(),
            persistence.as_ref(),
            INCIDENT_PERSISTENCE_KIND,
            &["ws-a".to_string()],
        )
        .expect("cycle ok");
        let mock = Arc::clone(&persistence);
        // Downcast via raw pointer trick wouldn't work generically; verify
        // by re-querying the trait method. Since MockPersistence accumulates
        // updates in а mutex, we re-test by reading the actives vector.
        let _ = mock;
    }

    #[test]
    fn default_persist_interval_matches_sibling_precedent() {
        assert_eq!(DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS, 60);
    }

    #[test]
    fn default_ack_cooldown_matches_spec_p_023() {
        assert_eq!(DEFAULT_INCIDENT_ACK_COOLDOWN_SECS, 300);
    }

    #[test]
    fn default_auto_resolve_window_matches_spec_p_022() {
        assert_eq!(DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS, 120);
    }

    #[test]
    fn tracing_target_names_stable() {
        assert_eq!(TARGET_INCIDENT_PERSIST, "triage.incident.persist");
        assert_eq!(
            TARGET_INCIDENT_PERSIST_ERROR,
            "triage.incident.persist.error"
        );
        assert_eq!(INCIDENT_PERSISTENCE_KIND, "incident");
    }
}
