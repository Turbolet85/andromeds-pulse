# Scope — 2026-08-26-cadence-runaway-blocking-pool

**Working-route entry (verbatim intent):** Cadence runaway starves the blocking pool — the cue→cadence→digest
loop stays bounded under sustained load, and the buffer keeps draining.

**Epoch:** Epoch 4 — Polish & ship: verification · **Version:** andromeda-pulse-0.3.0

---

## Outcome

**Amended at P5 validation-1 (intent-incomplete; operator-approved at the P4 review).** This section was
authored at promotion from the working entry's causal direction, which P3 then measured false — the cue storm
FOLLOWS the freeze by 32 seconds. The premise-closure section below corrected the mechanism but left this
derived section stating an outcome the chunk cannot deliver. Corrected here so P4/implement consume one
consistent scope.

Three things must be true when this chunk is done:

1. **The loop stays bounded.** Under a sustained OTLP feed the cue→cadence→digest→L1a path cannot spawn work
   at a rate that starves the tokio blocking pool. A bound exists, is enforced, and is stated.
2. **A wedge stops being unrecoverable.** Today a frozen consumer starves the baseline, which produces ~300
   silence cues/min, which fan into cadence cycles that deepen the starvation — so the system can never climb
   back out. Breaking that cycle is this chunk's substantive repair.
3. **The next occurrence is diagnosable without forensics.** `triage.cue.tick`'s `cues_suppressed` /
   `bypass_triggered` are `"<redacted>"` at HEAD, so the question "did suppression engage?" is unanswerable
   from any log. Un-muting that one target, plus recording cadence cycle rate, replaces a 131 MB forensic
   read with a readable signal.

**NOT an outcome of this chunk, stated plainly:** preventing the initiating freeze. What stopped the consumer
at 17:20:50 under ~1.7 blocking queries/second is unidentified, and the predecessor's matched 15-minute leg
did not reproduce it. That gets its own route entry at wrap, instrumented by outcome 3 so the next occurrence
can be attributed. The predecessor delivered "the buffer keeps draining" by MEASUREMENT; this chunk does not
convert it to repair, and no acceptance criterion claims otherwise.

## The measurement this chunk answers to

Attributed at `2026-08-26-ingest-consumer-stall-under-sustained-load` from the predecessor's own wedge log
(still on disk at `…/fdc6dabd-…/scratchpad/l4run-171923/logs/agent-latest.jsonl.2026-08-25`, 131 MB,
re-verified present at promotion). Comparable windows, wedged run vs a healthy 15-minute control:

| signal | wedged | healthy |
|---|---|---|
| `metric.pipeline.l1a.query_count_total` | **26,821** | 220 |
| `cadence.trigger` | 2,683 | 22 |
| `cadence.tick` | 51 | — |
| `digest.assemble.request` | 2,682 | 22 |
| `digest.lww.drop` | 2,646 | 1 |
| `duckdb.append` | 132 → **stopped 17:20:50** | 2,599 → running |
| `viz.query.traces` | 270 → **stopped 17:24:13** | 780 → running |

**The mechanism is settled** (overseer-verified first-hand): both `spawn_blocking` users died while every
async task ran 16 minutes longer, and they use DIFFERENT mutexes — `viz` the appender connection, L1a a
separate `try_clone`d one — so no single lock explains both. The shared resource is the tokio blocking pool.
Max append duration was 11 ms in BOTH runs, so DuckDB contention is excluded.

**What is NOT settled, and is this chunk's first job:** why that run's loop fired 122× more than a matched
control did.

## Deliverable A — answer the TRIGGER question (operator-directed)

Per the operator's 2026-08-26 ruling, this chunk's research owns the **conditions**, not merely the mechanism:
what made THAT run's loop spawn 122× while a 15-minute leg reproducing every stated precondition stayed at
baseline. The answer must name conditions that can be stated, tested, and bounded — not a plausible story.

Facts already in hand that the research starts from (measured, not assumed):
- The L1a flood is *exactly proportional* to trigger count — 26,821 ÷ 2,683 ≈ **10.0** queries per trigger.
  `run_cadence_cycle` emits `cadence.trigger` then drives Q1–Q7 through `drive_query!`, each a
  `spawn_blocking`. So nothing is wrong with the per-cycle query count; the runaway is entirely in HOW OFTEN
  a cycle runs.
- `cadence.tick` fired 51 times against 2,683 triggers. Triggers therefore did **not** come from the ticker.
- Candidate conditions observed alongside, none yet attributed: L4 at ~4.3 s/inference, 67
  `interpretation.inference.error`, `interpretation.degraded.enter` fired once, 2,646
  `digest.lww.drop`.

