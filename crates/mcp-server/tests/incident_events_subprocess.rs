//! Cross-process CONTENT assertion for `retrieve_incident_events`.
//!
//! The events are seeded through the production writer (`update_incident_status`)
//! into an on-disk encrypted corpus, then read back by a real sidecar subprocess
//! that opens its OWN connection over the same database — so the test reaches
//! what an agent reaches, not an in-process double.
//!
//! The creation event is seeded the way the L4 incident producer writes it
//! (`save_incident_event` with `INCIDENT_EVENT_CREATED` and an empty payload);
//! the producer itself lives in pulse-app, whose own cross-process leg drives it.
//!
//! Both sides open with `OsKeychainBackend::new("com.andromeda.pulse")`, the
//! sidecar's own backend and service id. On a host with no credential store the
//! open fails and each test SKIPS cleanly rather than failing; CI has no store.

#![cfg(feature = "mcp-server")]

use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use corpus::contract::{Corpus, CorpusWriter, KeychainBackend, OsKeychainBackend};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, INCIDENT_EVENT_CREATED, Incident, IncidentStatus,
    PriorityTier, Severity,
};

const WS: &str = "ws-mcp-incident-events";
const TITLE_CANARY: &str = "evt-canary-{0}-%s-7Q4Z";
// Digit strings chosen to occur nowhere else in a sidecar log line.
const T0: i64 = 3_141_592_653_589_793_211;
const T1: i64 = 3_141_592_653_589_793_238;
const T2: i64 = 3_141_592_653_589_793_299;
const SKIP_LINE: &str = "[skip] no OS credential store; cross-process corpus unavailable";

fn sidecar_binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_andromeda-pulse-mcp")
}

fn sample_incident() -> Incident {
    Incident {
        id: 0,
        workspace: WS.to_string(),
        fingerprint: "0123456789abcdef0123456789abcdef".to_string(),
        title: TITLE_CANARY.to_string(),
        detail: TITLE_CANARY.to_string(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: None,
        status: IncidentStatus::Active,
        severity: Severity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: vec![],
            fingerprint_hashes: vec![],
            timestamps_unix_nano: vec![],
        },
        opened_at_unix_nano: 1_000,
        updated_at_unix_nano: 1_000,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

/// Seed one incident, its creation event (@ T0) and two status transitions
/// (acknowledged @ T1, resolved @ T2) through the production writers. `None` =
/// no credential store on this host, so the caller skips cleanly.
fn seed_incident_with_events(data_dir: &Path) -> Option<i64> {
    let corpus_dir = data_dir.join("corpus");
    std::fs::create_dir_all(&corpus_dir).expect("corpus dir");
    let keychain: Arc<dyn KeychainBackend> =
        Arc::new(OsKeychainBackend::new("com.andromeda.pulse"));
    let corpus = Corpus::open(corpus_dir.join("corpus.db"), keychain).ok()?;
    let mut incident = sample_incident();
    let payload = bincode::serialize(&incident).expect("encode");
    let id = corpus
        .save_incident(WS, "active", 1_000, 1_000, None, None, &payload)
        .expect("seed incident");
    corpus
        .save_incident_event(id, INCIDENT_EVENT_CREATED, T0, &[])
        .expect("creation event");

    incident.status = IncidentStatus::Acknowledged;
    let payload = bincode::serialize(&incident).expect("encode");
    corpus
        .update_incident_status(id, "acknowledged", T1, None, &payload)
        .expect("acknowledge");

    incident.status = IncidentStatus::Resolved;
    let payload = bincode::serialize(&incident).expect("encode");
    corpus
        .update_incident_status(id, "resolved", T2, Some(T2), &payload)
        .expect("resolve");
    Some(id)
}

/// One `tools/call` against a fresh sidecar. Closing stdin ends the sidecar's
/// read loop, so it exits on its own and its file sink drains before the
/// stderr capture is read whole.
async fn call_tool(data_dir: &Path, request_id: u64, arguments: Value) -> (Value, String) {
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("spawn sidecar");

    let mut stdin = child.stdin.take().expect("stdin handle");
    let stdout = child.stdout.take().expect("stdout handle");
    let mut stderr = child.stderr.take().expect("stderr handle");
    let stderr_task = tokio::spawn(async move {
        let mut text = String::new();
        let _ = stderr.read_to_string(&mut text).await;
        text
    });
    let mut lines = BufReader::new(stdout).lines();

    let frame = serde_json::json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": "tools/call",
        "params": { "name": "retrieve_incident_events", "arguments": arguments }
    });
    stdin
        .write_all(&serde_json::to_vec(&frame).expect("frame"))
        .await
        .expect("write");
    stdin.write_all(b"\n").await.expect("newline");
    stdin.flush().await.expect("flush");

    let line = tokio::time::timeout(Duration::from_secs(15), lines.next_line())
        .await
        .expect("response within 15s")
        .expect("read")
        .expect("line present");

    drop(stdin);
    if tokio::time::timeout(Duration::from_secs(10), child.wait())
        .await
        .is_err()
    {
        let _ = child.kill().await;
    }
    let stderr_text = tokio::time::timeout(Duration::from_secs(5), stderr_task)
        .await
        .expect("stderr closed within 5s")
        .expect("stderr task");
    (
        serde_json::from_str(&line).expect("JSON-RPC envelope"),
        stderr_text,
    )
}

