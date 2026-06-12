//! MCP `#[tool]` method dispatch — chunk #49.
//!
//! Translates JSON-RPC 2.0 `tools/call` requests into calls against the
//! shared library primitives (`viz::query::*` for trace/metric/log queries,
//! `snapshot::contract::{curate, format_markdown}` for snapshot rendering).
//!
//! Per arch §Cross-bridge data shape + design plan Decisions Log 2026-05-02:
//! the snapshot curation pipeline is a single domain concept — both the
//! IPC-side `snapshot.generate` resolver (chunk #44) and the MCP-side
//! `generate_snapshot` tool here reach the same `snapshot::contract::curate`
//! primitive. No duplication.
//!
//! Per security plan §Logging Vector 4: tool response bodies NEVER appear
//! in tracing fields. The dispatch span emits `tool_name` + `result_type` +
//! `result_count` + `duration_ms` only; `result_content` / params bodies
//! are excluded by `skip_all` on the instrument macro AND defended in depth
//! by the `AllowList::for_mcp_server` scrubber at the subscriber layer
//! (chunk #48 substrate, extended in tracing_setup.rs).

use std::sync::{Arc, Mutex};
use std::time::Instant;

use corpus::contract::{CorpusWriter, IncidentRowRaw};
use duckdb::Connection;
use interpretation::markdown::{
    PreviouslySeenMatch, assemble_report, previously_seen_from_incidents, serialize_report,
};
use interpretation::schema::L4Output;
use serde::Deserialize;
use serde_json::{Value, json};
use snapshot::contract::{SpanRecord, TokenBudget, curate, format_markdown};
use triage::contract::{
    CORPUS_RETRIEVAL_WINDOW_SECONDS, DIGEST_CORPUS_RETRIEVAL_LIMIT, Incident,
    select_previously_seen,
};
use viz::query::{
    LIMIT_DEFAULT, LIMIT_MAX, LogsQueryArgs, MetricsQueryArgs, TracesQueryArgs, query_logs,
    query_metrics, query_traces,
};
use viz::state::VizState;

use crate::contract::Error;

pub const TOOL_QUERY_TRACES: &str = "query_traces";
pub const TOOL_QUERY_METRICS: &str = "query_metrics";
pub const TOOL_QUERY_LOGS: &str = "query_logs";
pub const TOOL_GENERATE_SNAPSHOT: &str = "generate_snapshot";
pub const TOOL_QUERY_INCIDENT_LIST: &str = "query_incident_list";
pub const TOOL_RETRIEVE_REPORT: &str = "retrieve_report";
pub const TOOL_RETRIEVE_TELEMETRY_SLICE: &str = "retrieve_telemetry_slice";
pub const TOOL_MARK_INCIDENT_RESOLVED: &str = "mark_incident_resolved";

pub const ALL_TOOL_NAMES: &[&str] = &[
    TOOL_QUERY_TRACES,
    TOOL_QUERY_METRICS,
    TOOL_QUERY_LOGS,
    TOOL_GENERATE_SNAPSHOT,
    TOOL_QUERY_INCIDENT_LIST,
    TOOL_RETRIEVE_REPORT,
    TOOL_RETRIEVE_TELEMETRY_SLICE,
    TOOL_MARK_INCIDENT_RESOLVED,
];

/// Cross-process corpus access for the chunk #94 incident/report/telemetry
/// tools. The sidecar is a separate process with no handle on the main
/// process's in-memory `IncidentRegistry`, so these tools read/write the
/// on-disk `corpus/corpus.db` directly. `None` when corpus open failed at
/// boot (keychain unavailable etc.) — the incident tools then return a
/// JSON-RPC error rather than crashing the sidecar.
#[derive(Clone)]
pub struct IncidentToolContext {
    pub corpus: Arc<dyn CorpusWriter>,
    pub workspace_root: String,
}

const DEFAULT_TIME_WINDOW_SECONDS: u64 = 300;

// Spans table is lean per crates/buffer/src/schema.rs (chunk #44 reality
// preserved from snapshot_runtime.rs); only 6 columns selected here.
const SELECT_SPANS_RECENT: &str = "\
    SELECT trace_id, span_id, ts_unix_nano, service_name, end_time_unix_nano, status_code \
    FROM spans \
    WHERE ts_unix_nano >= ? \
    ORDER BY ts_unix_nano DESC \
    LIMIT ?";

const SNAPSHOT_SPANS_LIMIT: usize = 5000;

