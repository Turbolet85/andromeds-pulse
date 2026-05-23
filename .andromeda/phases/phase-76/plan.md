# Phase 76 Plan — Chunk #79 SQL aggregation queries + scheduler

**Phase number:** 76
**Route chunks covered:** route#79 (single chunk; Form 1 append; chunk #80 not yet registered)
**Epoch:** Epoch 9 — Foundation v0.2.0 (Phase 7 of pulse v0.2.0 — second Phase 7 chunk)
**Source plan:** `docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 7 §79 (lines 502-513)
**Combined extracts:** `.andromeda/phases/phase-76/combined.md`
**Research:** `.andromeda/phases/phase-76/research.md`

---

## Combined goal

Implement the L1a SQL aggregation layer that executes templates Q1-Q7 (per dist-arch v3 §Appendix A, lines 986-1105) against the L0 DuckDB ring buffer. Q1-Q6 are bounded by GROUP BY + ORDER BY + LIMIT in the SQL itself; Q7 (recursive CTE for critical-path extraction over the 5 slowest root-span traces) is bounded by FOUR orthogonal constraints — `tt.depth < 10` recursive cap + outer `LIMIT 100` + `slow_traces LIMIT 5` + 200 ms wall-clock timeout — with а Q7-fallback shallow non-recursive variant invoked on timeout OR row-limit hit. Queries are consumed in-process by а future chunk #80 Cadence Coordinator that schedules invocation; chunk #79 itself adds zero IPC surface, zero broadcast topics, zero arch-registry deltas.

Per pulse v0.2.0 capabilities, this chunk is а prerequisite substrate for P-020 (Three-Tier Severity Model — algorithmic detection requires aggregated input) + P-021 (Algorithmic Attention Cues — cue evaluator consumes Q1-Q6 outputs). Per the dist-arch v3 performance envelope (line 146), full L1a query set runs in approximately 200-500ms per cadence tick; Q7 worst-case bounded at 200ms by explicit timeout.

---

## Implementation Steps

### Step 1 — Add SQL templates module к triage

Create `crates/triage/src/baseline/sql.rs` with the 8 SQL template constants (Q1-Q7 + Q7-fallback) verbatim from `pulse-distillation-architecture.md` §Appendix A, with two precise corrections:

1. Rewrite `FROM logs` → `FROM log_records` in Q6 (the canonical table name per arch §Occupied Resources DuckDB database / schema names + `crates/buffer/src/schema.rs:18`).
2. Verify `FROM span_events` in Q3 — table exists (chunk #65 substrate).

Each constant:
```rust
const Q1_RED_PER_SERVICE: &str = "SELECT service_name, COUNT(*) AS request_count, ... FROM spans WHERE start_time > now() - INTERVAL ? GROUP BY service_name";
// ... Q2-Q6 similarly
const Q7_CRITICAL_PATH: &str = "WITH RECURSIVE slow_traces AS (...), trace_tree AS (...) SELECT * FROM trace_tree ORDER BY trace_id, depth, start_time LIMIT 100";
const Q7_FALLBACK: &str = "WITH slow_traces AS (...) SELECT s.* FROM spans s WHERE s.trace_id IN (...) ...";
```

ALL templates use `?` placeholder для the `INTERVAL` window-duration parameter; NEVER `format!()` interpolation (security constraint).

### Step 2 — Add typed result structs

For each query, define а result struct exposing decoded fields:
```rust
pub struct Q1RedRow { pub service_name: String, pub request_count: i64, pub error_count: i64, pub error_rate: f64, pub p50_ns: i64, pub p95_ns: i64, pub p99_ns: i64 }
pub struct Q2OperationRow { pub service_name: String, pub operation: String, pub request_count: i64, pub error_count: i64, pub p99_ns: i64 }
pub struct Q3FingerprintRow { pub fingerprint: String, pub occurrences: i64, pub first_seen: i64, pub last_seen: i64, pub services: Vec<String> }
pub struct Q4InteractionRow { pub caller: String, pub callee: String, pub call_count: i64, pub error_count: i64 }
pub struct Q5CardinalityRow { pub service_name: String, pub distinct_operations: i64, pub total_spans: i64 }
pub struct Q6LogRow { pub service_name: String, pub template_id: i64, pub occurrences: i64, pub peak_severity: i64 }
pub struct Q7CriticalPathRow { pub trace_id: Vec<u8>, pub span_id: Vec<u8>, pub parent_span_id: Option<Vec<u8>>, pub depth: i64, pub start_time: i64, pub service_name: String, /* + other decoded columns */ }
```

### Step 3 — Add SqlAggregationError enum

Mirror `crates/triage/src/baseline/error.rs:BaselineError` shape:
```rust
#[derive(Debug, Error)]
pub enum SqlAggregationError {
    #[error("duckdb query failed for {query_id}")]
    QueryFailed { query_id: &'static str },
    #[error("q7 timeout after {timeout_ms}ms; fallback triggered")]
    Q7Timeout { timeout_ms: u64 },
    #[error("connection lock poisoned")]
    LockPoisoned,
    #[error("join error in spawn_blocking")]
    Join,
}
impl SqlAggregationError {
    pub fn error_category(&self) -> &'static str { ... }
}
```

### Step 4 — Add `TriageSqlState` constructor + connection handle

Create `crates/triage/src/baseline/sql.rs::TriageSqlState { conn: Arc<Mutex<Connection>> }` constructor. Boot wiring at `pulse-app/src/main.rs` shares the same DuckDB connection instance as `BufferState` + `VizState` (single connection per arch §Occupied Resources DuckDB database identity `pulse_buffer`).

### Step 5 — Implement Q1-Q6 async functions

Per Q (q1, q2, q3, q4, q5, q6):
```rust
#[tracing::instrument(skip(state), fields(query_name = "q1", query_id = "q1", param_count = 1, row_count_returned))]
pub async fn run_q1(state: &TriageSqlState, window: Duration) -> Result<Vec<Q1RedRow>, SqlAggregationError> {
    let conn = state.conn.clone();
    let window_param = format!("{} seconds", window.as_secs());
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || run_q1_blocking(&conn, &window_param))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    let elapsed = start.elapsed();
    tracing::info!(target: "metric.pipeline.l1a.query_count_total", query_name = "q1", value = 1);
    tracing::info!(target: "metric.pipeline.l1a.query_latency_p99_milliseconds", query_name = "q1", duration_ms = elapsed.as_millis() as u64);
    tracing::Span::current().record("row_count_returned", result.len() as i64);
    Ok(result)
}

