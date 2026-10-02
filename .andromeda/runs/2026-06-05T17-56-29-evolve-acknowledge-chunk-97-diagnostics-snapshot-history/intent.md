# Intent — acknowledge-chunk-97-diagnostics-snapshot-history

_Captured by /andromeda-evolve Phase 1 at 2026-06-05T17:56:29Z._

## Brief intent (Phase 1b)

Acknowledge the chunk #97 `diagnostics.snapshot` + `diagnostics.history`
TauRPC procedures in arch §Occupied Resources Tauri IPC routes (close the
D3 capability-drift surfaced by /andromeda-new-session).

## Deep intent (Phase 1c)

The chunk #97 implementation (session 178) landed two TauRPC procedures on
the existing `diagnostics.*` router — `diagnostics.snapshot` (HYBRID-RENDER
L6 self-observability point-in-time aggregate of Model/Hardware/Pipeline
live state) + `diagnostics.history` (validated stub) — at
`pulse-app/src/diagnostics_router.rs` (snapshot :193/:309, history
:194/:383). The `cargo xtask capability-drift` gate is clean (both
procedures are in EXPECTED_PROCEDURES + bindings.ts), but arch §Occupied
Resources Tauri IPC routes had not yet enumerated them — the expected
chunk-then-amendment Type 6 follow-up that every prior `diagnostics.*`
addition (#69 template_distribution, #86 retry_interpretation, #96
reevaluate_recent_window) has closed via /andromeda-evolve
--allow-arch-registry.

- Slug: `acknowledge-chunk-97-diagnostics-snapshot-history`
- Flag: `--allow-arch-registry` (Type 6 — Architecture registry update)
- Drift closed: D3 (first_observed session 178)
