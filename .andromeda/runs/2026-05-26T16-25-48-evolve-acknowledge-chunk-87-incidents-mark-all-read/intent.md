# Evolve Intent — acknowledge-chunk-87-incidents-mark-all-read

**Invoked:** 2026-05-26T16:25:48Z
**Flag:** `--allow-arch-registry`
**Slug:** `acknowledge-chunk-87-incidents-mark-all-read`

## Phase 1b brief intent (verbatim user input)

> chunk #87 added `incidents.mark_all_read` TauRPC procedure but procedure NOT yet listed в arch.md §Occupied Resources Tauri IPC routes

## Phase 1c deep dialogue (no follow-up questions needed — 0/4 used)

Context fully resolved from /andromeda-new-session preflight + Phase 0 pipeline state read:

- **Source-of-truth event:** chunk #87 "Findings counter + dropdown" implementation (commit `6fbcc2a`, session 153) extended `IncidentsApiImpl` with a 4th TauRPC procedure `incidents.mark_all_read` (the Findings counter dropdown footer's bulk-acknowledge button). The quadruple binding is closed end-to-end (router registration + capabilities/default.json description + xtask EXPECTED_PROCEDURES + emit_taurpc_bindings test), `cargo xtask capability-drift` clean, bindings.ts regenerated.
- **Drift signal:** D3 fired в session 153 wrap (per `state.yaml.drift_warnings` first_observed_session_count=153) and surfaced на new-session dashboard этой session. The arch.md §Occupied Resources Tauri IPC routes `incidents.*` row (line 178) still enumerates only `incidents.list_active`, `incidents.acknowledge`, `incidents.mark_resolved` (chunk #78 origin per §Architecture Registry Updates 2026-05-23).
- **Target sections:** (a) arch.md line 178 (§Occupied Resources Tauri IPC routes `incidents.*` row) — extend the enumeration с `incidents.mark_all_read`; (b) arch.md §Architecture Registry Updates — append а new dated 2026-05-26 entry в the canonical compact 5-content-line format per Proposal 8 Phase 1.
- **Code evidence:** `pulse-app/src/incidents_router.rs:97` (`IncidentsApi::mark_all_read` trait method declaration) + `pulse-app/src/incidents_router.rs:292` (`IncidentsApiImpl::mark_all_read` resolver impl).
- **Precedent shape:** mirrors 2026-05-25 chunk #86 `diagnostics.retry_interpretation` precedent (single-coordinated single-item amendment; same single-procedure-added-to-existing-namespace pattern). Earlier siblings: chunk #82 `model.current_profile`, chunk #80 `pulse://stream/cadence-events`, chunk #69 `diagnostics.template_distribution`.

## Final slug + classification preview

- **Slug (final):** `acknowledge-chunk-87-incidents-mark-all-read`
- **Anticipated classification:** Type 6 (Architecture registry update — `--allow-arch-registry`)
- **Anticipated marker count:** 1
- **Anticipated downstream propagation:** Branch (a) legacy empty (TauRPC procedure additions do NOT cascade into CLAUDE.md `<!-- GENERATED:setup:modules -->` Modules enumeration or `<!-- GENERATED:setup:overview -->` Stack/Key-directories — those derive from §Occupied Resources Cargo workspace crate names + §Inherited Defaults Workspace crates, NOT from Tauri IPC routes; per Proposal 12 cascade target map, this Type 6 is NOT one of the configured cascade targets).
