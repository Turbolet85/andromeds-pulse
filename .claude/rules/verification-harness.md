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
    "otlp_grpc_receiver": { "status": "initialized" | "error", "error_msg": null | "string", "last_tick_at": null | "ISO-8601" },
    "otlp_http_receiver": { "status": "initialized" | "error", "error_msg": null | "string", "last_tick_at": null | "ISO-8601" },
    "buffer": { "status": "ready" | "error", "rows_ingested": N, "retention_seconds": N, "last_tick_at": null | "ISO-8601" },
    "ingest_channel": { "status": "ready" | "error", "broadcast_subscribers": N, "last_tick_at": null | "ISO-8601" },
    "viz": { "status": "ready" | "error", "last_tick_at": null | "ISO-8601" },
    "plugins": { "status": "ready" | "error", "last_tick_at": null | "ISO-8601" }
  },
  "uptime_ms": N,
  "pid": N
}
```
**Agent reads:** `status == "ok"`, all `subsystems.*.status` non-error, `subsystems.buffer.rows_ingested` increments after ingest, `subsystems.ingest_channel.broadcast_subscribers >= 1` if streams active, `subsystems.{ingest_channel,buffer,viz,plugins}.last_tick_at` advances within ≤45s windows for stall detection per obs-plan §3 heartbeat ticks, `pid` matches spawned process.

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

- 2026-05-14: `cargo xtask ci-gates` is sensitive to historical daily-rolled tracing logs in the persistent dev data dir (`~/.andromeda-pulse/logs/` on macOS/Linux, `%APPDATA%\andromeda-pulse\logs\` on Windows). The `collect_log_files` helper globs `agent-latest.jsonl*` in the resolved log dir; the zero-panic gate then scans ALL matched files for `app.panic.fatal` records. ANY historical panic record (e.g., from chunks #27-#30 latent-panic era per testing.md 2026-05-09) anywhere in the daily-rolled archive fails the gate even when the current chunk doesn't touch boot paths. In CI the data dir is `${{ runner.temp }}/andromeda-pulse-ci-data/` (ephemeral per run) so historical residue can't accumulate. To verify a chunk's effect on `ci-gates` in isolation from dev-env residue, run `ANDROMEDA_PULSE_DATA_DIR="$TEMP/andromeda-pulse-ci-gates-verify-$$" cargo xtask ci-gates` (pointing at a fresh empty dir); the gate will return NEUTRAL on empty logs, confirming chunk code is correct. Caught at chunk #55 /implement Phase 2 when local ci-gates failed on `agent-latest.jsonl.2026-05-07:7` while chunks #55+#56 only touched CI tooling layer (xtask + ci.yml + workflow self-lint tests). Pattern: when ci-gates fails locally with a pre-dating panic record, classify as out-of-scope per Phase 2 §Bounded retry caps strict scope classification (originates from `crates/*/src/` files NOT touched in current implementation); verify CI-equivalent behavior via fresh-dir override; proceed to Phase 3 report with note. Optional one-time dev-env maintenance: delete stale `agent-latest.jsonl.YYYY-MM-DD` files containing past-known-panic records (out-of-project-tree operation; analogous to `cargo clean`).

- 2026-05-19: `/andromeda-implement` Phase 2b smoke check using `npx @tauri-apps/cli dev` in background via `run_in_background: true` has a Windows-specific failure mode: the spawned pulse-app.exe is NOT reaped when the outer bash command hits its 10-min timeout. The dev binary keeps running with the file handle open, blocking subsequent `cargo nextest run --workspace` (or any cargo command rebuilding pulse-app) with `error: failed to remove file 'D:\...\target\debug\pulse-app.exe' Caused by: Access is denied. (os error 5)`. The bash tool's timeout reaps the spawning shell + `head -60` filter pipe but does NOT cascade-kill the child Tauri process. **Diagnostic recipe:** `tasklist | grep -iE "pulse|tauri"` after suspicious bash timeout to surface the orphan PID. **Recovery:** `Stop-Process -Id <PID> -Force` via PowerShell (`taskkill /F /PID <N>` from git bash misinterprets `/F` as a Unix path; use cmd.exe or PowerShell). Mass-killing (`taskkill /F /IM cargo.exe /T`) is correctly blocked by Claude Code's auto-mode classifier because it affects shared dev environment; kill-by-specific-PID is the safe path. **Prevention for future Phase 2b smoke runs on Windows:** prefer `cargo build -p pulse-app` (no Tauri runtime spawn) + integration-test runtime smoke (e.g., `pulse-app::e2e_p1_otlp_grpc_to_traces_query` exercises boot+ingest+query roundtrip in ~2s) as a lightweight runtime-smoke alternative for non-UI chunks. Tauri dev launch as smoke is meaningful only for UI-touching chunks where webview behavior matters; for backend-only sessions (e.g., session 98 chunk #69 Phase B Sessions 1-2: schema + appender + consumer wiring) integration tests cover the same runtime invariants more reliably + with proper process cleanup. Verified at session 98 Session 2 when PID 44412 lingered from Session 1's tauri dev background run, blocking the wrap-time nextest re-run until killed by specific PID. **[extended 2026-08-23]** The `/F` mangling is a CLASS, not one flag: `tasklist /FI "PID eq N"` fails the same way (`Invalid argument/option - 'C:/Program Files/Git/FI'`) — any `/X` switch to a native Windows tool is a candidate for Git Bash path conversion. The dangerous part is not the failure but its SHAPE when the probe is written `tasklist /FI … || echo "not running"`: a zero-is-healthy `||` fallback fires on the COMMAND ERROR and prints a conclusion indistinguishable from a genuine negative result, so a liveness/absence check reports "not running" without ever having looked. Write such probes with PowerShell (`Get-Process -Id N -ErrorAction SilentlyContinue`) whose exit semantics survive the shell, and never let a `||` branch be the only thing that produces the answer — the branch must be reachable ONLY by the probe genuinely succeeding-and-finding-nothing.

- 2026-06-29: Tauri 2 embeds the capability ACL at COMPILE time — `tauri-build` reads `pulse-app/capabilities/*.json` + `tauri.conf.json` during `cargo build` and generates the embedded ACL (`target/debug/build/pulse-app-*/out/capabilities.json` + the `acl-manifests`); an INVALID permission identifier (or an invalid window-config field) FAILS the build. Consequence for the boot-smoke decision: for a capability/config-only change (granting `core:window:*` perms, adding `"center": true`, a new window), a successful `cargo build -p pulse-app` already VALIDATES + embeds the permissions — a GUI boot smoke (`cargo run`/`tauri dev`) adds NO capability-validation value beyond the build (Tauri capabilities are not a runtime "load" that could fail post-build). Pair this with the 2026-05-19 Windows GUI-orphan hazard: boot smoke is legitimately SKIPPABLE-with-cause when (a) the boot-path change is capability/config + setup-closure wiring that is panic-safe by construction (sync best-effort IO via `unwrap_or_default`/`let _`, window ops warn-on-err, and NO new async/spawn/reactor → the "no reactor running" latent-panic class cannot apply), AND (b) the embedded ACL is confirmed via `find target -path '*pulse-app*' -name capabilities.json` (or grep the generated acl-manifest for the granted permission names). Record the skip + its cause in the report. Runtime UI behavior that the build can't prove (e.g. an actual drag-delta) is then a CARRY to the headful e2e/Conductor suite, not a reason to risk the orphan-hazard GUI launch. Verified at chunk 2026-06-29-window-geometry-movable-shell (P-061).

- 2026-06-29: Every agent-log READER must glob `agent-latest.jsonl*` (read all matches + concatenate non-empty lines), NEVER open / `tail` the bare `agent-latest.jsonl` nor a `*.log` glob — because the obs sink is `tracing_appender::rolling::daily(logs_dir, "agent-latest.jsonl")` (`pulse-app/src/observability.rs`), which DATE-SUFFIXES every file (`agent-latest.jsonl.YYYY-MM-DD`). A bare-name reader SILENTLY finds nothing (zero log lines) and a boot-health / log assertion then false-reports "no spans / empty log" with no obvious cause. (Complements the 2026-05-14 ci-gates `collect_log_files` entry, which documents the date-suffix fact only incidentally — this is the forward rule + the silent-failure warning it lacks.) Verified at the predictable-close-self-verify chunk: the NEW `cargo xtask self-verify` first false-failed shell-health reading the bare name; fixed `read_log_lines` to glob the family. KNOWN latent bare-name readers handed off for a targeted cleanup chunk: `xtask/src/smoke.rs::run_smoke` (l.95), `scripts/agent-run.sh` `logs` cmd (`$DATA_DIR/logs/agent-latest.jsonl`), and `test-plan §3` `logs` cmd (`*.log`). When writing OR reviewing any new harness / test / xtask log-reader, glob the date-suffixed family from the start.

- 2026-06-30: EXTENSION to the 2026-06-29 build-can't-prove-runtime-UI entry above — the `cargo xtask self-verify` boot-quit harness proves the shell BOOTS (window shown + receivers + heartbeat + zero panics) but it NEVER CLICKS a user-facing control (it hides/quits programmatically). So a UI capability can be matrix-`verified` yet ship a DEAD affordance. Verified at chunk 2026-06-29-window-size-constraints: P-063 (predictable-close) was matrix-`verified`, yet the custom-titlebar ✕ did NOTHING — `getCurrentWindow().close()` was silently rejected by the negative-default ACL (missing `core:window:allow-close`; see security.md 2026-06-29), so the Rust `CloseRequested`→hide-to-tray interception never fired. 1719 nextest + the boot-quit self-verify ALL passed; only a human clicking ✕ (or the deferred P-076 headful tauri-driver e2e, which drives the real DOM) surfaced it. Discipline: a chunk whose acceptance is "a user-facing control WORKS" is NOT fully proven by unit tests + boot-quit self-verify — its live click/keyboard behaviour CARRIES to the headful e2e suite (P-076) or needs manual verification, and the matrix `ref` should say so. Corollary: when a frameless custom-titlebar button "does nothing", suspect a missing `core:window:allow-*` grant FIRST (the negative-default ACL drops the IPC silently).

- 2026-06-30: EXTENSION to the 2026-06-30 self-verify-never-clicks entry — the TECHNIQUE for proving a UI affordance the boot-quit self-verify can't click. When a UI behavior carries to a manual headful click-through (or a future tauri-driver e2e), use the structured obs log as OBJECTIVE evidence of what the user's clicks did: the nav/close behaviors emit `ui.layout.transition` (`layout_mode_from`→`layout_mode_to`) + `tray.signpost.shown`; grep their targets/counts/timestamps from the date-suffixed `agent-latest.jsonl.<date>` family (per the 2026-06-29 glob entry) to turn a subjective "looks right" into a checkable assertion WITHOUT a headful driver — e.g. N `tray.signpost.shown` firings = N app-to-tray closes (every-time toast); `layout_mode_to=top-right` = a corner snap, not a stale "remembered" restore; `compact-widget→hidden` vs `main→hidden` distinguishes WHICH window closed (the per-window close model). Verified across 3 dogfood rounds at 2026-06-30-widget-to-dashboard-navigation (6 toast firings one-per-widget-close; dashboard closes silent — all log-confirmed against the user's live click-through).
