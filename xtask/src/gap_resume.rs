//! `cargo xtask smoke:gap-resume` — drive the ingest wedge's real shape and let
//! the shipped drain-progress detector return the verdict.
//!
//! The 2026-08-25 wedge was reconstructed at chunk
//! `2026-08-28-ingest-consumer-initiating-freeze`: `ingest.tick.span_count` sat
//! flat with `buffer_capacity_pct` 0.0 for thirteen ticks (the producer was
//! silent, so the consumer had nothing to consume), the producer resumed, and
//! only then did the channel climb 0 -> 100 % with `rows_ingested` frozen. Tier1
//! `service_went_silent` cadence triggers had begun ~55s BEFORE that onset.
//!
//! So the shape that matters is seed -> GAP -> seed, not sustained load. A
//! sustained feed never lets a service cross its silence threshold, which is why
//! the predecessor's matched leg reproduced the stated preconditions and stayed
//! healthy. `--sustained` runs that control arm deliberately.
//!
//! The verdict comes from `ingest_progress::evaluate` — the detector armed by
//! `2026-08-26-ingest-consumer-stall-under-sustained-load` for exactly this
//! occasion. This module adds no obs target and no second detector.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use serde_json::Value;
use tokio::process::{Child, Command};

use crate::ingest_progress::{self, Verdict};
use crate::self_verify::{locate_pulse_binary, workspace_root};

/// Cold-start window override. The production default is 3600s, which no smoke
/// can outlast; 20s is the value the wedge run itself carried.
const DEFAULT_BOOTSTRAP_SECONDS: u64 = 20;
/// Long enough that every seeded service crosses the bootstrap window and the
/// silence family escalates to tier1 (measured: cues at +21s, tier1 at +120s).
const DEFAULT_GAP_SECONDS: u64 = 180;
/// Run 1 only has to establish per-service baselines before the gap.
const DEFAULT_SEED_MINUTES: u64 = 1;
/// Run 2 must outlast the stall announcement threshold (30 non-draining ticks =
/// 450s) or a real wedge would end before the detector could name it.
const DEFAULT_OBSERVE_MINUTES: u64 = 9;

const OTLP_GRPC_PORT: u16 = 4317;
const OTLP_HTTP_PORT: u16 = 4318;
/// One record per table append. Its presence proves the producer delivered and
/// the consumer reached DuckDB at least once — the reconnect arm's precondition.
const TARGET_DUCKDB_APPEND: &str = "duckdb.append";
/// The stall announcement fires after 30 non-draining ticks at 15s each. A run
/// whose observe window cannot reach this CANNOT fail — `evaluate` would be
/// structurally incapable of reporting the stall, so its PASS would be vacuous.
const STALL_ANNOUNCEMENT_SECONDS: u64 = 450;
/// `tracing_appender`'s non-blocking writer holds the log open; give it room to
/// flush before the process is torn down and the family is read.
const FLUSH_SETTLE: Duration = Duration::from_secs(5);

/// Which scenario this run drives. The gap arm changes TWO variables at once —
/// an idle window long enough for the silence family to escalate, AND a new
/// gRPC connection (the injector connects once per process) — so it cannot say
/// which one triggers the wedge. `ReconnectOnly` removes the idle and keeps the
/// reconnect; the pair separates them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegArm {
    /// seed -> GAP -> resume. Reproduces the wedge.
    #[default]
    GapResume,
    /// One unbroken feed. No gap, so no storm can form.
    Sustained,
    /// seed -> resume with NO gap between the two producer processes.
    ReconnectOnly,
}

impl LegArm {
    pub fn label(&self) -> &'static str {
        match self {
            LegArm::GapResume => "gap-resume",
            LegArm::Sustained => "sustained-control",
            LegArm::ReconnectOnly => "reconnect-only",
        }
    }
}

pub struct GapResumeOptions {
    pub arm: LegArm,
    pub bootstrap_seconds: u64,
    pub gap_seconds: u64,
    pub observe_minutes: u64,
}

impl Default for GapResumeOptions {
    fn default() -> Self {
        Self {
            arm: LegArm::GapResume,
            bootstrap_seconds: DEFAULT_BOOTSTRAP_SECONDS,
            gap_seconds: DEFAULT_GAP_SECONDS,
            observe_minutes: DEFAULT_OBSERVE_MINUTES,
        }
    }
}

