# layouts extract — phase-66

## Chunk relevance

Chunk #69 Phase B is partially in-domain for layouts. The chunk introduces a **Settings → Diagnostics "Template Distribution" panel** (`pulse-app/ui/diagnostics/TemplateDistribution.tsx`) — a new UI surface inside the full dashboard. The backend algorithm, ingestion integration, persistence, and TauRPC procedure registration are out-of-domain for layouts. Only the new panel's placement, density conventions, and data table patterns are in-scope.

## Constraints

**Full dashboard — tab nav and panel placement**
Per layout-templates.md §Full Dashboard Shell: new content panels must land inside an existing tab or a new tab in the tab nav (Traces / Metrics / Logs / Snapshots / **Settings** / future). A "Diagnostics" sub-surface under Settings tab is the natural placement; it should not introduce a new top-level tab unless the specialist plan explicitly authorizes it (layout-templates.md §Full Dashboard Shell is silent on a dedicated Diagnostics tab — orchestrator may need to surface this to user).

**Density scale — chart-first, low-chrome**
Per layout-templates.md §Design Philosophy anchor (dense, chart-first, low-chrome dashboard): the template distribution panel displays top-50 templates with occurrence counts and sample messages. The table must use the project density scale (compact row height, monospace font for sample messages per JetBrains Mono token, truncated cells with tooltip expansion) — not a spacious consumer-app layout.

**Settings modal form — restart-required notice surface**
Per layout-templates.md §Settings Modal Form: Drain params (depth / similarity / max_clusters) loaded from config require a "restart-required" notice surface (capability P-055). The layout pattern for restart-required notices in the Settings modal must follow the existing settings form conventions — inline notice adjacent to the affected field group, not a modal-over-modal or toast.

**Data table layout — top-N template list**
The TemplateDistribution panel is fundamentally a data table (top-50 rows). Per layout-templates.md §Full Dashboard Shell §Trace Timeline: sortable data tables in the dashboard use the established column layout (fixed-width ID/count columns, flex-grow name/sample column). The template distribution table should follow this precedent: Template ID / Occurrence Count / Drift Indicator as fixed-width columns; Sample Message as flex-grow truncated column.

## Patterns to follow

- **Settings sub-panel pattern** (layout-templates.md §Settings Modal Form): Drain config fields (depth, similarity, max_clusters) follow the existing settings form field group pattern — labeled input + inline constraint hint + restart-required badge. No new layout primitives needed.
- **Sortable data table inside dashboard view** (layout-templates.md §Trace Timeline + per-service constellation): Template distribution table reuses the sortable-table column pattern already established for trace data (Trace ID / Service / Latency / Error columns → Template ID / Count / Drift / Sample).
- **Modal primitive for expanded sample** (layout-templates.md §Modal Primitive Scaffold): if the panel needs to show a full-length sample message, use the existing modal primitive (overlay card, focus trap, aria-busy/aria-live) rather than an inline expansion that disrupts the table layout.

## Anti-patterns to avoid

- **New top-level tab for Diagnostics** without explicit layout plan authorization — the full dashboard tab nav (Traces / Metrics / Logs / Snapshots / Settings) is fixed in layout-templates.md §Full Dashboard Shell; a new tab requires a layout spec amendment.
- **Spacious consumer-app table layout** — wide padding, large row heights, card-per-row — contradicts the density constraint (layout-templates.md §Design Philosophy: "dense, chart-first, low-chrome dashboard targeting developers").
- **Toast for restart-required notice** — restart-required notices for config params belong inline in the settings form adjacent to the field, not as a transient toast (layout-templates.md §Settings Modal Form + OS notification policy cross-constraint).
- **Inline full sample message in table cell** without truncation — long log template sample messages will break the fixed-height table row density; truncate with tooltip or modal expansion only.

## Contract bindings

- **Design domain**: the TemplateDistribution panel consumes NASA Deep Space palette tokens and JetBrains Mono for sample message cells. Layouts domain defers color and motion token selection to the design plan; the constraint here is that the table uses monospace tokens for sample message columns (established design-system convention for code/log content).
- **A11y domain**: the data table requires proper `<thead>` / `<tbody>` semantic structure, sortable column aria-sort attributes, and keyboard-navigable rows — a11y extractor owns these constraints; layouts domain only specifies the visual density shape.
- **Settings restart-required notice**: the restart-required badge/notice positioned inline in the Drain config field group creates a cross-domain binding with the obs plan (P-055 capability) and the design domain (badge token / color). Layouts owns the placement rule (inline, adjacent to field group); design owns the badge visual token.

## Acceptance criteria contributions

- **AC-L1**: The Template Distribution panel is accessible under the Settings tab (or a Settings sub-page) in the full dashboard — no new top-level tab introduced without explicit layout plan amendment.
- **AC-L2**: Template distribution table uses compact row height and monospace font (JetBrains Mono) for the Sample Message column; rows are truncated at single-line height with tooltip or modal expansion for overflow content.
- **AC-L3**: Sample Message column is flex-grow; Template ID / Occurrence Count / Drift Indicator columns are fixed-width — matches the established sortable-table column layout pattern (layout-templates.md §Trace Timeline).
- **AC-L4**: Drain config fields (depth / similarity / max_clusters) in Settings render with an inline restart-required notice adjacent to the field group — not a toast and not a modal-over-modal.
- **AC-L5**: Top-50 row limit is enforced in the UI rendering layer (table never renders more than 50 rows regardless of TauRPC response size) to preserve dashboard performance density contract.
