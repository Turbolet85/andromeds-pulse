# Design Summary

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md` by `/setup-project`. Read on demand._

## Brand identity
- **Personality:** Ambient constellation — luminous, patient, emergence-driven.
- **Domain anchors:** Observatory station / constellation topology / Mission Control console / spectroscopy / ring buffer ephemeris.
- **Aesthetic stance:** Quiet ambient telemetry presence — calm precision of a mission-control station during nocturnal surveillance. Dark surfaces minimize glare; color encodes meaning without ornament; motion is data-driven, never decorative.
- **Design Direction:** Data & Analysis — data-density priority, developer-tool aesthetic, monospace for telemetry, minimal chrome, pattern-seeking visual hierarchy.
- **References (yes):** macOS Activity Monitor compact view, Apple Watch Activity rings, Cleanshot X, Linear app, tldraw, Raycast.
- **References (no):** Datadog / New Relic / Grafana enterprise density; Status Hero / Pingdom marketing-app sterility; Neon / Supabase heavy gradient SaaS.

## Signature element: Halo State Pulse

> **Measured 2026-08-21:** the dedicated WebGPU halo canvas layer does NOT render on desktop-webview (no production render site). The shipped signature there is the constellation DOT carrying the severity hue via `severityToHueFraction`; the tray layer was not probed. Build-or-retire is owned by the "Halo State Pulse canvas disposition" route entry.
WebGPU shader-driven circular animated glow rendered on a dedicated canvas layer around each service constellation dot:
- **Frequency:** breathing period 4–5 s when quiet → ~2 s under active flow (≈0.2–0.5 Hz; driven by activity state, not raw throughput). Opacity + blur modulation only, never scale (P-026). Supersedes the chunk #31-era 0.8–2.4 Hz `throughput_hz / 1000` band per design-system.md Decisions Log 2026-05-29.
- **Hue:** LCH interpolation Earth Blue (`#4A90E2`) ↔ Alert Burgundy (`#C7556A`) by cumulative incident severity.
- **Blur radius:** 4–16 px envelope per pulse cycle (mapped from cumulative incident severity).
- **Connection axis:** connection state grays out / desaturates the halo as an orthogonal axis (P-004), independent of the severity hue.
- **Tray icon variant:** unified halo around aggregated service-count badge.
- **Reduced motion (`prefers-reduced-motion: reduce`):** degrades to static glow; hue still updates per cumulative incident severity.
- **Required presence:** full dashboard constellation map + compact widget aggregated badge + tray icon (3 places).
- **Exempt from chrome budget** (separate WebGPU canvas layer).

## Color palette (NASA Deep Space — locked)
| Role | Hex | Anchor |
|---|---|---|
| Primary | `#4A90E2` | Earth Blue (actions, focus, healthy state) |
| Secondary | `#2C3E7F` | Stellar Indigo (panel backgrounds, secondary nav) |
| Accent | `#C7556A` | Alert Burgundy (errors, anomaly) |
| Success | `#17B3A3` | Feedback Cyan (ISS-cabin lighting) |
| Base | `#1A1D24` | Deep Control Gray (page background) |
| Raised-1 / 2 / 3 | `#262A33` / `#2D3139` / `#343A45` | Cards / popovers / modals |
| Inset | `#0F1117` | Inputs, code blocks, Halo canvas bg |
| Text Primary | `#E8EEF7` | Status White-Blue (~8.5:1 contrast) |
| Text Secondary | `#B4BCCB` | (~6.8:1) |
| Text Tertiary | `#7D8697` | (~4.2:1, large text only) |
| Text Muted | `#56606E` | (~2.1:1, decorative only) |

## Typography (locked)
- **Display / Body / Heading / Label:** IBM Plex Sans (humanist sans = "human-readable summary")
- **Code / Data:** JetBrains Mono (monospace = "immutable telemetry fact"; tabular-nums for data tables)
- Bundled locally as WOFF2 (CSP-safe `script-src 'self'`); no CDN.
- **BANNED primary fonts:** Inter, Roboto, Arial, Helvetica, Open Sans, Lato, system-ui, Space Grotesk.

