//! L4 LLM interpretation output schema (chunk #83 — Epoch 9 Foundation v0.2.0).
//!
//! `L4Output` struct + supporting types per pulse-distillation-architecture.md
//! §L4 output format. The embedded JSON schema (`L4_OUTPUT_JSON_SCHEMA`)
//! drives mistralrs strict-schema mode constrained generation; defense-in-
//! depth Rust-side post-parse validation enforces bounded-length + bounded-
//! array invariants on top of the schema-level structural enforcement.
//!
//! Capabilities anchored: P-019 (Three-Tier Severity Model — model
//! decision side via `Severity`), P-020 (Model-Driven Severity Decision —
//! `Decision` + `Severity`), P-033 (Ranked Hypothesis Generation —
//! `Hypothesis` array), P-034 (Suggested Investigation Steps —
//! `InvestigationStep` array).

use serde::{Deserialize, Serialize};

use crate::contract::InferenceError;

/// Embedded JSON schema string driving mistralrs strict-schema-mode
/// constrained generation. Compile-time-loaded from sibling JSON file
/// so the schema document stays human-reviewable.
pub const L4_OUTPUT_JSON_SCHEMA: &str = include_str!("schema.json");

/// Schema version embedded in every emitted payload per dist-arch v3 §L4.
pub const SCHEMA_VERSION: &str = "2.0";

/// Prompt template version. Primary-tier prompt assembled by
/// [`crate::prompt::build_primary_tier_prompt`] (chunk #83).
pub const PROMPT_VERSION_PRIMARY: &str = "v2.1";

/// Prompt template version. Fallback-tier prompt assembled by
/// [`crate::prompt::build_fallback_tier_prompt`] (chunk #85 — Epoch 9
/// Foundation v0.2.0). Distinct namespace from primary's `v2.1` lineage;
/// future fallback prompt iterations bump к `"v1.1-fallback"` etc.
pub const PROMPT_VERSION_FALLBACK: &str = "v1.0-fallback";

/// Prompt template version. Reflection-tier prompt assembled by
/// [`crate::prompt::build_reflection_tier_prompt`] (chunk #98 — Epoch 9
/// Foundation v0.2.0). Cumulative-pattern-emphasis variant of the
/// primary prompt for the 30-minute background reflection window;
/// distinct namespace from primary's `v2.1` + fallback's `v1.0-fallback`
/// lineages. Reflection runs at primary-tier quality (the model emits
/// `model_tier: "primary"`); it is NOT a fallback-tier prompt.
pub const PROMPT_VERSION_REFLECTION: &str = "v1.0-reflection";

/// Defense-in-depth pre-parse cap on raw inference output bytes.
/// mistralrs strict-schema-mode caps total tokens, but the byte budget
/// is the canonical untrusted-input boundary check per security plan
/// §Anti-Patterns Code Patterns serde_json+size-cap rule.
pub const L4_OUTPUT_MAX_BYTES: usize = 16 * 1024;

/// Maximum length (chars) of the free-text `title` field.
pub const TITLE_MAX_LEN: usize = 200;

/// Maximum length (chars) of the free-text `symptom` field.
pub const SYMPTOM_MAX_LEN: usize = 1000;

/// Maximum length (chars) of the free-text `timeline` field.
pub const TIMELINE_MAX_LEN: usize = 1500;

/// Maximum length (chars) of the free-text `statement` / `justification`
/// fields inside `Hypothesis`.
pub const HYPOTHESIS_STATEMENT_MAX_LEN: usize = 500;

/// Maximum length (chars) of the free-text `step` / `expected_yield`
/// fields inside `InvestigationStep`.
pub const INVESTIGATION_STEP_MAX_LEN: usize = 500;

/// Maximum count of ranked hypotheses per output. Primary tier emits up
/// to this; fallback tier (chunk #85) emits 1.
pub const HYPOTHESES_MAX: usize = 5;

/// Maximum count of investigation steps per output. Primary tier emits
/// up to this; fallback tier (chunk #85) emits up to 2.
pub const INVESTIGATION_STEPS_MAX: usize = 5;

