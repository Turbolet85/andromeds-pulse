# Intent — append-chunk-67-service-registry-lifecycle

_Captured by /andromeda-evolve at 2026-05-17T23:51:55Z._

## User invocation (verbatim)

```
/andromeda-evolve --allow-route-append chunk #68
```

## Phase 1b brief intent

"chunk #68" — the v0.2.0-plan chunk identifier surfaced by the session 88 handoff Next Recommended Action as an unblocked alternative to chunk #67 (Drain Rust, blocked on Pre-D2 spike validation).

## Phase 1c derived intent (autonomous mode — no user dialogue)

Register the capability described in `docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 4 line 276 (v0.2.0-plan chunk #68 "Service registry + lifecycle state machine") at the next available route.md §2 position — which is route position #67 (since v0.2.0-plan chunk #67 "Drain Rust" is blocked and unregistered).

The user passed `chunk #68` referring to v0.2.0-plan numbering. The skill registers at route position #67 because:

1. Route.md §2 chunks are positionally numbered (chunk N = position N in the flat ↓-list).
2. Current route.md §2 has 66 chunks; next available position is #67.
3. v0.2.0-plan chunk #67 (Drain Rust) is blocked on Pre-D2 spike validation per pulse-v0_2_0-route ordering note; user explicitly choosing to skip it.
4. Chunk text preserves traceability to v0.2.0-plan via "detail in pulse-v0_2_0-route §68" suffix.

Route.md and v0.2.0-plan numbering diverge from this amendment onward (route position #67 = v0.2.0-plan §68; v0.2.0-plan §67 remains unregistered pending Drain spike).

## Classification

- **Type:** Type 7 — Route registry update (Form 1, chunk append to existing epoch)
- **Flag:** `--allow-route-append`

## Source-plan citation

`docs/v0_2_0/pulse-v0_2_0-route.md` §Phase 4 lines 274-287:

```
## Phase 4 — Service identity lifecycle

### #68 — Service registry + lifecycle state machine

- **Depends on:** #61 (baseline trackers — feed activity state), #63 (restart detector — feeds transitions)
- **Capabilities enabled:** P-027 (Service Constellation Auto-Discovery — formal lifecycle), prerequisite for P-014, P-068 lifecycle-aware behavior
- **Distillation layer:** L1b (state machine), L5 (constellation surface)
- **Crates touched:** `crates/triage/lifecycle`, `crates/triage/baseline` (state queries)
- **TauRPC delta:** +1 procedure `services.list_with_states()`
- **Broadcast topics delta:** +1 `pulse://stream/service-lifecycle` (state transitions)
- **Workspace deps delta:** none
- **Arch registry delta:** +1 broadcast topic, +1 TauRPC procedure, new schema table `service_registry`
- **Specialist plan touches:** arch, test-plan (state transitions: Unknown→Bootstrapping→Active→Quiet→Silent→Dormant→Archived; corpus history lookup on Archived→Active transition; manual override paths), obs-plan (`pipeline.l1b.tracked_services_total`, lifecycle state distribution metric)
- **Summary:** Service state machine per dist-arch v3 §Service Identity Lifecycle. Seven states with configurable boundaries via `[triage.lifecycle.dormant_after_secs]` (default 3600) and `[triage.lifecycle.archived_after_secs]` (default 86400). Edge case handling for ring buffer eviction (corpus lookup on long-silent services returning). Manual override in Settings → Services to mark Dormant/Archived.
```

## Pipeline context (from session 88 handoff)

Session 88 wrap completed clean (chunk #66 implementation). Handoff Next Recommended Action surfaced chunk #67 (Drain Rust) as the default next, but noted it is BLOCKED on Pre-D2 spike validation. Alternatives listed: chunk #68 (Service registry), chunk #69 (Corpus SQLite scaffold).

User selected chunk #68 via this evolve invocation.

Dependencies for chunk #68:
- #61 (baseline trackers + corpus persistence) — landed session 78 (commit history)
- #63 (restart event detector + dual-condition bypass) — landed session 80 (commit history)

Both deps complete; chunk ready for /andromeda-phase planning after this route registration.
