// Agent-headful webview drive: drives the REAL assembled product path in the
// live Tauri window and proves each stage from the app's own record, which the
// boot-quit self-verify cannot (it never clicks — verification-harness.md
// §2026-06-30). Sibling to self_verify.rs; reuses its binary/log/headless
// helpers rather than standing up a second launcher.
//
// Division of labour: the node driver presses and REPORTS observations; THIS
// side asserts. A press that returned without throwing proves nothing — the ACL
// can drop the IPC silently, which is exactly the dead-affordance failure this
// harness exists to catch.
//
// Every obs predicate matches an effect FIELD, never the target alone: boot
// itself emits `ui.layout.transition`, and `viz.query.traces` fires on every 1s
// poll including empty ones, so a target-only match would pass on a record the
// stage did not cause.
//
// Two arms, one leg. `--expect-absent` names a stage that must NOT be observed,
// making the leg's discrimination provable instead of assumed. The cheap
// discriminating arm is `--no-inject`: with no telemetry the telemetry-dependent
// stages must go red while `investigate` stays green (an empty buffer still
// yields a result — pulse-app/tests/integration_investigate_actions.rs).

use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use serde_json::Value;
use tempfile::TempDir;
use tokio::process::Command;

use crate::self_verify::{
    headless_skip_reason, locate_pulse_binary, read_log_lines, workspace_root,
};

const MSEDGEDRIVER_ENV: &str = "ANDROMEDA_PULSE_MSEDGEDRIVER_PATH";
const DRIVE_TIMEOUT: Duration = Duration::from_secs(420);
const TRANSITION_TARGET: &str = "ui.layout.transition";
const HIDDEN_MODE: &str = "hidden";
const WIDGET_LABEL: &str = "compact-widget";
const PANIC_TARGET: &str = "app.panic.fatal";
const STAGE_REPORT_BASENAME: &str = "webview-drive-stages.json";

/// One stage of the assembled path. `id` is what `--expect-absent` names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stage {
    pub id: &'static str,
    pub what: &'static str,
}

pub const STAGES: &[Stage] = &[
    Stage {
        id: "launch",
        what: "the dashboard surface is reachable in the live window",
    },
    Stage {
        id: "traces-empty",
        what: "the trace table renders its empty state before injection",
    },
    Stage {
        id: "traces-populate",
        what: "viz.query.traces reports row_count > 0 and the table filled with no reload",
    },
    Stage {
        id: "storm-incident",
        what: "interpretation.incident.created reports created = true",
    },
    Stage {
        id: "investigate",
        what: "investigate.run_action.request reports status = success after a real press",
    },
    Stage {
        id: "empty-states",
        what: "metrics and logs render the empty state with its exporter hint",
    },
    // Terminal by nature: this press sends the whole app to the tray. Carried
    // forward from the single-press leg rather than retired with it — it is the
    // only guard on `core:window:allow-close`, which the ACL drops silently.
    Stage {
        id: "widget-close",
        what: "ui.layout.transition reports compact-widget → hidden after a real close press",
    },
];

pub fn stage_ids() -> Vec<&'static str> {
    STAGES.iter().map(|s| s.id).collect()
}

