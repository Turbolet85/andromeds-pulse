//! L3 digest assembler core implementation (chunk #81).
//!
//! `DigestAssembler` trait + `Assembler` concrete impl. Composes a
//! `Digest` per dist-arch v3 §Appendix C from L1a Q1-Q7 (via
//! `SqlQueryRunner` injected from chunk #80 substrate), attention cues
//! (from `AttentionCue` triggering arg), project context (via
//! `DigestProjectContext` injected by binary adapter), and active-incident
//! state (via `IncidentRegistry` injected from chunk #78 substrate).
//!
//! ## Corpus retrieval (capability P-044)
//!
//! The CORPUS MATCHES section carries top-N similar past incidents —
//! same workspace, last 30 days, fingerprint or scope match, top-5,
//! newest first — supplied by the injected [`CorpusIncidentSource`]
//! and selected via `digest::retrieval::select_corpus_matches`.
//! Candidate fingerprints come from the window's Q3 rows; candidate
//! scopes from the Q1 service set. Retrieval failure or an absent
//! corpus degrades to an empty section — the digest always assembles.
//!
//! ## Token-budget enforcement
//!
//! Hugging Face `tokenizers` (Llama-3 fixture downloaded at build time
//! per `crates/triage/build.rs` Phase 2). Soft target 500-2000; hard cap
//! 3000. Over-cap triggers truncation strategy: drop `corpus_matches`,
//! then lowest-priority `attention_cues`, then `services` rows by lowest
//! anomaly severity. `payload_summary` text re-rendered after truncation.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokenizers::Tokenizer;

use crate::cadence::CadenceMode;
use crate::contract::{
    AttentionCue, CueKind, Digest, DigestCueRef, DigestKind, DigestLwwMode, DigestServiceRow,
    IncidentStatus, PriorityTier, Severity, SqlAggregationError, SqlQueryRunner, cue_cause_label,
    hex_lower,
};
use crate::digest::broadcast::DigestBroadcast;
use crate::digest::queue::{LwwQueue, QueueAction};
use crate::digest::retrieval::{
    CORPUS_RETRIEVAL_WINDOW_SECONDS, CorpusIncidentSource, format_corpus_match_line,
    select_corpus_matches,
};
use crate::digest::{
    DIGEST_CORPUS_RETRIEVAL_LIMIT, DIGEST_TOKEN_BUDGET_HARD_CAP, DIGEST_TOKEN_BUDGET_SOFT_MAX,
    DIGEST_TOKEN_BUDGET_SOFT_MIN, DigestError, TARGET_DIGEST_ASSEMBLE,
    TARGET_DIGEST_CORPUS_RETRIEVE, TARGET_DIGEST_LWW_DROP, TARGET_DIGEST_LWW_REPLACE,
    TARGET_DIGEST_TOKEN_COUNT_VALIDATE, TARGET_METRIC_ACTIVE_INCIDENT_QUEUE_DEPTH,
    TARGET_METRIC_DIGEST_TOKEN_COUNT_MS, TARGET_METRIC_LWW_DROP_COUNT_TOTAL,
};
use crate::incident::IncidentRegistry;

/// Build-time-embedded Llama-3 tokenizer.json fixture (per
/// `crates/triage/build.rs` Phase 2). Future chunk #82+ may swap to a
/// different tokenizer if LLM runtime choice mandates.
const TOKENIZER_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tokenizer.json"));

/// Future return-position alias used by `DigestAssembler::assemble`.
/// Manual `Pin<Box<dyn Future + Send + 'a>>` returns keep the trait
/// object-safe (`Arc<dyn DigestAssembler>`) without `async-trait` dep
/// (per CLAUDE.md §Session Learnings 2026-05-23 manual `Pin<Box<dyn
/// Future>>` pattern).
pub type DigestFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, DigestError>> + Send + 'a>>;

/// Project context inputs to digest assembly. Constructed by the binary
/// adapter at `pulse-app/src/digest_runtime.rs` from
/// `workspace_detector::ProjectContextProvider`. Triage owns this lower-
/// level shape; binary-side does the mapping per arch §Module dependency
/// direction (triage does not depend on workspace-detector).
#[derive(Debug, Clone, Default)]
pub struct DigestProjectContext {
    pub workspace_canonical_path: String,
    pub project_name: Option<String>,
    pub vcs_type: Option<&'static str>,
    pub recent_commits: Vec<DigestRecentCommit>,
    pub framework_signals: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct DigestRecentCommit {
    pub basename: String,
    pub age_seconds: u64,
    pub files_changed_count: u32,
}

/// Trait for assembling a digest. Implemented by [`Assembler`]; tests
/// can substitute a stub.
pub trait DigestAssembler: Send + Sync {
    fn assemble<'a>(
        &'a self,
        mode: CadenceMode,
        triggering_cue: Option<&'a AttentionCue>,
        project_context: &'a DigestProjectContext,
        now_unix_nano: i64,
        window_duration: Duration,
    ) -> DigestFuture<'a, Digest>;
}

/// L3 digest assembler. Composes a digest from L1a queries + L2 cues +
/// active-incident state + project context.
///
/// Cross-crate dep injection per chunks #69/78 precedent:
/// - `Arc<dyn SqlQueryRunner>` from chunk #80 cadence substrate
/// - `Arc<dyn IncidentRegistry>` from chunk #78 substrate
/// - `Arc<DigestBroadcast>` for `pulse://stream/digests` emit
/// - `Arc<Mutex<LwwQueue>>` for LWW + active-incident bypass tracking
/// - `Arc<Tokenizer>` for token-count budget enforcement
/// - `Box<dyn Fn(&str) -> String + Send + Sync>` scrub closure
///   (`security::scrubber::scrub_attribute`-wrapping at binary boundary)
///
/// Corpus writes (digest_archive append) are routed through an injected
/// adapter at the binary boundary to keep triage free of corpus crate dep
/// (the assembler emits its digest and the binary boundary captures the
/// emission, scrubs + persists). For chunk #81 substrate, this is
/// implemented via the broadcast subscription pattern: pulse-app
/// subscribes to `DigestBroadcast` and persists each emitted digest to
/// corpus via `CorpusWriter::save_digest` (chunk #81 trait extension).
pub struct Assembler {
    tokenizer: Arc<Tokenizer>,
    sql_runner: Arc<dyn SqlQueryRunner>,
    incident_registry: Arc<dyn IncidentRegistry>,
    broadcast: Arc<DigestBroadcast>,
    queue: Arc<Mutex<LwwQueue>>,
    scrub: Arc<dyn Fn(&str) -> String + Send + Sync>,
    corpus_source: Arc<dyn CorpusIncidentSource>,
}

