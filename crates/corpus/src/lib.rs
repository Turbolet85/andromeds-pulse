//! Persistent incident corpus — chunk #68.
//!
//! SQLite-backed persistent storage for v0.2.0 capability cluster
//! P-041/P-047–P-051 (Persistent Incident Corpus + PII Scrubbing at
//! Ingestion + No Raw OTLP Attribute Values Stored + Encryption at Rest
//! + Cross-Project Sharing Opt-In default + Transparent Storage).
//!
//! # Distinction from `triage::baseline::corpus`
//!
//! This crate (`crates/corpus/`) is the **incident corpus** — SQLite
//! file storing distilled incident records / digest archive / lifecycle
//! state for v0.2.0+ chunks (#70 incidents, #71+ digest pipeline, #74
//! LLM retrieval, etc.).
//!
//! `triage::baseline::corpus` (introduced chunk #61) is the **baseline
//! corpus** — bincode-serialized in-memory `BaselineState` persisted to
//! a single `.bin` file for restart-recovery of streaming baseline
//! trackers. Different concept, different on-disk format, different
//! purpose. The two never share storage.
//!
//! # Architecture
//!
//! - [`contract`] — public surface ([`Corpus`] struct, [`CorpusReader`]
//!   trait, [`InspectionMetadata`] envelope, [`Error`] enum,
//!   [`KeychainBackend`] re-export)
//! - encryption is per-cell AES-256-GCM (key sourced from OS keychain
//!   via the `KeychainBackend` trait); table/column metadata is visible
//!   without the key but cell payloads are opaque
//! - schema versioning via `PRAGMA user_version`; first-launch idempotent
//!   migration

pub mod contract;
pub mod disposition;

pub(crate) mod db;
pub(crate) mod encryption;
pub(crate) mod error;
pub(crate) mod keychain;
pub(crate) mod schema;
