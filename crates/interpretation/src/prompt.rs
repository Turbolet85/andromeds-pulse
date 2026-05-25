//! L4 LLM interpretation prompt scaffolding (chunk #83 — Epoch 9
//! Foundation v0.2.0).
//!
//! Composes the system + user prompt drove into mistralrs strict-schema-
//! mode constrained generation. Layout per pulse-distillation-architecture.md
//! §L4 input composition:
//!
//! 1. Role definition (~150 tokens)
//! 2. Project conventions snippet (~100 tokens)
//! 3. Embedded JSON schema (~400 tokens; sourced from
//!    [`crate::schema::L4_OUTPUT_JSON_SCHEMA`])
//! 4. Current L3 digest payload (~500-2000 tokens; supplied by caller)
//! 5. Project context block (~100-500 tokens; supplied by caller)
//! 6. Corpus retrieval context (~0-1000 tokens; supplied by caller; empty
//!    string at chunk #83 substrate per chunk #81 corpus retrieval deferral)
//! 7. Output format reminder (~200-300 tokens)
//!
//! Total target: ~6-8K tokens for the primary tier prompt.
//!
//! Token counting itself is deferred to а future chunk because importing
//! the Hugging Face `tokenizers` crate adds substantial transitive build
//! cost; chunk #81's tokenizer fixture in `crates/triage/build.rs` Phase 2
//! is available for cross-crate reuse via а `pub` re-export pattern once
//! the actual mistralrs runtime binding lands.

use crate::schema::{
    L4_OUTPUT_JSON_SCHEMA, PROMPT_VERSION_FALLBACK, PROMPT_VERSION_PRIMARY, SCHEMA_VERSION,
};

/// Marker bounding the L3 digest insertion in the assembled prompt.
pub const DIGEST_OPEN_MARKER: &str = "<DIGEST>";
/// Marker closing the L3 digest insertion.
pub const DIGEST_CLOSE_MARKER: &str = "</DIGEST>";
/// Marker bounding the project context insertion.
pub const PROJECT_OPEN_MARKER: &str = "<PROJECT>";
/// Marker closing the project context insertion.
pub const PROJECT_CLOSE_MARKER: &str = "</PROJECT>";
/// Marker bounding the corpus retrieval insertion.
pub const CORPUS_OPEN_MARKER: &str = "<CORPUS>";
/// Marker closing the corpus retrieval insertion.
pub const CORPUS_CLOSE_MARKER: &str = "</CORPUS>";

/// Role definition section emitted at the top of every primary-tier
/// prompt. Bounded к role + conventions; no project-specific facts here.
const ROLE_DEFINITION: &str = "\
You are а severity classifier for а local OpenTelemetry triage assistant. \
Your job is к decide whether an observed signal warrants surfacing к the \
developer, dismissing as noise, or watching for further evolution. \
You receive а distilled digest of recent telemetry plus а project context \
block. Your output MUST conform к the embedded JSON schema, with no prose \
before or after the JSON object.";

/// Convention snippet outlining decision categories + severity labels +
/// hypothesis ranking discipline. Bounded; does not reference any
/// project-specific identifiers.
const CONVENTIONS_SNIPPET: &str = "\
Decision categories: \"surface\" (create an incident the user should see), \
\"dismiss\" (no action), \"watch\" (record but do not surface). \
Severity labels: \"autonomous\" (act now), \"suggested\" (likely intervention), \
\"curious\" (worth investigating), \"none\" (informational). \
Hypotheses ranked highest-confidence-first; each carries а bounded \
confidence label (high/medium/low) and а brief justification. \
Investigation steps point к concrete checks the developer can run.";

/// Output format reminder emitted at the bottom of every primary-tier
/// prompt. Instructs strict JSON-only emission matching the embedded
/// schema. Final defense against the model dropping into prose.
const OUTPUT_REMINDER: &str = "\
Emit exactly one JSON object matching the schema above. \
Do not emit any text outside the JSON object. \
Do not emit а markdown code fence around the JSON. \
Do not emit explanatory prose before or after the JSON object.";

