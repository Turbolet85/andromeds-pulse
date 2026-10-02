
## 2026-09-30-perf-budget-gate-reads-real-samples — perf:budget and perf:frame-sample; the frame leg's env var; release owns its cache
**Section:** §Occupied Resources → xtask CLI surfaces · §Occupied Resources → Environment variables · §Infrastructure Patterns → CI/CD approach
**Change:**
- Registered `cargo xtask perf:budget --data-dir <DIR> --require <arm,arm>` (grades every `agent-latest.jsonl*` member; nearest-rank p99; Unreadable or required-empty fails; exit 0/1/2; a non-required empty frame arm prints the named cannot-evaluate line). `ci-gates` and `perf:load-profiles` use it in-process with no arm required; the `perf-slo-check` scripts are deleted.
- Registered `cargo xtask perf:frame-sample` — Windows dev-host frame gate, exit 0 PASS / 1 FAIL / 2 INCONCLUSIVE, artifact `target/perf-frame/`, NOT CI-wired.
- Env var `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`: harness-only, SET on the frame leg's app child only, never read by the product, never in product config.
- CI/CD: lint-test Linux adds the perf-samples producer → `perf:budget --require memory,snapshot` → upload; `release` owns `release-{os}` (was a read-only restore of lint-test's key) and has no frame boot step; 8 cache entries, 10 605 172 169 of the 10 737 418 240 B cap (a watch).
**Why:** new CLI verbs and an env var are registry resources; the frame leg left CI after the hosted runner exposed no WebGPU adapter (operator decision). Release was going cold whenever lint-test re-saved without release dependencies.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/
