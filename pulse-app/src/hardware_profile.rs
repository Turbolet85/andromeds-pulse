//! Boot-time `HardwareProfileSource` adapter (chunk #80). Returns
//! `HardwareProfile::Unknown` until chunk #82 lands the real detector;
//! coordinator treats `Unknown` as Tier-2-enabled per security plan
//! §Error Handling boundary discipline (defense-in-depth: missing
//! profile source defaults к the safe posture).
//!
//! Future chunk #82 replaces this stub с а real detector implementation
//! reading GPU / CPU / memory characteristics.

pub use triage::contract::UnknownHardwareProfile;
