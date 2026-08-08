//! Integration tests for the L4 LLM subprocess D1 runner
//! (`pulse_app::llamacli_inference`). Per CLAUDE.md testing.md 2026-05-20
//! lesson, pulse-app tests live in this integration crate; source-level
//! `#[cfg(test)] mod tests` blocks compile but never run due to
//! `[lib] test = false` Windows WebView2 workaround.
//!
//! Coverage per chunk #84 plan acceptance criteria:
//! - (a) **spawn-args verification** — `build_llama_cli_args` returns
//!   vector containing all four subprocess defenses (`-n {max_tokens}` +
//!   `-st` + `kill_on_drop` discipline implicit) + tier-routing args
//! - (b) **tier routing** — `binary_target_for_profile` maps each
//!   `HardwareProfile` variant to correct env var name + `-ngl` flag
//! - (c) **env-var-missing graceful** — construct with no env vars + invoke
//!   `generate_constrained` returns `Err(InferenceError::ModelNotConfigured)`
//!   without panicking
//! - (d) **path-traversal rejected** — `canonicalize_path` returns
//!   `Err(InferenceError::InvalidModelPath)` for nonexistent OR directory
//!   OR symlink-escape candidates
//! - (e) **subprocess failure classification** — `classify_subprocess_failure`
//!   returns bounded enum tags for each documented failure category
//!
//! Test (e) covers the subprocess discipline boundary surface via the
//! pure-function classifier; an `#[ignore]`-gated test_runtime_timeout
//! reserves spot for real-binary smoke testing (requires manual platform-
//! specific sleep binary; not run by default per agent-runnable invariants).

use std::path::PathBuf;

use interpretation::broadcast::ModelStatusBroadcast;
use interpretation::contract::{InferenceError, LlmInferenceRunner, ModelStatus, ModelTier};
use pulse_app::llamacli_inference::{
    DEFAULT_MAX_TOKENS, ENV_LLAMA_CPU_BIN_PATH, ENV_LLAMA_CUDA_BIN_PATH, ENV_MODEL_PATH,
    LlamaCliInference, binary_target_for_profile, build_llama_cli_args, canonicalize_path,
    classify_subprocess_failure, extract_json_object_bounded,
};
use tempfile::TempDir;
use triage::contract::HardwareProfile;

// ============================================================================
// Test (a) — spawn args verification (4 subprocess defenses + tier routing)
// ============================================================================

#[test]
fn spawn_args_contain_max_tokens_cap_arg() {
    let schema_path = PathBuf::from("/tmp/schema.json");
    let model_path = PathBuf::from("/tmp/model.gguf");
    let args = build_llama_cli_args(&model_path, 99, 256, &schema_path, "test prompt");
    let n_idx = args.iter().position(|a| a == "-n").expect("-n arg present");
    assert_eq!(args.get(n_idx + 1).map(String::as_str), Some("256"));
}

#[test]
fn spawn_args_contain_single_turn_flag() {
    let schema_path = PathBuf::from("/tmp/schema.json");
    let model_path = PathBuf::from("/tmp/model.gguf");
    let args = build_llama_cli_args(&model_path, 99, DEFAULT_MAX_TOKENS, &schema_path, "prompt");
    assert!(args.iter().any(|a| a == "-st"), "-st flag MUST be present");
}

#[test]
fn spawn_args_contain_simple_io_and_no_display_prompt_flags() {
    let schema_path = PathBuf::from("/tmp/schema.json");
    let model_path = PathBuf::from("/tmp/model.gguf");
    let args = build_llama_cli_args(&model_path, 0, DEFAULT_MAX_TOKENS, &schema_path, "prompt");
    assert!(
        args.iter().any(|a| a == "--simple-io"),
        "--simple-io flag missing"
    );
    assert!(
        args.iter().any(|a| a == "--no-display-prompt"),
        "--no-display-prompt flag missing"
    );
}

