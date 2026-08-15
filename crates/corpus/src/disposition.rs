//! Disposition of content orphaned by a key change.
//!
//! Recovery of content written under a lost key is cryptographically closed,
//! so the only honest disposition is retirement. The inventory is written
//! BEFORE anything is deleted: table/column metadata is plaintext by design
//! (security-plan.md §Data Protection → At rest), so the record of what was
//! lost survives even though the content cannot.
//!
//! Purge is not optional cleanup. `service_registry` is the one reserved table
//! with no `payload BLOB`, so it is never orphaned and never touched.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::contract::Corpus;
use crate::encryption::cell_decrypt;
use crate::error::Error;

/// Reserved tables holding an encrypted `payload BLOB`. `service_registry` is
/// absent by design — it stores no BLOB, so a lost key never orphans it.
const PAYLOAD_TABLES: &[&str] = &[
    "baseline_state",
    "pipeline_metrics",
    "incidents",
    "incident_events",
    "digest_archive",
];

/// Deletion order. `incident_events` references `incidents(id)`, so children
/// go first or the foreign key rejects the parent delete.
const PURGE_ORDER: &[&str] = &[
    "incident_events",
    "incidents",
    "baseline_state",
    "pipeline_metrics",
    "digest_archive",
];

/// Outcome of one disposition pass. Mirrors the `MigrationOutcome` shape at
/// `pulse-app/src/baseline_persistence.rs` — rows are deleted only on FULL
/// success, so a failed pass leaves the corpus intact for the next boot.
#[derive(Debug, PartialEq, Eq)]
pub enum DispositionOutcome {
    /// No undecryptable rows — the common path once disposition has run.
    Noop,
    /// Inventory written, then the orphaned rows deleted.
    Completed {
        inventory_path: PathBuf,
        rows_purged: u64,
        tables_affected: u64,
    },
    /// Failed before deletion. Every row is preserved.
    Failed { error_category: &'static str },
}

/// Plaintext columns of an orphaned incident. Readable without the key, which
/// is what makes the inventory possible at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanedIncident {
    pub id: i64,
    pub workspace: String,
    pub status: String,
    pub created_unix_nano: i64,
    pub updated_unix_nano: i64,
    pub resolved_unix_nano: Option<i64>,
    pub read_unix_nano: Option<i64>,
}

/// What a scan found, before anything is written or deleted.
#[derive(Debug, Default)]
pub struct OrphanScan {
    pub per_table: BTreeMap<String, Vec<i64>>,
    pub incidents: Vec<OrphanedIncident>,
}

impl OrphanScan {
    pub fn total_rows(&self) -> u64 {
        self.per_table.values().map(|ids| ids.len() as u64).sum()
    }

    pub fn tables_affected(&self) -> u64 {
        self.per_table
            .values()
            .filter(|ids| !ids.is_empty())
            .count() as u64
    }

    pub fn is_empty(&self) -> bool {
        self.total_rows() == 0
    }
}

/// Scan every payload-bearing table for rows this corpus's key cannot decrypt.
pub fn scan_orphaned(corpus: &Corpus) -> Result<OrphanScan, Error> {
    let conn = corpus.connection();
    let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
    let mut scan = OrphanScan::default();

    for table in PAYLOAD_TABLES {
        let ids = scan_table(&guard, corpus, table)?;
        if *table == "incidents" && !ids.is_empty() {
            scan.incidents = load_incident_metadata(&guard, &ids)?;
        }
        scan.per_table.insert((*table).to_string(), ids);
    }
    Ok(scan)
}

fn scan_table(conn: &Connection, corpus: &Corpus, table: &str) -> Result<Vec<i64>, Error> {
    // `table` comes only from PAYLOAD_TABLES, never from caller input — the
    // DuckDB/SQLite no-interpolation rule binds VALUES, and an identifier
    // cannot be a bound parameter.
    let sql = format!("SELECT id, payload FROM {table} ORDER BY id");
    let mut stmt = conn.prepare(&sql).map_err(|_| Error::QueryFailed)?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?))
        })
        .map_err(|_| Error::QueryFailed)?;

    let mut orphaned = Vec::new();
    for row in rows {
        let (id, encrypted) = row.map_err(|_| Error::QueryFailed)?;
        if cell_decrypt(corpus.key(), &encrypted).is_err() {
            orphaned.push(id);
        }
    }
    Ok(orphaned)
}

