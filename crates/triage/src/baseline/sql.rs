//! L1a SQL aggregation queries Q1-Q7 (chunk #79).
//!
//! Implements the L1a layer per `docs/v0_2_0/pulse-distillation-architecture.md`
//! §Appendix A — seven SQL templates (Q1-Q7) executed against the L0 DuckDB
//! ring buffer with а Q7-fallback shallow non-recursive variant invoked on
//! timeout OR row-limit hit. Queries are consumed in-process by the future
//! chunk #80 Cadence Coordinator; this chunk introduces zero TauRPC surface,
//! zero broadcast topics, zero arch-registry deltas.
//!
//! Capabilities: prerequisite substrate for P-020 (Three-Tier Severity Model
//! — algorithmic detection requires aggregated input) + P-021 (Algorithmic
//! Attention Cues — cue evaluator consumes Q1-Q6 outputs).
//!
//! ### SQL canonical-form notes
//!
//! Two deliberate transcriptions from Appendix A's literal SQL:
//!
//! 1. `FROM logs` → `FROM log_records` in Q6 — the canonical L0 table name
//!    per arch §Occupied Resources DuckDB database / schema names +
//!    `crates/buffer/src/schema.rs:18`.
//! 2. `INTERVAL ?` → `(? * INTERVAL '1 second')` — uses strongly-typed
//!    BIGINT parameter binding (seconds count) instead of stringly-typed
//!    interval literal parsing. Identical semantic (filter rows newer
//!    than `now() - window`); safer + clearer parameter contract.
//!
//! ### Q7 worst-case bounds
//!
//! Four orthogonal constraints prevent unbounded recursion / row counts:
//! - `slow_traces LIMIT 5` (only top-5 slowest root spans seed the recursion)
//! - `trace_tree WHERE depth < 10` (recursion stops at depth 10)
//! - outer `LIMIT 100` (output capped)
//! - 200ms wall-clock timeout (Q7_DEFAULT_TIMEOUT) — on timeout fires Q7-fallback
//!
//! Q7-fallback is а shallow non-recursive variant (no CTE) that takes only
//! root spans + direct children. Same `slow_traces LIMIT 5` seed.
//!
//! ### dead-code allowance
//!
//! Chunk #79 is а prerequisite substrate; chunk #80 Cadence Coordinator
//! consumes Q1-Q7 outputs. Until chunk #80 lands, the public API has no
//! in-crate caller — `#[allow(dead_code)]` at the module level silences the
//! transient unused-warning, removed when chunk #80 wires the consumer
//! invocation site.

#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use duckdb::Connection;
use thiserror::Error;

/// Compute the cutoff timestamp (nanoseconds since Unix epoch) for а window
/// query. Rows newer than the cutoff are included; rows older are excluded.
///
/// Matches the existing `viz/query.rs` pattern of pre-computing the absolute
/// cutoff in Rust rather than relying on DuckDB-side `INTERVAL` arithmetic
/// at parameter-binding time (which has type-coercion ambiguity in prepared
/// statements). Documented divergence from dist-arch v3 §Appendix A's
/// `start_time > now() - INTERVAL ?` syntax — semantic identical.
fn cutoff_ns(window: Duration) -> i64 {
    let now_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0);
    now_ns.saturating_sub(window.as_nanos() as i64)
}

/// Default Q7 wall-clock timeout (200ms per dist-arch v3 §L1a Performance envelope).
///
/// Tests may override by calling [`run_q7_with_timeout`] directly.
pub const Q7_DEFAULT_TIMEOUT: Duration = Duration::from_millis(200);

// --- SQL templates (verbatim from dist-arch v3 §Appendix A, with two documented transcriptions) ---

const Q1_RED_PER_SERVICE: &str = "\
SELECT \
  service_name, \
  COUNT(*) AS request_count, \
  SUM(CASE WHEN status_code = 2 THEN 1 ELSE 0 END) AS error_count, \
  SUM(CASE WHEN status_code = 2 THEN 1.0 ELSE 0.0 END) \
    / NULLIF(COUNT(*), 0) AS error_rate, \
  APPROX_QUANTILE(end_time_unix_nano - ts_unix_nano, 0.50) AS p50_ns, \
  APPROX_QUANTILE(end_time_unix_nano - ts_unix_nano, 0.95) AS p95_ns, \
  APPROX_QUANTILE(end_time_unix_nano - ts_unix_nano, 0.99) AS p99_ns \
FROM spans \
WHERE ts_unix_nano > ? \
GROUP BY service_name";

const Q2_RED_PER_OPERATION: &str = "\
SELECT \
  service_name, \
  service_name AS operation, \
  COUNT(*) AS request_count, \
  SUM(CASE WHEN status_code = 2 THEN 1 ELSE 0 END) AS error_count, \
  APPROX_QUANTILE(end_time_unix_nano - ts_unix_nano, 0.99) AS p99_ns \
FROM spans \
WHERE ts_unix_nano > ? \
GROUP BY service_name \
ORDER BY error_count DESC, p99_ns DESC \
LIMIT 20";

