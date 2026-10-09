# tests extract

## Relevance
relevant

## Constraints
- Per §4 Unit Test Strategy: co-located vitest 3.x tests (jsdom 26 + @testing-library/react 16) for new CompactWidget affordance control, DOM-shape assertions only (aria-label, focusable, className passthrough), no visual regression (§4 / §11 agent-driven discipline)
- Per §3 Per-chunk gate discipline: standard baseline required (cargo fmt, clippy, nextest, cargo xtask capability-drift) plus webview gates (npm lint/typecheck/test) because CompactWidget.tsx touched (§3)
- Per amendment 2026-05-09: boot-smoke gate required (npx @tauri-apps/cli dev 60s timeout) because pulse-app/capabilities/*.json touched (§3 Boot-smoke gate / amendment 2026-05-09)
- Per §3 Tauri capability gating: negative test required if new TauRPC procedure added (attempt undeclared call, assert rejection); positive test for declared procedure (assert success); Xtask drift check enforces crate/capability sync (§1 coverage trigger / §3)
- Per §4 Webview unit test pattern: factory fixtures via vitest fixture functions (parallel to Rust rstest pattern), real event dispatch (click/keyboard) must trigger affordance activation (§4 / §6 P5 agent-driven surrogate)
- Per amendment 2026-06-10: if new capability entry added to pulse-app/capabilities/*.json, cargo xtask verify:capability-matrix must pass (CI step validating matrix JSON ids/file-refs/grep-anchors) (§9 CI Integration)

## Patterns to follow
- Per §4: co-located *.test.tsx file adjacent to CompactWidget.tsx source; vitest `describe/test` blocks; DOM assertions using @testing-library/react queries (getByRole, getByLabelText, etc.)
- Per §3 Harness Contract: agent-parseable exit codes (test exit 0 = pass; non-zero = failure); structured output via vitest JUnit XML to target/junit-ui.xml (matches nextest shape)
- Per §6 E2E P5 surrogate: verify IPC contract consistency via TauRPC health command shape validation (JSON response with required subsystems fields)
- Per §8 Mocking: use vitest mocked @tauri-apps/api window module (or TauRPC route) in unit test; real event dispatch on rendered affordance control; assert mock calls to window.show() / window.setFocus() / window.unminimize() received

## Anti-patterns to avoid
- Per §4 webview: NEVER use inline SVG `<animate>`, `<animateTransform>`, `<set>`, or gradients in affordance icon/button (§4 Unit Test Strategy / banned-element grep)
- Per §11 Universal: NEVER use visual regression (Percy/Chromatic) requiring human approval; NEVER include manual verification step ("developer confirms button click works"); NEVER use real network calls (assertion on IPC contract only, per agent-driven discipline) (§11 Universal / §6 E2E agent-driven-discipline)
- Per §11 E2E: NEVER use `sleep(N)` for synchronization (use explicit signal polling or Channel subscription); NEVER include "human reviews window state" in test steps (agent verifies IPC contract shape only, not screenshot) (§11 E2E / §6 P5 Current residual)

## Contract bindings
- **5-command harness** (boot/run/status/cleanup/logs in chunk's Test Commands) binds to obs §3 (status endpoint JSON shape; log format NDJSON with required fields; PID-file location)
- **Tauri capability entries** (new pulse-app/capabilities/*.json grant for window API or TauRPC procedure) bind to Tauri 2 security model (negative-default: without explicit grant, IPC call silently rejected per §3 anti-pattern "NEVER add TauRPC procedure without matching capability entry")
- **TauRPC IPC contract** (if route uses new procedure or calls existing window-show procedure) binds to architecture Standard Contracts (traces.query / health / ready response shape; AppError sanitization — NEVER expose stack traces/file paths/Rust struct names per §11 project-specific)

## Acceptance criteria contributions
- "(tests) npm run test --prefix pulse-app/ui passes: CompactWidget affordance component vitest suite (real click/keyboard event dispatch, assert window.show() + window.setFocus() mock calls received or TauRPC route equivalent)"
- "(tests) CompactWidget affordance DOM contract: aria-label or aria-describes set, focusable (tabIndex or role with native focus), keyboard Enter/Space triggers activation (vitest @testing-library/react userEvent.keyboard / click)"
- "(tests) Boot-smoke gate: npx @tauri-apps/cli dev 60s timeout succeeds; stdout contains boot-completion signal (Local: / ready in / Compiled successfully) before SIGTERM; no panic at capability load"
- "(tests) Cargo xtask capability-drift: new capability entry in pulse-app/capabilities/*.json syncs with pulse-app/src/command/*.rs TauRPC procedure definition (if TauRPC route chosen); Xtask validates match"
- "(tests) Cargo fmt --check, cargo clippy --workspace --all-targets, cargo nextest run --workspace --profile ci all pass"

## Relevant amendment history
- **2026-05-09 — Boot-smoke gate (conditional):** Chunks touching `pulse-app/capabilities/*.json` MUST include runtime smoke gate: `npx @tauri-apps/cli dev` 60s timeout, watch stdout for boot-completion signals, verify no panic (particularly trait-level binding-emission panics at boot in non-Tokio context). This chunk touches capabilities/*.json, so boot-smoke gate is required. (Per amendment marker `.andromeda/runs/2026-05-09T16-00-54-spec-amendment-smoke-check-boot-discipline/amendment.md`)
- **2026-05-10 — Standard gate baseline (unconditional):** Every chunk plan's Test Commands MUST list: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo nextest run --workspace --profile ci` + `cargo xtask capability-drift`. Since this chunk touches pulse-app/ui/**, also include `npm run lint --prefix pulse-app/ui` + `npm run typecheck --prefix pulse-app/ui` + `npm run test --prefix pulse-app/ui`. Gate-coverage drift accumulated across prior sessions when baselines were omitted; now mandatory unconditionally. (Per amendment marker `.andromeda/runs/2026-05-10T12-41-03-spec-amendment-mandate-standard-chunk-gates/amendment.md`)
- **2026-06-10 — Capability verification matrix (CI step):** If chunk adds new capability entry to pulse-app/capabilities/*.json, CI step `cargo xtask verify:capability-matrix` must pass (validates `docs/v0_2_0/capability-verification-matrix.json` ids/file-refs/grep-anchors; future capability additions must extend the matrix JSON in the same chunk landing the capability). (Per amendment marker `.andromeda/runs/2026-06-10T00-35-00-spec-amendment-adopt-four-load-profiles/amendment.md`, Section "Capability verification matrix")
