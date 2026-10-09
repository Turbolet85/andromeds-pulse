# tests extract

## Relevance
relevant

## Constraints
1. Per §3 Test Harness Contract — 5-command discipline mandatory (boot / run / status / cleanup / logs); boot readiness confirmed via TauRPC `health` command polling, not manual inspection (per agent-driven-discipline §3).
2. Per §3 Boot-smoke gate (conditional) — since chunk touches `pulse-app/src/main.rs` + `pulse-app/src-tauri/tauri.conf.json` (boot-path), MUST include runtime smoke gate: `cd pulse-app && npx @tauri-apps/cli dev` with 60s timeout, watching stdout for boot-completion signals before SIGTERM. On smoke failure the chunk surfaces "green per scope; runtime blocked" rather than blocking commit (per amendment 2026-05-09).
3. Per §3 Per-chunk gate discipline — MUST include full standard gate set unconditionally (per amendment 2026-05-10 chunk-gate-baseline-coverage): `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace --profile ci`, `cargo xtask capability-drift`, PLUS webview gates `npm run lint --prefix pulse-app/ui`, `npm run typecheck --prefix pulse-app/ui`, `npm run test --prefix pulse-app/ui` (since webview titlebar component touched).
4. Per §4 Unit Test Strategy — webview React components must use vitest 3.x + jsdom 26 + @testing-library/react 16, co-located as `*.test.tsx` adjacent to source; DOM-shape assertions only (no visual rendering); assert drag-region attribute presence and interactivity preservation (titlebar controls clickable).
5. Per §6 E2E Test Strategy — P-061 acceptance explicitly requires e2e verification of window opening behavior; agent-driven surrogate via IPC contract (TauRPC `health` consistency, window metadata fields if present); full headful tauri-driver GUI automation (window resize, position verification via pixel coordinate read) documented as P5 residual per chunk #99 tag gate (currently deferred, covered by browser-driven a11y chain).
6. Per §10 Quality Gates — cumulative coverage ≥ 75% line / ≥ 70% branch / ≥ 85% function (Standard tier); exclude test fixtures and generated code; performance budget for boot-time window initialization not explicitly gated (component-level, not on critical path).

## Patterns to follow
1. Per §3 "Per-chunk gate discipline" — test commands must include the unconditional baseline set + boot-smoke gate (not listed separately; integrated into test plan).
2. Per §4 Unit tests pattern — co-located `*.test.tsx` for React titlebar component; DOM-shape contract test: assert `data-tauri-drag-region` attribute present, parent `role` and `aria-*` attributes unchanged, button/interactive children preserve `onClick` handlers (use `@testing-library/react` `fireEvent` or `userEvent` to assert click propagation).
3. Per §6 P5 IPC surrogate pattern — invoke TauRPC `health` command at boot; assert response shape includes subsystems metadata; verify no new subsystem fields introduced (window_state is optional, deferred).
4. Per §7 Fixture pattern — if Tauri window config tested at Rust level (unlikely for this geometry chunk; primarily declarative config), use rstest fixtures for config variants (e.g., centered vs remembered position).

## Anti-patterns to avoid
1. Per §11 "NEVER use `sleep(N)` for synchronization" — window-ready synchronization must poll TauRPC `health` command (waiting for subsystems ready), not hardcoded delay.
2. Per §11 "NEVER include manual smoke step" — smoke gate (boot-time window initialization) is automated via CLI timeout + stdout signal detection, no developer manual verification.
3. Per §11 E2E "NEVER include visual regression checks requiring human approval" — window position / size verification must be programmatic (e.g., reading window bounds via IPC metadata or headful tauri-driver frame buffer coordinate read), not screenshot comparison.

## Contract bindings
5-command harness (§3) binds to obs-plan §3 JSON log format + status endpoint shape; window boot-time initialization ties to boot-smoke discipline (amendment 2026-05-09) for Tauri lifecycle verification.

## Acceptance criteria contributions
1. (tests) `cargo nextest run --workspace --profile ci` passes for new titlebar component unit tests; `npm run test --prefix pulse-app/ui` passes (vitest React component DOM-shape assertions on drag-region attribute).
2. (tests) Boot-smoke gate `npx @tauri-apps/cli dev` (60s) completes successfully, logging boot-completion signals without panic (catches trait-level binding-emission panics at boot per amendment 2026-05-09).
3. (tests) E2E path P-061 verification: app boots with TauRPC `health status == "ok"` within 10s; assert window opened at configured default size (compare against tauri.conf.json `width` / `height` via headful tauri-driver coordinate read or IPC metadata if available). Titlebar drag region functional via headful tauri-driver click-drag + window position delta > 0 assertion (full P5-style GUI automation; currently deferred per chunk #99 residual, agent-driven surrogate is IPC contract only).
4. (tests) Coverage: new-code line coverage ≥ 75% (Standard tier §10).

## Relevant amendment history
- **2026-05-09 — Boot-smoke gate** (amendment marker `.andromeda/runs/2026-05-09T16-00-54-spec-amendment-smoke-check-boot-discipline/amendment.md`): Chunk touches boot/setup paths (`pulse-app/src/main.rs`, `pulse-app/src-tauri/tauri.conf.json`), so MUST include runtime smoke gate `npx @tauri-apps/cli dev` (60s timeout; watch stdout for `Local:` / `ready in` / `Compiled successfully` before SIGTERM). Catches trait-level binding-emission panics at boot in non-Tokio context (empirically surfaced at chunk #31 wrap catching chunk #27/#30 latent panic). On smoke failure: chunk surfaces "green per scope; runtime blocked" not commit-blocking.
- **2026-05-10 — Chunk-gate baseline coverage** (amendment marker `.andromeda/runs/2026-05-10T12-41-03-spec-amendment-mandate-standard-chunk-gates/amendment.md`): Every chunk plan MUST include unconditional standard gate set (`cargo fmt --check`, `clippy`, `nextest`, `xtask capability-drift`, webview gates when UI touched). Verified empirically at chunk #37 wrap — chunk #36 omitted gates exposing pre-existing failures. Without unconditional baseline, coverage drift accumulates.