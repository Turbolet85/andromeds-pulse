use duckdb::Connection;

use crate::contract::Error;

// Reserved table list per arch §Occupied Resources §DuckDB reserved tables.
// Chunk #20 baseline locked 7 tables; chunk #69 Phase B adds `log_templates`
// (8th table) for Drain L1c log-template-mining surface. Arch §Occupied
// Resources update via /andromeda-evolve --allow-arch-registry lands at
// Session 6 wrap per chunk #69 Phase B plan.md Step 32.
pub const RESERVED_TABLES: [&str; 8] = [
    "spans",
    "span_events",
    "span_links",
    "metrics_points",
    "log_records",
    "resources",
    "instrumentation_scopes",
    "log_templates",
];

// Static DDL — never `format!`-style interpolation, even for table names
// (per security plan §Anti-Patterns Input + 2026 DuckDB CVE cluster).
//
// Per-table constants kept as named source-of-truth for readers; the
// concatenated SCHEMA_DDL below is what gets executed. The ddl-spot-check
// test verifies the two stay in sync.

#[allow(dead_code)]
const CREATE_SPANS: &str = "\
CREATE TABLE IF NOT EXISTS spans (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    service_name VARCHAR NOT NULL,
    end_time_unix_nano BIGINT NOT NULL,
    status_code INTEGER NOT NULL,
    PRIMARY KEY (trace_id, span_id)
);";

#[allow(dead_code)]
const CREATE_SPAN_EVENTS: &str = "\
CREATE TABLE IF NOT EXISTS span_events (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    event_index INTEGER NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    name VARCHAR NOT NULL DEFAULT '',
    exception_type VARCHAR,
    exception_message VARCHAR,
    exception_stacktrace VARCHAR,
    fingerprint BLOB,
    PRIMARY KEY (trace_id, span_id, event_index)
);";

#[allow(dead_code)]
const CREATE_SPAN_LINKS: &str = "\
CREATE TABLE IF NOT EXISTS span_links (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    link_index INTEGER NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (trace_id, span_id, link_index)
);";

#[allow(dead_code)]
const CREATE_METRICS_POINTS: &str = "\
CREATE TABLE IF NOT EXISTS metrics_points (
    metric_name VARCHAR NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    resource_hash BLOB NOT NULL,
    value DOUBLE NOT NULL DEFAULT 0.0,
    data_point_kind INTEGER NOT NULL DEFAULT 0,
    seq BIGINT NOT NULL,
    PRIMARY KEY (metric_name, ts_unix_nano, resource_hash, seq)
);";

#[allow(dead_code)]
const CREATE_LOG_RECORDS: &str = "\
CREATE TABLE IF NOT EXISTS log_records (
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    resource_hash BLOB NOT NULL,
    severity_number INTEGER NOT NULL,
    body VARCHAR NOT NULL DEFAULT '',
    severity_text VARCHAR NOT NULL DEFAULT '',
    trace_id BLOB NOT NULL DEFAULT X'',
    span_id BLOB NOT NULL DEFAULT X'',
    template_id BIGINT,
    seq BIGINT NOT NULL,
    PRIMARY KEY (ts_unix_nano, resource_hash, severity_number, seq)
);";

#[allow(dead_code)]
const CREATE_RESOURCES: &str = "\
CREATE TABLE IF NOT EXISTS resources (
    resource_hash BLOB NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (resource_hash)
);";

#[allow(dead_code)]
const CREATE_INSTRUMENTATION_SCOPES: &str = "\
CREATE TABLE IF NOT EXISTS instrumentation_scopes (
    resource_hash BLOB NOT NULL,
    scope_name VARCHAR NOT NULL,
    scope_version VARCHAR NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (resource_hash, scope_name, scope_version)
);";

// Chunk #69 Phase B — Drain log-template-mining surface. Per-cluster metadata
// (template_id sequence + masked-template-tokens body + occurrence count +
// first/last-seen wall-clock). `template_content` stores PII-scrubbed template
// text (scrubbed via `crates/security::scrubber::scrub_attribute` before any
// write per capability P-047 + security plan §Logging NEVER-log discipline).
// `template_id` is the sequence assigned by `crates/buffer::drain::DrainMiner`;
// matches `log_records.template_id` foreign-key-shape (no explicit FK because
// log_records ring-buffer evictions race the template lifecycle).
#[allow(dead_code)]
const CREATE_LOG_TEMPLATES: &str = "\
CREATE TABLE IF NOT EXISTS log_templates (
    template_id BIGINT NOT NULL,
    template_content VARCHAR NOT NULL,
    occurrence_count BIGINT NOT NULL DEFAULT 0,
    first_seen_unix_nano BIGINT NOT NULL,
    last_seen_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (template_id)
);";

