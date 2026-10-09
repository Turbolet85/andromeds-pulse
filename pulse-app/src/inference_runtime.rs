//! L4 inference subscriber adapter (chunk #83 — Epoch 9 Foundation v0.2.0).
//!
//! Subscribes to `pulse://stream/digests` (chunk #81), invokes the
//! `LlmInferenceRunner::generate_constrained` trait surface with the
//! primary-tier prompt + embedded JSON schema, parses + validates the
//! output via `interpretation::schema::parse_bounded`, and emits
//! observability events along the way.
//!
//! Persistence of the parsed L4Output to incident records is deferred to
//! a follow-up chunk (chunk #86+ Findings counter + corpus integration)
//! because the IncidentPersistence trait surface from chunk #78 does NOT
//! currently accept a payload arg on `save_incident_event` AND extending
//! that surface is out of chunk #83's plan Files-to-modify scope. Chunk
//! #83 substrate establishes the subscriber + observability instrumentation
//! end-to-end; future chunks wire the persistence side once the parsed
//! L4Output → Incident mapping is fully specified.
//!
//! Pattern: mirrors chunk #81 `spawn_cadence_subscriber` +
//! `spawn_digest_persister` shape — long-running tokio task with recv loop
//! handling Ok / Lagged / Closed broadcast cases. Per arch §Cross-cutting
//! Patterns Module dependency direction, the subscriber lives at the
//! binary boundary; library crates stay Tauri-free + runtime-free.

use std::sync::Arc;
use std::time::{Duration, Instant};

use interpretation::contract::{InferenceError, LlmInferenceRunner, ModelTier};
use interpretation::degraded_mode::DegradedModeStatus;
use interpretation::hardware::profile_label;
use interpretation::prompt::{
    build_fallback_tier_prompt, build_primary_tier_prompt, build_reflection_tier_prompt,
};
use interpretation::schema::{
    Decision, L4_OUTPUT_JSON_SCHEMA, L4Output, PROMPT_VERSION_FALLBACK, PROMPT_VERSION_PRIMARY,
    PROMPT_VERSION_REFLECTION, Severity as L4Severity,
};
use security::scrubber::mask_secret_spans;
use tokio::sync::broadcast::error::RecvError;
use tokio::task::JoinHandle;
use triage::contract::{
    CueKind, CueScope, DamperVerdict, Digest, DigestBroadcast, DigestKind, EvidenceRefs,
    GenerationDamper, INCIDENT_EVENT_CREATED, Incident, IncidentPersistence, IncidentRegistry,
    IncidentStatus, PriorityTier, Severity as IncidentSeverity, cue_cause_label,
    generate_reason_label,
};

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
/// Tracing target — L4 inference skipped due to active degraded-mode
/// backoff window (chunk #86).
pub const TARGET_L4_INFERENCE_SKIPPED: &str = "interpretation.inference.skipped";
/// Tracing target — generation-damper state transitions. ONCE per
/// engage (first suppression of a run) and once per release (the
/// generate ending a run) — never per decision, per obs-plan §11
/// hot-path discipline. Aggregate/bounded fields only.
pub const TARGET_L4_DAMPER_TRANSITION: &str = "interpretation.generation.damper";
/// Tracing target — resolution-summary persist failures (chunk #86).
/// Sanitized error_category only.
pub const TARGET_L4_RESOLUTION_SUMMARY_PERSIST_ERROR: &str =
    "interpretation.resolution_summary.persist.error";
/// Metric target — degraded-mode cumulative seconds gauge (chunk #86).
pub const TARGET_METRIC_L4_DEGRADED_MODE_ACTIVE_SECONDS_TOTAL: &str =
    "metric.pipeline.l4.degraded_mode_active_seconds_total";
/// Metric target — count of degraded-mode entries since boot (chunk #86).
pub const TARGET_METRIC_L4_DEGRADED_MODE_ENTRIES_TOTAL: &str =
    "metric.pipeline.l4.degraded_mode_entries_total";
/// Metric target — remaining seconds until next eligible retry (chunk #86).
pub const TARGET_METRIC_L4_BACKOFF_REMAINING_SECONDS: &str =
    "metric.pipeline.l4.backoff_remaining_seconds";
