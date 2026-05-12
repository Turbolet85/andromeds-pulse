//! Per-Store ResourceLimiter substrate for the andromeda-pulse plugin host.
//!
//! Chunk #46 extends the chunk #45 Engine substrate (`build_engine` in
//! `engine.rs`) with per-Store memory + table + instance caps via
//! `wasmtime::ResourceLimiter`. April 2026 advisory cluster
//! (CVE-2026-27572 + 6 others) requires resource bounds orthogonal to
//! capability scoping. Caps attach AT THE STORE LEVEL (per-instantiation)
//! rather than at the shared Engine level — per security plan §API
//! Security row "Plugin host capability sandbox" + §Security Decisions
//! Log 2026-05-02 + arch §Established Decisions [Plugin Runtime].

use wasmtime::{Engine, ResourceLimiter, Store};

use crate::contract::PluginCategory;

/// Default per-Store memory cap. 64 MB per security plan §API Security row
/// "Plugin host capability sandbox" (uniform default across all 3 plugin
/// categories at chunk #46; per-category differentiation deferred to a
/// future chunk if usage profiling exposes need).
pub const DEFAULT_MEMORY_CAP_BYTES: usize = 64 * 1024 * 1024;

/// Default per-Store table-elements cap. Conservative bound; wasmtime's
/// own default is 10_000 — the plugin sandbox tightens further.
pub const DEFAULT_TABLE_ELEMENTS_CAP: usize = 1000;

/// Default per-Store instances cap. Conservative bound; wasmtime's own
/// default is 10_000 — the plugin sandbox tightens further.
pub const DEFAULT_INSTANCES_CAP: usize = 100;

// Compile-time sanity bounds on the cap defaults. Any future edit that
// would zero them out OR raise them above documented upper bounds fails
// the build instead of silently regressing the sandbox posture. Module
// scope so the check runs on every build, mirroring `engine.rs:42-45`
// precedent for MAX_WASM_HTTP_FIELDS_SIZE_BYTES.
const _: () = {
    assert!(DEFAULT_MEMORY_CAP_BYTES > 0);
    assert!(DEFAULT_MEMORY_CAP_BYTES <= 256 * 1024 * 1024);
    assert!(DEFAULT_TABLE_ELEMENTS_CAP > 0);
    assert!(DEFAULT_TABLE_ELEMENTS_CAP <= 100_000);
    assert!(DEFAULT_INSTANCES_CAP > 0);
    assert!(DEFAULT_INSTANCES_CAP <= 1000);
};

/// Per-Store resource caps for the plugin sandbox. Implements
/// `wasmtime::ResourceLimiter` so wasmtime consults these caps before
/// growing memory / tables and reports the per-Store ceiling on instance
/// / table / memory counts.
#[derive(Debug, Clone, Copy)]
pub struct ResourceLimiterState {
    pub mem_max: usize,
    pub tables_max: usize,
    pub instances_max: usize,
}

impl Default for ResourceLimiterState {
    fn default() -> Self {
        Self {
            mem_max: DEFAULT_MEMORY_CAP_BYTES,
            tables_max: DEFAULT_TABLE_ELEMENTS_CAP,
            instances_max: DEFAULT_INSTANCES_CAP,
        }
    }
}

impl ResourceLimiterState {
    /// Resource cap defaults for a plugin category. Today: uniform
    /// 64 MB / 1000 tables / 100 instances across all 3 categories.
    /// Per-category differentiation deferred to a future chunk if usage
    /// profiling exposes need (e.g., custom-dashboard could benefit
    /// from larger memory for UI rendering payloads; data-transform
    /// stays minimal). The function exists at chunk #46 to anchor the
    /// per-category dispatch site even when all branches return the
    /// same value today.
    pub fn for_category(_category: PluginCategory) -> Self {
        Self::default()
    }
}