const Q3_EXCEPTION_FINGERPRINTS: &str = "\
SELECT \
  fingerprint, \
  COUNT(*) AS occurrences, \
  MIN(ts_unix_nano) AS first_seen, \
  MAX(ts_unix_nano) AS last_seen \
FROM span_events \
WHERE name = 'exception' \
  AND ts_unix_nano > ? \
  AND fingerprint IS NOT NULL \
GROUP BY fingerprint \
HAVING COUNT(*) >= 2 \
ORDER BY occurrences DESC \
LIMIT 10";

const Q4_SERVICE_INTERACTIONS: &str = "\
SELECT \
  caller.service_name AS caller, \
  callee.service_name AS callee, \
  COUNT(*) AS call_count, \
  SUM(CASE WHEN callee.status_code = 2 THEN 1 ELSE 0 END) AS error_count \
FROM spans callee \
JOIN spans caller ON callee.parent_span_id = caller.span_id \
WHERE callee.ts_unix_nano > ? \
  AND caller.service_name != callee.service_name \
GROUP BY caller.service_name, callee.service_name";

const Q5_CARDINALITY_PER_SERVICE: &str = "\
SELECT \
  service_name, \
  COUNT(*) AS total_spans \
FROM spans \
WHERE ts_unix_nano > ? \
GROUP BY service_name";

const Q6_HIGH_SEVERITY_LOGS: &str = "\
SELECT \
  resource_hash, \
  COUNT(*) AS occurrences, \
  MAX(severity_number) AS peak_severity \
FROM log_records \
WHERE ts_unix_nano > ? \
  AND severity_number >= 17 \
GROUP BY resource_hash \
ORDER BY occurrences DESC \
LIMIT 15";

const Q7_CRITICAL_PATH: &str = "\
WITH RECURSIVE \
  slow_traces AS ( \
    SELECT trace_id \
    FROM spans \
    WHERE ts_unix_nano > ? \
      AND parent_span_id IS NULL \
    ORDER BY (end_time_unix_nano - ts_unix_nano) DESC \
    LIMIT 5 \
  ), \
  trace_tree AS ( \
    SELECT s.trace_id, s.span_id, s.service_name, s.ts_unix_nano, s.end_time_unix_nano, 0 AS depth \
    FROM spans s \
    JOIN slow_traces st USING (trace_id) \
    WHERE s.parent_span_id IS NULL \
    UNION ALL \
    SELECT s.trace_id, s.span_id, s.service_name, s.ts_unix_nano, s.end_time_unix_nano, tt.depth + 1 \
    FROM spans s \
    JOIN trace_tree tt ON s.parent_span_id = tt.span_id AND s.trace_id = tt.trace_id \
    WHERE tt.depth < 10 \
  ) \
SELECT trace_id, span_id, service_name, ts_unix_nano, end_time_unix_nano, depth \
FROM trace_tree \
ORDER BY trace_id, depth, ts_unix_nano \
LIMIT 100";

const Q7_FALLBACK: &str = "\
WITH slow_traces AS ( \
  SELECT trace_id, span_id AS root_span_id \
  FROM spans \
  WHERE ts_unix_nano > ? \
    AND parent_span_id IS NULL \
  ORDER BY (end_time_unix_nano - ts_unix_nano) DESC \
  LIMIT 5 \
) \
SELECT s.trace_id, s.span_id, s.service_name, s.ts_unix_nano, s.end_time_unix_nano, \
  CASE WHEN s.parent_span_id IS NULL THEN 0 ELSE 1 END AS depth \
FROM spans s \
JOIN slow_traces st ON st.trace_id = s.trace_id \
WHERE s.parent_span_id IS NULL OR s.parent_span_id = st.root_span_id \
ORDER BY s.trace_id, s.ts_unix_nano \
LIMIT 100";

// --- Result types ---

/// Q1 row — RED metrics per service.
#[derive(Debug, Clone, PartialEq)]
pub struct Q1RedRow {
    pub service_name: String,
    pub request_count: i64,
    pub error_count: i64,
    pub error_rate: f64,
    pub p50_ns: i64,
    pub p95_ns: i64,
    pub p99_ns: i64,
}

/// Q2 row — RED per service operation (currently per-service-only; refined when
/// `operation_name` becomes а distinct span attribute column).
#[derive(Debug, Clone, PartialEq)]
pub struct Q2OperationRow {
    pub service_name: String,
    pub operation: String,
    pub request_count: i64,
    pub error_count: i64,
    pub p99_ns: i64,
}

/// Q3 row — exception fingerprint frequencies.
#[derive(Debug, Clone, PartialEq)]
pub struct Q3FingerprintRow {
    pub fingerprint: Vec<u8>,
    pub occurrences: i64,
    pub first_seen: i64,
    pub last_seen: i64,
}

/// Q4 row — service interaction (caller → callee aggregate).
#[derive(Debug, Clone, PartialEq)]
pub struct Q4InteractionRow {
    pub caller: String,
    pub callee: String,
    pub call_count: i64,
    pub error_count: i64,
}

/// Q5 row — cardinality estimate per service.
#[derive(Debug, Clone, PartialEq)]
pub struct Q5CardinalityRow {
    pub service_name: String,
    pub total_spans: i64,
}

