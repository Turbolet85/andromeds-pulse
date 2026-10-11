//! The door: the console engine's one way in besides telemetry.
//!
//! A Unix-domain stream socket at `run/door/sock` under the data dir. A caller
//! of the engine's own account sends JSON-RPC 2.0 requests, one per line, and
//! is answered by the tools crate reading the stores the running engine holds.
//! The door holds no store of its own, and `engine_boot::start` is the only
//! place that seats it.
//!
//! Its three records (`app.boot.door`, `door.call`, `door.refused`) carry
//! closed labels and counts only: no argument, no answer, no path, no account
//! id and no error text.

use std::ffi::OsStr;
use std::sync::{Arc, Mutex};

use corpus::contract::CorpusWriter;
use duckdb::{Connection, InterruptHandle};
use mcp_server_crate::contract::Error as ToolError;
use mcp_server_crate::tools::{self, IncidentToolContext, ToolAnswer};
use serde_json::Value;
use viz::state::VizState;

#[cfg(unix)]
pub use socket::{Door, bind_endpoint, open, own_account};

#[doc(hidden)]
pub const TARGET_BOOT_DOOR: &str = "app.boot.door";
#[doc(hidden)]
pub const TARGET_DOOR_CALL: &str = "door.call";
#[doc(hidden)]
pub const TARGET_DOOR_REFUSED: &str = "door.refused";

pub const MAX_REQUEST_BYTES: usize = 65_536;
pub const MAX_ANSWER_BYTES: usize = 1_048_576;
pub const CALL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
pub const IDLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
pub const MAX_CONNECTIONS: usize = 8;

pub const CODE_WRITE_OFF: i64 = -32001;
pub const CODE_TIMED_OUT: i64 = -32002;
pub const CODE_ANSWER_TOO_LARGE: i64 = -32003;

/// The write switch for the one tool that changes what the engine stores.
/// Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveWrite {
    On,
    Off,
    Rejected,
}

impl ResolveWrite {
    /// Unset or `on` is on and `off` is off. Any other value is off as well
    /// and reads `rejected`, so a mistyped value cannot leave the write on.
    pub fn from_value(raw: Option<&OsStr>) -> Self {
        match raw.map(OsStr::to_str) {
            None | Some(Some("on")) => Self::On,
            Some(Some("off")) => Self::Off,
            Some(_) => Self::Rejected,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Rejected => "rejected",
        }
    }

    pub fn allows_write(self) -> bool {
        self == Self::On
    }
}

/// What became of the endpoint at boot. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointState {
    Listening,
    PathTooLong,
    InUse,
    Occupied,
    BindFailed,
}

impl EndpointState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Listening => "listening",
            Self::PathTooLong => "path_too_long",
            Self::InUse => "in_use",
            Self::Occupied => "occupied",
            Self::BindFailed => "bind_failed",
        }
    }
}

/// How one answered request ended. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Ok,
    ToolError,
    InvalidArguments,
    UnknownTool,
    UnknownMethod,
    ParseError,
    WriteOff,
    TimedOut,
    AnswerTooLarge,
}

impl Outcome {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::ToolError => "tool_error",
            Self::InvalidArguments => "invalid_arguments",
            Self::UnknownTool => "unknown_tool",
            Self::UnknownMethod => "unknown_method",
            Self::ParseError => "parse_error",
            Self::WriteOff => "write_off",
            Self::TimedOut => "timed_out",
            Self::AnswerTooLarge => "answer_too_large",
        }
    }
}

/// Why a connection or a line was refused before an answer. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    OtherAccount,
    TooManyConnections,
    RequestTooLarge,
    Idle,
}

impl Refusal {
    pub fn label(self) -> &'static str {
        match self {
            Self::OtherAccount => "other_account",
            Self::TooManyConnections => "too_many_connections",
            Self::RequestTooLarge => "request_too_large",
            Self::Idle => "idle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Peer {
    Owner,
    Other,
}

impl Peer {
    pub fn label(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Other => "other",
        }
    }
}

/// The owner check: the peer's account, as the kernel reports it, against the
/// engine's own effective account. Nothing else identifies a caller.
pub fn admits(peer_account: u32, own_account: u32) -> bool {
    peer_account == own_account
}

