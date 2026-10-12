//! The door on the built console program, `andromeda-pulse-engine run`.
//!
//! In the form of `integration_console_engine.rs`: each arm spawns the binary
//! under a CLEARED environment that holds only its own `TempDir` data dir, two
//! ports the test claimed and a corpus passphrase (plus the coverage profile
//! variable when this process has one), sends its telemetry over the real gRPC
//! receiver, talks to `run/door/sock` under the data dir as a caller does, and
//! reads the program's log family only after the process has ended. An engine
//! is ready when its log holds the door's boot record and the socket takes a
//! connection.
//!
//! A child still running when an arm unwinds is killed on drop.
#![cfg(unix)]

use std::collections::BTreeSet;
use std::io::Write as _;
use std::net::{SocketAddr, TcpStream};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::logs::v1::logs_service_client::LogsServiceClient;
use serde_json::{Value, json};
use test_support::otlp::{log_record, resource};
use test_support::ports::{PortClaim, claim_port};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const ENGINE: &str = env!("CARGO_BIN_EXE_andromeda-pulse-engine");
const PASSPHRASE: &str = "door-arm-corpus-passphrase-2c90aa";
const BOOT_WAIT: Duration = Duration::from_secs(60);
const END_WAIT: Duration = Duration::from_secs(30);
const ANSWER_WAIT: Duration = Duration::from_secs(60);
const CALL_FIELDS: [&str; 8] = [
    "method",
    "tool",
    "outcome",
    "write",
    "result_type",
    "result_count",
    "answer_bytes",
    "duration_ms",
];
const IDENTITY_FIELDS: [&str; 3] = ["service.name", "service.version", "deployment.environment"];

fn loopback(port: u16) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port))
}

fn accepts(port: u16) -> bool {
    TcpStream::connect_timeout(&loopback(port), Duration::from_millis(250)).is_ok()
}

struct Running {
    child: Child,
    data_dir: tempfile::TempDir,
    // Outside the data dir, so the data dir holds only what the program wrote.
    streams: tempfile::TempDir,
    grpc: u16,
    http: u16,
    _ports: [PortClaim; 2],
}

