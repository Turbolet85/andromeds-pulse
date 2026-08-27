//! Community-training export record assembly — chunk #95.
//!
//! Maps corpus `incidents` rows to anonymized JSONL export records at the
//! pulse-app binary boundary (the only place that depends on `corpus`,
//! `triage`, and `security` together). Each incident becomes one record
//! carrying its trigger context (`kind`, `scope`, `scope_id`, `title`,
//! `detail`), model interpretation (`resolution_summary_text` — the latest
//! cleanly-parsed interpretation, attached from creation onward since chunk
//! 2026-08-26; the resolution-summary generation is the final write), user
//! feedback (`acknowledged`, `read`), and resolution outcome (`status`,
//! `resolved_at_unix_nano`).
//!
//! Every user-facing text field is routed through
//! `security::scrubber::scrub_attribute` at this egress boundary — defense
//! in depth on top of the chunk #72 producer-side scrub, mirroring the
//! chunk #88 `incidents.get_report` resolver-boundary discipline. The
//! export writes a local file only and NEVER transmits to any endpoint
//! (capability P-046 opt-in posture; the user manually shares the file).
//!
//! The `~/Downloads` default target is a deliberate out-of-data-dir export
//! sink; the data-dir-confinement rule (arch §Critical Warnings) does not
//! apply to an explicit user-chosen egress path, but the target is still
//! validated (no `..` traversal; parent directory must exist) before write.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use corpus::contract::IncidentRowRaw;
use security::scrubber::{ScrubbedValue, scrub_attribute};
use serde::{Deserialize, Serialize};
use triage::contract::{CueKind, CueScope, Incident, IncidentStatus, PriorityTier, Severity};
use ui_bridge::contract::AppError;

/// One anonymized export record per incident. Bounded enums serialize as
/// snake_case per their `#[serde(rename_all)]` derives; text fields are
/// pre-scrubbed via [`scrub_string`]. `interpretation` carries the scrubbed
/// `resolution_summary_text` (the latest attached interpretation — present
/// for live incidents too once their first generation parses; null only
/// before that).
#[derive(Debug, Clone, Serialize)]
pub struct ExportRecord {
    pub id: i64,
    pub workspace: String,
    pub status: IncidentStatus,
    pub kind: CueKind,
    pub scope: CueScope,
    pub scope_id: Option<String>,
    pub severity: Severity,
    pub priority_tier: PriorityTier,
    pub title: String,
    pub detail: String,
    pub opened_at_unix_nano: i64,
    pub resolved_at_unix_nano: Option<i64>,
    pub acknowledged: bool,
    pub read: bool,
    pub interpretation: Option<String>,
    pub fingerprint_hashes: Vec<String>,
}

/// Pre-write preview summary surfaced across the TauRPC bridge. `written`
/// is false on the preview-only path (no file written); true after the
/// confirmed write. `categories` aggregates counts by `status` + `severity`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ExportPreviewPayload {
    pub categories: Vec<ExportCategoryCount>,
    pub total_records: u64,
    pub date_range_start_unix_nano: Option<i64>,
    pub date_range_end_unix_nano: Option<i64>,
    pub anonymization_confirmed: bool,
    pub written: bool,
    pub written_path_basename: Option<String>,
}

/// One `(dimension, label) -> count` entry in the preview summary. `Vec` of
/// these (rather than a map) keeps the TS binding shape stable across specta
/// releases. `dimension` is `"status"` or `"severity"`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ExportCategoryCount {
    pub dimension: String,
    pub label: String,
    pub count: u64,
}

const REDACTED_PREFIX: &str = "[redacted:";

/// Scrub a single text field at the egress boundary. Redactions render as
/// `[redacted: {category}]` (only the bounded category label, never the
/// matched value) per the chunk #88 resolver-scrub precedent.
fn scrub_string(value: &str) -> String {
    match scrub_attribute(value) {
        ScrubbedValue::Allowed(s) => s,
        ScrubbedValue::Redacted { category } => format!("{REDACTED_PREFIX} {category}]"),
    }
}

/// snake_case label for a bounded enum via its serde representation. All the
/// incident enums are unit-variant `#[serde(rename_all = "snake_case")]`, so
/// `to_value` yields a `Value::String`.
fn enum_snake_label<T: Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(serde_json::Value::String(s)) => s,
        _ => "unknown".to_string(),
    }
}

/// Decode one corpus `incidents` row to a scrubbed [`ExportRecord`]. Mirrors
/// the `incident_persistence.rs` decode pattern: bincode-deserialize the
/// `Incident` BLOB (plain config — cross-process parity with the producer),
/// then re-stamp authoritative SQL-column metadata before scrubbing.
pub fn incident_row_to_export_record(row: &IncidentRowRaw) -> Result<ExportRecord, AppError> {
    let mut incident: Incident =
        bincode::deserialize::<Incident>(&row.payload).map_err(|_| AppError::Internal {
            message: "export record decode failed".to_string(),
        })?;
    incident.id = row.id;
    incident.workspace = row.workspace.clone();
    incident.opened_at_unix_nano = row.created_unix_nano;
    incident.updated_at_unix_nano = row.updated_unix_nano;
    incident.resolved_at_unix_nano = row.resolved_unix_nano;
    incident.read_at_unix_nano = row.read_unix_nano;

    Ok(ExportRecord {
        id: incident.id,
        workspace: scrub_string(&incident.workspace),
        status: incident.status,
        kind: incident.kind,
        scope: incident.scope,
        scope_id: incident.scope_id.as_deref().map(scrub_string),
        severity: incident.severity,
        priority_tier: incident.priority_tier,
        title: scrub_string(&incident.title),
        detail: scrub_string(&incident.detail),
        opened_at_unix_nano: incident.opened_at_unix_nano,
        resolved_at_unix_nano: incident.resolved_at_unix_nano,
        acknowledged: incident.acknowledged_at_unix_nano.is_some(),
        read: incident.read_at_unix_nano.is_some(),
        interpretation: incident
            .resolution_summary_text
            .as_deref()
            .map(scrub_string),
        fingerprint_hashes: incident
            .evidence_refs
            .fingerprint_hashes
            .iter()
            .map(|h| scrub_string(h))
            .collect(),
    })
}

