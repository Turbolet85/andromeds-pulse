//! `cargo xtask smoke:hue-shift` — the live leg for the P-025 hue-shift
//! observable (`metric.constellation.hue_update_ms`).
//!
//! Shape: boot the release app on a fresh data dir under deterministic L4,
//! drive the finite storm over real OTLP until the widget paints a tier RISE,
//! then swap to a healthy sustained feed (so the dots stay visible) and wait
//! for the tier FALL once the 120 s auto-resolve fires.
//!
//! The verdict is ANCHOR-shaped, never a clean log: the defect emitted normally
//! with 0 ERROR, it just measured the wrong interval. Each sample's anchor is
//! `record timestamp − duration_ms`; the rise anchor must land on the
//! incident's creation record and the fall anchor on the auto-resolve tick that
//! resolved it, both within `ANCHOR_TOLERANCE_MS`. Before the fix the anchor
//! was the service's `last_seen` lifecycle stamp, uniform over a 15 s tick.
//!
//! The ≤ 2 000 ms budget line is printed as context only — Conductor's live leg
//! grades P-025; this leg proves the observable measures the right interval.
//!
//! The leg proves its own preconditions and reports INCONCLUSIVE — never PASS
//! — when no incident formed or a sample never appeared.

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

const HUE_TARGET: &str = "metric.constellation.hue_update_ms";
const CREATED_TARGET: &str = "interpretation.incident.created";
const AUTO_RESOLVE_TARGET: &str = "triage.incident.auto_resolve.tick";

/// The paint→resolver IPC hop and the backend record's own timestamping are
/// milliseconds; an error of a second or more means a different anchor.
pub const ANCHOR_TOLERANCE_MS: i64 = 1_000;

/// The P-025 budget, printed as context only.
const BUDGET_MS: f64 = 2_000.0;

/// Short enough that the silence family is reachable inside the leg (the
/// `smoke:external-resolve` precedent).
const BOOTSTRAP_SECONDS: u64 = 20;

/// The finite storm runs ~300 s; the rise lands well inside it.
const RISE_WAIT: Duration = Duration::from_secs(300);

/// Auto-resolve fires 120 s after the incident's last re-emission, then the
/// widget's 1 s poll repaints.
const FALL_WAIT: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub duration_ms: f64,
    pub anchor_error_ms: i64,
}

