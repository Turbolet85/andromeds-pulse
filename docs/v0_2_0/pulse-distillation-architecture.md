# Pulse Telemetry Distillation Architecture

**Status:** Draft v3, foundational baseline for v0.2.0 implementation kickoff
**Last revised:** 2026-05-14
**Reference documents:** `pulse-capability-spec.md` (product contract), `architecture.md` (system architecture), `widget-state-validation-mini-route.md` (delivery plan)

**Note on document lifecycle:** This document reaches foundational stability at v3. It is the agreed architectural baseline against which implementation chunks will start. Further revisions are expected, but they happen through `andromeda-evolve` when code meets reality — not through additional pre-implementation review iterations. Theoretical refinement past this point yields diminishing returns; remaining open questions are calibration questions that resolve through running real workload through Conductor scenarios, not through document polish.

---

## Purpose

This document specifies the telemetry distillation pipeline that transforms raw OTLP signal streams into LLM-consumable digests. It defines the multi-layer architecture, the responsibilities of each layer, the data flow between them, the algorithms and SQL templates that implement them, the performance envelope they operate within, and the operational concerns that govern their lifecycle.

This document is the foundational design for **how** Pulse's capabilities (defined in `pulse-capability-spec.md`) are realized in the data path. Where the capability spec answers "what does Pulse claim to do," this document answers "how does data flow from OTLP receiver to model interpretation efficiently enough to do that on a single laptop, and what happens when things don't go to plan."

---

## Why this document exists

A naive design "feed raw telemetry to local LLM, get interpretation" fails for three known reasons documented in recent academic and industry literature:

1. **Token explosion from verbosity.** Raw OTLP serialization carries repeated schema, nested keys, redundant resource attributes per span. A single minute of moderate traffic can consume tens of thousands of tokens, exceeding 16GB-class model context windows.

2. **Context rot.** Even when raw telemetry fits in context, models miss salient signals buried in volume — the "needle in haystack" failure mode. Output quality degrades as input size grows, even within nominal context limits.

3. **Numeric sequence reasoning weakness.** Time-series data tokenized as text consumes enormous token count while presenting LLMs with a representation they reason about poorly. P99 latency expressed as "240ms vs baseline 80ms (3.0× regression)" is more useful to a model than a list of 60 individual latency values.

The distillation pipeline addresses all three by reducing raw streams to bounded-size, structurally-organized digests before any model invocation.

---

## Design principles

**Multi-stage reduction.** Each layer reduces volume by at least an order of magnitude. Raw OTLP at thousands of events per second compresses through six layers to a digest of approximately 500-2000 tokens per model invocation.

**Streaming and bounded-memory at the foundation.** Layers 0-2 are continuously running infrastructure with memory footprint bounded by project compositional complexity (number of services, operations, exception types), not by event volume.

**Event-driven inference, not polling.** Layer 4 (LLM interpretation) is invoked on cadence intervals and triggered events, never as a function of raw event rate.

**SQL-first, Rust-state-second.** Stateless aggregations are expressed as SQL queries against DuckDB. Stateful tracking is Rust-side. This minimizes new code and leverages DuckDB's optimized columnar execution.

**Reuse existing primitives.** Pulse's `snapshot` crate primitives are reused via the planned `curation` crate extraction.

**Graceful degradation by construction.** Layers degrade independently. Failure of L4 (LLM) leaves L0-L3 functional and L5 surfaces hard signals. Failure of L1b (state corruption) does not block L1a SQL aggregations. Each layer has a defined failure mode and recovery path.

**Self-observable by design.** The pipeline emits metrics about its own operation through Layer 6. Operational health of the distillation pipeline itself is treated as a first-class concern, not bolted on later.

**Interpretation continuity for active incidents (NEW in v3).** Last-write-wins applies to background cadence digests but is suspended while an incident is active. Pulse interprets the evolution of active incidents, not just their final state.

---

## Pipeline overview

```
                    ┌──────────────────────────────────┐
                    │  OTLP Receivers (gRPC + HTTP)    │
                    └──────────────┬───────────────────┘
                                   │
                                   ▼  Raw spans/metrics/logs
                    ┌──────────────────────────────────┐
                    │  L0: DuckDB Ring Buffer          │
                    │      10-minute retention window  │
                    └──────────────┬───────────────────┘
                                   │
                                   ▼  Continuous append + L1c enrichment
                    ┌──────────────────────────────────┐
                    │  L1: Streaming Distillation      │
                    │                                  │
                    │  L1a: SQL aggregations           │
                    │  L1b: Stateful Rust trackers     │
                    │       (with corpus persistence)  │
                    │  L1c: Ingestion-time enrichment  │
                    └──────────────┬───────────────────┘
                                   │
                                   ▼  Compact state
                    ┌──────────────────────────────────┐
                    │  L2: Attention Cue Detectors     │
                    │      Three-tier triggering       │
                    │      Dual-condition bypass       │
                    └──────────────┬───────────────────┘
                                   │
                                   ▼  Attention cues
                    ┌──────────────────────────────────┐
                    │  L3: Digest Assembler            │
                    │      All digests → corpus        │
                    │      LWW for cadence,            │
                    │      no LWW for active incidents │
                    └──────────────┬───────────────────┘
                                   │
                                   ▼  Structured digest
                    ┌──────────────────────────────────┐
                    │  L4: LLM Interpretation Layer    │
                    │      Hardware-profile-aware      │
                    │      Per-incident failure scope  │
                    └──────────────┬───────────────────┘
                                   │
                                   ▼  Interpretation
                    ┌──────────────────────────────────┐
                    │  L5: Surface & Persist           │
                    └──────────────────────────────────┘

       ╔═══════════════════════════════════════════════╗
       ║  L6: Pipeline Self-Observability              ║
       ║  Cross-cutting metrics from every layer       ║
       ║  pulse://stream/pipeline-health               ║
       ╚═══════════════════════════════════════════════╝
```

---

## L0 — DuckDB Ring Buffer (existing)

**Status:** Existing infrastructure, no changes required beyond schema additions documented below.

**Responsibility:** Volatile storage of recent telemetry with 10-minute retention window. All raw OTLP signals (spans, metrics, logs, span_events) land here after receiver ingestion.

**Why DuckDB:** Embedded columnar analytical engine with SIMD-vectorized execution and built-in approximate aggregates (`APPROX_QUANTILE` via t-digest, `APPROX_COUNT_DISTINCT` via HyperLogLog). Single-file in-memory database that handles L1 SQL aggregation workloads without external service dependencies.

**Memory:** Approximately 100-200MB steady state for typical vibe-coder workload.

**Retention:** Fixed 10-minute rolling window. Distilled summaries in corpus preserve long-term history.

**Failure modes:** If DuckDB cannot accept inserts (disk pressure, lock contention), receiver applies backpressure. If query returns empty result (genuine quiet period vs query failure), L1a distinguishes via row count and re-checks via L6 health metrics.

---

## L1 — Streaming Distillation

### L1a — SQL Aggregations (cadence-driven)

**Invocation:** Periodic, default cadence 60 seconds, accelerated to 20 seconds following Tier-2 medium-priority cue (see Cadence and Event Triggers section). Triggered immediately on Tier-1 hard signal. Tier-2 acceleration disabled on CPU-only hardware (see Hardware Profile Matrix in L4).

**Implementation:** Pre-defined SQL templates parameterized by time window, executed against L0 ring buffer. Results returned as Arrow RecordBatches.

**Core queries:** See Appendix A (query templates Q1-Q7).

**Q7 worst-case bounds:** Recursive CTE for critical-path extraction has the following limits to prevent pathological execution times on broad traces:

- `LIMIT 100` on total spans in trace tree
- `LIMIT 10` on recursion depth
- 200ms wall-clock timeout
- Fallback to non-recursive shallow path query (immediate child spans of root only) if either limit or timeout is hit
- L6 emits `q7_timeout_count` and `q7_fallback_count` metrics

**Performance envelope:** Each query executes in 5-50 milliseconds for typical case. Q7 worst case bounded at 200ms by explicit timeout. Full L1a query set runs in approximately 200-500ms per cadence tick.

**Failure modes:** SQL execution error logged to L6, retried on next cadence. Persistent failure (3 consecutive ticks) raises operational warning visible in pulse Settings → Diagnostics.

### L1b — Stateful Rust Trackers

