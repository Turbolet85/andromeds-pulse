//! The door's owner predicate, endpoint rule, request bounds, time bound and
//! write switch, in process.
//!
//! The endpoint arms bind `run/door/sock` under their own `TempDir`. The bound
//! arms seat a door on a `TempDir` with a tool stand-in in place of the
//! engine's stores, and talk to it over the socket as a caller does. The door
//! on a booted engine is `integration_engine_boot.rs`'s; on a spawned console
//! engine, `integration_door.rs`'s.
#![cfg(unix)]

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, mpsc};
use std::time::{Duration, Instant};

use mcp_server_crate::contract::Error as ToolError;
use mcp_server_crate::tools::ToolAnswer;
use pulse_app::door::{
    self, CALL_TIMEOUT, CODE_ANSWER_TOO_LARGE, CODE_TIMED_OUT, CODE_WRITE_OFF, EndpointState,
    IDLE_TIMEOUT, MAX_ANSWER_BYTES, MAX_CONNECTIONS, MAX_REQUEST_BYTES, ResolveWrite, Tools,
};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

const WAIT: Duration = Duration::from_secs(60);
const BLOCKING_TOOL: &str = "query_logs";
const LARGE_TOOL: &str = "generate_snapshot";

fn socket_path(data_dir: &Path) -> PathBuf {
    data_dir.join("run").join("door").join("sock")
}

fn mode(path: &Path) -> u32 {
    std::fs::symlink_metadata(path)
        .unwrap_or_else(|e| panic!("metadata of {}: {e}", path.display()))
        .permissions()
        .mode()
        & 0o777
}

// ── the owner predicate ─────────────────────────────────────────────────────

#[test]
fn the_owner_predicate_admits_the_engines_own_account_and_no_other() {
    let own = door::own_account();
    assert!(door::admits(own, own), "the engine's own account");
    assert!(
        !door::admits(own.wrapping_add(1), own),
        "a different account id"
    );
    assert!(door::admits(1000, 1000));
    assert!(!door::admits(1001, 1000), "a different account id");
    assert!(!door::admits(0, 1000), "account 0 is another account");
}

// ── the endpoint rule ───────────────────────────────────────────────────────

#[tokio::test]
async fn the_directory_reads_0700_and_the_socket_0600() {
    let dir = tempfile::tempdir().expect("tempdir");
    let _listener = door::bind_endpoint(dir.path()).expect("the endpoint binds");

    assert_eq!(mode(&dir.path().join("run").join("door")), 0o700);
    let socket = socket_path(dir.path());
    assert_eq!(mode(&socket), 0o600);
    assert!(
        std::fs::symlink_metadata(&socket)
            .expect("socket metadata")
            .file_type()
            .is_socket(),
        "`run/door/sock` is a socket"
    );
}

#[tokio::test]
async fn a_door_directory_left_wider_is_closed_before_the_bind() {
    let dir = tempfile::tempdir().expect("tempdir");
    let door_dir = dir.path().join("run").join("door");
    std::fs::create_dir_all(&door_dir).expect("create the door dir");
    std::fs::set_permissions(&door_dir, std::fs::Permissions::from_mode(0o755))
        .expect("widen the door dir");

    let _listener = door::bind_endpoint(dir.path()).expect("the endpoint binds");

    assert_eq!(mode(&door_dir), 0o700);
}

