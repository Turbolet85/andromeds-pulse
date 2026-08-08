//! Chunk #77 — Test (c) MCP response body redaction PII vector test.
//!
//! Per pulse-v0_2_0-route §77 Part 2 third sub-bullet + acceptance criterion
//! #6 item (c) + security plan §Anti-Pattern Logging row 1 ("NEVER log ...
//! MCP tool response bodies") + obs-plan §5 Section 5 Vector 4:
//!
//! Spawn the `andromeda-pulse-mcp` rmcp sidecar subprocess (feature-gated by
//! `mcp-server` per chunk #50 precedent), invoke a `query_traces` JSON-RPC
//! tool call, capture the sidecar's stderr (JSON-formatted `tracing` output
//! per obs-plan §3 sidecar discipline), and assert the captured stderr does
//! NOT contain a verbatim copy of the response body — only structured
//! metadata fields (`result_type` / `result_count` / etc.) per the
//! redaction discipline.
//!
//! Note: this test extends the chunk #49 e2e_p3 sidecar-subprocess precedent.
//! The sidecar boots with its OWN ephemeral DuckDB; query_* tools return empty
//! `items` per the e2e_p3 docstring. The test's negative-canary discipline
//! is that EVEN the empty response shape doesn't surface the raw response
//! object in stderr tracing — only the structured-metadata form.

#![cfg(feature = "mcp-server")]

use std::process::Stdio;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

fn sidecar_binary_path() -> std::path::PathBuf {
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

/// Spawn the sidecar, send a JSON-RPC tools/call request, collect both the
/// JSON-RPC response on stdout AND any tracing output on stderr (the sidecar
/// writes tracing JSON to stderr per obs-plan §3 sidecar discipline).
async fn spawn_and_collect(
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
    let deadline = tokio::time::Instant::now() + Duration::from_millis(800);
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
async fn mcp_query_traces_stderr_does_not_leak_response_body_verbatim() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (parsed, stderr) = spawn_and_collect(
        tmp.path(),
        1,
        "query_traces",
        serde_json::json!({
            "time_window_seconds": 60,
            "limit": 10,
        }),
    )
    .await;

    // The JSON-RPC response body shape. Per the e2e_p3 docstring, query_traces
    // returns empty `items` because the sidecar boots with its own ephemeral
    // DuckDB. Verify the shape is a valid JSON-RPC envelope with a result or
    // an error.
    assert!(
        parsed.get("result").is_some() || parsed.get("error").is_some(),
        "MCP response should have result or error: {parsed}"
    );

    // The KEY redaction invariant per security plan §Anti-Pattern Logging row 1
    // + obs-plan §5 Vector 4: the FULL response body MUST NOT appear verbatim
    // in stderr tracing. Serialize the response to its byte representation
    // and grep stderr for the substring.
    let response_serialized = serde_json::to_string(&parsed).expect("serialize response");

    // The full serialized response (~50-200 bytes for empty result) should not
    // appear verbatim in the stderr tracing stream.
    assert!(
        !stderr.contains(&response_serialized),
        "stderr tracing leaked verbatim MCP response body per Vector 4. \
         response: {response_serialized}\n stderr (truncated): {}",
        stderr.chars().take(2000).collect::<String>()
    );

    // Stronger: if the response carries a `content` / `result_content` /
    // `text` field (rmcp tool-result frame), assert that specific field's
    // value substring is NOT in stderr. Empty-items result is ~"[]" string;
    // exercise the assertion for whatever shape rmcp emits.
    if let Some(result) = parsed.get("result") {
        if let Some(content) = result.get("content") {
            let content_serialized = serde_json::to_string(content).expect("serialize content");
            // For empty content this may be "[]" which trivially appears in
            // most stderr — skip the assertion if content is empty array.
            if content_serialized != "[]" && content_serialized.len() > 4 {
                assert!(
                    !stderr.contains(&content_serialized),
                    "stderr tracing leaked verbatim MCP result.content per Vector 4. \
                     content: {content_serialized}\n stderr (truncated): {}",
                    stderr.chars().take(2000).collect::<String>()
                );
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn mcp_invalid_tool_call_stderr_does_not_leak_full_request_arguments() {
    // Send a tool call with a distinctive canary marker in the `arguments` field
    // to a valid tool that will likely reject the unrecognized argument shape.
    // The sidecar's tracing of the error MUST NOT echo the verbatim arguments
    // value into stderr — only structured error type + tool name metadata.
    let tmp = tempfile::tempdir().expect("tempdir");
    let canary = "secret-canary-mcp-arg-token-XYZ-67890";

    let (parsed, stderr) = spawn_and_collect(
        tmp.path(),
        2,
        "query_traces",
        serde_json::json!({
            "time_window_seconds": 60,
            "limit": 10,
            "deliberately_unrecognized_field_with_canary": canary,
        }),
    )
    .await;

    // The response may be a successful envelope (rmcp ignores extra fields)
    // OR an error. Either way, stderr MUST NOT contain the canary verbatim
    // per security plan §Anti-Pattern Logging row 1 + Vector 4 discipline
    // (response/request bodies are not loggable verbatim).
    let _ = parsed;
    assert!(
        !stderr.contains(canary),
        "stderr tracing leaked verbatim MCP request argument canary `{canary}` per Vector 4. \
         stderr (truncated): {}",
        stderr.chars().take(2000).collect::<String>()
    );
}
