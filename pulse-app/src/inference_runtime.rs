//! L4 inference subscriber adapter (chunk #83 — Epoch 9 Foundation v0.2.0).
//!
//! Subscribes to `pulse://stream/digests` (chunk #81), invokes the
//! `LlmInferenceRunner::generate_constrained` trait surface with the
//! primary-tier prompt + embedded JSON schema, parses + validates the
//! output via `interpretation::schema::parse_bounded`, and emits
//! observability events along the way.
//!
//! Persistence of the parsed L4Output к incident records is deferred to
//! а follow-up chunk (chunk #86+ Findings counter + corpus integration)
//! because the IncidentPersistence trait surface from chunk #78 does NOT
//! currently accept а payload arg on `save_incident_event` AND extending
//! that surface is out of chunk #83's plan Files-to-modify scope. Chunk
//! #83 substrate establishes the subscriber + observability instrumentation
//! end-to-end; future chunks wire the persistence side once the parsed
//! L4Output → Incident mapping is fully specified.
//!
//! Pattern: mirrors chunk #81 `spawn_cadence_subscriber` +
//! `spawn_digest_persister` shape — long-running tokio task с recv loop
//! handling Ok / Lagged / Closed broadcast cases. Per arch §Cross-cutting
//! Patterns Module dependency direction, the subscriber lives at the
//! binary boundary; library crates stay Tauri-free + runtime-free.

use std::sync::Arc;
use std::time::{Duration, Instant};

use interpretation::contract::{InferenceError, LlmInferenceRunner};
use interpretation::prompt::build_primary_tier_prompt;
use interpretation::schema::{L4_OUTPUT_JSON_SCHEMA, L4Output};
use tokio::sync::broadcast::error::RecvError;
use tokio::task::JoinHandle;
use triage::contract::{Digest, DigestBroadcast};

/// Tracing target — top-level L4 inference request span (per L3 digest).
pub const TARGET_L4_INFERENCE_REQUEST: &str = "interpretation.inference.request";
/// Tracing target — prompt assembly internal span.
pub const TARGET_L4_PROMPT_ASSEMBLE: &str = "interpretation.prompt.assemble";
/// Tracing target — constrained generation invocation internal span.
pub const TARGET_L4_CONSTRAINED_GENERATE: &str = "interpretation.constrained.generate";
/// Tracing target — JSON parse + schema validation internal span.
pub const TARGET_L4_JSON_PARSE: &str = "interpretation.json.parse";
/// Tracing target — runtime errors caught at the `LlmInferenceRunner`
/// boundary. Per obs extract: bounded `error_category` field, never raw
/// `Debug` of the underlying error.
pub const TARGET_L4_INFERENCE_ERROR: &str = "interpretation.inference.error";

/// Metric target — per-inference counter; label `result ∈ {success,
/// parse_failure, runtime_error, schema_violation, output_too_large}`.
pub const TARGET_METRIC_L4_INFERENCES_TOTAL: &str = "metric.pipeline.l4.inferences_total";
/// Metric target — per-inference latency distribution (milliseconds).
pub const TARGET_METRIC_L4_INFERENCE_LATENCY_P99_MS: &str =
    "metric.pipeline.l4.inference_latency_p99_milliseconds";
/// Metric target — queue depth gauge (emitted per heartbeat tick by the
/// queue-depth task, not per-event).
pub const TARGET_METRIC_L4_INFERENCE_QUEUE_DEPTH: &str = "metric.pipeline.l4.inference_queue_depth";

/// Default heartbeat interval for the L4 queue-depth gauge emission task
/// (mirrors obs-plan §3 Heartbeat ticks 15s cadence).
pub const DEFAULT_QUEUE_DEPTH_TICK_INTERVAL: Duration = Duration::from_secs(15);

/// Build the project-context string passed into [`build_primary_tier_prompt`].
/// Pulls the workspace path + VCS hints from the L3 digest payload itself;
/// future chunks may enrich с recent commits + framework signals from
/// chunk #81's `DigestProjectContext`.
fn build_project_context(digest: &Digest) -> String {
    let mut ctx = String::with_capacity(256);
    ctx.push_str("workspace=");
    ctx.push_str(&digest.workspace);
    ctx
}

/// Spawn the L4 inference subscriber. Subscribes к `digest_broadcast`,
/// invokes `runner` per digest, emits observability events.
///
/// Returns the JoinHandle so callers can await graceful shutdown if needed.
/// Per chunk #83 substrate scope, the parsed L4Output is logged but NOT
/// persisted — future chunks add the persistence side once the
/// L4-к-incident mapping is fully specified (see chunk #86 Findings
/// counter + corpus integration).
pub fn spawn_l4_inference_subscriber(
    digest_broadcast: Arc<DigestBroadcast>,
    runner: Arc<dyn LlmInferenceRunner>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut rx = digest_broadcast.subscribe();
        loop {
            match rx.recv().await {
                Ok(digest) => {
                    handle_digest(&*runner, &digest).await;
                }
                Err(RecvError::Lagged(skipped)) => {
                    tracing::warn!(
                        target: TARGET_L4_INFERENCE_ERROR,
                        skipped_events = skipped,
                        error_category = "broadcast_lagged",
                        recovery_action = "resubscribe",
                        "L4 subscriber lagged behind digest broadcast; resubscribing",
                    );
                    rx = digest_broadcast.subscribe();
                }
                Err(RecvError::Closed) => return,
            }
        }
    })
}

