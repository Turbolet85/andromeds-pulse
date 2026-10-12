//! `harness:boot-series` — the CI boot smoke's cycle, run N more times in
//! one job so a run reads several boots of equal source instead of one.
//!
//! Each boot gets its own data dir under the resolved one, its own display
//! server and empty per-user data and cache directories, and is recorded
//! with its ordinal (the smoke is ordinal 1). What a boot keeps is copied
//! file by file under the uploaded `logs/`; nothing of its `run/` is. The
//! verdict follows the `harness:status` shape: one pretty-JSON object on
//! stdout, exit 0 / 1 / 2, `cannot-evaluate` never passing. No value of an
//! environment variable and no path is printed in it or written with it.
//!
//! A boot that ends before the boot verb reads ready takes no settle
//! verdict. Its exit record and witness label are then read from what it
//! left in its own data dir, through the settle verdict's own readers.

use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::time::Duration;

use anyhow::Result;
use serde_json::{Value, json};

use crate::harness_status::{end_file, read_ended, read_pid, resolve_data_dir};
use crate::harness_witness::WITNESS_FILE;

const MIN_COUNT: u32 = 1;
const MAX_COUNT: u32 = 16;
/// The smoke step's own boot is ordinal 1.
const FIRST_ORDINAL: u32 = 2;
/// One cycle is a warm pre-build, a 10 s readiness poll, a 30 s settle
/// wait and a teardown; the bound only has to outlast a cold pre-build.
const CYCLE_TIMEOUT: Duration = Duration::from_secs(1200);

const SETTLE_VERDICT_FILE: &str = "harness-settled.json";
const SERIES_VERDICT_FILE: &str = "boot-series.json";
/// The boot verb's cleanup removes the pid file on a failed boot; the spawn
/// record and the exit record beside it stay.
const SPAWN_RECORD_FILE: &str = "andromeda-pulse.spawn";
const SMOKE_CYCLE: &str = "smoke";
const LOG_FAMILY_PREFIX: &str = "agent-latest.jsonl";
const KEPT_FILES: [&str; 4] = ["boot.log", SETTLE_VERDICT_FILE, "xvfb.log", WITNESS_FILE];
const LABEL_MAX_BYTES: usize = 48;

/// The smoke step's sequence with every exit recorded, so cleanup runs
/// whatever the verbs before it returned.
const CYCLE_SCRIPT: &str = r#"bash scripts/agent-run.sh boot; a=$?
b=-; c=-
if [ "$a" = 0 ]; then
  cargo xtask harness:settled; b=$?
  bash scripts/agent-run.sh status; c=$?
fi
bash scripts/agent-run.sh cleanup; d=$?
echo "series-cycle boot=$a settled=$b status=$c cleanup=$d"
"#;
const CYCLE_MARKER: &str = "series-cycle ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CycleExits {
    boot: i32,
    status: Option<i32>,
    cleanup: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BootClass {
    Settled,
    Ended,
    Other,
}

/// What a boot that took no settle verdict left in its own data dir.
#[derive(Debug, Clone, PartialEq, Eq)]
struct EndRead {
    ended: String,
    exit_witness: &'static str,
}

#[derive(Debug)]
struct BootRecord {
    ordinal: u32,
    cycle: &'static str,
    settle: Option<Value>,
    /// Read only when `settle` is absent.
    end: Option<EndRead>,
}

pub(crate) async fn run(count: u32) -> Result<ExitCode> {
    let data_dir = evaluable_data_dir(count);
    let mut records = Vec::new();
    let mut smoke = None;
    if let Some(data_dir) = &data_dir {
        smoke = smoke_record(data_dir);
        let root = workspace_root();
        for ordinal in FIRST_ORDINAL..FIRST_ORDINAL + count {
            let record = run_boot(&root, data_dir, ordinal).await;
            let stop = cycle_stops_series(record.cycle);
            records.push(record);
            if stop {
                break;
            }
        }
    }
    let verdict = decide_series(&records);
    let text = serde_json::to_string_pretty(&series_payload(verdict, &records, smoke.as_ref()))?;
    println!("{text}");
    let kept = data_dir.map(|dir| dir.join("logs").join(SERIES_VERDICT_FILE));
    if kept.is_some_and(|path| std::fs::write(path, format!("{text}\n")).is_err()) {
        eprintln!("harness:boot-series: could not write logs/{SERIES_VERDICT_FILE}");
    }
    Ok(ExitCode::from(exit_status(verdict)))
}

/// The resolved data dir when the series can be evaluated at all: Linux, a
/// count in range, `xvfb-run` on PATH, and no earlier series in the dir.
fn evaluable_data_dir(count: u32) -> Option<PathBuf> {
    if !cfg!(target_os = "linux") || !count_in_range(count) || !on_path("xvfb-run") {
        return None;
    }
    let data_dir = resolve_data_dir()?;
    std::fs::create_dir_all(data_dir.join("logs")).ok()?;
    let data_dir = std::fs::canonicalize(data_dir).ok()?;
    (!data_dir.join("series").exists()).then_some(data_dir)
}

