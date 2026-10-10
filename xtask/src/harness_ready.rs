//! `harness:ready` and `harness:settled` — two verdicts about the REAL
//! pulse-app process that `harness:status` does not give.
//!
//! `harness:ready` is the `boot` verb's readiness signal (test-plan §3,
//! 5-command implementation): the status verdict reads `running-healthy`,
//! both OTLP receivers accept a TCP connection on loopback, and the run's
//! own log holds each receiver's bound record at that port. A port that
//! accepts with no such record is another program's listener (`foreign`),
//! never this run's.
//! `harness:settled` reads the app once its windows have settled, by the
//! app's own `app.boot.window.navigation` records, or at its end, and says
//! what it saw of the display and the session bus the app depends on, and
//! what the exit witness recorded of the app's end (`harness_witness`).
//!
//! Both follow the `harness:status` shape, with one pretty-JSON verdict
//! object on stdout and exit 0 / 1 / 2. `harness:ready` carries the status
//! verdict's program reading: asked for one program over a log that records
//! the other, it reads `wrong-program`. The three system variables read here
//! (`DISPLAY`, `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR`) are reported as
//! closed labels only; no value of theirs is printed or written.

use std::collections::BTreeSet;
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use anyhow::Result;
use serde_json::{Value, json};

use crate::harness_status::{
    BOOT_ENGINE_TARGET, PROGRAM_UNKNOWN, Program, Verdict, classify, end_file, for_program,
    newest_family_member, pid_alive, program_of, read_ended, read_pid, resolve_data_dir,
    resolve_paths,
};

const DEFAULT_GRPC_PORT: u16 = 4317;
const DEFAULT_HTTP_PORT: u16 = 4318;
const RECEIVER_PROBE_TIMEOUT: Duration = Duration::from_secs(1);
const GRPC_BIND_TARGET: &str = "app.boot.otlp.grpc.bind";
const HTTP_BIND_TARGET: &str = "app.boot.otlp.http.bind";

const NAVIGATION_TARGET: &str = "app.boot.window.navigation";
const EXIT_TARGET: &str = "app.exit";
const WINDOW_LABELS: [&str; 4] = ["compact-widget", "main", "findings", "report"];

/// The app emits its navigation records 5 s after boot, so a shorter window
/// could never read `settled`.
const MIN_SETTLE_TIMEOUT_SECONDS: u64 = 8;
const SETTLE_POLL_INTERVAL: Duration = Duration::from_millis(200);
/// How long the waiting wrapper gets to write the exit record once the pid
/// is seen dead (the bound `agent-run.sh boot` gives it).
const EXIT_RECORD_WAIT: Duration = Duration::from_secs(2);
const KEPT_VERDICT_FILE: &str = "harness-settled.json";

pub(crate) fn run_ready(asked: Option<Program>) -> Result<ExitCode> {
    let ports = (
        env_port("ANDROMEDA_PULSE_OTLP_GRPC_PORT", DEFAULT_GRPC_PORT),
        env_port("ANDROMEDA_PULSE_OTLP_HTTP_PORT", DEFAULT_HTTP_PORT),
    );
    let payload = match (resolve_paths(), ports) {
        (Some((pidfile, log_base)), (Some(grpc_port), Some(http_port))) => {
            let pid = read_pid(&pidfile);
            // The ports are probed before the log is read, so a receiver
            // that accepts has had its bound record written first.
            let grpc_accepted = receiver_accepts(grpc_port);
            let http_accepted = receiver_accepts(http_port);
            let lines = crate::smoke::read_jsonl_lines(&log_base).unwrap_or_default();
            let program = program_of(&lines);
            let status = for_program(
                classify(
                    pid,
                    pid.and_then(pid_alive),
                    newest_family_member(&log_base),
                ),
                asked,
                program,
            );
            let grpc = Receiver::read(
                grpc_accepted,
                bound_by_this_run(&lines, GRPC_BIND_TARGET, grpc_port),
            );
            let http = Receiver::read(
                http_accepted,
                bound_by_this_run(&lines, HTTP_BIND_TARGET, http_port),
            );
            let verdict = decide_ready(&status, grpc, http);
            let ended = (verdict == "ended")
                .then(|| read_ended(&end_file(&pidfile)))
                .flatten();
            ready_payload(
                verdict,
                pid,
                ended,
                program,
                Some((grpc.label(), http.label())),
            )
        }
        _ => ready_payload("cannot-evaluate", None, None, PROGRAM_UNKNOWN, None),
    };
    println!("{}", serde_json::to_string_pretty(&payload)?);
    let verdict = payload["verdict"].as_str().unwrap_or("cannot-evaluate");
    Ok(ExitCode::from(exit_status(verdict)))
}

