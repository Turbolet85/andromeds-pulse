# Scope — 2026-08-28-duplicate-span-replay-fails-loudly

**Working entry (verbatim outcome):** Duplicate-span replay fails loudly — a constraint-violating flush never
wedges ingest, and the injector stops colliding with itself across restarts.

**Epoch:** Epoch 4 — Polish & ship: verification
**Predecessor:** `2026-08-28-ingest-consumer-block-under-gap-resume` (attributed the wedge; repair blocked by
scope T3 and moved forward whole)

---

## What this chunk builds

Three items, in the order the entry states them. The first is the defect repair; the second removes the
harness's accidental version of the trigger; the third closes a vacuous-PASS shape in the leg that reads the
verdict.

### 1. The appender: a post-failure flush fails loudly, never hangs

**The defect, as measured at the predecessor:** `append_record_batch_to_table`
(`crates/buffer/src/appender.rs:632`) opens `conn.appender(table)`, calls `append_record_batch()`, then
`flush()`. When a batch violates the `spans` composite PK `(trace_id, span_id)`
(`crates/buffer/src/schema.rs:38`), the FIRST such flush fails loudly and correctly — one `duckdb.append`
ERROR, `reject_reason: "append_failed"`. The **NEXT** batch's `flush()` then hangs unbounded while holding the
shared connection mutex: `rows_ingested` frozen at 3,240, ingest channel 0 → 100 %, `ingest.channel.full`,
`buffer.consumer.stalled` at 450 s, 24k+ accepted spans never persisted — at **0 ERROR and 0 panics after the
first**, because nothing returns, so nothing reports.

**The outcome owed:** a constraint violation is a recoverable, reportable per-batch failure. After one, the
consumer keeps draining and every subsequent batch either succeeds or fails with a returned error.

The entry states the acceptable end-states as a **fork**, and does not pick one:
> the connection must be left usable, **or** the failure must be terminal and reported

Both satisfy "never wedges ingest"; they differ in what happens to ingest afterwards. This fork is a genuine
design decision for P4 and is NOT pre-resolved here.

**Boundary the entry sets explicitly:** this defect is **NOT** a connection-isolation violation. `obs-plan.md`
§10 defect 4 and `.claude/rules/observability.md` §DuckDB connection isolation both record the isolation
mandate as **satisfied by shipped code**, name this entry as the defect's owner, and record that the earlier
reading which implicated the shared connection was measured FALSE. Do not re-open that mandate.

### 2. The demo injector: stop colliding with itself across restarts

**The mechanism, verified at HEAD:** `crates/ingest/examples/inject_demo.rs` derives span identity as a pure
function of a per-process batch counter — `let seq = b * 100_000 + (si as u64) * 1_000 + (oi as u64) * 100 + k;`
(:301) feeding `trace_id(seq)` (:224) and `span_id(seq)` (:231), with `b` starting at 0 in every process
(:277). Process 2 therefore replays process 1's exact primary keys. The harness manufactures the collision.

**The outcome owed:** a restarted injector emits identities distinct from its predecessor's, so a
`smoke:gap-resume` run stops producing an accidental self-collision.

**The nuance the entry raises and does not settle:** it says fixing this "also upgrades arm A from a
self-collision into the production retry-resend shape it was always meant to model." A retrying OTLP exporter
resending spans is normal client behaviour that reaches the hang — so once the accidental collision is gone,
whether the harness still needs a **deliberate** duplicate-resend to keep exercising the appender repair is
a scope question for P4. `[inferred]` — the entry implies arm A should model the production shape, but does
not say by what mechanism.

### 3. The observe-window guard (folded in by the entry)

A leg whose verdict reads the 450 s / 30-tick stall threshold must **refuse to report PASS when its observe
window cannot reach that threshold**. Measured 2026-08-28: `--observe-minutes=3` reported PASS over a consumer
that had in fact stopped, because `ingest_progress::evaluate` is structurally incapable of failing inside a
window shorter than the threshold it reads. `test-plan.md` §3 records this as a known gap owned by this entry.

---

## Acceptance

`cargo xtask smoke:gap-resume` **arm A exits 0 with its preconditions satisfied.**

Two halves, both required — an exit 0 whose preconditions were not met is exactly the vacuous PASS item 3
exists to kill. Arm A's precondition is a silence storm in the log (per `test-plan.md` §3: a gap-arm pass with
no storm reports INCONCLUSIVE, not PASS).

**The RED already exists free.** Arm A is red at HEAD by design — the documented scenario-not-gate treatment —
so no mutation is needed to establish the failing state. Arms B (`--sustained`) and `--reconnect-only` must
stay green / behave as documented.

---

## Boundaries

- **In:** `crates/buffer/src/appender.rs` · `crates/ingest/examples/inject_demo.rs` · `xtask/src/gap_resume.rs`
  (the observe-window guard) and whatever the appender repair minimally requires at its production call site.
- **Out:** the connection-isolation topology (settled, satisfied — see §1 boundary) · the characterized-not-fixed
  DuckDB maintenance pause (needs a ~12.7M-row table; excluded by measurement) · blocking-pool starvation
  (excluded by direct probe at the predecessor) · widening `smoke:gap-resume` into a CI gate (it is a dev-host
  scenario leg by decision).
