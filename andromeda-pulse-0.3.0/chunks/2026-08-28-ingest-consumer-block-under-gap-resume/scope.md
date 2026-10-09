# Scope — Ingest consumer block under gap→resume

**Marker:** `2026-08-28-ingest-consumer-block-under-gap-resume`
**Version:** andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification
**Working entry:** first markerless, operator-placed FIRST at the `2026-08-28-ingest-consumer-initiating-freeze` wrap.

## 1. Surface

The consumer keeps draining after a producer restart, so accepted spans stop being silently dropped.

## 2. Outcomes

1. **The trigger is separated.** Today's reproducing arm changes two variables at once — 180 s of idle
   AND a new gRPC connection (the injector connects once per process). One of them is the trigger, or
   neither alone is; this chunk decides which before any repair is designed.
2. **WHY the first post-resume batch blocks is attributed** — a named mechanism, measured, not a
   plausible story. The predecessor left this explicitly unestablished.
3. **The consumer drains after a producer restart** — `rows_ingested` advances again, the ingest channel
   returns toward empty, and accepted spans reach DuckDB.
4. **`cargo xtask smoke:gap-resume` arm A turns GREEN by product repair** — the leg is the acceptance
   test. Arm A is red today BY DESIGN against this defect; greening it by weakening the arm is banned
   (§5).

## 3. Starting position (measured at the predecessor, re-verified at HEAD)

- `cargo xtask smoke:gap-resume` exists and reproduces the wedge on demand — `xtask/src/gap_resume.rs`
  (`run_gap_resume` `:189`, arm selection `:246`, seed/gap/resume narration `:260`–`:270`); registered
  in `xtask/src/main.rs`. Arm B (`--sustained`) is the gate-grade control and PASSED at 29,160 rows.
- Arm A measured: `rows_ingested` frozen at **3,240**, ingest channel **0 → 85.7 %**, `ingest.channel.full`
  fired, 24k+ accepted spans unpersisted, `buffer.consumer.stalled` announced at 450 s, blocked 9+ minutes
  — all at **0 ERROR and 0 panics**. Nothing returns, so nothing reports.
- The consumer drains the pre-gap seed **completely** (last `duckdb.append` at its end) and sits correctly
  idle through the gap — retention swept **successfully twice** during the gap, so nothing held a lock
  there. The block begins on the **first post-resume batch**.
