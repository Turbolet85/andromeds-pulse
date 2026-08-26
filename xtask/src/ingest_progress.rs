//! `cargo xtask check:ingest-progress` — fail a run in which the buffer
//! consumer stopped draining.
//!
//! The measured wedge (2026-08-26) carried 0 ERROR and 0 `app.panic.fatal` with
//! heartbeats ticking throughout, so every existing gate read it as healthy:
//! obs-plan §3/§10 define a stall as tick ABSENCE >45s, and the ticks never
//! stopped. This gate keys on PROGRESS instead of liveness.
//!
//! NEUTRAL-tolerant per obs-plan §10 Load-profile constraints: an absent or
//! tick-free log stream is not a failure, so the same command is safe in
//! headless CI and after a booted-app session.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

pub const TARGET_STALLED: &str = "buffer.consumer.stalled";
pub const TARGET_BUFFER_TICK: &str = "buffer.tick";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// No usable stream — reported, never failed.
    Neutral(String),
    Pass {
        ticks: usize,
        longest_zero_delta_run: usize,
    },
    Fail {
        reason: String,
        consequence: String,
        stalled_seconds: u64,
    },
}

/// Scan the run's own record. A stall announcement that is not a recovery is
/// the failure; everything else is pass-or-neutral.
pub fn evaluate(lines: &[Value]) -> Verdict {
    let mut ticks = 0usize;
    let mut longest_zero_delta_run = 0usize;
    let mut current_zero_run = 0usize;

    for line in lines {
        let target = line.get("target").and_then(Value::as_str).unwrap_or("");
        let fields = line.get("fields");

        if target == TARGET_STALLED {
            let reason = fields
                .and_then(|f| f.get("reason"))
                .and_then(Value::as_str)
                .unwrap_or("");
            if reason != "recovered" {
                return Verdict::Fail {
                    reason: reason.to_string(),
                    consequence: fields
                        .and_then(|f| f.get("consequence"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string(),
                    stalled_seconds: fields
                        .and_then(|f| f.get("stalled_seconds"))
                        .and_then(Value::as_u64)
                        .unwrap_or(0),
                };
            }
        }

        if target == TARGET_BUFFER_TICK {
            ticks += 1;
            let delta = fields
                .and_then(|f| f.get("rows_ingested_delta"))
                .and_then(Value::as_u64);
            match delta {
                Some(0) => {
                    current_zero_run += 1;
                    longest_zero_delta_run = longest_zero_delta_run.max(current_zero_run);
                }
                Some(_) => current_zero_run = 0,
                // A tick without the field predates the drain-progress fields;
                // it carries no progress information either way.
                None => {}
            }
        }
    }

    if ticks == 0 {
        return Verdict::Neutral(
            "no `buffer.tick` events in the log family — nothing to assert".to_string(),
        );
    }

    Verdict::Pass {
        ticks,
        longest_zero_delta_run,
    }
}

pub fn parse_lines(log_files: &[PathBuf]) -> Vec<Value> {
    let mut out = Vec::new();
    for path in log_files {
        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };
        for line in content.lines() {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                out.push(v);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn tick(delta: Option<u64>) -> Value {
        match delta {
            Some(d) => json!({"target": TARGET_BUFFER_TICK, "fields": {"rows_ingested_delta": d}}),
            None => json!({"target": TARGET_BUFFER_TICK, "fields": {}}),
        }
    }

    #[test]
    fn empty_stream_is_neutral_not_failure() {
        assert!(matches!(evaluate(&[]), Verdict::Neutral(_)));
    }

    #[test]
    fn ticks_without_progress_fields_are_neutral_on_progress_but_still_counted() {
        let v = evaluate(&[tick(None), tick(None)]);
        assert_eq!(
            v,
            Verdict::Pass {
                ticks: 2,
                longest_zero_delta_run: 0
            }
        );
    }

    #[test]
    fn advancing_ticks_pass() {
        let v = evaluate(&[tick(Some(10)), tick(Some(7))]);
        assert_eq!(
            v,
            Verdict::Pass {
                ticks: 2,
                longest_zero_delta_run: 0
            }
        );
    }

    #[test]
    fn zero_delta_alone_is_not_a_failure() {
        // An idle producer legitimately leaves rows static; only the runtime
        // classifier, which also sees channel occupancy, can call a stall.
        let v = evaluate(&[tick(Some(0)), tick(Some(0)), tick(Some(5))]);
        assert_eq!(
            v,
            Verdict::Pass {
                ticks: 3,
                longest_zero_delta_run: 2
            }
        );
    }

    #[test]
    fn a_stall_announcement_fails_the_run() {
        let lines = vec![
            tick(Some(3)),
            json!({
                "target": TARGET_STALLED,
                "fields": {
                    "reason": "rows_static_while_channel_queued",
                    "consequence": "accepted_spans_not_persisted",
                    "stalled_seconds": 450
                }
            }),
        ];
        match evaluate(&lines) {
            Verdict::Fail {
                reason,
                stalled_seconds,
                ..
            } => {
                assert_eq!(reason, "rows_static_while_channel_queued");
                assert_eq!(stalled_seconds, 450);
            }
            other => panic!("expected Fail, got {other:?}"),
        }
    }

    #[test]
    fn a_recovery_announcement_does_not_fail_the_run() {
        // The recovery event shares the target; keying on presence alone would
        // fail every run that recovered, which is the opposite of the intent.
        let lines = vec![
            json!({
                "target": TARGET_STALLED,
                "fields": {"reason": "recovered", "consequence": "drain_resumed", "stalled_seconds": 0}
            }),
            tick(Some(4)),
        ];
        assert!(matches!(evaluate(&lines), Verdict::Pass { .. }));
    }
}
