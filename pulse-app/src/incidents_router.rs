//! Incidents TauRPC router — chunk #78.
//!
//! `incidents.list_active()` / `incidents.acknowledge(id)` /
//! `incidents.mark_resolved(id)` resolvers backed by `IncidentRegistry`
//! (in-memory live state) + `IncidentPersistence` (corpus durability) +
//! `IncidentLifecycleBroadcast` (`pulse://stream/incidents` emission).
//! Mirrors `services_router.rs` shape: library crate (`crates/triage`)
//! stays Tauri-free; the Tauri-aware router lives here at the binary
//! boundary so taurpc + specta deps don't leak into the workspace crate
//! per arch §Cross-cutting Patterns Module dependency direction.

use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use interpretation::markdown::{
    HypothesisView, InvestigationStepView, Report as MarkdownReport, serialize_report,
};
use interpretation::schema::{Confidence as L4Confidence, L4Output};
use security::scrubber::{ScrubbedValue, scrub_attribute};
use serde::{Deserialize, Serialize};
use triage::contract::{
    CueKind, CueScope, DEFAULT_INCIDENT_ACK_COOLDOWN_SECS, Incident, IncidentLifecycleBroadcast,
    IncidentLifecycleEvent, IncidentPersistence, IncidentRegistry, IncidentRegistryError,
    IncidentStatus, PriorityTier, ResolutionTrigger, Severity,
};
use ui_bridge::contract::AppError;

/// Resolver-facing view of an Incident. Narrower than the full `triage::
/// contract::Incident` struct — drops raw `evidence_refs` byte arrays
/// (downstream UI displays the `evidence_count` instead) AND drops the
/// fingerprint string (internal grouping identifier; not user-relevant).
/// Webview consumers receive this shape over the TauRPC bridge.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct IncidentRecord {
    pub id: i64,
    pub workspace: String,
    pub kind: CueKind,
    pub scope: CueScope,
    pub status: IncidentStatus,
    pub severity: Severity,
    pub priority_tier: PriorityTier,
    pub title: String,
    pub detail: String,
    pub opened_at_unix_nano: i64,
    pub updated_at_unix_nano: i64,
    pub acknowledged_at_unix_nano: Option<i64>,
    pub resolved_at_unix_nano: Option<i64>,
    pub read_at_unix_nano: Option<i64>,
    pub evidence_count: usize,
}

impl From<&Incident> for IncidentRecord {
    fn from(i: &Incident) -> Self {
        Self {
            id: i.id,
            workspace: i.workspace.clone(),
            kind: i.kind,
            scope: i.scope,
            status: i.status,
            severity: i.severity,
            priority_tier: i.priority_tier,
            title: i.title.clone(),
            detail: i.detail.clone(),
            opened_at_unix_nano: i.opened_at_unix_nano,
            updated_at_unix_nano: i.updated_at_unix_nano,
            acknowledged_at_unix_nano: i.acknowledged_at_unix_nano,
            resolved_at_unix_nano: i.resolved_at_unix_nano,
            read_at_unix_nano: i.read_at_unix_nano,
            evidence_count: i.evidence_refs.span_ids.len()
                + i.evidence_refs.fingerprint_hashes.len(),
        }
    }
}

/// Paginated list envelope per arch §Standard Contracts. `next_cursor`
/// reserved для future pagination wire-up; chunk #78 returns the full
/// active-incident set in one response (bounded by 120s auto-resolution
/// + 10-min recent-history display window).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct IncidentsListPayload {
    pub items: Vec<IncidentRecord>,
    pub total: usize,
    pub next_cursor: Option<String>,
}

/// Single ranked hypothesis. Mirrors `interpretation::markdown::HypothesisView`
/// but с specta::Type derive for cross-bridge transport (interpretation
/// crate has no taurpc/specta dep per arch §Module dependency direction).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct HypothesisPayload {
    pub statement: String,
    pub confidence_label: String,
    pub justification: String,
}

/// Single suggested investigation step с expected yield description.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct InvestigationStepPayload {
    pub step: String,
    pub expected_yield: String,
}

