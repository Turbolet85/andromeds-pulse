// `capability_drift_tests` mod is followed by `test_a11y_placeholder` and
// `status_to_code` helpers; clippy::items_after_test_module flags this as a
// "restriction" lint, but reorganizing this file's helper layout for a single
// chunk's tests yields churn without correctness benefit. The tests are
// `#[cfg(test)]` and excluded from release builds; non-test items after them
// stay accessible to the rest of the file unchanged.
#![allow(clippy::items_after_test_module)]

mod bundle_format;
mod discovery;
mod external_resolve;
mod gap_resume;
mod harness_ready;
mod harness_series;
mod harness_status;
mod harness_witness;
mod hue_shift;
mod ingest_progress;
#[cfg(test)]
mod license_check;
mod npm_gate;
mod perf_budget;
mod perf_frame;
mod pre_push;
#[cfg(test)]
mod rust_floor;
mod self_verify;
mod smoke;
mod source_lint;
mod staged_gate;
mod webview_drive;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde_json::Value;

use crate::bundle_format::BundleFormat;

#[derive(Parser)]
#[command(name = "xtask", about = "andromeda-pulse task runner")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    #[command(name = "harness:status")]
    HarnessStatus,
    #[command(
        name = "harness:ready",
        about = "The boot verb's readiness verdict: the harness:status verdict reads running-healthy AND both OTLP receivers accept a TCP connection on 127.0.0.1 at the ports resolved from ANDROMEDA_PULSE_OTLP_GRPC_PORT / _HTTP_PORT (defaults 4317 / 4318), each attempt bounded at one second. One JSON verdict (ready, not-ready, ended, cannot-evaluate) with the pid, the exit record when the app ended and one label per receiver (accepting, refusing). Exit 0 ready, 1 not-ready or ended, 2 cannot-evaluate"
    )]
    HarnessReady,
    #[command(
        name = "harness:settled",
        about = "Wait, bounded, until the app's log family holds an app.boot.window.navigation record for each of its four windows with the app alive (settled), or the app's pid is gone (ended), or the timeout passes (not-settled). One JSON verdict with the pid, the exit record, whether the log holds an app.exit record, the count of settled windows, whether the display and the session bus are reachable (labels only, never a variable's value), and what the exit witness recorded of the app's end (one label: unset, unreadable, loaded, exit-call, runtime-exit, no-record); the same object is written to logs/harness-settled.json under the data dir. Exit 0 settled, 1 ended or not-settled, 2 cannot-evaluate (no pid, or a timeout below 8 s, which could never read settled)"
    )]
    HarnessSettled {
        #[arg(long, value_name = "SECONDS", default_value_t = 30)]
        timeout_seconds: u64,
    },
    #[command(
        name = "harness:boot-series",
        about = "Boot the release app N more times after the CI boot smoke (ordinals 2 to N+1), each boot on its own data dir series/boot-{ordinal}/ under the resolved data dir, its own display server (xvfb-run) and empty XDG_DATA_HOME and XDG_CACHE_HOME, through the smoke's cycle: boot, harness:settled, status, cleanup. Each boot's log family, boot.log, harness-settled.json, xvfb.log and exit-witness.jsonl are copied to logs/series/boot-{ordinal}/; nothing of a boot's run/ is. One JSON verdict (all-settled, self-ended, not-all-settled, cannot-evaluate) with a per-boot entry of ordinal, cycle, settle verdict, exit record and exit-witness label, also written to logs/boot-series.json. Exit 0 all-settled, 1 self-ended (a boot's settle verdict read ended) or not-all-settled, 2 cannot-evaluate (a count outside 1 to 16, not Linux, no data dir or one that already holds a series, no xvfb-run, or a boot whose cleanup did not read clean)"
    )]
    HarnessBootSeries {
        #[arg(long, value_name = "N")]
        count: u32,
    },
    #[command(
        name = "test",
        about = "cargo nextest run --workspace --profile ci --no-tests=pass"
    )]
    Test {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "test:coverage",
        about = "cargo llvm-cov nextest --workspace --lcov --no-tests=pass (writes lcov.info)"
    )]
    TestCoverage {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "check:ingest-progress",
        about = "fail a run whose buffer consumer stopped draining (progress, not liveness); NEUTRAL on an absent log stream"
    )]
    CheckIngestProgress,
    #[command(
        name = "smoke:gap-resume",
        about = "Drive the ingest wedge's real shape — seed, GAP until every service crosses its bootstrap window and the silence family escalates to tier1, then resume — and read the verdict from the shipped drain-progress detector. A pass with no storm in the log is INCONCLUSIVE, not a pass. --sustained runs the control arm (continuous feed, no gap: the shape the predecessor's leg ran, where no storm can form). --reconnect-only runs the separator arm (seed then reconnect with NO idle gap), which isolates the new gRPC connection from the idle window the default arm varies alongside it"
    )]
    SmokeGapResume {
        #[arg(long, conflicts_with = "reconnect_only")]
        sustained: bool,
        #[arg(long)]
        reconnect_only: bool,
        #[arg(long, value_name = "SECONDS")]
        bootstrap_seconds: Option<u64>,
        #[arg(long, value_name = "SECONDS")]
        gap_seconds: Option<u64>,
        #[arg(long, value_name = "MINUTES")]
        observe_minutes: Option<u64>,
    },
    #[command(
        name = "smoke:external-resolve",
        about = "Drive an incident to Active, resolve it through a REAL andromeda-pulse-mcp subprocess (the cross-process writer an agent uses), then observe the app's own persist cycles. GREEN: reconciled_count goes positive and item_count drops within two cycles, without a restart. RED (pre-fix): item_count static while declined_count climbs. The verdict rests on field VALUES, never a clean log — a declined write is not an ERROR. Reports INCONCLUSIVE, never PASS, when no incident formed or the sidecar resolve did not apply"
    )]
    SmokeExternalResolve {
        #[arg(long, value_name = "SECONDS")]
        bootstrap_seconds: Option<u64>,
        #[arg(long, value_name = "SECONDS")]
        observe_seconds: Option<u64>,
    },
    #[command(
        name = "smoke:hue-shift",
        about = "Drive a tier rise (finite storm) and fall (120 s auto-resolve under a healthy feed) through the release app and grade the P-025 hue-shift samples by ANCHOR: each sample's timestamp minus duration_ms must land within 1000 ms of the incident creation record (rise) and the resolving auto-resolve tick (fall). The 2000 ms budget line is context only. Exit 0 PASS, 1 FAIL, 2 INCONCLUSIVE (no incident, or a sample never appeared)"
    )]
    SmokeHueShift,
    #[command(
        name = "smoke:discovery",
        about = "Boot the release app on a fresh data dir, wait until the webview polls services.list_with_states, start a healthy feed over real OTLP, and grade the P-027 discovery bound: the first metric.constellation.discovery_ms record must land within 5000 ms of the first duckdb.append {table_name: spans} record (the first sighting), with its own anchor (timestamp minus duration_ms) within 1000 ms of that record, and the log must hold 0 app.panic.fatal and 0 ERROR. Exit 0 PASS, 1 FAIL, 2 INCONCLUSIVE (no spans appended, or the webview was not polling first)"
    )]
    SmokeDiscovery,
    #[command(name = "audit", about = "cargo audit (RustSec advisory DB)")]
    Audit,
    #[command(name = "deny-bans", about = "cargo deny check bans licenses sources")]
    DenyBans,
    #[command(
        name = "check:npm-supply-chain",
        about = "npm advisory + license + ban gate over pulse-app/ui (policy: pulse-app/ui/npm-policy.json; license/class source: package-lock.json)"
    )]
    CheckNpmSupplyChain,
    #[command(
        name = "check:english-sources",
        about = "Scan crates, pulse-app/src, pulse-app/tests, pulse-app/ui/src and xtask/src (.rs/.ts/.tsx) for Cyrillic characters (U+0400..U+04FF). One GitHub ::error annotation per hit naming file:line, rendered ASCII-only, then one JSON verdict; twin at target/english-sources/report.json. Exit 0 clean, 1 findings, 2 cannot-evaluate (a root absent or unreadable)"
    )]
    CheckEnglishSources,
    #[command(
        name = "ci-gates",
        about = "obs SLO gates: zero-spans + zero-panic + heartbeat-gap + perf-budget (NEUTRAL over a log carrying no perf samples, never PASS)"
    )]
    CiGates,
    #[command(
        name = "perf:budget",
        about = "Grade <DIR>/logs/agent-latest.jsonl* against the obs-plan §10 perf budgets (frame p99 <= 33 ms, metric.buffer.memory_bytes max <= 512000000, snapshot p99 <= 500 ms). A --require arm with no readable sample fails; a non-numeric graded field always fails. Exit 0 PASS, 1 FAIL, 2 cannot-evaluate (no log family, an unknown arm, or nothing to grade)"
    )]
    PerfBudget {
        #[arg(long, value_name = "DIR")]
        data_dir: PathBuf,
        #[arg(long, value_name = "ARM,ARM", value_delimiter = ',')]
        require: Vec<String>,
    },
    #[command(
        name = "perf:frame-sample",
        about = "Windows only: boot target/release/pulse-app.exe on a fresh data dir with a software WebGPU adapter exposed to WebView2 (child env only), drive target/release/examples/inject_demo.exe --sustained for 30 s, then grade the frame arm as required. Exit 0 PASS, 1 FAIL (0 frame samples once healthy, or p99 over 33 ms), 2 INCONCLUSIVE (not Windows, a binary missing, :4317/:4318 in use, or the app never healthy). Artifact target/perf-frame/"
    )]
    PerfFrameSample,
    #[command(
        name = "lint",
        about = "npm run lint (ESLint flat config in pulse-app/ui/)"
    )]
    Lint {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "typecheck",
        about = "npm run typecheck (tsc --noEmit in pulse-app/ui/) — gates webview against TauRPC-generated bindings"
    )]
    Typecheck {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "test:a11y",
        about = "npm run test:a11y --prefix pulse-app/ui — axe-core + Lighthouse + pa11y + colorjs.io + Playwright + aggregator + regression detector orchestrator chain"
    )]
    TestA11y {
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    #[command(
        name = "capability-drift",
        about = "diff TauRPC procedures (from pulse-app/ui/src/bindings/index.ts) vs arch §Occupied Resources expected procedure list"
    )]
    CapabilityDrift,
    #[command(
        name = "capability-widening-check",
        about = "Chunk #77 — static-analyze pulse-app/capabilities/*.json against NEVER-widen bans per security plan §API Anti-Patterns rows 6-7 (pulse:notification / pulse:tray / pulse:plugin-fs MUST stay outbound-emit-only / read-only)"
    )]
    CapabilityWideningCheck,
    #[command(
        name = "check:staged-artifacts",
        about = "diff the STAGED (git index) copies of pulse-app/ui/src/bindings/index.ts + pulse-app/capabilities/*.json against EXPECTED_PROCEDURES + EXPECTED_GRANTS — the committed copy is the subject, never the worktree (exit 0 staged-clean / 1 staged-drift / 2 cannot-evaluate); also runs inside capability-drift"
    )]
    CheckStagedArtifacts,
    #[command(
        name = "smoke",
        about = "install-launch-ingest-query smoke per bundle format (chunk #51)"
    )]
    Smoke {
        #[arg(long, value_name = "PATH")]
        bundle: PathBuf,
        #[arg(long, value_enum)]
        format: BundleFormat,
    },
    #[command(
        name = "self-verify",
        about = "Agent-headful self-verify (P-078): boot the real pulse-app binary, assert shell health from agent-latest.jsonl (boot spans + window shown + heartbeat + zero panics) + run the a11y/contrast harness + clean quit with zero orphan; skip-clean on a display-less host"
    )]
    SelfVerify,
    #[command(
        name = "webview-drive",
        about = "Headful webview drive: drive the assembled product path (launch → traces → storm → incident → Investigate) in the live Tauri window and assert each stage against the app's own obs record. Closes the gap self-verify cannot (it never clicks). --expect-absent <stage> inverts one stage's assertion for the mutation check; --no-inject suppresses telemetry so the telemetry-dependent stages must go red"
    )]
    WebviewDrive {
        #[arg(long, value_name = "STAGE")]
        expect_absent: Option<String>,
        #[arg(long)]
        no_inject: bool,
    },
    #[command(
        name = "perf:slo-load",
        about = "10k spans/sec sustained-load test + post-test metric.webgpu.frame_duration_ms p99 ≤33ms + metric.buffer.memory_bytes max ≤512MB gate"
    )]
    PerfSloLoad,
    #[command(
        name = "coverage-regression",
        about = "Chunk #55 — compare current lcov.info against base-branch baseline; fail on any line/branch/function regression > +0.0pp default (test-plan §10 + §11)"
    )]
    CoverageRegression {
        #[arg(long, value_name = "PATH")]
        current: PathBuf,
        #[arg(long, value_name = "PATH")]
        baseline: PathBuf,
    },
    #[command(
        name = "quarantine-tracking",
        about = "Chunk #55 — assert every #[ignore] in Rust source carries a GitHub issue URL in surrounding 5-line window (test-plan §11)"
    )]
    QuarantineTracking,
    #[command(
        name = "criterion-regression",
        about = "Chunk #56 — compare current target/criterion/<bench>/new/estimates.json mean.point_estimate against baseline; fail on +10% regression default (obs-plan §10 row 4)"
    )]
    CriterionRegression {
        #[arg(long, value_name = "PATH")]
        current: PathBuf,
        #[arg(long, value_name = "PATH")]
        baseline: PathBuf,
    },
    #[command(
        name = "verify:capability-matrix",
        about = "Chunk #99 — validate docs/v0_2_0/capability-verification-matrix.json: all 60 P-001..P-060 ids present exactly once, every scenario file ref exists, every `contains` anchor greps non-empty, by-construction entries carry justification notes"
    )]
    VerifyCapabilityMatrix,
    #[command(
        name = "perf:load-profiles",
        about = "Chunk #99 — dist-arch v3 four-profile load suite (baseline 1k / high 10k / burst 50k / sustained-extreme 50k spans/s) via nextest --profile load-profiles, then heartbeat-gap + perf-slo gates over harness logs when present"
    )]
    PerfLoadProfiles,
    #[command(
        name = "pre-push:linux",
        about = "Run the Linux-reachable CI gates (script modes, the English-only source lint, npm build, clippy, xtask test, ci-gates) on the Linux dev host, in the working tree, before a push: six stages in order, first failure stops, each under a constructed environment. One JSON verdict; exit 0 green / 1 red / 2 cannot-evaluate (not Linux, or a pin read from the repo is unmet: the Rust channel, ci.yml's Node major, a missing tool). Installs nothing, puts the generated bindings back as found, never binds a port"
    )]
    PrePushLinux,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let result: Result<ExitCode> = match cli.command {
        Cmd::HarnessStatus => harness_status::run(),
        Cmd::HarnessReady => harness_ready::run_ready(),
        Cmd::HarnessSettled { timeout_seconds } => harness_ready::run_settled(timeout_seconds),
        Cmd::HarnessBootSeries { count } => harness_series::run(count).await,
        Cmd::Test { extra } => run_cargo_nextest(extra).await,
        Cmd::TestCoverage { extra } => run_cargo_llvm_cov(extra).await,
        Cmd::CheckIngestProgress => run_check_ingest_progress(),
        Cmd::SmokeGapResume {
            sustained,
            reconnect_only,
            bootstrap_seconds,
            gap_seconds,
            observe_minutes,
        } => {
            let defaults = gap_resume::GapResumeOptions::default();
            let arm = match (sustained, reconnect_only) {
                (true, _) => gap_resume::LegArm::Sustained,
                (_, true) => gap_resume::LegArm::ReconnectOnly,
                _ => gap_resume::LegArm::GapResume,
            };
            gap_resume::run_gap_resume(gap_resume::GapResumeOptions {
                arm,
                bootstrap_seconds: bootstrap_seconds.unwrap_or(defaults.bootstrap_seconds),
                gap_seconds: gap_seconds.unwrap_or(defaults.gap_seconds),
                observe_minutes: observe_minutes.unwrap_or(defaults.observe_minutes),
            })
            .await
        }
        Cmd::SmokeExternalResolve {
            bootstrap_seconds,
            observe_seconds,
        } => {
            let defaults = external_resolve::ExternalResolveOptions::default();
            external_resolve::run_external_resolve(external_resolve::ExternalResolveOptions {
                bootstrap_seconds: bootstrap_seconds.unwrap_or(defaults.bootstrap_seconds),
                observe_seconds: observe_seconds.unwrap_or(defaults.observe_seconds),
                incident_wait_seconds: defaults.incident_wait_seconds,
            })
            .await
        }
        Cmd::SmokeHueShift => hue_shift::run_hue_shift().await,
        Cmd::SmokeDiscovery => discovery::run_discovery().await,
        Cmd::Audit => run_cargo("audit", &[]).await,
        Cmd::DenyBans => run_cargo("deny", &["check", "bans", "licenses", "sources"]).await,
        Cmd::CheckNpmSupplyChain => npm_gate::run_npm_gate().await,
        Cmd::CheckEnglishSources => source_lint::run(),
        Cmd::CiGates => run_ci_gates().await,
        Cmd::PerfBudget { data_dir, require } => perf_budget::run_perf_budget(&data_dir, &require),
        Cmd::PerfFrameSample => perf_frame::run_perf_frame_sample().await,
        Cmd::Lint { extra } => run_npm_script("lint", extra).await,
        Cmd::Typecheck { extra } => run_npm_script("typecheck", extra).await,
        Cmd::TestA11y { extra } => run_npm_script("test:a11y", extra).await,
        Cmd::CapabilityDrift => capability_drift().await,
        Cmd::CapabilityWideningCheck => capability_widening_check().await,
        Cmd::CheckStagedArtifacts => staged_gate::run().await,
        Cmd::Smoke { bundle, format } => smoke::run_smoke(&bundle, format).await,
        Cmd::SelfVerify => self_verify::run_self_verify().await,
        Cmd::WebviewDrive {
            expect_absent,
            no_inject,
        } => webview_drive::run_webview_drive(expect_absent, no_inject).await,
        Cmd::PerfSloLoad => run_perf_slo_load().await,
        Cmd::CoverageRegression { current, baseline } => {
            run_coverage_regression(&current, &baseline).await
        }
        Cmd::QuarantineTracking => run_quarantine_tracking().await,
        Cmd::CriterionRegression { current, baseline } => {
            run_criterion_regression(&current, &baseline).await
        }
        Cmd::VerifyCapabilityMatrix => verify_capability_matrix().await,
        Cmd::PerfLoadProfiles => run_perf_load_profiles().await,
        Cmd::PrePushLinux => pre_push::run(),
    };
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("xtask error: {e:?}");
            ExitCode::FAILURE
        }
    }
}

