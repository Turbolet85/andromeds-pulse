# tests extract

## Relevance
relevant — P-063 (close behavior) and P-078 (self-verify harness) both depend heavily on boot-path code, capability declarations, and IPC surfaces; all are Standard-tier testable via the 5-command harness and agent-runnable assertions.

## Constraints
1. §3: Per-chunk standard gate baseline mandatory (cargo fmt, clippy, nextest, xtask capability-drift, npm lint/typecheck/test for webview touch)
2. §3: Boot-smoke gate mandatory (chunk touches `pulse-app/src/main.rs` + `pulse-app/capabilities/*.json`)
3. §1: Desktop-native tray surface verification via IPC liveness or platform-specific ATI (cannot fully automate cross-platform)
4. §1: Self-verify harness reuses existing a11y/contrast harness (a11y-plan §3 binding)
5. §3: 5-command harness contract—status endpoint must return health/subsystem JSON shape for window/tray state verification
6. §1: IPC procedures serialize errors to AppError only (never expose stack traces, file paths, struct names)
7. §2: Agent-runnable invariants—deterministic exit codes, no manual verification; window state verified via IPC + geometry JSON, not visual inspection

## Patterns to follow
1. §3 Test Harness Contract: TauRPC `health`/`ready` endpoints for boot readiness + subsystem state assertion (per §3 status-endpoint-shape)
2. §1 Surface: Desktop-webview / desktop-native cross-surface coordination (tray icon state tied to window visibility; notification delivery via IPC Channel or platform notification spy)
3. §3 Bootstrap: Extend existing 5-command-discipline-wire + status-endpoint-implement patterns for boot-smoke gate coverage
4. §1 P5: If "still running" signpost uses Tauri IPC Channel, subscribe + assert Arrow schema / row-count via existing Channel pattern

## Anti-patterns to avoid
1. §11: NEVER expose stack traces / file paths / Rust struct names in TauRPC error responses (AppError must sanitize; grepped via agent logs)
2. §3 Agent-driven discipline: No "did you see the tray?" or manual window inspection—all assertions machine-parseable (exit code / JSON / log line)
3. §1 Capability drift: Do NOT add TauRPC procedure without matching capability entry; xtask capability-drift enforces sync; the 3 NEVER-widen caps (`pulse:notification` / `pulse:tray` / `pulse:plugin-fs`) stay outbound-only

## Contract bindings
- **5-command harness** ↔ obs §3: boot/run/status/cleanup/logs; status returns JSON shape per §3 status-endpoint-shape; logs emit NDJSON `{"timestamp","level","target","message","fields"}`
- **Self-verify harness** ↔ a11y-plan §3: reuse existing a11y/contrast chain (Playwright / axe-core / colorjs.io; SC 2.3.3 reduced-motion)
- **Capability declarations** ↔ pulse-app/capabilities/*.json: xtask capability-drift verifies sync; `allow-close` pre-exists (P-061); signpost may request `pulse:notification` (no widening of outbound-only 3)
- **Window geometry + IPC** ↔ ui-bridge: P-061's `window-geometry.json` + `health` / `ready` responses supply assert surface for self-verify harness

## Acceptance criteria contributions
1. (tests) `cargo nextest run --workspace --profile ci` passes; `cargo xtask capability-drift` green (P-063's close behavior wired via existing `allow-close` capability; P-078 harness adds no new procedures without matching capability entries)
2. (tests) Boot-smoke gate (`npx @tauri-apps/cli dev` with 60s timeout, watch for `Local:` / `ready in` / `Compiled successfully`) completes without panic (catches bind-emission panics at non-Tokio boot context per §3 Boot-smoke gate)
3. (tests) Agent-headful self-verify harness (invoked via `scripts/agent-run.sh boot` / `status` / `cleanup` + a11y/contrast subprocess) exits deterministically (0 on pass, non-zero on failure); zero orphan process on cleanup (verified via `ps -p $PID` after SIGTERM)
4. (tests) Cross-surface consistency: TauRPC `health.subsystems.ingest_channel.broadcast_subscribers >= 1` if real-time push/notification channels active; tray state reflects window close behavior per honest signpost (no false "closed" claims while process running)

## Relevant amendment history
- **2026-05-09 — Boot-smoke gate (conditional)** — P-063/P-078 touch `pulse-app/src/main.rs` + `pulse-app/capabilities/*.json`, making boot-smoke MANDATORY. Per amendment: chunk #31 wrap caught bind-emission panic at `crates/ui-bridge/src/health.rs:291` latent across 4 wraps; in-process 5-command harness alone does not catch trait-level binding-emission panics at boot in non-Tokio context. Boot-smoke must be included in this chunk's test commands.
- **2026-05-10 — Standard gate baseline** — Every chunk MUST include full standard gate set unconditionally: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings` + `cargo nextest run --workspace --profile ci` + `cargo xtask capability-drift` + (if webview touched) `npm run lint/typecheck/test --prefix pulse-app/ui`. Verified empirically at chunk #37 wrap—omitted gates exposed pre-existing whitespace + TypeScript errors.