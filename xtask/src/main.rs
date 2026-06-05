// `capability_drift_tests` mod is followed by `test_a11y_placeholder` and
// `status_to_code` helpers; clippy::items_after_test_module flags this as a
// "restriction" lint, but reorganizing this file's helper layout for a single
// chunk's tests yields churn without correctness benefit. The tests are
// `#[cfg(test)]` and excluded from release builds; non-test items after them
// stay accessible to the rest of the file unchanged.
#![allow(clippy::items_after_test_module)]

mod bundle_format;
mod smoke;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde_json::Value;

use crate::bundle_format::BundleFormat;

#[derive(Parser)]
#[command(name = "xtask", about = "andromeda-pulse task runner")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    #[command(name = "harness:status")]
    HarnessStatus,
    #[command(
        name = "test",
        about = "cargo nextest run --workspace --profile ci --no-tests=pass"
    )]
    Test {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "test:coverage",
        about = "cargo llvm-cov nextest --workspace --lcov --no-tests=pass (writes lcov.info)"
    )]
    TestCoverage {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(name = "audit", about = "cargo audit (RustSec advisory DB)")]
    Audit,
    #[command(name = "deny-bans", about = "cargo deny check bans licenses sources")]
    DenyBans,
    #[command(
        name = "ci-gates",
        about = "obs SLO gates: zero-spans + zero-panic + heartbeat-gap (perf-budget deferred)"
    )]
    CiGates,
    #[command(
        name = "lint",
        about = "npm run lint (ESLint flat config in pulse-app/ui/)"
    )]
    Lint {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "typecheck",
        about = "npm run typecheck (tsc --noEmit in pulse-app/ui/) — gates webview against TauRPC-generated bindings"
    )]
    Typecheck {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "test:a11y",
        about = "npm run test:a11y --prefix pulse-app/ui — axe-core + Lighthouse + pa11y + colorjs.io + Playwright + aggregator + regression detector orchestrator chain"
    )]
    TestA11y {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "capability-drift",
        about = "diff TauRPC procedures (from pulse-app/ui/src/bindings/index.ts) vs arch §Occupied Resources expected procedure list"
    )]
    CapabilityDrift,
    #[command(
        name = "capability-widening-check",
        about = "Chunk #77 — static-analyze pulse-app/capabilities/*.json against NEVER-widen bans per security plan §API Anti-Patterns rows 6-7 (pulse:notification / pulse:tray / pulse:plugin-fs MUST stay outbound-emit-only / read-only)"
    )]
    CapabilityWideningCheck,
    #[command(
        name = "smoke",
        about = "install-launch-ingest-query smoke per bundle format (chunk #51)"
    )]
    Smoke {
        #[arg(long, value_name = "PATH")]
        bundle: PathBuf,
        #[arg(long, value_enum)]
        format: BundleFormat,
    },
    #[command(
        name = "perf:slo-load",
        about = "10k spans/sec sustained-load test + post-test metric.webgpu.frame_duration_ms p99 ≤33ms + metric.buffer.memory_bytes max ≤512MB gate"
    )]
    PerfSloLoad,
    #[command(
        name = "coverage-regression",
        about = "Chunk #55 — compare current lcov.info against base-branch baseline; fail on any line/branch/function regression > +0.0pp default (test-plan §10 + §11)"
    )]
    CoverageRegression {
        #[arg(long, value_name = "PATH")]
        current: PathBuf,
        #[arg(long, value_name = "PATH")]
        baseline: PathBuf,
    },
    #[command(
        name = "quarantine-tracking",
        about = "Chunk #55 — assert every #[ignore] in Rust source carries a GitHub issue URL in surrounding 5-line window (test-plan §11)"
    )]
    QuarantineTracking,
    #[command(
        name = "criterion-regression",
        about = "Chunk #56 — compare current target/criterion/<bench>/new/estimates.json mean.point_estimate against baseline; fail on +10% regression default (obs-plan §10 row 4)"
    )]
    CriterionRegression {
        #[arg(long, value_name = "PATH")]
        current: PathBuf,
        #[arg(long, value_name = "PATH")]
        baseline: PathBuf,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let result: Result<ExitCode> = match cli.command {
        Cmd::HarnessStatus => harness_status().await,
        Cmd::Test { extra } => run_cargo_nextest(extra).await,
        Cmd::TestCoverage { extra } => run_cargo_llvm_cov(extra).await,
        Cmd::Audit => run_cargo("audit", &[]).await,
        Cmd::DenyBans => run_cargo("deny", &["check", "bans", "licenses", "sources"]).await,
        Cmd::CiGates => run_ci_gates().await,
        Cmd::Lint { extra } => run_npm_script("lint", extra).await,
        Cmd::Typecheck { extra } => run_npm_script("typecheck", extra).await,
        Cmd::TestA11y { extra } => run_npm_script("test:a11y", extra).await,
        Cmd::CapabilityDrift => capability_drift().await,
        Cmd::CapabilityWideningCheck => capability_widening_check().await,
        Cmd::Smoke { bundle, format } => smoke::run_smoke(&bundle, format).await,
        Cmd::PerfSloLoad => run_perf_slo_load().await,
        Cmd::CoverageRegression { current, baseline } => {
            run_coverage_regression(&current, &baseline).await
        }
        Cmd::QuarantineTracking => run_quarantine_tracking().await,
        Cmd::CriterionRegression { current, baseline } => {
            run_criterion_regression(&current, &baseline).await
        }
    };
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("xtask error: {e:?}");
            ExitCode::FAILURE
        }
    }
}

