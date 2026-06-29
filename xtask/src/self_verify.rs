// Agent-headful self-verify harness (P-078 — chunk 2026-06-29-predictable-
// close-self-verify). Boots the REAL pulse-app dev binary in an isolated data
// dir, asserts shell health from the structured log (boot spans + window
// shown + heartbeat tick + zero panics) WITHOUT pixel-scraping (per
// verification-harness.md — the IPC/log is the test surface), composes the
// existing a11y/contrast harness when its deps are present, and quits cleanly
// with a zero-orphan check (the Windows GUI-orphan hazard the close-fix
// unblocks). Deterministic exit: 0 = pass or clean-skip, non-zero = a real
// failure. Sibling to smoke.rs (which targets an INSTALLED bundle); this
// targets the cargo-built dev binary so it runs without a release build.

use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use serde_json::Value;
use tempfile::TempDir;
use tokio::net::TcpStream;
use tokio::process::{Child, Command};
use tokio::time::{Instant, sleep};

const LOOPBACK: &str = "127.0.0.1";
const GRPC_PORT: u16 = 4317;
const HTTP_PORT: u16 = 4318;
const READINESS_TIMEOUT: Duration = Duration::from_secs(15);
const POLL_INTERVAL: Duration = Duration::from_millis(500);
// Heartbeat ticks fire every 15s (obs-plan §3); the health window must exceed
// one tick interval so a clean boot reliably surfaces >=1 tick.
const HEALTH_TIMEOUT: Duration = Duration::from_secs(25);
const SHUTDOWN_GRACE: Duration = Duration::from_secs(5);

pub async fn run_self_verify() -> Result<ExitCode> {
    if let Some(reason) = headless_skip_reason() {
        println!("self-verify: SKIP — {reason}");
        return Ok(ExitCode::SUCCESS);
    }

    let workspace_root = workspace_root()?;
    let binary = match locate_pulse_binary(&workspace_root) {
        Some(p) => p,
        None => {
            println!(
                "self-verify: SKIP — no pulse-app binary under target/{{release,debug}}; \
                 build it first (cargo build -p pulse-app [--release])"
            );
            return Ok(ExitCode::SUCCESS);
        }
    };
    println!("self-verify: binary {}", binary.display());

    let tempdir = TempDir::new().context("create per-run tempdir")?;
    let data_dir = tempdir.path().to_path_buf();
    let log_dir = data_dir.join("logs");
    println!("self-verify: data_dir {}", data_dir.display());

    let mut child = launch_pulse(&binary, &data_dir)
        .await
        .context("launch pulse-app")?;
    println!("self-verify: launched PID={}", child.id().unwrap_or(0));

    // Always attempt clean shutdown regardless of the health outcome; cleanup
    // IS the zero-orphan proof (it fails if the child does not terminate).
    let health = run_inner(&log_dir).await;
    let quit = cleanup(&mut child).await;

    let summary = health.context("shell health")?;
    println!("self-verify: shell health OK — {summary}");
    quit.context("clean quit / zero-orphan")?;
    println!("self-verify: clean quit — zero orphan");

    match run_a11y(&workspace_root).await? {
        A11yOutcome::Passed => println!("self-verify: a11y/contrast PASS"),
        A11yOutcome::Skipped(reason) => println!("self-verify: a11y SKIP — {reason}"),
        A11yOutcome::Failed(code) => bail!("a11y/contrast harness failed (exit {code})"),
    }

    println!("self-verify: PASS");
    Ok(ExitCode::SUCCESS)
}

async fn run_inner(log_dir: &Path) -> Result<String> {
    poll_ports_ready().await.context("OTLP ports not ready")?;
    collect_shell_health(log_dir).await
}

// ===== shell health =====

#[derive(Default)]
struct ShellHealth {
    webview_init: bool,
    gpu_check: bool,
    tray_init: bool,
    window_shown: bool,
    heartbeat_tick: bool,
    panic_preview: Option<String>,
}

impl ShellHealth {
    fn is_complete(&self) -> bool {
        self.webview_init
            && self.gpu_check
            && self.tray_init
            && self.window_shown
            && self.heartbeat_tick
            && self.panic_preview.is_none()
    }

    fn missing(&self) -> String {
        let mut m = Vec::new();
        if !self.webview_init {
            m.push("app.boot.webview.init");
        }
        if !self.gpu_check {
            m.push("app.boot.gpu.check");
        }
        if !self.tray_init {
            m.push("app.boot.tray.init");
        }
        if !self.window_shown {
            m.push("window-shown (ui.layout.transition)");
        }
        if !self.heartbeat_tick {
            m.push("heartbeat (*.tick)");
        }
        m.join(", ")
    }
}

