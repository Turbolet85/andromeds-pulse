# Observability Summary

_Distilled from `.andromeda/obs-plan.md` by `/setup-project`. Read on demand. **Tier: Standard (1)** — multi-surface trace propagation, perf-budget triggers, security-sensitive logging vectors._

## Architectural choice (load-bearing)
Self-observation runtime uses **ONLY** the `tracing` ecosystem — NO OTel SDK linked into the product binary for self-observation. Recursion-free by construction: the JSON file at `~/.andromeda-pulse/logs/agent-latest.jsonl` IS the agent-readable surface.

The product's external surfaces (OTLP receivers ingesting third-party clients) continue to use `tonic` + `axum` + `prost` + `opentelemetry-proto` — those crates parse OTLP wire format only and don't require an OTel SDK runtime.

## Tracing init (binding)
- **Crate set:** `tracing` 0.1 + `tracing-subscriber` 0.3 (JSON formatter + `EnvFilter`) + `tracing-appender` 0.2 (daily-rolling file sink, non-blocking writer) + `tracing-error` 0.2 (SpanTrace).
- **Init order at boot, before HTTP/gRPC bind:**
  1. Load env (`ANDROMEDA_PULSE_LOG_LEVEL` with fallback `RUST_LOG`)
  2. Build `tracing_subscriber::registry()` composing stderr + non-blocking file appender + `EnvFilter` + `ErrorLayer`
  3. Install `std::panic::set_hook` calling `tracing::error!(target: "app.panic.fatal", ...)` with SpanTrace
  4. Spawn Tauri + bind OTLP receivers

```rust
let (file_writer, _guard) = tracing_appender::non_blocking(rolling::daily(log_dir, "agent-latest.jsonl"));
tracing_subscriber::registry()
  .with(fmt::layer().json().with_writer(file_writer))
  .with(EnvFilter::from_env("ANDROMEDA_PULSE_LOG_LEVEL"))
  .with(ErrorLayer::default()).init();
std::panic::set_hook(Box::new(panic_to_tracing_error));
```

## Service identity (default subscriber fields)
- `service.name` = compile-time `"com.andromeda.pulse"` (Tauri bundle id); rmcp sidecar = `"andromeda-pulse-mcp"`
- `service.version` = `env!("CARGO_PKG_VERSION")`
- `deployment.environment` = `"production"` (hardcoded)

Registered once at subscriber init via `Layer::with_default_fields(...)` — every JSON line carries identity without per-call boilerplate.

## Log format (binding contract from tests §3)
```json
{
  "timestamp": "2026-05-02T16:18:34.567Z",
  "level": "INFO",
  "target": "ingest::grpc",
  "message": "TraceService.Export received 10 spans",
  "fields": {
    "span_count": 10,
    "service": "my-app"
  }
}
```
Optional fields: `trace_id`, `span_id` (W3C traceparent strings), `duration_ms`, `plugin_path_basename`, `query_id`, `param_count`, `token_count_actual`, `token_budget_limit`, `body_size_bytes`, `webview_backend`, `tray_api`, `wgpu_backend`, `dedup_count`, `anomaly_markers`, `p50_ms`/`p95_ms`/`p99_ms`/`max_ms`, `error_rate_percent`, `value`.

## Log sink
- **Path:** `~/.andromeda-pulse/logs/agent-latest.jsonl` (per-platform per arch §Filesystem locations).
- **Rotation:** daily via `tracing_appender::rolling::daily()`.
- **App dual sink:** stderr (JSON when not TTY, pretty when TTY) + file (always JSON).
- **rmcp sidecar:** stderr forced JSON (no TTY check); stdout reserved for JSON-RPC 2.0 framing — ANY accidental `println!` corrupts MCP protocol.

## Trace context propagation
W3C `traceparent` (HTTP) and gRPC `grpc-trace-bin` extracted at receiver entry; attached as a regular `tracing` field; downstream `tracing::Span::current()` inherits. IPC envelope (TauRPC) carries optional `traceparent`. Real-time push streams carry trace context in Arrow `_trace_context` metadata. **Treated as opaque string** — not bound to any OTel SDK trace context.

