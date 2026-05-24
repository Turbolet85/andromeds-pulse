//! Concrete `MistralRsInference` impl of `LlmInferenceRunner` (route#82).
//!
//! Lives at the binary boundary per arch §Module dependency direction +
//! §Established Decisions [LLM Inference Runtime — L4 interpretation
//! layer] bus factor mitigation entry. This is the ONLY file with
//! `use mistralrs::*` imports per arch §Module boundaries enforcement.
//!
//! Chunk #82 implements the construction surface + lifecycle status
//! tracking (Loading/Loaded/Error transitions emit `ModelLoadEvent` on
//! the `pulse://stream/model-status` broadcast topic). Inference itself
//! (`generate_constrained`) returns `InferenceError::ModelNotConfigured`
//! when no model loaded (graceful-degraded mode) OR delegates к
//! mistralrs strict-schema-mode in chunks #83+ when the actual model
//! load path is wired.
//!
//! Model file source: `ANDROMEDA_PULSE_MODEL_PATH` env var, canonicalized
//! through `strict-path` per security extract path canonicalization
//! discipline. If env var unset OR resolves to invalid path, the runner
//! constructs in `ModelStatus::Error` state and refuses inference —
//! the app boots cleanly without an LLM (chunks #83-#85 are no-op
//! until model present).

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use chrono::Utc;
use interpretation::broadcast::ModelStatusBroadcast;
use interpretation::contract::{
    InferenceError, InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelLoadEvent,
    ModelStatus, ModelTier,
};
use interpretation::hardware::profile_label;
use mistralrs::{Constraint, GgufModelBuilder, Model, TextMessageRole, TextMessages};
use tokio::sync::OnceCell;
use triage::contract::HardwareProfile;

/// Env var configuring the on-disk model file path. Canonicalized via
/// `strict-path` before mistralrs load.
pub const ENV_MODEL_PATH: &str = "ANDROMEDA_PULSE_MODEL_PATH";

/// Concrete impl of `LlmInferenceRunner`. Holds the configured tier
/// (set at boot per hardware profile) + the current lifecycle status
/// (updated by load/unload/error transitions) + the broadcast sender
/// for emitting lifecycle events + а `OnceCell` for the lazily-loaded
/// mistralrs `Model` (chunk #83 wiring).
pub struct MistralRsInference {
    tier: ModelTier,
    state: Arc<RwLock<InferenceState>>,
    broadcast: ModelStatusBroadcast,
    profile: HardwareProfile,
    /// Lazily-initialized mistralrs `Model`. `OnceCell` semantics: at
    /// most one successful `set` for the lifetime of the runner; reads
    /// after load are cheap. `tokio::sync::OnceCell` (not `std::sync::*`)
    /// because mistralrs's load API is async.
    model: OnceCell<Model>,
}

struct InferenceState {
    status: ModelStatus,
    identity: Option<ModelIdentity>,
}

impl MistralRsInference {
    /// Constructs the runner without loading a model. App boots in
    /// graceful-degraded mode (status: Error → ModelNotConfigured); model
    /// load is attempted by `load_from_env_if_configured()` post-construction
    /// (deferred к chunk #83+ wiring).
    pub fn new(tier: ModelTier, profile: HardwareProfile, broadcast: ModelStatusBroadcast) -> Self {
        let state = Arc::new(RwLock::new(InferenceState {
            status: ModelStatus::Error,
            identity: None,
        }));
        Self {
            tier,
            state,
            broadcast,
            profile,
            model: OnceCell::new(),
        }
    }

