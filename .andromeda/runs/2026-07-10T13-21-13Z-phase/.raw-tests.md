# tests extract

## Relevance
Relevant — webview (React) + Tauri second-window + `core:window` capabilities + boot-path chunk; squarely in the tests domain (per test-plan.md §3/§4/§6-P5).

## Constraints
- Standard gate set is unconditional in the chunk's `## Test Commands`, and the webview gates (`npm run lint` / `typecheck` / `test --prefix pulse-app/ui`) are INCLUDED because `pulse-app/ui/**` is touched (per test-plan.md §3 Per-chunk gate discipline).
- Boot-smoke gate is MANDATORY: the chunk edits `pulse-app/capabilities/*.json` (and likely `pulse-app/src/main.rs` / `crates/ui-bridge/src/` for a `WebviewWindowBuilder`) — all listed boot/setup paths. The in-process 5-command harness does not catch boot-time binding-emission panics or silently-rejected `core:window` IPC grants (per §3 Boot-smoke gate).
- Window geometry / always-on-top / below-widget positioning / multi-monitor clamp / cross-window focus are NOT agent-driven testable — they are the P5 headful residual; the agent-driven layer asserts IPC contract + DOM-shape only, never pixels/window position (per §6 P5, §1 agent-driven-discipline trigger).
- New webview React logic is unit-tested with vitest 3.x + jsdom 26 + @testing-library/react 16, DOM-shape assertions only, co-located `*.test.tsx` under `pulse-app/ui/src/` (per §4).
- The `use-findings.ts` re-poll CARRY (~1s cadence) and any dismiss-timing logic MUST use fake timers, never real time or `sleep(N)` (per §11 Universal + E2E anti-patterns).
- Webview presentational components are EXCLUDED from the 75/70/85 coverage gate at Foundation pre-shell stage; behavioural logic (render-branch selection, use-findings, dismiss lifecycle) is still tested (per §4 coverage target, §10).

## Patterns to follow
- Co-located `*.test.tsx` adjacent to source, discovered via vitest `include: ["src/**/*.{test,spec}.{ts,tsx}"]`; `setupFiles` registers `afterEach(cleanup)`; emits JUnit to `target/junit-ui.xml` for agent-driven discipline (§4).
- DOM-shape a11y assertions via @testing-library: focus-move into the panel and Esc focus-restore to the unread badge asserted at role/`aria-*`/`document.activeElement` level (the icon ARIA-flip + focus pattern precedent) (§4 webview React components).
- IPC-contract surrogate for window behaviour (P5 pattern): assert TauRPC/IPC response shape + `cargo build` ACL compile-embed rather than window geometry (§6 P5 agent-driven surrogate).
- Fake-timer cadence testing — the webview analog of the plan's `tokio::time::pause()` / `advance()` time-injection discipline — for the re-poll badge-refresh test (§7 time-sensitive data, §8 time mocking).

## Anti-patterns to avoid
- NEVER visual-regression / pixel-inspect / "human reviews the window" for position, opacity, or always-on-top — assert IPC contract + ACL compile instead (§11 E2E + Universal).
- NEVER `sleep(N)` or real wall-clock for the re-poll/dismiss timing — inject fake timers (§11 E2E + Universal).
- NEVER widen the 3 never-widen caps (`pulse:notification` / `pulse:tray` / `pulse:plugin-fs`); grant only the `core:window` / `core:webview` operations the panel needs (§1 pending "Capability widening static analysis"; scope §5).

## Contract bindings
- tests ↔ a11y: the chunk #87 disclosure a11y contract (cross-window focus-move + Esc-restore) rides the webview test surface (vitest DOM-shape) plus the browser-driven a11y chain that covers the P5 window residual (§6 P5 "current residual"; focus-guide a11y-CI-gate binding).
- tests ↔ arch/capabilities: boot-smoke + `cargo build` (ACL compile-embed) is the `core:window` grant-validity gate; `cargo xtask capability-drift` + capability-widening static-analysis bind to the `pulse-app/capabilities/` manifest (§3, §1).
- tests ↔ obs: no new subsystem or `health` status field this chunk; boot-smoke only tails obs-declared stdout boot-completion signals (`Local:` / `ready in` / `Compiled successfully`) — no new binding (§3).

## Acceptance criteria contributions
- (tests) Standard gate set green including webview gates (`npm run lint` / `typecheck` / `test --prefix pulse-app/ui`) since `pulse-app/ui/**` is touched (§3).
- (tests) Boot-smoke gate passes — `cd pulse-app && npx @tauri-apps/cli dev` reaches a boot-completion signal within 60s then SIGTERM — because `capabilities/*.json` and the new-window boot wiring changed (§3 Boot-smoke gate).
- (tests) Co-located vitest `*.test.tsx` cover: window-label branch renders Findings content; dismiss lifecycle (Esc / blur / row-select / mark-all-read); `use-findings` re-poll updates the badge without a manual resize/focus, using fake timers (§4, §11).
- (tests) `cargo xtask capability-drift` + capability-widening static-analysis stay clean, the 3 never-widen caps are unchanged, and `cargo build` succeeds as the grant-validity gate (§3, §1).

## Relevant amendment history
- **2026-05-09 — Boot-smoke gate for boot-path chunks:** directly on point — `capabilities/*.json` + `crates/ui-bridge/src/` + `main.rs` are the exact listed boot paths, and the empirical root cause was a ui-bridge boot-time "no reactor running" panic latent across 4 chunks; the new window + `core:window` grants fire at boot, so this gate guards this chunk.
- **2026-05-10 — Standard gate baseline mandatory regardless of scope:** mandates the full gate set incl. the webview `lint`/`typecheck`/`test` trio (root cause: `cargo fmt --check` + `tsc` drift surfaced N chunks later at #36/#37).
- **2026-06-10 — Chunk #99 tag gate (P5 residual + capability matrix):** (a) the P5 tray/window matrix is covered by the IPC surrogate + browser-driven a11y chain while `tauri-driver` headful stays deferred — this chunk's window positioning/always-on-top IS that documented residual, not a coverage gap; (b) `verify:capability-matrix` expects P-061+ additions to extend the matrix JSON in the landing chunk, but scope §Origin states no new capability id (P-061..P-082 all claimed), so no matrix row is added here.
- **2026-05-08 — Capability-widening static-analysis trigger:** documents the xtask check asserting the 3 never-widen caps hold their permission shapes; relevant because this chunk edits `pulse-app/capabilities/` to add `core:window`/`core:webview` grants.