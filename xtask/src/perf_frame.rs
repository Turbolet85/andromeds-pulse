//! `cargo xtask perf:frame-sample` — the frame budget gate, read from the
//! release binary on a Windows GPU dev host.
//!
//! WebView2 is handed a software WebGPU adapter (SwiftShader over Vulkan). The
//! flag set rides the CHILD's environment only: product config never carries
//! it, because it would change the shipped webview's GPU posture. On the dev
//! host this set was the only one of three that exposed an adapter (0 / 0 / 496
//! frame samples). A hosted Windows runner under the same set produced 0
//! (ci#36723465727, app healthy), so the leg is not CI-wired.
//!
//! Exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE.

use std::ffi::OsString;
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde_json::Value;
use tokio::process::{Child, Command};

use crate::perf_budget::{self, Arm, ArmState};

pub const WEBVIEW2_ARGS_VAR: &str = "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS";
pub const SOFTWARE_ADAPTER_ARGS: &str = "--enable-unsafe-webgpu --enable-features=Vulkan \
     --use-vulkan=swiftshader --use-webgpu-adapter=swiftshader";

const OTLP_PORTS: [u16; 2] = [4317, 4318];
const HEALTHY_WAIT: Duration = Duration::from_secs(60);
const FIRST_FRAME_WAIT: Duration = Duration::from_secs(60);
const FEED_DURATION: Duration = Duration::from_secs(30);
const PORT_RELEASE_WAIT: Duration = Duration::from_secs(15);
const POLL: Duration = Duration::from_millis(500);

/// The leg reads the Windows WebView2 runtime and nothing else.
pub fn platform_guard(os: &str) -> Result<(), String> {
    if os == "windows" {
        Ok(())
    } else {
        Err(format!(
            "perf:frame-sample drives WebView2 and runs on Windows only (this host: {os})"
        ))
    }
}

/// The environment the app child is spawned with, beside the inherited one.
pub fn child_env(data_dir: &Path) -> Vec<(OsString, OsString)> {
    vec![
        (
            OsString::from("ANDROMEDA_PULSE_DATA_DIR"),
            data_dir.as_os_str().to_os_string(),
        ),
        (
            OsString::from(WEBVIEW2_ARGS_VAR),
            OsString::from(SOFTWARE_ADAPTER_ARGS),
        ),
    ]
}

fn exe(name: &str) -> String {
    if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}

fn port_accepting(port: u16) -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    TcpStream::connect_timeout(&addr, Duration::from_millis(300)).is_ok()
}

fn read_lines(data_dir: &Path) -> Vec<Value> {
    perf_budget::read_family(&data_dir.join("logs")).unwrap_or_default()
}

fn has_frame(lines: &[Value]) -> bool {
    lines
        .iter()
        .any(|l| l.get("target").and_then(Value::as_str) == Some("metric.webgpu.frame_duration_ms"))
}

fn inconclusive(why: &str) -> Result<ExitCode> {
    println!("perf:frame-sample: INCONCLUSIVE — {why}");
    Ok(ExitCode::from(2))
}

async fn stop_by_pidfile(data_dir: &Path, app: &mut Child) {
    let pid = std::fs::read_to_string(data_dir.join("run").join("andromeda-pulse.pid"))
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok());
    if let Some(pid) = pid.filter(|p| Some(*p) != app.id()) {
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!("Stop-Process -Id {pid} -Force -ErrorAction SilentlyContinue"),
            ])
            .stdin(Stdio::null())
            .status()
            .await;
    }
    let _ = app.kill().await;
}

async fn ports_released() -> bool {
    let deadline = Instant::now() + PORT_RELEASE_WAIT;
    loop {
        if OTLP_PORTS.iter().all(|p| !port_accepting(*p)) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(POLL).await;
    }
}

fn preserve_logs(workspace_root: &Path, data_dir: &Path, stamp: &str) -> Result<PathBuf> {
    let out = workspace_root.join("target").join("perf-frame").join(stamp);
    std::fs::create_dir_all(&out).context("create perf-frame artifact dir")?;
    for src in perf_budget::family_members(&data_dir.join("logs")) {
        if let Some(name) = src.file_name() {
            std::fs::copy(&src, out.join(name)).context("copy log family")?;
        }
    }
    Ok(out)
}

