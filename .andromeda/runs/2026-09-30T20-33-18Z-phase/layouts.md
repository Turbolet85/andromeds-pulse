# layouts extract

## Relevance
partial — the chunk creates or restructures no surface, region, focusable element, modal, nav or breakpoint. It changes only the TEXT that existing read-only content regions render: span-masked digest, report, snapshot and attribute text in place of a single whole-value placeholder. The layout exposure is limited to how those regions absorb longer mixed content.

## Constraints
- The Report window is a fixed surface: the six-section `Report` in the `Modal` `fill` variant, edge-to-edge with no backdrop, positioned relative to the findings window (per layout-templates §Surface: desktop-webview → Primary screens, "Report window"). Span masking may change what those sections contain, but it must not add, remove or reorder sections, change the window's variant, or change its positioning. Whether any section's rendering branches on "value was wholly redacted" today is a question for P3 research.
- The Investigation modal renders the structured trace detail (span tree, attributes, events) inside a scroll region, and the card is capped at about 80% of the viewport with scrollable overflow (per layout-templates §Component — Investigation modal). Attribute and event values that change from a short `[REDACTED:…]` or `[redacted:…]` cell to longer span-masked text must be absorbed by that scroll region. They must not widen the card past its cap or make the page scroll.
- The Traces table keeps a bounded internal scroll region (`flex:1 / min-height:0 / overflow-y:auto`), and the dashboard never grows an outer page scrollbar (per layout-templates §Component — Trace data table, "Internal scroll region"). Any ring-buffer cell it displays that now carries span-masked text is bound by the same containment.
- The Snapshots view shows generated snapshots with view/copy actions; the snapshot-detail viewer layout is deferred to the implementation route (per layout-templates §Surface: desktop-webview → Primary screens, "Full dashboard (Snapshots view)"). Longer snapshot bodies change content length only, not that list's structure.
- The chunk adds no interactive affordance: the placeholder is inert text, not a reveal or expand control. That leaves every surface's focus order unchanged, including the Traces table's roving-tabindex single tab stop (per layout-templates §Component — Trace data table, "Keyboard / focus").

## Patterns to follow
- Content overflow goes to the component's OWN bounded scroll region, never to page growth (per layout-templates §Component — Trace data table, "Internal scroll region", and §Component — Investigation modal, "Layout").
- Error and empty variants show static copy and never raw internals (per layout-templates §Component — Empty / error state). A redaction placeholder is data inside a populated view. It is not an empty or error state and must not route into the `EmptyState` branch.
- The Report reuses the shared `Modal` `fill` variant rather than a bespoke container (per layout-templates §Surface: desktop-webview → Primary screens, "Report window"). Text-content changes stay inside the existing section components.

## Anti-patterns to avoid
- Do not add an in-line "reveal redacted" or expand control, or any new focusable element, next to a masked span. That would be a new interactive affordance and would change focus order (per layout-templates §Component — Trace data table, "Keyboard / focus"). It is also outside a chunk that changes only what gets replaced.
- Do not let longer span-masked content push a bounded region into outer page scroll (per layout-templates §Component — Trace data table, "Internal scroll region").

## Contract bindings
- layouts ↔ security: the placeholder spelling that renders inline (`[REDACTED:{category}]` vs `[redacted:{category}]` / `[redacted: {category}]`) is security's P4 decision, per the scope's Boundaries. Layout has no requirement on the spelling, only that it renders as inert inline text inside existing regions.
- layouts ↔ a11y: no new focusable element means focus order (SC 2.4.3) is unchanged on every surface. How a screen reader announces an inline placeholder is a11y's domain, not layout's.

## Acceptance criteria contributions
- (layouts) The Report window still renders its six sections in the `Modal` `fill` variant, in the same order and position, when a digest carries one masked span among intact lines. This is content-only change with no structural delta (per layout-templates §Surface: desktop-webview → Primary screens, "Report window").
- (layouts) An Investigation-modal attribute or event carrying span-masked text renders inside the modal's scroll region, with the card still capped at about 80% of the viewport and no outer page scroll (per layout-templates §Component — Investigation modal).
- (layouts) The chunk introduces no new focusable element on any desktop-webview surface, so the Traces table stays one tab stop (per layout-templates §Component — Trace data table, "Keyboard / focus").