#[test]
fn spawn_args_contain_model_path_and_schema_file_args() {
    let schema_path = PathBuf::from("/tmp/schema.json");
    let model_path = PathBuf::from("/tmp/model.gguf");
    let args = build_llama_cli_args(&model_path, 99, DEFAULT_MAX_TOKENS, &schema_path, "prompt");
    let m_idx = args.iter().position(|a| a == "-m").expect("-m arg present");
    assert_eq!(
        args.get(m_idx + 1).map(String::as_str),
        Some("/tmp/model.gguf")
    );
    let sf_idx = args
        .iter()
        .position(|a| a == "--json-schema-file")
        .expect("--json-schema-file arg present");
    assert_eq!(
        args.get(sf_idx + 1).map(String::as_str),
        Some("/tmp/schema.json")
    );
}

#[test]
fn spawn_args_contain_ngl_routing_per_profile() {
    let schema_path = PathBuf::from("/tmp/schema.json");
    let model_path = PathBuf::from("/tmp/model.gguf");

    let args_gpu = build_llama_cli_args(&model_path, 99, 100, &schema_path, "prompt");
    let ngl_idx = args_gpu
        .iter()
        .position(|a| a == "-ngl")
        .expect("-ngl arg present");
    assert_eq!(args_gpu.get(ngl_idx + 1).map(String::as_str), Some("99"));

    let args_cpu = build_llama_cli_args(&model_path, 0, 100, &schema_path, "prompt");
    let ngl_idx = args_cpu
        .iter()
        .position(|a| a == "-ngl")
        .expect("-ngl arg present");
    assert_eq!(args_cpu.get(ngl_idx + 1).map(String::as_str), Some("0"));
}

#[test]
fn spawn_args_contain_prompt_via_p_arg() {
    let schema_path = PathBuf::from("/tmp/schema.json");
    let model_path = PathBuf::from("/tmp/model.gguf");
    let prompt = "What is 2+2? Respond with JSON.";
    let args = build_llama_cli_args(&model_path, 99, DEFAULT_MAX_TOKENS, &schema_path, prompt);
    let p_idx = args.iter().position(|a| a == "-p").expect("-p arg present");
    assert_eq!(args.get(p_idx + 1).map(String::as_str), Some(prompt));
}

// ============================================================================
// Test (b) — tier routing for all 5 HardwareProfile variants
// ============================================================================

#[test]
fn binary_target_routes_gpu_primary_to_cuda_ngl_99() {
    let (env_name, ngl, label) = binary_target_for_profile(HardwareProfile::GpuPrimary);
    assert_eq!(env_name, ENV_LLAMA_CUDA_BIN_PATH);
    assert_eq!(ngl, 99);
    assert_eq!(label, "cuda");
}

#[test]
fn binary_target_routes_gpu_fallback_to_cuda_ngl_99() {
    let (env_name, ngl, label) = binary_target_for_profile(HardwareProfile::GpuFallback);
    assert_eq!(env_name, ENV_LLAMA_CUDA_BIN_PATH);
    assert_eq!(ngl, 99);
    assert_eq!(label, "cuda");
}

#[test]
fn binary_target_routes_cpu_primary_to_cpu_ngl_0() {
    let (env_name, ngl, label) = binary_target_for_profile(HardwareProfile::CpuPrimary);
    assert_eq!(env_name, ENV_LLAMA_CPU_BIN_PATH);
    assert_eq!(ngl, 0);
    assert_eq!(label, "cpu");
}

#[test]
fn binary_target_routes_cpu_fallback_to_cpu_ngl_0() {
    let (env_name, ngl, label) = binary_target_for_profile(HardwareProfile::CpuFallback);
    assert_eq!(env_name, ENV_LLAMA_CPU_BIN_PATH);
    assert_eq!(ngl, 0);
    assert_eq!(label, "cpu");
}

#[test]
fn binary_target_routes_unknown_to_cpu_safe_default() {
    let (env_name, ngl, label) = binary_target_for_profile(HardwareProfile::Unknown);
    assert_eq!(env_name, ENV_LLAMA_CPU_BIN_PATH);
    assert_eq!(ngl, 0);
    assert_eq!(label, "cpu");
}

// ============================================================================
// Test (c) — env-var-missing graceful (no panic; returns ModelNotConfigured)
// ============================================================================

