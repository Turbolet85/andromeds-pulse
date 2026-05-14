//! Chunks #55 + #56 workflow self-lint tests. Asserts `.github/workflows/ci.yml`
//! and `.config/nextest.toml` preserve the quality-gate and obs-gate posture
//! introduced by these chunks (zero-flake retry policy, coverage regression
//! gate, perf-budget regression gate, lint/typecheck gate, criterion bench
//! regression gate, CI subscriber default fields, no `continue-on-error`
//! on gate steps). Mirrors chunk #54 `a11y_perf_workflow.rs` shape — pure
//! file-read + substring/pattern assertion; no Tauri runtime; no network.

use std::path::PathBuf;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("pulse-app has parent (workspace root)")
        .to_path_buf()
}

fn read_workflow() -> String {
    let full_path = project_root().join(".github/workflows/ci.yml");
    std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("read {} failed: {e}", full_path.display()))
}

fn read_nextest_config() -> String {
    let full_path = project_root().join(".config/nextest.toml");
    std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("read {} failed: {e}", full_path.display()))
}

#[test]
fn nextest_ci_profile_enforces_zero_retries() {
    let content = read_nextest_config();
    let profile_idx = content.find("[profile.ci]").expect(
        ".config/nextest.toml MUST declare `[profile.ci]` per chunk #55 zero-flake retry policy",
    );
    let profile_block = &content[profile_idx..];
    let retries_line = profile_block
        .lines()
        .find(|line| line.trim_start().starts_with("retries"))
        .expect(
            "[profile.ci] MUST contain `retries = ...` per test-plan §10 zero-flakiness budget",
        );
    assert!(
        retries_line.contains("= 0") || retries_line.contains("=0"),
        "[profile.ci] MUST declare `retries = 0` (zero-flake retry policy); found: `{retries_line}`"
    );
}

#[test]
fn ci_workflow_invokes_quarantine_tracking_check() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask quarantine-tracking"),
        "ci.yml MUST invoke `cargo xtask quarantine-tracking` per chunk #55 \
         quarantine convention enforcement (test-plan §11 `#[ignore]` GitHub \
         issue URL tracking)"
    );
}

#[test]
fn ci_workflow_invokes_coverage_regression_check() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask coverage-regression"),
        "ci.yml MUST invoke `cargo xtask coverage-regression` per chunk #55 \
         coverage regression gate (test-plan §10 + §11 no-decrease policy)"
    );
}

#[test]
fn ci_workflow_downloads_coverage_baseline_artifact() {
    let content = read_workflow();
    let download_idx = content.find("name: coverage-linux-base").expect(
        "ci.yml MUST have download-artifact step with `name: coverage-linux-base` per chunk #55",
    );
    let window_start = download_idx.saturating_sub(400);
    let window = &content[window_start..download_idx];
    assert!(
        window.contains("if: github.event_name == 'pull_request'"),
        "coverage-linux-base download step MUST be gated `if: github.event_name == 'pull_request'` per chunk #54 PR-only baseline pattern"
    );
    assert!(
        window.contains("actions/download-artifact@"),
        "coverage-linux-base download step MUST use SHA-pinned actions/download-artifact"
    );
}

#[test]
fn ci_workflow_clippy_uses_deny_warnings() {
    let content = read_workflow();
    assert!(
        content.contains("cargo clippy --workspace --all-targets --all-features -- -D warnings"),
        "ci.yml MUST invoke `cargo clippy --workspace --all-targets --all-features -- -D warnings` \
         (chunk #55 lint gate regression backstop; `-D warnings` cannot be downgraded к `-W warnings` \
         per security plan §Dependency Security CI integration + CLAUDE.md §Workflow)"
    );
}