/// Decode + scrub every row. Propagates the first decode failure.
pub fn assemble_records(rows: &[IncidentRowRaw]) -> Result<Vec<ExportRecord>, AppError> {
    rows.iter().map(incident_row_to_export_record).collect()
}

/// Serialize records to JSONL (one `serde_json` object per line, `\n`-joined,
/// trailing newline).
pub fn serialize_jsonl(records: &[ExportRecord]) -> Result<String, AppError> {
    let mut out = String::new();
    for record in records {
        let line = serde_json::to_string(record).map_err(|_| AppError::Internal {
            message: "export serialize failed".to_string(),
        })?;
        out.push_str(&line);
        out.push('\n');
    }
    Ok(out)
}

/// Count scrubber redactions across all records (aggregate-only field for
/// the `storage.export_for_training.request` tracing event — never the
/// redacted values themselves).
pub fn count_redactions(records: &[ExportRecord]) -> u64 {
    let is_redacted = |s: &str| s.starts_with(REDACTED_PREFIX);
    let mut n: u64 = 0;
    for record in records {
        if is_redacted(&record.workspace) {
            n += 1;
        }
        if record.scope_id.as_deref().is_some_and(is_redacted) {
            n += 1;
        }
        if is_redacted(&record.title) {
            n += 1;
        }
        if is_redacted(&record.detail) {
            n += 1;
        }
        if record.interpretation.as_deref().is_some_and(is_redacted) {
            n += 1;
        }
        n += record
            .fingerprint_hashes
            .iter()
            .filter(|h| is_redacted(h))
            .count() as u64;
    }
    n
}

/// Build the pre-write preview summary: count-by-status + count-by-severity,
/// `opened_at` date range, and the write outcome.
pub fn build_preview_summary(
    records: &[ExportRecord],
    written: bool,
    written_path_basename: Option<String>,
) -> ExportPreviewPayload {
    let mut status_counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut severity_counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut date_min: Option<i64> = None;
    let mut date_max: Option<i64> = None;
    for record in records {
        *status_counts
            .entry(enum_snake_label(&record.status))
            .or_insert(0) += 1;
        *severity_counts
            .entry(enum_snake_label(&record.severity))
            .or_insert(0) += 1;
        date_min = Some(date_min.map_or(record.opened_at_unix_nano, |m| {
            m.min(record.opened_at_unix_nano)
        }));
        date_max = Some(date_max.map_or(record.opened_at_unix_nano, |m| {
            m.max(record.opened_at_unix_nano)
        }));
    }
    let mut categories = Vec::with_capacity(status_counts.len() + severity_counts.len());
    for (label, count) in status_counts {
        categories.push(ExportCategoryCount {
            dimension: "status".to_string(),
            label,
            count,
        });
    }
    for (label, count) in severity_counts {
        categories.push(ExportCategoryCount {
            dimension: "severity".to_string(),
            label,
            count,
        });
    }
    ExportPreviewPayload {
        categories,
        total_records: records.len() as u64,
        date_range_start_unix_nano: date_min,
        date_range_end_unix_nano: date_max,
        anonymization_confirmed: true,
        written,
        written_path_basename,
    }
}

/// Default export target `<home>/Downloads/pulse-corpus-export-{ts}.jsonl`.
/// `now_unix_nano` is injected so the filename is deterministic in tests
/// (NEVER `SystemTime::now()` in the testable path). Home resolves via
/// `USERPROFILE` (Windows) / `HOME` (unix) — no new workspace dependency.
pub fn resolve_default_target(now_unix_nano: i64) -> Result<PathBuf, AppError> {
    let home = home_dir().ok_or_else(|| AppError::Internal {
        message: "home directory unavailable".to_string(),
    })?;
    let filename = format!("pulse-corpus-export-{now_unix_nano}.jsonl");
    Ok(home.join("Downloads").join(filename))
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
}

/// Validate a user-supplied export target before write. Rejects `..`
/// traversal components (CWE-22 defense for a user-controlled egress path)
/// and requires the parent directory to exist. The default `~/Downloads`
/// target is an explicit out-of-data-dir sink (chunk #95 arch exception);
/// the data-dir-confinement rule does not apply, but traversal + parent
/// existence are still enforced.
pub fn validate_export_target(target: &Path) -> Result<PathBuf, AppError> {
    if target
        .components()
        .any(|c| matches!(c, Component::ParentDir))
    {
        return Err(AppError::Validation {
            field: "target_path".to_string(),
            reason: "path must not contain parent-directory (..) components".to_string(),
        });
    }
    let parent = target.parent().ok_or_else(|| AppError::Validation {
        field: "target_path".to_string(),
        reason: "path has no parent directory".to_string(),
    })?;
    if !parent.is_dir() {
        return Err(AppError::Validation {
            field: "target_path".to_string(),
            reason: "parent directory does not exist".to_string(),
        });
    }
    Ok(target.to_path_buf())
}

// Unit tests live at `pulse-app/tests/unit_training_export.rs` (integration
// test crate) per session-learnings 2026-05-13 — Cargo.toml `[lib] test =
// false` disables the lib auto-generated test binary on Windows (WebView2
// DLL load), so source-level `#[cfg(test)] mod tests` would compile but
// never run.
