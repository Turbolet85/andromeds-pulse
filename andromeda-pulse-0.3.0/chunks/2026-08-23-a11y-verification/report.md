# Report — 2026-08-23-a11y-verification

**Chunk:** A11y verification — the v0.3.0 interactive surfaces are verified for focus/keyboard/contrast/SR + SC 2.3.3, and the Traces-semantics claim five artifacts carry is reconciled to what measurement shows
**Date:** 2026-08-23
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:** 7 modified + 1 new, **all under `pulse-app/ui/`** —
  `src/dashboard/routes/traces/TraceTable.tsx` · `src/dashboard/routes/traces/TraceTable.test.tsx` ·
  `src/dashboard/routes/traces/ConstellationCanvas.tsx` ·
  `src/dashboard/routes/traces/ConstellationCanvas.test.tsx` · `src/styles/tokens.css` ·
  `tests-a11y/helpers/mock-tauri.ts` · `tests-a11y/keyboard-focus/widget-and-modals.spec.ts` ·
  **NEW** `tests-a11y/axe/p14-investigate-states.spec.ts`.

- **Symbols / APIs:** **No new public fn, IPC procedure, endpoint, export, port, socket, or env var.**
  Module-private additions only: `focusRow` / `handleRowKeyDown` / `activeRowIndex` state / `rowRefs` /
  `errorsOnlyRef` (TraceTable.tsx); `SUMMARY_ID` + `VISUALLY_HIDDEN` consts (ConstellationCanvas.tsx).
  **Changed signature:** `TraceRowView` gains `index` / `isActive` / `rowRef` / `onKeyDown` — a
  module-private component whose **sole caller is TraceTable itself** (verified: zero references outside
  the defining file). **Changed behaviour, callers preserved:** `installTauriIpcMock` now honours two
  optional response sentinels (`__mockReject`, `__mockDelayMs`); plain values still resolve exactly as
  before, so **all existing a11y specs keep working unchanged** (16 spec files re-verified green).
  New DOM handles: `data-testid="constellation-summary"`, CSS class `.trace-row`.
  **Removed from the tab order (deliberate):** the in-row `trace-row-investigate` button is now
  `tabIndex={-1}`; it keeps its accessible name and click behaviour, and Enter on the focused row
  activates it. Verified no consumer bound it by tab order (its only occurrence is its own definition;
  the headful leg reaches Investigate via the **titlebar** control).

- **Crates / modules:** none added, removed, or changed. **Zero compiled-source delta** — no `.rs`,
  no `Cargo.toml`, no `Cargo.lock`, no `pulse-app/capabilities/*.json`, no `tauri.conf.json`
  (mechanically confirmed against `git status --porcelain`).

- **Dependencies:** **none added, none bumped.** `package.json` untouched; no npm devDependency added.

- **Schema / config:** none. No migration, no config key, no change to the a11y violation JSON schema
  or the obs log schema.

- **Spec-master edits:** **none** — implement is read-only on the seven masters; every correction below
  is routed to this wrap's amendment flow.

- **Counts / qualifiers moved:**
  - **Axe spec set p1–p13 → p1–p14** (new `p14-investigate-states.spec.ts`). Documented as **"p1–p12"**
    in `.claude/rules/a11y.md` §Harness shape and as the **P1–P12** surface set in `a11y-plan.md` §1
    "Surface set extension" — both already stale by one (p13 landed 2026-07-08) and now stale by two.
  - **Playwright a11y suite 34 → 37 tests / 15 → 16 files** (`npx playwright test --list`).
  - **Webview vitest 801 → 809.**
  - **Owned upgradeable advisory IDs: security-plan states 7; measured 8** — see Spec claims disproved.

- **Dev-tool versions:** none installed or upgraded.

