//! `harness:engine-cycle` — one cycle of the harness verbs on the console
//! engine, on a data dir and ports of its own (test-plan §3, 5-command
//! implementation).
//!
//! In order: build the injector, `agent-run.sh boot engine`, a healthy feed
//! over OTLP by the injector run by path, `harness:engine-settled`, the
//! injector ended by its own child handle, `agent-run.sh status engine`,
//! `agent-run.sh cleanup`, `check:engine-log`. Cleanup runs whatever the
//! verbs before it returned; after a failed boot the two verbs between are
//! skipped.
//!
//! It refuses, as `cannot-evaluate` and before anything is started: a host
//! that is not Linux, port 4317 or 4318 (shared on a dev host), a data dir
//! that already holds a log family (a gap read across two boots would be
//! false).
//!
//! Every child gets a cleared environment plus the set `child_env` builds.
//! The session bus address, the runtime dir and the display variables are
//! not in it, so no child reaches an OS credential store; the corpus key
//! comes from a passphrase made for the run, never printed or written.
//!
//! One pretty-JSON verdict on stdout (`pass`, `fail`, `cannot-evaluate`),
//! exit 0 / 1 / 2. The children's own output goes to stderr, so stdout
//! carries the verdict alone. No member of the verdict holds a path or an
//! environment value.

use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};

use anyhow::Result;
use serde_json::{Value, json};

use crate::harness_witness::WITNESS_FILE;
use crate::perf_budget::{family_members, read_family};

pub(crate) const DEFAULT_GRPC_PORT: u16 = 24317;
pub(crate) const DEFAULT_HTTP_PORT: u16 = 24318;
/// The receivers' default ports, which other programs on a dev host hold.
const SHARED_PORTS: [u16; 2] = [4317, 4318];

const SCRIPT: &str = "scripts/agent-run.sh";
const INJECTOR: &str = "target/debug/examples/inject_demo";
/// The registered minimum: the first retention sweep, which sets the memory
/// gauge, then runs 10 s after boot instead of 100 s.
const RETENTION_SECONDS: &str = "60";
const WITNESS_LIB_VAR: &str = "ANDROMEDA_PULSE_EXIT_WITNESS_LIB";

/// One variable of a child's environment.
type Var = (OsString, OsString);

pub(crate) struct Options {
    pub(crate) data_dir: Option<PathBuf>,
    pub(crate) grpc_port: u16,
    pub(crate) http_port: u16,
}

/// Each verb's exit; `None` for a verb that did not run.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Exits {
    boot: Option<i32>,
    settled: Option<i32>,
    status: Option<i32>,
    cleanup: Option<i32>,
    check: Option<i32>,
}

#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    exits: Exits,
    /// ERROR records in the cycle's log family; `None` when no cycle ran.
    error_records: Option<usize>,
    witness_file: &'static str,
}

impl Outcome {
    fn not_run() -> Self {
        Self {
            exits: Exits::default(),
            error_records: None,
            witness_file: "absent",
        }
    }
}

pub(crate) fn run(options: &Options) -> Result<ExitCode> {
    let root = workspace_root();
    let outcome = match drive(&root, options) {
        Ok(outcome) => outcome,
        Err(reason) => {
            eprintln!("harness:engine-cycle: cannot-evaluate ({reason})");
            Outcome::not_run()
        }
    };
    let verdict = decide(&outcome);
    println!(
        "{}",
        serde_json::to_string_pretty(&payload(verdict, &outcome))?
    );
    Ok(ExitCode::from(exit_status(verdict)))
}

/// Why the cycle cannot be evaluated, as one closed label.
fn refusal(linux: bool, grpc_port: u16, http_port: u16, holds_log: bool) -> Option<&'static str> {
    if !linux {
        Some("not-linux")
    } else if [grpc_port, http_port]
        .iter()
        .any(|port| SHARED_PORTS.contains(port))
    {
        Some("shared-port")
    } else if holds_log {
        Some("data-dir-holds-a-log-family")
    } else {
        None
    }
}

fn holds_log_family(data_dir: &Path) -> bool {
    !family_members(&data_dir.join("logs")).is_empty()
}

