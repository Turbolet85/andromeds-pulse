//! npm supply-chain gate over `pulse-app/ui` — advisory / license / ban checks
//! against the project-owned policy at `pulse-app/ui/npm-policy.json`
//! (security-plan §Dependency Security: visible dispositions, ID-scoped
//! exceptions with provenance, never threshold suppression).
//!
//! License + dependency-class source is `package-lock.json` alone (906/909
//! entries carry a `license` field, measured 2026-08-30; the `dev` flags give
//! the runtime/dev class split). Never a node_modules walk — nested
//! test-fixture stubs pollute it, and the lockfile covers platform optional
//! packages that are not installed on this host.
//!
//! Verdict arms (each defined; exit code in parentheses):
//! - `clean` (0) — no findings, no exceptions consumed
//! - `green-with-dispositions` (0) — findings exist and every one is covered
//!   by an ID-scoped policy exception, listed in the output
//! - `findings-red` (1) — at least one advisory lacks an exception
//! - `policy-red` (1) — a license violation or banned package (no advisory arm red)
//! - `registry-unreachable` (2) — `npm audit` could not evaluate (network /
//!   npm error); NEVER a findings pass
//! - `missing-input` (2) — package-lock.json / npm-policy.json absent or unparseable

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use serde_json::{Value, json};

const POLICY_FILE: &str = "npm-policy.json";
const LOCKFILE: &str = "package-lock.json";

