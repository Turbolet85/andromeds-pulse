//! Filesystem watcher + debounced reload core.
//!
//! A `notify` recommended watcher observes the data dir; config-file
//! changes are coalesced with a 500ms debounce, then the file is re-loaded
//! and re-validated through `ui_bridge::Settings` + `Settings::validate()`.
//! On success the validated `Settings` is published on a `tokio::sync::watch`
//! channel (latest-value → prospective application) and an aggregate
//! `ConfigEvent` is broadcast. On failure the previous valid config is
//! retained (malformed config never applies).

use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use thiserror::Error;
use tokio::sync::{mpsc, watch};
use ui_bridge::contract::Settings;

use crate::event::{ConfigEvent, ConfigEventBroadcast, ConfigEventKind, kind_label};
use crate::partition::{ChangedKeys, partition_changed_keys};

/// Debounce window applied to coalesce a burst of filesystem events into a
/// single reload (per chunk §94 "500ms debounce").
pub const DEBOUNCE: Duration = Duration::from_millis(500);

pub const TARGET_CONFIG_LOAD: &str = "config.load";
pub const TARGET_CONFIG_LOAD_REJECTED: &str = "config.load.rejected";
pub const TARGET_CONFIG_LOAD_PATH_VALIDATION: &str = "config.load.path_validation";

#[derive(Debug, Error)]
pub enum ConfigWatchError {
    #[error("config data dir path rejected (parent-dir traversal)")]
    PathTraversal,
    #[error("config data dir could not be resolved")]
    DataDirUnresolved,
    #[error("filesystem watcher initialization failed")]
    WatcherInit,
}

/// Read-only status snapshot surfaced by `config.status()`. No raw config
/// values / paths — only a timestamp, a bounded error-category label, and
/// counts.
#[derive(Debug, Clone, Default)]
pub struct ConfigStatus {
    pub last_reload_unix_nano: i64,
    pub last_error_category: Option<String>,
    pub restart_required_pending: u32,
    pub reload_count: u64,
}

/// Outcome of a reload attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReloadOutcome {
    /// New config validated + applied; carries the changed-key partition.
    Applied(ChangedKeys),
    /// Settled change failed parse / validation / read; previous retained.
    Rejected { category: &'static str },
    /// Settled change produced an identical config — no-op.
    Unchanged,
}

/// Canonicalize the data dir + return the `config.toml` path under it.
///
/// Rejects a data-dir override containing a `..` component BEFORE touching
/// the filesystem (CWE-22; mirrors chunk #95 `training_export.rs` egress
/// validation). The config basename is a fixed literal, so the resolved
/// path cannot escape the canonicalized data dir.
pub fn resolve_config_path(data_dir: &Path) -> Result<PathBuf, ConfigWatchError> {
    if data_dir
        .components()
        .any(|c| matches!(c, Component::ParentDir))
    {
        tracing::warn!(
            target: TARGET_CONFIG_LOAD_PATH_VALIDATION,
            reason = "parent_dir_traversal",
            "config data dir rejected",
        );
        return Err(ConfigWatchError::PathTraversal);
    }
    let canonical =
        std::fs::canonicalize(data_dir).map_err(|_| ConfigWatchError::DataDirUnresolved)?;
    Ok(canonical.join("config.toml"))
}

/// Pure reload-and-classify: read + parse + validate the config file and
/// compare against the previous valid config. Never mutates anything;
/// callers decide what to do with the outcome.
pub fn reload_and_classify(path: &Path, previous: &Settings) -> ReloadOutcome {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            return ReloadOutcome::Rejected {
                category: "read_failed",
            };
        }
    };
    let parsed: Settings = match toml::from_str(&content) {
        Ok(s) => s,
        Err(_) => {
            return ReloadOutcome::Rejected {
                category: "toml_parse",
            };
        }
    };
    if parsed.validate().is_err() {
        return ReloadOutcome::Rejected {
            category: "validation",
        };
    }
    if &parsed == previous {
        return ReloadOutcome::Unchanged;
    }
    ReloadOutcome::Applied(partition_changed_keys(previous, &parsed))
}

/// Shared handle wiring a reload to its side effects (watch publish + event
/// broadcast + status update). Cloned into both the watcher task (settle
/// loop) and the `config.reload()` TauRPC resolver (manual trigger).
#[derive(Clone)]
pub struct ConfigWatchHandle {
    config_path: PathBuf,
    settings_tx: watch::Sender<Settings>,
    events: Arc<ConfigEventBroadcast>,
    status: Arc<Mutex<ConfigStatus>>,
}

