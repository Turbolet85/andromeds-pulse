//! Corpus-backed lifecycle persistence adapter — chunk #71 + chunk #72.
//!
//! Wires `triage::contract::LifecyclePersistence` to
//! `corpus::contract::CorpusWriter` at the pulse-app binary boundary,
//! preserving the arch §Module dependency direction DAG (triage stays
//! corpus-free; corpus stays triage-free; pulse-app owns the wire-up).
//! Mirrors chunk #70 `CorpusBaselinePersistence` + chunk #69
//! `CorpusDrainPersistence` precedents.
//!
//! Schema choice: per-row INSERT/UPDATE on the pre-allocated
//! `service_registry` SQLite table (chunk #68 schema, no version bump).
//! Each `ServiceRegistryEntry` maps directly onto the table's columns
//! (state TEXT, three timestamp INTEGERs, manual_override TEXT NULL).
//! `service_name` is the UNIQUE key; the new
//! `CorpusWriter::save_service_registry_row` method UPSERTs.
//!
//! PII discipline (chunk #72): `service_name` is user-content classification.
//! `save_all` runs each service through `security::scrubber::mask_secret_spans`
//! BEFORE the `save_service_registry_row` call (`[REDACTED:{category}]`
//! marker convention per chunk #68/#69/drain.rs:600 precedent); AES
//! corpus-wide encryption stays as defense-in-depth for the at-rest threat
//! model. Aggregation-collapse note: PII-shaped service names collapse into
//! the same scrubbed bucket on the unique `service_name` column (e.g., two
//! distinct misconfigured services that both look like emails merge into
//! one `[REDACTED:email]` row); intentional security > attribution trade-off
//! for misconfigured services per chunk #72 plan §Implementation Note 3.

use std::sync::Arc;

use corpus::contract::{CorpusWriter, Error as CorpusError};
use security::scrubber::mask_secret_spans;
use triage::contract::{
    LifecycleError, LifecyclePersistence, SCHEMA_VERSION, ServiceLifecycleState,
    ServiceRegistryEntry, state_label,
};

/// Adapter implementing `triage::LifecyclePersistence` over a
/// `corpus::contract::CorpusWriter`. Cheap to clone (single Arc inside).
#[derive(Clone)]
pub struct CorpusLifecyclePersistence {
    writer: Arc<dyn CorpusWriter>,
}

impl CorpusLifecyclePersistence {
    pub fn new(writer: Arc<dyn CorpusWriter>) -> Self {
        Self { writer }
    }
}

impl LifecyclePersistence for CorpusLifecyclePersistence {
    fn load_all(&self) -> Result<Option<Vec<(String, ServiceRegistryEntry)>>, LifecycleError> {
        let rows = self
            .writer
            .load_all_service_registry_rows()
            .map_err(corpus_error_to_lifecycle_error)?;
        if rows.is_empty() {
            return Ok(None);
        }
        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            let state = parse_state(&row.state).ok_or(LifecycleError::Deserialize)?;
            let manual_override = match row.manual_override.as_deref() {
                Some(s) => Some(parse_state(s).ok_or(LifecycleError::Deserialize)?),
                None => None,
            };
            entries.push((
                row.service_name,
                ServiceRegistryEntry {
                    state,
                    first_seen_unix_nano: row.first_seen_unix_nano,
                    last_seen_unix_nano: row.last_seen_unix_nano,
                    last_transition_unix_nano: row.last_transition_unix_nano,
                    manual_override,
                },
            ));
        }
        Ok(Some(entries))
    }

    fn save_all(&self, entries: &[(String, ServiceRegistryEntry)]) -> Result<(), LifecycleError> {
        for (service, entry) in entries {
            let manual_override_label = entry.manual_override.map(state_label);
            let scrubbed_service = scrub_service_name(service);
            self.writer
                .save_service_registry_row(
                    &scrubbed_service,
                    state_label(entry.state),
                    entry.first_seen_unix_nano,
                    entry.last_seen_unix_nano,
                    entry.last_transition_unix_nano,
                    manual_override_label,
                )
                .map_err(corpus_error_to_lifecycle_error)?;
        }
        Ok(())
    }
}

/// Producer-side PII scrub for `service_name` before
/// `CorpusWriter::save_service_registry_row` (chunk #72). Masks each secret
/// as the `[REDACTED:{category}]` marker per the chunk #68/#69 convention —
/// the same masking as the `extract_service_name` choke point.
#[doc(hidden)]
pub fn scrub_service_name(service: &str) -> String {
    mask_secret_spans(service, |category| format!("[REDACTED:{category}]")).text
}

/// Sanitized cross-crate error mapping. Per arch §Established Decisions
/// [Error Handling Pattern]: no SQLite stack traces / file paths /
/// library versions appear in the LifecycleError surfaced upward.
/// Mirrors chunk #70 `corpus_error_to_baseline_error` precedent.
///
/// Free function (not `From` impl) — orphan rule forbids
/// `impl From<corpus::Error> for triage::LifecycleError` here (both
/// types foreign to pulse-app). Per session-learnings 2026-05-18.
#[doc(hidden)]
pub fn corpus_error_to_lifecycle_error(err: CorpusError) -> LifecycleError {
    match err {
        CorpusError::KeyringUnavailable => LifecycleError::Io {
            kind: std::io::ErrorKind::PermissionDenied,
        },
        CorpusError::MigrationFailed => LifecycleError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::EncryptionFailed => LifecycleError::Serialize,
        CorpusError::DecryptionFailed => LifecycleError::Deserialize,
        CorpusError::QueryFailed => LifecycleError::Io {
            kind: std::io::ErrorKind::Other,
        },
        CorpusError::PathTraversal => LifecycleError::PathTraversal,
        CorpusError::SizeCapExceeded => LifecycleError::SizeCapExceeded { actual: 0, max: 0 },
        CorpusError::SchemaVersionMismatch => LifecycleError::SchemaVersionMismatch {
            expected: SCHEMA_VERSION,
            got: 0,
        },
        CorpusError::Io { kind } => LifecycleError::Io { kind },
    }
}

/// Parse the corpus `state` TEXT column into a `ServiceLifecycleState`
/// enum variant. Mirrors the snake_case serialization defined by the
/// `#[serde(rename_all = "snake_case")]` derive on
/// `ServiceLifecycleState`. Returns `None` on unknown variant — caller
/// surfaces as `LifecycleError::Deserialize`.
#[doc(hidden)]
pub fn parse_state(s: &str) -> Option<ServiceLifecycleState> {
    match s {
        "unknown" => Some(ServiceLifecycleState::Unknown),
        "bootstrapping" => Some(ServiceLifecycleState::Bootstrapping),
        "active" => Some(ServiceLifecycleState::Active),
        "quiet" => Some(ServiceLifecycleState::Quiet),
        "silent" => Some(ServiceLifecycleState::Silent),
        "dormant" => Some(ServiceLifecycleState::Dormant),
        "archived" => Some(ServiceLifecycleState::Archived),
        _ => None,
    }
}

// Unit tests live at `pulse-app/tests/unit_lifecycle_persistence.rs`
// (integration test crate) per session-learnings 2026-05-13 — Cargo.toml
// `[lib] test = false` disables the lib auto-generated test binary on
// Windows due to a WebView2 DLL load failure, so source-level
// `#[cfg(test)] mod tests` would compile but never run.
