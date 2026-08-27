//! Investigate-action TauRPC router (P-072 · intent F12 — Investigate actions
//! functional).
//!
//! `investigate.run_action` runs a real L4-LLM-backed analysis of the current
//! telemetry for one of the 4 Investigate actions and returns a TRANSIENT
//! structured result — it does NOT create or persist an incident. It reuses the
//! existing `LlmInferenceRunner::generate_constrained` + the incident `L4Output`
//! schema (2026-06-28 phase decision: reuse over a new free-form contract) and
//! honours the deterministic env-gated L4 mode (P-073), so the path is
//! reproducible without GPU/model. The library crates stay Tauri-free; the
//! Tauri-aware router lives here at the binary boundary per arch §Cross-cutting
//! Patterns Module dependency direction (mirrors `model_router.rs`).

use std::sync::{Arc, Mutex};
use std::time::Instant;

use duckdb::Connection;
use interpretation::contract::{LlmInferenceRunner, model_tier_label};
use interpretation::prompt::build_primary_tier_prompt;
use interpretation::schema::{L4_OUTPUT_JSON_SCHEMA, L4Output, parse_bounded};
use security::scrubber::{ScrubbedValue, scrub_attribute};
use serde::{Deserialize, Serialize};
use ui_bridge::contract::AppError;

use crate::deterministic_inference::deterministic_mode_enabled;
use crate::model_router::inference_error_to_app_error;
use crate::snapshot_runtime::load_curated_markdown;

/// Tracing target — per-action aggregate request event (bounded `action_id`
/// enum tag + `status` + numeric `duration_ms`; NO prompt / result / telemetry
/// content per obs §5 + the 2026-05-17 aggregate-only mandate).
const TARGET_INVESTIGATE_REQUEST: &str = "investigate.run_action.request";
/// Metric target — per-action latency sample (milliseconds).
const TARGET_METRIC_INVESTIGATE_DURATION_MS: &str = "metric.investigate.run_action.duration_ms";

/// The 4 Investigate actions: id (bounded, mirrors the webview
/// `preset-prompts.ts` + `snapshot_runtime::PRESET_PROMPT_DEFINITIONS`) →
/// server-side analysis-framing instruction. Framing is backend-owned so the
/// resolver bounds its input to these 4 ids and never trusts a webview-supplied
/// prompt string.
const INVESTIGATE_ACTIONS: &[(&str, &str)] = &[
    (
        "diagnose-latency-outlier",
        "Diagnose the latency outlier in the telemetry below: identify the slowest service on the critical path and the spans contributing most to its latency.",
    ),
    (
        "find-error-correlation",
        "Examine the error-correlated spans in the telemetry below: group errors by service and identify the likely upstream cause.",
    ),
    (
        "trace-failed-request",
        "Walk the trace of the failed request in the telemetry below from the entry span through downstream calls; identify where the error first appears.",
    ),
    (
        "summarize-service-health",
        "Summarize per-service health from the telemetry below using p50/p95/p99 latencies and error rates; flag any service exceeding expected bounds.",
    ),
];

fn action_framing(action_id: &str) -> Option<&'static str> {
    INVESTIGATE_ACTIONS
        .iter()
        .find(|(id, _)| *id == action_id)
        .map(|(_, framing)| *framing)
}

/// One ranked hypothesis surfaced to the webview (scrubbed L4Output subset).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct InvestigateHypothesis {
    pub statement: String,
    pub justification: String,
}

/// One suggested investigation step surfaced to the webview.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct InvestigateStep {
    pub step: String,
    pub expected_yield: String,
}

/// Transient analysis result returned by `investigate.run_action`. A scrubbed
/// projection of the incident `L4Output` analysis fields; carries NO incident
/// id (nothing is persisted).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct InvestigateResultDto {
    pub action_id: String,
    pub title: String,
    pub symptom: String,
    pub timeline: String,
    pub hypotheses: Vec<InvestigateHypothesis>,
    pub investigation_steps: Vec<InvestigateStep>,
}

#[taurpc::procedures(path = "investigate")]
pub trait InvestigateApi {
    async fn run_action(action_id: String) -> Result<InvestigateResultDto, AppError>;
}

#[derive(Clone)]
pub struct InvestigateApiImpl {
    conn: Option<Arc<Mutex<Connection>>>,
    runner: Arc<dyn LlmInferenceRunner>,
}

impl InvestigateApiImpl {
    pub fn new(conn: Option<Arc<Mutex<Connection>>>, runner: Arc<dyn LlmInferenceRunner>) -> Self {
        Self { conn, runner }
    }
}

/// Scrub a model-derived text field at the resolver egress boundary
/// (defense-in-depth on top of any producer-side scrubbing per chunk #72/#88).
fn scrub(raw: &str) -> String {
    match scrub_attribute(raw) {
        ScrubbedValue::Allowed(s) => s,
        ScrubbedValue::Redacted { category } => format!("[redacted: {category}]"),
    }
}

