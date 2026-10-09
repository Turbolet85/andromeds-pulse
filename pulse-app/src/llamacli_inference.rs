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
//! single-turn flag + outer `tokio::time::timeout` wall-clock guard. The
//! argv also pins `-c 8192` (the footprint follows the weights, not the
//! model's training context) and `-rea off` (no thinking pass printed to
//! stdout ahead of the JSON).
//!
//! Output is constrained by the committed GBNF [`L4_OUTPUT_GBNF`] passed
//! through `--grammar-file`, never `--json-schema-file`: b9305 prefills an
//! output-format grammar with the chat template's generation prompt, which a
//! thinking template's grammar rejects at sampler init, while a user grammar
//! is never prefilled. The grammar is the b9305 converter's output for
//! `L4_OUTPUT_JSON_SCHEMA` (pinned by `pulse-app/tests/unit_l4_grammar.rs`).
//! Sampling is the shipped model's published setting: `--temp 1.0 --top-p
//! 0.95 --top-k 64 --min-p 0`.
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
use interpretation::schema::L4_OUTPUT_JSON_SCHEMA;
use tokio::io::AsyncReadExt;
use tokio::time::timeout;
use triage::contract::HardwareProfile;

/// Env var configuring the on-disk model file path. Preserved from chunk
/// #82 `MistralRsInference` for backward compatibility with existing
/// development scripts and documentation.
pub const ENV_MODEL_PATH: &str = "ANDROMEDA_PULSE_MODEL_PATH";

/// Env var resolving the prebuilt `llama-cli.exe` CUDA build (b9305-pinned
/// series). Consumed by GPU-primary / GPU-fallback tiers. Naming follows
/// arch §Conventions `_PATH` suffix discipline for path-shaped env vars.
pub const ENV_LLAMA_CUDA_BIN_PATH: &str = "ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH";

/// Env var resolving the prebuilt `llama-cli.exe` CPU build (b9305-pinned
/// series). Consumed by CPU-primary / CPU-fallback tiers. Naming follows
/// arch §Conventions `_PATH` suffix discipline.
pub const ENV_LLAMA_CPU_BIN_PATH: &str = "ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH";

/// Env var declaring an OPT-IN confinement root for the three L4 path
/// inputs. When set, every resolved model / binary path must live under it;
/// when unset the paths stay unconfined and one record per boot says so.
/// Opt-in rather than defaulting to the data dir because the GGUF and the
/// prebuilt `llama-cli.exe` are user-managed and live outside it by design
/// per arch §Established Decisions [LLM Inference Runtime] — a data-dir
/// default would reject every shipped configuration.
pub const ENV_L4_ALLOW_ROOT: &str = "ANDROMEDA_PULSE_L4_ALLOW_ROOT";

/// Byte ceiling on a raw path env-var value before canonicalization.
/// Mirrors `workspace_detector`'s `MAX_WORKSPACE_KEY_BYTES` bound.
pub const MAX_PATH_INPUT_BYTES: usize = 4096;

/// Byte ceiling on the assembled prompt before it becomes the `-p` argv
/// value. Measured basis: 154 real-model assemblies spanned 5947..=6297
/// bytes, so this is ~2.6x the observed maximum. It also sits below the
/// Windows `CreateProcess` command-line limit (32767), so an over-long
/// prompt is rejected here with a bounded category instead of failing
/// opaquely at spawn.
pub const MAX_PROMPT_BYTES: usize = 16 * 1024;

/// Wall-clock timeout for each subprocess invocation. Bounds runaway
/// generation per arch §Established Decisions [LLM Inference Runtime]
/// defense-in-depth paragraph + CLAUDE.md testing.md 2026-05-25 subprocess
/// discipline. Sized for typical L4 envelope (warm-cache ~4s; cold ~5-10s
/// plus cushion for outlier prompts). Settings exposure deferred per chunk
/// #84 plan implementation notes.
pub const LLAMA_CLI_TIMEOUT: Duration = Duration::from_secs(60);

