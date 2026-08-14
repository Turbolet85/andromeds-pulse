# tests extract

## Relevance
Relevant — webview-primary chunk adds a testable UI surface (status line) plus a time-dependent recency regression (ConnectionDot CARRY); tests domain applies via vitest DOM-shape tests + gate discipline, with a conditional Rust/harness dimension iff a backend delta lands.

## Constraints
- **Standard tier breadth** (per test-plan §1): the new status line + ConnectionDot change get DOM-shape unit tests; no new E2E critical path is owed — the widget/window P5 headful path stays deferred (per test-plan §6 P5 "Current residual").
- **Webview unit framework is fixed** (per test-plan §4): `vitest` 3.x + `jsdom` 26 + `@testing-library/react` 16, DOM-shape assertions only (no visual diff), co-located `*.test.tsx`, emitting JUnit XML to `target/junit-ui.xml`; config at `pulse-app/ui/vitest.config.mjs`.
- **Standard gate set is mandatory in `## Test Commands`** (per test-plan §3 "Per-chunk gate discipline"): webview gates (`npm run lint`/`typecheck`/`test --prefix pulse-app/ui`) are required because `pulse-app/ui/**` is touched.
- **Boot-smoke gate is conditional** (per test-plan §3 "Boot-smoke gate"): required only if a backend delta touches `crates/ui-bridge/src/`, `pulse-app/src/main.rs`, `tauri.conf.json`, or `capabilities/*.json` — i.e. if the reuse-first path extends a ui-bridge API impl rather than staying webview-only.
- **Deterministic time is mandatory** (per test-plan §11 Universal "NEVER use real time"; §8 time mocking): the "just now" vs "no spans yet" recency branch must be driven by an injected/controlled timestamp, never `chrono::Utc::now()`/`Instant::now()`.
- **Self-bootstrapping, factory fixtures** (per test-plan §7; §11 Test Data): no pre-baked data, no raw object literals; any Rust-side seeding uses `MockTraceSpan::builder()` via live OTLP ingest.
- **Zero-flakiness budget** (per test-plan §10): a recency/rate test that flakes is treated as a real bug (quarantine + root-cause), no retry-once.

## Patterns to follow
- **Co-located `*.test.tsx` + DOM-shape assertions** (per test-plan §4; §1 webview-unit row): assert rendered text and aria/attribute shape for both populated and zero-telemetry branches of the status line, and the tooltip-text branch for ConnectionDot — model on the existing `pulse-app/ui/src/components/icons/Icon.test.tsx` pattern.
- **Controlled-`now` recency test** (per test-plan §8 time mocking; §11 Universal): feed a fixed last-span timestamp vs. "no span ever" and assert "just now" appears only when a span exists — the CARRY regression.
- **health/ready IPC contract shape as the data-source assertion** (per test-plan §5 cross-module patterns): `ready` → `ingest_mpsc_capacity_pct` + `duckdb_connection`; `health` → `subsystems.buffer.{rows_ingested, retention_seconds}`, `ingest_channel.broadcast_subscribers`. If the line reads buffer-fill/rate from these, assert against these declared field names.
- **rstest + builder factories for any Rust delta** (per test-plan §4 fixture pattern; §7): `#[fixture]` composition over `MockTraceSpan::builder()`, not literals.

## Anti-patterns to avoid
- **No visual regression / "human reviews canvas"** (per test-plan §11 E2E + Universal; no Percy/Chromatic): status-line wording and tooltip honesty are asserted via DOM text, never screenshots.
- **No real clock in recency logic** (per test-plan §11 Universal): un-injected time makes the "just now" branch flaky/untestable.
- **No pre-baked DB / raw-literal fixtures** (per test-plan §11 Test Data; §7): fixture data via runtime ingest or builders only.

## Contract bindings
- **tests ↔ obs (status-endpoint shape):** if the status line sources buffer-fill/ingest-rate, assertions bind to the obs-declared `health`/`ready` fields (per test-plan §3 "Status endpoint shape": `subsystems.buffer.{rows_ingested,retention_seconds}`, `ingest_channel.broadcast_subscribers`; §5 `ready.ingest_mpsc_capacity_pct`). Any new backend field must also land in the obs schema, and a derived spans/s should bind to an obs `metric.*` name rather than an ad-hoc counter.
- **tests ↔ arch/route (capability matrix):** P-070 must carry a verification-matrix entry whose id/file-ref/grep-anchor resolves; `cargo xtask verify:capability-matrix` (and `capability-drift` in the standard gate) fail CI on dangling anchors (per test-plan §9; amendment 2026-06-10).

## Acceptance criteria contributions
- (tests) `npm run test --prefix pulse-app/ui` passes for new status-line + ConnectionDot vitest DOM-shape tests (per test-plan §4).
- (tests) A DOM-shape test asserts the honest zero-telemetry branch: with zero live spans the line renders explicit empty phrasing (e.g. "No telemetry yet…") AND the ConnectionDot tooltip does NOT read "just now" — the CARRY regression (per test-plan §4; §11 Universal time-injection).
- (tests) `## Test Commands` carries the full standard gate set incl. webview gates; boot-smoke gate added iff a `crates/ui-bridge`/`tauri.conf`/`capabilities` delta lands (per test-plan §3).
- (tests) P-070's `andromeda-pulse-0.3.0/verification-matrix.json` entry resolves so `verify:capability-matrix` passes; render-only → mode likely automated-nextest/a11y or by-construction (per test-plan §9; amendment 2026-06-10).

## Relevant amendment history
- **2026-05-10 — standard gate baseline (`chunk-gate-baseline-coverage`):** every chunk plan MUST list the full gate set incl. webview gates when `pulse-app/ui` is touched; empirically motivated by chunk #36/#37 omitting `cargo fmt --check`/`tsc` and surfacing pre-existing failures N chunks later. Directly governs this chunk's `## Test Commands`.
- **2026-05-09 — boot-smoke gate for boot-path chunks (`boot-smoke-coverage`):** relevant only if the reuse-first research forces a `crates/ui-bridge/src/` (or main.rs/tauri.conf/capabilities) delta; motivated by a latent `ui-bridge/src/health.rs` Tokio-reactor boot panic the in-process harness missed.
- **2026-06-10 — chunk #99 tag gate (capability verification matrix):** established that P-061+ capabilities extend the matrix JSON in the same chunk that lands them and that `verify:capability-matrix` fails CI on dangling ids/paths/anchors — applies because this chunk lands P-070.
