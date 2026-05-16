# Widget-State-Validation Plan — Pre-Implementation Feasibility Report

**Date:** 2026-05-14
**Project:** andromeda-pulse
**Reviewed plan:** `widget-state-validation-plan-en.md` (draft v1, recipient-supplied)
**Reviewer:** Claude Code (full repository context)
**Last project commit at review time:** `de35e82` (chunks #55+#56 wrap; session 65; Epoch 8 closes)

---

## ⚠ Critical baseline correction (read first)

The plan opens with: _"Critical assumption for validation: route is currently 51/56 done, current chunk is approximately #52."_

**Actual state at review time:** **route §2 is 56/56 complete. Epoch 8 — Polish & ship closed as of session 65 (commit de35e82, ~30 minutes ago).** `state.yaml.last_completed_chunk.route_index = 56`. There is no chunk #52 to "finish first" — chunks #50 through #56 are all committed and tested green.

This shifts the entire **Sequencing** section of the plan (§"Sequencing within v0.1.0"). The hypothesized epoch insertion "between Snapshot and remaining Polish" is no longer possible — Polish is done. The new scope must be **appended as Epoch 9** (or scoped as v0.2.0 feature work) rather than slotted before existing chunks. Details in §7 below.

Two carry-over rot warnings from session 65's combined.md Phase 2 §Step 8 scan are pre-existing and unresolved:
- **Pattern 1**: arch §Cross-cutting Patterns still cites stale `opentelemetry-stdout` text (obs pivot to `tracing-subscriber` JSON propagated к security + tests body annotations but NOT к arch.md body)
- **Pattern 3**: tests-plan §10 declares WebGPU frame p99 `≥20 fps` while obs-plan §10 declares `≤33ms` (= ≥30 fps)

If you proceed with `/andromeda-scope-arch` for the widget-state-validation scope, **these should resolve first** (via `/andromeda-arch` re-run + `/andromeda-tests` re-run) — otherwise the new epoch's arch touches compound the drift.

---

## 1. Pre-flight findings (6 points the plan asked us to verify)

### 1.1 Halo State Pulse formula (chunk #31)

**Location:** `pulse-app/ui/src/halo/HaloCanvas.tsx:124-164` (TS-side uniform computation) + `pulse-app/ui/src/halo/shaders/halo.wgsl:24-64` (WGSL fragment shader applying the breathing envelope).

**Inputs (both React props):**
- `errorRate: f32 ∈ [0, 1]` — drives hue (`lchInterpolate` at `pulse-app/ui/src/halo/lch.ts:20-44` interpolates LCH-space between `--color-primary` Earth Blue and `--color-accent` Alert Burgundy via `colorjs.io`) AND blur envelope (`pulse-app/ui/src/halo/error-rate-to-blur.ts:9-15` linear maps к [4px, 16px])
- `throughputHz` — clamped к [0.8 Hz, 2.4 Hz] (`pulse-app/ui/src/halo/throughput-to-hz.ts:9-17`); phase = `(timestamp / 1000) * 2π * hz` (`HaloCanvas.tsx:164`)

**Source of those props:** **`pulse-app/ui/src/hooks/use-synthetic-widget-metrics.ts:51-52`** — а synthetic simulator producing sine-wave-like values on а 1000ms tick. Comment at line 5 of that file says "real `streams.subscribe_metrics` binding deferred к chunks #34/#35." **Chunks #34 + #35 landed but the widget itself was NOT rewired** — the synthetic simulator survived. This is а major finding (see §5 Hidden complexity).

**WGSL/TS split:** TS computes uniforms (color RGBA + blur_target + pulse_phase as 8-float buffer); shader applies `current_blur = blur_target * (0.5 + 0.5 * sin(pulse_phase))` + smoothstep radial falloff. **No color computation in shader** (per design-tokens.md Session Additions 2026-05-08 audit — colors arrive pre-resolved from design tokens).

**Plan's assumption check:** _"halo formula by error rate"_ — confirmed but oversimplifies. Formula is **two-axis** (errorRate → hue+blur; throughputHz → pulse frequency). The plan's "remove direct error_rate → hue" needs к remove BOTH axes' synthetic source, not just substitute the hue input.

### 1.2 Widget infographics + footer (chunk #32)

**Component:** `pulse-app/ui/src/widget/CompactWidget.tsx:24-71` (orchestrator) + `pulse-app/ui/src/widget/FooterBand.tsx:50-165` (footer) + `pulse-app/ui/src/components/Titlebar.tsx:36-136` (titlebar) + `pulse-app/ui/src/widget/AggregatedBadgeCanvas.tsx:33-79` (constellation badge area).

**Each footer element's data source** (all surveyed):
| Element | Source path:line | Status |
|---|---|---|
| `Ingest 200/s` | `widget-types.ts:25-34` → prop wired from `use-synthetic-widget-metrics.ts:51-52` | **Synthetic** — no real metrics subscription |
| `Error 4.2%` | `widget-types.ts:36-42` → prop from same synthetic hook (line 49) | **Synthetic**. Accent border at threshold 0.05 (`FooterBand.tsx:18`) |
| `Retention 3m / 10m` | `widget-types.ts:44-56` → prop from synthetic hook (lines 51-58, ramps 0→600s over 30 ticks then resets) | **Synthetic** — no real config-toml subscription |
| Service constellation aggregated badge | `AggregatedBadgeCanvas.tsx:33-79` | **Synthetic** — `serviceCount` prop, no actual `BaselineKeeper.activity_windows` source |

**Titlebar:** App icon (constellation-grid glyph) + title text + optional Investigate button (telescope glyph, conditional on `onInvestigateClick`) + Settings button (aperture glyph) + platform-specific `WindowControls`. **No connection dot exists yet** — the plan's §5a "header redesign" is greenfield.

**Plan's assumption check:** The plan implicitly assumed the widget is showing **real** Ingest/Error/Retention numbers. **It is not** — they are synthetic. **Phase 5 must add а "wire real broadcast subscriptions" preliminary chunk** before any redesign work.

### 1.3 Broadcast topics reality

| Topic (arch §Occupied Resources) | Emit site | Webview subscriber |
|---|---|---|
| `pulse://stream/spans` | `crates/buffer/src/consumer.rs:79` → broadcast sender at line 106 | `pulse-app/ui/src/App.tsx:13` (comment only, no real consumer). TauRPC `streams.subscribe_spans` at `bindings/index.ts:92` exposed but NOT wired in production code paths |
| `pulse://stream/metrics` | `crates/buffer/src/consumer.rs:87` → line 106 | TauRPC `streams.subscribe_metrics` exposed at `bindings/index.ts:91`. Comment-only reference at `use-synthetic-widget-metrics.ts:5` |
| `pulse://stream/logs` | `crates/buffer/src/consumer.rs:95` → line 106 | TauRPC `streams.subscribe_logs` exposed at `bindings/index.ts:90`. Comment-only reference at `HaloCanvas.tsx:9` |
| `pulse://stream/snapshot-progress` | **NOT IMPLEMENTED** | — |
| `pulse://stream/plugin-events` | **NOT IMPLEMENTED** | — |

**Material finding:** 3 of 5 declared topics are Rust-side emitting, but **zero are wired to actual webview consumers**. The widget reads error/throughput from synthetic; the dashboard reads tracing via `traces.query`/`metrics.query`/`logs.query` TauRPC paginated APIs (NOT live streams). The streams.* namespace is declared infrastructure waiting for consumers.

**Plan's assumption check:** The plan implies adding `pulse://stream/connection-state` / `/anomalies` / `/restart-events` / `/incidents` (= 4 new topics) on top of existing 5. The reality is the existing 5 are 3 declared-and-emitting + 2 declared-and-stubbed. Adding 4 more pushes the namespace к 9 topics, of which **5 will lack webview subscribers** at first. Recommend: bundle Phase 5's incident-card binding chunk to **include the widget's first real `streams.*` subscription** (collapse the chunk #34/#35 deferred wiring into the Phase 5 redesign).

### 1.4 Snapshot curation pipeline (chunks #39 + #44) — most important finding

**ALL anomaly + curation primitives ARE implemented:**

| Primitive | Location | Algorithm |
|---|---|---|
| Dedupe spans | `crates/snapshot/src/dedupe.rs:30` (`dedupe_spans`) | (service_name, name, 100ms-bucket) HashMap |
| Anomaly: latency outliers | `crates/snapshot/src/anomaly.rs:70` (`detect_latency_outliers`) | **Z-score**; threshold 3.0 (`LATENCY_OUTLIER_Z_THRESHOLD`); severity ∈ [0, 100] |
| Anomaly: error correlation | `crates/snapshot/src/anomaly.rs:113` (`detect_error_correlation`) | `status_code == 2` clustering per service; min cluster 3 errors |
| Anomaly: cardinality spikes | `crates/snapshot/src/anomaly.rs:161` (`detect_cardinality_spikes`) | unique-span-name count per service vs median; 2.0× multiplier |
| Critical-path extraction | `crates/snapshot/src/critical_path.rs:18` (`extract_critical_path`) | DAG longest-path traversal via parent_span_id backlinks |
| p50/p95/p99 aggregation | `crates/snapshot/src/aggregation.rs:21` (`aggregate_metrics`) | nearest-rank percentile over sorted durations; global + per-service |

**Reusability assessment:** **Fully reusable as algorithms — locked by visibility.** All primitives operate on `&[SpanRecord]` (pure data, no DuckDB coupling, no I/O). However:

- All functions are `pub(crate)` — invisible to other workspace crates per Rust module discipline + CLAUDE.md "one crate per module" rule
- Logic operates on **post-query materialized Vec<SpanRecord>**, not on streaming spans from а broadcast subscription
- Coupled к snapshot crate's `SpanRecord` type definition

**STRONG argument для the plan's Phase 2 (BaselineKeeper):** Extract а new `crates/curation/` shared library housing these 6 primitives + their `SpanRecord` type. snapshot depends on it (refactor — chunks #39 + #44 surface unchanged); new `crates/triage/` (or `crates/baseline/`) depends on it for **streaming variants** of the same algorithms.

**The plan completely misses this** — it proposes а greenfield BaselineKeeper using `tdigest` + `dashmap` (neither currently а workspace dep) without acknowledging 6 algorithmic primitives already in the codebase. Even if streaming-vs-batch is а meaningfully different shape, **error-correlation + cardinality-spike + critical-path** are batch-only operations that the BaselineKeeper would benefit from sharing.

### 1.5 Service identity in spans

**Schema:** `service_name VARCHAR NOT NULL` in `spans` table (`crates/buffer/src/schema.rs:31`).

**Population path:** `crates/buffer/src/appender.rs:101-114` (`extract_service_name`):
1. Reads OTLP `Resource.attributes` (line 105)
2. Matches key `"service.name"` (line 106)
3. Extracts `StringValue` from `AnyValue.value` (lines 107-108)
4. **Fallback when absent:** returns empty string (line 113)

**Edge cases:**
- Span с no `service.name` attribute → stored as `""` (empty string)
- Span с `service.name` = `""` → stored as `""` (same as above; indistinguishable)
- Span с whitespace-only `service.name` → stored verbatim (no trim)
- Multiple service.name attributes — first match wins (HashMap iteration order)

**Plan's Q3 ("what does baseline keeper do when span arrives without `service.name`?"):** Recommend **drop silently in BaselineKeeper** (don't create а phantom "unknown" service that bloats activity windows + UI dots). Reason: empty-string services are typically test fixtures / dev experimentation; promoting them к а constellation dot is high-noise / low-signal. Add а debug-level `tracing::warn!` count of dropped-spans-per-tick для observability.