pub async fn run_webview_drive(expect_absent: Option<String>, no_inject: bool) -> Result<ExitCode> {
    if let Some(stage) = expect_absent.as_deref()
        && !STAGES.iter().any(|s| s.id == stage)
    {
        bail!(
            "unknown stage {stage:?} for --expect-absent; known stages: {}",
            stage_ids().join(", ")
        );
    }

    if let Some(reason) = headless_skip_reason() {
        println!("webview-drive: SKIP — {reason}");
        return Ok(ExitCode::SUCCESS);
    }

    let workspace_root = workspace_root()?;
    let Some(binary) = locate_pulse_binary(&workspace_root) else {
        println!(
            "webview-drive: SKIP — no pulse-app binary under target/{{release,debug}}; \
             build it first (cargo build -p pulse-app)"
        );
        return Ok(ExitCode::SUCCESS);
    };

    let Some(msedgedriver) = resolve_msedgedriver() else {
        println!(
            "webview-drive: SKIP — {MSEDGEDRIVER_ENV} unset or not a file. tauri-driver needs an \
             msedgedriver matching the installed WebView2 runtime; fetch the matching build from \
             https://msedgedriver.microsoft.com/<version>/edgedriver_win64.zip and point \
             {MSEDGEDRIVER_ENV} at msedgedriver.exe"
        );
        return Ok(ExitCode::SUCCESS);
    };

    let script = workspace_root
        .join("pulse-app")
        .join("ui")
        .join("tests-e2e")
        .join("webview-drive.mjs");
    if !script.exists() {
        bail!("driver script missing at {}", script.display());
    }

    // The injector is built OUTSIDE the timed section and invoked by path: a
    // `cargo run` inside the leg would spend the budget compiling and send zero
    // telemetry, which reads downstream exactly like a broken feature.
    let injector = if no_inject {
        println!("webview-drive: --no-inject — telemetry-dependent stages must go red");
        None
    } else {
        Some(build_injector(&workspace_root).await?)
    };

    let tempdir = TempDir::new().context("create per-run tempdir")?;
    let data_dir = tempdir.path().to_path_buf();
    println!("webview-drive: binary {}", binary.display());
    println!("webview-drive: data_dir {}", data_dir.display());
    println!(
        "webview-drive: arm {}",
        match expect_absent.as_deref() {
            Some(stage) => format!("RED (expect {stage} absent)"),
            None => "GREEN (expect every stage observed)".to_string(),
        }
    );

    let drive = run_driver(
        &script,
        &binary,
        &data_dir,
        &msedgedriver,
        injector.as_deref(),
    );
    match tokio::time::timeout(DRIVE_TIMEOUT, drive).await {
        Ok(Ok(())) => {}
        Ok(Err(e)) if expect_absent.is_none() && !no_inject => return Err(e),
        // A RED arm's driver legitimately fails to reach its later stages; only
        // the observations decide the outcome, so a non-zero exit is not itself
        // a verdict.
        Ok(Err(e)) => {
            println!("webview-drive: driver reported {e} (RED arm — observations decide)")
        }
        Err(_) => bail!("driver did not finish within {DRIVE_TIMEOUT:?}"),
    }

    let log_dir = data_dir.join("logs");
    let records: Vec<Value> = read_log_lines(&log_dir)
        .iter()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .collect();
    let dom = read_stage_report(&data_dir);

    // Per test-plan §3 Direct-binary smoke variant the clean-log assertion binds
    // the GREEN leg only: a RED leg is by construction an unhealthy run, and
    // demanding a clean log over it would contradict what it measures.
    if expect_absent.is_none() && !no_inject {
        let unhealthy = unhealthy_records(&records);
        if !unhealthy.is_empty() {
            eprintln!(
                "webview-drive: FAIL — GREEN arm log carries {} ERROR/panic record(s): {}",
                unhealthy.len(),
                unhealthy.join(", ")
            );
            return Ok(ExitCode::FAILURE);
        }
        println!("webview-drive: clean log — 0 ERROR, 0 app.panic.fatal");
    }

    for stage in STAGES {
        let halves = stage_halves(stage.id, &records, &dom);
        let half = |label: &str, v: Option<bool>| match v {
            None => format!("{label}:n/a"),
            Some(true) => format!("{label}:yes"),
            Some(false) => format!("{label}:NO "),
        };
        println!(
            "webview-drive:   halves {:<16} {} {}",
            stage.id,
            half("obs", halves.obs),
            half("dom", halves.dom)
        );
    }

    // When the close stage goes dark, the useful question is which transitions
    // the app DID write — absence of the target is not absence of the family.
    if !stage_observed("widget-close", &records, &dom) {
        for record in records
            .iter()
            .filter(|r| r.get("target").and_then(Value::as_str) == Some(TRANSITION_TARGET))
        {
            println!("webview-drive:   transition seen: {}", record);
        }
    }

    let observed: Vec<(Stage, bool)> = STAGES
        .iter()
        .map(|stage| (*stage, stage_observed(stage.id, &records, &dom)))
        .collect();
    report(&observed, expect_absent.as_deref())
}

