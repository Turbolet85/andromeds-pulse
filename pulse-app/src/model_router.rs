//! Model TauRPC router — chunk #82.
//!
//! The `model.current_profile` resolver returns the current hardware
//! profile, model tier, load status, and (when loaded) semantic model
//! identity. Mirrors `connection_router.rs` shape: library crates
//! (`crates/interpretation`, `crates/triage`) stay Tauri-free, and the
//! Tauri-aware router lives here at the binary boundary so taurpc and
//! specta deps don't leak into the workspace crates per arch
//! cross-cutting patterns on module dependency direction.
//!
//! `LlmInferenceRunner` injection sourced from the concrete
//! `MistralRsInference` impl (chunk #82 stub); `HardwareProfileSource`
//! injection sourced from `HardwareProfileDetector` (chunk #82 replaces
//! the boot stub at `pulse-app/src/hardware_profile.rs`).

use std::sync::Arc;

use interpretation::contract::{
    InferenceError, LlmInferenceRunner, ModelStatus, ModelTier, model_status_label,
    model_tier_label,
};
use interpretation::hardware::profile_label;
use serde::{Deserialize, Serialize};
use triage::contract::{HardwareProfile, HardwareProfileSource};
use ui_bridge::contract::AppError;

/// Derives the appropriate `ModelTier` from а detected `HardwareProfile`
/// per pulse-distillation-architecture.md L4 §Hardware Profile Matrix.
/// `Unknown` defaults к Primary (most-capable safe-default until a real
/// detector lands at boot per chunk #82).
pub fn tier_for_profile(profile: HardwareProfile) -> ModelTier {
    match profile {
        HardwareProfile::GpuPrimary | HardwareProfile::CpuPrimary | HardwareProfile::Unknown => {
            ModelTier::Primary
        }
        HardwareProfile::GpuFallback | HardwareProfile::CpuFallback => ModelTier::Fallback,
    }
}

/// Payload returned by `model.current_profile`. All fields are String /
/// Option<String> к sidestep cross-crate specta derive plumbing — the
/// strings are bounded snake_case labels (validated by the upstream
/// enum-to-label fns); webview consumers get string types in their
/// TypeScript bindings + assert against the same bounded set.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ModelProfilePayload {
    /// Bounded snake_case label: `unknown` / `gpu_primary` / `gpu_fallback`
    /// / `cpu_primary` / `cpu_fallback`.
    pub profile_label: String,
    /// Bounded snake_case label: `primary` / `fallback`.
    pub tier_label: String,
    /// Bounded snake_case label: `loading` / `loaded` / `error`.
    pub load_status: String,
    /// Semantic name (e.g., `"llama-3.2-3b-instruct-q4_k_m"`); `None`
    /// when the model is not loaded (Loading or Error state).
    /// NEVER а file path or checkpoint URL per security extract.
    pub model_identity_name: Option<String>,
}

#[taurpc::procedures(path = "model")]
pub trait ModelApi {
    async fn current_profile() -> Result<ModelProfilePayload, AppError>;
}

#[derive(Clone)]
pub struct ModelApiImpl {
    runner: Arc<dyn LlmInferenceRunner>,
    profile_source: Arc<dyn HardwareProfileSource>,
}

impl ModelApiImpl {
    pub fn new(
        runner: Arc<dyn LlmInferenceRunner>,
        profile_source: Arc<dyn HardwareProfileSource>,
    ) -> Self {
        Self {
            runner,
            profile_source,
        }
    }
}

#[taurpc::resolvers]
impl ModelApi for ModelApiImpl {
    #[tracing::instrument(skip_all, fields(
        profile = tracing::field::Empty,
        tier = tracing::field::Empty,
        load_status = tracing::field::Empty,
    ))]
    async fn current_profile(self) -> Result<ModelProfilePayload, AppError> {
        let profile = self.profile_source.current_profile();
        let tier = self.runner.tier();
        let status = self.runner.current_status();
        let identity = self.runner.identity();

        let payload = ModelProfilePayload {
            profile_label: profile_label(profile).to_string(),
            tier_label: model_tier_label(tier).to_string(),
            load_status: model_status_label(status).to_string(),
            model_identity_name: identity.map(|i| i.semantic_name),
        };

        let span = tracing::Span::current();
        span.record("profile", &payload.profile_label);
        span.record("tier", &payload.tier_label);
        span.record("load_status", &payload.load_status);

        tracing::info!(
            target: "model.current_profile.request",
            profile = payload.profile_label.as_str(),
            tier = payload.tier_label.as_str(),
            load_status = payload.load_status.as_str(),
            "model.current_profile returned",
        );

        Ok(payload)
    }
}

/// Free-fn map of `InferenceError` к `AppError` for use at the resolver
/// boundary. Per CLAUDE.md 2026-05-18 session-learning cross-crate-error
/// free-fn pattern (orphan rule: neither From trait nor source type nor
/// target type belongs to pulse-app, so trait impl is forbidden;
/// free-function в pulse-app router file is the canonical resolution).
///
/// Sanitization invariant: error variants map к bounded `AppError`
/// variants с sanitized messages — no stack traces, mistralrs internal
/// types, or file paths leak through.
pub fn inference_error_to_app_error(err: InferenceError) -> AppError {
    match err {
        InferenceError::ModelNotConfigured => AppError::Validation {
            field: "model_path".into(),
            reason: "ANDROMEDA_PULSE_MODEL_PATH not configured".into(),
        },
        InferenceError::InvalidModelPath => AppError::Validation {
            field: "model_path".into(),
            reason: "model path validation failed".into(),
        },
        InferenceError::ModelLoadFailed { reason } => AppError::Internal {
            message: format!("model load failed: {reason}"),
        },
        InferenceError::TokenizerInitFailed { reason } => AppError::Internal {
            message: format!("tokenizer init failed: {reason}"),
        },
        InferenceError::InferenceFailed { reason } => AppError::Internal {
            message: format!("inference failed: {reason}"),
        },
        InferenceError::OutputTooLarge {
            actual_bytes,
            max_bytes,
        } => AppError::Internal {
            message: format!("L4 output exceeded size cap: {actual_bytes} > {max_bytes}"),
        },
        InferenceError::JsonParseFailed { reason } => AppError::Internal {
            message: format!("L4 JSON parse failed: {reason}"),
        },
        InferenceError::SchemaViolation { reason } => AppError::Internal {
            message: format!("L4 schema violation: {reason}"),
        },
    }
}

#[allow(dead_code)]
fn _check_model_status_handling(_status: ModelStatus) {
    // Compile-time exhaustiveness anchor — adding а new ModelStatus
    // variant breaks compile here, forcing payload extension.
    match _status {
        ModelStatus::Loading | ModelStatus::Loaded | ModelStatus::Error => {}
    }
}

#[allow(dead_code)]
fn _check_model_tier_handling(_tier: ModelTier) {
    // Compile-time exhaustiveness anchor — adding а new ModelTier variant
    // breaks compile here, forcing payload extension.
    match _tier {
        ModelTier::Primary | ModelTier::Fallback => {}
    }
}
