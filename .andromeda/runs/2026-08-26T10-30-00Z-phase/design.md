# design extract

## No domain coverage

This chunk is entirely backend Rust — bounding cue-driven `cadence.trigger` frequency in `crates/triage/src/cadence/` (plus `crates/triage/src/cue/`) and the tokio blocking-pool contention it causes at `pulse-app/src/main.rs:276-279`; no surface in the scope's "Surfaces and contracts in frame" table renders UI, so no design-system token, typography, motion, iconography, or component-pattern mandate applies. (The word "cadence" here is the triage Q1–Q7 cycle coordinator, not the Halo State Pulse breathing cadence of design-system.md §Motion, which is driven by activity state on a WebGPU canvas layer and is untouched by this chunk.)
