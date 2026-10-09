# obs extract

## Relevance
partial — adds new LLM-backed analyze operation with explicitly-scoped aggregate observability (no prompt/result content in logs); not a critical path (P1–P7) but follows standard instrumentable-operation pattern.

## Constraints
- Per §1 (Instrumentable entities table, row "snapshot"): New user-facing analyzed operations follow snapshot's instrumentation boundary pattern — entry span, child steps (query/process/validate), metadata emission; investigate action is analogous.
- Per §2 (Agent-readable invariants): All observability produces JSON-per-line via `tracing` to `~/.andromeda-pulse/logs/agent-latest.jsonl`; no human-only surfaces.
- Per §5 (logging-sensitive Vector 1): Raw OTLP attribute values / captured telemetry payloads must NEVER appear in logs — only aggregate counts emitted.
- Per §5 (logging-sensitive Vector 4): Tool response bodies / analysis result text must NEVER be logged; only result_type + result_count metadata captured.
- Per §3 (Tracing init): Self-observation uses `tracing` 0.1 + `tracing-subscriber` 0.3 JSON formatter exclusively; no OTel SDK in self-runtime.
- Per §1 (Telemetry surfaces, desktop-webview): TauRPC procedures instrument via `#[tracing::instrument]` on handler; trace context propagates via IPC envelope `traceparent` field as plain `tracing` field.

## Patterns to follow
- Per §1 (Critical Path P2, snapshot pipeline): Investigate action mirrors entry span → child operations → result metadata flow. Use parent span `session.investigate`; child spans `llm.inference.query`, `analyze.result.validate`.
- Per §2 (Naming conventions): `investigate.{action_id}.request`, `llm.inference.query`, `metric.investigate.duration_ms` (target prefix for metric events).
- Per §1 (Service identity §3): Register default subscriber fields once at init: `service.name: "com.andromeda.pulse"`, `service.version` (from Cargo.toml), `deployment.environment: "production"` — all spans inherit automatically.
- Per §1 (Trace context propagation): Attach IPC envelope `traceparent` field to span as `fields(traceparent = %tp)`; downstream spans inherit via `tracing::Span::current()`.

## Anti-patterns to avoid
- Per §5 (Vector 1): Do NOT log raw telemetry attributes from captured buffer context; emit only `telemetry_context_size_bytes`, `spans_analyzed_count` (aggregate only).
- Per §5 (Vector 4): Do NOT log LLM result body or analysis text; emit only `result_type: "analysis"`, `result_length_chars`, `error: {code, user_message}` (metadata only).
- Per §5 (Vector 2): Do NOT expose stack traces or Rust struct names in logs; use `tracing-error` SpanTrace (user-defined spans only) for error chains.

## Contract bindings
- **Test harness contract** (focus guide cross-domain bindings): E2E / webview tests consume `investigate.action.request` + `investigate.action.response` span frames from JSON log; assert span present + result metadata captured + no raw content leaked.
- **Security plan, logging-sensitive vectors** (§5): Vectors 1, 2, 4 bind to this chunk's error rendering + telemetry filtering; logger redaction config enforces field skips (skip_all on raw buffers, custom Layer redaction on error bodies).

## Acceptance criteria contributions
- "(obs) Investigate action span emitted: `investigate.{action_id}.request` (entry) → child `llm.inference.query` (LLM call with model_name, deterministic_mode_enabled, duration_ms, token_count_input, token_count_output fields) → `investigate.{action_id}.response` (exit with status: success/error/timeout, result_count, error_message if error)."
- "(obs) Aggregate-only logging: investigate telemetry emits action_id, status, duration_ms, result_count, deterministic_mode_enabled — NO prompt template, NO analysis text, NO raw telemetry attributes in JSON log."
- "(obs) Error sanitization: `AppError` logs emit error_category + user_message only; no stack trace, no Rust struct names, no file paths; child span marked with error tag via `tracing-error` SpanTrace."
- "(obs) Negative test: capture telemetry with secret attribute → invoke investigate action → verify secret NOT in logs; only context_size_bytes + span_count emitted."

## Relevant amendment history
- **2026-06-10 (chunk #99 tag gate):** L4 latency budgets per hardware profile confirmed canonical in `xtask/ci/l4-latency-p99`. Investigate action runs deterministic L4 model; must emit `llm.inference.query` span with duration_ms for post-test SLO assertion (p99 latency check per chunk's own performance gates). Frame-budget posture active when webview rendered during inference; `metric.webgpu.frame_duration_ms` events from frontend bridge track concurrent rendering.
- **2026-05-04 (PII grep heuristic UI-vocabulary exemption):** PII heuristics distinguish secret *formats* (regex patterns) from UI labels. Investigate action may reference "Token budget" (label, not PII) in progress message; never log token_count_actual or per-service metrics extracted during analysis (secret format, PII).
- **2026-05-02 (tracing-only self-observation pivot):** Abandoned OTel SDK from self-observation runtime. Investigate action's `interpretation` crate dependency must NOT introduce OTel SDK into self-runtime; wire-format parsing is fine (for external OTLP), but all self-observation spans must use `tracing` 0.1 events only.