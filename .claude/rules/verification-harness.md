---
paths:
  - "scripts/agent-run.*"
  - "xtask/**/*.rs"
  - "tests/integration/**"
  - "tests/e2e/**"
---

# Verification Harness Rules

Path-scoped rules for the agent-driven verification harness — applies when Claude is working on `scripts/agent-run.{sh,ps1}`, the `xtask` task runner, or integration / E2E test infrastructure.

**Authoritative sources (binding contracts):**
- `.andromeda/test-plan.md` §3 — Test Harness Contract (5-command discipline, status endpoint shape, PID file location, log format)
- `.andromeda/obs-plan.md` §3 — Observability Harness Contract (`tracing` ecosystem only, JSON log sink, service identity, heartbeat ticks)

These two contracts are **byte-bound**: the test harness consumes the obs harness's structured log format and `health` endpoint shape. Edits to either side must keep both in sync.

## 5-command discipline
`scripts/agent-run.{sh,ps1}` MUST expose exactly 5 commands. Do not add a 6th without amending both binding contracts.

- **`boot`** — `cargo run --bin pulse-app --release` with env `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/agent-run-$$` + `RUST_LOG=debug`; poll TauRPC `health` every 500ms up to 10s; assert `status == "ok"` and `subsystems.{otlp_grpc_receiver,otlp_http_receiver,buffer,ingest_channel}.status == "initialized"|"ready"`; confirm TCP handshake on `:4317` and `:4318`; write PID to PID file. Exit 0 on ready, non-zero on timeout.
- **`run`** — `cargo nextest run --workspace --profile ci --message-format libtest-json` (or per-target `cargo nextest run --filter-expr 'package(ingest)'`); exit code is failure count.
- **`status`** — invoke TauRPC `health` via test client (`tauri::test::mock_builder()` + `get_ipc_response()`) OR direct subprocess if Unix socket exposed; parse JSON.
- **`cleanup`** — `kill -TERM $(cat $PID_FILE)`; wait 5s; verify ports `:4317`/`:4318` no longer accept TCP; SIGKILL escalation if needed; remove TempDir. Idempotent: safe to call twice.
- **`logs`** — `cat ~/.andromeda-pulse/logs/agent-latest.jsonl` (or `$ANDROMEDA_PULSE_DATA_DIR/logs/*.jsonl` fallback); supports `jq` for level filtering.

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
**Agent reads:** `status == "ok"`, all `subsystems.*.status` non-error, `subsystems.buffer.rows_ingested` increments after ingest, `subsystems.ingest_channel.broadcast_subscribers >= 1` if streams active, `pid` matches spawned process.

The status endpoint MUST NOT include sensitive data (no env vars, no secrets, no per-user data). It is exposed via `health` TauRPC command per arch §Standard Contracts.

## Structured log discipline (binding to obs §3)
- Log to file at `~/.andromeda-pulse/logs/agent-latest.jsonl` (or `$ANDROMEDA_PULSE_DATA_DIR/logs/agent-latest.jsonl` for harness override).
- One JSON object per line via `tracing_subscriber::fmt::Layer::json()`.
- Required fields per record: `timestamp` (ISO-8601), `level`, `target`, `message`, `fields.service.name`, `fields.service.version`, `fields.deployment.environment`.
- Harness greps the log file for assertions; format break = harness break.
- Agent-parseable signals: `jq 'select(.level == "ERROR") | .message' agent-latest.jsonl | head -20`; `grep 'TraceService.Export received' agent-latest.jsonl | tail -1`; `! grep 'query parameters:' agent-latest.jsonl` (assert no SQL params logged — Vector 5).

## PID file commitment
- **Location:** `$XDG_RUNTIME_DIR/andromeda-pulse.pid` (Linux) / `$TMPDIR/andromeda-pulse.pid` (macOS) / `%LOCALAPPDATA%\andromeda-pulse\pid` (Windows). Fallback: `~/.andromeda-pulse/run/andromeda-pulse.pid`.
- **Format:** single decimal PID on one line.
- **Lifecycle:** `boot` writes after `ready` confirms; `status` reads + verifies `ps -p $PID`; `cleanup` terminates via `kill -TERM $(cat $PID_FILE)`.

