# security extract — phase-67

## Chunk relevance

Chunk #70 (BaselineState → corpus migration) is HIGHLY relevant к security domain. The migration:
- Eliminates a plaintext bincode flat-file persistence path (`<data_dir>/triage/baseline-corpus.bin`) — closes the audit Section 1.A persistence triple-mechanism finding
- Couples BaselineState into the corpus crate's AES-256-GCM cell-level encryption + OS keychain key custody (P-047 — chunk #68 substrate)
- Adds new boundary: corpus write/read error → AppError mapping at the resolver boundary (no new TauRPC delta, but `BaselinePersistence` trait crosses crate boundaries)
- Migration path (read-once + delete legacy bincode file) introduces a transient filesystem read of plaintext baseline state with PII potential (service names + latency baselines are NOT PII by spec but redaction discipline still applies к log emissions)
- Touches existing tracing emissions (`triage.baseline.persist` info event + `pipeline.l1b.persist_count_total` counter + new `triage.baseline.persist.error` event)

## Constraints

- **C1 (path canonicalization):** Legacy bincode file path resolution (`<data_dir>/triage/baseline-corpus.bin`) MUST canonicalize via `strict-path` and verify resolution under the resolved data dir before reading-to-migrate or deleting. Per security plan §Input Validation row "Configuration values" + §Security Anti-Patterns §Input ban "NEVER read `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env vars without `std::path::Path::canonicalize()` + an assertion that the canonical path starts under the resolved per-platform data dir" — CWE-22 defense. Migration code reading the legacy file inherits the same canonicalization discipline.

- **C2 (AES-256-GCM + keychain custody preservation):** BaselineState routed through `CorpusWriter` MUST exercise the existing chunk #68 cell-level AES-256-GCM encryption + OS-keychain key custody — DO NOT introduce a parallel encryption-bypass write path. Per security plan §Data Protection ("baseline state now inherits corpus AES-256-GCM encryption + PII scrub-before-encrypt contract — couples with #72") as noted in chunk detail Specialist plan touches.

- **C3 (error sanitization at boundary):** Corpus write/read errors crossing into pulse-app resolver → `AppError::Storage { message }` MUST sanitize per §Error Handling rules: no stack traces, no file paths (especially the legacy bincode path), no library versions, no Rust struct names. The new `triage.baseline.persist.error` event field MUST contain a sanitized reason (per chunk detail "emit `triage.baseline.persist.error` with sanitized reason field for corpus write failures").

- **C4 (logging redaction):** New `triage.baseline.persist.error` tracing event MUST NOT log raw baseline values (per-operation latency percentiles, error counts) — service.name is acceptable cardinality; ALL OTLP-attribute-derived values are forbidden per §Logging & Monitoring NEVER-log list ("OTLP attribute values" + "DuckDB query parameters") + §Security Anti-Patterns §Logging. Log query identifier + operation count + bucket count instead.

- **C5 (legacy file deletion safety):** After successful migration to corpus, deletion of the legacy `<data_dir>/triage/baseline-corpus.bin` MUST canonicalize the path AND verify the file lives under the resolved data dir before `std::fs::remove_file` — same CWE-22 defense applied к ANY path operation on user-overridable data dir. A symlinked `baseline-corpus.bin` pointing к `/etc/shadow` outside the canonicalized data dir MUST be refused, not deleted.

- **C6 (cross-crate error mapping discipline):** Per session-learnings 2026-05-18 (already in CLAUDE.md security rules), the new `BaselinePersistence` trait error → `AppError::Storage` mapping at `pulse-app/src/baseline_persistence.rs` boundary MUST use the free-function `corpus_error_to_app_error` pattern (or equivalent) — NEVER `impl From<corpus::Error> for AppError` (orphan rule violation + reverses arch DAG). Mirrors chunk #69 `CorpusDrainPersistence` precedent.

## Patterns к follow

- **P1 (trait-in-lower-crate + adapter-at-pulse-app):** `trait BaselinePersistence` defined in `crates/triage/src/baseline/` (lower crate) with `CorpusBaselinePersistence` adapter in `pulse-app/src/baseline_persistence.rs` mirroring chunk #69 `CorpusDrainPersistence` over `Arc<dyn CorpusWriter>` precedent. Per CLAUDE.md session-learnings 2026-05-16 + 2026-05-19 + chunk detail Specialist plan touches.

- **P2 (Arc<dyn Trait> dual-role from single concrete):** Existing chunk #69 pattern (one `Arc<Corpus>` → both `Arc<dyn CorpusReader>` for storage.inspect AND `Arc<dyn CorpusWriter>` for baseline + drain persistence) preserved. BaselineState consumes ONLY the writer view (read-back happens during boot init within the same trait surface). Per CLAUDE.md session-learnings 2026-05-19.

- **P3 (sanitized tracing fields per obs discipline):** The new `triage.baseline.persist.error` event follows the existing `pipeline.l1b.persist_count_total` counter + `triage.baseline.persist` info event field shapes — `service` (string, cardinality-controlled), `kind` ("ewma"|"tdigest"|"rolling_window"|"activity_floor"), `bucket_count` (u32). No persisted baseline values in event fields.

