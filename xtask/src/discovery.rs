//! `cargo xtask smoke:discovery` — the live leg for the P-027 discovery bound
//! (`metric.constellation.discovery_ms`).
//!
//! Shape: boot the release app on a fresh data dir, wait until the webview is
//! polling `services.list_with_states`, then start a healthy sustained feed
//! over real OTLP. Every injector batch carries all five demo services, so the
//! first `duckdb.append {table_name: "spans"}` record is every service's first
//! sighting. The verdict grades the interval from that record to the first
//! `metric.constellation.discovery_ms` record (the paint), and requires the
//! sample's own anchor (`timestamp − duration_ms`) to land on the first
//! sighting — a sample anchored on a later registry tick measures a different
//! interval.
//!
//! The leg proves its own preconditions and reports INCONCLUSIVE — never PASS
//! — when no spans were appended or the webview was not polling first.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde_json::Value;
use tokio::process::{Child, Command};

use crate::external_resolve::{
    LegOutcome, binary_age, build_injector, locate_app_binary, log_family, shutdown, spawn_app,
    tempdir, wait_for_receiver, wait_ports_released,
};

const DISCOVERY_TARGET: &str = "metric.constellation.discovery_ms";
const APPEND_TARGET: &str = "duckdb.append";
const POLL_TARGET: &str = "services.list_with_states.request";
const PANIC_TARGET: &str = "app.panic.fatal";

/// The P-027 bound.
pub const BOUND_MS: i64 = 5_000;

/// The paint→resolver IPC hop and the backend record's own timestamping are
/// milliseconds; an error of a second or more means a different anchor.
pub const ANCHOR_TOLERANCE_MS: i64 = 1_000;

/// How long after the first spans append a discovery sample is waited for.
pub const OBSERVE_WINDOW: Duration = Duration::from_secs(30);

const LIFECYCLE_TICK_S: u64 = 15;
const POLL_S: u64 = 1;

/// Discovery is independent of the baseline bootstrap; the smoke precedent's
/// value keeps the app's boot identical to `smoke:hue-shift`.
const BOOTSTRAP_SECONDS: u64 = 20;

const WEBVIEW_POLL_WAIT: Duration = Duration::from_secs(60);
const FIRST_APPEND_WAIT: Duration = Duration::from_secs(60);

#[derive(Debug, PartialEq)]
pub struct DiscoveryJudgement {
    pub interval_ms: Option<i64>,
    pub anchor_error_ms: Option<i64>,
    pub discovered_count: Option<u64>,
    pub outcome: LegOutcome,
}

fn timestamp_ms(line: &Value) -> Option<i64> {
    let raw = line.get("timestamp")?.as_str()?;
    chrono::DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|t| t.timestamp_millis())
}

fn field<'a>(line: &'a Value, name: &str) -> Option<&'a Value> {
    line.get("fields")?.get(name)
}

fn target(line: &Value) -> Option<&str> {
    line.get("target").and_then(Value::as_str)
}

fn first_spans_append_ms(lines: &[Value]) -> Option<i64> {
    lines
        .iter()
        .filter(|l| target(l) == Some(APPEND_TARGET))
        .filter(|l| field(l, "table_name").and_then(Value::as_str) == Some("spans"))
        .find_map(timestamp_ms)
}

fn first_poll_ms(lines: &[Value]) -> Option<i64> {
    lines
        .iter()
        .filter(|l| target(l) == Some(POLL_TARGET))
        .find_map(timestamp_ms)
}

fn discovery_at_or_after(lines: &[Value], from_ms: i64) -> Option<(i64, &Value)> {
    lines
        .iter()
        .filter(|l| target(l) == Some(DISCOVERY_TARGET))
        .filter_map(|l| Some((timestamp_ms(l)?, l)))
        .find(|(ts, _)| *ts >= from_ms)
}

