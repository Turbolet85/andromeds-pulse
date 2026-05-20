//! Retry storm persistence trait (chunk #71).
//!
//! Defines the abstraction `RetryStormDetector` traverses to reach durable
//! storage. Implementation lives at `pulse-app/src/storm_persistence.rs`
//! wrapping `corpus::contract::CorpusWriter` — Schema choice: reuse the
//! `pipeline_metrics` blob slot (chunk #68 schema, no version bump) with
//! `metric_name = "storm_state"`, `layer = "l2"`, `payload =
//! bincode-serialized StormStateSnapshot`. Cell-level AES-256-GCM
//! encryption is applied by the corpus crate transparently to the
//! adapter; serialization → encryption → INSERT is the save path.
//!
//! Trait defined in this lower (triage) crate per CLAUDE.md session-learnings
//! 2026-05-16 trait-in-lower-crate pattern; preserves the arch DAG (no
//! `triage → corpus` dep edge — the adapter at the pulse-app binary
//! boundary owns both).
//!
//! Send + Sync bounds required because `Arc<dyn StormPersistence>` is
//! cross-spawned into the periodic persist task.

use std::io;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

use super::storm::{FingerprintState, RetryStormDetector};

/// Default cadence for the periodic storm persist loop. Mirrors the
/// chunk #70 baseline persist cadence so all corpus writers share one
/// 60s tick rhythm. Per the chunk #71 plan deferred decision, higher-
/// frequency persist (e.g. 5-10s) is a follow-up tuning chunk.
pub const DEFAULT_STORM_PERSIST_INTERVAL_SECS: u64 = 60;

/// Tracing target for successful storm persist events. Aggregate-only
/// fields per CLAUDE.md 2026-05-17 session 84 + chunk #62/#63/#64 triage
/// AllowList convention.
pub const TARGET_PATTERN_STORM_PERSIST: &str = "triage.pattern.storm.persist";

/// Tracing target for storm persist failures. Sanitized error_category
/// + duration_ms only.
pub const TARGET_PATTERN_STORM_PERSIST_ERROR: &str = "triage.pattern.storm.persist.error";

/// Tracing target for storm corpus-restore events emitted at boot when
/// `load()` returns `Ok(Some(snapshot))`. Aggregate-only fields:
/// `restored_fingerprint_count` + `duration_ms` + `kind`.
pub const TARGET_PATTERN_STORM_CORPUS_RESTORE: &str = "triage.pattern.storm.corpus_restore";

/// Stable `persist_kind` field value emitted on `TARGET_PATTERN_STORM_PERSIST`.
pub const STORM_PERSISTENCE_KIND: &str = "storm";

/// Sanitized error envelope for storm persistence operations. Mirrors
/// `BaselineError` / `LifecycleError` shape — same variants so cross-crate
/// error mapping at the binary boundary stays uniform across all
/// corpus-backed persistence paths.
#[derive(Debug, Error)]
pub enum StormError {
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

impl StormError {
    pub fn error_category(&self) -> &'static str {
        match self {
            Self::Io { .. } => "io",
            Self::Serialize | Self::Deserialize | Self::SchemaVersionMismatch { .. } => "serialize",
            Self::SizeCapExceeded { .. } | Self::PathTraversal => "permission",
        }
    }
}

/// Serializable snapshot of the `RetryStormDetector` state. `DashMap` is
/// not serde-able directly; this snapshot materializes the per-fingerprint
/// entries as a `Vec<([u8; 16], FingerprintState)>` plus the detector's
/// configuration knobs so restore can rebuild the exact same detector
/// instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StormStateSnapshot {
    pub entries: Vec<([u8; 16], FingerprintState)>,
    pub window_seconds: u64,
    pub detection_window_seconds: u64,
    pub suggested_threshold: u64,
    pub autonomous_threshold: u64,
}