/// Tracing target — incident created (or deduped) from a parsed L4Output
/// (chunk #92). Aggregate-only fields per the triage AllowList convention;
/// NEVER carries scope_id / service_name / incident_id / title / detail.
pub const TARGET_L4_INCIDENT_CREATED: &str = "interpretation.incident.created";
/// Tracing target — a cleanly-parsed generation that created no incident.
/// Once per such generation, naming which gate held; bounded labels only
/// (`skip_reason` / `decision` / `severity` / `digest_kind`), never
/// scope_id / title / symptom / digest or model text.
pub const TARGET_L4_INCIDENT_SKIPPED: &str = "interpretation.incident.skipped";
/// Tracing target — incident persist failures from the chunk #92 producer.
/// Sanitized `error_category` only.
pub const TARGET_L4_INCIDENT_PERSIST_ERROR: &str = "interpretation.incident.persist.error";
/// Metric target — count of incidents produced from L4 output (chunk #92);
/// label `result ∈ {created, deduped}`.
pub const TARGET_METRIC_L4_INCIDENTS_CREATED_TOTAL: &str =
    "metric.pipeline.l4.incidents_created_total";

/// Default heartbeat interval for the L4 queue-depth gauge emission task
/// (mirrors obs-plan §3 Heartbeat ticks 15s cadence).
pub const DEFAULT_QUEUE_DEPTH_TICK_INTERVAL: Duration = Duration::from_secs(15);

/// Build the project-context string passed into [`build_primary_tier_prompt`].
/// Pulls the workspace path + VCS hints from the L3 digest payload itself;
/// future chunks may enrich with recent commits + framework signals from
/// chunk #81's `DigestProjectContext`.
fn build_project_context(digest: &Digest) -> String {
    let mut ctx = String::with_capacity(256);
    ctx.push_str("workspace=");
    ctx.push_str(&digest.workspace);
    ctx
}

/// Collect the digest's REAL citable evidence ids — the attention cues'
/// full-hex L1 exception fingerprints (arch §Fault Identity: the 32-char
/// `hex_lower` form, the only encoding a citation can resolve against).
/// Deduped, order-stable; baseline-family cues carry `fingerprint: None`
/// and contribute nothing, so a cue-less or baseline-only digest yields an
/// empty list and the prompt instructs an empty `evidence_refs`.
fn citable_evidence_ids(digest: &Digest) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for cue in &digest.attention_cues {
        if let Some(fp) = &cue.fingerprint
            && !ids.iter().any(|existing| existing == fp)
        {
            ids.push(fp.clone());
        }
    }
    ids
}