/// Role definition section emitted at the top of every fallback-tier
/// prompt (chunk #85 — Epoch 9 Foundation v0.2.0). Reduced specificity vs
/// primary; explicitly instructs single-hypothesis output для 3-4B class
/// models running on hardware-constrained hosts per P-053.
const ROLE_DEFINITION_FALLBACK: &str = "\
You are а severity classifier for а local OpenTelemetry triage assistant \
running on а hardware-constrained host (fallback tier). Decide whether \
an observed signal warrants surfacing к the developer, dismissing as \
noise, or watching for further evolution. Emit exactly ONE hypothesis \
(not а ranked list) and up к 2 investigation steps. Output MUST conform \
к the embedded JSON schema; no prose outside the JSON.";

/// Output format reminder emitted at the bottom of every fallback-tier
/// prompt. Reinforces reduced-quality output contract (single hypothesis,
/// ≤2 investigation steps) per P-053 boundaries on P-033 + P-034.
const OUTPUT_REMINDER_FALLBACK: &str = "\
Emit exactly one JSON object matching the schema above. \
Set `model_tier` to \"fallback\" in the output. \
Provide exactly ONE hypothesis (highest-confidence; not а ranked list of multiple). \
Provide at most 2 investigation steps. \
Do not emit text outside the JSON object. \
Do not emit а markdown code fence around the JSON. \
Do not emit explanatory prose before or after the JSON object.";

/// Composes the primary-tier prompt per [`PROMPT_VERSION_PRIMARY`].
///
/// All free-text inputs are inserted verbatim between delimited markers so
/// the model can locate them unambiguously. Length bounds on the inputs are
/// the caller's responsibility (digest assembler enforces digest token
/// budget per chunk #81; project context + corpus retrieval are bounded by
/// the digest pipeline upstream).
///
/// Returns the assembled prompt as а single `String`. The string IS the
/// prompt passed to `LlmInferenceRunner::generate_constrained(prompt,
/// schema_json)`; the `schema_json` argument к that call is the same
/// [`L4_OUTPUT_JSON_SCHEMA`] constant the prompt references.
pub fn build_primary_tier_prompt(
    digest_payload: &str,
    project_context: &str,
    corpus_retrieval: &str,
) -> String {
    let mut prompt = String::with_capacity(
        ROLE_DEFINITION.len()
            + CONVENTIONS_SNIPPET.len()
            + L4_OUTPUT_JSON_SCHEMA.len()
            + digest_payload.len()
            + project_context.len()
            + corpus_retrieval.len()
            + OUTPUT_REMINDER.len()
            + 512, // markers + section headers
    );

    prompt.push_str("# Role\n");
    prompt.push_str(ROLE_DEFINITION);
    prompt.push_str("\n\n");

    prompt.push_str("# Conventions\n");
    prompt.push_str(CONVENTIONS_SNIPPET);
    prompt.push_str("\n\n");

    prompt.push_str("# Output Schema (JSON Schema draft 2020-12)\n");
    prompt.push_str("schema_version: ");
    prompt.push_str(SCHEMA_VERSION);
    prompt.push_str("; prompt_version: ");
    prompt.push_str(PROMPT_VERSION_PRIMARY);
    prompt.push_str("\n```json\n");
    prompt.push_str(L4_OUTPUT_JSON_SCHEMA);
    prompt.push_str("\n```\n\n");

    prompt.push_str("# Project Context\n");
    prompt.push_str(PROJECT_OPEN_MARKER);
    prompt.push('\n');
    prompt.push_str(project_context);
    prompt.push('\n');
    prompt.push_str(PROJECT_CLOSE_MARKER);
    prompt.push_str("\n\n");

    prompt.push_str("# Current Digest\n");
    prompt.push_str(DIGEST_OPEN_MARKER);
    prompt.push('\n');
    prompt.push_str(digest_payload);
    prompt.push('\n');
    prompt.push_str(DIGEST_CLOSE_MARKER);
    prompt.push_str("\n\n");

    if !corpus_retrieval.trim().is_empty() {
        prompt.push_str("# Corpus Retrieval (past similar incidents)\n");
        prompt.push_str(CORPUS_OPEN_MARKER);
        prompt.push('\n');
        prompt.push_str(corpus_retrieval);
        prompt.push('\n');
        prompt.push_str(CORPUS_CLOSE_MARKER);
        prompt.push_str("\n\n");
    }

    prompt.push_str("# Output Instructions\n");
    prompt.push_str(OUTPUT_REMINDER);
    prompt.push('\n');

    prompt
}