fn ready_payload(
    verdict: &str,
    pid: Option<u32>,
    ended: Option<String>,
    program: &str,
    receivers: Option<(&str, &str)>,
) -> Value {
    json!({
        "verdict": verdict,
        "pid": pid,
        "ended": ended,
        "program": program,
        "otlp_grpc": receivers.map(|(grpc, _)| grpc),
        "otlp_http": receivers.map(|(_, http)| http),
    })
}

pub(crate) fn run_settled(timeout_seconds: u64) -> Result<ExitCode> {
    let paths = resolve_paths();
    let (verdict, pid) = match &paths {
        Some((pidfile, log_base)) if timeout_supports_verdict(timeout_seconds) => {
            wait_for_settle(pidfile, log_base, Duration::from_secs(timeout_seconds))
        }
        _ => ("cannot-evaluate", None),
    };
    // Everything below is read AFTER the verdict, so an `ended` run reports
    // what was there once the app was gone.
    let ended = paths
        .as_ref()
        .and_then(|(pidfile, _)| read_exit_record(&end_file(pidfile), verdict == "ended"));
    let evidence = paths
        .as_ref()
        .map(|(_, log_base)| read_log_evidence(log_base))
        .unwrap_or_default();
    let env = |name: &str| std::env::var(name).ok();
    let display = probe_label(&parse_display(env("DISPLAY").as_deref()));
    let session_bus = probe_label(&parse_session_bus(
        env("DBUS_SESSION_BUS_ADDRESS").as_deref(),
        env("XDG_RUNTIME_DIR").as_deref(),
    ));

    let exit_witness =
        crate::harness_witness::label(resolve_data_dir().as_deref(), pid, ended.as_deref());

    let text = serde_json::to_string_pretty(&settled_payload(
        verdict,
        pid,
        ended,
        evidence,
        display,
        session_bus,
        exit_witness,
    ))?;
    println!("{text}");
    keep_verdict(&text);
    Ok(ExitCode::from(exit_status(verdict)))
}

fn exit_status(verdict: &str) -> u8 {
    match verdict {
        "ready" | "settled" => 0,
        "cannot-evaluate" => 2,
        _ => 1,
    }
}

/// What one receiver's port read. A port that accepts is this run's only
/// when the run's own log says the receiver bound it: on a shared host
/// another program's listener accepts too.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Receiver {
    Accepting,
    Foreign,
    Refusing,
}

impl Receiver {
    fn read(accepted: bool, bound_by_this_run: bool) -> Self {
        match (accepted, bound_by_this_run) {
            (false, _) => Receiver::Refusing,
            (true, true) => Receiver::Accepting,
            (true, false) => Receiver::Foreign,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Receiver::Accepting => "accepting",
            Receiver::Foreign => "foreign",
            Receiver::Refusing => "refusing",
        }
    }
}

/// Whether `lines` hold, after their last `app.boot.engine` record, an INFO
/// record of `target` whose `bind_address` is loopback at `port`. A bind that
/// failed is an ERROR record of the same target, and a record before the
/// last boot record belongs to an earlier boot on the same data dir.
fn bound_by_this_run(lines: &[String], target: &str, port: u16) -> bool {
    let records: Vec<Value> = lines
        .iter()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    let is =
        |record: &Value, wanted: &str| record.get("target").and_then(Value::as_str) == Some(wanted);
    let Some(boot) = records
        .iter()
        .rposition(|record| is(record, BOOT_ENGINE_TARGET))
    else {
        return false;
    };
    let address = format!("127.0.0.1:{port}");
    records[boot + 1..].iter().any(|record| {
        is(record, target)
            && record.get("level").and_then(Value::as_str) == Some("INFO")
            && record
                .get("fields")
                .and_then(|fields| fields.get("bind_address"))
                .and_then(Value::as_str)
                == Some(address.as_str())
    })
}