/// Spawn the L4 inference subscriber. Subscribes to `digest_broadcast`,
/// invokes `runner` per digest, emits observability events. Threads the
/// chunk #86 degraded-mode FSM + incident registry/persistence so the
/// subscriber can (a) skip generation during active backoff windows,
/// (b) record success/failure outcomes to the FSM, (c) attach resolution
/// summaries to Resolved incidents on `DigestKind::ResolutionSummary`.
///
/// Returns the JoinHandle so callers can await graceful shutdown if needed.
pub fn spawn_l4_inference_subscriber(
    digest_broadcast: Arc<DigestBroadcast>,
    runner: Arc<dyn LlmInferenceRunner>,
    degraded_mode: Arc<dyn DegradedModeStatus>,
    incident_registry: Arc<dyn IncidentRegistry>,
    incident_persistence: Arc<dyn IncidentPersistence>,
    damper: Arc<GenerationDamper>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut rx = digest_broadcast.subscribe();
        loop {
            match rx.recv().await {
                Ok(digest) => {
                    let now = current_unix_nanos();
                    process_digest(
                        &*runner,
                        &*degraded_mode,
                        incident_registry.as_ref(),
                        incident_persistence.as_ref(),
                        &damper,
                        &digest,
                        now,
                    )
                    .await;
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

/// Single-digest processing pass: backoff gate → generation damper →
/// generation → outcome handling. Extracted from the subscriber loop so
/// integration tests can drive it without the broadcast machinery.
/// Returns the outcome when a generation ran; `None` when the digest was
/// skipped (active backoff, or damper suppression of unchanged input).
///
/// The damper is consulted AFTER the backoff gate and its key is updated
/// ONLY on `L4DigestOutcome::Success` — a failed generation must never
/// mark its content analyzed (a failing kind would self-suppress after
/// one attempt).
pub async fn process_digest(
    runner: &dyn LlmInferenceRunner,
    degraded_mode: &dyn DegradedModeStatus,
    incident_registry: &dyn IncidentRegistry,
    incident_persistence: &dyn IncidentPersistence,
    damper: &GenerationDamper,
    digest: &Digest,
    now: i64,
) -> Option<L4DigestOutcome> {
    if degraded_mode.is_in_backoff(now) {
        let snap = degraded_mode.current_snapshot(now);
        let tier_label = interpretation::contract::model_tier_label(runner.tier());
        tracing::info!(
            target: TARGET_L4_INFERENCE_SKIPPED,
            reason = "backoff_active",
            model_tier = tier_label,
            backoff_seconds_remaining = snap.backoff_seconds_remaining,
            "L4 inference skipped due to active backoff window",
        );
        return None;
    }
    match damper.decide(digest, now) {
        DamperVerdict::Suppress { engaged, run_len } => {
            if engaged {
                tracing::info!(
                    target: TARGET_L4_DAMPER_TRANSITION,
                    decision = "engaged",
                    reason = "unchanged_digest",
                    digest_kind = digest_kind_label(digest),
                    suppressed_run_len = run_len,
                    "generation damper engaged — unchanged digest, generation suppressed",
                );
            }
            return None;
        }
        DamperVerdict::Generate {
            reason,
            released_run_len,
        } => {
            if released_run_len > 0 {
                tracing::info!(
                    target: TARGET_L4_DAMPER_TRANSITION,
                    decision = "released",
                    reason = generate_reason_label(reason),
                    digest_kind = digest_kind_label(digest),
                    suppressed_run_len = released_run_len,
                    "generation damper released — digest content changed",
                );
            }
        }
    }
    let outcome = handle_digest_outcome(runner, digest).await;
    match &outcome {
        L4DigestOutcome::Success(parsed) => {
            degraded_mode.record_success(now);
            damper.record_generated(digest, now);
            if matches!(digest.kind, DigestKind::ResolutionSummary) || parsed.is_resolution_summary
            {
                if !matches!(digest.kind, DigestKind::ResolutionSummary) {
                    emit_incident_skipped(SKIP_MODEL_RESOLUTION_SUMMARY, digest, parsed);
                }
                attach_resolution_summary_to_incident(
                    incident_registry,
                    incident_persistence,
                    &digest.incident_refs,
                    parsed,
                    now,
                );
            } else {
                create_incident_from_l4_output(
                    incident_registry,
                    incident_persistence,
                    digest,
                    parsed,
                    now,
                );
            }
        }
        L4DigestOutcome::ParseFailure
        | L4DigestOutcome::SchemaViolation
        | L4DigestOutcome::OutputTooLarge
        | L4DigestOutcome::RuntimeError => {
            degraded_mode.record_failure(now);
        }
    }
    Some(outcome)
}

/// Wall-clock unix-nano helper. Mirrors `pulse-app/src/diagnostics_router.rs`
/// `current_unix_nanos` shape; reused for the chunk #86 degraded-mode FSM
/// timestamp injections.
pub fn current_unix_nanos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

/// Spawn the chunk #86 backoff-remaining gauge tick. Emits
/// `metric.pipeline.l4.backoff_remaining_seconds` every 15s (mirrors
/// chunk #83 `spawn_l4_queue_depth_heartbeat` cadence + obs-plan §3
/// Heartbeat ticks). Bounded to single `value` field per chunk #86 obs
/// constraint aggregate-only discipline.
pub fn spawn_l4_backoff_remaining_heartbeat(
    degraded_mode: Arc<dyn DegradedModeStatus>,
    damper: Arc<GenerationDamper>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(DEFAULT_QUEUE_DEPTH_TICK_INTERVAL);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            let now = current_unix_nanos();
            let snap = degraded_mode.current_snapshot(now);
            // Damper counters ride this existing heartbeat as tick-
            // aggregated fields (obs-plan §11 bars per-decision records).
            tracing::info!(
                target: TARGET_METRIC_L4_BACKOFF_REMAINING_SECONDS,
                value = snap.backoff_seconds_remaining,
                generations_suppressed_total = damper.generations_suppressed_total(),
                generations_run_total = damper.generations_run_total(),
                "L4 backoff remaining heartbeat",
            );
        }
    })
}

/// Spawn the queue-depth heartbeat tick task. Emits the
/// `metric.pipeline.l4.inference_queue_depth` gauge every
/// [`DEFAULT_QUEUE_DEPTH_TICK_INTERVAL`] (15s).
///
/// `queued_count_fn` returns the current count of digests awaiting L4
/// inference. At chunk #83 substrate the subscriber is a single-consumer
/// recv loop with no internal queue; the broadcast channel's
/// `sender.len()` would return zero most of the time. Future chunks with
/// LWW + active-incident bypass queues per dist-arch v3 §L4 will provide
/// a richer counter.
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

/// Classified outcome of a single L4 digest processing pass. Drives the
/// chunk #86 degraded-mode FSM transitions in
/// [`spawn_l4_inference_subscriber`] AND surfaces the parsed `L4Output`
/// for the resolution-summary attachment path. `L4Output` is boxed
/// because its serialized size dwarfs the other variants (per clippy
/// `large_enum_variant`).
#[derive(Debug)]
pub enum L4DigestOutcome {
    /// Generation + parse + validate succeeded; the parsed payload is
    /// attached for the resolution-summary path OR ignored otherwise.
    Success(Box<L4Output>),
    /// JSON parse failed (malformed output bytes). Drives
    /// `degraded_mode.record_failure`.
    ParseFailure,
    /// Schema validation failed (parse succeeded but bounded-shape
    /// invariant violated). Drives `degraded_mode.record_failure`.
    SchemaViolation,
    /// Output exceeded the defense-in-depth byte cap before parse.
    /// Drives `degraded_mode.record_failure`.
    OutputTooLarge,
    /// Runtime / LLM-side error (subprocess failure / model not
    /// configured / etc.). Drives `degraded_mode.record_failure`.
    RuntimeError,
}

/// Single-digest inference handling. Composes prompt → invokes runner →
/// parses result → emits observability. Backward-compat shim around
/// [`handle_digest_outcome`]; discards the outcome for existing test
/// callers that pre-date chunk #86. New degraded-mode-aware code uses
/// `handle_digest_outcome` directly.
pub async fn handle_digest(runner: &dyn LlmInferenceRunner, digest: &Digest) {
    let _ = handle_digest_outcome(runner, digest).await;
}

/// Outcome-returning single-digest inference handler. Identical
/// observability behavior to `handle_digest`; additionally surfaces a
/// classified `L4DigestOutcome` so callers (e.g.,
/// [`spawn_l4_inference_subscriber`]) can update the chunk #86
/// degraded-mode FSM + handle the resolution-summary attachment path.
pub async fn handle_digest_outcome(
    runner: &dyn LlmInferenceRunner,
    digest: &Digest,
) -> L4DigestOutcome {
    let tier = runner.tier();
    let started = Instant::now();
    let tier_label = interpretation::contract::model_tier_label(tier);
    let hardware_profile = profile_label(runner.hardware_profile());
    let digest_kind = digest_kind_label(digest);

    // Prompt assembly — branch on runner tier per chunk #85. Primary uses
    // the chunk #83 prompt builder; fallback uses the chunk #85
    // reduced-quality builder. Both consume the SAME L4 JSON schema (chunk
    // #83 substrate); the schema's `model_tier` field discriminates downstream.
    let prompt_started = Instant::now();
    let project_context = build_project_context(digest);
    let citable_ids = citable_evidence_ids(digest);
    let (prompt, prompt_version_label): (String, &'static str) = match (tier, digest.kind) {
        (ModelTier::Primary, DigestKind::Reflection) => (
            build_reflection_tier_prompt(
                &digest.payload_summary,
                &project_context,
                "",
                &citable_ids,
            ),
            PROMPT_VERSION_REFLECTION,
        ),
        (ModelTier::Primary, _) => (
            build_primary_tier_prompt(&digest.payload_summary, &project_context, "", &citable_ids),
            PROMPT_VERSION_PRIMARY,
        ),
        // Fallback-tier reflection digests reuse the fallback acute builder —
        // cumulative-trend emphasis is a primary-tier enrichment (chunk #98).
        (ModelTier::Fallback, _) => (
            build_fallback_tier_prompt(&digest.payload_summary, &project_context, "", &citable_ids),
            PROMPT_VERSION_FALLBACK,
        ),
    };
    let prompt_elapsed_ms = prompt_started.elapsed().as_millis() as u64;
    tracing::info!(
        target: TARGET_L4_PROMPT_ASSEMBLE,
        prompt_version = prompt_version_label,
        token_count = prompt.len() as u64,
        duration_ms = prompt_elapsed_ms,
        "L4 prompt assembled",
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
            handle_parse_outcome(
                &raw_output,
                tier_label,
                hardware_profile,
                digest_kind,
                total_elapsed_ms,
            )
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
                hardware_profile = hardware_profile,
                "L4 inference latency sample",
            );
            L4DigestOutcome::RuntimeError
        }
    }
}

