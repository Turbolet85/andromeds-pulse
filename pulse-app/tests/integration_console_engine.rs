//! The built console program, `andromeda-pulse-engine`, run with no display.
//!
//! Each arm spawns the binary under a CLEARED environment that holds only its
//! own `TempDir` data dir, two ports the test claimed and a corpus passphrase
//! canary (plus the coverage profile variable when this process has one): no
//! display variable, no session bus, no runtime dir, no home. The lock-file
//! arm alone also names a temp dir and a runtime dir of its own. Its working
//! directory is the data dir, so a file written relative to it would show up
//! there. Telemetry goes in over the real gRPC receiver, and the program's own
//! log family is read only after the process has ended.
//!
//! A child still running when an arm unwinds is killed on drop.

use std::collections::BTreeSet;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest;
use ingest::grpc::proto::opentelemetry::proto::collector::trace::v1::trace_service_client::TraceServiceClient;
use ingest::grpc::proto::opentelemetry::proto::common::v1::{AnyValue, KeyValue, any_value};
use ingest::grpc::proto::opentelemetry::proto::resource::v1::Resource;
use ingest::grpc::proto::opentelemetry::proto::trace::v1::{ResourceSpans, ScopeSpans, Span};
use serde_json::Value;

const ENGINE: &str = env!("CARGO_BIN_EXE_andromeda-pulse-engine");
const PASSPHRASE_CANARY: &str = "console-engine-passphrase-canary-4be81d";
const BOOT_WAIT: Duration = Duration::from_secs(60);
const TICK_WAIT: Duration = Duration::from_secs(60);
const END_WAIT: Duration = Duration::from_secs(30);
const MAX_TICK_GAP: Duration = Duration::from_secs(45);
const SPAN_COUNT: usize = 5;

/// A loopback port claimed for one arm. It lies below the kernel's ephemeral
/// range, so no bind to port 0 is ever handed it, and an advisory lock under
/// `target/` holds it against every other test of this workspace for as long
/// as the claim lives. A port bound to 0, read and released is taken again by
/// another test often enough to fail a run when many start at once.
struct PortClaim {
    number: u16,
    _lock: std::fs::File,
}

const CLAIMED_PORTS: std::ops::Range<u16> = 20_000..32_000;
/// The harness's own registered ports inside that range.
const HARNESS_PORTS: [u16; 2] = [24_317, 24_318];

fn claim_port() -> PortClaim {
    use std::hash::{BuildHasher, Hasher};
    let claims = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the package sits under the workspace root")
        .join("target")
        .join("test-ports");
    std::fs::create_dir_all(&claims).expect("the port claims directory");
    let span = u64::from(CLAIMED_PORTS.end - CLAIMED_PORTS.start);
    for _ in 0..10_000 {
        let word = std::collections::hash_map::RandomState::new()
            .build_hasher()
            .finish();
        let offset = u16::try_from(word % span).expect("an offset inside the range");
        let number = CLAIMED_PORTS.start + offset;
        if HARNESS_PORTS.contains(&number) {
            continue;
        }
        let Ok(lock) = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(claims.join(number.to_string()))
        else {
            continue;
        };
        if lock.try_lock().is_ok() && TcpListener::bind(("127.0.0.1", number)).is_ok() {
            return PortClaim {
                number,
                _lock: lock,
            };
        }
    }
    panic!("no loopback port could be claimed")
}

fn loopback(port: u16) -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], port))
}

