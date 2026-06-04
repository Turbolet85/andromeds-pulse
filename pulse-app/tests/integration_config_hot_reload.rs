//! Chunk #96 — end-to-end configuration hot-reload integration test.
//!
//! Drives the real `notify` watcher against a temp-dir `config.toml`;
//! assertions wait on the `pulse://stream/config-events` broadcast signal
//! (never `sleep(N)` per testing.md). Covers: a hot cadence key applies +
//! publishes the new Settings; malformed config is rejected with the previous
//! retained; an out-of-range value is rejected; a restart-required key raises a
//! notice without applying.

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use config_watcher::{
    ConfigEvent, ConfigEventBroadcast, ConfigEventKind, ConfigStatus, start_config_watcher,
};
use tokio::sync::broadcast::Receiver as BroadcastReceiver;
use tokio::sync::watch;
use ui_bridge::contract::Settings;

fn write_config(dir: &Path, body: &str) {
    std::fs::write(dir.join("config.toml"), body).expect("write config.toml");
}

async fn wait_for_kind(
    rx: &mut BroadcastReceiver<ConfigEvent>,
    kind: ConfigEventKind,
) -> ConfigEvent {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match rx.recv().await {
                Ok(ev) if ev.kind == kind => return ev,
                Ok(_) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    panic!("config-events channel closed before {kind:?}")
                }
            }
        }
    })
    .await
    .unwrap_or_else(|_| panic!("config event {kind:?} not received within 10s"))
}

struct Harness {
    _td: tempfile::TempDir,
    dir: std::path::PathBuf,
    settings_rx: watch::Receiver<Settings>,
    status: Arc<Mutex<ConfigStatus>>,
    event_rx: BroadcastReceiver<ConfigEvent>,
}

fn start(initial: Settings) -> Harness {
    let td = tempfile::TempDir::new().expect("temp dir");
    let dir = td.path().to_path_buf();
    let (settings_tx, settings_rx) = watch::channel(initial);
    let events = Arc::new(ConfigEventBroadcast::new());
    let status = Arc::new(Mutex::new(ConfigStatus::default()));
    let event_rx = events.subscribe();
    let (_handle, task) =
        start_config_watcher(&dir, settings_tx, Arc::clone(&events), Arc::clone(&status))
            .expect("watcher starts");
    tokio::spawn(task.run());
    Harness {
        _td: td,
        dir,
        settings_rx,
        status,
        event_rx,
    }
}

#[tokio::test]
async fn hot_reload_applies_cadence_change_and_publishes_new_settings() {
    let mut h = start(Settings::default());
    // Change a hot cadence key (baseline 60 → 30).
    write_config(&h.dir, "cadence_baseline_seconds = 30\n");
    let ev = wait_for_kind(&mut h.event_rx, ConfigEventKind::Reloaded).await;
    assert!(ev.hot_applied_count >= 1, "expected a hot-applied key");
    // The watch channel reflects the new validated Settings (prospective).
    assert_eq!(h.settings_rx.borrow().cadence_baseline_seconds, 30);
}

#[tokio::test]
async fn hot_reload_rejects_malformed_and_retains_previous() {
    let mut h = start(Settings::default());
    let previous = h.settings_rx.borrow().cadence_baseline_seconds;
    write_config(&h.dir, "this is not = = valid toml\n");
    let ev = wait_for_kind(&mut h.event_rx, ConfigEventKind::ParseRejected).await;
    assert!(ev.error_category.is_some());
    // Previous valid config retained (watch unchanged).
    assert_eq!(h.settings_rx.borrow().cadence_baseline_seconds, previous);
    assert!(
        h.status
            .lock()
            .expect("status")
            .last_error_category
            .is_some()
    );
}

#[tokio::test]
async fn hot_reload_out_of_range_value_rejected_and_retained() {
    let mut h = start(Settings::default());
    // cadence_baseline_seconds below the safety floor (5) fails validate().
    write_config(&h.dir, "cadence_baseline_seconds = 1\n");
    let ev = wait_for_kind(&mut h.event_rx, ConfigEventKind::ParseRejected).await;
    assert_eq!(ev.error_category, Some("validation"));
    assert_eq!(h.settings_rx.borrow().cadence_baseline_seconds, 60);
}

#[tokio::test]
async fn hot_reload_restart_required_key_raises_notice() {
    let mut h = start(Settings::default());
    // drain_depth is restart-required; 5 is in range (3..=5).
    write_config(&h.dir, "drain_depth = 5\n");
    let ev = wait_for_kind(&mut h.event_rx, ConfigEventKind::RestartRequired).await;
    assert!(ev.restart_required_count >= 1);
    assert!(h.status.lock().expect("status").restart_required_pending >= 1);
}