## Deliverable B — bound the loop

Whatever the trigger answer, the loop needs a stated ceiling that holds under sustained load. Two properties:

- **The bound must be enforced where the triggers originate**, not by widening what absorbs them.
- **It must not silence legitimate acceleration.** The cadence design has three modes with configured
  intervals; a bound that flattens accelerated mode into baseline would trade this defect for a slower
  incident path.

## Boundaries — out of scope

- **Raising `max_blocking_threads` is REJECTED as the fix**, carried forward from the predecessor's plan: it
  enlarges the queue in front of a blocked holder without unblocking it, postponing the wedge and making it
  harder to observe. Setting it *explicitly as a documented bound* alongside a real ceiling is a separate
  question this chunk may consider, but never as the remedy on its own.
- **Not the drain observable** — `buffer.consumer.stalled`, the `buffer.tick` drain-progress pair and
  `cargo xtask check:ingest-progress` shipped last chunk and stay as they are. This chunk should make that
  gate's RED arm unreachable in practice, not modify it.
- **Not the L4 interpretation path's own defects** — the ~4.3 s inference and the brief's content problems
  belong to `Interpretation brief completeness`.
- **Not the muted-diagnostics sweep** — if a target this chunk needs is muted, un-mute exactly that one and
  say so.
- **No change to the ingest→buffer substrate** — bounded `tokio::sync::mpsc`, `try_send` →
  `ResourceExhausted` stays (arch §Established Decisions [In-Process Channel Architecture]).

## Surfaces and contracts in frame

| Surface | Path | Role |
|---|---|---|
| cadence coordinator | `crates/triage/src/cadence/coordinator.rs` (`run_cadence_cycle` ~:160-200, `drive_query!`) | emits `cadence.trigger`, drives Q1–Q7 |
| cadence config + floors | `crates/triage/src/cadence/config.rs:5-25` | baseline 60s (floor 5) · accelerated 20s (floor 1) · reflection 1800s (floor 300) |
| cue-driven trigger carrier | `crates/triage/src/cue::CadenceTriggerChannel` (`coordinator.rs:23,278`) | the non-ticker path into a cycle |
| obs targets | `crates/triage/src/cadence/mod.rs:42-43` (`TARGET_CADENCE_TICK` / `TARGET_CADENCE_TRIGGER`) | the two counters whose 51-vs-2,683 split localises the source |
| digest subscriber | `pulse-app/src/digest_runtime.rs:129` (`spawn_cadence_subscriber`) | consumes triggers → `digest.assemble.request` |
| L1a query surface | `crates/triage/src/baseline/sql.rs` (Q1–Q7, each `spawn_blocking`) | the work the flood multiplies |
| tokio runtime | `pulse-app/src/main.rs:276-279` | `new_multi_thread().enable_all().build()` — `max_blocking_threads` verified **NOT set anywhere in the workspace**, so the cap is tokio's default 512 |

## Folded annotation — PREREQ (from the working entry)

