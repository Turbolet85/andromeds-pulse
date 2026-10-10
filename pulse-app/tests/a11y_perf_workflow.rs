//! Chunk #54 workflow self-lint test. Asserts `.github/workflows/ci.yml`
//! post-extension preserves SHA-pin discipline + harden-runner first-step +
//! workflow-level `permissions: contents: read` + new a11y/perf gate steps.
//! Mirrors chunk #53 `pulse-app/tests/distribution_manifests.rs` shape —
//! pure file-read + substring/pattern assertion; no Tauri runtime; no
//! network.

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

#[test]
fn ci_workflow_invokes_xtask_test_a11y() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask test:a11y"),
        "ci.yml MUST invoke `cargo xtask test:a11y` per chunk #54 a11y CI \
         gate activation"
    );
}

#[test]
fn ci_workflow_invokes_xtask_perf_slo_load() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask perf:slo-load"),
        "ci.yml MUST invoke `cargo xtask perf:slo-load` per chunk #54 perf \
         SLO gate activation"
    );
}

// The lines of one job, from its `  {name}:` header to the next job header.
// Line-anchored so a CRLF checkout reads the same as an LF one.
fn job_block(content: &str, name: &str) -> String {
    let header = format!("  {name}:");
    let lines: Vec<&str> = content.lines().collect();
    let start = lines
        .iter()
        .position(|l| *l == header)
        .unwrap_or_else(|| panic!("ci.yml MUST declare a `{name}` job"));
    lines[start + 1..]
        .iter()
        .take_while(|l| !(l.starts_with("  ") && !l.starts_with("   ") && l.ends_with(':')))
        .copied()
        .collect::<Vec<&str>>()
        .join("\n")
}

#[test]
fn ci_workflow_builds_ui_before_a11y_run() {
    let block = job_block(&read_workflow(), "a11y");
    let build_idx = block
        .find("npm run build --prefix pulse-app/ui")
        .or_else(|| block.find("npm --prefix pulse-app/ui run build"))
        .expect("the a11y job MUST invoke npm run build before the a11y audit (Vite dist needed by Lighthouse/Playwright)");
    let a11y_idx = block
        .find("cargo xtask test:a11y")
        .expect("the a11y job MUST invoke cargo xtask test:a11y");
    assert!(
        build_idx < a11y_idx,
        "npm build step MUST appear before cargo xtask test:a11y inside the a11y job"
    );
}

#[test]
fn ci_workflow_uploads_a11y_violations_artifact() {
    let content = read_workflow();
    assert!(
        content.contains("a11y-violations-"),
        "ci.yml MUST upload `a11y-violations-${{ runner.os }}` artifact per \
         chunk #54 (per-PR regression baseline + current run + contrast \
         report)"
    );
}

#[test]
fn ci_workflow_uploads_playwright_a11y_report_artifact() {
    let content = read_workflow();
    assert!(
        content.contains("playwright-a11y-report-"),
        "ci.yml MUST upload `playwright-a11y-report-${{ runner.os }}` \
         artifact per chunk #54 (Playwright + axe-core results)"
    );
}

const A11Y_BASELINE: &str = "pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json";

// The a11y job downloads no baseline: the comparison reads this file.
#[test]
fn a11y_regression_baseline_is_committed_in_the_tree() {
    let path = project_root().join(A11Y_BASELINE);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {} failed: {e}", path.display()));
    let baseline: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{A11Y_BASELINE} MUST parse as JSON: {e}"));
    assert!(
        baseline
            .get("per_surface")
            .is_some_and(serde_json::Value::is_object),
        "{A11Y_BASELINE} MUST hold a `per_surface` object, the tuples the regression \
         detector compares against (a11y-plan §3 Harness wiring & conventions)"
    );
}

#[test]
fn a11y_regression_detector_fails_when_its_baseline_is_absent() {
    let scratch = tempfile::tempdir().expect("create a scratch dir");
    let current = scratch.path().join("a11y-violations-summary.json");
    std::fs::write(&current, r#"{"per_surface":{}}"#).expect("write the current summary");
    let absent = scratch.path().join("absent-baseline.json");
    let output = std::process::Command::new("node")
        .arg("tests-a11y/regression-detector.mjs")
        .arg("--baseline")
        .arg(&absent)
        .arg("--current")
        .arg(&current)
        .current_dir(project_root().join("pulse-app/ui"))
        .output()
        .expect("`node` MUST be on PATH: the a11y comparison is a node script");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("baseline not found"),
        "the detector MUST say its baseline was not found; stderr:\n{stderr}"
    );
    assert!(
        !output.status.success(),
        "the detector MUST fail, not pass, when its baseline is absent: with no tuple in the \
         current summary an empty baseline compares clean (a11y-plan §3 Harness wiring & \
         conventions); status: {}; stderr:\n{stderr}",
        output.status
    );
}

#[test]
fn ci_workflow_invokes_xtask_verify_capability_matrix() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask verify:capability-matrix"),
        "ci.yml MUST invoke `cargo xtask verify:capability-matrix` per chunk \
         #99 (P-001..P-060 capability scenario mapping is a tag-gate \
         invariant)"
    );
}

#[test]
fn ci_workflow_invokes_xtask_capability_widening_check() {
    let content = read_workflow();
    assert!(
        content.contains("cargo xtask capability-widening-check"),
        "ci.yml MUST invoke `cargo xtask capability-widening-check` per \
         chunk #99 CI wiring of the chunk #77 NEVER-widen static analysis \
         (security plan §API Anti-Patterns rows 6-7)"
    );
}

#[test]
fn ci_workflow_uses_sha_pin_discipline_unchanged() {
    let content = read_workflow();
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("uses:") {
            continue;
        }
        for forbidden_pattern in [
            "@v1\n", "@v2\n", "@v3\n", "@v4\n", "@v5\n", "@main", "@master", "@latest",
        ] {
            assert!(
                !trimmed.contains(forbidden_pattern),
                "ci.yml `uses:` line MUST NOT use floating tag pattern \
                 `{forbidden_pattern}` per security plan §Supply Chain + CI \
                 (line: `{trimmed}`)"
            );
        }
        let after_at = trimmed.split_once('@').map(|(_, rest)| rest);
        let sha_part = after_at
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap_or("");
        assert!(
            sha_part.len() >= 40 && sha_part.chars().take(40).all(|c| c.is_ascii_hexdigit()),
            "ci.yml `uses:` line MUST be pinned to a 40-char SHA per security \
             plan §Supply Chain + CI (line: `{trimmed}`)"
        );
    }
}

#[test]
fn ci_workflow_preserves_workflow_level_contents_read_permission() {
    let content = read_workflow();
    let intro: String = content.lines().take(40).collect::<Vec<&str>>().join("\n");
    assert!(
        intro.contains("permissions:") && intro.contains("contents: read"),
        "ci.yml MUST preserve workflow-level `permissions: contents: read` \
         per security plan §Secret Management (no widening to contents: \
         write at workflow level)"
    );
}