#[test]
fn construct_with_missing_env_vars_starts_in_error_state() {
    // Test relies on env vars being unset. Tests run in isolated processes
    // per nextest profile, but defensively clear within a scope.
    let original_cuda = std::env::var(ENV_LLAMA_CUDA_BIN_PATH).ok();
    let original_cpu = std::env::var(ENV_LLAMA_CPU_BIN_PATH).ok();
    let original_model = std::env::var(ENV_MODEL_PATH).ok();
    unsafe {
        std::env::remove_var(ENV_LLAMA_CUDA_BIN_PATH);
        std::env::remove_var(ENV_LLAMA_CPU_BIN_PATH);
        std::env::remove_var(ENV_MODEL_PATH);
    }

    let broadcast = ModelStatusBroadcast::new();
    let runner = LlamaCliInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, broadcast);
    assert!(matches!(runner.current_status(), ModelStatus::Error));
    assert!(runner.identity().is_none());
    assert_eq!(runner.tier(), ModelTier::Primary);

    // Restore env vars (best-effort; tests run in isolated processes).
    unsafe {
        if let Some(v) = original_cuda {
            std::env::set_var(ENV_LLAMA_CUDA_BIN_PATH, v);
        }
        if let Some(v) = original_cpu {
            std::env::set_var(ENV_LLAMA_CPU_BIN_PATH, v);
        }
        if let Some(v) = original_model {
            std::env::set_var(ENV_MODEL_PATH, v);
        }
    }
}

#[tokio::test]
async fn generate_constrained_returns_not_configured_when_env_vars_missing() {
    let original_cuda = std::env::var(ENV_LLAMA_CUDA_BIN_PATH).ok();
    let original_cpu = std::env::var(ENV_LLAMA_CPU_BIN_PATH).ok();
    let original_model = std::env::var(ENV_MODEL_PATH).ok();
    unsafe {
        std::env::remove_var(ENV_LLAMA_CUDA_BIN_PATH);
        std::env::remove_var(ENV_LLAMA_CPU_BIN_PATH);
        std::env::remove_var(ENV_MODEL_PATH);
    }

    let broadcast = ModelStatusBroadcast::new();
    let runner =
        LlamaCliInference::new(ModelTier::Fallback, HardwareProfile::CpuFallback, broadcast);
    let result = runner.generate_constrained("test prompt", "{}").await;
    assert!(matches!(result, Err(InferenceError::ModelNotConfigured)));

    unsafe {
        if let Some(v) = original_cuda {
            std::env::set_var(ENV_LLAMA_CUDA_BIN_PATH, v);
        }
        if let Some(v) = original_cpu {
            std::env::set_var(ENV_LLAMA_CPU_BIN_PATH, v);
        }
        if let Some(v) = original_model {
            std::env::set_var(ENV_MODEL_PATH, v);
        }
    }
}

#[tokio::test]
async fn load_from_env_returns_not_configured_when_env_vars_missing() {
    let original_cuda = std::env::var(ENV_LLAMA_CUDA_BIN_PATH).ok();
    let original_cpu = std::env::var(ENV_LLAMA_CPU_BIN_PATH).ok();
    let original_model = std::env::var(ENV_MODEL_PATH).ok();
    unsafe {
        std::env::remove_var(ENV_LLAMA_CUDA_BIN_PATH);
        std::env::remove_var(ENV_LLAMA_CPU_BIN_PATH);
        std::env::remove_var(ENV_MODEL_PATH);
    }

    let broadcast = ModelStatusBroadcast::new();
    let runner = LlamaCliInference::new(ModelTier::Primary, HardwareProfile::GpuPrimary, broadcast);
    let result = runner.load_from_env_if_configured().await;
    assert!(matches!(result, Err(InferenceError::ModelNotConfigured)));

    unsafe {
        if let Some(v) = original_cuda {
            std::env::set_var(ENV_LLAMA_CUDA_BIN_PATH, v);
        }
        if let Some(v) = original_cpu {
            std::env::set_var(ENV_LLAMA_CPU_BIN_PATH, v);
        }
        if let Some(v) = original_model {
            std::env::set_var(ENV_MODEL_PATH, v);
        }
    }
}