## Heartbeat verification (obs invariant)
- Daemon emits `{module}.tick` events at fixed intervals (15s for ingest/buffer/viz/plugins; 100ms for realtime throughput counter).
- Stall threshold: missing tick for >45s = harness flags failure.
- Harness verifies at least one tick per long-running subsystem present in logs after `boot`.
- CI step `xtask/ci/heartbeat-gap-check.sh` parses log timestamps per tick target, computes consecutive deltas, asserts max ≤45000ms or exits 1.

## Tempdir discipline
- Test runs use ephemeral tempdir: `$TMPDIR/agent-run-$$` (Unix) / `$env:TEMP\agent-run-$PID` (Windows).
- Tempdir created by `boot`, removed by `cleanup`. NEVER use the user's project directory or a fixed system path.
- Path resolution honors `ANDROMEDA_PULSE_DATA_DIR` env var for harness override.

## Synthetic event injection (no in-process bypass)
- Integration tests inject events via the same IPC contract as production clients per arch §Cross-cutting Patterns Test-time telemetry injection.
- OTLP via `tonic` 0.14.5 native client to `:4317` (gRPC) or `reqwest` 0.12.x to `:4318` (HTTP).
- TauRPC roundtrip via `tauri::test::mock_builder()` + `get_ipc_response()`.
- Channel subscription via Tauri 2 IPC Channel API; decode binary Arrow IPC via `arrow_ipc::StreamReader::try_new(bytes, None)`.
- MCP via `tokio::process::Command` + JSON-RPC 2.0 frames on stdin/stdout.
- NEVER pixel-scrape or rely on screen geometry — Tauri IPC procedure surface is the test surface.

## xtask drift checks (CI gate)
- `xtask capability-drift` — diff TauRPC procedures (introspected from `taurpc` derive) against `pulse-app/capabilities/` JSON; fail CI on mismatch.
- `xtask deny-bans` — wraps `cargo deny check bans licenses sources`; catches `tonic 0.14 ↔ tonic 0.13 (via opentelemetry-otlp 0.31)` duplicate.
- `xtask audit` — wraps `cargo audit` against RustSec advisory DB.
- `xtask test:a11y` — runs Playwright + axe-core + Lighthouse + pa11y; emits violation JSON + uploads as CI artifact.

## Cross-platform discipline
- `agent-run.sh` (POSIX shell, `set -euo pipefail`); `agent-run.ps1` (PowerShell, `$ErrorActionPreference = 'Stop'`).
- Both expose identical 5 commands with identical exit semantics.
- Path resolution differs per OS — both honor `ANDROMEDA_PULSE_DATA_DIR` consistently.

## Anti-patterns
- NEVER add a 6th command without specialist plan amendment.
- NEVER `sleep(N)` for synchronization — poll `health` status or Channel events.
- NEVER include sensitive data in `status` response.
- NEVER use real `setTimeout` / `chrono::Utc::now()` — `tokio::time::pause()` / injected clock for determinism.
- NEVER pre-populate test DB via `.sql` scripts — self-bootstrapping fixtures via OTLP ingest.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._

- 2026-05-03: cargo-nextest 0.9.x gates `--message-format libtest-json` behind the `NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1` env var. Any xtask wrapper that invokes nextest with libtest-json output (e.g., `cargo xtask test`) MUST set this env in the spawned `tokio::process::Command` before exec — otherwise nextest exits with "libtest JSON output is an experimental feature" error before any test discovery runs. CI workflow steps that invoke `cargo xtask test` inherit the env from xtask's spawn; no need to set at workflow / step level. See `xtask/src/main.rs` `run_cargo_nextest()` `cmd.env(...)` call.
