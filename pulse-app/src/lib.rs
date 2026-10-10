pub mod baseline_observer;
pub mod baseline_persistence;
pub mod bincode_bounded;
pub mod cadence_runner;
pub mod config_router;
pub mod connection_router;
pub mod console;
pub mod corpus_retrieval;
pub mod degraded_mode_runtime;
pub mod deterministic_inference;
pub mod diagnostics_router;
pub mod digest_runtime;
pub mod discovery_observer;
pub mod drain_persistence;
pub mod engine_boot;
pub mod hardware_profile;
pub mod heartbeat;
pub mod incident_observer;
pub mod incident_persistence;
pub mod incidents_router;
pub mod inference_runtime;
pub mod investigate_router;
pub mod lifecycle_persistence;
pub mod llamacli_inference;
#[cfg(feature = "mcp-server")]
pub mod mcp_router;
pub mod model_router;
pub mod observability;
pub mod plugins_router;
pub mod reevaluation;
pub mod render_posture;
pub mod restart_observer;
pub mod services_router;
pub mod snapshot_runtime;
pub mod storage_router;
pub mod storm_observer;
pub mod storm_persistence;
pub mod streams;
pub mod training_export;
pub mod tray;
pub mod viz_routers;
pub mod window;
pub mod window_geometry;
pub mod xlib_threads;

pub fn taurpc_export_config() -> specta_typescript::Typescript {
    specta_typescript::Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number)
}