fn run_q1_blocking(conn: &Arc<Mutex<Connection>>, window_param: &str) -> Result<Vec<Q1RedRow>, SqlAggregationError> {
    let guard = conn.lock().map_err(|_| SqlAggregationError::LockPoisoned)?;
    let mut stmt = guard.prepare(Q1_RED_PER_SERVICE).map_err(|_| SqlAggregationError::QueryFailed { query_id: "q1" })?;
    let rows = stmt.query_map([window_param], |row| Ok(Q1RedRow { ... }))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SqlAggregationError::QueryFailed { query_id: "q1" })?;
    drop(stmt);
    Ok(rows)
}
```

Mirror the pattern for Q2-Q6 (different column decoders, different result struct, identical telemetry shape).

### Step 6 — Implement Q7 with timeout + fallback

```rust
const Q7_TIMEOUT: Duration = Duration::from_millis(200);

#[tracing::instrument(skip(state), fields(query_name = "q7", query_id, param_count = 1, row_count_returned))]
pub async fn run_q7(state: &TriageSqlState, window: Duration) -> Result<Vec<Q7CriticalPathRow>, SqlAggregationError> {
    let conn = state.conn.clone();
    let window_param = format!("{} seconds", window.as_secs());
    let start = Instant::now();

    // Try Q7-primary with 200ms timeout
    let q7_result = tokio::time::timeout(
        Q7_TIMEOUT,
        tokio::task::spawn_blocking({
            let conn = conn.clone();
            let window = window_param.clone();
            move || run_q7_primary_blocking(&conn, &window)
        })
    ).await;

    match q7_result {
        Ok(Ok(Ok(rows))) => {
            // Q7 primary succeeded
            let elapsed = start.elapsed();
            tracing::info!(target: "metric.pipeline.l1a.query_count_total", query_name = "q7", value = 1);
            tracing::info!(target: "metric.pipeline.l1a.query_latency_p99_milliseconds", query_name = "q7", duration_ms = elapsed.as_millis() as u64);
            tracing::Span::current().record("query_id", "q7");
            tracing::Span::current().record("row_count_returned", rows.len() as i64);
            Ok(rows)
        }
        Ok(Ok(Err(e))) => Err(e),
        Ok(Err(_)) => Err(SqlAggregationError::Join),
        Err(_) => {
            // Timeout — emit timeout telemetry + invoke Q7-fallback
            tracing::warn!(target: "metric.pipeline.l1a.q7_timeout_count_total", value = 1, timeout_ms = Q7_TIMEOUT.as_millis() as u64, rejection_reason = "timeout");
            run_q7_fallback(&conn, &window_param, start).await
        }
    }
}

