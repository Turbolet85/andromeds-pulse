//! `pre-push:linux` — the Linux-reachable CI gates, run before a push on the
//! Linux dev host itself, in the working tree.
//!
//! Contract (the xtask verdict shape, per `check:npm-supply-chain`): one
//! pretty-JSON verdict on stdout plus a twin at `target/pre-push/report.json`;
//! exit 0 green · 1 red · 2 cannot-evaluate. Six stages run in order and the
//! first failure stops. Every pin is read from the repo (`rust-toolchain.toml`,
//! ci.yml's Node version), never restated here. An unmet pin or a missing tool
//! is `cannot-evaluate`, never green, and the verb installs nothing. Every
//! stage child gets a cleared environment plus the set `stage_env` builds, and
//! the per-run area `target/pre-push/run/` is removed and created again on
//! each run. The `test` stage rewrites the tracked TauRPC bindings and the
//! verb puts them back as found. Stage output goes to stderr so stdout carries
//! the verdict alone. The `ci-gates` stage runs one console engine cycle
//! (`harness:engine-cycle`, which starts `andromeda-pulse-engine` on the
//! harness-only loopback ports 24317 and 24318) over the stage data dir, then
//! `ci-gates` over that cycle's log. A cycle port already taken is
//! `cannot-evaluate`, never green, and the stage ends what it started
//! whichever way the cycle returned. No other stage starts a program or binds
//! a port.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use anyhow::Result;
use serde_json::{Value, json};

use crate::engine_cycle::{DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT};
use crate::harness_ready::receiver_accepts;
use crate::harness_status::{pid_alive, read_pid};

const SCRIPT: &str = "scripts/agent-run.sh";
const BINDINGS: &str = "pulse-app/ui/src/bindings/index.ts";
const SYSTEM_PATH: &str = "/usr/local/bin:/usr/bin:/bin";

/// One variable of a child's environment.
type Var = (OsString, OsString);

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
}

#[derive(Debug, PartialEq, Eq)]
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
    let text = serde_json::to_string_pretty(&document(verdict, &reason, &doc))?;
    let dir = root.join("target").join("pre-push");
    if fs::create_dir_all(&dir).is_ok() {
        let _ = fs::write(dir.join("report.json"), format!("{text}\n"));
    }
    println!("{text}");
    Ok(ExitCode::from(verdict.exit()))
}

/// The verdict document. No member carries an environment value or a path:
/// `missing` names tools and the versions they report.
fn document(verdict: Verdict, reason: &str, doc: &Doc) -> Value {
    json!({
        "verdict": verdict.word(),
        "reason": reason,
        "head": doc.head,
        "tree": doc.tree,
        "stages": doc.stages,
        "missing": doc.missing,
    })
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// The stages are the Linux CI jobs' own commands; no other system evaluates
/// them.
fn host_supported(os: &str) -> bool {
    os == "linux"
}

fn drive(root: &Path, doc: &mut Doc) -> Result<(), Stop> {
    if !host_supported(std::env::consts::OS) {
        return Err(Stop::cannot("not-linux"));
    }

    let channel = fs::read_to_string(root.join("rust-toolchain.toml"))
        .ok()
        .and_then(|t| rust_channel(&t));
    let ci = fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap_or_default();
    let (Some(channel), Some(node)) = (channel, node_major(&ci)) else {
        return Err(Stop::cannot("pins-unreadable"));
    };

    let Some(home) = std::env::var_os("HOME").filter(|home| !home.is_empty()) else {
        return Err(Stop::cannot("home-unset"));
    };
    let verb_path = std::env::var_os("PATH").unwrap_or_default();
    let area = RunArea::under(root);
    let host = Host {
        root,
        area: &area,
        plain: stage_env(&home, &verb_path, &area, false),
        npm: stage_env(&home, &verb_path, &area, true),
    };

    let probes = probe(&host, &stage_path(&home, &verb_path), &channel);
    let missing = missing_pieces(&probes, &channel, &node);
    if let Some(stop) = provisioning_stop(&missing) {
        doc.missing = missing;
        return Err(stop);
    }

    area.reset().map_err(|_| Stop::cannot("run-dir-unusable"))?;
    let (head, tree) = head_and_tree(&host)?;
    doc.head = Some(head);
    doc.tree = Some(tree);

    for stage in Stage::ALL {
        let started = Instant::now();
        let outcome = stage.run(&host);
        doc.stages
            .push(json!({"name": stage.name(), "ok": outcome.is_ok(), "ms": elapsed_ms(started)}));
        outcome?;
    }
    Ok(())
}

/// What every probe and stage runs against: the working tree, the per-run
/// area, and the two variable sets a child may receive.
struct Host<'a> {
    root: &'a Path,
    area: &'a RunArea,
    plain: Vec<Var>,
    npm: Vec<Var>,
}