/// Cross-incident "Previously seen" match per P-036. Chunk #88 reserves
/// the field shape; current resolver always returns empty Vec (corpus
/// fingerprint-similarity query path is а follow-up chunk).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct PreviouslySeenPayload {
    pub incident_id: i64,
    pub opened_at_unix_nano: i64,
    pub title: String,
    pub workspace: String,
}

/// Six-section Report payload returned by `incidents.get_report(id)`
/// (chunk #88 — Epoch 9 Foundation v0.2.0). Carries structured fields
/// for the in-app webview surface AND а pre-serialized `markdown` string
/// for the Copy markdown action (P-038 byte-identical к future MCP
/// delivery #92 per project doc §87 contract).
///
/// Hybrid render contract (chunk #88 Phase 1 user-approved scope):
/// - `degraded_mode = false` indicates Resolved incident с parsed L4Output
///   payload — full six-section content.
/// - `degraded_mode = true` indicates Active/Acknowledged incident OR
///   Resolved incident с unparseable / redacted resolution_summary_text —
///   hypotheses + investigation_steps replaced by explicit "interpretation
///   pending" notice in the markdown OR webview.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct ReportPayload {
    pub incident_id: i64,
    pub title: String,
    pub workspace: String,
    pub opened_at_unix_nano: i64,
    pub status: IncidentStatus,
    pub severity: Severity,
    pub symptom: String,
    pub timeline: String,
    pub hypotheses: Vec<HypothesisPayload>,
    pub investigation_steps: Vec<InvestigationStepPayload>,
    pub evidence_refs: Vec<String>,
    pub project_context: String,
    pub degraded_mode: bool,
    pub resolution_summary: Option<String>,
    pub previously_seen: Vec<PreviouslySeenPayload>,
    pub markdown: String,
}

// Chunk #87 — `mark_all_read` is the Findings counter dropdown footer's
// bulk action. Sets `read_at_unix_nano` on every Active / Acknowledged
// incident in the current workspace that is currently unread; returns
// affected_count. Persist failures inside the iteration are non-fatal
// (warn + continue per chunk #78 `acknowledge` precedent). Single
// aggregate `incident.broadcast.bulk_acknowledged` tracing event emits
// per invocation regardless of affected_count (cardinality discipline
// per CLAUDE.md observability 2026-05-17 session 84). NO broadcast
// channel event emit — read-state changes propagate via pull-on-focus
// per project-doc §86 "no separate state file"; future webview live
// updates can land с а dedicated `streams.subscribe_incidents` chunk.
//
// Chunk #88 — `get_report` is the L5 Diagnostic Report surface. Fetches
// Incident from registry; if Resolved + resolution_summary_text parses
// as L4Output JSON, renders full six-section markdown с hypotheses +
// investigation_steps from the LLM output. Otherwise renders с degraded-
// mode notice in those two sections (Active/Acknowledged incidents OR
// Resolved incidents с unparseable summary text). Project doc §87
// capabilities P-031 + P-035–P-038. Markdown output is byte-identical
// к the future MCP delivery (#92) per P-038 single-source-of-truth.
#[taurpc::procedures(path = "incidents")]
pub trait IncidentsApi {
    async fn list_active() -> Result<IncidentsListPayload, AppError>;
    async fn acknowledge(id: i64) -> Result<(), AppError>;
    async fn mark_resolved(id: i64) -> Result<(), AppError>;
    async fn mark_all_read() -> Result<MarkAllReadPayload, AppError>;
    async fn get_report(id: i64) -> Result<ReportPayload, AppError>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, specta::Type)]
pub struct MarkAllReadPayload {
    pub affected_count: usize,
    pub marked_at_unix_nano: i64,
}

#[derive(Clone)]
pub struct IncidentsApiImpl {
    registry: Arc<dyn IncidentRegistry>,
    broadcast: Arc<IncidentLifecycleBroadcast>,
    persistence: Arc<dyn IncidentPersistence>,
    workspace_root: String,
    cooldown_secs: u64,
}

impl IncidentsApiImpl {
    pub fn new(
        registry: Arc<dyn IncidentRegistry>,
        broadcast: Arc<IncidentLifecycleBroadcast>,
        persistence: Arc<dyn IncidentPersistence>,
        workspace_root: String,
    ) -> Self {
        Self {
            registry,
            broadcast,
            persistence,
            workspace_root,
            cooldown_secs: DEFAULT_INCIDENT_ACK_COOLDOWN_SECS,
        }
    }
}

