//! Dev-only L4 decision probe — measures what the REAL model decides over
//! synthetic Tier1 storm digests, one factor varied per arm. Neither a test
//! nor a gate: a real-model generation cannot give a deterministic verdict.
//!
//! ```text
//! cargo build -p pulse-app --example l4_decision_probe
//! ./target/debug/examples/l4_decision_probe[.exe] --arms A0,A1,A2,A3,A4,A5 [--shapes S1,S2,S3,S4] --n 10 [--min 27] [--min-rank1 36] [--out DIR] [--footprint] [--gbnf FILE] [--dry-run]
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
//! Each row also carries `names_trigger`, a closed label computed in-process:
//! `rank1` (the first hypothesis names the triggering cue), `elsewhere` (the
//! title, the symptom or a later hypothesis does), `none`, or `unparsed`.
//! `--min-rank1 K` adds a names-trigger verdict line over all generations;
//! with both flags set, the exit is 1 if either verdict fails. Rows and
//! summary lines also carry `names_trigger_stem`, the same reading over the
//! stem forms (`retries`, `retried`); it is recorded only and never feeds a
//! verdict.
//!
//! Footprint readings, per row and as one `footprint` summary line per arm:
//! `thinking` (`present` when the raw stdout carries b9305's
//! `[Start thinking]` marker, `absent` when not, `unread` when no stdout was
//! captured), `elapsed_ms` (spawn to reap, failed rows included),
//! `peak_rss_kib` (the child's `VmHWM` polled from `/proc/{pid}/status`,
//! Linux only) and, with `--footprint`, `peak_vram_mib` (the child's largest
//! `used_memory` polled from `nvidia-smi`). An unread value is `null` in the
//! rows and `-` in the summary.
//!
//! Shapes S1-S3 are retry storms on one service each; S4 is S1 plus one
//! corpus match (an older, active error-rate-spike incident on another
//! service), rendered through the real `format_corpus_match_line`. S5 is a
//! storm whose only abnormal metric is latency; S6 carries both an elevated
//! error rate and an elevated latency. `--shapes` selects among them
//! (default S1-S4).
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
//!   (the shipped order since prompt v2.3, so A5 now composes as A0)
//! - `nf` — the shipped composition with the three trigger-framing lines
//!   removed: the digest's TRIGGER line, its corpus framing note and the
//!   prompt's framing instruction (the no-framing counterfactual)
//! - `shipped` — the tree as it is, no transform (the post-fix re-measure)
//! - `nr` — `shipped` with the `-rea off` pair removed from the argv (does
//!   the model think when the product does not pass the switch?)
//! - `gb` — `shipped` with `--json-schema-file` swapped for
//!   `--grammar-file {--gbnf}`: b9305 prefills a thinking template's
//!   generation prompt into a schema grammar (rejected, so sampler init
//!   fails) but never into a user grammar
//! - `R1` — the framing instruction reworded to oblige the first hypothesis
//!   statement to name the TRIGGER line's signal in its own words
//! - `R3` — one conventions sentence carrying the same obligation
//! - `R2` — one sentence in the schema's `hypotheses` description carrying
//!   it, in both schema copies
//! - `R1R3` / `R1R2` / `R3R2` — the two named candidates applied together
//!
//! Each candidate composes as A0 once its text is already in the tree.

use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use interpretation::hardware::HardwareProfileDetector;
use interpretation::prompt::{TRIGGER_FRAMING_INSTRUCTION, build_primary_tier_prompt};
use interpretation::schema::{Decision, L4_OUTPUT_JSON_SCHEMA, L4Output, Severity, parse_bounded};
use pulse_app::llamacli_inference::{
    DEFAULT_MAX_TOKENS, ENV_MODEL_PATH, LLAMA_CLI_MAX_OUTPUT_BYTES, LLAMA_CLI_TIMEOUT,
    MAX_PROMPT_BYTES, binary_target_for_profile, build_llama_cli_args, extract_json_object_bounded,
    resolve_allow_root, validate_path_input, validate_prompt_bounded,
};
use serde_json::{Value, json};
use tokio::io::AsyncReadExt;
use triage::contract::{
    AttentionCue, CORPUS_MATCHES_FRAMING_NOTE, CueKind, CueScope, DigestCueRef,
    DigestProjectContext, DigestServiceRow, EvidenceRefs, HardwareProfileSource, Incident,
    IncidentStatus, PriorityTier, Severity as IncidentSeverity, TRIGGER_LINE_PREFIX, cue_summary,
    format_corpus_match_line, render_payload,
};

const ARMS: [&str; 16] = [
    "A0", "A1", "A2", "A3", "A4", "A5", "nf", "shipped", "nr", "gb", "R1", "R3", "R2", "R1R3",
    "R1R2", "R3R2",
];
// b9305 `tools/cli/cli.cpp` prints a reasoning pass to stdout between
// `[Start thinking]` and `[End thinking]`, ahead of the content.
const THINKING_MARKER: &str = "[Start thinking]";
// The only llama-cli flags `--sampling` may carry: a model's sampling values
// and its chat-template kwargs, never a path, a grammar or a model.
const SAMPLING_FLAGS: [&str; 6] = [
    "--temp",
    "--top-p",
    "--top-k",
    "--min-p",
    "--presence-penalty",
    "--chat-template-kwargs",
];
const RSS_POLL_INTERVAL: Duration = Duration::from_millis(100);
const VRAM_POLL_INTERVAL: Duration = Duration::from_millis(250);
const NVIDIA_SMI_ARGS: [&str; 2] = [
    "--query-compute-apps=pid,used_memory",
    "--format=csv,noheader,nounits",
];
const DEFAULT_SHAPES: [&str; 4] = ["S1", "S2", "S3", "S4"];
// `crate::cadence::mode_label(CadenceMode::Tier1)` — a storm cue always takes
// the Tier1 cycle, whose window is 60 s.
const TIER1_MODE_LABEL: &str = "tier1";
const TIER1_WINDOW: Duration = Duration::from_secs(60);
const WORKSPACE: &str = "/synthetic/demo-shop";
const STORM_FINGERPRINT: &str = "5e1f0a9c3b7d42e68a0c1f3e5b7d9a2c";
const CORPUS_FINGERPRINT: &str = "9c2e7b41d05a3f86e1b4c7d02a59f3e8";
// A fixed render instant keeps the corpus line's age, and so the prompt bytes,
// identical across runs.
const RENDER_NOW_UNIX_NANO: i64 = 1_700_000_000_000_000_000;
const CORPUS_MATCH_AGE_NANOS: i64 = 180_000_000_000;
const A4_ANCHOR: &str = "\"watch\" (record but do not surface). ";
const A4_SENTENCE: &str = "A signal warrants \"surface\" when a service's error rate or \
latency is far above its baseline or an attention cue reports a storm. ";
// The candidate texts are kind-generic and never name a cue kind, so a
// remedy cannot be a word plant the grader rewards.
const R1_FRAMING_INSTRUCTION: &str = "\
When the digest carries a TRIGGER line, that line names the signal this \
output describes: the title, the symptom and the first hypothesis must be \
about that signal, and the first hypothesis statement must name that signal \
in the TRIGGER line's own words. Treat any other abnormal metric on the same \
service as a cause or an effect of that signal, never as a separate first \
hypothesis. CORPUS MATCHES lines are OTHER incidents, past or still open on \
another signal, given for context only; never describe one of them as the \
current signal.";
const R3_ANCHOR: &str = "Investigation steps point to concrete checks";
const R3_CONVENTIONS_SENTENCE: &str = "When the digest carries a TRIGGER \
line, the first hypothesis names that signal in the TRIGGER line's own words. ";
const R2_ANCHOR: &str =
    "Ranked hypothesis list per P-033 (primary tier emits up to 5; fallback tier emits 1).";
