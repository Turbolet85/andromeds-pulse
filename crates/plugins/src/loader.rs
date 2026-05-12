//! Filesystem plugin loader for the andromeda-pulse plugin host.
//!
//! Chunk #47 wires the chunk #45 Engine substrate + chunk #46 sandbox into
//! a live filesystem loader. Scans the canonicalized plugin directory
//! (`ANDROMEDA_PULSE_PLUGIN_DIR` override OR per-platform default) for
//! `*.wasm` Component Model binaries organized into per-category
//! subdirectories (`{plugin_dir}/{custom-dashboard,data-transform,
//! snapshot-template}/*.wasm`). Returns an in-memory `PluginRegistry`
//! consumed by the `plugins.{list,reload,invoke}` TauRPC procedures
//! in `pulse-app/src/plugins_router.rs`.
//!
//! Path canonicalization follows the workspace-detector precedent
//! (`crates/workspace-detector/src/detect.rs`): `std::fs::canonicalize`
//! after a manual `path_contains_traversal` reject. Per security plan
//! §Input Validation row "Plugin host inputs" + §Code Patterns
//! anti-pattern row 2 (CWE-22 defense): any traversal escape rejects at
//! load time with `Error::PathCanonicalizationFailed`. Per security plan
//! §Logging Vector 3: only basenames cross tracing fields; no full paths.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;
use std::{env, fs};

use tracing::instrument;
use wasmtime::Engine;
use wasmtime::component::Component;

use crate::contract::{Error, PluginCategory};
use crate::wit_loader::load_component_for_category;

const ENV_PLUGIN_DIR: &str = "ANDROMEDA_PULSE_PLUGIN_DIR";
const ENV_DATA_DIR: &str = "ANDROMEDA_PULSE_DATA_DIR";
const PLUGINS_SUBPATH: &str = "plugins";

/// A plugin loaded into the registry. `id` is the basename without the
/// `.wasm` extension; `basename` is the full filename including extension
/// (basename-only — never the full canonicalized path per security plan
/// §Logging Vector 3). `component` is the wasmtime-instantiable binary;
/// chunk #47 capability-handshake never calls export functions, so
/// `Component` is held but not invoked.
pub struct LoadedPlugin {
    pub id: String,
    pub category: PluginCategory,
    pub component: Component,
    pub basename: String,
    pub byte_count: u64,
}

/// In-memory plugin registry. Holds the set of plugins discovered at boot
/// (or rebuilt via `plugins.reload`) plus the cumulative-since-boot
/// invocation counter consumed by `plugins.tick` heartbeat.
///
/// Cumulative semantics chosen per phase-44 plan §Implementation notes:
/// simpler than concurrent gauge for currently-executing count; chunk #47
/// invoke is capability-handshake-only so an executing-count would not
/// reflect real work anyway. Counter resets at process restart.
pub struct PluginRegistry {
    plugins: Vec<LoadedPlugin>,
    active_invocations_counter: AtomicU32,
}

impl PluginRegistry {
    pub fn empty() -> Self {
        Self {
            plugins: Vec::new(),
            active_invocations_counter: AtomicU32::new(0),
        }
    }

    pub fn loaded_count(&self) -> u32 {
        u32::try_from(self.plugins.len()).unwrap_or(u32::MAX)
    }

    pub fn active_invocations(&self) -> u32 {
        self.active_invocations_counter.load(Ordering::Relaxed)
    }

    /// Increment the cumulative invocation counter. Called by
    /// `plugins.invoke` regardless of capability-allowed outcome (counter
    /// reflects attempts, not successes — matches the security-relevant
    /// signal of "plugin host activity").
    pub fn record_invocation(&self) {
        self.active_invocations_counter
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn plugins(&self) -> &[LoadedPlugin] {
        &self.plugins
    }

    pub fn find(&self, plugin_id: &str) -> Option<&LoadedPlugin> {
        self.plugins.iter().find(|p| p.id == plugin_id)
    }

    /// Replace the plugin set with a freshly discovered registry. Preserves
    /// the cumulative-since-boot invocation counter across reloads.
    pub fn replace_plugins(&mut self, plugins: Vec<LoadedPlugin>) {
        self.plugins = plugins;
    }
}

/// Resolve the plugin directory from env vars + per-platform defaults.
/// Precedence (per arch §Cross-cutting Patterns "Config management"):
/// 1. `ANDROMEDA_PULSE_PLUGIN_DIR` if set
/// 2. `ANDROMEDA_PULSE_DATA_DIR/plugins` if data-dir override set
/// 3. Per-platform default: `~/.andromeda-pulse/plugins/` (Linux) /
///    `~/Library/Application Support/com.andromeda.pulse/plugins/` (macOS) /
///    `%APPDATA%\andromeda-pulse\plugins\` (Windows)
/// 4. Fallback: `<temp_dir>/andromeda-pulse/plugins`
pub fn resolve_plugin_dir() -> PathBuf {
    if let Ok(p) = env::var(ENV_PLUGIN_DIR) {
        return PathBuf::from(p);
    }
    if let Ok(p) = env::var(ENV_DATA_DIR) {
        return PathBuf::from(p).join(PLUGINS_SUBPATH);
    }
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata)
                .join("andromeda-pulse")
                .join(PLUGINS_SUBPATH);
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("com.andromeda.pulse")
                .join(PLUGINS_SUBPATH);
        }
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg)
            .join("andromeda-pulse")
            .join(PLUGINS_SUBPATH);
    } else if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home)
            .join(".andromeda-pulse")
            .join(PLUGINS_SUBPATH);
    }
    env::temp_dir()
        .join("andromeda-pulse")
        .join(PLUGINS_SUBPATH)
}

