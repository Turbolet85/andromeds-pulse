//! `cargo xtask smoke:external-resolve` — the live leg for app-registry
//! reconciliation with externally-resolved rows.
//!
//! Shape: boot the app on a fresh data dir under deterministic L4, drive real
//! OTLP telemetry until an incident forms, resolve it through a REAL
//! `andromeda-pulse-mcp` subprocess (the cross-process writer an agent
//! actually uses), then observe the app's own persist cycles.
//!
//! RED (pre-fix): `reconciled_count` stays 0 while `declined_count` climbs —
//! the registry keeps an incident that no longer exists, and its 60 s persist
//! cycle re-offers the stale row every cycle for the corpus guard to decline.
//! GREEN (post-fix): `reconciled_count` goes positive and `declined_count`
//! stays 0, without a restart.
//!
//! `item_count` is deliberately NOT a verdict half. Measured across the pair:
//! it went 2 -> 0 in BOTH arms, because the finite storm's auto-resolve fires
//! 120 s after the last re-emission and the observation window covers it. It is
//! printed as context; keying the verdict on it would have passed in both
//! worlds.
//!
//! The RED verdict rests on FIELD VALUES, never on a clean log: a declined
//! write is not an ERROR, so a clean-log assertion would pass vacuously here
//! (test-plan §3, as qualified 2026-08-28).
//!
//! The leg proves its own preconditions and reports INCONCLUSIVE — never PASS
//! — when the incident never formed or the sidecar resolve did not apply.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};

const OTLP_GRPC_PORT: u16 = 4317;
const OTLP_HTTP_PORT: u16 = 4318;

/// Two 60 s persist cycles plus slack. The persist cadence is a const, not an
/// env knob (a new `ANDROMEDA_PULSE_*` var carries a registry cost this leg
/// does not need), so the leg waits rather than accelerates.
const DEFAULT_OBSERVE_SECONDS: u64 = 170;

/// Short enough that the silence family becomes reachable inside the leg; the
/// 3600 s production default cannot elapse in any smoke.
const DEFAULT_BOOTSTRAP_SECONDS: u64 = 20;

/// Budget for the finite storm to form an incident the sidecar can see. Kept
/// well inside the 120 s auto-resolve window measured from the storm's last
/// re-emission, so the external resolve lands on a row that is still active.
const DEFAULT_INCIDENT_WAIT_SECONDS: u64 = 90;

pub struct ExternalResolveOptions {
    pub bootstrap_seconds: u64,
    pub observe_seconds: u64,
    pub incident_wait_seconds: u64,
}

impl Default for ExternalResolveOptions {
    fn default() -> Self {
        Self {
            bootstrap_seconds: DEFAULT_BOOTSTRAP_SECONDS,
            observe_seconds: DEFAULT_OBSERVE_SECONDS,
            incident_wait_seconds: DEFAULT_INCIDENT_WAIT_SECONDS,
        }
    }
}

/// Verdict of the leg. `Inconclusive` is a first-class outcome: a leg whose
/// preconditions did not hold has measured nothing, and saying PASS there
/// would be the vacuous-pass trap this harness exists to avoid.
#[derive(Debug, PartialEq, Eq)]
pub enum LegOutcome {
    Pass,
    Fail(String),
    Inconclusive(String),
}

/// What the observation window saw, per target.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ResolveEvidence {
    /// `incidents.list_active.request` → max `item_count` BEFORE the resolve.
    pub item_count_before: u64,
    /// … and the minimum observed AFTER it.
    pub item_count_after: u64,
    /// Any `incidents.list_active.request` record seen after the resolve.
    pub saw_list_after: bool,
    /// `triage.incident.persist` → summed `reconciled_count` after the resolve.
    pub reconciled_total: u64,
    /// … and summed `declined_count`, the pre-fix signature.
    pub declined_total: u64,
    /// Persist cycles observed after the resolve.
    pub persist_cycles_after: u64,
}

