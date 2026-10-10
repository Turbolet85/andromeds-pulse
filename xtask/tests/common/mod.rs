//! What the shell-level suites share: the built binaries, one scratch area
//! per run, the stand-in `cargo`, and bounded reads of `/proc`.
//!
//! A scratch area holds a root the committed script runs in, a `bin`
//! directory placed first on the child's PATH, a home, a temp dir and a data
//! dir, plus two claimed loopback ports and a passphrase made for the run.
//! Dropping it ends any engine its data dir still names.

#![allow(dead_code)]

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

pub const XTASK: &str = env!("CARGO_BIN_EXE_xtask");
pub const ENGINE_NAME: &str = "andromeda-pulse-engine";
const SCRIPT: &str = "scripts/agent-run.sh";
const SYSTEM_PATH: &str = "/usr/local/bin:/usr/bin:/bin";
const POLL: Duration = Duration::from_millis(50);

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the xtask package sits under the workspace root")
        .to_path_buf()
}

pub fn script() -> PathBuf {
    let script = repo_root().join(SCRIPT);
    assert!(script.is_file(), "{SCRIPT} MUST be in the checkout");
    script
}

/// The engine built into this test binary's own profile directory.
pub fn engine_bin() -> PathBuf {
    let exe = std::env::current_exe().expect("this test binary's own path");
    let engine = exe
        .parent()
        .and_then(Path::parent)
        .expect("a test binary sits under {profile}/deps")
        .join(ENGINE_NAME);
    assert!(
        engine.is_file(),
        "the built `{ENGINE_NAME}` MUST sit in this test binary's profile directory: \
         a `--workspace` run builds it"
    );
    engine
}

pub fn real_bash() -> PathBuf {
    let path = std::env::var_os("PATH").expect("PATH MUST be set: the pins run `bash`");
    std::env::split_paths(&path)
        .map(|dir| dir.join("bash"))
        .find(|candidate| candidate.is_file())
        .expect("`bash` MUST be on PATH: scripts/agent-run.sh is a bash script")
}

fn require_proc() {
    assert!(
        Path::new("/proc/self/stat").is_file(),
        "`/proc` MUST be mounted: the pins read process state from it"
    );
}

/// A loopback port claimed for one run. It lies below the kernel's ephemeral
/// range, so no bind to port 0 is ever handed it, and an advisory lock under
/// `target/` holds it against every other test of this workspace for as long
/// as the claim lives. A port bound to 0, read and released is taken again by
/// another test often enough to fail a run when many start at once.
pub struct PortClaim {
    pub number: u16,
    _lock: std::fs::File,
}

const CLAIMED_PORTS: std::ops::Range<u16> = 20_000..32_000;
/// The harness's own registered ports inside that range.
const HARNESS_PORTS: [u16; 2] = [24_317, 24_318];

fn random_word() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

pub fn claim_port() -> PortClaim {
    let claims = repo_root().join("target").join("test-ports");
    std::fs::create_dir_all(&claims).expect("the port claims directory");
    let span = u64::from(CLAIMED_PORTS.end - CLAIMED_PORTS.start);
    for _ in 0..10_000 {
        let offset = u16::try_from(random_word() % span).expect("an offset inside the range");
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

pub fn listen(port: u16) -> TcpListener {
    TcpListener::bind(("127.0.0.1", port)).expect("the test's own listener binds its port")
}

pub fn accepts(port: u16) -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok()
}

/// Polls `condition` until it holds or `bound` has passed.
pub fn wait_until(bound: Duration, mut condition: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + bound;
    loop {
        if condition() {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(POLL);
    }
}

pub fn read_pid(path: &Path) -> Option<u32> {
    std::fs::read_to_string(path).ok()?.trim().parse().ok()
}

/// The state letter of `/proc/{pid}/stat`; `None` for a pid that is not
/// there.
pub fn pid_state(pid: u32) -> Option<char> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    stat[stat.rfind(')')? + 1..].trim_start().chars().next()
}

/// Gone, or exited and waiting for its parent to read its status.
pub fn pid_ended(pid: u32) -> bool {
    matches!(pid_state(pid), None | Some('Z'))
}

pub fn is_engine(pid: u32) -> bool {
    std::fs::read(format!("/proc/{pid}/cmdline"))
        .is_ok_and(|line| String::from_utf8_lossy(&line).contains(ENGINE_NAME))
}

/// How many processes sit in process group `pgid`.
pub fn group_members(pgid: u32) -> usize {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return 0;
    };
    entries
        .flatten()
        .filter_map(|entry| entry.file_name().to_string_lossy().parse::<u32>().ok())
        .filter(|pid| {
            std::fs::read_to_string(format!("/proc/{pid}/stat"))
                .ok()
                .and_then(|stat| {
                    let rest = stat[stat.rfind(')')? + 1..].to_owned();
                    rest.split_whitespace().nth(2)?.parse::<u32>().ok()
                })
                == Some(pgid)
        })
        .count()
}

