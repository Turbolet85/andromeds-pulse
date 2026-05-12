use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use mcp_server::feature_gate::{GateState, validate_double_gate};
use mcp_server::jsonrpc::{
    CODE_METHOD_NOT_FOUND, empty_tools_list, error, initialize_result, parse_request, success,
};
use mcp_server::tracing_setup;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[cfg(feature = "mcp-server")]
use rmcp as _;

fn resolve_data_dir() -> PathBuf {
    if let Ok(p) = env::var("ANDROMEDA_PULSE_DATA_DIR") {
        return PathBuf::from(p);
    }
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata).join("andromeda-pulse");
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("com.andromeda.pulse");
        }
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("andromeda-pulse");
    } else if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".andromeda-pulse");
    }
    env::temp_dir().join("andromeda-pulse")
}

fn main() -> ExitCode {
    let data_dir = resolve_data_dir();
    let _guard = match tracing_setup::init(&data_dir) {
        Ok(g) => g,
        Err(e) => {
            eprintln!(
                "{{\"level\":\"ERROR\",\"target\":\"mcp.boot.tracing.init\",\"message\":\"tracing init failed; sidecar aborting\",\"error_detail\":\"{e}\"}}"
            );
            return ExitCode::from(1);
        }
    };

    match validate_double_gate() {
        GateState::Enabled => {
            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    tracing::error!(
                        target: "mcp.boot.runtime.init",
                        reason = "runtime_build_failed",
                        error_detail = %e,
                        "tokio runtime build failed; sidecar aborting",
                    );
                    return ExitCode::from(1);
                }
            };
            match runtime.block_on(run_stdio_loop()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    tracing::error!(
                        target: "mcp.server.error",
                        reason = "stdio_loop_error",
                        error_detail = %e,
                        "stdio loop terminated with error",
                    );
                    ExitCode::from(1)
                }
            }
        }
        GateState::EnvDisabled | GateState::FeatureMissing => ExitCode::SUCCESS,
    }
}

async fn run_stdio_loop() -> Result<(), std::io::Error> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();

    loop {
        match reader.next_line().await? {
            None => return Ok(()),
            Some(line) if line.trim().is_empty() => continue,
            Some(line) => {
                let response_bytes = dispatch_line(&line);
                if let Some(bytes) = response_bytes {
                    stdout.write_all(&bytes).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
            }
        }
    }
}

fn dispatch_line(line: &str) -> Option<Vec<u8>> {
    let req = match parse_request(line) {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                target: "mcp.server.parse_error",
                reason = "framing_error",
                error_detail = %e,
                "rejected malformed JSON-RPC frame",
            );
            let resp = error(Value::Null, -32700, "parse error");
            return serde_json::to_vec(&resp).ok();
        }
    };

    let id = req.id.clone().unwrap_or(Value::Null);

    if req.id.is_none() {
        tracing::info!(
            target: "mcp.server.notification",
            method = %req.method,
            params_count = req.params.as_ref().map(count_params).unwrap_or(0_u64),
            "received notification (no response)",
        );
        return None;
    }

    let body = match req.method.as_str() {
        "initialize" => {
            let resp = success(id, initialize_result());
            serde_json::to_vec(&resp).ok()
        }
        "tools/list" => {
            let resp = success(id, empty_tools_list());
            serde_json::to_vec(&resp).ok()
        }
        "tools/call" => {
            let resp = error(
                id,
                CODE_METHOD_NOT_FOUND,
                "tools not yet registered; chunk #48 substrate only",
            );
            serde_json::to_vec(&resp).ok()
        }
        "ping" => {
            let resp = success(id, json!({}));
            serde_json::to_vec(&resp).ok()
        }
        _ => {
            let resp = error(id, CODE_METHOD_NOT_FOUND, "method not found");
            serde_json::to_vec(&resp).ok()
        }
    };

    tracing::info!(
        target: "mcp.server.dispatch",
        method = %req.method,
        params_count = req.params.as_ref().map(count_params).unwrap_or(0_u64),
        result_type = "envelope",
        "dispatched JSON-RPC request",
    );

    body
}

fn count_params(v: &Value) -> u64 {
    match v {
        Value::Object(o) => o.len() as u64,
        Value::Array(a) => a.len() as u64,
        _ => 0,
    }
}