**Invocation:** Updated on every ingested event in the hot path. Read by L2 detectors and L3 digest assembler on demand. State persisted to corpus on cadence intervals and graceful shutdown; loaded on startup.

**Responsibility:** Persistent state that cannot be efficiently reconstructed from L0 query each cadence interval — true streaming statistics with long memory, plus state required to survive application restarts.

**Components:**

**EwmaTracker** — exponentially weighted moving average per service for error rate baseline. Alpha calibrated for ~5-minute effective window. State per service: `{alpha, current_value, last_update, sample_count}`. Approximately 40 bytes per service.

**ActivityHistogram** — 24-hour rolling histogram per service, bucketed by 5-minute intervals (288 buckets). For P-013 activity floor learning. Approximately 2.5KB per service.

**RestartDetector** — last-seen span timestamp per service plus gap-threshold logic. Approximately 16 bytes per service.

**LastIngestTracker** — atomic Instant for connection state machine (P-001). Single instance.

**State persistence:** L1b state is persisted to corpus on the following triggers:

- Every 60 seconds (aligned with cadence tick, after L1a completion)
- On graceful application shutdown
- On explicit `flush_state()` call from Settings → Diagnostics

State is loaded from corpus on startup. If corpus state is older than 1 hour, baselines bootstrap fresh. If corpus state is fresher than 1 hour, trackers load with adjusted alpha to account for time gap.

**Per-service storage:** approximately 3KB combined. For 20 services, total persisted state ≤ 60KB.

**Cold start contract:** When loading state, trackers report their bootstrap status to L6. During the first 60 seconds after fresh bootstrap, L2 detectors that depend on baseline (P-010 spike, P-012 regression) are suppressed for affected services. Hard signals and pattern detection continue normally.

**Failure modes:** Corpus state load failure falls through to fresh bootstrap, logged to L6. Persistence write failure logged but does not block ongoing operation.

### L1c — Ingestion-Time Enrichment

**Invocation:** Synchronous within the ingest hot path, between OTLP decode and DuckDB append.

**Responsibility:** Compute derived fields that are expensive to compute later — log template assignment and exception fingerprinting.

**Components:**

**Log template miner (Drain algorithm)** — fixed-depth parse tree clustering log messages into templates with variable extraction. Rust implementation matching Drain3 behavior including masking config, persistence, parameter extraction, and edge case handling. Estimated 1000-1500 LOC.

**Template profiling surface:** Settings → Diagnostics exposes a "Template Distribution" panel showing top-N templates with sample messages and occurrence counts. Users can identify pathological clustering and tune `[triage.drain]` parameters accordingly.

**Exception fingerprint computer** — cryptographic hash over `exception.type` plus first three stack frames after normalization. Stored in `span_events.fingerprint` column. Approximately 200 LOC.

**PII scrubber** — pattern-based redaction applied before persistence per P-047.

**Performance envelope:** Approximately 10-50 microseconds per event in ingest path.

**Failure modes:** Template extraction failure assigns NULL template_id and logs to L6. Exception fingerprint computation failure assigns NULL fingerprint. Neither blocks ingestion.

---

## L2 — Attention Cue Detectors

**Invocation:** Triggered by L1a query results and L1b state changes. Output to broadcast channel consumed by L3 and Cadence Coordinator.

**Responsibility:** Compare L1 outputs against thresholds and historical patterns to emit `AttentionCue` records. Cues are not user-facing; they are inputs to L3 digest composition and triggers for Cadence Coordinator.

**Detectors (canonical set):** See Appendix B (detector specifications).

### Dual-condition suppression bypass (revised in v3)

Restart-window suppression (P-016) ordinarily suppresses ErrorRateSpike cues with persistence under 30 seconds within the 60-second window following RestartEvent. Suppression is bypassed when **either** of two conditions holds:

- **Relative magnitude bypass:** `current_rate > 10× baseline_ewma` (configurable via `[triage.suppression.magnitude_bypass_multiplier]`, default 10.0)
- **Absolute threshold bypass:** `current_error_rate > 0.05` (5%, configurable via `[triage.suppression.absolute_bypass_threshold]`, default 0.05)

Either condition triggers surfacing despite suppression window. Rationale: relative magnitude catches catastrophic regressions, but on projects with naturally low baselines (0.5% baseline error rate) a 4% absolute error is meaningful even if relative magnitude is "only" 8×. The dual condition prevents this hole.

Setting either threshold to infinity disables that condition. Setting both to infinity recovers v1 blanket-suppression behavior.

**Output format:**

```rust
pub struct AttentionCue {
    pub id: Ulid,
    pub kind: CueKind,
    pub scope: CueScope,
    pub magnitude: f64,
    pub absolute_value: Option<f64>,  // for dual-condition evaluation
    pub persistence_secs: u32,
    pub confidence: f64,
    pub priority_tier: PriorityTier,  // Hard / Medium / Baseline
    pub detected_at: SystemTime,
    pub evidence_refs: EvidenceRefs,
    pub suppression_bypassed: Option<BypassReason>,  // Relative | Absolute
}
```

**Output channels:**

- `pulse://stream/attention-cues` for L3 consumption (all cues, regardless of tier)
- `pulse://stream/cadence-triggers` for Cadence Coordinator consumption (only Tier-1 and Tier-2 cues)

**Failure modes:** Detector logic errors logged to L6, fail-open behavior (skip detector, do not block other detectors).

---

## L3 — Digest Assembler

**Invocation:** Two modes governed by Cadence Coordinator (see Cadence and Event Triggers section):

- **Cadence mode:** 60-second baseline cadence, accelerated to 20s after Tier-2 cue
- **Event mode:** Immediate digest on Tier-1 hard signal

Cadence-mode and event-mode digests are identical in structure.

**Responsibility:** Compose a structured, token-bounded representation of current observability state from L1 outputs, L2 cues, and surrounding context.

**Composition sources:**

1. L1a query results (RED metrics, top operations, exception frequencies, service graph, cardinality, log frequencies, critical path samples)
2. L1b tracker state (current baselines for comparison)
3. L2 cues fired within window
4. Project context (workspace path, git activity, framework signals)
5. Corpus retrieval (similar past incidents in this workspace, ranked by relevance)

**Composition rules:** See Appendix C (digest composition rules and format specification).

**Token budget:** Target 500-2000 tokens. Hard cap 3000. Token counting via `tokenizers` crate matched to active model.

### Queue behavior for L4 invocation (revised in v3)

The L4 invocation queue uses last-write-wins (LWW) by default, with explicit exception for active-incident interpretation continuity:

**Default LWW (cadence-mode, no active incident):**

- If a newer cadence-mode digest arrives while previous is queued, the older is dropped from the queue.
- Prevents backlog accumulation when inference is slower than cadence.
- Dropped digest is still written to corpus (LWW applies only to L4 queue, not persistence).

**Active-incident exception (NEW in v3):**

- When L5 has at least one **active** incident with severity ≥ Suggested, the LWW exception activates.
- During active-incident state, cadence-mode digests for that incident scope are **not** LWW-replaced. Each digest queues independently.
- Pulse interprets evolution of active incidents, not just final state. A 10-minute incident produces multiple interpretations as it evolves, each capturing the incident's state at that moment.
- Queue depth is bounded at 5 active-incident invocations to prevent runaway backlog if multiple incidents are concurrently active. Beyond depth 5, oldest queued is dropped with L6 warning.
- Hard-signal (Tier-1) digests never LWW-replaced regardless of incident state. They queue or interrupt depending on Cadence Coordinator policy.

**After incident resolution:**

- When incident transitions to Resolved (per P-022 auto-resolution or user action), LWW returns to default behavior for that scope.
- The final cadence-mode digest produced during Resolved transition is sent to L4 with explicit `resolution_event: true` flag, allowing L4 prompt to generate resolution summary.

**Tier-1 hard signal digests:** Never LWW-replaced. Queue up to depth 3 then drop-with-warning.

**All digests written to corpus** regardless of LWW status, preserving full diagnostic continuity.

**Output channels:**

- `pulse://stream/digests` consumed by L4 (LWW-managed queue with active-incident exception)
- Corpus appends (every digest, no LWW)

**Failure modes:** Composition failure produces partial digest with explicit "context unavailable" markers. Tokenizer error degrades to character-count estimation with conservative budget.

---

## L4 — LLM Interpretation Layer

