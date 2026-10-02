# Intent — append-chunk-86-json-parse-fail

**Invocation:** `/andromeda-evolve --allow-route-append to register chunk #86`

**Phase 1b brief intent** (from invocation argument): "register chunk #86" — purely additive route append to route.md §2 Epoch 9 — Foundation v0.2.0.

**Phase 1c deeper intent** (grounded in project context — session 148 wrap recommendation + pulse-v0_2_0-route §Phase 8 §85 detail spec):

Register chunk #86 "JSON parse failure handling + backoff + resolution summary" as the next chunk in Epoch 9 — Foundation v0.2.0 following chunk #85 "Fallback model tier support" (landed session 148).

Per pulse-v0_2_0-route §Phase 8 §85 detail spec:
- Distillation layer: L4 failure handling + L5 surface
- Depends on:
  - chunk #83 (primary tier inference — substrate landed session 142)
  - chunk #84 (LLM runtime swap — landed session 145)
  - chunk #78 (incident lifecycle — landed session 122)
- Capabilities enabled:
  - P-020 graceful degradation (reaches "full" status)
  - P-022 (resolution summary attachment)
  - P-059 (resolution summary generation)
- Crates touched: `crates/interpretation/`, `crates/triage/incident`
- TauRPC delta: +1 procedure `diagnostics.retry_interpretation()` (manual override for backoff)
- Arch registry delta: +1 TauRPC procedure (requires post-impl Type 6 arch-registry amendment per chunks #78/#80/#81/#82 precedent — to be handled in a separate evolve invocation after chunk #86 implementation)

**Insertion target:** terminal position of Epoch 9 — Foundation v0.2.0 (Form 1 — chunk append to existing epoch); after the current terminal chunk #85 "Fallback model tier support".

**Why now:** chunk #85 wrap (session 148) closes substrate for v0.2.0 Phase 8 LLM interpretation work; chunk #86 is the next chunk in the pulse-v0_2_0-route Phase 8 sequence; v0.2.0-route §85 spec is stable and well-defined; pipeline reality (chunks #78/#83/#84/#85 all landed) clears all dependencies. Following the same pattern as previous Type 7 Form 1 chunk appends (chunks #58-#85 each registered via /andromeda-evolve --allow-route-append individually).

**Final slug:** append-chunk-86-json-parse-fail

**Clarifying questions used:** 0 of 4

**Refuse-pattern fast scan:** none matched. Intent is purely additive route append → Refuse 6 narrow exception via --allow-route-append flag.

**Classification expected:** Type 7 — Route registry update (Form 1).
