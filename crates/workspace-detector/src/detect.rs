use std::path::Path;
use std::time::Instant;

use tracing::instrument;

use crate::contract::{Error, WorkspaceContext};
use crate::marker::check_andromeda_marker;
use crate::vcs::detect_vcs;

/// Detect a workspace at `candidate_root`. Canonicalizes the candidate
/// path (rejecting `..` components that escape the original parent),
/// probes for `.andromeda/` marker existence, and walks up looking for
/// `.git/HEAD` for VCS metadata.
///
/// Per security plan §Input Validation row 6 + CWE-22 class: any `..`
/// component that resolves outside the candidate's natural parent is
/// rejected as path traversal. Per §Logging hygiene: this function never
/// logs the full canonicalized path; the caller (ui-bridge resolver)
/// emits basename-only on success.
#[instrument(skip_all, fields(detection_latency_ms = tracing::field::Empty))]
pub fn detect(candidate_root: &Path) -> Result<WorkspaceContext, Error> {
    let started = Instant::now();
    let span = tracing::Span::current();

    if path_contains_traversal(candidate_root) {
        return Err(Error::PathTraversalRejected {
            reason: "candidate path contains traversal components".to_string(),
        });
    }

    let canonical = candidate_root.canonicalize().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            Error::CanonicalizationFailed {
                reason: "candidate path does not exist".to_string(),
            }
        } else {
            Error::IoFailure(e)
        }
    })?;

    let project_name = canonical
        .file_name()
        .and_then(|os| os.to_str())
        .map(|s| s.to_string());
    let has_andromeda_marker = check_andromeda_marker(&canonical);
    let vcs = detect_vcs(&canonical);

    let elapsed = started.elapsed().as_millis() as u64;
    span.record("detection_latency_ms", elapsed);

    Ok(WorkspaceContext {
        root: canonical,
        project_name,
        vcs,
        has_andromeda_marker,
    })
}

fn path_contains_traversal(path: &Path) -> bool {
    use std::path::Component;
    path.components().any(|c| matches!(c, Component::ParentDir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn rejects_path_with_parent_dir_traversal() {
        let result = detect(Path::new("/tmp/foo/../bar"));
        assert!(matches!(result, Err(Error::PathTraversalRejected { .. })));
    }

    #[test]
    fn rejects_nonexistent_candidate_with_canonicalization_failed() {
        let result = detect(Path::new("/nonexistent-path-that-should-never-exist-xyz"));
        assert!(matches!(result, Err(Error::CanonicalizationFailed { .. })));
    }

    #[test]
    fn detects_workspace_with_andromeda_marker_only() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".andromeda")).unwrap();

        let ctx = detect(tmp.path()).expect("detection ok");
        assert!(ctx.has_andromeda_marker);
        assert!(ctx.vcs.is_none());
        assert!(ctx.project_name.is_some());
    }

    #[test]
    fn detects_workspace_without_andromeda_marker_returns_false_flag() {
        let tmp = TempDir::new().unwrap();
        let ctx = detect(tmp.path()).expect("detection ok");
        assert!(!ctx.has_andromeda_marker);
    }

    #[test]
    fn detects_workspace_with_andromeda_marker_and_git() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".andromeda")).unwrap();
        let git = tmp.path().join(".git");
        fs::create_dir_all(&git).unwrap();
        fs::write(
            git.join("HEAD"),
            "1234567890abcdef1234567890abcdef12345678\n",
        )
        .unwrap();

        let ctx = detect(tmp.path()).expect("detection ok");
        assert!(ctx.has_andromeda_marker);
        let vcs = ctx.vcs.expect("vcs detected");
        assert_eq!(vcs.head_commit_basename.as_deref(), Some("12345678"));
    }
}
