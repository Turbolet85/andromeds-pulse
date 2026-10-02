# layouts extract

## Relevance
Partial — the chunk builds no surface, but every DOM-side assertion (Traces render, empty states, footer line, Investigate result) reads regions this plan defines, so layouts supplies the target-state selectors and region semantics the driver must bind to.

## Constraints
- The Traces route is required to be a flex column bounded to the shell's `main` height with the hero + `Errors only` toolbar fixed above a flex-filling table that scrolls internally under a sticky opaque thead, and **no outer page scrollbar** (per layout-templates.md §Wireframe — Full dashboard (Traces primary screen) + §Component — Trace data table). Whether the code already satisfies this at HEAD is research's question — it decides whether CARRY #8's single-default-size half absorbs cheaply.
- The `Errors only` control is required to be a right-aligned `aria-pressed` toggle in its own toolbar row above the table, with the default (unsorted) order anomaly-first (per layout-templates.md §Component — Trace data table, Sorting + Filter toolbar). This is the filter/sort state CARRY #5 requires to survive an auto-refresh.
- The data-view empty state is required to be one shared region across Metrics/Logs/Snapshots — Observatory glyph + worded message + `:4318`/`:4317` exporter hint — shown **only on a settled zero-data load**, never flashed while loading (per layout-templates.md §Component — Empty / error state). The error variant is required to be checked BEFORE the empty branch and to carry NO hint.
- The worded connection-status readout is required to be dashboard-only: connected-source count + spans/s + buffer fill on a 1s poll, with the compact widget staying aggregate-glance and carrying no worded line (per layout-templates.md §Component — Footer (read-only status bar) → Full dashboard).
- The desktop-webview signature is recorded as the constellation **dot** hue driven by cumulative incident severity; the Halo State Pulse WebGPU canvas is SPECIFIED-BUT-UNBUILT on both webview surfaces (per layout-templates.md §Component — Halo State Pulse canvas, MEASURED 2026-08-21). Any "live dots" count in the CARRY #6 cross-check must target dots, not a halo layer.
- The full dashboard is required to be resizable with **no hard breakpoints** and the compact widget fixed quarter-screen with no media queries (per layout-templates.md §IA notes → Responsive behavior). There is therefore no per-breakpoint behavior for CARRY #8's multi-size clause to assert — only size-invariance of the bounded layout.
- Views are required to be routable per nav entry (`/traces`, `/metrics`, `/logs`, `/snapshots`, `/settings`), with the tabs-vs-sidebar nav model still uncommitted (per layout-templates.md §IA notes → Deep-link convention + §Component — Primary navigation). Reaching Metrics/Logs for CARRY #7 is route navigation; the driver must not hard-bind to one nav model.