/// Variables `cargo run` sets on the program it launches, describing xtask's own
/// package. They leak into every child cargo this process spawns.
fn is_cargo_run_injected(name: &str) -> bool {
    name.starts_with("CARGO_PKG_")
        || matches!(
            name,
            "CARGO_MANIFEST_DIR"
                | "CARGO_MANIFEST_PATH"
                | "CARGO_MANIFEST_LINKS"
                | "CARGO_CRATE_NAME"
                | "CARGO_BIN_NAME"
                | "CARGO_PRIMARY_PACKAGE"
        )
}

/// A child `cargo` without the `cargo run`-injected package variables. Build
/// scripts track some of them (ring's reruns on `CARGO_MANIFEST_DIR` and
/// `CARGO_PKG_*`), so a child build seeing xtask's values while a shell build
/// sees none reruns ring on every alternation — and ring's rebuild recompiles
/// rustls, libduckdb-sys and the whole workspace above it.
pub(crate) fn cargo_command() -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("cargo");
    for (key, _) in env::vars_os() {
        if key.to_str().is_some_and(is_cargo_run_injected) {
            cmd.env_remove(&key);
        }
    }
    cmd
}

#[cfg(test)]
mod cargo_command_tests {
    use super::*;

    #[test]
    fn cargo_run_package_variables_are_recognized() {
        for name in [
            "CARGO_PKG_NAME",
            "CARGO_PKG_VERSION_MAJOR",
            "CARGO_MANIFEST_DIR",
            "CARGO_MANIFEST_PATH",
            "CARGO_BIN_NAME",
        ] {
            assert!(is_cargo_run_injected(name), "{name}");
        }
        for name in [
            "CARGO",
            "CARGO_HOME",
            "CARGO_TARGET_DIR",
            "CARGO_INCREMENTAL",
            "CARGO_TERM_COLOR",
            "CC",
        ] {
            assert!(!is_cargo_run_injected(name), "{name}");
        }
    }