async fn harness_status() -> Result<ExitCode> {
    // ui-bridge built without taurpc-runtime feature here — xtask is non-IPC
    // and Tauri runtime DLLs aren't available on Windows without WebView2.
    // current_health() returns the same envelope that the TauRPC resolver
    // would emit; full IPC roundtrip lands at the integration-test chunk
    // when actual subsystems (chunks #15-#18) exist to hit.
    let envelope = ui_bridge::health::current_health();

    let json = serde_json::to_string_pretty(&envelope)?;
    println!("{json}");

    let exit = match envelope.status {
        ui_bridge::health::HealthStatus::Ok => ExitCode::SUCCESS,
        ui_bridge::health::HealthStatus::Degraded => ExitCode::FAILURE,
    };
    Ok(exit)
}

async fn run_cargo(subcommand: &str, args: &[&str]) -> Result<ExitCode> {
    let mut cmd = tokio::process::Command::new("cargo");
    cmd.arg(subcommand).args(args);
    let status = cmd
        .status()
        .await
        .with_context(|| format!("failed to spawn `cargo {subcommand}`"))?;
    Ok(status_to_code(status))
}

async fn run_cargo_nextest(extra: Vec<String>) -> Result<ExitCode> {
    let mut cmd = tokio::process::Command::new("cargo");
    // libtest-json is gated behind an experimental flag in cargo-nextest 0.9.x;
    // set the env var unconditionally so the message-format parses agent-side.
    cmd.env("NEXTEST_EXPERIMENTAL_LIBTEST_JSON", "1");
    cmd.args([
        "nextest",
        "run",
        "--workspace",
        "--profile",
        "ci",
        "--no-tests=pass",
        "--message-format",
        "libtest-json",
    ]);
    for arg in extra {
        cmd.arg(arg);
    }
    let status = cmd
        .status()
        .await
        .context("failed to spawn `cargo nextest`")?;
    Ok(status_to_code(status))
}

async fn run_cargo_llvm_cov(extra: Vec<String>) -> Result<ExitCode> {
    let mut cmd = tokio::process::Command::new("cargo");
    cmd.args([
        "llvm-cov",
        "nextest",
        "--workspace",
        "--lcov",
        "--output-path",
        "lcov.info",
        "--no-tests=pass",
    ]);
    for arg in extra {
        cmd.arg(arg);
    }
    let status = cmd
        .status()
        .await
        .context("failed to spawn `cargo llvm-cov`")?;
    Ok(status_to_code(status))
}