#[derive(Debug, PartialEq)]
pub struct HueJudgement {
    pub rise: Option<Sample>,
    pub fall: Option<Sample>,
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

struct HueRecord {
    index: usize,
    ts_ms: i64,
    duration_ms: f64,
    tier: String,
}

fn hue_records(lines: &[Value]) -> Vec<HueRecord> {
    lines
        .iter()
        .enumerate()
        .filter(|(_, l)| target(l) == Some(HUE_TARGET))
        .filter_map(|(index, l)| {
            Some(HueRecord {
                index,
                ts_ms: timestamp_ms(l)?,
                duration_ms: field(l, "duration_ms")?.as_f64()?,
                tier: field(l, "severity_tier")?.as_str()?.to_string(),
            })
        })
        .collect()
}

fn anchor_ms(r: &HueRecord) -> i64 {
    r.ts_ms - r.duration_ms.round() as i64
}

/// Grade the leg from the app's parsed obs-log family, in emission order.
pub fn judge_hue_shift(lines: &[Value]) -> HueJudgement {
    let created: Vec<i64> = lines
        .iter()
        .filter(|l| target(l) == Some(CREATED_TARGET))
        .filter(|l| field(l, "created").and_then(Value::as_bool) == Some(true))
        .filter_map(timestamp_ms)
        .collect();
    let resolve_ticks: Vec<i64> = lines
        .iter()
        .filter(|l| target(l) == Some(AUTO_RESOLVE_TARGET))
        .filter(|l| {
            field(l, "resolved_count")
                .and_then(Value::as_u64)
                .unwrap_or(0)
                >= 1
        })
        .filter_map(timestamp_ms)
        .collect();
    let hue = hue_records(lines);

    let judged = |rise, fall, outcome| HueJudgement {
        rise,
        fall,
        outcome,
    };

    if created.is_empty() {
        return judged(
            None,
            None,
            LegOutcome::Inconclusive(
                "no incident formed — the storm did not trip, or its creation record never \
                 reached the log"
                    .to_string(),
            ),
        );
    }

    let Some(r) = hue.iter().find(|h| h.tier != "none") else {
        return judged(
            None,
            None,
            LegOutcome::Inconclusive(
                "no rise sample — the widget never painted a non-none tier, or its sample never \
                 reached the log"
                    .to_string(),
            ),
        );
    };
    // An incident that opens after the paint cannot be what the paint shows.
    let Some(rise_error) = created
        .iter()
        .filter(|&&c| c <= r.ts_ms)
        .map(|&c| (anchor_ms(r) - c).abs())
        .min()
    else {
        return judged(
            None,
            None,
            LegOutcome::Inconclusive(
                "the first rise sample precedes every incident creation record".to_string(),
            ),
        );
    };
    let rise = Some(Sample {
        duration_ms: r.duration_ms,
        anchor_error_ms: rise_error,
    });

    // The fall is graded independently of the rise, so a failing run still
    // records both anchor errors.
    let f = hue.iter().find(|h| h.index > r.index && h.tier == "none");
    let t = resolve_ticks.iter().copied().find(|&t| t > r.ts_ms);
    let fall = match (f, t) {
        (Some(f), Some(t)) => Some(Sample {
            duration_ms: f.duration_ms,
            anchor_error_ms: (anchor_ms(f) - t).abs(),
        }),
        _ => None,
    };

    if rise_error > ANCHOR_TOLERANCE_MS {
        return judged(
            rise,
            fall,
            LegOutcome::Fail(format!(
                "the rise anchor is {rise_error} ms from the incident's creation record \
                 (tolerance {ANCHOR_TOLERANCE_MS} ms) — the sample measures a different interval"
            )),
        );
    }
    let Some(fall_sample) = fall.clone() else {
        let why = if f.is_none() {
            "no fall sample within the window — the auto-resolve did not fire, the dot was no \
             longer visible, or the sample never reached the log"
        } else {
            "a fall sample appeared but no auto-resolve tick resolved anything after the rise"
        };
        return judged(rise, None, LegOutcome::Inconclusive(why.to_string()));
    };
    if fall_sample.anchor_error_ms > ANCHOR_TOLERANCE_MS {
        return judged(
            rise,
            fall,
            LegOutcome::Fail(format!(
                "the fall anchor is {} ms from the resolving auto-resolve tick \
                 (tolerance {ANCHOR_TOLERANCE_MS} ms) — the sample measures a different interval",
                fall_sample.anchor_error_ms
            )),
        );
    }
    judged(rise, fall, LegOutcome::Pass)
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
        tokio::time::sleep(Duration::from_secs(2)).await;
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
    let out = workspace_root.join("target").join("hue-shift").join(stamp);
    std::fs::create_dir_all(&out).context("create hue-shift artifact dir")?;
    for src in log_family(data_dir) {
        if let Some(name) = src.file_name() {
            std::fs::copy(&src, out.join(name)).context("copy log family")?;
        }
    }
    Ok(out)
}

fn print_sample(label: &str, sample: Option<&Sample>) {
    match sample {
        Some(s) => println!(
            "smoke:hue-shift: {label} duration_ms={:.0} anchor_error_ms={}",
            s.duration_ms, s.anchor_error_ms
        ),
        None => println!("smoke:hue-shift: {label} none observed"),
    }
}

pub async fn run_hue_shift() -> Result<std::process::ExitCode> {
    let stamp = chrono::Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let workspace_root = &crate::self_verify::workspace_root()?;
    let app = match locate_app_binary(workspace_root) {
        Ok(p) => p,
        Err(e) => {
            println!("smoke:hue-shift: INCONCLUSIVE — {e}");
            return Ok(std::process::ExitCode::from(2));
        }
    };
    println!(
        "smoke:hue-shift: app={} ({})",
        app.display(),
        binary_age(&app)
    );

    let injector = build_injector(workspace_root).await?;
    let data_dir = tempdir(workspace_root)?;
    println!("smoke:hue-shift: data dir {}", data_dir.display());

    let mut child = spawn_app(&app, &data_dir, BOOTSTRAP_SECONDS)?;
    wait_for_receiver().await?;

    println!("smoke:hue-shift: seeding the finite storm over real OTLP");
    let mut storm = spawn_injector(&injector, &data_dir, &[])?;
    let lines = poll_log(&data_dir, RISE_WAIT, |lines| {
        hue_records(lines).iter().any(|h| h.tier != "none")
    })
    .await;
    let rise_seen = hue_records(&lines).iter().any(|h| h.tier != "none");
    let _ = storm.kill().await;

    let mut feed = None;
    if rise_seen {
        // A healthy feed keeps every service inside the widget's 60 s recency
        // window, so the dot is still painted when its tier falls; a 0 % error
        // rate cannot re-trip the storm cue.
        println!("smoke:hue-shift: rise painted — holding the services live with a healthy feed");
        feed = Some(spawn_injector(
            &injector,
            &data_dir,
            &["--sustained", "--error-pct=0"],
        )?);
        poll_log(&data_dir, FALL_WAIT, |lines| {
            let hue = hue_records(lines);
            let Some(r) = hue.iter().find(|h| h.tier != "none") else {
                return false;
            };
            hue.iter().any(|h| h.index > r.index && h.tier == "none")
        })
        .await;
        // The resolving tick precedes the repaint by the poll interval; give a
        // straggling record a moment to flush.
        tokio::time::sleep(Duration::from_secs(3)).await;
    }

    if let Some(mut f) = feed {
        let _ = f.kill().await;
    }
    shutdown(&mut child).await?;
    wait_ports_released().await;

    let lines = read_log_lines(&data_dir);
    let artifact = preserve_logs(workspace_root, &data_dir, &stamp)?;
    println!(
        "smoke:hue-shift: log family preserved at {}",
        artifact.display()
    );
    let panics = lines
        .iter()
        .filter(|l| target(l) == Some("app.panic.fatal"))
        .count();

    let judgement = judge_hue_shift(&lines);
    print_sample("rise", judgement.rise.as_ref());
    print_sample("fall", judgement.fall.as_ref());
    let within = [&judgement.rise, &judgement.fall]
        .iter()
        .filter_map(|s| s.as_ref())
        .all(|s| s.duration_ms <= BUDGET_MS);
    println!(
        "smoke:hue-shift: within the 2000 ms budget: {} (context only — Conductor grades)",
        if within { "yes" } else { "no" }
    );

    match judgement.outcome {
        LegOutcome::Pass if panics > 0 => {
            println!("smoke:hue-shift: FAIL — {panics} panic record(s) in the log");
            Ok(std::process::ExitCode::from(1))
        }
        LegOutcome::Pass => {
            println!("smoke:hue-shift: PASS");
            Ok(std::process::ExitCode::SUCCESS)
        }
        LegOutcome::Fail(why) => {
            println!("smoke:hue-shift: FAIL — {why}");
            Ok(std::process::ExitCode::from(1))
        }
        LegOutcome::Inconclusive(why) => {
            println!("smoke:hue-shift: INCONCLUSIVE — {why}");
            Ok(std::process::ExitCode::from(2))
        }
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

    fn created(ms: i64) -> Value {
        json!({"timestamp": at(ms), "target": CREATED_TARGET,
               "fields": {"created": true, "deduped": false}})
    }

    fn hue(ms: i64, duration_ms: f64, tier: &str) -> Value {
        json!({"timestamp": at(ms), "target": HUE_TARGET,
               "fields": {"duration_ms": duration_ms, "severity_tier": tier}})
    }

    fn resolve_tick(ms: i64, resolved: u64) -> Value {
        json!({"timestamp": at(ms), "target": AUTO_RESOLVE_TARGET,
               "fields": {"evaluated_count": 1, "resolved_count": resolved}})
    }

    fn anchored_run() -> Vec<Value> {
        vec![
            created(T0),
            hue(T0 + 1_400, 1_380.0, "autonomous"),
            resolve_tick(T0 + 200_000, 1),
            hue(T0 + 200_900, 870.0, "none"),
        ]
    }

    #[test]
    fn anchored_rise_and_fall_pass() {
        let j = judge_hue_shift(&anchored_run());
        assert_eq!(j.outcome, LegOutcome::Pass);
        assert_eq!(j.rise.map(|s| s.anchor_error_ms), Some(20));
        assert_eq!(j.fall.map(|s| s.anchor_error_ms), Some(30));
    }

    #[test]
    fn no_incident_is_inconclusive_not_a_pass() {
        let lines = vec![hue(T0, 500.0, "autonomous")];
        assert!(matches!(
            judge_hue_shift(&lines).outcome,
            LegOutcome::Inconclusive(_)
        ));
    }

    #[test]
    fn a_failing_rise_still_reports_the_fall_sample() {
        let mut lines = anchored_run();
        lines[1] = hue(T0 + 1_400, 200.0, "autonomous");
        let j = judge_hue_shift(&lines);
        assert!(matches!(j.outcome, LegOutcome::Fail(_)));
        assert_eq!(j.rise.map(|s| s.anchor_error_ms), Some(1_200));
        assert_eq!(j.fall.map(|s| s.anchor_error_ms), Some(30));
    }

    #[test]
    fn a_rise_anchored_on_a_stale_lifecycle_stamp_fails() {
        // The pre-fix shape: the anchor is the preceding 15 s lifecycle tick.
        let lines = vec![created(T0), hue(T0 + 1_400, 9_400.0, "autonomous")];
        let j = judge_hue_shift(&lines);
        assert!(matches!(j.outcome, LegOutcome::Fail(_)), "{:?}", j.outcome);
        assert_eq!(j.rise.map(|s| s.anchor_error_ms), Some(8_000));
    }

    #[test]
    fn a_missing_fall_is_inconclusive_never_a_pass() {
        let lines = vec![created(T0), hue(T0 + 1_400, 1_380.0, "autonomous")];
        let j = judge_hue_shift(&lines);
        assert!(matches!(j.outcome, LegOutcome::Inconclusive(_)));
        assert!(j.rise.is_some());
    }

    #[test]
    fn a_fall_anchored_away_from_the_resolving_tick_fails() {
        let mut lines = anchored_run();
        lines[3] = hue(T0 + 200_900, 6_000.0, "none");
        assert!(matches!(
            judge_hue_shift(&lines).outcome,
            LegOutcome::Fail(_)
        ));
    }

    #[test]
    fn a_tick_that_resolved_nothing_does_not_anchor_the_fall() {
        let lines = vec![
            created(T0),
            hue(T0 + 1_400, 1_380.0, "autonomous"),
            resolve_tick(T0 + 185_000, 0),
            hue(T0 + 200_900, 870.0, "none"),
        ];
        assert!(matches!(
            judge_hue_shift(&lines).outcome,
            LegOutcome::Inconclusive(_)
        ));
    }

    #[test]
    fn calm_samples_before_the_rise_are_not_the_fall() {
        let mut lines = vec![hue(T0 - 5_000, 0.0, "none")];
        lines.extend(anchored_run());
        assert_eq!(judge_hue_shift(&lines).outcome, LegOutcome::Pass);
    }

    #[test]
    fn a_creation_after_the_paint_never_anchors_the_rise() {
        let lines = vec![hue(T0 + 1_400, 1_380.0, "autonomous"), created(T0 + 1_500)];
        assert!(matches!(
            judge_hue_shift(&lines).outcome,
            LegOutcome::Inconclusive(_)
        ));
    }

    #[test]
    fn log_timestamps_parse_at_millisecond_resolution() {
        let line = json!({"timestamp": "2026-09-29T05:42:03.123Z"});
        assert_eq!(timestamp_ms(&line).map(|ms| ms % 1_000), Some(123));
    }
}
