---
paths:
  - "crates/**/src/**/*.rs"
  - "pulse-app/**/*.rs"
  - "xtask/**/*.rs"
  - "tests/**/*.rs"
  - "pulse-app/ui/**/*.{ts,tsx}"
---

# Testing Rules

Path-scoped rules for Rust source + colocated test modules + webview frontend tests. Loaded only when Claude is working with files matching the `paths:` frontmatter above.

**Authoritative source:** `.andromeda/test-plan.md` §3 (Test Harness Contract) + §6 (E2E P1–P7) + §10 (Quality Gates) + §11 (Anti-Patterns). Tier=Standard.

## Framework
- **Unit tests (Rust):** `cargo test` (libtest, rustc 1.85+) + `cargo-nextest` 0.9.x for per-process isolation (port-binding tests need this).
- **Unit tests (webview, since chunk #11):** `vitest` 3.x with `jsdom` 26 environment + `@testing-library/react` 16 for DOM-shape assertions on React 19 components. Co-located `*.test.tsx` adjacent to source under `pulse-app/ui/src/`. Emits JUnit XML to `target/junit-ui.xml` (matches `cargo-nextest --message-format junit` shape) for agent-driven discipline. Coverage gate scope: presentational components (e.g., `pulse-app/ui/src/components/icons/`) explicitly EXCLUDED at Foundation pre-shell stage; integration coverage applies via `tauri-driver` E2E from chunk #25 (webview shell). `vitest.config.mjs` lives at `pulse-app/ui/vitest.config.mjs` with `setupFiles: ["./src/test-setup.ts"]` registering `@testing-library/react` `afterEach(cleanup)`.
- **Integration tests:** `cargo nextest run --filter-expr 'integration'` with `#[tokio::test(flavor = "multi_thread")]` for concurrent OTLP ingest tests.
- **E2E desktop-webview:** `tauri-driver` 2.x + WebdriverIO 9.x via `mocha` (headless `xvfb-run` on Linux; native macOS/Windows). Canvas pixel inspection is OUT of agent-driven scope (assert IPC contract instead).
- **E2E TauRPC IPC:** `tauri::test::mock_builder()` + `get_ipc_response()` (in-process, fast).
- **E2E OTLP gRPC:** `tonic` 0.14.5 native client → loopback `:4317`. **OTLP HTTP:** `reqwest` 0.12.x async + `axum-test` 18.7.0 → loopback `:4318`. **MCP sidecar:** `tokio::process::Command` + JSON-RPC 2.0 `serde_json` framing.

## File placement
- **Rust:** co-located `#[cfg(test)] mod tests { … }` within each crate's `src/` (no separate `tests/` directory per arch convention).
- **Webview:** co-located `*.test.tsx` adjacent to source under `pulse-app/ui/src/`. Vitest discovers via `include: ["src/**/*.{test,spec}.{ts,tsx}"]` per `pulse-app/ui/vitest.config.mjs`.
- Test fixtures (pre-compiled WASM Component Model binaries): `tests/fixtures/plugins/` (committed to repo, shared read-only per suite).
- Property-based regression files: `proptest-regressions/` (committed for replay).

## Patterns + agent-driven discipline
- Every test MUST exit with a deterministic signal (exit code 0/non-zero, structured JSON, log-line match) — no human-in-the-loop, no visual inspection, no Percy/Chromatic.
- Self-bootstrapping fixtures: tests seed their own data via OTLP ingest through the live receiver — NO pre-baked DuckDB snapshots, NO `.sql` scripts.
- Synthetic data generators: `MockTraceSpan::builder()` (`ingest` crate), `MockArrowBatch::builder()` (`buffer`), `MockMetricPoint::builder()` (`viz`).
- Per-test isolation: `tempfile::TempDir` + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/test-$$` + ephemeral in-memory DuckDB (`duckdb::open_in_memory()`).
- Deterministic time: `tokio::time::pause()` + `tokio::time::advance(Duration)` for retention-window tests; NEVER `std::time::Instant::now()` / `chrono::Utc::now()` without injection.
- AAA pattern visible per test; one assertion concept per test.

## Mocking discipline
- NEVER mock the OTLP receiver under test — use live `tonic` client / `reqwest` against loopback.
- NEVER mock DuckDB at integration level — use ephemeral `duckdb::open_in_memory()` per test.
- NEVER mock `tokio` / `tonic` / framework internals — mock only traits you own (`mockall` 0.13.x with `#[automock]`).
- Time mocking: built-in `tokio::time::pause()` only.
- HTTP mock for `tauri-plugin-updater` `latest.json`: `httpmock` 0.7.x (fallback `mockito` 1.6.x); valid + invalid Minisign signatures.
- Property-based: `proptest` 1.10.0 with seed control.

## Critical paths (E2E P1–P7) — must each pass
- **P1** Receive OTLP gRPC → buffer → `traces.query` returns rows.
- **P2** `snapshot.generate({token_budget: 25000})` → markdown with anomaly markers + `token_count <= 25000` + `dedup_count > 0` + p50/p95/p99 aggregates.
- **P3** MCP `tools/call` with `query_traces` returns array of trace objects.
- **P4** Plugin lifecycle: load WASM → `plugins.list` → `plugins.invoke({capability})` → negative test on disallowed capability.
- **P5** Widget ↔ dashboard ↔ tray (IPC contract surrogate at agent level; full window/tray UI deferred to tauri-driver headful suite).
- **P6** Subscribe to `pulse://stream/spans` → ingest gRPC trace → receive Arrow IPC payload → schema match.
- **P7** `workspace.detect()` returns `{project_name, root_path, vcs_type: "git", vcs_root}`.

## Quality gates (Standard tier)
- Coverage: ≥75% line, ≥70% branch, ≥85% function via `cargo-llvm-cov` 0.8.5 (LLVM source-based; cross-platform on all 3 CI runners). Exclude generated code (`prost` stubs, `taurpc` IPC bindings) and test fixtures.
- Performance budgets (p99): OTLP gRPC Export <100ms, OTLP HTTP POST <120ms, TauRPC `traces.query` <150ms, Snapshot 25k budget <500ms, DuckDB Arrow appender <50ms, WebGPU 10k spans/sec sustained ≥30 fps.
- **Zero-flakiness budget:** flaky tests are NOT tolerated. Quarantine immediately via `#[ignore]` + open issue; root-cause + fix or delete before unquarantining. NO retry-once policies.
- `cargo deny check bans` blocks `multiple-versions = "deny"` regressions (catches `tonic` 0.14/0.13 duplicate).

## Synchronization
- NEVER `sleep(N)` for sync — wait for explicit signal: poll `health` TauRPC (`subsystems.buffer.rows_ingested >= N`), Channel event subscription, or PID-file existence check.
- NEVER share mutable state across parallel tests — each test gets `TempDir` + in-memory DuckDB.

## Project-specific bans (andromeda-pulse)
- NEVER allow OTLP self-dialing — negative test asserts product cannot be configured to dial own `:4317`/`:4318`.
- NEVER assert "raw OTLP JSON dump" passes as snapshot — must show dedup count + anomaly markers + p50/p95/p99 aggregates.
- NEVER skip post-`prost` invariant negative tests — span_id != 8 bytes / trace_id != 16 bytes must reject without panic.
- NEVER assert receiver bound to non-loopback succeeds — security model invariant.

## Running tests
- **Single crate:** `cargo nextest run --filter-expr 'package(ingest)'`
- **All workspace:** `cargo nextest run --workspace --profile ci --message-format libtest-json`
- **With coverage:** `cargo llvm-cov nextest --workspace --lcov --output-path lcov.info --summary-only`
- **xtask shortcut:** `cargo xtask test`
- **CI matrix:** Linux/macOS/Windows × Rust stable; tauri-driver matrix per platform for E2E desktop-webview.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._

- 2026-05-03: `#[tokio::test(start_paused = true)]` requires the `test-util` feature on the `tokio` workspace dep — the workspace currently pins `features = ["full"]`, which does NOT include `test-util` (per tokio docs the `full` feature unifies fs/io/macros/net/process/rt/sync/time/tracing but excludes test-util). Compile fails with `no method named 'start_paused' found for struct 'tokio::runtime::Builder'` — misleading because the missing capability is a feature gate, not a missing method. Two paths: (a) drop `start_paused = true` from tests that don't actually need paused clock (smoke tests / spawn-and-abort tests run fine on real time within milliseconds); (b) add `tokio = { workspace = true, features = ["test-util"] }` to the affected crate's `[dev-dependencies]` to unify in test builds without bloating production. Default to (a) when the test doesn't assert on tick cadence — keeps dev-dep surface minimal. Use (b) only when the test genuinely needs `tokio::time::pause()` + `tokio::time::advance(Duration)` semantics (e.g., retention-window tests; multi-interval timing assertions).

- 2026-05-04: motion v12's `useReducedMotion` hook from `motion/react` (verified 12.38.0) is intentionally NON-reactive after mount: the hook captures `prefersReducedMotion.current` via `useState(...)` on mount and never re-renders on subsequent media-query change (per framer-motion source TODO comment in `node_modules/framer-motion/dist/es/utils/reduced-motion/use-reduced-motion.mjs`). The matchMedia listener is single-global, registered ONCE per process via motion-dom's `initPrefersReducedMotion()` lazy-init on first hook call; subsequent hook mounts read the cached ref. Test consequence for Vitest specs consuming `useReducedMotion`: MUST reset motion-dom's module-global state between tests via `import { hasReducedMotionListener, prefersReducedMotion } from "motion-dom"; hasReducedMotionListener.current = false; prefersReducedMotion.current = null;` in `beforeEach`. Without the reset, the FIRST test's `mockReducedMotion(value)` is captured globally and subsequent tests with different values get the stale prior state. Per-instance `addEventListener` / `removeEventListener` spies are NOT a valid contract — assert global subscription pattern instead (matchMedia called once + addEventListener called once globally on first hook mount). The older "reactive on every change" framer-motion behavior is NOT preserved in motion v12. See `pulse-app/ui/src/hooks/use-reduced-motion.test.tsx` for canonical pattern.

- 2026-05-05: When integration tests boot a server that takes a `tokio::sync::mpsc::Sender` (or wrapper around one), the test MUST hold the matching `Receiver` alive for the test duration — either by returning it from the helper tuple OR by spawning a placeholder consumer task `tokio::spawn(async move { while receiver.recv().await.is_some() {} })` that drains messages. Dropping the receiver before the test completes causes `Sender::try_send` to return `TrySendError::Closed` immediately on subsequent calls, and the receiver-side endpoints fail with the channel-saturated error path (HTTP 503 / gRPC `ResourceExhausted`) instead of the expected success path. The chunk #18 helpers `crates/ingest/tests/{grpc_loopback,http_loopback}.rs::start_test_*_server` show both patterns: `grpc_loopback.rs` uses the spawn-drainer pattern (no receiver returned); `tests/invariants.rs::start_test_grpc_server` uses the return-tuple pattern (receiver returned to the test for explicit `rx.recv().await` assertions verifying the batch landed in the channel). Choose return-tuple when the test asserts on channel content; choose spawn-drainer when the test only cares about the response code. Both are valid; ad-hoc ignoring the receiver is not.

- 2026-05-06: `#[tokio::test(start_paused = true)]` is INCOMPATIBLE with `flavor = "multi_thread"` — only `flavor = "current_thread"` (the default) supports paused clock semantics. Compile error is explicit: `error: The 'start_paused' option requires the 'current_thread' runtime flavor. Use #[tokio::test(flavor = "current_thread")]`. This rules out the common chunk #20 pattern `#[tokio::test(flavor = "multi_thread", worker_threads = 4)]` for any test that also needs `tokio::time::pause()` clock injection. Resolution: drop the `multi_thread` flavor + `worker_threads` for paused-clock tests (chunk #21 retention.rs initial design hit this); use `flavor = "current_thread"` and structure the test to NOT depend on multi-threaded scheduling. If a test genuinely needs both (rare — usually means the test is asserting too much about runtime internals), refactor to extract a sync/async helper from the periodic loop body and test the helper directly without paused-clock orchestration. See `crates/buffer/src/retention.rs::run_one_sweep` extraction for the canonical "extract testable async helper" pattern that sidestepped the start_paused × multi_thread × spawn_blocking × abort race entirely.