/// The verdict reads a sample that, before the fix, lands only after the
/// registry's first lifecycle tick. A window shorter than a tick plus a poll
/// could never see that sample, so the RED reading would be unreachable.
pub fn observe_window_supports_verdict(observe_seconds: u64) -> Result<(), String> {
    let floor = LIFECYCLE_TICK_S + POLL_S + 5;
    if observe_seconds < floor {
        return Err(format!(
            "an observe window of {observe_seconds}s cannot outlast a {LIFECYCLE_TICK_S}s \
             lifecycle tick plus a {POLL_S}s poll (needs >= {floor}s); the RED reading would be \
             unreachable"
        ));
    }
    Ok(())
}

/// Grade the leg from the app's parsed obs-log family, in emission order.
pub fn judge_discovery(lines: &[Value]) -> DiscoveryJudgement {
    let judged = |interval_ms, anchor_error_ms, discovered_count, outcome| DiscoveryJudgement {
        interval_ms,
        anchor_error_ms,
        discovered_count,
        outcome,
    };

    let Some(a) = first_spans_append_ms(lines) else {
        return judged(
            None,
            None,
            None,
            LegOutcome::Inconclusive(
                "no spans were appended — the injector never delivered, or the append record \
                 never reached the log"
                    .to_string(),
            ),
        );
    };
    match first_poll_ms(lines) {
        Some(p) if p <= a => {}
        _ => {
            return judged(
                None,
                None,
                None,
                LegOutcome::Inconclusive(
                    "the webview was not polling before the first span arrived — the interval \
                     would measure the webview mount, not discovery"
                        .to_string(),
                ),
            );
        }
    }

    let window_ms = OBSERVE_WINDOW.as_millis() as i64;
    let Some((d_ms, d)) = discovery_at_or_after(lines, a).filter(|(d_ms, _)| d_ms - a <= window_ms)
    else {
        return judged(
            None,
            None,
            None,
            LegOutcome::Fail(format!(
                "no discovery sample within {}s of the first spans append — the dot did not \
                 appear, or its sample never reached the log",
                OBSERVE_WINDOW.as_secs()
            )),
        );
    };

    let duration_ms = field(d, "duration_ms")
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let interval_ms = d_ms - a;
    let anchor_error_ms = ((d_ms - duration_ms.round() as i64) - a).abs();
    let discovered_count = field(d, "discovered_count").and_then(Value::as_u64);
    let panics = lines
        .iter()
        .filter(|l| target(l) == Some(PANIC_TARGET))
        .count();
    let errors = lines
        .iter()
        .filter(|l| l.get("level").and_then(Value::as_str) == Some("ERROR"))
        .count();

    let mut failed = Vec::new();
    if interval_ms > BOUND_MS {
        failed.push(format!(
            "the first-sighting→dot interval is {interval_ms} ms (bound {BOUND_MS} ms)"
        ));
    }
    if anchor_error_ms > ANCHOR_TOLERANCE_MS {
        failed.push(format!(
            "the sample's anchor is {anchor_error_ms} ms from the first spans append \
             (tolerance {ANCHOR_TOLERANCE_MS} ms) — it measures a different interval"
        ));
    }
    if panics > 0 {
        failed.push(format!("{panics} app.panic.fatal record(s) in the log"));
    }
    if errors > 0 {
        failed.push(format!("{errors} ERROR record(s) in the log"));
    }

    let outcome = if failed.is_empty() {
        LegOutcome::Pass
    } else {
        LegOutcome::Fail(failed.join("; "))
    };
    judged(
        Some(interval_ms),
        Some(anchor_error_ms),
        discovered_count,
        outcome,
    )
}

fn read_log_lines(data_dir: &Path) -> Vec<Value> {
    log_family(data_dir)
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .flat_map(|text| {
            text.lines()
                .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                .collect::<Vec<_>>()
        })
        .collect()
}