fn load_incident_metadata(conn: &Connection, ids: &[i64]) -> Result<Vec<OrphanedIncident>, Error> {
    let mut stmt = conn
        .prepare(
            "SELECT id, workspace, status, created_unix_nano, updated_unix_nano, \
             resolved_unix_nano, read_unix_nano FROM incidents WHERE id = ?1",
        )
        .map_err(|_| Error::QueryFailed)?;
    let mut out = Vec::with_capacity(ids.len());
    for id in ids {
        let row = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok(OrphanedIncident {
                    id: row.get(0)?,
                    workspace: row.get(1)?,
                    status: row.get(2)?,
                    created_unix_nano: row.get(3)?,
                    updated_unix_nano: row.get(4)?,
                    resolved_unix_nano: row.get(5)?,
                    read_unix_nano: row.get(6)?,
                })
            })
            .map_err(|_| Error::QueryFailed)?;
        out.push(row);
    }
    Ok(out)
}

/// Render the inventory. Everything here is already plaintext in the corpus,
/// so the artifact exposes nothing the database did not.
pub fn render_inventory(scan: &OrphanScan, generated_unix_nano: i64) -> String {
    let mut out = String::new();
    out.push_str("# Orphaned corpus inventory\n\n");
    out.push_str(
        "Rows whose encrypted payload could not be decrypted by the current key. Recovery is \
         cryptographically closed; this artifact is the surviving record of what was retired.\n\n",
    );
    out.push_str(&format!(
        "Generated (unix nanos): {generated_unix_nano}\n\n"
    ));
    out.push_str("| table | orphaned rows |\n|---|---|\n");
    for (table, ids) in &scan.per_table {
        out.push_str(&format!("| {} | {} |\n", table, ids.len()));
    }
    out.push_str("| service_registry | 0 (no encrypted payload) |\n");
    out.push_str(&format!("\nTotal rows: {}\n", scan.total_rows()));

    if !scan.incidents.is_empty() {
        out.push_str("\n## Incidents (plaintext columns, readable without the key)\n\n");
        out.push_str(
            "| id | workspace | status | created | updated | resolved | read |\n\
             |---|---|---|---|---|---|---|\n",
        );
        for inc in &scan.incidents {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                inc.id,
                inc.workspace,
                inc.status,
                inc.created_unix_nano,
                inc.updated_unix_nano,
                inc.resolved_unix_nano
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                inc.read_unix_nano
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ));
        }
    }
    out
}

/// Inventory, then purge. Nothing is deleted unless the inventory is safely on
/// disk first, so a failure here costs a retry rather than the record.
pub fn dispose_orphaned_content(
    corpus: &Corpus,
    inventory_dir: &Path,
    generated_unix_nano: i64,
) -> DispositionOutcome {
    let scan = match scan_orphaned(corpus) {
        Ok(scan) => scan,
        Err(_) => {
            return DispositionOutcome::Failed {
                error_category: "scan_failed",
            };
        }
    };
    if scan.is_empty() {
        return DispositionOutcome::Noop;
    }

    let rendered = render_inventory(&scan, generated_unix_nano);
    let inventory_path = inventory_dir.join(format!("orphaned-inventory-{generated_unix_nano}.md"));
    if write_atomic(&inventory_path, &rendered).is_err() {
        return DispositionOutcome::Failed {
            error_category: "inventory_write_failed",
        };
    }

    let rows_purged = match purge(corpus, &scan) {
        Ok(n) => n,
        Err(_) => {
            return DispositionOutcome::Failed {
                error_category: "purge_failed",
            };
        }
    };

    DispositionOutcome::Completed {
        inventory_path,
        rows_purged,
        tables_affected: scan.tables_affected(),
    }
}

/// Aggregate-only emission for one disposition pass — counts and a bounded
/// outcome tag, never a row id, workspace, path, or payload (obs-plan.md §5).
pub fn emit_disposition_outcome(outcome: &DispositionOutcome) {
    match outcome {
        DispositionOutcome::Noop => {}
        DispositionOutcome::Completed {
            rows_purged,
            tables_affected,
            ..
        } => {
            tracing::warn!(
                target: "corpus.orphan.disposition",
                disposition_outcome = "completed",
                rows_purged = *rows_purged,
                tables_affected = *tables_affected,
                "orphaned corpus rows inventoried then purged",
            );
        }
        DispositionOutcome::Failed { error_category } => {
            tracing::warn!(
                target: "corpus.orphan.disposition",
                disposition_outcome = "failed",
                error_category = *error_category,
                "orphaned-row disposition failed; rows preserved for retry",
            );
        }
    }
}

