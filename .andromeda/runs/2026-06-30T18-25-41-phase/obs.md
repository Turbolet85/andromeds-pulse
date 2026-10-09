# obs extract

## Relevance
partial — touches webview surface (obs-instrumentable per obs-plan §1.4) but chunk is UI implementation (suppression handler + CSS styling), not new instrumentation

## Constraints
1. Per obs §8 (PII Scrubbing) vectors 3 & 5: context-menu event handler must NOT log user-interaction content, keyboard input details, or parameter values
2. Per obs §2 (Telemetry Strategy) cardinality discipline: if handler emits any telemetry events, avoid unbounded label cardinality (e.g., don't emit per-click; batch or drop)
3. Per obs §10 (SLO Invariants) + 2026-06-10 amendment (frame-budget posture): verify suppression handler doesn't block or impact frame render timing (p99 ≤33ms invariant remains active)
4. Per obs §3 (Harness Contract) §6 (Log Coverage): if test harness captures logs from the DOM test execution, entries must follow JSON schema (timestamp / level / target / message / fields with service.{name,version,environment})

## Patterns to follow
1. Per obs §2 (naming conventions): if handler emits metric events, use `metric.{module}.{measure}` target prefix on `tracing` calls
2. Per obs §3 (logging stack): frontend events bridge through TauRPC commands to JSON sink; DOM-level test assertions follow standard vitest patterns (unlikely to emit backend telemetry)

## Anti-patterns to avoid
1. Logging raw contextmenu event properties, keyboard state, or parameter values in structured logs (obs §8 vectors 3 & 5 — same security scrubbing applied to ingest boundary)
2. Emitting high-frequency telemetry events inside DOM event handlers or frame-render loops (frame-budget cardinality concern per obs §10 + 2026-06-10 amendment)

## Contract bindings
test harness — if chunk's vitest DOM test captures stdout/stderr logs, those logs bind to Section 3 Harness Contract + Section 6 JSON schema; no new IPC/TauRPC expected (scope: no capability grants required for DOM event handlers)

## Acceptance criteria contributions
1. (obs) If logs emitted during test, they follow JSON schema verbatim (§6: timestamp, level, target, message, fields.{service.name, service.version, deployment.environment} as default fields)
2. (obs) No user interaction data (contextmenu event properties, key codes, click coordinates) leaked in logs; handler uses `preventDefault()` only, no event logging
3. (obs) Frame p99 latency remains ≤33ms post-implementation (per §10 SLO + 2026-06-10 amendment frame-budget posture — ACTIVE state when webview booted)

## Relevant amendment history
- **2026-06-10** § Frame-budget posture two-state: ACTIVE when webview boots (p99 frame ≤33ms, verified at 27.3ms over 56k real frames, no regression expected from event-handler suppression). Chunk should verify handler does not introduce blocking on frame timing.
- **2026-05-04** § PII grep UI-vocabulary exemption: if logs mention UI label "Token" (e.g., "Token budget" accessible name), distinguish from actual token secrets (literal "Token budget" label ≠ PII). Not directly relevant unless canvas/menu suppression logging references UI vocabulary.
