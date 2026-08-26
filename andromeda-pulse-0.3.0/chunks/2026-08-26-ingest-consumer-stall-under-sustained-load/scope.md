# Scope — 2026-08-26-ingest-consumer-stall-under-sustained-load

**Working-route entry (verbatim intent):** Ingest consumer stall under sustained load — the buffer keeps
draining, and a wedged consumer is visible instead of silent.

**Epoch:** Epoch 4 — Polish & ship: verification · **Version:** andromeda-pulse-0.3.0

---

## Outcome

Two things must be true when this chunk is done:

1. **The buffer keeps draining.** A sustained OTLP feed at ordinary rates does not permanently stop
   `buffer.tick.rows_ingested` from advancing. Spans accepted into the ingest channel reach DuckDB.
2. **A wedged consumer is visible instead of silent.** If the consumer does stop draining — for this cause or
   any future one — the condition surfaces on the app's own record. Today it does not: the measured wedge
   produced **0 ERROR, 0 `app.panic.fatal`, and healthy heartbeats throughout**, which is externally
   indistinguishable from health.

Half (2) is the harder requirement and the one that must not be traded away: a fix without an observable
leaves the next occurrence just as silent.

## The measurement this chunk answers to

Measured 2026-08-26 at `2026-08-25-demo-injector-formalized-api-surface-retire`, by the sustained injector
profile that chunk shipped. Under a moderate sustained feed (~54 spans/s — far below the 10k spans/s design
target in arch §Established Decisions [Database]):

- `buffer.tick.rows_ingested` froze at **2187** and never advanced across **56 consecutive ticks**;
- `ingest.tick.buffer_capacity_pct` climbed monotonically 0 → 6 → 17 → 29 → 40 → 52 → 63 → 75 → 86 → 98 →
  **100 %** and pinned there;
- **27,675 spans** were accepted into the mpsc and never persisted;
- the receiver then returned `ResourceExhausted` to every subsequent export;
- the run's own obs log carried **0 ERROR and 0 `app.panic.fatal`**, heartbeats ticking throughout.

Why 100 chunks never saw it: the finite ~600-batch storm profile ends before the wedge is reachable. Only the
unbounded `--sustained` profile exposes it. Not caused by that chunk — `LwwQueue::drain_all`, the code it
deleted, was graph-proven to have 0 production callers.

Evidence lives in that chunk's `report.md` (§ Surfaced not fixed, item 2). Cite the path; do not copy content.

## Deliverable A — root-cause the consumer stall

Attribute the stall to a mechanism, on evidence, before changing anything. The route entry names candidates
that were **observed alongside** the stall and are explicitly **not yet attributed**:

- 454 `digest.lww.drop drop_reason=queue_cap_reached` on the tier-1 path;
- 11 `interpretation.inference.error error_category=broadcast_lagged`;
- L4 running at ~4.3 s per inference;
- DuckDB append contention, per the obs-plan §10 connection-isolation invariant.

Correlation is not attribution. The chunk must show which mechanism holds the consumer, and say so from the
evidence — a candidate that is ruled out is a result worth recording, not a gap.

## Deliverable B — make the condition observable

A saturated ingest channel, or a `rows_ingested` that has stopped advancing while the feed is live, must
surface on the app's own record. Two properties the observable must have:

- **It must distinguish "the producer stopped" from "the consumer stopped."** Both leave `rows_ingested`
  static; only the second is a defect. `buffer_capacity_pct` already separates them in principle (a stalled
  producer drains the channel; a stalled consumer fills it) — the two counters exist and both were emitting
  throughout the measured wedge. What is missing is anything that reads them **together** and says so.
- **It must not depend on a human reading a heartbeat.** The measured run emitted every number needed to
  diagnose the wedge and still read as healthy for 12 minutes.

Explicitly in scope as the *reason* half B exists: **the existing feed precondition does not catch this.**
`rows_ingested > 0` holds at 2187 — it proves the producer RAN, never that it is still running. (See
§Coordinate corrections: that precondition is a documented discipline, not a shipped mechanical check.)

## Boundaries — out of scope

- **Not a throughput/performance chunk.** The target is "does not permanently wedge at 54 spans/s", not
  "sustains 10k spans/s". Any headroom gained is incidental.
