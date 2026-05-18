pub mod baseline_observer;
pub mod connection_router;
pub mod heartbeat;
#[cfg(feature = "mcp-server")]
pub mod mcp_router;
pub mod observability;
pub mod plugins_router;
pub mod restart_observer;
pub mod services_router;
pub mod snapshot_runtime;
pub mod storm_observer;
pub mod streams;
pub mod tray;
pub mod viz_routers;
pub mod window;

pub fn taurpc_export_config() -> specta_typescript::Typescript {
    specta_typescript::Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number)
}
