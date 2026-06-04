//! Configuration hot-reload TauRPC router (chunk #96 — capabilities P-055 / P-056).
//!
//! `config.reload()` forces an immediate re-read + re-apply of `config.toml`;
//! `config.status()` returns the last-reload timestamp + last error category +
//! restart-required pending count. The notify watcher + the `Settings`→consumer
//! fan-out live at the `pulse-app` boot boundary (`main.rs`); this router
//! exposes the manual trigger + status surface. Mirrors `diagnostics_router.rs`.

use std::sync::{Arc, Mutex, OnceLock};

use config_watcher::{ConfigStatus, ConfigWatchError, ConfigWatchHandle, ReloadOutcome};
use serde::{Deserialize, Serialize};
use triage::contract::{CadenceConfig, LifecycleThresholds};
use ui_bridge::contract::{AppError, Settings};

/// Map persisted `Settings` to the cadence coordinator's config value object.
/// `try_new` re-checks the safety floors (already enforced by
/// `Settings::validate`); falls back to defaults defensively.
pub fn settings_to_cadence_config(s: &Settings) -> CadenceConfig {
    CadenceConfig::try_new(
        s.cadence_baseline_seconds,
        s.cadence_accelerated_seconds,
        s.cadence_reflection_seconds,
        s.cadence_tier2_acceleration_enabled,
    )
    .unwrap_or_default()
}

/// Map persisted `Settings` to the lifecycle heartbeat's threshold value object.
pub fn settings_to_lifecycle_thresholds(s: &Settings) -> LifecycleThresholds {
    LifecycleThresholds {
        dormant_after_secs: s.lifecycle_dormant_after_secs,
        archived_after_secs: s.lifecycle_archived_after_secs,
    }
}

/// Bounded category label for a watcher-init failure (sanitized — no path).
pub fn config_watch_error_category(err: &ConfigWatchError) -> &'static str {
    match err {
        ConfigWatchError::PathTraversal => "path_traversal",
        ConfigWatchError::DataDirUnresolved => "data_dir_unresolved",
        ConfigWatchError::WatcherInit => "watcher_init",
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ConfigReloadPayload {
    pub applied: bool,
    /// Bounded outcome label: `"applied"` / `"unchanged"` / `"rejected"`.
    pub outcome: String,
    pub hot_applied_count: u32,
    pub restart_required_count: u32,
    /// Bounded error-category label when `outcome == "rejected"`; else `None`.
    pub error_category: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ConfigStatusPayload {
    pub last_reload_unix_nano: i64,
    pub last_error_category: Option<String>,
    pub restart_required_pending: u32,
    pub reload_count: u64,
}

// `config.reload` forces an immediate re-read + re-apply of config.toml: hot
// keys (cadence + lifecycle) publish to their watch channels, restart-required
// keys raise a notice, malformed config is rejected with the previous retained.
// `config.status` returns the last-reload snapshot. Doc lives above the macro
// (taurpc 0.7 rejects multi-line /// attributes inside the trait body).
#[taurpc::procedures(path = "config")]
pub trait ConfigApi {
    async fn reload() -> Result<ConfigReloadPayload, AppError>;
    async fn status() -> Result<ConfigStatusPayload, AppError>;
}

#[derive(Clone)]
pub struct ConfigApiImpl {
    handle: Arc<OnceLock<ConfigWatchHandle>>,
    status: Arc<Mutex<ConfigStatus>>,
}

impl ConfigApiImpl {
    pub fn new(handle: Arc<OnceLock<ConfigWatchHandle>>, status: Arc<Mutex<ConfigStatus>>) -> Self {
        Self { handle, status }
    }
}

#[taurpc::resolvers]
impl ConfigApi for ConfigApiImpl {
    #[tracing::instrument(skip_all, fields(
        outcome = tracing::field::Empty,
        hot_applied_count = tracing::field::Empty,
        restart_required_count = tracing::field::Empty,
    ))]
    async fn reload(self) -> Result<ConfigReloadPayload, AppError> {
        let handle = self
            .handle
            .get()
            .ok_or_else(|| AppError::internal("config watcher not initialized"))?;
        let payload = match handle.reload_now() {
            ReloadOutcome::Applied(changed) => ConfigReloadPayload {
                applied: true,
                outcome: "applied".to_string(),
                hot_applied_count: changed.hot_applied.len() as u32,
                restart_required_count: changed.restart_required.len() as u32,
                error_category: None,
            },
            ReloadOutcome::Unchanged => ConfigReloadPayload {
                applied: false,
                outcome: "unchanged".to_string(),
                hot_applied_count: 0,
                restart_required_count: 0,
                error_category: None,
            },
            ReloadOutcome::Rejected { category } => ConfigReloadPayload {
                applied: false,
                outcome: "rejected".to_string(),
                hot_applied_count: 0,
                restart_required_count: 0,
                error_category: Some(category.to_string()),
            },
        };
        let span = tracing::Span::current();
        span.record("outcome", payload.outcome.as_str());
        span.record("hot_applied_count", payload.hot_applied_count);
        span.record("restart_required_count", payload.restart_required_count);
        tracing::info!(
            target: "config.reload.request",
            outcome = payload.outcome.as_str(),
            hot_applied_count = payload.hot_applied_count as u64,
            restart_required_count = payload.restart_required_count as u64,
            "config.reload returned",
        );
        Ok(payload)
    }

    #[tracing::instrument(skip_all, fields(
        restart_required_pending = tracing::field::Empty,
        reload_count = tracing::field::Empty,
    ))]
    async fn status(self) -> Result<ConfigStatusPayload, AppError> {
        let st = self
            .status
            .lock()
            .map_err(|_| AppError::internal("config status mutex poisoned"))?
            .clone();
        let payload = ConfigStatusPayload {
            last_reload_unix_nano: st.last_reload_unix_nano,
            last_error_category: st.last_error_category,
            restart_required_pending: st.restart_required_pending,
            reload_count: st.reload_count,
        };
        let span = tracing::Span::current();
        span.record("restart_required_pending", payload.restart_required_pending);
        span.record("reload_count", payload.reload_count);
        tracing::info!(
            target: "config.status.request",
            restart_required_pending = payload.restart_required_pending as u64,
            reload_count = payload.reload_count,
            "config.status returned",
        );
        Ok(payload)
    }
}
