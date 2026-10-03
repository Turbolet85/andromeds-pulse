# Session Handoff

**Last Updated:** 2026-10-02T13:12:00Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-01-conductor-return — chunk wrap (every loggable pulse-app exit names its cause; P-075 verified by a fresh Conductor round)

## Position
- Done: `2026-10-01-conductor-return`.
  - **PREREQ:** one `app.exit` record per loggable process end, the file sink drained before the process goes (`run_return` tail · `atexit` hook via a reporter thread · Unix SIGTERM/SIGINT re-raised).
  - **Measured:** Windows `std::process::exit` = `ExitProcess`, runs no `atexit` — unloggable there (`evidence/red-probe.md`); flush race 0/20 vs 20/20 with the guard dropped.
  - **P-075:** Conductor round on S `03ec944` = 6/6 PASS at Conductor `2a494804` (CI#36970919487); ref written on the overseer's word.
  - **CI on S:** `ci#36964436269` green 13/13.
- Next: **Incident events readable through MCP** (Epoch 4, founder ruling 2026-10-02). It carries P-075's re-verification (a fresh Conductor round covering the events) and the stderr-sink spec correction. Then **Retry-storm interpretation names its cause** (measure first).

## Work done
- `pulse-app/src/{observability,main}.rs`, `libc` direct dep, two new test targets (+9 tests Windows / +11 Linux; workspace 2560 / 2562).
- Evidence in `chunks/2026-10-01-conductor-return/evidence/`: red-probe · mutation-checks · round-request · round-binary · operator-pass.

## Drift resolved
9 amendments across 3 masters, 0 escalations (`.andromeda/runs/2026-10-02T12-54-57Z-wrap/fanout-results.md`).
- **obs-plan:** §1 / §3 init order (guard parking, exit hook, signal listener, `run_return` tail) · §6 `app.exit` · §7 process-end class + unloggable ends · §8 the exact leaf.
- **security-plan:** Logging (`app.exit` NO-SCRUB boundary) · Universal (no logging from an `atexit`/signal handler on the exiting thread).
- **test-plan:** §1 trigger `exit-hook-main-composition-coverage` · §3 the process-end witness form.
- **Leaves re-derived:** obs-summary, rules/observability, rules/security, security-summary, tests-summary.

## Notes
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches pulse-app, a window or the model.
- **LSP flycheck:** rust-analyzer respawns a `cargo check` after EVERY source save; stop it by PID (parent `rust-analyzer.exe`) before each cargo run.
- **Founder rulings:** record by name and date, relayed by the pc overseer, never as "Viola"; sidecars name the ruling, never quote it. Today's: everything planned for 0.3.0 lands in 0.3.0 (two entries minted; no 0.4.0 residual).
- **Epoch 4** is at 51 entries; the no-split ruling holds. It closes at the wrap that completes "Retry-storm interpretation…", with the sidecar consolidation and the diagnose nudge then.
- **Pre-existing tool verdicts, not this chunk's:** `route.py` UNPARSED/INDETERMINATE on frozen lines; `matrix.py show` UNPARSED P-072; `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y).
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout, and the U35 door. PR #39 stays a draft.
- **Still open:** the `sidecar.py` Ref defect relayed to overseer1 at session 65. Actions cache headroom not re-read.
- **Last failed command:** none.

## Deferred learnings
- `recurrence-despite-learning` (timeout sizing): the prior wrap's remedy (a targeted nextest entry carries a `timeout`) was applied at 3600 s and the cold run still measured 3564.77 s under host contention; the operator raised it to 5400. The remedy names no sizing rule — the CHECK should size from a measured cold build plus margin.
- Still open from prior wraps:
  - the `producer | grep -q` under pipefail plan-authoring CHECK;
  - the implement report-step CHECK (unit-only claims vs a longer live run);
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.

## Session End Status
Completed normally at 2026-10-03 16:01:52
