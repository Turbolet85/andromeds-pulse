
## 2026-09-29-p-025-hue-shift-observable-made-gradable — the CI perf-budget gates are recorded VACUOUS, owned
**Section:** §1 Telemetry triggers → perf-budget-instruments (WebGPU row) · §10 Performance budgets (WebGPU frame row; Buffer memory row) · §10 CI gates (perf-budget p99 bullet; snapshot p99 bullet)
**Change:** Each site keeps its intended gate and now states that it is measured VACUOUS in CI: the `ci-gates` step feeds `perf-slo-check` only the boot-smoke log (52 records, 0 frame / memory / snapshot samples), so it reads NEUTRAL and cannot fail, and the p99 SLOs are not CI-enforced today. Owner at every site: the working-route entry "Perf-budget gate reads real samples".
**Why:** measured at the chunk's first real CI run; recorded as an OPEN defect with a named owner (playbook 2026-08-28) rather than leaving the sections reading as enforced.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
