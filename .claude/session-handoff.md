# Session Handoff

**Last Updated:** 2026-09-30T20:09:25Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-09-30-perf-instruments-measure-their-budgets — chunk wrap (perf instruments measure what their budgets name)

## Position
- Done: `2026-09-30-perf-instruments-measure-their-budgets`.
  - **Snapshot:** the snapshot sample now spans load + curate + format through `snapshot::contract::GenerationTimer`, its sole emitter. p99 is 94.0 ms (n = 50); it was 0.0 ms when it timed formatting only.
  - **Frame cause:** a 0-frame log names its cause from `ui.webgpu.adapter` records. The new `telemetry.frontend.record_webgpu_adapter` procedure was founder-ratified at P4 («Да, делай»).
  - **Release cache:** the `release-{os}` cache now saves on failure.
  - **Operator pass:** pre-CI commit `5fbf762`. `ci#36765040464` is green 13/13; both CI frame lines read `… no adapter record in this log`.
  - **Dev host:** frame p99 2.7 ms (n = 8045), with 2 `obtained` adapter records on the wire.
- Next (first markerless): **Span-level redaction**, then Real-model incident surfacing → Conductor return (P-075). The relay said no new entry.

## Work done
- Code: `crates/snapshot` (timer), `crates/ui-bridge` (procedure), `pulse-app` (observability leaf, snapshot_runtime), `crates/mcp-server`, `xtask` (`frame_cause`), webview `canvas/adapter-state.ts` + `webgpu-adapter.ts`, `ci.yml` (one line), and the regenerated bindings.
- Workspace nextest 2466 → 2485. `EXPECTED_PROCEDURES` 43 → 44. No dependency change and no capability JSON change.
- Evidence: `chunks/2026-09-30-perf-instruments-measure-their-budgets/evidence/` holds red-at-base, green-after, adapter-wire and ci.

## Drift resolved
22 amendments across 6 masters: 18 from detectors, 4 raised by the wrap. Two escalations resolved with the operator (`.andromeda/runs/2026-09-30T19-50-57Z-wrap/fanout-results.md`).
- **Boundary widening** (arch IPC routes · security §Input Validation + §Logging · obs §6/§8): resolved by the P4 ratification, quoted in the four sidecar entries.
- **Procedure-changing bindings close:** now specified in test-plan §3. The playbook bindings rule was sharpened IN PLACE: the regen comes BEFORE the `ui/dist` build and every release build a live leg drives.
- **obs-plan:** snapshot scope, frame cause, `ui.webgpu.adapter` dual-site. The dev-host frame leg is named as the only live adapter witness; the CI boot job never runs the webview long enough to issue IPC.
- **a11y-plan §3:** the suite-health probe names `--config=playwright-a11y.config.ts`.
- **design-system:** the fallback renders on any `unavailable` result.
- **Leaves re-derived:** rules `observability` / `security` / `a11y` / `frontend` / `testing` / `verification-harness`; docs `obs-summary` / `a11y-summary` / `tests-summary`.

## Notes
- **Ports:** 4317/4318 are shared with conductor-builder. STOP and ask the operator for the slot before any window-opening run (`self-verify`, `perf:frame-sample`).
- **Watch:** Actions cache headroom is 10 605 172 169 of 10 737 418 240 B (1.23 %), unchanged after the release key gained `cache-on-failure`, with no eviction.
- **Pre-existing tool verdicts, not this chunk's:** `route.py` UNPARSED/INDETERMINATE on frozen lines (:52, :54 ×2, :60, :100, :116, :125); `matrix.py show` UNPARSED P-072.
- **Epoch 4** is at 49 entries; the operator's no-split ruling stands.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout, and the U35 door. PR #39 stays a draft.
- **Still open:** the `sidecar.py` Ref defect relayed to overseer1 at session 65.
- **Last failed command:** none.

## Deferred learnings
- `recurrence-despite-learning`: testing.md 2026-08-30 "a suite-health probe in a multi-config Playwright repo must name the config". The plan copied the bare `--list` from a11y-plan §3; fixed at the source this wrap.
- `recurrence-despite-learning`: testing.md 2026-08-30 "the bundled `ARGS_MAP` decides a TauRPC method's runtime existence". The plan built dist + release before the regen; the check now sits in the playbook bindings rule + test-plan §3.
- Still open from prior wraps:
  - the release-cache `cache-on-failure` recurrence (now closed by this chunk's fix);
  - the implement report-step CHECK (unit-only claims vs a longer live run);
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.

## Session End Status
Completed normally at 2026-10-01 06:32:47
