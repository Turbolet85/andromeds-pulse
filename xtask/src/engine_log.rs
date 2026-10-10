//! The console engine's log, read by the harness (obs-plan §10 CI gates,
//! obs-plan §3 Heartbeat ticks): `check:engine-log` grades the log family of
//! a run that has ended, and `harness:engine-settled` holds a run until its
//! log holds what that check reads.
//!
//! `check:engine-log` prints one line per arm and a last line
//! `engine-log: PASS`, `engine-log: FAIL` or `engine-log: cannot-evaluate`;
//! exit 0 / 1 / 2. A FAIL on any arm is exit 1; with no FAIL, an arm that
//! cannot be evaluated is exit 2, so no arm reads PASS over an input it
//! cannot grade. The arms, in order, are family, program, panic,
//! heartbeat-gap, progress, process-end and budget. A line carries closed
//! labels, counts, a member's file name and a line number, never a record's
//! text, a path or an environment value.
//!
//! `harness:engine-settled` follows the `harness:status` shape, one
//! pretty-JSON verdict on stdout (`settled`, `ended`, `wrong-program`,
//! `not-settled`, `cannot-evaluate`) and exit 0 / 1 / 1 / 1 / 2. It writes
//! no file.

use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use anyhow::Result;
use serde_json::{Value, json};

use crate::ci_gates;
use crate::harness_status::{
    PROGRAM_UNKNOWN, Program, boot_record_program, end_file, pid_alive, read_ended, read_pid,
    resolve_paths,
};
use crate::ingest_progress;
use crate::perf_budget::{self, Arm, ArmState, MEMORY_MAX_BUDGET_BYTES};

const INGEST_TICK: &str = "ingest.tick";
const BUFFER_TICK: &str = ingest_progress::TARGET_BUFFER_TICK;
const CONNECTION_TICK: &str = "connection.tick";
/// The ticks both programs emit. `viz.tick` and `plugins.tick` are the
/// window's alone, so their absence from a console log is no stall.
const ENGINE_TICKS: [&str; 3] = [INGEST_TICK, BUFFER_TICK, CONNECTION_TICK];
const EXIT_TARGET: &str = "app.exit";
const MEMORY_TARGET: &str = "metric.buffer.memory_bytes";

const MAX_TICK_GAP_MS: i64 = 45_000;
/// The harness's exit record for a run ended by SIGKILL, which no process
/// can log.
const KILL_RECORD: &str = "signal 9 (KILL)";

/// One tick interval is 15 s, so a shorter window could never hold the
/// second tick of a target.
const MIN_SETTLE_TIMEOUT_SECONDS: u64 = 20;
const SETTLE_POLL_INTERVAL: Duration = Duration::from_millis(500);

const UNGRADED_BUDGET_LINE: &str = "engine-log: budget frame and snapshot not graded (a console engine has no producer for either)";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum State {
    Pass,
    Fail,
    CannotEvaluate,
}

impl State {
    fn word(self) -> &'static str {
        match self {
            State::Pass => "PASS",
            State::Fail => "FAIL",
            State::CannotEvaluate => "cannot-evaluate",
        }
    }

    pub(crate) fn exit_code(self) -> u8 {
        match self {
            State::Pass => 0,
            State::Fail => 1,
            State::CannotEvaluate => 2,
        }
    }
}