fn on_path(name: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|path| std::env::split_paths(&path).any(|dir| dir.join(name).is_file()))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

async fn run_boot(root: &Path, data_dir: &Path, ordinal: u32) -> BootRecord {
    let name = format!("boot-{ordinal}");
    let boot_dir = data_dir.join("series").join(&name);
    let logs = boot_dir.join("logs");
    let made = [
        &logs,
        &boot_dir.join("xdg-data"),
        &boot_dir.join("xdg-cache"),
    ]
    .into_iter()
    .all(|dir| std::fs::create_dir_all(dir).is_ok());
    if !made {
        return BootRecord {
            ordinal,
            cycle: cycle_label(None, false),
            settle: None,
            end: None,
        };
    }

    eprintln!("harness:boot-series: boot {ordinal}");
    let (exits, timed_out) = run_cycle(root, &boot_dir).await;
    let settle = read_settle(&logs.join(SETTLE_VERDICT_FILE));
    let end = match settle {
        Some(_) => None,
        None => read_end(&boot_dir),
    };
    let record = BootRecord {
        ordinal,
        cycle: cycle_label(exits, timed_out),
        settle,
        end,
    };
    if copy_kept(&logs, &data_dir.join("logs").join("series").join(&name)).is_err() {
        eprintln!("harness:boot-series: could not keep the files of boot {ordinal}");
    }
    record
}

async fn run_cycle(root: &Path, boot_dir: &Path) -> (Option<CycleExits>, bool) {
    let mut cmd = tokio::process::Command::new("xvfb-run");
    cmd.current_dir(root)
        .arg("-a")
        .arg("-e")
        .arg(boot_dir.join("logs").join("xvfb.log"))
        .arg("--server-args=-screen 0 1280x1024x24")
        .args(["bash", "-c", CYCLE_SCRIPT])
        .env("ANDROMEDA_PULSE_DATA_DIR", boot_dir)
        .env("XDG_DATA_HOME", boot_dir.join("xdg-data"))
        .env("XDG_CACHE_HOME", boot_dir.join("xdg-cache"))
        .env_remove("ANDROMEDA_PULSE_PIDFILE")
        .env_remove("ANDROMEDA_PULSE_LOGFILE")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true);
    for (key, _) in std::env::vars_os() {
        if key.to_str().is_some_and(crate::is_cargo_run_injected) {
            cmd.env_remove(&key);
        }
    }
    let Ok(child) = cmd.spawn() else {
        return (None, false);
    };
    let wrapper = child.id();
    match tokio::time::timeout(CYCLE_TIMEOUT, child.wait_with_output()).await {
        Ok(Ok(output)) => {
            // The cycle's own lines go to stderr: stdout holds the verdict alone.
            let stdout = String::from_utf8_lossy(&output.stdout);
            eprint!("{stdout}");
            (parse_cycle_marker(&stdout), false)
        }
        Ok(Err(_)) => (None, false),
        Err(_) => {
            stop_timed_out_cycle(root, boot_dir, wrapper).await;
            (None, true)
        }
    }
}

/// A cycle past its bound: its display server and shell are ended by parent
/// pid (the wrapper itself was killed with the dropped wait), then the app
/// through the boot's own cleanup verb.
async fn stop_timed_out_cycle(root: &Path, boot_dir: &Path, wrapper: Option<u32>) {
    if let Some(pid) = wrapper {
        let _ = tokio::process::Command::new("pkill")
            .args(["-TERM", "-P", &pid.to_string()])
            .status()
            .await;
    }
    let _ = tokio::process::Command::new("bash")
        .current_dir(root)
        .args(["scripts/agent-run.sh", "cleanup"])
        .env("ANDROMEDA_PULSE_DATA_DIR", boot_dir)
        .env_remove("ANDROMEDA_PULSE_PIDFILE")
        .env_remove("ANDROMEDA_PULSE_LOGFILE")
        .status()
        .await;
}

fn count_in_range(count: u32) -> bool {
    (MIN_COUNT..=MAX_COUNT).contains(&count)
}

/// The script's last line: `series-cycle boot=A settled=B status=C cleanup=D`,
/// `-` for a verb a failed boot skipped.
fn parse_cycle_marker(stdout: &str) -> Option<CycleExits> {
    let line = stdout
        .lines()
        .rev()
        .find(|line| line.starts_with(CYCLE_MARKER))?;
    let mut fields = line[CYCLE_MARKER.len()..].split(' ');
    let mut field = |name: &str| {
        let value = fields.next()?.strip_prefix(name)?.strip_prefix('=')?;
        match value {
            "-" => Some(None),
            code => code.parse::<i32>().ok().map(Some),
        }
    };
    let boot = field("boot")??;
    field("settled")?;
    let status = field("status")?;
    let cleanup = field("cleanup")??;
    Some(CycleExits {
        boot,
        status,
        cleanup,
    })
}

