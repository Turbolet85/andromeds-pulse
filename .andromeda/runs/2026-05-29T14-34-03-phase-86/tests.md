# tests extract — phase-86

## Chunk relevance

- route#89 "Header redesign: connection dot + chrome cleanup" — relevant (webview-only: vitest component-test surface for the three touched `.tsx` files + standard chunk-gate baseline; no Rust nextest test work, no E2E P-path, no harness/5-command wiring).

## Constraints

- Webview component tests use `vitest` 3.x + `jsdom` 26 + `@testing-library/react` 16; co-located `*.test.tsx` adjacent to source under `pulse-app/ui/src/` (per test-plan §4 webview framework row + §Surfaces-under-test "desktop-webview unit tests"). All three touched files (`Titlebar.tsx`, `FooterBand.tsx`, `CompactWidget.tsx`) already have a sibling `.test.tsx`; the FooterBand removal MUST also delete/retarget `FooterBand.test.tsx`, and the Titlebar/CompactWidget tests MUST be updated to cover the new connection-dot + chrome-cleanup behavior.
- DOM-shape assertions ONLY — no visual diff, no pixel inspection, no Percy/Chromatic (per test-plan §4 "DOM-shape assertions only (no visual diff per agent-driven discipline)" + §11 Universal agent-driven bans). The connection dot is verified via DOM/ARIA shape (color attribute, `aria-label`, tooltip text, presence/absence of footer band), not by rendering appearance.
- Standard chunk-gate baseline is UNCONDITIONAL and webview gates ARE in scope because the chunk touches `pulse-app/ui/**`: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo nextest run --workspace --profile ci` + `cargo xtask capability-drift` + `npm run lint --prefix pulse-app/ui` + `npm run typecheck --prefix pulse-app/ui` + `npm run test --prefix pulse-app/ui` (per test-plan §12 Decisions Log 2026-05-10 "chunk-gate-baseline-coverage" + testing.md §Pending coverage triggers).
- Coverage gate (Standard tier ≥75% line / ≥70% branch / ≥85% function) applies to Rust crates; webview presentational components are EXCLUDED from the Rust coverage gate at the documented scope boundary — new component tests still required for behavior verification but do not feed the Rust LCOV threshold (per test-plan §4 coverage-target paragraph + §10).
- Zero-flakiness budget: any flaky webview test is quarantined + root-caused immediately, never retry-once (per test-plan §10 + §11 CI/Quality bans).
- Removing the footer band is a NEGATIVE-presence assertion: a test MUST assert the "Ingest"/"Error"/"Retention" footer metrics are NOT rendered (the P-024 ambient-invariant proof), in addition to asserting the new connection dot IS rendered (per test-plan §11 "assert IPC/DOM contract" agent-driven discipline; this chunk has no E2E path so the contract proof lives at the component level).

## Patterns to follow

- Existing FooterBand/Titlebar test shape: `describe(...)` + `it(...)` blocks using `render(...)` + `screen.getByRole/getByText/getByTestId`, asserting `tagName`, `aria-*`, `data-testid`, and inline `style.*` token strings (per `pulse-app/ui/src/widget/FooterBand.test.tsx` + `pulse-app/ui/src/components/Titlebar.test.tsx`, both following test-plan §4 conventions). New connection-dot tests follow the same shape (e.g., `screen.getByTestId("connection-dot")`, assert `aria-label` matches "Connection: <state> — last span <ago>", assert color encodes state).
- Tauri runtime-API mocking for the connection-state subscription: `vi.mock(...)` at top of file for the bindings/Channel proxy; the connection dot subscribes to the EXISTING `pulse://stream/connection-state` topic / `connection.current_state` TauRPC (chunk #59), so the test mocks the `createTauRPCProxy` callback shape and captures the callback for synthetic state-event injection (per testing.md Session Additions 2026-05-16 taurpc-callback-shape + 2026-05-09 webview-API mock pattern). No new procedure to mock.
- Focus-order / no-new-focusable assertions on the redesigned titlebar can use `tabbable(container)` to confirm the existing expand-to-dashboard button affordance is preserved and the dot (non-interactive on hover) does not introduce an unintended focusable element (per testing.md Session Additions 2026-05-09 tabbable pattern + Titlebar.test.tsx existing "no tabindex" assertions).
- Mock-state hygiene: per-file `afterEach` calling `vi.unstubAllGlobals()` / `vi.mocked(...).mockReset()` for any stubbed Tauri API or global, since the shared `test-setup.ts` `afterEach(cleanup)` only unmounts the DOM (per testing.md Session Additions 2026-05-08 + 2026-05-09).

## Anti-patterns to avoid

- NEVER include human-review / "verify the dot looks right" / screenshot-comparison steps — connection-dot correctness is DOM/ARIA-contract verified, not visual (per test-plan §11 Universal agent-driven bans + §4 DOM-shape-only discipline).
- NEVER assert against `getByRole("status")` ambiguously if the redesigned titlebar/widget renders more than one `role="status"`/live region — disambiguate via `data-testid` (per testing.md Session Additions 2026-05-19 role-status-ambiguity lesson; directly relevant since FooterBand currently IS a `role="status"` region being removed and a tooltip/dot may add a new announced region).
- NEVER leave `FooterBand.test.tsx` asserting a now-deleted component — stale tests on removed surfaces fail the `npm run test` gate (per test-plan §11 Quality "no known test failures").

## Contract bindings

- **tests ↔ a11y**: the connection dot's `aria-label` ("Connection: <state> — last span <ago>") and focus-order are a11y-owned assertions; tests provide the runnable vitest checks that verify the DOM shape a11y specifies. The a11y CI gate (axe-core/Playwright) reuses the tests' webview surface per the shared-driver binding, but this chunk's component-level proof is the vitest layer (per test-plan §1 surfaces "desktop-webview unit tests" + focus-guide cross-domain bindings "A11y CI gate binds to a11y §3.5 phase 6").
- **tests ↔ obs**: (none load-bearing for this chunk) — no new tracing target, no harness/5-command/status-endpoint/log-format change; the chunk subscribes to an existing broadcast topic consumer-side only.

## Acceptance criteria contributions

- (tests) `npm run test --prefix pulse-app/ui` passes — new/updated vitest specs for `Titlebar.test.tsx` + `CompactWidget.test.tsx` cover the connection dot (presence, state-driven color, `aria-label`, tooltip-on-hover text) and assert the footer-band metrics ("Ingest"/"Error"/"Retention") are NOT rendered; `FooterBand.test.tsx` is deleted or retargeted in the same commit.
- (tests) Webview gate set passes: `npm run lint --prefix pulse-app/ui` + `npm run typecheck --prefix pulse-app/ui` + `npm run test --prefix pulse-app/ui` all exit 0 (per test-plan §12 2026-05-10 chunk-gate-baseline).
- (tests) Full standard Rust gate set stays green: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo nextest run --workspace --profile ci` + `cargo xtask capability-drift` (capability-drift must report no drift since no new TauRPC procedure is introduced).
- (tests) Connection dot subscribes to the existing `pulse://stream/connection-state` topic via a mocked taurpc callback in tests (no live network); synthetic state-event injection updates the dot's asserted `aria-label`/color, verified deterministically with no `sleep()` (per test-plan §11 synchronization ban + testing.md 2026-05-16 callback-capture pattern).
