//! L4 LLM interpretation layer for andromeda-pulse v0.2.0 (route#82 onward).
//!
//! Exposes the `LlmInferenceRunner` async trait, supporting contract
//! types, the concrete `HardwareProfileDetector` implementing the chunk
//! #80 `HardwareProfileSource` trait declared by `triage::contract`,
//! and the `pulse://stream/model-status` broadcast topic wrapper.
//! Concrete mistralrs implementation lives at the binary boundary
//! (in `pulse-app/`) per the LLM Inference Runtime bus factor mitigation
//! entry recorded under arch §Established Decisions.
//!
//! Module structure: `contract` exposes the public types crossing crate
//! boundary (LlmInferenceRunner trait, ModelTier, ModelStatus,
//! ModelIdentity, ModelLoadEvent payloads, InferenceError enum);
//! `hardware` exposes the concrete `HardwareProfileDetector` reading
//! host hardware characteristics; `broadcast` exposes the
//! `pulse://stream/model-status` topic wrapper.

pub mod broadcast;
pub mod contract;
pub mod degraded_mode;
pub mod hardware;
pub mod prompt;
pub mod schema;