**Invocation:** Receives digest from L3 LWW-managed queue. Composes final prompt, invokes local model, parses structured output.

### Hardware profile matrix (NEW in v3)

Pulse adapts its operational guarantees based on detected hardware capability. The matrix below specifies SLO behavior per profile:

| Profile | Hardware | Primary Tier Inference | Tier-1 SLO | Tier-2 SLO | Tier-3 SLO |
|---|---|---|---|---|---|
| `gpu-primary` | ≥16GB VRAM | 1-3s | < 5s | < 20s | < 90s |
| `gpu-fallback` | 4-8GB VRAM | (fallback model) | < 3s | < 15s | < 90s |
| `cpu-primary` | No GPU, primary model | 10-30s | < 30s (degraded) | **disabled** | < 90s |
| `cpu-fallback` | No GPU, fallback model | 3-8s | < 15s | < 30s | < 90s |

**Hardware profile detection:**

On startup, pulse inspects available accelerators and the configured model. Profile is computed and logged to L6. Visible in Settings → Diagnostics → Hardware Profile.

**Tier-2 acceleration on CPU primary:**

`cpu-primary` profile cannot meet Tier-2 SLO (< 20s) because inference latency alone consumes 10-30 seconds. Tier-2 acceleration is **disabled** on this profile. Medium-confidence cues fall through to Tier-3 baseline cadence. Pulse does not pretend Tier-2 works when hardware cannot support it.

Users on `cpu-primary` profile see a one-time notice on first run: "CPU inference detected with primary model. Tier-2 acceleration disabled for performance. Consider switching to fallback model tier for more responsive interpretation. → [Switch to fallback] [Keep current]." User retains choice; pulse does not auto-switch.

**Tier-2 on cpu-fallback:**

`cpu-fallback` can support Tier-2 because fallback model inference is fast enough (3-8s) to honor accelerated cadence. SLO is degraded to < 30s rather than < 20s (single-tier acknowledgment).

### Model tiers

**Primary tier (7-13B class, 16GB VRAM target):** Full capability per P-020. All Report sections generated. Hypothesis ranking, investigation steps, full project context grounding.

**Fallback tier (3-4B class, 4-6GB VRAM):** Reduced quality. Hypothesis ranking simplified to single hypothesis without confidence breakdown. Investigation steps reduced from up to 5 to up to 2. Project context grounding present but less specific. Reports explicitly mark fallback-tier provenance.

### Input composition

- System prompt with role definition and JSON schema (~800-1000 tokens primary, ~500-700 tokens fallback)
- Project context block (~500-1000 tokens)
- Current digest (~500-2000 tokens, from L3)
- Corpus retrieval context (~1000-2000 tokens primary, ~500-1000 tokens fallback)
- Output format reminder (~200-300 tokens)

**Total: ~6-8K tokens for primary, ~4-5K tokens for fallback.**

### Output format

```json
{
  "decision": "surface" | "dismiss" | "watch",
  "severity": "autonomous" | "suggested" | "curious" | "none",
  "title": "...",
  "symptom": "...",
  "timeline": "...",
  "hypotheses": [
    {"statement": "...", "confidence": "high|medium|low", "justification": "..."}
  ],
  "investigation_steps": [
    {"step": "...", "expected_yield": "..."}
  ],
  "evidence_refs": [...],
  "fingerprint": "...",
  "schema_version": "2.0",
  "prompt_version": "v2.1",
  "model_tier": "primary" | "fallback",
  "hardware_profile": "gpu-primary" | "gpu-fallback" | "cpu-primary" | "cpu-fallback",
  "is_resolution_summary": boolean
}
```

### Implementation runtime

Local LLM inference via `mistralrs` or `candle`. **This choice is blocking for L4 implementation start; see Blocking Decisions section below.** Model loaded once at startup, kept resident. JSON-constrained output via library-supported grammar enforcement. Tokenization via `tokenizers` crate.

**Tokenizer-model sync:** When model selection changes, L4 reloads both the model and the tokenizer paired with it. Tokenizer matched to model checkpoint; token-budget calculation in L3 uses currently-loaded tokenizer; recalibration is automatic on model swap.

### Concurrency

Single inference at a time. LWW for cadence-mode digests (per L3 queue behavior with active-incident exception). Tier-1 hard-signal digests do not get replaced; they queue or interrupt depending on configured policy (default: queue, max 3 deep before drop-with-warning).

### Timeout

Inference timeout varies by hardware profile (see matrix above). Beyond timeout, current invocation cancelled and counted as failure.

### JSON parse failure handling

Failure modes and recovery:

- **Single parse failure:** Failure logged to L6, current incident cycle dismissed. Next cadence tick proceeds normally. Failure counter incremented.
- **Three consecutive failures within 5 minutes:** L4 enters degraded mode. Subsequent digests not sent to LLM; L5 surfaces hard signals only with default Suggested severity. Diagnostic warning visible in Settings.
- **Recovery from degraded mode:** Pulse retries LLM invocation on next cadence tick after entering degraded mode. If successful parse, exits degraded mode immediately. If fails, continues degraded with retry on each subsequent cadence (exponential backoff: 2min → 5min → 10min capped).
- **Backoff visibility (NEW in v3):** Settings → Diagnostics displays current backoff state with countdown timer: "Next interpretation retry in 3m 24s." When in degraded mode, a "Retry interpretation now" button overrides backoff and forces immediate retry. Allows users to escape stuck backoff without restarting pulse.
- **Manual recovery:** "Retry interpretation now" button forces immediate retry regardless of backoff state.

### Graceful degradation when model unavailable

Per P-020, this layer is skipped entirely when LLM is unavailable. L5 surfaces hard signals from L1a Q1 (status_code = 2 spans, root-span scope) with default Suggested severity. No hypothesis ranking, no investigation steps, explicit notice in any generated Report.

### Failure modes summary

| Failure | Scope | Recovery |
|---|---|---|
| Single parse error | Current cadence cycle | Next tick |
| 3 consecutive parse errors / 5min | Global L4 (degraded) | Backoff retry with visible countdown |
| Inference timeout | Current cadence cycle | Counts toward failure counter |
| Model load failure (startup) | Pulse-wide (degraded) | Manual model reconfig + retry |
| Tokenizer/model version mismatch | Pulse-wide (degraded) | Auto-detected, requires reconfig |
| GPU OOM during inference | Single inference | Counts as failure; user notice |
| Hardware profile downgrade detected | Settings notice | User chooses action |

---

## L5 — Surface and Persist

**Responsibility:** Apply L4 decisions to user-visible surfaces (widget, counter, dropdown, Report) and persist incident records to corpus.

**Surface updates:**

- Widget halo: hue updated based on cumulative active incident severity (P-025)
- Widget service constellation: per-service dots reflect current state (P-027)
- Findings counter: incremented on new active incident (P-028)
- Findings dropdown: incident added to sort order (P-029)
- Report content: generated from L4 output, ready for in-app display (P-031)

**Resolution summary handling (NEW in v3):**

When L4 produces output with `is_resolution_summary: true`, L5 attaches the summary to the now-Resolved incident record without producing new surface notification. The summary appears in the incident's Report when accessed historically, providing a "what happened, how it ended" narrative that closes the loop on incident lifecycle.

**Corpus persistence:** Incidents written with full lifecycle tracking. User feedback events (P-046) appended to records. Both L4-produced incidents and L3 digests are written; only the former produce user-visible surface changes.

**Failure modes:** Surface update failure (TauRPC error) retried with backoff. Corpus write failure logged to L6 and queued for later flush. Persistent corpus failure raises critical diagnostic warning.

---

## L6 — Pipeline Self-Observability

**Status:** Cross-cutting layer, not in main data path.

**Responsibility:** Continuous observation of distillation pipeline operational health. Emits metrics about pipeline behavior for operational debugging, regression detection, and Conductor scenario validation.

**Why a dedicated layer:** Pipeline failures (Drain over-clustering, baseline EWMA divergence, LLM parse rate degradation, Q7 timeouts) are not user-facing problems but they materially affect pulse's interpretation quality. Without explicit self-observability, these failure modes manifest only as "pulse feels off lately" — un-diagnosable. L6 makes them inspectable.

### Metrics emitted

**Per-layer metrics:**

