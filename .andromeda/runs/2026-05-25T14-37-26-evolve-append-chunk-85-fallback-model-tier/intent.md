# Intent — append-chunk-85-fallback-model-tier

## Invocation

```
/andromeda-evolve --allow-route-append
      Register chunk #85
```

## Brief intent (Phase 1b)

"Register chunk #85" — verbatim from invocation arguments.

## Resolved intent (Phase 1c, auto-mode + handoff context)

Register chunk #85 "Fallback model tier support" in route.md §2 Epoch 9 —
Foundation v0.2.0 via Type 7 Form 1 (chunk append to existing epoch). Per
session 146 handoff "Next Recommended Action" item 1:

> 1. `/andromeda-evolve --allow-route-append` — register chunk #85 в
>    route.md §2 Epoch 9 + §1 Total chunks 84 → 85 + §3 Decisions Log
>    compact P9 entry. Chunk #85 was originally planned as chunk #84
>    "Fallback model tier support" per pulse-v0_2_0-route §85.

The chunk position number mapping reflects the renumbering documented в
pulse-v0_2_0-route.md (where "Fallback model tier" landed at §84 in the
post-renumber view; in the andromeda project route.md where chunk #84
was assigned к "L4 LLM runtime swap" session 144, this maps к route#85).

## Chunk specification (pulse-v0_2_0-route.md §Phase 8 §84)

- **Title:** Fallback model tier support
- **Depends on:** chunk #83 (prompt scaffolding + JSON schema + primary tier
  inference; registered session 141, substrate landed session 142)
- **Capabilities enabled:** P-053 (Fallback Model Tier — full)
- **Distillation layer:** L4 (interpretation)
- **Crates touched:** crates/interpretation/ (fallback prompt + reduced-
  quality output)
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** none (uses crates/interpretation/ added in #82)
- **Specialist plan touches:** test-plan (fallback-tier scenarios produce
  reduced output; Report displays fallback-tier indicator), security
  (fallback model file integrity equal to primary)
- **Summary:** Reduced-quality prompt and schema for 3-4B class models.
  Single hypothesis instead of ranked list. Up to 2 investigation steps
  instead of 5. Less specific project context grounding (~500-700 token
  system prompt vs primary's ~800-1000). Output JSON includes
  `model_tier: "fallback"` for downstream rendering. CPU inference fully
  supported at 3-8s typical latency.

## Form determination

- **Form 1** — chunk append to existing Epoch 9 — Foundation v0.2.0
- NOT Form 2 — no new epoch creation needed (Epoch 9 actively hosts all
  v0.2.0 chunks #57-#84)

## Insertion position

- **Position:** 85 (1-based route_index)
- **After:** chunk #84 "L4 LLM runtime swap" (last chunk in Epoch 9
  currently)
- **Last completed chunk:** route#84 (commit_sha=13d80e6 — HEAD-reachable)
- **Insertion > last_completed:** 85 > 84 ✓ Check 8.3 PASS
- **In-progress chunks:** none (state.yaml.in_progress = null) → no Check
  8.4 shift confirmation needed

## Classification (predicted)

Type 7 Form 1 — Route registry update (chunk append to existing epoch via
`--allow-route-append` flag exception к Refuse 6). Mirrors precedents from
chunks #58/#59/#60/#61/#62/#63/#64/#65/#66/#67/#68/#69/#70/#71/#72/#73/
#74/#75/#76/#77/#78/#79/#80/#81/#82/#83/#84 (all Form 1 single-cycle
amendments per session-wrap audit).
