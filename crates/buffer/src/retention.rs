use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use duckdb::Connection;
use tracing_error::SpanTrace;

use crate::contract::Error;
use crate::state::BufferState;

// Per-table DELETE constants. Static stable strings — never `format!`-built
// (per security plan §Anti-Patterns Input + chunk #20 grep gate). Order
// parallels `crate::schema::RESERVED_TABLES` but is local to this module to
// keep DELETE / CREATE concerns separable.
//
// Chunk #69 Phase B note: `log_templates` (8th reserved table) is excluded
// from the retention sweep — template lifecycle is LRU-managed by the
// `crates/buffer::drain::DrainMiner` (`max_clusters` cap, default 1000).
// Templates have no per-record `ts_unix_nano`; the `first_seen_unix_nano`
// / `last_seen_unix_nano` window-based metadata is informational. Evicting
// templates by retention cutoff would orphan `log_records.template_id`
// references; LRU-on-write keeps the cap without that issue.
const DELETE_BY_CUTOFF: [&str; 7] = [
    "DELETE FROM spans WHERE ts_unix_nano < ?",
    "DELETE FROM span_events WHERE ts_unix_nano < ?",
    "DELETE FROM span_links WHERE ts_unix_nano < ?",
    "DELETE FROM metrics_points WHERE ts_unix_nano < ?",
    "DELETE FROM log_records WHERE ts_unix_nano < ?",
    "DELETE FROM resources WHERE ts_unix_nano < ?",
    "DELETE FROM instrumentation_scopes WHERE ts_unix_nano < ?",
];

/// Reserved tables NOT subject to retention sweep. Excluded because their
/// lifecycle is managed elsewhere (LRU, not time-window). See
/// `DELETE_BY_CUTOFF` doc for rationale per table.
#[cfg(test)]
const RETENTION_EXCLUDED_TABLES: &[&str] = &["log_templates"];

/// Long-running periodic task that issues `DELETE WHERE ts_unix_nano < ?`
/// against each of the 7 reserved tables to enforce the in-memory ring-
/// buffer retention window. Runs DuckDB calls inside `tokio::task::spawn_blocking`
/// to keep the tokio runtime healthy under DuckDB's blocking-IO semantics
/// (per chunk #20 `crates/buffer/src/consumer.rs::run_consumer` precedent).
///
/// Cadence: `retention_seconds.max(60) / 6` (10s minimum at 60s retention,
/// 100s at 600s default — fine-grained enough for SLO, coarse enough that
/// the sweep doesn't dominate CPU). The cadence is independent of the
/// `buffer.tick` heartbeat (15s, fixed in pulse-app/src/heartbeat.rs).
pub async fn run_retention(
    conn: Arc<Mutex<Connection>>,
    state: Arc<BufferState>,
    retention_seconds: u64,
) {
    let cadence_secs = retention_seconds.max(60) / 6;
    let mut interval = tokio::time::interval(Duration::from_secs(cadence_secs));
    // Skip the immediate first tick — `tokio::time::interval` fires
    // immediately on first poll, which would sweep before any rows have
    // landed and lead to a misleading first-tick eviction-of-zero log line.
    interval.tick().await;

    loop {
        interval.tick().await;
        run_one_sweep(&conn, &state, retention_seconds).await;
    }
}

/// Single retention sweep: compute cutoff, spawn_blocking the SQL execution,
/// record outcome to BufferState, emit structured tracing events. Exposed
/// at crate level so tests can drive a single sweep deterministically without
/// needing tokio paused-clock + interval orchestration.
pub(crate) async fn run_one_sweep(
    conn: &Arc<Mutex<Connection>>,
    state: &Arc<BufferState>,
    retention_seconds: u64,
) {
    let cutoff_ns = compute_cutoff_ns(retention_seconds);
    let conn_clone = Arc::clone(conn);

    let start = Instant::now();
    let join =
        tokio::task::spawn_blocking(move || retention_sweep_inner(&conn_clone, cutoff_ns)).await;
    let duration_ms = start.elapsed().as_millis() as u64;

    match join {
        Ok(Ok(rows_evicted)) => {
            state.record_eviction(rows_evicted);
            state.mark_retention_active();

            let snap = state.snapshot();
            let rows_active = snap.rows_ingested.saturating_sub(snap.eviction_count);
            state.set_memory_bytes(rows_active.saturating_mul(BYTES_PER_ROW_ESTIMATE));

            tracing::info!(
                target: "buffer.retention.sweep",
                query_id = "buffer.retention.sweep",
                param_count = 1_u64,
                param_types = "timestamp",
                rows_evicted = rows_evicted,
                duration_ms = duration_ms,
                cutoff_ts_unix_nano = cutoff_ns,
                "retention sweep completed",
            );
        }
        Ok(Err(e)) => {
            tracing::error!(
                target: "buffer.retention.sweep.error",
                error_type = describe_error(&e),
                duration_ms = duration_ms,
                spantrace = ?SpanTrace::capture(),
                "retention sweep failed",
            );
        }
        Err(join_err) => {
            tracing::error!(
                target: "buffer.retention.sweep.error",
                error_type = if join_err.is_panic() { "panic" } else { "join_error" },
                duration_ms = duration_ms,
                spantrace = ?SpanTrace::capture(),
                "retention sweep blocking task failed",
            );
        }
    }
}

