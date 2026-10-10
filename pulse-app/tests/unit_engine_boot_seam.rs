//! The seam between the two programs and the one engine boot they share.
//!
//! Read from the sources themselves: each entry point calls the two boot
//! functions once and wires no engine part itself; the shared boot names no
//! window framework; the console program names no model, no hardware probe
//! and none of the window's first statements. A part put back into an entry
//! point, or the framework named in the shared boot, turns a pin red.

use std::path::Path;

const WINDOW_ENTRY: &str = "main.rs";
const CONSOLE_ENTRY: &str = "console.rs";
const CONSOLE_BIN: &str = "bin/andromeda-pulse-engine.rs";
const SHARED_BOOT: &str = "engine_boot.rs";

const BOOT_START: &str = "engine_boot::start(";
const BOOT_INIT_PROCESS: &str = "engine_boot::init_process(";

/// The engine's parts, as call text. Closed: a part added to the shared boot
/// is added here.
const ENGINE_PARTS: [&str; 17] = [
    "run_consumer(",
    "run_retention(",
    "start_emitter(",
    "start_cadence_coordinator(",
    "start_lifecycle_heartbeat(",
    "start_restart_detector(",
    "start_storm_detector(",
    "run_persist_loop(",
    "run_lifecycle_persist_loop(",
    "run_storm_persist_loop(",
    "run_incident_persist_loop(",
    "run_auto_resolution_loop(",
    "grpc::serve_on(",
    "http::serve_on(",
    "spawn_l4_inference_subscriber(",
    "Corpus::open(",
    "spawn_engine_ticks(",
];

const WINDOW_FRAMEWORK: [&str; 2] = ["tauri", "taurpc"];

/// What the console program must not name: the model runner and its path
/// variables, the hardware probe and its variable, the harness's two locator
/// variables, and the window's first two statements.
const MODEL_AND_WINDOW_START: [&str; 4] = [
    "LlamaCliInference",
    "HardwareProfileDetector",
    "render_posture",
    "xlib_threads",
];
const VARIABLES_THE_CONSOLE_NEVER_READS: [&str; 7] = [
    "ANDROMEDA_PULSE_MODEL_PATH",
    "ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH",
    "ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH",
    "ANDROMEDA_PULSE_L4_ALLOW_ROOT",
    "ANDROMEDA_PULSE_HARDWARE_PROFILE",
    "ANDROMEDA_PULSE_PIDFILE",
    "ANDROMEDA_PULSE_LOGFILE",
];

fn source(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read src/{name}: {e}"))
}

#[test]
fn each_entry_point_calls_the_two_boot_functions_once() {
    for name in [WINDOW_ENTRY, CONSOLE_ENTRY] {
        let text = source(name);
        assert_eq!(
            text.matches(BOOT_START).count(),
            1,
            "{name} calls the engine boot once"
        );
        assert_eq!(
            text.matches(BOOT_INIT_PROCESS).count(),
            1,
            "{name} calls the process start once"
        );
    }
}

#[test]
fn the_console_bin_only_hands_its_arguments_to_the_console_module() {
    let text = source(CONSOLE_BIN);
    assert_eq!(text.matches("pulse_app::console::main(").count(), 1);
    assert_eq!(text.matches(BOOT_START).count(), 0);
    assert_eq!(text.matches(BOOT_INIT_PROCESS).count(), 0);
}

#[test]
fn no_entry_point_wires_an_engine_part() {
    for name in [WINDOW_ENTRY, CONSOLE_ENTRY, CONSOLE_BIN] {
        let text = source(name);
        for part in ENGINE_PARTS {
            assert!(
                !text.contains(part),
                "{name} names the engine part `{part}`; it belongs to the shared boot"
            );
        }
    }
}

// The known positive of the pin above: every part it forbids is spelled as the
// shared boot spells it, so a renamed part cannot leave the pin reading nothing.
#[test]
fn the_shared_boot_wires_every_engine_part() {
    let text = source(SHARED_BOOT);
    for part in ENGINE_PARTS {
        assert!(text.contains(part), "the shared boot calls `{part}`");
    }
}

#[test]
fn the_shared_boot_names_no_window_framework() {
    let text = source(SHARED_BOOT).to_lowercase();
    for word in WINDOW_FRAMEWORK {
        assert!(!text.contains(word), "the shared boot names `{word}`");
    }
}

#[test]
fn the_console_program_names_no_window_framework() {
    for name in [CONSOLE_ENTRY, CONSOLE_BIN] {
        let text = source(name).to_lowercase();
        for word in WINDOW_FRAMEWORK {
            assert!(!text.contains(word), "{name} names `{word}`");
        }
    }
}

#[test]
fn the_console_program_names_no_model_no_probe_and_no_window_start() {
    for name in [CONSOLE_ENTRY, CONSOLE_BIN] {
        let text = source(name);
        for word in MODEL_AND_WINDOW_START
            .iter()
            .chain(VARIABLES_THE_CONSOLE_NEVER_READS.iter())
        {
            assert!(!text.contains(word), "{name} names `{word}`");
        }
    }
}

// The known positive of the pin above: the window entry names all four, so the
// words are spelled as the sources spell them.
#[test]
fn the_window_entry_names_the_model_the_probe_and_its_first_statements() {
    let text = source(WINDOW_ENTRY);
    for word in MODEL_AND_WINDOW_START {
        assert!(text.contains(word), "the window entry names `{word}`");
    }
}
