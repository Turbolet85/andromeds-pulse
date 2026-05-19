# Pulse v0.2.0 Implementation Route

**Status:** Foundational baseline for v0.2.0 implementation kickoff
**Last revised:** 2026-05-20
**Reference documents:**
- `pulse-capability-spec.md` v2 (product contract, 60 capabilities)
- `pulse-distillation-architecture.md` v3 (data pipeline design)
- `widget-state-validation-mini-route.md` v1 (original continuation route, superseded by this document)
- `widget-state-validation-report-2026-05-14.md` (validation context at start)

**Pipeline status at start:** route §2 56/56 complete (commit `de35e82`, session 65). This document defines chunks #57 onwards.

**Approach:** evolve-driven chunk appends (Type 7 route-append), no `/andromeda-scope-arch` ceremony. Sequential by default; parallel-safe noted explicitly.

**Target:** v0.2.0 ship — Pulse as AI-native ambient observability companion with local LLM interpretation, full corpus persistence, three-surface communication, and MCP integration.

---

## How to use this document

Each chunk is sized for a single `/andromeda-evolve --allow-route-append` invocation. Order is dependency-driven — do not skip ahead unless deps are explicitly noted as parallel-safe.

For each chunk, the following fields appear:

- **Depends on** — verify all listed chunks are committed before starting
- **Capabilities enabled** — P-XXX claims from `pulse-capability-spec.md` v2 that this chunk enables (full coverage requires multiple chunks per capability typically)
- **Distillation layer** — which L0–L6 layer from `pulse-distillation-architecture.md` v3 this chunk implements
- **Crates touched** — Rust workspace members affected
- **TauRPC delta** — new procedures (counted for bindings.ts regeneration discipline)
- **Broadcast topics delta** — new internal channels
- **Workspace deps delta** — new external crate dependencies
- **Arch registry delta** — entries to add to `arch.md` §Occupied Resources or §Established Decisions
- **Specialist plan touches** — bundle these plan updates into the same evolve where possible
- **Summary** — what the chunk does and why

After each chunk's implement + wrap-session, verify no new rot warnings before moving to next chunk.

---

## Blocking decisions before implementation start

These are not chunks per se — they are decisions that must be resolved before specific dependent chunks can start. Each has owner and deadline.

### Pre-D1 — LLM runtime choice (mistralrs vs candle)

**Why blocking:** Chunks #82-#85 (L4 LLM interpretation layer) depend on chosen runtime's API patterns, error types, and dependency tree.

**Decision criteria:**
- Maturity at implementation time
- Quantization format support (GGUF essential)
- Backend coverage (Metal critical for M-series Mac, CUDA for NVIDIA)
- JSON-constrained generation support (essential)
- Tokenizer integration ergonomics
- Memory management quality

**Decision deadline:** Before chunk #82.

**Owner:** Project lead.

**Output:** Decision recorded in arch §Established Decisions with rationale.

### Pre-D2 — Drain Rust spike

**Why blocking:** Drain LOC estimate (1000-1500) is the largest single component in route. Validating the estimate via spike before committing to chunk #67 implementation is prudent.

**Spike scope:** Minimal Drain prototype in Rust covering: fixed-depth parse tree, template extraction, similarity matching, basic masking config. Test against subset of LogHub corpus. Measure actual LOC and template assignment quality.

**Decision criteria:**
- Actual LOC vs 1000-1500 estimate
- Template quality on real log corpora
- Performance (per-event microseconds, lookup amortization)

**Spike output:** Confirmation or revised estimate with rationale. Spike code is throwaway; production implementation in #67 follows.

**Decision deadline:** Before chunk #67.

**Owner:** Project lead.

---

## Pre-flight evolves (BEFORE chunk #57)

Two pre-existing rot warnings from session 65 may need resolution if they haven't naturally resolved through earlier chunk work. Per Andromeda philosophy, these typically resolve as chunks touch affected sections, so explicit pre-flight evolves are likely unnecessary. Check at chunk #57 start:

### Pre-E1 — Arch stale `opentelemetry-stdout` reference

If still present at #57: `/andromeda-evolve` → target `arch.md` §Cross-cutting Patterns → propagate `tracing-subscriber` JSON pivot.

### Pre-E2 — Frame p99 threshold reconciliation

If still present at #57: `/andromeda-evolve` → target `tests-plan.md` §10 → align with obs-plan §10 `≤33ms` (= ≥30 fps).

If naturally resolved through other work, skip.

---

## Phase 0 — Widget data binding + curation foundation

### #57 — Widget real-data binding

> **Critical first chunk.** Until this lands, all subsequent visual work renders synthetic data.

- **Depends on:** nothing
- **Capabilities enabled:** prerequisite for P-024 through P-030 (widget surfaces require real data)
- **Distillation layer:** L0 consumer (widget reads existing L0 stream subscriptions)
- **Crates touched:** `pulse-app/ui/` (frontend)
- **TauRPC delta:** consume existing `streams.subscribe_metrics`
- **Broadcast topics delta:** none (consumer-side wiring only)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (E2E coverage for real-data path)
- **Summary:** Delete `use-synthetic-widget-metrics.ts`, wire compact widget's `CompactWidget.tsx` + `FooterBand.tsx` to real `streams.subscribe_metrics`. Halo continues to consume the old `errorRate / throughputHz` shape for now (refactored in #81). This unblocks all downstream visual work from synthetic data.

### #58 — Curation crate extraction

> Refactor — extracts existing snapshot primitives into shared lib. Parallel-safe with #57.

- **Depends on:** nothing
- **Capabilities enabled:** prerequisite infrastructure for P-010, P-012, P-018 (detection logic needs reusable primitives)
- **Distillation layer:** foundational (used across L1a and L2)
- **Crates touched:** NEW `crates/curation/`, `crates/snapshot/` becomes consumer
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** +1 crate (`curation`)
- **Specialist plan touches:** arch (occupied resources), test-plan (move snapshot anomaly tests to curation crate)
- **Summary:** Create `crates/curation/`, move `dedupe`, `anomaly` (latency outliers / error correlation / cardinality spikes), `critical_path`, `aggregation` modules from snapshot. Pub-ify primitives via `curation::contract` re-exports. Snapshot crate's external surface unchanged.

---

## Phase 1 — Connection awareness

### #59 — Connection state machine

