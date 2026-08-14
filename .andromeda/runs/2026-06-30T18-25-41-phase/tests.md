# tests extract

## Relevance
Partial — this chunk is webview-only (no Rust backend or IPC changes), but touches `pulse-app/ui/**` and requires standard gate discipline + unit tests.

## Constraints
1. Per §4 Unit Test Strategy — webview unit tests use vitest 3.x + jsdom 26 + @testing-library/react 16; emit JUnit XML to `target/junit-ui.xml`
2. Per §4 — co-locate `*.test.tsx` adjacent to component source under `pulse-app/ui/src/`; Vitest auto-discovers via `include: ["src/**/*.{test,spec}.{ts,tsx}"]`
3. Per §4 — presentational components EXCLUDED from coverage gate at Foundation pre-shell stage (no coverage % threshold for this chunk)
4. Per §3 Test Harness Contract / Per-chunk gate discipline (2026-05-10 amendment) — standard gate set MANDATORY when `pulse-app/ui/**` touched: `npm run lint`, `npm run typecheck`, `npm run test` (all `--prefix pulse-app/ui`); Rust gates (`cargo fmt/clippy/nextest/capability-drift`) are safe no-ops for webview-only chunks
5. Per §4 — afterEach(cleanup) via test-setup.ts; no manual verification or screenshot comparison
6. Per scope acceptance — verification method is "webview" (DOM tests dispatching real `contextmenu` event in PROD mode, asserting suppression); dev mode must preserve right-click/Inspect (production gate: `import.meta.env.PROD`)

## Patterns to follow
1. Per §4 — DOM-shape assertions only; fixture-based test data if context mock needed
2. Per scope acceptance — test contextmenu handler with `fireEvent.contextMenu()` (or `userEvent`) on root and canvas elements, assert `preventDefault` called when `import.meta.env.PROD === true`
3. Per scope — canvas element tests assert `draggable={false}` prop or equivalent CSS (`user-select: none`, `-webkit-user-drag: none`) in place

## Anti-patterns to avoid
1. Per §2 agent-driven invariants — no headful Playwright; no manual "did right-click work?" verification
2. Per §4 — no visual regression / pixel-snapshot assertions on canvas rendering
3. Per scope — suppression must NOT break legitimate copy/paste in editable fields (Settings form inputs); test that contextmenu exemption may apply to input elements, OR test that keyboard selection/copy remains intact despite contextmenu suppression

## Contract bindings
Webview unit test gate binds to §3 Test Harness Contract / standard gate set discipline (npm run test / lint / typecheck); no IPC procedures, no obs logging bindings expected.

## Acceptance criteria contributions
1. "(tests) `npm run test --prefix pulse-app/ui` passes; contextmenu suppression tests dispatch real `contextmenu` event, assert `preventDefault` in PROD build, assert dev mode allows context menu."
2. "(tests) Canvas element carry no drag/save affordance (no `draggable=true`; CSS `user-select` / `-webkit-user-drag` suppression in place)."
3. "(tests) `npm run lint --prefix pulse-app/ui` and `npm run typecheck --prefix pulse-app/ui` pass (no new type errors or linting violations)."
4. "(tests) Legitimate text interaction (copy/paste in input fields) not broken by contextmenu suppression (verify via keyboard-select + copy test, or input exemption if used)."

## Relevant amendment history
2026-05-10 amendment (chunk-gate-baseline-coverage) — standard gate baseline (npm run lint/typecheck/test for webview chunks) is unconditional when `pulse-app/ui/**` touched; applies to this chunk. Boot-smoke gate is NOT conditional here (scope explicitly excludes boot-path files per intent). No Rust gates expected to have changes, but Rust gates must still be run per the standard baseline (safe no-ops for webview-only work).