```
pipeline.l0.ring_buffer_rows_total
pipeline.l0.ring_buffer_evictions_per_minute
pipeline.l0.insert_latency_p99_microseconds

pipeline.l1a.query_count_total{query_name}
pipeline.l1a.query_latency_p99_milliseconds{query_name}
pipeline.l1a.query_failure_count_total{query_name, reason}
pipeline.l1a.q7_timeout_count_total
pipeline.l1a.q7_fallback_count_total

pipeline.l1b.persist_count_total
pipeline.l1b.persist_failure_count_total
pipeline.l1b.bootstrap_count_total{kind: fresh | restored}
pipeline.l1b.tracked_services_total

pipeline.l1c.drain_template_count_total
pipeline.l1c.drain_assignment_latency_p99_microseconds
pipeline.l1c.exception_fingerprint_count_total

pipeline.l2.cues_emitted_total{kind, tier}
pipeline.l2.suppression_active_seconds_total
pipeline.l2.magnitude_bypass_triggered_total{reason: relative | absolute}

pipeline.l3.digests_assembled_total{mode: cadence | event | resolution}
pipeline.l3.digest_token_count_p99
pipeline.l3.digest_token_budget_exceeded_count_total
pipeline.l3.assembly_latency_p99_milliseconds
pipeline.l3.lww_drop_count_total
pipeline.l3.active_incident_queue_depth

pipeline.l4.inferences_total{result: success | parse_error | timeout | oom}
pipeline.l4.inference_latency_p99_milliseconds
pipeline.l4.inference_queue_depth
pipeline.l4.degraded_mode_active_seconds_total
pipeline.l4.degraded_mode_entries_total
pipeline.l4.model_tier_active{tier: primary | fallback}
pipeline.l4.hardware_profile_active{profile}
pipeline.l4.backoff_remaining_seconds

pipeline.l5.incidents_surfaced_total{severity}
pipeline.l5.incidents_resolved_total{resolution_kind: auto | user | timeout}
pipeline.l5.corpus_writes_total
pipeline.l5.corpus_write_failure_count_total
```

### Output channels

- Internal broadcast `pulse://stream/pipeline-health` for in-app consumption
- Self-obs metrics rendered in Settings → Diagnostics view (read-only initially per P-051)
- Self-obs metrics also emitted to pulse's self-instrumented `tracing-subscriber` JSON log to local file

### Use cases

**For users:** Settings → Diagnostics shows current pipeline health. If pulse "feels off," user can verify whether it's a known operational issue (LLM in degraded mode, Drain over-clustering, baselines bootstrap fresh, hardware profile downgrade) vs perceived but unactual.

**For Conductor scenarios:** Verification of capability behavior often requires checking pipeline metrics. "When this scenario runs, did P-010 actually emit a cue with expected magnitude?" is a self-obs query.

**For development debugging:** When iterating on prompt engineering, threshold tuning, or model selection, real metric history of pipeline behavior is essential.

**For corpus-based regression detection:** Pipeline metrics over time, stored in corpus, allow comparison of "does pulse perform similarly today as it did last week?"

### Implementation

L6 instrumentation lives as `tracing` spans and counters in each layer's code. Aggregation happens at boundary points. Storage uses dedicated corpus table (`pipeline_metrics`) with retention of 30 days (configurable). Estimated storage cost: < 10MB per month.

---

## Cadence and event triggers

Three concurrent priority tiers govern pipeline rhythm. SLO targets depend on hardware profile (see L4 Hardware Profile Matrix).

### Tier 1 — Hard signal / Immediate

**Triggers:** P-005 through P-008 hard signals (span ERROR with root scope, exception events, FATAL-level logs). Confidence threshold not applicable.

**Behavior:** Immediate L3 digest assembly + L4 invocation, bypassing cadence wait. Queue position 1 (interrupts cadence-mode if currently queued; does not interrupt mid-inference).

**SLO:** Profile-dependent. < 5s on `gpu-primary`, < 30s on `cpu-primary`, etc.

### Tier 2 — Medium-priority cue / Accelerated

**Triggers:** L2 cues with confidence in range [0.7, 0.85].

**Behavior:** Schedules next L3 + L4 cycle at `min(remaining_time_to_next_cadence, 20s)` instead of waiting full 60s.

**Hardware-conditional availability:** Disabled on `cpu-primary` profile (see L4). Falls through to Tier-3 baseline behavior on that profile.

**SLO:** Profile-dependent. < 20s on `gpu-primary`, disabled on `cpu-primary`.

### Tier 3 — Baseline cadence

**Triggers:** Cadence tick (default 60s) regardless of cue state.

**Behavior:** L1a queries execute, L3 digest assembles, L4 invoked (subject to LWW per L3 rules).

**SLO:** < 90s on all profiles.

### Background reflection cadence (15-30 minutes)

**Triggers:** Cadence tick at long interval. Produces longer-window digest (30-minute history) for cumulative pattern detection.

**Output semantics:** Reflection digests distinguishable via `digest_kind` field. Different system prompt emphasizing trend analysis. L5 treats reflection output with default-Curious severity unless model identifies high-confidence pattern.

**Cancellation:** Lower priority than cadence and tier-1/tier-2 triggers. If LLM busy when reflection cadence fires, reflection is skipped (not queued).

### Active-incident continuity (NEW in v3)

When L5 has at least one active incident with severity ≥ Suggested, cadence-mode digests for that incident scope are exempt from LWW per L3 queue rules. This ensures incident evolution is interpreted, not just final state. See L3 queue behavior section.

### Rate limiting

LLM invocations bounded by cadence and trigger frequency. Maximum sustained invocation rate approximately 1 per 10 seconds. Active-incident exception increases this rate proportionally to concurrent active incidents (typically 1-3), capped at 1 per 5 seconds total.

---

## Service identity lifecycle

Services discovered from `service.name` attribute progress through a lifecycle that affects detection behavior.

### States

| State | Definition | Detection behavior |
|---|---|---|
| `Unknown` | Service name has not been seen | No tracking |
| `Bootstrapping` | First 60 seconds since first appearance | All baselines suppressed; only hard signals surface |
| `Active` | Recently emitted spans (within learned activity floor) | Full detection |
| `Quiet` | No spans, within learned quiet duration | No ServiceWentSilent; otherwise normal |
| `Silent` | No spans, beyond learned quiet duration | ServiceWentSilent cue emitted |
| `Dormant` | No spans for > 1 hour (configurable) | Trackers preserved in corpus, dimmed in active constellation |
| `Archived` | No spans for > 24 hours (configurable) | Trackers archived to corpus; service hidden from constellation |

### Rationale for state count

The seven states distinguish behaviorally-distinct phases that drive different pipeline behavior:

- `Bootstrapping` vs `Active`: baseline-dependent detectors suppressed during bootstrap
- `Active` vs `Quiet`: ServiceWentSilent gating differs
- `Quiet` vs `Silent`: actual cue emission differs
- `Silent` vs `Dormant`: visible in constellation vs dimmed (UX)
- `Dormant` vs `Archived`: visible in constellation vs hidden (UX), in-memory tracker vs archived

Reducing state count would conflate behaviors that are logically and observably different. The seven-state model is intentional; configurability of boundary durations (below) addresses the "arbitrary timing" concern.

### Configurable boundaries

- `[triage.lifecycle.dormant_after_secs]` (default 3600 = 1 hour)
- `[triage.lifecycle.archived_after_secs]` (default 86400 = 24 hours)

Users can adjust based on their workload patterns. Long-running production-like deployments might extend durations; rapid-iteration vibe-coder workflows might shorten.

### Transitions

Same as v2 transition table. See [Transitions] subsection below.

- `Unknown` → `Bootstrapping`: First valid span with this service name
- `Bootstrapping` → `Active`: After 60s of activity, or first qualifying span flow
- `Active` → `Quiet`: No spans for duration ≤ activity floor 95th percentile
- `Quiet` → `Active`: New span arrives
- `Quiet` → `Silent`: Duration exceeds activity floor 95th percentile
- `Silent` → `Active`: New span arrives (resolves ServiceWentSilent automatically)
- `Active` / `Quiet` / `Silent` → `Dormant`: Configured dormant duration without spans
- `Dormant` → `Active`: New span arrives, treated as restart with `RestartEvent` emitted
- `Dormant` → `Archived`: Configured archive duration without spans
- `Archived` → `Active`: New span arrives, check corpus history:
  - If service in corpus within last 7 days, treat as restart (`RestartEvent` emitted)
  - If service older than 7 days or absent, treat as new discovery (`Bootstrapping` state, no restart event)

