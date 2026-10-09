# layouts extract

## Relevance
Partial — the chunk changes the *behavior* (live re-poll) of a layout-plan-covered component (Trace data table) but builds no new layout structure, region, focusable element, modal, nav, or breakpoint; a few placement/interaction constraints govern it.

## Constraints
- The Trace data table is the **primary content block** of the Full dashboard Traces view — positioned below the constellation-map hero and above the footer status band; a refresh must keep rows rendering in that region and must not relocate/restructure it (per layout-templates §Wireframe — Full dashboard (Traces primary screen) + §Component — Trace data table).
- Sorting is a first-class table interaction (headers cycle asc → desc → unsorted, active state shown via bold text + caret); a re-poll must **preserve the active sort state**, not reset it (per §Component — Trace data table "Sorting").
- The default **anomaly-first ordering** (erroring rows hoisted to top, surfaced via the Error status-glyph column) must survive a refresh — the re-poll cannot reorder the read (per §Component — Trace data table "Columns" + "Sorting").
- Row rendering is **instant — "no transition (instant per 0.35 expression)"** — so the refresh must swap rows without transitions/flicker that would exceed the table's low-motion posture (per §Component — Trace data table hover state + §Expression level 0.35).
- The Full dashboard has **no hard responsive breakpoints** and the table does not reflow by width; refresh introduces no per-breakpoint behavior (per §IA notes "Responsive behavior").
- The empty-state layout ("No traces yet") is **not specified in the plan** — its design is owned by the separate P-071 chunk; here only the empty→populated *transition* is in play, so no plan layout governs the empty-state visual (plan silence + focus guide §Empty states, deferred to P-071).

## Patterns to follow
- The **constellation hero** on the same Traces screen already updates live (~1s); the table (content region directly below it) should update on the same in-app cadence so the whole Traces surface reads as one coherent live surface (per §IA notes "Multi-surface coordination" / shared data feeds).
- Table structure is the shadcn `Table` (thead + tbody + rows) — refresh should update tbody rows while the sortable header row stays stable (per §Component — Trace data table "Layout" grid).
- Sorting affordance (clickable headers, caret in `color-primary`, bold text on active) is the established interaction — carry its active/visual state across the re-poll (per §Component — Trace data table "Sorting").

## Anti-patterns to avoid
- Do **not** add row-appear/replace transitions or reflow on refresh — the row model is explicitly instant; animated row churn violates the 0.35 low-motion table posture and defeats the anomaly-first read (per §Component — Trace data table + §Expression level 0.35).
- Do **not** add refresh chrome (spinner, reload button, "stale" banner) to the table region or footer — the footer is a read-only status band and the plan specifies no manual-reload affordance; live refresh is silent, like the constellation (per §Component — Footer (read-only status bar) + §Wireframe notes).

## Contract bindings
- **Layouts ↔ a11y (focus order):** the "Errors only" filter toggle and sortable headers are focusable controls whose focus position + state must survive the re-poll — layouts places them; a11y owns `aria-pressed` / focus-order integrity (focus guide binding "Focus order ↔ a11y §Focus Order SC 2.4.3"; scope: `aria-pressed` survives a refresh).
- **Layouts ↔ obs/data feed:** the content region binds to the `viz.query.traces` feed; the route/obs specialist owns the poll lifecycle, layouts only governs where rows render (per amendment 2026-05-02 handoff "DuckDB query layer … route specialist owns component integration").

## Acceptance criteria contributions
- (layouts) Once telemetry lands, the Trace table renders populated rows in the **primary content block** of the Full dashboard Traces view — below the constellation hero, above the footer — with no manual reload (layout-templates §Wireframe — Full dashboard Traces + §Component — Trace data table).
- (layouts) A re-poll **preserves active sort and the anomaly-first default ordering** (erroring rows stay hoisted to top) — no sort reset, no reorder beyond newly-arrived data (layout-templates §Component — Trace data table "Sorting").
- (layouts) Refresh updates rows **without transition/flicker** — the instant, low-motion row model is preserved (layout-templates §Component — Trace data table hover "no transition per 0.35 expression").

## Relevant amendment history
(none) — neither recorded amendment touches this area: the 2026-05-02 initial generation established the Trace-table / Full-dashboard-Traces layout that is still current, unamended, truth; the 2026-06-29 close-to-tray entry is a desktop-native notification trigger, unrelated to the webview Traces table.
