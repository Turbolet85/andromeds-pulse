//! Truthful `harness:status` — a report about the REAL pulse-app process.
//!
//! The pre-2026-08-30 form returned `ui_bridge::health::current_health()`,
//! an envelope constructed inside THIS xtask process — well-formed, exit 0,
//! with no pulse-app running at all (test-plan §3's measured hazard: a
//! status-green gate describing a different process). The verdict now
//! derives from the artifacts only a running app maintains: the registered
//! PID file (`run/andromeda-pulse.pid`, arch §Occupied Resources) and the
//! app's own rotated log family (`logs/agent-latest.jsonl*`).
//!
//! Formalized contract (per the `check:npm-supply-chain` registry shape):
//! one pretty-JSON verdict object on stdout; verdict arms
//! `running-healthy` / `stale` / `not-running` / `cannot-evaluate`;
//! exit 0 / 1 / 1 / 2. The reported `pid` is the identity a caller matches
//! against the process it spawned (test-plan §3 status-endpoint identity).

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

use anyhow::Result;
use serde_json::json;

/// Ticks emit every 15s; four missed ticks reads as a wedged or dead
/// writer. Deliberately far below the 450s drain-stall threshold — this is
/// liveness-of-writes, not drain progress.
const STALE_AFTER_SECONDS: u64 = 60;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Verdict {
    pub(crate) arm: &'static str,
    pub(crate) pid: Option<u32>,
    pub(crate) log_file_basename: Option<String>,
    pub(crate) last_write_age_seconds: Option<u64>,
}

pub(crate) fn run() -> Result<ExitCode> {
    let verdict = match resolve_paths() {
        Some((pidfile, log_base)) => {
            let pid = read_pid(&pidfile);
            let newest = newest_family_member(&log_base);
            classify(pid, newest)
        }
        None => Verdict {
            arm: "cannot-evaluate",
            pid: None,
            log_file_basename: None,
            last_write_age_seconds: None,
        },
    };

    let payload = json!({
        "verdict": verdict.arm,
        "pid": verdict.pid,
        "log_file_basename": verdict.log_file_basename,
        "last_write_age_seconds": verdict.last_write_age_seconds,
        "stale_after_seconds": STALE_AFTER_SECONDS,
    });
    println!("{}", serde_json::to_string_pretty(&payload)?);

    Ok(match verdict.arm {
        "running-healthy" => ExitCode::SUCCESS,
        "cannot-evaluate" => ExitCode::from(2),
        _ => ExitCode::FAILURE,
    })
}

/// The verdict core — pure so the arms are pinnable without a filesystem.
pub(crate) fn classify(pid: Option<u32>, newest_log: Option<(String, u64)>) -> Verdict {
    match (pid, newest_log) {
        (None, newest) => Verdict {
            arm: "not-running",
            pid: None,
            log_file_basename: newest.as_ref().map(|(b, _)| b.clone()),
            last_write_age_seconds: newest.map(|(_, age)| age),
        },
        (Some(pid), None) => Verdict {
            arm: "stale",
            pid: Some(pid),
            log_file_basename: None,
            last_write_age_seconds: None,
        },
        (Some(pid), Some((basename, age))) => Verdict {
            arm: if age <= STALE_AFTER_SECONDS {
                "running-healthy"
            } else {
                "stale"
            },
            pid: Some(pid),
            log_file_basename: Some(basename),
            last_write_age_seconds: Some(age),
        },
    }
}

/// Harness-only path resolution (trim + fall back; the vars are the
/// `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` / `_DATA_DIR` class — external
/// tool-locator carve-out, never data-dir-confined).
fn resolve_paths() -> Option<(PathBuf, PathBuf)> {
    let env_path = |name: &str| {
        std::env::var(name).ok().and_then(|v| {
            let t = v.trim();
            (!t.is_empty()).then(|| PathBuf::from(t))
        })
    };
    let data_dir = env_path("ANDROMEDA_PULSE_DATA_DIR").or_else(default_data_dir)?;
    let pidfile = env_path("ANDROMEDA_PULSE_PIDFILE")
        .unwrap_or_else(|| data_dir.join("run").join("andromeda-pulse.pid"));
    let log_base = env_path("ANDROMEDA_PULSE_LOGFILE")
        .unwrap_or_else(|| data_dir.join("logs").join("agent-latest.jsonl"));
    Some((pidfile, log_base))
}

