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
const MAIN_LABEL: &str = "main";
const SIGNPOST_TARGET: &str = "tray.signpost.shown";
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
        id: "traces-scroll",
        what: "the trace table scrolls inside its own region while the page does not (P-082)",
    },
    // The mechanics probe measured that no synthesized OS pointer input reaches
    // this window, so a NATIVE WebView2 menu can neither be opened nor observed
    // by the driver. This is the sanctioned fallback: the SUPPRESSION side, in
    // the shipped production bundle — which vitest cannot reach (it runs the
    // hook in jsdom under a stubbed PROD flag). Ordered right after
    // traces-scroll so it observes the /traces route the leg is already on,
    // rather than navigating and perturbing every stage downstream.
    Stage {
        id: "native-menu-suppressed",
        what: "a real contextmenu in the live window is defaultPrevented, so no WebView2 menu shows (P-064)",
    },
    Stage {
        id: "connection-status",
        what: "the dashboard footer words services + spans/s + buffer, absent on the widget (P-070)",
    },
    Stage {
        id: "storm-incident",
        what: "interpretation.incident.created reports created = true",
    },
    Stage {
        id: "findings-window",
        what: "the widget badge opens the findings window docked below it with incident rows",
    },
    Stage {
        id: "report-window",
        what: "a findings row opens the report beside it; Esc unwinds with focus restored to the badge",
    },
    Stage {
        id: "investigate",
        what: "investigate.run_action.request reports status = success after a real press",
    },
    Stage {
        id: "empty-states",
        what: "metrics and logs render the empty state with its exporter hint",
    },
    Stage {
        id: "dashboard-toggle",
        what: "the toggle button and Ctrl+Shift+P each hide/show the dashboard; the widget stays (P-066)",
    },
    Stage {
        id: "dashboard-close",
        what: "the dashboard's own close collapses to the widget: main → hidden, no signpost (P-063)",
    },
    // This press sends the whole app to the tray. It WAS terminal;
    // signpost-repeat now follows it deliberately, restoring the widget to prove
    // the toast fires on every close. Carried forward from the single-press leg;
    // `allow-close` is guarded here and by dashboard-close (the ACL drops either
    // close silently when revoked).
    Stage {
        id: "widget-close",
        what: "ui.layout.transition reports compact-widget → hidden after a real close press",
    },
    // The every-time half of P-063: one close proves the signpost fires, never
    // that it fires EVERY time. Proving repetition needs a second close, hence a
    // restore — and tray restore is an OS surface no synthesized input reaches
    // (measured by the probe), so the re-show is programmatic SETUP while both
    // closes stay real affordance presses.
    Stage {
        id: "signpost-repeat",
        what: "a SECOND real widget close emits a second labelled tray.signpost.shown (P-063 every-time)",
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
    print_mechanics_probe(&dom);

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
        // DOM-only, field-level: the driver records layout facts; the verdict
        // re-derives from those fields here rather than trusting `observed`.
        "traces-scroll" => StageHalves {
            obs: None,
            dom: Some(dom_traces_scroll_ok(dom)),
        },
        "connection-status" => StageHalves {
            obs: None,
            dom: Some(dom_connection_status_ok(dom)),
        },
        "storm-incident" => StageHalves {
            obs: obs(records.iter().any(is_incident_created)),
            dom: None,
        },
        // The findings/report lifecycle is JS `hide()`/`show()` end to end —
        // no Rust hook, no obs record — so these two are DOM-only by
        // construction (the launch/traces-empty precedent).
        "findings-window" => StageHalves {
            obs: None,
            dom: Some(dom_findings_window_ok(dom)),
        },
        "report-window" => StageHalves {
            obs: None,
            dom: Some(dom_report_window_ok(dom)),
        },
        "investigate" => StageHalves {
            obs: obs(records.iter().any(is_investigate_success)),
            dom: seen("investigate"),
        },
        "empty-states" => StageHalves {
            obs: None,
            dom: Some(dom_observed(dom, "empty-states") && dom_empty_states_have_hints(dom)),
        },
        // DOM-only by construction: suppression is a webview-side preventDefault
        // with no Rust hook. The field, not the stage flag, is the verdict — a
        // dispatch that never ran would leave it null and read as false.
        "native-menu-suppressed" => StageHalves {
            obs: None,
            dom: Some(
                dom_stage(dom, "native-menu-suppressed")
                    .and_then(|s| s.get("context_menu_prevented").and_then(Value::as_bool))
                    .unwrap_or(false),
            ),
        },
        // Toggle hide/show run through JS `hide()`/`show()` (no record either
        // direction), so the toggle is DOM-only — which is also why the
        // `main → hidden` record below is unique to the dashboard ✕.
        "dashboard-toggle" => StageHalves {
            obs: None,
            dom: Some(dom_dashboard_toggle_ok(dom)),
        },
        "dashboard-close" => StageHalves {
            obs: obs(records.iter().any(is_dashboard_hidden_transition)),
            dom: Some(dom_dashboard_close_ok(dom)),
        },
        // Both records are required: the transition proves the hide reached the
        // Rust handler, the signpost (with its bounded label field) proves the
        // every-time P-063 toast fired for the widget and not another window.
        "widget-close" => StageHalves {
            obs: obs(records.iter().any(is_widget_hidden_transition)
                && records.iter().any(is_close_signpost)),
            dom: seen("widget-close"),
        },
        // COUNT, not presence: widget-close above already proves one labelled
        // signpost exists, so a presence check here would pass on that same
        // record and prove nothing about repetition. Two is what "every time"
        // needs, and the DOM half requires the second press actually landed.
        "signpost-repeat" => StageHalves {
            obs: obs(records.iter().filter(|r| is_close_signpost(r)).count() >= 2),
            dom: Some(
                dom_stage(dom, "signpost-repeat")
                    .and_then(|s| s.get("second_close_pressed").and_then(Value::as_bool))
                    .unwrap_or(false),
            ),
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

/// The toggle path hides `main` via JS `hide()` (no Rust hook, no record), so
/// a `main → hidden` transition is written ONLY by the dashboard ✕
/// (CloseRequested → handle_close_to_tray) — the record attributes the press.
fn is_dashboard_hidden_transition(record: &Value) -> bool {
    if record.get("target").and_then(Value::as_str) != Some(TRANSITION_TARGET) {
        return false;
    }
    let Some(fields) = record.get("fields") else {
        return false;
    };
    fields.get("layout_mode_to").and_then(Value::as_str) == Some(HIDDEN_MODE)
        && fields.get("layout_mode_from").and_then(Value::as_str) == Some(MAIN_LABEL)
}

/// The P-063 signpost record. The bounded `window_label` field is what makes
/// this assertable at field level (its exact allowlist leaf permits it); a
/// label other than the widget's would mean the toast fired for the wrong
/// close, so bare target presence is deliberately not enough.
fn is_close_signpost(record: &Value) -> bool {
    if record.get("target").and_then(Value::as_str) != Some(SIGNPOST_TARGET) {
        return false;
    }
    record
        .get("fields")
        .and_then(|f| f.get("window_label"))
        .and_then(Value::as_str)
        == Some(WIDGET_LABEL)
}

fn stage_bool(stage: &Value, field: &str) -> Option<bool> {
    stage.get(field).and_then(Value::as_bool)
}

/// Internal scroll (P-082): the table's own region overflows while the page
/// does not. Presence of the region alone would pass on a page that scrolls at
/// the outer level — the exact defect the layout chunk closed.
fn dom_traces_scroll_ok(dom: &[Value]) -> bool {
    let Some(stage) = dom_stage(dom, "traces-scroll") else {
        return false;
    };
    stage_bool(&stage, "scroll_region_found") == Some(true)
        && stage_bool(&stage, "region_overflows") == Some(true)
        && stage_bool(&stage, "page_overflows") == Some(false)
}

/// Footer line (P-070): the worded services / spans-per-second / buffer parts
/// each matched on the dashboard, and the line is absent on the compact widget
/// (layout-templates keeps the widget aggregate-glance — a line there is a
/// defect, not extra coverage).
fn dom_connection_status_ok(dom: &[Value]) -> bool {
    let Some(stage) = dom_stage(dom, "connection-status") else {
        return false;
    };
    stage_bool(&stage, "line_on_dashboard") == Some(true)
        && stage_bool(&stage, "live_kind") == Some(true)
        && stage_bool(&stage, "matched_services") == Some(true)
        && stage_bool(&stage, "matched_spans_rate") == Some(true)
        && stage_bool(&stage, "matched_buffer") == Some(true)
        && stage_bool(&stage, "line_on_widget") == Some(false)
}

/// Findings disclosure, first half: a real badge press, the findings window
/// reported visible by the window API, at least one incident row in its DOM,
/// and the dock relation (below the widget) derived from the geometry getters
/// — derived booleans only, never raw coordinates (security-plan §Input
/// Validation → Persisted window geometry).
fn dom_findings_window_ok(dom: &[Value]) -> bool {
    let Some(stage) = dom_stage(dom, "findings-window") else {
        return false;
    };
    stage_bool(&stage, "badge_pressed") == Some(true)
        && stage_bool(&stage, "findings_visible") == Some(true)
        && stage_bool(&stage, "docked_below") == Some(true)
        && stage
            .get("row_count")
            .and_then(Value::as_u64)
            .is_some_and(|n| n > 0)
}

/// Findings disclosure, second half: row-select opens the report beside the
/// findings window, and the Esc chain unwinds it — report hides with focus
/// returning to findings, findings hides with DOM focus restored to the
/// widget's badge (a11y-plan §5 Focus restoration, SC 2.1.2).
fn dom_report_window_ok(dom: &[Value]) -> bool {
    let Some(stage) = dom_stage(dom, "report-window") else {
        return false;
    };
    [
        "row_pressed",
        "report_visible",
        "dialog_shown",
        "positioned_beside",
        "report_hidden_after_escape",
        "findings_hidden_after_escape",
        "badge_focus_restored",
    ]
    .iter()
    .all(|field| stage_bool(&stage, field) == Some(true))
}

/// Widget↔dashboard toggle (P-066): every press must FLIP the visibility it
/// found (open-if-hidden / hide-if-shown) — two button presses cover both
/// directions between them, the shortcut fires once from each window, and the
/// widget stays visible throughout. One route passing does not cover the
/// other, and a flip predicate cannot pass on a press that did nothing.
fn dom_dashboard_toggle_ok(dom: &[Value]) -> bool {
    let Some(stage) = dom_stage(dom, "dashboard-toggle") else {
        return false;
    };
    [
        "button_flip_1",
        "button_flip_2",
        "shortcut_flip_dashboard",
        "shortcut_flip_widget",
        "widget_stayed",
    ]
    .iter()
    .all(|field| stage_bool(&stage, field) == Some(true))
}

/// Dashboard ✕ (P-063): the press landed, the window API reports `main`
/// hidden, and the signpost count did not move — the dashboard collapse is
/// silent by design; only the widget close signposts.
fn dom_dashboard_close_ok(dom: &[Value]) -> bool {
    let Some(stage) = dom_stage(dom, "dashboard-close") else {
        return false;
    };
    stage_bool(&stage, "close_pressed") == Some(true)
        && stage_bool(&stage, "main_hidden") == Some(true)
        && stage.get("signpost_delta").and_then(Value::as_u64) == Some(0)
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

/// Print what the driver's mechanics probe measured about this host's transport.
///
/// Deliberately NOT a stage: an unsupported mechanism is a finding, not a
/// failure, so it must never move the leg's verdict. `verdict` reads the delta
/// rather than whether the call threw — a driver command can return cleanly and
/// move nothing, which is the whole reason the probe measures the effect.
pub fn print_mechanics_probe(dom: &[Value]) {
    let Some(probe) = dom_stage(dom, "mechanics-probe") else {
        println!("webview-drive: mechanics-probe — not recorded (driver did not reach it)");
        return;
    };
    let field = |k: &str| probe.get(k).cloned().unwrap_or(Value::Null);
    let delta_moved = |k: &str, a: &str, b: &str| match probe.get(k) {
        Some(Value::Object(d)) => {
            let get = |n: &str| d.get(n).and_then(Value::as_i64).unwrap_or(0);
            Some(get(a) != 0 || get(b) != 0)
        }
        _ => None,
    };
    let verdict = |moved: Option<bool>| match moved {
        Some(true) => "SUPPORTED",
        Some(false) => "no effect",
        None => "unmeasured",
    };
    println!(
        "webview-drive: mechanics-probe — geometry_readable={} drag_region_found={}",
        field("geometry_readable"),
        field("drag_region_found")
    );
    println!(
        "webview-drive:   pointer Actions  ran={} delta={} => {}",
        field("pointer_actions_ran"),
        field("drag_delta"),
        verdict(delta_moved("drag_delta", "dx", "dy"))
    );
    println!(
        "webview-drive:   setWindowRect    ran={} delta={} => {}",
        field("set_window_rect_ran"),
        field("resize_delta"),
        verdict(delta_moved("resize_delta", "dw", "dh"))
    );
    for key in [
        "pointer_actions_error",
        "set_window_rect_error",
        "probe_error",
    ] {
        if let Some(Value::String(msg)) = probe.get(key) {
            println!("webview-drive:   {key}: {msg}");
        }
    }
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

    fn signpost(label: &str) -> Value {
        json!({
            "target": "tray.signpost.shown",
            "fields": { "window_label": label }
        })
    }

    #[test]
    fn signpost_repeat_needs_two_signposts_not_one() {
        // widget-close already proves ONE labelled signpost exists, so a
        // presence check here would pass on that same record and prove nothing
        // about repetition — the count is the whole assertion.
        let pressed = dom(vec![json!({
            "stage": "signpost-repeat",
            "observed": true,
            "second_close_pressed": true
        })]);
        let one = vec![signpost("compact-widget")];
        let two = vec![signpost("compact-widget"), signpost("compact-widget")];
        assert!(!stage_observed("signpost-repeat", &one, &pressed));
        assert!(stage_observed("signpost-repeat", &two, &pressed));
    }

    #[test]
    fn signpost_repeat_ignores_other_windows_signposts() {
        // A second signpost carrying a different window label is not a second
        // WIDGET close; counting bare records would let it pass.
        let pressed = dom(vec![json!({
            "stage": "signpost-repeat",
            "observed": true,
            "second_close_pressed": true
        })]);
        let mixed = vec![signpost("compact-widget"), signpost("main")];
        assert!(!stage_observed("signpost-repeat", &mixed, &pressed));
    }

    #[test]
    fn signpost_repeat_needs_the_second_press_to_have_landed() {
        // Two signposts with no second press means the count came from
        // somewhere other than the affordance under test.
        let unpressed = dom(vec![json!({
            "stage": "signpost-repeat",
            "observed": false,
            "second_close_pressed": false
        })]);
        let two = vec![signpost("compact-widget"), signpost("compact-widget")];
        assert!(!stage_observed("signpost-repeat", &two, &unpressed));
    }

    #[test]
    fn native_menu_keys_on_the_prevented_field_not_the_stage_flag() {
        // A dispatch that never ran leaves the field null; the stage flag alone
        // would report a suppression that was never measured.
        let prevented = dom(vec![json!({
            "stage": "native-menu-suppressed",
            "observed": true,
            "context_menu_prevented": true
        })]);
        let not_prevented = dom(vec![json!({
            "stage": "native-menu-suppressed",
            "observed": true,
            "context_menu_prevented": false
        })]);
        let never_dispatched = dom(vec![json!({
            "stage": "native-menu-suppressed",
            "observed": true,
            "context_menu_prevented": Value::Null
        })]);
        assert!(stage_observed("native-menu-suppressed", &[], &prevented));
        assert!(!stage_observed(
            "native-menu-suppressed",
            &[],
            &not_prevented
        ));
        assert!(!stage_observed(
            "native-menu-suppressed",
            &[],
            &never_dispatched
        ));
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
        let records = vec![
            json!({
                "target": "ui.layout.transition",
                "fields": { "layout_mode_from": "compact-widget", "layout_mode_to": "hidden" }
            }),
            json!({
                "target": "tray.signpost.shown",
                "fields": { "window_label": "compact-widget" }
            }),
        ];
        let pressed = dom(vec![json!({ "stage": "widget-close", "observed": true })]);
        let not_pressed = dom(vec![json!({ "stage": "widget-close", "observed": false })]);
        assert!(stage_observed("widget-close", &records, &pressed));
        assert!(!stage_observed("widget-close", &records, &not_pressed));
        // The dead-affordance mode this whole harness exists to catch: the
        // press lands, the ACL drops the IPC, no transition is ever written.
        assert!(!stage_observed("widget-close", &[], &pressed));
    }

    #[test]
    fn widget_close_transition_alone_no_longer_satisfies() {
        // The P-063 signpost is part of the close contract: a run where the
        // hide landed but the every-time toast never fired must go red.
        let transition_only = vec![json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "compact-widget", "layout_mode_to": "hidden" }
        })];
        let pressed = dom(vec![json!({ "stage": "widget-close", "observed": true })]);
        assert!(!stage_observed("widget-close", &transition_only, &pressed));
    }

    #[test]
    fn signpost_with_another_windows_label_does_not_satisfy() {
        let record = json!({
            "target": "tray.signpost.shown",
            "fields": { "window_label": "main" }
        });
        assert!(!is_close_signpost(&record));
    }

    #[test]
    fn close_signpost_requires_its_label_field() {
        // A fieldless record is bare presence — exactly what the exact
        // allowlist leaf + the field predicate exist to rule out.
        let record = json!({ "target": "tray.signpost.shown", "fields": {} });
        assert!(!is_close_signpost(&record));
        let with_label = json!({
            "target": "tray.signpost.shown",
            "fields": { "window_label": "compact-widget" }
        });
        assert!(is_close_signpost(&with_label));
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
    fn dashboard_hidden_transition_is_recognized() {
        let record = json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "main", "layout_mode_to": "hidden" }
        });
        assert!(is_dashboard_hidden_transition(&record));
    }

    #[test]
    fn widget_close_is_not_the_dashboard_close() {
        let record = json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "compact-widget", "layout_mode_to": "hidden" }
        });
        assert!(!is_dashboard_hidden_transition(&record));
    }

    #[test]
    fn boot_geometry_is_not_the_dashboard_close() {
        let record = json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "boot_default", "layout_mode_to": "top-right" }
        });
        assert!(!is_dashboard_hidden_transition(&record));
    }

    #[test]
    fn traces_scroll_requires_region_overflow_without_page_overflow() {
        let good = dom(vec![json!({
            "stage": "traces-scroll", "observed": true,
            "scroll_region_found": true, "region_overflows": true, "page_overflows": false
        })]);
        assert!(dom_traces_scroll_ok(&good));
        assert!(stage_observed("traces-scroll", &[], &good));
    }

    #[test]
    fn a_scrolling_page_fails_traces_scroll() {
        // The outer page scrolling is the P-082 defect itself — a stage that
        // only checked the region's presence would pass here.
        let page_scrolls = dom(vec![json!({
            "stage": "traces-scroll", "observed": true,
            "scroll_region_found": true, "region_overflows": true, "page_overflows": true
        })]);
        assert!(!dom_traces_scroll_ok(&page_scrolls));
        let no_overflow = dom(vec![json!({
            "stage": "traces-scroll", "observed": true,
            "scroll_region_found": true, "region_overflows": false, "page_overflows": false
        })]);
        assert!(!dom_traces_scroll_ok(&no_overflow));
    }

    #[test]
    fn connection_status_requires_every_worded_part_and_widget_absence() {
        let good = dom(vec![json!({
            "stage": "connection-status", "observed": true,
            "line_on_dashboard": true, "live_kind": true, "matched_services": true,
            "matched_spans_rate": true, "matched_buffer": true, "line_on_widget": false
        })]);
        assert!(dom_connection_status_ok(&good));
    }

    #[test]
    fn a_line_on_the_widget_fails_connection_status() {
        // layout-templates keeps the compact widget aggregate-glance: a worded
        // line there is a defect, so its presence must redden the stage.
        let widget_line = dom(vec![json!({
            "stage": "connection-status", "observed": true,
            "line_on_dashboard": true, "live_kind": true, "matched_services": true,
            "matched_spans_rate": true, "matched_buffer": true, "line_on_widget": true
        })]);
        assert!(!dom_connection_status_ok(&widget_line));
        let empty_state_kind = dom(vec![json!({
            "stage": "connection-status", "observed": true,
            "line_on_dashboard": true, "live_kind": false, "matched_services": false,
            "matched_spans_rate": false, "matched_buffer": false, "line_on_widget": false
        })]);
        assert!(!dom_connection_status_ok(&empty_state_kind));
    }

    #[test]
    fn findings_window_requires_press_visibility_rows_and_dock() {
        let good = dom(vec![json!({
            "stage": "findings-window", "observed": true,
            "badge_pressed": true, "findings_visible": true,
            "docked_below": true, "row_count": 2
        })]);
        assert!(dom_findings_window_ok(&good));
    }

    #[test]
    fn an_undocked_or_empty_findings_window_fails() {
        let undocked = dom(vec![json!({
            "stage": "findings-window", "observed": true,
            "badge_pressed": true, "findings_visible": true,
            "docked_below": false, "row_count": 2
        })]);
        assert!(!dom_findings_window_ok(&undocked));
        let no_rows = dom(vec![json!({
            "stage": "findings-window", "observed": true,
            "badge_pressed": true, "findings_visible": true,
            "docked_below": true, "row_count": 0
        })]);
        assert!(!dom_findings_window_ok(&no_rows));
    }

    #[test]
    fn report_window_requires_the_full_escape_chain() {
        let good = dom(vec![json!({
            "stage": "report-window", "observed": true,
            "row_pressed": true, "report_visible": true, "dialog_shown": true,
            "positioned_beside": true, "report_hidden_after_escape": true,
            "findings_hidden_after_escape": true, "badge_focus_restored": true
        })]);
        assert!(dom_report_window_ok(&good));
    }

    #[test]
    fn a_lost_focus_restore_fails_report_window() {
        // SC 2.1.2: the escape chain is only proven when focus lands back on
        // the badge — a report that closes into a focus void must go red.
        let focus_lost = dom(vec![json!({
            "stage": "report-window", "observed": true,
            "row_pressed": true, "report_visible": true, "dialog_shown": true,
            "positioned_beside": true, "report_hidden_after_escape": true,
            "findings_hidden_after_escape": true, "badge_focus_restored": false
        })]);
        assert!(!dom_report_window_ok(&focus_lost));
    }

    #[test]
    fn dashboard_toggle_requires_both_routes_and_the_widget_staying() {
        let good = dom(vec![json!({
            "stage": "dashboard-toggle", "observed": true,
            "button_flip_1": true, "button_flip_2": true,
            "shortcut_flip_dashboard": true, "shortcut_flip_widget": true,
            "widget_stayed": true
        })]);
        assert!(dom_dashboard_toggle_ok(&good));
    }

    #[test]
    fn a_dead_shortcut_route_fails_the_toggle() {
        // Design requires a verdict per route: the button passing does not
        // cover Ctrl+Shift+P (the shortcut once fell through to the browser
        // print dialog while the button worked).
        let shortcut_dead = dom(vec![json!({
            "stage": "dashboard-toggle", "observed": true,
            "button_flip_1": true, "button_flip_2": true,
            "shortcut_flip_dashboard": false, "shortcut_flip_widget": false,
            "widget_stayed": true
        })]);
        assert!(!dom_dashboard_toggle_ok(&shortcut_dead));
        let widget_vanished = dom(vec![json!({
            "stage": "dashboard-toggle", "observed": true,
            "button_flip_1": true, "button_flip_2": true,
            "shortcut_flip_dashboard": true, "shortcut_flip_widget": true,
            "widget_stayed": false
        })]);
        assert!(!dom_dashboard_toggle_ok(&widget_vanished));
    }

    #[test]
    fn dashboard_close_needs_press_hide_and_a_silent_signpost() {
        let good = dom(vec![json!({
            "stage": "dashboard-close", "observed": true,
            "close_pressed": true, "main_hidden": true, "signpost_delta": 0
        })]);
        assert!(dom_dashboard_close_ok(&good));
        let records = vec![json!({
            "target": "ui.layout.transition",
            "fields": { "layout_mode_from": "main", "layout_mode_to": "hidden" }
        })];
        assert!(stage_observed("dashboard-close", &records, &good));
        // Both halves required: the record without the press facts, or the
        // press facts without the record, each leave the stage red.
        assert!(!stage_observed("dashboard-close", &records, &[]));
        assert!(!stage_observed("dashboard-close", &[], &good));
    }

    #[test]
    fn a_signposting_dashboard_close_fails() {
        // The dashboard collapse is silent by design (P-063): a toast firing
        // here means the close model regressed to the widget's path.
        let toasted = dom(vec![json!({
            "stage": "dashboard-close", "observed": true,
            "close_pressed": true, "main_hidden": true, "signpost_delta": 1
        })]);
        assert!(!dom_dashboard_close_ok(&toasted));
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