const R2_SCHEMA_SENTENCE: &str = " When the digest carries a TRIGGER line, the \
first hypothesis statement names that signal in the TRIGGER line's own words.";

struct Shape {
    id: &'static str,
    services: Vec<DigestServiceRow>,
    cue: AttentionCue,
    corpus_matches: Vec<String>,
}

struct Args {
    arms: Vec<String>,
    shapes: Vec<String>,
    n: u32,
    min: Option<u32>,
    min_rank1: Option<u32>,
    out: PathBuf,
    dry_run: bool,
    footprint: bool,
    gbnf: Option<PathBuf>,
    /// Validated `--sampling` flag/value tokens, appended to every arm's argv.
    sampling: Vec<String>,
}

struct Prepared {
    arm: String,
    shape: &'static str,
    trigger: CueKind,
    prompt: String,
    schema: String,
    extra_args: Vec<String>,
    /// Flags removed from the production argv together with their value.
    drop_flags: Vec<&'static str>,
    /// The arm passes the `--gbnf` file as `--grammar-file`.
    grammar_file: bool,
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
            corpus_matches: Vec::new(),
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
            corpus_matches: Vec::new(),
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
            corpus_matches: Vec::new(),
        },
        Shape {
            id: "S4",
            services: vec![
                row("checkout-api", 12.0, 1.0, 180.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("checkout-api", 20.0, 1.0),
            corpus_matches: vec![format_corpus_match_line(
                &corpus_match_incident(),
                RENDER_NOW_UNIX_NANO,
            )],
        },
        Shape {
            id: "S5",
            services: vec![
                row("checkout-api", 12.0, 0.0, 120.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 620.0),
            ],
            cue: storm_cue("auth-service", 9.0, 0.0),
            corpus_matches: Vec::new(),
        },
        Shape {
            id: "S6",
            services: vec![
                row("checkout-api", 11.0, 0.20, 390.0),
                row("payment-service", 9.0, 0.0, 95.0),
                row("inventory-service", 7.0, 0.0, 60.0),
                row("auth-service", 15.0, 0.0, 40.0),
            ],
            cue: storm_cue("checkout-api", 10.0, 0.20),
            corpus_matches: Vec::new(),
        },
    ]
}

/// The listed shapes, in `shapes()` order whatever the list order.
fn select_shapes(ids: &[String]) -> Vec<Shape> {
    shapes()
        .into_iter()
        .filter(|s| ids.iter().any(|id| id == s.id))
        .collect()
}

/// An older, still-active incident of another kind on another service: the
/// d3 shape, where a corpus line sat in front of a retry storm.
fn corpus_match_incident() -> Incident {
    let opened = RENDER_NOW_UNIX_NANO - CORPUS_MATCH_AGE_NANOS;
    Incident {
        id: 7,
        workspace: WORKSPACE.to_string(),
        fingerprint: CORPUS_FINGERPRINT.to_string(),
        title: "Error-rate spike: Error Rate Spike in demo-shop".to_string(),
        detail: String::new(),
        kind: CueKind::ErrorRateSpike,
        scope: CueScope::Service,
        scope_id: Some("payment-service".to_string()),
        status: IncidentStatus::Active,
        severity: IncidentSeverity::Warn,
        priority_tier: PriorityTier::Suggested,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: Vec::new(),
            fingerprint_hashes: Vec::new(),
            timestamps_unix_nano: Vec::new(),
        },
        opened_at_unix_nano: opened,
        updated_at_unix_nano: opened,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        resolution_summary_text: None,
    }
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
    if cut_start > cut_end {
        // The embedded schema already lists decision / severity after the
        // analysis (prompt v2.3+), so A5's order is the shipped one.
        return Ok(s.to_string());
    }
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
        &shape.corpus_matches,
        &[],
        false,
    );
    if arm == "A2" {
        payload = truthful_overall(&payload, cues.len(), 0);
    }
    if arm == "nf" {
        payload = remove_lines(&payload, is_trigger_line, 1, "TRIGGER line")?;
        let notes = usize::from(!shape.corpus_matches.is_empty());
        payload = remove_lines(&payload, is_framing_note_line, notes, "corpus framing note")?;
    }
    let citable = vec![STORM_FINGERPRINT.to_string()];
    let mut prompt =
        build_primary_tier_prompt(&payload, &format!("workspace={WORKSPACE}"), "", &citable);
    if arm == "nf" {
        prompt = remove_lines(
            &prompt,
            is_framing_instruction_line,
            1,
            "framing instruction",
        )?;
    }
    let mut schema = L4_OUTPUT_JSON_SCHEMA.to_string();
    let mut extra_args = Vec::new();
    let mut drop_flags = Vec::new();
    match arm {
        "A1" => extra_args.extend(["--temp".to_string(), "0".to_string()]),
        "nr" => drop_flags.push("-rea"),
        "gb" => drop_flags.push("--json-schema-file"),
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
    for part in candidate_parts(arm) {
        match *part {
            "R1" => prompt = apply_r1(&prompt)?,
            "R3" => prompt = apply_r3(&prompt)?,
            "R2" => (prompt, schema) = apply_r2(&prompt, &schema)?,
            _ => {}
        }
    }
    validate_prompt_bounded(&prompt)
        .map_err(|r| format!("{arm}: prompt rejected: {}", r.label()))?;
    Ok(Prepared {
        arm: arm.to_string(),
        shape: shape.id,
        trigger: shape.cue.kind,
        prompt,
        schema,
        extra_args,
        drop_flags,
        grammar_file: arm == "gb",
    })
}

fn candidate_parts(arm: &str) -> &'static [&'static str] {
    match arm {
        "R1" => &["R1"],
        "R3" => &["R3"],
        "R2" => &["R2"],
        "R1R3" => &["R1", "R3"],
        "R1R2" => &["R1", "R2"],
        "R3R2" => &["R3", "R2"],
        _ => &[],
    }
}

/// R1: the framing instruction line replaced by the R1 text.
fn apply_r1(prompt: &str) -> Result<String, String> {
    match prompt
        .lines()
        .filter(|l| *l == R1_FRAMING_INSTRUCTION)
        .count()
    {
        1 => return Ok(prompt.to_string()),
        0 => {}
        hits => return Err(format!("R1: text found {hits} times")),
    }
    let hits = count_matching_lines(prompt, is_framing_instruction_line);
    if hits != 1 {
        return Err(format!(
            "R1: framing instruction found {hits} times, expected 1"
        ));
    }
    Ok(prompt
        .split_inclusive('\n')
        .map(|l| {
            let body = l.trim_end_matches('\n');
            if is_framing_instruction_line(body) {
                format!("{R1_FRAMING_INSTRUCTION}{}", &l[body.len()..])
            } else {
                l.to_string()
            }
        })
        .collect())
}