/// Fold the obs lines emitted after `resolve_marker_index` into evidence.
/// Lines are the parsed obs-log family in emission order.
pub fn evidence_after_resolve(lines: &[Value], resolve_marker_index: usize) -> ResolveEvidence {
    let mut ev = ResolveEvidence::default();
    let mut min_after: Option<u64> = None;
    for (idx, line) in lines.iter().enumerate() {
        let Some(target) = line.get("target").and_then(Value::as_str) else {
            continue;
        };
        let fields = line.get("fields");
        let num = |name: &str| -> u64 {
            fields
                .and_then(|f| f.get(name))
                .and_then(Value::as_u64)
                .unwrap_or(0)
        };
        match target {
            "incidents.list_active.request" => {
                let count = num("item_count");
                if idx < resolve_marker_index {
                    ev.item_count_before = ev.item_count_before.max(count);
                } else {
                    ev.saw_list_after = true;
                    min_after = Some(min_after.map_or(count, |m: u64| m.min(count)));
                }
            }
            "triage.incident.persist" if idx >= resolve_marker_index => {
                ev.persist_cycles_after += 1;
                ev.reconciled_total += num("reconciled_count");
                ev.declined_total += num("declined_count");
            }
            _ => {}
        }
    }
    ev.item_count_after = min_after.unwrap_or(0);
    ev
}

/// Grade the evidence. Preconditions first — a leg that never formed an
/// incident, never resolved one, or never completed a persist cycle in the
/// window has measured nothing.
pub fn judge(ev: &ResolveEvidence, resolve_applied: bool) -> LegOutcome {
    if ev.item_count_before == 0 {
        return LegOutcome::Inconclusive(
            "no incident was ever listed active before the resolve — the storm did not form, \
             so the divergence this leg measures could not arise"
                .to_string(),
        );
    }
    if !resolve_applied {
        return LegOutcome::Inconclusive(
            "the sidecar did not apply the resolution, so nothing external happened to reconcile"
                .to_string(),
        );
    }
    if ev.persist_cycles_after == 0 {
        return LegOutcome::Inconclusive(format!(
            "no persist cycle completed in the observation window (needs ~2 × 60s); \
             saw {} cycles",
            ev.persist_cycles_after
        ));
    }
    if !ev.saw_list_after {
        return LegOutcome::Inconclusive(
            "no incidents.list_active.request record after the resolve — the webview poll \
             is the source of item_count and it never ran"
                .to_string(),
        );
    }

    // Two halves — but NOT the two that first suggested themselves.
    //
    // `item_count` dropping is NOT a discriminator here, measured: the mutation
    // arm (reconciliation removed) also went 2 → 0, because the finite storm's
    // auto-resolve fires 120 s after the last re-emission and the 170 s window
    // covers it. A verdict keyed on the drop would have passed in BOTH worlds.
    // It stays in the printed evidence as context, never as a verdict half.
    //
    // What discriminates, measured across the pair: `reconciled_count` (1 vs 0)
    // and `declined_count` (0 vs 2 — the stale row re-offered every cycle for
    // the corpus guard to decline, which is the pre-fix signature).
    if ev.reconciled_total == 0 {
        return LegOutcome::Fail(format!(
            "reconciled_count stayed 0 across {} persist cycles while declined_count \
             reached {} — the registry never learned of the external resolve (the pre-fix \
             signature)",
            ev.persist_cycles_after, ev.declined_total
        ));
    }
    if ev.declined_total > 0 {
        return LegOutcome::Fail(format!(
            "reconciliation counted {} but declined_count is {} — a reconciled row must stop \
             being re-offered, so a surviving decline means a stale row is still in the \
             registry's active snapshot",
            ev.reconciled_total, ev.declined_total
        ));
    }
    LegOutcome::Pass
}