/// Hard cap on subprocess stdout byte length. Defense against runaway
/// generation that bypasses the `-n {max_tokens}` cap (e.g., constrained
/// generation that loops within the GBNF grammar). Reuses the chunk #82
/// L4 output bound (~4 KB schema-conformant JSON) plus generous headroom.
pub const LLAMA_CLI_MAX_OUTPUT_BYTES: usize = 64 * 1024;

/// Default token cap for `-n` arg when caller does not specify. Sized
/// for L4 envelope (typical ~530 bytes / ~250-tok schema-conformant JSON).
pub const DEFAULT_MAX_TOKENS: u32 = 1024;

/// Context size passed as `-c`. Without it b9305 runs the model's
/// TRAINING context, shrunk only to fit free device memory, so the
/// resident footprint tracks the model's context length rather than its
/// weights. 8192 covers the largest measured prompt (~2k tokens) plus the
/// `DEFAULT_MAX_TOKENS` generation with headroom.
pub const LLAMA_CLI_CTX_SIZE: u32 = 8192;

/// Value passed with `-rea`. b9305 prints a thinking pass to stdout AHEAD
/// of the content, and `extract_json_object_bounded` takes the first `{`,
/// so reasoning text is a parse hazard. `off` reaches only chat templates
/// that support `enable_thinking`; on others the switch is a no-op.
pub const LLAMA_CLI_REASONING: &str = "off";

/// Sampling temperature, `--temp`. With the three below, the sampling the
/// shipped model's authors publish (gemma-4 `generation_config.json`).
pub const LLAMA_CLI_TEMP: &str = "1.0";
/// Nucleus sampling threshold, `--top-p`.
pub const LLAMA_CLI_TOP_P: &str = "0.95";
/// Top-k cutoff, `--top-k`.
pub const LLAMA_CLI_TOP_K: &str = "64";
/// Min-p threshold, `--min-p`.
pub const LLAMA_CLI_MIN_P: &str = "0";

/// The L4 output grammar: the b9305 `json_schema_to_grammar.py` output for
/// `L4_OUTPUT_JSON_SCHEMA`, byte for byte.
pub const L4_OUTPUT_GBNF: &str = include_str!("l4-output.gbnf");

/// GPU layer-offload count for CUDA build (all layers offloaded to VRAM).
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
    /// Canonicalized path to the `llama-cli.exe` binary per tier routing
    /// (CUDA build for GPU profiles, CPU build for CPU profiles). `None`
    /// when the corresponding env var is unset OR resolution failed at
    /// construction (graceful-degraded mode).
    binary_path: Option<PathBuf>,
    /// Canonicalized path to the GGUF model file per `ANDROMEDA_PULSE_MODEL_PATH`.
    /// `None` when env var unset OR resolution failed (graceful-degraded mode).
    model_path: Option<PathBuf>,
    /// GPU layer-offload count: 99 for CUDA binaries, 0 for CPU binaries.
    ngl: u32,
    /// Bounded snake_case label for tracing field cardinality discipline.
    binary_kind: &'static str,
}

struct InferenceState {
    status: ModelStatus,
    identity: Option<ModelIdentity>,
}

impl LlamaCliInference {
    /// Constructs the runner without spawning a subprocess. Reads env
    /// vars + canonicalizes paths per tier routing. App boots in
    /// graceful-degraded mode (status: Error → ModelNotConfigured) when
    /// any required path is missing OR canonicalization fails; explicit
    /// `load_from_env_if_configured()` post-construction transitions to
    /// Loaded when both paths resolve cleanly.
    pub fn new(tier: ModelTier, profile: HardwareProfile, broadcast: ModelStatusBroadcast) -> Self {
        let (env_name, ngl, binary_kind) = binary_target_for_profile(profile);
        let allow_root = resolve_allow_root();
        emit_allow_root_posture(&allow_root);
        let binary_path = resolve_guarded_path(env_name, &allow_root);
        let model_path = resolve_guarded_path(ENV_MODEL_PATH, &allow_root);

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
    /// set to a non-empty string. Does NOT canonicalize (graceful path
    /// reporting helper for diagnostics; canonicalization happens at
    /// construction time via `canonicalize_path`).
    pub fn configured_model_path() -> Option<PathBuf> {
        read_env_path(ENV_MODEL_PATH)
    }

    /// Returns the bounded snake_case label for the configured binary
    /// (`"cuda"` or `"cpu"`). Cardinality-friendly identifier for
    /// observability events.
    pub fn binary_kind(&self) -> &'static str {
        self.binary_kind
    }

