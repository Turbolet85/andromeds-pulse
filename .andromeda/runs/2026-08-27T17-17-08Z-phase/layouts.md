# layouts extract

## Relevance
Partial — this chunk creates and modifies no surface, but it changes the incident population that count-driven layout regions (findings window sizing, constellation dot hue, constellation-summary findings tally) consume, so layout's stake is guardrails, not new structure.

## Constraints
- No new user-facing surface or primary screen may be introduced. layout-templates §Surface: desktop-webview → Primary screens is the closed inventory (compact widget · full dashboard Traces/Metrics/Logs/Snapshots · Settings panel · findings window · report window); scope §Boundaries puts the damper upstream of the wire, so its decisions belong at an observable leaf, not a new region.
- The findings window is required to size itself to the incident count (fits the rows, caps ~8 then the list scrolls), per layout-templates §Primary screens → Findings window. Outcome 2 ("incident population converges") is an INPUT to that rule, which must continue to hold for a smaller/stable population including the degenerate near-zero case. Whether the current sizing code already handles a shrinking population without a stale height is research's question.
- The live desktop-webview signature is the constellation DOT hue driven by cumulative incident severity (tiers none/curious/suggested/autonomous), per layout-templates §Component — Halo State Pulse canvas (measured note 2026-08-21). Damping generations must not alter that hue-driver contract or its tier vocabulary.
- The Traces hero landmark must keep the STABLE literal accessible name `"Telemetry traces chart"`, with lifecycle counts + findings tally carried only by the visually-hidden `data-testid="constellation-summary"` description, per layout-templates §Wireframe notes — Full dashboard (Traces primary screen). A converging tally may move the description; it may not move the name.
- The Halo State Pulse canvas is SPECIFIED-BUT-UNBUILT and its build-or-retire is owned by the "Halo State Pulse canvas disposition" working-route entry, per layout-templates §Component — Halo State Pulse canvas — matching scope §Boundaries, which excludes deciding it here.
- If damping yields a settled zero-data reading on any data view, it must render the empty (no-data) branch, never the error variant — the error variant is checked BEFORE empty precisely so a failure never masquerades as "no data", per layout-templates §Component — Empty / error state. This is the layout-side analogue of outcome 3's "quiet because damped ≠ quiet because dead".
- The compact widget stays aggregate-glance — no worded status line (that readout is dashboard-only), per layout-templates §Component — Footer (read-only status bar). Any future damper indication does not land in the widget footer.

## Patterns to follow
- Read-only live status band: layout-templates §Component — Footer specifies live-polled, read-only, non-focusable status display. If damper state ever surfaces visually, this shape (dashboard footer line) is the precedent — not a new panel.
- Shared-row reuse: the findings window reuses the P-080 FindingsDropdown row content on the `--color-raised-2` popover surface, per layout-templates §Primary screens → Findings window. An incident-population change flows through the existing shared row component; no new row layout.
- Settled-state gating: the empty state is shown only on a settled zero-data load, never flashed while loading, hidden once populated, per layout-templates §Component — Empty / error state. Same discipline applies to any "damped/quiet" reading — it is a settled condition, not a transient one.
- Single-tab-stop table contract: layout-templates §Component — Trace data table (Keyboard / focus) requires the table body be a roving-tabindex row list contributing exactly ONE tab stop, with the in-row Investigate at `tabIndex={-1}`. Backend-only work must leave this untouched.
- Bounded-route/internal-scroll composition: layout-templates §Wireframe notes — Full dashboard (Traces primary screen) requires fixed hero + toolbar with the table flex-filling and scrolling internally (no outer page scrollbar). Any incident-count-driven content change must not reintroduce an outer scrollbar.

## Anti-patterns to avoid
- Do not add a surface, window, drawer, or panel to signal damping — layout-templates §Surface: desktop-webview → Primary screens is the surface inventory, and scope §Boundaries places the damper's visibility at an exact observable leaf.
- Do not let a live-changing count drive a landmark's accessible NAME — layout-templates §Wireframe notes — Full dashboard (Traces) makes the name deliberately static because a name that tracks the data churns the screen-reader rotor.
- Do not add focusable elements or new tab stops as a side effect — layout-templates §Component — Trace data table (Keyboard / focus) fixes the table body at one tab stop, and §Component — Empty / error state is explicitly affordance-exempt (no focusable element).

## Contract bindings
- layouts ↔ a11y: the constellation-summary description wired via `aria-describedby`, with `role="status"` deliberately NOT applied (layout-templates §Wireframe notes — Traces). Layout owns placement of the tally; a11y owns announcement behavior — if a converging findings tally changes announcement frequency, that is a11y's call, not layout's.
- layouts ↔ incidents/triage data: the findings-window height rule (layout-templates §Primary screens → Findings window) consumes the incident count this chunk's outcome 2 changes; the constellation dot hue (§Component — Halo State Pulse canvas) consumes cumulative incident severity.
- layouts ↔ obs: scope outcome 3 requires the damper's decision be visible at the wire via an exact allowlist leaf. Layout's binding is negative — that observable is a wire leaf, not a rendered region, so no layout region is contributed for it.

## Acceptance criteria contributions
- (layouts) No new user-facing surface or primary screen is added by this chunk; the desktop-webview Primary screens inventory is unchanged (per layout-templates §Surface: desktop-webview → Primary screens).
- (layouts) The findings window still sizes to the incident count (fits rows, caps ~8 then scrolls) under a converged/smaller incident population, with no stale or collapsed height (per layout-templates §Primary screens → Findings window).
- (layouts) The Traces hero landmark retains the stable literal accessible name `"Telemetry traces chart"`; any findings-tally change appears only in the visually-hidden `constellation-summary` description (per layout-templates §Wireframe notes — Full dashboard (Traces primary screen)).
- (layouts) No focusable element or tab stop is added — the trace table body still contributes exactly one tab stop and the in-row Investigate stays `tabIndex={-1}` (per layout-templates §Component — Trace data table, Keyboard / focus).

## Relevant amendment history
- **2026-07-10-incidents-floating-window-disclosure** — added the findings + report windows to §Primary screens, with the findings window sizing to the incident count and reusing FindingsDropdown rows; ASCII wireframes deferred. Relevant because this chunk changes the incident population that sizing rule reads.
- **2026-08-21-delegated-timing-observables** — recorded the webview Halo canvas as unbuilt, made the constellation DOT hue the shipped signature, and corrected the hue driver from error rate to cumulative incident severity; build-or-retire assigned to the "Halo State Pulse canvas disposition" route entry. Relevant because the damper's incident population feeds that hue driver, and scope §Boundaries excludes deciding the disposition here.
- **2026-08-23-a11y-verification** — made the hero a landmark with a deliberately STABLE name and moved the live lifecycle counts + findings tally into the `aria-describedby` description (operator's P4 fork: a data-tracking name churns the rotor). Relevant because this chunk converges that tally; the same fork's reasoning bars re-attaching it to the name.
- **2026-07-08-self-explaining-empty-states** — added the shared empty/error region with the honest-error variant checked BEFORE the empty branch, so a failure never reads as "no data". Relevant as the documented precedent for outcome 3's "damped vs dead" distinguishability requirement.
