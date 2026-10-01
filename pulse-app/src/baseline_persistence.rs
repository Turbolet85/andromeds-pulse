//! Corpus-backed BaselineState persistence adapter — chunk #70 + chunk #72.
//!
//! Wires `triage::contract::BaselinePersistence` to
//! `corpus::contract::CorpusWriter` at the pulse-app binary boundary,
//! preserving the arch §Module dependency direction DAG (triage stays
//! corpus-free; corpus stays triage-free; pulse-app owns the wire-up).
//! Mirrors chunk #69's `CorpusDrainPersistence` precedent + chunk #59
//! `HeartbeatBindStatus` trait-in-lower-crate + adapter-in-pulse-app
//! pattern (per session-learnings 2026-05-16 + 2026-05-19).
//!
//! Persistence cell shape per chunk #70 schema decision Option A
//! (mirrors chunk #69): re-uses the existing `pipeline_metrics` SQLite
//! table (chunk #68 schema, no version bump) with
//! `metric_name = "baseline_state"`, `layer = "l1b"`, `payload =
//! bincode-serialized BaselineState`. Cell-level AES-256-GCM encryption
//! is applied by the corpus crate transparently to this adapter;
//! serialization → encryption → INSERT is the save path; SELECT →
//! decryption → deserialization is the load path.
//!
//! PII discipline (chunk #72): `BaselineState` per-service `DashMap` keys
//! and per-operation key prefixes carry raw `service.name` strings
//! (user-content classification). `save` calls
//! `BaselineState::scrubbed_clone` with `security::scrubber::mask_secret_spans`
//! to pre-scrub the keys before bincode, rendering `[REDACTED:{category}]`
//! markers for matched categories. Aggregation-collapse note: PII-shaped
//! service names collapse to the same scrubbed bucket per chunk #72 plan
//! §Implementation Note 3 (intentional security > attribution trade-off).
//!
//! Legacy bincode migration: at boot, `migrate_legacy_baseline_if_present`
//! checks for `<data_dir>/triage/baseline-corpus.bin` (chunk #61
//! substrate). If present + valid, the contents are read once,
//! re-persisted through the trait, and the legacy file is deleted.
//! Failures preserve the legacy file for retry on next boot.

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use corpus::contract::{CorpusWriter, Error as CorpusError};
use security::scrubber::mask_secret_spans;
use triage::contract::{BaselineError, BaselinePersistence, BaselineState, DEFAULT_MAX_SIZE_BYTES};

/// Stable `metric_name` value in `pipeline_metrics` for BaselineState.
/// Changing this string breaks in-place persistence reads against
/// existing on-disk corpora — schema-equivalent renaming would require
/// a migration path.
pub const BASELINE_STATE_METRIC_NAME: &str = "baseline_state";

/// Stable `layer` tag in `pipeline_metrics`. Per pulse v0.2.0
/// distillation architecture, `l1b` is the statistical-baseline layer.
pub const BASELINE_PERSISTENCE_LAYER: &str = "l1b";

/// Legacy bincode subpath under the resolved data dir (chunk #61
/// substrate; eliminated by chunk #70). Migration helper looks for this
/// file at boot.
#[doc(hidden)]
pub const LEGACY_BASELINE_BASENAME: &str = "baseline-corpus.bin";
#[doc(hidden)]
pub const LEGACY_BASELINE_SUBDIR: &str = "triage";

