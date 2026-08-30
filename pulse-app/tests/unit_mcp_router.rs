// Migrated 2026-08-30 from `pulse-app/src/mcp_router.rs::tests` — that crate
// sets `[lib] test = false` (the WebView2 workaround), so a src-level
// `mod tests` compiles, passes clippy, and NEVER RUNS. File-level feature
// gate per the `e2e_p3_mcp_subprocess_tools_call.rs` precedent: the
// `mcp_router` module itself only exists under `--features mcp-server`.

#![cfg(feature = "mcp-server")]

use std::sync::Arc;

use mcp_server_crate::feature_gate::GateState;
use ui_bridge::contract::AppError;

use pulse_app::mcp_router::{
    McpApi, McpApiImpl, McpServerState, gate_state_to_mcp_state, state_label,
};

fn impl_with_stub_path() -> McpApiImpl {
    let path = Arc::new(std::env::temp_dir().join("nonexistent-mcp-binary-for-tests"));
    McpApiImpl::new(path)
}

#[tokio::test]
async fn status_returns_dto_with_double_gate_mapped_state() {
    let api = impl_with_stub_path();
    let result = api.status().await.expect("status returns ok");
    // State depends on cfg!(feature = "mcp-server") + env var at test time;
    // we assert only that sidecar_running is false (no spawn yet).
    assert!(!result.sidecar_running);
    assert!(result.pid.is_none());
}

#[tokio::test]
async fn status_state_distinguishes_three_gates() {
    let api = impl_with_stub_path();
    let result = api.status().await.expect("status returns ok");
    // Just verify the field carries one of the 3 enum values.
    assert!(matches!(
        result.state,
        McpServerState::Enabled | McpServerState::Disabled | McpServerState::Unavailable
    ));
}

#[tokio::test]
async fn start_refused_when_double_gate_inactive() {
    let api = impl_with_stub_path();
    // Ensure env var is unset so double-gate cannot be Enabled.
    unsafe {
        std::env::remove_var("ANDROMEDA_PULSE_MCP_ENABLED");
    }
    let err = api.start().await.expect_err("start refuses");
    match err {
        AppError::Internal { message } => {
            assert!(message.contains("feature flag and env var must both be active"));
        }
        other => panic!("expected AppError::Internal, got {other:?}"),
    }
}

#[tokio::test]
async fn stop_is_idempotent_when_sidecar_not_running() {
    let api = impl_with_stub_path();
    let result = api.stop().await.expect("stop ok");
    assert!(matches!(
        result.state,
        McpServerState::Enabled | McpServerState::Disabled | McpServerState::Unavailable
    ));
}

#[test]
fn gate_state_to_mcp_state_maps_three_variants() {
    assert!(matches!(
        gate_state_to_mcp_state(GateState::Enabled),
        McpServerState::Enabled
    ));
    assert!(matches!(
        gate_state_to_mcp_state(GateState::EnvDisabled),
        McpServerState::Disabled
    ));
    assert!(matches!(
        gate_state_to_mcp_state(GateState::FeatureMissing),
        McpServerState::Unavailable
    ));
}

#[test]
fn state_label_returns_lowercase_for_each_variant() {
    assert_eq!(state_label(McpServerState::Enabled), "enabled");
    assert_eq!(state_label(McpServerState::Disabled), "disabled");
    assert_eq!(state_label(McpServerState::Unavailable), "unavailable");
}
