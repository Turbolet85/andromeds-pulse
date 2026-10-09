# Library Shortlist — andromeda-pulse

## Color Palettes

### 1. Cybersecurity Platform

- **Primary:** #00FF41 | **Secondary:** #0D0D0D | **Accent:** #FF3333
- **Background:** #000000 | **Card:** #0C130E | **Border:** #1F1F1F
- **Match reason:** Deep black environment matches NASA Artemis Mission Control station darkness (#1A1D24). Alert red (#FF3333 vs brief's #8B2E3B) and neon green encoding encode emergency/status states like the Deep Control Gray + Alert Burgundy + Earth Blue palette. Zero distractions, high contrast — matches "ambient constellation" contemplative tone with maximum data readability.

### 2. Financial Dashboard

- **Primary:** #0F172A | **Secondary:** #1E293B | **Accent:** #22C55E
- **Background:** #020617 | **Card:** #0E1223 | **Border:** #334155
- **Match reason:** Deep space black (#0F172A closest to Deep Control Gray #1A1D24). Real-time monitoring dashboard style directly parallels telemetry watching during dev iteration. High-contrast green accent for "active/healthy" state aligns with Earth Blue accent semantics. OLED-optimized for always-on tray icon + long session endurance (matches NASA observation sessions design philosophy).

### 3. Smart Home/IoT Dashboard

- **Primary:** #1E293B | **Secondary:** #334155 | **Accent:** #22C55E
- **Background:** #0F172A | **Card:** #1B2336 | **Border:** #475569
- **Match reason:** Dark tech foundation (#1E293B) with slate greys (#334155, #475569) provides visual hierarchy without introducing fourth hue (preserves mission-control grid discipline). Real-time monitoring + device status pulse aligns with Halo State Pulse metaphor. Supports both full dashboard expansion and compact widget surface via dark mode default.

## Font Pairings

### 1. JetBrains Mono + IBM Plex Sans

- **Mood:** Developer-focused, technical, precise, functional
- **Heading:** JetBrains Mono, 500–600 weight, normal
- **Body:** IBM Plex Sans, 400 weight, normal
- **Google Fonts:** https://fonts.google.com/share?selection.family=IBM+Plex+Sans:wght@300;400;500;600;700|JetBrains+Mono:wght@400;500;600;700
- **Match reason:** JetBrains Mono = data rows (span IDs, trace IDs, latency metrics, ring buffer timestamps) mirrors Linear's monospace discipline + observational exactness. IBM Plex Sans = control panel labels, settings, help text — professional, humanist alternative to ban-listed fonts. Matches domain's "Mission Control console" + "spectroscopy decomposition" (monospace for immutable telemetry facts, sans for operator communication). Linear reference directly endorses monospace for traces.
- **Anti-pattern check:** PASS — neither JetBrains Mono nor IBM Plex Sans is on ban list (Inter, Roboto, Arial, Helvetica, Open Sans, Lato, system-ui, Space Grotesk avoided). Both bundle locally via Google Fonts with WOFF2 support (CSP-safe per security-plan.md).

### 2. Fira Code + Fira Sans

- **Mood:** Dashboard, data, analytics, code, technical, precise
- **Heading:** Fira Code, 500–600 weight, normal
- **Body:** Fira Sans, 400 weight, normal
- **Google Fonts:** https://fonts.google.com/share?selection.family=Fira+Code:wght@400;500;600;700|Fira+Sans:wght@300;400;500;600;700
- **Match reason:** Fira family cohesion — single font family split by purpose (monospace for data, sans for UI). Fira Code's readability at small sizes matches dashboard density requirements. Fira Sans is clean geometric sans (not Roboto, not Inter) with strong personality. Supports full dashboard expansion + compact widget readability at any scale. "Fira Sans is highly readable" supports both webview chrome (settings, titles) and tray icon label rendering.
- **Anti-pattern check:** PASS — Fira Code and Fira Sans not on ban list. Both Google Fonts available with WOFF2.

### 3. Share Tech Mono + Fira Code (HUD variant)

- **Mood:** Tech, futuristic, HUD, sci-fi, data, monospaced, precise
- **Heading:** Share Tech Mono, 400 weight, normal
- **Body:** Fira Code, 400–500 weight, normal
- **Google Fonts:** https://fonts.google.com/share?selection.family=Fira+Code:wght@300;400;500;600;700|Share+Tech+Mono
- **Match reason:** Share Tech Mono (sci-fi retro aesthetic) bridges "ambient constellation" + "mission control console" into visual metaphor — evokes 1970s Apollo era alongside modern telemetry. All-monospace for chart labels, axis text, tooltips encoding the "spectroscopy" decomposition metaphor (Fira Code for body data, Share Tech for display labels). Honors Raycast reference's "minimal chrome, no gradients" via tight monospace typesetting. Direct tie to WebGPU canvas visualization layer (shader-driven labels).
- **Anti-pattern check:** PASS — Share Tech Mono + Fira Code both available on Google Fonts, neither on ban list. CSP-safe WOFF2 bundling.

## Design Direction

- **Primary direction:** Data & Analysis
- **Style preset:** Dark Mode (OLED)
- **Key properties:**
  - **Keywords:** Mission control center, information-rich, organized, dark background, color-coded data, tabular figures, data-optimized typography, bordered definition, dense but structured, heat map gradients
  - **Effects:** Minimal glow (text-shadow sparingly), dark-to-light transitions, low white emission, high readability, visible focus rings
  - **Color focus:** Deep black (#000000 / #121212) background + vibrant neon accents (Earth Blue #4A90E2, Alert Burgundy #8B2E3B mapped to green/red for status). Halo State Pulse adds LCH color interpolation layer independent of chrome.
  - **Depth approach:** Borders + status-indicator colors only (no shadows per Raycast "invisible chrome" reference). Tray icon glyphs use OS-native template SVG (status color via border + halo, not fill).

## Industry Rules

- **Product type match:** Developer Tool / IDE (palette entry #81)
- **Primary style:** Dark Mode (OLED) + Data-Dense
- **Dashboard style:** Real-Time Monitoring + Drill-Down Analytics
- **Color focus:** Dark background + green active indicators + red/burgundy alerts + cool-tinted text
- **Anti-patterns:** Playful design, light mode default, slow rendering, gradient overlays, glassmorphism on chrome (canvas layer exempt)
- **Key considerations:** High contrast for 8+ hour sessions, real-time updates critical, motion budget constrained (0.3 chrome / 0.35 webview per quiz), monospace for immutable data rows, canvas-layer Halo State Pulse independent of chrome expression budget

## UX Guidelines

| # | Guideline | Severity | Platform | DO | DON'T |
|---|---|---|---|---|---|
| 1 | **Reduced Motion** | High | all | Check `prefers-reduced-motion` media query; degrade Halo State Pulse to static glow + hue updates, not pulsing rhythm | Force sine-wave breathing animations ignoring accessibility settings |
| 2 | **Loading Indicators** | High | all | Show skeleton screens or spinners for operations >300ms (telemetry ingest, snapshot generation) | Leave UI frozen with no feedback during trace buffering |
| 3 | **Color Contrast** | High | all | Minimum 7:1 ratio for body text (Status White-Blue #E8EEF7 on Deep Control Gray #1A1D24 = ~8.5:1 WCAG AAA); verify Halo colors on dark background | Low contrast status text; unverified color pairs |
| 4 | **Focus States** | High | all | Visible focus rings (3–4px) on all interactive elements (compact widget buttons, settings toggles, table rows); keyboard navigation for snapshot viewer | Remove focus outline without replacement; rely on visual context only |
| 5 | **Hover vs Tap** | High | all | Click/tap for primary actions (Investigate button, snapshot download); hover changes cursor only | Hover-only interactions on touch surfaces (tray menu expansion) |
| 6 | **Excessive Motion** | High | all | Animate 1–2 key elements per view maximum; Halo State Pulse lives outside React chrome budget per WebGPU layer | Animate everything; parallel pulsing on 5+ icons + panel transitions + canvas halos causes distraction |
| 7 | **Active State** | Medium | all | Highlight active view (Dashboard expanded vs Widget compact) with color shift or underline; show selected snapshot in viewer | All navigation items same style; no feedback on current context |
| 8 | **Duration Timing** | Medium | all | Use 150–300ms for chrome transitions (panel slide, button press); Investigation Capture Collapse 250ms ease-out. Canvas Halo pulse 0.6–1.2s per frequency (data-driven, exempt) | UI animations >500ms; sluggish-feeling interactions |
| 9 | **Transform Performance** | Medium | web | Use `transform: translateX` + `opacity` for panel animations; never animate `width`/`height`/`top`/`left` on compact widget resize | Animate width/height for panel expansion (repaint tax); avoid GPU-accelerated properties |
| 10 | **Content Jumping** | High | web | Reserve space for async telemetry (skeleton state for trace rows); fixed aspect ratio on canvas container | Layout shift when new spans arrive; undefined canvas dimensions |