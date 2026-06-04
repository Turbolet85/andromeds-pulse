//! Chunk #96 — pulse-app-boundary glue tests for the configuration hot-reload
//! layer (the `config_router` Settings→consumer mappers + the `reevaluation`
//! engine). The `config-watcher` crate's own pure logic (debounce / partition /
//! validate-and-retain) is covered by its co-located unit tests; this file
//! covers the binary-boundary glue. Lives under `pulse-app/tests/` because
//! `[lib] test = false` means source-level `mod tests` never run (CLAUDE.md
//! 2026-05-20).

use std::sync::Arc;

use tokio::sync::watch;

use pulse_app::config_router::{
    config_watch_error_category, settings_to_cadence_config, settings_to_lifecycle_thresholds,
};
use pulse_app::reevaluation::{LiveReevaluator, RecentWindowReevaluator};
use triage::contract::{
    BaselineState, InMemoryServiceRegistry, LifecycleThresholds, ServiceLifecycleBroadcast,
    ServiceRegistry,
};
use ui_bridge::contract::Settings;

const NANOS_PER_SEC: i64 = 1_000_000_000;

#[test]
fn settings_to_cadence_config_maps_fields() {
    let s = Settings::default();
    let c = settings_to_cadence_config(&s);
    assert_eq!(c.baseline_seconds, s.cadence_baseline_seconds);
    assert_eq!(c.accelerated_seconds, s.cadence_accelerated_seconds);
    assert_eq!(c.reflection_seconds, s.cadence_reflection_seconds);
    assert_eq!(
        c.tier2_acceleration_enabled,
        s.cadence_tier2_acceleration_enabled
    );
}

#[test]
fn settings_to_lifecycle_thresholds_maps_fields() {
    let s = Settings::default();
    let t = settings_to_lifecycle_thresholds(&s);
    assert_eq!(t.dormant_after_secs, s.lifecycle_dormant_after_secs);
    assert_eq!(t.archived_after_secs, s.lifecycle_archived_after_secs);
}

#[test]
fn config_watch_error_category_is_sanitized_label() {
    use config_watcher::ConfigWatchError;
    assert_eq!(
        config_watch_error_category(&ConfigWatchError::PathTraversal),
        "path_traversal"
    );
    assert_eq!(
        config_watch_error_category(&ConfigWatchError::DataDirUnresolved),
        "data_dir_unresolved"
    );
    assert_eq!(
        config_watch_error_category(&ConfigWatchError::WatcherInit),
        "watcher_init"
    );
}

fn make_reevaluator(
    baseline: Arc<BaselineState>,
    registry: Arc<dyn ServiceRegistry>,
    thresholds: LifecycleThresholds,
) -> (LiveReevaluator, Arc<ServiceLifecycleBroadcast>) {
    let broadcast = Arc::new(ServiceLifecycleBroadcast::new());
    let (_tx, rx) = watch::channel(thresholds);
    let reeval = LiveReevaluator::new(registry, Arc::clone(&broadcast), baseline, rx);
    (reeval, broadcast)
}

#[test]
fn reevaluate_on_empty_registry_is_noop_but_marks_cadence_prospective() {
    let baseline = Arc::new(BaselineState::new());
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    let (reeval, _bcast) = make_reevaluator(
        baseline,
        registry,
        LifecycleThresholds {
            dormant_after_secs: 3_600,
            archived_after_secs: 86_400,
        },
    );
    let summary = reeval.reevaluate();
    assert_eq!(summary.services_reclassified, 0);
    assert_eq!(summary.transitions_emitted, 0);
    assert!(summary.cadence_prospective);
}

#[test]
fn reevaluate_reclassifies_tracked_services_and_emits_transitions() {
    let baseline = Arc::new(BaselineState::new());
    baseline.observe_span("svc-a", "op", 0, 50, 1_000 * NANOS_PER_SEC);
    let registry: Arc<dyn ServiceRegistry> = Arc::new(InMemoryServiceRegistry::new());
    let (reeval, mut bcast_rx) = {
        let (r, b) = make_reevaluator(
            Arc::clone(&baseline),
            Arc::clone(&registry),
            LifecycleThresholds {
                dormant_after_secs: 3_600,
                archived_after_secs: 86_400,
            },
        );
        let rx = b.subscribe();
        (r, rx)
    };

    let summary = reeval.reevaluate();
    // First retrospective pass tracks svc-a (Unknown → Bootstrapping).
    assert_eq!(summary.services_reclassified, 1);
    assert_eq!(summary.transitions_emitted, 1);
    let event = bcast_rx.try_recv().expect("lifecycle event broadcast");
    assert_eq!(event.service, "svc-a");
}