fn handle_parse_outcome(
    raw_output: &str,
    tier_label: &'static str,
    hardware_profile: &'static str,
    digest_kind: &'static str,
    total_elapsed_ms: u64,
) -> L4DigestOutcome {
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
                hardware_profile = hardware_profile,
                "L4 inference latency sample",
            );
            L4DigestOutcome::Success(Box::new(parsed))
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
            L4DigestOutcome::OutputTooLarge
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
            L4DigestOutcome::ParseFailure
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
            L4DigestOutcome::SchemaViolation
        }
        Err(other) => {
            // Other variants (ModelNotConfigured / etc) routed via the
            // runtime-error path inside `handle_digest_outcome`; reaching
            // here would indicate a new InferenceError variant — fall
            // through to counter increment for visibility.
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
            L4DigestOutcome::RuntimeError
        }
    }
}

/// Attach the parsed L4Output as a resolution summary to the resolved
/// incident referenced by the digest's `incident_refs[0]`. Chunk #86
/// resolution-summary path; capability P-022 + P-059.
///
/// Discipline:
/// - Render summary as JSON-serialized L4Output (chunk #87 Report UI
///   parses back; preserves structured fields without a new schema).
/// - Mask each string field via `security::scrubber::mask_secret_spans` at
///   the persistence boundary per chunk #72 uniform-coverage invariant.
/// - Call `registry.attach_resolution_summary` (updates in-memory state).
/// - Call `persistence.update_incident_status` (rewrites full BLOB to
///   corpus via existing chunk #78 trait method; the new
///   `resolution_summary_text` field flows through via serde).
/// - Defensive skip on malformed `incident_refs` / invalid id / registry
///   error (NotFound or InvalidTransition — incident may have been
///   archived OR not yet transitioned to Resolved).
pub fn attach_resolution_summary_to_incident(
    registry: &dyn IncidentRegistry,
    persistence: &dyn IncidentPersistence,
    incident_refs: &[String],
    parsed: &L4Output,
    now_unix_nano: i64,
) {
    let Some(id_str) = incident_refs.first() else {
        return;
    };
    let Ok(id) = id_str.parse::<i64>() else {
        return;
    };
    let Some(incident) = registry.get(id) else {
        return;
    };

    let Some(scrubbed_text) = scrubbed_l4_json(&grounded_output(parsed, incident.kind)) else {
        return;
    };

    let updated = match registry.attach_resolution_summary(id, scrubbed_text, now_unix_nano) {
        Ok(u) => u,
        Err(_) => return,
    };

    if let Err(err) = persistence.update_incident_status(id, &updated) {
        tracing::warn!(
            target: TARGET_L4_RESOLUTION_SUMMARY_PERSIST_ERROR,
            error_category = err.error_category(),
            "resolution summary persist failed",
        );
    }
}