fn read_log_family(data_dir: &Path) -> String {
    let mut text = String::new();
    let Ok(entries) = std::fs::read_dir(data_dir.join("logs")) else {
        return text;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("agent-latest.jsonl") {
            text.push_str(&std::fs::read_to_string(entry.path()).unwrap_or_default());
        }
    }
    text
}

fn has_rendered_response_record(stream: &str) -> bool {
    stream
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .any(|record| {
            record["target"] == "mcp.tools.call.response"
                && record["fields"]["tool_name"] == "retrieve_incident_events"
                && record["fields"]["result_type"] == "incident_events"
                && record["fields"]["result_count"] == 3
        })
}

#[tokio::test(flavor = "multi_thread")]
async fn incident_events_read_back_cross_process_with_body_and_clean_logs() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let Some(id) = seed_incident_with_events(tmp.path()) else {
        eprintln!("{SKIP_LINE}");
        return;
    };

    let (parsed, stderr_text) =
        call_tool(tmp.path(), 950, serde_json::json!({ "incident_id": id })).await;

    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["id"], 950);
    assert!(parsed.get("error").is_none(), "no error expected: {parsed}");
    let result = &parsed["result"];
    assert_eq!(result["incident_id"], id);
    assert_eq!(
        result["events"],
        serde_json::json!([
            { "event_kind": "created", "occurred_unix_nano": T0 },
            { "event_kind": "acknowledged", "occurred_unix_nano": T1 },
            { "event_kind": "resolved", "occurred_unix_nano": T2 },
        ]),
        "the response BODY carries the transitions the production writer recorded"
    );
    assert_eq!(result["total"], 3);
    assert_eq!(result["truncated"], false);
    assert!(!parsed.to_string().contains(TITLE_CANARY));
    assert!(!parsed.to_string().contains("unknown"));

    let file_text = read_log_family(tmp.path());
    for (stream, text) in [("stderr", &stderr_text), ("file sink", &file_text)] {
        assert!(
            has_rendered_response_record(text),
            "{stream} must carry the rendered tool_name / result_type record; got: {text}"
        );
        for canary in [
            T0.to_string(),
            T1.to_string(),
            T2.to_string(),
            TITLE_CANARY.to_string(),
        ] {
            assert_eq!(
                text.matches(canary.as_str()).count(),
                0,
                "{stream} must never carry the response body ({canary})"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn incident_events_unknown_id_returns_a_sanitized_not_found_error() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let Some(id) = seed_incident_with_events(tmp.path()) else {
        eprintln!("{SKIP_LINE}");
        return;
    };

    let (parsed, _stderr) = call_tool(
        tmp.path(),
        951,
        serde_json::json!({ "incident_id": id + 1_000 }),
    )
    .await;

    assert!(parsed["result"].is_null(), "no success result: {parsed}");
    assert_eq!(parsed["error"]["code"], -32603);
    let message = parsed["error"]["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("incident not found"),
        "unexpected message: {message}"
    );
    assert!(
        !message.contains('/') && !message.contains('\\'),
        "no path separator may cross the envelope: {message}"
    );
}