/// Reads the driver's stage report — DOM facts the driver saw, which it
/// deliberately does not judge.
fn read_stage_report(data_dir: &Path) -> Vec<Value> {
    let path = data_dir.join(STAGE_REPORT_BASENAME);
    let Ok(body) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str::<Value>(&body)
        .ok()
        .and_then(|v| v.get("stages").and_then(Value::as_array).cloned())
        .unwrap_or_default()
}

fn dom_stage(dom: &[Value], id: &str) -> Option<Value> {
    dom.iter()
        .find(|s| s.get("stage").and_then(Value::as_str) == Some(id))
        .cloned()
}

fn dom_observed(dom: &[Value], id: &str) -> bool {
    dom_stage(dom, id)
        .and_then(|s| s.get("observed").and_then(Value::as_bool))
        .unwrap_or(false)
}

/// A stage's two independent halves. Reported SEPARATELY, because a combined
/// boolean cannot say whether telemetry failed to land or merely failed to
/// reach the screen — and that is the first question a red stage raises.
/// `None` means the half does not apply to this stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StageHalves {
    pub obs: Option<bool>,
    pub dom: Option<bool>,
}

impl StageHalves {
    fn satisfied(&self) -> bool {
        self.obs.unwrap_or(true) && self.dom.unwrap_or(true)
    }
}

pub fn stage_halves(id: &str, records: &[Value], dom: &[Value]) -> StageHalves {
    let obs = |hit: bool| Some(hit);
    let seen = |stage: &str| Some(dom_observed(dom, stage));
    match id {
        "launch" => StageHalves {
            obs: None,
            dom: seen("launch"),
        },
        "traces-empty" => StageHalves {
            obs: None,
            dom: seen("traces-empty"),
        },
        // Both halves: the app's own query record proves rows reached the read
        // path, and the DOM proves they reached the screen without a reload.
        "traces-populate" => StageHalves {
            obs: obs(records.iter().any(is_traces_rendered)),
            dom: seen("traces-populate"),
        },
        "storm-incident" => StageHalves {
            obs: obs(records.iter().any(is_incident_created)),
            dom: None,
        },
        "investigate" => StageHalves {
            obs: obs(records.iter().any(is_investigate_success)),
            dom: seen("investigate"),
        },
        "empty-states" => StageHalves {
            obs: None,
            dom: Some(dom_observed(dom, "empty-states") && dom_empty_states_have_hints(dom)),
        },
        "widget-close" => StageHalves {
            obs: obs(records.iter().any(is_widget_hidden_transition)),
            dom: seen("widget-close"),
        },
        _ => StageHalves {
            obs: Some(false),
            dom: Some(false),
        },
    }
}

pub fn stage_observed(id: &str, records: &[Value], dom: &[Value]) -> bool {
    stage_halves(id, records, dom).satisfied()
}

/// The hint is the discriminator: the honest error variant renders a message
/// with NO hint, so "a message appeared" would pass on a query failure.
fn dom_empty_states_have_hints(dom: &[Value]) -> bool {
    let Some(stage) = dom_stage(dom, "empty-states") else {
        return false;
    };
    ["metrics", "logs"].iter().all(|route| {
        stage
            .get(route)
            .map(|r| {
                r.get("empty").and_then(Value::as_bool) == Some(true)
                    && r.get("hint").and_then(Value::as_bool) == Some(true)
                    && r.get("error").and_then(Value::as_bool) != Some(true)
            })
            .unwrap_or(false)
    })
}

