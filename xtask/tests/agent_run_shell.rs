//! Shell-level pins of `scripts/agent-run.sh`: the committed script run by
//! `bash` in a scratch root, against the engine built beside this test
//! binary.
//!
//! A stand-in `cargo` on the script's PATH skips the pre-build (or fails it)
//! and hands `cargo xtask` to the built xtask, so readiness and status are
//! the real verbs' answers. The two port tokens of `cleanup` are read over a
//! listener the test holds, and `app-survived` over a child the test has not
//! reaped, since no real process outlives SIGKILL. Every run has its own data
//! dir, ports, home and temp dir, and an engine a run started is ended before
//! the run returns.

#![cfg(unix)]

mod common;

use std::io::{BufRead, BufReader};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

use common::{Cargo, Ran, Scratch};

const ENGINE_WAIT: Duration = Duration::from_secs(60);
const LINE_WAIT: Duration = Duration::from_secs(30);
const REFUSING_LINE: &str = "the receivers never both accepted: OTLP gRPC 127.0.0.1:";
const FOREIGN_LINE: &str =
    "a receiver port accepts but this run's log holds no bound record for it: OTLP gRPC 127.0.0.1:";

/// A scratch area whose root holds the built engine and whose `cargo` skips
/// the pre-build.
fn engine_scratch() -> Scratch {
    let scratch = Scratch::new();
    scratch.link_engine();
    scratch.stand_in_cargo(Cargo::SkipsBuilds);
    scratch
}

fn boot_engine(scratch: &Scratch) -> Ran {
    scratch.run(&mut scratch.script(&["boot", "engine"]))
}

/// Boots the engine and returns its pid once the verb has read ready.
fn booted(scratch: &Scratch) -> u32 {
    let boot = boot_engine(scratch);
    assert_eq!(boot.code, Some(0), "{}", scratch.printed(&boot));
    assert!(
        boot.stdout.contains("boot: ready (PID="),
        "{}",
        scratch.printed(&boot)
    );
    let pid = scratch.spawned_pid().expect("the spawn record names a pid");
    assert!(common::is_engine(pid), "the spawned pid is the engine");
    pid
}

fn cleanup(scratch: &Scratch) -> Ran {
    scratch.run(&mut scratch.script(&["cleanup"]))
}

fn assert_cleanup(scratch: &Scratch, token: &str, code: i32) {
    let ran = cleanup(scratch);
    assert_eq!(
        ran.stdout,
        format!("cleanup: {token}\n"),
        "{}",
        scratch.printed(&ran)
    );
    assert_eq!(ran.code, Some(code), "{}", scratch.printed(&ran));
}

fn write_pidfile(scratch: &Scratch, pid: u32) {
    let pidfile = scratch.run_file("andromeda-pulse.pid");
    std::fs::create_dir_all(pidfile.parent().expect("run dir")).expect("create the run dir");
    std::fs::write(pidfile, format!("{pid}\n")).expect("write the pidfile");
}

/// A never-ready boot: exit 1, the timeout line, and the self-run cleanup's
/// token as the verb's last stdout line.
fn assert_never_ready(scratch: &Scratch, boot: &Ran, cleanup_token: &str) {
    assert_eq!(boot.code, Some(1), "{}", scratch.printed(boot));
    assert!(
        boot.stderr
            .contains("boot: failed to reach ready state within"),
        "{}",
        scratch.printed(boot)
    );
    assert_eq!(
        boot.stdout.lines().last(),
        Some(format!("cleanup: {cleanup_token}").as_str()),
        "{}",
        scratch.printed(boot)
    );
    assert!(
        !boot.stdout.contains("boot: ready"),
        "{}",
        scratch.printed(boot)
    );
    let pid = scratch.spawned_pid().expect("the spawn record names a pid");
    assert!(
        common::wait_until(ENGINE_WAIT, || common::pid_ended(pid)),
        "the engine the boot started is ended"
    );
}

#[test]
fn cleanup_with_no_pidfile_and_both_ports_refusing_is_clean_twice_over() {
    let scratch = Scratch::new();
    assert_cleanup(&scratch, "clean", 0);
    assert_cleanup(&scratch, "clean", 0);
}

