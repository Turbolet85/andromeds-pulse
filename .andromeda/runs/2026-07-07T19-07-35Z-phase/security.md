# security extract

## Relevance
Partial — a webview display surface consuming existing telemetry/state contracts; security is a standing display/error/log-hygiene constraint plus conditional IPC-boundary rules that fire only if P3 research surfaces a backend delta.

## Constraints
- The status line and the folded ConnectionDot tooltip MUST render only aggregate/derived values (live-source count, human-rounded spans/s, buffer minutes, recency timestamps) — never raw OTLP attribute values, span/log payloads, or DuckDB query-parameter values, which are user-controlled content that may carry incidentally captured secrets (per security-plan §Threat Model Summary data-classification sensitivity note + §Logging & Monitoring NEVER-log list).
- Any error the status line surfaces to the webview MUST collapse to a sanitized `AppError` variant — no stack traces, file paths, Rust struct names, or library versions in `AppError::Internal { message }` (per security-plan §Error Handling + §Security Anti-Patterns §Logging bullet 4).
- IF research shows a backend delta is required and it adds/extends a TauRPC procedure, every `#[taurpc::procedure]` argument MUST be `serde`-validated with `AppError::Validation { field, reason }` on rejection (per security-plan §Input Validation, TauRPC bridge row).
- IF a delta adds a NEW TauRPC procedure, both enforcement pieces are required — router registration AND a `pulse-app/capabilities/` entry (xtask drift check); reusing/extending an existing procedure or payload rides the existing `pulse:default` grant and adds no capability entry (per security-plan §API Security, TauRPC capability authorization).
- Any new log emission added by a backend delta MUST NOT log raw OTLP attribute values or DuckDB query parameters — log identifiers/counts only (per security-plan §Security Anti-Patterns §Logging bullets 1 + 3).
- The buffer-fill readout consuming `ANDROMEDA_PULSE_RETENTION_SECONDS` reuses an already-registered + bounded (60–86400s) env boundary; reading it for display introduces no new input boundary (per security-plan §Input Validation, CLI/env var inputs row).

## Patterns to follow
- Module-internal `thiserror` 2.x enums converting to the `serde`-friendly `AppError` at the bridge via `From` impls, with the sanitized one-liner going to the UI and the full chain staying in the internal log (per security-plan §Error Handling, TauRPC bridge).
- Reuse existing procedures (`connection.current_state`, `services.list_with_states`) and the `pulse://stream/connection-state` topic — outbound, first-party payloads under the existing `pulse:default` capability — rather than a new namespace (per security-plan §API Security TauRPC capability authorization; aligns with scope reuse-first discipline).
- Consuming already-validated config/state rather than re-parsing untrusted input at this surface (per security-plan §Input Validation, env-var + config-values rows).
- If any logging is touched, field redaction is applied at the `tracing-subscriber` layer, not at call sites (per security-plan §Logging & Monitoring, Log format).

## Anti-patterns to avoid
- NEVER add a TauRPC procedure to a router without a matching `pulse-app/capabilities/` entry — the call is silently rejected at runtime (per security-plan §Security Anti-Patterns §API).
- NEVER serialize an `anyhow::Error` (or leak stack traces / struct names / paths / library versions) across the TauRPC bridge — convert to `AppError` first (per security-plan §Security Anti-Patterns §Code Patterns + §Logging).
- NEVER log or surface raw OTLP attribute values, span/log payloads, or DuckDB query parameters via the new status surface (per security-plan §Security Anti-Patterns §Logging bullet 1).

## Contract bindings
- Aggregate-only display hygiene binds to obs §PII Scrubbing (the uniform `security::scrubber::scrub_attribute` coverage) + tests (no real PII in fixtures) — this chunk does not persist, so the scrubber is not newly invoked; the binding is the display surface staying aggregate.
- IF a new capability entry is added, the xtask capability-drift check binds to tests §CI Integration (runs as the CI security gate job).
- Visible-text legibility of the status line is a11y/design territory (WCAG, typography tokens), not security — flagged as out of my domain.

## Acceptance criteria contributions
- (security) Review/grep of touched webview + any backend code confirms the status line and ConnectionDot tooltip surface only aggregate/derived values — no raw OTLP attribute value, span payload, or DuckDB query parameter is rendered or logged.
- (security) Any error path in the status surface yields an `AppError` variant with a sanitized message — no stack trace / path / struct name / library version reaches the webview.
- (security) If a new TauRPC procedure is introduced, the xtask capability-drift check passes (declared procedures == `pulse-app/capabilities/` JSON); if none is added, this is trivially satisfied.
- (security) No new or unpinned Rust dependency is introduced — `cargo deny check` / `cargo audit` remain green (a webview-only chunk is expected to add zero deps).

## Relevant amendment history
Both trailing amendments — `2026-06-28-deterministic-env-gated-l4-mode` (registered `ANDROMEDA_PULSE_L4_DETERMINISTIC`) and `2026-06-29-window-geometry-movable-shell` (registered `window-geometry.json`) — added NEW deserialized-input boundary rows to §Input Validation under the bounded-config-input playbook (code-validated + unit-tested ⇒ registry-completeness applied silently, not an unvalidated-boundary HALT). Relevance: they establish that any new deserialized input boundary must get a §Input Validation row. This chunk's deltas (if any) are read/producer/outbound paths consuming already-registered surfaces (`connection.current_state`, `services.list_with_states`, `ANDROMEDA_PULSE_RETENTION_SECONDS`), so no new input boundary is expected and the playbook does not trigger. The window-geometry entry is the nearest-area precedent (a dashboard-shell chunk) but touches the geometry file boundary, which this chunk does not.