/// The fields of one `door.call` record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallRecord {
    pub method: &'static str,
    pub tool: &'static str,
    pub outcome: Outcome,
    pub write: bool,
    pub result_type: &'static str,
    pub result_count: u64,
    pub answer_bytes: u64,
    pub duration_ms: u64,
}

/// INFO when the door listens with the switch not rejected, WARN otherwise.
#[doc(hidden)]
pub fn emit_boot(state: EndpointState, resolve: ResolveWrite) {
    let (state_label, resolve_write) = (state.label(), resolve.label());
    if state == EndpointState::Listening && resolve != ResolveWrite::Rejected {
        tracing::info!(
            target: TARGET_BOOT_DOOR,
            state = state_label,
            resolve_write,
            "door boot",
        );
    } else {
        tracing::warn!(
            target: TARGET_BOOT_DOOR,
            state = state_label,
            resolve_write,
            "door boot: the door is not listening, or its write switch was rejected and the write is off",
        );
    }
}

/// INFO for `ok`, WARN otherwise.
#[doc(hidden)]
pub fn emit_call(record: CallRecord) {
    let outcome = record.outcome.label();
    if record.outcome == Outcome::Ok {
        tracing::info!(
            target: TARGET_DOOR_CALL,
            method = record.method,
            tool = record.tool,
            outcome,
            write = record.write,
            result_type = record.result_type,
            result_count = record.result_count,
            answer_bytes = record.answer_bytes,
            duration_ms = record.duration_ms,
            "door call",
        );
    } else {
        tracing::warn!(
            target: TARGET_DOOR_CALL,
            method = record.method,
            tool = record.tool,
            outcome,
            write = record.write,
            result_type = record.result_type,
            result_count = record.result_count,
            answer_bytes = record.answer_bytes,
            duration_ms = record.duration_ms,
            "door call not answered with a result",
        );
    }
}

#[doc(hidden)]
pub fn emit_refused(reason: Refusal, peer: Peer) {
    tracing::warn!(
        target: TARGET_DOOR_REFUSED,
        reason = reason.label(),
        peer = peer.label(),
        "door refused a connection or a request",
    );
}

/// What the door calls for `tools/call`. The engine's own is [`EngineTools`].
pub trait Tools: Send + Sync + 'static {
    fn call(&self, tool: &'static str, arguments: &Value) -> Result<ToolAnswer, ToolError>;

    /// Ends a call that passed its time bound, where its store can be
    /// interrupted.
    fn interrupt(&self);
}

/// The tools over the stores the running engine holds: a read connection of
/// the door's own taken from the engine's buffer connection, the engine's
/// corpus writer and the workspace key the engine stamps its incidents with.
pub struct EngineTools {
    buffer: Option<Arc<Mutex<Connection>>>,
    // Present only for a connection of the door's own: where the read
    // connection fell back to the engine's shared one, an interrupt would end
    // the engine's own statement.
    buffer_interrupt: Option<Arc<InterruptHandle>>,
    viz_state: VizState,
    incidents: Option<IncidentToolContext>,
}

impl EngineTools {
    pub fn new(
        buffer_conn: Option<&Arc<Mutex<Connection>>>,
        corpus: Option<&Arc<dyn CorpusWriter>>,
        workspace_key: &str,
    ) -> Self {
        let buffer = buffer_conn.map(viz::query::read_connection);
        let buffer_interrupt = match (buffer.as_ref(), buffer_conn) {
            (Some(read), Some(shared)) if !Arc::ptr_eq(read, shared) => {
                read.lock().ok().map(|guard| guard.interrupt_handle())
            }
            _ => None,
        };
        Self {
            buffer,
            buffer_interrupt,
            viz_state: VizState::new(),
            incidents: corpus.map(|corpus| IncidentToolContext {
                corpus: Arc::clone(corpus),
                workspace_root: workspace_key.to_string(),
            }),
        }
    }
}

