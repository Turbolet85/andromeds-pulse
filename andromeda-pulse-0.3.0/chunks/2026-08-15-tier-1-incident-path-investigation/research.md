# Codebase Research — 2026-08-15-tier-1-incident-path-investigation

## Scope
- **Depth:** deep (mature codebase; the chunk's whole subject is a live seam) · **Reads:** 8 · **Globs/Greps:** 11 · **Code-graph queries:** 2

## Files inspected
- `pulse-app/src/digest_runtime.rs` (full) — the ranked suspect's file. `resolve_workspace_for_incidents` (l.109-122) returns `(key, context)` from ONE call so filter-key and stamped-workspace cannot diverge by construction; `spawn_cadence_subscriber` (l.129-177) threads `trigger.triggering_cue.as_ref()` into `assemble(...)` — the P-074 fix is PRESENT at HEAD.
- `crates/triage/src/cadence/coordinator.rs` (l.330-450) — the trigger-send sites. The Tier-1 arm is `cue_rx.recv()` gated `matches!(cue.priority_tier, PriorityTier::Autonomous)`.
- `crates/triage/src/pattern/storm.rs` (l.150-400) — `record_occurrence` tier selection + `synthesize_cue` + `observe_and_dispatch_storm`.
- `crates/triage/src/cue/emitter.rs` (l.1-235) — `run_one_emit_cycle` + `emit_cue` (the ONLY writer of `triage.cue.emit`).
- `pulse-app/src/storm_observer.rs` (full) — `StormObserverAdapter`, the buffer→triage bridge.
- `pulse-app/src/inference_runtime.rs` (l.118-180, 620-700) — `spawn_l4_inference_subscriber` + `create_incident_from_l4_output`.
- `pulse-app/src/main.rs` (l.570-650, 725-762) — storm-detector construction with production thresholds; the single production call of `resolve_workspace_for_incidents`.
- `pulse-app/src/observability.rs` (allowlist region) — the §4 muted-leaf state on HEAD.

## Graph impact
- **`resolve_workspace_for_incidents`** — `refs` returned **9 rows**: exactly ONE production site (`pulse-app/src/main.rs:735`) plus 8 in `pulse-app/tests/integration_constellation_severity_workspace_key.rs`. A single production caller that destructures BOTH halves of one return value — so the P-079 byte-equality invariant holds structurally, not by discipline.
- **`DigestTrigger` / `DigestTriggerBroadcast`** — `refs` returned **74 rows** across `crates/triage/src/cadence/{broadcast,coordinator,mod}.rs`, `crates/triage/src/contract.rs`, `pulse-app/src/digest_runtime.rs` (l.37/129/134/138/140/144/150/170), `pulse-app/src/main.rs` (l.26/482). Four trigger-send sites in the coordinator (l.345 Tier3, l.365 Reflection, l.400 Tier1-from-cue, l.438 Tier2-from-cadence-cue). The carrier is internal (no `pulse://stream/*` registration) — consistent with the 2026-06-28 PII-free-topic rule; **not** an unregistered IPC event.

## Patterns detected
- **Two disjoint cue producers, one shared broadcast** (`crates/triage/src/pattern/storm.rs:395` vs `crates/triage/src/cue/emitter.rs:189`): the storm detector dispatches straight onto `AttentionCueBroadcast` from the buffer hot path, while `run_one_emit_cycle` derives cues from `BaselineState` only (`evaluate_thresholds` + `evaluate_service_went_silent`, emitter.rs:57-59). **The storm detector is not a source in the emit cycle.**
- **The storm producer logs a different target family** (`storm.rs:341-365`): `triage.pattern.storm.detected` / `triage.pattern.storm.emit` / `metric.storm.detected_count` — never `triage.cue.emit`, which only `emit_cue` writes (`crates/triage/src/cue/mod.rs:44`).
- **Tier selection is threshold-arithmetic** (`storm.rs:246-288`): `should_emit_autonomous = count >= autonomous_threshold && last_emitted ∈ {None, Suggested, Curious}`; `should_emit_suggested = count >= suggested_threshold && last_emitted.is_none()`. Both increment `storms_detected_total`.
- **Production thresholds** (`crates/triage/src/pattern/storm.rs:63-78`, wired at `pulse-app/src/main.rs:592-624`): window 60s · detection sub-window **30s** · `DEFAULT_SUGGESTED_THRESHOLD = 5` · `DEFAULT_AUTONOMOUS_THRESHOLD = 10`.
- **Incident creation is cue-or-reflection-derived** (`inference_runtime.rs:648-654`): a non-Reflection digest with an empty `attention_cues` returns early — the documented P-074 shape.

## The chain, end to end (each hop cited)

| # | Hop | Site | Gate |
|---|---|---|---|
| 1 | span-event → fingerprint → observer | `pulse-app/src/storm_observer.rs:42` | none (measured healthy 6/6/6) |
| 2 | occurrence → cue | `crates/triage/src/pattern/storm.rs:254-288` | `count >= 10` → Autonomous · `count >= 5` → Suggested |
| 3 | cue → Tier-1 cycle | `crates/triage/src/cadence/coordinator.rs:390` | **`PriorityTier::Autonomous` ONLY** |
| 4 | cycle → DigestTrigger | `coordinator.rs:398-405` | `stats.digest_emitted` |
| 5 | trigger → digest | `pulse-app/src/digest_runtime.rs:148-156` | cue threaded (P-074 present) |
| 6 | digest → L4 | `pulse-app/src/inference_runtime.rs:131-144` | `degraded_mode.is_in_backoff` skip |
| 7 | L4 → incident | `pulse-app/src/inference_runtime.rs:636-654` | `Dismiss` / `Severity::None` / empty `attention_cues` → return |

## Findings against the chunk's two open premises

**Premise 2 (the named FIRST CHECK) — RESOLVED, and it resolves in the caution's favour.**
`triage.cue.emit` has exactly one writer (`emit_cue`), fed only by the BaselineState-derived emit cycle. The storm producer is a *separate* path that broadcasts the cue and logs `triage.pattern.storm.*`. So `triage.cue.emit == 0` is the EXPECTED reading for a storm-only run and proves nothing about whether a cue was carried. The evidence's own conservative phrasing was right. (The `DigestTriggerBroadcast` carrier sits one hop further downstream than suspected — it carries the trigger, not the cue, and is fed by the coordinator, not by the storm detector.)

**Premise 1 (the in-window regression) — a competing, arithmetic explanation now exists for the Conductor arm.**
The arm measured 6 span-events → 6 fingerprints → 6 observer invocations, and `storms_detected_total` 0→**1**. Against thresholds 5 / 10 that is exactly the *Suggested* branch: `count` reached 5-6, cleared `suggested_threshold` (5), never reached `autonomous_threshold` (10), and set `last_emitted = Some(Suggested)` — one detection, tier Suggested. The coordinator's Tier-1 arm accepts **only** Autonomous, so the cue is dropped at hop 3 (`coordinator.rs:407` `Ok(_) => {}`), and zero incidents follows **by design, not by defect**. Under this reading the arm never had enough events to make an incident, and the "regressed within the last two chunks' window" premise is not supported by that run.

This is a hypothesis with an arithmetic fit, not a verdict — it is exactly what the operator's §1 premise-check discriminates, and the discriminator gains power from it: `inject_demo` drives hundreds of same-fingerprint exceptions, comfortably past 10, so it exercises the Autonomous branch the arm never reached.

**A real gap the same reading exposes (independent of the premise-check's outcome).**
The coordinator's `Ok(_)` arm is commented "Non-Autonomous cues (Suggested / Curious) flow through their dedicated channels — ignored here". For BaselineState-derived cues that is true: `emit_cue` forwards `Suggested` cues to `CadenceTriggerChannel` (`emitter.rs:187-190`), which the coordinator's Tier-2 arm consumes. **`observe_and_dispatch_storm` performs no such forward** — it only broadcasts on `AttentionCueBroadcast` (`storm.rs:352`). So a *Suggested storm* has no dedicated channel to flow through; it is dropped outright. Whether that is intended (Tier-1 is deliberately Autonomous-only) or an omission at the storm producer is the judgement the fix turns on, and the Tier-2 arm carries two further gates anyway (`tier2_acceleration_enabled` AND `hw_profile != CpuPrimary`, `coordinator.rs:419-424`).

**Ranked suspect (operator §3) — weakened, not eliminated; still bisected.**
`resolve_workspace_for_incidents` returns filter-key and stamped context from a single call (`digest_runtime.rs:109-122`), and the graph shows ONE production caller destructuring both halves (`main.rs:735`), which then publishes that same key to the sidecar (`main.rs:739`). A stamp/filter divergence would have to come from a *different* writer of `digest.workspace` or of the `incidents.workspace` column, not from this function. Also note the workspace filter sits at hop 7's dedup lookup (`registry.list_active(&digest.workspace)`) — a mismatch there would produce **duplicate** incidents, not zero. Rank it below the threshold-arithmetic reading; still test it at the bisect rather than dismissing it.

## Conventions to follow
- **Exact allowlist leaves, never a prefix key** — `pulse-app/src/observability.rs` already carries `incidents.mark_all_read.request` (l.2082) and `incidents.get_report.request` (l.2099) as sibling exact leaves with no bare `incidents` key; the §4 additions follow that shape (obs-plan §8 `for_target` prefix-fallback trap).
- **Aggregate-only triage/cue fields** — bounded counts + enum labels; never `scope_id` / `service_name` (the 2026-05-17 session-84 convention, upheld by `emit_cue`'s field list at `emitter.rs:194-204`).
- **`pulse-app` probes live in `pulse-app/tests/*.rs`** — `[lib] test = false` makes a src-level `mod tests` compile-but-never-run (test-plan §2; the 2026-05-20 + 2026-06-04 entries). `digest_runtime.rs` and `inference_runtime.rs` are both `pulse-app/src/`.
- **Counter-differencing over gauges** — `storms_detected_total` / `fingerprints_evicted_total` are `AtomicU64` monotonic counters surfaced on `triage.pattern.storm.tick` (`storm.rs:392`); difference them across a run rather than reading a point value.

## New files to create
- `andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/evidence/premise-check.md` — the §1 discriminator run + its query-cited result (either branch is a result).
- A regression-lock test under `pulse-app/tests/` — path decided once the break is located; must FAIL against pre-fix code (test-plan §4 corpus precedent).

## Files to modify
- `pulse-app/src/observability.rs` — three new EXACT allowlist leaves (`incidents.list_active.request` → `item_count`; `triage.incident.persist` → `incident_count`, `persist_kind`; `triage.incident.corpus_restore` → `kind`, `restored_incident_count`) + their resolution unit test. **Verified absent on HEAD** (all three greps returned no key; the `persist_kind` / `item_count` hits at l.1155/1202/1467/1537/3203/3300 belong to unrelated `triage.lifecycle.*` / `services.list_with_states.request` / other leaves).
- **Provisional pending the premise-check** (the located break decides which): `crates/triage/src/pattern/storm.rs` (tier selection / cadence forward), `crates/triage/src/cadence/coordinator.rs` (the Tier-1 Autonomous-only gate at l.390), `pulse-app/src/inference_runtime.rs` (hop-7 gates). No caller-threading is implied by any of these — they are decision-site edits inside existing signatures, so the graph's caller sets do not expand the list.

## Open questions
- Did the corpus-key chunk's two smoke boots inject a storm at all, or only boot the app? If they never drove same-fingerprint exceptions past 10, their zero-incident reading carries the same arithmetic explanation as the Conductor arm and adds no independent evidence of regression. → blocks: **plan-decision** (it decides whether the bisect is even warranted; the §1 premise-check answers it directly).
- Is the Tier-1 Autonomous-only gate (`coordinator.rs:390`) the intended contract, with the Suggested-storm drop a producer-side omission — or should a Suggested storm reach Tier-2 via `CadenceTriggerChannel` as BaselineState-derived Suggested cues do? → blocks: **implementation-scope** (it selects the fix site; the premise-check + bisect inform it, and the Tier-2 arm's own two gates make the answer non-obvious).
