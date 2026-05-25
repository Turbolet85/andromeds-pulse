//! Concrete `LlamaCliInference` impl of `LlmInferenceRunner` (route#84).
//!
//! Lives at the binary boundary per arch §Module dependency direction +
//! §Established Decisions [LLM Inference Runtime — L4 interpretation
//! layer] bus factor mitigation entry. Replaces the earlier
//! `MistralRsInference` sibling impl (chunks #82/#83) after empirical
//! invalidation of the in-process mistralrs path per upstream issue
//! #1134 (CPU sampler deadlock; multi-model). Subprocess D1 invokes
//! prebuilt `llama-cli.exe` binaries (b9305-pinned series) via
//! `tokio::process::Command` with FOUR-bound defense discipline:
//! `kill_on_drop(true)` + explicit `-n {max_tokens}` cap + `-st`
//! single-turn flag + outer `tokio::time::timeout` wall-clock guard.
//!
//! Tier routing follows chunk #80 `HardwareProfileSource` output:
//! GPU-primary / GPU-fallback → CUDA binary + `-ngl 99`; CPU-primary
//! / CPU-fallback → CPU binary + `-ngl 0`. Binary paths source from
//! `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` + `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`
//! env vars (canonicalized + asserted as regular files before any
//! `Command::new()` invocation; CWE-22 defense). Model file path
//! sources from `ANDROMEDA_PULSE_MODEL_PATH` (preserved from chunk #82).
//!
//! Graceful-degraded mode: if any required path is missing or invalid,
//! the runner constructs in `ModelStatus::Error` state and refuses
//! inference — the app boots cleanly without an LLM.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use chrono::Utc;
use interpretation::broadcast::ModelStatusBroadcast;
use interpretation::contract::{
    InferenceError, InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelLoadEvent,
    ModelStatus, ModelTier,
};
use interpretation::hardware::profile_label;
use tokio::io::AsyncReadExt;
use tokio::time::timeout;
use triage::contract::HardwareProfile;

/// Env var configuring the on-disk model file path. Preserved from chunk
/// #82 `MistralRsInference` for backward compatibility with existing
/// development scripts and documentation.
pub const ENV_MODEL_PATH: &str = "ANDROMEDA_PULSE_MODEL_PATH";

/// Env var resolving the prebuilt `llama-cli.exe` CUDA build (b9305-pinned
/// series). Consumed by GPU-primary / GPU-fallback tiers. Naming follows
/// arch §Conventions `_PATH` suffix discipline для path-shaped env vars.
pub const ENV_LLAMA_CUDA_BIN_PATH: &str = "ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH";

/// Env var resolving the prebuilt `llama-cli.exe` CPU build (b9305-pinned
/// series). Consumed by CPU-primary / CPU-fallback tiers. Naming follows
/// arch §Conventions `_PATH` suffix discipline.
pub const ENV_LLAMA_CPU_BIN_PATH: &str = "ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH";

/// Wall-clock timeout for each subprocess invocation. Bounds runaway
/// generation per arch §Established Decisions [LLM Inference Runtime]
/// defense-in-depth paragraph + CLAUDE.md testing.md 2026-05-25 subprocess
/// discipline. Sized для typical L4 envelope (warm-cache ~4s; cold ~5-10s
/// plus cushion для outlier prompts). Settings exposure deferred per chunk
/// #84 plan implementation notes.
pub const LLAMA_CLI_TIMEOUT: Duration = Duration::from_secs(60);

/// Hard cap on subprocess stdout byte length. Defense against runaway
/// generation that bypasses the `-n {max_tokens}` cap (e.g., constrained
/// generation that loops within the GBNF grammar). Reuses the chunk #82
/// L4 output bound (~4 KB schema-conformant JSON) plus generous headroom.
pub const LLAMA_CLI_MAX_OUTPUT_BYTES: usize = 64 * 1024;

/// Default token cap для `-n` arg when caller does not specify. Sized
/// при L4 envelope (typical ~530 bytes / ~250-tok schema-conformant JSON).
pub const DEFAULT_MAX_TOKENS: u32 = 1024;