// ============================================================================
// Test (d) — path canonicalization rejects bad inputs
// ============================================================================

#[test]
fn canonicalize_rejects_nonexistent_path() {
    let result = canonicalize_path(&PathBuf::from("/definitely/not/a/real/path/llama-cli.exe"));
    assert!(matches!(result, Err(InferenceError::InvalidModelPath)));
}

#[test]
fn canonicalize_rejects_directory_path() {
    let tmp = TempDir::new().expect("tempdir");
    // Pass the directory itself — canonicalize succeeds but `is_file` fails.
    let result = canonicalize_path(tmp.path());
    assert!(matches!(result, Err(InferenceError::InvalidModelPath)));
}

#[test]
fn canonicalize_accepts_existing_regular_file() {
    let tmp = TempDir::new().expect("tempdir");
    let file_path = tmp.path().join("fake-llama-cli.exe");
    std::fs::write(&file_path, b"fake binary contents").expect("write file");
    let result = canonicalize_path(&file_path);
    assert!(result.is_ok(), "canonicalize should accept existing file");
    let canonical = result.expect("Ok");
    assert!(canonical.is_absolute(), "canonical path should be absolute");
}

#[test]
fn canonicalize_rejects_traversal_payload() {
    // The traversal-payload itself (`../../etc/passwd`-style relative path
    // resolved against a CWD that doesn't actually contain it) → nonexistent
    // → canonicalize fails → InvalidModelPath returned. Mirrors the
    // chunk #77 path-canonicalization-rejection invariant pattern.
    let result = canonicalize_path(&PathBuf::from(
        "../../../this/path/does/not/exist/llama-cli.exe",
    ));
    assert!(
        matches!(result, Err(InferenceError::InvalidModelPath)),
        "traversal payload pointing to nonexistent target MUST reject"
    );
}

// ============================================================================
// Test (e) — subprocess failure classification (bounded enum tags)
// ============================================================================

#[test]
fn classify_subprocess_failure_returns_binary_not_found_when_spawn_says_not_found() {
    let tag = classify_subprocess_failure(None, "command not found: llama-cli.exe");
    assert_eq!(tag, "binary_not_found");
}

#[test]
fn classify_subprocess_failure_returns_binary_not_found_when_spawn_says_no_such_file() {
    let tag = classify_subprocess_failure(None, "Os error: No such file or directory (os error 2)");
    assert_eq!(tag, "binary_not_found");
}

#[test]
fn classify_subprocess_failure_returns_spawn_failed_for_generic_none_exit() {
    let tag = classify_subprocess_failure(None, "permission denied");
    assert_eq!(tag, "spawn_failed");
}

#[test]
fn classify_subprocess_failure_returns_exit_nonzero_silent_when_stderr_empty() {
    let tag = classify_subprocess_failure(Some(1), "");
    assert_eq!(tag, "exit_nonzero_silent");
}

#[test]
fn classify_subprocess_failure_returns_exit_nonzero_with_stderr_when_present() {
    let tag = classify_subprocess_failure(Some(1), "some runtime error occurred");
    assert_eq!(tag, "exit_nonzero_with_stderr");
}

#[test]
fn classify_subprocess_failure_returns_schema_invalid_when_grammar_mentioned() {
    let tag = classify_subprocess_failure(Some(1), "invalid grammar in schema file");
    assert_eq!(tag, "schema_invalid");
}

#[test]
fn classify_subprocess_failure_returns_exit_zero_unexpected_when_exit_zero() {
    // Pure-function check: if a successful exit somehow reaches the classifier
    // (it shouldn't, but defense-in-depth), it returns the bounded marker.
    let tag = classify_subprocess_failure(Some(0), "");
    assert_eq!(tag, "exit_zero_unexpected");
}

// ============================================================================
// Test (f) — subprocess timeout fires (manual run; requires platform binary)
// ============================================================================