/// Adapter implementing `triage::BaselinePersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusBaselinePersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusBaselinePersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl BaselinePersistence for CorpusBaselinePersistence {
    fn load(&self) -> Result<Option<BaselineState>, BaselineError> {
        let bytes_opt = self
            .writer
            .load_pipeline_metric(BASELINE_STATE_METRIC_NAME, BASELINE_PERSISTENCE_LAYER)
            .map_err(corpus_error_to_baseline_error)?;
        match bytes_opt {
            Some(bytes) => {
                let state: BaselineState = crate::bincode_bounded::deserialize(&bytes)
                    .map_err(|_| BaselineError::Deserialize)?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    fn save(&self, state: &BaselineState) -> Result<(), BaselineError> {
        let scrubbed = state.scrubbed_clone(scrub_service_key);
        let bytes = bincode::serialize(&scrubbed).map_err(|_| BaselineError::Serialize)?;
        self.writer
            .save_pipeline_metric(
                BASELINE_STATE_METRIC_NAME,
                BASELINE_PERSISTENCE_LAYER,
                &bytes,
            )
            .map_err(corpus_error_to_baseline_error)
    }
}

/// Producer-side PII scrub for `BaselineState` per-service map keys + the
/// `service` prefix of per-operation keys before bincode (chunk #72).
/// Masks each secret as the `[REDACTED:{category}]` marker per the chunk
/// #68/#69 convention — the same masking as the `extract_service_name` choke
/// point, so a key that already passed it comes back unchanged;
/// dep-injected into `BaselineState::scrubbed_clone` to keep the triage
/// crate security-crate-free.
#[doc(hidden)]
pub fn scrub_service_key(service: &str) -> String {
    mask_secret_spans(service, |category| format!("[REDACTED:{category}]")).text
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the BaselineError surfaced upward. Mirrors
/// chunk #69 `corpus_error_to_buffer_error` precedent.
///
/// Free function (not `From` impl) — orphan rule forbids
/// `impl From<corpus::Error> for triage::BaselineError` here (both types
/// foreign to pulse-app). Per session-learnings 2026-05-18.
#[doc(hidden)]
pub fn corpus_error_to_baseline_error(err: CorpusError) -> BaselineError {
    use triage::contract::SCHEMA_VERSION;
    match err {
        CorpusError::KeyringUnavailable => BaselineError::Io {
            kind: std::io::ErrorKind::PermissionDenied,
        },
        CorpusError::MigrationFailed => BaselineError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::EncryptionFailed => BaselineError::Serialize,
        CorpusError::DecryptionFailed => BaselineError::Deserialize,
        CorpusError::QueryFailed => BaselineError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::PathTraversal => BaselineError::PathTraversal,
        CorpusError::SizeCapExceeded => BaselineError::SizeCapExceeded { actual: 0, max: 0 },
        CorpusError::SchemaVersionMismatch => BaselineError::SchemaVersionMismatch {
            expected: SCHEMA_VERSION,
            got: 0,
        },
        CorpusError::Io { kind } => BaselineError::Io { kind },
    }
}

/// Outcome of a legacy-baseline migration attempt. Returned by
/// `migrate_legacy_baseline_if_present`; tracing events for each outcome
/// fire inside that function before return.
#[derive(Debug, PartialEq, Eq)]
pub enum MigrationOutcome {
    /// Legacy file not present — no-op (cold-start path).
    Noop,
    /// Legacy file read + re-persisted through trait + cleanup attempted.
    Completed {
        legacy_file_deleted: bool,
        bytes: u64,
        service_count: u64,
    },
    /// Migration failed at the indicated stage. Legacy file is preserved
    /// for retry on next boot (no auto-delete on partial migration).
    Failed { error_category: &'static str },
}

/// Attempt one-shot migration of the legacy `<data_dir>/triage/
/// baseline-corpus.bin` flat-file (chunk #61 substrate) into the corpus
/// SQLite-backed `BaselinePersistence`. Idempotent: once the legacy file
/// is deleted (post-completion), subsequent boots return `Noop`.
///
/// PII discipline: emitted tracing events carry aggregate-only fields per
/// `.claude/rules/observability.md` Session Addition 2026-05-17 session
/// 84 — `legacy_state_size_bytes` / `migrated_service_count` /
/// `duration_ms` / `migration_outcome` enum. No raw path, no service
/// names, no error chain.
pub fn migrate_legacy_baseline_if_present(
    data_dir: &Path,
    persistence: &dyn BaselinePersistence,
) -> MigrationOutcome {
    let triage_dir = data_dir.join(LEGACY_BASELINE_SUBDIR);
    let legacy_path = triage_dir.join(LEGACY_BASELINE_BASENAME);

    if !legacy_path.exists() {
        return MigrationOutcome::Noop;
    }

    let start = Instant::now();
    let outcome = migrate_legacy_inner(&legacy_path, persistence);
    let duration_ms = start.elapsed().as_millis() as u64;

    match &outcome {
        MigrationOutcome::Noop => {}
        MigrationOutcome::Completed {
            legacy_file_deleted,
            bytes,
            service_count,
        } => {
            let outcome_tag = if *legacy_file_deleted {
                "completed"
            } else {
                "completed_legacy_kept"
            };
            tracing::info!(
                target: triage::contract::TARGET_BASELINE_MIGRATE,
                legacy_state_size_bytes = *bytes,
                migrated_service_count = *service_count,
                duration_ms = duration_ms,
                migration_outcome = outcome_tag,
                "migrated legacy baseline-corpus.bin to corpus",
            );
        }
        MigrationOutcome::Failed { error_category } => {
            tracing::warn!(
                target: triage::contract::TARGET_BASELINE_MIGRATE_FAILED,
                error_category = *error_category,
                duration_ms = duration_ms,
                "legacy baseline-corpus.bin migration failed; preserving legacy file for retry",
            );
        }
    }

    outcome
}

fn migrate_legacy_inner(
    legacy_path: &Path,
    persistence: &dyn BaselinePersistence,
) -> MigrationOutcome {
    // Canonicalize to defeat symlink-based path-traversal attempts. We
    // do NOT assert resolved-path-under-data-dir here because the data
    // dir itself was already canonicalized at the boot site; a symlink
    // inside the triage subdir pointing outside the data root would
    // surface as a canonicalize error against the symlink target.
    let canonical = match std::fs::canonicalize(legacy_path) {
        Ok(p) => p,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "io",
            };
        }
    };

    let metadata = match std::fs::metadata(&canonical) {
        Ok(m) => m,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "io",
            };
        }
    };

    if metadata.len() > DEFAULT_MAX_SIZE_BYTES {
        return MigrationOutcome::Failed {
            error_category: "size_exceeded",
        };
    }

    let bytes = match std::fs::read(&canonical) {
        Ok(b) => b,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "io",
            };
        }
    };

    // Pre-flight prefix sanity check on untrusted-input boundary (chunk #72
    // follow-up). bincode 1.3.3 pre-allocates Vec / HashMap capacity from
    // the u64 length prefix BEFORE attempting to read entries; a
    // valid-looking crafted prefix (e.g., `b"\x00\x01\x02 garbage"` decodes
    // its first 8 bytes as ~7e18) would trigger an immediate OOM process
    // abort even though `Options::with_limit` is configured. This validator
    // checks the `services` map length prefix at the known struct offset
    // (4-byte schema_version + 8-byte u64 map len) and rejects implausible
    // counts before bincode allocates anything. Layout MUST stay in sync with
    // `crates/triage/src/baseline/mod.rs::BaselineState` field declaration
    // order (schema_version → services → operations → persisted_at).
    if !baseline_bytes_prefix_plausible(&bytes) {
        return MigrationOutcome::Failed {
            error_category: "deserialize",
        };
    }

    let state: BaselineState = match crate::bincode_bounded::deserialize(&bytes) {
        Ok(s) => s,
        Err(_) => {
            return MigrationOutcome::Failed {
                error_category: "deserialize",
            };
        }
    };

    let service_count = state.service_count() as u64;
    let bytes_len = bytes.len() as u64;

    if let Err(e) = persistence.save(&state) {
        return MigrationOutcome::Failed {
            error_category: e.error_category(),
        };
    }

    let legacy_file_deleted = std::fs::remove_file(&canonical).is_ok();

    MigrationOutcome::Completed {
        legacy_file_deleted,
        bytes: bytes_len,
        service_count,
    }
}

