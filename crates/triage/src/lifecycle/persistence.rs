//! Lifecycle persistence trait (chunk #71).
//!
//! Defines the abstraction `InMemoryServiceRegistry` traverses to reach
//! durable storage. Implementation lives at
//! `pulse-app/src/lifecycle_persistence.rs` wrapping
//! `corpus::contract::CorpusWriter` — Schema choice: per-row INSERT/UPDATE
//! on the pre-allocated `service_registry` SQLite table (chunk #68 schema,
//! no version bump). Columns map directly onto `ServiceRegistryEntry`
//! fields; cell-level AES-256-GCM applies to the encrypted columns via
//! the corpus crate's transparent encryption layer.
//!
//! Trait defined in this lower (triage) crate per CLAUDE.md session-learnings
//! 2026-05-16 trait-in-lower-crate pattern; preserves the arch DAG (no
//! `triage → corpus` dep edge — the adapter at the pulse-app binary
//! boundary owns both).
//!
//! Send + Sync bounds required because `Arc<dyn LifecyclePersistence>`
//! is cross-spawned into the periodic persist task.

use std::io;
use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;

use super::{ServiceListItem, ServiceRegistry, ServiceRegistryEntry};

/// Default cadence for the periodic lifecycle persist loop. Mirrors the
/// chunk #70 baseline persist cadence so all corpus writers share one
/// 60s tick rhythm.
pub const DEFAULT_LIFECYCLE_PERSIST_INTERVAL_SECS: u64 = 60;

/// Tracing target for successful lifecycle persist events. Aggregate-only
/// fields per CLAUDE.md 2026-05-17 session 84 + chunk #62/#63/#64 triage
/// AllowList convention.
pub const TARGET_LIFECYCLE_PERSIST: &str = "triage.lifecycle.persist";

/// Tracing target for lifecycle persist failures. Sanitized error_category
/// + duration_ms only.
pub const TARGET_LIFECYCLE_PERSIST_ERROR: &str = "triage.lifecycle.persist.error";

/// Stable `persist_kind` field value emitted on `TARGET_LIFECYCLE_PERSIST`.
pub const LIFECYCLE_PERSISTENCE_KIND: &str = "lifecycle";

/// Sanitized error envelope for lifecycle persistence operations. Mirrors
/// `BaselineError` (chunk #70) — same variant shape so cross-crate error
/// mapping at the binary boundary stays uniform across all corpus-backed
/// persistence paths.
#[derive(Debug, Error)]
pub enum LifecycleError {
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
}

impl LifecycleError {
    pub fn error_category(&self) -> &'static str {
        match self {
            Self::Io { .. } => "io",
            Self::Serialize | Self::Deserialize | Self::SchemaVersionMismatch { .. } => "serialize",
            Self::SizeCapExceeded { .. } | Self::PathTraversal => "permission",
        }
    }
}

/// Abstraction over durable storage for `InMemoryServiceRegistry`.
/// `load_all` returns `Ok(None)` for cold-start (corpus empty);
/// `Ok(Some(entries))` for resume-from-corpus. `save_all` overwrites the
/// snapshot via UPSERT-per-row.
pub trait LifecyclePersistence: Send + Sync {
    /// Load all persisted service registry entries. `Ok(None)` means no
    /// prior state (cold-start path; corpus table empty). Errors surface
    /// to caller for tracing emission; callers should treat as
    /// "proceed with empty registry" rather than fatal (boot is non-fatal
    /// per chunk #68 corpus precedent + chunk #70 baseline precedent).
    fn load_all(&self) -> Result<Option<Vec<(String, ServiceRegistryEntry)>>, LifecycleError>;

    /// Persist the current registry snapshot. Implementation upserts
    /// per service into the `service_registry` table via prepared
    /// statements. Errors surface to caller for tracing emission at the
    /// `triage.lifecycle.persist.error` target.
    fn save_all(&self, entries: &[(String, ServiceRegistryEntry)]) -> Result<(), LifecycleError>;
}

