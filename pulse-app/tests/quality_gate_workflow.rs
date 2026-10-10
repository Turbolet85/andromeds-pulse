//! Workflow self-lint tests. Asserts `.github/workflows/ci.yml` and
//! `.config/nextest.toml` keep the quality-gate and obs-gate posture (the
//! zero-flake retry policy, the Linux-only runner and the trigger block, the
//! lint and quarantine gates, the CI subscriber default fields, no
//! `continue-on-error` on any step, no artifact download without a producer,
//! every upload failing its step when it finds no file, every nextest run
//! failing on an empty selection, the coverage thresholds step failing on a
//! report that tracks nothing, and the boot job's smoke, exit-witness and
//! series steps). Same shape as `a11y_perf_workflow.rs`, a file read and a
//! substring or pattern assertion, with no Tauri runtime and no network; the
//! thresholds witness also runs the step's own script with `bash` and `awk`.

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
fn ci_workflow_runs_on_linux_only() {
    let content = read_workflow();
    let mut runner_lines = 0;
    for (idx, line) in content.lines().enumerate() {
        let number = idx + 1;
        let trimmed = line.trim();
        if trimmed.starts_with("runs-on:") {
            runner_lines += 1;
            assert_eq!(
                trimmed, "runs-on: ubuntu-22.04",
                "ci.yml:{number}: every job MUST run on ubuntu-22.04 (test-plan §9 Pipeline structure)"
            );
        }
        let lowered = line.to_ascii_lowercase();
        for token in ["macos", "windows", "matrix.os", "runner.os =="] {
            assert!(
                !lowered.contains(token),
                "ci.yml:{number}: the workflow MUST name no other system, matrix value or \
                 system condition; found `{token}` in: {line}"
            );
        }
    }
    assert!(
        runner_lines > 0,
        "ci.yml MUST carry at least one `runs-on:` line (sanity check on the line pattern)"
    );
}

// With no other-system release job, these two steps are the only
// release-profile builds the workflow runs.
#[test]
fn ci_workflow_keeps_the_linux_release_build_witnesses() {
    let content = read_workflow();
    let supply_chain = workflow_job_block(&content, "supply-chain");
    assert!(
        supply_chain.contains("cargo auditable build --workspace --release"),
        "the supply-chain job MUST keep `cargo auditable build --workspace --release` \
         (test-plan §9 Pipeline structure, Supply chain row); block:\n{supply_chain}"
    );
    let boot = workflow_job_block(&content, "boot");
    assert!(
        boot.contains("cargo build --workspace --release --features mcp-server"),
        "the boot job MUST keep `cargo build --workspace --release --features mcp-server` \
         (test-plan §9 Pipeline structure, Release build row); block:\n{boot}"
    );
}

// The trigger block is pinned whole, so no other event can be added unseen.
#[test]
fn ci_workflow_triggers_are_pull_request_and_push_on_main() {
    let content = read_workflow();
    let lines: Vec<&str> = content.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == "on:")
        .expect("ci.yml MUST declare a top-level `on:` block");
    let triggers: Vec<&str> = lines[start + 1..]
        .iter()
        .take_while(|line| line.is_empty() || line.starts_with(' ') || line.starts_with('#'))
        .copied()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .collect();
    assert_eq!(
        triggers,
        ["  pull_request: {}", "  push:", "    branches: [main]"],
        "ci.yml MUST run on a pull request and on a push to main, and on nothing else \
         (architecture §Infrastructure Patterns, CI/CD approach)"
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
        "cargo audit",
        "cargo xtask test",
        "cargo xtask quarantine-tracking",
        "cargo xtask ci-gates",
        "cargo xtask test:a11y",
        "cargo xtask perf:slo-load",
        "cargo xtask capability-drift",
        "--profile perf-samples",
        "perf:budget --data-dir",
        "perf:frame-sample",
    ];
    let blocks = workflow_step_blocks(&content);
    assert!(
        !blocks.is_empty(),
        "ci.yml MUST contain at least one `      - name:` step (sanity check on indentation pattern)"
    );
    for block in &blocks {
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
    // Every step, whatever it runs: a soft-fail key on a step the list above
    // does not name passes a failed step as well.
    for block in &blocks {
        assert!(
            !block.contains("continue-on-error"),
            "ci.yml: no step may carry `continue-on-error` (test-plan §11 Test \
             Anti-Patterns); block:\n{block}"
        );
    }
}