#[derive(Clone, Copy)]
enum Stage {
    ScriptModes,
    SourceLint,
    Npm,
    Clippy,
    Test,
    CiGates,
}

impl Stage {
    const ALL: [Stage; 6] = [
        Stage::ScriptModes,
        Stage::SourceLint,
        Stage::Npm,
        Stage::Clippy,
        Stage::Test,
        Stage::CiGates,
    ];

    fn name(self) -> &'static str {
        match self {
            Stage::ScriptModes => "script-modes",
            Stage::SourceLint => "source-lint",
            Stage::Npm => "npm",
            Stage::Clippy => "clippy",
            Stage::Test => "test",
            Stage::CiGates => "ci-gates",
        }
    }

    /// `Err` carries the verdict the run stops on.
    fn run(self, host: &Host) -> Result<(), Stop> {
        let root = host.root;
        let ui = root.join("pulse-app").join("ui");
        let passed = match self {
            Stage::ScriptModes => {
                let line = out(&mut command(
                    &host.plain,
                    root,
                    "git",
                    &["ls-files", "-s", SCRIPT],
                ))
                .unwrap_or_default();
                let mode = script_mode(&line);
                if mode != Some("100755") {
                    eprintln!(
                        "pre-push:linux: {SCRIPT} is git mode {} (CI runs it by path; 100755 required)",
                        mode.unwrap_or("absent")
                    );
                }
                mode == Some("100755")
            }
            Stage::SourceLint => succeeds(
                &host.plain,
                root,
                "cargo",
                &["xtask", "check:english-sources"],
            ),
            Stage::Npm => {
                succeeds(&host.npm, &ui, "npm", &["ci"])
                    && succeeds(&host.npm, &ui, "npm", &["run", "build"])
            }
            Stage::Clippy => succeeds(
                &host.plain,
                root,
                "cargo",
                &[
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--all-features",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            Stage::Test => {
                let test = || succeeds(&host.plain, root, "cargo", &["xtask", "test"]);
                restoring(&root.join(BINDINGS), test)
                    .map_err(|RestoreFailed| Stop::red("restore-failed:bindings"))?
            }
            Stage::CiGates => return decide_ci_gates(read_engine_cycle(host)),
        };
        if passed { Ok(()) } else { Err(self.failed()) }
    }

    fn failed(self) -> Stop {
        Stop::red(format!("stage-failed:{}", self.name()))
    }
}

/// What the `ci-gates` stage read around its engine cycle.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct CycleReading {
    /// A cycle port accepted before the stage had started anything.
    port_taken: bool,
    /// The cycle verb's exit; `None` when it was not run.
    cycle_exit: Option<i32>,
    /// A cycle port still accepting once the verb had returned.
    port_accepting_after: bool,
    /// The pid of the cycle's own pid file still alive once the verb had
    /// returned.
    pid_alive_after: bool,
    /// `cargo xtask ci-gates` over the cycle's log; `None` when it was not
    /// run.
    gates_passed: Option<bool>,
}

/// The stage's decision, pure so each arm is pinnable without a process. A
/// taken port is refused before anything starts; a cycle that left a port
/// accepting or its engine alive is red whatever it returned.
fn decide_ci_gates(reading: CycleReading) -> Result<(), Stop> {
    if reading.port_taken {
        return Err(Stop::cannot("engine-port-taken"));
    }
    if reading.port_accepting_after || reading.pid_alive_after {
        return Err(Stage::CiGates.failed());
    }
    match (reading.cycle_exit, reading.gates_passed) {
        (Some(0), Some(true)) => Ok(()),
        (Some(2), _) => Err(Stop::cannot("engine-cycle-unevaluated")),
        _ => Err(Stage::CiGates.failed()),
    }
}

