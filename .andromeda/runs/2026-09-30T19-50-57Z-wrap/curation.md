# Curation — 2026-09-30-perf-instruments-measure-their-budgets

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup → recurrence (→ handoff Deferred learnings) · 2 rejected (1 task-specific · 1 below threshold) · 0 conflict · 0 deferred by cap
  CLAUDE.md size: see the P7 health check-1 row

## Recurrences (Filter 1: a dedup hit on a DEFECT record → never a silent drop, never a third entry)

- **`recurrence-despite-learning`: `.claude/rules/testing.md` Session Additions 2026-08-30 "A SUITE-HEALTH PROBE IN A MULTI-CONFIG PLAYWRIGHT REPO MUST NAME THE CONFIG".**
  The chunk plan's `## Test Commands` listed the bare `cd pulse-app/ui && npx playwright test --list`, copied from a11y-plan §3, which still taught the bare form. It read red by construction at /implement (exit 1, `Total: 0 tests in 0 files`; config-named form exit 0, 41 tests in 18 files).
  The remedy landed at its source this wrap:
  - a11y-plan §3 amended;
  - leaves `.claude/rules/a11y.md` (Suite health) and `.claude/docs/a11y-summary.md` re-derived.
  So the next plan copying the master copies the right form. Relay item 2.1 is discharged there. Plan-template sweep: `grep -- --list` over the phase skill's `plan-template.md` → 0 hits (it does not teach the bare form).
  Proof: implement gate trail `.andromeda/runs/2026-09-30T18-06-28Z-implement/` (entry 18); `report.md` Spec claims disproved 2.
- **`recurrence-despite-learning`: `.claude/rules/testing.md` Session Additions 2026-08-30 "THE BUNDLED `ARGS_MAP` DECIDES A TAURPC METHOD'S RUNTIME EXISTENCE"** (regen → dist → release → legs).
  The plan ordered the `ui/dist` build and the release build BEFORE the bindings regen for a chunk adding a procedure. Caught at /implement before any window opened: 1 bundle occurrence vs 2, dist + release re-run after the regen. The live leg then recorded 2 `obtained` records.
  The remedy is a CHECK in the owning step's reference. On the operator's approval this wrap:
  - the playbook bindings-order rule was sharpened in place;
  - test-plan §3 now specifies the procedure-changing close and the regen-before-dist/release order;
  - leaves `rules/testing.md` (chunk-gate-baseline-coverage) and `docs/tests-summary.md` were re-derived.
  Relay item 2.2 is discharged there.
  Proof: `report.md` Deviation 2; `evidence/adapter-wire.md` §Binary freshness.

## Rejected

- **Gate tool's backgrounded output file stays empty until the run ends; progress lives in the per-entry logs under `%TEMP%/andromeda-gate/{marker}/{run}/`.** Score 0.4 (specific technical detail +0.2; no-other-home +0.2 would apply only at exactly 0.6). Below threshold: it cost one expired monitor and changed no design; the stdout block-buffering family is already curated (testing.md 2026-08-25).
- **`no WebGPU adapter` as a sweep pattern also matches the new cause text; sweep by the full retired form.** Task-specific: the retired string is now gone from every master and leaf, so the hazard has no future site.
- (not a candidate) The CI boot job never runs the webview long enough to issue IPC. It is a durable fact, amended this wrap into obs-plan §1/§10, security-plan §Logging, architecture §Occupied Resources and test-plan §1. The masters own it.
