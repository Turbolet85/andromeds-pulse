# layouts extract

## Relevance
relevant — modifies the Trace data table (row ordering + Error-column flag) and adds a filter control within the existing **Full dashboard (Traces view)** on desktop-webview; no new surface/wireframe is created (region = primary content block + footer control band).

## Constraints
- The Traces table is the **primary content block** of the Full dashboard (Traces view) and is a shadcn `Table` (thead + tbody + rows), header row `color-base`, data rows transparent w/ 1px subtle bottom border — anomaly work is ordering + cell rendering *inside* this component, not a new panel. Per layout-templates §Component — Trace data table (Layout).
- An **Error** column already exists as the rightmost column carrying a status glyph (✓/✗) — the semantic error flag rides THIS existing column; do not add a parallel column. Per §Component — Trace data table (Columns → Error).
- Existing sort model: column headers are clickable, cycling ascending → descending → unsorted with bold-text + caret feedback; anomaly-first ordering must reconcile with (not silently break) this user-sortable model. Per §Component — Trace data table (Sorting).
- Row-click is reserved: clicking a trace row triggers the Investigation modal; any new flag/badge affordance must not collide with the row-click → modal gesture, and detail is a modal, not in-table inline expansion. Per §Component — Investigation modal (Trigger) + §Trace data table (Pagination/virtualization note).
- Filters live in the Full dashboard **footer** as "button + dropdown-caret" controls (time-range picker, service filter already present) — the new errors/anomalies filter belongs in this footer control band. Per §Component — Footer (Full dashboard).
- Full dashboard is resizable with **no hard breakpoints**; table + footer must not depend on CSS media-query breakpoints. Per §IA notes — Responsive behavior.

## Patterns to follow
- Reuse the shadcn `Table` structure (header row + transparent data rows, cell padding, row hover) — implement ordering + flagging within it rather than a new component. Per §Component — Trace data table (Layout).
- Extend the existing Error-column **glyph + status** encoding (already non-color-only) for the anomaly flag rather than inventing a new indicator. Per §Component — Trace data table (Columns → Error).
- Model the errors-only filter on the existing footer **service filter** control (button with dropdown caret). Per §Component — Footer (Full dashboard).
- If anomaly-first is expressed as a sort, reuse the column-header sort-feedback affordance (bold text + caret). Per §Component — Trace data table (Sorting).

## Anti-patterns to avoid
- Do NOT add a new column or a separate anomaly panel — the error signal rides the existing Error column and filters ride the footer. Per §Trace data table (Columns) + §Footer.
- Do NOT introduce motion/transition on the flagged row or the filter toggle — table hover is explicitly instant (no transition) and this is a no-new-motion chunk. Per §Trace data table (Hover state).

## Contract bindings
- **Focus order ↔ a11y §Focus Order (SC 2.4.3):** the new filter control is a focusable element added to the footer; its Tab position must match visible reading order (focus-guide cross-domain binding + §Footer).
- **Error flag ↔ design (semantic error token) + a11y (SC 1.4.1 non-color-only):** layout supplies the glyph/badge slot in the Error column; the color token + contrast are owned by design/a11y (§Trace data table Columns → Error). Color tokens are out of layouts scope.
- **Filter control ↔ tests harness (affordance honesty):** scope val-1 requires the control be exercised by a REAL DOM event, so the layout must expose a genuinely operable/focusable footer control, not a state-only proxy (scope Acceptance anchor).

## Acceptance criteria contributions
- (layouts) The errors/anomalies filter renders in the Full dashboard **footer control band**, consistent with the existing service-filter button+caret pattern (layout-templates §Footer — Full dashboard).
- (layouts) The semantic error flag renders **within the existing Error column** of the Trace data table (glyph slot), not as a new column (layout-templates §Trace data table Columns).
- (layouts) Anomaly-first ordering places erroring rows at the **top of the tbody** without disabling the clickable column-header sort model (layout-templates §Trace data table Sorting).
- (layouts) Focus order: the new filter control is reachable via Tab at its visible footer position (layout-templates §Footer; binds a11y SC 2.4.3).

## Relevant amendment history
(none) — no amendment has touched the desktop-webview Trace data table or footer since initial generation (2026-05-02, foundational). The only post-initial entry (2026-06-29 close-to-tray signpost) is a desktop-native OS-notification trigger, outside this chunk's area.