/// One console engine cycle on the stage data dir, then `ci-gates` over its
/// log when the cycle passed.
fn read_engine_cycle(host: &Host) -> CycleReading {
    let data = &host.area.data;
    let mut reading = CycleReading::default();
    if remove_tree(data).is_err() || fs::create_dir_all(data).is_err() {
        return reading;
    }
    let accepting = || {
        [DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT]
            .into_iter()
            .any(receiver_accepts)
    };
    reading.port_taken = accepting();
    if reading.port_taken {
        return reading;
    }
    let exit = command(
        &host.plain,
        host.root,
        "cargo",
        &[
            "xtask",
            "harness:engine-cycle",
            "--data-dir",
            &data.to_string_lossy(),
        ],
    )
    .status()
    .ok()
    .and_then(|status| status.code())
    .unwrap_or(-1);
    reading.cycle_exit = Some(exit);
    reading.port_accepting_after = accepting();
    reading.pid_alive_after = read_pid(&data.join("run").join("andromeda-pulse.pid"))
        .is_some_and(|pid| pid_alive(pid) != Some(false));
    if reading.port_accepting_after || reading.pid_alive_after {
        end_cycle(host);
    } else if exit == 0 {
        reading.gates_passed = Some(succeeds(
            &host.plain,
            host.root,
            "cargo",
            &["xtask", "ci-gates"],
        ));
    }
    reading
}

/// The script's `cleanup` on the stage data dir, once: it ends the pid the
/// cycle's own pid file holds and no other process.
fn end_cycle(host: &Host) {
    let mut cleanup = command(&host.plain, host.root, "bash", &[SCRIPT, "cleanup"]);
    cleanup
        .env(
            "ANDROMEDA_PULSE_OTLP_GRPC_PORT",
            DEFAULT_GRPC_PORT.to_string(),
        )
        .env(
            "ANDROMEDA_PULSE_OTLP_HTTP_PORT",
            DEFAULT_HTTP_PORT.to_string(),
        );
    let _ = cleanup.status();
}

/// The verb's own per-run area: the browser cache, the stages' data dir and
/// the temporary git index. Nothing else under `target/pre-push/` is its to
/// touch.
struct RunArea {
    dir: PathBuf,
    data: PathBuf,
    puppeteer: PathBuf,
    index: PathBuf,
}

impl RunArea {
    fn under(root: &Path) -> Self {
        let dir = root.join("target").join("pre-push").join("run");
        Self {
            data: dir.join("data"),
            puppeteer: dir.join("puppeteer"),
            index: dir.join("index"),
            dir,
        }
    }

    /// Removed and created again, so the browser cache is fresh on every run
    /// and nothing accumulates.
    fn reset(&self) -> io::Result<()> {
        remove_tree(&self.dir)?;
        fs::create_dir_all(&self.data)?;
        fs::create_dir_all(&self.puppeteer)
    }
}

fn remove_tree(dir: &Path) -> io::Result<()> {
    match fs::remove_dir_all(dir) {
        Err(err) if err.kind() != io::ErrorKind::NotFound => Err(err),
        _ => Ok(()),
    }
}

/// The first absolute directory of a PATH value that holds a file `name`.
fn dir_holding(path: &OsStr, name: &str) -> Option<PathBuf> {
    std::env::split_paths(path).find(|dir| dir.is_absolute() && dir.join(name).is_file())
}

/// The PATH a stage child searches: rustup's proxies, the directory of the
/// first `node` on the verb's own PATH, then the system directories. With no
/// `node` there the directory is left out and the provisioning check names
/// Node as missing.
fn stage_path(home: &OsStr, verb_path: &OsStr) -> OsString {
    let mut path = home.to_os_string();
    path.push("/.cargo/bin");
    if let Some(node) = dir_holding(verb_path, "node") {
        path.push(":");
        path.push(node);
    }
    path.push(":");
    path.push(SYSTEM_PATH);
    path
}