/// Maximum count of hypotheses for fallback-tier outputs (chunk #85).
/// Enforced как post-parse defense-in-depth on top of the prompt-level
/// constraint per P-053 reduced-quality contract.
pub const FALLBACK_HYPOTHESES_MAX: usize = 1;

/// Maximum count of investigation steps for fallback-tier outputs
/// (chunk #85). Enforced как post-parse defense-in-depth on top of the
/// prompt-level constraint per P-053 reduced-quality contract.
pub const FALLBACK_INVESTIGATION_STEPS_MAX: usize = 2;

// Compile-time invariant: fallback bounds MUST be strictly smaller than
// primary bounds per P-053 reduced-quality contract. Const-block per
// CLAUDE.md testing.md 2026-05-11 pattern (clippy::assertions_on_constants
// rejects the assertion inside а `#[test]` fn).
const _: () = {
    assert!(FALLBACK_HYPOTHESES_MAX < HYPOTHESES_MAX);
    assert!(FALLBACK_INVESTIGATION_STEPS_MAX < INVESTIGATION_STEPS_MAX);
};

/// Maximum count of evidence references per output.
pub const EVIDENCE_REFS_MAX: usize = 32;

/// Maximum length (chars) of a single evidence reference string.
pub const EVIDENCE_REF_MAX_LEN: usize = 256;

/// Maximum length (chars) of the `fingerprint` field.
pub const FINGERPRINT_MAX_LEN: usize = 256;

/// Surface decision emitted by the model per dist-arch v3 §L4 output
/// format. Bounded enum per security plan §Input Validation row
/// TauRPC bridge serde+smart-enum discipline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Surface,
    Dismiss,
    Watch,
}

/// Severity classification per P-019 Three-Tier Severity Model + the
/// `none` baseline value covering ambiguous-but-not-dismissed cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Autonomous,
    Suggested,
    Curious,
    None,
}

/// Hypothesis confidence band. Bounded к three labels per dist-arch v3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

/// One ranked hypothesis with confidence + justification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hypothesis {
    pub statement: String,
    pub confidence: Confidence,
    pub justification: String,
}

/// One suggested investigation step with expected yield description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestigationStep {
    pub step: String,
    pub expected_yield: String,
}

/// Structured L4 output payload per dist-arch v3 §L4 output format.
///
/// `model_tier` and `hardware_profile` are stored as bounded `String`
/// labels rather than re-exported `triage::contract::HardwareProfile`
/// enums per CLAUDE.md §Session Learnings 2026-05-24 cross-crate serde
/// pattern — keeps the schema independent of the triage crate's enum
/// derive surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct L4Output {
    pub schema_version: String,
    pub prompt_version: String,
    pub decision: Decision,
    pub severity: Severity,
    pub title: String,
    pub symptom: String,
    pub timeline: String,
    pub hypotheses: Vec<Hypothesis>,
    pub investigation_steps: Vec<InvestigationStep>,
    pub evidence_refs: Vec<String>,
    pub fingerprint: String,
    pub model_tier: String,
    pub hardware_profile: String,
    pub is_resolution_summary: bool,
}