    /// Emits a `ModelLoadEvent` on the broadcast topic. Used by load /
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
    /// event. Subprocess D1 has no in-process model to cache, so "Loaded"
    /// here is a readiness assertion (paths resolved + binary callable).
    pub fn mark_loaded(&self, identity: ModelIdentity) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Loaded;
            s.identity = Some(identity.clone());
        }
        self.emit_event(ModelStatus::Loaded, Some(identity), None);
    }

    /// Marks the runner as Error with a sanitized error message + emits
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
    /// stays in Error. Subprocess D1 differs from the chunk #82 in-process
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
            self.mark_error("binary path not a regular file".to_string());
            return Err(InferenceError::InvalidModelPath);
        }
        if !model.is_file() {
            self.mark_error("model path not a regular file".to_string());
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
            model_identity = %semantic_name,
            "llama-cli readiness check complete",
        );
        Ok(())
    }
}

/// Bounded category label derived from subprocess failure signals (exit
/// code / stderr first-line content / timeout-fired). Never surfaces raw
/// stderr verbatim — only a fixed enum tag per security extract
/// sanitization discipline. Pure-function shape enables exhaustive unit
/// tests without spawning real subprocesses.
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

/// Maximum number of bytes from raw stdout to include in the
/// [`InferenceError::JsonParseFailed`] reason field when extraction fails.
/// Small enough to keep error payloads bounded per security plan §Error
/// Handling boundary discipline (sanitized one-liner) yet long enough to
/// give a follow-up reader a representative sample of what came back.
pub const EXTRACT_SNIPPET_MAX_BYTES: usize = 240;

/// Extracts the JSON object body from raw `llama-cli` stdout. b9305
/// surrounds the schema-constrained JSON with a startup banner (~1400
/// bytes; "Loading model...", ASCII logo, build/model/modalities metadata,
/// "available commands:" interactive-mode hint) and a trailing perf-stats
/// line (`[ Prompt: X t/s | Generation: Y t/s ]` + "Exiting..."). The
/// schema-constrained generation (GBNF) + `-st` single-turn discipline
/// guarantee exactly one top-level JSON object between the framing, so the
/// "first `{` to matching closing `}`" slice is well-defined.
///
/// The match-pair scan walks bytes counting `{`/`}` parity (string-aware:
/// double-quote toggle with backslash escape) to find the END of the FIRST
/// top-level object — robust against trailing perf-stats text that may
/// contain stray punctuation. Returns the slice between (inclusive of
/// both braces). If no `{` exists OR the parity never balances, returns
/// [`InferenceError::JsonParseFailed`] with the truncated stdout snippet for
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
            "unbalanced braces from offset {start}; final_depth={depth}; snippet=<{}>",
            stdout_snippet(stdout)
        ),
    })
}

/// Truncates raw stdout to [`EXTRACT_SNIPPET_MAX_BYTES`] to embed safely
/// in error payloads without log-payload bloat. Replaces non-printable bytes
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