/// What the log says about the scenario's own preconditions. A verdict read
/// without this is not interpretable: a PASS with no storm in the log means the
/// scenario never produced the condition under test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StormEvidence {
    pub silence_cues: usize,
    pub tier1_triggers: usize,
    pub buffer_ticks: usize,
    pub appends: usize,
    /// Appends the buffer REJECTED — `duckdb.append` carrying a `reject_reason`.
    /// The gap arm replays span identities on purpose, so a rejection here is
    /// the positive evidence that the constraint-violation path was exercised
    /// rather than quietly avoided.
    pub append_rejections: usize,
}

impl StormEvidence {
    pub fn fired(&self) -> bool {
        self.silence_cues > 0 && self.tier1_triggers > 0
    }

    /// The reconnect arm's own precondition. It has no storm by construction, so
    /// it cannot borrow `fired()`; what makes its verdict countable is that the
    /// producer actually delivered — at least one batch reached DuckDB. Without
    /// this a run where the injector never connected would report a clean PASS
    /// while exercising nothing.
    pub fn fed(&self) -> bool {
        self.appends > 0
    }
}

/// Count the records that prove the silence storm actually formed. Keyed on the
/// same fields the emitters carry: `triage.cue.emit{kind}` and
/// `cadence.trigger{mode, cue_kind}`.
pub fn storm_evidence(lines: &[Value]) -> StormEvidence {
    let mut evidence = StormEvidence::default();
    for line in lines {
        let target = line.get("target").and_then(Value::as_str).unwrap_or("");
        let fields = line.get("fields");
        match target {
            "triage.cue.emit" => {
                let kind = fields
                    .and_then(|f| f.get("kind"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                if kind == "service_went_silent" {
                    evidence.silence_cues += 1;
                }
            }
            "cadence.trigger" => {
                let mode = fields
                    .and_then(|f| f.get("mode"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let cue_kind = fields
                    .and_then(|f| f.get("cue_kind"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                if mode == "tier1" && cue_kind == "service_went_silent" {
                    evidence.tier1_triggers += 1;
                }
            }
            ingest_progress::TARGET_BUFFER_TICK => evidence.buffer_ticks += 1,
            TARGET_DUCKDB_APPEND => {
                evidence.appends += 1;
                if fields
                    .and_then(|f| f.get("reject_reason"))
                    .and_then(Value::as_str)
                    .is_some_and(|r| !r.is_empty())
                {
                    evidence.append_rejections += 1;
                }
            }
            _ => {}
        }
    }
    evidence
}

/// The leg's outcome, separating "the consumer stalled" from "the scenario never
/// produced the condition". Both are non-zero exits; only one is a product fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegOutcome {
    Pass { detail: String },
    Inconclusive { reason: String },
    Fail { reason: String },
}

/// Combine the shipped verdict with the precondition evidence. In the gap arm a
/// storm is REQUIRED (its absence makes any pass vacuous); in the sustained
/// control arm the storm must be ABSENT, which is what makes the two arms
/// different tests rather than the same one run twice. The reconnect arm has no
/// storm by construction, so it carries its own precondition — that the
/// producer actually delivered across the reconnect (`fed`).
pub fn judge(verdict: &Verdict, evidence: &StormEvidence, arm: LegArm) -> LegOutcome {
    if let Verdict::Fail {
        reason,
        consequence,
        stalled_seconds,
    } = verdict
    {
        return LegOutcome::Fail {
            reason: format!(
                "buffer consumer stalled for {stalled_seconds}s (reason={reason}, consequence={consequence})"
            ),
        };
    }

    if evidence.buffer_ticks == 0 {
        return LegOutcome::Inconclusive {
            reason: "no `buffer.tick` records in the run's log family — the app never ticked"
                .to_string(),
        };
    }

    if arm == LegArm::Sustained {
        if evidence.fired() {
            return LegOutcome::Inconclusive {
                reason: format!(
                    "control arm produced a silence storm it should not have ({} cues / {} tier1 triggers) — the sustained feed lapsed",
                    evidence.silence_cues, evidence.tier1_triggers
                ),
            };
        }
        return LegOutcome::Pass {
            detail: format!(
                "sustained control: no silence storm (as designed), {} buffer ticks, consumer kept draining",
                evidence.buffer_ticks
            ),
        };
    }

    if arm == LegArm::ReconnectOnly {
        if evidence.fired() {
            return LegOutcome::Inconclusive {
                reason: format!(
                    "reconnect arm produced a silence storm it should not have ({} cues / {} tier1 triggers) — the gap-free run lapsed, so it is not isolating the reconnect",
                    evidence.silence_cues, evidence.tier1_triggers
                ),
            };
        }
        if !evidence.fed() {
            return LegOutcome::Inconclusive {
                reason: "no `duckdb.append` records — the producer never delivered across the reconnect, so the arm exercised nothing and its pass would prove nothing".to_string(),
            };
        }
        return LegOutcome::Pass {
            detail: format!(
                "reconnect-only: producer reconnected with no idle gap and no storm formed (as designed), {} appends across {} buffer ticks, consumer kept draining — the idle window, not the reconnect, is implicated",
                evidence.appends, evidence.buffer_ticks
            ),
        };
    }

    if !evidence.fired() {
        return LegOutcome::Inconclusive {
            reason: format!(
                "the silence storm never formed ({} `service_went_silent` cues / {} tier1 triggers) — the scenario did not produce the condition under test, so its pass proves nothing",
                evidence.silence_cues, evidence.tier1_triggers
            ),
        };
    }

    // The gap arm replays identities on purpose (`--replay`), so the append
    // path MUST have rejected at least one batch. Without that the arm proves
    // only that nothing stalled — which it would also report if the collision
    // never happened, i.e. a pass for the wrong reason.
    if evidence.append_rejections == 0 {
        return LegOutcome::Inconclusive {
            reason: format!(
                "no rejected `duckdb.append` in {} appends — the replayed identities never reached the primary key, so a clean drain proves the collision was absent rather than survived",
                evidence.appends
            ),
        };
    }

    LegOutcome::Pass {
        detail: format!(
            "gap arm: storm formed ({} cues / {} tier1 triggers), {} of {} appends rejected on replayed identities, and the consumer kept draining across {} buffer ticks",
            evidence.silence_cues,
            evidence.tier1_triggers,
            evidence.append_rejections,
            evidence.appends,
            evidence.buffer_ticks
        ),
    }
}

/// Refuse a verdict the run window cannot support. `evaluate` keys on a stall
/// announced only after `STALL_ANNOUNCEMENT_SECONDS`, so a shorter observe
/// window makes the failure branch unreachable and every run reports PASS.
pub fn observe_window_supports_verdict(observe_minutes: u64) -> Result<(), String> {
    let window_seconds = observe_minutes.saturating_mul(60);
    if window_seconds < STALL_ANNOUNCEMENT_SECONDS {
        return Err(format!(
            "--observe-minutes={observe_minutes} gives a {window_seconds}s window, below the {STALL_ANNOUNCEMENT_SECONDS}s stall announcement threshold the verdict reads — the leg could not fail, so its PASS would be vacuous"
        ));
    }
    Ok(())
}

pub async fn run_gap_resume(options: GapResumeOptions) -> Result<std::process::ExitCode> {
    if let Err(reason) = observe_window_supports_verdict(options.observe_minutes) {
        eprintln!("::error::smoke:gap-resume: INCONCLUSIVE — {reason}");
        return Ok(std::process::ExitCode::FAILURE);
    }

    let root = workspace_root()?;
    let Some(binary) = locate_pulse_binary(&root) else {
        println!(
            "smoke:gap-resume: SKIP — no pulse-app binary found; run `cargo build -p pulse-app` first"
        );
        return Ok(std::process::ExitCode::SUCCESS);
    };
    let injector = build_injector(&root).await?;

    // `locate_pulse_binary` prefers target/release over target/debug, so the leg
    // can silently measure a build older than the change under test — a leg that
    // runs, logs clean, and reports the OLD behaviour as current. Name the file
    // and its age so a stale read is visible instead of inverted.
    println!(
        "smoke:gap-resume: binary={} ({})",
        binary.display(),
        binary_age(&binary)
    );

    let data_dir = tempdir(&root)?;
    println!(
        "smoke:gap-resume: arm={} data_dir={}",
        options.arm.label(),
        data_dir.display()
    );

    let mut app = spawn_app(&binary, &data_dir, options.bootstrap_seconds)?;
    let result = drive(&injector, &data_dir, &options).await;
    let _ = app.kill().await;
    wait_ports_released().await;
    result?;

    let lines = ingest_progress::parse_lines(&log_family(&data_dir));
    let verdict = ingest_progress::evaluate(&lines);
    let evidence = storm_evidence(&lines);

    match judge(&verdict, &evidence, options.arm) {
        LegOutcome::Pass { detail } => {
            println!("smoke:gap-resume: PASS — {detail}");
            Ok(std::process::ExitCode::SUCCESS)
        }
        LegOutcome::Inconclusive { reason } => {
            eprintln!("::error::smoke:gap-resume: INCONCLUSIVE — {reason}");
            Ok(std::process::ExitCode::FAILURE)
        }
        LegOutcome::Fail { reason } => {
            eprintln!("::error::smoke:gap-resume: FAIL — {reason}");
            Ok(std::process::ExitCode::FAILURE)
        }
    }
}

async fn drive(injector: &Path, data_dir: &Path, options: &GapResumeOptions) -> Result<()> {
    wait_for_receiver().await?;

    match options.arm {
        LegArm::Sustained => {
            println!("smoke:gap-resume: sustained control — no gap, continuous feed");
            seed(
                injector,
                &[
                    "--sustained".to_string(),
                    format!("--minutes={}", options.observe_minutes),
                ],
            )
            .await?;
        }
        LegArm::ReconnectOnly => {
            println!("smoke:gap-resume: seed 1 ({}m)", DEFAULT_SEED_MINUTES);
            seed(injector, &[format!("--minutes={DEFAULT_SEED_MINUTES}")]).await?;

            // No sleep between the two producer processes: the injector connects
            // once per process, so run 2 is a NEW gRPC connection with no idle
            // window. That is the whole point of this arm — the gap arm varies
            // both at once.
            println!(
                "smoke:gap-resume: reconnect immediately — new gRPC connection, NO idle gap ({}m)",
                options.observe_minutes
            );
            seed(
                injector,
                &[format!("--minutes={}", options.observe_minutes)],
            )
            .await?;
        }
        LegArm::GapResume => {
            // BOTH runs carry `--replay`, which pins the injector's identity
            // salt so run 2 deliberately re-emits run 1's `(trace_id, span_id)`
            // pairs. Without it the salted default never collides and the arm
            // would pass because the collision was absent, not survived —
            // modelling a retrying OTLP exporter is the point of the arm.
            println!(
                "smoke:gap-resume: seed 1 ({}m, --replay)",
                DEFAULT_SEED_MINUTES
            );
            seed(
                injector,
                &[
                    format!("--minutes={DEFAULT_SEED_MINUTES}"),
                    "--replay".to_string(),
                ],
            )
            .await?;

            println!(
                "smoke:gap-resume: gap {}s — every service crosses the {}s bootstrap window",
                options.gap_seconds, options.bootstrap_seconds
            );
            tokio::time::sleep(Duration::from_secs(options.gap_seconds)).await;

            println!(
                "smoke:gap-resume: resume ({}m, --replay, outlasting the {}s stall threshold)",
                options.observe_minutes, STALL_ANNOUNCEMENT_SECONDS
            );
            seed(
                injector,
                &[
                    format!("--minutes={}", options.observe_minutes),
                    "--replay".to_string(),
                ],
            )
            .await?;
        }
    }

    tokio::time::sleep(FLUSH_SETTLE).await;
    let _ = data_dir;
    Ok(())
}

async fn seed(injector: &Path, args: &[String]) -> Result<()> {
    let status = Command::new(injector)
        .args(args)
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .status()
        .await
        .context("spawn inject_demo")?;
    if !status.success() {
        bail!("inject_demo exited with {status}");
    }
    Ok(())
}

fn spawn_app(binary: &Path, data_dir: &Path, bootstrap_seconds: u64) -> Result<Child> {
    // CWD is the throwaway data dir, not the workspace: TauRPC's dev-mode
    // `export_types()` writes bindings RELATIVE to the working directory, so a
    // repo-rooted CWD strands a stray copy over the committed one.
    Command::new(binary)
        .current_dir(data_dir)
        .env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .env(
            "ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS",
            bootstrap_seconds.to_string(),
        )
        .env("ANDROMEDA_PULSE_L4_DETERMINISTIC", "true")
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .context("spawn pulse-app")
}

async fn wait_for_receiver() -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(60);
    while Instant::now() < deadline {
        if tokio::net::TcpStream::connect(("127.0.0.1", OTLP_GRPC_PORT))
            .await
            .is_ok()
        {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    bail!("OTLP gRPC receiver never accepted on :{OTLP_GRPC_PORT} within 60s")
}

async fn wait_ports_released() {
    let deadline = Instant::now() + Duration::from_secs(15);
    while Instant::now() < deadline {
        let grpc = tokio::net::TcpStream::connect(("127.0.0.1", OTLP_GRPC_PORT))
            .await
            .is_ok();
        let http = tokio::net::TcpStream::connect(("127.0.0.1", OTLP_HTTP_PORT))
            .await
            .is_ok();
        if !grpc && !http {
            return;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    eprintln!("smoke:gap-resume: warning — OTLP ports still accepting after shutdown");
}

fn binary_age(binary: &Path) -> String {
    let Ok(modified) = std::fs::metadata(binary).and_then(|m| m.modified()) else {
        return "mtime unavailable".to_string();
    };
    match modified.elapsed() {
        Ok(age) => format!("built {}m ago", age.as_secs() / 60),
        Err(_) => "built in the future (clock skew)".to_string(),
    }
}

fn tempdir(root: &Path) -> Result<PathBuf> {
    let dir = root
        .join("target")
        .join("gap-resume")
        .join(format!("run-{}", std::process::id()));
    std::fs::create_dir_all(&dir).context("create gap-resume data dir")?;
    Ok(dir)
}

// The obs sink is `rolling::daily`, so every file is date-suffixed and the bare
// name matches nothing — resolve the family (verification-harness.md 2026-06-29).
fn log_family(data_dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(data_dir.join("logs")) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("agent-latest.jsonl")
        })
        .map(|e| e.path())
        .collect();
    files.sort();
    files
}

async fn build_injector(workspace_root: &Path) -> Result<PathBuf> {
    println!("smoke:gap-resume: building inject_demo (outside the timed section)");
    let status = Command::new("cargo")
        .args(["build", "-p", "ingest", "--example", "inject_demo"])
        .current_dir(workspace_root)
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .status()
        .await
        .context("build inject_demo")?;
    if !status.success() {
        bail!("building inject_demo failed with {status}");
    }
    let exe = if cfg!(windows) {
        "inject_demo.exe"
    } else {
        "inject_demo"
    };
    let path = workspace_root
        .join("target")
        .join("debug")
        .join("examples")
        .join(exe);
    if !path.is_file() {
        bail!("inject_demo built but not found at {}", path.display());
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn cue(kind: &str) -> Value {
        json!({"target": "triage.cue.emit", "fields": {"kind": kind}})
    }

    fn trigger(mode: &str, cue_kind: &str) -> Value {
        json!({"target": "cadence.trigger", "fields": {"mode": mode, "cue_kind": cue_kind}})
    }

    fn tick(delta: u64) -> Value {
        json!({"target": "buffer.tick", "fields": {"rows_ingested_delta": delta}})
    }

    fn append(rows: u64) -> Value {
        json!({"target": "duckdb.append", "fields": {"rows_appended": rows}})
    }

    #[test]
    fn storm_evidence_counts_only_the_silence_family() {
        let lines = vec![
            cue("service_went_silent"),
            cue("error_rate_spike"),
            trigger("tier1", "service_went_silent"),
            trigger("tier2", "service_went_silent"),
            trigger("tier1", "error_rate_spike"),
            tick(5),
        ];
        let e = storm_evidence(&lines);
        assert_eq!(e.silence_cues, 1);
        assert_eq!(e.tier1_triggers, 1);
        assert_eq!(e.buffer_ticks, 1);
        assert!(e.fired());
    }

    #[test]
    fn feed_evidence_counts_appends_and_is_independent_of_the_storm() {
        // The reconnect arm's precondition must not be satisfiable by storm
        // records, and must not require them — the two are separate signals.
        let fed_no_storm = storm_evidence(&[append(10), append(4), tick(3)]);
        assert_eq!(fed_no_storm.appends, 2);
        assert!(fed_no_storm.fed());
        assert!(!fed_no_storm.fired());

        let storm_no_feed = storm_evidence(&[
            cue("service_went_silent"),
            trigger("tier1", "service_went_silent"),
            tick(0),
        ]);
        assert_eq!(storm_no_feed.appends, 0);
        assert!(!storm_no_feed.fed());
        assert!(storm_no_feed.fired());
    }

    #[test]
    fn storm_has_not_fired_without_both_halves() {
        let cues_only = storm_evidence(&[cue("service_went_silent"), tick(1)]);
        assert!(!cues_only.fired());
        let triggers_only = storm_evidence(&[trigger("tier1", "service_went_silent"), tick(1)]);
        assert!(!triggers_only.fired());
    }

    #[test]
    fn a_gap_arm_pass_without_a_storm_is_inconclusive_not_a_pass() {
        let verdict = Verdict::Pass {
            ticks: 40,
            longest_zero_delta_run: 3,
        };
        let evidence = StormEvidence {
            silence_cues: 0,
            tier1_triggers: 0,
            buffer_ticks: 40,
            appends: 12,
            append_rejections: 0,
        };
        match judge(&verdict, &evidence, LegArm::GapResume) {
            LegOutcome::Inconclusive { reason } => assert!(reason.contains("never formed")),
            other => panic!("expected Inconclusive, got {other:?}"),
        }
    }

    #[test]
    fn a_gap_arm_pass_with_a_storm_is_a_pass() {
        let verdict = Verdict::Pass {
            ticks: 40,
            longest_zero_delta_run: 12,
        };
        let evidence = StormEvidence {
            silence_cues: 20,
            tier1_triggers: 4,
            buffer_ticks: 40,
            appends: 30,
            append_rejections: 2,
        };
        assert!(matches!(
            judge(&verdict, &evidence, LegArm::GapResume),
            LegOutcome::Pass { .. }
        ));
    }

    // The gap arm replays identities deliberately, so a drain with NOTHING
    // rejected means the collision never happened — the pass would be for the
    // wrong reason. This is the discriminating half: the pass-arm test above
    // stays green whether or not the rule exists.
    #[test]
    fn a_gap_arm_drain_with_no_rejected_append_is_inconclusive() {
        let verdict = Verdict::Pass {
            ticks: 40,
            longest_zero_delta_run: 12,
        };
        let evidence = StormEvidence {
            silence_cues: 20,
            tier1_triggers: 4,
            buffer_ticks: 40,
            appends: 30,
            append_rejections: 0,
        };
        match judge(&verdict, &evidence, LegArm::GapResume) {
            LegOutcome::Inconclusive { reason } => {
                assert!(reason.contains("rejected"), "got {reason}")
            }
            other => panic!("expected Inconclusive, got {other:?}"),
        }
    }

    #[test]
    fn a_rejected_append_is_counted_from_the_reject_reason_field() {
        let rejected = serde_json::json!({
            "target": "duckdb.append",
            "fields": { "reject_reason": "append_failed" }
        });
        let accepted = serde_json::json!({
            "target": "duckdb.append",
            "fields": { "rows_appended": 12, "table_name": "spans" }
        });
        let evidence = storm_evidence(&[rejected, accepted, tick(1)]);
        assert_eq!(evidence.appends, 2, "both records are appends");
        assert_eq!(
            evidence.append_rejections, 1,
            "only the record carrying reject_reason is a rejection"
        );
    }

    #[test]
    fn an_observe_window_below_the_stall_threshold_is_refused() {
        // Measured 2026-08-28: --observe-minutes=3 reported PASS over a
        // consumer that had in fact stopped, because 180s cannot reach the
        // 450s announcement the verdict reads.
        let err = observe_window_supports_verdict(3).expect_err("3m must be refused");
        assert!(err.contains("450"), "got {err}");
        assert!(err.contains("vacuous"), "got {err}");

        assert!(observe_window_supports_verdict(7).is_err(), "420s < 450s");
        observe_window_supports_verdict(8).expect("480s clears the threshold");
        observe_window_supports_verdict(DEFAULT_OBSERVE_MINUTES)
            .expect("the default must support its own verdict");
    }

    #[test]
    fn a_stall_record_fails_both_arms_regardless_of_evidence() {
        let verdict = Verdict::Fail {
            reason: "channel_backlogged".to_string(),
            consequence: "spans_accepted_not_persisted".to_string(),
            stalled_seconds: 450,
        };
        let evidence = StormEvidence::default();
        for arm in [LegArm::GapResume, LegArm::Sustained, LegArm::ReconnectOnly] {
            match judge(&verdict, &evidence, arm) {
                LegOutcome::Fail { reason } => assert!(reason.contains("450")),
                other => panic!("expected Fail in {arm:?}, got {other:?}"),
            }
        }
    }

    #[test]
    fn the_control_arm_requires_the_storm_to_be_absent() {
        let verdict = Verdict::Pass {
            ticks: 40,
            longest_zero_delta_run: 0,
        };
        let quiet = StormEvidence {
            silence_cues: 0,
            tier1_triggers: 0,
            buffer_ticks: 40,
            appends: 30,
            append_rejections: 0,
        };
        assert!(matches!(
            judge(&verdict, &quiet, LegArm::Sustained),
            LegOutcome::Pass { .. }
        ));

        let stormed = StormEvidence {
            silence_cues: 9,
            tier1_triggers: 2,
            buffer_ticks: 40,
            appends: 30,
            append_rejections: 0,
        };
        match judge(&verdict, &stormed, LegArm::Sustained) {
            LegOutcome::Inconclusive { reason } => assert!(reason.contains("should not have")),
            other => panic!("expected Inconclusive, got {other:?}"),
        }
    }

    #[test]
    fn the_reconnect_arm_requires_a_delivered_feed_and_no_storm() {
        let verdict = Verdict::Pass {
            ticks: 40,
            longest_zero_delta_run: 0,
        };

        // The arm it exists to be: reconnect happened, feed landed, no storm.
        let clean = StormEvidence {
            silence_cues: 0,
            tier1_triggers: 0,
            buffer_ticks: 40,
            appends: 25,
            append_rejections: 0,
        };
        match judge(&verdict, &clean, LegArm::ReconnectOnly) {
            LegOutcome::Pass { detail } => {
                assert!(detail.contains("reconnect-only"));
                assert!(detail.contains("25 appends"));
            }
            other => panic!("expected Pass, got {other:?}"),
        }

        // A storm means the gap-free run lapsed — it is no longer isolating the
        // reconnect, so its verdict says nothing about the trigger.
        let stormed = StormEvidence {
            silence_cues: 4,
            tier1_triggers: 1,
            buffer_ticks: 40,
            appends: 25,
            append_rejections: 0,
        };
        match judge(&verdict, &stormed, LegArm::ReconnectOnly) {
            LegOutcome::Inconclusive { reason } => assert!(reason.contains("gap-free run lapsed")),
            other => panic!("expected Inconclusive, got {other:?}"),
        }

        // The precondition that stops a vacuous pass: the producer never
        // delivered, so a clean drain-progress verdict proves nothing.
        let unfed = StormEvidence {
            silence_cues: 0,
            tier1_triggers: 0,
            buffer_ticks: 40,
            appends: 0,
            append_rejections: 0,
        };
        match judge(&verdict, &unfed, LegArm::ReconnectOnly) {
            LegOutcome::Inconclusive { reason } => {
                assert!(reason.contains("never delivered across the reconnect"));
            }
            other => panic!("expected Inconclusive, got {other:?}"),
        }
    }

    #[test]
    fn a_tickless_run_is_inconclusive_in_every_arm() {
        let verdict = Verdict::Neutral("no ticks".to_string());
        let evidence = StormEvidence::default();
        for arm in [LegArm::GapResume, LegArm::Sustained, LegArm::ReconnectOnly] {
            match judge(&verdict, &evidence, arm) {
                LegOutcome::Inconclusive { reason } => assert!(reason.contains("never ticked")),
                other => panic!("expected Inconclusive in {arm:?}, got {other:?}"),
            }
        }
    }
}
