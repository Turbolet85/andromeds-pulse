---
paths:
  - "crates/**/src/**/*.rs"
  - "pulse-app/**/*.rs"
  - "xtask/**/*.rs"
  - "pulse-app/ui/**/*.{ts,tsx}"
---

# Observability Rules

Path-scoped rules for telemetry instrumentation across Rust crates + webview frontend bridges.

**Authoritative source:** `.andromeda/obs-plan.md` §3 (Observability Harness Contract) + §10 (SLO Invariants) + §11 (Anti-Patterns). Tier=Standard. **Architectural choice:** `tracing` ecosystem only — NO OTel SDK linked into the self-observation runtime (recursion-free by construction: the JSON file at `~/.andromeda-pulse/logs/agent-latest.jsonl` IS the agent surface).

## Self-observation runtime (binding)
- **Crates:** `tracing` 0.1 + `tracing-subscriber` 0.3 (JSON formatter + `EnvFilter`) + `tracing-appender` 0.2 (daily-rolling file sink, non-blocking writer) + `tracing-error` 0.2 (SpanTrace).
- **NO OTel SDK** in self-runtime. The product's external OTLP receivers (`tonic` + `axum` + `prost` + `opentelemetry-proto`) parse wire format only — they do not require an OTel SDK runtime.
- **NEVER** add `opentelemetry`, `opentelemetry_sdk`, or `opentelemetry-otlp` as a self-observation exporter. The recursion concern is resolved by construction.
- **Init order at boot, before HTTP/gRPC bind:** load env (`ANDROMEDA_PULSE_LOG_LEVEL` / `RUST_LOG` fallback) → build `tracing_subscriber::registry()` composing stderr + non-blocking file appender + `EnvFilter` + `ErrorLayer` → install `std::panic::set_hook` calling `tracing::error!(target: "app.panic.fatal", ...)` → spawn Tauri.

## Service identity (default subscriber fields)
- `service.name` = compile-time `"com.andromeda.pulse"` (Tauri bundle id); for the rmcp sidecar = `"andromeda-pulse-mcp"`.
- `service.version` = `env!("CARGO_PKG_VERSION")`.
- `deployment.environment` = `"production"` (hardcoded).
- Register once at subscriber init via `Layer::with_default_fields([service_name, service_version, deployment_environment])` — every JSON line carries identity without per-call boilerplate.

## Log format (binding contract from tests §3)
JSON-per-line via `tracing_subscriber::fmt::Layer::json()`:
```json
{"timestamp":"…","level":"INFO","target":"ingest::grpc","message":"…","fields":{"span_count":N,"service":"…"}}
```
Optional fields: `trace_id` / `span_id` (W3C traceparent strings), `duration_ms`, `plugin_path_basename`, `query_id`, `param_count`, `token_count_actual`, `token_budget_limit`, `body_size_bytes`, `webview_backend`, `tray_api`, `wgpu_backend`, `dedup_count`, `anomaly_markers`, `p50_ms`/`p95_ms`/`p99_ms`/`max_ms`, `error_rate_percent`, `value`.

## Log sink
- **Path:** `~/.andromeda-pulse/logs/agent-latest.jsonl` (per-platform per arch §Occupied Resources Filesystem locations).
- **Rotation:** daily via `tracing_appender::rolling::daily(log_dir, "agent-latest.jsonl")`.
- **Dual sink (app):** stderr (JSON when not a TTY, pretty-printed when TTY) + file (always JSON).
- **rmcp sidecar:** stderr forced JSON (no TTY check) — stdout reserved for JSON-RPC 2.0 framing. ANY accidental `println!` / `dbg!` / library stdout write corrupts MCP protocol and silently disconnects the client.

## Span discipline
- Span naming: `{module}.{operation}` (e.g., `ingest.grpc.export.request`, `duckdb.append`, `snapshot.generate.request`, `plugin.invoke.request`). Avoid high-cardinality names (no per-trace-ID names).
- `#[tracing::instrument(skip(req), fields(rpc.system = "grpc", rpc.service, rpc.method, span_count, traceparent))]` on tonic gRPC handlers.
- `#[tracing::instrument(skip_all, fields(traceparent = %tp))]` on TauRPC routers.
- `#[tracing::instrument(skip(input, output))]` on `wasmtime::component::Instance::call` host wrappers.
- NEVER hold a `tracing::Span` guard across `.await` without `.in_current_span()` — span context bleeds into unrelated tokio tasks, breaking trace_id correlation and bloating memory.
- Manual `tower::Layer` (or `tower-http::TraceLayer` for plain `tracing` spans, NOT OTel-flavored) on the axum router.

