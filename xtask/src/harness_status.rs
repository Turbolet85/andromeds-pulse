//! Truthful `harness:status` — a report about the REAL pulse-app process.
//!
//! The pre-2026-08-30 form returned `ui_bridge::health::current_health()`,
//! an envelope constructed inside THIS xtask process — well-formed, exit 0,
//! with no pulse-app running at all (test-plan §3's measured hazard: a
//! status-green gate describing a different process). The verdict now
//! derives from the artifacts only a running app maintains: the registered
//! PID file (`run/andromeda-pulse.pid`, arch §Occupied Resources) and the
//! app's own rotated log family (`logs/agent-latest.jsonl*`).
//!
//! Formalized contract (per the `check:npm-supply-chain` registry shape):
//! one pretty-JSON verdict object on stdout; verdict arms
//! `running-healthy` / `stale` / `wrong-program` / `not-running` /
//! `cannot-evaluate`; exit 0 / 1 / 1 / 1 / 2. The reported `pid` is the
//! identity a caller matches against the process it spawned (test-plan §3
//! status-endpoint identity).
//!
//! Two programs write the same pid file and log family, so the verdict also
//! names the program the log itself records (`program`: the last
//! `app.boot.engine` record's label, `window` / `console`, or `unknown`). A
//! caller that asks for one program and reads the other gets `wrong-program`
//! instead of a grade of the wrong run.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

use anyhow::Result;
use serde_json::{Value, json};

/// Ticks emit every 15s; four missed ticks reads as a wedged or dead
/// writer. Deliberately far below the 450s drain-stall threshold — this is
/// liveness-of-writes, not drain progress.
const STALE_AFTER_SECONDS: u64 = 60;

pub(crate) const BOOT_ENGINE_TARGET: &str = "app.boot.engine";
pub(crate) const PROGRAM_UNKNOWN: &str = "unknown";

/// The program a verb is asked to grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub(crate) enum Program {
    Window,
    Console,
}

impl Program {
    pub(crate) const ALL: [Program; 2] = [Program::Window, Program::Console];

    /// The label the program's own boot record carries.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Program::Window => "window",
            Program::Console => "console",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Verdict {
    pub(crate) arm: &'static str,
    pub(crate) pid: Option<u32>,
    pub(crate) log_file_basename: Option<String>,
    pub(crate) last_write_age_seconds: Option<u64>,
}

pub(crate) fn run(asked: Option<Program>) -> Result<ExitCode> {
    let (verdict, ended, program) = match resolve_paths() {
        Some((pidfile, log_base)) => {
            let pid = read_pid(&pidfile);
            let alive = pid.and_then(pid_alive);
            let newest = newest_family_member(&log_base);
            let program = read_program(&log_base);
            let verdict = for_program(classify(pid, alive, newest), asked, program);
            let ended = (verdict.arm != "running-healthy")
                .then(|| read_ended(&end_file(&pidfile)))
                .flatten();
            (verdict, ended, program)
        }
        None => (
            Verdict {
                arm: "cannot-evaluate",
                pid: None,
                log_file_basename: None,
                last_write_age_seconds: None,
            },
            None,
            PROGRAM_UNKNOWN,
        ),
    };

    let payload = status_payload(&verdict, ended, program);
    println!("{}", serde_json::to_string_pretty(&payload)?);

    Ok(match verdict.arm {
        "running-healthy" => ExitCode::SUCCESS,
        "cannot-evaluate" => ExitCode::from(2),
        _ => ExitCode::FAILURE,
    })
}

fn status_payload(verdict: &Verdict, ended: Option<String>, program: &str) -> Value {
    json!({
        "verdict": verdict.arm,
        "pid": verdict.pid,
        "ended": ended,
        "program": program,
        "log_file_basename": verdict.log_file_basename,
        "last_write_age_seconds": verdict.last_write_age_seconds,
        "stale_after_seconds": STALE_AFTER_SECONDS,
    })
}

/// The program a boot record names, `unknown` when it names none of the two;
/// `None` for any other record.
pub(crate) fn boot_record_program(record: &Value) -> Option<&'static str> {
    if record.get("target").and_then(Value::as_str) != Some(BOOT_ENGINE_TARGET) {
        return None;
    }
    let label = record
        .get("fields")
        .and_then(|fields| fields.get("program"))
        .and_then(Value::as_str);
    Some(
        Program::ALL
            .into_iter()
            .map(Program::label)
            .find(|known| Some(*known) == label)
            .unwrap_or(PROGRAM_UNKNOWN),
    )
}