#[derive(Debug)]
pub(crate) struct Policy {
    advisory_exceptions: BTreeMap<String, String>, // GHSA id -> package
    runtime_licenses: BTreeSet<String>,
    dev_additional_licenses: BTreeSet<String>,
    license_exception_packages: BTreeSet<String>,
    banned: Vec<String>, // exact name, or prefix when ending in '/'
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Advisory {
    ghsa: String,
    package: String,
    severity: String,
    title: String,
}

pub(crate) struct LockPackage {
    name: String,
    version: Option<String>,
    dev: bool,
    license: Option<String>,
}

pub(crate) struct LicenseViolation {
    package: String,
    version: Option<String>,
    class: &'static str,
    license: Option<String>,
}

fn require_str(v: &Value, key: &str, what: &str) -> Result<String> {
    let s = v
        .get(key)
        .and_then(Value::as_str)
        .with_context(|| format!("{what}: missing or non-string `{key}`"))?;
    if s.trim().is_empty() {
        anyhow::bail!("{what}: `{key}` is empty — provenance fields are mandatory");
    }
    Ok(s.to_string())
}

pub(crate) fn parse_policy(v: &Value) -> Result<Policy> {
    let mut advisory_exceptions = BTreeMap::new();
    for (i, e) in v
        .get("advisory_exceptions")
        .and_then(Value::as_array)
        .context("policy: `advisory_exceptions` array missing")?
        .iter()
        .enumerate()
    {
        let what = format!("policy advisory_exceptions[{i}]");
        let ghsa = require_str(e, "ghsa", &what)?;
        let package = require_str(e, "package", &what)?;
        // Provenance is part of the contract, not decoration — an exception
        // without a reason/owner/closing condition is invalid, per the
        // carve-out discipline the cargo-side deny.toml follows.
        require_str(e, "reason", &what)?;
        require_str(e, "owner", &what)?;
        require_str(e, "closing_condition", &what)?;
        advisory_exceptions.insert(ghsa, package);
    }

    let allow = v
        .get("license_allow")
        .context("policy: `license_allow` missing")?;
    let read_set = |key: &str| -> Result<BTreeSet<String>> {
        Ok(allow
            .get(key)
            .and_then(Value::as_array)
            .with_context(|| format!("policy: `license_allow.{key}` array missing"))?
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect())
    };
    let runtime_licenses = read_set("runtime")?;
    let dev_additional_licenses = read_set("dev_additional")?;

    let mut license_exception_packages = BTreeSet::new();
    for (i, e) in v
        .get("license_exceptions")
        .and_then(Value::as_array)
        .context("policy: `license_exceptions` array missing")?
        .iter()
        .enumerate()
    {
        let what = format!("policy license_exceptions[{i}]");
        let package = require_str(e, "package", &what)?;
        require_str(e, "reason", &what)?;
        require_str(e, "owner", &what)?;
        require_str(e, "closing_condition", &what)?;
        license_exception_packages.insert(package);
    }

    let banned = v
        .get("banned_packages")
        .and_then(Value::as_array)
        .context("policy: `banned_packages` array missing")?
        .iter()
        .enumerate()
        .map(|(i, e)| require_str(e, "name", &format!("policy banned_packages[{i}]")))
        .collect::<Result<Vec<_>>>()?;

    Ok(Policy {
        advisory_exceptions,
        runtime_licenses,
        dev_additional_licenses,
        license_exception_packages,
        banned,
    })
}

pub(crate) fn lockfile_packages(lock: &Value) -> Result<Vec<LockPackage>> {
    let packages = lock
        .get("packages")
        .and_then(Value::as_object)
        .context("package-lock.json: `packages` map missing (lockfileVersion >= 2 required)")?;
    let mut out = Vec::new();
    for (path, meta) in packages {
        if path.is_empty() {
            continue; // the root project entry
        }
        let name = path
            .rsplit_once("node_modules/")
            .map(|(_, n)| n)
            .unwrap_or(path)
            .to_string();
        out.push(LockPackage {
            name,
            version: meta
                .get("version")
                .and_then(Value::as_str)
                .map(str::to_string),
            dev: meta.get("dev").and_then(Value::as_bool).unwrap_or(false),
            license: meta
                .get("license")
                .and_then(Value::as_str)
                .map(str::to_string),
        });
    }
    Ok(out)
}

pub(crate) fn check_licenses(
    pkgs: &[LockPackage],
    policy: &Policy,
) -> (Vec<LicenseViolation>, usize) {
    let mut violations = Vec::new();
    let mut excepted = 0usize;
    for p in pkgs {
        let class: &'static str = if p.dev { "dev" } else { "runtime" };
        let allowed = match &p.license {
            Some(l) => {
                policy.runtime_licenses.contains(l)
                    || (p.dev && policy.dev_additional_licenses.contains(l))
            }
            None => false,
        };
        if allowed {
            continue;
        }
        if policy.license_exception_packages.contains(&p.name) {
            excepted += 1;
            continue;
        }
        violations.push(LicenseViolation {
            package: p.name.clone(),
            version: p.version.clone(),
            class,
            license: p.license.clone(),
        });
    }
    (violations, excepted)
}

pub(crate) fn check_bans(pkgs: &[LockPackage], policy: &Policy) -> Vec<(String, String)> {
    let mut hits = Vec::new();
    for p in pkgs {
        for ban in &policy.banned {
            let hit = if let Some(prefix) = ban.strip_suffix('/') {
                // A trailing-slash entry bans a scoped family (`@radix-ui/`).
                p.name.starts_with(prefix) && p.name[prefix.len()..].starts_with('/')
                // bare-name prefix entries like `ariakit` are exact-only below
            } else {
                p.name == *ban
            };
            if hit {
                hits.push((p.name.clone(), ban.clone()));
            }
        }
    }
    hits
}

pub(crate) enum AuditOutcome {
    Report(Value),
    Infra(String),
}

pub(crate) fn classify_audit_stdout(stdout: &str) -> AuditOutcome {
    match serde_json::from_str::<Value>(stdout) {
        Ok(v) => {
            if let Some(err) = v.get("error") {
                let code = err.get("code").and_then(Value::as_str).unwrap_or("unknown");
                let summary = err
                    .get("summary")
                    .and_then(Value::as_str)
                    .unwrap_or("npm audit reported an error");
                AuditOutcome::Infra(format!("npm audit error [{code}]: {summary}"))
            } else if v.get("vulnerabilities").is_some() || v.get("metadata").is_some() {
                AuditOutcome::Report(v)
            } else {
                AuditOutcome::Infra(
                    "npm audit output parsed but carries neither `vulnerabilities` nor `metadata`"
                        .to_string(),
                )
            }
        }
        Err(e) => AuditOutcome::Infra(format!("npm audit stdout is not JSON: {e}")),
    }
}

pub(crate) fn extract_advisories(audit: &Value) -> Vec<Advisory> {
    let mut by_ghsa: BTreeMap<String, Advisory> = BTreeMap::new();
    let Some(vulns) = audit.get("vulnerabilities").and_then(Value::as_object) else {
        return Vec::new();
    };
    for v in vulns.values() {
        let Some(via) = v.get("via").and_then(Value::as_array) else {
            continue;
        };
        for entry in via {
            // String entries chain to another package's advisory; only object
            // entries carry an advisory of their own.
            let Some(obj) = entry.as_object() else {
                continue;
            };
            let url = obj.get("url").and_then(Value::as_str).unwrap_or("");
            let ghsa = url
                .rsplit('/')
                .next()
                .filter(|t| t.starts_with("GHSA-"))
                .map(str::to_string)
                .or_else(|| {
                    obj.get("source")
                        .and_then(Value::as_u64)
                        .map(|s| format!("NPM-{s}"))
                });
            let Some(ghsa) = ghsa else { continue };
            by_ghsa.entry(ghsa.clone()).or_insert_with(|| Advisory {
                ghsa,
                package: obj
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
                    .to_string(),
                severity: obj
                    .get("severity")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
                    .to_string(),
                title: obj
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
            });
        }
    }
    by_ghsa.into_values().collect()
}

pub(crate) fn split_advisories(
    advisories: Vec<Advisory>,
    policy: &Policy,
) -> (Vec<Advisory>, Vec<Advisory>) {
    advisories
        .into_iter()
        .partition(|a| !policy.advisory_exceptions.contains_key(&a.ghsa))
}

fn ui_dir() -> Result<PathBuf> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    Ok(workspace_root.join("pulse-app").join("ui"))
}