/// Builds the argument vector passed to `tokio::process::Command::args()`
/// for a single L4 inference invocation. Extracted to a pure helper for
/// unit-testable spawn-arg-vector assertion (per chunk #84 plan acceptance
/// criterion (a) — verifies all four subprocess defenses present in args).
///
/// The `kill_on_drop(true)` discipline is applied at the `Command` level
/// (not encoded in args); the unit test asserts it separately via the
/// `LlamaCliInference` construction path.
///
/// `-c {LLAMA_CLI_CTX_SIZE}` pins the resident footprint independent of the
/// model's training context, and `-rea {LLAMA_CLI_REASONING}` keeps a
/// thinking pass off stdout ahead of the JSON. The four sampling pairs
/// follow `-n`, then `--grammar-file {grammar_path}` (a file holding
/// [`L4_OUTPUT_GBNF`]); `-p {prompt}` is last. Every operand but the prompt
/// is a first-party constant or path.
pub fn build_llama_cli_args(
    model_path: &Path,
    ngl: u32,
    max_tokens: u32,
    grammar_path: &Path,
    prompt: &str,
) -> Vec<String> {
    vec![
        "-m".to_string(),
        model_path.to_string_lossy().into_owned(),
        "-ngl".to_string(),
        ngl.to_string(),
        "-c".to_string(),
        LLAMA_CLI_CTX_SIZE.to_string(),
        "-rea".to_string(),
        LLAMA_CLI_REASONING.to_string(),
        "-st".to_string(),
        "--simple-io".to_string(),
        "--no-display-prompt".to_string(),
        // `--log-disable` suppresses llama.cpp's load + perf-stats lines on
        // stderr (b9305 common/log.cpp). Does NOT remove the interactive-mode
        // banner llama-cli writes to stdout (build/model/modalities lines +
        // "available commands:" hint) — `-no-cnv` would handle that but is
        // rejected by b9305 (output: "--no-conversation is not supported by
        // llama-cli; please use llama-completion instead"). The banner is
        // therefore stripped post-hoc by `extract_json_object_bounded`.
        "--log-disable".to_string(),
        "-n".to_string(),
        max_tokens.to_string(),
        "--temp".to_string(),
        LLAMA_CLI_TEMP.to_string(),
        "--top-p".to_string(),
        LLAMA_CLI_TOP_P.to_string(),
        "--top-k".to_string(),
        LLAMA_CLI_TOP_K.to_string(),
        "--min-p".to_string(),
        LLAMA_CLI_MIN_P.to_string(),
        "--grammar-file".to_string(),
        grammar_path.to_string_lossy().into_owned(),
        "-p".to_string(),
        prompt.to_string(),
    ]
}

/// Returns the env var name plus `-ngl` value plus bounded label for the
/// configured `HardwareProfile`. GPU-primary / GPU-fallback route to CUDA
/// binary + `-ngl 99`; CPU-primary / CPU-fallback / Unknown route to CPU
/// binary + `-ngl 0`. Unknown defaults to CPU as the safe-everywhere
/// fallback per the chunk #82 `tier_for_profile` precedent that maps
/// Unknown to Primary tier; pairing Unknown with CPU binary preserves the
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

/// Reads an env var + returns `Some(PathBuf)` if set to a non-empty
/// string. Stripping here happens via `str::trim()` to accept env vars
/// with accidental surrounding whitespace.
fn read_env_path(name: &str) -> Option<PathBuf> {
    std::env::var(name)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

/// Bounded rejection categories for the path guard. Carries the reason
/// class only — never the rejected path, per obs-plan §8 Data
/// classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathRejection {
    TraversalComponent,
    PathTooLong,
    CanonicalizeFailed,
    NotRegularFile,
    OutsideAllowRoot,
    AllowRootUnresolvable,
}

impl PathRejection {
    /// Bounded snake_case label for the `error_category` tracing field.
    pub fn label(self) -> &'static str {
        match self {
            Self::TraversalComponent => "traversal_component",
            Self::PathTooLong => "path_too_long",
            Self::CanonicalizeFailed => "canonicalize_failed",
            Self::NotRegularFile => "not_regular_file",
            Self::OutsideAllowRoot => "outside_allow_root",
            Self::AllowRootUnresolvable => "allow_root_unresolvable",
        }
    }
}