impl std::fmt::Debug for Assembler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Assembler")
            .field("broadcast_subscribers", &self.broadcast.subscriber_count())
            .finish()
    }
}

impl Assembler {
    /// Construct an assembler. Tokenizer loaded once at construction;
    /// fixture embedded at build time via `include_bytes!`.
    pub fn new(
        sql_runner: Arc<dyn SqlQueryRunner>,
        incident_registry: Arc<dyn IncidentRegistry>,
        broadcast: Arc<DigestBroadcast>,
        queue: Arc<Mutex<LwwQueue>>,
        scrub: Arc<dyn Fn(&str) -> String + Send + Sync>,
        corpus_source: Arc<dyn CorpusIncidentSource>,
    ) -> Result<Self, DigestError> {
        let tokenizer = Tokenizer::from_bytes(TOKENIZER_BYTES)
            .map_err(|e| DigestError::Tokenizer(e.to_string()))?;
        Ok(Self {
            tokenizer: Arc::new(tokenizer),
            sql_runner,
            incident_registry,
            broadcast,
            queue,
            scrub,
            corpus_source,
        })
    }

    /// Test-only constructor that takes a pre-built tokenizer (lets unit
    /// tests inject a tokenizer they constructed elsewhere). Production
    /// callers use `new()`.
    #[cfg(test)]
    pub fn with_tokenizer(
        tokenizer: Arc<Tokenizer>,
        sql_runner: Arc<dyn SqlQueryRunner>,
        incident_registry: Arc<dyn IncidentRegistry>,
        broadcast: Arc<DigestBroadcast>,
        queue: Arc<Mutex<LwwQueue>>,
        scrub: Arc<dyn Fn(&str) -> String + Send + Sync>,
        corpus_source: Arc<dyn CorpusIncidentSource>,
    ) -> Self {
        Self {
            tokenizer,
            sql_runner,
            incident_registry,
            broadcast,
            queue,
            scrub,
            corpus_source,
        }
    }

    /// Count tokens of the rendered payload using the loaded tokenizer.
    /// Returns 0 on encoding failure (tokenizer should always succeed
    /// on valid UTF-8 input; treat failures as token-count=0 +
    /// observability event).
    fn count_tokens(&self, text: &str) -> usize {
        self.tokenizer
            .encode(text, false)
            .map(|enc| enc.get_ids().len())
            .unwrap_or(0)
    }
}

