# layouts extract

## Relevance
Partial — this chunk creates/modifies no surface, region, component, focus target, modal, nav entry, or breakpoint; the only layout tie is the desktop-webview **Metrics data view**'s rendered outcome (empty vs. error region) and any label-half change to the `viz` metrics response shape.

## Constraints
- The Metrics view is a full-dashboard primary screen that **shares the Traces header/nav structure**; a data-layer change must not introduce view-specific chrome (per layout-templates §Surface: desktop-webview → Primary screens, "Full dashboard (Metrics view)").
- A zero-row metrics read renders through the **shared `EmptyState` component** in the primary content region, in place of the chart — one component backs Metrics/Logs/Snapshots so the data views read as one system (per layout-templates §Component — Empty / error state (desktop-webview data views)).
- The plan requires the **error variant be checked BEFORE the empty branch**, so a query failure "never masquerades as 'no data'" — no raw `AppError`, no exporter hint on the error path (per layout-templates §Component — Empty / error state → Error). This chunk's whole-batch flush rejection produces a zero-row read that is *not* a genuine "no metrics received" condition; **whether the Metrics view's current branch order already distinguishes a rejected batch from true zero-data is research's question.**
- The empty message is shown **only on a settled zero-data load** (never flashed while loading, hidden once populated) and names the OTLP receiver ports in `font-code` (per layout-templates §Component — Empty / error state → Empty). Any change to the metrics read shape must preserve that settle-then-show gating.
- The empty/error region has **no focusable element** (read-only, affordance-exempt), so this chunk owes no focus-order position (per layout-templates §Component — Empty / error state → Layout).
- The full dashboard is **resizable with no hard breakpoints** and its views are routable at `/metrics`; no responsive or routing work is owed here (per layout-templates §IA notes → Responsive behavior, Deep-link convention).

## Patterns to follow
- **Bounded primary-content region**: the dashboard route is a flex column bounded to the shell's `main` height with `overflow:hidden`, fixed regions above and the scrolling region flex-filling the remainder — the established precedent for any added metrics column/list (per layout-templates §Wireframe — Full dashboard (Traces primary screen); §Component — Trace data table → Internal scroll region).
- **One shared empty/error component across the three data views** rather than a per-view state (per layout-templates §Component — Empty / error state → Applies to).
- **Column role assignment on data tables**: identifiers in `font-code`, numerics in `font-data` tabular right-aligned — the shape a label/dimension column would inherit if the label half lands this-chunk (per layout-templates §Component — Trace data table → Columns).
- **Worded read-only status in the footer** (`ConnectionStatusLine`) as the existing place a user learns whether data is arriving, distinct from an empty chart (per layout-templates §Component — Footer → Full dashboard).

## Anti-patterns to avoid
- Rendering a dropped/rejected batch as the ordinary empty state with the exporter hint — a failure presented as "no data" is explicitly banned (per layout-templates §Component — Empty / error state → Error).
- Surfacing raw error text (`AppError`) in the data view region (per layout-templates §Component — Empty / error state → Error).
- Adding a Metrics-specific nav entry or altering the fixed five-entry tab/sidebar structure (per layout-templates §Component — Primary navigation → Tab / sidebar structure).

## Contract bindings
- **Empty/error copy ↔ a11y**: the plan itself cross-cites SC 1.4.3 for body-size text (`color-text-secondary`, not `color-text-tertiary`) — contrast is a11y's call, the region placement is layouts' (per layout-templates §Component — Empty / error state).
- **Empty/error copy ↔ design**: token choices (`color-text-secondary`, `color-accent`, `font-body`, `font-code`) are design-owned; layouts owns only where the region sits and its branch order.
- **Scope's "the ordinal makes the label loss SILENT" ↔ layouts**: the scope records that after the fix both colliding rows land instead of failing the batch. That flips the Metrics view's rendered outcome from empty/error toward populated-but-indistinguishable rows — a layouts-visible consequence of a data-layer decision, and the reason the error-before-empty rule (above) matters to this chunk.

## Acceptance criteria contributions
- (layouts) No new surface, region, component, or focus-order position is introduced; the desktop-webview Primary screens list and the five-entry navigation are unchanged (per layout-templates §Surface: desktop-webview → Primary screens; §Component — Primary navigation).
- (layouts) A metrics read that returns zero rows because a batch was rejected at flush is not rendered as the "No metrics received yet" empty state with the `:4318` / `:4317` exporter hint — the error variant is evaluated first (per layout-templates §Component — Empty / error state → Error). Whether the code already satisfies this is research's question.
- (layouts) If the label half lands this-chunk and changes the `viz` metrics response shape, any added column/field renders inside the existing bounded primary-content region — no outer page scrollbar at any window size (per layout-templates §Wireframe — Full dashboard (Traces primary screen); §Component — Trace data table → Internal scroll region).

## Relevant amendment history
- **2026-07-08-self-explaining-empty-states** — added §Component — Empty / error state for the Metrics/Logs/Snapshots views (shared `EmptyState`, exporter hint, and the distinct honest-error variant checked first). Why it matters here: it is the only layout section that governs how this chunk's zero-row / rejected-batch outcome reaches a user, and the honest-error rule was added precisely so a failure cannot read as "no data".
- **2026-07-09-traces-table-layout-polish** — registered the bounded-route / fixed-hero / internal-scroll table region and the anomaly-first + `Errors only` toolbar (P-082 / P-068). Why it matters here: it defines the primary-content region contract any metrics column or label rendering would inherit, and it recorded the sketch-lag handoff discipline (illustrative ASCII may lag current truth — do not treat wireframe detail as binding where the §Component text is newer).
- **2026-07-07-plain-language-connection-status** — dashboard footer `ConnectionStatusLine` (worded connected-source count, spans/s, buffer fill). Adjacent only: it is the existing worded "is data arriving" readout, and it is spans-keyed, so this chunk changes nothing there — noted so it is not mistaken for a metrics-side obligation.
