//! The one engine boot both programs call (`pulse_app::engine_boot`), run in
//! process.
//!
//! Four arms call `engine_boot::start` on their own `TempDir` data dir, with
//! the fake key backend and two ports they claimed: the composition (receivers,
//! buffer, the three span observers), the two interpretation seats, and the
//! program label. Each arm boots a different program and seat pair, so its one
//! boot record is told apart in the process-wide capture under any runner.
//!
//! Five arms read the door the console program's boot seats: each connects to
//! `run/door/sock` under its own data dir as a caller does. Each runs on a
//! runtime whose threads carry the arm's name, so its door records are told
//! apart in the process-wide capture under any runner.
//!
//! Two more arms cross a real process boundary, in the form of
//! `integration_exit_cause_record.rs`: the parent re-executes this test binary,
//! the child calls `engine_boot::init_process` on its own data dir and ends one
//! way, and the parent reads the child's log family only after it has ended.
//! Each child proves it ran (`app.boot.tracing.init`, the pid record and the
//! pid file holding its own pid).

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use corpus::contract::FakeKeychainBackend;
use ingest::contract::OtlpPort;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use interpretation::contract::ModelTier;
use pulse_app::deterministic_inference::DeterministicInferenceRunner;
use pulse_app::door::ResolveWrite;
use pulse_app::engine_boot::{
    self, EngineConfig, EngineHandles, EngineInputs, InterpretationSeat, Program, SeatKind,
};
use serde_json::Value;
use test_support::otlp::{resource, span};
use test_support::ports::{PortClaim, claim_port};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, PriorityTier,
    UnknownHardwareProfile,
};

const BOOT_TARGET: &str = "app.boot.engine";
const DOOR_BOOT_TARGET: &str = "app.boot.door";
const DOOR_CALL_TARGET: &str = "door.call";
const DOOR_REFUSED_TARGET: &str = "door.refused";
// Every door arm names its runtime's threads with this prefix. The four boot
// arms read the capture without them, so a door arm that boots the same
// program and seat pair does not show in their count.
const DOOR_ARM_PREFIX: &str = "door-arm-";
const WAIT: Duration = Duration::from_secs(30);

// ── the process-wide capture ────────────────────────────────────────────────

/// One captured event: `(target, level, field name -> value, the emitting
/// thread's name)`.
type Captured = Arc<Mutex<Vec<(String, tracing::Level, BTreeMap<String, String>, String)>>>;

fn global_capture() -> Captured {
    static CAPTURE: OnceLock<Captured> = OnceLock::new();
    Arc::clone(CAPTURE.get_or_init(|| {
        let events: Captured = Arc::new(Mutex::new(Vec::new()));
        tracing::subscriber::set_global_default(CapturingSubscriber {
            events: Arc::clone(&events),
        })
        .expect("the capture is the binary's only global subscriber");
        events
    }))
}

struct CapturingSubscriber {
    events: Captured,
}

impl tracing::Subscriber for CapturingSubscriber {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }
    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
    fn event(&self, event: &tracing::Event<'_>) {
        // Only the boot record and the door's three are read; the engine's
        // other records are not kept.
        if ![
            BOOT_TARGET,
            DOOR_BOOT_TARGET,
            DOOR_CALL_TARGET,
            DOOR_REFUSED_TARGET,
        ]
        .contains(&event.metadata().target())
        {
            return;
        }
        struct Fields(BTreeMap<String, String>);
        impl tracing::field::Visit for Fields {
            fn record_str(&mut self, f: &tracing::field::Field, value: &str) {
                self.0.insert(f.name().to_string(), value.to_string());
            }
            fn record_debug(&mut self, f: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                self.0.insert(f.name().to_string(), format!("{value:?}"));
            }
        }
        let mut fields = Fields(BTreeMap::new());
        event.record(&mut fields);
        self.events.lock().expect("lock").push((
            event.metadata().target().to_string(),
            *event.metadata().level(),
            fields.0,
            std::thread::current().name().unwrap_or("").to_string(),
        ));
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}

