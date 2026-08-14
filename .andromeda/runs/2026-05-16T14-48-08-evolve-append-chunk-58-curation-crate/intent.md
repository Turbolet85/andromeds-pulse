# Intent — append-chunk-58-curation-crate

_Captured at Phase 1c step 6 per /andromeda-evolve SKILL.md. User's stated
intent verbatim + clarifying-question answers + resolved values for downstream
phases (classification, validation, artifact construction)._

## Invocation

```
/andromeda-evolve --allow-route-append
```

- Mode: default + `--allow-route-append` (Refuse 6 narrow exception enabled)
- Phase 0 state: schema_version=2, spec_amendments.active=0, drift_warnings=0, last_completed_chunk=route#57

## User's brief intent (Phase 1b verbatim)

User pasted full chunk #58 entry from `docs/v0_2_0/pulse-v0_2_0-route.md`:

```
### #58 — Curation crate extraction

> Refactor — extracts existing snapshot primitives into shared lib. Parallel-safe with #57.

- **Depends on:** nothing
- **Capabilities enabled:** prerequisite infrastructure for P-010, P-012, P-018 (detection logic needs reusable primitives)
- **Distillation layer:** foundational (used across L1a and L2)
- **Crates touched:** NEW `crates/curation/`, `crates/snapshot/` becomes consumer
- **TauRPC delta:** none
- **Broadcast topics delta:** none
- **Workspace deps delta:** none
- **Arch registry delta:** +1 crate (`curation`)
- **Specialist plan touches:** arch (occupied resources), test-plan (move snapshot anomaly tests to curation crate)
- **Summary:** Create `crates/curation/`, move `dedupe`, `anomaly` (latency outliers / error correlation / cardinality spikes), `critical_path`, `aggregation` modules from snapshot. Pub-ify primitives via `curation::contract` re-exports. Snapshot crate's external surface unchanged.
```

## Phase 1c clarifications gathered

- **Chunk text wording for route.md §2:** user selected option 2 (full Summary verbatim, ~33 words). Check 8.5 will surface WARNING (>25 words; not strict fail; matches Epoch 9 chunk #57 ~30-word precedent).
- Clarifying questions used: 1 of 4. Phase 1b's single sanity prompt does NOT count toward the cap.

## Resolved values

- **Plan target:** `.andromeda/route.md` (sole plan touched by THIS evolve invocation)
- **Form:** 1 (chunk append to existing Epoch 9 — Foundation v0.2.0)
- **Insertion position:** route_index=58 (after current chunk #57, end of Epoch 9 body)
- **Slug:** `append-chunk-58-curation-crate`
- **Chunk text (verbatim for route.md §2 single line):**

  > Curation crate extraction — Create `crates/curation/`, move `dedupe`, `anomaly` (latency outliers / error correlation / cardinality spikes), `critical_path`, `aggregation` modules from snapshot; pub-ify primitives via `curation::contract` re-exports; snapshot crate's external surface unchanged

- **chunks_renumbered:** empty (chunk #58 is new last position in Epoch 9; no subsequent chunks to renumber)
- **Motivation:** prerequisite infrastructure for P-010 / P-012 / P-018 per pulse v0.2.0 capability spec (algorithmic detection logic needs reusable curation primitives); refactor-only (no behavior change; snapshot external API preserved); parallel-safe with chunk #57 (zero shared dependencies).

## Specialist plan touches deferred to /implement (NOT in this evolve)

The chunk spec lists "Specialist plan touches: arch (occupied resources), test-plan (move snapshot anomaly tests to curation crate)". These are FUTURE work for the chunk's `/implement` cycle:

- arch §Occupied Resources +1 crate (`curation`) — will be authored via separate `/andromeda-evolve --allow-arch-registry` Type 6 amendment at /implement phase 4 (capability-drift acknowledgment after `crates/curation/` lands in workspace).
- test-plan tests move — will surface via Trigger 4 spec-drift-protocol if /implement encounters drift between test-plan §6 anomaly coverage and post-refactor reality. If no drift surfaces, test-plan stays as-is.

This evolve invocation registers ONLY the route entry. Downstream specialist plan amendments happen at /implement time, NOT now.

## Anticipated Tier 1 surface impact (CLAUDE.md grep-expansion target)

- CLAUDE.md pointer-table line "Roadmap (9 epochs / 56 chunks)" → "Roadmap (9 epochs / 57 chunks)" via /andromeda-setup-project --delta grep-expansion (per session 68 dogfood pattern: marker's `expected_propagation: []` is undercount; --delta Detection step 8 grep-expansion catches CLAUDE.md staleness automatically).
