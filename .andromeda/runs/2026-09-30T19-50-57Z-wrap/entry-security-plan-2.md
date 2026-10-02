
## 2026-09-30-perf-instruments-measure-their-budgets — ui.webgpu.adapter no-scrub log boundary
**Section:** §Security Anti-Patterns → Logging
**Change:** NEW deliberate NO-SCRUB webview-originated log boundary `ui.webgpu.adapter`: one record per canvas-mount adapter request (INFO on `obtained`, WARN otherwise) behind its own exact leaf `{outcome, window_label}` beside `ui.ipc.rejection`; nothing client-controlled or free-text crosses it (closed enum + coerced label; the raw adapter / error text excluded); still no bare `ui` key; pinned ×3; its only live witness the dev-host frame leg — no CI job witnesses it. The body quotes the founder's ratification at P4 2026-09-30, «Да, делай».
**Why:** the Logging section enumerates every deliberate no-scrub boundary; this is the second webview-originated one, a Boundary widening ratified by the founder at P4.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
