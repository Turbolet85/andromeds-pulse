# custom-dashboard plugin template

WIT contract: [`../../crates/plugins/wit/custom-dashboard.wit`](../../crates/plugins/wit/custom-dashboard.wit)

## Category overview

`custom-dashboard` plugins render UI surfaces inside the host webview.
The host calls the plugin's `render` export on every frame the
dashboard surface needs to repaint, passing:

- `viewport: viewport { width: u32, height: u32 }` — current render
  region dimensions in pixels.
- `a11y: a11y-state { reduced-motion: bool }` — read-only accessibility
  state from the host webview. Plugins MUST consume this rather than
  detect preferences independently.

## Accessibility obligations (WCAG 2.1 AA + SC 2.3.3 AAA)

Plugin authors MUST honor these when implementing `render`:

- **Semantic HTML first.** Native semantic elements (`<button>`, `<nav>`,
  `<section>`, etc.); ARIA roles ONLY on non-native elements. Anti-pattern:
  ARIA role on `<div>` when `<button>` would suffice.

- **Reduced motion respect.** Plugin frame loops MUST honor
  `a11y.reduced_motion`. The host detects `prefers-reduced-motion: reduce`
  once and passes the state to every render call. Anti-pattern: plugin
  runs independent `matchMedia` query or ignores the host state.

- **No keyboard traps.** Plugin UI MUST NOT trap keyboard focus. Modals
  or dialogs MUST honor the host focus-trap-react patterns; never
  capture `Tab` globally.

- **Contrast.** Plugin UI MUST meet WCAG 1.4.3 4.5:1 minimum via host
  design tokens (`var(--color-text-primary)` on `var(--color-base)`,
  etc.). Hardcoded RGB literals bypass token discipline.

## Design tokens (NASA Deep Space palette)

If your plugin renders any UI surface inside the host webview, consume
the host's CSS custom properties:

- Colors: `var(--color-base)`, `var(--color-raised-1)`,
  `var(--color-text-primary)`, `var(--color-text-secondary)`,
  `var(--color-accent)`, `var(--color-alert)`, etc.
- Typography: `var(--font-body)` (IBM Plex Sans),
  `var(--font-code)` (JetBrains Mono).
- Spacing: `var(--space-xs)`, `var(--space-sm)`, `var(--space-md)`, etc.
- Radius: `var(--radius-sm)` (4px), `var(--radius-md)` (6px).
- Motion: `var(--duration-fast)` (150ms), `var(--duration-medium)` (200ms).

Banned fonts (do NOT use as primary): Inter, Roboto, Arial, Helvetica,
Open Sans, Lato, system-ui default, Space Grotesk.

## Motion budget

Chrome interactions: 150ms (hover/focus) to 200ms (panel transitions)
ease-out / ease-in-out. NO parallax, scroll animations, spring physics,
staggered reveals, 3D transforms, opacity fades > 200ms, or
canvas/WebGL (the Halo State Pulse layer is exempt and reserved for the
host).

## Status at chunk #47

Compilable example scaffold deferred to a future chunk introducing
`wasmtime::component::bindgen!` for the `custom-dashboard` category.
Until then, this README documents the contract.