async fn run_ci_gates() -> Result<ExitCode> {
    let log_files = collect_log_files();
    if log_files.is_empty() {
        // INACTIVE state: no harness boot in this CI run; gate trivially passes.
        // Activates organically when integration tests boot pulse-app (chunks #15+).
        println!(
            "ci-gates: zero-spans NEUTRAL (no `agent-latest.jsonl*` under {} — pre-integration-test state)",
            resolve_log_dir().display()
        );
        println!("ci-gates: zero-panic NEUTRAL (no log file to scan)");
        println!("ci-gates: heartbeat-gap NEUTRAL (no log file to scan)");
        println!("ci-gates: perf-budget DEFERRED (no criterion bench yet)");
        return Ok(ExitCode::SUCCESS);
    }

    let mut total_lines = 0usize;
    let mut panic_violation: Option<String> = None;
    for path in &log_files {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("ci-gates: skipping {} ({e})", path.display());
                continue;
            }
        };
        for (idx, raw) in content.lines().enumerate() {
            if raw.trim().is_empty() {
                continue;
            }
            total_lines += 1;
            let v: Value = match serde_json::from_str(raw) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let target = v.get("target").and_then(Value::as_str).unwrap_or("");
            let level = v.get("level").and_then(Value::as_str).unwrap_or("");
            if target == "app.panic.fatal" && level == "ERROR" && panic_violation.is_none() {
                let preview: String = raw.chars().take(240).collect();
                panic_violation = Some(format!("{}:{} — {}", path.display(), idx + 1, preview));
            }
        }
    }

    if total_lines == 0 {
        eprintln!("::error::ci-gates: zero-spans FAIL (log files present but contain no events)");
        return Ok(ExitCode::FAILURE);
    }
    println!(
        "ci-gates: zero-spans PASS ({total_lines} log records across {} file(s))",
        log_files.len()
    );

    if let Some(violation) = panic_violation {
        eprintln!("::error::ci-gates: zero-panic FAIL — app.panic.fatal at {violation}");
        return Ok(ExitCode::FAILURE);
    }
    println!("ci-gates: zero-panic PASS");

    // Heartbeat-gap gate: shell out to xtask/ci/heartbeat-gap-check.{sh,ps1}
    match invoke_heartbeat_check(&log_files).await {
        Ok(true) => println!("ci-gates: heartbeat-gap PASS"),
        Ok(false) => {
            eprintln!("::error::ci-gates: heartbeat-gap FAIL");
            return Ok(ExitCode::FAILURE);
        }
        Err(e) => {
            eprintln!(
                "ci-gates: heartbeat-gap script unavailable ({e:#}) — treating as NEUTRAL at chunk #5/#6"
            );
        }
    }

    // Perf-budget gate: shell out to xtask/ci/perf-slo-check.{sh,ps1} per
    // chunk #54 activation. The script tails `agent-latest.jsonl` for
    // `metric.webgpu.frame_duration_ms` events, computes p99 ≤33ms, and
    // checks `metric.buffer.memory_bytes` max ≤512MB. Missing script or
    // empty event stream maps к NEUTRAL (pre-perf-instrumentation states).
    match invoke_perf_slo_check(&log_files).await {
        Ok(true) => println!("ci-gates: perf-budget PASS"),
        Ok(false) => {
            eprintln!("::error::ci-gates: perf-budget FAIL");
            return Ok(ExitCode::FAILURE);
        }
        Err(e) => {
            eprintln!("ci-gates: perf-budget script unavailable ({e:#}) — treating as NEUTRAL");
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn collect_log_files() -> Vec<PathBuf> {
    let dir = resolve_log_dir();
    let entries = match fs::read_dir(&dir) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|res| res.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.starts_with("agent-latest.jsonl"))
                .unwrap_or(false)
        })
        .collect();
    files.sort();
    files
}

fn resolve_log_dir() -> PathBuf {
    if let Ok(p) = env::var("ANDROMEDA_PULSE_DATA_DIR") {
        return PathBuf::from(p).join("logs");
    }
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata).join("andromeda-pulse").join("logs");
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("com.andromeda.pulse")
                .join("logs");
        }
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("andromeda-pulse").join("logs");
    } else if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".andromeda-pulse").join("logs");
    }
    env::temp_dir().join("andromeda-pulse").join("logs")
}

