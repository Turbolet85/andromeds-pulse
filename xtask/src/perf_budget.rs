//! Perf-budget grader over the app's obs-log family (obs-plan §10 Performance
//! budgets): frame p99 ≤ 33 ms, `metric.buffer.memory_bytes` max ≤ 512 000 000,
//! snapshot p99 ≤ 500 ms.
//!
//! An arm with no record of its target reads NEUTRAL. A caller that EXPECTS
//! samples names that arm as required, and a required arm with nothing to
//! read fails the gate — a log with no samples can never grade PASS.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Result;
use serde_json::Value;

use crate::smoke::read_jsonl_lines;

pub const FRAME_P99_BUDGET_MS: f64 = 33.0;
pub const MEMORY_MAX_BUDGET_BYTES: f64 = 512_000_000.0;
pub const SNAPSHOT_P99_BUDGET_MS: f64 = 500.0;

const LOG_FAMILY_STEM: &str = "agent-latest.jsonl";

/// The webview's per-request WebGPU adapter outcome
/// (`telemetry.frontend.record_webgpu_adapter`).
const ADAPTER_TARGET: &str = "ui.webgpu.adapter";
const ADAPTER_OBTAINED: &str = "obtained";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    Frame,
    Memory,
    Snapshot,
}

impl Arm {
    pub const ALL: [Arm; 3] = [Arm::Frame, Arm::Memory, Arm::Snapshot];

