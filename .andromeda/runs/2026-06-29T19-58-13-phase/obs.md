# obs extract

## Relevance
Partial — P-063 (close behavior) requires UI lifecycle instrumentation; P-078 (self-verify harness) is a test framework that consumes obs infrastructure, not an instrumentation scope itself.

## Constraints

1. **Obs tier Standard + desktop-native surface** — per obs-plan §1 Obs Scope Summary, desktop-native (tray) is an instrumentable surface requiring Tauri lifecycle span emission (`tracing::info!(target: "tray.{event}", ...)` on tray visibility/dismiss/action).

2. **Panic hook mandatory + zero-panics SLO** — per obs-plan §3 Tracing init, `std::panic::set_hook()` calling `tracing::error!(target: "app.panic.fatal", ...)` is required at boot before Tauri app spawn; obs-plan §10 Error-budget-SLO enforces zero panics/month.

3. **Status endpoint shape + heartbeat complementarity** — per obs-plan §3 Snapshot section + obs-plan §10 (2026-05-08 amendment), `health` IPC command must expose subsystems fields synchronously for active liveness probing; heartbeat ticks (async, 10–30s interval, >45s absence = stall) are complementary for retroactive analysis. P-078 harness uses both.

4. **Log format JSON schema binding** — per obs-plan §3 Logging stack + §6 Log Coverage, all structured logs match verbatim schema (timestamp/level/target/message/fields with trace_id/span_id when available); P-078 harness parses `agent-latest.jsonl` for deterministic pass/fail.

5. **Trace context propagation + span current inheritance** — per obs-plan §3 Trace context propagation, W3C `traceparent` extracted at boundaries and attached as regular `tracing` span field; downstream calls inherit via `tracing::Span::current()` for end-to-end correlation.

6. **PII vector 6 (path sanitization)** — per obs-plan §1 Logging-sensitive trigger vector 6, env vars subject to CWE-22 are canonicalized at startup and logged basename-only; any data-dir config errors must not leak full paths.

7. **A11y violation JSON schema binding** — per focus guide cross-domain bindings, a11y-plan §3 violation output must conform to obs structured log format for harness consumption by P-078.

## Patterns to follow

1. **Tauri lifecycle tray events** — per obs-plan §1 desktop-native surface: emit `tracing::info!(target: "tray.{event}", ...)` on close-requested/hide/show/quit actions; P-063 `window.close.requested` handler emits span with `action_taken` field (hide-to-tray or quit per P4 decision).

2. **UI layout transitions** — per obs-plan §1 critical path P5, emit `ui.layout.transition` span with `layout_mode_from`, `layout_mode_to` fields; if P-063 close-behavior includes tray visibility toggle, instrument with same pattern.

3. **IPC health consistency verification** — per obs-plan §1 critical path P5, `ipc.health.check` span wraps consistency verification; P-078 harness invokes TauRPC `health` command post-boot and parses response JSON to assert subsystems present.

4. **Agent-readable JSON file sink** — per obs-plan §3, all telemetry flows to `~/.andromeda-pulse/logs/agent-latest.jsonl` via `tracing-appender` daily rotation; P-078 harness tails this file to verify no ERROR/FATAL during boot and heartbeat presence.

## Anti-patterns to avoid

1. **Raw stack traces in logs** — per obs-plan §1 logging-sensitive vector 2, never expose Rust struct names or full backtraces; use `tracing-error` SpanTrace (user-defined spans only) when capturing panics in the error hook.

2. **PII grep false positives on UI vocabulary** — per 2026-05-04 amendment, PII heuristics must distinguish literal UI labels ("Token budget") from secret formats (regex envelopes); P-063 signpost messaging or P-078 assertion output must not trigger redaction on innocuous label text.

## Contract bindings

- **obs ↔ tests:** §3 Observability Harness Contract binds to tests §3 (tests consume structured log format + status endpoint shape); P-078 harness reuses a11y/contrast and IPC introspection as the test vehicle.
- **obs ↔ a11y:** a11y-plan §3 violation JSON schema output must conform to obs log format (per focus guide); P-078 harness chains a11y assertions into the self-verify pipeline.
- **obs ↔ arch:** desktop-native tray surface per arch §Tray icon policy; P-063 close behavior aligns to "closing main window minimizes to tray" architectural default.

## Acceptance criteria contributions

1. "(obs) P-063: `window.close.requested` span emitted on titlebar/taskbar close; logs record `action_taken: "hide_to_tray"` or `"quit"` per P4 decision; if OS notification sent on first close, `pulse:notification` boundary captured without exposing user config secrets."

2. "(obs) P-063: Tray state transitions logged via `tracing::info!(target: "tray.{event}", ...)` on visibility toggle; P-078 harness verifies tray presence post-close via `health` endpoint or IPC consistency check."

3. "(obs) P-078: Agent-headful harness boots real app, reads `agent-latest.jsonl` to verify (1) `app.boot` span + children (`window.init`, `workspace.detect`) present within 5s; (2) zero ERROR/FATAL log lines during boot; (3) heartbeat ticks (`ingest.tick`, `buffer.tick`) present every 45s max; (4) `health` IPC command returns valid JSON with subsystems fields; (5) clean quit with zero orphan processes (Windows `tasklist` / Unix `ps` confirmation)."

4. "(obs) P-078: All a11y/contrast assertions from a11y-plan §3 harness integrated into self-verify pipeline; violations logged in obs structured format; zero violations reported in exit status."

## Relevant amendment history

**2026-06-10 — Chunk #99 tag gate: frame-budget posture + heartbeat ticks verification** — Clarified heartbeat-tick complementarity (async 10–30s, >45s absence = stall for retroactive analysis) vs. `health` command (sync polling, active liveness). P-078 harness uses both: tail JSON file for tick presence; poll `health` endpoint for active readiness. Topology confirmed: write/sweep/read isolation on DuckDB `:memory:` prevents stalls. Relevant: P-078 liveness heuristic derives from chunk #99 findings.