# Codebase Research — 2026-06-28-tier1-incident-path-reliability

## Scope
- **Depth:** deep · **Reads:** 8 (queue.rs, assembler.rs, digest/mod.rs, digest_runtime.rs, inference_runtime.rs, cadence/coordinator.rs, pattern/storm.rs, cookbook) · **Greps:** 2 · **Code-graph queries:** 3 (LwwQueue refs · drain_all refs · assemble refs)

## The storm → incident pipeline (as it exists today)
1. **Storm detection** — `crates/triage/src/pattern/storm.rs::record_occurrence`: per-fingerprint occurrence tracking. **Already coalesces at the cue level**: 5-in-30s → ONE `Suggested` `RetryStorm` cue; 10-in-30s → ONE `Autonomous` cue; then **one-shot dedup** (`last_emitted`) suppresses further cues until the 60s window fully empties. A 426-identical-fingerprint storm emits only ~2 cues, NOT 426. Cue → `pulse://stream/attention-cues` broadcast.
2. **Cadence coordinator** — `crates/triage/src/cadence/coordinator.rs::start_cadence_coordinator`: subscribes to the cue broadcast; an **Autonomous** cue fires `run_one_coordinator_cycle(Tier1, …, Some(&cue))` → runs Q1-Q7 → emits a `CadenceEvent` on `CadenceEventBroadcast`.
3. **Digest assembler adapter** — `pulse-app/src/digest_runtime.rs::spawn_cadence_subscriber`: subscribes to `CadenceEventBroadcast`; per event calls `assembler.assemble(mode, **None**, …)`.
4. **Assembler** — `crates/triage/src/digest/assembler.rs::assemble`: builds the `Digest`, `push`es to the shared `Arc<Mutex<LwwQueue>>` (Tier1 → `push_tier1`, cap 3), and **unconditionally broadcasts** on `DigestBroadcast` (line 543).
5. **L4 subscriber** — `pulse-app/src/inference_runtime.rs::spawn_l4_inference_subscriber`: subscribes to **`DigestBroadcast`** (NOT the queue); per digest runs L4 → `create_incident_from_l4_output`.

## Two root-cause findings (the intent's mechanical diagnosis is incomplete)

### FINDING 1 (KEYSTONE) — the cue is never threaded cadence→digest, so a storm yields ZERO incidents
`spawn_cadence_subscriber` calls `assemble(mode, None, …)` (`digest_runtime.rs:125`) — the triggering cue is dropped (comment line 7-8: *"triggering cue passthrough deferred к future chunk"*). So a Tier1 cadence digest has `attention_cues == []`. Then `create_incident_from_l4_output` (`inference_runtime.rs:626-654`) derives incident identity from `digest.attention_cues.first()`, and with no cue (and kind ≠ Reflection) hits `else { return; }` — **no incident is ever created from a cadence storm digest.** This is the dominant reason "a real storm produced no incident": even a clean L4 success cannot create the incident. **This must be fixed or the acceptance is unreachable regardless of any queue change.** (This is the 2026-05-30 "data producer assumed-but-inert" pattern: the cue-carrying storm digest is constructed only in `#[cfg(test)]`; the production cadence path passes `None`.)

### FINDING 2 — the `LwwQueue` is NOT the L4 feed; the intent's "TIER1_QUEUE_CAP=3 drops 75%" describes a drop-METRIC, not L4 delivery
Code-graph: `drain_all` has **zero production callers** (refs only at `queue.rs:393,408`, both `#[cfg(test)]`). The `LwwQueue` is push-only in production — `push_tier1` tracks depth + emits `digest.lww.drop` warnings + `metric.pipeline.l3.lww_drop_count_total`, but its contents are never consumed. The real L4 feed is `DigestBroadcast` (bounded `BROADCAST_CAPACITY`). The actual storm-loss path under sustained load + ~4s L4 is **broadcast lag** (`RecvError::Lagged` → skipped digests), not the Tier1 cap. Because the storm detector already coalesces to ~1-2 cues/digests per storm, for a SINGLE-fingerprint storm there is little volume to drop — the dominant defect is FINDING 1, not queue overflow. → "make the queue elastic" needs reinterpretation (see Open question 1).

### Already-present coalescing/dedup (reduces what this chunk must build)
- Cue-level coalescing: storm detector one-shot per fingerprint/window (above).
- Incident-level dedup: `create_incident_from_l4_output` re-emission dedup on `(kind, scope, scope_id)` (`inference_runtime.rs:661-682`) → bumps an existing active incident instead of creating a duplicate. So "exactly ONE incident" is largely **already handled** once the cue threads through.

## Graph impact (code-graph)
- **`LwwQueue`** — refs only within `crates/triage` (`contract.rs` re-export, `assembler.rs` construction/use). Leaf-internal; changing its policy has zero cross-crate blast beyond the assembler + the binary adapters.
- **`drain_all`** — zero production callers (test-only) → safe to repurpose/remove/wire-in.
- **`DigestAssembler::assemble`** — production caller is `digest_runtime.rs:124`; threading a real cue is a one-call-site change (plus the `CadenceEvent` carrier).

