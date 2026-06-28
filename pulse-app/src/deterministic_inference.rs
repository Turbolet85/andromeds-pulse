//! Deterministic env-gated L4 inference runner (P-073): a binary-boundary
//! `LlmInferenceRunner` impl returning a fixed schema-valid canned `L4Output`
//! JSON — no GPU / model / subprocess — so the digest -> L4 -> incident chain
//! completes reproducibly for demos, tests, and external (Conductor)
//! verification. Selected at boot by `main.rs` when
//! `ANDROMEDA_PULSE_L4_DETERMINISTIC` is truthy; `LlamaCliInference` is the
//! real-mode runner otherwise. The canned output flows through the existing
//! `inference_runtime` parse -> `create_incident_from_l4_output` path unchanged.
//! Models the test `StubInferenceRunner` pattern, productionized.

use interpretation::contract::{
    InferenceFuture, LlmInferenceRunner, ModelIdentity, ModelStatus, ModelTier,
};

/// Env var gating the deterministic L4 mode. A truthy value (`1` / `true` /
/// `yes`, case-insensitive, surrounding whitespace trimmed) selects the
/// canned-output runner at boot.
pub const ENV_L4_DETERMINISTIC: &str = "ANDROMEDA_PULSE_L4_DETERMINISTIC";

/// Fixed canned L4 output. Schema-valid per `interpretation::schema`
/// (`schema_version` 2.0; `decision: surface` + `severity: autonomous` ->
/// `IncidentSeverity::Error`) so a cue-bearing digest yields exactly one
/// red-dot incident, reproducibly. First-party constants only — no PII, no
/// real telemetry (a no-canary unit test guards this).
pub const CANNED_L4_OUTPUT_JSON: &str = r#"{
  "schema_version": "2.0",
  "prompt_version": "v2.1",
  "decision": "surface",
  "severity": "autonomous",
  "title": "Deterministic verification incident",
  "symptom": "Synthetic incident emitted by the deterministic L4 mode for reproducible verification.",
  "timeline": "Deterministic mode active; no model inference was performed.",
  "hypotheses": [],
  "investigation_steps": [],
  "evidence_refs": [],
  "fingerprint": "deterministic-l4-fixture",
  "model_tier": "primary",
  "hardware_profile": "cpu_primary",
  "is_resolution_summary": false
}"#;

/// Returns `true` when [`ENV_L4_DETERMINISTIC`] is set to a truthy value.
pub fn deterministic_mode_enabled() -> bool {
    deterministic_mode_enabled_for(std::env::var(ENV_L4_DETERMINISTIC).ok().as_deref())
}

/// Pure truthy classification of a raw env value (extracted so unit tests
/// avoid process-global env mutation).
pub fn deterministic_mode_enabled_for(value: Option<&str>) -> bool {
    matches!(
        value.map(|v| v.trim().to_ascii_lowercase()).as_deref(),
        Some("1" | "true" | "yes")
    )
}

/// Deterministic `LlmInferenceRunner`: always `Loaded`, returns
/// [`CANNED_L4_OUTPUT_JSON`] regardless of prompt or schema, never spawns a
/// subprocess.
pub struct DeterministicInferenceRunner {
    tier: ModelTier,
}

impl DeterministicInferenceRunner {
    pub fn new(tier: ModelTier) -> Self {
        Self { tier }
    }
}

impl LlmInferenceRunner for DeterministicInferenceRunner {
    fn current_status(&self) -> ModelStatus {
        ModelStatus::Loaded
    }

    fn identity(&self) -> Option<ModelIdentity> {
        Some(ModelIdentity {
            semantic_name: "deterministic-stub".to_string(),
        })
    }

    fn tier(&self) -> ModelTier {
        self.tier
    }

    fn generate_constrained<'a>(
        &'a self,
        _prompt: &'a str,
        _schema_json: &'a str,
    ) -> InferenceFuture<'a, String> {
        Box::pin(async move { Ok(CANNED_L4_OUTPUT_JSON.to_string()) })
    }
}