#[derive(Debug, Deserialize)]
pub struct QueryTracesArgs {
    #[serde(default = "default_time_window")]
    pub time_window_seconds: u64,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryMetricsArgs {
    #[serde(default = "default_time_window")]
    pub time_window_seconds: u64,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryLogsArgs {
    #[serde(default = "default_time_window")]
    pub time_window_seconds: u64,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateSnapshotArgs {
    #[serde(default = "default_token_budget")]
    pub token_budget: u32,
    #[serde(default = "default_time_window")]
    pub time_window_seconds: u64,
}

fn default_time_window() -> u64 {
    DEFAULT_TIME_WINDOW_SECONDS
}

fn default_limit() -> u32 {
    LIMIT_DEFAULT
}

fn default_token_budget() -> u32 {
    25_000
}

// Maps a user-supplied token-budget number к the nearest TokenBudget preset
// (10k / 25k / 50k per arch §Established Decisions [Snapshot Curation Default]).
// MCP clients don't pick from а preset enum directly; they pass а number и
// the dispatcher snaps к the closest spec-locked tier.
fn budget_for_count(target: u32) -> TokenBudget {
    if target < 17_500 {
        TokenBudget::Conservative
    } else if target < 37_500 {
        TokenBudget::Balanced
    } else {
        TokenBudget::Detailed
    }
}

/// Top-level dispatch from the JSON-RPC `tools/call` envelope. `params` is
/// the `params.arguments` field. Unknown tool names produce
/// `Error::ToolDispatchFailed` with reason "unknown tool"; malformed args
/// produce `Error::ToolArgsInvalid`.
pub fn dispatch_tool(
    conn: &Arc<Mutex<Connection>>,
    viz_state: &VizState,
    incident_ctx: Option<&IncidentToolContext>,
    tool_name: &str,
    arguments: &Value,
) -> Result<Value, Error> {
    let started = Instant::now();
    let result = match tool_name {
        TOOL_QUERY_TRACES => dispatch_query_traces(conn, viz_state, arguments),
        TOOL_QUERY_METRICS => dispatch_query_metrics(conn, viz_state, arguments),
        TOOL_QUERY_LOGS => dispatch_query_logs(conn, viz_state, arguments),
        TOOL_GENERATE_SNAPSHOT => dispatch_generate_snapshot(conn, arguments),
        TOOL_QUERY_INCIDENT_LIST => dispatch_query_incident_list(incident_ctx),
        TOOL_RETRIEVE_REPORT => dispatch_retrieve_report(incident_ctx, arguments),
        TOOL_RETRIEVE_TELEMETRY_SLICE => dispatch_retrieve_telemetry_slice(incident_ctx, arguments),
        TOOL_MARK_INCIDENT_RESOLVED => dispatch_mark_incident_resolved(incident_ctx, arguments),
        other => Err(Error::ToolDispatchFailed {
            tool_name: other.to_string(),
            reason: "unknown tool".to_string(),
        }),
    };

    let duration_ms = started.elapsed().as_millis() as u64;
    match &result {
        Ok(value) => {
            let result_type = result_type_label(tool_name);
            let result_count = result_count_for(tool_name, value);
            tracing::info!(
                target: "mcp.tools.call.response",
                tool_name = tool_name,
                result_type = result_type,
                result_count = result_count,
                duration_ms = duration_ms,
                "tool dispatch ok",
            );
            tracing::info!(
                target: "metric.mcp.tool_call_duration_ms",
                value = duration_ms,
                method = tool_name,
                result_count = result_count,
                "tool call duration metric",
            );
        }
        Err(e) => {
            tracing::warn!(
                target: "mcp.tools.call.error",
                tool_name = tool_name,
                duration_ms = duration_ms,
                error_detail = %e,
                "tool dispatch failed",
            );
        }
    }
    result
}

fn dispatch_query_traces(
    conn: &Arc<Mutex<Connection>>,
    viz_state: &VizState,
    arguments: &Value,
) -> Result<Value, Error> {
    let args: QueryTracesArgs = parse_args(TOOL_QUERY_TRACES, arguments)?;
    let viz_args = TracesQueryArgs {
        time_window_seconds: args.time_window_seconds,
        limit: clamp_limit(args.limit),
        cursor: args.cursor,
    };
    let response =
        query_traces(conn, viz_state, &viz_args).map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_QUERY_TRACES.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
    serde_json::to_value(&response).map_err(|_| Error::ToolDispatchFailed {
        tool_name: TOOL_QUERY_TRACES.to_string(),
        reason: "response serialization failed".to_string(),
    })
}

fn dispatch_query_metrics(
    conn: &Arc<Mutex<Connection>>,
    viz_state: &VizState,
    arguments: &Value,
) -> Result<Value, Error> {
    let args: QueryMetricsArgs = parse_args(TOOL_QUERY_METRICS, arguments)?;
    let viz_args = MetricsQueryArgs {
        time_window_seconds: args.time_window_seconds,
        limit: clamp_limit(args.limit),
        cursor: args.cursor,
    };
    let response =
        query_metrics(conn, viz_state, &viz_args).map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_QUERY_METRICS.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
    serde_json::to_value(&response).map_err(|_| Error::ToolDispatchFailed {
        tool_name: TOOL_QUERY_METRICS.to_string(),
        reason: "response serialization failed".to_string(),
    })
}

fn dispatch_query_logs(
    conn: &Arc<Mutex<Connection>>,
    viz_state: &VizState,
    arguments: &Value,
) -> Result<Value, Error> {
    let args: QueryLogsArgs = parse_args(TOOL_QUERY_LOGS, arguments)?;
    let viz_args = LogsQueryArgs {
        time_window_seconds: args.time_window_seconds,
        limit: clamp_limit(args.limit),
        cursor: args.cursor,
    };
    let response =
        query_logs(conn, viz_state, &viz_args).map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_QUERY_LOGS.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
    serde_json::to_value(&response).map_err(|_| Error::ToolDispatchFailed {
        tool_name: TOOL_QUERY_LOGS.to_string(),
        reason: "response serialization failed".to_string(),
    })
}

fn dispatch_generate_snapshot(
    conn: &Arc<Mutex<Connection>>,
    arguments: &Value,
) -> Result<Value, Error> {
    let args: GenerateSnapshotArgs = parse_args(TOOL_GENERATE_SNAPSHOT, arguments)?;
    let spans = load_recent_spans(conn, args.time_window_seconds)?;
    let curation = curate(&spans).map_err(|e| Error::ToolDispatchFailed {
        tool_name: TOOL_GENERATE_SNAPSHOT.to_string(),
        reason: short_reason(&e.to_string()),
    })?;
    let budget = budget_for_count(args.token_budget);
    let report = format_markdown(&curation, budget).map_err(|e| Error::ToolDispatchFailed {
        tool_name: TOOL_GENERATE_SNAPSHOT.to_string(),
        reason: short_reason(&e.to_string()),
    })?;
    Ok(json!({
        "token_count": report.token_count,
        "markdown": report.markdown,
        "dedup_count": curation.dedup_count,
        "input_row_count": curation.input_row_count,
        "output_row_count": curation.output_row_count,
    }))
}

// ---- Chunk #94 incident/report/telemetry tools (corpus-backed) ----

#[derive(Debug, Deserialize)]
struct IncidentIdArgs {
    incident_id: i64,
}

fn require_incident_ctx<'a>(
    tool_name: &str,
    ctx: Option<&'a IncidentToolContext>,
) -> Result<&'a IncidentToolContext, Error> {
    ctx.ok_or_else(|| Error::ToolDispatchFailed {
        tool_name: tool_name.to_string(),
        reason: "incident corpus unavailable".to_string(),
    })
}

// Decode the encrypted-then-decrypted corpus BLOB into a triage Incident.
// Same default bincode config the producer side uses in
// `pulse-app/src/incident_persistence.rs` (no custom Options on either
// side — config parity is the silent-failure risk flagged in plan Step 0).
fn decode_incident(tool_name: &str, row: &IncidentRowRaw) -> Result<Incident, Error> {
    bincode::deserialize::<Incident>(&row.payload).map_err(|_| Error::ToolDispatchFailed {
        tool_name: tool_name.to_string(),
        reason: "incident payload decode failed".to_string(),
    })
}

fn dispatch_query_incident_list(ctx: Option<&IncidentToolContext>) -> Result<Value, Error> {
    let ctx = require_incident_ctx(TOOL_QUERY_INCIDENT_LIST, ctx)?;
    let rows = ctx
        .corpus
        .load_active_incidents(&ctx.workspace_root)
        .map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_QUERY_INCIDENT_LIST.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
    let mut items: Vec<Value> = Vec::with_capacity(rows.len());
    for row in &rows {
        let incident = decode_incident(TOOL_QUERY_INCIDENT_LIST, row)?;
        items.push(json!({
            "incident_id": row.id,
            "status": row.status,
            "severity": severity_str(&incident),
            "title": incident.title,
            "opened_at_unix_nano": incident.opened_at_unix_nano,
        }));
    }
    let total = items.len();
    Ok(json!({ "items": items, "total": total, "next_cursor": Value::Null }))
}

fn dispatch_retrieve_report(
    ctx: Option<&IncidentToolContext>,
    arguments: &Value,
) -> Result<Value, Error> {
    let ctx = require_incident_ctx(TOOL_RETRIEVE_REPORT, ctx)?;
    let args: IncidentIdArgs = parse_args(TOOL_RETRIEVE_REPORT, arguments)?;
    let row = load_incident_row(ctx, TOOL_RETRIEVE_REPORT, args.incident_id)?;
    let incident = decode_incident(TOOL_RETRIEVE_REPORT, &row)?;
    let parsed_l4: Option<L4Output> = incident
        .resolution_summary_text
        .as_deref()
        .and_then(|text| serde_json::from_str::<L4Output>(text).ok());
    let degraded_mode = parsed_l4.is_none();
    // Same assemble_report + serialize_report path the in-app
    // incidents.get_report resolver uses — P-038 byte-identical delivery,
    // including the P-036 previously-seen selection over the same corpus
    // candidates + the same selection helper.
    let previously_seen = load_previously_seen(ctx, &incident);
    let report = assemble_report(&incident, parsed_l4.as_ref(), previously_seen);
    let markdown = serialize_report(&report);
    Ok(json!({ "markdown": markdown, "degraded_mode": degraded_mode }))
}

/// P-036 candidate retrieval for the sidecar's `retrieve_report`. Mirrors
/// the in-app resolver: same workspace, last 30 days, fingerprint/scope
/// match, top-5. Any failure (query or per-row decode) degrades to an
/// empty section — history never blocks report delivery.
fn load_previously_seen(
    ctx: &IncidentToolContext,
    incident: &Incident,
) -> Vec<PreviouslySeenMatch> {
    let since = current_unix_nanos()
        .saturating_sub(CORPUS_RETRIEVAL_WINDOW_SECONDS.saturating_mul(1_000_000_000));
    let Ok(rows) = ctx
        .corpus
        .load_incidents_for_workspace_since(&incident.workspace, since)
    else {
        return Vec::new();
    };
    let mut candidates = Vec::with_capacity(rows.len());
    for row in &rows {
        let Ok(mut candidate) = bincode::deserialize::<Incident>(&row.payload) else {
            return Vec::new();
        };
        candidate.id = row.id;
        candidate.workspace = row.workspace.clone();
        candidate.opened_at_unix_nano = row.created_unix_nano;
        candidates.push(candidate);
    }
    previously_seen_from_incidents(&select_previously_seen(
        incident,
        candidates,
        DIGEST_CORPUS_RETRIEVAL_LIMIT,
    ))
}

fn dispatch_retrieve_telemetry_slice(
    ctx: Option<&IncidentToolContext>,
    arguments: &Value,
) -> Result<Value, Error> {
    let ctx = require_incident_ctx(TOOL_RETRIEVE_TELEMETRY_SLICE, ctx)?;
    let args: IncidentIdArgs = parse_args(TOOL_RETRIEVE_TELEMETRY_SLICE, arguments)?;
    let row = load_incident_row(ctx, TOOL_RETRIEVE_TELEMETRY_SLICE, args.incident_id)?;
    let incident = decode_incident(TOOL_RETRIEVE_TELEMETRY_SLICE, &row)?;
    let span_refs: Vec<String> = incident
        .evidence_refs
        .span_ids
        .iter()
        .map(|bytes| format!("span:{}", hex_lower(bytes)))
        .collect();
    let fingerprint_refs: Vec<String> = incident.evidence_refs.fingerprint_hashes.clone();
    Ok(json!({
        "incident_id": row.id,
        "span_refs": span_refs,
        "fingerprint_refs": fingerprint_refs,
        "timestamps_unix_nano": incident.evidence_refs.timestamps_unix_nano,
    }))
}

fn dispatch_mark_incident_resolved(
    ctx: Option<&IncidentToolContext>,
    arguments: &Value,
) -> Result<Value, Error> {
    let ctx = require_incident_ctx(TOOL_MARK_INCIDENT_RESOLVED, ctx)?;
    let args: IncidentIdArgs = parse_args(TOOL_MARK_INCIDENT_RESOLVED, arguments)?;
    let row = load_incident_row(ctx, TOOL_MARK_INCIDENT_RESOLVED, args.incident_id)?;
    let mut incident = decode_incident(TOOL_MARK_INCIDENT_RESOLVED, &row)?;
    let now = current_unix_nanos();
    incident.status = triage::contract::IncidentStatus::Resolved;
    incident.resolved_at_unix_nano = Some(now);
    incident.updated_at_unix_nano = now;
    let payload = bincode::serialize(&incident).map_err(|_| Error::ToolDispatchFailed {
        tool_name: TOOL_MARK_INCIDENT_RESOLVED.to_string(),
        reason: "incident payload encode failed".to_string(),
    })?;
    ctx.corpus
        .update_incident_status(row.id, "resolved", now, Some(now), &payload)
        .map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_MARK_INCIDENT_RESOLVED.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
    Ok(json!({ "resolved": true, "incident_id": row.id }))
}

fn load_incident_row(
    ctx: &IncidentToolContext,
    tool_name: &str,
    id: i64,
) -> Result<IncidentRowRaw, Error> {
    ctx.corpus
        .load_incident_by_id(id)
        .map_err(|e| Error::ToolDispatchFailed {
            tool_name: tool_name.to_string(),
            reason: short_reason(&e.to_string()),
        })?
        .ok_or_else(|| Error::ToolDispatchFailed {
            tool_name: tool_name.to_string(),
            reason: "incident not found".to_string(),
        })
}

fn severity_str(incident: &Incident) -> &'static str {
    use triage::contract::Severity;
    match incident.severity {
        Severity::Info => "info",
        Severity::Warn => "warn",
        Severity::Error => "error",
        Severity::Critical => "critical",
    }
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn current_unix_nanos() -> i64 {
    chrono::Utc::now().timestamp_nanos_opt().unwrap_or(i64::MAX)
}

fn parse_args<T: for<'de> Deserialize<'de>>(tool_name: &str, value: &Value) -> Result<T, Error> {
    serde_json::from_value(value.clone()).map_err(|e| Error::ToolArgsInvalid {
        tool_name: tool_name.to_string(),
        reason: short_reason(&e.to_string()),
    })
}

fn clamp_limit(limit: u32) -> u32 {
    if limit == 0 {
        LIMIT_DEFAULT
    } else if limit > LIMIT_MAX {
        LIMIT_MAX
    } else {
        limit
    }
}

// Truncate an error message to a single short sentence and strip path /
// crate-path / version-like tokens before it crosses the JSON-RPC envelope.
// Defense in depth: jsonrpc::sanitize_error_data also scrubs at framing time.
fn short_reason(raw: &str) -> String {
    let first_line = raw.lines().next().unwrap_or("").trim();
    let mut out = String::new();
    for token in first_line.split_whitespace() {
        let redact = token.contains('/')
            || token.contains('\\')
            || token.contains("::")
            || (token.starts_with('v') && token.chars().any(|c| c.is_ascii_digit()));
        if redact {
            out.push_str("[redacted] ");
        } else {
            out.push_str(token);
            out.push(' ');
        }
    }
    let trimmed = out.trim();
    if trimmed.len() > 120 {
        trimmed[..120].to_string()
    } else {
        trimmed.to_string()
    }
}

fn result_type_label(tool_name: &str) -> &'static str {
    match tool_name {
        TOOL_QUERY_TRACES => "paginated_traces",
        TOOL_QUERY_METRICS => "paginated_metrics",
        TOOL_QUERY_LOGS => "paginated_logs",
        TOOL_GENERATE_SNAPSHOT => "markdown_snapshot",
        TOOL_QUERY_INCIDENT_LIST => "incident_list",
        TOOL_RETRIEVE_REPORT => "markdown_report",
        TOOL_RETRIEVE_TELEMETRY_SLICE => "telemetry_slice",
        TOOL_MARK_INCIDENT_RESOLVED => "resolve_ack",
        _ => "unknown",
    }
}

