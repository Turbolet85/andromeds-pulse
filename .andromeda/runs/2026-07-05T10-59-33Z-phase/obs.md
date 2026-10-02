# obs extract

## Relevance
Partial — frontend/webview presentation chunk; obs binds lightly via don't-log-surfaced-OTLP-content (Vector 1), conditional query-param scrubbing (Vector 5) IF a viz-query field is threaded, NDJSON log format, and the UI-vocabulary PII exemption. The bulk of the obs plan (ingest/buffer/plugins/MCP/snapshot/panic/heartbeat) is out-of-scope for this chunk.

## Constraints
- The error/anomaly rows this chunk surfaces are EXTERNAL OTLP data (service names, error messages, span attributes) — displaying them in the UI is fine, but this content must NEVER enter the self-observation JSON log; log only counts/tags, not row content (per obs-plan §8 Vector 1 / §11 Logs).
- IF the chunk threads a viz-query field/filter (error-count/status), the `viz.query.traces` boundary logs `query_id` + `param_count` (+ `param_types`) only — never the raw filter predicate or `service_filter` string; prepared statements, never `format!` SQL (per obs-plan §8 Vector 5 / §6 boundary-call wrappers / §4 P1 scenario).
- The error signal the chunk surfaces already exists at query time: `metric.trace.error_count` (`service_name`, `error_type` enum, `error_count`) and `metric.trace.latency_percentiles` (`error_rate_percent`) are emitted per `viz.query.traces` invocation — consume the existing signal; add no new detection/metric (per obs-plan §5 per-path metrics / §1 creator-explicit-telemetry "Error rate sparkline + count"; aligns with the chunk's "no change to detection logic" non-goal).
- Any new log line on this path is valid NDJSON with required fields `timestamp`/`level`/`target`/`message`/`fields` plus default `service.{name,version,environment}`; JSON-per-line to the file sink, never unstructured stderr text (per obs-plan §6 required fields / §3 log format).
- If the query-aggregation error metric is touched, keep cardinality disciplined: `service_name` stays query-time-only unbounded (not time-series-stored); no per-row-ID / per-trace-ID labels (per obs-plan §5 cardinality discipline / §11 Metrics).

## Patterns to follow
- Consume the row's existing error/status field (preferred per scope) — it maps to the query-time `error_count`/`error_rate_percent` aggregation already documented (obs-plan §5 per-path metric events; §1 P1 `viz.query.traces`).
- Frontend telemetry only via the TauRPC bridge — if any frontend counter is added (e.g. error-state count), route `telemetry.frontend.record_*` → backend `tracing::info!(target: "metric.{module}.{measure}", ...)`; no browser OTel SDK (obs-plan §3 frontend bridge / §4 desktop-webview row).
- Query anonymizer: `query_id` + `param_count` + `param_types`, redact raw params at source via `#[instrument(skip(...))]` + `fields(...)` allowlist (obs-plan §8 integration points / §5 Vector 5).
- Default subscriber fields carry service identity automatically — no per-call `service.*` boilerplate (obs-plan §3 service identity).

## Anti-patterns to avoid
- NEVER log raw OTLP attribute values / surfaced row content into `agent-latest.jsonl` (Vector 1) — the sorted/flagged/filtered rows are external client data (obs-plan §11 Logs / §8 Vector 1).
- NEVER emit raw DuckDB query text or filter predicate in span attributes (Vector 5) — `query_id` + `param_count` only (obs-plan §11 Spans/Traces / §8 Vector 5).
- NEVER over-instrument the render/sort/filter path — table re-sort/re-render can be frequent; keep spans off tight loops and level-gate any heavy field formatting (obs-plan §11 Telemetry Strategy + Metrics).

## Contract bindings
- **obs ↔ tests:** the NDJSON log schema is a binding contract (obs aligns to tests §5); any new log line from a viz-field addition must match. The webview val-1 test runs against the WebGPU dashboard — frame-budget CI check scripts are NEUTRAL-tolerant when headless (obs-plan §10 two-state posture).
- **obs ↔ a11y:** the chunk's non-color-only semantic error token + contrast (SC 1.4.1) is a11y/design-system governed; a11y-emitted violations ride the obs NDJSON structured-log format (focus-guide binding).
- **obs ↔ security (PII):** Vector 1/5 redaction applied at logger config; the "errors only" filter label + "error" badge/label text are UI vocabulary, exempt from PII grep heuristics (obs-plan §8 UI-vocabulary exemption).

## Acceptance criteria contributions
- (obs) No raw OTLP row content (service names, error messages, span attributes) from the surfaced/sorted/filtered rows appears in `agent-latest.jsonl` (§8 Vector 1).
- (obs) IF a viz-query field/filter is added: logs carry `query_id` + `param_count` only — the raw errors-only predicate / `service_filter` string does NOT appear in any log line (§8 Vector 5).
- (obs) Any new log line emitted on this path is valid NDJSON with required fields + `service.{name,version,environment}` defaults (§6).
- (obs) No new unbounded metric labels (no per-row-ID / per-trace-ID); if the query-aggregation error metric is touched, `service_name` stays query-time-only, not time-series-stored (§5 cardinality discipline).

## Relevant amendment history
- **2026-05-04 — UI-vocabulary exemption (folded to §8):** directly on point. This chunk introduces an "errors only" / "anomalies only" filter label and a semantic error token bearing "error" badge/label text. Per this amendment, PII grep heuristics (e.g. the a11y CI gate) must distinguish secret *formats* (regex envelopes) from UI-label terminology, so the new error/filter labels must NOT trip false positives. Why it exists: chunk #14 phase #11 obs PII grep false-positived on the "Token budget" screen-reader label — the same false-positive class this chunk's error labels could trigger.
- **2026-06-10 — Frame-budget two-state posture (folded to §10):** partially relevant background. The Traces table lives in the WebGPU dashboard; the frame p99 ≤ 33ms check reports NEUTRAL when the webview is absent (headless verification) and ACTIVE only when a booted app measures real frames, so CI check scripts stay NEUTRAL-tolerant. Why it exists: chunk #99 tag-gate load-suite verification established the two-state posture. (The DuckDB connection-isolation portion of that amendment does not apply — this chunk adds no multi-second query.)
