# Codebase Research — 2026-08-26-cadence-runaway-blocking-pool

## Scope
- **Depth:** deep · **Reads:** 9 · **Greps/log-analyses:** 12 · **Graph queries:** 1 (rust plane, `db_state: fresh`, 83 rows)

## Headline: the chunk's central premise is INVERTED by measurement

The working entry and `scope.md` both assert that a cadence runaway **starves** the blocking pool and thereby
wedges the consumer. The wedge log says the opposite ordering:

| event | timestamp |
|---|---|
| first `error_rate_spike` cue · first `cadence.trigger` | 17:20:16 |
| **last `duckdb.append` — the consumer freezes** | **17:20:50.787** |
| **first `service_went_silent` cue** | **17:21:22** (+32 s) |
| cue emissions reach a flat 300/min | 17:22 onward |
| `viz.query.traces` stops | 17:24:13 |

**The cue storm begins 32 seconds AFTER the freeze, not before it.** In the whole 66-second window from boot
to freeze the load was modest: 11 `cadence.trigger`, 110 `metric.pipeline.l1a.query_count_total`, 10
`interpretation.inference.request`, 9 `triage.cue.emit`. That is ~1.7 L1a queries/second against a 512-thread
default pool — not a flood.

So the 122× runaway is an **amplifier and a recovery-blocker, not the initiating cause**. The initiating
cause of the 17:20:50 freeze remains unidentified.

## Why the storm follows the wedge (the feedback loop, traced)

1. `observe_spans_for_baseline` runs **inside `run_consumer`**, on the async task, before the
   `spawn_blocking` dispatch (`crates/buffer/src/consumer.rs:47-56`). A wedged consumer therefore stops
   feeding `BaselineState` as well as DuckDB.
2. With no span observations, every tracked service crosses its silence threshold ~32 s later.
3. `run_one_emit_cycle` then emits one `service_went_silent` cue **per service per tick** — 5 services × 60
   ticks/min = the observed flat **300/min**, and **4,590 of the run's 4,599 cues** are that one kind
   (`error_rate_spike` accounts for just 9).
4. `emit_cue` fans every **Suggested** cue 1:1 into a cadence trigger, unconditionally and with no rate
   limit (`crates/triage/src/cue/emitter.rs:189-192`):
   ```rust
   if matches!(cue.priority_tier, PriorityTier::Suggested) {
       let _ = cadence_handle.sender().send(cue.clone());
       *cadence_emitted += 1;
   }
   ```
5. Each trigger runs a cadence cycle that drives Q1–Q7 through `drive_query!`, each a `spawn_blocking`
   (`crates/triage/src/cadence/coordinator.rs:174-200`). Measured ratio: **26,821 ÷ 2,683 = 9.996** L1a
   queries per trigger, and the pre-freeze window confirms it exactly at 110 ÷ 11 = 10.0.
6. That load then deepens the starvation — `viz.query.traces`, the other `spawn_blocking` user, dies 3.5
   minutes after the consumer.

**Unresolved inside this loop:** `cadence.trigger` = 2,683 but only **606** cues were `suggested`, and
`cadence.tick` = 51. 606 + 51 = 657, not 2,683. `start_cadence_coordinator` has THREE call sites into a
cycle (`coordinator.rs:277`, `:298`, `:449`); a third driver accounts for ~2,000 cycles and is not yet
identified.

## Graph impact (rust plane, `rows: 83`)
- **`CadenceTriggerChannel`** — production consumers: `cue/emitter.rs:33` (`run_one_emit_cycle`), `:178`+`:189`
  (`emit_cue`), `:256` (`start_emitter`); `cadence/coordinator.rs:277`, `:298`, `:449`
  (`start_cadence_coordinator`); the channel itself at `cue/broadcast.rs:62-83`; boot wiring at
  `pulse-app/src/main.rs:471`. Exported through `triage::contract` (`contract.rs:40`).
- The fan-out semantics are pinned by two existing in-crate tests whose names state the contract:
  `run_one_emit_cycle_tier_two_fans_to_cadence_triggers` and
  `run_one_emit_cycle_autonomous_tier_does_not_fan_to_cadence_triggers` (`emitter.rs:518`, `:543`).
- `run_one_emit_cycle_emits_suppression_check_per_cue` (`emitter.rs:850`) confirms a suppression mechanism
  exists on the emit path.

## Patterns detected
- **Emitter tick rate is NOT the variable.** `triage.cue.tick` 619 (healthy, 10 min) vs 1,045 (wedged,
  ~18 min) — comparable per minute. Cues *per tick* is what exploded: 0.015 → 4.4.