fn result_count_for(tool_name: &str, value: &Value) -> u64 {
    match tool_name {
        TOOL_QUERY_TRACES | TOOL_QUERY_METRICS | TOOL_QUERY_LOGS | TOOL_QUERY_INCIDENT_LIST => {
            value
                .get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.len() as u64)
                .unwrap_or(0)
        }
        TOOL_GENERATE_SNAPSHOT => value
            .get("output_row_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
        TOOL_RETRIEVE_TELEMETRY_SLICE => value
            .get("span_refs")
            .and_then(|v| v.as_array())
            .map(|a| a.len() as u64)
            .unwrap_or(0),
        // Single-item results: 1 on success (the dispatch only reaches the
        // count helper on the Ok branch).
        TOOL_RETRIEVE_REPORT | TOOL_MARK_INCIDENT_RESOLVED => 1,
        _ => 0,
    }
}

// Pulls a recent slice from the spans table; mirrors snapshot_runtime.rs
// behavior so both surfaces feed the curation pipeline with consistent
// input. Returns empty vec when buffer is empty (chunk #49 substrate's
// ephemeral connection starts empty; live cross-process buffer sharing is
// deferred per plan §Implementation notes).
fn load_recent_spans(
    conn: &Arc<Mutex<Connection>>,
    time_window_seconds: u64,
) -> Result<Vec<SpanRecord>, Error> {
    let cutoff = compute_cutoff(time_window_seconds)?;
    let guard = conn.lock().map_err(|_| Error::ToolDispatchFailed {
        tool_name: TOOL_GENERATE_SNAPSHOT.to_string(),
        reason: "buffer connection lock poisoned".to_string(),
    })?;
    let mut stmt = guard
        .prepare(SELECT_SPANS_RECENT)
        .map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_GENERATE_SNAPSHOT.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
    let rows = stmt
        .query_map(
            duckdb::params![cutoff, SNAPSHOT_SPANS_LIMIT as i64],
            |row| {
                let trace_id_blob: Vec<u8> = row.get(0)?;
                let span_id_blob: Vec<u8> = row.get(1)?;
                let ts_unix_nano: i64 = row.get(2)?;
                let service_name: String = row.get(3)?;
                let end_time_unix_nano: i64 = row.get(4)?;
                let status_code: i32 = row.get(5)?;
                Ok((
                    trace_id_blob,
                    span_id_blob,
                    ts_unix_nano,
                    service_name,
                    end_time_unix_nano,
                    status_code,
                ))
            },
        )
        .map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_GENERATE_SNAPSHOT.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
    let mut out: Vec<SpanRecord> = Vec::new();
    for row in rows {
        let (
            trace_id_blob,
            span_id_blob,
            ts_unix_nano,
            service_name,
            end_time_unix_nano,
            status_code,
        ) = row.map_err(|e| Error::ToolDispatchFailed {
            tool_name: TOOL_GENERATE_SNAPSHOT.to_string(),
            reason: short_reason(&e.to_string()),
        })?;
        let mut trace_id = [0u8; 16];
        let trace_len = trace_id_blob.len().min(16);
        trace_id[..trace_len].copy_from_slice(&trace_id_blob[..trace_len]);
        let mut span_id = [0u8; 8];
        let span_len = span_id_blob.len().min(8);
        span_id[..span_len].copy_from_slice(&span_id_blob[..span_len]);
        out.push(SpanRecord {
            trace_id,
            span_id,
            parent_span_id: None,
            service_name,
            name: String::new(),
            start_time_unix_nano: ts_unix_nano,
            end_time_unix_nano,
            status_code: status_code.clamp(0, u8::MAX as i32) as u8,
            attributes: Vec::new(),
        });
    }
    Ok(out)
}

