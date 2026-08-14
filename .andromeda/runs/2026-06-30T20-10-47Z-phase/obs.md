# obs extract

## Relevance — widget-to-dashboard navigation affects IPC instrumentation + P5 critical path

## Constraints

- (obs-plan §1 Instrumentability) TauRPC router methods for IPC command surfaces must emit instrumentation with traceparent propagation — any new window-open/focus IPC procedure (if chosen over direct `@tauri-apps/api` call) requires `#[tracing::instrument(skip_all, fields(traceparent = %tp))]` on the handler.
- (obs-plan §5 P5) UI layout transitions trigger `ui.layout.transition` span with required fields: `layout_mode_from`, `layout_mode_to`, `tray_visible`, `health.status`, `health.subsystems`, `ipc.contract.valid`.
- (obs-plan §2 Naming conventions) Span naming follows `{module}.{operation}` pattern (e.g., `ui.window.show.request`, `tray.visibility.toggle`); avoid high-cardinality span names (window labels must not be unbounded).
- (obs-plan §3 Service identity) All spans inherit `service.name` (`com.andromeda.pulse`), `service.version`, `deployment.environment` as default subscriber fields — no per-call boilerplate needed.
- (obs-plan §8 PII Scrubbing Vector 3) Log only basename for file paths (if widget config paths appear in error logs); never full canonicalized paths.
- (obs-plan §3 Heartbeat ticks vs health command) Complementary liveness: heartbeat ticks (async, 15s, in JSON log) for retroactive stall detection; `health` IPC command (sync polling) for active boot liveness. Both may be triggered by the activation of this affordance's window-show sequence.

## Patterns to follow

- (obs-plan §1 Telemetry surfaces — IPC-internal) Instrument TauRPC router method entry with `#[tracing::instrument(skip_all, fields(traceparent = %tp))]`; extract traceparent from IPC envelope, attach to span as plain field; downstream calls inherit via `tracing::Span::current()` for unified trace_id across widget→window→buffer pipeline.
- (obs-plan §5 P5 critical path) Emit `tracing::info!(target: "metric.ui.layout.transition", layout_mode_from = "widget", layout_mode_to = "dashboard", ...)` event when affordance activated; correlate with `health` command response for contract validation.
- (obs-plan §6 Log Coverage) Structured JSON fields in span body: `duration_ms`, `transition_success`, `window_focus_acquired`, `ipc_contract_valid`.

## Anti-patterns to avoid

- (obs-plan §8 PII Scrubbing) Do not log full window file paths or config directory paths in transition spans; emit only operation outcome (success/failure) and duration.
- (obs-plan §2 Agent-readable invariants) Do not emit high-cardinality window labels or user-facing UI state enums without cardinality check; prefer boolean flags (`window_visible: true`) over unbounded label enums.

## Contract bindings

- **TauRPC router contract** (obs-plan §3 Standard Contracts): if new `window.show_dashboard` IPC procedure added, binds to tests verifying the handler emits traceparent-propagating span + correct `health` status.
- **P5 critical path** (obs-plan §5): cross-binding to tests verifying `health` command JSON schema consistency + UI transition fields present in logs.

## Acceptance criteria contributions

- "(obs) Affordance activation emits structured span with `layout_mode_from`, `layout_mode_to`, `window_focus_acquired` fields in JSON log."
- "(obs) W3C traceparent propagated through IPC envelope (if TauRPC path chosen); single trace_id correlates widget button click → window.show span → focus operation."
- "(obs) No unredacted file paths or raw window labels in telemetry; only operation outcome + duration logged."

## Relevant amendment history

- 2026-05-08 — Heartbeat ticks vs health command complementarity (obs-plan §3): Clarified that `health` command (sync, for active boot liveness) and heartbeat ticks (async, 15s, for retroactive stall detection) are complementary. This chunk's affordance activation will likely trigger a `health` check to verify IPC contract consistency before showing the dashboard; amendment ensures both signals are captured and not conflated.