/// Order-preserving dedup union of the model's evidence refs and the
/// triggering cue's real full-hex fingerprint (when the cue carried one).
fn grounded_fingerprint_hashes(parsed_refs: &[String], cue_fp: Option<&str>) -> Vec<String> {
    let mut out = parsed_refs.to_vec();
    if let Some(fp) = cue_fp
        && !out.iter().any(|r| r == fp)
    {
        out.push(fp.to_string());
    }
    out
}

/// `{Cause label}: {model title}` — the incident's trigger named in its
/// title whatever the model wrote. Grounded BEFORE the scrub, which masks
/// only the model's words (no P-047 arm matches a cause label).
#[doc(hidden)]
pub fn grounded_title(kind: CueKind, model_title: &str) -> String {
    format!("{}: {}", cue_cause_label(kind), model_title)
}

/// `parsed` with its title grounded on the incident's cue kind; every other
/// field untouched.
fn grounded_output(parsed: &L4Output, kind: CueKind) -> L4Output {
    L4Output {
        title: grounded_title(kind, &parsed.title),
        ..parsed.clone()
    }
}

/// Serialize + scrub a parsed `L4Output` for attachment as the incident's
/// `resolution_summary_text` (the chunk #72 uniform-coverage invariant
/// applies at every persistence boundary). Every string leaf is masked
/// before serializing — never the serialized text, where `secret_kv`'s `\S+`
/// would run across compact-JSON delimiters — so the summary stays
/// parseable. `None` only when serialization fails.
#[doc(hidden)]
pub fn scrubbed_l4_json(parsed: &L4Output) -> Option<String> {
    let mut value = serde_json::to_value(parsed).ok()?;
    mask_string_leaves(&mut value);
    serde_json::to_string(&value).ok()
}