### 1.6 What counts as "error" in current halo formula

**Current halo formula (chunk #31 final):** **NONE of the broadcast topics feed the halo's error signal.**

Error rate flows как follows:
1. Webview queries traces via TauRPC `traces.query(args)` → `PaginatedResponse<TraceRow>` (`bindings/index.ts:84`)
2. TraceRow.`error_count` field aggregated client-side (each row contributes 1 if `status_code == 2`, else 0; `crates/viz/src/query.rs:162`)
3. **BUT** — as noted in §1.1 — chunks #31/#32 widget passes а **synthetic** errorRate, not the queried one. The architectural pathway exists; the widget never consumes it.

**Storage shapes:**
- Spans: `status_code: i32` (OTLP enum: 0=Unset, 1=Ok, 2=Error)
- Logs: `severity_number: i32` (raw OTLP scale 1-24; 8=WARN, 9-12=ERROR, 13-16=FATAL)
- Exception events: `span_events` table **exists in schema but is never populated** (chunk #39 stub; deferred к follow-on chunk)

**Plan's Q6 implicit assumption:** the plan's `RetryStorm` detector keys off `exception.type` + `exception.stacktrace` (Phase 3 §3a). **These fields are not currently captured** — exception events ride on spans as `Span.events` (per OTLP spec) but the buffer's `span_events` table population is deferred. The plan needs an explicit dependency: **"Phase 3 requires chunk implementing span events ingestion + storage первой"** — this is а meaningful prerequisite chunk that isn't called out.

---

## 2. Plan feasibility per phase

### Phase 1 — Connection state machine: **Feasible с adjustment**

Realistic given current code. The `LastIngestTracker` (atomic `Instant` updated in ingest hot path) is а natural extension of `crates/ingest/src/mpsc.rs` (where the hot-path receiver already exists).

**Adjustment:** Recommend placing `ConnectionState` machine **inside `crates/ingest/`** as а new module rather than а new top-level crate. `ingest` already owns receiver lifecycle (gRPC + HTTP) and watches mpsc hand-off; bolting Connection state onto its existing instrumentation surface is less workspace churn (skip new crate + new Cargo.toml + new module-boundary discussion). The `pulse://stream/connection-state` broadcast topic is а new topic regardless; can be emitted from inside ingest just like other topics are emitted from buffer.

**Missing dependency:** `ReceiverFailed` state requires а callback path from receiver-task panics back к the state machine. Currently receiver tasks panic-out via `std::panic::set_hook` → `tracing::error!(target: "app.panic.fatal", ...)` (per .claude/rules/observability.md). The state machine needs к either subscribe к panic events OR have an explicit "receiver task health" channel.

**Realistic chunk count:** **1-2 chunks** (1 if module-inside-ingest; 2 if standalone crate).

### Phase 2 — Baseline keeper: **Feasible but plan misses huge reuse opportunity**

See §1.4 finding. Recommended adjustment: **insert "Extract `crates/curation/` shared library" as а prerequisite chunk**. Then BaselineKeeper composes:
- **Streaming variants** of EWMA + t-digest + rolling-window (new — these are streaming, not batch)
- **Batch primitives** reused from `crates/curation/` for restart-suppression / error-correlation / cardinality-spike contextual analysis

**Missing dependency from plan:** `tdigest` crate (or `tdigests`) not in workspace; needs к be added к `Cargo.toml [workspace.dependencies]` + `deny.toml [bans] skip` list. `dashmap` similarly. Both cargo deny / cargo audit clean per current state (no banned advisories для these crates as of 2026-05-14 RustSec DB), but cargo deny `multiple-versions = "deny"` may bite if transitive duplicates exist.

**Restart-awareness coordination:** The plan's "Phase 2 subscribes к Phase 3 RestartEvent" creates а **circular dep risk** — Phase 2 + Phase 3 both subscribe к ingest broadcast, AND Phase 2 subscribes к Phase 3. Either:
- Both subscribe к а shared `pulse://stream/ingest-internal-events` (Phase 2's restart suppression listens for `RestartEvent` from Phase 3 on this channel)
- OR collapse Phase 2 + Phase 3 detection into а single state holder (single crate, no cross-subscription)

I recommend **option 2** — single `triage` crate с modules `baseline` + `pattern`, internal channels. Simpler dep graph; matches workspace discipline (snapshot crate has multiple internal modules — anomaly, dedupe, critical_path — without cross-crate ceremony).

**Realistic chunk count:** **4 chunks** (1 extract curation + 3 for plan's described work; not the plan's 3).

### Phase 3 — Pattern detector: **Partially feasible — exception events gap**

Restart-event detector (3b) is feasible immediately — а per-service `last_seen_span_timestamp` tracker против `Instant::now()`-derived gap is straightforward.

**Retry storm detector (3a) blocked on prerequisite:** Exception fingerprinting requires `exception.type` + `exception.stacktrace` fields. Per §1.6, **span_events table exists in schema but is not populated**. Either:
- Pre-requisite chunk: implement OTLP `Span.events` ingestion into `span_events` table
- OR Defer Phase 3a to а post-MVP scope (Phase 4 can ship without retry storm; ErrorRateSpike from Phase 2 covers similar ground at less precision)

**Missing dependency from plan:** Span-events ingestion chunk. Roughly equivalent в complexity к chunk #25 (Query routers) — schema is already defined; the work is in `appender.rs` extending к decode span events from OTLP + new query path.

**Realistic chunk count:** **2-3 chunks** (1 span-events-ingestion prerequisite + 2 for plan's described work; OR defer 3a + 1 chunk для just 3b).

### Phase 4 — Severity classifier + Incident state: **Feasible as described**

Rule-based engine is а straightforward fit; no new infrastructure beyond existing TauRPC procedure boilerplate.

**Adjustment:** The plan's `Incident.evidence_refs: EvidenceRefs` containing `span_ids: Vec<SpanId>` (8 bytes each) + `trace_ids: Vec<TraceId>` (16 bytes) is а security-touchpoint — these are user-data attribute fragments. Per `.claude/rules/security.md` "NEVER log raw OTLP attribute values, span/log/metric content payloads..." — Incident.detail rendering needs к use anonymized identifiers (e.g., `query_id` / `param_count` style) when emitted to the obs JSON log. **Webview-side display via TauRPC IS the boundary** where the actual IDs cross, gated by the existing `AppError`-style sanitization discipline.

**Missing detail from plan:** `incidents.acknowledge(id)` — what does "acknowledge" do? Plan says "user-driven dismiss". Implication: incidents have а user-state field (`acknowledged_at: Option<SystemTime>`) and the UI hides acknowledged incidents OR sorts them lower. Either way, this needs к be in the `Incident` struct + the dedup logic ("don't re-create acknowledged incidents within а cool-down period").

**Realistic chunk count:** **3 chunks** (matches plan estimate).

### Phase 5 — Widget redesign: **Feasible but plan-underspecified**

Largest scope. The plan's 4-5 chunks is **optimistic by 1-2 chunks** because:

1. **Real-data binding prerequisite** (currently missing). Before redesign, the widget MUST stop using `use-synthetic-widget-metrics.ts` and start using real streams (probably `streams.subscribe_metrics` for throughput + а new `incidents.list_active` poll for cumulative_severity). That's **1 chunk** in itself, not folded into header redesign.

2. **Halo refactor cascades to chunk #34 ConstellationCanvas + dashboard consumers.** The halo's API contract changes from `(errorRate, throughputHz)` к `(connectionState, cumulative_severity, activity_state)`. Chunk #34's per-service Halo dots в the constellation map consume the same `HaloCanvas` component; they need а corresponding refactor OR continue с old props (impossible if HaloCanvas signature changes). Plan should call out the cascade explicitly.

3. **Service constellation aggregated badge replacement:** The existing badge wraps `HaloCanvas` (per §1.2 finding). Replacing с "actual constellation dots" means **removing the HaloCanvas wrap** + adding а NEW per-dot rendering. The plan describes the new behavior but understates the surgery — the existing component is а composite, not а thin display layer.

4. **Chrome cleanup interacts с motion-reduce respect.** The footer removal removes а render path; if the recent-incident-dot-history replacement uses motion (likely), reduced-motion respect needs к propagate. Plan calls out the design touchpoint but not the implementation cost.

**Realistic chunk count:** **5-6 chunks** (1 real-data binding + 5 redesign chunks including connection-dot header + halo refactor + constellation replace + incident card + chrome cleanup; possibly +1 cascade-update chunk for chunk #34's ConstellationCanvas).

### Phase totals

| Phase | Plan estimate | Realistic (this review) | Delta |
|---|---|---|---|
| 1 — Connection | 2 | 1-2 | -1 to 0 |
| 2 — BaselineKeeper | 3 | 4 (incl. curation extract) | +1 |
| 3 — Pattern | 2 | 2-3 (incl. span-events prereq if 3a kept) | 0 to +1 |
| 4 — Triage | 3 | 3 | 0 |
| 5 — Widget | 4-5 | 5-6 (incl. real-data wire-up + chunk #34 cascade) | +1 to +2 |
| **Total** | **14-15** | **15-18** | **+0 to +3** |

Plan's estimate **slightly optimistic** but within reasonable margin. Top-of-range total reflects (a) curation extraction + (b) span-events prereq + (c) widget real-data binding — all three are non-negotiable per current code state.

---

## 3. Crate organization opinion (answer to plan's Q1)

**Recommendation: ONE umbrella crate + ONE extracted shared library.**

```
crates/
├── curation/   (NEW; extracted from snapshot)
│   ├── dedupe.rs           (moved from snapshot/src/)
│   ├── anomaly.rs          (moved from snapshot/src/; pub-ify primitives)
│   ├── critical_path.rs    (moved from snapshot/src/)
│   ├── aggregation.rs      (moved from snapshot/src/)
│   └── contract.rs         (SpanRecord type + curation result types; pub)
├── triage/     (NEW; umbrella for plan's Phases 1-4)
│   ├── connection.rs       (Phase 1 state machine; subscribe к ingest)
│   ├── baseline.rs         (Phase 2 streaming EWMA/t-digest/rolling)
│   ├── pattern.rs          (Phase 3 retry-storm + restart detection)
│   ├── classifier.rs       (Phase 4 rules engine)
│   ├── incident.rs         (Phase 4 incident state store + dedup + auto-resolve)
│   └── contract.rs         (ConnectionState + AnomalyCandidate + PatternCandidate + Incident + Severity + AppError-friendly variants)
├── ingest/ buffer/ viz/ ui-bridge/ snapshot/ workspace-detector/ plugins/ mcp-server/  (existing)
└── pulse-app/ xtask/                                                                    (existing)
```

**Rationale:**
- **vs plan's 4 separate crates** (connection / baseline / pattern / triage): 4 new crates pushes workspace к 14 members с very tight inter-dep coupling. Snapshot's precedent (anomaly + dedupe + critical_path + aggregation as modules of one crate) shows this domain fits cleanly into а single-crate umbrella.
- **vs all-in-snapshot:** snapshot's role is "on-demand markdown generation for paste-to-AI", не "real-time triage". Conflating live state с batch generation muddies the crate boundary + the cargo deny / public-api surface.
- **Connection state inside ingest** (plan §"crate: new connection crate (or а module inside ingest — debatable)"): see §2 Phase 1 — preferred is module-inside-ingest. The triage umbrella above leaves room для it; you could place connection inside ingest AND keep the broadcast topic separate from triage's other topics.

Final crate count: **12** (existing 10 + curation + triage), or **11** if connection-inside-ingest. Manageable.

---

## 4. Estimated chunks revision

Plan: 14-15. **Revised: 15-18** (median 16). See §2 phase-by-phase table.

Breakdown:
- Phase 1: 1-2 (likely 1; ingest module)
- Phase 2: 4 (1 curation extract + 3 baseline-keeper proper)
- Phase 3: 2-3 (likely 2 if 3a deferred; 3 if span-events prereq kept)
- Phase 4: 3
- Phase 5: 5-6 (1 real-data wire-up + 4-5 redesign + possibly 1 chunk #34 cascade)

**Optimistic delta breakdown:**
- +1 chunk: Curation extraction (cleanup work; would be technical-debt чтобы avoid)
- +1 chunk: Widget real-data binding (must happen; currently synthetic)
- +1 chunk maybe: chunk #34 ConstellationCanvas cascade-update (depends on HaloCanvas API change boldness)

Net: plan's 14-15 → **16 average** (range 15-18). Plan is reasonable but should add 2-3 chunks к match reality.

---

## 5. Hidden complexity (what the plan misses)

### 5.1 Widget IS synthetic (largest gap)

**The plan starts Phase 5 as if the widget renders real numbers.** It does not — `use-synthetic-widget-metrics.ts` generates ingest rate / error rate / retention via sine-wave-ish synthetic ticks. Real binding к `streams.subscribe_metrics` was deferred at chunk #34/#35 and never re-attempted.

Until the widget reads real data, redesign is rearranging deck chairs on а synthetic ship. **The very first chunk of this epoch should be "wire real broadcast subscriptions"** — this is а Phase 0 / "prerequisite" chunk that the plan doesn't have.

### 5.2 Cascade from HaloCanvas API change → chunk #34 ConstellationCanvas

Halo refactor (§5b в plan) changes `HaloCanvas` props from `(errorRate, throughputHz)` к `(connectionState, cumulative_severity, activity_state)`. Two production consumers exist:
- `AggregatedBadgeCanvas.tsx:44-48` (compact widget badge) — replaced by service constellation per plan §5c
- `ConstellationCanvas.tsx` (dashboard constellation map per chunk #34) — **NOT mentioned in plan**

The dashboard constellation's per-service Halo dots are а semantic match (per-service severity), so the cascade is а natural extension — but it's а chunk's worth of work that the plan elides.

### 5.3 Span-events ingestion prerequisite for retry storm detector

Plan Phase 3 §3a fingerprints exceptions, but `span_events` table is unpopulated. Either pre-implement span-events ingestion OR defer retry-storm.

### 5.4 Snapshot's anomaly primitives already exist — reuse vs duplicate

See §1.4 + §3. Strong argument для extract-shared-curation-crate rather than greenfield baseline-keeper.

### 5.5 `tdigest` / `dashmap` workspace dep additions touch supply-chain CI

Both crates not currently in workspace. Adding them:
- Cargo.toml `[workspace.dependencies]` extension
- `deny.toml` review для `multiple-versions = "deny"` посure (tdigest transitive deps may collide с existing crates)
- `cargo audit` runs against RustSec DB on every PR; new advisories possible
- The bindings.ts transient regeneration pattern (testing.md 2026-05-13) compounds per chunk — multiple TauRPC additions across phases means multiple wrap-session restores. Consider bundling all new TauRPC procedures (connection.current_state + incidents.list_active + incidents.acknowledge — 3 new) into a single chunk to minimize the regeneration tax.

### 5.6 Calibration surface cross-cutting

Plan §"Calibration surface" mentions `~/.andromeda-pulse/config.toml [triage]` section. This requires:
- `crates/ui-bridge/src/contract.rs` `Settings` struct extension (add `triage_thresholds: TriageThresholds`)
- `Settings::load_from_data_dir()` boot helper extension (per chunk #30 pattern)
- `get_settings` / `update_settings` round-trip behavior for the new field
- Validation: thresholds must be positive; persistence multipliers within [0.1, 10.0]; etc.
- Settings modal display + edit UX (read-only initially per plan; later editable)

This is а cross-cutting touchpoint that adds work к Phase 4 OR Phase 5; plan doesn't allocate а dedicated chunk for it.

### 5.7 Acknowledge state semantics

`incidents.acknowledge(id)` — plan says user-driven dismiss. Open questions:
- Does acknowledge persist across app restarts? (probably yes; store in `~/.andromeda-pulse/`)
- Does acknowledge prevent re-creation of same-kind+scope incident? (probably yes для some cool-down)
- Does acknowledge affect halo severity computation? (per plan: max severity wins among active — acknowledge moves incident к "Acknowledged" status, presumably out of `cumulative_severity` count)

Not blockers but should be spec-explicit before implementation.

### 5.8 Halo's existing `errorRateToBlur` linear shape may not survive

Plan §5b says halo hue/breathing computed from `cumulative_severity`. Existing `errorRateToBlur` is linear [0,1] → [4px, 16px]; severity is discrete (`Autonomous` / `Suggested` / `Curious`). Translation table needs design spec (е.g., `Autonomous → 16px; Suggested → 12px; Curious → 8px; None → 4px`). Touches design-system specialist plan.

---

## 6. Specialist plan touch density

| Plan | Touch density | What changes |
|---|---|---|
| **architecture.md** | **HEAVY** | §Occupied Resources: +2 crates (curation + triage); +4 broadcast topics (connection-state, anomalies, restart-events, incidents); +3 TauRPC procedures (`connection.current_state`, `incidents.list_active`, `incidents.acknowledge`). §Established Decisions: new entry для rule-based classifier с model-interface upgrade path (mirror `wgpu` pattern). §Existing Scopes: "None" → first scope "widget-state-validation". §Cross-cutting Patterns: connection state machine ownership boundary (in ingest? в triage?) |
| **test-plan.md** | **HEAVY** | §6 E2E critical paths: new P8 "Connection state transitions" + P9 "Anomaly detection → incident → acknowledge flow". §7 fixtures: synthetic span streams с controlled error rate / latency injection / restart gap patterns. §10 quality gates: classifier rule unit tests + EWMA convergence tests + t-digest accuracy tests. §11 anti-patterns: "NEVER assert real wall-clock в triage tests" (use injected clock) |
| **obs-plan.md** | **MEDIUM-HEAVY** | §3 Heartbeat ticks: 4 new sources (connection.tick, baseline.tick, pattern.tick, triage.tick). §5 Metrics: `metric.triage.incidents_active_count`, `metric.baseline.ewma_short_window_size`, `metric.pattern.exception_fingerprints_tracked`. §10 SLO: anomaly-detection latency budget (event-to-incident-created ≤2s p99); per-tick CPU budget |
| **security-plan.md** | **MEDIUM** | §Logging vectors extension: Incident.title / .detail must use anonymized identifiers in obs log emission (verified at TauRPC bridge by AppError sanitization). §API Security: triage subscribers must respect existing redaction layers (security plan §Logging "Field redaction is applied at the subscriber layer"). No new auth model (loopback boundary preserved) |
| **design-system.md** | **MEDIUM** | §Color Palette: severity-to-color mapping (Autonomous=Alert Burgundy; Suggested=warm amber; Curious=Earth Blue) — explicit gradient table. §Motion: halo refactor's new hue/breathing inputs; reduced-motion mapping table. Connection dot palette (Listening=dim; Receiving=subtle glow; Idle=static; Stalled=warm warning; ReceiverFailed=burgundy). Incident card visual spec (severity indicator + title + detail typography) |
| **layout-templates.md** | **MEDIUM** | New components: connection dot в titlebar (6-8px circular indicator near app icon); incident card slot (below halo; max-1-visible); recent-incident-dot-history footer (last 3 resolved as small dots). Per-surface: compact widget redesign per plan §5; dashboard incident card integration (separate from widget instance) |
| **a11y-plan.md** | **MEDIUM** | §5 ARIA: incident card aria-live=polite (announce new Suggested+ severity); aria-busy during Investigate transition. §6 Focus order: connection dot focusable s aria-label `"Connection: {state} — {N} services active, last span {ago}"`; incident card focusable; Investigate button focus order. §11 Status messages: incident lifecycle announcements |

**Run cost estimate for `/andromeda-scope-arch`:** 6 specialist plans × heavy/medium edits each = full 6-specialist re-derive run. Probably 30-60 minutes for the full scope. The route epoch insertion + chunk decomposition adds another /andromeda-route re-run. Conservative estimate: **2 hours total for /andromeda-scope-arch + /andromeda-route convergence**.

---

## 7. Sequencing recommendation

**Plan's sequence is no longer applicable** — Polish & ship epoch closed (state.yaml.last_completed_chunk.route_index = 56 as of session 65; commit de35e82). There is no chunk #52 к finish.

**Three viable sequencing paths:**

### Path A: Append as Epoch 9 (recommended for technical clarity)

```
1. [done] Epoch 1-8: route §2 complete (56/56 chunks)
2. Resolve pre-existing rot warnings (1-2 chunks):
   a. /andromeda-arch к annotate stale opentelemetry-stdout text (rot Pattern 1)
   b. /andromeda-tests к align frame p99 threshold к obs-plan ≤33ms (rot Pattern 3)
3. /andromeda-scope-arch "widget-state-validation" — full 6-plan re-derive + route Epoch 9 append
4. Epoch 9 implementation (15-18 chunks per §4):
   - Chunk #57: real-data binding wire-up (widget stops using synthetic)
   - Chunk #58: curation extraction (snapshot → curation lib)
   - Chunks #59-#60: Phase 1 connection state machine
   - Chunks #61-#63 (or #64): Phase 2 baseline keeper
   - Chunks #64-#65 (or #66): Phase 3 pattern detector (incl. span-events prereq если 3a kept)
   - Chunks #66-#68 (or #69): Phase 4 triage / incident state
   - Chunks #69-#73 (or #74): Phase 5 widget redesign
5. v0.2.0 ship (or v0.1.0 if you want то delay шип) + operator-driven DEFERRED Items 1-6 from chunks #52/#53
```

**Pro:** clean technical history; v0.1.0 still ships at chunk #56 ship-ready milestone; widget-state-validation is а major feature warranting а v0.2.0 minor bump.
**Con:** delays "honest widget" к v0.2.0 (may not match product vision если widget honesty is core к v0.1.0 differentiator).

### Path B: Squeeze as pre-v0.1.0 polish (matches plan's spirit)

```
1. [done] Epoch 1-8: route §2 complete
2. Resolve rot warnings (1-2 chunks; as in Path A step 2)
3. /andromeda-scope-arch "widget-state-validation" — same as Path A step 3
4. New Epoch 9 implementation (same 15-18 chunks)
5. Re-verify smoke + a11y + perf SLO + flakiness gates STILL pass after the widget rewrite (~1 verification chunk; OR fold into Phase 5 closing chunk)
6. v0.1.0 ship (single major release с honest widget from day 1)
```

**Pro:** v0.1.0 ships с the honest widget per product vision; no версионная split.
**Con:** v0.1.0 release date pushed by ~15-18 chunks (estimate: 4-8 weeks). The "ship-ready" milestone reached в session 65 becomes deferred.

### Path C: Hybrid — v0.1.0-rc.1 ships chunk-#56 state; widget redesign в v0.1.0 final

```
1. Tag v0.1.0-rc.1 at commit de35e82 (current state) → operator-driven DEFERRED Items 1-6 acquisition starts
2. Resolve rot warnings (parallel к DEFERRED prep)
3. /andromeda-scope-arch widget-state-validation → Epoch 9 implementation
4. v0.1.0-rc.2 at end of Epoch 9
5. v0.1.0 final shipped when both Epoch 9 done AND DEFERRED Items 1-6 done
```

**Pro:** parallelize widget redesign с operator-prep (paid creds acquisition + EV cert + Apple Dev ID are independent of code work).
**Con:** more release engineering ceremony (rc tags, etc.).

**My recommendation: Path A.** Cleanest technical separation; v0.1.0 ships с the substantial work session 65 just completed; widget redesign is materially new feature work warranting а v0.2.0 minor bump. Path B is acceptable if product strategy says "no v0.1.0 without honest widget"; the timeline cost is real but the release coherence is cleaner.

**On the question "agree с plan's sequence (after #52, before #54-56)?"** — Answer: not applicable; chunks #54-56 are done. The plan was written against an earlier state. The new sequencing must append, not insert.

---

## 8. Open questions answers

### Q1 — Crate organization

**See §3 above.** TL;DR: 1 umbrella `triage` crate (modules: connection, baseline, pattern, classifier, incident) + 1 extracted `curation` shared lib (refactor from snapshot). Net workspace +2 crates (10 → 12). Plan's 4 separate crates is overkill; matches workspace boundary discipline poorly.

### Q2 — Snapshot curation overlap

**Strong reuse opportunity. See §1.4 + §3.** Extract `crates/curation/` shared lib housing snapshot's anomaly + dedupe + critical_path + aggregation primitives. Both snapshot AND triage depend on it (snapshot for batch, triage for batch-in-pattern-context after streaming-aware EWMA/t-digest produces candidate windows).

The plan implicitly proposes greenfield duplication via `tdigest`+`dashmap`. This works but **adds maintenance debt** — two implementations of latency-outlier detection will drift over time, and the snapshot version (Z-score) vs triage version (EWMA-baseline-relative) might disagree on edge cases. Better to: extract shared batch primitives + добавь NEW streaming primitives (EWMA + t-digest in а new file).

### Q3 — Service identity edge cases

**Recommend: drop spans с empty `service.name`** в BaselineKeeper (see §1.5 reasoning). Emit а debug-level `tracing::warn!(target: "triage.service_id_missing", count = N)` per tick aggregating dropped count; surface в obs log without per-span verbosity.

Edge case to call out separately: spans с `service.name` = `""` (explicit empty string) AND spans missing the attribute look identical in storage (both empty string). Plan should treat them uniformly (drop). Future: add а sentinel-distinction column в spans schema (`service_name_origin: "explicit" | "absent" | "malformed"`) if granularity becomes important.

### Q4 — t-digest crate choice

**Recommend `tdigest` 0.x** (the crate by `MnO2`; established, permissive Apache-2.0 license, no_std-friendly, ~1.5k downloads/day per crates.io trend, no open RustSec advisories as of 2026-05-14). Alternatives:
- `tdigests` (newer fork, less battle-tested)
- `tdigest-rs` (early-stage; smaller adoption)

Check `cargo deny check bans` posture после addition; expect 1-2 transitive duplicates worth skipping (`num-traits` ecosystem) but nothing к block `multiple-versions = "deny"` posture per chunk #45/#48 pattern.

### Q5 — Settings UX for calibration

**Recommend Hybrid (config.toml + read-only display initially):**

- v0.1.0 (or first Epoch 9 ship): config.toml `[triage]` section editable; Settings modal shows current values **read-only** с link к config docs
- post-v0.1.0 backlog: full editable Settings UX
- **Env vars NO** — adding `ANDROMEDA_PULSE_TRIAGE_*` к the reserved namespace requires `/andromeda-evolve --allow-arch-registry` per arch §Occupied Resources discipline. config.toml is а cleaner extension point (the file already exists; modal load handler already plumbed).

Rationale: editable UI = а material amount of webview work (validation messages, dirty-state, save flow per chunk #38 SettingsModalForm precedent); not worth bundling into Epoch 9. Expert-operator escape hatch via config.toml is enough при v0.1.0.

---

## 9. Anything else (the full-code-context view)

### 9.1 Pre-existing carry-over drift к resolve FIRST

Session 65 wrap's combined.md emitted 2 rot warnings:
- Pattern 1: arch §Cross-cutting Patterns stale `opentelemetry-stdout` reference (obs pivot к `tracing-subscriber` propagated к security + tests body annotations но not arch)
- Pattern 3: tests-plan §10 frame p99 `≥20 fps` vs obs-plan §10 `≤33ms` cross-plan inconsistency

If Path A or B sequencing is chosen, **resolve these via `/andromeda-arch` re-run + `/andromeda-tests` re-run BEFORE `/andromeda-scope-arch`**. Otherwise the scope-arch run inherits broken-plan-to-plan baseline + multiplies drift on touched arch surfaces.

### 9.2 bindings.ts transient regeneration multiplier

Every TauRPC procedure added increments the bindings.ts regeneration friction (per .claude/rules/testing.md 2026-05-13 entry). Each procedure brings 1 commit's worth of `git checkout HEAD -- bindings.ts` operations during wrap-session.

**Mitigation: bundle all new TauRPC procedures into the SAME chunk** (or 2 adjacent chunks). The plan's distribution across phases means:
- connection.current_state — Phase 1
- incidents.list_active — Phase 4
- incidents.acknowledge — Phase 4

If Phase 1 lands chunk before Phase 4, the bindings.ts regeneration tax accrues twice. Recommendation: **fold connection.current_state into Phase 4's TauRPC chunk** (Phase 4 has 3 chunks; let chunk 3 of Phase 4 land ALL new TauRPC procedures simultaneously).

### 9.3 Chunk #51's deferred tauri-driver headful tests intersect this scope

Chunk #51 ACTIVE scope landed install-launch-ingest-query smoke harness; tauri-driver headful UI tests for tray + window state P5 were DEFERRED follow-on. The widget redesign в Phase 5 makes those headful tests more valuable (window-state interactions: incident card → opens dashboard → focus restoration). Consider **bundling the chunk #51 deferred WebdriverIO + Mocha + 4 npm devDeps setup INTO Phase 5's first chunk** — it adds 1 chunk worth of substrate but pays off с per-component visual verification.

### 9.4 Halo's `pulse://stream/connection-state` semantically overlaps `health` IPC

Connection state machine emits к а new broadcast topic AND exposes via TauRPC `connection.current_state()`. Meanwhile, existing `health` IPC (arch §Standard Contracts) already exposes `subsystems.otlp_grpc_receiver.status` / `subsystems.otlp_http_receiver.status` / etc. Cross-domain reconciliation needed:
- Is connection.current_state а **specialized subset** of health (just receiver state, no buffer/ingest)?
- Or **orthogonal** (connection focuses on "data arriving" vs health focuses on "subsystems initialized")?

Probably orthogonal (health tracks subsystem state; connection tracks data flow). But this should be **explicit в the new arch entries** + ideally tested cross-comparison (when receiver bind fails, health.subsystems.otlp_grpc_receiver = `error` AND connection.current_state = `ReceiverFailed` — both signals should agree).

### 9.5 Trauma test: what if span ingestion rate в the wild is bursty?

Plan's BaselineKeeper assumes ~steady stream of spans for EWMA convergence. Reality для а solo-dev local setup: bursty (long quiet periods + occasional dev-server-rebuild traffic spikes). EWMA с default alpha tuned for 5-min effective window may produce false `ServiceWentSilent` warnings during a normal "developer-on-lunch" pattern.

Mitigation suggestion: **per-service "activity_floor"** учиться from histograms — а service that historically goes quiet for 10-min stretches shouldn't trip `Suggested` severity on its 10th quiet minute. This is more sophisticated than plan's "Active/Quiet/Silent" 3-state. Either add к Phase 2 OR document as known v0.1.0 false-positive class (config.toml threshold к make EWMA per-service tunable).

### 9.6 Restart event detection has а corner case

Plan §3b detects restart via "per-service last seen span timestamp gap > G". If а user runs `cargo run` repeatedly during development, EVERY rebuild registers as а restart. The RestartEvent suppression (Phase 2 suppresses anomaly emission for 60s after RestartEvent) means **during active development the system effectively suppresses ALL its detection capability**. This is а pathological dev pattern, but it's the very pattern the product targets.

Mitigation: maybe restart events shouldn't suppress ALL anomalies — only "ErrorRateSpike with persistence < 30s" (catch real spikes, suppress only the "startup error burst" false positive). Plan currently suppresses everything за 60s.

### 9.7 Acknowledge state pollution

If а dev rebuilds the app and the same incident kind+scope re-triggers, did the previous incident's "acknowledge" carry over? If yes → user sees ZERO halo signal during the second occurrence (acknowledge transfer = incident never created). If no → user sees burgundy halo они just dismissed. Both pathological.

Recommendation: **acknowledge has а natural cool-down** (е.g., 5 minutes). After cool-down expires, the next same-kind+scope detection creates а new incident. Document explicitly в Phase 4 plan.

### 9.8 Settings persistence for connection-dot-tooltip preference

Plan §5a "Tooltip on connection dot shows ingest rate + last-span-ago". Some users may want this minimized OR off. Add к `[triage]` config: `show_connection_tooltip: bool` (default true), persistable per the Settings struct extension.

---

## Closing notes

The plan is **well-structured but written against an outdated route state**. The Phase 1-5 decomposition is reasonable; the chunk estimates are slightly optimistic; the sequencing assumption is materially wrong (route IS 56/56 complete, не 51/56).

**Top three corrections к incorporate before `/andromeda-scope-arch`:**
1. Update sequencing к Path A (append as Epoch 9) or Path B (extend Polish & ship)
2. Add prerequisite chunks: real-data binding wire-up (Phase 5 prereq) + curation extraction (Phase 2 prereq) + optional span-events ingestion (Phase 3a prereq)
3. Reduce к 1 umbrella `triage` crate + 1 extracted `curation` lib (instead of 4 new crates)

After those corrections, the plan is solid foundation для а scope-arch run. Estimated total implementation: 15-18 chunks ≈ Epoch 9. Specialist plan touch density: 4 heavy plans (arch + test + obs + а new-scope-introduction для existing plans) + 3 medium plans (security + design + layout-templates + a11y depending on how you count).

The single most important caveat: **the widget is currently 100% synthetic.** Any "redesign" work без first wiring real broadcast subscriptions is rearranging UI on data that doesn't represent the application state. Make that wire-up chunk #1 of Epoch 9, before anything else.
