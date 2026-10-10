//! The one engine boot both programs call (`pulse_app::engine_boot`), run in
//! process.
//!
//! Four arms call `engine_boot::start` on their own `TempDir` data dir, with
//! the fake key backend and two ports they picked: the composition (receivers,
//! buffer, the three span observers), the two interpretation seats, and the
//! program label. Each arm boots a different program and seat pair, so its one
//! boot record is told apart in the process-wide capture under any runner.
//!
//! Two more arms cross a real process boundary, in the form of
//! `integration_exit_cause_record.rs`: the parent re-executes this test binary,
//! the child calls `engine_boot::init_process` on its own data dir and ends one
//! way, and the parent reads the child's log family only after it has ended.
//! Each child proves it ran (`app.boot.tracing.init`, the pid record and the
//! pid file holding its own pid).

use std::collections::BTreeMap;
use std::net::{SocketAddr, TcpListener};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use corpus::contract::FakeKeychainBackend;
use ingest::contract::OtlpPort;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use interpretation::contract::ModelTier;
use pulse_app::deterministic_inference::DeterministicInferenceRunner;
use pulse_app::engine_boot::{
    self, EngineConfig, EngineHandles, EngineInputs, InterpretationSeat, Program, SeatKind,
};
use serde_json::Value;
use triage::contract::{
    CueKind, CueScope, Digest, DigestCueRef, DigestKind, DigestLwwMode, PriorityTier,
    UnknownHardwareProfile,
};

const BOOT_TARGET: &str = "app.boot.engine";
const WAIT: Duration = Duration::from_secs(30);

// ── the process-wide capture ────────────────────────────────────────────────

/// One captured event: `(target, level, field name -> value)`.
type Captured = Arc<Mutex<Vec<(String, tracing::Level, BTreeMap<String, String>)>>>;

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
        // Only the boot record is read; the engine's other records are not kept.
        if event.metadata().target() != BOOT_TARGET {
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
        .filter(|(target, _, fields)| {
            target == BOOT_TARGET
                && fields.get("program").map(String::as_str) == Some(program)
                && fields.get("interpretation").map(String::as_str) == Some(interpretation)
                && fields.get("reason").map(String::as_str) == Some(reason)
        })
        .map(|(_, level, _)| *level)
        .collect()
}

// ── booting the engine in process ───────────────────────────────────────────

struct Booted {
    engine: EngineHandles,
    grpc: SocketAddr,
    http: SocketAddr,
    _dir: tempfile::TempDir,
}

/// A port nothing holds: bound on `127.0.0.1:0`, read, released.
fn pick_port() -> OtlpPort {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind an ephemeral loopback port");
    let port = listener.local_addr().expect("local addr").port();
    OtlpPort::try_from(port).expect("an ephemeral port is a valid receiver port")
}

fn canned_seat(kind: SeatKind) -> InterpretationSeat {
    InterpretationSeat {
        runner: Arc::new(DeterministicInferenceRunner::new(ModelTier::Primary)),
        kind,
    }
}

fn boot(program: Program, interpretation: Option<InterpretationSeat>) -> Booted {
    let dir = tempfile::tempdir().expect("tempdir");
    let grpc = pick_port();
    let http = pick_port();
    let engine = engine_boot::start(
        EngineConfig {
            data_dir: dir.path().to_path_buf(),
            grpc_port: Some(grpc),
            http_port: Some(http),
            retention_seconds: 600,
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
        _dir: dir,
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

fn one_span_request(service: &str) -> ExportTraceServiceRequest {
    let start = now_ns();
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(Resource {
                attributes: vec![KeyValue {
                    key: "service.name".into(),
                    value: Some(AnyValue {
                        value: Some(any_value::Value::StringValue(service.into())),
                    }),
                }],
                dropped_attributes_count: 0,
            }),
            scope_spans: vec![ScopeSpans {
                scope: None,
                spans: vec![Span {
                    trace_id: vec![7u8; 16],
                    span_id: vec![9u8; 8],
                    name: "engine-boot-span".to_string(),
                    kind: 0,
                    start_time_unix_nano: start,
                    end_time_unix_nano: start + 1_000_000,
                    ..Default::default()
                }],
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
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
