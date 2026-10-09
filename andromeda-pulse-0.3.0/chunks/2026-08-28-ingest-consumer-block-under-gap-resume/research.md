# Codebase Research — 2026-08-28-ingest-consumer-block-under-gap-resume

## Scope
- **Depth:** deep · **Reads:** 11 · **Globs/Greps:** 8 · **Graph queries:** 2 (rust plane, `db_state` warm — refreshed 2026-08-28T18:29Z)

## Headline finding (a premise correction, not a detail)

**The shared appender connection has exactly ONE production user: `dispatch_batch`.** Every other DuckDB
consumer already holds a dedicated `try_clone()`d connection:

| consumer | dedicated connection | site |
|---|---|---|
| retention sweep | **YES** — since chunk #99 | `crates/buffer/src/retention.rs:72-86` |
| triage baseline / L1a | YES | `crates/triage/src/baseline/sql.rs:314`, `:460` |
| viz queries | YES — since 2026-08-26 | `crates/viz/src/query.rs:114` |
| **consumer / appender** | **NO — the shared `conn`** | `crates/buffer/src/consumer.rs:104` |

The working entry, `obs-plan.md` §10 defect 4, and `.claude/rules/observability.md` all state the mechanism
as *"`dispatch_batch` takes `conn.lock()` FIRST and holds it across every append, so BOTH shared-connection
users die — the consumer and the retention sweep."* **The second half is false at HEAD.** `run_retention`
touches the shared `conn` exactly once — to make its clone at task start (`:73`) — and every sweep thereafter
runs on `sweep_conn`, a distinct `Arc<Mutex<Connection>>` (`:94` → `run_one_sweep` `:103` → `retention_sweep_inner`
`:165`, where the `conn` parameter is the *sweep* connection). A held `conn` mutex cannot block `sweep_conn.lock()`.

This does not weaken the defect — the wedge is measured and reproducible — it re-aims the attribution.

## Graph impact (rust plane; trace at `.andromeda/runs/2026-08-28T19-05-00Z-phase/tree-query-*.json`)

- **`consumer/run_consumer()`** — def `crates/buffer/src/consumer.rs:33`. **9 out-of-crate callers**: production
  spawn `pulse-app/src/main.rs:1154`, plus 7 `pulse-app/tests/` harnesses (`e2e_drain_template_assignment.rs:154`
  · `e2e_p1_otlp_grpc_to_traces_query.rs:100` · `e2e_p6_channel_arrow_ipc.rs:84` ·
  `e2e_pii_bare_credential_stored_field.rs:89` · `e2e_storm_detection.rs:150` · `perf_load_profiles.rs:79` ·
  `perf_slo_10k_spans.rs:125`) and the `lib.rs:14` re-export. Plus **8 in-crate `consumer::tests` callers**
  (`:414`, `:453`, `:495`, `:536`, `:575`, `:623`, `:654`, `:762`). **Any signature change threads 17 sites.**
- **`consumer/dispatch_batch()`** — def `:95`. **ONE caller**: `run_consumer` `:62`. Crate-private, so its
  internals are freely re-shapeable; the blast radius is entirely inside the loop.
- **`consumer/observe_spans_for_baseline()`** — def `:233`. **ONE caller**: `run_consumer` `:52`.
- **`retention/run_retention()`** — def `crates/buffer/src/retention.rs:48`. Production spawn
  `pulse-app/src/main.rs:1170`; test callers `retention.rs:482`, `perf_load_profiles.rs:515`.

## Files inspected
- `crates/buffer/src/consumer.rs` (1–260, plus test-name survey) — the serial loop and its two work items.
- `crates/buffer/src/retention.rs` (40–130, 160–170) — the dedicated-clone shape and the sweep path.
- `crates/buffer/src/appender.rs` (632–653) — `append_record_batch_to_table`: `conn.appender()` →
  `append_record_batch()` → `flush()`, three DuckDB calls under the held guard.
