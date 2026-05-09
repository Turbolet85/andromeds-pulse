use std::sync::{Arc, Mutex};
use std::time::Instant;

use duckdb::Connection;
use serde::{Deserialize, Serialize};

use crate::contract::Error;
use crate::state::VizState;

pub const LIMIT_MAX: u32 = 1_000;
pub const LIMIT_DEFAULT: u32 = 100;

const SELECT_TRACES: &str = "SELECT trace_id, span_id, ts_unix_nano, service_name, end_time_unix_nano, status_code \
     FROM spans \
     WHERE ts_unix_nano >= ? AND ts_unix_nano < ? \
     ORDER BY ts_unix_nano DESC, trace_id LIMIT ?";

const SELECT_METRICS: &str = "SELECT metric_name, ts_unix_nano, resource_hash FROM metrics_points \
     WHERE ts_unix_nano >= ? AND ts_unix_nano < ? \
     ORDER BY ts_unix_nano DESC, metric_name LIMIT ?";

const SELECT_LOGS: &str = "SELECT ts_unix_nano, resource_hash, severity_number FROM log_records \
     WHERE ts_unix_nano >= ? AND ts_unix_nano < ? \
     ORDER BY ts_unix_nano DESC, severity_number LIMIT ?";

const COUNT_TRACES: &str =
    "SELECT COUNT(*) FROM spans WHERE ts_unix_nano >= ? AND ts_unix_nano < ?";
const COUNT_METRICS: &str =
    "SELECT COUNT(*) FROM metrics_points WHERE ts_unix_nano >= ? AND ts_unix_nano < ?";
