# layouts extract

## Relevance
Relevant — D1 (internal scroll) + D3 (constellation hero) are layout structure and D2 literally refreshes this plan's Traces wireframe; D4 is a11y/render-correctness (not layout).

## Constraints
- Full-dashboard Traces stacks constellation hero (dominant) → sortable data table → footer with local controls; D1's internal scroll must preserve this vertical region order with the hero region fixed on top (per layout-templates §Wireframe — Full dashboard (Traces primary screen) + wireframe notes).
- Full dashboard is resizable with **no hard breakpoints**; the scroll region's `max-height` must adapt to the resizable window, not assume a fixed viewport (per §IA notes → Responsive behavior).
- The trace table is a shadcn `Table` (thead/tbody/rows): header row bg `color-base` + font-label 600, data rows transparent with 1px subtle bottom border, cell padding `space-sm`; wrapping it in a scroll container must not alter this row/cell structure (per §Component — Trace data table → Layout).
- Per-service Halo dots render on each constellation dot on the dashboard Traces hero (signature); D3 collision-avoidance must keep every dot + its always-on label rendered (per §Component — Halo State Pulse canvas → Signature element details; §Signature placement).
- Constellation hero container = `color-inset` canvas, 1px subtle border, `radius-md`, padding `space-md`, fixed hero height — it stays above/outside the table scroll region (per §Component — Halo State Pulse canvas → Layout).
- Pagination/virtualization is TBD-downstream and "investigate" opens a detail view, not in-table inline expansion; the internal scroll is a single-page bounded region, not pagination or row-expansion (per §Component — Trace data table → Pagination / virtualization).

## Patterns to follow
- Keep the shadcn `Table` thead/tbody pattern intact; add only a bounded scroll wrapper around it (per §Component — Trace data table → Layout).
- Fixed hero pattern: `position: relative`, `color-inset` bg, subtle border + `radius-md`, padding `space-md`, fixed hero height — hero sits outside the scroll region (per §Component — Halo State Pulse canvas → Layout).
- Sort-control pattern: column headers clickable, cycle asc → desc → unsorted, text bold + caret in `color-primary` — D4's `handleSort` announce maps to this existing control (per §Component — Trace data table → Sorting).
- Full-dashboard footer pattern: flex-row local controls (time-range picker, service filter as button+caret) + read-only status; the footer is distinct from the Errors-only toolbar and stays below the scroll region (per §Component — Footer → Full dashboard).

## Anti-patterns to avoid
- Do NOT add per-service dots or label collision-avoidance to the **compact widget** constellation — it stays a single unified aggregate halo/badge (per §Component — Halo State Pulse canvas → Signature element details; §Signature placement). D3 is dashboard-hero only.
- Do NOT introduce a hover or scroll-linked transition on table rows — row hover is instant per the `0.35` expression (per §Component — Trace data table → Hover state).
- Do NOT convert the scroll into in-table inline row expansion — investigation opens the Investigation modal / detail view (per §Component — Trace data table → Pagination/virtualization; §Component — Investigation modal).

## Contract bindings
- layouts ↔ a11y (focus order): fixed hero + toolbar above the scroll region must keep visible reading order == focus order (hero → Errors-only/sort toolbar → table rows → footer) — binds a11y §Focus Order SC 2.4.3.
- layouts ↔ a11y (D3 labels): per-dot label placement must preserve the P-069 accessible name + non-color severity token (SC 4.1.2 / 1.4.1) — a11y owns the assertion, layouts owns the non-overlap placement (scope D3).
- layouts ↔ a11y/obs (D4 announce): the Errors-only toggle + sort controls drive `StatusLiveRegion` announcements; the announce-out-of-updater fix is render-correctness, not layout structure — layouts only owns that these controls sit in the toolbar above the table (scope D4).
- layouts ↔ design (§Spacing): the scroll region's `max-height`/`overflow-y` reuses existing spacing tokens (no new token) — binds design §Spacing (scope: "reuse existing tokens").

## Acceptance criteria contributions
- (layouts) The trace table renders in the primary content region of the Full-dashboard Traces surface, below the constellation hero and Errors-only toolbar (layout-templates §Wireframe — Full dashboard (Traces primary screen)).
- (layouts) Internal scroll: the table's scroll wrapper has a bounded `max-height` + `overflow-y:auto`; the constellation hero and Errors-only/sort toolbar remain outside the scroll region and stay visible as rows accumulate (layout-templates §Wireframe notes — Full dashboard + §Component — Trace data table).
- (layouts) The refreshed Traces wireframe shows the "Errors only" filter toolbar above the table plus the fixed-hero / internal-scroll table region (layout-templates §Wireframe — Full dashboard (Traces primary screen)).
- (layouts) Constellation per-dot labels do not overlap/occlude neighbouring dots when dense; per-service dot rendering is preserved on the dashboard hero and the compact widget stays aggregate-glance/unchanged (layout-templates §Component — Halo State Pulse canvas → Signature element details).

## Relevant amendment history
- **Initial 2026-05-02** established the Full-dashboard Traces wireframe, the Trace data table component, and the "full dashboard resizable, no hard breakpoints" responsive posture — the body D2 refreshes and D1/D3 operate within.
- **No amendment has touched the Traces wireframe since initial generation:** P-068 (Errors-only anomaly-first toolbar) and P-069 (per-dot labels) shipped WITHOUT a layout-templates amendment, so the body wireframe is currently stale (footer-only controls, no Errors-only toolbar, no per-dot labels). D2 is the first Traces-wireframe amendment — the D-layout-surface within-surface refinement routine-REJECTED at the P-068 wrap, homed here.
- **2026-07-07-plain-language-connection-status (P-070)** added the full-dashboard footer `ConnectionStatusLine`, which sits below the table — D1's internal scroll must leave the footer + connection line intact (scope: no ConnectionStatusLine change).
- **2026-07-08-self-explaining-empty-states (P-071)** scoped the shared `EmptyState` to Metrics/Logs/Snapshots data views, explicitly NOT Traces — confirms D1 adds no empty-state work to the Traces table (scope boundary).
