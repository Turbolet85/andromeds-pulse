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
- **Unit tests:** `cargo test` (libtest, rustc 1.85+) + `cargo-nextest` 0.9.x for per-process isolation (port-binding tests need this).
- **Integration tests:** `cargo nextest run --filter-expr 'integration'` with `#[tokio::test(flavor = "multi_thread")]` for concurrent OTLP ingest tests.
- **E2E desktop-webview:** `tauri-driver` 2.x + WebdriverIO 9.x via `mocha` (headless `xvfb-run` on Linux; native macOS/Windows). Canvas pixel inspection is OUT of agent-driven scope (assert IPC contract instead).
- **E2E TauRPC IPC:** `tauri::test::mock_builder()` + `get_ipc_response()` (in-process, fast).
- **E2E OTLP gRPC:** `tonic` 0.14.5 native client → loopback `:4317`. **OTLP HTTP:** `reqwest` 0.12.x async + `axum-test` 18.7.0 → loopback `:4318`. **MCP sidecar:** `tokio::process::Command` + JSON-RPC 2.0 `serde_json` framing.

## File placement
- Co-located `#[cfg(test)] mod tests { … }` within each crate's `src/` (no separate `tests/` directory per arch convention).
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