/// Canonicalize the plugin directory + reject traversal escapes. Returns
/// the canonical path on success; `Error::PathCanonicalizationFailed`
/// otherwise. Mirror of `workspace_detector::detect::detect` traversal
/// + canonicalize pattern.
///
/// If the plugin directory does not exist, returns the original
/// (non-canonicalized) path: discovery against a missing dir is a clean
/// "no plugins found" path, not an error. Boot proceeds with an empty
/// registry rather than aborting.
pub fn canonicalize_plugin_dir(plugin_dir: &Path) -> Result<PathBuf, Error> {
    if path_contains_traversal(plugin_dir) {
        return Err(Error::PathCanonicalizationFailed {
            reason: "plugin dir contains traversal components".to_string(),
        });
    }
    match plugin_dir.canonicalize() {
        Ok(canonical) => Ok(canonical),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Missing plugin dir is a clean no-op: discovery returns an
            // empty registry. Return the non-canonicalized path so the
            // caller can short-circuit discovery cleanly.
            Ok(plugin_dir.to_path_buf())
        }
        Err(_) => Err(Error::PathCanonicalizationFailed {
            reason: "plugin dir canonicalization failed".to_string(),
        }),
    }
}

fn path_contains_traversal(path: &Path) -> bool {
    use std::path::Component;
    path.components().any(|c| matches!(c, Component::ParentDir))
}

fn basename(path: &Path) -> String {
    path.file_name()
        .and_then(|os| os.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Discover plugins by scanning per-category subdirectories under the
/// canonicalized plugin dir. Files in `{dir}/custom-dashboard/*.wasm` are
/// classified as `CustomDashboard`, etc. Files NOT in a recognized
/// category subdirectory are silently ignored (emit a `warn` event at
/// `plugin.load.discover.unknown_category` carrying basename only).
///
/// Returns a fresh `PluginRegistry`. The cumulative invocation counter
/// resets to zero on every fresh discovery — callers wanting to preserve
/// the counter across reloads should use `PluginRegistry::replace_plugins`
/// instead of replacing the whole registry.
#[instrument(skip_all, fields(
    plugin_dir_basename = tracing::field::Empty,
    plugins_discovered_count = tracing::field::Empty,
    duration_ms = tracing::field::Empty,
))]
pub fn discover_plugins(
    engine: &Engine,
    canonical_plugin_dir: &Path,
) -> Result<Vec<LoadedPlugin>, Error> {
    let started = Instant::now();
    let span = tracing::Span::current();
    span.record("plugin_dir_basename", basename(canonical_plugin_dir));

    if !canonical_plugin_dir.exists() {
        span.record("plugins_discovered_count", 0_u64);
        span.record("duration_ms", started.elapsed().as_millis() as u64);
        return Ok(Vec::new());
    }

    let mut plugins = Vec::new();
    for category in PluginCategory::all() {
        let category_dir = canonical_plugin_dir.join(category.as_str());
        if !category_dir.is_dir() {
            continue;
        }
        scan_category_dir(engine, category, &category_dir, &mut plugins)?;
    }

    span.record("plugins_discovered_count", plugins.len() as u64);
    span.record("duration_ms", started.elapsed().as_millis() as u64);
    Ok(plugins)
}