    pub fn name(self) -> &'static str {
        match self {
            Arm::Frame => "frame",
            Arm::Memory => "memory",
            Arm::Snapshot => "snapshot",
        }
    }

    pub fn parse(name: &str) -> Option<Arm> {
        Arm::ALL.into_iter().find(|a| a.name() == name.trim())
    }

    fn target(self) -> &'static str {
        match self {
            Arm::Frame => "metric.webgpu.frame_duration_ms",
            Arm::Memory => "metric.buffer.memory_bytes",
            Arm::Snapshot => "metric.snapshot.token_count_ms",
        }
    }

    fn field(self) -> &'static str {
        match self {
            Arm::Frame | Arm::Snapshot => "duration_ms",
            Arm::Memory => "value",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArmState {
    Pass {
        stat: f64,
        n: usize,
    },
    Fail {
        stat: f64,
        n: usize,
    },
    Neutral {
        reason: String,
    },
    /// A record of the target whose graded field is not a JSON number (a
    /// redacted field reads `"<redacted>"`). The sample exists but cannot be
    /// graded, so the arm fails rather than reading as empty.
    Unreadable {
        unreadable: usize,
        n: usize,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArmResult {
    pub arm: Arm,
    pub state: ArmState,
    /// Memory only: how many of the samples are non-zero.
    pub populated: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
    Neutral,
}

impl Verdict {
    pub fn word(self) -> &'static str {
        match self {
            Verdict::Pass => "PASS",
            Verdict::Fail => "FAIL",
            Verdict::Neutral => "NEUTRAL",
        }
    }
}

/// Nearest-rank p99: the ⌈0.99·n⌉-th smallest sample, as a 0-based index.
pub fn nearest_rank_p99_index(n: usize) -> usize {
    (n * 99).div_ceil(100).max(1) - 1
}

fn samples(lines: &[Value], arm: Arm) -> (Vec<f64>, usize) {
    let mut numbers = Vec::new();
    let mut unreadable = 0;
    for line in lines
        .iter()
        .filter(|l| l.get("target").and_then(Value::as_str) == Some(arm.target()))
    {
        match line
            .get("fields")
            .and_then(|f| f.get(arm.field()))
            .and_then(Value::as_f64)
        {
            Some(v) => numbers.push(v),
            None => unreadable += 1,
        }
    }
    (numbers, unreadable)
}

/// Why a log carries no frame sample, read from the adapter records in the same
/// log: none at all (no webview reached the request), an obtained adapter (frames
/// need only one), or the non-obtained outcomes seen.
pub fn frame_cause(lines: &[Value]) -> String {
    let outcomes: std::collections::BTreeSet<&str> = lines
        .iter()
        .filter(|l| l.get("target").and_then(Value::as_str) == Some(ADAPTER_TARGET))
        .map(|l| {
            l.get("fields")
                .and_then(|f| f.get("outcome"))
                .and_then(Value::as_str)
                .unwrap_or("unreadable")
        })
        .collect();
    if outcomes.is_empty() {
        "no adapter record in this log".to_string()
    } else if outcomes.contains(ADAPTER_OBTAINED) {
        "adapter obtained but no frame recorded".to_string()
    } else {
        let seen: Vec<&str> = outcomes.into_iter().collect();
        format!("no WebGPU adapter ({})", seen.join(", "))
    }
}

fn p99(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    values[nearest_rank_p99_index(values.len())]
}

pub fn grade_arm(lines: &[Value], arm: Arm) -> ArmResult {
    let (values, unreadable) = samples(lines, arm);
    let n = values.len() + unreadable;
    let result = |state, populated| ArmResult {
        arm,
        state,
        populated,
    };
    if unreadable > 0 {
        return result(ArmState::Unreadable { unreadable, n }, None);
    }
    if n == 0 {
        let reason = match arm {
            Arm::Frame => frame_cause(lines),
            Arm::Memory | Arm::Snapshot => format!("no {} record", arm.target()),
        };
        return result(ArmState::Neutral { reason }, None);
    }
    let (stat, budget, populated) = match arm {
        Arm::Frame => (p99(values), FRAME_P99_BUDGET_MS, None),
        Arm::Snapshot => (p99(values), SNAPSHOT_P99_BUDGET_MS, None),
        Arm::Memory => {
            // A zero is the placeholder written before the first retention
            // sweep sets the gauge, so it says nothing about the budget.
            let populated = values.iter().filter(|v| **v != 0.0).count();
            if populated == 0 {
                return result(
                    ArmState::Neutral {
                        reason: format!("populated 0 of {n}"),
                    },
                    Some(0),
                );
            }
            let max = values.iter().copied().fold(f64::MIN, f64::max);
            (max, MEMORY_MAX_BUDGET_BYTES, Some(populated))
        }
    };
    let state = if stat <= budget {
        ArmState::Pass { stat, n }
    } else {
        ArmState::Fail { stat, n }
    };
    result(state, populated)
}

pub fn grade(lines: &[Value]) -> Vec<ArmResult> {
    Arm::ALL.into_iter().map(|a| grade_arm(lines, a)).collect()
}

pub fn evaluate(results: &[ArmResult], required: &[Arm]) -> Verdict {
    let failed = results.iter().any(|r| match r.state {
        ArmState::Fail { .. } | ArmState::Unreadable { .. } => true,
        ArmState::Neutral { .. } => required.contains(&r.arm),
        ArmState::Pass { .. } => false,
    });
    if failed {
        Verdict::Fail
    } else if results
        .iter()
        .any(|r| matches!(r.state, ArmState::Pass { .. }))
    {
        Verdict::Pass
    } else {
        Verdict::Neutral
    }
}

fn format_stat(arm: Arm, stat: f64) -> String {
    match arm {
        Arm::Memory => format!("{stat:.0} B"),
        Arm::Frame | Arm::Snapshot => format!("{stat:.1} ms"),
    }
}

fn format_budget(arm: Arm) -> String {
    match arm {
        Arm::Frame => format!("{FRAME_P99_BUDGET_MS:.0} ms"),
        Arm::Memory => format!("{MEMORY_MAX_BUDGET_BYTES:.0} B"),
        Arm::Snapshot => format!("{SNAPSHOT_P99_BUDGET_MS:.0} ms"),
    }
}

/// One line per arm, e.g. `perf-budget: frame p99 4.1 ms <= 33 ms (n=496) PASS`.
pub fn arm_lines(results: &[ArmResult], required: &[Arm]) -> Vec<String> {
    results
        .iter()
        .map(|r| {
            let name = r.arm.name();
            let stat_name = if r.arm == Arm::Memory { "max" } else { "p99" };
            let counts = |n: usize| match r.populated {
                Some(p) => format!("n={n}, populated {p}"),
                None => format!("n={n}"),
            };
            match &r.state {
                ArmState::Pass { stat, n } => format!(
                    "perf-budget: {name} {stat_name} {} <= {} ({}) PASS",
                    format_stat(r.arm, *stat),
                    format_budget(r.arm),
                    counts(*n)
                ),
                ArmState::Fail { stat, n } => format!(
                    "perf-budget: {name} {stat_name} {} > {} ({}) FAIL",
                    format_stat(r.arm, *stat),
                    format_budget(r.arm),
                    counts(*n)
                ),
                ArmState::Unreadable { unreadable, n } => format!(
                    "perf-budget: {name} UNREADABLE — {unreadable} of {n} record(s) carry a \
                     non-numeric {} FAIL",
                    r.arm.field()
                ),
                ArmState::Neutral { reason } if required.contains(&r.arm) => {
                    format!("perf-budget: {name} NEUTRAL — {reason} (required) FAIL")
                }
                // The frame arm is graded by `perf:frame-sample` on a GPU host; where
                // it reads empty it says so by name with its cause, never as a pass.
                ArmState::Neutral { reason } if r.arm == Arm::Frame => {
                    format!("perf-budget: frame: cannot-evaluate: 0 samples, {reason}")
                }
                ArmState::Neutral { reason } => format!("perf-budget: {name} NEUTRAL — {reason}"),
            }
        })
        .collect()
}

pub fn parse_lines(raw: &[String]) -> Vec<Value> {
    raw.iter()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .collect()
}

/// Every member of the `agent-latest.jsonl*` family under `logs_dir`.
pub fn family_members(logs_dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(logs_dir) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
        .filter(|e| e.file_name().to_string_lossy().starts_with(LOG_FAMILY_STEM))
        .map(|e| e.path())
        .collect();
    files.sort();
    files
}

/// The whole family, concatenated in member order.
pub fn read_family(logs_dir: &Path) -> Result<Vec<Value>> {
    let raw = read_jsonl_lines(&logs_dir.join(LOG_FAMILY_STEM))?;
    Ok(parse_lines(&raw))
}

pub fn parse_required(names: &[String]) -> Result<Vec<Arm>, String> {
    names
        .iter()
        .filter(|n| !n.trim().is_empty())
        .map(|n| {
            Arm::parse(n)
                .ok_or_else(|| format!("unknown arm {n:?} (known: frame, memory, snapshot)"))
        })
        .collect()
}

/// `cargo xtask perf:budget --data-dir <DIR> --require <arm,arm>`.
/// Exit 0 PASS · 1 FAIL · 2 cannot-evaluate (an unknown arm, no log family,
/// or nothing to grade with nothing required).
pub fn run_perf_budget(data_dir: &Path, require: &[String]) -> Result<ExitCode> {
    let required = match parse_required(require) {
        Ok(r) => r,
        Err(why) => {
            println!("perf-budget: cannot evaluate — {why}");
            return Ok(ExitCode::from(2));
        }
    };
    let logs_dir = data_dir.join("logs");
    let members = family_members(&logs_dir);
    if members.is_empty() {
        println!(
            "perf-budget: cannot evaluate — no {LOG_FAMILY_STEM}* family under {}",
            logs_dir.display()
        );
        return Ok(ExitCode::from(2));
    }
    let lines = read_family(&logs_dir)?;
    println!(
        "perf-budget: graded {} record(s) across {} file(s)",
        lines.len(),
        members.len()
    );
    let results = grade(&lines);
    for line in arm_lines(&results, &required) {
        println!("{line}");
    }
    let verdict = evaluate(&results, &required);
    println!("perf-budget: {}", verdict.word());
    Ok(match verdict {
        Verdict::Pass => ExitCode::SUCCESS,
        Verdict::Fail => ExitCode::FAILURE,
        Verdict::Neutral => ExitCode::from(2),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn record(target: &str, field: &str, value: Value) -> Value {
        json!({"timestamp": "2026-09-30T12:00:00.000000Z", "level": "INFO",
               "target": target, "fields": {field: value}})
    }

    fn frames(values: &[f64]) -> Vec<Value> {
        values
            .iter()
            .map(|v| record("metric.webgpu.frame_duration_ms", "duration_ms", json!(v)))
            .collect()
    }

    fn memory(values: &[f64]) -> Vec<Value> {
        values
            .iter()
            .map(|v| record("metric.buffer.memory_bytes", "value", json!(v)))
            .collect()
    }

    fn snapshots(values: &[f64]) -> Vec<Value> {
        values
            .iter()
            .map(|v| record("metric.snapshot.token_count_ms", "duration_ms", json!(v)))
            .collect()
    }

    fn noise() -> Vec<Value> {
        vec![record("app.boot.ready", "message", json!("seed"))]
    }

    #[test]
    fn frame_in_budget_passes() {
        let r = grade_arm(&frames(&[1.0, 4.1, 2.0]), Arm::Frame);
        assert_eq!(r.state, ArmState::Pass { stat: 4.1, n: 3 });
    }

    #[test]
    fn frame_over_budget_fails() {
        let mut v = vec![5.0; 99];
        v.push(40.0);
        v.push(41.0);
        let r = grade_arm(&frames(&v), Arm::Frame);
        assert_eq!(r.state, ArmState::Fail { stat: 40.0, n: 101 });
    }

    #[test]
    fn frame_absent_is_neutral() {
        let r = grade_arm(&noise(), Arm::Frame);
        assert!(matches!(r.state, ArmState::Neutral { .. }), "{:?}", r.state);
        let line = &arm_lines(std::slice::from_ref(&r), &[])[0];
        assert_eq!(
            line,
            "perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log"
        );
        assert!(!line.contains("PASS"), "{line}");
    }

    fn adapter(outcome: &str) -> Value {
        record("ui.webgpu.adapter", "outcome", json!(outcome))
    }

    fn frame_line(lines: &[Value], required: &[Arm]) -> String {
        let r = grade_arm(lines, Arm::Frame);
        arm_lines(std::slice::from_ref(&r), required)[0].clone()
    }

    #[test]
    fn frame_cause_no_adapter_record_names_that_cause() {
        assert_eq!(frame_cause(&noise()), "no adapter record in this log");
    }

    #[test]
    fn frame_cause_obtained_adapter_without_frames() {
        let mut lines = noise();
        lines.push(adapter("obtained"));
        assert_eq!(
            frame_line(&lines, &[]),
            "perf-budget: frame: cannot-evaluate: 0 samples, adapter obtained but no frame recorded"
        );
    }

    #[test]
    fn frame_cause_names_the_distinct_non_obtained_outcomes_sorted() {
        let lines = vec![
            adapter("no_navigator_gpu"),
            adapter("adapter_null"),
            adapter("no_navigator_gpu"),
        ];
        assert_eq!(
            frame_line(&lines, &[]),
            "perf-budget: frame: cannot-evaluate: 0 samples, \
             no WebGPU adapter (adapter_null, no_navigator_gpu)"
        );
    }

    #[test]
    fn frame_cause_mixed_log_reads_obtained() {
        let lines = vec![adapter("device_request_failed"), adapter("obtained")];
        assert_eq!(
            frame_cause(&lines),
            "adapter obtained but no frame recorded"
        );
    }

    #[test]
    fn frame_cause_required_empty_frame_arm_still_fails() {
        let lines = vec![adapter("adapter_request_rejected")];
        let results = grade(&lines);
        assert_eq!(evaluate(&results, &[Arm::Frame]), Verdict::Fail);
        assert_eq!(
            frame_line(&lines, &[Arm::Frame]),
            "perf-budget: frame NEUTRAL — no WebGPU adapter (adapter_request_rejected) \
             (required) FAIL"
        );
    }

    #[test]
    fn frame_cause_is_not_consulted_when_frames_exist() {
        let mut lines = frames(&[2.0]);
        lines.push(adapter("no_navigator_gpu"));
        assert_eq!(
            grade_arm(&lines, Arm::Frame).state,
            ArmState::Pass { stat: 2.0, n: 1 }
        );
    }

    #[test]
    fn memory_in_budget_passes() {
        let r = grade_arm(&memory(&[256_000.0, 512_000_000.0]), Arm::Memory);
        assert_eq!(
            r.state,
            ArmState::Pass {
                stat: 512_000_000.0,
                n: 2
            }
        );
        assert_eq!(r.populated, Some(2));
    }

    #[test]
    fn memory_over_budget_fails() {
        let r = grade_arm(&memory(&[1_000.0, 512_000_001.0]), Arm::Memory);
        assert_eq!(
            r.state,
            ArmState::Fail {
                stat: 512_000_001.0,
                n: 2
            }
        );
    }

    #[test]
    fn memory_absent_is_neutral() {
        let r = grade_arm(&noise(), Arm::Memory);
        assert!(matches!(r.state, ArmState::Neutral { .. }), "{:?}", r.state);
    }

    #[test]
    fn memory_zeros_are_not_informative() {
        let r = grade_arm(&memory(&[0.0, 0.0, 0.0]), Arm::Memory);
        assert_eq!(
            r.state,
            ArmState::Neutral {
                reason: "populated 0 of 3".to_string()
            }
        );
        assert_eq!(evaluate(&[r], &[Arm::Memory]), Verdict::Fail);
    }

    #[test]
    fn snapshot_in_budget_passes() {
        let r = grade_arm(&snapshots(&[12.0, 480.0]), Arm::Snapshot);
        assert_eq!(r.state, ArmState::Pass { stat: 480.0, n: 2 });
    }

    #[test]
    fn snapshot_over_budget_fails() {
        let r = grade_arm(&snapshots(&[12.0, 501.0]), Arm::Snapshot);
        assert_eq!(r.state, ArmState::Fail { stat: 501.0, n: 2 });
    }

    #[test]
    fn snapshot_absent_is_neutral() {
        let r = grade_arm(&noise(), Arm::Snapshot);
        assert!(matches!(r.state, ArmState::Neutral { .. }), "{:?}", r.state);
    }

    #[test]
    fn unreadable_field_fails_even_when_not_required() {
        let mut lines = snapshots(&[10.0]);
        lines.push(record(
            "metric.snapshot.token_count_ms",
            "duration_ms",
            json!("<redacted>"),
        ));
        let results = grade(&lines);
        let snapshot = &results[2];
        assert_eq!(
            snapshot.state,
            ArmState::Unreadable {
                unreadable: 1,
                n: 2
            }
        );
        assert_eq!(evaluate(&results, &[]), Verdict::Fail);
    }

    #[test]
    fn required_but_empty_fails() {
        let results = grade(&memory(&[4_096.0]));
        assert_eq!(evaluate(&results, &[]), Verdict::Pass);
        assert_eq!(
            evaluate(&results, &[Arm::Memory, Arm::Snapshot]),
            Verdict::Fail
        );
        let lines = arm_lines(&results, &[Arm::Memory, Arm::Snapshot]);
        assert!(lines[2].ends_with("(required) FAIL"), "{}", lines[2]);
    }

    #[test]
    fn all_empty_with_nothing_required_is_neutral_not_pass() {
        let results = grade(&noise());
        assert_eq!(evaluate(&results, &[]), Verdict::Neutral);
        assert_eq!(evaluate(&grade(&[]), &[]), Verdict::Neutral);
    }

    #[test]
    fn nearest_rank_p99_index_boundaries() {
        assert_eq!(nearest_rank_p99_index(1), 0);
        assert_eq!(nearest_rank_p99_index(99), 98);
        assert_eq!(nearest_rank_p99_index(100), 98);
        assert_eq!(nearest_rank_p99_index(101), 99);
    }

    #[test]
    fn family_read_concatenates_two_date_suffixed_members() {
        let dir = tempfile::tempdir().expect("tmp");
        let line = |v: &Value| format!("{v}\n");
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-09-29"),
            line(&frames(&[3.0])[0]),
        )
        .expect("write day 1");
        std::fs::write(
            dir.path().join("agent-latest.jsonl.2026-09-30"),
            line(&memory(&[8_192.0])[0]),
        )
        .expect("write day 2");
        let results = grade(&read_family(dir.path()).expect("family reads"));
        assert!(matches!(results[0].state, ArmState::Pass { n: 1, .. }));
        assert!(matches!(results[1].state, ArmState::Pass { n: 1, .. }));
        assert_eq!(family_members(dir.path()).len(), 2);
    }
}