const STEP_NAME_PREFIX: &str = "      - name: ";

// One block per step, from its `      - name: ` line to the next one.
fn workflow_step_blocks(content: &str) -> Vec<&str> {
    let starts: Vec<usize> = content
        .match_indices(STEP_NAME_PREFIX)
        .map(|(i, _)| i)
        .collect();
    starts
        .iter()
        .enumerate()
        .map(|(idx, &start)| {
            let end = starts.get(idx + 1).copied().unwrap_or(content.len());
            &content[start..end]
        })
        .collect()
}

// The artifact name a step block hands to `action`, or None when the block
// does not use that action.
fn artifact_step_name<'a>(block: &'a str, action: &str) -> Option<&'a str> {
    let uses = format!("uses: {action}@");
    if !block.lines().any(|l| l.trim_start().starts_with(&uses)) {
        return None;
    }
    let name = block
        .lines()
        .find_map(|l| l.trim_start().strip_prefix("name: "))
        .unwrap_or_else(|| panic!("a `{action}` step MUST name its artifact; block:\n{block}"));
    Some(name.trim_end())
}

fn uploaded_artifact_names() -> Vec<String> {
    let dir = project_root().join(".github/workflows");
    let mut names = Vec::new();
    let entries =
        std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {} failed: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("read a workflow dir entry").path();
        if !path
            .extension()
            .is_some_and(|ext| ext == "yml" || ext == "yaml")
        {
            continue;
        }
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {} failed: {e}", path.display()));
        for block in workflow_step_blocks(&content) {
            if let Some(name) = artifact_step_name(block, "actions/upload-artifact") {
                names.push(name.to_string());
            }
        }
    }
    names
}

#[test]
fn ci_workflow_makes_no_download_without_a_producer() {
    let uploaded = uploaded_artifact_names();
    assert!(
        !uploaded.is_empty(),
        "the workflows MUST hold at least one upload step (sanity check on the step parse)"
    );
    let content = read_workflow();
    for block in workflow_step_blocks(&content) {
        let Some(artifact) = artifact_step_name(block, "actions/download-artifact") else {
            continue;
        };
        assert!(
            uploaded.iter().any(|name| name == artifact),
            "ci.yml downloads `{artifact}`, which no workflow uploads under that name: a \
             download without a producer reads nothing (test-plan §9 Pipeline structure); \
             uploaded names: {uploaded:?}; block:\n{block}"
        );
    }
}

#[test]
fn ci_workflow_uploads_fail_when_they_find_no_file() {
    let content = read_workflow();
    let uploads: Vec<&str> = workflow_step_blocks(&content)
        .into_iter()
        .filter(|block| artifact_step_name(block, "actions/upload-artifact").is_some())
        .collect();
    assert!(
        !uploads.is_empty(),
        "ci.yml MUST hold at least one upload step (sanity check on the step parse)"
    );
    for block in uploads {
        assert!(
            block
                .lines()
                .any(|line| line.trim() == "if-no-files-found: error"),
            "every upload step of ci.yml MUST carry `if-no-files-found: error`, so an upload \
             that finds no file fails its step (obs-plan §9 Telemetry artifact handling); \
             block:\n{block}"
        );
    }
}

#[test]
fn ci_workflow_audit_step_is_a_plain_run_step() {
    let content = read_workflow();
    let supply_chain = workflow_job_block(&content, "supply-chain");
    let job_lines: Vec<&str> = supply_chain.lines().collect();
    let name_line = "      - name: cargo audit (RustSec advisory DB)";
    let start = job_lines
        .iter()
        .position(|line| *line == name_line)
        .unwrap_or_else(|| panic!("the supply-chain job MUST keep the step `{name_line}`"));
    let step = job_lines[start + 1..]
        .iter()
        .take_while(|line| !line.starts_with("      - name: "))
        .copied()
        .collect::<Vec<&str>>()
        .join("\n");
    assert!(
        step.lines().any(|line| line.trim() == "run: cargo audit"),
        "the audit step MUST be the plain step `run: cargo audit`, whose exit is the same on a \
         push and on a pull request (security-plan §Dependency Security, CI integration; \
         P-120); step:\n{step}"
    );
    for banned in ["uses:", "with:", "token", "continue-on-error", "||"] {
        assert!(
            !step.contains(banned),
            "the audit step MUST hold no `{banned}`: no action, no token and no soft-fail \
             (security-plan §Dependency Security, CI integration; P-120); step:\n{step}"
        );
    }
    assert!(
        !job_lines
            .iter()
            .any(|line| line.trim_start().starts_with("permissions:")),
        "the supply-chain job MUST carry no `permissions:` key of its own (security-plan \
         §Dependency Security, CI integration; P-120); block:\n{supply_chain}"
    );
    assert!(
        !content.contains("rustsec/audit-check"),
        "ci.yml MUST NOT use `rustsec/audit-check`: on a push it fails on its own reporting \
         (security-plan §Dependency Security, CI integration; P-120)"
    );
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
         (the record count and the panic read over the boot log)"
    );
}

