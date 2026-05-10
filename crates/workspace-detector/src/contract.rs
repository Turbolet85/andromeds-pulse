use std::path::PathBuf;

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
}
