use std::path::{Path, PathBuf};
use std::{env, fs};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use ui_bridge::health::{HealthApi, HealthApiImpl, record_start};

const SERVICE_NAME: &str = "com.andromeda.pulse";
const DEPLOYMENT_ENVIRONMENT: &str = "production";

fn resolve_data_dir() -> PathBuf {
    if let Ok(p) = env::var("ANDROMEDA_PULSE_DATA_DIR") {
        return PathBuf::from(p);
    }
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata).join("andromeda-pulse");
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join(SERVICE_NAME);
        }
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("andromeda-pulse");
    } else if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".andromeda-pulse");
    }
    env::temp_dir().join("andromeda-pulse")
}

fn init_tracing(data_dir: &Path) -> WorkerGuard {
    let logs_dir = data_dir.join("logs");
    fs::create_dir_all(&logs_dir).expect("failed to create logs dir");

    let file_appender = tracing_appender::rolling::daily(&logs_dir, "agent-latest.jsonl");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_env("ANDROMEDA_PULSE_LOG_LEVEL")
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let json_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_target(true)
        .with_current_span(true)
        .with_span_list(false);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .with(ErrorLayer::default())
        .init();

    tracing::info!(
        target: "app.boot.tracing.init",
        service_name = SERVICE_NAME,
        service_version = env!("CARGO_PKG_VERSION"),
        deployment_environment = DEPLOYMENT_ENVIRONMENT,
        log_dir = ?logs_dir,
        "tracing subscriber initialized",
    );

    guard
}

fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown".to_string());
        let msg = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("(non-string panic payload)");
        tracing::error!(
            target: "app.panic.fatal",
            panic_message = %msg,
            location = %location,
            spantrace = ?tracing_error::SpanTrace::capture(),
            "panic captured",
        );
    }));
}

fn write_pid_file(data_dir: &Path) {
    let run_dir = data_dir.join("run");
    if let Err(e) = fs::create_dir_all(&run_dir) {
        tracing::warn!(target: "app.boot.pid", error = %e, "failed to create run dir");
        return;
    }
    let pid_path = run_dir.join("andromeda-pulse.pid");
    let canonical_data = match fs::canonicalize(data_dir) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(target: "app.boot.pid", error = %e, "data dir canonicalize failed");
            return;
        }
    };
    let canonical_run = match fs::canonicalize(&run_dir) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(target: "app.boot.pid", error = %e, "run dir canonicalize failed");
            return;
        }
    };
    if !canonical_run.starts_with(&canonical_data) {
        tracing::error!(
            target: "app.boot.pid",
            run_dir = ?canonical_run,
            data_dir = ?canonical_data,
            "run dir escaped data dir; refusing to write PID",
        );
        return;
    }
    let pid = std::process::id();
    if let Err(e) = fs::write(&pid_path, pid.to_string()) {
        tracing::warn!(target: "app.boot.pid", error = %e, "failed to write PID file");
        return;
    }
    tracing::info!(target: "app.boot.pid", pid = pid, path = ?pid_path, "PID file written");
}

fn main() {
    let data_dir = resolve_data_dir();
    let _guard = init_tracing(&data_dir);
    install_panic_hook();
    record_start();
    write_pid_file(&data_dir);

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(taurpc::create_ipc_handler(HealthApiImpl.into_handler()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