#[taurpc::resolvers]
impl IncidentsApi for IncidentsApiImpl {
    #[tracing::instrument(skip_all, fields(item_count = tracing::field::Empty))]
    async fn list_active(self) -> Result<IncidentsListPayload, AppError> {
        let items_raw = self.registry.list_active(&self.workspace_root);
        let items: Vec<IncidentRecord> = items_raw.iter().map(IncidentRecord::from).collect();
        let total = items.len();
        let payload = IncidentsListPayload {
            items,
            total,
            next_cursor: None,
        };
        let span = tracing::Span::current();
        span.record("item_count", total as u64);
        tracing::info!(
            target: "incidents.list_active.request",
            item_count = total as u64,
            "incidents.list_active returned",
        );
        Ok(payload)
    }

    #[tracing::instrument(skip_all, fields(outcome = tracing::field::Empty))]
    async fn acknowledge(self, id: i64) -> Result<(), AppError> {
        let now = current_unix_nanos();
        let result = self.registry.acknowledge(id, now, self.cooldown_secs);
        match result {
            Ok(updated) => {
                let span = tracing::Span::current();
                span.record("outcome", "acknowledged");
                if let Err(err) = self.persistence.update_incident_status(id, &updated) {
                    tracing::warn!(
                        target: "triage.incident.persist.error",
                        error_category = err.error_category(),
                        persist_kind = "incident_acknowledge",
                        "incident persist on acknowledge failed",
                    );
                }
                let _ = self.broadcast.sender().send(IncidentLifecycleEvent {
                    incident_id: id,
                    kind: updated.kind,
                    scope: updated.scope,
                    from_state: IncidentStatus::Active,
                    to_state: IncidentStatus::Acknowledged,
                    transitioned_at_unix_nano: now,
                });
                tracing::info!(
                    target: "incidents.acknowledge.request",
                    outcome = "acknowledged",
                    "incidents.acknowledge succeeded",
                );
                Ok(())
            }
            Err(IncidentRegistryError::NotFound) => {
                tracing::Span::current().record("outcome", "not_found");
                tracing::info!(
                    target: "incidents.acknowledge.request",
                    outcome = "not_found",
                    "incidents.acknowledge returned NotFound",
                );
                Err(AppError::NotFound {
                    resource: "incident".to_string(),
                })
            }
            Err(IncidentRegistryError::CooldownActive { remaining_secs }) => {
                tracing::Span::current().record("outcome", "cooldown_rejected");
                tracing::warn!(
                    target: "incidents.acknowledge.cooldown_rejected",
                    cooldown_remaining_ms = remaining_secs * 1000,
                    "incidents.acknowledge rejected during cool-down",
                );
                Err(AppError::Validation {
                    field: "id".to_string(),
                    reason: "cool-down active".to_string(),
                })
            }
            Err(IncidentRegistryError::InvalidTransition) => {
                tracing::Span::current().record("outcome", "invalid_transition");
                Err(AppError::Validation {
                    field: "id".to_string(),
                    reason: "invalid lifecycle transition".to_string(),
                })
            }
        }
    }