/// The levels of every captured boot record carrying exactly these labels.
fn boot_records(
    events: &Captured,
    program: &str,
    interpretation: &str,
    reason: &str,
) -> Vec<tracing::Level> {
    events
        .lock()
        .expect("lock")
        .iter()
        .filter(|(target, _, fields, thread)| {
            target == BOOT_TARGET
                && !thread.starts_with(DOOR_ARM_PREFIX)
                && fields.get("program").map(String::as_str) == Some(program)
                && fields.get("interpretation").map(String::as_str) == Some(interpretation)
                && fields.get("reason").map(String::as_str) == Some(reason)
        })
        .map(|(_, level, _, _)| *level)
        .collect()
}

/// The records of one door target written on the threads of the arm `arm`.
fn door_records(
    events: &Captured,
    arm: &str,
    target: &str,
) -> Vec<(tracing::Level, BTreeMap<String, String>)> {
    events
        .lock()
        .expect("lock")
        .iter()
        .filter(|(seen, _, _, thread)| seen == target && thread == arm)
        .map(|(_, level, fields, _)| (*level, fields.clone()))
        .collect()
}

// ── booting the engine in process ───────────────────────────────────────────

struct Booted {
    engine: EngineHandles,
    grpc: SocketAddr,
    http: SocketAddr,
    // Held for the arm's life: the engine binds these numbers after `boot`
    // has returned.
    _ports: [PortClaim; 2],
    dir: tempfile::TempDir,
}

impl Booted {
    fn door_socket(&self) -> PathBuf {
        self.dir.path().join("run").join("door").join("sock")
    }
}

fn receiver_port(claim: &PortClaim) -> OtlpPort {
    OtlpPort::try_from(claim.number).expect("a claimed port is a valid receiver port")
}

fn canned_seat(kind: SeatKind) -> InterpretationSeat {
    InterpretationSeat {
        runner: Arc::new(DeterministicInferenceRunner::new(ModelTier::Primary)),
        kind,
    }
}

fn boot(program: Program, interpretation: Option<InterpretationSeat>) -> Booted {
    boot_with(program, interpretation, ResolveWrite::On)
}

fn boot_with(
    program: Program,
    interpretation: Option<InterpretationSeat>,
    door_resolve: ResolveWrite,
) -> Booted {
    let dir = tempfile::tempdir().expect("tempdir");
    let ports = [claim_port(), claim_port()];
    let (grpc, http) = (receiver_port(&ports[0]), receiver_port(&ports[1]));
    let engine = engine_boot::start(
        EngineConfig {
            data_dir: dir.path().to_path_buf(),
            grpc_port: Some(grpc),
            http_port: Some(http),
            retention_seconds: 600,
            door_resolve,
        },
        EngineInputs {
            program,
            key_backend: Arc::new(FakeKeychainBackend::new()),
            hardware_profile: Arc::new(UnknownHardwareProfile),
            interpretation,
        },
    );
    Booted {
        engine,
        grpc: SocketAddr::from(([127, 0, 0, 1], grpc.value())),
        http: SocketAddr::from(([127, 0, 0, 1], http.value())),
        _ports: ports,
        dir,
    }
}