## Trace context propagation
- W3C `traceparent` header (HTTP) and gRPC `grpc-trace-bin` metadata extracted at receiver entry; attached as a regular `tracing` field (`fields(traceparent = %tp)`); downstream `tracing::Span::current()` inherits.
- IPC envelope (TauRPC command struct) carries optional `traceparent` field; receiver attaches to local span.
- Real-time push streams (`pulse://stream/spans` etc.) carry trace context in Arrow metadata column `_trace_context` for end-to-end correlation.
- `tokio::spawn` calls use `.in_current_span()` to propagate.

## PII scrubbing (Vectors 1–6)
- NEVER log raw OTLP attribute values — use `attributes_count` + `service_name_tag` only. `#[instrument(skip(req))]` excludes the payload at the source.
- NEVER log full canonicalized plugin file paths — basename only (`plugin_path_basename: "my-plugin.wasm"`).
- NEVER log DuckDB query parameter values — emit `query_id` + `param_count` + `param_types: ["string", "timestamp"]`.
- NEVER log MCP tool response bodies — emit `result_type` + `result_count` only.
- NEVER log clipboard contents (`snapshot.copy_to_clipboard` MUST emit a non-suppressible "X bytes copied" toast event instead).
- NEVER log Tauri updater download URL query strings (could carry tokens in a future scope).
- NEVER log multi-line stack traces — use `tracing-error` SpanTrace (one-line serialization, user-defined spans only, no Rust struct names).

## Heartbeat ticks (stall detection)
- Long-running subsystems emit `tracing::info!(target: "{module}.tick", ...)` every 15s via `tokio::time::interval(Duration::from_secs(15))`: `ingest.tick`, `buffer.tick`, `viz.tick`, `plugins.tick`. Exception: realtime throughput counter ticks at 100ms (animation source).
- Required tick fields per module: `span_count` / `buffer_capacity_pct` / `broadcast_subscribers` / `rows_ingested` / `retention_window_active` / `eviction_count` / `loaded_count` / `active_invocations`.
- Stall threshold: missing tick for >45s = stall signal. CI fails build if any tick gap >45s during test run.

## Metrics convention (`metric.*` target prefix)
- `tracing::info!(target: "metric.{module}.{measure}", value = N, ...)` — agent computes percentiles by tailing the JSON log; offline regression via `criterion` 0.5 in `xtask benches/`.
- Required perf-budget metrics: `metric.snapshot.token_count_ms` (p99 ≤500ms), `metric.webgpu.frame_duration_ms` (p99 ≤33ms / 30 fps), `metric.buffer.memory_bytes` (≤512MB for 10-min retention), `metric.buffer.ingest_throughput_spans_per_sec`, `metric.trace.latency_percentiles`.
- NEVER use unbounded label cardinality in event fields (per-trace-ID, arbitrary request paths) — explodes `jq` aggregation cost. Exception: query-time aggregation events emit unbounded `service_name` per single-query scope.
- NEVER format expensive payloads inside hot paths without level-gating: wrap in `if tracing::enabled!(Level::DEBUG) { ... }`.

## SLO invariants (CI gates)
- **Zero unlogged panics:** every `panic!()` captured by `std::panic::set_hook` → `tracing::error!(target: "app.panic.fatal", ...)` with SpanTrace. CI greps for `app.panic.fatal` spans; ANY match = build fail.
- **Heartbeat-stall detection:** post-test gap analysis fails build if any `{module}.tick` gap >45s. Script in `xtask/ci/heartbeat-gap-check.sh`.
- **Perf budget enforcement:** `xtask test` post-run aggregates `metric.snapshot.token_count_ms` p99 via `jq`; >500ms = build fail. `metric.webgpu.frame_duration_ms` p99 >33ms = release build fail.
- **Buffer memory bounded:** chaos test (10k spans/sec for 15 min) post-run tail `metric.buffer.memory_bytes` max; >512_000_000 = build fail.
- **Build fails if `xtask test` produces zero spans in log file** — indicates instrumentation missing.