/// R3: the conventions sentence inserted before the investigation-steps one.
fn apply_r3(prompt: &str) -> Result<String, String> {
    match prompt.matches(R3_CONVENTIONS_SENTENCE).count() {
        1 => return Ok(prompt.to_string()),
        0 => {}
        hits => return Err(format!("R3: text found {hits} times")),
    }
    if prompt.matches(R3_ANCHOR).count() != 1 {
        return Err("R3: conventions anchor not found exactly once".to_string());
    }
    Ok(prompt.replacen(
        R3_ANCHOR,
        &format!("{R3_CONVENTIONS_SENTENCE}{R3_ANCHOR}"),
        1,
    ))
}

/// The schema with R2's sentence appended to the `hypotheses` description,
/// checked to differ from `schema` in that one string only.
fn r2_schema(schema: &str) -> Result<String, String> {
    let extended = format!("{R2_ANCHOR}{R2_SCHEMA_SENTENCE}");
    match schema.matches(&extended).count() {
        1 => return Ok(schema.to_string()),
        0 => {}
        hits => return Err(format!("R2: text found {hits} times")),
    }
    if schema.matches(R2_ANCHOR).count() != 1 {
        return Err("R2: hypotheses description not found exactly once".to_string());
    }
    let out = schema.replacen(R2_ANCHOR, &extended, 1);
    let mut parsed: Value =
        serde_json::from_str(&out).map_err(|_| "R2: edited schema is not JSON")?;
    let original: Value = serde_json::from_str(schema).map_err(|_| "R2: schema is not JSON")?;
    let description = parsed
        .pointer_mut("/properties/hypotheses/description")
        .ok_or("R2: hypotheses description not at its path")?;
    if *description != Value::String(extended) {
        return Err("R2: the sentence landed outside the hypotheses description".to_string());
    }
    *description = Value::String(R2_ANCHOR.to_string());
    if parsed != original {
        return Err("R2: edit changed more than the hypotheses description".to_string());
    }
    Ok(out)
}

/// R2: both schema copies carry the sentence — the `--json-schema-file` one
/// and the one embedded in the prompt (the A5 two-copy discipline).
fn apply_r2(prompt: &str, schema: &str) -> Result<(String, String), String> {
    let edited = r2_schema(schema)?;
    if edited == schema {
        return Ok((prompt.to_string(), edited));
    }
    if prompt.matches(schema).count() != 1 {
        return Err("R2: embedded schema not found exactly once".to_string());
    }
    Ok((prompt.replacen(schema, &edited, 1), edited))
}

fn count_matching_lines(text: &str, target: fn(&str) -> bool) -> usize {
    text.lines().filter(|l| target(l)).count()
}

fn is_trigger_line(line: &str) -> bool {
    line.starts_with(TRIGGER_LINE_PREFIX)
}

fn is_framing_note_line(line: &str) -> bool {
    line.strip_prefix("  ") == Some(CORPUS_MATCHES_FRAMING_NOTE)
}

fn is_framing_instruction_line(line: &str) -> bool {
    line == TRIGGER_FRAMING_INSTRUCTION
}

/// Drops every whole line matching `target`, after checking it occurs exactly
/// `expected` times (the A4/A5 exactly-once transform discipline).
fn remove_lines(
    text: &str,
    target: fn(&str) -> bool,
    expected: usize,
    what: &str,
) -> Result<String, String> {
    let hits = text.lines().filter(|l| target(l)).count();
    if hits != expected {
        return Err(format!(
            "nf: {what} found {hits} times, expected {expected}"
        ));
    }
    Ok(text
        .split_inclusive('\n')
        .filter(|l| !target(l.trim_end_matches('\n')))
        .collect())
}

/// Case-insensitive ASCII terms that name each cue kind in model text.
fn trigger_terms(kind: CueKind) -> &'static [&'static str] {
    match kind {
        CueKind::RetryStorm => &["retry"],
        CueKind::ErrorRateSpike => &["error rate", "error-rate"],
        CueKind::LatencyRegression => &["latency"],
        CueKind::RestartEvent => &["restart"],
        CueKind::ServiceWentSilent => &["silent", "silence"],
        CueKind::ReflectionTrend => &["trend"],
    }
}

/// Whether a parsed interpretation names its triggering cue: in the rank-1
/// hypothesis (`rank1`), only in the title, the symptom or a later hypothesis
/// (`elsewhere`), or nowhere (`none`). Reads model text in-process only.
fn names_trigger(output: &L4Output, kind: CueKind) -> &'static str {
    let terms = trigger_terms(kind);
    let names = |text: &str| {
        let lower = text.to_ascii_lowercase();
        terms.iter().any(|t| lower.contains(t))
    };
    if output
        .hypotheses
        .first()
        .is_some_and(|h| names(&h.statement))
    {
        return "rank1";
    }
    let later = output
        .hypotheses
        .iter()
        .skip(1)
        .any(|h| names(&h.statement));
    if names(&output.title) || names(&output.symptom) || later {
        "elsewhere"
    } else {
        "none"
    }
}

fn names_trigger_label(parsed: Option<&L4Output>, kind: CueKind) -> &'static str {
    parsed.map_or("unparsed", |out| names_trigger(out, kind))
}

/// `trigger_terms` widened to the stem forms a substring `retry` misses
/// (`retrying` and `retry_storm` already contain it).
fn stem_terms(kind: CueKind) -> &'static [&'static str] {
    match kind {
        CueKind::RetryStorm => &["retry", "retries", "retried"],
        other => trigger_terms(other),
    }
}

/// `names_trigger` over `stem_terms`: a record-only reading that never
/// feeds a verdict.
fn names_trigger_stem(output: &L4Output, kind: CueKind) -> &'static str {
    let terms = stem_terms(kind);
    let names = |text: &str| {
        let lower = text.to_ascii_lowercase();
        terms.iter().any(|t| lower.contains(t))
    };
    if output
        .hypotheses
        .first()
        .is_some_and(|h| names(&h.statement))
    {
        return "rank1";
    }
    let later = output
        .hypotheses
        .iter()
        .skip(1)
        .any(|h| names(&h.statement));
    if names(&output.title) || names(&output.symptom) || later {
        "elsewhere"
    } else {
        "none"
    }
}

fn names_trigger_stem_label(parsed: Option<&L4Output>, kind: CueKind) -> &'static str {
    parsed.map_or("unparsed", |out| names_trigger_stem(out, kind))
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

/// Footprint readings of one generation; `None` is an unread value.
#[derive(Clone, Copy)]
struct Metrics {
    elapsed_ms: u64,
    peak_rss_kib: Option<u64>,
    peak_vram_mib: Option<u64>,
}

enum Spawned {
    Output(String, Metrics),
    Failed(&'static str, Metrics),
}

/// `present` when the raw stdout carries the b9305 thinking marker, `absent`
/// when it does not, `unread` when no stdout was captured.
fn thinking_label(stdout: Option<&str>) -> &'static str {
    match stdout {
        Some(text) if text.contains(THINKING_MARKER) => "present",
        Some(_) => "absent",
        None => "unread",
    }
}

