//! Staged-artifacts gate — the committed copy is the subject, never the
//! worktree. Any default-features test run rewrites the generated
//! `pulse-app/ui/src/bindings/index.ts` to its no-mcp shape, and the wrap's
//! own light gate is such a run, so the worktree can be correct while the
//! git INDEX carries a clobbered copy (measured at commit 70344d5). The gate
//! therefore reads `git show :<path>` for the bindings and for every
//! `pulse-app/capabilities/*.json`, diffing the former against
//! `EXPECTED_PROCEDURES` and the latter against the `EXPECTED_GRANTS` pin
//! (semantic surface only — prose `description` edits must not red a
//! security gate, while a revoked or added grant/window must, in BOTH
//! directions). Formalized contract per the `check:npm-supply-chain`
//! registry shape (one pretty-JSON verdict on stdout; arms `staged-clean` /
//! `staged-drift` / `cannot-evaluate`; exit 0 / 1 / 2). Also invoked from
//! `capability_drift`, whose gate-list slot runs LAST everywhere, so the
//! staged assertion cannot be skipped (security-plan §API Security;
//! test-plan §3).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::parse_bindings;

pub(crate) const BINDINGS_SUBJECT: &str = "pulse-app/ui/src/bindings/index.ts";
pub(crate) const CAPABILITIES_PREFIX: &str = "pulse-app/capabilities/";

pub(crate) struct ExpectedCapability {
    pub(crate) file: &'static str,
    pub(crate) identifier: &'static str,
    pub(crate) windows: &'static [&'static str],
    pub(crate) permissions: &'static [&'static str],
}