impl DigestAssembler for Assembler {
    fn assemble<'a>(
        &'a self,
        mode: CadenceMode,
        triggering_cue: Option<&'a AttentionCue>,
        project_context: &'a DigestProjectContext,
        now_unix_nano: i64,
        window_duration: Duration,
    ) -> DigestFuture<'a, Digest> {
        Box::pin(async move {
            let workspace = project_context.workspace_canonical_path.clone();
            let mode_label = crate::cadence::mode_label(mode);
            tracing::info!(
                target: TARGET_DIGEST_ASSEMBLE,
                mode = mode_label,
                cue_kind = triggering_cue.map(|c| cue_kind_label(c.kind)).unwrap_or(""),
                cue_priority_tier = triggering_cue
                    .map(|c| priority_tier_label(c.priority_tier))
                    .unwrap_or(""),
                "digest assemble start",
            );

            // Determine LWW mode based on cadence + active-incident state.
            let active_incidents = self.incident_registry.list_active(&workspace);
            let has_critical_active = active_incidents.iter().any(|i| {
                i.status != IncidentStatus::Resolved && severity_at_least_suggested(i.severity)
            });
            let lww_mode = match mode {
                CadenceMode::Tier1 => DigestLwwMode::Tier1NeverLww,
                CadenceMode::Reflection => DigestLwwMode::Reflection,
                CadenceMode::Tier2 | CadenceMode::Tier3 => {
                    if has_critical_active {
                        DigestLwwMode::ActiveIncidentBypass
                    } else {
                        DigestLwwMode::Default
                    }
                }
            };
            let active_incident_bypass = matches!(lww_mode, DigestLwwMode::ActiveIncidentBypass);

            // L1a: fetch Q1-Q7 results.
            let q1 = self
                .sql_runner
                .run_q1(window_duration)
                .await
                .map_err(map_sql_err)?;
            // Q2/Q4-Q7 fetched but not embedded in chunk #81 substrate digest;
            // execution stays to ensure the SQL surface is exercised
            // identically to the cadence coordinator (consistency +
            // diagnostics value). Future chunk integrates richer fields.
            // Q3 fingerprint rows feed the corpus-retrieval match below.
            let _q2 = self.sql_runner.run_q2(window_duration).await.ok();
            let q3 = self.sql_runner.run_q3(window_duration).await.ok();
            let _q4 = self.sql_runner.run_q4(window_duration).await.ok();
            let _q5 = self.sql_runner.run_q5(window_duration).await.ok();
            let _q6 = self.sql_runner.run_q6(window_duration).await.ok();
            let _q7 = self.sql_runner.run_q7(window_duration).await.ok();

            // Compose SERVICES section from Q1 (RED metrics). Q1RedRow
            // shape carries per-service rate + error count + p99 latency.
            let services = compose_services_from_q1(&q1);

            // Compose ATTENTION CUES section from triggering cue (if any).
            let attention_cues = triggering_cue
                .map(|c| {
                    vec![DigestCueRef {
                        kind: c.kind,
                        priority_tier: c.priority_tier,
                        summary: cue_summary(c),
                        scope: c.scope,
                        fingerprint: c.fingerprint.clone(),
                        scope_id: c.scope_id.clone(),
                    }]
                })
                .unwrap_or_default();

            // CORPUS MATCHES (capability P-044): same-workspace, last-30-day
            // candidates matched on the window's Q3 fingerprints + Q1 service
            // scopes; top-5 newest first. Under a triggering cue that carries a
            // scope_id the scope arm keeps that scope's incidents alone. Each
            // line is routed through the injected scrub closure at this egress
            // boundary (chunk #88 precedent) — `Digest::scrubbed_clone`
            // deliberately skips `corpus_matches`, so this is the field's only
            // scrub pass.
            let current_fingerprints: Vec<String> = q3
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .map(|row| hex_lower(&row.fingerprint))
                .collect();
            let current_scopes: Vec<String> =
                services.iter().map(|row| row.service.clone()).collect();
            let retrieval_since = now_unix_nano
                .saturating_sub(CORPUS_RETRIEVAL_WINDOW_SECONDS.saturating_mul(1_000_000_000));
            let retrieval_start = Instant::now();
            let corpus_matches: Vec<String> = match self
                .corpus_source
                .load_candidates(&workspace, retrieval_since)
                .await
            {
                Ok(candidates) => {
                    let candidate_count = candidates.len();
                    let selected = select_corpus_matches(
                        candidates,
                        &current_fingerprints,
                        &current_scopes,
                        triggering_cue.and_then(|c| c.scope_id.as_deref()),
                        DIGEST_CORPUS_RETRIEVAL_LIMIT,
                    );
                    tracing::info!(
                        target: TARGET_DIGEST_CORPUS_RETRIEVE,
                        query_id = "incidents_for_workspace_since",
                        param_count = 2_u64,
                        row_count_returned = candidate_count as u64,
                        duration_ms = retrieval_start.elapsed().as_millis() as u64,
                        "corpus retrieval for digest",
                    );
                    selected
                        .iter()
                        .map(|i| (self.scrub)(&format_corpus_match_line(i, now_unix_nano)))
                        .collect()
                }
                Err(_) => {
                    tracing::warn!(
                        target: TARGET_DIGEST_CORPUS_RETRIEVE,
                        query_id = "incidents_for_workspace_since",
                        error_category = "retrieval_failed",
                        duration_ms = retrieval_start.elapsed().as_millis() as u64,
                        "corpus retrieval failed; digest assembles with empty corpus_matches",
                    );
                    Vec::new()
                }
            };

            // Incident refs included in digest if active.
            let incident_refs: Vec<String> = active_incidents
                .iter()
                .filter(|i| severity_at_least_suggested(i.severity))
                .map(|i| i.id.to_string())
                .collect();

            // Render payload_summary as Appendix C text.
            let payload_summary = render_payload(
                window_duration,
                mode_label,
                project_context,
                &services,
                &attention_cues,
                &corpus_matches,
                &incident_refs,
                active_incident_bypass,
            );

            // Token-count + truncation.
            let mut token_count = self.count_tokens(&payload_summary);
            let mut budget_exceeded = false;
            let mut final_payload = payload_summary;
            let mut final_services = services;
            let mut final_cues = attention_cues;
            let mut final_corpus = corpus_matches;
            if token_count > DIGEST_TOKEN_BUDGET_HARD_CAP {
                budget_exceeded = true;
                // Strategy: drop corpus_matches, then attention_cues
                // (lowest priority first), then services rows. Re-render
                // after each step.
                final_corpus.clear();
                final_payload = render_payload(
                    window_duration,
                    mode_label,
                    project_context,
                    &final_services,
                    &final_cues,
                    &final_corpus,
                    &incident_refs,
                    active_incident_bypass,
                );
                token_count = self.count_tokens(&final_payload);
                while token_count > DIGEST_TOKEN_BUDGET_HARD_CAP && !final_cues.is_empty() {
                    // Drop lowest-priority cue first (Curious < Suggested < Autonomous).
                    let drop_idx = lowest_priority_cue_index(&final_cues);
                    final_cues.remove(drop_idx);
                    final_payload = render_payload(
                        window_duration,
                        mode_label,
                        project_context,
                        &final_services,
                        &final_cues,
                        &final_corpus,
                        &incident_refs,
                        active_incident_bypass,
                    );
                    token_count = self.count_tokens(&final_payload);
                }
                while token_count > DIGEST_TOKEN_BUDGET_HARD_CAP && !final_services.is_empty() {
                    // Drop lowest-anomaly-severity service row.
                    let drop_idx = lowest_anomaly_service_index(&final_services);
                    final_services.remove(drop_idx);
                    final_payload = render_payload(
                        window_duration,
                        mode_label,
                        project_context,
                        &final_services,
                        &final_cues,
                        &final_corpus,
                        &incident_refs,
                        active_incident_bypass,
                    );
                    token_count = self.count_tokens(&final_payload);
                }
            }

            // Emit token-count metric event + validation event.
            tracing::info!(
                target: TARGET_METRIC_DIGEST_TOKEN_COUNT_MS,
                value = token_count as u64,
                mode = mode_label,
                token_count_actual = token_count as u64,
                token_budget_limit = DIGEST_TOKEN_BUDGET_HARD_CAP as u64,
                budget_exceeded = budget_exceeded,
                "digest token count",
            );
            if budget_exceeded && token_count > DIGEST_TOKEN_BUDGET_HARD_CAP {
                tracing::error!(
                    target: TARGET_DIGEST_TOKEN_COUNT_VALIDATE,
                    mode = mode_label,
                    token_count_actual = token_count as u64,
                    token_budget_limit = DIGEST_TOKEN_BUDGET_HARD_CAP as u64,
                    budget_exceeded = true,
                    truncation_applied = true,
                    "digest exceeded hard cap after truncation",
                );
            } else if token_count < DIGEST_TOKEN_BUDGET_SOFT_MIN {
                tracing::info!(
                    target: TARGET_DIGEST_TOKEN_COUNT_VALIDATE,
                    mode = mode_label,
                    token_count_actual = token_count as u64,
                    token_budget_limit = DIGEST_TOKEN_BUDGET_HARD_CAP as u64,
                    soft_min = DIGEST_TOKEN_BUDGET_SOFT_MIN as u64,
                    budget_exceeded = false,
                    truncation_applied = false,
                    "digest below soft minimum (sparse window)",
                );
            } else if token_count > DIGEST_TOKEN_BUDGET_SOFT_MAX {
                tracing::warn!(
                    target: TARGET_DIGEST_TOKEN_COUNT_VALIDATE,
                    mode = mode_label,
                    token_count_actual = token_count as u64,
                    token_budget_limit = DIGEST_TOKEN_BUDGET_HARD_CAP as u64,
                    soft_max = DIGEST_TOKEN_BUDGET_SOFT_MAX as u64,
                    budget_exceeded = false,
                    truncation_applied = false,
                    "digest above soft maximum but under hard cap",
                );
            }

            // Compose final digest.
            let kind = match mode {
                CadenceMode::Tier1 => DigestKind::CadenceTier1,
                CadenceMode::Tier2 => DigestKind::CadenceTier2,
                CadenceMode::Tier3 => DigestKind::CadenceTier3,
                CadenceMode::Reflection => DigestKind::Reflection,
            };
            let window_start_unix_nano = now_unix_nano - (window_duration.as_nanos() as i64);
            let mut digest = Digest {
                kind,
                token_count,
                payload_summary: final_payload,
                incident_refs,
                generated_at_unix_nano: now_unix_nano,
                workspace: workspace.clone(),
                window_start_unix_nano,
                window_end_unix_nano: now_unix_nano,
                services: final_services,
                attention_cues: final_cues,
                corpus_matches: final_corpus,
                lww_mode,
                active_incident_bypass,
                resolution_event: false,
            };

            // Apply PII scrubber.
            let scrub = Arc::clone(&self.scrub);
            digest = digest.scrubbed_clone(|s| scrub(s));

            // LWW queue + observability emit.
            let action = {
                let mut q = self.queue.lock().expect("LwwQueue mutex poisoned");
                q.push(digest.clone())
            };
            match &action {
                QueueAction::Queued => {}
                QueueAction::Replaced { replaced_kind } => {
                    tracing::info!(
                        target: TARGET_DIGEST_LWW_REPLACE,
                        mode = mode_label,
                        cadence_tier = mode_label,
                        active_incident_bypass,
                        replaced_kind = replaced_kind.as_str(),
                        "digest LWW replace",
                    );
                    tracing::info!(
                        target: TARGET_METRIC_LWW_DROP_COUNT_TOTAL,
                        value = 1_u64,
                        cadence_tier = mode_label,
                        drop_reason = "lww_replace",
                        "lww drop count increment",
                    );
                }
                QueueAction::DroppedOldest { dropped_kind } => {
                    tracing::warn!(
                        target: TARGET_DIGEST_LWW_DROP,
                        mode = mode_label,
                        cadence_tier = mode_label,
                        drop_reason = "queue_cap_reached",
                        dropped_kind = dropped_kind.as_str(),
                        "digest queue dropped oldest at cap",
                    );
                    tracing::info!(
                        target: TARGET_METRIC_LWW_DROP_COUNT_TOTAL,
                        value = 1_u64,
                        cadence_tier = mode_label,
                        drop_reason = "queue_cap_reached",
                        "lww drop count increment",
                    );
                }
            }

            // Emit active-incident queue depth gauge.
            let depth = {
                let q = self.queue.lock().expect("LwwQueue mutex poisoned");
                q.active_incident_depth()
            };
            if active_incident_bypass {
                tracing::info!(
                    target: TARGET_METRIC_ACTIVE_INCIDENT_QUEUE_DEPTH,
                    value = depth as u64,
                    severity_tier = "suggested_or_higher",
                    queue_depth = depth as u64,
                    "active incident queue depth gauge",
                );
            }

            // Broadcast emit. send().is_err() = no subscribers, which is
            // benign (broadcast semantics; chunk #80 precedent uses
            // `let _ = ...` discard).
            let _ = self.broadcast.sender().send(digest.clone());

            Ok(digest)
        })
    }
}

