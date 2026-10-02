# layouts extract

## Relevance
partial — P-063's core (close behavior) is primarily backend/Tauri; P-078 (self-verify) uses existing layout surfaces (window geometry, titlebar, nav) for assertions. Signpost form is layout-open pending P4.

## Constraints
1. Tray surface must remain the single running-state indicator and be reachable after window close — per layout-templates §desktop-native §Tray icon policy; IA notes: "tray icon click (left-click) focuses / restores the compact widget"
2. No new tray menu actions beyond arch reservations — per layout-templates §desktop-native §Tray menu (OS-native) structure; scope.md §Boundaries A: "does not redesign the tray menu surface"
3. Titlebar close button layout unchanged from established component — per layout-templates §Custom titlebar (Flex segments: `[app-icon + title | grow | settings-button | window-controls]`)
4. Window geometry assertions must use P-061's existing structure — per layout-templates §Wireframe notes (compact widget: fixed quarter-screen; full dashboard: resizable) and §IA notes §Responsive behavior
5. Signpost form ("still running" indicator) is layout-open pending P4 decision (OS-notification vs persistent tray tooltip vs tray title change) — scope.md §Open questions Q2
6. Self-verify must reuse existing layout surfaces; no new patterns — per scope.md §Deliverable B ("reuses the a11y/contrast harness"); window geometry + titlebar + navigation element as assert surface

## Patterns to follow
1. Desktop-webview close flow: minimize to tray on titlebar close (extend existing Esc→tray pattern) — per layout-templates §desktop-webview §IA notes ("Esc minimizes the widget to the tray")
2. Tray is the single running-state surface; Quit is the terminate path — per layout-templates §desktop-native §Tray menu (OS-native); menu IA notes: "Quit terminates the entire process"
3. Window placement assertion pattern: `window-geometry.json` structure + centered/remembered logic — per layout-templates §Wireframe notes + §IA notes (compact: fixed quarter-screen; full: resizable, no breakpoints)
4. Shell-health assertions check key interactive surfaces: titlebar visible (layout-templates §Custom titlebar) + navigation element functional (layout-templates §Primary navigation) + data table sortable (layout-templates §Trace data table)

## Anti-patterns to avoid
1. Do NOT add new tray menu actions or redesign tray surface — scope.md §Boundaries A
2. Do NOT widen NEVER-widen capabilities (`pulse:notification`, `pulse:tray`, `pulse:plugin-fs`) without P4 explicit decision — scope.md §Surfaces & contracts touched; xtask capability-widening-check must stay green
3. Do NOT prescribe signpost form before P4 resolves — scope.md §Open questions Q2

## Contract bindings
- a11y domain: focus trap + focus-order SC 2.4.3 if signpost is modal/dialog (per focus-guide bindings §Modal patterns)
- a11y/contrast harness: self-verify composes it; no new layout assertions, reuse existing (scope.md §Deliverable B)
- P-061 window-geometry: self-verify uses its geometry structure for placement assertions (scope.md §Surfaces & contracts touched)

## Acceptance criteria contributions
1. "(layouts) Titlebar close button renders per layout-templates §Custom titlebar component layout (window-controls segment present + functional)"
2. "(layouts) Tray surface visible + reachable after window close; Quit action remains single terminate path — per layout-templates §desktop-native §Tray icon/menu"
3. "(layouts) Self-verify asserts window geometry matches P-061 structure: compact widget fixed quarter-screen, full dashboard resizable — per layout-templates §Wireframe notes"
4. "(layouts) Self-verify asserts key surface renders + responds: titlebar present + navigation element functional + data column sortable — per layout-templates §Custom titlebar, §Primary navigation, §Trace data table"

## Relevant amendment history
- 2026-05-02 — Established Tray icon policy (close→minimize-to-tray; Quit terminates) + Custom titlebar layout (with window-controls) + Tray menu structure (flat, OS-native). Why: Phase 8 foundational design. Body now: per layout-templates §Custom titlebar, §desktop-webview IA notes; §desktop-native §Tray icon, §Tray menu. Handoff note: route specialist owns close-button behavioral implementation.