async fn poll_log(
    data_dir: &Path,
    budget: Duration,
    found: impl Fn(&[Value]) -> bool,
) -> Vec<Value> {
    let deadline = Instant::now() + budget;
    loop {
        let lines = read_log_lines(data_dir);
        if found(&lines) || Instant::now() >= deadline {
            return lines;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

fn spawn_injector(injector: &Path, data_dir: &Path, args: &[&str]) -> Result<Child> {
    Command::new(injector)
        .args(args)
        .current_dir(data_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .context("spawn inject_demo")
}

fn preserve_logs(workspace_root: &Path, data_dir: &Path, stamp: &str) -> Result<PathBuf> {
    let out = workspace_root.join("target").join("discovery").join(stamp);
    std::fs::create_dir_all(&out).context("create discovery artifact dir")?;
    for src in log_family(data_dir) {
        if let Some(name) = src.file_name() {
            std::fs::copy(&src, out.join(name)).context("copy log family")?;
        }
    }
    Ok(out)
}

fn inconclusive(why: &str) -> Result<std::process::ExitCode> {
    println!("smoke:discovery: INCONCLUSIVE — {why}");
    Ok(std::process::ExitCode::from(2))
}

pub async fn run_discovery() -> Result<std::process::ExitCode> {
    let stamp = chrono::Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    if let Err(why) = observe_window_supports_verdict(OBSERVE_WINDOW.as_secs()) {
        return inconclusive(&why);
    }
    let workspace_root = &crate::self_verify::workspace_root()?;
    let app = match locate_app_binary(workspace_root) {
        Ok(p) => p,
        Err(e) => return inconclusive(&e.to_string()),
    };
    println!(
        "smoke:discovery: app={} ({})",
        app.display(),
        binary_age(&app)
    );

    let injector = build_injector(workspace_root).await?;
    let data_dir = tempdir(workspace_root)?;
    println!("smoke:discovery: data dir {}", data_dir.display());
    if !log_family(&data_dir).is_empty() {
        return inconclusive("the data dir already holds a log family — it is not fresh");
    }

    let mut child = spawn_app(&app, &data_dir, BOOTSTRAP_SECONDS)?;
    wait_for_receiver().await?;

    let lines = poll_log(&data_dir, WEBVIEW_POLL_WAIT, |l| first_poll_ms(l).is_some()).await;
    let mut feed = None;
    if first_poll_ms(&lines).is_some() {
        println!("smoke:discovery: webview polling — starting a healthy feed over real OTLP");
        feed = Some(spawn_injector(
            &injector,
            &data_dir,
            &["--sustained", "--error-pct=0"],
        )?);
        let lines = poll_log(&data_dir, FIRST_APPEND_WAIT, |l| {
            first_spans_append_ms(l).is_some()
        })
        .await;
        if let Some(a) = first_spans_append_ms(&lines) {
            // The margin lets a sample painted at the window's edge flush.
            poll_log(&data_dir, OBSERVE_WINDOW + Duration::from_secs(3), |l| {
                discovery_at_or_after(l, a).is_some()
            })
            .await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    if let Some(mut f) = feed {
        let _ = f.kill().await;
    }
    shutdown(&mut child).await?;
    wait_ports_released().await;

    let lines = read_log_lines(&data_dir);
    let artifact = preserve_logs(workspace_root, &data_dir, &stamp)?;

    let judgement = judge_discovery(&lines);
    match (judgement.interval_ms, judgement.anchor_error_ms) {
        (Some(interval), Some(anchor)) => println!(
            "smoke:discovery: first-sighting→dot interval_ms={interval} anchor_error_ms={anchor} \
             discovered_count={}",
            judgement
                .discovered_count
                .map_or_else(|| "absent".to_string(), |c| c.to_string())
        ),
        _ => println!("smoke:discovery: first-sighting→dot none observed"),
    }
    println!(
        "smoke:discovery: within the {BOUND_MS} ms bound: {}",
        if judgement.interval_ms.is_some_and(|i| i <= BOUND_MS) {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "smoke:discovery: log family preserved at {}",
        artifact.display()
    );

    match judgement.outcome {
        LegOutcome::Pass => {
            println!("smoke:discovery: PASS");
            Ok(std::process::ExitCode::SUCCESS)
        }
        LegOutcome::Fail(why) => {
            println!("smoke:discovery: FAIL — {why}");
            Ok(std::process::ExitCode::from(1))
        }
        LegOutcome::Inconclusive(why) => inconclusive(&why),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const T0: i64 = 1_790_000_000_000;

    fn at(ms: i64) -> String {
        chrono::DateTime::from_timestamp_millis(ms)
            .expect("valid instant")
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    }

    fn poll(ms: i64) -> Value {
        json!({"timestamp": at(ms), "level": "INFO", "target": POLL_TARGET,
               "fields": {"item_count": 0}})
    }

    fn append(ms: i64, table: &str) -> Value {
        json!({"timestamp": at(ms), "level": "INFO", "target": APPEND_TARGET,
               "fields": {"rows_appended": 5, "duration_ms": 1, "table_name": table}})
    }

    fn discovery(ms: i64, duration_ms: f64) -> Value {
        json!({"timestamp": at(ms), "level": "INFO", "target": DISCOVERY_TARGET,
               "fields": {"duration_ms": duration_ms, "discovered_count": 5}})
    }

    #[test]
    fn judge_passes_within_bound_anchored_on_first_append() {
        let lines = vec![
            poll(T0 - 3_000),
            append(T0 - 10, "span_events"),
            append(T0, "spans"),
            append(T0 + 400, "spans"),
            discovery(T0 + 1_200, 1_150.0),
        ];
        let j = judge_discovery(&lines);
        assert_eq!(j.outcome, LegOutcome::Pass);
        assert_eq!(j.interval_ms, Some(1_200));
        assert_eq!(j.anchor_error_ms, Some(50));
        assert_eq!(j.discovered_count, Some(5));
    }

    #[test]
    fn judge_fails_over_bound_when_anchor_is_the_tick() {
        // The pre-fix shape: the dot waits for the first lifecycle tick, and
        // the sample anchors on the tick's `last_seen` stamp.
        let lines = vec![
            poll(T0 - 3_000),
            append(T0, "spans"),
            discovery(T0 + 10_000, 500.0),
        ];
        let j = judge_discovery(&lines);
        assert_eq!(j.interval_ms, Some(10_000));
        assert_eq!(j.anchor_error_ms, Some(9_500));
        let LegOutcome::Fail(why) = j.outcome else {
            panic!("expected FAIL, got {:?}", j.outcome);
        };
        assert!(why.contains("interval") && why.contains("anchor"), "{why}");
    }

    #[test]
    fn judge_inconclusive_without_spans_append() {
        let lines = vec![
            poll(T0),
            append(T0 + 100, "logs"),
            discovery(T0 + 500, 400.0),
        ];
        assert!(matches!(
            judge_discovery(&lines).outcome,
            LegOutcome::Inconclusive(_)
        ));
    }

    #[test]
    fn judge_inconclusive_when_webview_not_polling_first() {
        let late = vec![
            append(T0, "spans"),
            poll(T0 + 2_000),
            discovery(T0 + 3_000, 2_900.0),
        ];
        assert!(matches!(
            judge_discovery(&late).outcome,
            LegOutcome::Inconclusive(_)
        ));
        let never = vec![append(T0, "spans"), discovery(T0 + 1_000, 900.0)];
        assert!(matches!(
            judge_discovery(&never).outcome,
            LegOutcome::Inconclusive(_)
        ));
    }

    #[test]
    fn judge_fails_without_discovery_sample_in_window() {
        let none = vec![poll(T0 - 1_000), append(T0, "spans")];
        assert!(matches!(
            judge_discovery(&none).outcome,
            LegOutcome::Fail(_)
        ));
        let outside = vec![
            poll(T0 - 1_000),
            discovery(T0 - 500, 100.0),
            append(T0, "spans"),
            discovery(T0 + 31_000, 500.0),
        ];
        let j = judge_discovery(&outside);
        assert!(matches!(j.outcome, LegOutcome::Fail(_)), "{:?}", j.outcome);
        assert_eq!(j.interval_ms, None);
    }

    #[test]
    fn observe_window_supports_verdict_boundary() {
        assert!(observe_window_supports_verdict(21).is_ok());
        assert!(observe_window_supports_verdict(20).is_err());
        assert!(observe_window_supports_verdict(OBSERVE_WINDOW.as_secs()).is_ok());
    }
}