    // The test runner itself sets CARGO_PKG_NAME and CARGO_MANIFEST_DIR on this
    // process, exactly as `cargo run` does for xtask.
    #[test]
    fn cargo_command_drops_the_inherited_package_variables() {
        assert!(env::var_os("CARGO_PKG_NAME").is_some(), "precondition");
        let cmd = cargo_command();
        let removed: Vec<String> = cmd
            .as_std()
            .get_envs()
            .filter(|(_, value)| value.is_none())
            .map(|(key, _)| key.to_string_lossy().into_owned())
            .collect();
        assert!(
            removed.contains(&"CARGO_PKG_NAME".to_owned()),
            "{removed:?}"
        );
        assert!(
            removed.contains(&"CARGO_MANIFEST_DIR".to_owned()),
            "{removed:?}"
        );
        assert!(!removed.iter().any(|k| k == "CARGO_HOME" || k == "PATH"));
    }
}

async fn run_cargo(subcommand: &str, args: &[&str]) -> Result<ExitCode> {
    let mut cmd = cargo_command();
    cmd.arg(subcommand).args(args);
    let status = cmd
        .status()
        .await
        .with_context(|| format!("failed to spawn `cargo {subcommand}`"))?;
    Ok(status_to_code(status))
}

async fn run_cargo_nextest(extra: Vec<String>) -> Result<ExitCode> {
    let mut cmd = cargo_command();
    // libtest-json is gated behind an experimental flag in cargo-nextest 0.9.x;
    // set the env var unconditionally so the message-format parses agent-side.
    cmd.env("NEXTEST_EXPERIMENTAL_LIBTEST_JSON", "1");
    cmd.args([
        "nextest",
        "run",
        "--workspace",
        "--profile",
        "ci",
        "--no-tests=pass",
        "--message-format",
        "libtest-json",
    ]);
    for arg in extra {
        cmd.arg(arg);
    }
    let status = cmd
        .status()
        .await
        .context("failed to spawn `cargo nextest`")?;
    Ok(status_to_code(status))
}

/// TEMPORARY scope of the coverage measure (founder ruling 2026-09-29): the
/// xtask dev task-runner is excluded, thresholds unchanged. Owned by the next
/// epoch-boundary code audit, which revisits coverage quality, thresholds
/// above 85 % and xtask's inclusion (test-plan §10).
const COVERAGE_IGNORE_FILENAME_REGEX: &str = r"(^|[/\\])xtask[/\\]";

async fn run_cargo_llvm_cov(extra: Vec<String>) -> Result<ExitCode> {
    let mut cmd = cargo_command();
    cmd.args([
        "llvm-cov",
        "nextest",
        "--workspace",
        "--lcov",
        "--output-path",
        "lcov.info",
        "--no-tests=pass",
        "--ignore-filename-regex",
        COVERAGE_IGNORE_FILENAME_REGEX,
    ]);
    for arg in extra {
        cmd.arg(arg);
    }
    let status = cmd
        .status()
        .await
        .context("failed to spawn `cargo llvm-cov`")?;
    Ok(status_to_code(status))
}