#[test]
fn cleanup_with_no_pidfile_and_a_port_accepting_is_no_pid_ports_accepting() {
    let scratch = Scratch::new();
    let _listener = common::listen(scratch.grpc);
    assert_cleanup(&scratch, "no-pid-ports-accepting", 1);
    assert!(
        common::accepts(scratch.grpc),
        "the listener is left as it was"
    );
}

#[test]
fn cleanup_with_a_dead_pid_and_a_port_accepting_is_ports_lingering() {
    let scratch = Scratch::new();
    let mut child = Command::new(common::real_bash())
        .args(["-c", "exit 0"])
        .spawn()
        .expect("spawn a child");
    let pid = child.id();
    child.wait().expect("reap the child");
    assert!(common::pid_ended(pid), "a reaped child is gone");
    write_pidfile(&scratch, pid);
    let _listener = common::listen(scratch.http);

    assert_cleanup(&scratch, "ports-lingering", 1);
    assert!(
        !scratch.run_file("andromeda-pulse.pid").exists(),
        "the pidfile is removed"
    );
}

#[test]
fn cleanup_with_an_unreaped_child_in_the_pidfile_is_app_survived() {
    let scratch = Scratch::new();
    let mut child = Command::new(common::real_bash())
        .args(["-c", "exit 0"])
        .spawn()
        .expect("spawn a child");
    let pid = child.id();
    assert!(
        common::wait_until(LINE_WAIT, || common::pid_state(pid) == Some('Z')),
        "the child has exited and is not reaped"
    );
    write_pidfile(&scratch, pid);

    assert_cleanup(&scratch, "app-survived", 1);
    child.wait().expect("reap the child");
}

#[test]
fn boot_engine_reads_ready_and_cleanup_ends_the_engine_clean() {
    let scratch = engine_scratch();
    let pid = booted(&scratch);
    assert!(
        common::accepts(scratch.grpc) && common::accepts(scratch.http),
        "both receivers accept once the verb read ready"
    );
    assert_eq!(
        common::read_pid(&scratch.run_file("andromeda-pulse.pid")),
        Some(pid),
        "the pidfile names the engine"
    );

    assert_cleanup(&scratch, "clean", 0);
    assert!(common::pid_ended(pid), "the engine is ended");
    assert!(
        !scratch.run_file("andromeda-pulse.pid").exists(),
        "the pidfile is gone"
    );
    assert!(
        !common::accepts(scratch.grpc) && !common::accepts(scratch.http),
        "both ports refuse"
    );
    let exit_record = scratch.run_file("andromeda-pulse.exit");
    assert!(
        common::wait_until(LINE_WAIT, || {
            std::fs::read_to_string(&exit_record).is_ok_and(|end| end == "signal 15 (TERM)\n")
        }),
        "the waiting wrapper recorded the end by SIGTERM"
    );
}

#[test]
fn boot_fails_when_the_engine_build_fails() {
    let scratch = Scratch::new();
    scratch.link_engine();
    scratch.stand_in_cargo(Cargo::FailsBuildOf(common::ENGINE_NAME));
    let boot = boot_engine(&scratch);
    assert_eq!(boot.code, Some(1), "{}", scratch.printed(&boot));
    assert!(
        boot.stderr
            .contains("boot: cargo build --bin andromeda-pulse-engine --release failed"),
        "{}",
        scratch.printed(&boot)
    );
    assert_eq!(scratch.spawned_pid(), None, "nothing was spawned");
    assert_eq!(scratch.cargo_calls().len(), 1, "the build stopped the verb");
}

#[test]
fn boot_fails_when_the_xtask_build_fails() {
    let scratch = Scratch::new();
    scratch.link_engine();
    scratch.stand_in_cargo(Cargo::FailsBuildOf("-p xtask"));
    let boot = boot_engine(&scratch);
    assert_eq!(boot.code, Some(1), "{}", scratch.printed(&boot));
    assert!(
        boot.stderr.contains("boot: cargo build -p xtask failed"),
        "{}",
        scratch.printed(&boot)
    );
    assert_eq!(scratch.spawned_pid(), None, "nothing was spawned");
}

