# tests extract

## Relevance
Relevant (partial) — webview unit-test surface (a React data hook + table component); tests-domain owns timer determinism + the per-chunk gate discipline. Rust test surface is conditional on the CARRY (viz cursor) only.

## Constraints
- Webview unit tests use `vitest` 3.x + `jsdom` 26 + `@testing-library/react` 16, co-located `*.test.ts(x)` adjacent to source, DOM-shape assertions ONLY (no visual diff), emitting JUnit XML to `target/junit-ui.xml` — per test-plan §1 ("desktop-webview unit tests" surface row) + §4.
- The interval re-poll MUST be exercised deterministically — no real `setTimeout`, no `sleep(N)`, no wall-clock time; time is injected/faked — per test-plan §2 (agent-runnable invariants: "no real `setTimeout`"), §11 Universal ("NEVER use real time … without injection") and §11 E2E ("NEVER use `sleep(N)` for synchronization").
- Zero-flakiness budget: an async/interval test that flakes is quarantined + root-caused, never retry-masked — per test-plan §10 (Zero-flakiness) + §11 Quality.
- The full standard gate set is unconditional this chunk, and the webview gates (`npm run lint` / `typecheck` / `test --prefix pulse-app/ui`) fire because `pulse-app/ui/**` is touched — per test-plan §3 ("Per-chunk gate discipline → standard gate set").
- Fixtures self-bootstrap through the mocked IPC seam (no raw pre-baked snapshots); the hook test drives the proxy boundary, not internals — per test-plan §7 + §8 ("mock only traits/boundaries you own").
- §3-body boot-smoke gate is scoped to boot-path files (`main.rs` / `crates/ui-bridge/src/` / `tauri.conf.json` / `capabilities/*.json`) — this chunk touches NONE, so that body trigger does not fire; the warm-boot obligation comes from the operator extension instead (see Acceptance) — per test-plan §3 (Boot-smoke gate conditional).

## Patterns to follow
- Constellation ~1s poll precedent (the in-app model the scope names): `pulse-app/ui/src/hooks/use-service-constellation.ts` — `poll()` on mount + `window.setInterval(poll, 1000)` + `focus` listener, silent `.catch` keeping last value, `clearInterval` on the effect-cleanup return (no leaked timer). `.claude/rules/frontend.md` 2026-07-06 names this exact fetch-once-stale bug class + prescribes this precedent.
- Existing hook test shape to extend: `pulse-app/ui/src/dashboard/routes/traces/use-traces.test.ts` — `renderHook(() => useTraces(...))` + the `__setProxyForTest({ traces: { query: queryFn } })` seam + `waitFor`. Add fake timers on top for the re-poll assertions; the modify target is `use-traces.ts` (currently one `useEffect` fetch, cancels on unmount).
- Fake-timer determinism (`.claude/rules/testing.md` 2026-07-02): `vi.useFakeTimers({ toFake: [...] })` faking ONLY what's needed — a bare `vi.useFakeTimers()` also fakes `setTimeout`/`setInterval` and stalls RTL `waitFor`/`findBy*` + React scheduling; advance with the async variant (`await vi.advanceTimersByTimeAsync(1000)`) so the pending `traces.query` promise flushes between ticks; restore in `try/finally` (vitest does not auto-restore timers/globals/env — 2026-05-08 / 2026-06-30).
- React-19 effect double-invoke (`.claude/rules/testing.md` 2026-05-08): for effect-driven mock call-counts prefer `toHaveBeenCalled()` / `≥ N` over exact `toHaveBeenCalledTimes(N)`.
- Filter/sort state is component-local in `TraceTable.tsx` (`const [errorsOnly] = useState(false)`, `aria-pressed={errorsOnly}`, filtered view via `useMemo([rows, sortState, errorsOnly])`) — a hook re-poll that only updates the `rows` prop re-derives the filtered view WITHOUT resetting `errorsOnly`/`sortState`; the test asserts this survives a tick (contrast: a remount would reset it).