fn default_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .ok()
            .map(|a| PathBuf::from(a).join("andromeda-pulse"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".andromeda-pulse"))
    }
}

fn read_pid(pidfile: &Path) -> Option<u32> {
    std::fs::read_to_string(pidfile)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
}

/// Newest rotated-family member by mtime — `tracing_appender`'s daily
/// roller date-suffixes the sink, so a bare-name read misses a healthy
/// boot (the same family rule as `smoke::read_jsonl_lines`).
fn newest_family_member(log_base: &Path) -> Option<(String, u64)> {
    let dir = log_base.parent()?;
    let stem = log_base.file_name()?.to_string_lossy().into_owned();
    let newest = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().starts_with(&stem))
                .unwrap_or(false)
        })
        .filter_map(|p| {
            let mtime = std::fs::metadata(&p).ok()?.modified().ok()?;
            Some((p, mtime))
        })
        .max_by_key(|(_, mtime)| *mtime)?;
    let (path, mtime) = newest;
    let age = SystemTime::now()
        .duration_since(mtime)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let basename = path.file_name()?.to_string_lossy().into_owned();
    Some((basename, age))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_pidfile_is_not_running_regardless_of_logs() {
        let v = classify(None, Some(("agent-latest.jsonl".into(), 1)));
        assert_eq!(v.arm, "not-running");
        assert_eq!(v.pid, None);
    }

    #[test]
    fn pid_without_any_log_family_is_stale() {
        let v = classify(Some(1234), None);
        assert_eq!(v.arm, "stale");
        assert_eq!(v.pid, Some(1234));
    }

    #[test]
    fn pid_with_fresh_log_writes_is_running_healthy() {
        let v = classify(
            Some(1234),
            Some(("agent-latest.jsonl.2026-08-30".into(), 3)),
        );
        assert_eq!(v.arm, "running-healthy");
        assert_eq!(v.last_write_age_seconds, Some(3));
    }

    #[test]
    fn pid_with_old_log_writes_is_stale() {
        let v = classify(
            Some(1234),
            Some(("agent-latest.jsonl".into(), STALE_AFTER_SECONDS + 1)),
        );
        assert_eq!(v.arm, "stale");
    }

    #[test]
    fn boundary_age_is_still_healthy() {
        let v = classify(
            Some(1),
            Some(("agent-latest.jsonl".into(), STALE_AFTER_SECONDS)),
        );
        assert_eq!(v.arm, "running-healthy");
    }

    #[test]
    fn newest_family_member_prefers_latest_mtime_and_reads_the_family() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let base = dir.path().join("agent-latest.jsonl");
        // ONLY a date-suffixed member exists — the family branch, not the
        // bare name.
        let dated = dir.path().join("agent-latest.jsonl.2026-08-30");
        std::fs::write(&dated, "x").expect("write");
        let (basename, _age) = newest_family_member(&base).expect("family member found");
        assert_eq!(basename, "agent-latest.jsonl.2026-08-30");
    }

    #[test]
    fn read_pid_rejects_garbage_content() {
        let dir = tempfile::TempDir::new().expect("tmp");
        let pidfile = dir.path().join("andromeda-pulse.pid");
        std::fs::write(&pidfile, "not-a-pid").expect("write");
        assert_eq!(read_pid(&pidfile), None);
        std::fs::write(&pidfile, " 4321 ").expect("write");
        assert_eq!(read_pid(&pidfile), Some(4321));
    }
}