#[test]
fn boot_fails_when_the_built_binary_is_absent() {
    let scratch = Scratch::new();
    scratch.stand_in_cargo(Cargo::SkipsBuilds);
    let boot = boot_engine(&scratch);
    assert_eq!(boot.code, Some(1), "{}", scratch.printed(&boot));
    assert!(
        boot.stderr.contains(
            "boot: built binary not found at target/release/andromeda-pulse-engine[.exe]"
        ),
        "{}",
        scratch.printed(&boot)
    );
    assert_eq!(scratch.spawned_pid(), None, "nothing was spawned");
}

#[test]
fn boot_with_no_word_resolves_the_window_program() {
    let scratch = engine_scratch();
    let boot = scratch.run(&mut scratch.script(&["boot"]));
    assert_eq!(boot.code, Some(1), "{}", scratch.printed(&boot));
    assert!(
        boot.stderr
            .contains("boot: built binary not found at target/release/pulse-app[.exe]"),
        "{}",
        scratch.printed(&boot)
    );
    assert_eq!(
        scratch.cargo_calls().first().map(String::as_str),
        Some("build --bin pulse-app --release")
    );
}

#[test]
fn boot_with_no_spawn_record_exits_one_and_leaves_the_engine_it_started() {
    let scratch = engine_scratch();
    let absent = scratch.path("absent").join("andromeda-pulse.pid");
    let boot = scratch.run(
        scratch
            .script(&["boot", "engine"])
            .env("ANDROMEDA_PULSE_PIDFILE", &absent),
    );
    assert_eq!(boot.code, Some(1), "{}", scratch.printed(&boot));
    assert!(
        boot.stderr
            .contains("boot: the app did not start (no spawn record)"),
        "{}",
        scratch.printed(&boot)
    );
    assert!(!absent.exists(), "no harness pidfile was written");

    // The engine itself did start: its own pid file names it.
    let own = scratch.run_file("andromeda-pulse.pid");
    assert!(
        common::wait_until(ENGINE_WAIT, || common::read_pid(&own).is_some()),
        "the engine wrote its own pid file"
    );
    let pid = common::read_pid(&own).expect("a pid");
    assert!(
        common::is_engine(pid) && !common::pid_ended(pid),
        "the engine is left running"
    );
    assert!(common::signal("TERM", &pid.to_string()), "SIGTERM sent");
    assert!(
        common::wait_until(ENGINE_WAIT, || common::pid_ended(pid)),
        "the engine ends by SIGTERM"
    );
}

/// A privileged port number nothing on this host accepts on: the product
/// rejects it, so the receiver is never started.
fn refusing_privileged_port() -> u16 {
    [1, 7, 9, 13, 19]
        .into_iter()
        .find(|port| !common::accepts(*port))
        .expect("one privileged loopback port refuses")
}

#[test]
fn boot_never_ready_over_a_refusing_receiver_prints_the_receivers_line() {
    let scratch = engine_scratch();
    let port = refusing_privileged_port();
    let boot = scratch.run(
        scratch
            .script(&["boot", "engine"])
            .env("ANDROMEDA_PULSE_OTLP_GRPC_PORT", port.to_string())
            .env("HARNESS_STATUS_TIMEOUT", "6"),
    );
    assert_never_ready(&scratch, &boot, "clean");
    assert!(
        boot.stderr.contains("app still running"),
        "{}",
        scratch.printed(&boot)
    );
    assert!(
        boot.stderr.contains(&format!(
            "{REFUSING_LINE}{port}, OTLP HTTP 127.0.0.1:{}",
            scratch.http
        )),
        "{}",
        scratch.printed(&boot)
    );
    assert!(
        !boot.stderr.contains(FOREIGN_LINE),
        "{}",
        scratch.printed(&boot)
    );
}

