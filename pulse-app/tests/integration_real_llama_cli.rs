//! Real-subprocess integration test for the chunk #84 L4 LLM runtime
//! (`LlamaCliInference`). Closes the "stub-only unit tests miss real
//! stdout layout" coverage gap that let the b9305 banner-around-JSON
//! parse-failure ship past the standard gate set.
//!
//! **Gating discipline:** the test is env-var-gated rather than
//! `#[ignore]`-gated so dev hosts with the binaries + model installed
//! get real coverage on every `cargo nextest run --workspace` while
//! CI (which doesn't install the b9305 binaries or the 2 GB GGUF model)
//! sees a clean skip via early return — never a failure. The three
//! required env vars match the production wiring per arch §Occupied
//! Resources Environment variables:
//!
//!   - `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` (chunk #84 GPU tier route)
//!   - `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`  (chunk #84 CPU tier route)
//!   - `ANDROMEDA_PULSE_MODEL_PATH`          (chunk #82 model file)
//!
//! The test pre-checks env-var presence + canonicalization; if ANY check
//! fails, it eprintln!'s a skip notice (visible with `--nocapture`) and
//! returns. Otherwise it drives the REAL `LlamaCliInference` against the
//! real `llama-cli` subprocess against the real GGUF model file via the
//! real `build_primary_tier_prompt` over the embedded L4 JSON schema —
//! exactly the production path that handles each cadence-driven digest.
//!
//! What it proves:
//!   - subprocess actually spawns (b9305 binary callable from this host)
//!   - tier routing returns the binary + `-ngl` value matching detected profile
//!   - readiness check (load_from_env_if_configured) transitions Loading -> Loaded
//!   - schema-constrained generation completes within the four-bound discipline
//!     (`-n` + `-st` + `--log-disable` + outer timeout + kill_on_drop)
//!   - `extract_json_object_bounded` strips banner + perf-stats successfully
//!   - `parse_bounded` accepts the extracted JSON
//!   - resulting `L4Output` validates against schema invariants

use std::path::Path;
use std::time::Instant;

use interpretation::broadcast::ModelStatusBroadcast;
use interpretation::contract::{HardwareProfileSource, LlmInferenceRunner, ModelStatus, ModelTier};
use interpretation::hardware::HardwareProfileDetector;
use interpretation::prompt::build_primary_tier_prompt;
use interpretation::schema::{L4_OUTPUT_JSON_SCHEMA, parse_bounded, validate};
use pulse_app::llamacli_inference::{
    ENV_LLAMA_CPU_BIN_PATH, ENV_LLAMA_CUDA_BIN_PATH, ENV_MODEL_PATH, LlamaCliInference,
};
use triage::contract::HardwareProfile;

const REPRESENTATIVE_DIGEST: &str = "\
service.name=checkout-service\n\
attention_cues:\n\
  - kind=http_5xx_burst, severity=suggested, scope=service\n\
    observed=4.1% over 30s, baseline=0.2%, errors=12\n\
restart_events: 0 in last 5 min\n\
storm_state: idle (no retry cluster)\n\
recent_logs (sampled 3 of 12):\n\
  ERROR checkout: payment gateway timeout (3 occurrences)\n\
  WARN  checkout: retry budget exceeded\n\
  ERROR checkout: payment gateway returned 503 (9 occurrences)\n";

const REPRESENTATIVE_PROJECT_CTX: &str = "\
Project: andromeda-pulse real-subprocess integration test fixture.\n\
Stack: Rust 2024 / Tauri 2 / tonic 0.14 / DuckDB.\n\
Recent commits relevant to digest: synthetic.\n";