#[tokio::test]
async fn a_data_dir_too_deep_reads_path_too_long() {
    let dir = tempfile::tempdir().expect("tempdir");
    let base = dir.path().canonicalize().expect("canonical tempdir");
    let deep = base.join("d".repeat(120));
    std::fs::create_dir(&deep).expect("create the deep data dir");

    let state = door::bind_endpoint(&deep).expect_err("the socket's address does not fit");

    assert_eq!(state, EndpointState::PathTooLong);
    assert!(
        !socket_path(&deep).exists(),
        "nothing was bound at the too-long path"
    );

    // The boundary, where the temp dir is shallow enough to build it: an
    // address of 107 bytes binds and one of 108 does not.
    let fixed = base.as_os_str().len() + "/".len() + "/run/door/sock".len();
    let Some(fill) = 107usize.checked_sub(fixed).filter(|fill| *fill >= 1) else {
        eprintln!("path-length boundary: not built, the temp dir is {fixed} bytes deep");
        return;
    };
    let fits = base.join("f".repeat(fill));
    std::fs::create_dir(&fits).expect("create the 107-byte data dir");
    assert_eq!(socket_path(&fits).as_os_str().len(), 107);
    let _listener = door::bind_endpoint(&fits).expect("an address of 107 bytes binds");
    let over = base.join("o".repeat(fill + 1));
    std::fs::create_dir(&over).expect("create the 108-byte data dir");
    assert_eq!(socket_path(&over).as_os_str().len(), 108);
    assert_eq!(
        door::bind_endpoint(&over).expect_err("an address of 108 bytes does not fit"),
        EndpointState::PathTooLong
    );
}

#[tokio::test]
async fn a_regular_file_at_the_sockets_path_reads_occupied() {
    let dir = tempfile::tempdir().expect("tempdir");
    let socket = socket_path(dir.path());
    std::fs::create_dir_all(socket.parent().expect("door dir")).expect("create the door dir");
    std::fs::write(&socket, b"not a socket").expect("write a regular file");

    let state = door::bind_endpoint(dir.path()).expect_err("a regular file holds the path");

    assert_eq!(state, EndpointState::Occupied);
    assert_eq!(
        std::fs::read(&socket).expect("the file is still there"),
        b"not a socket",
        "the file was left as it was"
    );
}

#[tokio::test]
async fn a_live_listener_reads_in_use_and_keeps_listening() {
    let dir = tempfile::tempdir().expect("tempdir");
    let _first = door::bind_endpoint(dir.path()).expect("the first bind");

    let state = door::bind_endpoint(dir.path()).expect_err("a live listener holds the path");

    assert_eq!(state, EndpointState::InUse);
    assert!(
        std::os::unix::net::UnixStream::connect(socket_path(dir.path())).is_ok(),
        "the first listener still accepts"
    );
}

#[tokio::test]
async fn a_stale_socket_is_replaced() {
    let dir = tempfile::tempdir().expect("tempdir");
    let socket = socket_path(dir.path());
    drop(door::bind_endpoint(dir.path()).expect("the first bind"));
    assert!(
        socket.exists(),
        "the ended listener left its socket file behind"
    );
    assert!(
        std::os::unix::net::UnixStream::connect(&socket).is_err(),
        "nothing accepts on the stale socket"
    );

    let _second = door::bind_endpoint(dir.path()).expect("the stale socket is replaced");

    assert_eq!(mode(&socket), 0o600);
    assert!(
        std::os::unix::net::UnixStream::connect(&socket).is_ok(),
        "the new listener accepts"
    );
}

#[tokio::test]
async fn a_door_directory_that_leaves_the_data_dir_reads_bind_failed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let elsewhere = tempfile::tempdir().expect("a directory outside the data dir");
    std::fs::create_dir_all(dir.path().join("run")).expect("create run");
    std::os::unix::fs::symlink(elsewhere.path(), dir.path().join("run").join("door"))
        .expect("link the door dir out of the data dir");

    let state = door::bind_endpoint(dir.path()).expect_err("the door dir resolves outside");

    assert_eq!(state, EndpointState::BindFailed);
    assert!(
        std::fs::read_dir(elsewhere.path())
            .expect("read the outside directory")
            .next()
            .is_none(),
        "nothing was bound outside the data dir"
    );
}

// ── the write switch's reading ──────────────────────────────────────────────