fn purge(corpus: &Corpus, scan: &OrphanScan) -> Result<u64, Error> {
    let conn = corpus.connection();
    let mut guard = conn.lock().map_err(|_| Error::QueryFailed)?;
    let tx = guard.transaction().map_err(|_| Error::QueryFailed)?;
    let mut purged = 0u64;
    for table in PURGE_ORDER {
        let Some(ids) = scan.per_table.get(*table) else {
            continue;
        };
        // Identifier from PURGE_ORDER only; the id is a bound parameter.
        let sql = format!("DELETE FROM {table} WHERE id = ?1");
        for id in ids {
            purged += tx
                .execute(&sql, rusqlite::params![id])
                .map_err(|_| Error::QueryFailed)? as u64;
        }
    }
    tx.commit().map_err(|_| Error::QueryFailed)?;
    Ok(purged)
}

fn write_atomic(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("md.tmp");
    fs::write(&tmp, contents)?;
    fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tempfile::TempDir;

    use super::*;
    use crate::contract::{Corpus, CorpusReader, FakeKeychainBackend, KeychainBackend};

    const KEY_A: [u8; 32] = [0xA1u8; 32];
    const KEY_B: [u8; 32] = [0xB2u8; 32];

    fn backend(key: [u8; 32]) -> Arc<dyn KeychainBackend> {
        Arc::new(FakeKeychainBackend::with_seeded_key("corpus-key", key))
    }

    /// Seeds a corpus under `KEY_A` with rows in every payload table plus one
    /// `service_registry` row, then reopens it under `KEY_B` so every payload
    /// row is orphaned.
    fn corpus_orphaned_under_a_new_key(path: &Path) -> Corpus {
        {
            let corpus = Corpus::open(path.to_path_buf(), backend(KEY_A)).expect("open under A");
            let conn = corpus.connection();
            let guard = conn.lock().expect("lock");
            guard
                .execute(
                    "INSERT INTO incidents (workspace, status, created_unix_nano, updated_unix_nano, payload) \
                     VALUES (?, ?, ?, ?, ?)",
                    rusqlite::params!["ws-a", "active", 1_000i64, 1_000i64, &b"cipher"[..]],
                )
                .expect("incident");
            guard
                .execute(
                    "INSERT INTO incident_events (incident_id, event_kind, occurred_unix_nano, payload) \
                     VALUES (?, ?, ?, ?)",
                    rusqlite::params![1i64, "created", 1_000i64, &b"cipher"[..]],
                )
                .expect("incident event");
            guard
                .execute(
                    "INSERT INTO digest_archive (digest_kind, assembled_unix_nano, token_count, payload) \
                     VALUES (?, ?, ?, ?)",
                    rusqlite::params!["l2", 1_000i64, 10i64, &b"cipher"[..]],
                )
                .expect("digest");
            guard
                .execute(
                    "INSERT INTO baseline_state (service_name, operation, snapshot_unix_nano, payload) \
                     VALUES (?, ?, ?, ?)",
                    rusqlite::params!["svc", Option::<String>::None, 1_000i64, &b"cipher"[..]],
                )
                .expect("baseline");
            guard
                .execute(
                    "INSERT INTO pipeline_metrics (metric_name, layer, snapshot_unix_nano, payload) \
                     VALUES (?, ?, ?, ?)",
                    rusqlite::params!["m", "l1", 1_000i64, &b"cipher"[..]],
                )
                .expect("metric");
            guard
                .execute(
                    "INSERT INTO service_registry (service_name, state, first_seen_unix_nano, last_seen_unix_nano, last_transition_unix_nano) \
                     VALUES (?, ?, ?, ?, ?)",
                    rusqlite::params!["svc", "live", 1_000i64, 1_000i64, 1_000i64],
                )
                .expect("registry");
        }
        Corpus::open(path.to_path_buf(), backend(KEY_B)).expect("reopen under B")
    }

    #[test]
    fn scan_finds_every_payload_table_and_never_service_registry() {
        let tmp = TempDir::new().expect("tmp");
        let corpus = corpus_orphaned_under_a_new_key(&tmp.path().join("c.db"));

        let scan = scan_orphaned(&corpus).expect("scan");

        assert_eq!(scan.total_rows(), 5);
        assert_eq!(scan.tables_affected(), 5);
        assert!(!scan.per_table.contains_key("service_registry"));
    }

    #[test]
    fn scan_reads_incident_metadata_without_the_key() {
        let tmp = TempDir::new().expect("tmp");
        let corpus = corpus_orphaned_under_a_new_key(&tmp.path().join("c.db"));

        let scan = scan_orphaned(&corpus).expect("scan");

        assert_eq!(scan.incidents.len(), 1);
        assert_eq!(scan.incidents[0].workspace, "ws-a");
        assert_eq!(scan.incidents[0].status, "active");
        assert_eq!(scan.incidents[0].created_unix_nano, 1_000);
    }

    #[test]
    fn disposition_writes_the_inventory_then_purges_and_spares_service_registry() {
        let tmp = TempDir::new().expect("tmp");
        let corpus = corpus_orphaned_under_a_new_key(&tmp.path().join("c.db"));
        let sink = tmp.path().join("sink");

        let outcome = dispose_orphaned_content(&corpus, &sink, 4_242);

        let DispositionOutcome::Completed {
            inventory_path,
            rows_purged,
            tables_affected,
        } = outcome
        else {
            panic!("expected Completed, got {outcome:?}");
        };
        assert_eq!(rows_purged, 5);
        assert_eq!(tables_affected, 5);

        let rendered = fs::read_to_string(&inventory_path).expect("inventory readable");
        assert!(rendered.contains("ws-a"), "incident metadata retained");

        let counts = corpus.inspect().expect("inspect").record_counts;
        for table in PAYLOAD_TABLES {
            assert_eq!(counts.get(*table), Some(&0), "{table} should be purged");
        }
        assert_eq!(
            counts.get("service_registry"),
            Some(&1),
            "service_registry holds no encrypted payload and must be untouched"
        );
    }

    #[test]
    fn disposition_preserves_every_row_when_the_inventory_cannot_be_written() {
        let tmp = TempDir::new().expect("tmp");
        let corpus = corpus_orphaned_under_a_new_key(&tmp.path().join("c.db"));

        // A regular file where the sink directory must go — create_dir_all
        // fails, so the inventory never lands.
        let blocked = tmp.path().join("blocked");
        fs::write(&blocked, b"not a directory").expect("blocker");

        let outcome = dispose_orphaned_content(&corpus, &blocked, 4_242);

        assert_eq!(
            outcome,
            DispositionOutcome::Failed {
                error_category: "inventory_write_failed"
            }
        );
        let counts = corpus.inspect().expect("inspect").record_counts;
        assert_eq!(
            counts.get("incidents"),
            Some(&1),
            "rows preserved for retry"
        );
        assert_eq!(counts.get("incident_events"), Some(&1));
    }

    #[test]
    fn disposition_is_a_noop_when_nothing_is_orphaned() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("clean.db");
        let corpus = Corpus::open(path, backend(KEY_A)).expect("open");

        let outcome = dispose_orphaned_content(&corpus, &tmp.path().join("sink"), 1);

        assert_eq!(outcome, DispositionOutcome::Noop);
    }

    #[test]
    fn disposition_is_idempotent_across_repeated_boots() {
        let tmp = TempDir::new().expect("tmp");
        let corpus = corpus_orphaned_under_a_new_key(&tmp.path().join("c.db"));
        let sink = tmp.path().join("sink");

        let first = dispose_orphaned_content(&corpus, &sink, 1);
        let second = dispose_orphaned_content(&corpus, &sink, 2);

        assert!(matches!(first, DispositionOutcome::Completed { .. }));
        assert_eq!(second, DispositionOutcome::Noop);
    }

    #[test]
    fn rendered_inventory_names_service_registry_as_unaffected() {
        let scan = OrphanScan::default();
        let rendered = render_inventory(&scan, 7);

        assert!(rendered.contains("service_registry | 0 (no encrypted payload)"));
        assert!(rendered.contains("Total rows: 0"));
    }
}
