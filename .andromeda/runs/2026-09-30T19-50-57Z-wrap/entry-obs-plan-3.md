
## 2026-09-30-perf-instruments-measure-their-budgets — ui.webgpu.adapter registered dual-site
**Section:** §1 multi-platform-exporter-compat row · §6 warn row · §8 leaves
**Change:** NEW target `ui.webgpu.adapter` {`outcome`, `window_label`}: once per canvas-mount adapter request (never per frame), INFO on `obtained`, WARN on `no_navigator_gpu` | `adapter_null` | `adapter_request_rejected` | `device_request_failed`; reported fire-and-forget through `telemetry.frontend.record_webgpu_adapter`. An EXACT §8 leaf beside `ui.ipc.rejection`, no bare `ui` key, guarded ×3 under `pulse-app/tests/`; its only live witness is the dev-host frame leg. The body quotes the founder's ratification at P4 2026-09-30, «Да, делай». Was (§1): "no truthful adapter-state record exists yet", with a route-entry owner.
**Why:** the webview is the only place the adapter outcome is known, so it crosses the bridge as a closed enum + coerced label and lands behind its own leaf (both fields the emit site sends). The crossing is a Boundary widening, ratified by the founder at P4.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
