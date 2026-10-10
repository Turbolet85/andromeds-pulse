//! The console program's command grammar (`pulse_app::console`), pinned in
//! process: each word of the closed set, no word, an unknown word and an extra
//! word. `run` itself is never called here: the spawned program is driven by
//! `integration_console_engine.rs`.

use std::process::ExitCode;

use pulse_app::console::{Command, EXIT_USAGE, PROGRAM_NAME, USAGE, main, parse, version_line};

const NO_WORDS: [&str; 0] = [];

// `ExitCode` has no `PartialEq`; its `Debug` form carries the code.
fn code(exit: ExitCode) -> String {
    format!("{exit:?}")
}

#[test]
fn run_is_the_run_command() {
    assert_eq!(parse(["run"]), Some(Command::Run));
}

#[test]
fn version_is_the_version_command() {
    assert_eq!(parse(["version"]), Some(Command::Version));
}

#[test]
fn no_word_is_refused() {
    assert_eq!(parse(NO_WORDS), None);
}

#[test]
fn an_unknown_word_is_refused() {
    for word in [
        "stop",
        "start",
        "status",
        "--help",
        "-h",
        "--version",
        "RUN",
        "Run",
        "run ",
        " version",
        "",
    ] {
        assert_eq!(parse([word]), None, "`{word}` is outside the closed set");
    }
}

#[test]
fn an_extra_word_is_refused() {
    assert_eq!(parse(["run", "extra"]), None);
    assert_eq!(parse(["version", "run"]), None);
    assert_eq!(parse(["run", "run"]), None);
}

#[cfg(unix)]
#[test]
fn a_word_that_is_not_unicode_is_refused() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    assert_eq!(
        parse([OsString::from_vec(vec![b'r', b'u', b'n', 0xff])]),
        None
    );
}

#[test]
fn a_refused_command_line_exits_two() {
    assert_eq!(EXIT_USAGE, 2);
    let usage = code(ExitCode::from(2));
    assert_eq!(code(main(NO_WORDS)), usage, "no word");
    assert_eq!(code(main(["stop"])), usage, "an unknown word");
    assert_eq!(code(main(["version", "extra"])), usage, "an extra word");
}

#[test]
fn version_exits_zero_and_names_the_program_and_its_version() {
    assert_eq!(
        version_line(),
        format!("andromeda-pulse-engine {}", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(code(main(["version"])), code(ExitCode::SUCCESS));
}

#[test]
fn the_usage_line_names_the_program_and_both_words() {
    assert_eq!(PROGRAM_NAME, "andromeda-pulse-engine");
    assert_eq!(USAGE, "usage: andromeda-pulse-engine <run|version>");
}
