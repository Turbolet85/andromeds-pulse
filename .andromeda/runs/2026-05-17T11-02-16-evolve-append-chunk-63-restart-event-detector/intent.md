# Intent — append-chunk-63-restart-event-detector

**Captured:** 2026-05-17T11:02:16Z
**Skill invocation:** `/andromeda-evolve --allow-route-append`
**Slug:** append-chunk-63-restart-event-detector

## Phase 1b — Sanity check (one-sentence intent, verbatim)

> add chunk 63

## Phase 1c — Deep intent collection (no clarifying questions needed)

Skill resolved intent from project context:

- **Source spec:** `docs/v0_2_0/pulse-v0_2_0-route.md` Phase 2 "Algorithmic detection layer" §#63 (line 193)
- **Chunk #63 title (source):** "Restart event detector + dual-condition bypass"
- **Source bullets:**
  - Depends on: #60 (triage scaffold) + #62 (cue emitter — bypass overrides its suppression)
  - Capabilities enabled: P-015 (Restart Event Detection) + P-016 (Restart-Window Suppression Surgical) + P-057 (Dual-Condition Suppression Bypass)
  - Distillation layer: L1b (RestartDetector) + L2 (suppression rules in cue emitter)
  - Crates touched: `crates/triage/pattern`, `crates/triage/cue` (suppression logic)
  - TauRPC delta: none yet
  - Broadcast topics delta: +1 `pulse://stream/restart-events`
  - Arch registry delta: +1 broadcast topic (future Type 6 when chunk #63 lands)
  - Specialist plan touches: arch / test-plan (synthetic stream gap → restart detection; dual-condition bypass coverage 8×/3%, 12×/4%, 6×/7%) / obs-plan (`pipeline.l2.magnitude_bypass_triggered_total{reason}`)
- **Placement:** Form 1 — append chunk #63 to existing Epoch 9 — Foundation v0.2.0 (chunk #63 is the natural successor to chunk #62 in pulse-v0_2_0-route Phase 2)

## Resolved scope

- **Affected plan:** `.andromeda/route.md` (Type 7, flag-authorized)
- **Affected sections:** §1 Route Scope Summary (Total chunks mechanical update per Proposal 6), §2 Roadmap (Epoch 9 body append), §3 Decisions Log (new entry)
- **Form:** 1 (append to existing epoch; not new epoch creation)
- **Insertion position:** route_index=63 (immediately after chunk #62 in Epoch 9 body)
- **Chunks renumbered:** none (chunk #63 is terminal-position; no chunks after it to shift)

## Motivation

Route §2 Epoch 9 currently terminates at chunk #62 (committed 2026-05-17 commit `aeb4d7d`). Chunk #63 "Restart event detector + dual-condition bypass" is the next dependency-driven chunk per `pulse-v0_2_0-route.md` Phase 2 — depends on chunk #60 (triage crate scaffold for `crates/triage/pattern` module) + chunk #62 (cue emitter's suppression rules require modification per P-057 dual-condition bypass).

Registering chunk #63 in route §2 brings route into alignment with pipeline reality (the v0.2.0 source plan declares it; route §2 has not yet acknowledged it) so `/andromeda-phase` can plan against it as the next chunk-up.

This is the bundled second Type-flag amendment in this session (alongside Type 6 `--allow-arch-registry` for `pulse://stream/attention-cues` from earlier this same session). Both will land in `spec_amendments.active` and propagate together at the next `/andromeda-setup-project --delta`.

## Anticipated next step after this evolve

`/andromeda-setup-project --delta` to propagate BOTH amendments (Type 6 attention-cues + Type 7 chunk #63):
- Type 6 (registry-only): empty `expected_propagation` → lifecycle progression only.
- Type 7 (chunk #63 append): CLAUDE.md pointer-table line `(9 epochs / 62 chunks)` → `(9 epochs / 63 chunks)` cascade (pre-populated in marker `expected_propagation` per Proposal 5 cascade visibility).

Then `/andromeda-wrap-session` archives both amendments + composes the session-80 wrap commit.

Then `/andromeda-phase` can plan chunk #63 implementation.
