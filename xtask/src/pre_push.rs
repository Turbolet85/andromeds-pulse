//! `pre-push:linux` — the Linux-reachable CI gates, run before a push inside a
//! WSL `Ubuntu` clone synced to HEAD plus the working tree.
//!
//! Contract (the xtask verdict shape, per `check:npm-supply-chain`): one
//! pretty-JSON verdict on stdout plus a twin at `target/pre-push/report.json`;
//! exit 0 green · 1 red · 2 cannot-evaluate. Every pin is read from the repo
//! (`rust-toolchain.toml`, ci.yml's Node version and apt list), never restated
//! here. A missing tool or package is `cannot-evaluate`, never green, and the
//! verb never escalates: it prints the one install command for the operator.
//! Stage output goes to stderr so stdout carries the verdict alone. No stage
//! starts pulse-app or binds a port.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use anyhow::Result;
use serde_json::{Value, json};

const WSL: &str = "wsl.exe";
const DISTRO: &str = "Ubuntu";
const CLONE_DIR: &str = "andromeda-pulse-pre-push";
const CACHE_CAP_BYTES: u64 = 40 * 1024 * 1024 * 1024;
const SCRIPT: &str = "scripts/agent-run.sh";

/// The distro's apt `nodejs` is an older major than ci.yml's pin (npm 10 and 11
/// disagree on lockfiles), so Node comes from the user-local install the Viola
/// repo's `scripts/wsl-provision.sh` maintains on this host.
const NODE_BIN: &str = ".local/viola-node/bin";

/// Tools the stages call, each with the apt package that provides it.
const TOOLS: [(&str, &str); 4] = [
    ("git", "git"),
    ("jq", "jq"),
    ("xvfb-run", "xvfb"),
    ("cc", "gcc"),
];

/// A heartbeat pair and a boot record, no metric: drives the heartbeat arm and
/// the perf-budget script's empty-metric-stream path on Linux bash.
const SEED_LOG: &str = concat!(
    r#"{"timestamp":"2026-01-01T00:00:00.000000Z","level":"INFO","target":"app.boot.ready","fields":{"message":"pre-push seed"}}"#,
    "\n",
    r#"{"timestamp":"2026-01-01T00:00:00.000000Z","level":"INFO","target":"ingest.tick","fields":{}}"#,
    "\n",
    r#"{"timestamp":"2026-01-01T00:00:15.000000Z","level":"INFO","target":"ingest.tick","fields":{}}"#,
    "\n",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verdict {
    Green,
    Red,
    CannotEvaluate,
}

impl Verdict {
    pub(crate) fn word(self) -> &'static str {
        match self {
            Verdict::Green => "green",
            Verdict::Red => "red",
            Verdict::CannotEvaluate => "cannot-evaluate",
        }
    }

    pub(crate) fn exit(self) -> u8 {
        match self {
            Verdict::Green => 0,
            Verdict::Red => 1,
            Verdict::CannotEvaluate => 2,
        }
    }
}

#[derive(Default)]
struct Doc {
    head: Option<String>,
    tree: Option<String>,
    stages: Vec<Value>,
    missing: Vec<String>,
    remediation: Option<String>,
    cache: Option<Value>,
}

struct Stop {
    verdict: Verdict,
    reason: String,
}

impl Stop {
    fn cannot(reason: &str) -> Self {
        Self {
            verdict: Verdict::CannotEvaluate,
            reason: reason.to_owned(),
        }
    }

    fn red(reason: impl Into<String>) -> Self {
        Self {
            verdict: Verdict::Red,
            reason: reason.into(),
        }
    }
}

