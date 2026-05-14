//! Chunk #53 distribution-manifest validation tests. Lightweight smoke
//! checks asserting `.github/workflows/update-channels.yml` contains the
//! arch §Occupied Resources canonical bundle artifact names + the
//! brand-voice description copy + Scoop manifest required fields. Full
//! cross-repo push verification (actual Homebrew tap + Scoop bucket repo
//! updates) is operator-driven DEFERRED per /implement Phase 6 Open
//! Question 1 = Path A precedent mirroring chunk #52.
//!
//! Path resolution: `env!("CARGO_MANIFEST_DIR")` returns the `pulse-app/`
//! dir; the workflow file lives at project root `.github/workflows/`, so
//! tests resolve via `.parent()` traversal. Tests are pure file-read +
//! substring / pattern check; no Tauri runtime context, no network.

use std::path::PathBuf;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("pulse-app has parent (workspace root)")
        .to_path_buf()
}

fn read_workflow() -> String {
    let full_path = project_root().join(".github/workflows/update-channels.yml");
    std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("read {} failed: {e}", full_path.display()))
}

#[test]
fn update_channels_workflow_references_canonical_bundle_artifact_names() {
    // Per arch §Occupied Resources Bundle artifact names: the canonical
    // bundle filenames are the contract between chunk #52 release.yml (emitter)
    // + chunk #53 update-channels.yml (consumer). Naming drift here breaks
    // the chunk #51 smoke harness cross-validation and the Homebrew/Scoop
    // install paths simultaneously.
    let content = read_workflow();
    for canonical in [
        // Macos variants for Homebrew Formula
        r"andromeda-pulse_${VERSION}_x64.dmg",
        r"andromeda-pulse_${VERSION}_aarch64.dmg",
        // Linux variants for Homebrew Formula
        r"andromeda-pulse_${VERSION}_amd64.AppImage",
        // Windows variant for Scoop bucket
        r"andromeda-pulse_${VERSION}_x64-setup.msi",
    ] {
        assert!(
            content.contains(canonical),
            "update-channels.yml MUST reference canonical bundle artifact name \
             `{canonical}` per arch §Occupied Resources"
        );
    }
}

#[test]
fn update_channels_workflow_targets_both_distribution_channels() {
    // Per arch §Established Decisions [Distribution Channels] + §Inherited
    // Defaults: three-channel mandate (GitHub Releases primary + Homebrew +
    // Scoop). chunk #53 ships BOTH Homebrew tap update AND Scoop bucket
    // update — not just one. The two external repos are named in the
    // workflow per route §3 (turbolet85/homebrew-andromeda-pulse +
    // turbolet85/scoop-andromeda-pulse).
    let content = read_workflow();
    assert!(
        content.contains("update-homebrew"),
        "update-channels.yml MUST declare an update-homebrew job per arch \
         three-channel mandate"
    );
    assert!(
        content.contains("update-scoop"),
        "update-channels.yml MUST declare an update-scoop job per arch \
         three-channel mandate"
    );
    assert!(
        content.contains("HOMEBREW_TAP_PUSH_TOKEN"),
        "update-channels.yml MUST reference HOMEBREW_TAP_PUSH_TOKEN secret \
         per production-release-environment.md §DEFERRED scope Item 6"
    );
    assert!(
        content.contains("SCOOP_BUCKET_PUSH_TOKEN"),
        "update-channels.yml MUST reference SCOOP_BUCKET_PUSH_TOKEN secret \
         per production-release-environment.md §DEFERRED scope Item 6"
    );
}

#[test]
fn update_channels_workflow_homebrew_desc_under_80_chars() {
    // Homebrew Formula convention: `desc` string ≤ 80 chars, no period, no
    // leading article. Per design plan §Brand Identity: contemplative voice
    // (Observatory / Mission Control framing) — short + descriptive. The
    // canonical desc is "Local OpenTelemetry observability dashboard for
    // developers" (54 chars).
    let content = read_workflow();
    let desc_pattern = r#"desc "Local OpenTelemetry observability dashboard for developers""#;
    assert!(
        content.contains(desc_pattern),
        "update-channels.yml MUST contain the canonical Homebrew Formula \
         `desc` line per design plan §Brand Identity contemplative voice"
    );
    // Brand-voice anti-pattern check: no generic SaaS framing per design plan
    // §Anti-Patterns Universal Bans + §Self-Validation Protocol §5 Sameness
    // Test. The install-time entry point IS а brand touchpoint.
    for forbidden in [
        "powerful telemetry visualization platform",
        "modern observability for developers",
        "enterprise-grade observability",
    ] {
        assert!(
            !content.contains(forbidden),
            "update-channels.yml MUST NOT contain generic SaaS framing \
             phrase `{forbidden}` per design plan §Anti-Patterns Universal \
             Bans (Sameness Test)"
        );
    }
}

