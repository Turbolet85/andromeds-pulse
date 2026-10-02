# Report — 2026-08-28-ingest-consumer-block-under-gap-resume

**Chunk:** Ingest consumer block under gap→resume — the consumer keeps draining after a producer restart, so accepted spans stop being silently dropped
**Date:** 2026-08-28
**Commits:** none since `last_wrap` (2026-08-28T18:50:00Z) — this wrap's commit is the chunk's first

## Changes (structured — detectors read this)

- **Files:** `xtask/src/gap_resume.rs` · `xtask/src/main.rs` (the only source delta). Plus chunk artifacts
  (`scope.md` / `research.md` / `plan.md` / this report), `.andromeda/master-route.md`,
  `andromeda-pulse-0.3.0/working-route.md`, `.andromeda/obs-plan.md` (+ sidecar),
  `.claude/rules/observability.md` (cascade), run dirs.
- **Symbols / APIs:**
  - NEW `pub enum LegArm { GapResume, Sustained, ReconnectOnly }` + `LegArm::label()` (`xtask/src/gap_resume.rs`).
  - CHANGED `GapResumeOptions.sustained: bool` → `GapResumeOptions.arm: LegArm`. **Callers: `xtask/src/main.rs`
    only** (the single construction site) — xtask-internal, no external consumers.
  - CHANGED `pub fn judge(&Verdict, &StormEvidence, sustained: bool)` → `(…, arm: LegArm)`. **Remaining callers:
    `run_gap_resume` plus 6 in-crate `mod tests` call sites, all updated** — no consumer outside `xtask`.
  - NEW field `StormEvidence.appends: usize` + NEW `pub fn StormEvidence::fed(&self) -> bool`. The existing
    `fired()` is unchanged and still carries the storm precondition.
  - NEW private const `TARGET_DUCKDB_APPEND = "duckdb.append"` (read-only consumer of an existing target;
    emits nothing).
  - NEW CLI flag `--reconnect-only` on `smoke:gap-resume`, `conflicts_with = "sustained"`.
- **Crates / modules:** none added, none removed. Changed: `xtask` only.
- **Dependencies:** none added, none bumped.
- **Schema / config:** no DDL, no corpus table, no config key, no TauRPC procedure, no env var. One CLI flag.
- **Spec-master edits:** `obs-plan.md` §10 (DuckDB Connection Isolation) defect 4 — mechanism corrected to the
  measured chain, defect kept **OPEN** with its new owner named (applied this wrap, per operator directive).
- **Counts / qualifiers moved:** workspace nextest **2042 → 2044** (+2 = the two new pins exactly).
  `smoke:gap-resume` arm count **2 → 3** (`test-plan.md` §3 and `rules/verification-harness.md` §Scenario legs
  both describe it as two-arm).
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** temporary step-probe instrumentation was written into
  `crates/buffer/src/consumer.rs` and `crates/buffer/src/appender.rs` (an env-gated `eprintln!` marker per
  dispatch step), used to localize the hang, then **fully reverted** — both files are byte-identical to HEAD and
  the tree carries **zero probe residue** (verified by grep). Nothing from that instrumentation ships.
- **Spec claims disproved by measurement:**
  1. *"`dispatch_batch` takes `conn.lock()` FIRST and holds it across every append, so BOTH shared-connection
     users die — the consumer and the retention sweep."* Stated in the working-route entry, `obs-plan.md` §10
     defect 4, and `.claude/rules/observability.md`. **FALSE at HEAD:** `run_retention` has held a dedicated
     `try_clone()`d connection since chunk #99 (`crates/buffer/src/retention.rs:72-86`); it touches the shared
     `conn` exactly once, to make that clone. The shared appender connection has exactly ONE production user.
     → **Amended this wrap** (obs-plan §10 + cascade).
  2. *"the reproducing arm changes two variables at once (180 s idle AND a new gRPC connection), and nothing yet
     separates them."* **The idle is not the operative variable:** `--reconnect-only` (seed → immediate
     reconnect, ~2 s) reproduces the wedge identically — same `buffer.consumer.stalled` at 450 s,
     `reason=rows_static_while_channel_queued`, same 3,240 rows, same 660 span-events as the 180 s-gap arm.
     → **Amended this wrap.**
  3. The chunk plan's three enumerated candidate causes are **all excluded by direct probe**, not inference:
     (a) the inline baseline tap — its `tapped` marker printed for the hung batch; (b) blocking-pool starvation
     — `spawn_blocking:running` printed for the hung batch, and the archived wedge shows 771 `viz.query.traces`
     + 1,300 `metric.pipeline.l1a.*` **completing after onset**; (c) an engine-level block of the whole append —
     wrong shape: `conn.appender()` opened and `append_record_batch` returned; only `flush()` never does.
     → Recorded here; the plan is a closed chunk artifact and is not amended (per the 2026-08-26 ruling).
- **Coverage of new surfaces:**
  - `cargo xtask smoke:gap-resume --reconnect-only` (harness CLI arm) → validation `n/a` (clap-parsed bool,
    `conflicts_with` enforced) · instrumentation `n/a` (harness-side; adds no product obs target, field, or
    allowlist leaf) · PII `n/a` (reads aggregate log targets only) · tests `unit` (5 pins: the arm's three-way
    precondition test, the feed/storm independence test, and 3 updated multi-arm pins) · a11y `n/a` ·
    tokens `n/a`.

## Deviations from intent

