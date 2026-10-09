// Migrated 2026-08-30 from `pulse-app/src/plugins_router.rs::tests` — that
// crate sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS. The registry handle
// reaches here via the `#[doc(hidden)]` field widening per test-plan §2/§4.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use plugins::contract::PluginCategory;
use plugins::engine::build_engine;
use plugins::loader::{LoadedPlugin, PluginRegistry};
use ui_bridge::contract::AppError;
use wasmtime::Engine;
use wasmtime::component::Component;

use pulse_app::plugins_router::{PluginsApi, PluginsApiImpl};

fn empty_component(engine: &Engine) -> Component {
    let bytes = wat::parse_str("(component)").expect("empty component WAT");
    Component::from_binary(engine, &bytes).expect("component from binary")
}

fn impl_with_one_plugin(category: PluginCategory, id: &str) -> PluginsApiImpl {
    let engine = Arc::new(build_engine().expect("engine builds"));
    let mut registry = PluginRegistry::empty();
    let component = empty_component(&engine);
    registry.replace_plugins(vec![LoadedPlugin {
        id: id.to_string(),
        category,
        component,
        basename: format!("{id}.wasm"),
        byte_count: 42,
    }]);
    let registry = Arc::new(Mutex::new(registry));
    let plugin_dir = Arc::new(PathBuf::from("/tmp/test-plugins"));
    PluginsApiImpl::new(engine, registry, plugin_dir)
}

fn empty_impl() -> PluginsApiImpl {
    let engine = Arc::new(build_engine().expect("engine builds"));
    let registry = Arc::new(Mutex::new(PluginRegistry::empty()));
    let plugin_dir = Arc::new(PathBuf::from("/tmp/test-plugins"));
    PluginsApiImpl::new(engine, registry, plugin_dir)
}

#[tokio::test]
async fn list_returns_empty_envelope_when_registry_empty() {
    let api = empty_impl();
    let result = api.list().await.expect("list ok");
    assert!(result.items.is_empty());
    assert_eq!(result.total, 0);
    assert!(result.next_cursor.is_none());
}

#[tokio::test]
async fn list_returns_loaded_plugin_dtos() {
    let api = impl_with_one_plugin(PluginCategory::DataTransform, "echo");
    let result = api.list().await.expect("list ok");
    assert_eq!(result.total, 1);
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].id, "echo");
    assert_eq!(result.items[0].basename, "echo.wasm");
    assert_eq!(result.items[0].category, "data-transform");
    assert_eq!(result.items[0].byte_count, 42);
}

#[tokio::test]
async fn invoke_allowed_capability_returns_success() {
    let api = impl_with_one_plugin(PluginCategory::DataTransform, "echo");
    let result = api
        .invoke("echo".to_string(), "transform".to_string())
        .await
        .expect("invoke ok");
    assert_eq!(result.plugin_id, "echo");
    assert_eq!(result.capability, "transform");
    assert!(result.allowed);
}

#[tokio::test]
async fn invoke_disallowed_capability_returns_app_error_plugin() {
    let api = impl_with_one_plugin(PluginCategory::DataTransform, "echo");
    let result = api
        .invoke("echo".to_string(), "disallowed_capability".to_string())
        .await;
    match result {
        Err(AppError::Plugin { plugin_id, message }) => {
            assert_eq!(plugin_id, "echo");
            assert!(message.contains("capability"));
        }
        other => panic!("expected AppError::Plugin, got {other:?}"),
    }
}

#[tokio::test]
async fn invoke_not_found_plugin_returns_app_error_plugin() {
    let api = empty_impl();
    let result = api
        .invoke("missing".to_string(), "transform".to_string())
        .await;
    match result {
        Err(AppError::Plugin { plugin_id, message }) => {
            assert_eq!(plugin_id, "missing");
            assert!(message.contains("not found"));
        }
        other => panic!("expected AppError::Plugin not-found, got {other:?}"),
    }
}

#[tokio::test]
async fn invoke_records_cumulative_counter_regardless_of_outcome() {
    let api = impl_with_one_plugin(PluginCategory::DataTransform, "echo");
    let _ = api
        .clone()
        .invoke("echo".to_string(), "transform".to_string())
        .await;
    let _ = api
        .clone()
        .invoke("echo".to_string(), "disallowed".to_string())
        .await;
    let _ = api
        .clone()
        .invoke("missing".to_string(), "transform".to_string())
        .await;

    let registry = api.registry.lock().expect("lock not poisoned");
    assert_eq!(
        registry.active_invocations(),
        3,
        "every invoke increments the cumulative counter — allowed / rejected / not-found alike"
    );
}

#[tokio::test]
async fn invoke_each_category_accepts_declared_export() {
    for (category, export) in [
        (PluginCategory::CustomDashboard, "render"),
        (PluginCategory::DataTransform, "transform"),
        (PluginCategory::SnapshotTemplate, "render"),
    ] {
        let api = impl_with_one_plugin(category, "fixture");
        let result = api
            .invoke("fixture".to_string(), export.to_string())
            .await
            .expect("invoke ok");
        assert!(
            result.allowed,
            "category {} must accept its declared export `{}`",
            category.as_str(),
            export
        );
    }
}
