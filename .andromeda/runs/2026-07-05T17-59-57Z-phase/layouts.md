# layouts extract

## Relevance
Relevant (partial) — the constellation-canvas hero, per-dot label placement, canvas-vs-DOM-overlay layer, and focus order are in-domain; the palette/Halo hue mapping is design and WCAG SC 1.4.1/1.4.11 are a11y.

## Constraints
- The constellation dots + per-service Halo render inside the **Halo State Pulse canvas**, which is the hero/signature section of the Full dashboard Traces view — positioned above the trace data table, `color-inset` bg, subtle border + `radius-md`, `padding space-md`, fixed hero height (per plan §Component — Halo State Pulse canvas → Layout + §Wireframe — Full dashboard (Traces primary screen)).
- Per-service Halo **dots** are the signature placement on the full-dashboard Traces view (distinct from the compact widget's single unified badge); this is the dot set the chunk labels (per plan §Signature placement + §Component — Halo State Pulse canvas → Signature element details).
- Dot positions are computed from **service topology via the viz crate**, not hand-placed; the `<canvas>` fills the relative container with devicePixelRatio scaling handled by React+browser (per plan §Component — Halo State Pulse canvas → Canvas rendering).
- Reduced motion: the per-dot Halo degrades to a **static glow** while hue still updates per error/severity — matches the chunk's reduced-motion boundary (per plan §Component — Halo State Pulse canvas → Reduced motion).
- The compact widget is **fixed quarter-screen with no CSS media-query breakpoints** and the full dashboard is resizable with no hard breakpoints — so any density-driven label behavior (scope open-question 1 hybrid) adapts to content/zoom, not to media queries (per plan §IA notes → Responsive behavior).
- The compact widget shows a **single unified aggregated badge** ("N services, avg X% error"); per-dot labeling is a full-dashboard concern only (per plan §Wireframe — Compact widget + §Signature element details).

## Patterns to follow
- **Canvas-as-hero with relative container**: `position: relative` wrapper, `<canvas>` fills it — a positioned DOM overlay label layer (if scope open-question 2 picks DOM over canvas text) sits above the `<canvas>` within this same relative container (per plan §Component — Halo State Pulse canvas → Layout).
- **Data-driven per-service encoding**: halo frequency/hue derive directly from that service's throughput/error rate from the query result; labels + the health cue should likewise be derived per-service, not decorative (per plan §Component — Halo State Pulse canvas → Signature element details).
- **Status-glyph precedent for a non-color cue**: the Trace data table Error column already pairs a shape glyph (✓/✗) with hue — the existing in-plan precedent for scope open-question 4's non-color signal form (per plan §Component — Trace data table → Columns / Error).
- **Multi-surface Halo consistency**: compact widget + full-dashboard Traces + tray share one Halo shader logic (frequency clamp, LCH interpolation) — a per-dot label/health scheme must not fork that shared shader (per plan §IA notes → Multi-surface coordination).

## Anti-patterns to avoid
- Do not add per-dot labels to the **compact widget** — it is a single-badge glance surface, not a labeled map (per plan §Wireframe — Compact widget).
- Do not introduce **CSS media-query breakpoints / reflow** to drive label density on the compact widget (fixed quarter-screen, no reflow) (per plan §IA notes → Responsive behavior).
- Do not **hard-code dot/label coordinates** — labels attach to the viz-crate-computed constellation positions (per plan §Component — Halo State Pulse canvas → Canvas rendering).

## Contract bindings
- **Focus order ↔ a11y**: the canvas-vs-DOM-overlay label choice (scope open-question 2) is a layout decision that directly drives a11y — canvas-drawn text is not screen-reader-reachable/selectable; a DOM overlay's tab order must match visible reading order (binds a11y §Focus Order SC 2.4.3; plan §Component — Halo State Pulse canvas → Layout).
- **Non-color cue form ↔ a11y**: SC 1.4.1 is a11y's requirement, but the component/placement form of the cue (label text vs shape/icon/severity token) is a layout call, reusing the Trace-table status-glyph pattern (plan §Component — Trace data table).
- **Dot/label color + Halo ↔ design**: the LCH hue mapping (`color-primary`→`color-accent`) and Halo treatment are design tokens; layout only places the dot + label (plan §Component — Halo State Pulse canvas).
- **Label spacing ↔ design**: on/near-dot label offsets use design spacing tokens (`space-sm`/`space-md`).

## Acceptance criteria contributions
- (layouts) Per-service dots + their labels render in the Halo State Pulse canvas hero region of the Full dashboard Traces view, above the trace data table (layout-templates §Wireframe — Full dashboard (Traces) + §Component — Halo State Pulse canvas).
- (layouts) Each dot's label is anchored to that dot's viz-crate-computed constellation position, not hand-placed (layout-templates §Component — Halo State Pulse canvas → Canvas rendering).
- (layouts) If labels are a DOM overlay, they sit in a positioned layer above the `<canvas>` inside the relative canvas container and their focus order follows visible reading order (layout-templates §Component — Halo State Pulse canvas → Layout; binds a11y §Focus Order).
- (layouts) The compact widget still shows only the single unified aggregated badge — no per-dot labels leak onto it (layout-templates §Wireframe — Compact widget + §Signature element details).

## Relevant amendment history
- **2026-05-02 — Initial layout templates (Phase 8)**: established the full-dashboard constellation map hero with **per-service Halo dots** (vs unified halo on the compact-widget badge) and recorded the Halo State Pulse build-ownership handoff — the route + obs specialist owns canvas init, shader authoring, data-feed subscription, and reduced-motion degradation. Relevant because this is the foundational per-service-dot layout truth this chunk extends with labels + a non-color health cue; any label/health work rides the existing canvas + shared-shader ownership rather than re-placing the hero. (The 2026-06-29 Notifications-trigger amendment is unrelated to the constellation.)
