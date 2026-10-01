# `corpus` — Persistent Incident Corpus (SQLite)

## Responsibility
SQLite-backed persistent storage layer (chunk #68 — Epoch 9 Foundation v0.2.0). Six schema tables per dist-arch v3 for baseline state, service registry, pipeline metrics, incidents, incident events, and digest archive. Cell-level AES-256-GCM encryption with key sourced from OS keychain. Capabilities P-041 (Persistent Corpus) + P-047–P-051 (PII scrubbing categories at ingestion).

## Key integrations

### Consumes from
- `security::scrubber::mask_secret_spans` (span-level masking gated by `scrub_attribute`'s verdict, since chunk 2026-09-30-span-level-redaction) for PII scrubbing at ingestion boundary (defense at corpus write; observability subscriber Layer scrubbing deferred to chunk #70+).
- OS keychain via `keyring` crate (macOS Keychain / Linux Secret Service / Windows DPAPI) for encryption key retrieval at first-launch + restoration on subsequent boots.

### Publishes to
- `pulse-app::storage_router::StorageApiImpl` consumes `CorpusReader::inspect()` for `storage.inspect` TauRPC + `CorpusReader::path()` for `storage.path` TauRPC (chunk #68 quadruple binding: router + capability JSON + EXPECTED_PROCEDURES + emit_taurpc_bindings test).
- Future write surface (chunks #64 / #66 / #70+): `triage::baseline::persist_state` writes BaselineState snapshots; `triage::pattern::storm` writes fingerprint counts; chunk #70+ incident write surface.

### Dependencies
- `rusqlite v0.32.1` (bundled feature — statically links SQLite C; chunk #68 dep addition).
- `keyring v3.x` (cross-platform OS keychain; chunk #68 dep addition).
- `aes-gcm v0.10.x` (RustCrypto AEAD for cell-level encryption; chunk #68 dep addition).
- `rand v0.8.x` (nonce generation per encrypted cell; chunk #68 dep addition).
- Intra-workspace: `crates/security` (PII scrubber path dep).

## Internal conventions
- **Module layout:** `lib.rs` (re-exports) / `contract.rs` (`CorpusReader` trait + `InspectionMetadata` struct + `TableRecordCount` + 234-line public surface) / `db.rs` (`Corpus` connection root + rusqlite wrapper) / `schema.rs` (DDL for 6 tables + PRAGMA user_version idempotent migration) / `encryption.rs` (`encrypt_cell` / `decrypt_cell` with AES-256-GCM + 12-byte nonce) / `keychain.rs` (`KeychainBackend` trait + `OsKeychainBackend` + `FakeKeychainBackend` for tests + `BackendKind` 5-variant enum for bounded-cardinality obs) / `error.rs` (`Error` 9-variant `thiserror`-derived enum).
- **Public contract:** `corpus::contract` re-exports `Corpus` / `CorpusReader` / `InspectionMetadata` / `TableRecordCount` / `Error` / `KeychainBackend` / `OsKeychainBackend` / `FakeKeychainBackend` / `BackendKind` / `KeychainError`.
- **Schema tables (5 reserved; SCHEMA_VERSION 2):** `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive` — `baseline_state` DROPPED at chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep via the v1→v2 ladder migration (fresh→v2 · v1→`DROP TABLE IF EXISTS baseline_state`+stamp · newer→mismatch). `incident_events` gains its first production writer the same chunk: `update_incident_status` records one lifecycle row per status VALUE-change inside the guarded transaction (bounded `event_kind`, empty encrypted payload; same-status refresh and `DeclinedStale` record nothing).
- **Migration:** PRAGMA `user_version` = schema version int; first-launch creates all 6 tables; subsequent launches verify version + apply diffs.
- **Encryption posture:** Phase 6 default = cell-level AES-256-GCM with OS-keychain-stored key (alternatives documented: SQLCipher whole-database, age full-file).
- **Cross-crate error mapping:** `pulse-app::storage_router::corpus_error_to_app_error()` free function (orphan-rule sidestep per security.md Session Additions 2026-05-18) maps `corpus::Error` → `AppError` at binary boundary.

## Service-specific gotchas
- **First-launch keychain prompt:** `OsKeychainBackend::get_or_create_encryption_key()` may surface an OS-level keychain unlock prompt on macOS / Linux; tests use `FakeKeychainBackend` to bypass.
- **Concurrent access:** rusqlite default connection serializes via internal mutex; no parallel writers. If write throughput becomes a concern, document migration to `Arc<Mutex<Connection>>` + `tokio::task::spawn_blocking` pattern.
- **`storage.{inspect,path}` are read-only by design** — `pulse-app/capabilities/default.json` notes "read-only-by-design per capability P-051"; no write methods exposed via TauRPC.
- **Path canonicalization:** `corpus_db_path = data_dir.join("corpus").join("corpus.db")` resolved via the `std` both-sides-canonicalize boot path per security.md's universal path-env-var rule (the project's single path primitive since 2026-08-29 — `strict-path` dropped, never used).

## Entry points for modification
- **Public contract:** `crates/corpus/src/contract.rs` (only `pub` surface beyond `lib.rs` re-exports)
- **Schema changes:** `crates/corpus/src/schema.rs` (DDL + migration logic; bump PRAGMA user_version)
- **Encryption tweaks:** `crates/corpus/src/encryption.rs` (cell envelope: nonce + ciphertext + auth tag)
- **Keychain backend:** `crates/corpus/src/keychain.rs` (add per-platform variants as needed)
- **TauRPC envelopes:** `pulse-app/src/storage_router.rs` (`StorageInspectPayload` / `StoragePathPayload` + `StorageApi` trait)
- **Tests:** 35 corpus tests (chunk #68 landed); cover schema, encryption, keychain (fake), inspection metadata.
