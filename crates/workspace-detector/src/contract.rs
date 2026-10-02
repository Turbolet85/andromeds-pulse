use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// VCS detected for the workspace candidate. Closed enum: only Git is
/// supported in chunk #43; richer VCS support deferred per Open Q5
/// resolution (filesystem-only `.git/HEAD` parse covers the P7 critical
/// path).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VcsType {
    Git,
}

/// VCS metadata for a detected workspace. `head_commit_basename` carries
/// only the short SHA prefix (basename slice); never the full SHA + never
/// the commit message body — per security plan §Logging hygiene.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VcsMetadata {
    pub vcs_type: VcsType,
    pub vcs_root: PathBuf,
    pub head_commit_basename: Option<String>,
}

/// Detected workspace context. `root` is the canonicalized candidate path;
/// `project_name` is best-effort from the root basename. `has_andromeda_marker`
/// is true if a `.andromeda/` directory exists at the root (content NEVER
/// read).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContext {
    pub root: PathBuf,
    pub project_name: Option<String>,
    pub vcs: Option<VcsMetadata>,
    pub has_andromeda_marker: bool,
}

/// Errors emitted by `workspace_detector::detect`. PathTraversalRejected /
/// CanonicalizationFailed map to ui-bridge `AppError::Validation`; IoFailure
/// maps to `AppError::Storage`. The From-impl in `ui-bridge::contract`
/// sanitizes messages (no full paths, no struct names, no library
/// versions).
#[derive(Debug, Error)]
pub enum Error {
    #[error("path traversal rejected: {reason}")]
    PathTraversalRejected { reason: String },

    #[error("canonicalization failed: {reason}")]
    CanonicalizationFailed { reason: String },

    #[error("io failure")]
    IoFailure(#[from] std::io::Error),
}

/// Basename of the file the app publishes its resolved workspace key to,
/// under `{data_dir}/run/`.
pub const WORKSPACE_KEY_BASENAME: &str = "workspace-key";

/// Upper bound on a published key, enforced on read. A workspace key is a
/// filesystem path; anything larger than this is not one.
pub const MAX_WORKSPACE_KEY_BYTES: usize = 4096;

/// Path of the published workspace-key file under `data_dir`.
pub fn workspace_key_path(data_dir: &Path) -> PathBuf {
    data_dir.join("run").join(WORKSPACE_KEY_BASENAME)
}

/// The incident workspace key: the identity incidents are STAMPED with and
/// FILTERED by. Detection succeeded ⇒ the detected root; detection failed ⇒
/// `data_dir`, so both halves still agree.
///
/// `ctx.root` is already `detect`'s single `canonicalize()` output — this
/// returns it verbatim. Re-canonicalizing here would diverge from the value
/// the producer stamped (the trap `resolver_preserves_windows_extended_length_prefix`
/// locks).
pub fn workspace_key(detected: Option<&WorkspaceContext>, data_dir: &Path) -> String {
    match detected {
        Some(ctx) => ctx.root.to_string_lossy().into_owned(),
        None => data_dir.to_string_lossy().into_owned(),
    }
}

/// Publish the resolved key so a separate process sharing this data dir (the
/// MCP stdio sidecar) filters incidents by the same identity the app stamps.
/// The data dir is the only value both processes independently agree on.
///
/// Atomic `.tmp`-then-rename; refuses to write if `run/` resolves outside
/// `data_dir` (CWE-22 class, mirroring the PID-file guard).
pub fn publish_workspace_key(data_dir: &Path, key: &str) -> Result<(), Error> {
    let run_dir = data_dir.join("run");
    fs::create_dir_all(&run_dir)?;
    let canonical_data = fs::canonicalize(data_dir)?;
    let canonical_run = fs::canonicalize(&run_dir)?;
    if !canonical_run.starts_with(&canonical_data) {
        return Err(Error::PathTraversalRejected {
            reason: "run dir escaped data dir".to_string(),
        });
    }
    let path = workspace_key_path(data_dir);
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, key.as_bytes())?;
    fs::rename(&tmp, &path)?;
    Ok(())
}

