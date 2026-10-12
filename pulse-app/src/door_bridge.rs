//! The relay a stdio agent starts: `andromeda-pulse-engine door`.
//!
//! Each request line read on standard input is carried to the door of the
//! engine running on the data dir, on a connection of its own: the line is
//! written, the write side is closed, and what the door answers before it
//! closes is copied to standard output unchanged. One connection per request
//! keeps every connection shorter than the door's idle bound.
//!
//! The relay is a client of the door and nothing more: the door reads its
//! account as it reads any caller's. It parses no request, answers none on the
//! door's behalf and retries none; it writes no record and no file, and its one
//! line on standard error is [`UNREACHABLE`].

use std::process::ExitCode;

pub const UNREACHABLE: &str = "andromeda-pulse-engine door: the door cannot be reached";
pub const EXIT_UNREACHABLE: u8 = 1;

#[cfg(unix)]
pub use unix::{relay, socket_path};

/// The door could not be reached, or its answer could not be written out.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unreachable;

/// The word's whole behaviour. Standard input ending is exit 0; anything else
/// that ends the relay is [`UNREACHABLE`] and exit 1. Off Unix no door is
/// seated, so the word ends there at once.
pub fn main() -> ExitCode {
    #[cfg(unix)]
    {
        let relayed = socket_path(&crate::engine_boot::resolve_data_dir())
            .ok_or(Unreachable)
            .and_then(|socket| {
                relay(
                    &mut std::io::stdin().lock(),
                    &mut std::io::stdout().lock(),
                    &socket,
                )
            });
        if relayed.is_ok() {
            return ExitCode::SUCCESS;
        }
    }
    eprintln!("{UNREACHABLE}");
    ExitCode::from(EXIT_UNREACHABLE)
}

#[cfg(unix)]
mod unix {
    use std::fs;
    use std::io::{self, BufRead, Read, Write};
    use std::net::Shutdown;
    use std::os::unix::net::UnixStream;
    use std::path::{Path, PathBuf};

    use super::Unreachable;
    use crate::door::MAX_REQUEST_BYTES;

    // A socket address holds 108 bytes on Linux, its terminator among them.
    const SOCKET_ADDRESS_BYTES: usize = 108;

    /// `{data dir}/run/door/sock`, both sides canonical. `None` when either
    /// does not resolve, when the socket resolves outside the data dir, or
    /// when its path is too long to connect to.
    #[doc(hidden)]
    pub fn socket_path(data_dir: &Path) -> Option<PathBuf> {
        let data = fs::canonicalize(data_dir).ok()?;
        let socket = fs::canonicalize(data.join("run").join("door").join("sock")).ok()?;
        (socket.starts_with(&data) && socket.as_os_str().len() < SOCKET_ADDRESS_BYTES)
            .then_some(socket)
    }

    /// Carries `input` to the door line by line until `input` ends. A line of
    /// whitespace alone opens nothing.
    #[doc(hidden)]
    pub fn relay<R: BufRead, W: Write>(
        input: &mut R,
        output: &mut W,
        socket: &Path,
    ) -> Result<(), Unreachable> {
        let mut line = Vec::new();
        loop {
            line.clear();
            // A failed read of standard input ends the relay as its end does.
            let Ok(read) =
                Read::take(&mut *input, MAX_REQUEST_BYTES as u64 + 1).read_until(b'\n', &mut line)
            else {
                return Ok(());
            };
            if read == 0 {
                return Ok(());
            }
            if line.last() == Some(&b'\n') {
                line.pop();
            } else if line.len() > MAX_REQUEST_BYTES {
                // One byte past the bound, as the door reads it, so the door
                // refuses it; the rest of the line is never sent.
                exchange(socket, &line, output)?;
                if discard_line(input).is_err() {
                    return Ok(());
                }
                continue;
            }
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            line.push(b'\n');
            exchange(socket, &line, output)?;
        }
    }

    /// One request on a connection of its own. The answer is read whole before
    /// any of it is written out, so a connection that fails writes nothing.
    fn exchange<W: Write>(
        socket: &Path,
        request: &[u8],
        output: &mut W,
    ) -> Result<(), Unreachable> {
        let failed = |_| Unreachable;
        let mut stream = UnixStream::connect(socket).map_err(failed)?;
        stream.write_all(request).map_err(failed)?;
        stream.shutdown(Shutdown::Write).map_err(failed)?;
        let mut answer = Vec::new();
        stream.read_to_end(&mut answer).map_err(failed)?;
        output.write_all(&answer).map_err(failed)?;
        output.flush().map_err(failed)
    }

    fn discard_line<R: BufRead>(input: &mut R) -> io::Result<()> {
        loop {
            let buffered = input.fill_buf()?;
            if buffered.is_empty() {
                return Ok(());
            }
            if let Some(end) = buffered.iter().position(|byte| *byte == b'\n') {
                input.consume(end + 1);
                return Ok(());
            }
            let all = buffered.len();
            input.consume(all);
        }
    }
}