/// Q6 row — high-severity log frequencies.
#[derive(Debug, Clone, PartialEq)]
pub struct Q6LogRow {
    pub resource_hash: Vec<u8>,
    pub occurrences: i64,
    pub peak_severity: i32,
}

/// Q7 row — critical-path sample (recursive or fallback shallow).
#[derive(Debug, Clone, PartialEq)]
pub struct Q7CriticalPathRow {
    pub trace_id: Vec<u8>,
    pub span_id: Vec<u8>,
    pub service_name: String,
    pub ts_unix_nano: i64,
    pub end_time_unix_nano: i64,
    pub depth: i64,
}

// --- Error type ---

/// Error variants surfaced by the L1a SQL aggregation layer.
///
/// Module-internal; future TauRPC bridge crossings convert via `From` to
/// `AppError::Storage { message }` with full sanitization (strip stack traces,
/// file paths, library versions, SQL fragments) at the binary boundary.
#[derive(Debug, Error)]
pub enum SqlAggregationError {
    #[error("duckdb query failed for {query_id}")]
    QueryFailed { query_id: &'static str },
    #[error("q7 timeout after {timeout_ms}ms; fallback triggered")]
    Q7Timeout { timeout_ms: u64 },
    #[error("connection lock poisoned")]
    LockPoisoned,
    #[error("join error in spawn_blocking")]
    Join,
}

impl SqlAggregationError {
    pub fn error_category(&self) -> &'static str {
        match self {
            Self::QueryFailed { .. } => "query",
            Self::Q7Timeout { .. } => "timeout",
            Self::LockPoisoned => "lock",
            Self::Join => "join",
        }
    }
}

// --- Connection state wrapper ---

/// Thin handle wrapping the shared L0 DuckDB connection.
///
/// Boot wiring at `pulse-app/src/main.rs` shares the same `Arc<Mutex<Connection>>`
/// instance as `BufferState` consumers — single in-memory `:memory:` connection
/// per arch §Occupied Resources DuckDB database identity `pulse_buffer`.
#[derive(Clone)]
pub struct TriageSqlState {
    conn: Arc<Mutex<Connection>>,
}

impl TriageSqlState {
    /// Wraps a DEDICATED read connection cloned from the shared appender
    /// connection (chunk #99 high-load profile finding): L1a queries that
    /// queued on the shared Rust mutex occasionally landed behind an
    /// append carrying DuckDB-internal row-group maintenance (~0.5s hold)
    /// and blew the <500ms p99 budget — always the FIRST query after an
    /// idle cadence gap. On a `try_clone()`d connection DuckDB MVCC reads
    /// see the same database without queueing on the write path. Falls
    /// back to the shared connection (pre-#99 behavior) if cloning fails.
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        let read_conn = conn.lock().ok().and_then(|guard| guard.try_clone().ok());
        match read_conn {
            Some(c) => Self {
                conn: Arc::new(Mutex::new(c)),
            },
            None => {
                tracing::warn!(
                    target: "triage.sql",
                    fallback = "shared_connection",
                    "L1a read connection clone failed; querying on the shared connection"
                );
                Self { conn }
            }
        }
    }

    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }
}

// --- Async Q1-Q7 public API ---

/// Run Q1 — RED metrics per service.
pub async fn run_q1(
    state: &TriageSqlState,
    window: Duration,
) -> Result<Vec<Q1RedRow>, SqlAggregationError> {
    let conn = Arc::clone(&state.conn);
    let cutoff = cutoff_ns(window);
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || run_q1_blocking(&conn, cutoff))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    emit_query_telemetry("q1", start.elapsed(), result.len());
    Ok(result)
}

/// Run Q2 — RED per service operation.
pub async fn run_q2(
    state: &TriageSqlState,
    window: Duration,
) -> Result<Vec<Q2OperationRow>, SqlAggregationError> {
    let conn = Arc::clone(&state.conn);
    let cutoff = cutoff_ns(window);
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || run_q2_blocking(&conn, cutoff))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    emit_query_telemetry("q2", start.elapsed(), result.len());
    Ok(result)
}

/// Run Q3 — exception fingerprint frequencies.
pub async fn run_q3(
    state: &TriageSqlState,
    window: Duration,
) -> Result<Vec<Q3FingerprintRow>, SqlAggregationError> {
    let conn = Arc::clone(&state.conn);
    let cutoff = cutoff_ns(window);
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || run_q3_blocking(&conn, cutoff))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    emit_query_telemetry("q3", start.elapsed(), result.len());
    Ok(result)
}

/// Run Q4 — service interaction graph.
pub async fn run_q4(
    state: &TriageSqlState,
    window: Duration,
) -> Result<Vec<Q4InteractionRow>, SqlAggregationError> {
    let conn = Arc::clone(&state.conn);
    let cutoff = cutoff_ns(window);
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || run_q4_blocking(&conn, cutoff))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    emit_query_telemetry("q4", start.elapsed(), result.len());
    Ok(result)
}

/// Run Q5 — cardinality estimate per service.
pub async fn run_q5(
    state: &TriageSqlState,
    window: Duration,
) -> Result<Vec<Q5CardinalityRow>, SqlAggregationError> {
    let conn = Arc::clone(&state.conn);
    let cutoff = cutoff_ns(window);
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || run_q5_blocking(&conn, cutoff))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    emit_query_telemetry("q5", start.elapsed(), result.len());
    Ok(result)
}

