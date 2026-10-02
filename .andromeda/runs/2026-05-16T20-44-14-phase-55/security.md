# security extract — phase-55

## Chunk relevance

- **chunk #59 "Connection state machine"** — IN DOMAIN. Introduces (a) new TauRPC procedure `connection.current_state` (capability gating + AppError sanitization apply), (b) new broadcast topic `pulse://stream/connection-state` (outbound IPC payload discipline + cardinality control apply), (c) atomic state tracker in OTLP receiver hot path (loopback-only binding invariant must be preserved when modifying ingest), (d) receiver panic-hook wiring (error sanitization at boundary + no stack-trace leakage into broadcast), (e) state values potentially derived from receiver failure modes (logging redaction for failure causes).

## Constraints

1. **TauRPC capability double-binding (4-place pattern per .claude/rules/security.md Session Additions 2026-05-12)** — Adding `connection.current_state` requires: (1) router registration in owning crate, (2) entry in `pulse-app/capabilities/` JSON, (3) `xtask/src/main.rs::EXPECTED_PROCEDURES` extension, AND (4) `emit_taurpc_bindings` test in `pulse-app/src/main.rs::tests` extended to include the new router's `.into_handler()` in `.merge(...)`. Per security plan §API Security row "TauRPC capability authorization" + Anti-Patterns §API row 3 (silent runtime rejection otherwise).

2. **Loopback-only invariant preserved when modifying ingest hot path** — Per security plan §API Security + Anti-Patterns §API row 6: OTLP receivers MUST bind `127.0.0.1` only. LastIngestTracker insertion in tonic 0.14 / axum 0.8 receive paths MUST NOT alter the bind address or remove `.max_decoding_message_size` / `DefaultBodyLimit` / Host-header allowlist / CORS deny / tower_governor layer ordering.

3. **Receiver panic-hook MUST NOT leak sanitization-banned fields** — Per security plan §Anti-Patterns §Logging row 1+4 + §Error Handling: ReceiverFailed state payload broadcast on `pulse://stream/connection-state` MUST NOT include stack traces, Rust struct names, file paths, library versions, OTLP attribute values, or DuckDB query parameters. Panic payload content sanitized at the From<PanicInfo> conversion site, not at the broadcast site.

4. **New broadcast topic `pulse://stream/connection-state` payload size discipline** — Per security plan §Input Validation last row (Tauri IPC Channel API streaming payloads) + Anti-Patterns §Input row 6: state-change payloads are first-party Rust-emitted (no plugin-sourced bytes) and bounded by the enum-variant size (5 variants × small metadata), but apply the same Channel API size-cap discipline as other `pulse://stream/*` topics so a future amendment that adds free-form metadata cannot regress to unbounded payloads.

5. **State machine state values are NOT incidental-secret carriers** — Per security plan §Logging "What NEVER to log" list: state-transition events MUST NOT echo OTLP attribute values, span/trace IDs, or any user-instrumented data even if the receiver failure was triggered by malformed input. The five enum variants {Listening, Receiving, Idle, Stalled, ReceiverFailed} are spec-fixed; failure metadata (e.g., bind-error string for ReceiverFailed) goes through the sanitized one-liner discipline of `AppError::Internal { message }`.

6. **AppError sanitization at the `connection.current_state` TauRPC boundary** — Per security plan §Error Handling + Conventions Error response schema: the new procedure MUST return `Result<ConnectionState, AppError>` where any internal error path collapses to `AppError::Internal { message: "<sanitized one-liner>" }` (e.g., "connection state unavailable" — never "PoisonError<RwLockReadGuard<LastIngestTracker>> at ingest/src/state.rs:42").

7. **DuckDB query parameter discipline preserved if state computation touches buffer** — Per security plan Anti-Patterns §Input row 2: if the connection state poller queries DuckDB for last-ingest-timestamp (instead of an in-process atomic), the query MUST use `Connection::prepare` + `?` placeholders; NEVER `format!(...)`. (Likely N/A — the route entry says "LastIngestTracker atomic" which is in-process — but flagged for plan.md if implementation deviates.)

## Patterns to follow