### Edge case: ring buffer eviction

A service silent for 11+ minutes will not appear in L1a Q1 results. State of that service is preserved in L1b trackers (in-memory) and corpus (after persistence). When the service returns, L1b RestartDetector recognizes it via in-memory last-seen state. If L1b state has been evicted too (rare for services under 1 hour silence), corpus lookup resolves the service identity and lifecycle position.

### Manual override

Users can manually mark services as `Dormant` or `Archived` via Settings → Services.

---

## Configuration and hot reload

Pulse configuration lives in `~/.andromeda-pulse/config.toml`. Some configuration changes can be applied without restart; others require restart.

### Hot reload supported

The following config keys are file-watched and applied within 2 seconds of save (with 500ms debounce):

- `[triage.cadence.*]` (baseline_secs, accelerated_secs, reflection_secs)
- `[triage.thresholds.*]` (error_rate_multiplier, latency_multiplier, retry_storm_count, retry_storm_autonomous)
- `[triage.suppression.*]` (window_secs, magnitude_bypass_multiplier, absolute_bypass_threshold)
- `[triage.severity_rules.*]` (rule weight overrides)
- `[triage.tooltip.*]`
- `[triage.lifecycle.*]` (dormant_after_secs, archived_after_secs)

### Restart required

The following config keys require pulse restart (file-watched changes raise notice in Diagnostics):

- `[triage.drain.*]` (depth, similarity, max_clusters) — invalidates template tree
- `[triage.model.*]` (gguf_path, tier) — requires model unload/reload
- `[ingest.otlp.*]` (grpc_port, http_port) — receiver bind
- `[storage.ring_buffer_minutes]` — DuckDB schema reconfiguration
- `[storage.corpus_path]` — corpus reattachment

### Prospective application of threshold changes (NEW in v3)

Hot-reloaded threshold changes apply **prospectively, not retroactively**. Concretely:

- Threshold changes do not reset L1b baselines. EWMA continues from current state.
- Threshold changes do not re-evaluate the current rolling window against new thresholds.
- New thresholds apply starting from the next L1a tick. Cues that would have fired under old thresholds for already-observed data are not retroactively triggered.

**Rationale:** Without this rule, lowering `error_rate_multiplier` from 3.0 to 1.5 on a project with accumulated baseline would suddenly trigger cues for many services that were quietly running at slightly-elevated error rates — a notification flood that violates the calm-by-default invariant. Prospective application means threshold changes are inputs to future evaluation only; they cannot retroactively transform "normal" past observations into "anomalous" surfaced incidents.

If users want to apply new thresholds to recent history, they can manually trigger backfill via Settings → Diagnostics → "Re-evaluate recent window with current thresholds." This is opt-in and explicitly user-initiated, not automatic.

### Hot reload interaction with baselines

Hot-reloaded threshold changes do not reset L1b baselines. Hot-reloaded cadence changes apply on the next cadence boundary.

### Failure modes

Malformed config rejected with notice in Diagnostics. Previous valid config remains active.

---

## Testability strategy

The distillation pipeline mixes deterministic components (SQL, Rust trackers) with non-deterministic components (LLM inference). Test strategy reflects this split.

### Unit testing

**L0 — Ring buffer:** Standard SQL integration tests.

**L1a — SQL aggregations:** Time-injected unit tests with fixture data. Coverage of every Q1-Q7 query plus edge cases.

**L1b — Stateful trackers:** Time-injected unit tests with injectable `Clock` trait. State persistence verified by round-tripping through serialization.

**Property-based testing for L1b (NEW in v3):** Use `proptest` crate to verify invariants:

- EwmaTracker: For any sequence of inputs, current_value is bounded by [min(inputs), max(inputs)] within alpha-dependent tolerance.
- EwmaTracker: Convergence time to within ε of true mean is bounded by `log(ε) / log(1-alpha)` iterations.
- ActivityHistogram: Bucket transitions are correct for any monotonic time sequence.
- ActivityHistogram: 24-hour rolling property — buckets older than 24h are evicted; sum of bucket counts equals total observations.
- RestartDetector: Idempotent on repeated identical inputs; transition only on gap threshold.

Property-based testing is a natural fit because L1b trackers are pure data structures with mathematical invariants. Tests generate randomized inputs and verify properties hold, catching edge cases that example-based tests miss.

**L1c — Ingestion enrichment:** Golden-file tests for Drain template miner with LogHub and Drain3 test corpora. Exception fingerprinter: deterministic input → deterministic fingerprint with normalization edge cases.

**L2 — Attention cue detectors:** Synthetic input tests. Suppression interactions tested explicitly (RestartEvent → 60s suppression → magnitude bypass → absolute bypass).

**L3 — Digest assembler:** Golden-file tests with canonical input fixtures. Token count assertions. Active-incident exception path tested explicitly.

**L4 — LLM interpretation:** Schema validation tests with constructed JSON. Parse failure counter tests. LLM output quality covered by Conductor scenarios (not unit tests).

**L5 — Surface and persist:** State transition tests with mock incident inputs. Resolution summary path tested.

**L6 — Self-observability:** Self-reporting accuracy tests injecting known pipeline behaviors and verifying metric output.

### Integration testing via Conductor

End-to-end tests through Conductor scenarios. Each P-XXX capability has at least one corresponding scenario.

### Load testing (NEW in v3)

Conductor scenarios include sustained-load tests verifying pipeline behavior at scale:

- **Baseline load:** 1000 spans/sec for 60 seconds. Verify pipeline operates within performance budget.
- **High load:** 10000 spans/sec for 5 minutes. Verify graceful operation, no memory growth beyond budget, L1a query latency remains under 500ms p99.
- **Burst load:** 50000 spans/sec for 30 seconds, returning to baseline. Verify pipeline absorbs burst without crash, recovers to normal operation within 1 minute.
- **Sustained extreme:** 50000 spans/sec for 5 minutes. Verify degraded but functional operation. Acceptable degradation: increased L1a query latency, possible LWW drops on cadence digests, no data loss in L0 within retention window.

Load test pass criteria are SLO-relative: pipeline performance budget targets (memory, CPU, latency) must hold for baseline and high loads, may degrade gracefully for burst and sustained extreme.

### Coverage targets

- L1a/L1b/L1c/L2/L3: 90%+ line coverage via unit tests
- L4: 80%+ coverage of schema parsing and failure paths
- L5: 85%+ via state transition tests
- L6: 90%+ via self-reporting tests
- End-to-end: Conductor scenario suite covers each P-XXX capability
- Load: All four load profiles pass before v0.2.0 release

---

## Prompt and schema versioning

LLM prompts and output schemas evolve. The pipeline tracks evolution explicitly to maintain provenance and prevent silent semantic drift.

### Versioning scheme

- **Prompt version:** `v{major}.{minor}`. Major bumps on substantial prompt restructure. Minor bumps on parameter or instruction adjustments.
- **Schema version:** `{major}.{minor}`. Major bumps on JSON output structural changes. Minor bumps on optional field additions.

Both versions embedded in every L4 output and persisted in corpus.

### Provenance, not invalidation

Old corpus records remain queryable and used for retrieval. Retrieval-time annotation includes prompt/schema versions when matched past incidents use different versions than current. Reports include "Historical context note" when relevant.

### Schema validator

L4 output validator includes schema_version field. Mismatched schema_version treated as parse failure.

---

## Schema additions

```sql
-- Log template registry
CREATE TABLE log_templates (
  template_id INTEGER PRIMARY KEY,
  template_text VARCHAR NOT NULL,
  first_seen TIMESTAMP NOT NULL,
  last_seen TIMESTAMP NOT NULL,
  occurrence_count BIGINT NOT NULL DEFAULT 0
);

ALTER TABLE logs ADD COLUMN template_id INTEGER REFERENCES log_templates(template_id);
ALTER TABLE span_events ADD COLUMN fingerprint VARCHAR;
CREATE INDEX idx_span_events_fingerprint ON span_events(fingerprint) WHERE name = 'exception';

-- L1b tracker state persistence
CREATE TABLE baseline_state (
  service_name VARCHAR NOT NULL,
  state_kind VARCHAR NOT NULL,
  state_blob BLOB NOT NULL,
  saved_at TIMESTAMP NOT NULL,
  schema_version INTEGER NOT NULL,
  PRIMARY KEY (service_name, state_kind)
);

-- Pipeline self-observability
CREATE TABLE pipeline_metrics (
  metric_name VARCHAR NOT NULL,
  labels JSON NOT NULL,
  value DOUBLE NOT NULL,
  recorded_at TIMESTAMP NOT NULL
);
CREATE INDEX idx_pipeline_metrics_time ON pipeline_metrics(recorded_at);

-- Service registry
CREATE TABLE service_registry (
  service_name VARCHAR PRIMARY KEY,
  first_seen TIMESTAMP NOT NULL,
  last_seen TIMESTAMP NOT NULL,
  current_state VARCHAR NOT NULL,
  manually_archived BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE VIEW recent_red_metrics AS ...;
```