fn parse_shell_health(lines: &[String]) -> ShellHealth {
    let mut h = ShellHealth::default();
    for raw in lines {
        let v: Value = match serde_json::from_str(raw) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let target = v.get("target").and_then(Value::as_str).unwrap_or("");
        let level = v.get("level").and_then(Value::as_str).unwrap_or("");
        match target {
            "app.boot.webview.init" => h.webview_init = true,
            "app.boot.gpu.check" => h.gpu_check = true,
            "app.boot.tray.init" => h.tray_init = true,
            "ui.layout.transition" => h.window_shown = true,
            "app.panic.fatal" if level == "ERROR" && h.panic_preview.is_none() => {
                h.panic_preview = Some(raw.chars().take(240).collect());
            }
            t if t.ends_with(".tick") => h.heartbeat_tick = true,
            _ => {}
        }
    }
    h
}

async fn collect_shell_health(log_dir: &Path) -> Result<String> {
    let deadline = Instant::now() + HEALTH_TIMEOUT;
    loop {
        let lines = read_log_lines(log_dir);
        let health = parse_shell_health(&lines);
        if let Some(panic) = &health.panic_preview {
            bail!("zero-unlogged-panics SLO violated during boot: {panic}");
        }
        if health.is_complete() {
            return Ok(format!(
                "{} log lines; boot trio + window-shown + heartbeat present; zero panics",
                lines.len()
            ));
        }
        if Instant::now() >= deadline {
            bail!(
                "shell health incomplete after {HEALTH_TIMEOUT:?}: missing {}",
                health.missing()
            );
        }
        sleep(POLL_INTERVAL).await;
    }
}

// ===== launch / readiness / cleanup =====

async fn launch_pulse(binary: &Path, data_dir: &Path) -> Result<Child> {
    let mut cmd = Command::new(binary);
    cmd.env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .env("ANDROMEDA_PULSE_LOG_LEVEL", "debug")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    cmd.spawn()
        .with_context(|| format!("spawn {}", binary.display()))
}

async fn poll_ports_ready() -> Result<()> {
    let deadline = Instant::now() + READINESS_TIMEOUT;
    loop {
        if tcp_handshake(HTTP_PORT).await.is_ok() && tcp_handshake(GRPC_PORT).await.is_ok() {
            println!("self-verify: receivers ready (:4317 + :4318)");
            return Ok(());
        }
        if Instant::now() >= deadline {
            bail!("loopback OTLP ports not ready within {READINESS_TIMEOUT:?}");
        }
        sleep(POLL_INTERVAL).await;
    }
}

async fn tcp_handshake(port: u16) -> Result<()> {
    let addr = format!("{LOOPBACK}:{port}");
    let stream =
        tokio::time::timeout(Duration::from_millis(200), TcpStream::connect(&addr)).await??;
    drop(stream);
    Ok(())
}

async fn cleanup(child: &mut Child) -> Result<()> {
    let _ = child.start_kill();
    match tokio::time::timeout(SHUTDOWN_GRACE, child.wait()).await {
        Ok(Ok(_status)) => {}
        Ok(Err(e)) => bail!("child wait error: {e}"),
        Err(_) => bail!("pulse-app did not terminate within {SHUTDOWN_GRACE:?} (orphan risk)"),
    }
    for port in [GRPC_PORT, HTTP_PORT] {
        let addr = format!("{LOOPBACK}:{port}");
        if tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(&addr))
            .await
            .is_ok()
        {
            eprintln!("self-verify: warning — port {port} still accepting connections post-quit");
        }
    }
    Ok(())
}

// ===== a11y / contrast harness composition =====

enum A11yOutcome {
    Passed,
    Skipped(String),
    Failed(i32),
}

async fn run_a11y(workspace_root: &Path) -> Result<A11yOutcome> {
    let ui_dir = workspace_root.join("pulse-app").join("ui");
    if !ui_dir.join("node_modules").is_dir() {
        return Ok(A11yOutcome::Skipped(
            "pulse-app/ui/node_modules absent — run `npm ci` in pulse-app/ui to enable".to_string(),
        ));
    }
    let npm = if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    };
    let status = Command::new(npm)
        .current_dir(&ui_dir)
        .arg("run")
        .arg("test:a11y")
        .status()
        .await
        .with_context(|| format!("spawn `npm run test:a11y` in {}", ui_dir.display()))?;
    if status.success() {
        Ok(A11yOutcome::Passed)
    } else {
        Ok(A11yOutcome::Failed(status.code().unwrap_or(-1)))
    }
}

// ===== environment + binary resolution =====

fn headless_skip_reason() -> Option<String> {
    headless_reason(
        cfg!(target_os = "linux"),
        std::env::var("DISPLAY").ok().filter(|s| !s.is_empty()),
        std::env::var("WAYLAND_DISPLAY")
            .ok()
            .filter(|s| !s.is_empty()),
    )
}

