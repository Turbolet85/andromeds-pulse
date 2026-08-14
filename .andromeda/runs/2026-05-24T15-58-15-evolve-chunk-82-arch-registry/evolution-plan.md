# Evolution Plan — chunk-82-arch-registry

**Skill:** `/andromeda-evolve --allow-arch-registry`
**Run datetime:** 2026-05-24T15:58:15Z
**Slug:** `chunk-82-arch-registry`
**Run dir:** `.andromeda/runs/2026-05-24T15-58-15-evolve-chunk-82-arch-registry/`

## User intent (verbatim from intent.md)

File the chunk #82 Type 6 follow-up amendment registering 3 items in arch §Occupied Resources:

1. **`interpretation` crate** in §Occupied Resources Cargo workspace crate names sub-section
2. **`model.current_profile`** in §Occupied Resources Tauri IPC routes sub-section
3. **`pulse://stream/model-status`** in §Occupied Resources Tauri IPC events (broadcast channels) sub-section

Plus a single compact §Architecture Registry Updates Decisions Log entry per Proposal 8 Phase 1 canonical compact template.

## Classification

**Type 6 — Architecture registry update**

- Refuse check: NO match (Refuse 5/6/2/3/1/4 all clear)
- `--allow-arch-registry` flag exception applies
- Single-coordinated multi-item amendment mirroring chunks #67/#68/#78 precedents

## Impact

- 1 plan touched: `.andromeda/architecture.md`
- 1 Decisions Log entry: arch.md §Architecture Registry Updates compact template
- 1 marker file: `.andromeda/runs/2026-05-24T15-58-15-spec-amendment-acknowledge-chunk-82-additions/amendment.md`
- state.yaml.spec_amendments.active: +1 entry

## Files touched

**Modified:**
- `.andromeda/architecture.md` — 4 surgical edits:
  - Line 186 (§Occupied Resources Cargo workspace crate names): added `interpretation` between `security` and `pulse-app`
  - Line 178 (§Occupied Resources Tauri IPC routes): inserted new bullet for `model.current_profile`
  - Line 181 (§Occupied Resources Tauri IPC events broadcast channels): appended `pulse://stream/model-status` к inline list
  - Append section (§Architecture Registry Updates): new compact entry dated 2026-05-24

**Created:**
- `.andromeda/runs/2026-05-24T15-58-15-spec-amendment-acknowledge-chunk-82-additions/amendment.md` — Type 6 marker
- `.andromeda/runs/2026-05-24T15-58-15-evolve-chunk-82-arch-registry/intent.md` — user intent record
- `.andromeda/runs/2026-05-24T15-58-15-evolve-chunk-82-arch-registry/evolution-plan.md` — this file

**Updated:**
- `.andromeda/state.yaml` — `spec_amendments.active` gains 1 entry (the chunk-82-additions amendment)

## Cross-references between amendments

(Single coordinated marker; no sibling amendments к cross-reference.)

## Validation results

**Phase 3 — Validation summary:** 11 ✓ / 1 ⚠ / 0 ✗ (pass)

- Check 1 — Spec-amendment-protocol compliance: ✓
- Check 2 — State.yaml schema compliance: ✓
- Check 3 — Decisions Log format consistency: ✓ (compact template matches chunks #67/#68/#78/#80/#81 precedents)
- Check 4 — Cross-reference integrity: ✓ (single marker; trivially passes)
- Check 5 — Vision document principles compliance: ✓ (no Project Intent conflict)
- Check 6 — Refuse taxonomy double-check: ✓
- Check 7.1 — Pure additive change: ✓
- Check 7.2 — Registry section only: ✓ (§Occupied Resources sub-sections)
- Check 7.3 — Code evidence: ✓ (all 3 paths verified)
- Check 7.4 — No new architectural concept: ✓ (existing non-empty sub-sections)
- Check 7.5 — Narrative-cascade staleness: ⚠ 5 word-form warnings (informational; never auto-fixed per Option B)
- Check 7.6 — Decisions Log compact-template conformance: ✓
- Check 7.7 — CLAUDE.md cascade pre-populate: ✓ (expected_propagation pre-populated с modules + overview anchors)

## Suggested next steps

1. **`/andromeda-setup-project --delta`** — propagate the amendment.
   - Reads state.yaml.spec_amendments.active filtering propagated_by_run=null
   - Reads marker's expected_propagation list: CLAUDE.md GENERATED:setup:modules + :overview anchors
   - Regenerates ONLY the affected CLAUDE.md anchors (preserves other content byte-identical)
   - Sets propagated_by_run on the amendment
   - One commit (chore(setup-project): cascade-style)

2. **After --delta succeeds, next /andromeda-wrap-session** will archive the amendment (move from active к archive с archived_at set; matches the textbook standard Type 6 single-cycle wrap pattern mirroring sessions 115/118/120/122/123/125/127/132/138).

3. **Word-form narrative-cascade warnings (Check 7.5) — DEFERRED:**
   - 5 word-form mismatches captured in marker `narrative_cascade_warnings` field (informational only)
   - Address via /andromeda-arch re-run OR manual edit OR accept staleness
   - Mirrors chunks #58/#68 precedent (those amendments also left narrative count words stale; project policy is к accept narrative drift unless а deliberate re-derive is scheduled)

## Audit trail

- intent.md (this run dir)
- evolution-plan.md (this file)
- amendment.md (`.andromeda/runs/2026-05-24T15-58-15-spec-amendment-acknowledge-chunk-82-additions/amendment.md`)
- arch.md edits (4 surgical edits via Edit tool)
- state.yaml entry (1 addition via Edit tool)

All writes atomic; no partial-write rollback required.