async fn run_ci_gates() -> Result<ExitCode> {
    let log_files = collect_log_files();
    if log_files.is_empty() {
        // INACTIVE state: no harness boot in this CI run; gate trivially passes.
        // Activates organically when integration tests boot pulse-app (chunks #15+).
        println!(
            "ci-gates: zero-spans NEUTRAL (no `agent-latest.jsonl*` under {} — pre-integration-test state)",
            resolve_log_dir().display()
        );
        println!("ci-gates: zero-panic NEUTRAL (no log file to scan)");
        println!("ci-gates: heartbeat-gap NEUTRAL (no log file to scan)");
        println!("ci-gates: perf-budget NEUTRAL (no log file to grade)");
        return Ok(ExitCode::SUCCESS);
    }

    let mut total_lines = 0usize;
    let mut panic_violation: Option<String> = None;
    for path in &log_files {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("ci-gates: skipping {} ({e})", path.display());
                continue;
            }
        };
        for (idx, raw) in content.lines().enumerate() {
            if raw.trim().is_empty() {
                continue;
            }
            total_lines += 1;
            let v: Value = match serde_json::from_str(raw) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let target = v.get("target").and_then(Value::as_str).unwrap_or("");
            let level = v.get("level").and_then(Value::as_str).unwrap_or("");
            if target == "app.panic.fatal" && level == "ERROR" && panic_violation.is_none() {
                let preview: String = raw.chars().take(240).collect();
                panic_violation = Some(format!("{}:{} — {}", path.display(), idx + 1, preview));
            }
        }
    }

    if total_lines == 0 {
        eprintln!("::error::ci-gates: zero-spans FAIL (log files present but contain no events)");
        return Ok(ExitCode::FAILURE);
    }
    println!(
        "ci-gates: zero-spans PASS ({total_lines} log records across {} file(s))",
        log_files.len()
    );

    if let Some(violation) = panic_violation {
        eprintln!("::error::ci-gates: zero-panic FAIL — app.panic.fatal at {violation}");
        return Ok(ExitCode::FAILURE);
    }
    println!("ci-gates: zero-panic PASS");

    // Heartbeat-gap gate: shell out to xtask/ci/heartbeat-gap-check.{sh,ps1}
    match invoke_heartbeat_check(&log_files).await {
        Ok(true) => println!("ci-gates: heartbeat-gap PASS"),
        Ok(false) => {
            eprintln!("::error::ci-gates: heartbeat-gap FAIL");
            return Ok(ExitCode::FAILURE);
        }
        Err(e) => {
            eprintln!(
                "ci-gates: heartbeat-gap script unavailable ({e:#}) — treating as NEUTRAL at chunk #5/#6"
            );
        }
    }

    // No arm is required here: the boot-smoke log and pre-push:linux's seeded
    // record legitimately carry no perf samples. The gates that expect samples
    // run `perf:budget --require` over their own producer's log.
    let perf_results = perf_budget::grade(&perf_budget::read_family(&resolve_log_dir())?);
    for line in perf_budget::arm_lines(&perf_results, &[]) {
        println!("ci-gates: {line}");
    }
    match perf_budget::evaluate(&perf_results, &[]) {
        perf_budget::Verdict::Fail => {
            eprintln!("::error::ci-gates: perf-budget FAIL");
            return Ok(ExitCode::FAILURE);
        }
        verdict => println!("ci-gates: perf-budget {}", verdict.word()),
    }

    Ok(ExitCode::SUCCESS)
}

// Progress gate. The existing obs gates key on tick PRESENCE (obs-plan §3/§10
// define a stall as tick absence >45s), which the measured 2026-08-26 wedge
// satisfied throughout — heartbeats ticked while nothing drained. This one
// keys on the app's own drain-progress record instead.
fn run_check_ingest_progress() -> Result<ExitCode> {
    let log_files = collect_log_files();
    let lines = ingest_progress::parse_lines(&log_files);

    match ingest_progress::evaluate(&lines) {
        ingest_progress::Verdict::Neutral(reason) => {
            println!("check:ingest-progress: NEUTRAL — {reason}");
            Ok(ExitCode::SUCCESS)
        }
        ingest_progress::Verdict::Pass {
            ticks,
            longest_zero_delta_run,
        } => {
            println!(
                "check:ingest-progress: PASS — {ticks} buffer ticks, longest zero-delta run {longest_zero_delta_run}"
            );
            Ok(ExitCode::SUCCESS)
        }
        ingest_progress::Verdict::Fail {
            reason,
            consequence,
            stalled_seconds,
        } => {
            eprintln!(
                "::error::check:ingest-progress: FAIL — buffer consumer stalled for {stalled_seconds}s (reason={reason}, consequence={consequence})"
            );
            Ok(ExitCode::FAILURE)
        }
    }
}

fn collect_log_files() -> Vec<PathBuf> {
    let dir = resolve_log_dir();
    let entries = match fs::read_dir(&dir) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|res| res.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.starts_with("agent-latest.jsonl"))
                .unwrap_or(false)
        })
        .collect();
    files.sort();
    files
}

// Chunk #99 — scope the obs gate scripts to the CURRENT run's time window.
// The persistent dev data dir accumulates daily-rolled logs across booted
// sessions (verification-harness.md 2026-05-14 dev-residue trap), so
// cross-session tick gaps in `agent-latest.jsonl*` are not stalls. Keep only
// lines whose RFC3339 UTC timestamp prefix is >= the run-start prefix and
// hand the scripts the windowed artifact instead of the raw archive.
// Lexicographic compare on the 19-char `YYYY-MM-DDTHH:MM:SS` prefix is
// order-correct because both sides are UTC.
fn write_run_window_log(
    log_files: &[PathBuf],
    run_start_utc19: &str,
    out_path: &Path,
) -> Result<usize> {
    let mut kept: Vec<String> = Vec::new();
    for file in log_files {
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for line in content.lines() {
            let Some(idx) = line.find("\"timestamp\":\"") else {
                continue;
            };
            let ts_start = idx + "\"timestamp\":\"".len();
            let Some(ts19) = line.get(ts_start..ts_start + 19) else {
                continue;
            };
            if ts19 >= run_start_utc19 {
                kept.push(line.to_string());
            }
        }
    }
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create run-window log dir {}", parent.display()))?;
    }
    fs::write(out_path, kept.join("\n"))
        .with_context(|| format!("write run-window log {}", out_path.display()))?;
    Ok(kept.len())
}

fn resolve_log_dir() -> PathBuf {
    if let Ok(p) = env::var("ANDROMEDA_PULSE_DATA_DIR") {
        return PathBuf::from(p).join("logs");
    }
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = env::var("APPDATA") {
            return PathBuf::from(appdata).join("andromeda-pulse").join("logs");
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("com.andromeda.pulse")
                .join("logs");
        }
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("andromeda-pulse").join("logs");
    } else if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".andromeda-pulse").join("logs");
    }
    env::temp_dir().join("andromeda-pulse").join("logs")
}

async fn invoke_heartbeat_check(log_files: &[PathBuf]) -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("heartbeat-gap-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("heartbeat-gap-check.sh")
    };
    if !script.exists() {
        bail!("heartbeat-gap script missing at {}", script.display());
    }
    let primary_log = log_files
        .last()
        .context("no log file to pass to heartbeat-gap script")?;
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(primary_log)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(primary_log)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_perf_slo_load() -> Result<ExitCode> {
    // 10k spans/sec sustained-load test runs the perf_slo_10k_spans
    // integration test via cargo-nextest; post-test p99 / max gates fire
    // via run_ci_gates() (invoked as a separate xtask step in CI).
    // Narrowed with -E under --workspace, never -p: a -p selection unifies
    // features differently and recompiles the graph `cargo xtask test` built.
    let mut cmd = cargo_command();
    cmd.env("NEXTEST_EXPERIMENTAL_LIBTEST_JSON", "1");
    cmd.args([
        "nextest",
        "run",
        "--workspace",
        "-E",
        "binary(perf_slo_10k_spans)",
        "--profile",
        "ci",
        "--no-tests=pass",
        "--message-format",
        "libtest-json",
    ]);
    let status = cmd
        .status()
        .await
        .context("failed to spawn `cargo nextest run -E binary(perf_slo_10k_spans)`")?;
    Ok(status_to_code(status))
}

