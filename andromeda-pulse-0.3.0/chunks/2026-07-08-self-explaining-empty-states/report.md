# Report — 2026-07-08-self-explaining-empty-states

**Chunk:** Self-explaining empty states — Metrics/Logs empty surfaces explain themselves with an actionable hint + design-system iconography (P-071)
**Date:** 2026-07-08
**Commits:** none since last_wrap (chunk commit is P7 of this wrap)

## Changes (structured — detectors read this)
- **Files:** 3 new — `pulse-app/ui/src/components/EmptyState.tsx`, `pulse-app/ui/src/components/EmptyState.test.tsx`, `pulse-app/ui/tests-a11y/axe/p13-empty-states.spec.ts`; 5 modified — `pulse-app/ui/src/dashboard/routes/{MetricsRoute,LogsRoute,SnapshotsRoute}.tsx` + `{MetricsRoute,LogsRoute}.test.tsx`; ledger — `andromeda-pulse-0.3.0/verification-matrix.json` (P-071). **Zero `.rs` delta.**
- **Symbols / APIs:** new exported webview component `EmptyState({ message, hint?, glyph?, testId? })`. NO new TauRPC procedure / IPC method / endpoint / port / socket / env var / cross-bridge export. No `pulse://` topic. Webview stays under `pulse:default`.
- **Crates / modules:** none added / removed / changed (webview-only; no Rust crate touched).
- **Dependencies:** none added / bumped.
- **Schema / config:** none (no migration, no config key, no violation-schema change).
- **Coverage of new surfaces:**
  - `Metrics empty state (/metrics, no data)` → validation n/a · instrumentation n/a (no telemetry emitted; obs no-sink) · PII redacted✓ (static first-party copy; no OTLP/telemetry/service-name/AppError interpolation) · tests unit(vitest MetricsRoute)✓ + a11y(p13 axe + pa11y)✓ · a11y WCAG✓ (SC 1.4.3 `--color-text-secondary`; decorative glyph SC 1.1.1; plain text SC 4.1.3) · tokens design-token✓ (`--color-text-secondary`/`--font-body`/`--font-code`/`<Icon>`)
  - `Logs empty state (/logs, no data)` → same profile as Metrics; branch on PRE-filter rows so LogTable's "No logs match filter" still covers the filter-empty-with-data case
  - `Error state (Metrics/Logs query failure)` → static "Couldn't load {metrics|logs}", NO exporter hint, NO raw AppError · PII redacted✓ · tests unit✓ · a11y✓ · tokens✓
  - `EmptyState shared component` → tests unit(7 DOM-shape)✓ · a11y✓ (decorative aria-hidden glyph; no role=status; no focusable) · tokens✓
  - `Snapshots empty state (adopted shared component)` → contrast fixed `--color-text-tertiary`→`--color-text-secondary` (SC 1.4.3); existing `SnapshotsRoute.test.tsx` green unchanged · a11y✓ · tokens✓

## Deviations from intent
- **`v02-fixtures.ts` not modified** — plan listed it conditionally ("if needed"); the default IPC mock (`tests-a11y/helpers/mock-tauri.ts`) already resolves `metrics.query`/`logs.query` empty, so the empty state renders under the axe sweep with no override. Justified: resolved conditional.
- **`SnapshotsRoute.test.tsx` not modified** — plan said "verify stays green (no rewrite expected)"; it stays green (the shared `EmptyState` preserves `data-testid="route-empty-state"` + renders the message). Justified.
- **Rust workspace gates deferred** — `cargo fmt --check` / `clippy --workspace --all-targets --all-features` / `nextest --workspace --profile ci` / `xtask capability-drift` + the self-verify BOOT half deferred per the source-delta-proportional rule (zero `.rs` delta, no TauRPC surface → bindings.ts/capability-drift unaffected). The P3 warm-reembed `cargo build -p pulse-app` compiled the binary clean (de-facto compile check). Re-run at the next `.rs`-touching chunk.
- **Smoke via warm re-embed, not `agent-run.sh`/`cargo xtask self-verify`** — per the operator `boot-smoke-webview-warm-reembed` directive + `.claude/rules/frontend.md` 2026-07-05: for a webview-only chunk those boot a STALE-embedded binary (old frontend); ran `npm build → cargo build -p pulse-app → launch → obs-log + operator visual` instead. self-verify's frontend-meaningful (a11y) half ran in P2 as `npm run test:a11y`.
- NOTE (not a deviation): the two scope expansions — Snapshots adoption + the 4th honest-error state — were folded into `scope.md` at /andromeda-phase P5 as user-approved intent-incomplete amendments, so they are the approved plan, not deviations.

## Decisions & corrections
- **P4 user decisions (AskUserQuestion):** (1) "promote & adopt" — one shared `EmptyState` promoted from SnapshotsRoute's existing local copy, adopted by Metrics/Logs/Snapshots; (2) "minimal honest error line" — a distinct static error message, empty branch gated on `error === null`.
- **Default a11y IPC mock already returns empty metrics/logs** (`mock-tauri.ts` `"metrics.query"`/`"logs.query"` = `{items:[],total:0}`), so a new empty-state axe audit needs no fixture override — the settled empty state renders on a bare `installTauriIpcMock(page)`.
- **Promoting a local component to shared fixed a latent a11y bug** — SnapshotsRoute's local EmptyState used `--color-text-tertiary` (< 4.5:1 at 14px); the a11y extract's chunk-#99 `LogTable/LogFilter` tertiary→secondary precedent flagged it; the promotion moved it to `--color-text-secondary` (6.8:1). No axe spec had audited Metrics/Logs/Snapshots before this chunk.
- **Four-state honesty branch order** — check `error` FIRST (both `useMetrics`/`useLogs` set `rows:[]` on rejection), so the "point an exporter at :4318/:4317" hint is gated to the genuine no-data case and never masquerades over a query failure. Extends the 2026-07-05/07 "state honesty & legibility" line.
- **Warm re-embed boot for a webview-only chunk** — `npm run build` then `cargo build -p pulse-app` re-embeds the fresh `ui/dist` (`index-C7UBbzM2.js` → `index-ZqKtV7Or.js` confirmed via `strings`), because Tauri compile-time-embeds the frontend; the warm binary/self-verify otherwise show the old bundle.

## Outcome
- **Met P-071 acceptance criteria** — settled zero-data Metrics AND Logs render message + hint (naming `:4318`/`:4317`) + Observatory glyph; not flashed while loading; hidden when populated; distinct error state without the hint; a11y `--color-text-secondary` + decorative glyph + plain text (affordance-EXEMPT pure-visual cap).
- **Gates green:** `npm run typecheck` ✓ · `npm run lint` ✓ · `npm run test` (vitest **732 passed**, +15) ✓ · `npm run test:a11y` (contrast 12/12 · axe **34** incl. p13 metrics+logs · lighthouse 7/7 ≥90 · pa11y **7/7** incl. /metrics+/logs · regression-detector **0/0 new**) ✓. Deferred (zero `.rs`): cargo fmt/clippy/nextest --workspace/capability-drift + self-verify boot half.
- **Smoke (warm re-embed boot) PASSED:** 0 `app.panic.fatal` / 0 obs-log ERROR · `app.boot.webview.init` · 15,640 `metric.webgpu.frame_duration_ms` · all subsystem ticks; **operator visual verify of the Metrics/Logs empty states PASSED**; clean **exit 0, ports freed, zero orphan** (the prior chunk's SIGILL-132 teardown flake did not recur; the lone `Failed to unregister class Chrome_WidgetWin_0` line is benign WebView2 C++ teardown noise, not an obs-log ERROR).