impl Tools for EngineTools {
    fn call(&self, tool: &'static str, arguments: &Value) -> Result<ToolAnswer, ToolError> {
        // With no buffer connection the engine runs degraded and no tool is
        // answered: the tools crate takes the connection for every tool.
        let Some(buffer) = self.buffer.as_ref() else {
            return Err(ToolError::ToolDispatchFailed {
                tool_name: tool.to_string(),
                reason: "buffer unavailable".to_string(),
            });
        };
        tools::call_tool(
            buffer,
            &self.viz_state,
            self.incidents.as_ref(),
            tool,
            arguments,
        )
    }

    fn interrupt(&self) {
        if let Some(handle) = self.buffer_interrupt.as_ref() {
            handle.interrupt();
        }
    }
}

#[cfg(unix)]
mod socket {
    use std::fs;
    use std::io::ErrorKind;
    use std::os::unix::fs::{DirBuilderExt, FileTypeExt, PermissionsExt};
    use std::path::Path;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use mcp_server_crate::contract::Error as ToolError;
    use mcp_server_crate::jsonrpc::{
        self, CODE_INTERNAL_ERROR, CODE_INVALID_PARAMS, CODE_INVALID_REQUEST,
        CODE_METHOD_NOT_FOUND, CODE_PARSE_ERROR,
    };
    use mcp_server_crate::tools::{ALL_TOOL_NAMES, TOOL_MARK_INCIDENT_RESOLVED};
    use serde_json::{Value, json};
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
    use tokio::net::{UnixListener, UnixStream};
    use tokio::sync::Semaphore;

    use super::{
        CALL_TIMEOUT, CODE_ANSWER_TOO_LARGE, CODE_TIMED_OUT, CODE_WRITE_OFF, CallRecord,
        EndpointState, IDLE_TIMEOUT, MAX_ANSWER_BYTES, MAX_CONNECTIONS, MAX_REQUEST_BYTES, Outcome,
        Peer, Refusal, ResolveWrite, Tools, admits, emit_boot, emit_call, emit_refused,
    };

    const RUN_DIR: &str = "run";
    const DOOR_DIR: &str = "door";
    const SOCKET_NAME: &str = "sock";
    // A socket address holds 108 bytes on Linux, its terminator among them.
    const SOCKET_ADDRESS_BYTES: usize = 108;
    const ACCEPT_RETRY: Duration = Duration::from_millis(100);

    const METHOD_INITIALIZE: &str = "initialize";
    const METHOD_TOOLS_LIST: &str = "tools/list";
    const METHOD_TOOLS_CALL: &str = "tools/call";
    const METHOD_PING: &str = "ping";
    const NONE: &str = "none";

    /// The engine's own effective account.
    pub fn own_account() -> u32 {
        // SAFETY: `geteuid` takes no argument, touches no memory and cannot
        // fail.
        unsafe { libc::geteuid() }
    }

    /// Creates `run/door/` mode 0700 before anything is bound, then binds
    /// `run/door/sock` and sets it 0600. An `Err` leaves the door unstarted.
    #[doc(hidden)]
    pub fn bind_endpoint(data_dir: &Path) -> Result<UnixListener, EndpointState> {
        let failed = |_| EndpointState::BindFailed;
        let run_dir = data_dir.join(RUN_DIR);
        let door_dir = run_dir.join(DOOR_DIR);
        fs::create_dir_all(&run_dir).map_err(failed)?;
        match fs::DirBuilder::new().mode(0o700).create(&door_dir) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
            Err(_) => return Err(EndpointState::BindFailed),
        }
        let canonical_data = fs::canonicalize(data_dir).map_err(failed)?;
        let canonical_door = fs::canonicalize(&door_dir).map_err(failed)?;
        if !canonical_door.starts_with(&canonical_data) {
            return Err(EndpointState::BindFailed);
        }
        fs::set_permissions(&canonical_door, fs::Permissions::from_mode(0o700)).map_err(failed)?;

