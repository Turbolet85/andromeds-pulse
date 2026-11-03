//! The stdio relay of `andromeda-pulse-engine door` (`pulse_app::door_bridge`),
//! in process over a stand-in door: a `UnixListener` bound at `run/door/sock`
//! under a data dir in the temp root, which reads each connection until the
//! relay closes its write side and answers with bytes the arm chose. Where an
//! arm reads the word's exit code and its standard error, it spawns the built
//! program with `door` under a cleared environment; no engine runs here.
#![cfg(unix)]

use std::io::{self, Cursor, ErrorKind, Read, Write};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use pulse_app::door::MAX_REQUEST_BYTES;
use pulse_app::door_bridge::{EXIT_UNREACHABLE, UNREACHABLE, Unreachable, relay, socket_path};

const ENGINE: &str = env!("CARGO_BIN_EXE_andromeda-pulse-engine");
const STAND_IN_WAIT: Duration = Duration::from_secs(10);
const SOCKET_TAIL: &str = "/run/door/sock";
const PING: &str = r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#;

/// A data dir holding `run/door/`, directly under the temp root so the socket
/// path stays short.
fn data_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("data dir");
    std::fs::create_dir_all(dir.path().join("run").join("door")).expect("run/door");
    dir
}

fn door_socket(data: &Path) -> PathBuf {
    data.join("run").join("door").join("sock")
}

/// A listener at `socket` and the thread serving it: one connection per
/// answer, in order, each read until the relay closed its write side, then
/// answered with those bytes and closed. The thread returns what each
/// connection carried.
struct StandIn {
    listener: UnixListener,
    serving: JoinHandle<Vec<Vec<u8>>>,
}

impl StandIn {
    fn bind(socket: &Path, answers: Vec<Vec<u8>>) -> Self {
        let listener = UnixListener::bind(socket).expect("bind the stand-in door");
        listener.set_nonblocking(true).expect("nonblocking accept");
        let serving_listener = listener.try_clone().expect("clone the listener");
        let serving = thread::spawn(move || {
            let deadline = Instant::now() + STAND_IN_WAIT;
            let mut taken = Vec::new();
            for answer in answers {
                let mut stream = loop {
                    match serving_listener.accept() {
                        Ok((stream, _)) => break stream,
                        Err(e)
                            if e.kind() == ErrorKind::WouldBlock && Instant::now() < deadline =>
                        {
                            thread::sleep(Duration::from_millis(5));
                        }
                        Err(_) => return taken,
                    }
                };
                stream.set_nonblocking(false).expect("blocking stream");
                stream
                    .set_read_timeout(Some(STAND_IN_WAIT))
                    .expect("read timeout");
                let mut carried = Vec::new();
                stream
                    .read_to_end(&mut carried)
                    .expect("the relay closes its write side after its line");
                taken.push(carried);
                stream.write_all(&answer).expect("answer the relay");
            }
            taken
        });
        Self { listener, serving }
    }

    /// What each connection carried, once every expected one was served or
    /// the wait passed; and no connection beyond them is waiting.
    fn taken(self) -> Vec<Vec<u8>> {
        let taken = self.serving.join().expect("the stand-in thread");
        assert_no_connection(&self.listener);
        taken
    }
}

fn assert_no_connection(listener: &UnixListener) {
    listener.set_nonblocking(true).expect("nonblocking accept");
    match listener.accept() {
        Err(e) if e.kind() == ErrorKind::WouldBlock => {}
        Ok(_) => panic!("a connection the arm did not expect reached the stand-in"),
        Err(e) => panic!("accept failed: {e}"),
    }
}

fn line(text: &str) -> Vec<u8> {
    format!("{text}\n").into_bytes()
}

fn relay_input(input: &[u8], socket: &Path) -> (Result<(), Unreachable>, Vec<u8>) {
    let mut output = Vec::new();
    let relayed = relay(&mut Cursor::new(input.to_vec()), &mut output, socket);
    (relayed, output)
}

/// The built program run with `door` on `data`, one ping on its standard
/// input. Its working directory is a dir of its own, so an instrumented build
/// writes its profile there.
fn door_word(data: &Path) -> Output {
    let cwd = tempfile::tempdir().expect("working dir");
    let mut command = Command::new(ENGINE);
    command
        .env_clear()
        .env("ANDROMEDA_PULSE_DATA_DIR", data)
        .current_dir(cwd.path())
        .arg("door")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile);
    }
    let mut child = command.spawn().expect("spawn the console program");
    let mut stdin = child.stdin.take().expect("stdin");
    // The word may end before it reads: a refused write is no finding here.
    let _ = stdin.write_all(&line(PING));
    drop(stdin);
    child.wait_with_output().expect("the word ends")
}

