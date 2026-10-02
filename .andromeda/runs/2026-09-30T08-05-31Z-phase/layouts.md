# layouts extract

## Relevance
partial — the chunk changes WHEN a constellation dot first appears, not any surface's layout structure; layout bears only on where that dot lives and what must stay structurally unchanged around it.

## Constraints
- The first dot for a new service renders inside the constellation hero of the full dashboard Traces screen, the fixed (`flex-shrink:0`) region stacked above the `Errors only` toolbar and the internally-scrolling table (per layout-templates §Wireframe — Full dashboard (Traces primary screen), wireframe notes). A discovery fix that adds dots earlier must not change that fixed-hero / flex-filled-table stacking, and must not grow an outer page scrollbar (P-082 bounded route, same §).
- The compact widget also displays the service constellation in its canvas region between the titlebar and the footer. That region is a fixed quarter-screen size with no reflow (per layout-templates §Primary screens → Compact widget; §IA notes → Responsive behavior). An earlier-appearing dot lands in that fixed region, and no layout change is needed or allowed.
- The hero is a landmark `<section>` with the STABLE literal name `"Telemetry traces chart"`, described by the visually-hidden `constellation-summary` element, which carries lifecycle counts and a findings tally only (per layout-templates §Wireframe — Full dashboard (Traces primary screen), wireframe notes, 2026-08-23-a11y-verification). A dot that appears earlier may change the summary's counts sooner, but the landmark name must stay static. `role="status"` stays unapplied, and no service name enters the accessible tree.
- Each hero dot carries an always-on per-dot name and severity-token label (P-069, cited in layout-templates §Wireframe — Full dashboard (Traces primary screen) notes). A newly discovered service's first dot is expected to come with its label on the same render. Whether the label and the dot share one data source today is research's question.
- The live desktop-webview signature is the constellation DOT hue keyed to cumulative incident severity. The Halo State Pulse canvas is deferred and has no production render site (per layout-templates §Component — Halo State Pulse canvas, status note; §Signature placement). The discovery mark and the P-025 hue mark therefore attach to the dot surface, not to any halo layer.
- Both surfaces share the same underlying data feeds (per layout-templates §IA notes → Multi-surface coordination). A discovery-path fix is expected to reach the widget's constellation and the dashboard's constellation alike. Whether both consume the same `services.list_with_states` poll is research's question.

## Patterns to follow
- Live data surfaces update through a 1 s poll: the dashboard `ConnectionStatusLine` "Receiving from {N} services" count and the P-067 live-recency gate it reuses (per layout-templates §Component — Footer (read-only status bar) → Full dashboard). The constellation's 1 s `services.list_with_states` poll follows the same cadence class (scope's inferred timeline). If the fix is backend-side and keeps the poll, the webview needs no new layout element.
- Degrade by data availability with no layout shift: sections render in place and are hidden or populated as data arrives, never flashed while loading (per layout-templates §Component — Empty / error state, "shown only on a settled zero-data load"). Note that §Empty / error state is scoped to Metrics / Logs / Snapshots, not the constellation hero. The hero's zero-services appearance before the first dot is not a spec'd empty-state variant.

## Anti-patterns to avoid
- Attaching the P-027 discovery mark, or any new timing mark, to Halo State Pulse / `HaloCanvas` code paths. They have no production render site, so a mark there would never fire (per layout-templates §Component — Halo State Pulse canvas, status note).
- Introducing a data-driven landmark name, or a live-region role on the hero summary, as part of surfacing new services faster (per layout-templates §Wireframe — Full dashboard (Traces primary screen) notes: static name, `role="status"` deliberately not applied).

## Contract bindings
- layouts ↔ a11y: the hero landmark name, the `aria-describedby` summary and the no-service-name-in-accessible-tree rule bind to the a11y plan's landmark / region coverage (2026-08-23-a11y-verification). An earlier-populating summary changes only the counts, never the structure.
- layouts ↔ obs: the P-027 `metric.constellation.discovery_ms` and P-025 `metric.constellation.hue_update_ms` marks are measured on the constellation DOT surface named in layout-templates §Component — Halo State Pulse canvas (status note). The obs plan owns the observable contract. This plan only fixes which surface the marks may sit on.

## Acceptance criteria contributions
- (layouts) After the fix, a brand-new service's first dot renders inside the constellation hero region of the full dashboard Traces screen. The hero, the `Errors only` toolbar and the table keep their fixed / flex-fill stacking, with no outer page scrollbar (per layout-templates §Wireframe — Full dashboard (Traces primary screen)).
- (layouts) The hero landmark's accessible name remains the literal `"Telemetry traces chart"` before and after the first dot appears. Only the `constellation-summary` counts change (per layout-templates §Wireframe — Full dashboard (Traces primary screen), wireframe notes).
- (layouts) The discovery and hue timing marks sit on the constellation dot render path, with zero marks in Halo / `HaloCanvas` code (per layout-templates §Component — Halo State Pulse canvas, status note).