/// The exact variable set a child receives on top of a cleared environment.
/// The witness variable is passed on only when this verb's own environment
/// holds it, so the run shows `boot engine` leaving the library unloaded.
fn child_env(
    home: &OsStr,
    path: &OsStr,
    data_dir: &Path,
    ports: (u16, u16),
    passphrase: &str,
    witness_lib: Option<&OsStr>,
) -> Vec<Var> {
    let fixed: [(&str, OsString); 7] = [
        ("HOME", home.to_os_string()),
        ("PATH", path.to_os_string()),
        ("ANDROMEDA_PULSE_DATA_DIR", data_dir.into()),
        ("ANDROMEDA_PULSE_OTLP_GRPC_PORT", ports.0.to_string().into()),
        ("ANDROMEDA_PULSE_OTLP_HTTP_PORT", ports.1.to_string().into()),
        (
            "ANDROMEDA_PULSE_RETENTION_SECONDS",
            RETENTION_SECONDS.into(),
        ),
        ("ANDROMEDA_PULSE_CORPUS_PASSPHRASE", passphrase.into()),
    ];
    let mut env: Vec<Var> = fixed
        .into_iter()
        .map(|(name, value)| (OsString::from(name), value))
        .collect();
    if let Some(lib) = witness_lib {
        env.push((OsString::from(WITNESS_LIB_VAR), lib.to_os_string()));
    }
    env
}

/// The verdict core, pure so the arms are pinnable without a process. A verb
/// that failed outranks one that could not evaluate (exit 2) or did not run.
fn decide(outcome: &Outcome) -> &'static str {
    let exits = [
        outcome.exits.boot,
        outcome.exits.settled,
        outcome.exits.status,
        outcome.exits.cleanup,
        outcome.exits.check,
    ];
    let failed = exits.iter().flatten().any(|code| !matches!(code, 0 | 2))
        || outcome.error_records.is_some_and(|count| count > 0)
        || outcome.witness_file == "present";
    let unevaluated =
        exits.iter().any(|code| matches!(code, None | Some(2))) || outcome.error_records.is_none();
    if failed {
        "fail"
    } else if unevaluated {
        "cannot-evaluate"
    } else {
        "pass"
    }
}

fn exit_status(verdict: &str) -> u8 {
    match verdict {
        "pass" => 0,
        "cannot-evaluate" => 2,
        _ => 1,
    }
}

fn payload(verdict: &str, outcome: &Outcome) -> Value {
    json!({
        "verdict": verdict,
        "boot": outcome.exits.boot,
        "settled": outcome.exits.settled,
        "status": outcome.exits.status,
        "cleanup": outcome.exits.cleanup,
        "check": outcome.exits.check,
        "error_records": outcome.error_records,
        "witness_file": outcome.witness_file,
    })
}

fn default_data_dir_name(now: chrono::DateTime<chrono::Utc>) -> String {
    now.format("%Y%m%dT%H%M%SZ").to_string()
}

/// A value for the corpus passphrase variable, made for this run.
fn run_passphrase() -> String {
    use std::hash::{BuildHasher, Hasher};
    let word = || {
        std::collections::hash_map::RandomState::new()
            .build_hasher()
            .finish()
    };
    format!("{:016x}{:016x}", word(), word())
}

