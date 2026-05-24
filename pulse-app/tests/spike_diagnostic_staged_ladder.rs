//! Step 0 spike diagnostic — staged ladder validating what the original
//! spike actually measured.
//!
//! Per user's directive after session 142 wrap: the original spike ran
//! 60+ min CPU-bound and produced ZERO complete output before manual
//! termination, then concluded "120× SLO / CPU too slow." But а slow CPU
//! still emits tokens incrementally — total silence for an hour smells
//! like а hang/runaway, not honest slowness. This test isolates the
//! variables с а cheap-first staged ladder + LIVE token streaming +
//! explicit max_tokens caps + wall-clock timeouts:
//!
//! Stage 1 — plain inference, no schema, "What is 2+2?" (isolates basic
//!   inference + model integrity)
//! Stage 2 — trivial schema (isolates grammar-constraint mechanism)
//! Stage 3 — real L4 fixture с real schema, bounded (isolates L4 schema
//!   complexity + prompt size)
//!
//! Each stage:
//! - streams tokens live via `stream_chat_request` (flush stdout per chunk)
//! - sets explicit `max_len` via `RequestBuilder::set_sampler_max_len`
//! - wraps в `tokio::time::timeout` for а wall-clock bound (NO repeat of
//!   the 60-min unbounded burn)
//! - measures time-to-first-token, tokens/sec, total time, completion
//!   reason
//!
//! ORIGINAL SPIKE BUG CONFIRMED PRE-RUN (from code inspection):
//! - `pulse-app/src/mistralrs_inference.rs::generate_constrained` calls
//!   `messages.into() -> RequestBuilder` без any `.set_sampler_max_len(N)`
//! - `RequestLike::take_sampling_params()` defaults к
//!   `SamplingParams::deterministic()` which has `max_len: None`
//!   (mistralrs-core sampler.rs:118 — explicit "No maximum length" in
//!   the doc comment)
//! - The original spike test used `eprintln!` piped к `tail -80` which
//!   buffers until EOF, hiding token-level progress
//! - Combined effect: unbounded generation под grammar constraint that
//!   may never reach а natural stop state + buffered output that hides
//!   whether token 1 ever emerged
//!
//! This test is `#[ignore]`-gated (one test per ladder). Run с:
//!
//!   $env:ANDROMEDA_PULSE_MODEL_PATH = "...path к GGUF..."
//!   cargo test -p pulse-app --test spike_diagnostic_staged_ladder -- --ignored --nocapture

use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use mistralrs::{
    Constraint, GgufModelBuilder, RequestBuilder, Response, TextMessageRole, TextMessages,
};
use tokio::time::timeout;

const ENV_MODEL_PATH: &str = "ANDROMEDA_PULSE_MODEL_PATH";

/// Resolve the model path; panic с а clear message if env var absent.
fn resolve_model_path() -> PathBuf {
    let raw = std::env::var(ENV_MODEL_PATH).unwrap_or_else(|_| {
        panic!(
            "[diagnostic] {} env var not set; export it к the GGUF model path before \
             running `cargo test -- --ignored`",
            ENV_MODEL_PATH
        )
    });
    let path = PathBuf::from(raw);
    assert!(
        path.exists(),
        "[diagnostic] model path {:?} does not exist",
        path
    );
    path
}

/// Load the GGUF model directly (NOT through the production
/// MistralRsInference wrapper) к keep the diagnostic self-contained per
/// user's "do NOT change production code" constraint. Returns the
/// loaded `Model` + load wall-time.
async fn load_model() -> (mistralrs::Model, Duration) {
    let path = resolve_model_path();
    let parent = path
        .parent()
        .expect("model path has parent dir")
        .to_string_lossy()
        .into_owned();
    let filename = path
        .file_name()
        .expect("model path has file name")
        .to_string_lossy()
        .into_owned();

    eprintln!("[diagnostic] loading model: parent={parent:?} file={filename:?}");
    std::io::stderr().flush().ok();

    let start = Instant::now();
    let model = GgufModelBuilder::new(parent, vec![filename])
        .with_force_cpu()
        .build()
        .await
        .expect("model load");
    let elapsed = start.elapsed();
    eprintln!(
        "[diagnostic] model loaded в {:.2}s",
        elapsed.as_secs_f64()
    );
    std::io::stderr().flush().ok();

    (model, elapsed)
}