- **Arm A's red must never be weakened, removed, or `--expect-absent`'d** to reach the acceptance — turning it
  green is this chunk's job, and only by repairing the defect.

## Premise closure (P3 verified these against the code — see `research.md` §Scope premise closure)

1. **VERIFIED** — the repair belongs in `append_record_batch_to_table` (`appender.rs:632`), where the
   `Appender` lives; the mutex is taken by the caller `consumer::dispatch_batch` (`consumer.rs:99`). Nuance:
   the "terminal and reported" fork would additionally need `consumer.rs` and possibly `contract.rs`.
2. **VERIFIED** — `dispatch_batch`'s `Batch::Spans` arm appends `spans` (`consumer.rs:111`) *and* `span_events`
   (`consumer.rs:125`) on the same guard. Replayed spans replay their events, and `span_events` PK
   `(trace_id, span_id, event_index)` collides identically. The repair must hold for every table
   `append_table_traced` serves.
3. **VERIFIED as a real gap** — arm A's storm precondition comes from the silence family during the gap, not
   from duplicates. Once the injector stops self-colliding, arm A exits 0 **because the collision is gone, not
   because the hang is fixed**. Keeping the repair exercised is a P4 decision.
4. **[premise-corrected: the hang is a documented `libduckdb-sys` 1.10502 behaviour recorded in-repo at
   `crates/buffer/src/schema.rs:320–323` — "Runtime PK check via duplicate-INSERT path was observed to hang in
   libduckdb-sys 1.10502 on this build" — not an unexplained one]** — the project has routed around this hang
   in unit tests since `2026-08-22-log-records-identity`. What remains open is narrower: why the *appender*
   path's first violating flush returns cleanly while the next hangs. Two candidate mechanisms, to be
   discriminated by probe at /implement, never by inference: **M1** connection-level residual state (leftover
   transaction / table lock) the next flush waits on — repairable in-code, satisfying the "connection stays
   usable" fork; **M2** an upstream lock the connection cannot clear — leaving only prevention or the
   "terminal and reported" fork.
5. **VERIFIED** — the allowlist key `"duckdb"` (resolved from `duckdb.append` via `split('.').next()`) exists
   at `pulse-app/src/observability.rs:233–244` permitting exactly `rows_appended` / `duration_ms` /
   `table_name` / `reject_reason`, guarded at :3142. No new leaf is owed for the existing fields.
   **Caveat the premise did not state:** any NEW field added to `duckdb.append` is silently redacted unless
   added to that entry and its guard.

## Constraint surfaced at P3 (not in the working entry)
`architecture.md` §Stack requires the injector's arg-less default to emit its storm "byte-identically", and
`webview_drive.rs:728` threads only the PATH — so the headful leg always runs that default. In the injector,
`seq` drives **three** things: identity, the error pattern (`error_roll`), and duration (`jitter`). A fix that
perturbs `seq` would move the storm's error distribution and the headful leg's stage budget; folding a
per-process nonce into **only** `trace_id()`/`span_id()` preserves batch/span/error counts and durations. The
clause's historical verification was a COUNT equality (`3540 = 3540`), not a byte comparison.

## PREREQ (folded from the working entry)

**`cargo audit` re-check** — standing deferral since `2026-08-15-corpus-key-persistence`, ratified at the
`2026-08-16-fault-identity-semantics-decided` wrap, **pin #20**, re-pinned onto this entry with origin
preserved.

- **Basis (re-verified at the predecessor's wrap, unchanged):** `cargo audit` still cannot load the RustSec DB
  — `parse error: duplicate advisory ID: RUSTSEC-2026-0244` (upstream).
- **Named overlap signal:** `cargo deny check advisories` — designed-red, 8 DISTINCT ids
  0189/0190/0194/0195/0204/0222/0253/0258, with `bans licenses sources` exit 0.
- **Interval:** session 52 ran the probe full-form; sessions 53 and 54 are between-points. Session 53 recorded
  `probe skipped per ratified interval (next: 55)`. **This chunk wraps as session 54 — also a between-point**,
  so it owes basis + overlap re-verification and the same recorded skip, not a full probe. **Next interval
  point: 55.**
- Probe records carry **no** running "Nth consecutive" ordinal (ruled 2026-08-28 with the operator).

## Provenance

Working-route entry "Duplicate-span replay fails loudly" (operator-placed FIRST markerless at the
`2026-08-28-ingest-consumer-block-under-gap-resume` wrap). Coordinates re-verified first-hand at HEAD before
shaping this scope, per the verify-at-HEAD discipline: injector `seq` derivation (:301/:224/:231/:277) ✓ ·
`spans` PK (`schema.rs:38`) ✓ · the single `flush()` site (`appender.rs:649`) ✓ · `xtask/src/gap_resume.rs`
three-arm leg ✓ · obs-plan §10 (`:649`) + `rules/observability.md` (`:98`) + test-plan §3 (`:306`) all naming
this entry as owner ✓.
