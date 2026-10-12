
## 2026-10-09-pre-push-check-native-on-linux — the boot job's ci-gates frame line is printed only when the smoke step passes
**Section:** §10 SLO Invariants & Telemetry Budgets (the WebGPU canvas frame row · the CI gates bullet "Build fails if a perf-budget arm exceeds its SLO or cannot be read")
**Change:**
- Frame row: the step's order makes `frame: cannot-evaluate: 0 samples, no WebGPU adapter (no_navigator_gpu)` the expected `ci-gates` line on every run whose smoke step passes (the settle verdict reads `settled`); was "on every run that reaches the settle verdict". On a run whose smoke step fails the job skips its `ci-gates` step and prints no such line, though the log holds the records: as measured on `ci#38010977166`, settle verdict `ended`, the step `skipped`, 2 `ui.webgpu.adapter` records, both `no_navigator_gpu`. The boot job's LOG stays a witness of the WARN arm on a run that reaches the settle verdict.
- CI gates bullet: the boot-smoke line reads `… no WebGPU adapter (no_navigator_gpu)` on a run whose smoke step passes and is not printed on a run whose smoke step fails; was "on a run that reaches the settle verdict".
**Why:** Measured false on the first run whose settle verdict read `ended`: the failed smoke step makes the job skip the step that prints the line. The log-side claims held on that run and stand.
**Kept:** the statements of §1's `multi-platform-exporter-compat` trigger row and of §PII Scrubbing's `ui.webgpu.adapter` bullet that the boot job's log holds the `ui.webgpu.adapter` record on every run that reaches the settle verdict (2 records on the red run).
**Ref:** .andromeda/runs/2026-10-10T01-19-03Z-wrap/