/// Resolution of the opt-in confinement root declared by
/// [`ENV_L4_ALLOW_ROOT`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowRoot {
    /// Unset or empty — resolved paths are not confined.
    NotConfigured,
    /// Set and canonicalized to an existing directory.
    Enforced(PathBuf),
    /// Set but not resolvable to a directory. Every candidate is rejected
    /// rather than silently falling back to unconfined: a typo must not
    /// disable the guard the operator explicitly asked for.
    Unresolvable,
}

/// Resolves [`ENV_L4_ALLOW_ROOT`] into an [`AllowRoot`]. Bounded parse per
/// the `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` precedent: trimmed,
/// empty treated as unset, never panics, never blocks boot.
pub fn resolve_allow_root() -> AllowRoot {
    let Some(raw) = read_env_path(ENV_L4_ALLOW_ROOT) else {
        return AllowRoot::NotConfigured;
    };
    if raw.as_os_str().len() > MAX_PATH_INPUT_BYTES || path_contains_traversal(&raw) {
        return AllowRoot::Unresolvable;
    }
    match raw.canonicalize() {
        Ok(root) if root.is_dir() => AllowRoot::Enforced(root),
        _ => AllowRoot::Unresolvable,
    }
}

/// True when any component is a `..` parent-directory hop. Checked BEFORE
/// canonicalization, which resolves `..` away — a post-canonicalize check
/// can never observe it. Mirrors `crates/plugins/src/loader.rs`.
fn path_contains_traversal(path: &Path) -> bool {
    use std::path::Component;
    path.components().any(|c| matches!(c, Component::ParentDir))
}

/// Basename of a path for observability. The full path is never emitted
/// per obs-plan §11 Spans/Traces.
fn path_basename(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Validates a path env-var input: rejects a traversal component and an
/// over-long value BEFORE canonicalizing, canonicalizes, asserts a regular
/// file, then applies the opt-in confinement root.
///
/// Both sides of the confinement comparison are canonicalized — on Windows
/// `canonicalize` yields an extended-length `\\?\` prefix, so comparing a
/// prefixed child against a bare root would fail regardless of the true
/// relationship. Mirrors `workspace_detector::contract::publish_workspace_key`.
pub fn validate_path_input(
    candidate: &Path,
    allow_root: &AllowRoot,
) -> Result<PathBuf, PathRejection> {
    if path_contains_traversal(candidate) {
        return Err(PathRejection::TraversalComponent);
    }
    if candidate.as_os_str().len() > MAX_PATH_INPUT_BYTES {
        return Err(PathRejection::PathTooLong);
    }
    let resolved = candidate
        .canonicalize()
        .map_err(|_| PathRejection::CanonicalizeFailed)?;
    let metadata = std::fs::metadata(&resolved).map_err(|_| PathRejection::CanonicalizeFailed)?;
    if !metadata.is_file() {
        return Err(PathRejection::NotRegularFile);
    }
    match allow_root {
        AllowRoot::NotConfigured => Ok(resolved),
        AllowRoot::Unresolvable => Err(PathRejection::AllowRootUnresolvable),
        AllowRoot::Enforced(root) => {
            if resolved.starts_with(root) {
                Ok(resolved)
            } else {
                Err(PathRejection::OutsideAllowRoot)
            }
        }
    }
}

/// Canonicalizes a path candidate + asserts it resolves to a regular file.
/// Returns `Err(InferenceError::InvalidModelPath)` if canonicalization
/// fails (symlink loop / unreadable parent / nonexistent path) OR if the
/// resolved path is not a regular file (directory / symlink-to-directory).
/// Mirrors the chunk #41 `canonicalize_plugin_dir` security pattern from
/// `crates/plugins/src/loader.rs` adapted for binary-file targets.
///
/// Confinement-free by construction: delegates to [`validate_path_input`]
/// with [`AllowRoot::NotConfigured`]. The confined form used at
/// construction is [`resolve_guarded_path`].
pub fn canonicalize_path(candidate: &Path) -> Result<PathBuf, InferenceError> {
    validate_path_input(candidate, &AllowRoot::NotConfigured)
        .map_err(|_| InferenceError::InvalidModelPath)
}

/// Reads a path env var and validates it against the confinement root,
/// emitting a bounded rejection record on failure. `None` puts the runner
/// into graceful-degraded mode rather than failing the boot.
pub fn resolve_guarded_path(env_name: &str, allow_root: &AllowRoot) -> Option<PathBuf> {
    let raw = read_env_path(env_name)?;
    match validate_path_input(&raw, allow_root) {
        Ok(resolved) => Some(resolved),
        Err(rejection) => {
            tracing::warn!(
                target: "interpretation.model.load.error",
                env_var = env_name,
                path_basename = path_basename(&raw),
                error_category = rejection.label(),
                recovery_action = "degraded_boot",
                "L4 path input rejected",
            );
            None
        }
    }
}

/// Announces the confinement posture exactly once per process.
fn emit_allow_root_posture(allow_root: &AllowRoot) {
    static ANNOUNCED: std::sync::Once = std::sync::Once::new();
    ANNOUNCED.call_once(|| match allow_root {
        AllowRoot::NotConfigured => tracing::warn!(
            target: "interpretation.model.allow_root",
            confinement = "unconfined",
            "L4 model and binary paths are unconfined; set ANDROMEDA_PULSE_L4_ALLOW_ROOT to enforce a root",
        ),
        AllowRoot::Enforced(root) => tracing::info!(
            target: "interpretation.model.allow_root",
            confinement = "enforced",
            root_basename = path_basename(root),
            "L4 path confinement enforced",
        ),
        AllowRoot::Unresolvable => tracing::warn!(
            target: "interpretation.model.allow_root",
            confinement = "unresolvable",
            "ANDROMEDA_PULSE_L4_ALLOW_ROOT does not resolve to a directory; all L4 paths are rejected",
        ),
    });
}

/// Bounded rejection categories for the prompt guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptRejection {
    TooLong,
    ControlCharacter,
}

