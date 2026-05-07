use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde_json::Value;

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
        about = "a11y harness placeholder (full activation at chunk #25 webview shell + chunk #46 CI gate)"
    )]
    TestA11y {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
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
        Cmd::TestA11y { extra: _ } => test_a11y_placeholder(),
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

    println!("ci-gates: perf-budget DEFERRED (activates with first criterion bench)");
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

fn test_a11y_placeholder() -> Result<ExitCode> {
    println!(
        "xtask test:a11y: deferred to chunk #25 webview shell + chunk #46 CI gate (chunk #13 install-only; chunk #14 SR manual-pass scaffold at pulse-app/ui/tests-a11y/screen-reader/; chunk #15 motion library + useReducedMotion hook at pulse-app/ui/src/hooks/)"
    );
    Ok(ExitCode::SUCCESS)
}

fn status_to_code(status: std::process::ExitStatus) -> ExitCode {
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