/// GPU layer-offload count for CUDA build (all layers offloaded к VRAM).
const NGL_GPU: u32 = 99;
/// GPU layer-offload count for CPU build (all layers on CPU).
const NGL_CPU: u32 = 0;

/// Concrete impl of `LlmInferenceRunner`. Holds the configured tier
/// (set at boot per hardware profile) + the current lifecycle status +
/// the broadcast sender for emitting lifecycle events + the resolved
/// binary + model paths + `-ngl` flag value per tier routing.
pub struct LlamaCliInference {
    tier: ModelTier,
    state: Arc<RwLock<InferenceState>>,
    broadcast: ModelStatusBroadcast,
    profile: HardwareProfile,
    /// Canonicalized path к the `llama-cli.exe` binary per tier routing
    /// (CUDA build для GPU profiles, CPU build для CPU profiles). `None`
    /// when the corresponding env var is unset OR resolution failed at
    /// construction (graceful-degraded mode).
    binary_path: Option<PathBuf>,
    /// Canonicalized path к the GGUF model file per `ANDROMEDA_PULSE_MODEL_PATH`.
    /// `None` when env var unset OR resolution failed (graceful-degraded mode).
    model_path: Option<PathBuf>,
    /// GPU layer-offload count: 99 для CUDA binaries, 0 для CPU binaries.
    ngl: u32,
    /// Bounded snake_case label для tracing field cardinality discipline.
    binary_kind: &'static str,
}

struct InferenceState {
    status: ModelStatus,
    identity: Option<ModelIdentity>,
}

impl LlamaCliInference {
    /// Constructs the runner without spawning а subprocess. Reads env
    /// vars + canonicalizes paths per tier routing. App boots in
    /// graceful-degraded mode (status: Error → ModelNotConfigured) when
    /// any required path is missing OR canonicalization fails; explicit
    /// `load_from_env_if_configured()` post-construction transitions к
    /// Loaded when both paths resolve cleanly.
    pub fn new(tier: ModelTier, profile: HardwareProfile, broadcast: ModelStatusBroadcast) -> Self {
        let (env_name, ngl, binary_kind) = binary_target_for_profile(profile);
        let binary_path = read_env_path(env_name).and_then(|raw| canonicalize_path(&raw).ok());
        let model_path = read_env_path(ENV_MODEL_PATH).and_then(|raw| canonicalize_path(&raw).ok());

        let state = Arc::new(RwLock::new(InferenceState {
            status: ModelStatus::Error,
            identity: None,
        }));
        Self {
            tier,
            state,
            broadcast,
            profile,
            binary_path,
            model_path,
            ngl,
            binary_kind,
        }
    }

    /// Reads `ANDROMEDA_PULSE_MODEL_PATH` and returns `Some(PathBuf)` if
    /// set к а non-empty string. Does NOT canonicalize (graceful path
    /// reporting helper for diagnostics; canonicalization happens at
    /// construction time via `canonicalize_path`).
    pub fn configured_model_path() -> Option<PathBuf> {
        read_env_path(ENV_MODEL_PATH)
    }