const SCHEMA_DDL: &str = concat!(
    "CREATE TABLE IF NOT EXISTS spans (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    service_name VARCHAR NOT NULL,
    end_time_unix_nano BIGINT NOT NULL,
    status_code INTEGER NOT NULL,
    PRIMARY KEY (trace_id, span_id)
);",
    "CREATE TABLE IF NOT EXISTS span_events (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    event_index INTEGER NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    name VARCHAR NOT NULL DEFAULT '',
    exception_type VARCHAR,
    exception_message VARCHAR,
    exception_stacktrace VARCHAR,
    fingerprint BLOB,
    PRIMARY KEY (trace_id, span_id, event_index)
);",
    "CREATE TABLE IF NOT EXISTS span_links (
    trace_id BLOB NOT NULL,
    span_id BLOB NOT NULL,
    link_index INTEGER NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (trace_id, span_id, link_index)
);",
    "CREATE TABLE IF NOT EXISTS metrics_points (
    metric_name VARCHAR NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    resource_hash BLOB NOT NULL,
    value DOUBLE NOT NULL DEFAULT 0.0,
    data_point_kind INTEGER NOT NULL DEFAULT 0,
    seq BIGINT NOT NULL,
    PRIMARY KEY (metric_name, ts_unix_nano, resource_hash, seq)
);",
    "CREATE TABLE IF NOT EXISTS log_records (
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    resource_hash BLOB NOT NULL,
    severity_number INTEGER NOT NULL,
    body VARCHAR NOT NULL DEFAULT '',
    severity_text VARCHAR NOT NULL DEFAULT '',
    trace_id BLOB NOT NULL DEFAULT X'',
    span_id BLOB NOT NULL DEFAULT X'',
    template_id BIGINT,
    seq BIGINT NOT NULL,
    PRIMARY KEY (ts_unix_nano, resource_hash, severity_number, seq)
);",
    "CREATE TABLE IF NOT EXISTS resources (
    resource_hash BLOB NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (resource_hash)
);",
    "CREATE TABLE IF NOT EXISTS instrumentation_scopes (
    resource_hash BLOB NOT NULL,
    scope_name VARCHAR NOT NULL,
    scope_version VARCHAR NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (resource_hash, scope_name, scope_version)
);",
    "CREATE TABLE IF NOT EXISTS log_templates (
    template_id BIGINT NOT NULL,
    template_content VARCHAR NOT NULL,
    occurrence_count BIGINT NOT NULL DEFAULT 0,
    first_seen_unix_nano BIGINT NOT NULL,
    last_seen_unix_nano BIGINT NOT NULL,
    PRIMARY KEY (template_id)
);",
);

pub fn create_schema(conn: &Connection) -> Result<(), Error> {
    conn.execute_batch(SCHEMA_DDL)
        .map_err(|e| Error::SchemaCreate {
            reason: format!("execute_batch: {}", sanitize_duckdb_error(&e.to_string())),
        })
}

