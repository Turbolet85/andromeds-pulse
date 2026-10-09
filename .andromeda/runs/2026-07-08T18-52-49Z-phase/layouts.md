# layouts extract

## Relevance
partial — component-placement, surface, responsive, and focus-order rules apply; the plan has no dedicated empty-state section, so the nearest layout precedent is the Halo-canvas fallback message.

## Constraints
- The empty state lives in the full-dashboard **primary content block** — the region the data table / time-series chart occupies, below the hero and above the footer (per plan §Wireframe — Full dashboard (Traces primary screen)).
- Metrics and Logs views reuse the **Traces header/nav structure**, so the empty branch touches only the content region — header, hero, nav, and footer stay unchanged (per plan §Primary screens).
- Logs' empty state substitutes for the **shadcn Table tbody rows** (thead + tbody + rows) and Metrics' for the chart body — the branch replaces content within the panel, not the panel frame (per plan §Component — Trace data table).
- **No CSS media-query breakpoints**: the full dashboard reflows fluidly; the empty state must lay out fluidly and stay readable at the min ~13-inch full-dashboard width (per plan §IA notes → Responsive behavior).
- The **compact widget is aggregate-glance only** (Halo badge + read-only footer, no per-route data lists) — no empty-state panel there (per plan §Wireframe — Compact widget; §IA notes → Responsive behavior).

## Patterns to follow
- **In-panel muted-message precedent**: the Halo canvas already renders a centered `font-body` message in `color-text-tertiary` for its unavailable/absent state — the empty state mirrors this same in-panel muted-explanatory placement (per plan §Component — Halo State Pulse canvas → Fallback).
- **Icon-vocabulary reuse**: the chrome already uses aperture / telescope ("Investigate") / constellation-grid glyphs — reuse an existing Observatory glyph rather than a novel one (per plan §Component — Custom titlebar; §Component — Trace data table), matching scope OQ#2 "prefer reuse".
- **"Reads as one system" reuse**: the plan's cross-surface coherence (shared shader/feed for visual consistency) applied at the component level — one shared `EmptyState` across both Metrics and Logs (per plan §IA notes → Multi-surface coordination).

## Anti-patterns to avoid
- Do NOT add an empty-state panel to the compact widget — it stays aggregate-glance (per plan §Wireframe — Compact widget).
- Do NOT introduce media-query breakpoints for the empty state — the dashboard reflows fluidly with no hard breakpoints (per plan §IA notes → Responsive behavior).
- Do NOT restructure header / hero / nav / footer — confine the change to the primary content region (per plan §Primary screens).

## Contract bindings
- **layouts ↔ design**: message/glyph styling (muted `font-body`, `color-text-tertiary`/`-muted`, 24px hero glyph, centering/spacing) is design/tokens — layouts owns the region + placement only; the fluid centering binds to design §Spacing per-surface spacing tokens.
- **layouts ↔ a11y**: focus order — the empty state adds no focusable element (read-only, affordance-EXEMPT per scope Acceptance) → no new focus stop; the semantic role (`role="status"` vs plain text, scope OQ#3) is the a11y distiller's call, not layout's.

## Acceptance criteria contributions
- (layouts) EmptyState renders in the **primary content region** of the full-dashboard Metrics & Logs views — below the hero, above the footer — not in header/hero/nav/footer (plan §Wireframe — Full dashboard / §Primary screens).
- (layouts) EmptyState is **absent from the compact widget** — no per-route empty panel on the glance surface (plan §Wireframe — Compact widget).
- (layouts) **Responsive**: EmptyState lays out fluidly with no CSS media-query breakpoint and stays readable at the min ~13-inch full-dashboard width (plan §IA notes → Responsive behavior).
- (layouts) **Focus order unchanged** — EmptyState adds no focusable element; the Metrics/Logs tab-through sequence matches the pre-change order (plan §Component — Trace data table region; scope Acceptance affordance-EXEMPT).

## Relevant amendment history
- **2026-07-07-plain-language-connection-status** (nearby, same Epoch 3 "state honesty & legibility"): added the full-dashboard footer `ConnectionStatusLine` plain-language readout ("Receiving from N services"). Why relevant — same surface + same legibility epoch; it is the precedent for self-explaining, plain-language state readouts in the full dashboard that this chunk extends from the footer into the Metrics/Logs **content** region (note: different region — footer vs primary content block).
- **2026-05-02 initial generation** (foundational, not a nearby change): established the full-dashboard primary content block + shadcn Table structure the empty state renders within.
- (Excluded: **2026-06-29-predictable-close-self-verify** — desktop-native tray notification trigger, outside this chunk's webview content-region area.)