fn drive(root: &Path, options: &Options) -> Result<Outcome, &'static str> {
    let data_dir = match &options.data_dir {
        Some(dir) => std::path::absolute(dir).map_err(|_| "data-dir-unusable")?,
        None => root
            .join("target")
            .join("engine-cycle")
            .join(default_data_dir_name(chrono::Utc::now())),
    };
    let refused = refusal(
        cfg!(target_os = "linux"),
        options.grpc_port,
        options.http_port,
        holds_log_family(&data_dir),
    );
    if let Some(reason) = refused {
        return Err(reason);
    }
    let home = std::env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .ok_or("home-unset")?;
    let path = std::env::var_os("PATH").ok_or("path-unset")?;
    std::fs::create_dir_all(data_dir.join("logs")).map_err(|_| "data-dir-unusable")?;
    let data_dir = std::fs::canonicalize(&data_dir).map_err(|_| "data-dir-unusable")?;
    let witness_lib = std::env::var_os(WITNESS_LIB_VAR);
    let env = child_env(
        &home,
        &path,
        &data_dir,
        (options.grpc_port, options.http_port),
        &run_passphrase(),
        witness_lib.as_deref(),
    );
    let run = |program: &str, args: &[&str]| exit_of(&mut command(&env, root, program, args));

    // Built before anything is started, so the feed is not compiled inside
    // the run it feeds.
    if run(
        "cargo",
        &["build", "-p", "ingest", "--example", "inject_demo"],
    ) != 0
    {
        return Err("injector-build-failed");
    }

    let boot = run("bash", &[SCRIPT, "boot", "engine"]);
    let (settled, status) = if boot == 0 {
        let mut injector = spawn_injector(&env, root);
        if injector.is_none() {
            eprintln!("harness:engine-cycle: the injector did not start");
        }
        let settled = run("cargo", &["xtask", "harness:engine-settled"]);
        if let Some(child) = injector.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
        let status = run("bash", &[SCRIPT, "status", "engine"]);
        (Some(settled), Some(status))
    } else {
        (None, None)
    };
    let cleanup = run("bash", &[SCRIPT, "cleanup"]);
    let check = run("cargo", &["xtask", "check:engine-log"]);

    let logs = data_dir.join("logs");
    let error_records = read_family(&logs)
        .unwrap_or_default()
        .iter()
        .filter(|record| record.get("level").and_then(Value::as_str) == Some("ERROR"))
        .count();
    Ok(Outcome {
        exits: Exits {
            boot: Some(boot),
            settled,
            status,
            cleanup: Some(cleanup),
            check: Some(check),
        },
        error_records: Some(error_records),
        witness_file: if logs.join(WITNESS_FILE).exists() {
            "present"
        } else {
            "absent"
        },
    })
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

/// The child's exit code; -1 for one that could not start or was ended by a
/// signal, so neither reads as a pass.
fn exit_of(cmd: &mut Command) -> i32 {
    cmd.status()
        .ok()
        .and_then(|status| status.code())
        .unwrap_or(-1)
}