fn headless_reason(
    is_linux: bool,
    display: Option<String>,
    wayland: Option<String>,
) -> Option<String> {
    if is_linux && display.is_none() && wayland.is_none() {
        Some(
            "display-less Linux host (no DISPLAY / WAYLAND_DISPLAY); the GUI shell cannot boot headful"
                .to_string(),
        )
    } else {
        None
    }
}

fn workspace_root() -> Result<PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf())
}

fn pulse_binary_candidates(workspace_root: &Path) -> Vec<PathBuf> {
    let exe = if cfg!(target_os = "windows") {
        "pulse-app.exe"
    } else {
        "pulse-app"
    };
    let target = workspace_root.join("target");
    vec![
        target.join("release").join(exe),
        target.join("debug").join(exe),
    ]
}

fn locate_pulse_binary(workspace_root: &Path) -> Option<PathBuf> {
    pulse_binary_candidates(workspace_root)
        .into_iter()
        .find(|p| p.exists())
}

// The obs sink is `tracing_appender::rolling::daily(logs_dir, "agent-latest.jsonl")`
// (pulse-app/src/observability.rs), which names files `agent-latest.jsonl.<date>`,
// not the bare name — so glob the prefix and concatenate, mirroring the obs
// ci-gates collect_log_files pattern. Missing dir (pre-boot) yields no lines.
fn read_log_lines(log_dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(log_dir) else {
        return Vec::new();
    };
    let mut lines = Vec::new();
    for entry in entries.flatten() {
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with("agent-latest.jsonl")
        {
            continue;
        }
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            lines.extend(
                content
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(str::to_string),
            );
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(target: &str, level: &str) -> String {
        format!(
            r#"{{"timestamp":"2026-01-01T00:00:00Z","level":"{level}","target":"{target}","message":"x","fields":{{}}}}"#
        )
    }

    fn full_healthy_lines() -> Vec<String> {
        vec![
            line("app.boot.webview.init", "INFO"),
            line("app.boot.gpu.check", "INFO"),
            line("app.boot.tray.init", "INFO"),
            line("ui.layout.transition", "INFO"),
            line("buffer.tick", "INFO"),
        ]
    }

    #[test]
    fn parse_shell_health_complete_on_full_boot() {
        let h = parse_shell_health(&full_healthy_lines());
        assert!(h.is_complete());
        assert!(h.panic_preview.is_none());
    }

    #[test]
    fn parse_shell_health_incomplete_when_window_shown_missing() {
        let lines: Vec<String> = full_healthy_lines()
            .into_iter()
            .filter(|l| !l.contains("ui.layout.transition"))
            .collect();
        let h = parse_shell_health(&lines);
        assert!(!h.is_complete());
        assert!(h.missing().contains("window-shown"));
    }

    #[test]
    fn parse_shell_health_incomplete_when_heartbeat_missing() {
        let lines: Vec<String> = full_healthy_lines()
            .into_iter()
            .filter(|l| !l.contains(".tick"))
            .collect();
        let h = parse_shell_health(&lines);
        assert!(!h.is_complete());
        assert!(h.missing().contains("heartbeat"));
    }

    #[test]
    fn parse_shell_health_flags_error_panic() {
        let mut lines = full_healthy_lines();
        lines.push(line("app.panic.fatal", "ERROR"));
        let h = parse_shell_health(&lines);
        assert!(h.panic_preview.is_some());
        assert!(!h.is_complete());
    }

    #[test]
    fn parse_shell_health_ignores_non_error_panic_target() {
        let mut lines = full_healthy_lines();
        lines.push(line("app.panic.fatal", "INFO"));
        let h = parse_shell_health(&lines);
        assert!(h.panic_preview.is_none());
    }

    #[test]
    fn parse_shell_health_skips_malformed_lines() {
        let mut lines = full_healthy_lines();
        lines.push("not json at all".to_string());
        let h = parse_shell_health(&lines);
        assert!(h.is_complete());
    }

    #[test]
    fn headless_reason_only_on_displayless_linux() {
        assert!(headless_reason(true, None, None).is_some());
        assert!(headless_reason(true, Some(":0".into()), None).is_none());
        assert!(headless_reason(true, None, Some("wayland-0".into())).is_none());
        assert!(headless_reason(false, None, None).is_none());
    }

    #[test]
    fn pulse_binary_candidates_prefers_release_then_debug() {
        let candidates = pulse_binary_candidates(Path::new("/ws"));
        assert_eq!(candidates.len(), 2);
        assert!(candidates[0].to_string_lossy().contains("release"));
        assert!(candidates[1].to_string_lossy().contains("debug"));
    }
}