    /// Returns the bounded snake_case label для the configured binary
    /// (`"cuda"` или `"cpu"`). Cardinality-friendly identifier for
    /// observability events.
    pub fn binary_kind(&self) -> &'static str {
        self.binary_kind
    }

    /// Emits а `ModelLoadEvent` on the broadcast topic. Used by load /
    /// unload / error transitions. Non-fatal if no subscribers attached.
    fn emit_event(
        &self,
        status: ModelStatus,
        identity: Option<ModelIdentity>,
        err: Option<String>,
    ) {
        let event = ModelLoadEvent {
            timestamp: Utc::now(),
            model_identity: identity,
            profile_label: profile_label(self.profile).to_string(),
            tier: self.tier,
            status,
            error_message: err,
        };
        let _ = self.broadcast.sender().send(event);
    }

    /// Marks the runner as Loading + emits an event.
    pub fn mark_loading(&self) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Loading;
        }
        self.emit_event(ModelStatus::Loading, None, None);
    }

    /// Marks the runner as Loaded with the given identity + emits an
    /// event. Subprocess D1 has no in-process model к cache, so "Loaded"
    /// here is а readiness assertion (paths resolved + binary callable).
    pub fn mark_loaded(&self, identity: ModelIdentity) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Loaded;
            s.identity = Some(identity.clone());
        }
        self.emit_event(ModelStatus::Loaded, Some(identity), None);
    }

    /// Marks the runner as Error with а sanitized error message + emits
    /// an event.
    pub fn mark_error(&self, sanitized_reason: String) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Error;
        }
        self.emit_event(ModelStatus::Error, None, Some(sanitized_reason));
    }

    /// Verifies the configured binary + model paths exist + are regular
    /// files. Transitions Loading → Loaded if both checks pass; otherwise
    /// stays в Error. Subprocess D1 differs from the chunk #82 in-process
    /// path: no actual model load happens here (the model loads per-
    /// generation inside `llama-cli`); this method asserts readiness only.
    pub async fn load_from_env_if_configured(&self) -> Result<(), InferenceError> {
        let Some(binary) = self.binary_path.as_ref() else {
            return Err(InferenceError::ModelNotConfigured);
        };
        let Some(model) = self.model_path.as_ref() else {
            return Err(InferenceError::ModelNotConfigured);
        };

        self.mark_loading();
        tracing::info!(
            target: "interpretation.model.load",
            tier = interpretation::contract::model_tier_label(self.tier),
            load_status = "loading",
            "llama-cli readiness check starting",
        );

        if !binary.is_file() {
            self.mark_error("binary path not а regular file".to_string());
            return Err(InferenceError::InvalidModelPath);
        }
        if !model.is_file() {
            self.mark_error("model path not а regular file".to_string());
            return Err(InferenceError::InvalidModelPath);
        }

        let semantic_name = model
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown-gguf-model".to_string());
        self.mark_loaded(ModelIdentity {
            semantic_name: semantic_name.clone(),
        });
        tracing::info!(
            target: "interpretation.model.load",
            tier = interpretation::contract::model_tier_label(self.tier),
            load_status = "loaded",
            "llama-cli readiness check complete",
        );
        Ok(())
    }
}

/// Bounded category label derived from subprocess failure signals (exit
/// code / stderr first-line content / timeout-fired). Never surfaces raw
/// stderr verbatim — only а fixed enum tag per security extract
/// sanitization discipline. Pure-function shape enables exhaustive unit
/// tests без spawning real subprocesses.
pub fn classify_subprocess_failure(
    exit_code: Option<i32>,
    stderr_first_line: &str,
) -> &'static str {
    let lower = stderr_first_line.to_lowercase();
    match exit_code {
        None => {
            if lower.contains("not found") || lower.contains("no such file") {
                "binary_not_found"
            } else {
                "spawn_failed"
            }
        }
        Some(0) => "exit_zero_unexpected",
        Some(_) => {
            if stderr_first_line.is_empty() {
                "exit_nonzero_silent"
            } else if lower.contains("schema") || lower.contains("grammar") {
                "schema_invalid"
            } else {
                "exit_nonzero_with_stderr"
            }
        }
    }
}

/// Maximum number of bytes from raw stdout к include в the
/// [`InferenceError::JsonParseFailed`] reason field when extraction fails.
/// Small enough к keep error payloads bounded per security plan §Error
/// Handling boundary discipline (sanitized one-liner) yet long enough к
/// give а follow-up reader а representative sample of what came back.
pub const EXTRACT_SNIPPET_MAX_BYTES: usize = 240;