fn to_result_dto(action_id: &str, parsed: &L4Output) -> InvestigateResultDto {
    InvestigateResultDto {
        action_id: action_id.to_string(),
        title: scrub(&parsed.title),
        symptom: scrub(&parsed.symptom),
        timeline: scrub(&parsed.timeline),
        hypotheses: parsed
            .hypotheses
            .iter()
            .map(|h| InvestigateHypothesis {
                statement: scrub(&h.statement),
                justification: scrub(&h.justification),
            })
            .collect(),
        investigation_steps: parsed
            .investigation_steps
            .iter()
            .map(|s| InvestigateStep {
                step: scrub(&s.step),
                expected_yield: scrub(&s.expected_yield),
            })
            .collect(),
    }
}

/// Aggregate-only outcome emission. `action_id` is always a bounded value —
/// callers pass the validated id, or the literal `"unknown"` on the
/// validation-error path so an arbitrary webview-supplied id never reaches the
/// log (cardinality discipline per obs §5).
fn emit_outcome(
    action_id: &str,
    status: &str,
    started: Instant,
    deterministic_mode: bool,
    model_tier: &'static str,
) {
    let duration_ms = started.elapsed().as_millis() as u64;
    tracing::info!(
        target: TARGET_INVESTIGATE_REQUEST,
        action_id = action_id,
        status = status,
        duration_ms = duration_ms,
        deterministic_mode = deterministic_mode,
        model_tier = model_tier,
        "investigate action completed",
    );
    tracing::info!(
        target: TARGET_METRIC_INVESTIGATE_DURATION_MS,
        value = duration_ms,
        duration_ms = duration_ms,
        action_id = action_id,
        status = status,
        model_tier = model_tier,
        "investigate action latency sample",
    );
}

#[taurpc::resolvers]
impl InvestigateApi for InvestigateApiImpl {
    #[tracing::instrument(skip_all)]
    async fn run_action(self, action_id: String) -> Result<InvestigateResultDto, AppError> {
        let started = Instant::now();
        let tier_label = model_tier_label(self.runner.tier());
        let deterministic = deterministic_mode_enabled();

        let Some(framing) = action_framing(&action_id) else {
            emit_outcome(
                "unknown",
                "validation_error",
                started,
                deterministic,
                tier_label,
            );
            return Err(AppError::Validation {
                field: "action_id".to_string(),
                reason: "unknown investigate action".to_string(),
            });
        };

        let Some(conn) = self.conn.clone() else {
            emit_outcome(
                &action_id,
                "storage_error",
                started,
                deterministic,
                tier_label,
            );
            return Err(AppError::Storage {
                message: "buffer unavailable".to_string(),
            });
        };

        // Curated-telemetry context — the same load + curate + format path the
        // snapshot resolver uses, run off the async runtime (DuckDB is blocking).
        let conn_for_load = Arc::clone(&conn);
        let context = match tokio::task::spawn_blocking(move || -> Result<String, AppError> {
            let guard = conn_for_load.lock().map_err(|_| AppError::Storage {
                message: "investigate: lock poisoned".to_string(),
            })?;
            load_curated_markdown(&guard)
        })
        .await
        {
            Ok(Ok(ctx)) => ctx,
            Ok(Err(e)) => {
                emit_outcome(
                    &action_id,
                    "storage_error",
                    started,
                    deterministic,
                    tier_label,
                );
                return Err(e);
            }
            Err(_) => {
                emit_outcome(
                    &action_id,
                    "internal_error",
                    started,
                    deterministic,
                    tier_label,
                );
                return Err(AppError::internal("investigate: context task failed"));
            }
        };

        // Investigate runs over ad-hoc curated context with no digest cues,
        // so the citable-ids list is empty — the prompt's citing instruction
        // then mandates an empty evidence_refs array (honest empty).
        let prompt = build_primary_tier_prompt(
            &format!("INVESTIGATION FOCUS: {framing}\n\n{context}"),
            "",
            "",
            &[],
        );

        let raw = match self
            .runner
            .generate_constrained(&prompt, L4_OUTPUT_JSON_SCHEMA)
            .await
        {
            Ok(raw) => raw,
            Err(e) => {
                emit_outcome(
                    &action_id,
                    "runtime_error",
                    started,
                    deterministic,
                    tier_label,
                );
                return Err(inference_error_to_app_error(e));
            }
        };

        let parsed = match parse_bounded(raw.as_bytes()) {
            Ok(p) => p,
            Err(e) => {
                emit_outcome(
                    &action_id,
                    "parse_failure",
                    started,
                    deterministic,
                    tier_label,
                );
                return Err(inference_error_to_app_error(e));
            }
        };

        let dto = to_result_dto(&action_id, &parsed);
        emit_outcome(&action_id, "success", started, deterministic, tier_label);
        Ok(dto)
    }
}

// Tests live at `pulse-app/tests/unit_investigate_router.rs` +
// `pulse-app/tests/integration_investigate_actions.rs` (integration test crate)
// per CLAUDE.md testing.md 2026-05-20 — pulse-app's `[lib] test = false`
// disables source-level `mod tests` on Windows (WebView2 DLL load at discovery).
