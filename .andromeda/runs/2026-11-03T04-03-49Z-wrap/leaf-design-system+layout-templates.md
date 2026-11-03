group: design-system+layout-templates
read: .claude/docs/design-summary.md · 96 · «Full plan: `.andromeda/design-system.md` + `.andromeda/layout»

- leaf: .claude/docs/design-summary.md:77   class: retired
  leaf-text: «retention seconds (300–600) / MCP toggle (visible only when the `mcp-server` feature is built).»
  source: fct-wwnwzbqm «- **Settings panel** — theme (dark/light/auto), widget snap position, retention duration.»
  replace-with: «retention seconds (300–600).»

- leaf: .claude/docs/design-summary.md:84   class: retired
  leaf-text: «Generate Snapshot / Toggle MCP Server (when feature built) / Open Settings / Quit.»
  source: fct-4he46uzc «Actions: open / focus main window, generate snapshot, open settings, quit.»
  replace-with: «Generate Snapshot / Open Settings / Quit.»

- leaf: .claude/docs/design-summary.md:85   class: retired
  leaf-text: «snapshot ready, MCP server state change, update available, and the close-to-tray signpost»
  source: fct-jonk5unv «- **Notifications (OS-native)** — toasts fired on snapshot generation completion, updater state transitions, and the window-close-to-tray signpost»
  replace-with: «snapshot ready, update available, and the close-to-tray signpost»