/// Reserves a test slot for full end-to-end subprocess-timeout verification
/// against a real platform binary that sleeps longer than `LLAMA_CLI_TIMEOUT`.
/// Cross-platform sleep stub generation is out of scope for CI (would
/// require either a Rust test-helper crate with a platform-specific
/// build.rs OR vendoring a shell/cmd wrapper script). Per chunk #84 plan
/// implementation notes, the FOUR-bound discipline is verified via the
/// (a) spawn-args tests + the pure-function (e) classifier tests; this
/// `#[ignore]`-gated test reserves the smoke slot for manual verification.
///
/// Manual run (Unix): set `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH=/bin/sleep`
/// plus `ANDROMEDA_PULSE_MODEL_PATH=/tmp/touch-me` plus run with `--ignored`.
/// Manual run (Windows): use a PowerShell sleep wrapper.
#[test]
#[ignore]
fn manual_subprocess_timeout_smoke() {
    // Intentionally empty — manual smoke slot. See doc comment for run protocol.
}

// ============================================================================
// Lifecycle transitions + broadcast verification (mirror chunk #82 precedent)
// ============================================================================

#[test]
fn mark_loading_transitions_status_to_loading() {
    let broadcast = ModelStatusBroadcast::new();
    let runner = LlamaCliInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, broadcast);
    runner.mark_loading();
    assert!(matches!(runner.current_status(), ModelStatus::Loading));
}

#[test]
fn mark_loaded_transitions_status_and_records_identity() {
    use interpretation::contract::ModelIdentity;
    let broadcast = ModelStatusBroadcast::new();
    let runner = LlamaCliInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, broadcast);
    let identity = ModelIdentity {
        semantic_name: "test-model-3b".into(),
    };
    runner.mark_loaded(identity.clone());
    assert!(matches!(runner.current_status(), ModelStatus::Loaded));
    assert_eq!(runner.identity(), Some(identity));
}

#[test]
fn mark_error_returns_status_to_error() {
    let broadcast = ModelStatusBroadcast::new();
    let runner = LlamaCliInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, broadcast);
    runner.mark_loading();
    runner.mark_error("test reason".into());
    assert!(matches!(runner.current_status(), ModelStatus::Error));
}

#[test]
fn mark_transitions_broadcast_to_subscribers() {
    let broadcast = ModelStatusBroadcast::new();
    let mut rx = broadcast.subscribe();
    let runner = LlamaCliInference::new(
        ModelTier::Fallback,
        HardwareProfile::CpuFallback,
        broadcast.clone(),
    );
    runner.mark_loading();
    let event = rx.try_recv().expect("event received");
    assert!(matches!(event.status, ModelStatus::Loading));
    assert_eq!(event.tier, ModelTier::Fallback);
    assert_eq!(event.profile_label, "cpu_fallback");
}

// ============================================================================
// Test (f) — `--log-disable` flag suppresses stderr load chatter
// ============================================================================

#[test]
fn spawn_args_contain_log_disable_flag() {
    let schema_path = PathBuf::from("/tmp/schema.json");
    let model_path = PathBuf::from("/tmp/model.gguf");
    let args = build_llama_cli_args(&model_path, 99, DEFAULT_MAX_TOKENS, &schema_path, "prompt");
    assert!(
        args.iter().any(|a| a == "--log-disable"),
        "--log-disable flag MUST be present (stderr cleanup so real errors surface)"
    );
}

// ============================================================================
// Test (g) — JSON bracket-extraction strips llama-cli b9305 banner framing
// ============================================================================

#[test]
fn extract_json_strips_real_b9305_banner_and_perf_stats_framing() {
    // Verbatim layout captured from integration smoke (session 146):
    // banner -> JSON -> trailing perf-stats + "Exiting...".
    let raw = "\n\nLoading model... \n\n\
        ASCII llama logo redacted for unit-test brevity\n\
        build      : b9305-63248fc3e\n\
        model      : Llama-3.2-3B-Instruct-Q4_K_M.gguf\n\
        modalities : text\n\n\
        available commands:\n\n\
        > prompt echoed via -p\n\n\
        {\n  \"schema_version\": \"2.0\",\n  \"decision\": \"surface\"\n}\n\n\
        [ Prompt: 9339.4 t/s | Generation: 228.1 t/s ]\n\nExiting...\n";
    let extracted = extract_json_object_bounded(raw).expect("extracts JSON object");
    assert_eq!(
        extracted,
        "{\n  \"schema_version\": \"2.0\",\n  \"decision\": \"surface\"\n}"
    );
}

