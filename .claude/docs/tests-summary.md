# Tests Summary

_Distilled from `.andromeda/test-plan.md` by `/setup-project`. Read on demand. **Tier: Standard (1).**_

## Why Standard tier
Cross-platform desktop app (Windows/macOS/Linux via Tauri 2) with 19 testable entities, 7 critical paths, 28+ security anti-patterns requiring negative tests, and "portfolio-worthy GPU-accelerated visualization" risk tolerance. Agent-driven discipline requires every test layer to be machine-parseable end-to-end (no human-in-the-loop, no Percy/Chromatic).

## Test harness contract (5-command discipline)

| Command | Body |
|---|---|
| `boot` | `cargo run --bin pulse-app --release` + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/test-$$` + `RUST_LOG=debug`; poll TauRPC `health` every 500ms up to 10s; assert `status == "ok"` and subsystems initialized |
| `run` | `cargo nextest run --workspace --profile ci --message-format libtest-json` |
| `status` | TauRPC `health` via `tauri::test::mock_builder()` |
| `cleanup` | `kill -TERM $(cat $PID_FILE)` + 5s wait + verify ports `:4317`/`:4318` released |
| `logs` | `cat ~/.andromeda-pulse/logs/agent-latest.jsonl` (or `$ANDROMEDA_PULSE_DATA_DIR` fallback) |

## Status endpoint shape (binding)
```json
{
  "status": "ok" | "degraded" | "unhealthy",
  "subsystems": {
    "otlp_grpc_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "otlp_http_receiver": { "status": "initialized" | "error", "error_msg": null | "string" },
    "buffer": { "status": "ready" | "error", "rows_ingested": N, "retention_seconds": N },
    "ingest_channel": { "status": "ready" | "error", "broadcast_subscribers": N }
  },
  "uptime_ms": N,
  "pid": N
}
```

## PID file
`$XDG_RUNTIME_DIR/andromeda-pulse.pid` (Linux) / `$TMPDIR/andromeda-pulse.pid` (macOS) / `%LOCALAPPDATA%\andromeda-pulse\pid` (Windows). Fallback: `~/.andromeda-pulse/run/andromeda-pulse.pid`. Single decimal PID per line.

## Log format (binding contract)
JSON-per-line; fields: `timestamp` (ISO-8601), `level`, `target`, `message`, `fields`. Optional: `trace_id`, `span_id`, `duration_ms`, `span_count`, `service`. Source: `tracing-subscriber::fmt::Layer::json()` per obs §3.

## Critical paths (P1–P7 E2E)

| Path | Surfaces | Verification |
|---|---|---|
| **P1** | OTLP/gRPC + ingest + buffer + viz + TauRPC + webview | Send 100 gRPC spans → `health.subsystems.buffer.rows_ingested >= 100` → `traces.query` returns matching rows |
| **P2** | buffer + snapshot + TauRPC + Channel `pulse://stream/snapshot-progress` | Populate 500 spans → `snapshot.generate({token_budget: 25000})` → assert anomaly markers + `token_count <= 25000` + `dedup_count > 0` + p50/p95/p99 aggregates |
| **P3** | MCP stdio sidecar + buffer | Spawn with `--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true` → JSON-RPC `tools/call query_traces` → assert array of trace objects |
| **P4** | plugins + WASM Component Model + TauRPC | Stage fixture WASM → `plugins.reload` → `plugins.list` → `plugins.invoke({capability})` → negative test on disallowed capability |
| **P5** | webview + tray + IPC | IPC contract surrogate (window/tray UI deferred to tauri-driver headful suite) |
| **P6** | buffer + Tauri Channel + Arrow IPC | Subscribe to `pulse://stream/spans` → ingest gRPC → decode Arrow IPC → schema match |
| **P7** | workspace-detector + TauRPC | `workspace.detect` → assert `{project_name, root_path, vcs_type: "git", vcs_root}` |

## Test pyramid (Standard tier)
| Level | Coverage | Tools |
|---|---|---|
| Unit | ≥75% line / ≥70% branch / ≥85% function | `cargo test` + `cargo-nextest` 0.9 |
| Integration | All Standard Contracts + boundary types | `tauri::test::mock_builder()` + `tonic` 0.14.5 + `axum-test` 18.7 + `duckdb-rs` 1.5 |
| E2E | All 7 critical paths | `tauri-driver` 2.x + `WebdriverIO` 9.x + `mocha` (headless `xvfb-run` Linux; native macOS/Windows) |
| Property | Selective per trigger | `proptest` 1.10 |
| Performance / Load | Trigger-driven | Custom load drivers; `criterion` 0.5 in `xtask benches/` |
| Chaos / Fault | Trigger-driven | `tokio::time::pause()` + manual broadcast disconnect |

## Self-bootstrapping fixtures
- NO pre-baked DuckDB snapshots, NO `.sql` scripts.
- OTLP ingest via `tonic` client (fixture seeding through the live receiver path).
- Builder factories: `MockTraceSpan::builder()` (`ingest`), `MockArrowBatch::builder()` (`buffer`), `MockMetricPoint::builder()` (`viz`).
- Fixture composition: `rstest` 0.26.1 with `#[fixture]`.
- Per-test isolation: `tempfile::TempDir` + `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/test-$$`.
- WASM plugin fixtures: pre-compiled minimal Component Model `.wasm` binaries in `tests/fixtures/plugins/` (committed read-only).

## Quality gates (CI)
- Coverage ≥75% line / ≥70% branch / ≥85% function via `cargo-llvm-cov` 0.8.5.
- **Zero-flakiness budget** — flake = real bug; quarantine + fix or delete (NO retry-once policy).
- Performance budgets (p99): OTLP gRPC <100ms, OTLP HTTP <120ms, TauRPC `traces.query` <150ms, snapshot 25k budget <500ms, DuckDB Arrow appender <50ms, WebGPU 10k spans/sec ≥30 fps sustained.
- `cargo deny check bans` blocks `multiple-versions = "deny"` regression (catches `tonic` 0.14/0.13 duplicate).
- Cranelift-only WASM enforcement: build-time check on `wasmtime` Cargo.lock.

## CI integration
GitHub Actions matrix (Linux/macOS/Windows × Rust stable):
- Lint → Unit tests → Integration → E2E (matrix per platform) → Coverage → Quality gates.
- JUnit XML output via `cargo nextest --message-format junit`; `dorny/test-reporter` for inline PR annotations.
- Build fails on: any test failure, coverage below threshold, flaky test, perf regression, lint/typecheck/`cargo deny check` failure.

## Top anti-patterns (test-plan §11)
- NEVER `sleep(N)` for sync — wait for `health` polling or Channel events.
- NEVER mock OTLP receiver under test — use live `tonic`/`reqwest` to loopback.
- NEVER mock DuckDB at integration — use ephemeral `duckdb::open_in_memory()`.
- NEVER pre-populate test DB via `.sql` scripts — self-bootstrapping fixtures only.
- NEVER use `std::time::Instant::now()` / `chrono::Utc::now()` without injection — `tokio::time::pause()` for determinism.
- NEVER tolerate flake — quarantine and fix immediately.
- NEVER ship release with known test failures.
- NEVER include "human verifies canvas" steps — pixel inspection out of agent-driven scope.
- NEVER allow OTLP self-dialing — negative test must assert prevention.
- NEVER dump raw OTLP JSON as snapshot — assertion requires dedup count + anomaly markers + p50/p95/p99 aggregates.
- NEVER skip post-`prost` invariant negative tests.
- NEVER assert receiver bound to `0.0.0.0` succeeds.

Full plan: `.andromeda/test-plan.md`.