#[test]
fn the_switch_reads_on_when_unset_or_on_off_when_off_and_rejected_otherwise() {
    let read = |value: &str| ResolveWrite::from_value(Some(OsStr::new(value)));
    assert_eq!(ResolveWrite::from_value(None), ResolveWrite::On);
    assert_eq!(read("on"), ResolveWrite::On);
    assert_eq!(read("off"), ResolveWrite::Off);
    for outside in [
        "", "ON", "Off", " off", "off ", "0", "1", "true", "no", "yes",
    ] {
        assert_eq!(read(outside), ResolveWrite::Rejected, "`{outside}`");
    }
    assert_eq!(
        ResolveWrite::from_value(Some(OsStr::from_bytes(&[0x6f, 0x6e, 0xff]))),
        ResolveWrite::Rejected,
        "a value that is not UTF-8"
    );

    assert!(ResolveWrite::On.allows_write());
    assert!(!ResolveWrite::Off.allows_write());
    assert!(
        !ResolveWrite::Rejected.allows_write(),
        "a rejected value leaves the write off"
    );
    assert_eq!(
        [
            ResolveWrite::On.label(),
            ResolveWrite::Off.label(),
            ResolveWrite::Rejected.label()
        ],
        ["on", "off", "rejected"]
    );
}

#[test]
fn the_bounds_and_the_three_codes_read_as_stated() {
    assert_eq!(MAX_REQUEST_BYTES, 65_536);
    assert_eq!(MAX_ANSWER_BYTES, 1_048_576);
    assert_eq!(CALL_TIMEOUT, Duration::from_secs(5));
    assert_eq!(IDLE_TIMEOUT, Duration::from_secs(30));
    assert_eq!(MAX_CONNECTIONS, 8);
    assert_eq!(
        [CODE_WRITE_OFF, CODE_TIMED_OUT, CODE_ANSWER_TOO_LARGE],
        [-32001, -32002, -32003]
    );
}

// ── the process-wide capture ────────────────────────────────────────────────

/// One captured door record: `(target, level, field name -> value)`.
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
        let target = event.metadata().target();
        if !(target == "app.boot.door" || target.starts_with("door.")) {
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
            target.to_string(),
            *event.metadata().level(),
            fields.0,
        ));
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}

/// Whether the capture holds a record of `target` carrying every pair of
/// `signature`. The arms of this binary share one capture under libtest, so a
/// signature is chosen that one arm alone produces.
fn captured(events: &Captured, target: &str, signature: &[(&str, &str)]) -> bool {
    events
        .lock()
        .expect("lock")
        .iter()
        .any(|(seen, _, fields)| {
            seen == target
                && signature
                    .iter()
                    .all(|(name, value)| fields.get(*name).map(String::as_str) == Some(*value))
        })
}

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

// ── a door seated in process over a tool stand-in ───────────────────────────

/// Answers every tool with one small result, except that the first call of
/// `BLOCKING_TOOL` waits until the arm releases it, and `LARGE_TOOL` returns a
/// result over the answer bound.
struct StandIn {
    calls: AtomicUsize,
    interrupts: AtomicUsize,
    block: Mutex<Option<mpsc::Receiver<()>>>,
}

impl Tools for StandIn {
    fn call(&self, tool: &'static str, arguments: &Value) -> Result<ToolAnswer, ToolError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        // Taken in a statement of its own: the lock is not held while waiting.
        let release = (tool == BLOCKING_TOOL)
            .then(|| self.block.lock().expect("lock").take())
            .flatten();
        if let Some(release) = release {
            // Returns when the arm drops its sender, or at the bound.
            let _ = release.recv_timeout(WAIT);
        }
        if arguments.get("limit").is_some_and(Value::is_string) {
            return Err(ToolError::ToolArgsInvalid {
                tool_name: tool.to_string(),
                reason: "invalid type: string \"limit-canary\", expected u32".to_string(),
            });
        }
        if arguments
            .get("cursor")
            .is_some_and(|cursor| cursor == "fail")
        {
            return Err(ToolError::ToolDispatchFailed {
                tool_name: tool.to_string(),
                reason: "stand-in failure at /a/host/path".to_string(),
            });
        }
        let value = if tool == LARGE_TOOL {
            json!({ "markdown": "m".repeat(MAX_ANSWER_BYTES) })
        } else {
            json!({ "items": [{ "tool": tool }], "total": 1 })
        };
        Ok(ToolAnswer {
            value,
            result_type: "stand_in",
            result_count: 1,
        })
    }

