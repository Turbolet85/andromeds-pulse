//! WIT-defined Component loading + linker helpers for the andromeda-pulse
//! plugin host substrate.
//!
//! Chunk #45 (substrate-only) exposes:
//! - `load_component(engine, bytes)` — wraps `wasmtime::component::Component::from_binary`
//!   with size-cap check + sanitized error mapping.
//! - `linker_for(engine)` — constructs an empty `ComponentLinker` ready for
//!   per-category host-import wiring in subsequent chunks (#46 sandbox, #47 loader).
//!   This file's linker construction MUST NOT call `func_wrap` / `define_*`
//!   outside what each WIT contract declares — capability-scoped sandboxing
//!   is the architectural trust boundary (security plan §Anti-Patterns Code
//!   Patterns row 6).
//!
//! WIT schemas live at `crates/plugins/wit/{custom-dashboard,data-transform,
//! snapshot-template}.wit` (kebab-case per arch §Conventions File naming).
//! They are documentation + future-use scaffolding at this chunk; actual
//! capability enforcement at Component instantiation is via the Linker's
//! declared host imports — guests importing functions NOT declared fail at
//! link time, satisfying the negative-test acceptance criterion.

use std::time::Instant;

use tracing::instrument;
use wasmtime::Engine;
use wasmtime::component::{Component, Linker};

use crate::contract::{Error, PluginCategory};

/// Max bytes accepted for a single Component WASM payload. Matches the
/// Arrow IPC 8 MB bound used elsewhere in the host (security plan §API
/// Security Plugin host); enforces a substrate-level size cap before
/// the bytes reach `wasmtime::component::Component::from_binary`.
pub const MAX_COMPONENT_BYTES: usize = 8 * 1024 * 1024;

/// Load a WASM Component Model binary into a `Component` value bound to
/// the provided `Engine`. Size-capped at `MAX_COMPONENT_BYTES`; emits a
/// `plugin.load.request` span with `wasm_size_bytes` + `duration_ms` per
/// obs-plan §4 Scenario P4 attributes.
///
/// `plugin_id` is the plugin's category identifier ("custom-dashboard" etc.);
/// it is recorded as a span field for diagnostic correlation. Note: per
/// security plan §Logging Vector 3, full plugin file paths are NEVER
/// recorded — callers MUST pass a basename-only identifier (typically the
/// `PluginCategory::as_str()` value, not a filesystem path).
#[instrument(skip(engine, bytes), fields(
    plugin_id = %plugin_id,
    wasm_size_bytes = bytes.len(),
    duration_ms = tracing::field::Empty,
))]
pub fn load_component(engine: &Engine, plugin_id: &str, bytes: &[u8]) -> Result<Component, Error> {
    let started = Instant::now();
    let span = tracing::Span::current();

    if bytes.len() > MAX_COMPONENT_BYTES {
        return Err(Error::ComponentInstantiate {
            plugin_id: plugin_id.to_string(),
            reason: format!(
                "component bytes {} exceed cap {} (MAX_COMPONENT_BYTES)",
                bytes.len(),
                MAX_COMPONENT_BYTES
            ),
        });
    }

    let component =
        Component::from_binary(engine, bytes).map_err(|e| Error::ComponentInstantiate {
            plugin_id: plugin_id.to_string(),
            reason: crate::engine::sanitize_wasmtime_error(&e.to_string()),
        })?;

    let elapsed = started.elapsed().as_millis() as u64;
    span.record("duration_ms", elapsed);

    Ok(component)
}

/// Construct an empty `ComponentLinker<T>` bound to the provided `Engine`.
/// Returns a linker with NO host imports defined — substrate posture for
/// chunk #45.
///
/// Chunks #46 (sandbox / ResourceLimiter) and #47 (loader / IPC routers)
/// MAY extend the linker per-category to define the host imports declared
/// in each WIT contract. Any `func_wrap` / `define_*` call MUST correspond
/// to an explicit WIT-declared import for the targeted category.
///
/// `T` is the linker's per-Store data type (typically `()` for empty-state
/// linkers; subsequent chunks parameterize on `WasiCtx` or similar).
pub fn linker_for<T: 'static>(engine: &Engine) -> Linker<T> {
    Linker::new(engine)
}

