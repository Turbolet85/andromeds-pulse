# Pulse v0.2.0 mid-stream consolidation audit

**Date:** 2026-05-19 (session 102)
**Branch:** main @ 3602f82
**Scope:** chunks #57-#69 (Epoch 9 Foundation v0.2.0); v0.1.0 chunks #1-#56 tagged scope-adjacent where surfaced
**Standard applied:** Consistency-first — every two-pattern-where-one-should-suffice flagged HIGH regardless of "it works"
**Investigation only:** no chunks registered, no code modified, no remediation authored

---

## Table of contents

- [Executive summary](#executive-summary)
- [Dimension 1 — Persistence mechanism consistency](#dimension-1--persistence-mechanism-consistency)
- [Dimension 2 — Capability claims vs implementation reality](#dimension-2--capability-claims-vs-implementation-reality)
- [Dimension 3 — Capability coverage gaps](#dimension-3--capability-coverage-gaps)
- [Dimension 4 — Architecture registry vs codebase reality](#dimension-4--architecture-registry-vs-codebase-reality)
- [Dimension 5 — Specialist plan compliance](#dimension-5--specialist-plan-compliance)
- [Dimension 6 — Documentation cross-reference consistency](#dimension-6--documentation-cross-reference-consistency)
- [Dimension 7 — Living artifacts freshness](#dimension-7--living-artifacts-freshness)
- [Dimension 8 — Improvements proposals lifecycle review](#dimension-8--improvements-proposals-lifecycle-review)
- [Dimension 9 — State.yaml integrity](#dimension-9--stateyaml-integrity)
- [Section 1 — Critical inconsistencies (HIGH)](#section-1--critical-inconsistencies-high)
- [Section 2 — Notable inconsistencies (MEDIUM)](#section-2--notable-inconsistencies-medium)
- [Section 3 — Cleanup opportunities (LOW)](#section-3--cleanup-opportunities-low)
- [Section 4 — Implementation proposals lifecycle](#section-4--implementation-proposals-lifecycle)
- [Section 5 — Remediation recommendations](#section-5--remediation-recommendations)
- [Section 6 — Suggested consolidation chunk grouping](#section-6--suggested-consolidation-chunk-grouping)
- [Appendix A — File:line evidence index](#appendix-a--fileline-evidence-index)
- [Appendix B — Capability ↔ chunk mapping table](#appendix-b--capability--chunk-mapping-table)
- [Appendix C — Architecture registry ↔ code diff table](#appendix-c--architecture-registry--code-diff-table)

---

## Executive summary

The audit covered all nine specified dimensions plus surfaced two cross-cutting patterns. **Net verdict: route §2 narrative "v0.2.0 Foundation Epoch 9 reaches 100%" is structurally misleading — 100% means "69 chunks landed in route §2," not "60 capabilities live."** Actual capability spec coverage per Dim 2 is: **9 LIVE (15%) / 20 PARTIAL with HIGH severity gaps (33%) / 27 deferred (45%) / 3 orphan (5%).**

**HIGH severity findings — 7 critical inconsistency clusters** require remediation before v0.2.0 ships:

1. **Persistence triple-mechanism**: confirmed dual + a hidden third. BaselineState flat-file (chunks #61/#64) vs corpus SQLite (chunk #69), plus pure-in-memory (ServiceRegistry chunk #67, RetryStormState chunk #66, RestartDetector last_seen chunk #63, SuppressionState chunk #63). Spec P-009 + P-013 explicitly demand corpus; P-027 implies persistence; chunk #66 docstring explicitly defers persistence to chunk #69 (broken promise).
2. **PII scrubber single-site coverage**: `crates/security/src/scrubber.rs::scrub_attribute` invoked at exactly one production site (`crates/buffer/src/drain.rs:600` for Drain DuckDB write). NOT applied at OTLP ingestion / log_records / span_events / BaselineState save / Drain corpus pipeline_metrics save. P-047 SHALL "all content before persistence" — violated at multiple paths.
3. **Capability spec PARTIAL gaps (20)**: numeric thresholds wrong (P-001 5s/30s vs 10s/60s), persistence inheritance broken (P-010/P-012 use FIXED constants instead of P-009/P-011 streaming baselines), surface-side capabilities have ZERO observable implementation (P-019 three-tier severity, P-024 widget "no numerical readouts" violated, P-051 delete entirely absent).
4. **Arch registry drift — chunk #69 Step 32 technically incomplete**: Session 101 Type 6 amendment closed TauRPC piece (`diagnostics.template_distribution`) but left DuckDB sibling (`log_templates` table) unacknowledged. `crates/buffer/src/schema.rs:5-9` explicitly self-flags pending registration. Also: corpus SQLite 6-table schema unregistered (chunk #68 amendment covered path only); `triage/baseline-corpus.bin` filesystem subpath unregistered.
5. **Capability-to-chunk table internal stale** (`docs/v0_2_0/pulse-v0_2_0-route.md:712`): row "P-019 to P-023, P-060 | #67 superseded by #72-#77" points to v1 chunk #67 (Severity classifier); v2 chunk #67 = Service Registry. Reader following table gets wrong chunk pointers.
6. **Chunk #69 internal scrubber inconsistency**: Drain template tokens scrubbed at DuckDB write (`drain.rs:586-606`) but bypass scrubber at corpus persist (`snapshot_state()` → bincode → `CorpusWriter::save_pipeline_metric`). Same logical data → two persist targets → different PII discipline.
7. **Specialist plan staleness**: security-plan.md §Threat Model + §Data Protection assert "no persistent disk database" — directly contradicted by chunks #61 (baseline-corpus.bin) + #68 (corpus.db). Plan never re-derived; `/andromeda-security re-run` documented as outstanding for 8+ wraps.

**MEDIUM severity** (Section 2): security plan staleness; 4 deferred PII vector tests unmaterialized; 7 documentation cross-references broken; forward-promise registry entries (`snapshot.list_recent`, `workspace.list`, `pulse://stream/plugin-events`, `ANDROMEDA_PULSE_CONFIG_PATH`); state.yaml.last_completed_chunk.commit_sha references non-existent git object (auto-reconciles next wrap); api-surface.md 7th consecutive deferral.

**LOW severity** (Section 3): tray Halo Pulse missing (scope-adjacent v0.1.0 design Self-Validation #3 fails); widget errorRate hardcoded 0; Tauri capability identifier prefix cosmetic mismatch; chunk #69 LogHub golden corpus directory not visible; METADATA bloat in api-surface.md.

**Improvements proposals (Section 4):** verified count 14 total (3 IMPLEMENTED + 2 PHASE 1 IMPLEMENTED + 9 PROPOSED). **IMPLEMENT NOW candidates**: P7 (Type 6 narrative-cascade visibility — 4+ unaddressed occurrences) + P12 (Type 6 CLAUDE.md derived-section cascade — 3+ occurrences). Together close cross-reference rows 9-11 (arch narrative "eight library crates" → 12). Four new proposals surfaced from current friction patterns (P15-P18 in Section 4).

**Suggested consolidation chunk groupings (Section 6):** 8 natural scope boundaries identified; no ordering imposed.

---

## Dimension 1 — Persistence mechanism consistency

### 1.1 Known dual-mechanism confirmed + expanded to triple

The user's stated concern — BaselineState flat-file vs corpus pipeline_metrics — is confirmed. **The pattern is worse than dual: it is triple.**

- **Mechanism A — flat-file BaselineState:** EwmaTracker, TDigestPair, RollingWindow, ActivityFloor; stored at `<data_dir>/triage/baseline-corpus.bin`; plaintext bincode; introduced chunks #61 + #64 (predates corpus crate)
- **Mechanism B — corpus SQLite pipeline_metrics:** DrainState (template tree, next_template_id, occurrences); AES-256-GCM cell-level encryption + OS keychain key; introduced chunk #69
- **Mechanism C — pure in-memory (no persistence):** ServiceRegistry (chunk #67), RetryStormState (chunk #66), RestartDetector last_seen (chunk #63), SuppressionState (chunk #63); lost on every process restart

### 1.2 Component-by-component findings

| Component | Chunk | Current mechanism | Target mechanism | Consistent? | Severity | Spec language |
|---|---|---|---|---|---|---|
| **EwmaTracker** | #61 | flat-file BaselineState | corpus pipeline_metrics OR baseline_state table (already in schema, unused) | NO | **HIGH** | P-009: "Baseline state SHALL be persisted to corpus across application restarts" (capability-spec line 168 — **explicit "to corpus"**) |
| **TDigestPair** | #61 | flat-file BaselineState | corpus | NO | **HIGH** | P-011 inherits P-009 persistence; spec line 188 silent on persist but shares BaselineState |
| **RollingWindow<u32>** | #61 | flat-file BaselineState | corpus | NO | **HIGH** | P-009 same as EWMA |
| **ActivityFloor** (288-bucket histogram + quiet-duration t-digest) | #64 | flat-file BaselineState | corpus | NO | **HIGH** | P-013: "Histogram state SHALL be persisted across application restarts" + "via corpus persistence" (capability-spec lines 208 + 214 — **stated twice**) |
| **ServiceRegistry** (per-service ServiceLifecycleState + first_seen/last_seen + manual_override) | #67 | DashMap pure in-memory; lost on restart | corpus `service_registry` table (table exists, never written) | NO | **HIGH** | P-027: "Restart Pulse; verify dot positions match prior session" (capability-spec line 382); schema reserves slot at `crates/corpus/src/schema.rs:44-53` |
| **DrainState** (template tree, next_template_id, occurrences) | #69 | corpus `pipeline_metrics(metric_name="drain_template_tree", layer="l1c")` | (same) | YES within-system | OK | P-007 silent on persistence; chosen corpus per Drain plan |
| **ExceptionFingerprint** (16-byte BLAKE3) | #66 | DuckDB span_events.fingerprint BLOB (ring buffer) | inherits buffer's ring | YES (no state to persist; hash is deterministic per input) | OK | P-017 silent |
| **RetryStormState** (`DashMap<[u8; 16], FingerprintState>`) | #66 | DashMap pure in-memory | corpus | NO | **HIGH** ([elevated from MEDIUM per Dim 1 agent] — broken promise) | storm.rs:38-39 docstring: "Persistence deferred к chunk #69 corpus scaffold." **Chunk #69 closed without wiring.** Effect: restart resets storm dedup; same storm cue can re-emit |
| **RestartDetector.last_seen** (`DashMap<String, i64>`) | #63 | DashMap pure in-memory | (open question) | NO but defensible | LOW | P-015 silent; restart-on-restart reset arguably acceptable since first-observation boundary handles cold start |
| **SuppressionState** | #63 | in-memory; derives from RestartDetector | n/a | YES (within-system follows RestartDetector) | OK | P-016 silent |
| **ConnectionState FSM** | #59 | in-memory; 1-2s poller derives from IngestState | n/a (pure derivation) | YES | OK | P-001-P-004 silent on persist; no state to persist |
| **Settings (config.toml)** | #38 | TOML file at `<data_dir>/config.toml` | n/a (user config not runtime state) | YES (justified separate mechanism) | OK | arch §Cross-cutting Patterns Config management |
| **Snapshot artifacts** | #44 | flat-file `<data_dir>/snapshots/` | n/a (user-visible export) | YES (justified) | OK | obs-plan §3 |
| **Tracing logs** | #7 | flat-file `<data_dir>/logs/agent-latest.jsonl` | n/a (log file IS the agent surface) | YES (justified) | OK | obs-plan §3 binding contract |

### 1.3 Cross-cutting concerns matrix

| Mechanism | Components stored | PII scrub | Encryption at rest | Observability | Backup story |
|---|---|---|---|---|---|
| Flat-file BaselineState (`baseline-corpus.bin`) | EwmaTracker, TDigestPair, RollingWindow, ActivityFloor + persisted_at | **NO** (only canary-test for column-tag absence; no PII pattern test) | **NO** (plaintext bincode) | Y (`triage.baseline.persist` + `pipeline.l1b.persist_count_total` + `pipeline.l1b.bootstrap_count_total`) | Manual file copy |
| Corpus SQLite (`<data_dir>/corpus/corpus.db`) | DrainState (pipeline_metrics rows); StorageApi inspect/path | **PARTIAL** — Drain tokens scrubbed for DuckDB write at `drain.rs:586-606`; corpus-persisted DrainState payload BYPASSES scrubber (via `snapshot_state()` → bincode) | **YES** AES-256-GCM cell-level + OS keychain | Limited (`drain.persistence.load.ok` + `drain.persistence.unavailable`; no per-persist counter) | Manual SQLite file copy + keychain dependency |
| TOML Settings | All Settings fields | n/a | NO (plaintext) | tracing on read/write | User text-edit |
| Snapshot files | Curated OTLP markdown + JSON | YES via curation primitives | NO | snapshot.* tracing | User export |
| tracing log file | Field-name-allowlisted obs events | YES via JsonFieldVisitor default-deny | NO | self-referential | tracing-appender daily rotation |
| In-memory DashMap (no persistence) | ServiceRegistry, RetryStormState, RestartDetector last_seen, SuppressionState | n/a (never persisted) | n/a | varies | none — restart wipes |

### 1.4 Internal inconsistency within chunk #69 itself

Drain template content writes follow **two paths from the same source data:**

- **Path 1** — DuckDB `log_templates` table write via `write_template_to_table()` at `crates/buffer/src/drain.rs:586-606`. **Scrubbed** via `scrub_attribute` before write; replaces match-hit tokens with `[REDACTED:{category}]` markers.
- **Path 2** — corpus `pipeline_metrics` persist via `DrainMiner::snapshot_state()` → `bincode::serialize(DrainState)` → `CorpusDrainPersistence::save()` → `CorpusWriter::save_pipeline_metric()`. **NOT scrubbed.** DrainState.templates[].tokens carry the original (masked-only-not-scrubbed) tokens from `assign()` path at `drain.rs:430` (`mask_body` performs regex masking of IPs/paths but does NOT call `scrub_attribute`).

**Net effect:** the same template content lands in two persistence targets with different PII discipline. Chunk #69 internally inconsistent.

### 1.5 PII scrubber coverage map (project-wide)

`crates/security/src/scrubber.rs::scrub_attribute` (7 P-047 categories: JWT / bearer / API key / secret KV / email / credit card / SSN) is invoked at exactly **one production site:**

- `crates/buffer/src/drain.rs:600` (write_template_to_table for in-memory DuckDB log_templates write — chunk #69)

**Production sites that bypass the scrubber:**

- OTLP ingestion via appender → DuckDB writes for spans / log_records / span_events / metrics_points / resources / instrumentation_scopes (`crates/buffer/src/appender.rs:332-356` for exception.message; same pattern for all OTLP attribute paths)
- BaselineState save (`crates/triage/src/baseline/corpus.rs:45-73` — service.name keys land verbatim in flat-file)
- Drain DrainState corpus payload save (Path 2 above — internal chunk #69 inconsistency)
- Settings round-trip (`crates/ui-bridge/src/health.rs:421-486` — bounded fields, low risk, low priority to add)
- All other CorpusWriter::save_pipeline_metric callers (today: zero outside CorpusDrainPersistence, but the contract docstring at `crates/corpus/src/contract.rs:157-160` only says "callers SHOULD pre-scrub" — does not enforce)

**Encryption coverage asymmetry:** corpus SQLite cells get AES-256-GCM; flat-file `baseline-corpus.bin` is plaintext on disk. P-049 spec language ("encrypt corpus SQLite database") technically holds at letter level — but at intent level, assuming all persistent state is in corpus.

### 1.6 P-051 user-surface gap

`storage_router.rs StorageApiImpl::inspect` reads only corpus tables via `CorpusReader::inspect()`. **The flat-file `baseline-corpus.bin` is invisible** through the Settings storage panel — user cannot inspect / export / delete that file via UI; must navigate to data directory manually.

P-051 SHALL: "Pulse SHALL provide users with the ability to inspect, export, and delete corpus content through Settings interface." Spec is explicit on three abilities; current implementation: inspect ✓ / export ✗ (chunk #85 deferred) / **delete ✗ (no procedure exists, despite explicit SHALL)**.

### 1.7 Beyond-the-known-case discovery

- `triage-experimental/` standalone workspace: own `[workspace]` root + gitignored + `publish = false` + Phase A Drain spike per `.andromeda/decisions/pre-d2-drain-spike.md`. **Intentional. Not a persistence-mechanism issue.** Explains 13-on-disk-vs-14-declared discrepancy.
- All other inert-artifact mechanisms (snapshots/, logs/, plugins/, PID file, Settings TOML) are **justified separate mechanisms** because the artifacts are user-visible / inert / static.

---

## Dimension 2 — Capability claims vs implementation reality

See [Appendix B](#appendix-b--capability--chunk-mapping-table) for complete capability ↔ chunk mapping table. This section summarizes the **PARTIAL findings with HIGH severity gaps.**

### 2.1 Capability coverage breakdown

- **LIVE (full spec satisfied):** P-002, P-004, P-015, P-016, P-017, P-018, P-021, P-040, P-050 — **9 of 60 (15%)**
- **PARTIAL (HIGH severity gaps):** P-001, P-003, P-006, P-010, P-011, P-012, P-013, P-014, P-019, P-024, P-025, P-027, P-038, P-039, P-041, P-047, P-048, P-049, P-051, P-058 — **20 of 60 (33%)**
- **DEFERRED (later chunks):** P-020, P-022, P-023, P-026, P-028, P-029, P-030, P-031-P-036, P-037, P-042-P-046, P-052-P-056, P-059, P-060 — **27 of 60 (45%)**
- **ORPHAN (no chunk attribution):** P-005, P-007 (mis-attributed), P-008 — **3 of 60 (5%) with HIGH severity each**

### 2.2 Detailed PARTIAL findings (HIGH severity)

#### P-001 (Receiver Lifecycle State) — wrong thresholds

- **Spec:** "Idle (10–60s quiet), Stalled (>60s quiet)"
- **Code** at `crates/ingest/src/connection.rs:39-44`: `IDLE_THRESHOLD_NANOS = 5_000_000_000` (5s), `STALLED_THRESHOLD_NANOS = 30_000_000_000` (30s)
- **Gap:** Numeric-boundary deviation. Users observing the FSM see Idle/Stalled fire on different boundaries than the spec promises.
- **Remediation:** Implementation should match spec (10s/60s), OR amend spec to "configurable bounds" if 45s heartbeat-CI-alarm coupling justifies.

#### P-003 (Receiver Failure Surface) — only one of three failure paths wired

- **Spec:** "port bind errors at startup, runtime panics in receiver tasks, and explicit shutdown signals ... failures surface in the UI within 2 seconds"
- **Code:** `connection.rs:264-282` `derive_reason` returns `BindFailed` or `StaleHeartbeat`; `ReceiverPanicked` defined but unreachable (no panic hook routes to FSM). 1s tick cadence may exceed 2s budget under scheduling pressure.
- **Gap:** spec lists three failure paths; only one wires. No shutdown-signal handler.
- **Remediation:** `std::panic::set_hook` already routes to `tracing::error!`; extend to update shared atomic that `ReceiverBindStatus::any_receiver_failed` reads.

#### P-006 (Exception Event Capture) — exception.message stored raw

- **Spec:** "extracting `exception.type`, `exception.message` (**after PII scrubbing per P-047**), and `exception.stacktrace`"
- **Code** at `crates/buffer/src/appender.rs:332-356`: writes raw `exception.message` directly to span_events.exception_message BLOB without `security::scrubber::scrub_attribute()` invocation
- **Gap:** Direct cross-capability contract violation; P-047 SHALL applies "before persistence."
- **Remediation:** Wire scrubber between extract and push; same for `exception.stacktrace`.

#### P-010 (Error Rate Spike Detection) — fixed baseline constants

- **Spec:** "exceeds its baseline by a factor of 3.0"
- **Code** at `crates/triage/src/cue/evaluate.rs:30-64` + `thresholds.rs:19`: `base_error_rate = 0.01` HARDCODED 1%, NOT per-service streaming baseline from P-009. `persistence_seconds = snapshot.samples` (EWMA sample count, NOT wall-clock seconds).
- **Gap:** A service with natural 0.5% error rate triggers cue at 3% absolute (6× of natural baseline); a service with natural 2% baseline triggers at 3% (1.5× of natural baseline) — neither matches "3.0× baseline" semantic.
- **Remediation:** Wire `evaluate.rs` to read per-service EWMA value (P-009 output) instead of fixed constant.

#### P-011 (Per-Operation Latency Baseline) — operation identity hashed

- **Spec:** "per operation identified by combination of service.name and operation identifier"
- **Code** at `crates/triage/src/baseline/mod.rs:368-375`: `operation_key()` produces `service/<hashed>` — original operation_name irrecoverable.
- **Gap:** `OperationMetricSnapshot.operation_key` carries hashed form. Downstream surfaces cannot show "p99 of GET /endpoint regressed" because operation name is hashed away.
- **Remediation:** Add `operation_name` field on OperationBaseline + OperationMetricSnapshot; cue's scope_id should be human-readable.

#### P-012 (Latency Regression Detection) — wrong percentile + fixed baseline

- **Spec:** "p99 latency for an operation exceeds baseline p99 by factor of 2.5"
- **Code** at `crates/triage/src/cue/thresholds.rs:36`: `DEFAULT_LATENCY_PERCENTILE = 0.95` (NOT 0.99); `evaluate.rs:74-87` uses fixed `base_latency_ms = 100ms` not per-operation baseline.
- **Gap:** spec p99 + baseline-relative; code p95 + constant.
- **Remediation:** Change percentile constant; wire to t-digest baseline.

#### P-013 (Service Activity Floor Learning) — persistence works but staleness gate defeats promise

- **Spec:** "Histogram state SHALL be persisted across application restarts" + "via corpus persistence"
- **Code** at `crates/triage/src/baseline/corpus.rs`: persists via bincode flat-file (NOT SQLite corpus from chunk #68). Also: `STATE_AGE_THRESHOLD_NANOS = 1h` staleness gate discards 2-hour-old persisted state.
- **Gap:** (a) Wrong mechanism (flat-file vs corpus per Dim 1); (b) 1-hour staleness eviction defeats "eliminates per-restart cold-start blindness" if user closes Pulse overnight.
- **Remediation:** (a) migrate to corpus per Dim 1 finding; (b) drop staleness gate or extend.

#### P-014 (Service Went Silent Detection) — no 30s floor

- **Spec:** "with minimum threshold of 30 seconds" + bootstrap 60min suppression
- **Code:** Bootstrap 60min present. **No 30s floor.** If service observed only twice with 1-second gap, p95 = 1s, a 2-second quiet triggers a cue.
- **Remediation:** Add `max(p95_seconds, 30)` floor in `evaluate_service_went_silent`.

#### P-019 (Three-Tier Severity Model) — contract types only, ZERO observable surfaces

- **Spec:** "Autonomous (prominent halo shift + counter increment), Suggested (counter increment + minimal halo), Curious (Findings dropdown collapsed section)"
- **Code:** Contract types + classifier present (`contract.rs:111-116`, `classify.rs:15-30`). **No FindingsCounter, no FindingsDropdown components exist.** HaloCanvas consumes `(errorRate, throughputHz)` not severity tier.
- **Gap:** Observable signal contract entirely absent.
- **Remediation:** Phase 8 chunks #78/#79/#81 (none landed); contract types alone do NOT satisfy P-019.

#### P-024 (Widget Ambient Surface) — direct spec violation

- **Spec:** "Widget never displays incident card content, **never displays counters or rates**, never displays toasts or popups"
- **Code:** `pulse-app/ui/src/widget/FooterBand.tsx` exists and displays numerical metrics (throughputHz / errorRate / retentionUsedSeconds).
- **Remediation:** Chunk #80 (unland; addresses this directly) — land it before claiming P-024.

#### P-027 (Service Constellation Auto-Discovery) — corpus restore is no-op stub

- **Spec:** "Restart Pulse; verify dot positions match prior session" + "corpus history lookup on Archived → Active"
- **Code** at `crates/triage/src/lifecycle/state_machine.rs:38`: documented "CorpusRestore = Archived→Active via chunk #69 corpus history lookup (**no-op stub until #69 lands**)." Chunk #69 landed as Drain — did NOT wire lifecycle CorpusRestore. `TransitionTrigger::CorpusRestore` enum variant unreachable.
- **Remediation:** Couple lifecycle registry to corpus `service_registry` table at boot.

#### P-038 / P-039 (MCP tool surface mismatch)

- **Spec:** `query_incident_list` / `retrieve_report(id)` / `retrieve_telemetry_slice(id)` / `mark_incident_resolved(id)`
- **Code** at `crates/mcp-server/src/tools.rs:26`: `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` — wrong tool surface (telemetry query vs incident retrieval)
- **Remediation:** Add four spec-mandated tools when chunks #84 + #79 land (incidents records + Reports).

#### P-041 (Persistent Incident Corpus) — schema scaffolded, never written

- **Spec:** "SHALL persistently store every detected incident ... 30-day default retention"
- **Code:** SQLite scaffold + 6 tables LIVE; **no incident records stored anywhere** (no chunk #70 wiring); `pipeline_metrics` only Drain template path; **no 30-day retention sweep**.
- **Remediation:** Chunk #70 (incident lifecycle) + retention task. P-041 attribution to #69 is over-claimed.

#### P-047 (PII Scrubbing at Ingestion) — coverage gap

- **Spec:** "SHALL apply pattern-based PII scrubbing to **all content before persistence to corpus or inclusion in Reports**"
- **Code:** scrubber consumed at exactly ONE production site (Drain DuckDB write per `drain_persistence.rs`). NOT at OTLP ingestion (`appender.rs`), span_events (raw exception.message/stacktrace), log_records.body, cue payloads at broadcast emission. See [Dim 1 §1.5 PII scrubber coverage map](#15-pii-scrubber-coverage-map-project-wide).
- **Remediation:** Wire scrubber at appender per-attribute write; OR clarify spec scope to "before persistence to SQLite corpus" only.

#### P-049 (Encryption at Rest) — no passphrase fallback

- **Spec:** "OS keychain (when available) **or user-provided passphrase**"
- **Code** at `crates/corpus/src/contract.rs:60-71`: if keychain unavailable, returns `Error::KeyringUnavailable` and corpus init fails entirely. No passphrase prompt or fallback path.
- **Remediation:** Implement passphrase fallback, OR amend spec to "OS keychain only."

#### P-051 (Transparent Storage) — delete entirely absent

- **Spec:** "ability to **inspect, export, and delete** corpus content through Settings interface"
- **Code:** `storage.inspect()` ✓; `storage.export_for_training` (chunk #85, deferred); **delete: NOT IMPLEMENTED** — no procedure exists.
- **Remediation:** Add `storage.delete_all()` TauRPC procedure with confirmation dialog flow.

#### P-058 (Pipeline Self-Observability) — one of ~30 metrics; no Diagnostics view

- **Spec:** "approximately 30 other operational metrics" in Diagnostics view + 30-day history
- **Code:** Only `diagnostics.template_distribution()` TauRPC exposed; no Diagnostics panel UI; no history queries (chunk #87 deferred); no 30-day metric retention.
- **Remediation:** Phase 7+ chunks.

### 2.3 Orphan capabilities (no chunk attribution — HIGH severity each)

- **P-005 (Span Status Error Detection):** "SHALL identify ALL `Span.Status.Code = ERROR`; detection latency <500ms p99." No chunk attributes itself. Only implicit via baseline EWMA at `baseline/mod.rs:205` — but aggregation isn't "candidate emission" to severity layer.
- **P-007 (High-Severity Log Capture):** spec "SeverityNumber ≥ 17 ... candidate hard signals." Route §2 attributes to chunk #69, but chunk #69 implements Drain template mining (clustering of log MESSAGES), NOT SeverityNumber-based hard-signal detection. **Mis-attribution.**
- **P-008 (Root-Span Error Scope):** "distinguish error spans where the root span carries ERROR ... root-span errors weight more heavily." No chunk; no `parent_span_id`-aware detector exists.

### 2.4 Over-claimed chunks summary

| Chunk | Capabilities claimed | Capabilities actually delivered fully | Delta (over-claimed) |
|---|---|---|---|
| #59 | P-001, P-002, P-003, P-004 | P-002, P-004 | P-001 (wrong thresholds) + P-003 (no panic path) |
| #61 | P-009, P-011 | P-009 | P-011 (operation identity hashed) |
| #62 | P-010, P-012, P-021 | P-021 | P-010 + P-012 (fixed baselines, wrong percentile) |
| #64 | P-013, P-014 | (partial both) | P-013 + P-014 |
| #65 | P-006 | (partial) | P-006 (no PII scrubbing) |
| #66 | P-017, P-018 | P-017, P-018 | — (LIVE) |
| #67 | P-027 | (stub) | P-027 (CorpusRestore unreachable) |
| #68 | P-041, P-047, P-048, P-049, P-050, P-051 | P-050 only | P-041 / P-047 / P-048 / P-049 / P-051 |
| #69 | P-007 | (mis-attributed) | P-007 |

### 2.5 Spec language ambiguity (cannot verify objectively)

1. P-005 "candidate emission" surface unspecified
2. P-007 "candidate hard signals" same surface ambiguity
3. P-014 "minimum threshold of 30 seconds" — floor on p95 result or hard quiet floor?
4. P-019 "halo response intensity" — no quantitative test
5. P-020 "model retains full discretion" — non-deterministic
6. P-026 "never scale modulation" — needs WGSL static analysis
7. P-027 "Up to 20 services rendered" — different from cardinality cap
8. P-048 "in corpus" — DuckDB ring buffer scope ambiguous
9. P-058 "approximately 30 metrics" — approximation language

---

## Dimension 3 — Capability coverage gaps

Folded into Dimension 2 (Appendix B). Summary tables:

### 3.1 Capabilities by status (high-level)

See [Appendix B](#appendix-b--capability--chunk-mapping-table) for full P-001..P-060 table.

### 3.2 Chunks by coverage delta

See Dim 2 §2.4 Over-claimed chunks summary.

### 3.3 Orphan capabilities (no chunk attribution)

- **HIGH severity:** P-005, P-007 (mis-attributed), P-008
- **DEFERRED (no severity issue — chunks not yet shipped):** P-020, P-022, P-023, P-028, P-029, P-031-P-036, P-042-P-046, P-052-P-056, P-059, P-060

### 3.4 Implicit capabilities (code implements, spec mapping missing)

- BaselineState observe_span tracks STATUS_CODE_ERROR — partial P-005 coverage but EWMA aggregation isn't candidate emission per spec surface requirement (HIGH)
- `pipeline_metrics` table reserved for L1a/L1b/L2/L3 per schema docstring; only Drain L1c uses it; the other 4 layers' snapshot writes never materialized
- `BaselineState` cardinality cap `ACTIVITY_FLOOR_SERVICE_CAP = 1024` not in spec (defensive, reasonable)
- `RestartObserverAdapter` + `CompositeSpanObserver` are internal substrate (not capability surfaces)

---

## Dimension 4 — Architecture registry vs codebase reality

See [Appendix C](#appendix-c--architecture-registry--code-diff-table) for full diff table. This section summarizes drift directions.

### 4.1 HIGH severity — code-not-acknowledged (Type 6 amendment overdue)

#### 4.1.1 DuckDB `log_templates` table (chunk #69 Step 32 incomplete)

- **Code:** `crates/buffer/src/schema.rs:18,125` — `RESERVED_TABLES = ["spans", ..., "log_templates"]` (8 entries)
- **arch.md §Occupied Resources DuckDB reserved tables:** 7 entries — `log_templates` absent
- **Schema docstring** at `crates/buffer/src/schema.rs:5-9` explicitly self-flags: "Reserved table list per arch §Occupied Resources §DuckDB reserved tables. Chunk #20 baseline locked 7 tables; chunk #69 Phase B adds `log_templates` (8th table) for Drain L1c log-template-mining surface. **Arch §Occupied Resources update via /andromeda-evolve --allow-arch-registry lands at Session 6 wrap per chunk #69 Phase B plan.md Step 32.**"
- **Reality:** Session 101 Type 6 amendment closed only the TauRPC piece (`diagnostics.template_distribution`); the DuckDB table sibling was left unacknowledged. **Chunk #69 Step 32 is technically incomplete at the registry level.**

#### 4.1.2 Corpus SQLite schema tables (chunk #68 amendment scope gap)

- **Code:** `crates/corpus/src/schema.rs:26-33` — TABLE_NAMES list: `baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive`
- **arch.md §Occupied Resources:** corpus tables NOT listed. §Occupied Resources DuckDB section enumerates the in-memory `pulse_buffer` tables. Corpus is a SEPARATE rusqlite database (introduced chunk #68).
- **Existing Type 6 amendment (2026-05-18):** added the FILE PATH (`corpus/corpus.db`) but **NOT the table schema list**.
- **Remediation:** New sub-section under §Occupied Resources for "Corpus SQLite database / schema names" mirroring DuckDB sub-section. Corpus is FIRST persistent on-disk DB — deserves explicit registry presence.

#### 4.1.3 Filesystem subpath `triage/baseline-corpus.bin` (chunk #61 substrate)

- **Code:** `crates/triage/src/baseline/corpus.rs:42` — `Ok(canonical_triage.join("baseline-corpus.bin"))`; `pulse-app/src/main.rs:275` — `data_dir.join("triage").join("baseline-corpus.bin")`
- **arch.md §Occupied Resources Filesystem locations:** subpath absent
- **Remediation:** Type 6 amendment. (Or — if Dim 1 persistence-consolidation chunk migrates BaselineState to corpus — file ceases to exist; remove instead.)

### 4.2 MEDIUM severity — registry-stale forward-promises

| Item | In arch.md §Occupied Resources | In code | Notes |
|---|---|---|---|
| `snapshot.list_recent` TauRPC | YES | NO (`snapshot_runtime.rs:130` has only `snapshot.generate`) | Forward-promise; xtask EXPECTED_PROCEDURES correctly omits as "future-deferred" (lines 681-683) — arch-vs-xtask already desynced |
| `snapshot.copy_to_clipboard` TauRPC | YES | NO | Same as above |
| `workspace.list` TauRPC | YES | NO (`workspace_ipc.rs:47` has only `workspace.detect`) | Same |
| `pulse://stream/plugin-events` broadcast | YES | NO (only doc refs in `plugins-examples/`; no runtime emitter) | [scope-adjacent — v0.1.0] Epoch 7 chunks #45-#49 |
| `ANDROMEDA_PULSE_CONFIG_PATH` env var | YES | NO (Settings uses fixed `<data_dir>/config.toml` via `CONFIG_FILE` const; no env override) | Forward-promise |

### 4.3 LOW severity — cosmetic + harness-only

- Tauri capability identifiers: arch says `pulse:default` / `pulse:tray`; JSON files use bare `"default"` / `"tray"`. Cosmetic mismatch; no functional drift.
- Harness-only env vars `ANDROMEDA_PULSE_PIDFILE` / `LOGFILE` / `DATA_DIR_KEEP`: used only in `scripts/agent-run.{sh,ps1}`. Optional to register.
- `run/andromeda-pulse.pid` filesystem subpath: written by `pulse-app/src/main.rs:188`, not registered. [scope-adjacent — v0.1.0]

### 4.4 §Architecture Registry Updates audit — clean

All 11 Type 6 amendments verified: markers exist on disk; items still in code; redundancy with §Occupied Resources main list is intentional audit-trail per `spec-amendment-protocol.md`. **No drift in this sub-section.**

### 4.5 CLAUDE.md pointer table — clean

All 25 rows resolve to existing files. 12 service docs match 12 lib crates. No broken pointers.

### 4.6 `triage-experimental/` resolution

`crates/triage-experimental/Cargo.toml` declares own `[workspace]` root + `publish = false` + gitignored at parent level. Phase A Drain spike per `.andromeda/decisions/pre-d2-drain-spike.md`. **Intentional. Not a registry concern.** Explains 13-on-disk-vs-14-declared-in-workspace.

---

## Dimension 5 — Specialist plan compliance

### 5.1 Security plan — HIGH/MEDIUM drifts

**Cross-cutting status:**
- C1 Loopback-only OTLP binding: ✓
- C2 DuckDB prepared statements: ✓
- C3 Capability JSON pairing: ✓ (4-place/5-place binding pattern preserved)
- C4 PII scrubber on persistence: ⚠ **GAP** — see [Dim 1 §1.5](#15-pii-scrubber-coverage-map-project-wide)
- C5 Tracing AllowList default-deny: ✓
- C6 Fingerprint hash discipline: ✓ (negative test enforces exception.message exclusion)

**Drift findings:**

1. **HIGH — Plaintext baseline-corpus.bin contains user-controlled service.name keys.**
   - security-plan.md §Anti-Pattern Logging row 1: "NEVER log raw OTLP attribute values, span/log/metric content payloads ... they may contain incidentally captured secrets"
   - security-plan.md §Threat Model: "OTLP attributes are user-controlled content"
   - `crates/triage/src/baseline/corpus.rs:45-73` `persist_state()` calls `bincode::serialize(state)` → `fs::write(&tmp_path, &bytes)`; BaselineState contains `DashMap<String, ServiceBaseline>` keyed by raw service.name. No `scrub_attribute` call. Plaintext on disk.
   - **Remediation:** (a) wire BaselineState through `corpus::CorpusWriter` (Dim 1 consolidation chunk), OR (b) call scrub_attribute on each service_name key before bincode-serializing, replacing match-hit keys with `[REDACTED:{category}]` markers per Drain precedent.

2. **MEDIUM — Security plan §Threat Model + §Data Protection assert "no persistent disk database" — contradicted by chunks #61 + #68.**
   - security-plan.md line 22: "no persistent user data store (in-memory DuckDB ring buffer with 5–10 min retention)"
   - security-plan.md line 151: "DuckDB ring buffer: in-memory `:memory:` connection ... no persistent disk database"
   - **Reality:** chunk #68 corpus.db + chunk #61 baseline-corpus.bin are both persistent.
   - **Remediation:** `/andromeda-security` re-run extending §Data Protection §At rest with rows for corpus + baseline-corpus.bin; §Secret Management with corpus key custody flow.

3. **MEDIUM — Deferred PII vector test gaps + capability widening static analysis.**
   - testing.md §Pending coverage triggers: 4 PII vector tests (AppError sanitization / Plugin path basename / MCP response body redaction / Path env var canonicalization) + capability widening static analysis gap.
   - All deferred via amendments at 2026-05-08; never materialized.
   - **Remediation:** `/andromeda-tests` re-run.

### 5.2 Test plan — overall clean

- Coverage gates: ✓ (1128/1128 passing per session 100 baseline)
- E2E P1-P7 critical paths: ✓ all 7 present
- Property-based tests: ✓ (8 proptest sites including all 4 triage baseline trackers)
- Chunk-specific E2E: ✓ (storm/lifecycle/drain/security canaries)

**Drift findings:**

1. **MEDIUM — PII vector test gaps inherited** (same as Security §3 above)
2. **LOW — Chunk #69 LogHub golden corpus not visible:** chunk plan referenced LogHub golden corpus; no `tests/golden/drain/` or `tests/fixtures/drain/` directory visible. Algorithm correctness verified via unit tests at `crates/buffer/src/drain.rs:40+`. Documentation-vs-impl mismatch only.

### 5.3 Obs plan — strong alignment

All chunks #57-#69 commit-to-emit metrics verified in `pulse-app/src/observability.rs` AllowList:
- chunk #57 widget frontend bridge
- chunks #58-#60 substrate
- chunk #61 baseline (`triage.baseline.{tick,persist,persist.error,service_id_missing}` + `metric.baseline.ewma_short_window_size` + `pipeline.l1b.{persist,bootstrap}_count_total`)
- chunk #62 cue (`triage.cue.{tick,evaluate,emit}` + `metric.cue.emit_count_total`)
- chunk #63 pattern (`triage.pattern.{tick,restart_detect,restart_emit}` + bypass metric)
- chunk #64 activity floor (`triage.baseline.{service_went_silent.evaluate,service_cap_exceeded}` + `metric.triage.activity_floor.bootstrap_state`)
- chunk #66 storm (`triage.pattern.storm.{detected,emit,tick}` + 3 metric counters)
- chunk #67 lifecycle (`triage.lifecycle.{tick,transition,corpus_restore}` + `services.list_with_states.request` + `pipeline.l1b.tracked_services_total` + `metric.triage.lifecycle.state_distribution`)
- chunk #69 Drain (`drain` + `drain.persistence.{load.ok,unavailable}` + `diagnostics.template_distribution.request` + 2 `metric.pipeline.l1c.*`)

**Net: no drift findings beyond cross-cutting Security §2 about `/andromeda-security re-run`.**

### 5.4 Design plan — drifts

1. **LOW + [scope-adjacent — v0.1.0]** — Tray icon Halo State Pulse missing.
   - design-system.md §Brand Identity: "On the tray icon, a single unified halo pulses and shifts hue around the aggregated service-count badge."
   - Design-tokens.md Self-Validation #3: "Signature test — Halo present in 3 places (full dashboard / compact widget / tray icon)?"
   - layout-templates.md §Surface: desktop-native §Component — Tray icon §Halo State Pulse: Option A WebGPU transparent click-through window OR Option B SVG-filter fallback.
   - `pulse-app/src/tray.rs:5,110-161` ships only monochrome SVG glyph. No secondary halo layer.
   - **v0.1.0 chunk #36 substrate landed the static glyph; data-driven Halo Pulse layer was never built.**

2. **LOW — Widget errorRate hardcoded zero.**
   - design-system.md §Brand Identity: "shifting hue via LCH interpolation from Earth Blue (#4A90E2) to Alert Burgundy (#C7556A) based on error rate"
   - `pulse-app/ui/src/hooks/use-widget-metrics.ts:88`: returns `errorRate: 0` always
   - **Chunk #57 delivered widget binding but deferred error-rate plumbing to chunk #81 ("Halo data path refactor"). Currently widget Halo encodes only throughputHz; errorRate dimension is broken.**

### 5.5 A11y plan — no drift in chunks #57-#69 scope

Semantic HTML, ARIA, keyboard nav, focus management, reduced-motion, axe/Lighthouse/pa11y CI gates all preserved. Live regions wired (`FooterBand.tsx` uses `role="status" aria-live="polite"`).

### 5.6 Layout templates — Tray Halo missing (same as Design D1)

Compact widget / Full dashboard / Tray menu structure: ✓. **Tray secondary halo composited layer mandated by layout-templates.md (Option A WebGPU or Option B SVG-filter): NOT implemented.**

### 5.7 Cross-specialist patterns

- **Persistent storage tier drift** (Security + Tests + Obs intersect): chunks #61 and #68 introduced persistent disk storage; security-plan still asserts "no persistent disk database"; testing-plan lacks persistent-storage E2E test triggers; obs-plan AllowList lacks reason-field policy preventing path strings in persist.error events. `/andromeda-security re-run` is the canonical fix.
- **Tray icon** (Design + Layouts intersect): static SVG glyph satisfies Layouts' menu structure but not Design's Halo Self-Validation #3.
- **Widget errorRate=0** (Design + Test intersect): Design plan's "halo hue shifts per error rate" broken on widget; no test enforces the contract.

---

## Dimension 6 — Documentation cross-reference consistency

### 6.1 Cross-reference drift table

| # | Source | Reference text | Target resolution | Severity |
|---|--------|---------------|-------------------|----------|
| 1 | `docs/v0_2_0/pulse-v0_2_0-route.md:712` | "P-019 to P-023, P-060 \| #67 superseded by #72-#77" (capability-to-chunk mapping) | v2 chunk #67 = "Drain Rust implementation" (Phase 3, capability P-007); has NO relationship to severity model. Row is stale from v1 (where v1 #67 was "Severity classifier + incident state store" — per migration table line 754) | **HIGH** |
| 2 | `.andromeda/architecture.md:167` | "per obs-plan §11 Frontend bridge" | obs-plan §11 = "Obs Anti-Patterns"; "Frontend bridge:" subsection is at §3 (Logging stack, line 195) | MEDIUM |
| 3 | `.andromeda/route.md:329` | "pulse-v0_2_0-route.md §Phase 4 line 276" (chunk #67 marker) | Line 276 is inside chunk #67 Drain Phase A; chunk #68 (Service registry) header is at line 312 | MEDIUM |
| 4 | `docs/v0_2_0/pulse-distillation-architecture.md:5` | "`widget-state-validation-mini-route.md` (delivery plan)" | File renamed to `pulse-v0_2_0-route.md` per changelog v2 | MEDIUM |
| 5 | `docs/v0_2_0/pulse-capability-spec.md:5` | Same reference as #4 | Same | MEDIUM |
| 6 | `docs/v0_2_0/pulse-capability-spec.md:789` | "existing widget-state-validation-mini-route can be remapped" | Action already done (capability-to-chunk mapping in v0_2_0-route §line 705); sentence stale | MEDIUM |
| 7 | `docs/v0_2_0/pulse-capability-spec.md:5` | "`widget-state-validation-report-2026-05-14.md` (validation context)" | File exists at `.andromeda/scope-validation/widget-state-validation-report-2026-05-14.md`; reference unprefixed | LOW |
| 8 | `docs/v0_2_0/pulse-distillation-architecture.md:980-995` | "TODO: capability spec formalization (NEW in v3)" — 8 architectural concepts to formalize | All 8 ARE formalized as P-052..P-060 in capability spec v2 | MEDIUM |
| 9 | `.andromeda/architecture.md:4` (§Design Philosophy) | "eight library crates ... ten workspace members total" | Actual 12 library crates + 14 workspace members per §Occupied Resources line 178 | MEDIUM |
| 10 | `.andromeda/architecture.md:220` (§Infrastructure Patterns) | "one Tauri process hosting all eight library crates" | Same | MEDIUM |
| 11 | `.andromeda/architecture.md:303` (§Project Intent) | "modular monolith (eight Rust crates wired into one Tauri binary)" | Same | MEDIUM |

### 6.2 Resolved (sample spot-checks)

- CLAUDE.md pointer table: all 25 rows resolve ✓
- route.md §3 Decisions Log §Phase numbers chunks #58-#69: 11 of 12 resolve (only #67 broken per row 3 above)
- dist-arch v3 §-references: all resolve ✓
- Capability spec internal `per P-XXX` cross-references (154 occurrences): spot-checked P-057, P-058, P-022, P-047 — all resolve

### 6.3 pulse-v0_2_0-route.md ↔ route.md numbering map

**Numbering offset for chunks #67-#69** (route.md vs v0_2_0-route):

| route.md §2 # | route.md text | v0_2_0-route § | v0_2_0-route line | Match? |
|---|---|---|---|---|
| #57-#66 | (10 chunks) | §57-§66 | matching lines | ✓ exact for all 10 |
| **#67** | "Service registry + lifecycle state machine" | **§68** | line 312 | ⚠ DIVERGENT |
| **#68** | "Corpus SQLite scaffold + ... PII scrubber" | **§69** | line 329 | ⚠ DIVERGENT |
| **#69** | "Drain Rust implementation + template profiling" | **§67** | line 257 | ⚠ DIVERGENT |

**Subject-content alignment is CLEAN** — route.md §2 short text faithfully summarizes corresponding v0_2_0-route §-content. Divergence is purely numeric mapping (chunk #67 Service Registry registered first because v0_2_0-route §67 Drain was Pre-D2 blocked). Capability annotations + distillation layer references all resolve.

Single drift point from this map: row 3 in §6.1 (route.md:329's broken line number).

---

## Dimension 7 — Living artifacts freshness

### 7.1 dependency-tree.md — FRESH

- **Last reconciled:** 2026-05-19T23:10:00Z
- **File length:** 458 lines total; ~444-line LIVING block (cargo tree output)
- **Session 100 baseline preserved; +1 line delta from chunk #69 Session 7+ adding `security` direct dep to `crates/buffer/Cargo.toml`**
- **Status:** ✓ substantively current

### 7.2 api-surface.md — FORMALLY FRESH, ACTUALLY 7-WRAP DEFERRED

- **Last reconciled timestamp:** 2026-05-19T23:10:00Z (refreshed; but with explicit "deferred" suffix)
- **File length:** 7794 lines total
- **LIVING block:** ~6913 lines (session 89 baseline); growth tracked only via METADATA narrative additions
- **state.yaml flag:** `api_surface_deferred: true`
- **Deferral count:** 7 consecutive sessions (per session 101 handoff: "7th consecutive deferral per multi-crate tooling time budget")
- **METADATA bloat:** session-by-session narrative since session 70 has accumulated to extreme size; readers must parse hundreds of session entries to understand current state

**MEDIUM severity finding:** the "Last reconciled" timestamp misleads readers. A reader trusting the timestamp would think the LIVING block reflects current code. The actual LIVING block is multiple sessions stale; only METADATA narrative tracks deltas.

**Remediation:** next /implement-followed wrap is the natural re-baseline checkpoint per integrity-protocol.md. After chunk #70+ lands, refresh per-crate `cargo +nightly public-api --simplified` iteration.

### 7.3 METADATA section bloat

Both files (api-surface.md and dependency-tree.md) have METADATA sections that have grown to enormous size — each containing dozens of session-by-session change descriptions. **LOW severity but worth noting** — the METADATA section is becoming a session log rather than a focused "last reconciled when, how" snapshot. Session 70+ details should probably be summarized or pruned periodically.

---

## Dimension 8 — Improvements proposals lifecycle review

### 8.1 Verified count: 14 total

session-handoff says "5 IMPLEMENTED + 9 PROPOSED" = 14. Reconciled:
- Pure IMPLEMENTED: 3 (P4, P5, P6)
- PHASE 1 IMPLEMENTED (Phase 2 still open): 2 (P8, P9)
- PROPOSED: 9 (P1, P2, P3, P7, P10, P11, P12, P13, P14)

### 8.2 Master table

| # | Title | Status | Date | Dogfood occurrences | Recommendation |
|---|---|---|---|---|---|
| 1 | `--allow-arch-decision` flag | PROPOSED | 2026-05-16 (s66) | 0 (explicit author defer to feel friction first) | KEEP PENDING — awaits first manual structural arch edit (likely chunk #74 LLM) |
| 2 | setup-project generates andromeda-after-mvp-playbook.md | PROPOSED | 2026-05-16 (s66) | 1 | KEEP PENDING — implement when 2nd Andromeda project reaches post-MVP |
| 3 | setup-project includes playbook + improvements in pointer table | PROPOSED | 2026-05-16 (s66) | 1 | KEEP PENDING — bundle with P2 |
| 4 | Extend `--allow-route-append` for terminal-position new epoch | **IMPLEMENTED** | 2026-05-16 | 1 dogfood + chunks #58-#69 use it | DONE |
| 5 | Type 7 evolve markers pre-populate `expected_propagation: [CLAUDE.md]` | **IMPLEMENTED** | 2026-05-17 | 2 obs + 1 live test | DONE |
| 6 | Form 1 chunk-append auto-updates `§1 Total chunks` | **IMPLEMENTED** | 2026-05-17 | 3 occurrences | DONE |
| 7 | **Type 6 narrative-cascade visibility (arch.md structural section count)** | PROPOSED | 2026-05-16 (s70) | **4+ unaddressed**: chunks #43, #58, #60, #68 → arch.md still says "eight library crates" | **IMPLEMENT NOW** — highest evidence; resolves Dim 6 rows 9-11 |
| 8 | Arch Registry Updates section compaction | PHASE 1 IMPLEMENTED | 2026-05-17 (s82) | 7 entries refactored; Phase 2 deferred | KEEP PENDING — threshold not reached |
| 9 | route.md decisions log compaction | PHASE 1 IMPLEMENTED | 2026-05-17 (s82) | 8 entries refactored | KEEP PENDING — Phase 2 explicitly post-v1.0 ship gate |
| 10 | setup-project --delta detects non-delta-scoped uncommitted work | PROPOSED | 2026-05-17 | 1 | KEEP PENDING — defer-until-2nd-occurrence per author |
| 11 | /andromeda-phase merge-protocol cross-extract evidence-consistency | PROPOSED | 2026-05-17 | 1 | KEEP PENDING — author defers |
| 12 | **Type 6 evolve markers pre-populate CLAUDE.md derived-section cascade** | PROPOSED | 2026-05-18 (s94) | **3+ direct**: chunks #58, #60, #68 → CLAUDE.md Modules/Stack desynced | **IMPLEMENT NOW** (alongside P7) — same theme; closes recurring drift |
| 13 | First-class sub-phase state for two-phase chunks | PROPOSED | 2026-05-19 (s97) | 1 (chunk #69 Phase A); chunk #74 LLM expected 2nd | KEEP PENDING — defer until 2nd occurrence |
| 14 | /implement Phase 2b smoke-check supports "integration-test runtime smoke" | PROPOSED | 2026-05-19 (s98) | 2 (chunk #69 Phase B Sessions 1+2) | KEEP PENDING — defer-until-3rd-occurrence per author |

### 8.3 IMPLEMENT NOW pair — P7 + P12

**P7 — Type 6 narrative-cascade visibility:**
- 4+ documented dogfood occurrences (chunks #43, #58, #60, #68)
- Explicit carry-over in session-handoff.md
- Implementing resolves Dim 6 rows 9-11 (arch.md "eight library crates" stale at 12 in 3 locations)
- Author preference: bundle with P5/P6 (already implemented; standalone now feasible)
- Cost estimate: ~75 LOC across 4 skill files

**P12 — Type 6 CLAUDE.md derived-section cascade pre-populate:**
- 3+ direct occurrences (chunks #58, #60, #68 each Type 6 left CLAUDE.md Modules/Stack desynced)
- Operational workaround documented in session-learnings 2026-05-18
- Author preference: "Pairs naturally with Proposal 7 Phase 2"
- Cost estimate: ~80 LOC across 5 skill files

**Together: ~155 LOC across ~8 skill files in `~/.claude/skills/andromeda-{evolve,setup-project,wrap-session}/`. Closes recurring drift category.**

### 8.4 Newly-surfaced patterns (NEW PROPOSAL candidates)

Four patterns surfaced from this audit + recent session-learnings that aren't yet captured:

#### P15 — Route ↔ source-doc chunk-number divergence canonical artifact

- **Evidence:** session-learnings 2026-05-19 documents the full divergence table (route §67 = v0_2_0-route §68 = Service Registry; route §69 = v0_2_0-route §67 = Drain; etc.); BROKEN cross-ref row 3 (route.md:329) directly caused by this divergence
- **Scope:** Surface divergence table in route.md §3 Decisions Log preamble OR generate at evolve time OR living artifact at `.andromeda/context/route-source-doc-mapping.md`
- **Confidence:** 0.85 — pattern recurred 3+ times; will continue accumulating

#### P16 — D7 cross-document reference resolution check at wrap-session

- **Evidence:** this audit found 8 BROKEN + 4 STALE cross-references that NO existing drift dimension catches (D1-D6 all miss these patterns)
- **Scope:** Add Dim D7 to wrap-session integrity protocol: parse §/chunk/file references; verify each target resolves; emit warnings for unresolved
- **Confidence:** 0.80 — would catch all 8 audit BROKEN refs

#### P17 — Doc-rename cascading reference update

- **Evidence:** `widget-state-validation-mini-route.md` renamed to `pulse-v0_2_0-route.md`; cross-refs in 3 files still cite old name
- **Scope:** When `/andromeda-evolve` renames a doc, automatically grep all other docs for old filename and offer to update
- **Confidence:** 0.65 — 1 observed occurrence; cost low

#### P18 — Stale TODO detection in v0_2_0 planning docs

- **Evidence:** dist-arch:980-995 stale "TODO: capability spec formalization" — all 8 items done in P-052..P-060
- **Scope:** wrap-session or new-session lightweight check that greps TODO markers in v0_2_0/*.md; reports those that may be resolved based on other-doc changes
- **Confidence:** 0.55 — 1 observed; speculative recurrence

---

## Dimension 9 — State.yaml integrity

### 9.1 Field-by-field verification

| Field | Recorded value | Reality | Match? |
|---|---|---|---|
| `schema_version` | 2 | per session-state-contract.md v2 schema | ✓ |
| `session_count` | 101 | matches commit lineage; 138 total commits | ✓ |
| `last_wrap` | 2026-05-19T23:10:00Z | recent; consistent with handoff | ✓ |
| `last_reconcile` | 2026-05-19T23:10:00Z | matches Last reconciled in dep-tree.md | ✓ |
| `last_completed_chunk.route_index` | 69 | matches route.md §2 final chunk | ✓ |
| `last_completed_chunk.epoch` | 9 | matches route.md §2 Epoch 9 | ✓ |
| **`last_completed_chunk.commit_sha`** | `5402d3f` | **non-existent git object** (verified via `git cat-file -e`); actual session 101 wrap = `3602f82` | **⚠ STALE** |
| `last_completed_chunk.committed_at` | 2026-05-19T23:10:00Z | consistent | ✓ |
| `in_progress` | null | no active phase | ✓ |
| `spec_amendments.active` | [] | empty | ✓ |
| `spec_amendments.archive` | 38 entries | matches `ls .andromeda/runs/*-spec-amendment-*/ \| wc -l` = 38 | ✓ |
| `drift_warnings` | [] | empty post-wrap | ✓ |
| `living_artifact_freshness.dep_tree_reconciled_at` | 2026-05-19T23:10:00Z | matches | ✓ |
| `living_artifact_freshness.api_surface_reconciled_at` | 2026-05-19T23:10:00Z | matches but DEFERRED (see Dim 7) | ✓ formally |
| `living_artifact_freshness.api_surface_deferred` | true | 7th consecutive deferral | ✓ honest |

### 9.2 Findings

1. **MEDIUM — `last_completed_chunk.commit_sha` references non-existent git object.**
   - Recorded: `5402d3f`
   - Actual: `3602f82` (per `git log --oneline | head -1`)
   - Known pipeline artifact: wrap-session writes state.yaml BEFORE the wrap commit, capturing a planned-hash placeholder. The placeholder doesn't always match actual final commit hash (planning-vs-reality gap).
   - **Auto-reconciles next /wrap-session** (state H detector at wrap-session Phase 8 catches this; sessions 89/91/94 have prior precedent of "State H stale commit_sha reconciled").
   - **No remediation needed** — system self-corrects.

2. **Honest deferral flag.** `living_artifact_freshness.api_surface_deferred: true` is the correct way to signal known state — even though the file's own "Last reconciled" timestamp is misleading (Dim 7 finding). The state.yaml field tells the truth; the file metadata doesn't.

### 9.3 Cross-source consistency check

| Source | Claim | Match? |
|---|---|---|
| state.yaml.session_count | 101 | matches session-handoff.md "Last Updated 2026-05-19T23:10:00Z" + git wrap commit subjects "session 101" |
| state.yaml.spec_amendments.archive count | 38 | matches `grep -c "amendment_id:" .andromeda/state.yaml` = 38 ✓ |
| state.yaml.spec_amendments.archive count | 38 | matches `ls .andromeda/runs/*-spec-amendment-*/ \| wc -l` = 38 ✓ |
| run-dirs total | 175 | includes setup-project / evolve / phase / wrap runs in addition to amendment markers; 38 of 175 are amendment markers — consistent |

### 9.4 Net Dim 9 verdict

state.yaml is **structurally honest** about its known limitations (deferral flag) but has **one self-corrected stale field** (commit_sha placeholder). Next wrap-session auto-reconciles. No active integrity violations.

---

## Section 1 — Critical inconsistencies (HIGH)

These MUST be remediated before pulse v0.2.0 ships.

### 1.A — Persistence triple-mechanism (Dims 1 + 2 + 4 + 5 overlap)

**Cluster:** EwmaTracker / TDigestPair / RollingWindow / ActivityHistogram (chunks #61, #64) ride flat-file BaselineState bincode at `<data_dir>/triage/baseline-corpus.bin`. DrainState (chunk #69) rides corpus SQLite. ServiceRegistry (chunk #67) / RetryStormState (chunk #66) / RestartDetector / SuppressionState (chunk #63) are pure in-memory — lost on every restart.

Spec violations: P-009 + P-013 explicitly mandate corpus persistence ("baseline state SHALL be persisted to corpus", "histogram state SHALL be persisted ... via corpus persistence"). P-027 verification "verify dot positions match prior session" implies ServiceRegistry persistence. Chunk #66 storm.rs docstring explicitly defers persistence to chunk #69 — broken promise.

Cross-cutting impact: PII scrubber asymmetric (flat-file BaselineState bypasses scrubber); encryption asymmetric (flat-file plaintext vs corpus AES-256-GCM); arch §Occupied Resources doesn't acknowledge `triage/baseline-corpus.bin`; security plan §Threat Model "no persistent disk database" stale; P-051 Settings storage panel can't inspect/delete the flat-file.

### 1.B — PII scrubber single-site coverage (Dims 1 + 2 + 5 overlap)

`crates/security/src/scrubber.rs::scrub_attribute` invoked at exactly one production site (`crates/buffer/src/drain.rs:600` — Drain DuckDB write only).

Bypassed paths:
- OTLP ingestion → DuckDB writes (spans, log_records, span_events including raw exception.message + exception.stacktrace per P-006 violation, metrics_points)
- BaselineState save (service.name keys verbatim in flat-file)
- Drain DrainState corpus pipeline_metrics save (same chunk #69 — internal inconsistency)
- All future CorpusWriter::save_pipeline_metric callers (contract docstring says "callers SHOULD pre-scrub" — does not enforce)

Spec violation: P-047 SHALL "all content before persistence."

### 1.C — Chunk #69 internal scrubber inconsistency

Same source data (Drain template tokens) writes to two persist targets with different PII discipline:
- DuckDB log_templates write: scrubbed via `write_template_to_table` at `drain.rs:586-606` ✓
- Corpus pipeline_metrics persist via `snapshot_state()` → bincode → `CorpusWriter::save_pipeline_metric`: NOT scrubbed ✗

Token source is the same (`TemplateRecord.tokens` at `drain.rs:200`); only one of two writes scrubs.

### 1.D — Capability spec PARTIAL gaps (20 items)

Twenty capabilities claimed live by chunks #57-#69 have HIGH-severity unfulfilled spec language. Detail in [Dim 2 §2.2](#22-detailed-partial-findings-high-severity); summary:

- **Threshold/percentile mismatches:** P-001 (5s/30s vs 10s/60s), P-012 (p95 vs p99), P-014 (missing 30s floor)
- **Baseline-relative semantics broken:** P-010 + P-012 use FIXED constants instead of per-service streaming baselines from P-009/P-011 (negates P-009/P-011's value)
- **Identity preservation broken:** P-011 (operation_name hashed away — irrecoverable in surface)
- **Persistence broken or absent:** P-013 (1h staleness gate defeats restart-survival promise), P-027 (CorpusRestore stub unreachable)
- **PII scrubber not applied:** P-006 (exception.message raw), P-047 (single-site only), P-048 (raw OTLP attributes in DuckDB)
- **Encryption fallback missing:** P-049 (no passphrase fallback)
- **Surfaces entirely absent:** P-019 (no FindingsCounter/Dropdown), P-051 (delete entirely absent), P-058 (1 of ~30 metrics; no Diagnostics view)
- **Wrong tool surface:** P-038/P-039 (telemetry tools vs spec incident-retrieval tools)
- **Direct prohibition violated:** P-024 (widget FooterBand displays numerical readouts — spec explicitly forbids)
- **Failure paths incomplete:** P-003 (only BindFailed wired; ReceiverPanicked unreachable)

### 1.E — Capability ORPHAN + mis-attribution (3 items)

- P-005 (Span Status Error Detection): no chunk attribution; EWMA aggregation isn't "candidate emission"
- P-007 (High-Severity Log Capture): route §2 attributes to chunk #69 but Drain template mining ≠ SeverityNumber≥17 filter — **mis-attribution**
- P-008 (Root-Span Error Scope): no chunk; no parent_span_id-aware detector

### 1.F — Chunk #69 Step 32 incomplete at registry level (Dim 4)

Session 101 Type 6 amendment registered `diagnostics.template_distribution` TauRPC but did NOT register `log_templates` DuckDB table. `schema.rs:5-9` self-flags pending registration. **Highest-priority single arch-fix.**

Additional Dim 4 HIGH: corpus SQLite 6-table schema (`baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive`) NOT registered (chunk #68 amendment covered file path only); `triage/baseline-corpus.bin` subpath NOT registered.

### 1.G — Documentation capability-to-chunk table internal stale (Dim 6)

`docs/v0_2_0/pulse-v0_2_0-route.md:712` row "P-019 to P-023, P-060 | #67 superseded by #72-#77" — v2 chunk #67 = Drain, not Severity classifier. Reader following the table gets wrong chunk pointers for severity-related capabilities.

---

## Section 2 — Notable inconsistencies (MEDIUM)

### 2.H — Security plan §Threat Model + §Data Protection staleness

Plan asserts "no persistent disk database" / "no persistent user data store" — directly contradicted by chunks #61 + #68 introducing persistent storage. Plan never re-derived; session-handoff carries "Cross-cutting `/andromeda-security` re-run flagged" across 8+ wraps.

### 2.I — Registry-stale forward-promises (Dim 4)

arch §Occupied Resources lists items that don't exist in code:
- `snapshot.list_recent`, `snapshot.copy_to_clipboard` TauRPC
- `workspace.list` TauRPC
- `pulse://stream/plugin-events` broadcast topic
- `ANDROMEDA_PULSE_CONFIG_PATH` env var

Either implement OR remove from arch registry.

### 2.J — Doc cross-reference drift (Dim 6)

7 MEDIUM-severity broken/stale references:
- arch.md:167 wrong obs-plan section number
- route.md:329 broken line number
- 3 refs to renamed `widget-state-validation-mini-route.md`
- dist-arch:980-995 stale TODO (8 items already done in P-052..P-060)
- capability-spec:789 stale mini-route reorganization clause
- arch.md narrative cascade "eight library crates" at lines 4 + 220 + 303 (count actually 12)

### 2.K — PII vector test gaps (Dim 5)

4 deferred PII vector tests + capability widening static analysis test never materialized. Documented in `.claude/rules/testing.md` §Pending coverage triggers.

### 2.L — state.yaml.last_completed_chunk.commit_sha stale (Dim 9)

`5402d3f` non-existent vs actual `3602f82`. Auto-reconciles at next /wrap-session. No action needed; known pipeline artifact.

### 2.M — api-surface.md 7-wrap deferred (Dim 7)

Formally fresh timestamp but LIVING block is from session ~93 baseline; current code surface tracked only in METADATA narrative. Next /implement-followed wrap is natural re-baseline checkpoint.

---

## Section 3 — Cleanup opportunities (LOW)

### 3.N — Tray icon Halo State Pulse missing [scope-adjacent — v0.1.0]

design-system.md Self-Validation #3 fails: tray icon ships static SVG glyph only; secondary halo composited layer (WebGPU or SVG-filter per layout-templates.md Option A/B) never built. v0.1.0 chunk #36 substrate landed the static glyph. Pre-v0.1.0 ship if Self-Validation is release gate.

### 3.O — Widget errorRate hardcoded zero (Dim 5)

`pulse-app/ui/src/hooks/use-widget-metrics.ts:88` returns `errorRate: 0`. Halo never shifts to Alert Burgundy. Deferred to chunk #81 ("Halo data path refactor"). On route plan elsewhere.

### 3.P — Tauri capability identifier prefix cosmetic (Dim 4)

arch uses `pulse:default`/`pulse:tray`; JSON uses bare `"default"`/`"tray"`. No functional drift. Optional doc convention reconciliation.

### 3.Q — Harness-only env vars unregistered (Dim 4)

`ANDROMEDA_PULSE_PIDFILE`, `LOGFILE`, `DATA_DIR_KEEP` used only in `scripts/agent-run.{sh,ps1}`. Optional to register. verification-harness rule documents PID/log locations.

### 3.R — METADATA section bloat in living artifacts (Dim 7)

api-surface.md and dependency-tree.md METADATA sections grew to large size accumulating session-by-session narrative. Worth pruning periodically.

### 3.S — Chunk #69 LogHub golden corpus not visible (Dim 5)

Chunk plan referenced LogHub corpus; no `tests/golden/drain/` directory visible. Algorithm correctness verified via unit tests. Documentation-vs-impl mismatch only.

### 3.T — `run/andromeda-pulse.pid` subpath [scope-adjacent — v0.1.0]

Written by `pulse-app/src/main.rs:188`. Not in arch §Occupied Resources. Chunk #4 (Epoch 1) substrate.

### 3.U — Capability-spec validation report unprefixed (Dim 6)

`pulse-capability-spec.md:5` references `widget-state-validation-report-2026-05-14.md` (exists at `.andromeda/scope-validation/`). Unprefixed — readers can't navigate without grep.

---

## Section 4 — Implementation proposals lifecycle

### 4.1 IMPLEMENT NOW (mature pending, resolves audit findings)

- **P7 — Type 6 narrative-cascade visibility** — 4+ unaddressed occurrences (chunks #43/#58/#60/#68); implementation resolves Section 2.J narrative cascade drift (3 arch.md lines)
- **P12 — Type 6 CLAUDE.md derived-section cascade pre-populate** — 3+ direct occurrences; pairs naturally with P7

Together: ~155 LOC across ~8 skill files; closes recurring drift category (Type 6 amendments leaving CLAUDE.md + arch narrative stale).

### 4.2 KEEP PENDING (proposed but not yet mature)

- P1 — `--allow-arch-decision` flag (0 occurrences; explicit author defer to feel friction first)
- P2 — setup-project generates playbook (1 occurrence; awaits 2nd Andromeda project)
- P3 — pointer table includes playbook (1 occurrence; bundle with P2)
- P8 — Arch Registry Updates compaction Phase 2 (threshold not reached; ~12 entries vs proposed ~15-20)
- P9 — route.md decisions log compaction Phase 2 (explicitly post-v1.0 ship gate)
- P10 — setup-project --delta detects non-delta-scoped uncommitted (1 occurrence)
- P11 — /andromeda-phase merge-protocol cross-extract (1 occurrence)
- P13 — first-class sub-phase state (1 occurrence; chunk #74 LLM expected 2nd)
- P14 — /implement Phase 2b integration-test smoke (2 same-chunk occurrences)

### 4.3 NEW PROPOSALS to file (P15-P18)

Four patterns surfaced during this audit not yet captured:

- **P15** — Route ↔ source-doc chunk-number divergence canonical artifact (confidence 0.85; 3 occurrences)
- **P16** — D7 cross-document reference resolution check at wrap-session (confidence 0.80; catches all 8 BROKEN refs from this audit)
- **P17** — Doc-rename cascading reference update (confidence 0.65; 1 occurrence)
- **P18** — Stale TODO detection in v0_2_0 planning docs (confidence 0.55; 1 occurrence, speculative)

Filing cost ~30 min total per author convention. Capture before pattern memory fades.

### 4.4 DEPRECATE candidates

None. All 14 existing proposals remain applicable.

---

## Section 5 — Remediation recommendations

For each Section 1 HIGH finding, propose remediation category (no chunk authoring — categories only).

### 5.1 → 1.A Persistence triple-mechanism

**Category: Persistence consolidation chunk(s)** — migrate all runtime state currently in flat-file or pure-in-memory to corpus SQLite.

Mechanical scope:
- Define `trait BaselinePersistence` in `crates/triage/src/baseline/` (lower-crate pattern preserved per session-learnings 2026-05-16)
- Implement `CorpusBaselinePersistence` adapter in `pulse-app/src/baseline_persistence.rs` over `Arc<dyn CorpusWriter>` — mirror chunk #69 `CorpusDrainPersistence` pattern
- Replace `crates/triage/src/baseline/corpus.rs` flat-file code with calls through the trait
- Delete `<data_dir>/triage/baseline-corpus.bin` filesystem path
- Wire `service_registry` table (already exists in schema) to `InMemoryServiceRegistry` save/load; implement missing `set_state_on_corpus_restore` method; make `TransitionTrigger::CorpusRestore` reachable
- Wire RetryStormState (`DashMap<[u8; 16], FingerprintState>`) save/load to corpus (closes broken chunk #66 promise)
- Migration: handle existing `baseline-corpus.bin` files — read-and-migrate-once OR document acceptable loss

Schema decision: use existing `baseline_state` table (per-service rows; current bincode stores entire DashMap as one blob → needs schema refactor) OR use `pipeline_metrics` as flat blob slot mirroring Drain precedent. Both valid; chunk author chooses.

Possibly split into 2-3 sub-chunks if too large per single phase.

### 5.2 → 1.B + 1.C PII scrubber coverage closure

**Category: PII scrubber extension chunk**

Mechanical scope:
- Wire `scrub_attribute` at OTLP appender per-attribute write (`crates/buffer/src/appender.rs:332-356` for exception.message + extend to log_records.body + span attributes)
- Wire scrubber at Drain corpus persistence (close 1.C internal inconsistency — scrub DrainState.templates[].tokens before bincode-serializing for corpus payload)
- Decision: extend CorpusWriter trait with pre-scrub contract (test-enforceable invariant) OR document per-caller scrub discipline + add tests at each call site
- Add PII negative-canary tests at each new scrub site

### 5.3 → 1.D Capability spec PARTIAL alignment

**Category: Capability spec alignment chunks** — per-capability decision (implement spec OR amend spec language). Recommend grouping by remediation type:

- **Numeric threshold adjustments** (single PR): P-001 thresholds (5s/30s → 10s/60s + heartbeat alarm decoupling); P-014 add 30s floor; P-012 percentile constant (0.95 → 0.99)
- **Per-service baseline wiring**: P-010 + P-012 read from per-service EWMA + t-digest instead of fixed constants. Couples with persistence consolidation since baselines must be loadable
- **Failure path completion**: P-003 wire `ReceiverPanicked` via `std::panic::set_hook`
- **Identity preservation**: P-011 add `operation_name` field on OperationBaseline + OperationMetricSnapshot
- **Surface additions**: P-019 + P-024 + P-051 — Phase 8 UI chunks (#78/#79/#80/#81); folds into existing route plan
- **MCP tool surface refactor**: P-038/P-039 — add incident-retrieval tools when chunks #84 + #79 land
- **Encryption fallback**: P-049 passphrase prompt + fallback path
- **Spec downgrades (alternative path)** for items where implementation truth is preferred: P-001 numeric → "configurable bounds"; P-010/P-012 → "by configurable multiplier (default 3.0/2.5)"; P-019 surface intensity → "tier metadata visible in incident records"

### 5.4 → 1.E Orphan capabilities

**Category: Spec amendment for orphan capabilities**

- P-005 + P-008: clarify whether required at MVP. If yes, schedule implementation chunks; if no, mark v0.3.0+ in spec
- P-007: remove route §2 chunk #69 mis-attribution ("(capability P-007)" annotation); downgrade to "prerequisite for P-007 via future Q6 query" OR implement SeverityNumber≥17 filter as new chunk

### 5.5 → 1.F Architecture registry alignment

**Category: Type 6 amendment batch chunk**

Run `/andromeda-evolve --allow-arch-registry` adding:
- DuckDB `log_templates` table (closes chunk #69 Step 32 properly)
- New §Occupied Resources sub-section "Corpus SQLite database / schema names" with 6 tables
- Filesystem subpath `triage/baseline-corpus.bin` (or omit if Persistence Consolidation removes the file first)

Then registry cleanup:
- Remove or tag-as-deferred: `snapshot.list_recent`, `snapshot.copy_to_clipboard`, `workspace.list`, `pulse://stream/plugin-events`, `ANDROMEDA_PULSE_CONFIG_PATH`

### 5.6 → 1.G + 2.J Documentation consolidation

**Category: Documentation consolidation chunk** — single batch edit session resolves all 11 cross-reference drift items + 3 narrative cascade lines:

- arch.md:167 → "obs-plan §3 Logging stack > Frontend bridge"
- route.md:329 → "§Phase 4 line 312"
- dist-arch:5 / capability-spec:5 → replace `widget-state-validation-mini-route.md` with `pulse-v0_2_0-route.md`
- capability-spec:5 → prefix `widget-state-validation-report-2026-05-14.md` with `.andromeda/scope-validation/`
- capability-spec:789 → strike or rephrase to past-tense
- dist-arch:980-995 → delete §TODO subsection OR rewrite as "Resolved in capability spec v2"
- v0_2_0-route.md:712 → split row to accurate v2 chunk pointers (`P-019 to P-023 | #75 (LLM severity)`, `P-060 | #72, #74`)
- arch.md narrative cascade (lines 4 + 220 + 303) → "twelve library crates / fourteen workspace members" (depends on Section 5.7 P7 implementation OR manual edit)

Cost ~30 minutes manual batch edit.

### 5.7 → recurring drift theme Andromeda meta-improvements

**Category: Andromeda pipeline improvement chunk (skill enhancement)**

Implement P7 + P12 together (~155 LOC across ~8 skill files in `~/.claude/skills/andromeda-{evolve,setup-project,wrap-session}/`):
- P7 makes Type 6 amendments cascade to arch narrative count lines (mechanical regex-replace OR full re-derive trigger)
- P12 makes Type 6 amendments cascade to CLAUDE.md derived sections (Modules/Stack/Key directories)

Closes recurring drift theme. Section 5.6 narrative cascade items become auto-maintained after P7 lands. Sessions 95 + 101 manually-handled CLAUDE.md cascades become automatic after P12 lands.

### 5.8 → 2.H + 2.K Specialist plan reconciliation

**Category: Specialist plan re-runs**

- `/andromeda-security` — extends §Threat Model + §Data Protection §At rest with persistent-storage rows; §Secret Management with corpus AES-256-GCM key custody flow. Closes Section 2.H. Pairs with Persistence Consolidation timing.
- `/andromeda-tests` — materializes 4 deferred PII vector triggers + capability widening static analysis trigger; possibly chunk #69 LogHub golden corpus harness. Closes Section 2.K.

### 5.9 → 2.L + 2.M State.yaml + living artifacts housekeeping

**Category: housekeeping (not a chunk — routine wrap-session action)**

- 2.L self-corrects at next /wrap-session
- 2.M reconciles at next /implement-followed wrap (per integrity-protocol.md pragmatic deviation pattern; 8th deferral could become explicit forcing function)

### 5.10 → Section 3 LOW items

Most LOW items fold into bigger chunks or defer to post-v0.2.0 ship:
- 3.N tray Halo + 3.T `run/*.pid` — bundle into v0.1.0 ship blocker chunk (per session-handoff carry-over)
- 3.O widget errorRate=0 — already on route plan as chunk #81
- 3.P + 3.Q cosmetic registry cleanups — fold into Section 5.5 arch registry chunk
- 3.R METADATA bloat — defer; consider one-time prune as side effect of api-surface re-baseline
- 3.S LogHub corpus — fold into Section 5.8 `/andromeda-tests` re-run
- 3.U `widget-state-validation-report` path prefix — fold into Section 5.6 doc consolidation

---

## Section 6 — Suggested consolidation chunk grouping

Natural scope boundaries identified (no ordering imposed, no session estimates).

### Group A — Persistence Consolidation
Closes findings: 1.A (full), 1.B partial (corpus boundary scrub), 1.D partial (P-013/P-027 persistence), 2.H partial (security plan persistent-storage section).

Scope: migrate BaselineState components + ServiceRegistry + RetryStormState from flat-file/in-memory to corpus SQLite using DrainPersistence-trait-pattern. Possibly split into 2-3 sub-chunks (baselines / service registry / storm state) if too large per phase. Pairs with `/andromeda-security` re-run.

### Group B — PII Scrubber Coverage Extension
Closes findings: 1.B (full), 1.C (full), 1.D partial (P-006 + P-047 + P-048).

Scope: wire `scrub_attribute` at OTLP appender + log_records + span_events; close Drain internal inconsistency; extend CorpusWriter trait contract (test-enforceable) OR add per-caller PII canary tests.

### Group C — Capability Spec Numeric Alignment
Closes findings: 1.D partial (P-001 thresholds, P-012 percentile, P-014 floor, P-010/P-012 baseline-relative, P-011 operation identity, P-003 failure paths).

Scope: per-capability decision (implement vs amend). Group by remediation type — threshold adjustments / baseline wiring / surface plumbing / amendment.

### Group D — Capability Spec Surface Additions
Closes findings: 1.D partial (P-019 + P-024 + P-051 + P-058), 3.O widget errorRate.

Scope: Phase 8 UI chunks #78 + #79 + #80 + #81 + #87 (already on route plan; needs implementation prioritization). Folds chunk #81 widget errorRate refactor.

### Group E — Capability Spec MCP + Encryption + Orphans
Closes findings: 1.D partial (P-038/P-039/P-049), 1.E (P-005/P-007/P-008).

Scope: MCP tool surface refactor (incident-retrieval); P-049 passphrase fallback; orphan capability decision (implement OR mark v0.3.0+). Includes P-007 mis-attribution cleanup in route §2.

### Group F — Architecture Registry Alignment
Closes findings: 1.F (full), 2.I (full), 3.P cosmetic, 3.Q harness-only env vars.

Scope: Type 6 amendment batch for log_templates DuckDB table + corpus SQLite schema sub-section + baseline-corpus.bin (if not removed by Group A); registry cleanup for forward-promises; cosmetic prefix reconciliation.

### Group G — Documentation Consolidation
Closes findings: 1.G (full), 2.J (full), 3.U.

Scope: single batch edit fixing 8 BROKEN + 4 STALE cross-references + arch.md narrative cascade lines (depends on Group H P7 OR manual). Cost ~30 minutes.

### Group H — Andromeda Pipeline Meta-Improvements
Closes findings: recurring drift theme; resolves Section 2.J narrative cascade lines automatically going forward.

Scope: implement Proposal 7 (Type 6 → arch narrative cascade) + Proposal 12 (Type 6 → CLAUDE.md derived sections cascade) in `~/.claude/skills/andromeda-{evolve,setup-project,wrap-session}/`. ~155 LOC across ~8 skill files. File 4 NEW proposals (P15-P18) before pattern memory fades.

### Group I — Specialist Plan Re-runs
Closes findings: 2.H (full), 2.K (full), 3.S.

Scope: `/andromeda-security` re-run + `/andromeda-tests` re-run. Pairs with Group A timing (security plan needs current persistence picture to re-derive).

### Group J — v0.1.0 Ship Blockers (scope-adjacent)
Closes findings: 3.N (tray Halo), 3.T (run/*.pid), chunk #3 deferred signing items.

Scope: pre-v0.1.0 release administrative + UI completeness. Not v0.2.0 consolidation per se; flagged here because it's the natural close-out adjacent to Section 6 work.

---

## Appendix A — File:line evidence index

Grep-able index of every file:line cited in this audit.

### crates/

- `crates/buffer/src/appender.rs:317,332-333,349,1080-1160,1282-1335` — P-006 raw exception.message; PII canary test
- `crates/buffer/src/drain.rs:40+,169-180,200,420-424,430,583-628,600-606` — Drain algorithm, DrainPersistence trait, PII scrubber call, masking, write_template_to_table
- `crates/buffer/src/fingerprint.rs:79-95,118-169,357` — chunk #66 fingerprint compute + normalize_stacktrace + negative test
- `crates/buffer/src/schema.rs:5-9,10-19,18,51,86,125` — RESERVED_TABLES (8 entries with log_templates self-flagged for registration); span_events schema
- `crates/buffer/src/broadcast.rs:9-11` — pulse://stream/{spans,metrics,logs} constants
- `crates/corpus/src/contract.rs:60-71,127-146,157-161,162,191,198-208,220` — Corpus::open, CorpusReader::inspect, CorpusWriter trait + impl, pre-scrub docstring, read query
- `crates/corpus/src/db.rs` — chunk #68 SQLite scaffold
- `crates/corpus/src/encryption.rs:52-78,92-93` — AES-256-GCM cell wrap; encryption round-trip test
- `crates/corpus/src/keychain.rs` — OS keychain backend abstraction
- `crates/corpus/src/schema.rs:26-33,44-53,55-62` — TABLE_NAMES (6 corpus tables); service_registry DDL; pipeline_metrics DDL
- `crates/ingest/src/connection.rs:25,39-44,79-93,195-201,264-282` — STREAM_NAME_CONNECTION_STATE; thresholds (5s/30s); ReceiverFailureReason derivation; last_span_ago_ms
- `crates/ingest/src/grpc.rs,http.rs` — receiver loopback bind
- `crates/security/src/scrubber.rs:113-179,166` — 7-category scrubber + proptest
- `crates/snapshot/src/contract.rs` — snapshot crate contract
- `crates/triage/src/baseline/mod.rs:112-122,205,368-375,444-452,454,469,483-495,500,512-544,537,780,985,1000-1049` — BaselineState struct, observe_span STATUS_CODE_ERROR, operation_key hash, persist event, persist_count_total, run_persist_loop, bootstrap_state
- `crates/triage/src/baseline/corpus.rs:32-43,45-73,75-126,299-317` — resolve_corpus_path, persist_state, load_state, PII canary test
- `crates/triage/src/baseline/activity_floor.rs:33,422` — BOOTSTRAP_WINDOW_SECONDS (1h), proptest
- `crates/triage/src/baseline/ewma.rs:122` — proptest
- `crates/triage/src/baseline/tdigest_pair.rs:193` — proptest
- `crates/triage/src/baseline/rolling_window.rs:118` — proptest
- `crates/triage/src/contract.rs:111-116` — PriorityTier enum
- `crates/triage/src/cue/broadcast.rs:8` — STREAM_NAME_ATTENTION_CUES
- `crates/triage/src/cue/classify.rs:15-30,40-57` — classify_priority, dual_condition_bypass
- `crates/triage/src/cue/evaluate.rs:30-64,45,66-99,74-87,116-153,117-153` — error rate spike + latency regression + service-went-silent evaluation
- `crates/triage/src/cue/thresholds.rs:19,24,36` — base_error_rate, base_latency_ms, DEFAULT_LATENCY_PERCENTILE
- `crates/triage/src/lifecycle/state_machine.rs:38,47` — CorpusRestore no-op stub, TransitionTrigger enum
- `crates/triage/src/lifecycle/registry.rs:7-11,113-116` — InMemoryServiceRegistry DashMap-only
- `crates/triage/src/lifecycle/broadcast.rs:16` — STREAM_NAME_SERVICE_LIFECYCLE
- `crates/triage/src/pattern/storm.rs:38-39,94-95` — storm.rs persistence-deferred-to-chunk-#69 docstring
- `crates/triage/src/pattern/detector.rs` — RestartDetector
- `crates/triage/src/pattern/suppression.rs:130-159` — evaluate_with_suppression with bypass
- `crates/triage/src/pattern/broadcast.rs:8` — STREAM_NAME_RESTART_EVENTS
- `crates/triage/src/cue/emitter.rs:759+` — cue emitter PII bans tests
- `crates/triage/src/pattern/mod.rs:49+` — pattern module PII tests
- `crates/ui-bridge/src/contract.rs:295` — config.toml path
- `crates/ui-bridge/src/health.rs:291,304,421-486` — TauRPC top-level envelope, CONFIG_FILE const, settings round-trip
- `crates/ui-bridge/src/workspace_ipc.rs:47` — workspace.detect (only `detect` not `list`)
- `crates/ui-bridge/src/telemetry.rs:97` — telemetry.frontend.record_frame_ms
- `crates/curation/src/aggregation.rs:239` — proptest
- `crates/snapshot/src/markdown.rs:1009` — proptest
- `crates/mcp-server/src/tools.rs:26` — query_traces/metrics/logs/generate_snapshot (wrong P-038/P-039 surface)
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:27` — ANDROMEDA_PULSE_DATA_DIR
- `crates/mcp-server/src/feature_gate.rs:3` — MCP feature double-gate
- `crates/plugins/src/loader.rs:32,33,107` — ANDROMEDA_PULSE_PLUGIN_DIR
- `crates/triage-experimental/Cargo.toml:16-20` — standalone workspace root declaration

### pulse-app/

- `pulse-app/src/main.rs:58,59,60,160,188,213,275,278-283,358,382-402,417,428` — env var consts; PID file; data_dir resolution; baseline boot wire; corpus_db_path; corpus boot
- `pulse-app/src/connection_router.rs:46,99-112` — ConnectionApiImpl + derive_reason_for_response (no ReceiverPanicked)
- `pulse-app/src/diagnostics_router.rs:59` — diagnostics.template_distribution
- `pulse-app/src/drain_persistence.rs:35-40,42-81,55` — DRAIN_TEMPLATE_METRIC_NAME, DRAIN_PERSISTENCE_LAYER, CorpusDrainPersistence
- `pulse-app/src/mcp_router.rs:55,168,271` — mcp.status/start/stop, env-var double-gate
- `pulse-app/src/services_router.rs:32` — services.list_with_states
- `pulse-app/src/snapshot_runtime.rs:130,212,219,242` — snapshot.generate (only); snapshot file write; pulse://stream/snapshot-progress event-name literal
- `pulse-app/src/storage_router.rs:49` — storage.inspect/path
- `pulse-app/src/streams.rs:14` — streams.subscribe_spans/metrics/logs
- `pulse-app/src/viz_routers.rs:10,41,72` — traces/metrics/logs.query
- `pulse-app/src/plugins_router.rs:48` — plugins.list/reload/invoke
- `pulse-app/src/observability.rs:99-1394,1051-1391,1057-1105,1121-1168,1182-1209,1230-1273,1291-1341,1350-1391,1582` — AllowList registry per chunk
- `pulse-app/src/heartbeat.rs:138,200-211,201,251,269,284` — ingest/buffer/viz/plugins/connection tick spawns
- `pulse-app/src/tray.rs:5,110,110-161` — static SVG glyph (no Halo Pulse layer)
- `pulse-app/src/window.rs:167-209` — widget snap-to-edge
- `pulse-app/capabilities/*.json` — capability identifiers (`default.json:3`, `clipboard.json:3`, `tray.json:3`, `updater.json:3`, `notification.json:3`, `plugin-fs.json:3`)
- `pulse-app/ui/src/App.tsx:3,13-15,19` — useSyntheticHaloInput defer marker
- `pulse-app/ui/src/hooks/use-widget-metrics.ts:15-17,49,74-80,88` — errorRate: 0; throughputHz subscription; chunk #81 defer comment
- `pulse-app/ui/src/widget/CompactWidget.tsx:38-48` — semantic main
- `pulse-app/ui/src/widget/FooterBand.tsx:61-63` — role=status aria-live; P-024 violation (numerical readouts)
- `pulse-app/ui/src/widget/AggregatedBadgeCanvas.tsx:23-79` — Halo widget surface
- `pulse-app/ui/src/dashboard/Dashboard.tsx` — Halo dashboard surface
- `pulse-app/ui/src/dashboard/routes/diagnostics/TemplateDistribution.tsx` — chunk #69 UI
- `pulse-app/ui/src/styles/tokens.css:4-16,44-73,76-82` — NASA palette + fonts + reduced-motion
- `pulse-app/ui/src/components/icons/` — Observatory iconography
- `pulse-app/ui/src/dashboard/SkipToMain.tsx` — a11y skip link
- `pulse-app/ui/src/hooks/use-reduced-motion.ts` — useReducedMotion hook
- `pulse-app/ui/src/bindings/index.ts` — auto-gen TauRPC bindings
- `pulse-app/tauri.conf.json:13-20,22-30` — widget + dashboard window declarations

### xtask/

- `xtask/src/main.rs:350,655-684,681-683` — EXPECTED_PROCEDURES (with future-deferred comments)
- `xtask/src/smoke.rs:254` — ANDROMEDA_PULSE_LOG_LEVEL

### scripts/

- `scripts/agent-run.sh:22,23,35,99` — ANDROMEDA_PULSE_PIDFILE, LOGFILE, RUST_LOG, DATA_DIR_KEEP
- `scripts/agent-run.ps1:15,16,71` — same

### .andromeda/ + docs/

- `.andromeda/architecture.md:4,159-162,163-175,178,184,187,192,194-203,204,220,303,167` — Design Philosophy, ports, TauRPC, broadcast topics, crates, DuckDB tables, paths, env vars, capabilities, Infrastructure narrative, Project Intent, obs-plan §11 cite
- `.andromeda/route.md:52,329` — chunk #57 design plan §Color Palette cite, chunk #67 §Phase 4 line 276
- `.andromeda/security-plan.md:22,94,139,151,447` — stale "no persistent" assertions
- `.andromeda/decisions/pre-d2-drain-spike.md` — Phase A spike decision (chunk #69)
- `.andromeda/state.yaml` — full state at session 101
- `.andromeda/context/api-surface.md:5+` — METADATA narrative
- `.andromeda/context/dependency-tree.md:5+` — METADATA narrative
- `docs/v0_2_0/pulse-capability-spec.md:5,789,706,716,168,188,208,214,232-250,278-308,376,382,388,534-648,602,612,622,632,642,706-714` — Reference docs; mini-route remap clause; P-XXX language citations
- `docs/v0_2_0/pulse-distillation-architecture.md:5,130,150,180,202,248,307,419,441,521,569,678,759,980-995,999,1133` — Reference docs; layer references; stale TODO; appendix lines
- `docs/v0_2_0/pulse-v0_2_0-route.md:99,114,133,152,165,178,193,208,225,240,257,312,329,705-724,712,776,754` — chunk header line numbers; capability-to-chunk mapping table; rename changelog; v1 migration table
- `docs/andromeda-improvements.md` — 14 proposals (P1-P14)
- `.claude/rules/testing.md` §Session Additions + §Pending coverage triggers
- `.claude/rules/security.md` §Session Additions
- `.claude/session-handoff.md` — session 101 close state
- `.claude/docs/session-learnings.md` — 2026-05-16 trait-in-lower-crate pattern; 2026-05-18 free-fn map_err pattern; 2026-05-19 multi-trait single-Arc + Settings scaled-integer + bindings.ts regen patterns

---

## Appendix B — Capability ↔ chunk mapping table

Complete spec inventory P-001..P-060 with status per Dim 2 analysis.

| P-XXX | Title | Category | Attributed chunk | Status | Severity | Evidence |
|---|---|---|---|---|---|---|
| P-001 | Receiver Lifecycle State | Connection awareness | #59 | PARTIAL | HIGH | `connection.rs:39-44` thresholds wrong |
| P-002 | Last-Span-Ago Tracking | Connection awareness | #59 | LIVE | — | `connection.rs:195-201` |
| P-003 | Receiver Failure Surface | Connection awareness | #59 | PARTIAL | HIGH | `connection.rs:264-282` no panic path |
| P-004 | Orthogonal Health Domains | Connection awareness | #59 | LIVE | — | severity decoupled from app-health |
| P-005 | Span Status Error Detection | Hard signal | (orphan) | ORPHAN | HIGH | no candidate emission surface |
| P-006 | Exception Event Capture | Hard signal | #65 | PARTIAL | HIGH | `appender.rs:332-333` raw exception.message |
| P-007 | High-Severity Log Capture | Hard signal | #69 (claim) | ORPHAN (mis-attr) | HIGH | Drain ≠ SeverityNumber≥17 |
| P-008 | Root-Span Error Scope | Hard signal | (orphan) | ORPHAN | HIGH | no root-span detector |
| P-009 | Per-Service Error Rate Baseline | Statistical anomaly | #61 | LIVE (PARTIAL persistence per Dim 1) | HIGH (wrong mechanism) | `baseline/mod.rs` EWMA; flat-file persist violates "to corpus" |
| P-010 | Error Rate Spike Detection | Statistical anomaly | #62 | PARTIAL | HIGH | `evaluate.rs:30-64` fixed constant |
| P-011 | Per-Operation Latency Baseline | Statistical anomaly | #61 | PARTIAL | HIGH | `baseline/mod.rs:368-375` operation hashed |
| P-012 | Latency Regression Detection | Statistical anomaly | #62 | PARTIAL | HIGH | `thresholds.rs:36` p95 + fixed const |
| P-013 | Service Activity Floor Learning | Statistical anomaly | #64 | PARTIAL | HIGH | flat-file + 1h staleness gate |
| P-014 | Service Went Silent Detection | Statistical anomaly | #64 | PARTIAL | HIGH | no 30s floor |
| P-015 | Restart Event Detection | Pattern recognition | #63 | LIVE | — | `pattern/detector.rs` |
| P-016 | Restart-Window Suppression | Pattern recognition | #63 | LIVE | — | `suppression.rs:130-159` |
| P-017 | Exception Fingerprinting | Pattern recognition | #66 | LIVE | — | `buffer/fingerprint.rs:79-95` |
| P-018 | Retry Storm Detection | Pattern recognition | #66 | LIVE | — | `pattern/storm.rs` |
| P-019 | Three-Tier Severity Model | Severity calibration | #60+#62 | PARTIAL | HIGH | contract types only; ZERO observable surfaces |
| P-020 | Model-Driven Severity Decision | Severity calibration | #74+ | DEFERRED | LOW | Phase 7 |
| P-021 | Algorithmic Attention Cues | Severity calibration | #62 | LIVE | — | `cue/emitter.rs` |
| P-022 | Auto-Resolution + Lifecycle | Severity calibration | #70 | DEFERRED | LOW | — |
| P-023 | Acknowledge Cool-Down | Severity calibration | #70 | DEFERRED | LOW | — |
| P-024 | Widget Ambient Surface | Three-Surface Communication | #80 | PARTIAL | HIGH | `FooterBand.tsx` violates "no numerical readouts" |
| P-025 | Halo Hue Encoding | Three-Surface Communication | #81 | PARTIAL | HIGH | errorRate=0 hardcoded |
| P-026 | Halo Breathing Encoding | Three-Surface Communication | #81 | UNVERIFIED | MEDIUM | WGSL inspection needed |
| P-027 | Service Constellation Auto-Discovery | Three-Surface Communication | #67 | PARTIAL | HIGH | `lifecycle/state_machine.rs:38` CorpusRestore stub |
| P-028 | Findings Counter | Three-Surface Communication | #78 | DEFERRED | LOW | — |
| P-029 | Findings Dropdown | Three-Surface Communication | #78 | DEFERRED | LOW | — |
| P-030 | No Interrupting Notifications | Three-Surface Communication | (implicit) | LIVE (by absence) | LOW | no notifications wired |
| P-031–P-036 | Diagnostic Quality / Report sections | Diagnostic Quality | #79 | DEFERRED | LOW | Phase 7+ |
| P-037 | In-App Report Surface | Diagnostic Quality | #79 | DEFERRED | LOW | — |
| P-038 | Copy to Clipboard | Diagnostic Quality | partial #44 | PARTIAL | HIGH | Snapshot ≠ Report char-for-char |
| P-039 | MCP Delivery When Configured | Diagnostic Quality | partial #49 | PARTIAL | HIGH | `tools.rs:26` wrong tool surface |
| P-040 | MCP Independence | Diagnostic Quality | (double-gate) | LIVE | — | `mcp_router.rs:168,271` |
| P-041 | Persistent Incident Corpus | Memory & Learning | partial #68 | PARTIAL | HIGH | scaffold only; no incident writes; no 30-day retention |
| P-042 | Cross-Session Continuity | Memory & Learning | #70 | DEFERRED | LOW | — |
| P-043 | Project-Scoped Memory | Memory & Learning | #70 | DEFERRED | LOW | — |
| P-044 | Retrieval-Augmented Interpretation | Memory & Learning | #73 | DEFERRED | LOW | — |
| P-045 | Counter Derivation from Corpus | Memory & Learning | #70 | DEFERRED | LOW | — |
| P-046 | Export for Community Training | Memory & Learning | #85 | DEFERRED | LOW | — |
| P-047 | PII Scrubbing at Ingestion | Privacy & Security | partial #68 | PARTIAL | HIGH | scrubber consumed at 1 site only |
| P-048 | No Raw OTLP Attribute Values Stored | Privacy & Security | partial #68 | PARTIAL | HIGH | raw values in DuckDB ring buffer + span_events |
| P-049 | Encryption at Rest | Privacy & Security | #68 | PARTIAL | HIGH | no passphrase fallback |
| P-050 | Cross-Project Sharing Opt-In | Privacy & Security | (implicit) | LIVE | — | no outbound net |
| P-051 | Transparent Storage | Privacy & Security | partial #68 | PARTIAL | HIGH | delete entirely absent |
| P-052–P-056 | Cadence config / fallback / hardware / hot reload / prospective | Pipeline Operations | (Phase 7+) | DEFERRED | LOW | — |
| P-057 | Dual-Condition Suppression Bypass | Pipeline Operations | #63 | LIVE | — | `cue/classify.rs:40-57` + `suppression.rs:130-159` |
| P-058 | Pipeline Self-Observability | Pipeline Operations | partial #69 | PARTIAL | HIGH | 1 of ~30 metrics; no Diagnostics view |
| P-059 | Active-Incident Interpretation Continuity | Pipeline Operations | #77 | DEFERRED | LOW | — |
| P-060 | Tiered Triggering Priority | Pipeline Operations | #72 | DEFERRED | LOW | — |

**Summary counts:**
- LIVE: 9 (P-002, P-004, P-015, P-016, P-017, P-018, P-021, P-040, P-050) + P-009 + P-030 = depending on Dim 1 stance
- PARTIAL HIGH: 20 (P-001, P-003, P-006, P-010, P-011, P-012, P-013, P-014, P-019, P-024, P-025, P-027, P-038, P-039, P-041, P-047, P-048, P-049, P-051, P-058)
- DEFERRED: 27 (P-020, P-022, P-023, P-026, P-028, P-029, P-031-P-037, P-042-P-046, P-052-P-056, P-059, P-060)
- ORPHAN: 3 (P-005, P-007, P-008)

---

## Appendix C — Architecture registry ↔ code diff table

Per-category drift summary.

### Workspace crates

- 14 declared = 14 in code ✓ (with `triage-experimental` standalone-workspace gitignored at parent level)

### TauRPC procedures

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| Top-level envelope (5) | YES | YES | none |
| `traces.query` / `metrics.query` / `logs.query` | YES (wildcards) | YES | none |
| `streams.subscribe_{spans,metrics,logs}` (3) | YES (Type 6) | YES | none |
| `telemetry.frontend.record_frame_ms` | YES (Type 6) | YES | none |
| `snapshot.generate` | YES | YES | none |
| `snapshot.list_recent` | YES | **NO** | registry-stale forward-promise (MEDIUM) |
| `snapshot.copy_to_clipboard` | YES | **NO** | registry-stale forward-promise (MEDIUM) |
| `plugins.list/reload/invoke` (3) | YES | YES | none |
| `mcp.status/start/stop` (3) | YES | YES | none |
| `workspace.detect` | YES | YES | none |
| `workspace.list` | YES | **NO** | registry-stale forward-promise (MEDIUM) |
| `connection.current_state` | YES (Type 6) | YES | none |
| `services.list_with_states` | YES (Type 6) | YES | none |
| `storage.inspect/path` (2) | YES (Type 6) | YES | none |
| `diagnostics.template_distribution` | YES (Type 6) | YES | none |

### Broadcast topics

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| `pulse://stream/spans/metrics/logs` (3) | YES | YES | none |
| `pulse://stream/snapshot-progress` | YES | YES (literal string emit) | none (could be tightened with const) |
| `pulse://stream/plugin-events` | YES | **NO** (doc refs only; no runtime emitter) | registry-stale forward-promise + [scope-adjacent — v0.1.0] (MEDIUM) |
| `pulse://stream/connection-state` | YES (Type 6) | YES | none |
| `pulse://stream/attention-cues` | YES (Type 6) | YES | none |
| `pulse://stream/restart-events` | YES (Type 6) | YES | none |
| `pulse://stream/service-lifecycle` | YES (Type 6) | YES | none |

### DuckDB tables

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| spans / span_events / span_links / metrics_points / log_records / resources / instrumentation_scopes (7) | YES | YES | none |
| **`log_templates`** | **NO** | YES (RESERVED_TABLES at `schema.rs:18`) | **code-not-acknowledged — chunk #69 Step 32 incomplete (HIGH)** |

### Corpus SQLite tables (NEW; not in arch §Occupied Resources DuckDB section)

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| **`baseline_state`, `service_registry`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive` (6)** | **NO** (only file path acknowledged) | YES (`crates/corpus/src/schema.rs:26-33`) | **code-not-acknowledged — new sub-section needed (HIGH)** |

### Tauri capability identifiers

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| 6 identifiers | YES (with `pulse:` prefix) | YES (without prefix) | cosmetic mismatch (LOW) |

### Environment variables

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| `ANDROMEDA_PULSE_DATA_DIR`, `OTLP_GRPC_PORT`, `OTLP_HTTP_PORT`, `RETENTION_SECONDS`, `LOG_LEVEL`, `MCP_ENABLED`, `PLUGIN_DIR` (7) | YES | YES | none |
| `RUST_LOG` | YES (fallback) | indirect via EnvFilter | none |
| `ANDROMEDA_PULSE_CONFIG_PATH` | YES | **NO** | registry-stale forward-promise (MEDIUM) |
| `ANDROMEDA_PULSE_PIDFILE/LOGFILE/DATA_DIR_KEEP` | NO | YES (scripts only) | code-not-acknowledged harness-only (LOW) |

### Filesystem subpaths

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| `config.toml`, `plugins/`, `snapshots/`, `logs/` (4) | YES | YES | none |
| `corpus/corpus.db` | YES (Type 6) | YES | none |
| **`triage/baseline-corpus.bin`** | **NO** | YES (`triage/src/baseline/corpus.rs:42`) | **code-not-acknowledged (HIGH)** |
| `run/andromeda-pulse.pid` | NO | YES (`pulse-app/src/main.rs:188`) | code-not-acknowledged [scope-adjacent — v0.1.0] (MEDIUM) |

### Network ports

| Item | In arch.md | In code | Drift |
|---|---|---|---|
| `:4317` OTLP gRPC, `:4318` OTLP HTTP | YES | YES | none |

---

**End of audit.**

This document is investigation only. No chunks registered, no code modified, no remediation authored. User will review findings and register consolidation chunks based on Section 6 groupings, applying own ordering decisions.
