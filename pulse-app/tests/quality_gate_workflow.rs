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

// Line-anchored TOML section extraction: header lines sit at column 0;
// comment lines mentioning bracketed profile names must not terminate the
// window. CRLF-safe via str::lines() per testing.md 2026-05-14.
fn nextest_profile_window(content: &str, header: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim() == header)
        .unwrap_or_else(|| panic!(".config/nextest.toml MUST declare `{header}` per chunk #99"));
    lines[start + 1..]
        .iter()
        .take_while(|l| !l.trim_start().starts_with("[profile."))
        .copied()
        .collect::<Vec<&str>>()
        .join("\n")
}

#[test]
fn nextest_default_profile_excludes_load_profiles_suite() {
    let content = read_nextest_config();
    let window = nextest_profile_window(&content, "[profile.default]");
    assert!(
        window.contains("default-filter") && window.contains("not binary(perf_load_profiles)"),
        "`[profile.default]` MUST exclude `binary(perf_load_profiles)` via \
         default-filter so the ~12-minute four-profile suite never runs in \
         default/ci/coverage invocations (chunk #99; no #[ignore] gating \
         per quarantine-tracking discipline, no new env var per the \
         arch registry freeze)"
    );
}

#[test]
fn nextest_default_profile_excludes_perf_budget_samples_producer() {
    let content = read_nextest_config();
    let window = nextest_profile_window(&content, "[profile.default]");
    assert!(
        window.contains("not binary(perf_budget_samples)"),
        "`[profile.default]` MUST exclude `binary(perf_budget_samples)`: the producer runs \
         only under `[profile.perf-samples]`, so workspace and coverage runs never pay for it"
    );
    let samples = nextest_profile_window(&content, "[profile.perf-samples]");
    assert!(
        samples.contains("\"binary(perf_budget_samples)\"") && samples.contains("retries = 0"),
        "`[profile.perf-samples]` MUST select exactly the producer with retries = 0"
    );
}

// The job block runs from its `  {name}:` header to the next job header at
// the same two-space indent.
fn workflow_job_block(content: &str, job: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let header = format!("  {job}:");
    let start = lines
        .iter()
        .position(|l| *l == header)
        .unwrap_or_else(|| panic!("ci.yml MUST declare the `{job}` job"));
    lines[start + 1..]
        .iter()
        .take_while(|l| {
            let is_job_header = l.starts_with("  ")
                && !l.starts_with("   ")
                && !l.trim_start().starts_with('#')
                && l.trim_end().ends_with(':');
            !is_job_header
        })
        .copied()
        .collect::<Vec<&str>>()
        .join("\n")
}

#[test]
fn ci_workflow_release_job_owns_and_saves_its_cache_key() {
    let block = workflow_job_block(&read_workflow(), "release");
    assert!(
        block.contains("shared-key: release-${{ runner.os }}"),
        "the release job MUST own `release-${{{{ runner.os }}}}`: restoring lint-test's key \
         leaves it cold once lint-test re-saves without the release dependencies"
    );
    assert!(
        !block.contains("save-if: false"),
        "the release job MUST save its own key (no `save-if: false`); block:\n{block}"
    );
}

#[test]
fn nextest_load_profiles_profile_preserves_zero_flake_posture() {
    let content = read_nextest_config();
    let window = nextest_profile_window(&content, "[profile.load-profiles]");
    assert!(
        window.contains("retries = 0"),
        "`[profile.load-profiles]` MUST keep `retries = 0` (zero-flake \
         budget applies to the release-gate load suite; test-plan §10)"
    );
    assert!(
        window.contains("default-filter") && window.contains("\"binary(perf_load_profiles)\""),
        "`[profile.load-profiles]` MUST re-select exactly the \
         perf_load_profiles binary via default-filter (chunk #99)"
    );
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
         (chunk #55 lint gate regression backstop; `-D warnings` cannot be downgraded to `-W warnings` \
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
        "--profile perf-samples",
        "perf:budget --data-dir",
        "perf:frame-sample",
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
        "ci.yml MUST preserve `Upload criterion bench artifact` step with \
         `name: criterion-${{ runner.os }}` (chunk #56 regression backstop \
         — chunk #54 substrate must remain)"
    );
}

