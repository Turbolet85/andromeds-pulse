# Fan-out results — 2026-09-29-ci-wall-time-and-round-trips

Seven Explore doc-agents, one batch, verbatim prompt; returns YAML-only (no stripping beyond trailing commentary lines,
which carried notes only); no HTML entities in any return (probe: 0).

| doc | verdict |
|---|---|
| architecture | 8 proposals |
| security-plan | `proposals: []` (notes: no new boundary, deps none, run-dir files harness-written) |
| design-system | `proposals: []` (no UI; no status claim touched) |
| layout-templates | `proposals: []` (no UI surface) |
| test-plan | 11 proposals |
| obs-plan | `proposals: []` (note: §9 Telemetry artifact handling :583 still owed — expected amendment) |
| a11y-plan | `proposals: []` (note: §9 / §3 CI integration owed — expected amendment) |

## architecture (8)
- A1 D-arch-resources · §Occupied Resources → xtask CLI surfaces · register `cargo xtask pre-push:linux` (0/1/2, JSON
  verdict + twin, WSL sync, 5 stages, no port/env) — **apply** (check 1: Accurate this-chunk addition; the bullet
  enumerates formalized verbs, prior chunks registered theirs; expected amendment). Folds A8's Viola Node fragility.
- A2 D-arch-resources · same · `harness:status` JSON gains `ended` — **apply** (check 1: accurate this-chunk change).
- A3 D-arch-resources · same · `agent-run.{sh,ps1}` boot waiting wrapper, ps1 not mirrored — **apply** after reading the
  site (check 1; check 4: body read before editing).
- A4 D-arch-resources · same · register `smoke:gap-resume` + `smoke:external-resolve` (CARRY E) — **apply** (expected
  amendment; by-construction evidence in the report).
- A5 D-arch-resources · §Occupied Resources → Filesystem locations · `run/andromeda-pulse.{spawn,exit}` — **apply**.
- A6 (dependent-of D-arch-resources) · Filesystem pidfile passage · boot's pid from the spawn record — **apply** with A5.
- A7 D-arch-decisions · §Infrastructure Patterns → CI/CD approach · 7-job ci.yml — **apply** (expected; disproved claim
  arch:319 disposed here).
- A8 D-arch-decisions · §Stack · new dev-only "Linux pre-push verification" row — **reject** (check 1: Registry
  over-reach — a dev-host environment serving one xtask verb is realization inside the registered xtask CLI surface,
  not a stack member); its fact (the cross-project Node 24 fragility) lands in A1's registration instead.

## test-plan (11)
- T1 D-tests-obs-harness · §3 → status · verdict JSON gains `ended` — **apply**.
- T2 (dependent) · §1 harness summary · same shape — **apply**.
- T3 D-tests-obs-harness · §3 → boot · waiting wrapper, spawn/exit records, `app ended:`, ps1 not mirrored — **apply**.
- T4 (dependent) · §3 → PID file lifecycle · provisional pid from the spawn record; the two harness files — **apply**.
- T5 D-tests-obs-harness · §3 · `pre-push:linux` paragraph — **apply** (expected amendment).
- T6 D-tests-framework · §9 Pipeline structure → A11y row · own `a11y` job — **apply** (expected).
- T7–T11 (dependents) · §9 rows: Boot smoke (own `boot` job) · Lint (`lint-test`, key) · Supply chain (restore-only
  boot-Linux, sole Linux release build) · Coverage (registry-only) · new `release` + `mcp-test` rows — **apply**.

## Raised by the orchestrator (check 5 — expected amendments no detector proposed)
- O1 obs-plan §9 Telemetry artifact handling · `logs-boot-${{ runner.os }}` beside `logs-${{ runner.os }}`; the row's
  "every CI job (fmt + clippy + xtask test + release)" wording follows the jobs — **apply** (routine; report Schema/config).
- Y1 a11y-plan §9 Per-pipeline-stage a11y artifacts (+ §3 CI integration where it names the job) · the a11y gate is its own
  matrix job — **apply** (routine; report Schema/config).

## Check 3 (intent) · check 6 (disproved claims)
- Scope record: 3 in-intent (the `cargo_command()` spawn sites, serving main.rs) + 2 widening carrying the overseer's word
  (`agent-run.sh`, `harness_status.rs`) — justified branch; intent amended by the operator directives recorded in the
  report (7 jobs; the recorder). No escalation.
- Disproved: plan forecast + research cause → recorded in the report, no amendment owed; arch:319 → A7.
- Cross-contradiction: none (A2/T1 and A3/T3 state the same facts in their own masters).
