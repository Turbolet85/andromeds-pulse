
## 2026-09-30-perf-instruments-measure-their-budgets — perf arm coverage + frame row reflect the cause-derived line
**Section:** §1 perf-slo-check-arm-coverage row · §1 performance-budget: WebGPU canvas throughput row
**Change:** `xtask::perf_budget` is pinned by 21 unit tests (6 `frame_cause_*`); the frame arm's 0-sample line names a cause from the log's `ui.webgpu.adapter` records and reads `… no adapter record in this log` on CI (`ci#36765040464`); the snapshot value spans the whole generation (`GenerationTimer`, `generation_timer_*` ×5). Was: 15 pins, the fixed `no WebGPU adapter in this run`, and "the snapshot value times formatting only".
**Why:** the rows stated the retired frame text and snapshot scope.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