// Sanitize DuckDB error strings before they cross into structured fields.
// Keeps a leading short token; strips paths / numbers that vary across runs.
// Per security plan §Anti-Patterns Logging: never expose file paths or
// library versions in error messages crossing the bridge.
fn sanitize_duckdb_error(raw: &str) -> String {
    raw.lines()
        .next()
        .unwrap_or("duckdb error")
        .chars()
        .take(120)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::collections::BTreeSet;

    fn open_in_memory() -> Connection {
        Connection::open_in_memory().expect("DuckDB :memory: open must succeed")
    }

    #[test]
    fn create_schema_succeeds_on_fresh_connection() {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create on fresh :memory: connection must succeed");
    }

    #[test]
    fn create_schema_is_idempotent() {
        let conn = open_in_memory();
        create_schema(&conn).expect("first create must succeed");
        create_schema(&conn).expect("second create must be a no-op (CREATE TABLE IF NOT EXISTS)");
    }

    #[test]
    fn schema_creates_exactly_eight_reserved_tables() {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        let mut stmt = conn
            .prepare("SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' ORDER BY table_name")
            .expect("information_schema query must prepare");
        let names: BTreeSet<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .expect("query_map must succeed")
            .map(|r| r.expect("row must read"))
            .collect();

        let expected: BTreeSet<String> = RESERVED_TABLES.iter().map(|s| s.to_string()).collect();
        assert_eq!(names, expected);
        assert_eq!(names.len(), 8, "chunk #69 added log_templates as 8th table");
    }

    // log_templates excluded — it uses `first_seen_unix_nano` / `last_seen_unix_nano`
    // instead of the standard `ts` / `ts_unix_nano` convention (template lifecycle
    // is window-based, not per-record timestamped). Coverage for log_templates
    // ts-column shape lives in `log_templates_has_first_seen_and_last_seen_columns`.
    #[rstest]
    #[case("spans")]
    #[case("span_events")]
    #[case("span_links")]
    #[case("metrics_points")]
    #[case("log_records")]
    #[case("resources")]
    #[case("instrumentation_scopes")]
    fn each_reserved_table_has_ts_and_ts_unix_nano_columns(#[case] table: &str) {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        let mut stmt = conn
            .prepare(
                "SELECT column_name FROM information_schema.columns WHERE table_schema = 'main' AND table_name = ?",
            )
            .expect("information_schema query must prepare");
        let columns: BTreeSet<String> = stmt
            .query_map([table], |row| row.get::<_, String>(0))
            .expect("query_map must succeed")
            .map(|r| r.expect("row must read"))
            .collect();

        assert!(
            columns.contains("ts"),
            "table {table} must have a `ts` TIMESTAMPTZ column"
        );
        assert!(
            columns.contains("ts_unix_nano"),
            "table {table} must have a `ts_unix_nano` BIGINT column"
        );
    }

    #[test]
    fn spans_primary_key_is_composite_trace_id_span_id() {
        // Verify composite PK by introspecting schema metadata via DuckDB's
        // information_schema.table_constraints + key_column_usage. Runtime PK
        // check via duplicate-INSERT path was observed to hang in
        // libduckdb-sys 1.10502 on this build — schema introspection is the
        // contract assertion, not behavioral PK enforcement. Behavioral
        // enforcement is exercised at the appender path (chunk #22+ when
        // queries land).
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        let mut stmt = conn
            .prepare(
                "SELECT column_name, ordinal_position FROM information_schema.key_column_usage \
                 WHERE table_schema = 'main' AND table_name = 'spans' \
                 ORDER BY ordinal_position",
            )
            .expect("prepare must succeed");
        let rows: Vec<(String, i64)> = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })
            .expect("query_map must succeed")
            .map(|r| r.expect("row must read"))
            .collect();

        assert_eq!(
            rows.len(),
            2,
            "spans primary key must have exactly 2 columns; got {rows:?}"
        );
        let names: Vec<&str> = rows.iter().map(|(n, _)| n.as_str()).collect();
        assert!(
            names.contains(&"trace_id"),
            "trace_id must be part of spans primary key; got {names:?}"
        );
        assert!(
            names.contains(&"span_id"),
            "span_id must be part of spans primary key; got {names:?}"
        );
    }

    // The contract half of the log-record identity fix; the behavioural half
    // is appender::tests::append_logs_batch_same_tick_same_severity_keeps_both_records.
    // An OTLP LogRecord has no spec-defined unique id, so `seq` carries the
    // identity its OTLP-native columns cannot.
    #[test]
    fn log_records_primary_key_includes_seq_ordinal() {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        let mut stmt = conn
            .prepare(
                "SELECT column_name, ordinal_position FROM information_schema.key_column_usage \
                 WHERE table_schema = 'main' AND table_name = 'log_records' \
                 ORDER BY ordinal_position",
            )
            .expect("prepare must succeed");
        let rows: Vec<(String, i64)> = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })
            .expect("query_map must succeed")
            .map(|r| r.expect("row must read"))
            .collect();

        assert_eq!(
            rows.len(),
            4,
            "log_records primary key must have exactly 4 columns; got {rows:?}"
        );
        let names: Vec<&str> = rows.iter().map(|(n, _)| n.as_str()).collect();
        for expected in ["ts_unix_nano", "resource_hash", "severity_number", "seq"] {
            assert!(
                names.contains(&expected),
                "{expected} must be part of log_records primary key; got {names:?}"
            );
        }
    }

    // Schema-introspection for the same reason as the two above (the
    // duplicate-INSERT probe hangs on this build); the behavioural pair is
    // appender::tests::append_metrics_batch_*_keeps_both_points. `metrics_points`
    // stores no attributes column, so two points of one metric differing only by
    // label set are identical on every OTLP-native column — `seq` carries the
    // identity they cannot.
    #[test]
    fn metrics_points_primary_key_includes_seq_ordinal() {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        let mut stmt = conn
            .prepare(
                "SELECT column_name, ordinal_position FROM information_schema.key_column_usage \
                 WHERE table_schema = 'main' AND table_name = 'metrics_points' \
                 ORDER BY ordinal_position",
            )
            .expect("prepare must succeed");
        let rows: Vec<(String, i64)> = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })
            .expect("query_map must succeed")
            .map(|r| r.expect("row must read"))
            .collect();

        assert_eq!(
            rows.len(),
            4,
            "metrics_points primary key must have exactly 4 columns; got {rows:?}"
        );
        let names: Vec<&str> = rows.iter().map(|(n, _)| n.as_str()).collect();
        for expected in ["metric_name", "ts_unix_nano", "resource_hash", "seq"] {
            assert!(
                names.contains(&expected),
                "{expected} must be part of metrics_points primary key; got {names:?}"
            );
        }
    }

    #[test]
    fn ts_unix_nano_round_trips_full_u64_precision() {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        // Nanosecond precision boundary: a value DuckDB's TIMESTAMPTZ column
        // (microsecond precision) cannot represent losslessly.
        conn.execute_batch(
            "INSERT INTO spans (trace_id, span_id, ts, ts_unix_nano, service_name, end_time_unix_nano, status_code) VALUES \
             (X'07070707070707070707070707070707', X'0808080808080808', '2026-05-06T00:00:00Z'::TIMESTAMPTZ, 1700000000123456789, '', 1700000000123456789, 0);",
        )
        .expect("insert must succeed");

        let read_back: i64 = conn
            .query_row(
                "SELECT ts_unix_nano FROM spans WHERE trace_id = X'07070707070707070707070707070707' AND span_id = X'0808080808080808'",
                [],
                |row| row.get(0),
            )
            .expect("row must read");
        assert_eq!(
            read_back, 1_700_000_000_123_456_789_i64,
            "ts_unix_nano column must round-trip the full nanosecond precision"
        );
    }

    #[test]
    fn ddl_constants_match_concatenated_schema() {
        // Spot-check that all 8 named DDL constants together cover the same
        // ground as SCHEMA_DDL — guards against accidental DDL drift if
        // someone edits one but forgets the other. Chunk #69 Phase B extended
        // from 7 → 8 with the addition of CREATE_LOG_TEMPLATES.
        let union = format!(
            "{}{}{}{}{}{}{}{}",
            CREATE_SPANS,
            CREATE_SPAN_EVENTS,
            CREATE_SPAN_LINKS,
            CREATE_METRICS_POINTS,
            CREATE_LOG_RECORDS,
            CREATE_RESOURCES,
            CREATE_INSTRUMENTATION_SCOPES,
            CREATE_LOG_TEMPLATES,
        );
        assert_eq!(SCHEMA_DDL, union);
    }

    // Chunk #69 Phase B — log_templates uses first_seen/last_seen window-based
    // timestamps instead of per-record ts/ts_unix_nano. Verify the actual
    // column names match the DDL.
    #[test]
    fn log_templates_has_first_seen_and_last_seen_columns() {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        let mut stmt = conn
            .prepare(
                "SELECT column_name FROM information_schema.columns WHERE table_schema = 'main' AND table_name = ?",
            )
            .expect("information_schema query must prepare");
        let columns: BTreeSet<String> = stmt
            .query_map(["log_templates"], |row| row.get::<_, String>(0))
            .expect("query_map must succeed")
            .map(|r| r.expect("row must read"))
            .collect();

        for expected in [
            "template_id",
            "template_content",
            "occurrence_count",
            "first_seen_unix_nano",
            "last_seen_unix_nano",
        ] {
            assert!(
                columns.contains(expected),
                "log_templates must have `{expected}` column; got {columns:?}"
            );
        }
    }

    // Chunk #69 Phase B — log_records gains a nullable `template_id BIGINT`
    // column populated by `crates/buffer::drain::DrainMiner` at hot-path
    // append. Existing rows tolerate NULL when Drain disabled or not yet
    // assigned per chunk #69 Phase B plan §Acceptance Criteria (tests).
    #[test]
    fn log_records_has_nullable_template_id_column() {
        let conn = open_in_memory();
        create_schema(&conn).expect("schema create must succeed");

        let mut stmt = conn
            .prepare(
                "SELECT column_name, is_nullable FROM information_schema.columns \
                 WHERE table_schema = 'main' AND table_name = 'log_records' \
                 AND column_name = 'template_id'",
            )
            .expect("information_schema query must prepare");
        let rows: Vec<(String, String)> = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .expect("query_map must succeed")
            .map(|r| r.expect("row must read"))
            .collect();
        assert_eq!(rows.len(), 1, "template_id column must exist exactly once");
        // DuckDB reports nullable as "YES" / "NO" per SQL standard.
        assert_eq!(
            rows[0].1, "YES",
            "template_id MUST be nullable; existing rows tolerate NULL pre-Drain-assignment"
        );
    }
}
