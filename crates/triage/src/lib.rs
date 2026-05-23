//! Triage crate — Pulse v0.2.0 L1 streaming distillation layer
//! (capability spec P-019 Three-Tier Severity Model + P-021 Algorithmic
//! Attention Cues). Public surface lives in `triage::contract`; internal
//! modules (`baseline`, `cadence`, `cue`, `digest`, `incident`,
//! `interpretation`, `lifecycle`, `pattern`) are `pub(crate)` per arch
//! §Project Intent Template patterns. Module skeletons at scaffold stage;
//! consumed by chunks #61+ per pulse v0.2.0 plan Phase 2.

pub(crate) mod baseline;
pub(crate) mod cadence;
pub mod contract;
pub(crate) mod cue;
pub(crate) mod digest;
pub(crate) mod incident;
pub(crate) mod interpretation;
pub(crate) mod lifecycle;
pub(crate) mod pattern;
