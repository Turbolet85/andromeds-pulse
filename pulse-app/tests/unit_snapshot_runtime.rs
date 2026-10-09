// Migrated 2026-08-30 from `pulse-app/src/snapshot_runtime.rs::tests` — that
// crate sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS.
//
// Four resolver-level `generate` tests from that dead module are DELETED
// rather than migrated: any test binary that links `SnapshotApiImpl::generate`
// pulls the tauri clipboard/notification plugin import chain and ABORTS at
// load with STATUS_ENTRYPOINT_NOT_FOUND before test discovery (measured on
// this host; the same loader mechanism that set `[lib] test = false`), so
// they were never runnable anywhere. The P2 curation/markdown invariants are
// covered by `e2e_p2_snapshot_generate.rs`, which deliberately takes the
// direct-library path for this exact reason; resolver-level snapshot
// coverage needs the Tauri lift (mock_builder IPC-surrogate) and is a
// surfaced gap, not silent.

use ui_bridge::contract::SnapshotPreset;

use pulse_app::snapshot_runtime::{preset_label, preset_prompts, preset_to_budget};
use snapshot::contract::TokenBudget;

#[test]
fn preset_prompts_returns_all_four_canonical_entries() {
    let p = preset_prompts();
    assert_eq!(p.len(), 4);
    let ids: Vec<&str> = p.iter().map(|d| d.id.as_str()).collect();
    assert!(ids.contains(&"diagnose-latency-outlier"));
    assert!(ids.contains(&"find-error-correlation"));
    assert!(ids.contains(&"trace-failed-request"));
    assert!(ids.contains(&"summarize-service-health"));
}

#[test]
fn preset_label_maps_each_variant_to_lowercase_string() {
    assert_eq!(preset_label(SnapshotPreset::Conservative), "conservative");
    assert_eq!(preset_label(SnapshotPreset::Balanced), "balanced");
    assert_eq!(preset_label(SnapshotPreset::Detailed), "detailed");
}

#[test]
fn preset_to_budget_maps_each_variant_to_matching_budget() {
    assert_eq!(
        preset_to_budget(SnapshotPreset::Conservative),
        TokenBudget::Conservative
    );
    assert_eq!(
        preset_to_budget(SnapshotPreset::Balanced),
        TokenBudget::Balanced
    );
    assert_eq!(
        preset_to_budget(SnapshotPreset::Detailed),
        TokenBudget::Detailed
    );
}