/// Stage result captured per run.
#[derive(Debug, Clone)]
struct StageReport {
    stage_label: &'static str,
    started_within_timeout: bool,
    time_to_first_token: Option<Duration>,
    tokens_emitted: usize,
    total_elapsed: Duration,
    finish_reason: Option<String>,
    tokens_per_sec: Option<f64>,
    final_text: String,
    truncated_at_timeout: bool,
}

impl StageReport {
    fn render(&self) {
        eprintln!("\n=== {} ===", self.stage_label);
        if !self.started_within_timeout {
            eprintln!("  RESULT: hung — no tokens emitted before wall-clock timeout");
        } else {
            eprintln!(
                "  time-to-first-token: {}",
                self.time_to_first_token
                    .map(|d| format!("{:.2}s", d.as_secs_f64()))
                    .unwrap_or_else(|| "(no tokens)".to_string())
            );
            eprintln!("  tokens emitted: {}", self.tokens_emitted);
            eprintln!("  total elapsed: {:.2}s", self.total_elapsed.as_secs_f64());
            eprintln!(
                "  tokens/sec: {}",
                self.tokens_per_sec
                    .map(|r| format!("{:.2}", r))
                    .unwrap_or_else(|| "(insufficient data)".to_string())
            );
            eprintln!(
                "  finish reason: {}",
                self.finish_reason.as_deref().unwrap_or("(none)")
            );
            eprintln!("  truncated by wall-clock timeout: {}", self.truncated_at_timeout);
            let preview: String = self.final_text.chars().take(300).collect();
            eprintln!("  final text (≤300 chars): {preview:?}");
        }
        std::io::stderr().flush().ok();
    }
}

/// Run а single stage: build request с optional schema constraint +
/// explicit `max_len`, stream tokens with live flush, enforce wall-clock
/// timeout. Returns а `StageReport` summarizing what happened.
async fn run_stage(
    label: &'static str,
    model: &mistralrs::Model,
    prompt: &str,
    constraint: Option<Constraint>,
    max_tokens: usize,
    wall_timeout: Duration,
) -> StageReport {
    eprintln!("\n[{label}] starting (max_tokens={max_tokens}, wall_timeout={:.0}s, prompt {} chars)",
        wall_timeout.as_secs_f64(),
        prompt.len()
    );
    std::io::stderr().flush().ok();

    let messages =
        TextMessages::new().add_message(TextMessageRole::User, prompt);
    let mut request: RequestBuilder = messages.into();
    if let Some(c) = constraint {
        request = request.set_constraint(c);
    }
    request = request.set_sampler_max_len(max_tokens);

    // Spawn а heartbeat task that prints elapsed time every 10s — proves
    // the test process IS alive even when no tokens are emerging. If
    // heartbeats continue без any [stream] tokens, that's strong evidence
    // inference itself is stuck (vs. test-process freeze OR stdout buffering).
    let heartbeat = tokio::spawn({
        let label_for_hb = label.to_string();
        async move {
            let hb_start = Instant::now();
            let mut iv = tokio::time::interval(Duration::from_secs(10));
            iv.tick().await; // skip first immediate tick
            loop {
                iv.tick().await;
                eprintln!(
                    "  [{label_for_hb} heartbeat] {:.0}s elapsed; awaiting tokens...",
                    hb_start.elapsed().as_secs_f64()
                );
                let _ = std::io::stderr().flush();
            }
        }
    });

    let stage_started = Instant::now();
    let stream_result = model.stream_chat_request(request).await;
    let mut stream = match stream_result {
        Ok(s) => s,
        Err(err) => {
            eprintln!("[{label}] stream init FAILED: {err}");
            return StageReport {
                stage_label: label,
                started_within_timeout: true,
                time_to_first_token: None,
                tokens_emitted: 0,
                total_elapsed: stage_started.elapsed(),
                finish_reason: Some(format!("stream_init_failed: {err}")),
                tokens_per_sec: None,
                final_text: String::new(),
                truncated_at_timeout: false,
            };
        }
    };
    eprintln!("[{label}] stream open; awaiting first chunk...");
    std::io::stderr().flush().ok();

    let mut first_token_at: Option<Instant> = None;
    let mut tokens_emitted: usize = 0;
    let mut accumulated = String::new();
    let mut finish_reason: Option<String> = None;
    let mut truncated = false;
    let mut started = true;

    // Inner loop bounded by wall_timeout — each chunk awaits до timeout
    // expires.
    let inner = async {
        while let Some(resp) = stream.next().await {
            match resp {
                Response::Chunk(chunk) => {
                    if first_token_at.is_none() {
                        first_token_at = Some(Instant::now());
                    }
                    for choice in &chunk.choices {
                        if let Some(text) = &choice.delta.content {
                            if !text.is_empty() {
                                tokens_emitted += 1;
                                accumulated.push_str(text);
                                // STREAM live: print + flush per chunk.
                                eprint!("{text}");
                                std::io::stderr().flush().ok();
                            }
                        }
                        if let Some(reason) = &choice.finish_reason {
                            finish_reason = Some(reason.clone());
                        }
                    }
                }
                Response::Done(done) => {
                    if let Some(choice) = done.choices.first() {
                        // Choice::finish_reason is `String` (not Option<String>)
                        // per mistralrs-core response.rs:88
                        finish_reason = Some(choice.finish_reason.clone());
                    }
                    break;
                }
                Response::ModelError(msg, _) => {
                    finish_reason = Some(format!("model_error: {msg}"));
                    break;
                }
                Response::InternalError(err) | Response::ValidationError(err) => {
                    finish_reason = Some(format!("error: {err}"));
                    break;
                }
                _ => {
                    // ignore other Response variants (CompletionChunk /
                    // ImageGeneration / etc — not relevant к chat path)
                }
            }
        }
    };

    match timeout(wall_timeout, inner).await {
        Ok(_) => {}
        Err(_) => {
            truncated = true;
            if first_token_at.is_none() {
                // No tokens at all before timeout. This is the key
                // diagnostic signal: differentiates hung-on-token-0
                // от slow-but-steady.
                started = false;
            }
            finish_reason = Some(format!(
                "wall_clock_timeout_after_{:.0}s",
                wall_timeout.as_secs_f64()
            ));
        }
    }
    heartbeat.abort();
    eprintln!(); // newline after streamed tokens
    std::io::stderr().flush().ok();

    let total_elapsed = stage_started.elapsed();
    let time_to_first_token = first_token_at.map(|t| t.duration_since(stage_started));
    let tokens_per_sec = match (first_token_at, total_elapsed.as_secs_f64() > 0.1) {
        (Some(_), true) if tokens_emitted >= 2 => {
            // Compute steady-state rate от first-token-emission
            // boundary к now (excludes prompt-processing time before
            // first token, which is а separate cost).
            let active = (total_elapsed - time_to_first_token.unwrap_or(Duration::ZERO))
                .as_secs_f64();
            if active > 0.05 {
                Some(tokens_emitted as f64 / active)
            } else {
                None
            }
        }
        _ => None,
    };

    StageReport {
        stage_label: label,
        started_within_timeout: started,
        time_to_first_token,
        tokens_emitted,
        total_elapsed,
        finish_reason,
        tokens_per_sec,
        final_text: accumulated,
        truncated_at_timeout: truncated,
    }
}

