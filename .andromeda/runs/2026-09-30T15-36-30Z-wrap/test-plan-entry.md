
## 2026-09-30-perf-budget-gate-reads-real-samples — perf grader coverage, the dev-host frame gate, the CI perf-sample producer
**Section:** §1 `perf-slo-check-arm-coverage` · §1 `performance-budget: WebGPU canvas throughput` · §3 Per-chunk gate discipline (the `check:ingest-progress` analogy; the new `perf:frame-sample` / `perf:budget` paragraph) · §9 lint-test row (commands, cache cell) · §9 release row · §10 frame-budget note · §10 Load profiles
**Change:**
- `perf-slo-check-arm-coverage` DISCHARGED: the scripts are deleted; `xtask::perf_budget`, 15 unit pins (per arm empty ⇒ NEUTRAL, in ⇒ PASS, over ⇒ FAIL; non-numeric ⇒ FAIL; required-empty ⇒ FAIL; all-empty ⇒ NEUTRAL, never PASS). Its present-tense "CI gate VACUOUS" text is retired.
- Frame budget: was "deferred to tauri-driver headful E2E"; now the dev-host `perf:frame-sample` (Windows, frame arm required; exit 0/1/2), not CI-wired; the CI frame arm prints the named cannot-evaluate line. The §10 note no longer says "all CI assertions".
- §9 lint-test Linux adds `--profile perf-samples` → `perf:budget --require memory,snapshot` → the `logs-perf-samples-*` upload; §10 records `[profile.perf-samples]` and the default filter's second exclusion; `perf:load-profiles` grades in-process (was "the check scripts").
- §9 release: owns and saves `release-${{ runner.os }}` (was restore-only lint-test); no CI frame leg; lint-test's key is restored read-only by mcp-test and a11y only.
**Why:** the chunk replaced the untested script with a pinned grader and fed CI real samples. The frame gate moved to the dev host on the operator's decision after the hosted runner read 0 samples. Release got its own cache key.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/
