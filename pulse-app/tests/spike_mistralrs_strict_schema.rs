//! Step 0 spike — mistralrs strict-schema-mode validation against а real
//! local GGUF model (chunks #82/#83).
//!
//! Per arch §Established Decisions [LLM Inference Runtime — L4
//! interpretation layer] §Caveat к verify, this test exercises the real
//! mistralrs 0.8.0 API surface:
//!
//! 1. Resolve `ANDROMEDA_PULSE_MODEL_PATH` to а GGUF file.
//! 2. Construct `MistralRsInference` + call `load_from_env_if_configured`
//!    к trigger the actual `GgufModelBuilder::build().await`.
//! 3. Build а representative ~100-token L3 digest fixture + feed it
//!    through `generate_constrained(prompt, schema_json)`.
//! 4. Parse the returned JSON string via `interpretation::schema::
//!    parse_bounded` к assert strict-schema mode constrained the output
//!    к the embedded `L4_OUTPUT_JSON_SCHEMA`.
//! 5. Record total wall-time + emit detail к stdout for the spike-result
//!    archival at `.andromeda/runs/{ISO}-step0-spike/spike-result.md`.
//!
//! The test is `#[ignore]`-gated: it requires а model file on disk +
//! takes minutes к run on CPU (no GPU). Default `cargo nextest run`
//! does NOT execute it; explicit `--ignored` flag does. Per CLAUDE.md
//! testing.md 2026-05-20 / 2026-05-22, the test lives в pulse-app/tests/
//! (integration test crate).

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use interpretation::broadcast::ModelStatusBroadcast;
use interpretation::contract::{LlmInferenceRunner, ModelStatus, ModelTier};
use interpretation::schema::{L4_OUTPUT_JSON_SCHEMA, parse_bounded};
use pulse_app::mistralrs_inference::{ENV_MODEL_PATH, MistralRsInference};
use triage::contract::HardwareProfile;

/// Representative ~100-token L3 digest fixture used as the inference
/// input. Mimics the shape of а chunk #81-assembled digest's
/// `payload_summary` string: window + services + аnomaly notes.
fn fixture_digest() -> &'static str {
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

/// Construct the primary-tier prompt around the fixture digest.
fn fixture_prompt() -> String {
    interpretation::prompt::build_primary_tier_prompt(
        fixture_digest(),
        "workspace=/dev/test-fixture",
        "",
    )
}

/// Resolve the model path; skips the test with а tracing message if
/// `ANDROMEDA_PULSE_MODEL_PATH` is unset OR the file is missing — the
/// test only makes sense against а real model.
fn resolve_or_skip_model_path() -> Option<PathBuf> {
    let raw = std::env::var(ENV_MODEL_PATH).ok()?;
    let path = PathBuf::from(raw);
    if !path.exists() {
        eprintln!(
            "[spike] {} resolves к non-existent path {:?}; skipping",
            ENV_MODEL_PATH, path
        );
        return None;
    }
    Some(path)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires ANDROMEDA_PULSE_MODEL_PATH к point at а real local GGUF file; spike-only"]
async fn spike_mistralrs_strict_schema_round_trips_against_real_model() {
    // Re-validate env var presence at test body entry (it's the gating
    // signal); the resolver returns None if unset.
    let Some(model_path) = resolve_or_skip_model_path() else {
        panic!(
            "[spike] {} env var unset OR path invalid; set it к а valid GGUF \
             file before running с --ignored",
            ENV_MODEL_PATH
        );
    };
    eprintln!("[spike] model path: {:?}", model_path);

    // Construct the runner against the host's detected hardware
    // profile. `cpu_primary` is а safe default for non-GPU runs; the
    // load body calls `.with_force_cpu()` regardless так this matches.
    let broadcast = ModelStatusBroadcast::new();
    let runner = Arc::new(MistralRsInference::new(
        ModelTier::Primary,
        HardwareProfile::CpuPrimary,
        broadcast,
    ));

    // PHASE 1 — model load.
    let load_start = Instant::now();
    eprintln!("[spike] phase 1: load starting");
    let load_result = runner.load_from_env_if_configured().await;
    let load_elapsed = load_start.elapsed();
    eprintln!(
        "[spike] phase 1: load complete ({:.2}s) — result: {:?}",
        load_elapsed.as_secs_f64(),
        load_result
            .as_ref()
            .map(|_| "Ok")
            .map_err(|e| format!("{:?}", e))
    );
    load_result.expect("model load must succeed against а real GGUF file");

    assert!(
        matches!(runner.current_status(), ModelStatus::Loaded),
        "after successful load_from_env_if_configured, status must be Loaded; got {:?}",
        runner.current_status()
    );

    let identity = runner.identity();
    eprintln!("[spike] phase 1: model identity = {:?}", identity);
    assert!(
        identity.is_some(),
        "loaded model must carry а semantic identity"
    );

    // PHASE 2 — strict-schema-mode inference.
    let prompt = fixture_prompt();
    eprintln!(
        "[spike] phase 2: prompt assembled ({} chars; fixture digest \
         + project context + schema embed + output reminder)",
        prompt.len()
    );

    let inference_start = Instant::now();
    eprintln!("[spike] phase 2: inference starting");
    let raw_output_result = runner
        .generate_constrained(&prompt, L4_OUTPUT_JSON_SCHEMA)
        .await;
    let inference_elapsed = inference_start.elapsed();
    let inference_ms = inference_elapsed.as_millis();
    eprintln!(
        "[spike] phase 2: inference complete ({}ms / {:.2}s)",
        inference_ms,
        inference_elapsed.as_secs_f64()
    );

    let raw_output = raw_output_result.expect("inference must return а constrained JSON string");
    eprintln!(
        "[spike] phase 2: raw output {} bytes (first 200 chars): {}",
        raw_output.len(),
        raw_output.chars().take(200).collect::<String>()
    );

    // PHASE 3 — JSON round-trip + schema validation.
    let parsed = parse_bounded(raw_output.as_bytes())
        .expect("strict-schema-mode output must parse + validate as L4Output");
    eprintln!(
        "[spike] phase 3: schema validation PASSED — decision={:?} severity={:?} \
         hypotheses={} steps={}",
        parsed.decision,
        parsed.severity,
        parsed.hypotheses.len(),
        parsed.investigation_steps.len()
    );

    // PHASE 4 — totals.
    let total_elapsed = load_start.elapsed();
    eprintln!(
        "[spike] PHASE 4 totals: load {:.2}s + inference {:.2}s = total {:.2}s",
        load_elapsed.as_secs_f64(),
        inference_elapsed.as_secs_f64(),
        total_elapsed.as_secs_f64()
    );
    eprintln!(
        "[spike] VERDICT: PASS — mistralrs 0.8.0 strict-schema mode validated against \
         real model; output round-trips against L4_OUTPUT_JSON_SCHEMA"
    );
}