- **Reverted / negative API facts:**
  - A **per-row live-region announce** was prescribed by plan step 1 and deliberately **not** implemented
    (it would double-speak against the row's own native focus announcement).
  - **`role="status"` on the summary element** appeared in the operator-approved preview sketch and was
    deliberately **omitted** (1s-polling surface + an existing dashboard live region + the documented
    multi-`role="status"` query-ambiguity trap). Flagged to the operator as reversible on request.
  - A **mutation** neutralising `focusRow` was applied to prove the new pins discriminate, then **fully
    reverted** (verified: zero `sorted.length >= 0` matches remain).
  - The chunk's webview source was **stashed and restored twice** for flake attribution; final tree
    verified to carry all chunk changes.

- **Spec claims disproved by measurement** (nothing authored — these await disposition):
  1. `a11y-plan.md` §1 **P1 row** — "`region[aria-label="Telemetry traces chart"]` and `table` (trace list)
     are NOT SHIPPED — measured absent at HEAD" → **FALSE on both halves.** `TraceTable.tsx:115` renders a
     native `<table>` (`thead`/`tbody`/`th scope="col"`/`aria-sort`; inline style sets only
     `borderCollapse`, no `display:` override), so the implicit ARIA `table` role is intact; and
     `ConstellationCanvas.tsx:229` already wrapped the canvas in a **named `<section>`**, which *is*
     `role="region"`. The claim also **mis-located** the region: §7 assigns it to the canvas wrapper — a
     **sibling** component — while all sites looked for it inside `TraceTable.tsx`.
  2. `a11y-plan.md` §1 **P4 row** — "`table` … NOT SHIPPED at HEAD, in step with P1" → **FALSE**, same evidence.
  3. `a11y-plan.md` **§5** (full-dashboard-traces) — cell navigation "presumes the unshipped `table` role and
     is therefore not assertable at HEAD" → **premise void**; the role ships. Row-level traversal now ships
     and IS asserted.
  4. `a11y-plan.md` **§7** Landmark roles — region "REQUIRED but NOT SHIPPED" → a labelled region shipped
     (under a dynamic name); this chunk lands the literal name plus an `aria-describedby` summary.
  5. `test-plan.md` §1 trigger **`traces-surface-a11y-semantics-absent`** — "NEITHER exists at HEAD" → **FALSE**;
     this chunk is its named owner.
  6. `test-plan.md` **§9** CI stage table lists **no** a11y suite → **FALSE at HEAD**: `.github/workflows/ci.yml:125`
     runs `cargo xtask test:a11y` inside the `lint-test-build` job (`xtask/src/main.rs:174` delegates to
     `npm run test:a11y --prefix pulse-app/ui`).
  7. `design-system.md` §Component Patterns → Error state — the 2026-05-03 deferral (migrate body-size error
     text off the accent token) → **already discharged**: `InvestigationModalForm.tsx:280` and `:381` render
     `color: var(--color-text-primary)` with the accent carried as a border only.
  8. `security-plan.md` §Dependency Security → CI integration — "**7** owned upgradeable IDs" → measured
     **8** at HEAD (0189/0190/0194/0195/0204/0222/0253/0258), matching the route annotation.
  9. `.claude/rules/a11y.md` (Tier-2 cascade) — §WebGPU canvas a11y and §Critical paths **P1** both restate
     "REQUIRED but NOT SHIPPED" → now false on the same evidence; §Harness shape says spec files are "p1–p12".

- **Coverage of new surfaces:**
  - `TraceTable row keyboard traversal (roving tabindex, Up/Down/Home/End/Enter/Esc)` → validation n/a ·
    instrumentation n/a (no new telemetry emitted) · PII n/a · tests **unit ✓ (8 vitest) + webview ✓
    (Playwright, real keypresses)** · a11y **WCAG/focus/kbd ✓** (SC 2.1.1 / 2.4.3 / 2.4.7; no `tabindex > 0`) ·
    tokens **design-token ✓** (`--border-focus` via `:focus-visible`; no hardcoded hex or px literal)
  - `ConstellationCanvas stable landmark name + aria-describedby summary` → validation n/a ·
    instrumentation n/a · PII **n/a — measured**: `constellationSummary` is aggregate-only (counts by
    lifecycle state + findings tally; **no service name reaches the accessible tree**) · tests **unit ✓** ·
    a11y **✓** (SC 1.3.1 / 4.1.2; stable landmark name) · tokens n/a (visually-hidden, no visual token)
  - `p14 Investigate result/error/progress axe spec` → tests **a11y/e2e ✓** (3 tests, populated fixtures) ·
    a11y ✓ · PII **n/a** (fixtures carry no credential-shaped values; the error fixture is the sanitized
    `AppError` boundary form and the spec asserts no path/stack/type marker reaches the alert)
  - `mock-tauri settle (reject / delay)` → test-harness helper; tests **✓** (exercised by p14) ·
    backward-compatible for all existing callers

## Deviations from intent

1. **Plan step 1 contradicted itself** — it prescribed roving `tabindex` on `<tr>` while banning nested
   focusables (a11y-plan §11), but every row already contains an Investigate button. **Resolved** by making
   the in-row button `tabIndex={-1}` so the row is the single tab stop and Enter activates it — the standard
   grid-lite pattern satisfying both constraints. Surfaced, not silently interpreted.
2. **The prescribed per-row live-region announce was not implemented** — a focused row announces natively;
   announcing per arrow key would double-speak (a11y-plan §4 stacking ban).
3. **`role="status"` omitted** from the summary element vs the approved preview sketch — justification above;
   reversible on request.
4. **Two files edited outside research's boundary list** (gray-area judged in-scope, both reported):
   `tokens.css` (a `:focus-visible` ring is inexpressible inline, and that file already hosts `.traces-scroll`
   for this same component) and `mock-tauri.ts` (the shared mock only resolved, so error/progress states were
   unauditable). `v02-fixtures.ts` was listed but proved unnecessary — fixtures inlined in the spec.
5. **Spec number is `p14`, not the `p13` the route entry's CARRY named** — `p13-empty-states.spec.ts`
   already occupies that slot.
6. **Two workspace Rust gates deferred** under the source-delta-proportional rule (zero compiled-source
   delta, mechanically confirmed): `cargo clippy --workspace --all-targets --all-features` and
   `cargo nextest run --workspace --profile ci`. This chunk is the deferral origin.

## Decisions & corrections

- **Operator decision (P4 fork 1):** canvas region takes the **stable literal name** `"Telemetry traces chart"`
  with the live aggregate summary preserved as an `aria-describedby` target — chosen over renaming-and-dropping
  and over amending the plan to the dynamic name. Rationale accepted: a landmark whose name mutates with data
  churns the screen-reader rotor.
- **Operator decision (P4 fork 2):** **row-level** traversal (Up/Down/Home/End/Enter/Esc), not full 2-D cell
  traversal — a 4-column read-only table with one action per row does not warrant a grid; a11y-plan §5 is
  amended to match rather than the code stretched to it.
- **Operator correction (durable-text discipline):** a CARRY draft stated flatly "pure startup race, not
  load-sensitive timing" while the conversational report carried the caveat that n=5 cannot exclude a load
  effect on *frequency*. **Route text outlives conversations — the caveat belongs in the durable copy.**
  Corrected clause now reads: removing contention did NOT stabilize it (2/5 vs 1/5–1/4 — one run at n=5;
  *a residual load effect on frequency is not excluded, its elimination is*): operative diagnosis a startup
  race, with the bimodal toggle-recovery structure as the load-independent half. The same overclaim was
  retracted in the friction log with a scoping note.
- **Operator directive (measurement):** re-sample the flaky leg on a quiesced host and record the pass rate
  as a measured datapoint regardless of outcome; the leg stays out of this chunk's scope either way.
- **Verification discipline applied:** every new pin was **mutation-checked** both tiers, and the Playwright
  arm was only read after rebuilding `ui/dist` — a stale bundle would have passed the mutation silently.

## Outcome

**Acceptance criteria met.** Gates run and green:
`npm run lint` · `npm run typecheck` · `npm run test` (**809/809**, was 801) ·
`npx playwright test --config=playwright-a11y.config.ts --list` (suite-health: 37 tests / 16 files) ·
`cargo xtask test:a11y` (**37/37** Playwright · Lighthouse 7 surfaces all ≥90 · pa11y 7/7 URLs **0 errors** ·
aggregator 186 tuples **all `severity: info` from Lighthouse**, zero axe violations · regression-detector
**0 new vs baseline**) · `cargo xtask self-verify` **PASS** · `cargo fmt --check` ·
`cargo xtask capability-drift` **clean** · `cargo xtask capability-widening-check` **clean (0/3)** ·
`cargo deny check bans licenses sources` **ok**.

**Supply-chain arms (designed-red / PREREQ):** `cargo deny check advisories` exit 1 at the **8** owned IDs.
**`cargo audit` PREREQ (pin #12, point 40) DISCHARGED IN FULL FORM** — true exit **1**, basis reproduced
byte-identical (`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`);
the RustSec DB still cannot load, so **the standing deferral does NOT end**; next point 43.

**Deferred (noted, not skipped):** `clippy --workspace --all-features` + `nextest --workspace` — zero
compiled-source delta.

**Boot smoke ✓ (UI surface changed).** Warm re-embed verified by **matching the embedded bundle name to
`ui/dist`** (`index-Bd3mgCt6.js`) rather than assuming; fresh data dir; **2970 rows ingested over real OTLP**;
`viz.query.traces` returning 100 rows; **0 ERROR, 0 `app.panic.fatal`**; 12 tick families emitting; zero
orphans; `:4317`/`:4318` closed after cleanup. The 4 WARNs are L4 inference degrading gracefully with no
model configured (documented FSM), unrelated to this chunk.

**Headful leg — PASSED with these changes, and separately found FLAKY (pre-existing).**
`cargo xtask webview-drive` drove **all 7 stages** green on runs `drive6` / `q1` / `q4`, which is the
regression evidence the Traces DOM changes needed. Independently, the leg fails intermittently at its
`launch` stage (the `main` window never leaves `about:blank`). Measured across **14 runs / 3 conditions**:
loaded host with changes **1/5**; loaded host **without** changes (source stashed, frontend rebuilt, binary
re-embedded) **1/4**; quiesced host with changes **2/5**. Removing GPU/CPU contention did NOT stabilize it
(one run at n=5; *a residual load effect on frequency is not excluded, its elimination is*) — operative
diagnosis a **startup race**, PRE-EXISTING and not introduced here. **Structural, load-independent half:**
the outcome is perfectly bimodal across all 14 runs (`observed=true` ⟺ `toggle_pressed=false`;
`observed=false` ⟺ `toggle_pressed=true`) with **toggle-press recovery 0/10** — so a longer wait cannot fix
it (50s already elapses in failing runs); the 30s fallback is ineffective rather than short. The leg is
untouched by this chunk and needs an owner (test-plan §10 sets a zero-flakiness budget).

**Verification matrix:** this chunk claims **zero capabilities** — re-confirmed by reading all 22 acceptances;
the 10 a11y-adjacent caps are already `verified` under other chunks, and neither unclaimed cap (P-075
Conductor/MCP timing, P-077 demo injector) is satisfied here. Coverage gate is a no-op.
**Regression guard noted:** P-081's verified acceptance requires the Traces table to preserve focus across its
re-poll — the path this chunk's roving tabindex lands on; its assertions re-ran green (809/809 includes the
P-081 focus-preservation test).