/// The readiness core — pure so the arms are pinnable without a process.
fn decide_ready(status: &Verdict, grpc: Receiver, http: Receiver) -> &'static str {
    match status.arm {
        "cannot-evaluate" => "cannot-evaluate",
        "wrong-program" => "wrong-program",
        "not-running" if status.pid.is_some() => "ended",
        "running-healthy" if grpc == Receiver::Accepting && http == Receiver::Accepting => "ready",
        _ => "not-ready",
    }
}

fn resolve_port(raw: Option<&str>, default: u16) -> Option<u16> {
    match raw.map(str::trim) {
        None | Some("") => Some(default),
        Some(text) => text.parse::<u16>().ok().filter(|port| *port != 0),
    }
}

fn env_port(name: &str, default: u16) -> Option<u16> {
    resolve_port(std::env::var(name).ok().as_deref(), default)
}

pub(crate) fn receiver_accepts(port: u16) -> bool {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    TcpStream::connect_timeout(&addr, RECEIVER_PROBE_TIMEOUT).is_ok()
}

fn timeout_supports_verdict(timeout_seconds: u64) -> bool {
    timeout_seconds >= MIN_SETTLE_TIMEOUT_SECONDS
}

/// The settle core — pure so the arms are pinnable without a process.
/// `None` means keep polling. A dead pid is `ended` whatever the log holds:
/// an app that settled and then ended has ended.
fn decide_settled(
    pid: Option<u32>,
    alive: Option<bool>,
    windows_settled: usize,
    timed_out: bool,
) -> Option<&'static str> {
    match (pid, alive) {
        (None, _) | (_, None) => Some("cannot-evaluate"),
        (Some(_), Some(false)) => Some("ended"),
        (Some(_), Some(true)) if windows_settled == WINDOW_LABELS.len() => Some("settled"),
        (Some(_), Some(true)) if timed_out => Some("not-settled"),
        (Some(_), Some(true)) => None,
    }
}

fn wait_for_settle(
    pidfile: &Path,
    log_base: &Path,
    timeout: Duration,
) -> (&'static str, Option<u32>) {
    let started = Instant::now();
    loop {
        let pid = read_pid(pidfile);
        let alive = pid.and_then(pid_alive);
        let windows_settled = read_log_evidence(log_base).windows_settled;
        let timed_out = started.elapsed() >= timeout;
        if let Some(verdict) = decide_settled(pid, alive, windows_settled, timed_out) {
            return (verdict, pid);
        }
        std::thread::sleep(SETTLE_POLL_INTERVAL);
    }
}

