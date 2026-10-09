# tests extract

## Relevance
Relevant — touches boot-path files (tauri.conf.json, main.rs) requiring standard gates + boot-smoke gate; config-level window constraints have no webview/IPC surface.

## Constraints
- **Per §3 (5-command discipline):** all chunks implement the full harness cycle (boot/run/status/cleanup/logs); `boot` readiness signal via `health` TauRPC (status == "ok").
- **Per §3 (boot-smoke gate, conditional):** this chunk touches boot-path (`tauri.conf.json` + `src/main.rs`), so `cd pulse-app && npx @tauri-apps/cli dev` with 60s timeout watching for `Local:` / `ready in` / `Compiled successfully` before SIGTERM is mandatory; on failure: "green per scope; runtime blocked" (does not block commit but surfaces the issue).
- **Per §3 (standard gate baseline):** unconditional requirement for all chunks: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace --profile ci`, `cargo xtask capability-drift` (expected no-op; no capability changes in scope). Webview gates (`npm run lint/typecheck/test --prefix pulse-app/ui`) excluded — this chunk does not touch `pulse-app/ui/**`.
- **Per §1 (test tier Standard):** Unit coverage ≥ 75% line / 70% branch / 85% function; integration required for Standard Contracts tested.
- **Per §10 (quality gates):** Standard tier gates: `cargo-llvm-cov` 0.8.5 coverage + zero-flakiness enforcement.

## Patterns to follow
- **Config-level testing:** tauri.conf.json `minWidth`/`minHeight` fields parsed correctly; if aspect-ratio is runtime-enforced via Tauri API or resize-event clamp, unit test the bounds-check logic (e.g., `#[test] fn test_resize_clamp_below_min_rejects()`).
- **Self-bootstrapping fixtures:** if integration test needed, use in-process DuckDB or config builder (no pre-baked JSON fixtures).
- **Boot-smoke discipline:** per §3, chunk plan MUST include the smoke command and verify no panics during `cargo run --bin pulse-app` startup (catches the boot-time binding-emission panic class per the 2026-05-09 amendment precedent).

## Anti-patterns to avoid
- **No manual verification:** Window resize behavior must NOT require headful e2e (tauri-driver) for in-chunk validation; unit/config proof suffices (e.g., config schema validation + unit test of clamp logic). Headful resize-delta assertion may CARRY to Epoch-4 e2e (P-076) if pinned harness cannot drive it.
- **Never skip boot-smoke gate:** This chunk modifies boot-path; omitting smoke check exposes the "no reactor running" panic risk per the 2026-05-09 history.

## Contract bindings
- **5-command harness ↔ obs §3:** `health` TauRPC response shape (status, subsystems, uptime_ms) unchanged; window constraints are config/boot-side, not IPC-visible.
- **(none other)** — no new TauRPC procedures, no new capability grants, no broadcast topics added.

## Acceptance criteria contributions
- **(tests) Standard gate baseline passes:** `cargo fmt --check`, `cargo clippy --workspace`, `cargo nextest run --workspace --profile ci`, `cargo xtask capability-drift --check` all exit 0.
- **(tests) Boot-smoke gate passes:** `cd pulse-app && npx @tauri-apps/cli dev` boots without panic, stdout logs `Local:` or `ready in`, terminates cleanly on SIGTERM within 60s.
- **(tests) Config validation:** tauri.conf.json `minWidth` / `minHeight` fields present and valid (unit test via schema parser); if aspect-ratio runtime-enforced, unit test clamp logic returns valid bounds.
- **(coverage)** Config parsing path: ≥ 75% line coverage (expected minimal; config read is typically <20 lines).

## Relevant amendment history
- **2026-05-09 — Boot-smoke gate discipline (§3 Per-chunk gate discipline → Boot-smoke gate (conditional)):** Chunks touching boot-path (`pulse-app/src/main.rs`, `pulse-app/src-tauri/tauri.conf.json`, etc.) MUST include runtime smoke check (`npx @tauri-apps/cli dev`, 60s timeout). Rationale: in-process 5-command harness does not catch trait-level binding-emission panics at boot in non-Tokio context. **THIS CHUNK APPLIES** (touches tauri.conf.json + main.rs).
- **2026-05-10 — Standard gate baseline (§3 Per-chunk gate discipline → standard gate set):** Every chunk plan MUST list unconditional baseline: `cargo fmt --check`, `cargo clippy`, `cargo nextest run`, `cargo xtask capability-drift`, + webview gates only if UI touched. **THIS CHUNK APPLIES** (baseline unconditional; webview gates excluded—no UI changes).