/// The program of the LAST `app.boot.engine` record in `lines`: a data dir
/// booted twice is graded as what booted last.
pub(crate) fn program_of(lines: &[String]) -> &'static str {
    lines
        .iter()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter_map(|record| boot_record_program(&record))
        .next_back()
        .unwrap_or(PROGRAM_UNKNOWN)
}

/// `program_of` over the log family at `log_base`; an unreadable or absent
/// family is `unknown`.
pub(crate) fn read_program(log_base: &Path) -> &'static str {
    crate::smoke::read_jsonl_lines(log_base)
        .map(|lines| program_of(&lines))
        .unwrap_or(PROGRAM_UNKNOWN)
}

/// The arm once the asked program is held against the one the log records.
/// A run that is not there is not graded: `cannot-evaluate` and
/// `not-running` stand whatever was asked.
pub(crate) fn graded_arm(arm: &'static str, asked: Option<Program>, program: &str) -> &'static str {
    match (arm, asked) {
        ("cannot-evaluate" | "not-running", _) | (_, None) => arm,
        (_, Some(asked)) if asked.label() == program => arm,
        _ => "wrong-program",
    }
}

pub(crate) fn for_program(verdict: Verdict, asked: Option<Program>, program: &str) -> Verdict {
    Verdict {
        arm: graded_arm(verdict.arm, asked, program),
        ..verdict
    }
}

/// The verdict core — pure so the arms are pinnable without a filesystem.
///
/// `alive` is the pid's measured liveness (`None` when no probe could run).
/// A dead pid is `not-running` whatever the log says: an app that crashed
/// a few milliseconds after boot leaves a pidfile and a fresh log behind,
/// which write-freshness alone reads as healthy for the whole window.
pub(crate) fn classify(
    pid: Option<u32>,
    alive: Option<bool>,
    newest_log: Option<(String, u64)>,
) -> Verdict {
    match (pid, newest_log) {
        (Some(pid), newest) if alive == Some(false) => Verdict {
            arm: "not-running",
            pid: Some(pid),
            log_file_basename: newest.as_ref().map(|(b, _)| b.clone()),
            last_write_age_seconds: newest.map(|(_, age)| age),
        },
        (None, newest) => Verdict {
            arm: "not-running",
            pid: None,
            log_file_basename: newest.as_ref().map(|(b, _)| b.clone()),
            last_write_age_seconds: newest.map(|(_, age)| age),
        },
        (Some(pid), None) => Verdict {
            arm: "stale",
            pid: Some(pid),
            log_file_basename: None,
            last_write_age_seconds: None,
        },
        (Some(pid), Some((basename, age))) => Verdict {
            arm: if age <= STALE_AFTER_SECONDS {
                "running-healthy"
            } else {
                "stale"
            },
            pid: Some(pid),
            log_file_basename: Some(basename),
            last_write_age_seconds: Some(age),
        },
    }
}

/// Harness-only path resolution (trim + fall back; the vars are the
/// `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` / `_DATA_DIR` class — external
/// tool-locator carve-out, never data-dir-confined).
pub(crate) fn resolve_paths() -> Option<(PathBuf, PathBuf)> {
    let data_dir = resolve_data_dir()?;
    let pidfile = env_path("ANDROMEDA_PULSE_PIDFILE")
        .unwrap_or_else(|| data_dir.join("run").join("andromeda-pulse.pid"));
    let log_base = env_path("ANDROMEDA_PULSE_LOGFILE")
        .unwrap_or_else(|| data_dir.join("logs").join("agent-latest.jsonl"));
    Some((pidfile, log_base))
}

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var(name).ok().and_then(|v| {
        let t = v.trim();
        (!t.is_empty()).then(|| PathBuf::from(t))
    })
}

pub(crate) fn resolve_data_dir() -> Option<PathBuf> {
    env_path("ANDROMEDA_PULSE_DATA_DIR").or_else(default_data_dir)
}

fn default_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .ok()
            .map(|a| PathBuf::from(a).join("andromeda-pulse"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".andromeda-pulse"))
    }
}

