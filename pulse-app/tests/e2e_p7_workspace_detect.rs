//! P7 E2E coverage — workspace.detect critical path per test-plan §6 P7.
//!
//! Drives `workspace_detector::detect` directly (the same function the
//! `workspace.detect` TauRPC resolver calls; chunk #50 documents in
//! plan.md Open Question 3 that mock_builder()-based TauRPC roundtrip
//! is а fallback path; the direct-function-call form here preserves the
//! cross-crate data-flow coverage P7 requires).

use std::fs;

use assert_fs::TempDir;
use workspace_detector::contract::VcsType;
use workspace_detector::detect::detect;

#[test]
fn p7_detect_workspace_with_andromeda_marker_returns_marker_true() {
    let tmp = TempDir::new().expect("tempdir");
    fs::create_dir(tmp.path().join(".andromeda")).expect("create .andromeda/");

    let ctx = detect(tmp.path()).expect("detect succeeds on existing dir");

    assert!(
        ctx.has_andromeda_marker,
        "marker .andromeda/ present should yield has_andromeda_marker = true"
    );
    assert!(
        ctx.project_name.is_some(),
        "project_name derived from tmpdir basename"
    );
    assert_eq!(
        ctx.root,
        tmp.path().canonicalize().expect("tmp.path() canonicalizes")
    );
}

#[test]
fn p7_detect_workspace_without_andromeda_marker_returns_marker_false() {
    let tmp = TempDir::new().expect("tempdir");

    let ctx = detect(tmp.path()).expect("detect succeeds even without marker");

    assert!(
        !ctx.has_andromeda_marker,
        "no .andromeda/ should yield has_andromeda_marker = false"
    );
}

#[test]
fn p7_detect_with_git_repo_reports_vcs_git() {
    let tmp = TempDir::new().expect("tempdir");
    fs::create_dir_all(tmp.path().join(".git")).expect("create .git/");
    fs::write(tmp.path().join(".git/HEAD"), "ref: refs/heads/main\n").expect("write HEAD");

    let ctx = detect(tmp.path()).expect("detect succeeds in git-repo tempdir");

    let vcs = ctx.vcs.expect("vcs metadata present in git-repo tempdir");
    assert_eq!(vcs.vcs_type, VcsType::Git);
}

#[test]
fn p7_detect_rejects_path_traversal_input() {
    // Per security plan §Input Validation row 6 + plan.md security canary
    // for `ANDROMEDA_PULSE_DATA_DIR=../../../etc` style payloads — `..`
    // components in candidate path produce PathTraversalRejected before
    // any FS operation.
    let candidate = std::path::Path::new("../../../etc");
    let result = detect(candidate);
    assert!(
        matches!(
            result,
            Err(workspace_detector::contract::Error::PathTraversalRejected { .. })
        ),
        "candidate с .. components must be rejected; got {:?}",
        result
    );
}

#[test]
fn p7_detect_nonexistent_path_returns_canonicalization_failed() {
    let tmp = TempDir::new().expect("tempdir");
    let nonexistent = tmp.path().join("does-not-exist");

    let result = detect(&nonexistent);

    assert!(
        matches!(
            result,
            Err(workspace_detector::contract::Error::CanonicalizationFailed { .. })
        ),
        "nonexistent path must map to CanonicalizationFailed; got {:?}",
        result
    );
}
