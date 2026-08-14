# obs extract

## Relevance
partial — frontend-primary re-poll of the already-instrumented `viz.query.traces` TauRPC query; no new spans/metrics to mint, but the now-recurring (~1s) query inherits viz scrubbing/log-level/cardinality constraints, and any new frontend state-transition signal must route through the obs bridge. Rust-side obs gates apply only if the conditional CARRY activates.

## Constraints
- `viz.query.traces` span attributes are fixed to `query_id`, `param_count`, `time_window_seconds` (obs-plan §4 P1); a re-poll reuses the existing instrumented handler and must not add new high-cardinality attributes (e.g. no per-mount/per-poll id).
- DuckDB query parameters — including the carried "Errors only" filter state and any service_filter — MUST be scrubbed to `query_id` + `param_count` only; the raw filter/params list is NEVER logged (obs-plan §8 PII table + §11 Spans, security Vector 5). viz allowlist is exactly `query_id, param_count, row_count, latency_ms` (obs-plan §8 default-deny whitelist).
- viz log level is `info` for query execution, `debug` for aggregation detail (obs-plan §6); because the re-poll makes the query recurring (~1s), per-query detail stays debug/trace-gated via `ANDROMEDA_PULSE_LOG_LEVEL` — do not emit a fresh info line every tick (obs-plan §11 Logs "NEVER log in hot path at info level"; §11 Telemetry "NEVER over-instrument hot paths").
- `metric.trace.latency_percentiles` fires once per `viz.query.traces` invocation and is query-time-only / not time-series-stored; its `service_name` label is intentionally unbounded but single-query-scoped (obs-plan §5 cardinality discipline + §11 Metrics exception). The recurring poll must honor that scope — reuse the existing metric, add no per-poll label.
- Any new frontend signal (empty→populated transition, empty-state counter, skeleton-pulse timing) MUST flow through TauRPC `telemetry.frontend.record_*` → backend `tracing::info!(target: "metric.{name}")`; NO browser OTel SDK / no direct webview file write (obs-plan §1 desktop-webview surface, §4 desktop-webview row, §11 Universal).
- If the CARRY activates (Rust `viz` `next_cursor` change): zero-unlogged-panics + `viz.tick` heartbeat (≤45s gap) invariants and the deferred CI obs gates come due (obs-plan §10).

## Patterns to follow
- Reuse the existing `#[tracing::instrument(skip_all, fields(traceparent = %tp))]` TauRPC viz handler (obs-plan §4 IPC-internal row); child context inherits via `tracing::Span::current()`, so re-polling needs no new span wiring.
- Query-anonymizer / prepared-statement pattern for the recurring query (obs-plan §8 Integration points; §11 Spans, Vector 5) — `query_id` + `param_count`, never `format!`-built SQL.
- Query-time metric emission as one `tracing::info!(target: "metric.trace.latency_percentiles", …)` per query, consumed immediately by the requesting view (obs-plan §5 / §1 creator-explicit-telemetry) — the recurring poll fits the "aggregate at query time, not stored" model already in place.
- Frontend telemetry bridge: component-lifecycle / empty-error-state counters → TauRPC `telemetry.frontend.record_*` → backend tracing event (obs-plan §3 Frontend bridge, §4 desktop-webview).

## Anti-patterns to avoid
- Info-level per-tick logging on the ~1s re-poll (obs-plan §11 Logs / §11 Telemetry Strategy) — a recurring query at info level is log spam; gate to debug/trace.
- Emitting the carried "Errors only" / service_filter string into span attributes or logs (obs-plan §11 Spans, Vector 5) — stays scrubbed to `param_count`.
- Adding any browser OTel SDK or direct webview log sink for refresh telemetry (obs-plan §11 Universal) — must go via the TauRPC bridge.

## Contract bindings
- obs ↔ tests harness: JSON log schema (obs-plan §3, verbatim from tests §5) — any log line the re-poll emits keeps the `timestamp/level/target/message/fields` shape + default `service.*`; `viz.tick` heartbeat-gap CI gate (`xtask/ci/heartbeat-gap-check.sh`, obs-plan §10) applies if Rust viz is touched.
- obs ↔ security: query-param scrubbing (Vector 5) binds to security §Logging — the "Errors only" filter is a query parameter (obs-plan §8).
- obs ↔ frontend/a11y: new empty-state telemetry binds through the TauRPC `telemetry.frontend.*` bridge (obs-plan §3); the UI-vocabulary exemption (obs-plan §8) means the "No traces yet" / "Errors only" label text is UI vocabulary, not a PII secret — grep-based PII gates must not false-positive on it.

## Acceptance criteria contributions
- (obs) Recurring `viz.query.traces` re-poll emits no raw query params/filter strings in `agent-latest.jsonl` — only `query_id` + `param_count` (Vector 5); verify by issuing an "Errors only"/service-filtered poll and confirming the filter string is absent from the log file.
- (obs) At default log level the ~1s re-poll adds no per-tick info-level line — per-query detail is debug-gated (obs-plan §6 + §11 Logs).
- (obs) Any new empty→populated frontend signal is emitted as a backend `metric.*` event via the TauRPC bridge, with no browser OTel SDK and no direct webview file write (obs-plan §3, §11 Universal).
- (obs, conditional) If CARRY activates (Rust viz): zero `app.panic.fatal` spans and `viz.tick` gap ≤45s across the test run (obs-plan §10 CI gates).

## Relevant amendment history
- 2026-05-04 — PII grep UI-vocabulary exemption (§8): clarified that grep-based PII gates must distinguish secret *formats* from UI-label words. Directly relevant here — the chunk's "No traces yet" empty-state and "Errors only" filter labels are UI vocabulary, not secrets, and must not trip PII heuristics.
- 2026-06-10 — DuckDB connection-isolation + NEUTRAL-tolerant check scripts (§10): "any future DuckDB consumer with a multi-second statement MUST take a dedicated `try_clone()` read connection" and check scripts must be NEUTRAL-tolerant (absent stream ≠ FAIL). Relevant to the recurring query load and to the conditional Rust viz CARRY (viz.query already runs on the isolated L1a read connection); headless verification of this frontend chunk should read as NEUTRAL, not FAIL, on absent frame/heartbeat streams.