pub(crate) fn run() -> Result<ExitCode> {
    let root = workspace_root();
    let mut doc = Doc::default();
    let (verdict, reason) = match drive(&root, &mut doc) {
        Ok(()) => (Verdict::Green, "all-stages-ok".to_owned()),
        Err(stop) => (stop.verdict, stop.reason),
    };
    let payload = json!({
        "verdict": verdict.word(),
        "reason": reason,
        "head": doc.head,
        "tree": doc.tree,
        "stages": doc.stages,
        "missing": doc.missing,
        "remediation": doc.remediation,
        "cache": doc.cache,
    });
    let text = serde_json::to_string_pretty(&payload)?;
    let dir = root.join("target").join("pre-push");
    if fs::create_dir_all(&dir).is_ok() {
        let _ = fs::write(dir.join("report.json"), format!("{text}\n"));
    }
    if let Some(cmd) = &doc.remediation {
        eprintln!("pre-push:linux: run once in the distro, then re-run this verb:\n{cmd}");
    }
    println!("{text}");
    Ok(ExitCode::from(verdict.exit()))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn drive(root: &Path, doc: &mut Doc) -> Result<(), Stop> {
    if !cfg!(windows) {
        return Err(Stop::cannot("not-windows"));
    }
    let linux = Linux::new(distro_home()?);

    let channel = fs::read_to_string(root.join("rust-toolchain.toml"))
        .ok()
        .and_then(|t| rust_channel(&t));
    let ci = fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap_or_default();
    let (Some(channel), Some(node), Some(apt)) = (channel, node_major(&ci), apt_packages(&ci))
    else {
        return Err(Stop::cannot("pins-unreadable"));
    };

    let (missing, apt_missing) = provisioning(&linux, &channel, &node, &apt);
    if !missing.is_empty() {
        doc.remediation = remediation(&apt_missing);
        doc.missing = missing;
        return Err(Stop::cannot("provisioning-missing"));
    }

    let (head, tree) = sync(root, &linux)?;
    doc.head = Some(head);
    doc.tree = Some(tree);
    doc.cache = Some(cache(&linux)?);

    let data_dir = format!("{}/target/pre-push/data", linux.clone);
    for stage in Stage::ALL {
        let started = Instant::now();
        let ok = stage.run(&linux, &data_dir);
        doc.stages
            .push(json!({"name": stage.name(), "ok": ok, "ms": elapsed_ms(started)}));
        if !ok {
            return Err(Stop::red(format!("stage-failed:{}", stage.name())));
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Stage {
    ScriptModes,
    Npm,
    Clippy,
    Test,
    CiGates,
}

impl Stage {
    const ALL: [Stage; 5] = [
        Stage::ScriptModes,
        Stage::Npm,
        Stage::Clippy,
        Stage::Test,
        Stage::CiGates,
    ];

    fn name(self) -> &'static str {
        match self {
            Stage::ScriptModes => "script-modes",
            Stage::Npm => "npm",
            Stage::Clippy => "clippy",
            Stage::Test => "test",
            Stage::CiGates => "ci-gates",
        }
    }

    fn run(self, linux: &Linux, data_dir: &str) -> bool {
        let ui = format!("{}/pulse-app/ui", linux.clone);
        let status = |dir: &str, argv: &[&str]| {
            linux
                .cmd(Some(dir), argv, Some(data_dir))
                .status()
                .is_ok_and(|s| s.success())
        };
        match self {
            Stage::ScriptModes => {
                let line = out(&mut linux.git(&["ls-files", "-s", SCRIPT])).unwrap_or_default();
                let mode = script_mode(&line);
                if mode != Some("100755") {
                    eprintln!(
                        "pre-push:linux: {SCRIPT} is git mode {} (CI runs it by path; 100755 required)",
                        mode.unwrap_or("absent")
                    );
                }
                mode == Some("100755")
            }
            Stage::Npm => status(&ui, &["npm", "ci"]) && status(&ui, &["npm", "run", "build"]),
            Stage::Clippy => status(
                &linux.clone,
                &[
                    "cargo",
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--all-features",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            Stage::Test => status(&linux.clone, &["cargo", "xtask", "test"]),
            Stage::CiGates => {
                seed(linux, data_dir) && status(&linux.clone, &["cargo", "xtask", "ci-gates"])
            }
        }
    }
}

/// The distro side: its user's home and the clone under it.
struct Linux {
    home: String,
    clone: String,
}

impl Linux {
    fn new(home: String) -> Self {
        let clone = format!("{home}/{CLONE_DIR}");
        Self { home, clone }
    }

    /// `--exec` passes argv verbatim (the `--` form re-parses it through the
    /// distro shell), and `env -i` hands the child only HOME, a Linux PATH and
    /// the data dir: nothing of this host's environment crosses.
    fn cmd(&self, cd: Option<&str>, argv: &[&str], data_dir: Option<&str>) -> Command {
        let mut cmd = Command::new(WSL);
        cmd.args(["-d", DISTRO]);
        if let Some(dir) = cd {
            cmd.args(["--cd", dir]);
        }
        cmd.args(["--exec", "/usr/bin/env", "-i"])
            .arg(format!("HOME={}", self.home))
            .arg(format!(
                "PATH={home}/.cargo/bin:{home}/{NODE_BIN}:/usr/local/bin:/usr/bin:/bin",
                home = self.home
            ));
        if let Some(dir) = data_dir {
            cmd.arg(format!("ANDROMEDA_PULSE_DATA_DIR={dir}"));
        }
        cmd.args(argv).stdin(Stdio::null());
        cmd.stdout(Stdio::from(std::io::stderr()));
        cmd
    }

    fn git(&self, args: &[&str]) -> Command {
        let mut argv = vec!["git", "-C", self.clone.as_str()];
        argv.extend_from_slice(args);
        self.cmd(None, &argv, None)
    }
}

/// Stdout of a successful call; stderr stays on this process's stderr.
fn out(cmd: &mut Command) -> Option<String> {
    let output = cmd.stdout(Stdio::piped()).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn elapsed_ms(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

/// The distro answers with its user's home, which doubles as its presence probe.
fn distro_home() -> Result<String, Stop> {
    let output = Command::new(WSL)
        .args(["-d", DISTRO, "--exec", "/usr/bin/printenv", "HOME"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| Stop::cannot("wsl-missing"))?;
    let home = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if output.status.success() && home.starts_with('/') {
        Ok(home)
    } else {
        Err(Stop::cannot("distro-missing"))
    }
}

/// Every missing item by name, plus the apt packages that would supply the
/// apt-installable ones.
fn provisioning(
    linux: &Linux,
    channel: &str,
    node: &str,
    apt: &[String],
) -> (Vec<String>, Vec<String>) {
    let mut missing = Vec::new();
    let mut apt_missing = Vec::new();

    let toolchains = out(&mut linux.cmd(None, &["rustup", "toolchain", "list"], None));
    let has_channel = toolchains
        .as_deref()
        .is_some_and(|t| t.lines().any(|l| l.starts_with(&format!("{channel}-"))));
    if !has_channel {
        missing.push(format!(
            "rust:{channel} (rustup toolchain install {channel})"
        ));
    } else {
        let toolchain = format!("+{channel}");
        for (sub, name) in [("clippy", "rust:clippy"), ("nextest", "cargo-nextest")] {
            if out(&mut linux.cmd(None, &["cargo", &toolchain, sub, "--version"], None)).is_none() {
                missing.push(name.to_owned());
            }
        }
    }

    let version = out(&mut linux.cmd(None, &["node", "--version"], None)).unwrap_or_default();
    if node_major_of(&version) != Some(node) {
        missing.push(format!("node:{node} (found {})", version.trim()));
    }

    for (tool, package) in TOOLS {
        if out(&mut linux.cmd(None, &["which", tool], None)).is_none() {
            missing.push(format!("tool:{tool}"));
            apt_missing.push(package.to_owned());
        }
    }

    let mut query = vec!["dpkg-query", "-W", "--showformat=${Package} ${Status}\\n"];
    query.extend(apt.iter().map(String::as_str));
    let mut dpkg = linux.cmd(None, &query, None);
    let listed = dpkg
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    let installed = installed_packages(&listed);
    for package in apt {
        if !installed.contains(package) {
            missing.push(format!("apt:{package}"));
            if !apt_missing.contains(package) {
                apt_missing.push(package.clone());
            }
        }
    }
    (missing, apt_missing)
}

/// The working tree as one binary patch through a temporary index (the real
/// one is never touched), applied to the clone at the same HEAD; the clone's
/// tree id must then equal the working tree's.
fn sync(root: &Path, linux: &Linux) -> Result<(String, String), Stop> {
    let failed = |step: &str| Stop::red(format!("sync-failed:{step}"));
    let src = wsl_path(root).ok_or_else(|| failed("source-path"))?;
    let dir = root.join("target").join("pre-push");
    fs::create_dir_all(&dir).map_err(|_| failed("patch"))?;
    let index = dir.join("index");
    let patch = dir.join("tree.patch");
    let _ = fs::remove_file(&index);
    let host = |args: &[&str]| {
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(root)
            .args(["-c", "core.safecrlf=false"])
            .args(args)
            .env("GIT_INDEX_FILE", &index);
        cmd
    };
    let head = out(&mut host(&["rev-parse", "HEAD"])).ok_or_else(|| failed("patch"))?;
    let head = head.trim().to_owned();
    let mut tree = String::new();
    for args in [&["read-tree", "HEAD"][..], &["add", "-A"], &["write-tree"]] {
        tree = out(&mut host(args)).ok_or_else(|| failed("patch"))?;
    }
    let tree = tree.trim().to_owned();
    let names = out(&mut host(&["diff", "--cached", "--name-only", "HEAD"]))
        .ok_or_else(|| failed("patch"))?;
    let files = names.lines().filter(|l| !l.trim().is_empty()).count();
    let output = format!("--output={}", patch.display());
    let diff = [
        "diff",
        "--cached",
        "--binary",
        "--no-color",
        "--no-ext-diff",
        &output,
        "HEAD",
    ];
    out(&mut host(&diff)).ok_or_else(|| failed("patch"))?;

    if out(&mut linux.git(&["rev-parse", "--git-dir"])).is_none() {
        let clone = ["git", "clone", "-q", "--no-hardlinks", &src, &linux.clone];
        out(&mut linux.cmd(None, &clone, None)).ok_or_else(|| failed("clone"))?;
    }
    out(&mut linux.git(&["fetch", "-q", "--no-tags", &src, "HEAD"]))
        .ok_or_else(|| failed("fetch"))?;
    let fetched = out(&mut linux.git(&["rev-parse", "FETCH_HEAD"]));
    if fetched.as_deref().map(str::trim) != Some(head.as_str()) {
        return Err(failed("fetch"));
    }
    out(&mut linux.git(&["reset", "-q", "--hard", &head])).ok_or_else(|| failed("reset"))?;
    out(&mut linux.git(&["clean", "-fdq"])).ok_or_else(|| failed("clean"))?;
    if files > 0 {
        let patch = wsl_path(&patch).ok_or_else(|| failed("apply"))?;
        out(&mut linux.git(&["apply", "--binary", &patch])).ok_or_else(|| failed("apply"))?;
    }
    out(&mut linux.git(&["add", "-A"])).ok_or_else(|| failed("apply"))?;
    let synced = out(&mut linux.git(&["write-tree"])).ok_or_else(|| failed("apply"))?;
    if synced.trim() != tree {
        return Err(Stop::red("sync-mismatch"));
    }
    Ok((head, tree))
}

/// The clone's `target/` against its cap, cleared above it.
fn cache(linux: &Linux) -> Result<Value, Stop> {
    let target = format!("{}/target", linux.clone);
    let bytes = out(&mut linux.cmd(None, &["du", "-sb", &target], None))
        .and_then(|t| t.split_whitespace().next()?.parse::<u64>().ok())
        .unwrap_or(0);
    let cleaned = bytes > CACHE_CAP_BYTES;
    if cleaned {
        out(&mut linux.cmd(None, &["rm", "-rf", &target], None))
            .ok_or_else(|| Stop::red("sync-failed:cache"))?;
    }
    Ok(json!({"bytes": bytes, "cap": CACHE_CAP_BYTES, "cleaned": cleaned}))
}

/// A fresh data dir whose log holds only `SEED_LOG`.
fn seed(linux: &Linux, data_dir: &str) -> bool {
    let script = r#"rm -rf "$1" && mkdir -p "$1/logs" && cat > "$1/logs/agent-latest.jsonl""#;
    let mut cmd = linux.cmd(None, &["/bin/sh", "-c", script, "sh", data_dir], None);
    let Ok(mut child) = cmd.stdin(Stdio::piped()).spawn() else {
        return false;
    };
    let wrote = child
        .stdin
        .take()
        .is_some_and(|mut stdin| stdin.write_all(SEED_LOG.as_bytes()).is_ok());
    child.wait().is_ok_and(|s| s.success()) && wrote
}

/// A drive path as the distro mounts it (`D:\a\b` → `/mnt/d/a/b`); `None` for
/// anything else.
pub(crate) fn wsl_path(path: &Path) -> Option<String> {
    let text = path.to_string_lossy();
    let text = text.strip_prefix(r"\\?\").unwrap_or(&text);
    let mut chars = text.chars();
    let drive = chars.next().filter(char::is_ascii_alphabetic)?;
    let rest = chars.as_str().strip_prefix(':')?;
    if !(rest.is_empty() || rest.starts_with(['\\', '/'])) {
        return None;
    }
    Some(format!(
        "/mnt/{}{}",
        drive.to_ascii_lowercase(),
        rest.replace('\\', "/")
    ))
}

/// `channel = "…"` under `[toolchain]`.
pub(crate) fn rust_channel(toml: &str) -> Option<String> {
    let mut in_toolchain = false;
    for line in toml.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_toolchain = line == "[toolchain]";
            continue;
        }
        if !in_toolchain {
            continue;
        }
        if let Some(value) = line.strip_prefix("channel").map(str::trim_start)
            && let Some(value) = value.strip_prefix('=')
        {
            let value = value.trim().trim_matches('"');
            return (!value.is_empty()).then(|| value.to_owned());
        }
    }
    None
}

/// ci.yml's `node-version:` major; `None` when absent, not a bare major, or
/// when two jobs pin different ones.
pub(crate) fn node_major(ci_yml: &str) -> Option<String> {
    let mut pins = ci_yml.lines().filter_map(|l| {
        let value = l.trim().strip_prefix("node-version:")?;
        Some(value.trim().trim_matches(['\'', '"']).to_owned())
    });
    let first = pins.next()?;
    if first.is_empty() || !first.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    pins.all(|p| p == first).then_some(first)
}

/// The packages of ci.yml's first `apt-get install` command, continuation
/// lines joined; `None` when there is no such command or it names nothing.
pub(crate) fn apt_packages(ci_yml: &str) -> Option<Vec<String>> {
    let mut lines = ci_yml
        .lines()
        .skip_while(|l| !l.contains("apt-get install"));
    let mut command = String::new();
    for line in lines.by_ref() {
        let line = line.trim();
        match line.strip_suffix('\\') {
            Some(head) => {
                command.push_str(head);
                command.push(' ');
            }
            None => {
                command.push_str(line);
                break;
            }
        }
    }
    let (_, args) = command.split_once("apt-get install")?;
    let packages: Vec<String> = args
        .split_whitespace()
        .filter(|t| !t.starts_with('-'))
        .map(str::to_owned)
        .collect();
    (!packages.is_empty()).then_some(packages)
}

/// The mode of one `git ls-files -s` line (`100755 <sha> 0\t<path>`).
pub(crate) fn script_mode(ls_files_line: &str) -> Option<&str> {
    let (meta, _path) = ls_files_line.lines().next()?.split_once('\t')?;
    let mode = meta.split_whitespace().next()?;
    (mode.len() == 6 && mode.chars().all(|c| c.is_ascii_digit())).then_some(mode)
}

/// Package names `dpkg-query -W -f='${Package} ${Status}\n'` reports installed.
pub(crate) fn installed_packages(dpkg: &str) -> BTreeSet<String> {
    dpkg.lines()
        .filter_map(|l| l.trim().strip_suffix(" install ok installed"))
        .map(str::to_owned)
        .collect()
}

/// `v24.21.0` → `24`.
pub(crate) fn node_major_of(version: &str) -> Option<&str> {
    let major = version.trim().strip_prefix('v')?.split('.').next()?;
    (!major.is_empty() && major.chars().all(|c| c.is_ascii_digit())).then_some(major)
}

/// The one install command the operator runs with sudo; `None` when nothing
/// apt-installable is missing.
pub(crate) fn remediation(apt_missing: &[String]) -> Option<String> {
    (!apt_missing.is_empty()).then(|| {
        format!(
            "sudo apt-get install -y --no-install-recommends {}",
            apt_missing.join(" ")
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CI: &str = "      - name: Install Linux system libraries (Tauri + dbus)
        if: runner.os == 'Linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y --no-install-recommends pkg-config libssl-dev \\
            libdbus-1-dev xvfb

      - name: Setup Node
        with:
          node-version: '24'
";

    #[test]
    fn apt_packages_join_continuations_and_drop_flags() {
        assert_eq!(
            apt_packages(CI),
            Some(
                ["pkg-config", "libssl-dev", "libdbus-1-dev", "xvfb"]
                    .map(str::to_owned)
                    .to_vec()
            )
        );
        assert_eq!(apt_packages("run: sudo apt-get update\n"), None);
        assert_eq!(apt_packages("run: sudo apt-get install -y\n"), None);
    }

    #[test]
    fn apt_packages_read_the_real_workflow() {
        let ci =
            fs::read_to_string(workspace_root().join(".github/workflows/ci.yml")).expect("ci.yml");
        let packages = apt_packages(&ci).expect("apt list");
        assert!(packages.contains(&"libwebkit2gtk-4.1-dev".to_owned()));
        assert!(packages.contains(&"xvfb".to_owned()));
        assert!(!packages.iter().any(|p| p.starts_with('-')));
        assert_eq!(node_major(&ci).as_deref(), Some("24"));
    }

    #[test]
    fn node_major_reads_one_agreed_major() {
        assert_eq!(node_major(CI).as_deref(), Some("24"));
        let twice = format!("{CI}          node-version: \"24\"\n");
        assert_eq!(node_major(&twice).as_deref(), Some("24"));
        let split = format!("{CI}          node-version: '22'\n");
        assert_eq!(node_major(&split), None, "two jobs disagree");
        assert_eq!(node_major("node-version: '24.x'\n"), None);
        assert_eq!(node_major("jobs: {}\n"), None);
    }

    #[test]
    fn rust_channel_reads_the_toolchain_table() {
        let toml = "[toolchain]\nchannel = \"1.95.0\"\ncomponents = [\"clippy\"]\n";
        assert_eq!(rust_channel(toml).as_deref(), Some("1.95.0"));
        assert_eq!(
            rust_channel("[other]\nchannel = \"1.0.0\"\n[toolchain]\n"),
            None
        );
        assert_eq!(rust_channel("[toolchain]\nchannel = \"\"\n"), None);
        let real = fs::read_to_string(workspace_root().join("rust-toolchain.toml")).expect("toml");
        assert!(rust_channel(&real).is_some());
    }

    #[test]
    fn verdict_maps_to_word_and_exit() {
        assert_eq!((Verdict::Green.word(), Verdict::Green.exit()), ("green", 0));
        assert_eq!((Verdict::Red.word(), Verdict::Red.exit()), ("red", 1));
        assert_eq!(
            (
                Verdict::CannotEvaluate.word(),
                Verdict::CannotEvaluate.exit()
            ),
            ("cannot-evaluate", 2)
        );
        assert_eq!(
            Stop::cannot("provisioning-missing").verdict,
            Verdict::CannotEvaluate,
            "a missing pin never reads green"
        );
    }

    #[test]
    fn script_mode_parses_ls_files() {
        let exec = "100755 0123456789abcdef0123456789abcdef01234567 0\tscripts/agent-run.sh\n";
        assert_eq!(script_mode(exec), Some("100755"));
        let plain = "100644 0123456789abcdef0123456789abcdef01234567 0\tscripts/agent-run.sh\n";
        assert_eq!(script_mode(plain), Some("100644"));
        assert_eq!(script_mode(""), None, "untracked script");
        assert_eq!(script_mode("garbage\n"), None);
    }

    #[test]
    fn installed_packages_keep_only_installed() {
        let out =
            "jq install ok installed\nxvfb deinstall ok config-files\ngit install ok installed\n";
        let set = installed_packages(out);
        assert!(set.contains("jq") && set.contains("git"));
        assert!(!set.contains("xvfb"));
        assert!(installed_packages("").is_empty());
    }

    #[test]
    fn node_major_of_reads_v_prefixed_versions() {
        assert_eq!(node_major_of("v24.21.0\n"), Some("24"));
        assert_eq!(node_major_of("24.21.0"), None);
        assert_eq!(node_major_of(""), None);
    }

    #[test]
    fn remediation_is_one_apt_line_or_none() {
        assert_eq!(
            remediation(&["jq".to_owned(), "libxdo-dev".to_owned()]).as_deref(),
            Some("sudo apt-get install -y --no-install-recommends jq libxdo-dev")
        );
        assert_eq!(remediation(&[]), None);
    }

    #[test]
    fn wsl_path_maps_drive_paths() {
        let cases = [
            (
                r"D:\dev\projects\andromeda-pulse",
                Some("/mnt/d/dev/projects/andromeda-pulse"),
            ),
            ("c:/a/b", Some("/mnt/c/a/b")),
            (r"\\?\E:\x", Some("/mnt/e/x")),
            ("D:x", None),
            (r"\\server\share", None),
            ("/home/x", None),
        ];
        for (input, want) in cases {
            assert_eq!(wsl_path(Path::new(input)).as_deref(), want, "{input}");
        }
    }
}
