//! Corpus SQLite schema — chunk #68; v2 at 2026-08-30.
//!
//! Five tables (dist-arch v3 §Schema Additions minus `baseline_state`,
//! dropped at schema v2 — it had zero readers and zero production writers;
//! real baseline persistence rides `pipeline_metrics` under
//! `metric_name = "baseline_state"` / `layer = "l1b"`, and the same-name
//! collision kept reading as a broken persistence path):
//!
//! - `service_registry` — per-service lifecycle state machine entries
//! - `pipeline_metrics` — pipeline L1a/L1b/L2/L3 metric snapshots
//! - `incidents` — top-level incident records
//! - `incident_events` — events attached to incidents
//! - `digest_archive` — distilled digests for L4 retrieval
//!
//! All BLOB columns store AES-256-GCM-encrypted payloads (nonce||ciphertext||tag);
//! integer / text columns are plaintext metadata (table + column names are
//! visible without the key per Phase 6 user decision; cell payloads
//! opaque).

use rusqlite::Connection;

use crate::error::Error;

/// Schema version embedded in `PRAGMA user_version`. Bump on any DDL
/// change. Known versions migrate forward in a ladder; an UNKNOWN (newer)
/// version rejects on mismatch.
pub(crate) const SCHEMA_VERSION: u32 = 2;

pub(crate) const TABLE_NAMES: &[&str] = &[
    "service_registry",
    "pipeline_metrics",
    "incidents",
    "incident_events",
    "digest_archive",
];

const CREATE_SERVICE_REGISTRY_SQL: &str = "\
CREATE TABLE IF NOT EXISTS service_registry (
    id INTEGER PRIMARY KEY,
    service_name TEXT NOT NULL UNIQUE,
    state TEXT NOT NULL,
    first_seen_unix_nano INTEGER NOT NULL,
    last_seen_unix_nano INTEGER NOT NULL,
    last_transition_unix_nano INTEGER NOT NULL,
    manual_override TEXT
)";

const CREATE_PIPELINE_METRICS_SQL: &str = "\
CREATE TABLE IF NOT EXISTS pipeline_metrics (
    id INTEGER PRIMARY KEY,
    metric_name TEXT NOT NULL,
    layer TEXT NOT NULL,
    snapshot_unix_nano INTEGER NOT NULL,
    payload BLOB NOT NULL
)";

const CREATE_INCIDENTS_SQL: &str = "\
CREATE TABLE IF NOT EXISTS incidents (
    id INTEGER PRIMARY KEY,
    workspace TEXT NOT NULL,
    status TEXT NOT NULL,
    created_unix_nano INTEGER NOT NULL,
    updated_unix_nano INTEGER NOT NULL,
    resolved_unix_nano INTEGER,
    read_unix_nano INTEGER,
    payload BLOB NOT NULL
)";

const CREATE_INCIDENT_EVENTS_SQL: &str = "\
CREATE TABLE IF NOT EXISTS incident_events (
    id INTEGER PRIMARY KEY,
    incident_id INTEGER NOT NULL,
    event_kind TEXT NOT NULL,
    occurred_unix_nano INTEGER NOT NULL,
    payload BLOB NOT NULL,
    FOREIGN KEY (incident_id) REFERENCES incidents(id)
)";

const CREATE_DIGEST_ARCHIVE_SQL: &str = "\
CREATE TABLE IF NOT EXISTS digest_archive (
    id INTEGER PRIMARY KEY,
    digest_kind TEXT NOT NULL,
    assembled_unix_nano INTEGER NOT NULL,
    token_count INTEGER NOT NULL,
    payload BLOB NOT NULL
)";

const DDL_STATEMENTS: &[&str] = &[
    CREATE_SERVICE_REGISTRY_SQL,
    CREATE_PIPELINE_METRICS_SQL,
    CREATE_INCIDENTS_SQL,
    CREATE_INCIDENT_EVENTS_SQL,
    CREATE_DIGEST_ARCHIVE_SQL,
];