fn read_exit_record(path: &Path, wait: bool) -> Option<String> {
    let deadline = Instant::now()
        + if wait {
            EXIT_RECORD_WAIT
        } else {
            Duration::ZERO
        };
    loop {
        if let Some(record) = read_ended(path) {
            return Some(record);
        }
        if Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct LogEvidence {
    windows_settled: usize,
    app_exit_record: bool,
}

fn log_evidence(lines: &[String]) -> LogEvidence {
    let mut windows = BTreeSet::new();
    let mut app_exit_record = false;
    for record in lines
        .iter()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
    {
        match record.get("target").and_then(Value::as_str) {
            Some(EXIT_TARGET) => app_exit_record = true,
            Some(NAVIGATION_TARGET) => {
                let label = record
                    .get("fields")
                    .and_then(|fields| fields.get("window_label"))
                    .and_then(Value::as_str);
                if let Some(known) = WINDOW_LABELS.iter().find(|known| Some(**known) == label) {
                    windows.insert(*known);
                }
            }
            _ => {}
        }
    }
    LogEvidence {
        windows_settled: windows.len(),
        app_exit_record,
    }
}

fn read_log_evidence(log_base: &Path) -> LogEvidence {
    crate::smoke::read_jsonl_lines(log_base)
        .map(|lines| log_evidence(&lines))
        .unwrap_or_default()
}

/// What a system variable names, reduced to the one thing probed.
#[derive(Debug, PartialEq, Eq)]
enum SocketTarget {
    Unset,
    Socket(PathBuf),
    Unknown,
}

fn non_blank(raw: Option<&str>) -> Option<&str> {
    raw.map(str::trim).filter(|text| !text.is_empty())
}

/// A local display (`:N` or `:N.S`) is the X socket of that number; any
/// other form is not probed.
fn parse_display(raw: Option<&str>) -> SocketTarget {
    let Some(text) = non_blank(raw) else {
        return SocketTarget::Unset;
    };
    let Some(local) = text.strip_prefix(':') else {
        return SocketTarget::Unknown;
    };
    let (number, screen) = match local.split_once('.') {
        Some((number, screen)) => (number, Some(screen)),
        None => (local, None),
    };
    let digits = |part: &str| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
    if !digits(number) || !screen.is_none_or(digits) {
        return SocketTarget::Unknown;
    }
    match number.parse::<u32>() {
        Ok(number) => SocketTarget::Socket(PathBuf::from(format!("/tmp/.X11-unix/X{number}"))),
        Err(_) => SocketTarget::Unknown,
    }
}

/// A `unix:path=` session-bus address, else the `bus` socket under the
/// runtime dir. A `%`-escaped path is not decoded, so it is not probed.
fn parse_session_bus(address: Option<&str>, runtime_dir: Option<&str>) -> SocketTarget {
    match (non_blank(address), non_blank(runtime_dir)) {
        (Some(address), _) => {
            let first = address.split(';').next().unwrap_or(address);
            let path = first.strip_prefix("unix:").and_then(|params| {
                params
                    .split(',')
                    .find_map(|param| param.strip_prefix("path="))
            });
            match path {
                Some(path) if !path.is_empty() && !path.contains('%') => {
                    SocketTarget::Socket(PathBuf::from(path))
                }
                _ => SocketTarget::Unknown,
            }
        }
        (None, Some(dir)) => SocketTarget::Socket(Path::new(dir).join("bus")),
        (None, None) => SocketTarget::Unset,
    }
}

/// Whether a Unix socket accepts a connection; `None` where the platform has
/// no such socket.
fn unix_socket_accepts(path: &Path) -> Option<bool> {
    #[cfg(unix)]
    {
        Some(std::os::unix::net::UnixStream::connect(path).is_ok())
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        None
    }
}

fn socket_label(probe: Option<bool>) -> &'static str {
    match probe {
        Some(true) => "reachable",
        Some(false) => "gone",
        None => "unknown",
    }
}

fn probe_label(target: &SocketTarget) -> &'static str {
    match target {
        SocketTarget::Unset => "unset",
        SocketTarget::Unknown => "unknown",
        SocketTarget::Socket(path) => socket_label(unix_socket_accepts(path)),
    }
}

fn settled_payload(
    verdict: &str,
    pid: Option<u32>,
    ended: Option<String>,
    evidence: LogEvidence,
    display: &str,
    session_bus: &str,
    exit_witness: &str,
) -> Value {
    json!({
        "verdict": verdict,
        "pid": pid,
        "ended": ended,
        "app_exit_record": if evidence.app_exit_record { "present" } else { "absent" },
        "windows_settled": evidence.windows_settled,
        "display": display,
        "session_bus": session_bus,
        "exit_witness": exit_witness,
    })
}