## Spacing + radius
- Spacing scale (4px base): `micro` 2 / `xs` 4 / `sm` 8 / `md` 16 / `lg` 24 / `xl` 32.
- Border radius: `sm` 4 / `md` 6 / `lg` 8 / `full` 9999px (badges/toggles).
- Geometric, sharp — no organic curves.

## Depth strategy: borders-only
- Default border `1px solid rgba(74, 144, 226, 0.3)` (Earth Blue 30%).
- Focus border `1px solid #4A90E2` + `box-shadow: 0 0 0 3px rgba(74, 144, 226, 0.2)`.
- Error border `1px solid rgba(199, 85, 106, 0.5)`.
- NO shadows on dark surfaces (flatten into darkness, lose psychological weight).
- Modal elevation via `Raised-3` background + 1px border + z-index only.

## Motion (expression level 0.3 base / 0.35 webview / 0.2 native)
- Easing: ease-out for hover/focus; ease-in-out for panel transitions. NO spring physics, NO bounce.
- Durations: hover/focus 150ms; panel transition 200ms; Investigation Capture Collapse 250–350ms (supporting moment, exempt from 200ms hard limit).
- Halo State Pulse: data-driven, exempt from chrome budget.
- **Hard limits (NEVER):** parallax / scroll animations / spring / staggered reveals / 3D transforms / canvas-WebGL except Halo / opacity fades >200ms.
- **All transitions respect `prefers-reduced-motion: reduce`** (becomes instant; Halo → static glow with hue updates).

## Iconography
- **Custom Observatory glyphs:** `aperture`, `telescope`, `constellation-grid`, `star`, `circular-pulse` — built into token system, registered as React components at `pulse-app/ui/src/components/icons/`. Monochrome `#E8EEF7` default.
- **Fallback:** Lucide / Heroicons for generic controls only — NEVER as primary visual language.
- Sizes: 16/20/24px per surface context.
- Rule: icons clarify, never decorate.

## Surfaces

### desktop-webview (React 19 + Tailwind v4 + shadcn/ui + WebGPU)
- **Compact widget (primary surface):** quarter-screen, custom frameless titlebar, snap-to-edge, always-on-top toggle, single-window. Esc minimizes to tray.
- **Full dashboard:** resizable 2/3 to full screen, same custom titlebar, tab or sidebar nav (TBD downstream) — Traces / Metrics / Logs / Snapshots / Settings views.
- **Settings modal:** theme / widget snap position / retention / MCP toggle / snapshot preset / plugin manager.
- **Investigation modal:** triggered by Investigate button (telescope glyph) or trace row click; Capture Collapse motion 350ms.
- Dark mode default (`prefers-color-scheme: dark`); light + auto via Settings.
- CSP: `script-src 'self'`; WOFF2 fonts bundled locally.

### desktop-native (Tauri tray icon + menu)
- **Tray icon:** monochrome SVG glyph (constellation-star/aperture) at 16–22px platform-dependent. Halo State Pulse composited as secondary WebGPU canvas layer (or SVG-filter fallback ≥95% visual equivalence).
- **Tray menu (OS-native, flat hierarchy):** Open andromeda-pulse / read-only summary line / Generate Snapshot / Toggle MCP Server (when feature built) / Open Settings / Quit.
- **OS notifications:** "Snapshot ready ({N} tokens). Paste in {AI tool} to investigate." — 2 lines max via `tauri-plugin-notification`.
- Per-platform conventions honored: macOS template-image flag + traffic-light buttons; Windows NotifyIcon + Action Center toast; Linux AppIndicator/StatusNotifier + libnotify.

## Self-Validation Protocol (run before presenting any UI)
1. **Swap test** — replace fonts/palette/icons/Halo with defaults; meaningful diff?
2. **Squint test** — hierarchy still perceptible at blur?
3. **Signature test** — Halo present in 3 places (full dashboard / compact widget / tray)?
4. **Token test** — values trace to Color World / spacing / font stack?
5. **Sameness test** — would another AI produce same output? Re-anchor to Observatory if yes.
6. **Contrast test** — match Text Hierarchy ratios (Primary ≥4.5:1, etc.)?

Full plan: `.andromeda/design-system.md` + `.andromeda/layout-templates.md`.
