# Codebase Research — 2026-08-23-ingestion-scrub-coverage

## Scope
- **Depth:** deep · **Reads:** 12 · **Globs/Greps:** 9 · **Graph queries:** 2 (rust plane, `db_state: fresh`)

## Files inspected
- `crates/buffer/src/appender.rs` (`:35–95`, `:107–128`, `:120–180`, `:200–300`, `:302–340`, `:341–475`, `:525–612`, `:725–745`) — the four record-batch builders, `extract_service_name`, `scrub_otlp_field`, the `redactions_applied` folds.
- `crates/buffer/src/consumer.rs` (`:222–248`) — `observe_spans_for_baseline`, the baseline observer tap.
- `crates/buffer/src/schema.rs` (`:20–110`, `:185–200`) — DDL for all eight tables, both representations.
- `crates/buffer/src/retention.rs` (`:29`) — the only other `instrumentation_scopes` mention.
- `crates/buffer/src/state.rs` (`:29–63`) + `contract.rs` (`:53–76`) — `redactions_applied` plumbing.
- `crates/security/src/scrubber.rs` (`:45–70`) — the recall-over-precision posture + catalog ordering.
- `crates/triage/src/baseline/sql.rs` (`:80–140`) — Q1/Q2/Q3/Q4/Q5 consumer queries.
- `crates/triage/src/baseline/mod.rs` (`:480–495`) — `iter_operations` composite-key parse.
- `crates/corpus/src/contract.rs` (`:468–510`) — `metric_name` lookup + purge.
- `crates/viz/src/query.rs` (`:19`, `:29`, `:96`, `:325–361`) — `SELECT_TRACES` / `SELECT_LOGS` column lists.

## Graph impact (rust plane, `db_state: fresh`)
- **`security::scrubber::scrub_attribute`** — **26 references** across 12 files. Already consumed by `buffer` (appender `:17`/`:319`, drain `:45`/`:374`/`:634`), `interpretation` (markdown `:28`/`:336`), and seven `pulse-app` persist/egress sites. **The cross-crate edge `buffer → security` already exists** — no new dependency is needed for any treatment.
- **`buffer::appender::extract_service_name`** — **4 references, 3 of them production call sites** at `appender.rs:44`, `appender.rs:359`, `consumer.rs:232` (+ the `consumer.rs:12` import). **The working-route entry named only one.**
- **`buffer::appender::scrub_otlp_field`** — 3 call sites, all in `appender.rs` (`:306` log body, `:388`/`:390` exception message + stacktrace). Definition `:319`.
- **`buffer::state::BufferState::record_redactions`** — 2 call sites (`appender.rs:240` logs batch, `:432` span-events batch). Already the once-per-batch fold.

## Patterns detected
- **Single-choke-point extraction** (`crates/buffer/src/appender.rs:107`): `extract_service_name(resource) -> String` is one `pub(crate)` fn returning an owned `String`, defaulting to `String::new()` when the attribute is absent. All three consumers call it. A scrub applied *inside* it covers every consumer with one edit.
- **Boundary scrub helper** (`crates/buffer/src/appender.rs:319`): `scrub_otlp_field(&str, &mut u64) -> String` wraps `scrub_attribute`, maps `Redacted{category}` to `[REDACTED:{category}]`, and increments the caller's counter. The established shape for any new column coverage.
- **Once-per-batch counter fold** (`appender.rs:240`, `:432`): each builder accumulates a local `redactions_applied: u64` and folds it once via `state.record_redactions(n)` — never per row. Matches the obs hot-path ban.
- **Observer taps run beside the Arrow batch, not through it** (`consumer.rs:232`, `appender.rs:425`): the baseline tap and the fingerprint/storm observer each receive `&service_name` directly from `extract_service_name`, bypassing DuckDB entirely.

## Conventions to follow
- **Static DDL, never interpolated** (`crates/buffer/src/schema.rs:20` comment) — per-table `const` + a concatenated `SCHEMA_DDL`, kept in sync by a spot-check test.
- **Scrub returns an owned String at the push site** (`appender.rs:307`, `:389`) — the value is scrubbed *before* the per-Vec push, so the Arrow array never holds the raw value.
- **`pub(crate)` internals, contract-only public surface** — `extract_service_name` and `scrub_otlp_field` are both `pub(crate)`; a new helper follows suit.

## Findings that decide the design

**F1 — Only ONE of the four targets sits in a primary key.** From `schema.rs`:
| table | PRIMARY KEY | target column in PK? |
|---|---|---|
| `spans` | `(trace_id, span_id)` | `service_name` — **NO** |
| `span_events` | `(trace_id, span_id, event_index)` | `name` — **NO** |
| `metrics_points` | `(metric_name, ts_unix_nano, resource_hash)` | `metric_name` — **YES** |
| `log_records` | `(ts_unix_nano, resource_hash, severity_number, seq)` | `severity_text` — **NO** |

So a redacting rewrite is PK-safe on three of four. On `metrics_points` it is **not**: two distinct metric names redacting to the same `[REDACTED:{category}]` string at the same `ts_unix_nano` + `resource_hash` collide, and the immediately preceding chunk measured that a colliding pair fails the **ENTIRE batch** at `flush()` (arch §Conventions → Primary key convention). This is the sharpest constraint in the chunk.

**F2 — `service_name` reaches three destinations; only one is a DuckDB column.** Verified via the graph plus reads:
1. `appender.rs:44` → `spans.service_name` (the column).
2. `appender.rs:359` → collected into `service_names`, **never written to a column** (`span_events` DDL and its Arrow schema both have no `service_name` field) — used only at `:425` to feed `FingerprintObserver::on_fingerprint(fp, &service_names[row_idx], ts)`, i.e. the storm detector.
3. `consumer.rs:232` → `SpanObserver::observe_span(&service_name, …)`, the baseline tap feeding `triage`'s in-memory baseline state.