async fn run_q7_fallback(conn: &Arc<Mutex<Connection>>, window_param: &str, start_time: Instant) -> Result<Vec<Q7CriticalPathRow>, SqlAggregationError> {
    let conn = conn.clone();
    let window = window_param.to_string();
    let result = tokio::task::spawn_blocking(move || run_q7_fallback_blocking(&conn, &window))
        .await
        .map_err(|_| SqlAggregationError::Join)??;
    let elapsed = start_time.elapsed();
    tracing::info!(target: "metric.pipeline.l1a.q7_fallback_count_total", value = 1, fallback_query_kind = "shallow_non_recursive", cause = "timeout");
    tracing::info!(target: "metric.pipeline.l1a.query_latency_p99_milliseconds", query_name = "q7-fallback", duration_ms = elapsed.as_millis() as u64);
    tracing::Span::current().record("query_id", "q7-fallback");
    tracing::Span::current().record("row_count_returned", result.len() as i64);
    Ok(result)
}
```

Note: `tokio::time::timeout` is Option A from research.md §Open implementation questions Q2. The cooperative timeout means spawn_blocking thread may continue briefly after the await unblocks; an `Option B` upgrade (DuckDB `interrupt_handle()`) is а follow-up if available in the duckdb 1.10500.x crate API. Verify at /implement time.

### Step 7 — Export public API via mod.rs

In `crates/triage/src/baseline/mod.rs`, add:
```rust
mod sql;
pub use sql::{
    run_q1, run_q2, run_q3, run_q4, run_q5, run_q6, run_q7,
    Q1RedRow, Q2OperationRow, Q3FingerprintRow, Q4InteractionRow, Q5CardinalityRow, Q6LogRow, Q7CriticalPathRow,
    SqlAggregationError, TriageSqlState,
};
```

### Step 8 — Tests (co-located `#[cfg(test)] mod tests` in `sql.rs`)

For each of Q1-Q7:
- Happy-path test seeded with synthetic spans/events/logs via `Connection::execute("INSERT INTO spans (...) VALUES (?, ?, ...)")` per `crates/buffer/src/schema.rs` table layout. Use `duckdb::Connection::open_in_memory()` + `crates/buffer/src/schema.rs::create_schema()` to bootstrap а test DuckDB.
- Time-injected test: `#[tokio::test(start_paused = true)]` + seed data with synthetic `start_time` values relative to а fixed `now()` baseline; call `run_qN(...)` with а window Duration; assert filtered rows.
- Edge: missing rows (table empty) — Q1-Q6 return `Ok(vec![])`; assert.