/// Polls `condition` until it holds; panics naming `what` when the bound passes.
async fn wait_until(what: &str, mut condition: impl FnMut() -> bool) {
    let deadline = tokio::time::Instant::now() + WAIT;
    while !condition() {
        assert!(
            tokio::time::Instant::now() < deadline,
            "not reached within {WAIT:?}: {what}"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

async fn accepts(addr: SocketAddr) -> bool {
    tokio::net::TcpStream::connect(addr).await.is_ok()
}

async fn wait_for_both_receivers(booted: &Booted) {
    let deadline = tokio::time::Instant::now() + WAIT;
    while !(accepts(booted.grpc).await && accepts(booted.http).await) {
        assert!(
            tokio::time::Instant::now() < deadline,
            "both receivers accept within {WAIT:?}"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

// The engine's retention sweep reads a record's stamp against the wall clock,
// so a span sent to a running engine is stamped now.
fn one_span_request(service: &str) -> ExportTraceServiceRequest {
    resource()
        .service(service)
        .span(span().name("engine-boot-span").at(now_ns()))
        .trace_request()
}

/// The shape of `digest_with_cue()` in `integration_deterministic_l4_mode.rs`,
/// stamped with the workspace the engine filters its incidents by.
fn digest_with_cue(workspace: &str) -> Digest {
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
            kind: CueKind::RetryStorm,
            priority_tier: PriorityTier::Autonomous,
            summary: "retry storm".to_string(),
            scope: CueScope::Global,
            fingerprint: None,
            scope_id: None,
        }],
        corpus_matches: vec![],
        lww_mode: DigestLwwMode::Default,
        active_incident_bypass: false,
        resolution_event: false,
    }
}

fn archived_digests(engine: &EngineHandles) -> u64 {
    engine
        .corpus_reader
        .as_ref()
        .expect("the corpus opened with the fake key backend")
        .inspect()
        .expect("inspect")
        .record_counts
        .get("digest_archive")
        .copied()
        .unwrap_or(0)
}

// ── the four in-process arms ────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn both_receivers_accept_and_a_span_lands_and_lists_its_service() {
    const SERVICE: &str = "engine-boot-composition-svc";
    let events = global_capture();
    let booted = boot(Program::Window, Some(canned_seat(SeatKind::Model)));

    wait_for_both_receivers(&booted).await;
    assert!(booted.grpc.ip().is_loopback() && booted.http.ip().is_loopback());

    let mut client = TraceServiceClient::connect(format!("http://{}", booted.grpc))
        .await
        .expect("the gRPC receiver accepts a client");
    client
        .export(tonic::Request::new(one_span_request(SERVICE)))
        .await
        .expect("the export is accepted");

    let buffer_state = Arc::clone(&booted.engine.buffer_state);
    wait_until("the span advances rows_ingested", || {
        buffer_state.snapshot().rows_ingested >= 1
    })
    .await;
    // The first-sighting adapter is the last of the three span observers: a
    // service listed here went through the whole composition.
    let registry = Arc::clone(&booted.engine.lifecycle_registry);
    wait_until("the service is listed in the lifecycle registry", || {
        registry
            .list_all()
            .iter()
            .any(|item| item.service == SERVICE)
    })
    .await;

    assert_eq!(
        boot_records(&events, "window", "model", "window_runs_model"),
        [tracing::Level::INFO],
        "one boot record, at INFO"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_seated_canned_runner_turns_a_cue_bearing_digest_into_one_incident() {
    let events = global_capture();
    let booted = boot(Program::Console, Some(canned_seat(SeatKind::Deterministic)));
    let engine = &booted.engine;

    // The persister and the interpretation subscriber.
    let digests = Arc::clone(&engine.digest_broadcast);
    wait_until("both digest subscribers are listening", || {
        digests.subscriber_count() == 2
    })
    .await;
    engine
        .digest_broadcast
        .sender()
        .send(digest_with_cue(&engine.incident_workspace_key))
        .expect("the digest reaches its subscribers");

    let registry = Arc::clone(&engine.incident_registry);
    let workspace = engine.incident_workspace_key.clone();
    wait_until("one incident forms", || {
        registry.list_active(&workspace).len() == 1
    })
    .await;
    wait_until("the digest reaches the archive", || {
        archived_digests(engine) == 1
    })
    .await;

    assert_eq!(
        boot_records(
            &events,
            "console",
            "deterministic",
            "deterministic_gate_set"
        ),
        [tracing::Level::INFO],
        "one boot record, at INFO"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_empty_seat_archives_the_digest_and_forms_no_incident() {
    let events = global_capture();
    let booted = boot(Program::Console, None);
    let engine = &booted.engine;

    // The persister alone.
    let digests = Arc::clone(&engine.digest_broadcast);
    wait_until("the digest persister is listening", || {
        digests.subscriber_count() >= 1
    })
    .await;
    engine
        .digest_broadcast
        .sender()
        .send(digest_with_cue(&engine.incident_workspace_key))
        .expect("the digest reaches the persister");
    wait_until("the digest reaches the archive", || {
        archived_digests(engine) == 1
    })
    .await;

    assert_eq!(
        engine.digest_broadcast.subscriber_count(),
        1,
        "no interpretation subscriber was spawned"
    );
    assert!(
        engine
            .incident_registry
            .list_active(&engine.incident_workspace_key)
            .is_empty(),
        "no incident forms with nothing seated"
    );
    assert_eq!(
        boot_records(&events, "console", "none", "deterministic_gate_unset"),
        [tracing::Level::WARN],
        "one boot record, at WARN"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_window_program_with_a_seated_runner_reads_window_in_the_record() {
    let events = global_capture();
    let _booted = boot(Program::Window, Some(canned_seat(SeatKind::Deterministic)));

    assert_eq!(
        boot_records(&events, "window", "deterministic", "deterministic_gate_set"),
        [tracing::Level::INFO],
        "one boot record, at INFO"
    );
}

// ── the door over the booted engine ─────────────────────────────────────────

/// Runs `arm` on a runtime of its own whose threads all carry `name`: the
/// door's records are written on them, and the capture keeps each record's
/// thread name.
fn run_door_arm<F>(name: &'static str, arm: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    assert!(name.starts_with(DOOR_ARM_PREFIX));
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name(name)
        .enable_all()
        .build()
        .expect("runtime");
    if let Err(ended) = runtime.block_on(async move { tokio::spawn(arm).await }) {
        std::panic::resume_unwind(ended.into_panic());
    }
}

fn tool_call(id: u64, name: &str, arguments: Value) -> Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "tools/call",
        "params": { "name": name, "arguments": arguments }
    })
}

/// One request on a connection of its own, and its one answer line.
async fn door_ask(socket: &Path, request: Value) -> Value {
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

/// The steps of `a_seated_canned_runner_turns_a_cue_bearing_digest_into_one_incident`,
/// up to the one incident; its id.
async fn form_one_incident(engine: &EngineHandles) -> i64 {
    let digests = Arc::clone(&engine.digest_broadcast);
    wait_until("both digest subscribers are listening", || {
        digests.subscriber_count() == 2
    })
    .await;
    engine
        .digest_broadcast
        .sender()
        .send(digest_with_cue(&engine.incident_workspace_key))
        .expect("the digest reaches its subscribers");
    let registry = Arc::clone(&engine.incident_registry);
    let workspace = engine.incident_workspace_key.clone();
    wait_until("one incident forms", || {
        registry.list_active(&workspace).len() == 1
    })
    .await;
    engine
        .incident_registry
        .list_active(&engine.incident_workspace_key)[0]
        .id
}

fn corpus_row(engine: &EngineHandles, id: i64) -> corpus::contract::IncidentRowRaw {
    engine
        .corpus_writer
        .as_ref()
        .expect("the corpus opened with the fake key backend")
        .load_incident_by_id(id)
        .expect("the corpus read")
        .expect("the incident's row")
}

fn field<'a>(fields: &'a BTreeMap<String, String>, name: &str) -> &'a str {
    fields
        .get(name)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("the record carries `{name}`: {fields:?}"))
}

#[test]
fn the_door_answers_from_the_stores_the_boot_built() {
    const ARM: &str = "door-arm-stores";
    const SERVICE: &str = "door-reads-the-engine-stores-svc";
    run_door_arm(ARM, async {
        let events = global_capture();
        let booted = boot(Program::Console, None);
        wait_for_both_receivers(&booted).await;

        let mut client = TraceServiceClient::connect(format!("http://{}", booted.grpc))
            .await
            .expect("the gRPC receiver accepts a client");
        client
            .export(tonic::Request::new(one_span_request(SERVICE)))
            .await
            .expect("the export is accepted");
        let buffer_state = Arc::clone(&booted.engine.buffer_state);
        wait_until("the span advances rows_ingested", || {
            buffer_state.snapshot().rows_ingested >= 1
        })
        .await;

        let answer = door_ask(
            &booted.door_socket(),
            tool_call(1, "query_traces", serde_json::json!({})),
        )
        .await;
        let items = answer["result"]["items"]
            .as_array()
            .unwrap_or_else(|| panic!("a result with items: {answer}"));
        assert!(
            items.iter().any(|row| row["service"] == SERVICE),
            "the span the receiver took is what the door returns: {answer}"
        );

        let boots = door_records(&events, ARM, DOOR_BOOT_TARGET);
        assert_eq!(boots.len(), 1, "one door boot record: {boots:?}");
        assert_eq!(boots[0].0, tracing::Level::INFO);
        assert_eq!(field(&boots[0].1, "state"), "listening");
        assert_eq!(field(&boots[0].1, "resolve_write"), "on");

        let calls = door_records(&events, ARM, DOOR_CALL_TARGET);
        assert_eq!(calls.len(), 1, "one call record: {calls:?}");
        let (level, fields) = &calls[0];
        assert_eq!(*level, tracing::Level::INFO);
        assert_eq!(field(fields, "method"), "tools/call");
        assert_eq!(field(fields, "tool"), "query_traces");
        assert_eq!(field(fields, "outcome"), "ok");
        assert_eq!(field(fields, "write"), "false");
        assert_eq!(field(fields, "result_type"), "paginated_traces");
        assert_eq!(field(fields, "result_count"), items.len().to_string());
        assert!(door_records(&events, ARM, DOOR_REFUSED_TARGET).is_empty());
    });
}

#[test]
fn the_door_resolves_an_incident_and_says_it_was_a_write() {
    const ARM: &str = "door-arm-resolve";
    run_door_arm(ARM, async {
        let events = global_capture();
        let booted = boot(Program::Console, Some(canned_seat(SeatKind::Deterministic)));
        let id = form_one_incident(&booted.engine).await;
        let socket = booted.door_socket();

        let listed = door_ask(
            &socket,
            tool_call(1, "query_incident_list", serde_json::json!({})),
        )
        .await;
        let items = listed["result"]["items"]
            .as_array()
            .unwrap_or_else(|| panic!("a result with items: {listed}"));
        assert_eq!(items.len(), 1, "the one incident is listed: {listed}");
        assert_eq!(items[0]["incident_id"], id);
        assert_eq!(corpus_row(&booted.engine, id).status, "active");

        let resolved = door_ask(
            &socket,
            tool_call(
                2,
                "mark_incident_resolved",
                serde_json::json!({ "incident_id": id }),
            ),
        )
        .await;
        assert_eq!(resolved["result"]["resolved"], true, "{resolved}");

        // Read through the engine's own corpus writer, not through the door.
        let row = corpus_row(&booted.engine, id);
        assert_eq!(row.status, "resolved");
        assert!(row.resolved_unix_nano.is_some());

        let calls = door_records(&events, ARM, DOOR_CALL_TARGET);
        assert_eq!(calls.len(), 2, "the list and the resolve: {calls:?}");
        let (list_level, list) = &calls[0];
        assert_eq!(field(list, "tool"), "query_incident_list");
        assert_eq!(field(list, "write"), "false");
        assert_eq!(*list_level, tracing::Level::INFO);
        let (write_level, write) = &calls[1];
        assert_eq!(field(write, "tool"), "mark_incident_resolved");
        assert_eq!(field(write, "write"), "true");
        assert_eq!(field(write, "outcome"), "ok");
        assert_eq!(*write_level, tracing::Level::INFO);
    });
}

#[test]
fn the_door_refuses_the_write_with_the_switch_off() {
    const ARM: &str = "door-arm-write-off";
    run_door_arm(ARM, async {
        let events = global_capture();
        let booted = boot_with(
            Program::Console,
            Some(canned_seat(SeatKind::Deterministic)),
            ResolveWrite::Off,
        );
        let id = form_one_incident(&booted.engine).await;
        let socket = booted.door_socket();

        let refused = door_ask(
            &socket,
            tool_call(
                1,
                "mark_incident_resolved",
                serde_json::json!({ "incident_id": id }),
            ),
        )
        .await;
        assert_eq!(refused["error"]["code"], -32001, "{refused}");
        assert!(refused.get("result").is_none());

        let row = corpus_row(&booted.engine, id);
        assert_eq!(row.status, "active", "the incident stays active");
        assert!(row.resolved_unix_nano.is_none());

        let calls = door_records(&events, ARM, DOOR_CALL_TARGET);
        assert_eq!(calls.len(), 1, "one call record: {calls:?}");
        let (level, fields) = &calls[0];
        assert_eq!(field(fields, "tool"), "mark_incident_resolved");
        assert_eq!(field(fields, "write"), "true");
        assert_eq!(field(fields, "outcome"), "write_off");
        assert_eq!(*level, tracing::Level::WARN);

        let listed = door_ask(
            &socket,
            serde_json::json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }),
        )
        .await;
        let named: Vec<&str> = listed["result"]["tools"]
            .as_array()
            .unwrap_or_else(|| panic!("a tool list: {listed}"))
            .iter()
            .filter_map(|tool| tool["name"].as_str())
            .collect();
        assert!(
            named.contains(&"mark_incident_resolved"),
            "the list still names the write: {named:?}"
        );

        let boots = door_records(&events, ARM, DOOR_BOOT_TARGET);
        assert_eq!(boots.len(), 1, "one door boot record: {boots:?}");
        assert_eq!(field(&boots[0].1, "state"), "listening");
        assert_eq!(field(&boots[0].1, "resolve_write"), "off");
    });
}

#[test]
fn an_incident_forms_with_the_door_listening_and_no_reader() {
    const ARM: &str = "door-arm-no-reader";
    run_door_arm(ARM, async {
        let events = global_capture();
        let booted = boot(Program::Console, Some(canned_seat(SeatKind::Deterministic)));
        let socket = booted.door_socket();

        let boots = door_records(&events, ARM, DOOR_BOOT_TARGET);
        assert_eq!(boots.len(), 1, "one door boot record: {boots:?}");
        assert_eq!(field(&boots[0].1, "state"), "listening");
        assert!(socket.exists(), "the door's socket exists");

        // Nothing connects to the door until the incident has formed.
        let id = form_one_incident(&booted.engine).await;
        assert_eq!(corpus_row(&booted.engine, id).status, "active");
        wait_until("the digest reaches the archive", || {
            archived_digests(&booted.engine) == 1
        })
        .await;

        // The door was accepting all along: a connection that asks nothing is
        // taken and, once the caller ends its side, closed.
        let mut stream = tokio::net::UnixStream::connect(&socket)
            .await
            .expect("the door's socket accepts a connection");
        stream.shutdown().await.expect("end the caller's side");
        let mut rest = Vec::new();
        let read = tokio::time::timeout(WAIT, stream.read_to_end(&mut rest))
            .await
            .expect("the door closes its side within the bound")
            .expect("read to the end");
        assert_eq!(read, 0, "nothing was asked and nothing was written");

        assert!(
            door_records(&events, ARM, DOOR_CALL_TARGET).is_empty(),
            "no call record with no reader"
        );
        assert!(
            door_records(&events, ARM, DOOR_REFUSED_TARGET).is_empty(),
            "no refusal record with no reader"
        );
    });
}

#[test]
fn the_window_program_seats_no_door() {
    const ARM: &str = "door-arm-window";
    run_door_arm(ARM, async {
        let events = global_capture();
        let booted = boot(Program::Window, Some(canned_seat(SeatKind::Deterministic)));
        wait_for_both_receivers(&booted).await;

        assert!(
            !booted.dir.path().join("run").join("door").exists(),
            "no `run/door` under the window program's data dir"
        );
        assert!(
            door_records(&events, ARM, DOOR_BOOT_TARGET).is_empty(),
            "no door boot record"
        );
        // The known positive of the reading above: this arm's engine boot
        // record is in the capture under the arm's thread name.
        assert_eq!(door_records(&events, ARM, BOOT_TARGET).len(), 1);
    });
}

// ── the two re-exec arms: the process-start function ────────────────────────

const ARM_ENV: &str = "PULSE_ENGINE_BOOT_ARM";
const DIR_ENV: &str = "PULSE_ENGINE_BOOT_DIR";
const PANIC_CANARY: &str = "engine-boot-panic-canary-71c2e9";

struct Ended {
    status: ExitStatus,
    pid: u32,
    lines: Vec<Value>,
    dir: tempfile::TempDir,
}

impl Ended {
    fn records(&self, target: &str) -> Vec<&Value> {
        self.lines
            .iter()
            .filter(|v| v.get("target").and_then(Value::as_str) == Some(target))
            .collect()
    }
}

// Child side: `Some` only in the process the parent started for this arm.
fn child_arm(arm: &str) -> Option<PathBuf> {
    if std::env::var(ARM_ENV).ok().as_deref() != Some(arm) {
        return None;
    }
    std::env::var_os(DIR_ENV).map(PathBuf::from)
}

fn run_child(test: &str, arm: &str) -> Ended {
    let exe = std::env::current_exe().expect("the test binary re-executes itself");
    let dir = tempfile::tempdir().expect("tempdir");
    let mut child = Command::new(exe)
        .args(["--exact", test, "--nocapture"])
        .env(ARM_ENV, arm)
        .env(DIR_ENV, dir.path())
        .env_remove("RUST_LOG")
        .env_remove("ANDROMEDA_PULSE_LOG_LEVEL")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn child");
    let pid = child.id();
    let status = child.wait().expect("the child ends");
    let lines = family_lines(dir.path());
    let ended = Ended {
        status,
        pid,
        lines,
        dir,
    };
    assert_child_ran_the_process_start(&ended);
    ended
}

fn family_lines(dir: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir.join("logs")) else {
        return out;
    };
    for entry in entries.flatten() {
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with("agent-latest.jsonl")
        {
            continue;
        }
        let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            out.push(serde_json::from_str(line).expect("every sink line is one JSON object"));
        }
    }
    out
}

