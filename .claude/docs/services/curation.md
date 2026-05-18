# `curation` — Algorithmic Primitives (Dedup / Anomaly / Critical-Path / Aggregation)

## Responsibility
Algorithmic primitives extracted from `snapshot` crate (chunk #58 — Epoch 9 Foundation v0.2.0). Pure-function building blocks consumed by `snapshot` for L3 markdown rendering and by future L1a / L2 distillation chunks.

## Key integrations

### Consumes from
- DuckDB query results materialized as Arrow `RecordBatch` (via `buffer` query routers + viz queries).
- `curation::contract` types passed through from upstream callers.

### Publishes to
- `curation::contract` re-exports (the only `pub` surface — public contract module pattern per arch §Project Intent Template patterns).
- Consumed by `snapshot` for L3 markdown rendering pipeline.

### Dependencies
- Workspace-inherited: `serde`, `thiserror`, `tracing`. No external direct deps (algorithmic primitives are pure CPU work over Arrow data; no I/O).

## Internal conventions
- **Module layout:** `dedupe.rs` (logical span collapse via service+name+duration bucket) / `anomaly.rs` (latency outliers / error correlation / cardinality spikes) / `critical_path.rs` (longest-duration branch extraction) / `aggregation.rs` (p50/p95/p99/max per-service + global percentiles).
- **Pure-function discipline:** primitives take `&[Span]` / `&RecordBatch` references; return owned summaries. No mutation of inputs, no I/O.
- **Errors:** `curation::Error` (`thiserror`-derived); collapses to `AppError::Internal { message }` at IPC bridge if surfaced.
- **`contract` module:** the ONLY `pub` surface; internal modules are `pub(crate)`.

## Service-specific gotchas
- **Dedup bucket precision:** `(service, span_name, duration_ms_bucket)` — bucket size tuned to balance dedup yield vs information loss; do NOT widen without test plan §6 critical-path coverage.
- **Anomaly thresholds:** latency outlier = `p99 > 5× p50` over rolling window; error correlation = same-trace correlation factor; cardinality spike = `service_count * 3` deviation from baseline. Thresholds live in `curation::anomaly::Thresholds`.

## Entry points for modification
- **Public contract:** `crates/curation/src/contract.rs` (re-exports)
- **Per-primitive impl:** `crates/curation/src/{dedupe,anomaly,critical_path,aggregation}.rs`
- **Tests:** unit tests inline with each primitive; integration via `snapshot` end-to-end.