**`cargo audit` standing deferral.** Origin `2026-08-15-corpus-key-persistence`; ratified 2026-08-16 (pin
#15) with an every-3rd-wrap re-run INTERVAL; re-pinned onto this entry at the 2026-08-26
ingest-consumer-stall wrap, origin preserved.

**This chunk's wrap IS interval point 46** — `session_count` reached 45 at that wrap, so the next wrap is the
point. The obligation here is therefore **NOT** a skip-with-basis-recheck: the probe must actually RUN.
Discharge in full form — run `cargo audit`, read its true exit status DIRECTLY (never through a pipe; a
pipeline reports the last command's status and has masked a true exit 1 three times, once this session at a
light gate), record whether the RustSec DB still fails to load, and re-enumerate the owned set as DISTINCT
`RUSTSEC-` ids, never error blocks (a crate at two lockfile versions raises one block per version). The set
read **8** at the last three probes: 0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258. It has GROWN
before (7 → 8), so re-enumerate rather than carrying the count forward. Full rationale: the
`2026-08-15-corpus-key-persistence` report.

No `CARRY:` and no `BLOCKED-ON:` annotation is present on this entry (checked at an annotation position).

## Coordinate verification (re-derived at HEAD before shaping scope)

Every coordinate the entry names was re-derived first-hand at promotion. **All verified; no corrections.**

- ✔ `crates/triage/src/cadence/` — 4 files: `broadcast.rs` · `config.rs` · `coordinator.rs` · `mod.rs`
- ✔ `pulse-app/src/main.rs:276-279` — the runtime builder, verbatim
- ✔ `max_blocking_threads` — **zero occurrences workspace-wide**, so the default-512 claim holds
- ✔ `CadenceTriggerChannel` — `coordinator.rs:23` (import), `:278` (field)
- ✔ `TARGET_CADENCE_TICK` / `TARGET_CADENCE_TRIGGER` — `cadence/mod.rs:42-43`
- ✔ `cadence.trigger` emit site — `coordinator.rs:174-181`
- ✔ `spawn_cadence_subscriber` — `digest_runtime.rs:129`
- ✔ the evidence log — present, 131,324,444 bytes

## Premise closure (P3 — resolved against research)

**The chunk's CENTRAL premise is corrected, not merely refined.** This section's heading above ("Cadence
runaway starves the blocking pool") states a causal direction the wedge log contradicts: the cue storm begins
at 17:21:22, **32 seconds AFTER** the consumer's last append at 17:20:50.787, and in the entire 66-second
pre-freeze window the load was 11 cadence cycles / 110 L1a queries / 9 cues — roughly 1.7 blocking queries
per second against a 512-thread pool. **The runaway is an amplifier and a recovery-blocker, not the
initiating cause.** The initiating cause of the freeze is unidentified and is now an open question (see
`research.md` §Open questions). Deliverable A's TRIGGER question is answered — and its answer reshapes the
chunk, which is a P4 decision.

Per-premise:

1. **The runaway enters through the CUE-DRIVEN path, not the ticker** — **VERIFIED and sharpened.** The
   variable is cue VOLUME: `emit_cue` (`crates/triage/src/cue/emitter.rs:189-192`) fans every **Suggested**
   cue 1:1 into a cadence trigger, unconditionally, with no rate limit.
2. **The existing safety floors do not bound this path** — **VERIFIED.** `cadence/config.rs:5-25` bounds
   configured ticker INTERVALS; nothing bounds the cue-driven entry.
3. ~~A feedback loop is plausible: L4 lag → digests queue → cues keep firing.~~ **[premise-corrected: a
   feedback loop IS the dominant effect, but it runs the OPPOSITE way. `observe_spans_for_baseline` executes
   INSIDE `run_consumer` (`crates/buffer/src/consumer.rs:47-56`), so a wedged consumer starves `BaselineState`
   too; ~32 s later every tracked service crosses its silence threshold, and the emitter produces one
   `service_went_silent` cue per service per tick — 5 × 60 = the observed flat 300/min. 4,590 of the run's
   4,599 cues are that single kind; `error_rate_spike` accounts for 9.]**
4. **Each cycle's 10 blocking queries are not excessive** — **VERIFIED.** 26,821 ÷ 2,683 = 9.996 across the
   run, and 110 ÷ 11 = 10.0 in the pre-freeze window.
5. ~~No product code outside `crates/triage/src/cadence/` needs to change.~~ **[premise-corrected: the
   amplifier originates in `crates/triage/src/cue/emitter.rs`; `cadence/` may need no change at all. The
   modify set moves from `cadence/` to `cue/`.]**

**A new gap the closure surfaced:** `cadence.trigger` = 2,683 but only 606 cues were `suggested` and
`cadence.tick` = 51 — leaving ~2,000 cycles unaccounted for. `start_cadence_coordinator` has THREE entry
points into a cycle (`coordinator.rs:277` / `:298` / `:449`); the third driver is unidentified, so a bound
placed only on the cue fan-out would miss most triggers.

**A blocked diagnostic:** `triage.cue.tick` emits `"cues_suppressed":"<redacted>"` and
`"bypass_triggered":"<redacted>"` at HEAD — confirmed live in this session's own healthy leg. That is
obs-plan §8's muted-diagnostic backlog, and it is exactly the field that would say whether suppression
engaged and was overwhelmed or never engaged. The scope's boundary already permits un-muting precisely the
targets this chunk needs.

## Provenance

- Working-route entry (Epoch 4) — the intent anchor, authored at the predecessor's wrap.
- `andromeda-pulse-0.3.0/chunks/2026-08-26-ingest-consumer-stall-under-sustained-load/report.md` — the
  attribution and the rejected-fix note.
- `.andromeda/obs-plan.md` §10 — the blocking-pool-adjacent load constraints and the isolation topology.
- Operator ruling 2026-08-26 — the research owns the TRIGGER question (conditions, not just mechanism).