impl ConfigWatchHandle {
    /// Force an immediate re-read + re-apply. Returns the outcome so callers
    /// (e.g. `config.reload()`) can build a response payload.
    pub fn reload_now(&self) -> ReloadOutcome {
        let previous = self.settings_tx.borrow().clone();
        let content = std::fs::read_to_string(&self.config_path);
        let outcome = match content {
            Err(_) => ReloadOutcome::Rejected {
                category: "read_failed",
            },
            Ok(text) => match toml::from_str::<Settings>(&text) {
                Err(_) => ReloadOutcome::Rejected {
                    category: "toml_parse",
                },
                Ok(parsed) => {
                    if parsed.validate().is_err() {
                        ReloadOutcome::Rejected {
                            category: "validation",
                        }
                    } else if parsed == previous {
                        ReloadOutcome::Unchanged
                    } else {
                        let changed = partition_changed_keys(&previous, &parsed);
                        self.apply(parsed, &changed);
                        ReloadOutcome::Applied(changed)
                    }
                }
            },
        };
        if let ReloadOutcome::Rejected { category } = &outcome {
            self.record_rejection(category);
        }
        outcome
    }

    fn apply(&self, parsed: Settings, changed: &ChangedKeys) {
        let now = now_unix_nano();
        let _ = self.settings_tx.send_replace(parsed);
        {
            let mut st = self.status.lock().expect("config status mutex");
            st.last_reload_unix_nano = now;
            st.last_error_category = None;
            st.restart_required_pending = changed.restart_required.len() as u32;
            st.reload_count += 1;
        }
        emit_event(&self.events, ConfigEventKind::Reloaded, now, changed, None);
        if !changed.restart_required.is_empty() {
            emit_event(
                &self.events,
                ConfigEventKind::RestartRequired,
                now,
                changed,
                None,
            );
        }
        tracing::info!(
            target: TARGET_CONFIG_LOAD,
            hot_applied_count = changed.hot_applied.len() as u64,
            restart_required_count = changed.restart_required.len() as u64,
            silent_count = changed.silent.len() as u64,
            "config reloaded",
        );
    }

    fn record_rejection(&self, category: &'static str) {
        let now = now_unix_nano();
        {
            let mut st = self.status.lock().expect("config status mutex");
            st.last_error_category = Some(category.to_string());
        }
        emit_event(
            &self.events,
            ConfigEventKind::ParseRejected,
            now,
            &ChangedKeys::default(),
            Some(category),
        );
        tracing::warn!(
            target: TARGET_CONFIG_LOAD_REJECTED,
            error_category = category,
            "config reload rejected; previous config retained",
        );
    }

    pub fn status_snapshot(&self) -> ConfigStatus {
        self.status.lock().expect("config status mutex").clone()
    }
}

fn emit_event(
    events: &ConfigEventBroadcast,
    kind: ConfigEventKind,
    at: i64,
    changed: &ChangedKeys,
    error_category: Option<&'static str>,
) {
    let _ = events.sender().send(ConfigEvent {
        kind,
        kind_label: kind_label(kind),
        at_unix_nano: at,
        hot_applied_count: changed.hot_applied.len() as u32,
        restart_required_count: changed.restart_required.len() as u32,
        silent_count: changed.silent.len() as u32,
        error_category,
    });
}

/// Await a debounce settle: consume coalescing events until `window` of
/// quiet elapses. Returns `false` if the event channel closed. Pure async
/// (no `notify` dependency) → deterministic under `tokio::time::pause()`.
pub async fn wait_for_settled(rx: &mut mpsc::UnboundedReceiver<()>, window: Duration) -> bool {
    loop {
        tokio::select! {
            maybe = rx.recv() => {
                if maybe.is_none() {
                    return false;
                }
            }
            _ = tokio::time::sleep(window) => {
                return true;
            }
        }
    }
}

/// Background watcher task: owns the `notify` watcher + the event channel and
/// drives the debounced reload loop. The caller spawns `run()` with its own
/// async spawner so `config-watcher` stays runtime-agnostic (no `tokio::spawn`
/// at an uncertain runtime-context boundary — e.g. a Tauri setup closure uses
/// `tauri::async_runtime::spawn`).
pub struct ConfigWatchTask {
    _watcher: notify::RecommendedWatcher,
    rx: mpsc::UnboundedReceiver<()>,
    handle: ConfigWatchHandle,
}

impl ConfigWatchTask {
    pub async fn run(mut self) {
        while self.rx.recv().await.is_some() {
            if !wait_for_settled(&mut self.rx, DEBOUNCE).await {
                break;
            }
            let _ = self.handle.reload_now();
        }
    }
}

