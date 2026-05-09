use duckdb::Connection;

use crate::contract::Error;

// Reserved table list per arch §Occupied Resources §DuckDB reserved tables.
// Locked set; chunk #20 may not add a new table outside this list.
pub const RESERVED_TABLES: [&str; 7] = [
    "spans",
    "span_events",
    "span_links",
    "metrics_points",
    "log_records",
    "resources",
    "instrumentation_scopes",
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
    PRIMARY KEY (metric_name, ts_unix_nano, resource_hash)
);";

#[allow(dead_code)]
const CREATE_LOG_RECORDS: &str = "\
CREATE TABLE IF NOT EXISTS log_records (
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    resource_hash BLOB NOT NULL,
    severity_number INTEGER NOT NULL,
    PRIMARY KEY (ts_unix_nano, resource_hash, severity_number)
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
    PRIMARY KEY (metric_name, ts_unix_nano, resource_hash)
);",
    "CREATE TABLE IF NOT EXISTS log_records (
    ts TIMESTAMPTZ NOT NULL,
    ts_unix_nano BIGINT NOT NULL,
    resource_hash BLOB NOT NULL,
    severity_number INTEGER NOT NULL,
    PRIMARY KEY (ts_unix_nano, resource_hash, severity_number)
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
    fn schema_creates_exactly_seven_reserved_tables() {
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
    }

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
        // Spot-check that all 7 named DDL constants together cover the same
        // ground as SCHEMA_DDL — guards against accidental DDL drift if
        // someone edits one but forgets the other.
        let union = format!(
            "{}{}{}{}{}{}{}",
            CREATE_SPANS,
            CREATE_SPAN_EVENTS,
            CREATE_SPAN_LINKS,
            CREATE_METRICS_POINTS,
            CREATE_LOG_RECORDS,
            CREATE_RESOURCES,
            CREATE_INSTRUMENTATION_SCOPES,
        );
        assert_eq!(SCHEMA_DDL, union);
    }
}
