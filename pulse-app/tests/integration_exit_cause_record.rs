//! Process-end witness for the `app.exit` record.
//!
//! "The record survives the process ending" is a cross-process claim, so each
//! arm crosses a real process boundary: the parent re-executes this test
//! binary with `--exact <fn>` and an arm name in the environment, the child
//! calls `observability::init` on its own TempDir data dir and ends its
//! process through ONE class, and the parent reads the child's
//! `agent-latest.jsonl*` family only after the child has ended. Every child
//! also asserts it ran (its `app.boot.tracing.init` record is required), so a
//! child that returned early cannot pass.

use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

use pulse_app::observability;
use serde_json::Value;

const ARM_ENV: &str = "PULSE_EXIT_CAUSE_ARM";
const DIR_ENV: &str = "PULSE_EXIT_CAUSE_DIR";
const CANARY_ENV: &str = "PULSE_EXIT_CAUSE_CANARY_PASSPHRASE";
const CANARY: &str = "exit-cause-canary-5d1f0b";

struct Ended {
    status: ExitStatus,
    lines: Vec<Value>,
    dir: tempfile::TempDir,
}

impl Ended {
    fn exit_records(&self) -> Vec<&Value> {
        self.lines
            .iter()
            .filter(|v| v.get("target").and_then(Value::as_str) == Some("app.exit"))
            .collect()
    }

    fn the_exit_record(&self) -> &Value {
        let records = self.exit_records();
        assert_eq!(
            records.len(),
            1,
            "exactly one app.exit record per process end, got {records:?}",
        );
        records[0]
    }
}

// Child side: returns only when this process is not the named arm's child.
fn child_arm(arm: &str) -> Option<std::path::PathBuf> {
    if std::env::var(ARM_ENV).ok().as_deref() != Some(arm) {
        return None;
    }
    std::env::var_os(DIR_ENV).map(std::path::PathBuf::from)
}

fn init_child(dir: &Path) {
    observability::hold_log_guard(observability::init(dir));
}

// Parent side. Returns None (skip-clean) when the test binary cannot be
// re-executed on this host.
fn run_child(test: &str, arm: &str) -> Option<Ended> {
    let Ok(exe) = std::env::current_exe() else {
        eprintln!("skip: current_exe unavailable, cannot re-exec {test}");
        return None;
    };
    let dir = tempfile::tempdir().expect("tempdir");
    let status = Command::new(exe)
        .args(["--exact", test, "--nocapture"])
        .env(ARM_ENV, arm)
        .env(DIR_ENV, dir.path())
        .env(CANARY_ENV, CANARY)
        .env_remove("RUST_LOG")
        .env_remove("ANDROMEDA_PULSE_LOG_LEVEL")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("spawn child");
    let lines = family_lines(dir.path());
    let ended = Ended { status, lines, dir };
    assert_child_ran(&ended);
    assert_no_path_and_no_canary(&ended);
    Some(ended)
}

fn family_lines(dir: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir.join("logs")) else {
        return out;
    };
    for entry in entries.flatten() {
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with("agent-latest.jsonl")
        {
            continue;
        }
        let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            out.push(serde_json::from_str(line).expect("every sink line is one JSON object"));
        }
    }
    out
}

fn assert_child_ran(ended: &Ended) {
    let init = ended
        .lines
        .iter()
        .filter(|v| v.get("target").and_then(Value::as_str) == Some("app.boot.tracing.init"))
        .count();
    assert_eq!(init, 1, "the child must have run observability::init");
}

fn strings_in(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) => out.push(s.clone()),
        Value::Array(items) => items.iter().for_each(|v| strings_in(v, out)),
        Value::Object(map) => map.values().for_each(|v| strings_in(v, out)),
        _ => {}
    }
}

fn assert_no_path_and_no_canary(ended: &Ended) {
    let full = ended.dir.path().to_string_lossy().to_string();
    let forward = full.replace('\\', "/");
    let mut strings = Vec::new();
    ended.lines.iter().for_each(|v| strings_in(v, &mut strings));
    for s in &strings {
        assert!(!s.contains(CANARY), "the planted canary reached the log");
        assert!(
            !s.contains(&full) && !s.contains(&forward),
            "a full data-dir path reached the log",
        );
    }
    let record = ended.exit_records();
    for r in record {
        let fields = r["fields"].as_object().expect("fields object");
        for key in ["exit_class", "exit_code", "exit_code_known", "signal"] {
            assert!(fields.contains_key(key), "app.exit carries `{key}`");
            assert_ne!(fields[key], Value::from("<redacted>"), "`{key}` rendered");
        }
    }
}

fn assert_fields(record: &Value, level: &str, class: &str, code: i64, known: bool, signal: &str) {
    assert_eq!(record["level"], level, "level of {record}");
    let f = &record["fields"];
    assert_eq!(f["exit_class"], class, "exit_class of {record}");
    assert_eq!(f["exit_code"], code, "exit_code of {record}");
    assert_eq!(f["exit_code_known"], known, "exit_code_known of {record}");
    assert_eq!(f["signal"], signal, "signal of {record}");
}

// (a) the event-loop tail, non-zero and zero codes.
#[test]
fn exit_cause_event_loop_nonzero_code_is_carried_at_error() {
    const ARM: &str = "event_loop_3";
    if let Some(dir) = child_arm(ARM) {
        init_child(&dir);
        observability::exit_after_event_loop(3);
    }
    let Some(ended) = run_child(
        "exit_cause_event_loop_nonzero_code_is_carried_at_error",
        ARM,
    ) else {
        return;
    };
    assert_eq!(ended.status.code(), Some(3), "the exit code is unchanged");
    assert_fields(
        ended.the_exit_record(),
        "ERROR",
        "event_loop",
        3,
        true,
        "none",
    );
}

