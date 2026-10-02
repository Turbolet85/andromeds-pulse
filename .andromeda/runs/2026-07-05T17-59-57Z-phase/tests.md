# tests extract

## Relevance
Relevant (partial) — this webview-only chunk touches only the webview unit-test surface (Vitest DOM-shape + pure-function dot-model tests); the Rust harness / integration / E2E P1–P7 tiers are out of scope.

## Constraints
- Standard test tier governs breadth (per test-plan §1) — webview unit coverage is "Comprehensive" for pure logic but presentational canvas rendering stays out of agent scope.
- Webview units run under Vitest 3.x + jsdom 26 + @testing-library/react 16, DOM-shape assertions only, co-located `*.test.tsx`/`*.test.ts` adjacent to source, emitting JUnit XML (per test-plan §4). All four target files already carry co-located tests (`ConstellationCanvas.test.tsx`, `constellation-types.test.ts`, `use-service-constellation.test.ts`; note `constellation-pipeline.ts` has NO co-located test yet).
- Agent-driven discipline: NO canvas pixel inspection / visual diff / "did you see the color" — assert the label + non-color health cue through DOM/accessible-name queries, not the `<canvas>` (per test-plan §1 agent-driven-discipline + §2 agent-runnable invariants). This forces scope-Q2's a11y answer: a canvas-only label is untestable and unreachable; a DOM/accessible-name-reachable cue is both.
- Standard gate set is unconditional AND the webview gates fire because `pulse-app/ui/**` is touched: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -D warnings` + `cargo nextest run --workspace --profile ci` + `cargo xtask capability-drift` + `npm run lint|typecheck|test --prefix pulse-app/ui` (per test-plan §3 Per-chunk gate discipline).
- Boot-smoke gate applies: the plan's file list hits none of the Rust boot paths, but the 2026-07-05 operator directive EXTENDS boot-smoke to webview-only *user-visible* chunks — the constellation qualifies (per test-plan §3 Boot-smoke-gate as extended; see amendment history below).
- Fixtures use factory functions, no raw object literals (per test-plan §7) — reuse the existing local `item(service, state, priorityTier, lastSeenNano)` builder + relative-dated `nowNano() - OFFSET` fixtures.

## Patterns to follow
- Pure dot-model mapping tests in `widget/constellation-types.test.ts`: injected `now`, `it.each` state→value tables (`lifecycleToBrightness`), and the ALREADY-PRESENT severity-hue tests (`derives hueFraction from priority_tier`, `normalizes undefined priority_tier to null (calm baseline)`). The health/severity color encoding partly exists here — extend this mapping + its `it.each` table rather than inventing a new path.
- Non-color-only cue assertion in `dashboard/routes/traces/ConstellationCanvas.test.tsx`: the existing `conveys per-state counts + active findings in the accessible name (not color-alone)` test (asserts `aria-label` substrings + `data-service-count`) is the exact SC 1.4.1 template to extend for per-dot health; `constellationSummary` builds that off-canvas accessible name.
- WebGPU-component test harness: mock `canvas/webgpu-adapter` + `canvas/frame-metrics` via `vi.hoisted`/`vi.mock`, `vi.unstubAllGlobals()` in `afterEach`, assert the `role="region"` implicit-role wrapper + fallback-when-adapter-unavailable path (existing `ConstellationCanvas.test.tsx`).
- Hook test shape in `hooks/use-service-constellation.test.ts`: `vi.hoisted` proxy mock of `services.list_with_states`, `renderHook` + `waitFor`, focus re-poll + unmount-cleanup (only if the resolver hook changes — scope says likely not).
- Two-copies fan-out: `visibleDots`/`constellationSummary` in `widget/constellation-types.ts` feed BOTH `widget/ConstellationCanvas.tsx` and `dashboard/routes/traces/ConstellationCanvas.tsx`; any shared-helper signature change must update both source copies + both co-located tests in one pass.

## Anti-patterns to avoid
- NEVER assert the dot label / health signal via canvas pixels, screenshot, or "visual" checks — canvas text is not agent-reachable (per test-plan §1 agent-driven-discipline); the cue must land in the DOM/accessible name.
- NEVER let the SC 1.4.1 health signal be color-only in the test surface — a green/red hue with no asserted text/token/shape fails the "not color-alone" contract (per test-plan §1 + a11y binding).
- NEVER hand-roll raw `ServiceListItem` object literals in fixtures — use the `item()` factory; `?? null`-normalize `priority_tier` (serde-default optional+nullable field).

## Contract bindings
- tests ↔ a11y: the SC 1.4.1 non-color cue + SC 1.4.11 contrast are a11y-owned but asserted through the tests-owned webview driver; the constellation-semantics axe spec already exists (`pulse-app/ui/tests-a11y/axe/p11`) and the a11y CI gate rides the same `npm run test:a11y` chain (test-plan §Cross-domain a11y-CI-gate binding).
- tests ↔ obs: the (extended) boot-smoke gate verifies the obs log family (`agent-latest.jsonl*`: 0 `app.panic.fatal`, `app.boot.webview.init`, `metric.webgpu.frame_duration_ms`, `viz.query.traces`) per test-plan §3 + obs §3 log-format binding.
- tests ↔ data contract: fixtures bind to `ServiceListItem` (`name` + `state: ServiceLifecycleState` + `priority_tier: PriorityTier | null`) from `services.list_with_states` (chunk #91) — no new TauRPC namespace expected.

## Acceptance criteria contributions
- (tests) `npm run test --prefix pulse-app/ui` passes with updated co-located tests for `ConstellationCanvas.test.tsx` + `constellation-types.test.ts` (add a `constellation-pipeline.test.ts` if new dot-model logic lands in the pipeline file, which is currently untested).
- (tests) Each rendered dot's service name is asserted via a DOM/accessible-name query (`getByText`/`getByRole`/`aria-label`), never via canvas inspection (agent-driven discipline, test-plan §1/§2).
- (tests) The non-color-only health cue (SC 1.4.1) is asserted in the DOM — a text/severity-token/shape present in the accessible name or a DOM attribute — extending the existing "not color-alone" accessible-name test.
- (tests) The health/severity → color+cue mapping has a pure-function `it.each` table test in `constellation-types.test.ts` (extending the existing `hueFraction`/`lifecycleToBrightness` pattern); both ConstellationCanvas copies + tests stay green; full standard gate set + boot-smoke pass.

## Relevant amendment history
- 2026-05-10 "mandate standard chunk gates" (test-plan-amendments.md → §3): every chunk plan's Test Commands must list the full standard gate set; the webview gates (`lint`/`typecheck`/`test --prefix pulse-app/ui`) are mandatory here because the chunk touches `pulse-app/ui/**`. Why: gate-coverage drift otherwise surfaces N chunks later.
- 2026-05-09 "smoke-check boot discipline" (test-plan-amendments.md → §3): established the conditional boot-smoke gate for Rust boot-path chunks. Why relevant: its scope was later EXTENDED — see next.
- 2026-07-05 boot-smoke extension (distilled in `.claude/rules/testing.md` Session Additions, owned by wrap-session): a FULL warm boot smoke runs at implement P3 for ANY user-visible surface change INCLUDING webview-only chunks (re-embed `ui/dist` via `cargo build -p pulse-app`, seed, verify obs log family, clean up by specific PID). Directly governs this chunk (webview-only + user-visible); a mock-IPC/frontend-only smoke is explicitly deemed insufficient.
- Load-bearing distilled test memory for this exact area (`.claude/rules/{testing,frontend}.md` Session Additions): 2026-07-02 two-copies fan-out (widget + dashboard ConstellationCanvas share `visibleDots`/`constellationSummary`); 2026-05-30 serde-default `priority_tier?: PriorityTier | null` → `?? null`-normalize at consumption; 2026-07-02 Date-only `vi.useFakeTimers({ toFake: ["Date"] })` for render-recompute tests; 2026-05-08 `vi.unstubAllGlobals()` + WebGPU-adapter mock discipline for the canvas component test.
