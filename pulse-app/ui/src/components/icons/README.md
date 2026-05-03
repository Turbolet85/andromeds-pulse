# Iconography Registry

Custom Observatory / Mission Control SVG glyphs registered as React components.
Brand contract: `.andromeda/design-system.md` §Iconography (Primary custom set,
Size grid, Rule "Icons clarify, not decorate").

## Catalog (5 glyphs)

| Glyph | Domain meaning | Canonical surfaces |
|---|---|---|
| `aperture` | View settings — telescope/microscope diaphragm parallels data filtering | Custom titlebar settings button (compact widget + full dashboard); App-icon-eligible |
| `telescope` | Investigate — observational astronomy parallels distributed-trace deep inspection | Full dashboard header Investigate trigger; trace data-table row hover affordance |
| `constellation-grid` | Service map — celestial coordinates parallel service topology mapping | Custom titlebar App-icon (interchangeable with aperture); future service-map view |
| `star` | Favorite snapshot — night-sky reference point for navigation | Snapshots view favorite indicator; canonical tray-icon source shape |
| `circular-pulse` | Compact widget / tray surface toggle — mirrors Halo State Pulse rhythm visually (STATIC; rhythm itself lives on the WebGPU canvas, NOT in this SVG) | Compact widget toggle button; widget ↔ dashboard mode swap |

## API

Two equivalent forms:

```tsx
// Per-glyph named import (tree-shakeable)
import { Aperture } from "@/components/icons";
<Aperture size={20} aria-label="Open settings" />

// Dispatcher form (per design-system.md §Iconography)
import { Icon } from "@/components/icons";
<Icon glyph="aperture" size={20} aria-label="Open settings" />
```

### Props

```tsx
interface IconProps {
  size?: 16 | 20 | 24;        // default 24
  "aria-label"?: string;       // when provided, icon becomes meaningful (role="img")
  "aria-labelledby"?: string;  // alternative to aria-label
  title?: string;              // optional <title> child for tooltip / legacy AT
  className?: string;          // Tailwind utility classes (e.g., "text-text-secondary")
}
```

## Decorative-vs-meaningful pattern

Every glyph component supports BOTH modes. Default is decorative; provide an
accessible name to flip к meaningful:

```tsx
// DECORATIVE — icon supplements an already-labeled control. Screen readers
// skip the icon and read the wrapper button's accessible name.
<button aria-label="Open settings">
  <Aperture size={20} aria-hidden="true" />  {/* default; aria-hidden="true" implicit */}
  Settings
</button>

// MEANINGFUL — icon IS the only accessible name. Screen readers announce it.
<Aperture size={24} aria-label="Settings open" />
```

Per `.claude/rules/a11y.md` "Semantic HTML first, ARIA second": prefer
decorative-mode-on-icon + accessible name on the wrapper interactive element.
Avoid double-labeling (`aria-label` on both icon AND wrapper) — produces
duplicate screen-reader announcements (a11y-plan §11 SR anti-patterns).

## Color tokens (currentColor propagation)

Icons render с `fill="currentColor"` / `stroke="currentColor"` so the inherited
CSS `color` property determines the visual color. Use Tailwind v4 text-color
utilities mapped to the chunk-#10 `@theme` tokens:

```tsx
<Aperture className="text-text-primary" />     {/* default surface chrome */}
<Aperture className="text-text-secondary" />   {/* secondary tone (titlebar default per layouts) */}
<Aperture className="text-feedback-success" /> {/* success state */}
<Aperture className="text-accent" />           {/* error / alert state */}
```

The chunk #12 contrast verification harness reads `getComputedStyle(wrapper).color`
to assert SC 1.4.3 / SC 1.4.11 thresholds at the consumer's site, not at the
icon source — registry honors propagation by design.

## Not-color-alone state-indicator pattern (SC 1.4.1)

When using an icon as a state indicator, ALWAYS pair с а text label or
icon-glyph that distinguishes the state independently of color (per a11y-plan §6
not-color-alone discipline + design plan §Anti-Patterns Universal Bans):

```tsx
// ✓ Compliant — icon + text + color all reinforce the state
<span className="inline-flex items-center gap-xs text-feedback-success">
  <Star size={16} aria-hidden="true" />
  <span>Snapshot saved</span>
</span>

// ✗ NON-compliant — color alone signals success. Color-blind users
// see no distinguishing signal versus а neutral state.
<span className="text-feedback-success">Saved</span>
```

## Size grid + wrapper padding

Per design-system.md §Iconography Size grid:

| Size | Padding (wrapper) | Surface examples |
|---|---|---|
| 16px | `--spacing-xs` (4px)   | Tray icon density, inline labels |
| 20px | `--spacing-sm` (8px)   | Buttons, list items, navigation |
| 24px | `--spacing-md` (16px)  | Card titles, large buttons, hero elements |

WCAG SC 2.5.8 (target size 24×24 CSS px) is satisfied by the WRAPPER element,
not the icon itself. The 16/20px icons are valid only when their wrapper hits
24×24 minimum (per a11y-plan §6 Target size tokens). The `--target-button-min`
(44px) and `--target-input-min` (24px) tokens are the wrapper-side enforcement.

## Motion deferral

The `circular-pulse` glyph LOOKS like а pulse but is а STATIC SVG. The actual
rhythmic pulsing motion lives on the WebGPU canvas at chunk #28 (Halo State
Pulse signature element); icon components MUST NOT include the SVG animation
elements `animate`, `animateTransform`, `animateMotion`, or `set` per a11y-plan §11 Motion
("NEVER autoplay motion without `prefers-reduced-motion: reduce` respect") +
design-system.md §Motion Hard limits. Motion tokens + `useReducedMotion` hook
land at chunk #15.

## Coverage scope (test-plan §10)

Per the chunk #11 phase plan acceptance criteria (path b), pure-presentation
icon components are explicitly EXCLUDED from the workspace coverage gate
(75%/70%/85%) at this Foundation pre-shell stage. Integration coverage applies
when `tauri-driver` E2E lands at chunk #25 (webview shell). Vitest unit tests
in this directory verify ARIA + DOM-shape contracts, NOT visual fidelity.
