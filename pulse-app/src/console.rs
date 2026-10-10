//! The console program: the engine with no window, driven by a closed command
//! grammar. `run` boots the engine through `engine_boot` and stays up until it
//! is signalled; `version` prints one line; anything else is refused.

use std::ffi::OsStr;
use std::process::ExitCode;
use std::sync::Arc;

use triage::contract::{HardwareProfileSource, UnknownHardwareProfile};

use crate::deterministic_inference::{DeterministicInferenceRunner, deterministic_mode_enabled};
use crate::engine_boot::{self, EngineConfig, EngineInputs, InterpretationSeat, Program, SeatKind};
use crate::model_router::tier_for_profile;

pub const PROGRAM_NAME: &str = "andromeda-pulse-engine";
pub const USAGE: &str = "usage: andromeda-pulse-engine <run|version>";
pub const EXIT_USAGE: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Run,
    Version,
}

/// Exactly one word from the closed set; no word, an unknown word or a second
/// word is `None`.
pub fn parse<I, S>(args: I) -> Option<Command>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut args = args.into_iter();
    let command = match args.next()?.as_ref().to_str() {
        Some("run") => Command::Run,
        Some("version") => Command::Version,
        _ => return None,
    };
    if args.next().is_some() {
        return None;
    }
    Some(command)
}

pub fn version_line() -> String {
    format!("{PROGRAM_NAME} {}", env!("CARGO_PKG_VERSION"))
}

/// The program's whole behaviour for the arguments after its own name.
pub fn main<I, S>(args: I) -> ExitCode
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    match parse(args) {
        Some(Command::Run) => run(),
        Some(Command::Version) => {
            println!("{}", version_line());
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("{USAGE}");
            ExitCode::from(EXIT_USAGE)
        }
    }
}

fn run() -> ExitCode {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    let _enter = runtime.enter();

    let data_dir = engine_boot::resolve_data_dir();
    engine_boot::init_process(&data_dir);

    let hardware_profile: Arc<dyn HardwareProfileSource> = Arc::new(UnknownHardwareProfile);
    let interpretation = deterministic_mode_enabled().then(|| InterpretationSeat {
        runner: Arc::new(DeterministicInferenceRunner::new(tier_for_profile(
            hardware_profile.current_profile(),
        ))),
        kind: SeatKind::Deterministic,
    });
    let _engine = engine_boot::start(
        EngineConfig::from_env(data_dir),
        EngineInputs {
            program: Program::Console,
            key_backend: engine_boot::os_key_backend(),
            hardware_profile,
            interpretation,
        },
    );

    // Never completes: the process ends by SIGTERM or SIGINT through the signal
    // listener, which records the end and re-raises the signal.
    runtime.block_on(std::future::pending())
}
