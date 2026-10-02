//! Public contract surface for the L4 LLM interpretation layer.
//!
//! Declares the `LlmInferenceRunner` async trait + supporting types
//! (ModelTier / ModelStatus / ModelIdentity / ModelLoadEvent payloads +
//! InferenceError enum). The trait is object-safe — concrete impl lives at
//! the binary boundary (`pulse-app/src/llamacli_inference.rs` since
//! chunk #84; was `mistralrs_inference.rs` at chunks #82/#83) per arch
//! §Established Decisions [LLM Inference Runtime] bus factor entry +
//! §Module dependency direction.
//!
//! NO LLM-runtime imports in this crate per arch §Module boundaries
//! enforcement — leaf-crate surface stays runtime-agnostic.

use std::future::Future;
use std::pin::Pin;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Re-export hardware profile substrate from triage (chunk #80) so consumers
// of `interpretation::contract::*` see one cohesive surface. The trait + enum
// already live in `triage::contract` per existing precedent.
pub use triage::contract::{HardwareProfile, HardwareProfileSource, UnknownHardwareProfile};

/// Model tier classification per P-053. Primary models target the GPU/CPU
/// preferred-runtime tier with full prompt budget; fallback models target
/// degraded-environment scenarios with reduced output specificity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelTier {
    Primary,
    Fallback,
}

/// Static bounded label for `ModelTier` per obs §5 cardinality discipline.
pub fn model_tier_label(tier: ModelTier) -> &'static str {
    match tier {
        ModelTier::Primary => "primary",
        ModelTier::Fallback => "fallback",
    }
}

/// Lifecycle status emitted on the `pulse://stream/model-status` broadcast
/// topic. Bounded enum — no string-shaped extension without contract update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelStatus {
    Loading,
    Loaded,
    Error,
}

/// Static bounded label for `ModelStatus` per obs §5 cardinality discipline.
pub fn model_status_label(status: ModelStatus) -> &'static str {
    match status {
        ModelStatus::Loading => "loading",
        ModelStatus::Loaded => "loaded",
        ModelStatus::Error => "error",
    }
}

/// Semantic model identity — NEVER a file path. Per security plan §Logging
/// & Monitoring redact paths (CLAUDE.md Critical Warnings invariant).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelIdentity {
    /// Semantic name (e.g., "llama-3.2-3b-instruct-q4_k_m"); NO file paths,
    /// NO checkpoint URLs.
    pub semantic_name: String,
}

/// Payload broadcast on `pulse://stream/model-status` for lifecycle
/// transitions (load → loaded / load → error / loaded → unload / etc.).
/// Schema bounded per security extract "no `model_path` field; only enum
/// variants + primitive counts" discipline.
///
/// `profile_label` carries the bounded snake_case label (one of `unknown`
/// / `gpu_primary` / `gpu_fallback` / `cpu_primary` / `cpu_fallback`)
/// rather than the `HardwareProfile` enum directly — keeps the broadcast
/// payload independent of the triage crate's enum derive surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelLoadEvent {
    pub timestamp: DateTime<Utc>,
    pub model_identity: Option<ModelIdentity>,
    pub profile_label: String,
    pub tier: ModelTier,
    pub status: ModelStatus,
    /// Sanitized error message present only when `status == Error`. Never
    /// contains file paths, stack traces, library internals, or LLM-runtime
    /// type names per security extract §Error Handling §External responses.
    pub error_message: Option<String>,
}

/// Sanitization-friendly error enum at the LlmInferenceRunner trait
/// boundary. Maps to `AppError` via free-fn at `pulse-app/src/model_router.rs`
/// per CLAUDE.md 2026-05-18 cross-crate-error free-fn pattern.
#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InferenceError {
    /// Model file source (env var `ANDROMEDA_PULSE_MODEL_PATH`) not set OR
    /// resolves to non-existent file. App boots in graceful-degraded mode;
    /// model.current_profile() reports load_status: Error with this reason.
    #[error("model not configured: ANDROMEDA_PULSE_MODEL_PATH unset or invalid")]
    ModelNotConfigured,

    /// Path canonicalization rejected the configured model file (e.g.,
    /// escaped the data-dir confinement). Sanitized — never contains path.
    #[error("model path validation failed")]
    InvalidModelPath,

    /// Model file load failed (file present but the LLM runtime rejected
    /// it). Sanitized — never contains LLM-runtime internal error chain.
    #[error("model load failed: {reason}")]
    ModelLoadFailed {
        /// Sanitized one-liner — strip LLM-runtime internal types, file
        /// paths, library versions per security extract.
        reason: String,
    },

    /// Tokenizer initialization failed after model loaded successfully.
    #[error("tokenizer initialization failed: {reason}")]
    TokenizerInitFailed { reason: String },

    /// Strict-schema-mode JSON generation failed (output did not parse OR
    /// schema constraint rejected the model output).
    #[error("inference failed: {reason}")]
    InferenceFailed { reason: String },

    /// Raw inference output exceeded the defense-in-depth byte cap before
    /// `serde_json::from_slice` could allocate a parse buffer. Per security
    /// plan §Anti-Pattern Code Patterns serde_json+size-cap rule.
    #[error("L4 output too large: {actual_bytes} > {max_bytes}")]
    OutputTooLarge {
        actual_bytes: usize,
        max_bytes: usize,
    },

    /// JSON-shaped output failed structural deserialization. Sanitized one-
    /// liner — no line/column noise or internal serde paths.
    #[error("L4 JSON parse failed: {reason}")]
    JsonParseFailed { reason: String },

    /// Output deserialized cleanly but violated a bounded-length or bounded-
    /// enum invariant per the schema's defense-in-depth post-parse check.
    #[error("L4 schema violation: {reason}")]
    SchemaViolation { reason: String },
}

