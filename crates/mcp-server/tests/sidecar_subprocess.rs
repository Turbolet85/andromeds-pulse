#![cfg(feature = "mcp-server")]

use std::process::Stdio;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

fn sidecar_binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_andromeda-pulse-mcp")
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_initialize_round_trip_succeeds() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", tmp.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().expect("stdin handle");
    let stdout = child.stdout.take().expect("stdout handle");
    let mut stdout_reader = BufReader::new(stdout).lines();

    let frame = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    stdin
        .write_all(frame.as_bytes())
        .await
        .expect("write frame");
    stdin.write_all(b"\n").await.expect("newline");
    stdin.flush().await.expect("flush");

    let response_line = tokio::time::timeout(Duration::from_secs(5), stdout_reader.next_line())
        .await
        .expect("response within 5s")
        .expect("read line")
        .expect("response line present");

    let parsed: serde_json::Value =
        serde_json::from_str(&response_line).expect("response is JSON-RPC envelope");
    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["id"], 1);
    assert_eq!(parsed["result"]["protocolVersion"], "2024-11-05");

    let _ = child.kill().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_env_unset_exits_cleanly_without_stdio() {
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

    assert!(status.success(), "expected exit 0, got {status:?}");

    let stdout = child.stdout.take().expect("stdout handle");
    let mut reader = BufReader::new(stdout).lines();
    let first = reader.next_line().await.expect("read line");
    assert!(
        first.is_none() || first.as_deref().map(str::is_empty).unwrap_or(false),
        "stdout should be empty when env-disabled; got: {first:?}",
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_tools_list_returns_8_tools_with_names() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", tmp.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout).lines();

    let frame = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
    stdin.write_all(frame.as_bytes()).await.expect("write");
    stdin.write_all(b"\n").await.expect("newline");
    stdin.flush().await.expect("flush");

    let response_line = tokio::time::timeout(Duration::from_secs(5), reader.next_line())
        .await
        .expect("response within 5s")
        .expect("read")
        .expect("line present");
    let parsed: serde_json::Value = serde_json::from_str(&response_line).expect("envelope");
    assert_eq!(parsed["jsonrpc"], "2.0");
    let tools = parsed["result"]["tools"].as_array().expect("tools array");
    assert_eq!(
        tools.len(),
        8,
        "chunk #49 (4 query tools) + chunk #94 (4 incident tools)"
    );
    let names: Vec<&str> = tools
        .iter()
        .map(|t| t["name"].as_str().expect("name string"))
        .collect();
    assert!(names.contains(&"query_traces"));
    assert!(names.contains(&"query_metrics"));
    assert!(names.contains(&"query_logs"));
    assert!(names.contains(&"generate_snapshot"));
    assert!(names.contains(&"query_incident_list"));
    assert!(names.contains(&"retrieve_report"));
    assert!(names.contains(&"retrieve_telemetry_slice"));
    assert!(names.contains(&"mark_incident_resolved"));

    let _ = child.kill().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_stream_segregation_holds() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", tmp.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let stderr = child.stderr.take().expect("stderr");
    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    let frame = r#"{"jsonrpc":"2.0","id":3,"method":"initialize","params":{}}"#;
    stdin.write_all(frame.as_bytes()).await.expect("write");
    stdin.write_all(b"\n").await.expect("newline");
    stdin.flush().await.expect("flush");

    let stdout_line = tokio::time::timeout(Duration::from_secs(5), stdout_reader.next_line())
        .await
        .expect("stdout within 5s")
        .expect("read stdout")
        .expect("stdout line present");

    assert!(
        !stdout_line.contains("\"level\""),
        "stdout must not contain log lines (got: {stdout_line})",
    );
    assert!(
        stdout_line.contains("\"jsonrpc\""),
        "stdout must contain JSON-RPC envelope (got: {stdout_line})",
    );

    let mut stderr_collected = String::new();
    let collect_deadline = tokio::time::Instant::now() + Duration::from_millis(500);
    while tokio::time::Instant::now() < collect_deadline {
        match tokio::time::timeout(Duration::from_millis(100), stderr_reader.next_line()).await {
            Ok(Ok(Some(line))) => {
                stderr_collected.push_str(&line);
                stderr_collected.push('\n');
            }
            _ => break,
        }
    }
    assert!(
        !stderr_collected.contains("\"jsonrpc\":\"2.0\""),
        "stderr must not contain JSON-RPC frames (got: {stderr_collected})",
    );
    if !stderr_collected.trim().is_empty() {
        assert!(
            stderr_collected.contains("\"level\""),
            "stderr lines (when present) must be tracing JSON (got: {stderr_collected})",
        );
    }

    let _ = child.kill().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_unknown_method_returns_neg_32601() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", tmp.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let mut reader = BufReader::new(stdout).lines();

    let frame = r#"{"jsonrpc":"2.0","id":4,"method":"unknown/method"}"#;
    stdin.write_all(frame.as_bytes()).await.expect("write");
    stdin.write_all(b"\n").await.expect("newline");
    stdin.flush().await.expect("flush");

    let response_line = tokio::time::timeout(Duration::from_secs(5), reader.next_line())
        .await
        .expect("response within 5s")
        .expect("read")
        .expect("line present");
    let parsed: serde_json::Value = serde_json::from_str(&response_line).expect("envelope");
    assert_eq!(parsed["error"]["code"], -32601);

    let _ = child.kill().await;
}

// ===== Chunk #49 — tool dispatch integration tests =====

async fn send_tools_call_and_read(
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
        .expect("spawn");

    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");
    let stderr = child.stderr.take().expect("stderr");
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
    let frame_bytes = serde_json::to_vec(&frame).expect("frame bytes");
    stdin.write_all(&frame_bytes).await.expect("write");
    stdin.write_all(b"\n").await.expect("newline");
    stdin.flush().await.expect("flush");

    let response_line = tokio::time::timeout(Duration::from_secs(10), stdout_reader.next_line())
        .await
        .expect("response within 10s")
        .expect("read")
        .expect("line present");
    let parsed: serde_json::Value =
        serde_json::from_str(&response_line).expect("response is JSON-RPC envelope");

    // Drain stderr opportunistically up to 500ms for assertion inspection.
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
async fn sidecar_tools_call_query_traces_returns_empty_paginated_response() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, _stderr) = send_tools_call_and_read(
        tmp.path(),
        100,
        "query_traces",
        serde_json::json!({"time_window_seconds": 300, "limit": 50}),
    )
    .await;
    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["id"], 100);
    assert!(parsed.get("error").is_none(), "no error expected: {parsed}");
    let items = parsed["result"]["items"].as_array().expect("items array");
    assert_eq!(items.len(), 0);
    assert_eq!(parsed["result"]["total"], 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_tools_call_query_metrics_returns_empty_paginated_response() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, _stderr) = send_tools_call_and_read(
        tmp.path(),
        101,
        "query_metrics",
        serde_json::json!({"time_window_seconds": 300, "limit": 50}),
    )
    .await;
    assert_eq!(parsed["id"], 101);
    assert!(parsed.get("error").is_none());
    let items = parsed["result"]["items"].as_array().expect("items array");
    assert_eq!(items.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_tools_call_query_logs_returns_empty_paginated_response() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, _stderr) = send_tools_call_and_read(
        tmp.path(),
        102,
        "query_logs",
        serde_json::json!({"time_window_seconds": 300, "limit": 50}),
    )
    .await;
    assert_eq!(parsed["id"], 102);
    assert!(parsed.get("error").is_none());
    let items = parsed["result"]["items"].as_array().expect("items array");
    assert_eq!(items.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_tools_call_generate_snapshot_returns_markdown_envelope() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, _stderr) = send_tools_call_and_read(
        tmp.path(),
        103,
        "generate_snapshot",
        serde_json::json!({"token_budget": 25000, "time_window_seconds": 300}),
    )
    .await;
    assert_eq!(parsed["id"], 103);
    assert!(parsed.get("error").is_none(), "no error: {parsed}");
    assert!(
        parsed["result"]["markdown"].as_str().is_some(),
        "markdown field present"
    );
    assert_eq!(parsed["result"]["input_row_count"], 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_response_body_never_in_stderr_for_query_traces() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let sentinel = "sentinel-canary-NEVERLOG-RESPONSE-123";
    let (parsed, stderr_collected) = send_tools_call_and_read(
        tmp.path(),
        104,
        "query_traces",
        serde_json::json!({
            "time_window_seconds": 300,
            "limit": 50,
            "cursor": sentinel,
        }),
    )
    .await;
    // The cursor value is a user-supplied string; if logged anywhere it would
    // surface as a PII leak vector (Vector 4 from security plan). Allowlist
    // scrubber must redact `cursor` field at the subscriber layer.
    // Note: viz::query::compute_window may reject malformed cursor producing
    // an error response — both Ok and Err shapes are acceptable here; the
    // assertion is purely about stderr containment of the sentinel.
    let _ = parsed;
    assert!(
        !stderr_collected.contains(sentinel),
        "sentinel must NEVER appear in stderr; got: {stderr_collected}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn sidecar_sql_injection_canary_query_traces() {
    let tmp = tempfile::tempdir().expect("tempdir");
    // The cursor field is the only string arg consumed by query_traces. SQL
    // injection via cursor would be rejected at compute_window's parse step
    // before any DuckDB query runs; either way the schema must remain intact
    // and the sentinel SQL fragment must not appear in stderr verbatim.
    let sql_fragment = "'; DROP TABLE spans; --";
    let (parsed, stderr_collected) = send_tools_call_and_read(
        tmp.path(),
        105,
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
async fn sidecar_unknown_tool_returns_neg_32601() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, _stderr) = send_tools_call_and_read(
        tmp.path(),
        106,
        "nonexistent_tool_for_testing",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["id"], 106);
    assert_eq!(
        parsed["error"]["code"], -32601,
        "unknown tool routes to method-not-found code"
    );
}