## Heartbeat ticks
- 15s interval for ingest/buffer/viz/plugins via `tokio::time::interval(Duration::from_secs(15))`.
- 100ms for realtime throughput counter (animation source).
- Format: `tracing::info!(target: "{module}.tick", span_count=N, buffer_capacity_pct=M, broadcast_subscribers=X, ...)`.
- **Stall threshold (liveness):** missing tick for >45s = stall signal. CI fails build via `xtask/ci/heartbeat-gap-check.sh`.
- **Stall threshold (progress):** tick presence is not progress. `buffer.tick` carries `rows_ingested_delta` + `last_append_age_seconds`; a 0 delta across consecutive ticks while the ingest channel holds queued work is a stalled consumer → `buffer.consumer.stalled` (after 30 ticks = 450s, above §10's 420s sustained-drain cap), asserted by `cargo xtask check:ingest-progress`. Added 2026-08-26 after a wedge froze `rows_ingested` for 16 minutes while passing every liveness check.
- **Heartbeat ticks vs TauRPC `health` command:** these are TWO DIFFERENT liveness mechanisms; both must remain. Ticks = asynchronous emission for retroactive analysis (was the subsystem alive during this window?); `health` IPC = synchronous probe for active liveness (used for boot readiness polling per `tests-summary.md` §Test harness contract — poll every 500ms up to 10s, plus runtime health checks). Implementations MUST NOT replace tick emission with `health`-only state, MUST NOT treat absence of tick as failure of `health` (or vice versa). No shared state between the two paths required. Per amendment `2026-05-08T17-28-28Z-cross-ref-heartbeat-vs-health`.

## Critical paths (P1–P7 — must-trace)
Every P1–P7 must emit parent + child spans with the `{module}.{operation}` naming convention. Trace context propagated end-to-end via traceparent field. See obs-plan §1 critical paths table for required spans + log fields per path.

## SLO invariants (CI gates)
| Invariant | Enforcement |
|---|---|
| **Zero unlogged panics** | `std::panic::set_hook` → `tracing::error!(target: "app.panic.fatal", ...)`; CI greps for `app.panic.fatal` spans (any match = build fail) |
| **Heartbeat ticks (>45s gap = stall)** | Post-test gap analysis: parse log timestamps per `{module}.tick` target, compute deltas, assert max ≤45000ms |
| **Drain progress (consumer wedge)** | `cargo xtask check:ingest-progress` — fails on a `buffer.consumer.stalled` record whose `reason` is not `recovered`; NEUTRAL when the stream carries no `buffer.tick`. The companion to the gap check, which the measured wedge passed throughout |
| **Snapshot p99 ≤500ms** | `metric.snapshot.token_count_ms` events; CI tail aggregation via `jq` |
| **WebGPU frame p99 ≤33ms (ms-form governs; ≈30 fps descriptive)** | `metric.webgpu.frame_duration_ms` events bridged from frontend via TauRPC; ACTIVE-verified session 183 (p99 27.3ms, n=56,642 real frames) |
| **Buffer memory ≤512MB** | `metric.buffer.memory_bytes` per heartbeat tick; chaos test (10k spans/sec for 15min) post-test max check |
| **Module-boundary error logging** | Every error at boundary logged at WARN/ERROR with trace context + error category (not full stack trace) |
| **Trace context propagation** | Cross-surface call propagates W3C traceparent / IPC envelope context |

### NEUTRAL/ACTIVE gate posture + connection isolation (chunk #99, per obs-plan §12 2026-06-10)
- The check scripts (`xtask/ci/{perf-slo-check,heartbeat-gap-check,l4-latency-p99}`) are **NEUTRAL-tolerant**: absent metric stream → NEUTRAL, not FAIL — one script set serves headless CI and booted-app (ACTIVE) sessions. `cargo xtask perf:load-profiles` time-windows collected logs to the current run (`target/load-profiles/agent-window.jsonl`); unscoped dev daily-rolled logs false-FAIL heartbeat on cross-session gaps.
- ACTIVE evidence flow: `scripts/agent-run boot` (or direct binary) → `cargo run -p ingest --example load_profiles -- custom <rate> <secs>` → stop app → run gate scripts via `pwsh` (5.1 misparses `l4-latency-p99.ps1`'s UTF-8 punctuation; `tracing-appender` holds the live log without read-share on Windows).
- **DuckDB connection isolation:** multi-second statements take a dedicated `Connection::try_clone()` — write (appender) / sweep (retention) / read (L1a + viz) topology; never hold the shared appender connection (three production defects found+fixed at 50k spans/s by the load suite). `viz`'s three query fns joined the isolated set 2026-08-26 via `viz::query::read_connection` — they had held the shared connection while the Traces surface re-polls on a timer, a violation of this invariant rather than a change to it.

## PII scrubbing (Vectors 1–6 from security plan)
| Vector | Rule |
|---|---|
| 1 | NEVER include raw OTLP attribute values in span attributes — use `attributes_count` + `service_name_tag` only. `#[instrument(skip(req))]` excludes payload at source. |
| 2 | NEVER serialize `anyhow::Error` directly across IPC — convert to `AppError` enum + sanitize. Use `tracing-error` SpanTrace (user-defined spans only, no Rust struct names). |
| 3 | NEVER log full canonicalized plugin file paths — basename only (`plugin_path_basename`). |
| 4 | NEVER log MCP tool response bodies — `result_type` + `result_count` metadata only. |
| 5 | NEVER log DuckDB query parameter values — `query_id` + `param_count` + `param_types: ["string","timestamp"]`. |
| 6 | NEVER log raw env-var path values — sanitize to canonicalized basename only. Secret env vars are identified by naming convention (`*_SECRET` / `*_TOKEN` / `*_KEY` / `*_PASSPHRASE` redacted): `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` and any key derived from it are never emitted as a value, a field, or a presence flag. |

**Cold-start-window override leaf (chunk 2026-08-16-baseline-family-reachability).** One EXACT `§8` leaf —
`triage.baseline.bootstrap_window.override` (`resolved_seconds` / `default_seconds` / `reason`), all three
fields the emit site emits, two integers plus a bounded static label. Registered in `§6`'s once-per-boot WARN
enumeration too (it fires only when the resolved bound ≠ the 3600s default, because an env-shortened window
silently changes when services become eligible for silence detection). Neither a bare `triage` nor a bare
`triage.baseline` key may exist. Its guard lives at `pulse-app/tests/unit_observability_allowlist_bootstrap_window.rs`

**Close-to-tray signpost leaf (chunk 2026-08-23-headful-leg-extension).** One EXACT `§8` leaf —
`tray.signpost.shown` (`window_label` ONLY, bounded via `sanitize_window_label`; never content, title, or
coordinates). No bare `tray` key may exist. Guard: `pulse-app/tests/unit_observability_allowlist_close_signpost.rs`
(mutation-checked RED 3/3 → GREEN).

**Boot navigation-check leaf (chunk 2026-08-24-headful-mechanics-probe-race-disposition).** One EXACT `§8`
leaf — `app.boot.window.navigation` (`window_label` / `navigated` / `reason`, ALL THREE the emit site emits;
label bounded via the same `sanitize_window_label`, `reason` a bounded static; never the webview URL, title,
or coordinates). NO bare `app` key is registered, so without the exact leaf the fallback resolves nothing and
every field is redacted. Registered at BOTH `§6`'s warn row and `§8` per the dual-site rule. Guard:
`pulse-app/tests/unit_observability_allowlist_window_navigation.rs` (4 tests, mutation-checked — leaf renamed
→ 3/4 RED, the fallback-leak pin correctly green).
— under `tests/` because `[lib] test = false` makes a src-level guard compile and never run.

**Incident-producer outcome leaf (chunk 2026-08-16-fault-identity-semantics-decided).** One EXACT `§8` leaf —
`interpretation.incident.created` (`created` / `deduped` bools + `severity` / `priority_tier` bounded labels — all
four the emit site emits). **No bare `interpretation` key may exist** (it would widen the muted
`interpretation.model.load` sibling) — **but one DOES exist today** at `pulse-app/src/observability.rs:1934`
(measured 2026-08-26), so the requirement stands while the code violates it; latent because each live
`interpretation.*` target carries its own exact leaf — and that held on its first test: chunk
`2026-08-26-l4-runtime-security-residuals` added `interpretation.model.allow_root` (`confinement`,
`root_basename`) WITH its own leaf and completed `interpretation.model.load.error` 4 → 6 fields
(`env_var`, `path_basename`), the latter having had ZERO producers until then. That chunk also
mutation-PROVED the mechanism: deleting the new exact leaf left `for_target` resolving via the bare key
while only the field-set pin failed — which is why a leaf guard must assert the field SET, never merely
that the target resolves. Owner: the same sweep entry. Registered AS MEASURED: the leaf had shipped in `AllowList::production()`
before §8 enumerated it, so this closed a doc gap rather than a production one. **Completeness is documented but
NOT enforced** — that guard sits in `observability.rs`'s dead `mod tests`; only the PII half runs
(`pulse-app/tests/unit_incident_producer.rs`). Guard migration is owned by the "Diagnostics un-muting +
harness-truth sweep" entry.

**Corpus allowlist leaves (chunk 2026-08-15-corpus-key-persistence).** Four EXACT `§8` leaves — `corpus.open.error` (`error_kind`; a REPAIR, it had emitted since chunk #68 with no resolvable entry so its field was redacted), `corpus.keychain.fallback` (`backend_kind` / `reason` / `consequence`), `corpus.read.undecryptable` (`query_id` / `rows_skipped`), `corpus.orphan.disposition` (`disposition_outcome` / `rows_purged` / `tables_affected` / `error_category`). **No bare `corpus` prefix key may exist** — it would silently widen every future `corpus.*` target to one field set. 
**Incident-path allowlist leaves (chunk 2026-08-15-tier-1-incident-path-investigation).** Three EXACT `§8` leaves, all previously redacted — `incidents.list_active.request` (`item_count`), `triage.incident.persist` (`incident_count` / `persist_kind` / `duration_ms` / `declined_count` — all FOUR: the emit site emits `duration_ms`, and `declined_count` joined at chunk 2026-08-27-incident-persist-vs-resolve-write-race counting stale writes the corpus monotonic guard declined, folded once per persist CYCLE; a leaf naming fewer leaves the target partly redacted), `triage.incident.corpus_restore` (`kind` / `restored_incident_count`). **No bare `incidents` or `triage` prefix key may exist** (same `for_target` trap). Guarded by `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs`. The muted-diagnostic backlog in obs-plan §8 now names **FOUR** remaining redacted targets (`metric.pipeline.l1a.query_count_total`, `metric.pipeline.l1a.query_latency_p99_milliseconds`, `triage.incident.auto_resolve.tick`, `interpretation.model.load`) — `triage.cue.tick` left it 2026-08-26 — owned by the "Diagnostics un-muting + harness-truth sweep" route entry. **The census's "resolve to no entry" mechanism holds for only ONE of the four** (measured 2026-08-26): `triage.incident.auto_resolve.tick` resolves to nothing; the two `metric.pipeline.l1a.*` fall back to the bare `metric` key keeping `value`; `interpretation.model.load` has an exact leaf missing only `inference_mode`. The repair is "complete the leaf", not "add a leaf".

**Cue-latch + cadence-rate leaves (chunk 2026-08-26-cadence-runaway-blocking-pool).** Two EXACT `§8` leaves. `triage.cue.tick` carries all NINE emitted fields — a leaf COMPLETION, since an exact leaf already named 5 of the 7 then emitted, leaving `cues_suppressed` / `bypass_triggered` redacted while their siblings rendered (the partial-redaction shape every gate passes). `cadence.tick` carries all EIGHT, registered now because `cycles_executed` — the cadence cycle RATE — was previously accumulated and discarded via `let _ =`, leaving the rate readable only by counting per-trigger records out of a 131 MB log. Both new counter families are tick-aggregated FIELDS on existing heartbeats (§5), never per-cue or per-trigger records: `cues_latched` / `latch_tracked` bound the cue→cadence amplifier at emission, which is where BOTH cadence entry points are fed. Guarded by `pulse-app/tests/unit_observability_allowlist_cue_tick.rs`, mutation-checked three ways — the narrowed-leaf arm is the instructive one: **the exact-resolve pin PASSES while the field-set pin fails**, so asserting `for_target(...).is_some()` cannot guard a partial leaf.

**Delegated-timing allowlist leaves (chunk 2026-08-21-delegated-timing-observables).** Three EXACT `§8` leaves for the timing bounds that END at a webview paint the backend cannot see, so `telemetry.frontend.*` is their only sanctioned route to the log — `metric.constellation.hue_update_ms` (`duration_ms` / `severity_tier`, bounded `HueSeverityTier` none|curious|suggested|autonomous — **P-025**, measuring the constellation DOT hue via `severityToHueFraction`, NOT a Halo canvas, which has no production render site), `metric.constellation.discovery_ms` (`duration_ms` / `discovered_count`, bound 10_000 — **P-027**; `service` is an OTLP resource attribute and may never be a label), `metric.findings.counter_refresh_ms` (`duration_ms` — **P-045**). **The guard here is the INVERSE of the corpus / incident-path families:** a bare `metric` key legitimately EXISTS (carrying only `value` / `unit` / `module`), so the rule is not "no bare prefix key" but that each target owns its exact leaf — without one, `for_target`'s first-`.`-segment fallback keeps `value` and silently redacts every label (the mechanism behind the still-open `metric.pipeline.l1a.*` muted backlog). Guarded by `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs`, whose fallback-discrimination test was mutation-checked. These three names are the coordinates the Conductor Epoch-4 return consumes.

> **Vectors 1-6 scope clarification (per obs-plan §12 Decisions Log entry 2026-05-04 "Clarify PII grep heuristic UI-vocabulary exemption"):** the vectors above forbid LEAKAGE of real OTLP attribute values, real plugin paths, real DuckDB query parameter values, MCP response bodies, clipboard contents, and updater URL query strings. They do NOT forbid the literal word "token" / "password" / "api_key" in UI-label documentation (e.g., "Token budget" Investigation modal slider per a11y-plan §3 P2; "API key field" / "Password reset" UX flow text). PII grep heuristics in CI tests (e.g., chunk #46 a11y CI gate + violation-JSON regression) MUST distinguish literal secret formats (`Bearer [a-zA-Z0-9]{40,}`, `password=[^\s]+`, `AKIA[0-9A-Z]{16}`, `sk-[a-zA-Z0-9]{40,}`, `ghp_[a-zA-Z0-9]{36}`, `xox[baprs]-[0-9a-zA-Z-]{10,}`) from UI-label terminology that legitimately contains secret-format keywords.

## Frontend bridge
`web-vitals` 5.x callbacks (LCP / CLS / INP / FCP / TTFB) + WebGPU frame timing → TauRPC `telemetry.frontend.record_*` → backend `tracing::info!(target: "metric.{name}", ...)`. NO browser OTel SDK linked. Single JSON file is the unified self-observation surface.

## Sentry (opt-in)
- `sentry-rust` 0.46 + `sentry-tauri` 0.5 — opt-in via `ANDROMEDA_PULSE_SENTRY_DSN`, default OFF.
- `before_send` scrubbing MANDATORY when enabled.
- Sentry uses its own SDK directly — NO OTel→Sentry bridge.
- Local panic hook + `tracing::error!` MUST always work even with Sentry disabled.

## Snapshot / paste-to-AI separation
- **External-OTLP snapshot** (user-invoked feature): `~/.andromeda-pulse/snapshots/{timestamp}.md` — curated markdown of EXTERNAL clients' OTLP data.
- **Self-observation paste-to-AI**: `~/.andromeda-pulse/logs/agent-latest.jsonl` — JSON log file IS the snapshot.
- The two surfaces NEVER intersect.

## Top anti-patterns (obs §11)
- NEVER link OTel SDK into self-observation runtime.
- NEVER write to stdout from any subsystem when running as rmcp stdio sidecar.
- NEVER hold a `tracing::Span` guard across `.await` without `.in_current_span()`.
- NEVER use unbounded label cardinality in event fields.
- NEVER log in hot path at `info` level — use `trace`/`debug` gated by env var.
- NEVER use proprietary APM as ONLY exporter — JSON file is the canonical agent surface.
- NEVER skip zero-unlogged-panics invariant.
- NEVER define soft SLO budgets — must trigger build/deploy failure.

Full plan: `.andromeda/obs-plan.md`.
