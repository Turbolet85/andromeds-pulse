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
///
/// The array fields are POPULATED, not empty, and that is load-bearing:
/// `evidence_refs` reaches `Incident.evidence_refs.fingerprint_hashes` (the
/// sole join, `inference_runtime::create_incident_from_l4_output`) and from
/// there both the MCP `retrieve_telemetry_slice` response and the Report's
/// Evidence section. An empty vector makes every payload-identity assertion
/// against those surfaces compare nothing and pass — and makes an ABSENCE
/// check ("no evidence refs leaked") pass for the wrong reason, so it would
/// keep passing after a real leak. Values are bounded well inside
/// `schema::EVIDENCE_REFS_MAX` / `EVIDENCE_REF_MAX_LEN`, carry no digit run
/// long enough to trip the scrubber's card pattern, and carry no secret-KV
/// label — so they survive the two `scrub_string` passes intact rather than
/// arriving as `[redacted]`.
pub const CANNED_L4_OUTPUT_JSON: &str = r#"{
  "schema_version": "2.0",
  "prompt_version": "v2.1",
  "decision": "surface",
  "severity": "autonomous",
  "title": "Deterministic verification incident",
  "symptom": "Synthetic incident emitted by the deterministic L4 mode for reproducible verification.",
  "timeline": "Deterministic mode active; no model inference was performed.",
  "hypotheses": [
    {
      "statement": "Deterministic fixture hypothesis: the retry storm originates in the synthetic verification scope.",
      "confidence": "high",
      "justification": "Fixture justification emitted by the deterministic L4 mode; no inference was performed."
    },
    {
      "statement": "Deterministic fixture hypothesis: a downstream dependency amplified the synthetic failure rate.",
      "confidence": "low",
      "justification": "Second fixture justification, present so a multi-hypothesis render is exercised."
    }
  ],
  "investigation_steps": [
    {
      "step": "Deterministic fixture step: confirm the verification scope reported the synthetic exception.",
      "expected_yield": "Fixture expected yield; the step is a render fixture, not a real instruction."
    },
    {
      "step": "Deterministic fixture step: compare the synthetic occurrence count against the storm threshold.",
      "expected_yield": "Second fixture expected yield, present so an ordered multi-step render is exercised."
    }
  ],
  "evidence_refs": [
    "det-span-9f2c4a7e1b6d0358",
    "det-template-0007",
    "det-fingerprint-4a7f2b91c6e05d3849b1e7a2c5f08d63"
  ],
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
