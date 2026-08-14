# Evolve Intent — append-chunk-81-digest-assembler-lww-queue

_Captured by /andromeda-evolve at 2026-05-23T15:54:23Z._

## Invocation

`/andromeda-evolve --allow-route-append`

## Phase 1b — Brief intent (auto-mode interpretation)

User invoked /andromeda-evolve with the `--allow-route-append` flag. Per
sessions 127-128 handoff and the dashboard rendered at session-start of
session 130, the unambiguous mandate is to register chunk #81 — the
next Phase 7 chunk per `docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 7 §81.

Brief intent (auto-mode derivation):

> Register chunk #81 "Digest assembler + LWW queue + active-incident
> exception" in `.andromeda/route.md` §2 Epoch 9 — Foundation v0.2.0.

Slug: `append-chunk-81-digest-assembler-lww-queue`

## Phase 1c — Deep intent (sourced from pulse-v0_2_0-route.md §Phase 7 §81)

- **Plan target:** `.andromeda/route.md` only (Form 1 — chunk append to
  existing Epoch 9)
- **What to add:** new chunk at position 81 with compact text per route
  format invariant (≤25 words, single line; detail referenced to source
  doc)
- **Why now:** carry-over from sessions 127-128 (handoff Alternative
  path); chunk #80 (Cadence coordinator) committed 2026-05-23T11:40Z
  (sha=9296fa3) and its Type 6 arch-registry amendment archived at
  session 127. Chunk #81 dependencies all landed: #79 (L1a SQL) +
  #62 (attention cues) + #66 (fingerprints) + #67 (templates) +
  #69 (corpus retrieval) + #78 (active incident state).
- **Chunk source detail** (from pulse-v0_2_0-route.md §Phase 7 §81):
  - **Depends on:** #79, #62, #66, #67, #69, #78 — verified all landed
  - **Capabilities enabled:** P-031 foundation, P-032, P-044, P-059
  - **Distillation layer:** L3 (digest composition)
  - **Crates touched:** `crates/triage/digest`,
    `crates/workspace-detector` (consumer)
  - **TauRPC delta:** none yet
  - **Broadcast topics delta:** +1 `pulse://stream/digests` (LWW queue
    for L4); corpus appends (all digests, no LWW)
  - **Workspace deps delta:** +`tokenizers` (Hugging Face tokenizer for
    token counting matched to active model set by chunk #82)
  - **Arch registry delta:** +1 broadcast topic, +1 workspace dep
    (registered via Type 6 amendment AFTER implement lands; NOT part
    of this Type 7 route-append)

## Chunk text (compact form per Check 8.5)

```
Digest assembler + LWW queue + active-incident exception — compose L3 digest from L1a/L2/corpus; LWW for cadence with active-incident bypass (capabilities P-031/P-032/P-044/P-059; detail in pulse-v0_2_0-route §81)
```

Word count: 24/25.

## Phase 2 — Classification

Type 7 — Route registry update (Form 1 — chunk append to existing epoch).

## Sequencing context

This amendment is the standard Phase 7 chunk-then-amendment cycle:

1. **This session 130 (META):** /andromeda-evolve --allow-route-append
   (register chunk #81) → /andromeda-setup-project --delta (CLAUDE.md
   pointer-table cascade 80→81 chunks) → /andromeda-wrap-session
   (archive amendment, single-cycle close mirroring sessions
   115/118/120/123/125 Type 7 precedents).
2. **Next implementation session:** /andromeda-phase (chunk #81
   plan) → /andromeda-implement → wrap. Chunk #81 implementation will
   introduce +tokenizers workspace dep + `pulse://stream/digests`
   broadcast topic — those trigger a follow-on Type 6
   --allow-arch-registry amendment in a subsequent META session
   (mirroring the chunk #67/#78 Type 6 chunk-then-amendment pattern).
