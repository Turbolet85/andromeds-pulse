
## 2026-10-10-no-gate-stands-while-reading-nothing — ci-gates narrowed: the Boot smoke row, the two frame-line rows of §1, the boot job's failure conditions, the pre-push paragraph
**Section:** §9 → Pipeline structure (Boot smoke row) · §9 → Build failure conditions · §1 → Coverage triggers (the WebGPU canvas throughput row) · §1 → Pending coverage triggers (`perf-slo-check-arm-coverage`) · §3 → Per-chunk gate discipline
**Change:**
- Boot smoke row: `cargo xtask ci-gates` over the log the smoke wrote now states what it reads and returns: two arms over the `agent-latest.jsonl*` family, zero-spans and zero-panic; exit 0 PASS with exactly `ci-gates: zero-spans PASS ({n} log records across {k} file(s))` and `ci-gates: zero-panic PASS` · 1 FAIL · 2 cannot-evaluate on an absent log family; no heartbeat, perf-budget or frame line; no CI step makes the heartbeat gap check; 110 records on `ci#38042949735`.
- Build failure conditions: where `ci-gates` runs, the boot job also fails on its exit 1 or exit 2; an absent log no longer reads exit 0.
- §1 WebGPU canvas throughput row: was "the boot job's line reads `no WebGPU adapter (no_navigator_gpu)`" on a run whose smoke and series pass; now the frame arm's `cannot-evaluate` line on CI is the `lint-test` `perf:budget` step's alone, the boot job prints no frame line on any run, and the earlier boot-job readings are dated.
- §1 `perf-slo-check-arm-coverage` row: the same boot-job reading bounded to before this chunk.
- Per-chunk gate discipline key, the `pre-push:linux` paragraph: the `ci-gates` stage reads two lines over its seed and makes no heartbeat or perf-budget read; it reads a record it wrote itself, a reading carried on the working route; the `test` stage's `cargo xtask test` exits non-zero on a selection that matches no test.
**Why:** The chunk narrowed the verb to the arms that read the boot log and gave an absent log its own exit; four master sentences still described the arms that left or the line they printed.
**Kept:** "`ci-gates` is skipped when the smoke or the series fails" and "runs in this job only" stand. The pre-push stage is outside P-128, which is about what a CI job does (the operator's answer at the plan's dialog).
**Ref:** .andromeda/runs/2026-10-10T10-27-16Z-wrap/
