# `triage` — L1 Streaming Distillation Scaffold

## Responsibility
L1 streaming distillation layer (chunk #60 scaffold; chunks #61+ fill the implementations). Algorithmic detection of attention cues + restart events + retry storms + service lifecycle states from streaming span observations. Capabilities P-019 (Three-Tier Severity Model) + P-021 (Algorithmic Attention Cues) + P-015/P-016/P-057 (Restart Event + Suppression) + P-017/P-018 (Exception Fingerprint + Retry Storm) + P-027 (Service Constellation Auto-Discovery).

## Key integrations

### Consumes from
- `ingest::observer::SpanObserver` trait via `buffer::run_consumer` 5th parameter — receives every decoded OTLP span post-buffer-append.
- `buffer::fingerprint::FingerprintObserver` for exception fingerprint stream (chunk #66 retry-storm detector).
- `corpus::CorpusReader` for baseline-state bootstrap on cold start + persistence on tick (chunks #61 / #64 / #66 / etc. write through corpus).

### Publishes to
- `pulse://stream/attention-cues` broadcast topic (`triage::cue::AttentionCueBroadcast`; chunk #62)
- `pulse://stream/restart-events` broadcast topic (`triage::pattern::RestartEventBroadcast`; chunk #63)
- `pulse://stream/service-lifecycle` broadcast topic (`triage::lifecycle::ServiceLifecycleBroadcast`; chunk #67)
- `tracing` events at `triage.{baseline,cue,pattern,lifecycle}.*` targets (registered in `pulse-app/src/observability.rs::AllowList`).

### Dependencies
- Workspace-inherited: `tdigest` (streaming percentiles per chunk #61), `dashmap` (per-service tracker map), `bincode` (corpus persist serialization), `tokio` (broadcast + sleep loops), `tracing`, `thiserror`, `serde`, `specta` (optional via `taurpc-runtime` feature for TauRPC envelope types).
- Internal: `crates/security` (PII scrubber consumed at cue emission boundary, deferred to chunk #70+).

## Internal conventions
- **Module layout:** 7 sub-modules per chunk #60 scaffold:
  - `baseline/` — `BaselineState` + `ServiceBaseline` + `OperationBaseline` per-service streaming trackers (chunk #61); `ActivityFloor` 24h rolling histogram (chunk #64)
  - `pattern/` — `RestartDetector` (chunk #63), `RetryStormDetector` + `storm.rs` (chunk #66), `suppression.rs` (P-057 magnitude bypass — chunk #63)
  - `cue/` — `evaluate_thresholds` + `classify_priority` + `dual_condition_bypass` + emitter loop (chunk #62); `evaluate_service_went_silent` gate (chunk #64)
  - `digest/` — empty skeleton (future chunks)
  - `interpretation/` — empty skeleton (future chunks)
  - `incident/` — empty skeleton (future chunks)
  - `lifecycle/` — Service registry + 7-state FSM (chunk #67)
- **`contract` module:** the ONLY `pub` surface; 10+ contract types (`AttentionCue`, `CueKind`, `CueScope`, `PriorityTier`, `Severity`, `Incident`, `IncidentStatus`, `EvidenceRefs`, `Digest`, `DigestKind`, plus chunk-specific exports).
- **Cross-crate state delivery:** `Arc<dyn Trait>` injection at the binary boundary per session-learnings 2026-05-16 trait-in-lower-crate pattern (e.g., `Arc<dyn ServiceRegistry>` threaded through `start_lifecycle_heartbeat`).
- **Aggregate-only tracing:** per `.claude/rules/observability.md` Session Additions 2026-05-17 chunk #62-#64 precedent — `triage.*` emit aggregate counters, NOT per-service field tags (PII / cardinality discipline).

## Service-specific gotchas
- **`#[cfg(feature = "taurpc-runtime")]` gate on `specta::Type` derives** — chunk #67 added taurpc-runtime feature to triage crate for `ServiceLifecycleState` / `ServiceLifecycleEvent` etc.; the derive only fires when pulse-app enables the feature flag.
- **Service cap enforcement** — `ACTIVITY_FLOOR_SERVICE_CAP = 1024` (chunk #64); evicted services emit `triage.baseline.service_cap_exceeded` aggregate counter, not per-service.
- **Heartbeat loops are tokio::spawn** — chunks #61 (persist) / #62 (cue emitter) / #63 (restart detector) / #67 (lifecycle FSM) each spawn a long-running async task. Coordinate cadence per Cadence Coordinator design (chunk #72 future scope).

## Entry points for modification
- **Public contract:** `crates/triage/src/contract.rs`
- **Per-module impl:** `crates/triage/src/{baseline,pattern,cue,digest,interpretation,incident,lifecycle}/`
- **Boot wiring:** `pulse-app/src/main.rs` (constructs broadcasts, spawns heartbeats, threads Arc<dyn Trait> into resolvers)
- **Resolvers:** `pulse-app/src/{services_router,storm_observer,baseline_observer,restart_observer}.rs` (chunk-specific TauRPC routers + observer adapters)
- **Tests:** unit tests inline per sub-module; integration via boot-smoke + cue emission scenarios.