/// Defense-in-depth Rust-side post-parse validation enforcing bounded-
/// length + bounded-array invariants on top of the JSON-schema-level
/// structural enforcement.
///
/// Returns the first detected violation as `InferenceError::SchemaViolation`.
/// Order of checks is stable (length-bounded fields first, then arrays,
/// then enum values) so test fixtures can predict the surfaced reason.
pub fn validate(output: &L4Output) -> Result<(), InferenceError> {
    if output.schema_version != SCHEMA_VERSION {
        return Err(InferenceError::SchemaViolation {
            reason: format!(
                "schema_version mismatch: expected {SCHEMA_VERSION}, got {}",
                output.schema_version
            ),
        });
    }
    if output.prompt_version.is_empty() || output.prompt_version.len() > 32 {
        return Err(InferenceError::SchemaViolation {
            reason: "prompt_version length out of bounds (1..=32)".into(),
        });
    }
    if output.title.is_empty() || output.title.len() > TITLE_MAX_LEN {
        return Err(InferenceError::SchemaViolation {
            reason: format!("title length out of bounds (1..={TITLE_MAX_LEN})"),
        });
    }
    if output.symptom.len() > SYMPTOM_MAX_LEN {
        return Err(InferenceError::SchemaViolation {
            reason: format!("symptom length exceeds bound ({SYMPTOM_MAX_LEN})"),
        });
    }
    if output.timeline.len() > TIMELINE_MAX_LEN {
        return Err(InferenceError::SchemaViolation {
            reason: format!("timeline length exceeds bound ({TIMELINE_MAX_LEN})"),
        });
    }
    if output.hypotheses.len() > HYPOTHESES_MAX {
        return Err(InferenceError::SchemaViolation {
            reason: format!(
                "hypotheses count {} exceeds bound ({HYPOTHESES_MAX})",
                output.hypotheses.len()
            ),
        });
    }
    for (idx, h) in output.hypotheses.iter().enumerate() {
        if h.statement.is_empty() || h.statement.len() > HYPOTHESIS_STATEMENT_MAX_LEN {
            return Err(InferenceError::SchemaViolation {
                reason: format!("hypotheses[{idx}].statement length out of bounds"),
            });
        }
        if h.justification.len() > HYPOTHESIS_STATEMENT_MAX_LEN {
            return Err(InferenceError::SchemaViolation {
                reason: format!("hypotheses[{idx}].justification length exceeds bound"),
            });
        }
    }
    if output.investigation_steps.len() > INVESTIGATION_STEPS_MAX {
        return Err(InferenceError::SchemaViolation {
            reason: format!(
                "investigation_steps count {} exceeds bound ({INVESTIGATION_STEPS_MAX})",
                output.investigation_steps.len()
            ),
        });
    }
    for (idx, s) in output.investigation_steps.iter().enumerate() {
        if s.step.is_empty() || s.step.len() > INVESTIGATION_STEP_MAX_LEN {
            return Err(InferenceError::SchemaViolation {
                reason: format!("investigation_steps[{idx}].step length out of bounds"),
            });
        }
        if s.expected_yield.len() > INVESTIGATION_STEP_MAX_LEN {
            return Err(InferenceError::SchemaViolation {
                reason: format!("investigation_steps[{idx}].expected_yield length exceeds bound"),
            });
        }
    }
    if output.evidence_refs.len() > EVIDENCE_REFS_MAX {
        return Err(InferenceError::SchemaViolation {
            reason: format!(
                "evidence_refs count {} exceeds bound ({EVIDENCE_REFS_MAX})",
                output.evidence_refs.len()
            ),
        });
    }
    for (idx, r) in output.evidence_refs.iter().enumerate() {
        if r.len() > EVIDENCE_REF_MAX_LEN {
            return Err(InferenceError::SchemaViolation {
                reason: format!("evidence_refs[{idx}] length exceeds bound"),
            });
        }
    }
    if output.fingerprint.is_empty() || output.fingerprint.len() > FINGERPRINT_MAX_LEN {
        return Err(InferenceError::SchemaViolation {
            reason: format!("fingerprint length out of bounds (1..={FINGERPRINT_MAX_LEN})"),
        });
    }
    match output.model_tier.as_str() {
        "primary" | "fallback" => {}
        other => {
            return Err(InferenceError::SchemaViolation {
                reason: format!("model_tier label {other:?} not in bounded set"),
            });
        }
    }
    // Fallback-tier reduced-quality contract per P-053 (chunk #85): cap
    // hypotheses + investigation steps below primary's bounds. Defense-in-
    // depth on top of the prompt-level instruction в OUTPUT_REMINDER_FALLBACK.
    if output.model_tier == "fallback" {
        if output.hypotheses.len() > FALLBACK_HYPOTHESES_MAX {
            return Err(InferenceError::SchemaViolation {
                reason: format!(
                    "fallback tier hypotheses count {} exceeds bound ({FALLBACK_HYPOTHESES_MAX})",
                    output.hypotheses.len()
                ),
            });
        }
        if output.investigation_steps.len() > FALLBACK_INVESTIGATION_STEPS_MAX {
            return Err(InferenceError::SchemaViolation {
                reason: format!(
                    "fallback tier investigation_steps count {} exceeds bound ({FALLBACK_INVESTIGATION_STEPS_MAX})",
                    output.investigation_steps.len()
                ),
            });
        }
    }
    match output.hardware_profile.as_str() {
        "gpu_primary" | "gpu_fallback" | "cpu_primary" | "cpu_fallback" => {}
        other => {
            return Err(InferenceError::SchemaViolation {
                reason: format!("hardware_profile label {other:?} not in bounded set"),
            });
        }
    }
    Ok(())
}