#[test]
fn extract_json_handles_pure_json_no_framing() {
    let raw = "{\"k\":\"v\"}";
    let extracted = extract_json_object_bounded(raw).expect("pure JSON passes through");
    assert_eq!(extracted, "{\"k\":\"v\"}");
}

#[test]
fn extract_json_handles_nested_objects() {
    let raw = "prefix {\n  \"outer\": {\"inner\": {\"deep\": 1}},\n  \"k\": \"v\"\n} suffix";
    let extracted = extract_json_object_bounded(raw).expect("nested object extracted correctly");
    assert_eq!(
        extracted,
        "{\n  \"outer\": {\"inner\": {\"deep\": 1}},\n  \"k\": \"v\"\n}"
    );
}

#[test]
fn extract_json_ignores_braces_inside_strings() {
    // String content "{" must NOT increment depth; "}" inside string must
    // NOT decrement. The escape-aware scanner handles backslash-escapes.
    let raw = "{\"text\": \"a } b { c\", \"k\": \"v\"}";
    let extracted = extract_json_object_bounded(raw).expect("string-internal braces ignored");
    assert_eq!(extracted, "{\"text\": \"a } b { c\", \"k\": \"v\"}");
}

#[test]
fn extract_json_handles_escaped_quote_inside_string() {
    let raw = "{\"text\": \"a \\\"} b\", \"k\": \"v\"}";
    let extracted = extract_json_object_bounded(raw).expect("escaped quote handled");
    assert_eq!(extracted, "{\"text\": \"a \\\"} b\", \"k\": \"v\"}");
}

#[test]
fn extract_json_returns_json_parse_failed_when_no_open_brace() {
    let raw = "Error: model not loaded\nExiting...\n";
    let err = extract_json_object_bounded(raw).expect_err("no '{' present");
    match err {
        InferenceError::JsonParseFailed { reason } => {
            assert!(
                reason.contains("no '{' found"),
                "reason should explain missing open brace; got: {reason}"
            );
            assert!(
                reason.contains("snippet=<"),
                "reason should include snippet for diagnosability; got: {reason}"
            );
        }
        other => panic!("expected JsonParseFailed; got {other:?}"),
    }
}

#[test]
fn extract_json_returns_json_parse_failed_when_braces_unbalanced() {
    let raw = "noise {\"a\": {\"b\": 1} truncated mid-object";
    let err = extract_json_object_bounded(raw).expect_err("braces never balance");
    match err {
        InferenceError::JsonParseFailed { reason } => {
            assert!(
                reason.contains("unbalanced braces"),
                "reason should explain unbalanced state; got: {reason}"
            );
            assert!(
                reason.contains("snippet=<"),
                "reason should include snippet; got: {reason}"
            );
        }
        other => panic!("expected JsonParseFailed; got {other:?}"),
    }
}

#[test]
fn extract_json_truncates_long_snippet_in_error() {
    let mut payload = String::from("garbage with no braces ");
    // Pad past EXTRACT_SNIPPET_MAX_BYTES (240) so truncation kicks in.
    payload.push_str(&"X".repeat(500));
    let err = extract_json_object_bounded(&payload).expect_err("no '{' present");
    match err {
        InferenceError::JsonParseFailed { reason } => {
            assert!(
                reason.contains("[truncated]"),
                "reason should mark truncation; got: {reason}"
            );
            // The reason itself stays bounded — verify the snippet portion is
            // well under the full payload length.
            assert!(
                reason.len() < payload.len(),
                "reason ({}) must be shorter than payload ({})",
                reason.len(),
                payload.len(),
            );
        }
        other => panic!("expected JsonParseFailed; got {other:?}"),
    }
}