fn spawn_injector(env: &[Var], root: &Path) -> Option<Child> {
    let mut cmd = command(
        env,
        root,
        &root.join(INJECTOR).to_string_lossy(),
        &["--sustained", "--error-pct=0"],
    );
    cmd.stdout(Stdio::null()).spawn().ok()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    const SESSION_VARIABLES: [&str; 5] = [
        "DBUS_SESSION_BUS_ADDRESS",
        "XDG_RUNTIME_DIR",
        "DISPLAY",
        "WAYLAND_DISPLAY",
        "LD_PRELOAD",
    ];

    #[test]
    fn a_host_that_is_not_linux_is_refused() {
        assert_eq!(
            refusal(false, DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT, false),
            Some("not-linux")
        );
    }

    #[test]
    fn a_shared_port_in_either_place_is_refused() {
        for (grpc, http) in [
            (4317, DEFAULT_HTTP_PORT),
            (4318, DEFAULT_HTTP_PORT),
            (DEFAULT_GRPC_PORT, 4317),
            (DEFAULT_GRPC_PORT, 4318),
            (4317, 4318),
        ] {
            assert_eq!(
                refusal(true, grpc, http, false),
                Some("shared-port"),
                "{grpc} / {http}"
            );
        }
    }

    #[test]
    fn a_data_dir_that_already_holds_a_log_family_is_refused() {
        assert_eq!(
            refusal(true, DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT, true),
            Some("data-dir-holds-a-log-family")
        );
    }

    #[test]
    fn the_default_ports_on_linux_over_a_fresh_data_dir_are_not_refused() {
        assert_eq!(
            refusal(true, DEFAULT_GRPC_PORT, DEFAULT_HTTP_PORT, false),
            None
        );
        assert_eq!(refusal(true, 14317, 14318, false), None);
        assert!(!SHARED_PORTS.contains(&DEFAULT_GRPC_PORT));
        assert!(!SHARED_PORTS.contains(&DEFAULT_HTTP_PORT));
    }

    #[test]
    fn a_log_family_is_read_under_the_data_dir_s_logs_alone() {
        let dir = tempfile::TempDir::new().expect("tmp");
        assert!(!holds_log_family(dir.path()), "no logs dir");
        let logs = dir.path().join("logs");
        std::fs::create_dir_all(&logs).expect("mkdir");
        std::fs::write(logs.join("boot.log"), "x\n").expect("write");
        std::fs::write(dir.path().join("agent-latest.jsonl"), "x\n").expect("write");
        assert!(!holds_log_family(dir.path()), "no member under logs");
        std::fs::write(logs.join("agent-latest.jsonl.2026-10-10"), "").expect("write");
        assert!(
            holds_log_family(dir.path()),
            "an empty member is a family too"
        );
    }

    fn env_of(witness_lib: Option<&OsStr>) -> BTreeMap<OsString, OsString> {
        child_env(
            OsStr::new("/dev-home"),
            OsStr::new("/dev-home/.cargo/bin:/usr/bin"),
            Path::new("/repo/target/engine-cycle/run"),
            (24317, 24318),
            "made-for-the-run",
            witness_lib,
        )
        .into_iter()
        .collect()
    }

    fn fixed_set() -> BTreeMap<OsString, OsString> {
        [
            ("HOME", "/dev-home"),
            ("PATH", "/dev-home/.cargo/bin:/usr/bin"),
            ("ANDROMEDA_PULSE_DATA_DIR", "/repo/target/engine-cycle/run"),
            ("ANDROMEDA_PULSE_OTLP_GRPC_PORT", "24317"),
            ("ANDROMEDA_PULSE_OTLP_HTTP_PORT", "24318"),
            ("ANDROMEDA_PULSE_RETENTION_SECONDS", "60"),
            ("ANDROMEDA_PULSE_CORPUS_PASSPHRASE", "made-for-the-run"),
        ]
        .into_iter()
        .map(|(name, value)| (OsString::from(name), OsString::from(value)))
        .collect()
    }

    #[test]
    fn the_child_environment_is_exactly_the_fixed_set() {
        let env = env_of(None);
        assert_eq!(env, fixed_set());
        for name in SESSION_VARIABLES {
            assert!(!env.contains_key(OsStr::new(name)), "{name}");
        }
    }

    #[test]
    fn the_witness_variable_is_passed_on_only_when_the_verb_s_own_environment_holds_it() {
        let mut expected = fixed_set();
        expected.insert(
            OsString::from(WITNESS_LIB_VAR),
            OsString::from("/elsewhere/exit-witness.so"),
        );
        let env = env_of(Some(OsStr::new("/elsewhere/exit-witness.so")));
        assert_eq!(env, expected);
        for name in SESSION_VARIABLES {
            assert!(!env.contains_key(OsStr::new(name)), "{name}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_child_sees_the_constructed_set_and_nothing_of_this_process() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let env: Vec<Var> = fixed_set().into_iter().collect();
        let output = command(&env, dir.path(), "/usr/bin/env", &[])
            .stdout(Stdio::piped())
            .output()
            .expect("run env");
        let seen: BTreeSet<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| line.split_once('=').map(|(name, _)| name.to_owned()))
            .collect();
        let expected: BTreeSet<String> = fixed_set()
            .keys()
            .map(|name| name.to_string_lossy().into_owned())
            .collect();
        assert_eq!(seen, expected);
    }

    fn ran(exits: [Option<i32>; 5], error_records: usize, witness_file: &'static str) -> Outcome {
        Outcome {
            exits: Exits {
                boot: exits[0],
                settled: exits[1],
                status: exits[2],
                cleanup: exits[3],
                check: exits[4],
            },
            error_records: Some(error_records),
            witness_file,
        }
    }

    const ALL_ZERO: [Option<i32>; 5] = [Some(0); 5];

    #[test]
    fn five_zero_exits_no_error_record_and_no_witness_file_pass() {
        assert_eq!(decide(&ran(ALL_ZERO, 0, "absent")), "pass");
    }

    #[test]
    fn any_verb_that_exits_one_fails() {
        for idx in 0..5 {
            let mut exits = ALL_ZERO;
            exits[idx] = Some(1);
            assert_eq!(decide(&ran(exits, 0, "absent")), "fail", "verb {idx}");
            exits[idx] = Some(-1);
            assert_eq!(decide(&ran(exits, 0, "absent")), "fail", "verb {idx}");
        }
    }

    #[test]
    fn an_error_record_fails() {
        assert_eq!(decide(&ran(ALL_ZERO, 1, "absent")), "fail");
    }

    #[test]
    fn a_witness_file_fails() {
        assert_eq!(decide(&ran(ALL_ZERO, 0, "present")), "fail");
    }

    #[test]
    fn a_failed_boot_fails_with_the_verbs_between_skipped() {
        let outcome = ran([Some(1), None, None, Some(0), Some(2)], 0, "absent");
        assert_eq!(decide(&outcome), "fail");
    }

    #[test]
    fn a_verb_that_could_not_evaluate_is_cannot_evaluate_unless_another_failed() {
        for idx in [1, 2, 4] {
            let mut exits = ALL_ZERO;
            exits[idx] = Some(2);
            assert_eq!(
                decide(&ran(exits, 0, "absent")),
                "cannot-evaluate",
                "verb {idx}"
            );
            assert_eq!(decide(&ran(exits, 1, "absent")), "fail", "verb {idx}");
            assert_eq!(decide(&ran(exits, 0, "present")), "fail", "verb {idx}");
        }
        assert_eq!(
            decide(&ran(
                [Some(0), Some(2), Some(1), Some(0), Some(0)],
                0,
                "absent"
            )),
            "fail",
            "a fail outranks a cannot-evaluate"
        );
    }

    #[test]
    fn a_cycle_that_did_not_run_cannot_be_evaluated() {
        assert_eq!(decide(&Outcome::not_run()), "cannot-evaluate");
        let skipped = ran([Some(0), None, None, Some(0), Some(0)], 0, "absent");
        assert_eq!(decide(&skipped), "cannot-evaluate", "a skipped verb");
    }

    #[test]
    fn the_exit_follows_the_verdict() {
        assert_eq!(exit_status("pass"), 0);
        assert_eq!(exit_status("fail"), 1);
        assert_eq!(exit_status("cannot-evaluate"), 2);
    }

    #[test]
    fn the_verdict_carries_the_closed_member_set() {
        let outcome = ran([Some(0), Some(1), None, Some(0), Some(2)], 3, "present");
        let value = payload("fail", &outcome);
        let keys: BTreeSet<&str> = value
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            BTreeSet::from([
                "verdict",
                "boot",
                "settled",
                "status",
                "cleanup",
                "check",
                "error_records",
                "witness_file",
            ])
        );
        assert_eq!(value["verdict"], "fail");
        assert_eq!(value["boot"], 0);
        assert_eq!(value["settled"], 1);
        assert!(value["status"].is_null(), "a skipped verb is null");
        assert_eq!(value["check"], 2);
        assert_eq!(value["error_records"], 3);
        assert_eq!(value["witness_file"], "present");
    }

    #[test]
    fn a_cycle_that_did_not_run_reports_nulls_and_no_witness_file() {
        let value = payload("cannot-evaluate", &Outcome::not_run());
        for verb in [
            "boot",
            "settled",
            "status",
            "cleanup",
            "check",
            "error_records",
        ] {
            assert!(value[verb].is_null(), "{verb}");
        }
        assert_eq!(value["witness_file"], "absent");
    }

    #[test]
    fn the_verdict_text_is_what_the_gate_entry_reads() {
        let text = serde_json::to_string_pretty(&payload("pass", &ran(ALL_ZERO, 0, "absent")))
            .expect("json");
        for atom in [
            r#""verdict": "pass""#,
            r#""witness_file": "absent""#,
            r#""error_records": 0"#,
        ] {
            assert!(text.contains(atom), "`{atom}` in:\n{text}");
        }
        assert!(
            !text.contains('/') && !text.contains("made-for-the-run"),
            "no path and no environment value:\n{text}"
        );
    }

    #[test]
    fn the_default_data_dir_is_named_by_the_utc_second() {
        let at = chrono::DateTime::from_timestamp(1_791_645_846, 0).expect("in range");
        assert_eq!(default_data_dir_name(at), "20261010T152406Z");
    }

    #[test]
    fn the_passphrase_is_made_for_each_run() {
        let (first, second) = (run_passphrase(), run_passphrase());
        assert_eq!(first.len(), 32);
        assert!(first.bytes().all(|b| b.is_ascii_hexdigit()), "hex only");
        assert_ne!(first, second);
    }
}