/// Build the filesystem watcher + debounce-reload task. Returns the
/// `ConfigWatchHandle` (for the manual `config.reload()` trigger) plus the
/// `ConfigWatchTask` the caller spawns. The watcher is owned by the task and
/// stays alive for its lifetime.
pub fn start_config_watcher(
    data_dir: &Path,
    settings_tx: watch::Sender<Settings>,
    events: Arc<ConfigEventBroadcast>,
    status: Arc<Mutex<ConfigStatus>>,
) -> Result<(ConfigWatchHandle, ConfigWatchTask), ConfigWatchError> {
    use notify::{RecursiveMode, Watcher};

    let config_path = resolve_config_path(data_dir)?;
    let watch_dir = config_path
        .parent()
        .map(Path::to_path_buf)
        .ok_or(ConfigWatchError::DataDirUnresolved)?;

    let handle = ConfigWatchHandle {
        config_path: config_path.clone(),
        settings_tx,
        events,
        status,
    };

    let (tx, rx) = mpsc::unbounded_channel::<()>();
    let cb_name = config_path.file_name().map(|n| n.to_os_string());
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            // Filter by file name, not full-path equality: `canonicalize` on
            // Windows yields `\\?\`-prefixed paths that need not match notify's
            // reported event paths byte-for-byte.
            if event
                .paths
                .iter()
                .any(|p| p.file_name() == cb_name.as_deref())
            {
                let _ = tx.send(());
            }
        }
    })
    .map_err(|_| ConfigWatchError::WatcherInit)?;
    watcher
        .watch(&watch_dir, RecursiveMode::NonRecursive)
        .map_err(|_| ConfigWatchError::WatcherInit)?;

    let task = ConfigWatchTask {
        _watcher: watcher,
        rx,
        handle: handle.clone(),
    };
    Ok((handle, task))
}

fn now_unix_nano() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn write_config(dir: &Path, body: &str) -> PathBuf {
        let path = dir.join("config.toml");
        let mut f = std::fs::File::create(&path).expect("create config");
        f.write_all(body.as_bytes()).expect("write config");
        path
    }

    #[test]
    fn resolve_config_path_rejects_parent_dir_traversal() {
        let td = TempDir::new().expect("temp dir");
        let evil = td.path().join("..").join("escape");
        let err = resolve_config_path(&evil).expect_err("traversal must reject");
        assert!(matches!(err, ConfigWatchError::PathTraversal));
    }

    #[test]
    fn resolve_config_path_returns_config_under_canonical_dir() {
        let td = TempDir::new().expect("temp dir");
        let resolved = resolve_config_path(td.path()).expect("resolve");
        assert_eq!(resolved.file_name().unwrap(), "config.toml");
        assert!(resolved.starts_with(std::fs::canonicalize(td.path()).unwrap()));
    }

    #[test]
    fn reload_and_classify_applies_valid_change() {
        let td = TempDir::new().expect("temp dir");
        let previous = Settings::default();
        let path = write_config(
            td.path(),
            &format!(
                "cadence_baseline_seconds = {}\n",
                previous.cadence_baseline_seconds + 10
            ),
        );
        match reload_and_classify(&path, &previous) {
            ReloadOutcome::Applied(changed) => {
                assert!(changed.hot_applied.contains(&"cadence_baseline_seconds"));
            }
            other => panic!("expected Applied, got {other:?}"),
        }
    }

    #[test]
    fn reload_and_classify_rejects_out_of_range_and_retains() {
        let td = TempDir::new().expect("temp dir");
        let previous = Settings::default();
        // cadence_baseline_seconds below the safety floor (5) fails validate().
        let path = write_config(td.path(), "cadence_baseline_seconds = 1\n");
        assert_eq!(
            reload_and_classify(&path, &previous),
            ReloadOutcome::Rejected {
                category: "validation"
            }
        );
    }

    #[test]
    fn reload_and_classify_rejects_malformed_toml() {
        let td = TempDir::new().expect("temp dir");
        let previous = Settings::default();
        let path = write_config(td.path(), "this is not valid toml = = =\n");
        assert_eq!(
            reload_and_classify(&path, &previous),
            ReloadOutcome::Rejected {
                category: "toml_parse"
            }
        );
    }

    #[test]
    fn reload_and_classify_unchanged_for_identical_config() {
        let td = TempDir::new().expect("temp dir");
        let previous = Settings::default();
        let toml = toml::to_string(&previous).expect("serialize default");
        let path = write_config(td.path(), &toml);
        assert_eq!(
            reload_and_classify(&path, &previous),
            ReloadOutcome::Unchanged
        );
    }

    #[tokio::test(start_paused = true)]
    async fn wait_for_settled_returns_true_after_quiet_window() {
        let (tx, mut rx) = mpsc::unbounded_channel::<()>();
        tx.send(()).expect("send first event");
        // tx stays alive; after draining the queued event the loop sleeps
        // one DEBOUNCE window with no further events → auto-advance settles.
        let settled = wait_for_settled(&mut rx, DEBOUNCE).await;
        assert!(settled);
        drop(tx);
    }

    #[tokio::test(start_paused = true)]
    async fn wait_for_settled_returns_false_when_channel_closes() {
        let (tx, mut rx) = mpsc::unbounded_channel::<()>();
        tx.send(()).expect("send");
        drop(tx); // close after queuing one event
        // first recv drains the queued event; second recv sees closed → false.
        assert!(!wait_for_settled(&mut rx, DEBOUNCE).await);
    }
}