// The boot smoke writes the log ci-gates reads, in the boot job.
#[test]
fn ci_workflow_uploads_logs_artifact_unchanged() {
    let content = read_workflow();
    assert!(
        content.contains("name: logs-boot-${{ runner.os }}"),
        "ci.yml MUST upload the boot job's logs as `logs-boot-${{ runner.os }}` \
         (obs-plan §9 artifact triage; upload-artifact v4 refuses a duplicate name)"
    );
}

// The boot job's smoke step without its comment lines, each line trimmed.
fn boot_smoke_step_lines(content: &str) -> Vec<String> {
    let boot = workflow_job_block(content, "boot");
    let lines: Vec<&str> = boot.lines().collect();
    let name_line = "      - name: Boot pulse-app smoke";
    let start = lines
        .iter()
        .position(|line| *line == name_line)
        .unwrap_or_else(|| panic!("the boot job MUST keep the step `{name_line}`"));
    lines[start + 1..]
        .iter()
        .take_while(|line| !line.starts_with("      - name: "))
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect()
}

fn only_line_with(step: &[String], needle: &str) -> usize {
    let hits: Vec<usize> = step
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains(needle))
        .map(|(idx, _)| idx)
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "the boot smoke step MUST hold exactly one line with `{needle}` (test-plan §9 \
         Pipeline structure, Boot smoke row); step:\n{}",
        step.join("\n")
    );
    hits[0]
}

#[test]
fn ci_workflow_boot_smoke_reads_the_app_past_its_settle_inside_one_display() {
    let step = boot_smoke_step_lines(&read_workflow());
    let display = only_line_with(&step, "xvfb-run ");
    assert!(
        step[display].ends_with("bash -c '"),
        "one `xvfb-run … bash -c '` MUST span the whole sequence, so the display server \
         outlives boot's backgrounded app; line: {}",
        step[display]
    );
    let order = [
        display,
        only_line_with(&step, "scripts/agent-run.sh boot"),
        only_line_with(&step, "cargo xtask harness:settled"),
        only_line_with(&step, "scripts/agent-run.sh status"),
        only_line_with(&step, "scripts/agent-run.sh cleanup"),
    ];
    assert!(
        order.windows(2).all(|pair| pair[0] < pair[1]),
        "the smoke MUST run boot, harness:settled, status, cleanup in that order inside the \
         one xvfb-run (test-plan §9 Pipeline structure, Boot smoke row); step:\n{}",
        step.join("\n")
    );
    let close = step
        .iter()
        .rposition(|line| line == "'")
        .expect("the `bash -c '` block MUST close on its own line");
    assert!(
        close > order[4],
        "cleanup MUST sit inside the one xvfb-run; step:\n{}",
        step.join("\n")
    );
}

