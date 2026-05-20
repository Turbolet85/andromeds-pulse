//! Pattern detection — chunk #63.
//!
//! L1b algorithmic detection layer per pulse v0.2.0 plan Phase 2. The
//! `RestartDetector` tracks per-service last-seen OTLP span timestamps;
//! when a service's gap between observations exceeds the configurable
//! threshold (default 20s per chunk spec) and is followed by a resume
//! observation, the detector emits a `RestartEvent` to broadcast topic
//! `pulse://stream/restart-events`. The cue emitter (chunk #62) consumes
//! restart events into per-service `SuppressionState` windows that gate
//! short-persistence `ErrorRateSpike` cues during the 60s post-restart
//! window — EXCEPT when the dual-condition magnitude bypass (P-057)
//! trips, ensuring catastrophic regressions remain visible during
//! restart-induced noise.
//!
//! Capability spec coverage: P-015 (Restart Event Detection) +
//! P-016 (Restart-Window Suppression Surgical) + P-057 (Dual-Condition
//! Suppression Bypass).
//!
//! Thresholds live on the `crate::cue::Thresholds` struct (extended
//! this chunk with 6 new fields covering bypass multipliers + window
//! durations). Hot-reload wiring lands in chunk #86.

mod broadcast;
mod detector;
mod persistence;
mod storm;
mod suppression;

pub use broadcast::{RestartEvent, RestartEventBroadcast, STREAM_NAME_RESTART_EVENTS};
pub use detector::{
    DEFAULT_HEARTBEAT_INTERVAL, DetectCycleStats, RestartDetector, observe_and_dispatch,
    run_one_detect_cycle, start_restart_detector,
};
pub use persistence::{
    DEFAULT_STORM_PERSIST_INTERVAL_SECS, STORM_PERSISTENCE_KIND, StormError, StormPersistence,
    StormStateSnapshot, TARGET_PATTERN_STORM_CORPUS_RESTORE, TARGET_PATTERN_STORM_PERSIST,
    TARGET_PATTERN_STORM_PERSIST_ERROR, persist_storm_on_shutdown, run_storm_persist_cycle,
    run_storm_persist_loop,
};
pub use storm::{
    DEFAULT_AUTONOMOUS_THRESHOLD, DEFAULT_DETECTION_SUB_WINDOW_SECONDS,
    DEFAULT_STORM_WINDOW_SECONDS, DEFAULT_SUGGESTED_THRESHOLD, FingerprintState,
    RetryStormDetector, StormCycleStats, observe_and_dispatch_storm, record_occurrence,
    run_one_storm_cycle, start_storm_detector,
};
pub use suppression::{
    BypassReason, BypassTrigger, SuppressionOutcome, SuppressionParams, SuppressionState,
    evaluate_with_suppression,
};

pub(crate) const TARGET_PATTERN_TICK: &str = "triage.pattern.tick";
pub(crate) const TARGET_PATTERN_RESTART_DETECT: &str = "triage.pattern.restart_detect";
pub(crate) const TARGET_PATTERN_RESTART_EMIT: &str = "triage.pattern.restart_emit";
pub(crate) const TARGET_METRIC_MAGNITUDE_BYPASS: &str =
    "metric.pipeline.l2.magnitude_bypass_triggered_total";
// Chunk #66 — retry storm detector tracing targets. Aggregate-only fields
// per AllowList convention established by chunks #62/#63/#64 (no per-service
// identifiers in self-observation events).
pub(crate) const TARGET_PATTERN_STORM_DETECTED: &str = "triage.pattern.storm.detected";
pub(crate) const TARGET_PATTERN_STORM_EMIT: &str = "triage.pattern.storm.emit";
pub(crate) const TARGET_PATTERN_STORM_TICK: &str = "triage.pattern.storm.tick";
pub(crate) const TARGET_METRIC_STORM_DETECTED_COUNT: &str =
    "metric.triage.pattern.storm_detected_count";
pub(crate) const TARGET_METRIC_FINGERPRINTS_TRACKED: &str =
    "metric.triage.pattern.fingerprints_tracked";
pub(crate) const TARGET_METRIC_FINGERPRINT_EVICTED_COUNT: &str =
    "metric.triage.pattern.fingerprint_evicted_count";