/// Apply schema migrations. Reads `PRAGMA user_version` and walks the
/// ladder inside one transaction:
///
/// - 0 → first launch; create the v2 tables, set `user_version = 2`.
/// - 1 → v1 corpus; `DROP TABLE IF EXISTS baseline_state` (destructive by
///   measurement, not by risk: the table had zero readers and its only
///   writer was a test — a v1 corpus holds no production data there), then
///   set `user_version = 2`.
/// - equals `SCHEMA_VERSION` → no-op.
/// - anything newer/unknown → `Error::SchemaVersionMismatch` (refuse to
///   proceed rather than risk data corruption).
///
/// Idempotent: both the CREATEs and the DROP are `IF (NOT) EXISTS`.
pub(crate) fn apply_migrations(conn: &Connection) -> Result<u32, Error> {
    let current: u32 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|_| Error::MigrationFailed)?;

    if current == SCHEMA_VERSION {
        return Ok(SCHEMA_VERSION);
    }
    if current > SCHEMA_VERSION {
        return Err(Error::SchemaVersionMismatch);
    }

    let tx = conn
        .unchecked_transaction()
        .map_err(|_| Error::MigrationFailed)?;
    if current == 0 {
        for ddl in DDL_STATEMENTS {
            tx.execute(ddl, []).map_err(|_| Error::MigrationFailed)?;
        }
    }
    if current <= 1 {
        tx.execute("DROP TABLE IF EXISTS baseline_state", [])
            .map_err(|_| Error::MigrationFailed)?;
    }
    tx.execute(&format!("PRAGMA user_version = {SCHEMA_VERSION}"), [])
        .map_err(|_| Error::MigrationFailed)?;
    tx.commit().map_err(|_| Error::MigrationFailed)?;
    Ok(SCHEMA_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_apply_to_fresh_db() {
        let conn = Connection::open_in_memory().expect("in-memory");
        let version = apply_migrations(&conn).expect("migrations apply");
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn migrations_are_idempotent() {
        let conn = Connection::open_in_memory().expect("in-memory");
        let v1 = apply_migrations(&conn).expect("first call");
        let v2 = apply_migrations(&conn).expect("second call");
        assert_eq!(v1, v2);
    }

    #[test]
    fn migrations_create_all_expected_tables() {
        let conn = Connection::open_in_memory().expect("in-memory");
        apply_migrations(&conn).expect("apply");
        for table in TABLE_NAMES {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?",
                    [table],
                    |row| row.get(0),
                )
                .expect("query sqlite_master");
            assert_eq!(count, 1, "table {table} not created");
        }
    }

    #[test]
    fn migrations_reject_unknown_schema_version() {
        let conn = Connection::open_in_memory().expect("in-memory");
        conn.execute_batch("PRAGMA user_version = 99")
            .expect("set version");
        let result = apply_migrations(&conn);
        assert!(matches!(result, Err(Error::SchemaVersionMismatch)));
    }

    #[test]
    fn user_version_set_after_migration() {
        let conn = Connection::open_in_memory().expect("in-memory");
        apply_migrations(&conn).expect("apply");
        let v: u32 = conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("read user_version");
        assert_eq!(v, SCHEMA_VERSION);
    }

    #[test]
    fn v1_corpus_upgrades_to_v2_dropping_baseline_state() {
        let conn = Connection::open_in_memory().expect("in-memory");
        // Reconstruct a v1 corpus: the surviving tables plus the
        // then-present baseline_state, stamped user_version = 1.
        for ddl in DDL_STATEMENTS {
            conn.execute(ddl, []).expect("v1 ddl");
        }
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS baseline_state (
                id INTEGER PRIMARY KEY,
                service_name TEXT NOT NULL,
                operation TEXT,
                snapshot_unix_nano INTEGER NOT NULL,
                payload BLOB NOT NULL
            ); PRAGMA user_version = 1;",
        )
        .expect("v1 shape");

        let version = apply_migrations(&conn).expect("upgrade");
        assert_eq!(version, SCHEMA_VERSION);
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'baseline_state'",
                [],
                |row| row.get(0),
            )
            .expect("query sqlite_master");
        assert_eq!(
            count, 0,
            "baseline_state must be dropped by the v1->v2 step"
        );
        // Converges on the fresh-create shape: exactly the v2 tables.
        for table in TABLE_NAMES {
            let present: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?",
                    [table],
                    |row| row.get(0),
                )
                .expect("query sqlite_master");
            assert_eq!(present, 1, "table {table} must survive the upgrade");
        }
    }
}