/// A FAIL outranks a reading that could not be evaluated, which outranks a
/// PASS.
fn worst(states: impl IntoIterator<Item = State>) -> State {
    let mut worst = State::Pass;
    for state in states {
        match state {
            State::Fail => return State::Fail,
            State::CannotEvaluate => worst = State::CannotEvaluate,
            State::Pass => {}
        }
    }
    worst
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArmReading {
    arm: &'static str,
    state: State,
    detail: String,
}

impl ArmReading {
    fn new(arm: &'static str, state: State, detail: impl Into<String>) -> Self {
        Self {
            arm,
            state,
            detail: detail.into(),
        }
    }

    fn line(&self) -> String {
        format!(
            "engine-log: {} {} ({})",
            self.arm,
            self.state.word(),
            self.detail
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Report {
    arms: Vec<ArmReading>,
}

impl Report {
    pub(crate) fn overall(&self) -> State {
        worst(self.arms.iter().map(|reading| reading.state))
    }

    /// The lines the verb prints, in order.
    pub(crate) fn lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for reading in &self.arms {
            lines.push(reading.line());
            if reading.arm == "budget" {
                lines.push(UNGRADED_BUDGET_LINE.to_owned());
            }
        }
        lines.push(format!("engine-log: {}", self.overall().word()));
        lines
    }
}

/// Grades the family under `log_dir`. `ended` is the harness's exit record
/// for the run, read only where the log holds no end of its own.
pub(crate) fn evaluate(log_dir: &Path, ended: Option<&str>) -> Report {
    let family = ci_gates::evaluate(log_dir);
    if family == ci_gates::Verdict::CannotEvaluate {
        return Report {
            arms: vec![family_arm(&family)],
        };
    }
    let lines = perf_budget::read_family(log_dir).unwrap_or_default();
    Report {
        arms: vec![
            family_arm(&family),
            program_arm(&lines),
            panic_arm(&family),
            heartbeat_arm(&lines),
            progress_arm(&lines),
            process_end_arm(&lines, ended),
            budget_arm(&lines),
        ],
    }
}

fn target_of(record: &Value) -> Option<&str> {
    record.get("target").and_then(Value::as_str)
}

fn timestamp_ms(record: &Value) -> Option<i64> {
    let raw = record.get("timestamp").and_then(Value::as_str)?;
    chrono::DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|at| at.timestamp_millis())
}

fn family_arm(family: &ci_gates::Verdict) -> ArmReading {
    let arm = "family";
    match family {
        ci_gates::Verdict::CannotEvaluate => ArmReading::new(
            arm,
            State::CannotEvaluate,
            "no agent-latest.jsonl* file in the log dir",
        ),
        ci_gates::Verdict::NoRecords => {
            ArmReading::new(arm, State::Fail, "log files present but hold no record")
        }
        ci_gates::Verdict::Panic { records, files, .. }
        | ci_gates::Verdict::Pass { records, files } => ArmReading::new(
            arm,
            State::Pass,
            format!("{records} records across {files} file(s)"),
        ),
    }
}

/// One cycle per data dir: a gap read across two boots would be false, so a
/// second boot record cannot be graded.
fn program_arm(lines: &[Value]) -> ArmReading {
    let arm = "program";
    let programs: Vec<&str> = lines.iter().filter_map(boot_record_program).collect();
    match programs.as_slice() {
        [] => ArmReading::new(arm, State::CannotEvaluate, "no app.boot.engine record"),
        [program] if *program == Program::Console.label() => ArmReading::new(
            arm,
            State::Pass,
            "one app.boot.engine record, reading console",
        ),
        [program] if *program == Program::Window.label() => {
            ArmReading::new(arm, State::Fail, "the app.boot.engine record reads window")
        }
        [_] => ArmReading::new(
            arm,
            State::CannotEvaluate,
            "the app.boot.engine record names no known program",
        ),
        several => ArmReading::new(
            arm,
            State::CannotEvaluate,
            format!(
                "{} app.boot.engine records, one cycle per data dir",
                several.len()
            ),
        ),
    }
}

/// The panic read is `ci_gates`'s own, taken from its verdict.
fn panic_arm(family: &ci_gates::Verdict) -> ArmReading {
    let arm = "panic";
    match family {
        ci_gates::Verdict::Panic { member, line, .. } => ArmReading::new(
            arm,
            State::Fail,
            format!("app.panic.fatal at {member}:{line}"),
        ),
        ci_gates::Verdict::Pass { .. } => {
            ArmReading::new(arm, State::Pass, "no app.panic.fatal record at ERROR")
        }
        ci_gates::Verdict::NoRecords | ci_gates::Verdict::CannotEvaluate => {
            ArmReading::new(arm, State::CannotEvaluate, "no record to read")
        }
    }
}

fn tick_gap(lines: &[Value], target: &str) -> (State, String) {
    let stamps: Vec<Option<i64>> = lines
        .iter()
        .filter(|record| target_of(record) == Some(target))
        .map(timestamp_ms)
        .collect();
    let n = stamps.len();
    if n < 2 {
        return (
            State::CannotEvaluate,
            format!("{target} {n} record(s), two needed"),
        );
    }
    let Some(stamps) = stamps.into_iter().collect::<Option<Vec<i64>>>() else {
        return (
            State::CannotEvaluate,
            format!("{target} holds a record with no readable timestamp"),
        );
    };
    let largest = stamps
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .max()
        .unwrap_or(0);
    if largest > MAX_TICK_GAP_MS {
        (
            State::Fail,
            format!("{target} gap {largest} ms > {MAX_TICK_GAP_MS} ms"),
        )
    } else {
        (
            State::Pass,
            format!("{target} max gap {largest} ms (n={n})"),
        )
    }
}

fn heartbeat_arm(lines: &[Value]) -> ArmReading {
    let readings: Vec<(State, String)> = ENGINE_TICKS
        .iter()
        .map(|target| tick_gap(lines, target))
        .collect();
    let state = worst(readings.iter().map(|(state, _)| *state));
    let detail: Vec<&str> = readings.iter().map(|(_, detail)| detail.as_str()).collect();
    ArmReading::new("heartbeat-gap", state, detail.join("; "))
}

/// `ingest_progress`'s own read, with the two readings it leaves open made
/// required: a log with no `buffer.tick` cannot be graded, and a run in which
/// no span landed fails.
fn progress_arm(lines: &[Value]) -> ArmReading {
    let arm = "progress";
    match ingest_progress::evaluate(lines) {
        ingest_progress::Verdict::Fail {
            stalled_seconds, ..
        } => ArmReading::new(
            arm,
            State::Fail,
            format!("buffer consumer stall announced, stalled {stalled_seconds} s"),
        ),
        ingest_progress::Verdict::Neutral(_) => {
            ArmReading::new(arm, State::CannotEvaluate, "no buffer.tick record")
        }
        ingest_progress::Verdict::Pass { ticks, .. } => {
            let largest = lines
                .iter()
                .filter(|record| target_of(record) == Some(BUFFER_TICK))
                .filter_map(|record| {
                    record
                        .get("fields")
                        .and_then(|fields| fields.get("rows_ingested"))
                        .and_then(Value::as_u64)
                })
                .max();
            match largest {
                None => ArmReading::new(
                    arm,
                    State::CannotEvaluate,
                    format!("no numeric rows_ingested on {ticks} buffer.tick record(s)"),
                ),
                Some(0) => ArmReading::new(
                    arm,
                    State::Fail,
                    format!(
                        "no span landed, largest rows_ingested 0 over {ticks} buffer.tick record(s)"
                    ),
                ),
                Some(rows) => ArmReading::new(
                    arm,
                    State::Pass,
                    format!("{ticks} buffer.tick record(s), largest rows_ingested {rows}"),
                ),
            }
        }
    }
}

fn process_end_arm(lines: &[Value], ended: Option<&str>) -> ArmReading {
    let arm = "process-end";
    let exits: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, record)| target_of(record) == Some(EXIT_TARGET))
        .map(|(idx, _)| idx)
        .collect();
    match exits.as_slice() {
        [] if ended == Some(KILL_RECORD) => ArmReading::new(
            arm,
            State::Fail,
            "unloggable-end: no app.exit record, the run was ended by signal 9",
        ),
        [] => ArmReading::new(arm, State::Fail, "end-not-recorded: no app.exit record"),
        [only] if only + 1 == lines.len() => ArmReading::new(
            arm,
            State::Pass,
            "one app.exit record, the last of the family",
        ),
        [only] => ArmReading::new(
            arm,
            State::Fail,
            format!(
                "{} record(s) after the app.exit record",
                lines.len() - only - 1
            ),
        ),
        several => ArmReading::new(
            arm,
            State::Fail,
            format!("{} app.exit records", several.len()),
        ),
    }
}

