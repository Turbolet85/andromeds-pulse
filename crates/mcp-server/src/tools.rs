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

use duckdb::Connection;
use serde::Deserialize;
use serde_json::{Value, json};
use snapshot::contract::{SpanRecord, TokenBudget, curate, format_markdown};
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

pub const ALL_TOOL_NAMES: &[&str] = &[
    TOOL_QUERY_TRACES,
    TOOL_QUERY_METRICS,
    TOOL_QUERY_LOGS,
    TOOL_GENERATE_SNAPSHOT,
];

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
    tool_name: &str,
    arguments: &Value,
) -> Result<Value, Error> {
    let started = Instant::now();
    let result = match tool_name {
        TOOL_QUERY_TRACES => dispatch_query_traces(conn, viz_state, arguments),
        TOOL_QUERY_METRICS => dispatch_query_metrics(conn, viz_state, arguments),
        TOOL_QUERY_LOGS => dispatch_query_logs(conn, viz_state, arguments),
        TOOL_GENERATE_SNAPSHOT => dispatch_generate_snapshot(conn, arguments),
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
        _ => "unknown",
    }
}

fn result_count_for(tool_name: &str, value: &Value) -> u64 {
    match tool_name {
        TOOL_QUERY_TRACES | TOOL_QUERY_METRICS | TOOL_QUERY_LOGS => value
            .get("items")
            .and_then(|v| v.as_array())
            .map(|a| a.len() as u64)
            .unwrap_or(0),
        TOOL_GENERATE_SNAPSHOT => value
            .get("output_row_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
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
            dispatch_tool(&conn, &state, TOOL_QUERY_TRACES, &json!({})).expect("dispatch ok");
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
        let value =
            dispatch_tool(&conn, &state, TOOL_QUERY_METRICS, &json!({})).expect("dispatch ok");
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
        let value = dispatch_tool(&conn, &state, TOOL_QUERY_LOGS, &json!({})).expect("dispatch ok");
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
        let value =
            dispatch_tool(&conn, &state, TOOL_GENERATE_SNAPSHOT, &json!({})).expect("dispatch ok");
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
        let err = dispatch_tool(&conn, &state, "nonexistent_tool", &json!({}))
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
    fn all_tool_names_count_is_four() {
        assert_eq!(ALL_TOOL_NAMES.len(), 4);
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
        assert_eq!(result_type_label("bogus"), "unknown");
    }
}
