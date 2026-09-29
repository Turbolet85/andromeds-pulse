//! Services TauRPC router — chunk #67.
//!
//! `services.list_with_states` resolver returning the per-service
//! lifecycle registry snapshot. Mirrors `connection_router.rs` shape:
//! library crate (`crates/triage`) stays Tauri-free; the Tauri-aware
//! router lives here at the binary boundary so taurpc + specta deps don't
//! leak into the workspace crate per arch §Cross-cutting Patterns Module
//! dependency direction.
//!
//! The registry trait + in-memory impl live in `triage::lifecycle::registry`;
//! this file wraps them in a `ServicesApi` TauRPC procedure trait + emits
//! the request-side observability event per
//! `.claude/rules/observability.md` Session Addition 2026-05-07 (exact-match
//! AllowList entry for `services.list_with_states.request`).
//!
//! Per-service severity enrichment: the resolver reads the workspace's
//! incidents ONCE (Resolved included) and joins them on `Incident.scope_id`
//! (service-scoped incidents only). `ServiceListItem.priority_tier` carries
//! the max tier across the non-Resolved subset; `tier_effective_at_unix_nano`
//! carries the instant that maximum last changed value (the raising
//! incident's open, or the last max holder's resolution). Both come from the
//! one snapshot, so they can never disagree across two reads. The join lives
//! at the binary boundary (the `triage` service registry holds no
//! cross-domain incident state).
//!
//! Tests live at `pulse-app/tests/unit_services_router.rs` (integration
//! crate) because `pulse-app` sets `[lib] test = false` per the WebView2
//! test-link workaround — colocated `#[cfg(test)]` blocks never run
//! (.claude/rules/testing.md Session Additions 2026-05-20).

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use triage::contract::{
    CueScope, IncidentRegistry, IncidentStatus, PriorityTier, ServiceLifecycleBroadcast,
    ServiceListItem, ServiceRegistry, tier_effective_at,
};
use ui_bridge::contract::AppError;

/// Paginated list envelope per arch §Standard Contracts. `next_cursor`
/// reserved for future pagination wire-up; chunk #67 returns the full set
/// in one response (registry is bounded by `ACTIVITY_FLOOR_SERVICE_CAP`).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ServiceListPayload {
    pub items: Vec<ServiceListItem>,
    pub total: usize,
    pub next_cursor: Option<String>,
}

#[taurpc::procedures(path = "services")]
pub trait ServicesApi {
    async fn list_with_states() -> Result<ServiceListPayload, AppError>;
}

#[derive(Clone)]
pub struct ServicesApiImpl {
    registry: Arc<dyn ServiceRegistry>,
    incident_registry: Arc<dyn IncidentRegistry>,
    workspace_root: String,
    #[allow(dead_code)]
    broadcast: Arc<ServiceLifecycleBroadcast>,
}

impl ServicesApiImpl {
    pub fn new(
        registry: Arc<dyn ServiceRegistry>,
        incident_registry: Arc<dyn IncidentRegistry>,
        workspace_root: String,
        broadcast: Arc<ServiceLifecycleBroadcast>,
    ) -> Self {
        Self {
            registry,
            incident_registry,
            workspace_root,
            broadcast,
        }
    }
}

/// Total-order rank for `PriorityTier` (which derives no `Ord`). Higher =
/// more severe; used to pick the max tier across a service's incidents.
pub fn tier_rank(tier: PriorityTier) -> u8 {
    match tier {
        PriorityTier::Autonomous => 3,
        PriorityTier::Suggested => 2,
        PriorityTier::Curious => 1,
    }
}

#[taurpc::resolvers]
impl ServicesApi for ServicesApiImpl {
    #[tracing::instrument(skip_all, fields(
        item_count = tracing::field::Empty,
    ))]
    async fn list_with_states(self) -> Result<ServiceListPayload, AppError> {
        let mut items = self.registry.list_all();
        let incidents = self
            .incident_registry
            .list_for_workspace(&self.workspace_root);
        for item in items.iter_mut() {
            item.priority_tier = incidents
                .iter()
                .filter(|inc| {
                    inc.status != IncidentStatus::Resolved
                        && inc.scope == CueScope::Service
                        && inc.scope_id.as_deref() == Some(item.service.as_str())
                })
                .map(|inc| inc.priority_tier)
                .max_by_key(|tier| tier_rank(*tier));
            item.tier_effective_at_unix_nano = tier_effective_at(&incidents, &item.service);
        }
        let total = items.len();
        let payload = ServiceListPayload {
            items,
            total,
            next_cursor: None,
        };

        let span = tracing::Span::current();
        span.record("item_count", total as u64);

        tracing::info!(
            target: "services.list_with_states.request",
            item_count = total as u64,
            "services.list_with_states returned",
        );

        Ok(payload)
    }
}