// Chunk #99 — dist-arch v3 §Load testing four-profile release-gate suite.
// The tests are excluded from default/ci nextest profiles via
// `.config/nextest.toml` default-filter; `--profile load-profiles`
// re-selects exactly the perf_load_profiles binary. After the suite, the
// heartbeat-gap + perf-slo gates run over harness logs when present (the
// booted-app ACTIVE window flow); absent logs map to NEUTRAL — the
// in-process suite does not write agent-latest.jsonl itself.
async fn run_perf_load_profiles() -> Result<ExitCode> {
    let run_start_utc19 = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let mut cmd = cargo_command();
    cmd.env("NEXTEST_EXPERIMENTAL_LIBTEST_JSON", "1");
    cmd.args([
        "nextest",
        "run",
        "-p",
        "pulse-app",
        "--test",
        "perf_load_profiles",
        "--profile",
        "load-profiles",
        "--no-tests=pass",
        "--message-format",
        "libtest-json",
    ]);
    let status = cmd
        .status()
        .await
        .context("failed to spawn `cargo nextest run --test perf_load_profiles`")?;
    if !status.success() {
        return Ok(status_to_code(status));
    }

    let log_files = collect_log_files();
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let window_path = workspace_root
        .join("target")
        .join("load-profiles")
        .join("agent-window.jsonl");
    let in_window = if log_files.is_empty() {
        0
    } else {
        write_run_window_log(&log_files, &run_start_utc19, &window_path)?
    };
    if in_window == 0 {
        println!(
            "perf:load-profiles: heartbeat-gap + perf-slo gates NEUTRAL (no `agent-latest.jsonl*` telemetry within this run's window under {} — the load suite uses ephemeral rigs; for the ACTIVE window boot the app via scripts/agent-run boot + drive crates/ingest/examples/load_profiles.rs, then re-run this command)",
            resolve_log_dir().display()
        );
        return Ok(ExitCode::SUCCESS);
    }
    println!(
        "perf:load-profiles: {in_window} in-window log line(s) — running obs gates over {}",
        window_path.display()
    );
    let windowed_files = vec![window_path.clone()];
    match invoke_heartbeat_check(&windowed_files).await {
        Ok(true) => println!("perf:load-profiles: heartbeat-gap PASS"),
        Ok(false) => {
            eprintln!("::error::perf:load-profiles: heartbeat-gap FAIL");
            return Ok(ExitCode::FAILURE);
        }
        Err(e) => {
            eprintln!("perf:load-profiles: heartbeat-gap script unavailable ({e:#}) — NEUTRAL")
        }
    }
    let window_lines = perf_budget::parse_lines(&smoke::read_jsonl_lines(&window_path)?);
    let perf_results = perf_budget::grade(&window_lines);
    for line in perf_budget::arm_lines(&perf_results, &[]) {
        println!("perf:load-profiles: {line}");
    }
    match perf_budget::evaluate(&perf_results, &[]) {
        perf_budget::Verdict::Fail => {
            eprintln!("::error::perf:load-profiles: perf-budget FAIL");
            return Ok(ExitCode::FAILURE);
        }
        verdict => println!("perf:load-profiles: perf-budget {}", verdict.word()),
    }
    Ok(ExitCode::SUCCESS)
}

