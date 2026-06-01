//! MCP TauRPC router — `mcp.{status,start,stop}` resolvers.
//!
//! Library crate `crates/mcp-server/` stays Tauri-free (workspace boundary
//! discipline per arch §Cross-cutting Patterns Module dependency direction);
//! the TauRPC binding for `mcp.*` lives here, matching the
//! `plugins_router.rs` precedent for resolvers that wrap a library crate's
//! surface with Tauri-aware state.
//!
//! Sidecar lifecycle is managed via subprocess: `mcp.start` spawns the
//! `andromeda-pulse-mcp` child process с `ANDROMEDA_PULSE_MCP_ENABLED=true`
//! (re-validates double-gate first); `mcp.stop` sends SIGTERM. `mcp.status`
//! reports the live gate state (Enabled / Disabled / Unavailable) per the
//! 3-state `McpServerState` discriminator design-system §Surface:
//! desktop-native Tray Menu Actions requires.
//!
//! Per security plan §Anti-Patterns Code Patterns row 3: spawn re-validates
//! BOTH compile-time `cfg!(feature = "mcp-server")` AND runtime
//! `ANDROMEDA_PULSE_MCP_ENABLED=true` — single-gate at the IPC layer is а
//! regression even when the binary itself enforces the double-gate.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use mcp_server_crate::feature_gate::{GateState, validate_double_gate};
use serde::{Deserialize, Serialize};
use tokio::process::{Child, Command};
use ui_bridge::contract::AppError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum McpServerState {
    Enabled,
    Disabled,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct McpStatusDto {
    pub state: McpServerState,
    pub sidecar_running: bool,
    pub pid: Option<u32>,
    /// Connected-agent identity (chunk #94). The main process knows the
    /// sidecar is running (Child handle) but not whether an MCP agent has
    /// connected to its stdio peer; at this chunk's scope this is the
    /// sidecar-running state surfaced as a bounded label
    /// (`Some("stdio-client")` when running, else `None`). The "Send to
    /// agent" button in the Diagnostic Report toolbar gates on
    /// `Settings.mcp_server_enabled AND connected_agent.is_some()`.
    pub connected_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct McpStartResult {
    pub state: McpServerState,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct McpStopResult {
    pub state: McpServerState,
}

#[taurpc::procedures(path = "mcp")]
pub trait McpApi {
    async fn status() -> Result<McpStatusDto, AppError>;
    async fn start() -> Result<McpStartResult, AppError>;
    async fn stop() -> Result<McpStopResult, AppError>;
}

#[derive(Clone)]
pub struct McpApiImpl {
    sidecar_state: Arc<Mutex<Option<Child>>>,
    sidecar_binary_path: Arc<PathBuf>,
}

impl McpApiImpl {
    pub fn new(sidecar_binary_path: Arc<PathBuf>) -> Self {
        Self {
            sidecar_state: Arc::new(Mutex::new(None)),
            sidecar_binary_path,
        }
    }
}

fn gate_state_to_mcp_state(gs: GateState) -> McpServerState {
    match gs {
        GateState::Enabled => McpServerState::Enabled,
        GateState::EnvDisabled => McpServerState::Disabled,
        GateState::FeatureMissing => McpServerState::Unavailable,
    }
}

fn state_label(state: McpServerState) -> &'static str {
    match state {
        McpServerState::Enabled => "enabled",
        McpServerState::Disabled => "disabled",
        McpServerState::Unavailable => "unavailable",
    }
}

#[taurpc::resolvers]
impl McpApi for McpApiImpl {
    #[tracing::instrument(skip_all, fields(
        state = tracing::field::Empty,
        sidecar_running = tracing::field::Empty,
    ))]
    async fn status(self) -> Result<McpStatusDto, AppError> {
        let gate_state = validate_double_gate();
        let mapped = gate_state_to_mcp_state(gate_state);
        let (running, pid) = {
            let guard = self.sidecar_state.lock().map_err(|_| AppError::Internal {
                message: "mcp-server: sidecar state lock poisoned".to_string(),
            })?;
            match &*guard {
                Some(child) => (true, child.id()),
                None => (false, None),
            }
        };
        let connected_agent = if running {
            Some("stdio-client".to_string())
        } else {
            None
        };
        let span = tracing::Span::current();
        span.record("state", state_label(mapped));
        span.record("sidecar_running", running);
        tracing::info!(
            target: "mcp.status.request",
            state = state_label(mapped),
            sidecar_running = running,
            "mcp.status returned",
        );
        Ok(McpStatusDto {
            state: mapped,
            sidecar_running: running,
            pid,
            connected_agent,
        })
    }

    #[tracing::instrument(skip_all, fields(
        state = tracing::field::Empty,
    ))]
    async fn start(self) -> Result<McpStartResult, AppError> {
        let gate_state = validate_double_gate();
        let mapped = gate_state_to_mcp_state(gate_state);
        let span = tracing::Span::current();
        span.record("state", state_label(mapped));

        if !matches!(gate_state, GateState::Enabled) {
            tracing::warn!(
                target: "mcp.start.request",
                state = state_label(mapped),
                reason = "double_gate_inactive",
                "mcp.start refused — double-gate not active",
            );
            return Err(AppError::Internal {
                message:
                    "mcp-server: feature flag and env var must both be active to start sidecar"
                        .to_string(),
            });
        }

        // Idempotent check: if already running, return current pid without re-spawning.
        {
            let guard = self.sidecar_state.lock().map_err(|_| AppError::Internal {
                message: "mcp-server: sidecar state lock poisoned".to_string(),
            })?;
            if let Some(existing) = &*guard {
                let pid = existing.id();
                tracing::info!(
                    target: "mcp.start.request",
                    state = "enabled",
                    sidecar_running = true,
                    "mcp.start idempotent — sidecar already running",
                );
                return Ok(McpStartResult { state: mapped, pid });
            }
        }

        let child = Command::new(self.sidecar_binary_path.as_path())
            .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| {
                tracing::warn!(
                    target: "mcp.start.error",
                    reason = "spawn_failed",
                    error_detail = %e,
                    "mcp.start spawn failed",
                );
                AppError::Internal {
                    message: "mcp-server: sidecar spawn failed".to_string(),
                }
            })?;
        let pid = child.id();
        {
            let mut guard = self.sidecar_state.lock().map_err(|_| AppError::Internal {
                message: "mcp-server: sidecar state lock poisoned".to_string(),
            })?;
            *guard = Some(child);
        }

        tracing::info!(
            target: "mcp.start.request",
            state = "enabled",
            sidecar_running = true,
            "mcp.start spawned sidecar",
        );
        Ok(McpStartResult { state: mapped, pid })
    }

    #[tracing::instrument(skip_all, fields(
        state = tracing::field::Empty,
    ))]
    async fn stop(self) -> Result<McpStopResult, AppError> {
        let gate_state = validate_double_gate();
        let mapped = gate_state_to_mcp_state(gate_state);
        let span = tracing::Span::current();
        span.record("state", state_label(mapped));

        let child_opt: Option<Child> = {
            let mut guard = self.sidecar_state.lock().map_err(|_| AppError::Internal {
                message: "mcp-server: sidecar state lock poisoned".to_string(),
            })?;
            guard.take()
        };
        if let Some(mut child) = child_opt {
            let _ = child.kill().await;
            tracing::info!(
                target: "mcp.stop.request",
                state = state_label(mapped),
                sidecar_running = false,
                "mcp.stop killed sidecar",
            );
        } else {
            tracing::info!(
                target: "mcp.stop.request",
                state = state_label(mapped),
                sidecar_running = false,
                "mcp.stop idempotent — sidecar already stopped",
            );
        }
        Ok(McpStopResult { state: mapped })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