## Anti-patterns to avoid
- NEVER assert the refresh via real time / `setTimeout` / `sleep` — advance a fake clock (test-plan §11 Universal + §11 E2E).
- NEVER rely on pixel / visual-regression / human-approval screenshots as the automated gate; jsdom has no layout engine (`getBoundingClientRect` = 0×0), so empty→populated "looks right" is NOT vitest-assertable — assert DOM/state (rows present, empty-state gone, `aria-pressed`) and leave pixels to the operator verify (test-plan §11 E2E/Universal; `.claude/rules/testing.md` 2026-07-05 layout-blind).
- NEVER test hook internals — assert the observable contract (query re-invoked, table leaves "No traces yet", filter/sort preserved, timer cleared on unmount) (test-plan §11 Unit).

## Contract bindings
- tests ↔ a11y: the `aria-pressed` (Errors-only) preservation and the P4 "live updates do NOT steal focus / `aria-live` polite live list" behavior are asserted in the same webview vitest DOM-shape suite that runs under the tests §3 `run` command / CI (a11y-plan P1 Traces table + P4 live trace list; test-plan §1 unit-row "`aria-*` attrs").
- tests ↔ obs: effectively none new — the re-poll reuses the existing `viz.query.traces` (its `tracing` target already exists); no new status-endpoint/log-format surface is introduced, so the 5-command status/log binding is not newly exercised.
- tests ↔ viz/arch (CONDITIONAL — CARRY): only if pagination/cursor is touched, a Rust `viz` co-located unit test (`crates/viz/src/query.rs` `#[cfg(test)] mod tests`) must cover `next_cursor` keyed on `end_time` (no skip/duplicate across pages), and `cargo nextest run --workspace` + the deferred `clippy` / `capability-drift` re-runs come due (scope §Gate note). Default expectation per scope: CARRY re-defers.

## Acceptance criteria contributions
- (tests) `npm run test --prefix pulse-app/ui` passes with a new/extended `use-traces.test.ts` that advances a fake interval timer and asserts the mocked `traces.query` is re-invoked (≥2 calls) and the hook `rows` update lands → the table transitions out of "No traces yet" — using fake timers, never real time (§2/§4/§10).
- (tests) Timer-lifecycle test: after unmount, advancing timers yields no further `query` calls (interval cleared, no leaked timer) — the constellation cleanup-return precedent (§11 determinism).
- (tests) State-preservation test: with Errors-only toggled (`aria-pressed=true`) and a sort applied, a re-poll tick preserves `aria-pressed` + sort and does not blank rows mid-poll (no flicker defeating the anomaly-first read) (§11 Unit / DOM-shape).
- (tests) Standard gate set green — `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace --profile ci`, `cargo xtask capability-drift` + the three webview gates; PLUS a warm-boot smoke + operator leave-running visual verify that the Traces table populates under live telemetry with filter state intact (operator directive `.claude/rules/testing.md` 2026-07-05 extends §3 boot-smoke to user-visible webview-only chunks; jsdom cannot prove empty→populated). If CARRY activates, the viz cursor test + deferred `.rs` gate re-runs also gate (scope §Gate note).

## Relevant amendment history
- 2026-05-10 "chunk plans MUST include the standard gate baseline" (test-plan-amendments.md → §3) — directly sets this chunk's gate list: full Rust gate set unconditionally + webview `npm run lint/typecheck/test` because `pulse-app/ui/**` is touched. Its empirical trigger was a dashboard-route webview test (`MetricsChart.test.tsx`), a direct sibling of this chunk's traces-route tests.
- 2026-05-09 "per-chunk Tauri-dev runtime smoke gate" (test-plan-amendments.md → §3) — establishes the boot-smoke gate; its file-scoping (boot-path files) means the *body* trigger does NOT fire here, but it is the anchor the 2026-07-05 operator directive extends to warm-boot this user-visible webview chunk. (Other sidecar entries — 2026-05-02 initial, the three 2026-05-08 self-OTLP/PII/capability entries, 2026-06-10 load-profiles/capability-matrix — are out of this chunk's area.)