#[test]
fn boot_never_ready_over_an_unparseable_port_prints_neither_receiver_line() {
    let scratch = engine_scratch();
    let boot = scratch.run(
        scratch
            .script(&["boot", "engine"])
            .env("ANDROMEDA_PULSE_OTLP_GRPC_PORT", "not-a-port")
            .env("HARNESS_STATUS_TIMEOUT", "6"),
    );
    assert_never_ready(&scratch, &boot, "clean");
    assert!(
        boot.stderr.contains("app still running"),
        "{}",
        scratch.printed(&boot)
    );
    assert!(
        !boot.stderr.contains(REFUSING_LINE) && !boot.stderr.contains(FOREIGN_LINE),
        "{}",
        scratch.printed(&boot)
    );
}

#[test]
fn boot_never_ready_over_another_listener_prints_the_foreign_line() {
    let scratch = engine_scratch();
    let _listener = common::listen(scratch.grpc);
    let boot = scratch.run(
        scratch
            .script(&["boot", "engine"])
            .env("HARNESS_STATUS_TIMEOUT", "6"),
    );
    // The listener outlives the boot, so the self-run cleanup reads the port
    // still accepting.
    assert_never_ready(&scratch, &boot, "ports-lingering");
    assert!(
        boot.stderr.contains("app still running"),
        "{}",
        scratch.printed(&boot)
    );
    assert!(
        boot.stderr.contains(&format!(
            "{FOREIGN_LINE}{}, OTLP HTTP 127.0.0.1:{}",
            scratch.grpc, scratch.http
        )),
        "{}",
        scratch.printed(&boot)
    );
    assert!(
        !boot.stderr.contains(REFUSING_LINE),
        "{}",
        scratch.printed(&boot)
    );
    assert!(
        common::accepts(scratch.grpc),
        "the listener is left as it was"
    );
}

#[test]
fn boot_names_how_the_engine_ended_when_it_is_killed_during_the_poll() {
    let scratch = engine_scratch();
    let child = scratch
        .script(&["boot", "engine"])
        .env("ANDROMEDA_PULSE_OTLP_GRPC_PORT", "not-a-port")
        .env("HARNESS_STATUS_TIMEOUT", "8")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the boot verb");
    assert!(
        common::wait_until(ENGINE_WAIT, || scratch.spawned_pid().is_some()),
        "the spawn record appears"
    );
    let pid = scratch.spawned_pid().expect("a pid");
    assert!(common::is_engine(pid), "the spawned pid is the engine");
    assert!(common::signal("KILL", &pid.to_string()), "SIGKILL sent");

    let boot: Ran = child.wait_with_output().expect("the boot verb ends").into();
    assert_never_ready(&scratch, &boot, "clean");
    assert!(
        boot.stderr.contains("app ended: signal 9 (KILL)"),
        "{}",
        scratch.printed(&boot)
    );
    assert!(
        !boot.stderr.contains("app still running"),
        "{}",
        scratch.printed(&boot)
    );
}