/// The exact variable set a stage child receives on top of a cleared
/// environment. The session bus address, the runtime dir and the display
/// variables are not in it, so no stage reaches an OS credential store.
fn stage_env(home: &OsStr, verb_path: &OsStr, area: &RunArea, npm: bool) -> Vec<Var> {
    let mut env = vec![
        (OsString::from("HOME"), home.to_os_string()),
        (OsString::from("PATH"), stage_path(home, verb_path)),
        (
            OsString::from("ANDROMEDA_PULSE_DATA_DIR"),
            area.data.clone().into_os_string(),
        ),
    ];
    if npm {
        env.push((
            OsString::from("PUPPETEER_CACHE_DIR"),
            area.puppeteer.clone().into_os_string(),
        ));
    }
    env
}

/// A child with a cleared environment plus `env`; its stdout joins stderr so
/// this process's stdout carries the verdict alone.
fn command(env: &[Var], dir: &Path, program: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(program);
    cmd.args(args)
        .current_dir(dir)
        .env_clear()
        .envs(env.iter().map(|(name, value)| (name, value)))
        .stdin(Stdio::null())
        .stdout(Stdio::from(io::stderr()));
    cmd
}

fn succeeds(env: &[Var], dir: &Path, program: &str, args: &[&str]) -> bool {
    command(env, dir, program, args)
        .status()
        .is_ok_and(|s| s.success())
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

/// What the provisioning probes read, each in the stage environment, so what
/// is probed is what the stages will find.
struct Probes {
    toolchains: String,
    clippy: bool,
    nextest: bool,
    node: String,
    npm: bool,
    git: bool,
    cc: bool,
    python3: bool,
}

fn probe(host: &Host, stage_path: &OsStr, channel: &str) -> Probes {
    let answer =
        |program: &str, args: &[&str]| out(&mut command(&host.plain, host.root, program, args));
    let toolchains = answer("rustup", &["toolchain", "list"]).unwrap_or_default();
    // A `cargo +{channel}` call against a channel rustup does not hold could
    // install it; the verb installs nothing.
    let listed = channel_listed(&toolchains, channel);
    let toolchain = format!("+{channel}");
    Probes {
        clippy: listed && answer("cargo", &[&toolchain, "clippy", "--version"]).is_some(),
        nextest: listed && answer("cargo", &[&toolchain, "nextest", "--version"]).is_some(),
        toolchains,
        node: answer("node", &["--version"]).unwrap_or_default(),
        npm: answer("npm", &["--version"]).is_some(),
        git: dir_holding(stage_path, "git").is_some(),
        cc: dir_holding(stage_path, "cc").is_some(),
        python3: ["python3", "python"].into_iter().any(|python| {
            answer(python, &["--version"]).is_some_and(|v| v.starts_with("Python 3"))
        }),
    }
}

fn channel_listed(toolchains: &str, channel: &str) -> bool {
    toolchains
        .lines()
        .any(|l| l.starts_with(&format!("{channel}-")))
}

/// Every required piece the probes did not find, by name.
fn missing_pieces(probes: &Probes, channel: &str, node: &str) -> Vec<String> {
    let mut missing = Vec::new();
    if channel_listed(&probes.toolchains, channel) {
        for (found, name) in [
            (probes.clippy, "rust:clippy"),
            (probes.nextest, "cargo-nextest"),
        ] {
            if !found {
                missing.push(name.to_owned());
            }
        }
    } else {
        missing.push(format!("rust:{channel}"));
    }
    if node_major_of(&probes.node) != Some(node) {
        missing.push(format!(
            "node:{node} (found {})",
            found_version(&probes.node)
        ));
    }
    for (found, name) in [
        (probes.npm, "tool:npm"),
        (probes.git, "tool:git"),
        (probes.cc, "tool:cc"),
        (probes.python3, "tool:python3"),
    ] {
        if !found {
            missing.push(name.to_owned());
        }
    }
    missing
}

/// A tool's version line as the verdict may carry it: a short version token,
/// never free text or a path.
fn found_version(raw: &str) -> &str {
    let version = raw.trim();
    let token = |c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+');
    if version.is_empty() {
        "none"
    } else if version.len() <= 32 && version.chars().all(token) {
        version
    } else {
        "unreadable"
    }
}

/// Anything missing stops the run before a stage: cannot-evaluate, never
/// green.
fn provisioning_stop(missing: &[String]) -> Option<Stop> {
    (!missing.is_empty()).then(|| Stop::cannot("provisioning-missing"))
}

/// HEAD's sha and the tree id of HEAD plus the working tree, written through a
/// temporary index in the per-run area (the real one is never touched).
fn head_and_tree(host: &Host) -> Result<(String, String), Stop> {
    let unreadable = || Stop::cannot("tree-unreadable");
    let git = |args: &[&str]| {
        let mut cmd = command(
            &host.plain,
            host.root,
            "git",
            &["-c", "core.safecrlf=false"],
        );
        cmd.args(args).env("GIT_INDEX_FILE", &host.area.index);
        cmd
    };
    let head = out(&mut git(&["rev-parse", "HEAD"])).ok_or_else(unreadable)?;
    let mut tree = String::new();
    for args in [&["read-tree", "HEAD"][..], &["add", "-A"], &["write-tree"]] {
        tree = out(&mut git(args)).ok_or_else(unreadable)?;
    }
    Ok((head.trim().to_owned(), tree.trim().to_owned()))
}

#[derive(Debug, PartialEq, Eq)]
struct RestoreFailed;

/// Runs `action`, then puts `path` back to the bytes it held before, or
/// removes it when it was absent, whichever way the action ended. `Ok`
/// carries the action's own result. The restore runs when the action returns,
/// not on a signal.
fn restoring(path: &Path, action: impl FnOnce() -> bool) -> Result<bool, RestoreFailed> {
    let before = match fs::read(path) {
        Ok(bytes) => Some(bytes),
        Err(err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(_) => return Err(RestoreFailed),
    };
    let ended = action();
    let restored = match &before {
        Some(bytes) => {
            fs::read(path).is_ok_and(|now| now == *bytes) || fs::write(path, bytes).is_ok()
        }
        None => !matches!(fs::remove_file(path), Err(err) if err.kind() != io::ErrorKind::NotFound),
    };
    if restored {
        Ok(ended)
    } else {
        Err(RestoreFailed)
    }
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

/// The mode of one `git ls-files -s` line (`100755 <sha> 0\t<path>`).
pub(crate) fn script_mode(ls_files_line: &str) -> Option<&str> {
    let (meta, _path) = ls_files_line.lines().next()?.split_once('\t')?;
    let mode = meta.split_whitespace().next()?;
    (mode.len() == 6 && mode.chars().all(|c| c.is_ascii_digit())).then_some(mode)
}

/// `v24.21.0` → `24`.
pub(crate) fn node_major_of(version: &str) -> Option<&str> {
    let major = version.trim().strip_prefix('v')?.split('.').next()?;
    (!major.is_empty() && major.chars().all(|c| c.is_ascii_digit())).then_some(major)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use tempfile::TempDir;

    use super::*;

    const CI: &str = "      - name: Setup Node
        with:
          node-version: '24'
";

    const SESSION_VARIABLES: [&str; 4] = [
        "DBUS_SESSION_BUS_ADDRESS",
        "XDG_RUNTIME_DIR",
        "DISPLAY",
        "WAYLAND_DISPLAY",
    ];

    fn provisioned() -> Probes {
        Probes {
            toolchains: "stable-x86_64-unknown-linux-gnu (default)\n\
                         1.95.0-x86_64-unknown-linux-gnu (active)\n"
                .to_owned(),
            clippy: true,
            nextest: true,
            node: "v24.21.0\n".to_owned(),
            npm: true,
            git: true,
            cc: true,
            python3: true,
        }
    }

    /// A directory holding an empty file `node`, and one holding nothing.
    fn node_dirs(tmp: &TempDir) -> (PathBuf, PathBuf) {
        let (with, without) = (tmp.path().join("with-node"), tmp.path().join("without"));
        for dir in [&with, &without] {
            fs::create_dir(dir).expect("dir");
        }
        fs::write(with.join("node"), "").expect("node");
        (with, without)
    }

    fn joined(dirs: &[&Path]) -> OsString {
        std::env::join_paths(dirs).expect("a PATH value")
    }

    #[test]
    fn only_a_linux_host_is_supported() {
        assert!(host_supported("linux"));
        for os in ["windows", "macos"] {
            assert!(!host_supported(os), "{os}");
        }
    }

    #[test]
    fn nothing_is_missing_on_a_provisioned_host() {
        let missing = missing_pieces(&provisioned(), "1.95.0", "24");
        assert_eq!(missing, Vec::<String>::new());
        assert!(provisioning_stop(&missing).is_none());
    }

    #[test]
    fn each_required_piece_is_named_when_it_alone_is_missing() {
        type Strip = fn(&mut Probes);
        let cases: [(Strip, &str); 8] = [
            (
                |p: &mut Probes| {
                    p.toolchains = "stable-x86_64-unknown-linux-gnu (default)\n".to_owned();
                },
                "rust:1.95.0",
            ),
            (|p: &mut Probes| p.clippy = false, "rust:clippy"),
            (|p: &mut Probes| p.nextest = false, "cargo-nextest"),
            (|p: &mut Probes| p.node.clear(), "node:24 (found none)"),
            (|p: &mut Probes| p.npm = false, "tool:npm"),
            (|p: &mut Probes| p.git = false, "tool:git"),
            (|p: &mut Probes| p.cc = false, "tool:cc"),
            (|p: &mut Probes| p.python3 = false, "tool:python3"),
        ];
        for (strip, name) in cases {
            let mut probes = provisioned();
            strip(&mut probes);
            let missing = missing_pieces(&probes, "1.95.0", "24");
            assert_eq!(missing, [name]);
            let stop = provisioning_stop(&missing).expect("a missing piece stops the run");
            assert_eq!(
                stop.verdict,
                Verdict::CannotEvaluate,
                "{name}: a missing pin never reads green"
            );
            assert_eq!(stop.reason, "provisioning-missing");
        }
    }

    #[test]
    fn a_node_major_off_the_pin_is_named_with_what_was_found() {
        let mut probes = provisioned();
        probes.node = "v26.8.2\n".to_owned();
        assert_eq!(
            missing_pieces(&probes, "1.95.0", "24"),
            ["node:24 (found v26.8.2)"]
        );
        probes.node = "/opt/node/bin/node: bad interpreter\n".to_owned();
        assert_eq!(
            missing_pieces(&probes, "1.95.0", "24"),
            ["node:24 (found unreadable)"]
        );
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
    fn node_major_reads_the_real_workflow() {
        let ci =
            fs::read_to_string(workspace_root().join(".github/workflows/ci.yml")).expect("ci.yml");
        assert_eq!(node_major(&ci).as_deref(), Some("24"));
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
    fn node_major_of_reads_v_prefixed_versions() {
        assert_eq!(node_major_of("v24.21.0\n"), Some("24"));
        assert_eq!(node_major_of("24.21.0"), None);
        assert_eq!(node_major_of(""), None);
    }

    #[test]
    fn dir_holding_finds_the_first_directory_with_the_file() {
        let tmp = TempDir::new().expect("tempdir");
        let (with, without) = node_dirs(&tmp);
        let also = tmp.path().join("also-node");
        fs::create_dir(&also).expect("dir");
        fs::write(also.join("node"), "").expect("node");

        assert_eq!(
            dir_holding(&joined(&[&with, &also]), "node").as_deref(),
            Some(with.as_path()),
            "the first directory"
        );
        assert_eq!(
            dir_holding(&joined(&[&without, &also]), "node").as_deref(),
            Some(also.as_path()),
            "a later directory"
        );
        assert_eq!(dir_holding(&joined(&[&without]), "node"), None);
        assert_eq!(dir_holding(OsStr::new(""), "node"), None);
    }

    #[test]
    fn stage_env_is_exactly_the_constructed_set() {
        let tmp = TempDir::new().expect("tempdir");
        let (with, without) = node_dirs(&tmp);
        let home = OsStr::new("/dev-home");
        let verb_path = joined(&[&without, &with]);
        let area = RunArea::under(Path::new("/repo"));

        let mut path = OsString::from("/dev-home/.cargo/bin:");
        path.push(&with);
        path.push(":/usr/local/bin:/usr/bin:/bin");
        let mut expected = BTreeMap::from([
            (OsString::from("HOME"), OsString::from("/dev-home")),
            (OsString::from("PATH"), path),
            (
                OsString::from("ANDROMEDA_PULSE_DATA_DIR"),
                area.data.clone().into_os_string(),
            ),
        ]);
        let plain: BTreeMap<_, _> = stage_env(home, &verb_path, &area, false)
            .into_iter()
            .collect();
        assert_eq!(plain, expected);

        expected.insert(
            OsString::from("PUPPETEER_CACHE_DIR"),
            area.puppeteer.clone().into_os_string(),
        );
        let npm: BTreeMap<_, _> = stage_env(home, &verb_path, &area, true)
            .into_iter()
            .collect();
        assert_eq!(npm, expected);

        for name in SESSION_VARIABLES {
            for env in [&plain, &npm] {
                assert!(!env.contains_key(OsStr::new(name)), "{name}");
            }
        }
    }

    #[test]
    fn stage_path_leaves_the_node_directory_out_when_the_verb_has_none() {
        let tmp = TempDir::new().expect("tempdir");
        let (_with, without) = node_dirs(&tmp);
        assert_eq!(
            stage_path(OsStr::new("/dev-home"), &joined(&[&without])),
            OsString::from("/dev-home/.cargo/bin:/usr/local/bin:/usr/bin:/bin")
        );
    }

    #[test]
    fn the_per_run_paths_sit_under_the_run_dir() {
        let area = RunArea::under(Path::new("/repo"));
        let run = Path::new("/repo/target/pre-push/run");
        assert_eq!(area.dir, run);
        assert_eq!(area.data, run.join("data"));
        assert_eq!(area.puppeteer, run.join("puppeteer"));
        assert_eq!(area.index, run.join("index"));
    }

    #[test]
    fn reset_empties_the_per_run_area_and_nothing_beside_it() {
        let tmp = TempDir::new().expect("tempdir");
        let area = RunArea::under(tmp.path());
        let beside = tmp.path().join("target/pre-push/report.json");
        fs::create_dir_all(&area.puppeteer).expect("dir");
        fs::write(area.puppeteer.join("stale"), "x").expect("stale");
        fs::write(&area.index, "x").expect("index");
        fs::write(&beside, "kept").expect("beside");

        area.reset().expect("reset");

        assert!(area.data.is_dir() && area.puppeteer.is_dir());
        assert!(!area.puppeteer.join("stale").exists());
        assert!(!area.index.exists());
        assert_eq!(fs::read_to_string(&beside).expect("beside"), "kept");
    }

    #[test]
    fn six_stages_in_the_registered_order() {
        assert_eq!(
            Stage::ALL.map(Stage::name),
            [
                "script-modes",
                "source-lint",
                "npm",
                "clippy",
                "test",
                "ci-gates"
            ]
        );
    }

    fn passed_cycle() -> CycleReading {
        CycleReading {
            cycle_exit: Some(0),
            gates_passed: Some(true),
            ..CycleReading::default()
        }
    }

    fn stage_six_red() -> Result<(), Stop> {
        Err(Stop::red("stage-failed:ci-gates"))
    }

    #[test]
    fn a_passed_cycle_with_passing_gates_and_nothing_left_running_is_green() {
        assert_eq!(decide_ci_gates(passed_cycle()), Ok(()));
    }

    #[test]
    fn a_taken_cycle_port_cannot_be_evaluated_and_never_reads_green() {
        for reading in [
            CycleReading {
                port_taken: true,
                ..CycleReading::default()
            },
            CycleReading {
                port_taken: true,
                ..passed_cycle()
            },
        ] {
            let stop = decide_ci_gates(reading).expect_err("a taken port stops the run");
            assert_eq!(stop, Stop::cannot("engine-port-taken"));
            assert_eq!(stop.verdict, Verdict::CannotEvaluate);
        }
    }

    #[test]
    fn a_cycle_that_could_not_be_evaluated_stops_as_cannot_evaluate() {
        for gates_passed in [None, Some(true)] {
            let reading = CycleReading {
                cycle_exit: Some(2),
                gates_passed,
                ..CycleReading::default()
            };
            assert_eq!(
                decide_ci_gates(reading),
                Err(Stop::cannot("engine-cycle-unevaluated"))
            );
        }
    }

    #[test]
    fn a_failed_cycle_is_red() {
        for exit in [1, -1, 101] {
            let reading = CycleReading {
                cycle_exit: Some(exit),
                ..CycleReading::default()
            };
            assert_eq!(decide_ci_gates(reading), stage_six_red(), "exit {exit}");
        }
        assert_eq!(
            decide_ci_gates(CycleReading::default()),
            stage_six_red(),
            "a cycle that was never run"
        );
    }

    #[test]
    fn a_passed_cycle_whose_gates_failed_or_did_not_run_is_red() {
        for gates_passed in [Some(false), None] {
            let reading = CycleReading {
                gates_passed,
                ..passed_cycle()
            };
            assert_eq!(
                decide_ci_gates(reading),
                stage_six_red(),
                "{gates_passed:?}"
            );
        }
    }

    #[test]
    fn a_cycle_that_left_a_port_accepting_or_its_engine_alive_is_red_whatever_it_returned() {
        for exit in [0, 1, 2] {
            for (port_accepting_after, pid_alive_after) in
                [(true, false), (false, true), (true, true)]
            {
                let reading = CycleReading {
                    cycle_exit: Some(exit),
                    port_accepting_after,
                    pid_alive_after,
                    ..passed_cycle()
                };
                assert_eq!(
                    decide_ci_gates(reading),
                    stage_six_red(),
                    "exit {exit}, port {port_accepting_after}, pid {pid_alive_after}"
                );
            }
        }
    }

    #[test]
    fn the_stage_s_cycle_ports_are_the_harness_only_pair_and_never_the_shared_ones() {
        assert_eq!((DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT), (24317, 24318));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn end_cycle_ends_the_process_the_stage_data_dir_s_pid_file_names() {
        let tmp = TempDir::new().expect("tempdir");
        let area = RunArea::under(tmp.path());
        let run = area.data.join("run");
        fs::create_dir_all(&run).expect("run dir");
        // Started through a shell that exits, so the process is no child of
        // this test and its end needs no reaping here.
        let started =
            out(Command::new("bash")
                .args(["-c", "sleep 600 < /dev/null > /dev/null 2>&1 & echo $!"]))
            .expect("`bash` MUST be on PATH: the teardown is the bash script's");
        let pid: u32 = started.trim().parse().expect("a pid");
        fs::write(run.join("andromeda-pulse.pid"), format!("{pid}\n")).expect("pid file");
        assert_eq!(pid_alive(pid), Some(true), "the process runs");

        let root = workspace_root();
        let home = std::env::var_os("HOME").unwrap_or_default();
        let verb_path = std::env::var_os("PATH").unwrap_or_default();
        end_cycle(&Host {
            root: &root,
            area: &area,
            plain: stage_env(&home, &verb_path, &area, false),
            npm: Vec::new(),
        });

        let ended = pid_alive(pid);
        let _ = Command::new("kill")
            .args(["-KILL", &pid.to_string()])
            .stderr(Stdio::null())
            .status();
        assert_eq!(
            ended,
            Some(false),
            "the process the pid file named is ended"
        );
        assert!(
            !run.join("andromeda-pulse.pid").exists(),
            "the pid file is gone"
        );
    }

    #[test]
    fn restoring_puts_the_file_back_however_the_action_ended() {
        let tmp = TempDir::new().expect("tempdir");
        let file = tmp.path().join("index.ts");
        fs::write(&file, "as found").expect("file");
        for ended in [true, false] {
            let got = restoring(&file, || {
                fs::write(&file, "rewritten").expect("rewrite");
                ended
            });
            assert_eq!(got, Ok(ended));
            assert_eq!(fs::read_to_string(&file).expect("file"), "as found");
        }
    }

    #[test]
    fn restoring_removes_a_file_that_was_absent_before() {
        let tmp = TempDir::new().expect("tempdir");
        let file = tmp.path().join("index.ts");
        let got = restoring(&file, || {
            fs::write(&file, "rewritten").expect("rewrite");
            true
        });
        assert_eq!(got, Ok(true));
        assert!(!file.exists());
    }

    #[test]
    fn a_restore_that_cannot_write_is_a_failure() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().join("bindings");
        fs::create_dir(&dir).expect("dir");
        let file = dir.join("index.ts");
        fs::write(&file, "as found").expect("file");
        let got = restoring(&file, || {
            fs::remove_dir_all(&dir).expect("remove");
            true
        });
        assert_eq!(got, Err(RestoreFailed));
    }

    #[test]
    fn the_verdict_document_has_exactly_six_members() {
        let doc = document(Verdict::Green, "all-stages-ok", &Doc::default());
        let members: BTreeSet<&str> = doc
            .as_object()
            .expect("an object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            members,
            BTreeSet::from(["head", "missing", "reason", "stages", "tree", "verdict"])
        );
    }
}