/// Spawn the queue-depth heartbeat tick task. Emits the
/// `metric.pipeline.l4.inference_queue_depth` gauge every
/// [`DEFAULT_QUEUE_DEPTH_TICK_INTERVAL`] (15s).
///
/// `queued_count_fn` returns the current count of digests awaiting L4
/// inference. At chunk #83 substrate the subscriber is а single-consumer
/// recv loop with no internal queue; the broadcast channel's
/// `sender.len()` would return zero most of the time. Future chunks с
/// LWW + active-incident bypass queues per dist-arch v3 §L4 will provide
/// а richer counter.
pub fn spawn_l4_queue_depth_heartbeat<F>(queued_count_fn: F) -> JoinHandle<()>
where
    F: Fn() -> u64 + Send + Sync + 'static,
{
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(DEFAULT_QUEUE_DEPTH_TICK_INTERVAL);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            let queued = queued_count_fn();
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCE_QUEUE_DEPTH,
                value = queued,
                queued_count = queued,
                "L4 inference queue depth heartbeat",
            );
        }
    })
}

/// Single-digest inference handling. Composes prompt → invokes runner →
/// parses result → emits observability. Pulled out from
/// [`spawn_l4_inference_subscriber`] для unit-test reach via а stub
/// runner.
pub async fn handle_digest(runner: &dyn LlmInferenceRunner, digest: &Digest) {
    let tier = runner.tier();
    let started = Instant::now();
    let tier_label = interpretation::contract::model_tier_label(tier);
    let digest_kind = digest_kind_label(digest);

    // Prompt assembly — emit one event с the resulting token count proxy.
    let prompt_started = Instant::now();
    let project_context = build_project_context(digest);
    let prompt = build_primary_tier_prompt(&digest.payload_summary, &project_context, "");
    let prompt_elapsed_ms = prompt_started.elapsed().as_millis() as u64;
    tracing::info!(
        target: TARGET_L4_PROMPT_ASSEMBLE,
        prompt_version = interpretation::schema::PROMPT_VERSION_PRIMARY,
        token_count = prompt.len() as u64,
        duration_ms = prompt_elapsed_ms,
        "L4 primary-tier prompt assembled",
    );

    // Constrained generation invocation.
    let gen_started = Instant::now();
    let raw_output_result = runner
        .generate_constrained(&prompt, L4_OUTPUT_JSON_SCHEMA)
        .await;
    let gen_elapsed_ms = gen_started.elapsed().as_millis() as u64;
    let total_elapsed_ms = started.elapsed().as_millis() as u64;

    match raw_output_result {
        Ok(raw_output) => {
            tracing::info!(
                target: TARGET_L4_CONSTRAINED_GENERATE,
                model_tier = tier_label,
                duration_ms = gen_elapsed_ms,
                success = true,
                "L4 constrained generation returned",
            );
            handle_parse(&raw_output, tier_label, digest_kind, total_elapsed_ms);
        }
        Err(err) => {
            let category = inference_error_label(&err);
            tracing::warn!(
                target: TARGET_L4_INFERENCE_ERROR,
                model_tier = tier_label,
                error_category = category,
                recovery_action = "skip_digest",
                "L4 constrained generation failed",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCES_TOTAL,
                value = 1u64,
                result = "runtime_error",
                model_tier = tier_label,
                "L4 inference counter",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCE_LATENCY_P99_MS,
                value = total_elapsed_ms,
                duration_ms = total_elapsed_ms,
                model_tier = tier_label,
                "L4 inference latency sample",
            );
        }
    }
}

