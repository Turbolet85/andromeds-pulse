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

#[test]
fn ci_workflow_builds_ui_before_a11y_run() {
    let content = read_workflow();
    let build_idx = content
        .find("npm run build --prefix pulse-app/ui")
        .or_else(|| content.find("npm --prefix pulse-app/ui run build"))
        .expect("ci.yml MUST invoke npm run build before a11y audit (Vite dist needed by Lighthouse/Playwright)");
    let a11y_idx = content
        .find("cargo xtask test:a11y")
        .expect("ci.yml MUST invoke cargo xtask test:a11y");
    assert!(
        build_idx < a11y_idx,
        "npm build step MUST appear before cargo xtask test:a11y in ci.yml"
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