- `crates/buffer/src/state.rs` (surface survey) — `record_rows_appended` / `record_redactions` / snapshot.
- `crates/ingest/src/channel.rs` (1–50) — `MPSC_CAPACITY = 1024`; `capacity_pct()` is `used/capacity`.
- `pulse-app/src/main.rs` (274–292, 1140–1185) — runtime construction and the two spawn sites.
- `xtask/src/gap_resume.rs` (40–80, 134–180, 189–300) — options, `judge()`, `drive()`, the arms.
- `xtask/src/main.rs` (55–72, 191–205) — the `smoke:gap-resume` CLI surface.

## Patterns detected
- **Serial consume loop** (`consumer.rs:45-70`): `recv().await` → inline `observe_spans_for_baseline` →
  `spawn_blocking(dispatch_batch).await`. One batch at a time, and the `.await` on the join handle means a
  blocking call that never returns stops the loop from ever calling `recv()` again — the channel then fills
  to `MPSC_CAPACITY` (1024) and `ingest.channel.full` fires. This is the observed wedge shape exactly.
- **Lock-first dispatch** (`consumer.rs:104`): `conn.lock()` is taken before the match, held across every
  `append_table_traced` call (spans + span_events, or metrics, or logs + `log_templates`), and dropped at
  `:170` before `record_rows_appended`.
- **Dedicated-connection with warn-on-fallback** (`retention.rs:72-86`; mirrored in `viz::query::read_connection`
  and `TriageSqlState`): clone once at construction, `tracing::warn!(target: …, fallback = "shared_connection")`
  if the clone fails. This is the established shape the repair would copy if it gives the consumer its own
  connection.
- **Post-append per-table redaction fold** (`consumer.rs:112`, `:126`, `:133`, `:150`): `state.record_redactions`
  fires after each table's own append, so a batch rejected at `flush()` contributes zero. obs-plan §5 records
  this siting as structural; any restructure of `dispatch_batch` must preserve it.

## Conventions to follow
- **`tokio::task::spawn_blocking` for every DuckDB call** — stated at `consumer.rs:24-27` and restated at
  `retention.rs:40-42` as the chunk #20 precedent.
- **Dedicated-clone + `fallback = "shared_connection"` WARN** — `retention.rs:76-82` is the canonical text.
- **Crate-private helpers, contract-module-only `pub`** — `dispatch_batch` and `observe_spans_for_baseline`
  are both private; only `run_consumer` / `run_retention` are re-exported (`crates/buffer/src/lib.rs:14`, `:20`).
- **In-crate `consumer::tests` pins that discriminate as a pair** — `dispatch_folds_redactions_after_a_successful_append`
  (`:623`) / `…_when_the_append_is_rejected` (`:654`).

## What the measurement excludes, and what it leaves