// The semantic surface of each capability file. A legitimate grant change
// updates this pin in the same commit (the EXPECTED_PROCEDURES discipline);
// `description` / `$schema` / `local` are deliberately outside it.
pub(crate) const EXPECTED_GRANTS: &[ExpectedCapability] = &[
    ExpectedCapability {
        file: "clipboard.json",
        identifier: "clipboard",
        windows: &["compact-widget", "main", "report"],
        permissions: &["clipboard-manager:allow-write-text"],
    },
    ExpectedCapability {
        file: "default.json",
        identifier: "default",
        windows: &["compact-widget", "main", "findings", "report"],
        permissions: &[
            "core:default",
            "core:window:allow-start-dragging",
            "core:window:allow-minimize",
            "core:window:allow-toggle-maximize",
            "core:window:allow-close",
            "core:window:allow-show",
            "core:window:allow-set-focus",
            "core:window:allow-hide",
            "core:window:allow-set-position",
            "core:window:allow-set-size",
            "updater:default",
        ],
    },
    ExpectedCapability {
        file: "notification.json",
        identifier: "notification",
        windows: &["compact-widget", "main"],
        permissions: &[
            "notification:allow-notify",
            "notification:allow-show",
            "notification:allow-is-permission-granted",
            "notification:allow-request-permission",
        ],
    },
    ExpectedCapability {
        file: "plugin-fs.json",
        identifier: "plugin-fs",
        windows: &[],
        permissions: &[],
    },
    ExpectedCapability {
        file: "tray.json",
        identifier: "tray",
        windows: &["compact-widget", "main"],
        permissions: &[],
    },
    ExpectedCapability {
        file: "updater.json",
        identifier: "updater",
        windows: &[],
        permissions: &["updater:default"],
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SubjectContent {
    Present(String),
    AbsentFromIndex,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct StagedFindings {
    pub(crate) bindings_missing: Vec<String>,
    pub(crate) bindings_extra: Vec<String>,
    pub(crate) bindings_issue: Option<String>,
    pub(crate) grant_findings: Vec<String>,
    pub(crate) unpinned_capability_files: Vec<String>,
}

impl StagedFindings {
    pub(crate) fn is_clean(&self) -> bool {
        self.bindings_missing.is_empty()
            && self.bindings_extra.is_empty()
            && self.bindings_issue.is_none()
            && self.grant_findings.is_empty()
            && self.unpinned_capability_files.is_empty()
    }
}

pub(crate) struct StagedReport {
    pub(crate) arm: &'static str,
    pub(crate) exit: u8,
    pub(crate) payload: Value,
}

fn str_set(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|s| s.to_string()).collect()
}

fn json_str_set(value: &Value, key: &str) -> BTreeSet<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn classify(
    bindings: &SubjectContent,
    capabilities: &[(&'static ExpectedCapability, SubjectContent)],
    unpinned_capability_files: &[String],
) -> StagedFindings {
    let mut findings = StagedFindings {
        unpinned_capability_files: unpinned_capability_files.to_vec(),
        ..StagedFindings::default()
    };

    match bindings {
        SubjectContent::AbsentFromIndex => {
            findings.bindings_issue = Some(format!(
                "{BINDINGS_SUBJECT} is not in the git index (staged deletion?)"
            ));
        }
        SubjectContent::Present(content) => match parse_bindings(content) {
            Ok(discovered) => {
                let expected: BTreeSet<String> = crate::EXPECTED_PROCEDURES
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                findings.bindings_missing = expected.difference(&discovered).cloned().collect();
                findings.bindings_extra = discovered.difference(&expected).cloned().collect();
            }
            Err(e) => {
                findings.bindings_issue = Some(format!("staged bindings unparseable: {e:#}"));
            }
        },
    }

    for (pin, content) in capabilities {
        let file = format!("{CAPABILITIES_PREFIX}{}", pin.file);
        match content {
            SubjectContent::AbsentFromIndex => {
                findings
                    .grant_findings
                    .push(format!("{file}: not in the git index (staged deletion?)"));
            }
            SubjectContent::Present(raw) => {
                let parsed: Value = match serde_json::from_str(raw) {
                    Ok(v) => v,
                    Err(e) => {
                        findings
                            .grant_findings
                            .push(format!("{file}: staged copy unparseable: {e}"));
                        continue;
                    }
                };
                let identifier = parsed
                    .get("identifier")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                if identifier != pin.identifier {
                    findings.grant_findings.push(format!(
                        "{file}: staged identifier `{identifier}` does not match pinned `{}`",
                        pin.identifier
                    ));
                }
                let pinned_windows = str_set(pin.windows);
                let staged_windows = json_str_set(&parsed, "windows");
                for revoked in pinned_windows.difference(&staged_windows) {
                    findings.grant_findings.push(format!(
                        "{file}: staged copy REVOKES pinned window `{revoked}`"
                    ));
                }
                for added in staged_windows.difference(&pinned_windows) {
                    findings.grant_findings.push(format!(
                        "{file}: staged copy ADDS unpinned window `{added}`"
                    ));
                }
                let pinned_perms = str_set(pin.permissions);
                let staged_perms = json_str_set(&parsed, "permissions");
                for revoked in pinned_perms.difference(&staged_perms) {
                    findings.grant_findings.push(format!(
                        "{file}: staged copy REVOKES pinned permission `{revoked}`"
                    ));
                }
                for added in staged_perms.difference(&pinned_perms) {
                    findings.grant_findings.push(format!(
                        "{file}: staged copy ADDS unpinned permission `{added}`"
                    ));
                }
            }
        }
    }

    findings
}

fn report_for(findings: &StagedFindings) -> StagedReport {
    let (arm, verdict, exit) = if findings.is_clean() {
        ("staged-clean", "green", 0u8)
    } else {
        ("staged-drift", "red", 1)
    };
    let payload = json!({
        "gate": "staged-artifacts",
        "arm": arm,
        "verdict": verdict,
        "bindings": {
            "subject": BINDINGS_SUBJECT,
            "missing": findings.bindings_missing,
            "extra": findings.bindings_extra,
            "issue": findings.bindings_issue,
        },
        "grants": {
            "files_checked": EXPECTED_GRANTS.len(),
            "findings": findings.grant_findings,
            "unpinned_files": findings.unpinned_capability_files,
        },
    });
    StagedReport { arm, exit, payload }
}

fn cannot_evaluate(detail: String) -> StagedReport {
    StagedReport {
        arm: "cannot-evaluate",
        exit: 2,
        payload: json!({
            "gate": "staged-artifacts",
            "arm": "cannot-evaluate",
            "verdict": "cannot-evaluate",
            "detail": detail,
        }),
    }
}

async fn git_output(root: &Path, args: &[&str]) -> std::io::Result<std::process::Output> {
    tokio::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .await
}

async fn read_subject(
    root: &Path,
    path: &str,
    staged_paths: &BTreeSet<String>,
) -> Result<SubjectContent, String> {
    if !staged_paths.contains(path) {
        return Ok(SubjectContent::AbsentFromIndex);
    }
    match git_output(root, &["show", &format!(":{path}")]).await {
        Ok(out) if out.status.success() => Ok(SubjectContent::Present(
            String::from_utf8_lossy(&out.stdout).into_owned(),
        )),
        Ok(out) => Err(format!(
            "git show :{path} failed on an index that lists it: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )),
        Err(e) => Err(format!("git could not be spawned for `show :{path}`: {e}")),
    }
}

pub(crate) async fn evaluate_at(root: &Path) -> StagedReport {
    let ls = match git_output(root, &["ls-files", "--stage"]).await {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).into_owned(),
        Ok(out) => {
            return cannot_evaluate(format!(
                "git ls-files --stage failed (not a git repository?): {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        Err(e) => return cannot_evaluate(format!("git could not be spawned: {e}")),
    };
    let staged_paths: BTreeSet<String> = ls
        .lines()
        .filter_map(|line| line.split('\t').nth(1))
        .map(str::to_string)
        .collect();

    let unpinned: Vec<String> = staged_paths
        .iter()
        .filter(|p| {
            p.starts_with(CAPABILITIES_PREFIX)
                && p.ends_with(".json")
                && !EXPECTED_GRANTS
                    .iter()
                    .any(|cap| format!("{CAPABILITIES_PREFIX}{}", cap.file) == **p)
        })
        .cloned()
        .collect();

    let bindings = match read_subject(root, BINDINGS_SUBJECT, &staged_paths).await {
        Ok(content) => content,
        Err(detail) => return cannot_evaluate(detail),
    };
    let mut capabilities: Vec<(&'static ExpectedCapability, SubjectContent)> = Vec::new();
    for cap in EXPECTED_GRANTS {
        let path = format!("{CAPABILITIES_PREFIX}{}", cap.file);
        match read_subject(root, &path, &staged_paths).await {
            Ok(content) => capabilities.push((cap, content)),
            Err(detail) => return cannot_evaluate(detail),
        }
    }

    report_for(&classify(&bindings, &capabilities, &unpinned))
}

pub(crate) fn emit(root: &Path, report: &StagedReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&report.payload)?);
    let report_dir = root.join("target").join("staged-artifacts");
    fs::create_dir_all(&report_dir).context("create staged-artifacts report dir")?;
    fs::write(
        report_dir.join("report.json"),
        serde_json::to_string_pretty(&report.payload)?,
    )
    .context("write staged-artifacts report")?;
    eprintln!(
        "check:staged-artifacts: {} (exit {})",
        report.arm, report.exit
    );
    for key in ["missing", "extra"] {
        if let Some(items) = report
            .payload
            .pointer(&format!("/bindings/{key}"))
            .and_then(Value::as_array)
        {
            for item in items {
                if let Some(name) = item.as_str() {
                    eprintln!("  ✗ bindings {key}: {name}");
                }
            }
        }
    }
    if let Some(issue) = report
        .payload
        .pointer("/bindings/issue")
        .and_then(Value::as_str)
    {
        eprintln!("  ✗ {issue}");
    }
    for key in ["findings", "unpinned_files"] {
        if let Some(items) = report
            .payload
            .pointer(&format!("/grants/{key}"))
            .and_then(Value::as_array)
        {
            for item in items {
                if let Some(name) = item.as_str() {
                    eprintln!("  ✗ {name}");
                }
            }
        }
    }
    if let Some(detail) = report.payload.get("detail").and_then(Value::as_str) {
        eprintln!("  ✗ {detail}");
    }
    Ok(())
}

fn workspace_root() -> Result<PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf())
}

pub(crate) async fn run() -> Result<ExitCode> {
    let root = workspace_root()?;
    let report = evaluate_at(&root).await;
    emit(&root, &report)?;
    Ok(ExitCode::from(report.exit))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn bindings_content_for(procs: &[&str]) -> String {
        let mut routers: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
        for p in procs {
            let (router, method) = match p.rsplit_once('.') {
                Some((r, m)) => (r.to_string(), m.to_string()),
                None => (String::new(), p.to_string()),
            };
            routers
                .entry(router)
                .or_default()
                .insert(method, Vec::new());
        }
        let body = routers
            .iter()
            .map(|(router, methods)| {
                format!(
                    "'{router}':'{}'",
                    serde_json::to_string(methods).expect("methods json")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("// test fixture\nconst ARGS_MAP = {{{body}}};\nexport const X = 1;\n")
    }

    fn capability_json_for(cap: &ExpectedCapability) -> String {
        serde_json::to_string_pretty(&json!({
            "$schema": "https://schema.tauri.app/config/2",
            "identifier": cap.identifier,
            "description": "fixture",
            "local": true,
            "windows": cap.windows,
            "permissions": cap.permissions,
        }))
        .expect("capability json")
    }

    fn clean_capabilities() -> Vec<(&'static ExpectedCapability, SubjectContent)> {
        EXPECTED_GRANTS
            .iter()
            .map(|cap| (cap, SubjectContent::Present(capability_json_for(cap))))
            .collect()
    }

    fn clean_bindings() -> SubjectContent {
        SubjectContent::Present(bindings_content_for(crate::EXPECTED_PROCEDURES))
    }

    #[test]
    fn bindings_fixture_roundtrips_through_parse_bindings() {
        let content = bindings_content_for(crate::EXPECTED_PROCEDURES);
        let discovered = parse_bindings(&content).expect("fixture parses");
        let expected: BTreeSet<String> = crate::EXPECTED_PROCEDURES
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(discovered, expected);
    }

    #[test]
    fn classify_clean_when_all_subjects_match_pins() {
        let findings = classify(&clean_bindings(), &clean_capabilities(), &[]);
        assert!(findings.is_clean(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn classify_reds_when_staged_bindings_lack_the_mcp_namespace() {
        let without_mcp: Vec<&str> = crate::EXPECTED_PROCEDURES
            .iter()
            .copied()
            .filter(|p| !p.starts_with("mcp."))
            .collect();
        let bindings = SubjectContent::Present(bindings_content_for(&without_mcp));
        let findings = classify(&bindings, &clean_capabilities(), &[]);
        assert!(!findings.is_clean());
        for name in ["mcp.start", "mcp.status", "mcp.stop"] {
            assert!(
                findings.bindings_missing.iter().any(|m| m == name),
                "missing set must name {name}: {:?}",
                findings.bindings_missing
            );
        }
        assert!(findings.bindings_extra.is_empty());
    }

    #[test]
    fn classify_reds_on_extra_staged_procedure() {
        let mut procs: Vec<&str> = crate::EXPECTED_PROCEDURES.to_vec();
        procs.push("rogue.procedure");
        let bindings = SubjectContent::Present(bindings_content_for(&procs));
        let findings = classify(&bindings, &clean_capabilities(), &[]);
        assert_eq!(findings.bindings_extra, vec!["rogue.procedure".to_string()]);
        assert!(findings.bindings_missing.is_empty());
    }

    #[test]
    fn classify_reds_on_unparseable_staged_bindings() {
        let bindings = SubjectContent::Present("export const nothing = 1;\n".to_string());
        let findings = classify(&bindings, &clean_capabilities(), &[]);
        let issue = findings.bindings_issue.expect("issue recorded");
        assert!(issue.contains("unparseable"), "{issue}");
    }

    #[test]
    fn classify_reds_on_staged_deletion_of_each_subject_kind() {
        let findings = classify(&SubjectContent::AbsentFromIndex, &clean_capabilities(), &[]);
        let issue = findings.bindings_issue.expect("bindings deletion recorded");
        assert!(issue.contains("not in the git index"), "{issue}");

        let mut caps = clean_capabilities();
        caps[1].1 = SubjectContent::AbsentFromIndex;
        let findings = classify(&clean_bindings(), &caps, &[]);
        assert!(
            findings
                .grant_findings
                .iter()
                .any(|f| f.contains("default.json") && f.contains("not in the git index")),
            "{:?}",
            findings.grant_findings
        );
    }

    #[test]
    fn classify_reds_on_revoked_grant_naming_file_and_permission() {
        let mut caps = clean_capabilities();
        let default_pin = EXPECTED_GRANTS
            .iter()
            .find(|c| c.file == "default.json")
            .expect("default pin");
        let revoked: Vec<&str> = default_pin
            .permissions
            .iter()
            .copied()
            .filter(|p| *p != "core:window:allow-close")
            .collect();
        let doc = serde_json::to_string(&json!({
            "identifier": default_pin.identifier,
            "windows": default_pin.windows,
            "permissions": revoked,
        }))
        .expect("json");
        caps[1].1 = SubjectContent::Present(doc);
        let findings = classify(&clean_bindings(), &caps, &[]);
        assert!(
            findings.grant_findings.iter().any(|f| {
                f.contains("default.json")
                    && f.contains("REVOKES")
                    && f.contains("core:window:allow-close")
            }),
            "{:?}",
            findings.grant_findings
        );
    }

    #[test]
    fn classify_reds_on_added_grant_and_added_window() {
        let mut caps = clean_capabilities();
        let tray_pin = EXPECTED_GRANTS
            .iter()
            .find(|c| c.file == "tray.json")
            .expect("tray pin");
        let doc = serde_json::to_string(&json!({
            "identifier": tray_pin.identifier,
            "windows": ["compact-widget", "main", "evil"],
            "permissions": ["shell:default"],
        }))
        .expect("json");
        caps[4].1 = SubjectContent::Present(doc);
        let findings = classify(&clean_bindings(), &caps, &[]);
        assert!(
            findings
                .grant_findings
                .iter()
                .any(|f| f.contains("tray.json")
                    && f.contains("ADDS")
                    && f.contains("shell:default")),
            "{:?}",
            findings.grant_findings
        );
        assert!(
            findings
                .grant_findings
                .iter()
                .any(|f| f.contains("tray.json") && f.contains("ADDS") && f.contains("`evil`")),
            "{:?}",
            findings.grant_findings
        );
    }

    #[test]
    fn classify_reds_on_identifier_mismatch() {
        let mut caps = clean_capabilities();
        let updater_pin = EXPECTED_GRANTS
            .iter()
            .find(|c| c.file == "updater.json")
            .expect("updater pin");
        let doc = serde_json::to_string(&json!({
            "identifier": "not-updater",
            "windows": updater_pin.windows,
            "permissions": updater_pin.permissions,
        }))
        .expect("json");
        caps[5].1 = SubjectContent::Present(doc);
        let findings = classify(&clean_bindings(), &caps, &[]);
        assert!(
            findings
                .grant_findings
                .iter()
                .any(|f| f.contains("updater.json") && f.contains("identifier")),
            "{:?}",
            findings.grant_findings
        );
    }

    #[test]
    fn classify_reds_on_unpinned_capability_file() {
        let unpinned = vec![format!("{CAPABILITIES_PREFIX}evil.json")];
        let findings = classify(&clean_bindings(), &clean_capabilities(), &unpinned);
        assert_eq!(findings.unpinned_capability_files, unpinned);
        assert!(!findings.is_clean());
    }

    #[test]
    fn expected_grants_pin_covers_exactly_the_six_capability_files() {
        let files: Vec<&str> = EXPECTED_GRANTS.iter().map(|c| c.file).collect();
        assert_eq!(
            files,
            vec![
                "clipboard.json",
                "default.json",
                "notification.json",
                "plugin-fs.json",
                "tray.json",
                "updater.json"
            ]
        );
    }

    #[test]
    fn expected_grants_pin_holds_the_escape_and_window_affordance_grants_granted() {
        let default_pin = EXPECTED_GRANTS
            .iter()
            .find(|c| c.file == "default.json")
            .expect("default pin");
        for grant in [
            "core:window:allow-close",
            "core:window:allow-start-dragging",
            "core:window:allow-minimize",
            "core:window:allow-toggle-maximize",
            "core:window:allow-show",
            "core:window:allow-set-focus",
            "core:window:allow-hide",
            "core:window:allow-set-position",
            "core:window:allow-set-size",
        ] {
            assert!(
                default_pin.permissions.contains(&grant),
                "the pin must hold {grant} in its GRANTED state"
            );
        }
    }

    #[test]
    fn report_maps_clean_and_drift_to_the_contract_arms() {
        let clean = report_for(&StagedFindings::default());
        assert_eq!(clean.arm, "staged-clean");
        assert_eq!(clean.exit, 0);
        let drift = report_for(&StagedFindings {
            bindings_missing: vec!["mcp.start".to_string()],
            ..StagedFindings::default()
        });
        assert_eq!(drift.arm, "staged-drift");
        assert_eq!(drift.exit, 1);
        assert_eq!(drift.payload["verdict"], "red");
    }

    // Fixture repos are runtime-built TempDirs (git init + add; no commit, so
    // no user identity config is needed) — the real tree is never mutated.

    async fn git_in(dir: &Path, args: &[&str]) {
        let out = tokio::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .await
            .expect("spawn git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    fn write_fixture(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        fs::write(path, content).expect("write fixture");
    }

    async fn fixture_repo(bindings: &str) -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let root = dir.path();
        git_in(root, &["init", "-q"]).await;
        write_fixture(root, BINDINGS_SUBJECT, bindings);
        for cap in EXPECTED_GRANTS {
            write_fixture(
                root,
                &format!("{CAPABILITIES_PREFIX}{}", cap.file),
                &capability_json_for(cap),
            );
        }
        git_in(root, &["add", "-A"]).await;
        dir
    }

    #[tokio::test]
    async fn fixture_repo_clean_reports_staged_clean() {
        let dir = fixture_repo(&bindings_content_for(crate::EXPECTED_PROCEDURES)).await;
        let report = evaluate_at(dir.path()).await;
        assert_eq!(report.arm, "staged-clean", "{}", report.payload);
        assert_eq!(report.exit, 0);
    }

    #[tokio::test]
    async fn fixture_repo_with_staged_no_mcp_bindings_reports_staged_drift() {
        let without_mcp: Vec<&str> = crate::EXPECTED_PROCEDURES
            .iter()
            .copied()
            .filter(|p| !p.starts_with("mcp."))
            .collect();
        let dir = fixture_repo(&bindings_content_for(&without_mcp)).await;
        let report = evaluate_at(dir.path()).await;
        assert_eq!(report.arm, "staged-drift", "{}", report.payload);
        assert_eq!(report.exit, 1);
        let missing = report.payload["bindings"]["missing"]
            .as_array()
            .expect("missing array")
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>();
        assert!(missing.contains(&"mcp.start"), "{missing:?}");
    }

    #[tokio::test]
    async fn fixture_repo_with_staged_revoked_grant_reports_staged_drift() {
        let dir = fixture_repo(&bindings_content_for(crate::EXPECTED_PROCEDURES)).await;
        let root = dir.path();
        let default_pin = EXPECTED_GRANTS
            .iter()
            .find(|c| c.file == "default.json")
            .expect("default pin");
        let revoked: Vec<&str> = default_pin
            .permissions
            .iter()
            .copied()
            .filter(|p| *p != "core:window:allow-close")
            .collect();
        let doc = serde_json::to_string_pretty(&json!({
            "identifier": default_pin.identifier,
            "windows": default_pin.windows,
            "permissions": revoked,
        }))
        .expect("json");
        write_fixture(root, "pulse-app/capabilities/default.json", &doc);
        git_in(root, &["add", "-A"]).await;
        let report = evaluate_at(root).await;
        assert_eq!(report.arm, "staged-drift", "{}", report.payload);
        let findings = report.payload["grants"]["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>();
        assert!(
            findings
                .iter()
                .any(|f| f.contains("core:window:allow-close") && f.contains("REVOKES")),
            "{findings:?}"
        );
    }

    #[tokio::test]
    async fn fixture_repo_reads_the_index_not_the_worktree() {
        // Sixth-occurrence shape: index correct, worktree clobbered after the
        // add — a worktree-reading check would false-RED here.
        let dir = fixture_repo(&bindings_content_for(crate::EXPECTED_PROCEDURES)).await;
        let root = dir.path();
        let without_mcp: Vec<&str> = crate::EXPECTED_PROCEDURES
            .iter()
            .copied()
            .filter(|p| !p.starts_with("mcp."))
            .collect();
        write_fixture(root, BINDINGS_SUBJECT, &bindings_content_for(&without_mcp));
        let report = evaluate_at(root).await;
        assert_eq!(report.arm, "staged-clean", "{}", report.payload);
    }

    #[tokio::test]
    async fn fixture_repo_reds_when_only_the_index_carries_the_clobber() {
        // Fifth-occurrence shape: the worktree was repaired after staging, so
        // the commit would still ship the clobbered copy.
        let without_mcp: Vec<&str> = crate::EXPECTED_PROCEDURES
            .iter()
            .copied()
            .filter(|p| !p.starts_with("mcp."))
            .collect();
        let dir = fixture_repo(&bindings_content_for(&without_mcp)).await;
        let root = dir.path();
        write_fixture(
            root,
            BINDINGS_SUBJECT,
            &bindings_content_for(crate::EXPECTED_PROCEDURES),
        );
        let report = evaluate_at(root).await;
        assert_eq!(report.arm, "staged-drift", "{}", report.payload);
    }

    #[tokio::test]
    async fn fixture_repo_with_unpinned_capability_file_reports_staged_drift() {
        let dir = fixture_repo(&bindings_content_for(crate::EXPECTED_PROCEDURES)).await;
        let root = dir.path();
        write_fixture(
            root,
            "pulse-app/capabilities/evil.json",
            "{\"identifier\":\"evil\",\"windows\":[],\"permissions\":[]}",
        );
        git_in(root, &["add", "-A"]).await;
        let report = evaluate_at(root).await;
        assert_eq!(report.arm, "staged-drift", "{}", report.payload);
        let unpinned = report.payload["grants"]["unpinned_files"]
            .as_array()
            .expect("unpinned array")
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>();
        assert_eq!(unpinned, vec!["pulse-app/capabilities/evil.json"]);
    }

    #[tokio::test]
    async fn non_repo_dir_reports_cannot_evaluate() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let report = evaluate_at(dir.path()).await;
        assert_eq!(report.arm, "cannot-evaluate", "{}", report.payload);
        assert_eq!(report.exit, 2);
    }
}