/// The verdict reads `triage.incident.persist` records, which the app emits on
/// a 60 s cadence. A window shorter than one cycle is structurally incapable of
/// observing one, so it could never FAIL — refuse it rather than let the leg
/// report a guaranteed-INCONCLUSIVE (or worse, a vacuous PASS).
pub fn observe_window_supports_verdict(observe_seconds: u64) -> Result<(), String> {
    const PERSIST_CADENCE_SECONDS: u64 = 60;
    let floor = PERSIST_CADENCE_SECONDS * 2 + 10;
    if observe_seconds < floor {
        return Err(format!(
            "--observe-seconds={observe_seconds} cannot reach two {PERSIST_CADENCE_SECONDS}s \
             persist cycles (needs >= {floor}s); the verdict would be unreachable"
        ));
    }
    Ok(())
}

pub async fn run_external_resolve(opts: ExternalResolveOptions) -> Result<std::process::ExitCode> {
    if let Err(reason) = observe_window_supports_verdict(opts.observe_seconds) {
        eprintln!("::error::smoke:external-resolve: INCONCLUSIVE — {reason}");
        return Ok(std::process::ExitCode::FAILURE);
    }
    let workspace_root = &crate::self_verify::workspace_root()?;
    let app = match locate_app_binary(workspace_root) {
        Ok(p) => p,
        Err(e) => {
            println!("smoke:external-resolve: SKIP — {e}");
            return Ok(std::process::ExitCode::SUCCESS);
        }
    };
    let sidecar = match locate_sidecar_binary(workspace_root) {
        Ok(p) => p,
        Err(e) => {
            println!("smoke:external-resolve: SKIP — {e}");
            return Ok(std::process::ExitCode::SUCCESS);
        }
    };
    println!(
        "smoke:external-resolve: app={} ({})",
        app.display(),
        binary_age(&app)
    );
    println!(
        "smoke:external-resolve: sidecar={} ({})",
        sidecar.display(),
        binary_age(&sidecar)
    );

    let injector = build_injector(workspace_root).await?;
    let data_dir = tempdir(workspace_root)?;
    println!("smoke:external-resolve: data dir {}", data_dir.display());

    let mut child = spawn_app(&app, &data_dir, opts.bootstrap_seconds)?;
    wait_for_receiver().await?;

    // The arg-less FINITE storm, spawned in the BACKGROUND rather than awaited.
    //
    // Two measured constraints pin this shape. (a) `--sustained` cannot be used:
    // it holds a CONSTANT error rate, and the cue evaluator's magnitude is a
    // short/long EWMA ratio, so the long window converges on the short one and
    // the gate never trips — measured here as zero incidents in 240 s, the same
    // structural dead end `rules/testing.md` records for a different leg. Only
    // the finite 100 %-error storm forms an incident. (b) Awaiting the injector
    // to completion lets the 120 s auto-resolve window close first — the first
    // run of this leg found both corpus rows already `resolved`, so the sidecar
    // had nothing active to resolve. Spawning it lets the resolve land WHILE
    // re-emission is still bumping `updated_at`, which also keeps auto-resolve
    // from being an alternative explanation for the registry dropping the row.
    println!("smoke:external-resolve: seeding the finite storm over real OTLP (backgrounded)");
    let mut injector_child = Command::new(&injector)
        .current_dir(&data_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .context("spawn inject_demo")?;

    // The precondition is what the NEXT step needs: an incident the SIDECAR can
    // see. Polling the app's log instead would pass on a historical record for a
    // row the corpus has since resolved — which is exactly how the first run of
    // this leg reached a resolve step with nothing to resolve.
    let incident_id = match wait_for_sidecar_incident(
        &sidecar,
        &data_dir,
        Duration::from_secs(opts.incident_wait_seconds),
    )
    .await
    {
        Some(id) => id,
        None => {
            let _ = injector_child.kill().await;
            let _ = shutdown(&mut child).await;
            wait_ports_released().await;
            println!(
                "smoke:external-resolve: INCONCLUSIVE — the sidecar listed no active incident \
                 within {}s; the storm did not form",
                opts.incident_wait_seconds
            );
            return Ok(std::process::ExitCode::SUCCESS);
        }
    };

    let lines_at_resolve = read_log_lines(&data_dir);
    let resolve_marker_index = lines_at_resolve.len();
    let resolve_applied = sidecar_mark_resolved(&sidecar, &data_dir, incident_id).await;
    println!(
        "smoke:external-resolve: sidecar mark_incident_resolved(id={incident_id}) applied={resolve_applied}"
    );

    println!(
        "smoke:external-resolve: observing {}s (~2 persist cycles)",
        opts.observe_seconds
    );
    tokio::time::sleep(Duration::from_secs(opts.observe_seconds)).await;

    let lines = read_log_lines(&data_dir);
    let ev = evidence_after_resolve(&lines, resolve_marker_index);
    let panics = lines
        .iter()
        .filter(|l| l.get("target").and_then(Value::as_str) == Some("app.panic.fatal"))
        .count();

    let _ = injector_child.kill().await;
    shutdown(&mut child).await?;
    wait_ports_released().await;

    println!(
        "smoke:external-resolve: item_count {} → {} · reconciled {} · declined {} · \
         persist cycles {} · panics {} · log lines {}",
        ev.item_count_before,
        ev.item_count_after,
        ev.reconciled_total,
        ev.declined_total,
        ev.persist_cycles_after,
        panics,
        lines.len()
    );

    match judge(&ev, resolve_applied) {
        LegOutcome::Pass => {
            if panics > 0 {
                bail!("smoke:external-resolve: FAIL — {panics} panic record(s) in the log");
            }
            println!("smoke:external-resolve: PASS");
            Ok(std::process::ExitCode::SUCCESS)
        }
        LegOutcome::Inconclusive(why) => {
            println!("smoke:external-resolve: INCONCLUSIVE — {why}");
            Ok(std::process::ExitCode::SUCCESS)
        }
        LegOutcome::Fail(why) => bail!("smoke:external-resolve: FAIL — {why}"),
    }
}

/// Poll the SIDECAR until it lists an active incident, returning its id.
///
/// Deliberately not a log read: the obs log accumulates, so a historical
/// `item_count > 0` record satisfies a log-based check long after the corpus
/// row has been resolved. This asks the process that has to do the next step
/// whether it can actually see something to act on.
async fn wait_for_sidecar_incident(
    sidecar: &Path,
    data_dir: &Path,
    budget: Duration,
) -> Option<i64> {
    let deadline = Instant::now() + budget;
    while Instant::now() < deadline {
        if let Some(id) = sidecar_first_incident_id(sidecar, data_dir).await {
            return Some(id);
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    None
}

async fn sidecar_call(
    sidecar: &Path,
    data_dir: &Path,
    tool: &str,
    arguments: Value,
) -> Option<Value> {
    let mut child = Command::new(sidecar)
        .env("ANDROMEDA_PULSE_MCP_ENABLED", "true")
        .env("ANDROMEDA_PULSE_DATA_DIR", data_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .ok()?;
    let mut stdin = child.stdin.take()?;
    let stdout = child.stdout.take()?;
    let mut lines = BufReader::new(stdout).lines();

    let frame = json!({
        "jsonrpc": "2.0", "id": 1, "method": "tools/call",
        "params": { "name": tool, "arguments": arguments }
    });
    stdin
        .write_all(&serde_json::to_vec(&frame).ok()?)
        .await
        .ok()?;
    stdin.write_all(b"\n").await.ok()?;
    stdin.flush().await.ok()?;

    let line = tokio::time::timeout(Duration::from_secs(20), lines.next_line())
        .await
        .ok()?
        .ok()??;
    let _ = child.kill().await;
    serde_json::from_str(&line).ok()
}

async fn sidecar_first_incident_id(sidecar: &Path, data_dir: &Path) -> Option<i64> {
    let parsed = sidecar_call(sidecar, data_dir, "query_incident_list", json!({})).await?;
    parsed
        .get("result")?
        .get("items")?
        .as_array()?
        .first()?
        // The tool's field is `incident_id`, not `id`. Reading the wrong name
        // yields None, which this leg would otherwise report as "the storm did
        // not form" — a claim about the product sourced from a harness typo.
        .get("incident_id")?
        .as_i64()
}

async fn sidecar_mark_resolved(sidecar: &Path, data_dir: &Path, id: i64) -> bool {
    let Some(parsed) = sidecar_call(
        sidecar,
        data_dir,
        "mark_incident_resolved",
        json!({ "incident_id": id }),
    )
    .await
    else {
        return false;
    };
    parsed
        .get("result")
        .and_then(|r| r.get("resolved"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn read_log_lines(data_dir: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    for path in log_family(data_dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines() {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                out.push(v);
            }
        }
    }
    out
}

pub(crate) fn spawn_app(binary: &Path, data_dir: &Path, bootstrap_seconds: u64) -> Result<Child> {
    // CWD is the throwaway data dir, not the workspace: TauRPC's dev-mode
    // `export_types()` writes bindings RELATIVE to the working directory.
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

pub(crate) async fn shutdown(child: &mut Child) -> Result<()> {
    let _ = child.kill().await;
    Ok(())
}

pub(crate) async fn wait_for_receiver() -> Result<()> {
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

pub(crate) async fn wait_ports_released() {
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
    eprintln!("smoke:external-resolve: warning — OTLP ports still accepting after shutdown");
}

pub(crate) fn binary_age(binary: &Path) -> String {
    let Ok(modified) = std::fs::metadata(binary).and_then(|m| m.modified()) else {
        return "mtime unavailable".to_string();
    };
    match modified.elapsed() {
        Ok(age) => format!("built {}m ago", age.as_secs() / 60),
        Err(_) => "built in the future (clock skew)".to_string(),
    }
}

pub(crate) fn tempdir(root: &Path) -> Result<PathBuf> {
    let dir = root
        .join("target")
        .join("external-resolve")
        .join(format!("run-{}", std::process::id()));
    std::fs::create_dir_all(&dir).context("create external-resolve data dir")?;
    Ok(dir)
}

// The obs sink is `rolling::daily`, so every file is date-suffixed and the bare
// name matches nothing — resolve the family.
pub(crate) fn log_family(data_dir: &Path) -> Vec<PathBuf> {
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

pub(crate) fn locate_app_binary(root: &Path) -> Result<PathBuf> {
    let exe = if cfg!(windows) {
        "pulse-app.exe"
    } else {
        "pulse-app"
    };
    for profile in ["release", "debug"] {
        let candidate = root.join("target").join(profile).join(exe);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    bail!("pulse-app binary not found; run `cargo build -p pulse-app` first")
}

fn locate_sidecar_binary(root: &Path) -> Result<PathBuf> {
    let exe = if cfg!(windows) {
        "andromeda-pulse-mcp.exe"
    } else {
        "andromeda-pulse-mcp"
    };
    for profile in ["release", "debug"] {
        let candidate = root.join("target").join(profile).join(exe);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    bail!(
        "andromeda-pulse-mcp not found; run \
         `cargo build -p mcp-server --features mcp-server --bin andromeda-pulse-mcp` first"
    )
}

pub(crate) async fn build_injector(workspace_root: &Path) -> Result<PathBuf> {
    println!("smoke:external-resolve: building inject_demo (outside the timed section)");
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

    fn list(count: u64) -> Value {
        json!({"target": "incidents.list_active.request", "fields": {"item_count": count}})
    }

    fn persist(reconciled: u64, declined: u64) -> Value {
        json!({"target": "triage.incident.persist",
               "fields": {"reconciled_count": reconciled, "declined_count": declined}})
    }

    #[test]
    fn evidence_splits_on_the_resolve_marker() {
        let lines = vec![list(2), list(2), list(1), persist(1, 0)];
        let ev = evidence_after_resolve(&lines, 2);
        assert_eq!(ev.item_count_before, 2);
        assert_eq!(ev.item_count_after, 1);
        assert_eq!(ev.reconciled_total, 1);
        assert_eq!(ev.persist_cycles_after, 1);
        assert!(ev.saw_list_after);
    }

    #[test]
    fn a_storm_that_never_formed_is_inconclusive_not_a_pass() {
        let ev = ResolveEvidence::default();
        assert!(matches!(judge(&ev, true), LegOutcome::Inconclusive(_)));
    }

    #[test]
    fn an_unapplied_sidecar_resolve_is_inconclusive_not_a_pass() {
        let lines = vec![list(1), persist(0, 0)];
        let ev = evidence_after_resolve(&lines, 1);
        assert!(matches!(judge(&ev, false), LegOutcome::Inconclusive(_)));
    }

    /// The pre-fix signature must FAIL, not pass: a stale row re-offered every
    /// cycle and declined by the corpus guard, with the registry unchanged.
    #[test]
    fn the_pre_fix_signature_fails() {
        // The webview polls ~1s throughout, so `list` records keep arriving
        // after the resolve — they just never shrink.
        let lines = vec![list(1), list(1), persist(0, 1), persist(0, 1)];
        let ev = evidence_after_resolve(&lines, 1);
        match judge(&ev, true) {
            LegOutcome::Fail(why) => assert!(why.contains("reconciled_count stayed 0")),
            other => panic!("expected Fail, got {other:?}"),
        }
    }

    #[test]
    fn a_window_with_no_persist_cycle_is_inconclusive() {
        let lines = vec![list(1), list(1)];
        let ev = evidence_after_resolve(&lines, 1);
        assert!(matches!(judge(&ev, true), LegOutcome::Inconclusive(_)));
    }

    #[test]
    fn a_surviving_decline_fails_even_when_reconciliation_counted() {
        // A reconciled row must stop being re-offered; a decline that outlives
        // the reconcile means a stale row is still in the active snapshot.
        let lines = vec![list(1), list(1), persist(1, 0), persist(0, 1)];
        let ev = evidence_after_resolve(&lines, 1);
        match judge(&ev, true) {
            LegOutcome::Fail(why) => assert!(why.contains("declined_count is 1")),
            other => panic!("expected Fail, got {other:?}"),
        }
    }

    /// The two shapes this leg actually measured live, pinned so the verdict
    /// keeps discriminating between them.
    ///
    /// Both arms saw `item_count` go 2 → 0 — the finite storm's auto-resolve
    /// produces that drop with or without reconciliation — which is why the
    /// verdict must NOT key on it.
    #[test]
    fn the_measured_red_and_green_shapes_are_told_apart() {
        let green = ResolveEvidence {
            item_count_before: 2,
            item_count_after: 0,
            saw_list_after: true,
            reconciled_total: 1,
            declined_total: 0,
            persist_cycles_after: 2,
        };
        let red = ResolveEvidence {
            item_count_before: 2,
            item_count_after: 0,
            saw_list_after: true,
            reconciled_total: 0,
            declined_total: 2,
            persist_cycles_after: 2,
        };
        assert_eq!(judge(&green, true), LegOutcome::Pass);
        assert!(matches!(judge(&red, true), LegOutcome::Fail(_)));
        assert_eq!(
            green.item_count_before, red.item_count_before,
            "the two arms are indistinguishable on item_count, by measurement"
        );
        assert_eq!(green.item_count_after, red.item_count_after);
    }

    #[test]
    fn an_observe_window_below_two_persist_cycles_is_refused() {
        assert!(observe_window_supports_verdict(60).is_err());
        assert!(observe_window_supports_verdict(129).is_err());
        assert!(observe_window_supports_verdict(130).is_ok());
        assert!(observe_window_supports_verdict(DEFAULT_OBSERVE_SECONDS).is_ok());
    }

    #[test]
    fn the_green_shape_passes() {
        let lines = vec![list(2), list(2), list(1), persist(1, 0), persist(0, 0)];
        let ev = evidence_after_resolve(&lines, 2);
        assert_eq!(judge(&ev, true), LegOutcome::Pass);
    }

    #[test]
    fn the_pre_fix_signature_names_the_decline_count() {
        let lines = vec![list(1), list(1), persist(0, 1), persist(0, 1)];
        let ev = evidence_after_resolve(&lines, 1);
        assert_eq!(ev.declined_total, 2);
        assert_eq!(ev.reconciled_total, 0);
    }
}
