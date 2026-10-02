//! Dev-only L4 decision probe — measures what the REAL model decides over
//! synthetic Tier1 storm digests, one factor varied per arm. Neither a test
//! nor a gate: a real-model generation cannot give a deterministic verdict.
//!
//! ```text
//! cargo build -p pulse-app --example l4_decision_probe
//! ./target/debug/examples/l4_decision_probe[.exe] --arms A0,A1,A2,A3,A4,A5 --n 10 [--min 27] [--out DIR] [--dry-run]
//! ```
//!
//! Inputs come from the product's own guarded resolution: the hardware
//! profile picks `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH`, and
//! `ANDROMEDA_PULSE_MODEL_PATH` names the GGUF; `ANDROMEDA_PULSE_L4_ALLOW_ROOT`
//! confines both when set. Only basenames are printed. An unset or rejected
//! path exits 2 (INCONCLUSIVE) before any generation.
//!
//! Digests are rendered through the real `render_payload` + `cue_summary`
//! and composed with the real `build_primary_tier_prompt`; every string is
//! synthetic ASCII, never captured telemetry. Each generation is one
//! llama-cli spawn with production's argv (`build_llama_cli_args`), output
//! cap, wall-clock timeout and `kill_on_drop`.
//!
//! Per generation it keeps only bounded labels — the decision, the severity,
//! the model's `is_resolution_summary`, whether the producer would create an
//! incident, the first three JSON keys (the grammar's field order) and a
//! hash of the output (the determinism reading). No title, symptom,
//! hypothesis, justification or raw output is written anywhere. Rows go to
//! `{out}/runs.json`; stdout carries one summary line per arm, and with
//! `--min K` a final verdict line (exit 0 PASS / 1 FAIL).
//!
//! Arms (each differs from A0 in ONE factor):
//! - `A0` — the rendered digest, prompt and argv as the tree has them
//! - `A1` — A0 plus `--temp 0`
//! - `A2` — A0 with the `OVERALL:` line made truthful (`anomalous` when the
//!   digest carries a cue)
//! - `A3` — A0 with the cue line carrying the cue's magnitude, absolute
//!   value and persistence
//! - `A4` — A0 plus one sentence in the conventions stating when a signal
//!   warrants `surface`
//! - `A5` — A0 with `decision` / `severity` moved after `hypotheses` in the
//!   schema, both the `--json-schema-file` copy and the prompt's embedded copy
//! - `shipped` — the tree as it is, no transform (the post-fix re-measure)

use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::time::{Duration, Instant};

use interpretation::hardware::HardwareProfileDetector;
use interpretation::prompt::build_primary_tier_prompt;
use interpretation::schema::{Decision, L4_OUTPUT_JSON_SCHEMA, Severity, parse_bounded};
use pulse_app::llamacli_inference::{
    DEFAULT_MAX_TOKENS, ENV_MODEL_PATH, LLAMA_CLI_MAX_OUTPUT_BYTES, LLAMA_CLI_TIMEOUT,
    MAX_PROMPT_BYTES, binary_target_for_profile, build_llama_cli_args, extract_json_object_bounded,
    resolve_allow_root, validate_path_input, validate_prompt_bounded,
};
use serde_json::{Value, json};
use tokio::io::AsyncReadExt;
use triage::contract::{
    AttentionCue, CueKind, CueScope, DigestCueRef, DigestProjectContext, DigestServiceRow,
    HardwareProfileSource, PriorityTier, cue_summary, render_payload,
};

const ARMS: [&str; 7] = ["A0", "A1", "A2", "A3", "A4", "A5", "shipped"];
// `crate::cadence::mode_label(CadenceMode::Tier1)` — a storm cue always takes
// the Tier1 cycle, whose window is 60 s.
const TIER1_MODE_LABEL: &str = "tier1";
const TIER1_WINDOW: Duration = Duration::from_secs(60);
const WORKSPACE: &str = "/synthetic/demo-shop";
const STORM_FINGERPRINT: &str = "5e1f0a9c3b7d42e68a0c1f3e5b7d9a2c";
const A4_ANCHOR: &str = "\"watch\" (record but do not surface). ";
const A4_SENTENCE: &str = "A signal warrants \"surface\" when a service's error rate or \
latency is far above its baseline or an attention cue reports a storm. ";