const COUNT_LOGS: &str =
    "SELECT COUNT(*) FROM log_records WHERE ts_unix_nano >= ? AND ts_unix_nano < ?";

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TracesQueryArgs {
    pub time_window_seconds: u64,
    pub limit: u32,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct MetricsQueryArgs {
    pub time_window_seconds: u64,
    pub limit: u32,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct LogsQueryArgs {
    pub time_window_seconds: u64,
    pub limit: u32,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PaginatedResponse<T: specta::Type> {
    pub items: Vec<T>,
    pub total: u64,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TraceRow {
    pub trace_id: String,
    pub span_id: String,
    pub ts_unix_nano: i64,
    pub service: String,
    pub duration_ms: u64,
    // 0 (Unset) | 1 (Ok) → 0; 2 (Error) → 1, per OTLP Status.code spec.
    pub error_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct MetricRow {
    pub metric_name: String,
    pub ts_unix_nano: i64,
    pub resource_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct LogRow {
    pub ts_unix_nano: i64,
    pub resource_hash: String,
    pub severity_number: i32,
}

pub fn query_traces(
    conn: &Arc<Mutex<Connection>>,
    state: &VizState,
    args: &TracesQueryArgs,
) -> Result<PaginatedResponse<TraceRow>, Error> {
    validate_args(args.limit)?;
    let (start_ns, end_ns) = compute_window(args.time_window_seconds, args.cursor.as_deref())?;

    let start = Instant::now();
    let guard = conn.lock().map_err(|_| Error::ConnectionLost)?;

    let total: u64 = {
        let mut stmt = guard
            .prepare(COUNT_TRACES)
            .map_err(|e| Error::QueryFailed {
                reason: format!("prepare count: {}", short_err(&e.to_string())),
            })?;
        stmt.query_row([start_ns, end_ns], |row| row.get::<_, i64>(0))
            .map_err(|e| Error::QueryFailed {
                reason: format!("count: {}", short_err(&e.to_string())),
            })? as u64
    };

    let mut stmt = guard
        .prepare(SELECT_TRACES)
        .map_err(|e| Error::QueryFailed {
            reason: format!("prepare select: {}", short_err(&e.to_string())),
        })?;

    let rows = stmt
        .query_map(
            duckdb::params![start_ns, end_ns, args.limit as i64],
            |row| {
                let trace_id_blob: Vec<u8> = row.get(0)?;
                let span_id_blob: Vec<u8> = row.get(1)?;
                let ts_unix_nano: i64 = row.get(2)?;
                let service: String = row.get(3)?;
                let end_time_unix_nano: i64 = row.get(4)?;
                let status_code: i32 = row.get(5)?;
                Ok((
                    trace_id_blob,
                    span_id_blob,
                    ts_unix_nano,
                    service,
                    end_time_unix_nano,
                    status_code,
                ))
            },
        )
        .map_err(|e| Error::QueryFailed {
            reason: format!("query_map: {}", short_err(&e.to_string())),
        })?;

    let mut items: Vec<TraceRow> = Vec::new();
    for row in rows {
        let (trace_id_blob, span_id_blob, ts_unix_nano, service, end_time_unix_nano, status_code) =
            row.map_err(|e| Error::Decode {
                reason: format!("row: {}", short_err(&e.to_string())),
            })?;
        let duration_ns = end_time_unix_nano.saturating_sub(ts_unix_nano).max(0);
        items.push(TraceRow {
            trace_id: hex_encode(&trace_id_blob),
            span_id: hex_encode(&span_id_blob),
            ts_unix_nano,
            service,
            duration_ms: (duration_ns / 1_000_000) as u64,
            error_count: if status_code == 2 { 1 } else { 0 },
        });
    }

    drop(stmt);
    drop(guard);

    let next_cursor = compute_next_cursor(items.last().map(|r| r.ts_unix_nano), args.limit, &items);

    let duration_ms = start.elapsed().as_millis() as u64;
    let row_count = items.len() as u64;
    state.record_query_latency_ms(duration_ms);

    tracing::info!(
        target: "viz.query.traces",
        query_id = "viz.query.traces",
        param_count = 3_u64,
        param_types = "i64,i64,i64",
        time_window_seconds = args.time_window_seconds,
        row_count = row_count,
        latency_ms = duration_ms,
        "viz traces query completed",
    );

    Ok(PaginatedResponse {
        items,
        total,
        next_cursor,
    })
}

pub fn query_metrics(
    conn: &Arc<Mutex<Connection>>,
    state: &VizState,
    args: &MetricsQueryArgs,
) -> Result<PaginatedResponse<MetricRow>, Error> {
    validate_args(args.limit)?;
    let (start_ns, end_ns) = compute_window(args.time_window_seconds, args.cursor.as_deref())?;

    let start = Instant::now();
    let guard = conn.lock().map_err(|_| Error::ConnectionLost)?;

    let total: u64 = {
        let mut stmt = guard
            .prepare(COUNT_METRICS)
            .map_err(|e| Error::QueryFailed {
                reason: format!("prepare count: {}", short_err(&e.to_string())),
            })?;
        stmt.query_row([start_ns, end_ns], |row| row.get::<_, i64>(0))
            .map_err(|e| Error::QueryFailed {
                reason: format!("count: {}", short_err(&e.to_string())),
            })? as u64
    };

    let mut stmt = guard
        .prepare(SELECT_METRICS)
        .map_err(|e| Error::QueryFailed {
            reason: format!("prepare select: {}", short_err(&e.to_string())),
        })?;

    let rows = stmt
        .query_map(
            duckdb::params![start_ns, end_ns, args.limit as i64],
            |row| {
                let metric_name: String = row.get(0)?;
                let ts_unix_nano: i64 = row.get(1)?;
                let resource_hash_blob: Vec<u8> = row.get(2)?;
                Ok((metric_name, ts_unix_nano, resource_hash_blob))
            },
        )
        .map_err(|e| Error::QueryFailed {
            reason: format!("query_map: {}", short_err(&e.to_string())),
        })?;

    let mut items: Vec<MetricRow> = Vec::new();
    for row in rows {
        let (metric_name, ts_unix_nano, resource_hash_blob) = row.map_err(|e| Error::Decode {
            reason: format!("row: {}", short_err(&e.to_string())),
        })?;
        items.push(MetricRow {
            metric_name,
            ts_unix_nano,
            resource_hash: hex_encode(&resource_hash_blob),
        });
    }

    drop(stmt);
    drop(guard);

    let next_cursor = compute_next_cursor(items.last().map(|r| r.ts_unix_nano), args.limit, &items);

    let duration_ms = start.elapsed().as_millis() as u64;
    let row_count = items.len() as u64;
    state.record_query_latency_ms(duration_ms);

    tracing::info!(
        target: "viz.query.metrics",
        query_id = "viz.query.metrics",
        param_count = 3_u64,
        param_types = "i64,i64,i64",
        time_window_seconds = args.time_window_seconds,
        row_count = row_count,
        latency_ms = duration_ms,
        "viz metrics query completed",
    );

    Ok(PaginatedResponse {
        items,
        total,
        next_cursor,
    })
}

pub fn query_logs(
    conn: &Arc<Mutex<Connection>>,
    state: &VizState,
    args: &LogsQueryArgs,
) -> Result<PaginatedResponse<LogRow>, Error> {
    validate_args(args.limit)?;
    let (start_ns, end_ns) = compute_window(args.time_window_seconds, args.cursor.as_deref())?;

    let start = Instant::now();
    let guard = conn.lock().map_err(|_| Error::ConnectionLost)?;

    let total: u64 = {
        let mut stmt = guard.prepare(COUNT_LOGS).map_err(|e| Error::QueryFailed {
            reason: format!("prepare count: {}", short_err(&e.to_string())),
        })?;
        stmt.query_row([start_ns, end_ns], |row| row.get::<_, i64>(0))
            .map_err(|e| Error::QueryFailed {
                reason: format!("count: {}", short_err(&e.to_string())),
            })? as u64
    };

    let mut stmt = guard.prepare(SELECT_LOGS).map_err(|e| Error::QueryFailed {
        reason: format!("prepare select: {}", short_err(&e.to_string())),
    })?;

    let rows = stmt
        .query_map(
            duckdb::params![start_ns, end_ns, args.limit as i64],
            |row| {
                let ts_unix_nano: i64 = row.get(0)?;
                let resource_hash_blob: Vec<u8> = row.get(1)?;
                let severity_number: i32 = row.get(2)?;
                Ok((ts_unix_nano, resource_hash_blob, severity_number))
            },
        )
        .map_err(|e| Error::QueryFailed {
            reason: format!("query_map: {}", short_err(&e.to_string())),
        })?;

    let mut items: Vec<LogRow> = Vec::new();
    for row in rows {
        let (ts_unix_nano, resource_hash_blob, severity_number) =
            row.map_err(|e| Error::Decode {
                reason: format!("row: {}", short_err(&e.to_string())),
            })?;
        items.push(LogRow {
            ts_unix_nano,
            resource_hash: hex_encode(&resource_hash_blob),
            severity_number,
        });
    }

    drop(stmt);
    drop(guard);

    let next_cursor = compute_next_cursor(items.last().map(|r| r.ts_unix_nano), args.limit, &items);

    let duration_ms = start.elapsed().as_millis() as u64;
    let row_count = items.len() as u64;
    state.record_query_latency_ms(duration_ms);

    tracing::info!(
        target: "viz.query.logs",
        query_id = "viz.query.logs",
        param_count = 3_u64,
        param_types = "i64,i64,i64",
        time_window_seconds = args.time_window_seconds,
        row_count = row_count,
        latency_ms = duration_ms,
        "viz logs query completed",
    );

    Ok(PaginatedResponse {
        items,
        total,
        next_cursor,
    })
}

fn validate_args(limit: u32) -> Result<(), Error> {
    if limit == 0 {
        return Err(Error::InvalidArgument {
            field: "limit".into(),
            reason: "must be > 0".into(),
        });
    }
    if limit > LIMIT_MAX {
        return Err(Error::InvalidArgument {
            field: "limit".into(),
            reason: "above max".into(),
        });
    }
    Ok(())
}

fn compute_window(time_window_seconds: u64, cursor: Option<&str>) -> Result<(i64, i64), Error> {
    let now_ns = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(i64::MAX);
    let start_ns =
        now_ns.saturating_sub((time_window_seconds as i64).saturating_mul(1_000_000_000));
    let end_ns = match cursor {
        Some(s) => decode_cursor(s)?,
        None => i64::MAX,
    };
    if end_ns < start_ns {
        return Ok((start_ns, start_ns));
    }
    Ok((start_ns, end_ns))
}

fn compute_next_cursor<T>(last_ts_ns: Option<i64>, limit: u32, items: &[T]) -> Option<String> {
    if items.len() < limit as usize {
        return None;
    }
    last_ts_ns.map(encode_cursor)
}

fn encode_cursor(ts_unix_nano: i64) -> String {
    format!("c1:{ts_unix_nano}")
}

fn decode_cursor(cursor: &str) -> Result<i64, Error> {
    let after_prefix = cursor
        .strip_prefix("c1:")
        .ok_or_else(|| Error::InvalidArgument {
            field: "cursor".into(),
            reason: "unrecognized cursor".into(),
        })?;
    after_prefix
        .parse::<i64>()
        .map_err(|_| Error::InvalidArgument {
            field: "cursor".into(),
            reason: "cursor decode failed".into(),
        })
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn short_err(raw: &str) -> String {
    raw.lines().next().unwrap_or("").chars().take(120).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::Error as VizError;
    use std::sync::{Arc, Mutex};

    fn open_in_memory_with_schema() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().expect("open_in_memory");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS spans (
                trace_id BLOB NOT NULL,
                span_id BLOB NOT NULL,
                ts TIMESTAMPTZ NOT NULL,
                ts_unix_nano BIGINT NOT NULL,
                service_name VARCHAR NOT NULL,
                end_time_unix_nano BIGINT NOT NULL,
                status_code INTEGER NOT NULL,
                PRIMARY KEY (trace_id, span_id)
            );
            CREATE TABLE IF NOT EXISTS metrics_points (
                metric_name VARCHAR NOT NULL,
                ts TIMESTAMPTZ NOT NULL,
                ts_unix_nano BIGINT NOT NULL,
                resource_hash BLOB NOT NULL,
                PRIMARY KEY (metric_name, ts_unix_nano, resource_hash)
            );
            CREATE TABLE IF NOT EXISTS log_records (
                ts TIMESTAMPTZ NOT NULL,
                ts_unix_nano BIGINT NOT NULL,
                resource_hash BLOB NOT NULL,
                severity_number INTEGER NOT NULL,
                PRIMARY KEY (ts_unix_nano, resource_hash, severity_number)
            );",
        )
        .expect("create_schema");
        Arc::new(Mutex::new(conn))
    }

    fn seed_span(conn: &Arc<Mutex<Connection>>, trace_id: u8, span_id: u8, ts_ns: i64) {
        seed_span_full(conn, trace_id, span_id, ts_ns, "", ts_ns + 1_000_000, 0);
    }

    fn seed_span_full(
        conn: &Arc<Mutex<Connection>>,
        trace_id: u8,
        span_id: u8,
        ts_ns: i64,
        service: &str,
        end_ns: i64,
        status_code: i32,
    ) {
        let guard = conn.lock().expect("lock");
        let trace_blob: Vec<u8> = vec![trace_id; 16];
        let span_blob: Vec<u8> = vec![span_id; 8];
        guard
            .execute(
                "INSERT INTO spans (trace_id, span_id, ts, ts_unix_nano, service_name, end_time_unix_nano, status_code) \
                 VALUES (?, ?, '2026-05-06T00:00:00Z'::TIMESTAMPTZ, ?, ?, ?, ?)",
                duckdb::params![
                    trace_blob.as_slice(),
                    span_blob.as_slice(),
                    ts_ns,
                    service,
                    end_ns,
                    status_code,
                ],
            )
            .expect("insert span");
    }

    fn seed_log(conn: &Arc<Mutex<Connection>>, ts_ns: i64, severity: i32) {
        let guard = conn.lock().expect("lock");
        let resource_hash: Vec<u8> = vec![1u8; 16];
        guard
            .execute(
                "INSERT INTO log_records (ts, ts_unix_nano, resource_hash, severity_number) VALUES ('2026-05-06T00:00:00Z'::TIMESTAMPTZ, ?, ?, ?)",
                duckdb::params![ts_ns, resource_hash.as_slice(), severity],
            )
            .expect("insert log");
    }

    fn seed_metric(conn: &Arc<Mutex<Connection>>, name: &str, ts_ns: i64) {
        let guard = conn.lock().expect("lock");
        let resource_hash: Vec<u8> = vec![2u8; 16];
        guard
            .execute(
                "INSERT INTO metrics_points (metric_name, ts, ts_unix_nano, resource_hash) VALUES (?, '2026-05-06T00:00:00Z'::TIMESTAMPTZ, ?, ?)",
                duckdb::params![name, ts_ns, resource_hash.as_slice()],
            )
            .expect("insert metric");
    }

    fn now_ns() -> i64 {
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    }

    #[test]
    fn validate_args_rejects_zero_limit() {
        let r = validate_args(0);
        assert!(matches!(
            r,
            Err(Error::InvalidArgument { ref field, .. }) if field == "limit"
        ));
    }

    #[test]
    fn validate_args_rejects_above_max() {
        let r = validate_args(LIMIT_MAX + 1);
        assert!(matches!(
            r,
            Err(Error::InvalidArgument { ref field, .. }) if field == "limit"
        ));
    }

    #[test]
    fn validate_args_accepts_in_range() {
        validate_args(1).expect("min");
        validate_args(LIMIT_DEFAULT).expect("default");
        validate_args(LIMIT_MAX).expect("max");
    }

    #[test]
    fn cursor_round_trip_preserves_value() {
        let encoded = encode_cursor(1_700_000_000_123_456_789);
        let decoded = decode_cursor(&encoded).expect("decode");
        assert_eq!(decoded, 1_700_000_000_123_456_789);
    }

    #[test]
    fn decode_cursor_rejects_unprefixed_input() {
        let r = decode_cursor("' OR 1=1 --");
        assert!(matches!(
            r,
            Err(Error::InvalidArgument { ref field, .. }) if field == "cursor"
        ));
    }

    #[test]
    fn decode_cursor_rejects_garbage_after_prefix() {
        let r = decode_cursor("c1:not-a-number");
        assert!(matches!(r, Err(Error::InvalidArgument { .. })));
    }

    #[test]
    fn query_traces_returns_seeded_rows_in_descending_ts_order() {
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        seed_span(&conn, 1, 1, now - 1_000_000);
        seed_span(&conn, 2, 2, now - 2_000_000);
        seed_span(&conn, 3, 3, now - 3_000_000);

        let state = VizState::new();
        let resp = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        )
        .expect("query");

        assert_eq!(resp.items.len(), 3);
        assert_eq!(resp.total, 3);
        assert_eq!(resp.next_cursor, None);
        assert!(resp.items[0].ts_unix_nano > resp.items[1].ts_unix_nano);
        assert!(resp.items[1].ts_unix_nano > resp.items[2].ts_unix_nano);
    }

    #[test]
    fn query_traces_populates_service_duration_error_count() {
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        // 5ms span, status_code=2 (Error per OTLP)
        seed_span_full(
            &conn,
            1,
            1,
            now - 1_000_000,
            "svc-a",
            now - 1_000_000 + 5_000_000,
            2,
        );
        // 0ms span, status_code=1 (Ok per OTLP)
        seed_span_full(&conn, 2, 2, now - 2_000_000, "svc-b", now - 2_000_000, 1);

        let state = VizState::new();
        let resp = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        )
        .expect("query");
        assert_eq!(resp.items.len(), 2);
        // Newest first (svc-a at -1ms older offset).
        assert_eq!(resp.items[0].service, "svc-a");
        assert_eq!(resp.items[0].duration_ms, 5);
        assert_eq!(resp.items[0].error_count, 1);
        assert_eq!(resp.items[1].service, "svc-b");
        assert_eq!(resp.items[1].duration_ms, 0);
        assert_eq!(resp.items[1].error_count, 0);
    }

    #[test]
    fn query_traces_clamps_negative_duration_to_zero() {
        // Defensive: end_time before start (malformed OTLP); duration_ms must
        // not panic or overflow.
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        seed_span_full(&conn, 4, 4, now - 1_000_000, "svc-c", now - 5_000_000, 0);
        let state = VizState::new();
        let resp = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        )
        .expect("query");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].duration_ms, 0);
    }

    #[test]
    fn trace_row_round_trips_through_serde_with_new_fields() {
        let row = TraceRow {
            trace_id: "deadbeef".to_string(),
            span_id: "cafebabe".to_string(),
            ts_unix_nano: 1_700_000_000_000_000_000,
            service: "svc-x".to_string(),
            duration_ms: 42,
            error_count: 1,
        };
        let s = serde_json::to_string(&row).expect("serialize");
        let parsed: TraceRow = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(parsed.service, "svc-x");
        assert_eq!(parsed.duration_ms, 42);
        assert_eq!(parsed.error_count, 1);
    }

    #[test]
    fn query_traces_returns_empty_envelope_when_no_data_in_window() {
        let conn = open_in_memory_with_schema();
        let state = VizState::new();
        let resp = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        )
        .expect("query");
        assert!(resp.items.is_empty());
        assert_eq!(resp.total, 0);
        assert_eq!(resp.next_cursor, None);
    }

    #[test]
    fn query_traces_records_query_latency_into_state() {
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        seed_span(&conn, 1, 1, now - 1_000_000);
        let state = VizState::new();
        let _ = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        )
        .expect("query");
        assert_eq!(state.snapshot().query_count, 1);
    }

    #[test]
    fn query_traces_returns_invalid_argument_on_zero_limit() {
        let conn = open_in_memory_with_schema();
        let state = VizState::new();
        let r = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 0,
                cursor: None,
            },
        );
        assert!(matches!(
            r,
            Err(Error::InvalidArgument { ref field, .. }) if field == "limit"
        ));
    }

    #[test]
    fn query_traces_sql_injection_via_cursor_is_rejected_safely() {
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        seed_span(&conn, 1, 1, now - 1_000_000);
        let state = VizState::new();
        let r = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: Some("' OR 1=1 --".to_string()),
            },
        );
        assert!(matches!(
            r,
            Err(VizError::InvalidArgument { ref field, .. }) if field == "cursor"
        ));

        let guard = conn.lock().expect("lock");
        let count: i64 = guard
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count");
        assert_eq!(count, 1, "spans table must remain intact after attack");
    }

    #[test]
    fn query_traces_pagination_emits_next_cursor_when_full_page_returned() {
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        for i in 0..5u8 {
            seed_span(&conn, i, i, now - (i as i64 + 1) * 1_000_000);
        }
        let state = VizState::new();
        let resp = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 3,
                cursor: None,
            },
        )
        .expect("query");

        assert_eq!(resp.items.len(), 3);
        assert!(resp.next_cursor.is_some());
        let cursor = resp.next_cursor.as_ref().unwrap();
        assert!(cursor.starts_with("c1:"));
    }

    #[test]
    fn query_metrics_returns_seeded_rows() {
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        seed_metric(&conn, "cpu", now - 1_000_000);
        seed_metric(&conn, "memory", now - 2_000_000);
        let state = VizState::new();
        let resp = query_metrics(
            &conn,
            &state,
            &MetricsQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        )
        .expect("query");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.total, 2);
    }

    #[test]
    fn query_logs_returns_seeded_rows() {
        let conn = open_in_memory_with_schema();
        let now = now_ns();
        seed_log(&conn, now - 1_000_000, 9);
        seed_log(&conn, now - 2_000_000, 17);
        let state = VizState::new();
        let resp = query_logs(
            &conn,
            &state,
            &LogsQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        )
        .expect("query");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.total, 2);
    }

    #[test]
    fn query_traces_returns_connection_lost_on_poisoned_mutex() {
        let conn = open_in_memory_with_schema();
        let conn_clone = Arc::clone(&conn);
        let _ = std::thread::spawn(move || {
            let _guard = conn_clone.lock().unwrap();
            panic!("poison");
        })
        .join();

        let state = VizState::new();
        let r = query_traces(
            &conn,
            &state,
            &TracesQueryArgs {
                time_window_seconds: 60,
                limit: 10,
                cursor: None,
            },
        );
        assert!(matches!(r, Err(Error::ConnectionLost)));
    }

    #[test]
    fn hex_encode_round_trips_known_bytes() {
        assert_eq!(hex_encode(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
        assert_eq!(hex_encode(&[0; 16]), "00000000000000000000000000000000");
    }

    #[test]
    fn compute_window_with_cursor_inside_window_uses_cursor_as_end() {
        // Use a cursor near now() so it falls inside the 60s window. Pre-2020
        // cursors get clamped because they're outside the window — that
        // behavior is verified separately by the clamp-test below.
        let now = now_ns();
        let cursor_ts = now - 30_000_000_000; // 30s ago
        let cursor = encode_cursor(cursor_ts);
        let (start, end) = compute_window(60, Some(&cursor)).expect("compute");
        assert_eq!(end, cursor_ts);
        assert!(start <= end, "start {start} must be <= end {end}");
    }

    #[test]
    fn compute_window_with_cursor_outside_window_clamps_end_to_start() {
        // Cursor predating the window (2017) clamps end to start so we never
        // query rows before the active window.
        let cursor = encode_cursor(1_500_000_000_000_000_000);
        let (start, end) = compute_window(60, Some(&cursor)).expect("compute");
        assert_eq!(start, end, "out-of-window cursor must clamp end to start");
    }

    #[test]
    fn compute_window_without_cursor_uses_max_as_end() {
        let (_start, end) = compute_window(60, None).expect("compute");
        assert_eq!(end, i64::MAX);
    }
}
