---
paths:
  - "pulse-app/ui/**/*.{ts,tsx,jsx,js}"
  - "pulse-app/ui/**/*.css"
  - "pulse-app/ui/**/tailwind.config.*"
---

# Design Token Rules

Path-scoped rules for the desktop-webview surface (React 19 + Tailwind CSS v4 + shadcn/ui). Loaded only when Claude is working with files matching the `paths:` frontmatter above.

**Authoritative source:** `.andromeda/design-system.md` §Color Palette + §Typography + §Spacing + §Motion + §Anti-Patterns + §Self-Validation Protocol. Brand: NASA Deep Space Mission Control Station (Modern Artemis Design). Signature element: Halo State Pulse (WebGPU shader-driven).

## Color palette (NASA Deep Space — locked)
The palette is derived from the user-confirmed Color World; library-shortlist palettes are STRUCTURE only, NOT literal values.

**Core:**
- `--color-primary` `#4A90E2` — Earth Blue (actions, focus rings, active states, healthy state)
- `--color-secondary` `#2C3E7F` — Stellar Indigo (panel backgrounds, secondary nav)
- `--color-accent` `#C7556A` — Alert Burgundy (alerts, errors, anomaly indicators); lifted from #8B2E3B per design-system.md Decisions Log 2026-05-03 (a11y SC 1.4.11 non-text 3:1 minimum). Now classified as non-text token (border / icon / large-text emphasis); body-size error text should use `--color-text-primary` paired with `--color-accent` border + icon per a11y "never color alone" discipline.

**Surface scale (borders-only depth strategy):**
- `--color-base` `#1A1D24` — Deep Control Gray (page/app background)
- `--color-raised-1` `#262A33` — cards, panels (2-3% lighter)
- `--color-raised-2` `#2D3139` — dropdowns, popovers, tooltips
- `--color-raised-3` `#343A45` — modals, dialogs (max elevation)
- `--color-inset` `#0F1117` — input fields, code blocks, Halo canvas background

**Text hierarchy (on Base):**
- `--color-text-primary` `#E8EEF7` — Status White-Blue (~8.5:1)
- `--color-text-secondary` `#B4BCCB` (~6.8:1)
- `--color-text-tertiary` `#7D8697` (~4.2:1, large text only)
- `--color-text-muted` `#56606E` (~2.1:1, decorative only)

**Semantic / feedback:**
- `--color-feedback-success` `#17B3A3` — Feedback Cyan (ISS-cabin lighting)

**Border progression (Earth Blue at varying opacity):**
- Subtle `rgba(74, 144, 226, 0.1)` / Standard `0.3` / Emphasis `0.6` / Focus solid `#4A90E2`

NEVER deviate from these hex values without an explicit Decisions Log entry. NEVER use Tailwind default palette (purple/sky/zinc/etc.) as brand identity. The Halo State Pulse signature element is the ONLY exception that uses radial gradient + blur.

## Typography (locked: JetBrains Mono + IBM Plex Sans)
- `--font-display` / `--font-body` `'IBM Plex Sans', sans-serif` — UI, humanist sans = "human-readable summary"
- `--font-code` `'JetBrains Mono', monospace` — data rows, immutable telemetry fact

**Sizes / weights** (per design-system §Typography table): Display 32/600, Heading 20/600, Body 14/400, Label 12/500, Code 12/400 (tracking 0.5), Data 12/400 tabular-nums.

Both fonts bundled locally as WOFF2 (CSP-safe per `script-src 'self'`); no CDN. Source: `pulse-app/ui/public/fonts/`.

**BANNED font families** (NEVER use as primary): Inter, Roboto, Arial, Helvetica, Open Sans, Lato, system-ui default, Space Grotesk.

## Spacing (4px base unit)
- `--spacing-micro` 2px / `--spacing-xs` 4px / `--spacing-sm` 8px / `--spacing-md` 16px / `--spacing-lg` 24px / `--spacing-xl` 32px

