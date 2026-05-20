//! Baseline state persistence trait (chunk #70).
//!
//! Defines the abstraction `BaselineState` traverses to reach durable
//! storage. Implementation lives at `pulse-app/src/baseline_persistence.rs`
//! wrapping `corpus::contract::CorpusWriter` (Schema Option A — reuses the
//! `pipeline_metrics` blob slot with `metric_name="baseline_state"` +
//! `layer="l1b"`, mirroring chunk #69's `CorpusDrainPersistence`).
//!
//! Trait defined in this lower (triage) crate per CLAUDE.md session-learnings
//! 2026-05-16 trait-in-lower-crate pattern; preserves the arch DAG (no
//! `triage → corpus` dep edge — the adapter at the pulse-app binary
//! boundary owns both).
//!
//! Send + Sync bounds required because `Arc<dyn BaselinePersistence>` is
//! cross-spawned into the persist loop tokio task.

use super::{BaselineError, BaselineState};

pub trait BaselinePersistence: Send + Sync {
    /// Load a previously-saved state. `Ok(None)` means no prior state
    /// (cold-start). Errors surface to caller for tracing emission;
    /// callers should treat as "proceed with empty state" rather than
    /// fatal (boot is non-fatal per chunk #68 corpus precedent +
    /// chunk #69 Drain precedent).
    fn load(&self) -> Result<Option<BaselineState>, BaselineError>;

    /// Save the current state snapshot. Implementation is responsible
    /// for serializing + delegating to the corpus encryption layer
    /// (AES-256-GCM at-rest per chunk #68). Errors surface to caller
    /// for tracing emission at the `triage.baseline.persist.error`
    /// target.
    fn save(&self, state: &BaselineState) -> Result<(), BaselineError>;
}
