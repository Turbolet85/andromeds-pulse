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
//! this file wraps them in а `ServicesApi` TauRPC procedure trait + emits
//! the request-side observability event per
//! `.claude/rules/observability.md` Session Addition 2026-05-07 (exact-match
//! AllowList entry for `services.list_with_states.request`).

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use triage::contract::{ServiceLifecycleBroadcast, ServiceListItem, ServiceRegistry};
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
    #[allow(dead_code)]
    broadcast: Arc<ServiceLifecycleBroadcast>,
}

impl ServicesApiImpl {
    pub fn new(
        registry: Arc<dyn ServiceRegistry>,
        broadcast: Arc<ServiceLifecycleBroadcast>,
    ) -> Self {
        Self {
            registry,
            broadcast,
        }
    }
}

#[taurpc::resolvers]
impl ServicesApi for ServicesApiImpl {
    #[tracing::instrument(skip_all, fields(
        item_count = tracing::field::Empty,
    ))]
    async fn list_with_states(self) -> Result<ServiceListPayload, AppError> {
        let items = self.registry.list_all();
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

#[cfg(test)]
mod tests {
    use super::*;
    use triage::contract::{InMemoryServiceRegistry, ServiceLifecycleState};

    fn make_impl() -> ServicesApiImpl {
        let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
        let broadcast = Arc::new(ServiceLifecycleBroadcast::new());
        ServicesApiImpl::new(registry, broadcast)
    }

    #[tokio::test]
    async fn list_with_states_returns_empty_for_fresh_registry() {
        let api = make_impl();
        let payload = api.list_with_states().await.expect("returns Ok");
        assert_eq!(payload.total, 0);
        assert!(payload.items.is_empty());
        assert!(payload.next_cursor.is_none());
    }

    #[tokio::test]
    async fn list_with_states_reflects_manual_override() {
        let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
        registry.set_manual_override("svc-a", Some(ServiceLifecycleState::Dormant), 1_000_000_000);
        let broadcast = Arc::new(ServiceLifecycleBroadcast::new());
        let api = ServicesApiImpl::new(Arc::clone(&registry), broadcast);
        let payload = api.list_with_states().await.expect("returns Ok");
        assert_eq!(payload.total, 1);
        assert_eq!(payload.items[0].service, "svc-a");
        assert_eq!(payload.items[0].state, ServiceLifecycleState::Dormant);
        assert_eq!(
            payload.items[0].manual_override,
            Some(ServiceLifecycleState::Dormant),
        );
    }
}
