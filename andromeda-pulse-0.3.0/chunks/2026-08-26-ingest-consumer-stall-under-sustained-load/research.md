# Codebase Research — 2026-08-26-ingest-consumer-stall-under-sustained-load

## Scope
- **Depth:** deep · **Reads:** 11 · **Globs/Greps:** 14 · **Graph queries:** 1 (rust plane, `db_state: fresh`, 33 rows)

## Files inspected
- `crates/buffer/src/consumer.rs` (1–260) — the drain loop and `dispatch_batch`; the wedge site.
- `crates/buffer/src/retention.rs` (48–105, 159–200) — sweep task; dedicated `try_clone()` connection + `MissedTickBehavior::Delay`.
- `crates/buffer/src/state.rs` (1–80) — `BufferState` counter storage.
- `crates/buffer/src/drain.rs` (grep) — `DrainMiner` lock topology.
- `crates/ingest/src/channel.rs` (21–50) — bounded mpsc + `try_send`.
- `crates/ingest/src/grpc.rs` (147, 185, 222) — the three `try_send` call sites.
- `crates/viz/src/query.rs` (100–130 + lock-site grep) — the three production query fns.
- `pulse-app/src/viz_routers.rs` (25–100) — how those queries are scheduled.
- `crates/triage/src/baseline/sql.rs` (277–500) — L1a Q1–Q7 connection discipline + the chunk-#99 timeout rationale.
- `crates/triage/src/baseline/mod.rs` (182–300) — `BaselineState` lock topology (the pre-dispatch tap).
- `pulse-app/src/main.rs` (272–290, 1150–1200) — runtime construction + consumer/retention wiring.
- `pulse-app/src/heartbeat.rs` (128–210) — `ingest.tick` / `buffer.tick` emit sites and their field sets.

## Graph impact (rust plane, `rows: 33`, 19 non-test)
- **`run_consumer`** — exactly ONE production caller: `pulse-app/src/main.rs:1154` (`tauri::async_runtime::spawn`). Plus 6 in-crate tests at `consumer.rs:414/453/495/536/575/762`.
- **`dispatch_batch`** — one production caller, `run_consumer` @ `consumer.rs:62`; 2 direct in-crate tests (`consumer.rs:623`, `:654`).
- **`append_record_batch_to_table`** — production callers are `append_table_traced` @ `consumer.rs:188` and the four `#[cfg(test)]` wrappers in `appender.rs:668/692/717/742`; `write_template_to_table` is the only other DuckDB write. Confirms the amendment-history claim that this is the crate's single write path.
- **Load-suite consumers** (test-side but load-bearing for this chunk): `pulse-app/tests/perf_load_profiles.rs:37,79` and `pulse-app/tests/perf_slo_10k_spans.rs:25,125` — the chunk-#99 four-profile suite already boots the full rig.

## Patterns detected
- **Strictly-serial drain** (`consumer.rs:47–90`): `while let Some(batch) = receiver.recv().await` → `spawn_blocking(dispatch_batch)` → `.await` the join → only then loop. One indefinitely-blocked dispatch stops all draining. The loop's ONLY error arms (`Ok(Err(e))`, `Err(join_err)`) both log ERROR on `duckdb.append` — so a wedge with 0 ERROR is a dispatch that never *returns*, not one that fails.
- **Single-guard batch append** (`consumer.rs:100–175`): `conn.lock()` is taken once and held across every append in the batch (spans + span_events, or metrics, or logs + `write_template_to_table` per new template), released by `drop(guard)` immediately before `state.record_rows_appended(rows)`. `rows_ingested` therefore advances only after the whole batch lands.
- **Counters are lock-free** (`state.rs:1–80`): `rows_ingested` and all siblings are `AtomicU64`; `record_rows_appended` is a `fetch_add`. The counters cannot themselves deadlock, and a frozen `rows_ingested` means the fold was never reached.
- **Chunk-#99 connection isolation, partially applied**: `retention.rs:73` and `baseline/sql.rs:314` both `try_clone()` with a loud `warn!` fallback to the shared connection; `run_q7_with_timeout` (`sql.rs:448–470`) runs the primary CTE on a clone with an `interrupt_handle()` so an abandoned task "holds nothing shared".
- **Timeout abandons but cannot cancel** (`sql.rs:441–447`, verbatim rationale): `tokio::time::timeout` around `spawn_blocking` leaves the blocking task running. Chunk #99 measured this convoying on the shared mutex.
- **`BaselineState` is `DashMap` + atomics** (`mod.rs:182–201`): the pre-dispatch tap (`observe_spans_for_baseline`, which runs on the *async* task before `spawn_blocking`) contends per shard, not globally.