/// The real L4 fixture от the original spike (verbatim copy от
/// `pulse-app/tests/spike_mistralrs_strict_schema.rs::fixture_digest`).
fn original_l4_fixture_digest() -> &'static str {
    "\
WINDOW: 2026-05-24T19:00:00Z к 2026-05-24T19:01:00Z (60s)
SERVICES:
  - service-a: 240 spans / 12.5% error rate / p99 latency 1.4s (baseline 0.3s)
  - service-b: 50 spans / 0% error rate / p99 latency 0.4s
ATTENTION CUES:
  - latency-regression: service-a (4.6× baseline; persistence 50s)
  - error-correlation: service-a → service-b call chain
EVIDENCE:
  - span:abc123 (service-a, status_code=2)
  - span:def456 (service-a → service-b dependency span)"
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "diagnostic; requires ANDROMEDA_PULSE_MODEL_PATH; streams tokens live"]
async fn diagnostic_staged_ladder_isolates_spike_root_cause() {
    eprintln!("[diagnostic] === Step 0 spike validation (staged ladder) ===");
    std::io::stderr().flush().ok();

    let (model, load_elapsed) = load_model().await;

    // STAGE 1 — plain inference, no schema. Tests: model + basic
    // tokenization + tokens-per-second baseline на this host.
    let stage1 = run_stage(
        "STAGE 1: plain '2+2', no schema",
        &model,
        "What is 2+2? Answer in one word.",
        None,
        16,                        // tiny budget — single word answer
        Duration::from_secs(300),  // 5-min wall-clock; long enough к distinguish "slow but progressing" от "stuck"
    )
    .await;
    stage1.render();

    // Gate Stage 2 on Stage 1 having produced tokens. If Stage 1
    // hangs, basic inference is broken — Stage 2 + 3 wouldn't add new
    // information.
    if !stage1.started_within_timeout || stage1.tokens_emitted == 0 {
        eprintln!(
            "\n[diagnostic] STAGE 1 hung OR emitted no tokens — STOP. Basic inference is \
             not working на this host. Stage 2/3 would not add diagnostic value."
        );
        std::io::stderr().flush().ok();
        eprintln!("\nDIAGNOSIS: model-broken (basic inference fails)");
        return;
    }

    // STAGE 2 — trivial schema. Tests: grammar-constraint mechanism +
    // per-token cost increase от grammar vs no-grammar (Stage 1 baseline).
    let trivial_schema = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "additionalProperties": false,
        "required": ["answer"],
        "properties": {
            "answer": {
                "type": "string",
                "maxLength": 32
            }
        }
    });
    let stage2 = run_stage(
        "STAGE 2: trivial schema, '2+2' answer",
        &model,
        "Respond с the answer к 2+2.",
        Some(Constraint::JsonSchema(trivial_schema)),
        64,                        // small budget; single JSON object should fit
        Duration::from_secs(90),
    )
    .await;
    stage2.render();

    if !stage2.started_within_timeout || stage2.tokens_emitted == 0 {
        eprintln!(
            "\n[diagnostic] STAGE 2 hung OR emitted no tokens despite Stage 1 success. \
             The grammar-constraint mechanism is the culprit. Stage 3 would not add value."
        );
        std::io::stderr().flush().ok();
        eprintln!("\nDIAGNOSIS: grammar-bug (constraint never reaches stop state OR setup error)");
        // Render comparative table even on partial pass.
        render_comparative_table(&stage1, &stage2, None);
        return;
    }

    // STAGE 3 — real L4 fixture + real L4 schema (verbatim от original
    // spike), но bounded с explicit max_tokens (512) + wall-clock 10min.
    let real_schema_str = include_str!("../../crates/interpretation/src/schema.json");
    let real_schema_value: serde_json::Value = serde_json::from_str(real_schema_str)
        .expect("L4 schema JSON parses");
    let real_prompt = interpretation::prompt::build_primary_tier_prompt(
        original_l4_fixture_digest(),
        "workspace=/dev/test-fixture",
        "",
    );
    let stage3 = run_stage(
        "STAGE 3: real L4 fixture + real schema, bounded",
        &model,
        &real_prompt,
        Some(Constraint::JsonSchema(real_schema_value)),
        512,                       // explicit cap — never repeat 60-min unbounded burn
        Duration::from_secs(600),  // 10-min wall-clock timeout
    )
    .await;
    stage3.render();

    eprintln!(
        "\n[diagnostic] model-load wall time: {:.2}s",
        load_elapsed.as_secs_f64()
    );
    eprintln!(
        "[diagnostic] original spike fixture in /tests/spike_mistralrs_strict_schema.rs: real \
         representative L3-style digest summary (NOT а placeholder/garbage)"
    );
    eprintln!(
        "[diagnostic] original spike's generate_constrained body: NO max_tokens cap, NO stop_toks, NO timeout — \
         unbounded generation per SamplingParams::deterministic() (max_len: None per mistralrs-core sampler.rs:118)"
    );
    std::io::stderr().flush().ok();

    render_comparative_table(&stage1, &stage2, Some(&stage3));
}