- **P4 (capability-drift discipline preserved):** Chunk has TauRPC delta = none (per chunk detail "TauRPC delta: none"); no new `pulse-app/capabilities/` JSON additions required. `cargo xtask capability-drift` MUST remain clean post-chunk. No EXPECTED_PROCEDURES updates needed. Per CLAUDE.md security rules §Tauri capability gating + session-learnings 2026-05-09.

## Anti-patterns к avoid

- **A1:** NEVER write baseline state к the legacy `<data_dir>/triage/baseline-corpus.bin` flat-file path after migration ships — the migration's whole purpose is eliminating this plaintext bincode path. The dual-write transitional pattern is rejected; closure of audit Section 1.A persistence triple-mechanism requires single-source corpus persistence. Per chunk detail Summary ("Eliminate the BaselineState flat-file persistence mechanism").

- **A2:** NEVER log the legacy bincode file path verbatim in the migration warn-log emission — log only the existence-detected event with sanitized field (e.g., `triage.baseline.migration_detected` with `bytes: u64` + `legacy_path_basename: String` where basename is `"baseline-corpus.bin"` literal, not the full canonicalized path). Per §Security Anti-Patterns §Logging "NEVER log full plugin file paths — basename of canonicalized path only" extended к this triage subpath.

- **A3:** NEVER serialize the new `BaselinePersistence::Error` type directly across the TauRPC bridge — chunk has no new TauRPC routes, but if a downstream chunk adds a baseline-state-inspection IPC (e.g., diagnostics surface), the error type MUST convert through `AppError::Storage { message }` via free-function mapping per §Security Anti-Patterns §Code Patterns "NEVER serialize an `anyhow::Error` directly across the TauRPC bridge" + 2026-05-18 session-learnings entry.

- **A4:** NEVER bypass the corpus AES-256-GCM encryption layer with a "performance optimization" plaintext write — the bincode format is preserved (chunk reuses `pipeline_metrics` flat blob slot OR populates `baseline_state` table; either way blob bytes are encrypted at the cell level by the existing chunk #68 encryption layer). Per §Data Protection "baseline state now inherits corpus AES-256-GCM encryption".

## Contract bindings

- **Binds к obs:** The new `triage.baseline.persist.error` tracing event field schema cross-cuts §Logging & Monitoring redaction rules (this plan) + obs-plan §PII Scrubbing (sibling). Both domains agree: error reason field is sanitized (no stack trace, no internal struct names, no library versions); cardinality-bounded fields (`service`, `kind`) are acceptable. Obs domain owns the `pipeline.l1b.persist_count_total` counter shape; security domain owns the redaction contract.

- **Binds к tests:** Per chunk detail Specialist plan touches: "replace existing bincode round-trip test at `crates/triage/src/baseline/corpus.rs:299-317` with corpus round-trip restart-and-reload test; preserve proptest coverage on streaming math." Security domain contributes 2 test triggers:
  - Test: migration warn-log fires + legacy file removed when legacy bincode file exists pre-boot
  - Test: corpus write failure mapping → `AppError::Storage` sanitization (no internal paths / struct names in `message` field)

- **Binds к arch:** Per chunk detail "Removal of `triage/baseline-corpus.bin` subpath relevant for #74 batch (subpath was implicitly forward-promised but never registered per audit Dim 4)." Security domain notes that the registry omission is a cross-cutting audit cleanup deferred к chunk #74, not blocking chunk #70.

- **Binds к chunk #72 (PII scrubber coverage extension):** Per chunk detail "couples with #72" — baseline state values flowing into corpus encryption inherit chunk #72's PII scrub-before-encrypt contract WHEN that chunk lands. Chunk #70 itself does NOT introduce PII scrubber wiring on the BaselineState write path (the data shape — service names + numeric percentile/error metrics — has low PII risk vs. log templates / span attributes); the scrub-before-encrypt contract attaches at chunk #72.

## Acceptance criteria contributions

- **(security #1)** `cargo deny check bans licenses sources` passes against the new dependency graph (no new dep introduction expected, but verify Cargo.lock delta does NOT introduce duplicates). Per §Dependency Security CI integration.

- **(security #2)** Migration path canonicalizes legacy `<data_dir>/triage/baseline-corpus.bin` via `strict-path` (or equivalent canonicalize + ancestor-assertion primitive) BEFORE read or delete; verified via grep for `canonicalize` adjacent к the migration code site + manual review that no raw `std::fs::read("baseline-corpus.bin")` exists.

- **(security #3)** New `triage.baseline.persist.error` tracing event field schema redacts internal details: `kind` field is enum (`ewma`|`tdigest`|`rolling_window`|`activity_floor`), no `path` field, no `error_chain` field, no `library_version` field. Verified via grep over `tracing::error!` / `tracing::warn!` call sites in `crates/triage/src/baseline/` matching the new event name.

- **(security #4)** Migration warn-log message (legacy file detected on boot) emits basename only (`"baseline-corpus.bin"`), never full canonicalized path. Verified via grep for the warn-log emission + structured-field inspection.

- **(security #5)** `BaselinePersistence::Error` → `AppError` mapping (if used at any future resolver) uses free-function pattern (`baseline_error_to_app_error` or extends existing `corpus_error_to_app_error`), NEVER `impl From<...> for AppError`. Verified via grep over `pulse-app/src/baseline_persistence.rs` for `impl From<` patterns (should be zero).