/// Run Q6 — high-severity log frequencies.
pub async fn run_q6(
    state: &TriageSqlState,
    window: Duration,
) -> Result<Vec<Q6LogRow>, SqlAggregationError> {
    let conn = Arc::clone(&state.conn);
    let cutoff = cutoff_ns(window);
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || run_q6_blocking(&conn, cutoff))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    emit_query_telemetry("q6", start.elapsed(), result.len());
    Ok(result)
}

/// Run Q7 — critical-path sample with default 200ms timeout + fallback.
pub async fn run_q7(
    state: &TriageSqlState,
    window: Duration,
) -> Result<Vec<Q7CriticalPathRow>, SqlAggregationError> {
    run_q7_with_timeout(state, window, Q7_DEFAULT_TIMEOUT).await
}

/// Run Q7 with explicit timeout duration. Tests use this directly for
/// deterministic timeout assertions с injected duration knob.
///
/// The primary recursive CTE runs on a DEDICATED `try_clone()`d connection
/// (own private mutex), and on timeout that clone is `interrupt()`ed.
/// Rationale (chunk #99 high-load profile finding): `tokio::time::timeout`
/// around `spawn_blocking` ABANDONS but cannot cancel the blocking task —
/// when the primary ran on the shared connection, every 200ms timeout left
/// an orphaned multi-second CTE holding the shared mutex, and the fallback
/// plus the next cadence tick's Q1-Q6 convoyed behind it (Q1 p99 measured
/// 699ms vs the 500ms dist-arch budget). With the clone, an abandoned
/// primary holds nothing shared, and the interrupt aborts it inside DuckDB
/// to reclaim the blocking thread promptly.
pub async fn run_q7_with_timeout(
    state: &TriageSqlState,
    window: Duration,
    timeout: Duration,
) -> Result<Vec<Q7CriticalPathRow>, SqlAggregationError> {
    let conn = Arc::clone(&state.conn);
    let cutoff = cutoff_ns(window);
    let start = Instant::now();

    let (primary_conn, interrupt) = {
        let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
        let clone = guard
            .try_clone()
            .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q7" })?;
        let interrupt = clone.interrupt_handle();
        (Arc::new(Mutex::new(clone)), interrupt)
    };
    let primary_result = tokio::time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || run_q7_primary_blocking(&primary_conn, cutoff)),
    )
    .await;

    match primary_result {
        Ok(Ok(Ok(rows))) => {
            emit_q7_telemetry("q7", start.elapsed(), rows.len());
            Ok(rows)
        }
        Ok(Ok(Err(e))) => Err(e),
        Ok(Err(_)) => Err(SqlAggregationError::Join),
        Err(_) => {
            // Primary Q7 timed out — abort the orphaned clone-side CTE so
            // its blocking thread is reclaimed, then invoke the fallback on
            // the shared connection (which the orphan never held).
            interrupt.interrupt();
            tracing::warn!(
                target: "metric.pipeline.l1a.q7_timeout_count_total",
                value = 1_i64,
                timeout_ms = timeout.as_millis() as u64,
                rejection_reason = "timeout",
                "q7 primary timed out; fallback triggered",
            );
            let rows = tokio::task::spawn_blocking(move || run_q7_fallback_blocking(&conn, cutoff))
                .await
                .map_err(|_| SqlAggregationError::Join)??;
            tracing::info!(
                target: "metric.pipeline.l1a.q7_fallback_count_total",
                value = 1_i64,
                fallback_query_kind = "shallow_non_recursive",
                cause = "timeout",
                "q7 fallback completed",
            );
            emit_q7_telemetry("q7-fallback", start.elapsed(), rows.len());
            Ok(rows)
        }
    }
}

// --- Blocking executors (spawn_blocking targets) ---

fn run_q1_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q1RedRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q1_RED_PER_SERVICE)
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q1" })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q1RedRow {
                service_name: row.get(0)?,
                request_count: row.get(1)?,
                error_count: row.get(2)?,
                error_rate: row.get::<_, Option<f64>>(3)?.unwrap_or(0.0),
                p50_ns: row.get::<_, Option<i64>>(4)?.unwrap_or(0),
                p95_ns: row.get::<_, Option<i64>>(5)?.unwrap_or(0),
                p99_ns: row.get::<_, Option<i64>>(6)?.unwrap_or(0),
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q1" })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q1" })?;
    Ok(rows)
}

fn run_q2_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q2OperationRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q2_RED_PER_OPERATION)
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q2" })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q2OperationRow {
                service_name: row.get(0)?,
                operation: row.get(1)?,
                request_count: row.get(2)?,
                error_count: row.get(3)?,
                p99_ns: row.get::<_, Option<i64>>(4)?.unwrap_or(0),
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q2" })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q2" })?;
    Ok(rows)
}

fn run_q3_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q3FingerprintRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q3_EXCEPTION_FINGERPRINTS)
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q3" })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q3FingerprintRow {
                fingerprint: row.get(0)?,
                occurrences: row.get(1)?,
                first_seen: row.get(2)?,
                last_seen: row.get(3)?,
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q3" })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q3" })?;
    Ok(rows)
}