fn assert_unreachable(output: &Output, case: &str) {
    assert_eq!(
        output.status.code(),
        Some(i32::from(EXIT_UNREACHABLE)),
        "{case}"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!("{UNREACHABLE}\n"),
        "{case}: the one fixed line on stderr"
    );
    assert!(output.stdout.is_empty(), "{case}: nothing on stdout");
}

/// A data dir whose `run/door/sock`, canonical, is exactly `length` bytes.
fn data_dir_with_socket_path_of(root: &Path, length: usize) -> PathBuf {
    let base = root.canonicalize().expect("the root resolves");
    let pad = length - base.as_os_str().len() - 1 - SOCKET_TAIL.len();
    let data = base.join("p".repeat(pad));
    std::fs::create_dir_all(data.join("run").join("door")).expect("run/door");
    assert_eq!(door_socket(&data).as_os_str().len(), length);
    data
}

#[test]
fn each_request_line_takes_a_connection_of_its_own_in_order_and_each_answer_passes_unchanged() {
    let data = data_dir();
    let requests = [
        PING,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"query_logs","arguments":{}}}"#,
    ];
    // Not all of it JSON: the relay passes bytes, it parses nothing.
    let answers = vec![
        line(r#"{"jsonrpc":"2.0","id":1,"result":{}}"#),
        b"not a json line\t \x01 kept as sent\n".to_vec(),
        line(r#"{"jsonrpc":"2.0","id":3,"result":{"items":[]}}"#),
    ];
    let door = StandIn::bind(&door_socket(data.path()), answers.clone());
    let socket = socket_path(data.path()).expect("the stand-in's socket resolves");

    let input = requests.map(line).concat();
    let (relayed, output) = relay_input(&input, &socket);

    assert_eq!(relayed, Ok(()), "standard input ending ends the relay");
    assert_eq!(
        door.taken(),
        requests.map(line).to_vec(),
        "each line whole with its terminator, then the end of the stream, one connection each"
    );
    assert_eq!(
        output,
        answers.concat(),
        "each answer byte for byte, in order"
    );
}

#[test]
fn a_line_the_door_does_not_answer_writes_nothing_and_still_took_one_connection() {
    let data = data_dir();
    let notification = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
    let door = StandIn::bind(&door_socket(data.path()), vec![Vec::new()]);
    let socket = socket_path(data.path()).expect("the stand-in's socket resolves");

    let (relayed, output) = relay_input(&line(notification), &socket);

    assert_eq!(relayed, Ok(()));
    assert_eq!(door.taken(), vec![line(notification)]);
    assert!(output.is_empty(), "no answer of the relay's own");
}

#[test]
fn whitespace_only_lines_open_no_connection() {
    let data = data_dir();
    let door = StandIn::bind(&door_socket(data.path()), Vec::new());
    let socket = socket_path(data.path()).expect("the stand-in's socket resolves");

    let (relayed, output) = relay_input(b"\n   \n\t\r\n \x0c \n  ", &socket);

    assert_eq!(relayed, Ok(()));
    assert!(door.taken().is_empty(), "no connection for a blank line");
    assert!(output.is_empty());
}

#[test]
fn a_line_over_the_bound_is_sent_one_byte_past_it_and_the_next_line_whole_on_its_own_connection() {
    let data = data_dir();
    let over: String = "x".repeat(MAX_REQUEST_BYTES + 4096);
    let at_bound: String = "y".repeat(MAX_REQUEST_BYTES);
    let refused = line(r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32600}}"#);
    let door = StandIn::bind(
        &door_socket(data.path()),
        vec![refused.clone(), line("next"), line("at bound")],
    );
    let socket = socket_path(data.path()).expect("the stand-in's socket resolves");

    let input = [line(&over), line(PING), line(&at_bound)].concat();
    let (relayed, output) = relay_input(&input, &socket);

    assert_eq!(relayed, Ok(()));
    let taken = door.taken();
    assert_eq!(
        taken.len(),
        3,
        "one connection per line, the rest of the long line never sent"
    );
    assert_eq!(
        taken[0],
        over.as_bytes()[..MAX_REQUEST_BYTES + 1].to_vec(),
        "the first MAX_REQUEST_BYTES + 1 bytes, then the end of the stream"
    );
    assert_eq!(
        taken[1],
        line(PING),
        "the next line whole, on its own connection"
    );
    assert_eq!(
        taken[2],
        line(&at_bound),
        "a line at the bound is sent whole"
    );
    assert_eq!(output, [refused, line("next"), line("at bound")].concat());
}