async fn run_npm_audit_json(cwd: &Path) -> Result<Option<String>> {
    let npm = if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    };
    let mut cmd = tokio::process::Command::new(npm);
    cmd.current_dir(cwd).arg("audit").arg("--json");
    match cmd.output().await {
        Ok(out) => Ok(Some(String::from_utf8_lossy(&out.stdout).into_owned())),
        Err(e) => {
            eprintln!("npm-supply-chain: failed to spawn npm audit: {e}");
            Ok(None)
        }
    }
}

pub async fn run_npm_gate() -> Result<ExitCode> {
    let ui = ui_dir()?;

    let read_json = |name: &str| -> Result<Value> {
        let path = ui.join(name);
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("missing input: {}", path.display()))?;
        serde_json::from_str(&raw).with_context(|| format!("unparseable JSON: {}", path.display()))
    };

    let policy_value = match read_json(POLICY_FILE) {
        Ok(v) => v,
        Err(e) => {
            print_verdict(&json!({
                "gate": "npm-supply-chain", "arm": "missing-input", "verdict": "cannot-evaluate",
                "detail": format!("{e:#}"),
            }));
            return Ok(ExitCode::from(2));
        }
    };
    let policy = match parse_policy(&policy_value) {
        Ok(p) => p,
        Err(e) => {
            print_verdict(&json!({
                "gate": "npm-supply-chain", "arm": "missing-input", "verdict": "cannot-evaluate",
                "detail": format!("policy invalid: {e:#}"),
            }));
            return Ok(ExitCode::from(2));
        }
    };
    let lock = match read_json(LOCKFILE) {
        Ok(v) => v,
        Err(e) => {
            print_verdict(&json!({
                "gate": "npm-supply-chain", "arm": "missing-input", "verdict": "cannot-evaluate",
                "detail": format!("{e:#}"),
            }));
            return Ok(ExitCode::from(2));
        }
    };
    let pkgs = lockfile_packages(&lock)?;

    let (license_violations, license_excepted) = check_licenses(&pkgs, &policy);
    let ban_hits = check_bans(&pkgs, &policy);

    let audit_stdout = run_npm_audit_json(&ui).await?;
    let outcome = match &audit_stdout {
        Some(s) => classify_audit_stdout(s),
        None => AuditOutcome::Infra("npm could not be spawned (is Node installed?)".to_string()),
    };
    let audit = match outcome {
        AuditOutcome::Report(v) => v,
        AuditOutcome::Infra(detail) => {
            print_verdict(&json!({
                "gate": "npm-supply-chain", "arm": "registry-unreachable", "verdict": "cannot-evaluate",
                "detail": detail,
            }));
            return Ok(ExitCode::from(2));
        }
    };
    let advisories = extract_advisories(&audit);
    let (unexcepted, excepted) = split_advisories(advisories, &policy);

    let runtime_count = pkgs.iter().filter(|p| !p.dev).count();
    let advisory_red = !unexcepted.is_empty();
    let policy_red = !license_violations.is_empty() || !ban_hits.is_empty();
    let dispositions = excepted.len() + license_excepted;
    let (arm, verdict, code) = if advisory_red {
        ("findings-red", "red", 1u8)
    } else if policy_red {
        ("policy-red", "red", 1)
    } else if dispositions > 0 {
        ("green-with-dispositions", "green", 0)
    } else {
        ("clean", "green", 0)
    };

    print_verdict(&json!({
        "gate": "npm-supply-chain",
        "arm": arm,
        "verdict": verdict,
        "advisory": {
            "distinct": unexcepted.len() + excepted.len(),
            "excepted": excepted.iter().map(|a| json!({
                "ghsa": a.ghsa, "package": a.package, "severity": a.severity,
            })).collect::<Vec<_>>(),
            "unexcepted": unexcepted.iter().map(|a| json!({
                "ghsa": a.ghsa, "package": a.package, "severity": a.severity, "title": a.title,
            })).collect::<Vec<_>>(),
        },
        "license": {
            "checked": pkgs.len(),
            "runtime_packages": runtime_count,
            "excepted": license_excepted,
            "violations": license_violations.iter().map(|v| json!({
                "package": v.package, "version": v.version, "class": v.class, "license": v.license,
            })).collect::<Vec<_>>(),
        },
        "bans": {
            "hits": ban_hits.iter().map(|(p, b)| json!({"package": p, "ban": b})).collect::<Vec<_>>(),
        },
    }));
    if advisory_red {
        eprintln!(
            "npm-supply-chain: {} advisory finding(s) lack a documented exception — a finding \
             with a stated safe forward upgrade is upgraded at the source, never excepted \
             (security-plan §Dependency Security); only a no-forward-fix finding takes an \
             ID-scoped entry in pulse-app/ui/npm-policy.json",
            unexcepted.len()
        );
    }
    Ok(ExitCode::from(code))
}

