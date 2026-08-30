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
//! Threshold multipliers loaded from a `Thresholds` config struct with
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
pub use emitter::{
    CUE_LATCH_REFRACTORY_NANOS, CueLatch, LatchOutcome, run_one_emit_cycle, start_emitter,
};
pub use evaluate::{evaluate_service_went_silent, evaluate_thresholds};
pub use thresholds::{
    DEFAULT_ABSOLUTE_BYPASS_ERROR_RATE, DEFAULT_ABSOLUTE_BYPASS_LATENCY_MS,
    DEFAULT_BASE_ERROR_RATE, DEFAULT_BASE_LATENCY_MS, DEFAULT_BOOTSTRAP_WINDOW_SECONDS,
    DEFAULT_ERROR_RATE_MULTIPLIER, DEFAULT_LATENCY_MULTIPLIER, DEFAULT_LATENCY_PERCENTILE,
    DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER, DEFAULT_MIN_PERSISTENCE_SECONDS,
    DEFAULT_QUIET_DURATION_PERCENTILE, DEFAULT_RESTART_GAP_THRESHOLD_SECONDS,
    DEFAULT_RESTART_SUPPRESSION_WINDOW_SECONDS, DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SAMPLES,
    DEFAULT_TICK_INTERVAL, MIN_EWMA_SAMPLES, MIN_LATENCY_SAMPLES, Thresholds, ThresholdsError,
};

pub(crate) const TARGET_CUE_TICK: &str = "triage.cue.tick";
pub(crate) const TARGET_CUE_EVALUATE: &str = "triage.cue.evaluate";
pub(crate) const TARGET_CUE_EMIT: &str = "triage.cue.emit";
pub(crate) const TARGET_METRIC_CUE_EMIT_COUNT: &str = "metric.cue.emit_count_total";
/// Per-cue surgical-suppression decision event (chunk #63) — fires every
/// tick per evaluated cue carrying the decision inputs (`cue_kind`,
/// `persistence`, `restart_window_active`, `suppression_bypassed`,
/// `bypass_reason`). Drives observability of the restart-window surgical
/// suppression posture per capability P-016.
pub(crate) const TARGET_CUE_SUPPRESSION_CHECK: &str = "triage.cue.suppression_check";
/// Per-bypass-trigger event (chunk #63) — fires when a cue survives
/// surgical suppression via dual-condition bypass (P-057). Paired with
/// the `metric.pipeline.l2.magnitude_bypass_triggered_total` metric stream.
pub(crate) const TARGET_CUE_SUPPRESSION_BYPASS: &str = "triage.cue.suppression_bypass";

/// Per-tick aggregate evaluation event for chunk #64 ServiceWentSilent
/// gating. Fields cover counts only (services_tracked / services_in_bootstrap
/// / services_ready / silence_cues_emitted) — no per-service identifiers
/// per chunk #62/#63 PII discipline (`service.name` is OTLP-attribute-
/// derived user-content; never logged in self-observation events).
pub(crate) const TARGET_SERVICE_WENT_SILENT_EVALUATE: &str =
    "triage.baseline.service_went_silent.evaluate";

/// Per-tick activity-floor bootstrap-state gauge for chunk #64. Aggregate
/// only — fields cover learning/ready counts and total tracked services.
pub(crate) const TARGET_METRIC_BOOTSTRAP_STATE: &str =
    "metric.triage.activity_floor.bootstrap_state";