/// `viz.query.traces` fires on every 1s poll, including the empty ones before
/// injection — only a non-zero `row_count` means rows actually rendered.
fn is_traces_rendered(record: &Value) -> bool {
    if record.get("target").and_then(Value::as_str) != Some("viz.query.traces") {
        return false;
    }
    record
        .get("fields")
        .and_then(|f| f.get("row_count"))
        .and_then(Value::as_u64)
        .is_some_and(|n| n > 0)
}

/// The producer emits this target for a DEDUPED incident too, so `created` is
/// what separates a new incident from a coalesced one.
fn is_incident_created(record: &Value) -> bool {
    if record.get("target").and_then(Value::as_str) != Some("interpretation.incident.created") {
        return false;
    }
    record
        .get("fields")
        .and_then(|f| f.get("created"))
        .and_then(Value::as_bool)
        == Some(true)
}

/// The resolver emits this target on every outcome including validation and
/// runner errors, so `status` is what separates a result from a failure.
fn is_investigate_success(record: &Value) -> bool {
    if record.get("target").and_then(Value::as_str) != Some("investigate.run_action.request") {
        return false;
    }
    record
        .get("fields")
        .and_then(|f| f.get("status"))
        .and_then(Value::as_str)
        == Some("success")
}

// Retained from the single-press leg: the app emits `ui.layout.transition` at
// boot too (`layout_mode_to = "top-right"`), so the close path is the one whose
// destination is `hidden` and whose origin is the widget.
fn is_widget_hidden_transition(record: &Value) -> bool {
    if record.get("target").and_then(Value::as_str) != Some(TRANSITION_TARGET) {
        return false;
    }
    let Some(fields) = record.get("fields") else {
        return false;
    };
    fields.get("layout_mode_to").and_then(Value::as_str) == Some(HIDDEN_MODE)
        && fields.get("layout_mode_from").and_then(Value::as_str) == Some(WIDGET_LABEL)
}

fn unhealthy_records(records: &[Value]) -> Vec<String> {
    records
        .iter()
        .filter(|record| {
            let target = record.get("target").and_then(Value::as_str).unwrap_or("");
            let level = record.get("level").and_then(Value::as_str).unwrap_or("");
            target == PANIC_TARGET || level == "ERROR"
        })
        .map(|record| {
            record
                .get("target")
                .and_then(Value::as_str)
                .unwrap_or("<no target>")
                .to_string()
        })
        .collect()
}

async fn build_injector(workspace_root: &Path) -> Result<PathBuf> {
    println!("webview-drive: building inject_demo (outside the timed section)");
    let status = Command::new("cargo")
        .args(["build", "-p", "ingest", "--example", "inject_demo"])
        .current_dir(workspace_root)
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .status()
        .await
        .context("build inject_demo")?;
    if !status.success() {
        bail!("building inject_demo failed with {status}");
    }
    let exe = if cfg!(windows) {
        "inject_demo.exe"
    } else {
        "inject_demo"
    };
    let path = workspace_root
        .join("target")
        .join("debug")
        .join("examples")
        .join(exe);
    if !path.is_file() {
        bail!("inject_demo built but not found at {}", path.display());
    }
    Ok(path)
}

async fn run_driver(
    script: &Path,
    binary: &Path,
    data_dir: &Path,
    msedgedriver: &Path,
    injector: Option<&Path>,
) -> Result<()> {
    // CWD is the throwaway data dir, not the workspace: tauri-driver spawns the
    // app as a child of this process, and TauRPC's dev-mode `export_types()`
    // writes its bindings RELATIVE to the working directory — pointing anywhere
    // under the repo either clobbers the real bindings or strands a stray copy.
    // Node resolves its own modules from the script path, so CWD is free here.
    let mut cmd = Command::new("node");
    cmd.arg(script)
        .current_dir(data_dir)
        .env("PULSE_BIN", binary)
        .env("PULSE_DATA_DIR", data_dir)
        .env("MSEDGEDRIVER_PATH", msedgedriver)
        .stdin(Stdio::null())
        .kill_on_drop(true);
    if let Some(injector) = injector {
        cmd.env("PULSE_INJECTOR", injector);
    }

    let status = cmd
        .status()
        .await
        .context("spawn node driver (is node on PATH?)")?;

    if !status.success() {
        bail!("driver exited with {status}");
    }
    Ok(())
}

