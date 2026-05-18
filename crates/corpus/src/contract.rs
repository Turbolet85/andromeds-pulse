//! Public corpus contract — chunk #68.
//!
//! Defines the [`Corpus`] handle struct, [`CorpusReader`] trait for
//! cross-crate state delivery into TauRPC resolvers (per session-learnings
//! 2026-05-16 trait-in-lower-crate pattern), [`InspectionMetadata`]
//! envelope, and re-exports of [`Error`] + [`KeychainBackend`] for
//! crate-level convenience.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub use crate::error::Error;
pub use crate::keychain::{
    BackendKind, FakeKeychainBackend, KeychainBackend, KeychainError, OsKeychainBackend,
};

use crate::db;
use crate::encryption::EncryptionKey;
use crate::schema::{SCHEMA_VERSION, TABLE_NAMES};

/// Per-table record count + on-disk byte size + schema version. Returned
/// by `Corpus::inspect()` + the `storage.inspect` TauRPC procedure.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionMetadata {
    pub record_counts: BTreeMap<String, u64>,
    pub total_bytes_on_disk: u64,
    pub schema_version: u32,
}

/// Corpus handle — wraps the rusqlite Connection + tracks the resolved
/// on-disk path + the loaded encryption key. Construct via
/// [`Corpus::open`] (on-disk) or [`Corpus::open_in_memory`] (tests).
pub struct Corpus {
    conn: Arc<Mutex<Connection>>,
    path: PathBuf,
    schema_version: u32,
    #[allow(dead_code)]
    key: EncryptionKey,
}

impl Debug for Corpus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Corpus")
            .field("path_basename", &basename(&self.path))
            .field("schema_version", &self.schema_version)
            .finish()
    }
}

impl Corpus {
    /// Open (or create) the corpus at `path`. Fetches the encryption
    /// key from the keychain backend on first call; subsequent opens с
    /// the same backend reuse the same key. Runs first-launch schema
    /// migration if the file is new.
    pub fn open(path: PathBuf, keychain: Arc<dyn KeychainBackend>) -> Result<Self, Error> {
        let key_bytes = keychain
            .fetch_or_create_key("corpus-key")
            .map_err(|_| Error::KeyringUnavailable)?;
        let key = EncryptionKey::new(key_bytes);
        let conn = db::open_at_path(&path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            path,
            schema_version: SCHEMA_VERSION,
            key,
        })
    }

    /// Open an in-memory corpus с the given keychain backend. Used by
    /// tests + the bindings emission flow.
    pub fn open_in_memory(keychain: Arc<dyn KeychainBackend>) -> Result<Self, Error> {
        let key_bytes = keychain
            .fetch_or_create_key("corpus-key")
            .map_err(|_| Error::KeyringUnavailable)?;
        let key = EncryptionKey::new(key_bytes);
        let conn = db::open_in_memory()?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            path: PathBuf::from(":memory:"),
            schema_version: SCHEMA_VERSION,
            key,
        })
    }

    /// Path returned by [`CorpusReader::path`]. `:memory:` for the
    /// in-memory variant.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Schema version currently applied. Always [`SCHEMA_VERSION`] for
    /// a freshly-opened corpus; surfaces in [`InspectionMetadata`].
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Encryption key accessor for crate-internal write/read paths
    /// (chunk #68 scaffold defers write/read to chunks #70+).
    #[allow(dead_code)]
    pub(crate) fn key(&self) -> &EncryptionKey {
        &self.key
    }

    /// Shared connection handle for crate-internal queries. Not exposed
    /// publicly — callers go through `CorpusReader` instead.
    pub(crate) fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }
}

/// Read-only corpus operations consumed by the TauRPC resolver layer.
/// Trait-in-lower-crate pattern per session-learnings 2026-05-16 —
/// resolvers в `pulse-app` hold `Arc<dyn CorpusReader>`.
pub trait CorpusReader: Send + Sync + Debug {
    /// Per-table record counts + total file size + schema version.
    fn inspect(&self) -> Result<InspectionMetadata, Error>;

    /// Returns the corpus DB filesystem path.
    fn path(&self) -> PathBuf;
}

