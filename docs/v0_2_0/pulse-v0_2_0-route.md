# Pulse v0.2.0 Implementation Route

**Status:** Foundational baseline for v0.2.0 implementation kickoff
**Last revised:** 2026-05-14
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

**Why blocking:** Chunks #74-#77 (L4 LLM interpretation layer) depend on chosen runtime's API patterns, error types, and dependency tree.

**Decision criteria:**
- Maturity at implementation time
- Quantization format support (GGUF essential)
- Backend coverage (Metal critical for M-series Mac, CUDA for NVIDIA)
- JSON-constrained generation support (essential)
- Tokenizer integration ergonomics
- Memory management quality

**Decision deadline:** Before chunk #74.

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
- **Summary:** Background tick task (1-2s) reads all trackers, evaluates thresholds (3.0× error rate multiplier, 2.5× latency multiplier — calibration values loaded from config), emits `AttentionCue` to broadcast with `priority_tier` classification (Hard / Medium / Baseline based on confidence and magnitude). Tier-2 cues additionally emit to `cadence-triggers` channel for Cadence Coordinator (#72). **Threshold multipliers loaded from config or hardcoded defaults for now; hot-reload wiring added in #86.**

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

> **Blocked by Pre-D2** (Drain spike validation). Largest single component in route. Don't start without spike confirmation of estimate.

- **Depends on:** #65 (logs table schema), Pre-D2 (Drain spike validation)
- **Capabilities enabled:** P-007 (High-Severity Log Capture — template-aware aggregation enables Q6)
- **Distillation layer:** L1c (Drain assignment) + L6 (template profiling surface)
- **Crates touched:** `crates/buffer/drain.rs` (NEW), `crates/buffer/appender.rs` (assignment integration), `pulse-app/ui/diagnostics/TemplateDistribution.tsx` (NEW)
- **TauRPC delta:** +1 procedure `diagnostics.template_distribution()` returns top-N templates with sample messages
- **Broadcast topics delta:** none
- **Workspace deps delta:** maybe `regex` if not already present (used for masking patterns)
- **Arch registry delta:** +1 TauRPC procedure, new schema table `log_templates`, +1 column `logs.template_id`
- **Specialist plan touches:** arch (schema additions), test-plan (golden-file tests with LogHub corpus subset, parameter sensitivity tests for depth/similarity), obs-plan (`pipeline.l1c.drain_template_count_total`, `pipeline.l1c.drain_assignment_latency_p99_microseconds`), security (template content goes through PII scrubber)
- **Summary:** Rust port of Drain3 algorithm (fixed-depth parse tree, similarity threshold matching, template extraction with parameter masking). Estimated 1000-1500 LOC pending Pre-D2 validation. Settings → Diagnostics exposes "Template Distribution" panel: top-50 templates with sample messages, occurrence counts, drift indicators. Users tune `[triage.drain.depth]` (default 4), `[triage.drain.similarity]` (default 0.5), `[triage.drain.max_clusters]` (default 1000) based on observed pathology. Drain params require restart per P-055 (template tree invalidation).

---

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
- **Summary:** Create `crates/corpus/` with SQLite-based persistent storage. Schema per dist-arch v3 §Schema Additions. Encryption at rest using OS keychain key (macOS Keychain, Linux Secret Service, Windows DPAPI) or user passphrase fallback. First-run setup prompts for keychain access. PII scrubber consolidated as security crate primitive consumed by corpus on ingestion. Settings → Storage section enables inspection (read-only), export trigger (per P-046 — implemented in #85), and deletion with explicit confirmation.

### #70 — Incident records + lifecycle persistence

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

---

## Phase 6 — Digest pipeline

This phase builds the L1a → L2 → L3 cadence machinery that produces LLM-ready digests. No surfaces appear from this phase alone — surfaces appear after L4 (#75) consumes digests.

### #71 — SQL aggregation queries + scheduler

- **Depends on:** #58 (curation), #67 (log templates available), #66 (fingerprints available)
- **Capabilities enabled:** prerequisite for P-020, P-021 (algorithmic detection requires aggregated input)
- **Distillation layer:** L1a
- **Crates touched:** `crates/triage/baseline` (SQL templates module), `crates/buffer/` (query execution helpers if needed)
- **TauRPC delta:** none yet
- **Broadcast topics delta:** none (results consumed in-process by Cadence Coordinator #72)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** arch, test-plan (time-injected unit tests per query Q1-Q7; Q7 worst-case bounds verified with broad trace fixture), obs-plan (`pipeline.l1a.query_count_total{query_name}`, `pipeline.l1a.query_latency_p99_milliseconds{query_name}`, `pipeline.l1a.q7_timeout_count_total`, `pipeline.l1a.q7_fallback_count_total`)
- **Summary:** SQL templates Q1-Q7 per dist-arch v3 §Appendix A executed against L0 ring buffer. Q7 includes explicit bounds (`LIMIT 100`, `LIMIT 10` depth, 200ms timeout, fallback to shallow non-recursive query). Scheduler invocation hook to be wired by Cadence Coordinator (#72).

### #72 — Cadence coordinator + three-tier triggering

- **Depends on:** #62 (attention cues + cadence-triggers channel), #71 (queries to invoke)
- **Capabilities enabled:** P-052 (Cadence Configuration), P-060 (Tiered Triggering Priority)
- **Distillation layer:** L2 routing + L3 invocation
- **Crates touched:** `crates/triage/cue` (extends), NEW `crates/triage/cadence` module
- **TauRPC delta:** none yet
- **Broadcast topics delta:** +1 `pulse://stream/cadence-events` (for L6 visibility)
- **Workspace deps delta:** none
- **Arch registry delta:** +1 broadcast topic
- **Specialist plan touches:** test-plan (Tier-1 hard signal → immediate L3 + L4 invocation; Tier-2 cue → accelerated next cadence; Tier-3 baseline cadence; cpu-primary disables Tier-2 acceleration), obs-plan (`pipeline.l3.digests_assembled_total{mode}`)
- **Summary:** Three-tier coordinator per dist-arch v3 §Cadence and Event Triggers. Default cadence 60s baseline, 20s accelerated, 1800s reflection. Cadence values loaded from `[triage.cadence]` config with hot-reload support (delivered in #86). Safety floor enforcement: baseline ≥ 5s, reflection ≥ 5min. Tier-2 acceleration conditionally disabled when hardware profile is `cpu-primary` (set by #74). Coordinator schedules L1a query execution + L3 digest assembly + L4 inference invocation.

### #73 — Digest assembler + LWW queue + active-incident exception

- **Depends on:** #71 (L1a outputs), #62 (attention cues), #66 (fingerprints), #67 (templates), #69 (corpus retrieval), #70 (active incident state)
- **Capabilities enabled:** P-031 foundation (Report Structure — digest is precursor), P-032 (Project Context Grounding), P-044 (Retrieval-Augmented Interpretation), P-059 (Active-Incident Interpretation Continuity)
- **Distillation layer:** L3
- **Crates touched:** `crates/triage/digest`, `crates/workspace-detector` (consumer)
- **TauRPC delta:** none yet
- **Broadcast topics delta:** +1 `pulse://stream/digests` (LWW queue for L4), corpus appends (all digests, no LWW)
- **Workspace deps delta:** +`tokenizers` (Hugging Face tokenizer library for token counting)
- **Arch registry delta:** +1 broadcast topic, +1 dep
- **Specialist plan touches:** arch, test-plan (**golden-file tests for digest composition** per dist-arch v3 testability strategy; LWW behavior; active-incident exception; corpus retrieval), obs-plan (`pipeline.l3.digest_token_count_p99`, `pipeline.l3.lww_drop_count_total`, `pipeline.l3.active_incident_queue_depth`)
- **Summary:** Digest assembler composes structured digest per dist-arch v3 §Appendix C from L1a outputs, L2 cues, project context (workspace path, framework signals, recent git activity from filesystem inspection), corpus retrieval (top-3 similar past incidents in workspace via fingerprint match). Token budget 500-2000, hard cap 3000, counted via `tokenizers` matched to active model (set by #74). **Queue behavior:** LWW for cadence-mode by default; active-incident exception bypasses LWW for cadence digests when L5 has unresolved incident with severity ≥ Suggested; Tier-1 hard signals never LWW-replaced; all digests written to corpus regardless of queue state.

---

## Phase 7 — LLM interpretation

> **Phase blocked by Pre-D1** (LLM runtime choice). Chunks #74-#77 are the highest-risk chunks in route — LLM integration brings dependencies, hardware quirks, and runtime maturity questions that historical practice resolves better than upfront analysis.

### #74 — Hardware profile detection + model loading + tokenizer

- **Depends on:** Pre-D1 (runtime choice resolved)
- **Capabilities enabled:** P-053 (Fallback Model Tier — detection side), P-054 (Hardware Profile Awareness)
- **Distillation layer:** L4 foundation
- **Crates touched:** NEW `crates/interpretation/` (or named per runtime choice), `crates/triage/cadence` (cpu-primary tier-2 disable wiring)
- **TauRPC delta:** +1 procedure `model.current_profile()` returns profile + tier + model identity
- **Broadcast topics delta:** +1 `pulse://stream/model-status` (load/unload/error events)
- **Workspace deps delta:** +chosen LLM runtime (`mistralrs` or `candle`)
- **Arch registry delta:** +1 crate, +1 TauRPC procedure, +1 broadcast topic, **§Established Decisions: LLM runtime choice with rationale** (per Pre-D1)
- **Specialist plan touches:** arch (Established Decision entry), test-plan (profile detection on each hardware configuration; cpu-primary detection triggers one-time notice exactly once), security (model file integrity, no model auto-download without user consent), obs-plan (`pipeline.l4.hardware_profile_active{profile}`, `pipeline.l4.model_tier_active{tier}`)
- **Summary:** Hardware profile detection on startup classifies into `gpu-primary`, `gpu-fallback`, `cpu-primary`, `cpu-fallback`. Model loading via chosen runtime with tokenizer paired to checkpoint. Tokenizer reload on model change (handled in #75). On `cpu-primary` profile, emit one-time notice with "Switch to fallback" / "Keep current" options; record decision so notice doesn't repeat. Profile and tier visible in Settings → Diagnostics → Model (frontend in #87).

### #75 — Prompt scaffolding + JSON schema + primary tier inference

- **Depends on:** #74 (model loaded), #73 (digest produced)
- **Capabilities enabled:** P-019 (Three-Tier Severity Model — model decision side), P-020 (Model-Driven Severity Decision — primary tier), P-033 (Ranked Hypothesis Generation — primary tier), P-034 (Suggested Investigation Steps — primary tier)
- **Distillation layer:** L4
- **Crates touched:** `crates/interpretation/` (prompt + inference modules)
- **TauRPC delta:** none yet (consumed via incidents stream from #70)
- **Broadcast topics delta:** none new (writes to existing incidents stream)
- **Workspace deps delta:** none
- **Arch registry delta:** none (uses crate added in #74)
- **Specialist plan touches:** test-plan (schema validation tests with constructed JSON; LLM output quality covered by Conductor scenarios separately), obs-plan (`pipeline.l4.inferences_total{result}`, `pipeline.l4.inference_latency_p99_milliseconds`, `pipeline.l4.inference_queue_depth`)
- **Summary:** System prompt with role definition, project conventions, JSON output schema embedded. JSON-constrained inference via runtime's grammar enforcement. Output schema per dist-arch v3 §L4 output format with `schema_version`, `prompt_version`, `model_tier`, `hardware_profile`, `is_resolution_summary` fields. Provenance, not invalidation — old corpus records remain queryable with version annotations.

### #76 — Fallback model tier support

- **Depends on:** #75
- **Capabilities enabled:** P-053 (Fallback Model Tier — full)
- **Distillation layer:** L4
- **Crates touched:** `crates/interpretation/` (fallback prompt + reduced-quality output)
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (fallback-tier scenarios produce reduced output: single hypothesis, ≤2 investigation steps; Report displays fallback-tier indicator), security (fallback model file integrity equal to primary)
- **Summary:** Reduced-quality prompt and schema for 3-4B class models. Single hypothesis instead of ranked list. Up to 2 investigation steps instead of 5. Less specific project context grounding (~500-700 token system prompt vs primary's ~800-1000). Output JSON includes `model_tier: "fallback"` for downstream rendering. CPU inference fully supported at 3-8s typical latency.

### #77 — JSON parse failure handling + backoff + resolution summary

- **Depends on:** #75 (primary inference), #76 (fallback inference), #70 (incident lifecycle)
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

## Phase 8 — User-facing surfaces

This phase replaces the original mini-route's #69-#73 work with redesigned surfaces matching capability spec v2 three-surface architecture.

### #78 — Findings counter + dropdown + TauRPC + corpus integration

> **Replaces original mini-route #72** (incident card slot) — new three-surface design has no incident card embedded in widget.

- **Depends on:** #70 (incident records), #73 (digest produces incidents via #75), #77 (severity decided)
- **Capabilities enabled:** P-028 (Findings Counter), P-029 (Findings Dropdown), P-030 (No Interrupting Notifications)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/widget/FindingsCounter.tsx` (NEW), `pulse-app/ui/widget/FindingsDropdown.tsx` (NEW), `crates/ui-bridge/contract.rs`
- **TauRPC delta:** consumer-side mostly (uses procedures from #70); +1 `incidents.mark_all_read()`
- **Broadcast topics delta:** consumer-side (subscribes to `pulse://stream/incidents`)
- **Workspace deps delta:** none
- **Arch registry delta:** +1 TauRPC procedure
- **Specialist plan touches:** design-system (counter visual spec — circle with number, severity-colored; dropdown row visual spec — severity dot + title + relative time), layout-templates (counter position adjacent to widget; dropdown overlay positioning), a11y-plan (counter aria-label, dropdown keyboard navigation, focus management on row click), test-plan (counter derives correctly from corpus; persists across app restart; clears on Report-opening; "Mark all as read" action)
- **Summary:** Compact circular counter below widget showing count of unread active incidents in current workspace. Hidden when count is zero. Counter derived via corpus query on incident events and widget focus, no separate state file. Click expands dropdown listing unread incidents sorted by severity descending then chronologically: severity indicator + title + relative timestamp ("3m ago"). Row click opens Report (#79). Dropdown footer has "Mark all as read" action. **No OS notifications, no toasts, no modals** per P-030 invariant.

### #79 — Diagnostic Report generation (in-app + copy markdown)

- **Depends on:** #75 (LLM output to render), #77 (resolution summary path)
- **Capabilities enabled:** P-031 (Report Structure), P-035 (Anonymized Telemetry Excerpts), P-036 (Cross-Incident Pattern Reference), P-037 (In-App Report Surface), P-038 (Copy to Clipboard)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/report/Report.tsx` (NEW), `pulse-app/ui/report/ReportRenderer.tsx` (NEW), `crates/interpretation/markdown.rs` (NEW — markdown serialization)
- **TauRPC delta:** +1 procedure `incidents.get_report(id)` returns Report content + markdown serialization
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** +1 TauRPC procedure
- **Specialist plan touches:** design-system (Report visual spec — sections, hierarchy, typography), layout-templates (Report panel/window layout), a11y-plan (Report keyboard navigation, section anchors, copy shortcut), test-plan (six-section structure verified; fallback-tier reduced fidelity verified; copy markdown identical to MCP delivery format per P-038)
- **Summary:** In-app Report rendering in full-window panel or separate window. Six sections per P-031: symptom, timeline, hypotheses, investigation steps, evidence, project context. Section collapse for managing density. Telemetry excerpts respect PII scrubbing. "Previously seen" subsection when corpus matches exist per P-036. **Copy Report** action serializes complete Report as markdown to clipboard; format identical to MCP delivery (#84) for consistency. Reports in degraded mode omit hypotheses and investigation steps with explicit notice. Resolution summary section appears for Resolved incidents.

### #80 — Header redesign: connection dot + chrome cleanup

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

### #81 — Halo formula refactor

- **Depends on:** #75 (severity decisions from LLM), #59 (connection state)
- **Capabilities enabled:** P-025 (Halo Hue Encoding), P-026 (Halo Breathing Encoding)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/halo/HaloCanvas.tsx`, `pulse-app/ui/halo/lch.ts`, `pulse-app/ui/halo/shaders/halo.wgsl`, delete `pulse-app/ui/halo/error-rate-to-blur.ts` and `pulse-app/ui/halo/throughput-to-hz.ts`
- **TauRPC delta:** consumer-side
- **Broadcast topics delta:** consumer-side (subscribes to incidents)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** design-system (severity-to-blur mapping: Autonomous=16px, Suggested=12px, Curious=8px, None=4px; severity-to-hue Earth Blue → Alert Burgundy interpolated), a11y-plan (reduced-motion mapping for breathing model)
- **Summary:** Halo props change from `(errorRate: f32, throughputHz: f32)` to `(connectionState: ConnectionState, cumulativeSeverity: Severity, activityState: ActivityState)`. `cumulativeSeverity` = max severity of active incidents. **Breathing via opacity AND blur modulation only, NEVER scale** per P-026 invariant — 4-5s cycle quiet, down to 2s under active flow, ease-in-out. Hue interpolated on cumulative severity. Connection state grays out halo (separate visual axis from severity per P-004 orthogonality).

### #82 — Service constellation rendering

- **Depends on:** #61 (baseline trackers expose per-service activity), #68 (service lifecycle states), #81 (halo refactor settles canvas layout)
- **Capabilities enabled:** P-027 (Service Constellation Auto-Discovery)
- **Distillation layer:** L5
- **Crates touched:** `pulse-app/ui/widget/AggregatedBadgeCanvas.tsx` (replace), maybe new `ConstellationCanvas.tsx`
- **TauRPC delta:** consumer-side (uses procedure from #68)
- **Broadcast topics delta:** consumer-side (subscribes to service-lifecycle from #68)
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** design-system (per-service dot palette: brightness=activity, hue=error state; pseudo-random scatter seed convention), layout-templates (constellation positioning rules, up to 20 services rendered), a11y-plan (dot hover tooltips, keyboard navigation)
- **Summary:** Replace existing aggregated badge with actual constellation dots. One dot per service from `service_registry`. Brightness=recent activity from histogram, hue=per-service severity from incidents scoped to that service. Position: stable pseudo-random scatter with `seed = hash(service_name)` for cross-session consistency. Dormant services dimmed, Archived hidden. Hover tooltip shows service name + recent activity rate + current lifecycle state.

### #83 — ConstellationCanvas dashboard cascade

> Addresses cascade from original validation report §5.2 — dashboard constellation needs same halo API update.

- **Depends on:** #81 (HaloCanvas API changed)
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

## Phase 9 — Output channels

### #84 — MCP server + tool exposure

- **Depends on:** #70 (incident records), #79 (Report content + markdown serialization)
- **Capabilities enabled:** P-039 (MCP Delivery When Configured), P-040 (MCP Independence — verified by configurable off-by-default)
- **Distillation layer:** L5 channel
- **Crates touched:** NEW `crates/mcp-server/` OR extension of existing rmcp integration
- **TauRPC delta:** +1 procedure `mcp.status()` returns connected agent identity if any
- **Broadcast topics delta:** none
- **Workspace deps delta:** verify existing `rmcp` is sufficient for server-side
- **Arch registry delta:** maybe +1 crate, +1 TauRPC procedure, **§Established Decisions: MCP is one of three equal-tier output channels, not coupling**
- **Specialist plan touches:** arch, security (MCP server exposes only configured tools, no privilege escalation paths), test-plan (MCP delivery content identical to clipboard markdown from P-038; "Send to agent" button visible only when configured + connected; pulse fully functional without MCP per P-040)
- **Summary:** MCP server exposes four tools: `query_incident_list`, `retrieve_report(id)`, `retrieve_telemetry_slice(id)`, `mark_incident_resolved(id)`. Server enabled via `[mcp.enable]` config (default false). Connected agent identity tracked, "Send to agent" button in Report toolbar visible only when both configured AND agent connected. Identical markdown content to clipboard delivery.

### #85 — Export for community training

- **Depends on:** #69 (corpus), #70 (incident records), #47 PII scrubbing (already enforced at ingestion)
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

## Phase 10 — Operations

### #86 — Configuration hot reload + prospective threshold application

- **Depends on:** #62, #63, #64, #67, #72, #74 (chunks producing config-bearing params)
- **Capabilities enabled:** P-055 (Configuration Hot Reload), P-056 (Prospective Threshold Application)
- **Distillation layer:** cross-cutting
- **Crates touched:** NEW `crates/config-watcher/` (or extension of existing config module), `crates/triage/` (consumer hooks)
- **TauRPC delta:** +1 procedure `config.reload()` (manual trigger), +1 `config.status()` (last-reload timestamp + any errors)
- **Broadcast topics delta:** +1 `pulse://stream/config-events` (reload, restart-required notice)
- **Workspace deps delta:** +`notify` (filesystem watcher)
- **Arch registry delta:** maybe +1 crate, +2 TauRPC procedures, +1 broadcast topic
- **Specialist plan touches:** test-plan (hot-reload param applies on next L1a tick; restart-required param raises notice without applying; prospective threshold change does not retroactively trigger cues; opt-in backfill action triggers retrospective evaluation), security (config validation: positive thresholds, multipliers in allowed ranges, file path safety)
- **Summary:** Filesystem watcher on `~/.andromeda-pulse/config.toml` with 500ms debounce. Hot-reloadable keys (thresholds, cadences, severity rules, suppression, lifecycle boundaries) apply within 2s. Restart-required keys (Drain params, model path, ports, storage path) raise notice in Diagnostics. **Prospective application:** threshold changes do not reset baselines and do not retroactively re-evaluate rolling windows. Opt-in `diagnostics.reevaluate_recent_window()` action for users wanting retrospective application. Malformed config rejected with previous valid config retained.

### #87 — Settings → Diagnostics view

> Brings together all L6 self-observability metrics from prior chunks into single user-facing surface.

- **Depends on:** all prior chunks producing L6 metrics (#59, #61-#67, #71-#77)
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

## Phase 11 — Reflection (optional, defer-friendly)

### #88 — Background reflection cadence

> Lower priority than active interpretation. Drop from v0.2.0 if scope tight.

- **Depends on:** #73 (digest infrastructure), #75 (LLM inference)
- **Capabilities enabled:** prerequisite for future trend-detection capabilities; supports P-044 with broader-window context
- **Distillation layer:** L3 + L4 (long-window mode)
- **Crates touched:** `crates/triage/digest`, `crates/interpretation/`
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none
- **Specialist plan touches:** test-plan (reflection digest distinguishable via `digest_kind` field; L4 prompt for reflection emphasizes trend analysis; L5 surfaces reflection incidents with default-Curious severity unless model identifies high-confidence pattern), obs-plan (reflection cadence metrics)
- **Summary:** Cadence Coordinator (#72) emits reflection-mode trigger every 1800s (configurable). Digest assembler builds 30-minute-window digest instead of 60s slice. L4 prompt for reflection mode emphasizes cumulative trend analysis over acute interpretation. L5 surfacing rules: reflection-produced incidents default to Curious severity unless model identifies high-confidence pattern warranting Suggested or higher. Cancellation: reflection skipped (not queued) if LLM busy with higher-priority work.

---

## Phase 12 — Finalization

### #89 — A11y + perf re-verify + capability coverage check

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

**Total chunks:** 33 (#57 through #89), plus 2 pre-flight decisions (Pre-D1, Pre-D2)

**Phase breakdown:**
- Phase 0 (Foundation): 2 chunks
- Phase 1 (Connection): 1 chunk
- Phase 2 (Algorithmic detection): 7 chunks
- Phase 3 (Log templates): 1 chunk
- Phase 4 (Service lifecycle): 1 chunk
- Phase 5 (Corpus): 2 chunks
- Phase 6 (Digest pipeline): 3 chunks
- Phase 7 (LLM interpretation): 4 chunks
- Phase 8 (Surfaces): 6 chunks
- Phase 9 (Output channels): 2 chunks
- Phase 10 (Operations): 2 chunks
- Phase 11 (Reflection — optional): 1 chunk
- Phase 12 (Finalization): 1 chunk

**Crates added:** approximately 5-6 (`curation`, `triage`, `corpus`, `interpretation`, possibly `mcp-server`, possibly `config-watcher`). Workspace goes from 10 to 15-16 members. Some new crates may be folded into existing crates depending on implementation taste.

**TauRPC procedures added:** approximately 15-18 across chunks:
- connection (#59): 1
- diagnostics (#67, #77, #87): 4
- incidents (#70, #78, #79): 5
- services (#68): 1
- storage (#69, #85): 3
- model (#74): 1
- mcp (#84): 1
- config (#86): 2
- Bindings.ts regeneration discipline: bundle TauRPC additions within phases to minimize regen tax

**Broadcast topics added:** approximately 8:
- connection-state (#59)
- attention-cues, cadence-triggers (#62)
- restart-events (#63)
- service-lifecycle (#68)
- incidents (#70)
- cadence-events (#72)
- digests (#73)
- model-status (#74)
- config-events (#86)

**Workspace dep additions:** chosen LLM runtime (`mistralrs` OR `candle`), `tokenizers`, `tdigest`, `dashmap`, `bincode`, `rusqlite`, `age` (or platform keychain integration), `notify`, possibly `regex` if not present.

**Schema additions (DuckDB + new SQLite corpus):**
- DuckDB: `log_templates` table, `logs.template_id` column, `span_events.fingerprint` column
- Corpus SQLite: `baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive` tables

**Capability coverage:** All 60 P-XXX capabilities from `pulse-capability-spec.md` v2 enabled by chunk completion. Verify via #89 capability coverage check.

---

## Capability-to-chunk mapping (quick reference)

| Capability | Primary chunks |
|---|---|
| P-001 to P-004 | #59 |
| P-005 to P-008 | #65, #66, baseline always-on |
| P-009 to P-014 | #61, #62, #64 |
| P-015, P-016 | #63 |
| P-017, P-018 | #65, #66 |
| P-019 to P-023, P-060 | #67 superseded by #72-#77 |
| P-024 to P-030 | #78, #80, #81, #82, #83 |
| P-031 to P-036 | #73, #75, #79 |
| P-037 to P-040 | #79, #84 |
| P-041 to P-046 | #69, #70, #85 |
| P-047 to P-051 | #69, #87 |
| P-052 | #72, #86 |
| P-053, P-054 | #74, #76 |
| P-055, P-056 | #86 |
| P-057 | #63 |
| P-058 | cross-cutting (all chunks instrument) + #87 surface |
| P-059 | #73, #77 |
| P-060 | #72, #74 |

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

## Changelog

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