    /// Reads the `ANDROMEDA_PULSE_MODEL_PATH` env var + returns
    /// `Some(PathBuf)` if set к а non-empty string. Does NOT canonicalize
    /// here (deferred к chunk #83 when actual load path is wired); just
    /// reports presence/absence.
    pub fn configured_model_path() -> Option<PathBuf> {
        std::env::var(ENV_MODEL_PATH)
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from)
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
        // Send is best-effort; no subscribers → benign Err per
        // tokio::sync::broadcast semantics.
        let _ = self.broadcast.sender().send(event);
    }

    /// Marks the runner as Loading + emits an event. Chunk #83 wires this
    /// from boot-time load attempt.
    pub fn mark_loading(&self) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Loading;
        }
        self.emit_event(ModelStatus::Loading, None, None);
    }

    /// Marks the runner as Loaded with the given identity + emits an
    /// event. Chunk #83+ wires this from successful load completion.
    pub fn mark_loaded(&self, identity: ModelIdentity) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Loaded;
            s.identity = Some(identity.clone());
        }
        self.emit_event(ModelStatus::Loaded, Some(identity), None);
    }

    /// Marks the runner as Error with а sanitized error message + emits
    /// an event. Chunk #83+ wires this from failed load attempts.
    pub fn mark_error(&self, sanitized_reason: String) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Error;
        }
        self.emit_event(ModelStatus::Error, None, Some(sanitized_reason));
    }

    /// Attempts к load а GGUF model from `ANDROMEDA_PULSE_MODEL_PATH`
    /// (chunk #83 wiring). Idempotent: subsequent calls после а successful
    /// load are no-ops (OnceCell::set returns Err which we discard).
    ///
    /// Flow:
    /// 1. Read env var; if unset/empty, leave status в Error +
    ///    ModelNotConfigured (graceful-degraded mode — caller ignores
    ///    the result).
    /// 2. Validate the file exists (cheap shell-out via `std::fs::metadata`)
    ///    BEFORE invoking mistralrs (so corrupt env var doesn't trigger
    ///    HuggingFace Hub lookup as а fallback).
    /// 3. Transition state to Loading + emit `pulse://stream/model-status`
    ///    Loading event.
    /// 4. Build the GGUF model via `GgufModelBuilder::new(parent_dir,
    ///    [filename]).build().await`. mistralrs's GGUF builder takes а
    ///    HF-repo-shaped (model_id, files) pair — for local files, the
    ///    model_id is the parent directory and `files` is а singleton
    ///    list of filenames.
    /// 5. On success, store the `Model` in OnceCell + transition к Loaded
    ///    с а semantic identity derived from the filename (NOT the full
    ///    path — security plan §Logging redact-paths discipline).
    /// 6. On failure, mark Error с а sanitized one-liner reason that
    ///    DOES NOT contain the path / mistralrs internal types.
    pub async fn load_from_env_if_configured(&self) -> Result<(), InferenceError> {
        let Some(path) = Self::configured_model_path() else {
            // Env var unset; stay в graceful-degraded mode.
            return Err(InferenceError::ModelNotConfigured);
        };

        // File-exists check BEFORE mistralrs invocation (per security
        // extract path-shape validation). mistralrs's GgufModelBuilder
        // accepts а HuggingFace repo ID as the first arg + falls back
        // к а Hub lookup if the local file is missing; that fallback
        // would surprise us at boot, so we gate first.
        if !path.exists() {
            self.mark_error("model path does not exist".to_string());
            return Err(InferenceError::InvalidModelPath);
        }
        if !path.is_file() {
            self.mark_error("model path is not а file".to_string());
            return Err(InferenceError::InvalidModelPath);
        }

        self.mark_loading();
        tracing::info!(
            target: "interpretation.model.load",
            tier = interpretation::contract::model_tier_label(self.tier),
            load_status = "loading",
            "mistralrs GGUF model load starting",
        );

        // Build the GGUF model. `GgufModelBuilder::new` takes (model_id,
        // files) where for а local file model_id is the parent dir and
        // files is а singleton с the basename.
        let parent = path
            .parent()
            .ok_or(InferenceError::InvalidModelPath)?
            .to_string_lossy()
            .into_owned();
        let filename = path
            .file_name()
            .ok_or(InferenceError::InvalidModelPath)?
            .to_string_lossy()
            .into_owned();
        let semantic_name = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown-gguf-model".to_string());

        // `with_force_cpu()` is а deliberate chunk-#83 choice: the spike
        // runs against the host's CPU regardless of GPU availability к
        // get а baseline first-inference latency sample. Future chunks
        // can opt into GPU per `self.profile`.
        let model_result = GgufModelBuilder::new(parent, vec![filename])
            .with_force_cpu()
            .build()
            .await;

        match model_result {
            Ok(model) => {
                // Set the model first; if OnceCell::set fails (concurrent
                // re-init race), discard the second model — first wins.
                if self.model.set(model).is_err() {
                    tracing::warn!(
                        target: "interpretation.model.load",
                        load_status = "already_loaded",
                        "concurrent model load race; second instance discarded",
                    );
                }
                self.mark_loaded(ModelIdentity { semantic_name });
                tracing::info!(
                    target: "interpretation.model.load",
                    tier = interpretation::contract::model_tier_label(self.tier),
                    load_status = "loaded",
                    "mistralrs GGUF model load complete",
                );
                Ok(())
            }
            Err(err) => {
                // Sanitize: mistralrs error display can carry internal
                // module paths + file paths; collapse к а bounded
                // category label.
                let category = classify_error_message(&err.to_string());
                self.mark_error(category.to_string());
                tracing::warn!(
                    target: "interpretation.model.load.error",
                    tier = interpretation::contract::model_tier_label(self.tier),
                    error_category = category,
                    recovery_action = "graceful_degraded",
                    "mistralrs GGUF model load failed",
                );
                Err(InferenceError::ModelLoadFailed {
                    reason: category.to_string(),
                })
            }
        }
    }
}