impl CorpusReader for Corpus {
    fn inspect(&self) -> Result<InspectionMetadata, Error> {
        let conn = self.connection();
        let guard = conn.lock().map_err(|_| Error::QueryFailed)?;
        let mut counts = BTreeMap::new();
        for table in TABLE_NAMES {
            let n = db::count_table(&guard, table)?;
            counts.insert((*table).to_string(), n);
        }
        Ok(InspectionMetadata {
            record_counts: counts,
            total_bytes_on_disk: db::file_size_bytes(&self.path),
            schema_version: self.schema_version,
        })
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

fn basename(path: &Path) -> String {
    path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(":memory:")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_corpus() -> Corpus {
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
        Corpus::open_in_memory(backend).expect("in-memory corpus")
    }

    #[test]
    fn open_in_memory_returns_corpus_with_default_schema_version() {
        let corpus = test_corpus();
        assert_eq!(corpus.schema_version(), SCHEMA_VERSION);
        assert_eq!(corpus.path(), Path::new(":memory:"));
    }

    #[test]
    fn inspect_returns_zero_counts_for_fresh_corpus() {
        let corpus = test_corpus();
        let meta = corpus.inspect().expect("inspect");
        assert_eq!(meta.schema_version, SCHEMA_VERSION);
        for table in TABLE_NAMES {
            assert_eq!(meta.record_counts.get(*table), Some(&0));
        }
    }

    #[test]
    fn inspect_includes_all_six_tables() {
        let corpus = test_corpus();
        let meta = corpus.inspect().expect("inspect");
        assert_eq!(meta.record_counts.len(), TABLE_NAMES.len());
        for table in TABLE_NAMES {
            assert!(meta.record_counts.contains_key(*table));
        }
    }

    #[test]
    fn inspect_reflects_inserted_rows() {
        let corpus = test_corpus();
        {
            let conn = corpus.connection();
            let guard = conn.lock().expect("lock");
            guard
                .execute(
                    "INSERT INTO incidents (workspace, status, created_unix_nano, updated_unix_nano, payload) VALUES (?, ?, ?, ?, ?)",
                    rusqlite::params!["ws", "active", 1_000i64, 1_000i64, &b"x"[..]],
                )
                .expect("insert");
        }
        let meta = corpus.inspect().expect("inspect");
        assert_eq!(meta.record_counts.get("incidents"), Some(&1));
    }

    #[test]
    fn open_on_disk_persists_across_close_and_reopen() {
        let tmp = TempDir::new().expect("tmp");
        let path = tmp.path().join("persist-test.db");
        let backend_key = [0x77u8; 32];
        {
            let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
                "corpus-key",
                backend_key,
            ));
            let corpus = Corpus::open(path.clone(), backend).expect("open");
            let conn = corpus.connection();
            let guard = conn.lock().expect("lock");
            guard
                .execute(
                    "INSERT INTO incidents (workspace, status, created_unix_nano, updated_unix_nano, payload) VALUES (?, ?, ?, ?, ?)",
                    rusqlite::params!["ws-a", "active", 2_000i64, 2_000i64, &b"x"[..]],
                )
                .expect("insert");
        }
        // Reopen with the SAME seeded key → row persists.
        let backend2: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::with_seeded_key(
            "corpus-key",
            backend_key,
        ));
        let corpus2 = Corpus::open(path.clone(), backend2).expect("reopen");
        let meta = corpus2.inspect().expect("inspect");
        assert_eq!(meta.record_counts.get("incidents"), Some(&1));
        assert!(meta.total_bytes_on_disk > 0);
    }

    #[test]
    fn debug_does_not_leak_full_path() {
        let tmp = TempDir::new().expect("tmp");
        let secret_segment = "extremely-distinctive-segment-do-not-leak";
        let path = tmp.path().join(secret_segment).with_extension("db");
        let parent = path.parent().expect("parent");
        std::fs::create_dir_all(parent).expect("mkdir");
        let backend: Arc<dyn KeychainBackend> = Arc::new(FakeKeychainBackend::new());
        let corpus = Corpus::open(path.clone(), backend).expect("open");
        let dbg = format!("{corpus:?}");
        // Full canonicalized path MUST NOT appear; only basename (which
        // contains the segment) is acceptable. Verify path-parent
        // components don't leak.
        if let Some(parent_str) = parent.to_str() {
            assert!(
                !dbg.contains(parent_str),
                "Debug output leaked parent path: {dbg}"
            );
        }
    }

    #[test]
    fn corpus_reader_trait_dispatch_works() {
        let corpus = test_corpus();
        let reader: Arc<dyn CorpusReader> = Arc::new(corpus);
        let meta = reader.inspect().expect("inspect via trait");
        assert_eq!(meta.schema_version, SCHEMA_VERSION);
    }
}