/// The `VmHWM` (peak resident set) value of a `/proc/{pid}/status` text, in KiB.
fn parse_vm_hwm_kib(status: &str) -> Option<u64> {
    status
        .lines()
        .find_map(|l| l.strip_prefix("VmHWM:"))
        .and_then(|v| v.trim().strip_suffix("kB"))
        .and_then(|v| v.trim().parse().ok())
}

/// The largest `used_memory` (MiB) listed for `pid` in
/// `nvidia-smi --query-compute-apps=pid,used_memory --format=csv,noheader,nounits`.
fn parse_vram_mib(csv: &str, pid: u32) -> Option<u64> {
    csv.lines()
        .filter_map(|l| l.split_once(','))
        .filter(|(p, _)| p.trim().parse::<u32>().ok() == Some(pid))
        .filter_map(|(_, used)| used.trim().parse::<u64>().ok())
        .max()
}

fn read_vm_hwm_kib(pid: u32) -> Option<u64> {
    if cfg!(target_os = "linux") {
        std::fs::read_to_string(format!("/proc/{pid}/status"))
            .ok()
            .as_deref()
            .and_then(parse_vm_hwm_kib)
    } else {
        None
    }
}

async fn read_vram_mib(pid: u32) -> Option<u64> {
    let out = tokio::process::Command::new("nvidia-smi")
        .args(NVIDIA_SMI_ARGS)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .await
        .ok()?;
    parse_vram_mib(std::str::from_utf8(&out.stdout).ok()?, pid)
}

fn record_peak(peak: &Mutex<Option<u64>>, value: Option<u64>) {
    if let (Some(v), Ok(mut p)) = (value, peak.lock()) {
        *p = Some(p.map_or(v, |old| old.max(v)));
    }
}

fn read_peak(peak: &Mutex<Option<u64>>) -> Option<u64> {
    peak.lock().ok().and_then(|p| *p)
}

/// The production argv for one generation, with the arm's dropped flags
/// (each with its value) removed and its extra args appended; a
/// `grammar_file` arm also appends `--grammar-file {gbnf}`.
fn compose_argv(
    prepared: &Prepared,
    model: &Path,
    ngl: u32,
    schema_path: &Path,
    gbnf: Option<&Path>,
) -> Vec<String> {
    let mut args = build_llama_cli_args(
        model,
        ngl,
        DEFAULT_MAX_TOKENS,
        schema_path,
        &prepared.prompt,
    );
    for flag in &prepared.drop_flags {
        if let Some(at) = args.iter().position(|a| a == flag) {
            let end = (at + 2).min(args.len());
            args.drain(at..end);
        }
    }
    args.extend(prepared.extra_args.iter().cloned());
    if let (true, Some(gbnf)) = (prepared.grammar_file, gbnf) {
        args.extend([
            "--grammar-file".to_string(),
            gbnf.to_string_lossy().into_owned(),
        ]);
    }
    args
}

async fn generate(
    binary: &Path,
    model: &Path,
    ngl: u32,
    prepared: &Prepared,
    schema_path: &Path,
    gbnf: Option<&Path>,
    footprint: bool,
) -> Spawned {
    let args = compose_argv(prepared, model, ngl, schema_path, gbnf);
    let mut cmd = tokio::process::Command::new(binary);
    cmd.args(&args)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let started = Instant::now();
    let unread = |started: Instant| Metrics {
        elapsed_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        peak_rss_kib: None,
        peak_vram_mib: None,
    };
    let Ok(mut child) = cmd.spawn() else {
        return Spawned::Failed("spawn_failed", unread(started));
    };
    let rss_peak = Arc::new(Mutex::new(None));
    let vram_peak = Arc::new(Mutex::new(None));
    let mut pollers = Vec::new();
    if let Some(pid) = child.id() {
        let peak = Arc::clone(&rss_peak);
        pollers.push(tokio::spawn(async move {
            loop {
                record_peak(&peak, read_vm_hwm_kib(pid));
                tokio::time::sleep(RSS_POLL_INTERVAL).await;
            }
        }));
        if footprint {
            let peak = Arc::clone(&vram_peak);
            pollers.push(tokio::spawn(async move {
                loop {
                    record_peak(&peak, read_vram_mib(pid).await);
                    tokio::time::sleep(VRAM_POLL_INTERVAL).await;
                }
            }));
        }
    }
    let stdout = child.stdout.take();
    let waited = tokio::time::timeout(LLAMA_CLI_TIMEOUT, async move {
        let mut buf = Vec::new();
        if let Some(mut h) = stdout {
            let _ = h.read_to_end(&mut buf).await;
        }
        (child.wait().await, buf)
    })
    .await;
    for poller in &pollers {
        poller.abort();
    }
    let metrics = Metrics {
        peak_rss_kib: read_peak(&rss_peak),
        peak_vram_mib: read_peak(&vram_peak),
        ..unread(started)
    };
    let Ok((status, bytes)) = waited else {
        return Spawned::Failed("timeout", metrics);
    };
    if !status.map(|s| s.success()).unwrap_or(false) {
        return Spawned::Failed("exit_failure", metrics);
    }
    if bytes.len() > LLAMA_CLI_MAX_OUTPUT_BYTES {
        return Spawned::Failed("output_too_large", metrics);
    }
    match String::from_utf8(bytes) {
        Ok(text) => Spawned::Output(text, metrics),
        Err(_) => Spawned::Failed("stdout_utf8_invalid", metrics),
    }
}

/// Nearest-rank median.
fn p50(values: &[u64]) -> Option<u64> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let rank = sorted.len().div_ceil(2);
    rank.checked_sub(1).and_then(|i| sorted.get(i).copied())
}

fn reading(value: Option<u64>) -> String {
    value.map_or_else(|| "-".to_string(), |v| v.to_string())
}

/// `--sampling '--temp 0.7 --top-p 0.8 …'`: whitespace-separated flag/value
/// pairs, each flag in `SAMPLING_FLAGS`, each value a finite number except
/// `--chat-template-kwargs`, whose value is a JSON object.
fn parse_sampling(value: &str) -> Result<Vec<String>, String> {
    let tokens: Vec<&str> = value.split_whitespace().collect();
    if !tokens.len().is_multiple_of(2) {
        return Err("--sampling takes flag value pairs".to_string());
    }
    let mut out = Vec::new();
    for pair in tokens.chunks(2) {
        let (flag, v) = (pair[0], pair[1]);
        if !SAMPLING_FLAGS.contains(&flag) {
            return Err(format!("--sampling: unsupported flag {flag}"));
        }
        let ok = if flag == "--chat-template-kwargs" {
            serde_json::from_str::<Value>(v).is_ok_and(|j| j.is_object())
        } else {
            v.parse::<f64>().is_ok_and(f64::is_finite)
        };
        if !ok {
            return Err(format!("--sampling: bad value for {flag}"));
        }
        out.extend([flag.to_string(), v.to_string()]);
    }
    Ok(out)
}

fn parse_args() -> Result<Args, String> {
    parse_args_from(std::env::args().skip(1))
}