/// Abstraction over durable storage for `RetryStormDetector`.
/// `load` returns `Ok(None)` for cold-start (no prior storm_state blob);
/// `Ok(Some(snapshot))` for resume-from-corpus. `save` writes the
/// full snapshot via `save_pipeline_metric`.
pub trait StormPersistence: Send + Sync {
    /// Load the most recent persisted storm-state snapshot. `Ok(None)`
    /// means no prior state (cold-start path; corpus has no row for
    /// `(metric_name="storm_state", layer="l2")`). Errors surface to
    /// caller for tracing emission; callers should treat as
    /// "proceed with fresh detector" rather than fatal.
    fn load(&self) -> Result<Option<StormStateSnapshot>, StormError>;

    /// Persist the current detector snapshot. Implementation bincode-
    /// serializes + delegates to the corpus encryption layer
    /// (AES-256-GCM at-rest per chunk #68). Errors surface to caller
    /// for tracing emission at `triage.pattern.storm.persist.error`.
    fn save(&self, snapshot: &StormStateSnapshot) -> Result<(), StormError>;
}

/// Synchronous one-pass persist cycle: snapshot the detector, save via
/// the trait, emit aggregate tracing events. Returns the persist outcome
/// for callers that want explicit error handling.
pub fn run_storm_persist_cycle(
    detector: &RetryStormDetector,
    persistence: &dyn StormPersistence,
    persist_kind: &'static str,
    corpus_basename: &str,
) -> Result<(), StormError> {
    let snapshot = detector.snapshot();
    let fingerprint_count = snapshot.entries.len() as u64;
    let bytes_estimate: u64 = bincode::serialize(&snapshot)
        .map(|v| v.len() as u64)
        .unwrap_or(0);

    let persist_start = std::time::Instant::now();
    match persistence.save(&snapshot) {
        Ok(()) => {
            let duration_ms = persist_start.elapsed().as_millis() as u64;
            info!(
                target: TARGET_PATTERN_STORM_PERSIST,
                fingerprint_count = fingerprint_count,
                state_size_bytes = bytes_estimate,
                duration_ms = duration_ms,
                persist_kind = persist_kind,
                corpus_basename = corpus_basename,
                "storm state persisted"
            );
            Ok(())
        }
        Err(e) => {
            let duration_ms = persist_start.elapsed().as_millis() as u64;
            tracing::warn!(
                target: TARGET_PATTERN_STORM_PERSIST_ERROR,
                error_category = e.error_category(),
                duration_ms = duration_ms,
                "storm persist failed"
            );
            Err(e)
        }
    }
}

/// Long-running future spawned at boot that fires `run_storm_persist_cycle`
/// on the supplied `interval`. Skips the immediate first tick (chunk
/// #62/#63/#66 convention; first ACTUAL save fires `interval` after spawn).
pub async fn run_storm_persist_loop(
    detector: Arc<RetryStormDetector>,
    persistence: Arc<dyn StormPersistence>,
    interval: Duration,
    corpus_basename: String,
) {
    let mut ticker = tokio::time::interval(interval);
    ticker.tick().await;
    loop {
        ticker.tick().await;
        let _ = run_storm_persist_cycle(
            detector.as_ref(),
            persistence.as_ref(),
            "periodic",
            &corpus_basename,
        );
    }
}