fn print_verdict(v: &Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy_json(with_exception: bool) -> Value {
        let exceptions = if with_exception {
            json!([{
                "ghsa": "GHSA-aaaa-bbbb-cccc", "package": "webdriver",
                "reason": "no forward fix", "owner": "test", "closing_condition": "upstream release",
            }])
        } else {
            json!([])
        };
        json!({
            "advisory_exceptions": exceptions,
            "license_allow": {
                "runtime": ["MIT", "Apache-2.0"],
                "dev_additional": ["LGPL-3.0-only"],
            },
            "license_exceptions": [{
                "package": "css-value",
                "reason": "no license upstream", "owner": "test", "closing_condition": "upstream publishes",
            }],
            "banned_packages": [
                {"name": "@radix-ui/", "reason": "second ARIA lib"},
                {"name": "ariakit", "reason": "second ARIA lib"},
            ],
        })
    }

    fn audit_json() -> Value {
        json!({
            "auditReportVersion": 2,
            "vulnerabilities": {
                "webdriver": {
                    "name": "webdriver", "severity": "high", "isDirect": false,
                    "via": [{
                        "source": 1099999, "name": "webdriver", "dependency": "webdriver",
                        "title": "test advisory", "url": "https://github.com/advisories/GHSA-aaaa-bbbb-cccc",
                        "severity": "high", "range": ">=8.0.0",
                    }],
                    "range": ">=8.0.0", "fixAvailable": false,
                },
                "webdriverio": {
                    "name": "webdriverio", "severity": "high", "isDirect": true,
                    "via": ["webdriver"],
                    "range": ">=8.0.0", "fixAvailable": false,
                },
            },
            "metadata": {"vulnerabilities": {"total": 2}},
        })
    }

    #[test]
    fn extract_advisories_dedupes_object_vias_and_skips_string_chains() {
        let advisories = extract_advisories(&audit_json());
        assert_eq!(advisories.len(), 1);
        assert_eq!(advisories[0].ghsa, "GHSA-aaaa-bbbb-cccc");
        assert_eq!(advisories[0].package, "webdriver");
        assert_eq!(advisories[0].severity, "high");
    }

    #[test]
    fn advisory_with_exception_is_green_and_without_is_red() {
        let advisories = extract_advisories(&audit_json());

        let with = parse_policy(&policy_json(true)).expect("policy");
        let (unexcepted, excepted) = split_advisories(advisories.clone(), &with);
        assert!(unexcepted.is_empty(), "excepted advisory must not stay red");
        assert_eq!(excepted.len(), 1);

        let without = parse_policy(&policy_json(false)).expect("policy");
        let (unexcepted, excepted) = split_advisories(advisories, &without);
        assert_eq!(
            unexcepted.len(),
            1,
            "the SAME finding without its exception must be red — the discriminating half"
        );
        assert!(excepted.is_empty());
    }

    #[test]
    fn license_dev_lgpl_allowed_runtime_lgpl_red() {
        let policy = parse_policy(&policy_json(false)).expect("policy");
        let pkgs = vec![
            LockPackage {
                name: "pa11y".into(),
                version: Some("9.1.1".into()),
                dev: true,
                license: Some("LGPL-3.0-only".into()),
            },
            LockPackage {
                name: "hypothetical-runtime-lgpl".into(),
                version: Some("1.0.0".into()),
                dev: false,
                license: Some("LGPL-3.0-only".into()),
            },
        ];
        let (violations, excepted) = check_licenses(&pkgs, &policy);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].package, "hypothetical-runtime-lgpl");
        assert_eq!(violations[0].class, "runtime");
        assert_eq!(excepted, 0);
    }

    #[test]
    fn license_missing_field_needs_exception() {
        let policy = parse_policy(&policy_json(false)).expect("policy");
        let pkgs = vec![
            LockPackage {
                name: "css-value".into(),
                version: Some("0.0.1".into()),
                dev: true,
                license: None,
            },
            LockPackage {
                name: "not-excepted".into(),
                version: None,
                dev: true,
                license: None,
            },
        ];
        let (violations, excepted) = check_licenses(&pkgs, &policy);
        assert_eq!(excepted, 1, "css-value rides its license exception");
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].package, "not-excepted");
    }

    #[test]
    fn bans_prefix_matches_scoped_family_and_exact_matches_bare_name() {
        let policy = parse_policy(&policy_json(false)).expect("policy");
        let pkgs = vec![
            LockPackage {
                name: "@radix-ui/react-dialog".into(),
                version: None,
                dev: true,
                license: Some("MIT".into()),
            },
            LockPackage {
                name: "ariakit".into(),
                version: None,
                dev: true,
                license: Some("MIT".into()),
            },
            LockPackage {
                name: "ariakit-utils".into(),
                version: None,
                dev: true,
                license: Some("MIT".into()),
            },
        ];
        let hits = check_bans(&pkgs, &policy);
        assert_eq!(hits.len(), 2);
        assert!(
            hits.iter()
                .any(|(p, b)| p == "@radix-ui/react-dialog" && b == "@radix-ui/")
        );
        assert!(hits.iter().any(|(p, b)| p == "ariakit" && b == "ariakit"));
        assert!(
            !hits.iter().any(|(p, _)| p == "ariakit-utils"),
            "a bare-name ban is exact, never a prefix"
        );
    }

    #[test]
    fn classify_audit_error_json_is_infra_not_findings() {
        let err = json!({"error": {"code": "ENOTFOUND", "summary": "registry unreachable"}});
        match classify_audit_stdout(&err.to_string()) {
            AuditOutcome::Infra(d) => assert!(d.contains("ENOTFOUND")),
            AuditOutcome::Report(_) => panic!("error JSON must classify as infra, never findings"),
        }
        match classify_audit_stdout("not json at all") {
            AuditOutcome::Infra(d) => assert!(d.contains("not JSON")),
            AuditOutcome::Report(_) => panic!("unparseable stdout must classify as infra"),
        }
        match classify_audit_stdout(&audit_json().to_string()) {
            AuditOutcome::Report(_) => {}
            AuditOutcome::Infra(d) => panic!("real report classified as infra: {d}"),
        }
    }

    #[test]
    fn policy_exception_without_provenance_is_rejected() {
        let mut v = policy_json(true);
        v["advisory_exceptions"][0]
            .as_object_mut()
            .expect("obj")
            .remove("closing_condition");
        let err = parse_policy(&v).expect_err("missing closing_condition must fail");
        assert!(format!("{err:#}").contains("closing_condition"));

        let mut v = policy_json(true);
        v["advisory_exceptions"][0]["reason"] = json!("   ");
        let err = parse_policy(&v).expect_err("blank reason must fail");
        assert!(format!("{err:#}").contains("reason"));
    }

    #[test]
    fn lockfile_dev_flag_splits_classes_and_root_entry_skipped() {
        let lock = json!({
            "packages": {
                "": {"name": "ui", "version": "0.1.0"},
                "node_modules/react": {"version": "19.2.0", "license": "MIT"},
                "node_modules/pa11y": {"version": "9.1.1", "dev": true, "license": "LGPL-3.0-only"},
                "node_modules/@wdio/utils/node_modules/import-meta-resolve":
                    {"version": "5.0.0", "dev": true, "license": "MIT"},
            }
        });
        let pkgs = lockfile_packages(&lock).expect("parse");
        assert_eq!(pkgs.len(), 3, "root entry is skipped");
        let react = pkgs.iter().find(|p| p.name == "react").expect("react");
        assert!(!react.dev, "no dev flag = runtime class");
        let pa11y = pkgs.iter().find(|p| p.name == "pa11y").expect("pa11y");
        assert!(pa11y.dev);
        assert!(
            pkgs.iter().any(|p| p.name == "import-meta-resolve"),
            "nested node_modules paths resolve to the leaf package name"
        );
    }
}