async fn invoke_heartbeat_check(log_files: &[PathBuf]) -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("heartbeat-gap-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("heartbeat-gap-check.sh")
    };
    if !script.exists() {
        bail!("heartbeat-gap script missing at {}", script.display());
    }
    let primary_log = log_files
        .last()
        .context("no log file to pass to heartbeat-gap script")?;
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(primary_log)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(primary_log)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn invoke_perf_slo_check(log_files: &[PathBuf]) -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("perf-slo-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("perf-slo-check.sh")
    };
    if !script.exists() {
        bail!("perf-slo-check script missing at {}", script.display());
    }
    let primary_log = log_files
        .last()
        .context("no log file to pass to perf-slo-check script")?;
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(primary_log)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(primary_log)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_perf_slo_load() -> Result<ExitCode> {
    // 10k spans/sec sustained-load test runs the perf_slo_10k_spans
    // integration test via cargo-nextest; post-test p99 / max gates fire
    // via run_ci_gates() (invoked as а separate xtask step in CI).
    let mut cmd = tokio::process::Command::new("cargo");
    cmd.env("NEXTEST_EXPERIMENTAL_LIBTEST_JSON", "1");
    cmd.args([
        "nextest",
        "run",
        "-p",
        "pulse-app",
        "--test",
        "perf_slo_10k_spans",
        "--profile",
        "ci",
        "--no-tests=pass",
        "--message-format",
        "libtest-json",
    ]);
    let status = cmd
        .status()
        .await
        .context("failed to spawn `cargo nextest run --test perf_slo_10k_spans`")?;
    Ok(status_to_code(status))
}

