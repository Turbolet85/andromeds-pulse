//! Incident registry — in-memory live state holder + per-(kind, scope,
//! workspace) acknowledge cool-down per capability spec P-022 + P-023.
//! Chunk #78.
//!
//! `IncidentRegistry` trait + `InMemoryIncidentRegistry` backed by а
//! `DashMap<i64, Incident>` keyed on corpus rowid (Incident.id). Mirrors
//! chunk #67 `InMemoryServiceRegistry` lock-free shard pattern.
//!
//! Cool-down tracking: separate `DashMap<(CueKind, CueScope, String_workspace),
//! i64_expiry_unix_nano>` — acknowledge checks expiry before transition;
//! reject if active.

use std::fmt::Debug;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::contract::{CueKind, CueScope, Incident, IncidentStatus};

use super::state_machine::{
    cooldown_expiry_unix_nano, is_valid_incident_transition, should_auto_resolve,
};

/// Trigger reason for а Resolved-state transition, carried on
/// `IncidentLifecycleEvent` and surface logs. Distinguishes auto-resolution
/// from explicit user resolve.
#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionTrigger {
    AutoResolve,
    ExplicitResolve,
}

/// Sanitized error envelope for incident registry operations. Distinct from
/// `IncidentError` (persistence-side); registry errors are логические
/// (NotFound / InvalidTransition / CooldownActive). No corpus/io variants
/// here.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum IncidentRegistryError {
    #[error("incident not found")]
    NotFound,
    #[error("invalid lifecycle transition")]
    InvalidTransition,
    #[error("acknowledge cool-down active; {remaining_secs}s remaining")]
    CooldownActive { remaining_secs: u64 },
}

impl IncidentRegistryError {
    pub fn error_category(&self) -> &'static str {
        match self {
            Self::NotFound => "not_found",
            Self::InvalidTransition => "invalid_transition",
            Self::CooldownActive { .. } => "cooldown_active",
        }
    }
}

/// In-memory incident state holder + per-(kind, scope, workspace)
/// acknowledge cool-down tracker. Implementations MUST be `Send + Sync +
/// Debug` so callers can hold `Arc<dyn IncidentRegistry>` and thread it
/// through TauRPC resolvers.
pub trait IncidentRegistry: Send + Sync + Debug {
    /// Insert а new incident (post-corpus-INSERT; caller already has the
    /// assigned rowid in `incident.id`).
    fn insert(&self, incident: Incident);

    /// Lookup by corpus rowid.
    fn get(&self, id: i64) -> Option<Incident>;

    /// Snapshot of all active (incl. acknowledged) incidents for а
    /// workspace. Used by `incidents.list_active()`.
    fn list_active(&self, workspace: &str) -> Vec<Incident>;

    /// Transition an Active incident to Acknowledged. Checks the
    /// (kind, scope, workspace) cool-down and returns `CooldownActive`
    /// if active. On success sets the incident's
    /// `acknowledged_at_unix_nano` and `updated_at_unix_nano` and records
    /// а new cool-down expiry for the tuple. Returns the updated
    /// Incident snapshot.
    fn acknowledge(
        &self,
        id: i64,
        now_unix_nano: i64,
        cooldown_secs: u64,
    ) -> Result<Incident, IncidentRegistryError>;

    /// Transition an incident to Resolved (Active/Acknowledged → Resolved).
    /// Sets `resolved_at_unix_nano` + `updated_at_unix_nano`. Returns the
    /// updated Incident snapshot on success.
    fn mark_resolved(
        &self,
        id: i64,
        now_unix_nano: i64,
        _trigger: ResolutionTrigger,
    ) -> Result<Incident, IncidentRegistryError>;

    /// Mark an incident as read (Report-opening event; chunk #78 schema
    /// supports the column but chunk #87+ wires the UI trigger).
    fn mark_read(&self, id: i64, now_unix_nano: i64) -> Result<Incident, IncidentRegistryError>;

