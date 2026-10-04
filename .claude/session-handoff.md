# Session Handoff

**Last Updated:** 2026-10-04T00:42:51Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-02-incident-events-readable-through-mcp — chunk wrap (an incident's lifecycle events read back through MCP; P-075 re-verified on a fresh Conductor round)

## Position
- Done: `2026-10-02-incident-events-readable-through-mcp`.
  - The ninth MCP tool `retrieve_incident_events`.
  - The closed four-kind event vocabulary in `triage::contract`, shared by producer and sidecar (founder option A, 2026-10-03).
  - P-075 7/7 at Conductor `e6e1eef` (CI#37162108538) against S2 `cdb6c1e`, under `WEBKIT_DISABLE_DMABUF_RENDERER=1`.
- **Next: "Supply-chain advisories on wasmtime resolved"** (founder ruling 2026-10-04). Then, in order:
  - the corpus key-creation race (a locked create-or-read);
  - the Linux launch on NVIDIA + Wayland (measure the cause first);
  - "Retry-storm interpretation names its cause";
  - "pre-push:linux runs natively on Linux" (overseer placement; the founder can move it).

## Work done
- `corpus` / `mcp-server` / `triage` / `pulse-app` sources; two new cross-process legs (mcp-server subprocess + real-producer e2e).
- Workspace 2575 tests on Linux; dev host moved to Omarchy Linux 2026-10-03.

## Drift resolved
26 amendments across 4 masters, 2 escalations resolved by recorded founder ratifications (`.andromeda/runs/2026-10-03T23-46-09Z-wrap/fanout-results.md`).
- **architecture:** MCP roster 8 → 9 at four sites; `incident_events` census — two writers, four kinds, one reader.
- **security-plan:** roster 8 → 9 at two sites (Boundary widening: founder P4 2026-10-02 + option A 2026-10-03); `incident_events` NO-SCRUB basis restated.
- **test-plan:** the cross-process trigger narrowed and P3 residual; `pre-push:linux` cannot run on the Linux dev host; the Unix-only exit arms run natively.
- **obs-plan:** the `method` set names all nine tools (5 sites); the app log sink is file-only (8 sites — discharges the route CARRY).
- **Leaves re-derived:** CLAUDE.md modules, conventions, services/corpus, services/mcp-server, obs-summary, rules/observability, tests-summary, rules/verification-harness.

## Notes
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches pulse-app, a window or the model.
- **Host:** the dev host is Omarchy Linux since 2026-10-03, not Windows. The `host-win32.md` and LSP-flycheck notes from the Windows era may not apply. Python `duckdb` is not installed here, so the code-graph refresh reads STALE until it is.
- **Founder rulings:** record by name and date, relayed by the pc overseer, never as "Viola"; sidecars name the ruling, never quote it.
- **Epoch 4** is at 55 entries; the no-split ruling holds. It now closes at the wrap that completes "pre-push:linux runs natively on Linux", with the sidecar consolidation and the diagnose nudge then.
- **Plan gate 6** (no pulse-app source change) is an owned red superseded by the founder ruling of 2026-10-03 (option A widening).
- **Pre-existing tool verdicts, not this chunk's:** `route.py` UNPARSED/INDETERMINATE on frozen lines; `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y).
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Still open:** the `sidecar.py` Ref defect relayed to overseer1 at session 65. Actions cache headroom not re-read.
- **Last failed command:** none.

## Deferred learnings
- `recurrence-despite-learning` (writer census at the wrong layer): research mapped `incident_events` writers at the corpus SQL layer and missed `IncidentPersistence::save_incident_event` one layer up. This happened despite the CLAUDE.md 2026-05-30 producer-existence learning. The remedy is a CHECK in phase research: grep the trait-level persist call, not only the SQL.
- `recurrence-despite-learning` (timeout sizing): the remedy (a targeted nextest entry carries a `timeout`) names no sizing rule. The CHECK should size from a measured cold build plus margin.
- Still open from prior wraps:
  - the `producer | grep -q` under pipefail plan-authoring CHECK;
  - the implement report-step CHECK (unit-only claims vs a longer live run);
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.