impl Drop for Running {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

impl Running {
    fn spawn(extra_env: &[(&str, &str)]) -> Self {
        let data_dir = tempfile::tempdir().expect("data dir");
        let streams = tempfile::tempdir().expect("streams dir");
        let ports = [claim_port(), claim_port()];
        let (grpc, http) = (ports[0].number, ports[1].number);
        let stdout = std::fs::File::create(streams.path().join("stdout")).expect("stdout file");
        let stderr = std::fs::File::create(streams.path().join("stderr")).expect("stderr file");
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
            .stdout(stdout)
            .stderr(stderr);
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        for (name, value) in extra_env {
            command.env(name, value);
        }
        let child = command.spawn().expect("spawn the console program");
        Self {
            child,
            data_dir,
            streams,
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

    /// Polls `condition` over the log family until it holds.
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

    /// Both receivers accept, the door's boot record is in the log and the
    /// door's socket takes a connection.
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
        // Ended without a line: the door takes it and writes no record.
        std::os::unix::net::UnixStream::connect(self.socket())
            .expect("the door's socket takes a connection");
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
        let lines = family_lines(self.data_dir.path());
        for line in &lines {
            assert_ne!(line["level"], "ERROR", "an ERROR record: {line}");
        }
        lines
    }

    fn stream(&self, name: &str) -> String {
        std::fs::read_to_string(self.streams.path().join(name)).expect("read the stream file")
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

/// The log family's bytes as written, for a count of a text in them.
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

fn assert_no_log_record_in(stream: &str, name: &str) {
    for line in stream.lines() {
        assert!(
            !(line.contains("\"target\"") || line.contains("\"level\"")),
            "{name} holds a log record"
        );
    }
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
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
    serde_json::from_str(&ask_line(socket, request).await)
        .expect("an answer line is one JSON object")
}

/// One request on a connection of its own, and its one answer line as sent,
/// terminator included.
async fn ask_line(socket: &Path, request: Value) -> String {
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
    line
}

/// Sends `count` log records of one service, each with `body`, stamped now.
async fn send_logs(engine: &Running, service: &str, body: &str, count: usize) {
    let mut client = LogsServiceClient::connect(format!("http://{}", loopback(engine.grpc)))
        .await
        .expect("the gRPC receiver accepts a client");
    let start = now_ns();
    let stamps: Vec<u64> = (0..count as u64).map(|offset| start + offset).collect();
    for batch in stamps.chunks(200) {
        let request = resource()
            .service(service)
            .log_records(batch.iter().map(|at| log_record().body(body).at(*at)))
            .logs_request();
        client
            .export(tonic::Request::new(request))
            .await
            .expect("the export is accepted");
    }
}

/// The fields of a door record without the three identity fields every record
/// carries.
fn own_fields(record: &Value) -> BTreeSet<String> {
    record["fields"]
        .as_object()
        .expect("fields object")
        .keys()
        .filter(|name| !IDENTITY_FIELDS.contains(&name.as_str()))
        .cloned()
        .collect()
}

fn assert_nothing_redacted(record: &Value) {
    for (name, value) in record["fields"].as_object().expect("fields object") {
        assert_ne!(
            value,
            &Value::from("<redacted>"),
            "`{name}` rendered: {record}"
        );
    }
}

fn the_one_door_boot_record(lines: &[Value]) -> &Value {
    let boots = records(lines, "app.boot.door");
    assert_eq!(boots.len(), 1, "exactly one door boot record: {boots:?}");
    assert_nothing_redacted(boots[0]);
    assert_eq!(
        own_fields(boots[0]),
        BTreeSet::from(["state", "resolve_write"].map(String::from))
    );
    boots[0]
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_owner_is_answered_with_what_the_engine_holds_and_the_call_record_holds_none_of_it() {
    const SERVICE: &str = "door-owner-arm-svc";
    const MARKER: &str = "door owner arm marker b7e3 read back through the door";

    let mut engine = Running::spawn(&[]);
    engine.wait_until_ready().await;
    send_logs(&engine, SERVICE, MARKER, 1).await;
    engine
        .wait_for_log("the log record is stored", |lines| {
            appended(lines, "log_records") >= 1
        })
        .await;

    let answer = ask(&engine.socket(), tool_call(1, "query_logs", json!({}))).await;
    let items = answer["result"]["items"]
        .as_array()
        .unwrap_or_else(|| panic!("a result with items: {answer}"));
    assert!(
        items.iter().any(|row| row["body"] == MARKER),
        "the door returns the record the receiver took: {answer}"
    );

    let lines = engine.end_by_sigterm().await;

    let boot = the_one_door_boot_record(&lines);
    assert_eq!(boot["level"], "INFO");
    assert_eq!(boot["fields"]["state"], "listening");
    assert_eq!(boot["fields"]["resolve_write"], "on");

    let calls = records(&lines, "door.call");
    assert_eq!(calls.len(), 1, "exactly one call record: {calls:?}");
    let call = calls[0];
    assert_eq!(call["level"], "INFO");
    assert_eq!(
        own_fields(call),
        BTreeSet::from(CALL_FIELDS.map(String::from)),
        "every field of the call record, and no other"
    );
    assert_nothing_redacted(call);
    assert_eq!(call["fields"]["method"], "tools/call");
    assert_eq!(call["fields"]["tool"], "query_logs");
    assert_eq!(call["fields"]["outcome"], "ok");
    assert_eq!(call["fields"]["write"], false);
    assert_eq!(call["fields"]["result_type"], "paginated_logs");
    assert_eq!(call["fields"]["result_count"], items.len() as u64);
    assert!(call["fields"]["answer_bytes"].as_u64() > Some(MARKER.len() as u64));
    assert!(call["fields"]["duration_ms"].is_u64());
    assert!(records(&lines, "door.refused").is_empty());

    let log = family_text(engine.data_dir.path());
    assert_eq!(
        log.matches(MARKER).count(),
        0,
        "what the door returned is not in the log"
    );
    let full = engine.data_dir.path().to_string_lossy().to_string();
    let resolved = engine
        .data_dir
        .path()
        .canonicalize()
        .expect("the data dir resolves")
        .to_string_lossy()
        .to_string();
    for path in [&full, &resolved] {
        assert_eq!(
            log.matches(path.as_str()).count(),
            0,
            "the data dir's full path is not in the log"
        );
    }
    assert_no_log_record_in(&engine.stream("stderr"), "stderr");
    assert_no_log_record_in(&engine.stream("stdout"), "stdout");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_request_outside_the_tool_set_is_refused_beside_an_answered_call() {
    const UNKNOWN_TOOL: &str = "read_the_sender_credential_canary_19ce";
    const UNDECLARED: &str = "undeclared_member_canary_6a0d";
    const UNKNOWN_METHOD: &str = "resources/read_canary_e2b4";

    let mut engine = Running::spawn(&[]);
    engine.wait_until_ready().await;
    let socket = engine.socket();

    // The positive arm: a call inside the set is answered.
    let listed = ask(&socket, tool_call(1, "query_traces", json!({ "limit": 5 }))).await;
    assert!(
        listed["result"]["items"].is_array(),
        "a call inside the tool set is answered: {listed}"
    );

    let unknown_tool = ask(&socket, tool_call(2, UNKNOWN_TOOL, json!({}))).await;
    assert_eq!(unknown_tool["error"]["code"], -32601, "{unknown_tool}");
    let undeclared = ask(
        &socket,
        tool_call(3, "query_traces", json!({ "limit": 5, UNDECLARED: "x" })),
    )
    .await;
    assert_eq!(undeclared["error"]["code"], -32602, "{undeclared}");
    let unknown_method = ask(
        &socket,
        json!({ "jsonrpc": "2.0", "id": 4, "method": UNKNOWN_METHOD, "params": {} }),
    )
    .await;
    assert_eq!(unknown_method["error"]["code"], -32601, "{unknown_method}");
    for refused in [&unknown_tool, &undeclared, &unknown_method] {
        assert!(refused.get("result").is_none(), "{refused}");
        let text = refused.to_string();
        for canary in [UNKNOWN_TOOL, UNDECLARED, UNKNOWN_METHOD] {
            assert!(
                !text.contains(canary),
                "the answer repeats `{canary}`: {text}"
            );
        }
    }

    let lines = engine.end_by_sigterm().await;

    let outcomes: Vec<(&str, &str)> = records(&lines, "door.call")
        .iter()
        .map(|call| {
            assert_nothing_redacted(call);
            (
                call["fields"]["outcome"].as_str().expect("outcome"),
                call["level"].as_str().expect("level"),
            )
        })
        .collect();
    assert_eq!(
        outcomes,
        [
            ("ok", "INFO"),
            ("unknown_tool", "WARN"),
            ("invalid_arguments", "WARN"),
            ("unknown_method", "WARN"),
        ],
        "one record per request, a refusal at WARN with its closed label"
    );
    let log = family_text(engine.data_dir.path());
    for canary in [UNKNOWN_TOOL, UNDECLARED, UNKNOWN_METHOD] {
        assert_eq!(
            log.matches(canary).count(),
            0,
            "no caller-supplied text in the log"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_answer_over_the_size_bound_is_refused_and_the_same_query_at_a_small_limit_is_answered()
{
    const SERVICE: &str = "door-size-bound-svc";
    const COUNT: usize = 1000;
    // Over the bound by construction: a thousand rows of this body alone pass
    // 1 048 576 bytes.
    let body = "door size bound filler ".repeat(53);
    assert!(body.len() * COUNT > 1_048_576);

    let mut engine = Running::spawn(&[]);
    engine.wait_until_ready().await;
    send_logs(&engine, SERVICE, &body, COUNT).await;
    engine
        .wait_for_log("the log records are stored", |lines| {
            appended(lines, "log_records") >= COUNT as u64
        })
        .await;
    let socket = engine.socket();

    let cut = ask(
        &socket,
        tool_call(1, "query_logs", json!({ "limit": COUNT })),
    )
    .await;
    assert_eq!(cut["error"]["code"], -32003, "{cut}");
    assert_eq!(
        cut["error"]["message"],
        "answer too large: the bound is 1048576 bytes"
    );
    assert!(cut.get("result").is_none(), "no part of the result is sent");

    let small = ask(&socket, tool_call(2, "query_logs", json!({ "limit": 5 }))).await;
    let items = small["result"]["items"]
        .as_array()
        .unwrap_or_else(|| panic!("the same query at a small limit is answered: {small}"));
    assert_eq!(items.len(), 5);
    assert_eq!(items[0]["body"], body);

    let lines = engine.end_by_sigterm().await;

    let calls = records(&lines, "door.call");
    assert_eq!(calls.len(), 2, "one record per call: {calls:?}");
    assert_eq!(calls[0]["level"], "WARN");
    assert_eq!(calls[0]["fields"]["outcome"], "answer_too_large");
    assert_eq!(calls[0]["fields"]["tool"], "query_logs");
    assert!(
        calls[0]["fields"]["answer_bytes"].as_u64() < Some(1024),
        "the record counts the error line, not the result that was cut"
    );
    assert_eq!(calls[1]["level"], "INFO");
    assert_eq!(calls[1]["fields"]["outcome"], "ok");
    assert_eq!(calls[1]["fields"]["result_count"], 5);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_write_switch_is_read_from_the_environment_and_a_value_outside_the_set_leaves_it_off() {
    const OUTSIDE: &str = "door-switch-value-canary-9f2b";
    let resolve = |id: u64| tool_call(id, "mark_incident_resolved", json!({ "incident_id": 4242 }));

    // The positive arm: with the variable unset the write reaches the tool,
    // which finds no such incident.
    let mut on = Running::spawn(&[]);
    on.wait_until_ready().await;
    let reached = ask(&on.socket(), resolve(1)).await;
    assert_eq!(reached["error"]["code"], -32603, "{reached}");
    assert_eq!(reached["error"]["message"], "tool failed");
    let lines = on.end_by_sigterm().await;
    assert_eq!(
        the_one_door_boot_record(&lines)["fields"]["resolve_write"],
        "on"
    );
    let calls = records(&lines, "door.call");
    assert_eq!(calls.len(), 1, "{calls:?}");
    assert_eq!(calls[0]["fields"]["outcome"], "tool_error");
    assert_eq!(calls[0]["fields"]["write"], true);

    for (value, label, level) in [("off", "off", "INFO"), (OUTSIDE, "rejected", "WARN")] {
        let mut engine = Running::spawn(&[("ANDROMEDA_PULSE_DOOR_RESOLVE", value)]);
        engine.wait_until_ready().await;
        let socket = engine.socket();

        let refused = ask(&socket, resolve(1)).await;
        assert_eq!(refused["error"]["code"], -32001, "`{value}`: {refused}");
        let listed = ask(
            &socket,
            json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }),
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
            "`{value}`: the list still names the write"
        );

        let lines = engine.end_by_sigterm().await;

        let boot = the_one_door_boot_record(&lines);
        assert_eq!(boot["fields"]["state"], "listening", "`{value}`");
        assert_eq!(boot["fields"]["resolve_write"], label, "`{value}`");
        assert_eq!(boot["level"], level, "`{value}`");
        let calls = records(&lines, "door.call");
        assert_eq!(calls.len(), 2, "`{value}`: {calls:?}");
        assert_eq!(calls[0]["fields"]["outcome"], "write_off", "`{value}`");
        assert_eq!(calls[0]["fields"]["write"], true, "`{value}`");
        assert_eq!(calls[0]["level"], "WARN", "`{value}`");
        assert_eq!(
            family_text(engine.data_dir.path()).matches(OUTSIDE).count(),
            0,
            "the variable's value is not in the log"
        );
    }
}

// ── another account ─────────────────────────────────────────────────────────

const SKIP_NO_SECOND_ACCOUNT: &str = "door other-account arm: skipped, no second account";
const THREE_LAYERS: &str =
    "door other-account arm: directory refused, socket mode refused, account check refused";

// Connects, asks one thing and prints what it got: `connect-error {errno}`
// when the kernel refused the connection, `closed` when the door ended it
// with no byte, `answered` when any byte came back.
const CLIENT: &str = r#"
import socket, sys
s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
s.settimeout(20)
try:
    s.connect(sys.argv[1])
except OSError as e:
    print("connect-error", e.errno if e.errno is not None else 0)
    sys.exit(0)
try:
    s.sendall(b'{"jsonrpc":"2.0","id":1,"method":"ping"}\n')
    got = s.recv(64)
except OSError:
    got = b""
print("answered" if got else "closed")
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reading {
    /// The kernel refused the connection, with this `errno`.
    Kernel(i32),
    /// The door took the connection and closed it with no byte.
    Closed,
    Answered,
}

fn sudo(account: Option<&str>) -> Command {
    let mut command = Command::new("sudo");
    command.arg("-n");
    if let Some(account) = account {
        command.args(["-u", account]);
    }
    command.stdin(Stdio::null());
    command
}

/// Runs the one client as `account` (root when `None`). Nothing else is run
/// as another account.
fn client_as(account: Option<&str>, socket: &Path) -> Reading {
    let output = sudo(account)
        .args(["python3", "-c", CLIENT])
        .arg(socket)
        .output()
        .expect("run the client under sudo");
    let printed = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert!(
        output.status.success(),
        "the client ran as {account:?}: {printed} / {}",
        String::from_utf8_lossy(&output.stderr)
    );
    match printed.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["connect-error", errno] => Reading::Kernel(errno.parse().expect("an errno")),
        ["closed"] => Reading::Closed,
        ["answered"] => Reading::Answered,
        other => panic!("an unreadable client line: {other:?}"),
    }
}

fn set_mode(path: &Path, mode: u32) {
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
        .unwrap_or_else(|e| panic!("set mode {mode:o} on {}: {e}", path.display()));
}

/// Whether every directory above `path` can be passed through by an account
/// that is neither its owner nor in its group.
fn reachable_by_another_account(path: &Path) -> bool {
    path.ancestors()
        .skip(1)
        .all(|dir| std::fs::metadata(dir).is_ok_and(|meta| meta.permissions().mode() & 0o001 != 0))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn another_account_is_refused_at_each_of_three_layers() {
    const OTHER: &str = "nobody";
    const OWNER_SERVICE: &str = "door-other-account-arm-svc";
    const OWNER_MARKER: &str = "door other account arm marker 4d7a read back by the owner";
    let has_second_account = sudo(None)
        .arg("true")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if !has_second_account {
        println!("{SKIP_NO_SECOND_ACCOUNT}");
        return;
    }

    let mut engine = Running::spawn(&[]);
    engine.wait_until_ready().await;
    let socket = engine
        .socket()
        .canonicalize()
        .expect("the door's socket resolves");
    let door_dir = socket.parent().expect("the door dir").to_path_buf();
    let run_dir = door_dir.parent().expect("the run dir").to_path_buf();
    let data_dir = run_dir.parent().expect("the data dir").to_path_buf();

    // As the engine's owner: the way to the door directory is opened, the
    // door directory and the socket are left as the engine made them.
    set_mode(&data_dir, 0o755);
    set_mode(&run_dir, 0o755);
    if !reachable_by_another_account(&data_dir) {
        println!("door other-account arm: skipped, the temp dir is closed to another account");
        engine.end_by_sigterm().await;
        return;
    }
    assert_eq!(
        std::fs::metadata(&door_dir)
            .expect("door dir")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );

    let read = |socket: &Path| [client_as(Some(OTHER), socket), client_as(None, socket)];
    // Layer one: the door directory as made, 0700.
    let directory = read(&socket);
    // Layer two: the directory opened, the socket as made, 0600.
    set_mode(&door_dir, 0o755);
    let socket_mode = read(&socket);
    // Layer three: the socket opened too; only the account check is left.
    set_mode(&socket, 0o666);
    let account_check = read(&socket);
    println!(
        "door other-account arm: readings as [{OTHER}, root]: directory 0700 {directory:?}, \
         socket 0600 {socket_mode:?}, both opened {account_check:?}"
    );

    // The owner, in the same run, is answered with what the engine took.
    send_logs(&engine, OWNER_SERVICE, OWNER_MARKER, 1).await;
    engine
        .wait_for_log("the owner's log record is stored", |lines| {
            appended(lines, "log_records") >= 1
        })
        .await;
    let owner = ask(&socket, tool_call(1, "query_logs", json!({}))).await;
    assert!(
        owner["result"]["items"]
            .as_array()
            .is_some_and(|items| items.iter().any(|row| row["body"] == OWNER_MARKER)),
        "the owner's call is answered with the record the receiver took: {owner}"
    );

    let every = [directory, socket_mode, account_check].concat();
    assert!(
        !every.contains(&Reading::Answered),
        "no client of another account got a byte: {every:?}"
    );
    assert!(
        matches!(directory[0], Reading::Kernel(_)),
        "the closed directory refuses another account's connect: {directory:?}"
    );
    assert_eq!(
        account_check,
        [Reading::Closed, Reading::Closed],
        "with both outer layers opened the door itself refuses, root included"
    );

    let lines = engine.end_by_sigterm().await;

    let reached_the_door = every
        .iter()
        .filter(|reading| **reading == Reading::Closed)
        .count();
    let refusals = records(&lines, "door.refused");
    assert_eq!(
        refusals.len(),
        reached_the_door,
        "one refusal record per client that reached the door: {refusals:?}"
    );
    for refusal in &refusals {
        assert_nothing_redacted(refusal);
        assert_eq!(refusal["level"], "WARN");
        assert_eq!(refusal["fields"]["reason"], "other_account");
        assert_eq!(refusal["fields"]["peer"], "other");
    }
    let calls = records(&lines, "door.call");
    assert_eq!(calls.len(), 1, "the owner's call and no other: {calls:?}");
    assert_eq!(calls[0]["fields"]["tool"], "query_logs");
    assert_eq!(calls[0]["fields"]["outcome"], "ok");

    let socket_mode_refused = matches!(socket_mode[0], Reading::Kernel(_));
    if socket_mode_refused {
        println!("{THREE_LAYERS}");
    } else {
        println!(
            "door other-account arm: directory refused, socket mode not refused, account check refused"
        );
    }
}

// ── the stdio relay, `andromeda-pulse-engine door` ──────────────────────────

const BRIDGE_UNREACHABLE: &str = "andromeda-pulse-engine door: the door cannot be reached";
const SKIP_BRIDGE_NO_SECOND_ACCOUNT: &str =
    "door bridge other-account arm: skipped, no second account";
const BRIDGE_REFUSED: &str = "door bridge other-account arm: no byte reached another account through the bridge, the account check refused";

struct Relayed {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: String,
}

/// Writes `input` to a spawned relay's standard input, closes it and waits.
fn relay_through(mut command: Command, input: &str) -> Relayed {
    let streams = tempfile::tempdir().expect("streams dir");
    let stderr_path = streams.path().join("stderr");
    let stderr = std::fs::File::create(&stderr_path).expect("stderr file");
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(stderr)
        .spawn()
        .expect("spawn the relay");
    let mut stdin = child.stdin.take().expect("stdin");
    // A relay that ends before it reads leaves this write refused: no finding.
    let _ = stdin.write_all(input.as_bytes());
    drop(stdin);
    let output = child.wait_with_output().expect("the relay ends");
    Relayed {
        status: output.status,
        stdout: output.stdout,
        stderr: std::fs::read_to_string(&stderr_path).expect("read the relay's stderr"),
    }
}

/// `andromeda-pulse-engine door` as a stdio agent starts it on the engine's
/// data dir: a cleared environment holding the data dir alone, and a working
/// directory outside the data dir and the checkout.
fn owner_relay(data_dir: &Path, input: &str) -> Relayed {
    let cwd = tempfile::tempdir().expect("relay working dir");
    let mut command = Command::new(ENGINE);
    command
        .env_clear()
        .env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .current_dir(cwd.path())
        .arg("door");
    if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile);
    }
    relay_through(command, input)
}

/// Every path under `dir` but the log family, relative to it.
fn file_set(dir: &Path) -> BTreeSet<PathBuf> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeSet<PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("read a dir").flatten() {
            let path = entry.path();
            let relative = path
                .strip_prefix(root)
                .expect("under the root")
                .to_path_buf();
            let in_log_family = relative.starts_with("logs")
                && relative
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("agent-latest.jsonl"));
            if !in_log_family {
                out.insert(relative);
            }
            if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                walk(root, &path, out);
            }
        }
    }
    let mut out = BTreeSet::new();
    walk(dir, dir, &mut out);
    out
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_stdio_agent_reaches_the_door_through_the_bridge_and_leaves_one_call_per_request() {
    const SERVICE: &str = "door-bridge-arm-svc";
    const MARKER: &str = "door bridge arm marker 51c8 read back through the relay";

    let mut engine = Running::spawn(&[("ANDROMEDA_PULSE_DOOR_RESOLVE", "off")]);
    engine.wait_until_ready().await;
    send_logs(&engine, SERVICE, MARKER, 1).await;
    engine
        .wait_for_log("the log record is stored", |lines| {
            appended(lines, "log_records") >= 1
        })
        .await;

    let list_request = json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" });
    let requests = [
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "door-bridge-arm", "version": "0" }
            }
        }),
        json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
        list_request.clone(),
        tool_call(3, "query_logs", json!({})),
        tool_call(4, "mark_incident_resolved", json!({ "incident_id": 1 })),
    ];
    let input: String = requests
        .iter()
        .map(|request| format!("{request}\n"))
        .collect();
    let before = file_set(engine.data_dir.path());

    let relayed = owner_relay(engine.data_dir.path(), &input);

    let after = file_set(engine.data_dir.path());
    assert_eq!(relayed.status.code(), Some(0), "standard input ended");
    assert!(relayed.stderr.is_empty(), "stderr: {}", relayed.stderr);
    let stdout = String::from_utf8(relayed.stdout).expect("the answers are UTF-8");
    let answer_lines: Vec<&str> = stdout.split_inclusive('\n').collect();
    assert_eq!(
        answer_lines.len(),
        4,
        "one line per request with an id, none for the notification: {stdout}"
    );
    let answers: Vec<Value> = answer_lines
        .iter()
        .map(|line| serde_json::from_str(line).expect("each line is one JSON object"))
        .collect();
    assert_eq!(answers[0]["id"], 1);
    assert_eq!(
        answers[0]["result"]["serverInfo"]["name"],
        "andromeda-pulse-mcp"
    );
    assert_eq!(answers[0]["result"]["protocolVersion"], "2024-11-05");
    let tools = answers[1]["result"]["tools"]
        .as_array()
        .unwrap_or_else(|| panic!("a tool list: {}", answers[1]));
    assert_eq!(tools.len(), 9, "the nine tools");
    let writes: Vec<&str> = tools
        .iter()
        .filter(|tool| tool["write"] == true)
        .filter_map(|tool| tool["name"].as_str())
        .collect();
    assert_eq!(writes, ["mark_incident_resolved"], "the one write");
    assert!(
        answers[2]["result"]["items"]
            .as_array()
            .is_some_and(|items| items.iter().any(|row| row["body"] == MARKER)),
        "the relay returns the record the receiver took: {}",
        answers[2]
    );
    assert_eq!(answers[3]["error"]["code"], -32001, "{}", answers[3]);
    assert_eq!(before, after, "the relay adds nothing to the data dir");
    let pid = std::fs::read_to_string(engine.data_dir.path().join("run/andromeda-pulse.pid"))
        .expect("the pid file");
    assert_eq!(
        pid.trim(),
        engine.child.id().to_string(),
        "the engine's pid"
    );

    // The same request straight from the socket: the relay's line unchanged.
    let direct = ask_line(&engine.socket(), list_request).await;
    assert_eq!(answer_lines[1], direct, "byte for byte");

    let lines = engine.end_by_sigterm().await;

    let calls = records(&lines, "door.call");
    let shapes: Vec<(&str, &str, &str)> = calls
        .iter()
        .map(|call| {
            assert_nothing_redacted(call);
            assert_eq!(
                own_fields(call),
                BTreeSet::from(CALL_FIELDS.map(String::from)),
                "every field of the call record, and no other"
            );
            (
                call["fields"]["method"].as_str().expect("method"),
                call["fields"]["tool"].as_str().expect("tool"),
                call["fields"]["outcome"].as_str().expect("outcome"),
            )
        })
        .collect();
    assert_eq!(
        shapes,
        [
            ("initialize", "none", "ok"),
            ("tools/list", "none", "ok"),
            ("tools/call", "query_logs", "ok"),
            ("tools/call", "mark_incident_resolved", "write_off"),
            ("tools/list", "none", "ok"),
        ],
        "one record per answered request: the relay's four, then the direct one"
    );
    assert!(
        records(&lines, "door.refused").is_empty(),
        "one connection per request leaves no idle refusal"
    );
    assert_eq!(records(&lines, "app.boot.engine").len(), 1);
    assert_eq!(
        family_text(engine.data_dir.path()).matches(MARKER).count(),
        0,
        "what the relay returned is not in the log"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn another_account_through_the_bridge_receives_nothing() {
    const OTHER: &str = "nobody";
    let has_second_account = sudo(None)
        .arg("true")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if !has_second_account {
        println!("{SKIP_BRIDGE_NO_SECOND_ACCOUNT}");
        return;
    }

    let mut engine = Running::spawn(&[]);
    engine.wait_until_ready().await;
    let socket = engine
        .socket()
        .canonicalize()
        .expect("the door's socket resolves");
    let door_dir = socket.parent().expect("the door dir").to_path_buf();
    let run_dir = door_dir.parent().expect("the run dir").to_path_buf();
    let data_dir = run_dir.parent().expect("the data dir").to_path_buf();
    set_mode(&data_dir, 0o755);
    set_mode(&run_dir, 0o755);

    // Another account runs a copy from a directory of the arm's own, so the
    // checkout's modes do not decide whether it can be executed.
    let own = tempfile::tempdir().expect("the arm's own dir");
    set_mode(own.path(), 0o755);
    let copy = own.path().join("andromeda-pulse-engine");
    std::fs::copy(ENGINE, &copy).expect("copy the built program");
    set_mode(&copy, 0o755);
    if !(reachable_by_another_account(&data_dir) && reachable_by_another_account(&copy)) {
        println!(
            "door bridge other-account arm: skipped, the temp dir is closed to another account"
        );
        engine.end_by_sigterm().await;
        return;
    }

    let ping = format!(
        "{}\n",
        json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" })
    );
    let as_account = |account: Option<&str>| {
        let mut command = sudo(account);
        command
            .arg("env")
            .arg(format!("ANDROMEDA_PULSE_DATA_DIR={}", data_dir.display()))
            .arg(&copy)
            .arg("door")
            .current_dir(own.path());
        relay_through(command, &ping)
    };
    let read = || [as_account(Some(OTHER)), as_account(None)];
    // As made: the door directory 0700, the socket 0600.
    let as_made = read();
    // Opened: only the account check is left.
    set_mode(&door_dir, 0o755);
    set_mode(&socket, 0o666);
    let opened = read();
    // The owner, in the same run and with no sudo, is answered.
    let owner = owner_relay(&data_dir, &ping);

    let readings = [
        ("nobody as made", &as_made[0]),
        ("root as made", &as_made[1]),
        ("nobody opened", &opened[0]),
        ("root opened", &opened[1]),
    ];
    for (case, reading) in readings {
        println!(
            "door bridge other-account arm: {case} exit {:?}, {} byte(s) on stdout",
            reading.status.code(),
            reading.stdout.len()
        );
        assert!(
            reading.stdout.is_empty(),
            "{case}: no byte reached another account"
        );
        // A connection the door closes unread reads as a failed write or read
        // on this kernel: the relay ends unreachable.
        assert_eq!(reading.status.code(), Some(1), "{case}: {}", reading.stderr);
        assert!(
            reading
                .stderr
                .lines()
                .any(|line| line == BRIDGE_UNREACHABLE),
            "{case}: the fixed line, as one line of stderr: {}",
            reading.stderr
        );
    }
    assert_eq!(owner.status.code(), Some(0), "the owner: {}", owner.stderr);
    let pong: Value = serde_json::from_slice(&owner.stdout).expect("the owner's answer");
    assert_eq!(pong["id"], 1, "the owner is answered: {pong}");

    let lines = engine.end_by_sigterm().await;

    // Root passes the modes, so it reaches the door at both readings; `nobody`
    // only once both outer layers are opened.
    let refusals = records(&lines, "door.refused");
    assert_eq!(
        refusals.len(),
        3,
        "one refusal per reading that reached the door: {refusals:?}"
    );
    for refusal in &refusals {
        assert_nothing_redacted(refusal);
        assert_eq!(refusal["level"], "WARN");
        assert_eq!(refusal["fields"]["reason"], "other_account");
        assert_eq!(refusal["fields"]["peer"], "other");
    }
    let calls = records(&lines, "door.call");
    assert_eq!(calls.len(), 1, "the owner's ping and no other: {calls:?}");
    assert_eq!(calls[0]["fields"]["method"], "ping");
    assert_eq!(calls[0]["fields"]["outcome"], "ok");
    println!("{BRIDGE_REFUSED}");
}
