# Session Handoff

**Last Updated:** 2026-09-30T11:41:40Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-09-30-p-027-discovery-bound — chunk wrap (first-sighting registration; the smoke:discovery leg)

## Position
- Done: `2026-09-30-p-027-discovery-bound`.
  - A service is registered in the lifecycle registry at its FIRST span, by a third `SpanObserver`
    (`DiscoveryObserverAdapter`, composed after the baseline adapter) instead of at the 15 s registry tick.
  - New scenario leg `cargo xtask smoke:discovery`: RED at `71f3369` 15 219 ms; GREEN 177 ms (anchor error 6 ms).
    P-025 `smoke:hue-shift` PASS, rise 644 ms (was 9 986).
  - Operator pass on the operator's word: pre-CI commit `87fe658`; CI green (ci#36706243490 13/13 + secret-scan).
- Next (first markerless): **Perf-budget gate reads real samples** — it carries the release-job cache CARRY and the
  `agent-run.ps1` recorder-mirror CARRY.
- Then: Span-level redaction → Real-model incident surfacing → Conductor return (P-075).

## Work done
- Files: registry (`register_first_sighting` / `take_first_sightings`) · tick-fold into
  `triage.lifecycle.transition` · `BaselineState::tracks_service` · `pulse-app/src/discovery_observer.rs` ·
  the `main.rs` composition · `xtask/src/discovery.rs` · the v0.2.0 matrix P-027 note.
- 14 new tests; workspace 2447/2447. No dependency, procedure, bindings or `pulse-app/ui/**` change.
- Cross-project (Conductor :63 evidence): the GREEN-leg `target/release/pulse-app.exe` sha256 is
  `9e51d1d92e80fdc0b998fe5e1c65fbbd9c5eef4dd9e6b7c4883c5ccad1bf9ab4`. It was built from the product sources
  committed at `87fe658` (recorded in `report.md` Cross-project).

## Drift resolved
The six plan expected amendments all arrived as detector proposals, all routine; 0 escalations.
- architecture: `smoke:discovery` in the xtask CLI surfaces; the P-027 `discovery_ms` anchor in the delegated-timing
  entry.
- security-plan Logging: the tap's composite fan-out now keys the lifecycle registry; the choke point's consumers
  stay three.
- test-plan: the fourth SCENARIO leg (§3); the new open trigger `discovery-observer-wiring-coverage` (§1).
- obs-plan §8: the anchor sentence.
- Leaves re-derived: tests-summary, obs-summary, `rules/{observability,security,verification-harness}.md`. The
  last also gained `smoke:hue-shift`, which the P-025 cascade had missed.

Trail: `.andromeda/runs/2026-09-30T11-29-23Z-wrap/`.

## Notes
- Ports 4317/4318 are shared with conductor-builder (operator protocol this session): STOP and ask the operator for
  the slot before any live leg or `self-verify`. The operator may also hold a quiet-desktop window with no app
  windows.
- Pre-existing tool verdicts, not this chunk's:
  - `route.py` prints 7 UNPARSED/INDETERMINATE on frozen working-route lines (:52, :54 ×2, :60, :100, :116, :125).
  - `matrix.py show` prints `UNPARSED: P-072 — legacy notes placement` at `verification-matrix.json:161`.
- Epoch 4 is at 48 entries. The operator's no-split ruling stands; the version close is the boundary.
- Not this wrap (founder's hand): the `.gitattributes` re-checkout; the U35 door. PR #39 stays a draft.
- Still open: the `sidecar.py` Ref defect relayed to overseer1 at session 65.
- Last failed command: none.

## Deferred learnings
- `recurrence-despite-learning`: implement reported the tick fold "unit-only" from one short leg's log, while a
  longer run in the same slot had proven it live. The corpus already carries the absence-needs-its-probe and
  run-window rules; the remedy is a CHECK in implement's report step, not a third entry.
- Still open from prior wraps:
  - The bindings-regen PIPELINE half: the plan template's gate order.
  - macOS `SystemTime` µs ticks.
  - Windows `.ico` vs palette PNG.
  - The deferral-destination generalization.
  - `inject_demo --sustained` cannot form an incident — a CHECK for the leg-authoring reference.
