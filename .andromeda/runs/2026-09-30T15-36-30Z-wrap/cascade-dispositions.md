# Cascade dispositions — 2026-09-30-perf-budget-gate-reads-real-samples

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml`, run AFTER the last body amendment of the pass (the second
run; the first preceded the test-plan :135 trim). Baseline `fb93fcac` (parent of the oldest pre-CI commit). 16
patterns, every control fired on the pre-pass masters. Retired claims keyed by token AND by mechanism:
- CI perf gate vacuous / fed by `perf-slo-check` / owned by "Perf-budget gate reads real samples" (`vacuous`,
  `slo-script`, `owner-entry`, `boot-smoke-only`, `check-scripts`, `neutral-tol`);
- p99 as a floor index or a max (`p99-floor`, `assert-max`);
- frame p99 fails the release build / deferred to headful E2E (`release-fail`, `frame-headful`);
- `app.boot.gpu.check` carries `gpu_available` (`gpu-avail`);
- ps1 writes no spawn/exit record (`ps1-mirror`, `ps1-null`);
- release restores lint-test read-only; memory not CI-enforced (`save-if`, `ro-lint-test`, `tail-max`).
Plus a hand mechanism grep over the seven masters for `cannot fail|perf gate|perf-budget gate|vacuous` (4 hits, read).

## Master rows (final listing)
| row | disposition |
|---|---|
| test-plan.md:333 vacuous (VACUOUSLY) | no change — a true claim about the wrong-reason-pass class, shares the token |
| architecture.md:243 slo-script (new) | amended text — states the scripts were deleted (true) |
| test-plan.md:135 slo-script (edited ×2) | amended — the row id `perf-slo-check-arm-coverage` + "scripts are deleted" (true); the original present-tense VACUOUS/owner text was cut from the discharged row this pass |
| obs-plan.md:641 slo-script (new) | amended text — states the deletion (true) |
| obs-plan.md:132 gpu-avail (new) | amended text — names the removed field as former (true) |
| obs-plan.md:640 assert-max | no change — the heartbeat gap check's own `assert max delta` (true, unrelated) |
| test-plan.md:727 check-scripts (new) | amended text — "the check scripts it once called were deleted" (true) |
| obs-plan.md:661 neutral-tol (edited) | amended — NEUTRAL tolerance scoped to unrequired arms (true) |
| architecture.md:321 ro-lint-test (edited) | amended — mcp-test / a11y / supply-chain still restore read-only (true) |
| test-plan.md:641 ro-lint-test (edited) | amended — mcp-test and a11y restore lint-test read-only (true) |
| test-plan.md:643 ro-lint-test | no change — mcp-test restore-only `lint-test-Linux` (true) |
| test-plan.md:645 ro-lint-test | no change — supply-chain restore-only `boot-Linux` (true) |
| obs-plan.md:630 tail-max (edited) | amended — the tail-and-max is the release-cadence load-profiles path (true) |
| `release-fail`, `p99-floor`, `ps1-null`, `frame-headful` | 0 rows after the pass (controls fired) — the retired mechanisms are gone from every master |
| hand grep: security-plan.md:438, test-plan.md:125 | no change — "vacuous" in other claims (a column count; evidence non-vacuity) |

No master cites another master as saying a retired thing: the four amended masters' cross-references (obs §10 ↔ test
§1/§3/§9/§10 ↔ arch xtask CLI / CI/CD ↔ security carve-out) were each amended in this pass to the same facts.
Bound pair test-plan §3 ↔ obs-plan §3: obs §3 carries no spawn/exit or ps1 claim (detector grep 0) — consistent.

## Curation homes and judgment bases
| row | disposition |
|---|---|
| .claude/rules/testing.md:240, :264 (VACUOUSLY) | no change — unrelated vacuous-pass learnings (Session Additions, preserve-verbatim) |
| .claude/docs/session-learnings.md:14 (`save-if: false`) | no change — a general cache-allocation rule that still holds (jobs sharing a graph restore read-only); names no job |
| playbook.md / drift-base.md | 0 rows |

## Leaves (step 3 re-derived)
| leaf | action |
|---|---|
| .claude/rules/observability.md:95/96/98 | re-derived — perf enforcement via `perf_budget`, frame dev-host, snapshot timer scope, memory gauge rows × 256 B, NEUTRAL posture scoped |
| .claude/rules/verification-harness.md:22/24 + xtask list | re-derived — ps1 recorder, `ended` real under ps1, `perf:budget` + `perf:frame-sample` bullets |
| .claude/rules/security.md:17 | re-derived — sibling harness-only class (boot-recorder state files) |
| .claude/rules/testing.md:59 | re-derived — `[profile.perf-samples]` |
| .claude/docs/obs-summary.md:75–78, :83 | re-derived — the three perf rows; the VACUOUS status row removed; NEUTRAL posture |
| .claude/docs/tests-summary.md:12/14/76 + scenario section | re-derived — ps1 recorder, `ended`, perf-samples profile, `perf:frame-sample` / `perf:budget` |
| .claude/docs/security-summary.md:43 | re-derived — sibling class |
| .claude/docs/commands.md, .claude/docs/workflow.md:44 | re-derived — perf verbs; perf-budget mechanism |
| CLAUDE.md overview (xtask dir), modules (xtask), warnings (path env vars) | re-derived — perf verbs; sibling harness-only class |
| .claude/docs/tests-summary.md:92 (VACUOUSLY) | no change — unrelated vacuous-pass claim |
| .claude/docs/obs-summary.md:73 (assert max) | no change — heartbeat gap (true) |
