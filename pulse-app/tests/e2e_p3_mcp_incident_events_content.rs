//! `retrieve_incident_events` CONTENT over what the REAL producer writes, read
//! through the console engine's door. The incident is opened by
//! `create_incident_from_l4_output` through the production
//! `CorpusIncidentPersistence` adapter onto the engine's own encrypted corpus,
//! so its ledger carries the producer's own creation event; the door then reads
//! it back, resolves it, and reads it again.
//!
//! The read surface coerces any kind outside the shared vocabulary
//! (`triage::contract::incident_event_kinds`) to `unknown`. A leg seeded by hand
//! cannot see a kind the producer writes and the reader lacks — this one can.
//!
//! The console program's engine boots in this process through
//! `engine_boot::start` with the fake key backend, so no OS credential store is
//! needed.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use corpus::contract::{CorpusWriter, FakeKeychainBackend};
use ingest::contract::OtlpPort;
use interpretation::schema::{Decision, L4Output, Severity as L4Severity};
use pulse_app::door::ResolveWrite;
use pulse_app::engine_boot::{self, EngineConfig, EngineHandles, EngineInputs, Program};
use pulse_app::incident_persistence::CorpusIncidentPersistence;
use pulse_app::inference_runtime::create_incident_from_l4_output;
use serde_json::{Value, json};
use test_support::ports::{PortClaim, claim_port};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, INCIDENT_EVENT_CREATED,
    InMemoryIncidentRegistry, IncidentRegistry, PriorityTier, UnknownHardwareProfile,
    incident_event_kinds,
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

fn now_unix_nano() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

fn surfacing_digest(workspace: &str) -> Digest {
    Digest {
        kind: DigestKind::CadenceTier3,
        token_count: 512,
        payload_summary: "WINDOW ... SERVICES ...".to_string(),
        incident_refs: vec![],
        generated_at_unix_nano: 1_700_000_000_000,
        workspace: workspace.to_string(),
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

/// Opens the incident through the real producer and adapter, over the
/// engine's own corpus writer.
fn open_incident_through_the_producer(booted: &Booted, opened_at: i64) -> i64 {
    let workspace = booted.engine.incident_workspace_key.clone();
    let persistence = CorpusIncidentPersistence::new(booted.corpus());
    let registry = InMemoryIncidentRegistry::new();
    create_incident_from_l4_output(
        &registry,
        &persistence,
        &surfacing_digest(&workspace),
        &surfacing_output(),
        opened_at,
    );
    let active = registry.list_active(&workspace);
    assert_eq!(active.len(), 1, "the producer opened exactly one incident");
    active[0].id
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

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn real_producer_incident_events_read_back_through_the_door() {
    let booted = boot_console().await;
    let opened_at = now_unix_nano() - 1_000_000_000;
    let id = open_incident_through_the_producer(&booted, opened_at);
    let socket = booted.socket();
    let vocabulary = incident_event_kinds();

    let before = ask(
        &socket,
        tool_call(
            960,
            "retrieve_incident_events",
            json!({ "incident_id": id }),
        ),
    )
    .await;
    assert_eq!(
        events(&before),
        vec![(INCIDENT_EVENT_CREATED.to_string(), opened_at)],
        "the producer's creation event reads back under its own kind"
    );
    assert!(!before.to_string().contains("unknown"), "{before}");

    let sent = now_unix_nano();
    let resolved = ask(
        &socket,
        tool_call(961, "mark_incident_resolved", json!({ "incident_id": id })),
    )
    .await;
    let received = now_unix_nano();
    assert_eq!(resolved["result"]["resolved"], true, "{resolved}");

    let after = ask(
        &socket,
        tool_call(
            962,
            "retrieve_incident_events",
            json!({ "incident_id": id }),
        ),
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
