# Scope — A11y verification

**Marker:** `2026-08-23-a11y-verification`
**Version:** andromeda-pulse-0.3.0 · **Epoch:** 4 — Polish & ship: verification
**Working entry (verbatim intent):** A11y verification — v0.3.0 interactive surfaces (window,
widget-to-dashboard nav, anomaly controls, constellation, status, empty states):
focus/keyboard/contrast/SR + SC 2.3.3 (per a11y-plan §3/§6/§7).

---

## Outcome

The v0.3.0 interactive surfaces are **a11y-verified against the shipped reality**, and the a11y
specs describe what the product actually does. Two directions of work, not one: add the coverage
that is genuinely missing, and reconcile the plan where it asserts absences that measurement
disproves. A spec that mis-describes the surface is not a smaller defect than a missing test —
it is the defect that makes future coverage bind to phantom selectors.

## Verified premise corrections (measured at HEAD **before** this scope was written)

Promotion folds annotations as **hypotheses**; each named coordinate was re-derived at HEAD. Three
held exactly. One is falsified, and it changes the chunk's shape — recorded here, closed formally
at P3, resolved with the operator at P4/P5.

### ✗ FALSIFIED — CARRY-3 "the Traces surface ships neither a region label nor a `table` role"

The claim is carried identically by **five artifacts**: the working entry, a11y-plan §1 P1 row,
a11y-plan §1 P4 row, a11y-plan §5, a11y-plan §7, and test-plan §1 trigger
`traces-surface-a11y-semantics-absent`. Measured at HEAD, **both halves are wrong, in different
ways**:

1. **The `table` role IS shipped.** `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx:115`
   renders a native `<table>`, with `<thead>` (`:124`), `<tbody>` (`:167`), `<tr>` ×4,
   `<th scope="col">` carrying `aria-sort` (`:137`), `<td>` ×6. Its inline style sets only
   `width` / `borderCollapse: "collapse"` / `fontFamily` / `fontSize` — **no `display:` override**,
   so the implicit ARIA `table` role is intact and exposed. A native `<table>` needs no explicit
   `role="table"`.
2. **A labelled region wrapping the canvas IS shipped** — under a different accessible name.
   `ConstellationCanvas.tsx:229-231` renders
   `<section aria-label={summary} data-testid="constellation-canvas">` around the `<canvas>`
   (`:247`). A `<section>` with an accessible name **is** `role="region"`. What does not exist
   anywhere in the codebase is the *literal* name `"Telemetry traces chart"`; the shipped name is
   a dynamic state summary (`ConstellationCanvas.tsx:67`).
3. **The measurement's locus was wrong.** All five artifacts pin the absence to `TraceTable.tsx`
   ("exposes 13 `data-testid` handles and neither a region label nor a table role"). But per
   a11y-plan §7 the region is *"for WebGPU canvas wrapper"* — that is `ConstellationCanvas.tsx`,
   a **sibling component in the same route** (both mounted by `TracesRoute.tsx:74,76`). Looking
   for a canvas-wrapper region inside the table component and concluding it is absent from the
   surface is a too-narrow search, the same failure mode as the 2026-08-21 session-learning
   (and one this scope's own first probe reproduced, hitting a nonexistent
   `components/TraceTable.tsx` with stderr suppressed).
4. Already **tested**, not merely shipped: `tests-a11y/axe/p11-constellation-semantics.spec.ts`
   asserts *"every canvas must live inside a labelled section wrapper"* and is in the shipped suite.

The **13 `data-testid` count is exact** (verified `grep -c` = 13).

**What genuinely remains from CARRY-3** (the real, much smaller residue):
- **Cell-level keyboard navigation** in the trace list. a11y-plan §5 defers it as *"presumes the
  unshipped `table` role and is therefore not assertable at HEAD"* — that premise is void, so it
  **is** assertable now, and `TraceTable.tsx` implements no arrow-key/cell traversal. This is the
  one piece of CARRY-3 that is unambiguously unbuilt work.
- **The accessible-name question** — whether the canvas region should be renamed to
  the literal `"Telemetry traces chart"`, or the plan amended toward the shipped dynamic summary.
  A state summary carries strictly more information; SC 1.3.1 / 4.1.2 are satisfied either way.
  This is a genuine fork for P4, not a defect to silently fix. **[P3 VERIFIED as a real fork, and
  one discriminator retired:** security flagged that a dynamic summary might embed client-controlled
  telemetry and so count as user-content. Measured — `constellationSummary`
  (`widget/constellation-types.ts:192`) is **aggregate-only**: it counts visible services by
  lifecycle state plus a findings tally, returning e.g. `"Service constellation: no active
  services."`. No service name reaches the accessible name, so both options are equally data-safe
  and the fork turns on information value alone.**]
- **Re-pointing the headful leg's Traces selectors** from `data-testid` to role/accessible-name
  now that the table role is confirmed present.
- **Amending the five artifacts** to what measurement shows.

### ✓ HELD (with one coordinate correction) — CARRY-1, Investigate axe spec

