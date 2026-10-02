//! Baseline state persistence constants + bootstrap helper (chunk #70).
//!
//! Flat-file persistence (chunk #61: `<data-dir>/triage/baseline-corpus.bin`)
//! is eliminated in chunk #70. Persistence now routes through the
//! [`super::BaselinePersistence`] trait, implemented at
//! `pulse-app/src/baseline_persistence.rs` over corpus SQLite (chunk #68
//! substrate). This module retains the size / count constants + the
//! `BootstrapResult` + `PersistStats` value shapes referenced by
//! `run_persist_cycle` + `bootstrap_state` in [`super`].

use super::{BaselineError, BaselinePersistence, BaselineState, SCHEMA_VERSION};

/// Legacy bincode size cap surfaced for the migration helper in
/// `pulse-app/src/baseline_persistence.rs::migrate_legacy_baseline_if_present`.
/// Bounded to 1 MB — sufficient for the maximum expected
/// `DEFAULT_SERVICE_COUNT_CAP × per-service bincode shape` AND defensive
/// against malicious / corrupted legacy files. Corpus SQLite cells are
/// bound separately at the corpus crate boundary; this constant is for the
/// transient migration read path only.
pub const DEFAULT_MAX_SIZE_BYTES: u64 = 1024 * 1024;

/// Per-instance ceiling on `BaselineState::services` cardinality —
/// `observe_span` drops new-service spans once `services.len() ==
/// DEFAULT_SERVICE_COUNT_CAP`. Mirror constant in the lower-tier
/// `ACTIVITY_FLOOR_SERVICE_CAP` ensures both the in-process cap and the
/// persisted-state cap reject the same number.
pub const DEFAULT_SERVICE_COUNT_CAP: usize = 1024;

#[derive(Debug, Clone, Copy)]
pub struct PersistStats {
    pub bytes_written: u64,
    pub service_count: usize,
}

#[derive(Debug)]
pub enum BootstrapResult {
    /// Corpus loaded successfully; `age_nanos` is the elapsed time since
    /// `persisted_at_unix_nanos` was last written. Caller decides whether to
    /// keep loaded state or fall through to fresh based on age threshold.
    Loaded {
        state: BaselineState,
        age_nanos: i64,
    },
    /// No usable corpus; caller should bootstrap fresh state. `reason_kind`
    /// is one of "cold_start" / "corpus_corrupt_reset" / "schema_mismatch"
    /// / "service_count_cap_exceeded" for the
    /// `pipeline.l1b.bootstrap_count_total{kind}` metric label.
    Fresh { reason_kind: &'static str },
}

/// Attempt to bootstrap a `BaselineState` from the supplied persistence
/// trait object. Maps `BaselinePersistence::load` results onto the
/// `BootstrapResult` shape that `bootstrap_state` in [`super`] consumes for
/// the `pipeline.l1b.bootstrap_count_total{kind}` metric label.
///
/// `service_count_cap` is enforced post-deserialize: if a corpus row
/// somehow encoded more services than the in-process cap allows, treat as
/// `service_count_cap_exceeded` rather than load the oversized state
/// (defensive — corpus SQLite has its own cell size cap, but the per-
/// instance service cap is the runtime guarantee).
pub fn bootstrap_from_persistence(
    persistence: &dyn BaselinePersistence,
    service_count_cap: usize,
    now_nanos: i64,
) -> BootstrapResult {
    match persistence.load() {
        Ok(None) => BootstrapResult::Fresh {
            reason_kind: "cold_start",
        },
        Ok(Some(state)) => {
            if state.schema_version() != SCHEMA_VERSION {
                return BootstrapResult::Fresh {
                    reason_kind: "schema_mismatch",
                };
            }
            if state.service_count() > service_count_cap {
                return BootstrapResult::Fresh {
                    reason_kind: "service_count_cap_exceeded",
                };
            }
            let persisted_at = state.persisted_at_unix_nanos();
            let age_nanos = now_nanos.saturating_sub(persisted_at);
            BootstrapResult::Loaded { state, age_nanos }
        }
        Err(BaselineError::SchemaVersionMismatch { .. }) => BootstrapResult::Fresh {
            reason_kind: "schema_mismatch",
        },
        Err(BaselineError::ServiceCountCapExceeded { .. }) => BootstrapResult::Fresh {
            reason_kind: "service_count_cap_exceeded",
        },
        Err(_) => BootstrapResult::Fresh {
            reason_kind: "corpus_corrupt_reset",
        },
    }
}