- **Candidate (b), blocking-pool starvation — WEAK.** `pulse-app/src/main.rs:276` builds the runtime with
  `Builder::new_multi_thread().enable_all()` and sets **no `max_blocking_threads`**, so the pool is tokio's
  default **512** threads. Arm A ran with the amplifier bound in force (650 L1a queries vs the original
  wedge's 26,821), so there is no plausible source of ~512 concurrent blocking tasks. One stuck append cannot
  starve the pool.
- **Candidate (a), the inline baseline tap — NOT EXCLUDED but does not fit the retention evidence.** If
  `observe_spans_for_baseline` (`:234`) blocked on the async task, `dispatch_batch` would never start, the
  shared `conn` mutex would stay FREE, and retention — on its own connection and its own `spawn_blocking` —
  would be entirely unaffected. Retention did stop, so (a) alone does not explain the observation.
- **Candidate (c), an ENGINE-level DuckDB block — LEADING, and newly named.** If `dispatch_batch` is stuck
  inside one of the three DuckDB calls at `appender.rs:638-651` while holding a DuckDB-internal
  write/catalog/checkpoint lock, then retention's `DELETE` queues behind it *in the engine* despite running
  on a cloned connection — and both die with no Rust-level lock relationship. Chunk #99 recorded exactly this
  class ("L1a reads queued behind append-path row-group maintenance"), and `retention.rs:66-69` already
  documents that "DuckDB MVCC serializes the cloned connections internally per-operation".

**The decisive discriminator is cheap and has in-repo precedent.** (a)/(b) predict the shared `conn` mutex is
FREE during the wedge; (c) predicts it is HELD. Chunk #99 used "lock-probe forensics" to establish exactly
this ("showed the mutex held continuously from the second sweep onward"). A probe that reports whether
`conn.try_lock()` succeeds during a stall separates the candidates in one run.

## New files to create
- *(none identified)* — the reconnect-without-gap arm extends `xtask/src/gap_resume.rs`; the repair lands in
  `crates/buffer/src/consumer.rs`. Whether a probe helper earns its own module is an implementation call.

## Files to modify
- `xtask/src/gap_resume.rs` — a third arm. `GapResumeOptions` (`:49`) and `drive()` (`:246`) can express the
  drive shape today (`--gap-seconds=0` takes the gap branch and sleeps zero), **but `judge()` (`:134`) cannot
  judge it**: it has two arms only — `sustained` (a storm is a control FAILURE) and gap (no storm ⇒
  INCONCLUSIVE). A reconnect run has no storm by construction and is not `sustained`, so it is judged
  INCONCLUSIVE whatever the consumer does. The arm needs a mode discriminant plus its own precondition
  (what makes ITS verdict countable — the analogue of `StormEvidence::fired()` at `:78`).
- `xtask/src/main.rs` — the `SmokeGapResume` variant (`:63`) and its dispatch (`:198`) gain the mode flag.
- `crates/buffer/src/consumer.rs` — the repair. Note `dispatch_batch` has ONE caller, so its internals are
  freely re-shapeable; `run_consumer`'s signature is the expensive surface (17 threading sites).
- `crates/buffer/src/retention.rs` — read-only unless the repair changes the topology it documents.
- Crate-local companions if `run_consumer`'s signature changes: the 7 `pulse-app/tests/` harnesses and the
  8 in-crate `consumer::tests` callers enumerated under Graph impact.

## Scope premise closure (executed — `scope.md` amended before this file was written)
- §3 retention/discriminator bullet — **`[premise-corrected]`**: stronger than the tag anticipated. Not merely
  "the evidence does not discriminate", but "the stated mechanism names a shared-connection user that has not
  existed since chunk #99". Correction owed to three artifacts at wrap.
- §5 new-arm bullet — **VERIFIED necessary**, with the precise reason (`judge()`'s two-arm shape), not the
  guessed one (CLI expressiveness).
- §6 no-new-crate-edge bullet — **VERIFIED achievable**; both touchpoint crates are existing members.
- §6 gate-grade decision — unchanged; still a decision to surface, now with its cost known.
- A new §5 bullet was added recording `run_consumer`'s 17-site threading surface (graph-derived, not recalled).

## Open questions
- **Is the shared `conn` mutex HELD or FREE during the wedge?** → blocks: plan-decision. This is the (a)/(b)
  vs (c) discriminator; the repair's shape depends on the answer (relocate the tap / bound the pool vs.
  give the appender path its own connection and/or bound the DuckDB call). The plan must sequence this probe
  before prescribing a fix — which is also what scope §4's mandated ordering already requires.
- **Does the reconnect-without-gap arm reproduce the wedge at all?** → blocks: plan-decision. If it does, the
  trigger is the new gRPC connection and the 180 s idle is incidental; if it does not, the idle gap (or the
  storm it provokes) is load-bearing. Scope §4 fixes this as step 1.
- **Does anything else hold a DuckDB engine-level lock across the resume boundary?** → blocks:
  implementation-scope. `viz` polls on its own connection while the Traces surface is open, and L1a runs
  during the storm; both are engine-level co-tenants even though neither shares the Rust mutex.
