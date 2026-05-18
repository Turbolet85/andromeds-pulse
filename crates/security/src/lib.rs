//! Security primitives — chunk #68.
//!
//! Houses cross-consumer security building blocks. Currently exposes
//! [`scrubber`] (PII redaction primitive) consumed by:
//!
//! - `crates/corpus` at ingestion (capability P-047 — scrub before
//!   persistence per security plan §Logging NEVER-log discipline extended
//!   to a new persistence surface)
//! - `pulse-app/src/observability.rs` subscriber Layer for
//!   defense-in-depth before file write (per obs-plan §8 Integration
//!   points "At source (preferred)" + "At subscriber Layer
//!   (defense-in-depth)")
//!
//! Both layers MUST agree on the same scrubbing rules — single shared
//! crate-level primitive prevents pattern drift.

pub mod scrubber;