/// The memory arm of `perf_budget`, required: a log with no sample to grade
/// fails.
fn budget_arm(lines: &[Value]) -> ArmReading {
    let arm = "budget";
    let result = perf_budget::grade_arm(lines, Arm::Memory);
    let populated = result.populated.unwrap_or(0);
    match result.state {
        ArmState::Pass { stat, n } => ArmReading::new(
            arm,
            State::Pass,
            format!(
                "memory max {stat:.0} B <= {MEMORY_MAX_BUDGET_BYTES:.0} B, n={n}, populated {populated}"
            ),
        ),
        ArmState::Fail { stat, n } => ArmReading::new(
            arm,
            State::Fail,
            format!(
                "memory max {stat:.0} B > {MEMORY_MAX_BUDGET_BYTES:.0} B, n={n}, populated {populated}"
            ),
        ),
        ArmState::Unreadable { unreadable, n } => ArmReading::new(
            arm,
            State::Fail,
            format!("{unreadable} of {n} memory sample(s) carry a non-numeric value"),
        ),
        ArmState::Neutral { reason } => ArmReading::new(
            arm,
            State::Fail,
            format!("no memory sample to grade, {reason}"),
        ),
    }
}

/// `cargo xtask check:engine-log`. Exit 0 PASS, 1 FAIL, 2 cannot-evaluate.
pub(crate) fn run_check() -> ExitCode {
    let report = match resolve_paths() {
        Some((pidfile, log_base)) => {
            let log_dir = log_base.parent().unwrap_or_else(|| Path::new("."));
            evaluate(log_dir, read_ended(&end_file(&pidfile)).as_deref())
        }
        None => Report {
            arms: vec![ArmReading::new(
                "family",
                State::CannotEvaluate,
                "no data dir resolved",
            )],
        },
    };
    for line in report.lines() {
        println!("{line}");
    }
    ExitCode::from(report.overall().exit_code())
}

/// What the log holds of the readings `check:engine-log` needs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SettleEvidence {
    program: &'static str,
    ingest_ticks: usize,
    buffer_ticks: usize,
    connection_ticks: usize,
    memory_samples_populated: usize,
}

impl Default for SettleEvidence {
    fn default() -> Self {
        Self {
            program: PROGRAM_UNKNOWN,
            ingest_ticks: 0,
            buffer_ticks: 0,
            connection_ticks: 0,
            memory_samples_populated: 0,
        }
    }
}

impl SettleEvidence {
    /// Whether the log holds every reading the check needs.
    fn gradeable(&self) -> bool {
        self.program == Program::Console.label()
            && self.ingest_ticks >= 2
            && self.buffer_ticks >= 2
            && self.connection_ticks >= 2
            && self.memory_samples_populated >= 1
    }
}

fn settle_evidence(lines: &[String]) -> SettleEvidence {
    let mut evidence = SettleEvidence::default();
    for record in lines
        .iter()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
    {
        if let Some(program) = boot_record_program(&record) {
            evidence.program = program;
        }
        match target_of(&record) {
            Some(INGEST_TICK) => evidence.ingest_ticks += 1,
            Some(BUFFER_TICK) => evidence.buffer_ticks += 1,
            Some(CONNECTION_TICK) => evidence.connection_ticks += 1,
            Some(MEMORY_TARGET) => {
                let value = record
                    .get("fields")
                    .and_then(|fields| fields.get("value"))
                    .and_then(Value::as_f64);
                if value.is_some_and(|value| value != 0.0) {
                    evidence.memory_samples_populated += 1;
                }
            }
            _ => {}
        }
    }
    evidence
}

fn read_settle_evidence(log_base: &Path) -> SettleEvidence {
    crate::smoke::read_jsonl_lines(log_base)
        .map(|lines| settle_evidence(&lines))
        .unwrap_or_default()
}

fn timeout_supports_verdict(timeout_seconds: u64) -> bool {
    timeout_seconds >= MIN_SETTLE_TIMEOUT_SECONDS
}