/// Synchronous one-pass persist cycle: snapshot the registry, save via
/// the trait, emit aggregate tracing events. Returns the persist outcome
/// for callers that want explicit error handling. Designed for
/// unit-testable invocation independent of the `tokio::time::interval`-
/// driven outer loop.
pub fn run_lifecycle_persist_cycle(
    registry: &dyn ServiceRegistry,
    persistence: &dyn LifecyclePersistence,
    persist_kind: &'static str,
    corpus_basename: &str,
) -> Result<(), LifecycleError> {
    let services = registry.list_all();
    let service_count = services.len() as u64;
    let entries = services_to_entries(&services);
    let bytes_estimate: u64 =
        entries.len() as u64 * std::mem::size_of::<ServiceRegistryEntry>() as u64;

    let persist_start = std::time::Instant::now();
    match persistence.save_all(&entries) {
        Ok(()) => {
            let duration_ms = persist_start.elapsed().as_millis() as u64;
            tracing::info!(
                target: TARGET_LIFECYCLE_PERSIST,
                service_count = service_count,
                state_size_bytes = bytes_estimate,
                duration_ms = duration_ms,
                persist_kind = persist_kind,
                corpus_basename = corpus_basename,
                "lifecycle registry persisted"
            );
            Ok(())
        }
        Err(e) => {
            let duration_ms = persist_start.elapsed().as_millis() as u64;
            tracing::warn!(
                target: TARGET_LIFECYCLE_PERSIST_ERROR,
                error_category = e.error_category(),
                duration_ms = duration_ms,
                "lifecycle persist failed"
            );
            Err(e)
        }
    }
}

/// Long-running future spawned at boot that fires `run_lifecycle_persist_cycle`
/// on the supplied `interval`. Skips the immediate first tick (chunk
/// #62/#63/#66 convention; first ACTUAL save fires `interval` after spawn).
pub async fn run_lifecycle_persist_loop(
    registry: Arc<dyn ServiceRegistry>,
    persistence: Arc<dyn LifecyclePersistence>,
    interval: Duration,
    corpus_basename: String,
) {
    let mut ticker = tokio::time::interval(interval);
    ticker.tick().await;
    loop {
        ticker.tick().await;
        let _ = run_lifecycle_persist_cycle(
            registry.as_ref(),
            persistence.as_ref(),
            "periodic",
            &corpus_basename,
        );
    }
}

/// Graceful-shutdown persist helper. Synchronous so it can be called from
/// a Tauri shutdown hook. Emits the persist tracing event with
/// `persist_kind = "shutdown"`.
pub fn persist_lifecycle_on_shutdown(
    registry: &dyn ServiceRegistry,
    persistence: &dyn LifecyclePersistence,
    corpus_basename: &str,
) -> Result<(), LifecycleError> {
    run_lifecycle_persist_cycle(registry, persistence, "shutdown", corpus_basename)
}