fn handle_parse(
    raw_output: &str,
    tier_label: &'static str,
    digest_kind: &'static str,
    total_elapsed_ms: u64,
) {
    let parse_started = Instant::now();
    let output_bytes = raw_output.as_bytes();
    let parse_result = interpretation::schema::parse_bounded(output_bytes);
    let parse_elapsed_ms = parse_started.elapsed().as_millis() as u64;

    match parse_result {
        Ok(parsed) => {
            tracing::info!(
                target: TARGET_L4_JSON_PARSE,
                parse_outcome = "ok",
                output_bytes = output_bytes.len() as u64,
                duration_ms = parse_elapsed_ms,
                "L4 inference output parsed cleanly",
            );
            tracing::info!(
                target: TARGET_L4_INFERENCE_REQUEST,
                model_tier = tier_label,
                digest_kind = digest_kind,
                duration_ms = total_elapsed_ms,
                result = "success",
                "L4 inference request completed",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCES_TOTAL,
                value = 1u64,
                result = "success",
                model_tier = tier_label,
                "L4 inference counter",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCE_LATENCY_P99_MS,
                value = total_elapsed_ms,
                duration_ms = total_elapsed_ms,
                model_tier = tier_label,
                "L4 inference latency sample",
            );
            // L4 output persistence к incident records is deferred к
            // chunk #86+. The parsed L4Output IS produced here + observable
            // via the success-path events above; downstream chunks add the
            // persistence pathway.
            let _persistence_deferred: &L4Output = &parsed;
        }
        Err(InferenceError::OutputTooLarge {
            actual_bytes,
            max_bytes,
        }) => {
            tracing::warn!(
                target: TARGET_L4_JSON_PARSE,
                parse_outcome = "output_too_large",
                output_bytes = actual_bytes as u64,
                max_bytes = max_bytes as u64,
                duration_ms = parse_elapsed_ms,
                "L4 inference output rejected by size cap",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCES_TOTAL,
                value = 1u64,
                result = "output_too_large",
                model_tier = tier_label,
                "L4 inference counter",
            );
        }
        Err(InferenceError::JsonParseFailed { .. }) => {
            tracing::warn!(
                target: TARGET_L4_JSON_PARSE,
                parse_outcome = "json_parse_failed",
                output_bytes = output_bytes.len() as u64,
                duration_ms = parse_elapsed_ms,
                "L4 inference output failed JSON parse",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCES_TOTAL,
                value = 1u64,
                result = "parse_failure",
                model_tier = tier_label,
                "L4 inference counter",
            );
        }
        Err(InferenceError::SchemaViolation { .. }) => {
            tracing::warn!(
                target: TARGET_L4_JSON_PARSE,
                parse_outcome = "schema_violation",
                output_bytes = output_bytes.len() as u64,
                duration_ms = parse_elapsed_ms,
                "L4 inference output violated schema invariant",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCES_TOTAL,
                value = 1u64,
                result = "schema_violation",
                model_tier = tier_label,
                "L4 inference counter",
            );
        }
        Err(other) => {
            // Other variants (ModelNotConfigured / etc) routed via the
            // runtime-error path inside `handle_digest`; reaching here
            // would indicate а new InferenceError variant — fall through
            // к counter increment for visibility.
            let category = inference_error_label(&other);
            tracing::warn!(
                target: TARGET_L4_INFERENCE_ERROR,
                error_category = category,
                "L4 parse-stage received unexpected inference error",
            );
            tracing::info!(
                target: TARGET_METRIC_L4_INFERENCES_TOTAL,
                value = 1u64,
                result = "runtime_error",
                model_tier = tier_label,
                "L4 inference counter",
            );
        }
    }
}

/// Bounded label per `InferenceError` variant. Used for `error_category`
/// tracing field; never carries reason content (per obs+security extract
/// PII discipline).
pub fn inference_error_label(err: &InferenceError) -> &'static str {
    match err {
        InferenceError::ModelNotConfigured => "model_not_configured",
        InferenceError::InvalidModelPath => "invalid_model_path",
        InferenceError::ModelLoadFailed { .. } => "model_load_failed",
        InferenceError::TokenizerInitFailed { .. } => "tokenizer_init_failed",
        InferenceError::InferenceFailed { .. } => "inference_failed",
        InferenceError::OutputTooLarge { .. } => "output_too_large",
        InferenceError::JsonParseFailed { .. } => "json_parse_failed",
        InferenceError::SchemaViolation { .. } => "schema_violation",
    }
}

/// Bounded label per `DigestKind` for instrumentation. Mirrors chunk #81
/// `digest_runtime::digest_kind_label` shape but lives here so the
/// inference subscriber doesn't import private helpers from another file.
fn digest_kind_label(d: &Digest) -> &'static str {
    use triage::contract::DigestKind;
    match d.kind {
        DigestKind::Snapshot => "snapshot",
        DigestKind::IncidentSummary => "incident_summary",
        DigestKind::BaselineState => "baseline_state",
        DigestKind::AttentionCueDigest => "attention_cue_digest",
        DigestKind::CadenceTier1 => "cadence_tier1",
        DigestKind::CadenceTier2 => "cadence_tier2",
        DigestKind::CadenceTier3 => "cadence_tier3",
        DigestKind::Reflection => "reflection",
        DigestKind::ResolutionSummary => "resolution_summary",
    }
}

// Tests live at `pulse-app/tests/unit_inference_runtime.rs` (integration
// test crate) per CLAUDE.md testing.md 2026-05-20 lesson — pulse-app's
// `[lib] test = false` setting disables source-level `mod tests` blocks
// on Windows due к WebView2 DLL load.
