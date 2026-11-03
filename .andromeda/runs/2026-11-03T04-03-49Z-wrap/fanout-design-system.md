proposals:
  - detector: D-retired
    severity: warning
    section: §Surface: desktop-native → Component Patterns → Tray Menu
    change: Remove the "Toggle MCP Server" action item (its `--features mcp-server` condition, checkbox state and `mcp.start` / `mcp.stop` invocation); the Actions list reads "Generate Snapshot", "Open Settings", divider, "Quit" — the tray menu is Open · separator · summary · separator · Snapshot · Open Settings · separator · Quit in every build.
    sidecar: 2026-11-03-stdio-sidecar-retired — Tray Menu loses the retired "Toggle MCP Server" item (feature, procedures and toggle removed with the stdio sidecar); menu structure restated as shipped.
    rationale: The report's Retired bullet names the tray MCP toggle, the `mcp.*` TauRPC procedures and the double gate; Symbols / APIs records `MENU_ID_MCP_TOGGLE`, the check item and its handler removed and the menu as Open · separator · summary · separator · Snapshot · Open Settings · separator · Quit in every build; Schema / config removes the `mcp-server` cargo feature.
    basis: design-system fct-dzpd3ztt; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx
  - detector: D-retired
    severity: warning
    section: §Surface: desktop-native → Component Patterns → Tray Menu
    change: The Tone line keeps its voice rule with a standing label only — "Generate Snapshot" not "Export Data" — dropping the "Toggle MCP Server" not "Enable AI Integration" example, which names the retired toggle as a shipped label.
    sidecar: 2026-11-03-stdio-sidecar-retired — Tray Menu tone example no longer cites the retired "Toggle MCP Server" label.
    rationale: The report's Retired bullet names the tray MCP toggle; Symbols / APIs records the "Toggle MCP Server" check item removed. The tone line restates the toggle's label as a standing menu item.
    basis: design-system fct-syzuo4in; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Surface: desktop-native → Navigation Pattern
    change: Item 2 reads "Right-click tray icon → menu: Generate Snapshot, Open Settings, Quit, etc." — no "Toggle MCP".
    sidecar: 2026-11-03-stdio-sidecar-retired — tray Navigation Pattern no longer lists the retired MCP toggle.
    rationale: The report's Retired bullet names the tray MCP toggle; Symbols / APIs gives the tray menu as Open · summary · Snapshot · Open Settings · Quit in every build. This navigation step restates the toggle as a menu item.
    basis: design-system fct-5yxgeqtw; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Surface: desktop-native → Component Patterns → Notifications (OS-native)
    change: Trigger reads "snapshot generation completion, update available." — the "MCP server status change" trigger is dropped (it never had an emitter and its server left with the stdio sidecar).
    sidecar: 2026-11-03-stdio-sidecar-retired — Notifications trigger list drops the never-built "MCP server status change" (wording correction; the sidecar it would report on is retired).
    rationale: The report's Retired bullet states the never-built "MCP server status change" notification (design-system, layout-templates) had no emitter before this chunk and calls for a wording correction; Expected amendments lists design-system Notifications (wording), `status-notif`.
    basis: design-system fct-o6tiiqzk; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-vikzxbdd
note: D-design-tokens and D-design-status-narrative find no drift — the report's Coverage says no UI element was added (tokens n/a), Spec-master-edits is none, and the disproved spec claims (a11y mock helper, the TraceRow marker, the scope guard) reach no surface whose status this doc states. No design-system fact names the Settings MCP switch or Send to agent (the report's `send-agent` control never fired).
