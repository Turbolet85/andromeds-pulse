//! `mark_incident_resolved` CONTENT through the console engine's door, in both
//! arms the corpus monotonic guard can produce (test-plan §1, the door row;
//! §6 P3). A revert of the tool to an unconditional `{"resolved": true}` would
//! pass every name-only assertion; this reads the answer body and the row.
//!
//! The console program's engine boots in this process through
//! `engine_boot::start` with the fake key backend, so the corpus is the engine's
//! own and no OS credential store is needed; the incident is seeded through the
//! engine's own `corpus_writer` under its `incident_workspace_key`, and every
//! call goes over the door's socket as a caller's does.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use corpus::contract::{CorpusWriter, FakeKeychainBackend};
use ingest::contract::OtlpPort;
use pulse_app::door::ResolveWrite;
use pulse_app::engine_boot::{self, EngineConfig, EngineHandles, EngineInputs, Program};
use serde_json::{Value, json};
use test_support::ports::{PortClaim, claim_port};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use triage::contract::{
    CueKind, CueScope, EvidenceRefs, Incident, IncidentStatus, PriorityTier, Severity,
    UnknownHardwareProfile,
};

const WAIT: Duration = Duration::from_secs(30);

struct Booted {
    engine: EngineHandles,
    // Held for the arm's life: the engine binds these numbers after `start`
    // has returned.
    _ports: [PortClaim; 2],
    dir: tempfile::TempDir,
}

impl Booted {
    fn socket(&self) -> PathBuf {
        self.dir.path().join("run").join("door").join("sock")
    }

    fn corpus(&self) -> Arc<dyn CorpusWriter> {
        Arc::clone(
            self.engine
                .corpus_writer
                .as_ref()
                .expect("the corpus opened with the fake key backend"),
        )
    }
}

fn receiver_port(claim: &PortClaim) -> OtlpPort {
    OtlpPort::try_from(claim.number).expect("a claimed port is a valid receiver port")
}

/// The console program's engine, booted in this process on its own data dir.
async fn boot_console() -> Booted {
    let dir = tempfile::tempdir().expect("tempdir");
    let ports = [claim_port(), claim_port()];
    let engine = engine_boot::start(
        EngineConfig {
            data_dir: dir.path().to_path_buf(),
            grpc_port: Some(receiver_port(&ports[0])),
            http_port: Some(receiver_port(&ports[1])),
            retention_seconds: 600,
            door_resolve: ResolveWrite::On,
        },
        EngineInputs {
            program: Program::Console,
            key_backend: Arc::new(FakeKeychainBackend::new()),
            hardware_profile: Arc::new(UnknownHardwareProfile),
            interpretation: None,
        },
    );
    let booted = Booted {
        engine,
        _ports: ports,
        dir,
    };
    let deadline = tokio::time::Instant::now() + WAIT;
    while tokio::net::UnixStream::connect(booted.socket())
        .await
        .is_err()
    {
        assert!(
            tokio::time::Instant::now() < deadline,
            "the door's socket takes a connection within {WAIT:?}"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    booted
}

fn tool_call(id: u64, name: &str, arguments: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "tools/call",
        "params": { "name": name, "arguments": arguments }
    })
}

/// One request on a connection of its own, and its one answer line.
async fn ask(socket: &Path, request: Value) -> Value {
    let stream = tokio::net::UnixStream::connect(socket)
        .await
        .expect("the door's socket accepts a connection");
    let (read_half, mut writer) = stream.into_split();
    writer
        .write_all(format!("{request}\n").as_bytes())
        .await
        .expect("write the request");
    let mut line = String::new();
    let read = tokio::time::timeout(WAIT, BufReader::new(read_half).read_line(&mut line))
        .await
        .expect("the door answers within the bound")
        .expect("read the answer");
    assert!(read > 0, "the door answered before it closed");
    serde_json::from_str(&line).expect("an answer line is one JSON object")
}

fn sample_incident(workspace: &str, updated_at_unix_nano: i64) -> Incident {
    Incident {
        id: 0,
        workspace: workspace.to_string(),
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

/// One active incident in the engine's own corpus, stamped `updated_at`.
fn seed_incident(booted: &Booted, updated_at_unix_nano: i64) -> i64 {
    let workspace = booted.engine.incident_workspace_key.clone();
    let incident = sample_incident(&workspace, updated_at_unix_nano);
    let payload = bincode::serialize(&incident).expect("encode");
    booted
        .corpus()
        .save_incident(
            &workspace,
            "active",
            incident.opened_at_unix_nano,
            updated_at_unix_nano,
            None,
            None,
            &payload,
        )
        .expect("seed incident")
}

fn listed_ids(answer: &Value) -> Vec<i64> {
    answer["result"]["items"]
        .as_array()
        .unwrap_or_else(|| panic!("a result with items: {answer}"))
        .iter()
        .filter_map(|item| item["incident_id"].as_i64())
        .collect()
}

/// The APPLIED arm: the tool reports the resolution it actually performed.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn mark_incident_resolved_reports_the_applied_shape_through_the_door() {
    let booted = boot_console().await;
    let id = seed_incident(&booted, 1_000);
    let socket = booted.socket();
    assert_eq!(
        listed_ids(&ask(&socket, tool_call(899, "query_incident_list", json!({}))).await),
        vec![id]
    );

    let parsed = ask(
        &socket,
        tool_call(900, "mark_incident_resolved", json!({ "incident_id": id })),
    )
    .await;

    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["id"], 900);
    assert_eq!(
        parsed["result"]["resolved"], true,
        "the applied arm reports `resolved: true` in the answer body: {parsed}"
    );
    assert_eq!(
        parsed["result"]["incident_id"], id,
        "the answer names the incident it resolved"
    );
    let row = booted
        .corpus()
        .load_incident_by_id(id)
        .expect("the corpus read")
        .expect("the incident's row");
    assert_eq!(row.status, "resolved");
    assert!(
        listed_ids(&ask(&socket, tool_call(901, "query_incident_list", json!({}))).await)
            .is_empty(),
        "a resolved incident is no longer listed"
    );
}

/// The DECLINED arm, the one a revert to the unconditional shape would break:
/// a stored row NEWER than the engine's `now` wins the corpus monotonic guard,
/// and the tool must not claim success.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn mark_incident_resolved_reports_no_success_when_the_guard_declines() {
    let booted = boot_console().await;
    // Far enough ahead that the engine's wall-clock `now` is necessarily older,
    // so the bound `updated_unix_nano <= ?2` predicate matches no row.
    let id = seed_incident(&booted, i64::MAX / 2);
    let socket = booted.socket();

    let parsed = ask(
        &socket,
        tool_call(902, "mark_incident_resolved", json!({ "incident_id": id })),
    )
    .await;

    assert_eq!(parsed["id"], 902);
    assert!(
        parsed["result"].is_null(),
        "a declined resolution returns no success result: {parsed}"
    );
    assert!(
        parsed["error"]["code"].is_i64(),
        "an error answer: {parsed}"
    );
    let row = booted
        .corpus()
        .load_incident_by_id(id)
        .expect("the corpus read")
        .expect("the incident's row");
    assert_eq!(row.status, "active", "the guard left the row as it was");
    assert_eq!(
        listed_ids(&ask(&socket, tool_call(903, "query_incident_list", json!({}))).await),
        vec![id],
        "the declined incident is still listed"
    );
}
