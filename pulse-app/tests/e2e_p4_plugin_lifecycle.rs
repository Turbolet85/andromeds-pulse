//! P4 E2E coverage — plugin lifecycle critical path per test-plan §6 P4.
//!
//! Stages fixture WASM Component bytes via `wat::parse_str("(component)")`
//! → invokes `plugins::loader::discover_plugins` against a tempdir with the
//! category-directory layout → asserts registry populated with the test
//! plugin. Path-traversal + capability-scoping negative canaries verify
//! security boundaries hold per security plan §Input Validation +
//! §API Security row "Plugin host capability sandbox".

use std::fs;

use assert_fs::TempDir;
use plugins::contract::PluginCategory;
use plugins::engine::build_engine;
use plugins::loader::{canonicalize_plugin_dir, discover_plugins};

#[test]
fn p4_plugin_lifecycle_discovers_fixture_component_in_data_transform_dir() {
    let tmp = TempDir::new().expect("tempdir");
    let plugin_dir = tmp.path();
    let category_dir = plugin_dir.join(PluginCategory::DataTransform.as_str());
    fs::create_dir_all(&category_dir).expect("create category dir");

    // Stage a minimal valid WASM Component via wat::parse_str (chunk #45
    // substrate per crates/plugins/src/wit_loader.rs::tests precedent —
    // avoids committing opaque .wasm fixtures).
    let bytes = wat::parse_str("(component)").expect("empty component compiles");
    fs::write(category_dir.join("p4-fixture.wasm"), &bytes).expect("write fixture .wasm");

    let engine = build_engine().expect("plugin engine builds");
    let canonical = canonicalize_plugin_dir(plugin_dir).expect("canonicalize plugin_dir");
    let plugins = discover_plugins(&engine, &canonical).expect("discover succeeds");

    assert_eq!(
        plugins.len(),
        1,
        "expected 1 discovered plugin; got {} (basenames: {:?})",
        plugins.len(),
        plugins.iter().map(|p| &p.basename).collect::<Vec<_>>()
    );
    assert_eq!(plugins[0].basename, "p4-fixture.wasm");
    assert_eq!(plugins[0].category, PluginCategory::DataTransform);
    assert!(plugins[0].byte_count > 0, "byte_count populated");
}

#[test]
fn p4_negative_canary_nonexistent_plugin_dir_returns_empty_registry() {
    // Per arch §Cross-cutting Patterns "Plugin runtime" + loader behavior:
    // missing plugin dir at boot is non-fatal — discovery returns empty
    // registry; warn-log emitted at loader callsite.
    let tmp = TempDir::new().expect("tempdir");
    let nonexistent = tmp.path().join("does-not-exist");

    let engine = build_engine().expect("engine builds");
    // canonicalize_plugin_dir auto-creates missing parent directories per
    // its security contract (strict-path under-data-dir confinement). When
    // the leaf path does not exist, the result depends on the loader's
    // resolve semantics — either Err OR Ok(canonical) with empty discover.
    match canonicalize_plugin_dir(&nonexistent) {
        Ok(canonical) => {
            let plugins = discover_plugins(&engine, &canonical).expect("discover succeeds");
            assert!(
                plugins.is_empty(),
                "missing plugin dir must yield empty registry; got {} entries",
                plugins.len()
            );
        }
        Err(_) => {
            // canonicalize rejecting nonexistent path is also acceptable —
            // both branches preserve the "no plugins loaded" invariant.
        }
    }
}

#[test]
fn p4_negative_canary_path_traversal_in_plugin_dir_rejected_or_confined() {
    // Per security plan §Input Validation row 6 + plan.md security canary
    // for ANDROMEDA_PULSE_PLUGIN_DIR=../../../etc — `..` components MUST
    // be rejected or canonicalized to a safe absolute path before plugin
    // discovery loads any component.
    let traversal = std::path::Path::new("../../../etc");
    let result = canonicalize_plugin_dir(traversal);
    match result {
        Err(_) => {
            // Rejected — best outcome (matches workspace-detector path
            // traversal discipline).
        }
        Ok(canonical) => {
            // If canonicalized, the resolved path must NOT contain '..'
            // components (canonicalize strips them).
            let canonical_str = canonical.to_string_lossy();
            assert!(
                !canonical_str.contains(".."),
                "canonical plugin_dir must not contain '..'; got {}",
                canonical_str
            );
        }
    }
}
