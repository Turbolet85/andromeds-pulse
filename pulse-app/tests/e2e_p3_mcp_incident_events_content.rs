//! Cross-process CONTENT assertion for `retrieve_incident_events` over what the
//! REAL producer writes. The incident is opened by
//! `create_incident_from_l4_output` through the production
//! `CorpusIncidentPersistence` adapter onto an on-disk encrypted corpus, so its
//! ledger carries the producer's own creation event; a real sidecar subprocess
//! then reads it back, resolves it, and reads it again.
//!
//! The read surface coerces any kind outside the shared vocabulary
//! (`triage::contract::incident_event_kinds`) to `unknown`. A leg seeded by hand
//! cannot see a kind the producer writes and the reader lacks — this one can.
//!
//! Both sides open with `OsKeychainBackend::new("com.andromeda.pulse")`; on a
//! host with no credential store the test SKIPS cleanly — CI has no store.

#![cfg(feature = "mcp-server")]

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use corpus::contract::{Corpus, CorpusWriter, KeychainBackend, OsKeychainBackend};
use interpretation::schema::{Decision, L4Output, Severity as L4Severity};
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use pulse_app::inference_runtime::create_incident_from_l4_output;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, INCIDENT_EVENT_CREATED,
    InMemoryIncidentRegistry, IncidentRegistry, PriorityTier, incident_event_kinds,
};

const WORKSPACE: &str = "ws-mcp-incident-events-producer";
const SKIP_LINE: &str = "[skip] no OS credential store; cross-process corpus unavailable";

fn sidecar_binary_path() -> PathBuf {
    // `CARGO_BIN_EXE_*` is exposed only to the crate declaring the [[bin]]
    // (crates/mcp-server), so derive from CARGO_MANIFEST_DIR instead.
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
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

fn now_unix_nano() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

fn surfacing_digest() -> Digest {
    Digest {
        kind: DigestKind::CadenceTier3,
        token_count: 512,
        payload_summary: "WINDOW ... SERVICES ...".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000,
        workspace: WORKSPACE.to_string(),
        window_start_unix_nano: 1_700_000_000_000,
        window_end_unix_nano: 1_700_000_060_000,
        services: vec![],
        attention_cues: vec![DigestCueRef {
            kind: CueKind::ErrorRateSpike,
            priority_tier: PriorityTier::Suggested,
            summary: "cue summary".to_string(),
            scope: CueScope::Service,
            fingerprint: Some("a3f91c0b7e2d4568a3f91c0b7e2d4568".to_string()),
            scope_id: Some("checkout-service".to_string()),
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

fn surfacing_output() -> L4Output {
    L4Output {
        schema_version: "2.0".into(),
        prompt_version: "v2.1".into(),
        decision: Decision::Surface,
        severity: L4Severity::Suggested,
        title: "checkout error spike".into(),
        symptom: "symptom text".into(),
        timeline: "timeline text".into(),
        hypotheses: vec![],
        investigation_steps: vec![],
        evidence_refs: vec![],
        fingerprint: "incident-fp".into(),
        model_tier: "primary".into(),
        hardware_profile: "cpu_primary".into(),
        is_resolution_summary: false,
    }
}

/// Open the incident through the real producer and adapter. `None` = no
/// credential store on this host, so the caller skips cleanly.
fn open_incident_through_the_producer(data_dir: &Path, opened_at: i64) -> Option<i64> {
    let corpus_dir = data_dir.join("corpus");
    std::fs::create_dir_all(&corpus_dir).expect("corpus dir");
    let keychain: Arc<dyn KeychainBackend> =
        Arc::new(OsKeychainBackend::new("com.andromeda.pulse"));
    let corpus = Corpus::open(corpus_dir.join("corpus.db"), keychain).ok()?;
    let writer: Arc<dyn CorpusWriter> = Arc::new(corpus);
    let persistence = CorpusIncidentPersistence::new(writer);
    let registry = InMemoryIncidentRegistry::new();

    create_incident_from_l4_output(
        &registry,
        &persistence,
        &surfacing_digest(),
        &surfacing_output(),
        opened_at,
    );

    let active = registry.list_active(WORKSPACE);
    assert_eq!(active.len(), 1, "the producer opened exactly one incident");
    Some(active[0].id)
}

async fn call_tool(data_dir: &Path, request_id: u64, tool_name: &str, arguments: Value) -> Value {
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .expect("spawn sidecar");

    let mut stdin = child.stdin.take().expect("stdin handle");
    let stdout = child.stdout.take().expect("stdout handle");
    let mut lines = BufReader::new(stdout).lines();

    let frame = serde_json::json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": "tools/call",
        "params": { "name": tool_name, "arguments": arguments }
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
    serde_json::from_str(&line).expect("JSON-RPC envelope")
}

fn events(response: &Value) -> Vec<(String, i64)> {
    assert!(
        response.get("error").is_none(),
        "no error expected: {response}"
    );
    response["result"]["events"]
        .as_array()
        .expect("events array")
        .iter()
        .map(|e| {
            (
                e["event_kind"].as_str().expect("kind").to_string(),
                e["occurred_unix_nano"].as_i64().expect("ts"),
            )
        })
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn real_producer_incident_events_read_back_through_the_sidecar() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let opened_at = now_unix_nano() - 1_000_000_000;
    let Some(id) = open_incident_through_the_producer(tmp.path(), opened_at) else {
        eprintln!("{SKIP_LINE}");
        return;
    };
    let vocabulary = incident_event_kinds();

    let before = call_tool(
        tmp.path(),
        960,
        "retrieve_incident_events",
        serde_json::json!({ "incident_id": id }),
    )
    .await;
    assert_eq!(
        events(&before),
        vec![(INCIDENT_EVENT_CREATED.to_string(), opened_at)],
        "the producer's creation event reads back under its own kind"
    );
    assert!(!before.to_string().contains("unknown"), "{before}");

    let sent = now_unix_nano();
    let resolved = call_tool(
        tmp.path(),
        961,
        "mark_incident_resolved",
        serde_json::json!({ "incident_id": id }),
    )
    .await;
    let received = now_unix_nano();
    assert_eq!(resolved["result"]["resolved"], true, "{resolved}");

    let after = call_tool(
        tmp.path(),
        962,
        "retrieve_incident_events",
        serde_json::json!({ "incident_id": id }),
    )
    .await;
    let after_events = events(&after);
    assert!(!after.to_string().contains("unknown"), "{after}");
    for (kind, _) in &after_events {
        assert!(
            vocabulary.contains(&kind.as_str()),
            "{kind} is outside the shared vocabulary"
        );
    }
    assert_eq!(
        after_events.first(),
        Some(&(INCIDENT_EVENT_CREATED.to_string(), opened_at))
    );
    let (last_kind, last_at) = after_events.last().expect("a last event");
    assert_eq!(last_kind, "resolved");
    assert!(
        (sent..=received).contains(last_at),
        "resolved at {last_at} outside the call window {sent}..={received}"
    );
}