pub async fn run_perf_frame_sample() -> Result<ExitCode> {
    if let Err(why) = platform_guard(std::env::consts::OS) {
        return inconclusive(&why);
    }
    let stamp = chrono::Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let root = crate::self_verify::workspace_root()?;
    let app_bin = root.join("target").join("release").join(exe("pulse-app"));
    let injector = root
        .join("target")
        .join("release")
        .join("examples")
        .join(exe("inject_demo"));
    for bin in [&app_bin, &injector] {
        if !bin.is_file() {
            return inconclusive(&format!("{} is not built", bin.display()));
        }
    }
    if let Some(port) = OTLP_PORTS.into_iter().find(|p| port_accepting(*p)) {
        return inconclusive(&format!(
            ":{port} is already accepting — inject_demo targets 127.0.0.1:4317, so another app \
             holds the ports"
        ));
    }

    let data_dir = tempfile::TempDir::new().context("create frame-sample data dir")?;
    println!(
        "perf:frame-sample: app={} ({}) data dir {}",
        app_bin.display(),
        crate::external_resolve::binary_age(&app_bin),
        data_dir.path().display()
    );
    let mut app = Command::new(&app_bin)
        .current_dir(data_dir.path())
        .envs(child_env(data_dir.path()))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .context("spawn pulse-app")?;

    let deadline = Instant::now() + HEALTHY_WAIT;
    let mut healthy = false;
    while Instant::now() < deadline {
        if port_accepting(4317) {
            healthy = true;
            break;
        }
        tokio::time::sleep(POLL).await;
    }

    let mut frames_seen = false;
    if healthy {
        let deadline = Instant::now() + FIRST_FRAME_WAIT;
        while Instant::now() < deadline {
            if has_frame(&read_lines(data_dir.path())) {
                frames_seen = true;
                break;
            }
            tokio::time::sleep(POLL).await;
        }
        if frames_seen {
            println!(
                "perf:frame-sample: first frame sample seen — feeding inject_demo --sustained for \
                 {}s",
                FEED_DURATION.as_secs()
            );
            let mut feed = Command::new(&injector)
                .arg("--sustained")
                .current_dir(data_dir.path())
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .kill_on_drop(true)
                .spawn()
                .context("spawn inject_demo")?;
            let feed_end = Instant::now() + FEED_DURATION;
            while Instant::now() < feed_end {
                if let Ok(Some(status)) = feed.try_wait() {
                    println!("perf:frame-sample: inject_demo ended early ({status})");
                    break;
                }
                tokio::time::sleep(POLL).await;
            }
            let _ = feed.kill().await;
        }
    }

    stop_by_pidfile(data_dir.path(), &mut app).await;
    if ports_released().await {
        println!("perf:frame-sample: app stopped, :4317/:4318 released");
    } else {
        println!("perf:frame-sample: warning — :4317/:4318 still accepting after teardown");
    }

    let artifact = preserve_logs(&root, data_dir.path(), &stamp)?;
    println!(
        "perf:frame-sample: log family preserved at {}",
        artifact.display()
    );
    if !healthy {
        return inconclusive(&format!(
            "the app never accepted on :4317 within {}s",
            HEALTHY_WAIT.as_secs()
        ));
    }

    let results = perf_budget::grade(&perf_budget::read_family(&artifact)?);
    let required = [Arm::Frame];
    for line in perf_budget::arm_lines(&results, &required) {
        println!("perf:frame-sample: {line}");
    }
    let frame = results.iter().find(|r| r.arm == Arm::Frame);
    if matches!(frame.map(|r| &r.state), Some(ArmState::Neutral { .. })) {
        println!("perf:frame-sample: frame: 0 samples — no WebGPU adapter in this run");
    }
    let verdict = perf_budget::evaluate(&results, &required);
    println!("perf:frame-sample: {}", verdict.word());
    Ok(match verdict {
        perf_budget::Verdict::Pass => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_env_carries_the_flag_set_and_data_dir_without_touching_the_parent() {
        let before = std::env::var_os(WEBVIEW2_ARGS_VAR);
        let dir = Path::new("frame-sample-data");
        let env = child_env(dir);
        assert!(env.contains(&(
            OsString::from(WEBVIEW2_ARGS_VAR),
            OsString::from(
                "--enable-unsafe-webgpu --enable-features=Vulkan --use-vulkan=swiftshader \
                 --use-webgpu-adapter=swiftshader"
            )
        )));
        assert!(env.contains(&(
            OsString::from("ANDROMEDA_PULSE_DATA_DIR"),
            OsString::from("frame-sample-data")
        )));
        assert_eq!(std::env::var_os(WEBVIEW2_ARGS_VAR), before);
    }

    #[test]
    fn non_windows_host_cannot_evaluate() {
        assert!(platform_guard("windows").is_ok());
        for os in ["linux", "macos"] {
            let why = platform_guard(os).expect_err("non-Windows is cannot-evaluate");
            assert!(why.contains(os), "{why}");
        }
    }
}