    #[tracing::instrument(skip_all, fields(outcome = tracing::field::Empty))]
    async fn mark_resolved(self, id: i64) -> Result<(), AppError> {
        let now = current_unix_nanos();
        let result = self
            .registry
            .mark_resolved(id, now, ResolutionTrigger::ExplicitResolve);
        match result {
            Ok(updated) => {
                let span = tracing::Span::current();
                span.record("outcome", "resolved");
                let from_state = if updated.acknowledged_at_unix_nano.is_some() {
                    IncidentStatus::Acknowledged
                } else {
                    IncidentStatus::Active
                };
                if let Err(err) = self.persistence.update_incident_status(id, &updated) {
                    tracing::warn!(
                        target: "triage.incident.persist.error",
                        error_category = err.error_category(),
                        persist_kind = "incident_mark_resolved",
                        "incident persist on mark_resolved failed",
                    );
                }
                let _ = self.broadcast.sender().send(IncidentLifecycleEvent {
                    incident_id: id,
                    kind: updated.kind,
                    scope: updated.scope,
                    from_state,
                    to_state: IncidentStatus::Resolved,
                    transitioned_at_unix_nano: now,
                });
                tracing::info!(
                    target: "incidents.mark_resolved.request",
                    outcome = "resolved",
                    "incidents.mark_resolved succeeded",
                );
                Ok(())
            }
            Err(IncidentRegistryError::NotFound) => {
                tracing::Span::current().record("outcome", "not_found");
                tracing::info!(
                    target: "incidents.mark_resolved.request",
                    outcome = "not_found",
                    "incidents.mark_resolved returned NotFound",
                );
                Err(AppError::NotFound {
                    resource: "incident".to_string(),
                })
            }
            Err(IncidentRegistryError::CooldownActive { remaining_secs }) => {
                // Defensive — cool-down only applies к acknowledge; if reg
                // somehow rejects mark_resolved on cool-down, surface as
                // validation error rather than panic.
                tracing::Span::current().record("outcome", "cooldown_rejected");
                Err(AppError::Validation {
                    field: "id".to_string(),
                    reason: format!("cool-down active ({remaining_secs}s)"),
                })
            }
            Err(IncidentRegistryError::InvalidTransition) => {
                tracing::Span::current().record("outcome", "invalid_transition");
                Err(AppError::Validation {
                    field: "id".to_string(),
                    reason: "invalid lifecycle transition".to_string(),
                })
            }
        }
    }