- **Depends on:** nothing (parallel-safe with #57, #58)
- **Capabilities enabled:** P-001 (Receiver Lifecycle State), P-002 (Last-Span-Ago), P-003 (Receiver Failure Surface), P-004 (Orthogonal Health Domains)
- **Distillation layer:** L1b (state tracker, hot-path updated)
- **Crates touched:** `crates/ingest/` (new module inside)
- **TauRPC delta:** +1 procedure `connection.current_state()`
- **Broadcast topics delta:** +1 `pulse://stream/connection-state`
- **Workspace deps delta:** none
- **Arch registry delta:** +1 broadcast topic, +1 TauRPC procedure
- **Specialist plan touches:** arch (reconcile with existing `health` IPC — document orthogonality: health=subsystem state, connection=data flow), test-plan (state transition coverage), obs-plan (heartbeat tick for connection module)
- **Summary:** `LastIngestTracker` atomic Instant updated in ingest hot path. Background poller (1-2s tick) emits state changes (`Listening` / `Receiving` / `Idle` / `Stalled` / `ReceiverFailed`) to broadcast. Receiver-task panic path connects to state machine via existing panic hook.

---

## Phase 2 — Algorithmic detection layer

This phase establishes the L1 streaming distillation foundation. Capabilities here produce attention cues (P-021) consumed by later L3/L4 work — but until L3/L4 land, no user-visible incidents are surfaced from this phase. That is intentional: this phase builds the detection floor, surfaces come later.

### #60 — Triage crate scaffold + attention cue contract types

- **Depends on:** nothing (pure infrastructure, parallel-safe with prior phases)
- **Capabilities enabled:** prerequisite for P-021 (Algorithmic Attention Cues), P-019 (Three-Tier Severity Model — contract types)
- **Distillation layer:** L2 (foundation), L4 (output types)
- **Crates touched:** NEW `crates/triage/`
- **TauRPC delta:** none yet
- **Broadcast topics delta:** none yet
- **Workspace deps delta:** none
- **Arch registry delta:** +1 crate (`triage`), first registered scope "pulse-v0_2_0-route" in §Existing Scopes
- **Specialist plan touches:** arch (occupied resources + scope registration), security (sketch redaction discipline for future Incident.title / .detail content)
- **Summary:** Create `crates/triage/` with module skeletons (`baseline`, `pattern`, `cue`, `digest`, `interpretation`, `incident`, `lifecycle`). Define contract types in `triage::contract`: `AttentionCue` (with kind, scope, magnitude, absolute_value, persistence, confidence, priority_tier, suppression_bypassed fields per P-057), `CueKind`, `CueScope`, `PriorityTier`, `Severity`, `Incident`, `IncidentStatus`, `EvidenceRefs`, `Digest`, `DigestKind`. Empty implementations.

### #61 — Streaming baseline trackers + corpus persistence

- **Depends on:** #58 (curation), #60 (triage scaffold)
- **Capabilities enabled:** P-009 (Per-Service Error Rate Baseline including state persistence), P-011 (Per-Operation Latency Baseline)
- **Distillation layer:** L1b
- **Crates touched:** `crates/triage/baseline`
- **TauRPC delta:** none yet
- **Broadcast topics delta:** none yet
- **Workspace deps delta:** +`tdigest` (MnO2's `tdigest` 0.x), +`dashmap`, +`bincode` (for state serialization)
- **Arch registry delta:** workspace deps (deny.toml review for `multiple-versions = "deny"` posture)
- **Specialist plan touches:** test-plan (synthetic span stream fixtures, EWMA convergence assertions, t-digest accuracy assertions, **property-based tests via `proptest` per dist-arch v3 testability strategy**, state round-trip via serialization), obs-plan (`metric.baseline.ewma_short_window_size`, `pipeline.l1b.persist_count_total`, `pipeline.l1b.bootstrap_count_total{kind}`), security (no PII in baseline state)
- **Summary:** `EwmaTracker` per service (alpha calibrated 5-min effective window), per-operation t-digest pair (current + previous, swap on tick) for streaming p50/p95/p99, per-service `RollingWindow<u32>` for activity tracking. **State persistence:** flush to corpus every 60s and on graceful shutdown; load on startup with adjusted alpha if state age < 1 hour, else fresh bootstrap. Per-service state ≤ 3KB combined. Service identity: drop spans with empty `service.name`, emit aggregate `tracing::warn!(target: "triage.service_id_missing")` per tick.

### #62 — Attention cue emitter

> Renamed from "Baseline emitter + anomaly broadcast" to align with capability spec P-021 paradigm — these are attention cues for model interpretation, not severity-deciding anomalies.

- **Depends on:** #61
- **Capabilities enabled:** P-010 (Error Rate Spike Detection), P-012 (Latency Regression Detection), P-021 (Algorithmic Attention Cues)
- **Distillation layer:** L2
- **Crates touched:** `crates/triage/cue`
- **TauRPC delta:** none yet
- **Broadcast topics delta:** +1 `pulse://stream/attention-cues`, +1 `pulse://stream/cadence-triggers`
- **Workspace deps delta:** none
- **Arch registry delta:** +2 broadcast topics
- **Specialist plan touches:** arch, obs-plan (`pipeline.l2.cues_emitted_total{kind, tier}`, `pipeline.l2.magnitude_bypass_triggered_total{reason}`), test-plan (synthetic threshold violations produce expected AttentionCue emissions; tier classification correctness)
- **Summary:** Background tick task (1-2s) reads all trackers, evaluates thresholds (3.0× error rate multiplier, 2.5× latency multiplier — calibration values loaded from config), emits `AttentionCue` to broadcast with `priority_tier` classification (Hard / Medium / Baseline based on confidence and magnitude). Tier-2 cues additionally emit to `cadence-triggers` channel for Cadence Coordinator (#80). **Threshold multipliers loaded from config or hardcoded defaults for now; hot-reload wiring added in #94.**

### #63 — Restart event detector + dual-condition bypass

> Behavioral note: suppression must be surgical, not blanket. Dual-condition bypass (P-057) ensures real catastrophic regressions surface even during restart windows.

- **Depends on:** #60 (triage scaffold), #62 (cue emitter — bypass overrides its suppression)
- **Capabilities enabled:** P-015 (Restart Event Detection), P-016 (Restart-Window Suppression Surgical), P-057 (Dual-Condition Suppression Bypass)
- **Distillation layer:** L1b (RestartDetector) + L2 (suppression rules in cue emitter)
- **Crates touched:** `crates/triage/pattern`, `crates/triage/cue` (suppression logic)
- **TauRPC delta:** none yet
- **Broadcast topics delta:** +1 `pulse://stream/restart-events`
- **Workspace deps delta:** none
- **Arch registry delta:** +1 broadcast topic
- **Specialist plan touches:** arch, test-plan (synthetic stream gap → restart detection assertion; dual-condition bypass coverage: 8×/3%, 12×/4%, 6×/7% scenarios), obs-plan (`pipeline.l2.magnitude_bypass_triggered_total{reason: relative | absolute}`)
- **Summary:** Per-service last-seen span timestamp tracker, gap > 20s followed by resume → emit `RestartEvent`. Cue emitter (#62) subscribes to this topic and **suppresses ONLY `ErrorRateSpike` with `persistence < 30s`** within 60s suppression window. **Bypass conditions (P-057):** if `magnitude > 10× baseline` OR `absolute_error_rate > 5%`, override suppression. Both thresholds configurable via `[triage.suppression.magnitude_bypass_multiplier]` and `[triage.suppression.absolute_bypass_threshold]`.

### #64 — Activity floor learning + corpus persistence

> Solves "developer-on-lunch" false positive class plus eliminates per-restart cold-start blindness.

- **Depends on:** #61, #69 (corpus scaffold for persistence — see ordering note below)
- **Capabilities enabled:** P-013 (Service Activity Floor Learning including persistence), P-014 (Service Went Silent Detection)
- **Distillation layer:** L1b
- **Crates touched:** `crates/triage/baseline`
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (bursty pattern → ServiceWentSilent NOT emitted during learned quiet; histogram round-trip via serialization; **property-based tests** for bucket transitions and 24h rolling invariant), obs-plan (histogram bootstrap state metric)
- **Summary:** Per-service activity histogram (24h rolling, bucketed by 5-min intervals, 288 buckets), `ServiceWentSilent` emission gated by "current quiet duration > 95th percentile of historical quiet durations for this service". Initial histogram bootstrap window: 1 hour (during which `ServiceWentSilent` emission is universally suppressed for cold-start). **Persistence:** flush to corpus every 60s and on graceful shutdown, load on startup with state-age check identical to #61.

> **Ordering note:** #64 depends on #69 for corpus persistence. If #69 not yet ready, #64 can land with in-memory-only state initially, persistence wired in subsequent evolve. Practical sequence: do #69 before #64.

### #65 — Span events ingestion

> Prerequisite for retry storm detection. May be deferred if retry storm dropped from v0.2.0 scope.

- **Depends on:** nothing
- **Capabilities enabled:** P-006 (Exception Event Capture)
- **Distillation layer:** L0 (schema population) + L1c (decode in ingestion path)
- **Crates touched:** `crates/buffer/appender.rs`
- **TauRPC delta:** none (storage-side only)
- **Broadcast topics delta:** none (consumed via existing span broadcast)
- **Workspace deps delta:** none
- **Arch registry delta:** none (`span_events` table already in schema)
- **Specialist plan touches:** test-plan (span events round-trip assertion), security (`exception.message` content redaction discipline)
- **Summary:** Extend OTLP decode in `appender.rs` to populate `span_events` table. Decode `exception.type`, `exception.message`, `exception.stacktrace` attributes. Maintain existing redaction layer for exception.message content. Adds `span_events.fingerprint` column populated by #66.

### #66 — Exception fingerprinting + retry storm detector

- **Depends on:** #65, #60
- **Capabilities enabled:** P-017 (Exception Fingerprinting), P-018 (Retry Storm Detection)
- **Distillation layer:** L1c (fingerprint computation) + L2 (storm detection)
- **Crates touched:** `crates/triage/pattern`, `crates/buffer/appender.rs` (fingerprint write)
- **TauRPC delta:** none yet
- **Broadcast topics delta:** none (uses existing attention-cues channel)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (identical type+stack → identical fingerprint; same type different paths → identical fingerprint via normalization; different line numbers → different fingerprints; ≥5 occurrences → Suggested cue; ≥10 → Autonomous cue hint), security (fingerprint hash discipline — must not leak content)
- **Summary:** `ExceptionFingerprint = hash(exception.type + normalized first 3 stack frames)`. Normalization strips paths, addresses, line numbers. Fingerprints written to `span_events.fingerprint` column in ingestion hot path. `DashMap<ExceptionFingerprint, RecentOccurrences>` with 60s window. ≥5 occurrences per 30s → emit `RetryStorm` AttentionCue with Suggested severity hint, ≥10 → Autonomous hint, final severity determined by model interpretation per P-020.

---

## Phase 3 — Log template mining

### #67 — Drain Rust implementation + template profiling diagnostics

> **Two-phase chunk:** Phase A (Drain spike validation) MUST complete before Phase B (production implementation). Largest single component in route — spike resolves LOC estimate uncertainty before committing to multi-session production work.

- **Depends on:** #65 (logs table schema)
- **Capabilities enabled:** P-007 (High-Severity Log Capture — template-aware aggregation enables Q6)
- **Distillation layer:** L1c (Drain assignment) + L6 (template profiling surface)
- **Crates touched:** `crates/buffer/drain.rs` (NEW), `crates/buffer/appender.rs` (assignment integration), `pulse-app/ui/diagnostics/TemplateDistribution.tsx` (NEW)
- **TauRPC delta:** +1 procedure `diagnostics.template_distribution()` returns top-N templates with sample messages
- **Broadcast topics delta:** none
- **Workspace deps delta:** maybe `regex` if not already present (used for masking patterns)
- **Arch registry delta:** +1 TauRPC procedure, new schema table `log_templates`, +1 column `logs.template_id`
- **Specialist plan touches:** arch (schema additions), test-plan (golden-file tests with LogHub corpus subset, parameter sensitivity tests for depth/similarity), obs-plan (`pipeline.l1c.drain_template_count_total`, `pipeline.l1c.drain_assignment_latency_p99_microseconds`), security (template content goes through PII scrubber)

#### Phase A — Drain spike validation

**Type:** Research/spike sub-phase. Deliverable: decision document + LOC validation. No production code commits to main.

**Acceptance criteria:**
- Minimal Drain prototype in Rust implements: fixed-depth parse tree (depth=4), similarity matching (threshold=0.5), basic masking config (numbers, hex addresses, paths), template extraction with parameter substitution
- Prototype tested against LogHub corpus subset (Apache web logs + Linux syslog + HDFS application logs recommended)
- Measurements captured: actual LOC count of prototype, template assignment quality (% match rate per corpus), per-event latency p99 in microseconds, template tree memory footprint
- Findings document committed at `.andromeda/decisions/pre-d2-drain-spike.md` with one of four decisions: PROCEED (estimate confirmed), REVISE (estimate adjusted with rationale), SPLIT (chunk #67 should split into sub-chunks), DEFER (Drain pushed to v0.3.0+)
- Spike code lives in throwaway branch or `crates/triage-experimental/` (gitignored); does NOT commit to main

**Scope boundary:** Phase A does NOT implement persistence, max_clusters cap with LRU eviction, custom masking via regex config, performance optimization beyond observation, or production-quality tests. These are explicitly Phase B scope.

**Gate:** Phase B does not start until Phase A findings document exists with PROCEED, REVISE, or SPLIT decision. If decision is DEFER, chunk #67 closes here and v0.2.0 ships without Drain (capability P-007 documented as v0.3.0+ deferred).

#### Phase B — Drain production implementation 

**Type:** Production implementation. Deliverable: full Drain Rust port matching Drain3 Python parity including persistence, masking, edge cases.

**Acceptance criteria:**
- Rust port of Drain3 algorithm: fixed-depth parse tree, similarity threshold matching, template extraction with parameter masking, persistence (template tree serialization to/from storage), max_clusters cap with LRU eviction, custom masking via `[triage.drain.masking_patterns]` config
- Template assignment integrated into ingestion hot path at `crates/buffer/appender.rs` between OTLP decode and DuckDB append; per-event latency target <50μs p99
- `log_templates` table schema landed; `logs.template_id` column populated for all log records
- TauRPC `diagnostics.template_distribution()` returns top-N templates with sample messages, occurrence counts, drift indicators
- Settings → Diagnostics "Template Distribution" panel displays top-50 templates with sample messages, occurrence counts; users can identify pathological clustering (over-generalization like `[<:STAR:>]`, under-clustering with thousands of single-occurrence templates)
- Drain params (`[triage.drain.depth]`, `[triage.drain.similarity]`, `[triage.drain.max_clusters]`) loaded from config with safe defaults (depth=4, similarity=0.5, max_clusters=1000)
- Drain params require pulse restart to apply (template tree invalidation) per P-055; restart-required notice surfaces in Diagnostics when config changed
- Golden-file tests against LogHub corpus subset validate stable template assignments across refactoring; parameter sensitivity tests for depth/similarity verify quality degradation/recovery curves
- Template content passes through existing PII scrubber per P-047 before persistence

**Summary:** Rust port of Drain3 algorithm. Spike-validated LOC estimate (Phase A determines actual count vs initial 1000-1500 estimate). Settings → Diagnostics exposes "Template Distribution" panel for tuning. Drain params require restart per P-055 (template tree invalidation). All template content respects PII scrubbing per P-047.

#### Risk notes

- **Pre-D2 spike resolves estimate uncertainty.** Initial estimate 1000-1500 LOC based on Drain3 Python parity. Spike may confirm, revise upward (split chunk #67), or revise downward (single chunk faster than expected).
- **Largest single chunk in route.** Even confirmed estimate means 4-6 sessions multi-session implementation. Plan for sustained attention; don't interleave с unrelated work mid-implementation.
- **Cognitive context switch from Phase 2/3.** Drain is log-template-mining algorithm, conceptually distinct from baseline trackers / attention cues / corpus persistence work. Allow first session of Phase B for re-orientation if Phase A spike was earlier session.
- **Schema migration discipline.** `log_templates` table + `logs.template_id` column are first new schema additions since chunk #67 started; verify migration test covers existing-data scenario (logs ingested before chunk #67 lands should tolerate NULL template_id).

## Phase 4 — Service identity lifecycle

### #68 — Service registry + lifecycle state machine

- **Depends on:** #61 (baseline trackers — feed activity state), #63 (restart detector — feeds transitions)
- **Capabilities enabled:** P-027 (Service Constellation Auto-Discovery — formal lifecycle), prerequisite for P-014, P-068 lifecycle-aware behavior
- **Distillation layer:** L1b (state machine), L5 (constellation surface)
- **Crates touched:** `crates/triage/lifecycle`, `crates/triage/baseline` (state queries)
- **TauRPC delta:** +1 procedure `services.list_with_states()`
- **Broadcast topics delta:** +1 `pulse://stream/service-lifecycle` (state transitions)
- **Workspace deps delta:** none
- **Arch registry delta:** +1 broadcast topic, +1 TauRPC procedure, new schema table `service_registry`
- **Specialist plan touches:** arch, test-plan (state transitions: Unknown→Bootstrapping→Active→Quiet→Silent→Dormant→Archived; corpus history lookup on Archived→Active transition; manual override paths), obs-plan (`pipeline.l1b.tracked_services_total`, lifecycle state distribution metric)
- **Summary:** Service state machine per dist-arch v3 §Service Identity Lifecycle. Seven states with configurable boundaries via `[triage.lifecycle.dormant_after_secs]` (default 3600) and `[triage.lifecycle.archived_after_secs]` (default 86400). Edge case handling for ring buffer eviction (corpus lookup on long-silent services returning). Manual override in Settings → Services to mark Dormant/Archived.

---

## Phase 5 — Corpus foundation

### #69 — Corpus SQLite scaffold + schema + encryption + PII scrubber

> Foundational chunk. Many subsequent chunks depend on corpus availability.

- **Depends on:** nothing (parallel-safe with prior phases, but practically wait until algorithmic layer is producing data worth persisting)
- **Capabilities enabled:** P-041 (Persistent Incident Corpus — scaffold), P-047 (PII Scrubbing at Ingestion), P-048 (No Raw OTLP Attribute Values Stored), P-049 (Encryption at Rest), P-050 (Cross-Project Sharing Opt-In default), P-051 (Transparent Storage)
- **Distillation layer:** L5 persistence foundation, L1c PII scrubbing
- **Crates touched:** NEW `crates/corpus/`, `crates/security/` (PII scrubber consolidation if needed)
- **TauRPC delta:** +1 procedure `storage.inspect()` (read-only corpus query), +1 `storage.path()` (returns corpus location)
- **Broadcast topics delta:** none
- **Workspace deps delta:** +`rusqlite` with bundled feature, +`age` or similar for keychain integration on macOS/Linux/Windows
- **Arch registry delta:** +1 crate (`corpus`), +2 TauRPC procedures, new schema tables `baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive`
- **Specialist plan touches:** arch (occupied resources), security (encryption key management, keychain interaction, PII pattern catalog), test-plan (encryption round-trip, scrubber pattern coverage, corpus migration on first launch)
- **Summary:** Create `crates/corpus/` with SQLite-based persistent storage. Schema per dist-arch v3 §Schema Additions. Encryption at rest using OS keychain key (macOS Keychain, Linux Secret Service, Windows DPAPI) or user passphrase fallback. First-run setup prompts for keychain access. PII scrubber consolidated as security crate primitive consumed by corpus on ingestion. Settings → Storage section enables inspection (read-only), export trigger (per P-093 — implemented in #93), and deletion with explicit confirmation.

---

## Phase 6 — Consolidation

Mid-stream consolidation inserted in v3 per `pulse-v0_2_0-consolidation-audit-2026-05-19.md` (7 HIGH-severity inconsistency clusters in chunks #57-#69). Closes persistence triple-mechanism (BaselineState flat-file vs corpus SQLite vs pure-in-memory), PII scrubber single-site coverage gap, capability spec numeric drift, architecture registry incompleteness, doc cross-reference drift, and recurring Type 6 cascade-staleness pattern before downstream user-facing surfaces (Phase 7+) build on consolidated foundation. Eight session-scoped chunks; default ordering #70→#77 reflects dependency chain (persistence first, PII coverage of new corpus sites, capability alignment, then META cleanup).

### #70 — BaselineState → corpus migration

- **Depends on:** #61 (streaming baseline trackers: EwmaTracker / TDigestPair / RollingWindow), #64 (ActivityFloor 288-bucket histogram, part of BaselineState), #68 (corpus crate scaffold: SQLite + AES-256-GCM cell encryption + OS keychain key custody), #69 (`CorpusWriter` trait + `DrainPersistence` reference pattern — trait-in-lower-crate + adapter-at-pulse-app-boundary)
- **Capabilities affected (spec-compliance closure):** P-009 (close "Baseline state SHALL be persisted to corpus across application restarts" — currently persists to flat-file `<data_dir>/triage/baseline-corpus.bin` plaintext bincode at `crates/triage/src/baseline/corpus.rs:45-73`); P-011 (per-operation latency baseline — inherits BaselineState persistence path); P-013 (close "Histogram state SHALL be persisted ... via corpus persistence" — spec states twice); P-051 (transparent storage — `storage.inspect()` at `pulse-app/src/storage_router.rs:49` currently reads only corpus tables; baseline-corpus.bin invisible to Settings panel; after migration becomes visible)
- **Distillation layer:** L1b (statistical anomaly baselines)
- **Crates touched:** `crates/triage/src/baseline/` (replace `corpus.rs` flat-file `persist_state` + `load_state` with trait calls; add `trait BaselinePersistence` mirroring `buffer::drain::DrainPersistence`), `crates/triage/src/contract.rs` (export new trait), `crates/corpus/src/contract.rs` (no changes if reusing `CorpusWriter::save_pipeline_metric` flat blob slot like Drain; alternative: extend with `save_baseline_state` if dedicated table layout chosen), `crates/corpus/src/schema.rs` (decision: reuse `pipeline_metrics` row with `metric_name="baseline_state"` + `layer="l1b"` OR populate existing empty `baseline_state` table at lines 35-42 with per-service rows), `pulse-app/src/baseline_persistence.rs` (NEW — `CorpusBaselinePersistence` struct over `Arc<dyn CorpusWriter>` mirroring `pulse-app/src/drain_persistence.rs:42-81`), `pulse-app/src/main.rs` (boot: wire BaselinePersistence trait; remove `resolve_corpus_path` flat-file path resolution at `crates/triage/src/baseline/corpus.rs:32-43`; emit migration warn-log if existing baseline-corpus.bin file found on disk)
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none (corpus already workspace member since chunk #68; triage already transitive depends)
- **Arch registry delta:** if dedicated table populated rather than reused pipeline_metrics: schema unchanged (table exists). Removal of `triage/baseline-corpus.bin` subpath relevant for #74 batch (subpath was implicitly forward-promised but never registered per audit Dim 4).
- **Specialist plan touches:** arch (§Established Decisions [Telemetry Retention Surface] note: baseline state rides corpus), security (baseline state now inherits corpus AES-256-GCM encryption + PII scrub-before-encrypt contract — couples with #72; security-plan §Data Protection §At rest gains baseline-state row via #77 re-run), test-plan (replace existing bincode round-trip test at `crates/triage/src/baseline/corpus.rs:299-317` with corpus round-trip restart-and-reload test; preserve proptest coverage on streaming math), obs (preserve existing `triage.baseline.persist` info event + `pipeline.l1b.persist_count_total` counter; emit `triage.baseline.persist.error` with sanitized reason field for corpus write failures)
- **Summary:** Eliminate the BaselineState flat-file persistence mechanism (`<data_dir>/triage/baseline-corpus.bin` plaintext bincode) by routing all baseline trackers — `EwmaTracker` (error rate per service), `TDigestPair` (latency percentiles per operation), `RollingWindow<u32>` (recent error counts), `ActivityFloor` (288-bucket activity histogram per service) — through the corpus crate via a new `trait BaselinePersistence` defined in `crates/triage/src/baseline/` (lower-crate pattern per session-learnings 2026-05-16). Implement `CorpusBaselinePersistence` adapter in `pulse-app/src/baseline_persistence.rs` over `Arc<dyn CorpusWriter>`, mirroring chunk #69 `CorpusDrainPersistence`. Replace flat-file `persist_state`/`load_state` calls in `crates/triage/src/baseline/corpus.rs` with trait dispatch. Delete `<data_dir>/triage/baseline-corpus.bin` path resolution code. Schema decision (resolve during chunk planning): reuse `pipeline_metrics` table with `metric_name="baseline_state"` + `layer="l1b"` as flat AES-encrypted bincode blob (simplest, mirrors Drain) OR populate existing empty `baseline_state` table with per-service row layout (more queryable, requires schema interpretation). Migration: on boot, if legacy `baseline-corpus.bin` exists, read-and-migrate-once + delete with warn-log; OR document acceptable cold-start. Closes audit Section 1.A persistence triple-mechanism for 3 of 4 in-flat-file components.

### #71 — ServiceRegistry + RetryStormState → corpus migration

- **Depends on:** #66 (RetryStormDetector + `storm.rs` deferred-to-#69 docstring), #67 (ServiceRegistry + lifecycle state machine + `service_registry` corpus table pre-allocated at `crates/corpus/src/schema.rs:44-53`), #68 (corpus crate scaffold), #70 (BaselineState corpus migration sets the `*Persistence` adapter pattern)
- **Capabilities affected (spec-compliance closure):** P-027 (Service Constellation Auto-Discovery — close "Restart Pulse; verify dot positions match prior session"; currently `InMemoryServiceRegistry` at `crates/triage/src/lifecycle/registry.rs:113-116` is pure-in-memory; `TransitionTrigger::CorpusRestore` enum variant at `state_machine.rs:47` unreachable; `set_state_on_corpus_restore` method referenced in `registry.rs:7-11` docstring does not exist as code); P-017 (Exception fingerprinting — storm state continuity benefit); P-018 (Retry storm detection — close explicit broken promise; storm.rs docstring at lines 38-39 + 94-95 states "Persistence deferred к chunk #69 corpus scaffold"; chunk #69 closed without wiring)
- **Distillation layer:** L1b (service identity lifecycle) + L1c (storm fingerprint dedup)
- **Crates touched:** `crates/triage/src/lifecycle/registry.rs` (extend with corpus-backed save/load; implement `set_state_on_corpus_restore` method per docstring at lines 7-11; `trait LifecyclePersistence` in `crates/triage/src/lifecycle/mod.rs`), `crates/triage/src/lifecycle/state_machine.rs` (make `TransitionTrigger::CorpusRestore` reachable; emit synthetic restore events on boot), `crates/triage/src/pattern/storm.rs` (add corpus-backed save/load for `DashMap<[u8; 16], FingerprintState>`; update docstrings at lines 38-39 + 94-95), `crates/corpus/src/schema.rs` (decision per #70 schema choice — likely reuse pipeline_metrics rows for both), `pulse-app/src/lifecycle_persistence.rs` (NEW — `CorpusLifecyclePersistence` adapter), `pulse-app/src/storm_persistence.rs` (NEW — `CorpusStormPersistence` adapter), `pulse-app/src/main.rs` (boot: wire both adapters; restore order: corpus init → baseline restore (#70) → lifecycle restore (this chunk) → storm restore (this chunk); emit `CorpusRestore` lifecycle transitions for each service so constellation observers cascade correctly)
- **TauRPC delta:** none (existing `services.list_with_states` unchanged; restored state surfaces through existing surface)
- **Broadcast topics delta:** none added (existing `pulse://stream/service-lifecycle` continues; new CorpusRestore event variants flow through it on boot replay)
- **Workspace deps delta:** none
- **Arch registry delta:** 0 if reusing pipeline_metrics; +N if dedicated tables added (covered by #74 batch)
- **Specialist plan touches:** arch (Established Decisions note: lifecycle + storm states ride corpus), security (ServiceRegistry stores service.name strings — must scrub via #72 OR document as structural-metadata per P-048; storm fingerprints are hash bytes — no PII), test-plan (restart-and-reload preserves dot positions per P-027 verification; restart during active storm preserves dedup), obs (emit `metric.triage.lifecycle.corpus_restore_count_total` + `metric.triage.pattern.storm.corpus_restore_count_total` on boot)
- **Summary:** Close two persistence gaps remaining after #70. (1) `crates/triage/src/lifecycle/registry.rs::InMemoryServiceRegistry` (chunk #67) is `DashMap<String, ServiceRegistryEntry>` lost on every restart — but spec P-027 verification requires "Restart Pulse; verify dot positions match prior session", and the `service_registry` SQLite table is pre-allocated but never written. Wire DashMap → corpus via `trait LifecyclePersistence` + `CorpusLifecyclePersistence` adapter. Implement the missing `set_state_on_corpus_restore` method referenced in `registry.rs` module docstring but never defined as code; this makes `TransitionTrigger::CorpusRestore` enum variant at `state_machine.rs:47` reachable. On boot after corpus init, restore registry state + emit synthetic `CorpusRestore` lifecycle transitions per service so constellation observers cascade. (2) `crates/triage/src/pattern/storm.rs::RetryStormDetector` (chunk #66) maintains `DashMap<[u8; 16], FingerprintState>` for storm dedup; docstring explicitly defers persistence to chunk #69 which closed without wiring. Wire similarly via `trait StormPersistence` + `CorpusStormPersistence` adapter — without this, a process restart during an active retry storm immediately resets dedup, causing the same Suggested/Autonomous cue to re-emit (duplicate user-visible signals). Note: `RestartDetector.last_seen` + `SuppressionState` (chunk #63) intentionally NOT migrated — audit classified LOW since first-observation boundary handles cold-start natively. After landing: persistence model uniformly corpus-backed for all HIGH-severity runtime state; pure-in-memory tier reserved for trivially-derivable state. Closes audit Section 1.A in-memory tier findings + broken chunk #66 deferred-persistence promise.

### #72 — PII scrubber coverage extension

- **Depends on:** #65 (span_events ingestion which surfaced raw exception.message at appender), #68 (security crate `scrub_attribute` primitive with 7 P-047 categories: JWT / bearer / API key / secret KV / email / credit card / SSN), #69 (Drain template DuckDB write path that scrubs — reference pattern at `crates/buffer/src/drain.rs:586-606`), #70 (BaselineState corpus persistence — new corpus write site needs scrubber wired), #71 (Lifecycle + Storm corpus persistence — new corpus write sites need scrubber wired)
- **Capabilities affected (spec-compliance closure):** P-006 (close "exception.message (after PII scrubbing per P-047)" — currently `crates/buffer/src/appender.rs:332-356` writes raw exception.message to span_events.exception_message BLOB without scrubber); P-047 (close "all content before persistence to corpus or inclusion in Reports" — currently scrubber consumed at exactly ONE production site at `drain.rs:600`); P-048 (close "No raw OTLP attribute values stored" — under inclusive reading, DuckDB ring buffer + span_events currently store raw)
- **Distillation layer:** L0 (ingestion) + L5 (persistence boundary)
- **Crates touched:** `crates/buffer/src/appender.rs` (per-attribute scrubbing: span_events.exception_message + exception_stacktrace at lines 332-356; log_records.body; span attributes; metrics_points attributes), `crates/buffer/src/drain.rs` (close internal inconsistency at lines 580-628: `DrainMiner::snapshot_state` → `bincode::serialize` → `CorpusWriter::save_pipeline_metric` path currently bypasses scrubber while `write_template_to_table` for DuckDB scrubs — same `TemplateRecord.tokens` data, two persist targets, different PII discipline; fix by scrubbing tokens at snapshot_state boundary before bincode), `crates/corpus/src/contract.rs` (extend `CorpusWriter` trait at line 162 with test-enforceable pre-scrub invariant — either type-level `ScrubbedPipelineMetric` newtype wrapping payload OR runtime debug-assert + mandatory caller-side PII canary tests), `crates/security/src/scrubber.rs` (potentially extend with batch-scrub helper for performance — optional), PII negative-canary E2E tests at each new scrub site
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none (architectural surface unchanged; only call-site count expanded)
- **Specialist plan touches:** security (§Logging Anti-Pattern row 1 prohibits raw OTLP logging; this chunk brings code into compliance; chunk #77 specialist re-run formalizes), test-plan (PII negative-canary E2E tests: JWT in exception.message → `[REDACTED:jwt]`; bearer token in log_record body → `[REDACTED:bearer]`; credit card in OTLP attribute → `[REDACTED:credit_card]`; email in service.name (edge case from #71 ServiceRegistry persistence) → `[REDACTED:email]`; same for Drain DrainState corpus payload at snapshot_state boundary), obs (no new metrics; preserve tracing AllowList; scrubber operations aggregate-only)
- **Summary:** Extend PII scrubber coverage from current single production site (`crates/buffer/src/drain.rs:600` — Drain template DuckDB write only) to all OTLP-attribute persistence paths. Current state per audit: `scrub_attribute` from `crates/security/src/scrubber.rs:113-179` implements 7 P-047 categories but is invoked at exactly one production site; bypassed paths include OTLP ingestion via appender (raw exception.message + exception.stacktrace stored in `span_events` table), log_records.body, span attributes, metrics_points attributes, and the Drain corpus persist path (same `TemplateRecord.tokens` data scrubbed for DuckDB write but bypassed for `snapshot_state()` → bincode → corpus write — internal chunk #69 inconsistency). New scrub sites: (1) `crates/buffer/src/appender.rs:332-356` per-attribute writes — wire `scrub_attribute` over each OTLP attribute before push to Arrow record batch; (2) Drain corpus persistence — call `scrub_attribute` on each `TemplateRecord.tokens` entry inside `DrainMiner::snapshot_state` before bincode serialization, replacing match-hit tokens with `[REDACTED:{category}]` markers consistent with the DuckDB write path. Extend `CorpusWriter` trait contract at `crates/corpus/src/contract.rs:162` (currently docstrings "callers SHOULD pre-scrub" without enforcement) with type-level newtype `ScrubbedPipelineMetric` constructed only via scrubber-passing constructor (type-checker enforces) OR runtime debug-assert + mandatory caller-side PII negative-canary test per call site. Each new scrub site gets E2E negative-canary test injecting synthetic JWT/bearer/email/credit-card values + asserting `[REDACTED:{category}]` markers in persisted bytes. Algorithm unchanged — wiring expansion, not scrubber redesign. Closes audit Section 1.B + 1.C + P-006/P-047/P-048 findings.

### #73 — Capability spec numeric alignment

- **Depends on:** #59 (connection state machine — P-001 + P-003 scope), #61 (baseline trackers — P-010/P-011/P-012 wire into per-service EWMA + t-digest), #62 (cue evaluator + thresholds — P-010/P-012/P-014 evaluation paths), #66 (storm detector — P-014 boundary), #70 (BaselineState corpus persistence — restored state must reach this chunk's per-service baseline reads)
- **Capabilities affected (spec-compliance closure):** P-001 (close numeric threshold drift: current `IDLE_THRESHOLD_NANOS = 5_000_000_000` (5s) + `STALLED_THRESHOLD_NANOS = 30_000_000_000` (30s) at `crates/ingest/src/connection.rs:39-44` vs spec "Idle (10–60s), Stalled (>60s)"); P-003 (close failure path incompleteness: `derive_reason` returns only BindFailed/StaleHeartbeat; `ReceiverPanicked` enum variant at `connection.rs:79-93` unreachable because no panic hook routes panics to FSM); P-010 (close baseline-relative semantic: `crates/triage/src/cue/evaluate.rs:30-64` reads `thresholds.base_error_rate = 0.01` constant at `thresholds.rs:19` instead of per-service streaming baseline from P-009); P-011 (close operation identity hash-erasure: `operation_key` at `baseline/mod.rs:368-375` hashes operation_name into opaque key, irrecoverable downstream); P-012 (close percentile mismatch — current `DEFAULT_LATENCY_PERCENTILE = 0.95` at `thresholds.rs:36` vs spec p99 + fixed `base_latency_ms = 100ms` at `thresholds.rs:24`); P-014 (close minimum-threshold floor absence: current bootstrap suppression 60min but no 30s p95 floor)
- **Distillation layer:** L1b (anomaly detection thresholds) + L2 (attention cue evaluation)
- **Crates touched:** `crates/ingest/src/connection.rs` (P-001 threshold constants; P-003 ReceiverBindStatus extension + panic-hook propagation), `crates/triage/src/cue/thresholds.rs` (P-012 percentile 0.95 → 0.99; repurpose `base_error_rate` + `base_latency_ms` as fallback-only; P-014 `MIN_QUIET_SECONDS: u64 = 30` const), `crates/triage/src/cue/evaluate.rs` (P-010/P-012 rewrite to read per-service EwmaTracker + per-operation t-digest from BaselineState; replace `persistence_seconds = snapshot.samples` with wall-clock derivation; P-014 apply `max(p95_seconds, MIN_QUIET_SECONDS)` floor at lines 117-153), `crates/triage/src/baseline/mod.rs` (P-011 add `operation_name: Arc<str>` field on `OperationBaseline` + `OperationMetricSnapshot` at lines 368-375), `pulse-app/src/main.rs` or `pulse-app/src/observability.rs` (P-003 register `std::panic::set_hook` signaling shared atomic — must NOT log raw panic payload per security plan §Anti-Pattern Logging vector 7)
- **TauRPC delta:** none (existing `connection.current_state` unchanged; `ReceiverPanicked` variant becomes reachable, no new procedure)
- **Broadcast topics delta:** ReceiverPanicked variant emission on existing `pulse://stream/connection-state` topic (existing topic; new variant data, no new topic)
- **Workspace deps delta:** none
- **Arch registry delta:** none directly; if any threshold becomes user-configurable via Settings, Settings struct extension touches arch §Inherited Defaults — but recommend keeping spec defaults hard-coded for v0.2.0
- **Specialist plan touches:** arch (thresholds documented in §Established Decisions or §Inherited Defaults), test-plan (per-capability acceptance tests: P-001 boundary at 10s + 60s with FSM transitions; P-003 panic hook integration test via synthetic panic; P-010 baseline-relative spike injection with per-service baselines at different absolute rates (0.5% / 2% / 5%); P-011 operation_name round-trip via cue evaluation; P-012 percentile boundary at p99; P-014 floor exercised with high-frequency vs low-frequency edge cases), obs (per-tracker metric emission preserved; panic event uses sanitized payload), security (panic hook MUST NOT log raw panic payload per §Anti-Pattern Logging — panics from instrumented code can contain user data)
- **Summary:** Per-capability numeric / identity / failure-path fixes for chunks #59/#61/#62/#66 spec-compliance. Six sub-tasks: (1) **P-001 thresholds** — change `IDLE_THRESHOLD_NANOS` 5s→10s and `STALLED_THRESHOLD_NANOS` 30s→60s at `crates/ingest/src/connection.rs:39-44`; if heartbeat-CI-alarm at 45s couples (current rationale), decouple by independent alarm-side threshold. (2) **P-003 panic propagation** — register `std::panic::set_hook` in `pulse-app/src/main.rs` boot signaling shared atomic read by `ReceiverBindStatus::any_receiver_failed`; makes `ReceiverPanicked` reachable in `derive_reason` at `connection.rs:264-282`. Hook must NOT log raw panic payload (security plan §Anti-Pattern Logging — panics from instrumented code can contain PII). (3) **P-010 baseline-relative semantics** — rewrite `crates/triage/src/cue/evaluate.rs:30-64` to read per-service EwmaTracker value from BaselineState (existing accessor `error_rate(service_name)`) instead of fixed `thresholds.base_error_rate = 0.01`; replace `persistence_seconds = snapshot.samples` (sample-count units) with wall-clock derivation from timestamp deltas. (4) **P-011 operation identity preservation** — add `operation_name: Arc<str>` field on `OperationBaseline` + `OperationMetricSnapshot` at `crates/triage/src/baseline/mod.rs:368-375`; preserve hashed `operation_key` as DashMap key for collision-resistance but pass human-readable name to cue.scope_id so downstream surfaces (FindingsDropdown when chunk #86 lands) can show "p99 of GET /endpoint regressed". (5) **P-012 percentile + baseline-relative** — change `DEFAULT_LATENCY_PERCENTILE = 0.95` to `0.99` at `crates/triage/src/cue/thresholds.rs:36`; rewrite latency regression evaluator to read per-operation t-digest p99 from BaselineState instead of fixed `base_latency_ms = 100ms`. (6) **P-014 minimum quiet floor** — add `MIN_QUIET_SECONDS: u64 = 30` const + apply `max(p95_seconds, MIN_QUIET_SECONDS)` in `evaluate_service_went_silent` at `crates/triage/src/cue/evaluate.rs:117-153`. For each sub-task, chunk planning decides Implementation (fix code to satisfy spec) vs Spec Amendment (Type 5 evolve flag against `pulse-capability-spec.md` to downgrade SHALL → SHOULD or relax numeric values); default Implementation, but P-001 thresholds may justify configurability amendment if heartbeat-coupling rationale persists. Closes audit Section 1.D capability-PARTIAL findings (numeric/identity/threshold subgroup).

### #74 — Architecture registry alignment batch

- **Depends on:** #70 (BaselineState corpus migration — decides whether `triage/baseline-corpus.bin` subpath persists), #71 (ServiceRegistry + RetryStormState corpus migration — affects schema additions to register), #72 (PII scrubber extension — `CorpusWriter` trait contract change may register if formally encoded), #73 (capability spec numeric alignment — thresholds may register in arch §Established Decisions)
- **Capabilities affected:** none directly (META work — closes architecture document drift surfaced by audit Dim 4)
- **Distillation layer:** N/A — META
- **Crates touched:** none (`.andromeda/architecture.md` registry edits + `state.yaml` updates via `/andromeda-evolve --allow-arch-registry` Type 6 amendment flow)
- **TauRPC delta:** none added; CLEANUP of registry-stale forward-promises `snapshot.list_recent`, `snapshot.copy_to_clipboard`, `workspace.list` (registered in arch but never implemented; xtask `EXPECTED_PROCEDURES` already correctly omits them)
- **Broadcast topics delta:** none added; CLEANUP of `pulse://stream/plugin-events` registry-stale entry (registered in arch but only doc-references exist, no runtime emitter)
- **Workspace deps delta:** none
- **Arch registry delta:** +1 DuckDB reserved table (`log_templates` — closes chunk #69 Step 32 registry-level incompleteness); +1 new sub-section "Corpus SQLite database / schema names" with 6 tables (`baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive`); possibly +N tables/columns from #71 if dedicated new schema added rather than reusing pipeline_metrics; CLEANUP: remove or tag-deferred `snapshot.list_recent` / `snapshot.copy_to_clipboard` / `workspace.list` TauRPC + `pulse://stream/plugin-events` broadcast + `ANDROMEDA_PULSE_CONFIG_PATH` env var
- **Specialist plan touches:** arch ONLY (pure registry alignment)
- **Summary:** META chunk — Type 6 amendment batch via `/andromeda-evolve --allow-arch-registry` (or successor flag after #76 lands) closing arch registry drift accumulated through chunks #57-#69 + new state from #70-#73. Specifically: (1) **Add `log_templates` to arch §Occupied Resources DuckDB reserved tables** — chunk #69 Phase B added this 8th table at `crates/buffer/src/schema.rs:18,125` and the schema docstring at lines 5-9 explicitly self-flagged "Reserved table list per arch §Occupied Resources §DuckDB reserved tables. Chunk #20 baseline locked 7 tables; chunk #69 Phase B adds `log_templates` (8th table). Arch §Occupied Resources update via /andromeda-evolve --allow-arch-registry lands at Session 6 wrap per chunk #69 Phase B plan.md Step 32." Session 101 Type 6 closed only the TauRPC piece (`diagnostics.template_distribution`) leaving the DuckDB sibling unacknowledged; this chunk closes chunk #69 Step 32 properly at registry level. (2) **Add new sub-section "Corpus SQLite database / schema names"** mirroring existing DuckDB sub-section, listing 6 tables from `crates/corpus/src/schema.rs:26-33`. Currently corpus tables are not in arch §Occupied Resources anywhere — chunk #68 amendment registered file path (`corpus/corpus.db`) but not schema. Corpus is project's FIRST persistent on-disk DB; deserves equal registry presence. (3) **Handle filesystem subpath registration** — if #70 removed `<data_dir>/triage/baseline-corpus.bin`, no registry entry needed; if for any reason #70 deferred file removal, register the subpath as Type 6. (4) **Register additional tables from #71** if it created dedicated lifecycle/storm tables rather than reusing pipeline_metrics. (5) **Cleanup forward-promise drift**: remove or tag-deferred `snapshot.list_recent` + `snapshot.copy_to_clipboard` (only `snapshot.generate` implemented at `pulse-app/src/snapshot_runtime.rs:130`); `workspace.list` (only `workspace.detect` at `crates/ui-bridge/src/workspace_ipc.rs:47`); `pulse://stream/plugin-events` (no runtime emitter); `ANDROMEDA_PULSE_CONFIG_PATH` env var (Settings uses fixed `<data_dir>/config.toml`, no env override). (6) **Optionally register harness-only env vars** — `ANDROMEDA_PULSE_PIDFILE` / `LOGFILE` / `DATA_DIR_KEEP` used only in `scripts/agent-run.{sh,ps1}`. (7) **Optionally register `run/andromeda-pulse.pid` filesystem subpath**. Multiple Type 6 amendments may be needed per `spec-amendment-protocol.md` (one per amendment_id); chunk plan decides batching strategy. Closes audit Section 1.F + 2.I findings.

### #75 — Documentation consolidation

- **Depends on:** #74 (arch registry alignment — final arch state settled before doc cross-refs lock against arch §-numbers + narrative count lines)
- **Capabilities affected:** none
- **Distillation layer:** N/A — META
- **Crates touched:** none (documentation edit only)
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none (consumes #74's arch state, adds nothing)
- **Specialist plan touches:** none directly (doc-level only)
- **Summary:** META chunk — single batch edit session fixing all cross-reference drift surfaced in audit Dim 6 (8 BROKEN + 4 STALE references). Specifically: (1) **`arch.md:167`** — change "per obs-plan §11 Frontend bridge" to "per obs-plan §3 Logging stack > Frontend bridge" (obs-plan §11 = Anti-Patterns; Frontend bridge subsection at §3). (2) **`route.md:329`** — chunk #67 cite "pulse-v0_2_0-route.md §Phase 4 line 276" should point at correct v3 line for Service registry header (line numbers shift after v3 Phase 6 Consolidation insertion). (3) **`docs/v0_2_0/pulse-distillation-architecture.md:5`** — replace `widget-state-validation-mini-route.md (delivery plan)` with `pulse-v0_2_0-route.md (delivery plan)` (file renamed per v2 changelog). (4) **`docs/v0_2_0/pulse-capability-spec.md:5`** — same widget-state rename replacement + prefix `widget-state-validation-report-2026-05-14.md` with `.andromeda/scope-validation/`. (5) **`docs/v0_2_0/pulse-capability-spec.md:789`** — strike or rephrase to past-tense mini-route reorganization clause; capability-to-chunk mapping already in `pulse-v0_2_0-route.md`. (6) **`docs/v0_2_0/pulse-distillation-architecture.md:980-995`** — delete §TODO "capability spec formalization (NEW in v3)" subsection OR replace with "Resolved in capability spec v2 — see P-052..P-060" (all 8 items now formalized). (7) **`docs/v0_2_0/pulse-v0_2_0-route.md` capability-to-chunk mapping table** — audit Dim 6 flagged the row "P-019 to P-023, P-060 | #67 superseded by #72-#77" as stale (v2 #67 = Drain, not Severity classifier); v3 capability mapping is refreshed during the v3 manual edit (this chunk operates against any residual table issues). (8) **`arch.md` narrative cascade** — change "eight library crates" to "twelve library crates" at §Design Philosophy line 4, §Infrastructure Patterns line 220, §Project Intent line 303. If chunk #76 P7 (Type 6 narrative-cascade visibility) lands BEFORE this chunk, these arch.md narrative cascade edits propagate automatically when previous Type 6 amendments in #74 fire; in that case this chunk's scope reduces to items 1-7. (9) optionally: METADATA bloat prune in `.andromeda/context/api-surface.md` + `dependency-tree.md` (audit Section 3.R cleanup — defer to next api-surface re-baseline cycle acceptable). Cost estimate: ~30 minutes manual batch edit. Closes audit Section 1.G + 2.J cross-reference drift findings.

### #76 — Andromeda pipeline meta-improvements (P7 + P12 + file P15-P18)

- **Depends on:** none in andromeda-pulse codebase (META work touches Andromeda toolkit at user level); sequencing AFTER #74/#75 OR as last chunk before re-audit
- **Capabilities affected:** none (Andromeda-pipeline-level, not pulse-product-level)
- **Distillation layer:** N/A — META
- **Crates touched:** NONE in andromeda-pulse codebase
- **External skill files touched:** `~/.claude/skills/andromeda-evolve/` (Phase 5/6 logic — pre-populate expected_propagation; cascade detection), `~/.claude/skills/andromeda-setup-project/` (delta-rerun cascade behavior), `~/.claude/skills/andromeda-wrap-session/` (Phase 8 amendment archival cascade verification); ~155 LOC across ~8 files per audit Section 4 estimates
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none directly (closes recurring drift category for FUTURE chunks; existing drift from #57-#69 closed by #74)
- **Specialist plan touches:** none in this project; touches Andromeda toolkit documentation (`andromeda-improvements.md` Proposal 7/12 status update + new P15-P18 entries)
- **Summary:** META chunk — implement two `docs/andromeda-improvements.md` proposals identified by audit Section 4 as IMPLEMENT NOW candidates (mature pending; 4+ direct dogfood occurrences each; would resolve recurring drift category producing audit findings rows 9-11 narrative cascade). (1) **Proposal 7 "Type 6 narrative-cascade visibility"** — extend `/andromeda-evolve --allow-arch-registry` to detect when registry section additions imply a count line change in arch.md narrative sections (§Design Philosophy line 4 "eight library crates", §Infrastructure Patterns line 220, §Project Intent line 303 — stale at "eight" with reality at 12 after chunks #58/#60/#68 added 4 crates); mechanically regex-replace count lines as part of Type 6 propagation flow OR trigger automatic full setup-project re-derive when count-line affecting registry sections are touched. (2) **Proposal 12 "Type 6 CLAUDE.md derived-section cascade pre-populate"** — extend `/andromeda-evolve --allow-arch-registry` AND `/andromeda-setup-project --delta` to pre-populate `expected_propagation: [CLAUDE.md]` on spec_amendment entries when Type 6 addition touches a registry section appearing in CLAUDE.md derived sections (Modules / Stack / Key directories). 3+ direct occurrences (chunks #58/#60/#68 each Type 6 left CLAUDE.md Modules/Stack desynced); operational workaround documented session-learnings 2026-05-18 as "choose full /andromeda-setup-project when Type 6 adds workspace crates"; P12 makes this automatic. (3) **File 4 new proposals (P15-P18)** surfaced during audit: P15 "Route↔source-doc chunk-number divergence canonical artifact" (3+ occurrences in project: route.md #67/#68/#69 vs v0_2_0-route §67/§68/§69 inverted mapping); P16 "D7 cross-document reference resolution check at wrap-session" (audit found 8 BROKEN + 4 STALE cross-references no D1-D6 catches); P17 "Doc-rename cascading reference update" (`widget-state-validation-mini-route.md` rename left 3+ stale refs across other docs); P18 "Stale TODO detection in v0_2_0 planning docs" (dist-arch:980-995 stale TODO). After landing: P7 + P12 active eliminating recurring Tier 1/2 cascade staleness; P15-P18 captured for future implementation prioritization. **Sequencing note:** recommend landing AFTER #74/#75 with P7/P12 active only for chunks #77+ going forward (accepts one final manual cascade for #74/#75). Closes audit Section 4 IMPLEMENT NOW candidates.

### #77 — Specialist plan re-runs (security + tests)

- **Depends on:** #70-#76 (all prior consolidation chunks must land so specialist plan re-derive captures complete post-consolidation picture)
- **Capabilities affected:** none directly (specialist plans inform but don't enable capabilities; they document obligations + acceptance criteria — re-derive captures new reality)
- **Distillation layer:** N/A — META
- **Crates touched:** none in this chunk; specialist plan re-runs may identify follow-up work touching crates as separate future chunks
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none directly; may surface follow-up Type 6 amendments as separate sub-chunks
- **Specialist plan touches:** ALL specialist plans potentially — security-plan (definitely; persistent storage tier re-derive), test-plan (definitely; deferred PII vector triggers), obs-plan (possibly; new metric emission from #70-#73 may warrant per-target emission section update), design-system + a11y-plan + layout-templates (no direct trigger from consolidation chunks; optional sanity re-check)
- **Summary:** META chunk — two specialist plan re-runs closing audit Section 2.H (security plan staleness) + Section 2.K (PII vector test gaps). May split into #77a (security re-run) + #77b (tests re-run) during chunk planning phase if separate concerns justify granularity. (1) **`/andromeda-security` re-run** — extend `crates/security-plan.md` §Threat Model + §Data Protection §At rest with new rows reflecting post-consolidation persistent storage: corpus SQLite (encrypted via AES-256-GCM cell-level + OS keychain key custody per chunk #68); after #70 + #71 the flat-file `baseline-corpus.bin` row is eliminated (was plaintext-with-PII-risk). Extend §Secret Management with corpus encryption key custody flow including keyring backend abstraction + fallback handling per P-049 (note #73 may or may not have addressed passphrase fallback — per-capability decision; whichever way it went, security plan documents the chosen posture). Update §Anti-Pattern Logging entries to reflect new scrubber coverage paths from #72 (no longer single-site; now uniform across persistence paths). Session-handoff carry-over flag "Cross-cutting /andromeda-security re-run still flagged" (accumulating across 8+ wraps per audit) gets cleared. (2) **`/andromeda-tests` re-run** — materialize 4 deferred PII vector trigger tests + capability widening static analysis trigger documented in `.claude/rules/testing.md §Pending coverage triggers`: (a) **AppError sanitization PII vector test** — verify no stack traces / file paths / Rust struct names in `AppError::Internal { message }` via integration test exercising error path through TauRPC bridge; (b) **Plugin path basename-only logging test** — verify plugin file paths in tracing logs are basenames only per security plan §Logging vector 3; (c) **MCP response body redaction test** — verify response.body never logged per security plan §Logging vector 4; exercise MCP tool invocation + assert response body absent from tracing-appender output; (d) **Path env var canonicalization log redaction test** — verify `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env vars canonicalize via strict-path + don't leak raw user-supplied paths in logs; (e) **Capability widening static analysis test** — verify `pulse-app/capabilities/*.json` `permissions` array doesn't include disallowed Tauri core API names for `pulse:notification` / `pulse:tray` / `pulse:plugin-fs` capabilities per security.md Session Addition 2026-05-08. Plus chunk #69 LogHub golden corpus test harness if not landed during #69 closure — audit Section 3.S found algorithm correctness verified via unit tests but no `tests/golden/drain/` directory visible. Closes audit Section 2.H + 2.K + 3.S findings. After landing: specialist plans accurately reflect post-consolidation reality; ready for #78 Incident records (formerly v2 §70).

---

## Phase 7 — Incident records + digest pipeline

This phase combines (a) the incident records persistence layer originally planned as v2 §70 (Phase 5) with (b) the L1a → L2 → L3 cadence machinery (v2 Phase 6 Digest pipeline) — both now sit logically after Phase 6 Consolidation lands. No user-visible surfaces appear from this phase alone — surfaces appear after L4 (#83) consumes digests.

### #78 — Incident records + lifecycle persistence

- **Depends on:** #69 (corpus scaffold), #60 (triage Incident contract types)
- **Capabilities enabled:** P-022 (Auto-Resolution and Lifecycle scaffold), P-023 (Acknowledge Cool-Down), P-041 (Persistent Incident Corpus — incident records), P-042 (Cross-Session Continuity), P-043 (Project-Scoped Memory), P-045 (Counter Derivation from Corpus)
- **Distillation layer:** L5 (incident persistence)
- **Crates touched:** `crates/corpus/`, `crates/triage/incident`
- **TauRPC delta:** +1 procedure `incidents.list_active()`, +1 `incidents.acknowledge(id)`, +1 `incidents.mark_resolved(id)`
- **Broadcast topics delta:** +1 `pulse://stream/incidents` (lifecycle events: Created, Updated, Resolved, Acknowledged)
- **Workspace deps delta:** none
- **Arch registry delta:** +3 TauRPC procedures, +1 broadcast topic
- **Specialist plan touches:** arch, test-plan (incident lifecycle: create → update → auto-resolve at 120s → acknowledge with 5-min cool-down; cross-session continuity verified by restart-and-reload; project scoping verified by workspace switch), security (incident content anonymization per existing AppError sanitization patterns)
- **Summary:** Incident records persistently stored in corpus with workspace attribution via workspace-detector. Lifecycle: Active → Resolved (auto after 120s of no re-emission) → corpus-only after 10-minute recent-history display. Acknowledge state persists in corpus (replaces old JSON file approach from original #68). 5-minute cool-down per kind-and-scope. Findings counter derived via corpus query per P-045: `SELECT COUNT(*) FROM incidents WHERE workspace = ? AND read_at IS NULL AND status = 'active'`. Read-state updates on Report-opening event.

### #79 — SQL aggregation queries + scheduler

- **Depends on:** #58 (curation), #67 (log templates available), #66 (fingerprints available)
- **Capabilities enabled:** prerequisite for P-020, P-021 (algorithmic detection requires aggregated input)
- **Distillation layer:** L1a
- **Crates touched:** `crates/triage/baseline` (SQL templates module), `crates/buffer/` (query execution helpers if needed)
- **TauRPC delta:** none yet
- **Broadcast topics delta:** none (results consumed in-process by Cadence Coordinator #80)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** arch, test-plan (time-injected unit tests per query Q1-Q7; Q7 worst-case bounds verified with broad trace fixture), obs-plan (`pipeline.l1a.query_count_total{query_name}`, `pipeline.l1a.query_latency_p99_milliseconds{query_name}`, `pipeline.l1a.q7_timeout_count_total`, `pipeline.l1a.q7_fallback_count_total`)
- **Summary:** SQL templates Q1-Q7 per dist-arch v3 §Appendix A executed against L0 ring buffer. Q7 includes explicit bounds (`LIMIT 100`, `LIMIT 10` depth, 200ms timeout, fallback to shallow non-recursive query). Scheduler invocation hook to be wired by Cadence Coordinator (#80).

### #80 — Cadence coordinator + three-tier triggering

- **Depends on:** #62 (attention cues + cadence-triggers channel), #79 (queries to invoke)
- **Capabilities enabled:** P-052 (Cadence Configuration), P-060 (Tiered Triggering Priority)
- **Distillation layer:** L2 routing + L3 invocation
- **Crates touched:** `crates/triage/cue` (extends), NEW `crates/triage/cadence` module
- **TauRPC delta:** none yet
- **Broadcast topics delta:** +1 `pulse://stream/cadence-events` (for L6 visibility)
- **Workspace deps delta:** none
- **Arch registry delta:** +1 broadcast topic
- **Specialist plan touches:** test-plan (Tier-1 hard signal → immediate L3 + L4 invocation; Tier-2 cue → accelerated next cadence; Tier-3 baseline cadence; cpu-primary disables Tier-2 acceleration), obs-plan (`pipeline.l3.digests_assembled_total{mode}`)
- **Summary:** Three-tier coordinator per dist-arch v3 §Cadence and Event Triggers. Default cadence 60s baseline, 20s accelerated, 1800s reflection. Cadence values loaded from `[triage.cadence]` config with hot-reload support (delivered in #94). Safety floor enforcement: baseline ≥ 5s, reflection ≥ 5min. Tier-2 acceleration conditionally disabled when hardware profile is `cpu-primary` (set by #82). Coordinator schedules L1a query execution + L3 digest assembly + L4 inference invocation.

### #81 — Digest assembler + LWW queue + active-incident exception

- **Depends on:** #79 (L1a outputs), #62 (attention cues), #66 (fingerprints), #67 (templates), #69 (corpus retrieval), #78 (active incident state)
- **Capabilities enabled:** P-031 foundation (Report Structure — digest is precursor), P-032 (Project Context Grounding), P-044 (Retrieval-Augmented Interpretation), P-059 (Active-Incident Interpretation Continuity)
- **Distillation layer:** L3
- **Crates touched:** `crates/triage/digest`, `crates/workspace-detector` (consumer)
- **TauRPC delta:** none yet
- **Broadcast topics delta:** +1 `pulse://stream/digests` (LWW queue for L4), corpus appends (all digests, no LWW)
- **Workspace deps delta:** +`tokenizers` (Hugging Face tokenizer library for token counting)
- **Arch registry delta:** +1 broadcast topic, +1 dep
- **Specialist plan touches:** arch, test-plan (**golden-file tests for digest composition** per dist-arch v3 testability strategy; LWW behavior; active-incident exception; corpus retrieval), obs-plan (`pipeline.l3.digest_token_count_p99`, `pipeline.l3.lww_drop_count_total`, `pipeline.l3.active_incident_queue_depth`)
- **Summary:** Digest assembler composes structured digest per dist-arch v3 §Appendix C from L1a outputs, L2 cues, project context (workspace path, framework signals, recent git activity from filesystem inspection), corpus retrieval (top-3 similar past incidents in workspace via fingerprint match). Token budget 500-2000, hard cap 3000, counted via `tokenizers` matched to active model (set by #82). **Queue behavior:** LWW for cadence-mode by default; active-incident exception bypasses LWW for cadence digests when L5 has unresolved incident with severity ≥ Suggested; Tier-1 hard signals never LWW-replaced; all digests written to corpus regardless of queue state.

---

## Phase 8 — LLM interpretation

> **Phase blocked by Pre-D1** (LLM runtime choice). Chunks #82-#85 are the highest-risk chunks in route — LLM integration brings dependencies, hardware quirks, and runtime maturity questions that historical practice resolves better than upfront analysis.

### #82 — Hardware profile detection + model loading + tokenizer

- **Depends on:** Pre-D1 (runtime choice resolved)
- **Capabilities enabled:** P-053 (Fallback Model Tier — detection side), P-054 (Hardware Profile Awareness)
- **Distillation layer:** L4 foundation
- **Crates touched:** NEW `crates/interpretation/` (or named per runtime choice), `crates/triage/cadence` (cpu-primary tier-2 disable wiring)
- **TauRPC delta:** +1 procedure `model.current_profile()` returns profile + tier + model identity
- **Broadcast topics delta:** +1 `pulse://stream/model-status` (load/unload/error events)
- **Workspace deps delta:** +chosen LLM runtime (`mistralrs` or `candle`)
- **Arch registry delta:** +1 crate, +1 TauRPC procedure, +1 broadcast topic, **§Established Decisions: LLM runtime choice with rationale** (per Pre-D1)
- **Specialist plan touches:** arch (Established Decision entry), test-plan (profile detection on each hardware configuration; cpu-primary detection triggers one-time notice exactly once), security (model file integrity, no model auto-download without user consent), obs-plan (`pipeline.l4.hardware_profile_active{profile}`, `pipeline.l4.model_tier_active{tier}`)
- **Summary:** Hardware profile detection on startup classifies into `gpu-primary`, `gpu-fallback`, `cpu-primary`, `cpu-fallback`. Model loading via chosen runtime with tokenizer paired to checkpoint. Tokenizer reload on model change (handled in #83). On `cpu-primary` profile, emit one-time notice with "Switch to fallback" / "Keep current" options; record decision so notice doesn't repeat. Profile and tier visible in Settings → Diagnostics → Model (frontend in #95).

### #83 — Prompt scaffolding + JSON schema + primary tier inference

- **Depends on:** #82 (model loaded), #81 (digest produced)
- **Capabilities enabled:** P-019 (Three-Tier Severity Model — model decision side), P-020 (Model-Driven Severity Decision — primary tier), P-033 (Ranked Hypothesis Generation — primary tier), P-034 (Suggested Investigation Steps — primary tier)
- **Distillation layer:** L4
- **Crates touched:** `crates/interpretation/` (prompt + inference modules)
- **TauRPC delta:** none yet (consumed via incidents stream from #78)
- **Broadcast topics delta:** none new (writes to existing incidents stream)
- **Workspace deps delta:** none
- **Arch registry delta:** none (uses crate added in #82)
- **Specialist plan touches:** test-plan (schema validation tests with constructed JSON; LLM output quality covered by Conductor scenarios separately), obs-plan (`pipeline.l4.inferences_total{result}`, `pipeline.l4.inference_latency_p99_milliseconds`, `pipeline.l4.inference_queue_depth`)
- **Summary:** System prompt with role definition, project conventions, JSON output schema embedded. JSON-constrained inference via runtime's grammar enforcement. Output schema per dist-arch v3 §L4 output format with `schema_version`, `prompt_version`, `model_tier`, `hardware_profile`, `is_resolution_summary` fields. Provenance, not invalidation — old corpus records remain queryable with version annotations.

### #84 — Fallback model tier support

- **Depends on:** #83
- **Capabilities enabled:** P-053 (Fallback Model Tier — full)
- **Distillation layer:** L4
- **Crates touched:** `crates/interpretation/` (fallback prompt + reduced-quality output)
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (fallback-tier scenarios produce reduced output: single hypothesis, ≤2 investigation steps; Report displays fallback-tier indicator), security (fallback model file integrity equal to primary)
- **Summary:** Reduced-quality prompt and schema for 3-4B class models. Single hypothesis instead of ranked list. Up to 2 investigation steps instead of 5. Less specific project context grounding (~500-700 token system prompt vs primary's ~800-1000). Output JSON includes `model_tier: "fallback"` for downstream rendering. CPU inference fully supported at 3-8s typical latency.

### #85 — JSON parse failure handling + backoff + resolution summary

- **Depends on:** #83 (primary inference), #84 (fallback inference), #78 (incident lifecycle)
- **Capabilities enabled:** P-020 graceful degradation (full), P-022 (resolution summary attachment), P-059 (resolution summary generation)
- **Distillation layer:** L4 failure handling + L5 surface
- **Crates touched:** `crates/interpretation/`, `crates/triage/incident`
- **TauRPC delta:** +1 procedure `diagnostics.retry_interpretation()` (manual override for backoff)
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** +1 TauRPC procedure
- **Specialist plan touches:** test-plan (single parse failure dismisses cycle; 3-in-5min triggers degraded mode; backoff 2min→5min→10min progression; manual recovery; resolution summary generation), obs-plan (`pipeline.l4.degraded_mode_active_seconds_total`, `pipeline.l4.degraded_mode_entries_total`, `pipeline.l4.backoff_remaining_seconds`)
- **Summary:** Per-incident failure handling: single parse failure logs to L6, dismisses current incident cycle, increments failure counter. Three consecutive failures in 5 minutes → L4 degraded mode with exponential backoff retry (2→5→10min capped). Recovery: successful parse exits degraded mode immediately. Manual override via Settings → Diagnostics "Retry interpretation now" button. **Resolution summary:** when incident transitions to Resolved (P-022 auto-resolution or user mark), generate summary interpretation with `is_resolution_summary: true` flag, attach to incident record without new surface notification.

---

## Phase 9 — User-facing surfaces

This phase replaces the original mini-route's #69-#73 work with redesigned surfaces matching capability spec v2 three-surface architecture.

### #86 — Findings counter + dropdown + TauRPC + corpus integration

> **Replaces original mini-route #72** (incident card slot) — new three-surface design has no incident card embedded in widget.

- **Depends on:** #78 (incident records), #81 (digest produces incidents via #83), #85 (severity decided)
- **Capabilities enabled:** P-028 (Findings Counter), P-029 (Findings Dropdown), P-030 (No Interrupting Notifications)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/widget/FindingsCounter.tsx` (NEW), `pulse-app/ui/widget/FindingsDropdown.tsx` (NEW), `crates/ui-bridge/contract.rs`
- **TauRPC delta:** consumer-side mostly (uses procedures from #78); +1 `incidents.mark_all_read()`
- **Broadcast topics delta:** consumer-side (subscribes to `pulse://stream/incidents`)
- **Workspace deps delta:** none
- **Arch registry delta:** +1 TauRPC procedure
- **Specialist plan touches:** design-system (counter visual spec — circle with number, severity-colored; dropdown row visual spec — severity dot + title + relative time), layout-templates (counter position adjacent to widget; dropdown overlay positioning), a11y-plan (counter aria-label, dropdown keyboard navigation, focus management on row click), test-plan (counter derives correctly from corpus; persists across app restart; clears on Report-opening; "Mark all as read" action)
- **Summary:** Compact circular counter below widget showing count of unread active incidents in current workspace. Hidden when count is zero. Counter derived via corpus query on incident events and widget focus, no separate state file. Click expands dropdown listing unread incidents sorted by severity descending then chronologically: severity indicator + title + relative timestamp ("3m ago"). Row click opens Report (#87). Dropdown footer has "Mark all as read" action. **No OS notifications, no toasts, no modals** per P-030 invariant.

### #87 — Diagnostic Report generation (in-app + copy markdown)

- **Depends on:** #83 (LLM output to render), #85 (resolution summary path)
- **Capabilities enabled:** P-031 (Report Structure), P-035 (Anonymized Telemetry Excerpts), P-036 (Cross-Incident Pattern Reference), P-037 (In-App Report Surface), P-038 (Copy to Clipboard)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/report/Report.tsx` (NEW), `pulse-app/ui/report/ReportRenderer.tsx` (NEW), `crates/interpretation/markdown.rs` (NEW — markdown serialization)
- **TauRPC delta:** +1 procedure `incidents.get_report(id)` returns Report content + markdown serialization
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** +1 TauRPC procedure
- **Specialist plan touches:** design-system (Report visual spec — sections, hierarchy, typography), layout-templates (Report panel/window layout), a11y-plan (Report keyboard navigation, section anchors, copy shortcut), test-plan (six-section structure verified; fallback-tier reduced fidelity verified; copy markdown identical to MCP delivery format per P-038)
- **Summary:** In-app Report rendering in full-window panel or separate window. Six sections per P-031: symptom, timeline, hypotheses, investigation steps, evidence, project context. Section collapse for managing density. Telemetry excerpts respect PII scrubbing. "Previously seen" subsection when corpus matches exist per P-036. **Copy Report** action serializes complete Report as markdown to clipboard; format identical to MCP delivery (#92) for consistency. Reports in degraded mode omit hypotheses and investigation steps with explicit notice. Resolution summary section appears for Resolved incidents.

### #88 — Header redesign: connection dot + chrome cleanup

> Continues original mini-route #69 plan. Removes synthetic chrome.

- **Depends on:** #57 (real-data binding), #59 (connection state)
- **Capabilities enabled:** P-001 visual signal, P-024 (Widget Ambient Surface — chrome cleanup invariant)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/components/Titlebar.tsx`, `pulse-app/ui/widget/FooterBand.tsx`, `pulse-app/ui/widget/CompactWidget.tsx`
- **TauRPC delta:** consumer-side
- **Broadcast topics delta:** consumer-side (subscribes to connection-state)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** design-system (connection dot palette per state, tooltip styling), layout-templates (titlebar redesign), a11y-plan (connection dot aria-label "Connection: {state} — last span {ago}", focus order)
- **Summary:** **Remove `Ingest 200/s` / `Error 4.2%` / `Retention 3m / 10m` footer band entirely** — these are dashboard mindset, violate P-024 ambient invariant. Add 6-8px connection state dot near app icon. Add expand-to-dashboard button (existing affordance, kept). Tooltip on dot hover shows current state + last-span-ago.

### #89 — Halo formula refactor

- **Depends on:** #83 (severity decisions from LLM), #59 (connection state)
- **Capabilities enabled:** P-025 (Halo Hue Encoding), P-026 (Halo Breathing Encoding)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/halo/HaloCanvas.tsx`, `pulse-app/ui/halo/lch.ts`, `pulse-app/ui/halo/shaders/halo.wgsl`, delete `pulse-app/ui/halo/error-rate-to-blur.ts` and `pulse-app/ui/halo/throughput-to-hz.ts`
- **TauRPC delta:** consumer-side
- **Broadcast topics delta:** consumer-side (subscribes to incidents)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** design-system (severity-to-blur mapping: Autonomous=16px, Suggested=12px, Curious=8px, None=4px; severity-to-hue Earth Blue → Alert Burgundy interpolated), a11y-plan (reduced-motion mapping for breathing model)
- **Summary:** Halo props change from `(errorRate: f32, throughputHz: f32)` to `(connectionState: ConnectionState, cumulativeSeverity: Severity, activityState: ActivityState)`. `cumulativeSeverity` = max severity of active incidents. **Breathing via opacity AND blur modulation only, NEVER scale** per P-026 invariant — 4-5s cycle quiet, down to 2s under active flow, ease-in-out. Hue interpolated on cumulative severity. Connection state grays out halo (separate visual axis from severity per P-004 orthogonality).

### #90 — Service constellation rendering

- **Depends on:** #61 (baseline trackers expose per-service activity), #68 (service lifecycle states), #89 (halo refactor settles canvas layout)
- **Capabilities enabled:** P-027 (Service Constellation Auto-Discovery)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/widget/AggregatedBadgeCanvas.tsx` (replace), maybe new `ConstellationCanvas.tsx`
- **TauRPC delta:** consumer-side (uses procedure from #68)
- **Broadcast topics delta:** consumer-side (subscribes to service-lifecycle from #68)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** design-system (per-service dot palette: brightness=activity, hue=error state; pseudo-random scatter seed convention), layout-templates (constellation positioning rules, up to 20 services rendered), a11y-plan (dot hover tooltips, keyboard navigation)
- **Summary:** Replace existing aggregated badge with actual constellation dots. One dot per service from `service_registry`. Brightness=recent activity from histogram, hue=per-service severity from incidents scoped to that service. Position: stable pseudo-random scatter with `seed = hash(service_name)` for cross-session consistency. Dormant services dimmed, Archived hidden. Hover tooltip shows service name + recent activity rate + current lifecycle state.

### #91 — ConstellationCanvas dashboard cascade

> Addresses cascade from original validation report §5.2 — dashboard constellation needs same halo API update.

- **Depends on:** #89 (HaloCanvas API changed)
- **Capabilities enabled:** (visual consistency only; no new P-XXX)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/dashboard/ConstellationCanvas.tsx`
- **TauRPC delta:** consumer-side
- **Broadcast topics delta:** consumer-side
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (e2e dashboard constellation re-verification), a11y-plan (re-verify)
- **Summary:** Dashboard constellation map's per-service Halo dots consume new HaloCanvas API. Per-service `cumulativeSeverity` derived from incidents scoped to that service. Per-service `activityState` from baseline keeper. Cascade-only; no design changes beyond data wiring.

---

## Phase 10 — Output channels

### #92 — MCP server + tool exposure

- **Depends on:** #78 (incident records), #87 (Report content + markdown serialization)
- **Capabilities enabled:** P-039 (MCP Delivery When Configured), P-040 (MCP Independence — verified by configurable off-by-default)
- **Distillation layer:** L5 channel
- **Crates touched:** NEW `crates/mcp-server/` OR extension of existing rmcp integration
- **TauRPC delta:** +1 procedure `mcp.status()` returns connected agent identity if any
- **Broadcast topics delta:** none
- **Workspace deps delta:** verify existing `rmcp` is sufficient for server-side
- **Arch registry delta:** maybe +1 crate, +1 TauRPC procedure, **§Established Decisions: MCP is one of three equal-tier output channels, not coupling**
- **Specialist plan touches:** arch, security (MCP server exposes only configured tools, no privilege escalation paths), test-plan (MCP delivery content identical to clipboard markdown from P-038; "Send to agent" button visible only when configured + connected; pulse fully functional without MCP per P-040)
- **Summary:** MCP server exposes four tools: `query_incident_list`, `retrieve_report(id)`, `retrieve_telemetry_slice(id)`, `mark_incident_resolved(id)`. Server enabled via `[mcp.enable]` config (default false). Connected agent identity tracked, "Send to agent" button in Report toolbar visible only when both configured AND agent connected. Identical markdown content to clipboard delivery.

### #93 — Export for community training

- **Depends on:** #69 (corpus), #78 (incident records), #47 PII scrubbing (already enforced at ingestion)
- **Capabilities enabled:** P-046 (Export for Community Training)
- **Distillation layer:** L5 channel
- **Crates touched:** `crates/corpus/`, `pulse-app/ui/settings/Storage.tsx`
- **TauRPC delta:** +1 procedure `storage.export_for_training(target_path)` returns preview summary + JSONL file
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** +1 TauRPC procedure
- **Specialist plan touches:** security (export content review enforcement before submission), test-plan (export produces expected JSONL; preview displays summary before write; no transmission without explicit user confirmation)
- **Summary:** Settings → Storage → "Export for community training" action produces JSONL dump from corpus: anonymized trigger context, model interpretation, user feedback, resolution outcome per record. Preview dialog displays summary of what will be exported (count by category, date range, anonymization confirmation) before file write. Default target path is `~/Downloads/pulse-corpus-export-{timestamp}.jsonl`. No automatic submission to external endpoints — user manually shares the file.

---

## Phase 11 — Operations

### #94 — Configuration hot reload + prospective threshold application

- **Depends on:** #62, #63, #64, #67, #80, #82 (chunks producing config-bearing params)
- **Capabilities enabled:** P-055 (Configuration Hot Reload), P-056 (Prospective Threshold Application)
- **Distillation layer:** cross-cutting
- **Crates touched:** NEW `crates/config-watcher/` (or extension of existing config module), `crates/triage/` (consumer hooks)
- **TauRPC delta:** +1 procedure `config.reload()` (manual trigger), +1 `config.status()` (last-reload timestamp + any errors)
- **Broadcast topics delta:** +1 `pulse://stream/config-events` (reload, restart-required notice)
- **Workspace deps delta:** +`notify` (filesystem watcher)
- **Arch registry delta:** maybe +1 crate, +2 TauRPC procedures, +1 broadcast topic
- **Specialist plan touches:** test-plan (hot-reload param applies on next L1a tick; restart-required param raises notice without applying; prospective threshold change does not retroactively trigger cues; opt-in backfill action triggers retrospective evaluation), security (config validation: positive thresholds, multipliers in allowed ranges, file path safety)
- **Summary:** Filesystem watcher on `~/.andromeda-pulse/config.toml` with 500ms debounce. Hot-reloadable keys (thresholds, cadences, severity rules, suppression, lifecycle boundaries) apply within 2s. Restart-required keys (Drain params, model path, ports, storage path) raise notice in Diagnostics. **Prospective application:** threshold changes do not reset baselines and do not retroactively re-evaluate rolling windows. Opt-in `diagnostics.reevaluate_recent_window()` action for users wanting retrospective application. Malformed config rejected with previous valid config retained.

### #95 — Settings → Diagnostics view

> Brings together all L6 self-observability metrics from prior chunks into single user-facing surface.

- **Depends on:** all prior chunks producing L6 metrics (#59, #61-#67, #79-#85)
- **Capabilities enabled:** P-058 (Pipeline Self-Observability — full surface)
- **Distillation layer:** L6 surface
- **Crates touched:** `pulse-app/ui/settings/Diagnostics.tsx` (NEW), `crates/corpus/` (metric history queries)
- **TauRPC delta:** +1 procedure `diagnostics.snapshot()` returns current pipeline state, +1 `diagnostics.history(metric_name, window)` returns time-series
- **Broadcast topics delta:** consumer-side (subscribes to `pulse://stream/pipeline-health`)
- **Workspace deps delta:** none
- **Arch registry delta:** +2 TauRPC procedures
- **Specialist plan touches:** design-system (Diagnostics panel layout, metric card components, sparkline visualization), layout-templates (Diagnostics section organization: Connection, Model, Pipeline, Hardware, Templates), a11y-plan (metric values accessible, history charts have text alternatives), test-plan (self-reporting accuracy: inject known pipeline behaviors, verify Diagnostics reflects)
- **Summary:** Settings → Diagnostics view per dist-arch v3 §Layer 6. Sections: Connection (state, last-span-ago history), Model (active tier, hardware profile, inference success rate, queue depth, backoff state with countdown if active), Pipeline (per-layer metrics from L0-L5), Hardware (profile detection details), Templates (Drain template distribution from #67). All read-only initially per P-051. 30-day metric history default retention from corpus `pipeline_metrics` table.

---

## Phase 12 — Reflection (optional, defer-friendly)

### #96 — Background reflection cadence

> Lower priority than active interpretation. Drop from v0.2.0 if scope tight.

- **Depends on:** #81 (digest infrastructure), #83 (LLM inference)
- **Capabilities enabled:** prerequisite for future trend-detection capabilities; supports P-044 with broader-window context
- **Distillation layer:** L3 + L4 (long-window mode)
- **Crates touched:** `crates/triage/digest`, `crates/interpretation/`
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (reflection digest distinguishable via `digest_kind` field; L4 prompt for reflection emphasizes trend analysis; L5 surfaces reflection incidents with default-Curious severity unless model identifies high-confidence pattern), obs-plan (reflection cadence metrics)
- **Summary:** Cadence Coordinator (#80) emits reflection-mode trigger every 1800s (configurable). Digest assembler builds 30-minute-window digest instead of 60s slice. L4 prompt for reflection mode emphasizes cumulative trend analysis over acute interpretation. L5 surfacing rules: reflection-produced incidents default to Curious severity unless model identifies high-confidence pattern warranting Suggested or higher. Cancellation: reflection skipped (not queued) if LLM busy with higher-priority work.

---

## Phase 13 — Finalization

### #97 — A11y + perf re-verify + capability coverage check

- **Depends on:** all previous chunks
- **Capabilities enabled:** verifies all P-001 through P-060 actually hold
- **Distillation layer:** all
- **Crates touched:** e2e tests, a11y audit harness
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (P-test coverage update for redesigned widget; full capability spec verification matrix; load testing per dist-arch v3 testability strategy: baseline 1k spans/s, high 10k spans/s, burst 50k spans/s, sustained extreme), a11y-plan (full audit pass), obs-plan (frame p99 ≤33ms re-verification, end-to-end latency budgets per profile)
- **Summary:** Final verification pass. Each P-XXX capability has at least one Conductor scenario verifying it. Load testing at four profiles confirms performance budgets hold. A11y audit re-runs over redesigned widget. Frame p99 ≤33ms holds. Anomaly-detection latency budgets per hardware profile (Tier-1 <5s on gpu-primary, etc.) verified. Tray + window state interactions still work. **This chunk is the gate for v0.2.0 tag.**

---

## Summary

**Total chunks:** 41 (#57 through #97), plus 2 pre-flight decisions (Pre-D1, Pre-D2). 33 original v2 chunks + 8 new consolidation chunks (#70-#77) inserted in v3 per `pulse-v0_2_0-consolidation-audit-2026-05-19.md`.

**Phase breakdown:**
- Phase 0 (Foundation): 2 chunks
- Phase 1 (Connection): 1 chunk
- Phase 2 (Algorithmic detection): 7 chunks
- Phase 3 (Log templates): 1 chunk
- Phase 4 (Service lifecycle): 1 chunk
- Phase 5 (Corpus foundation): 1 chunk (tightened from 2 in v2 — Incident records moved to Phase 7)
- Phase 6 (Consolidation): 8 chunks [NEW in v3]
- Phase 7 (Incident records + digest pipeline): 4 chunks (merged from v2 Phase 5 §70 + v2 Phase 6 §71-§73)
- Phase 8 (LLM interpretation): 4 chunks
- Phase 9 (Surfaces): 6 chunks
- Phase 10 (Output channels): 2 chunks
- Phase 11 (Operations): 2 chunks
- Phase 12 (Reflection — optional): 1 chunk
- Phase 13 (Finalization): 1 chunk

**Crates added:** approximately 5-6 (`curation`, `triage`, `corpus`, `interpretation`, possibly `mcp-server`, possibly `config-watcher`). Workspace goes from 10 to 15-16 members. Some new crates may be folded into existing crates depending on implementation taste. Consolidation chunks #70/#71 add NEW adapter modules in `pulse-app/src/` (baseline_persistence.rs, lifecycle_persistence.rs, storm_persistence.rs) but no new workspace crates.

**TauRPC procedures added:** approximately 15-18 across chunks:
- connection (#59): 1
- diagnostics (#67, #85, #95): 4
- incidents (#78, #86, #87): 5
- services (#68): 1
- storage (#69, #93): 3
- model (#82): 1
- mcp (#92): 1
- config (#94): 2
- Bindings.ts regeneration discipline: bundle TauRPC additions within phases to minimize regen tax

**Broadcast topics added:** approximately 8:
- connection-state (#59)
- attention-cues, cadence-triggers (#62)
- restart-events (#63)
- service-lifecycle (#68)
- incidents (#78)
- cadence-events (#80)
- digests (#81)
- model-status (#82)
- config-events (#94)

**Workspace dep additions:** chosen LLM runtime (`mistralrs` OR `candle`), `tokenizers`, `tdigest`, `dashmap`, `bincode`, `rusqlite`, `age` (or platform keychain integration), `notify`, possibly `regex` if not present.

**Schema additions (DuckDB + new SQLite corpus):**
- DuckDB: `log_templates` table, `logs.template_id` column, `span_events.fingerprint` column
- Corpus SQLite: `baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive` tables (chunks #70/#71 populate previously-empty `baseline_state` + `service_registry` slots via consolidation work)

**Capability coverage:** All 60 P-XXX capabilities from `pulse-capability-spec.md` v2 enabled by chunk completion. Verify via #97 capability coverage check. **Consolidation chunks #70-#77 specifically close spec-vs-impl drift surfaced by the audit (P-001/P-003/P-006/P-009/P-010/P-011/P-012/P-013/P-014/P-027/P-047/P-048/P-051 — see audit deliverable Section 1.D for full PARTIAL → LIVE transition list).**

---

## Capability-to-chunk mapping (quick reference)

> **v3 update:** chunk numbers reflect v3 numbering after consolidation insertion. Old stale entry "P-019 to P-023, P-060 | #67 superseded by #72-#77" from v2 (which pointed at v1 #67 Severity classifier, semantically broken after v2 chunk #67 became Drain) is replaced with accurate v3 mapping below. Consolidation chunks #70-#77 close spec drift for capabilities listed in their respective rows.

| Capability | Primary chunks |
|---|---|
| P-001 to P-004 | #59, **#73** (numeric alignment: P-001 thresholds 10s/60s, P-003 panic hook propagation) |
| P-005 to P-008 | #65, #66, baseline always-on, **#72** (P-006 PII scrub for exception.message ingestion) |
| P-009 to P-014 | #61, #62, #64, **#70** (BaselineState corpus persistence closes P-009/P-011/P-013 SHALL-persisted-to-corpus drift), **#73** (P-010/P-011/P-012/P-014 numeric/identity alignment — baseline-relative semantics, operation identity preservation, percentile p99, 30s floor) |
| P-015, P-016 | #63 |
| P-017, P-018 | #65, #66, **#71** (RetryStormState corpus persistence — closes chunk #66 storm.rs deferred-to-#69 broken promise) |
| P-019, P-020 | #60 (contract types), #83 (LLM severity decision side) |
| P-021 | #62 (emitter) |
| P-022, P-023 | #78 (incident lifecycle), #85 (resolution summary path) |
| P-024 to P-030 | #86, #88, #89, #90, #91 |
| P-031 to P-036 | #81 (digest precursor), #83 (LLM output), #87 (Report) |
| P-037 to P-040 | #87 (Report surface), #92 (MCP delivery) |
| P-041 to P-046 | #69 (corpus scaffold), #78 (incident records), #93 (export), **#70** (baseline_state via corpus — completes P-041 schema usage), **#71** (service_registry via corpus) |
| P-047 to P-051 | #69, #95, **#72** (PII scrubber extension — closes single-site coverage gap for P-047/P-048), **#73** (P-049 passphrase fallback decision), **#74** (P-051 delete TauRPC + storage panel coverage), **#70** (P-051 baseline visibility via corpus inspect) |
| P-052 | #80, #94 |
| P-053, P-054 | #82, #84 |
| P-055, P-056 | #94 |
| P-057 | #63 |
| P-058 | cross-cutting (all chunks instrument) + #95 surface |
| P-059 | #81, #85 |
| P-060 | #80, #82 |
| **Consolidation chunks (#70-#77)** | **#70 BaselineState→corpus; #71 ServiceRegistry+RetryStormState→corpus; #72 PII scrubber coverage extension; #73 Capability spec numeric alignment; #74 Architecture registry alignment batch; #75 Documentation consolidation; #76 Andromeda pipeline meta-improvements (P7+P12+P15-P18); #77 Specialist plan re-runs (security+tests)** |

---

## Open items to address as they arise (NOT chunks per se)

- Pre-flight rot warnings (Pre-E1, Pre-E2) resolve naturally as chunks touch affected sections; explicit pre-flight evolves likely unnecessary
- `multiple-versions = "deny"` posture in `deny.toml` requires updates after each workspace dep addition (resolve inline per chunk)
- bindings.ts regeneration discipline per chunk (existing testing.md 2026-05-13 entry covers it)
- Capability spec amendments via `/andromeda-evolve --allow-spec-amendment` when implementation reveals capability formulations needing refinement
- LOC estimates are planning baselines, not contracts. Components prone to expansion (state machines, failure handling, self-observability) may exceed by 30-50%; mental reserve recommended per dist-arch v3 closing notes

---

## Migration from original mini-route (chunks #57-#75)

Original mini-route v1 chunks #57 through #75 cover work that this route renumbers and reorganizes. Mapping:

| Original | This route | Status |
|---|---|---|
| #57 (Widget real-data binding) | #57 | Kept as-is |
| #58 (Curation crate extraction) | #58 | Kept as-is |
| #59 (Connection state machine) | #59 | Kept as-is |
| #60 (Triage scaffold) | #60 | Modified: AttentionCue contract types per P-021 paradigm |
| #61 (Streaming baseline trackers) | #61 | Modified: corpus persistence added per P-009 v2 |
| #62 (Baseline emitter + anomaly broadcast) | #62 | Renamed: now "Attention cue emitter" per P-021 paradigm |
| #63 (Restart event detector) | #63 | Modified: dual-condition bypass added per P-057 |
| #64 (Activity floor learning) | #64 | Modified: corpus persistence added per P-013 v2 |
| #65 (Span events ingestion) | #65 | Kept as-is |
| #66 (Exception fingerprinting + retry storm) | #66 | Kept as-is |
| #67 (Severity classifier + incident state store) | Split: #70 (incident records) + #73 (digest) + #75 (LLM severity) | Fundamentally restructured: rule-based classifier replaced by LLM-driven interpretation with rules as attention cues |
| #68 (Incident TauRPC + acknowledge persistence) | Split: #70 (incident records + acknowledge) + #78 (counter + dropdown) | Restructured: ad-hoc JSON acknowledge replaced by SQLite corpus |
| #69 (Header redesign) | #80 | Renumbered |
| #70 (Halo formula refactor) | #81 | Renumbered, depends on LLM severity from #75 instead of rule-based |
| #71 (Service constellation) | #82 | Renumbered, depends on service lifecycle from #68 |
| #72 (Incident card slot) | **DROPPED** | Three-surface design has no incident card embedded in widget; replaced by counter + dropdown (#78) |
| #73 (ConstellationCanvas cascade) | #83 | Renumbered |
| #74 (Calibration config surface) | #86 | Renumbered, expanded for hot reload + prospective application |
| #75 (A11y + perf re-verify) | #89 | Renumbered, expanded with capability coverage check + load testing |

**New chunks not present in v1:** #67 (Drain), #68 (Service lifecycle), #69 (Corpus scaffold), #71 (SQL queries scheduler), #72 (Cadence coordinator), #73 (Digest assembler), #74 (Hardware profile + model loading), #75 (Prompt scaffolding + primary inference), #76 (Fallback tier), #77 (JSON parse failure + resolution summary), #78 (Counter + dropdown — replaces v1 incident card), #79 (Report generation), #84 (MCP server), #85 (Export for training), #87 (Diagnostics view), #88 (Reflection cadence).

**Test infrastructure (T1, T2, T3 from v1):** Subsumed by Conductor work, which is a separate parallel track defined in `conductor-vision.md` (TODO: create as separate document). Conductor implementation chunks would form their own route, sized at 25-30% of pulse effort per earlier discussion.

---

## Migration from v2 (chunks #70-#89)

v3 inserts 8 new **Consolidation** chunks (#70-#77) as a new Phase 6 to address audit findings from `pulse-v0_2_0-consolidation-audit-2026-05-19.md` (7 HIGH-severity inconsistency clusters in chunks #57-#69). All v2 chunks #70-#89 shift +8 to accommodate; v2 chunk #70 (Incident records) additionally moves from v2 Phase 5 (Corpus foundation) to v3 Phase 7 (Incident records + digest pipeline) since the corpus scaffold and incident records work split semantically — corpus #69 is the foundation, incident records is downstream consumer.

| v2 # | v3 # | Status | Notes |
|---|---|---|---|
| (new) | #70 | NEW | Consolidation: BaselineState → corpus migration (audit Section 1.A; closes P-009/P-011/P-013 spec drift) |
| (new) | #71 | NEW | Consolidation: ServiceRegistry + RetryStormState → corpus migration (closes P-027 + broken chunk #66 promise) |
| (new) | #72 | NEW | Consolidation: PII scrubber coverage extension (closes single-site coverage gap; P-006/P-047/P-048) |
| (new) | #73 | NEW | Consolidation: Capability spec numeric alignment (P-001/P-003/P-010/P-011/P-012/P-014 PARTIAL→LIVE) |
| (new) | #74 | NEW | Consolidation: Architecture registry alignment batch (log_templates DuckDB + corpus schema sub-section + forward-promise cleanup) |
| (new) | #75 | NEW | Consolidation: Documentation consolidation (8 BROKEN + 4 STALE cross-references + arch narrative cascade) |
| (new) | #76 | NEW | Consolidation: Andromeda pipeline meta-improvements (Proposal 7 + Proposal 12 + file P15-P18) |
| (new) | #77 | NEW | Consolidation: Specialist plan re-runs (/andromeda-security + /andromeda-tests) |
| #70 (Incident records + lifecycle persistence) | #78 | Renumbered + phase moved | Was Phase 5; now Phase 7 — merged with digest pipeline chunks |
| #71 (SQL aggregation queries + scheduler) | #79 | Renumbered | Phase 6 → 7; Cadence Coordinator ref #72 → #80 in Summary |
| #72 (Cadence coordinator) | #80 | Renumbered | Phase 6 → 7; Summary refs #74 → #82, #86 → #94 |
| #73 (Digest assembler) | #81 | Renumbered | Phase 6 → 7; Depends #71 → #79, #70 → #78; Summary ref #74 → #82 |
| #74 (Hardware profile + model loading) | #82 | Renumbered | Phase 7 → 8; Summary refs #75 → #83, #87 → #95 |
| #75 (Prompt scaffolding + primary inference) | #83 | Renumbered | Phase 7 → 8; Depends #74 → #82, #73 → #81 |
| #76 (Fallback model tier) | #84 | Renumbered | Phase 7 → 8; Depends #75 → #83 |
| #77 (JSON parse failure + resolution) | #85 | Renumbered | Phase 7 → 8; Depends #75 → #83, #76 → #84, #70 → #78 |
| #78 (Findings counter + dropdown) | #86 | Renumbered | Phase 8 → 9; Depends #70 → #78, #73 → #81, #77 → #85; Summary ref #79 → #87 |
| #79 (Diagnostic Report generation) | #87 | Renumbered | Phase 8 → 9; Depends #75 → #83, #77 → #85; Summary ref #84 → #92 |
| #80 (Header redesign) | #88 | Renumbered | Phase 8 → 9 |
| #81 (Halo formula refactor) | #89 | Renumbered | Phase 8 → 9; Depends #75 → #83 |
| #82 (Service constellation rendering) | #90 | Renumbered | Phase 8 → 9; Depends #81 → #89 |
| #83 (ConstellationCanvas cascade) | #91 | Renumbered | Phase 8 → 9; Depends #81 → #89 |
| #84 (MCP server) | #92 | Renumbered | Phase 9 → 10; Depends #70 → #78, #79 → #87 |
| #85 (Export for community training) | #93 | Renumbered | Phase 9 → 10; Depends #70 → #78 |
| #86 (Config hot reload) | #94 | Renumbered | Phase 10 → 11; Depends #72 → #80, #74 → #82 |
| #87 (Settings → Diagnostics view) | #95 | Renumbered | Phase 10 → 11; Depends #71-#77 → #79-#85 |
| #88 (Background reflection cadence) | #96 | Renumbered | Phase 11 → 12; Depends #73 → #81, #75 → #83; Summary ref #72 → #80 |
| #89 (A11y + perf re-verify) | #97 | Renumbered | Phase 12 → 13 |

**Driver:** Consolidation audit deliverable `docs/v0_2_0/pulse-v0_2_0-consolidation-audit-2026-05-19.md` (session 102) identified 7 HIGH-severity inconsistency clusters in shipped chunks #57-#69 requiring closure before user-facing Phase 7+ work builds on inconsistent foundation. User-explicit consistency-first standard ("two ways of doing X where one should suffice = HIGH severity") drove the consolidation-before-Incidents sequencing.

---

## Changelog

### v3 — 2026-05-20

Mid-stream consolidation insertion based on `pulse-v0_2_0-consolidation-audit-2026-05-19.md` findings (7 HIGH-severity inconsistency clusters in chunks #57-#69).

**Inserted:** Phase 6 — Consolidation (§70-§77, 8 new chunks) addressing audit findings:

- §70 BaselineState → corpus migration (persistence consolidation A1; closes spec drift P-009 + P-013 + P-051 inspect surface)
- §71 ServiceRegistry + RetryStormState → corpus migration (closes P-027 dot-positions-survive-restart + broken chunk #66 storm.rs deferred-persistence promise)
- §72 PII scrubber coverage extension (closes single-site coverage gap; uniform coverage at OTLP appender + log_records + span_events + Drain corpus path; CorpusWriter trait contract pre-scrub invariant)
- §73 Capability spec numeric alignment (P-001 thresholds, P-003 panic hook, P-010/P-012 baseline-relative, P-011 operation identity, P-012 percentile, P-014 floor)
- §74 Architecture registry alignment batch (log_templates DuckDB table, corpus SQLite schema sub-section, forward-promise cleanup)
- §75 Documentation consolidation (8 BROKEN + 4 STALE cross-references including arch narrative cascade)
- §76 Andromeda pipeline meta-improvements (implement Proposal 7 + Proposal 12; file P15-P18)
- §77 Specialist plan re-runs (`/andromeda-security` + `/andromeda-tests`)

**Reorganized:**

- Phase 5 tightened to §69 only (Corpus SQLite scaffold)
- Phase 6 (Consolidation) inserted as new
- Phase 7 (Incident records + digest pipeline) merges old Phase 5 §70 Incidents with old Phase 6 §71-§73 digest pipeline
- Phases 7-12 renamed to 8-13

**Renumbered:** §70-§89 → §78-§97 (shift +8); see §Migration from v2 (chunks #70-#89) above for full mapping.

**Capability-to-chunk mapping table:** fixed stale row "P-019 to P-023, P-060 | #67 superseded by #72-#77" pointing at v1 chunk #67 (Severity classifier) which became v2 chunk #67 Drain — semantically broken since v2 commit. Now points at correct v3 chunks per actual semantics (#83 LLM severity, #80 + #82 cadence + hardware profile). Also added consolidation row covering chunks #70-#77.

**Approach:** Andromeda Refuse 6 (in `.andromeda/route.md §2` Andromeda-managed) forbids mid-route insertion — consolidation chunks land there as Form 1 terminal appends in Epoch 9 (sequential #70-#77, no renumbering needed on Andromeda side). pulse-v0_2_0-route.md is project-internal planning doc with v1→v2 manual-edit-with-migration-table precedent; v2→v3 follows same pattern. The two route docs intentionally diverge in chunk numbering at #67-#69 already (per session-learnings 2026-05-18+19) — this consolidation widens the divergence at #70+.

**Driver:** consolidation audit findings; user-explicit consistency-first standard ("two ways of doing X where one should suffice = HIGH severity"); strategic pause before user-facing surfaces (Phase 7+) build on consolidated foundation.

### v2 — 2026-05-14

Major restructuring of the original `widget-state-validation-mini-route.md` v1 to align with `pulse-capability-spec.md` v2 (60 P-XXX capabilities across 11 categories) and `pulse-distillation-architecture.md` v3 (six-layer pipeline with self-observability).

**Renamed:** `widget-state-validation-mini-route.md` → `pulse-v0_2_0-route.md`. Scope expanded beyond widget state validation to full v0.2.0 implementation including LLM interpretation, corpus persistence, three-surface communication, MCP integration, and pipeline operations.

**Structural changes:**

- 19 chunks → 33 chunks (#57-#89)
- 4 phases → 12 phases (more granular for tracking)
- Added 2 blocking decisions (Pre-D1 LLM runtime, Pre-D2 Drain spike)
- Added capability-to-chunk mapping table
- Added migration table from v1 chunk numbering
- Each chunk now includes "Capabilities enabled" and "Distillation layer" fields for traceability

**Behavioral changes affecting existing chunks:**

- #60 — Contract types now AttentionCue per P-021 paradigm, not generic anomaly types
- #61 — Added corpus persistence for EWMA and rolling window state (P-009 v2)
- #62 — Renamed from "anomaly emitter" to "attention cue emitter" reflecting P-021 paradigm shift
- #63 — Added dual-condition magnitude bypass per P-057
- #64 — Added corpus persistence for activity histogram (P-013 v2)
- #67 (v1) — Rule-based classifier fundamentally restructured: rules now produce attention cues (P-021), severity decision moves to LLM (P-020) in new #75
- #68 (v1) — Acknowledge persistence moves from JSON file to full SQLite corpus in #70
- #72 (v1) — Incident card slot DROPPED; replaced by counter + dropdown (#78) per three-surface design
- #74 (v1) — Calibration config surface expanded to hot reload + prospective threshold application (#86)
- #75 (v1) — Finalization expanded with capability coverage check + load testing (#89)

**New chunks (not in v1):**

- #67 — Drain Rust implementation + template profiling diagnostics (L1c)
- #68 — Service registry + lifecycle state machine
- #69 — Corpus SQLite + encryption + PII scrubber
- #71 — SQL aggregation queries + scheduler (L1a)
- #72 — Cadence coordinator + three-tier triggering
- #73 — Digest assembler + LWW queue + active-incident exception (L3)
- #74-#77 — LLM interpretation layer (L4): hardware profile, primary inference, fallback tier, failure handling
- #78 — Findings counter + dropdown (replaces v1 incident card slot)
- #79 — Diagnostic Report generation
- #84 — MCP server + tool exposure
- #85 — Export for community training
- #87 — Settings → Diagnostics view (L6 surface)
- #88 — Background reflection cadence (optional)

**Removed from scope:**

- v1 chunk #72 (Incident card slot) — three-surface design has no card in widget

**Capability coverage:**

- v1 enabled approximately 20-25 P-XXX capabilities (informal, capability spec didn't exist yet at v1 time)
- v2 enables all 60 P-XXX capabilities from spec v2

### v1 — 2026-05-14 (earlier the same day, as `widget-state-validation-mini-route.md`)

Initial continuation route from pulse 56/56 baseline. 19 chunks (#57-#75) covering widget data binding, curation extraction, connection state, baseline trackers, restart detection, activity floor learning, span events, retry storm, rule-based classifier, incident state, header redesign, halo refactor, service constellation, incident card slot, dashboard cascade, calibration config, a11y/perf re-verify. Test infrastructure tracks T1-T3 outside route as xtask additions.
