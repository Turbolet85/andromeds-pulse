//! Corpus-backed RetryStormState persistence adapter — chunk #71 + chunk #72.
//!
//! Wires `triage::contract::StormPersistence` to
//! `corpus::contract::CorpusWriter` at the pulse-app binary boundary,
//! preserving the arch §Module dependency direction DAG (triage stays
//! corpus-free; corpus stays triage-free; pulse-app owns the wire-up).
//! Mirrors chunk #70 `CorpusBaselinePersistence` precedent.
//!
//! Persistence cell shape per chunk #71 schema decision (Option A —
//! mirrors chunk #70 baseline + chunk #69 drain): re-uses the existing
//! `pipeline_metrics` SQLite table (chunk #68 schema, no version bump)
//! with `metric_name = "storm_state"`, `layer = "l2"`,
//! `payload = bincode-serialized StormStateSnapshot`. Cell-level
//! AES-256-GCM encryption is applied by the corpus crate transparently
//! to this adapter; serialization → encryption → INSERT is the save
//! path; SELECT → decryption → deserialization is the load path.
//!
//! Boot non-fatal: if corpus is unavailable at boot, the storm detector
//! runs in-memory-only for the session. Storm fingerprint bytes (16-byte
//! hashes of exception.type + normalized stack) are non-PII by
//! construction; FingerprintState.service field IS PII-bearing and is
//! passed through `security::scrubber::mask_secret_spans` via
//! `StormStateSnapshot::scrubbed_clone` at save time (chunk #72) — the
//! producer-side defense-in-depth alongside corpus-wide AES-256-GCM
//! at-rest encryption. Aggregation-collapse note: PII-shaped service
//! names collapse to the same scrubbed bucket per chunk #72 plan
//! §Implementation Note 3.

use std::sync::Arc;

use corpus::contract::{CorpusWriter, Error as CorpusError};
use security::scrubber::mask_secret_spans;
use triage::contract::{SCHEMA_VERSION, StormError, StormPersistence, StormStateSnapshot};

/// Stable `metric_name` value in `pipeline_metrics` for StormState.
/// Changing this string breaks in-place persistence reads against
/// existing on-disk corpora — schema-equivalent renaming would require
/// a migration path.
pub const STORM_STATE_METRIC_NAME: &str = "storm_state";

/// Stable `layer` tag in `pipeline_metrics`. Per pulse v0.2.0
/// distillation architecture, `l2` is the storm-detection layer.
pub const STORM_PERSISTENCE_LAYER: &str = "l2";

/// Adapter implementing `triage::StormPersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusStormPersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusStormPersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl StormPersistence for CorpusStormPersistence {
    fn load(&self) -> Result<Option<StormStateSnapshot>, StormError> {
        let bytes_opt = self
            .writer
            .load_pipeline_metric(STORM_STATE_METRIC_NAME, STORM_PERSISTENCE_LAYER)
            .map_err(corpus_error_to_storm_error)?;
        match bytes_opt {
            Some(bytes) => {
                let snapshot: StormStateSnapshot = crate::bincode_bounded::deserialize(&bytes)
                    .map_err(|_| StormError::Deserialize)?;
                Ok(Some(snapshot))
            }
            None => Ok(None),
        }
    }

    fn save(&self, snapshot: &StormStateSnapshot) -> Result<(), StormError> {
        let scrubbed = snapshot.scrubbed_clone(scrub_fingerprint_service);
        let bytes = bincode::serialize(&scrubbed).map_err(|_| StormError::Serialize)?;
        self.writer
            .save_pipeline_metric(STORM_STATE_METRIC_NAME, STORM_PERSISTENCE_LAYER, &bytes)
            .map_err(corpus_error_to_storm_error)
    }
}

/// Producer-side PII scrub for `FingerprintState.service` before bincode
/// (chunk #72). Masks each secret as the `[REDACTED:{category}]` marker per
/// the chunk #68/#69 convention — the same masking as the
/// `extract_service_name` choke point; dep-injected into
/// `StormStateSnapshot::scrubbed_clone` to keep the triage crate
/// security-crate-free.
#[doc(hidden)]
pub fn scrub_fingerprint_service(service: &str) -> String {
    mask_secret_spans(service, |category| format!("[REDACTED:{category}]")).text
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the StormError surfaced upward. Mirrors
/// chunk #70 `corpus_error_to_baseline_error` precedent.
///
/// Free function (not `From` impl) — orphan rule forbids
/// `impl From<corpus::Error> for triage::StormError` here (both types
/// foreign to pulse-app). Per session-learnings 2026-05-18.
#[doc(hidden)]
pub fn corpus_error_to_storm_error(err: CorpusError) -> StormError {
    match err {
        CorpusError::KeyringUnavailable => StormError::Io {
            kind: std::io::ErrorKind::PermissionDenied,
        },
        CorpusError::MigrationFailed => StormError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::EncryptionFailed => StormError::Serialize,
        CorpusError::DecryptionFailed => StormError::Deserialize,
        CorpusError::QueryFailed => StormError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::PathTraversal => StormError::PathTraversal,
        CorpusError::SizeCapExceeded => StormError::SizeCapExceeded { actual: 0, max: 0 },
        CorpusError::SchemaVersionMismatch => StormError::SchemaVersionMismatch {
            expected: SCHEMA_VERSION,
            got: 0,
        },
        CorpusError::Io { kind } => StormError::Io { kind },
    }
}

// Unit tests live at `pulse-app/tests/unit_storm_persistence.rs`
// (integration test crate) per session-learnings 2026-05-13 — Cargo.toml
// `[lib] test = false` disables the lib auto-generated test binary on
// Windows due to a WebView2 DLL load failure, so source-level
// `#[cfg(test)] mod tests` would compile but never run.
