# arch extract

## Relevance
Relevant — arch governs this chunk as guardrails: crate placement, reuse-existing-resource discipline, env-var namespace, and module dependency direction all bind, even though the chunk adds no new arch surface.

## Constraints
- **No new IPC surface — reuse existing resolvers.** The fix flows through `services.list_with_states` (`ServicesApiImpl`) and `incidents.list_active` (`IncidentsApiImpl`), both already in the canonical procedure list; the chunk changes a filter-key *value*, not the namespace (per arch §Occupied Resources Tauri IPC routes; scope explicitly bans new §Occupied Resources entries).
- **Canonical workspace identity = `crates/workspace-detector` output, not the data dir.** `workspace-detector` is the reserved "detect host project context" crate; the reconciled key must converge on its `\\?\`-canonicalized detected-project-root, NOT `ANDROMEDA_PULSE_DATA_DIR` (per arch §Occupied Resources Cargo workspace crate names + Filesystem locations; §Infrastructure Patterns directory structure).
- **Producer/filter code lives in the locked crates.** `digest.workspace` is stamped in the cadence→digest→incident lineage inside `crates/triage`; the `incident_workspace_key` wiring lives in `pulse-app/src/main.rs`; incident persistence is `crates/corpus` (per arch §Occupied Resources — `pulse://stream/{cadence-events,digests,incidents}` all under `crates/triage`; §Inherited Defaults one-crate-per-module).
- **Module dependency direction — reconcile at the DAG root.** `pulse-app` wires the key; `workspace-detector` must remain a leaf with no reverse dependency on `triage`/`corpus`/`pulse-app` (per arch §Cross-cutting Patterns Module dependency direction).
- **No new env var.** The workspace identity derives from `workspace-detector` output, not a new `ANDROMEDA_PULSE_*` variable; the acceptance storm test reuses the reserved `ANDROMEDA_PULSE_L4_DETERMINISTIC` gate (per arch §Occupied Resources Environment variables — canonical list).
- **Cross-bridge shape unchanged.** `services.list_with_states` / `incidents.list_active` response DTOs stay `serde::Serialize` with unchanged wire shape; only the filter-key value moves (per arch §Cross-cutting Patterns Cross-bridge data shape + §Standard Contracts).
- **AppError envelope for any new failure path.** If key resolution can fail, it surfaces through the existing `AppError` enum at the bridge, not a new error type (per arch §Conventions Error response schema (Tauri IPC)).

## Patterns to follow
- **`workspace-detector` as single canonical source.** Both sides read the same detected-project-root value + canonicalization form from the one reserved crate, rather than each re-deriving it (arch §Occupied Resources crate names; §Infrastructure Patterns directory structure "detect host project context").
- **Boot-time identity computed in `pulse-app`.** The `incident_workspace_key` is resolved once at `main.rs` startup wiring and injected into the resolvers, matching the existing config-at-boot pattern (arch §Cross-cutting Patterns Config management; §Occupied Resources — `ServicesApiImpl`/`IncidentsApiImpl` wired in pulse-app).
- **Deterministic-L4 harness for the storm proof.** Drive the end-to-end test through the env-gated deterministic runner (canned `L4Output`, no GPU/model) for reproducible, machine-parseable verification (arch §Occupied Resources `ANDROMEDA_PULSE_L4_DETERMINISTIC`; §Cross-cutting Patterns Development Style = agent-driven).
- **Canonicalization parity discipline.** Match `workspace-detector`'s existing canonical form (Windows `\\?\` extended-length prefix, trailing-separator/case normalization) so string-equality holds cross-platform, consistent with arch's established path-canonicalization discipline (arch §Occupied Resources Filesystem locations canonicalization note).

## Anti-patterns to avoid
- **Do not add a new TauRPC procedure or §Occupied Resources entry.** A new procedure would also require a `pulse-app/capabilities/` file; unnecessary here (arch §Webview IPC capability policy; scope OUT-list).
- **Do not introduce a second/divergent canonicalization scheme or a new env var.** Reuse `workspace-detector`'s form and the existing env namespace (arch §Occupied Resources Environment variables + crate names).
- **Do not leak the key-derivation across crate boundaries as `pub(crate)`.** Cross-crate use goes through each crate's explicit contract module only (arch §Conventions Module visibility discipline).

## Contract bindings
- **arch ↔ tests harness:** the reconciled canonical workspace identity must equal the value the deterministic-L4 storm fixture produces; the storm test drives the `ANDROMEDA_PULSE_L4_DETERMINISTIC` gate (arch §Cross-cutting Patterns Test-time telemetry injection + §Occupied Resources env var).
- **arch ↔ producer/resolver join:** the filter key (`pulse-app` `incident_workspace_key`) and the producer key (`crates/triage` `digest.workspace`) bind through the chunk-#91 per-service severity join over the existing `services.list_with_states` + `incidents.list_active` resolvers.

## Acceptance criteria contributions
- (arch) The reconciled workspace key derives from `crates/workspace-detector` output, not `ANDROMEDA_PULSE_DATA_DIR` (arch §Occupied Resources).
- (arch) No new TauRPC procedure and no new §Occupied Resources entry — the fix reuses `services.list_with_states` + `incidents.list_active` unchanged in shape.
- (arch) No new env var beyond the existing `ANDROMEDA_PULSE_*` namespace; the storm proof uses the reserved `ANDROMEDA_PULSE_L4_DETERMINISTIC` gate.
- (arch) Wiring stays in `pulse-app` (DAG root) with `workspace-detector`/`triage`/`corpus` as leaf deps — no reverse dependency introduced (arch §Cross-cutting Patterns Module dependency direction).

## Relevant amendment history
- **2026-05-18 — `services.list_with_states` + `pulse://stream/service-lifecycle` (chunk #67):** landed the services resolver (`ServicesApiImpl` reading `triage::lifecycle::InMemoryServiceRegistry`) that hosts the per-service severity join this chunk lights up.
- **2026-05-23 — `incidents.*` namespace + `pulse://stream/incidents` (chunk #78):** landed `incidents.list_active` (`IncidentsApiImpl` over `triage::IncidentRegistry`/`IncidentPersistence`) — the workspace-filtered active-incident query being corrected.
- **2026-05-23 — `pulse://stream/cadence-events` (chunk #80) + `pulse://stream/digests` (chunk #81):** the cadence→digest producer lineage in `crates/triage` where `digest.workspace` is stamped (the producer side of the mismatch).
- **2026-06-28 — `ANDROMEDA_PULSE_L4_DETERMINISTIC` (P-073):** the reserved deterministic-L4 gate the acceptance storm test runs on; third impl behind the unchanged `LlmInferenceRunner` trait.
- **Note:** chunk #91's per-service severity join has *no* registry-closure amendment — it modified existing resolvers without adding an Occupied Resource, which is the same no-new-resource posture this chunk inherits (reinforces the "no new §Occupied Resources entry" constraint above).