struct Shape {
    id: &'static str,
    services: Vec<DigestServiceRow>,
    cue: AttentionCue,
}

struct Args {
    arms: Vec<String>,
    n: u32,
    min: Option<u32>,
    out: PathBuf,
    dry_run: bool,
}

struct Prepared {
    arm: String,
    shape: &'static str,
    prompt: String,
    schema: String,
    extra_args: Vec<String>,
}

fn row(service: &str, rate: f64, error_rate: f64, p99: f64) -> DigestServiceRow {
    DigestServiceRow {
        service: service.to_string(),
        rate_per_sec: rate,
        rate_baseline_per_sec: rate,
        error_rate,
        error_rate_baseline: error_rate,
        p99_latency_ms: p99,
        p99_baseline_ms: p99,
    }
}

fn storm_cue(service: &str, magnitude: f64, absolute_value: f64) -> AttentionCue {
    AttentionCue {
        kind: CueKind::RetryStorm,
        scope: CueScope::Service,
        scope_id: Some(service.to_string()),
        magnitude,
        absolute_value,
        persistence: 30,
        confidence: 0.95,
        priority_tier: PriorityTier::Autonomous,
        suppression_bypassed: false,
        fingerprint: Some(STORM_FINGERPRINT.to_string()),
    }
}

fn shapes() -> Vec<Shape> {
    vec![
        Shape {
            id: "S1",
            services: vec![
                row("checkout-api", 12.0, 1.0, 180.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("checkout-api", 20.0, 1.0),
        },
        Shape {
            id: "S2",
            services: vec![
                row("checkout-api", 11.0, 0.0, 170.0),
                row("payment-service", 9.0, 0.35, 110.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("payment-service", 12.0, 0.35),
        },
        Shape {
            id: "S3",
            services: vec![
                row("checkout-api", 11.0, 0.0, 120.0),
                row("payment-service", 9.0, 0.0, 110.0),
                row("inventory-service", 7.0, 0.12, 480.0),
                row("auth-service", 15.0, 0.0, 115.0),
            ],
            cue: storm_cue("inventory-service", 6.0, 0.12),
        },
    ]
}

fn project() -> DigestProjectContext {
    DigestProjectContext {
        workspace_canonical_path: WORKSPACE.to_string(),
        project_name: Some("demo-shop".to_string()),
        vcs_type: Some("git"),
        recent_commits: Vec::new(),
        framework_signals: Vec::new(),
    }
}

fn quantified_cue_summary(cue: &AttentionCue) -> String {
    format!(
        "{} magnitude={:.1}x_baseline absolute_value={:.3} persistence={}",
        cue_summary(cue),
        cue.magnitude,
        cue.absolute_value,
        cue.persistence
    )
}

fn truthful_overall(payload: &str, cue_count: usize, incident_count: usize) -> String {
    payload
        .lines()
        .map(|line| {
            if line.starts_with("OVERALL: ") {
                let word = if cue_count > 0 {
                    "anomalous"
                } else {
                    "nominal"
                };
                format!("OVERALL: {word} ({incident_count} active incident(s); {cue_count} cue(s))")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// `decision` / `severity` moved after `hypotheses` in the schema's
/// `properties`, every byte of each block kept.
fn reordered_schema() -> Result<String, String> {
    let s = L4_OUTPUT_JSON_SCHEMA;
    let cut_start = s
        .find("\n    \"decision\": {")
        .ok_or("schema: decision block not found")?;
    let cut_end = s
        .find("\n    \"title\": {")
        .ok_or("schema: title block not found")?;
    let block = &s[cut_start..cut_end];
    let rest = format!("{}{}", &s[..cut_start], &s[cut_end..]);
    let insert_at = rest
        .find("\n    \"investigation_steps\": {")
        .ok_or("schema: investigation_steps block not found")?;
    let out = format!("{}{}{}", &rest[..insert_at], block, &rest[insert_at..]);
    let parsed: Value =
        serde_json::from_str(&out).map_err(|_| "schema: reordered text is not JSON")?;
    let original: Value =
        serde_json::from_str(s).map_err(|_| "schema: embedded text is not JSON")?;
    if parsed != original {
        return Err("schema: reorder changed content".to_string());
    }
    Ok(out)
}

fn prepare(arm: &str, shape: &Shape) -> Result<Prepared, String> {
    let summary = if arm == "A3" {
        quantified_cue_summary(&shape.cue)
    } else {
        cue_summary(&shape.cue)
    };
    let cue_ref = DigestCueRef {
        kind: shape.cue.kind,
        priority_tier: shape.cue.priority_tier,
        summary,
        scope: shape.cue.scope,
        fingerprint: shape.cue.fingerprint.clone(),
        scope_id: shape.cue.scope_id.clone(),
    };
    let cues = [cue_ref];
    let mut payload = render_payload(
        TIER1_WINDOW,
        TIER1_MODE_LABEL,
        &project(),
        &shape.services,
        &cues,
        &[],
        &[],
        false,
    );
    if arm == "A2" {
        payload = truthful_overall(&payload, cues.len(), 0);
    }
    let citable = vec![STORM_FINGERPRINT.to_string()];
    let mut prompt =
        build_primary_tier_prompt(&payload, &format!("workspace={WORKSPACE}"), "", &citable);
    let mut schema = L4_OUTPUT_JSON_SCHEMA.to_string();
    let mut extra_args = Vec::new();
    match arm {
        "A1" => extra_args.extend(["--temp".to_string(), "0".to_string()]),
        "A4" => {
            if prompt.matches(A4_ANCHOR).count() != 1 {
                return Err("A4: conventions anchor not found exactly once".to_string());
            }
            prompt = prompt.replacen(A4_ANCHOR, &format!("{A4_ANCHOR}{A4_SENTENCE}"), 1);
        }
        "A5" => {
            if prompt.matches(L4_OUTPUT_JSON_SCHEMA).count() != 1 {
                return Err("A5: embedded schema not found exactly once".to_string());
            }
            schema = reordered_schema()?;
            prompt = prompt.replacen(L4_OUTPUT_JSON_SCHEMA, &schema, 1);
        }
        _ => {}
    }
    validate_prompt_bounded(&prompt)
        .map_err(|r| format!("{arm}: prompt rejected: {}", r.label()))?;
    Ok(Prepared {
        arm: arm.to_string(),
        shape: shape.id,
        prompt,
        schema,
        extra_args,
    })
}

/// The first three top-level keys of a JSON object, read off its text.
fn first_keys(json_text: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut current = String::new();
    let mut last_string: Option<String> = None;
    for c in json_text.chars() {
        if in_string {
            if escaped {
                escaped = false;
                current.push(c);
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
                last_string = Some(std::mem::take(&mut current));
            } else {
                current.push(c);
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '{' | '[' => {
                depth += 1;
                last_string = None;
            }
            '}' | ']' => depth = depth.saturating_sub(1),
            ':' if depth == 1 => {
                if let Some(key) = last_string.take() {
                    keys.push(key);
                    if keys.len() == 3 {
                        break;
                    }
                }
            }
            c if !c.is_whitespace() => last_string = None,
            _ => {}
        }
    }
    keys
}

fn decision_label(d: Decision) -> &'static str {
    match d {
        Decision::Surface => "surface",
        Decision::Dismiss => "dismiss",
        Decision::Watch => "watch",
    }
}

fn severity_label(s: Severity) -> &'static str {
    match s {
        Severity::Autonomous => "autonomous",
        Severity::Suggested => "suggested",
        Severity::Curious => "curious",
        Severity::None => "none",
    }
}

enum Spawned {
    Output(String),
    Failed(&'static str),
}

async fn generate(
    binary: &Path,
    model: &Path,
    ngl: u32,
    prepared: &Prepared,
    schema_path: &Path,
) -> Spawned {
    let mut args = build_llama_cli_args(
        model,
        ngl,
        DEFAULT_MAX_TOKENS,
        schema_path,
        &prepared.prompt,
    );
    args.extend(prepared.extra_args.iter().cloned());
    let mut cmd = tokio::process::Command::new(binary);
    cmd.args(&args)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let Ok(mut child) = cmd.spawn() else {
        return Spawned::Failed("spawn_failed");
    };
    let stdout = child.stdout.take();
    let waited = tokio::time::timeout(LLAMA_CLI_TIMEOUT, async move {
        let mut buf = Vec::new();
        if let Some(mut h) = stdout {
            let _ = h.read_to_end(&mut buf).await;
        }
        (child.wait().await, buf)
    })
    .await;
    let Ok((status, bytes)) = waited else {
        return Spawned::Failed("timeout");
    };
    if !status.map(|s| s.success()).unwrap_or(false) {
        return Spawned::Failed("exit_failure");
    }
    if bytes.len() > LLAMA_CLI_MAX_OUTPUT_BYTES {
        return Spawned::Failed("output_too_large");
    }
    match String::from_utf8(bytes) {
        Ok(text) => Spawned::Output(text),
        Err(_) => Spawned::Failed("stdout_utf8_invalid"),
    }
}

fn parse_args() -> Result<Args, String> {
    let mut arms = vec!["A0".to_string()];
    let mut n = 10;
    let mut min = None;
    let mut out = None;
    let mut dry_run = false;
    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        if flag == "--dry-run" {
            dry_run = true;
            continue;
        }
        let value = it.next().ok_or_else(|| format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--arms" => {
                arms = value.split(',').map(|a| a.trim().to_string()).collect();
                if let Some(bad) = arms.iter().find(|a| !ARMS.contains(&a.as_str())) {
                    return Err(format!("unknown arm {bad}"));
                }
            }
            "--n" => n = value.parse().map_err(|_| "--n takes a number")?,
            "--min" => min = Some(value.parse().map_err(|_| "--min takes a number")?),
            "--out" => out = Some(PathBuf::from(value)),
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let out = out.unwrap_or_else(|| {
        PathBuf::from("target/l4-decision-probe")
            .join(chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string())
    });
    Ok(Args {
        arms,
        n,
        min,
        out,
        dry_run,
    })
}

fn basename(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string())
}

fn inconclusive(why: &str) -> ExitCode {
    println!("l4-decision-probe: INCONCLUSIVE - {why}");
    ExitCode::from(2)
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(why) => return inconclusive(&why),
    };

    let mut prepared = Vec::new();
    for arm in &args.arms {
        for shape in shapes() {
            match prepare(arm, &shape) {
                Ok(p) => prepared.push(p),
                Err(why) => return inconclusive(&why),
            }
        }
    }

    if args.dry_run {
        // Composes every arm's prompt and spawns nothing: proves the transforms
        // apply and every prompt passes the production bound before a slot.
        for p in &prepared {
            println!(
                "dry-run: arm {} {}: prompt {} bytes (max {}) · schema {} bytes · extra args {}",
                p.arm,
                p.shape,
                p.prompt.len(),
                MAX_PROMPT_BYTES,
                p.schema.len(),
                if p.extra_args.is_empty() {
                    "none".to_string()
                } else {
                    p.extra_args.join(" ")
                }
            );
        }
        return ExitCode::SUCCESS;
    }

    let profile = HardwareProfileDetector::new().current_profile();
    let (bin_env, ngl, binary_kind) = binary_target_for_profile(profile);
    let allow_root = resolve_allow_root();
    let mut resolved = Vec::new();
    for env_name in [bin_env, ENV_MODEL_PATH] {
        let raw = std::env::var(env_name).unwrap_or_default();
        let raw = raw.trim();
        if raw.is_empty() {
            return inconclusive(&format!("{env_name} is unset"));
        }
        match validate_path_input(Path::new(raw), &allow_root) {
            Ok(path) => resolved.push(path),
            Err(rejection) => {
                return inconclusive(&format!("{env_name} rejected: {}", rejection.label()));
            }
        }
    }
    let (binary, model) = (resolved[0].clone(), resolved[1].clone());
    println!(
        "l4-decision-probe: binary {} ({binary_kind}, -ngl {ngl}) · model {} · arms {} · n {} per shape",
        basename(&binary),
        basename(&model),
        args.arms.join(","),
        args.n
    );

    if std::fs::create_dir_all(&args.out).is_err() {
        return inconclusive("output directory not creatable");
    }

    let mut rows: Vec<Value> = Vec::new();
    let mut total = 0u32;
    let mut total_create = 0u32;
    for arm in &args.arms {
        let started = Instant::now();
        let mut n_arm = 0u32;
        let mut create = 0u32;
        let mut decisions: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut severity_none = 0u32;
        let mut resolution_summary = 0u32;
        let mut key_orders: BTreeMap<String, u32> = BTreeMap::new();
        let mut distinct: BTreeMap<&'static str, BTreeSet<u64>> = BTreeMap::new();
        let mut per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        for p in prepared.iter().filter(|p| &p.arm == arm) {
            let schema_path = args.out.join(format!("schema-{}.json", p.arm));
            if std::fs::write(&schema_path, &p.schema).is_err() {
                return inconclusive("schema file not writable");
            }
            for run in 1..=args.n {
                let outcome = generate(&binary, &model, ngl, p, &schema_path).await;
                if let Spawned::Failed("spawn_failed") = outcome {
                    return inconclusive("llama-cli did not spawn");
                }
                let (decision, severity, is_rs, keys, digest) = match &outcome {
                    Spawned::Output(text) => match extract_json_object_bounded(text) {
                        Ok(obj) => {
                            let mut h = DefaultHasher::new();
                            obj.hash(&mut h);
                            let digest = h.finish();
                            match parse_bounded(obj.as_bytes()) {
                                Ok(out) => (
                                    decision_label(out.decision),
                                    severity_label(out.severity),
                                    out.is_resolution_summary,
                                    first_keys(obj),
                                    Some(digest),
                                ),
                                Err(_) => {
                                    ("parse_failed", "", false, first_keys(obj), Some(digest))
                                }
                            }
                        }
                        Err(_) => ("parse_failed", "", false, Vec::new(), None),
                    },
                    Spawned::Failed(why) => (*why, "", false, Vec::new(), None),
                };
                let would_create =
                    matches!(decision, "surface" | "watch") && severity != "none" && !is_rs;
                n_arm += 1;
                if would_create {
                    create += 1;
                    *per_shape.entry(p.shape).or_default() += 1;
                }
                *decisions.entry(decision).or_default() += 1;
                if severity == "none" {
                    severity_none += 1;
                }
                if is_rs {
                    resolution_summary += 1;
                }
                *key_orders.entry(keys.join(">")).or_default() += 1;
                if let Some(d) = digest {
                    distinct.entry(p.shape).or_default().insert(d);
                }
                eprintln!(
                    "l4-decision-probe: {} {} run {run}: decision {decision} severity {} would_create {would_create}",
                    p.arm,
                    p.shape,
                    if severity.is_empty() { "-" } else { severity }
                );
                rows.push(json!({
                    "arm": p.arm,
                    "shape": p.shape,
                    "run": run,
                    "decision": decision,
                    "severity": severity,
                    "is_resolution_summary": is_rs,
                    "would_create": would_create,
                    "first_keys": keys,
                    "output_hash": digest.map(|d| format!("{d:016x}")),
                }));
            }
        }
        total += n_arm;
        total_create += create;
        let count = |k: &str| decisions.get(k).copied().unwrap_or(0);
        let other: u32 = decisions
            .iter()
            .filter(|(k, _)| !matches!(**k, "surface" | "dismiss" | "watch"))
            .map(|(_, v)| *v)
            .sum();
        println!(
            "arm {arm}: would_create {create}/{n_arm} · decision {}/{}/{} · severity_none {severity_none} · resolution_summary {resolution_summary}",
            count("surface"),
            count("dismiss"),
            count("watch"),
        );
        println!(
            "  arm {arm}: per shape would_create {} · failed {other} · first_keys {} · distinct outputs {} · wall {}s",
            shapes()
                .iter()
                .map(|s| format!("{} {}", s.id, per_shape.get(s.id).copied().unwrap_or(0)))
                .collect::<Vec<_>>()
                .join(" "),
            key_orders
                .iter()
                .map(|(k, v)| format!("{}={v}", if k.is_empty() { "none" } else { k }))
                .collect::<Vec<_>>()
                .join(" "),
            shapes()
                .iter()
                .map(|s| format!(
                    "{} {}",
                    s.id,
                    distinct.get(s.id).map(|d| d.len()).unwrap_or(0)
                ))
                .collect::<Vec<_>>()
                .join(" "),
            started.elapsed().as_secs()
        );
    }

    let runs_path = args.out.join("runs.json");
    match serde_json::to_string_pretty(&rows) {
        Ok(text) if std::fs::write(&runs_path, &text).is_ok() => {}
        _ => return inconclusive("runs.json not writable"),
    }

    match args.min {
        Some(min) => {
            let pass = total_create >= min;
            println!(
                "l4-decision-probe: verdict: {} · would_create {total_create}/{total}",
                if pass { "PASS" } else { "FAIL" }
            );
            if pass {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        None => ExitCode::SUCCESS,
    }
}
