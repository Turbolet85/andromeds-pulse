//! Attention cue emitter — chunk #62.
//!
//! L1 algorithmic detection layer per pulse v0.2.0 plan Phase 2. The 1-second
//! background tick task spawned at boot reads streaming baseline trackers
//! (chunk #61 EwmaTracker + TDigestPair + RollingWindow exposed via
//! `triage::contract`), compares current values against threshold multipliers
//! (3.0× error rate, 2.5× latency), classifies above-threshold deviations
//! into a `PriorityTier` (Autonomous / Suggested / Curious per chunk #60
//! contract), and emits `AttentionCue` payloads to a new broadcast topic
//! `pulse://stream/attention-cues`. Tier-2 (`Suggested`) cues additionally
//! fan out to a `cadence-triggers` channel for the future Cadence Coordinator
//! (chunk #72).
//!
//! Capability spec coverage: P-021 (Algorithmic Attention Cues) +
//! P-019 partial (PriorityTier classification — three-tier severity model).
//! Threshold multipliers loaded from a `Thresholds` config struct с
//! hardcoded defaults this chunk; hot-reload wiring lands in chunk #86.

mod broadcast;
mod classify;
mod emitter;
mod evaluate;
mod thresholds;

pub use broadcast::{
    AttentionCueBroadcast, BROADCAST_CAPACITY, CHANNEL_NAME_CADENCE_TRIGGERS,
    CadenceTriggerChannel, STREAM_NAME_ATTENTION_CUES,
};
pub use classify::{classify_priority, dual_condition_bypass};
pub use emitter::{run_one_emit_cycle, start_emitter};
pub use evaluate::evaluate_thresholds;
pub use thresholds::{
    DEFAULT_BASE_ERROR_RATE, DEFAULT_BASE_LATENCY_MS, DEFAULT_ERROR_RATE_MULTIPLIER,
    DEFAULT_LATENCY_MULTIPLIER, DEFAULT_LATENCY_PERCENTILE, DEFAULT_MIN_PERSISTENCE_SECONDS,
    DEFAULT_TICK_INTERVAL, MIN_EWMA_SAMPLES, Thresholds, ThresholdsError,
};

pub(crate) const TARGET_CUE_TICK: &str = "triage.cue.tick";
pub(crate) const TARGET_CUE_EVALUATE: &str = "triage.cue.evaluate";
pub(crate) const TARGET_CUE_EMIT: &str = "triage.cue.emit";
pub(crate) const TARGET_METRIC_CUE_EMIT_COUNT: &str = "metric.cue.emit_count_total";