const BYTES_PER_ROW_ESTIMATE: u64 = 256;

/// Synchronous helper that performs one retention sweep across all 7
/// reserved tables. Called inside `tokio::task::spawn_blocking` from
/// `run_retention`. Returns total rows evicted.
pub(crate) fn retention_sweep_inner(
    conn: &Arc<Mutex<Connection>>,
    cutoff_ts_unix_nano: i64,
) -> Result<u64, Error> {
    let guard = conn.lock().map_err(|_| Error::ConnectionLost)?;
    let mut total = 0u64;
    for sql in DELETE_BY_CUTOFF.iter() {
        let mut stmt = guard.prepare(sql).map_err(|e| Error::Retention {
            reason: format!("prepare: {}", short_err(&e.to_string())),
        })?;
        let rows = stmt
            .execute([cutoff_ts_unix_nano])
            .map_err(|e| Error::Retention {
                reason: format!("execute: {}", short_err(&e.to_string())),
            })?;
        total += rows as u64;
    }
    Ok(total)
}

fn compute_cutoff_ns(retention_seconds: u64) -> i64 {
    let now_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0);
    now_ns.saturating_sub((retention_seconds as i64).saturating_mul(1_000_000_000))
}

fn describe_error(e: &Error) -> &'static str {
    match e {
        Error::Init { .. } => "init_failed",
        Error::SchemaCreate { .. } => "schema_create_failed",
        Error::Append { .. } => "append_failed",
        Error::ConnectionLost => "connection_lost",
        Error::InvalidBatch { .. } => "invalid_batch",
        Error::Retention { .. } => "retention_failed",
        Error::BroadcastEncode { .. } => "broadcast_encode_failed",
        Error::BroadcastSizeCapExceeded { .. } => "broadcast_size_cap_exceeded",
        // Chunk #69 Phase B — Drain operation errors.
        Error::Drain { .. } => "drain_failed",
    }
}

