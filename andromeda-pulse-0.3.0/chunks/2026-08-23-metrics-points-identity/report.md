# Report — 2026-08-23-metrics-points-identity

**Chunk:** metrics_points identity — two data points of one metric differing only by label set both survive ingestion, with their labels
**Date:** 2026-08-23
**Commits:** (none since `last_wrap` 2026-08-23T07:35:00Z — this chunk's work is uncommitted at report time)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/buffer/src/schema.rs` (M) · `crates/buffer/src/state.rs` (M) · `crates/buffer/src/appender.rs` (M) · `crates/buffer/src/consumer.rs` (M) · `crates/viz/src/query.rs` (M)
  - `crates/ingest/examples/inject_colliding_metrics.rs` (NEW — dev-only OTLP producer, `[[example]]`, not shipped in the binary)
  - +526 / −62 across the five modified files.

- **Symbols / APIs:**
  - **NEW public boundary** `BufferState::reserve_metric_seq_block(&self, n: u64) -> u64` (`crates/buffer/src/state.rs`) — monotonic buffer-global allocator for the `metrics_points.seq` ordinal, mirroring `reserve_log_seq_block`. Deliberately NOT an observable (absent from `BufferStateSnapshot`, `buffer.tick`, every read path).
  - **NEW private field** `BufferState.metric_seq: AtomicU64`.
  - **Signature change, all four record-batch builders** (`crates/buffer/src/appender.rs`, all `pub(crate)`): return type `Result<Option<RecordBatch>, Error>` → `Result<Option<(RecordBatch, u64)>, Error>` — the second element is the batch's redaction tally, which the builders now RETURN instead of folding into `BufferState`.
    - `build_spans_record_batch` ALSO **dropped its `state: &BufferState` parameter** (arity 2 → 1): its only use of `state` was the fold that moved out.
    - `build_metrics_record_batch` / `build_logs_record_batch` / `build_span_events_record_batch` keep `state` (metrics for the new ordinal, logs for `reserve_log_seq_block`, span_events for `record_feed_counts`).
  - **Remaining-caller facts** (from the P3 code-graph query, rust plane, `db_state: fresh`, 11 rows — NOT a "sole caller" default): the four builders' callers are ALL in-crate and all were updated — `consumer::dispatch_batch` (the production path, `consumer.rs:108/124/132/145`), the three `#[cfg(test)]` wrappers `append_{spans,metrics,logs,span_events}_batch`, and 6 in-crate test call sites. `push_metric_row` retains its five callers inside `collect_metric_points` (`appender.rs:671, 688, 705, 722, 739`) — **unchanged**, see Deviations. `reserve_log_seq_block` retains its single caller `build_logs_record_batch` (`appender.rs:276`) — unchanged.
  - **No IPC/TauRPC procedure, endpoint, port, socket, or env var added or changed.** `capability-drift` clean (0 missing / 0 extra); `EXPECTED_PROCEDURES` untouched.

- **Crates / modules:** none added or removed. Changed: `buffer` (`schema` · `state` · `appender` · `consumer`), `viz` (`query` test fixture only — no production `viz` code changed), `ingest` (new `examples/` target only).

- **Dependencies:** **none added, none bumped.** No `Cargo.toml` / `Cargo.lock` delta.

- **Schema / config:**
  - `metrics_points` gains column `seq BIGINT NOT NULL`.
  - `metrics_points` PRIMARY KEY widens 3 → 4 columns: `(metric_name, ts_unix_nano, resource_hash)` → `(metric_name, ts_unix_nano, resource_hash, seq)`.
  - Applied in **all three** DDL representations: `schema.rs:69` (`CREATE_METRICS_POINTS` const), `schema.rs:168` (the live `SCHEMA_DDL` concat), and the independent `crates/viz/src/query.rs:481` test fixture. The first two are pinned to each other by `ddl_constants_match_concatenated_schema`; the third has **no drift guard** (`viz` has no `buffer` dependency) and was edited by hand.
  - No migration: the ring buffer is in-memory and created at startup.
  - No config keys, no violation schemas.

- **Spec-master edits:** none at report time — the four expected amendments are listed under *Spec claims disproved by measurement* and are P2's to apply.

- **Counts / qualifiers moved:**
  - Workspace test count **1859 → 1870** (+11 new; a 12th changed name is the pre-existing `redaction_counter_folds_once_per_batch_across_all_four_builders`, re-pointed not added). Stated in the previous chunk's report/handoff only, not in a spec master.
  - `metrics_points` primary key column count **3 → 4** — stated in `architecture.md:85` §Conventions → Primary key convention.
  - `redactions_applied` fold SITE moved from the four builders to the four append sites — the "persisted cells ONLY" qualifier at `obs-plan.md:356` is now structurally true rather than aspirational.
  - `buffer` in-crate coverage gaps **1 → 0** — `test-plan.md:126` `buffer-log-seq-allocator-unit-coverage` is closed by this chunk; `test-plan.md:332` states it as "the sole remaining in-crate `buffer` gap".

- **Dev-tool versions:** none — no external CLI installed or upgraded.

- **Reverted / negative API facts:**
  - The plan's step-5 mechanism (thread the ordinal through `collect_metric_points` + `push_metric_row` + all five call sites) was **considered and not taken** — the `log_records` precedent (reserve one block after collection, index by position) achieves the same with zero changes to those two functions. See Deviations.
  - Two source mutations were applied and **fully reverted** as the plan's step-8 mutation check: (a) `seq` removed from the metrics PK in both `schema.rs` representations, (b) the metrics `record_redactions` fold moved above its append in `consumer.rs`. Both restored and re-verified green; neither is in the diff.

- **Spec claims disproved by measurement** (each needs disposition; none applied yet):
  1. **`architecture.md:85` §Conventions → Primary key convention** states "metric points use OTLP-native identity (`metric_name` + `ts_unix_nano` + `resource_hash`)". Measured false as a *sufficient* identity and changed by this chunk: those three columns cannot separate two points of one metric differing only by label set (the table stores no attributes column), so the key is now four columns including `seq`. The same sentence's "`seq` is the one declared exception" clause now covers a second table.
  2. **`security-plan.md:429` §Security Anti-Patterns → Logging** states the `metrics_points.metric_name` redaction collision is "an accepted, LOUD failure (ERROR at `flush()`), never a silent one". Measured true at HEAD (one `duckdb.append` ERROR, `reject_reason: "append_failed"`, `rows_ingested` 0) and **deliberately removed** by this chunk — with the ordinal in the key the pair no longer collides, so there is no failure to be loud about. The residual worth recording: two distinct credential-shaped names still redact to one placeholder and now land indistinguishable — no new leak (redaction holds), but the conflation loses its only signal.
  3. **`obs-plan.md:356`** states `redactions_applied` "Counts redactions to PERSISTED cells ONLY". Measured FALSE at HEAD on the live path: the RED leg recorded `redactions_applied: 2` with `rows_ingested: 0` — the fold fired inside the builders (`appender.rs:73/184/273/468`) before the append could fail, so a rejected batch counted cells nothing stored. This chunk moves the fold to each table's own post-append site, making the stated rule true.
  4. **`test-plan.md:270` §3 Bootstrap phases item 6** mandates a `MockMetricPoint` builder in the `viz` crate (restated at `test-plan.md:552`). Measured absent — a workspace grep for `MockMetricPoint` / `MockArrowBatch` returns zero hits. **Not fixed here** (out of scope); recorded for disposition.

- **Coverage of new surfaces:**
  - `metrics_points.seq` (new DuckDB column) → validation `n/a` (internal ordinal, not an external input) · instrumentation `n/a` (deliberately non-observable per the arch `seq` convention) · PII `n/a` (monotonic integer, carries no client content) · tests `unit` (`schema::tests::metrics_points_primary_key_includes_seq_ordinal` + 3 appender behavioural pins) · a11y `n/a` · tokens `n/a`
  - `BufferState::reserve_metric_seq_block` (new public boundary) → validation `n/a` (takes a `u64` width from the batch length) · instrumentation `n/a` (allocator, deliberately absent from `buffer.tick`) · PII `n/a` · tests `unit` (4 direct in-crate assertions + 1 concurrency test; the sibling `reserve_log_seq_block` gained the same, closing its trigger) · a11y `n/a` · tokens `n/a`
  - `crates/ingest/examples/inject_colliding_metrics.rs` (dev-only OTLP producer, not shipped) → validation `n/a` (emits, does not receive) · instrumentation `n/a` · PII `redacted✓` (synthetic `provider_key`-shaped literals only, never real credentials; verified absent from the obs log — 0 occurrences of either literal and 0 of the bare `sk_live_` form across 14,215 GREEN-run log lines) · tests — it IS the measuring instrument for the RED/GREEN legs · a11y `n/a` · tokens `n/a`
  - **No UI element, no interactive surface, no external network surface added.** Zero `pulse-app/ui/**` paths touched.

## Deviations from intent

1. **Ordinal threading mechanism** — plan step 5 directed threading the ordinal through `collect_metric_points` and `push_metric_row` and all five call sites. **Justification:** the `log_records` precedent the same plan names as the shape to reuse ("reuse the shape, not redesign") reserves one block AFTER collection and indexes it by position, needing zero changes to either function. Identical outcome, strictly smaller blast radius, exact precedent match. The five `push_metric_row` call sites are therefore unchanged.
2. **`build_spans_record_batch` lost its `state` parameter** — not named in the plan. **Justification:** once its redaction fold moved to the append site, `state` had no remaining use; leaving it would be an unused parameter (a clippy failure under `-D warnings`) or dead weight behind an `_` prefix. Every call site was already in the modify list.
3. **Two pre-existing tests were re-pointed** (`redaction_counter_folds_once_per_batch_across_all_four_builders`, `clean_batch_leaves_the_redaction_counter_at_zero`) — they pinned the OLD contract (builders fold into `state`) that this chunk deliberately changes. **Justification:** per the 2026-08-17 discipline they were STRENGTHENED, not relaxed — the re-pointed pin now additionally asserts `state.redactions_applied == 0` after building, which is a stronger guard (it pins the persisted-cells rule) than the fold-count it replaced.
4. **The label half is NOT delivered** — the working entry's title asks that points survive "with their labels"; this chunk delivers identity only. **Justification:** operator decision at the P4 gate (CARRY selected from three presented options), recorded in `scope.md` §The design tension. Research established the halves are fully separable — the ordinal closes both collisions without `p.attributes` ever being read.
5. **The `redactions_applied` fold fix was added to scope** — not in the working entry. **Justification:** operator decision at the same P4 gate (fix-in-chunk selected over CARRY); research measured it, and this chunk owns the harness that proves it.

## Decisions & corrections

- **P4 operator decision — label half CARRIES.** Three options were presented (CARRY / storage-only / full). CARRY selected. The follow-up entry must carry this chunk's evidence: the table stores no attributes column, `p.attributes` is never read at `push_metric_row`, and after this chunk the points survive but arrive label-less — identity closed, information loss open.
- **P4 operator decision — redaction over-count fixed in-chunk** rather than CARRY'd to the Diagnostics sweep. Consequence accepted: the RED leg loses `redactions_applied` as a divergence signature, so the RED evidence pins `rows_ingested` + the ERROR line instead. (In the event the RED leg captured the over-count too, because it ran at HEAD before the fix.)
- **Sequencing was load-bearing and held.** The plan required the RED measurement at HEAD *before* the fold fix. Followed: the RED run captured both defects (collision AND over-count) in one leg. Had the fold been fixed first, the over-count would have been unobservable.
- **Correction during the smoke leg** — a `tasklist /FI "PID eq N"` liveness probe was mangled by Git Bash path conversion (`Invalid argument/option - 'C:/Program Files/Git/FI'`), and its `|| echo` fallback printed a "not running" conclusion *for the wrong reason*. Caught and re-verified via PowerShell `Get-Process` before drawing any conclusion. Same host-shell class as the documented `taskkill /F` trap; the new facet is that a zero-is-healthy `||` fallback converts a command error into a plausible negative finding.
- **Harness-truth observation (not this chunk's to fix)** — `scripts/agent-run.sh status` exits 0 and emits a full health payload while nothing is running, naming a stale pid with `uptime_ms: 0` (verified: both ports closed, no `pulse-app` process). A gate keyed on that verb's exit code would read green against a dead system. Owned by the "Diagnostics un-muting + harness-truth sweep" working entry — it is the 2026-08-17 `mock_builder` finding re-measured, not a new one.
- **Bindings clobber recurred a 7th time** — the workspace nextest rewrote `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape despite this chunk touching NO TauRPC. Second consecutive chunk with that trigger, so an assertion scoped to "chunks that touch TauRPC" would miss it. HEAD stayed correct; recovered by regenerating as the last cargo-adjacent step and verifying no-diff vs HEAD before `capability-drift`.
- **Sibling-report defect (report-quality, not product)** — the `log_records` chunk's `scope.md` required its report to state whether the fix generalizes to `metrics_points` (in-scope at `scope.md:55` and `:150`); its `report.md` carries zero mentions of `metrics_points`. Detectors read reports, not scopes, so nothing caught the omission and this chunk derived the generalization first-hand.

## Outcome

**Acceptance criteria: met.** All 13 plan criteria satisfied. The key ones, with evidence:

- **Measured RED first, on the real OTLP path** (at HEAD `cbb316c`, before any fix): `rows_ingested` **0** (whole-batch loss — even the non-colliding control died), one `duckdb.append` ERROR with `reject_reason: "append_failed"` carrying no metric name, and `redactions_applied` **2** with nothing persisted.
- **GREEN after** (separate fresh data dir): `rows_ingested` **5** (2 label-differing + 2 redacting-alike + 1 control), `redactions_applied` **2**, **0** ERROR, **0** panics. Canary literals and the bare `sk_live_` form each **0 times across 14,215 log lines** — a non-empty read, so the absence is meaningful.
- **Mutation check — both pin families proven discriminating.** (a) Removing `seq` from the PK turns all 4 metrics pins RED, failing with the defect verbatim: `duplicate key "[REDACTED:provider_key], 170000000000000000`. (b) Moving the redaction fold above the append turns the rejection pin RED (`left: 1, right: 0`) while the success pin stays GREEN — confirming the success pin alone does not discriminate and the *pair* is the guard.
- **The ordinal stays non-observable** — absent from `SELECT_METRICS` / `COUNT_METRICS` (both name their columns explicitly), the `metrics.*` IPC response, the MCP `query_metrics` response, `BufferStateSnapshot` and `buffer.tick`; pinned by `state::tests::seq_allocators_are_absent_from_the_snapshot`.
- **`metric_name` still scrubs after the key change**, and the added discriminator is content-independent (a counter, no derivation from the metric name or any attribute value).
- **`buffer-log-seq-allocator-unit-coverage` closed** rather than re-minted for metrics — both allocators gained direct in-crate assertions (block width, monotonicity, independence, concurrency disjointness).

**Gates green** (commands run): `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run --workspace --profile ci` → **1870 passed / 1 skipped** (1859 → +11, exactly the tests added) · `cargo nextest run -p buffer -p viz` → **218/218** · `cargo xtask capability-drift` clean (0 missing, 0 extra) · `cargo xtask capability-widening-check` clean (0 violations / 3 inspected) · `cargo deny check bans licenses sources` **ok** · `cargo deny check advisories` designed-RED at exactly the **8 owned IDs** (0189/0190/0194/0195/0204/0222/0253/0258), no new findings.

**Smoke: PASS, and discriminating.** The boot-path did not change, but the plan listed harness commands and the measure-first design required two live legs; both ran as direct-binary boots against fresh `ANDROMEDA_PULSE_DATA_DIR`s. Feed precondition asserted before any absence claim (`rows_ingested` 0→5). Clean shutdown by specific PID via PowerShell `Stop-Process`, both `:4317`/`:4318` released, **0 orphan processes**. `bash scripts/agent-run.sh status` exited 0 but described residue — see *Decisions & corrections*; the direct-binary legs are the smoke evidence of record.

**Coverage:** 0 capabilities claimed — the matrix gate no-ops (all 22 swept at P1/P5; none covers ingestion identity, PK collision, or metric-point label fidelity). Proof lives in the tests and this report, as at both preceding chunks.

**`cargo audit` PREREQ:** session 35 is BETWEEN ratified probe points (25/28/31/34 ran; DISCHARGED at 34; next **37**) — skipped in the ratified form with basis + overlap re-verified this wrap, never silently.
