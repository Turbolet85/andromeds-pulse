## 2026-11-03-stdio-sidecar-retired — the Settings modal without the MCP switch
**Section:** §1 A11y Scope Summary (the Settings modal form entity row); CI Integration (the P7 critical path row); §5 Keyboard Navigation (the settings-modal focus order)
**Change:**
- The Settings modal form row's controls read theme selector, widget snap position, retention input, snapshot preset, plugin manager (was: an MCP toggle between the retention input and the snapshot preset).
- The P7 row reads "Settings modal form submission (theme, widget position, retention, snapshot preset)" (was: with the MCP toggle); its required roles read `dialog[aria-label="Settings"]` / `form` / `label` / `button` (Save / Cancel) / `radiogroup` (theme / widget position) (was: also `switch` (MCP toggle)); its Tab order reads theme radio → widget position radio → retention input → snapshot preset → Save button → Cancel button (was: an MCP toggle switch after the retention input). Its Esc, invalid-form focus and SC criteria stand.
- The settings-modal focus order reads modal title → theme radio group → widget position radio group → retention input → snapshot preset combobox → Save button → Cancel button (was: an MCP toggle switch after the retention input); Esc closes the modal, focus returns to the "Settings" button.
**Why:** the chunk retired the Settings MCP switch with the stdio sidecar and removed `Settings.mcp_server_enabled`; the Settings modal loses that control, the a11y focus order now runs without the switch (the a11y list and CI a11y green).
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the P3 path reaches the engine through the door, no UI surface
**Section:** CI Integration (the P3 critical path row)
**Change:**
- The P3 row reads "MCP tools reach the engine through the door (socket and stdio relay)" (was: "MCP server toggle in settings → notification on state change").
- Its surface reads none — the door has no UI surface; not-assertable and omitted (was: desktop-webview settings-modal plus the os-notification surface, the MCP server backend omitted).
- Its roles, keyboard and SC columns read N/A — no UI surface, the settings-modal focus order being P7's (was: `switch[aria-checked]` MCP toggle and the OS notification; Tab / Space on the switch; SC 2.1.1, 2.4.3, 4.1.2, 4.1.3).
- Its source reads Tests excerpt Critical Path P3 alone (was: also the Creator Brief "MCP server toggle" setting and the Obs Service Identity mcp-server sidecar identity).
**Why:** the chunk retired the stdio sidecar, its double gate and the Settings MCP switch; the engine's one way in besides telemetry is now the door (socket and stdio relay); the MCP server status change notification never had an emitter.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/