---

## Implementation stack

**Existing in pulse (reused):** DuckDB, Arrow, tokio, workspace-detector, snapshot crate primitives, tracing-subscriber.

**New dependencies:** `mistralrs` or `candle` (decision blocking — see below), `tokenizers`, `notify`.

### New code estimates

| Component | LOC estimate | Notes |
|---|---|---|
| L1a SQL templates + scheduler | 400-600 | |
| L1b stateful trackers | 400-600 | |
| L1b state serialization + corpus I/O | 200-300 | |
| L1c Drain Rust implementation | 1000-1500 | Spike validates estimate before implementation |
| L1c exception fingerprint normalizer | 150-200 | |
| L1c template profiling surface | 150-250 | |
| L2 attention cue detectors | 400-600 | |
| L2 dual-condition bypass + tier coordination | 150-250 | |
| L3 digest assembler | 500-800 | |
| L3 LWW queue + active-incident exception | 150-250 | |
| L4 LLM client + prompt scaffolding + JSON parsing | 600-1000 | |
| L4 failure handling + degraded mode + backoff + countdown | 300-450 | |
| L4 hardware profile detection + matrix logic | 250-400 | |
| L4 fallback tier support | 200-300 | |
| L5 surface and corpus persistence + resolution summary | 350-550 | |
| L6 self-observability instrumentation | 400-600 | |
| L6 Diagnostics view (frontend) | 350-550 | |
| Service identity lifecycle + configurable boundaries | 250-400 | |
| Configuration hot-reload (notify integration) + prospective application | 200-300 | |
| Cross-cutting wiring | 300-500 | |

**Total estimate: 6500-9750 LOC of new code** (slight increase from v2 reflecting v3 additions: dual-condition bypass, active-incident exception, hardware profile detection, resolution summary, configurable lifecycle, prospective application).

**Note on estimates:** LOC figures are planning baselines, not contracts. Components historically prone to expansion (state machines, failure handling, self-observability) may exceed estimates by 30-50% in practice. Reserve mental capacity for this when planning chunk durations. Andromeda's iteration speed compensates somewhat — chunks complete faster than typical engineering pace — but the reserve is psychologically useful.

---

## Performance budgets

| Metric | Target | Notes |
|---|---|---|
| Steady-state memory (entire pulse) | < 500MB | |
| L0 ring buffer memory | 100-200MB | |
| L1b tracker memory | < 10MB | |
| L1b persisted state size | < 100KB | |
| Model memory (VRAM, primary tier) | < 16GB | Per P-020 |
| Model memory (VRAM, fallback tier) | < 6GB | |
| Ingest hot path latency (per event) | < 100μs | |
| L1a query latency (full set, p99) | < 500ms | |
| L1a Q7 latency (p99) | < 200ms | Explicit timeout |
| L1a Q7 fallback rate | < 5% | |
| L3 digest assembly latency | < 100ms | |
| L4 inference latency (primary, GPU p99) | 1-3s | |
| L4 inference latency (primary, CPU p99) | 10-30s | |
| L4 inference latency (fallback, GPU p99) | < 1s | |
| L4 inference latency (fallback, CPU p99) | 3-8s | |
| End-to-end Tier-1 (gpu-primary) | < 5s p99 | |
| End-to-end Tier-1 (cpu-primary) | < 30s p99 | Degraded SLO |
| End-to-end Tier-2 (gpu-primary) | < 20s p99 | |
| End-to-end Tier-2 (cpu-primary) | disabled | Falls through to Tier-3 |
| End-to-end Tier-3 (any profile) | < 90s p99 | |
| LLM invocation rate (sustained) | ≤ 1 per 10s baseline | Active incidents may increase |
| Active-incident invocation rate cap | ≤ 1 per 5s | |
| CPU utilization (steady state, no incidents) | < 5% | |
| CPU utilization (active incident, peak) | < 25% | |
| CPU utilization (CPU-inference profile, peak) | < 70% | Inference dominates |
| Disk I/O (corpus writes) | < 20 writes/min | |
| L6 metric storage growth | < 10MB/month | |

---

## Blocking decisions before implementation start (NEW in v3)

The following decisions are blocking for implementation kickoff. Mini-route chunks that depend on these decisions cannot start until resolved. Listed in priority order:

### 1. LLM runtime choice (`mistralrs` vs `candle`)

**Why blocking:** L4 implementation directly depends on chosen runtime's API patterns, error types, and dependency tree. L4 chunks cannot start until selected.

**Decision criteria:**
- Maturity and stability at implementation time
- Quantization format support (GGUF essential, AWQ/EXL2 nice-to-have)
- Backend coverage (Metal critical for M-series Mac, CUDA for NVIDIA)
- JSON-constrained generation support (essential)
- Tokenizer integration ergonomics
- Memory management and resource cleanup quality

**Decision deadline:** Before implementation chunk #L4-1 (first L4 chunk).

**Owner:** Project lead.

### 2. Drain Rust implementation spike

**Why blocking:** Drain LOC estimate (1000-1500) is the largest single component and reviewer-flagged as potentially still optimistic. Validating the estimate via spike before committing to L1c implementation order is prudent.

**Spike scope:** Implement minimal Drain prototype in Rust covering: fixed-depth parse tree, template extraction, similarity matching, basic masking config. Test against subset of LogHub corpus. Measure actual LOC and quality of template assignments.

**Decision criteria:**
- Actual LOC vs 1000-1500 estimate (anchor for chunk planning)
- Template quality on real log corpora (validates algorithm correctness)
- Performance (per-event microseconds, lookup amortization)

**Spike output:** Either confirmation that 1000-1500 estimate holds, or revised estimate with rationale. Spike code is throwaway; production implementation follows.

**Decision deadline:** Before implementation chunk #L1c-1.

**Owner:** Project lead.

### 3. Mid-incident interpretation behavior (resolved in v3)

**Status:** Resolved in v3. Active-incident exception to LWW activates when L5 has at least one active incident with severity ≥ Suggested. Cadence-mode digests for that incident scope are not LWW-replaced. Documented in L3 queue behavior.

**Why this is listed:** Explicit pinning of the resolution to prevent silent drift during implementation. Documented here for traceability.

---

## Still open decisions (calibration only)

These are calibration questions resolved through Conductor scenario testing during implementation, not pre-implementation review:

1. **Cadence baseline (60s default).** Tunable; calibration via real workload feel.
2. **Token budget hard cap (3000).** Comfortable for 7B/8K context.
3. **Background reflection cadence (default 30 minutes).** Off-by-default option may emerge from user feedback.
4. **Drain parameters (depth=4, similarity=0.5).** Tunable via Settings → Diagnostics on real workload.
5. **Critical path sample count in digest (5).** Tunable.
6. **Corpus retrieval count in digest (3).** Tunable.
7. **Magnitude bypass threshold (10× relative, 5% absolute).** Tunable; dual-condition resolves the gap reviewer identified.
8. **Failure counter window (3 failures in 5 minutes).** Tunable; calibration of degraded mode threshold.
9. **L6 metric retention (30 days default).** Tunable; trades storage for visibility.
10. **Active-incident queue depth cap (5).** Tunable; balances incident continuity against backlog.
11. **Lifecycle boundaries (1h dormant, 24h archived).** Tunable per workload.

---

## Future considerations

Items out of scope for v0.2.0:

**Vector store for semantic similarity (v0.3.0+).** When structural fingerprint matching proves insufficient.

**Multi-window digest assembly with tool use (v0.3.0+).** LLM queries different time windows via tool calls.

