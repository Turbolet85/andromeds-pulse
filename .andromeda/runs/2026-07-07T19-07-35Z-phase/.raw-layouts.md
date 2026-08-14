# layouts extract

## Relevance
Partial — the worded status line has a real layout-placement dimension (which region of the full-dashboard surface hosts a read-only status readout); the folded CARRY (ConnectionDot "just now" tooltip fix) is out of layouts — it is tooltip-copy/formatting, not layout structure, and ConnectionDot is a post-design component with no coverage in the plan.

## Constraints
- **Surface / region / hierarchy:** the worded line lives on desktop-webview → Full dashboard → the read-only status region; the plan's designated read-only status zone on the dashboard is the footer band, which already carries "ingest rate and error % (read-only)" — the worded sentence is a plain-language expansion of that same readout (per layout-templates §Component — Footer (read-only status bar), Full dashboard).
- **Read-only, not a control:** footer status is explicitly read-only (no click handler); the line reports state and adds no interactive affordance (per layout-templates §Component — Footer).
- **No new focusable element / tab-order stable:** dashboard footer interactive elements remain the time-range picker + service filter; a read-only text line introduces no new tab stop (per layout-templates §Component — Primary navigation "Keyboard navigation" + §Component — Footer).
- **Dashboard is the home; widget stays glance-only:** the compact widget is a fixed quarter-screen glance surface that does not reflow — the worded sentence belongs on the dashboard, and any condensed widget variant is not assumed by the plan (per layout-templates §Primary screens "Compact widget" + §IA notes Responsive behavior).
- **No hard breakpoints:** full dashboard is resizable with no CSS media-query breakpoints, so the line must degrade within the existing flex footer rather than introduce breakpoints (per layout-templates §IA notes Responsive behavior).
- **Consistent across views:** Traces/Metrics/Logs share the same header/nav/footer structure, so a dashboard-global status line renders in the same region regardless of active tab (per layout-templates §Primary screens).

## Patterns to follow
- **Footer read-only-status pattern:** full-dashboard footer is a flex row whose right segment holds ingest rate + error % as read-only status with a 1px subtle border-top — the worded line follows this same in-footer status placement (per layout-templates §Component — Footer, Full dashboard).
- **Precedent content already exists:** the compact-widget footer already renders the three data points as stacked read-only lines — "Ingest: 2.4k/s Error: 1.2%" and "Retention: 8m / 10m used" — a direct precedent for the sources/rate/buffer phrasing and its status-band placement (per layout-templates §Wireframe — Compact widget).
- **Shared structure across dashboard views** keeps the status line in one consistent region as tabs change (per layout-templates §Primary screens "same header/nav structure").

## Anti-patterns to avoid
- Do not push the worded sentence into the compact widget by default — it is a fixed, non-reflowing quarter-screen glance surface; a full sentence risks overflow (per layout-templates §IA notes Responsive behavior + §Primary screens).
- Do not turn the read-only status readout into an interactive/focusable control — footer status is read-only (per layout-templates §Component — Footer).
- Do not add media-query breakpoints for the line — the dashboard has no hard breakpoints (per layout-templates §IA notes Responsive behavior).

## Contract bindings
- **layouts ↔ design:** the line's "design-system typography" is owned by the design distiller (tokens), not layouts — layouts owns only which region hosts it (focus guide: tokens are a separate distiller).
- **layouts ↔ a11y:** read-only line stays out of tab order and does not regress footer focus order (a11y §Focus Order SC 2.4.3); visible text is screen-reader-legible by construction per scope.
- **layouts ↔ obs/state:** the footer status region is fed by existing telemetry/state contracts (`connection.current_state`, `services.list_with_states`, `ready` ingest/buffer) — layouts places the readout, obs supplies the three values.

## Acceptance criteria contributions
- (layouts) The worded status line renders in the read-only status region (footer status band) of the Full dashboard surface (layout-templates §Component — Footer, Full dashboard).
- (layouts) The line adds no new focusable element — dashboard footer tab order remains time-range picker → service filter (layout-templates §Component — Footer / §Component — Primary navigation).
- (layouts) The compact widget's fixed quarter-screen glance layout is unchanged — the worded sentence does not appear there by default (layout-templates §Primary screens / §IA notes Responsive behavior).
- (layouts) At resized dashboard widths the line degrades within the flex footer with no new breakpoints (layout-templates §IA notes Responsive behavior).

## Relevant amendment history
- **2026-05-02 Initial generation** — folded the webview "Responsive posture" into §IA notes Responsive behavior (compact widget fixed quarter-screen / no reflow; full dashboard resizable / no hard breakpoints). Relevant because it is the authority governing where the worded line may live (dashboard, not the non-reflowing widget) and forbids breakpoint-based layout. The 2026-06-29 close-to-tray notification amendment is desktop-native and not relevant to this chunk.