// Chunk #99 — capability verification matrix validator (the v0.2.0 tag
// gate's "every P-XXX has at least one named scenario" enforcement).
// Validates docs/v0_2_0/capability-verification-matrix.json structurally:
// exactly P-001..P-060 present once each, every file-kind scenario ref
// exists on disk, every `contains` anchor greps non-empty in its ref,
// xtask-gate refs name known subcommands, and by-construction entries
// carry a justification note. Mirrors capability_drift's report shape at
// target/capability-matrix/report.json + obs §3 structured event line.
async fn verify_capability_matrix() -> Result<ExitCode> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let matrix_path = workspace_root
        .join("docs")
        .join("v0_2_0")
        .join("capability-verification-matrix.json");
    let content = fs::read_to_string(&matrix_path)
        .with_context(|| format!("read capability matrix at {}", matrix_path.display()))?;
    let doc: Value = serde_json::from_str(&content).context("parse capability matrix JSON")?;

    const FILE_KINDS: &[&str] = &[
        "nextest-file",
        "ui-test",
        "a11y-spec",
        "ci-script",
        "source-evidence",
    ];
    const ALL_KINDS: &[&str] = &[
        "nextest-file",
        "ui-test",
        "a11y-spec",
        "ci-script",
        "source-evidence",
        "xtask-gate",
        "by-construction",
    ];
    const MODES: &[&str] = &[
        "automated-nextest",
        "automated-a11y",
        "automated-e2e",
        "xtask-gate",
        "env-gated-runtime",
        "manual-sr-supplemental",
        "by-construction",
    ];
    const XTASK_GATES: &[&str] = &[
        "capability-drift",
        "capability-widening-check",
        "test:a11y",
        "perf:slo-load",
        "perf:load-profiles",
        "verify:capability-matrix",
        "ci-gates",
        "quarantine-tracking",
    ];

    let mut violations: Vec<String> = Vec::new();
    let mut mode_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen_ids: BTreeSet<String> = BTreeSet::new();

    let capabilities = doc
        .get("capabilities")
        .and_then(Value::as_array)
        .context("matrix JSON missing `capabilities` array")?;

    for entry in capabilities {
        let id = entry.get("id").and_then(Value::as_str).unwrap_or("");
        if id.is_empty() {
            violations.push("entry with missing/empty `id`".to_string());
            continue;
        }
        if !seen_ids.insert(id.to_string()) {
            violations.push(format!("{id}: duplicate id"));
        }
        let mode = entry
            .get("verification_mode")
            .and_then(Value::as_str)
            .unwrap_or("");
        if !MODES.contains(&mode) {
            violations.push(format!("{id}: unknown verification_mode `{mode}`"));
        }
        *mode_counts.entry(mode.to_string()).or_insert(0) += 1;
        let notes = entry.get("notes").and_then(Value::as_str).unwrap_or("");

        let scenarios = entry
            .get("scenarios")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if scenarios.is_empty() {
            violations.push(format!("{id}: zero scenarios (every capability needs ≥1)"));
            continue;
        }
        let mut has_verifying_scenario = false;
        for scenario in &scenarios {
            let kind = scenario.get("kind").and_then(Value::as_str).unwrap_or("");
            let reference = scenario.get("ref").and_then(Value::as_str).unwrap_or("");
            if !ALL_KINDS.contains(&kind) {
                violations.push(format!("{id}: unknown scenario kind `{kind}`"));
                continue;
            }
            match kind {
                "by-construction" => {
                    if notes.trim().is_empty() {
                        violations.push(format!(
                            "{id}: by-construction scenario requires a justification in `notes`"
                        ));
                    }
                    has_verifying_scenario = true;
                }
                "xtask-gate" => {
                    if !XTASK_GATES.contains(&reference) {
                        violations.push(format!(
                            "{id}: xtask-gate ref `{reference}` is not a known subcommand"
                        ));
                    }
                    has_verifying_scenario = true;
                }
                kind if FILE_KINDS.contains(&kind) => {
                    let path = workspace_root.join(reference);
                    if !path.is_file() {
                        violations.push(format!(
                            "{id}: scenario ref `{reference}` does not exist on disk"
                        ));
                        continue;
                    }
                    if let Some(anchor) = scenario.get("contains").and_then(Value::as_str) {
                        let file_content = fs::read_to_string(&path)
                            .with_context(|| format!("read scenario ref `{reference}` for {id}"))?;
                        if !file_content.contains(anchor) {
                            violations.push(format!(
                                "{id}: anchor `{anchor}` not found in `{reference}`"
                            ));
                            continue;
                        }
                    }
                    if kind != "source-evidence" {
                        has_verifying_scenario = true;
                    }
                }
                _ => unreachable!("kind membership checked above"),
            }
        }
        if !has_verifying_scenario && notes.trim().is_empty() {
            violations.push(format!(
                "{id}: only source-evidence scenarios and no `notes` justification"
            ));
        }
    }

    let expected_ids: BTreeSet<String> = (1..=60).map(|n| format!("P-{n:03}")).collect();
    for missing in expected_ids.difference(&seen_ids) {
        violations.push(format!("{missing}: capability missing from matrix"));
    }
    for unexpected in seen_ids.difference(&expected_ids) {
        violations.push(format!("{unexpected}: id outside P-001..P-060 range"));
    }

    let state = if violations.is_empty() {
        "clean"
    } else {
        "violations"
    };
    let report_dir = workspace_root.join("target").join("capability-matrix");
    fs::create_dir_all(&report_dir).context("create capability-matrix report dir")?;
    let report_path = report_dir.join("report.json");
    let report = serde_json::json!({
        "state": state,
        "capability_count": seen_ids.len(),
        "violation_count": violations.len(),
        "violations": violations,
        "verification_mode_counts": mode_counts,
        "generated_at": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&report_path, serde_json::to_string_pretty(&report)?)
        .context("write capability-matrix report")?;

    let event = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "level": if state == "clean" { "INFO" } else { "WARN" },
        "target": "xtask.verify_capability_matrix",
        "message": "capability verification matrix check complete",
        "fields": {
            "state": state,
            "capability_count": seen_ids.len(),
            "violation_count": violations.len(),
        },
    });
    println!("{}", serde_json::to_string(&event)?);

    eprintln!(
        "verify:capability-matrix: {state} ({}/60 capabilities, {} violation(s))",
        seen_ids.len(),
        violations.len()
    );
    for v in &violations {
        eprintln!("  violation: {v}");
    }
    eprintln!("  report: {}", report_path.display());

    if state == "clean" {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

async fn invoke_coverage_regression_check(current: &Path, baseline: &Path) -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("coverage-regression-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("coverage-regression-check.sh")
    };
    if !script.exists() {
        bail!(
            "coverage-regression-check script missing at {}",
            script.display()
        );
    }
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_coverage_regression(current: &Path, baseline: &Path) -> Result<ExitCode> {
    if invoke_coverage_regression_check(current, baseline).await? {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

async fn invoke_quarantine_tracking_check() -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("quarantine-tracking-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("quarantine-tracking-check.sh")
    };
    if !script.exists() {
        bail!(
            "quarantine-tracking-check script missing at {}",
            script.display()
        );
    }
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(&workspace_root)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(&workspace_root)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_quarantine_tracking() -> Result<ExitCode> {
    if invoke_quarantine_tracking_check().await? {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

async fn invoke_criterion_regression_check(current: &Path, baseline: &Path) -> Result<bool> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let script = if cfg!(target_os = "windows") {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("criterion-regression-check.ps1")
    } else {
        workspace_root
            .join("xtask")
            .join("ci")
            .join("criterion-regression-check.sh")
    };
    if !script.exists() {
        bail!(
            "criterion-regression-check script missing at {}",
            script.display()
        );
    }
    let status = if cfg!(target_os = "windows") {
        tokio::process::Command::new("pwsh")
            .args(["-NoProfile", "-File"])
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    } else {
        tokio::process::Command::new("bash")
            .arg(&script)
            .arg(current)
            .arg(baseline)
            .status()
            .await?
    };
    Ok(status.success())
}

async fn run_criterion_regression(current: &Path, baseline: &Path) -> Result<ExitCode> {
    if invoke_criterion_regression_check(current, baseline).await? {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

async fn run_npm_script(script: &str, extra: Vec<String>) -> Result<ExitCode> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let cwd = workspace_root.join("pulse-app").join("ui");
    let npm = if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    };
    let mut cmd = tokio::process::Command::new(npm);
    cmd.current_dir(&cwd).arg("run").arg(script);
    if !extra.is_empty() {
        cmd.arg("--");
        for arg in extra {
            cmd.arg(arg);
        }
    }
    let status = cmd
        .status()
        .await
        .with_context(|| format!("failed to spawn `npm run {script}` in {}", cwd.display()))?;
    Ok(status_to_code(status))
}

// EXPECTED_PROCEDURES tracks the TauRPC procedure surface arch §Occupied
// Resources Tauri IPC routes legitimizes for the current commit. New chunks
// extend this list as their crates ship. Future-deferred procedures
// (other snapshot/workspace verbs, mcp.*) commented out until their owning
// chunks land — uncommenting prematurely produces "missing" drift noise
// that hides real drift. mcp.* additionally gated by --features mcp-server
// at the binary level.
const EXPECTED_PROCEDURES: &[&str] = &[
    "app_info",
    "health",
    "ready",
    "get_settings",
    "update_settings",
    "config.reload",
    "config.status",
    "connection.current_state",
    "diagnostics.history",
    "diagnostics.reevaluate_recent_window",
    "diagnostics.retry_interpretation",
    "diagnostics.snapshot",
    "diagnostics.template_distribution",
    "incidents.acknowledge",
    "incidents.get_report",
    "incidents.list_active",
    "incidents.mark_all_read",
    "incidents.mark_resolved",
    "investigate.run_action",
    "logs.query",
    "metrics.query",
    "services.list_with_states",
    "storage.export_for_training",
    "storage.inspect",
    "storage.path",
    "streams.subscribe_logs",
    "streams.subscribe_metrics",
    "streams.subscribe_spans",
    "snapshot.generate",
    "plugins.list",
    "plugins.reload",
    "plugins.invoke",
    "mcp.status",
    "mcp.start",
    "mcp.stop",
    "model.current_profile",
    "telemetry.frontend.record_constellation_discovery_latency",
    "telemetry.frontend.record_constellation_hue_latency",
    "telemetry.frontend.record_findings_counter_refresh",
    "telemetry.frontend.record_frame_ms",
    "telemetry.frontend.record_ipc_rejection",
    "telemetry.frontend.record_webgpu_adapter",
    "traces.query",
    "workspace.detect",
    // future-deferred (per epoch landing):
    // "snapshot.list_recent", "snapshot.copy_to_clipboard",
    // "workspace.list",
];

async fn capability_drift() -> Result<ExitCode> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let bindings_path = workspace_root
        .join("pulse-app")
        .join("ui")
        .join("src")
        .join("bindings")
        .join("index.ts");
    let bindings_content = fs::read_to_string(&bindings_path)
        .with_context(|| format!("read bindings file at {}", bindings_path.display()))?;

    let discovered = parse_bindings(&bindings_content)
        .context("parse ARGS_MAP from bindings.ts (taurpc emission shape may have changed)")?;
    let expected: BTreeSet<String> = EXPECTED_PROCEDURES.iter().map(|s| s.to_string()).collect();

    let missing: BTreeSet<String> = expected.difference(&discovered).cloned().collect();
    let extra: BTreeSet<String> = discovered.difference(&expected).cloned().collect();

    let drift_state = if missing.is_empty() && extra.is_empty() {
        "clean"
    } else {
        "drifted"
    };

    let report_dir = workspace_root.join("target").join("capability-drift");
    fs::create_dir_all(&report_dir).context("create capability-drift report dir")?;
    let report_path = report_dir.join("report.json");

    let top_5_drifted: Vec<&String> = missing.iter().chain(extra.iter()).take(5).collect();
    let report = serde_json::json!({
        "drift_state": drift_state,
        "missing_count": missing.len(),
        "extra_count": extra.len(),
        "missing": missing.iter().collect::<Vec<_>>(),
        "extra": extra.iter().collect::<Vec<_>>(),
        "discovered": discovered.iter().collect::<Vec<_>>(),
        "expected": expected.iter().collect::<Vec<_>>(),
        "generated_at": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&report_path, serde_json::to_string_pretty(&report)?)
        .context("write drift report")?;

    // Structured event line to stdout matching the obs §3 JSON schema. xtask
    // does not init a tracing subscriber (per implementation notes); the JSON
    // line is the agent-tailable surface.
    let event = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "level": if drift_state == "clean" { "INFO" } else { "WARN" },
        "target": "xtask.capability_drift",
        "message": "capability drift check complete",
        "fields": {
            "missing_count": missing.len(),
            "extra_count": extra.len(),
            "drift_state": drift_state,
            "top_5_drifted_names": top_5_drifted,
        },
    });
    println!("{}", serde_json::to_string(&event)?);

    eprintln!(
        "capability-drift: {drift_state} ({} missing, {} extra)",
        missing.len(),
        extra.len()
    );
    if !missing.is_empty() {
        let names: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
        eprintln!("  missing: {names:?}");
    }
    if !extra.is_empty() {
        let names: Vec<&str> = extra.iter().map(|s| s.as_str()).collect();
        eprintln!("  extra: {names:?}");
    }
    eprintln!("  report: {}", report_path.display());

    // The staged assertion rides capability-drift's cannot-be-skipped slot
    // (it runs LAST in every gate list and in CI): any non-clean staged
    // outcome fails this verb too, preserving its 0/1 exit contract.
    let staged = staged_gate::evaluate_at(&workspace_root).await;
    staged_gate::emit(&workspace_root, &staged)?;

    if drift_state == "clean" && staged.exit == 0 {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

// Chunk #77 — capability-widening static analysis (per security plan §API
// Anti-Patterns rows 6-7 + security.md Session Addition 2026-05-08
// documented test gap).
//
// 3 capabilities have NEVER-widen invariants that the existing
// capability_drift check does NOT enforce (drift only verifies router↔JSON
// procedure sync, not permission widening within a capability's permissions
// array):
//   - pulse:notification — outbound-emit only (NEVER include
//     notification:allow-register-action-types OR notification:allow-register-listener
//     per pulse-app/capabilities/notification.json:4 description)
//   - pulse:tray — outbound emit only (NEVER include any input-event handler;
//     permissions ending in -register-* / -listen-* / -on-* indicate
//     input-event handlers banned per pulse-app/capabilities/tray.json:4
//     description)
//   - pulse:plugin-fs — backend-only (NEVER expose to webview JavaScript via
//     `windows: [...]` array population NOR add any fs:* / shell:* / dialog:*
//     / http:* permissions per pulse-app/capabilities/plugin-fs.json:4
//     description)
//
// On widening detection: report the specific (capability, permission,
// violated rule) tuple + exit non-zero. Mirrors capability_drift's structured
// report at target/capability-widening/report.json + structured JSON event
// line per obs-plan §3 schema.
async fn capability_widening_check() -> Result<ExitCode> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask manifest has no workspace parent")?
        .to_path_buf();
    let capabilities_dir = workspace_root.join("pulse-app").join("capabilities");

    // Per-capability rules: (capability_name, banned_substrings_in_permissions,
    // require_empty_windows). Order matters for stable reporting.
    let rules: &[(&str, &[&str], bool)] = &[
        // pulse:notification — banned input-event handler permission substrings
        // per security.md Session Addition 2026-05-08 + notification.json:4
        // description.
        (
            "notification",
            &["allow-register-action-types", "allow-register-listener"],
            false,
        ),
        // pulse:tray — banned input-event handler patterns (any -register- /
        // -listen- / -on- suffix in permission name indicates webview→backend
        // event re-entry; outbound-emit only is the invariant per tray.json:4
        // description).
        ("tray", &["-register-", "-listen-", "-on-"], false),
        // pulse:plugin-fs — backend-only, no webview exposure (`windows: []`
        // MUST be empty); no permissions allowed beyond the empty default
        // (any fs:* / shell:* / dialog:* / http:* permission represents
        // widening per plugin-fs.json:4 description).
        ("plugin-fs", &["fs:", "shell:", "dialog:", "http:"], true),
    ];

    let mut violations: Vec<String> = Vec::new();
    let mut inspected: Vec<String> = Vec::new();

    for (cap_name, banned_substrings, require_empty_windows) in rules {
        let path = capabilities_dir.join(format!("{cap_name}.json"));
        inspected.push(format!("{cap_name}.json"));
        let content = fs::read_to_string(&path)
            .with_context(|| format!("read capability JSON at {}", path.display()))?;
        let json: Value = serde_json::from_str(&content)
            .with_context(|| format!("parse capability JSON at {}", path.display()))?;

        // Check permissions array against banned substrings.
        if let Some(permissions) = json.get("permissions").and_then(|v| v.as_array()) {
            for perm in permissions {
                if let Some(perm_str) = perm.as_str() {
                    for banned in *banned_substrings {
                        if perm_str.contains(banned) {
                            violations.push(format!(
                                "pulse:{cap_name} permission `{perm_str}` matches banned substring `{banned}` (widening NEVER-allow per security plan §API Anti-Patterns rows 6-7 + {cap_name}.json description)"
                            ));
                        }
                    }
                }
            }
        }

        // Check windows array IF require_empty_windows.
        if *require_empty_windows
            && let Some(windows) = json.get("windows").and_then(|v| v.as_array())
            && !windows.is_empty()
        {
            let window_names: Vec<String> = windows
                .iter()
                .filter_map(|w| w.as_str().map(|s| s.to_string()))
                .collect();
            violations.push(format!(
                        "pulse:{cap_name} `windows` array MUST be empty (backend-only; no webview exposure per {cap_name}.json description); found {window_names:?}"
                    ));
        }
    }

    let widening_state = if violations.is_empty() {
        "clean"
    } else {
        "violations"
    };

    let report_dir = workspace_root.join("target").join("capability-widening");
    fs::create_dir_all(&report_dir).context("create capability-widening report dir")?;
    let report_path = report_dir.join("report.json");

    let report = serde_json::json!({
        "widening_state": widening_state,
        "violations_count": violations.len(),
        "violations": violations,
        "inspected": inspected,
        "generated_at": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&report_path, serde_json::to_string_pretty(&report)?)
        .context("write widening report")?;

    let event = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "level": if widening_state == "clean" { "INFO" } else { "WARN" },
        "target": "xtask.capability_widening_check",
        "message": "capability widening check complete",
        "fields": {
            "violations_count": violations.len(),
            "widening_state": widening_state,
            "inspected_count": inspected.len(),
        },
    });
    println!("{}", serde_json::to_string(&event)?);

    eprintln!(
        "capability-widening-check: {widening_state} ({} violations across {} inspected)",
        violations.len(),
        inspected.len()
    );
    for v in &violations {
        eprintln!("  ✗ {v}");
    }
    eprintln!("  report: {}", report_path.display());

    if widening_state == "clean" {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

// Parse the ARGS_MAP line from a taurpc-emitted bindings.ts file. Format:
//   const ARGS_MAP = { 'health':'{"check":[]}', 'streams':'{"subscribe_spans":[...]}' }
// Outer is a JS object literal: single-quoted keys → single-quoted values
// where each value is a stringified JSON method-map. Inner JSON uses double
// quotes; outer single quotes delimit. Procedure identifiers are restricted
// to [A-Za-z0-9_] so single quotes only ever delimit, never appear inside
// identifiers. Returns the flat (router.method or method) procedure set.
pub(crate) fn parse_bindings(bindings_content: &str) -> Result<BTreeSet<String>> {
    let args_map_line = bindings_content
        .lines()
        .map(str::trim_start)
        .find_map(|line| line.strip_prefix("const ARGS_MAP = "))
        .context("ARGS_MAP line not found in bindings.ts")?;
    let rhs = args_map_line.trim_end_matches(';').trim();
    let inner = rhs.trim_start_matches('{').trim_end_matches('}').trim();

    let bytes = inner.as_bytes();
    let mut idx = 0usize;
    let mut discovered = BTreeSet::new();

    while idx < bytes.len() {
        while idx < bytes.len() && (bytes[idx].is_ascii_whitespace() || bytes[idx] == b',') {
            idx += 1;
        }
        if idx >= bytes.len() {
            break;
        }
        if bytes[idx] != b'\'' {
            bail!("expected `'` at offset {idx} in ARGS_MAP body");
        }
        idx += 1;
        let router_start = idx;
        while idx < bytes.len() && bytes[idx] != b'\'' {
            idx += 1;
        }
        if idx >= bytes.len() {
            bail!("unterminated router key");
        }
        let router = std::str::from_utf8(&bytes[router_start..idx])
            .context("router key not utf-8")?
            .to_string();
        idx += 1; // consume closing `'`

        while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx >= bytes.len() || bytes[idx] != b':' {
            bail!("expected `:` after router key `{router}`");
        }
        idx += 1;
        while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx >= bytes.len() || bytes[idx] != b'\'' {
            bail!("expected `'` for methods_json of router `{router}`");
        }
        idx += 1;
        let methods_start = idx;
        while idx < bytes.len() && bytes[idx] != b'\'' {
            idx += 1;
        }
        if idx >= bytes.len() {
            bail!("unterminated methods_json for router `{router}`");
        }
        let methods_json =
            std::str::from_utf8(&bytes[methods_start..idx]).context("methods_json not utf-8")?;
        idx += 1; // consume closing `'`

        let methods: BTreeMap<String, Vec<String>> = serde_json::from_str(methods_json)
            .with_context(|| format!("parse methods_json for `{router}`: {methods_json}"))?;
        for method_name in methods.keys() {
            if router.is_empty() {
                discovered.insert(method_name.clone());
            } else {
                discovered.insert(format!("{router}.{method_name}"));
            }
        }
    }
    Ok(discovered)
}

#[cfg(test)]
mod capability_drift_tests {
    use super::*;

    #[test]
    fn parse_bindings_extracts_namespaced_procedures() {
        let bindings = "// header\nconst ARGS_MAP = { 'health':'{\"check\":[]}', 'traces':'{\"query\":[\"args\"]}' }\nexport type Foo = ...";
        let discovered = parse_bindings(bindings).expect("parses");
        assert!(discovered.contains("health.check"));
        assert!(discovered.contains("traces.query"));
        assert_eq!(discovered.len(), 2);
    }

    #[test]
    fn parse_bindings_handles_streams_router() {
        let bindings = "const ARGS_MAP = { 'streams':'{\"subscribe_logs\":[\"channel\"],\"subscribe_metrics\":[\"channel\"],\"subscribe_spans\":[\"channel\"]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        assert!(discovered.contains("streams.subscribe_logs"));
        assert!(discovered.contains("streams.subscribe_metrics"));
        assert!(discovered.contains("streams.subscribe_spans"));
        assert_eq!(discovered.len(), 3);
    }

    #[test]
    fn parse_bindings_handles_top_level_procedures_via_empty_router() {
        // Hypothetical chunk #27 emission shape: top-level procedures may emit
        // under an empty router string. Verify the parser handles this.
        let bindings = "const ARGS_MAP = { '':'{\"app_info\":[],\"health\":[]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        assert!(discovered.contains("app_info"));
        assert!(discovered.contains("health"));
        assert_eq!(discovered.len(), 2);
    }

    #[test]
    fn parse_bindings_missing_args_map_returns_err() {
        let bindings = "// only header here, no ARGS_MAP\nexport type Foo = ...";
        let result = parse_bindings(bindings);
        assert!(result.is_err());
    }

    #[test]
    fn expected_procedures_includes_chunk_27_introspection_envelope() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "app_info",
            "health",
            "ready",
            "get_settings",
            "update_settings",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc}"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_plugins_namespace_at_chunk_47() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in ["plugins.list", "plugins.reload", "plugins.invoke"] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #47 plugins router)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_mcp_namespace_at_chunk_49() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in ["mcp.status", "mcp.start", "mcp.stop"] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #49 mcp router)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_connection_namespace_at_chunk_59() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("connection.current_state"),
            "EXPECTED_PROCEDURES must include connection.current_state (chunk #59 connection router)"
        );
    }

    #[test]
    fn expected_procedures_includes_services_list_with_states_at_chunk_67() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("services.list_with_states"),
            "EXPECTED_PROCEDURES must include services.list_with_states (chunk #67 service registry + lifecycle state machine)"
        );
    }

    #[test]
    fn expected_procedures_includes_storage_namespace_at_chunk_68() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "storage.inspect",
            "storage.path",
            "storage.export_for_training",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #68 corpus SQLite scaffold + storage router; storage.export_for_training added chunk #95)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_diagnostics_namespace_at_chunk_69() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("diagnostics.template_distribution"),
            "EXPECTED_PROCEDURES must include diagnostics.template_distribution (chunk #69 Phase B Session 3 Drain template profiling diagnostics)"
        );
    }

    #[test]
    fn expected_procedures_includes_incidents_namespace_at_chunk_78() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "incidents.list_active",
            "incidents.acknowledge",
            "incidents.mark_resolved",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #78 incident records + lifecycle persistence)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_model_namespace_at_chunk_82() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("model.current_profile"),
            "EXPECTED_PROCEDURES must include model.current_profile (chunk #82 hardware profile detection + model loading + tokenizer)"
        );
    }

    #[test]
    fn expected_procedures_includes_diagnostics_retry_interpretation_at_chunk_86() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("diagnostics.retry_interpretation"),
            "EXPECTED_PROCEDURES must include diagnostics.retry_interpretation (chunk #86 JSON parse failure handling + backoff + resolution summary — Settings → Diagnostics manual override per capability P-020 graceful degradation reaching full)"
        );
    }

    #[test]
    fn expected_procedures_includes_mark_all_read_at_chunk_87() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("incidents.mark_all_read"),
            "EXPECTED_PROCEDURES must include incidents.mark_all_read (chunk #87 Findings counter + dropdown — bulk-mark-as-read action per capabilities P-028 / P-029 / P-030)"
        );
    }

    #[test]
    fn expected_procedures_includes_get_report_at_chunk_88() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        assert!(
            expected.contains("incidents.get_report"),
            "EXPECTED_PROCEDURES must include incidents.get_report (chunk #88 Diagnostic Report generation — L5 in-app report panel + copy markdown per capabilities P-031 + P-035–P-038)"
        );
    }

    #[test]
    fn expected_procedures_includes_config_hot_reload_at_chunk_96() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in [
            "config.reload",
            "config.status",
            "diagnostics.reevaluate_recent_window",
        ] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #96 Configuration hot reload + prospective threshold application per capabilities P-055 / P-056)"
            );
        }
    }

    #[test]
    fn expected_procedures_includes_diagnostics_snapshot_and_history_at_chunk_97() {
        let expected: BTreeSet<&str> = EXPECTED_PROCEDURES.iter().copied().collect();
        for proc in ["diagnostics.snapshot", "diagnostics.history"] {
            assert!(
                expected.contains(proc),
                "EXPECTED_PROCEDURES must include {proc} (chunk #97 Settings → Diagnostics view — L6 self-observability surface per capability P-058)"
            );
        }
    }

    #[test]
    fn drift_detected_when_extra_procedure_present() {
        // Synthetic ARGS_MAP: introduces `streams.subscribe_spans` not in the
        // expected list (D3 carry-over scenario).
        let bindings = "const ARGS_MAP = { 'health':'{\"check\":[]}', 'streams':'{\"subscribe_spans\":[\"channel\"]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        let expected: BTreeSet<String> = ["health.check"].iter().map(|s| s.to_string()).collect();
        let extra: BTreeSet<String> = discovered.difference(&expected).cloned().collect();
        assert!(extra.contains("streams.subscribe_spans"));
    }

    #[test]
    fn drift_clean_when_sets_match() {
        let bindings = "const ARGS_MAP = { 'health':'{\"check\":[]}' }";
        let discovered = parse_bindings(bindings).expect("parses");
        let expected: BTreeSet<String> = ["health.check"].iter().map(|s| s.to_string()).collect();
        let missing: BTreeSet<String> = expected.difference(&discovered).cloned().collect();
        let extra: BTreeSet<String> = discovered.difference(&expected).cloned().collect();
        assert!(missing.is_empty());
        assert!(extra.is_empty());
    }
}

fn status_to_code(status: std::process::ExitStatus) -> ExitCode {
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
