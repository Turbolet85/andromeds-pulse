# layouts extract — phase-80

## No domain coverage

Chunk #83 "Prompt scaffolding + JSON schema + primary tier inference" is entirely out of the layouts domain.

**Reasoning:**

- Chunk scope per pulse-v0_2_0-route §83 + epoch context is backend-only: system prompt construction + embedded JSON schema + JSON-constrained inference inside `crates/interpretation/`. Output is consumed by the existing `pulse://stream/incidents` broadcast topic from chunk #78.
- Crates touched: `crates/interpretation/` (prompt + inference modules) — backend Rust only. No webview source files, no React components, no canvas/shader work, no native window or tray changes.
- Surface delta vs. layout-templates.md surfaces (desktop-webview + desktop-native): zero. No modifications к compact widget, full dashboard (Traces / Metrics / Logs / Snapshots / Settings views), titlebar, Halo State Pulse canvas, navigation, data table, Investigation modal, Settings modal, footer, tray icon, tray menu, or OS notifications.
- No new focusable interactive elements introduced (no focus order delta).
- No new modal / dialog / sheet / drawer (no modal pattern binding).
- No responsive layout work (chunk is non-UI).
- No empty-state design surface (no user-facing data list created by this chunk).
- Epoch context explicitly states: "Report UI surface (chunk #87), findings counter / dropdown (chunk #86), and Halo refactor (chunk #89) consume #83's output but are future chunks." Layout consequences of #83's output land at those future chunks, not at #83 itself.
- Specialist plan touches enumerated в context: test-plan (schema validation) + obs-plan (3 metrics). Design / layouts / a11y explicitly NOT listed.

Layout-templates.md contributes no constraints, patterns, anti-patterns, contract bindings, or acceptance criteria к this chunk. The layouts domain re-engages при chunk #86 (findings dropdown placement в Compact widget / Full dashboard surfaces) and chunk #87 (Report UI surface — new wireframe required).
