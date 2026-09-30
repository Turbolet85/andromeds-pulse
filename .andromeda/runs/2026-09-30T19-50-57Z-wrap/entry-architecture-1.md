
## 2026-09-30-perf-instruments-measure-their-budgets — telemetry.frontend.record_webgpu_adapter registered
**Section:** §Occupied Resources → Tauri IPC routes
**Change:** NEW `telemetry.frontend.record_webgpu_adapter` — the 6th `TelemetryApi` method (roster 5 → 6): `WebgpuAdapterInput { outcome: WebgpuAdapterOutcome (closed serde enum of 5; unknown rejected), window_label (coerced 4 + unknown) }` → `ui.webgpu.adapter` {outcome, window_label} behind its own exact leaf; no capability-JSON change; `EXPECTED_PROCEDURES` 43 → 44; its only live witness the dev-host frame leg. The body quotes the founder's ratification at P4 2026-09-30, «Да, делай».
**Why:** every added TauRPC procedure needs its Occupied Resources entry; this one is a Boundary widening, ratified by the founder at P4.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