/// Read a published key. `None` whenever the file is absent, unreadable, or
/// fails validation — every such case falls back to `data_dir`, which is the
/// pre-publication behaviour.
///
/// The returned value is an OPAQUE filter string. It is never opened, joined,
/// or otherwise used as a path, so its content cannot reach the filesystem;
/// validation bounds it and rejects control characters so a corrupted file
/// cannot smuggle framing into a log line or a query.
pub fn read_published_workspace_key(data_dir: &Path) -> Option<String> {
    let bytes = fs::read(workspace_key_path(data_dir)).ok()?;
    if bytes.len() > MAX_WORKSPACE_KEY_BYTES {
        return None;
    }
    let key = String::from_utf8(bytes).ok()?;
    let key = key.trim_end_matches(['\r', '\n']);
    if key.is_empty() || key.chars().any(|c| c.is_control()) {
        return None;
    }
    Some(key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_type_serializes_snake_case() {
        let json = serde_json::to_string(&VcsType::Git).unwrap();
        assert_eq!(json, "\"git\"");
    }

    #[test]
    fn workspace_context_round_trips_through_serde() {
        let ctx = WorkspaceContext {
            root: PathBuf::from("/tmp/example"),
            project_name: Some("example".to_string()),
            vcs: Some(VcsMetadata {
                vcs_type: VcsType::Git,
                vcs_root: PathBuf::from("/tmp/example"),
                head_commit_basename: Some("a1b2c3d4".to_string()),
            }),
            has_andromeda_marker: true,
        };
        let json = serde_json::to_string(&ctx).unwrap();
        let parsed: WorkspaceContext = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, ctx);
    }

    #[test]
    fn error_path_traversal_display_includes_reason() {
        let e = Error::PathTraversalRejected {
            reason: "candidate escapes parent".to_string(),
        };
        assert!(format!("{e}").contains("candidate escapes parent"));
    }

    #[test]
    fn error_canonicalization_display_includes_reason() {
        let e = Error::CanonicalizationFailed {
            reason: "candidate not found".to_string(),
        };
        assert!(format!("{e}").contains("candidate not found"));
    }

    fn ctx_at(root: &str) -> WorkspaceContext {
        WorkspaceContext {
            root: PathBuf::from(root),
            project_name: Some("payments".to_string()),
            vcs: None,
            has_andromeda_marker: true,
        }
    }

    #[test]
    fn workspace_key_uses_detected_root_not_data_dir() {
        let ctx = ctx_at("/home/dev/payments");
        let key = workspace_key(Some(&ctx), Path::new("/var/data/andromeda-pulse"));
        assert_eq!(key, "/home/dev/payments");
    }

    #[test]
    fn workspace_key_falls_back_to_data_dir_when_detection_failed() {
        let data_dir = Path::new("/var/data/andromeda-pulse");
        assert_eq!(workspace_key(None, data_dir), data_dir.to_string_lossy());
    }

    #[test]
    fn workspace_key_returns_the_detected_root_verbatim_including_verbatim_prefix() {
        let ctx = ctx_at(r"\\?\C:\dev\payments");
        let key = workspace_key(
            Some(&ctx),
            Path::new(r"C:\Users\dev\AppData\Roaming\andromeda-pulse"),
        );
        assert_eq!(key, r"\\?\C:\dev\payments");
    }

    /// The normalization proof: a raw entry form and its `\\?\` verbatim form
    /// name one directory, and `detect`'s single `canonicalize()` maps both to
    /// one key — so the two processes cannot disagree by entry form alone.
    #[test]
    fn raw_and_verbatim_entry_forms_of_one_directory_resolve_to_one_key() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let raw = tmp.path();

        let from_raw = crate::detect::detect(raw).expect("detect raw form");
        let verbatim = PathBuf::from(format!(r"\\?\{}", raw.display()));
        let entry = if cfg!(windows) && verbatim.exists() {
            verbatim
        } else {
            raw.to_path_buf()
        };
        let from_entry = crate::detect::detect(&entry).expect("detect second entry form");

        let data_dir = Path::new("/var/data/andromeda-pulse");
        assert_eq!(
            workspace_key(Some(&from_raw), data_dir),
            workspace_key(Some(&from_entry), data_dir),
        );
    }

    #[test]
    fn published_key_round_trips_through_the_data_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let key = r"\\?\C:\dev\payments";
        publish_workspace_key(tmp.path(), key).expect("publish");
        assert_eq!(
            read_published_workspace_key(tmp.path()).as_deref(),
            Some(key)
        );
    }

    #[test]
    fn absent_published_key_reads_as_none() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(read_published_workspace_key(tmp.path()), None);
    }

    #[test]
    fn oversized_published_key_is_rejected() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let key = "x".repeat(MAX_WORKSPACE_KEY_BYTES + 1);
        publish_workspace_key(tmp.path(), &key).expect("publish");
        assert_eq!(read_published_workspace_key(tmp.path()), None);
    }

    #[test]
    fn empty_or_control_character_published_key_is_rejected() {
        let tmp = tempfile::tempdir().expect("tempdir");
        publish_workspace_key(tmp.path(), "").expect("publish empty");
        assert_eq!(read_published_workspace_key(tmp.path()), None);
        publish_workspace_key(tmp.path(), "a\u{0}b").expect("publish nul");
        assert_eq!(read_published_workspace_key(tmp.path()), None);
    }

    #[test]
    fn published_key_survives_a_trailing_newline() {
        let tmp = tempfile::tempdir().expect("tempdir");
        publish_workspace_key(tmp.path(), "/home/dev/payments\n").expect("publish");
        assert_eq!(
            read_published_workspace_key(tmp.path()).as_deref(),
            Some("/home/dev/payments"),
        );
    }
}