#[test]
fn ci_workflow_uploads_logs_artifact_unchanged() {
    let content = read_workflow();
    assert!(
        content.contains("name: logs-${{ runner.os }}"),
        "ci.yml MUST preserve `Upload logs artifact` step with `name: logs-${{ runner.os }}` \
         (chunk #56 regression backstop — chunk #54 substrate must remain; obs-plan §9 \
         CI failure → artifact triage workflow requires log file artifact upload)"
    );
    // The boot smoke writes the log ci-gates reads in its own job, so its
    // upload needs a name that cannot collide with lint-test's per-OS one.
    assert!(
        content.contains("name: logs-boot-${{ runner.os }}"),
        "ci.yml MUST upload the boot job's logs as `logs-boot-${{ runner.os }}` \
         (obs-plan §9 artifact triage; upload-artifact v4 refuses a duplicate name)"
    );
}

fn read_named_workflow(name: &str) -> String {
    let full_path = project_root().join(".github/workflows").join(name);
    std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("read {} failed: {e}", full_path.display()))
}

// The `runner`, `job` and `steps` contexts (and `env` self-reference) exist only
// at step level; a workflow- or job-level `env:` value naming one fails workflow
// parsing before any job starts (GitHub Actions context-availability table).
const STEP_ONLY_CONTEXTS: [&str; 4] = ["runner.", "job.", "steps.", "env."];

fn names_step_only_context(value: &str) -> Option<&'static str> {
    let mut rest = value;
    while let Some(open) = rest.find("${{") {
        let after = &rest[open + 3..];
        let close = after.find("}}").unwrap_or(after.len());
        let expr = &after[..close];
        for ctx in STEP_ONLY_CONTEXTS {
            for (idx, _) in expr.match_indices(ctx) {
                let boundary = expr[..idx]
                    .chars()
                    .next_back()
                    .is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'));
                if boundary {
                    return Some(ctx);
                }
            }
        }
        rest = &after[close..];
    }
    None
}

fn env_block_lines<'a>(lines: &[&'a str], header_idx: usize, indent: usize) -> Vec<&'a str> {
    lines[header_idx + 1..]
        .iter()
        .take_while(|l| l.trim().is_empty() || l.len() - l.trim_start().len() > indent)
        .copied()
        .collect()
}

#[test]
fn workflow_env_references_no_step_only_context() {
    let mut blocks_checked = 0;
    for name in [
        "ci.yml",
        "release.yml",
        "update-channels.yml",
        "secret-scan.yml",
    ] {
        let content = read_named_workflow(name);
        let lines: Vec<&str> = content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            let indent = match *line {
                "env:" => 0,
                "    env:" => 4,
                _ => continue,
            };
            blocks_checked += 1;
            for value in env_block_lines(&lines, idx, indent) {
                if let Some(ctx) = names_step_only_context(value) {
                    panic!(
                        "{name}: a workflow- or job-level `env:` value references the step-only \
                         `{ctx}` context, which fails workflow parsing before any job starts \
                         (export it from a step via $GITHUB_ENV instead): {value:?}"
                    );
                }
            }
        }
    }
    assert!(
        blocks_checked >= 3,
        "expected at least the three workflow-level `env:` blocks (ci, release, \
         update-channels); found {blocks_checked}"
    );
}

#[test]
fn data_dir_export_precedes_every_consumer() {
    let expected = [
        ("ci.yml", 7),
        ("release.yml", 1),
        ("update-channels.yml", 2),
    ];
    for (name, job_count) in expected {
        let content = read_named_workflow(name);
        let lines: Vec<&str> = content.lines().collect();
        let jobs_idx = lines
            .iter()
            .position(|l| *l == "jobs:")
            .unwrap_or_else(|| panic!("{name} has no `jobs:` key"));
        let job_starts: Vec<usize> = (jobs_idx + 1..lines.len())
            .filter(|&i| {
                let l = lines[i];
                l.starts_with("  ") && !l.starts_with("   ") && l.trim_end().ends_with(':')
            })
            .collect();
        assert_eq!(
            job_starts.len(),
            job_count,
            "{name}: expected {job_count} jobs, found {}",
            job_starts.len()
        );
        for (n, &start) in job_starts.iter().enumerate() {
            let end = job_starts.get(n + 1).copied().unwrap_or(lines.len());
            let step_names: Vec<&str> = lines[start..end]
                .iter()
                .filter_map(|l| l.strip_prefix("      - name: "))
                .collect();
            let job = lines[start].trim();
            assert_eq!(
                step_names.first().copied(),
                Some("Harden runner"),
                "{name} {job}: `Harden runner` must stay the first step"
            );
            assert_eq!(
                step_names.get(1).copied(),
                Some("Export ANDROMEDA_PULSE_DATA_DIR"),
                "{name} {job}: the `Export ANDROMEDA_PULSE_DATA_DIR` step must sit directly \
                 after `Harden runner`, before any step that reads the data dir"
            );
        }
    }
}
