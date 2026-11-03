//! Security vector (security-plan §Security Anti-Patterns → Logging, obs-plan
//! §1 Vector 4): an MCP tool's answer body never reaches the log. A planted
//! marker answered by `query_traces` through the console engine's door stands
//! 0 times in the engine's log family, beside the one `door.call` record that
//! carries only `result_type` and `result_count` of the answer.
//!
//! Each arm spawns the built `andromeda-pulse-engine run` under a cleared
//! environment, sends telemetry over the real gRPC receiver, asks the door's
//! socket, and ends the engine by SIGTERM before it reads the log family.
#![cfg(unix)]

use std::net::{SocketAddr, TcpStream};
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use serde_json::{Value, json};
use test_support::otlp::{resource, span};
use test_support::ports::{PortClaim, claim_port};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const ENGINE: &str = env!("CARGO_BIN_EXE_andromeda-pulse-engine");
const PASSPHRASE: &str = "door-redaction-corpus-passphrase-a37e";
const BOOT_WAIT: Duration = Duration::from_secs(60);
const END_WAIT: Duration = Duration::from_secs(30);
const ANSWER_WAIT: Duration = Duration::from_secs(60);

fn loopback(port: u16) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port))
}

fn accepts(port: u16) -> bool {
    TcpStream::connect_timeout(&loopback(port), Duration::from_millis(250)).is_ok()
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

struct Engine {
    child: Child,
    data_dir: tempfile::TempDir,
    grpc: u16,
    http: u16,
    _ports: [PortClaim; 2],
}

impl Drop for Engine {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

impl Engine {
    fn spawn() -> Self {
        let data_dir = tempfile::tempdir().expect("data dir");
        let ports = [claim_port(), claim_port()];
        let (grpc, http) = (ports[0].number, ports[1].number);
        let mut command = Command::new(ENGINE);
        command
            .env_clear()
            .env("ANDROMEDA_PULSE_DATA_DIR", data_dir.path())
            .env("ANDROMEDA_PULSE_OTLP_GRPC_PORT", grpc.to_string())
            .env("ANDROMEDA_PULSE_OTLP_HTTP_PORT", http.to_string())
            .env("ANDROMEDA_PULSE_CORPUS_PASSPHRASE", PASSPHRASE)
            .current_dir(data_dir.path())
            .arg("run")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        let child = command.spawn().expect("spawn the console program");
        Self {
            child,
            data_dir,
            grpc,
            http,
            _ports: ports,
        }
    }

    fn socket(&self) -> PathBuf {
        self.data_dir.path().join("run").join("door").join("sock")
    }

    fn still_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }

    async fn wait_for_log(&mut self, what: &str, condition: impl Fn(&[Value]) -> bool) {
        let deadline = Instant::now() + BOOT_WAIT;
        while !condition(&family_lines(self.data_dir.path())) {
            assert!(
                self.still_running(),
                "the program ended while awaited: {what}"
            );
            assert!(
                Instant::now() < deadline,
                "not in the log within {BOOT_WAIT:?}: {what}"
            );
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn wait_until_ready(&mut self) {
        let deadline = Instant::now() + BOOT_WAIT;
        while !(accepts(self.grpc) && accepts(self.http)) {
            assert!(self.still_running(), "the program ended before it listened");
            assert!(
                Instant::now() < deadline,
                "both ports accept within {BOOT_WAIT:?}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        self.wait_for_log("the door's boot record", |lines| {
            !records(lines, "app.boot.door").is_empty()
        })
        .await;
    }

    async fn end_by_sigterm(&mut self) -> Vec<Value> {
        let pid = libc::pid_t::try_from(self.child.id()).expect("pid fits pid_t");
        // SAFETY: signals the one child this test spawned, by its own pid.
        let sent = unsafe { libc::kill(pid, libc::SIGTERM) };
        assert_eq!(sent, 0, "the signal was delivered to the child");
        let deadline = Instant::now() + END_WAIT;
        let status = loop {
            if let Some(status) = self.child.try_wait().expect("try_wait") {
                break status;
            }
            assert!(
                Instant::now() < deadline,
                "the program ends within {END_WAIT:?}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        };
        assert_eq!(status.signal(), Some(15), "ended by SIGTERM");
        family_lines(self.data_dir.path())
    }
}

fn family_paths(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir.join("logs")) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with("agent-latest.jsonl"))
        })
        .collect();
    paths.sort();
    paths
}

fn family_lines(dir: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    for path in family_paths(dir) {
        let text = std::fs::read_to_string(path).unwrap_or_default();
        // A line still being written has no terminator yet and is not read.
        let complete = match text.rfind('\n') {
            Some(end) => &text[..end],
            None => "",
        };
        for line in complete.lines().filter(|l| !l.trim().is_empty()) {
            out.push(serde_json::from_str(line).expect("every sink line is one JSON object"));
        }
    }
    out
}

fn family_text(dir: &Path) -> String {
    family_paths(dir)
        .iter()
        .map(|path| std::fs::read_to_string(path).expect("read a member"))
        .collect()
}

fn records<'a>(lines: &'a [Value], target: &str) -> Vec<&'a Value> {
    lines
        .iter()
        .filter(|v| v.get("target").and_then(Value::as_str) == Some(target))
        .collect()
}

