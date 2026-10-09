# Codebase Research — 2026-08-22-log-records-identity

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 8 · **Code-graph:** 1 query (rust plane, `db_state` warm)

## Files inspected

- `crates/buffer/src/schema.rs` (`:75-100`, `:175-200`, `:295-345`, `:375-400`) — the two DDL representations, the sync test, and **the decisive prior observation** (below).
- `crates/buffer/src/appender.rs` (`:195-310`, `:329-360`, `:488-535`, `:990-1030`) — `build_logs_record_batch`, the append primitive, the `event_index` precedent, the existing single-record log test.
- `crates/buffer/src/consumer.rs` (`:60-185`) — `dispatch_batch` (production path), `append_table_traced`, and `run_consumer`'s error handling.
- `crates/buffer/src/retention.rs` (`:1-60`) — the sweep's `DELETE … WHERE ts_unix_nano < ?` constants.
- `crates/viz/src/query.rs` (`:20-45`, `:485-500`) — `SELECT_LOGS` / `COUNT_LOGS`, and a **third** `log_records` DDL in a test fixture.
- `crates/triage/src/baseline/sql.rs` (`:847`, `:866-895`) — a **fourth** `log_records` DDL, also test-only.
- `crates/mcp-server/src/tools.rs` (`:37`, `:172`, `:262-274`) — `query_logs` delegates to `viz::query::query_logs`; no independent SQL.
- `crates/ingest/src/grpc.rs` (`:216-260`), `crates/ingest/src/http.rs` (`:341-420`), `crates/ingest/src/state.rs` (`:47`) — receiver-side log counting.

## Graph impact (rust plane)

- **`append_record_batch_to_table` / `build_logs_record_batch`** — `rows=11`, `db_state=fresh`, plane `rust` (trace: `.andromeda/runs/2026-08-22T19-55-00Z-phase/tree-query-2026-08-22-log-records-identity.json`). Callers are **buffer-internal only**: production `consumer/dispatch_batch()` @ `crates/buffer/src/consumer.rs:138` and `consumer/append_table_traced()` @ `:181` (+ the `consumer/` module scope @ `:11`); the remaining 6 rows are the `#[cfg(test)]` wrappers `append_{spans,metrics,logs,span_events}_batch` @ `appender.rs:521/544/564/567/591` and one test. **Zero cross-crate callers of the append primitive** — an additive column change has no cross-crate blast radius at the write path. Readers reach the table by SQL string, not by symbol, so they are enumerated by grep below rather than by the graph (a real limit of the symbol graph here, not a cold-start).

## Patterns detected

- **Arrow Appender, not prepared INSERT** (`appender.rs:494-507`): `conn.appender(table)` → `append_record_batch(rb)` → `flush()`, each mapped to `Error::Append`. This is the only write path for `log_records`; `consumer.rs:143` is its production caller.
- **Ordinal-discriminator precedent is LIVE in the same builder** (`appender.rs:354`, `:383`, `:448`): `for (event_index, event) in span.events.iter().enumerate()` → `event_indices.push(event_index as i32)` → `Field::new("event_index", DataType::Int32, false)`, with an existing monotonicity test `append_span_events_batch_assigns_monotonic_event_index_per_span` (`:1527`). `span_links.link_index` mirrors it. This is exactly the shape arch's extract pointed to, and it already ships.
- **Readers select named columns, never `*`** (`viz/src/query.rs:30`): `SELECT_LOGS` names 7 columns and **omits `template_id`** — an already-shipped nullable column. So a column-additive change is provably non-breaking for the production read path, by existing precedent on this very table.
- **Retention keys on `ts_unix_nano` only** (`retention.rs:27`), prepared with `?` — unaffected by any key-shape change.
- **Scrub-on-write is live for `body`** (`appender.rs:290-307`): `extract_log_body` → `scrub_otlp_field`, metering `redactions_applied`. Confirms security's premise that keying on `body` would make identity depend on redaction output.

## Conventions to follow

- **Whole-batch error semantics** (`consumer.rs:143` + `:74-92`): `append_table_traced(...)?` propagates out of `dispatch_batch`; `run_consumer` catches `Ok(Err(e))` and emits `tracing::error!(target: "duckdb.append", reject_reason = …, "appender returned error")`. On that path `state.record_rows_appended(rows)` is **never reached** and the broadcast emit is **skipped**.
- **`rows_appended` counts REQUESTED, not LANDED** (`appender.rs:493`): `let row_count = record_batch.num_rows() as u64;` is computed *before* the append and returned on success. It can never witness a partial landing — answering obs's open question directly.
- **Receiver counts before the buffer** (`grpc.rs:225`, `http.rs:357` → `state.rs:47` `record_log_records`): ingest's counter advances at the receiver, so a batch lost at the appender leaves ingest climbing while zero rows land — the `app.boot.buffer.degraded` divergence class obs named, here with no counter that compares the two.
- **Test-fixture DDLs are already-divergent minimal subsets**, not mirrors: `viz/query.rs:490` (8 columns, no `template_id`) and `triage/baseline/sql.rs:877` (8 columns, no `template_id`, `trace_id`/`span_id` nullable). Both carry the **same defective PK**.

## New files to create