Q7-specific tests (additional к above):
- LIMIT 100 row cap: seed >100 trace_tree rows, assert truncation at 100.
- LIMIT 10 depth cap: seed а trace with depth >10, assert recursion stops at depth 10.
- 200ms timeout: seed pathological broad trace (>1000 root spans), drive the timeout via injected `Q7_TIMEOUT` constant (`#[cfg(test)] const Q7_TIMEOUT: Duration = Duration::from_millis(1)` test override OR explicit knob), assert fallback fires within deterministic budget. Use `tokio::time::pause()` + `tokio::time::advance(Duration::from_millis(200))` to drive the timer deterministically; OR pass timeout as а function parameter to enable `run_q7(state, window, override_timeout)` shape.
- Q7-fallback path: assert fallback emits `pipeline.l1a.q7_fallback_count_total` counter via `tracing-test::traced_test` capture.

SQL injection negative test (one):
- Verify that а duration parameter constructed as `Duration::from_secs(60)` formats к `"60 seconds"` and binds correctly; verify that а maliciously-crafted string like `"60 seconds'; DROP TABLE spans; --"` (if anyone bypasses the Duration type via test-only fault injection) is rejected at DuckDB parameter-bind time (parser error, not execution). Assert table `spans` row count unchanged post-injection-attempt.

### Step 9 — Observability allowlist extension (deferred к follow-up; track in plan §Deferred)

Add к `pulse-app/src/observability.rs` per-module allowlist for `triage::baseline::sql`:
- `query_name`, `query_id`, `param_count`, `row_count_returned`, `duration_ms`, `value`, `timeout_ms`, `rejection_reason`, `fallback_query_kind`, `cause`

Per chunk #78 precedent (session 121 plan §Deferred), this polish pass can carry to next session OR fold into this chunk's commit. Default: defer; surface in handoff §Session Goals carry-over.

### Step 10 — Standard chunk-gate baselines