fn render_comparative_table(s1: &StageReport, s2: &StageReport, s3: Option<&StageReport>) {
    eprintln!("\n=== COMPARATIVE TABLE ===");
    eprintln!(
        "Stage 1 (no schema):     ttft={:>7} tok/sec={:>6} tokens={} verdict={}",
        s1.time_to_first_token
            .map(|d| format!("{:.2}s", d.as_secs_f64()))
            .unwrap_or_else(|| "(none)".to_string()),
        s1.tokens_per_sec
            .map(|r| format!("{:.2}", r))
            .unwrap_or_else(|| "n/a".to_string()),
        s1.tokens_emitted,
        verdict_for(s1)
    );
    eprintln!(
        "Stage 2 (trivial schema):ttft={:>7} tok/sec={:>6} tokens={} verdict={}",
        s2.time_to_first_token
            .map(|d| format!("{:.2}s", d.as_secs_f64()))
            .unwrap_or_else(|| "(none)".to_string()),
        s2.tokens_per_sec
            .map(|r| format!("{:.2}", r))
            .unwrap_or_else(|| "n/a".to_string()),
        s2.tokens_emitted,
        verdict_for(s2)
    );
    if let Some(s3) = s3 {
        eprintln!(
            "Stage 3 (real L4):       ttft={:>7} tok/sec={:>6} tokens={} verdict={}",
            s3.time_to_first_token
                .map(|d| format!("{:.2}s", d.as_secs_f64()))
                .unwrap_or_else(|| "(none)".to_string()),
            s3.tokens_per_sec
                .map(|r| format!("{:.2}", r))
                .unwrap_or_else(|| "n/a".to_string()),
            s3.tokens_emitted,
            verdict_for(s3)
        );
    }

    // Compute grammar-cost-factor if Stage 2 and Stage 1 both have rates.
    if let (Some(r1), Some(r2)) = (s1.tokens_per_sec, s2.tokens_per_sec) {
        let factor = if r2 > 0.001 { r1 / r2 } else { f64::NAN };
        eprintln!(
            "Grammar-constraint per-token cost factor (no-schema/trivial-schema): {:.2}× slowdown",
            factor
        );
    }

    eprintln!();
    eprintln!("DIAGNOSIS:");
    eprintln!("  Original fixture: real representative L3-style digest (not garbage); ~100 tokens");
    eprintln!("  Original spike's generation: UNBOUNDED (no max_tokens cap, no stop_toks, no wall-clock timeout)");
    let stage3_present = s3.is_some();
    let diag = derive_diagnosis(s1, s2, s3);
    eprintln!("  Stage progression: {diag}");
    eprintln!(
        "\nRaw numbers above are the answer; the arch decision (GPU-only / SLO revision) follows \
         FROM these numbers, not the other way around."
    );
    let _ = stage3_present;
    std::io::stderr().flush().ok();
}