fn cleared(dir: &Path) -> Command {
    let mut command = Command::new(ENGINE);
    command
        .env_clear()
        .env("ANDROMEDA_PULSE_DATA_DIR", dir)
        .current_dir(dir)
        .stdin(Stdio::null());
    if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile);
    }
    command
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
        let mut command = cleared(data_dir.path());
        command
            .arg("run")
            .env("ANDROMEDA_PULSE_OTLP_GRPC_PORT", grpc.to_string())
            .env("ANDROMEDA_PULSE_OTLP_HTTP_PORT", http.to_string())
            .env("ANDROMEDA_PULSE_CORPUS_PASSPHRASE", PASSPHRASE_CANARY)
            .stdout(stdout)
            .stderr(stderr);
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

    fn still_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }

    async fn wait_for_both_ports(&mut self) {
        let deadline = Instant::now() + BOOT_WAIT;
        while !(accepts(self.grpc) && accepts(self.http)) {
            assert!(self.still_running(), "the program ended before it listened");
            assert!(
                Instant::now() < deadline,
                "both ports accept within {BOOT_WAIT:?}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    async fn wait_for_port(&mut self, port: u16) {
        let deadline = Instant::now() + BOOT_WAIT;
        while !accepts(port) {
            assert!(self.still_running(), "the program ended before it listened");
            assert!(
                Instant::now() < deadline,
                "the port accepts within {BOOT_WAIT:?}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    /// Polls the log family until `count` records of `target` are in it.
    async fn wait_for_records(&mut self, target: &str, count: usize, bound: Duration) {
        let deadline = Instant::now() + bound;
        while records(&family_lines(self.data_dir.path()), target).len() < count {
            assert!(
                self.still_running(),
                "the program ended while {target} was awaited"
            );
            assert!(
                Instant::now() < deadline,
                "{count} `{target}` records within {bound:?}"
            );
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    fn signal(&self, signal: libc::c_int) {
        let pid = libc::pid_t::try_from(self.child.id()).expect("pid fits pid_t");
        // SAFETY: signals the one child this test spawned, by its own pid.
        let sent = unsafe { libc::kill(pid, signal) };
        assert_eq!(sent, 0, "the signal was delivered to the child");
    }

    async fn wait_for_end(&mut self) -> ExitStatus {
        let deadline = Instant::now() + END_WAIT;
        loop {
            if let Some(status) = self.child.try_wait().expect("try_wait") {
                return status;
            }
            assert!(
                Instant::now() < deadline,
                "the program ends within {END_WAIT:?}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    fn stream(&self, name: &str) -> String {
        std::fs::read_to_string(self.streams.path().join(name)).expect("read the stream file")
    }
}

fn accepts(port: u16) -> bool {
    TcpStream::connect_timeout(&loopback(port), Duration::from_millis(250)).is_ok()
}

fn family_lines(dir: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir.join("logs")) else {
        return out;
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
    for path in paths {
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

fn records<'a>(lines: &'a [Value], target: &str) -> Vec<&'a Value> {
    lines
        .iter()
        .filter(|v| v.get("target").and_then(Value::as_str) == Some(target))
        .collect()
}

fn target_of(line: &Value) -> &str {
    line.get("target").and_then(Value::as_str).unwrap_or("")
}

fn strings_in(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) => out.push(s.clone()),
        Value::Array(items) => items.iter().for_each(|v| strings_in(v, out)),
        Value::Object(map) => map.iter().for_each(|(key, v)| {
            out.push(key.clone());
            strings_in(v, out);
        }),
        _ => {}
    }
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now after epoch")
        .as_nanos() as u64
}

fn spans_request(service: &str) -> ExportTraceServiceRequest {
    let start = now_ns();
    let spans = (0..SPAN_COUNT)
        .map(|i| Span {
            trace_id: vec![3u8; 16],
            span_id: (i as u64 + 1).to_le_bytes().to_vec(),
            name: "console-engine-span".to_string(),
            kind: 0,
            start_time_unix_nano: start,
            end_time_unix_nano: start + 1_000_000,
            ..Default::default()
        })
        .collect();
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
                spans,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

fn timestamp(line: &Value) -> chrono::DateTime<chrono::Utc> {
    let text = line["timestamp"].as_str().expect("timestamp string");
    chrono::DateTime::parse_from_rfc3339(text)
        .expect("an RFC 3339 timestamp")
        .with_timezone(&chrono::Utc)
}

fn assert_every_line_is_a_whole_record(lines: &[Value]) {
    assert!(!lines.is_empty(), "the program wrote its log");
    for line in lines {
        for key in ["timestamp", "level", "target", "message"] {
            assert!(line[key].is_string(), "`{key}` on {line}");
        }
        let fields = line["fields"].as_object().expect("fields object");
        assert_eq!(fields["service.name"], "com.andromeda.pulse", "{line}");
        assert_eq!(
            fields["service.version"],
            env!("CARGO_PKG_VERSION"),
            "{line}"
        );
        assert_eq!(fields["deployment.environment"], "production", "{line}");
    }
}

fn assert_the_one_exit_record_is_last(lines: &[Value], signal: &str) {
    let exits = records(lines, "app.exit");
    assert_eq!(exits.len(), 1, "exactly one app.exit, got {exits:?}");
    let last = lines.last().expect("records");
    assert_eq!(target_of(last), "app.exit", "app.exit is the last record");
    assert_eq!(last["level"], "WARN");
    let fields = last["fields"].as_object().expect("fields object");
    for key in ["exit_class", "exit_code", "exit_code_known", "signal"] {
        assert_ne!(fields[key], Value::from("<redacted>"), "`{key}` rendered");
    }
    assert_eq!(fields["exit_class"], "signal");
    assert_eq!(fields["exit_code_known"], false);
    assert_eq!(fields["signal"], signal);
}

fn the_one_boot_record(lines: &[Value]) -> &Value {
    let boots = records(lines, "app.boot.engine");
    assert_eq!(boots.len(), 1, "exactly one boot record, got {boots:?}");
    boots[0]
}

fn assert_no_log_record_in(stream: &str, name: &str) {
    for line in stream.lines() {
        assert!(
            !(line.contains("\"target\"") || line.contains("\"level\"")),
            "{name} holds a log record"
        );
    }
}

fn entries(dir: &Path) -> BTreeSet<String> {
    std::fs::read_dir(dir)
        .expect("read the data dir")
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect()
}

/// One socket a process holds: the kernel table that lists it and its local
/// end.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Socket {
    table: &'static str,
    local: String,
    listening: bool,
}

/// Every socket of one process, with the count of its socket inodes that no
/// table lists.
#[cfg(target_os = "linux")]
#[derive(Debug)]
struct Census {
    sockets: Vec<Socket>,
    unlisted: usize,
}

#[cfg(target_os = "linux")]
impl Census {
    fn listeners(&self) -> BTreeSet<(&'static str, String)> {
        self.sockets
            .iter()
            .filter(|socket| socket.listening)
            .map(|socket| (socket.table, socket.local.clone()))
            .collect()
    }

    /// No socket is in the middle of a connection the readiness probe made.
    fn at_rest(&self) -> bool {
        self.unlisted == 0
            && self
                .sockets
                .iter()
                .all(|socket| socket.listening || !socket.table.starts_with("tcp"))
    }
}

/// The socket inodes in a process's fd table.
#[cfg(target_os = "linux")]
fn socket_inodes(pid: u32) -> BTreeSet<u64> {
    std::fs::read_dir(format!("/proc/{pid}/fd"))
        .expect("read the process's fd table under /proc")
        .flatten()
        .filter_map(|entry| std::fs::read_link(entry.path()).ok())
        .filter_map(|target| {
            let target = target.to_string_lossy();
            target
                .strip_prefix("socket:[")?
                .strip_suffix(']')?
                .parse()
                .ok()
        })
        .collect()
}

/// `0100007F:10E1` as `127.0.0.1:4321`; an IPv6 address stays in its hex form.
#[cfg(target_os = "linux")]
fn local_end(column: &str) -> String {
    let (address, port) = column.rsplit_once(':').unwrap_or((column, ""));
    let port = u16::from_str_radix(port, 16).unwrap_or(0);
    match u32::from_str_radix(address, 16) {
        Ok(raw) if address.len() == 8 => {
            format!("{}:{port}", std::net::Ipv4Addr::from(raw.to_ne_bytes()))
        }
        _ => format!("[{address}]:{port}"),
    }
}

/// The sockets of `pid`, matched by inode in the kernel's TCP, TCP6, UDP, UDP6
/// and unix tables.
#[cfg(target_os = "linux")]
fn socket_census(pid: u32) -> Census {
    let inodes = socket_inodes(pid);
    let table =
        |name: &str| std::fs::read_to_string(format!("/proc/net/{name}")).unwrap_or_default();
    let mut listed = BTreeSet::new();
    let mut sockets = Vec::new();
    for name in ["tcp", "tcp6", "udp", "udp6"] {
        for line in table(name).lines().skip(1) {
            let columns: Vec<&str> = line.split_whitespace().collect();
            let Some(inode) = columns.get(9).and_then(|inode| inode.parse::<u64>().ok()) else {
                continue;
            };
            if inodes.contains(&inode) {
                listed.insert(inode);
                sockets.push(Socket {
                    table: name,
                    local: local_end(columns[1]),
                    listening: name.starts_with("tcp") && columns[3] == "0A",
                });
            }
        }
    }
    for line in table("unix").lines().skip(1) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        let Some(inode) = columns.get(6).and_then(|inode| inode.parse::<u64>().ok()) else {
            continue;
        };
        if inodes.contains(&inode) {
            listed.insert(inode);
            sockets.push(Socket {
                table: "unix",
                local: columns.get(7).copied().unwrap_or("(unnamed)").to_string(),
                listening: columns[3] == "00010000",
            });
        }
    }
    sockets.sort();
    Census {
        unlisted: inodes.len() - listed.len(),
        sockets,
    }
}

/// The census once the connections the readiness probe opened are closed, or
/// the last one read when `BOOT_WAIT` has passed.
#[cfg(target_os = "linux")]
async fn census_at_rest(pid: u32) -> Census {
    let deadline = Instant::now() + BOOT_WAIT;
    loop {
        let census = socket_census(pid);
        if census.at_rest() || Instant::now() >= deadline {
            return census;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// An empty census cannot pass: pointed at this process, it finds a listener
/// this process opened.
#[cfg(target_os = "linux")]
fn assert_the_census_finds_a_listener_of_this_process() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind an ephemeral loopback port");
    let port = listener.local_addr().expect("local addr").port();
    let census = socket_census(std::process::id());
    assert!(
        census
            .listeners()
            .contains(&("tcp", format!("127.0.0.1:{port}"))),
        "the census reads this process's own listener on port {port}: {census:?}"
    );
}

#[cfg(target_os = "linux")]
fn assert_no_socket_beyond_the_listeners(census: &Census) {
    for socket in &census.sockets {
        assert!(
            !socket.table.starts_with("udp"),
            "a UDP socket in table {} at {}",
            socket.table,
            socket.local
        );
        assert!(
            socket.listening || socket.table == "unix",
            "a connected socket in table {} at {}",
            socket.table,
            socket.local
        );
    }
    assert_eq!(
        census.unlisted, 0,
        "a socket inode outside the five tables: {census:?}"
    );
}

#[cfg(target_os = "linux")]
fn file_names_under(dir: &Path, out: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).expect("read a directory").flatten() {
        let path = entry.path();
        if path.is_dir() {
            file_names_under(&path, out);
        } else {
            out.push(entry.file_name().to_string_lossy().to_string());
        }
    }
}

/// The log family's bytes as written, for a count of a text in them.
#[cfg(target_os = "linux")]
fn family_text(dir: &Path) -> String {
    let mut names = Vec::new();
    file_names_under(&dir.join("logs"), &mut names);
    names
        .iter()
        .filter(|name| name.starts_with("agent-latest.jsonl"))
        .map(|name| std::fs::read_to_string(dir.join("logs").join(name)).expect("read a member"))
        .collect()
}

#[cfg(target_os = "linux")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_alone_listening()
{
    use std::os::unix::process::ExitStatusExt;

    assert_the_census_finds_a_listener_of_this_process();
    let claimed = claim_port();
    let digits = claimed.number;
    let rejected = format!("{digits}/tcp");
    let mut engine = Running::spawn(&[("ANDROMEDA_PULSE_OTLP_GRPC_PORT", rejected.as_str())]);
    let http = engine.http;
    engine.wait_for_port(http).await;
    engine
        .wait_for_records("app.boot.otlp.grpc.bind", 1, BOOT_WAIT)
        .await;

    let census = census_at_rest(engine.child.id()).await;
    eprintln!("socket census, gRPC port variable rejected: {census:?}");
    assert_eq!(
        census.listeners(),
        BTreeSet::from([("tcp", format!("127.0.0.1:{http}"))]),
        "the HTTP receiver is the one listener: {census:?}"
    );
    assert_no_socket_beyond_the_listeners(&census);
    assert!(
        !accepts(digits) && !accepts(engine.grpc),
        "no gRPC receiver was started"
    );
    assert!(engine.still_running(), "the program runs on");

    engine.signal(libc::SIGTERM);
    let status = engine.wait_for_end().await;
    assert_eq!(status.signal(), Some(15), "ended by SIGTERM");

    let lines = family_lines(engine.data_dir.path());
    let grpc_binds = records(&lines, "app.boot.otlp.grpc.bind");
    assert_eq!(grpc_binds.len(), 1, "one gRPC bind record: {grpc_binds:?}");
    assert_eq!(grpc_binds[0]["level"], "ERROR");
    assert_eq!(
        grpc_binds[0]["fields"]["reason"], "invalid_port",
        "the reason is rendered"
    );
    eprintln!(
        "gRPC bind record: level {}, reason {}",
        grpc_binds[0]["level"], grpc_binds[0]["fields"]["reason"]
    );
    assert!(
        grpc_binds[0]["fields"].get("bind_address").is_none(),
        "a rejected port has no bind address"
    );
    let http_binds = records(&lines, "app.boot.otlp.http.bind");
    assert_eq!(http_binds.len(), 1, "one HTTP bind record: {http_binds:?}");
    assert_eq!(http_binds[0]["level"], "INFO");
    assert_eq!(
        http_binds[0]["fields"]["bind_address"],
        format!("127.0.0.1:{http}")
    );
    assert_the_one_exit_record_is_last(&lines, "sigterm");
}

#[cfg(target_os = "linux")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_engine_listens_on_its_two_loopback_receivers_and_on_nothing_else() {
    use std::os::unix::process::ExitStatusExt;

    assert_the_census_finds_a_listener_of_this_process();
    let mut engine = Running::spawn(&[]);
    engine.wait_for_both_ports().await;

    let census = census_at_rest(engine.child.id()).await;
    eprintln!("socket census, both ports valid: {census:?}");
    assert_eq!(
        census.listeners(),
        BTreeSet::from([
            ("tcp", format!("127.0.0.1:{}", engine.grpc)),
            ("tcp", format!("127.0.0.1:{}", engine.http)),
        ]),
        "the two receivers are the only listeners: {census:?}"
    );
    assert_no_socket_beyond_the_listeners(&census);

    engine.signal(libc::SIGTERM);
    let status = engine.wait_for_end().await;
    assert_eq!(status.signal(), Some(15), "ended by SIGTERM");
}

#[cfg(target_os = "linux")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_corpus_key_lock_file_sits_in_the_directory_the_environment_gives() {
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::process::ExitStatusExt;
    const LOCK_PREFIX: &str = "andromeda-pulse-corpus-key-";

    for with_runtime_dir in [false, true] {
        let temp = tempfile::tempdir().expect("a temp dir of the run's own");
        let runtime = tempfile::tempdir().expect("a runtime dir of the run's own");
        let temp_text = temp.path().to_string_lossy().to_string();
        let runtime_text = runtime.path().to_string_lossy().to_string();
        let mut env = vec![("TMPDIR", temp_text.as_str())];
        if with_runtime_dir {
            env.push(("XDG_RUNTIME_DIR", runtime_text.as_str()));
        }
        let mut engine = Running::spawn(&env);
        engine.wait_for_both_ports().await;
        engine
            .wait_for_records("corpus.keychain.fallback", 1, BOOT_WAIT)
            .await;
        engine.signal(libc::SIGTERM);
        let status = engine.wait_for_end().await;
        assert_eq!(status.signal(), Some(15), "ended by SIGTERM");

        let holder = if with_runtime_dir {
            runtime.path()
        } else {
            temp.path()
        };
        let held = entries(holder);
        assert_eq!(held.len(), 1, "the lock file is the only entry: {held:?}");
        let name = held.first().expect("one entry");
        let hex = name
            .strip_prefix(LOCK_PREFIX)
            .and_then(|rest| rest.strip_suffix(".lock"))
            .unwrap_or_else(|| panic!("a lock file name: {name}"));
        assert!(
            hex.len() == 16
                && hex
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            "sixteen hex characters: {name}"
        );
        let meta = std::fs::metadata(holder.join(name)).expect("the lock file's metadata");
        assert_eq!(meta.len(), 0, "the lock file holds nothing");
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
        eprintln!(
            "lock file, {}: {name}, {} bytes, mode {:o}",
            if with_runtime_dir {
                "runtime dir given"
            } else {
                "temp dir given, no runtime dir"
            },
            meta.len(),
            meta.permissions().mode() & 0o777
        );
        if with_runtime_dir {
            assert!(
                entries(temp.path()).is_empty(),
                "nothing in the temp dir when a runtime dir is given"
            );
        }

        let data_dir = engine.data_dir.path();
        assert_eq!(
            entries(data_dir),
            BTreeSet::from(["corpus", "logs", "run"].map(String::from)),
            "the data dir's top entries"
        );
        let mut under_data_dir = Vec::new();
        file_names_under(data_dir, &mut under_data_dir);
        assert!(
            under_data_dir
                .iter()
                .all(|file| !file.starts_with(LOCK_PREFIX)),
            "no lock file under the data dir"
        );

        let log = family_text(data_dir);
        assert!(!log.is_empty(), "the program wrote its log");
        let resolved = holder.canonicalize().expect("the lock dir resolves");
        for text in [
            name.as_str(),
            &holder.to_string_lossy(),
            &resolved.to_string_lossy(),
        ] {
            assert_eq!(
                log.matches(text).count(),
                0,
                "the lock file's place is not in the log"
            );
        }
    }
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn run_boots_with_no_display_lands_telemetry_and_ends_by_sigterm_with_one_exit_record() {
    use std::os::unix::process::ExitStatusExt;
    const SERVICE: &str = "console-engine-svc";

    let mut engine = Running::spawn(&[]);
    engine.wait_for_both_ports().await;

    let pid_file = engine
        .data_dir
        .path()
        .join("run")
        .join("andromeda-pulse.pid");
    assert_eq!(
        std::fs::read_to_string(pid_file).expect("the pid file is written"),
        engine.child.id().to_string(),
        "the pid file holds the program's own pid"
    );

    let mut client = TraceServiceClient::connect(format!("http://{}", loopback(engine.grpc)))
        .await
        .expect("the gRPC receiver accepts a client");
    client
        .export(tonic::Request::new(spans_request(SERVICE)))
        .await
        .expect("the export is accepted");
    drop(client);

    engine.wait_for_records("duckdb.append", 1, BOOT_WAIT).await;
    // The first tick of each target is written at boot and the next one a tick
    // interval later, so the second pair shows a run held past one interval.
    engine.wait_for_records("ingest.tick", 2, TICK_WAIT).await;
    engine.wait_for_records("buffer.tick", 2, TICK_WAIT).await;

    engine.signal(libc::SIGTERM);
    let status = engine.wait_for_end().await;
    assert_eq!(
        status.signal(),
        Some(15),
        "ended by SIGTERM, not an exit code"
    );

    let lines = family_lines(engine.data_dir.path());
    assert_every_line_is_a_whole_record(&lines);
    assert_the_one_exit_record_is_last(&lines, "sigterm");

    let boot = the_one_boot_record(&lines);
    assert_eq!(boot["level"], "WARN");
    assert_eq!(boot["fields"]["program"], "console");
    assert_eq!(boot["fields"]["interpretation"], "none");
    assert_eq!(boot["fields"]["reason"], "deterministic_gate_unset");

    let fallback = records(&lines, "corpus.keychain.fallback");
    assert_eq!(fallback.len(), 1, "one credential-store fallback warning");
    assert_eq!(fallback[0]["level"], "WARN");

    let appended: u64 = records(&lines, "duckdb.append")
        .iter()
        .filter(|r| r["fields"]["table_name"] == "spans")
        .filter_map(|r| r["fields"]["rows_appended"].as_u64())
        .sum();
    assert_eq!(
        appended, SPAN_COUNT as u64,
        "the spans sent are the spans stored"
    );

    for line in &lines {
        let target = target_of(line);
        assert_ne!(line["level"], "ERROR", "an ERROR record: {line}");
        for absent in [
            "app.panic.fatal",
            "interpretation.incident.created",
            "viz.tick",
            "plugins.tick",
            "app.boot.webview.init",
            "app.boot.gpu.check",
            "app.boot.tray.init",
            "app.boot.render.posture",
        ] {
            assert_ne!(target, absent, "the console program wrote {line}");
        }
        for prefix in ["interpretation.model.", "app.boot.window."] {
            assert!(
                !target.starts_with(prefix),
                "the console program wrote {line}"
            );
        }
    }

    for target in ["ingest.tick", "buffer.tick"] {
        let ticks = records(&lines, target);
        assert!(ticks.len() >= 2, "two `{target}` records");
        for pair in ticks.windows(2) {
            let gap = timestamp(pair[1]) - timestamp(pair[0]);
            assert!(
                gap.to_std().expect("ticks in order") <= MAX_TICK_GAP,
                "`{target}` gap {gap} is within {MAX_TICK_GAP:?}"
            );
        }
    }
    for tick in records(&lines, "buffer.tick") {
        assert!(
            tick["fields"]["rows_ingested_delta"].is_u64(),
            "rows_ingested_delta rendered: {tick}"
        );
    }
    assert!(
        records(&lines, "buffer.tick")
            .iter()
            .any(|tick| tick["fields"]["rows_ingested"].as_u64() >= Some(SPAN_COUNT as u64)),
        "a buffer tick counts the stored spans"
    );

    let full = engine.data_dir.path().to_string_lossy().to_string();
    let mut strings = Vec::new();
    lines.iter().for_each(|line| strings_in(line, &mut strings));
    for s in &strings {
        assert!(
            !s.contains(PASSPHRASE_CANARY),
            "the passphrase reached the log"
        );
        assert!(
            !s.contains(&full),
            "the data dir's full path reached the log"
        );
    }

    assert_no_log_record_in(&engine.stream("stderr"), "stderr");
    assert_no_log_record_in(&engine.stream("stdout"), "stdout");

    assert!(
        !accepts(engine.grpc) && !accepts(engine.http),
        "both ports refuse after the program ended"
    );

    let registered: BTreeSet<String> = ["corpus", "logs", "run"].map(String::from).into();
    let written = entries(engine.data_dir.path());
    assert!(
        written.is_subset(&registered),
        "an entry outside the registered subpaths: {written:?}"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn run_with_the_deterministic_variable_set_seats_the_canned_runner_and_ends_by_sigint() {
    use std::os::unix::process::ExitStatusExt;

    let mut engine = Running::spawn(&[("ANDROMEDA_PULSE_L4_DETERMINISTIC", "1")]);
    engine.wait_for_both_ports().await;
    engine
        .wait_for_records("app.boot.engine", 1, BOOT_WAIT)
        .await;

    engine.signal(libc::SIGINT);
    let status = engine.wait_for_end().await;
    assert_eq!(
        status.signal(),
        Some(2),
        "ended by SIGINT, not an exit code"
    );

    let lines = family_lines(engine.data_dir.path());
    assert_every_line_is_a_whole_record(&lines);
    assert_the_one_exit_record_is_last(&lines, "sigint");

    let boot = the_one_boot_record(&lines);
    assert_eq!(boot["level"], "INFO");
    assert_eq!(boot["fields"]["program"], "console");
    assert_eq!(boot["fields"]["interpretation"], "deterministic");
    assert_eq!(boot["fields"]["reason"], "deterministic_gate_set");
    assert!(
        lines
            .iter()
            .all(|line| !target_of(line).starts_with("interpretation.model.")),
        "no model record with the canned runner seated"
    );
}

#[test]
fn version_prints_one_line_exits_zero_and_touches_no_data_dir() {
    let dir = tempfile::tempdir().expect("data dir");
    let output = cleared(dir.path())
        .arg("version")
        .output()
        .expect("run the console program");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("andromeda-pulse-engine {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert!(output.stderr.is_empty(), "nothing on stderr");
    assert!(entries(dir.path()).is_empty(), "the data dir stays empty");
}

#[test]
fn an_unknown_word_and_no_word_exit_two_with_usage_and_create_nothing() {
    for words in [&["stop"][..], &[][..], &["run", "extra"][..]] {
        let dir = tempfile::tempdir().expect("data dir");
        let output = cleared(dir.path())
            .args(words)
            .output()
            .expect("run the console program");
        assert_eq!(output.status.code(), Some(2), "{words:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "usage: andromeda-pulse-engine <run|version>\n",
            "{words:?}"
        );
        assert!(output.stdout.is_empty(), "{words:?}: nothing on stdout");
        assert!(
            entries(dir.path()).is_empty(),
            "{words:?}: the data dir stays empty"
        );
    }
}