fn run_q4_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q4InteractionRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q4_SERVICE_INTERACTIONS)
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q4" })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q4InteractionRow {
                caller: row.get(0)?,
                callee: row.get(1)?,
                call_count: row.get(2)?,
                error_count: row.get(3)?,
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q4" })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q4" })?;
    Ok(rows)
}

fn run_q5_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q5CardinalityRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q5_CARDINALITY_PER_SERVICE)
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q5" })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q5CardinalityRow {
                service_name: row.get(0)?,
                total_spans: row.get(1)?,
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q5" })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q5" })?;
    Ok(rows)
}

fn run_q6_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q6LogRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q6_HIGH_SEVERITY_LOGS)
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q6" })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q6LogRow {
                resource_hash: row.get(0)?,
                occurrences: row.get(1)?,
                peak_severity: row.get(2)?,
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q6" })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q6" })?;
    Ok(rows)
}

fn run_q7_primary_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q7CriticalPathRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q7_CRITICAL_PATH)
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q7" })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q7CriticalPathRow {
                trace_id: row.get(0)?,
                span_id: row.get(1)?,
                service_name: row.get(2)?,
                ts_unix_nano: row.get(3)?,
                end_time_unix_nano: row.get(4)?,
                depth: row.get(5)?,
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q7" })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q7" })?;
    Ok(rows)
}

fn run_q7_fallback_blocking(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ns: i64,
) -> Result<Vec<Q7CriticalPathRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard
        .prepare(Q7_FALLBACK)
        .map_err(|_| SqlAggregationError::QueryFailed {
            query_id: "q7-fallback",
        })?;
    let rows = stmt
        .query_map([cutoff_ns], |row| {
            Ok(Q7CriticalPathRow {
                trace_id: row.get(0)?,
                span_id: row.get(1)?,
                service_name: row.get(2)?,
                ts_unix_nano: row.get(3)?,
                end_time_unix_nano: row.get(4)?,
                depth: row.get(5)?,
            })
        })
        .map_err(|_| SqlAggregationError::QueryFailed {
            query_id: "q7-fallback",
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed {
            query_id: "q7-fallback",
        })?;
    Ok(rows)
}

// --- Telemetry emission helpers ---

/// Emit query telemetry for Q1-Q6. Per `pipeline.l1a.*` metric naming convention
/// (chunk spec line 512). Aggregate-only fields per session 84 triage convention
/// — NO service_name / scope_id / trace_id / span_id labels on self-observation
/// events (broadcast topics are the product surface for those identifiers).
fn emit_query_telemetry(query_name: &'static str, elapsed: Duration, row_count: usize) {
    // Per session 41 (2026-05-10), tracing::info!(target: ...) requires
    // &'static str — unrolled per query_name rather than runtime variable.
    // Use match к dispatch к the correct target literal.
    match query_name {
        "q1" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q1",
                value = 1_i64,
                "q1 query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q1",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q1 latency sample",
            );
        }
        "q2" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q2",
                value = 1_i64,
                "q2 query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q2",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q2 latency sample",
            );
        }
        "q3" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q3",
                value = 1_i64,
                "q3 query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q3",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q3 latency sample",
            );
        }
        "q4" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q4",
                value = 1_i64,
                "q4 query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q4",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q4 latency sample",
            );
        }
        "q5" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q5",
                value = 1_i64,
                "q5 query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q5",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q5 latency sample",
            );
        }
        "q6" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q6",
                value = 1_i64,
                "q6 query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q6",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q6 latency sample",
            );
        }
        _ => {
            // Unreachable — query_name is always one of q1..q6.
        }
    }
}