fn report(observed: &[(Stage, bool)], expect_absent: Option<&str>) -> Result<ExitCode> {
    for (stage, seen) in observed {
        println!(
            "webview-drive:   {:<16} {}  — {}",
            stage.id,
            if *seen { "OBSERVED" } else { "absent  " },
            stage.what
        );
    }

    match expect_absent {
        None => {
            let missing: Vec<&str> = observed
                .iter()
                .filter(|(_, seen)| !seen)
                .map(|(stage, _)| stage.id)
                .collect();
            if missing.is_empty() {
                println!(
                    "webview-drive: PASS — the assembled path drove all {} stages",
                    observed.len()
                );
                Ok(ExitCode::SUCCESS)
            } else {
                eprintln!(
                    "webview-drive: FAIL — {} stage(s) never reached their observable: {}",
                    missing.len(),
                    missing.join(", ")
                );
                Ok(ExitCode::FAILURE)
            }
        }
        Some(target) => {
            let seen = observed
                .iter()
                .find(|(stage, _)| stage.id == target)
                .map(|(_, seen)| *seen)
                .unwrap_or(false);
            if seen {
                eprintln!(
                    "webview-drive: FAIL (RED arm) — {target} was observed anyway, so the leg does \
                     not discriminate and would pass against a dead stage"
                );
                Ok(ExitCode::FAILURE)
            } else {
                println!(
                    "webview-drive: PASS (RED arm) — {target} absent, so the leg discriminates"
                );
                Ok(ExitCode::SUCCESS)
            }
        }
    }
}