#[test]
fn ci_workflow_test_gates_no_continue_on_error() {
    let content = read_workflow();
    let gate_substrings = [
        "cargo fmt --check",
        "cargo clippy --workspace",
        "cargo nextest run",
        "cargo deny check",
        "cargo xtask test",
        "cargo xtask quarantine-tracking",
        "cargo xtask coverage-regression",
        "cargo xtask criterion-regression",
        "cargo xtask ci-gates",
        "cargo xtask test:a11y",
        "cargo xtask perf:slo-load",
        "cargo xtask capability-drift",
    ];
    let step_pattern = "      - name: ";
    let step_starts: Vec<usize> = content
        .match_indices(step_pattern)
        .map(|(i, _)| i)
        .collect();
    assert!(
        !step_starts.is_empty(),
        "ci.yml MUST contain at least one `      - name:` step (sanity check on indentation pattern)"
    );
    for (idx, &start) in step_starts.iter().enumerate() {
        let end = step_starts.get(idx + 1).copied().unwrap_or(content.len());
        let block = &content[start..end];
        let contains_gate = gate_substrings.iter().any(|s| block.contains(s));
        if !contains_gate {
            continue;
        }
        assert!(
            !block.contains("continue-on-error: true"),
            "ci.yml step block contains gate command AND `continue-on-error: true` (forbidden \
             per test-plan §11 + §10 Build failure conditions); block:\n{block}"
        );
    }
}

#[test]
fn ci_workflow_env_includes_ci_run_id() {
    let content = read_workflow();
    let lines: Vec<&str> = content.lines().collect();
    let env_line_idx = lines
        .iter()
        .position(|line| *line == "env:")
        .expect(
            "ci.yml MUST have workflow-level `env:` block at top-level indentation per obs-plan §9 CI subscriber default fields",
        );
    let jobs_line_idx = lines
        .iter()
        .position(|line| *line == "jobs:")
        .unwrap_or(lines.len());
    let env_block = lines[env_line_idx..jobs_line_idx].join("\n");
    for required in ["CI_RUN_ID:", "GIT_COMMIT_SHA:", "DEPLOYMENT_ENVIRONMENT:"] {
        assert!(
            env_block.contains(required),
            "ci.yml workflow-level `env:` block MUST contain `{required}` per obs-plan §9 CI subscriber default fields; env block:\n{env_block}"
        );
    }
}

#[test]
fn ci_workflow_invokes_ci_gates() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask ci-gates"),
        "ci.yml MUST invoke `cargo xtask ci-gates` per chunk #56 obs gates \
         (zero-spans + zero-panic + heartbeat-gap + perf-budget delegation)"
    );
}

#[test]
fn ci_workflow_invokes_criterion_regression_check() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask criterion-regression"),
        "ci.yml MUST invoke `cargo xtask criterion-regression` per chunk #56 \
         criterion bench regression detection (obs-plan §10 row 4 stable estimators)"
    );
}

#[test]
fn ci_workflow_downloads_criterion_baseline_artifact() {
    let content = read_workflow();
    let download_idx = content
        .find("name: criterion-${{ runner.os }}-base")
        .expect("ci.yml MUST have download-artifact step with `name: criterion-${{ runner.os }}-base` per chunk #56");
    let window_start = download_idx.saturating_sub(400);
    let window = &content[window_start..download_idx];
    assert!(
        window.contains("if: github.event_name == 'pull_request'"),
        "criterion-${{ runner.os }}-base download step MUST be gated `if: github.event_name == 'pull_request'`"
    );
    assert!(
        window.contains("actions/download-artifact@"),
        "criterion-${{ runner.os }}-base download step MUST use SHA-pinned actions/download-artifact"
    );
}

#[test]
fn ci_workflow_uploads_criterion_artifact_unchanged() {
    let content = read_workflow();
    assert!(
        content.contains("name: criterion-${{ runner.os }}"),
        "ci.yml MUST preserve `Upload criterion bench artifact` step с \
         `name: criterion-${{ runner.os }}` (chunk #56 regression backstop \
         — chunk #54 substrate must remain)"
    );
}

#[test]
fn ci_workflow_uploads_logs_artifact_unchanged() {
    let content = read_workflow();
    assert!(
        content.contains("name: logs-${{ runner.os }}"),
        "ci.yml MUST preserve `Upload logs artifact` step с `name: logs-${{ runner.os }}` \
         (chunk #56 regression backstop — chunk #54 substrate must remain; obs-plan §9 \
         CI failure → artifact triage workflow requires log file artifact upload)"
    );
}