/// The same object, kept beside the app's log so the job's uploaded `logs/`
/// holds it. Harness-written; the product never reads it.
fn keep_verdict(text: &str) {
    let Some(logs) = resolve_data_dir()
        .map(|dir| dir.join("logs"))
        .filter(|dir| dir.is_dir())
    else {
        return;
    };
    if std::fs::write(logs.join(KEPT_VERDICT_FILE), format!("{text}\n")).is_err() {
        eprintln!("harness:settled: could not write logs/{KEPT_VERDICT_FILE}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy() -> Verdict {
        classify(
            Some(1234),
            Some(true),
            Some(("agent-latest.jsonl.2026-10-09".into(), 1)),
        )
    }

    #[test]
    fn ready_requires_running_healthy_and_both_receivers_accepting() {
        assert_eq!(
            decide_ready(&healthy(), Receiver::Accepting, Receiver::Accepting),
            "ready"
        );
    }

    #[test]
    fn a_refusing_receiver_is_not_ready() {
        assert_eq!(
            decide_ready(&healthy(), Receiver::Refusing, Receiver::Accepting),
            "not-ready"
        );
        assert_eq!(
            decide_ready(&healthy(), Receiver::Accepting, Receiver::Refusing),
            "not-ready"
        );
        assert_eq!(
            decide_ready(&healthy(), Receiver::Refusing, Receiver::Refusing),
            "not-ready"
        );
    }

    const GRPC_PORT: u16 = 24317;
    const HTTP_PORT: u16 = 24318;

    fn boot_line(program: &str) -> String {
        json!({"level": "INFO", "target": BOOT_ENGINE_TARGET, "fields": {"program": program}})
            .to_string()
    }

    fn bind_line(target: &str, level: &str, port: u16) -> String {
        json!({
            "level": level,
            "target": target,
            "fields": {"bind_address": format!("127.0.0.1:{port}"), "reason": "in use"},
        })
        .to_string()
    }

    /// One boot of a program whose two receivers bound their ports.
    fn own_log() -> Vec<String> {
        vec![
            boot_line("console"),
            bind_line(GRPC_BIND_TARGET, "INFO", GRPC_PORT),
            bind_line(HTTP_BIND_TARGET, "INFO", HTTP_PORT),
        ]
    }

    /// Both receivers as read over `lines` with both ports accepting.
    fn read_both(lines: &[String]) -> (Receiver, Receiver) {
        (
            Receiver::read(true, bound_by_this_run(lines, GRPC_BIND_TARGET, GRPC_PORT)),
            Receiver::read(true, bound_by_this_run(lines, HTTP_BIND_TARGET, HTTP_PORT)),
        )
    }

    #[test]
    fn both_ports_accepting_over_the_run_s_own_bound_records_is_ready() {
        let (grpc, http) = read_both(&own_log());
        assert_eq!((grpc.label(), http.label()), ("accepting", "accepting"));
        assert_eq!(decide_ready(&healthy(), grpc, http), "ready");
    }

    #[test]
    fn a_held_port_with_no_bound_record_is_foreign_and_not_ready() {
        let lines = vec![
            boot_line("console"),
            bind_line(HTTP_BIND_TARGET, "INFO", HTTP_PORT),
        ];
        let (grpc, http) = read_both(&lines);
        assert_eq!((grpc.label(), http.label()), ("foreign", "accepting"));
        assert_eq!(decide_ready(&healthy(), grpc, http), "not-ready");
        assert_eq!(decide_ready(&healthy(), http, grpc), "not-ready");
        assert_eq!(decide_ready(&healthy(), grpc, grpc), "not-ready");
    }

    #[test]
    fn a_failed_bind_record_of_the_same_target_does_not_count() {
        let lines = vec![
            boot_line("console"),
            bind_line(GRPC_BIND_TARGET, "ERROR", GRPC_PORT),
            bind_line(HTTP_BIND_TARGET, "INFO", HTTP_PORT),
        ];
        let (grpc, http) = read_both(&lines);
        assert_eq!((grpc.label(), http.label()), ("foreign", "accepting"));
        assert_eq!(decide_ready(&healthy(), grpc, http), "not-ready");
    }

    #[test]
    fn a_bound_record_for_another_port_or_the_other_receiver_does_not_count() {
        let other_port = vec![
            boot_line("window"),
            bind_line(GRPC_BIND_TARGET, "INFO", GRPC_PORT + 1),
            bind_line(HTTP_BIND_TARGET, "INFO", HTTP_PORT),
        ];
        assert_eq!(read_both(&other_port).0.label(), "foreign");
        let other_receiver = vec![
            boot_line("window"),
            bind_line(HTTP_BIND_TARGET, "INFO", GRPC_PORT),
            bind_line(HTTP_BIND_TARGET, "INFO", HTTP_PORT),
        ];
        assert_eq!(read_both(&other_receiver).0.label(), "foreign");
    }

    #[test]
    fn a_bound_record_before_the_last_boot_record_does_not_count() {
        let mut lines = own_log();
        lines.push(boot_line("console"));
        lines.push(bind_line(HTTP_BIND_TARGET, "INFO", HTTP_PORT));
        let (grpc, http) = read_both(&lines);
        assert_eq!((grpc.label(), http.label()), ("foreign", "accepting"));
        assert_eq!(decide_ready(&healthy(), grpc, http), "not-ready");
    }

    #[test]
    fn a_log_with_no_boot_record_holds_no_bound_record_of_this_run() {
        let lines = own_log()[1..].to_vec();
        let (grpc, http) = read_both(&lines);
        assert_eq!((grpc.label(), http.label()), ("foreign", "foreign"));
        assert_eq!(read_both(&[]).0.label(), "foreign");
        assert_eq!(read_both(&["not json".to_string()]).1.label(), "foreign");
    }

    #[test]
    fn a_port_that_refuses_is_refusing_whatever_the_log_holds() {
        let lines = own_log();
        let bound = bound_by_this_run(&lines, GRPC_BIND_TARGET, GRPC_PORT);
        assert_eq!(Receiver::read(false, bound).label(), "refusing");
        assert_eq!(Receiver::read(false, false).label(), "refusing");
    }

    #[test]
    fn a_receiver_reads_one_of_three_closed_labels() {
        let labels: BTreeSet<&str> = [(true, true), (true, false), (false, true), (false, false)]
            .into_iter()
            .map(|(accepted, bound)| Receiver::read(accepted, bound).label())
            .collect();
        assert_eq!(labels, BTreeSet::from(["accepting", "foreign", "refusing"]));
    }

    #[test]
    fn a_dead_pid_is_ended_whatever_the_receivers_say() {
        let dead = classify(
            Some(1234),
            Some(false),
            Some(("agent-latest.jsonl.2026-10-09".into(), 0)),
        );
        assert_eq!(
            decide_ready(&dead, Receiver::Accepting, Receiver::Accepting),
            "ended"
        );
    }

    #[test]
    fn a_stale_log_or_an_absent_pidfile_is_not_ready() {
        let stale = classify(Some(1234), Some(true), None);
        assert_eq!(
            decide_ready(&stale, Receiver::Accepting, Receiver::Accepting),
            "not-ready"
        );
        let no_pid = classify(None, None, None);
        assert_eq!(
            decide_ready(&no_pid, Receiver::Accepting, Receiver::Accepting),
            "not-ready"
        );
    }

    #[test]
    fn an_unresolvable_status_cannot_be_evaluated() {
        let unresolved = Verdict {
            arm: "cannot-evaluate",
            pid: None,
            log_file_basename: None,
            last_write_age_seconds: None,
        };
        assert_eq!(
            decide_ready(&unresolved, Receiver::Accepting, Receiver::Accepting),
            "cannot-evaluate"
        );
    }

    #[test]
    fn a_status_of_the_wrong_program_is_wrong_program_whatever_the_receivers_say() {
        let wrong = for_program(healthy(), Some(Program::Console), "window");
        assert_eq!(
            decide_ready(&wrong, Receiver::Accepting, Receiver::Accepting),
            "wrong-program"
        );
        assert_eq!(
            decide_ready(&wrong, Receiver::Refusing, Receiver::Refusing),
            "wrong-program"
        );
        let no_record = for_program(healthy(), Some(Program::Window), PROGRAM_UNKNOWN);
        assert_eq!(
            decide_ready(&no_record, Receiver::Accepting, Receiver::Accepting),
            "wrong-program"
        );
    }

    #[test]
    fn the_asked_program_matching_the_log_is_ready_as_before() {
        let console = for_program(healthy(), Some(Program::Console), "console");
        assert_eq!(
            decide_ready(&console, Receiver::Accepting, Receiver::Accepting),
            "ready"
        );
        let unasked = for_program(healthy(), None, "console");
        assert_eq!(
            decide_ready(&unasked, Receiver::Accepting, Receiver::Accepting),
            "ready"
        );
    }

    #[test]
    fn a_dead_pid_is_ended_whatever_program_was_asked() {
        let dead = for_program(
            classify(
                Some(1234),
                Some(false),
                Some(("agent-latest.jsonl.2026-10-10".into(), 0)),
            ),
            Some(Program::Console),
            "window",
        );
        assert_eq!(
            decide_ready(&dead, Receiver::Accepting, Receiver::Accepting),
            "ended"
        );
    }

    #[test]
    fn the_ready_payload_carries_the_closed_member_set() {
        let payload = ready_payload(
            "wrong-program",
            Some(7),
            None,
            "window",
            Some(("accepting", "refusing")),
        );
        let keys: BTreeSet<&str> = payload
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            BTreeSet::from([
                "verdict",
                "pid",
                "ended",
                "program",
                "otlp_grpc",
                "otlp_http"
            ])
        );
        assert_eq!(payload["program"], "window");
        assert_eq!(payload["otlp_grpc"], "accepting");
        assert_eq!(payload["otlp_http"], "refusing");
        let unresolved = ready_payload("cannot-evaluate", None, None, PROGRAM_UNKNOWN, None);
        assert!(unresolved["otlp_grpc"].is_null() && unresolved["otlp_http"].is_null());
        assert_eq!(unresolved["program"], PROGRAM_UNKNOWN);
    }

    #[test]
    fn exit_status_is_zero_only_for_a_passing_verdict() {
        assert_eq!(exit_status("ready"), 0);
        assert_eq!(exit_status("settled"), 0);
        assert_eq!(exit_status("not-ready"), 1);
        assert_eq!(exit_status("wrong-program"), 1);
        assert_eq!(exit_status("ended"), 1);
        assert_eq!(exit_status("not-settled"), 1);
        assert_eq!(exit_status("cannot-evaluate"), 2);
    }

    #[test]
    fn resolve_port_falls_back_to_the_default_and_rejects_garbage() {
        assert_eq!(resolve_port(None, 4317), Some(4317));
        assert_eq!(resolve_port(Some("  "), 4317), Some(4317));
        assert_eq!(resolve_port(Some(" 14317 "), 4317), Some(14317));
        assert_eq!(resolve_port(Some("0"), 4317), None);
        assert_eq!(resolve_port(Some("65536"), 4317), None);
        assert_eq!(resolve_port(Some("grpc"), 4317), None);
    }

    #[test]
    fn receiver_probe_tells_a_listening_loopback_port_from_a_closed_one() {
        // The claim outlives the listener, so no other test of the workspace
        // takes the port between the drop and the refusing read.
        let claim = test_support::ports::claim_port();
        let port = claim.number;
        let listener = std::net::TcpListener::bind(("127.0.0.1", port)).expect("bind");
        assert!(receiver_accepts(port));
        assert_eq!(Receiver::read(true, true).label(), "accepting");
        drop(listener);
        assert!(!receiver_accepts(port));
        assert_eq!(Receiver::read(false, true).label(), "refusing");
    }

    #[test]
    fn a_dead_pid_is_ended_even_after_every_window_settled() {
        assert_eq!(
            decide_settled(Some(7), Some(false), 4, false),
            Some("ended")
        );
        assert_eq!(decide_settled(Some(7), Some(false), 0, true), Some("ended"));
    }

    #[test]
    fn four_settled_windows_and_a_live_pid_is_settled() {
        assert_eq!(
            decide_settled(Some(7), Some(true), 4, false),
            Some("settled")
        );
    }

    #[test]
    fn a_live_pid_short_of_four_windows_polls_until_the_timeout() {
        assert_eq!(decide_settled(Some(7), Some(true), 3, false), None);
        assert_eq!(
            decide_settled(Some(7), Some(true), 3, true),
            Some("not-settled")
        );
    }

    #[test]
    fn an_absent_or_unprobeable_pid_cannot_be_evaluated() {
        assert_eq!(
            decide_settled(None, None, 4, false),
            Some("cannot-evaluate")
        );
        assert_eq!(
            decide_settled(Some(7), None, 4, false),
            Some("cannot-evaluate")
        );
    }

    #[test]
    fn a_window_shorter_than_the_settle_record_is_refused() {
        assert!(!timeout_supports_verdict(7));
        assert!(timeout_supports_verdict(8));
        assert!(timeout_supports_verdict(30));
    }

    fn record(target: &str, window_label: Option<&str>) -> String {
        match window_label {
            Some(label) => json!({
                "target": target,
                "fields": {"window_label": label, "navigated": true, "reason": "navigated"},
            }),
            None => json!({"target": target, "fields": {}}),
        }
        .to_string()
    }

    #[test]
    fn log_evidence_counts_each_known_window_once() {
        let lines = vec![
            record(NAVIGATION_TARGET, Some("compact-widget")),
            record(NAVIGATION_TARGET, Some("main")),
            record(NAVIGATION_TARGET, Some("main")),
            record(NAVIGATION_TARGET, Some("unknown")),
            record("ui.layout.transition", Some("findings")),
            "not json".to_string(),
        ];
        let evidence = log_evidence(&lines);
        assert_eq!(evidence.windows_settled, 2);
        assert!(!evidence.app_exit_record);
    }

    #[test]
    fn log_evidence_reads_all_four_windows_and_the_exit_record() {
        let mut lines: Vec<String> = WINDOW_LABELS
            .iter()
            .map(|label| record(NAVIGATION_TARGET, Some(label)))
            .collect();
        lines.push(record(EXIT_TARGET, None));
        let evidence = log_evidence(&lines);
        assert_eq!(evidence.windows_settled, 4);
        assert!(evidence.app_exit_record);
    }

    #[test]
    fn display_parser_reads_a_local_display_only() {
        assert_eq!(parse_display(None), SocketTarget::Unset);
        assert_eq!(parse_display(Some(" ")), SocketTarget::Unset);
        assert_eq!(
            parse_display(Some(":99")),
            SocketTarget::Socket(PathBuf::from("/tmp/.X11-unix/X99"))
        );
        assert_eq!(
            parse_display(Some(":0.0")),
            SocketTarget::Socket(PathBuf::from("/tmp/.X11-unix/X0"))
        );
        assert_eq!(parse_display(Some("localhost:10.0")), SocketTarget::Unknown);
        assert_eq!(parse_display(Some(":")), SocketTarget::Unknown);
        assert_eq!(parse_display(Some(":x")), SocketTarget::Unknown);
        assert_eq!(parse_display(Some(":1.")), SocketTarget::Unknown);
    }

    #[test]
    fn session_bus_parser_reads_a_unix_path_or_the_runtime_dir_socket() {
        assert_eq!(
            parse_session_bus(Some("unix:path=/run/user/1000/bus"), Some("/elsewhere")),
            SocketTarget::Socket(PathBuf::from("/run/user/1000/bus"))
        );
        assert_eq!(
            parse_session_bus(Some("unix:path=/tmp/dbus-abc,guid=0123"), None),
            SocketTarget::Socket(PathBuf::from("/tmp/dbus-abc"))
        );
        assert_eq!(
            parse_session_bus(None, Some("/run/user/1000")),
            SocketTarget::Socket(PathBuf::from("/run/user/1000/bus"))
        );
        assert_eq!(parse_session_bus(None, None), SocketTarget::Unset);
        assert_eq!(parse_session_bus(Some(""), Some(" ")), SocketTarget::Unset);
        assert_eq!(
            parse_session_bus(Some("unix:abstract=/tmp/dbus-abc"), Some("/run/user/1000")),
            SocketTarget::Unknown
        );
        assert_eq!(
            parse_session_bus(Some("tcp:host=127.0.0.1,port=1234"), None),
            SocketTarget::Unknown
        );
        assert_eq!(
            parse_session_bus(Some("unix:path=/tmp/a%20b"), None),
            SocketTarget::Unknown
        );
    }

    #[cfg(unix)]
    #[test]
    fn socket_probe_tells_a_listening_unix_socket_from_a_dead_or_absent_one() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let path = dir.path().join("bus");
        let listener = std::os::unix::net::UnixListener::bind(&path).expect("bind");
        assert_eq!(
            probe_label(&SocketTarget::Socket(path.clone())),
            "reachable"
        );
        // The socket file outlives its listener, as it does when a display
        // server is killed rather than shut down.
        drop(listener);
        assert!(path.exists());
        assert_eq!(probe_label(&SocketTarget::Socket(path)), "gone");
        assert_eq!(
            probe_label(&SocketTarget::Socket(dir.path().join("absent"))),
            "gone"
        );
    }

    #[test]
    fn an_unset_or_unreadable_target_is_labelled_without_a_probe() {
        assert_eq!(probe_label(&SocketTarget::Unset), "unset");
        assert_eq!(probe_label(&SocketTarget::Unknown), "unknown");
        assert_eq!(socket_label(None), "unknown");
    }

    #[test]
    fn settled_payload_carries_the_closed_field_set() {
        let payload = settled_payload(
            "ended",
            Some(7),
            Some("exit 1".to_string()),
            LogEvidence {
                windows_settled: 2,
                app_exit_record: false,
            },
            "gone",
            "unset",
            "exit-call",
        );
        let keys: BTreeSet<&str> = payload
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            BTreeSet::from([
                "verdict",
                "pid",
                "ended",
                "app_exit_record",
                "windows_settled",
                "display",
                "session_bus",
                "exit_witness",
            ])
        );
        assert_eq!(payload["verdict"], "ended");
        assert_eq!(payload["ended"], "exit 1");
        assert_eq!(payload["app_exit_record"], "absent");
        assert_eq!(payload["windows_settled"], 2);
        assert_eq!(payload["display"], "gone");
        assert_eq!(payload["exit_witness"], "exit-call");
    }
}