- **Silence cues do not appear to latch.** The flat 300/min for ~16 minutes is one cue per service per tick,
  sustained — consistent with re-evaluation each tick and no per-service latch once a service is already
  known-silent. Whether a latch exists and was simply overwhelmed cannot be read from the log (see next).
- **The field that would answer that is REDACTED at HEAD.** `triage.cue.tick` emits
  `"cues_suppressed":"<redacted>"` and `"bypass_triggered":"<redacted>"` while `cues_evaluated`,
  `cues_emitted` and `cadence_triggers_emitted` are visible — confirmed live in this session's own healthy
  leg. This is obs-plan §8's muted-diagnostic backlog, still live, and it blocks Deliverable A's suppression
  question directly.
- **Existing safety floors govern the ticker only.** `cadence/config.rs:5-25` bounds configured INTERVALS
  (baseline 60 s / floor 5, accelerated 20 s / floor 1, reflection 1800 s / floor 300) with the stated
  rationale that "below this floor the cadence ticker would saturate the runtime". Nothing bounds the
  cue-driven path, which is where 2,632 of 2,683 triggers entered.

## Conventions to follow
- `triage` clock convention: pass the instant as an explicit `now_nanos` parameter (`run_one_emit_cycle`
  already does) rather than `tokio::time::pause()`.
- In-crate `#[cfg(test)] mod tests` for `triage`; `pulse-app` probes go to `pulse-app/tests/*.rs` under
  `[lib] test = false`.
- Bounded-cardinality tracing on the cue path: `emit_cue`'s doc comment states `scope_id` is never logged
  because it may carry a user-controlled `service.name`.

## Files to modify (provisional — shape depends on the P4 decision)
- `crates/triage/src/cue/emitter.rs` — the 1:1 Suggested→trigger fan-out at `:189-192`; the per-tick
  re-emission of already-known-silent services. **This is the amplifier's origin, and it is in `cue/`, not
  `cadence/`.**
- `crates/triage/src/cadence/coordinator.rs` — only if the ceiling is enforced at consumption
  (`:277`/`:298`/`:449` are the three cycle entry points; the third needs identifying first).
- `pulse-app/src/observability.rs` — un-muting `triage.cue.tick`'s `cues_suppressed` / `bypass_triggered`
  (exact leaf; the scope permits un-muting exactly what this chunk needs).
- **Caller threading:** `triage::contract` re-exports `CadenceTriggerChannel` (`contract.rs:40`), and
  `pulse-app/src/main.rs:471` is the boot wiring — a signature change to the channel or the emitter reaches
  both. The two fan-out tests (`emitter.rs:518`, `:543`) PIN the current 1:1 semantics and will need
  updating with any change, which makes them the regression guard to re-point rather than delete.

## Scope premise closure
Applied to `scope.md` before P4 consumes it:
1. **"The runaway enters through the CUE-DRIVEN path, not the ticker"** — **VERIFIED and sharpened.** It is
   cue VOLUME, not the cue path's existence: `emit_cue` fans Suggested cues 1:1 with no limit.
2. **"The existing safety floors do not bound this path"** — **VERIFIED.** `config.rs` floors govern ticker
   intervals only.
3. **"A feedback loop is plausible and must be ruled in or out"** — **VERIFIED, and it is the dominant
   effect, running the OPPOSITE direction to the one the scope sketched.** The scope guessed L4-lag →
   digests queue → cues fire. Measured: consumer wedge → baseline starves → silence cues → triggers → deeper
   starvation.
4. **"Each cycle's 10 blocking queries are not themselves excessive"** — **VERIFIED** (10.0 both pre-freeze
   and across the run).
5. **"No product code outside `crates/triage/src/cadence/` needs to change"** — **FALSIFIED.** The amplifier
   lives in `crates/triage/src/cue/emitter.rs`; `cadence/` may not need to change at all.

**And the central premise itself is corrected:** the runaway does not cause the wedge — it follows it by 32
seconds and prevents recovery.

## Open questions
- **What froze the consumer at 17:20:50, under a load of ~1.7 L1a queries/s?** → blocks: **plan-decision**.
  This is the initiating cause; the chunk's outcome-2 ("the buffer keeps draining by repair") cannot be
  claimed without it.
- **What drives the ~2,000 cadence cycles unaccounted for by 606 Suggested cues + 51 ticks?** → blocks:
  **plan-decision** — a bound placed only on the cue fan-out would miss the majority of triggers.
- **Does the silence family latch per service, or re-emit every tick?** → blocks: **implementation-scope** —
  decides whether the fix is a latch, a rate limit, or both. Currently unanswerable from the log because
  `cues_suppressed` is redacted.
