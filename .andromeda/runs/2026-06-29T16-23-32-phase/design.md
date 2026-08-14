# design extract

## Relevance
Partial. This chunk implements the custom frameless titlebar as a drag region and manages window geometry. The design system specifies the titlebar's appearance (dark, flat, minimal chrome), interaction patterns (focus rings, keyboard navigation), and platform conventions. Design constrains the visual + interaction layer; the chunk owns the geometry logic and Tauri binding.

## Constraints
- Per §Brand Identity, the custom titlebar is 32px height, contains app icon (16px), title text, settings button (aperture icon, 20px), and window controls, with dark background and minimal chrome.
- Per §Motion (expression level 0.35), any micro-interactions on titlebar controls (hover, focus) are 150ms ease-out; no additional animated chrome introduced. If transitions exist, they must degrade per `prefers-reduced-motion: reduce`.
- Per §Spacing, titlebar control padding is space-xs to space-sm (4–8px); title text uses Label token (12px, 500 weight, IBM Plex Sans).
- Per §Border Radius, interactive controls within titlebar use radius-sm (4px) for geometric precision.
- Per §Color Palette, titlebar uses --color-base (#1A1D24) background, --color-text-primary (#E8EEF7) text, focus rings #4A90E2 (3–4px outset).
- Per §Depth Strategy, titlebar chrome is flat, matte, borders-only (no shadows, no gradients).
- Per §Iconography, aperture icon (settings) and window controls use custom SVG or platform-native glyphs, monochrome (#E8EEF7 default).
- Per §Surface: desktop-webview / Component Patterns / Navigation, drag region spans full titlebar width except buttons; interactive controls must remain clickable and focusable. Platform-specific: Windows/Linux controls right-aligned, macOS traffic-light left-aligned.
- Per §Anti-Patterns, never use Chromium artifacts (context menu, dev tools); never break focus/keyboard navigation; never ignore OS shortcuts (Cmd+Q, Alt+F4, etc.).

## Patterns to follow
1. Titlebar is a drag region (via `data-tauri-drag-region` or equivalent Tauri affordance) with interactive controls (settings, window buttons) exempted so clicks propagate.
2. All titlebar controls are Tab-navigable and display visible focus rings (#4A90E2, 3–4px outset via box-shadow or outline).
3. Hover feedback on controls (aperture, minimize, maximize, close buttons) uses 150ms ease-out transition to a lighter surface or opacity change (per expression level 0.35); degrades to instant on `prefers-reduced-motion: reduce`.
4. Platform-specific layout: Windows/Linux titlebar buttons (minimize/maximize/close) positioned right-aligned; macOS traffic-light buttons (red/yellow/green) left-aligned; both respect OS window-control placement conventions.

## Anti-patterns to avoid
1. NEVER make the drag region swallow button clicks; interactive controls must remain responsive and focusable.
2. NEVER use gradients, shadows, or glassmorphic effects on titlebar chrome; flat, matte, disciplined surfaces only.
3. NEVER apply hover-only interactions without keyboard alternatives or make controls unfocusable.

## Contract bindings
- Motion transitions (150ms ease-out) bind to a11y §Animation SC 2.3.3 (reduce-motion override mandatory).
- Focus ring color (#4A90E2) on --color-base (#1A1D24) binds to a11y §Contrast (≈7:1, meets SC 1.4.3).
- Keyboard navigation (Tab on all titlebar controls) binds to a11y (all interactive elements must be focusable).

## Acceptance criteria contributions
1. "(design) Titlebar uses only design tokens: --color-base, --color-text-primary, --radius-sm, --space-xs/sm, aperture/window-control icons (no hardcoded hex/pixels)."
2. "(design) Hover/focus transitions (if any) use 150ms ease-out and degrade to instant on `prefers-reduced-motion: reduce`."
3. "(design) Drag region does not prevent titlebar controls from being clicked and focused; focus rings (#4A90E2, 3–4px outset) are visible on all interactive controls."
4. "(design) Titlebar layout respects OS conventions (Windows/Linux: right-aligned controls; macOS: left-aligned traffic-light)."

## Relevant amendment history
(none) — The custom titlebar specification (32px height, drag region pattern, component structure, platform conventions) was locked in the 2026-05-02 initial design system under Surface: desktop-webview § Component Patterns. No amendments touch window chrome or titlebar design. The chunk applies existing design constraints with no plan updates required.