#[test]
fn ci_workflow_boot_smoke_runs_cleanup_whatever_settled_and_status_returned() {
    let step = boot_smoke_step_lines(&read_workflow());
    let settled = only_line_with(&step, "cargo xtask harness:settled");
    let status = only_line_with(&step, "scripts/agent-run.sh status");
    let cleanup = only_line_with(&step, "scripts/agent-run.sh cleanup");
    for (idx, capture) in [(settled, "; a=$?"), (status, "; b=$?"), (cleanup, "; c=$?")] {
        assert!(
            step[idx].ends_with(capture),
            "the smoke MUST record this verb's exit and go on (`{capture}`), so cleanup runs \
             whatever the two before it returned; line: {}",
            step[idx]
        );
    }
    let all_three = only_line_with(&step, r#"test "$a$b$c" = 000"#);
    assert!(
        all_three > cleanup,
        "the step MUST fail unless harness:settled, status and cleanup all returned 0"
    );
    assert!(
        !step.iter().any(|line| line == "set -e"),
        "`set -e` would end the sequence at a red settle verdict, before cleanup; step:\n{}",
        step.join("\n")
    );
}

#[test]
fn ci_workflow_boot_smoke_keeps_the_display_server_output_in_the_logs_artifact() {
    let content = read_workflow();
    let step = boot_smoke_step_lines(&content);
    let display = only_line_with(&step, "xvfb-run ");
    assert!(
        step[display].contains(r#"-e "$ANDROMEDA_PULSE_DATA_DIR/logs/xvfb.log""#),
        "xvfb-run MUST write the display server's own error output to `logs/xvfb.log` under \
         the data dir (obs-plan §9 Telemetry artifact handling); line: {}",
        step[display]
    );
    let logs_dir = only_line_with(&step, r#"mkdir -p "$ANDROMEDA_PULSE_DATA_DIR/logs""#);
    assert!(
        logs_dir < display,
        "the data dir's `logs/` MUST exist before xvfb-run opens its error file there"
    );
    assert!(
        workflow_job_block(&content, "boot")
            .contains("path: ${{ env.ANDROMEDA_PULSE_DATA_DIR }}/logs/"),
        "the boot job MUST upload the same `logs/` dir the smoke writes into"
    );
}

#[test]
fn ci_workflow_boot_smoke_carries_no_soft_fail() {
    let step = boot_smoke_step_lines(&read_workflow());
    for banned in ["continue-on-error", "retry", "|| true"] {
        assert!(
            !step.iter().any(|line| line.contains(banned)),
            "the boot smoke step MUST hold no `{banned}`: the smoke is a gating step \
             (test-plan §11 Test Anti-Patterns); step:\n{}",
            step.join("\n")
        );
    }
}

const BOOT_SMOKE_STEP: &str = "Boot pulse-app smoke";
const EXIT_WITNESS_STEP: &str = "Build the exit witness";
const BOOT_SERIES_STEP: &str = "Boot series (equal source)";

// Where a named step of the boot job starts, and its lines up to the next
// step without comment lines, each trimmed.
fn boot_job_step(content: &str, name: &str) -> (usize, Vec<String>) {
    let boot = workflow_job_block(content, "boot");
    let lines: Vec<&str> = boot.lines().collect();
    let name_line = format!("      - name: {name}");
    let start = lines
        .iter()
        .position(|line| *line == name_line)
        .unwrap_or_else(|| panic!("the boot job MUST hold the step `{name_line}`"));
    let step = lines[start + 1..]
        .iter()
        .take_while(|line| !line.starts_with("      - name: "))
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect();
    (start, step)
}

#[test]
fn ci_workflow_boot_job_builds_the_exit_witness_before_the_smoke() {
    let content = read_workflow();
    let (build, step) = boot_job_step(&content, EXIT_WITNESS_STEP);
    let (smoke, _) = boot_job_step(&content, BOOT_SMOKE_STEP);
    assert!(
        build < smoke,
        "the library MUST be built before the smoke step that loads it (test-plan §9 \
         Pipeline structure, Boot smoke row)"
    );
    assert!(
        step.iter().any(|line| line.starts_with("cc ")
            && line.contains("-shared")
            && line.contains("scripts/exit-witness.c")),
        "the step MUST build the library from `scripts/exit-witness.c` with the runner's own \
         compiler; step:\n{}",
        step.join("\n")
    );
    assert!(
        step.iter()
            .any(|line| line.contains("ANDROMEDA_PULSE_EXIT_WITNESS_LIB=")
                && line.ends_with(r#">> "$GITHUB_ENV""#)),
        "the step MUST name the built library to the harness through `$GITHUB_ENV` as \
         `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`; step:\n{}",
        step.join("\n")
    );
    assert!(
        !step.iter().any(|line| line.contains("LD_PRELOAD")),
        "the library is loaded on the boot verb's one spawn line, never through the job's \
         environment; step:\n{}",
        step.join("\n")
    );
}

#[test]
fn ci_workflow_boot_series_runs_after_the_smoke_whatever_it_returned_and_before_ci_gates() {
    let content = read_workflow();
    let (series, step) = boot_job_step(&content, BOOT_SERIES_STEP);
    let (smoke, _) = boot_job_step(&content, BOOT_SMOKE_STEP);
    let (ci_gates, _) = boot_job_step(&content, "cargo xtask ci-gates");
    assert!(
        smoke < series && series < ci_gates,
        "the series MUST sit after the smoke and before `ci-gates` (test-plan §9 Pipeline \
         structure, Boot smoke row)"
    );
    assert!(
        step.iter().any(|line| line == "if: always()"),
        "the series MUST run when the smoke is red too (`if: always()`); step:\n{}",
        step.join("\n")
    );
    assert!(
        step.iter()
            .any(|line| line == "run: cargo xtask harness:boot-series --count 7"),
        "the series MUST be the plain step `run: cargo xtask harness:boot-series --count 7`; \
         step:\n{}",
        step.join("\n")
    );
}

#[test]
fn ci_workflow_boot_series_carries_no_soft_fail() {
    let (_, step) = boot_job_step(&read_workflow(), BOOT_SERIES_STEP);
    for banned in ["continue-on-error", "retry", "|| true"] {
        assert!(
            !step.iter().any(|line| line.contains(banned)),
            "the boot series step MUST hold no `{banned}`: a boot that ends by itself fails the \
             job (test-plan §11 Test Anti-Patterns); step:\n{}",
            step.join("\n")
        );
    }
}

const COVERAGE_THRESHOLDS_STEP: &str = "Enforce coverage thresholds";
const RUN_SCRIPT_INDENT: &str = "          ";

// The script of a `run: |` step, cut out of its job by the step's name.
fn workflow_run_script(content: &str, job: &str, step: &str) -> String {
    let block = workflow_job_block(content, job);
    let lines: Vec<&str> = block.lines().collect();
    let name_line = format!("{STEP_NAME_PREFIX}{step}");
    let start = lines
        .iter()
        .position(|line| *line == name_line)
        .unwrap_or_else(|| panic!("the {job} job MUST hold the step `{name_line}`"));
    assert_eq!(
        lines.get(start + 1).copied(),
        Some("        run: |"),
        "the step `{step}` MUST be an inline `run: |` step"
    );
    lines[start + 2..]
        .iter()
        .take_while(|line| line.trim().is_empty() || line.starts_with(RUN_SCRIPT_INDENT))
        .map(|line| line.strip_prefix(RUN_SCRIPT_INDENT).unwrap_or(""))
        .collect::<Vec<&str>>()
        .join("\n")
}

fn lcov_report(lines: Option<(u32, u32)>, functions: Option<(u32, u32)>) -> String {
    let mut report = String::from("SF:src/lib.rs\n");
    if let Some((hit, found)) = functions {
        report.push_str(&format!("FNF:{found}\nFNH:{hit}\n"));
    }
    if let Some((hit, found)) = lines {
        report.push_str(&format!("LF:{found}\nLH:{hit}\n"));
    }
    report.push_str("end_of_record\n");
    report
}

// Runs the thresholds step as the runner does (`bash -e`), in a scratch dir
// that holds `report` as `lcov.info`, or no report. Its exit code and
// everything it printed.
fn run_coverage_thresholds_step(report: Option<&str>) -> (Option<i32>, String) {
    let awk = std::process::Command::new("awk")
        .arg("BEGIN { exit 0 }")
        .status()
        .expect("`awk` MUST be on PATH: the thresholds step sums the report with it");
    assert!(awk.success(), "`awk` MUST run a program; status: {awk}");
    let scratch = tempfile::tempdir().expect("create a scratch dir");
    if let Some(report) = report {
        std::fs::write(scratch.path().join("lcov.info"), report).expect("write the report");
    }
    let script = scratch.path().join("step.sh");
    let body = workflow_run_script(&read_workflow(), "coverage", COVERAGE_THRESHOLDS_STEP);
    std::fs::write(&script, body).expect("write the step's script");
    let output = std::process::Command::new("bash")
        .arg("-e")
        .arg(&script)
        .current_dir(scratch.path())
        .output()
        .expect("`bash` MUST be on PATH: the thresholds step is a bash script");
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    (output.status.code(), printed)
}

#[test]
fn coverage_thresholds_step_fails_on_a_report_tracking_nothing() {
    let cases = [
        ("an empty report", String::new()),
        ("lines and no function", lcov_report(Some((90, 100)), None)),
        ("functions and no line", lcov_report(None, Some((9, 10)))),
    ];
    for (case, report) in cases {
        let (code, printed) = run_coverage_thresholds_step(Some(&report));
        assert_eq!(
            code,
            Some(1),
            "{case}: a report tracking 0 lines or 0 functions MUST fail the step (test-plan \
             §10 Coverage thresholds); printed:\n{printed}"
        );
        assert!(
            printed.contains("::error::lcov.info tracks"),
            "{case}: the step MUST say the report tracks nothing; printed:\n{printed}"
        );
        assert!(
            !printed.contains("100.0"),
            "{case}: no percentage may read 100 over a zero total; printed:\n{printed}"
        );
    }
}

#[test]
fn coverage_thresholds_step_passes_a_report_over_both_thresholds() {
    let report = lcov_report(Some((90, 100)), Some((9, 10)));
    let (code, printed) = run_coverage_thresholds_step(Some(&report));
    assert_eq!(code, Some(0), "printed:\n{printed}");
    for line in [
        "Line:     90/100 = 90.0% (threshold 75%)",
        "Function: 9/10 = 90.0% (threshold 85%)",
    ] {
        assert!(
            printed.contains(line),
            "the step MUST print `{line}`; printed:\n{printed}"
        );
    }
}

#[test]
fn coverage_thresholds_step_fails_a_report_under_either_threshold() {
    let cases = [
        (
            lcov_report(Some((70, 100)), Some((9, 10))),
            "::error::line coverage 70.0% < 75%",
        ),
        (
            lcov_report(Some((90, 100)), Some((8, 10))),
            "::error::function coverage 80.0% < 85%",
        ),
    ];
    for (report, error) in cases {
        let (code, printed) = run_coverage_thresholds_step(Some(&report));
        assert_eq!(code, Some(1), "printed:\n{printed}");
        assert!(
            printed.contains(error),
            "the step MUST print `{error}`; printed:\n{printed}"
        );
    }
}

#[test]
fn coverage_thresholds_step_fails_without_a_report() {
    let (code, printed) = run_coverage_thresholds_step(None);
    assert_eq!(code, Some(1), "printed:\n{printed}");
    assert!(
        printed.contains("::error::lcov.info missing"),
        "printed:\n{printed}"
    );
}

// The pinned toolchain writes no branch count, so the job states and reads
// the two thresholds it can measure.
#[test]
fn coverage_job_reads_no_branch_count() {
    let coverage = workflow_job_block(&read_workflow(), "coverage");
    for token in ["BRF", "BRH", "branch_", "Branch:"] {
        assert!(
            !coverage.contains(token),
            "the coverage job MUST hold no `{token}`: its report carries a zero branch total \
             on every run (test-plan §10 Coverage thresholds)"
        );
    }
    let name = coverage
        .lines()
        .find_map(|line| line.strip_prefix("    name: "))
        .expect("the coverage job MUST carry a `name:`");
    assert!(
        !name.to_ascii_lowercase().contains("branch"),
        "the coverage job's name MUST state no branch threshold; name: {name}"
    );
}

#[test]
fn ci_workflow_nextest_runs_fail_on_an_empty_selection() {
    let content = read_workflow();
    let runs: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && !line.starts_with("- name:"))
        .filter(|line| line.contains("cargo nextest run"))
        .collect();
    assert!(
        !runs.is_empty(),
        "ci.yml MUST run `cargo nextest run` in at least one step (sanity check on the line \
         pattern)"
    );
    for line in &runs {
        assert!(
            line.contains("--no-tests=fail"),
            "every `cargo nextest run` of ci.yml MUST spell `--no-tests=fail`, so a run that \
             selects no test fails its step (test-plan §9 Pipeline structure); line: {line}"
        );
    }
    for (idx, line) in content.lines().enumerate() {
        assert_eq!(
            line.matches("no-tests=").count(),
            line.matches("no-tests=fail").count(),
            "ci.yml:{}: the only `no-tests` value the workflow may name is `fail`; line: {line}",
            idx + 1
        );
    }
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
        ("ci.yml", 6),
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
