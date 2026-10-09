# Intent — acknowledge chunk #86 diagnostics.retry_interpretation TauRPC procedure

**Slug:** `acknowledge-chunk-86-diagnostics-retry-interpretation`

**Invoked:** /andromeda-evolve --allow-arch-registry

**User intent (verbatim — from session continuity, session 150 wrap-session handoff Next Recommended Action item 1):**

> 1. **`/andromeda-evolve --allow-arch-registry`** к close D3 carryover. This is а Type 6 single-coordinated single-item amendment к acknowledge `diagnostics.retry_interpretation` TauRPC procedure в arch.md §Occupied Resources Tauri IPC routes. Mirrors chunk #78/#80/#81/#82 Type 6 precedent (single-procedure addition; no broadcast topic delta; no env var delta). Expected к fire а Type 6 single-cycle wrap pattern (Active → Propagated → Archived в single session, mirroring 7+ prior precedents).

**Grounding from state.yaml.drift_warnings (session 150 wrap):**

> D3 — arch.md §Occupied Resources Tauri IPC routes does NOT contain `diagnostics.retry_interpretation` TauRPC procedure (chunk #86 added the procedure; arch grep returns 0 matches). EXPECTED post-impl Type 6 follow-up drift.

**Grounding from chunk #86 implementation (commit 4489ae3, session 150):**

- `pulse-app/src/diagnostics_router.rs:60-89` declares `RetryInterpretationPayload` + extends `DiagnosticsApi` trait с `retry_interpretation()` method (returns `Result<RetryInterpretationPayload, AppError>`)
- `pulse-app/src/diagnostics_router.rs:107-155` implements the resolver
- `xtask/src/main.rs::EXPECTED_PROCEDURES` includes `"diagnostics.retry_interpretation"` (line 668)
- `pulse-app/capabilities/default.json` description prose extended to mention the new procedure
- `pulse-app/src/main.rs` boot wiring threads `Arc<dyn DegradedModeStatus>` к the diagnostics router constructor

**Phase 1c clarifying questions:** none asked. Intent unambiguous from session continuity + explicit handoff next-step recommendation + precedent precedence (chunks #78/#80/#81/#82 follow same pattern).

**Phase 1b fast-pattern result:** Refuse 1 keywords matched (arch.md body) BUT `--allow-arch-registry` flag set → Phase 1b proceeds к Phase 1c per refuse-taxonomy.md §Refuse 1 Exception. Phase 2 deep classification + Check 7 verification will validate registry-section-only scope.