fn mask_string_leaves(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(text) => *text = scrub_text(text),
        serde_json::Value::Array(items) => items.iter_mut().for_each(mask_string_leaves),
        serde_json::Value::Object(fields) => fields.values_mut().for_each(mask_string_leaves),
        _ => {}
    }
}

/// Scrub a telemetry-derived text field at the persistence boundary per the
/// chunk #72 uniform-coverage invariant. Each secret is masked in place as a
/// bounded category marker so no raw secret content reaches the corpus BLOB.
fn scrub_text(raw: &str) -> String {
    mask_secret_spans(raw, |category| format!("[redacted: {category}]")).text
}

/// Map the L4 surface severity onto the triage incident `PriorityTier`
/// (1:1 for the three non-baseline tiers). `Severity::None` is filtered by
/// the caller before this is reached.
fn map_l4_priority_tier(severity: L4Severity) -> PriorityTier {
    match severity {
        L4Severity::Autonomous => PriorityTier::Autonomous,
        L4Severity::Suggested => PriorityTier::Suggested,
        L4Severity::Curious | L4Severity::None => PriorityTier::Curious,
    }
}

/// Map the L4 surface severity onto the triage `IncidentSeverity` hue axis
/// (drives the Halo / constellation hue). `Critical` is reserved for future
/// escalation per phase-89 plan §Implementation notes.
fn map_l4_incident_severity(severity: L4Severity) -> IncidentSeverity {
    match severity {
        L4Severity::Autonomous => IncidentSeverity::Error,
        L4Severity::Suggested => IncidentSeverity::Warn,
        L4Severity::Curious | L4Severity::None => IncidentSeverity::Info,
    }
}

/// Bounded enum label for the triage incident severity (aggregate-only obs).
fn incident_severity_label(severity: IncidentSeverity) -> &'static str {
    match severity {
        IncidentSeverity::Info => "info",
        IncidentSeverity::Warn => "warn",
        IncidentSeverity::Error => "error",
        IncidentSeverity::Critical => "critical",
    }
}

/// Bounded enum label for the incident priority tier (aggregate-only obs).
fn priority_tier_label(tier: PriorityTier) -> &'static str {
    match tier {
        PriorityTier::Autonomous => "autonomous",
        PriorityTier::Suggested => "suggested",
        PriorityTier::Curious => "curious",
    }
}

