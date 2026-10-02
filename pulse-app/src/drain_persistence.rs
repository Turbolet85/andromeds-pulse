//! Corpus-backed Drain persistence adapter — chunk #69 Phase B Session 4.
//!
//! Wires `buffer::DrainPersistence` to `corpus::contract::CorpusWriter`
//! at the pulse-app binary boundary, preserving the arch §Module
//! dependency direction DAG (buffer stays corpus-free; corpus stays
//! buffer-free; pulse-app owns the wire-up). Mirrors chunk #59's
//! `HeartbeatBindStatus` trait-in-lower-crate + adapter-in-pulse-app
//! pattern (per session-learnings 2026-05-16).
//!
//! Persistence cell shape per chunk #69 plan Open Question Q1 option (a):
//! re-uses the existing `pipeline_metrics` SQLite table (chunk #68
//! schema, no version bump) with `metric_name = "drain_template_tree"`,
//! `layer = "l1c"`, `payload = bincode-serialized DrainState`.
//! Cell-level AES-256-GCM encryption is applied by the corpus crate
//! transparently to this adapter; serialization → encryption →
//! INSERT is the save path; SELECT → decryption → deserialization is
//! the load path.
//!
//! PII discipline: the DrainState payload is built from already-PII-
//! scrubbed template content (the appender / DuckDB write path calls
//! `security::scrubber::scrub_attribute` before storing a template;
//! the buffer-side `DrainMiner` operates on those scrubbed strings).
//! Encryption is defense-in-depth for the at-rest threat model, not a
//! substitute for that producer-side scrubbing.

use std::sync::Arc;

use buffer::{DrainPersistence, DrainState, Error as BufferError};
use corpus::contract::{CorpusWriter, Error as CorpusError};

/// Stable `metric_name` value in `pipeline_metrics` for the Drain
/// template tree. Changing this string breaks in-place persistence
/// reads against existing on-disk corpora — schema-equivalent renaming
/// would require a migration path.
pub const DRAIN_TEMPLATE_METRIC_NAME: &str = "drain_template_tree";

/// Stable `layer` tag in `pipeline_metrics`. Per pulse v0.2.0
/// distillation architecture, `l1c` is the log-template-mining layer
/// (DrainMiner output sits there).
pub const DRAIN_PERSISTENCE_LAYER: &str = "l1c";

/// Adapter implementing `buffer::DrainPersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusDrainPersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusDrainPersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl DrainPersistence for CorpusDrainPersistence {
    fn load(&self) -> Result<Option<DrainState>, BufferError> {
        let bytes_opt = self
            .writer
            .load_pipeline_metric(DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER)
            .map_err(corpus_error_to_buffer_error)?;
        match bytes_opt {
            Some(bytes) => {
                let state: DrainState =
                    crate::bincode_bounded::deserialize(&bytes).map_err(|_| {
                        BufferError::Drain {
                            reason: "drain state bincode decode failed".to_string(),
                        }
                    })?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    fn save(&self, state: &DrainState) -> Result<(), BufferError> {
        let bytes = bincode::serialize(state).map_err(|_| BufferError::Drain {
            reason: "drain state bincode encode failed".to_string(),
        })?;
        self.writer
            .save_pipeline_metric(DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER, &bytes)
            .map_err(corpus_error_to_buffer_error)
    }
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the message surfaced upward to the buffer
/// layer (the buffer Error::Drain.reason field IS visible via
/// `From<BufferError> for AppError` IPC propagation, so noise is risk).
#[doc(hidden)]
pub fn corpus_error_to_buffer_error(err: CorpusError) -> BufferError {
    let reason = match err {
        CorpusError::KeyringUnavailable => "corpus keychain unavailable",
        CorpusError::MigrationFailed => "corpus schema migration failed",
        CorpusError::EncryptionFailed => "corpus encryption failed",
        CorpusError::DecryptionFailed => "corpus decryption failed",
        CorpusError::QueryFailed => "corpus persistence query failed",
        CorpusError::PathTraversal => "corpus path rejected",
        CorpusError::SizeCapExceeded => "corpus file too large",
        CorpusError::SchemaVersionMismatch => "corpus schema version mismatch",
        CorpusError::Io { .. } => "corpus io error",
    };
    BufferError::Drain {
        reason: reason.to_string(),
    }
}

// Unit tests live at `pulse-app/tests/unit_drain_persistence.rs`
// (integration test crate) per session-learnings 2026-05-13 — Cargo.toml
// `[lib] test = false` disables the lib's auto-generated test binary on
// Windows due to a WebView2 DLL load failure, so source-level
// `#[cfg(test)] mod tests` would compile but never run.