/// How the cycle itself went, as one closed label; the app's own state is
/// the settle verdict's to say.
fn cycle_label(exits: Option<CycleExits>, timed_out: bool) -> &'static str {
    match exits {
        _ if timed_out => "timed-out",
        None => "no-record",
        Some(exits) if exits.cleanup != 0 => "cleanup-not-clean",
        Some(exits) if exits.boot != 0 => "boot-failed",
        Some(exits) if exits.status != Some(0) => "status-not-healthy",
        Some(_) => "complete",
    }
}

/// After any of these the next boot could meet this one's ports.
fn cycle_stops_series(cycle: &str) -> bool {
    matches!(cycle, "timed-out" | "no-record" | "cleanup-not-clean")
}

/// Whether the exit record of a boot that took no settle verdict is an end
/// the app made itself. `signal 15 (TERM)` and `signal 9 (KILL)` are the
/// boot verb's own cleanup ending an app still running at the readiness
/// timeout; the app re-raises a signal and never turns it into an exit code.
fn ended_by_itself(ended: Option<&str>) -> bool {
    !matches!(
        ended,
        None | Some("signal 15 (TERM)") | Some("signal 9 (KILL)")
    )
}

/// `ended` is the exit record of a boot that took no settle verdict.
fn classify(cycle: &str, settle_verdict: Option<&str>, ended: Option<&str>) -> BootClass {
    match settle_verdict {
        Some("ended") => BootClass::Ended,
        None if ended_by_itself(ended) => BootClass::Ended,
        Some("settled") if cycle == "complete" => BootClass::Settled,
        _ => BootClass::Other,
    }
}

fn settle_verdict(record: &BootRecord) -> Option<&str> {
    record
        .settle
        .as_ref()
        .and_then(|settle| settle.get("verdict"))
        .and_then(Value::as_str)
}

fn exit_record(record: &BootRecord) -> Option<&str> {
    match &record.settle {
        Some(_) => None,
        None => record.end.as_ref().map(|end| end.ended.as_str()),
    }
}

fn class_of(record: &BootRecord) -> BootClass {
    classify(record.cycle, settle_verdict(record), exit_record(record))
}

/// The series core — pure so the arms are pinnable without a process. It is
/// decided over the boots this verb ran; no boot at all cannot be evaluated.
fn decide_series(records: &[BootRecord]) -> &'static str {
    let class = |wanted| records.iter().any(|record| class_of(record) == wanted);
    if records.is_empty()
        || records
            .iter()
            .any(|record| cycle_stops_series(record.cycle))
    {
        "cannot-evaluate"
    } else if class(BootClass::Ended) {
        "self-ended"
    } else if class(BootClass::Other) {
        "not-all-settled"
    } else {
        "all-settled"
    }
}

fn exit_status(verdict: &str) -> u8 {
    match verdict {
        "all-settled" => 0,
        "cannot-evaluate" => 2,
        _ => 1,
    }
}

fn is_kept(name: &str) -> bool {
    name.starts_with(LOG_FAMILY_PREFIX) || KEPT_FILES.contains(&name)
}

/// Copies a boot's kept files, by name, from its own `logs/`.
fn copy_kept(from_logs: &Path, to: &Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(to)?;
    let mut copied = 0;
    for entry in std::fs::read_dir(from_logs)? {
        let entry = entry?;
        let name = entry.file_name();
        if entry.file_type()?.is_file() && name.to_str().is_some_and(is_kept) {
            std::fs::copy(entry.path(), to.join(&name))?;
            copied += 1;
        }
    }
    Ok(copied)
}

fn read_settle(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<Value>(&text)
        .ok()
        .filter(Value::is_object)
}

/// The exit record a boot left under `data_dir`, with the witness label for
/// the pid its spawn record names. No exit record reads as nothing.
fn read_end(data_dir: &Path) -> Option<EndRead> {
    let spawn_record = data_dir.join("run").join(SPAWN_RECORD_FILE);
    let ended = read_ended(&end_file(&spawn_record))?;
    let exit_witness =
        crate::harness_witness::label(Some(data_dir), read_pid(&spawn_record), Some(&ended));
    Some(EndRead {
        ended,
        exit_witness,
    })
}

/// The smoke step's own boot, read from the top data dir: its settle verdict
/// or, when it took none, the end it made itself.
fn smoke_record(data_dir: &Path) -> Option<BootRecord> {
    let settle = read_settle(&data_dir.join("logs").join(SETTLE_VERDICT_FILE));
    let end = match settle {
        Some(_) => None,
        None => read_end(data_dir).filter(|end| ended_by_itself(Some(&end.ended))),
    };
    (settle.is_some() || end.is_some()).then_some(BootRecord {
        ordinal: FIRST_ORDINAL - 1,
        cycle: SMOKE_CYCLE,
        settle,
        end,
    })
}

fn bounded_label(value: Option<&Value>) -> Value {
    match value.and_then(Value::as_str) {
        Some(text)
            if text.len() <= LABEL_MAX_BYTES
                && text.bytes().all(|byte| (0x20..=0x7e).contains(&byte)) =>
        {
            json!(text)
        }
        _ => Value::Null,
    }
}