/// Produce a new `Incident` from a parsed L4Output (chunk #92 — the deferred
/// L4Output → Incident production path; capabilities P-022 / P-041 / P-027).
/// This is the producer that activates the chunk #91 per-service-severity
/// join by attributing each incident to its originating service via
/// `Incident.scope_id`.
///
/// Discipline mirrors [`attach_resolution_summary_to_incident`]:
/// - Creation predicate: skip resolution summaries, `Decision::Dismiss`,
///   `Severity::None`, OR digests with no triggering cue (incidents are strictly
///   cue-derived per the `Incident.kind` contract doc).
/// - Scrub every telemetry-derived text field via `mask_secret_spans` before
///   the corpus write (chunk #72 uniform-coverage invariant).
/// - Re-emission dedup on the `(kind, scope, scope_id)` per-service identity:
///   bump an existing active incident rather than creating a duplicate, so
///   distinct services keep distinct incidents. This is COALESCE-PER-CUE-IDENTITY,
///   a decided semantic rather than an omission — see the note at the predicate.
/// - Persist-then-insert: `save_new_incident` assigns the rowid → set
///   `Incident.id` → `registry.insert` → `save_incident_event`.
/// - Latest-interpretation attach: the scrubbed L4Output JSON lands on
///   `resolution_summary_text` at creation and refreshes on every dedupe,
///   so the report renders the model's content for LIVE incidents (the
///   resolution-summary generation, when it fires, is the final write).
/// - Evidence grounding: `fingerprint_hashes` is the union of the model's
///   refs and the cue's real fingerprint.
/// - Aggregate-only observability; never panics on a corpus result.
pub fn create_incident_from_l4_output(
    registry: &dyn IncidentRegistry,
    persistence: &dyn IncidentPersistence,
    digest: &Digest,
    parsed: &L4Output,
    now_unix_nano: i64,
) {
    let skip_reason = if parsed.is_resolution_summary {
        Some(SKIP_MODEL_RESOLUTION_SUMMARY)
    } else if parsed.decision == Decision::Dismiss {
        Some("decision_dismiss")
    } else if parsed.severity == L4Severity::None {
        Some("severity_none")
    } else {
        None
    };
    if let Some(reason) = skip_reason {
        emit_incident_skipped(reason, digest, parsed);
        return;
    }

    // Identity derivation. Cue-triggered digests are service-attributable:
    // (kind, scope, scope_id) come from the triggering cue. Reflection-cadence
    // digests (chunk #98) carry no cue — they get the synthetic
    // workspace-global ReflectionTrend identity (scope_id = None) so the
    // cumulative-trend incident dedups one-per-workspace WITHOUT touching the
    // per-service constellation join (which keys on scope_id). All other
    // non-cue digests (baseline cadence) still skip — incidents stay strictly
    // cue-or-reflection-derived.
    let (kind, scope, scope_id, cue_fingerprint) = if digest.kind == DigestKind::Reflection {
        (CueKind::ReflectionTrend, CueScope::Global, None, None)
    } else if let Some(cue) = digest.attention_cues.first() {
        (
            cue.kind,
            cue.scope,
            cue.scope_id.as_deref().map(scrub_text),
            cue.fingerprint.clone(),
        )
    } else {
        emit_incident_skipped("no_cue", digest, parsed);
        return;
    };

    let grounded = grounded_output(parsed, kind);
    let severity = map_l4_incident_severity(parsed.severity);
    let priority_tier = map_l4_priority_tier(parsed.severity);

    // Re-emission dedup on the identity tuple — distinct services (distinct
    // scope_id) keep distinct incidents; reflection trends dedup per workspace.
    //
    // DECIDED SEMANTIC — incident identity is coalesce-per-cue-identity, which
    // for storms means per-service. A storm carrying a DIFFERENT exception
    // fingerprint on a service that already has an open incident is absorbed
    // here BY DESIGN, not by oversight (arch §Established Decisions [Fault
    // Identity], Layer 2).
    //
    // The cue NOW carries a real fingerprint, so the availability argument that
    // once justified this key no longer applies — the decision rests on the
    // ground that survives: incident-per-identity buys no correctness today
    // because every downstream surface is already N-safe (the per-service
    // constellation join reduces by max tier over all matching active
    // incidents; the digest assembler reads `list_active` only as a boolean).
    // A second concurrent incident would change no rendering and no L4
    // behaviour — it would only add rows. Adding `fingerprint` to this key is
    // therefore a product decision that is now POSSIBLE but not taken; it
    // belongs to a route entry that weighs it, not to an edit here.
    // `parsed.fingerprint` remains unusable for it regardless: model-authored,
    // and a constant under the deterministic runner.
    if let Some(existing) = registry
        .list_active(&digest.workspace)
        .into_iter()
        .find(|inc| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)
    {
        if registry
            .observe_reemission(existing.id, now_unix_nano)
            .is_ok()
        {
            // Refresh the latest interpretation on the deduped incident so
            // the report tracks the newest cleanly-parsed generation. Silent
            // attachment (chunk #86 precedent); a failure skips defensively
            // (Resolved is unreachable here — observe_reemission rejected it)
            // and the single persist below carries whatever state stands.
            if let Some(json) = scrubbed_l4_json(&grounded) {
                let _ = registry.attach_interpretation_summary(existing.id, json, now_unix_nano);
            }
            if let Some(updated) = registry.get(existing.id)
                && let Err(err) = persistence.update_incident_status(existing.id, &updated)
            {
                tracing::warn!(
                    target: TARGET_L4_INCIDENT_PERSIST_ERROR,
                    error_category = err.error_category(),
                    "incident reemission persist failed",
                );
            }
        }
        emit_incident_outcome(false, true, severity, priority_tier);
        return;
    }

    // Ground-truth union BEFORE the construction below moves
    // `cue_fingerprint`: the model's (now prompt-copied) refs plus the
    // triggering cue's real fingerprint, deduped. The incident record
    // carries a real id regardless of model behavior; parsed refs stay
    // FIRST (the deterministic P-073 contains-pins ride them).
    let fingerprint_hashes =
        grounded_fingerprint_hashes(&parsed.evidence_refs, cue_fingerprint.as_deref());

    let mut incident = Incident {
        id: 0,
        workspace: digest.workspace.clone(),
        // The L1 exception fingerprint the triggering cue carried — the
        // anonymized lowercase-hex grouping hash `Incident.fingerprint` is
        // contracted as, and the value the corpus-retrieval `fingerprint_match`
        // arm compares against. Empty when no cue supplied one (reflection
        // cadence, baseline families); both retrieval selectors guard on
        // non-empty and fall through to scope matching.
        //
        // NOT `parsed.fingerprint`: that is model-authored free text, and a
        // constant under the deterministic runner — writing it here is the
        // defect this producer previously had.
        fingerprint: cue_fingerprint.unwrap_or_default(),
        title: scrub_text(&grounded.title),
        detail: scrub_text(&parsed.symptom),
        kind,
        scope,
        scope_id,
        status: IncidentStatus::Active,
        severity,
        priority_tier,
        evidence_refs: EvidenceRefs {
            trace_id: None,
            span_ids: Vec::new(),
            fingerprint_hashes,
            timestamps_unix_nano: Vec::new(),
        },
        opened_at_unix_nano: now_unix_nano,
        updated_at_unix_nano: now_unix_nano,
        acknowledged_at_unix_nano: None,
        resolved_at_unix_nano: None,
        read_at_unix_nano: None,
        // The latest cleanly-parsed interpretation attaches AT CREATION, so
        // the report renders the model's content for a live incident instead
        // of a false-degraded notice. The resolution-summary generation, when
        // it fires, is the final write to this field.
        resolution_summary_text: scrubbed_l4_json(&grounded),
    };

    let id = match persistence.save_new_incident(&incident) {
        Ok(id) => id,
        Err(err) => {
            tracing::warn!(
                target: TARGET_L4_INCIDENT_PERSIST_ERROR,
                error_category = err.error_category(),
                "incident create persist failed",
            );
            return;
        }
    };
    incident.id = id;
    registry.insert(incident);
    if let Err(err) = persistence.save_incident_event(id, INCIDENT_EVENT_CREATED, now_unix_nano) {
        tracing::warn!(
            target: TARGET_L4_INCIDENT_PERSIST_ERROR,
            error_category = err.error_category(),
            "incident created-event persist failed",
        );
    }
    emit_incident_outcome(true, false, severity, priority_tier);
}

