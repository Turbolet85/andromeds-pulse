
## 2026-09-30-perf-instruments-measure-their-budgets — record_webgpu_adapter validated input
**Section:** §Input Validation → TauRPC bridge row
**Change:** `telemetry.frontend.record_webgpu_adapter` validates `outcome` as the closed serde `snake_case` enum `WebgpuAdapterOutcome` (5 values; an unknown value is rejected at deserialization, pinned by name) and coerces `window_label` through `coerce_window_label` (4 + `unknown`); pinned in `EXPECTED_PROCEDURES` (44). The body quotes the founder's ratification at P4 2026-09-30, «Да, делай».
**Why:** a new webview → Rust input class; it is a Boundary widening, ratified by the founder at P4, and it is validated at the boundary.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
