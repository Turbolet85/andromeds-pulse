## 2026-11-03-stdio-sidecar-retired — the tray menu without the MCP toggle
**Section:** §Surface: desktop-native → Component Patterns → Tray Menu (Actions, Tone); §Surface: desktop-native → Navigation Pattern (step 2) (design-system body)
**Change:**
- Tray Menu Actions: was "Generate Snapshot", "Toggle MCP Server" (only when built with `--features mcp-server`; checkbox-marked if enabled; invokes `mcp.start` / `mcp.stop`), "Open Settings", divider, "Quit"; now "Generate Snapshot" (invokes `snapshot.generate` TauRPC, shows "Snapshot ready" notification on completion), "Open Settings" (focuses the Settings panel in the app), divider, "Quit" (terminates the process; tray icon disappears).
- Tray Menu Tone: the contemplative, observational voice rule stands with one example, "Generate Snapshot" not "Export Data"; was also "Toggle MCP Server" not "Enable AI Integration".
- Navigation Pattern step 2: was "Right-click tray icon → menu: Generate Snapshot, Toggle MCP, Quit, etc."; now "Right-click tray icon → menu: Generate Snapshot, Open Settings, Quit, etc."
**Why:** the stdio sidecar retired with its double gate (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`), its `mcp.*` TauRPC procedures (`mcp.status`, `mcp.start`, `mcp.stop`) and the tray MCP toggle; `MENU_ID_MCP_TOGGLE`, the "Toggle MCP Server" check item and its handler are removed, the cargo feature `mcp-server` is gone, and the tray menu is the same in every build.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the notification triggers lose the MCP status change
**Section:** §Surface: desktop-native → Component Patterns → Notifications (OS-native) (Trigger) (design-system body)
**Change:**
- Notifications Trigger: was "snapshot generation completion, MCP server status change, update available"; now "snapshot generation completion, update available".
**Why:** a wording correction: the "MCP server status change" notification was never built and had no emitter before this chunk; the server it would have reported on left with the stdio sidecar.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/
