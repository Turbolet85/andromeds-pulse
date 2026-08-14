# tests extract

## Relevance
Relevant — webview unit-test (vitest DOM-shape) slice only; Rust harness / OTLP / security / perf / tauri-driver E2E surfaces are out-of-scope for this frontend-only render-branch chunk.

## Constraints
- Webview unit tests use `vitest` 3.x + `jsdom` 26 + `@testing-library/react` 16, **DOM-shape assertions only — no visual diff** (per test-plan §4 "Framework (webview unit tests)"; §1 Surfaces "desktop-webview unit tests"). The `EmptyState` + route empty branches are proven by presence/absence of message + hint + glyph nodes, never pixels.
- Co-locate `*.test.tsx` adjacent to source under `pulse-app/ui/src/`; Vitest discovers via `include: ["src/**/*.{test,spec}.{ts,tsx}"]` (per §4 Conventions). New `EmptyState.test.tsx` sits beside the component; empty-branch cases extend the existing route tests.
- Because Files-to-modify touch `pulse-app/ui/**`, the chunk's `## Test Commands` MUST carry the **webview gates** `npm run lint` / `npm run typecheck` / `npm run test` (all `--prefix pulse-app/ui`) plus the always-on `cargo fmt --check` / `cargo clippy … -D warnings` / `cargo nextest run --workspace --profile ci` / `cargo xtask capability-drift` (per §3 Per-chunk gate discipline).
- **Boot-smoke gate is NOT required** — it fires only when a chunk touches `pulse-app/src/main.rs`, `crates/ui-bridge/src/`, `tauri.conf.json`, or `pulse-app/capabilities/*.json`; this chunk touches none (frontend-only, no capability delta per scope Boundaries) (per §3 Boot-smoke gate).
- Agent-driven determinism: the three-state assertion (empty present only on settled `rows.length===0 && !isLoading`; absent while loading; absent when populated) must be driven via controlled hook/mock state with **no real timers**; `npm run test` exit 0 gates and emits JUnit XML to `target/junit-ui.xml` (per §2 Agent-runnable invariants; §4; §1 Surfaces).
- The Rust ≥75/70/85% coverage gate does **not** bind a new webview presentational component (that gate is `cargo-llvm-cov` over Rust src); gating here is `npm run test` green, not a coverage percentage (per §4 Coverage target; §10).

## Patterns to follow
- Icon/presentational DOM-shape contract (chunk #11, §4 "webview React components"): assert `svg viewBox` / `aria-*` / `currentColor` propagation (no hardcoded hex) / size-prop pass-through (16/20/24 grid) / decorative-vs-meaningful ARIA flip (default `aria-hidden="true"`; `aria-label` → `role="img"`) / banned-element grep (no `<animate>`/`<animateTransform>`/gradient/filter). The Observatory glyph in `EmptyState` is asserted this way. Model file present: `pulse-app/ui/src/components/icons/Icon.test.tsx`.
- Extend, don't replace, the existing co-located route tests: `pulse-app/ui/src/dashboard/routes/MetricsRoute.test.tsx` and `LogsRoute.test.tsx` already exist — add empty-branch cases there; mirror sibling `dashboard/routes/metrics/MetricsChart.test.tsx` shape (named in amendment 2026-05-10).
- Shared setup is inherited, add none: `pulse-app/ui/vitest.config.mjs` with `setupFiles: ["./src/test-setup.ts"]` registering `afterEach(cleanup)` (per §4; both files confirmed present).
- Reuse-first glyph selection: existing `Telescope.tsx` / `Aperture.tsx` / `ConstellationGrid.tsx` all present under `src/components/icons/` — assert whichever chosen glyph renders via the icon DOM-shape contract rather than introducing a new one.

## Anti-patterns to avoid
- NEVER add visual regression / pixel inspection / "human reviews the glyph" — no Percy/Chromatic/Applitools; render correctness is a DOM contract, not a screenshot (per §11 E2E + Universal).
- NEVER use `sleep(N)` / real `setTimeout` to synchronize the loading→settled transition for the "not flashed while loading" assertion — wait on explicit state (per §11 E2E "NEVER use sleep(N)"; §2).
- NEVER assert implementation details / fixture-internal ids — assert the public DOM contract (message text, hint naming `:4318`/`:4317`, glyph node), not internal component state (per §11 Unit).

## Contract bindings
- **tests ↔ a11y (live):** scope acceptance requires "the a11y sweep stays green" and possibly a new co-located axe spec if a new persistent surface warrants it; those a11y assertions ride the same webview vitest/CI wiring tests owns (per focus-guide "A11y CI gate binds to a11y §Bootstrap"; scope Surfaces). Tests provides the harness; a11y owns the role decision (plain text vs `role="status"`, scope open-Q3).
- **tests ↔ obs harness (inactive):** the 5-command / status-endpoint / log-format bindings are untouched — no `ready`/`health` field, no `pulse://` topic, no daemon-health delta (per scope Boundaries).

## Acceptance criteria contributions
- (tests) `npm run test --prefix pulse-app/ui` passes new DOM-shape assertions for **both** MetricsRoute and LogsRoute: on zero-data + not-loading, the explanatory message + actionable hint (naming `:4318`/`:4317`) + Observatory glyph node render; all three ABSENT when populated and ABSENT (not flashed) while loading (§4; scope Acceptance).
- (tests) Full webview gate set green — `npm run lint` + `npm run typecheck` + `npm run test` (`--prefix pulse-app/ui`) alongside the unconditional Rust gates (§3 Per-chunk gate discipline).
- (tests) The chosen glyph's usage satisfies the icon DOM-shape contract (viewBox / aria flip / `currentColor` / no banned animate/gradient/filter elements) via its co-located test (§4 "webview React components").
- (tests) a11y sweep stays green (plus, if warranted, a co-located axe spec passes) — the added empty-state nodes introduce no new violations (scope Acceptance; a11y CI-gate binding).

## Relevant amendment history
- **2026-05-10 — "chunk plans MUST include the standard gate baseline regardless of scope"** — directly on-point: mandates the unconditional gate set and the webview gates (`npm run lint`/`typecheck`/`test --prefix pulse-app/ui`) this chunk's Test Commands must carry; empirically driven by chunk #36/#37 catching pre-existing errors in `dashboard/routes/metrics/MetricsChart.test.tsx` — the exact route family this chunk edits.
- **2026-05-09 — "per-chunk Tauri-dev runtime smoke gate for boot-path chunks"** — relevant as a scoping (negative) signal: confirms boot-smoke is conditional on boot/setup paths; this frontend-only chunk touches none, so its omission is correct, not a gap.
- (Security/PII, self-OTLP, and four-load-profile amendments (2026-05-08 ×4, 2026-06-10) are outside this chunk's webview-unit-test area — omitted.)