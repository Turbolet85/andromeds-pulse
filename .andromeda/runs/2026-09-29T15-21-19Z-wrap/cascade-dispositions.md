# Cascade dispositions — 2026-09-29-p-025-hue-shift-observable-made-gradable

The search: `cascade.py sweep` over `cascade-patterns.toml` (15 patterns; listing in `sweep-out.txt`), run after the
last body amendment of the pass and before any sidecar entry. Every pattern's control fired on the pre-pass masters.

Patterns and the claims they key (verbs and phrasings, not only names):
- `wasm46` — the retired wasmtime pin (`46.0.3`, requirement `"46"`).
- `span-arrival` — the retired P-025 interval ("span-arrival → hue").
- `hue-leaf` — every statement of the `hue_update_ms` leaf, to read each for the interval it asserts.
- `pid-mtime` — the retired liveness mechanism ("pidfile + log mtime" as the whole derivation).
- `ciyml12` / `wf-dataenv` — the retired workflow-level `ANDROMEDA_PULSE_DATA_DIR`.
- `audit-noload` / `audit-cadence` — the retired "cannot load the DB" basis and the probe cadence / pin wording.
- `ggr8` — the pruned deepmerge-ts residual.
- `vitest3` — the retired webview runner major.
- `perf-ci` — every perf-budget statement, to read each for a claim that CI enforces it ("asserted in CI", "Build fails").
- `cov-scope` — coverage-threshold statements, to read each for a whole-workspace measure claim.
- `runs-in-ci` / `hs-healthy` / `harness-status` — the CI-run claim and every harness:status derivation statement.

Sections read beyond the rows: arch §Occupied Resources (:176, :182, :214, :235–:241), security-plan §Dependency Security
(:213–:221), obs-plan §1 (:125–:127), §8 (:542), §10 (:624–:642), test-plan §1 (:55, :66, :130–:135), §3 (:180–:268,
:308–:315), §4 (:326–:335), §5 (:399), §9 (:626–:640), §10 (:686–:696).

## Rows

### masters
- architecture.md:22 wasm46 — amended this pass (the row names 46.0.3 as the version the bump came FROM) — no further change.
- test-plan.md:399 wasm46 — STALE (the §5 plugins row pinned `wasmtime` 46.x / 46.0.3) → AMENDED this pass to 48.x / 48.0.3.
- obs-plan.md:542 span-arrival — amended (the retired interval is named as the prior one) — no change.
- architecture.md:176 · obs-plan.md:542 hue-leaf — amended this pass.
- architecture.md:241 · test-plan.md:315 hue-leaf — this pass's new text (the smoke:hue-shift registration).
- layout-templates.md:100 hue-leaf — true: says the observable measures the real dot surface; states no interval — no change.
- obs-plan.md:162 hue-leaf — a target-naming line with no interval claim — no change.
- test-plan.md:632 ciyml12 — the match is `ci.yml:125` in the a11y row (a true coordinate sharing the token) — no change.
- security-plan.md:219 audit-noload / audit-cadence — amended; the ended deferral's history names its basis — no change.
- security-plan.md:221 ggr8 — amended; names the prune — no change.
- test-plan.md:331 vitest3 — amended; names the vulnerable prior major — no change.
- obs-plan.md:126 · :629 · :630 · :641 perf-ci — amended (vacuity status + owner).
- obs-plan.md:27 · :29 · :125 · :127 · :235 · :585 · :594 · :624 · :688 perf-ci — scope prose, trigger names, artifact
  rows; none asserts that CI enforces a p99 today — no change.
- test-plan.md:135 perf-ci — this pass's new trigger row.
- test-plan.md:290 perf-ci — the quality-gate-config-emit TRIGGER text (a requirement description, not an enforcement
  claim) — no change.
- test-plan.md:335 · :640 · :734 cov-scope — :335 amended; :640 (the Quality gates row) and :734 (build-failure
  conditions) state the thresholds, which are unchanged; the measure's exclusion is stated at §9 Coverage report and §10
  — no change.
- test-plan.md:632 runs-in-ci — amended.
- test-plan.md:206 hs-healthy (row not printed under hs-healthy for the definition but read) — STALE ("7 unit pins") →
  AMENDED: pinned per arm, dead-pid / exited-child included; the `not-running` arm names the dead/zombie case.
- test-plan.md:183 · :199 · :222 · :323 · :662 hs-healthy / harness-status — readiness poll, the verdict enum, the
  health-envelope note, the boot-smoke fail condition — true, no derivation claim — no change.
- architecture.md:214 · :241 harness-status — amended. :235 · :236 — the PIDFILE / LOGFILE env-var rows (who reads the
  var) — true — no change.
- test-plan.md:66 · :195 · :268 — amended.

### leaves (step 3 set, re-derived from the amended masters)
- CLAUDE.md:9 wasm46 (GENERATED overview) → re-derived: requirement "48.0.3", RUSTSEC-2026-0316, the 1.96 bound.
- .claude/rules/security.md:74 audit-noload / audit-cadence → re-derived from security-plan :219 (deferral ENDED,
  `$CARGO_HOME` note, counting rules kept).
- .claude/rules/observability.md:72 hue-leaf → re-derived (interval); :95 perf-ci → re-derived (VACUOUS status + owner);
  :87 perf-ci — the metric list, true — no change.
- .claude/docs/obs-summary.md:140 hue-leaf → re-derived; the §SLO invariants table gained the CI-vacuity status row
  (found by reading the table the perf-ci rows point into, :69–:78); :3 perf-ci — scope prose — no change.
- .claude/rules/verification-harness.md:24 · :59 pid-mtime → re-derived; :22 boot → re-derived (failure diagnosis);
  :29 harness-status — true — no change.
- .claude/docs/tests-summary.md:14 pid-mtime · :55 vitest3 · :72 cov-scope → re-derived; :12 · :54 — true — no change.
- .claude/rules/testing.md:18 vitest3 · :56 cov-scope → re-derived.
- CLAUDE.md:67 · :69 · :89 perf-ci / cov-scope — the pointer table and the Workflow line state thresholds and pointers,
  still true — no change.
- .claude/docs/workflow.md:43 · services/snapshot.md:66 · commands.md:86/:88/:100 · rules/observability.md:83 ·
  obs-summary.md:64 — thresholds / pointers / verb listings, true — no change.
- Not in the sweep, checked by provenance: `.claude/docs/stack.md:16` and `services/plugins.md` state `wasmtime` 25+ (the
  floor, still true); `security-summary.md` carries no deferral, ggr8 or pin text (grep 0) — no change.

### curation homes (preserve-verbatim)
- .claude/rules/testing.md:171 · :177 perf-ci (Session Additions) — a load-generator note and a doc-lint note; neither
  states CI enforcement — no change, nothing routed.
- .claude/docs/session-learnings.md:2277 · :2284 · :2286 harness-status — the xtask alias entry — true — no change.

### judgment bases
- .andromeda/playbook.md:60 audit-noload — a SUPERSEDED rule's codification record quoting the 2026-08-15 basis
  (history) — no change, nothing proposed.
- .andromeda/playbook.md:112 audit-cadence — the generic external-decay rule — true — no change.

## Sidecar supersessions this pass
security-plan: the five discharge-only entries whose every claim was a probe point of the now-ENDED deferral — sessions
28 (2026-08-17), 46, 49, 55, 64 — are named in the deferral entry's Supersedes. Entries mixing a discharge with a rule
that still stands (2026-08-16 re-ratification, 2026-08-25 counting rules, 2026-08-28 ordinal retirement, 2026-08-29
backlog, 2026-08-30 acl) are left: partial retirement, named in Change.
