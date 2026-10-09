# obs extract

## Relevance
Partial — the worded status line + ConnectionDot phrasing are webview/design/a11y (out of obs); obs applies conditionally to the research-gated backend delta and via the data-source contracts (ingest-rate / buffer-fill / `health`|`ready` fields) this chunk consumes.

## Constraints
- Obs tier is **Standard**; the webview status line is a read/render surface — the frontend emits NO obs directly (no browser OTel SDK; frontend telemetry only ever crosses via the TauRPC `telemetry.frontend.*` bridge, which this chunk does not add). Do not add instrumentation to the render path per obs-plan §4 (Desktop-webview row) / §3 (Frontend bridge).
- IF research (P3) forces a backend delta to expose spans/s or buffer-fill, the new/extended TauRPC command must be span'd via `#[tracing::instrument(skip_all, fields(traceparent = %tp))]` with the IPC-envelope `traceparent` attached, child spans inheriting via `tracing::Span::current()` — per obs-plan §4 (IPC-internal row) / §3 (Trace context propagation → IPC boundaries).
- Any new backend log line emits NDJSON matching the binding schema (`timestamp`/`level`/`target`/`message`/`fields`, plus `service.name`/`service.version`/`deployment.environment` as default subscriber fields) — obs aligns to tests, not vice versa — per obs-plan §6 (Required fields) / §3 (Log format JSON schema, Service identity).
- If the buffer/rate delta issues a DuckDB query, log `query_id` + `param_count` only, NEVER raw query text/params (`viz` allowlist = `query_id`, `param_count`, `row_count`, `latency_ms`) — per obs-plan §8 (Vector 5, allowlist) / §6 (DuckDB must-log row).
- The connected-source data must never log raw OTLP attribute values or the raw service-name list — count + `service` tag only — per obs-plan §8 (Vector 1) / §11 (Spans).
- The three quantities already have obs producers to locate first (reuse-first): spans/s ← `metric.ingest.throughput_events_per_sec` (100ms tick) / `metric.buffer.ingest_throughput_spans_per_sec`; buffer-fill ← `metric.buffer.memory_bytes` gauge carrying `retention_window_seconds`, `rows_active`, `eviction_count` (15s `buffer.tick`) — per obs-plan §5 (per-surface metric table) / §1 (Heartbeat ticks).

## Patterns to follow
- `health`/`ready` TauRPC command exposes subsystem fields synchronously (`buffer_capacity_pct`, `broadcast_subscribers`, retention-window, `rows_ingested`) — the same fields heartbeat ticks emit to the JSON log; use the **sync command** for a live UI line, not log-tailing (obs-plan §3 health-complementarity paragraph, §1 Heartbeat ticks). This is the scope's flagged `ready` capacity/retention source.
- Metric-as-`tracing`-event convention: agent computes rates by tailing the JSON file; there are no precomputed rate instruments — any derived spans/s is a `value` field on a `metric.{module}.{measure}` event, not a new counter type (obs-plan §5 emission convention).
- Service identity rides default subscriber fields — new backend code adds no per-call identity boilerplate (obs-plan §3 Service identity).

## Anti-patterns to avoid
- NEVER log raw OTLP attribute values or the raw service list from `services.list_with_states` — count + `service` tag only (obs-plan §11 Spans/Logs, Vector 1).
- NEVER emit raw DuckDB query text/params if a buffer/rate query is added — `query_id` + `param_count` only (obs-plan §11 Spans, Vector 5).
- NEVER introduce an unbounded `service_name` label on a self-obs metric outside query-time aggregation (obs-plan §11 Metrics, §5 cardinality discipline).

## Contract bindings
- **obs ↔ tests:** the `health`/`ready` status/subsystem field shape binds to tests §3 — if this chunk extends `ready`/`health` to surface rate/buffer, the field shape is harness-asserted (obs-plan §3; focus-guide "Status endpoint shape").
- **obs ↔ tests:** the NDJSON log format is the binding contract (obs aligns to tests §5) — any new backend log line conforms (obs-plan §3/§6).
- **obs ↔ this chunk's data sources:** ingest-rate + buffer-fill quantities originate from obs-defined heartbeat/metric producers (obs-plan §1/§5) — reuse before proposing a new field.

## Acceptance criteria contributions
- (obs) IF a backend delta lands: new code logs NDJSON with required fields (`level`/`timestamp`/`target`/`message`/`fields` + `service.{name,version,environment}`) per §6.
- (obs) PII: no raw OTLP attribute values, no raw service-name list, no raw DuckDB query text/params appear in `agent-latest.jsonl` — count + `service` tag + `query_id`/`param_count` only (§8 Vectors 1, 5).
- (obs) IF a read-path command/query is added, it is span'd with `traceparent` propagated from the IPC envelope and adds no unbounded-cardinality metric label (§3/§4/§5).
- (obs) Webview-only verification must not FAIL obs check scripts for an absent metric stream — this chunk adds no new heartbeat/tick obligation (NEUTRAL-tolerant, two-state posture, §10).

## Relevant amendment history
- **2026-05-08 — Heartbeat ticks vs `health` command complementarity (→ §3):** clarifies the `health`/`ready` command (sync polling) and heartbeat ticks (async, JSON-log) are complementary, not redundant — directly governs the scope's `ready`/`health` source choice for a live worded line (read the sync command, don't tail ticks).
- **2026-06-10 — Chunk #99 load-suite findings (→ §10):** confirms `metric.buffer.memory_bytes` ≤512 MB with `retention_window_seconds`/`rows_active`/`eviction_count` as the canonical buffer-state telemetry (informs the buffer-fill data point) and establishes NEUTRAL-tolerant check scripts for webview-absent runs (relevant since this chunk is webview-primary).