- (none required by research; the plan decides whether the regression coverage lands in a new `crates/buffer/src/` test module or extends `appender.rs`'s existing `mod tests`.)

## Files to modify

- `crates/buffer/src/schema.rs` — the PK in **both** representations (`:92` const, `:185` live `SCHEMA_DDL`); `ddl_constants_match_concatenated_schema` (`:380`) enforces the pair.
- `crates/buffer/src/appender.rs` — `build_logs_record_batch` (`:195-289`): the discriminator's assignment, its Arrow `Field`, and its array, if option (a) is taken.
- `crates/buffer/src/appender.rs` `mod tests` — the in-crate regression coverage (`-p buffer`, per the tests extract's in-crate debt note).
- **Caller threading / companions:**
  - `crates/viz/src/query.rs:490` — test-fixture DDL carrying the old PK (decide: update or leave; see Open question 2).
  - `crates/triage/src/baseline/sql.rs:877` — the same, in a second crate.
  - `crates/buffer/src/schema.rs:310-345` — `spans_primary_key_is_composite_trace_id_span_id` is the introspection-test template a `log_records` PK assertion should copy.
- **Not affected (verified, not assumed):** `crates/viz/src/query.rs:30/:39` (named-column SELECT, omits `template_id` already) · `crates/mcp-server/src/tools.rs` (delegates to viz) · `crates/buffer/src/retention.rs:27` (`ts_unix_nano` only) · `pulse-app/capabilities/*.json` (no procedure change).

## Scope premise closure

1. **The failure mode** — `[premise-corrected: the route entry's "silently drops [the second record]" is falsified in SHAPE. Structurally the loss is WHOLE-BATCH and it IS logged: an appender error propagates via `?` out of `dispatch_batch` (consumer.rs:143) and surfaces as `tracing::error!(target: "duckdb.append", reject_reason=…)` at consumer.rs:78-83, with `record_rows_appended` skipped and the broadcast suppressed. What stays open is whether the Appender raises at all — see Open question 1.]`
2. **The write path** — **VERIFIED**: Arrow `Appender` (`appender.rs:494-507`), not a prepared INSERT.
3. **Blast radius** — `[premise-corrected: WIDER than scope assumed on DDL, NARROWER on readers. There are FOUR `log_records` DDL sites, not two — schema.rs:82 (const) · schema.rs:175 (live) · triage/baseline/sql.rs:877 · viz/query.rs:490 — and the sync test guards only the schema.rs pair. The two out-of-crate copies are test-only, already-divergent minimal subsets that both carry the OLD PK. Conversely every production reader is safe: viz names its columns and already omits template_id, MCP delegates to viz, retention keys on ts_unix_nano alone.]`
4. **The architectural consequence** — **VERIFIED, and already inaccurate at HEAD independent of this chunk**: arch §Conventions *Primary key convention* says log records use "timestamp + resource hash + name", but `log_records` has **no `name` column** (its columns are `ts`, `ts_unix_nano`, `resource_hash`, `severity_number`, `body`, `severity_text`, `trace_id`, `span_id`, `template_id`). Wrap-time drift amendment.

## Findings not anticipated by scope, directive, or any extract

- **A duplicate-INSERT PK probe is a documented HANG on this build.** `crates/buffer/src/schema.rs:311-318`: *"Runtime PK check via duplicate-INSERT path was observed to hang in libduckdb-sys 1.10502 on this build — schema introspection is the contract assertion, **not behavioral PK enforcement**. Behavioral enforcement is exercised at the appender path (chunk #22+ when queries land)."* Two consequences: (i) **behavioural PK enforcement has never been verified anywhere in this codebase** — the deferral that comment records is still open, and this chunk is where it comes due; (ii) the obvious way to measure the defect is the one way documented to wedge, so the measurement must go through the **Arrow Appender path under an explicit timeout**, never a duplicate INSERT. Against the zero-flakiness / no-retry discipline (test-plan §10), an unguarded hang would be materially worse than a failure.
- **The design fork narrows to (a) vs (b) on evidence.** Option (c) is ruled out from two independent directions: security's — keying on `body` makes identity depend on scrubber output (`extract_log_body` → `scrub_otlp_field` is live at `appender.rs:290-307`) — and the code's: untraced log records carry empty `trace_id`/`span_id` (`appender.rs:1102`-region tests construct exactly that), so widening with them does not separate the colliding case.
- **`metrics_points` shares the defect verbatim** — `PRIMARY KEY (metric_name, ts_unix_nano, resource_hash)` omits the point's attributes, in the same two representations plus the same viz fixture (`viz/query.rs:487`). Adjacent, fenced out of scope, recorded so a future entry is minted with evidence.

## Open questions

1. **Does the DuckDB Arrow Appender enforce the `log_records` PRIMARY KEY on `flush()`?** → blocks: **plan-decision**. If it does, the defect is real and *more* severe than the route entry describes (whole-batch loss, and per security's extract any local producer can poison a well-behaved producer's batch). If it does not, both rows land, there is no defect, and the chunk closes as a premise correction. Never verified in this codebase (`schema.rs:311-318`); must be settled by the measure-first leg over the Appender path under a timeout.
2. **Do the two out-of-crate test fixtures (`viz/query.rs:490`, `triage/baseline/sql.rs:877`) get updated to the fixed key?** → blocks: **implementation-scope**. They are already-divergent minimal subsets, so they will not break either way; leaving them means those suites keep exercising the old identity.
