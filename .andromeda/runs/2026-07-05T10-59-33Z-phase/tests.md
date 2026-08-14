# tests extract

## Relevance
relevant — the webview Traces-table (anomaly-first ordering + semantic error token + filter control) is squarely a tests-domain surface (webview `*.test.tsx`), with at most a minor viz→ui-bridge `traces.query` contract touch.

## Constraints
- **Agent-driven, machine-parseable — no human-in-loop, no visual diff.** The "erroring-at-top / flagged / filterable" checks must be DOM-shape assertions readable via exit code + structured output, never screenshot/pixel review (per test-plan §1, §2 agent-runnable invariants).
- **Webview tests = vitest 3.x + jsdom 26 + @testing-library/react 16**, co-located `*.test.tsx` under `pulse-app/ui/src/`, JUnit XML to `target/junit-ui.xml`, DOM-shape only (per test-plan §4 webview framework, §2 conventions).
- **Standard gate set is unconditional and webview gates fire here** (`npm run lint`/`typecheck`/`test --prefix pulse-app/ui`) because the chunk touches `pulse-app/ui/**`, alongside `cargo fmt --check` / `clippy … -D warnings` / `nextest run --workspace --profile ci` / `xtask capability-drift` (per test-plan §3 per-chunk gate discipline).
- **Boot-smoke gate stays OFF unless a viz field threads through a boot path.** Editing only `pulse-app/ui/**` (+ viz crate) does not trigger it; it fires only if `crates/ui-bridge/src/` (or main.rs / tauri.conf.json / capabilities) is edited to thread a new field (per test-plan §3 Boot-smoke gate (conditional)).
- **The error signal already rides the `traces.query` contract** — its response fields include `error_count` (per test-plan §5 traces.query contract), supporting the chunk's PREFERRED path (consume existing row status, no new procedure).
- **Self-bootstrapping fixtures — no pre-baked data.** The mixed healthy+erroring dataset must be synthetic fixture props built at runtime, not `.sql`/snapshot/production dumps (per test-plan §7, §11 Test Data bans).

## Patterns to follow
- **Co-located DOM-shape `*.test.tsx`** modeled on existing dashboard-route tests (e.g. `pulse-app/ui/src/dashboard/routes/metrics/MetricsChart.test.tsx`); Vitest discovers via `include: ["src/**/*.{test,spec}.{ts,tsx}"]`; setup `pulse-app/ui/vitest.config.mjs` → `src/test-setup.ts` with `afterEach(cleanup)` (per test-plan §4, §1 webview row).
- **REAL DOM-event exercise via @testing-library** (`fireEvent`/`user-event`) on the actual filter control, asserting rendered rows change — the affordance-honesty rule; a never-wired handler must fail (chunk val-1; aligns test-plan §11 "test public boundaries, not implementation details" + no-`sleep`/explicit-signal).
- **DOM assertions for ordering + token:** assert first rendered row is erroring (DOM row order) and the semantic error token renders as color + badge/icon/text via `role`/`aria-*`/text + token class / `currentColor` (no hardcoded hex) — mirrors test-plan §4 webview component-test bullet (decorative↔meaningful ARIA flip, currentColor propagation).
- **If a viz field is added:** unit-test the aggregation in the viz crate `#[cfg(test)] mod tests` (§4 viz crate) and assert the field shape on the `traces.query` IPC contract via `tauri::test::mock_builder()` + `get_ipc_response()` (per test-plan §5 viz→ui-bridge boundary).

## Anti-patterns to avoid
- **NEVER use visual regression / pixel inspection / human-approved screenshots** (Percy/Chromatic/Applitools) — the top-of-table + token checks are DOM-order/attribute assertions, not canvas pixels (per test-plan §11 E2E + Universal agent-driven bans, §2).
- **NEVER assert a state-only / dead-control proxy** for the filter — a handler verified only via internal state, never dispatched through a real DOM event, is banned by the affordance-honesty anchor (chunk val-1) and §11 "test public boundaries".
- **NEVER pre-populate via `.sql`/snapshot or production telemetry dumps** — build the mixed dataset with synthetic factory props (per test-plan §11 Test Data, §7).

## Contract bindings
- **tests ↔ a11y (live):** the semantic error token must be non-color-only (SC 1.4.1) — asserted in the webview `*.test.tsx` and gated by the shared `npm run test` a11y-in-webview gate (test-plan §4 webview bullet; chunk scope design-system/a11y note).
- **tests ↔ design-system (live):** token class / `currentColor` propagation, no hardcoded hex, asserted via DOM (test-plan §4).
- **tests ↔ viz/arch data-spine:** `traces.query` `error_count` is the surfaced signal; any field addition binds the webview test to the §5 IPC contract test.
- **tests ↔ obs harness:** inherited only — §3 5-command `status`/`logs` (obs JSON shape / NDJSON path) are NOT extended by this webview chunk (no new subsystem/log field).

## Acceptance criteria contributions
- (tests) `npm run test --prefix pulse-app/ui` exits 0 for the new Traces-table `*.test.tsx` (per test-plan §3 gate set, §4).
- (tests) Affordance-honesty: the filter control is exercised by a REAL DOM event (`@testing-library` `fireEvent`/`user-event`) and rendered rows change; a never-wired handler fails the test (chunk val-1).
- (tests) With a mixed fixture (healthy + erroring), the first rendered row is erroring/flagged and the top row is NOT all-healthy — asserted via DOM order/attrs (not pixels), token non-color-only (P-068 matrix + test-plan §11 no-visual-regression / a11y SC 1.4.1).
- (tests) Full standard gate set green — `cargo fmt --check`, `clippy … -D warnings`, `nextest run --workspace --profile ci`, `xtask capability-drift`, `npm run lint`/`typecheck`/`test --prefix pulse-app/ui`; plus the `traces.query` contract test if a viz field is added (per test-plan §3, §5).

## Relevant amendment history
- **2026-05-10 — standard gate baseline mandatory** (§3/§9): every chunk's `## Test Commands` must list the full gate set incl. webview `npm run lint`/`typecheck`/`test` when `pulse-app/ui` is touched. Empirically triggered by chunk #36 omitting `cargo fmt --check` + `npx tsc --noEmit`, which then exposed 2 pre-existing errors in `pulse-app/ui/src/dashboard/routes/metrics/MetricsChart.test.tsx`. Binds this webview chunk directly — drop no webview gate.
- **2026-06-10 — capability verification matrix + `cargo xtask verify:capability-matrix` CI step** (§9): P-061+ capabilities must extend `capability-verification-matrix.json` in the chunk that lands them (CI fails on dangling ids/paths/anchors). P-068 > P-060, so landing it likely needs a matrix entry (mode automated-nextest or automated-a11y for a webview capability) — research should confirm whether v0.3.0 uses this v0.2.0 matrix or a successor file.
- **2026-05-09 — boot-smoke gate (conditional)** (§3): relevant only as a branch — if the viz field addition edits `crates/ui-bridge/src/`, the 60s `npx @tauri-apps/cli dev` smoke gate becomes required (it caught a latent `ui-bridge/health.rs` Tokio-reactor boot panic that the in-process harness missed across 4 wraps).
