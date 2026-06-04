//! Configuration hot-reload watcher (chunk #96 — capabilities P-055 / P-056).
//!
//! Watches `<data_dir>/config.toml` and, on each settled change (500ms
//! debounce), re-loads + re-validates through `ui_bridge::Settings` +
//! `Settings::validate()`. Hot-reloadable keys (cadence + lifecycle) are
//! published on a `tokio::sync::watch` channel — latest-value semantics make
//! application **prospective** (consumers re-read on their next tick; missed
//! ticks are never replayed). Restart-required keys (Drain params, retention,
//! MCP sidecar) raise a notice via `pulse://stream/config-events` rather than
//! mutating arch-locked startup decisions live. Malformed config is rejected
//! and the previous valid config retained.
//!
//! The crate depends only on `ui-bridge` (for `Settings`) — a forward edge
//! per arch §Cross-cutting Module dependency direction. Consumer wiring +
//! the `config.*` TauRPC router + the `Settings`→consumer fan-out live at the
//! `pulse-app` binary boundary.

mod event;
mod partition;
mod watcher;

pub use event::{
    BROADCAST_CAPACITY, ConfigEvent, ConfigEventBroadcast, ConfigEventKind,
    STREAM_NAME_CONFIG_EVENTS, kind_label,
};
pub use partition::{ChangedKeys, partition_changed_keys};
pub use watcher::{
    ConfigStatus, ConfigWatchError, ConfigWatchHandle, ConfigWatchTask, DEBOUNCE, ReloadOutcome,
    TARGET_CONFIG_LOAD, TARGET_CONFIG_LOAD_PATH_VALIDATION, TARGET_CONFIG_LOAD_REJECTED,
    reload_and_classify, resolve_config_path, start_config_watcher, wait_for_settled,
};