/// Extracts the JSON object body from raw `llama-cli` stdout. b9305
/// surrounds the schema-constrained JSON with а startup banner (~1400
/// bytes; "Loading model...", ASCII logo, build/model/modalities metadata,
/// "available commands:" interactive-mode hint) and а trailing perf-stats
/// line (`[ Prompt: X t/s | Generation: Y t/s ]` + "Exiting..."). The
/// schema-constrained generation (GBNF) + `-st` single-turn discipline
/// guarantee exactly one top-level JSON object между the framing, so the
/// "first `{` к matching closing `}`" slice is well-defined.
///
/// The match-pair scan walks bytes counting `{`/`}` parity (string-aware:
/// double-quote toggle с backslash escape) к find the END of the FIRST
/// top-level object — robust against trailing perf-stats text that may
/// contain stray punctuation. Returns the slice between (inclusive of
/// both braces). If no `{` exists OR the parity never balances, returns
/// [`InferenceError::JsonParseFailed`] с the truncated stdout snippet for
/// diagnosability.
pub fn extract_json_object_bounded(stdout: &str) -> Result<&str, InferenceError> {
    let bytes = stdout.as_bytes();
    let Some(start) = bytes.iter().position(|&b| b == b'{') else {
        return Err(InferenceError::JsonParseFailed {
            reason: format!(
                "no '{{' found in stdout; snippet=<{}>",
                stdout_snippet(stdout)
            ),
        });
    };

    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escape_next = false;
    for (idx, &b) in bytes.iter().enumerate().skip(start) {
        if escape_next {
            escape_next = false;
            continue;
        }
        if in_string {
            match b {
                b'\\' => escape_next = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    let end = idx + 1;
                    return Ok(&stdout[start..end]);
                }
            }
            _ => {}
        }
    }

    Err(InferenceError::JsonParseFailed {
        reason: format!(
            "unbalanced braces от offset {start}; final_depth={depth}; snippet=<{}>",
            stdout_snippet(stdout)
        ),
    })
}

/// Truncates raw stdout к [`EXTRACT_SNIPPET_MAX_BYTES`] чтобы embed safely
/// in error payloads без log-payload bloat. Replaces non-printable bytes
/// to keep snippet readable.
fn stdout_snippet(stdout: &str) -> String {
    let trimmed: String = stdout
        .chars()
        .take(EXTRACT_SNIPPET_MAX_BYTES)
        .map(|c| {
            if c.is_control() && c != '\n' && c != '\r' && c != '\t' {
                '.'
            } else {
                c
            }
        })
        .collect();
    if stdout.len() > trimmed.len() {
        format!("{trimmed}...[truncated]")
    } else {
        trimmed
    }
}

/// Builds the argument vector passed к `tokio::process::Command::args()`
/// для а single L4 inference invocation. Extracted to а pure helper for
/// unit-testable spawn-arg-vector assertion (per chunk #84 plan acceptance
/// criterion (a) — verifies all four subprocess defenses present in args).
///
/// The `kill_on_drop(true)` discipline is applied at the `Command` level
/// (not encoded в args); the unit test asserts it separately via the
/// `LlamaCliInference` construction path.
pub fn build_llama_cli_args(
    model_path: &Path,
    ngl: u32,
    max_tokens: u32,
    schema_path: &Path,
    prompt: &str,
) -> Vec<String> {
    vec![
        "-m".to_string(),
        model_path.to_string_lossy().into_owned(),
        "-ngl".to_string(),
        ngl.to_string(),
        "-st".to_string(),
        "--simple-io".to_string(),
        "--no-display-prompt".to_string(),
        // `--log-disable` suppresses llama.cpp's load + perf-stats lines on
        // stderr (б9305 common/log.cpp). Does NOT remove the interactive-mode
        // banner llama-cli writes к stdout (build/model/modalities lines +
        // "available commands:" hint) — `-no-cnv` would handle that but is
        // rejected by б9305 (output: "--no-conversation is not supported by
        // llama-cli; please use llama-completion instead"). The banner is
        // therefore stripped post-hoc by `extract_json_object_bounded`.
        "--log-disable".to_string(),
        "-n".to_string(),
        max_tokens.to_string(),
        "--json-schema-file".to_string(),
        schema_path.to_string_lossy().into_owned(),
        "-p".to_string(),
        prompt.to_string(),
    ]
}

