# Materialization Plan (Delta) — Phase 0 checkpoint

**Skill:** `/andromeda-setup-project --delta`
**Run datetime:** 2026-05-24T16:07:09Z
**Mode:** delta-rerun (Type 6 permit path; flag-authorized arch.md registry update)
**Run dir:** `.andromeda/runs/2026-05-24T16-07-09-setup-project-delta/`

## Pending amendments processed

### Amendment: `2026-05-24T15-58-15-acknowledge-chunk-82-additions`

- **Marker path:** `.andromeda/runs/2026-05-24T15-58-15-spec-amendment-acknowledge-chunk-82-additions/amendment.md`
- **Flag used:** `--allow-arch-registry` (Type 6 permit path applies)
- **Plans amended (already by evolve):** `.andromeda/architecture.md` (§Occupied Resources Cargo workspace crate names + Tauri IPC routes + Tauri IPC events broadcast channels + §Architecture Registry Updates)
- **Expected propagation (this delta-rerun scope):**
  - CLAUDE.md `<!-- GENERATED:setup:modules -->` section: add `interpretation` bullet between `security` and `pulse-app`
  - CLAUDE.md `<!-- GENERATED:setup:overview -->` section: update Stack one-liner count "12 library crates" → "13 library crates"; Key directories `crates/` entry add `interpretation`

## Type 6 amendments propagated (flag-authorized arch.md registry update)

Per `delta-rerun-protocol.md` Type 6 permit path:

- 1 Type 6 amendment in this delta scope (chunk #82 additions; flag-authorized)
- Architecture.md already updated by `/andromeda-evolve --allow-arch-registry` (4 surgical edits to §Occupied Resources sub-sections + 1 compact Decisions Log entry in §Architecture Registry Updates)
- setup-project --delta cascades the arch registry update к CLAUDE.md (Branch (b) cascade per Proposal 12)
- No Tier 2/3 .claude/rules/ or .claude/docs/ regeneration required (marker's expected_propagation did NOT include those)

## Files touched (delta scope)

**Modified:**
- `CLAUDE.md` — 2 surgical edits inside GENERATED:setup:overview + GENERATED:setup:modules anchors

**Updated (lifecycle progression at Phase 9):**
- `.andromeda/state.yaml` — `spec_amendments.active.[id].propagated_by_run` set к this run-dir
- `.andromeda/runs/2026-05-24T15-58-15-spec-amendment-acknowledge-chunk-82-additions/amendment.md` — Lifecycle status `[x] Propagated {ISO timestamp}` checked

## Files preserved byte-identical

Everything outside the delta scope, including:
- CLAUDE.md other GENERATED:setup:* anchors (warnings / pointer-table / workflow / architecture / imports / deeper-topics)
- CLAUDE.md USER:session-learnings section
- All `.claude/rules/*.md` (security / testing / observability / a11y / verification-harness / design-tokens / frontend)
- All `.claude/docs/*.md` (5 core + 5 specialist summaries + 12 services + session-learnings + andromeda-after-mvp-playbook)
- `.claude/agents/code-reviewer.md`
- `.claude/settings.json`
- `.gitignore`
- `scripts/agent-run.{sh,ps1}`
- `.andromeda/context/dependency-tree.md`
- `.andromeda/context/api-surface.md`
- `.claude/session-handoff.md`
- All other `.andromeda/runs/` (audit trail)

Phase 8 byte-identity validation verifies all "preserved" files unchanged.

## Deferred (not in delta scope)

**Check 7.5 word-form narrative-cascade warnings (informational only per Option B):**

The marker's `narrative_cascade_warnings` field captured 5 word-form mismatches in arch.md structural sections that are NOT auto-fixed:
- §Design Philosophy line 4: "twelve library crates" + "fourteen workspace members"
- §Cross-cutting Patterns line 229: "all twelve library crates"
- §Project Intent line 312: "twelve Rust crates"
- §Project Intent line 314: "The twelve reserved crate names"

These propagate organically to CLAUDE.md sections NOT in delta scope:
- CLAUDE.md `<!-- GENERATED:setup:architecture -->` mirrors arch §Design Philosophy narrative paragraph (word-form "twelve")
- CLAUDE.md `<!-- GENERATED:setup:pointer-table -->` mentions "Per-module implementation notes (12 crates)" — numeric form, but not in delta scope
- CLAUDE.md `<!-- GENERATED:setup:deeper-topics -->` lists 12 services per-module crates — not in delta scope

User decides addressment path:
- /andromeda-arch re-run (regenerates arch.md narrative; cascades to CLAUDE.md via next full setup-project)
- Manual arch.md edit + full /andromeda-setup-project re-derive
- Accept staleness (precedent: chunks #58/#68 also left narrative count words stale post-amendment)

## Suggested next steps

1. **After --delta commits,** next `/andromeda-wrap-session` will archive the amendment (move from active к archive с archived_at set; clears the 3 D3 drift entries from session 139 wrap's drift_warnings; textbook standard Type 6 single-cycle wrap pattern mirroring sessions 122/127/132/138).

2. **0.2.0 ship blockers** continue к be the primary path; chunk #82 substrate landed + propagated; chunks #83-#85 (LLM interpretation pipeline) remain. Chunk #83 (Prompt scaffolding + JSON schema + primary tier inference) will trigger the first actual `use mistralrs::*` import — natural Step 0 spike runtime validation point per arch §Established Decisions caveat.