fn short_err(raw: &str) -> String {
    raw.lines().next().unwrap_or("").chars().take(120).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::create_schema;

    fn fresh_conn_with_schema() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().expect("open_in_memory");
        create_schema(&conn).expect("schema create");
        Arc::new(Mutex::new(conn))
    }

    fn seed_span(conn: &Arc<Mutex<Connection>>, trace_id: u8, span_id: u8, ts_ns: i64) {
        let guard = conn.lock().expect("lock");
        let ts_us = ts_ns / 1_000;
        let ts_str = format_timestamp_us(ts_us);
        let mut stmt = guard
            .prepare(
                "INSERT INTO spans (trace_id, span_id, ts, ts_unix_nano, service_name, end_time_unix_nano, status_code) \
                 VALUES (?, ?, ?::TIMESTAMPTZ, ?, '', ?, 0)",
            )
            .expect("prepare insert");
        let trace_blob = vec![trace_id; 16];
        let span_blob = vec![span_id; 8];
        let trace_param: &[u8] = trace_blob.as_slice();
        let span_param: &[u8] = span_blob.as_slice();
        stmt.execute(duckdb::params![
            trace_param,
            span_param,
            ts_str,
            ts_ns,
            ts_ns
        ])
        .expect("insert span");
    }

    fn format_timestamp_us(ts_us: i64) -> String {
        // Convert microseconds since epoch to an ISO-8601 string DuckDB accepts.
        // 1970-01-01 baseline + offset; for test purposes any in-range
        // timestamp works since the cutoff comparison is against ts_unix_nano,
        // not the formatted ts column.
        let secs = ts_us / 1_000_000;
        let micros = ts_us.rem_euclid(1_000_000);
        chrono_like(secs, micros)
    }

    fn chrono_like(secs: i64, micros: i64) -> String {
        // Minimal ISO-8601 formatter for test-time inserts. Avoids pulling
        // chrono into the buffer crate's [dependencies] (currently absent).
        // DuckDB accepts `YYYY-MM-DD HH:MM:SS.ffffff+00:00`.
        let days_since_epoch = secs / 86_400;
        let secs_of_day = secs.rem_euclid(86_400);
        let (year, month, day) = day_to_ymd(days_since_epoch);
        let hour = secs_of_day / 3600;
        let minute = (secs_of_day % 3600) / 60;
        let second = secs_of_day % 60;
        format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}.{micros:06}+00:00")
    }

    fn day_to_ymd(days_since_epoch: i64) -> (i64, u32, u32) {
        // Civil-from-days algorithm (Howard Hinnant's date library) — handles
        // the 1970+ range adequately for test fixtures.
        let z = days_since_epoch + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = (z - era * 146_097) as u64;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        (y, m as u32, d as u32)
    }

    #[test]
    fn retention_sweep_inner_with_cutoff_before_window_preserves_rows() {
        let conn = fresh_conn_with_schema();
        // Seed at t = 1_700_000_000 ns (2023-11-14)
        seed_span(&conn, 1, 1, 1_700_000_000_000_000_000);
        seed_span(&conn, 2, 2, 1_700_000_000_000_000_001);

        // Cutoff BEFORE seeded ts → no rows match `ts_unix_nano < cutoff`
        let evicted =
            retention_sweep_inner(&conn, 1_600_000_000_000_000_000).expect("sweep must succeed");

        assert_eq!(evicted, 0);
        let row_count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count");
        assert_eq!(row_count, 2);
    }

    #[test]
    fn retention_sweep_inner_with_cutoff_after_window_evicts_rows() {
        let conn = fresh_conn_with_schema();
        seed_span(&conn, 1, 1, 1_700_000_000_000_000_000);
        seed_span(&conn, 2, 2, 1_700_000_000_000_000_001);

        // Cutoff AFTER seeded ts → all rows match `ts_unix_nano < cutoff`
        let evicted =
            retention_sweep_inner(&conn, 1_800_000_000_000_000_000).expect("sweep must succeed");

        assert_eq!(evicted, 2);
        let row_count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count");
        assert_eq!(row_count, 0);
    }

    #[test]
    fn retention_sweep_inner_iterates_all_seven_tables_returns_zero_when_empty() {
        let conn = fresh_conn_with_schema();
        // Empty schema, sweep should iterate all 7 tables without error.
        let evicted = retention_sweep_inner(&conn, 1_800_000_000_000_000_000)
            .expect("sweep must succeed against empty tables");
        assert_eq!(evicted, 0);
    }

    #[test]
    fn retention_sweep_inner_chaos_evicts_partial_subset() {
        let conn = fresh_conn_with_schema();
        // Seed 100 spans evenly distributed across t=1700e9..1700e9+1000ns
        for i in 0..100u8 {
            seed_span(&conn, i, i, 1_700_000_000_000_000_000 + (i as i64) * 10);
        }
        // Cutoff at midpoint
        let cutoff = 1_700_000_000_000_000_000 + 500;
        let evicted = retention_sweep_inner(&conn, cutoff).expect("chaos sweep must succeed");

        let post: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM spans", [], |row| row.get(0))
            .expect("count");
        // Pre-cutoff rows evicted; post-cutoff rows preserved
        assert!(evicted > 0 && evicted < 100);
        assert!(post > 0 && post < 100);
        assert_eq!(evicted as i64 + post, 100);
    }

    #[test]
    fn retention_sweep_inner_returns_connection_lost_on_poisoned_mutex() {
        let conn = fresh_conn_with_schema();
        // Poison the Mutex by panicking in another thread while holding lock.
        let conn_clone = Arc::clone(&conn);
        let _ = std::thread::spawn(move || {
            let _guard = conn_clone.lock().unwrap();
            panic!("poison");
        })
        .join();

        let result = retention_sweep_inner(&conn, 1_800_000_000_000_000_000);
        assert!(matches!(result, Err(Error::ConnectionLost)));
    }

    #[test]
    fn delete_by_cutoff_constants_match_reserved_tables() {
        // Drift guard: the DELETE constants reference every retention-bounded
        // reserved table. Chunk #69 Phase B excludes `log_templates` (LRU-
        // managed by DrainMiner; see `DELETE_BY_CUTOFF` doc comment).
        for table in crate::schema::RESERVED_TABLES.iter() {
            if RETENTION_EXCLUDED_TABLES.contains(table) {
                continue;
            }
            let needle = format!(" {table} ");
            assert!(
                DELETE_BY_CUTOFF.iter().any(|sql| sql.contains(&needle)),
                "DELETE_BY_CUTOFF must reference reserved table `{table}` \
                 (not in RETENTION_EXCLUDED_TABLES)"
            );
        }
        // Length invariant: DELETE_BY_CUTOFF covers all reserved tables
        // EXCEPT the LRU-managed exclusions.
        assert_eq!(
            DELETE_BY_CUTOFF.len(),
            crate::schema::RESERVED_TABLES.len() - RETENTION_EXCLUDED_TABLES.len()
        );
    }

    #[test]
    fn retention_excluded_tables_are_subset_of_reserved() {
        // Sanity: every exclusion must be a real reserved table (catches
        // typos like "log_template" missing the trailing s).
        for excluded in RETENTION_EXCLUDED_TABLES {
            assert!(
                crate::schema::RESERVED_TABLES.contains(excluded),
                "RETENTION_EXCLUDED_TABLES entry `{excluded}` must appear in RESERVED_TABLES"
            );
        }
    }

    #[test]
    fn describe_error_returns_constant_strings_including_retention() {
        assert_eq!(
            describe_error(&Error::Retention { reason: "x".into() }),
            "retention_failed"
        );
        assert_eq!(describe_error(&Error::ConnectionLost), "connection_lost");
        assert_eq!(
            describe_error(&Error::Init { reason: "x".into() }),
            "init_failed"
        );
    }

    #[tokio::test]
    async fn run_one_sweep_evicts_old_rows_and_marks_active() {
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());

        // Seed rows with epoch-0 timestamps; cutoff = wall_clock - 60s sweeps them.
        seed_span(&conn, 1, 1, 0);
        seed_span(&conn, 2, 2, 1);
        seed_span(&conn, 3, 3, 2);

        // Pre-sweep state assertion
        assert!(!state.snapshot().retention_window_active);

        run_one_sweep(&conn, &state, 60).await;

        let snap = state.snapshot();
        assert!(
            snap.retention_window_active,
            "retention_window_active must flip to true after one sweep"
        );
        assert_eq!(
            snap.eviction_count, 3,
            "all 3 epoch-0 rows must be evicted at cutoff = wall_clock - 60s"
        );
    }

    #[tokio::test]
    async fn run_one_sweep_sets_memory_bytes_per_rows_active_estimate() {
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());

        // record_rows_appended(10) without inserting → after sweep, rows_active
        // = 10 (ingested) - 0 (evicted) = 10. memory_bytes = 10 * 256 = 2560.
        state.record_rows_appended(10);

        run_one_sweep(&conn, &state, 60).await;

        assert_eq!(state.snapshot().memory_bytes, 10 * 256);
    }

    #[tokio::test]
    async fn run_one_sweep_sets_zero_memory_bytes_when_all_evicted() {
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());

        // Seed 5 rows + record they were ingested. After sweep at cutoff = now - 60s,
        // all 5 evict; rows_active = 5 - 5 = 0; memory_bytes = 0.
        for i in 0..5u8 {
            seed_span(&conn, i, i, i as i64);
        }
        state.record_rows_appended(5);

        run_one_sweep(&conn, &state, 60).await;

        let snap = state.snapshot();
        assert_eq!(snap.eviction_count, 5);
        assert_eq!(snap.memory_bytes, 0);
    }

    #[tokio::test]
    async fn run_retention_can_be_spawned_and_aborted_cleanly() {
        // Smoke test: verifies run_retention can be wrapped in tokio::spawn
        // and aborted without panic. Detailed sweep behavior is covered by
        // run_one_sweep_* tests above.
        let conn = fresh_conn_with_schema();
        let state = Arc::new(BufferState::new());

        let handle = tokio::spawn(async move { run_retention(conn, state, 60).await });
        handle.abort();
        let result = handle.await;
        assert!(
            result.is_err(),
            "aborted task must complete with cancellation error"
        );
        assert!(result.unwrap_err().is_cancelled());
    }
}
