//! Boot-time `HardwareProfileSource` adapter (chunk #80; updated chunk #82).
//!
//! Chunk #80 substrate: returned `HardwareProfile::Unknown` until chunk
//! #82 lands the real detector; coordinator treats `Unknown` as
//! Tier-2-enabled per security plan §Error Handling boundary discipline
//! (defense-in-depth: missing profile source defaults к the safe posture).
//!
//! Chunk #82 swap: re-exports `interpretation::hardware::HardwareProfileDetector`
//! as the canonical boot detector. The chunk #80 `UnknownHardwareProfile`
//! re-export stays available as а safe fallback for tests + degraded
//! environments (per CLAUDE.md 2026-05-16 trait-in-lower-crate session
//! learning pattern: trait + safe-default-impl in lower crate; binary
//! boundary chooses concrete impl).

pub use interpretation::hardware::HardwareProfileDetector;
pub use triage::contract::UnknownHardwareProfile;
