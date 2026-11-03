//! The built `harness:engine-cycle` verb under a constructed environment or
//! argument: the refusals it makes before anything is started, and a cycle
//! that fails on a real engine.
//!
//! A refusal is returned before the verb builds or boots anything, so no
//! engine exists to read; the constructed environment is the whole reading.
//! The failing cycle runs the committed script on the engine built beside
//! this test binary: a stand-in `cargo` skips the builds and fails
//! `harness:engine-settled`, and a stand-in `bash` moves the script into a
//! scratch root that holds the built engine, since the verb runs it from the
//! workspace root.

#![cfg(target_os = "linux")]

mod common;

use std::process::Command;
use std::time::Duration;

use common::{Cargo, Ran, Scratch};
use serde_json::Value;

const VERBS: [&str; 5] = ["boot", "settled", "status", "cleanup", "check"];
const ENGINE_WAIT: Duration = Duration::from_secs(60);

/// The built cycle verb on this run's ports, under an environment that holds
/// nothing but the coverage profile variable when this process has one.
fn cycle(scratch: &Scratch, data_dir: &std::path::Path) -> Command {
    let mut command = Command::new(common::XTASK);
    command
        .args(["harness:engine-cycle", "--data-dir"])
        .arg(data_dir)
        .args(["--grpc-port", &scratch.grpc.to_string()])
        .args(["--http-port", &scratch.http.to_string()])
        .current_dir(scratch.path("root"))
        .env_clear()
        .stdin(std::process::Stdio::null());
    if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
        command.env("LLVM_PROFILE_FILE", profile);
    }
    command
}

fn verdict(scratch: &Scratch, ran: &Ran) -> Value {
    serde_json::from_str(&ran.stdout)
        .unwrap_or_else(|_| panic!("stdout is one verdict: {}", scratch.printed(ran)))
}

/// Exit 2, the label on stderr, a verdict that ran no verb, and no log
/// family under the data dir.
fn assert_refused(scratch: &Scratch, ran: &Ran, label: &str) {
    assert_eq!(ran.code, Some(2), "{}", scratch.printed(ran));
    assert!(
        ran.stderr
            .contains(&format!("harness:engine-cycle: cannot-evaluate ({label})")),
        "{}",
        scratch.printed(ran)
    );
    let verdict = verdict(scratch, ran);
    assert_eq!(verdict["verdict"], "cannot-evaluate");
    for verb in VERBS {
        assert!(verdict[verb].is_null(), "{verb}: {verdict}");
    }
    assert!(verdict["error_records"].is_null(), "{verdict}");
    assert_eq!(verdict["witness_file"], "absent");
    assert!(
        scratch.log_family().is_empty(),
        "no engine wrote a log under the data dir"
    );
}

fn assert_both_ports_refuse(scratch: &Scratch) {
    assert!(
        !common::accepts(scratch.grpc) && !common::accepts(scratch.http),
        "both ports refuse, as before the verb ran"
    );
}

#[test]
fn a_cycle_with_no_home_is_refused_as_home_unset() {
    let scratch = Scratch::new();
    scratch.stand_in_cargo(Cargo::SkipsBuilds);
    let ran = scratch.run(cycle(&scratch, &scratch.data()).env("PATH", scratch.search_path()));
    assert_refused(&scratch, &ran, "home-unset");
    assert!(scratch.cargo_calls().is_empty(), "nothing was built");
    assert_both_ports_refuse(&scratch);
}

#[test]
fn a_cycle_with_no_path_is_refused_as_path_unset() {
    let scratch = Scratch::new();
    scratch.stand_in_cargo(Cargo::SkipsBuilds);
    let ran = scratch.run(cycle(&scratch, &scratch.data()).env("HOME", scratch.path("home")));
    assert_refused(&scratch, &ran, "path-unset");
    assert!(scratch.cargo_calls().is_empty(), "nothing was built");
    assert_both_ports_refuse(&scratch);
}

#[test]
fn a_cycle_whose_data_dir_sits_under_a_regular_file_is_refused_as_data_dir_unusable() {
    let scratch = Scratch::new();
    scratch.stand_in_cargo(Cargo::SkipsBuilds);
    let file = scratch.path("a-file");
    std::fs::write(&file, "").expect("write a regular file");
    let ran = scratch.run(
        cycle(&scratch, &file.join("data"))
            .env("HOME", scratch.path("home"))
            .env("PATH", scratch.search_path()),
    );
    assert_refused(&scratch, &ran, "data-dir-unusable");
    assert!(scratch.cargo_calls().is_empty(), "nothing was built");
    assert_both_ports_refuse(&scratch);
}

#[test]
fn a_cycle_whose_injector_build_fails_is_refused_as_injector_build_failed() {
    let scratch = Scratch::new();
    scratch.stand_in_cargo(Cargo::FailsEverything);
    let ran = scratch.run(
        cycle(&scratch, &scratch.data())
            .env("HOME", scratch.path("home"))
            .env("PATH", scratch.search_path()),
    );
    assert_refused(&scratch, &ran, "injector-build-failed");
    assert_eq!(
        scratch.cargo_calls(),
        ["build -p ingest --example inject_demo"],
        "the one build call, and nothing after it"
    );
    assert_both_ports_refuse(&scratch);
}

#[test]
fn a_cycle_over_a_port_another_listener_holds_is_refused_as_port_taken() {
    for held_is_grpc in [true, false] {
        let scratch = Scratch::new();
        scratch.stand_in_cargo(Cargo::SkipsBuilds);
        let (held, free) = if held_is_grpc {
            (scratch.grpc, scratch.http)
        } else {
            (scratch.http, scratch.grpc)
        };
        let _listener = common::listen(held);
        let ran = scratch.run(
            cycle(&scratch, &scratch.data())
                .env("HOME", scratch.path("home"))
                .env("PATH", scratch.search_path()),
        );
        assert_refused(&scratch, &ran, "port-taken");
        assert!(scratch.cargo_calls().is_empty(), "nothing was built");
        assert!(!scratch.data().exists(), "no data dir was made");
        assert!(common::accepts(held), "the holder's listener still accepts");
        assert!(!common::accepts(free), "the other port still refuses");
    }
}

#[test]
fn a_cycle_that_fails_on_a_real_engine_ends_the_engine_it_started() {
    let scratch = Scratch::new();
    scratch.link_engine();
    scratch.stand_in_cargo(Cargo::FailsXtask("harness:engine-settled"));
    scratch.stand_in_bash();
    let ran = scratch.run(
        cycle(&scratch, &scratch.data())
            .env("HOME", scratch.path("home"))
            .env("PATH", scratch.search_path()),
    );

    assert_eq!(ran.code, Some(1), "{}", scratch.printed(&ran));
    let verdict = verdict(&scratch, &ran);
    assert_eq!(verdict["verdict"], "fail", "{verdict}");
    assert_eq!(verdict["boot"], 0, "the engine read ready: {verdict}");
    assert_eq!(verdict["settled"], 1, "the settle verb failed: {verdict}");
    assert_eq!(verdict["cleanup"], 0, "{verdict}");
    assert!(
        ran.stderr.contains("cleanup: clean"),
        "{}",
        scratch.printed(&ran)
    );

    assert!(
        !scratch.log_family().is_empty(),
        "a real engine wrote its log under the data dir"
    );
    let pid = scratch.spawned_pid().expect("the spawn record names a pid");
    assert!(
        common::wait_until(ENGINE_WAIT, || common::pid_ended(pid)),
        "the engine the cycle started is ended"
    );
    assert_both_ports_refuse(&scratch);
}
