# Codebase Research — 2026-08-28-ingest-consumer-initiating-freeze

## Scope
- **Depth:** deep · **Reads:** 9 source regions · **Globs/Greps:** 11 · **Log analyses:** 4 passes over the
  131 MB preserved corpus (386,279 lines) · **Code-graph queries:** 1 (`rust` plane, `db_state: fresh`, 11 rows)

---

## THE HEADLINE: the entry's causal premise is FALSIFIED

The working entry states:

> the freeze happened under **LIGHT load** and the cue storm **FOLLOWED it by 32 seconds**; the runaway is an
> amplifier and recovery-blocker, **never the trigger**.

**All three clauses are false.** The reconstructed timeline, from the preserved log's own `ingest.tick` /
`buffer.tick` / `cadence.trigger` records:

| time | event | evidence |
|---|---|---|
| 17:19:44.196 | `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS=20` override (prod default 3600) | `triage.baseline.bootstrap_window.override` `resolved_seconds=20` |
| 17:20:09–17:20:50 | **injector run 1**; 132 `duckdb.append`; `rows_ingested` 0 → 2187 | append `duration_ms` 2–3 throughout |
| 17:20:50.787 | last `duckdb.append` — **end of run 1, not a freeze** | — |
| 17:21:01 → 17:24:01 | **producer SILENT.** `ingest.tick.span_count` flat at 2187, `buffer_capacity_pct` **0.0** for 13 consecutive ticks | the consumer had **nothing to consume** |
| 17:21:22.138 | `service_went_silent` cues begin — exactly 20s after the producer stopped (the overridden window) | 4,590 emissions |
| 17:22:21.136 | tier2 cadence triggers on `service_went_silent` (600) | — |
| **17:23:21.134** | **tier1 cadence triggers on `service_went_silent` (2,061)** — runaway at full rate | — |
| 17:24:01 → 17:24:16 | **producer RESUMES.** `span_count` 2187 → 2295 → 3078 → 3888…; `buffer_capacity_pct` 0.0 → 0.29 → 3.1 → 6.1 → … → 100 | — |
| **17:24:16 onward** | **the actual consumer stall.** Channel fills monotonically; `rows_ingested` never leaves 2187 for 16 min | `buffer.tick` ×70 |
| 17:24:13.971 | last `viz.query.traces` | — |

**So:** the stall begins when input RESUMES at 17:24:16, by which point the tier1 runaway had been at full rate
for **~55 seconds**. The load at freeze onset was **heavy, not light**, and the storm **preceded** the stall
rather than following it. The "~1.7 blocking queries/sec across the 66-second pre-freeze window" figure is
accurate *about 17:20:50* — but 17:20:50 is the end of a healthy injection with an empty channel, not the freeze.

**Consequence:** the runaway is a live TRIGGER candidate, not merely an amplifier — which is what the successor
chunk `2026-08-26-cadence-runaway-blocking-pool` bounded. That materially changes what this chunk should do
(see §Open questions Q1 — a plan-decision blocker).

## Graph impact (code-graph query, `rust` plane, 11 rows)

- **`run_q7_with_timeout`** — `crates/triage/src/baseline/sql.rs:448`; callers `run_q7` (`:427`) → the
  `SqlQueryRunner` trait (`cadence/coordinator.rs:102`) → `CadenceSqlRunner` (`pulse-app/src/cadence_runner.rs:50`).
- **`viz::query::read_connection`** — `crates/viz/src/query.rs:112`, plus its lone pin at `:497`.
- The remaining 6 rows are `#[cfg(test)]` stub impls (`CountingSqlRunner`, `CannedSqlRunner` ×2).

## Files inspected

- `crates/triage/src/baseline/sql.rs` (425–504) — **the scope's candidate mechanism is ALREADY REPAIRED.**
  `run_q7_with_timeout` takes a dedicated `try_clone()`d connection with its own mutex, grabs
  `clone.interrupt_handle()` BEFORE spawning, and calls `interrupt.interrupt()` on the timeout arm (`:482`).
  The doc comment at `:438-447` **documents that repair**; it is not an open-hazard marker.
- `pulse-app/src/main.rs` (276–300) — `Builder::new_multi_thread().enable_all().build()`, **no
  `max_blocking_threads`** ⇒ tokio default **512**. The entry's "512-thread default pool" premise VERIFIES.
- `crates/buffer/src/consumer.rs` (1–90, 234–264) — the consumer loop. **`observe_spans_for_baseline` runs
  INLINE on the async task, BEFORE the `spawn_blocking`** (`:53`, fn at `:234`). It calls
  `observer.observe_span(...)` synchronously per span. A block there parks a runtime WORKER thread, not a
  blocking thread, and produces exactly the observed signature.
- `crates/buffer/src/retention.rs` (95–130) — `run_one_sweep` `spawn_blocking`s on the **shared** appender
  connection.
- `crates/triage/src/baseline/mod.rs` — `BaselineState` is `DashMap`-backed (`:184-185`); comments at
  `:459`/`:514`/`:544` state snapshots are taken so downstream work does **not** hold shard locks.
- `pulse-app/src/restart_observer.rs` (45–90) — `CompositeSpanObserver` fans out to each observer serially on
  the calling (async) task.

## Patterns detected