fn parse_args_from(argv: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut arms = vec!["A0".to_string()];
    let mut shape_ids: Vec<String> = DEFAULT_SHAPES.iter().map(|s| s.to_string()).collect();
    let mut n = 10;
    let mut min = None;
    let mut min_rank1 = None;
    let mut out = None;
    let mut dry_run = false;
    let mut footprint = false;
    let mut gbnf = None;
    let mut sampling = Vec::new();
    let mut it = argv.into_iter();
    while let Some(flag) = it.next() {
        if flag == "--dry-run" {
            dry_run = true;
            continue;
        }
        if flag == "--footprint" {
            footprint = true;
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
            "--shapes" => {
                shape_ids = value.split(',').map(|s| s.trim().to_string()).collect();
                let known = shapes();
                if let Some(bad) = shape_ids
                    .iter()
                    .find(|id| !known.iter().any(|s| s.id == id.as_str()))
                {
                    return Err(format!("unknown shape {bad}"));
                }
            }
            "--n" => n = value.parse().map_err(|_| "--n takes a number")?,
            "--min" => min = Some(value.parse().map_err(|_| "--min takes a number")?),
            "--min-rank1" => {
                min_rank1 = Some(value.parse().map_err(|_| "--min-rank1 takes a number")?)
            }
            "--out" => out = Some(PathBuf::from(value)),
            "--gbnf" => gbnf = Some(PathBuf::from(value)),
            "--sampling" => sampling = parse_sampling(&value)?,
            other => return Err(format!("unknown flag {other}")),
        }
    }
    let out = out.unwrap_or_else(|| {
        PathBuf::from("target/l4-decision-probe")
            .join(chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string())
    });
    Ok(Args {
        arms,
        shapes: shape_ids,
        n,
        min,
        min_rank1,
        out,
        dry_run,
        footprint,
        gbnf,
        sampling,
    })
}