- **Steps 3–5 of the plan (repair · pin · green arm A) were NOT delivered — soft-exit trigger 3 (out-of-scope).**
  Justification: Step 2's attribution matched none of the plan's three candidates, and the true cause lives in
  two files absent from the plan's Files-to-modify list — `crates/ingest/examples/inject_demo.rs` (the injector
  re-emits identical primary keys on every restart) and `crates/buffer/src/appender.rs` (the flush AFTER a
  constraint-violating flush hangs). The plan's own `## Constraints & rejected approaches` forbids
  pre-committing a repair shape before Step 2 answers; with the answer outside the modify-set, the disciplined
  outcome is to surface. Operator ruling at this wrap: **wrap as-is, the repair is the next chunk.**
- **One gray-area scope call, taken and reverted.** Step 2's pin-down needed step markers inside
  `append_record_batch_to_table` (`crates/buffer/src/appender.rs`), not on the modify list. Judged in-scope for a
  TEMPORARY diagnostic — it is the helper the in-scope consumer path calls directly, and measurement had already
  localized the hang to exactly that function. Reverted once the attribution landed.
- **Arm A remains red.** Per plan Step 5 it should have gone green; it cannot until the repair lands. This is
  the documented scenario-not-gate treatment (`test-plan.md` §3 registers arm A red-by-design with an owner) —
  recorded, never weakened.

## The attribution (the chunk's substantive deliverable)

Measured chain, end to end:

1. `inject_demo` derives `trace_id` / `span_id` as pure functions of `seq = b * 100_000 + …`, and the batch
   counter `b` restarts at 0 in **every** process (`crates/ingest/examples/inject_demo.rs:277,301,224,231`).
   So injector process 2 re-emits process 1's **exact primary keys**.
2. `spans` carries the composite PK `(trace_id, span_id)`, enforced by the Arrow `Appender` at `flush()`.
3. Process 2's first batch → duplicate PKs → `flush()` returns `Err` → the whole batch is rejected and ONE
   `duckdb.append` ERROR (`reject_reason: "append_failed"`) is logged. Correct and loud.
4. Process 2's **next** batch → `flush()` **hangs, unbounded**, while holding the shared connection mutex.
5. The consumer loop never returns to `recv()` → the ingest channel fills 0 → 100 % → `ingest.channel.full` →
   `buffer.consumer.stalled` at 450 s → 24k+ accepted spans unpersisted, at **0 ERROR** after step 3.

This explains every prior observation, including why `--sustained` is healthy (one process, monotonically
increasing ids, no collision) and why readers were unaffected in the archived gap arm (MVCC readers do not
queue behind a stalled writer).

**Production reachability:** a retrying OTLP exporter resending spans is normal client behaviour that reaches
this hang.

## Decisions & corrections

- **Operator wrap directive (this session):** wrap as-is; the repair becomes the next chunk, properly scoped —
  no in-place re-plan. Master desc rewrites to measured actuals at the flip.
- **Operator ruling:** the repair entry is minted FIRST markerless, with modify-set = injector + appender,
  acceptance = arm A exits 0 with its preconditions satisfied, and the observe-window guard folded in.
- **Operator ruling on probe records:** drop the running "Nth consecutive" ordinal — it has drifted twice;
  *"identical to the prior enumeration"* carries the same information without a carried count.
- **Self-caught measurement error.** Shortening a diagnostic leg to `--observe-minutes=3` produced a **vacuous
  PASS**: the stall threshold is 450 s (30 ticks), so a 3-minute observe window makes
  `ingest_progress::evaluate` structurally incapable of failing. Caught by the append count (232, against 233 in
  the failing 9-minute run) — the consumer had not kept draining. The leg has no guard against an observe window
  shorter than the threshold it reads; folded into the repair entry.
- **Session hygiene:** a background wait-loop of mine polled for a `Summary [` line that `tail -8` had already
  discarded, so it spun long after its gate chain finished. Same masking failure the 2026-06-05 learning
  records for exit codes, reintroduced in the *predicate* rather than the command.

## Outcome

**Acceptance criteria: partially met.** Delivered: the reconnect-only arm with its own precondition (criterion
2), the full gate set (criteria 10–12), plus the attribution the plan's Step 2 required. Not delivered: arm A
green, the in-crate `crates/buffer` pin, and the obs/security criteria that only apply to a repair — all
downstream of the blocked Step 3.

**Gates — all green:**
`cargo fmt --check` ✓ · `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓ ·
`cargo build --workspace --tests --jobs 4` ✓ · `cargo nextest run --workspace --profile ci` **2044/2044 + 1
skip** ✓ · `cargo xtask capability-widening-check` clean (0 violations / 3 inspected) ✓ ·
`cargo xtask check:ingest-progress` PASS (128 ticks, longest zero-delta run 0) ✓ ·
`cargo deny check bans licenses sources` ok ✓ · `cargo xtask capability-drift` **clean, run LAST** after the
documented bindings regen (`bindings/index.ts` byte-identical to HEAD) ✓ · `bash scripts/agent-run.sh status`
returned its contract payload ✓. **0 fix-loop iterations.**

`cargo deny check advisories` observed separately (designed-red): the owned set is **identical to the prior
enumeration** — 0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258, eight DISTINCT ids, with
`bans licenses sources` exit 0.

**`smoke:gap-resume` arm A: RED, by design.** Not a gate failure — `test-plan.md` §3 and
`rules/verification-harness.md` §Scenario legs register the leg as a SCENARIO outside the gate set, arm A
red-by-design against the open defect, with a named owner. Recorded, never weakened.

**Smoke: four real boots** of `target/release/pulse-app.exe`, each on a fresh `ANDROMEDA_PULSE_DATA_DIR` under
`target/gap-resume/`, seeding over the real OTLP receiver via the prebuilt `inject_demo`, each shutting down by
pid with `:4317`/`:4318` confirmed released — proven by each subsequent leg binding the ports. 0 orphans. The
release binary was rebuilt probe-free after the investigation so no later leg measures instrumented code.