- Substance TRUE: `tests-a11y/axe/p2-investigation-modal.spec.ts` runs only a generic axe sweep on
  the *opened* modal — it never triggers a run, an error, or a busy state, so the
  **result / error / progress** states have no Playwright axe coverage.
- Unit-level a11y is shipped as the CARRY states: `InvestigateButton.tsx:64` `aria-busy`,
  `InvestigationModalForm.tsx:280,381` `role="alert"`.
- **Coordinate correction:** the CARRY names the new spec `p13`. **`p13` is already taken** by
  `p13-empty-states.spec.ts`. Specs run p1–p13 contiguously, so the new spec is **`p14`**.

### ✓ HELD exactly — CARRY-2, findings + report disclosure windows

- `FindingsCounter.tsx:66-67` ships `aria-expanded={isOpen}` + `aria-haspopup="dialog"` with
  **no** `aria-controls` — exactly as annotated, and deliberately (the panel is a separate
  document; the file comments the axe `aria-valid-attr-value` rationale).
- `p8-findings-dropdown.spec.ts` + `p9-diagnostic-report-modal.spec.ts` exist.
- The CARRY itself assigns **the live cross-window focus drive to the "Headful leg extension"
  entry, not this chunk** — honoured; see Out of scope.

### ✓ HELD — PREREQ, `cargo audit` pin #12

Probe points ran at sessions 25/28/31/34/37; 37 discharged in full form, so the next point is
**40**. `state.yaml` `session_count: 39`, so this chunk's wrap **is** session 40 — the probe is
**owed in full form**, not interval-skippable. Auto-satisfy signature recorded in the annotation:
exit 1 + `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`,
with `cargo deny check advisories` overlap green at the eight owned IDs
(0189/0190/0194/0195/0204/0222/0253/0258).

---

## In scope

1. **Reconcile the Traces-semantics claim** across the five artifacts that carry it, to what is
   measured at HEAD. The requirement side stays; the *claim of current absence* is retired.
2. **Cell-level keyboard navigation** for the trace list, and its assertion (the genuinely
   unbuilt half of CARRY-3).
3. **`p14` Investigate states axe spec** — result / error / progress (CARRY-1), alongside the
   existing unit-level assertions.
4. **Verify the v0.3.0 interactive surfaces named by the working entry** — window, widget-to-dashboard
   nav, anomaly controls, constellation, status, empty states — for focus / keyboard / contrast /
   SR per a11y-plan §3 / §6 / §7, including **SC 2.3.3** (`prefers-reduced-motion`), which the
   universal CLAUDE.md invariant makes non-negotiable for every transition incl. the data-driven
   Halo State Pulse.
5. **Confirm CARRY-2's static coverage** (disclosure trigger + report dialog role/name) is real in
   the axe/unit suite; record what the live drive still owes its owning entry.
6. **`cargo audit` probe in full form** (PREREQ, point 40) — basis re-verified, overlap enumerated,
   result recorded in the chunk report.

## Out of scope / boundaries

- **The live cross-window focus drive** (findings→widget, report→findings) — explicitly the
  **Headful leg extension** entry's, per CARRY-2's own text. This chunk may bind selectors it will
  use, but does not build that leg.
- **P-075 / P-077** — the two unclaimed matrix caps are other entries' work.
- **No new TauRPC procedure, no capability-JSON edit, no Rust dependency change** — nothing in the
  intent implies a backend surface; keeping `Cargo.toml`/`Cargo.lock` untouched also preserves the
  PREREQ's *a fortiori* basis. **[P3 VERIFIED:** every in-scope edit lands under `pulse-app/ui/**`
  (React components + `tests-a11y/`). None of the five boot-smoke trigger paths — `pulse-app/src/main.rs`,
  `crates/ui-bridge/src/`, `pulse-app/src/observability.rs`, `pulse-app/tauri.conf.json`,
  `pulse-app/capabilities/*.json` — is touched, so the boot-smoke gate stays untriggered and the
  `a fortiori` basis holds.**]
- **Retro-fitting a11y to surfaces outside v0.3.0's interactive set** — the entry enumerates them.

## Surfaces & contracts touched

- **Webview components:** `dashboard/routes/traces/TraceTable.tsx` ·
  `dashboard/routes/traces/ConstellationCanvas.tsx` · `dashboard/routes/TracesRoute.tsx` ·
  `dashboard/InvestigationModalForm.tsx` · `components/InvestigateButton.tsx` ·
  `widget/FindingsCounter.tsx` **[P3 RESOLVED — the measured edit set is much narrower:**
  `TraceTable.tsx` (cell-level keyboard navigation, the one genuinely unbuilt item) and, depending on
  the P4 fork, `ConstellationCanvas.tsx` (accessible-name only). `InvestigationModalForm.tsx`,
  `InvestigateButton.tsx` and `FindingsCounter.tsx` need **no** source edit — their a11y is already
  shipped and correct; they are spec *subjects*, not edit targets.**]