fn assert_child_ran_the_process_start(ended: &Ended) {
    assert_eq!(
        ended.records("app.boot.tracing.init").len(),
        1,
        "the child ran the log sink init"
    );
    assert_eq!(
        ended.records("app.boot.pid").len(),
        1,
        "the child wrote its pid record"
    );
    let pid_file = ended.dir.path().join("run").join("andromeda-pulse.pid");
    assert_eq!(
        std::fs::read_to_string(pid_file).expect("the pid file is written"),
        ended.pid.to_string(),
        "the pid file holds the child's own pid"
    );
}

fn the_last_record_is_the_one_exit_record(ended: &Ended) -> &Value {
    let exits = ended.records("app.exit");
    assert_eq!(exits.len(), 1, "exactly one app.exit, got {exits:?}");
    let last = ended.lines.last().expect("the family holds records");
    assert_eq!(last["target"], "app.exit", "app.exit is the last record");
    let fields = last["fields"].as_object().expect("fields object");
    for key in ["exit_class", "exit_code", "exit_code_known", "signal"] {
        assert!(fields.contains_key(key), "app.exit carries `{key}`");
        assert_ne!(fields[key], Value::from("<redacted>"), "`{key}` rendered");
    }
    last
}

fn entered_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime")
}

#[cfg(unix)]
#[test]
fn the_process_start_keeps_the_panic_record_and_one_exit_record() {
    const ARM: &str = "panic_then_exit";
    if let Some(dir) = child_arm(ARM) {
        let runtime = entered_runtime();
        let _enter = runtime.enter();
        engine_boot::init_process(&dir);
        let joined = std::thread::Builder::new()
            .name("engine-boot-panics".into())
            .spawn(|| panic!("{PANIC_CANARY}"))
            .expect("spawn the panicking thread")
            .join();
        std::process::exit(if joined.is_err() { 7 } else { 8 });
    }
    let ended = run_child(
        "the_process_start_keeps_the_panic_record_and_one_exit_record",
        ARM,
    );
    assert_eq!(ended.status.code(), Some(7), "the thread panicked");

    let panics = ended.records("app.panic.fatal");
    assert_eq!(panics.len(), 1, "exactly one panic record, got {panics:?}");
    assert_eq!(panics[0]["level"], "ERROR");

    let exit = the_last_record_is_the_one_exit_record(&ended);
    assert_eq!(exit["level"], "ERROR");
    assert_eq!(exit["fields"]["exit_class"], "outside_event_loop");
    assert_eq!(exit["fields"]["exit_code_known"], false);
    assert_eq!(exit["fields"]["signal"], "none");

    let text = ended
        .lines
        .iter()
        .map(Value::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !text.contains(PANIC_CANARY),
        "the panic payload never reaches the log"
    );
}

#[cfg(unix)]
#[test]
fn the_process_start_records_a_sigterm_and_the_process_still_ends_by_it() {
    use std::os::unix::process::ExitStatusExt;
    const ARM: &str = "sigterm";
    if let Some(dir) = child_arm(ARM) {
        let runtime = entered_runtime();
        let _enter = runtime.enter();
        engine_boot::init_process(&dir);
        // SAFETY: raises a valid signal on this process.
        unsafe {
            libc::raise(libc::SIGTERM);
        }
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            std::thread::park_timeout(Duration::from_millis(100));
        }
        std::process::exit(99);
    }
    let ended = run_child(
        "the_process_start_records_a_sigterm_and_the_process_still_ends_by_it",
        ARM,
    );
    assert_eq!(
        ended.status.signal(),
        Some(15),
        "ended by SIGTERM, not an exit code"
    );

    let exit = the_last_record_is_the_one_exit_record(&ended);
    assert_eq!(exit["level"], "WARN");
    assert_eq!(exit["fields"]["exit_class"], "signal");
    assert_eq!(exit["fields"]["exit_code_known"], false);
    assert_eq!(exit["fields"]["signal"], "sigterm");
    assert!(ended.records("app.panic.fatal").is_empty());
}