fn resolve_msedgedriver() -> Option<PathBuf> {
    let raw = std::env::var(MSEDGEDRIVER_ENV).ok()?;
    let path = PathBuf::from(raw.trim());
    path.is_file().then_some(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn dom(stages: Vec<Value>) -> Vec<Value> {
        stages
    }

    #[test]
    fn traces_rendered_requires_a_nonzero_row_count() {
        let record = json!({
            "target": "viz.query.traces",
            "fields": { "row_count": 12, "query_id": "viz.query.traces" }
        });
        assert!(is_traces_rendered(&record));
    }

    #[test]
    fn empty_traces_poll_is_not_a_render() {
        // The 1s poll emits this target continuously before injection; matching
        // the target alone would pass here.
        let record = json!({
            "target": "viz.query.traces",
            "fields": { "row_count": 0, "query_id": "viz.query.traces" }
        });
        assert!(!is_traces_rendered(&record));
    }

    #[test]
    fn metrics_query_is_not_a_traces_render() {
        let record = json!({
            "target": "viz.query.metrics",
            "fields": { "row_count": 12 }
        });
        assert!(!is_traces_rendered(&record));
    }

    #[test]
    fn incident_created_requires_created_true() {
        let record = json!({
            "target": "interpretation.incident.created",
            "fields": { "created": true, "deduped": false, "severity": "error" }
        });
        assert!(is_incident_created(&record));
    }

    #[test]
    fn deduped_incident_is_not_a_creation() {
        let record = json!({
            "target": "interpretation.incident.created",
            "fields": { "created": false, "deduped": true, "severity": "error" }
        });
        assert!(!is_incident_created(&record));
    }

    #[test]
    fn investigate_success_requires_status_success() {
        let record = json!({
            "target": "investigate.run_action.request",
            "fields": { "status": "success", "action_id": "diagnose-latency-outlier" }
        });
        assert!(is_investigate_success(&record));
    }

    #[test]
    fn investigate_error_outcome_is_not_a_result() {
        let record = json!({
            "target": "investigate.run_action.request",
            "fields": { "status": "error", "action_id": "diagnose-latency-outlier" }
        });
        assert!(!is_investigate_success(&record));
    }

    #[test]
    fn empty_states_require_the_hint_on_both_routes() {
        let stages = dom(vec![json!({
            "stage": "empty-states",
            "observed": true,
            "metrics": { "empty": true, "hint": true, "error": false },
            "logs": { "empty": true, "hint": true, "error": false }
        })]);
        assert!(dom_empty_states_have_hints(&stages));
        assert!(stage_observed("empty-states", &[], &stages));
    }

    #[test]
    fn hintless_error_state_does_not_satisfy_empty_states() {
        // The error variant renders a message with no hint — "a message
        // appeared" would pass here, which is the whole reason the hint is the
        // discriminator.
        let stages = dom(vec![json!({
            "stage": "empty-states",
            "observed": true,
            "metrics": { "empty": false, "hint": false, "error": true },
            "logs": { "empty": true, "hint": true, "error": false }
        })]);
        assert!(!dom_empty_states_have_hints(&stages));
    }

    #[test]
    fn traces_populate_needs_both_the_record_and_the_dom() {
        let records = vec![json!({
            "target": "viz.query.traces",
            "fields": { "row_count": 5 }
        })];
        let with_dom = dom(vec![
            json!({ "stage": "traces-populate", "observed": true }),
        ]);
        let without_dom = dom(vec![
            json!({ "stage": "traces-populate", "observed": false }),
        ]);
        assert!(stage_observed("traces-populate", &records, &with_dom));
        assert!(!stage_observed("traces-populate", &records, &without_dom));
        assert!(!stage_observed("traces-populate", &[], &with_dom));
    }

    #[test]
    fn investigate_needs_both_the_press_and_the_outcome() {
        let records = vec![json!({
            "target": "investigate.run_action.request",
            "fields": { "status": "success" }
        })];
        let pressed = dom(vec![json!({ "stage": "investigate", "observed": true })]);
        let not_pressed = dom(vec![json!({ "stage": "investigate", "observed": false })]);
        assert!(stage_observed("investigate", &records, &pressed));
        assert!(!stage_observed("investigate", &records, &not_pressed));
    }

    #[test]
    fn widget_close_needs_both_the_press_and_the_transition() {
        let records = vec![json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "compact-widget", "layout_mode_to": "hidden" }
        })];
        let pressed = dom(vec![json!({ "stage": "widget-close", "observed": true })]);
        let not_pressed = dom(vec![json!({ "stage": "widget-close", "observed": false })]);
        assert!(stage_observed("widget-close", &records, &pressed));
        assert!(!stage_observed("widget-close", &records, &not_pressed));
        // The dead-affordance mode this whole harness exists to catch: the
        // press lands, the ACL drops the IPC, no transition is ever written.
        assert!(!stage_observed("widget-close", &[], &pressed));
    }

    #[test]
    fn halves_separate_a_missing_record_from_a_missing_render() {
        let with_record = vec![json!({
            "target": "viz.query.traces",
            "fields": { "row_count": 5 }
        })];
        let no_dom = dom(vec![
            json!({ "stage": "traces-populate", "observed": false }),
        ]);
        let halves = stage_halves("traces-populate", &with_record, &no_dom);
        assert_eq!(halves.obs, Some(true), "telemetry DID land");
        assert_eq!(halves.dom, Some(false), "but never reached the screen");
        assert!(!halves.satisfied());

        let with_dom = dom(vec![
            json!({ "stage": "traces-populate", "observed": true }),
        ]);
        let halves = stage_halves("traces-populate", &[], &with_dom);
        assert_eq!(halves.obs, Some(false), "telemetry never landed");
        assert_eq!(halves.dom, Some(true));
        assert!(!halves.satisfied());
    }

    #[test]
    fn an_inapplicable_half_does_not_veto_a_stage() {
        // storm-incident has no DOM half; a None must not read as false.
        let records = vec![json!({
            "target": "interpretation.incident.created",
            "fields": { "created": true }
        })];
        let halves = stage_halves("storm-incident", &records, &[]);
        assert_eq!(halves.dom, None);
        assert!(halves.satisfied());
    }

    #[test]
    fn a_missing_stage_report_leaves_dom_stages_absent() {
        assert!(!stage_observed("launch", &[], &[]));
        assert!(!stage_observed("traces-empty", &[], &[]));
    }

    #[test]
    fn unknown_stage_ids_are_never_observed() {
        let stages = dom(vec![json!({ "stage": "not-a-stage", "observed": true })]);
        assert!(!stage_observed("not-a-stage", &[], &stages));
    }

    #[test]
    fn widget_hidden_transition_is_recognized() {
        let record = json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "compact-widget", "layout_mode_to": "hidden" }
        });
        assert!(is_widget_hidden_transition(&record));
    }

    #[test]
    fn boot_geometry_transition_is_not_a_close() {
        let record = json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "boot_default", "layout_mode_to": "top-right" }
        });
        assert!(!is_widget_hidden_transition(&record));
    }

    #[test]
    fn dashboard_hide_is_not_the_widget_close() {
        let record = json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "main", "layout_mode_to": "hidden" }
        });
        assert!(!is_widget_hidden_transition(&record));
    }

    #[test]
    fn other_targets_are_ignored() {
        let record = json!({
            "target": "tray.signpost.shown",
            "fields": { "layout_mode_to": "hidden", "layout_mode_from": "compact-widget" }
        });
        assert!(!is_widget_hidden_transition(&record));
    }

    #[test]
    fn unhealthy_records_flag_errors_and_panics() {
        let records = vec![
            json!({ "target": "buffer.tick", "level": "INFO" }),
            json!({ "target": "duckdb.append", "level": "ERROR" }),
            json!({ "target": "app.panic.fatal", "level": "WARN" }),
        ];
        let flagged = unhealthy_records(&records);
        assert_eq!(flagged, vec!["duckdb.append", "app.panic.fatal"]);
    }

    #[test]
    fn unhealthy_records_empty_on_a_clean_log() {
        let records = vec![
            json!({ "target": "buffer.tick", "level": "INFO" }),
            json!({ "target": "ui.layout.transition", "level": "INFO" }),
        ];
        assert!(unhealthy_records(&records).is_empty());
    }

    #[test]
    fn green_arm_fails_when_any_stage_is_absent() {
        let observed = vec![(STAGES[0], true), (STAGES[1], false)];
        let code = report(&observed, None).expect("report");
        assert_eq!(code, ExitCode::FAILURE);
    }

    #[test]
    fn green_arm_passes_when_every_stage_is_observed() {
        let observed: Vec<(Stage, bool)> = STAGES.iter().map(|s| (*s, true)).collect();
        let code = report(&observed, None).expect("report");
        assert_eq!(code, ExitCode::SUCCESS);
    }

    #[test]
    fn red_arm_fails_when_the_named_stage_is_present() {
        let observed: Vec<(Stage, bool)> = STAGES.iter().map(|s| (*s, true)).collect();
        let code = report(&observed, Some("storm-incident")).expect("report");
        assert_eq!(code, ExitCode::FAILURE);
    }

    #[test]
    fn red_arm_passes_when_only_the_named_stage_is_absent() {
        let observed: Vec<(Stage, bool)> = STAGES
            .iter()
            .map(|s| (*s, s.id != "storm-incident"))
            .collect();
        let code = report(&observed, Some("storm-incident")).expect("report");
        assert_eq!(code, ExitCode::SUCCESS);
    }

    #[test]
    fn stage_ids_are_unique_and_nonempty() {
        let ids = stage_ids();
        assert!(!ids.is_empty());
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "stage ids must be unique");
    }
}