**Incremental materialized views (v0.3.0+).** If L1a query latency becomes bottleneck.

**Distributed inference (v1.0+).** Cloud tier for users with insufficient local hardware.

**Streaming model output (v0.3.0+).** Progressive disclosure in Reports.

**Adaptive cadence (v0.3.0+).** Cadence intervals self-tune based on observed project rhythm.

**Cross-workspace pattern matching (v0.3.0+, controversial).** Opt-in feature for multi-project setups.

**Automatic threshold tuning (v0.3.0+).** Pulse suggests threshold adjustments based on observed cue accuracy from user feedback signals.

---

## Relationship to capability spec

Each layer of this architecture implements specific capabilities. Mapping:

| Layer | Implements |
|---|---|
| L0 | (infrastructure, no direct capabilities) |
| L1a | P-009, P-011, P-014, P-027 |
| L1b | P-009, P-013, P-015 |
| L1c | P-006, P-007, P-017 |
| L2 | P-010, P-012, P-014, P-016, P-018, P-021 |
| L3 | P-031, P-032, P-044 |
| L4 | P-019, P-020, P-033, P-034 |
| L5 | P-022, P-023, P-024 through P-030, P-041 through P-046 |
| L6 | (operational infrastructure, supports verification of all capabilities) |

### TODO: capability spec formalization (NEW in v3)

The following architectural concepts function as user-facing contracts in this document but lack formal P-XXX numbers in `pulse-capability-spec.md`. They should be added as P-XXX claims before v0.2.0 implementation start:

- **Cadence configuration** (default 60s, configurable via `[triage.cadence]`) — formalize as user-facing config contract
- **Fallback model tier** (3-4B class, 6GB VRAM, reduced quality with explicit Report annotation) — formalize as graceful-degradation capability
- **Hot reload semantics** (which params reload, which require restart) — formalize as config-stability contract
- **Magnitude bypass behavior** (dual-condition: relative > 10× OR absolute > 5%) — formalize as suppression escape-valve contract
- **Self-observability surface** (Settings → Diagnostics view) — formalize as transparency contract
- **Hardware profile awareness** (gpu-primary, gpu-fallback, cpu-primary, cpu-fallback with profile-dependent SLOs) — formalize as hardware-honesty contract
- **Active-incident continuity** (LWW exception during active incidents) — formalize as interpretation-continuity contract
- **Prospective threshold application** (threshold changes do not retroactively re-evaluate history) — formalize as no-surprise contract

**Owner:** Project lead.
**Deadline:** Before mini-route chunk #1 implementation start.
**Estimated effort:** Single capability-spec amendment chunk, ~8-10 new P-XXX entries, ~500-800 words of new spec content.

---

## Appendix A — SQL Query Templates

Q1-Q7 templates. Q7 includes explicit limits and fallback as documented in L1a.

```sql
-- Q1: RED metrics per service
SELECT
  service_name,
  COUNT(*) AS request_count,
  SUM(CASE WHEN status_code = 2 THEN 1 ELSE 0 END) AS error_count,
  SUM(CASE WHEN status_code = 2 THEN 1.0 ELSE 0.0 END)
    / NULLIF(COUNT(*), 0) AS error_rate,
  APPROX_QUANTILE(duration_ns, 0.50) AS p50_ns,
  APPROX_QUANTILE(duration_ns, 0.95) AS p95_ns,
  APPROX_QUANTILE(duration_ns, 0.99) AS p99_ns
FROM spans
WHERE start_time > now() - INTERVAL ?
GROUP BY service_name;

-- Q2: RED per service::operation
SELECT
  service_name,
  COALESCE(http_route, db_statement_template, name) AS operation,
  COUNT(*) AS request_count,
  SUM(CASE WHEN status_code = 2 THEN 1 ELSE 0 END) AS error_count,
  APPROX_QUANTILE(duration_ns, 0.99) AS p99_ns
FROM spans
WHERE start_time > now() - INTERVAL ?
GROUP BY service_name, operation
ORDER BY error_count DESC, p99_ns DESC
LIMIT 20;

-- Q3: Exception fingerprint frequencies
SELECT
  fingerprint,
  COUNT(*) AS occurrences,
  MIN(time) AS first_seen,
  MAX(time) AS last_seen,
  array_agg(DISTINCT service_name) AS services
FROM span_events
WHERE name = 'exception'
  AND time > now() - INTERVAL ?
GROUP BY fingerprint
HAVING COUNT(*) >= 2
ORDER BY occurrences DESC
LIMIT 10;

-- Q4: Service interaction graph
SELECT
  caller_service.service_name AS caller,
  callee_service.service_name AS callee,
  COUNT(*) AS call_count,
  SUM(CASE WHEN callee_service.status_code = 2 THEN 1 ELSE 0 END) AS error_count
FROM spans callee_service
JOIN spans caller_service ON callee_service.parent_span_id = caller_service.span_id
WHERE callee_service.start_time > now() - INTERVAL ?
  AND caller_service.service_name != callee_service.service_name
GROUP BY caller, callee;

-- Q5: Cardinality estimate per service
SELECT
  service_name,
  APPROX_COUNT_DISTINCT(COALESCE(http_route, db_statement_template, name)) AS distinct_operations,
  COUNT(*) AS total_spans
FROM spans
WHERE start_time > now() - INTERVAL ?
GROUP BY service_name;

-- Q6: High-severity log frequencies
SELECT
  service_name,
  template_id,
  COUNT(*) AS occurrences,
  MAX(severity_number) AS peak_severity
FROM logs
WHERE time > now() - INTERVAL ?
  AND severity_number >= 17
GROUP BY service_name, template_id
ORDER BY occurrences DESC
LIMIT 15;

-- Q7: Critical path sample (primary, with bounds)
WITH RECURSIVE
  slow_traces AS (
    SELECT trace_id
    FROM spans
    WHERE parent_span_id IS NULL
      AND start_time > now() - INTERVAL ?
    ORDER BY duration_ns DESC
    LIMIT 5
  ),
  trace_tree AS (
    SELECT s.*, 0 AS depth
    FROM spans s JOIN slow_traces st USING (trace_id)
    WHERE s.parent_span_id IS NULL
    UNION ALL
    SELECT s.*, tt.depth + 1
    FROM spans s JOIN trace_tree tt
      ON s.parent_span_id = tt.span_id
    WHERE tt.depth < 10
  )
SELECT * FROM trace_tree
ORDER BY trace_id, depth, start_time
LIMIT 100;

-- Q7-fallback: Shallow non-recursive (used on timeout/limit hit)
WITH slow_traces AS (
  SELECT trace_id, span_id
  FROM spans
  WHERE parent_span_id IS NULL
    AND start_time > now() - INTERVAL ?
  ORDER BY duration_ns DESC
  LIMIT 5
)
SELECT s.*
FROM spans s
WHERE s.trace_id IN (SELECT trace_id FROM slow_traces)
  AND (s.parent_span_id IS NULL OR s.parent_span_id IN (SELECT span_id FROM slow_traces))
ORDER BY s.trace_id, s.start_time;
```

## Appendix B — Detector Specifications

| Detector | Triggered by | Threshold | Tier | Output |
|---|---|---|---|---|
| `ErrorRateSpike` | L1a Q1 result | `last_30s_rate > 3.0× baseline_ewma` with 30s persistence; subject to dual-condition bypass | Hard if dual-bypass triggered, else Medium/Baseline | Cue with magnitude, absolute_value, and persistence |
| `LatencyRegression` | L1a Q1, Q2 results | `current_p99 > 2.5× baseline_p99` with 60s persistence | Medium if confidence≥0.7 else Baseline | Cue with magnitude and persistence |
| `RetryStorm` | L1a Q3 result | Same fingerprint ≥ 5 occurrences in 30s; ≥10 escalates | Medium at 5+, Hard at 10+ | Cue with fingerprint and count |
| `ServiceWentSilent` | L1b ActivityHistogram | Current quiet duration > 95th percentile of learned quiet | Baseline | Cue with service and duration |
| `CardinalityExplosion` | L1a Q5 result | `distinct_operations > 2.0× baseline` per service | Baseline | Cue with service and magnitude |
| `NewLogTemplate` | L1c template miner | Template first seen within current cadence window | Baseline | Cue with template content |
| `RestartEvent` | L1b RestartDetector | Gap > 20s followed by resume | (event, not cue) | Event with service and gap duration |
| `HardSignalErrorSpan` | L1a Q1 with status_code = 2 filter | Any root-span error | Hard | Direct signal (immediate L5) |

