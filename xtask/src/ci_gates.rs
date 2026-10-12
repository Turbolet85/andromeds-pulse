//! The `ci-gates` verdict over the app's obs-log family (obs-plan 10, CI
//! gates): the family holds at least one record (zero-spans) and no
//! `app.panic.fatal` record at ERROR (zero-panic).
//!
//! A log dir with no family member is an exit of its own, never a pass: a
//! verb that read no log has no verdict to print. The verb prints closed
//! labels, counts, a member's file name and a line number, never a record's
//! text and never a path.

use std::path::Path;
use std::process::ExitCode;

use serde_json::Value;

use crate::perf_budget::family_members;

const LOG_FAMILY: &str = "agent-latest.jsonl*";
const PANIC_TARGET: &str = "app.panic.fatal";

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The log dir is absent or holds no family member.
    CannotEvaluate,
    /// Members are present and hold no record.
    NoRecords,
    /// The first panic record at ERROR, by its member's file name and line.
    Panic {
        records: usize,
        files: usize,
        member: String,
        line: usize,
    },
    Pass {
        records: usize,
        files: usize,
    },
}

impl Verdict {
    pub fn exit_code(&self) -> u8 {
        match self {
            Verdict::Pass { .. } => 0,
            Verdict::NoRecords | Verdict::Panic { .. } => 1,
            Verdict::CannotEvaluate => 2,
        }
    }

    /// The lines the verb prints, in order.
    pub fn lines(&self) -> Vec<String> {
        let counted = |records: &usize, files: &usize| {
            format!("ci-gates: zero-spans PASS ({records} log records across {files} file(s))")
        };
        match self {
            Verdict::CannotEvaluate => vec![format!(
                "::error::ci-gates: cannot-evaluate (no {LOG_FAMILY} file in the log dir)"
            )],
            Verdict::NoRecords => vec![
                "::error::ci-gates: zero-spans FAIL (log files present but contain no events)"
                    .to_owned(),
            ],
            Verdict::Panic {
                records,
                files,
                member,
                line,
            } => vec![
                counted(records, files),
                format!("::error::ci-gates: zero-panic FAIL: {PANIC_TARGET} at {member}:{line}"),
            ],
            Verdict::Pass { records, files } => vec![
                counted(records, files),
                "ci-gates: zero-panic PASS".to_owned(),
            ],
        }
    }
}