fn compute_cutoff(time_window_seconds: u64) -> Result<i64, Error> {
    let now_ns =
        chrono::Utc::now()
            .timestamp_nanos_opt()
            .ok_or_else(|| Error::ToolDispatchFailed {
                tool_name: TOOL_GENERATE_SNAPSHOT.to_string(),
                reason: "system clock out of range".to_string(),
            })?;
    let window_ns: i64 = (time_window_seconds as i64).saturating_mul(1_000_000_000);
    Ok(now_ns.saturating_sub(window_ns))
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffer::schema::create_schema;
    use serde_json::json;

    fn fresh_buffer() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().expect("open in-memory duckdb");
        create_schema(&conn).expect("schema init");
        Arc::new(Mutex::new(conn))
    }

    #[test]
    fn dispatch_query_traces_returns_empty_paginated_response_on_empty_buffer() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let value =
            dispatch_tool(&conn, &state, None, TOOL_QUERY_TRACES, &json!({})).expect("dispatch ok");
        let items = value
            .get("items")
            .and_then(|v| v.as_array())
            .expect("items array");
        assert_eq!(items.len(), 0);
        assert_eq!(value.get("total").and_then(|v| v.as_u64()), Some(0));
    }

    #[test]
    fn dispatch_query_metrics_returns_empty_paginated_response_on_empty_buffer() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let value = dispatch_tool(&conn, &state, None, TOOL_QUERY_METRICS, &json!({}))
            .expect("dispatch ok");
        let items = value
            .get("items")
            .and_then(|v| v.as_array())
            .expect("items array");
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn dispatch_query_logs_returns_empty_paginated_response_on_empty_buffer() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let value =
            dispatch_tool(&conn, &state, None, TOOL_QUERY_LOGS, &json!({})).expect("dispatch ok");
        let items = value
            .get("items")
            .and_then(|v| v.as_array())
            .expect("items array");
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn dispatch_generate_snapshot_returns_envelope_on_empty_buffer() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let value = dispatch_tool(&conn, &state, None, TOOL_GENERATE_SNAPSHOT, &json!({}))
            .expect("dispatch ok");
        assert!(value.get("markdown").and_then(|v| v.as_str()).is_some());
        assert_eq!(
            value.get("input_row_count").and_then(|v| v.as_u64()),
            Some(0)
        );
    }

    #[test]
    fn dispatch_unknown_tool_returns_tool_dispatch_failed_unknown_tool() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let err = dispatch_tool(&conn, &state, None, "nonexistent_tool", &json!({}))
            .expect_err("unknown tool errors");
        match err {
            Error::ToolDispatchFailed { tool_name, reason } => {
                assert_eq!(tool_name, "nonexistent_tool");
                assert!(reason.contains("unknown"));
            }
            other => panic!("expected ToolDispatchFailed, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_query_traces_rejects_malformed_arguments() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let err = dispatch_tool(
            &conn,
            &state,
            None,
            TOOL_QUERY_TRACES,
            &json!({"time_window_seconds": "not-a-number"}),
        )
        .expect_err("malformed args reject");
        match err {
            Error::ToolArgsInvalid { tool_name, .. } => {
                assert_eq!(tool_name, "query_traces");
            }
            other => panic!("expected ToolArgsInvalid, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_query_traces_clamps_limit_above_max() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let value = dispatch_tool(
            &conn,
            &state,
            None,
            TOOL_QUERY_TRACES,
            &json!({"time_window_seconds": 60, "limit": 100_000}),
        )
        .expect("dispatch ok with clamped limit");
        let items = value
            .get("items")
            .and_then(|v| v.as_array())
            .expect("items");
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn short_reason_redacts_paths_and_versions() {
        let r = short_reason("buffer error at /home/user/secret.rs v0.6.4 crate::Foo::Bar");
        assert!(!r.contains("/home"));
        assert!(!r.contains("v0.6.4"));
        assert!(!r.contains("::"));
        assert!(r.contains("[redacted]"));
    }

    #[test]
    fn all_tool_names_count_is_eight() {
        assert_eq!(ALL_TOOL_NAMES.len(), 8);
    }

    #[test]
    fn result_type_label_known_tools() {
        assert_eq!(result_type_label(TOOL_QUERY_TRACES), "paginated_traces");
        assert_eq!(result_type_label(TOOL_QUERY_METRICS), "paginated_metrics");
        assert_eq!(result_type_label(TOOL_QUERY_LOGS), "paginated_logs");
        assert_eq!(
            result_type_label(TOOL_GENERATE_SNAPSHOT),
            "markdown_snapshot"
        );
        assert_eq!(result_type_label(TOOL_QUERY_INCIDENT_LIST), "incident_list");
        assert_eq!(result_type_label(TOOL_RETRIEVE_REPORT), "markdown_report");
        assert_eq!(
            result_type_label(TOOL_RETRIEVE_TELEMETRY_SLICE),
            "telemetry_slice"
        );
        assert_eq!(
            result_type_label(TOOL_MARK_INCIDENT_RESOLVED),
            "resolve_ack"
        );
        assert_eq!(result_type_label("bogus"), "unknown");
    }

    // ---- Chunk #94 corpus-backed incident tools ----

    use corpus::contract::{Corpus, FakeKeychainBackend, KeychainBackend};
    use triage::contract::{
        CueKind, CueScope, EvidenceRefs, IncidentStatus, PriorityTier, Severity,
    };

    fn incident_ctx_with_one_incident() -> (IncidentToolContext, i64) {
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
        let corpus = Corpus::open_in_memory(backend).expect("in-memory corpus");
        let corpus: Arc<dyn CorpusWriter> = Arc::new(corpus);
        let incident = Incident {
            id: 0,
            workspace: "ws-test".into(),
            fingerprint: "fp-1".into(),
            title: "Pool saturation".into(),
            detail: "Connection pool exhausted".into(),
            kind: CueKind::ErrorRateSpike,
            scope: CueScope::Service,
            scope_id: Some("service-a".into()),
            status: IncidentStatus::Active,
            severity: Severity::Error,
            priority_tier: PriorityTier::Suggested,
            evidence_refs: EvidenceRefs {
                trace_id: None,
                span_ids: vec![[1u8; 8]],
                fingerprint_hashes: vec!["fp-1".into()],
                timestamps_unix_nano: vec![1_700_000_000_000],
            },
            opened_at_unix_nano: 1_700_000_000_000,
            updated_at_unix_nano: 1_700_000_000_000,
            acknowledged_at_unix_nano: None,
            resolved_at_unix_nano: None,
            read_at_unix_nano: None,
            resolution_summary_text: None,
        };
        let payload = bincode::serialize(&incident).expect("encode");
        let id = corpus
            .save_incident(
                "ws-test",
                "active",
                1_700_000_000_000,
                1_700_000_000_000,
                None,
                None,
                &payload,
            )
            .expect("save");
        let ctx = IncidentToolContext {
            corpus,
            workspace_root: "ws-test".into(),
        };
        (ctx, id)
    }

    #[test]
    fn query_incident_list_returns_active_incidents() {
        let (ctx, id) = incident_ctx_with_one_incident();
        let value = dispatch_query_incident_list(Some(&ctx)).expect("dispatch ok");
        let items = value["items"].as_array().expect("items array");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["incident_id"].as_i64(), Some(id));
        assert_eq!(items[0]["severity"].as_str(), Some("error"));
        assert_eq!(items[0]["title"].as_str(), Some("Pool saturation"));
        assert_eq!(value["total"].as_u64(), Some(1));
    }

    #[test]
    fn retrieve_report_returns_degraded_markdown_for_active_incident() {
        let (ctx, id) = incident_ctx_with_one_incident();
        let value =
            dispatch_retrieve_report(Some(&ctx), &json!({"incident_id": id})).expect("dispatch ok");
        let md = value["markdown"].as_str().expect("markdown string");
        assert!(md.starts_with("# Diagnostic Report:"));
        assert!(md.contains("## Symptom"));
        assert_eq!(value["degraded_mode"].as_bool(), Some(true));
    }

    #[test]
    fn retrieve_telemetry_slice_returns_evidence_refs() {
        let (ctx, id) = incident_ctx_with_one_incident();
        let value = dispatch_retrieve_telemetry_slice(Some(&ctx), &json!({"incident_id": id}))
            .expect("dispatch ok");
        let span_refs = value["span_refs"].as_array().expect("span_refs");
        assert_eq!(span_refs.len(), 1);
        assert_eq!(span_refs[0].as_str(), Some("span:0101010101010101"));
        let fp_refs = value["fingerprint_refs"].as_array().expect("fp_refs");
        assert_eq!(fp_refs[0].as_str(), Some("fp-1"));
    }

    #[test]
    fn mark_incident_resolved_writes_resolved_status() {
        let (ctx, id) = incident_ctx_with_one_incident();
        let value = dispatch_mark_incident_resolved(Some(&ctx), &json!({"incident_id": id}))
            .expect("dispatch ok");
        assert_eq!(value["resolved"].as_bool(), Some(true));
        assert_eq!(value["incident_id"].as_i64(), Some(id));
        // Re-read: status is now resolved, no longer in the active list.
        let row = ctx
            .corpus
            .load_incident_by_id(id)
            .expect("load")
            .expect("row");
        assert_eq!(row.status, "resolved");
    }

    #[test]
    fn incident_tools_error_when_corpus_unavailable() {
        let err = dispatch_query_incident_list(None).expect_err("no corpus");
        match err {
            Error::ToolDispatchFailed { reason, .. } => {
                assert!(reason.contains("corpus unavailable"));
            }
            other => panic!("expected ToolDispatchFailed, got {other:?}"),
        }
    }

    #[test]
    fn retrieve_report_errors_on_missing_incident() {
        let (ctx, _id) = incident_ctx_with_one_incident();
        let err = dispatch_retrieve_report(Some(&ctx), &json!({"incident_id": 99_999}))
            .expect_err("not found");
        match err {
            Error::ToolDispatchFailed { reason, .. } => {
                assert!(reason.contains("not found"));
            }
            other => panic!("expected ToolDispatchFailed, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_tool_routes_incident_tools_through_context() {
        let conn = fresh_buffer();
        let state = VizState::new();
        let (ctx, _id) = incident_ctx_with_one_incident();
        let value = dispatch_tool(
            &conn,
            &state,
            Some(&ctx),
            TOOL_QUERY_INCIDENT_LIST,
            &json!({}),
        )
        .expect("dispatch ok");
        assert_eq!(value["total"].as_u64(), Some(1));
    }
}