    fn interrupt(&self) {
        self.interrupts.fetch_add(1, Ordering::SeqCst);
    }
}

struct Seated {
    tools: Arc<StandIn>,
    socket: PathBuf,
    // Dropped at the arm's end: releases a stand-in call still waiting.
    _release: mpsc::Sender<()>,
    _dir: tempfile::TempDir,
}

fn seat(resolve: ResolveWrite) -> Seated {
    let dir = tempfile::tempdir().expect("tempdir");
    let (release, block) = mpsc::channel();
    let tools = Arc::new(StandIn {
        calls: AtomicUsize::new(0),
        interrupts: AtomicUsize::new(0),
        block: Mutex::new(Some(block)),
    });
    let seated = door::open(dir.path(), Arc::clone(&tools) as Arc<dyn Tools>, resolve)
        .expect("the door binds on a fresh TempDir");
    tokio::spawn(seated.serve());
    Seated {
        tools,
        socket: socket_path(dir.path()),
        _release: release,
        _dir: dir,
    }
}

struct Caller {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
}

impl Caller {
    async fn connect(socket: &Path) -> Self {
        let stream = UnixStream::connect(socket)
            .await
            .expect("the door's socket accepts a connection");
        let (read_half, writer) = stream.into_split();
        Self {
            reader: BufReader::new(read_half),
            writer,
        }
    }

    /// The line and its terminator in one write. A door that has already
    /// closed the connection fails the write; the arm then reads the close.
    async fn send(&mut self, line: &[u8]) {
        let mut framed = Vec::with_capacity(line.len() + 1);
        framed.extend_from_slice(line);
        framed.push(b'\n');
        let _ = self.writer.write_all(&framed).await;
    }

    /// The next answer line, or `None` when the door closed the connection.
    async fn next(&mut self) -> Option<Value> {
        let mut line = String::new();
        let read = tokio::time::timeout(WAIT, self.reader.read_line(&mut line))
            .await
            .unwrap_or_else(|_| panic!("the door answers or closes within {WAIT:?}"));
        match read {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(serde_json::from_str(&line).expect("an answer line is one JSON object")),
        }
    }

    async fn ask(&mut self, request: Value) -> Value {
        self.send(request.to_string().as_bytes()).await;
        self.next().await.expect("the request is answered")
    }
}

fn tool_call(id: u64, name: &str, arguments: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "tools/call",
        "params": { "name": name, "arguments": arguments }
    })
}