const SKIP_MODEL_RESOLUTION_SUMMARY: &str = "model_resolution_summary";

fn decision_label(decision: Decision) -> &'static str {
    match decision {
        Decision::Surface => "surface",
        Decision::Dismiss => "dismiss",
        Decision::Watch => "watch",
    }
}

fn l4_severity_label(severity: L4Severity) -> &'static str {
    match severity {
        L4Severity::Autonomous => "autonomous",
        L4Severity::Suggested => "suggested",
        L4Severity::Curious => "curious",
        L4Severity::None => "none",
    }
}

/// Emit the no-incident outcome of a cleanly-parsed generation. Without it a
/// model dismissal, a `severity: none`, a model-set `is_resolution_summary`
/// and a cue-less digest all leave the same footprint (parse ok, no
/// `interpretation.incident.created`).
fn emit_incident_skipped(skip_reason: &'static str, digest: &Digest, parsed: &L4Output) {
    tracing::info!(
        target: TARGET_L4_INCIDENT_SKIPPED,
        skip_reason = skip_reason,
        decision = decision_label(parsed.decision),
        severity = l4_severity_label(parsed.severity),
        digest_kind = digest_kind_label(digest),
        "incident producer skipped",
    );
}

/// Emit the aggregate-only producer-outcome event + counter. Bounded fields
/// only — never scope_id / service_name / incident_id / title / detail.
fn emit_incident_outcome(
    created: bool,
    deduped: bool,
    severity: IncidentSeverity,
    priority_tier: PriorityTier,
) {
    let severity_label = incident_severity_label(severity);
    let tier_label = priority_tier_label(priority_tier);
    tracing::info!(
        target: TARGET_L4_INCIDENT_CREATED,
        created = created,
        deduped = deduped,
        severity = severity_label,
        priority_tier = tier_label,
        "incident producer outcome",
    );
    tracing::info!(
        target: TARGET_METRIC_L4_INCIDENTS_CREATED_TOTAL,
        value = 1u64,
        result = if created { "created" } else { "deduped" },
        "incident producer counter",
    );
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
// on Windows due to WebView2 DLL load.