/// Resolves the env var for the binary that matches `profile`. Returns
/// `Some(path)` if the env var is set AND the path canonicalizes to a
/// regular file; `None` otherwise (gating signal).
fn resolve_binary_for_profile(profile: HardwareProfile) -> Option<std::path::PathBuf> {
    let env_name = match profile {
        HardwareProfile::GpuPrimary | HardwareProfile::GpuFallback => ENV_LLAMA_CUDA_BIN_PATH,
        HardwareProfile::CpuPrimary | HardwareProfile::CpuFallback | HardwareProfile::Unknown => {
            ENV_LLAMA_CPU_BIN_PATH
        }
    };
    let raw = std::env::var(env_name).ok()?;
    let path = Path::new(raw.trim()).canonicalize().ok()?;
    if std::fs::metadata(&path).ok()?.is_file() {
        Some(path)
    } else {
        None
    }
}

fn resolve_model_path() -> Option<std::path::PathBuf> {
    let raw = std::env::var(ENV_MODEL_PATH).ok()?;
    let path = Path::new(raw.trim()).canonicalize().ok()?;
    if std::fs::metadata(&path).ok()?.is_file() {
        Some(path)
    } else {
        None
    }
}

#[tokio::test]
async fn real_subprocess_round_trip_returns_schema_conformant_l4_output() {
    let detector = HardwareProfileDetector::default();
    let profile = detector.current_profile();

    let Some(binary_path) = resolve_binary_for_profile(profile) else {
        eprintln!(
            "[skip] llama-cli binary env var not set or unresolvable for profile {profile:?}; \
             set ANDROMEDA_PULSE_LLAMA_{{CUDA,CPU}}_BIN_PATH to a real b9305 llama-cli.exe \
             to exercise the real-subprocess path"
        );
        return;
    };
    let Some(model_path) = resolve_model_path() else {
        eprintln!(
            "[skip] ANDROMEDA_PULSE_MODEL_PATH not set or unresolvable; set to a real GGUF model \
             to exercise the real-subprocess path"
        );
        return;
    };

    eprintln!("[integration-real] profile={profile:?}");
    eprintln!("[integration-real] binary={}", binary_path.display());
    eprintln!("[integration-real] model={}", model_path.display());

    let broadcast = ModelStatusBroadcast::new();
    let runner = LlamaCliInference::new(ModelTier::Primary, profile, broadcast);

    let load_started = Instant::now();
    runner
        .load_from_env_if_configured()
        .await
        .expect("readiness check passes when env vars resolved + paths are regular files");
    let load_elapsed_ms = load_started.elapsed().as_millis();
    eprintln!("[integration-real] load_from_env_if_configured -> Loaded ({load_elapsed_ms} ms)");
    assert_eq!(runner.current_status(), ModelStatus::Loaded);

    let prompt = build_primary_tier_prompt(
        REPRESENTATIVE_DIGEST,
        REPRESENTATIVE_PROJECT_CTX,
        "", // corpus retrieval empty at chunk #83 substrate
    );
    eprintln!("[integration-real] prompt bytes: {}", prompt.len());

    let infer_started = Instant::now();
    let output = runner
        .generate_constrained(&prompt, L4_OUTPUT_JSON_SCHEMA)
        .await
        .expect(
            "generate_constrained succeeds with banner stripping; if this panics with \
             JsonParseFailed, extract_json_object_bounded is the regression site",
        );
    let infer_elapsed_ms = infer_started.elapsed().as_millis();
    eprintln!(
        "[integration-real] generate_constrained -> {} bytes in {infer_elapsed_ms} ms (real subprocess)",
        output.len()
    );

    assert!(
        output.starts_with('{') && output.trim_end().ends_with('}'),
        "extraction guarantees clean JSON object; got prefix={:?} suffix={:?}",
        &output[..output.len().min(20)],
        &output[output.len().saturating_sub(20)..],
    );

    let parsed =
        parse_bounded(output.as_bytes()).expect("extracted output is schema-conformant L4 JSON");
    validate(&parsed).expect("L4Output passes bounded-invariant validation");

    eprintln!(
        "[integration-real] L4Output: decision={:?} severity={:?} tier={} profile={} hypotheses={} steps={}",
        parsed.decision,
        parsed.severity,
        parsed.model_tier,
        parsed.hardware_profile,
        parsed.hypotheses.len(),
        parsed.investigation_steps.len(),
    );
    eprintln!("[integration-real] PASS — runtime swap functionally live end-to-end");
}