## Frontend bridge
- `web-vitals` 5.x callbacks (LCP / CLS / INP / FCP / TTFB) → TauRPC `telemetry.frontend.record_web_vital(name, value)` → backend `tracing::info!(target: "metric.web_vital.{name}", ...)`.
- WebGPU frame timing: DOM `performance.now()` + `device.queue.onSubmittedWorkDone()` → TauRPC `telemetry.frontend.record_frame_ms(duration_ms, wgpu_backend)` → backend `tracing::info!(target: "metric.webgpu.frame_duration_ms", ...)`.
- NO browser OTel SDK. Frontend has NO direct file access — all telemetry routes through TauRPC → backend tracing.

## Sentry (opt-in only)
- `sentry-rust` 0.46 + `sentry-tauri` 0.5 — opt-in via `ANDROMEDA_PULSE_SENTRY_DSN` env var, default OFF. Privacy-first per creator brief.
- NEVER ship a real Sentry DSN in dev / committed to repo.
- `before_send` scrubbing MANDATORY when enabled — strip OTLP attribute values + paths + plugin IDs.
- Sentry uses its own SDK directly — NO OTel→Sentry bridge.
- Local panic hook + `tracing::error!` MUST always work even with Sentry disabled.

## Snapshot / paste-to-AI separation
- External-OTLP snapshot (the user-invoked feature) writes to `~/.andromeda-pulse/snapshots/{timestamp}.md` — curated markdown of EXTERNAL clients' OTLP data.
- Self-observation paste-to-AI is `~/.andromeda-pulse/logs/agent-latest.jsonl` — the JSON log file IS the snapshot (greppable / `jq`-able / paste-to-LLM-able).
- The two surfaces NEVER intersect.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._

- 2026-05-03: tracing-subscriber 0.3 architectural constraints when implementing the obs-plan §3 binding contract in self-observation runtime: (a) `Layer<S>` is a read-only side-effect callback — it CANNOT mutate `Event` fields for downstream layers, so per-module field redaction (per obs-plan §8 default-deny scrubbing posture) MUST integrate into the `FormatEvent` formatter's field visitor, not as a separate `Layer<S>` impl in registry composition. (b) `tracing_subscriber::fmt::Layer` does NOT expose a native `with_default_fields([service.name, service.version, deployment.environment, ci.run.id, git.commit.sha])` method — the obs-plan body sketch references it conceptually, but injecting per-event identity fields requires a custom `FormatEvent` impl. (c) Pragmatic resolution: combine both concerns into one custom `FormatEvent` that walks event fields with a redacting visitor (`tracing::field::Visit`) and prepends default identity fields keyed off compile-time constants + `GITHUB_RUN_ID` / `GITHUB_SHA` env reads (length+charset bounded per security plan §Input Validation). Registry composition stays `EnvFilter -> fmt::layer().event_format(JsonWithDefaults).with_writer(non_blocking) -> ErrorLayer::default()`. The "subscriber-layer redaction before file write" intent of obs-plan §8 + the "default-fields on every line" intent of obs-plan §3 are both satisfied, just integrated into the formatter rather than spread across separate Layer impls. See `pulse-app/src/observability.rs` `JsonWithDefaults` + `JsonFieldVisitor` for the canonical implementation; `tracing_subscriber::fmt::FmtContext` (note: at `fmt::FmtContext`, NOT `fmt::format::FmtContext` — the latter is private).

- 2026-05-03: The `pulse-app/src/observability.rs::AllowList::production()` registry has BOTH a `plugin` (singular) entry — for `plugin.invoke.request` / `plugin.{operation}` events covering plugin-host invocation telemetry — AND a `plugins` (plural) entry — for `plugins.tick` heartbeat events from the workspace-crate-named `plugins` library. These are SEPARATE registry keys and they MUST coexist. Why: the `for_target` strip-suffix `.tick` lookup transforms `plugins.tick` → `plugins` (the plural workspace crate name from arch §Occupied Resources), NOT to `plugin` (singular). Adding heartbeat fields to the `plugin` (singular) entry silently leaks `plugins.tick` payload through the default-deny redaction path, because `for_target("plugins.tick")` resolves the `plugins` key (not present → fallback) → returns `None` → all fields redacted. Conversely, adding `plugin.{operation}` invocation fields to the `plugins` (plural) entry breaks the singular invocation telemetry. When extending allowlists for new tick or invocation events, identify the workspace crate name as it appears in arch §Occupied Resources Cargo workspace crate names (LOCKED list: `ingest`, `buffer`, `viz`, `ui-bridge`, `snapshot`, `workspace-detector`, `plugins`, `mcp-server`) and use it verbatim; do not abbreviate / pluralize / singularize.