#[test]
fn exit_cause_event_loop_zero_code_is_recorded_at_info() {
    const ARM: &str = "event_loop_0";
    if let Some(dir) = child_arm(ARM) {
        init_child(&dir);
        observability::exit_after_event_loop(0);
    }
    let Some(ended) = run_child("exit_cause_event_loop_zero_code_is_recorded_at_info", ARM) else {
        return;
    };
    assert_eq!(ended.status.code(), Some(0));
    assert_fields(
        ended.the_exit_record(),
        "INFO",
        "event_loop",
        0,
        true,
        "none",
    );
}

// (b) a Rust `process::exit` with no emitter (what tao does). Unix only: on
// Windows it is `ExitProcess`, which runs no `atexit` handler and stops every
// other thread first - unloggable by construction, measured (handler ran:
// false) in the chunk's RED probe.
#[cfg(unix)]
#[test]
fn exit_cause_process_exit_is_recorded_outside_event_loop() {
    const ARM: &str = "process_exit_4";
    if let Some(dir) = child_arm(ARM) {
        init_child(&dir);
        observability::install_exit_hook();
        std::process::exit(4);
    }
    let Some(ended) = run_child(
        "exit_cause_process_exit_is_recorded_outside_event_loop",
        ARM,
    ) else {
        return;
    };
    assert_eq!(ended.status.code(), Some(4));
    assert_fields(
        ended.the_exit_record(),
        "ERROR",
        "outside_event_loop",
        0,
        false,
        "none",
    );
}

// (c) a native C `exit(1)` - the class a GTK/GDK/WebKitGTK exit falls into.
#[test]
fn exit_cause_native_exit_is_recorded_outside_event_loop() {
    const ARM: &str = "native_exit_1";
    if let Some(dir) = child_arm(ARM) {
        init_child(&dir);
        observability::install_exit_hook();
        // SAFETY: ends the process through the C runtime's exit, as a native
        // library would.
        unsafe { libc::exit(1) }
    }
    let Some(ended) = run_child("exit_cause_native_exit_is_recorded_outside_event_loop", ARM)
    else {
        return;
    };
    assert_eq!(ended.status.code(), Some(1));
    assert_fields(
        ended.the_exit_record(),
        "ERROR",
        "outside_event_loop",
        0,
        false,
        "none",
    );
}

// (d) SIGTERM: recorded, and the process still ends BY the signal.
#[cfg(unix)]
#[test]
fn exit_cause_sigterm_is_recorded_and_still_ends_by_signal() {
    use std::os::unix::process::ExitStatusExt;
    const ARM: &str = "sigterm";
    if let Some(dir) = child_arm(ARM) {
        init_child(&dir);
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("runtime");
        let _enter = runtime.enter();
        observability::install_signal_listener();
        // SAFETY: raises a valid signal on this process.
        unsafe {
            libc::raise(libc::SIGTERM);
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            std::thread::park_timeout(std::time::Duration::from_millis(100));
        }
        std::process::exit(99);
    }
    let Some(ended) = run_child(
        "exit_cause_sigterm_is_recorded_and_still_ends_by_signal",
        ARM,
    ) else {
        return;
    };
    assert_eq!(
        ended.status.signal(),
        Some(15),
        "ended by SIGTERM, not an exit code"
    );
    assert_fields(
        ended.the_exit_record(),
        "WARN",
        "signal",
        0,
        false,
        "sigterm",
    );
}

// (e2) the same once-guard on every OS: an end already recorded, then a C
// `exit` that runs the at-exit hook. On Unix this IS the Step 4 tail
// (`process::exit` is `libc::exit` there); on Windows the tail ends in
// `ExitProcess`, which never reaches the hook, so (e) alone cannot fail here.
#[test]
fn exit_cause_at_exit_hook_does_not_duplicate_a_record_before_a_native_exit() {
    const ARM: &str = "recorded_then_native_exit";
    if let Some(dir) = child_arm(ARM) {
        init_child(&dir);
        observability::install_exit_hook();
        let first = observability::record_exit(
            observability::ExitClass::EventLoop,
            Some(3),
            observability::ExitSignal::None,
        );
        // The first flush closes the sink, so a second record could never
        // reach the file; the once-flag itself is read from the return value
        // and reported through the exit code (9 = a second emitter was let
        // through).
        let second = observability::record_exit(
            observability::ExitClass::OutsideEventLoop,
            None,
            observability::ExitSignal::None,
        );
        let code = if first && !second { 3 } else { 9 };
        // SAFETY: ends the process through the C runtime's exit, which runs
        // the at-exit hook on every OS.
        unsafe { libc::exit(code) }
    }
    let Some(ended) = run_child(
        "exit_cause_at_exit_hook_does_not_duplicate_a_record_before_a_native_exit",
        ARM,
    ) else {
        return;
    };
    assert_eq!(ended.status.code(), Some(3));
    assert_fields(
        ended.the_exit_record(),
        "ERROR",
        "event_loop",
        3,
        true,
        "none",
    );
}

// (e) the at-exit hook never duplicates an already-recorded end.
#[test]
fn exit_cause_at_exit_hook_does_not_duplicate_the_event_loop_record() {
    const ARM: &str = "event_loop_with_hook";
    if let Some(dir) = child_arm(ARM) {
        init_child(&dir);
        observability::install_exit_hook();
        observability::exit_after_event_loop(3);
    }
    let Some(ended) = run_child(
        "exit_cause_at_exit_hook_does_not_duplicate_the_event_loop_record",
        ARM,
    ) else {
        return;
    };
    assert_eq!(ended.status.code(), Some(3));
    assert_fields(
        ended.the_exit_record(),
        "ERROR",
        "event_loop",
        3,
        true,
        "none",
    );
}