/// Convenience: load a Component matching one of the 3 declared plugin
/// categories. Identical to [`load_component`] but accepts a typed
/// `PluginCategory` rather than a free-form string, eliminating typos in
/// the category identifier.
pub fn load_component_for_category(
    engine: &Engine,
    category: PluginCategory,
    bytes: &[u8],
) -> Result<Component, Error> {
    load_component(engine, category.as_str(), bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::build_engine;
    use rstest::rstest;

    /// Smallest valid Component Model binary (empty component) — used to
    /// verify Component::from_binary plumbing without requiring fixture
    /// WASM commits at chunk #45 substrate level. Generated at test
    /// runtime via the `wat` crate (workspace dev-dep).
    fn empty_component_bytes() -> Vec<u8> {
        // The smallest valid Component is `(component)` — an empty
        // component declaration. `wat::parse_str` compiles WAT text to
        // WASM bytes including Component Model preamble.
        wat::parse_str("(component)").expect("empty component WAT must parse")
    }

    #[test]
    fn load_component_succeeds_on_minimal_empty_component() {
        let engine = build_engine().expect("engine must build");
        let bytes = empty_component_bytes();
        let result = load_component(&engine, "custom-dashboard", &bytes);
        assert!(
            result.is_ok(),
            "load_component must succeed on minimal empty component: {:?}",
            result.err()
        );
    }

    #[test]
    fn load_component_rejects_oversized_payload() {
        let engine = build_engine().expect("engine must build");
        let oversized = vec![0u8; MAX_COMPONENT_BYTES + 1];
        let result = load_component(&engine, "data-transform", &oversized);
        match result {
            Err(Error::ComponentInstantiate { plugin_id, reason }) => {
                assert_eq!(plugin_id, "data-transform");
                assert!(
                    reason.contains("exceed cap"),
                    "reason must mention size cap; got: {reason}"
                );
            }
            Err(other) => panic!("expected ComponentInstantiate error; got {other:?}"),
            Ok(_) => panic!("expected ComponentInstantiate error; got Ok(Component)"),
        }
    }

    #[test]
    fn load_component_rejects_invalid_bytes() {
        let engine = build_engine().expect("engine must build");
        // Definitely not valid WASM; wasmtime should reject at parse time.
        let invalid = b"not a wasm component";
        let result = load_component(&engine, "snapshot-template", invalid);
        match result {
            Err(Error::ComponentInstantiate { plugin_id, .. }) => {
                assert_eq!(plugin_id, "snapshot-template");
            }
            Err(other) => panic!("expected ComponentInstantiate error; got {other:?}"),
            Ok(_) => panic!("expected ComponentInstantiate error; got Ok(Component)"),
        }
    }

    #[rstest]
    #[case(PluginCategory::CustomDashboard)]
    #[case(PluginCategory::DataTransform)]
    #[case(PluginCategory::SnapshotTemplate)]
    fn load_component_for_category_routes_to_correct_id(#[case] category: PluginCategory) {
        let engine = build_engine().expect("engine must build");
        let bytes = empty_component_bytes();
        let result = load_component_for_category(&engine, category, &bytes);
        assert!(
            result.is_ok(),
            "load_component_for_category must succeed for {}: {:?}",
            category.as_str(),
            result.err()
        );
    }

    #[test]
    fn linker_for_returns_empty_linker() {
        let engine = build_engine().expect("engine must build");
        let _linker: Linker<()> = linker_for(&engine);
        // No host imports defined; substrate posture sealed. Subsequent
        // chunks may extend the linker per WIT-declared imports.
    }

    /// Negative-test: a Component importing a host function NOT declared
    /// via the linker MUST fail at instantiate/link time, validating
    /// capability-scoping by construction (per tests acceptance criterion
    /// + security plan §Anti-Patterns Code Patterns row 6).
    ///
    /// This test creates a WAT-source Component that imports a host
    /// function `host:undeclared/foo.bar`, attempts to instantiate it
    /// against an empty linker, and asserts the instantiation fails. The
    /// failure surfaces as a `wasmtime` link-time error which we map to
    /// our own `Error::ComponentInstantiate` via the linker call site.
    #[test]
    fn component_with_undeclared_import_fails_to_instantiate() {
        let engine = build_engine().expect("engine must build");
        // Component declares an import on a host function it expects;
        // the empty linker provides NO host imports → instantiation fails.
        let wat = r#"
            (component
              (import "host:undeclared/foo" (instance $h
                (export "bar" (func))))
            )
        "#;
        let bytes = wat::parse_str(wat).expect("undeclared-import WAT must parse");

        // Load is OK (parsing succeeds — the import is declared in the
        // WIT/Component metadata but no host has been linked yet).
        let component = load_component(&engine, "negative-canary", &bytes)
            .expect("Component bytes parse cleanly; only instantiation should fail");

        // Instantiate against an empty linker: link-time error expected.
        let linker: Linker<()> = linker_for(&engine);
        let mut store = wasmtime::Store::new(&engine, ());
        let result = linker.instantiate(&mut store, &component);
        assert!(
            result.is_err(),
            "instantiating a Component with an undeclared host import \
             against an empty linker MUST fail at link time \
             (capability-scoping by construction)"
        );
    }
}
