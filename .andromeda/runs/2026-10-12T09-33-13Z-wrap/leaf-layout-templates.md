group: layout-templates
read: .claude/docs/design-summary.md · 96 · «Full plan: `.andromeda/design-system.md` + `.andromeda/layou»
read: .claude/rules/frontend.md · 139 · «- 2026-09-29: UNDER vitest 4, `vi.restoreAllMocks()` NO LONG»
read: CLAUDE.md · 156 · «<!-- USER:session-learnings end -->»

- leaf: .claude/rules/frontend.md:66   class: stale
  leaf-text: «Always handle loading / error / empty states explicitly (skeleton component for loading; `font-body` muted message for empty; `--color-accent` text + icon for error).»
  source: .andromeda/layout-templates.md:175 «a static message ("Couldn't load {metrics|logs}", never the raw `AppError`) with NO exporter hint, so a failure never masquerades as "no data". `color-text-secondary`; `color-accent` (Alert Burgu»
  replace-with: «Always handle loading / error / empty states explicitly, error checked FIRST (skeleton component for loading; empty — the shared `EmptyState`: a `font-body` message + the OTLP-port hint (`:4318` / `:4317`) in `color-text-secondary`, never `color-text-tertiary`, shown only after a settled zero-data load; error — a static "Couldn't load {metrics|logs}" message in `color-text-secondary`, no exporter hint, never the raw `AppError`, with `--color-accent` only on an optional icon / border).»

- leaf: .claude/docs/design-summary.md:77   class: stale
  leaf-text: «**Settings modal:** theme / widget snap position / retention / MCP toggle / snapshot preset / plugin manager.»
  source: .andromeda/layout-templates.md:32 «**Settings panel** — theme (dark/light/auto), widget snap position, retention duration, MCP server toggle (when feature is built).»
  replace-with: «**Settings modal:** theme (dark / light / auto) / widget snap position (four corners, default top-right) / retention seconds (300–600) / MCP toggle (visible only when the `mcp-server` feature is built).»

- leaf: .claude/docs/design-summary.md:78   class: omission
  leaf-text: «**Investigation modal:** triggered by Investigate button (telescope glyph) or trace row click; Capture Collapse motion 350ms.»
  source: .andromeda/layout-templates.md:179 «clicking the titlebar "Investigate" button (telescope icon), clicking a trace row, or pressing **Enter on the keyboard-focused row** — the in-row telescope button is no longer reachable by Tab»
  replace-with: «**Investigation modal:** triggered by the titlebar Investigate button (telescope glyph), a trace row click, or Enter on the keyboard-focused trace row (the in-row telescope button is `tabIndex={-1}`, not a Tab stop); Capture Collapse motion 350ms.»

- leaf: .claude/docs/design-summary.md:75   class: omission
  leaf-text: «always-on-top toggle, single-window. Esc minimizes to tray.»
  source: .andromeda/layout-templates.md:33 «**Findings window** — separate borderless always-on-top disclosure window docked directly below the compact widget (positioned from the widget's live geometry, work-area/multi-monitor clamped), opened»
  replace-with: «always-on-top toggle, single-window. Esc minimizes to tray. Its unread badge opens the **Findings window** (a separate borderless always-on-top window docked below the widget, work-area clamped, sized to the incident count — ~8 rows, then the list scrolls — on `--color-raised-2`); selecting a findings row opens the **Report window** (the six-section Report in the `Modal` `fill` variant, edge-to-edge on `--color-base`, no backdrop, left of the findings window).»

- leaf: .claude/docs/design-summary.md:85   class: omission
  leaf-text: «**OS notifications:** "Snapshot ready ({N} tokens). Paste in {AI tool} to investigate." — 2 lines max via `tauri-plugin-notification`.»
  source: .andromeda/layout-templates.md:341 «Window-close to tray (the predictable-close signpost): fires on EVERY close that sends the app to the tray — the compact-widget ✕; the dashboard ✕ merely collapses to the widget and never signposts»
  replace-with: «**OS notifications:** snapshot ready, MCP server state change, update available, and the close-to-tray signpost (P-063: every close that sends the app to the tray while `notifications_enabled` is true; the dashboard ✕ collapses to the widget and never signposts) — terse title ("Snapshot ready") + a body of 2 lines max ("2.5k tokens, 42 spans") via `tauri-plugin-notification`; opt-out `notifications_enabled` in `config.toml` (default true).»