impl ResourceLimiter for ResourceLimiterState {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        if desired > self.mem_max {
            // Per obs plan §1 Telemetry Vector 3 + §11 Anti-Patterns:
            // basename-only / cap-only — never the requesting plugin's
            // full path. Fields stay within the `plugin` AllowList
            // registry's 5 allowed (`error_msg` carries cap details).
            tracing::error!(
                target: "plugin.resource_limit.rejection",
                error_msg = %format!(
                    "memory grow denied: requested {} bytes, cap {} bytes",
                    desired, self.mem_max
                ),
            );
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn table_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        if desired > self.tables_max {
            tracing::error!(
                target: "plugin.resource_limit.rejection",
                error_msg = %format!(
                    "table grow denied: requested {} elements, cap {} elements",
                    desired, self.tables_max
                ),
            );
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn instances(&self) -> usize {
        self.instances_max
    }

    fn tables(&self) -> usize {
        self.tables_max
    }

    fn memories(&self) -> usize {
        // Cap memory instances per Store at the same conservative bound as
        // wasmtime instances (one memory per plugin is typical; symmetric
        // bound prevents accidental memory-instance explosion).
        self.instances_max
    }
}

/// Construct a sandboxed `Store` for a plugin category. The Store data
/// is a `ResourceLimiterState` configured per category defaults; the
/// limiter is attached so wasmtime consults the caps on grow operations
/// and on instance/table/memory provisioning.
pub fn store_for_category(
    engine: &Engine,
    category: PluginCategory,
) -> Store<ResourceLimiterState> {
    let mut store = Store::new(engine, ResourceLimiterState::for_category(category));
    store.limiter(|state| state as &mut dyn ResourceLimiter);
    store
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::build_engine;
    use rstest::rstest;

    #[test]
    fn default_uses_documented_caps() {
        let state = ResourceLimiterState::default();
        assert_eq!(state.mem_max, DEFAULT_MEMORY_CAP_BYTES);
        assert_eq!(state.tables_max, DEFAULT_TABLE_ELEMENTS_CAP);
        assert_eq!(state.instances_max, DEFAULT_INSTANCES_CAP);
    }

    #[test]
    fn memory_growing_allows_grow_at_cap() {
        let mut state = ResourceLimiterState::default();
        let result = state
            .memory_growing(0, DEFAULT_MEMORY_CAP_BYTES, None)
            .expect("memory_growing returns Ok");
        assert!(result, "growing exactly to cap must be allowed");
    }

    #[test]
    fn memory_growing_returns_false_past_cap() {
        let mut state = ResourceLimiterState::default();
        let result = state
            .memory_growing(0, DEFAULT_MEMORY_CAP_BYTES + 1, None)
            .expect("memory_growing returns Ok");
        assert!(!result, "growing past cap must be denied");
    }

    #[test]
    fn table_growing_allows_grow_at_cap() {
        let mut state = ResourceLimiterState::default();
        let result = state
            .table_growing(0, DEFAULT_TABLE_ELEMENTS_CAP, None)
            .expect("table_growing returns Ok");
        assert!(result, "growing exactly to cap must be allowed");
    }

    #[test]
    fn table_growing_returns_false_past_cap() {
        let mut state = ResourceLimiterState::default();
        let result = state
            .table_growing(0, DEFAULT_TABLE_ELEMENTS_CAP + 1, None)
            .expect("table_growing returns Ok");
        assert!(!result, "growing past cap must be denied");
    }

    #[test]
    fn instances_returns_configured_cap() {
        let state = ResourceLimiterState::default();
        assert_eq!(state.instances(), DEFAULT_INSTANCES_CAP);
    }

    #[test]
    fn tables_returns_configured_cap() {
        let state = ResourceLimiterState::default();
        assert_eq!(state.tables(), DEFAULT_TABLE_ELEMENTS_CAP);
    }

    #[test]
    fn memories_returns_instances_cap_for_symmetry() {
        let state = ResourceLimiterState::default();
        assert_eq!(state.memories(), DEFAULT_INSTANCES_CAP);
    }

    #[rstest]
    #[case(PluginCategory::CustomDashboard)]
    #[case(PluginCategory::DataTransform)]
    #[case(PluginCategory::SnapshotTemplate)]
    fn for_category_returns_uniform_defaults(#[case] category: PluginCategory) {
        let state = ResourceLimiterState::for_category(category);
        assert_eq!(
            state.mem_max, DEFAULT_MEMORY_CAP_BYTES,
            "all categories share the 64 MB default at chunk #46"
        );
        assert_eq!(state.tables_max, DEFAULT_TABLE_ELEMENTS_CAP);
        assert_eq!(state.instances_max, DEFAULT_INSTANCES_CAP);
    }

    #[rstest]
    #[case(PluginCategory::CustomDashboard)]
    #[case(PluginCategory::DataTransform)]
    #[case(PluginCategory::SnapshotTemplate)]
    fn store_for_category_has_documented_caps(#[case] category: PluginCategory) {
        let engine = build_engine().expect("engine must build");
        let store = store_for_category(&engine, category);
        let data = store.data();
        assert_eq!(data.mem_max, DEFAULT_MEMORY_CAP_BYTES);
        assert_eq!(data.tables_max, DEFAULT_TABLE_ELEMENTS_CAP);
        assert_eq!(data.instances_max, DEFAULT_INSTANCES_CAP);
    }
}