## Contract shapes (confirmed)
- `CadenceEvent` (`cadence/broadcast.rs:22`) carries only `cue_kind_label`/`cue_priority_label` (`Option<&'static str>`) — **not** the full cue. Threading the cue needs the carrier to hold cue identity (kind + scope + scope_id + priority_tier) or a parallel channel.
- `AttentionCue` (`contract.rs:284`): kind, scope, scope_id, magnitude, absolute_value, persistence_seconds, confidence, priority_tier, suppression_bypassed.
- `DigestCueRef` (`contract.rs:448`): kind, priority_tier, summary, scope, scope_id — exactly what `create_incident_from_l4_output` consumes. `Digest.attention_cues: Vec<DigestCueRef>` built from `triggering_cue` in `assemble` (empty when `None`).
- `DigestLwwMode::Tier1NeverLww` (`contract.rs:411`) + `TIER1_QUEUE_CAP=3` (`queue.rs:28`).

## Patterns to follow
- **Cross-crate state delivery** (CLAUDE.md 2026-05-16/05-23): traits/state in `triage`; binary adapters in `pulse-app`. The cue carrier extension stays in `triage` (`CadenceEvent`), the reconstruction-and-pass stays in `pulse-app/src/digest_runtime.rs`.
- **Aggregate-only obs** (`storm.rs`, `assembler.rs`): bounded counts + enum tags; never `scope_id`/`service_name` in self-observation events. New coalesce/queue metrics reuse the `metric.pipeline.l3.*` family.
- **Heartbeat tick shape** (`storm.rs::start_storm_detector`, `coordinator.rs` 15s interval, `inference_runtime.rs::spawn_l4_queue_depth_heartbeat`): `tokio::time::interval`, skip first tick, `MissedTickBehavior::Skip`, aggregate `value` field.
- **PII negative-canary tests** (`storm.rs` tests, `coordinator.rs::…does_not_leak_pii_canary…`): assert no canary in serialized payloads — required for any new cue-carrying `CadenceEvent` field.
- **pulse-app tests live in `pulse-app/tests/*.rs`** (`[lib] test = false`; CLAUDE.md 2026-05-20).

## Files to modify (best-effort; confirm at /implement)
- `crates/triage/src/cadence/broadcast.rs` — extend `CadenceEvent` to carry the triggering cue identity (full `AttentionCue` or a bounded subset) so the digest adapter can rebuild it. (serde + Clone + PII test.)
- `crates/triage/src/cadence/coordinator.rs` — populate the cue into the emitted `CadenceEvent`.
- `pulse-app/src/digest_runtime.rs` — `spawn_cadence_subscriber`: rebuild the cue from the event and call `assemble(mode, Some(&cue), …)`.
- `crates/triage/src/digest/queue.rs` — elastic Tier1 policy + (if chosen) a heartbeat-drain consumer; coalesce-in-place option.
- `crates/triage/src/digest/assembler.rs` — coalesce-count observability; (cue→attention_cues already works when `Some`).
- `pulse-app/src/inference_runtime.rs` — if the queue becomes the load-bearing feed, wire the heartbeat-drain → L4; else ensure broadcast delivery reliability.
- `pulse-app/src/observability.rs` — allowlist any new `metric.pipeline.l3.*` / `*.tick` target.

## New files to create
- `pulse-app/tests/integration_tier1_storm_one_incident.rs` — acceptance: ≥100 identical-fingerprint hard signals (via storm detector → cue → cadence → digest → deterministic-L4 P-073) yield exactly ONE incident with no dropped-by-cap loss. (deterministic-L4 makes it reproducible without GPU/model.)

## Open questions (resolve at P4 / surface at P5)
1. **"Elastic queue" interpretation** — the `LwwQueue` is not the L4 feed (`drain_all` dead). Three shapes: **(A)** fix cue-passthrough + rely on existing coalescing/dedup + ensure broadcast delivery (minimal, hits the root cause; the queue stays a metrics side-channel); **(B)** make the Tier1 queue the load-bearing heartbeat-drained L4 feed per the literal intent (largest change; wires `drain_all`→L4); **(C)** cue-passthrough + elastic Tier1 cap + heartbeat-drain alongside the broadcast (middle). All three REQUIRE FINDING 1's cue-passthrough. → AskUserQuestion at P4.
2. **Cue carrier mechanism** — extend `CadenceEvent` with the cue (simplest; one carrier) vs a parallel cue channel into the assembler. Lean: extend `CadenceEvent`.
3. **Acceptance harness** — run under deterministic-L4 (P-073) so "exactly one incident" is reproducible with no GPU/model. (Assumed yes.)