- **The predecessors' bound is measured NOT to be the fix.** Arm A ran with the runaway fully bounded
  (`cadence.trigger` 65 vs the wedge's 2,683 · L1a 650 vs 26,821 · `triage.cue.emit` 62 vs 4,599) and the
  consumer stalled anyway.
- **Excluded by measurement:** the characterized-not-fixed DuckDB maintenance pause — that needs a
  ~12.7M-row table and is bounded >60 s; this table held 3,240 rows and blocked 9+ minutes.
- `crates/buffer` was deliberately OUT of the predecessor's touchpoints (its plan states *"No production
  `.rs` outside `crates/triage`'s test module changes"*), so **this entry owns the repair whole**.

### Code shape at HEAD (verified first-hand this phase)

- `crates/buffer/src/consumer.rs::run_consumer` is a strictly serial loop: `receiver.recv().await` →
  (spans only) `observe_spans_for_baseline(...)` **inline on the async task** (`:53`) → `spawn_blocking(dispatch_batch)`
  → `.await` (`:62`–`:70`). One batch at a time; if the blocking call never returns, the loop never
  `recv()`s again and the channel fills. That is the observed shape.
- `dispatch_batch` (`:96`) takes `conn.lock()` **FIRST** (`:104`) and holds the guard across every append,
  dropping it only at `:170`.
- `observe_spans_for_baseline` (`:234`) is the only synchronous non-`spawn_blocking` work in the loop.

### The two candidates research left open

- (a) the inline per-span `observe_spans_for_baseline` on the consumer's async task;
- (b) blocking-pool starvation (the `spawn_blocking` call never gets a thread).

**`[premise-corrected: retention has held a DEDICATED `try_clone()`d connection since chunk #99 —
`crates/buffer/src/retention.rs:72-86`; it touches the shared `conn` exactly once, to make the clone, and
sweeps thereafter on `sweep_conn` (`:94` → `:103` → `:165`). The ONLY production user of the shared appender
connection's Rust mutex is `dispatch_batch`.]`** The working entry, obs-plan §10 defect 4 and
`rules/observability.md` all state the mechanism as *"`dispatch_batch` holds `conn.lock()` across every
append, so BOTH shared-connection users die — the consumer and the retention sweep."* The second half names a
user that no longer exists: a held `conn` mutex cannot block `sweep_conn.lock()`. Retention's death therefore
needs its own explanation, and the surviving candidates are (b) blocking-pool starvation, or (c) an
ENGINE-level DuckDB block — `dispatch_batch` stuck inside `conn.appender()` / `append_record_batch` / `flush()`
holding a DuckDB-internal lock that retention's `DELETE` then queues behind on its cloned connection. Chunk
#99 already recorded engine-level interaction of exactly this shape ("L1a reads queued behind append-path
row-group maintenance"), so (c) is the leading candidate, not (a) or (b). See `research.md` for the full
topology measurement; the correction is owed to the three artifacts above as an amendment at wrap.

## 4. The mandated first arm (ordering is part of the scope)

The working entry fixes the order, and it is not negotiable by convenience:

1. **A reconnect-WITHOUT-gap run** — separate the 180 s idle from the new gRPC connection. This decides
   the trigger *before* any repair is designed.
2. **Attribute** why the first post-resume batch blocks (the two candidates above are a starting field,
   not a conclusion).
3. **Repair.**
4. **Turn arm A green** — by fixing the product.

A repair designed before step 1 is out of order even if it works: the predecessor's own lesson is that a
repro is only as good as the premise it encodes.

## 5. Boundaries

**IN scope**
- `crates/buffer` — the consumer loop, `dispatch_batch`'s lock discipline, the shared-appender-connection
  topology, and the baseline tap's placement on the async task.
- A new arm / mode on `cargo xtask smoke:gap-resume` for the reconnect-without-gap separation. **VERIFIED
  necessary:** the CLI already carries `--gap-seconds`, so the *drive* shape is expressible, but
  `gap_resume::judge` (`xtask/src/gap_resume.rs:134`) has exactly TWO verdict arms — `sustained` (a storm is a
  FAILURE of the control) and gap (no storm ⇒ INCONCLUSIVE). A reconnect-without-gap run produces no storm by
  construction and is not `sustained`, so it lands in the gap arm and is judged INCONCLUSIVE regardless of
  what the consumer does. The arm needs its own mode plus its own stated precondition.
- The repair, with a discriminating pin (RED without it).
- Recording what was ruled OUT, so a recurrence starts from a narrower field.

**OUT of scope**
- **Weakening `smoke:gap-resume` arm A in any way** — it is the acceptance test; it greens by product
  repair or not at all. Loosening a threshold, widening a tolerance, or `#[ignore]` is a scope violation,
  not a judgment call.
- **Re-litigating the `CueLatch` amplifier bound or the idle-generation damper** — both landed, both are
  pinned, and both are *measured* not to be this defect's fix. This chunk consumes them unchanged.
- The ingest/OTLP receiving path — the receivers kept ACCEPTING throughout; the defect is on the consumer
  side of the channel.
- The characterized-not-fixed DuckDB maintenance pause (excluded by measurement above).
- `App-registry reconciliation with externally-resolved rows` — the next markerless entry, with its own owner.

## 6. Surfaces / contracts this chunk may touch

- **Observability** — any new field or target follows the exact-leaf discipline; a bare `buffer` key
  resolves to a POPULATED sibling set and silently redacts every field, so a new target needs its OWN
  exact allowlist leaf plus a field-set pin. `buffer.consumer.stalled`, `rows_ingested_delta`,
  `last_append_age_seconds` and the 450 s / 30-tick threshold keep their shipped semantics unless the
  repair provably requires otherwise (obs-plan §10).
- **The DuckDB connection-isolation invariant** (`rules/observability.md`; obs-plan §10) — the reference
  topology is write (appender) / sweep (retention) / read (L1a + viz) on separate connections. Defect 4 is
  recorded as the invariant being *correct but not satisfied by shipped code*.
- **No new workspace member, no new inter-crate dependency edge, no DDL, no TauRPC procedure** unless the
  attribution forces one — and then it is surfaced, not assumed. **VERIFIED achievable:** every touchpoint
  the measurement implicates sits in `crates/buffer` or `xtask`, both already workspace members with the
  edges they need.
- **`run_consumer`'s signature is a threading surface.** The graph shows 9 out-of-crate call sites — the
  production spawn at `pulse-app/src/main.rs:1154` plus 7 `pulse-app/tests/` integration/perf harnesses
  (`e2e_drain_template_assignment` · `e2e_p1_otlp_grpc_to_traces_query` · `e2e_p6_channel_arrow_ipc` ·
  `e2e_pii_bare_credential_stored_field` · `e2e_storm_detection` · `perf_load_profiles` ·
  `perf_slo_10k_spans`) — and 8 in-crate `consumer::tests` callers. Any parameter added to it threads all 17.
- **Test harness** — `test-plan.md` §3 registers `smoke:gap-resume` as a SCENARIO explicitly outside the
  gate set; if arm A turns green, whether it becomes gate-grade is a decision to surface, not to take
  silently. Unchanged by research — still a decision, now with the cost known (~13 min, dev-host only).

## 7. Folded annotations

- **PREREQ (from the working entry, origin preserved):** re-check `cargo audit`. Standing deferral since
  `2026-08-15-corpus-key-persistence`, ratified at the `2026-08-16-fault-identity-semantics-decided` wrap,
  **pin #19**, re-pinned onto this entry from `2026-08-28-ingest-consumer-initiating-freeze`. Basis:
  upstream RustSec DB duplicate-id parse error (`RUSTSEC-2026-0244`) — external decay. Overlap signal:
  `cargo deny check advisories`, designed-red at the same **eight owned DISTINCT ids**
  0189/0190/0194/0195/0204/0222/0253/0258 with `bans licenses sources` exit 0.
  **Interval status:** session 52 RAN the probe FULL-FORM (sixth consecutive identical result); **next
  interval point is 55**. This chunk's wrap is session 53 — **between-points**: re-verify basis + overlap
  and record `probe skipped per ratified interval (next: 55)`, never a silent skip.
- No `CARRY:` and no `BLOCKED-ON:` annotation on this entry (checked at annotation position, not in prose).

## 8. Owner obligations on landing

Two masters name **this entry** as owner of the open defect, and both record the isolation invariant as
correct-but-unsatisfied until it lands:

- `.andromeda/obs-plan.md` §10 — defect **4**, "Consumer block under gap→resume — OPEN, not fixed."
- `.claude/rules/observability.md` — the DuckDB connection-isolation rule's fourth-defect paragraph.

When the repair lands, both need amending from OPEN to closed-with-evidence. That is wrap's drift job, but
it is this chunk's obligation to produce the evidence they will cite.