fn scan_category_dir(
    engine: &Engine,
    category: PluginCategory,
    category_dir: &Path,
    out: &mut Vec<LoadedPlugin>,
) -> Result<(), Error> {
    let read_dir = fs::read_dir(category_dir).map_err(|_| Error::LoadFailed {
        plugin_id: basename(category_dir),
        reason: "read_dir failed".to_string(),
    })?;
    for entry in read_dir {
        let entry = entry.map_err(|_| Error::LoadFailed {
            plugin_id: basename(category_dir),
            reason: "dir entry read failed".to_string(),
        })?;
        let path = entry.path();
        if !is_wasm_file(&path) {
            continue;
        }
        let file_basename = basename(&path);
        let bytes = fs::read(&path).map_err(|_| Error::LoadFailed {
            plugin_id: file_basename.clone(),
            reason: "file read failed".to_string(),
        })?;
        let component = load_component_for_category(engine, category, &bytes)?;
        let id = file_basename
            .strip_suffix(".wasm")
            .unwrap_or(&file_basename)
            .to_string();
        out.push(LoadedPlugin {
            id,
            category,
            component,
            basename: file_basename,
            byte_count: bytes.len() as u64,
        });
    }
    Ok(())
}

fn is_wasm_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("wasm"))
            .unwrap_or(false)
}