- **A11y harness:** `pulse-app/ui/tests-a11y/axe/` (new `p14`), `keyboard-focus/`,
  `reduced-motion.spec.ts`, `playwright-a11y.config.ts`; suite entry `npm run test:a11y`.
- **Specs to amend (documentation, not code):** `.andromeda/a11y-plan.md` §1 P1 + §1 P4 + §5 + §7 ·
  `.andromeda/test-plan.md` §1 trigger `traces-surface-a11y-semantics-absent`. Amendments are
  **wrap's** job via the drift/amendment path — this chunk surfaces and evidences them, and does
  not edit spec sources itself (phase and implement are read-only on specs).
- **Verification matrix:** no a11y capability exists in
  `andromeda-pulse-0.3.0/verification-matrix.json` (22 entries; the only unclaimed are P-075 and
  P-077, both other entries'). **This chunk is expected to claim zero capabilities** — permitted:
  a chunk addressing no cap links nothing. Confirmed at P5.

## Known harness constraint

`npm run lint:a11y` passes rules as inline `--rule` args, which have previously failed to survive
Windows npm quoting (recorded in a prior evolve diagnosis). Expect to invoke the underlying eslint
form directly if it reproduces.

## Open questions carried to P4

1. **Canvas region name** — rename to the literal `"Telemetry traces chart"`, or amend the plan to
   the shipped dynamic summary? (Contestable lean; the summary carries more information.)
2. **Cell-level keyboard navigation depth** — full arrow-key grid traversal vs. row-level
   Tab/Enter. a11y-plan §5 describes "Tab across cells, arrow-key drill into rows/columns"; how
   much of that a 4-column read-only table warrants is a real design call.
3. **Breadth of item 4** — how much of the six-surface sweep is already covered by the shipped
   p1–p13 + keyboard-focus + reduced-motion specs vs. genuinely uncovered. **[P3 MEASURED — mostly
   covered.** `reduced-motion.spec.ts` already sweeps SIX surfaces (compact-widget, traces, metrics,
   logs, snapshots, settings) asserting animations degrade to 0ms; `p13-empty-states.spec.ts` covers
   the metrics + logs empty states; p1–p13 cover the routes and modals; `keyboard-focus/
   widget-and-modals.spec.ts` covers widget + modal focus. Genuinely uncovered: **(a)** the
   Investigate result/error/progress states (→ `p14`), and **(b)** trace-list cell-level keyboard
   navigation, which is unbuilt in the product. So the chunk is SMALL, and item 4 is a
   verify-and-record sweep rather than a build.**]

---

## P3 premise closure — further corrections measured at HEAD

Beyond the four `[inferred]` bullets closed inline above, research retired three more premises this
chunk inherited. All three shrink the work; none expands it.

- **The Halo State Pulse has no subject on the webview.** Item 4 says "incl. the data-driven Halo
  State Pulse", and four distillers independently flagged the render site as research's question.
  Measured at HEAD by code-graph refs + grep: `HaloCanvas` is referenced only by its own props
  interface (`halo/HaloCanvas.tsx:70-74`), one test (`HaloCanvas.test.tsx:193`), and four
  *comments*. **Zero production render sites** — the 2026-08-21 measurement still holds. The
  severity→hue semantic ships on the constellation dot instead, and **both** constellation surfaces
  already gate motion (`dashboard/routes/traces/ConstellationCanvas.tsx:63`,
  `widget/ConstellationCanvas.tsx:67`). The Halo's build-or-retire decision belongs to the "Halo
  State Pulse canvas disposition" entry, not this one.
- **`test:a11y` IS wired into CI — test-plan §9 is the stale side.** The tests extract reported that
  test-plan §9's CI stage table lists no a11y suite, while the a11y extract reported the chain runs
  in `ci.yml`; aggregate check C flagged the pair. Measured: `.github/workflows/ci.yml:124-125` runs
  `cargo xtask test:a11y` inside the `lint-test-build` job, and `xtask/src/main.rs:174` delegates
  straight to `npm run test:a11y --prefix pulse-app/ui`. So a11y-plan is substantively right and
  test-plan §9's table is missing a row — an expected amendment, not work.
- **The design deferral on error-text color is already discharged.** design-system's 2026-05-03
  amendment deferred migrating body-size error text off the accent token "to the error-UI chunk or a
  follow-up a11y audit", and flagged this chunk as the nearest such audit. Measured: both Investigate
  error sites (`InvestigationModalForm.tsx:280`, `:381`) already render
  `color: var(--color-text-primary)` with the accent carried as a *border*
  (`rgba(199, 85, 106, 0.5)`) — exactly the mandated non-text usage. Nothing owed; `p14` records it.

**One arch question resolved rather than corrected:** the a11y suite reaches populated state via
`installTauriIpcMock` (`tests-a11y/helpers/mock-tauri.ts:14`), a webview-level IPC mock. That is NOT
the class arch's test-time-telemetry-injection mandate governs — that mandate binds e2e tests against
a *running* app, whereas the a11y suite runs Playwright against the built Vite dist with no Tauri
backend at all. Pre-existing, shipped, CI-wired; no conflict.