fn appended(lines: &[Value], table: &str) -> u64 {
    records(lines, "duckdb.append")
        .iter()
        .filter(|record| record["fields"]["table_name"] == table)
        .filter_map(|record| record["fields"]["rows_appended"].as_u64())
        .sum()
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
    let read = tokio::time::timeout(ANSWER_WAIT, BufReader::new(read_half).read_line(&mut line))
        .await
        .expect("the door answers within the bound")
        .expect("read the answer");
    assert!(read > 0, "the door answered before it closed");
    serde_json::from_str(&line).expect("an answer line is one JSON object")
}

// The marker rides the `service.name` resource attribute: of a stored span,
// that attribute's value is the user content `query_traces` answers with.
const MARKER: &str = "door-redaction-marker-svc-5e7a";

async fn engine_with_marked_spans() -> Engine {
    let mut engine = Engine::spawn();
    engine.wait_until_ready().await;
    let start = now_ns();
    TraceServiceClient::connect(format!("http://{}", loopback(engine.grpc)))
        .await
        .expect("the gRPC receiver accepts a trace client")
        .export(tonic::Request::new(
            resource()
                .service(MARKER)
                .spans((0..3).map(|offset| span().name("redaction-span").at(start + offset)))
                .trace_request(),
        ))
        .await
        .expect("the span export is accepted");
    engine
        .wait_for_log("the spans are stored", |lines| {
            appended(lines, "spans") >= 3
        })
        .await;
    engine
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_traces_through_the_door_answers_the_marker_and_the_log_holds_none_of_it() {
    let mut engine = engine_with_marked_spans().await;

    let answer = ask(
        &engine.socket(),
        tool_call(
            1,
            "query_traces",
            json!({ "time_window_seconds": 300, "limit": 10 }),
        ),
    )
    .await;
    let items = answer["result"]["items"]
        .as_array()
        .unwrap_or_else(|| panic!("a result with items: {answer}"));
    assert!(
        items.iter().any(|row| row["service"] == MARKER),
        "the door answers with the planted marker: {answer}"
    );

    let lines = engine.end_by_sigterm().await;
    let calls = records(&lines, "door.call");
    assert_eq!(calls.len(), 1, "exactly one call record: {calls:?}");
    let call = calls[0];
    assert_eq!(call["fields"]["tool"], "query_traces");
    assert_eq!(call["fields"]["outcome"], "ok");
    assert_eq!(call["fields"]["result_type"], "paginated_traces");
    assert_eq!(call["fields"]["result_count"], items.len() as u64);

    let log = family_text(engine.data_dir.path());
    assert_eq!(
        log.matches(MARKER).count(),
        0,
        "the answer's marker is not in the engine's log family"
    );
    let answer_text = answer["result"].to_string();
    assert!(
        !log.contains(&answer_text),
        "the answer body is not in the log verbatim"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_refused_argument_canary_is_neither_answered_nor_logged() {
    const CANARY: &str = "secret-canary-mcp-arg-token-XYZ-67890";
    let mut engine = engine_with_marked_spans().await;

    let answer = ask(
        &engine.socket(),
        tool_call(
            2,
            "query_traces",
            json!({
                "time_window_seconds": 60,
                "limit": 10,
                "deliberately_unrecognized_field_with_canary": CANARY,
            }),
        ),
    )
    .await;
    assert_eq!(answer["error"]["code"], -32602, "{answer}");
    assert!(!answer.to_string().contains(CANARY), "{answer}");

    engine.end_by_sigterm().await;
    let log = family_text(engine.data_dir.path());
    assert_eq!(
        log.matches(CANARY).count(),
        0,
        "the canary is not in the log"
    );
}