/// The settle core, pure so the arms are pinnable without a process. `None`
/// means keep polling. A dead pid is `ended` whatever the log holds.
fn decide_engine_settled(
    pid: Option<u32>,
    alive: Option<bool>,
    evidence: &SettleEvidence,
    timed_out: bool,
) -> Option<&'static str> {
    match (pid, alive) {
        (None, _) | (_, None) => Some("cannot-evaluate"),
        (Some(_), Some(false)) => Some("ended"),
        (Some(_), Some(true)) if evidence.program == Program::Window.label() => {
            Some("wrong-program")
        }
        (Some(_), Some(true)) if evidence.gradeable() => Some("settled"),
        (Some(_), Some(true)) if timed_out => Some("not-settled"),
        (Some(_), Some(true)) => None,
    }
}

fn wait_for_settle(
    pidfile: &Path,
    log_base: &Path,
    timeout: Duration,
) -> (&'static str, Option<u32>, SettleEvidence) {
    let started = Instant::now();
    loop {
        let pid = read_pid(pidfile);
        let alive = pid.and_then(pid_alive);
        let evidence = read_settle_evidence(log_base);
        let timed_out = started.elapsed() >= timeout;
        if let Some(verdict) = decide_engine_settled(pid, alive, &evidence, timed_out) {
            return (verdict, pid, evidence);
        }
        std::thread::sleep(SETTLE_POLL_INTERVAL);
    }
}

fn settle_exit(verdict: &str) -> u8 {
    match verdict {
        "settled" => 0,
        "cannot-evaluate" => 2,
        _ => 1,
    }
}

fn settled_payload(verdict: &str, pid: Option<u32>, evidence: &SettleEvidence) -> Value {
    json!({
        "verdict": verdict,
        "pid": pid,
        "program": evidence.program,
        "ingest_ticks": evidence.ingest_ticks,
        "buffer_ticks": evidence.buffer_ticks,
        "connection_ticks": evidence.connection_ticks,
        "memory_samples_populated": evidence.memory_samples_populated,
    })
}