/// Graceful-shutdown persist helper. Synchronous so it can be called
/// from a Tauri shutdown hook. Emits the persist tracing event with
/// `persist_kind = "shutdown"`.
pub fn persist_storm_on_shutdown(
    detector: &RetryStormDetector,
    persistence: &dyn StormPersistence,
    corpus_basename: &str,
) -> Result<(), StormError> {
    run_storm_persist_cycle(detector, persistence, "shutdown", corpus_basename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakeStormPersistence {
        saved: Mutex<Option<StormStateSnapshot>>,
        load_returns: Mutex<Option<StormStateSnapshot>>,
        next_save_error: Mutex<Option<StormError>>,
    }

    impl FakeStormPersistence {
        fn new() -> Self {
            Self::default()
        }

        fn seed_load(&self, snapshot: StormStateSnapshot) {
            *self.load_returns.lock().unwrap() = Some(snapshot);
        }

        fn fail_next_save_with(&self, err: StormError) {
            *self.next_save_error.lock().unwrap() = Some(err);
        }

        fn saved(&self) -> Option<StormStateSnapshot> {
            self.saved.lock().unwrap().clone()
        }
    }

    impl StormPersistence for FakeStormPersistence {
        fn load(&self) -> Result<Option<StormStateSnapshot>, StormError> {
            Ok(self.load_returns.lock().unwrap().clone())
        }

        fn save(&self, snapshot: &StormStateSnapshot) -> Result<(), StormError> {
            if let Some(err) = self.next_save_error.lock().unwrap().take() {
                return Err(err);
            }
            *self.saved.lock().unwrap() = Some(snapshot.clone());
            Ok(())
        }
    }

    #[test]
    fn storm_error_category_groups_io_serialize_permission() {
        assert_eq!(
            StormError::Io {
                kind: io::ErrorKind::NotFound
            }
            .error_category(),
            "io"
        );
        assert_eq!(StormError::Serialize.error_category(), "serialize");
        assert_eq!(StormError::Deserialize.error_category(), "serialize");
        assert_eq!(
            StormError::SchemaVersionMismatch {
                expected: 1,
                got: 2
            }
            .error_category(),
            "serialize"
        );
        assert_eq!(
            StormError::SizeCapExceeded {
                actual: 100,
                max: 50
            }
            .error_category(),
            "permission"
        );
        assert_eq!(StormError::PathTraversal.error_category(), "permission");
    }

    #[test]
    fn storm_state_snapshot_round_trips_through_bincode() {
        let snapshot = StormStateSnapshot {
            entries: vec![],
            window_seconds: 60,
            detection_window_seconds: 30,
            suggested_threshold: 5,
            autonomous_threshold: 10,
        };
        let bytes = bincode::serialize(&snapshot).expect("serialize");
        let parsed: StormStateSnapshot = bincode::deserialize(&bytes).expect("deserialize");
        assert_eq!(parsed, snapshot);
    }

    #[test]
    fn run_persist_cycle_invokes_save_via_fake_persistence() {
        let detector = RetryStormDetector::new(60, 30, 5, 10);
        let persistence = FakeStormPersistence::new();
        run_storm_persist_cycle(&detector, &persistence, "periodic", "corpus.db")
            .expect("persist ok");
        let saved = persistence.saved().expect("Some");
        assert_eq!(saved.window_seconds, 60);
    }

    #[test]
    fn run_persist_cycle_emits_error_on_failure() {
        let detector = RetryStormDetector::new(60, 30, 5, 10);
        let persistence = FakeStormPersistence::new();
        persistence.fail_next_save_with(StormError::Io {
            kind: io::ErrorKind::PermissionDenied,
        });
        let result = run_storm_persist_cycle(&detector, &persistence, "periodic", "corpus.db");
        assert!(matches!(result, Err(StormError::Io { .. })));
    }

    #[test]
    fn fake_persistence_load_returns_none_by_default() {
        let p = FakeStormPersistence::new();
        let result = p.load().expect("ok");
        assert!(result.is_none());
    }

    #[test]
    fn fake_persistence_load_returns_seeded_snapshot() {
        let p = FakeStormPersistence::new();
        let seed = StormStateSnapshot {
            entries: vec![],
            window_seconds: 90,
            detection_window_seconds: 45,
            suggested_threshold: 3,
            autonomous_threshold: 7,
        };
        p.seed_load(seed.clone());
        let result = p.load().expect("ok").expect("Some");
        assert_eq!(result.window_seconds, 90);
        assert_eq!(result.detection_window_seconds, 45);
    }
}
