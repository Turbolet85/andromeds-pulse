//! Corpus error type — chunk #68.
//!
//! `thiserror`-derived. Sanitized messages — no file paths, library
//! versions, or stack traces leak across the TauRPC bridge (per arch
//! §Conventions Error response schema + §Established Decisions
//! [Error Handling Pattern]). Conversion to `ui_bridge::contract::AppError`
//! lives at the binary boundary in `pulse-app/src/storage_router.rs`.

use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("keychain backend unavailable")]
    KeyringUnavailable,
    #[error("schema migration failed")]
    MigrationFailed,
    #[error("encryption operation failed")]
    EncryptionFailed,
    #[error("decryption failed")]
    DecryptionFailed,
    #[error("query failed")]
    QueryFailed,
    #[error("path traversal rejected")]
    PathTraversal,
    #[error("file size exceeds cap")]
    SizeCapExceeded,
    #[error("schema version mismatch")]
    SchemaVersionMismatch,
    #[error("io error: {kind:?}")]
    Io { kind: io::ErrorKind },
}

impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Self {
        Error::QueryFailed
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io { kind: e.kind() }
    }
}
