pub mod baseline_observer;
pub mod baseline_persistence;
pub mod bincode_bounded;
pub mod connection_router;
pub mod diagnostics_router;
pub mod drain_persistence;
pub mod heartbeat;
pub mod lifecycle_persistence;
#[cfg(feature = "mcp-server")]
pub mod mcp_router;
pub mod observability;
pub mod plugins_router;
pub mod restart_observer;
pub mod services_router;
pub mod snapshot_runtime;
pub mod storage_router;
pub mod storm_observer;
pub mod storm_persistence;
pub mod streams;
pub mod tray;
pub mod viz_routers;
pub mod window;

pub fn taurpc_export_config() -> specta_typescript::Typescript {
    specta_typescript::Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number)
}
