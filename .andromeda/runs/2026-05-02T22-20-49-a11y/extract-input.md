## 7. Creator Brief Excerpt

### Must-Work Scenarios

- "**Quarter-screen widget mode** — default surface; window snaps к side of screen (left / right / corner); always-on-top toggle; remembers position per display." (compact glance-monitor; primary surface — must-be-accessible flow)
- "**Glance-readable от 2 meters** — typography legible at distance; high-contrast palette; motion convey state (pulse / flow / steady) без requiring focused attention." (distance-legibility flow — sets contrast / typography accessibility expectations)
- "**Click к expand** — opens full dashboard window; widget remains mounted (returns к compact mode on close)." (compact → expanded transition — focus management between surfaces)
- "**Tray icon (secondary)** — traffic-light status; click cycles widget visibility (visible / minimized / hidden)." (tray flow — keyboard/menu accessibility)
- "'Investigate' button on widget + main window + context menu." + "Generates **token-efficient curated snapshot** (not raw OTLP dump)." (Investigate workflow CORE feature — primary user-initiated flow that must be keyboard-reachable)
- "**Notification** — `Snapshot ready ({N} tokens). Paste in {AI tool} to investigate.`" (OS notification flow on snapshot completion — content must be screen-reader accessible)
- "AI agents query telemetry via MCP `tools/call` requests" (MCP server flow — agent-driven, no a11y surface itself but must not affect UI focus/state)
- Settings flows: "Buffer size / retention window", "Snapshot preset template (Claude Code / Cursor / ChatGPT / Custom)", "Theme (auto / light / dark)", "Widget mode (compact / hidden / disabled)", "Widget snap position", "MCP server toggle", "Plugin manager" (settings modal — form controls must be label-associated and keyboard-navigable)

### Rigor Hints

- "**Color:** dark + light themes. Status colors not-color-alone (paired с iconography per WCAG)." (explicit WCAG reference; signals SC 1.4.1 Use of Color discipline; WCAG mention without specifying conformance level — Standard tier WCAG 2.1 AA is a reasonable default mapping)
- "**Motion:** every state change is а transition; respects `prefers-reduced-motion` для accessibility (degrades к instant); motion-as-data principle (motion reflects telemetry character, не decoration)." (explicit accessibility callout; SC 2.3.3 Animation from Interactions / SC 2.3.1 Three Flashes discipline implied)
- "**Glance-readable от 2 meters** — typography legible at distance; high-contrast palette" (high-contrast palette commitment; aligns to SC 1.4.3 minimum, possibly SC 1.4.6 enhanced for the always-visible widget surface)
- "**Aesthetic stance:** quiet ambient telemetry presence. Not a noisy dashboard demanding attention; a glanceable surface that conveys state through motion and color quality." (signals motion-budget discipline; reduced-motion override is a first-class concern)
- "**Development Style:** agent-driven (built via Andromeda v2 pipeline — recursive dogfood validates our OTel mandate on its own creator tool)." (agent-driven verification mandate — every WCAG criterion must have machine-runnable verification; no manual-only tests)

### A11y Anti-Patterns (creator's explicit asks)

- "**NOT:** Datadog / New Relic / Grafana enterprise-dashboard density (information overload). Not Status Hero / Pingdom (vacant marketing-app sterility). Not Neon / Supabase (heavy gradient SaaS aesthetic)." (information-overload + decorative-gradient anti-patterns — implies cognitive accessibility discipline: scannable hierarchy, restraint over density)
- "motion-as-data principle (motion reflects telemetry character, не decoration)" (decorative-motion anti-pattern — motion must carry semantic meaning; aligns with reduced-motion respect)
- "Status colors not-color-alone (paired с iconography per WCAG)" (color-alone signaling anti-pattern — supplemental icon/label required for every status color)