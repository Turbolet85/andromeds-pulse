# Session Handoff

**Last Updated:** 2026-10-01T18:30:53Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-01-real-model-incident-surfacing — chunk wrap (the real model turns a storm digest into an incident for a measured cause)

## Position
- Done: `2026-10-01-real-model-incident-surfacing`.
  - **Surfacing — the fix:** A5 (the L4 schema emits `decision` / `severity` after the analysis) plus a truthful `OVERALL:` line. The shipped tree read 29/30 would-create, against 24/30 on the untouched base.
  - **Surfacing — selection:** A1 (`--temp 0`) was declined by founder ruling (2026-10-01), so the pre-registered order fell through to A5.
  - **Premise correction:** "the model dismissed" measured 1 of 30; the rest were `severity: none`.
  - **Observability:** the new `interpretation.incident.skipped` record makes a no-incident outcome visible.
  - **PREREQ:** `cargo xtask check:english-sources`, ASCII-only, in CI and as the sixth `pre-push:linux` stage.
  - **Operator pass:** two CI reds on `69f0b93` were solved in-chunk:
    - a `basic-ftp` npm override (a widening on the overseer's word);
    - the boot-smoke death re-ran healthy.
  - **Final CI:** `f37cd3e` green, `ci#36902837947`, 13/13.
- Next: **Conductor return** (P-075). It carries a new PREREQ: every non-zero exit path logs its cause before exiting, because the Linux CI boot smoke's app died once with `exit 1` at 0.27 s, cause unexplained.

## Work done
- Code: the xtask `source_lint` verb plus its pre-push stage and the ci.yml step; the skip emit and its exact allowlist leaf; `render_payload` / `cue_summary` exposed (doc-hidden); the OVERALL rule; the schema reorder with prompt v2.3 / v1.2-fallback / v1.2-reflection; the dev probe `pulse-app/examples/l4_decision_probe.rs`; the npm override.
- Workspace nextest 2530 → 2551 (+21). Webview 863/863.
- Evidence is in `chunks/2026-10-01-real-model-incident-surfacing/evidence/`: the arm matrix, the plant proof, mutation ×4, Slot 2 and the operator pass.

## Drift resolved
8 amendments across 3 masters, 0 escalations (`.andromeda/runs/2026-10-01T18-16-18Z-wrap/fanout-results.md`).
- **architecture:** `check:english-sources` registered; `pre-push:linux` has six stages.
- **test-plan:** §3 six stages · §4 interpretation (lineage v2.3, schema-order pin) · §4 triage (OVERALL pins) · §1 new trigger `l4-decision-probe-arg-parse-unit-coverage` · §9 the lint-test row names the source gate.
- **obs-plan:** §8 `interpretation.incident.skipped`.
- **Leaves re-derived:** `CLAUDE.md` (xtask lines), `docs/commands.md`, `docs/tests-summary.md`, `docs/obs-summary.md`, `rules/verification-harness.md`, `rules/observability.md`. The sweep caught two stale five-stage leaves.

## Notes
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches pulse-app, a window or the model.
- **LSP flycheck:** rust-analyzer respawns a `cargo check` after every source edit. Stop it by PID before cargo runs, identified by its `rust-analyzer.exe` parent; clippy's own `cargo check` child looks the same (curated to `host-win32.md`).
- **Founder rulings:** record by name and date, relayed by the pc overseer, never as "Viola" (the driver tool); sidecars name the ruling, never quote it.
- **Watch:** Actions cache headroom was 1.23 % at an earlier read (not re-read this wrap).
- **Pre-existing tool verdicts, not this chunk's:** `route.py` UNPARSED/INDETERMINATE on frozen lines; `matrix.py show` UNPARSED P-072; `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y).
- **Epoch 4** is at 49 entries; the operator's no-split ruling stands. It closes when Conductor return completes.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout, and the U35 door. PR #39 stays a draft.
- **Still open:** the `sidecar.py` Ref defect relayed to overseer1 at session 65.
- **Last failed command:** none.

## Deferred learnings
- `recurrence-despite-learning`:
  - Matched: the Tier 3 entry "A gate entry's time bound can read as a link failure…" (2026-10-01) and testing.md 2026-06-28 ("-E filters RUN not COMPILE").
  - What recurred: this plan's targeted nextest entry carried no `timeout` key and timed out twice at the 1800 s default, compiling every test binary after source edits.
  - Remedy: a CHECK in phase's plan authoring — a targeted nextest entry carries a `timeout` sized for a cold test build.
- `recurrence-despite-learning` (from the prior wrap, still open): a `producer | grep -q` poll under pipefail. The remedy is the same plan-authoring CHECK class.
- Still open from prior wraps:
  - the implement report-step CHECK (unit-only claims vs a longer live run);
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.