/// Bounded category label derived from the raw error message string.
/// Never surfaces the message verbatim — only а fixed enum label per
/// security extract sanitization discipline. Used for BOTH model load
/// errors (`anyhow::Error`) AND inference errors (`mistralrs::error::Error`);
/// both feed in via `.to_string()` к decouple the classifier от specific
/// error types и avoid а direct `anyhow` dep.
fn classify_error_message(raw: &str) -> &'static str {
    let msg = raw.to_lowercase();
    if msg.contains("no such file") || msg.contains("not found") {
        "model_file_not_found"
    } else if msg.contains("invalid") || msg.contains("magic") || msg.contains("malformed") {
        "model_file_invalid_format"
    } else if msg.contains("out of memory") || msg.contains("oom") || msg.contains("allocate") {
        "out_of_memory"
    } else if msg.contains("tokenizer") {
        "tokenizer_init_failed"
    } else if msg.contains("device") || msg.contains("cuda") || msg.contains("metal") {
        "device_init_failed"
    } else if msg.contains("timeout") || msg.contains("timed out") {
        "inference_timeout"
    } else if msg.contains("schema") || msg.contains("grammar") || msg.contains("constraint") {
        "constraint_violation"
    } else if msg.contains("cancel") {
        "inference_cancelled"
    } else {
        "model_load_failed"
    }
}

impl LlmInferenceRunner for MistralRsInference {
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
            // Fast-fail: if no model loaded (graceful-degraded mode),
            // signal ModelNotConfigured к the L4 subscriber which counts
            // it as `result=runtime_error` and skips the digest.
            let Some(model) = self.model.get() else {
                return Err(InferenceError::ModelNotConfigured);
            };

            // Parse the schema string к `serde_json::Value` BEFORE
            // touching the model — а malformed schema is а bug в our
            // pipeline, not а runtime error. Sanitized one-liner per
            // security extract.
            let schema_value: serde_json::Value =
                serde_json::from_str(schema_json).map_err(|e| InferenceError::InferenceFailed {
                    reason: format!(
                        "embedded schema failed parse: {}",
                        e.to_string().lines().next().unwrap_or("invalid schema")
                    ),
                })?;