All values are multiples of 4 — high-DPI crisp rendering + Tailwind v4 alignment + dense Mission Control grid discipline.

## Border radius (geometric, sharp)
- `--radius-sm` 4px (buttons/inputs) / `--radius-md` 6px (cards/panels) / `--radius-lg` 8px (modals) / `--radius-full` 9999px (badges/toggles)

NEVER use organic curves. Mixed strategy: most chrome at sm/md; only badges + toggles at full.

## Motion (expression level 0.3 base / 0.35 webview / 0.2 native)
- `--duration-fast` 150ms / `--duration-standard` 200ms / `--easing-out` `cubic-bezier(0.4, 0, 0.2, 1)`
- Investigation Capture Collapse 250–350ms (supporting moment, EXEMPT from 200ms hard limit per design §Motion)
- Halo State Pulse breathing period 4–5 s quiet → ~2 s under active flow (≈0.2–0.5 Hz; data-driven via WebGPU shader, driven by activity state — supersedes the chunk #31-era 0.8–2.4 Hz `throughput_hz / 1000` band per design-system.md Decisions Log 2026-05-29); opacity + blur modulation ONLY, never scale (P-026); blur radius 4–16 px per cycle mapped from cumulative incident severity; LCH hue interpolation Earth Blue ↔ Alert Burgundy by cumulative incident severity; connection state is an orthogonal grayout/desaturation axis; EXEMPT from chrome budget (separate WebGPU canvas layer)

**Hard limits (NEVER do):** parallax / scroll animations / spring physics / staggered reveals / 3D transforms / canvas-WebGL except Halo / opacity fades >200ms.

**Reduced motion (`prefers-reduced-motion: reduce`):** all transitions become instant (200ms→0ms, 150ms→0ms); Halo degrades to static glow (hue still updates per cumulative incident severity).

## Iconography
- **Custom SVG glyphs** (registered as React components at `src/components/icons/`): `aperture`, `telescope`, `constellation-grid`, `star`, `circular-pulse` — astronomy/Observatory metaphor, monochrome `#E8EEF7` default.
- **Secondary fallback:** Lucide or Heroicons for generic controls only — NEVER as primary visual language.
- Sizes: 16px (tray, inline labels), 20px (buttons, list items), 24px (card titles, hero).

## Component patterns (chrome — Tailwind v4 `@theme` consumption)
- **Cards:** `bg-raised-1` + 1px subtle border + `radius-md` + `padding space-md`. Hovered: emphasis border. NO shadows (borders-only depth).
- **Inputs:** `bg-inset` + 1px subtle border + `radius-sm` + `padding space-sm`. Focused: 1px solid `--color-primary` + 3px outset `box-shadow rgba(74, 144, 226, 0.2)`. Error: `--color-accent` border + accent text below.
- **Buttons (primary):** `bg-primary` + `text-base` + `padding space-sm horizontal + space-xs vertical` + `radius-sm`. Hovered: `#5BA5F0` (Earth Blue +10% lightness), 150ms ease-out.
- **Tables:** transparent rows + 1px subtle border between rows; hover row `rgba(74, 144, 226, 0.1)` instant; cells `font-data` 12px tabular-nums + space-sm padding.
- **Halo State Pulse canvas container:** `bg-inset` + 1px subtle border + `radius-md` + `padding space-md`; `<canvas>` fills container.

## Dark mode default
`prefers-color-scheme: dark` is default; light mode + auto via Settings, persisted to `~/.andromeda-pulse/config.toml`.

## CSP policy
`script-src 'self'` per security plan §API Security CSP — no eval, no remote, no CDN. WOFF2 fonts bundled locally. WGSL shaders first-party inline or module-imported.

## Anti-patterns (design §Universal Bans + Per-Surface Bans)
- NEVER use generic font families as primary (banned list above).
- NEVER use Tailwind default palette colors as brand identity.
- NEVER use gradient overlays / glassmorphic effects on dashboard chrome (Halo State Pulse is the ONLY exception).
- NEVER use the same layout for different information types — metric display ≠ form ≠ data table ≠ status page.
- NEVER use color purely for decoration — every color encodes meaning per the palette.
- NEVER converge on safe defaults (sidebar + card grid, hero→features, form-in-card) — interface emerges from Observatory / Mission Control domain.
- NEVER use uniform monochrome status icons (green ✓ / red ✗) — Halo encodes throughput rhythm + error hue independently.
- NEVER use bounce easing or animations >250ms in chrome (supporting moments at 350ms exempt per design §Motion).
- NEVER use generic stock icons across chrome — custom Observatory glyphs first.

**desktop-webview specific bans:**
- NEVER ship visible Chromium/WebView2 artifacts (default context menu, dev tools, text selection on non-text).
- NEVER use unstyled web scrollbars (apply Tailwind v4 `scrollbar-*` utilities, dark bg + Earth Blue thumb).
- NEVER use browser navigation chrome (back/forward, URL bar).
- NEVER use hover-only interactions without keyboard alternatives.
- NEVER make windows non-resizable without strong justification.
- NEVER use `alert()` / `confirm()` / `prompt()` — use shadcn/ui Dialog.

## Self-Validation (run before presenting any UI)
1. **Swap test** — replace fonts/palette/icons/Halo with Inter + Tailwind defaults + Lucide + spinner; if no meaningful difference → defaulted, redo.
2. **Squint test** — blur eyes; hierarchy still perceptible? Nothing screams?
3. **Signature test** — Halo present in 3 places (full dashboard / compact widget / tray icon)?
4. **Token test** — every value traces to Color World / spacing scale / font stack?
5. **Sameness test** — would another AI produce the same output? If yes, re-anchor to Observatory metaphor.
6. **Contrast test** — values match Text Hierarchy table ratios?

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._

- 2026-05-03: Tailwind v4 `@theme` directive tree-shakes unused theme tokens by default — only emits CSS custom properties for tokens referenced by utility classes scanned in HTML/JS sources. For design-token registry chunks where utility class usage hasn't materialized yet (e.g., chunk #10 lands tokens before chunk #25 React shell consumes them), the `@theme static { ... }` modifier is REQUIRED to force emission of all theme variables to `:root` regardless of usage. Without `static`, `npm run build:css` succeeds silently but compiled CSS contains only Tailwind's defaults — no NASA palette, no a11y tokens, no `getComputedStyle('--token-name')` reads will resolve. Pair with `@import "tailwindcss"` at the top of the input file to enable Tailwind v4 directive processing. See `pulse-app/ui/src/styles/tokens.css` lines 1-3.

- 2026-05-08: WGSL fragment shaders MUST receive design-token colors as uniforms read from CSSOM at canvas-init time, NOT inline as `vec4<f32>(0.X, 0.Y, ...)` literals — RGB literals in shader source bypass the design-system token registry, the colorjs.io contrast harness in `pulse-app/ui/scripts/verify-contrast.mjs`, and the a11y plan §6 token-pair contrast assertions (chunks #10/#12). Pattern: declare a `ColorUniforms` struct in WGSL with the four-to-six tokens the chart needs (Primary / Accent / Text-Primary / Inset is the chunk #28 baseline; extend per chart family); on JS side, read tokens via `getComputedStyle(document.documentElement).getPropertyValue('--color-X')` at WebGPU pipeline init, parse to `vec4<f32>` floats, write into the uniform buffer before each render-pass dispatch. The only allowed `vec4<f32>(0.X, ...)` literals in shader source are STRUCTURAL — clear-to-transparent or position-output coordinates — never tinted color values. Verified in chunk #28 shaders (`pulse-app/ui/src/canvas/shaders/{trace-timeline,flamegraph,metrics-chart}.wgsl`); enforced via `grep -E 'vec[34]<f32>\(0\.[0-9]' pulse-app/ui/src/canvas/shaders/*.wgsl` returning only structural-zero literals. Applies to every Epoch 5 chart shader added downstream (#34 trace timeline data, #35 metrics charts data).