fn ping(id: u64) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "method": "ping" })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_four_methods_are_answered_and_a_tool_call_carries_the_tools_result() {
    let events = global_capture();
    let seated = seat(ResolveWrite::On);
    let mut caller = Caller::connect(&seated.socket).await;

    let initialized = caller
        .ask(json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} }))
        .await;
    assert_eq!(initialized["id"], 1);
    assert!(initialized["result"]["protocolVersion"].is_string());

    let listed = caller
        .ask(json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }))
        .await;
    assert_eq!(
        listed["result"]["tools"].as_array().expect("tools").len(),
        9
    );

    assert_eq!(caller.ask(ping(3)).await["result"], json!({}));

    let called = caller
        .ask(tool_call(4, "query_traces", json!({ "limit": 5 })))
        .await;
    assert_eq!(called["id"], 4);
    assert_eq!(called["result"]["items"][0]["tool"], "query_traces");
    assert_eq!(seated.tools.calls.load(Ordering::SeqCst), 1);

    assert!(
        captured(
            &events,
            "door.call",
            &[
                ("method", "tools/call"),
                ("tool", "query_traces"),
                ("outcome", "ok"),
                ("write", "false"),
                ("result_type", "stand_in"),
                ("result_count", "1"),
            ]
        ),
        "the tool call left its record"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_request_outside_the_tool_set_is_refused_with_a_fixed_message_and_no_tool_is_called() {
    const UNKNOWN_TOOL: &str = "read_the_token_canary_5d1c";
    const UNKNOWN_METHOD: &str = "resources/read_canary_77aa";
    const UNDECLARED: &str = "undeclared_member_canary_c41e";
    let seated = seat(ResolveWrite::On);
    let mut caller = Caller::connect(&seated.socket).await;

    let unknown_tool = caller.ask(tool_call(1, UNKNOWN_TOOL, json!({}))).await;
    assert_eq!(unknown_tool["error"]["code"], -32601);
    assert_eq!(unknown_tool["error"]["message"], "unknown tool");

    let unknown_method = caller
        .ask(json!({ "jsonrpc": "2.0", "id": 2, "method": UNKNOWN_METHOD }))
        .await;
    assert_eq!(unknown_method["error"]["code"], -32601);
    assert_eq!(unknown_method["error"]["message"], "method not found");

    let no_name = caller
        .ask(json!({ "jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {} }))
        .await;
    assert_eq!(no_name["error"]["code"], -32602);

    caller.send(b"{ this is not json").await;
    let unparsed = caller.next().await.expect("a parse error is answered");
    assert_eq!(unparsed["error"]["code"], -32700);
    assert_eq!(unparsed["id"], Value::Null);

    assert_eq!(
        seated.tools.calls.load(Ordering::SeqCst),
        0,
        "none of the four reached a tool"
    );

    // An argument member the tool does not declare, and arguments that are
    // no object: refused by the door, with no tool called.
    let undeclared = caller
        .ask(tool_call(
            4,
            "query_traces",
            json!({ "limit": 5, UNDECLARED: 1 }),
        ))
        .await;
    assert_eq!(undeclared["error"]["code"], -32602);
    assert_eq!(undeclared["error"]["message"], "invalid arguments");
    let no_object = caller
        .ask(tool_call(5, "query_incident_list", json!([1, 2])))
        .await;
    assert_eq!(no_object["error"]["code"], -32602);
    assert_eq!(
        seated.tools.calls.load(Ordering::SeqCst),
        0,
        "none of the six reached a tool"
    );

    // A declared argument the tool rejects, and a tool that fails: the tool's
    // own error text stays inside the engine.
    let rejected = caller
        .ask(tool_call(
            6,
            "query_traces",
            json!({ "limit": "limit-canary" }),
        ))
        .await;
    assert_eq!(rejected["error"]["code"], -32602);
    assert_eq!(rejected["error"]["message"], "invalid arguments");
    let failed = caller
        .ask(tool_call(7, "query_traces", json!({ "cursor": "fail" })))
        .await;
    assert_eq!(failed["error"]["code"], -32603);
    assert_eq!(failed["error"]["message"], "tool failed");
    assert_eq!(seated.tools.calls.load(Ordering::SeqCst), 2);
    for answer in [
        &unknown_tool,
        &unknown_method,
        &undeclared,
        &rejected,
        &failed,
    ] {
        let text = answer.to_string();
        for absent in [
            UNKNOWN_TOOL,
            UNKNOWN_METHOD,
            UNDECLARED,
            "limit-canary",
            "/a/host/path",
        ] {
            assert!(!text.contains(absent), "`{absent}` in the answer {text}");
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_request_with_no_id_is_not_answered() {
    let seated = seat(ResolveWrite::On);
    let mut caller = Caller::connect(&seated.socket).await;

    caller
        .send(
            json!({ "jsonrpc": "2.0", "method": "tools/call", "params": { "name": "query_traces" } })
                .to_string()
                .as_bytes(),
        )
        .await;
    let next = caller.ask(ping(9)).await;

    assert_eq!(next["id"], 9, "the first answer line is the ping's");
    assert_eq!(
        seated.tools.calls.load(Ordering::SeqCst),
        0,
        "a notification runs no tool"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_write_is_refused_before_any_tool_runs_when_the_switch_is_not_on() {
    for resolve in [ResolveWrite::Off, ResolveWrite::Rejected] {
        let seated = seat(resolve);
        let mut caller = Caller::connect(&seated.socket).await;

        let refused = caller
            .ask(tool_call(
                1,
                "mark_incident_resolved",
                json!({ "incident_id": 1 }),
            ))
            .await;
        assert_eq!(refused["error"]["code"], CODE_WRITE_OFF, "{resolve:?}");
        assert_eq!(refused["error"]["message"], "the write is switched off");
        assert_eq!(
            seated.tools.calls.load(Ordering::SeqCst),
            0,
            "{resolve:?}: the tool was not called"
        );

        let listed = caller
            .ask(json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }))
            .await;
        let write_tools: Vec<&str> = listed["result"]["tools"]
            .as_array()
            .expect("tools")
            .iter()
            .filter(|tool| tool["write"] == true)
            .map(|tool| tool["name"].as_str().expect("name"))
            .collect();
        assert_eq!(
            write_tools,
            ["mark_incident_resolved"],
            "{resolve:?}: the list still names the write"
        );

        let read = caller.ask(tool_call(3, "query_traces", json!({}))).await;
        assert!(
            read["result"].is_object(),
            "{resolve:?}: a read is answered"
        );
    }

    let on = seat(ResolveWrite::On);
    let mut caller = Caller::connect(&on.socket).await;
    let applied = caller
        .ask(tool_call(
            1,
            "mark_incident_resolved",
            json!({ "incident_id": 1 }),
        ))
        .await;
    assert!(
        applied["result"].is_object(),
        "with the switch on the write runs"
    );
    assert_eq!(on.tools.calls.load(Ordering::SeqCst), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_answer_over_its_bound_is_an_error_naming_the_bound() {
    let events = global_capture();
    let seated = seat(ResolveWrite::On);
    let mut caller = Caller::connect(&seated.socket).await;

    let cut = caller.ask(tool_call(1, LARGE_TOOL, json!({}))).await;

    assert_eq!(cut["error"]["code"], CODE_ANSWER_TOO_LARGE);
    assert_eq!(
        cut["error"]["message"],
        "answer too large: the bound is 1048576 bytes"
    );
    assert!(cut.get("result").is_none(), "no part of the result is sent");
    let small = caller.ask(tool_call(2, "query_traces", json!({}))).await;
    assert!(small["result"].is_object(), "the next call is answered");
    assert!(captured(
        &events,
        "door.call",
        &[("tool", LARGE_TOOL), ("outcome", "answer_too_large")]
    ));
}

// ── the request bounds ──────────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_over_long_line_is_answered_invalid_request_and_the_connection_closed() {
    let events = global_capture();
    let seated = seat(ResolveWrite::On);

    // A line of exactly the bound is a request like any other.
    let frame =
        json!({ "jsonrpc": "2.0", "id": 1, "method": "ping", "params": { "pad": "" } }).to_string();
    let padded = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "ping",
        "params": { "pad": "p".repeat(MAX_REQUEST_BYTES - frame.len()) }
    })
    .to_string();
    assert_eq!(padded.len(), MAX_REQUEST_BYTES);
    let mut at_bound = Caller::connect(&seated.socket).await;
    at_bound.send(padded.as_bytes()).await;
    let answered = at_bound
        .next()
        .await
        .expect("a line at the bound is answered");
    assert_eq!(answered["result"], json!({}));

    let mut over = Caller::connect(&seated.socket).await;
    over.send(&vec![b'x'; MAX_REQUEST_BYTES + 1]).await;
    let refused = over.next().await.expect("an over-long line is answered");
    assert_eq!(refused["error"]["code"], -32600);
    assert_eq!(refused["id"], Value::Null);
    assert!(
        over.next().await.is_none(),
        "the door closed the connection after the answer"
    );

    assert!(captured(
        &events,
        "door.refused",
        &[("reason", "request_too_large"), ("peer", "owner")]
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_ninth_connection_is_closed_at_accept() {
    let events = global_capture();
    let seated = seat(ResolveWrite::On);

    let mut held = Vec::new();
    for id in 0..MAX_CONNECTIONS as u64 {
        let mut caller = Caller::connect(&seated.socket).await;
        assert_eq!(
            caller.ask(ping(id)).await["id"],
            id,
            "connection {id} is served"
        );
        held.push(caller);
    }

    let mut ninth = Caller::connect(&seated.socket).await;
    assert!(
        ninth.next().await.is_none(),
        "the ninth connection is closed with no byte"
    );
    assert!(captured(
        &events,
        "door.refused",
        &[("reason", "too_many_connections"), ("peer", "owner")]
    ));

    // A held connection that ends frees its place.
    held.pop();
    let deadline = Instant::now() + WAIT;
    loop {
        let mut again = Caller::connect(&seated.socket).await;
        again.send(ping(99).to_string().as_bytes()).await;
        if again.next().await.is_some() {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "a place is free again within {WAIT:?}"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_idle_connection_is_closed_at_the_idle_bound() {
    let events = global_capture();
    let seated = seat(ResolveWrite::On);
    let mut caller = Caller::connect(&seated.socket).await;
    assert_eq!(caller.ask(ping(1)).await["id"], 1);

    let silent_from = Instant::now();
    assert!(
        caller.next().await.is_none(),
        "the door closes the silent connection"
    );
    let silent = silent_from.elapsed();

    assert!(
        silent >= IDLE_TIMEOUT - Duration::from_secs(1) && silent < IDLE_TIMEOUT * 2,
        "closed at the idle bound, after {silent:?}"
    );
    wait_until("the idle refusal is recorded", || {
        captured(
            &events,
            "door.refused",
            &[("reason", "idle"), ("peer", "owner")],
        )
    })
    .await;
}

// ── the time bound ──────────────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_call_over_its_time_bound_answers_timed_out_and_the_next_call_is_answered() {
    const MARGIN: Duration = Duration::from_secs(5);
    let events = global_capture();
    let seated = seat(ResolveWrite::On);
    let mut caller = Caller::connect(&seated.socket).await;

    let asked = Instant::now();
    let timed_out = caller.ask(tool_call(1, BLOCKING_TOOL, json!({}))).await;
    let took = asked.elapsed();

    assert_eq!(timed_out["error"]["code"], CODE_TIMED_OUT);
    assert_eq!(timed_out["error"]["message"], "timed out");
    assert!(
        took >= CALL_TIMEOUT && took < CALL_TIMEOUT + MARGIN,
        "answered at the time bound, after {took:?}"
    );
    assert_eq!(
        seated.tools.interrupts.load(Ordering::SeqCst),
        1,
        "the door interrupted the call it gave up on"
    );

    let next = caller.ask(tool_call(2, "query_traces", json!({}))).await;
    assert!(next["result"].is_object(), "the next call is answered");
    let mut other = Caller::connect(&seated.socket).await;
    let elsewhere = other.ask(tool_call(3, BLOCKING_TOOL, json!({}))).await;
    assert!(
        elsewhere["result"].is_object(),
        "and so is a call on another connection"
    );
    assert_eq!(
        seated.tools.interrupts.load(Ordering::SeqCst),
        1,
        "no call within its bound was interrupted"
    );
    assert!(captured(
        &events,
        "door.call",
        &[("tool", BLOCKING_TOOL), ("outcome", "timed_out")]
    ));
}