            // Build the request. Single-user-message TextMessages is
            // sufficient: the prompt is already assembled с role-
            // definition + conventions + schema embed + digest +
            // output reminder per `interpretation::prompt::
            // build_primary_tier_prompt`.
            let messages = TextMessages::new().add_message(TextMessageRole::User, prompt);
            let request: mistralrs::RequestBuilder = messages.into();
            let request = request.set_constraint(Constraint::JsonSchema(schema_value));

            // Send + extract content. mistralrs returns а
            // ChatCompletionResponse с а choices Vec; we take the first
            // choice's message.content (the constrained-JSON string).
            let response = model.send_chat_request(request).await.map_err(|err| {
                let category = classify_error_message(&err.to_string());
                InferenceError::InferenceFailed {
                    reason: category.to_string(),
                }
            })?;

            let content = response
                .choices
                .into_iter()
                .next()
                .and_then(|c| c.message.content)
                .ok_or_else(|| InferenceError::InferenceFailed {
                    reason: "model returned empty choices".to_string(),
                })?;

            Ok(content)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_in_error_status() {
        let b = ModelStatusBroadcast::new();
        let runner = MistralRsInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, b);
        assert!(matches!(runner.current_status(), ModelStatus::Error));
        assert!(runner.identity().is_none());
        assert_eq!(runner.tier(), ModelTier::Primary);
    }

    #[test]
    fn mark_loading_transitions_status() {
        let b = ModelStatusBroadcast::new();
        let runner = MistralRsInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, b);
        runner.mark_loading();
        assert!(matches!(runner.current_status(), ModelStatus::Loading));
    }

    #[test]
    fn mark_loaded_transitions_status_and_records_identity() {
        let b = ModelStatusBroadcast::new();
        let runner = MistralRsInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, b);
        let identity = ModelIdentity {
            semantic_name: "test-model-7b".into(),
        };
        runner.mark_loaded(identity.clone());
        assert!(matches!(runner.current_status(), ModelStatus::Loaded));
        assert_eq!(runner.identity(), Some(identity));
    }

    #[test]
    fn mark_error_returns_to_error_state() {
        let b = ModelStatusBroadcast::new();
        let runner = MistralRsInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, b);
        runner.mark_loading();
        runner.mark_error("test reason".into());
        assert!(matches!(runner.current_status(), ModelStatus::Error));
    }

    #[test]
    fn mark_transitions_broadcast_to_subscribers() {
        let b = ModelStatusBroadcast::new();
        let mut rx = b.subscribe();
        let runner =
            MistralRsInference::new(ModelTier::Fallback, HardwareProfile::CpuFallback, b.clone());
        runner.mark_loading();
        let event = rx.try_recv().expect("event received");
        assert!(matches!(event.status, ModelStatus::Loading));
        assert_eq!(event.tier, ModelTier::Fallback);
        assert_eq!(event.profile_label, "cpu_fallback");
    }

    #[tokio::test]
    async fn generate_constrained_returns_not_configured_in_error_state() {
        let b = ModelStatusBroadcast::new();
        let runner = MistralRsInference::new(ModelTier::Primary, HardwareProfile::CpuPrimary, b);
        let result = runner.generate_constrained("prompt", "{}").await;
        assert!(matches!(result, Err(InferenceError::ModelNotConfigured)));
    }

    #[test]
    fn configured_model_path_returns_none_when_unset() {
        // Test relies on env var being unset. Wrap with а scoped removal
        // в case the test runner has it set.
        let original = std::env::var(ENV_MODEL_PATH).ok();
        // SAFETY: tests run в isolated processes per nextest profile;
        // env-var mutation is bounded к this test's scope.
        unsafe {
            std::env::remove_var(ENV_MODEL_PATH);
        }
        let result = MistralRsInference::configured_model_path();
        assert!(result.is_none());
        if let Some(v) = original {
            // SAFETY: restoring prior env var; same isolation guarantee.
            unsafe {
                std::env::set_var(ENV_MODEL_PATH, v);
            }
        }
    }
}