/// Object-safe future-return-position alias for async trait methods. Manual
/// `Pin<Box<dyn Future + Send + 'a>>` returns keep the trait object-safe
/// (`Arc<dyn LlmInferenceRunner>`) without the `async-trait` crate dep.
/// Per CLAUDE.md 2026-05-23 session-learning (extends 2026-05-16 / 2026-05-18
/// / 2026-05-19 cross-crate state delivery family from SYNC to ASYNC traits).
pub type InferenceFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, InferenceError>> + Send + 'a>>;

/// L4 LLM inference contract. Concrete impl at the binary boundary
/// (`pulse-app/src/llamacli_inference.rs` since chunk #84; subprocess
/// D1 spawn-per-generation pattern) holds all LLM-runtime orchestration
/// logic per arch §Module boundaries enforcement.
///
/// Chunk #82 declares the surface; chunk #83 implements primary-tier
/// inference; chunk #84 implements fallback-tier inference; chunk #85
/// implements JSON parse failure handling + backoff.
pub trait LlmInferenceRunner: Send + Sync {
    /// Returns the current lifecycle status of the underlying model. Cheap
    /// + non-blocking; safe to invoke from TauRPC `model.current_profile()`.
    fn current_status(&self) -> ModelStatus;

    /// Returns the semantic model identity if a model is loaded. `None`
    /// indicates either Loading (transient) OR Error (no model loaded;
    /// app in graceful-degraded mode per `ModelNotConfigured` path).
    fn identity(&self) -> Option<ModelIdentity>;

    /// Returns the configured tier for this runner. Set at boot time per
    /// hardware profile + user config; does not change at runtime.
    fn tier(&self) -> ModelTier;

    /// Strict-schema-mode JSON-constrained generation. Chunk #82 declares
    /// the method; chunk #83 wires the primary-tier prompt + schema
    /// composition; chunk #84+ adds fallback handling. Stub impl returns
    /// `Err(InferenceError::ModelNotConfigured)` when no model loaded.
    fn generate_constrained<'a>(
        &'a self,
        prompt: &'a str,
        schema_json: &'a str,
    ) -> InferenceFuture<'a, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_tier_label_is_bounded() {
        assert_eq!(model_tier_label(ModelTier::Primary), "primary");
        assert_eq!(model_tier_label(ModelTier::Fallback), "fallback");
    }

    #[test]
    fn model_status_label_is_bounded() {
        assert_eq!(model_status_label(ModelStatus::Loading), "loading");
        assert_eq!(model_status_label(ModelStatus::Loaded), "loaded");
        assert_eq!(model_status_label(ModelStatus::Error), "error");
    }

    #[test]
    fn model_load_event_serializes_to_bounded_shape() {
        let event = ModelLoadEvent {
            timestamp: Utc::now(),
            model_identity: Some(ModelIdentity {
                semantic_name: "llama-3.2-3b-instruct-q4_k_m".into(),
            }),
            profile_label: "cpu_primary".into(),
            tier: ModelTier::Primary,
            status: ModelStatus::Loaded,
            error_message: None,
        };
        let json = serde_json::to_string(&event).expect("serializes");
        // Bounded shape assertion: no path-like fields in the JSON output.
        assert!(!json.contains("model_path"));
        assert!(!json.contains("checkpoint_url"));
        assert!(!json.contains("file_path"));
        assert!(json.contains("semantic_name"));
        assert!(json.contains("profile_label"));
        assert!(json.contains("tier"));
        assert!(json.contains("status"));
    }

    #[test]
    fn inference_error_serializes_to_tagged_enum() {
        let err = InferenceError::ModelNotConfigured;
        let json = serde_json::to_string(&err).expect("serializes");
        assert!(json.contains("\"kind\""));
        assert!(json.contains("\"model_not_configured\""));
    }
}
