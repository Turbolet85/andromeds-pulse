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
use triage::contract::HardwareProfile;

/// Env var configuring the on-disk model file path. Canonicalized via
/// `strict-path` before mistralrs load.
pub const ENV_MODEL_PATH: &str = "ANDROMEDA_PULSE_MODEL_PATH";

/// Concrete impl of `LlmInferenceRunner`. Holds the configured tier
/// (set at boot per hardware profile) + the current lifecycle status
/// (updated by load/unload/error transitions) + the broadcast sender
/// for emitting lifecycle events.
pub struct MistralRsInference {
    tier: ModelTier,
    state: Arc<RwLock<InferenceState>>,
    broadcast: ModelStatusBroadcast,
    profile: HardwareProfile,
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
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn mark_loading(&self) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Loading;
        }
        self.emit_event(ModelStatus::Loading, None, None);
    }

    /// Marks the runner as Loaded with the given identity + emits an
    /// event. Chunk #83+ wires this from successful load completion.
    #[cfg_attr(not(test), allow(dead_code))]
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
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn mark_error(&self, sanitized_reason: String) {
        {
            let mut s = self.state.write().expect("state lock poisoned");
            s.status = ModelStatus::Error;
        }
        self.emit_event(ModelStatus::Error, None, Some(sanitized_reason));
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
        _prompt: &'a str,
        _schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        // Chunk #82 spike scope: returns `ModelNotConfigured` until the
        // mistralrs strict-schema-mode load path is wired in chunks #83+.
        // This signature exists today к validate the trait surface compiles
        // + к provide а stable Arc<dyn LlmInferenceRunner> shape consumed
        // by `model_router::ModelApiImpl`.
        Box::pin(async move {
            let status = self.current_status();
            if matches!(status, ModelStatus::Error) {
                Err(InferenceError::ModelNotConfigured)
            } else {
                // Reserved for chunk #83+ (actual inference invocation).
                Err(InferenceError::InferenceFailed {
                    reason: "inference not implemented in chunk #82 (spike scope)".into(),
                })
            }
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
