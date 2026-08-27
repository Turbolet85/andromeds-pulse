# layouts extract

## Relevance
Partial — layouts owns only the surface identity of the `report` window that hosts the Copy control and the shape of any user-visible failure/success status; the ACL grant and the headful gate are other domains.

## Constraints
- The Copy control's home surface is the **Report window** — a separate borderless always-on-top Diagnostic Report window rendering the six-section `Report` in the `Modal` **`fill`** variant (edge-to-edge on `--color-base`, no backdrop), positioned left of the findings window and clamped, opened by selecting a findings row (per layout-templates §Surface: desktop-webview → Primary screens → Report window). It is a distinct window label, not an in-page modal inside `main` — this is the layout fact the chunk's window-list widening turns on.
- layout-templates §Primary screens → Report window carries **no ASCII wireframe** for this surface (explicitly deferred as a future targeted touch-up per the P-070/P-082 sketch-lag handoff pattern), so the plan mandates **no component-placement geometry or focus-order position** for the Copy control. Implementation may not read prescribed placement into this chunk.
- Exact control styling and label wording on modal/report surfaces are **owned by the implementation route**, not by layouts (per layout-templates §Component — Investigation modal and §Component — Settings modal, both closing "Exact control styling owned by the implementation route").
- Any user-visible failure surface must follow the plan's honest-error discipline: a **static human message, never the raw `AppError`**, and the failure branch checked **before** the success/optimistic branch so a failure never masquerades as the other outcome (per layout-templates §Component — Empty / error state → Error). Applied here: a swallowed ACL rejection that reads as an ordinary control state is exactly the shape this rule forbids.
- Read-only status text on this product's surfaces carries **no focusable element** ("read-only, affordance-exempt", per layout-templates §Component — Empty / error state → Layout). A diagnostic/status detail added under scope item 4 must therefore not add a tab stop.
- `Esc` closes the modal (per layout-templates §IA notes → Keyboard shortcuts). The `report-window` headful stage's Esc unwind depends on this contract; pressing Copy must leave it intact.
- desktop-webview has **no hard responsive breakpoints** (per layout-templates §IA notes → Responsive behavior), so no per-breakpoint criteria attach to this chunk.

## Patterns to follow
- **Separate-window `fill` modal, not centered-card-over-backdrop.** The report window is deliberately the `fill` variant with no backdrop (§Primary screens → Report window), distinct from the Investigation/Settings modals' centered card over a semi-opaque backdrop (§Component — Investigation modal). Do not reason about it as an in-page overlay.
- **Findings → Report entry path.** The report window is reached by selecting a row in the findings window and is positioned relative to it (§Primary screens → Findings window / Report window). The gate reaches the real Copy control through that documented path, on the report window's own webview.
- **One shared component backs repeated states so the surfaces read as one system** (§Component — Empty / error state: a single shared `EmptyState` backs Metrics/Logs/Snapshots). If a copy-status surface is added, prefer extending the report's existing state rendering over a bespoke one-off.
- **Deliberate tab-stop budget.** Where this product documents keyboard contracts it keeps the stop count explicit and minimal (§Component — Trace data table → Keyboard / focus: roving tabindex, in-row button `tabIndex={-1}`, one stop for the body). New status affordances follow that restraint.

## Anti-patterns to avoid
- Do not author the deferred report-window ASCII wireframe as a side effect of this chunk — it is reserved as a targeted touch-up per the P-070/P-082 sketch-lag handoff (per layout-templates §Primary screens → Report window).
- Do not put raw rejection/`AppError` text on the user-visible surface (per layout-templates §Component — Empty / error state → Error); raw detail belongs in a console/obs record.
- Do not add a new tab stop for a read-only status readout (per layout-templates §Component — Empty / error state → Layout, affordance-exempt).

## Contract bindings
- **Modal / Esc dismissal ↔ a11y §Modal focus trap** (Escape dismiss + restore focus): the report window's `Esc` contract (§IA notes → Keyboard shortcuts) is the same behavior the headful stage's unwind uses; a11y owns the trap/restore half.
- **Status-text placement ↔ a11y SC 2.4.3 focus order / live-region policy**: if scope item 4 lands a visible status detail, its visible reading order next to the Copy control and any announcement policy are a11y's call, not layouts'.
- **Layouts ↔ tests harness**: the `report-window` headful stage must resolve the Copy control on the **report** window surface (per §Primary screens → Report window), not on `main` or `findings`; the surface identity is the layout contract the selector/stage binds to.
- **Status label styling ↔ design tokens**: any copied/failed label's color + type roles are design's (e.g. `color-text-secondary` vs `color-accent`), out of layouts' scope.

## Acceptance criteria contributions
- (layouts) The pressed Copy control is exercised on the **report** window surface — the six-section Report in the `Modal` `fill` variant, edge-to-edge on `--color-base` with no backdrop — reached via the findings-row entry path, not a proxy in `main` (per layout-templates §Surface: desktop-webview → Primary screens → Report window). Whether the shipped report window already matches this documented structure is research's question.
- (layouts) After the Copy press, `Esc` still dismisses the report window and the stage's unwind path is unchanged (per layout-templates §IA notes → Keyboard shortcuts).
- (layouts) If a failure-legibility surface is added, it is a static message that never renders the raw rejection and is distinguishable from the success outcome, with the failure branch resolved before the success branch (per layout-templates §Component — Empty / error state → Error).
- (layouts) The report window's tab-stop count is unchanged by any added read-only status readout — no new focusable element (per layout-templates §Component — Empty / error state → Layout).

## Relevant amendment history
- **2026-07-10-incidents-floating-window-disclosure** — added the `findings` and `report` windows to §Primary screens (report = Diagnostic Report in the new `Modal` `fill` variant, positioned relative to findings), and **deferred both ASCII wireframes** to a future targeted touch-up. Directly this chunk's area: it is the amendment that recorded the report moving to its own window label — the same move the scope names as when the clipboard capability's window list was left un-widened.
- **2026-07-09-traces-table-layout-polish** — established the sketch-lag HANDOFF discipline (illustrative ASCII lag is a future *targeted* touch-up, not absorbed into an unrelated chunk); it is the precedent the 2026-07-10 report-window wireframe deferral cites, and it is why this chunk must not opportunistically draw that wireframe.
- **2026-07-08-self-explaining-empty-states** — added §Component — Empty / error state with the honest-error variant (static message, no hint, checked first, never the raw `AppError`) so a failure cannot masquerade as another state. This is the plan-side rule governing scope item 4's "keep the failure legible" decision.
- **2026-08-23-a11y-verification** — added the trace table's Keyboard / focus block (deliberate single tab stop, in-row button `tabIndex={-1}`, Esc exits to the toolbar). Relevant only as the precedent for keeping added affordances out of the tab order; it touches the Traces surface, not the report window.