#[test]
fn update_channels_workflow_scoop_manifest_required_keys() {
    // Per Scoop spec (https://github.com/ScoopInstaller/Scoop/wiki/App-
    // Manifests): valid manifest requires `version`, `description`,
    // `homepage`, `license`, `architecture.64bit.{url,hash}`, `bin`. Verify
    // the inline heredoc template includes all 6 required keys.
    let content = read_workflow();
    for required_key in [
        r#""version": "${VERSION}""#,
        r#""description""#,
        r#""homepage""#,
        r#""license": "MIT""#,
        r#""architecture""#,
        r#""64bit""#,
        r#""url""#,
        r#""hash""#,
        r#""bin": "andromeda-pulse.exe""#,
    ] {
        assert!(
            content.contains(required_key),
            "update-channels.yml Scoop manifest template MUST contain \
             required key/value `{required_key}` per Scoop spec"
        );
    }
}

#[test]
fn update_channels_workflow_sha_pin_discipline() {
    // Per security plan §Supply Chain + CI: every `uses:` line MUST be
    // pinned by 40-char commit SHA + `# vX.Y.Z` provenance comment. Zero
    // floating-tag matches allowed. Mirrors chunk #52 release.yml's 11/11
    // SHA-pin discipline.
    let content = read_workflow();
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("uses:") {
            continue;
        }
        // Floating tag patterns to reject: @v1, @v2.0, @main, @master, @latest
        for forbidden_pattern in [
            "@v1\n", "@v2\n", "@v3\n", "@v4\n", "@v5\n", "@main", "@master", "@latest",
        ] {
            assert!(
                !trimmed.contains(forbidden_pattern),
                "update-channels.yml `uses:` line MUST NOT use floating tag \
                 pattern `{forbidden_pattern}` per security plan §Supply \
                 Chain + CI (line: `{trimmed}`)"
            );
        }
        // Positive check: the `uses:` value MUST contain `@` followed by а
        // 40-char hex SHA. Look for the `@<40-hex>` pattern в the line.
        let after_at = trimmed.split_once('@').map(|(_, rest)| rest);
        let sha_part = after_at
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap_or("");
        assert!(
            sha_part.len() >= 40 && sha_part.chars().take(40).all(|c| c.is_ascii_hexdigit()),
            "update-channels.yml `uses:` line MUST be pinned к а 40-char \
             SHA per security plan §Supply Chain + CI (line: `{trimmed}`)"
        );
    }
}

#[test]
fn update_channels_workflow_environment_scoping_on_secret_jobs() {
    // Per security plan §Secret Management + chunk #52 precedent: any job
    // accessing HOMEBREW_TAP_PUSH_TOKEN или SCOOP_BUCKET_PUSH_TOKEN MUST
    // declare `environment: production-release` to gate the job behind
    // manual approval. This invariant prevents PR-time CI from ever
    // touching trusted-publisher tokens.
    let content = read_workflow();
    // Count occurrences of the Environment binding string. Expected: 2
    // (one for update-homebrew job + one for update-scoop job).
    let env_count = content.matches("environment: production-release").count();
    assert!(
        env_count >= 2,
        "update-channels.yml MUST declare `environment: production-release` \
         on EACH job accessing Environment secrets (expected ≥2 occurrences \
         for the 2 jobs; found {env_count})"
    );
}

#[test]
fn update_channels_workflow_workflow_run_trigger_with_success_guard() {
    // Per arch §Infrastructure Patterns CI/CD approach + chunk #53 plan:
    // workflow_run trigger MUST guard via `conclusion == 'success'` to
    // skip channel updates когда the originating release.yml run failed.
    let content = read_workflow();
    assert!(
        content.contains("workflows: [\"release\"]"),
        "update-channels.yml MUST trigger on workflow_run от the `release` \
         workflow (chunk #52 release.yml `name:` field)"
    );
    assert!(
        content.contains("types: [completed]"),
        "update-channels.yml MUST trigger on `types: [completed]` к detect \
         release.yml completion"
    );
    assert!(
        content.contains("github.event.workflow_run.conclusion == 'success'"),
        "update-channels.yml MUST guard each job via \
         `if: github.event.workflow_run.conclusion == 'success'` к skip \
         channel updates на а failed release"
    );
}