    #[tracing::instrument(skip_all, fields(
        affected_count = tracing::field::Empty,
        outcome = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ))]
    async fn mark_all_read(self) -> Result<MarkAllReadPayload, AppError> {
        let start = std::time::Instant::now();
        let now = current_unix_nanos();
        let affected_ids = self
            .registry
            .mark_all_read(&self.workspace_root, now)
            .map_err(|err| AppError::Internal {
                message: format!("registry mark_all_read failed: {}", err.error_category()),
            })?;
        let affected_count = affected_ids.len();
        for id in &affected_ids {
            if let Err(err) = self.persistence.mark_read(*id, now) {
                tracing::warn!(
                    target: "triage.incident.persist.error",
                    error_category = err.error_category(),
                    persist_kind = "incident_mark_all_read",
                    "incident persist on mark_all_read failed",
                );
            }
        }
        let duration_ms: u64 = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        let span = tracing::Span::current();
        span.record("affected_count", affected_count as u64);
        span.record("outcome", "succeeded");
        span.record("duration_ms", duration_ms);

        tracing::info!(
            target: "incidents.mark_all_read.request",
            affected_count = affected_count as u64,
            outcome = "succeeded",
            duration_ms = duration_ms,
            "incidents.mark_all_read returned",
        );

        tracing::info!(
            target: "incident.broadcast.bulk_acknowledged",
            affected_count = affected_count as u64,
            "incident bulk-acknowledged",
        );

        Ok(MarkAllReadPayload {
            affected_count,
            marked_at_unix_nano: now,
        })
    }

    #[tracing::instrument(skip_all, fields(
        item_id = id,
        outcome = tracing::field::Empty,
        section_count = tracing::field::Empty,
        markdown_size_bytes = tracing::field::Empty,
        degraded_mode = tracing::field::Empty,
        previously_seen_corpus_match_count = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    ))]
    async fn get_report(self, id: i64) -> Result<ReportPayload, AppError> {
        let start = Instant::now();
        let incident = self.registry.get(id).ok_or_else(|| {
            tracing::Span::current().record("outcome", "not_found");
            tracing::info!(
                target: "incidents.get_report.request",
                item_id = id,
                outcome = "not_found",
                "incidents.get_report returned NotFound",
            );
            AppError::NotFound {
                resource: "incident".to_string(),
            }
        })?;

        let parsed_l4: Option<L4Output> = incident
            .resolution_summary_text
            .as_deref()
            .and_then(|text| serde_json::from_str::<L4Output>(text).ok());
        let degraded_mode = parsed_l4.is_none();

        if degraded_mode {
            tracing::info!(
                target: "report.degraded_mode_notice",
                incident_id = incident.id,
                reason_category = degraded_reason_category(&incident),
                "report rendered in degraded mode",
            );
        }

        let report = assemble_report(&incident, parsed_l4.as_ref());
        let markdown = serialize_report(&report);
        let markdown_size_bytes = u64::try_from(markdown.len()).unwrap_or(u64::MAX);
        let section_count: u64 = 6;
        let previously_seen_corpus_match_count =
            u64::try_from(report.previously_seen.len()).unwrap_or(u64::MAX);

        tracing::info!(
            target: "report.render.markdown",
            incident_id = incident.id,
            report_section_count = section_count,
            markdown_size_bytes = markdown_size_bytes,
            degraded_mode = degraded_mode,
            previously_seen_corpus_match_count = previously_seen_corpus_match_count,
            duration_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
            "report markdown serialized",
        );

        let payload = ReportPayload {
            incident_id: incident.id,
            title: report.title.clone(),
            workspace: report.workspace.clone(),
            opened_at_unix_nano: report.opened_at_unix_nano,
            status: incident.status,
            severity: incident.severity,
            symptom: report.symptom.clone(),
            timeline: report.timeline.clone(),
            hypotheses: report
                .hypotheses
                .iter()
                .map(|h| HypothesisPayload {
                    statement: h.statement.clone(),
                    confidence_label: h.confidence_label.clone(),
                    justification: h.justification.clone(),
                })
                .collect(),
            investigation_steps: report
                .investigation_steps
                .iter()
                .map(|s| InvestigationStepPayload {
                    step: s.step.clone(),
                    expected_yield: s.expected_yield.clone(),
                })
                .collect(),
            evidence_refs: report.evidence_refs.clone(),
            project_context: report.project_context.clone(),
            degraded_mode,
            resolution_summary: report.resolution_summary.clone(),
            previously_seen: report
                .previously_seen
                .iter()
                .map(|m| PreviouslySeenPayload {
                    incident_id: m.incident_id,
                    opened_at_unix_nano: m.opened_at_unix_nano,
                    title: m.title.clone(),
                    workspace: m.workspace.clone(),
                })
                .collect(),
            markdown,
        };

        let duration_ms: u64 = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let span = tracing::Span::current();
        span.record("outcome", "succeeded");
        span.record("section_count", section_count);
        span.record("markdown_size_bytes", markdown_size_bytes);
        span.record("degraded_mode", degraded_mode);
        span.record(
            "previously_seen_corpus_match_count",
            previously_seen_corpus_match_count,
        );
        span.record("duration_ms", duration_ms);
        tracing::info!(
            target: "incidents.get_report.request",
            item_id = id,
            outcome = "succeeded",
            section_count = section_count,
            markdown_size_bytes = markdown_size_bytes,
            degraded_mode = degraded_mode,
            previously_seen_corpus_match_count = previously_seen_corpus_match_count,
            duration_ms = duration_ms,
            "incidents.get_report returned",
        );

        tracing::info!(
            target: "metric.report.render_ms",
            value = duration_ms,
            section_count = section_count,
            degraded_mode = degraded_mode,
            "report render latency sample",
        );

        Ok(payload)
    }
}

/// Bounded reason category for the `report.degraded_mode_notice` tracing
/// event. Aggregate-only label per CLAUDE.md observability 2026-05-17 +
/// no per-incident detail leaks beyond the incident_id field.
fn degraded_reason_category(incident: &Incident) -> &'static str {
    match (incident.status, incident.resolution_summary_text.is_some()) {
        (IncidentStatus::Resolved, true) => "resolved_summary_unparseable",
        (IncidentStatus::Resolved, false) => "resolved_no_summary_attached",
        (IncidentStatus::Active, _) => "active_no_l4_output",
        (IncidentStatus::Acknowledged, _) => "acknowledged_no_l4_output",
    }
}