fn map_sql_err(_e: SqlAggregationError) -> DigestError {
    DigestError::SqlAggregation
}

fn cue_kind_label(kind: CueKind) -> &'static str {
    match kind {
        CueKind::ErrorRateSpike => "error_rate_spike",
        CueKind::LatencyRegression => "latency_regression",
        CueKind::RestartEvent => "restart_event",
        CueKind::ServiceWentSilent => "service_went_silent",
        CueKind::RetryStorm => "retry_storm",
        CueKind::ReflectionTrend => "reflection_trend",
    }
}

fn priority_tier_label(tier: PriorityTier) -> &'static str {
    match tier {
        PriorityTier::Autonomous => "autonomous",
        PriorityTier::Suggested => "suggested",
        PriorityTier::Curious => "curious",
    }
}

fn severity_at_least_suggested(s: Severity) -> bool {
    // Severity ≥ Warn means triggering of the active-incident exception
    // per dist-arch v3 §Queue behavior (severity ≥ Suggested per spec
    // language; map to Severity::Warn since Severity enum lacks
    // explicit Suggested tier — Severity carries Info/Warn/Error/Critical
    // levels while PriorityTier carries the model-decision tiers).
    matches!(s, Severity::Warn | Severity::Error | Severity::Critical)
}

fn compose_services_from_q1(rows: &[crate::contract::Q1RedRow]) -> Vec<DigestServiceRow> {
    rows.iter()
        .map(|r| {
            // Q1 rows expose RED-style aggregates (rate, error count, p99
            // latency over the window). Chunk #81 substrate maps directly;
            // baselines per row default to the current observation so that
            // the "vs baselines" comparison reads neutral (×1.0) until
            // chunk #82+ integrates L1b baseline state. Q1RedRow stores
            // p99 in nanoseconds (per chunk #79 sql.rs); convert to ms for
            // digest payload.
            let p99_ms = r.p99_ns as f64 / 1_000_000.0;
            DigestServiceRow {
                service: r.service_name.clone(),
                rate_per_sec: r.request_count as f64 / 60.0_f64.max(1.0),
                rate_baseline_per_sec: r.request_count as f64 / 60.0_f64.max(1.0),
                error_rate: r.error_rate,
                error_rate_baseline: r.error_rate,
                p99_latency_ms: p99_ms,
                p99_baseline_ms: p99_ms,
            }
        })
        .collect()
}