## Conventions to follow
- **Tick-aggregated counters ride the existing heartbeat**: `ingest.tick` carries `span_count` / `buffer_capacity_pct` / `broadcast_subscribers` (`heartbeat.rs:137–144`); `buffer.tick` carries the `rows_ingested` family (`heartbeat.rs:200–210`). A progress signal belongs on this cadence, not per batch.
- **Loud, bounded, once-per-transition WARN** for a condition indistinguishable from health: `app.boot.buffer.degraded` (`main.rs:1186–1191`) with static `reason` + `consequence` fields is the in-repo template, and its stated rationale is this chunk's failure class verbatim.
- **Fallback WARNs already exist** on both clone paths (`buffer.retention` `fallback=shared_connection`; `triage.sql` `fallback=shared_connection`) — whether either fired in the measured run is a first thing for the RED leg to read.
- `pulse-app` probes live in `pulse-app/tests/*.rs` (`[lib] test = false`); `buffer`/`ingest` keep co-located `#[cfg(test)] mod tests`.

## Findings that decide the plan

**1. The obs-plan §10 isolation invariant is VIOLATED at HEAD by `viz`.**
`crates/viz/src/query.rs` locks the **shared appender connection** at `:114` (`query_traces`), `:211` (`query_metrics`), `:307` (`query_logs`). There is **no `try_clone` anywhere in `crates/viz`**. Each is dispatched via `spawn_blocking` with **no timeout** (`pulse-app/src/viz_routers.rs:32,63,94`). obs-plan §10 states: *"Any future DuckDB consumer with a multi-second statement MUST take a dedicated `try_clone()` connection, never the shared appender connection."* `query_traces` runs a `COUNT_TRACES` plus the page query under one guard on a table that grows for the whole run — and since `2026-07-07-traces-table-auto-refresh` the Traces table **re-polls on a timer**, so these are periodic, not user-driven.

**2. A second shared-connection consumer: the Q7 fallback.**
`sql.rs:490` runs `run_q7_fallback_blocking(&conn, …)` on the **shared** connection with **no timeout**, reached exactly when the primary already timed out — i.e. when the database is slow.

**3. The blocking pool is at its default cap.**
`pulse-app/src/main.rs:276–279` builds the runtime with `new_multi_thread().enable_all().build()` — `max_blocking_threads` is **not set**, so it is tokio's default 512. Every viz poll, every L1a query, every retention sweep and the consumer draw from that one pool. Tasks parked on the shared `std::sync::Mutex` occupy pool threads while parked, and the consumer's own `spawn_blocking` queues behind them — which resolves *never* if the head-of-line holder does not return. Heartbeats are async and keep ticking throughout, which matches the measurement exactly.

Together these give a coherent, code-supported mechanism for a permanent, silent wedge. **It is not yet attribution** — Deliverable A must confirm which holder actually blocked, on evidence from a reproduced run.

**4. There is no progress detector of any kind.**
`grep` for stall/wedge/no-progress/last-append across `heartbeat.rs` and `state.rs` returns only an unrelated comment. Nothing reads `rows_ingested` and `buffer_capacity_pct` together; `BufferState` records no last-append timestamp. This confirms scope's `✘` correction — Deliverable B builds the first such signal, it does not extend one.

## Extract questions answered
Two extracts explicitly deferred a question to research. Both are answered here.