/// Composes the fallback-tier prompt per [`PROMPT_VERSION_FALLBACK`]
/// (chunk #85 — Epoch 9 Foundation v0.2.0).
///
/// Mirrors [`build_primary_tier_prompt`] section structure (Role →
/// Conventions → Schema → Project Context → Current Digest → optional
/// Corpus Retrieval → Output Instructions) but substitutes reduced-
/// quality framing per P-053: smaller role definition emphasizing
/// single-hypothesis output, reinforced output reminder restating
/// the ≤1 hypothesis + ≤2 investigation steps constraints.
///
/// The reduced-quality contract is enforced TWICE in defense-in-depth:
/// at prompt time (this fn instructs the model) AND at parse time
/// ([`crate::schema::validate`] rejects fallback outputs с >1 hypothesis
/// OR >2 investigation steps).
pub fn build_fallback_tier_prompt(
    digest_payload: &str,
    project_context: &str,
    corpus_retrieval: &str,
) -> String {
    let mut prompt = String::with_capacity(
        ROLE_DEFINITION_FALLBACK.len()
            + CONVENTIONS_SNIPPET.len()
            + L4_OUTPUT_JSON_SCHEMA.len()
            + digest_payload.len()
            + project_context.len()
            + corpus_retrieval.len()
            + OUTPUT_REMINDER_FALLBACK.len()
            + 512,
    );

    prompt.push_str("# Role\n");
    prompt.push_str(ROLE_DEFINITION_FALLBACK);
    prompt.push_str("\n\n");

    prompt.push_str("# Conventions\n");
    prompt.push_str(CONVENTIONS_SNIPPET);
    prompt.push_str("\n\n");

    prompt.push_str("# Output Schema (JSON Schema draft 2020-12)\n");
    prompt.push_str("schema_version: ");
    prompt.push_str(SCHEMA_VERSION);
    prompt.push_str("; prompt_version: ");
    prompt.push_str(PROMPT_VERSION_FALLBACK);
    prompt.push_str("\n```json\n");
    prompt.push_str(L4_OUTPUT_JSON_SCHEMA);
    prompt.push_str("\n```\n\n");

    prompt.push_str("# Project Context\n");
    prompt.push_str(PROJECT_OPEN_MARKER);
    prompt.push('\n');
    prompt.push_str(project_context);
    prompt.push('\n');
    prompt.push_str(PROJECT_CLOSE_MARKER);
    prompt.push_str("\n\n");

    prompt.push_str("# Current Digest\n");
    prompt.push_str(DIGEST_OPEN_MARKER);
    prompt.push('\n');
    prompt.push_str(digest_payload);
    prompt.push('\n');
    prompt.push_str(DIGEST_CLOSE_MARKER);
    prompt.push_str("\n\n");

    if !corpus_retrieval.trim().is_empty() {
        prompt.push_str("# Corpus Retrieval (past similar incidents)\n");
        prompt.push_str(CORPUS_OPEN_MARKER);
        prompt.push('\n');
        prompt.push_str(corpus_retrieval);
        prompt.push('\n');
        prompt.push_str(CORPUS_CLOSE_MARKER);
        prompt.push_str("\n\n");
    }

    prompt.push_str("# Output Instructions\n");
    prompt.push_str(OUTPUT_REMINDER_FALLBACK);
    prompt.push('\n');

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_prompt_embeds_schema_string() {
        let prompt = build_primary_tier_prompt("digest body", "project body", "");
        assert!(prompt.contains(L4_OUTPUT_JSON_SCHEMA));
    }

    #[test]
    fn primary_prompt_embeds_role_definition() {
        let prompt = build_primary_tier_prompt("digest", "project", "");
        assert!(prompt.contains("severity classifier"));
        assert!(prompt.contains("# Role"));
    }

    #[test]
    fn primary_prompt_embeds_conventions_snippet() {
        let prompt = build_primary_tier_prompt("digest", "project", "");
        assert!(prompt.contains("# Conventions"));
        assert!(prompt.contains("\"surface\""));
        assert!(prompt.contains("\"dismiss\""));
        assert!(prompt.contains("\"watch\""));
        assert!(prompt.contains("\"autonomous\""));
        assert!(prompt.contains("\"suggested\""));
        assert!(prompt.contains("\"curious\""));
        assert!(prompt.contains("\"none\""));
    }

    #[test]
    fn primary_prompt_wraps_digest_in_markers() {
        let digest = "WINDOW: ... SERVICES: ...";
        let prompt = build_primary_tier_prompt(digest, "ctx", "");
        assert!(prompt.contains(DIGEST_OPEN_MARKER));
        assert!(prompt.contains(DIGEST_CLOSE_MARKER));
        assert!(prompt.contains(digest));
        let open_idx = prompt.find(DIGEST_OPEN_MARKER).expect("open marker found");
        let close_idx = prompt
            .find(DIGEST_CLOSE_MARKER)
            .expect("close marker found");
        let digest_idx = prompt.find(digest).expect("digest body found");
        assert!(open_idx < digest_idx);
        assert!(digest_idx < close_idx);
    }

    #[test]
    fn primary_prompt_wraps_project_context_in_markers() {
        let project = "workspace=/home/dev/example; vcs=git";
        let prompt = build_primary_tier_prompt("digest", project, "");
        assert!(prompt.contains(PROJECT_OPEN_MARKER));
        assert!(prompt.contains(PROJECT_CLOSE_MARKER));
        assert!(prompt.contains(project));
    }

    #[test]
    fn primary_prompt_omits_corpus_section_when_empty() {
        let prompt = build_primary_tier_prompt("digest", "project", "");
        assert!(!prompt.contains(CORPUS_OPEN_MARKER));
        assert!(!prompt.contains(CORPUS_CLOSE_MARKER));
        assert!(!prompt.contains("# Corpus Retrieval"));
    }

    #[test]
    fn primary_prompt_includes_corpus_section_when_present() {
        let corpus = "prior incident: db-saturation:service-a (3 days ago)";
        let prompt = build_primary_tier_prompt("digest", "project", corpus);
        assert!(prompt.contains(CORPUS_OPEN_MARKER));
        assert!(prompt.contains(CORPUS_CLOSE_MARKER));
        assert!(prompt.contains(corpus));
        assert!(prompt.contains("# Corpus Retrieval"));
    }

    #[test]
    fn primary_prompt_emits_output_format_reminder() {
        let prompt = build_primary_tier_prompt("digest", "project", "");
        assert!(prompt.contains("Emit exactly one JSON object"));
        assert!(prompt.contains("Do not emit"));
        assert!(prompt.contains("# Output Instructions"));
    }

    #[test]
    fn primary_prompt_embeds_versioning_metadata() {
        let prompt = build_primary_tier_prompt("digest", "project", "");
        assert!(prompt.contains(SCHEMA_VERSION));
        assert!(prompt.contains(PROMPT_VERSION_PRIMARY));
    }

    #[test]
    fn primary_prompt_section_ordering_is_stable() {
        let prompt = build_primary_tier_prompt("digest", "project", "corpus");
        let role_idx = prompt.find("# Role").expect("role section");
        let conv_idx = prompt.find("# Conventions").expect("conventions section");
        let schema_idx = prompt.find("# Output Schema").expect("schema section");
        let project_idx = prompt.find("# Project Context").expect("project section");
        let digest_idx = prompt.find("# Current Digest").expect("digest section");
        let corpus_idx = prompt.find("# Corpus Retrieval").expect("corpus section");
        let output_idx = prompt
            .find("# Output Instructions")
            .expect("output section");
        assert!(role_idx < conv_idx);
        assert!(conv_idx < schema_idx);
        assert!(schema_idx < project_idx);
        assert!(project_idx < digest_idx);
        assert!(digest_idx < corpus_idx);
        assert!(corpus_idx < output_idx);
    }

    #[test]
    fn primary_prompt_does_not_leak_internal_field_names_outside_schema() {
        // The prompt embeds the schema verbatim; otherwise it should not
        // mention raw OTLP attribute field names (span_id / trace_id / etc.)
        // outside of the schema's evidence_refs description per security
        // extract anti-pattern "no raw OTLP content in tracing or prompts
        // beyond the digest_payload supplied by the caller".
        let prompt = build_primary_tier_prompt("safe digest body", "safe project ctx", "");
        // Trim out the embedded schema chunk so we don't false-positive
        // against schema description text mentioning span_id etc.
        let schema_start = prompt.find(L4_OUTPUT_JSON_SCHEMA).expect("schema present");
        let schema_end = schema_start + L4_OUTPUT_JSON_SCHEMA.len();
        let outside_schema = format!("{}{}", &prompt[..schema_start], &prompt[schema_end..]);
        for banned in ["span_id", "trace_id", "parent_span_id", "ts_unix_nano"] {
            assert!(
                !outside_schema.contains(banned),
                "prompt scaffolding outside schema must not leak raw OTLP field name `{banned}`"
            );
        }
    }

    // ---- Fallback-tier prompt coverage (chunk #85) ----

    #[test]
    fn fallback_prompt_embeds_schema_string() {
        let prompt = build_fallback_tier_prompt("digest body", "project body", "");
        assert!(prompt.contains(L4_OUTPUT_JSON_SCHEMA));
    }

    #[test]
    fn fallback_prompt_embeds_reduced_role_definition() {
        let prompt = build_fallback_tier_prompt("digest", "project", "");
        assert!(prompt.contains("severity classifier"));
        assert!(prompt.contains("# Role"));
        assert!(prompt.contains("fallback tier"));
        assert!(prompt.contains("ONE hypothesis"));
    }

    #[test]
    fn fallback_prompt_embeds_conventions_snippet() {
        let prompt = build_fallback_tier_prompt("digest", "project", "");
        assert!(prompt.contains("# Conventions"));
        assert!(prompt.contains("\"surface\""));
        assert!(prompt.contains("\"dismiss\""));
        assert!(prompt.contains("\"watch\""));
        assert!(prompt.contains("\"autonomous\""));
        assert!(prompt.contains("\"suggested\""));
        assert!(prompt.contains("\"curious\""));
        assert!(prompt.contains("\"none\""));
    }

    #[test]
    fn fallback_prompt_emits_reduced_output_reminder() {
        let prompt = build_fallback_tier_prompt("digest", "project", "");
        assert!(prompt.contains("# Output Instructions"));
        assert!(prompt.contains("exactly ONE hypothesis"));
        assert!(prompt.contains("at most 2 investigation steps"));
        assert!(prompt.contains("\"fallback\""));
    }

    #[test]
    fn fallback_prompt_wraps_digest_in_markers() {
        let digest = "WINDOW: ... SERVICES: ...";
        let prompt = build_fallback_tier_prompt(digest, "ctx", "");
        assert!(prompt.contains(DIGEST_OPEN_MARKER));
        assert!(prompt.contains(DIGEST_CLOSE_MARKER));
        assert!(prompt.contains(digest));
        let open_idx = prompt.find(DIGEST_OPEN_MARKER).expect("open marker found");
        let close_idx = prompt
            .find(DIGEST_CLOSE_MARKER)
            .expect("close marker found");
        let digest_idx = prompt.find(digest).expect("digest body found");
        assert!(open_idx < digest_idx);
        assert!(digest_idx < close_idx);
    }

    #[test]
    fn fallback_prompt_wraps_project_context_in_markers() {
        let project = "workspace=/home/dev/example; vcs=git";
        let prompt = build_fallback_tier_prompt("digest", project, "");
        assert!(prompt.contains(PROJECT_OPEN_MARKER));
        assert!(prompt.contains(PROJECT_CLOSE_MARKER));
        assert!(prompt.contains(project));
    }

    #[test]
    fn fallback_prompt_omits_corpus_section_when_empty() {
        let prompt = build_fallback_tier_prompt("digest", "project", "");
        assert!(!prompt.contains(CORPUS_OPEN_MARKER));
        assert!(!prompt.contains(CORPUS_CLOSE_MARKER));
        assert!(!prompt.contains("# Corpus Retrieval"));
    }

    #[test]
    fn fallback_prompt_includes_corpus_section_when_present() {
        let corpus = "prior incident: db-saturation:service-a (3 days ago)";
        let prompt = build_fallback_tier_prompt("digest", "project", corpus);
        assert!(prompt.contains(CORPUS_OPEN_MARKER));
        assert!(prompt.contains(CORPUS_CLOSE_MARKER));
        assert!(prompt.contains(corpus));
        assert!(prompt.contains("# Corpus Retrieval"));
    }

    #[test]
    fn fallback_prompt_embeds_versioning_metadata() {
        let prompt = build_fallback_tier_prompt("digest", "project", "");
        assert!(prompt.contains(SCHEMA_VERSION));
        // The metadata line uses "prompt_version: " (colon + space) prefix;
        // assert fallback's exact metadata-line form is present AND primary's
        // metadata-line form is absent. (The embedded schema.json
        // description text legitimately references "v2.1" — chunk #83
        // prose — so asserting absence of the bare "v2.1" string would
        // false-positive against the schema document.)
        let fallback_meta = format!("prompt_version: {PROMPT_VERSION_FALLBACK}");
        let primary_meta = format!("prompt_version: {PROMPT_VERSION_PRIMARY}");
        assert!(prompt.contains(&fallback_meta));
        assert!(!prompt.contains(&primary_meta));
    }

    #[test]
    fn fallback_prompt_section_ordering_is_stable() {
        let prompt = build_fallback_tier_prompt("digest", "project", "corpus");
        let role_idx = prompt.find("# Role").expect("role section");
        let conv_idx = prompt.find("# Conventions").expect("conventions section");
        let schema_idx = prompt.find("# Output Schema").expect("schema section");
        let project_idx = prompt.find("# Project Context").expect("project section");
        let digest_idx = prompt.find("# Current Digest").expect("digest section");
        let corpus_idx = prompt.find("# Corpus Retrieval").expect("corpus section");
        let output_idx = prompt
            .find("# Output Instructions")
            .expect("output section");
        assert!(role_idx < conv_idx);
        assert!(conv_idx < schema_idx);
        assert!(schema_idx < project_idx);
        assert!(project_idx < digest_idx);
        assert!(digest_idx < corpus_idx);
        assert!(corpus_idx < output_idx);
    }

    #[test]
    fn fallback_prompt_role_definition_distinguishes_from_primary() {
        // Per P-053 reduced-output contract: fallback's ROLE_DEFINITION_FALLBACK
        // explicitly states the single-hypothesis + ≤2-steps constraints that
        // primary's ROLE_DEFINITION lacks. Asserting the distinguishing strings
        // is а stronger regression guard than byte-count comparison (fallback
        // adds explicit output-shape constraints + drops the "distilled digest"
        // educational text, net byte delta is approximately neutral).
        assert!(ROLE_DEFINITION_FALLBACK.contains("fallback tier"));
        assert!(ROLE_DEFINITION_FALLBACK.contains("ONE hypothesis"));
        assert!(!ROLE_DEFINITION.contains("fallback tier"));
        assert!(!ROLE_DEFINITION.contains("ONE hypothesis"));
    }

    #[test]
    fn fallback_prompt_total_bytes_bounded() {
        // Sanity bound: fallback prompt с empty digest + empty project ctx
        // + empty corpus should fit comfortably under 8 KB (the embedded
        // schema dominates at ~4.5 KB; framing text + markers + headers
        // contribute another ~1.5-2 KB). Regression guard against
        // accidental schema bloat OR framing-text runaway.
        let prompt = build_fallback_tier_prompt("", "", "");
        assert!(
            prompt.len() < 8000,
            "fallback prompt bytes ({}) exceeded 8000-byte sanity bound",
            prompt.len()
        );
    }

    #[test]
    fn fallback_prompt_does_not_leak_internal_field_names_outside_schema() {
        // Mirror primary's negative-canary discipline — fallback's
        // framing text MUST NOT mention raw OTLP attribute field names
        // outside the embedded schema region.
        let prompt = build_fallback_tier_prompt("safe digest body", "safe project ctx", "");
        let schema_start = prompt.find(L4_OUTPUT_JSON_SCHEMA).expect("schema present");
        let schema_end = schema_start + L4_OUTPUT_JSON_SCHEMA.len();
        let outside_schema = format!("{}{}", &prompt[..schema_start], &prompt[schema_end..]);
        for banned in ["span_id", "trace_id", "parent_span_id", "ts_unix_nano"] {
            assert!(
                !outside_schema.contains(banned),
                "fallback prompt scaffolding outside schema must not leak raw OTLP field name `{banned}`"
            );
        }
    }
}