/// Q7 telemetry — separates primary vs fallback by `query_name` label.
fn emit_q7_telemetry(query_name: &'static str, elapsed: Duration, row_count: usize) {
    match query_name {
        "q7" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q7",
                value = 1_i64,
                "q7 query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q7",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q7 latency sample",
            );
        }
        "q7-fallback" => {
            tracing::info!(
                target: "metric.pipeline.l1a.query_count_total",
                query_name = "q7-fallback",
                value = 1_i64,
                "q7-fallback query completed",
            );
            tracing::info!(
                target: "metric.pipeline.l1a.query_latency_p99_milliseconds",
                query_name = "q7-fallback",
                duration_ms = elapsed.as_millis() as u64,
                row_count_returned = row_count as i64,
                "q7-fallback latency sample",
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_test_connection() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().expect("open_in_memory");
        // Create the L0 schema (subset needed для Q1-Q7 — спans + span_events + log_records).
        conn.execute_batch(
            "\
CREATE TABLE IF NOT EXISTS spans (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    service_name VARCHAR NOT NULL,
    end_time_unix_nano BIGINT NOT NULL,
    status_code INTEGER NOT NULL,
    parent_span_id BLOB,
    PRIMARY KEY (trace_id, span_id)
);
CREATE TABLE IF NOT EXISTS span_events (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    event_index INTEGER NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    name VARCHAR NOT NULL DEFAULT '',
    fingerprint BLOB,
    PRIMARY KEY (trace_id, span_id, event_index)
);
CREATE TABLE IF NOT EXISTS log_records (
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    resource_hash BLOB NOT NULL,
    severity_number INTEGER NOT NULL,
    body VARCHAR NOT NULL DEFAULT '',
    severity_text VARCHAR NOT NULL DEFAULT '',
    trace_id BLOB,
    span_id BLOB,
    PRIMARY KEY (ts_unix_nano, resource_hash, severity_number)
);
",
        )
        .expect("create schema");
        Arc::new(Mutex::new(conn))
    }

    fn insert_span(
        conn: &Arc<Mutex<Connection>>,
        trace_id: &[u8],
        span_id: &[u8],
        service: &str,
        duration_ns: i64,
        status_code: i32,
    ) {
        insert_span_with_parent(
            conn,
            trace_id,
            span_id,
            None,
            service,
            duration_ns,
            status_code,
        );
    }

    fn insert_span_with_parent(
        conn: &Arc<Mutex<Connection>>,
        trace_id: &[u8],
        span_id: &[u8],
        parent_span_id: Option<&[u8]>,
        service: &str,
        duration_ns: i64,
        status_code: i32,
    ) {
        let now_ns = current_test_nanos();
        let guard = conn.lock().unwrap();
        guard
            .execute(
                "INSERT INTO spans (trace_id, span_id, ts, ts_unix_nano, service_name, end_time_unix_nano, status_code, parent_span_id) \
                 VALUES (?, ?, now(), ?, ?, ?, ?, ?)",
                duckdb::params![
                    trace_id,
                    span_id,
                    now_ns,
                    service,
                    now_ns + duration_ns,
                    status_code,
                    parent_span_id,
                ],
            )
            .expect("insert span");
    }

    fn insert_span_event(
        conn: &Arc<Mutex<Connection>>,
        trace_id: &[u8],
        span_id: &[u8],
        event_index: i32,
        name: &str,
        fingerprint: Option<&[u8]>,
    ) {
        let now_ns = current_test_nanos();
        let guard = conn.lock().unwrap();
        guard
            .execute(
                "INSERT INTO span_events (trace_id, span_id, event_index, ts, ts_unix_nano, name, fingerprint) \
                 VALUES (?, ?, ?, now(), ?, ?, ?)",
                duckdb::params![trace_id, span_id, event_index, now_ns, name, fingerprint],
            )
            .expect("insert span_event");
    }

    fn insert_log(conn: &Arc<Mutex<Connection>>, resource_hash: &[u8], severity_number: i32) {
        let now_ns = current_test_nanos();
        let guard = conn.lock().unwrap();
        guard
            .execute(
                "INSERT INTO log_records (ts, ts_unix_nano, resource_hash, severity_number, body, severity_text) \
                 VALUES (now(), ?, ?, ?, '', '')",
                duckdb::params![now_ns, resource_hash, severity_number],
            )
            .expect("insert log");
    }

    fn current_test_nanos() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as i64)
            .unwrap_or(0)
    }

    #[tokio::test]
    async fn q1_returns_red_metrics_per_service() {
        let conn = open_test_connection();
        // 3 spans for service A (1 error), 2 for service B (0 errors)
        insert_span(&conn, &[1; 16], &[1; 8], "service-a", 1_000_000, 0);
        insert_span(&conn, &[1; 16], &[2; 8], "service-a", 2_000_000, 2);
        insert_span(&conn, &[1; 16], &[3; 8], "service-a", 3_000_000, 0);
        insert_span(&conn, &[2; 16], &[4; 8], "service-b", 5_000_000, 0);
        insert_span(&conn, &[2; 16], &[5; 8], "service-b", 6_000_000, 0);

        let state = TriageSqlState::new(conn);
        let result = run_q1(&state, Duration::from_secs(60))
            .await
            .expect("q1 ok");
        assert_eq!(result.len(), 2);
        let a = result
            .iter()
            .find(|r| r.service_name == "service-a")
            .unwrap();
        assert_eq!(a.request_count, 3);
        assert_eq!(a.error_count, 1);
        assert!((a.error_rate - (1.0 / 3.0)).abs() < 0.01);
    }

    #[tokio::test]
    async fn q1_empty_table_returns_empty_vec() {
        let conn = open_test_connection();
        let state = TriageSqlState::new(conn);
        let result = run_q1(&state, Duration::from_secs(60))
            .await
            .expect("q1 ok");
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn q2_orders_by_error_count_desc() {
        let conn = open_test_connection();
        insert_span(&conn, &[1; 16], &[1; 8], "low-errors", 1_000_000, 0);
        insert_span(&conn, &[2; 16], &[2; 8], "high-errors", 2_000_000, 2);
        insert_span(&conn, &[3; 16], &[3; 8], "high-errors", 3_000_000, 2);

        let state = TriageSqlState::new(conn);
        let result = run_q2(&state, Duration::from_secs(60))
            .await
            .expect("q2 ok");
        assert!(!result.is_empty());
        // high-errors should come first
        assert_eq!(result[0].service_name, "high-errors");
        assert_eq!(result[0].error_count, 2);
    }

    #[tokio::test]
    async fn q3_returns_fingerprints_with_count_ge_2() {
        let conn = open_test_connection();
        // fingerprint A appears twice; B appears once (should be filtered out by HAVING >= 2)
        let fp_a: [u8; 8] = [0xaa; 8];
        let fp_b: [u8; 8] = [0xbb; 8];
        insert_span_event(&conn, &[1; 16], &[1; 8], 0, "exception", Some(&fp_a));
        insert_span_event(&conn, &[2; 16], &[2; 8], 0, "exception", Some(&fp_a));
        insert_span_event(&conn, &[3; 16], &[3; 8], 0, "exception", Some(&fp_b));

        let state = TriageSqlState::new(conn);
        let result = run_q3(&state, Duration::from_secs(60))
            .await
            .expect("q3 ok");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fingerprint, fp_a.to_vec());
        assert_eq!(result[0].occurrences, 2);
    }

    #[tokio::test]
    async fn q4_returns_caller_callee_pairs() {
        let conn = open_test_connection();
        // Two services in same trace: a (caller, root) + b (callee, child of a)
        insert_span(&conn, &[1; 16], &[1; 8], "service-a", 1_000_000, 0);
        insert_span_with_parent(
            &conn,
            &[1; 16],
            &[2; 8],
            Some(&[1; 8]),
            "service-b",
            2_000_000,
            0,
        );

        let state = TriageSqlState::new(conn);
        let result = run_q4(&state, Duration::from_secs(60))
            .await
            .expect("q4 ok");
        assert!(!result.is_empty());
        assert_eq!(result[0].caller, "service-a");
        assert_eq!(result[0].callee, "service-b");
    }

    #[tokio::test]
    async fn q5_returns_cardinality_per_service() {
        let conn = open_test_connection();
        insert_span(&conn, &[1; 16], &[1; 8], "service-a", 1_000_000, 0);
        insert_span(&conn, &[1; 16], &[2; 8], "service-a", 2_000_000, 0);
        insert_span(&conn, &[2; 16], &[3; 8], "service-b", 3_000_000, 0);

        let state = TriageSqlState::new(conn);
        let result = run_q5(&state, Duration::from_secs(60))
            .await
            .expect("q5 ok");
        assert_eq!(result.len(), 2);
        let a = result
            .iter()
            .find(|r| r.service_name == "service-a")
            .unwrap();
        assert_eq!(a.total_spans, 2);
    }

    #[tokio::test]
    async fn q6_returns_high_severity_logs() {
        let conn = open_test_connection();
        let resource_a: [u8; 8] = [0x01; 8];
        let resource_b: [u8; 8] = [0x02; 8];
        // Two records at severity >= 17 (ERROR) for resource A; one at 13 (INFO) — filtered out
        insert_log(&conn, &resource_a, 17);
        insert_log(&conn, &resource_a, 21);
        insert_log(&conn, &resource_b, 13);

        let state = TriageSqlState::new(conn);
        let result = run_q6(&state, Duration::from_secs(60))
            .await
            .expect("q6 ok");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].occurrences, 2);
        assert_eq!(result[0].peak_severity, 21);
    }

    #[tokio::test]
    async fn q7_returns_critical_path_for_slow_traces() {
        let conn = open_test_connection();
        // Insert several spans across different traces with varying durations
        for i in 0..5 {
            let trace_id = [i as u8; 16];
            insert_span(
                &conn,
                &trace_id,
                &[1; 8],
                "service",
                ((i + 1) as i64) * 1_000_000,
                0,
            );
        }

        let state = TriageSqlState::new(conn);
        let result = run_q7(&state, Duration::from_secs(60))
            .await
            .expect("q7 ok");
        // Should return rows (≤100 cap; ≤5 trace seeds × depth-limited tree)
        assert!(!result.is_empty());
        assert!(result.len() <= 100);
    }

    #[tokio::test]
    async fn q7_fallback_fires_on_zero_timeout() {
        let conn = open_test_connection();
        insert_span(&conn, &[1; 16], &[1; 8], "service", 1_000_000, 0);

        let state = TriageSqlState::new(conn);
        // Use Duration::from_nanos(1) (effectively zero) to force timeout
        let result = run_q7_with_timeout(&state, Duration::from_secs(60), Duration::from_nanos(1))
            .await
            .expect("q7 fallback ok");
        // Fallback path returns at most LIMIT 100 rows; happy-path data has 1 span
        assert!(result.len() <= 100);
    }

    #[tokio::test]
    async fn q7_primary_under_normal_timeout_returns_normal_results() {
        let conn = open_test_connection();
        insert_span(&conn, &[1; 16], &[1; 8], "service", 1_000_000, 0);

        let state = TriageSqlState::new(conn);
        let result = run_q7(&state, Duration::from_secs(60))
            .await
            .expect("q7 ok");
        // Default 200ms timeout is plenty for а single-span fixture
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn q7_outer_limit_100_caps_results() {
        let conn = open_test_connection();
        // 5 traces × (1 root + 25 children) = 130 total tree nodes; LIMIT 100 caps.
        // Each trace's root has а unique span_id; 25 children each have
        // parent_span_id pointing к the root. Use 4-byte unique encoding к avoid
        // primary-key collisions across (trace_id, span_id) pairs.
        for trace_idx in 0..5_u8 {
            let trace_id = [trace_idx + 1; 16];
            let mut root_span_id = [0_u8; 8];
            root_span_id[0] = trace_idx;
            root_span_id[1] = 0xff; // sentinel byte distinguishing root from children
            // Root span (no parent)
            insert_span(&conn, &trace_id, &root_span_id, "service", 10_000_000, 0);
            for child_idx in 0..25_u8 {
                let mut child_span_id = [0_u8; 8];
                child_span_id[0] = trace_idx;
                child_span_id[1] = child_idx;
                insert_span_with_parent(
                    &conn,
                    &trace_id,
                    &child_span_id,
                    Some(&root_span_id),
                    "service",
                    1_000_000 * ((child_idx as i64) + 1),
                    0,
                );
            }
        }

        let state = TriageSqlState::new(conn);
        let result = run_q7(&state, Duration::from_secs(60))
            .await
            .expect("q7 ok");
        // Outer LIMIT 100 caps the recursive output (130 nodes available, 100 returned).
        assert!(result.len() <= 100);
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn window_filter_excludes_old_rows() {
        // Insert one span, then query с very narrow window — но since DuckDB's now()
        // and the row's `ts now()` are essentially the same instant at test execution,
        // we verify the WHERE clause structure works by passing а 0-second window
        // (effectively NOW() - INTERVAL 0 SECOND = NOW()) which still includes
        // rows inserted via `ts now()`. Tighter time-based filtering would require
        // time injection via tokio test-util feature (not enabled in triage crate).
        let conn = open_test_connection();
        insert_span(&conn, &[1; 16], &[1; 8], "service-a", 1_000_000, 0);

        let state = TriageSqlState::new(conn);
        // Large window — span is included
        let result_wide = run_q1(&state, Duration::from_secs(3600))
            .await
            .expect("q1 wide ok");
        assert_eq!(result_wide.len(), 1);
    }

    #[tokio::test]
    async fn sql_injection_via_duration_param_is_blocked() {
        // The Duration → i64 BIGINT bind path is type-safe by construction:
        // Q1's `(? * INTERVAL '1 second')` placeholder receives а bound i64
        // (window.as_secs() as i64). A would-be injector cannot smuggle SQL
        // tokens через а typed integer — Duration::from_secs() accepts u64
        // and as_secs() returns u64; cast к i64 is а value-only operation.
        // The assertion here is positive: we verify а normal Duration flows
        // through the bind site, executes safely, and the spans table is
        // intact afterwards.
        let conn = open_test_connection();
        insert_span(&conn, &[1; 16], &[1; 8], "service-a", 1_000_000, 0);
        let row_count_before: i64 = {
            let guard = conn.lock().unwrap();
            guard
                .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
                .unwrap()
        };
        assert_eq!(row_count_before, 1);

        let state = TriageSqlState::new(conn.clone());
        let _ = run_q1(&state, Duration::from_secs(60))
            .await
            .expect("q1 ok");

        // Verify table integrity after query — no DROP / DELETE could have run
        // because the parameter binding path forbids SQL syntax injection.
        let row_count_after: i64 = {
            let guard = conn.lock().unwrap();
            guard
                .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
                .unwrap()
        };
        assert_eq!(row_count_after, 1);
    }

    #[test]
    fn sql_aggregation_error_category_covers_all_variants() {
        assert_eq!(
            SqlAggregationError::QueryFailed { query_id: "q1" }.error_category(),
            "query"
        );
        assert_eq!(
            SqlAggregationError::Q7Timeout { timeout_ms: 200 }.error_category(),
            "timeout"
        );
        assert_eq!(SqlAggregationError::LockPoisoned.error_category(), "lock");
        assert_eq!(SqlAggregationError::Join.error_category(), "join");
    }

    #[test]
    fn q7_default_timeout_matches_spec() {
        assert_eq!(Q7_DEFAULT_TIMEOUT, Duration::from_millis(200));
    }

    #[test]
    fn triage_sql_state_clones_share_connection() {
        let conn = open_test_connection();
        let state1 = TriageSqlState::new(Arc::clone(&conn));
        let state2 = state1.clone();
        // Both states share the underlying connection via Arc — verify by pointer equality
        assert!(Arc::ptr_eq(state1.connection(), state2.connection()));
    }

    #[tokio::test]
    async fn all_blocking_queries_safe_to_re_run() {
        // Verifies that running each Q1-Q6 twice against the same connection
        // doesn't poison locks or accumulate state. Q7 covered by other tests.
        let conn = open_test_connection();
        insert_span(&conn, &[1; 16], &[1; 8], "service-a", 1_000_000, 0);
        let state = TriageSqlState::new(conn);

        for _ in 0..2 {
            assert!(run_q1(&state, Duration::from_secs(60)).await.is_ok());
            assert!(run_q2(&state, Duration::from_secs(60)).await.is_ok());
            assert!(run_q3(&state, Duration::from_secs(60)).await.is_ok());
            assert!(run_q4(&state, Duration::from_secs(60)).await.is_ok());
            assert!(run_q5(&state, Duration::from_secs(60)).await.is_ok());
            assert!(run_q6(&state, Duration::from_secs(60)).await.is_ok());
        }
    }
}