## Patterns to follow
- Investigate is triggered from the telescope "Investigate" button **or** a trace-row click, opening a centered modal over a backdrop with title "Investigate {trace-id}" and a top-right `✕` (per layout-templates.md §Component — Investigation modal) — that modal is the result surface for acceptance assertion 3.
- One shared `EmptyState` component backs all three data views (per layout-templates.md §Component — Empty / error state, "Applies to"), so a single selector family covers both the Metrics and Logs legs of CARRY #7.
- Toggles carry accessible state on the element (`aria-pressed`), matching the predecessor leg's `button[aria-label="Close to tray"]` selector discipline (per layout-templates.md §Component — Trace data table, Filter toolbar).
- Hero dots carry always-on per-dot name + severity-token labels (P-069), giving countable labelled DOM elements for the footer-count vs live-dots cross-check (per layout-templates.md §Wireframe notes — Full dashboard Traces).
- Findings docks directly below the compact widget sized to the incident count (caps ~8, then scrolls); Report is the `Modal` `fill` variant positioned left of findings (per layout-templates.md §Primary screens — Findings window / Report window). Boundary for this chunk (CARRY #9), but this is the geometry any future window-mechanics leg asserts.

## Anti-patterns to avoid
- Do not bind any assertion to a Halo State Pulse canvas render site on desktop-webview — the plan records it as unbuilt, so such a check would fail for the wrong reason (per layout-templates.md §Component — Halo State Pulse canvas).
- Do not assert "a message rendered" for the empty state without discriminating the error variant — the plan bans a failure masquerading as no-data, so the hint's presence/absence is the discriminator (per layout-templates.md §Component — Empty / error state).
- Do not assert Traces scrolling via window/page scroll, and do not expect the worded status line on the compact widget (per layout-templates.md §Component — Trace data table, Internal scroll region; §Component — Footer).

## Contract bindings
- **Focus survival across auto-refresh** (CARRY #5) binds layouts' focus-order position to a11y §Focus Order SC 2.4.3 — layouts names the position, a11y owns the criterion.
- **Modal / floating-window dismiss** (Investigation modal; findings/report Esc-blur dismiss with cross-window focus restore, CARRY #9) binds to a11y §Modal focus trap (Escape dismiss + restore focus).
- **Bounded-height arithmetic** (`calc(100vh − titlebar − tabnav − footer var(--spacing-lg))`) binds to design §Spacing tokens — layouts owns the region structure, design owns the token values.
- **DOM selectors ↔ tests harness:** the regions above are the selector anchors the WebDriver leg reads; the obs-record half of each binding (allowlist leaves, record families) is obs/tests, not layouts.

## Acceptance criteria contributions
- (layouts) Traces rows render in the primary content region inside the table's own bounded scroll region with a sticky thead, and the route shows no outer page scrollbar (per layout-templates.md §Wireframe — Full dashboard (Traces primary screen) / §Component — Trace data table).
- (layouts) Under a traces-only stream, `/metrics` and `/logs` render the shared empty state — glyph + worded message + `:4318`/`:4317` hint — and not the hint-less error variant (per layout-templates.md §Component — Empty / error state).
- (layouts) The full-dashboard footer renders the worded connection-status line (connected count + spans/s + buffer fill) while the compact widget renders none (per layout-templates.md §Component — Footer (read-only status bar) → Full dashboard).
- (layouts) The Investigate result renders in the centered Investigation modal over its backdrop, titled "Investigate {trace-id}" (per layout-templates.md §Component — Investigation modal).

## Relevant amendment history
- **2026-07-09-traces-table-layout-polish** — registered the fixed-hero + `Errors only` toolbar + internal-scroll/sticky-thead table as current truth (P-082, plus the P-068 toolbar catch-up). Directly the layout CARRY #8 and CARRY #5 assert against. Its explicit HANDOFF: the Traces ASCII sketch's footer row and per-dot labels lag truth (superseded by P-070 and P-069) — read the §Component text, not the sketch, when deriving selectors.
- **2026-07-07-plain-language-connection-status** — added the dashboard-only `ConnectionStatusLine` footer readout with the compact-widget exclusion and the P-067 recency gate for the connected count. This is CARRY #6's source of truth, including the "count matches constellation live-dots" premise.
- **2026-07-08-self-explaining-empty-states** — created the Empty/error §Component (shared `EmptyState`, exporter hint, error-checked-first). This is CARRY #7's source; the amendment's own split (settled zero-data vs query failure) is exactly the absorbed/boundary split the scope draws.
- **2026-08-21-delegated-timing-observables** — recorded the webview Halo canvas as unbuilt across twelve sites and corrected the hue driver to cumulative incident severity; build-or-retire is owned by a separate route entry. Explains why constellation assertions must target dots and why a canvas-presence check is off the table.
- **2026-07-10-incidents-floating-window-disclosure** — added the findings + report windows to Primary screens with ASCII wireframes deferred. Context for boundary CARRY #9 (the geometry exists in prose only, which is part of why it needs a dedicated window-mechanics leg).