/// Parses raw inference output bytes into a validated `L4Output`.
///
/// Two-stage discipline:
/// 1. Size cap rejection BEFORE `serde_json::from_slice` allocates parse
///    buffer (per security plan §Anti-Pattern Code Patterns).
/// 2. Structural deserialize through `serde_json`, then post-parse
///    [`validate`] for bounded-length + bounded-array enforcement.
pub fn parse_bounded(bytes: &[u8]) -> Result<L4Output, InferenceError> {
    if bytes.len() > L4_OUTPUT_MAX_BYTES {
        return Err(InferenceError::OutputTooLarge {
            actual_bytes: bytes.len(),
            max_bytes: L4_OUTPUT_MAX_BYTES,
        });
    }
    let parsed: L4Output =
        serde_json::from_slice(bytes).map_err(|e| InferenceError::JsonParseFailed {
            reason: sanitize_serde_error(&e),
        })?;
    validate(&parsed)?;
    Ok(parsed)
}

/// Strips line/column noise from serde_json error messages so the
/// surfaced `reason` field stays bounded and content-free per security
/// plan §Anti-Pattern Logging redaction discipline. Only the first
/// sentence (up to first newline) survives.
fn sanitize_serde_error(err: &serde_json::Error) -> String {
    let raw = err.to_string();
    raw.lines().next().unwrap_or("invalid JSON").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_sample() -> L4Output {
        L4Output {
            schema_version: SCHEMA_VERSION.into(),
            prompt_version: PROMPT_VERSION_PRIMARY.into(),
            decision: Decision::Surface,
            severity: Severity::Suggested,
            title: "Database connection saturation".into(),
            symptom: "Connection pool exhaustion observed in service-a".into(),
            timeline: "Saturation began at 12:34Z; recovered at 12:40Z".into(),
            hypotheses: vec![Hypothesis {
                statement: "Pool size too small for current load".into(),
                confidence: Confidence::High,
                justification: "Pool size 10 vs observed 40 req/sec".into(),
            }],
            investigation_steps: vec![InvestigationStep {
                step: "Inspect pool config".into(),
                expected_yield: "Confirm max_connections setting".into(),
            }],
            evidence_refs: vec!["span:abc123".into()],
            fingerprint: "db-saturation:service-a".into(),
            model_tier: "primary".into(),
            hardware_profile: "cpu_primary".into(),
            is_resolution_summary: false,
        }
    }

    #[test]
    fn schema_version_constant_matches_json_schema() {
        assert!(L4_OUTPUT_JSON_SCHEMA.contains(SCHEMA_VERSION));
    }

    #[test]
    fn embedded_schema_includes_required_fields() {
        for field in [
            "schema_version",
            "prompt_version",
            "decision",
            "severity",
            "title",
            "symptom",
            "timeline",
            "hypotheses",
            "investigation_steps",
            "evidence_refs",
            "fingerprint",
            "model_tier",
            "hardware_profile",
            "is_resolution_summary",
        ] {
            assert!(
                L4_OUTPUT_JSON_SCHEMA.contains(field),
                "embedded schema missing required field reference: {field}"
            );
        }
    }

    #[test]
    fn valid_sample_round_trips_through_serde() {
        let original = valid_sample();
        let json = serde_json::to_string(&original).expect("serialize ok");
        let parsed: L4Output = serde_json::from_str(&json).expect("deserialize ok");
        assert_eq!(parsed, original);
    }

    #[test]
    fn valid_sample_passes_validate() {
        let sample = valid_sample();
        validate(&sample).expect("valid sample passes validation");
    }

    #[test]
    fn parse_bounded_rejects_oversize_payload() {
        let payload = vec![b'a'; L4_OUTPUT_MAX_BYTES + 1];
        match parse_bounded(&payload) {
            Err(InferenceError::OutputTooLarge {
                actual_bytes,
                max_bytes,
            }) => {
                assert_eq!(actual_bytes, L4_OUTPUT_MAX_BYTES + 1);
                assert_eq!(max_bytes, L4_OUTPUT_MAX_BYTES);
            }
            other => panic!("expected OutputTooLarge, got {other:?}"),
        }
    }

    #[test]
    fn parse_bounded_accepts_valid_payload() {
        let sample = valid_sample();
        let json = serde_json::to_vec(&sample).expect("serialize ok");
        let parsed = parse_bounded(&json).expect("parse ok");
        assert_eq!(parsed, sample);
    }

    #[test]
    fn parse_bounded_rejects_malformed_json() {
        let bytes = br#"{ this is not json }"#;
        match parse_bounded(bytes) {
            Err(InferenceError::JsonParseFailed { reason }) => {
                assert!(!reason.is_empty());
                assert!(!reason.contains('\n'));
            }
            other => panic!("expected JsonParseFailed, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_wrong_schema_version() {
        let mut sample = valid_sample();
        sample.schema_version = "9.9".into();
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("schema_version"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_empty_title() {
        let mut sample = valid_sample();
        sample.title.clear();
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("title"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_oversized_title() {
        let mut sample = valid_sample();
        sample.title = "x".repeat(TITLE_MAX_LEN + 1);
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("title"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_too_many_hypotheses() {
        let mut sample = valid_sample();
        sample.hypotheses = (0..HYPOTHESES_MAX + 1)
            .map(|i| Hypothesis {
                statement: format!("stmt {i}"),
                confidence: Confidence::Medium,
                justification: "".into(),
            })
            .collect();
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("hypotheses"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_too_many_investigation_steps() {
        let mut sample = valid_sample();
        sample.investigation_steps = (0..INVESTIGATION_STEPS_MAX + 1)
            .map(|i| InvestigationStep {
                step: format!("step {i}"),
                expected_yield: "".into(),
            })
            .collect();
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("investigation_steps"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_unknown_model_tier() {
        let mut sample = valid_sample();
        sample.model_tier = "experimental".into();
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("model_tier"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_unknown_hardware_profile() {
        let mut sample = valid_sample();
        sample.hardware_profile = "npu_primary".into();
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("hardware_profile"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn decision_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&Decision::Surface).unwrap(),
            "\"surface\""
        );
        assert_eq!(
            serde_json::to_string(&Decision::Dismiss).unwrap(),
            "\"dismiss\""
        );
        assert_eq!(
            serde_json::to_string(&Decision::Watch).unwrap(),
            "\"watch\""
        );
    }

    #[test]
    fn severity_serializes_to_snake_case() {
        for (variant, expected) in [
            (Severity::Autonomous, "\"autonomous\""),
            (Severity::Suggested, "\"suggested\""),
            (Severity::Curious, "\"curious\""),
            (Severity::None, "\"none\""),
        ] {
            assert_eq!(serde_json::to_string(&variant).unwrap(), expected);
        }
    }

    #[test]
    fn confidence_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&Confidence::High).unwrap(),
            "\"high\""
        );
        assert_eq!(
            serde_json::to_string(&Confidence::Medium).unwrap(),
            "\"medium\""
        );
        assert_eq!(serde_json::to_string(&Confidence::Low).unwrap(), "\"low\"");
    }

    #[test]
    fn parse_bounded_rejects_decision_out_of_range() {
        let mut sample = serde_json::to_value(valid_sample()).expect("serialize ok");
        sample["decision"] = serde_json::Value::String("explode".into());
        let bytes = serde_json::to_vec(&sample).expect("serialize ok");
        match parse_bounded(&bytes) {
            Err(InferenceError::JsonParseFailed { reason }) => {
                assert!(reason.contains("decision") || reason.contains("variant"));
            }
            other => panic!("expected JsonParseFailed, got {other:?}"),
        }
    }

    #[test]
    fn parse_bounded_rejects_missing_required_field() {
        let mut sample = serde_json::to_value(valid_sample()).expect("serialize ok");
        sample
            .as_object_mut()
            .expect("object")
            .remove("schema_version");
        let bytes = serde_json::to_vec(&sample).expect("serialize ok");
        match parse_bounded(&bytes) {
            Err(InferenceError::JsonParseFailed { reason }) => {
                assert!(reason.contains("schema_version") || reason.contains("missing"));
            }
            other => panic!("expected JsonParseFailed, got {other:?}"),
        }
    }

    // ---- Fallback-tier validation coverage (chunk #85) ----

    fn fallback_sample() -> L4Output {
        let mut sample = valid_sample();
        sample.model_tier = "fallback".into();
        sample.prompt_version = PROMPT_VERSION_FALLBACK.into();
        sample
    }

    #[test]
    fn validate_accepts_fallback_with_single_hypothesis() {
        let sample = fallback_sample();
        validate(&sample).expect("fallback с 1 hypothesis + 1 investigation step passes");
    }

    #[test]
    fn validate_accepts_fallback_with_two_investigation_steps() {
        let mut sample = fallback_sample();
        sample.investigation_steps = vec![
            InvestigationStep {
                step: "step one".into(),
                expected_yield: "yield one".into(),
            },
            InvestigationStep {
                step: "step two".into(),
                expected_yield: "yield two".into(),
            },
        ];
        validate(&sample).expect("fallback с 1 hypothesis + 2 investigation steps passes");
    }

    #[test]
    fn validate_rejects_fallback_with_too_many_hypotheses() {
        let mut sample = fallback_sample();
        sample.hypotheses = vec![
            Hypothesis {
                statement: "first hypothesis".into(),
                confidence: Confidence::High,
                justification: "first justification".into(),
            },
            Hypothesis {
                statement: "second hypothesis".into(),
                confidence: Confidence::Medium,
                justification: "second justification".into(),
            },
        ];
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("fallback"));
                assert!(reason.contains("hypotheses"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_rejects_fallback_with_too_many_investigation_steps() {
        let mut sample = fallback_sample();
        sample.investigation_steps = vec![
            InvestigationStep {
                step: "step one".into(),
                expected_yield: "yield one".into(),
            },
            InvestigationStep {
                step: "step two".into(),
                expected_yield: "yield two".into(),
            },
            InvestigationStep {
                step: "step three".into(),
                expected_yield: "yield three".into(),
            },
        ];
        match validate(&sample) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("fallback"));
                assert!(reason.contains("investigation_steps"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    #[test]
    fn validate_accepts_primary_with_five_hypotheses() {
        // Regression guard: tier-conditional fallback check MUST NOT
        // affect primary's bounds (primary still allows up to 5 hypotheses).
        let mut sample = valid_sample();
        sample.hypotheses = (0..HYPOTHESES_MAX)
            .map(|i| Hypothesis {
                statement: format!("hypothesis {i}"),
                confidence: Confidence::Medium,
                justification: format!("justification {i}"),
            })
            .collect();
        sample.investigation_steps = (0..INVESTIGATION_STEPS_MAX)
            .map(|i| InvestigationStep {
                step: format!("step {i}"),
                expected_yield: format!("yield {i}"),
            })
            .collect();
        validate(&sample).expect("primary с 5 hypotheses + 5 investigation steps passes");
    }

    #[test]
    fn parse_bounded_rejects_fallback_with_too_many_hypotheses() {
        let mut sample = fallback_sample();
        sample.hypotheses = vec![
            Hypothesis {
                statement: "first".into(),
                confidence: Confidence::High,
                justification: "j1".into(),
            },
            Hypothesis {
                statement: "second".into(),
                confidence: Confidence::Medium,
                justification: "j2".into(),
            },
        ];
        let bytes = serde_json::to_vec(&sample).expect("serialize ok");
        match parse_bounded(&bytes) {
            Err(InferenceError::SchemaViolation { reason }) => {
                assert!(reason.contains("fallback"));
                assert!(reason.contains("hypotheses"));
            }
            other => panic!("expected SchemaViolation, got {other:?}"),
        }
    }

    // NOTE: the `FALLBACK_*_MAX < *_MAX` invariant is enforced at
    // compile time via а module-level `const _: () = { assert!(...) };`
    // block; clippy::assertions_on_constants rejects the assertion
    // here per CLAUDE.md testing.md 2026-05-11 pattern.
}