/// Returns the env var name plus `-ngl` value plus bounded label для the
/// configured `HardwareProfile`. GPU-primary / GPU-fallback route к CUDA
/// binary + `-ngl 99`; CPU-primary / CPU-fallback / Unknown route к CPU
/// binary + `-ngl 0`. Unknown defaults к CPU as the safe-everywhere
/// fallback per the chunk #82 `tier_for_profile` precedent that maps
/// Unknown к Primary tier; pairing Unknown с CPU binary preserves the
/// "boot on any hardware" invariant.
pub fn binary_target_for_profile(profile: HardwareProfile) -> (&'static str, u32, &'static str) {
    match profile {
        HardwareProfile::GpuPrimary | HardwareProfile::GpuFallback => {
            (ENV_LLAMA_CUDA_BIN_PATH, NGL_GPU, "cuda")
        }
        HardwareProfile::CpuPrimary | HardwareProfile::CpuFallback | HardwareProfile::Unknown => {
            (ENV_LLAMA_CPU_BIN_PATH, NGL_CPU, "cpu")
        }
    }
}

/// Reads an env var + returns `Some(PathBuf)` if set к а non-empty
/// string. Stripping здесь happens via `str::trim()` к accept env vars
/// with accidental surrounding whitespace.
fn read_env_path(name: &str) -> Option<PathBuf> {
    std::env::var(name)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

/// Canonicalizes а path candidate + asserts it resolves к а regular file.
/// Returns `Err(InferenceError::InvalidModelPath)` if canonicalization
/// fails (symlink loop / unreadable parent / nonexistent path) OR if the
/// resolved path is not а regular file (directory / symlink-к-directory).
/// Mirrors the chunk #41 `canonicalize_plugin_dir` security pattern from
/// `crates/plugins/src/loader.rs` adapted for binary-file targets (no
/// bounded confinement root since binary paths are intentionally user-
/// managed in dev mode per chunk #84 plan).
pub fn canonicalize_path(candidate: &Path) -> Result<PathBuf, InferenceError> {
    let resolved = candidate
        .canonicalize()
        .map_err(|_| InferenceError::InvalidModelPath)?;
    let metadata = std::fs::metadata(&resolved).map_err(|_| InferenceError::InvalidModelPath)?;
    if !metadata.is_file() {
        return Err(InferenceError::InvalidModelPath);
    }
    Ok(resolved)
}

/// RAII drop guard для the per-call temp file holding the JSON schema.
/// Constructed before subprocess spawn; dropped after `wait_with_output`
/// completes. Drop attempts cleanup but never panics (the file persists
/// в the OS temp dir if cleanup fails; OS reaps eventually).
struct SchemaTempFile {
    path: PathBuf,
}

impl SchemaTempFile {
    fn create(schema_json: &str) -> Result<Self, InferenceError> {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let unique = format!(
            "andromeda-pulse-llama-schema-{}-{}.json",
            std::process::id(),
            nanos
        );
        path.push(unique);
        std::fs::write(&path, schema_json).map_err(|_| InferenceError::InferenceFailed {
            reason: "schema_tempfile_write_failed".to_string(),
        })?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for SchemaTempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

impl LlmInferenceRunner for LlamaCliInference {
    fn current_status(&self) -> ModelStatus {
        self.state.read().expect("state lock poisoned").status
    }

    fn identity(&self) -> Option<ModelIdentity> {
        self.state
            .read()
            .expect("state lock poisoned")
            .identity
            .clone()
    }

    fn tier(&self) -> ModelTier {
        self.tier
    }

    fn generate_constrained<'a>(
        &'a self,
        prompt: &'a str,
        schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        Box::pin(async move {
            let (Some(binary), Some(model)) = (self.binary_path.as_ref(), self.model_path.as_ref())
            else {
                return Err(InferenceError::ModelNotConfigured);
            };
            if !matches!(self.current_status(), ModelStatus::Loaded) {
                return Err(InferenceError::ModelNotConfigured);
            }

            let schema_file = SchemaTempFile::create(schema_json)?;
            let args = build_llama_cli_args(
                model,
                self.ngl,
                DEFAULT_MAX_TOKENS,
                schema_file.path(),
                prompt,
            );

            let mut cmd = tokio::process::Command::new(binary);
            cmd.args(&args);
            cmd.kill_on_drop(true);
            cmd.stdin(Stdio::null());
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            let mut child = cmd.spawn().map_err(|err| {
                let category = classify_subprocess_failure(None, &err.to_string());
                InferenceError::InferenceFailed {
                    reason: category.to_string(),
                }
            })?;

            let stdout_handle = child.stdout.take();
            let stderr_handle = child.stderr.take();

            let wait_result = timeout(LLAMA_CLI_TIMEOUT, async move {
                let mut stdout_buf = Vec::new();
                let mut stderr_buf = Vec::new();
                if let Some(mut h) = stdout_handle {
                    let _ = h.read_to_end(&mut stdout_buf).await;
                }
                if let Some(mut h) = stderr_handle {
                    let _ = h.read_to_end(&mut stderr_buf).await;
                }
                let status = child.wait().await;
                (status, stdout_buf, stderr_buf)
            })
            .await;

            drop(schema_file);

            let (status_result, stdout_bytes, stderr_bytes) = match wait_result {
                Ok(triple) => triple,
                Err(_elapsed) => {
                    tracing::warn!(
                        target: "interpretation.inference.error",
                        model_tier = interpretation::contract::model_tier_label(self.tier),
                        hardware_profile = profile_label(self.profile),
                        error_category = "timeout",
                        recovery_action = "skip_digest",
                        "llama-cli subprocess wall-clock timeout",
                    );
                    return Err(InferenceError::InferenceFailed {
                        reason: "timeout".to_string(),
                    });
                }
            };

            let status = status_result.map_err(|_| InferenceError::InferenceFailed {
                reason: "io_error".to_string(),
            })?;

            if stdout_bytes.len() > LLAMA_CLI_MAX_OUTPUT_BYTES {
                return Err(InferenceError::OutputTooLarge {
                    actual_bytes: stdout_bytes.len(),
                    max_bytes: LLAMA_CLI_MAX_OUTPUT_BYTES,
                });
            }

            if !status.success() {
                let stderr_str = String::from_utf8_lossy(&stderr_bytes);
                let first_line = stderr_str.lines().next().unwrap_or("");
                let category = classify_subprocess_failure(status.code(), first_line);
                tracing::warn!(
                    target: "interpretation.inference.error",
                    model_tier = interpretation::contract::model_tier_label(self.tier),
                    hardware_profile = profile_label(self.profile),
                    error_category = category,
                    recovery_action = "skip_digest",
                    "llama-cli subprocess exited с failure",
                );
                return Err(InferenceError::InferenceFailed {
                    reason: category.to_string(),
                });
            }

            let stdout_string =
                String::from_utf8(stdout_bytes).map_err(|_| InferenceError::InferenceFailed {
                    reason: "stdout_utf8_invalid".to_string(),
                })?;

            // Strip llama-cli b9305's startup banner + trailing perf-stats by
            // extracting the schema-constrained JSON object. Defense-in-depth:
            // `--log-disable` shrinks the stderr-side noise; this extraction
            // owns correctness on the stdout side regardless of banner drift в
            // future b9305+ builds. See `extract_json_object_bounded` docs.
            let extracted = extract_json_object_bounded(&stdout_string)?;

            tracing::info!(
                target: "interpretation.constrained.generate",
                model_tier = interpretation::contract::model_tier_label(self.tier),
                hardware_profile = profile_label(self.profile),
                duration_ms = 0u64,
                success = true,
                raw_output_bytes = stdout_string.len() as u64,
                extracted_bytes = extracted.len() as u64,
                "llama-cli subprocess completed",
            );

            Ok(extracted.to_string())
        })
    }
}
