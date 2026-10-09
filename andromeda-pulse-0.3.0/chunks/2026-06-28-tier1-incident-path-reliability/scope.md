# Scope — Tier1 incident-path reliability

**Marker:** `2026-06-28-tier1-incident-path-reliability`
**Capability:** P-074 · intent F13b
**Version:** andromeda-pulse-0.3.0 · Epoch 1 — Foundation: AI-debug spine
**Status at promotion:** pending

## One-line
Make a sustained identical-fingerprint hard-signal storm yield **exactly ONE incident, reproducibly**,
by fixing the real defect: the triggering cue is never threaded cadence→digest, so the
digest→L4→incident chain returns early and no incident is ever created.

## Premise correction (research-corrects-intent — recorded as a conscious amendment)
The intent (F13b) and the original matrix P-074 framed the fix as *"coalesce identical hard-signals
into one digest + an elastic queue with heartbeat ticks; the TIER1_QUEUE_CAP=3 + ~4s L4 latency drops
~75% of storm digests."* Phase-3 research **falsified that mechanism**:

1. **The real bug is an un-threaded cue.** `pulse-app/src/digest_runtime.rs::spawn_cadence_subscriber`
   calls `assemble(mode, None, …)` — the triggering cue is dropped (a chunk #81 "deferred к future
   chunk" TODO). So a Tier1 cadence storm digest has `attention_cues == []`, and
   `create_incident_from_l4_output` (`inference_runtime.rs:648-654`) hits `else { return }` → **zero
   incidents from a storm**, regardless of any queue behavior. This is THE defect.
2. **The `LwwQueue` is dead on the L4 path.** `drain_all` has zero production callers (code-graph:
   refs only in `#[cfg(test)]`); the real L4 feed is `DigestBroadcast`. `TIER1_QUEUE_CAP=3` only
   gates a drop-*metric*, not L4 delivery. There is no elastic queue to make elastic.
3. **Coalescing to exactly-one already exists** (to be CONFIRMED, not rebuilt): the storm detector
   one-shots ~2 cues per fingerprint/window (`pattern/storm.rs`), and `create_incident_from_l4_output`
   dedups on `(kind, scope, scope_id)` (`inference_runtime.rs:661-682`), processed serially by the
   single-consumer L4 recv loop.

The OUTCOME the intent wanted (one reliable incident under a storm) is unchanged; only the falsified
*mechanism* (elastic queue / TIER1_QUEUE_CAP / "drop 75%") is dropped. This scope + the matrix P-074
entry are corrected to reality (per the user dialogue at /phase P4; the working-route line + intent.md
are left as the human-authored historical source).

## What this chunk builds
1. **Thread the triggering cue cadence→digest (the fix).** Carry the cue identity (kind + scope +
   scope_id + priority_tier) from the cadence trigger through to the assembled digest's
   `attention_cues`, so `create_incident_from_l4_output` derives a real `(kind, scope, scope_id)`
   identity and actually creates the incident. Lean mechanism: extend `CadenceEvent` to carry the cue;
   `spawn_cadence_subscriber` rebuilds it and calls `assemble(mode, Some(&cue), …)`.
2. **Confirm — and only-if-needed complete — dedup to exactly ONE.** Verify a sustained storm
   coalesces to exactly one incident through the existing storm one-shot + incident dedup; if the
   acceptance test reveals a gap (e.g. Suggested-Tier2 + Autonomous-Tier1 producing two incidents, or
   any ordering edge), close THAT gap — still far cheaper than a queue.
3. **Aggregate-only observability** for the threaded-cue path as warranted (no per-service identifiers;
   reuse the `metric.pipeline.l3.*` family). Minimal — only what the behavior change needs.

## What this chunk does NOT build (premise-correction boundaries)
- **No elastic queue. No heartbeat-drain. No reviving `LwwQueue`/`drain_all`.** The queue is a dead
  side-channel; this chunk does not make it load-bearing. (Flag the Tier1-`LwwQueue` path as a
  REMOVAL candidate for a future cleanup chunk — a one-line note in the wrap report; do not action
  removal here unless trivially safe.)
- No change to `TIER1_QUEUE_CAP` framing as a delivery mechanism (it isn't one).
- Real 3B-model judgment quality (intent §5 non-goal — separate deeper problem).
- The deterministic-L4 mode itself (P-073, complete — the acceptance test RUNS under it for
  reproducibility without GPU/model).
- Investigate actions (P-072), Conductor e2e closure (P-075), integration UX e2e (P-076).
- Window/shell hygiene, state-honesty, legibility (Epochs 2–3).

## Surfaces / contracts this chunk touches
- `crates/triage/src/cadence/broadcast.rs` — `CadenceEvent` extended to carry the triggering cue
  identity (serde + Clone + PII negative-canary test).
- `crates/triage/src/cadence/coordinator.rs` — populate the cue into the emitted `CadenceEvent`.
- `pulse-app/src/digest_runtime.rs` — `spawn_cadence_subscriber` rebuilds the cue and passes
  `Some(&cue)` to `assemble`.
- `crates/triage/src/digest/assembler.rs` — `attention_cues` already builds from `triggering_cue` when
  `Some`; add coalesce/cue-threaded obs only if warranted.
- `pulse-app/src/inference_runtime.rs` — confirm `create_incident_from_l4_output` dedup; close a gap
  only if the test exposes one.
- `pulse-app/src/observability.rs` — allowlist any new `metric.pipeline.l3.*` target (only if added).

## Acceptance anchor (corrected — matches verification-matrix#P-074 as amended)
A sustained identical-fingerprint storm (≥100 events) drives the cue → cadence → digest →
**deterministic-L4 (P-073)** path to **exactly ONE incident, reproducibly** — not zero (the cue bug),
not many (a dedup gap). Verified by an `integration` test.

## Open questions for planning (now resolved)
- ~~Elastic-queue interpretation~~ → RESOLVED (user dialogue): no queue; fix the cue; amend the premise.
- ~~Coalescing window/key~~ → existing storm detector + incident dedup; confirm via the acceptance test.
- Cue carrier mechanism → extend `CadenceEvent` (decided; simplest single-carrier path).
