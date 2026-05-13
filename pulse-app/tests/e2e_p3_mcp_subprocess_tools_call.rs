//! P3 E2E coverage — MCP rmcp sidecar subprocess + JSON-RPC 2.0 frames.
//! Per test-plan §6 P3 canonical structure; extends chunk #49 sidecar
//! subprocess precedent с the cross-process happy-path + canaries.
//!
//! Cross-process buffer sharing deferred per plan.md Open Question 1 —
//! the sidecar boots с its OWN ephemeral DuckDB; query_* tools return
//! empty `items`. P3 coverage verifies envelope shape + canaries +
//! double-gate; non-empty data flow is а separate future chunk.

#![cfg(feature = "mcp-server")]

use std::process::Stdio;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

fn sidecar_binary_path() -> std::path::PathBuf {
    // pulse-app/tests can't access CARGO_BIN_EXE_andromeda-pulse-mcp
    // (cargo exposes that env var only к the crate that declares the
    // [[bin]] — that's crates/mcp-server). Derive the path from
    // CARGO_MANIFEST_DIR (pulse-app/) + ../target/{profile}/<binary>.
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let target_dir = manifest
        .parent()
        .expect("workspace root")
        .join("target")
        .join("debug");
    let binary_name = if cfg!(target_os = "windows") {
        "andromeda-pulse-mcp.exe"
    } else {
        "andromeda-pulse-mcp"
    };
    target_dir.join(binary_name)
}

async fn spawn_sidecar_and_call(
    tmp_path: &std::path::Path,
    request_id: u64,
    tool_name: &str,
    arguments: serde_json::Value,
) -> (serde_json::Value, String) {
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", tmp_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn sidecar");

    let mut stdin = child.stdin.take().expect("stdin handle");
    let stdout = child.stdout.take().expect("stdout handle");
    let stderr = child.stderr.take().expect("stderr handle");
    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    let frame = serde_json::json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments,
        }
    });
    let bytes = serde_json::to_vec(&frame).expect("frame bytes");
    stdin.write_all(&bytes).await.expect("write");
    stdin.write_all(b"\n").await.expect("newline");
    stdin.flush().await.expect("flush");

    let response_line = tokio::time::timeout(Duration::from_secs(10), stdout_reader.next_line())
        .await
        .expect("response within 10s")
        .expect("read")
        .expect("line present");
    let parsed: serde_json::Value =
        serde_json::from_str(&response_line).expect("response is JSON-RPC envelope");

    let mut stderr_collected = String::new();
    let deadline = tokio::time::Instant::now() + Duration::from_millis(500);
    while tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_millis(100), stderr_reader.next_line()).await {
            Ok(Ok(Some(line))) => {
                stderr_collected.push_str(&line);
                stderr_collected.push('\n');
            }
            _ => break,
        }
    }

    let _ = child.kill().await;
    (parsed, stderr_collected)
}

#[tokio::test(flavor = "multi_thread")]
async fn p3_mcp_subprocess_tools_call_query_traces_returns_envelope_shape() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, _stderr) = spawn_sidecar_and_call(
        tmp.path(),
        200,
        "query_traces",
        serde_json::json!({"time_window_seconds": 300, "limit": 50}),
    )
    .await;
    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["id"], 200);
    assert!(parsed.get("error").is_none(), "no error expected: {parsed}");
    // Per arch §Standard Contracts MCP server: result is paginated envelope.
    let items = parsed["result"]["items"]
        .as_array()
        .expect("items array present");
    // Empty per chunk #49 ephemeral-buffer limitation (cross-process buffer
    // sharing deferred per plan.md Open Question 1).
    assert_eq!(
        items.len(),
        0,
        "empty buffer yields empty items; got {}",
        items.len()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn p3_mcp_subprocess_sql_injection_canary_kept_out_of_stderr() {
    // Per security plan §Input Validation row DuckDB + plan.md security
    // canary on MCP query_* tools — SQL injection via cursor argument
    // MUST be rejected at compute_window parse step before any DuckDB
    // query runs; sentinel SQL fragment MUST NOT appear in stderr log.
    let tmp = tempfile::tempdir().expect("tempdir");
    let sql_fragment = "'; DROP TABLE spans; --";
    let (parsed, stderr_collected) = spawn_sidecar_and_call(
        tmp.path(),
        201,
        "query_traces",
        serde_json::json!({
            "time_window_seconds": 300,
            "limit": 50,
            "cursor": sql_fragment,
        }),
    )
    .await;
    assert_eq!(parsed["jsonrpc"], "2.0", "envelope well-formed");
    assert!(
        !stderr_collected.contains("DROP TABLE"),
        "SQL fragment must not appear in stderr: {stderr_collected}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn p3_mcp_subprocess_response_body_sentinel_never_in_stderr() {
    // Per security plan §Logging snapshot/clipboard/MCP hygiene + obs §PII
    // Vector 4: MCP tool response body content MUST NEVER appear in stderr
    // log (or any tracing output). Sentinel-in-cursor canary asserts the
    // AllowList scrubber drops the cursor field at the subscriber layer.
    let tmp = tempfile::tempdir().expect("tempdir");
    let sentinel = "sentinel-canary-NEVERLOG-RESPONSE-CHUNK50";
    let (_parsed, stderr_collected) = spawn_sidecar_and_call(
        tmp.path(),
        202,
        "query_traces",
        serde_json::json!({
            "time_window_seconds": 300,
            "limit": 50,
            "cursor": sentinel,
        }),
    )
    .await;
    assert!(
        !stderr_collected.contains(sentinel),
        "response-body sentinel must NEVER appear in stderr; got: {stderr_collected}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn p3_mcp_subprocess_double_gate_negative_canary_unset_env_exits_clean() {
    // Per security plan §API Security row "MCP feature double-gate" +
    // arch §Cross-cutting Patterns "Feature-gate hygiene": even when
    // binary is built with --features mcp-server, runtime env var must
    // also be set; without it, sidecar exits cleanly without writing to
    // stdout (preserving JSON-RPC framing discipline для downstream MCP
    // clients).
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut child = Command::new(sidecar_binary_path())
        .env_remove("ANDROMEDA_PULSE_MCP_ENABLED")
        .env("ANDROMEDA_PULSE_DATA_DIR", tmp.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");

    let status = tokio::time::timeout(Duration::from_secs(5), child.wait())
        .await
        .expect("exit within 5s")
        .expect("wait succeeds");
    assert!(
        status.success(),
        "double-gate-disabled binary must exit cleanly; got {status:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn p3_mcp_subprocess_generate_snapshot_returns_markdown_field() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, _stderr) = spawn_sidecar_and_call(
        tmp.path(),
        203,
        "generate_snapshot",
        serde_json::json!({"token_budget": 25000, "time_window_seconds": 300}),
    )
    .await;
    assert_eq!(parsed["id"], 203);
    assert!(parsed.get("error").is_none(), "no error: {parsed}");
    assert!(
        parsed["result"]["markdown"].as_str().is_some(),
        "result.markdown field present"
    );
}
