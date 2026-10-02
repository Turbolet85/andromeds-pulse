
## 2026-09-30-perf-instruments-measure-their-budgets — perf-budget frame line names a cause
**Section:** §Occupied Resources → xtask CLI surfaces (`perf:budget` · `perf:frame-sample`)
**Change:** An empty frame arm names its cause via `xtask::perf_budget::frame_cause` over the same log's `ui.webgpu.adapter` records: unrequired → `frame: cannot-evaluate: 0 samples, {cause}`; `--require`d → `frame NEUTRAL — {cause} (required) FAIL`; `perf:frame-sample` 0-sample → `frame: 0 samples — {cause}`. Exit codes and the nearest-rank rule unchanged. Was: the fixed `frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run`.
**Why:** the registered CLI contract stated retired output text.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