/// Whether `pid` names a live process; `None` when the probe cannot run.
pub(crate) fn pid_alive(pid: u32) -> Option<bool> {
    #[cfg(windows)]
    {
        let out = std::process::Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}"), "/NH", "/FO", "CSV"])
            .output()
            .ok()?;
        Some(tasklist_lists_pid(
            &String::from_utf8_lossy(&out.stdout),
            pid,
        ))
    }
    #[cfg(not(windows))]
    {
        let out = std::process::Command::new("ps")
            .args(["-o", "stat=", "-p", &pid.to_string()])
            .output()
            .ok()?;
        Some(ps_state_is_live(
            out.status.success(),
            &String::from_utf8_lossy(&out.stdout),
        ))
    }
}

/// `ps -o stat=` prints nothing and fails for an absent pid; an exited but
/// unreaped child is still listed, as a zombie (`Z`), and is not running.
#[cfg_attr(windows, allow(dead_code))]
fn ps_state_is_live(success: bool, stat: &str) -> bool {
    let state = stat.trim();
    success && !state.is_empty() && !state.starts_with('Z')
}

/// `tasklist /FO CSV /NH` quotes the pid as its own field; a miss prints an
/// INFO line with no fields.
#[cfg_attr(not(windows), allow(dead_code))]
fn tasklist_lists_pid(csv: &str, pid: u32) -> bool {
    let quoted = format!("\"{pid}\"");
    csv.lines()
        .any(|line| line.split(',').any(|field| field.trim() == quoted))
}

/// Where `agent-run.sh boot`'s waiting wrapper records how the app ended,
/// beside the pidfile. The app is an orphan once boot returns, so no later
/// verb can reap it and read its status any other way.
pub(crate) fn end_file(pidfile: &Path) -> PathBuf {
    pidfile.with_file_name("andromeda-pulse.exit")
}

/// The recorded end (`exit N` / `signal N (NAME)`), bounded to one short
/// printable line; anything else reads as no record.
pub(crate) fn read_ended(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let line = text.trim();
    (!line.is_empty() && line.len() <= 48 && line.chars().all(|c| c.is_ascii_graphic() || c == ' '))
        .then(|| line.to_owned())
}

pub(crate) fn read_pid(pidfile: &Path) -> Option<u32> {
    std::fs::read_to_string(pidfile)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
}

