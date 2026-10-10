//! `harness:ready` and `harness:settled` — two verdicts about the REAL
//! pulse-app process that `harness:status` does not give.
//!
//! `harness:ready` is the `boot` verb's readiness signal (test-plan §3,
//! 5-command implementation): the status verdict reads `running-healthy`
//! and both OTLP receivers accept a TCP connection on loopback.
//! `harness:settled` reads the app once its windows have settled, by the
//! app's own `app.boot.window.navigation` records, or at its end, and says
//! what it saw of the display and the session bus the app depends on, and
//! what the exit witness recorded of the app's end (`harness_witness`).
//!
//! Both follow the `harness:status` shape, with one pretty-JSON verdict
//! object on stdout and exit 0 / 1 / 2. The three system variables read here
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
    Verdict, classify, end_file, newest_family_member, pid_alive, read_ended, read_pid,
    resolve_data_dir, resolve_paths,
};

const DEFAULT_GRPC_PORT: u16 = 4317;
const DEFAULT_HTTP_PORT: u16 = 4318;
const RECEIVER_PROBE_TIMEOUT: Duration = Duration::from_secs(1);

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

pub(crate) fn run_ready() -> Result<ExitCode> {
    let ports = (
        env_port("ANDROMEDA_PULSE_OTLP_GRPC_PORT", DEFAULT_GRPC_PORT),
        env_port("ANDROMEDA_PULSE_OTLP_HTTP_PORT", DEFAULT_HTTP_PORT),
    );
    let payload = match (resolve_paths(), ports) {
        (Some((pidfile, log_base)), (Some(grpc_port), Some(http_port))) => {
            let pid = read_pid(&pidfile);
            let status = classify(
                pid,
                pid.and_then(pid_alive),
                newest_family_member(&log_base),
            );
            let grpc = receiver_accepts(grpc_port);
            let http = receiver_accepts(http_port);
            let verdict = decide_ready(&status, grpc, http);
            let ended = (verdict == "ended")
                .then(|| read_ended(&end_file(&pidfile)))
                .flatten();
            json!({
                "verdict": verdict,
                "pid": pid,
                "ended": ended,
                "otlp_grpc": receiver_label(grpc),
                "otlp_http": receiver_label(http),
            })
        }
        _ => json!({
            "verdict": "cannot-evaluate",
            "pid": null,
            "ended": null,
            "otlp_grpc": null,
            "otlp_http": null,
        }),
    };
    println!("{}", serde_json::to_string_pretty(&payload)?);
    let verdict = payload["verdict"].as_str().unwrap_or("cannot-evaluate");
    Ok(ExitCode::from(exit_status(verdict)))
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

/// The readiness core — pure so the arms are pinnable without a process.
fn decide_ready(status: &Verdict, grpc_accepting: bool, http_accepting: bool) -> &'static str {
    match status.arm {
        "cannot-evaluate" => "cannot-evaluate",
        "not-running" if status.pid.is_some() => "ended",
        "running-healthy" if grpc_accepting && http_accepting => "ready",
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

fn receiver_accepts(port: u16) -> bool {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    TcpStream::connect_timeout(&addr, RECEIVER_PROBE_TIMEOUT).is_ok()
}

fn receiver_label(accepting: bool) -> &'static str {
    if accepting { "accepting" } else { "refusing" }
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
        assert_eq!(decide_ready(&healthy(), true, true), "ready");
    }

    #[test]
    fn a_refusing_receiver_is_not_ready() {
        assert_eq!(decide_ready(&healthy(), false, true), "not-ready");
        assert_eq!(decide_ready(&healthy(), true, false), "not-ready");
        assert_eq!(decide_ready(&healthy(), false, false), "not-ready");
    }

    #[test]
    fn a_dead_pid_is_ended_whatever_the_receivers_say() {
        let dead = classify(
            Some(1234),
            Some(false),
            Some(("agent-latest.jsonl.2026-10-09".into(), 0)),
        );
        assert_eq!(decide_ready(&dead, true, true), "ended");
    }

    #[test]
    fn a_stale_log_or_an_absent_pidfile_is_not_ready() {
        let stale = classify(Some(1234), Some(true), None);
        assert_eq!(decide_ready(&stale, true, true), "not-ready");
        let no_pid = classify(None, None, None);
        assert_eq!(decide_ready(&no_pid, true, true), "not-ready");
    }

    #[test]
    fn an_unresolvable_status_cannot_be_evaluated() {
        let unresolved = Verdict {
            arm: "cannot-evaluate",
            pid: None,
            log_file_basename: None,
            last_write_age_seconds: None,
        };
        assert_eq!(decide_ready(&unresolved, true, true), "cannot-evaluate");
    }

    #[test]
    fn exit_status_is_zero_only_for_a_passing_verdict() {
        assert_eq!(exit_status("ready"), 0);
        assert_eq!(exit_status("settled"), 0);
        assert_eq!(exit_status("not-ready"), 1);
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
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("bind");
        let port = listener.local_addr().expect("addr").port();
        assert!(receiver_accepts(port));
        assert_eq!(receiver_label(true), "accepting");
        drop(listener);
        assert!(!receiver_accepts(port));
        assert_eq!(receiver_label(false), "refusing");
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
