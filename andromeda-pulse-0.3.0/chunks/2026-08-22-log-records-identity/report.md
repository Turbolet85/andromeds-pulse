# Report — 2026-08-22-log-records-identity

**Chunk:** log_records identity — two log records in the same tick at the same severity both survive ingestion
**Date:** 2026-08-23T00:10Z
**Commits:** (none yet — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/buffer/src/schema.rs` — `seq BIGINT NOT NULL` + 4-column PK in BOTH DDL representations (`:92` const, `:186` live `SCHEMA_DDL`); new test `log_records_primary_key_includes_seq_ordinal`.
  - `crates/buffer/src/state.rs` — `log_seq: AtomicU64` field + `pub fn reserve_log_seq_block`.
  - `crates/buffer/src/appender.rs` — `build_logs_record_batch` assigns `seq`; Arrow `Field` + `Int64Array` appended LAST; new test `append_logs_batch_same_tick_same_severity_keeps_both_records` + helper `colliding_log_record`.
  - `crates/viz/src/query.rs` — test-fixture DDL (`:500`) + both INSERTs (`:545`, `:597`) + `SEED_LOG_SEQ`/`next_log_seq()`.
  - `crates/triage/src/baseline/sql.rs` — test-fixture DDL (`:887`) + `insert_log` (`:973`) + `SEED_LOG_SEQ`.
  - `crates/ingest/examples/inject_colliding_logs.rs` — NEW dev-only OTLP producer (live-leg vehicle).
  - `pulse-app/ui/src/bindings/index.ts` — regenerated; **operational cleanup, not chunk-functional** (see Deviations).

- **Symbols / APIs:**
  - NEW `pub fn BufferState::reserve_log_seq_block(&self, n: u64) -> u64` (`crates/buffer/src/state.rs`) — reserves `n` contiguous ordinals in one `fetch_add`. **Callers: exactly one** — `build_logs_record_batch` (`appender.rs`). Deliberately NOT surfaced on `BufferStateSnapshot`.
  - CHANGED `build_logs_record_batch` — internal (`pub(crate)`); signature UNCHANGED (already took `&BufferState`). **Remaining callers unchanged and still live:** `consumer::dispatch_batch` (`consumer.rs:138`, production) + the `#[cfg(test)]` wrapper `append_logs_batch`. Not a sole-caller claim: the graph query returned `rows=11` across both.
  - NEW dev-only binary target `inject_colliding_logs` (cargo example in `ingest`). Not shipped in any bundle.
  - **No TauRPC procedure added, renamed, or removed.** `EXPECTED_PROCEDURES` untouched; no capability-JSON edit.
  - **No port, socket, or env var added or changed.**

- **Crates / modules:** none added or removed. Changed: `buffer` (schema + state + appender), `viz` (test fixture only), `triage` (test fixture only), `ingest` (new example target only — no `src/` change).

- **Dependencies:** **none added, none bumped.** No `Cargo.toml` / `Cargo.lock` delta.

- **Schema / config:** DuckDB `log_records` gains column `seq BIGINT NOT NULL`; PRIMARY KEY widens `(ts_unix_nano, resource_hash, severity_number)` → `(ts_unix_nano, resource_hash, severity_number, seq)`. **No migration and no schema-version bump** — the ring buffer is in-memory (`:memory:`, `pulse_buffer`), created on startup per arch §Established Decisions [ORM / Migrations]. Retention still keys on `ts_unix_nano` alone (`retention.rs:27`), unaffected. Reserved-table count UNCHANGED at 8. No config key added.

- **Spec-master edits:** **none** — this chunk edited no `.andromeda/` master. The three expected amendments are listed under *Spec claims disproved by measurement* for P2 to dispose.

- **Counts / qualifiers moved:**
  - Workspace nextest **1833 → 1835** (+1 skip unchanged). Exactly +2: the behavioural collision test and the PK contract test. Stated in `.claude/session-handoff.md` (previous wrap).
  - `buffer` crate tests **158 → 160**.
  - `log_records` column count 9 → 10; its PK column count 3 → 4.
  - **UNCHANGED and verified:** the 8 reserved DuckDB tables (arch §Occupied Resources); the 8 P-047 scrubber categories; the TauRPC procedure list.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** none — nothing was written then withdrawn. Recording one deliberate NON-surface: `seq` was kept OFF `BufferStateSnapshot` and off `buffer.tick` by design, so it never became an observable (verified live: the string `seq` appears 0 times in the smoke obs log).

- **Spec claims disproved by measurement:**
  1. **arch §Conventions → *Primary key convention*** states "metric points and log records use OTLP-native identity (timestamp + resource hash + name)". **`log_records` has no `name` column at all** (columns: `ts`, `ts_unix_nano`, `resource_hash`, `severity_number`, `body`, `severity_text`, `trace_id`, `span_id`, `template_id`, and now `seq`). Measured false, and **already inaccurate at HEAD independent of this chunk**. Evidence: `crates/buffer/src/schema.rs:82-93`. *(Expected amendment.)*
  2. **`crates/buffer/src/schema.rs:311-318`** defers behavioural PK enforcement — "schema introspection is the contract assertion, **not** behavioral PK enforcement. Behavioral enforcement is exercised at the appender path (chunk #22+ when queries land)." **That deferral is now discharged**: the Appender path enforces the PK, measured. The same comment's duplicate-INSERT hang claim was **not** re-tested and is neither confirmed nor refuted — only the Appender path was exercised, and it did not hang (0.09s). Do not read this as retiring the INSERT-path caution.
  3. **The working-route entry's own premise** — "silently drops same-tick same-severity records" — is false in SHAPE. Measured: a hard `PRIMARY KEY or UNIQUE constraint violation` at `flush()`, propagated out of `dispatch_batch` and logged at ERROR on `duckdb.append` with a `reject_reason`; **zero** rows landed, so the loss is whole-batch, not per-record. Already corrected in `scope.md` §Premise closure.
  4. **The same entry's "the SAME PK shape recurs at `:185`, so check both tables"** — one table, TWO representations (const + live `SCHEMA_DDL`), guarded by `ddl_constants_match_concatenated_schema`. Corrected in `scope.md`; not a finding per operator directive.
  5. **test-plan.md §1 Pending coverage triggers** records `buffer` as having no in-crate test run (basis of the `buffer-redaction-counter-unit-coverage` trigger). **`cargo nextest run -p buffer` ran this chunk (160/160)**, so `buffer`'s zero-in-crate-run status ends — though that trigger itself (the redaction counter) is **NOT** discharged. *(Expected amendment.)*
  6. **obs-plan §10** module-boundary mandate — **checked, no change needed**: the boundary DOES emit at ERROR (`consumer.rs:78-83`, live-verified). Recorded so the check is visible rather than skipped.

- **Coverage of new surfaces:**
  - `log_records.seq` (DuckDB column) → validation n/a (internal ordinal, not an external input) · instrumentation n/a (**deliberately** not an observable — see Reverted/negative) · PII n/a (monotonic integer, carries no user content) · tests unit✓ (`log_records_primary_key_includes_seq_ordinal` asserts the 4-column PK; `append_logs_batch_same_tick_same_severity_keeps_both_records` asserts the behaviour) · a11y n/a · tokens n/a
  - `BufferState::reserve_log_seq_block` (new pub fn) → validation n/a · instrumentation n/a (deliberate) · PII n/a · tests unit✓ **indirect only** — exercised through the collision test; **no direct unit test of its own contract** (monotonicity / block-width). In-crate coverage exists, so this is NOT a repeat of the prior chunk's zero-in-crate-tests gap, but the allocator's own contract is unpinned. · a11y n/a · tokens n/a
  - `inject_colliding_logs` (dev-only cargo example, not bundled) → validation n/a · instrumentation n/a · PII✓ (bodies chosen credential-free; both canaries and the service label verified **absent** from the obs log) · tests — it IS the live-leg vehicle, exercised end-to-end against the real receiver · a11y n/a · tokens n/a

## Deviations from intent

1. **Column position was not fixed by the plan.** Step 4 said to place the Arrow `Field` "in the same position as the DDL column" while step 5 never fixed a DDL position — the two steps refer to each other. **Justification:** resolved from repo precedent rather than invented — `template_id`, the previous column added to this same table, was appended last, and appending keeps every existing column position stable for the positional Arrow→DuckDB append and the `pulse://stream/logs` broadcast encode. Surfaced rather than silently interpreted.
2. **Fixture `seq` value source was unspecified.** The plan said "a corresponding bound value". **Justification:** used a per-call `AtomicI64` in both fixtures. This is what the plan's own step-8 note asked for in `triage` ("threading an explicit per-call `seq` removes that latent flake") — `insert_log` timestamps from a real wall clock and is called repeatedly with identical resource+severity, so a coarse timer could collide *within the fixture*. Applied the same shape to `viz`, whose helpers use a fixed `resource_hash`.
3. **`pulse-app/ui/src/bindings/index.ts` regenerated — OUTSIDE the plan's touchpoint list.** **Justification:** `cargo xtask capability-drift` is a gate in this plan and it was RED before the chunk began — commit `70344d5` committed the file in the no-mcp shape (verified: worktree byte-identical to HEAD before the regen, so this chunk did not cause it). The artifact is GENERATED, the remediation is one documented command, and the project carries a curated rule (`rules/testing.md` 2026-05-17) prescribing exactly this recovery and stating the regenerated file rides the current chunk's commit. Recorded, not absorbed.

## Decisions & corrections

- **Operator decision (P4 dialogue, 2026-08-22) — key design.** Chose the **monotonic sequence column** over (a) a per-batch ordinal mirroring `span_events.event_index` and (b) dropping the PK. Rationale that emerged while composing the question and reshaped the fork: `event_index` is safe only because its key carries `(trace_id, span_id)`, a unique parent scoping the ordinal; `log_records` has **no unique parent**, so a per-batch reset leaves two same-nanosecond records in different export batches both at index 0 — still colliding. A literal reading of the arch extract's "follow the `event_index` precedent" would have shipped a half-fix.
- **Operator decision (P4) — fixture scope.** Both out-of-crate fixture DDLs updated so those suites stop exercising the pre-fix identity.
- **Operator correction (P5 review) — the fixture INSERTs are UNCONDITIONAL.** The plan had written them as conditional ("if the fixture takes the column as `NOT NULL`") — a predicate that **can never be false**, since `seq` is a PRIMARY KEY column and a PK column cannot be NULL. Read literally by `/implement` this would have licensed skipping all three INSERT sites, and because these are **SQL strings inside Rust the compiler cannot catch a missed one** — it surfaces only in a test run that exercises that fixture path. Corrected to an exhaustive three-site list, plus a grep-based acceptance criterion and a guard against the `DEFAULT` shortcut that would have hidden the same miss.
- **Design decision — `seq` is an allocator, not an observable.** Kept off `BufferStateSnapshot`, `buffer.tick`, the `viz` `SELECT_LOGS` column list, and the MCP response shape; adding it to any of those would mint an obs field requiring the three-site amendment plus its own exact allowlist leaf, for an internal counter. Verified live.
- **Measure-first was mechanical, not procedural.** The post-fix assertion was written first and run against unfixed code; its failure output *is* the measurement, and the same committed test then proves the fix. Had it passed cold, the chunk would have closed as a premise correction.

## Outcome

**Acceptance criteria: met.** All 13 criteria discharged, including the two that needed explicit discharge rather than assertion: the grep criterion (exactly 3 INSERT sites, each naming `seq`; 4 DDL sites, zero 3-column PKs remaining) and the bounded-probe criterion (the collision leg runs on a worker thread with a 30s `recv_timeout`, so a hang fails rather than wedges, and uses no duplicate-`INSERT` probe).

**Gates green** (commands as run): `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` (exit 0) · `cargo nextest run -p buffer` (160/160) · `cargo nextest run --workspace --profile ci` (**1835/1835 + 1 skip**) · `cargo xtask capability-drift` (clean, after the regen) · `cargo xtask capability-widening-check` (clean, 0 violations / 3 inspected) · `cargo deny check bans licenses sources` (ok) · `cargo deny check advisories` (**designed-RED at exactly the 8 owned upgradeable IDs — 0189/0190/0194/0195/0204/0222/0253/0258 — stable, no new finding**).

**`cargo audit`:** probe **skipped per the ratified interval (next: 34)** — this wrap is session 33. Never silent. Basis UNCHANGED and upstream (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`); overlap re-derived this run and stable at eight.

**Smoke (live leg, direct-binary variant — the boot-path did not change; this ran because the chunk's own acceptance needs a live receiver):** PASS and discriminating.
- Feed precondition asserted FIRST: `rows_ingested` **0 → 2**.
- Positive: `duckdb.append` `table_name=log_records` `rows_appended=2` `duration_ms=4`.
- `0` ERROR · `0` `app.panic.fatal` · `0` `appender returned error` · `0` `PRIMARY KEY` violations.
- Obs PII: both body canaries and the `collision-probe` service label appear **0** times; the string `seq` appears **0** times anywhere in the log.
- 6 tick families; `buffer.tick` max consecutive gap **15.0s** ≤ 45s budget.
- Clean shutdown by specific PID; `:4317` and `:4318` both released; no orphan processes.
- Pre-fix, this same producer would have logged `appender returned error` and left `rows_ingested` at 0 — the smoke discriminates rather than merely passing.

**Capability claim: NONE** — all 22 matrix entries swept; none covers log-record identity, PK collision, or ingestion-time record loss. The coverage gate is a no-op for this chunk.
