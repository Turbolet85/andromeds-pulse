# layouts extract

## Relevance
Partial — The chunk adds an in-widget affordance to the compact-widget surface (desktop-webview), touching layout placement and keyboard navigation; the full-dashboard target is out-of-scope (no layout changes there); desktop-native/tray is out-of-scope.

## Constraints
1. Per layout-templates §Compact widget (wireframe): The affordance must fit within the fixed quarter-screen, frameless window layout; the titlebar, canvas, and footer are immutable regions.
2. Per layout-templates §Component - Halo State Pulse (canvas layout): The affordance must not overlay, clip, or disrupt the signature canvas fill (100% width/height of remaining viewport).
3. Per layout-templates §Responsive behavior: The compact widget is a fixed quarter-screen size with no CSS media-query breakpoints; the affordance must not introduce responsive reflow or breakpoint logic.
4. Per layout-templates §IA notes - Navigation model (compact widget): "Minimalist chrome" is the design intent; the affordance must be simple, clean, and must not add bloat (e.g., no nested menus or multi-level nav).
5. Per layout-templates §IA notes - Keyboard shortcuts: Esc minimizes the widget to tray; the affordance must not conflict with this binding.
6. Per layout-templates §Component - Custom titlebar: If the affordance is a titlebar button, it must follow the titlebar flex-row structure, hover states (color-text-secondary → color-text-primary over duration-fast), and fit the `[app-icon + title | grow | settings-button | window-controls]` flex segmentation.

## Patterns to follow
1. Per layout-templates §Component - Custom titlebar: Settings gear icon triggers a modal; the affordance (button-shaped control) can mirror this modal/action trigger pattern.
2. Per layout-templates §Component - Footer (compact widget): Read-only footer uses flex row with space-between, padding space-md, font-label, color-text-tertiary; if the affordance is footer-adjacent, follow this visual structure.
3. Per layout-templates §IA notes - Primary navigation (compact widget): Settings control is the only interactive affordance in the widget (settings gear → modal); any new affordance must integrate cleanly with this existing chrome (no duplication or visual conflict).

## Anti-patterns to avoid
1. Per layout-templates §Responsive behavior: Do NOT introduce CSS media-query breakpoints or conditional layout reflow — the widget is fixed quarter-screen regardless of viewport.
2. Per layout-templates §Component - Halo State Pulse (canvas): Do NOT overlay interactive elements on top of the canvas or obscure its rendering with additional chrome layers.
3. Per layout-templates §IA notes - Navigation model: Do NOT add nested submenus, dropdown hierarchies, or tab bars to the compact widget (those are full-dashboard only per the IA model).

## Contract bindings
**a11y** — Focus order binds to a11y §Focus Order SC 2.4.3 (visible reading order matches focus order); the affordance must be Tab-reachable in a predictable position within the compact widget's focus chain.

## Acceptance criteria contributions
1. "(layouts) Affordance renders in a defined, visible region of the compact-widget without overlaying the Halo State Pulse canvas (layout-templates §Compact widget wireframe + §Component - Halo State Pulse)."
2. "(layouts) Affordance is keyboard-reachable via Tab and activatable via Enter or Space (layout-templates §IA notes Keyboard navigation)."
3. "(layouts) Compact widget maintains fixed quarter-screen geometry with no CSS breakpoint reflow (layout-templates §Responsive behavior)."

## Relevant amendment history
(none) — No prior amendments to layout-templates affect this chunk's area (widget-to-dashboard navigation affordance placement). The 2026-05-02 initial generation established the compact widget's immutable fixed quarter-screen constraint, but is not recorded as an amendment. The 2026-06-29 amendment (close-to-tray notification signpost) is outside this chunk's scope (notifications, not navigation).