**Consequence:** scrubbing at the *column write alone* would leave paths 2 and 3 carrying raw names into `triage`, and would make `spans.service_name` (scrubbed) disagree with the baseline/registry identity (raw) — so the per-service joins that light the constellation would MISS. A scrub inside `extract_service_name` covers all three consistently. (At-rest corpus exposure from paths 2–3 is already covered by `BaselineState::scrubbed_clone` / `StormStateSnapshot::scrubbed_clone`; the divergence, not the leak, is the risk there.)

**F3 — The four targets are exactly the remaining client-controlled text columns.** Enumerated from the DDL of the four written tables: `spans.service_name` · `span_events.name` · `metrics_points.metric_name` · `log_records.severity_text` are the only client-controlled VARCHARs not already scrubbed (`log_records.body`, `exception_message`, `exception_stacktrace`) or deliberately excluded (`exception_type`). `span_links` carries no text; `metrics_points` has no service column. The entry's target list — minus its vacuous fifth — is complete.

**F4 — `instrumentation_scopes` has no producer (confirmed).** `append_record_batch_to_table` is the only DuckDB write path and is called for exactly `spans`, `metrics_points`, `log_records`, `span_events` (`appender.rs:534/557/580/604`, plus `consumer.rs:182` generic and one test at `:1257`). `instrumentation_scopes` appears only in `schema.rs` DDL (both representations) and `retention.rs:29`. `resources` is in the same never-written state.

**F5 — Both webview consumer questions resolve YES.** `SELECT_TRACES` (`viz/query.rs:19`) selects `service_name`, so it reaches the constellation dots and the table Service column (layouts' concern is live). `SELECT_LOGS` (`:29`) selects `severity_text` into `LogItem.severity_text` (`:96`, `:325–361`), so it reaches the Logs view (a11y's concern is live for the Logs surface; the 2026-06-10 not-color-alone remediation was on the incident `ReportRenderer` path, which reads the `triage` severity enum, not this column).

**F6 — No new crate edge, and the counter already exists.** `buffer → security` is an existing edge (26 `scrub_attribute` refs incl. `buffer`), and `redactions_applied` is already folded once per batch in two builders with an obs allowlist leaf and a guard test shipped last chunk. Extending coverage needs neither a new dependency nor a new observable.

## Files to modify
- `crates/buffer/src/appender.rs` — the scrub coverage itself. Threading set, enumerated from the graph rather than memory:
  - `extract_service_name` (`:107`) — the single choke point for all three `service_name` consumers (`:44`, `:359`, `consumer.rs:232`).
  - `build_spans_record_batch` (`:44`–`:83`) · `build_span_events_record_batch` name push (`:404` region) · `build_metrics_record_batch` / its push helper (`:737`) · `build_logs_record_batch` severity push (`:229`).
  - the two `record_redactions` folds (`:240`, `:432`) — and note the **spans** and **metrics** builders currently have NO fold at all, so covering them requires adding the counter plumbing those two builders lack.
- `crates/buffer/src/consumer.rs` — only if the treatment is applied at a call site rather than inside `extract_service_name` (the `:232` tap); no change if the choke point is used.
- `crates/buffer/src/appender.rs` `#[cfg(test)] mod tests` — in-crate assertions (tests-plan §4 requires in-crate, not `pulse-app/tests/`).
- `crates/security/src/scrubber.rs` — **only if** the chosen treatment needs a key-safe entry point; the existing `scrub_attribute` may suffice unchanged.
- No manifest change (the `buffer → security` edge exists); no allowlist/registry edit required unless a new tick field is added.

## New files to create
- None required. (An in-crate test module already exists in `appender.rs`; a new `pulse-app/tests/` guard is only needed if a tick field is added, which F6 says it is not.)

## Open questions
- **Which treatment per column class, given F1 + F2?** → blocks: **plan-decision**. F2 makes the `service_name` choke point clearly right; F1 makes `metrics_points.metric_name` genuinely contestable (a redacting rewrite there can fail whole batches on collision). P4 resolves before synthesis.
- **Does `instrumentation_scopes` get a disposition, or stay silently unaddressed?** → blocks: **plan-decision**. The entry and the spec both name it as a target; F4 proves it vacuous. P4 must state a disposition rather than silently drop it.
- **Do the spans + metrics builders get the `redactions_applied` fold they currently lack?** → blocks: **implementation-scope**. Adding it is a two-line plumbing change per builder, but it changes what the counter counts, which the obs extract flags as a registered-scope question.

## Scope premise closure
- `[inferred]` **`log_records.severity_text` consumer map** → **VERIFIED**: it is SELECTed by `viz::query::SELECT_LOGS` and rendered in the Logs view (F5). Tag dropped in `scope.md`.
- `[inferred]` **touchpoint `crates/security/src/scrubber.rs`** → **VERIFIED as conditional**: needed only if the treatment requires a key-safe entry point; `buffer → security` already exists (F6). Restated conditionally in `scope.md`.
- `[inferred]` **touchpoint `crates/buffer/src/state.rs` / `contract.rs`** → **VERIFIED as not-required**: `redactions_applied` already exists and is folded once per batch (F6). Restated in `scope.md`.
- `[inferred]` **per-class treatment candidates** → **retained as a P4 decision**, now grounded in F1/F2 rather than speculation.
- **FALSIFIED — the write-site premise.** `scope.md`'s target table implied `spans.service_name` has a single write path. It has **three consumers via one extractor**, two of which never touch a column (F2). Corrected in `scope.md` with `[premise-corrected: …]`.