/// Prefix of the digest line naming the cue this digest was triggered by
/// (the first cue, which the producer takes the incident identity from).
/// ASCII: the line reaches the llama-cli argv inside `payload_summary`.
#[doc(hidden)]
pub const TRIGGER_LINE_PREFIX: &str = "TRIGGER: ";

/// Note rendered directly under the `CORPUS MATCHES:` header so the model
/// reads the matches as other incidents, never as the signal being reported.
#[doc(hidden)]
pub const CORPUS_MATCHES_FRAMING_NOTE: &str =
    "(other or past incidents - context only, not the signal this digest reports)";

/// The cue-summary text a digest's ATTENTION CUES line carries.
#[doc(hidden)]
pub fn cue_summary(c: &AttentionCue) -> String {
    c.scope_id
        .as_deref()
        .map(|s| format!("{} scope_id={s}", cue_kind_label(c.kind)))
        .unwrap_or_else(|| cue_kind_label(c.kind).to_string())
}

/// Renders a digest's `payload_summary` text. Public only so a dev tool can
/// render synthetic digests through the real code; not a stable API.
#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub fn render_payload(
    window: Duration,
    mode_label: &str,
    project: &DigestProjectContext,
    services: &[DigestServiceRow],
    cues: &[DigestCueRef],
    corpus_matches: &[String],
    incident_refs: &[String],
    active_incident_bypass: bool,
) -> String {
    let mut s = String::with_capacity(1024);
    s.push_str(&format!(
        "WINDOW: {}s, {} cadence\n",
        window.as_secs(),
        mode_label
    ));
    let name = project
        .project_name
        .clone()
        .unwrap_or_else(|| "(unknown project)".to_string());
    let vcs = project.vcs_type.unwrap_or("(no vcs)");
    s.push_str(&format!("PROJECT: {name} (vcs={vcs})\n"));
    if !project.recent_commits.is_empty() {
        s.push_str("RECENT CHANGES:\n");
        // Last 5 commits per capability spec P-032 §Project Context Grounding.
        for c in project.recent_commits.iter().take(5) {
            s.push_str(&format!(
                "  {}m ago | {} | {} files\n",
                c.age_seconds / 60,
                c.basename,
                c.files_changed_count
            ));
        }
    }
    // A Tier1 digest never carries the active-incident bypass, so keying the
    // state word on the bypass alone told the model "nominal" during every storm.
    let overall = if active_incident_bypass {
        "degraded"
    } else if !cues.is_empty() {
        "anomalous"
    } else {
        "nominal"
    };
    s.push_str(&format!(
        "OVERALL: {overall} ({} active incident(s); {} cue(s))\n",
        incident_refs.len(),
        cues.len()
    ));
    // The first cue is the one the producer takes the incident identity from.
    if let Some(trigger) = cues.first() {
        s.push_str(&format!(
            "{TRIGGER_LINE_PREFIX}{}\n",
            cue_cause_label(trigger.kind)
        ));
    }
    if !services.is_empty() {
        s.push_str("SERVICES (rate, error%, p99 vs baselines):\n");
        for row in services {
            s.push_str(&format!(
                "  {}     {:.1}/s | {:.1}% | {:.0}ms\n",
                row.service,
                row.rate_per_sec,
                row.error_rate * 100.0,
                row.p99_latency_ms
            ));
        }
    }
    if !cues.is_empty() {
        s.push_str("ATTENTION CUES:\n");
        for c in cues {
            s.push_str(&format!(
                "  [{}] {} — {}\n",
                priority_tier_label(c.priority_tier),
                cue_kind_label(c.kind),
                c.summary
            ));
        }
    }
    if !corpus_matches.is_empty() {
        s.push_str("CORPUS MATCHES:\n");
        s.push_str(&format!("  {CORPUS_MATCHES_FRAMING_NOTE}\n"));
        for m in corpus_matches.iter().take(DIGEST_CORPUS_RETRIEVAL_LIMIT) {
            s.push_str(&format!("  - {m}\n"));
        }
    }
    s
}