/// Newest rotated-family member by mtime — `tracing_appender`'s daily
/// roller date-suffixes the sink, so a bare-name read misses a healthy
/// boot (the same family rule as `smoke::read_jsonl_lines`).
pub(crate) fn newest_family_member(log_base: &Path) -> Option<(String, u64)> {
    let dir = log_base.parent()?;
    let stem = log_base.file_name()?.to_string_lossy().into_owned();
    let newest = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().starts_with(&stem))
                .unwrap_or(false)
        })
        .filter_map(|p| {
            let mtime = std::fs::metadata(&p).ok()?.modified().ok()?;
            Some((p, mtime))
        })
        .max_by_key(|(_, mtime)| *mtime)?;
    let (path, mtime) = newest;
    let age = SystemTime::now()
        .duration_since(mtime)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let basename = path.file_name()?.to_string_lossy().into_owned();
    Some((basename, age))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_pidfile_is_not_running_regardless_of_logs() {
        let v = classify(None, None, Some(("agent-latest.jsonl".into(), 1)));
        assert_eq!(v.arm, "not-running");
        assert_eq!(v.pid, None);
    }

    #[test]
    fn pid_without_any_log_family_is_stale() {
        let v = classify(Some(1234), Some(true), None);
        assert_eq!(v.arm, "stale");
        assert_eq!(v.pid, Some(1234));
    }

    #[test]
    fn pid_with_fresh_log_writes_is_running_healthy() {
        let v = classify(
            Some(1234),
            Some(true),
            Some(("agent-latest.jsonl.2026-08-30".into(), 3)),
        );
        assert_eq!(v.arm, "running-healthy");
        assert_eq!(v.last_write_age_seconds, Some(3));
    }

    #[test]
    fn pid_with_old_log_writes_is_stale() {
        let v = classify(
            Some(1234),
            Some(true),
            Some(("agent-latest.jsonl".into(), STALE_AFTER_SECONDS + 1)),
        );
        assert_eq!(v.arm, "stale");
    }

    #[test]
    fn boundary_age_is_still_healthy() {
        let v = classify(
            Some(1),
            Some(true),
            Some(("agent-latest.jsonl".into(), STALE_AFTER_SECONDS)),
        );
        assert_eq!(v.arm, "running-healthy");
    }

    #[test]
    fn newest_family_member_prefers_latest_mtime_and_reads_the_family() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let base = dir.path().join("agent-latest.jsonl");
        // ONLY a date-suffixed member exists — the family branch, not the
        // bare name.
        let dated = dir.path().join("agent-latest.jsonl.2026-08-30");
        std::fs::write(&dated, "x").expect("write");
        let (basename, _age) = newest_family_member(&base).expect("family member found");
        assert_eq!(basename, "agent-latest.jsonl.2026-08-30");
    }

    #[test]
    fn a_dead_pid_with_fresh_log_writes_is_not_running() {
        // The measured CI shape: the app panicked 12 ms after boot, leaving
        // a pidfile and a just-written log.
        let v = classify(
            Some(71667),
            Some(false),
            Some(("agent-latest.jsonl.2026-09-29".into(), 0)),
        );
        assert_eq!(v.arm, "not-running");
        assert_eq!(v.pid, Some(71667));
    }

    #[test]
    fn an_unprobeable_pid_falls_back_to_write_freshness() {
        let v = classify(Some(1234), None, Some(("agent-latest.jsonl".into(), 3)));
        assert_eq!(v.arm, "running-healthy");
    }

    #[test]
    fn pid_alive_tells_a_running_process_from_an_exited_one() {
        assert_eq!(pid_alive(std::process::id()), Some(true));
        #[cfg(windows)]
        let mut child = std::process::Command::new("cmd")
            .args(["/C", "exit 0"])
            .spawn()
            .expect("spawn");
        #[cfg(not(windows))]
        let mut child = std::process::Command::new("true").spawn().expect("spawn");
        let pid = child.id();
        child.wait().expect("wait");
        assert_eq!(pid_alive(pid), Some(false));
    }

    #[test]
    fn a_zombie_or_absent_ps_row_is_not_live() {
        assert!(ps_state_is_live(true, "Ss\n"));
        assert!(!ps_state_is_live(true, "Z+\n"));
        assert!(!ps_state_is_live(false, ""));
    }

    #[test]
    fn tasklist_matches_the_pid_field_exactly() {
        let hit = "\"pulse-app.exe\",\"4321\",\"Console\",\"1\",\"50,000 K\"\r\n";
        assert!(tasklist_lists_pid(hit, 4321));
        assert!(!tasklist_lists_pid(hit, 432));
        let miss = "INFO: No tasks are running which match the specified criteria.\r\n";
        assert!(!tasklist_lists_pid(miss, 4321));
    }

    #[test]
    fn read_ended_takes_one_bounded_line_beside_the_pidfile() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let exit = end_file(&dir.path().join("andromeda-pulse.pid"));
        assert_eq!(exit, dir.path().join("andromeda-pulse.exit"));
        assert_eq!(read_ended(&exit), None, "no record");
        std::fs::write(&exit, "signal 11 (SEGV)\n").expect("write");
        assert_eq!(read_ended(&exit).as_deref(), Some("signal 11 (SEGV)"));
        std::fs::write(&exit, "exit 0").expect("write");
        assert_eq!(read_ended(&exit).as_deref(), Some("exit 0"));
        std::fs::write(&exit, "exit\u{1b}[31m 1").expect("write");
        assert_eq!(read_ended(&exit), None, "control bytes");
        std::fs::write(&exit, "x".repeat(49)).expect("write");
        assert_eq!(read_ended(&exit), None, "over the bound");
    }

    fn boot_record(program: &str) -> String {
        json!({
            "timestamp": "2026-10-10T00:00:00.000000Z",
            "level": "INFO",
            "target": BOOT_ENGINE_TARGET,
            "fields": {"program": program, "interpretation": "none", "reason": "deterministic_gate_unset"},
        })
        .to_string()
    }

    #[test]
    fn the_program_is_the_last_boot_record_of_the_family() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let base = dir.path().join("agent-latest.jsonl");
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-10-09"),
            format!("{}\n", boot_record("window")),
        )
        .expect("write");
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-10-10"),
            format!(
                "{}\n{}\n",
                r#"{"target":"ingest.tick","fields":{}}"#,
                boot_record("console")
            ),
        )
        .expect("write");
        assert_eq!(read_program(&base), "console");
    }

    #[test]
    fn a_family_with_no_boot_record_reads_unknown() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let base = dir.path().join("agent-latest.jsonl");
        assert_eq!(read_program(&base), PROGRAM_UNKNOWN, "no family at all");
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-10-10"),
            "{\"target\":\"ingest.tick\",\"fields\":{}}\nnot json\n",
        )
        .expect("write");
        assert_eq!(read_program(&base), PROGRAM_UNKNOWN);
        assert_eq!(
            read_program(&dir.path().join("absent").join("agent-latest.jsonl")),
            PROGRAM_UNKNOWN,
            "no log dir"
        );
    }

    #[test]
    fn a_boot_record_naming_no_known_program_reads_unknown() {
        assert_eq!(program_of(&[boot_record("<redacted>")]), PROGRAM_UNKNOWN);
        assert_eq!(
            program_of(&[r#"{"target":"app.boot.engine","fields":{}}"#.to_owned()]),
            PROGRAM_UNKNOWN
        );
        assert_eq!(
            program_of(&[boot_record("console"), boot_record("tray")]),
            PROGRAM_UNKNOWN,
            "the last record decides, also when it names nothing known"
        );
    }

    #[test]
    fn a_file_beside_the_family_is_not_read_for_the_program() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let base = dir.path().join("agent-latest.jsonl");
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-10-10"),
            format!("{}\n", boot_record("console")),
        )
        .expect("write");
        // Sorts after the member, so a reader that swept the whole dir would
        // take it as the last record.
        std::fs::write(
            dir.path().join("boot.log"),
            format!("{}\n", boot_record("window")),
        )
        .expect("write");
        assert_eq!(read_program(&base), "console");
    }

    #[test]
    fn an_asked_program_that_the_log_does_not_record_is_wrong_program() {
        for arm in ["running-healthy", "stale"] {
            assert_eq!(
                graded_arm(arm, Some(Program::Console), "window"),
                "wrong-program"
            );
            assert_eq!(
                graded_arm(arm, Some(Program::Window), "console"),
                "wrong-program"
            );
            assert_eq!(
                graded_arm(arm, Some(Program::Console), PROGRAM_UNKNOWN),
                "wrong-program",
                "a log with no boot record is a mismatch too"
            );
        }
    }

    #[test]
    fn the_asked_program_matching_the_log_keeps_the_arm() {
        assert_eq!(
            graded_arm("running-healthy", Some(Program::Console), "console"),
            "running-healthy"
        );
        assert_eq!(
            graded_arm("stale", Some(Program::Window), "window"),
            "stale"
        );
    }

    #[test]
    fn with_no_program_asked_the_arm_stands_whatever_the_log_records() {
        for program in ["window", "console", PROGRAM_UNKNOWN] {
            assert_eq!(
                graded_arm("running-healthy", None, program),
                "running-healthy"
            );
        }
    }

    #[test]
    fn a_run_that_is_not_there_is_not_graded_for_its_program() {
        for arm in ["not-running", "cannot-evaluate"] {
            assert_eq!(graded_arm(arm, Some(Program::Console), "window"), arm);
            assert_eq!(graded_arm(arm, Some(Program::Window), PROGRAM_UNKNOWN), arm);
        }
    }

    #[test]
    fn for_program_changes_the_arm_alone() {
        let healthy = classify(Some(7), Some(true), Some(("agent-latest.jsonl".into(), 1)));
        let graded = for_program(healthy, Some(Program::Console), "window");
        assert_eq!(graded.arm, "wrong-program");
        assert_eq!(graded.pid, Some(7));
        assert_eq!(graded.last_write_age_seconds, Some(1));
    }

    #[test]
    fn the_status_payload_carries_the_closed_member_set() {
        let verdict = classify(Some(7), Some(true), Some(("agent-latest.jsonl".into(), 1)));
        let payload = status_payload(&verdict, None, "console");
        let keys: std::collections::BTreeSet<&str> = payload
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            std::collections::BTreeSet::from([
                "verdict",
                "pid",
                "ended",
                "program",
                "log_file_basename",
                "last_write_age_seconds",
                "stale_after_seconds",
            ])
        );
        assert_eq!(payload["program"], "console");
    }

    #[test]
    fn the_program_labels_are_the_boot_record_s_own() {
        assert_eq!(Program::Window.label(), "window");
        assert_eq!(Program::Console.label(), "console");
    }

    #[test]
    fn read_pid_rejects_garbage_content() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let pidfile = dir.path().join("andromeda-pulse.pid");
        std::fs::write(&pidfile, "not-a-pid").expect("write");
        assert_eq!(read_pid(&pidfile), None);
        std::fs::write(&pidfile, " 4321 ").expect("write");
        assert_eq!(read_pid(&pidfile), Some(4321));
    }
}
