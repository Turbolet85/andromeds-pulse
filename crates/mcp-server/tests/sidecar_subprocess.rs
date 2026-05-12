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
async fn sidecar_tools_list_returns_empty_array() {
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
    assert_eq!(
        parsed["result"]["tools"].as_array().expect("array").len(),
        0
    );

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