#[test]
fn status_reads_the_program_asked_for_on_a_running_engine() {
    let scratch = engine_scratch();
    booted(&scratch);

    let engine = scratch.run(&mut scratch.script(&["status", "engine"]));
    assert_eq!(engine.code, Some(0), "{}", scratch.printed(&engine));
    assert!(
        engine.stdout.contains(r#""verdict": "running-healthy""#)
            && engine.stdout.contains(r#""program": "console""#),
        "{}",
        scratch.printed(&engine)
    );

    let window = scratch.run(&mut scratch.script(&["status"]));
    assert_eq!(window.code, Some(1), "{}", scratch.printed(&window));
    assert!(
        window.stdout.contains(r#""verdict": "wrong-program""#),
        "{}",
        scratch.printed(&window)
    );

    assert_cleanup(&scratch, "clean", 0);
}

#[test]
fn any_other_word_after_boot_or_status_is_usage_exit_two_and_no_data_dir() {
    for words in [
        &["boot", "window"][..],
        &["boot", "engines"],
        &["status", "console"],
        &["status", "engine "],
    ] {
        let scratch = engine_scratch();
        let ran = scratch.run(&mut scratch.script(words));
        assert_eq!(ran.code, Some(2), "{words:?}: {}", scratch.printed(&ran));
        assert!(
            ran.stderr.contains("Usage: ")
                && ran
                    .stderr
                    .contains("{boot [engine]|run|status [engine]|cleanup|logs}"),
            "{words:?}: {}",
            scratch.printed(&ran)
        );
        assert!(ran.stdout.is_empty(), "{words:?}: nothing on stdout");
        assert!(!scratch.data().exists(), "{words:?}: no data dir is made");
        assert!(scratch.cargo_calls().is_empty(), "{words:?}: nothing built");
    }
}

/// The `logs` verb in a process group of its own, its stdout read line by
/// line.
struct Tail {
    child: std::process::Child,
    lines: mpsc::Receiver<String>,
}

impl Tail {
    fn spawn(command: &mut Command) -> Self {
        let mut child = command
            .process_group(0)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn the logs verb");
        let stdout = child.stdout.take().expect("piped stdout");
        let (sender, lines) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                if sender.send(line).is_err() {
                    break;
                }
            }
        });
        Self { child, lines }
    }

    fn first_line(&self) -> String {
        self.lines
            .recv_timeout(LINE_WAIT)
            .expect("the logs verb printed a line")
    }

    /// Ends the verb and the `tail` under it by their process group.
    fn end(mut self) {
        let group = self.child.id();
        assert!(
            common::signal("TERM", &format!("-{group}")),
            "SIGTERM sent to the group"
        );
        self.child.wait().expect("the logs verb ends");
        assert!(
            common::wait_until(LINE_WAIT, || common::group_members(group) == 0),
            "nothing of the group is left"
        );
    }
}

#[test]
fn logs_prints_a_record_of_the_date_suffixed_member_a_real_engine_wrote() {
    let scratch = engine_scratch();
    booted(&scratch);
    let family = scratch.log_family();
    let dated = |name: &String| {
        name.strip_prefix("agent-latest.jsonl.")
            .is_some_and(|date| date.len() == 10 && date.starts_with("20"))
    };
    assert!(
        !family.is_empty() && family.iter().all(dated),
        "every member carries a date suffix and no bare name stands: {family:?}"
    );

    let tail = Tail::spawn(&mut scratch.script(&["logs"]));
    let line = tail.first_line();
    let record: serde_json::Value =
        serde_json::from_str(&line).expect("the line is one record of the engine's log");
    assert!(record["target"].is_string() && record["timestamp"].is_string());
    assert_eq!(
        record["fields"]["service.name"], "com.andromeda.pulse",
        "the record is the engine's own"
    );
    tail.end();

    assert_cleanup(&scratch, "clean", 0);
}

#[test]
fn logs_reads_the_base_the_log_file_variable_names_before_the_data_dir() {
    let scratch = Scratch::new();
    let other = scratch.path("other");
    let own = scratch.data().join("logs");
    for dir in [&other, &own] {
        std::fs::create_dir_all(dir).expect("a logs dir");
    }
    std::fs::write(other.join("agent-latest.jsonl.2026-01-01"), "named-older\n").expect("write");
    std::fs::write(
        other.join("agent-latest.jsonl.2026-01-02"),
        "named-newest\n",
    )
    .expect("write");
    std::fs::write(own.join("agent-latest.jsonl.2026-01-03"), "data-dir\n").expect("write");

    let tail = Tail::spawn(
        scratch
            .script(&["logs"])
            .env("ANDROMEDA_PULSE_LOGFILE", other.join("agent-latest.jsonl")),
    );
    assert_eq!(tail.first_line(), "named-newest");
    tail.end();
}

#[test]
fn logs_with_no_base_resolving_says_so_and_exits_one() {
    let scratch = Scratch::new();
    let ran = scratch.run(&mut scratch.script(&["logs"]));
    assert_eq!(ran.code, Some(1), "{}", scratch.printed(&ran));
    assert!(
        ran.stderr.contains("logs: no log file found at"),
        "{}",
        scratch.printed(&ran)
    );
    assert!(ran.stdout.is_empty(), "nothing on stdout");
}
