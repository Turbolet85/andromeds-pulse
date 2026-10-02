# Report — 2026-07-07-plain-language-connection-status

**Chunk:** Plain-language connection status — worded connected-sources + spans/s + buffer-fill line (design-system typography) + honest ConnectionDot zero-span recency (P-070)
**Date:** 2026-07-08
**Commits:** (uncommitted — this wrap's commit is the first)

## Changes (structured — detectors read this)
- **Files:**
  - Backend: `crates/ui-bridge/src/contract.rs`, `crates/ui-bridge/src/health.rs`, `crates/buffer/src/state.rs`, `pulse-app/src/main.rs`
  - Frontend (new): `pulse-app/ui/src/hooks/use-ingest-stats.ts` (+ `.test.ts`), `pulse-app/ui/src/dashboard/ConnectionStatusLine.tsx` (+ `.test.tsx`)
  - Frontend (mod): `pulse-app/ui/src/dashboard/FooterStatusBar.tsx` (+ `.test.tsx`), `pulse-app/ui/src/components/ConnectionDot.tsx` (+ `.test.tsx`), `pulse-app/ui/tests-a11y/helpers/v02-fixtures.ts`, `pulse-app/ui/src/bindings/bindings.test.ts`
  - Generated: `pulse-app/ui/src/bindings/index.ts` (regenerated — mcp full-set + new ReadyChecks fields)
- **Symbols / APIs:**
  - `ReadyChecks` (ui-bridge contract) gains 3 fields: `rows_ingested: u64` · `buffer_used_seconds: u64` · `retention_seconds: u64`. These ride the EXISTING top-level `ready` TauRPC procedure — **NO new procedure / namespace / endpoint / port / env var / capability**.
  - `BufferState` + `BufferStateSnapshot` (buffer) gain set-once `first_append_at_nanos: u64` (via `std::time::SystemTime`).
  - `IntrospectionApiImpl::new` signature: +2 params (`buffer_state: Option<Arc<buffer::BufferState>>`, `retention_seconds: u64`) — internal binary-boundary wiring.
  - New webview: `useIngestStats()` hook + `ConnectionStatusLine` component.
- **Crates / modules:** no crate added/removed; changed — `buffer`, `ui-bridge`, `pulse-app` (bin), `pulse-app/ui`.
- **Dependencies:** NONE added / bumped. (Deliberately used `std::time::SystemTime` instead of adding `chrono` to `buffer` — chrono is a buffer dev-dep only.)
- **Schema / config:** none — no DB migration, no `config.toml` key, no new violation schema. (`ReadyChecks` wire-payload shape extended by 3 fields — a cross-bridge contract, not a persisted schema; no `#[serde(default)]` since `ready()` always fully constructs it.)
- **Coverage of new surfaces:**
  - `ready` envelope extension (ReadyChecks +3 fields) → validation n/a (read-only output, no input arg) · instrumentation: existing `ui-bridge.ready` span, no new fields ✓ · PII: aggregate-only (counts/seconds), no raw OTLP/query values — redacted✓ · tests: unit (`health.rs::introspection_tests` +2) + webview (`use-ingest-stats` +4) ✓ · a11y n/a · tokens n/a
  - `ConnectionStatusLine` (webview dashboard footer) → validation n/a · instrumentation n/a (webview; no direct obs, no telemetry.frontend bridge added) · PII: aggregate-only display (count / rounded rate / minutes / state words) — redacted✓ · tests: webview (`ConnectionStatusLine.test.tsx` +6) ✓ · a11y: SC 1.4.3 contrast (`--color-text-primary`) + SC 4.1.1 not-color-alone + SC 4.1.3 polite once-on-edge announce via the shared `StatusLiveRegion`; p1 axe **0 new violation tuple** ✓ · tokens: design-token✓ (`--font-code`/`--font-body`/`--color-text-primary`/`--spacing-*`, no hardcoded hex/px)
  - `ConnectionDot` CARRY (webview widget titlebar) → PII n/a · tests: webview (+3) ✓ · a11y: honest accessible-name "no spans yet" on the Listening zero-ingest sentinel (SC 4.1.2 / 1.1.1), in the `role="img"` aria-label ✓ · tokens ✓

## Deviations from intent
1. **`chrono` → `std::time::SystemTime`** for `BufferState.first_append_at_nanos` — research assumed `chrono` usable in `buffer` non-test code (grep saw it in `retention.rs`), but it is a dev-dep only. SystemTime is std-only, same unix-epoch nanos (comparable with health.rs's chrono `now`). One re-edit; zero new dep. (Plan Step 2 intent — an honest data-span anchor — preserved.)
2. **Two in-scope test-fixture consumers updated** (`bindings.test.ts` + the `contract.rs` `ready_envelope_serializes_with_required_fields` fixture) — both construct `ReadyChecks` and broke on the +3 fields; co-located consumers of the changed type (in-scope-by-extension, not scope creep).
3. **`FooterStatusBar.test.tsx`** gained hook mocks + a mounting assertion (co-located test of the modified `FooterStatusBar.tsx`; mocks keep the isolated render off the real TauRPC proxy in jsdom).
4. **`cargo xtask self-verify` substituted** by `npm run test:a11y` (a11y half, green) + the warm re-embed boot (boot/clean-quit half) — self-verify boots the release binary with the STALE embedded frontend (2026-07-05 boot-smoke directive); the warm re-embed is the operator-directed fresher equivalent covering the same a11y+boot+clean-quit ground.

## Decisions & corrections
- **Scope fork (P4, operator-selected):** backend-delta (extend `ReadyChecks`) over the webview-only-partial hybrid — spans/s + buffer fill were on NO IPC surface (raw data heartbeat-log-only in `BufferState`); the P-070 acceptance requires all three, so the full backend delta lands them + verifies P-070 this chunk. Reused the existing `ready` envelope → zero new procedure/capability/arch entry.
- **`chrono` is a `buffer` DEV-dep only** (used in `retention.rs` tests) — non-test buffer code must use `std::time::SystemTime`; it yields the same unix-epoch nanos as chrono's `timestamp_nanos_opt`, so the buffer-side `first_append` anchor stays comparable with the ui-bridge-side chrono `now`.
- **Honest `buffer_used_seconds`:** `min(now − first_append, retention_seconds)` — never an app-uptime proxy (uptime would over-claim data-span, the same dishonesty class as the CARRY). Set-once `first_append` on the buffer's first append; eviction pins the value to the retention window at the full-window limit.
- **Footer line is dashboard-only** — `ConnectionStatusLine` mounts in the dashboard shell (`router.tsx`), not the compact widget (which stays aggregate-glance per the layout boundary); the operator visual verify was done with the dashboard open.
- **Boot-smoke teardown caveat (not this chunk):** the first warm-boot run self-exited 132 (SIGILL) at teardown after ~68k obs lines — 0 `app.panic.fatal`, zero orphan (ports freed); a known WebView2/wgpu Windows teardown class. The operator-watched re-run clean-quit exit 0.

## Outcome
- **Acceptance MET (P-070):** the footer line renders connected-source count + spans/s + buffer fill in plain language — operator warm-boot VISUAL verify PASSED (*"Receiving from 5 services · ~54 spans/s · buffer 1 min / 10 min"*); connected count uses the P-067 `isServiceLive` shared gate; ConnectionDot reads honest "no spans yet" on the zero-ingest sentinel.
- **Gates green:** `npm run typecheck` · `vitest` 717 · `npm run lint` · `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -D warnings` · `cargo nextest run --workspace --profile ci` (1730 passed / 1 skip) · `cargo xtask capability-drift` (clean 0/0) · `npm run test:a11y` (axe 32/32 · lighthouse 7/7 ≥90 · pa11y 7/7 · regression 0/0 new).
- **Smoke:** warm re-embed boot — 0 panics/0 ERROR, `ui-bridge.ready` polled 258× live, `duckdb.append` 644, heartbeats 15/15/15, storm 31 / incident 37; operator visual PASSED; operator clean-quit exit 0.
- **Matrix:** P-070 → `status: implemented` (ref set; all named tests ran+passed).
