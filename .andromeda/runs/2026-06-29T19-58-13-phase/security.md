# security extract

## Relevance
Partial — the chunk implements window-close behavior and an agent-headful self-verify harness that reuse existing Tauri IPC boundaries and window-geometry inputs (already registered); it does not introduce new threat surfaces, only applies established security constraints to new behavior.

## Constraints

1. **Tauri capability authorization (§API Security TauRPC capability authorization)** — The close signpost (if using `pulse:notification`) must respect existing capability gates; the three NEVER-widen caps (`pulse:notification`, `pulse:tray`, `pulse:plugin-fs`) remain outbound-only. Xtask `capability-widening-check` must pass with no regressions.

2. **Error sanitization at IPC boundary (§Error Handling external responses)** — All close-behavior and harness errors crossing the TauRPC bridge must convert to serde-able `AppError` enum variants; no stack traces, file paths, or Rust struct names exposed to the webview.

3. **Logging discipline — NEVER-log rules (§Logging & Monitoring NEVER-log list; §Security Anti-Patterns §Logging)** — The harness and close signpost MUST NOT log: OTLP attribute values, window-geometry coordinates, notification toast contents, clipboard data, or MCP responses. Uniform scrubber discipline applies at all persistence boundaries (§Logging uniform scrubber coverage).

4. **Existing IPC procedure validation (§API Security §Input Validation tables)** — The harness uses only enumerated TauRPC procedures (`app_info`, `health`, `ready`, etc.) with serde deserialization + smart enum types. No new argument types introduced.

5. **Window-geometry.json input validation (2026-06-29 amendment §Input Validation)** — Harness asserts on window-geometry.json using already-validated integer x/y via serde to `Position { x: i32, y: i32 }` with graceful default; no additional validation required.

6. **No OTLP attribute surfaces in signpost (§Logging & Monitoring snapshot/clipboard hygiene note)** — Close signpost (notification or tooltip) must not emit telemetry attribute data incidentally captured from the app's instrumentation; document user-facing warning that running-state signals are app-internal, not telemetry-derived.

## Patterns to follow

1. **Capability-gating pattern (§API Security)** — Use Tauri 2 `pulse:default` for harness IPC; delegate close signpost notification to `pulse:notification` capability + existing `notifications_enabled` opt-out setting.

2. **Error-to-AppError bridge (§Error Handling §Bootstrap phases error-sanitization-wire)** — Harness internal errors convert at the IPC boundary via `From<HarnessError> for AppError::Internal { message }` (sanitized one-liner for UI; full chain stays in internal logs).

3. **Logging redaction at subscriber layer (§Logging & Monitoring log format; 2026-06-28 amendment §Logging opentelemetry-stdout → tracing-subscriber)** — Harness emits via `tracing` with field redaction applied at `tracing-subscriber` JSON formatter layer, not at call sites. Span-local context only (no coordinates, no OTLP attributes).

4. **Input validation boundary registry (§Bootstrap phases input-validation-library-install)** — Chunk reuses window-geometry.json (already registered amendment 2026-06-29) and existing IPC procedures (already enumerated). No new env-var or config boundaries introduced.

## Anti-patterns to avoid

1. **Capability widening (§Security Anti-Patterns §API — NEVER widen pulse:notification/pulse:tray/pulse:plugin-fs)** — Close signpost notification is outbound-only emit; xtask `capability-widening-check` must pass.

2. **Logging sensitive fields (§Security Anti-Patterns §Logging)** — NEVER log window coordinates, notification toast text, OTLP attribute values, clipboard contents, or harness subprocess telemetry output; NEVER expose stack traces, file paths, or struct names in error messages across the TauRPC bridge.

3. **Unvalidated IPC inputs (§Security Anti-Patterns §Code Patterns — NEVER serialize anyhow::Error directly across TauRPC bridge)** — All harness errors must convert to `AppError` enum; no `thiserror`/`anyhow` Error direct serialization.

## Contract bindings

- **Tests harness ↔ a11y harness** — Scope declares reuse of a11y/contrast harness from a11y-plan §3; security domain constraint is zero-logging of test assertions + no attribute/notification leakage.
- **Obs harness** — Harness tracing to `~/.andromeda-pulse/logs/agent-latest.jsonl` per §Logging self-observation (tracing-only; field redaction at subscriber layer per 2026-06-28 amendment).

## Acceptance criteria contributions

1. **(security) Xtask `capability-widening-check` passes** — TauRPC procedures match `pulse-app/capabilities/pulse:default.json` enumeration; NEVER-widen caps unwidened.

2. **(security) Error handling** — Close-behavior and harness errors convert to `AppError` enum variants (no stack traces, paths, or struct names in webview-facing messages); verified by grep and compile checks.

3. **(security) Logging discipline** — Harness tracing events do NOT log window coordinates, notification toast contents, OTLP attributes, or subprocess telemetry; verified by scanning `~/.andromeda-pulse/logs/agent-latest.jsonl` output and clippy on harness code.

4. **(security) Window-geometry.json access** — Harness does not re-validate; reads use already-validated `Position` struct from amendment 2026-06-29; no raw integer parsing or path operations introduced.

## Relevant amendment history

- **2026-06-29-window-geometry-movable-shell** (amendment §Input Validation, persisted window geometry row) — P-061 registered window-geometry.json boundary with serde integer validation + graceful default + non-fatal missing/corrupt handling. This chunk may read this file via harness assertions; boundary already validated.

- **2026-06-28 opentelemetry-stdout deprecation** (amendments §Data Protection logs, §Bootstrap logging-redaction-wire, §Logging log format) — Harness logging uses `tracing-subscriber` JSON formatter at `~/.andromeda-pulse/logs/agent-latest.jsonl` (no OTel SDK); field-level redaction applied at subscriber layer per amendment.

- **2026-06-28-deterministic-env-gated-l4-mode** (amendment §Input Validation CLI/env var row) — Shows pattern for registering new env vars; chunk does not add new env-var boundaries (reuses existing config + window-geometry.json).