    /// Bulk-mark all unread non-Resolved incidents in а workspace as read
    /// (chunk #87 — Findings counter "Mark all as read" action; capability
    /// P-029 dropdown footer). Iterates the workspace's incidents and sets
    /// `read_at_unix_nano = Some(now)` + `updated_at_unix_nano = now` for
    /// each Active/Acknowledged incident с `read_at_unix_nano.is_none()`.
    /// Returns the Vec<i64> of incident ids that transitioned from unread
    /// → read so the caller can drive per-incident persistence updates +
    /// aggregate-only observability emission. Already-read incidents and
    /// Resolved incidents are skipped. Empty workspace returns empty Vec.
    fn mark_all_read(
        &self,
        workspace: &str,
        now_unix_nano: i64,
    ) -> Result<Vec<i64>, IncidentRegistryError>;

    /// Bump `updated_at_unix_nano` when а re-emission observed for а live
    /// incident (resets the 120s no-reemission auto-resolve timer).
    fn observe_reemission(&self, id: i64, now_unix_nano: i64) -> Result<(), IncidentRegistryError>;

    /// Evaluate every Active incident against the auto-resolve window.
    /// Returns the list of ids to auto-resolve. Pure function w.r.t.
    /// `now_unix_nano` — caller injects deterministic time for tests.
    fn evaluate_auto_resolution(&self, now_unix_nano: i64, window_secs: u64) -> Vec<i64>;

    /// Attach an L4-generated resolution summary к а Resolved incident
    /// (chunk #86; capabilities P-022 + P-059). The caller (L4 inference
    /// subscriber) ensures the `summary_text` payload has already passed
    /// through `security::scrubber::scrub_attribute` per chunk #72
    /// uniform-coverage invariant — the registry side does NOT re-scrub.
    /// Updates `resolution_summary_text` + `updated_at_unix_nano`. Errors:
    /// `NotFound` (no such incident); `InvalidTransition` when target
    /// incident is NOT already Resolved (chunk spec: attach к Resolved
    /// only). Returns the updated incident snapshot. Does NOT emit а
    /// `pulse://stream/incidents` lifecycle event per chunk #86 silent-
    /// attachment Phase 6 resolution.
    fn attach_resolution_summary(
        &self,
        id: i64,
        summary_text: String,
        now_unix_nano: i64,
    ) -> Result<Incident, IncidentRegistryError>;

    /// Total incident count в the registry (Active + Acknowledged + Resolved
    /// — for diagnostics + tests).
    fn count(&self) -> usize;
}

/// In-memory `IncidentRegistry` implementation backed by `DashMap`.
/// Mirrors chunk #67 `InMemoryServiceRegistry` shape.
#[derive(Debug, Default)]
pub struct InMemoryIncidentRegistry {
    incidents: DashMap<i64, Incident>,
    cooldowns: DashMap<(CueKind, CueScope, String), i64>,
}

impl InMemoryIncidentRegistry {
    pub fn new() -> Self {
        Self {
            incidents: DashMap::new(),
            cooldowns: DashMap::new(),
        }
    }

    /// Hydrate from corpus-loaded incidents at boot (chunk #78). Used by
    /// `pulse-app/src/main.rs` setup closure after
    /// `IncidentPersistence::load_active_incidents` returns
    /// `Ok(Vec<Incident>)`.
    pub fn from_persisted(incidents: Vec<Incident>) -> Self {
        let map = DashMap::with_capacity(incidents.len());
        for incident in incidents {
            map.insert(incident.id, incident);
        }
        Self {
            incidents: map,
            cooldowns: DashMap::new(),
        }
    }

    fn cooldown_key(&self, incident: &Incident) -> (CueKind, CueScope, String) {
        (incident.kind, incident.scope, incident.workspace.clone())
    }
}

impl IncidentRegistry for InMemoryIncidentRegistry {
    fn insert(&self, incident: Incident) {
        self.incidents.insert(incident.id, incident);
    }

    fn get(&self, id: i64) -> Option<Incident> {
        self.incidents.get(&id).map(|e| e.clone())
    }

