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

    /// Per-Store `wasmtime::ResourceLimiter` denied a resource grow
    /// request. `plugin_id` carries the requesting plugin identity
    /// (typically a category identifier at chunk #46; basename of the
    /// loaded WASM at chunk #47+). `limit_kind` is a kebab-case marker
    /// from `{"memory-bytes", "tables", "instances"}` for serde-friendly
    /// Display. Chunk #46 emits the rejection via `tracing::error!` at
    /// `plugin.resource_limit.rejection`; this variant exists as the
    /// typed path for chunk #47+ IPC consumers.
    #[error(
        "resource limit exceeded for plugin `{plugin_id}`: {limit_kind} requested {requested}, cap {configured_cap}"
    )]
    ResourceLimitExceeded {
        plugin_id: String,
        limit_kind: String,
        requested: u64,
        configured_cap: u64,
    },

    /// A WIT-declared capability check rejected an attempted invocation.
    /// Today's 3 WIT files declare zero host imports so this variant is
    /// unreached at chunk #46 — it lays down the rejection path so future
    /// chunks introducing host imports can populate without re-amending
    /// the enum. `plugin_id` + `capability_name` + `reason` are
    /// sanitized one-liners.
    #[error(
        "capability rejected for plugin `{plugin_id}` capability `{capability_name}`: {reason}"
    )]
    CapabilityRejected {
        plugin_id: String,
        capability_name: String,
        reason: String,
    },
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

    #[test]
    fn resource_limit_exceeded_displays_all_fields() {
        let e = Error::ResourceLimitExceeded {
            plugin_id: "data-transform".to_string(),
            limit_kind: "memory-bytes".to_string(),
            requested: 67_108_865,
            configured_cap: 67_108_864,
        };
        let msg = e.to_string();
        assert!(
            msg.contains("data-transform"),
            "message must carry plugin_id"
        );
        assert!(
            msg.contains("memory-bytes"),
            "message must carry limit_kind"
        );
        assert!(msg.contains("67108865"), "message must carry requested");
        assert!(
            msg.contains("67108864"),
            "message must carry configured_cap"
        );
    }

    #[test]
    fn capability_rejected_displays_all_fields() {
        let e = Error::CapabilityRejected {
            plugin_id: "custom-dashboard".to_string(),
            capability_name: "host:logging/log".to_string(),
            reason: "not declared in WIT".to_string(),
        };
        let msg = e.to_string();
        assert!(msg.contains("custom-dashboard"));
        assert!(msg.contains("host:logging/log"));
        assert!(msg.contains("not declared in WIT"));
    }
}
