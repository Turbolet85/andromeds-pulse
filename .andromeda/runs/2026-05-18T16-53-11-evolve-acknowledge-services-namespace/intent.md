# Intent — acknowledge-services-namespace

_Captured by /andromeda-evolve --allow-arch-registry at 2026-05-18T16-53-11Z._

## User intent (verbatim from session-handoff "Next Recommended Action")

Author the two queued post-merge `/andromeda-evolve --allow-arch-registry`
amendments for arch.md §Architecture Registry Updates:

1. `services.list_with_states` TauRPC procedure → §Occupied Resources Tauri IPC routes
2. `pulse://stream/service-lifecycle` broadcast topic → §Occupied Resources Tauri IPC events (broadcast channels)

These follow chunks #59/#62/#63 dual-amendment precedent — a separate evolve
cycle after the implementation wrap, mirroring the META-cycle pattern.

Per session-handoff: "Run `/andromeda-evolve --allow-arch-registry` twice
(one per resource) OR in a single dual-cascade session per session 80
precedent."

## Strategy chosen

**Single coordinated marker** covering both registry additions in one amendment_id.
Mirrors chunk #59 (`acknowledge-connection-namespace`) which bundled
`connection.current_state` TauRPC + `pulse://stream/connection-state` broadcast
into a single Type 6 marker. Both additions originate from the same chunk
implementation (chunk #67) and target the same arch.md section (§Occupied
Resources); single-marker strategy keeps the audit trail aligned with chunk
provenance.

## Code evidence (verified at Phase 1)

- `services.list_with_states` — `pulse-app/src/services_router.rs:61`
  (`ServicesApiImpl::list_with_states` resolver; trait def at line 34;
  tracing target `services.list_with_states.request` at line 74)
- `pulse://stream/service-lifecycle` — `crates/triage/src/lifecycle/broadcast.rs:16`
  (`pub const STREAM_NAME_SERVICE_LIFECYCLE: &str = "pulse://stream/service-lifecycle"`)

## Final slug

`acknowledge-services-namespace`