- **Not the L4 interpretation path's own defects.** `broadcast_lagged` and the ~4.3 s inference are candidate
  *causes* to rule in or out here; the brief's content defects belong to the `Interpretation brief
  completeness` entry.
- **Not the muted-diagnostics sweep.** `Diagnostics un-muting + harness-truth sweep` owns the 5 redacted obs
  targets and its CARRY cluster. If a target this chunk needs is muted, un-mute exactly that one and say so —
  do not absorb the sweep.
- **No new ingest-side backpressure policy** (dropping, sampling, spill-to-disk) unless the root cause makes
  one unavoidable; `try_send` → `ResourceExhausted` is the arch-declared behaviour and stays.

## Surfaces and contracts in frame

**Amended at P5 validation-1 (intent-incomplete, operator-approved at the P4 review).** This section was
authored at promotion assuming the fix surface was the ingest→buffer seam the working entry names, with
everything else read-only. P3 falsified that: `crates/viz` (three production query fns) and the Q7 fallback
violate the obs-plan §10 isolation invariant on this same seam. The operator's P4 decision was to close both
violations in this chunk on their own merits, so **`crates/viz/src/query.rs` and
`crates/triage/src/baseline/sql.rs` are WRITE surfaces here, not read-only anchors** — while attribution of the
measured wedge remains owed on evidence from the RED leg and is not assumed by those fixes. The rest of the
table stays read-only unless the root cause requires otherwise.

Anchors:

| Surface | Path | Role |
|---|---|---|
| ingest→buffer mpsc | `crates/ingest/src/channel.rs:26` (`try_send`), `:46` (bounded `mpsc::channel`) | where saturation becomes `ResourceExhausted` |
| gRPC receiver hand-off | `crates/ingest/src/grpc.rs:147,185,222` | the three `try_send` call sites |
| consumer loop | `crates/buffer/src/consumer.rs:47` (`while let Some(batch) = receiver.recv().await`) | the drain that stopped |
| batch dispatch | `crates/buffer/src/consumer.rs` `dispatch_batch` (`conn.lock()`, appends under one guard) | the awaited `spawn_blocking` body |
| retention sweep | `crates/buffer/src/retention.rs:73` | takes a `try_clone()`d connection |
| L1a baseline reads | `crates/triage/src/baseline/sql.rs:314,460` | take `try_clone()`d connections |
| heartbeat emitters | `pulse-app/src/heartbeat.rs:140` (`ingest.tick`), `:202` (`buffer.tick`) | the two counters that stayed silent-but-correct |
| obs allowlist | `pulse-app/src/observability.rs:147` (`buffer_capacity_pct`) | any new field needs its own exact leaf |
| tier-1 drop counter | `crates/triage/src/digest/assembler.rs:512,520` | candidate-evidence only |
| L4 broadcast lag | `pulse-app/src/inference_runtime.rs:179` | candidate-evidence only |

**Governing invariant** — obs-plan §10 (`.andromeda/obs-plan.md:622-628`), *DuckDB Connection Isolation &
Load-Profile Operational Constraints*: write (appender) / sweep (retention) / read (L1a) isolation on the same
`:memory:` database via `Connection::try_clone()`; **"Any future DuckDB consumer with a multi-second statement
MUST take a dedicated `try_clone()` connection, never the shared appender connection."** A violation of this
invariant would produce exactly the observed signature. Verified at HEAD: retention and both L1a read paths do
honour it.

## Folded annotation — PREREQ (from the working entry)

**`cargo audit` standing deferral.** Origin `2026-08-15-corpus-key-persistence`; ratified at the 2026-08-16
0-pending adaptation wrap (pin #15) with an every-3rd-wrap re-run INTERVAL; re-pinned onto this entry at the
2026-08-26 demo-injector wrap, origin preserved.

- Point 43 was **discharged** at the 2026-08-25 operator-adaptation wrap.
- Point 44 was **SKIPPED per the ratified interval** at the demo-injector wrap, with basis + overlap
  re-verified: `cargo deny check bans licenses sources` ok (exit 0); `cargo deny check advisories` exit 1,
  designed-red at the same **eight** owned upgradeable IDs (0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 /
  0258, set unchanged); the 10-blocks-for-8-IDs counting rule reconfirmed live.
- **Next interval point: 46.** This chunk's wrap is not that point, so the obligation here is to re-verify
  basis + overlap and record `probe skipped per ratified interval (next: 46)` — never a silent skip. Enumerate
  DISTINCT `RUSTSEC-` ids, never error blocks. Full rationale: the `2026-08-15-corpus-key-persistence` report.

No `CARRY:` and no `BLOCKED-ON:` annotation is present on this entry (checked at an annotation position; the
token is absent from the line entirely).

## Coordinate corrections (re-derived at HEAD before shaping scope)

Every coordinate the working entry names was re-derived first-hand. All verified **except one**, corrected here
so it is not re-derived downstream:

- ✔ `buffer.tick` / `rows_ingested` — `pulse-app/src/heartbeat.rs:202`
- ✔ `ingest.tick` / `buffer_capacity_pct` — `pulse-app/src/heartbeat.rs:140`, allowlist leaf at
  `pulse-app/src/observability.rs:147`
- ✔ `digest.lww.drop` + `drop_reason = "queue_cap_reached"` — `crates/triage/src/digest/assembler.rs:512,520`
- ✔ `error_category = "broadcast_lagged"` — `pulse-app/src/inference_runtime.rs:179`
- ✔ obs-plan §10 connection-isolation invariant — `.andromeda/obs-plan.md:622,628`
- ✘ **"this chunk's own feed-precondition assertion"** — there is **no shipped mechanical assertion**.
  `grep rows_ingested` returns nothing in `xtask/src/`, `pulse-app/ui/tests-e2e/`, or `scripts/`. What exists
  is (a) a doc-comment mandate at `crates/ingest/examples/inject_demo.rs:26` ("Any recipe driving this tool
  must assert the FEED PRECONDITION") and (b) a codified discipline at `.claude/rules/testing.md:266` (b)
  ("assert the FEED advanced first — `rows_ingested > 0` — and print `INCONCLUSIVE` rather than a verdict").
  The entry's *claim* is correct — the precondition would not catch this wedge — but the artifact is prose,
  not code. **Consequence for deliverable B:** there is no existing check to extend; whatever observable this
  chunk adds is the first mechanical one on this seam.

## Premise closure (P3 — resolved against research)

Each bullet was tagged `[inferred]` at promotion because the working entry did not state it. P3 verified or
corrected each against the code; corrections are binding on P4.

- The consumer is **strictly serial**: `run_consumer` awaits each `spawn_blocking(dispatch_batch)` join before
  the next `receiver.recv()`, so one indefinitely-blocked dispatch halts all draining — **VERIFIED**
  (`crates/buffer/src/consumer.rs:47-90`).
- `dispatch_batch` holds `conn.lock()` (a `std::sync::Mutex<Connection>`) across all appends in a batch, and the
  loop's only error arms log ERROR on `duckdb.append` — so a wedge with 0 ERROR is a dispatch that never
  RETURNS, not one that fails — **VERIFIED** (`consumer.rs:100-175`, error arms `:78-90`). Corollary now
  established: `BufferState` counters are `AtomicU64` (`state.rs:1-80`), so a frozen `rows_ingested` means the
  fold at `record_rows_appended` was never reached — the counters cannot themselves deadlock.
- ~~Because retention and both L1a read paths use `try_clone()`, the obs-plan §10 invariant is honoured on the
  paths that exist today.~~ **[premise-corrected: `crates/viz/src/query.rs` locks the SHARED appender
  connection at `:114` / `:211` / `:307` and `crates/viz` contains no `try_clone` at all; each is dispatched
  via `spawn_blocking` with NO timeout (`pulse-app/src/viz_routers.rs:32,63,94`), and the Traces table
  re-polls on a timer since `2026-07-07-traces-table-auto-refresh`. A second violator: the Q7 fallback runs on
  the shared connection with no timeout (`crates/triage/src/baseline/sql.rs:490`).]** The corrected statement:
  **the §10 invariant is violated at HEAD by `viz` (three production query fns) and by the Q7 fallback**;
  retention, Q1–Q6 and the Q7 primary honour it. Shared-connection contention is therefore a
  *first-class* candidate with a named mechanism, not a residual one — and the fix surface is wider than the
  `buffer` crate the working entry names.
- ~~The wedge is permanent, not slow.~~ **[premise-corrected: not decidable by code reading — promoted to
  `research.md` §Open questions, owned by the RED leg.]** Code reading did establish the mechanism *class*
  that would make it permanent (an unbounded shared-mutex convoy plus `tokio::time::timeout` abandoning
  rather than cancelling a blocking task — `sql.rs:441-447`, chunk #99's measured finding — against a
  blocking pool left at tokio's default 512 cap, `main.rs:276-279`). Which it is decides whether half B's
  predicate is "no progress for N ticks while the feed is live" or a hard-deadlock signal.
- Whatever field half B adds needs its **own exact obs allowlist leaf** — **VERIFIED, and upgraded from
  inference to mandate**: it is a standing rule in obs-plan §8 (per-target-own-leaf) and an arch requirement
  (§Occupied Resources, delegated-timing entry), not a guess.

**One more fact research settled, which the entry assumed the opposite of:** there is no progress detector of
any kind at HEAD. Nothing reads `rows_ingested` and `buffer_capacity_pct` together, and `BufferState` records
no last-append timestamp. Deliverable B builds the first such signal rather than extending one.

## Provenance

- Working-route entry (Epoch 4) — the intent anchor.
- `andromeda-pulse-0.3.0/chunks/2026-08-25-demo-injector-formalized-api-surface-retire/report.md` — the
  measurement.
- `.andromeda/obs-plan.md` §10 — the connection-isolation invariant.
- `.claude/rules/testing.md:266` — the feed-precondition discipline.
