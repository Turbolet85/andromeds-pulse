# arch extract — phase-66

## Chunk relevance

Chunk #69 Phase B has strong arch domain coverage across multiple dimensions:
- New TauRPC namespace (`diagnostics.*`) triggers the 4+1-place binding pattern
- New crate-level module (`crates/buffer/drain.rs`) touches module dependency direction rules
- `log_templates` table + `log_records.template_id` column extension touches database entity naming + Occupied Resources
- Config extension (`[triage.drain.*]`) touches config management cross-cutting pattern
- Arch registry delta required: +1 TauRPC procedure, +1 schema table, +1 schema column
- Integration at `crates/buffer/appender.rs` hot path touches module boundary discipline and channel architecture
- `pulse-app/ui/diagnostics/TemplateDistribution.tsx` touches webview IPC capability policy

## Constraints

**Module dependency direction** (§Cross-cutting Patterns "Module dependency direction"):
- `crates/buffer/drain.rs` is a new module inside `buffer` crate — dependencies must flow TOWARD `pulse-app`. `buffer` MUST NOT introduce a dependency on `triage`, `corpus`, or `ui-bridge`. The Drain algorithm + persistence serialization belongs entirely inside `buffer` crate boundaries, with `corpus` injected as a parameter/trait rather than a hard dep edge from buffer→corpus.
- Persistence to corpus SQLite (chunk #68 substrate) must be wired at `pulse-app` boundary level, not inside `buffer` crate internals — use the trait-in-lower-crate + impl-in-pulse-app pattern (per CLAUDE.md §Session Additions 2026-05-16 entry).

**DuckDB query discipline** (§Critical Warnings + §Established Decisions [Database]):
- `log_templates` table queries MUST use prepared statements with `?` placeholders — NEVER `format!()` interpolation. Applies to both template lookup and template_id write-back.

**TauRPC 4+1-place binding** (CLAUDE.md §Session Additions 2026-05-12):
- `diagnostics.template_distribution()` requires all five wiring points: (1) router registration in owning crate, (2) `pulse-app/capabilities/` JSON entry, (3) `xtask/src/main.rs::EXPECTED_PROCEDURES` extension, (4) `emit_taurpc_bindings` test merge inclusion, AND the arch §Occupied Resources update via `/andromeda-evolve --allow-arch-registry`.

**Tauri capability negative-default** (§Cross-cutting Patterns "Webview IPC capability policy"):
- `diagnostics.template_distribution()` MUST have a matching `pulse-app/capabilities/` JSON entry before the procedure is callable; missing entry = silent runtime rejection.

**Config management precedence** (§Cross-cutting Patterns "Config management"):
- Drain params (`depth=4`, `similarity=0.5`, `max_clusters=1000`) must honor env vars > `config.toml` > built-in defaults layering. Restart-required notice surface per P-055 does not alter this precedence — it only gates the UI warning.

**Occupied Resources — reserved table names** (§Occupied Resources "DuckDB database / schema names"):
- `log_records` is the production reserved name (not "logs"). New `log_templates` table and `log_records.template_id` column must be added to §Occupied Resources reserved tables list via `/andromeda-evolve --allow-arch-registry`.

**Error handling at bridge** (§Conventions "Error response schema (Tauri IPC)"):
- `diagnostics.template_distribution()` returns `Result<T, AppError>`. Any drain-internal error type must convert to `AppError::Internal` or `AppError::Storage` via free-function map_err at the router boundary (per CLAUDE.md §Session Additions 2026-05-18 orphan-rule pattern) — NOT via `impl From<DrainError> for AppError` in buffer or a shared crate.

## Patterns to follow

**Trait-in-lower-crate + impl-in-pulse-app for cross-crate state/persistence** (CLAUDE.md §Session Additions 2026-05-16):
- Drain persistence to corpus SQLite follows this pattern: define a persistence trait (e.g., `DrainPersistence`) in `crates/buffer/`; implement the corpus-backed adapter in `pulse-app/src/drain_persistence.rs`; inject `Arc<dyn DrainPersistence>` into the Drain struct constructor. This preserves the arch DAG (buffer→corpus dep edge avoided).

**Free-function map_err for cross-crate errors** (CLAUDE.md §Session Additions 2026-05-18):
- `pub fn drain_error_to_app_error(err: DrainError) -> AppError { ... }` in `pulse-app/src/diagnostics_router.rs`. Callers use `.map_err(drain_error_to_app_error)?`.

**D3 capability-drift coupling** (CLAUDE.md §Session Additions 2026-05-10):
- The `diagnostics.*` EXPECTED_PROCEDURES extension + emit_taurpc_bindings test update MUST land in the same commit as the router registration + capability JSON — not as standalone housekeeping.

**Schema evolution via `PRAGMA user_version`** (§Established Decisions [Database]):
- `log_templates` table + `log_records.template_id` column addition follows the idempotent migration pattern established at chunk #68 (`corpus` crate) — bump schema version, ADD COLUMN IF NOT EXISTS, CREATE TABLE IF NOT EXISTS.

**Two-phase chunk wrap-state discipline** (CLAUDE.md §Session Additions + session-handoff.md):
- Phase B completes chunk #69; `state.yaml.last_completed_chunk` advances to 69 only when Phase B is done. Intermediate wrap commits use `chore(wrap):` prefix to avoid D6 false fire.

**PII scrubber integration point** (§Critical Warnings + §Modules `security`):
- Template content passes through `security::scrub_attribute()` BEFORE persistence to `log_templates`. This is the same defense-in-depth layer used by `corpus` at chunk #68 ingestion.

## Anti-patterns to avoid

**Reverse dependency edges** (§Cross-cutting Patterns "Module dependency direction"):
- NEVER add `buffer` → `corpus`, `buffer` → `ui-bridge`, or `buffer` → `triage` dep edges. The Drain module lives in `buffer` and must stay Tauri-free and corpus-free in its core — persistence is injected, not hard-linked.

**Format-string SQL** (§Critical Warnings):
- NEVER `format!("SELECT ... WHERE template_id = '{}'", id)` for `log_templates` or `log_records` queries.

**Unregistered TauRPC procedure** (§Cross-cutting Patterns "Webview IPC capability policy"):
- NEVER merge `diagnostics.template_distribution()` without all 5 binding points AND the arch §Occupied Resources update.

**Orphan rule workaround via reverse dep** (CLAUDE.md §Session Additions 2026-05-18):
- NEVER put `impl From<DrainError> for AppError` in `buffer` or `ui-bridge` — use free function in router file.

**Floating self-observation export** (§Critical Warnings "Self-observation"):
- Drain metric emission (`metric.pipeline.l1c.drain_template_count_total`, `metric.pipeline.l1c.drain_assignment_latency_p99_microseconds`) MUST use `tracing` events only — NEVER OTel SDK / OTLP network exporter pointed at `:4317`/`:4318`.

**Schema name collision** (§Occupied Resources):
- Do NOT use `logs` as table name for the log records table — reserved name is `log_records`. New table is `log_templates` (not `drain_templates` or `templates`).

## Contract bindings

**Arch ↔ tests**: `diagnostics.template_distribution()` must be exercised via TauRPC roundtrip test (per arch §Cross-cutting Patterns "Test-time telemetry injection" + standard gate discipline). Golden-file tests for Drain algorithm live in `crates/buffer/`; integration test exercises the full path log ingest → template assignment → `log_records.template_id` populated → `diagnostics.template_distribution()` returns correct counts.

**Arch ↔ obs**: Phase A forward-referenced metric names (`metric.pipeline.l1c.drain_template_count_total`, `metric.pipeline.l1c.drain_assignment_latency_p99_microseconds`) must be emitted as `tracing` structured events matching obs-plan §Metric naming convention; these are arch-registered forward-refs from `.andromeda/decisions/pre-d2-drain-spike.md`.

**Arch ↔ security**: Template content PII scrub (via `security::scrub_attribute()`) is a contract between `corpus`/`buffer` integration and the `security` crate — arch §Modules `security` establishes this as defense-in-depth; the scrub MUST occur at the persistence boundary, not only at the TauRPC response boundary.

**Arch ↔ design/frontend**: `TemplateDistribution.tsx` consumes `diagnostics.template_distribution()` TauRPC — TypeScript binding shape is arch-determined (auto-generated by TauRPC derive macros; `tsc --noEmit` gate must pass). The Settings → Diagnostics panel must route through the existing Settings modal pattern (established at chunk #42) per arch §Conventions "Workspace API style."

**Arch ↔ registry update**: Upon Phase B completion, `/andromeda-evolve --allow-arch-registry` must record the `diagnostics.template_distribution` TauRPC route + `log_templates` table + `log_records.template_id` column in §Occupied Resources and §Architecture Registry Updates. This is the final Epoch 9 arch registry delta.

## Acceptance criteria contributions

1. **Module dependency graph preserved**: `cargo tree -p buffer` shows zero new edges to `corpus`, `ui-bridge`, or `triage` — Drain persistence is injected via trait, not hard-linked.
2. **TauRPC 4+1-place binding complete**: `cargo xtask capability-drift` exits 0 with `diagnostics.template_distribution` present in `EXPECTED_PROCEDURES`, `pulse-app/capabilities/` JSON, `emit_taurpc_bindings` test router merge, and arch §Occupied Resources (via evolve registry entry).
3. **Schema in reserved names**: `log_templates` table present in DuckDB schema; `log_records.template_id` column populated for log records post-Drain assignment; no `logs` table alias used.
4. **Prepared statements only**: `cargo clippy --workspace -- -D warnings` passes; no `format!()` string construction for `log_templates`/`log_records` SQL paths visible in source review.
5. **Error bridge clean**: `diagnostics_router.rs` uses free-function `drain_error_to_app_error` — no `impl From<DrainError> for AppError` in `buffer` or `ui-bridge` crates.
6. **Arch registry updated**: `git log --oneline | head -5` shows a wrap commit or implementation commit containing `/andromeda-evolve --allow-arch-registry` acknowledgment for `diagnostics.*` + `log_templates` + `log_records.template_id`; `state.yaml.last_completed_chunk` = 69 at Phase B completion.
7. **Self-observation metric events use tracing only**: No `opentelemetry` / OTLP SDK import in `crates/buffer/drain.rs` or any module introduced by this chunk — self-observation for Drain metrics uses `tracing::info!(metric.pipeline.l1c.drain_template_count_total = ...)` structured events.