/// Plausibility ceiling on the `services` map size — anything beyond this
/// is almost certainly a crafted length prefix rather than legitimate
/// telemetry state. Conservatively above the `ACTIVITY_FLOOR_SERVICE_CAP`
/// in-process bound (currently 100k) with headroom for future raises; well
/// below any value that would trigger OOM allocation on 64-bit hosts.
const BASELINE_PREFIX_SVC_LEN_PLAUSIBILITY_CEILING: u64 = 10_000_000;

/// Sanity-check the bincode bytes' `services` length-prefix slot before
/// passing to bincode. Prevents the OOM-via-crafted-prefix vulnerability
/// surfaced at chunk #72 follow-up (untrusted legacy file content). True
/// = pass (continue to bincode); false = reject (treat as `deserialize`
/// failure). Below 12 bytes, bincode will EOF safely on its own — let
/// it through.
fn baseline_bytes_prefix_plausible(bytes: &[u8]) -> bool {
    if bytes.len() < 12 {
        return true;
    }
    let svc_len_bytes: [u8; 8] = match bytes[4..12].try_into() {
        Ok(b) => b,
        Err(_) => return true,
    };
    let svc_len = u64::from_le_bytes(svc_len_bytes);
    svc_len <= BASELINE_PREFIX_SVC_LEN_PLAUSIBILITY_CEILING
}

// Unit tests live at `pulse-app/tests/unit_baseline_persistence.rs`
// (integration test crate) per session-learnings 2026-05-13 — Cargo.toml
// `[lib] test = false` disables the lib auto-generated test binary on
// Windows due to a WebView2 DLL load failure, so source-level
// `#[cfg(test)] mod tests` would compile but never run.