#[test]
fn a_door_that_cannot_be_reached_is_unreachable_and_nothing_is_written() {
    // No data dir.
    let root = tempfile::tempdir().expect("root");
    assert_eq!(
        socket_path(&root.path().join("absent")),
        None,
        "no data dir"
    );
    // A data dir with no socket.
    let empty = data_dir();
    assert_eq!(socket_path(empty.path()), None, "a data dir with no socket");
    // `run/door` a link to a directory outside the data dir.
    let outside = tempfile::tempdir().expect("outside dir");
    let listening = UnixListener::bind(outside.path().join("sock")).expect("bind outside");
    let linked = tempfile::tempdir().expect("linked data dir");
    std::fs::create_dir(linked.path().join("run")).expect("run");
    std::os::unix::fs::symlink(outside.path(), linked.path().join("run").join("door"))
        .expect("link run/door outside");
    assert_eq!(
        socket_path(linked.path()),
        None,
        "a door directory leading outside the data dir"
    );
    // A socket path of 108 bytes: the door's socket, reached by a second name.
    let deep = data_dir_with_socket_path_of(root.path(), 108);
    let short = data_dir();
    let short_socket = door_socket(short.path());
    let deep_door = UnixListener::bind(&short_socket).expect("bind the short socket");
    std::fs::hard_link(&short_socket, door_socket(&deep)).expect("a second name for the socket");
    assert_eq!(socket_path(&deep), None, "a socket path of 108 bytes");

    // A socket nobody accepts on: the relay itself reads it unreachable.
    let stale = data_dir();
    drop(UnixListener::bind(door_socket(stale.path())).expect("bind, then close"));
    let socket = socket_path(stale.path()).expect("the stale socket resolves");
    let (relayed, output) = relay_input(&line(PING), &socket);
    assert_eq!(relayed, Err(Unreachable));
    assert!(output.is_empty(), "nothing written");

    // The word on each of the four: exit 1, the fixed line, no connection.
    for (data, case) in [
        (root.path().join("absent"), "no data dir"),
        (empty.path().to_path_buf(), "no socket"),
        (
            linked.path().to_path_buf(),
            "a door directory leading outside",
        ),
        (deep.clone(), "a socket path of 108 bytes"),
    ] {
        assert_unreachable(&door_word(&data), case);
    }
    assert!(
        !root.path().join("absent").exists(),
        "the word creates no data dir"
    );
    assert_no_connection(&listening);
    assert_no_connection(&deep_door);
}

#[test]
fn a_socket_path_of_107_bytes_is_reached() {
    let root = tempfile::tempdir().expect("root");
    let data = data_dir_with_socket_path_of(root.path(), 107);
    let door = StandIn::bind(&door_socket(&data), vec![line("pong")]);

    let output = door_word(&data);

    assert_eq!(output.status.code(), Some(0), "standard input ended");
    assert!(output.stderr.is_empty(), "nothing on stderr");
    assert_eq!(output.stdout, line("pong"));
    assert_eq!(door.taken(), vec![line(PING)]);
}

struct RefusingOutput;

impl Write for RefusingOutput {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(ErrorKind::BrokenPipe))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::from(ErrorKind::BrokenPipe))
    }
}

#[test]
fn an_answer_that_cannot_be_written_out_ends_the_relay_unreachable() {
    let data = data_dir();
    let door = StandIn::bind(&door_socket(data.path()), vec![line("pong")]);
    let socket = socket_path(data.path()).expect("the stand-in's socket resolves");

    let relayed = relay(
        &mut Cursor::new([line(PING), line(PING)].concat()),
        &mut RefusingOutput,
        &socket,
    );

    assert_eq!(relayed, Err(Unreachable));
    assert_eq!(
        door.taken(),
        vec![line(PING)],
        "no request after the failed write"
    );
}

#[test]
fn the_fixed_line_and_its_exit_code() {
    assert_eq!(
        UNREACHABLE,
        "andromeda-pulse-engine door: the door cannot be reached"
    );
    assert_eq!(EXIT_UNREACHABLE, 1);
}