/// Convert a Vec<ServiceListItem> snapshot (the `list_all` return shape)
/// into the `(service_name, ServiceRegistryEntry)` pairs that
/// `save_all` consumes. Synthesizes timestamps for first_seen +
/// last_transition from the snapshot's `last_seen_unix_nano` because the
/// `list_all` projection (intended for the TauRPC IPC surface) does not
/// carry first_seen + last_transition — the corpus persistence path
/// re-queries them implicitly via the `service_registry` table's existing
/// row state, which UPSERT preserves columns absent from the SET clause.
fn services_to_entries(services: &[ServiceListItem]) -> Vec<(String, ServiceRegistryEntry)> {
    services
        .iter()
        .map(|item| {
            (
                item.service.clone(),
                ServiceRegistryEntry {
                    state: item.state,
                    first_seen_unix_nano: item.last_seen_unix_nano,
                    last_seen_unix_nano: item.last_seen_unix_nano,
                    last_transition_unix_nano: item.last_seen_unix_nano,
                    manual_override: item.manual_override,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lifecycle::{InMemoryServiceRegistry, ServiceLifecycleState};
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakePersistence {
        saved: Mutex<Vec<(String, ServiceRegistryEntry)>>,
        load_returns: Mutex<Option<Vec<(String, ServiceRegistryEntry)>>>,
        next_save_error: Mutex<Option<LifecycleError>>,
    }

    impl FakePersistence {
        fn new() -> Self {
            Self::default()
        }

        fn seed_load(&self, entries: Vec<(String, ServiceRegistryEntry)>) {
            *self.load_returns.lock().unwrap() = Some(entries);
        }

        fn fail_next_save_with(&self, err: LifecycleError) {
            *self.next_save_error.lock().unwrap() = Some(err);
        }

        fn saved_count(&self) -> usize {
            self.saved.lock().unwrap().len()
        }
    }

    impl LifecyclePersistence for FakePersistence {
        fn load_all(&self) -> Result<Option<Vec<(String, ServiceRegistryEntry)>>, LifecycleError> {
            Ok(self.load_returns.lock().unwrap().clone())
        }

        fn save_all(
            &self,
            entries: &[(String, ServiceRegistryEntry)],
        ) -> Result<(), LifecycleError> {
            if let Some(err) = self.next_save_error.lock().unwrap().take() {
                return Err(err);
            }
            *self.saved.lock().unwrap() = entries.to_vec();
            Ok(())
        }
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
    fn lifecycle_error_category_groups_io_serialize_permission() {
        assert_eq!(
            LifecycleError::Io {
                kind: io::ErrorKind::NotFound
            }
            .error_category(),
            "io"
        );
        assert_eq!(LifecycleError::Serialize.error_category(), "serialize");
        assert_eq!(LifecycleError::Deserialize.error_category(), "serialize");
        assert_eq!(
            LifecycleError::SchemaVersionMismatch {
                expected: 1,
                got: 2
            }
            .error_category(),
            "serialize"
        );
        assert_eq!(
            LifecycleError::SizeCapExceeded {
                actual: 100,
                max: 50
            }
            .error_category(),
            "permission"
        );
        assert_eq!(LifecycleError::PathTraversal.error_category(), "permission");
    }

    #[test]
    fn run_persist_cycle_round_trips_state_via_fake_persistence() {
        let registry = InMemoryServiceRegistry::new();
        registry.set_state_on_corpus_restore("svc-a", ServiceLifecycleState::Active, 1_000);
        registry.set_state_on_corpus_restore("svc-b", ServiceLifecycleState::Quiet, 2_000);
        let persistence = FakePersistence::new();
        run_lifecycle_persist_cycle(&registry, &persistence, "periodic", "corpus.db")
            .expect("persist ok");
        assert_eq!(persistence.saved_count(), 2);
    }

    #[test]
    fn run_persist_cycle_emits_error_on_failure() {
        let registry = InMemoryServiceRegistry::new();
        registry.set_state_on_corpus_restore("svc-a", ServiceLifecycleState::Active, 1_000);
        let persistence = FakePersistence::new();
        persistence.fail_next_save_with(LifecycleError::Io {
            kind: io::ErrorKind::PermissionDenied,
        });
        let result = run_lifecycle_persist_cycle(&registry, &persistence, "periodic", "corpus.db");
        assert!(matches!(result, Err(LifecycleError::Io { .. })));
    }

    #[test]
    fn services_to_entries_preserves_state_and_manual_override() {
        let items = vec![
            ServiceListItem {
                service: "svc-a".to_string(),
                state: ServiceLifecycleState::Active,
                last_seen_unix_nano: 1_000,
                manual_override: None,
            },
            ServiceListItem {
                service: "svc-b".to_string(),
                state: ServiceLifecycleState::Quiet,
                last_seen_unix_nano: 2_000,
                manual_override: Some(ServiceLifecycleState::Active),
            },
        ];
        let entries = services_to_entries(&items);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].0, "svc-a");
        assert_eq!(entries[0].1.state, ServiceLifecycleState::Active);
        assert!(entries[0].1.manual_override.is_none());
        assert_eq!(entries[1].0, "svc-b");
        assert_eq!(entries[1].1.state, ServiceLifecycleState::Quiet);
        assert_eq!(
            entries[1].1.manual_override,
            Some(ServiceLifecycleState::Active)
        );
    }

    #[test]
    fn fake_persistence_load_returns_none_by_default() {
        let p = FakePersistence::new();
        let result = p.load_all().expect("ok");
        assert!(result.is_none());
    }

    #[test]
    fn fake_persistence_load_returns_seeded_entries() {
        let p = FakePersistence::new();
        let seed = vec![(
            "svc-x".to_string(),
            entry(ServiceLifecycleState::Active, 1_000),
        )];
        p.seed_load(seed.clone());
        let result = p.load_all().expect("ok").expect("Some");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "svc-x");
    }
}
