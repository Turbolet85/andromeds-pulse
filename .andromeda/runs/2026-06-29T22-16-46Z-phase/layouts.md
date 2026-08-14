# layouts extract

## Relevance
relevant — P-062 constrains the compact widget's resize behavior; must respect the existing layout's minimum readable size and fixed-proportions character.

## Constraints
1. Compact widget is a **fixed quarter-screen glance surface** (layout-templates §Compact widget wireframe) — constraints must preserve the fixed-size design intent, not enable media-query-driven reflow
2. **Minimum recommended size stays readable on a 13-inch laptop** (layout-templates §Responsive behavior) — establishes the conceptual floor for min-width / min-height config
3. **No CSS media-query breakpoints** on compact widget (layout-templates §Responsive behavior) — size constraints are static Tauri config, not responsive CSS rules
4. Custom titlebar height is **space-lg** (layout-templates §Custom titlebar) — min-height must accommodate titlebar + canvas + footer without reflow
5. Halo State Pulse canvas fills container at **width 100%, height 100%** (layout-templates §Halo State Pulse component) — aspect-ratio bound must maintain readable proportions for signature element
6. Footer status bar height is **space-sm** (layout-templates §Footer compact widget) — included in the minimum-size stack

## Patterns to follow
1. Use Tauri 2 native window API for aspect-ratio / min-size (rust-side, not webview CSS) per scope's research item on Tauri 2.x capabilities
2. Aspect-ratio bound is static fixed constant, matching the "fixed quarter-screen" IA posture (no user-tunable per scope)
3. Constraint values derived from space-* token sizes (titlebar space-lg + footer space-sm bounds the min-height floor)

## Anti-patterns to avoid
1. Do NOT introduce CSS media-query breakpoints for compact widget (layout-templates §Responsive behavior commits to fixed no-breakpoints design)
2. Do NOT make min-size / aspect-ratio user-configurable (scope: a sane fixed constant is the floor, not a user setting)

## Contract bindings
None — P-062 does not add focusable elements (focus order N/A), does not create modals (modal patterns N/A), does not add CSS breakpoints (responsive breakpoints design is already closed).

## Acceptance criteria contributions
- "(layouts) Compact widget titlebar + canvas + footer stack survives min-size constraint without reflow (layout-templates §Custom titlebar + Halo State Pulse component + Footer)."
- "(layouts) Aspect-ratio constraint maintains Halo canvas readability during resize (layout-templates §Halo State Pulse component width/height 100% fill requirement)."

## Relevant amendment history
(none) — P-062 is the first size-constraint amendment. Prior amendment (2026-06-29-predictable-close-self-verify, P-063) is orthogonal (close behavior, not resize).
