# Design System — andromeda-pulse

## Brand Identity

**Personality:** Ambient constellation — luminous, patient, emergence-driven

**Domain anchors:**
- Observatory station: physical locus where telemetry watchers correlate traces/metrics/logs to detect performance patterns
- Constellation: service topology rendered as interconnected dots with halo brightness encoding operational health
- Mission Control console: NASA Artemis workstation with purpose-built directional lighting and color-coded alert states
- Spectroscopy: decomposing light by wavelength; halo's LCH color shift (Earth Blue ↔ Alert Burgundy) encodes error-rate composition
- Ring buffer ephemeris: transient telemetry (5–10 min window) becomes persistent narrative via snapshot generation

**Signature element:** "Halo State Pulse" — a circular animated glow rendered on a dedicated WebGPU canvas layer around each service constellation dot, pulsing at a rate proportional to service throughput (0.8–2.4 Hz, clamped from `throughput_hz / 1000`) and shifting hue via LCH interpolation from Earth Blue (#4A90E2) to Alert Burgundy (#8B2E3B) based on error rate. The blur radius expands/contracts with each pulse (4–16 px). On the tray icon, a single unified halo pulses and shifts hue around the aggregated service-count badge.

**Expression level:**
- **Base:** 0.3 (subtle hover states, fade transitions, quiet focus rings)
- **Per-surface:**
  - **desktop-webview:** 0.35 (compact widget + full dashboard expansion justify slightly more interactive feedback: skeleton pulsing, panel transitions, form state confirmation, command-palette emergence)
  - **desktop-native:** 0.2 (tray icon system integration constraints argue for minimal motion — icon state changes, traffic-light color transitions, badge emergence only)

**Design direction:** The calm precision of a mission-control station during nocturnal surveillance: dark surfaces minimize glare, color-coded states encode meaning without ornament, motion is data-driven and never decorative. Visual weight is reserved for the constellation visualization (WebGPU canvas, Halo State Pulse) and typographic hierarchy; chrome is flat, matte, disciplined. Observatory metaphor scales from intimate (tray icon) to expansive (full dashboard canvas), with every surface maintaining the same dark-first, pattern-seeking, contemplative voice.

**Design Direction (from library-shortlist):** Data & Analysis — data-density priority, developer-tool aesthetic, monospace for telemetry, fast interaction feedback, minimal chrome, pattern-seeking visual hierarchy.

---

## Color Palette

**Rationale:** The user selected "NASA Deep Space Mission Control Station (Modern Artemis Design)" as the mood anchor — a secondary monitor console within NASA's modern Artemis Mission Control room. Dimly lit walls with blue accents; carpeting depicts lunar mineral crystalline patterns in gray, blue, and burgundy. The color world drives the entire design system and is the single source of truth for all semantic and surface-elevation colors. Every hex value below is derived directly from the exploration's Color World section; the library-shortlist palettes were used only for structural reference (palette layout, industry rules, UX guidelines), not for literal color values.

### Core Colors

| Role | Value | Usage | Domain anchor |
|------|-------|-------|---------------|
| Primary | #4A90E2 | Actions, focus rings, active states, Earth accent lighting | Earth Blue — accent lighting representing Earth (as visible from the observation station), visible but not harsh; healthy active state, mission-phase markers, non-critical but important signals (service is running, responding nominally). |
| Secondary | #2C3E7F | Supporting actions, secondary navigation, panel backgrounds | Stellar Indigo — deep indigo of the night sky at twilight, after nautical darkness sets but before full astronomical darkness; used for secondary surfaces (panel backgrounds, borders) to maintain hierarchy without introducing a fourth hue. |
| Accent | #8B2E3B | Alerts, error states, anomaly indicators, emphasis | Alert Burgundy — lunar anorthite mineral crystalline pattern in Artemis Mission Control carpeting (burgundy veins in gray basalt); also the standard anomaly-state hue in NASA's color-coded alert taxonomy; error and outlier indicator. |

### Surface Scale (elevation hierarchy)

| Level | Value | Usage |
|-------|-------|-------|
| Base | #1A1D24 | Page/app background — Deep Control Gray, the interior surface of NASA Artemis Mission Control, dark envelope inside which operators sit for 8+ hour observation sessions; minimizes glare during nocturnal vigil. |
| Raised-1 | #262A33 | Cards, panels — 2-3% lighter than base for subtle elevation via color, no shadows (depth strategy: borders-only). |
| Raised-2 | #2D3139 | Dropdowns, popovers, tooltips — further elevation via lightness, maintained within dark envelope. |
| Raised-3 | #343A45 | Modals, dialogs — maximum elevation while preserving dark-first aesthetic, borders define boundary. |
| Inset | #0F1117 | Input fields, code blocks, Halo State Pulse canvas background — darker than parent, recessive visual weight. |

### Text Hierarchy

| Level | Value | Usage |
|-------|-------|-------|
| Primary | #E8EEF7 | Headlines, body text, primary labels — Status White-Blue, deliberately cool-tinted to align circadian rhythm for marathon observation sessions (nocturnal blue accent over warm amber avoids stimulation). Critical display text in Mission Control ensuring 8-hour reading legibility. |
| Secondary | #B4BCCB | Descriptions, supporting text — reduced saturation of Primary, maintains cool tone. |
| Tertiary | #7D8697 | Metadata, timestamps, captions — further reduced saturation, maintains hierarchy. |
| Muted | #56606E | Disabled text, placeholders — low-contrast, recessive. |

### Semantic Colors

| State | Background | Border | Text |
|-------|------------|--------|------|
| Success | #0F1117 | #17B3A3 | #17B3A3 |
| Warning | #1A1D24 | #8B2E3B | #E8EEF7 |
| Error | #1A1D24 | #8B2E3B | #8B2E3B |
| Info | #1A1D24 | #4A90E2 | #4A90E2 |

Feedback Cyan (#17B3A3) is used for Success — teal-cyan of emergency lighting in advanced spacecraft cabins (ISS module accent lighting); used sparingly for confirmation states and non-critical feedback (form validation success, investigative actions completed).

### Border Progression

| Intensity | Value | Usage |
|-----------|-------|-------|
| Subtle | rgba(74, 144, 226, 0.1) | Section separators, card edges — Earth Blue at 10% opacity. |
| Standard | rgba(74, 144, 226, 0.3) | Input borders, list dividers — Earth Blue at 30% opacity. |
| Emphasis | rgba(74, 144, 226, 0.6) | Active states, selected items — Earth Blue at 60% opacity. |
| Focus | #4A90E2 | Focus rings (accessibility-critical) — solid Earth Blue for 3–4px outset ring, 0.5–1px line-width via box-shadow or outline. |

---

## Typography

**Rationale:** The library-shortlist top pairing was JetBrains Mono + IBM Plex Sans, chosen for developer-focused technical precision and direct Linear reference (Q2 reference product endorses monospace for traces, sans for UI). JetBrains Mono signals "immutable telemetry fact" on data rows (span IDs, trace IDs, latency values, attribute keys) mirroring Mission Control LED panels; IBM Plex Sans signals "human-readable summary" for control labels, settings, help text — humanist alternative to banned-list fonts. Both fonts are bundled locally via Google Fonts WOFF2 (CSP-safe per security-plan.md `script-src 'self'`), eliminating any CDN dependency. The split between monospace (data) and sans (UI) is not merely aesthetic — it encodes domain semantics (spectroscopy decomposition metaphor: immutable vs. operator communication).

| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Display | IBM Plex Sans | 600 | 32px | 0 | Page titles, hero text — "Service constellation overview", "Investigation insights". Contemplative tone prioritizes clarity over flourish. |
| Heading | IBM Plex Sans | 600 | 20px | 0 | Section headers, card titles — "Traces", "Metrics", "Logs", "Snapshots". |
| Body | IBM Plex Sans | 400 | 14px | 0 | Paragraphs, descriptions, control labels — form labels, menu items, settings panel text. |
| Label | IBM Plex Sans | 500 | 12px | 0 | Form labels, button text, nav items — buttons, toggles, small control identifiers. |
| Code | JetBrains Mono | 400 | 12px | 0.5 | Code blocks, paths, technical values — within data tables, error messages, configuration strings. Monospace signals immutable telemetry fact. |
| Data | JetBrains Mono | 400 | 12px | 0 (tabular-nums) | Numbers, metrics, tables — span latencies, error counts, throughput values, ring-buffer timestamps. Tabular numerals align columns for readability. |

**Loading:** Bundled self-hosted WOFF2 via Google Fonts offline distribution (no CDN). Both JetBrains Mono and IBM Plex Sans are available with full weight ranges (300–700) and latin extended character sets.

**Surface-conditional guidance:**
- **desktop-webview:** Full table above applies. Webview renders typography via React 19 + Tailwind v4 `@theme` CSS custom properties mapping to font-family, font-size, font-weight, letter-spacing. Dark mode default (`prefers-color-scheme: dark`), light mode and auto via Settings.
- **desktop-native:** System font — OS-controlled. Brand personality expressed through: icon design (custom SVG glyphs for aperture, telescope, constellation-grid, star, circular-pulse), menu text tone (contemplative, observational voice in action labels), notification copy style (terse, 2 lines max, data-driven state indicators), tray icon aggregation (unified halo around service-count badge). Monospace-vs-sans semantic split is not applicable at this surface; tray menu is OS-native and typography is fixed by the platform.

---

## Spacing

| Token | Value | Usage |
|-------|-------|-------|
| space-micro | 2px | Icon-to-text gap, inline element spacing (within a label or button). |
| space-xs | 4px | Tight padding within small components (badge, small button). |
| space-sm | 8px | Component internal padding (buttons, inputs, small cards). |
| space-md | 16px | Card padding, section gaps, standard container padding. |
| space-lg | 24px | Major section separation, primary axis spacing between panels. |
| space-xl | 32px | Page-level margins, hero spacing, dashboard outer margins. |

**Base unit:** 4px — all values are multiples of this. Multiples of 4 enable crisp rendering on high-DPI displays and align with both Tailwind v4 spacing scale and the dense, grid-like layout discipline (Mission Control console metaphor).

**Surface-conditional:** Spacing tokens apply to `desktop-webview` surface (compact widget + full dashboard expansion). For `desktop-native` (tray icon and menu), spacing is platform-controlled via OS conventions — omit custom spacing rules and let the OS handle menu item padding, icon sizing, and label positioning.

---

## Depth Strategy

**Chosen approach:** borders-only

**Rationale:** Linear reference (Q2) and Raycast reference (Q2) both endorse "restrained dark-first SaaS palette rejecting gradient overload" and "invisible-until-summoned UI, minimal chrome, no decorative gradients." Shadows on dark backgrounds flatten into the surrounding darkness (psychological weight is lost); tints wash out under the cool, low-luminance NASA palette. Borders are sharp, readable, and architecturally meaningful — they define card boundaries, input focus states, and state transitions without adding visual weight. The Halo State Pulse (WebGPU canvas, dedicated layer outside React's render tree) is the sole exception: it uses a radial gradient + blur (visual weight reserved for the signature element, not chrome ornamentation).

**Specific values:**
- **Default border:** 1px solid rgba(74, 144, 226, 0.3) — Earth Blue at 30% opacity.
- **Focus/active border:** 1px solid rgba(74, 144, 226, 0.6) — Earth Blue at 60% opacity, or solid #4A90E2 for high-contrast focus rings.
- **Error/alert border:** 1px solid rgba(139, 46, 59, 0.5) — Alert Burgundy at 50% opacity.
- **Card elevation:** raised surface color (#262A33 for Raised-1) + border (subtle), no shadow. Layering is achieved via background lightness, not shadow depth.
- **Modal elevation:** #343A45 (Raised-3) background + 1px border (subtle or emphasis depending on context), positioned above other content via z-index only.

---

## Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| radius-sm | 4px | Inputs, buttons, small controls — subtle rounding signals interactive intent without softness. |
| radius-md | 6px | Cards, panels — slightly more prominent than buttons but still geometric (no organic curves). |
| radius-lg | 8px | Modals, large containers — reserved for largest surfaces. |
| radius-full | 9999px | Avatars, pills, toggles, badges — service status badges, loading indicators, focus rings. |

**Personality:** Sharper than neutral (technical aesthetic), but not fully squared (not cold/austere). Mixed strategy: most components use radius-sm/md (grid-like, precise, aligned with Mission Control console metaphor); only badges and toggles use radius-full (to signal "complete" or "whole," reinforcing the constellation/halo metaphor). Observatory station metaphor + mission control discipline argue for geometric precision over organic friendliness.

**Surface-conditional:** Border Radius applies to `desktop-webview` surface. Omit for `desktop-native` (tray menu uses OS-native corner styles).

---

## Motion (calibrated to expression level 0.3 base / 0.35 webview / 0.2 native)

All motion decisions flow from the expression level set in Brand Identity. Canvas motion (WebGPU shader-driven Halo State Pulse, Latency River, throughput counter) is a **separate dimension** governed by the data-viz layer and NOT bound by the chrome expression number. **Downstream: Obs specialist should provide a hook to log Halo pulse frequency (Hz) and color state (Earth Blue / Alert Burgundy / interpolation) on each pulse; design specifies the measurement intent, obs configures the backend platform.**

**Easing:** ease-out (standard) for focus/hover feedback; ease-in-out for panel transitions. No spring physics, no bounce — contemplative tone rejects the "delightful" framing in favor of "state-confirming" motion.

**Duration scale (adjusted for expression level 0.3–0.35):**

| Expression | Hover/Focus | Page transition | Entrance | Scroll effects |
|------------|-------------|-----------------|----------|----------------|
| 0.2–0.35 | 150ms ease-out | 200ms fade | none | none |

**This project's values:**
- **Micro-interactions (hover, focus):** 150ms ease-out — button background lightens, focus ring appears.
- **Transitions (panel open, page change):** 200ms ease-out fade (Investigation Capture Collapse candidate signature: 250ms scale + opacity ease into a centered snapshot object on Investigate click, per supporting moment).
- **Entrance animations:** none — components render instantly; motion confirms state changes only, not arrivals.
- **Scroll effects:** none — expression level does not warrant parallax or scroll-driven reveals.

**High-impact moments (max 2 at 0.3–0.35 level):**
1. **Halo State Pulse breathing:** WebGPU shader-driven, sinusoidal animation at 0.8–2.4 Hz (data-driven frequency, exempt from chrome budget). Blur radius 4–16 px per cycle, 0.6–1.2 s per breath. Color interpolation (Earth Blue ↔ Alert Burgundy via LCH) is simultaneous with rhythm.
2. **Investigation Capture Collapse (supporting moment):** 350 ms scale + opacity ease into a centered snapshot object when Investigate button is clicked. Confirms action without demanding attention. Supporting moments are EXEMPT from the 200ms opacity hard limit (max 2 high-impact moments per expression level justify one supporting moment at ~250–350ms). Hard limits are ceiling constraints for default micro-interactions (hover, focus, panel transitions); supporting moments (user-initiated capture, snapshot generation) may exceed hard limits as visual confirmation of significant action.

**Hard limits for 0.3–0.35 expression (desktop-webview, base):**
- NO parallax
- NO scroll animations (scroll-triggered reveals, scroll-driven opacity)
- NO spring physics or bounce easing
- NO staggered reveals beyond component load (no sequential entrance of list items)
- NO 3D transforms
- NO canvas/WebGL (except the dedicated Halo State Pulse layer, which is EXEMPT per WebGPU mandate)
- NO opacity fades longer than 200ms (feels sluggish at this expression level)

**Expression level 0.2 (desktop-native):** All above hard limits inherit to desktop-native surface. Additionally, desktop-native tray menu and notifications are OS-native UI; no custom chrome motion is applied. Only the Halo State Pulse glyph (if composited) may animate per the above rules; all other state changes are instant per OS convention.

**Accessibility:** All motion respects `prefers-reduced-motion` media query. Halo State Pulse degrades to static glow (no pulsing rhythm, but hue still updates per error rate). Other transitions become instant (200ms fade → 0ms, 150ms hover → 0ms).

---

## Iconography

**Style:** custom domain-specific SVGs + fallback monochrome library

**Primary custom set:**
- **aperture** (for "view settings"): the diaphragm of a telescope or microscope, adjusts field of view and focus — parallels data filtering and inspection in the Investigation workflow.
- **telescope** (for "investigate"): the instrument of observational astronomy, enables deep inspection of distant objects — parallels tracing and troubleshooting distributed service calls.
- **constellation-grid** (for "service map"): the navigational grid of celestial coordinates used to locate constellations — parallels mapping and visualizing service topology.
- **star** (for "favorite snapshot"): a reference point in the night sky for navigation and study — parallels bookmarking notable telemetry snapshots for later review.
- **circular-pulse** (for "compact widget / tray surface toggle"): the rhythmic expansion and contraction of the Halo State Pulse glow — parallels toggling between compact and full views.

These glyphs are built into the token system and referenced as `<Icon glyph="telescope" />` (React component registered at `<Root>/src/components/icons/`). Custom SVGs are monochrome (#E8EEF7 primary text color by default, adaptable per state).

**Secondary library:** Lucide or Heroicons as fallback for secondary/generic icons (controls, navigation, validation states). These are NOT the primary visual language — the custom set is. Use library icons only when a domain-specific glyph does not exist.

**Size grid:** 16px (tray icon, inline labels), 20px (buttons, list items, navigation), 24px (card titles, large buttons, hero elements). Padding within containers: 4px (16px icon), 6px (20px icon), 8px (24px icon).

**Rule:** Icons clarify, not decorate. If removing an icon loses no meaning, remove it. In the compact widget (quarter-screen size), every pixel of chrome counts — icons must serve information architecture, not visual filler.

---

## Surface: desktop-webview

**Platform:** Windows (WebView2), macOS (WKWebView), Linux (WKWebView equivalent)

**Toolkit / Framework:** React 19.x (Vite + TanStack Router), Tailwind CSS v4.x, shadcn/ui (Radix UI primitives + Tailwind classes, copy-not-install distribution), WebGPU canvas (`<canvas>` + `navigator.gpu`, WGSL shaders)

### Tokens (platform-specific)

**CSS custom properties** (Tailwind v4 `@theme` configuration):

**Ownership:** setup-project specialist scaffolds the initial Tailwind config file with `@theme` block and the values below; route specialist verifies the file is loaded and accessible to React components via Tailwind v4's `var()` function (e.g., `bg-[var(--color-primary)]`).

```css
@theme {
  /* Colors — derived from Color World */
  --color-primary: #4A90E2;
  --color-secondary: #2C3E7F;
  --color-accent: #8B2E3B;
  --color-base: #1A1D24;
  --color-raised-1: #262A33;
  --color-raised-2: #2D3139;
  --color-raised-3: #343A45;
  --color-inset: #0F1117;
  --color-text-primary: #E8EEF7;
  --color-text-secondary: #B4BCCB;
  --color-text-tertiary: #7D8697;
  --color-text-muted: #56606E;
  --color-feedback-success: #17B3A3;

  /* Spacing — base unit 4px */
  --spacing-micro: 2px;
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;

  /* Border radius */
  --radius-sm: 4px;
  --radius-md: 6px;
  --radius-lg: 8px;

  /* Typography */
  --font-display: 'IBM Plex Sans', sans-serif;
  --font-body: 'IBM Plex Sans', sans-serif;
  --font-code: 'JetBrains Mono', monospace;

  /* Motion */
  --duration-fast: 150ms;
  --duration-standard: 200ms;
  --easing-out: cubic-bezier(0.4, 0, 0.2, 1);
  --easing-in-out: cubic-bezier(0.4, 0, 0.2, 1);
}
```

**Dark mode default:** `prefers-color-scheme: dark` is the default; light mode and auto (follow-system) are Settings options. Toggle via Settings panel, persisted to `~/.andromeda-pulse/config.toml`.

### Component Patterns

**Navigation / App Shell:**
- Compact widget (primary surface): quarter-screen size, custom frameless titlebar with snap-to-edge + always-on-top behavior. Single-window mode (full dashboard expansion via settings icon or keyboard shortcut). Dark background (#1A1D24), minimal chrome.
- Full dashboard: expanded window (2/3 to full screen), same custom titlebar. Tab or sidebar navigation (TBD by Phase 8 layout templates) leading to Traces / Metrics / Logs / Snapshots / Settings views.
- Titlebar (custom-drawn, not OS): 32px height, contains app icon (16px), title ("andromeda-pulse"), settings button (aperture icon, 20px), and window controls (minimize/maximize/close on Windows/Linux, traffic-light buttons on macOS). Drag region: full titlebar width except buttons.

**Cards / Panels:**
- Default: #262A33 background (Raised-1), 1px border rgba(74, 144, 226, 0.3) (subtle), padding space-md (16px). Radius radius-md (6px).
- Hovered: border rgba(74, 144, 226, 0.6) (emphasis), no shadow, no background change.
- Selected/Active: border rgba(74, 144, 226, 0.6) or #4A90E2 (emphasis), text highlight (primary accent text color if semantic).

**Input Fields / Form Controls:**
- Background: #0F1117 (Inset), 1px border rgba(74, 144, 226, 0.3) (subtle).
- Focused: 1px border #4A90E2 (emphasis) + 3px outset box-shadow (0 0 0 3px rgba(74, 144, 226, 0.2)).
- Error state: border 1px rgba(139, 46, 59, 0.5) (Alert Burgundy), error text color #8B2E3B below the input.
- Placeholder text: color #56606E (Muted), font-style italic.
- Padding: space-sm (8px) horizontal, space-xs (4px) vertical (14px font).

**Buttons:**
- Primary (actions, "Investigate", "Generate Snapshot"): background #4A90E2 (Primary), text #1A1D24 (dark), padding space-sm (8px) horizontal + space-xs (4px) vertical, radius radius-sm (4px).
- Hovered: background #5BA5F0 (Earth Blue +10% lightness), transition 150ms ease-out.
- Focused: outline 3px #4A90E2.
- Disabled: background #56606E (Muted), cursor not-allowed, text opacity 0.5.
- Secondary (less emphatic): background #262A33 (Raised-1), text #E8EEF7 (Primary), border 1px rgba(74, 144, 226, 0.3), hovered border #4A90E2.

**Tables (telemetry data):**
- Row background: transparent (parent card background shows through).
- Hovered row: background rgba(74, 144, 226, 0.1) (Primary at 10%), no transition (instant change per 0.35 expression).
- Header: background #1A1D24 (Base), text #E8EEF7 (Primary), font-weight 600, font-size 12px.
- Data cells: font JetBrains Mono, font-size 12px, tabular numerals enabled, padding space-sm (8px).
- Borders: 1px rgba(74, 144, 226, 0.1) (Subtle) between rows.

**Loading / Empty States:**
- Skeleton: background #262A33 (Raised-1), animated opacity pulse (not smooth fade — discrete 50ms on/off per expression 0.3 constraint; use `animation: pulse 1.2s ease-in-out infinite;` with opacity 0.5–1.0).
- Empty state: centered text "No traces yet" or "Snapshot not generated", color #7D8697 (Tertiary), with optional icon (telescope icon, 24px, color #7D8697).
- Error state: error text color #8B2E3B (Alert Burgundy), optional error icon, message on one or two lines.

**Canvas Container (Halo State Pulse, Latency River, throughput counter):**
- Background: #0F1117 (Inset) — recessive, allows glowing halos to pop.
- Canvas dimensions: responsive to container (full width/height in full dashboard, quarter-screen in compact widget). Aspect ratio: free (chart determines shape).
- WebGPU initialization: fallback to `<canvas>` with message "WebGPU not supported in this browser" if `navigator.gpu` is undefined.

**Focus / Keyboard Navigation:**
- All interactive elements (buttons, inputs, links, table rows) are keyboard-navigable via Tab.
- Focus ring: 3–4px outset, color #4A90E2 (Primary), rendered via box-shadow `0 0 0 3px rgba(74, 144, 226, 0.2)` or `outline: 3px solid #4A90E2`.
- Focus visible: outline always shown (`:focus-visible` applies to all focusable elements).

### Navigation Pattern

**Compact widget (primary surface):**
1. Titlebar with settings icon → Settings panel (modal or side drawer, TBD by Phase 8).
2. Halo State Pulse canvas (full widget height) — service constellation with aggregated metrics.
3. Optional: footer bar showing "Ingest: {spans/sec}", "Retention: {min}", "Error rate: {%}" — no interactions, read-only.
4. Keyboard: Esc to minimize/focus loss; Cmd+Shift+P or Ctrl+Shift+P for command palette (TBD); settings icon navigable via Tab.

**Full dashboard:**
1. Titlebar (same as compact).
2. Primary navigation: sidebar or tab bar (TBD by Phase 8) with Traces / Metrics / Logs / Snapshots / Settings.
3. Content area: active view fills remaining space. Each view has its own local navigation (time picker, service filter, etc.).
4. Keyboard: Esc to return to compact widget; global search or command palette via Cmd+K / Ctrl+K.

### Platform-Specific Notes

- **Windows (WebView2):** Custom window controls (minimize/maximize/close buttons) must be rendered in titlebar and positioned according to Windows chrome conventions (right-aligned, 32px from edge). Window frameless via Tauri config: `"decorations": false`. High-DPI handling: React + browser handle `devicePixelRatio` automatically.
- **macOS (WKWebView):** Custom window controls rendered as traffic-light buttons (red, yellow, green) on the left side of the titlebar. Respect macOS keyboard shortcuts (Cmd+Q, Cmd+W, Cmd+H). Dark mode follows system preference via `prefers-color-scheme` media query.
- **Linux (WKWebView):** Custom window controls (minimize/maximize/close) right-aligned as per Windows convention. Support both X11 and Wayland tray protocols (tray icon implementation is the joint responsibility of Tauri + desktop-native surface guide).

**CSP policy:** `script-src 'self'` (no eval, no remote scripts, no CDN). WOFF2 fonts bundled locally. WebGPU shaders (WGSL) are first-party inline or module-imported (no external shader CDN).

**Performance notes:** React 19 Compiler v1.0 auto-memoizes to mitigate VDOM overhead during high-frequency telemetry re-renders. Charts (Halo State Pulse, Latency River) are rendered directly to canvas, NOT inside React's render tree — Arrow data is pushed into canvas via WebGPU compute shader, side-stepping VDOM diffing cost.

---

## Surface: desktop-native

**Platform:** Windows (tray via `NotifyIcon`), macOS (tray via `NSStatusItem` + `NSMenu`), Linux (tray via AppIndicator or StatusNotifier)

**Toolkit / Framework:** Tauri 2.x (native window management via `tauri-plugin-notification`, `tauri-plugin-updater`), custom OS-native menu via Tauri menu builder

### Tokens (platform-specific)

**System theme integration:**
- Icon: single monochrome SVG with macOS template-image flag set; Windows and Linux render the same SVG with system status-bar tint.
- Menu font: OS-native (system-ui). Brand personality is expressed through menu text tone and action labels, not font choice.
- Colors: system tray does not expose custom color control. Halo State Pulse color is rendered as a secondary glow layer around the tray icon glyph (achievable via composited WebGPU canvas in a small window behind the tray, or via SVG+CSS in a hidden overlay — TBD by Phase 8).
- Dark/light mode: follows `prefers-color-scheme` system preference. Tauri 2 `theme()` API exposes the current OS theme; menu can adapt label tone (e.g., "Pause monitoring" vs. "Resume monitoring") based on app state, independent of OS theme.

### Component Patterns

**Tray Icon:**
- Shape: monochrome SVG, 16–22px (platform-dependent). Design: a stylized pulsar or constellation star with a circular outline (the aperture metaphor). Single glyph, no animation in the icon itself.
- Halo State Pulse color encoding: unified halo glow around the badge (throughput rhythm via pulsing frequency 0.8–2.4 Hz, error rate via LCH hue shift Earth Blue ↔ Alert Burgundy). The halo is composited as a secondary layer via WebGPU canvas (matching desktop-webview implementation for visual consistency). If WebGPU is unavailable in the tray context, SVG filters may substitute provided they match visual equivalence: blur radius 4–16 px per pulse cycle, LCH color interpolation fidelity ≥95% of Earth Blue #4A90E2 ↔ Alert Burgundy #8B2E3B shift, opacity 0.6–1.0 envelope matching WebGPU baseline.
- State variants: clicking the icon opens/focuses the compact widget; double-click expands to full dashboard (TBD by phase 8). Right-click or context-menu icon opens the tray menu.

**Tray Menu:**
- Structure: flat hierarchy, groups separated by dividers.
- Top item: "Open andromeda-pulse" or "Focus window" (click focuses/restores the compact widget).
- Summary line (read-only): "Ingest: {spans/sec} | Error: {%} | Retention: {min} used"
- Actions:
  - "Generate Snapshot" (invokes `snapshot.generate` TauRPC, shows "Snapshot ready" notification on completion).
  - "Toggle MCP Server" (only when `--features mcp-server` is built; checkbox-marked if enabled; invokes `mcp.start` / `mcp.stop`).
  - "Open Settings" (focuses the Settings panel in the app).
  - Divider.
  - "Quit" (terminates the process; tray icon disappears).
- Keyboard: all menu items accessible via keyboard (arrow keys, Return to select).
- Tone: contemplative, observational voice. "Generate Snapshot" not "Export Data"; "Toggle MCP Server" not "Enable AI Integration".

**Notifications (OS-native):**
- Trigger: snapshot generation completion, MCP server status change, update available.
- Format: terse, 2 lines max. Example: "Snapshot ready | 2.5k tokens, 42 spans".
- Action button (if OS supports): "View" (opens the snapshot viewer in the app) or "Copy to Clipboard" (direct action).
- Icon: app icon (small).

**Dialog (native file picker, confirmation modals — if used):**
- File picker for snapshot export: native OS dialog (`NSOpenPanel` on macOS, `IFileDialog` on Windows, zenity on Linux). Never custom file browser.
- Confirmation dialog (if any destructive action is added): native modal with standard button ordering (Cancel left / Confirm right on macOS; Confirm left / Cancel right on Windows/Linux).

### Navigation Pattern

**Tray surface is always-visible and passive.** Primary interaction is:
1. Click tray icon → focus/restore compact widget.
2. Right-click tray icon → menu: Generate Snapshot, Toggle MCP, Quit, etc.
3. Notification action → app-initiated navigation (e.g., "View Snapshot" opens the snapshot viewer).

**No keyboard navigation at the tray level** — tray menu is OS-controlled, keyboard interaction is handled by the OS.

### Platform-Specific Notes

- **macOS:** Tray icon uses template-image mode (system auto-tints with the current menu-bar color scheme). Menu items use standard macOS button ordering (Cancel left, Confirm right). Respect Cmd+Q for quit, Cmd+W for close window. Dark mode transitions are instant (no fade animation per 0.2 expression level).
- **Windows:** Tray icon rendered with the system notification-area tint (gray or white depending on accent color). High-DPI support required (125%, 150%, 200%); provide multi-resolution SVG or use vector scaling. Button ordering: Confirm left, Cancel right. Notification Center integration via Windows Toast.
- **Linux:** AppIndicator or StatusNotifier protocol support (per Tauri 2 platform support). GTK+ menu integration with light/dark theme adaptation. XDG desktop integration for autostart settings (in Settings panel, TBD). Support both X11 and Wayland tray protocols.

**CSP policy:** N/A — native surface, no web content.

**Performance notes:** Tray icon is drawn once at startup and updated on state changes (error rate, throughput). No real-time animation in the tray icon glyph itself; Halo State Pulse color (if rendered) is a secondary WebGPU canvas layer, not the icon SVG.

---

## Anti-Patterns (NEVER do these)

### Universal Bans

- **NEVER** use generic font families as primary: Inter, Roboto, Arial, Helvetica, Open Sans, Lato, system-ui default, Space Grotesk. JetBrains Mono + IBM Plex Sans are the locked choices; both bundle locally and neither is on the ban list.
- **NEVER** use purple gradient on white or Tailwind default palette colors as brand identity. The NASA Deep Space Mission Control palette (#1A1D24 / #4A90E2 / #8B2E3B / #E8EEF7 / #2C3E7F / #17B3A3) is the source of truth.
- **NEVER** use gradient overlays or glassmorphic effects on the dashboard chrome. Flat, matte surfaces with intentional color blocking are the discipline. The Halo State Pulse (WebGPU canvas, radial gradient + blur) is the ONLY exception — visual weight is reserved for the signature element, not chrome ornamentation.
- **NEVER** use the same layout for different information types. A metric display (throughput, error rate) ≠ a form (settings) ≠ a data table (traces) ≠ a status page (snapshots).
- **NEVER** use color purely for decoration. Every color in the palette communicates meaning: Deep Control Gray = background envelope; Earth Blue = healthy state; Alert Burgundy = anomaly; Status White-Blue = critical text; Stellar Indigo = secondary structure; Feedback Cyan = confirmation.
- **NEVER** converge on common "safe" choices (sidebar + card grid, hero section → features, standard form-in-a-card). The interface must emerge from the Observatory / Mission Control domain, not statistical patterns in training data.

### Rejected Defaults (from exploration)

- **Uniform monochrome status icons (green checkmark / red X)** — rejected because Halo State Pulse encodes TWO independent dimensions (throughput rhythm + error hue). At-a-glance peripheral detection of both "is the service alive?" (pulsing) and "is the service broken?" (color) simultaneously. Mission Control metaphor: red alert lighting encodes anomaly type and severity via hue, not just on/off.
- **Bright, contrasty color palette (white foreground on dark background everywhere)** — rejected because Status White-Blue (#E8EEF7) foreground on Deep Control Gray (#1A1D24) background has deliberate cool tone and ~8.5:1 contrast (WCAG AAA but slightly reduced vs. #FFFFFF on #000000 pure black). NASA Artemis Mission Control metaphor: text tone aligns circadian rhythm for marathon observation sessions (nocturnal blue accent over warm amber avoids stimulation).
- **Animated state transitions using bounce easing and 500ms+ durations** — rejected because Halo State Pulse is shader-driven and independent of chrome expression budget. Chrome transitions (0.3–0.35) are snap/scale/opacity at 150–250ms (state-confirming, not "delightful"). Contemplative tone rejects delightful framing in favor of state-confirming motion.
- **Sans-serif with mixed weights (regular 400, semibold 600, bold 700) for all hierarchy** — rejected because the architecture commits to Linear reference (Q2) which uses disciplined monospace for data rows (span IDs, trace IDs, latency values) and display sans for headings/controls. Monospace signals "immutable telemetry fact"; sans signals "human-readable summary". This split MUST be encoded in tokens and available to the chart visualization layer.
- **Generic icons from a stock library used across chrome without modification** — rejected because Observatory/Constellation domain-specific glyphs (aperture, telescope, constellation-grid, star, circular-pulse) are built into the token system. Custom SVGs are registered as React components (`<Icon glyph="telescope" />`); Lucide/Heroicons serve as fallback for secondary/generic icons only. Tray icon is an OS-native template glyph (one monochrome SVG with macOS template-image flag); desktop-webview titlebar uses the custom set.

### Per-Surface Bans

**desktop-webview:**
- NEVER ship with visible Chromium/WebView2 artifacts (context menu, developer tools, text selection on non-text elements). Hide right-click context menu; disable text selection on buttons/icons.
- NEVER use web-style scrollbars without styling — they look foreign in a desktop app. Custom scrollbar styling (dark background, Earth Blue thumb) per Tailwind v4 `scrollbar-*` utilities.
- NEVER use browser-style navigation (back/forward buttons, URL bar).
- NEVER use hover-only interactions without keyboard alternatives. All interactive elements must be keyboard-accessible via Tab and focusable.
- NEVER ignore OS-level keyboard shortcuts (Cmd+Q, Ctrl+W, Alt+F4). Tauri 2 respects these by default.
- NEVER make the window non-resizable without strong justification. Compact widget is resizable; full dashboard is resizable.
- NEVER use `alert()` / `confirm()` / `prompt()` — use styled modals (shadcn/ui Dialog component).

**desktop-native:**
- NEVER use web-style design language (cards, shadows, rounded corners) in native menus — it clashes with OS chrome. Tray menu is OS-native; no custom styling.
- NEVER use custom window chrome unless the app is specifically branded. The tray icon is the only visible desktop-native surface; custom chrome is N/A.
- NEVER fight the system font in menu items and notifications. Use OS-native labels.
- NEVER use web notification libraries — use OS-native notifications (NSUserNotification on macOS, Windows Toast, libnotify on Linux).
- NEVER block the UI thread with dialogs. Snapshots generate asynchronously; show notifications for completion, not blocking modals.

---

## Self-Validation Protocol

Before presenting ANY UI output, downstream implementation phases (per project's specialist plans) must run these checks:

### 1. Swap Test
Replace JetBrains Mono + IBM Plex Sans with Inter. Replace NASA palette colors with Tailwind defaults. Replace custom aperture/telescope icons with Lucide defaults. Replace the Halo State Pulse with a generic loading spinner. If the design doesn't feel meaningfully different → you defaulted. Redo with intent.

### 2. Squint Test
Blur your eyes at the interface. Can you still perceive hierarchy (text levels, card elevation, active state)? Does anything jump out harshly? Good craft whispers (Deep Control Gray absorbs stare-time, Earth Blue guides attention without screaming, Alert Burgundy signals anomaly without panic). Nothing should scream.

### 3. Signature Test
Point to the Halo State Pulse in your output. Can you find it in at least 3 places: (1) full dashboard canvas (service dots), (2) compact widget (aggregated badge), (3) tray icon (unified badge)? If you can't locate it in all three → it doesn't exist. Inject it explicitly. **Note:** Tray icon Halo implementation is mandatory per Brand Identity Coherence; visual equivalence criteria in Surface: desktop-native subsection ensure WebGPU or SVG filter fallback produces indistinguishable glow. If Phase 8 implementation cannot materialize Halo on tray, design must provide static halo color fallback (steady glow without rhythm) to satisfy signature presence.

### 4. Token Test
Read your color, spacing, and typography values aloud. Do they trace back to the Color World palette (Deep Control Gray / Alert Burgundy / Earth Blue / Status White-Blue / Stellar Indigo / Feedback Cyan), the spacing scale (2px–32px multiples of 4px), and the font stack (JetBrains Mono + IBM Plex Sans)? Random hex values or magic numbers signal no system.

### 5. Sameness Test
If another AI given a similar prompt ("design a dark telemetry dashboard") would produce substantially the same output — you have failed. The interface must emerge from THIS product's Observatory / Constellation / Mission Control domain exploration, not from statistical patterns in training data. Check: are the icons astronomy-themed? Is the color palette NASA-derived? Is the motion data-driven (Halo pulse frequency = throughput Hz)? Is every interaction explainable via domain metaphor?

---

## Design Decisions Log

`2026-05-02` — Initial design system generated by `/andromeda-design` Phase 4

- **Brand personality:** Ambient constellation — luminous, patient, emergence-driven (Observatory/Mission Control metaphor; contemplative, pattern-seeking voice).
- **Surfaces:** desktop-webview (compact widget + full dashboard, React 19 + Tailwind v4 + shadcn/ui + WebGPU canvas) and desktop-native (tray icon via Tauri 2 `tauri-plugin-notification`).
- **Signature element:** Halo State Pulse — service icon aura encoding throughput (rhythm 0.8–2.4 Hz) and error rate (LCH hue Earth Blue → Alert Burgundy). Lives on dedicated WebGPU canvas layer, exempt from chrome expression budget (0.3 base / 0.35 webview / 0.2 native).
- **Key rejection:** Gradient overlays, generic fonts (Inter/Roboto), uniform monochrome status icons, bounce easing, and "delightful" motion tone. Replaced with: flat surfaces + NASA palette + Halo State Pulse + data-driven state encoding + contemplative motion.
- **Color World locked:** Deep Control Gray #1A1D24 / Alert Burgundy #8B2E3B / Earth Blue #4A90E2 / Status White-Blue #E8EEF7 / Stellar Indigo #2C3E7F / Feedback Cyan #17B3A3 — all derived from NASA Artemis Mission Control mood (user-confirmed in Q3, overriding library-shortlist palette structures).
- **Typography locked:** JetBrains Mono (data, monospace = immutable telemetry fact) + IBM Plex Sans (UI, humanist sans = operator communication). Both bundled locally WOFF2 (CSP-safe, no CDN).
- **Expression level committed:** 0.3 base (subtle, no parallax, no scroll animations, no spring physics). Per-surface: 0.35 webview (skeleton pulsing, panel transitions, 250ms Investigation Capture Collapse signature), 0.2 native (icon state changes, traffic-light colors, badge emergence only). Canvas motion (WebGPU Halo State Pulse, Latency River) is a separate dimension and NOT constrained by chrome budget.
- **Design Direction locked:** Data & Analysis (from library-shortlist) — prioritizes data density, monospace for immutable telemetry, developer-tool aesthetic, minimal ornamentation, pattern-seeking hierarchy.

[Iteration 1] [substantive] Added note to Signature Test: Tray icon Halo is mandatory per Brand Identity Coherence; if Phase 8 cannot materialize animating Halo, static halo color fallback must be provided to satisfy signature presence.

[Iteration 2] [substantive] Added explicit Design Direction statement (Data & Analysis) to Brand Identity section and Decisions Log entry per Library-Shortlist Faithfulness dimension.