- **arch:** *"Whether the running code ever sets `degraded` when the consumer wedges is research's question."*
  **No — it cannot, by construction.** `crates/ui-bridge/src/health.rs:190-192` drives
  `subsystems.ingest_channel.last_tick_at` / `subsystems.buffer.last_tick_at` from `HeartbeatState`'s
  **tick** timestamps, and the ticks kept firing throughout the measured wedge. `health` therefore reports
  LIVENESS (did the tick fire), never PROGRESS (did rows advance). The `ready` envelope does carry both
  `ingest_mpsc_capacity_pct` (`crates/ui-bridge/src/contract.rs:380`) and `rows_ingested` — the two numbers
  Deliverable B needs — but nothing correlates them. So arch's "maps onto a declared contract rather than a
  new surface" holds for the FIELDS while the DERIVATION is entirely absent.
- **security:** *"Whether the `tower_governor` rate-limit layer is installed at HEAD is research's question."*
  **It is installed** — `crates/ingest/Cargo.toml:29` plus `crates/ingest/src/grpc.rs:7-9` and the
  `GovernorConfigBuilder` at `:367` with a `GlobalKeyExtractor`. It did not and could not bound the measured
  window: a request-rate cap limits acceptance rate, not consumer progress, so a wedged consumer still fills
  the channel at whatever rate the limiter permits. No rate-limit change is implied by this chunk; the
  criterion is no-regression.

## Files to modify (provisional — the fix shape is Deliverable A's outcome)
- `crates/viz/src/query.rs` — candidate: take a dedicated `try_clone()` read connection (mirroring `retention.rs:73` / `sql.rs:314`, warn-on-fallback). Callers: `pulse-app/src/viz_routers.rs:32,63,94` (the only production callers; MCP reaches the same fns via its own path).
- `crates/triage/src/baseline/sql.rs` (~490) — candidate: the Q7 fallback's shared-connection use.
- `crates/buffer/src/state.rs` — candidate home for a last-append instant / progress accessor (atomics already; add in the same style).
- `pulse-app/src/heartbeat.rs` (~128–210) — where the two counters are already emitted together; the natural site to correlate them.
- `pulse-app/src/observability.rs` (~147) — the allowlist; any new field needs its own **exact** leaf. **This file is a listed boot-path trigger, so editing it pulls in the boot-smoke gate** (test-plan §3).
- **Threading/companions from the graph:** `crates/buffer/src/consumer.rs` tests at `:414/453/495/536/575/623/654/762` pin the drain loop and the redaction fold — any signature change to `dispatch_batch`/`run_consumer` touches all of them. `pulse-app/tests/perf_load_profiles.rs` + `perf_slo_10k_spans.rs` boot the full rig and are the existing home for a load-shaped assertion. `crates/buffer/src/lib.rs:14` re-exports the consumer surface.
- **No new workspace crate, no new dependency** is implied by any candidate.

## Scope premise closure
Applied to `scope.md` (see that file; corrections written there before P4):
1. **Consumer strictly serial** — **VERIFIED** (`consumer.rs:47–90`); tag dropped.
2. **`dispatch_batch` holds `conn.lock()` across the batch; only error arms log** — **VERIFIED** (`consumer.rs:100–175`, error arms `:78–90`); tag dropped.
3. **"obs-plan §10 is honoured on the paths that exist today"** — **FALSIFIED**. `viz`'s three production query fns and the Q7 fallback use the shared connection; only retention, Q1–Q6 and the Q7 primary honour it.
4. **"The wedge is permanent, not slow"** — **not decidable by code reading**; promoted to an Open Question below, owned by the RED leg.
5. **"A new field needs its own exact allowlist leaf"** — **VERIFIED and upgraded**: it is a standing rule (obs-plan §8 per-target-own-leaf) and an arch mandate, not an inference.

## Open questions
- Is the wedge a permanent block or an unbounded convoy that would eventually drain? → blocks: **implementation-scope** — decides whether Deliverable B's predicate is "no progress while the feed is live for N ticks" (tolerating the in-spec >60s append pause obs-plan §10 characterizes) or a hard deadlock signal. The RED leg answers it.
- Did either `fallback=shared_connection` WARN (`buffer.retention` / `triage.sql`) fire in the measured run? → blocks: **plan-decision** — a fired fallback would put retention or L1a back on the shared connection and change which holder is prime suspect. Cheap to read from the predecessor run's log before the RED leg.
- Does the wedge reproduce with the webview closed (no viz polling)? → blocks: **plan-decision** — the single cleanest discriminator between the viz-contention mechanism and an appender-internal one.
