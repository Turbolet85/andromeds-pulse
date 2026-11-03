proposals:
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-webview → Component — Settings modal (form)'
    change: 'Remove the "MCP server toggle" form-control bullet (checkbox gated on the mcp-server Cargo feature, label "Enable MCP Server", helper text on Claude Code / MCP clients); the form controls are theme, widget snap position and retention seconds, in every build.'
    sidecar: 'Settings modal: the MCP server toggle control removed (Settings.mcp_server_enabled and the mcp-server cargo feature retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'The report Retired bullet names the Settings MCP switch; Schema / config records Settings.mcp_server_enabled and the mcp-server cargo feature removed; the design/layouts outcome reads no Settings switch, met.'
    basis: 'layout-templates fct-m5cqrktj; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-xsrenprx, fct-dozpu5cz; pulse-app/ui/src/dashboard/routes/SettingsModalForm.tsx'
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-webview → Primary screens'
    change: 'Settings panel bullet reads: theme (dark/light/auto), widget snap position, retention duration — drop "MCP server toggle (when feature is built)".'
    sidecar: 'Primary screens (webview): the Settings panel no longer lists an MCP server toggle (retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the retired Settings MCP switch (report Retired bullet; Schema / config: Settings.mcp_server_enabled removed).'
    basis: 'layout-templates fct-wwnwzbqm; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-xsrenprx'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-webview → Component — Primary navigation (desktop-webview)'
    change: 'The Settings tab entry reads: theme, widget snap position, retention — drop "MCP toggle".'
    sidecar: 'Primary navigation: the Settings entry no longer lists an MCP toggle (retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the retired Settings MCP switch (report Retired bullet; Schema / config: Settings.mcp_server_enabled removed).'
    basis: 'layout-templates fct-drj7tbkq; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-xsrenprx'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Component — Tray menu (OS-native)'
    change: 'Action items read "Generate Snapshot", "Open Settings" — drop "[✓] Toggle MCP Server" (when feature built); the menu is Open · divider · summary · divider · Generate Snapshot · Open Settings · divider · Quit, in every build.'
    sidecar: 'Tray menu: the Toggle MCP Server check item removed (MENU_ID_MCP_TOGGLE and its handler retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'The report Retired bullet names the tray MCP toggle; Symbols / APIs records MENU_ID_MCP_TOGGLE, the Toggle MCP Server check item and its handler removed, the tray menu Open · separator · summary · separator · Snapshot · Open Settings · separator · Quit in every build.'
    basis: 'layout-templates fct-qftsgrxw; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc; pulse-app/src/tray.rs'
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'Tray menu bullet actions read: open / focus main window, generate snapshot, open settings, quit — drop "toggle MCP server (when feature is built)".'
    sidecar: 'Primary screens (native): the tray menu no longer lists an MCP server toggle (retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the retired tray MCP toggle (report Retired bullet; Symbols / APIs: the tray menu in every build).'
    basis: 'layout-templates fct-4he46uzc; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Wireframe — Tray menu (right-click context)'
    change: 'Remove the "[✓] Toggle MCP Server  <- checkbox toggle (visible when mcp-server feature built)" row; Generate Snapshot is followed directly by Open Settings.'
    sidecar: 'Tray menu wireframe: the Toggle MCP Server row removed (retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the retired tray MCP toggle (report Retired bullet; Symbols / APIs: tray menu Open · separator · summary · separator · Snapshot · Open Settings · separator · Quit); no governing sketch-lag note covers this wireframe.'
    basis: 'layout-templates fct-zrtafiai; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Component — Tray menu (OS-native)'
    change: 'Implementation-details bullet reads: menu item selection fires Tauri IPC commands (e.g., `snapshot.generate`) — drop `mcp.start` / `mcp.stop`.'
    sidecar: 'Tray menu implementation: the mcp.start / mcp.stop IPC example removed (mcp.* TauRPC procedures retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'The report Retired bullet names the mcp.* TauRPC procedures; Symbols / APIs records mcp.status, mcp.start, mcp.stop removed with their router and bindings.'
    basis: 'layout-templates fct-boykhivb; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Component — Tray menu (OS-native)'
    change: 'Remove the tone bullet "Toggle MCP Server" — not "Enable AI Integration" or "Start LLM Mode"; the tone examples are Generate Snapshot and Open Settings.'
    sidecar: 'Tray menu tone: the Toggle MCP Server label example removed (the item is retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the retired tray MCP toggle as a standing menu label (report Retired bullet; Symbols / APIs).'
    basis: 'layout-templates fct-732ketbq; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → IA notes'
    change: 'Menu actions are limited to "open app", "generate snapshot", "open settings", "quit" — drop "toggle MCP".'
    sidecar: 'Tray IA notes: the toggle MCP action removed from the menu-action list (retired, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the retired tray MCP toggle (report Retired bullet; Symbols / APIs: the tray menu in every build).'
    basis: 'layout-templates fct-fb5svsti; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Component — Notifications (OS-native)'
    change: 'Remove trigger 2 (MCP server status change, start / stop, when feature is built); the triggers read snapshot generation completion, update available, window-close to tray.'
    sidecar: 'Notifications: the never-built MCP server status-change trigger removed — a wording correction; the sidecar it would announce is retired (chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'The report Retired bullet: the never-built "MCP server status change" notification (design-system, layout-templates) had no emitter before this chunk — a wording correction; the stdio sidecar it would announce is retired.'
    basis: 'layout-templates fct-px726b3i; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y'
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'Notifications bullet reads: toasts fired on snapshot generation completion, updater state transitions, and the window-close-to-tray signpost — drop "MCP server state changes".'
    sidecar: 'Primary screens (native): MCP server state changes dropped from the notification list (wording correction, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the never-built MCP status notification the report Retired bullet corrects.'
    basis: 'layout-templates fct-jonk5unv; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Component — Notifications (OS-native)'
    change: 'Title examples read "Snapshot ready", "Update available" — drop "MCP Server started".'
    sidecar: 'Notification format: the MCP Server started title example removed (wording correction, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the never-built MCP status notification the report Retired bullet corrects.'
    basis: 'layout-templates fct-yh3haupv; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Component — Notifications (OS-native)'
    change: 'Body examples read "2.5k tokens, 42 spans" — drop "Ready to accept Claude Code connections".'
    sidecar: 'Notification format: the Claude Code connections body example removed (wording correction, chunk 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Restates the never-built MCP status notification (its body text) the report Retired bullet corrects.'
    basis: 'layout-templates fct-z3fbkvvx; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y'
    dependent-of: D-retired
note: 'D-layout-surface and D-layout-status-narrative fire nothing: the report adds no UI surface (Coverage: no element added), its Spec-master edits are none, and its three disproved claims touch no master site and no layout status. The Report window bullet names no Send to agent control (send-agent control unfired).'