impl PromptRejection {
    /// Bounded snake_case label for the `error_category` tracing field.
    pub fn label(self) -> &'static str {
        match self {
            Self::TooLong => "prompt_too_long",
            Self::ControlCharacter => "prompt_control_character",
        }
    }
}

/// Bounds the assembled prompt before it becomes the `-p` argv value,
/// closing the `Command::arg(user_input)` shape security-plan §Security
/// Anti-Patterns → Code Patterns bans.
///
/// `\n`, `\r` and `\t` are layout characters the prompt builder emits by
/// construction (section headers and blank lines), so only NUL and the
/// remaining C0/C1 controls are rejected — the same predicate
/// [`stdout_snippet`] already applies in this module.
pub fn validate_prompt_bounded(prompt: &str) -> Result<(), PromptRejection> {
    if prompt.len() > MAX_PROMPT_BYTES {
        return Err(PromptRejection::TooLong);
    }
    if prompt.chars().any(is_forbidden_control) {
        return Err(PromptRejection::ControlCharacter);
    }
    Ok(())
}

fn is_forbidden_control(c: char) -> bool {
    c.is_control() && c != '\n' && c != '\r' && c != '\t'
}

/// The committed grammar for `schema_json`. Only `L4_OUTPUT_JSON_SCHEMA` has
/// one: any other schema is `grammar_schema_mismatch`, since constraining it
/// with the L4 grammar would produce output the caller's schema never asked
/// for.
pub fn grammar_for_schema(schema_json: &str) -> Result<&'static str, InferenceError> {
    if schema_json == L4_OUTPUT_JSON_SCHEMA {
        Ok(L4_OUTPUT_GBNF)
    } else {
        Err(InferenceError::InferenceFailed {
            reason: "grammar_schema_mismatch".to_string(),
        })
    }
}

