//! Cadence coordinator + three-tier triggering (chunk #80).
//!
//! L2 routing + L3 invocation orchestration layer per
//! `docs/v0_2_0/pulse-distillation-architecture.md` §Cadence and Event
//! Triggers. Three concurrent schedulers (Tier-1 immediate / Tier-2
//! accelerated / Tier-3 baseline) plus a reflection-mode tick orchestrate
//! L1a SQL query execution + L3 digest assembly stub + L4 inference
//! invocation per attention cue priority tier. Tier-2 acceleration is
//! conditionally disabled when the hardware profile is `cpu-primary`
//! (forward-binding to chunk #82).
//!
//! Capabilities: P-052 (Cadence Configuration) + P-060 (Tiered Triggering
//! Priority). See pulse-v0_2_0-route.md §80 for chunk-level detail.
//!
//! Default cadence values: baseline 60s / accelerated 20s / reflection
//! 1800s. Safety floors enforced at config-load: baseline ≥ 5s,
//! accelerated ≥ 1s, reflection ≥ 300s per pulse-v0_2_0-route §80.
//!
//! New broadcast topic `pulse://stream/cadence-events` carries L6
//! visibility events for future consumer chunks; no TauRPC procedure is
//! introduced this chunk (capability-drift gate stays clean).

mod broadcast;
mod config;
mod coordinator;

pub use broadcast::{CadenceEvent, CadenceEventBroadcast, STREAM_NAME_CADENCE_EVENTS};
pub use config::{
    CADENCE_ACCELERATED_SECONDS_MIN, CADENCE_BASELINE_SECONDS_MIN, CADENCE_REFLECTION_SECONDS_MIN,
    CadenceConfig, CadenceConfigError, DEFAULT_CADENCE_ACCELERATED_SECONDS,
    DEFAULT_CADENCE_BASELINE_SECONDS, DEFAULT_CADENCE_REFLECTION_SECONDS,
};
pub use coordinator::{
    CadenceCoordinator, CadenceMode, CoordinatorCycleStats, HardwareProfile, HardwareProfileSource,
    SqlQueryRunner, UnknownHardwareProfile, mode_label, run_one_coordinator_cycle,
    start_cadence_coordinator,
};

pub(crate) const TARGET_CADENCE_TICK: &str = "cadence.tick";
pub(crate) const TARGET_CADENCE_TRIGGER: &str = "cadence.trigger";
pub(crate) const TARGET_METRIC_PIPELINE_L3_DIGESTS_ASSEMBLED_TOTAL: &str =
    "metric.pipeline.l3.digests_assembled_total";
// `cadence.config.load` + `cadence.config.safety_floor` are emitted from
// `pulse-app/src/main.rs` boot wiring (Settings-load path) — string
// literal at emit site mirrors chunk #62 precedent for emit-only targets
// owned by а different crate than the consumer.