1. `cargo fmt --check` — passes.
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings` — passes.
3. `cargo nextest run --workspace --profile ci` — passes (new tests pass + existing 1279 unchanged unless new chunk affected — most likely 1279 → 1279+N where N = new chunk tests).
4. `cargo xtask capability-drift` — clean (no new TauRPC procedures + arch registry delta zero per chunk spec).
5. `cargo deny check bans licenses sources` — passes (no new deps).

---

## Codebase touchpoints

### New files

- `crates/triage/src/baseline/sql.rs` — Q1-Q7 SQL template constants + execution functions + result structs + error enum + tests. ~400-700 LOC estimate.

### Files to modify

- `crates/triage/src/baseline/mod.rs` — add `mod sql;` + `pub use sql::{ ... };` block.
- `pulse-app/src/main.rs` — construct `TriageSqlState` at boot using the existing DuckDB `Arc<Mutex<Connection>>` instance (shared с `BufferState`/`VizState`); thread into а state container ready for chunk #80 Cadence Coordinator consumption.
- `crates/triage/Cargo.toml` — verify `duckdb` + `tokio` deps already present (likely yes via inherited workspace.dependencies; verify at /implement time).

### Files to potentially modify (deferred — see §Deferred)

- `pulse-app/src/observability.rs` — allowlist extension for new `triage::baseline::sql` fields (deferral precedent from chunk #78).

---

## Test Commands (per test-plan §3 5-command discipline)

The 5 standard commands are unchanged from the existing harness (per `scripts/agent-run.sh` + tests-plan §3 binding contract). Chunk #79 adds NO new harness command. Smoke-test verification:

1. `cargo nextest run -p triage` — exercises new Q1-Q7 tests under `crates/triage/src/baseline/sql.rs::tests`.
2. `cargo nextest run --workspace --profile ci` — exercises the full workspace + asserts no regression on existing 1279 tests.
3. `cargo llvm-cov nextest --workspace --lcov` — exercises coverage gate (≥75% line / ≥70% branch / ≥85% function).
4. `cargo xtask capability-drift` — verifies arch registry delta=zero claim (chunk #79 introduces no new TauRPC / broadcast / capabilities).
5. `cargo xtask capability-widening-check` — verifies no new structural capability widening.

Phase 2b smoke check: SKIPPED per testing.md 2026-05-19 Session Addition (chunk #79 is backend-only; integration tests cover the same runtime invariants more reliably; avoid tauri dev smoke к sidestep Windows orphan-process risk).

---

## Acceptance Criteria

Per chunk #79 (single chunk; all criteria attribute к route#79):

### Functional

1. (chunk #79) `crates/triage/src/baseline/sql.rs` exposes `pub fn run_q{1..7}(state, window) -> Result<Vec<Q{N}Row>, SqlAggregationError>` via `pub use` from `mod.rs`.
2. (chunk #79) Q1-Q7 templates are byte-identical к `pulse-distillation-architecture.md` §Appendix A modulo the two documented rewrites (`FROM logs` → `FROM log_records` в Q6 only).
3. (chunk #79) Q7 timeout fires at 200ms (or injected test value); Q7-fallback returns shallow non-recursive results on timeout OR row-limit hit.

### Architecture (per arch extract)

4. (arch) SQL templates module lives in existing `crates/triage/` (no new workspace member); no new arch-registry surface; `cargo xtask capability-drift` clean.
5. (arch) Module dependency direction preserved: `crates/triage/Cargo.toml` does not add reverse-direction sibling-crate dep (no `triage` → `viz`; no `buffer` → `triage`).
6. (arch) DuckDB queries use `Connection::prepare` + `?` placeholders + `Statement::execute([&param])` / `query_map([&param], ...)`; `grep -rEn 'format!\("[^"]*INTERVAL [^?]' crates/triage/src/baseline/ crates/buffer/src/` returns zero hits.

### Security (per security extract)

7. (security) `rg "format!\(\"[^\"]*INTERVAL [^?]" crates/triage/baseline crates/buffer` returns zero hits (no f-string SQL interpolation against `INTERVAL`).
8. (security) Q7 timeout enforced deterministically: а unit test feeds pathological trace fixture (broad trace, >100 spans, depth >10) and asserts Q7 transitions к fallback within 200ms (test injects clock via `tokio::time::pause()` + `advance()`, not wall-clock sleep).
9. (security) Q1-Q7 error path asserts а `duckdb::Error` containing fabricated SQL fragment does NOT leak в log emission — `tracing-test` capture shows only `query_id` + `param_count` + duration; raw SQL fragments absent.

### Tests (per tests extract)

10. (tests) `cargo nextest run -p triage` passes for new Q1-Q7 tests; workspace `cargo nextest run --workspace --profile ci` passes без regression (1279 → 1279+N baseline).
11. (tests) Coverage gate maintained: workspace line ≥75% / branch ≥70% / function ≥85% via `cargo llvm-cov nextest --workspace --lcov`.
12. (tests) Each of Q1-Q7 has ≥1 time-injected unit test (`#[tokio::test(start_paused = true)]` + `tokio::time::advance(Duration)`); Q7 additionally has dedicated tests для LIMIT 100 row cap, LIMIT 10 depth cap, 200ms timeout deterministic drive, fallback path.
13. (tests) Zero flakiness: no test relies on real wall-clock time или non-deterministic data ordering; Q7 timeout uses injected duration knob.

### Observability (per obs extract)

14. (obs) Q1-Q7 query execution emits `tracing::info!(target: "metric.pipeline.l1a.query_count_total", query_name = "qN", value = 1, ...)` per invocation; verifiable by `tracing-test` capture (preferred) OR tailing `~/.andromeda-pulse/logs/agent-latest.jsonl` for 7 distinct `query_name` values.
15. (obs) Q1-Q7 query latency emitted as `tracing::info!(target: "metric.pipeline.l1a.query_latency_p99_milliseconds", query_name = "qN", duration_ms = X, ...)` raw per-event; agent-computed p99 verified via `jq` для Q7 ≤ 200ms (test-time computation).
16. (obs) Q7 timeout path emits `tracing::warn!(target: "metric.pipeline.l1a.q7_timeout_count_total", value = 1, timeout_ms = 200, rejection_reason = "timeout")`; Q7 fallback path emits `tracing::info!(target: "metric.pipeline.l1a.q7_fallback_count_total", value = 1, fallback_query_kind = "shallow_non_recursive", cause = "timeout")`.
17. (obs) Zero raw SQL query text + zero parameter values in any `pipeline.l1a.*` emission — `tracing-test` capture filtered to `target` starting with `metric.pipeline.l1a` shows zero substring matches for `SELECT|WHERE|FROM|INTERVAL`.