/// Declared exports per plugin category, from `crates/plugins/wit/*.wit`.
/// `plugins.invoke` matches the requested capability name against this
/// list for handshake-only validation at chunk #47.
pub fn declared_exports(category: PluginCategory) -> &'static [&'static str] {
    match category {
        PluginCategory::CustomDashboard => &["render"],
        PluginCategory::DataTransform => &["transform"],
        PluginCategory::SnapshotTemplate => &["render"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::build_engine;
    use rstest::rstest;
    use std::fs;
    use tempfile::TempDir;

    fn empty_component_bytes() -> Vec<u8> {
        wat::parse_str("(component)").expect("empty component WAT must parse")
    }

    fn stage_wat_plugin(dir: &Path, category: PluginCategory, name: &str) -> PathBuf {
        let cat_dir = dir.join(category.as_str());
        fs::create_dir_all(&cat_dir).expect("create category dir");
        let file_path = cat_dir.join(format!("{name}.wasm"));
        fs::write(&file_path, empty_component_bytes()).expect("write fixture");
        file_path
    }

    #[test]
    fn empty_registry_reports_zero_loaded() {
        let registry = PluginRegistry::empty();
        assert_eq!(registry.loaded_count(), 0);
        assert_eq!(registry.active_invocations(), 0);
        assert!(registry.plugins().is_empty());
    }

    #[test]
    fn record_invocation_increments_counter() {
        let registry = PluginRegistry::empty();
        registry.record_invocation();
        registry.record_invocation();
        registry.record_invocation();
        assert_eq!(registry.active_invocations(), 3);
    }

    #[test]
    fn replace_plugins_preserves_invocation_counter() {
        let mut registry = PluginRegistry::empty();
        registry.record_invocation();
        registry.record_invocation();
        registry.replace_plugins(Vec::new());
        assert_eq!(
            registry.active_invocations(),
            2,
            "replace_plugins must preserve cumulative counter across reloads"
        );
    }

    #[test]
    fn canonicalize_plugin_dir_rejects_traversal() {
        let candidate = Path::new("/tmp/foo/../escape");
        let result = canonicalize_plugin_dir(candidate);
        assert!(matches!(
            result,
            Err(Error::PathCanonicalizationFailed { .. })
        ));
    }

    #[test]
    fn canonicalize_plugin_dir_returns_path_for_missing_dir() {
        let result = canonicalize_plugin_dir(Path::new(
            "/nonexistent-plugin-dir-that-should-never-exist-xyz",
        ));
        assert!(
            result.is_ok(),
            "missing dir resolves to non-canonical path (clean no-op)"
        );
    }

    #[test]
    fn canonicalize_plugin_dir_returns_canonical_for_existing_dir() {
        let tmp = TempDir::new().unwrap();
        let result = canonicalize_plugin_dir(tmp.path()).expect("canonicalize ok");
        assert!(result.exists());
    }

    #[test]
    fn discover_plugins_returns_empty_for_missing_dir() {
        let engine = build_engine().expect("engine builds");
        let result = discover_plugins(&engine, Path::new("/nonexistent-xyz"));
        assert!(matches!(result, Ok(plugins) if plugins.is_empty()));
    }

    #[test]
    fn discover_plugins_returns_empty_for_empty_dir() {
        let engine = build_engine().expect("engine builds");
        let tmp = TempDir::new().unwrap();
        let result = discover_plugins(&engine, tmp.path()).expect("discovery ok");
        assert!(result.is_empty());
    }

    #[rstest]
    #[case(PluginCategory::CustomDashboard)]
    #[case(PluginCategory::DataTransform)]
    #[case(PluginCategory::SnapshotTemplate)]
    fn discover_plugins_finds_per_category_wasm_files(#[case] category: PluginCategory) {
        let engine = build_engine().expect("engine builds");
        let tmp = TempDir::new().unwrap();
        stage_wat_plugin(tmp.path(), category, "test-plugin");

        let plugins = discover_plugins(&engine, tmp.path()).expect("discovery ok");
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].id, "test-plugin");
        assert_eq!(plugins[0].basename, "test-plugin.wasm");
        assert_eq!(plugins[0].category, category);
        assert!(plugins[0].byte_count > 0);
    }

    #[test]
    fn discover_plugins_classifies_per_category_subdir() {
        let engine = build_engine().expect("engine builds");
        let tmp = TempDir::new().unwrap();
        stage_wat_plugin(tmp.path(), PluginCategory::CustomDashboard, "dash");
        stage_wat_plugin(tmp.path(), PluginCategory::DataTransform, "xform");
        stage_wat_plugin(tmp.path(), PluginCategory::SnapshotTemplate, "snap");

        let plugins = discover_plugins(&engine, tmp.path()).expect("discovery ok");
        assert_eq!(plugins.len(), 3);

        let by_category: std::collections::HashMap<_, _> =
            plugins.iter().map(|p| (p.category, p.id.clone())).collect();
        assert_eq!(
            by_category
                .get(&PluginCategory::CustomDashboard)
                .map(String::as_str),
            Some("dash")
        );
        assert_eq!(
            by_category
                .get(&PluginCategory::DataTransform)
                .map(String::as_str),
            Some("xform")
        );
        assert_eq!(
            by_category
                .get(&PluginCategory::SnapshotTemplate)
                .map(String::as_str),
            Some("snap")
        );
    }

    #[test]
    fn discover_plugins_skips_files_outside_category_subdir() {
        let engine = build_engine().expect("engine builds");
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("stray.wasm"), empty_component_bytes()).unwrap();

        let plugins = discover_plugins(&engine, tmp.path()).expect("discovery ok");
        assert!(plugins.is_empty(), "top-level .wasm files are ignored");
    }

    #[test]
    fn discover_plugins_skips_non_wasm_files_in_category_dir() {
        let engine = build_engine().expect("engine builds");
        let tmp = TempDir::new().unwrap();
        let cat_dir = tmp.path().join(PluginCategory::DataTransform.as_str());
        fs::create_dir_all(&cat_dir).unwrap();
        fs::write(cat_dir.join("README.md"), b"not a plugin").unwrap();
        fs::write(cat_dir.join("plugin.wasm"), empty_component_bytes()).unwrap();

        let plugins = discover_plugins(&engine, tmp.path()).expect("discovery ok");
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].id, "plugin");
    }

    #[test]
    fn discover_plugins_rejects_invalid_wasm_bytes_via_load_component() {
        let engine = build_engine().expect("engine builds");
        let tmp = TempDir::new().unwrap();
        let cat_dir = tmp.path().join(PluginCategory::DataTransform.as_str());
        fs::create_dir_all(&cat_dir).unwrap();
        fs::write(cat_dir.join("bad.wasm"), b"not a wasm component").unwrap();

        let result = discover_plugins(&engine, tmp.path());
        assert!(matches!(result, Err(Error::ComponentInstantiate { .. })));
    }

    #[rstest]
    #[case(PluginCategory::CustomDashboard, &["render"])]
    #[case(PluginCategory::DataTransform, &["transform"])]
    #[case(PluginCategory::SnapshotTemplate, &["render"])]
    fn declared_exports_matches_wit_contract(
        #[case] category: PluginCategory,
        #[case] expected: &'static [&'static str],
    ) {
        assert_eq!(declared_exports(category), expected);
    }

    #[test]
    fn resolve_plugin_dir_uses_env_override_when_set() {
        const TEST_ENV: &str = "ANDROMEDA_PULSE_PLUGIN_DIR";
        unsafe {
            std::env::set_var(TEST_ENV, "/tmp/test-plugins");
        }
        let resolved = resolve_plugin_dir();
        unsafe {
            std::env::remove_var(TEST_ENV);
        }
        assert_eq!(resolved, PathBuf::from("/tmp/test-plugins"));
    }

    #[test]
    fn registry_find_returns_loaded_plugin_by_id() {
        let engine = build_engine().expect("engine builds");
        let tmp = TempDir::new().unwrap();
        stage_wat_plugin(tmp.path(), PluginCategory::DataTransform, "find-me");

        let plugins = discover_plugins(&engine, tmp.path()).expect("discovery ok");
        let mut registry = PluginRegistry::empty();
        registry.replace_plugins(plugins);

        assert!(registry.find("find-me").is_some());
        assert!(registry.find("not-there").is_none());
    }
}