/// One boot's entry: its ordinal, how its cycle went, and five members of
/// its settle verdict reduced to bounded labels and a count. A boot that
/// took no settle verdict has `ended` and `exit_witness` from `end` and the
/// other three null: no settle read ran.
fn boot_entry(ordinal: u32, cycle: &str, settle: Option<&Value>, end: Option<&EndRead>) -> Value {
    let member = |name: &str| settle.and_then(|settle| settle.get(name));
    let (ended, exit_witness) = match (settle, end) {
        (None, Some(end)) => (
            bounded_label(Some(&json!(end.ended))),
            bounded_label(Some(&json!(end.exit_witness))),
        ),
        _ => (
            bounded_label(member("ended")),
            bounded_label(member("exit_witness")),
        ),
    };
    json!({
        "ordinal": ordinal,
        "cycle": cycle,
        "verdict": bounded_label(member("verdict")),
        "ended": ended,
        "app_exit_record": bounded_label(member("app_exit_record")),
        "windows_settled": member("windows_settled").and_then(Value::as_u64),
        "exit_witness": exit_witness,
    })
}

/// The counts are of the boots this verb ran; the smoke's own boot, when the
/// data dir holds its settle verdict or its own end, is listed first as
/// ordinal 1.
fn series_payload(verdict: &str, records: &[BootRecord], smoke: Option<&BootRecord>) -> Value {
    let count = |wanted| {
        records
            .iter()
            .filter(|record| class_of(record) == wanted)
            .count()
    };
    let per_boot: Vec<Value> = smoke
        .into_iter()
        .chain(records)
        .map(|record| {
            boot_entry(
                record.ordinal,
                record.cycle,
                record.settle.as_ref(),
                record.end.as_ref(),
            )
        })
        .collect();
    json!({
        "verdict": verdict,
        "boots": records.len(),
        "settled": count(BootClass::Settled),
        "ended": count(BootClass::Ended),
        "other": count(BootClass::Other),
        "per_boot": per_boot,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn exits(boot: i32, status: Option<i32>, cleanup: i32) -> Option<CycleExits> {
        Some(CycleExits {
            boot,
            status,
            cleanup,
        })
    }

    fn settle(verdict: &str) -> Value {
        json!({
            "verdict": verdict,
            "pid": 4242,
            "ended": if verdict == "ended" { json!("exit 1") } else { Value::Null },
            "app_exit_record": "absent",
            "windows_settled": if verdict == "settled" { 4 } else { 0 },
            "display": "reachable",
            "session_bus": "reachable",
            "exit_witness": if verdict == "ended" { "exit-call" } else { "loaded" },
        })
    }

    fn boot(ordinal: u32, cycle: &'static str, verdict: Option<&str>) -> BootRecord {
        BootRecord {
            ordinal,
            cycle,
            settle: verdict.map(settle),
            end: None,
        }
    }

    /// A boot that took no settle verdict, with what its data dir held.
    fn unsettled(
        ordinal: u32,
        cycle: &'static str,
        ended: &str,
        exit_witness: &'static str,
    ) -> BootRecord {
        BootRecord {
            ordinal,
            cycle,
            settle: None,
            end: Some(EndRead {
                ended: ended.to_string(),
                exit_witness,
            }),
        }
    }

    const FIXTURE_PID: u32 = 374_465;

    /// A data dir as a failed boot leaves it: the spawn record (when
    /// `spawn`), the exit record and a witness file of `witness`.
    fn left_by_a_boot(spawn: bool, ended: &str, witness: &str) -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tmp");
        let run = dir.path().join("run");
        let logs = dir.path().join("logs");
        std::fs::create_dir_all(&run).expect("run");
        std::fs::create_dir_all(&logs).expect("logs");
        if spawn {
            std::fs::write(run.join(SPAWN_RECORD_FILE), format!("{FIXTURE_PID}\n")).expect("spawn");
        }
        std::fs::write(run.join("andromeda-pulse.exit"), format!("{ended}\n")).expect("exit");
        std::fs::write(logs.join(WITNESS_FILE), witness).expect("witness");
        dir
    }

    fn witness_of_an_exit_call() -> String {
        format!(
            "{{\"kind\":\"loaded\",\"pid\":{FIXTURE_PID},\"comm\":\"pulse-app\"}}\n{{\"kind\":\"end\",\"pid\":{FIXTURE_PID},\"tid\":{FIXTURE_PID},\"comm\":\"pulse-app\",\"call\":\"_exit\",\"code\":1,\"errno\":11,\"frames\":[]}}\n"
        )
    }

    fn keys(value: &Value) -> BTreeSet<&str> {
        value
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect()
    }

    #[test]
    fn the_count_is_taken_from_1_to_16() {
        assert!(!count_in_range(0));
        assert!(count_in_range(1));
        assert!(count_in_range(7));
        assert!(count_in_range(16));
        assert!(!count_in_range(17));
    }

    #[test]
    fn the_cycle_marker_is_read_from_its_own_line() {
        let stdout = "boot: ready (PID=7, data_dir=x)\ncleanup: clean\nseries-cycle boot=0 settled=1 status=1 cleanup=0\n";
        assert_eq!(parse_cycle_marker(stdout), exits(0, Some(1), 0));
        assert_eq!(
            parse_cycle_marker("series-cycle boot=1 settled=- status=- cleanup=0\n"),
            exits(1, None, 0)
        );
        for bad in [
            "",
            "cleanup: clean\n",
            "series-cycle boot=0 settled=0 status=0\n",
            "series-cycle boot=x settled=0 status=0 cleanup=0\n",
            "series-cycle boot=0 settled=0 status=0 cleanup=-\n",
            "echo series-cycle boot=0 settled=0 status=0 cleanup=0\n",
        ] {
            assert_eq!(parse_cycle_marker(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn a_cycle_whose_verbs_all_returned_zero_is_complete() {
        assert_eq!(cycle_label(exits(0, Some(0), 0), false), "complete");
        assert!(!cycle_stops_series("complete"));
    }

    #[test]
    fn a_failed_boot_or_an_unhealthy_status_is_named_and_the_series_goes_on() {
        assert_eq!(cycle_label(exits(1, None, 0), false), "boot-failed");
        assert_eq!(
            cycle_label(exits(0, Some(1), 0), false),
            "status-not-healthy"
        );
        assert_eq!(cycle_label(exits(0, None, 0), false), "status-not-healthy");
        assert!(!cycle_stops_series("boot-failed"));
        assert!(!cycle_stops_series("status-not-healthy"));
    }

    #[test]
    fn a_cycle_that_left_its_ports_in_doubt_stops_the_series() {
        assert_eq!(
            cycle_label(exits(0, Some(0), 1), false),
            "cleanup-not-clean"
        );
        assert_eq!(cycle_label(exits(1, None, 1), false), "cleanup-not-clean");
        assert_eq!(cycle_label(None, false), "no-record");
        assert_eq!(cycle_label(None, true), "timed-out");
        assert_eq!(cycle_label(exits(0, Some(0), 0), true), "timed-out");
        for cycle in ["cleanup-not-clean", "no-record", "timed-out"] {
            assert!(cycle_stops_series(cycle), "{cycle}");
        }
    }

    #[test]
    fn a_boot_is_settled_only_on_a_complete_cycle_and_ended_whatever_the_cycle() {
        assert_eq!(
            classify("complete", Some("settled"), None),
            BootClass::Settled
        );
        assert_eq!(
            classify("status-not-healthy", Some("settled"), None),
            BootClass::Other
        );
        assert_eq!(classify("complete", Some("ended"), None), BootClass::Ended);
        assert_eq!(
            classify("status-not-healthy", Some("ended"), None),
            BootClass::Ended
        );
        assert_eq!(
            classify("complete", Some("not-settled"), None),
            BootClass::Other
        );
        assert_eq!(
            classify("complete", Some("cannot-evaluate"), None),
            BootClass::Other
        );
        assert_eq!(classify("boot-failed", None, None), BootClass::Other);
    }

    #[test]
    fn a_boot_that_ended_by_itself_before_ready_is_ended_with_its_record_and_label() {
        assert_eq!(
            classify("boot-failed", None, Some("exit 1")),
            BootClass::Ended
        );
        let records = [
            boot(2, "complete", Some("settled")),
            unsettled(3, "boot-failed", "exit 1", "exit-call"),
        ];
        assert_eq!(class_of(&records[1]), BootClass::Ended);
        assert_eq!(decide_series(&records), "self-ended");
        let payload = series_payload("self-ended", &records, None);
        assert_eq!(payload["settled"], 1);
        assert_eq!(payload["ended"], 1);
        assert_eq!(payload["other"], 0);
        let entry = &payload["per_boot"][1];
        assert_eq!(entry["cycle"], "boot-failed");
        assert_eq!(entry["ended"], "exit 1");
        assert_eq!(entry["exit_witness"], "exit-call");
        assert_eq!(entry["verdict"], Value::Null);
        assert_eq!(entry["app_exit_record"], Value::Null);
        assert_eq!(entry["windows_settled"], Value::Null);
    }

    #[test]
    fn the_boot_verbs_own_term_and_kill_are_not_self_ends() {
        for ended in ["signal 15 (TERM)", "signal 9 (KILL)"] {
            assert!(!ended_by_itself(Some(ended)), "{ended}");
            let records = [
                boot(2, "complete", Some("settled")),
                unsettled(3, "boot-failed", ended, "loaded"),
            ];
            assert_eq!(class_of(&records[1]), BootClass::Other, "{ended}");
            assert_eq!(decide_series(&records), "not-all-settled", "{ended}");
            let payload = series_payload("not-all-settled", &records, None);
            assert_eq!(payload["ended"], 0);
            assert_eq!(payload["other"], 1);
            assert_eq!(payload["per_boot"][1]["ended"], ended);
            assert_eq!(payload["per_boot"][1]["exit_witness"], "loaded");
        }
    }

    #[test]
    fn another_signal_is_an_end_the_app_made_itself() {
        let records = [unsettled(2, "boot-failed", "signal 11 (SEGV)", "loaded")];
        assert_eq!(class_of(&records[0]), BootClass::Ended);
        assert_eq!(decide_series(&records), "self-ended");
    }

    #[test]
    fn no_exit_record_is_other_with_null_members() {
        assert!(!ended_by_itself(None));
        let records = [boot(2, "boot-failed", None)];
        assert_eq!(class_of(&records[0]), BootClass::Other);
        assert_eq!(decide_series(&records), "not-all-settled");
        let entry = boot_entry(2, "boot-failed", None, None);
        for member in [
            "verdict",
            "ended",
            "app_exit_record",
            "windows_settled",
            "exit_witness",
        ] {
            assert_eq!(entry[member], Value::Null, "{member}");
        }
    }

    #[test]
    fn a_settle_verdict_outranks_what_the_data_dir_held() {
        let record = BootRecord {
            end: Some(EndRead {
                ended: "exit 1".to_string(),
                exit_witness: "exit-call",
            }),
            ..boot(2, "complete", Some("settled"))
        };
        assert_eq!(class_of(&record), BootClass::Settled);
        let entry = boot_entry(2, "complete", record.settle.as_ref(), record.end.as_ref());
        assert_eq!(entry["verdict"], "settled");
        assert_eq!(entry["ended"], Value::Null);
        assert_eq!(entry["exit_witness"], "loaded");
    }

    #[test]
    fn a_boot_dir_is_read_to_its_exit_record_and_witness_label() {
        let dir = left_by_a_boot(true, "exit 1", &witness_of_an_exit_call());
        assert_eq!(
            read_end(dir.path()),
            Some(EndRead {
                ended: "exit 1".to_string(),
                exit_witness: "exit-call",
            })
        );

        let no_spawn_record = left_by_a_boot(false, "exit 1", &witness_of_an_exit_call());
        assert_eq!(
            read_end(no_spawn_record.path()),
            Some(EndRead {
                ended: "exit 1".to_string(),
                exit_witness: "unreadable",
            })
        );

        std::fs::remove_file(dir.path().join("run").join("andromeda-pulse.exit")).expect("remove");
        assert_eq!(read_end(dir.path()), None);
    }

    #[test]
    fn a_witness_file_outside_the_grammar_reads_unreadable_and_the_boot_is_still_ended() {
        let dir = left_by_a_boot(true, "exit 1", "not a witness line\n");
        let end = read_end(dir.path()).expect("an exit record");
        assert_eq!(end.exit_witness, "unreadable");
        let record = BootRecord {
            ordinal: 2,
            cycle: "boot-failed",
            settle: None,
            end: Some(end),
        };
        assert_eq!(class_of(&record), BootClass::Ended);
        assert_eq!(decide_series(&[record]), "self-ended");
    }

    #[test]
    fn every_boot_settled_is_all_settled() {
        let records = [
            boot(2, "complete", Some("settled")),
            boot(3, "complete", Some("settled")),
        ];
        assert_eq!(decide_series(&records), "all-settled");
        assert_eq!(exit_status("all-settled"), 0);
    }

    #[test]
    fn one_boot_that_ended_by_itself_is_self_ended() {
        let records = [
            boot(2, "complete", Some("settled")),
            boot(3, "status-not-healthy", Some("ended")),
            boot(4, "boot-failed", None),
        ];
        assert_eq!(decide_series(&records), "self-ended");
        assert_eq!(exit_status("self-ended"), 1);
    }

    #[test]
    fn a_boot_that_neither_settled_nor_ended_is_not_all_settled() {
        for other in [
            boot(3, "complete", Some("not-settled")),
            boot(3, "boot-failed", None),
            boot(3, "status-not-healthy", Some("settled")),
        ] {
            let records = [boot(2, "complete", Some("settled")), other];
            assert_eq!(decide_series(&records), "not-all-settled");
        }
        assert_eq!(exit_status("not-all-settled"), 1);
    }

    #[test]
    fn no_boot_or_a_cycle_that_stopped_the_series_cannot_be_evaluated() {
        assert_eq!(decide_series(&[]), "cannot-evaluate");
        for cycle in ["cleanup-not-clean", "no-record", "timed-out"] {
            let records = [
                boot(2, "complete", Some("settled")),
                boot(3, cycle, Some("ended")),
            ];
            assert_eq!(decide_series(&records), "cannot-evaluate", "{cycle}");
        }
        assert_eq!(exit_status("cannot-evaluate"), 2);
        assert_eq!(exit_status("anything-else"), 1);
    }

    #[test]
    fn the_verdict_carries_the_closed_member_sets() {
        let records = [
            boot(2, "complete", Some("settled")),
            boot(3, "status-not-healthy", Some("ended")),
            boot(4, "boot-failed", None),
        ];
        let payload = series_payload("self-ended", &records, None);
        assert_eq!(
            keys(&payload),
            BTreeSet::from(["verdict", "boots", "settled", "ended", "other", "per_boot"])
        );
        assert_eq!(payload["verdict"], "self-ended");
        assert_eq!(payload["boots"], 3);
        assert_eq!(payload["settled"], 1);
        assert_eq!(payload["ended"], 1);
        assert_eq!(payload["other"], 1);
        let per_boot = payload["per_boot"].as_array().expect("per_boot");
        assert_eq!(per_boot.len(), 3);
        for entry in per_boot {
            assert_eq!(
                keys(entry),
                BTreeSet::from([
                    "ordinal",
                    "cycle",
                    "verdict",
                    "ended",
                    "app_exit_record",
                    "windows_settled",
                    "exit_witness",
                ])
            );
        }
        assert_eq!(per_boot[1]["ordinal"], 3);
        assert_eq!(per_boot[1]["cycle"], "status-not-healthy");
        assert_eq!(per_boot[1]["verdict"], "ended");
        assert_eq!(per_boot[1]["ended"], "exit 1");
        assert_eq!(per_boot[1]["app_exit_record"], "absent");
        assert_eq!(per_boot[1]["windows_settled"], 0);
        assert_eq!(per_boot[1]["exit_witness"], "exit-call");
        assert_eq!(per_boot[2]["verdict"], Value::Null);
        assert_eq!(per_boot[2]["exit_witness"], Value::Null);
    }

    #[test]
    fn the_smoke_is_listed_as_ordinal_1_and_stays_out_of_the_counts() {
        let records = [boot(2, "complete", Some("settled"))];
        let smoke = boot(1, SMOKE_CYCLE, Some("ended"));
        let payload = series_payload("all-settled", &records, Some(&smoke));
        assert_eq!(payload["boots"], 1);
        assert_eq!(payload["settled"], 1);
        assert_eq!(payload["ended"], 0);
        let per_boot = payload["per_boot"].as_array().expect("per_boot");
        assert_eq!(per_boot.len(), 2);
        assert_eq!(per_boot[0]["ordinal"], 1);
        assert_eq!(per_boot[0]["cycle"], "smoke");
        assert_eq!(per_boot[0]["verdict"], "ended");
        assert_eq!(per_boot[0]["exit_witness"], "exit-call");
        assert_eq!(per_boot[1]["ordinal"], 2);
    }

    #[test]
    fn the_smokes_own_end_is_listed_as_ordinal_1_and_stays_out_of_the_counts() {
        let top = left_by_a_boot(true, "exit 1", &witness_of_an_exit_call());
        let smoke = smoke_record(top.path()).expect("the smoke's own end");
        assert_eq!(smoke.ordinal, 1);
        assert_eq!(smoke.cycle, "smoke");

        let records = [boot(2, "complete", Some("settled"))];
        assert_eq!(decide_series(&records), "all-settled");
        let payload = series_payload("all-settled", &records, Some(&smoke));
        assert_eq!(payload["boots"], 1);
        assert_eq!(payload["settled"], 1);
        assert_eq!(payload["ended"], 0);
        assert_eq!(payload["other"], 0);
        let per_boot = payload["per_boot"].as_array().expect("per_boot");
        assert_eq!(per_boot.len(), 2);
        assert_eq!(
            keys(&per_boot[0]),
            BTreeSet::from([
                "ordinal",
                "cycle",
                "verdict",
                "ended",
                "app_exit_record",
                "windows_settled",
                "exit_witness",
            ])
        );
        assert_eq!(per_boot[0]["ordinal"], 1);
        assert_eq!(per_boot[0]["cycle"], "smoke");
        assert_eq!(per_boot[0]["ended"], "exit 1");
        assert_eq!(per_boot[0]["exit_witness"], "exit-call");
        assert_eq!(per_boot[0]["verdict"], Value::Null);
        assert_eq!(per_boot[1]["ordinal"], 2);
    }

    #[test]
    fn a_top_dir_is_listed_from_its_settle_verdict_and_not_from_the_verbs_own_term() {
        let settled = left_by_a_boot(true, "exit 1", &witness_of_an_exit_call());
        std::fs::write(
            settled.path().join("logs").join(SETTLE_VERDICT_FILE),
            settle("settled").to_string(),
        )
        .expect("settle verdict");
        let smoke = smoke_record(settled.path()).expect("the smoke's settle verdict");
        assert_eq!(smoke.end, None);
        let entry = boot_entry(
            smoke.ordinal,
            smoke.cycle,
            smoke.settle.as_ref(),
            smoke.end.as_ref(),
        );
        assert_eq!(entry["verdict"], "settled");
        assert_eq!(entry["ended"], Value::Null);
        assert_eq!(entry["exit_witness"], "loaded");

        let ended_by_the_verb = left_by_a_boot(true, "signal 15 (TERM)", "");
        assert!(smoke_record(ended_by_the_verb.path()).is_none());
        let empty = tempfile::TempDir::new().expect("tmp");
        assert!(smoke_record(empty.path()).is_none());
    }

    #[test]
    fn an_entry_takes_bounded_labels_and_a_count_and_nothing_else() {
        let wide = json!({
            "verdict": "settled",
            "ended": "x".repeat(LABEL_MAX_BYTES + 1),
            "app_exit_record": "caf\u{e9}",
            "windows_settled": "four",
            "exit_witness": {"path": "/somewhere"},
        });
        let entry = boot_entry(2, "complete", Some(&wide), None);
        assert_eq!(entry["verdict"], "settled");
        assert_eq!(entry["ended"], Value::Null);
        assert_eq!(entry["app_exit_record"], Value::Null);
        assert_eq!(entry["windows_settled"], Value::Null);
        assert_eq!(entry["exit_witness"], Value::Null);
        let at_the_bound = json!({"ended": "x".repeat(LABEL_MAX_BYTES)});
        assert_eq!(
            boot_entry(2, "complete", Some(&at_the_bound), None)["ended"],
            "x".repeat(LABEL_MAX_BYTES)
        );
        let read_wide = EndRead {
            ended: "x".repeat(LABEL_MAX_BYTES + 1),
            exit_witness: "exit-call",
        };
        let unsettled_entry = boot_entry(2, "boot-failed", None, Some(&read_wide));
        assert_eq!(unsettled_entry["ended"], Value::Null);
        assert_eq!(unsettled_entry["exit_witness"], "exit-call");
    }

    #[test]
    fn the_kept_file_list_is_the_log_family_and_four_named_files() {
        for kept in [
            "agent-latest.jsonl",
            "agent-latest.jsonl.2026-10-10",
            "boot.log",
            "harness-settled.json",
            "xvfb.log",
            "exit-witness.jsonl",
        ] {
            assert!(is_kept(kept), "{kept}");
        }
        for dropped in [
            "build.log",
            "andromeda-pulse.pid",
            "andromeda-pulse.exit",
            "andromeda-pulse.spawn",
            "workspace-key",
            "boot-series.json",
            "exit-witness.jsonl.bak",
            "xboot.log",
        ] {
            assert!(!is_kept(dropped), "{dropped}");
        }
        assert_eq!(
            KEPT_FILES,
            [
                "boot.log",
                "harness-settled.json",
                "xvfb.log",
                "exit-witness.jsonl"
            ]
        );
    }

    #[test]
    fn a_boot_keeps_its_named_files_and_nothing_of_its_run_dir() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let boot_dir = dir.path().join("series").join("boot-2");
        let logs = boot_dir.join("logs");
        let run = boot_dir.join("run");
        std::fs::create_dir_all(&logs).expect("logs");
        std::fs::create_dir_all(&run).expect("run");
        std::fs::create_dir_all(logs.join("boot.log.d")).expect("a directory is not a file");
        for name in [
            "agent-latest.jsonl.2026-10-10",
            "boot.log",
            "harness-settled.json",
            "xvfb.log",
            "exit-witness.jsonl",
            "build.log",
        ] {
            std::fs::write(logs.join(name), name).expect("write");
        }
        std::fs::write(run.join("workspace-key"), "a path").expect("write");
        std::fs::write(run.join("andromeda-pulse.exit"), "exit 1").expect("write");

        let kept = dir.path().join("logs").join("series").join("boot-2");
        assert_eq!(copy_kept(&logs, &kept).expect("copy"), 5);
        let mut names: Vec<String> = std::fs::read_dir(&kept)
            .expect("kept dir")
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        assert_eq!(
            names,
            [
                "agent-latest.jsonl.2026-10-10",
                "boot.log",
                "exit-witness.jsonl",
                "harness-settled.json",
                "xvfb.log",
            ]
        );
        assert_eq!(
            std::fs::read_to_string(kept.join("boot.log")).expect("read"),
            "boot.log"
        );
    }

    #[test]
    fn the_cycle_script_runs_the_smoke_sequence_and_always_cleans_up() {
        let order = [
            "scripts/agent-run.sh boot",
            "cargo xtask harness:settled",
            "scripts/agent-run.sh status",
            "scripts/agent-run.sh cleanup",
            CYCLE_MARKER,
        ]
        .map(|needle| {
            let hits: Vec<usize> = CYCLE_SCRIPT
                .match_indices(needle)
                .map(|(at, _)| at)
                .collect();
            assert_eq!(hits.len(), 1, "exactly one `{needle}`");
            hits[0]
        });
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]));
        let cleanup_line = CYCLE_SCRIPT
            .lines()
            .find(|line| line.contains("scripts/agent-run.sh cleanup"))
            .expect("cleanup line");
        assert!(
            !cleanup_line.starts_with(' '),
            "cleanup sits outside the branch a failed boot skips"
        );
        for banned in ["set -e", "|| true", "sleep"] {
            assert!(!CYCLE_SCRIPT.contains(banned), "{banned}");
        }
    }
}