### Out-of-scope domains

18. Design/layouts/a11y: chunk #79 adds zero pixels, zero focusable elements, zero design tokens — no acceptance criteria from these domains apply. Downstream chunks consuming Q1-Q7 outputs will trigger design/layouts/a11y scope when UI surfaces materialize.

---

## Acceptance Criteria → Deferred

Carry-over candidates (decision at /implement time whether к fold into this chunk OR defer):

- (obs) `pulse-app/src/observability.rs` allowlist extension for `triage::baseline::sql` fields. Deferred per chunk #78 precedent (session 121 plan §Deferred — production log-quality concern, not blocking functional correctness).
- (impl) DuckDB `Connection::interrupt()` API verification — if available in `duckdb` 1.10500.x crate, upgrade Q7 timeout from cooperative `tokio::time::timeout` (Option A) to true cancellation (Option B). Research note: combined.md security extract calls out "200 ms timeout MUST be enforced via DuckDB query-cancellation primitive (interrupt-on-timeout), not by a parallel sleep that lets the query keep running". /implement to verify duckdb crate API + bench. If Option B unavailable, Option A is acceptable interim (the spawn_blocking thread continuation is bounded by DuckDB's own internal query progress; Q7's LIMIT 100 + depth 10 bounds make worst-case continuation finite even without interrupt).

---

## Provenance

### Combined extracts referenced

- §Chunk #79 → Security section (constraints / patterns / anti-patterns / criteria)
- §Chunk #79 → Tests section (criteria 10-13; time-injected unit test discipline)
- §Chunk #79 → Obs section (metric event targets `pipeline.l1a.*`; raw-per-event emission)
- §Chunk #79 → Arch section (workspace placement; module dep direction; DuckDB discipline)
- §Cross-domain bindings (security ↔ obs ↔ arch DuckDB prepared-statement; security ↔ tests SQL injection negative; obs ↔ tests timeout/fallback drive)

### Research findings referenced

- `crates/viz/src/query.rs` lines 96-175 — canonical prepared-statement + tracing pattern.
- `crates/buffer/src/retention.rs` line 80 — `tokio::task::spawn_blocking` discipline.
- `docs/v0_2_0/pulse-distillation-architecture.md` §Appendix A lines 986-1105 — Q1-Q7 + Q7-fallback canonical SQL.
- `crates/triage/src/baseline/error.rs` 1-40 — thiserror enum pattern.
- `crates/buffer/src/schema.rs:18` — `log_records` is canonical table name (Q6 rewrite confirmed).
- `pulse-app/src/observability.rs` 1-80 — allowlist mechanism (Step 9 deferral target).

### Open questions (defer к /implement)

1. Duration parameter formatting: `format!("{} seconds", dur.as_secs())` per Q1-Q7 `INTERVAL ?` placeholder; verify DuckDB rejects malformed INTERVAL literals at parameter-bind time.
2. Q7 timeout mechanism: Option A `tokio::time::timeout` vs Option B DuckDB `interrupt_handle()`. Default к A; upgrade к B if available.
3. Connection acquisition: explicit `TriageSqlState` constructor in `pulse-app/src/main.rs` sharing the DuckDB `Arc<Mutex<Connection>>` from `BufferState`. Verify at /implement.
4. `MockArrowBatch::builder()` existence: combined.md tests extract referenced it but research did not confirm. /implement to verify; fallback к direct `Connection::execute("INSERT ...")` test fixture helper inline.
