use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use corpus::contract::{Corpus, CorpusWriter, KeychainBackend, OsKeychainBackend};
use duckdb::Connection;
use mcp_server::feature_gate::{GateState, validate_double_gate};
use mcp_server::jsonrpc::{
    CODE_INTERNAL_ERROR, CODE_INVALID_PARAMS, CODE_METHOD_NOT_FOUND, error, initialize_result,
    parse_request, success, tools_list_manifest,
};
use mcp_server::tools::{ALL_TOOL_NAMES, IncidentToolContext, dispatch_tool};
use mcp_server::tracing_setup;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use viz::state::VizState;

#[cfg(feature = "mcp-server")]
use rmcp as _;

struct SidecarContext {
    conn: Arc<Mutex<Connection>>,
    viz_state: VizState,
    incident_ctx: Option<IncidentToolContext>,
}

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

fn init_buffer_connection() -> Result<Arc<Mutex<Connection>>, String> {
    let conn =
        Connection::open_in_memory().map_err(|e| format!("duckdb open_in_memory failed: {e}"))?;
    buffer::schema::create_schema(&conn).map_err(|e| format!("buffer schema init failed: {e}"))?;
    Ok(Arc::new(Mutex::new(conn)))
}

// Open the persistent incident corpus for the chunk #94 incident/report
// tools. Non-fatal: corpus-open failure (keychain unavailable etc.) logs a
// warn and returns None so the sidecar still serves the live-buffer query
// tools; the incident tools then return a JSON-RPC error. Mirrors the
// existing init_buffer_connection resilience posture.
fn init_incident_context(data_dir: &Path) -> Option<IncidentToolContext> {
    let corpus_db_path = data_dir.join("corpus").join("corpus.db");
    let keychain: Arc<dyn KeychainBackend> =
        Arc::new(OsKeychainBackend::new("com.andromeda.pulse"));
    match Corpus::open(corpus_db_path, keychain) {
        Ok(c) => {
            let corpus: Arc<dyn CorpusWriter> = Arc::new(c);
            // The app STAMPS incidents with its detected project root, which
            // this process cannot derive: our cwd belongs to whoever spawned
            // us (an MCP client, or Conductor from its own repo), not to the
            // workspace under observation. So the app publishes the key it
            // stamps and we read it. Absent ⇒ `data_dir`, which is both the
            // app's own detection-failure fallback and the behaviour before
            // publication existed.
            let workspace_root =
                workspace_detector::contract::read_published_workspace_key(data_dir)
                    .unwrap_or_else(|| data_dir.to_string_lossy().to_string());
            Some(IncidentToolContext {
                corpus,
                workspace_root,
            })
        }
        Err(_) => {
            tracing::warn!(
                target: "mcp.boot.corpus.init",
                reason = "corpus_open_failed",
                "incident corpus unavailable; incident tools will return errors",
            );
            None
        }
    }
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
            let conn = match init_buffer_connection() {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!(
                        target: "mcp.boot.buffer.init",
                        reason = "buffer_init_failed",
                        error_detail = %e,
                        "ephemeral buffer init failed; sidecar aborting",
                    );
                    return ExitCode::from(1);
                }
            };
            let incident_ctx = init_incident_context(&data_dir);
            let ctx = SidecarContext {
                conn,
                viz_state: VizState::new(),
                incident_ctx,
            };
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
            match runtime.block_on(run_stdio_loop(&ctx)) {
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

async fn run_stdio_loop(ctx: &SidecarContext) -> Result<(), std::io::Error> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();

    loop {
        match reader.next_line().await? {
            None => return Ok(()),
            Some(line) if line.trim().is_empty() => continue,
            Some(line) => {
                let response_bytes = dispatch_line(ctx, &line);
                if let Some(bytes) = response_bytes {
                    stdout.write_all(&bytes).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
            }
        }
    }
}

fn dispatch_line(ctx: &SidecarContext, line: &str) -> Option<Vec<u8>> {
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
            let resp = success(id, tools_list_manifest());
            tracing::info!(
                target: "mcp.tools.list.response",
                result_type = "tools_array",
                result_count = ALL_TOOL_NAMES.len() as u64,
                "tools/list returned canonical tool set",
            );
            serde_json::to_vec(&resp).ok()
        }
        "tools/call" => dispatch_tools_call(ctx, id, &req.params),
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

fn dispatch_tools_call(ctx: &SidecarContext, id: Value, params: &Option<Value>) -> Option<Vec<u8>> {
    let params = match params {
        Some(p) => p,
        None => {
            let resp = error(id, CODE_INVALID_PARAMS, "tools/call requires params object");
            return serde_json::to_vec(&resp).ok();
        }
    };
    let tool_name = match params.get("name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => {
            let resp = error(
                id,
                CODE_INVALID_PARAMS,
                "tools/call params.name missing or not string",
            );
            return serde_json::to_vec(&resp).ok();
        }
    };
    let empty_args = Value::Object(Default::default());
    let arguments = params.get("arguments").unwrap_or(&empty_args);

    tracing::info!(
        target: "mcp.tools.call.request",
        tool_name = %tool_name,
        params_count = count_params(arguments),
        "tool dispatch begin",
    );

    match dispatch_tool(
        &ctx.conn,
        &ctx.viz_state,
        ctx.incident_ctx.as_ref(),
        &tool_name,
        arguments,
    ) {
        Ok(value) => {
            let resp = success(id, value);
            serde_json::to_vec(&resp).ok()
        }
        Err(e) => {
            let code = match e {
                mcp_server::contract::Error::ToolArgsInvalid { .. } => CODE_INVALID_PARAMS,
                mcp_server::contract::Error::ToolDispatchFailed { ref reason, .. }
                    if reason == "unknown tool" =>
                {
                    CODE_METHOD_NOT_FOUND
                }
                _ => CODE_INTERNAL_ERROR,
            };
            let resp = error(id, code, e.to_string());
            serde_json::to_vec(&resp).ok()
        }
    }
}

fn count_params(v: &Value) -> u64 {
    match v {
        Value::Object(o) => o.len() as u64,
        Value::Array(a) => a.len() as u64,
        _ => 0,
    }
}