    fn list_active(&self, workspace: &str) -> Vec<Incident> {
        self.incidents
            .iter()
            .filter(|entry| {
                entry.workspace == workspace && entry.status != IncidentStatus::Resolved
            })
            .map(|entry| entry.clone())
            .collect()
    }

    fn acknowledge(
        &self,
        id: i64,
        now_unix_nano: i64,
        cooldown_secs: u64,
    ) -> Result<Incident, IncidentRegistryError> {
        let mut entry = self
            .incidents
            .get_mut(&id)
            .ok_or(IncidentRegistryError::NotFound)?;
        if !is_valid_incident_transition(entry.status, IncidentStatus::Acknowledged) {
            return Err(IncidentRegistryError::InvalidTransition);
        }
        let key = (entry.kind, entry.scope, entry.workspace.clone());
        if let Some(expiry) = self.cooldowns.get(&key) {
            if *expiry > now_unix_nano {
                let remaining_nanos = *expiry - now_unix_nano;
                let remaining_secs = (remaining_nanos / 1_000_000_000) as u64;
                return Err(IncidentRegistryError::CooldownActive { remaining_secs });
            }
        }
        entry.status = IncidentStatus::Acknowledged;
        entry.acknowledged_at_unix_nano = Some(now_unix_nano);
        entry.updated_at_unix_nano = now_unix_nano;
        let new_expiry = cooldown_expiry_unix_nano(now_unix_nano, cooldown_secs);
        self.cooldowns.insert(key, new_expiry);
        Ok(entry.clone())
    }

    fn mark_resolved(
        &self,
        id: i64,
        now_unix_nano: i64,
        _trigger: ResolutionTrigger,
    ) -> Result<Incident, IncidentRegistryError> {
        let mut entry = self
            .incidents
            .get_mut(&id)
            .ok_or(IncidentRegistryError::NotFound)?;
        if !is_valid_incident_transition(entry.status, IncidentStatus::Resolved) {
            return Err(IncidentRegistryError::InvalidTransition);
        }
        entry.status = IncidentStatus::Resolved;
        entry.resolved_at_unix_nano = Some(now_unix_nano);
        entry.updated_at_unix_nano = now_unix_nano;
        Ok(entry.clone())
    }

    fn mark_read(&self, id: i64, now_unix_nano: i64) -> Result<Incident, IncidentRegistryError> {
        let mut entry = self
            .incidents
            .get_mut(&id)
            .ok_or(IncidentRegistryError::NotFound)?;
        entry.read_at_unix_nano = Some(now_unix_nano);
        entry.updated_at_unix_nano = now_unix_nano;
        Ok(entry.clone())
    }

    fn mark_all_read(
        &self,
        workspace: &str,
        now_unix_nano: i64,
    ) -> Result<Vec<i64>, IncidentRegistryError> {
        let mut affected: Vec<i64> = Vec::new();
        for mut entry in self.incidents.iter_mut() {
            if entry.workspace != workspace {
                continue;
            }
            if entry.status == IncidentStatus::Resolved {
                continue;
            }
            if entry.read_at_unix_nano.is_some() {
                continue;
            }
            entry.read_at_unix_nano = Some(now_unix_nano);
            entry.updated_at_unix_nano = now_unix_nano;
            affected.push(*entry.key());
        }
        affected.sort_unstable();
        Ok(affected)
    }

    fn observe_reemission(&self, id: i64, now_unix_nano: i64) -> Result<(), IncidentRegistryError> {
        let mut entry = self
            .incidents
            .get_mut(&id)
            .ok_or(IncidentRegistryError::NotFound)?;
        if entry.status == IncidentStatus::Resolved {
            return Err(IncidentRegistryError::InvalidTransition);
        }
        entry.updated_at_unix_nano = now_unix_nano;
        Ok(())
    }