/// Assemble the markdown::Report struct from an Incident + optional
/// parsed L4Output. Pure projection — no I/O, no scrubbing (caller is
/// expected to ensure incident strings are pre-scrubbed per chunks #72,
/// #78, #86 uniform coverage; defense-in-depth scrub happens at field
/// projection time below).
fn assemble_report(incident: &Incident, l4: Option<&L4Output>) -> MarkdownReport {
    let workspace = scrub_string(&incident.workspace);
    let project_context = format!("workspace={workspace}");
    let status_label = incident_status_label(incident.status).to_string();
    let severity_label = severity_label(incident.severity).to_string();

    match l4 {
        Some(l4) => MarkdownReport {
            incident_id: incident.id,
            title: scrub_string(&l4.title),
            workspace,
            opened_at_unix_nano: incident.opened_at_unix_nano,
            status_label,
            severity_label,
            symptom: scrub_string(&l4.symptom),
            timeline: scrub_string(&l4.timeline),
            hypotheses: l4
                .hypotheses
                .iter()
                .map(|h| HypothesisView {
                    statement: scrub_string(&h.statement),
                    confidence_label: l4_confidence_label(h.confidence).to_string(),
                    justification: scrub_string(&h.justification),
                })
                .collect(),
            investigation_steps: l4
                .investigation_steps
                .iter()
                .map(|s| InvestigationStepView {
                    step: scrub_string(&s.step),
                    expected_yield: scrub_string(&s.expected_yield),
                })
                .collect(),
            evidence_refs: l4.evidence_refs.iter().map(|r| scrub_string(r)).collect(),
            project_context,
            degraded_mode: false,
            // When the L4Output renders the Report itself for а Resolved
            // incident, the same payload IS the resolution summary —
            // surfacing it as а duplicate section under "Resolution
            // Summary" would be redundant. Skip.
            resolution_summary: None,
            previously_seen: Vec::new(),
        },
        None => MarkdownReport {
            incident_id: incident.id,
            title: scrub_string(&incident.title),
            workspace,
            opened_at_unix_nano: incident.opened_at_unix_nano,
            status_label,
            severity_label,
            // Symptom falls back к incident.detail when no L4Output
            // available (chunk #78 producer-side scrubbed; defense-in-
            // depth scrub here is а no-op for already-scrubbed input).
            symptom: scrub_string(&incident.detail),
            timeline: String::new(),
            hypotheses: Vec::new(),
            investigation_steps: Vec::new(),
            evidence_refs: incident
                .evidence_refs
                .span_ids
                .iter()
                .map(|bytes| format!("span:{}", hex_lower(bytes)))
                .chain(
                    incident
                        .evidence_refs
                        .fingerprint_hashes
                        .iter()
                        .map(|fp| format!("fp:{}", scrub_string(fp))),
                )
                .collect(),
            project_context,
            degraded_mode: true,
            resolution_summary: None,
            previously_seen: Vec::new(),
        },
    }
}

/// Defense-in-depth scrubber application. Routes every user-facing text
/// field through `security::scrubber::scrub_attribute` BEFORE markdown
/// composition per the chunk #72 uniform-coverage invariant + plan.md
/// Step 3 step 9 scrub_report discipline. Already-scrubbed input passes
/// through verbatim (idempotent at the scrubber boundary); raw OTLP-
/// derived bytes that somehow bypassed upstream scrubbing get redacted
/// here.
fn scrub_string(text: &str) -> String {
    match scrub_attribute(text) {
        ScrubbedValue::Allowed(s) => s,
        ScrubbedValue::Redacted { category } => format!("[redacted: {category}]"),
    }
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn incident_status_label(status: IncidentStatus) -> &'static str {
    match status {
        IncidentStatus::Active => "active",
        IncidentStatus::Acknowledged => "acknowledged",
        IncidentStatus::Resolved => "resolved",
    }
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warn => "warn",
        Severity::Error => "error",
        Severity::Critical => "critical",
    }
}

fn l4_confidence_label(c: L4Confidence) -> &'static str {
    match c {
        L4Confidence::High => "high",
        L4Confidence::Medium => "medium",
        L4Confidence::Low => "low",
    }
}

fn current_unix_nanos() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

// Tests live at `pulse-app/tests/unit_incident_persistence.rs` +
// `pulse-app/tests/e2e_incidents_lifecycle.rs` per session-learnings
// 2026-05-13 — pulse-app `[lib] test = false` makes source-level
// `#[cfg(test)] mod tests` dead. Integration tests cover the resolver
// methods end-to-end through `tauri::test::mock_builder()` +
// `get_ipc_response()`.
