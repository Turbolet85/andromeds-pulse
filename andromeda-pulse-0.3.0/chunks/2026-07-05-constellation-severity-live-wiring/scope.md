# Scope — Constellation severity live-wiring

**Marker:** `2026-07-05-constellation-severity-live-wiring`
**Version:** andromeda-pulse-0.3.0 · Epoch 3 (State honesty & legibility)
**Prospective capability:** P-079 (new; operator-surfaced — no existing matrix cap maps to this work; confirm/mint at P5 matrix-link)

**Working entry (verbatim intent):**
> Constellation severity live-wiring — reconcile the incident workspace key so the per-service severity join lights up: the resolver's `incident_workspace_key` (`main.rs` = `data_dir`) ≠ the incident producer's `digest.workspace` (detected project root, `\\?\`-canonicalized), so the workspace-filtered `list_active()` finds ZERO active incidents → constellation dots all read "healthy" AND the incidents panel stays empty even under a live storm. Fix both sides to one detected-workspace source. HIGH VALUE — unblocks P-069's live severity differentiation + the incidents surface (both runtime-inert today; the chunk-#91 join landed forward-inert pending exactly this). Operator-surfaced at the 2026-07-05-legible-labeled-constellation (P-069) leave-running verify (docs/session-learnings.md 2026-07-05).

**Provenance:** Operator-surfaced at the P-069 (`2026-07-05-legible-labeled-constellation`) leave-running verify; root cause recorded in `docs/session-learnings.md` 2026-07-05. The chunk-#91 per-service severity join (`Incident.scope_id` + `ServiceListItem.priority_tier` + the resolver join) landed **forward-inert** pending exactly this reconciliation. P-069's matrix note names the same defect: "incident_workspace_key (main.rs = data_dir) != the incident producer's digest.workspace → the per-service join finds zero active incidents → all dots render 'healthy'."

## Problem (observed)
On a live retry-storm (deterministic-L4 or real), incidents ARE created and persisted, but nothing downstream sees them **as active-in-this-workspace**:
- The services / incidents resolver filters active incidents by an `incident_workspace_key` that `main.rs` sets to the **data dir** (`~/.andromeda-pulse/` or `ANDROMEDA_PULSE_DATA_DIR`).
- The incident PRODUCER stamps each incident's workspace as `digest.workspace` = the **detected project root** (workspace-detector output; `\\?\`-canonicalized on Windows).
- The two keys never string-match → the workspace-filtered `list_active()` returns ZERO active incidents → every service's computed `priority_tier`/severity falls back to "healthy" **and** the incidents panel renders empty, even under a sustained storm.

Net: P-069's live severity differentiation and the incidents surface are both **runtime-inert** — the render plumbing is correct and test-proven, but never receives non-empty data.

## What this builds
Reconcile the incident workspace key to a SINGLE canonical source of truth so the workspace-filtered active-incident query returns the storm's incidents:
- The resolver's filter key and the producer's `digest.workspace` resolve to the **same** canonical workspace identity — converging on the **detected project root** (the producer's existing value, so persisted incidents from prior sessions still match), NOT the data dir.
- Canonicalization parity so string-equality holds cross-platform (notably the Windows `\\?\` extended-length prefix).
- Under a live storm, the per-service severity join returns real `priority_tier` values → constellation dots render non-healthy hue + severity token; the incidents panel populates.

## Boundaries
**IN:**
- The workspace-key mismatch itself: the resolver filter side (`main.rs` `incident_workspace_key`) and/or the producer side (`digest.workspace`), converged to one canonical form.
- Canonicalization parity (Windows `\\?\` normalization; any trailing-separator / case differences) so the two keys compare equal.
- End-to-end proof: a deterministic-L4 (P-073) storm yields active incidents that the workspace-filtered query returns, lighting up per-service severity + the incidents list.

**OUT:**
- The severity→hue model (P-069's `severityToHueFraction` / 4-tier `priority_tier`) — unchanged.
- The constellation render / per-dot labels (P-069) and the incidents-panel render — unchanged (already correct; this chunk feeds them data).
- The compact widget (aggregate-glance boundary — no per-dot severity there).
- New TauRPC procedures / new arch §Occupied Resources entries — reuse the existing `services.list_with_states` + `incidents.list_active`; no new namespace expected.
- The incident-creation pipeline shape (cadence → digest → L4 → incident) — unchanged except the workspace-key value it stamps / is filtered by.

## Surfaces / contracts touched (to confirm in P3 codebase research)
- `pulse-app/src/main.rs` — the `incident_workspace_key` assignment (= data dir; ~`main.rs:686` per the P-069 matrix note) feeding the services / incidents resolver(s).
- The incident producer / digest path — where `digest.workspace` is set to the detected project root (cadence → digest → incident-creation lineage; chunks #80/#81/#91).
- `crates/workspace-detector` — the canonical detected-project-root source + its canonicalization form.
- The chunk-#91 per-service severity join (services resolver — `ServicesApiImpl` / `services.list_with_states`) that filters incidents by workspace.
- `incidents.list_active` (`IncidentsApiImpl`) workspace filter — same key.

## Acceptance (intent-level; refined at P4 + matrix-link)
- With deterministic-L4 (P-073) + a sustained identical-fingerprint storm, the workspace-filtered active-incident query returns ≥1 incident (not zero) — the key mismatch is closed.
- The affected service's constellation dot renders a non-healthy severity (hue + non-color token) reflecting the incident tier.
- The incidents panel lists the storm incident(s).
- No regression: genuinely-zero-active-incident state still renders "all healthy" / empty (P-067 live-only truth preserved).

## Notes for planning
- Strongly parallels the **research-corrects-intent** family (P-067 / P-074): the operator diagnosis names a precise mechanism (data_dir vs detected-root key mismatch) — P3 confirms which side to move + the exact canonical form; if the mechanism is refined, record it as a premise-correction in the plan (leave this scope + the working entry as historical source).
- Likely a backend/data-flow correctness chunk (method: `integration`, proven by a deterministic-L4 storm test asserting the workspace-filtered query returns the incident + non-healthy per-service tier) — not an affordance cap; pure-render surfaces it feeds are already P-069-proven.
