//! Per-category WIT-driven Linker construction for the andromeda-pulse
//! plugin host substrate.
//!
//! Chunk #46 lays down the per-category dispatch site. All 3 WIT files
//! today (`custom-dashboard.wit` / `data-transform.wit` /
//! `snapshot-template.wit`) declare ZERO host imports — the function
//! returns an empty `Linker` for each category. The discipline anchor:
//! capability differentiation MUST flow through the WIT contract per
//! arch §Established Decisions [Plugin Runtime] + §Cross-cutting
//! Patterns "Capability-scoped extensibility" + security plan §API
//! Security row "Plugin host capability sandbox" + §Code Patterns ban
//! on `wasmtime::Linker` exposing host functions outside WIT.
//!
//! Future chunks introducing category-specific host imports (e.g., a
//! `host:logging/log` import declared in a category's WIT) MUST extend
//! the per-category match arms here — NEVER add ad-hoc `Linker::define`
//! / `Linker::func_wrap` calls for functions not declared in the
//! category's WIT.

use wasmtime::Engine;
use wasmtime::component::Linker;

use crate::contract::PluginCategory;
use crate::sandbox::ResourceLimiterState;

/// Construct an empty `Linker<ResourceLimiterState>` for a plugin
/// category. Today: all 3 categories return an empty Linker because
/// none of the 3 WIT files declare host imports (`viewport` and
/// `a11y-state` in `custom-dashboard.wit` are records passed as
/// call-time arguments to the `render` export, not host imports).
/// The per-category match arms are the dispatch site for future
/// host-import additions.
pub fn linker_for_category(
    engine: &Engine,
    category: PluginCategory,
) -> Linker<ResourceLimiterState> {
    match category {
        // custom-dashboard.wit declares zero host imports.
        PluginCategory::CustomDashboard => Linker::new(engine),
        // data-transform.wit declares zero host imports (pure Arrow IPC
        // bytes in/out).
        PluginCategory::DataTransform => Linker::new(engine),
        // snapshot-template.wit declares zero host imports (pure JSON
        // in / markdown out).
        PluginCategory::SnapshotTemplate => Linker::new(engine),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::build_engine;
    use rstest::rstest;

    /// Per-category dispatch site is callable and returns a typed
    /// `Linker<ResourceLimiterState>` for each of the 3 categories.
    /// Today all 3 are empty by construction; the functional check that
    /// the empty Linker rejects undeclared host imports lives at
    /// `wit_loader::tests::sandbox_with_undeclared_import_fails_at_link_time`
    /// (the integration site exercising sandbox + capability stacks
    /// together).
    #[rstest]
    #[case(PluginCategory::CustomDashboard)]
    #[case(PluginCategory::DataTransform)]
    #[case(PluginCategory::SnapshotTemplate)]
    fn linker_for_category_returns_typed_linker(#[case] category: PluginCategory) {
        let engine = build_engine().expect("engine must build");
        let _linker: Linker<ResourceLimiterState> = linker_for_category(&engine, category);
    }
}
