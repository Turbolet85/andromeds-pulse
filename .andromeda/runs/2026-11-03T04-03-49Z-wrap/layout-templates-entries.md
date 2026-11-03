## 2026-11-03-stdio-sidecar-retired — the Settings MCP server toggle retired
**Section:** §Surface: desktop-webview → Primary screens (the Settings panel); Component — Primary navigation (desktop-webview) (the Settings entry); Component — Settings modal (form) (the form controls)
**Change:**
- Primary screens: the Settings panel reads theme (dark/light/auto), widget snap position, retention duration (was … retention duration, MCP server toggle (when feature is built)).
- Primary navigation: the Settings entry reads theme, widget snap position, retention (was … retention, MCP toggle).
- Settings modal (form): the MCP server toggle bullet retired (was a checkbox visible only when the `mcp-server` Cargo feature is built, label "Enable MCP Server", helper text "Exposes query tools to Claude Code and other MCP clients."); the modal names no MCP control.
**Why:** the chunk retired the Settings MCP switch: `Settings.mcp_server_enabled` and the `mcp-server` cargo feature are removed, and the design/layouts criterion (no Settings switch) is met.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the tray menu's MCP toggle retired
**Section:** §Surface: desktop-native → Primary screens (the Tray menu); Component — Tray menu (OS-native) (action items, implementation details, tone); Wireframe — Tray menu (right-click context); IA notes
**Change:**
- Primary screens: the tray menu's actions read open / focus main window, generate snapshot, open settings, quit (was … generate snapshot, toggle MCP server (when feature is built), open settings, quit).
- Component — Tray menu: the action items read "Generate Snapshot", "Open Settings" (was "Generate Snapshot", "[✓] Toggle MCP Server" (when feature built), "Open Settings").
- Component — Tray menu: menu item selection fires Tauri IPC commands, e.g. `snapshot.generate` (was `snapshot.generate`, `mcp.start` / `mcp.stop`).
- Component — Tray menu: the tone bullet "Toggle MCP Server" — not "Enable AI Integration" or "Start LLM Mode" retired.
- Wireframe: the body retired one row of the tray-menu wireframe, the `[✓] Toggle MCP Server` row (checkbox toggle, visible when the mcp-server feature built); the wireframe block stands, Generate Snapshot now followed by Open Settings.
- IA notes: the menu actions are limited to "open app", "generate snapshot", "open settings", "quit" (was … "generate snapshot", "toggle MCP", "open settings", "quit"); deep navigation stays inside the app window.
**Why:** the chunk retired the tray MCP toggle (`MENU_ID_MCP_TOGGLE`, the Toggle MCP Server check item and its handler) and the `mcp.status` / `mcp.start` / `mcp.stop` TauRPC procedures; the tray menu is Open · separator · summary · separator · Snapshot · Open Settings · separator · Quit, in every build.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the MCP server status notification, a wording correction
**Section:** §Surface: desktop-native → Primary screens (Notifications (OS-native)); Component — Notifications (OS-native) (triggers, title and body examples)
**Change:**
- Primary screens: toasts fire on snapshot generation completion, updater state transitions, and the window-close-to-tray signpost (P-063) (was … snapshot generation completion, MCP server state changes, updater state transitions …).
- Component — Notifications: trigger 2, MCP server status change (start / stop, when feature is built), retired.
- Component — Notifications: the title examples read "Snapshot ready", "Update available" (was … "MCP Server started" …); the body example reads "2.5k tokens, 42 spans" (was Examples: … "Ready to accept Claude Code connections").
**Why:** the never-built "MCP server status change" notification had no emitter before this chunk — a wording correction; the stdio sidecar it would announce is retired.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/
