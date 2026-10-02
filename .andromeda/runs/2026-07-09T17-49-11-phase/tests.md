# tests extract

## Relevance
Relevant — this is a webview-only React unit-test + a11y-regression surface (D1/D3/D4 land `*.test.tsx` DOM-shape assertions; the a11y axe sweeps are the tests-run vehicle), squarely in-domain; the backend/harness half (obs status/logs, cargo suite) is dormant.

## Constraints
- Webview unit framework is fixed: `vitest` 3.x + `jsdom` 26 + `@testing-library/react` 16, co-located `*.test.tsx` adjacent to source, discovered via `include: ["src/**/*.{test,spec}.{ts,tsx}"]`, `setupFiles` registering `afterEach(cleanup)`, emitting JUnit XML to `target/junit-ui.xml` (per test-plan §4 webview framework + §1 desktop-webview-unit-tests row).
- DOM-shape assertions ONLY — no visual diff, no pixel/geometry inspection; jsdom is layout-blind (`getBoundingClientRect`=0), so D1's bounded `max-height`/`overflow-y:auto` and D3's non-overlap are asserted as attribute/style/DOM-structure presence, with the real geometry left to the /implement P3 visual verify (per test-plan §1 desktop-webview-unit-tests row + §2 agent-runnable invariants).
- Per-chunk gate discipline: the webview gates `npm run lint` / `npm run typecheck` / `npm run test` (all `--prefix pulse-app/ui`) are MANDATORY here because Files-to-modify touch `pulse-app/ui/**` (per test-plan §3 "Per-chunk gate discipline"; the chunk's own Gate-note defers only the cargo half while zero `.rs`).
- Boot-smoke gate is NOT triggered — this chunk touches none of `pulse-app/src/main.rs` / `crates/ui-bridge/src/` / `tauri.conf.json` / `capabilities/*.json`, so the conditional Tauri-dev runtime smoke is out of scope (per test-plan §3 "Boot-smoke gate (conditional)").
- Zero-flakiness + determinism: no retry-once, no `sleep(N)` for sync, no real timers — the D4 announce assertions must key on an explicit signal (RTL `findBy`/`waitFor` on the live-region text), not a delay (per test-plan §10 zero-flakiness budget + §11 E2E "NEVER use sleep(N)").
- Coverage numeric gate (≥75% line / 70% branch / 85% function) is workspace-cumulative excluding generated code; the Foundation-stage presentational carve-out was scoped to `components/icons/`, so these Epoch-3 route components are exercised by passing DOM-shape/a11y tests rather than a per-file % target (per test-plan §4 coverage target + §10).

## Patterns to follow
- React DOM-shape / ARIA-contract assertion pattern (the icon-test model): assert `aria-*`, accessible-name, role, and structural containers rather than internals — for D3 this means asserting each constellation dot keeps its always-on accessible name + non-color severity token (the P-069 SC 1.4.1 / 4.1.2 contract) survives collision-avoidance (per test-plan §4 "webview React components" bullet).
- Co-located `*.test.tsx` next to source (e.g. `TraceTable.test.tsx`, `ConstellationCanvas.test.tsx`, plus a `constellation-types.test.ts` for pure label-layout logic) — extend existing files in place (per test-plan §4 conventions + §1 example `Icon.test.tsx`).
- `@testing-library/react` interaction pattern for D4: drive the sort / Errors-only controls via RTL events and assert the `StatusLiveRegion` announces exactly once per action (polite), asserting observable live-region content — not setState call ordering (per test-plan §2 agent-runnable invariants).
- a11y axe regression-sweep pattern: run the p1-traces + p11-constellation-semantics specs inside the same `npm run test` suite and hold them at 0-new-violation (per test-plan §1 desktop-webview-unit-tests row; scope §Surfaces "the relevant a11y axe spec(s)").

## Anti-patterns to avoid
- NEVER add visual-regression/human-approval checks (Percy / Chromatic / Applitools) or assert canvas pixels for D1/D3 — layout correctness is DOM-shape + the separate visual verify, not a screenshot diff (per test-plan §11 E2E + Universal).
- NEVER test implementation details — for D4 assert the emitted announcement (live-region text), not the internal `setState`-updater refactor mechanics (per test-plan §11 Unit).
- NEVER commit `#[ignore]`/`.skip` tests without a tracked reason, and never mask flake with retries (per test-plan §11 Quality / CI).

## Contract bindings
- a11y ↔ tests harness: the p1-traces / p11-constellation-semantics axe specs execute inside the webview `npm run test` gate (the 5-command `run` surface), and D3 must preserve the a11y-owned P-069 accessible-name + non-color-severity contract (SC 1.4.1 / 4.1.2) at 0-new-violation — tests supplies the wiring, a11y owns the assertions.
- obs ↔ tests harness: (none) — zero `.rs`/log-format/status-endpoint delta, so the §3 status/logs bindings are not engaged this chunk (re-engages with the full cargo gate + `bindings.ts` regen only if `.rs` is unexpectedly touched, per scope Gate-note).

## Acceptance criteria contributions
- (tests) `npm run test --prefix pulse-app/ui` passes with the new/changed DOM-shape assertions: `TraceTable.test.tsx` (scroll wrapper carries bounded `max-height` + `overflow-y:auto`; hero + toolbar rendered outside the scroll region; announce fires from the handler not the updater) and `ConstellationCanvas.test.tsx` / `constellation-types.test.ts` (label collision-avoidance) — per test-plan §3 + §4.
- (tests) `npm run lint` + `npm run typecheck --prefix pulse-app/ui` pass (unconditional webview gates, `pulse-app/ui/**` touched) — per test-plan §3.
- (tests) a11y axe sweeps p1-traces + p11-constellation-semantics report 0 new violations, with D3 asserting each dot retains an always-on accessible name + non-color severity token — per test-plan §1 + a11y binding.
- (tests) D4: sort-direction and Errors-only-toggle announcements fire exactly once per user action (politely) with no "Cannot update a component while rendering" warning, verified via an explicit live-region signal (no timer) — per test-plan §2 + §11.

## Relevant amendment history
- 2026-05-10 "chunk plans MUST include the standard gate baseline regardless of scope" — DIRECTLY governs this chunk's Gate-note: the webview `lint`/`typecheck`/`test` baseline is mandatory even for a frontend-only chunk; empirically chunk #36 omitting `fmt`/`tsc` let pre-existing failures (incl. `pulse-app/ui/.../MetricsChart.test.tsx`) surface N chunks later — do not silently drop gates.
- 2026-05-09 "per-chunk Tauri-dev runtime smoke gate for boot-path chunks" — cited as NOT-triggered: this chunk touches no boot/setup path, so boot-smoke is legitimately absent (the latent `ui-bridge/health.rs` reactor panic it guards against cannot be introduced by a zero-`.rs` webview chunk).
- 2026-05-10 "wrap-session Phase 5 reconcile MUST rerun tooling unconditionally" — marginal/skill-level: this is exactly the webview-dominant chunk profile (chunks #33–#36) whose skipped tooling reruns accreted living-artifact staleness; flagged so the wrap does not skip tooling on this webview-only delta.