## Appendix C — Digest Composition Rules and Format

**Composition rules:**

- Numeric values always presented with baseline comparison
- Services sorted by anomaly severity
- Top-N truncation with explicit markers
- Time-series collapsed to summary statistics
- Empty sections omitted
- Project-relative paths only
- Anonymized identifiers per P-047

**Example digest:**

```
WINDOW: 2026-05-14 14:32:00 → 14:33:00 (60s, baseline cadence)
PROJECT: andromeda-pulse (Rust workspace, axum + tokio)
RECENT CHANGES:
  6m ago | commit a4b9c2 "Refactor auth middleware to use new JWT lib"
        | files: src/auth/jwt.rs, src/auth/middleware.rs

OVERALL: degraded (2 high-confidence cues)

SERVICES (rate, error%, p99 vs baselines):
  auth-service     45.2/s ≈    | 12.3% ×15.4 ⚠ | 240ms ×3.0 ⚠
  payment-service   8.1/s ≈    |  0.0% ≈        | 120ms ≈
  inventory         3.2/s ≈    |  0.0% ≈        |  45ms ≈

ATTENTION CUES:
  HIGH:
  - error-rate-spike: auth-service, 12.3% vs 0.8% baseline (×15.4), persistence 45s
  - latency-regression: auth-service::validate_token, p99 240ms vs 80ms (×3.0), persistence 50s

TOP EXCEPTIONS (last 60s, ≥2 occurrences):
  - jose::JWTError "Signature verification failed" [a3f9]
    24 occurrences, first 2m ago, services: auth-service

SERVICE GRAPH (call counts, errors):
  frontend → auth-service: 22 calls, 8 errors
  auth-service → user-db: 44 calls, 0 errors
  frontend → inventory: 12 calls, 0 errors

CRITICAL PATH SAMPLE (slowest root span):
  POST /api/login (240ms, ERROR)
    └─ auth-service::validate_token (220ms, ERROR)
       └─ jose::verify_signature (218ms, ERROR)

CORPUS MATCHES (this workspace):
  - 2026-04-28: fingerprint [a3f9], resolved by reverting jose 4.0→3.5
  - 2026-03-15: similar latency in auth-service after JWT refactor (unresolved)
```

This example consumes approximately 350 tokens.

---

## Closing notes

This document is the foundational baseline for the Pulse v0.2.0 distillation pipeline. It reached stability through three review iterations:

- **v1:** Initial design from research synthesis and product requirements.
- **v2:** Refinement based on review identifying gaps (cold start, self-observability, testability, prompt versioning) and over-optimistic estimates (Drain LOC, JSON failure scope).
- **v3:** Final tightening on remaining edges (LWW semantic gap, hardware profile honesty, prospective threshold application, dual-condition bypass, blocking vs calibration decision separation).

At v3, theoretical refinement has reached diminishing returns. Open decisions are calibration questions that resolve only by running real workload through Conductor scenarios. Three explicit blocking decisions (LLM runtime, Drain spike, mid-incident behavior) must be addressed before implementation chunks can start L4 and L1c work specifically; all other chunks can proceed in parallel.

Future revisions to this document happen through `andromeda-evolve` when implementation chunks encounter unexpected reality. Pre-implementation revisions stop here.

---

## Changelog

### v3 — 2026-05-14

Revision based on second-pass external review. Surgical changes resolving remaining edge cases identified in v2.

**Behavioral changes:**

- **LWW semantic gap closed via active-incident exception.** Cadence-mode digests for active incidents are not LWW-replaced; pulse interprets incident evolution, not just final state. Queue depth capped at 5 active-incident invocations. Resolution summary (`is_resolution_summary: true`) generated when incident transitions to Resolved. L3 queue behavior section rewritten.
- **Hardware Profile Matrix added (L4).** Explicit per-profile SLO tables: `gpu-primary`, `gpu-fallback`, `cpu-primary`, `cpu-fallback`. Tier-2 acceleration disabled on `cpu-primary` profile (CPU inference cannot meet <20s SLO). One-time user notice with model tier switch option. Replaces ambiguous "CPU may be slower" guidance with concrete contract.
- **Prospective application of threshold changes (Hot Reload).** Threshold changes apply to next L1a tick only; do not retroactively re-evaluate rolling window. Prevents notification flood when user lowers thresholds on accumulated baselines. Opt-in backfill action available in Diagnostics for users who want it.
- **Dual-condition suppression bypass (L2).** Bypass triggered when `magnitude > 10×` OR `absolute_error_rate > 5%`. Catches catastrophic regressions on projects with naturally low baselines where relative magnitude alone misses meaningful absolute breakouts.
- **JSON parse failure backoff visibility.** Diagnostics shows countdown timer to next retry attempt. Manual "Retry now" button overrides backoff.

**Structural changes:**

- **Blocking Decisions section added.** Three items separated from calibration "Still open" list: LLM runtime choice, Drain implementation spike, mid-incident interpretation behavior (resolved). Each with owner, deadline, decision criteria.
- **Capability spec formalization TODO added.** Explicit list of 8 architectural concepts that should receive P-XXX numbers in capability spec before implementation start: cadence config, fallback tier, hot reload, magnitude bypass, self-observability, hardware profile, active-incident continuity, prospective threshold application.
- **Property-based testing added to L1b strategy.** EWMA convergence, histogram bucket invariants, RestartDetector idempotence verified via `proptest` crate.
- **Load testing added to Conductor scenarios.** Four load profiles (baseline, high, burst, sustained extreme) with pass criteria SLO-relative.
- **Service lifecycle rationale documented.** Seven states explicitly justified by behavioral distinctions. Boundary durations made configurable (`dormant_after_secs`, `archived_after_secs`) to address "arbitrary timing" concern.

**Estimate adjustments:**

- **Total LOC: 6500-9750** (slight increase from v2's 6150-9400 reflecting v3 additions: dual-condition bypass, active-incident exception, hardware profile detection, resolution summary, configurable lifecycle, prospective application logic).
- **Footnote added: LOC estimates are planning baselines, not contracts.** Components historically prone to expansion may exceed by 30-50%; mental reserve recommended.

**Performance budget additions:**

- Profile-specific SLO matrix (Tier-1/Tier-2/Tier-3 × four hardware profiles)
- Active-incident invocation rate cap (≤ 1 per 5s)
- CPU utilization budget for CPU-inference profiles (< 70% peak)

**Resolved decisions (v2 → v3):**

- Mid-incident interpretation gap → active-incident LWW exception
- CPU vs Tier-2 SLO ambiguity → Hardware Profile Matrix
- Threshold change cue flood → prospective application
- Service lifecycle arbitrariness → configurable boundaries + rationale
- LLM runtime classification → moved from calibration to blocking
- Magnitude bypass hole → dual-condition (relative OR absolute)

### v2 — 2026-05-14

Revision based on first external review identifying gaps and optimistic estimates in v1.

Major additions: L1b state persistence (cold-start fix), Layer 6 Self-Observability (formalized as full layer), testability strategy section, prompt and schema versioning, primary/fallback model tier architecture, configuration hot reload, service identity lifecycle, three-tier triggering with confidence 0.7-0.85 dead zone resolution, L1a Q7 worst-case bounds, Drain template profiling surface, background reflection output semantics, L3 LWW resolution (LLM queue LWW, corpus appends all), tokenizer-model sync.

Estimate revisions: Drain LOC 500-700 → 1000-1500; total 3500-5400 → 6150-9400.

Resolved decisions: LWW vs merge → LWW with corpus full append; restart suppression → magnitude bypass at 10× default; JSON parse → per-incident progressive degradation; hardware envelope → primary/fallback tiers; confidence gap → Tier-2 accelerated cadence.

### v1 — 2026-05-14 (earlier the same day)

Initial draft. Five-layer pipeline (L0-L5) with SQL-first L1a, Rust-state L1b, ingestion-time L1c enrichment, threshold-based L2 cues, structured L3 digests, LLM L4 interpretation, L5 surfaces. Design principles, performance budgets, schema additions, capability mapping. Eight open decisions covering LWW behavior, restart suppression scope, JSON parse failure semantics, hardware envelope, confidence gap, hot reload, Drain params, retrieval depth.
