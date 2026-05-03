use std::path::{Path, PathBuf};
use std::{env, fs};

use ui_bridge::health::{HealthApi, HealthApiImpl, record_start};

mod observability;

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
                .join("com.andromeda.pulse");
        }
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("andromeda-pulse");
    } else if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".andromeda-pulse");
    }
    env::temp_dir().join("andromeda-pulse")
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
    let _guard = observability::init(&data_dir);
    record_start();
    write_pid_file(&data_dir);

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(taurpc::create_ipc_handler(HealthApiImpl.into_handler()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
