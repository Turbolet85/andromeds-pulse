//! Plugin TauRPC router — `plugins.{list,reload,invoke}` resolvers.
//!
//! Library crate `crates/plugins/` stays Tauri-free (workspace boundary
//! discipline per arch §Cross-cutting Patterns Module dependency direction);
//! the TauRPC binding for `plugins.*` lives here, matching the
//! `snapshot_runtime.rs` precedent for resolvers that wrap a library
//! crate's surface with Tauri-aware state.
//!
//! `invoke` is capability-handshake-only at chunk #47 (per phase-44 plan
//! §Implementation notes + research §Open question #1): the `capability`
//! argument is validated against the plugin's category WIT-declared
//! exports (`crates/plugins/src/loader.rs::declared_exports`). No
//! `wasmtime::component::Instance::call` invocation occurs — actual
//! export-fn invocation requires `wasmtime::component::bindgen!` per-category
//! type generation, deferred to a future chunk.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use plugins::contract::{Error as PluginsError, PluginCategory};
use plugins::loader::{PluginRegistry, declared_exports, discover_plugins};
use serde::{Deserialize, Serialize};
use ui_bridge::contract::AppError;
use wasmtime::Engine;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct PluginDto {
    pub id: String,
    pub basename: String,
    pub category: String,
    pub byte_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct PluginListEnvelope {
    pub items: Vec<PluginDto>,
    pub total: u32,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct PluginInvokeResult {
    pub plugin_id: String,
    pub capability: String,
    pub allowed: bool,
}

#[taurpc::procedures(path = "plugins")]
pub trait PluginsApi {
    async fn list() -> Result<PluginListEnvelope, AppError>;
    async fn reload() -> Result<PluginListEnvelope, AppError>;
    async fn invoke(plugin_id: String, capability: String) -> Result<PluginInvokeResult, AppError>;
}

#[derive(Clone)]
pub struct PluginsApiImpl {
    engine: Arc<Engine>,
    registry: Arc<Mutex<PluginRegistry>>,
    canonical_plugin_dir: Arc<PathBuf>,
}

impl PluginsApiImpl {
    pub fn new(
        engine: Arc<Engine>,
        registry: Arc<Mutex<PluginRegistry>>,
        canonical_plugin_dir: Arc<PathBuf>,
    ) -> Self {
        Self {
            engine,
            registry,
            canonical_plugin_dir,
        }
    }
}

fn category_str(category: PluginCategory) -> &'static str {
    category.as_str()
}

fn envelope_from_registry(registry: &PluginRegistry) -> PluginListEnvelope {
    let items: Vec<PluginDto> = registry
        .plugins()
        .iter()
        .map(|p| PluginDto {
            id: p.id.clone(),
            basename: p.basename.clone(),
            category: category_str(p.category).to_string(),
            byte_count: p.byte_count,
        })
        .collect();
    let total = u32::try_from(items.len()).unwrap_or(u32::MAX);
    PluginListEnvelope {
        items,
        total,
        next_cursor: None,
    }
}

#[taurpc::resolvers]
impl PluginsApi for PluginsApiImpl {
    #[tracing::instrument(skip_all, fields(
        plugin_count = tracing::field::Empty,
    ))]
    async fn list(self) -> Result<PluginListEnvelope, AppError> {
        let registry = self.registry.lock().map_err(|_| AppError::Internal {
            message: "plugins: registry lock poisoned".to_string(),
        })?;
        let envelope = envelope_from_registry(&registry);
        tracing::Span::current().record("plugin_count", envelope.total as u64);
        tracing::info!(
            target: "plugins.list.request",
            plugin_count = envelope.total as u64,
            "plugins list returned",
        );
        Ok(envelope)
    }

    #[tracing::instrument(skip_all, fields(
        previous_count = tracing::field::Empty,
        new_count = tracing::field::Empty,
    ))]
    async fn reload(self) -> Result<PluginListEnvelope, AppError> {
        let engine = Arc::clone(&self.engine);
        let plugin_dir = Arc::clone(&self.canonical_plugin_dir);
        let discovered =
            tokio::task::spawn_blocking(move || discover_plugins(&engine, plugin_dir.as_path()))
                .await
                .map_err(|_| AppError::internal("plugins: reload task failed"))?;
        let plugins = discovered.map_err(AppError::from)?;

        let mut registry = self.registry.lock().map_err(|_| AppError::Internal {
            message: "plugins: registry lock poisoned".to_string(),
        })?;
        let previous_count = registry.loaded_count() as u64;
        registry.replace_plugins(plugins);
        let envelope = envelope_from_registry(&registry);
        let new_count = envelope.total as u64;

        tracing::Span::current().record("previous_count", previous_count);
        tracing::Span::current().record("new_count", new_count);
        tracing::info!(
            target: "plugins.reload.request",
            previous_count = previous_count,
            new_count = new_count,
            "plugins reloaded",
        );
        Ok(envelope)
    }

    #[tracing::instrument(skip_all, fields(
        plugin_name = %plugin_id,
        capability_name = %capability,
        capability_allowed = tracing::field::Empty,
    ))]
    async fn invoke(
        self,
        plugin_id: String,
        capability: String,
    ) -> Result<PluginInvokeResult, AppError> {
        let registry = self.registry.lock().map_err(|_| AppError::Internal {
            message: "plugins: registry lock poisoned".to_string(),
        })?;
        registry.record_invocation();

        let plugin = registry
            .find(&plugin_id)
            .ok_or_else(|| PluginsError::NotFound {
                plugin_id: plugin_id.clone(),
            })?;
        let allowed_exports = declared_exports(plugin.category);
        let allowed = allowed_exports.iter().any(|exp| *exp == capability);
        tracing::Span::current().record("capability_allowed", allowed);

        if !allowed {
            tracing::warn!(
                target: "plugin.invoke.error",
                plugin_name = %plugin_id,
                capability_name = %capability,
                capability_allowed = false,
                "capability rejected",
            );
            return Err(AppError::from(PluginsError::CapabilityRejected {
                plugin_id: plugin_id.clone(),
                capability_name: capability.clone(),
                reason: "capability not declared in WIT".to_string(),
            }));
        }

        tracing::info!(
            target: "plugin.invoke.request",
            plugin_name = %plugin_id,
            capability_name = %capability,
            capability_allowed = true,
            "capability handshake ok",
        );
        Ok(PluginInvokeResult {
            plugin_id,
            capability,
            allowed: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plugins::engine::build_engine;
    use plugins::loader::LoadedPlugin;
    use wasmtime::component::Component;

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
}