    fn evaluate_auto_resolution(&self, now_unix_nano: i64, window_secs: u64) -> Vec<i64> {
        self.incidents
            .iter()
            .filter(|entry| {
                entry.status != IncidentStatus::Resolved
                    && should_auto_resolve(entry.updated_at_unix_nano, now_unix_nano, window_secs)
            })
            .map(|entry| *entry.key())
            .collect()
    }

    fn attach_resolution_summary(
        &self,
        id: i64,
        summary_text: String,
        now_unix_nano: i64,
    ) -> Result<Incident, IncidentRegistryError> {
        let mut entry = self
            .incidents
            .get_mut(&id)
            .ok_or(IncidentRegistryError::NotFound)?;
        if entry.status != IncidentStatus::Resolved {
            return Err(IncidentRegistryError::InvalidTransition);
        }
        entry.resolution_summary_text = Some(summary_text);
        entry.updated_at_unix_nano = now_unix_nano;
        Ok(entry.clone())
    }

    fn count(&self) -> usize {
        self.incidents.len()
    }
}

// Suppress unused-import warning when cooldown_key helper not called from
// other methods directly — used implicitly via cooldown lookup.
#[allow(dead_code)]
fn _ensure_cooldown_key_referenced(reg: &InMemoryIncidentRegistry, inc: &Incident) {
    let _ = reg.cooldown_key(inc);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{EvidenceRefs, PriorityTier, Severity};

    fn fresh_registry() -> InMemoryIncidentRegistry {
        InMemoryIncidentRegistry::new()
    }

    fn sample_incident(id: i64, workspace: &str, kind: CueKind, scope: CueScope) -> Incident {
        Incident {
            id,
            workspace: workspace.to_string(),
            fingerprint: format!("fp-{kind:?}-{scope:?}"),
            title: "[redacted] sample incident".to_string(),
            detail: "[redacted] sample detail".to_string(),
            kind,
            scope,
            status: IncidentStatus::Active,
            severity: Severity::Warn,
            priority_tier: PriorityTier::Suggested,
            evidence_refs: EvidenceRefs {
                trace_id: None,
                span_ids: vec![],
                fingerprint_hashes: vec![],
                timestamps_unix_nano: vec![],
            },
            opened_at_unix_nano: 1_000_000_000,
            updated_at_unix_nano: 1_000_000_000,
            acknowledged_at_unix_nano: None,
            resolved_at_unix_nano: None,
            read_at_unix_nano: None,
            resolution_summary_text: None,
        }
    }

    #[test]
    fn registry_starts_empty() {
        let r = fresh_registry();
        assert_eq!(r.count(), 0);
        assert!(r.list_active("ws-a").is_empty());
        assert!(r.get(42).is_none());
    }

    #[test]
    fn insert_and_get_round_trips() {
        let r = fresh_registry();
        let inc = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        r.insert(inc.clone());
        assert_eq!(r.count(), 1);
        let loaded = r.get(1).expect("Some");
        assert_eq!(loaded, inc);
    }

    #[test]
    fn list_active_filters_by_workspace_and_resolved() {
        let r = fresh_registry();
        let a = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        let b = sample_incident(2, "ws-b", CueKind::LatencyRegression, CueScope::Service);
        let mut c = sample_incident(3, "ws-a", CueKind::RetryStorm, CueScope::Global);
        c.status = IncidentStatus::Resolved;
        r.insert(a);
        r.insert(b);
        r.insert(c);
        let active_a = r.list_active("ws-a");
        assert_eq!(active_a.len(), 1, "only Active in ws-a");
        assert_eq!(active_a[0].id, 1);
    }

    #[test]
    fn acknowledge_active_succeeds() {
        let r = fresh_registry();
        let inc = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        r.insert(inc);
        let result = r.acknowledge(1, 2_000_000_000, 300);
        assert!(result.is_ok(), "ack succeeds; got {result:?}");
        let updated = result.unwrap();
        assert_eq!(updated.status, IncidentStatus::Acknowledged);
        assert_eq!(updated.acknowledged_at_unix_nano, Some(2_000_000_000));
        assert_eq!(updated.updated_at_unix_nano, 2_000_000_000);
    }

    #[test]
    fn acknowledge_not_found_returns_not_found() {
        let r = fresh_registry();
        let result = r.acknowledge(999, 1_000_000_000, 300);
        assert_eq!(result.unwrap_err(), IncidentRegistryError::NotFound);
    }

    #[test]
    fn acknowledge_already_acknowledged_returns_invalid_transition() {
        let r = fresh_registry();
        let mut inc = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        inc.status = IncidentStatus::Acknowledged;
        r.insert(inc);
        let result = r.acknowledge(1, 2_000_000_000, 300);
        assert_eq!(
            result.unwrap_err(),
            IncidentRegistryError::InvalidTransition
        );
    }

    #[test]
    fn acknowledge_within_cooldown_for_same_kind_scope_workspace_rejects() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        r.insert(sample_incident(
            2,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        // First ack establishes cool-down.
        r.acknowledge(1, 1_000_000_000_000_000_000, 300)
            .expect("first ack");
        // Second ack within cooldown for SAME (kind, scope, workspace) tuple.
        let result = r.acknowledge(2, 1_000_000_001_000_000_000, 300);
        match result {
            Err(IncidentRegistryError::CooldownActive { remaining_secs }) => {
                assert!((298..=300).contains(&remaining_secs));
            }
            other => panic!("expected CooldownActive; got {other:?}"),
        }
    }

    #[test]
    fn acknowledge_after_cooldown_window_succeeds() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        r.insert(sample_incident(
            2,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        let t0 = 1_000_000_000_000_000_000i64;
        r.acknowledge(1, t0, 300).expect("first ack");
        // 301s later — beyond cool-down.
        let t1 = t0 + 301_000_000_000;
        let result = r.acknowledge(2, t1, 300);
        assert!(
            result.is_ok(),
            "ack outside cooldown succeeds; got {result:?}"
        );
    }

    #[test]
    fn acknowledge_different_workspace_bypasses_cooldown() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        r.insert(sample_incident(
            2,
            "ws-b",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        let t0 = 1_000_000_000_000_000_000i64;
        r.acknowledge(1, t0, 300).expect("ws-a ack");
        let result = r.acknowledge(2, t0 + 100, 300);
        assert!(result.is_ok(), "different workspace bypasses cooldown");
    }

    #[test]
    fn mark_resolved_active_succeeds() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        let result = r.mark_resolved(1, 2_000_000_000, ResolutionTrigger::ExplicitResolve);
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.status, IncidentStatus::Resolved);
        assert_eq!(updated.resolved_at_unix_nano, Some(2_000_000_000));
    }

    #[test]
    fn mark_resolved_acknowledged_succeeds() {
        let r = fresh_registry();
        let mut inc = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        inc.status = IncidentStatus::Acknowledged;
        r.insert(inc);
        let result = r.mark_resolved(1, 2_000_000_000, ResolutionTrigger::ExplicitResolve);
        assert!(result.is_ok());
    }

    #[test]
    fn mark_resolved_already_resolved_returns_invalid_transition() {
        let r = fresh_registry();
        let mut inc = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        inc.status = IncidentStatus::Resolved;
        r.insert(inc);
        let result = r.mark_resolved(1, 2_000_000_000, ResolutionTrigger::AutoResolve);
        assert_eq!(
            result.unwrap_err(),
            IncidentRegistryError::InvalidTransition
        );
    }

    #[test]
    fn mark_resolved_not_found_returns_not_found() {
        let r = fresh_registry();
        let result = r.mark_resolved(999, 1_000_000_000, ResolutionTrigger::ExplicitResolve);
        assert_eq!(result.unwrap_err(), IncidentRegistryError::NotFound);
    }

    #[test]
    fn observe_reemission_updates_timestamp() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        let t = 5_000_000_000i64;
        r.observe_reemission(1, t).expect("ok");
        assert_eq!(r.get(1).unwrap().updated_at_unix_nano, t);
    }

    #[test]
    fn observe_reemission_on_resolved_rejects() {
        let r = fresh_registry();
        let mut inc = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        inc.status = IncidentStatus::Resolved;
        r.insert(inc);
        let result = r.observe_reemission(1, 2_000_000_000);
        assert_eq!(
            result.unwrap_err(),
            IncidentRegistryError::InvalidTransition
        );
    }

    #[test]
    fn evaluate_auto_resolution_returns_active_past_window() {
        let r = fresh_registry();
        let t0 = 1_000_000_000_000_000_000i64;
        let mut inc1 = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        inc1.updated_at_unix_nano = t0;
        let mut inc2 = sample_incident(2, "ws-a", CueKind::LatencyRegression, CueScope::Service);
        inc2.updated_at_unix_nano = t0 + 50_000_000_000; // 50s ago at evaluation
        r.insert(inc1);
        r.insert(inc2);
        let now = t0 + 130_000_000_000; // 130s after t0
        let to_resolve = r.evaluate_auto_resolution(now, 120);
        assert_eq!(to_resolve, vec![1], "only inc1 past window");
    }

    #[test]
    fn evaluate_auto_resolution_skips_resolved() {
        let r = fresh_registry();
        let t0 = 1_000_000_000_000_000_000i64;
        let mut inc1 = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        inc1.status = IncidentStatus::Resolved;
        inc1.updated_at_unix_nano = t0;
        r.insert(inc1);
        let now = t0 + 500_000_000_000;
        let to_resolve = r.evaluate_auto_resolution(now, 120);
        assert!(to_resolve.is_empty());
    }

    #[test]
    fn evaluate_auto_resolution_includes_acknowledged() {
        let r = fresh_registry();
        let t0 = 1_000_000_000_000_000_000i64;
        let mut inc1 = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        inc1.status = IncidentStatus::Acknowledged;
        inc1.updated_at_unix_nano = t0;
        r.insert(inc1);
        let now = t0 + 200_000_000_000;
        let to_resolve = r.evaluate_auto_resolution(now, 120);
        assert_eq!(to_resolve, vec![1]);
    }

    #[test]
    fn mark_read_sets_read_at() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        let result = r.mark_read(1, 7_000_000_000);
        assert!(result.is_ok());
        assert_eq!(r.get(1).unwrap().read_at_unix_nano, Some(7_000_000_000));
    }

    #[test]
    fn from_persisted_hydrates_registry() {
        let incs = vec![
            sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service),
            sample_incident(2, "ws-b", CueKind::LatencyRegression, CueScope::Operation),
        ];
        let r = InMemoryIncidentRegistry::from_persisted(incs);
        assert_eq!(r.count(), 2);
        assert!(r.get(1).is_some());
        assert!(r.get(2).is_some());
    }

    #[test]
    fn error_category_labels_stable() {
        assert_eq!(
            IncidentRegistryError::NotFound.error_category(),
            "not_found"
        );
        assert_eq!(
            IncidentRegistryError::InvalidTransition.error_category(),
            "invalid_transition"
        );
        assert_eq!(
            IncidentRegistryError::CooldownActive { remaining_secs: 60 }.error_category(),
            "cooldown_active"
        );
    }

    #[test]
    fn attach_resolution_summary_writes_summary_to_resolved_incident() {
        let r = fresh_registry();
        let mut inc = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        // Pre-resolve the incident — chunk #86 spec attaches summaries
        // only to Resolved incidents.
        inc.status = IncidentStatus::Resolved;
        inc.resolved_at_unix_nano = Some(2_000_000_000);
        r.insert(inc);

        let now = 3_000_000_000_i64;
        let summary = "[redacted] resolved after 120s of no re-emission".to_string();
        let updated = r
            .attach_resolution_summary(1, summary.clone(), now)
            .expect("attach succeeds on Resolved");
        assert_eq!(updated.resolution_summary_text, Some(summary));
        assert_eq!(updated.updated_at_unix_nano, now);
        assert_eq!(updated.status, IncidentStatus::Resolved);
    }

    #[test]
    fn attach_resolution_summary_rejects_active_incident() {
        let r = fresh_registry();
        // Default sample_incident is Active; resolution-summary attachment
        // requires Resolved state per chunk #86 spec.
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        let result = r.attach_resolution_summary(1, "summary".to_string(), 5_000_000_000);
        assert!(matches!(
            result,
            Err(IncidentRegistryError::InvalidTransition)
        ));
        // Verify the incident's resolution_summary_text was NOT mutated.
        assert_eq!(r.get(1).unwrap().resolution_summary_text, None);
    }

    #[test]
    fn attach_resolution_summary_returns_not_found_for_unknown_id() {
        let r = fresh_registry();
        let result = r.attach_resolution_summary(999, "summary".to_string(), 5_000_000_000);
        assert!(matches!(result, Err(IncidentRegistryError::NotFound)));
    }

    #[test]
    fn mark_all_read_on_empty_workspace_returns_empty_vec() {
        let r = fresh_registry();
        let result = r.mark_all_read("ws-a", 5_000_000_000).expect("ok");
        assert!(result.is_empty());
    }

    #[test]
    fn mark_all_read_marks_all_unread_in_workspace_and_returns_ids() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        r.insert(sample_incident(
            2,
            "ws-a",
            CueKind::LatencyRegression,
            CueScope::Service,
        ));
        r.insert(sample_incident(
            3,
            "ws-a",
            CueKind::RetryStorm,
            CueScope::Global,
        ));
        let now = 7_000_000_000_i64;
        let affected = r.mark_all_read("ws-a", now).expect("ok");
        assert_eq!(affected, vec![1, 2, 3]);
        for id in [1, 2, 3] {
            let inc = r.get(id).expect("present");
            assert_eq!(inc.read_at_unix_nano, Some(now));
            assert_eq!(inc.updated_at_unix_nano, now);
        }
    }

    #[test]
    fn mark_all_read_skips_already_read_incidents() {
        let r = fresh_registry();
        let mut a = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        a.read_at_unix_nano = Some(2_000_000_000);
        r.insert(a);
        r.insert(sample_incident(
            2,
            "ws-a",
            CueKind::LatencyRegression,
            CueScope::Service,
        ));
        let now = 7_000_000_000_i64;
        let affected = r.mark_all_read("ws-a", now).expect("ok");
        assert_eq!(affected, vec![2], "only the unread incident transitions");
        let preserved = r.get(1).expect("present");
        assert_eq!(
            preserved.read_at_unix_nano,
            Some(2_000_000_000),
            "previously-read timestamp must not be overwritten"
        );
    }

    #[test]
    fn mark_all_read_skips_resolved_incidents() {
        let r = fresh_registry();
        let mut a = sample_incident(1, "ws-a", CueKind::ErrorRateSpike, CueScope::Service);
        a.status = IncidentStatus::Resolved;
        r.insert(a);
        r.insert(sample_incident(
            2,
            "ws-a",
            CueKind::LatencyRegression,
            CueScope::Service,
        ));
        let now = 7_000_000_000_i64;
        let affected = r.mark_all_read("ws-a", now).expect("ok");
        assert_eq!(affected, vec![2], "Resolved incidents excluded from bulk");
        let resolved = r.get(1).expect("present");
        assert_eq!(resolved.read_at_unix_nano, None);
    }

    #[test]
    fn mark_all_read_filters_by_workspace() {
        let r = fresh_registry();
        r.insert(sample_incident(
            1,
            "ws-a",
            CueKind::ErrorRateSpike,
            CueScope::Service,
        ));
        r.insert(sample_incident(
            2,
            "ws-b",
            CueKind::LatencyRegression,
            CueScope::Service,
        ));
        let affected = r.mark_all_read("ws-a", 5_000_000_000).expect("ok");
        assert_eq!(affected, vec![1]);
        let other_ws = r.get(2).expect("present");
        assert_eq!(other_ws.read_at_unix_nano, None);
    }
}
