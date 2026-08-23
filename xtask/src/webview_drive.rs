// Agent-headful webview drive: proves a real user control in the live Tauri
// window still works, which the boot-quit self-verify cannot (it never clicks —
// verification-harness.md §2026-06-30). Sibling to self_verify.rs; reuses its
// binary/log/headless helpers rather than standing up a second launcher.
//
// Division of labour: the node driver presses the control, and THIS side reads
// the app's own `ui.layout.transition` record out of the obs log. A press that
// returned without throwing proves nothing — the ACL can drop the IPC silently,
// which is exactly the dead-affordance failure this harness exists to catch.
//
// Two arms, one leg. `--expect-absent` inverts the assertion so the same code
// path measures the mutation check (grant revoked ⇒ no transition), making the
// leg's discrimination provable instead of assumed.

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
const DRIVE_TIMEOUT: Duration = Duration::from_secs(240);
const TRANSITION_TARGET: &str = "ui.layout.transition";
const HIDDEN_MODE: &str = "hidden";
const WIDGET_LABEL: &str = "compact-widget";
const PANIC_TARGET: &str = "app.panic.fatal";

pub async fn run_webview_drive(expect_absent: bool) -> Result<ExitCode> {
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

    let tempdir = TempDir::new().context("create per-run tempdir")?;
    let data_dir = tempdir.path().to_path_buf();
    println!("webview-drive: binary {}", binary.display());
    println!("webview-drive: data_dir {}", data_dir.display());
    println!(
        "webview-drive: arm {}",
        if expect_absent {
            "RED (expect no hidden-transition)"
        } else {
            "GREEN (expect hidden-transition)"
        }
    );

    let drive = run_driver(&script, &binary, &data_dir, &msedgedriver);
    match tokio::time::timeout(DRIVE_TIMEOUT, drive).await {
        Ok(Ok(())) => {}
        Ok(Err(e)) if !expect_absent => return Err(e),
        // The RED arm's press legitimately fails to take effect; only the log
        // decides the outcome, so a non-zero driver exit is not itself a verdict.
        Ok(Err(e)) => println!("webview-drive: driver reported {e} (RED arm — log decides)"),
        Err(_) => bail!("driver did not finish within {DRIVE_TIMEOUT:?}"),
    }

    let log_dir = data_dir.join("logs");
    let records: Vec<Value> = read_log_lines(&log_dir)
        .iter()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .collect();

    // Per test-plan §3 Direct-binary smoke variant the clean-log assertion binds
    // the GREEN leg only: the RED leg is by construction an unhealthy run, and
    // demanding a clean log over it would contradict what it measures.
    if !expect_absent {
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

    let observed = records.iter().any(is_widget_hidden_transition);
    report(observed, expect_absent)
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

async fn run_driver(
    script: &Path,
    binary: &Path,
    data_dir: &Path,
    msedgedriver: &Path,
) -> Result<()> {
    // CWD is the throwaway data dir, not the workspace: tauri-driver spawns the
    // app as a child of this process, and TauRPC's dev-mode `export_types()`
    // writes its bindings RELATIVE to the working directory — pointing anywhere
    // under the repo either clobbers the real bindings or strands a stray copy.
    // Node resolves its own modules from the script path, so CWD is free here.
    let status = Command::new("node")
        .arg(script)
        .current_dir(data_dir)
        .env("PULSE_BIN", binary)
        .env("PULSE_DATA_DIR", data_dir)
        .env("MSEDGEDRIVER_PATH", msedgedriver)
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .status()
        .await
        .context("spawn node driver (is node on PATH?)")?;

    if !status.success() {
        bail!("driver exited with {status}");
    }
    Ok(())
}

// The app emits `ui.layout.transition` at boot too (applying widget geometry,
// `layout_mode_to = "top-right"`), so matching the target alone would pass on a
// record the press did not cause. The close path is the one whose destination is
// `hidden` and whose origin is the widget.
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

fn report(observed: bool, expect_absent: bool) -> Result<ExitCode> {
    match (observed, expect_absent) {
        (true, false) => {
            println!(
                "webview-drive: PASS — pressing the titlebar close control drove \
                 {WIDGET_LABEL} → {HIDDEN_MODE}"
            );
            Ok(ExitCode::SUCCESS)
        }
        (false, true) => {
            println!(
                "webview-drive: PASS (RED arm) — no {WIDGET_LABEL} → {HIDDEN_MODE} transition, \
                 so the leg discriminates a dead affordance"
            );
            Ok(ExitCode::SUCCESS)
        }
        (false, false) => {
            eprintln!(
                "webview-drive: FAIL — the control was pressed but no {WIDGET_LABEL} → \
                 {HIDDEN_MODE} transition reached the log; the affordance is dead"
            );
            Ok(ExitCode::FAILURE)
        }
        (true, true) => {
            eprintln!(
                "webview-drive: FAIL (RED arm) — the transition fired anyway, so the leg does \
                 not discriminate and would pass against a dead affordance"
            );
            Ok(ExitCode::FAILURE)
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
    fn green_arm_fails_when_transition_absent() {
        let code = report(false, false).expect("report");
        assert_eq!(code, ExitCode::FAILURE);
    }

    #[test]
    fn red_arm_fails_when_transition_present() {
        let code = report(true, true).expect("report");
        assert_eq!(code, ExitCode::FAILURE);
    }

    #[test]
    fn both_arms_pass_on_their_expected_outcome() {
        assert_eq!(report(true, false).expect("green"), ExitCode::SUCCESS);
        assert_eq!(report(false, true).expect("red"), ExitCode::SUCCESS);
    }
}
