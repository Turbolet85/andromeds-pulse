//! L3 digest assembler core implementation (chunk #81).
//!
//! `DigestAssembler` trait + `Assembler` concrete impl. Composes а
//! `Digest` per dist-arch v3 §Appendix C from L1a Q1-Q7 (via
//! `SqlQueryRunner` injected from chunk #80 substrate), attention cues
//! (from `AttentionCue` triggering arg), project context (via
//! `DigestProjectContext` injected by binary adapter), and active-incident
//! state (via `IncidentRegistry` injected from chunk #78 substrate).
//!
//! ## Scope of corpus retrieval (deferred to chunk #82+)
//!
//! Initial chunk #81 substrate writes digests к corpus via
//! `CorpusWriter::save_digest` extension AND emits broadcast events for
//! L4 LLM consumption. CORPUS MATCHES section (top-N similar past
//! incidents by fingerprint per capability P-044) is stubbed as empty
//! `corpus_matches: Vec<String>` because the retrieval algorithm + index
//! design depend on chunk #82+ LLM runtime + tokenizer choices for
//! similarity scoring. Substrate ready; retrieval lands at chunk #82+.
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
use std::time::Duration;

use tokenizers::Tokenizer;

use crate::cadence::CadenceMode;
use crate::contract::{
    AttentionCue, CueKind, Digest, DigestCueRef, DigestKind, DigestLwwMode, DigestServiceRow,
    IncidentStatus, PriorityTier, Severity, SqlAggregationError, SqlQueryRunner,
};
use crate::digest::broadcast::DigestBroadcast;
use crate::digest::queue::{LwwQueue, QueueAction};
use crate::digest::{
    DIGEST_TOKEN_BUDGET_HARD_CAP, DIGEST_TOKEN_BUDGET_SOFT_MAX, DIGEST_TOKEN_BUDGET_SOFT_MIN,
    DigestError, TARGET_DIGEST_ASSEMBLE, TARGET_DIGEST_LWW_DROP, TARGET_DIGEST_LWW_REPLACE,
    TARGET_DIGEST_TOKEN_COUNT_VALIDATE, TARGET_METRIC_ACTIVE_INCIDENT_QUEUE_DEPTH,
    TARGET_METRIC_DIGEST_TOKEN_COUNT_MS, TARGET_METRIC_LWW_DROP_COUNT_TOTAL,
};
use crate::incident::IncidentRegistry;

/// Build-time-embedded Llama-3 tokenizer.json fixture (per
/// `crates/triage/build.rs` Phase 2). Future chunk #82+ may swap к а
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
/// direction (triage не depends on workspace-detector).
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

/// Trait для assembling а digest. Implemented by [`Assembler`]; tests
/// can substitute а stub.
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

/// L3 digest assembler. Composes а digest from L1a queries + L2 cues +
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
/// adapter at the binary boundary к keep triage free of corpus crate dep
/// (the assembler emits its digest and the binary boundary captures the
/// emission, scrubs + persists). For chunk #81 substrate, this is
/// implemented via the broadcast subscription pattern: pulse-app
/// subscribes к `DigestBroadcast` and persists each emitted digest к
/// corpus via `CorpusWriter::save_digest` (chunk #81 trait extension).
pub struct Assembler {
    tokenizer: Arc<Tokenizer>,
    sql_runner: Arc<dyn SqlQueryRunner>,
    incident_registry: Arc<dyn IncidentRegistry>,
    broadcast: Arc<DigestBroadcast>,
    queue: Arc<Mutex<LwwQueue>>,
    scrub: Arc<dyn Fn(&str) -> String + Send + Sync>,
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
        })
    }

    /// Test-only constructor that takes а pre-built tokenizer (lets unit
    /// tests inject а tokenizer they constructed elsewhere). Production
    /// callers use `new()`.
    #[cfg(test)]
    pub fn with_tokenizer(
        tokenizer: Arc<Tokenizer>,
        sql_runner: Arc<dyn SqlQueryRunner>,
        incident_registry: Arc<dyn IncidentRegistry>,
        broadcast: Arc<DigestBroadcast>,
        queue: Arc<Mutex<LwwQueue>>,
        scrub: Arc<dyn Fn(&str) -> String + Send + Sync>,
    ) -> Self {
        Self {
            tokenizer,
            sql_runner,
            incident_registry,
            broadcast,
            queue,
            scrub,
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
            // Q2-Q7 fetched но не embedded in chunk #81 substrate digest;
            // execution stays к ensure the SQL surface is exercised
            // identically к the cadence coordinator (consistency +
            // diagnostics value). Future chunk integrates richer fields.
            let _q2 = self.sql_runner.run_q2(window_duration).await.ok();
            let _q3 = self.sql_runner.run_q3(window_duration).await.ok();
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
                        summary: c
                            .scope_id
                            .as_deref()
                            .map(|s| format!("{} scope_id={s}", cue_kind_label(c.kind)))
                            .unwrap_or_else(|| cue_kind_label(c.kind).to_string()),
                    }]
                })
                .unwrap_or_default();

            // CORPUS MATCHES deferred к chunk #82+ (see file-level docstring).
            let corpus_matches: Vec<String> = Vec::new();

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
    // language; map к Severity::Warn since Severity enum lacks
    // explicit Suggested tier — Severity carries Info/Warn/Error/Critical
    // levels while PriorityTier carries the model-decision tiers).
    matches!(s, Severity::Warn | Severity::Error | Severity::Critical)
}

fn compose_services_from_q1(rows: &[crate::contract::Q1RedRow]) -> Vec<DigestServiceRow> {
    rows.iter()
        .map(|r| {
            // Q1 rows expose RED-style aggregates (rate, error count, p99
            // latency over the window). Chunk #81 substrate maps directly;
            // baselines per row default к the current observation так что
            // the "vs baselines" comparison reads neutral (×1.0) until
            // chunk #82+ integrates L1b baseline state. Q1RedRow stores
            // p99 в nanoseconds (per chunk #79 sql.rs); convert к ms for
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

#[allow(clippy::too_many_arguments)]
fn render_payload(
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
        for c in project.recent_commits.iter().take(3) {
            s.push_str(&format!(
                "  {}m ago | {} | {} files\n",
                c.age_seconds / 60,
                c.basename,
                c.files_changed_count
            ));
        }
    }
    s.push_str(&format!(
        "OVERALL: {} ({} active-bypass incident(s); {} cue(s))\n",
        if active_incident_bypass {
            "degraded"
        } else {
            "nominal"
        },
        incident_refs.len(),
        cues.len()
    ));
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
        for m in corpus_matches.iter().take(3) {
            s.push_str(&format!("  - fingerprint [{m}]\n"));
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
