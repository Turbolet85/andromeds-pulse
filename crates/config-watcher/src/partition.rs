//! Hot-vs-restart key partition.
//!
//! Classifies which `Settings` fields changed between the previous and
//! newly-loaded config into three buckets per chunk #96 scope:
//!
//! - **hot_applied** — re-applied live this chunk (cadence + lifecycle keys);
//!   their consumers re-read from the `watch` channel within ~2s.
//! - **restart_required** — raise a notice but DO NOT apply live (Drain
//!   params, retention, MCP sidecar toggle map to arch-locked one-time-init
//!   substrate per arch §Established Decisions).
//! - **silent** — UI preferences / per-use values that need no notice (they
//!   are normally changed via the Settings modal + picked up on next read).

use ui_bridge::contract::Settings;

/// The set of changed keys partitioned by application policy.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangedKeys {
    pub hot_applied: Vec<&'static str>,
    pub restart_required: Vec<&'static str>,
    pub silent: Vec<&'static str>,
}

impl ChangedKeys {
    pub fn is_empty(&self) -> bool {
        self.hot_applied.is_empty() && self.restart_required.is_empty() && self.silent.is_empty()
    }
}

/// Compare `old` vs `new` field-by-field and bucket each changed key.
pub fn partition_changed_keys(old: &Settings, new: &Settings) -> ChangedKeys {
    let mut c = ChangedKeys::default();

    // Hot-applied (live this chunk): cadence coordinator (#80) + lifecycle
    // heartbeat (#67) re-read these from the watch channel.
    if old.cadence_baseline_seconds != new.cadence_baseline_seconds {
        c.hot_applied.push("cadence_baseline_seconds");
    }
    if old.cadence_accelerated_seconds != new.cadence_accelerated_seconds {
        c.hot_applied.push("cadence_accelerated_seconds");
    }
    if old.cadence_reflection_seconds != new.cadence_reflection_seconds {
        c.hot_applied.push("cadence_reflection_seconds");
    }
    if old.cadence_tier2_acceleration_enabled != new.cadence_tier2_acceleration_enabled {
        c.hot_applied.push("cadence_tier2_acceleration_enabled");
    }
    if old.lifecycle_dormant_after_secs != new.lifecycle_dormant_after_secs {
        c.hot_applied.push("lifecycle_dormant_after_secs");
    }
    if old.lifecycle_archived_after_secs != new.lifecycle_archived_after_secs {
        c.hot_applied.push("lifecycle_archived_after_secs");
    }

    // Restart-required (notice only): Drain template-miner knobs are
    // captured at boot per the contract.rs comment ("changes require
    // restart to apply"); retention + MCP sidecar likewise bind at boot.
    if old.drain_depth != new.drain_depth {
        c.restart_required.push("drain_depth");
    }
    if old.drain_similarity_x100 != new.drain_similarity_x100 {
        c.restart_required.push("drain_similarity_x100");
    }
    if old.drain_max_clusters != new.drain_max_clusters {
        c.restart_required.push("drain_max_clusters");
    }
    if old.retention_seconds != new.retention_seconds {
        c.restart_required.push("retention_seconds");
    }
    if old.mcp_server_enabled != new.mcp_server_enabled {
        c.restart_required.push("mcp_server_enabled");
    }

    // Silent (no notice): UI prefs / per-use values.
    if old.theme != new.theme {
        c.silent.push("theme");
    }
    if old.widget_position != new.widget_position {
        c.silent.push("widget_position");
    }
    if old.notifications_enabled != new.notifications_enabled {
        c.silent.push("notifications_enabled");
    }
    if old.always_on_top != new.always_on_top {
        c.silent.push("always_on_top");
    }
    if old.snapshot_preset != new.snapshot_preset {
        c.silent.push("snapshot_preset");
    }
    if old.snapshot_format != new.snapshot_format {
        c.silent.push("snapshot_format");
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_bridge::contract::{SnapshotPreset, Theme};

    #[test]
    fn no_change_yields_empty_buckets() {
        let s = Settings::default();
        assert!(partition_changed_keys(&s, &s).is_empty());
    }

    #[test]
    fn cadence_and_lifecycle_changes_land_in_hot_bucket() {
        let old = Settings::default();
        let new = Settings {
            cadence_baseline_seconds: old.cadence_baseline_seconds + 10,
            lifecycle_dormant_after_secs: old.lifecycle_dormant_after_secs + 60,
            ..old.clone()
        };
        let c = partition_changed_keys(&old, &new);
        assert!(c.hot_applied.contains(&"cadence_baseline_seconds"));
        assert!(c.hot_applied.contains(&"lifecycle_dormant_after_secs"));
        assert!(c.restart_required.is_empty());
        assert!(c.silent.is_empty());
    }

    #[test]
    fn drain_retention_mcp_changes_land_in_restart_bucket() {
        let old = Settings::default();
        let new = Settings {
            drain_depth: 5,
            retention_seconds: old.retention_seconds + 30,
            mcp_server_enabled: !old.mcp_server_enabled,
            ..old.clone()
        };
        let c = partition_changed_keys(&old, &new);
        assert!(c.restart_required.contains(&"drain_depth"));
        assert!(c.restart_required.contains(&"retention_seconds"));
        assert!(c.restart_required.contains(&"mcp_server_enabled"));
        assert!(c.hot_applied.is_empty());
    }

    #[test]
    fn ui_pref_changes_land_in_silent_bucket() {
        let old = Settings::default();
        let new = Settings {
            theme: Theme::Light,
            snapshot_preset: SnapshotPreset::Detailed,
            ..old.clone()
        };
        let c = partition_changed_keys(&old, &new);
        assert!(c.silent.contains(&"theme"));
        assert!(c.silent.contains(&"snapshot_preset"));
        assert!(c.hot_applied.is_empty());
        assert!(c.restart_required.is_empty());
    }
}
