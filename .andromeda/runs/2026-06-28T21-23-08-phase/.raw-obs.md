# obs extract

## Relevance
Partial. The chunk modifies internal triage queue logic (digest coalescing, elastic queue), but adds observability for heartbeat ticks, queue depth, and coalesce counts — binding to existing heartbeat-tick contract and perf-budget SLOs.

## Constraints
1. Per obs-plan §3 + 2026-05-08 amendment: Heartbeat ticks are async, 10-30s cadence, emitted to JSON log via `tracing::info!(target: "{module}.tick", ...)`; stall detection >45s without tick. Complement (do not conflate) TauRPC `health` command (sync polling).
2. Per obs-plan §5 Metric Coverage: Emit counters/histograms as `tracing::info!(target: "metric.{name}", value = N, ...)` events; agent computes aggregates from JSON log tail; no OTel Meter.
3. Per obs-plan §2 Telemetry Strategy Naming: Span names follow `{module}.{operation}` pattern (e.g., `triage.tick`, `digest.coalesce.request`); avoid high-cardinality names.
4. Per chunk scope "aggregate-only self-observation": Metric cardinality disciplined — no per-fingerprint or per-service labels in triage metrics (reuse existing `pipeline.l3.*` family if applicable).
5. Per obs-plan §1 Standard tier: `#[tracing::instrument]` on critical digest/queue paths; trace context propagated via `tracing::Span::current()`.
6. Per obs-plan §10 (SLO Invariants) + 2026-06-10 amendment: L4 per-hardware-profile latency budgets are canonical; load profile characterized at 50k spans/s; append-path blocking >60s in-spec tolerance informs queue-depth metric thresholds.

## Patterns to follow
1. Per obs-plan §3 Observability Harness Contract + 2026-05-08 amendment: Heartbeat tick → async `tracing::info!` to JSON file (retroactive stall detection); decouple from sync `health` TauRPC command polling (active liveness).
2. Per obs-plan §2 Telemetry Strategy: Structured JSON-per-line via `tracing-subscriber` JSON formatter; daily rotation via `tracing-appender::rolling::daily()`.
3. Per chunk scope "observability for the new behavior": Emit `triage.tick` events with `queue_depth`, `coalesce_count`, `drain_latency_ms` fields; heartbeat drains queued digests toward L4 cadence.

## Anti-patterns to avoid
1. Per obs-plan §5 Metric Coverage: No unbounded metric labels (e.g., no `fingerprint_id=X` cardinality; aggregate-only per chunk scope discipline).
2. Per obs-plan §2 Naming: Avoid high-cardinality span naming (e.g., do NOT emit separate span per fingerprint; use single `digest.coalesce` span with `coalesce_count` field).
3. Per 2026-05-08 amendment: Do NOT emit heartbeat tick as synchronous TauRPC `health` call or embed tick inside `health` response logic — keep async log-write path separate.

## Contract bindings
- **Heartbeat ticks** ↔ tests (per obs-plan §3, 2026-05-08 amendment): Tests consume heartbeat ticks from `agent-latest.jsonl` log tail to detect subsystem stalls (>45s no tick).
- **Queue depth / coalesce metrics** ↔ integration acceptance test (per chunk scope "acceptance anchor"): Integration test drives ≥100 identical-fingerprint hard signals; asserts single incident + no `QueueAction::DroppedOldest` events logged.
- **L4 latency budget** ↔ SLO invariants (per obs-plan §10, 2026-06-10 amendment): Elastic queue + heartbeat drain compensate for canonical L4 ~4s per-hardware-profile latency; queue depth metric informs backpressure analysis.

## Acceptance criteria contributions
1. "(obs) Triage heartbeat tick emits `tracing::info!(target: 'triage.tick', queue_depth, coalesce_count_last_interval, drain_latency_ms)` every 10-30s; stall detection >45s without tick emits error-level event."
2. "(obs) Coalesce counter emits `tracing::info!(target: 'metric.triage.coalesce_count', value = N)` per digest assembly; integration test verifies storm yields coalesce_count ≈ N-1 (1 digest + N-1 absorbed by coalescing)."
3. "(obs) Elastic queue depth emitted as gauge via `tracing::info!(target: 'metric.triage.queue_depth', current = X, capacity = Y)` per tick; integration test asserts no `QueueAction::DroppedOldest` entries in JSON log during storm."
4. "(obs) Integration test asserts single incident output when ≥100 identical-fingerprint hard signals injected; verifies log contains exactly one `digest.*` span + zero cap-drop errors + coalesce metrics matching storm input count."

## Relevant amendment history
- **2026-05-08** — "Heartbeat ticks vs health command complementarity" (folded to obs-plan §3, §10): Clarified heartbeat ticks are async, 10-30s, emitted to JSON log (retroactive stall analysis >45s) vs TauRPC `health` (sync, active liveness polling). **Directly applicable:** This chunk adds `triage.tick` heartbeat; must emit async to log, not conflate with sync health polling.
- **2026-06-10** — "Chunk #99 tag gate: L4 budgets + load-suite findings" (folded to obs-plan §10): Confirmed L4 per-hardware budgets as canonical; DuckDB connection-isolation pattern; load profile: 420s cap, append-path blocking >60s in-spec. **Context:** Informs queue-depth metric thresholds and explains why elastic queue + drain mechanism is necessary under characterized L4 latency.
