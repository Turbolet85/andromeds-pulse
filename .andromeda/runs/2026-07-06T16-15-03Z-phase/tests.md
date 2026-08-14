# tests extract

## Relevance
Partial — webview-only, user-visible **layout** bug fix under `pulse-app/ui/**`; the tests domain contributes the webview unit-test discipline (vitest DOM-shape), the unconditional chunk gate set, the agent-driven no-visual-regression rule, and the tests↔a11y binding (p8 axe + chunk #87 disclosure must stay green). The Rust harness/E2E surfaces (§5/§6 P1–P7) are out of play.

## Constraints
- Webview unit tests run on **vitest 3.x + jsdom 26 + @testing-library/react 16**, co-located `*.test.tsx` under `pulse-app/ui/src/`, discovered via `include: ["src/**/*.{test,spec}.{ts,tsx}"]`, JUnit → `target/junit-ui.xml` — assertions are **DOM-shape only, never visual diff** (per test-plan.md §4 + §2). jsdom has **no layout engine** (0×0 `getBoundingClientRect`), so the bounded-popover/no-stretch fix CANNOT be asserted via pixel/geometry math in vitest — lock it structurally (testids, anchor attributes, presence within the band container) and leave the visual outcome to the operator verify.
- The **standard gate set is unconditional regardless of scope**; because this chunk touches `pulse-app/ui/**`, the webview gates (`lint`/`typecheck`/`test`, all `--prefix pulse-app/ui`) are additionally required alongside the always-on Rust gates (per test-plan.md §3 Per-chunk gate discipline).
- **Agent-driven discipline:** every check exits on a deterministic machine-parseable signal — no Percy/Chromatic/Applitools, no pixel inspection, no "human reviews canvas" as the gate (per test-plan.md §2 + §11 Universal/E2E).
- **No `sleep(N)` for synchronization** — the dropdown's async open/close, click-outside, focus-return, and reduced-motion paths must be driven by explicit signals (`userEvent` + `waitFor`/`findBy`), never timers (per test-plan.md §11 E2E).
- **No new webview line-coverage number applies:** the §10 ≥75% line / ≥70% branch / ≥85% function thresholds are Rust `cargo-llvm-cov` and exclude webview presentational components; the effective webview gate is `npm run test` green (per test-plan.md §4 + §10). Do not invent a webview coverage %.
- **Zero-flakiness budget** — any flake is quarantined + root-caused, never retry-masked (per test-plan.md §10).

## Patterns to follow
- Extend the existing `pulse-app/ui/src/widget/FindingsDropdown.test.tsx` in its own style: render with props, assert via `getByTestId("findings-dropdown" | "-list" | "-empty" | "-row" | "-mark-all-read")`, the `makeTriggerRef()` helper that appends a real `<button aria-haspopup>` to `document.body`, `afterEach` clearing `document.body.innerHTML`. New layout-lock assertions belong here (+ `FindingsCounter.test.tsx` / `CompactWidget.test.tsx` for the band mount).
- Preserve the already-present **chunk #87 disclosure tests** verbatim in intent: Escape → `onClose` + `document.activeElement === triggerRef.current`; `mousedown` outside → close, inside/on-trigger → no close (`userEvent.setup()` + `user.keyboard("{Escape}")`). These must stay green — the fix is positioning-only.
- The **p8 axe spec** `pulse-app/ui/tests-a11y/axe/p8-findings-dropdown.spec.ts` is Playwright + `installTauriIpcMock(page, v02WidgetOverrides, "compact-widget")` + `runAxeSweep({surface:"findings-dropdown"})` + open-via-`[data-testid="findings-band"] button[aria-haspopup]` + `getByTestId("findings-dropdown").waitFor("visible")` + row-count 3. If fixtures/assertions need updating for the layout change, follow this template (p8–p12 are the canonical shape per a11y §Harness shape); keep zero critical/serious violations.
- Reduced-motion verification uses `page.emulateMedia({reducedMotion:'reduce'})` (a11y rules) / the `useReducedMotion` motion-dom global-reset discipline in vitest (testing.md 2026-05-04) — reuse, don't reinvent.

## Anti-patterns to avoid
- Do NOT assert the "no window stretch / bounded popover / no white background" outcome via jsdom geometry (`getBoundingClientRect`, offset math) — it is structurally blind and yields false confidence (per test-plan.md §11 Universal + §4 DOM-shape-only).
- Do NOT weaken or delete the existing disclosure/axe assertions to make the positioning change pass — scope is layout/anchoring only, contract stays intact (scope §Preserve).
- Do NOT use a visual-regression tool or a `sleep`-based wait as the verification mechanism (per test-plan.md §11).

## Contract bindings
- **tests ↔ a11y (primary):** the p8 axe sweep + `keyboard-focus/widget-and-modals.spec.ts` + the chunk #87 disclosure contract ride the same test/CI surface (`npm run test:a11y` / `cargo xtask test:a11y` reuses the tests E2E driver per a11y-plan §3 + §CI gate); the layout fix must keep them green with no new violation tuple against the v0.2.0 baseline.
- **tests-harness ↔ obs (minimal):** the boot-smoke obs-log check greps the obs-declared `agent-latest.jsonl*` family (obs §Log format) — engaged only by the P3 boot smoke below; this chunk adds no new obs fields/targets.

## Acceptance criteria contributions
- (tests) `npm run test --prefix pulse-app/ui` passes, including new/updated **DOM-shape** assertions in `widget/FindingsDropdown.test.tsx` (+ `FindingsCounter`/`CompactWidget`) that lock the bounded-popover anchoring to the badge; the existing Escape / click-outside / focus-return tests stay green (test-plan.md §3 + §4).
- (tests) Webview gates green: `npm run lint` + `npm run typecheck` (`--prefix pulse-app/ui`); unconditional Rust baseline (`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace --profile ci`, `cargo xtask capability-drift`) passes with no Rust delta (test-plan.md §3).
- (tests/a11y) p8 axe (`tests-a11y/axe/p8-findings-dropdown.spec.ts`) stays at zero critical/serious violations and keyboard-focus stays green; reduced-motion respected (a11y §CI gate binding).
- (tests) **Warm boot smoke at implement P3** (webview-only *user-visible* surface per the 2026-07-05 operator directive): `npm run build --prefix pulse-app/ui` → `cargo build -p pulse-app` to re-embed fresh `ui/dist` → boot with a fresh `ANDROMEDA_PULSE_DATA_DIR` → seed → assert 0 `app.panic.fatal` / 0 ERROR + `app.boot.webview.init` + frame metrics in `agent-latest.jsonl*`; **and pair with an operator leave-running visual verify** because the obs-log smoke is layout-blind (test-plan.md §3 boot-smoke, as extended below).

## Relevant amendment history
From `.andromeda/test-plan-amendments.md`:
- **2026-05-10 — standard gate baseline unconditional per chunk.** Directly governs this chunk's `## Test Commands`: full Rust gate set + (because `pulse-app/ui/**` is touched) `npm run lint`/`typecheck`/`test`. Landed empirically at chunk #37 to stop gate-drift.
- **2026-05-09 — boot-smoke gate conditional on boot-path files** (`pulse-app/src/main.rs`, `crates/ui-bridge/src/`, `tauri.conf.json`, `capabilities/*.json`). This chunk touches **none** of those, so the *file-path-scoped* boot-smoke trigger does not fire on its own — but see the 2026-07-05 extension.
- **2026-06-10 — chunk #99 tag gate** notes the browser-driven a11y chain (real Chromium against the built webview) as standing P5 coverage; the p8 axe spec is that surface (tangential, confirms the a11y chain is the coverage for this popover).

From `.claude/rules/testing.md` Session Additions (distilled tests memory, not in the amendment file but load-bearing for THIS webview-layout chunk):
- **2026-07-05 — boot-smoke extends to webview-only user-visible chunks** (operator directive): run the warm boot smoke at implement P3 even for `ui/**`-only chunks; the Rust *test* gates may still defer on zero-Rust-delta, but the boot+render+seed always runs (Windows how-to: re-embed via `cargo build -p pulse-app`, poll `:4317`, seed `inject_demo`, verify obs-log family, clean up by specific PID via `Stop-Process`).
- **2026-07-05 — the obs-log boot smoke is LAYOUT-BLIND:** it proves render/no-panic, not correct rendering (occlusion, overflow, clipping pass every obs check). For any UI-layout/rendering chunk (this is exactly one), pair the automated smoke with an operator leave-running visual verify or an agent-read screenshot. This is the decisive verification note for this chunk.
- Cross-domain pointer (arch/frontend domain will extract, not mine): `frontend.md` **2026-06-30** documented the same `CompactWidget <main>` fixed-`height: calc(100vh - 32px)` vs `minHeight` tension and explicitly named the downward-popping `position:absolute; top: calc(100% + …)` findings dropdown as the reason `overflow:hidden` is not a usable cure — reinforcing why the fix and its proof are structural/visual, not jsdom-geometric.
