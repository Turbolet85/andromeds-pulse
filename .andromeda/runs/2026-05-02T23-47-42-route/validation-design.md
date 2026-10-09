# Design validation — route draft

## Insert

- Between `Design tokens bundle` and `A11y dev stack install`: **"Iconography registry — custom SVG glyphs {aperture, telescope, constellation-grid, star, circular-pulse} registered as React components at `src/components/icons/`"** (epoch: `Foundation`)
  Reason: Per design-system.md §Iconography, custom SVGs must be registered and available before any UI surface consumes them; this is foundational infrastructure parallel to typography loading.

- Between `Full dashboard shell + tab nav` and `Trace timeline + per-service constellation`: **"Investigation modal + Settings modal component scaffold — modal overlay card layout (color-raised-3 bg + subtle border + radius-lg padding), close button, content area, focus trap and aria-busy/aria-live hooks"** (epoch: `Visualization surfaces`)
  Reason: Per layout-templates.md §Component — Investigation modal and §Component — Settings modal form, both modals have detailed structure specifications (header, inner card styling, footer controls) that should be bootstrapped as component primitives before trace/settings features build atop them.

- Between `Trace timeline + per-service constellation` and `Metrics charts + logs stream`: **"OS-native notifications component — Tauri 2 `tauri-plugin-notification` wrapper for snapshot completion / MCP server state / updater events with terse title + 2-line body format + action button contract"** (epoch: `Visualization surfaces`)
  Reason: Per layout-templates.md §Component — Notifications (OS-native) and design-system.md §Surface: desktop-native, notifications are a mandatory surface element triggered by Snapshot and MCP sub-systems (Epoch 6–7); they must be scaffolded in Visualization before downstream epochs consume them.

- Between `Halo State Pulse signature element` and `Compact widget infographics + footer`: **"Motion infrastructure — Investigation Capture Collapse supporting moment (350ms scale + opacity ease-out), motion-reduced graceful degradation via prefers-reduced-motion media query, motion metric event bridging via TauRPC `telemetry.frontend.record_frame_ms`"** (epoch: `Visualization surfaces`)
  Reason: Per design-system.md §Motion and layout-templates.md §Motion trigger placement, Investigation Capture Collapse requires explicit 350ms timing + aria-busy/aria-live accessibility hooks; this infrastructure (timing contract + aria markup boilerplate + TauRPC signal path) must precede the trigger point in Epoch 6.

## Reorder

- Move `Settings modal form` (currently in Visualization surfaces) after `Full dashboard shell + tab nav` and before `Trace timeline + per-service constellation`
  Reason: Per design-system.md §Component Patterns and layout-templates.md, Settings modal is a foundational component that routes and controls widget position, theme, retention; it should be scaffolded early in Visualization so Settings tab / compact-widget settings icon have the form infrastructure available by the time those surfaces are wired.

## Rewrite

- `Design tokens bundle`: change "Tailwind v4 @theme NASA palette + IBM Plex Sans + JetBrains Mono WOFF2 bundled local CSP-safe" → "Tailwind v4 @theme NASA palette (colors + spacing + border radius + motion tokens) + IBM Plex Sans + JetBrains Mono WOFF2 bundled local CSP-safe"
  Reason: Per design-system.md, spacing tokens (space-micro through space-xl), border radius tokens (radius-sm through radius-full), and motion duration tokens (duration-fast, duration-standard, easing tokens) are explicit design artifacts and should be named in the bundle scope marker.

- `Halo State Pulse signature element`: change "WebGPU shader pulse 0.8-2.4 Hz from throughput/1000, LCH hue Earth Blue ↔ Alert Burgundy, 4-16px blur" → "WebGPU shader pulse frequency (0.8–2.4 Hz clamped from throughput/1000), LCH hue interpolation (Earth Blue ↔ Alert Burgundy per error rate), blur radius envelope (4–16 px per pulse cycle), reduced-motion degradation (static glow, hue still updates)"
  Reason: Per design-system.md §Motion and layout-templates.md, the Halo State Pulse has explicit reduced-motion accessibility requirements and blur envelope timing per cycle; scope marker should include these contract details.