/// `cargo xtask harness:engine-settled`.
pub(crate) fn run_settled(timeout_seconds: u64) -> Result<ExitCode> {
    let (verdict, pid, evidence) = match resolve_paths() {
        Some((pidfile, log_base)) if timeout_supports_verdict(timeout_seconds) => {
            wait_for_settle(&pidfile, &log_base, Duration::from_secs(timeout_seconds))
        }
        _ => ("cannot-evaluate", None, SettleEvidence::default()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&settled_payload(verdict, pid, &evidence))?
    );
    Ok(ExitCode::from(settle_exit(verdict)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const MEMBER: &str = "agent-latest.jsonl.2026-10-10";
    const CANARY: &str = "canary-record-text";
    const TERM_RECORD: &str = "signal 15 (TERM)";
    /// 2026-10-10T00:00:00Z.
    const BASE_MS: i64 = 1_791_590_400_000;

    fn record(at_ms: i64, level: &str, target: &str, fields: Value) -> String {
        let timestamp = chrono::DateTime::from_timestamp_millis(BASE_MS + at_ms)
            .expect("in range")
            .to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
        json!({"timestamp": timestamp, "level": level, "target": target, "fields": fields})
            .to_string()
    }

    fn boot(program: &str) -> String {
        record(
            0,
            "WARN",
            "app.boot.engine",
            json!({"program": program, "interpretation": "none", "reason": "deterministic_gate_unset"}),
        )
    }

    fn tick(at_ms: i64, target: &str) -> String {
        record(at_ms, "INFO", target, json!({"message": "heartbeat"}))
    }

    fn buffer_tick(at_ms: i64, rows: u64, delta: u64) -> String {
        record(
            at_ms,
            "INFO",
            BUFFER_TICK,
            json!({"rows_ingested": rows, "rows_ingested_delta": delta}),
        )
    }

    fn memory(at_ms: i64, value: Value) -> String {
        record(at_ms, "INFO", MEMORY_TARGET, json!({"value": value}))
    }

    fn exit_record(at_ms: i64) -> String {
        record(
            at_ms,
            "WARN",
            EXIT_TARGET,
            json!({"exit_class": "signal", "exit_code": 0, "exit_code_known": false, "signal": "sigterm"}),
        )
    }

    fn panic_record(at_ms: i64, level: &str) -> String {
        record(
            at_ms,
            level,
            "app.panic.fatal",
            json!({"message": CANARY, "location": "src/console.rs:1"}),
        )
    }

    fn stalled(at_ms: i64, reason: &str) -> String {
        record(
            at_ms,
            "WARN",
            ingest_progress::TARGET_STALLED,
            json!({"reason": reason, "consequence": CANARY, "stalled_seconds": 450}),
        )
    }

    /// Ten records: the boot, two ticks of each engine target 15 s apart,
    /// two memory samples (the second populated) and the end, last.
    fn complete() -> Vec<String> {
        vec![
            boot("console"),
            tick(10, INGEST_TICK),
            buffer_tick(11, 0, 0),
            memory(12, json!(0)),
            tick(13, CONNECTION_TICK),
            tick(15_010, INGEST_TICK),
            buffer_tick(15_011, 360, 360),
            memory(15_012, json!(92_160)),
            tick(15_013, CONNECTION_TICK),
            exit_record(20_000),
        ]
    }

    fn without(lines: &[String], needle: &str) -> Vec<String> {
        let kept: Vec<String> = lines
            .iter()
            .filter(|line| !line.contains(needle))
            .cloned()
            .collect();
        assert!(kept.len() < lines.len(), "nothing held `{needle}`");
        kept
    }

    /// `extra` placed directly before the end record, so the end stays last.
    fn with_before_end(extra: &[String]) -> Vec<String> {
        let mut lines = complete();
        let end = lines.pop().expect("the end record");
        lines.extend(extra.iter().cloned());
        lines.push(end);
        lines
    }

    fn family(lines: &[String]) -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tmp");
        let mut content = lines.join("\n");
        content.push('\n');
        std::fs::write(dir.path().join(MEMBER), content).expect("write the member");
        dir
    }

    fn graded(lines: &[String]) -> Report {
        evaluate(family(lines).path(), Some(TERM_RECORD))
    }

    fn reading<'a>(report: &'a Report, arm: &str) -> &'a ArmReading {
        report
            .arms
            .iter()
            .find(|reading| reading.arm == arm)
            .unwrap_or_else(|| panic!("no `{arm}` arm in {report:?}"))
    }

    fn state(report: &Report, arm: &str) -> State {
        reading(report, arm).state
    }

    #[test]
    fn a_complete_console_log_passes_every_arm_in_order() {
        let report = graded(&complete());
        let arms: Vec<&str> = report.arms.iter().map(|reading| reading.arm).collect();
        assert_eq!(
            arms,
            [
                "family",
                "program",
                "panic",
                "heartbeat-gap",
                "progress",
                "process-end",
                "budget"
            ]
        );
        for reading in &report.arms {
            assert_eq!(reading.state, State::Pass, "{reading:?}");
            assert!(!reading.detail.is_empty(), "{reading:?}");
        }
        assert_eq!(report.overall(), State::Pass);
        assert_eq!(report.overall().exit_code(), 0);
        assert_eq!(
            report.lines().last().map(String::as_str),
            Some("engine-log: PASS")
        );
    }

    #[test]
    fn an_unrelated_file_beside_the_family_changes_nothing() {
        let dir = family(&complete());
        let alone = evaluate(dir.path(), Some(TERM_RECORD));
        std::fs::write(
            dir.path().join("boot.log"),
            format!("{}\n{}\n", boot("window"), panic_record(1, "ERROR")),
        )
        .expect("write a bystander");
        let beside = evaluate(dir.path(), Some(TERM_RECORD));
        assert_eq!(alone, beside);
        assert_eq!(beside.overall(), State::Pass);
    }

    fn assert_family_cannot_be_evaluated(report: &Report) {
        assert_eq!(report.arms.len(), 1, "nothing else is graded: {report:?}");
        assert_eq!(state(report, "family"), State::CannotEvaluate);
        assert_eq!(report.overall(), State::CannotEvaluate);
        assert_eq!(report.overall().exit_code(), 2);
        let lines = report.lines();
        assert_eq!(
            lines.last().map(String::as_str),
            Some("engine-log: cannot-evaluate")
        );
        assert!(
            !lines.iter().any(|line| line.contains("PASS")),
            "lines: {lines:?}"
        );
    }

    #[test]
    fn an_absent_log_dir_cannot_be_evaluated() {
        let dir = tempfile::TempDir::new().expect("tmp");
        assert_family_cannot_be_evaluated(&evaluate(&dir.path().join("logs"), None));
    }

    #[test]
    fn a_log_dir_with_no_family_member_cannot_be_evaluated() {
        let dir = tempfile::TempDir::new().expect("tmp");
        std::fs::write(dir.path().join("boot.log"), complete().join("\n")).expect("write");
        assert_family_cannot_be_evaluated(&evaluate(dir.path(), None));
    }

    #[test]
    fn members_that_hold_no_record_fail() {
        let dir = tempfile::TempDir::new().expect("tmp");
        std::fs::write(dir.path().join(MEMBER), "\n\n").expect("write");
        let report = evaluate(dir.path(), None);
        assert_eq!(state(&report, "family"), State::Fail);
        assert_eq!(report.overall(), State::Fail);
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_boot_record_that_reads_window_fails() {
        let mut lines = complete();
        lines[0] = boot("window");
        let report = graded(&lines);
        assert_eq!(state(&report, "program"), State::Fail);
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_log_with_no_boot_record_cannot_be_evaluated() {
        let report = graded(&without(&complete(), "app.boot.engine"));
        assert_eq!(state(&report, "program"), State::CannotEvaluate);
        assert_eq!(report.overall().exit_code(), 2);
    }

    #[test]
    fn two_boots_in_one_family_cannot_be_evaluated() {
        let report = graded(&with_before_end(&[boot("console")]));
        assert_eq!(state(&report, "program"), State::CannotEvaluate);
        assert_eq!(report.overall().exit_code(), 2);
    }

    #[test]
    fn a_boot_record_naming_no_known_program_cannot_be_evaluated() {
        let mut lines = complete();
        lines[0] = boot("<redacted>");
        assert_eq!(state(&graded(&lines), "program"), State::CannotEvaluate);
    }

    #[test]
    fn a_panic_record_at_error_fails_by_file_name_and_line() {
        let report = graded(&with_before_end(&[panic_record(16_000, "ERROR")]));
        let panic = reading(&report, "panic");
        assert_eq!(panic.state, State::Fail);
        assert!(
            panic.detail.contains(&format!("{MEMBER}:10")),
            "the tenth line of the member: {panic:?}"
        );
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_panic_target_record_below_error_passes() {
        let report = graded(&with_before_end(&[panic_record(16_000, "WARN")]));
        assert_eq!(state(&report, "panic"), State::Pass);
    }

    #[test]
    fn a_gap_over_45_seconds_between_two_ticks_of_one_target_fails_with_no_error_record() {
        for target in ENGINE_TICKS {
            let mut lines = with_before_end(&[]);
            // The target's second tick moves from 15 s to 46 s after its
            // first; every other record stays where it was.
            let late = if target == BUFFER_TICK {
                buffer_tick(46_011, 360, 360)
            } else {
                tick(46_011, target)
            };
            let second = lines
                .iter()
                .rposition(|line| line.contains(&format!("\"target\":\"{target}\"")))
                .expect("the target's second tick");
            lines[second] = late;
            assert!(
                !lines.iter().any(|line| line.contains("\"ERROR\"")),
                "the defect is block-shaped: the red input holds no ERROR record"
            );
            let report = graded(&lines);
            let gap = reading(&report, "heartbeat-gap");
            assert_eq!(gap.state, State::Fail, "{target}: {gap:?}");
            assert!(gap.detail.contains(target), "{gap:?}");
            assert!(gap.detail.contains(" ms > 45000 ms"), "{gap:?}");
            assert_eq!(report.overall().exit_code(), 1);
        }
    }

    #[test]
    fn a_gap_of_exactly_45_seconds_passes() {
        let mut lines = complete();
        lines[5] = tick(45_010, INGEST_TICK);
        let report = graded(&lines);
        let gap = reading(&report, "heartbeat-gap");
        assert_eq!(gap.state, State::Pass, "{gap:?}");
        assert!(gap.detail.contains("45000 ms"), "{gap:?}");
    }

    #[test]
    fn fewer_than_two_ticks_of_a_target_cannot_be_evaluated() {
        for target in ENGINE_TICKS {
            let mut lines = complete();
            let second = lines
                .iter()
                .rposition(|line| line.contains(&format!("\"target\":\"{target}\"")))
                .expect("the target's second tick");
            lines.remove(second);
            let report = graded(&lines);
            let gap = reading(&report, "heartbeat-gap");
            assert_eq!(gap.state, State::CannotEvaluate, "{target}: {gap:?}");
            assert!(gap.detail.contains(target), "{gap:?}");
        }
        let none = graded(&without(&complete(), "\"target\":\"ingest.tick\""));
        assert_eq!(state(&none, "heartbeat-gap"), State::CannotEvaluate);
        assert_eq!(none.overall().exit_code(), 2);
    }

    #[test]
    fn a_tick_with_no_readable_timestamp_cannot_be_evaluated() {
        let mut lines = complete();
        lines[5] = json!({"level": "INFO", "target": INGEST_TICK, "fields": {}}).to_string();
        assert_eq!(
            state(&graded(&lines), "heartbeat-gap"),
            State::CannotEvaluate
        );
    }

    #[test]
    fn viz_and_plugins_ticks_are_never_graded() {
        // One tick of each, and for viz a second one ten minutes later: a
        // graded target would read cannot-evaluate or FAIL.
        let report = graded(&with_before_end(&[
            tick(14, "viz.tick"),
            tick(15, "plugins.tick"),
            tick(615_000, "viz.tick"),
        ]));
        let gap = reading(&report, "heartbeat-gap");
        assert_eq!(gap.state, State::Pass, "{gap:?}");
        assert!(
            !gap.detail.contains("viz") && !gap.detail.contains("plugins"),
            "{gap:?}"
        );
    }

    #[test]
    fn a_stall_announcement_fails() {
        let report = graded(&with_before_end(&[stalled(
            16_000,
            "rows_static_while_channel_queued",
        )]));
        assert_eq!(state(&report, "progress"), State::Fail);
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_recovery_announcement_passes() {
        let report = graded(&with_before_end(&[stalled(16_000, "recovered")]));
        assert_eq!(state(&report, "progress"), State::Pass);
    }

    #[test]
    fn a_run_in_which_no_span_landed_fails() {
        let mut lines = complete();
        lines[6] = buffer_tick(15_011, 0, 0);
        let report = graded(&lines);
        let progress = reading(&report, "progress");
        assert_eq!(progress.state, State::Fail, "{progress:?}");
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_log_with_no_buffer_tick_cannot_be_evaluated_for_progress() {
        let report = graded(&without(&complete(), "\"target\":\"buffer.tick\""));
        assert_eq!(state(&report, "progress"), State::CannotEvaluate);
        assert_eq!(report.overall().exit_code(), 2);
    }

    #[test]
    fn buffer_ticks_with_no_numeric_row_count_cannot_be_evaluated_for_progress() {
        let mut lines = complete();
        for idx in [2, 6] {
            lines[idx] = record(
                if idx == 2 { 11 } else { 15_011 },
                "INFO",
                BUFFER_TICK,
                json!({"rows_ingested": "<redacted>"}),
            );
        }
        assert_eq!(state(&graded(&lines), "progress"), State::CannotEvaluate);
    }

    #[test]
    fn a_run_ended_by_sigkill_with_no_exit_record_is_unloggable_end() {
        let lines = without(&complete(), "app.exit");
        let report = evaluate(family(&lines).path(), Some(KILL_RECORD));
        let end = reading(&report, "process-end");
        assert_eq!(end.state, State::Fail);
        assert!(end.detail.starts_with("unloggable-end"), "{end:?}");
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_run_with_no_exit_record_otherwise_is_end_not_recorded() {
        let lines = without(&complete(), "app.exit");
        for ended in [None, Some(TERM_RECORD), Some("exit 0")] {
            let report = evaluate(family(&lines).path(), ended);
            let end = reading(&report, "process-end");
            assert_eq!(end.state, State::Fail, "{ended:?}");
            assert!(end.detail.starts_with("end-not-recorded"), "{end:?}");
        }
    }

    #[test]
    fn two_exit_records_fail() {
        let mut lines = complete();
        lines.push(exit_record(20_001));
        assert_eq!(state(&graded(&lines), "process-end"), State::Fail);
    }

    #[test]
    fn a_record_after_the_exit_record_fails() {
        let mut lines = complete();
        lines.push(tick(20_001, INGEST_TICK));
        let report = graded(&lines);
        assert_eq!(state(&report, "process-end"), State::Fail);
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_kill_record_does_not_excuse_a_log_that_holds_its_end() {
        let report = evaluate(family(&complete()).path(), Some(KILL_RECORD));
        assert_eq!(state(&report, "process-end"), State::Pass);
    }

    #[test]
    fn a_memory_sample_over_budget_fails() {
        let mut lines = complete();
        lines[7] = memory(15_012, json!(512_000_001_u64));
        let report = graded(&lines);
        assert_eq!(state(&report, "budget"), State::Fail);
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn a_memory_sample_at_the_budget_passes() {
        let mut lines = complete();
        lines[7] = memory(15_012, json!(512_000_000_u64));
        assert_eq!(state(&graded(&lines), "budget"), State::Pass);
    }

    #[test]
    fn a_log_with_no_memory_sample_fails_the_budget() {
        let report = graded(&without(&complete(), MEMORY_TARGET));
        assert_eq!(state(&report, "budget"), State::Fail);
        assert_eq!(report.overall().exit_code(), 1);
    }

    #[test]
    fn only_zero_memory_samples_fail_the_budget() {
        let mut lines = complete();
        lines[7] = memory(15_012, json!(0));
        assert_eq!(state(&graded(&lines), "budget"), State::Fail);
    }

    #[test]
    fn a_non_numeric_memory_value_fails_the_budget() {
        let mut lines = complete();
        lines[7] = memory(15_012, json!("<redacted>"));
        assert_eq!(state(&graded(&lines), "budget"), State::Fail);
    }

    #[test]
    fn one_line_says_frame_and_snapshot_are_not_graded() {
        let lines = graded(&complete()).lines();
        let ungraded: Vec<&String> = lines
            .iter()
            .filter(|line| line.contains("frame and snapshot not graded"))
            .collect();
        assert_eq!(ungraded.len(), 1, "lines: {lines:?}");
        assert!(
            !ungraded[0].contains("PASS") && !ungraded[0].contains("FAIL"),
            "the line is no verdict: {}",
            ungraded[0]
        );
        let budget = lines
            .iter()
            .position(|line| line.starts_with("engine-log: budget PASS"))
            .expect("the budget arm's own line");
        assert_eq!(lines[budget + 1], *ungraded[0]);
    }

    fn report_of(states: &[State]) -> Report {
        Report {
            arms: states
                .iter()
                .map(|state| ArmReading::new("family", *state, "constructed"))
                .collect(),
        }
    }

    #[test]
    fn a_fail_outranks_a_cannot_evaluate_and_the_exit_follows_the_verdict() {
        use State::{CannotEvaluate, Fail, Pass};
        for (states, overall, exit, last) in [
            (vec![Pass, Pass], Pass, 0, "engine-log: PASS"),
            (vec![Pass, Fail], Fail, 1, "engine-log: FAIL"),
            (
                vec![Pass, CannotEvaluate],
                CannotEvaluate,
                2,
                "engine-log: cannot-evaluate",
            ),
            (
                vec![CannotEvaluate, Fail, Pass],
                Fail,
                1,
                "engine-log: FAIL",
            ),
        ] {
            let report = report_of(&states);
            assert_eq!(report.overall(), overall, "{states:?}");
            assert_eq!(report.overall().exit_code(), exit, "{states:?}");
            assert_eq!(report.lines().last().map(String::as_str), Some(last));
        }
    }

    #[test]
    fn no_line_holds_a_record_s_text_or_a_path() {
        let mut lines = with_before_end(&[
            panic_record(16_000, "ERROR"),
            stalled(16_001, CANARY),
            memory(16_002, json!(CANARY)),
        ]);
        lines[0] = boot(CANARY);
        lines.push(record(20_001, "INFO", CANARY, json!({"message": CANARY})));
        let report = graded(&lines);
        assert_eq!(report.overall(), State::Fail);
        for line in report.lines() {
            assert!(line.starts_with("engine-log: "), "{line}");
            assert!(
                !line.contains(CANARY) && !line.contains('/') && !line.contains('\\'),
                "a line may hold neither a record's text nor a path: {line}"
            );
        }
    }

    fn evidence(program: &'static str, ticks: [usize; 3], populated: usize) -> SettleEvidence {
        SettleEvidence {
            program,
            ingest_ticks: ticks[0],
            buffer_ticks: ticks[1],
            connection_ticks: ticks[2],
            memory_samples_populated: populated,
        }
    }

    fn gradeable() -> SettleEvidence {
        evidence("console", [2, 2, 2], 1)
    }

    #[test]
    fn a_settle_window_shorter_than_a_tick_interval_and_its_margin_is_refused() {
        assert!(!timeout_supports_verdict(0));
        assert!(!timeout_supports_verdict(15));
        assert!(!timeout_supports_verdict(19));
        assert!(timeout_supports_verdict(20));
        assert!(timeout_supports_verdict(60));
    }

    #[test]
    fn a_live_console_run_with_a_gradeable_log_is_settled() {
        assert_eq!(
            decide_engine_settled(Some(7), Some(true), &gradeable(), false),
            Some("settled")
        );
        assert_eq!(
            decide_engine_settled(
                Some(7),
                Some(true),
                &evidence("console", [9, 3, 2], 4),
                true
            ),
            Some("settled")
        );
    }

    #[test]
    fn a_dead_pid_is_ended_whatever_the_log_holds() {
        assert_eq!(
            decide_engine_settled(Some(7), Some(false), &gradeable(), false),
            Some("ended")
        );
        assert_eq!(
            decide_engine_settled(Some(7), Some(false), &SettleEvidence::default(), true),
            Some("ended")
        );
    }

    #[test]
    fn a_log_whose_boot_record_reads_window_is_wrong_program() {
        assert_eq!(
            decide_engine_settled(
                Some(7),
                Some(true),
                &evidence("window", [2, 2, 2], 1),
                false
            ),
            Some("wrong-program")
        );
    }

    #[test]
    fn a_log_short_of_a_reading_polls_until_the_timeout() {
        for short in [
            evidence("console", [1, 2, 2], 1),
            evidence("console", [2, 1, 2], 1),
            evidence("console", [2, 2, 1], 1),
            evidence("console", [2, 2, 2], 0),
            evidence(PROGRAM_UNKNOWN, [2, 2, 2], 1),
        ] {
            assert_eq!(
                decide_engine_settled(Some(7), Some(true), &short, false),
                None,
                "{short:?}"
            );
            assert_eq!(
                decide_engine_settled(Some(7), Some(true), &short, true),
                Some("not-settled"),
                "{short:?}"
            );
        }
    }

    #[test]
    fn an_absent_or_unprobeable_pid_cannot_be_evaluated() {
        assert_eq!(
            decide_engine_settled(None, None, &gradeable(), false),
            Some("cannot-evaluate")
        );
        assert_eq!(
            decide_engine_settled(Some(7), None, &gradeable(), false),
            Some("cannot-evaluate")
        );
    }

    #[test]
    fn settle_evidence_counts_the_ticks_and_the_populated_memory_samples() {
        let mut lines = complete();
        lines.push(tick(30_010, INGEST_TICK));
        lines.push(memory(30_012, json!("<redacted>")));
        lines.push(tick(30_013, "viz.tick"));
        lines.push("not json".to_owned());
        assert_eq!(settle_evidence(&lines), evidence("console", [3, 2, 2], 1));
        assert_eq!(
            settle_evidence(&[]),
            evidence(PROGRAM_UNKNOWN, [0, 0, 0], 0)
        );
        assert!(settle_evidence(&complete()) == gradeable());
    }

    #[test]
    fn settle_evidence_reads_the_family_and_no_file_beside_it() {
        let dir = family(&complete());
        std::fs::write(dir.path().join("boot.log"), complete().join("\n")).expect("write");
        assert_eq!(
            read_settle_evidence(&dir.path().join("agent-latest.jsonl")),
            gradeable()
        );
        assert_eq!(
            read_settle_evidence(&dir.path().join("absent").join("agent-latest.jsonl")),
            SettleEvidence::default()
        );
    }

    #[test]
    fn the_settle_exit_is_zero_for_settled_alone() {
        assert_eq!(settle_exit("settled"), 0);
        assert_eq!(settle_exit("ended"), 1);
        assert_eq!(settle_exit("wrong-program"), 1);
        assert_eq!(settle_exit("not-settled"), 1);
        assert_eq!(settle_exit("cannot-evaluate"), 2);
    }

    #[test]
    fn the_settle_payload_carries_the_closed_member_set() {
        let payload = settled_payload("settled", Some(7), &evidence("console", [3, 2, 2], 1));
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
                "program",
                "ingest_ticks",
                "buffer_ticks",
                "connection_ticks",
                "memory_samples_populated",
            ])
        );
        assert_eq!(payload["verdict"], "settled");
        assert_eq!(payload["pid"], 7);
        assert_eq!(payload["program"], "console");
        assert_eq!(payload["ingest_ticks"], 3);
        assert_eq!(payload["memory_samples_populated"], 1);
    }

    #[test]
    fn the_console_label_is_the_one_the_settle_read_waits_for() {
        assert_eq!(Program::Console.label(), "console");
        assert_eq!(
            boot_record_program(&serde_json::from_str(&boot("console")).expect("json")),
            Some("console")
        );
    }
}
