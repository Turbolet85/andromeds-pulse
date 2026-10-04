//! rusqlite connection helper + prepared-statement query wrappers — chunk #68.
//!
//! Connection opening + per-table count helpers using ONLY prepared
//! statements + parameter binding. Table names come from a fixed
//! allowlist (`schema::TABLE_NAMES`) — never user-controlled — so the
//! count helper's `format!` to inject the table name is safe (table
//! name is not user input).

use std::fs;
use std::path::Path;

use rusqlite::Connection;

use crate::error::Error;
use crate::schema::{TABLE_NAMES, apply_migrations};

/// Open (or create) the corpus SQLite file at `path` and run first-launch
/// migrations. Caller is responsible for path canonicalization via
/// `strict-path`; this fn assumes `path` is already resolved.
pub(crate) fn open_at_path(path: &Path) -> Result<Connection, Error> {
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(path).map_err(|_| Error::QueryFailed)?;
    apply_migrations(&conn)?;
    Ok(conn)
}

/// Open an in-memory SQLite connection + run migrations. Used by tests
/// and by call sites that want a corpus instance without on-disk
/// persistence (e.g., bindings emission test).
pub(crate) fn open_in_memory() -> Result<Connection, Error> {
    let conn = Connection::open_in_memory().map_err(|_| Error::QueryFailed)?;
    apply_migrations(&conn)?;
    Ok(conn)
}

/// Count rows in `table_name`. Table names MUST come from the
/// [`TABLE_NAMES`] allowlist — rejected otherwise. The `format!` of the
/// table name into the SQL is safe because the allowlist contents are
/// crate-internal constants, never user input.
pub(crate) fn count_table(conn: &Connection, table_name: &str) -> Result<u64, Error> {
    if !TABLE_NAMES.contains(&table_name) {
        return Err(Error::QueryFailed);
    }
    let sql = format!("SELECT COUNT(*) FROM {table_name}");
    let count: i64 = conn
        .query_row(&sql, [], |row| row.get(0))
        .map_err(|_| Error::QueryFailed)?;
    Ok(count.max(0) as u64)
}

/// Total file size of the corpus DB. Returns 0 if path doesn't exist
/// yet (e.g., in-memory connection — there is no file on disk).
pub(crate) fn file_size_bytes(path: &Path) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn open_at_path_creates_file_and_runs_migrations() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("test-corpus.db");
        let conn = open_at_path(&path).expect("open");
        // Verify table presence via count_table — which exercises migrations.
        for table in TABLE_NAMES {
            assert_eq!(count_table(&conn, table).expect("count"), 0);
        }
    }

    #[test]
    fn open_in_memory_runs_migrations() {
        let conn = open_in_memory().expect("open");
        for table in TABLE_NAMES {
            assert_eq!(count_table(&conn, table).expect("count"), 0);
        }
    }

    #[test]
    fn count_table_rejects_arbitrary_names() {
        let conn = open_in_memory().expect("open");
        let result = count_table(&conn, "sqlite_master");
        assert!(matches!(result, Err(Error::QueryFailed)));
    }

    #[test]
    fn count_table_rejects_sql_injection_attempt() {
        let conn = open_in_memory().expect("open");
        // Even with a SQL-shaped string, the allowlist gate rejects.
        let result = count_table(&conn, "incidents; DROP TABLE incidents; --");
        assert!(matches!(result, Err(Error::QueryFailed)));
        // Verify incidents table still exists (not dropped).
        assert_eq!(count_table(&conn, "incidents").expect("count"), 0);
    }

    #[test]
    fn count_table_increments_after_insert() {
        let conn = open_in_memory().expect("open");
        assert_eq!(count_table(&conn, "incidents").expect("count"), 0);
        conn.execute(
            "INSERT INTO incidents (workspace, status, created_unix_nano, updated_unix_nano, payload) VALUES (?, ?, ?, ?, ?)",
            rusqlite::params!["ws", "active", 1_000i64, 1_000i64, &b"payload"[..]],
        )
        .expect("insert");
        assert_eq!(count_table(&conn, "incidents").expect("count"), 1);
    }

    #[test]
    fn file_size_zero_for_nonexistent_path() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("nonexistent.db");
        assert_eq!(file_size_bytes(&path), 0);
    }

    #[test]
    fn file_size_nonzero_after_open() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("with-data.db");
        let _conn = open_at_path(&path).expect("open");
        assert!(file_size_bytes(&path) > 0);
    }
}
