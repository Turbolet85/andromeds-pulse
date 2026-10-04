//! The Linux launch render posture (`pulse_app::render_posture`).
//!
//! The decision is pinned per arm with injected inputs; this test process
//! never mutates its own environment. The real `set_var` path runs only in
//! re-executed children: the parent re-runs this binary with `--exact <fn>`,
//! controls the child's environment, and reads what the child wrote to a
//! sink file. Each child writes a child-ran marker first, so a child that
//! returned early cannot pass.

use std::path::Path;
use std::process::{Command, Stdio};

use pulse_app::render_posture::{
    LEVER_ENV, LEVER_VALUE, RenderPosture, apply_linux_default, decide,
};

const GUARD_ENV: &str = "PULSE_RENDER_POSTURE_CHILD";
const SINK_ENV: &str = "PULSE_RENDER_POSTURE_SINK";
const RAN: &str = "child-ran";
const ABSENT: &str = "<absent>";

#[test]
fn linux_without_a_preset_applies_the_default() {
    assert_eq!(decide(true, false), RenderPosture::Applied);
}

#[test]
fn linux_with_a_preset_honours_it() {
    assert_eq!(decide(true, true), RenderPosture::PresetHonoured);
}

#[test]
fn other_platforms_are_not_applicable_whatever_the_preset() {
    assert_eq!(decide(false, false), RenderPosture::NotApplicable);
    assert_eq!(decide(false, true), RenderPosture::NotApplicable);
}

#[test]
fn labels_are_the_closed_set() {
    assert_eq!(RenderPosture::Applied.label(), "applied");
    assert_eq!(RenderPosture::PresetHonoured.label(), "preset_honoured");
    assert_eq!(RenderPosture::NotApplicable.label(), "not_applicable");
}

#[test]
fn lever_is_nvidia_explicit_sync_off() {
    assert_eq!(LEVER_ENV, "__NV_DISABLE_EXPLICIT_SYNC");
    assert_eq!(LEVER_VALUE, "1");
}

// Child side: a no-op unless the parent named this arm.
fn child_writes_posture(arm: &str) {
    if std::env::var(GUARD_ENV).ok().as_deref() != Some(arm) {
        return;
    }
    let sink = std::env::var_os(SINK_ENV).expect("the parent sets the sink path");
    let posture = apply_linux_default();
    let value = std::env::var_os(LEVER_ENV)
        .map(|v| v.to_string_lossy().into_owned())
        .unwrap_or_else(|| ABSENT.to_string());
    std::fs::write(sink, format!("{RAN}\n{}\n{value}\n", posture.label())).expect("write the sink");
}

#[test]
fn child_arm_lever_absent() {
    child_writes_posture("absent");
}

#[test]
fn child_arm_lever_preset_zero() {
    child_writes_posture("preset_zero");
}

/// Re-executes `test` as arm `arm`; `preset` is the lever's launch value
/// (`None` removes it). Returns `(label, post-call value)`.
fn run_child(test: &str, arm: &str, preset: Option<&str>) -> (String, String) {
    let exe = std::env::current_exe().expect("current_exe");
    let dir = tempfile::tempdir().expect("tempdir");
    let sink = dir.path().join("sink.txt");
    let mut cmd = Command::new(exe);
    cmd.args(["--exact", test, "--nocapture"])
        .env(GUARD_ENV, arm)
        .env(SINK_ENV, &sink)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    match preset {
        Some(v) => cmd.env(LEVER_ENV, v),
        None => cmd.env_remove(LEVER_ENV),
    };
    let status = cmd.status().expect("spawn child");
    assert!(status.success(), "child {test} failed: {status}");
    read_sink(&sink)
}

fn read_sink(sink: &Path) -> (String, String) {
    let text = std::fs::read_to_string(sink).expect("the child wrote its sink");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3, "sink holds marker, label, value: {lines:?}");
    assert_eq!(lines[0], RAN, "the child must have run");
    (lines[1].to_string(), lines[2].to_string())
}

#[test]
fn absent_lever_is_set_to_one_in_a_fresh_process() {
    let (label, value) = run_child("child_arm_lever_absent", "absent", None);
    if cfg!(target_os = "linux") {
        assert_eq!(label, "applied");
        assert_eq!(value, LEVER_VALUE);
    } else {
        assert_eq!(label, "not_applicable");
        assert_eq!(value, ABSENT);
    }
}

#[test]
fn preset_lever_is_honoured_and_never_overwritten() {
    let (label, value) = run_child("child_arm_lever_preset_zero", "preset_zero", Some("0"));
    if cfg!(target_os = "linux") {
        assert_eq!(label, "preset_honoured");
    } else {
        assert_eq!(label, "not_applicable");
    }
    assert_eq!(value, "0", "a preset value is never overwritten");
}