fn lowest_priority_cue_index(cues: &[DigestCueRef]) -> usize {
    cues.iter()
        .enumerate()
        .min_by_key(|(_, c)| priority_tier_rank(c.priority_tier))
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn priority_tier_rank(t: PriorityTier) -> u8 {
    match t {
        PriorityTier::Curious => 0,
        PriorityTier::Suggested => 1,
        PriorityTier::Autonomous => 2,
    }
}

fn lowest_anomaly_service_index(rows: &[DigestServiceRow]) -> usize {
    rows.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let a_score = a.error_rate + (a.p99_latency_ms / 1000.0);
            let b_score = b.error_rate + (b.p99_latency_ms / 1000.0);
            a_score
                .partial_cmp(&b_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{
        CueScope, EvidenceRefs, Incident, IncidentStatus, Q1RedRow, Q2OperationRow,
        Q3FingerprintRow, Q4InteractionRow, Q5CardinalityRow, Q6LogRow, Q7CriticalPathRow,
    };
    use crate::digest::retrieval::{NoopCorpusIncidentSource, RetrievalError, RetrievalFuture};
    use crate::incident::InMemoryIncidentRegistry;

    type SqlFuture<'a, T> =
        Pin<Box<dyn Future<Output = Result<T, SqlAggregationError>> + Send + 'a>>;

    struct CannedSqlRunner {
        q1: Vec<Q1RedRow>,
        q3: Vec<Q3FingerprintRow>,
    }

    impl SqlQueryRunner for CannedSqlRunner {
        fn run_q1<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q1RedRow>> {
            let rows = self.q1.clone();
            Box::pin(async move { Ok(rows) })
        }
        fn run_q2<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q2OperationRow>> {
            Box::pin(async move { Ok(Vec::new()) })
        }
        fn run_q3<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q3FingerprintRow>> {
            let rows = self.q3.clone();
            Box::pin(async move { Ok(rows) })
        }
        fn run_q4<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q4InteractionRow>> {
            Box::pin(async move { Ok(Vec::new()) })
        }
        fn run_q5<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q5CardinalityRow>> {
            Box::pin(async move { Ok(Vec::new()) })
        }
        fn run_q6<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q6LogRow>> {
            Box::pin(async move { Ok(Vec::new()) })
        }
        fn run_q7<'a>(&'a self, _w: Duration) -> SqlFuture<'a, Vec<Q7CriticalPathRow>> {
            Box::pin(async move { Ok(Vec::new()) })
        }
    }

    struct CannedCorpusSource {
        candidates: Vec<Incident>,
        fail: bool,
    }

    impl CorpusIncidentSource for CannedCorpusSource {
        fn load_candidates<'a>(
            &'a self,
            _workspace: &'a str,
            _since_unix_nano: i64,
        ) -> RetrievalFuture<'a, Vec<Incident>> {
            let fail = self.fail;
            let candidates = self.candidates.clone();
            Box::pin(async move {
                if fail {
                    Err(RetrievalError::SourceFailed)
                } else {
                    Ok(candidates)
                }
            })
        }
    }

    fn past_incident(
        id: i64,
        fingerprint: &str,
        scope_id: Option<&str>,
        title: &str,
        opened_at: i64,
    ) -> Incident {
        Incident {
            id,
            workspace: "/ws/project".to_string(),
            fingerprint: fingerprint.to_string(),
            title: title.to_string(),
            detail: String::new(),
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: scope_id.map(str::to_string),
            status: IncidentStatus::Active,
            severity: Severity::Info,
            priority_tier: PriorityTier::Curious,
            evidence_refs: EvidenceRefs {
                trace_id: None,
                span_ids: vec![],
                fingerprint_hashes: vec![],
                timestamps_unix_nano: vec![],
            },
            opened_at_unix_nano: opened_at,
            updated_at_unix_nano: opened_at,
            acknowledged_at_unix_nano: None,
            resolved_at_unix_nano: None,
            read_at_unix_nano: None,
            resolution_summary_text: None,
        }
    }

    fn q1_row(service: &str) -> Q1RedRow {
        Q1RedRow {
            service_name: service.to_string(),
            request_count: 60,
            error_count: 0,
            error_rate: 0.0,
            p50_ns: 1_000_000,
            p95_ns: 2_000_000,
            p99_ns: 3_000_000,
        }
    }

    fn make_assembler(
        q1: Vec<Q1RedRow>,
        q3: Vec<Q3FingerprintRow>,
        scrub: Arc<dyn Fn(&str) -> String + Send + Sync>,
        source: Arc<dyn CorpusIncidentSource>,
    ) -> Assembler {
        let tokenizer =
            Arc::new(Tokenizer::from_bytes(TOKENIZER_BYTES).expect("embedded tokenizer fixture"));
        Assembler::with_tokenizer(
            tokenizer,
            Arc::new(CannedSqlRunner { q1, q3 }),
            Arc::new(InMemoryIncidentRegistry::new()),
            Arc::new(DigestBroadcast::new()),
            Arc::new(Mutex::new(LwwQueue::new())),
            scrub,
            source,
        )
    }

    fn project_context() -> DigestProjectContext {
        DigestProjectContext {
            workspace_canonical_path: "/ws/project".to_string(),
            project_name: Some("project".to_string()),
            vcs_type: Some("git"),
            recent_commits: Vec::new(),
            framework_signals: Vec::new(),
        }
    }

    const NOW: i64 = 1_700_000_000_000_000_000;

    #[tokio::test]
    async fn assemble_populates_corpus_matches_from_fingerprint_match() {
        let fp_bytes = vec![0xAAu8; 16];
        let fp_hex = "aa".repeat(16);
        let q3 = vec![Q3FingerprintRow {
            fingerprint: fp_bytes,
            occurrences: 3,
            first_seen: NOW - 1_000,
            last_seen: NOW,
        }];
        let candidate = past_incident(1, &fp_hex, None, "[redacted] prior incident", NOW - 1_000);
        let assembler = make_assembler(
            vec![],
            q3,
            Arc::new(|s: &str| s.to_string()),
            Arc::new(CannedCorpusSource {
                candidates: vec![candidate],
                fail: false,
            }),
        );
        let digest = assembler
            .assemble(
                CadenceMode::Tier2,
                None,
                &project_context(),
                NOW,
                Duration::from_secs(60),
            )
            .await
            .expect("assemble succeeds");
        assert_eq!(digest.corpus_matches.len(), 1);
        assert!(digest.corpus_matches[0].contains(&fp_hex));
        assert!(digest.corpus_matches[0].contains("prior incident"));
        assert!(digest.payload_summary.contains("CORPUS MATCHES:"));
    }

    #[tokio::test]
    async fn assemble_matches_on_q1_service_scope_when_fingerprints_differ() {
        let q1 = vec![q1_row("svc-api")];
        let candidate = past_incident(
            2,
            "ffff0000ffff0000ffff0000ffff0000",
            Some("svc-api"),
            "[redacted] scoped incident",
            NOW - 2_000,
        );
        let assembler = make_assembler(
            q1,
            vec![],
            Arc::new(|s: &str| s.to_string()),
            Arc::new(CannedCorpusSource {
                candidates: vec![candidate],
                fail: false,
            }),
        );
        let digest = assembler
            .assemble(
                CadenceMode::Tier2,
                None,
                &project_context(),
                NOW,
                Duration::from_secs(60),
            )
            .await
            .expect("assemble succeeds");
        assert_eq!(digest.corpus_matches.len(), 1);
        assert!(digest.corpus_matches[0].contains("scoped incident"));
    }

    fn service_cue(scope_id: Option<&str>) -> AttentionCue {
        AttentionCue {
            kind: CueKind::RetryStorm,
            scope: CueScope::Service,
            scope_id: scope_id.map(str::to_string),
            magnitude: 20.0,
            absolute_value: 1.0,
            persistence: 30,
            confidence: 0.95,
            priority_tier: PriorityTier::Autonomous,
            suppression_bypassed: false,
            fingerprint: None,
        }
    }

    /// The titles of a digest's corpus lines over two services' incidents, a
    /// sibling's newest: under the given triggering cue, Tier1.
    async fn corpus_titles_under(cue: Option<&AttentionCue>) -> Vec<&'static str> {
        const TITLES: [&str; 3] = ["sibling newest", "own earlier", "sibling oldest"];
        let fp = "ffff0000ffff0000ffff0000ffff0000";
        let assembler = make_assembler(
            vec![q1_row("svc-api"), q1_row("svc-api-canary")],
            vec![],
            Arc::new(|s: &str| s.to_string()),
            Arc::new(CannedCorpusSource {
                candidates: vec![
                    past_incident(1, fp, Some("svc-api-canary"), TITLES[0], NOW - 1_000),
                    past_incident(2, fp, Some("svc-api"), TITLES[1], NOW - 2_000),
                    past_incident(3, fp, Some("svc-api-canary"), TITLES[2], NOW - 3_000),
                ],
                fail: false,
            }),
        );
        let digest = assembler
            .assemble(
                CadenceMode::Tier1,
                cue,
                &project_context(),
                NOW,
                Duration::from_secs(60),
            )
            .await
            .expect("assemble succeeds");
        digest
            .corpus_matches
            .iter()
            .map(|line| {
                TITLES
                    .into_iter()
                    .find(|title| line.contains(title))
                    .expect("each line carries one of the three titles")
            })
            .collect()
    }

    #[tokio::test]
    async fn assemble_drops_other_scopes_corpus_lines_under_a_cue_scope() {
        let cue = service_cue(Some("svc-api"));
        assert_eq!(corpus_titles_under(Some(&cue)).await, ["own earlier"]);
    }

    #[tokio::test]
    async fn assemble_keeps_every_scopes_corpus_lines_without_a_cue_scope() {
        let every_scope = ["sibling newest", "own earlier", "sibling oldest"];
        assert_eq!(corpus_titles_under(None).await, every_scope);
        let unscoped = service_cue(None);
        assert_eq!(corpus_titles_under(Some(&unscoped)).await, every_scope);
    }

    #[tokio::test]
    async fn assemble_degrades_to_empty_matches_on_retrieval_error() {
        let assembler = make_assembler(
            vec![],
            vec![],
            Arc::new(|s: &str| s.to_string()),
            Arc::new(CannedCorpusSource {
                candidates: vec![],
                fail: true,
            }),
        );
        let digest = assembler
            .assemble(
                CadenceMode::Tier2,
                None,
                &project_context(),
                NOW,
                Duration::from_secs(60),
            )
            .await
            .expect("assemble still succeeds on retrieval failure");
        assert!(digest.corpus_matches.is_empty());
        assert!(!digest.payload_summary.contains("CORPUS MATCHES:"));
    }

    #[tokio::test]
    async fn assemble_with_noop_source_yields_empty_matches() {
        let assembler = make_assembler(
            vec![],
            vec![],
            Arc::new(|s: &str| s.to_string()),
            Arc::new(NoopCorpusIncidentSource),
        );
        let digest = assembler
            .assemble(
                CadenceMode::Tier1,
                None,
                &project_context(),
                NOW,
                Duration::from_secs(60),
            )
            .await
            .expect("assemble succeeds");
        assert!(digest.corpus_matches.is_empty());
    }

    #[tokio::test]
    async fn assemble_routes_corpus_match_lines_through_scrub_closure() {
        let fp_hex = "bb".repeat(16);
        let q3 = vec![Q3FingerprintRow {
            fingerprint: vec![0xBBu8; 16],
            occurrences: 1,
            first_seen: NOW - 1_000,
            last_seen: NOW,
        }];
        let candidate = past_incident(3, &fp_hex, None, "title-with-CANARY-token", NOW - 1_000);
        let assembler = make_assembler(
            vec![],
            q3,
            Arc::new(|s: &str| s.replace("CANARY", "[scrubbed]")),
            Arc::new(CannedCorpusSource {
                candidates: vec![candidate],
                fail: false,
            }),
        );
        let digest = assembler
            .assemble(
                CadenceMode::Tier2,
                None,
                &project_context(),
                NOW,
                Duration::from_secs(60),
            )
            .await
            .expect("assemble succeeds");
        assert_eq!(digest.corpus_matches.len(), 1);
        assert!(digest.corpus_matches[0].contains("[scrubbed]"));
        assert!(!digest.corpus_matches[0].contains("CANARY"));
    }

    #[test]
    fn render_payload_includes_five_recent_commits_per_p032() {
        let mut ctx = project_context();
        ctx.recent_commits = (1..=6)
            .map(|i| DigestRecentCommit {
                basename: format!("commit-{i}"),
                age_seconds: i * 60,
                files_changed_count: 1,
            })
            .collect();
        let rendered = render_payload(
            Duration::from_secs(60),
            "tier2",
            &ctx,
            &[],
            &[],
            &[],
            &[],
            false,
        );
        for i in 1..=5 {
            assert!(
                rendered.contains(&format!("commit-{i}")),
                "commit-{i} must render (spec P-032 last-5)"
            );
        }
        assert!(
            !rendered.contains("commit-6"),
            "render caps at 5 commits per spec P-032"
        );
    }

    fn storm_cue_ref() -> DigestCueRef {
        DigestCueRef {
            kind: CueKind::RetryStorm,
            priority_tier: PriorityTier::Autonomous,
            summary: "retry_storm scope_id=svc".to_string(),
            scope: CueScope::Service,
            fingerprint: None,
            scope_id: Some("svc".to_string()),
        }
    }

    fn overall_line_of(rendered: &str) -> &str {
        rendered
            .lines()
            .find(|l| l.starts_with("OVERALL: "))
            .expect("the render carries an OVERALL line")
    }

    #[test]
    fn overall_line_reads_anomalous_for_a_cue_bearing_tier1_digest() {
        let rendered = render_payload(
            Duration::from_secs(60),
            "tier1",
            &project_context(),
            &[],
            &[storm_cue_ref()],
            &[],
            &[],
            false,
        );
        assert_eq!(
            overall_line_of(&rendered),
            "OVERALL: anomalous (0 active incident(s); 1 cue(s))"
        );
    }

    #[test]
    fn overall_line_reads_nominal_without_cue_or_incident() {
        let rendered = render_payload(
            Duration::from_secs(60),
            "tier3",
            &project_context(),
            &[],
            &[],
            &[],
            &[],
            false,
        );
        assert_eq!(
            overall_line_of(&rendered),
            "OVERALL: nominal (0 active incident(s); 0 cue(s))"
        );
    }

    #[test]
    fn overall_line_reads_degraded_with_an_active_incident() {
        let rendered = render_payload(
            Duration::from_secs(60),
            "tier2",
            &project_context(),
            &[],
            &[storm_cue_ref()],
            &[],
            &["7".to_string()],
            true,
        );
        assert_eq!(
            overall_line_of(&rendered),
            "OVERALL: degraded (1 active incident(s); 1 cue(s))"
        );
    }

    fn cue_ref_of(kind: CueKind) -> DigestCueRef {
        DigestCueRef {
            kind,
            priority_tier: PriorityTier::Autonomous,
            summary: format!("{} scope_id=svc", cue_kind_label(kind)),
            scope: CueScope::Service,
            fingerprint: None,
            scope_id: Some("svc".to_string()),
        }
    }

    fn render_with(cues: &[DigestCueRef], corpus_matches: &[String]) -> String {
        render_payload(
            Duration::from_secs(60),
            "tier1",
            &project_context(),
            &[],
            cues,
            corpus_matches,
            &[],
            false,
        )
    }

    #[test]
    fn render_payload_names_the_trigger_from_the_first_cue() {
        let rendered = render_with(
            &[
                cue_ref_of(CueKind::RetryStorm),
                cue_ref_of(CueKind::ErrorRateSpike),
            ],
            &[],
        );
        let lines: Vec<&str> = rendered.lines().collect();
        let trigger_lines: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.starts_with(TRIGGER_LINE_PREFIX))
            .map(|(i, _)| i)
            .collect();
        assert_eq!(trigger_lines.len(), 1, "exactly one TRIGGER line");
        let at = trigger_lines[0];
        assert_eq!(lines[at], "TRIGGER: Retry storm");
        assert!(
            at > 0 && lines[at - 1].starts_with("OVERALL: "),
            "the TRIGGER line directly follows OVERALL"
        );
    }

    #[test]
    fn render_payload_omits_the_trigger_line_without_a_cue() {
        let rendered = render_with(&[], &[]);
        assert!(
            !rendered.lines().any(|l| l.starts_with(TRIGGER_LINE_PREFIX)),
            "a cue-less digest names no trigger"
        );
    }

    #[test]
    fn render_payload_frames_corpus_matches_as_other_incidents() {
        let rendered = render_with(
            &[cue_ref_of(CueKind::RetryStorm)],
            &["[abcd] Error-rate spike: past incident - 3m ago, active".to_string()],
        );
        let lines: Vec<&str> = rendered.lines().collect();
        let header = lines
            .iter()
            .position(|l| *l == "CORPUS MATCHES:")
            .expect("the corpus header renders");
        assert_eq!(
            lines.get(header + 1).copied(),
            Some(format!("  {CORPUS_MATCHES_FRAMING_NOTE}").as_str()),
            "the framing note directly follows the header"
        );
        assert_eq!(
            lines.get(header + 2).copied(),
            Some("  - [abcd] Error-rate spike: past incident - 3m ago, active"),
            "the match line follows the note"
        );
    }

    #[test]
    fn render_payload_omits_the_corpus_framing_note_without_matches() {
        let rendered = render_with(&[cue_ref_of(CueKind::RetryStorm)], &[]);
        assert!(!rendered.contains(CORPUS_MATCHES_FRAMING_NOTE));
    }

    #[test]
    fn render_payload_framing_lines_are_ascii() {
        for kind in [
            CueKind::ErrorRateSpike,
            CueKind::LatencyRegression,
            CueKind::RestartEvent,
            CueKind::ServiceWentSilent,
            CueKind::RetryStorm,
            CueKind::ReflectionTrend,
        ] {
            let rendered = render_with(&[cue_ref_of(kind)], &["m".to_string()]);
            let trigger = rendered
                .lines()
                .find(|l| l.starts_with(TRIGGER_LINE_PREFIX))
                .unwrap_or_else(|| panic!("{kind:?}: the render carries a TRIGGER line"));
            assert!(trigger.is_ascii(), "{kind:?}: TRIGGER line is ASCII");
            let note = rendered
                .lines()
                .find(|l| l.trim_start() == CORPUS_MATCHES_FRAMING_NOTE)
                .unwrap_or_else(|| panic!("{kind:?}: the render carries the framing note"));
            assert!(note.is_ascii(), "{kind:?}: framing note is ASCII");
        }
    }

    #[test]
    fn hex_lower_encodes_q3_fingerprint_bytes() {
        assert_eq!(hex_lower(&[0xAA, 0x0F, 0x00]), "aa0f00");
    }
}
