//! Incident lifecycle persistence — chunk #78.
//!
//! L5 persistence layer for the chunk #60 `Incident` contract. Active /
//! Acknowledged / Resolved lifecycle FSM with auto-resolution at 120s of
//! no re-emission + 5-min acknowledge cool-down per `(kind, scope)` tuple
//! per capability spec P-022 + P-023. Workspace attribution via
//! `workspace-detector`; counter derivation per P-045
//! (`SELECT COUNT(*) FROM incidents WHERE workspace = ? AND read_unix_nano
//! IS NULL AND status = 'active'`).
//!
//! Per arch §Cross-cutting Patterns Module dependency direction, this
//! module lives inside the `triage` crate; cross-crate state delivery
//! happens via the `IncidentRegistry` + `IncidentPersistence` traits +
//! `pulse-app` boundary adapter (mirrors chunk #67 / #71 precedent).
//!
//! Distinct from `crate::lifecycle` which covers per-service (not
//! per-incident) state machine.

mod broadcast;
mod persistence;
mod registry;
mod state_machine;

pub use broadcast::{IncidentLifecycleBroadcast, IncidentLifecycleEvent, STREAM_NAME_INCIDENTS};
pub use persistence::{
    DEFAULT_INCIDENT_ACK_COOLDOWN_SECS, DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS,
    DEFAULT_INCIDENT_PERSIST_INTERVAL_SECS, DurableActiveIncidents, INCIDENT_PERSISTENCE_KIND,
    INCIDENT_RECONCILE_KIND, IncidentError, IncidentPersistence, IncidentRecordPayload,
    IncidentWriteOutcome, TARGET_INCIDENT_PERSIST, TARGET_INCIDENT_PERSIST_ERROR,
    run_incident_persist_cycle, run_incident_persist_loop,
};
pub use registry::{
    InMemoryIncidentRegistry, IncidentRegistry, IncidentRegistryError, ResolutionTrigger,
};
pub use state_machine::{
    cooldown_expiry_unix_nano, is_valid_incident_transition, should_auto_resolve, status_label,
};