fn verdict_for(s: &StageReport) -> &'static str {
    if !s.started_within_timeout {
        "HUNG (no tokens)"
    } else if s.truncated_at_timeout {
        "TIMEOUT (mid-generation)"
    } else if s.tokens_emitted == 0 {
        "NO-OUTPUT (clean exit but empty)"
    } else if matches!(
        s.finish_reason.as_deref(),
        Some("length")
            | Some("length_limit")
            | Some("max_len")
            | Some("max_tokens")
    ) {
        "BOUND-HIT (max_tokens cap reached)"
    } else {
        "COMPLETED"
    }
}

fn derive_diagnosis(s1: &StageReport, s2: &StageReport, s3: Option<&StageReport>) -> String {
    if !s1.started_within_timeout || s1.tokens_emitted == 0 {
        return "model-broken — basic inference fails; spike conclusion invalid".into();
    }
    if !s2.started_within_timeout || s2.tokens_emitted == 0 {
        return "grammar-bug — basic inference works; trivial schema hangs; the 60-min original \
                spike measured а grammar-constraint bug, not CPU slowness"
            .into();
    }
    let Some(s3) = s3 else {
        return "stages 1+2 ok; stage 3 not run (gated)".into();
    };
    if !s3.started_within_timeout {
        return "real-L4-hangs — basic inference + trivial schema work; real L4 schema hangs the \
                model. The deeply-nested L4 schema is the bottleneck. Either а grammar-walk \
                pathology on the complex schema, OR unbounded loop через а constraint that never \
                reaches stop. The 60-min original spike measured THIS specific schema's behavior, \
                not CPU throughput."
            .into();
    }
    if s3.tokens_emitted > 0 && s3.truncated_at_timeout {
        format!(
            "real-slowness — basic inference + trivial schema work + real L4 IS generating but \
             slowly ({} tokens in {:.0}s = {:.2} tok/sec). The arch SLO discussion is now \
             grounded в real numbers. GPU acceleration / SLO revision / smaller-model fallback \
             can be evaluated against this baseline.",
            s3.tokens_emitted,
            s3.total_elapsed.as_secs_f64(),
            s3.tokens_per_sec.unwrap_or(0.0)
        )
    } else if s3.tokens_emitted > 0 {
        format!(
            "real-l4-completes — bounded run completed cleanly ({} tokens, {:.2}s). The 60-min \
             original spike was likely runaway generation от the missing max_tokens cap, NOT а \
             fundamental L4 schema issue. Adding `.set_sampler_max_len(N)` к the production \
             `generate_constrained` body would mitigate.",
            s3.tokens_emitted,
            s3.total_elapsed.as_secs_f64()
        )
    } else {
        "real-l4-clean-but-empty — schema constraint may immediately satisfy с empty output OR \
         model emitted zero meaningful tokens (degenerate). Investigate."
            .into()
    }
}