- **Only ONE production `tokio::time::timeout` over `spawn_blocking` exists** (`sql.rs:465`). Every other
  `timeout(` hit in the workspace is `#[cfg(test)]` or the llamacli subprocess guard. So the
  "timeout abandons a mutex-holding blocking task" class has exactly one site, and it is repaired.
- **`interrupt_handle()` appears at exactly two lines repo-wide** (`sql.rs:462`, `:482`) — no other DuckDB
  consumer can abort a running statement.
- **Two facts jointly exclude the simplest explanations** (`crates/buffer/src/retention.rs` + the log):
  `buffer.retention.sweep` succeeded at 17:21:25 and 17:23:05 on the **shared appender connection**, so
  neither that mutex nor the blocking pool was exhausted at those moments.
- **Clean-log wedge confirmed:** 386,279 lines, `INFO` 383,553 + `WARN` 2,726, **ERROR 0**, 0 panics. The WARN
  mass is `digest.lww.drop queue_cap_reached` (2,646 across tiers) plus one
  `interpretation.inference.error{error_category: broadcast_lagged}`.

## Conventions to follow

- **Obs exact-leaf discipline** — a bare `buffer` key exists, so any new `buffer.*` target without its own
  exact leaf resolves to the tick field set and redacts silently (`pulse-app/src/observability.rs:217` is the
  `buffer.consumer.stalled` precedent). Leaf guards live under `pulse-app/tests/` (`[lib] test = false`).
- **Muted diagnostics observed live in this very corpus:** `metric.pipeline.l1a.query_count_total` renders
  `query_name: "<redacted>"`, `triage.incident.auto_resolve.tick` renders `duration_ms` /
  `evaluated_count` / `resolved_count` all `"<redacted>"`, and `triage.cue.tick` shows
  `bypass_triggered: "<redacted>"`. The obs extract's caveat is confirmed at the wire — **the 26,821 L1a
  records cannot be split by query name from this corpus.**
- **Corpus-vs-HEAD timing** — `rows_ingested_delta`, `last_append_age_seconds`, `cycles_executed`,
  `cues_latched`/`latch_tracked` and the completed `triage.cue.tick` leaf ALL post-date 2026-08-25 and are
  absent from this log. Do not read their absence as evidence.

## New files to create
- *(none anticipated — see Open questions Q1; the chunk's shape is not settled until that is answered)*

## Files to modify
- *(deferred to the plan — contingent on Q1)*

## Open questions

1. **Does the successor's `CueLatch` bound already fix the initiating freeze?** → blocks: **plan-decision.**
   The runaway is now a trigger candidate, and `2026-08-26-cadence-runaway-blocking-pool` bounded it at cue
   emission (887 refused vs 22 admitted, L1a 26,821 → 240). If the runaway *was* the trigger, the initiating
   freeze may already be fixed and this chunk's honest work is a **verification** rather than a repair.
   Must be resolved with the operator before P4 synthesis.
2. **Was the consumer already wedged during the 17:21–17:24 quiet window, or did it wedge on resume?** →
   blocks: **implementation-scope.** The channel at 0.0 % means the consumer was untested, not proven healthy,
   for those 3 minutes. Both remain live: (a) it wedged at/after resume under the runaway's blocking-pool
   pressure; (b) it wedged earlier on the last loop iteration. The last append at 17:20:50.787 SUCCEEDED
   (the record is emitted from inside the append path), and the post-append broadcast cannot block with 0
   subscribers — which weakens (b) but does not close it.
3. **Is `observe_spans_for_baseline`'s inline per-span `observe_span` a viable block site?** → blocks:
   **implementation-scope.** It is the only synchronous non-`spawn_blocking` work on the consumer's async
   task. `DashMap` shard contention against the cue evaluator (running every second over 19 services) is the
   candidate; the crate's own comments claim snapshots avoid holding shard locks, so this needs reading
   `snapshot_services` / the persist path rather than assuming.

---

## Scope premise closure

Re-read of `scope.md`'s `[inferred]` bullets against the findings above:

- §2 terminal-outcome ladder — **VERIFIED as a useful frame**, but its *weighting* changed: outcome 3
  ("not attributable") is now much less likely and a fourth shape appeared (attributed + already fixed
  upstream). Amended.
- §3 second evidence corpus — **VERIFIED** present (`D:/dev/evidence/pulse-l4run-20260827-064312/`); not
  needed so far, the wedge corpus was sufficient.
- §3 relayed signature counts — **VERIFIED first-hand**: `cadence.trigger` 2,683 · `triage.cue.emit` 4,599 ·
  `metric.pipeline.l1a.query_count_total` 26,821. All three exact.
- §3 "~1.7 blocking queries/sec / light load / storm followed by 32s" — **PREMISE-CORRECTED** (see headline).
- §4 "512-thread default pool" — **VERIFIED** (`main.rs:276`, no `max_blocking_threads`).
- §4 `sql.rs:441` abandoned-blocking-task candidate — **PREMISE-CORRECTED**: that comment documents a
  landed repair (dedicated clone + `interrupt()` at `:462`/`:482`), and it is the only such site in the
  workspace.
- §6 "no new TauRPC / no DDL / no new crate edge" — **VERIFIED** as still plausible; nothing found requires any.

`scope.md` amended accordingly before P4.