- **Settings-extension lower-cost path NOT applicable here** — Per .claude/rules/security.md Session Additions 2026-05-09 entry (chunk #30 precedent): when a new capability fits get/set semantics on `Settings`, extend Settings instead of adding a TauRPC namespace. Chunk #59 is "stream a state value + return current state on demand," NOT a persistable setting, so the new `connection.*` namespace is the correct shape (decision rule from that entry concludes "operations like 'invoke action X' or 'subscribe to stream Y' still need their own IPC namespace and incur the triple binding cost").

- **Couple D3 capability-drift cleanup with consuming chunk per .claude/rules/security.md Session Additions 2026-05-10** — If any prior chunks left unenumerated procedures in xtask EXPECTED_PROCEDURES, chunk #59's xtask update should land in the same commit as the new `connection.current_state` procedure rather than as standalone housekeeping. This ensures the consuming chunk exercises the procedure via implementation + tests.

- **Run /andromeda-evolve --allow-arch-registry after chunk lands** — Per .claude/rules/security.md Session Additions 2026-05-09 first entry (chunk #23/#29 precedents): the new `connection.*` namespace and new `pulse://stream/connection-state` topic require subsequent acknowledgment in arch.md §Occupied Resources Tauri IPC routes via Type 6 amendment. session-handoff.md already notes this deferral pattern explicitly ("future /implement against chunk #59 will introduce both at code level, then evolve --allow-arch-registry Type 6 amendment will acknowledge in arch"). Pattern matches prior streams.* + telemetry.frontend.* + pulse:clipboard precedents.

- **tracing self-observation only** — Per CLAUDE.md §Critical Warnings + security plan §Cross-cutting Patterns Self-observation: any tracing of state transitions or receiver-task panics uses `tracing` ecosystem (event!, instrument). NEVER add an OTel SDK exporter that could be pointed at own `:4317`/`:4318`.

## Anti-patterns to avoid

1. **NEVER include raw panic payload string in `ReceiverFailed` state broadcast** — Rust panic messages can include `format!("…{:?}", user_input)` style content from unwrap sites in tonic/axum decode paths; per security plan Anti-Patterns §Logging row 1, OTLP attribute values may surface incidentally. Sanitize at the panic-hook → state-transition conversion site to a stable enum-payload one-liner (e.g., `ReceiverFailed { reason: ReceiverFailureReason }` where `ReceiverFailureReason` is a closed enum).

2. **NEVER register `connection.current_state` in only some of the 4 binding sites** — Per .claude/rules/security.md Session Additions 2026-05-12: omitting any of {router registration, capability JSON, EXPECTED_PROCEDURES, emit_taurpc_bindings test} produces silent runtime IPC rejection OR confusing "drifted, N missing" diagnoses where bindings.ts lags production. All 4 in the same commit.

3. **NEVER add a tokio::process::Command path for receiver-restart functionality in this chunk** — Per security plan Anti-Patterns §Code Patterns row 1: if the receiver panic-hook eventually triggers an automatic restart of the receiver task, the restart path MUST NOT pass user-controlled strings (env-var overrides, etc.) to a shell. (Defensive — likely N/A for in-process tokio::spawn restart, but flagged because "receiver-task panic-hook wired" + "ReceiverFailed" implies a recovery surface.)

## Contract bindings

- **Binds to tests domain (§CI Integration + capability-drift gate)** — The 4-place capability binding pattern (router + capability JSON + EXPECTED_PROCEDURES + emit_taurpc_bindings test) is enforced by `xtask capability-drift` in ci.yml. Tests domain extract owns the assertion coverage requirement for the `connection.current_state` procedure; security domain owns the binding-discipline checklist.

- **Binds to obs domain (§Logging redaction + heartbeat ticks)** — Heartbeat ticks mechanism (route chunk #9) emits `{module}.tick` events; the new connection-state poller is conceptually a heartbeat-adjacent loop. Security PII-scrubbing rules (chunk #8) apply to any tracing events emitted by the poller — obs domain owns event field schema, security domain owns the NEVER-log list enforcement.

- **Binds to arch domain (§Occupied Resources Tauri IPC routes + new broadcast topic)** — Post-implementation Type 6 arch-registry amendment will acknowledge `connection.current_state` TauRPC + `pulse://stream/connection-state` broadcast. Arch extract owns the registry-acknowledgment workflow; security extract owns the capability JSON entry requirement that must accompany.

## Acceptance criteria contributions

1. **(security) `pulse-app/capabilities/` JSON includes the new `connection` router (or extends the existing `pulse:default` permissions list as appropriate per chunk #30 precedent) AND `xtask/src/main.rs::EXPECTED_PROCEDURES` lists `connection.current_state` AND `emit_taurpc_bindings` test merges the new router's handler** — verified by `cargo xtask capability-drift` exiting 0.

2. **(security) `cargo deny check bans licenses sources` passes after any new dependency (if any) added for atomic Instant tracking or panic-hook wiring** — verified by ci.yml `dep-security-ci-gate` step; in particular the `tonic 0.14 ↔ 0.13` duplicate canary stays clear.

3. **(security) `ReceiverFailed` state payload + any `connection.current_state` error path contain NO stack traces / file paths / Rust struct names / library versions / OTLP attribute values** — verified by grep audit of the panic-hook conversion site + From<E> for AppError impls in the affected crate; reviewer checks the closed-enum shape of any failure-reason field.

4. **(security) OTLP receiver bind addresses on `:4317` and `:4318` remain `127.0.0.1` after LastIngestTracker insertion in ingest hot path** — verified by inspection of tonic Server::builder() + axum Router bind calls in `crates/ingest/`; no regression of Host-header allowlist / CORS deny / `.max_decoding_message_size` / `DefaultBodyLimit` / tower_governor layer ordering.

5. **(security) No `tokio::process::Command` introduced in receiver-restart path (if any)** — verified by grep `Command::new` in `crates/ingest/` diff for this chunk; defensive against MCP STDIO command-injection cluster patterns reaching the ingest crate.