/// RAII drop guard for the per-call temp file holding the grammar.
/// Constructed before subprocess spawn; dropped after `wait_with_output`
/// completes. Drop attempts cleanup but never panics (the file persists
/// in the OS temp dir if cleanup fails; OS reaps eventually).
struct GrammarTempFile {
    path: PathBuf,
}

impl GrammarTempFile {
    fn create(grammar: &str) -> Result<Self, InferenceError> {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let unique = format!(
            "andromeda-pulse-llama-grammar-{}-{}.gbnf",
            std::process::id(),
            nanos
        );
        path.push(unique);
        std::fs::write(&path, grammar).map_err(|_| InferenceError::InferenceFailed {
            reason: "grammar_tempfile_write_failed".to_string(),
        })?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for GrammarTempFile {
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

    fn hardware_profile(&self) -> HardwareProfile {
        self.profile
    }

    fn generate_constrained<'a>(
        &'a self,
        prompt: &'a str,
        schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        Box::pin(async move {
            let grammar = match grammar_for_schema(schema_json) {
                Ok(grammar) => grammar,
                Err(err) => {
                    tracing::warn!(
                        target: "interpretation.inference.error",
                        model_tier = interpretation::contract::model_tier_label(self.tier),
                        hardware_profile = profile_label(self.profile),
                        error_category = "grammar_schema_mismatch",
                        recovery_action = "skip_digest",
                        "no committed grammar for the requested schema",
                    );
                    return Err(err);
                }
            };
            let (Some(binary), Some(model)) = (self.binary_path.as_ref(), self.model_path.as_ref())
            else {
                return Err(InferenceError::ModelNotConfigured);
            };
            if !matches!(self.current_status(), ModelStatus::Loaded) {
                return Err(InferenceError::ModelNotConfigured);
            }

            if let Err(rejection) = validate_prompt_bounded(prompt) {
                tracing::warn!(
                    target: "interpretation.inference.error",
                    model_tier = interpretation::contract::model_tier_label(self.tier),
                    hardware_profile = profile_label(self.profile),
                    error_category = rejection.label(),
                    recovery_action = "skip_digest",
                    "prompt rejected before subprocess spawn",
                );
                return Err(InferenceError::InferenceFailed {
                    reason: rejection.label().to_string(),
                });
            }

            let grammar_file = GrammarTempFile::create(grammar)?;
            let args = build_llama_cli_args(
                model,
                self.ngl,
                DEFAULT_MAX_TOKENS,
                grammar_file.path(),
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

            drop(grammar_file);

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

            let status = match status_result {
                Ok(status) => status,
                Err(_) => {
                    tracing::warn!(
                        target: "interpretation.inference.error",
                        model_tier = interpretation::contract::model_tier_label(self.tier),
                        hardware_profile = profile_label(self.profile),
                        error_category = "io_error",
                        recovery_action = "skip_digest",
                        "llama-cli subprocess wait failed",
                    );
                    return Err(InferenceError::InferenceFailed {
                        reason: "io_error".to_string(),
                    });
                }
            };

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
                    "llama-cli subprocess exited with failure",
                );
                return Err(InferenceError::InferenceFailed {
                    reason: category.to_string(),
                });
            }

            let stdout_string = match String::from_utf8(stdout_bytes) {
                Ok(stdout_string) => stdout_string,
                Err(_) => {
                    tracing::warn!(
                        target: "interpretation.inference.error",
                        model_tier = interpretation::contract::model_tier_label(self.tier),
                        hardware_profile = profile_label(self.profile),
                        error_category = "stdout_utf8_invalid",
                        recovery_action = "skip_digest",
                        "llama-cli stdout is not valid UTF-8",
                    );
                    return Err(InferenceError::InferenceFailed {
                        reason: "stdout_utf8_invalid".to_string(),
                    });
                }
            };

            // Strip llama-cli b9305's startup banner + trailing perf-stats by
            // extracting the schema-constrained JSON object. Defense-in-depth:
            // `--log-disable` shrinks the stderr-side noise; this extraction
            // owns correctness on the stdout side regardless of banner drift in
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
