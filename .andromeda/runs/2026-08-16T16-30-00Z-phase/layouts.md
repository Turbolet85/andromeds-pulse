# layouts extract

## Relevance
Partial — this chunk is a `crates/triage` gate change with no surface of its own; layouts binds only at two edges: where newly-reachable cue families surface as incidents, and IF the warm-up bound is surfaced as a user setting.

## Constraints
- Newly-reachable baseline-derived cue families must surface through the existing incident disclosure path — the Findings window (borderless, always-on-top, docked below the compact widget, opened from the widget unread badge) and the Report window opened from a findings row — not a new window or panel (per layout-templates §Surface: desktop-webview → Primary screens). Whether silence / activity-floor incidents already route into that list is research's question.
- The Findings window is required to size itself to the incident count, fitting rows and capping ~8 before the list scrolls (per layout-templates §Primary screens — Findings window). If warm-up reachability changes incident volume, that cap+scroll is the mandated containment; the window may not grow unbounded.
- Findings rows must reuse the P-080 FindingsDropdown row content on the opaque `--color-raised-2` popover surface (per layout-templates §Primary screens — Findings window) — no per-family bespoke row anatomy for the silence / activity-floor families.
- The Report window is required to be the `Modal` `fill` variant, positioned relative to (left of, clamped) the findings window (per layout-templates §Primary screens — Report window) — a family-specific report layout is not permitted.
- If the bound is config-surfaced as a user-visible setting, its only sanctioned home is the Settings modal's vertically stacked form-control section (`gap: space-lg`, labeled input + helper text in `font-label`) (per layout-templates §Component — Settings modal (form)). The plan's enumerated Settings controls (theme / snap position / retention / MCP toggle, per §Primary screens — Settings panel) do not include a warm-up control, so adding one is a layout-templates surface amendment at wrap, not implicit.
- Layout templates are frozen mid-implementation ("Do not edit during implementation runs", per layout-templates top-of-file scope note); any surface/control added here goes through the amendment sidecar at the wrap.

## Patterns to follow
- Shared-component-across-views pattern: one shared component backs all three data views "so the data views read as one system" (per layout-templates §Component — Empty / error state) — precedent for one presentation path across families rather than per-family surfaces.
- Bounded-numeric-setting pattern: the retention-seconds control (numeric input with a declared range, label + helper text) is the existing analogue for surfacing a bounded duration (per layout-templates §Component — Settings modal (form) → Form controls) if the warm-up bound becomes user-visible.
- Geometry-derived floating window pattern: the findings window is positioned from the compact widget's live geometry with work-area / multi-monitor clamping (per layout-templates §Primary screens — Findings window) — reuse this, do not invent placement, if any disclosure surface is touched.
- Read-only status-line pattern: dashboard-footer `ConnectionStatusLine` — plain-language words in `font-body`, numerics in the `font-code` Data role, 1s poll (per layout-templates §Component — Footer → Full dashboard) — the sanctioned shape IF operator-visible warm-up state is ever needed, and it is dashboard-only.

## Anti-patterns to avoid
- Do not add a warm-up / "baseline learning" worded readout to the compact-widget footer — the widget footer is ingest / error / retention only and the widget "stays aggregate-glance (no worded line)" (per layout-templates §Component — Footer → Compact widget and → Full dashboard).
- Do not represent a warming/not-yet-ready baseline as the data-view empty state: the empty state is mandated for settled zero-data loads only and must never be flashed while loading, and the error variant is checked before the empty branch (per layout-templates §Component — Empty / error state).
- Do not introduce a new always-on-top window, panel, or tab for the newly-reachable families — the Primary screens registry is the closed set of surfaces (per layout-templates §Surface: desktop-webview → Primary screens).

## Contract bindings
- layouts ↔ ui-bridge/config: if the bound reaches `crates/ui-bridge/src/contract.rs::Settings`, the Settings-modal control list (layout-templates §Component — Settings modal) binds to that contract's shape — a settings field with no control, or a control with no contract field, is a mismatch.
- layouts ↔ a11y: findings-row list and the Report window bind to a11y §Focus Order and §Modal focus trap (Escape dismiss + restore focus) if row count / scroll behavior changes.
- layouts ↔ design: `--color-raised-2` popover surface, `font-code` Data-role numerics, and `Modal` `fill` variant tokens are design-owned; layouts fixes placement only.
- If the change stays entirely inside `crates/triage` (no incident-presentation or Settings surfacing), there is no layout binding at all for this chunk.

## Acceptance criteria contributions
- (layouts) No new user-facing surface is introduced: incidents from the newly-reachable families render in the existing Findings window rows / Report window, with no addition to the Primary screens registry (per layout-templates §Surface: desktop-webview → Primary screens).
- (layouts) The Findings window still sizes to the incident count and caps ~8 rows before the list scrolls under any warm-up-driven incident volume (per layout-templates §Primary screens — Findings window).
- (layouts) If the warm-up bound is surfaced as a user setting, it renders as a labeled bounded input with helper text inside the Settings modal's stacked form section, and layout-templates §Component — Settings modal is amended at wrap (per layout-templates §Component — Settings modal (form)).
- (layouts) The compact-widget footer is unchanged — ingest / error / retention only, no warm-up or baseline-state wording (per layout-templates §Component — Footer → Compact widget).

## Relevant amendment history
- **2026-07-10-incidents-floating-window-disclosure** — added the Findings window and Report window to §Primary screens (findings sizes to incident count, reuses P-080 row content, replaces the interim in-widget popover; report = `Modal` `fill` variant positioned relative to findings). Why: the chunk introduced two genuinely-new windows absent from Primary screens (D-layout-surface); ASCII wireframes deferred per the P-070/P-082 sketch-lag pattern. Directly relevant: this is the surface any newly-reachable cue family's incidents must land in.
- **2026-07-08-self-explaining-empty-states** — added §Component — Empty / error state (shared `EmptyState`, exporter hint, honest-error variant checked before empty). Why: P-071 shipped states that layout-templates had not registered. Relevant as the precedent that transient/not-yet-ready conditions are not the empty state.
- **2026-07-07-plain-language-connection-status** — documented the dashboard-only `ConnectionStatusLine` and explicitly kept the compact widget aggregate-glance with no worded line. Why: P-070 (intent F10) landed the worded readout; the Footer component had documented only ingest + error. Relevant if any warm-up/readiness readout is proposed — dashboard footer only, never the widget.