/// `kill -{signal} -- {target}` by the shell's builtin; a negative target is
/// a process group.
pub fn signal(signal: &str, target: &str) -> bool {
    Command::new(real_bash())
        .args(["-c", r#"kill "-$1" -- "$2""#, "kill", signal, target])
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// A file a child will execute, written by a child of its own so this
/// process never holds it open for writing.
fn write_executable(path: &Path, text: &str) {
    let written = Command::new(real_bash())
        .args([
            "-c",
            r#"printf '%s' "$1" > "$2" && chmod 755 "$2""#,
            "write",
        ])
        .arg(text)
        .arg(path)
        .status()
        .expect("run bash");
    assert!(written.success(), "the stand-in was written");
}

fn run_passphrase() -> String {
    format!("{:016x}{:016x}", random_word(), random_word())
}

/// What the stand-in `cargo` does with a call.
pub enum Cargo {
    /// `build` succeeds without building; `xtask` runs the built xtask.
    SkipsBuilds,
    /// As `SkipsBuilds`, but a `build` call whose arguments hold the text
    /// exits 1.
    FailsBuildOf(&'static str),
    /// As `SkipsBuilds`, but the named `xtask` verb exits 1.
    FailsXtask(&'static str),
    /// Every call exits 1.
    FailsEverything,
}

pub struct Ran {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl From<Output> for Ran {
    fn from(output: Output) -> Self {
        Self {
            code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

pub struct Scratch {
    dir: tempfile::TempDir,
    pub grpc: u16,
    pub http: u16,
    _ports: [PortClaim; 2],
    passphrase: String,
}

impl Scratch {
    pub fn new() -> Self {
        require_proc();
        let dir = tempfile::tempdir().expect("a scratch area");
        for sub in ["bin", "home", "tmp", "root"] {
            std::fs::create_dir(dir.path().join(sub)).expect("a scratch directory");
        }
        let ports = [claim_port(), claim_port()];
        Self {
            dir,
            grpc: ports[0].number,
            http: ports[1].number,
            _ports: ports,
            passphrase: run_passphrase(),
        }
    }

    pub fn path(&self, sub: &str) -> PathBuf {
        self.dir.path().join(sub)
    }

    pub fn data(&self) -> PathBuf {
        self.path("data")
    }

    pub fn run_file(&self, name: &str) -> PathBuf {
        self.data().join("run").join(name)
    }

    /// The pid `boot`'s waiting wrapper recorded for the program it spawned.
    pub fn spawned_pid(&self) -> Option<u32> {
        read_pid(&self.run_file("andromeda-pulse.spawn"))
    }

    /// The names under the data dir's `logs/` that belong to the log family.
    pub fn log_family(&self) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(self.data().join("logs")) else {
            return Vec::new();
        };
        let mut names: Vec<String> = entries
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with("agent-latest.jsonl"))
            .collect();
        names.sort();
        names
    }

    /// `target/release/{engine}` under the scratch root, naming the built
    /// engine: where the script looks for the binary it spawns.
    pub fn link_engine(&self) {
        let release = self.path("root").join("target").join("release");
        std::fs::create_dir_all(&release).expect("the scratch release dir");
        std::os::unix::fs::symlink(engine_bin(), release.join(ENGINE_NAME))
            .expect("link the built engine");
    }

    pub fn stand_in_cargo(&self, cargo: Cargo) {
        let calls = self.path("bin").join("cargo.calls");
        let record = format!("echo \"$*\" >> '{}'", calls.display());
        let xtask = format!("exec '{XTASK}' \"$@\"");
        let arms = match cargo {
            Cargo::FailsEverything => "exit 1".to_owned(),
            Cargo::SkipsBuilds => {
                format!("case \"$1\" in build) exit 0 ;; xtask) shift; {xtask} ;; esac")
            }
            Cargo::FailsBuildOf(text) => format!(
                "case \"$1\" in build) case \"$*\" in *'{text}'*) exit 1 ;; esac; exit 0 ;; \
                 xtask) shift; {xtask} ;; esac"
            ),
            Cargo::FailsXtask(verb) => format!(
                "case \"$1\" in build) exit 0 ;; \
                 xtask) shift; case \"$1\" in '{verb}') exit 1 ;; esac; {xtask} ;; esac"
            ),
        };
        write_executable(
            &self.path("bin").join("cargo"),
            &format!("#!/bin/sh\n{record}\n{arms}\nexit 64\n"),
        );
    }

    /// Every call the stand-in `cargo` received, one line each.
    pub fn cargo_calls(&self) -> Vec<String> {
        std::fs::read_to_string(self.path("bin").join("cargo.calls"))
            .map(|text| text.lines().map(str::to_owned).collect())
            .unwrap_or_default()
    }

    /// A stand-in `bash` for a caller that runs the committed script from
    /// the workspace root: it moves into the scratch root, where the script
    /// finds the built engine, widens the readiness window and hands every
    /// other call to the real shell unchanged.
    pub fn stand_in_bash(&self) {
        let bash = real_bash();
        write_executable(
            &self.path("bin").join("bash"),
            &format!(
                "#!{bash}\ncase \"$1\" in {SCRIPT}) script=\"$PWD/$1\"; shift; \
                 cd '{root}' || exit 97; HARNESS_STATUS_TIMEOUT=60 exec '{bash}' \"$script\" \"$@\" ;; \
                 esac\nexec '{bash}' \"$@\"\n",
                bash = bash.display(),
                root = self.path("root").display(),
            ),
        );
    }

    /// The scratch `bin` first, then the system directories.
    pub fn search_path(&self) -> String {
        format!("{}:{SYSTEM_PATH}", self.path("bin").display())
    }

    /// A child under a cleared environment that holds this run's home and
    /// search path alone.
    pub fn cleared(&self, program: impl AsRef<std::ffi::OsStr>) -> Command {
        let mut command = Command::new(program);
        command
            .env_clear()
            .env("HOME", self.path("home"))
            .env("PATH", self.search_path())
            .stdin(Stdio::null());
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        command
    }

    /// The committed script by `bash` in the scratch root, under a cleared
    /// environment that names this run's data dir, ports, temp dir and
    /// passphrase.
    pub fn script(&self, args: &[&str]) -> Command {
        let mut command = self.cleared(real_bash());
        command
            .arg(script())
            .args(args)
            .current_dir(self.path("root"))
            .env("TMPDIR", self.path("tmp"))
            .env("ANDROMEDA_PULSE_DATA_DIR", self.data())
            .env("ANDROMEDA_PULSE_OTLP_GRPC_PORT", self.grpc.to_string())
            .env("ANDROMEDA_PULSE_OTLP_HTTP_PORT", self.http.to_string())
            .env("ANDROMEDA_PULSE_CORPUS_PASSPHRASE", &self.passphrase)
            .env("HARNESS_STATUS_TIMEOUT", "60");
        command
    }

    pub fn run(&self, command: &mut Command) -> Ran {
        command.output().expect("run the child").into()
    }

    /// `text` with this run's scratch path replaced, so a failed assertion
    /// prints no path of the host.
    pub fn shown(&self, text: &str) -> String {
        text.replace(&*self.dir.path().to_string_lossy(), "{scratch}")
            .replace(&self.passphrase, "{passphrase}")
    }

    /// Both streams of a run, as a failed assertion may print them.
    pub fn printed(&self, ran: &Ran) -> String {
        self.shown(&format!(
            "exit {:?}\n--- stdout\n{}--- stderr\n{}",
            ran.code, ran.stdout, ran.stderr
        ))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        for record in ["andromeda-pulse.pid", "andromeda-pulse.spawn"] {
            if let Some(pid) = read_pid(&self.run_file(record))
                && is_engine(pid)
            {
                signal("KILL", &pid.to_string());
            }
        }
    }
}