/// `S1 9 S2 6 …` over the selected shapes, in `shapes()` order.
fn per_shape_counts(selected: &[Shape], count: impl Fn(&str) -> usize) -> String {
    selected
        .iter()
        .map(|s| format!("{} {}", s.id, count(s.id)))
        .collect::<Vec<_>>()
        .join(" ")
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

    let selected = select_shapes(&args.shapes);
    let mut prepared = Vec::new();
    for arm in &args.arms {
        for shape in &selected {
            match prepare(arm, shape) {
                Ok(p) => prepared.push(p),
                Err(why) => return inconclusive(&why),
            }
        }
    }

    for p in &mut prepared {
        p.extra_args.extend(args.sampling.iter().cloned());
    }

    if args.dry_run {
        // Composes every arm's prompt and spawns nothing: proves the transforms
        // apply and every prompt passes the production bound before a slot.
        for p in &prepared {
            let listed = |items: Vec<String>| {
                if items.is_empty() {
                    "none".to_string()
                } else {
                    items.join(" ")
                }
            };
            println!(
                "dry-run: arm {} {}: prompt {} bytes (max {}) · schema {} bytes · extra args {} · dropped args {}",
                p.arm,
                p.shape,
                p.prompt.len(),
                MAX_PROMPT_BYTES,
                p.schema.len(),
                listed(p.extra_args.clone()),
                listed(p.drop_flags.iter().map(|f| f.to_string()).collect()),
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
    if prepared.iter().any(|p| p.grammar_file) {
        match args.gbnf.as_deref() {
            Some(g) if g.is_file() => println!("l4-decision-probe: gbnf {}", basename(g)),
            Some(_) => return inconclusive("--gbnf is not a file"),
            None => return inconclusive("arm gb needs --gbnf"),
        }
    }
    println!(
        "l4-decision-probe: sampling {}",
        if args.sampling.is_empty() {
            "default".to_string()
        } else {
            args.sampling.join(" ")
        }
    );
    println!(
        "l4-decision-probe: binary {} ({binary_kind}, -ngl {ngl}) · model {} · arms {} · shapes {} · n {} per shape",
        basename(&binary),
        basename(&model),
        args.arms.join(","),
        selected.iter().map(|s| s.id).collect::<Vec<_>>().join(","),
        args.n
    );

    if std::fs::create_dir_all(&args.out).is_err() {
        return inconclusive("output directory not creatable");
    }

    let mut rows: Vec<Value> = Vec::new();
    let mut total = 0u32;
    let mut total_create = 0u32;
    let mut total_rank1 = 0u32;
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
        let mut names_counts: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut rank1_per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut stem_counts: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut stem_rank1_per_shape: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut thinking_present = 0u32;
        let mut elapsed: Vec<u64> = Vec::new();
        let mut rss_max: Option<u64> = None;
        let mut vram_max: Option<u64> = None;
        for p in prepared.iter().filter(|p| &p.arm == arm) {
            let schema_path = args.out.join(format!("schema-{}.json", p.arm));
            if std::fs::write(&schema_path, &p.schema).is_err() {
                return inconclusive("schema file not writable");
            }
            for run in 1..=args.n {
                let outcome = generate(
                    &binary,
                    &model,
                    ngl,
                    p,
                    &schema_path,
                    args.gbnf.as_deref(),
                    args.footprint,
                )
                .await;
                if let Spawned::Failed("spawn_failed", _) = outcome {
                    return inconclusive("llama-cli did not spawn");
                }
                let (thinking, metrics) = match &outcome {
                    Spawned::Output(text, m) => (thinking_label(Some(text)), *m),
                    Spawned::Failed(_, m) => (thinking_label(None), *m),
                };
                if thinking == "present" {
                    thinking_present += 1;
                }
                elapsed.push(metrics.elapsed_ms);
                rss_max = rss_max.max(metrics.peak_rss_kib);
                vram_max = vram_max.max(metrics.peak_vram_mib);
                let mut names = names_trigger_label(None, p.trigger);
                let mut stem = names_trigger_stem_label(None, p.trigger);
                let (decision, severity, is_rs, keys, digest) = match &outcome {
                    Spawned::Output(text, _) => match extract_json_object_bounded(text) {
                        Ok(obj) => {
                            let mut h = DefaultHasher::new();
                            obj.hash(&mut h);
                            let digest = h.finish();
                            match parse_bounded(obj.as_bytes()) {
                                Ok(out) => {
                                    names = names_trigger_label(Some(&out), p.trigger);
                                    stem = names_trigger_stem_label(Some(&out), p.trigger);
                                    (
                                        decision_label(out.decision),
                                        severity_label(out.severity),
                                        out.is_resolution_summary,
                                        first_keys(obj),
                                        Some(digest),
                                    )
                                }
                                Err(_) => {
                                    ("parse_failed", "", false, first_keys(obj), Some(digest))
                                }
                            }
                        }
                        Err(_) => ("parse_failed", "", false, Vec::new(), None),
                    },
                    Spawned::Failed(why, _) => (*why, "", false, Vec::new(), None),
                };
                *names_counts.entry(names).or_default() += 1;
                if names == "rank1" {
                    *rank1_per_shape.entry(p.shape).or_default() += 1;
                }
                *stem_counts.entry(stem).or_default() += 1;
                if stem == "rank1" {
                    *stem_rank1_per_shape.entry(p.shape).or_default() += 1;
                }
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
                    "l4-decision-probe: {} {} run {run}: decision {decision} severity {} would_create {would_create} names_trigger {names} names_trigger_stem {stem} thinking {thinking} elapsed_ms {}",
                    p.arm,
                    p.shape,
                    if severity.is_empty() { "-" } else { severity },
                    metrics.elapsed_ms
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
                    "names_trigger": names,
                    "names_trigger_stem": stem,
                    "thinking": thinking,
                    "elapsed_ms": metrics.elapsed_ms,
                    "peak_rss_kib": metrics.peak_rss_kib,
                    "peak_vram_mib": metrics.peak_vram_mib,
                }));
            }
        }
        total += n_arm;
        total_create += create;
        let names_count = |k: &str| names_counts.get(k).copied().unwrap_or(0);
        total_rank1 += names_count("rank1");
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
            per_shape_counts(&selected, |id| per_shape.get(id).copied().unwrap_or(0)
                as usize),
            key_orders
                .iter()
                .map(|(k, v)| format!("{}={v}", if k.is_empty() { "none" } else { k }))
                .collect::<Vec<_>>()
                .join(" "),
            per_shape_counts(&selected, |id| distinct
                .get(id)
                .map(|d| d.len())
                .unwrap_or(0)),
            started.elapsed().as_secs()
        );
        println!(
            "  arm {arm}: names_trigger rank1 {}/{n_arm} · elsewhere {} · none {} · unparsed {} · per shape rank1 {}",
            names_count("rank1"),
            names_count("elsewhere"),
            names_count("none"),
            names_count("unparsed"),
            per_shape_counts(
                &selected,
                |id| rank1_per_shape.get(id).copied().unwrap_or(0) as usize
            ),
        );
        let stem_count = |k: &str| stem_counts.get(k).copied().unwrap_or(0);
        println!(
            "  arm {arm}: names_trigger_stem rank1 {}/{n_arm} · elsewhere {} · none {} · unparsed {} · per shape rank1 {}",
            stem_count("rank1"),
            stem_count("elsewhere"),
            stem_count("none"),
            stem_count("unparsed"),
            per_shape_counts(
                &selected,
                |id| stem_rank1_per_shape.get(id).copied().unwrap_or(0) as usize
            ),
        );
        println!(
            "  arm {arm}: footprint thinking present {thinking_present}/{n_arm} · elapsed_ms p50 {} max {} · peak_rss_kib max {} · peak_vram_mib max {}",
            reading(p50(&elapsed)),
            reading(elapsed.iter().copied().max()),
            reading(rss_max),
            reading(vram_max),
        );
    }

    let runs_path = args.out.join("runs.json");
    match serde_json::to_string_pretty(&rows) {
        Ok(text) if std::fs::write(&runs_path, &text).is_ok() => {}
        _ => return inconclusive("runs.json not writable"),
    }

    let verdict = |pass: bool| if pass { "PASS" } else { "FAIL" };
    let mut failed = false;
    if let Some(min) = args.min {
        let pass = total_create >= min;
        failed |= !pass;
        println!(
            "l4-decision-probe: verdict: {} · would_create {total_create}/{total}",
            verdict(pass)
        );
    }
    if let Some(min_rank1) = args.min_rank1 {
        let pass = total_rank1 >= min_rank1;
        failed |= !pass;
        println!(
            "l4-decision-probe: names-trigger verdict: {} · rank1 {total_rank1}/{total}",
            verdict(pass)
        );
    }
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interpretation::schema::{Confidence, Hypothesis};

    type LineTarget = fn(&str) -> bool;

    fn output(title: &str, symptom: &str, statements: &[&str]) -> L4Output {
        L4Output {
            schema_version: "2.0".to_string(),
            prompt_version: "v2.4".to_string(),
            decision: Decision::Surface,
            severity: Severity::Suggested,
            title: title.to_string(),
            symptom: symptom.to_string(),
            timeline: "Started 2m ago".to_string(),
            hypotheses: statements
                .iter()
                .map(|s| Hypothesis {
                    statement: s.to_string(),
                    confidence: Confidence::High,
                    justification: "observed".to_string(),
                })
                .collect(),
            investigation_steps: Vec::new(),
            evidence_refs: Vec::new(),
            fingerprint: String::new(),
            model_tier: "primary".to_string(),
            hardware_profile: "gpu".to_string(),
            is_resolution_summary: false,
        }
    }

    fn shape(id: &str) -> Shape {
        shapes()
            .into_iter()
            .find(|s| s.id == id)
            .unwrap_or_else(|| panic!("shape {id} exists"))
    }

    fn count_lines(text: &str, target: LineTarget) -> usize {
        text.lines().filter(|l| target(l)).count()
    }

    #[test]
    fn names_trigger_reads_rank1_when_the_first_hypothesis_names_the_retry() {
        let out = output(
            "Retry storm on checkout-api",
            "Clients retry checkout-api calls",
            &["Clients retry failed checkout-api calls in a tight loop"],
        );
        assert_eq!(names_trigger(&out, CueKind::RetryStorm), "rank1");
    }

    #[test]
    fn names_trigger_reads_elsewhere_when_only_the_title_or_symptom_names_it() {
        let cases = [
            (
                "title-only",
                output("Retry storm", "Errors rose", &["A deploy broke checkout"]),
            ),
            (
                "symptom-only",
                output(
                    "Checkout errors",
                    "Clients retry",
                    &["A deploy broke checkout"],
                ),
            ),
            (
                "second-hypothesis-only",
                output(
                    "Checkout errors",
                    "Errors rose",
                    &[
                        "A deploy broke checkout",
                        "A client retry loop amplifies it",
                    ],
                ),
            ),
        ];
        for (case, out) in cases {
            assert_eq!(
                names_trigger(&out, CueKind::RetryStorm),
                "elsewhere",
                "{case}"
            );
        }
    }

    #[test]
    fn names_trigger_reads_none_when_nothing_names_it() {
        let out = output(
            "Error Rate Spike in demo-shop",
            "payment-service errors rose",
            &["A deploy broke payment-service"],
        );
        assert_eq!(names_trigger(&out, CueKind::RetryStorm), "none");
    }

    #[test]
    fn names_trigger_matches_case_insensitively() {
        let out = output("x", "y", &["Clients RETRY failed checkout-api calls"]);
        assert_eq!(names_trigger(&out, CueKind::RetryStorm), "rank1");
    }

    #[test]
    fn names_trigger_label_set_is_exactly_the_closed_set() {
        const CLOSED: [&str; 4] = ["rank1", "elsewhere", "none", "unparsed"];
        let outputs = [
            output("x", "y", &["retry"]),
            output("retry", "y", &["z"]),
            output("x", "y", &["z"]),
            output("x", "y", &[]),
        ];
        let mut seen = BTreeSet::new();
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RestartEvent,
            CueKind::ServiceWentSilent,
            CueKind::RetryStorm,
            CueKind::ReflectionTrend,
        ] {
            seen.insert(names_trigger_label(None, kind));
            for out in &outputs {
                seen.insert(names_trigger_label(Some(out), kind));
            }
        }
        assert_eq!(seen, CLOSED.into_iter().collect::<BTreeSet<_>>());
    }

    #[test]
    fn nf_arm_strips_exactly_the_three_framing_lines() {
        let s4 = shape("S4");
        let shipped = prepare("shipped", &s4).expect("shipped composes").prompt;
        let nf = prepare("nf", &s4).expect("nf composes").prompt;
        let targets: [(&str, LineTarget); 3] = [
            ("TRIGGER line", is_trigger_line),
            ("framing note", is_framing_note_line),
            ("framing instruction", is_framing_instruction_line),
        ];
        let mut stripped = shipped.clone();
        for (what, target) in targets {
            assert_eq!(
                count_lines(&shipped, target),
                1,
                "shipped carries the {what}"
            );
            assert_eq!(count_lines(&nf, target), 0, "nf carries no {what}");
            stripped = stripped
                .split_inclusive('\n')
                .filter(|l| !target(l.trim_end_matches('\n')))
                .collect();
        }
        assert_eq!(
            stripped, nf,
            "nf is shipped minus exactly those three lines"
        );
    }

    #[test]
    fn s4_renders_a_framed_corpus_match_beside_the_trigger() {
        let prompt = prepare("shipped", &shape("S4"))
            .expect("shipped composes")
            .prompt;
        let lines: Vec<&str> = prompt.lines().collect();
        assert!(lines.contains(&"TRIGGER: Retry storm"));
        let note = format!("  {CORPUS_MATCHES_FRAMING_NOTE}");
        let at = lines
            .iter()
            .position(|l| *l == note)
            .expect("the corpus framing note renders");
        let next = lines.get(at + 1).copied().unwrap_or_default();
        assert!(
            next.starts_with("  - [") && next.contains("Error-rate spike:"),
            "a corpus match line follows the note: {next}"
        );
    }

    #[test]
    fn every_arm_and_shape_composes_within_the_production_bound() {
        let ids: Vec<&str> = shapes().iter().map(|s| s.id).collect();
        assert_eq!(ids, ["S1", "S2", "S3", "S4", "S5", "S6"]);
        for arm in ARMS {
            for s in shapes() {
                let prepared = prepare(arm, &s);
                assert!(prepared.is_ok(), "{arm} {}: {:?}", s.id, prepared.err());
                let bytes = prepared.map(|p| p.prompt.len()).unwrap_or_default();
                assert!(bytes <= MAX_PROMPT_BYTES, "{arm} {}: {bytes} bytes", s.id);
            }
        }
    }

    fn argv(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    fn selected_ids(args: &Args) -> Vec<&'static str> {
        select_shapes(&args.shapes).iter().map(|s| s.id).collect()
    }

    #[test]
    fn shapes_default_is_s1_through_s4() {
        let Ok(args) = parse_args_from(argv(&[])) else {
            panic!("no flags parse");
        };
        assert_eq!(selected_ids(&args), ["S1", "S2", "S3", "S4"]);
    }

    #[test]
    fn shapes_flag_selects_the_listed_shapes_in_shapes_order() {
        let Ok(args) = parse_args_from(argv(&["--shapes", "S6,S5", "--n", "3"])) else {
            panic!("--shapes S6,S5 parses");
        };
        assert_eq!(selected_ids(&args), ["S5", "S6"]);
        assert_eq!(args.n, 3);
    }

    #[test]
    fn shapes_flag_refuses_an_unknown_shape() {
        let refused = parse_args_from(argv(&["--shapes", "S1,S9"])).err();
        assert_eq!(refused.as_deref(), Some("unknown shape S9"));
    }

    #[test]
    fn stem_grader_label_set_is_exactly_the_closed_set() {
        const CLOSED: [&str; 4] = ["rank1", "elsewhere", "none", "unparsed"];
        let outputs = [
            output("x", "y", &["retries"]),
            output("retried", "y", &["z"]),
            output("x", "y", &["z"]),
            output("x", "y", &[]),
        ];
        let mut seen = BTreeSet::new();
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RestartEvent,
            CueKind::ServiceWentSilent,
            CueKind::RetryStorm,
            CueKind::ReflectionTrend,
        ] {
            seen.insert(names_trigger_stem_label(None, kind));
            for out in &outputs {
                seen.insert(names_trigger_stem_label(Some(out), kind));
            }
        }
        assert_eq!(seen, CLOSED.into_iter().collect::<BTreeSet<_>>());
    }

    #[test]
    fn stem_reads_rank1_for_retries_and_retried_where_the_strict_grader_does_not() {
        for statement in [
            "Excessive retries on payment-service",
            "Failed calls are retried",
        ] {
            let out = output("Payment errors", "Errors rose", &[statement]);
            assert_eq!(
                names_trigger_stem(&out, CueKind::RetryStorm),
                "rank1",
                "{statement}"
            );
            assert_eq!(
                names_trigger(&out, CueKind::RetryStorm),
                "none",
                "{statement}"
            );
        }
    }

    fn candidate_text_count(prompt: &str, part: &str) -> usize {
        match part {
            "R1" => prompt
                .lines()
                .filter(|l| *l == R1_FRAMING_INSTRUCTION)
                .count(),
            "R3" => prompt.matches(R3_CONVENTIONS_SENTENCE).count(),
            _ => prompt.matches(R2_SCHEMA_SENTENCE).count(),
        }
    }

    #[test]
    fn candidate_arms_carry_their_text_exactly_once_within_the_bound() {
        for arm in ["R1", "R3", "R2", "R1R3", "R1R2", "R3R2"] {
            for s in shapes() {
                let Ok(p) = prepare(arm, &s) else {
                    panic!("{arm} {} composes", s.id);
                };
                for part in candidate_parts(arm) {
                    assert_eq!(
                        candidate_text_count(&p.prompt, part),
                        1,
                        "{arm} {}: {part} text",
                        s.id
                    );
                }
                if candidate_parts(arm).contains(&"R2") {
                    assert_eq!(p.schema.matches(R2_SCHEMA_SENTENCE).count(), 1, "{arm}");
                }
                assert!(p.prompt.len() <= MAX_PROMPT_BYTES, "{arm} {}", s.id);
            }
        }
    }

    #[test]
    fn candidate_transforms_are_identity_once_their_text_is_present() {
        let s2 = shape("S2");
        let Ok(r1) = prepare("R1", &s2) else {
            panic!("R1 composes");
        };
        assert_eq!(apply_r1(&r1.prompt), Ok(r1.prompt.clone()));
        let Ok(r3) = prepare("R3", &s2) else {
            panic!("R3 composes");
        };
        assert_eq!(apply_r3(&r3.prompt), Ok(r3.prompt.clone()));
        let Ok(r2) = prepare("R2", &s2) else {
            panic!("R2 composes");
        };
        assert_eq!(
            apply_r2(&r2.prompt, &r2.schema),
            Ok((r2.prompt.clone(), r2.schema.clone()))
        );
    }

    #[test]
    fn r2_schema_parses_and_changes_only_the_hypotheses_description() {
        let base = L4_OUTPUT_JSON_SCHEMA.replace(R2_SCHEMA_SENTENCE, "");
        let Ok(edited) = r2_schema(&base) else {
            panic!("R2 edits the schema");
        };
        assert_ne!(edited, base);
        let Ok(mut parsed) = serde_json::from_str::<Value>(&edited) else {
            panic!("the edited schema is JSON");
        };
        let Ok(original) = serde_json::from_str::<Value>(&base) else {
            panic!("the base schema is JSON");
        };
        let at = "/properties/hypotheses/description";
        assert_eq!(
            parsed.pointer(at),
            Some(&Value::String(format!("{R2_ANCHOR}{R2_SCHEMA_SENTENCE}")))
        );
        if let Some(d) = parsed.pointer_mut(at) {
            *d = Value::String(R2_ANCHOR.to_string());
        }
        assert_eq!(parsed, original);
    }

    #[test]
    fn thinking_label_set_is_exactly_the_closed_set() {
        const CLOSED: [&str; 3] = ["present", "absent", "unread"];
        let seen: BTreeSet<&str> = [
            Some("[Start thinking]\nhmm\n[End thinking]\n{}"),
            Some("{}"),
            None,
        ]
        .into_iter()
        .map(thinking_label)
        .collect();
        assert_eq!(seen, CLOSED.into_iter().collect::<BTreeSet<_>>());
    }

    #[test]
    fn thinking_reads_present_on_the_b9305_marker() {
        let stdout =
            "> prompt\n[Start thinking]\nThe user wants {json}\n[End thinking]\n\n{\"a\":1}";
        assert_eq!(thinking_label(Some(stdout)), "present");
    }

    #[test]
    fn thinking_reads_absent_without_the_marker() {
        let stdout = "> prompt\n{\"decision\":\"surface\",\"thinking\":\"no\"}\n[ Prompt: 1 t/s ]";
        assert_eq!(thinking_label(Some(stdout)), "absent");
    }

    #[test]
    fn vm_hwm_parser_reads_the_peak_resident_set() {
        let status =
            "Name:\tllama-cli\nVmPeak:\t 9876543 kB\nVmHWM:\t 2345678 kB\nVmRSS:\t 2000000 kB\n";
        assert_eq!(parse_vm_hwm_kib(status), Some(2_345_678));
    }

    #[test]
    fn vm_hwm_parser_reads_none_without_the_line() {
        assert_eq!(
            parse_vm_hwm_kib("Name:\tllama-cli\nState:\tZ (zombie)\n"),
            None
        );
    }

    #[test]
    fn vram_parser_keeps_the_max_for_the_child_pid_only() {
        let csv = "4242, 1834\n999, 9000\n4242, 2100\nbad line\n";
        assert_eq!(parse_vram_mib(csv, 4242), Some(2100));
        assert_eq!(parse_vram_mib(csv, 7), None);
    }

    #[test]
    fn footprint_flag_is_off_by_default() {
        let Ok(args) = parse_args_from(argv(&["--n", "2"])) else {
            panic!("--n 2 parses");
        };
        assert!(!args.footprint);
    }

    #[test]
    fn footprint_flag_sets_it_without_taking_a_value() {
        let Ok(args) = parse_args_from(argv(&["--footprint", "--n", "2"])) else {
            panic!("--footprint --n 2 parses");
        };
        assert!(args.footprint);
        assert_eq!(args.n, 2);
    }

    #[test]
    fn nr_arm_removes_exactly_the_reasoning_switch() {
        let s1 = shape("S1");
        let (Ok(shipped), Ok(nr)) = (prepare("shipped", &s1), prepare("nr", &s1)) else {
            panic!("shipped and nr compose");
        };
        assert_eq!(nr.prompt, shipped.prompt, "nr varies the argv only");
        assert_eq!(nr.schema, shipped.schema, "nr varies the argv only");
        let (model, schema) = (Path::new("/m.gguf"), Path::new("/s.json"));
        let shipped_argv = compose_argv(&shipped, model, 99, schema, None);
        let Some(at) = shipped_argv
            .windows(2)
            .position(|w| w[0] == "-rea" && w[1] == "off")
        else {
            panic!("the shipped argv carries -rea off");
        };
        let mut expected = shipped_argv.clone();
        expected.drain(at..at + 2);
        assert_eq!(compose_argv(&nr, model, 99, schema, None), expected);
    }

    #[test]
    fn gb_arm_swaps_exactly_the_schema_file_for_the_grammar_file() {
        let s1 = shape("S1");
        let (Ok(shipped), Ok(gb)) = (prepare("shipped", &s1), prepare("gb", &s1)) else {
            panic!("shipped and gb compose");
        };
        assert_eq!(gb.prompt, shipped.prompt, "gb varies the argv only");
        let (model, schema, gbnf) = (
            Path::new("/m.gguf"),
            Path::new("/s.json"),
            Path::new("/g.gbnf"),
        );
        let shipped_argv = compose_argv(&shipped, model, 99, schema, Some(gbnf));
        let Some(at) = shipped_argv
            .windows(2)
            .position(|w| w[0] == "--json-schema-file" && w[1] == "/s.json")
        else {
            panic!("the shipped argv carries --json-schema-file");
        };
        let mut expected = shipped_argv.clone();
        expected.drain(at..at + 2);
        expected.extend(["--grammar-file".to_string(), "/g.gbnf".to_string()]);
        assert_eq!(compose_argv(&gb, model, 99, schema, Some(gbnf)), expected);
    }

    #[test]
    fn gbnf_flag_takes_a_path_and_is_unset_by_default() {
        let Ok(none) = parse_args_from(argv(&[])) else {
            panic!("no flags parse");
        };
        assert_eq!(none.gbnf, None);
        let Ok(set) = parse_args_from(argv(&["--arms", "gb", "--gbnf", "g.gbnf"])) else {
            panic!("--arms gb --gbnf g.gbnf parses");
        };
        assert_eq!(set.gbnf, Some(PathBuf::from("g.gbnf")));
        assert_eq!(set.arms, ["gb"]);
    }

    #[test]
    fn sampling_flag_keeps_the_allowlisted_pairs_in_order() {
        let value = "--temp 0.7 --top-p 0.8 --top-k 20 --min-p 0 --presence-penalty 1.5 \
                     --chat-template-kwargs {\"enable_thinking\":false}";
        let Ok(args) = parse_args_from(argv(&["--sampling", value])) else {
            panic!("the Qwen sampling parses");
        };
        assert_eq!(
            args.sampling,
            [
                "--temp",
                "0.7",
                "--top-p",
                "0.8",
                "--top-k",
                "20",
                "--min-p",
                "0",
                "--presence-penalty",
                "1.5",
                "--chat-template-kwargs",
                "{\"enable_thinking\":false}",
            ]
        );
        let Ok(none) = parse_args_from(argv(&[])) else {
            panic!("no flags parse");
        };
        assert!(none.sampling.is_empty());
    }

    #[test]
    fn sampling_flag_refuses_anything_outside_the_allowlist() {
        let refused = |v: &str| parse_args_from(argv(&["--sampling", v])).err();
        assert_eq!(
            refused("-m /x.gguf").as_deref(),
            Some("--sampling: unsupported flag -m")
        );
        assert_eq!(
            refused("--grammar-file g").as_deref(),
            Some("--sampling: unsupported flag --grammar-file")
        );
        assert_eq!(
            refused("--temp hot").as_deref(),
            Some("--sampling: bad value for --temp")
        );
        assert_eq!(
            refused("--chat-template-kwargs [1]").as_deref(),
            Some("--sampling: bad value for --chat-template-kwargs")
        );
        assert_eq!(
            refused("--temp").as_deref(),
            Some("--sampling takes flag value pairs")
        );
    }
}