async fn invoke_coverage_regression_check(current: &Path, baseline: &Path) -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("coverage-regression-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("coverage-regression-check.sh")
    };
    if !script.exists() {
        bail!(
            "coverage-regression-check script missing at {}",
            script.display()
        );
    }
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_coverage_regression(current: &Path, baseline: &Path) -> Result<ExitCode> {
    if invoke_coverage_regression_check(current, baseline).await? {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

async fn invoke_quarantine_tracking_check() -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("quarantine-tracking-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("quarantine-tracking-check.sh")
    };
    if !script.exists() {
        bail!(
            "quarantine-tracking-check script missing at {}",
            script.display()
        );
    }
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(&workspace_root)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(&workspace_root)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_quarantine_tracking() -> Result<ExitCode> {
    if invoke_quarantine_tracking_check().await? {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

async fn invoke_criterion_regression_check(current: &Path, baseline: &Path) -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("criterion-regression-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("criterion-regression-check.sh")
    };
    if !script.exists() {
        bail!(
            "criterion-regression-check script missing at {}",
            script.display()
        );
    }
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_criterion_regression(current: &Path, baseline: &Path) -> Result<ExitCode> {
    if invoke_criterion_regression_check(current, baseline).await? {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

async fn run_npm_script(script: &str, extra: Vec<String>) -> Result<ExitCode> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let cwd = workspace_root.join("pulse-app").join("ui");
    let npm = if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    };
    let mut cmd = tokio::process::Command::new(npm);
    cmd.current_dir(&cwd).arg("run").arg(script);
    if !extra.is_empty() {
        cmd.arg("--");
        for arg in extra {
            cmd.arg(arg);
        }
    }
    let status = cmd
        .status()
        .await
        .with_context(|| format!("failed to spawn `npm run {script}` in {}", cwd.display()))?;
    Ok(status_to_code(status))
}

// EXPECTED_PROCEDURES tracks the TauRPC procedure surface arch §Occupied
// Resources Tauri IPC routes legitimizes for the current commit. New chunks
// extend this list as their crates ship. Future-deferred procedures
// (other snapshot/workspace verbs, mcp.*) commented out until their owning
// chunks land — uncommenting prematurely produces "missing" drift noise
// that hides real drift. mcp.* additionally gated by --features mcp-server
// at the binary level.
const EXPECTED_PROCEDURES: &[&str] = &[
    "app_info",
    "health",
    "ready",
    "get_settings",
    "update_settings",
    "config.reload",
    "config.status",
    "connection.current_state",
    "diagnostics.history",
    "diagnostics.reevaluate_recent_window",
    "diagnostics.retry_interpretation",
    "diagnostics.snapshot",
    "diagnostics.template_distribution",
    "incidents.acknowledge",
    "incidents.get_report",
    "incidents.list_active",
    "incidents.mark_all_read",
    "incidents.mark_resolved",
    "logs.query",
    "metrics.query",
    "services.list_with_states",
    "storage.export_for_training",
    "storage.inspect",
    "storage.path",
    "streams.subscribe_logs",
    "streams.subscribe_metrics",
    "streams.subscribe_spans",
    "snapshot.generate",
    "plugins.list",
    "plugins.reload",
    "plugins.invoke",
    "mcp.status",
    "mcp.start",
    "mcp.stop",
    "model.current_profile",
    "telemetry.frontend.record_frame_ms",
    "traces.query",
    "workspace.detect",
    // future-deferred (per epoch landing):
    // "snapshot.list_recent", "snapshot.copy_to_clipboard",
    // "workspace.list",
];

async fn capability_drift() -> Result<ExitCode> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let bindings_path = workspace_root
        .join("pulse-app")
        .join("ui")
        .join("src")
        .join("bindings")
        .join("index.ts");
    let bindings_content = fs::read_to_string(&bindings_path)
        .with_context(|| format!("read bindings file at {}", bindings_path.display()))?;

    let discovered = parse_bindings(&bindings_content)
        .context("parse ARGS_MAP from bindings.ts (taurpc emission shape may have changed)")?;
    let expected: BTreeSet<String> = EXPECTED_PROCEDURES.iter().map(|s| s.to_string()).collect();

    let missing: BTreeSet<String> = expected.difference(&discovered).cloned().collect();
    let extra: BTreeSet<String> = discovered.difference(&expected).cloned().collect();

    let drift_state = if missing.is_empty() && extra.is_empty() {
        "clean"
    } else {
        "drifted"
    };

    let report_dir = workspace_root.join("target").join("capability-drift");
    fs::create_dir_all(&report_dir).context("create capability-drift report dir")?;
    let report_path = report_dir.join("report.json");

    let top_5_drifted: Vec<&String> = missing.iter().chain(extra.iter()).take(5).collect();
    let report = serde_json::json!({
        "drift_state": drift_state,
        "missing_count": missing.len(),
        "extra_count": extra.len(),
        "missing": missing.iter().collect::<Vec<_>>(),
        "extra": extra.iter().collect::<Vec<_>>(),
        "discovered": discovered.iter().collect::<Vec<_>>(),
        "expected": expected.iter().collect::<Vec<_>>(),
        "generated_at": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&report_path, serde_json::to_string_pretty(&report)?)
        .context("write drift report")?;

    // Structured event line to stdout matching the obs §3 JSON schema. xtask
    // does not init a tracing subscriber (per implementation notes); the JSON
    // line is the agent-tailable surface.
    let event = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "level": if drift_state == "clean" { "INFO" } else { "WARN" },
        "target": "xtask.capability_drift",
        "message": "capability drift check complete",
        "fields": {
            "missing_count": missing.len(),
            "extra_count": extra.len(),
            "drift_state": drift_state,
            "top_5_drifted_names": top_5_drifted,
        },
    });
    println!("{}", serde_json::to_string(&event)?);

    eprintln!(
        "capability-drift: {drift_state} ({} missing, {} extra)",
        missing.len(),
        extra.len()
    );
    if !missing.is_empty() {
        let names: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
        eprintln!("  missing: {names:?}");
    }
    if !extra.is_empty() {
        let names: Vec<&str> = extra.iter().map(|s| s.as_str()).collect();
        eprintln!("  extra: {names:?}");
    }
    eprintln!("  report: {}", report_path.display());

    if drift_state == "clean" {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

// Chunk #77 — capability-widening static analysis (per security plan §API
// Anti-Patterns rows 6-7 + security.md Session Addition 2026-05-08
// documented test gap).
//
// 3 capabilities have NEVER-widen invariants that the existing
// capability_drift check does NOT enforce (drift only verifies router↔JSON
// procedure sync, not permission widening within а capability's permissions
// array):
//   - pulse:notification — outbound-emit only (NEVER include
//     notification:allow-register-action-types OR notification:allow-register-listener
//     per pulse-app/capabilities/notification.json:4 description)
//   - pulse:tray — outbound emit only (NEVER include any input-event handler;
//     permissions ending in -register-* / -listen-* / -on-* indicate
//     input-event handlers banned per pulse-app/capabilities/tray.json:4
//     description)
//   - pulse:plugin-fs — backend-only (NEVER expose к webview JavaScript via
//     `windows: [...]` array population NOR add any fs:* / shell:* / dialog:*
//     / http:* permissions per pulse-app/capabilities/plugin-fs.json:4
//     description)
//
// On widening detection: report the specific (capability, permission,
// violated rule) tuple + exit non-zero. Mirrors capability_drift's structured
// report at target/capability-widening/report.json + structured JSON event
// line per obs-plan §3 schema.
async fn capability_widening_check() -> Result<ExitCode> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let capabilities_dir = workspace_root.join("pulse-app").join("capabilities");

    // Per-capability rules: (capability_name, banned_substrings_in_permissions,
    // require_empty_windows). Order matters for stable reporting.
    let rules: &[(&str, &[&str], bool)] = &[
        // pulse:notification — banned input-event handler permission substrings
        // per security.md Session Addition 2026-05-08 + notification.json:4
        // description.
        (
            "notification",
            &["allow-register-action-types", "allow-register-listener"],
            false,
        ),
        // pulse:tray — banned input-event handler patterns (any -register- /
        // -listen- / -on- suffix in permission name indicates webview→backend
        // event re-entry; outbound-emit only is the invariant per tray.json:4
        // description).
        ("tray", &["-register-", "-listen-", "-on-"], false),
        // pulse:plugin-fs — backend-only, no webview exposure (`windows: []`
        // MUST be empty); no permissions allowed beyond the empty default
        // (any fs:* / shell:* / dialog:* / http:* permission represents
        // widening per plugin-fs.json:4 description).
        ("plugin-fs", &["fs:", "shell:", "dialog:", "http:"], true),
    ];

    let mut violations: Vec<String> = Vec::new();
    let mut inspected: Vec<String> = Vec::new();

    for (cap_name, banned_substrings, require_empty_windows) in rules {
        let path = capabilities_dir.join(format!("{cap_name}.json"));
        inspected.push(format!("{cap_name}.json"));
        let content = fs::read_to_string(&path)
            .with_context(|| format!("read capability JSON at {}", path.display()))?;
        let json: Value = serde_json::from_str(&content)
            .with_context(|| format!("parse capability JSON at {}", path.display()))?;

        // Check permissions array against banned substrings.
        if let Some(permissions) = json.get("permissions").and_then(|v| v.as_array()) {
            for perm in permissions {
                if let Some(perm_str) = perm.as_str() {
                    for banned in *banned_substrings {
                        if perm_str.contains(banned) {
                            violations.push(format!(
                                "pulse:{cap_name} permission `{perm_str}` matches banned substring `{banned}` (widening NEVER-allow per security plan §API Anti-Patterns rows 6-7 + {cap_name}.json description)"
                            ));
                        }
                    }
                }
            }
        }

        // Check windows array IF require_empty_windows.
        if *require_empty_windows {
            if let Some(windows) = json.get("windows").and_then(|v| v.as_array()) {
                if !windows.is_empty() {
                    let window_names: Vec<String> = windows
                        .iter()
                        .filter_map(|w| w.as_str().map(|s| s.to_string()))
                        .collect();
                    violations.push(format!(
                        "pulse:{cap_name} `windows` array MUST be empty (backend-only; no webview exposure per {cap_name}.json description); found {window_names:?}"
                    ));
                }
            }
        }
    }

    let widening_state = if violations.is_empty() {
        "clean"
    } else {
        "violations"
    };

    let report_dir = workspace_root.join("target").join("capability-widening");
    fs::create_dir_all(&report_dir).context("create capability-widening report dir")?;
    let report_path = report_dir.join("report.json");

    let report = serde_json::json!({
        "widening_state": widening_state,
        "violations_count": violations.len(),
        "violations": violations,
        "inspected": inspected,
        "generated_at": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&report_path, serde_json::to_string_pretty(&report)?)
        .context("write widening report")?;

    let event = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "level": if widening_state == "clean" { "INFO" } else { "WARN" },
        "target": "xtask.capability_widening_check",
        "message": "capability widening check complete",
        "fields": {
            "violations_count": violations.len(),
            "widening_state": widening_state,
            "inspected_count": inspected.len(),
        },
    });
    println!("{}", serde_json::to_string(&event)?);

    eprintln!(
        "capability-widening-check: {widening_state} ({} violations across {} inspected)",
        violations.len(),
        inspected.len()
    );
    for v in &violations {
        eprintln!("  ✗ {v}");
    }
    eprintln!("  report: {}", report_path.display());

    if widening_state == "clean" {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

// Parse the ARGS_MAP line from a taurpc-emitted bindings.ts file. Format:
//   const ARGS_MAP = { 'health':'{"check":[]}', 'streams':'{"subscribe_spans":[...]}' }
// Outer is a JS object literal: single-quoted keys → single-quoted values
// where each value is a stringified JSON method-map. Inner JSON uses double
// quotes; outer single quotes delimit. Procedure identifiers are restricted
// to [A-Za-z0-9_] so single quotes only ever delimit, never appear inside
// identifiers. Returns the flat (router.method or method) procedure set.
pub(crate) fn parse_bindings(bindings_content: &str) -> Result<BTreeSet<String>> {
    let args_map_line = bindings_content
        .lines()
        .map(str::trim_start)
        .find_map(|line| line.strip_prefix("const ARGS_MAP = "))
        .context("ARGS_MAP line not found in bindings.ts")?;
    let rhs = args_map_line.trim_end_matches(';').trim();
    let inner = rhs.trim_start_matches('{').trim_end_matches('}').trim();

    let bytes = inner.as_bytes();
    let mut idx = 0usize;
    let mut discovered = BTreeSet::new();

    while idx < bytes.len() {
        while idx < bytes.len() && (bytes[idx].is_ascii_whitespace() || bytes[idx] == b',') {
            idx += 1;
        }
        if idx >= bytes.len() {
            break;
        }
        if bytes[idx] != b'\'' {
            bail!("expected `'` at offset {idx} in ARGS_MAP body");
        }
        idx += 1;
        let router_start = idx;
        while idx < bytes.len() && bytes[idx] != b'\'' {
            idx += 1;
        }
        if idx >= bytes.len() {
            bail!("unterminated router key");
        }
        let router = std::str::from_utf8(&bytes[router_start..idx])
            .context("router key not utf-8")?
            .to_string();
        idx += 1; // consume closing `'`

        while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx >= bytes.len() || bytes[idx] != b':' {
            bail!("expected `:` after router key `{router}`");
        }
        idx += 1;
        while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx >= bytes.len() || bytes[idx] != b'\'' {
            bail!("expected `'` for methods_json of router `{router}`");
        }
        idx += 1;
        let methods_start = idx;
        while idx < bytes.len() && bytes[idx] != b'\'' {
            idx += 1;
        }
        if idx >= bytes.len() {
            bail!("unterminated methods_json for router `{router}`");
        }
        let methods_json =
            std::str::from_utf8(&bytes[methods_start..idx]).context("methods_json not utf-8")?;
        idx += 1; // consume closing `'`

        let methods: BTreeMap<String, Vec<String>> = serde_json::from_str(methods_json)
            .with_context(|| format!("parse methods_json for `{router}`: {methods_json}"))?;
        for method_name in methods.keys() {
            if router.is_empty() {
                discovered.insert(method_name.clone());
            } else {
                discovered.insert(format!("{router}.{method_name}"));
            }
        }
    }
    Ok(discovered)
}

#[cfg(test)]
mod capability_drift_tests {
    use super::*;

    #[test]
    fn parse_bindings_extracts_namespaced_procedures() {
        let bindings = "// header\nconst ARGS_MAP = { 'health':'{\"check\":[]}', 'traces':'{\"query\":[\"args\"]}' }\nexport type Foo = ...";
        let discovered = parse_bindings(bindings).expect("parses");
        assert!(discovered.contains("health.check"));
        assert!(discovered.contains("traces.query"));
        assert_eq!(discovered.len(), 2);
    }

    #[test]
    fn parse_bindings_handles_streams_router() {
        let bindings = "const ARGS_MAP = { 'streams':'{\"subscribe_logs\":[\"channel\"],\"subscribe_metrics\":[\"channel\"],\"subscribe_spans\":[\"channel\"]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        assert!(discovered.contains("streams.subscribe_logs"));
        assert!(discovered.contains("streams.subscribe_metrics"));
        assert!(discovered.contains("streams.subscribe_spans"));
        assert_eq!(discovered.len(), 3);
    }

    #[test]
    fn parse_bindings_handles_top_level_procedures_via_empty_router() {
        // Hypothetical chunk #27 emission shape: top-level procedures may emit
        // under an empty router string. Verify the parser handles this.
        let bindings = "const ARGS_MAP = { '':'{\"app_info\":[],\"health\":[]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        assert!(discovered.contains("app_info"));
        assert!(discovered.contains("health"));
        assert_eq!(discovered.len(), 2);
    }

    #[test]
    fn parse_bindings_missing_args_map_returns_err() {
        let bindings = "// only header here, no ARGS_MAP\nexport type Foo = ...";
        let result = parse_bindings(bindings);
        assert!(result.is_err());
    }

    #[test]
    fn expected_procedures_includes_chunk_27_introspection_envelope() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "app_info",
            "health",
            "ready",
            "get_settings",
            "update_settings",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc}"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_plugins_namespace_at_chunk_47() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in ["plugins.list", "plugins.reload", "plugins.invoke"] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #47 plugins router)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_mcp_namespace_at_chunk_49() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in ["mcp.status", "mcp.start", "mcp.stop"] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #49 mcp router)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_connection_namespace_at_chunk_59() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("connection.current_state"),
            "EXPECTED_PROCEDURES must include connection.current_state (chunk #59 connection router)"
        );
    }

    #[test]
    fn expected_procedures_includes_services_list_with_states_at_chunk_67() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("services.list_with_states"),
            "EXPECTED_PROCEDURES must include services.list_with_states (chunk #67 service registry + lifecycle state machine)"
        );
    }

    #[test]
    fn expected_procedures_includes_storage_namespace_at_chunk_68() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "storage.inspect",
            "storage.path",
            "storage.export_for_training",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #68 corpus SQLite scaffold + storage router; storage.export_for_training added chunk #95)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_diagnostics_namespace_at_chunk_69() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("diagnostics.template_distribution"),
            "EXPECTED_PROCEDURES must include diagnostics.template_distribution (chunk #69 Phase B Session 3 Drain template profiling diagnostics)"
        );
    }

    #[test]
    fn expected_procedures_includes_incidents_namespace_at_chunk_78() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "incidents.list_active",
            "incidents.acknowledge",
            "incidents.mark_resolved",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #78 incident records + lifecycle persistence)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_model_namespace_at_chunk_82() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("model.current_profile"),
            "EXPECTED_PROCEDURES must include model.current_profile (chunk #82 hardware profile detection + model loading + tokenizer)"
        );
    }

    #[test]
    fn expected_procedures_includes_diagnostics_retry_interpretation_at_chunk_86() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("diagnostics.retry_interpretation"),
            "EXPECTED_PROCEDURES must include diagnostics.retry_interpretation (chunk #86 JSON parse failure handling + backoff + resolution summary — Settings → Diagnostics manual override per capability P-020 graceful degradation reaching full)"
        );
    }

    #[test]
    fn expected_procedures_includes_mark_all_read_at_chunk_87() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("incidents.mark_all_read"),
            "EXPECTED_PROCEDURES must include incidents.mark_all_read (chunk #87 Findings counter + dropdown — bulk-mark-as-read action per capabilities P-028 / P-029 / P-030)"
        );
    }

    #[test]
    fn expected_procedures_includes_get_report_at_chunk_88() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("incidents.get_report"),
            "EXPECTED_PROCEDURES must include incidents.get_report (chunk #88 Diagnostic Report generation — L5 in-app report panel + copy markdown per capabilities P-031 + P-035–P-038)"
        );
    }

    #[test]
    fn expected_procedures_includes_config_hot_reload_at_chunk_96() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "config.reload",
            "config.status",
            "diagnostics.reevaluate_recent_window",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #96 Configuration hot reload + prospective threshold application per capabilities P-055 / P-056)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_diagnostics_snapshot_and_history_at_chunk_97() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in ["diagnostics.snapshot", "diagnostics.history"] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #97 Settings → Diagnostics view — L6 self-observability surface per capability P-058)"
            );
        }
    }

    #[test]
    fn drift_detected_when_extra_procedure_present() {
        // Synthetic ARGS_MAP: introduces `streams.subscribe_spans` not in the
        // expected list (D3 carry-over scenario).
        let bindings = "const ARGS_MAP = { 'health':'{\"check\":[]}', 'streams':'{\"subscribe_spans\":[\"channel\"]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        let expected: BTreeSet<String> = ["health.check"].iter().map(|s| s.to_string()).collect();
        let extra: BTreeSet<String> = discovered.difference(&expected).cloned().collect();
        assert!(extra.contains("streams.subscribe_spans"));
    }

    #[test]
    fn drift_clean_when_sets_match() {
        let bindings = "const ARGS_MAP = { 'health':'{\"check\":[]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        let expected: BTreeSet<String> = ["health.check"].iter().map(|s| s.to_string()).collect();
        let missing: BTreeSet<String> = expected.difference(&discovered).cloned().collect();
        let extra: BTreeSet<String> = discovered.difference(&expected).cloned().collect();
        assert!(missing.is_empty());
        assert!(extra.is_empty());
    }
}

fn status_to_code(status: std::process::ExitStatus) -> ExitCode {
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