fn file_name(member: &Path) -> String {
    member
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn is_panic_at_error(raw: &str) -> bool {
    let Ok(record) = serde_json::from_str::<Value>(raw) else {
        return false;
    };
    let text = |key: &str| record.get(key).and_then(Value::as_str);
    text("target") == Some(PANIC_TARGET) && text("level") == Some("ERROR")
}

/// Reads every member of the family under `log_dir`. A non-empty line is a
/// record whether or not it parses.
pub fn evaluate(log_dir: &Path) -> Verdict {
    let members = family_members(log_dir);
    if members.is_empty() {
        return Verdict::CannotEvaluate;
    }
    let mut records = 0usize;
    let mut panic: Option<(String, usize)> = None;
    for member in &members {
        let Ok(content) = std::fs::read_to_string(member) else {
            eprintln!("ci-gates: skipping {} (unreadable)", file_name(member));
            continue;
        };
        for (idx, raw) in content.lines().enumerate() {
            if raw.trim().is_empty() {
                continue;
            }
            records += 1;
            if panic.is_none() && is_panic_at_error(raw) {
                panic = Some((file_name(member), idx + 1));
            }
        }
    }
    let files = members.len();
    match panic {
        _ if records == 0 => Verdict::NoRecords,
        Some((member, line)) => Verdict::Panic {
            records,
            files,
            member,
            line,
        },
        None => Verdict::Pass { records, files },
    }
}

/// `cargo xtask ci-gates`. Exit 0 PASS, 1 FAIL, 2 cannot-evaluate.
pub fn run(log_dir: &Path) -> ExitCode {
    let verdict = evaluate(log_dir);
    for line in verdict.lines() {
        if line.starts_with("::error::") {
            eprintln!("{line}");
        } else {
            println!("{line}");
        }
    }
    ExitCode::from(verdict.exit_code())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEMBER: &str = "agent-latest.jsonl.2026-10-10";
    const BOOT_RECORD: &str = r#"{"timestamp":"2026-10-10T00:00:00.000000Z","level":"INFO","target":"app.boot.ready","fields":{"message":"ready"}}"#;
    const PANIC_TEXT: &str = "canary-panic-text";

    fn panic_record(level: &str) -> String {
        format!(
            r#"{{"timestamp":"2026-10-10T00:00:01.000000Z","level":"{level}","target":"app.panic.fatal","fields":{{"message":"{PANIC_TEXT}","location":"src/main.rs:1"}}}}"#
        )
    }

    fn log_dir_holding(content: &str) -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tmp");
        std::fs::write(dir.path().join(MEMBER), content).expect("write the member");
        dir
    }

    fn assert_cannot_evaluate(verdict: &Verdict) {
        assert_eq!(*verdict, Verdict::CannotEvaluate);
        assert_eq!(verdict.exit_code(), 2);
        let lines = verdict.lines();
        assert_eq!(lines.len(), 1, "lines: {lines:?}");
        assert!(lines[0].contains("cannot-evaluate"), "lines: {lines:?}");
        assert!(
            !lines[0].contains("NEUTRAL") && !lines[0].contains("PASS"),
            "lines: {lines:?}"
        );
    }

    #[test]
    fn an_absent_log_dir_cannot_be_evaluated() {
        let dir = tempfile::TempDir::new().expect("tmp");
        assert_cannot_evaluate(&evaluate(&dir.path().join("absent").join("logs")));
    }

    #[test]
    fn a_log_dir_with_no_family_member_cannot_be_evaluated() {
        let dir = tempfile::TempDir::new().expect("tmp");
        std::fs::write(dir.path().join("boot.log"), "not a member\n").expect("write a bystander");
        assert_cannot_evaluate(&evaluate(dir.path()));
    }

    #[test]
    fn a_member_with_no_record_fails() {
        let dir = log_dir_holding("");
        let verdict = evaluate(dir.path());
        assert_eq!(verdict, Verdict::NoRecords);
        assert_eq!(verdict.exit_code(), 1);
    }

    #[test]
    fn one_record_passes_with_the_two_reading_lines_alone() {
        let dir = log_dir_holding(&format!("{BOOT_RECORD}\n"));
        let verdict = evaluate(dir.path());
        assert_eq!(verdict.exit_code(), 0);
        assert_eq!(
            verdict.lines(),
            [
                "ci-gates: zero-spans PASS (1 log records across 1 file(s))",
                "ci-gates: zero-panic PASS",
            ]
        );
    }

    #[test]
    fn a_panic_record_at_error_fails_by_file_name_and_line_alone() {
        let dir = log_dir_holding(&format!("{BOOT_RECORD}\n{}\n", panic_record("ERROR")));
        let verdict = evaluate(dir.path());
        assert_eq!(verdict.exit_code(), 1);
        let lines = verdict.lines();
        assert!(
            lines.contains(&format!(
                "::error::ci-gates: zero-panic FAIL: app.panic.fatal at {MEMBER}:2"
            )),
            "lines: {lines:?}"
        );
        for line in &lines {
            assert!(
                !line.contains(PANIC_TEXT) && !line.contains('/') && !line.contains('\\'),
                "a line may hold neither the record's text nor a path: {line}"
            );
        }
    }

    #[test]
    fn a_panic_target_record_below_error_passes() {
        let dir = log_dir_holding(&format!("{BOOT_RECORD}\n{}\n", panic_record("WARN")));
        let verdict = evaluate(dir.path());
        assert_eq!(
            verdict,
            Verdict::Pass {
                records: 2,
                files: 1
            }
        );
        assert_eq!(verdict.exit_code(), 0);
    }
}
