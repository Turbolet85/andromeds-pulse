//! Cross-process CONTENT assertion for `mark_incident_resolved` — discharges
//! the `mcp-incident-read-back-cross-process-coverage` trigger (test-plan §1)
//! and §6 P3's Current residual.
//!
//! The standing gap: the 4 incident tools appeared cross-process only as
//! `tools/list` NAME assertions, so a revert of `mark_incident_resolved` to its
//! pre-guard unconditional `{"resolved": true}` shape would have passed every
//! committed cross-process test. This asserts the RESPONSE BODY of a real
//! subprocess `tools/call`, in both arms the guard can produce.
//!
//! The process boundary is the point (test-plan §4 corpus-crate precedent): the
//! sidecar opens its OWN corpus connection over the same on-disk database, so an
//! in-process double would not exercise what an agent actually reaches.
//!
//! Key custody is what makes the shared database readable across the two
//! processes: both sides open with `OsKeychainBackend::new("com.andromeda.pulse")`,
//! which resolves the same persisted key (chunk 2026-08-15-corpus-key-persistence).
//! On a host with no credential store the open fails and each test SKIPS
//! cleanly rather than failing — CI has no store.

#![cfg(feature = "mcp-server")]

use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use corpus::contract::{Corpus, CorpusWriter, KeychainBackend, OsKeychainBackend};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, Incident, IncidentStatus, PriorityTier, Severity,
};

const WS: &str = "ws-mcp-resolve-content";

fn sidecar_binary_path() -> std::path::PathBuf {
    // `CARGO_BIN_EXE_*` is exposed only to the crate declaring the [[bin]]
    // (crates/mcp-server), so derive from CARGO_MANIFEST_DIR instead.
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

fn sample_incident(updated_at_unix_nano: i64) -> Incident {
    Incident {
        id: 0,
        workspace: WS.to_string(),
        fingerprint: "0123456789abcdef0123456789abcdef".to_string(),
        title: "[r]".to_string(),
        detail: "[r]".to_string(),
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
        updated_at_unix_nano,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
}

/// Seed one incident into an on-disk corpus under `data_dir`, using the same
/// backend + service id the sidecar uses. `None` = no credential store on this
/// host, so the caller skips cleanly.
fn seed_incident(data_dir: &std::path::Path, updated_at_unix_nano: i64) -> Option<i64> {
    let corpus_dir = data_dir.join("corpus");
    std::fs::create_dir_all(&corpus_dir).expect("corpus dir");
    let keychain: Arc<dyn KeychainBackend> =
        Arc::new(OsKeychainBackend::new("com.andromeda.pulse"));
    let corpus = Corpus::open(corpus_dir.join("corpus.db"), keychain).ok()?;
    let incident = sample_incident(updated_at_unix_nano);
    let payload = bincode::serialize(&incident).expect("encode");
    let id = corpus
        .save_incident(
            WS,
            "active",
            incident.opened_at_unix_nano,
            updated_at_unix_nano,
            None,
            None,
            &payload,
        )
        .expect("seed incident");
    Some(id)
}

async fn call_tool(
    data_dir: &std::path::Path,
    request_id: u64,
    tool_name: &str,
    arguments: serde_json::Value,
) -> serde_json::Value {
    let mut child = Command::new(sidecar_binary_path())
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
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

    let _ = child.kill().await;
    serde_json::from_str(&line).expect("JSON-RPC envelope")
}

/// The APPLIED arm: the tool reports the resolution it actually performed.
#[tokio::test(flavor = "multi_thread")]
async fn mark_incident_resolved_reports_the_applied_shape_cross_process() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let Some(id) = seed_incident(tmp.path(), 1_000) else {
        eprintln!("[skip] no OS credential store; cross-process corpus unavailable");
        return;
    };

    let parsed = call_tool(
        tmp.path(),
        900,
        "mark_incident_resolved",
        serde_json::json!({ "incident_id": id }),
    )
    .await;

    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["id"], 900);
    assert_eq!(
        parsed["result"]["resolved"], true,
        "the applied arm must report `resolved: true` in the response BODY, \
         not merely appear in tools/list"
    );
    assert_eq!(
        parsed["result"]["incident_id"], id,
        "the response must name the incident it resolved"
    );
}

/// The DECLINED arm — the one a revert to the unconditional shape would break.
/// A stored row NEWER than the sidecar's `now` loses the corpus monotonic
/// guard, and the tool must say so rather than claim success.
#[tokio::test(flavor = "multi_thread")]
async fn mark_incident_resolved_reports_not_applied_when_the_guard_declines() {
    let tmp = tempfile::tempdir().expect("tempdir");
    // Far enough ahead that the sidecar's wall-clock `now` is necessarily older,
    // so the bound `updated_unix_nano <= ?2` predicate matches no row.
    let far_future = i64::MAX / 2;
    let Some(id) = seed_incident(tmp.path(), far_future) else {
        eprintln!("[skip] no OS credential store; cross-process corpus unavailable");
        return;
    };

    let parsed = call_tool(
        tmp.path(),
        901,
        "mark_incident_resolved",
        serde_json::json!({ "incident_id": id }),
    )
    .await;

    assert!(
        parsed["result"].is_null(),
        "a declined resolution must NOT return a success result: {parsed}"
    );
    let message = parsed["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(
        message.contains("not applied"),
        "the declined arm must report the resolution was not applied; got: {message}"
    );
}
