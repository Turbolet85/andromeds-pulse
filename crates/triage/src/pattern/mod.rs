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
mod suppression;

pub use broadcast::{RestartEvent, RestartEventBroadcast, STREAM_NAME_RESTART_EVENTS};
pub use detector::{
    DEFAULT_HEARTBEAT_INTERVAL, DetectCycleStats, RestartDetector, observe_and_dispatch,
    run_one_detect_cycle, start_restart_detector,
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
