# design extract

## Relevance
Partial — This chunk implements two webview anti-pattern suppressions specified in design-system §Anti-Patterns (desktop-webview), but adds no visual design.

## Constraints
1. Per design-system §Anti-Patterns (desktop-webview): "NEVER ship with visible Chromium/WebView2 artifacts (context menu, developer tools, text selection on non-text elements). Hide right-click context menu." (scope: P-064 context-menu suppression, production-gated)
2. Per design-system §Anti-Patterns (desktop-webview, canvas artifacts): do not expose canvas elements as browser-draggable/saveable images (scope: P-065 canvas drag-affordance suppression)
3. Per design-system (Surface: desktop-webview, development preserve): dev builds must retain Inspect/devtools access and right-click debugging (suppression is production-only gate, `import.meta.env.PROD`)
4. Per design-system (Surface: desktop-webview, Input Fields): editable form controls (Settings inputs) must preserve native copy/paste/selection interaction (scope ambiguity: p4 resolution on whether context-menu suppression exempts editable elements)

## Patterns to follow
1. Production-gated suppression using Vite `import.meta.env.PROD` to preserve dev inner loop (per anti-pattern preservation intent).
2. Global `contextmenu` handler installed at webview root (app bootstrap or root effect) covering both dashboard and compact-widget entry points.

## Anti-patterns to avoid
1. Suppressing context menu and canvas affordances without production gating — degrades dev ergonomics (breaks Inspect access and dev debugging).
2. Exposing canvas as draggable/saveable browser image (conflicts with anti-pattern intent F5).
3. Disabling legitimate text selection/copy in editable form fields — breaks accessibility and UX (a11y binding).

## Contract bindings
a11y ↔ text interaction — suppression must not impair copy/paste/selection in editable contexts (a11y §Keyboard Navigation + Use of Color).

## Acceptance criteria contributions
1. (design) Context menu suppression is production-gated; dev builds preserve Inspect/devtools (per anti-pattern intent F4).
2. (design) Canvas elements expose no image-save/drag affordance (per anti-pattern intent F5).
3. (design + a11y) Editable form fields retain native copy/paste capability (a11y binding on text interaction preservation).

## Relevant amendment history
(none) — design-system §Anti-Patterns (desktop-webview, context-menu and canvas artifact bans) is current truth; no amendments touch this area. Guidance pre-dates this chunk.