        let socket = canonical_door.join(SOCKET_NAME);
        if socket.as_os_str().len() >= SOCKET_ADDRESS_BYTES {
            return Err(EndpointState::PathTooLong);
        }
        // A socket an earlier run left is replaced only when nothing accepts
        // on it.
        match fs::symlink_metadata(&socket) {
            Ok(meta) if !meta.file_type().is_socket() => return Err(EndpointState::Occupied),
            Ok(_) => match std::os::unix::net::UnixStream::connect(&socket) {
                Ok(_) => return Err(EndpointState::InUse),
                Err(e) if e.kind() == ErrorKind::ConnectionRefused => {
                    fs::remove_file(&socket).map_err(failed)?;
                }
                Err(_) => return Err(EndpointState::BindFailed),
            },
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(_) => return Err(EndpointState::BindFailed),
        }
        let listener = UnixListener::bind(&socket).map_err(failed)?;
        fs::set_permissions(&socket, fs::Permissions::from_mode(0o600)).map_err(failed)?;
        Ok(listener)
    }

    /// Binds the endpoint and writes the door's one boot record. `None` leaves
    /// the door unstarted and the engine goes on. Called with the tokio
    /// runtime entered.
    pub fn open(data_dir: &Path, tools: Arc<dyn Tools>, resolve: ResolveWrite) -> Option<Door> {
        match bind_endpoint(data_dir) {
            Ok(listener) => {
                emit_boot(EndpointState::Listening, resolve);
                Some(Door {
                    listener,
                    own_account: own_account(),
                    connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
                    calls: Arc::new(Calls {
                        tools,
                        resolve,
                        turn: tokio::sync::Mutex::new(()),
                    }),
                })
            }
            Err(state) => {
                emit_boot(state, resolve);
                None
            }
        }
    }

    pub struct Door {
        listener: UnixListener,
        own_account: u32,
        connections: Arc<Semaphore>,
        calls: Arc<Calls>,
    }

    impl Door {
        /// Accepts until the process ends.
        pub async fn serve(self) {
            loop {
                let stream = match self.listener.accept().await {
                    Ok((stream, _)) => stream,
                    Err(_) => {
                        tokio::time::sleep(ACCEPT_RETRY).await;
                        continue;
                    }
                };
                let peer = stream.peer_cred().ok().map(|cred| cred.uid());
                if !peer.is_some_and(|account| admits(account, self.own_account)) {
                    // Dropped here: closed with no byte written.
                    emit_refused(Refusal::OtherAccount, Peer::Other);
                    continue;
                }
                let Ok(permit) = Arc::clone(&self.connections).try_acquire_owned() else {
                    emit_refused(Refusal::TooManyConnections, Peer::Owner);
                    continue;
                };
                let calls = Arc::clone(&self.calls);
                tokio::spawn(async move {
                    serve_connection(stream, &calls).await;
                    drop(permit);
                });
            }
        }
    }

    enum Line {
        Whole,
        TooLong,
        End,
    }

    async fn read_line(
        reader: &mut BufReader<OwnedReadHalf>,
        line: &mut Vec<u8>,
    ) -> std::io::Result<Line> {
        let mut bounded = reader.take(MAX_REQUEST_BYTES as u64 + 1);
        if bounded.read_until(b'\n', line).await? == 0 {
            return Ok(Line::End);
        }
        if line.last() == Some(&b'\n') {
            line.pop();
            return Ok(Line::Whole);
        }
        if line.len() > MAX_REQUEST_BYTES {
            return Ok(Line::TooLong);
        }
        // The stream ended on a line with no terminator.
        Ok(Line::Whole)
    }

    async fn write_line(writer: &mut OwnedWriteHalf, bytes: &[u8]) -> std::io::Result<()> {
        writer.write_all(bytes).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await
    }

    async fn serve_connection(stream: UnixStream, calls: &Calls) {
        let (read_half, mut writer) = stream.into_split();
        let mut reader = BufReader::new(read_half);
        let mut line = Vec::new();
        loop {
            line.clear();
            match tokio::time::timeout(IDLE_TIMEOUT, read_line(&mut reader, &mut line)).await {
                Err(_) => {
                    emit_refused(Refusal::Idle, Peer::Owner);
                    return;
                }
                Ok(Err(_)) | Ok(Ok(Line::End)) => return,
                Ok(Ok(Line::TooLong)) => {
                    emit_refused(Refusal::RequestTooLarge, Peer::Owner);
                    let answer = error_line(Value::Null, CODE_INVALID_REQUEST, "request too large");
                    let _ =
                        tokio::time::timeout(IDLE_TIMEOUT, write_line(&mut writer, &answer)).await;
                    return;
                }
                Ok(Ok(Line::Whole)) => {}
            }
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let started = Instant::now();
            let Some(answered) = calls.answer(&line).await else {
                continue;
            };
            emit_call(CallRecord {
                method: answered.method,
                tool: answered.tool,
                outcome: answered.outcome,
                write: answered.write,
                result_type: answered.result_type,
                result_count: answered.result_count,
                answer_bytes: answered.line.len() as u64,
                duration_ms: started.elapsed().as_millis() as u64,
            });
            match tokio::time::timeout(IDLE_TIMEOUT, write_line(&mut writer, &answered.line)).await
            {
                Ok(Ok(())) => {}
                Ok(Err(_)) => return,
                Err(_) => {
                    emit_refused(Refusal::Idle, Peer::Owner);
                    return;
                }
            }
        }
    }

    /// One answer line and what its call record says of it.
    struct Answered {
        line: Vec<u8>,
        method: &'static str,
        tool: &'static str,
        outcome: Outcome,
        write: bool,
        result_type: &'static str,
        result_count: u64,
    }

    fn error_line(id: Value, code: i64, message: &str) -> Vec<u8> {
        serde_json::to_vec(&jsonrpc::error(id, code, message)).unwrap_or_default()
    }

    /// The code and the fixed message of an outcome that is not `ok`.
    fn refusal_of(outcome: Outcome) -> (i64, String) {
        let (code, message) = match outcome {
            Outcome::Ok | Outcome::ToolError => (CODE_INTERNAL_ERROR, "tool failed"),
            Outcome::InvalidArguments => (CODE_INVALID_PARAMS, "invalid arguments"),
            Outcome::UnknownTool => (CODE_METHOD_NOT_FOUND, "unknown tool"),
            Outcome::UnknownMethod => (CODE_METHOD_NOT_FOUND, "method not found"),
            Outcome::ParseError => (CODE_PARSE_ERROR, "parse error"),
            Outcome::WriteOff => (CODE_WRITE_OFF, "the write is switched off"),
            Outcome::TimedOut => (CODE_TIMED_OUT, "timed out"),
            Outcome::AnswerTooLarge => {
                return (
                    CODE_ANSWER_TOO_LARGE,
                    format!("answer too large: the bound is {MAX_ANSWER_BYTES} bytes"),
                );
            }
        };
        (code, message.to_string())
    }

    fn refused(
        id: Value,
        method: &'static str,
        tool: &'static str,
        write: bool,
        outcome: Outcome,
    ) -> Answered {
        let (code, message) = refusal_of(outcome);
        Answered {
            line: error_line(id, code, &message),
            method,
            tool,
            outcome,
            write,
            result_type: NONE,
            result_count: 0,
        }
    }

    /// An answer over its bound is not cut: the caller gets the error instead.
    fn answered(
        id: Value,
        method: &'static str,
        tool: &'static str,
        write: bool,
        result: Value,
        result_type: &'static str,
        result_count: u64,
    ) -> Answered {
        let Ok(line) = serde_json::to_vec(&jsonrpc::success(id.clone(), result)) else {
            return refused(id, method, tool, write, Outcome::ToolError);
        };
        if line.len() > MAX_ANSWER_BYTES {
            return refused(id, method, tool, write, Outcome::AnswerTooLarge);
        }
        Answered {
            line,
            method,
            tool,
            outcome: Outcome::Ok,
            write,
            result_type,
            result_count,
        }
    }

    /// Whether `arguments` is an object holding only members the tool's entry
    /// in the tool list declares. The tools' own argument structs read past a
    /// member they do not know, so the door refuses one itself.
    fn declared_only(tool: &str, arguments: &Value) -> bool {
        let Some(members) = arguments.as_object() else {
            return false;
        };
        let manifest = jsonrpc::tools_list_manifest();
        manifest["tools"]
            .as_array()
            .and_then(|entries| entries.iter().find(|entry| entry["name"] == tool))
            .and_then(|entry| entry["inputSchema"]["properties"].as_object())
            .is_some_and(|declared| members.keys().all(|member| declared.contains_key(member)))
    }

    struct Calls {
        tools: Arc<dyn Tools>,
        resolve: ResolveWrite,
        // One tool call at a time, so an interrupt reaches only the call that
        // passed its bound.
        turn: tokio::sync::Mutex<()>,
    }

    impl Calls {
        /// `None` for a notification: a request with no `id` is not answered.
        async fn answer(&self, line: &[u8]) -> Option<Answered> {
            let parsed = std::str::from_utf8(line)
                .ok()
                .and_then(|text| jsonrpc::parse_request(text).ok());
            let Some(request) = parsed else {
                return Some(refused(Value::Null, NONE, NONE, false, Outcome::ParseError));
            };
            let id = request.id?;
            Some(match request.method.as_str() {
                METHOD_INITIALIZE => answered(
                    id,
                    METHOD_INITIALIZE,
                    NONE,
                    false,
                    jsonrpc::initialize_result(),
                    "server_info",
                    1,
                ),
                METHOD_TOOLS_LIST => answered(
                    id,
                    METHOD_TOOLS_LIST,
                    NONE,
                    false,
                    jsonrpc::tools_list_manifest(),
                    "tools_array",
                    ALL_TOOL_NAMES.len() as u64,
                ),
                METHOD_PING => answered(id, METHOD_PING, NONE, false, json!({}), "pong", 0),
                METHOD_TOOLS_CALL => self.call(id, request.params).await,
                _ => refused(id, "other", NONE, false, Outcome::UnknownMethod),
            })
        }

        async fn call(&self, id: Value, params: Option<Value>) -> Answered {
            let refuse =
                |id, tool, write, outcome| refused(id, METHOD_TOOLS_CALL, tool, write, outcome);
            let name = params
                .as_ref()
                .and_then(|params| params.get("name"))
                .and_then(Value::as_str);
            let Some(name) = name else {
                return refuse(id, NONE, false, Outcome::InvalidArguments);
            };
            let Some(tool) = ALL_TOOL_NAMES.iter().copied().find(|known| *known == name) else {
                return refuse(id, NONE, false, Outcome::UnknownTool);
            };
            let write = tool == TOOL_MARK_INCIDENT_RESOLVED;
            if write && !self.resolve.allows_write() {
                return refuse(id, tool, write, Outcome::WriteOff);
            }
            let arguments = params
                .and_then(|mut params| params.get_mut("arguments").map(Value::take))
                .unwrap_or_else(|| json!({}));
            if !declared_only(tool, &arguments) {
                return refuse(id, tool, write, Outcome::InvalidArguments);
            }

            let deadline = tokio::time::Instant::now() + CALL_TIMEOUT;
            let Ok(turn) = tokio::time::timeout_at(deadline, self.turn.lock()).await else {
                return refuse(id, tool, write, Outcome::TimedOut);
            };
            let tools = Arc::clone(&self.tools);
            let ran = tokio::time::timeout_at(
                deadline,
                tokio::task::spawn_blocking(move || tools.call(tool, &arguments)),
            )
            .await;
            if ran.is_err() {
                // The blocking task cannot be cancelled: its statement is
                // ended inside the store, where the store allows it.
                self.tools.interrupt();
            }
            drop(turn);
            match ran {
                Err(_) => refuse(id, tool, write, Outcome::TimedOut),
                Ok(Ok(Ok(answer))) => answered(
                    id,
                    METHOD_TOOLS_CALL,
                    tool,
                    write,
                    answer.value,
                    answer.result_type,
                    answer.result_count,
                ),
                Ok(Ok(Err(ToolError::ToolArgsInvalid { .. }))) => {
                    refuse(id, tool, write, Outcome::InvalidArguments)
                }
                Ok(Ok(Err(_))) | Ok(Err(_)) => refuse(id, tool, write, Outcome::ToolError),
            }
        }
    }
}
