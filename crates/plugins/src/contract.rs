use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("placeholder")]
    Placeholder,

    /// wasmtime `Engine` construction failed. Reason carries a sanitized
    /// one-liner (no full paths / library versions / stack traces).
    /// Substrate-only at chunk #45; chunk #47 IPC binding re-routes to
    /// `AppError::Plugin` when a plugin_id is available at the call site.
    #[error("engine init failed: {reason}")]
    EngineInit { reason: String },

    /// WIT interface schema load / parse failure. `plugin_id` carries
    /// the requesting category identifier (`custom-dashboard` /
    /// `data-transform` / `snapshot-template`); `reason` is sanitized.
    #[error("wit load failed for plugin `{plugin_id}`: {reason}")]
    WitLoad { plugin_id: String, reason: String },

    /// Component instantiation / link failure. Fires when a guest WASM
    /// imports a host function NOT declared in its WIT contract
    /// (capability-scoping by construction) or when the Component bytes
    /// are otherwise invalid. `plugin_id` carries the plugin identity;
    /// `reason` is sanitized.
    #[error("component instantiate failed for plugin `{plugin_id}`: {reason}")]
    ComponentInstantiate { plugin_id: String, reason: String },
}

/// Plugin category taxonomy per route#45 (Epoch 7 opener). Names map
/// 1:1 to WIT schema files under `crates/plugins/wit/` (kebab-case).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginCategory {
    /// UI-emitting category — plugin authors honor WCAG 2.1 AA + SC 2.3.3 AAA
    /// obligations per `crates/plugins/wit/custom-dashboard.wit` doc-comments.
    CustomDashboard,
    /// Non-UI category — Arrow record-batch transforms.
    DataTransform,
    /// Non-UI category — markdown snapshot emission.
    SnapshotTemplate,
}

impl PluginCategory {
    /// Kebab-case identifier matching the WIT filename + WIT package name.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CustomDashboard => "custom-dashboard",
            Self::DataTransform => "data-transform",
            Self::SnapshotTemplate => "snapshot-template",
        }
    }

    /// All 3 categories in declaration order — useful for parametric tests.
    pub const fn all() -> [Self; 3] {
        [
            Self::CustomDashboard,
            Self::DataTransform,
            Self::SnapshotTemplate,
        ]
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PluginsHeartbeat {
    pub loaded_count: u32,
    pub active_invocations: u32,
}

pub fn heartbeat_payload() -> PluginsHeartbeat {
    PluginsHeartbeat::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_payload_is_callable() {
        let h = heartbeat_payload();
        assert_eq!(h.loaded_count, 0);
        assert_eq!(h.active_invocations, 0);
    }

    #[test]
    fn plugin_category_as_str_matches_wit_filenames() {
        assert_eq!(PluginCategory::CustomDashboard.as_str(), "custom-dashboard");
        assert_eq!(PluginCategory::DataTransform.as_str(), "data-transform");
        assert_eq!(
            PluginCategory::SnapshotTemplate.as_str(),
            "snapshot-template"
        );
    }

    #[test]
    fn plugin_category_all_returns_three_categories() {
        let all = PluginCategory::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&PluginCategory::CustomDashboard));
        assert!(all.contains(&PluginCategory::DataTransform));
        assert!(all.contains(&PluginCategory::SnapshotTemplate));
    }
}
