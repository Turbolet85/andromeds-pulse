group: a11y-plan
read: .claude/docs/a11y-summary.md · 109 · «Full plan: `.andromeda/a11y-plan.md`.»
read: .claude/rules/a11y.md · 125 · «- 2026-08-26: ESLINT'S `--rule` CLI FLAG CANNOT CARRY A PLUG»

- leaf: .claude/docs/a11y-summary.md:28   class: retired
  leaf-text: «**P3** MCP toggle in settings | `dialog[aria-label="Settings"]` / `switch[aria-checked]` + OS notification | Tab to MCP switch → Space toggles → focus stays on switch | 2.1.1 / 2.4.3 / 4.1.2 / 4.1.3»
  source: fct-ggne23uy «| **P3: MCP tools reach the engine through the door (socket and stdio relay)** | none — the door has no UI surface; not-assertable and omitted | N/A — no UI surface |»
  replace-with: «**P3** MCP tools reach the engine through the door (socket and stdio relay) | N/A — the door has no UI surface (not-assertable, omitted) | N/A — no UI surface; the settings-modal focus order is P7's | N/A — no UI surface»

- leaf: .claude/docs/a11y-summary.md:32   class: stale
  leaf-text: «`dialog` / `form` / `label` / `radiogroup` / `switch` / `button` | Tab theme→position→retention→MCP→preset→Save→Cancel»
  source: fct-v344ogev «`radiogroup` (theme / widget position) | Initial focus: first form control (theme selector) or modal title. Tab: theme radio → widget position radio → retention input → snapshot preset → Save button»
  replace-with: «`dialog` / `form` / `label` / `radiogroup` / `button` | Tab theme→position→retention→preset→Save→Cancel»

- leaf: .claude/rules/a11y.md:73   class: retired
  leaf-text: «- **P3** Settings modal MCP toggle: `<switch aria-checked>`, focus stays on switch during state change, OS notification on completion.»
  source: fct-ggne23uy «| **P3: MCP tools reach the engine through the door (socket and stdio relay)** | none — the door has no UI surface; not-assertable and omitted | N/A — no UI surface |»
  replace-with: «- **P3** MCP tools reach the engine through the door (socket and stdio relay): no UI surface — not-assertable and omitted; the settings-modal focus order is P7's.»

- leaf: .claude/rules/a11y.md:77   class: stale
  leaf-text: «/ `<radiogroup>` / `<switch>` / Tab cycle theme→position→retention→MCP→preset→Save→Cancel»
  source: fct-v344ogev «`radiogroup` (theme / widget position) | Initial focus: first form control (theme selector) or modal title. Tab: theme radio → widget position radio → retention input → snapshot preset → Save button»
  replace-with: «/ `<radiogroup>` / Tab cycle theme→position→retention→preset→Save→Cancel»
