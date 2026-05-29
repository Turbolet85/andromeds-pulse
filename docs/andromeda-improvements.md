# Andromeda Pipeline Enhancement Proposals

_Project-specific TODO list for Andromeda pipeline itself — separate from pulse application code. Each entry describes a proposed enhancement to skill mechanics in `~/.claude/skills/andromeda-*/` (the user-level skill folders), arising from frictions encountered during pulse v0.2.0 dogfood work (the first after-MVP evolution experience in Andromeda's history)._

_When a proposal is implemented (likely as a separate session that touches `~/.claude/skills/andromeda-*/` files), mark with **Status: IMPLEMENTED — {ISO date} — {commit reference}**. Implemented proposals stay in this file as historical record so future readers see WHY each enhancement landed._

_Authoring convention: each proposal includes Problem / Proposal / Design / Implementation cost / When to do / Cross-references. Prefer prose paragraphs over bullet lists for explanation — readers should understand context not just see specs._

---

## Status: PROPOSED — 2026-05-16 (session 66)

### Proposal 1 — `--allow-arch-decision` flag for `/andromeda-evolve`

**Problem:**

Some after-MVP work requires additions to architecture.md structural sections — `§Established Decisions` / `§Stack` / `§Cross-cutting Patterns`. Current Andromeda mechanics offer no clean automated path for this:

- `/andromeda-evolve` (no flag) — hits Refuse 1 (architecture body modification); suggests `/andromeda-arch` re-plan.
- `/andromeda-evolve --allow-arch-registry` — Check 7.2 explicitly disallows `§Established Decisions` / `§Stack` / `§Cross-cutting Patterns` / `§Project Intent` / `§Design Philosophy` even with the flag.
- `/andromeda-arch` re-plan — greenfield, regenerates everything; overkill for a single decision addition. Also, arch is intentionally write-once per its skill description.
- Manual `arch.md` edit — works, but breaks Andromeda's core audit-trail discipline (no marker file, no `state.yaml` entry, no formal Decisions Log entry mandated, no setup-project propagation tracking).

In pulse v0.2.0 plan (`docs/v0_2_0/pulse-v0_2_0-route.md`), at least 3 chunks require structural arch additions:

- **Chunk #69 (corpus scaffold)** — adds new architectural concept (encryption at rest + OS keychain integration + persistent SQLite corpus). The arch §Telemetry Retention Surface section explicitly says "persistent disk storage is reserved for the heavier backends and is out of scope" — this decision needs to be amended.
- **Chunk #74 (LLM runtime + hardware profile)** — adds new entry to §Stack (chosen runtime: `mistralrs` or `candle` per Pre-D1 resolution) and corresponding §Established Decisions rationale entry.
- **Chunk #84 (MCP server + tool exposure)** — adds §Established Decisions entry formally positioning MCP as "one of three equal-tier output channels, not coupling."

Without a documented automated path, these become manual arch edits — breaking audit-trail discipline at the very moments when audit trail matters most (foundational architectural decisions).

**Proposal:**

New flag `--allow-arch-decision` for `/andromeda-evolve`. Narrow Refuse 1 exception, analogous to existing `--allow-arch-registry` (Refuse 1 exception) and `--allow-route-append` (Refuse 6 exception). Permits purely additive entries to specific structural sections while keeping the protective restrictions on foundational sections.

**What flag permits:**

- Purely additive bullet/entry to `§Established Decisions`
- Purely additive row to `§Stack` table
- Purely additive bullet to `§Cross-cutting Patterns`

**What flag still refuses (Check 9.2):**

- `§Project Intent` — product-soul level; Refuse 2 territory (core principle violation).
- `§Design Philosophy` — product-soul level; Refuse 3 territory (architectural inversion).
- Modification of existing entries in any section (Check 9.1 strict additivity).
- Deletion of existing entries.
- New section creation (that's full architecture re-plan territory).

**New classification:** Type 8 — Architecture decision addition. Mirrors Type 6 structure (the existing `--allow-arch-registry` classification).

**New validation:** Check 9 with 5 sub-checks (mirrors Check 7's structure for the registry-flag case):

1. **Purely additive** — diff analysis confirms only new lines in target section; no modification/deletion of existing arch content (beyond positional renumbering if needed).
2. **Allowed section** — target heading matches one of `§Established Decisions` / `§Stack` / `§Cross-cutting Patterns` (case-insensitive); disallowed sections refuse even with flag.
3. **Decision text well-formed** — follows existing convention in the target section (e.g., for `§Established Decisions`: bold category prefix + subject + description body; format inferred from existing entries to maintain consistency).
4. **Rationale present** — marker's `Rationale:` field is non-empty with ≥2 sentences explaining why this decision now (not abstract "future need"; concrete grounding required — analogous to Check 8.6 motivation grounding for route appends).
5. **Downstream propagation list non-empty** — structural changes cascade to specialist plans + Tier 2/3 distillations (unlike registry additions which often have empty propagation); marker's `expected_propagation` field MUST explicitly enumerate affected files.

**Marker structure additions** (Type 8 fields, mirrors Type 6 fields per spec-amendment-protocol.md Part A Flag authorization block):

```
- **Flag used:** `--allow-arch-decision`
- **Section modified:** `§Established Decisions` (или §Stack / §Cross-cutting Patterns)
- **Decision added:** "{decision title}"
- **Rationale:** {≥2 sentence explanation}
- **Downstream impact assessment:**
  - {specialist plan affected — e.g., "security-plan.md §Bootstrap phases — will need encryption key-management entry"}
  - {Tier 2/3 file affected — e.g., ".claude/rules/security.md — top-rank anti-patterns shift"}
- **Originating chunk:** chunk #{N} (if known; otherwise N/A)
- **Check 9 status:** ✓ purely additive / ✓ allowed section / ✓ format conforming / ✓ rationale present / ✓ propagation list specified
```

**setup-project --delta integration** (mirrors Type 6 permit path in delta-rerun-protocol.md):

- Detection step 4 (or new step 4a) checks for `flag_used == "--allow-arch-decision"` + Trigger string matches `/andromeda-evolve` signature.
- If verified — proceeds with Type 8 permit path; uses marker's `expected_propagation` for delta scope (non-empty per Check 9.5).
- Apply grep-expansion defense-in-depth for missed cascades (existing mechanism).
- Set `propagated_by_run` to current --delta run-dir path.
- Surface in `materialization-plan-delta.md` under "Type 8 amendments propagated (flag-authorized arch.md structural addition)" subsection — analogous to existing Type 6 subsection.

**Asymmetric authoring:** like Type 6, only `/andromeda-evolve` can author Type 8 amendments. `/andromeda-implement`'s spec-drift-protocol does NOT get this flag — harness-fired drift detection should not auto-opt-in to flag-authorized structural arch modifications (deliberate restriction).

**Implementation cost estimate:**

| File | Change | Lines |
|---|---|---|
| `andromeda-evolve/SKILL.md` | Parse new flag, MUST/MUST NOT clauses, invocation example | ~25 |
| `andromeda-evolve/references/refuse-taxonomy.md` | Refuse 1 Exception subsection — add `--allow-arch-decision` variant + refuse template (route through Phase 1b sanity check) | ~40 |
| `andromeda-evolve/references/classification-taxonomy.md` | Type 8 — Architecture decision addition (mirrors Type 6 structure) + tie-breaking note | ~80 |
| `andromeda-evolve/references/validation-checks.md` | Check 9 specification (5 sub-checks + failure shape + recovery + anti-patterns) | ~80 |
| `andromeda-evolve/references/output-templates.md` | Type 8 marker template + Decisions Log entry format | ~30 |
| `andromeda-evolve/references/dialog-templates.md` | Type 8 classification confirmation prompt | ~15 |
| `andromeda-evolve/references/example-runs.md` | 1 happy-path + 1 refuse example for Type 8 (regression suite) | ~60 |
| `spec-amendment-protocol.md` (×3 triangle skills — byte-identical) | Part A Flag block variant + Part B Type 8 fields + Part C Case 5 (mirrors Case 4) + Part D Narrow Exception extension | ~50 × 3 = 150 |
| `andromeda-setup-project/references/delta-rerun-protocol.md` | Type 8 permit path | ~40 |
| `andromeda-new-session/references/session-state-contract.md` | State I-arch-decision-update variant (mirrors I-arch-registry-update) | ~15 |

**Total:** ~545 lines across ~13 files. Medium-effort skill extension, comparable in scope to the initial `--allow-arch-registry` implementation.

**Coordination note:** `spec-amendment-protocol.md` is byte-identical across the triangle (setup-project + wrap-session + new-session); changes must apply identically to all 3 copies, verified at setup-project Phase 8 cross-skill diff check.

**When to do:**

Two options under consideration:

- **Option A — Proactively, before pulse v0.2.0 chunks #69 / #74 / #84.** Get the flag working before the chunks that need it. Risk: design choices may be informed only by anticipated need rather than felt need.
- **Option B (recommended) — After first manual arch edit as a deliberate exercise.** Do manual edits for at least one chunk (likely #69, the first structural chunk encountered), feel the friction, then design `--allow-arch-decision` based on real observations. This is true dogfood discipline — informs Check 9 sub-checks with empirical grounding.

Decision 2026-05-16: defer. This proposal exists so the idea isn't lost in chat history; revisit when first Pattern 4 (Pulse v0.2.0 structural arch change) work approaches.

**Cross-references:**

- Originated in session 66 conversation (chat history not committed; high-level decisions captured in this session's `.claude/session-handoff.md` "Key Decisions" section).
- Pulse v0.2.0 chunks needing this: #69 (corpus encryption + keychain), #74 (LLM runtime decision per Pre-D1), #84 (MCP positioning) per `docs/v0_2_0/pulse-v0_2_0-route.md`.
- Precedent flag mechanisms: `~/.claude/skills/andromeda-evolve/references/refuse-taxonomy.md` §Refuse 1 Exception (`--allow-arch-registry`) and §Refuse 6 Exception (`--allow-route-append`).
- Companion playbook: `.claude/docs/andromeda-after-mvp-playbook.md` Pattern 4.

---

## Status: PROPOSED — 2026-05-16 (session 66)

### Proposal 2 — setup-project generates `.claude/docs/andromeda-after-mvp-playbook.md` from template

**Problem:**

The pulse project is Andromeda's first dogfood experience past v0.1.0 MVP. The post-MVP workflow (`/andromeda-evolve` + manual edits + `/andromeda-setup-project --delta` cycle) wasn't documented in any project-level file before session 66 — agents starting fresh in a new conversation had to rediscover the workflow by reading 4 skill SKILL.md + ~7 reference files (~2000 lines total) plus reasoning from the user's mental model.

This is unsustainable across projects. Every future Andromeda project that grows past v0.1.0 MVP will face the same rediscovery cost. The post-MVP workflow is GENERIC — Patterns 1-4 + decision tree + skill mechanics references work the same across any Andromeda project — but currently lives only in project-specific docs (session-learnings entries + this project's hand-authored playbook).

**Proposal:**

Add `.claude/docs/andromeda-after-mvp-playbook.md` to setup-project's generated file set. The playbook content is mostly project-agnostic; setup-project would generate it from a template at greenfield setup time, with minimal project-specific variable substitution.

**Generation strategy** (mirrors existing `.claude/docs/session-learnings.md` handling):

- Generated by setup-project Phase 1-3 (alongside other `.claude/docs/*.md` files).
- Generated **if missing**; NEVER regenerated on subsequent setup-project runs (preserves user customizations and per-project augmentations like dogfood observations).
- Same atomic-write discipline as other generated files.

**Template content** (mostly static; ~250 lines):

Located in `~/.claude/skills/andromeda-setup-project/references/docs-templates/andromeda-after-mvp-playbook.md`. Identical structure to the hand-authored version in pulse (`.claude/docs/andromeda-after-mvp-playbook.md`):

1. TL;DR — workflow shape (greenfield → evolve flow transition).
2. Project state context (project-specific variable: route progress).
3. The 4 patterns:
   - Pattern 1 — Arch registry update (`--allow-arch-registry` + setup --delta).
   - Pattern 2 — Route chunk append (`--allow-route-append` + setup --delta).
   - Pattern 3 — Specialist plan amendment (Type 1-5).
   - Pattern 4 — Arch structural change (manual edit + setup full mode; OR `--allow-arch-decision` if Proposal 1 implemented).
4. Drift resolution table (D1-D6 dimensions → fix pattern mapping).
5. Decision tree per chunk.
6. Key commands reference table.
7. Trigger 4 inside `/andromeda-implement` (alternative drift resolution path).
8. Cross-references to skill mechanics (~/.claude/skills/andromeda-*/ paths).

**Variable substitution** (minimal — most content is generic):

- Project-specific dogfood note (optional section): mentions the project name if pulse-style "first dogfood" framing applies. Most projects skip this; pulse's hand-authored version has it because pulse was authored before the template existed.

**Implementation cost estimate:**

| File | Change | Lines |
|---|---|---|
| `andromeda-setup-project/SKILL.md` | Add playbook to Phase 1-3 generation list; mention preserve-if-exists handling | ~10 |
| `andromeda-setup-project/references/docs-templates/andromeda-after-mvp-playbook.md` | NEW template file (essentially the playbook content; ~250 lines) | ~250 |
| `andromeda-setup-project/references/validation.md` | Add playbook to expected output files (Phase 8 health check) | ~3 |
| `andromeda-setup-project/references/health-criteria.md` | Maybe — add playbook existence to 14 health checks if relevant | ~5 |

**Total:** ~270 lines. Lower effort than Proposal 1 — mostly template content, less skill-mechanics surgery.

**Companion: `docs/andromeda-improvements.md` stub template (optional sub-proposal):**

Could similarly generate an empty stub for the project-level improvements file (this file). Stub would be ~15 lines (this file's header + format convention notes + empty "Status: PROPOSED" placeholder). Lower priority than playbook; users may naturally create this file when they have a first improvement to propose (as happened in pulse).

**When to do:**

After Proposal 1 (`--allow-arch-decision`) implemented OR independently. The playbook can document Pattern 4 in either form:

- **Before Proposal 1** — Pattern 4 documents manual arch edit path as the only option.
- **After Proposal 1** — Pattern 4 documents `--allow-arch-decision` as primary path with manual edit as fallback.

Either way, the playbook itself is useful. Implementation timing flexible; could even precede Proposal 1 (the playbook would just mention "manual edit; --allow-arch-decision flag proposed in docs/andromeda-improvements.md but not yet implemented").

**Cross-references:**

- Companion file in this project: `.claude/docs/andromeda-after-mvp-playbook.md` (hand-authored at session 66; serves as the source content for the future template).
- Proposal 1 above (`--allow-arch-decision` flag) — if implemented, Pattern 4 in the template gets updated accordingly.
- Existing setup-project docs templates pattern: `~/.claude/skills/andromeda-setup-project/references/docs-templates/*.md`.

---

## Status: PROPOSED — 2026-05-16 (session 66)

### Proposal 3 — setup-project includes playbook + improvements in CLAUDE.md pointer table generation

**Problem:**

Session 66 added two new project-level docs (`.claude/docs/andromeda-after-mvp-playbook.md` + `docs/andromeda-improvements.md`) and wanted them surfaced from CLAUDE.md §Where to Look pointer table for agent visibility. The pointer table lives inside `<!-- GENERATED:setup:pointer-table start/end -->` markers — owned by setup-project, regenerated from `materialization-plan.md` on each full setup-project run.

Manual addition to the pointer table (done in session 66 pragmatically) will be overwritten the next time `/andromeda-setup-project` runs in full mode, because `materialization-plan.md` doesn't list these two new files as pointer-table entries.

This is a related but distinct gap from Proposal 2 (which addresses generation of the playbook file itself). Proposal 2 generates the FILE; Proposal 3 ensures the pointer table REFERENCES the file.

**Proposal:**

When setup-project generates the §Where to Look pointer table (via `materialization-plan.md` Phase 1-3), include automatic entries for:

1. `.claude/docs/andromeda-after-mvp-playbook.md` — labeled "Andromeda post-MVP workflow (4 patterns + drift table + decision tree per chunk + skill mechanics refs)"
2. `docs/andromeda-improvements.md` — labeled "Andromeda improvement proposals + dogfood friction log (where to record pipeline gaps as they surface during chunk work)"

These entries appear automatically in every project's CLAUDE.md, so new agents (after `/clear` → `/andromeda-new-session`) see them as standard navigation targets alongside specialist plans + summaries.

**Generation conditions:**

- Generate pointer entry only if the corresponding file exists in the project. Project agnostic — works whether the project past v0.1.0 MVP or still in greenfield phase.
- If both files exist → both entries added at end of pointer table (after specialist + session-learnings entries).
- If only one exists → just that one entry added.
- If neither exists → no entries (greenfield project that hasn't generated playbook OR that opted out).

**Implementation cost estimate:**

| File | Change | Lines |
|---|---|---|
| `andromeda-setup-project/SKILL.md` | Document new pointer entries in Phase 1-3 logic + file existence conditional | ~10 |
| `andromeda-setup-project/references/docs-templates/pointer-table-template.md` (or equivalent — wherever pointer table seed lives) | Add conditional entries with file-existence check | ~5 |
| `andromeda-setup-project/references/validation.md` | Add check for pointer entries when files exist | ~5 |

**Total:** ~20 lines. Small enhancement; can be bundled with Proposal 2 implementation.

**Dependency:**

Most natural to implement together with Proposal 2 (the file template). Without Proposal 2 file template, Proposal 3 generates pointers to non-existent files (broken unless user hand-authors them like pulse did).

Sequence: implement Proposal 2 first (file template), then Proposal 3 (pointer entry).

**Cross-references:**

- Proposal 2 above (file template generation).
- Pulse-specific: manual pointer entries added in session 66 (`CLAUDE.md` §Where to Look) — will be overwritten if/when full setup-project runs before Proposals 2+3 implemented. Persist them in the meantime by either re-adding manually OR avoiding full setup-project runs (use `--delta` only).
- Existing pointer table convention: examine row format in CLAUDE.md §Where to Look section to mirror styling.

---

## Status: IMPLEMENTED — 2026-05-16 (session 66, commit pending) — first meta-Andromeda enhancement landed via dogfood

### Proposal 4 — Extend `--allow-route-append` to permit terminal-position new epoch creation + first chunk(s)

**Problem (surfaced during session 66 chunk #57 planning):**

When attempting to start pulse v0.2.0 chunk #57 (Widget real-data binding) via `/andromeda-evolve --allow-route-append`, immediately hit a structural blocker:

- pulse route §2 currently has 8 epochs, all closed (chunks #1-#56). Epoch 8 = "Polish & ship" (semantically about v0.1.0 finalization).
- pulse v0.2.0's 33 prospective chunks (#57-#89) belong to NEW conceptual phases (Foundation v0.2.0, Algorithmic detection, Log templates, Service lifecycle, Corpus, Digest pipeline, LLM interpretation, UI surfaces, Output channels, Operations, Reflection, Finalization).
- Squeezing chunks #57+ into Epoch 8 ("Polish & ship") is semantically wrong — they're new feature work, not polish.
- Original Check 8.2 refused new epoch creation under `--allow-route-append` even with flag (new epoch was /andromeda-route territory).
- /andromeda-route is greenfield-only — does not re-derive existing route; just regenerates from scratch.

Result: chunks #57+ had no automated route-registration path. Manual route.md edit + setup full mode was the only option — breaking the core Andromeda audit-trail discipline at exactly the moment the project transitions from MVP to evolution mode.

**User-proposed design (2026-05-16 session 66 conversation):**

> "давай знаешь как поступим разрешим --allow-route-append добавлять epoch но только последней записью и обязательно вместе с первым чанком эпохи"

(Translation: "let's allow `--allow-route-append` to add an epoch but only as the last entry and obligatorily together with the first chunk of the epoch")

Two-property restriction:
1. **Terminal-position only** — new epoch MUST appear after all existing epochs in route.md §2 (position K = max(existing K) + 1). No mid-route insertion (would shift existing positions = restructuring).
2. **Non-empty body** — new epoch creation MUST be accompanied by ≥1 chunk in the same evolve invocation. Empty placeholder epochs refused.

Combined, these restrictions:
- Preserve position-stability for completed and in-progress chunks (terminal position doesn't shift anything).
- Prevent accumulation of dead epoch headings (every epoch in route.md has at least one motivating chunk).
- Maintain atomic-discipline (epoch + first chunk land together in one Phase 6 atomic write).

**Implementation (this commit):**

Modified user-level skill files in `~/.claude/skills/andromeda-evolve/`:

| File | Changes |
|---|---|
| `SKILL.md` | Updated `--allow-route-append` MUST/MUST NOT clauses to permit terminal-position new epoch + ≥1 chunk; added "Flag-specific terminal-epoch rules" subsection with mechanical §1 update specification (Total chunks + Epochs count lines auto-update). |
| `references/refuse-taxonomy.md` | Extended Refuse 6 Exception subsection to describe Form 1 (existing-epoch append, original case) + Form 2 (terminal new epoch creation). Restrictions list explicitly carves out mid-route epoch insertion vs terminal append. |
| `references/classification-taxonomy.md` | Type 7 Definition section now explicitly documents both Forms. Added Form 2 examples (v0.2.0 evolution epoch, major feature line, post-stabilization category). Added Form 2-specific marker fields: `new_epoch_created` / `new_epoch_title` / `new_epoch_position` / `scope_summary_updates`. |
| `references/validation-checks.md` | Check 8.1 (purely additive) extended — §1 mechanical count line updates permitted under Form 2 only. Check 8.2 (insertion target) now matches Form 1 OR Form 2. Added Check 8.2.5 (Form 2 — terminal-position only; refuses mid-route epoch insertion). Added Check 8.2.6 (Form 2 — non-empty body; refuses empty epoch placeholder). Severity table + failure shape + anti-patterns extended accordingly. |
| `references/output-templates.md` | Type 7 marker template Flag authorization block extended with Form 2 fields (`new_epoch_created` / `new_epoch_title` / `new_epoch_position` / `epoch_boundary_rationale` / `scope_summary_updates`). state.yaml entry additions section updated with Form 2 yaml structure including the new fields. |

**Files NOT touched (intentional scope limit):**

- `references/example-runs.md` — adding a Form 2 happy-path example deferred. Existing Type 7 example continues to apply to Form 1; Form 2 testing in pulse v0.2.0 chunk #57 cycle will provide first real example.
- `~/.claude/skills/andromeda-{setup-project, wrap-session, new-session}/references/spec-amendment-protocol.md` (×3 byte-identical copies) — formal Part B schema for Type 7 fields was not documented even for Form 1 (pre-existing gap). Extending it now would require coordinated 3-copy update + Phase 8 byte-identity verification. Deferred as separate follow-up. Functional implementation works without spec-amendment-protocol.md schema documentation because output-templates.md drives marker + state.yaml format; setup-project --delta reads marker (not strict schema enforcement). Acknowledged limitation.
- `references/delta-rerun-protocol.md` — Type 7 permit path semantics unchanged. Route.md still has no Tier 2/3 dependents per plan→file mapping table; Form 2 doesn't change that. Delta scope for both Form 1 and Form 2 is typically empty (lifecycle progression only).

**Validation:**

This proposal was implemented INLINE during session 66 conversation to unblock pulse v0.2.0 chunk #57 evolve cycle. The implementation IS the dogfood test: chunk #57 evolve (immediately after this commit) will exercise Form 2 (Epoch 9 — Foundation v0.2.0 + chunk #57 as first member). If chunk #57 evolve cycle succeeds, Proposal 4 design is validated. If it surfaces additional friction, append Proposal 5+ here documenting refinements.

**Follow-up gaps for future enhancement sessions:**

1. **spec-amendment-protocol.md Part B Type 7 schema documentation** (×3 byte-identical copies in triangle skills) — Form 1 AND Form 2 fields should be formally documented in the canonical contract. Currently lives only in output-templates.md.
2. **example-runs.md Form 2 happy-path example** — add canonical case study showing v0.2.0-style new epoch creation.
3. **delta-rerun-protocol.md Type 7 permit path** — explicitly document Form 2 sub-case (epoch creation contributing nothing to delta scope; lifecycle progression only).

**Cross-references:**

- User design proposal: session 66 chat conversation (not committed; high-level idea captured in this Proposal 4 body).
- Triggering need: pulse v0.2.0 chunk #57 (Widget real-data binding) cannot fit semantically into existing Epoch 8 "Polish & ship".
- Companion patterns: Pattern 1 (existing arch-registry flag) and Pattern 4 (manual arch edits, no flag yet) per `.claude/docs/andromeda-after-mvp-playbook.md`.
- Implementation commit: `{this commit SHA — filled at commit time}`.

---

## Status: IMPLEMENTED — 2026-05-17 (session 78, commit pending) — first live dogfood test passed via chunk #62 cascade

### Proposal 5 — Type 7 evolve markers should pre-populate `expected_propagation: [CLAUDE.md]` when chunk count appears in pointer-table

**Problem:**

The pulse v0.2.0 dogfood has now run two consecutive `/andromeda-evolve --allow-route-append` invocations (chunk #57 session 67-68 Form 2, chunk #58 session 69 Form 1). Both produced markers with `expected_propagation: []` per the Type 7 baseline ("route additions typically don't cascade Tier 2/3"). Both then required `/andromeda-setup-project --delta` Detection step 8 grep-expansion (defense-in-depth) to catch the CLAUDE.md pointer-table chunk-count staleness:

- Session 67-68 (chunk #57): grep caught `CLAUDE.md:52` `(8 epochs / 55 chunks)` → `(9 epochs / 56 chunks)`.
- Session 69 (chunk #58): grep caught `CLAUDE.md:52` `(9 epochs / 56 chunks)` → `(9 epochs / 57 chunks)`.

Two-out-of-two is a strong signal that the marker's empty `expected_propagation` is undercount whenever the route's chunk count or epoch count is cited in CLAUDE.md pointer-table. The defense-in-depth grep is the safety net; relying on it indefinitely accumulates "almost-missed" cases and obscures the fact that the marker authoring path knows enough at write time to populate the cascade properly.

Recurring evidence is captured directly in this session's `.andromeda/runs/2026-05-16T15-04-39-setup-project-delta/materialization-plan-delta.md` audit-trail subsection "From Setup-detected stale-value grep matches (NOT in marker `expected_propagation`)".

**Proposal:**

Extend `/andromeda-evolve` Phase 4 step 2 (marker construction) to detect Type 7 chunk additions that affect CLAUDE.md pointer-table chunk count or epoch count, and pre-populate `expected_propagation: [CLAUDE.md]` accordingly.

Detection logic at evolve Phase 4 step 2:

1. If amendment is Type 7 (`flag_used: --allow-route-append`):
2. Grep `CLAUDE.md` for the pattern matching the pointer-table chunk-count cite — e.g., `\(\d+ epochs / \d+ chunks\)` or project-customized variant.
3. If pattern found AND amendment's `chunks_added` non-empty (Form 1 or Form 2 chunk addition) OR `new_epoch_created: true` (Form 2 epoch addition):
4. Pre-populate `expected_propagation` to include `CLAUDE.md` (specifically the GENERATED:setup:pointer-table block).

This makes the marker self-describing about its full propagation surface, removing reliance on setup-project's grep-expansion safety net for the most common pointer-table cascade.

**Design:**

The plan→file mapping table baseline in `delta-rerun-protocol.md` could ALSO be extended as an alternative path (add a row "route.md (when affecting chunk count or epoch count cited in pointer-table) → CLAUDE.md GENERATED:setup:pointer-table"). But per-marker detection is more precise — it only fires when grep finds an actual cite in CLAUDE.md, not a blanket assumption that may not match every project's pointer-table format.

Both layers (per-marker detection + grep-expansion defense-in-depth) keep the existing safety net intact while reducing how often it has to fire. Grep-expansion remains for cases the per-marker detection misses (unusual cite formats, new sections that future setup-project versions introduce, project-specific pointer-table phrasings).

**Implementation cost:**

| File | Change | Lines |
|---|---|---|
| `andromeda-evolve/SKILL.md` | Phase 4 step 2 — add pointer-table-grep step для Type 7 markers | ~15 |
| `andromeda-evolve/references/output-templates.md` | Type 7 marker variant — `expected_propagation` pre-populate logic + example | ~25 |
| `andromeda-setup-project/references/delta-rerun-protocol.md` | Plan→file table — add row for route chunk-count cascade (optional alternative path) | ~10 |
| (`spec-amendment-protocol.md` does NOT need changes — schema unchanged; only authoring logic shifts) | — | 0 |

**Total:** ~50 lines across 3 files. Small-effort enhancement; high-value because it eliminates the most common "expected_propagation undercount" pattern surfaced via consecutive dogfood evidence.

**When to do:**

Now-soon. Two consecutive dogfood Type 7 amendments hitting the same gap is enough signal to act; waiting for chunk #59 to make it "three-in-a-row" buys no additional insight. Bundling with Proposal 6 (Form 1 §1 staleness) makes sense since both touch Type 7 marker authoring correctness — one meta-Andromeda enhancement session covers both.

**Cross-references:**

- Recurring pattern evidence: session 68 wrap Tier 3 entry "First /andromeda-setup-project --delta dogfood validates grep-expansion design" + session 69 setup-project --delta materialization-plan-delta.md "SECOND consecutive --delta run with same pattern" note.
- Companion improvement: Proposal 6 (Form 1 §1 staleness) — different aspect of same Type 7 marker-authoring correctness theme.
- Triggering chunks: #57 (session 67-68, Form 2) + #58 (session 69, Form 1) — both Form 1 AND Form 2 surfaced the same pointer-table cascade gap, so the proposed pre-populate logic applies к both forms.
- Related skill mechanism: `delta-rerun-protocol.md` §Detection step 8 grep-expansion (the defense-in-depth safety net this proposal would shift load off of).

---

## Status: IMPLEMENTED — 2026-05-17 (session 78, commit pending) — first live dogfood test passed via chunk #62 cascade

### Proposal 6 — Form 1 chunk-append should auto-update `§1 Total chunks` line (mirror Form 2 mechanical behavior)

**Problem:**

`/andromeda-evolve --allow-route-append` Form 1 (chunk append to existing epoch) currently does NOT auto-update route.md §1 "Route Scope Summary → Total chunks" count line. Only Form 2 (terminal new epoch creation) mechanically auto-updates §1 per the explicit Proposal 4 spec.

This asymmetry creates accumulating §1 staleness as Form 1 amendments land over time:

- Chunk #44 amendment (2026-05-11, Form 1): left §1 stale at "Total chunks: 55" while §2 contained 56. Documented in amendment Decisions Log: "§1 Route Scope Summary 'Total chunks: 55' becomes stale; will refresh at next /andromeda-route or via manual edit."
- Chunk #57 amendment (2026-05-16, Form 2): mechanically bumped §1 к "Total chunks: 55 → 56" (Form 2 auto-update; ALSO carried the pre-existing chunk-#44 staleness forward by NOT correcting к 56 → 57).
- Chunk #58 amendment (2026-05-16, Form 1, this session): leaves §1 at "Total chunks: 56" while §2 is now 57. Same staleness pattern as chunk #44 repeats.

Pattern: every Form 1 amendment compounds §1 staleness by +1. Each amendment's Decisions Log explicitly documents the staleness as "intentional pending next `/andromeda-route` re-generation or manual edit", which treats user vigilance as the safety net — a known-bad workaround.

The asymmetry between Form 1 and Form 2 has no clear rationale. Both forms add chunks, and §1 "Total chunks: N" is a derived count over §2. Mechanically bumping +1 is safe for either form. The original Proposal 4 spec's restriction ("§1 mechanical update only on Form 2") was likely a conservative scope-limit at design time, not a load-bearing invariant.

**Proposal:**

Extend Form 1's mechanical auto-update к ALSO touch §1 "Total chunks: N" line, mirroring Form 2's behavior for this specific line. Keep "Epochs: N (...)" line untouched in Form 1 (no epoch creation, so no Epochs line change needed — Form 2 still owns that auto-update).

Concretely, evolve Phase 6 atomic write для Form 1:

- Currently: insert chunk + `↓` separator into §2 epoch body + append Decisions Log entry к §3.
- Add: edit §1 "Total chunks: N" → "Total chunks: N+M" where M = chunks added (typically 1; can be >1 for multi-chunk Form 1 batches).

Form 2 behavior unchanged (already auto-updates both Total chunks + Epochs lines).

**Design:**

This is a one-line mechanical edit at Phase 6 atomic write time. Pattern matches Form 2's existing implementation:

- Read §1 "Total chunks: N" line via regex `^- \*\*Total chunks:\*\* (\d+)$`.
- Substitute N → N+M.
- Atomic write.

**Edge case — pre-existing §1 staleness:** if §1 chunk count differs from §2 count BEFORE this amendment (e.g., current state where §1=56 but §2=57 expected after chunk #58 amendment), the auto-update implementation could choose between two policies:

- **Policy A — strict mechanical:** §1 = §1 + M (preserves pre-existing staleness; cleanly mechanical; doesn't surprise user).
- **Policy B — corrective:** §1 = count(§2) + M after recount (catches up multiple prior Form 1 amendments at once; corrective drift fix; less mechanical, more "smart").

Recommend Policy A (strict mechanical) for consistency with Form 2 + simpler reasoning. Pre-existing staleness can be corrected separately via `/andromeda-route` re-run when it accumulates beyond comfort.

Decisions Log Impact field text changes: drop the "(§1 Route Scope Summary 'Total chunks: N' remains stale pending next /andromeda-route re-generation OR manual edit ... Form 1 mechanical interpretation does NOT auto-update §1)" language. Replace with simple "§1 Total chunks: N → N+M (Form 1 mechanical auto-update)".

**Implementation cost:**

| File | Change | Lines |
|---|---|---|
| `andromeda-evolve/SKILL.md` | Phase 6 step 4 — extend Form 1 atomic write to touch §1 Total chunks | ~10 |
| `andromeda-evolve/references/output-templates.md` | Type 7 marker variant — Form 1 Plans amended block + Decisions Log entry template updates | ~20 |
| `andromeda-evolve/references/refuse-taxonomy.md` | §Refuse 6 Exception → Form 1 specification — extend "purely additive" scope к include §1 Total chunks mechanical edit | ~10 |
| `andromeda-evolve/references/validation-checks.md` | Check 8.1 (purely additive) clarification — Form 1 §1 mechanical edit permitted | ~5 |

**Total:** ~45 lines across 4 files. Small-effort mechanical fix; high-value because it eliminates a recurring "intentional staleness" workaround that compounds with every Form 1 amendment.

**When to do:**

Bundle with Proposal 5 (Type 7 expected_propagation pre-populate) — both are about Type 7 marker authoring correctness; one meta-Andromeda enhancement session covers both. Combined effort: ~95 lines across ~5 distinct files, well within a single ~1 hour focused session.

Alternative: independently before chunk #59 lands. Each additional Form 1 amendment compounds §1 staleness by +1; defer cost grows linearly. Once landed, the §1 vs §2 sync invariant becomes always-true after each evolve invocation again.

**Cross-references:**

- Recurring pattern evidence: route.md §3 Decisions Log entries для chunks #44 + #58 — both explicitly document intentional §1 staleness as known workaround in the Impact field.
- Companion improvement: Proposal 5 (Type 7 expected_propagation pre-populate) — different aspect of same authoring-correctness theme.
- Triggering chunks: chunk #44 (session 51) + chunk #58 (session 69) — Form 1 amendments accumulating §1 staleness across multiple wraps.
- Related spec: Proposal 4 (Form 2 mechanism) intentionally restricted §1 auto-update к Form 2; this proposal revisits that restriction with two-amendment evidence base.
- Current state.yaml.spec_amendments.archive[0] entry (chunk #58 amendment): preserves audit trail of this session's exact staleness instance for future readers.

---

_(Subsequent proposals appended below in chronological order. Each proposal has its own `## Status:` heading and `### Proposal N — title` subheading for navigability.)_

---

## Status: IMPLEMENTED — 2026-05-22 (session 116, commit pending) — Option A numeric auto-update + Option B word-form warning + Type 6 marker `Narrative-cascade auto-update` subsection landed via chunk #76 batch

### Proposal 7 — Type 6 narrative-cascade visibility (arch.md structural section count lines)

**Problem:**

`/andromeda-evolve --allow-arch-registry` (Type 6 amendments) registers a new item in a registry section (e.g., `§Occupied Resources` Cargo workspace crate names list). The flag is narrowly scoped to registry-section additions only — structural sections like `§Design Philosophy` and `§Inherited Defaults` and `§Cross-cutting Patterns` remain refused per Check 7.2.

But arch.md narrative content in structural sections often references the SAME count that the registry list contains. For pulse:

- `§Design Philosophy` paragraph 2 says: "eight library crates linked into the pulse-app Tauri binary (ten workspace members total: eight library crates + the pulse-app binary + the xtask task-runner crate)." After chunk #58 added curation crate, this is stale — should be "nine library crates ... eleven workspace members".
- Same arch.md self-documents the tradeoff: "Occupied Resources is the canonical workspace-member list; any claim of a count word elsewhere refers back to it." So arch.md ACKNOWLEDGES the staleness risk but Type 6 flag mechanics give no automated path to keep narrative counts synced.

Two dogfood instances so far:

1. **2026-05-11 chunk #43 / pulse:clipboard amendment** — added a Tauri capability identifier to `§Occupied Resources` capability registry. arch.md `§Cross-cutting Patterns` "Webview IPC capability policy" narrative paragraph cites the capability set (`pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`) — adding `pulse:clipboard` to the registry made this narrative incomplete (missing `pulse:clipboard`). Not fixed at the time.
2. **2026-05-16 chunk #58 / curation amendment** — added a workspace crate name to `§Occupied Resources` Cargo workspace crate names list. arch.md `§Design Philosophy` + `§Inherited Defaults` narrative count lines became stale ("eight" / "ten" — should be "nine" / "eleven"). Not fixed at this session — surfaced as warning in `/andromeda-setup-project --delta` materialization-plan-delta.md "Out-of-Type-6 staleness surfaced" subsection but accepted as deferred per arch.md's own "Occupied Resources is canonical" tradeoff documentation.

The recurring pattern: Type 6 registry additions cascade to arch.md structural narrative numbers. Type 6 flag can't fix that. Result: stale narrative counts accumulate one per amendment.

**Proposal:**

Extend `/andromeda-evolve --allow-arch-registry` Check 7 to detect arch.md narrative count lines that reference the registry section's content, AND either:

- **Option A — auto-update narrative count lines mechanically** (preferred): when the registry addition increments a count visible in narrative sections (e.g., "N library crates" / "N workspace members" / "N capability identifiers"), apply a mechanical regex-replace in those narrative sections too. Document the auto-update in the marker's Plans amended block + Decisions Log entry.

- **Option B — surface as strict warning** (fallback): if auto-update is too aggressive for some narrative idioms, at least surface the inconsistency clearly: "arch.md §Design Philosophy paragraph 2 references count N; registry section now has N+1 entries; either accept narrative staleness OR run /andromeda-arch to re-derive narrative."

The current materialization-plan-delta.md "Out-of-Type-6 staleness surfaced" subsection (from session 70) is a manual ad-hoc version of Option B; making it a built-in evolve Check would standardize the audit trail and prevent silent compounding.

**Design:**

Option A implementation:

1. At `/andromeda-evolve --allow-arch-registry` Check 7 time, after verifying the registry section addition (Check 7.1-7.4), scan arch.md narrative sections (§Design Philosophy / §Inherited Defaults / §Cross-cutting Patterns / §Project Intent — the structural sections refused by Check 7.2) for count-reference patterns.
2. Count-reference patterns for pulse: `(\d+|eight|nine|ten|eleven|...) (library crates|workspace members|capability identifiers|TauRPC procedures|...)` — regex matches the count noun phrase used in narrative.
3. For each matched count, compare against the post-amendment registry count.
4. If mismatch:
   - **If count is in numeric form ("8")**: mechanical replace to new value. Atomic edit alongside Phase 6 registry-section write.
   - **If count is in word form ("eight")**: surface as warning ("arch.md §X line N: 'eight library crates' may be stale; word-form auto-replace not attempted; verify manually or run /andromeda-arch"). Word forms are higher-risk for false positives.
5. Document in marker's Plans amended block:
   ```
   - Narrative-cascade auto-update:
     - §Design Philosophy paragraph 2: "8 library crates ... 10 workspace members" -> "9 library crates ... 11 workspace members"
   ```
6. Decisions Log entry includes "Narrative-cascade updates applied" subsection.

Option B implementation:

1. Same scan as Option A.
2. For ALL mismatches (numeric + word form): surface as Check 7 warning (NOT failure — Check 7 still passes). Render in materialization-plan-delta.md "Narrative-cascade staleness" subsection.
3. Phase 7 user review shows the warnings; user can decide accept-and-defer OR cancel-and-fix-via-/andromeda-arch.

**Implementation cost:**

| File | Change | Lines |
|---|---|---|
| `andromeda-evolve/SKILL.md` | Phase 3 step 7 - narrative-cascade detection sub-step | ~15 |
| `andromeda-evolve/references/validation-checks.md` | Check 7 extension - narrative-cascade sub-check | ~30 |
| `andromeda-evolve/references/output-templates.md` | Type 6 marker variant - Plans amended block narrative-cascade addition | ~20 |
| `andromeda-evolve/references/refuse-taxonomy.md` | Refuse 1 Exception narrative-cascade clarification | ~10 |

**Total:** ~75 lines across 4 files. Option B (surface-only) would be ~half that.

**When to do:**

Bundle with Proposal 5 + 6 (Type 7 expected_propagation pre-populate + Form 1 §1 auto-update). All three are about marker-authoring correctness across the evolve flag variants. One ~2-hour focused meta-Andromeda session can implement all three.

Alternative timing: defer until the next Type 6 amendment surfaces this gap again. Each future arch §Occupied Resources addition that affects a narrative count compounds the staleness by +1; deferring cost grows linearly.

**Cross-references:**

- Recurring pattern evidence: arch.md `§Architecture Registry Updates` 2026-05-11 (pulse:clipboard) + 2026-05-16 (curation) - both registry additions cascaded to structural narrative sections that the Type 6 flag could not update.
- Companion improvements: Proposal 5 (Type 7 expected_propagation pre-populate) + Proposal 6 (Form 1 §1 auto-update). All three are about marker-authoring correctness for the flag-variants.
- Triggering chunks: chunk #43 (session 51) + chunk #58 (session 70) - Type 6 amendments accumulating narrative-section staleness across multiple wraps.
- Related artifact: `.andromeda/runs/2026-05-16T16-34-14-setup-project-delta/materialization-plan-delta.md` "Out-of-Type-6 staleness surfaced" subsection documents the current ad-hoc manual surface-pattern.
- Current state: state.yaml.spec_amendments.archive contains the chunk #58 amendment (archived this session) with `flag_used: --allow-arch-registry` - preserves audit trail for future readers studying this proposal's context.

## Status: PHASE 1 IMPLEMENTED — 2026-05-17 (session 82, commit pending); Phase 2 still PROPOSED — first author-time compact-template + Check 7.6 conformance landed via P8/P9 bundle

### Proposal 8 — Arch Registry Updates section compaction strategy

**Status:** PHASE 1 IMPLEMENTED 2026-05-17 (session 82, commit pending) — compact Type 6 Decisions Log entry template + Check 7.6 conformance landed in `~/.claude/skills/andromeda-evolve/` (output-templates.md / validation-checks.md / SKILL.md / refuse-taxonomy.md / dialog-templates.md). Phase 2 (sliding-window demotion of older entries to single-line summaries, wrap-session demotion logic + threshold detection) still PROPOSED — observed session 79, threshold not yet reached. The Phase 1 body below preserves the original problem statement and design as historical record per this doc's convention.

**Status (original):** PROPOSED (observed session 79, threshold not yet reached)

**Problem:** §Architecture Registry Updates accumulates verbose entries (~15 lines each). 7 entries currently (109 lines, 33% of arch.md). Linear forecast: 30-40 additional entries through v0.2.0 ship → section grows to 500-700 lines, 50%+ of arch.md. Reader scans become slow, Established Decisions buried, precedent citations require long scroll-back.

**Proposal:** Two-phase intervention:

Phase 1 (immediate, no machinery): Tighten new entry format going forward. Drop Authority paragraph boilerplate, condense Rationale to single sentence + precedent citation. Target ~5-6 lines per new entry instead of 15. Existing entries untouched.

Phase 2 (deferred until ~15-20 active entries threshold): Implement sliding window of detail. Last N=10-15 entries verbose (full current format). Older entries compact-demoted: single line summary + marker path reference. Demotion via wrap-session phase step or manual evolve action.

**Design:**
- Phase 1: style discipline change, no code change. Update evolve output templates.
- Phase 2: detection at wrap-session Phase X — if §Architecture Registry Updates entry count > threshold (configurable, default 15), demote oldest verbose entry to compact format. Marker paths preserved as forensic access path.
- **Template artifacts (NEW):** generate concrete output template files for both compact verbose entry format (Phase 1) and compact-demoted single-line entry format (Phase 2). Templates live in `~/.claude/skills/andromeda-evolve/references/output-templates.md` (extend existing file) and become authoritative source consumed by evolve dialog flow + wrap-session demotion logic. Update skill SKILL.md files (`andromeda-evolve`, `andromeda-wrap-session`) to reference templates as canonical output format. Skills must enforce templates as default — agent-improvised format becomes refusal at Check 7 validation (compact format conformance check).

**Implementation cost:**
- Phase 1: ~30 LOC in evolve output-templates.md + style guide update + **compact verbose entry template authoring (~15 LOC of template + example)**
- Phase 2: ~150-250 LOC in wrap-session SKILL.md Phase X + demotion logic + **compact-demoted entry template authoring (~10 LOC of template + example)** + Check 7 conformance validation hook

**When to do:**
- Phase 1: next Type 6 amendment cycle (consciously author new entry in tighter format; if works without forensic loss, codify in evolve templates)
- Phase 2: chunk #67-#70 timeframe (when ~12-15 entries accumulated)

**Cross-references:** Related to Proposal X (state.yaml archive list compaction) if exists; same theme of "post-MVP artifact growth needing strategic compaction". Session 79 user observation.

## Status: PHASE 1 IMPLEMENTED — 2026-05-17 (session 82, commit pending); Phase 2 still PROPOSED — first author-time compact-template + Check 8.5 ack + Check 8.8 conformance landed via P8/P9 bundle

### Proposal 9 — route.md §3 Decisions Log + §2 Roadmap entry compaction strategy

**Status:** PHASE 1 IMPLEMENTED 2026-05-17 (session 82, commit pending) — both Phase 1(a) (§2 chunk text 25-word constraint elevated from quiet WARNING to Phase 5 ack-required surface) and Phase 1(b) (compact Type 7 Decisions Log entry template + Check 8.8 conformance) landed in `~/.claude/skills/andromeda-evolve/` (output-templates.md / validation-checks.md / SKILL.md / refuse-taxonomy.md / dialog-templates.md). Phase 2 (Epoch 1-8 archival via `<details>` HTML folding or separate `route-archive-v0_1_0.md`, post-v1.0 ship) still PROPOSED. The Phase 1 body below preserves the original problem statement and design as historical record per this doc's convention.

**Status (original):** PROPOSED (observed session 79, threshold not yet reached)

**Problem:** route.md accumulates verbosity in two locations. §2 Roadmap chunks grew from 12-25 words (Epoch 1-8 baseline) to 60-80 words (Epoch 9 chunks #57-#63), violating documented 25-word-single-line constraint; verbosity encodes file paths + capability lists + behavioral notes + test scenarios that already exist in pulse-v0_2_0-route.md. §3 Decisions Log entries average 7-8 lines per chunk append with significant boilerplate (chunk text duplicated from §2, "/implement standard flow" reference, "Trigger 4 spec-drift-protocol if drift surfaces" repeated paragraph, Form classification name + Refuse 6 exception citation redundant with flag name). Currently 277 lines total; linear forecast through v0.2.0 ship: 530+ lines with §3 Decisions Log reaching 60% of document. Route.md read every chunk planning session — high read frequency multiplies cognitive cost per verbose entry, more impactful than arch.md (Proposal 8 sibling).

**Proposal:** Two-phase intervention paralleling Proposal 8 (arch.md compaction):

Phase 1 (immediate, no machinery): Tighten new entries in two locations:

(a) §2 Roadmap chunk text: enforce 25-word single-line constraint. Move file paths + capability descriptions + behavioral notes + test scenarios to pulse-v0_2_0-route.md (where they already exist in fuller form). §2 chunks become glance-table summaries, not duplicate detailed spec. Target ~15-22 words per chunk.

(b) §3 Decisions Log entries: drop boilerplate (chunk text duplication → reference §2 line N instead; "/implement against chunk #X via /andromeda-phase + /andromeda-implement standard flow" repeated wording; "Form 1 = chunk append to existing epoch" tautology with classification name; "Type 7 narrow Refuse 6 exception per refuse-taxonomy.md" citation chain). Target ~4-5 lines per entry instead of 7-8.

Existing entries untouched.

Phase 2 (deferred until post-v1.0 ship): Epoch 1-8 collapse to archival form. These epochs CLOSED (v0.1.0 shipped). Individual chunks still useful as numerical reference ("see chunk #29 pattern"), so collapse via `<details>` HTML folding (or separate `route-archive-v0_1_0.md`) rather than deletion. Keep chunk lookup accessible, reduce visual scan area in active route.md. Recent epochs (9+) retain full detail.

**Design:**
- Phase 1(a): chunk-text style discipline change, no code change. Update evolve dialog template for `--allow-route-append` to surface 25-word constraint as Check 8.X validation (currently soft constraint, would become enforced refusal on overflow).
- Phase 1(b): output-templates.md update for Type 7 amendment marker generation. Compact entry format becomes default.
- Phase 2: archival mechanism — manual edit OR scripted `<details>` wrap injection. Triggers on user decision after v1.0 milestone, not automatic.
- **Template artifacts (NEW):** generate concrete output template files for compact §2 chunk text format (Phase 1a) and compact §3 Decisions Log entry format (Phase 1b). Templates live in `~/.claude/skills/andromeda-evolve/references/output-templates.md` (extend existing file alongside Proposal 8 templates) and become authoritative source consumed by evolve dialog flow for `--allow-route-append`. Update `andromeda-evolve` SKILL.md to reference templates as canonical output format for Type 7 Form 1 + Form 2 amendments. Skill must enforce templates as default — agent-improvised format becomes refusal at Check 8.X validation (compact format conformance check, parallel to 25-word constraint enforcement). Phase 2 archival mechanism includes its own template for `<details>`-wrapped Epoch archival blocks.

**Implementation cost:**
- Phase 1(a): ~20 LOC validation in evolve Check 8 family + dialog template update + style guide + **compact §2 chunk text template authoring (~8 LOC of template + example)**
- Phase 1(b): ~40 LOC in evolve output-templates.md + Type 7 entry generator + **compact §3 entry template authoring (~12 LOC of template + example)**
- Phase 2: ~80-150 LOC if scripted (archival section generator + chunk lookup preservation) + **Epoch archival template authoring (~15 LOC of template + example)**; ~10 minutes manual edit alternative

**When to do:**
- Phase 1: next route-append amendment cycle (chunk #64, likely 1-2 sessions ahead — try compact format manually, validate forensic feel before codifying in templates)
- Phase 2: post-v1.0 ship (when Epoch 1-8 reference frequency naturally drops; Epoch 9 still actively referenced)

**Cross-references:** Sibling to Proposal 8 (arch.md §Architecture Registry Updates compaction) — same theme of "post-MVP artifact growth needing strategic compaction" but route.md higher-frequency-read so larger cognitive ROI per tightened entry. Session 79 user observation, route.md current state 277 lines / 7 §3 entries / 7 Epoch 9 chunks.

## Status: PROPOSED — 2026-05-17 (session 82)

### Proposal 10 — `/andromeda-setup-project --delta` should detect non-delta-scoped uncommitted work and surface guidance

**Problem:**

`/andromeda-setup-project --delta` protocol assumes a clean working tree at invocation time — the only uncommitted changes should be the delta-scoped files derived from pending spec amendments. Phase 9 says "stage only delta-scoped files (per materialization-plan-delta.md 'Files touched') + state.yaml + marker files (with updated lifecycle status) + run-dir".

But the practical reality of multi-task sessions is that uncommitted work accumulates between commit boundaries. Session 82 dogfood example: between the prior wrap commit (`61ca564` session 81 chunk #63 wrap) and the `/setup-project --delta` invocation, the working tree carried THREE distinct uncommitted streams:

1. P8+P9 Phase 1 retroactive compact-format refactor (arch.md 7 verbose → compact entries + route.md §1 staleness fix + §2 Epoch 9 word-tightening + §3 8 verbose → compact entries) — cosmetic refactor pass executed in plan mode earlier in session
2. `docs/andromeda-improvements.md` Proposal 8/9 status updates (PROPOSED → PHASE 1 IMPLEMENTED) — companion to skill changes landed at `~/.claude/skills/andromeda-evolve/`
3. The `/evolve --allow-arch-registry` amendment writes (arch.md §Occupied Resources inline list + new §Architecture Registry Updates entry + state.yaml +1 entry)

Only stream (3) is "delta-scoped". Streams (1) and (2) are non-delta work that happened to be uncommitted at /delta invocation time.

The protocol's "stage only delta-scoped files" assumes single-task sessions. In multi-task sessions, splitting via `git add -p` is technically possible but RISKY for compounded edits to the same file — session 82's arch.md had BOTH retroactive refactor edits AND the new Type 6 amendment edits; both touched §Architecture Registry Updates but in different ways (refactor rewrote 7 historical entries; amendment appended an 8th). Interactive splitting risks misattributing edits across two commits.

**Proposal:**

Extend `/andromeda-setup-project --delta` Phase 9 to detect non-delta-scoped uncommitted files BEFORE the commit step, and surface guidance for the user to choose between three resolutions:

- **(A) Bundle into one commit (default if user accepts):** stage all uncommitted files; compose commit message that documents both streams (delta-rerun primary + bundled secondary). Add a `## Bundled uncommitted work` subsection to the commit message body listing the non-delta files + heuristic categorization (e.g., "cosmetic refactor" / "status update" / "unknown").

- **(B) Halt with diagnostic + manual split:** display the non-delta files with file paths + brief diff stats; suggest the user run `git stash` to isolate delta-scoped work, commit /delta cleanly, then unstash + commit the rest separately. Useful when the user wants clean two-commit history.

- **(C) Explicit `--bundle-uncommitted` flag (advanced):** user pre-authorizes bundling; skill skips the prompt and bundles directly with `## Bundled uncommitted work` subsection. Useful for autonomous-execution flows where pausing for prompts is expensive.

Default behavior without flag: prompt user at Phase 9 (interactive resolution); autonomous-execution directive in effect → default to (A) with explicit logging.

**Design:**

Phase 9 step (new, inserted before existing `git add` step):

1. Compute delta-scoped file list from `materialization-plan-delta.md` "Files touched (delta scope)" section + state.yaml + marker files + run-dir contents.
2. Compute non-delta uncommitted files: `git diff --name-only HEAD` minus delta-scoped list.
3. If non-delta list is empty: proceed with standard /delta commit (no change).
4. If non-delta list is non-empty:
   - Render diagnostic listing the non-delta files + diff stats per file
   - Prompt user: `Bundle into /delta commit? (Y/N/manual-split)` (default Y)
   - On Y: stage all uncommitted files; compose comprehensive commit message with `## Bundled uncommitted work` subsection
   - On N: halt; user resolves manually
   - On manual-split: render `git stash` + commit + unstash + commit sequence guidance; halt for user execution
5. Record the bundling decision in `materialization-plan-delta.md` audit trail.

**Implementation cost estimate:**

| File | Change | Lines |
|---|---|---|
| `andromeda-setup-project/SKILL.md` | Phase 9 — add detection + prompt + bundle logic | ~30 |
| `andromeda-setup-project/references/delta-rerun-protocol.md` | Add "Non-delta uncommitted detection" subsection + "Bundled uncommitted commit message variant" subsection | ~50 |
| `andromeda-setup-project/references/visual-references.md` | Phase 9 bundle-prompt template | ~15 |

**Total:** ~95 lines across 3 files. Small-effort enhancement; high-value because it eliminates a recurring "what do I do with my non-delta uncommitted work" friction at /delta invocation time.

**When to do:**

Now-soon. Session 82's bundled commit was the first observed instance + the rationale was documented inline in the commit message body + surfaced in the post-Phase-9 report. But the friction will recur whenever a session has multiple task streams (common in dogfood multi-improvement sessions like 82). Each future occurrence costs the agent (and user) cognitive overhead deciding between split vs bundle without skill-level guidance.

Alternative timing: defer until the second observed instance to confirm the pattern recurs. But pulse v0.2.0's remaining 26 chunks each could trigger multi-stream sessions when amendments cascade across multiple skill files; the proposal becomes increasingly valuable as the project moves through Epoch 9.

**Cross-references:**

- Triggering session: session 82 (`/andromeda-setup-project --delta` invocation, commit `ac09308`) — first observed bundled-uncommitted scenario in pulse v0.2.0 dogfood
- Related companion in session-learnings.md (Tier 3): "2026-05-17 (session 82) — Bundled `--delta` commit when prior uncommitted refactor exists" (workflow-lesson framing of same friction; this proposal is the skill-mechanic-enhancement framing)
- Sibling proposals: Proposal 5 (Type 7 expected_propagation pre-populate) + Proposal 6 (Form 1 §1 mechanical update) — same theme of "/andromeda-evolve + /andromeda-setup-project --delta authoring-time correctness"; this proposal extends the theme to /delta commit-time correctness

## Status: PROPOSED — 2026-05-17 (session 84)

### Proposal 11 — `/andromeda-phase` Phase 2 merge-protocol cross-extract evidence-consistency check (obs ↔ existing AllowList registry)

**Problem:**

Phase 1 sub-agents in `/andromeda-phase` work independently — the obs / security / tests / arch / a11y / design / layouts extractors each filter their own specialist plan to chunk relevance without consulting the existing CODEBASE STATE (e.g., the actual `pulse-app/src/observability.rs::AllowList::production()` registry contents that the project has accumulated over prior chunks). Phase 2 merge protocol's Cross-domain rot scan catches three patterns:

- Pattern 1: stale Decisions Log references (deprecates X but other extract cites X)
- Pattern 2: security ban without matching test trigger
- Pattern 3: shared concept with different vocabulary across extracts

But Pattern 1-3 don't catch a recurring class of friction: obs extract requests fields that the existing codebase AllowList would reject. The obs sub-agent reads obs-plan.md (which authorizes per-service fields under §5 "exception for query-time aggregation metrics — service_name is intentionally unbounded") but doesn't reconcile against the more-stringent convention that chunks #62/#63 ESTABLISHED in the actual implementation (`triage.cue.emit` deliberately excludes `scope_id` even though the AttentionCue payload carries it). The plan inherits the obs extract's per-service-field request verbatim; user Phase 6 review approves without seeing the conflict; /implement Phase 1 surfaces the conflict at code-writing time and the implementer chooses Path A' (fix impl to match existing precedent) without specialist-plan amendment.

Session 84 dogfood example: chunk #64 obs extract called for `service_went_silent.emit` target with `fields.service_name` + `fields.quiet_duration_ms` + `fields.p95_threshold_ms` + `fields.bootstrap_state` per-event. Plan acceptance criteria inherited this. /implement Phase 1 chose Path A' — ServiceWentSilent cues flow through existing `triage.cue.emit` target (chunk #62 precedent, no per-service fields in tracing); new chunk #64 events are aggregate-only (3 new allowlist entries vs plan's 7). Surfaced in /implement Phase 3 report; user accepts at wrap-session review. Cost: implementer cognition (recognize the conflict + design correction), report surface area (Path A' note in Phase 3 report + this proposal). Not catastrophic; recurs whenever a chunk adds new tracing targets in any namespace where prior chunks established a tighter PII convention than the literal obs-plan.

**Proposal:**

Extend `/andromeda-phase` Phase 2 merge-protocol Step 8 (Cross-domain rot scan) with a fourth pattern:

Pattern 4 — Obs extract field request conflicts with existing AllowList registry:

1. Parse obs extract `## Constraints` + `## Acceptance criteria contributions` sections for any tracing target naming convention pattern (`triage.{module}.{operation}` / `metric.{module}.{measure}` / etc.) accompanied by field specifications.
2. Cross-reference against the existing `pulse-app/src/observability.rs::AllowList::production()` registry: parse `by_target.insert(target, [fields].iter()...)` entries for the same target namespace family (e.g., all `triage.cue.*` entries).
3. Build a registry-derived "established convention" summary per namespace (e.g., "triage.cue.* entries permit `kind`, `priority`, `scope`, `magnitude`, etc. but exclude `service_name` / `scope_id`").
4. Flag conflict: "Pattern 4 — Obs extract requests field `{field}` for new target `{target}` in namespace `{ns}`; existing AllowList registry for `{ns}` entries does not admit `{field}` (chunks #X/#Y precedent). Plan implementer must choose: (a) extend AllowList convention (security-plan §Logging review needed); (b) follow precedent (rephrase obs criterion as aggregate-only). Default-recommendation: (b) per established codebase pattern."

User Phase 6 review sees the conflict explicitly + decides direction BEFORE /implement Phase 1 surfaces it.

**Design:**

- Phase 2 merge-protocol Step 8 gains a new pattern detector.
- Implementation requires read access to `pulse-app/src/observability.rs` (or stack-equivalent registry file) from the orchestrator — already present in Phase 3 codebase research scope (`/andromeda-phase` orchestrator reads source files anyway).
- Pattern 4 lives ONLY in `/andromeda-phase` Phase 2 merge protocol; does NOT modify Phase 1 sub-agent prompts (sub-agents stay scoped to their specialist plan; cross-extract reconciliation is orchestrator territory per Phase 2 design).
- Output renders in `combined.md` §Cross-domain rot warnings section + Phase 6 user review summary surfaces Pattern 4 hits.
- Generalizes beyond obs ↔ AllowList — the pattern is "Phase 1 sub-agent extract proposes shape conflicting with codebase-established convention captured in registry-like files". Future variants: tests extract proposes `#[tokio::test(start_paused = true, flavor = "multi_thread")]` (codebase precedent testing.md 2026-05-06 shows incompatible combination); design extract proposes a new color token (codebase precedent design-system.md already locked palette).

**Implementation cost estimate:**

| File | Change | Lines |
|---|---|---|
| `andromeda-phase/references/phase-2/merge-protocol.md` | Add §Pattern 4 — Obs extract field request vs existing AllowList registry; describe detection + output format | ~40 |
| `andromeda-phase/SKILL.md` | Phase 2 Step 8 — extend pattern list to include Pattern 4; orchestrator instructions to read AllowList file | ~10 |
| `andromeda-phase/references/visual-references.md` | Phase 6 review banner — show Pattern 4 hits prominently when present | ~10 |

**Total:** ~60 lines across 3 files. Moderate-effort enhancement; high-value because it shifts the cognitive cost of conflict detection from /implement-time to /phase-time (cheaper because user is reviewing the plan anyway; avoids the surprise + Path A' explanation in /implement Phase 3 report).

**When to do:**

- Optionally now-soon. Confidence that this pattern recurs is medium (one observed instance: session 84 chunk #64). The fix is small but pays off only when the pattern actually fires.
- Defer-until-second-observation alternative — wait until chunk #65 or later adds another tracing namespace AND triggers another Pattern 4-style /implement Path A'; bundle two observations into the same proposal-to-implementation cycle for higher signal-noise ratio.
- Author preference — lean toward defer; single observation is weak evidence of recurring friction. Chunk #65 is span-events ingestion which doesn't touch the cue/baseline namespace where #62-#64 established the precedent, so the next likely trigger is post-Phase-3 (LLM interpretation layer) chunks.

**Cross-references:**

- Triggering session: session 84 (chunk #64 implementation; /implement Phase 3 report documented the Path A' correction; this proposal extends the lesson to /phase Phase 2 merge protocol)
- Companion in `.claude/rules/observability.md` Session Additions (Tier 2 rule): "2026-05-17 (session 84): Chunks #62/#63/#64 established a TIGHTER AllowList convention..." — that rule tells chunk implementers about the convention; this proposal proposes catching the conflict at planning-time so implementers don't need to discover it at coding-time
- Sibling proposals: Proposal 7 (Type 6 narrative-cascade visibility) — same theme of "make implicit conventions visible to planning-time machinery so /implement doesn't surface them post-hoc"

---

## Status: IMPLEMENTED — 2026-05-22 (session 116, commit pending) — Phase 4 step 2h Type 6 → CLAUDE.md cascade detection + output-templates.md Branch (a)/(b) split + validation-checks.md Check 7.7 + delta-rerun-protocol.md Type 6 permit Branch (b) clarification landed via chunk #76 batch

### Proposal 12 — Type 6 evolve markers should pre-populate `expected_propagation: [CLAUDE.md]` when registry section appears in CLAUDE.md derived sections (Modules / Stack / Key directories)

**Problem:**

`/andromeda-evolve --allow-arch-registry` (Type 6 amendments) registers a new item in an arch.md registry section (e.g., a new workspace crate in §Occupied Resources Cargo workspace crate names). The amendment marker's `expected_propagation` field is typically empty per the Type 6 default — per `spec-amendment-protocol.md` Part D Narrow exception: "Type 6 amendments typically have empty or minimal `expected_propagation` (registry-section additions don't usually cascade through Tier 2/3); --delta MUST still progress the lifecycle... so wrap-session Phase 8 can archive cleanly."

But CLAUDE.md mirrors arch.md §Inherited Defaults Workspace crates list in its Stack one-liner ("N library crates"), Key directories enumeration ("`crates/` — N library crates (...)") and Modules section (one bullet per crate with module purpose). When a Type 6 amendment adds a new crate to arch §Occupied Resources, those CLAUDE.md derived sections silently desync. `--delta` mode preserves CLAUDE.md byte-identical (per literal protocol with empty expected_propagation), accumulating staleness.

Pulse dogfood evidence:

1. **Chunk #58 / curation crate** (2026-05-16) — added to arch §Occupied Resources via Type 6. CLAUDE.md Modules section eventually was updated to include curation (likely manually as "bundled evolve work" per commit `4539ba8` style), but Stack one-liner count didn't track perfectly.
2. **Chunk #60 / triage crate** (2026-05-16) — same pattern; manual fixup bundled.
3. **Chunk #68 / corpus + security crates (this session 94)** — Type 6 amendment propagated via setup-project --delta with empty expected_propagation. Post-propagation: CLAUDE.md Modules section OMITS corpus + security entries; Stack one-liner still reads "10 library crates" though reality post-amendment is 12. Materialization-plan-delta.md surfaced this as known-follow-up under "Known pre-existing CLAUDE.md staleness" but no automated cascade was triggered.

The `delta-rerun-protocol.md` Detection step 8 grep-expansion is defense-in-depth, but it targets OLD VALUES being replaced (e.g., hex `#8B2E3B` for color-token lift). For PURELY-ADDITIVE Type 6 amendments to enumeration lists, there's no "old value" to grep for — the gap is what's MISSING in CLAUDE.md (the new crates), not a stale value being replaced.

**Proposal:**

Extend `/andromeda-evolve --allow-arch-registry` Phase 4 artifact construction (parallel to Proposal 5's Type 7 cascade pre-populate) with a Type 6 cascade detection step:

For each Type 6 amendment, if the registry section being amended is one that CLAUDE.md derives from (configurable list; default targets: §Occupied Resources Cargo workspace crate names + §Inherited Defaults Workspace crates), AND CLAUDE.md contains a pattern matching the registry's derived content (default regexes: `\b\d+ library crates\b`, `\b\d+ workspace members\b`, the bulleted Modules section enumeration), pre-populate the amendment marker's `expected_propagation` to include `CLAUDE.md` with anchor citation:

```
- CLAUDE.md: `<!-- GENERATED:setup:stack -->` + `<!-- GENERATED:setup:modules -->` sections — crate count + Modules enumeration cascade from registry change via /andromeda-setup-project --delta. Pre-populated per Proposal 12 (Type 6 → CLAUDE.md derived-content cascade).
```

`/andromeda-setup-project --delta` would then regenerate the relevant CLAUDE.md GENERATED sections per standard delta-scoped logic, eliminating the silent compounding.

**Design:**

1. `/andromeda-evolve` Phase 4 step 2 — for Type 6 amendments, after constructing the baseline marker but before user review, scan CLAUDE.md for derived-content patterns matching the registry being amended:
   - `<!-- GENERATED:setup:stack -->` section: grep for `\b\d+ (library crates|workspace members)\b` patterns
   - `<!-- GENERATED:setup:modules -->` section: grep for `(?m)^- \*\*\`crate-name\`\*\*` enumeration matching workspace crate names
   - `<!-- GENERATED:setup:key-directories -->` section: same patterns
2. If any match found AND registry section is one of the configured cascade-targets: append CLAUDE.md to marker's `expected_propagation` with the specific anchor citation.
3. CLAUDE.md template (`claude-md-template.md`) gains the relevant `<!-- GENERATED:setup:stack -->` / `<!-- GENERATED:setup:modules -->` etc. anchors if not already present (precondition for setup-project --delta to know which span to regenerate).
4. `/andromeda-setup-project` Phase 1 CLAUDE.md regeneration honors these anchor markers when --delta cascades through them; non-cascaded CLAUDE.md content stays byte-identical.

**Implementation cost estimate:**

| File | Change | Lines |
|---|---|---|
| `andromeda-evolve/SKILL.md` | Phase 4 step 2 — add Type 6 CLAUDE.md cascade detection logic (parallel to Type 7 cascade per Proposal 5 Phase 4 step 2g) | ~25 |
| `andromeda-evolve/references/output-templates.md` | Type 6 marker variant — add "expected_propagation pre-populate" subsection mirroring Type 7's Branch (a) / Branch (b) | ~20 |
| `andromeda-evolve/references/validation-checks.md` | Check 7 — add 7.7 subcheck for CLAUDE.md cascade pre-populate (analogous to Type 7's Check 8.5) | ~15 |
| `andromeda-setup-project/references/claude-md-template.md` | Add `<!-- GENERATED:setup:modules -->` / `<!-- GENERATED:setup:stack -->` anchors around relevant sections | ~10 |
| `andromeda-setup-project/references/delta-rerun-protocol.md` | Plan→file mapping table — clarify that Type 6 amendments may legitimately list CLAUDE.md in `expected_propagation` when registry section appears in derived sections | ~10 |

**Total:** ~80 lines across 5 files. Moderate-effort enhancement; high-value because it eliminates the silent staleness compounding pattern observed across 3+ Type 6 amendments (chunks #58, #60, #68).

**When to do:**

- Now-soon. Confidence the pattern recurs: HIGH (3+ direct dogfood observations; structural — every new lib crate triggers it; will recur as v0.2.0 evolves through more crate additions).
- Defer-until-Phase-4-of-v0.2.0 alternative — wait until chunk count approaches 80+ and Modules section has 16+ entries; manual fixup at that point would become tedious.
- Author preference — lean toward soon (next 1-2 sessions). Each unaddressed Type 6 amendment compounds the CLAUDE.md staleness; manual fixup gets harder as count drift grows. Pairs naturally with Proposal 7 Phase 2 implementation (arch.md narrative cascade); the two together would close both the CLAUDE.md AND arch.md narrative cascade gaps in one coordinated enhancement.

**Cross-references:**

- Triggering session: session 94 (this wrap; chunk #68 Type 6 amendment propagated cleanly via --delta but left CLAUDE.md Modules + Stack desynced). Prior sessions 70 / 74-equivalent observations for chunks #58 + #60 set the precedent; those manual fixups were apparently bundled into "evolve work" commits (`8189530` commit message pattern).
- Sibling proposal: **Proposal 5** — Type 7 evolve markers pre-populate CLAUDE.md cascade for chunk count (same structural pattern; different amendment type + different derived section).
- Sibling proposal: **Proposal 7** — Type 6 narrative-cascade visibility WITHIN arch.md structural sections (P7 covers arch.md narrative cascade; P12 covers CLAUDE.md derived-section cascade — orthogonal scopes).
- `spec-amendment-protocol.md` Part D Architecture.md exception → Type 6 permit path: documents that empty `expected_propagation` is permitted/typical; P12 proposes evolve auto-detect when empty is wrong AND pre-populate with CLAUDE.md cascade.
- `delta-rerun-protocol.md` Detection step 8 grep-expansion: defense-in-depth for value-replacement amendments; P12 covers the gap for purely-additive enumeration-list amendments where grep-expansion can't help.

---

## Status: PROPOSED — 2026-05-19 (session 97)

### Proposal 13 — First-class sub-phase state for two-phase chunks

**Problem:**

Some route chunks have explicit two-phase internal scoping where Phase A is a research spike (gitignored throwaway code; deliverable is a `.andromeda/decisions/pre-d#-{slug}.md` findings document) and Phase B is the production implementation, gated on the Phase A decision document returning PROCEED / REVISE / SPLIT (DEFER closes the chunk). Chunk #69 "Drain Rust implementation + template profiling diagnostics" is the first such chunk in pulse v0.2.0 to actually exercise this discipline at /implement time; future chunks #74 (LLM runtime + hardware profile, Pre-D1 spike) + others may also adopt the pattern.

The Andromeda framework currently has NO structured representation for "Phase A complete; Phase B pending" in `state.yaml`. The workflow forced this session (chunk #69 Phase A wrap-session 97) to:

1. Leave `state.yaml.last_completed_chunk` at the predecessor chunk (#68) — because chunk #69 is sub-completed only — even though chunk #69's Phase A actually landed substantive artifacts on main (the findings doc + .gitignore update).
2. Encode the partial state in `state.yaml.in_progress` as free-text: `{phase: 65, chunks: [69], status: "phase_a_complete; phase_b_pending", artifacts: [".andromeda/decisions/pre-d2-drain-spike.md"], started_at: <ISO>}`.
3. Use a `chore(wrap):` commit prefix to avoid D6 drift detection's `^chunk\(\d+\):` / `^feat\({module}\):` chunk-progression pattern match. A `feat(chunk-69):` subject would falsely fire D6 ("git log suggests chunk #69 completed but state.yaml stuck at #68").
4. Document the workaround in session-learnings.md so the next-session orchestrator (or human reviewer) understands why `last_completed_chunk` is "stuck" at 68 despite chunk #69 artifacts being on main.

This works but is convention-driven, not framework-enforced. Failure modes:
- A new session orchestrator who skips reading session-learnings.md might "fix" the stale `last_completed_chunk` by advancing to 69, then `/andromeda-phase` next session targets chunk #70 (which doesn't exist) — missing Phase B entirely.
- A user invoking `/andromeda-phase` (default, no --chunks override) would expect to plan the next-chunk increment; without explicit signal that Phase B is pending, the heuristic falls back to "all chunks done; tell user to run /andromeda-evolve."
- The `/andromeda-new-session` dashboard cannot tell the user "Phase B of chunk #69 is your next action" — it can only say "chunk #68 complete; chunk #69 is next" which is ambiguous (is #69 pending entirely OR partially complete?).

**Proposal:**

Extend `state.yaml` schema (`session-state-contract.md` Part B) with structured sub-phase fields:

```yaml
last_completed_chunk:
  route_index: 68
  # ... existing fields ...

in_progress:
  phase: 65
  chunks: [69]
  sub_phase: phase_a        # NEW: explicit "phase_a" / "phase_b" / null
  completion_status: complete  # NEW: "complete" / "blocked" / "in_flight"
  next_phase_blocked_on:    # NEW: list of files/decisions required before next phase
    - .andromeda/decisions/pre-d2-drain-spike.md
  artifacts:
    - .andromeda/decisions/pre-d2-drain-spike.md
  started_at: 2026-05-19T01:00:00Z
```

`/andromeda-phase` Setup step 4 (next-chunk identification) consults `in_progress.sub_phase` + `completion_status`: if `sub_phase="phase_a"` AND `completion_status="complete"` → next phase is Phase B of same chunk (re-plan against chunk's Phase B scope), NOT next chunk index. `/andromeda-new-session` dashboard renders the "chunk N Phase A complete; Phase B pending" state directly from the structured fields rather than parsing free-text status.

**Design:**

- `sub_phase` enum: `phase_a` / `phase_b` / `phase_c` / `null` (single-phase chunks). Lowercase snake_case per existing serde conventions.
- `completion_status` enum: `complete` / `blocked` / `in_flight` / `failed`. Distinguishes "Phase A done, Phase B can start" (`complete`) from "Phase A done but findings doc says DEFER, chunk closes" (`failed`) from "Phase A still being implemented" (`in_flight`) from "Phase A complete, Phase B blocked on external decision" (`blocked`).
- `next_phase_blocked_on` list: paths or short identifiers (e.g., decision tokens) that gate the next phase. `/andromeda-phase` Setup checks file existence; `/andromeda-new-session` renders.
- D6 drift detection updates: when `in_progress.sub_phase` is non-null AND `completion_status="complete"`, treat the chunk as "in flight" (the route chunk is not done) — do NOT fire D6 "git log suggests chunk #N completed but state.yaml stuck at #N-1" because the framework now understands the sub-phase state.
- Commit-subject heuristic: with the structured fields, the commit-subject pattern match becomes a soft signal rather than the load-bearing detection. `chore(wrap):` prefix discipline can stay as a recommendation but is no longer required to avoid false D6 fires.

**Implementation cost:**

| File | Change | LOC est |
|---|---|---|
| `~/.claude/skills/andromeda-wrap-session/references/session-state-contract.md` | Part B — add `sub_phase` / `completion_status` / `next_phase_blocked_on` to in_progress schema | ~15 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` | Phase 8 — detect sub_phase from commit message keywords (`Phase A` / `Phase B` text) + completion_status from artifacts (decision doc present → complete) | ~25 |
| `~/.claude/skills/andromeda-phase/SKILL.md` | Setup step 4 — branch on in_progress.sub_phase + completion_status to decide next-chunk-vs-next-phase planning | ~20 |
| `~/.claude/skills/andromeda-new-session/SKILL.md` + `references/visual-references.md` | Dashboard render for two-phase chunks (e.g., "chunk #N Phase A complete; Phase B pending: .andromeda/decisions/...") | ~15 |
| `~/.claude/skills/andromeda-wrap-session/references/integrity-protocol.md` Part C | D6 — augment drift detection to consult sub_phase + completion_status before firing | ~10 |
| One-time v2 → v2.2 migration step in wrap-session Phase 8 | Detect existing in_progress entries without sub_phase fields; default sub_phase=null + completion_status=in_flight | ~10 |

**Total:** ~95 lines across 5 files. Moderate-effort enhancement; high-value for projects that exercise two-phase chunks (pulse v0.2.0 has at least 2 known: chunk #69 Drain + chunk #74 LLM hardware-profile; future Pre-D# spike-gated chunks generalize).

**When to do:**

- Soon-ish. Confidence the pattern recurs: MEDIUM-HIGH. Pulse v0.2.0 has the second two-phase chunk (#74) coming in Phase 5+ work; absent P13, will face the same workaround pattern. Other Andromeda-using projects may also need it as they adopt multi-phase chunks.
- Defer-until-second-occurrence alternative — wait until chunk #74 implementation makes the same workaround explicit a second time; that's the natural "filed twice = implement" precedent the proposal log uses elsewhere (P5, P7, P12). At that point, the implementation is fairly mechanical.
- Author preference — lean toward filing-and-deferring. The workaround works for chunk #69 Phase A; the structured fix lands more naturally when chunk #74's Phase A is also done and there's a clear two-occurrence pattern justifying the schema change. No urgency until then.

**Cross-references:**

- Triggering session: session 97 (this wrap; chunk #69 Phase A Drain spike validation). The discipline applied verbatim as session-learnings.md "Two-phase chunk wrap-state pattern" entry. Convention-driven workaround documented; framework support deferred to P13.
- Sibling proposal: **Proposal 7** — Type 6 narrative-cascade visibility (different concern — arch.md text staleness vs state.yaml schema gap). Both surface in pulse v0.2.0 Phase 3+ work.
- `session-state-contract.md` Part B in_progress schema — current shape is `{phase, chunks, artifacts, started_at}`; P13 extends with 3 new optional fields.
- `integrity-protocol.md` Part C D6 — current heuristic relies on commit subject pattern + state.yaml.last_completed_chunk numeric comparison; P13 makes D6 sub-phase-aware so wrap commits for partial-chunk work don't trip the drift gate.

---

## Status: PROPOSED — 2026-05-19 (session 98)

### Proposal 14 — `/andromeda-implement` Phase 2b smoke-check protocol should support "integration-test runtime smoke" for non-UI multi-session chunks

**Problem:**

Multi-session chunks like #69 Phase B (estimated 4-6 sessions per route §Risk notes) trigger `/andromeda-implement` Phase 2b boot-smoke gate on every session that touches files in the gate's mechanical trigger list (`pulse-app/src/main.rs`, `crates/ui-bridge/src/`, `pulse-app/src-tauri/tauri.conf.json`, `pulse-app/capabilities/*.json`). The protocol mandates `npx @tauri-apps/cli dev` with 60s timeout. In practice for backend-only multi-session chunks:

- Each session may touch one of the trigger paths (e.g., chunk #69 Session 1 touched `crates/ui-bridge/src/contract.rs` for the BufferError::Drain From-impl arm; Session 2 touched `pulse-app/src/main.rs` to thread the new `run_consumer` arg)
- The change is mechanically in-scope but not functionally boot-affecting (a From-impl arm or new `None` param to run_consumer doesn't introduce boot panics)
- Cold-rebuild of dev profile takes 5+ min on this codebase (verified at chunk #69 Phase B Session 1 post-`cargo clean`)
- Tauri dev launch as background bash leaves an orphan pulse-app.exe process when bash times out at 10 min (per verification-harness.md session addition 2026-05-19), which blocks subsequent cargo nextest runs
- Meanwhile the workspace nextest run includes `pulse-app::e2e_p1_otlp_grpc_to_traces_query` + `pulse-app::perf_slo_10k_spans` — both exercise full boot+ingest+query+broadcast paths via real tonic + axum + DuckDB + Arrow stack in <15s combined

The current Phase 2b protocol treats Tauri-dev-launch as load-bearing for runtime verification, but for non-UI sessions the integration-test path provides equivalent runtime coverage at a fraction of the budget cost and without the orphan-process side effect.

**Proposal:**

Extend `/andromeda-implement` Phase 2b smoke-check protocol with an "integration-test runtime smoke" alternative path:

- Phase 2b §Step 2 (smoke command detection) gains a pre-check: scan plan.md / `## Codebase touchpoints / New files` + `Files to modify` for any UI-touching paths (`pulse-app/ui/src/**`, `pulse-app/ui/dist/**`, `pulse-app/tauri.conf.json` build config keys affecting webview behavior). If zero UI-touching paths AND the workspace nextest just passed AND nextest includes ≥1 integration test exercising boot path (heuristic: tests in `pulse-app/tests/e2e_*.rs` OR `perf_slo_*.rs` — both boot pulse-app's full stack), classify smoke as "passed via integration-test path" without invoking Tauri dev.
- For UI-touching sessions OR sessions where integration tests don't exercise the boot path, fall back to current Tauri dev protocol.
- Phase 2b §Step 3 (run smoke check) gains an explicit "lightweight build verification": `cargo build -p pulse-app` (no Tauri runtime spawn) as a complementary smoke that verifies the dev-profile binary builds cleanly. Faster than tauri dev cold rebuild, deterministic, no orphan process risk.

**Design:**

- Phase 2b §Step 2 "Detect smoke-check classification":
  1. Read current chunk's plan.md `## Codebase touchpoints` section
  2. Glob "New files" + "Files to modify" entries; classify each path as `ui-touching` (matches `pulse-app/ui/src/**` or webview config keys) OR `backend-only`
  3. If ALL touched paths are `backend-only` AND workspace nextest just passed AND nextest test count includes ≥1 `e2e_*` or `perf_slo_*` integration test: smoke classification = `integration-test-path`; log result + proceed to Phase 3 with smoke status `✓ smoke passed via integration-test path ({N} integration tests boot pulse-app stack)`
  4. Otherwise: smoke classification = `tauri-dev-required`; proceed with current Tauri dev protocol
- Phase 2b §Step 3 "Run lightweight build verification" (new optional sub-step before full Tauri dev):
  - When tauri-dev-required: run `cargo build -p pulse-app` first; if it fails, surface immediately (build-error classification per Phase 2 §Bounded retry caps); if it succeeds, proceed with Tauri dev launch
  - When integration-test-path: skip the cargo build (workspace nextest already built it via test profile; dev profile would require separate rebuild but isn't load-bearing for the smoke classification)
- Phase 2b §Step 6 summary line gains new variants:
  - `Phase 2b passed via integration-test path ({N}/{M} integration tests boot pulse-app)`
  - `Phase 2b passed via lightweight build ({duration}; full Tauri dev deferred to UI-touching session)`

**Implementation cost:**

| File | Change | LOC est |
|---|---|---|
| `~/.claude/skills/andromeda-implement/SKILL.md` Phase 2b §Step 2 | Plan.md path scan + UI-touching classifier | ~30 |
| `~/.claude/skills/andromeda-implement/SKILL.md` Phase 2b §Step 3 | Branch on classification: integration-test-path vs tauri-dev-required | ~25 |
| `~/.claude/skills/andromeda-implement/SKILL.md` Phase 2b §Step 6 | Summary variants for new classification outcomes | ~10 |
| `~/.claude/skills/andromeda-implement/references/visual-references.md` | Phase 2b banner template variants | ~15 |
| `~/.claude/skills/andromeda-implement/SKILL.md` Phase 3 report | "via integration-test path" / "via lightweight build" lines | ~10 |

**Total:** ~90 lines across 2 files. Moderate-effort skill mechanic improvement. High value for projects with multi-session chunks where every session triggers the mechanical smoke gate.

**When to do:**

- Defer-until-second-occurrence per the proposal-log convention (P5/P7/P12/P13 precedent). The first occurrence was this session (chunk #69 Phase B Sessions 1+2 both hit the Phase 2b cold-rebuild friction). Second occurrence will likely be chunk #69 Phase B Session 3+ (still upcoming) OR chunk #74 LLM-runtime Phase B work (also multi-session per route plan).
- The current Tauri dev approach is not BROKEN — it works, just expensive. Workaround (manual classification + `cargo build -p pulse-app` as substitute) is documented in session-learnings.md 2026-05-19 + verification-harness.md Session Addition 2026-05-19 as a convention; framework support via P14 lands when the pattern is well-validated across multiple chunks.
- Author preference — lean toward filing-and-deferring. The workaround works; the structured fix lands when there's clear two-chunk-occurrence evidence justifying the skill change.

**Cross-references:**

- Triggering session: session 98 (this wrap; chunk #69 Phase B Sessions 1+2 backend-only work hit the Phase 2b mandatory-Tauri-dev gate on each session for trivial mechanical touches to boot-path files).
- Sibling proposal: **Proposal 13** — first-class sub-phase state for two-phase chunks (filed session 97). P14 is the runtime-smoke complement to P13's state-tracking enhancement; both address the multi-session chunk workflow friction. P13 + P14 together close most of the friction observed during chunk #69 Phase B implementation.
- `references/visual-references.md` Phase 2b — current variants are `✓ smoke passed ({boot-time}s)` / `– smoke skipped: no-cli` / `– smoke skipped: headless`. P14 adds two more: integration-test-path + lightweight-build.
- `references/runtime-failure-patterns.md` Phase 2b table — no new patterns needed; existing patterns (capability-drift, Tauri capability JSON, schema mismatch) still classify out-of-scope failures correctly. P14 changes WHEN tauri dev launches, not how surfaces classify failures.
- Cross-reference to **verification-harness.md Session Addition 2026-05-19** (filed this same wrap) which documents the Windows-specific orphan-process risk of background tauri dev — P14 mitigates by avoiding the launch entirely for non-UI sessions.

---

## Status: IMPLEMENTED — 2026-05-22 (session 116, commit pending) — wrap-session Phase 2 step 5 dead-test scan + Cargo.toml [lib]/[[bin]] test=false target detection + [package.metadata.andromeda] allow-dead-source-tests opt-out + Phase 11 report subsection + visual-references.md template landed via chunk #76 batch

### Proposal 15 — `/andromeda-wrap-session` or `/andromeda-implement` should detect dead `#[cfg(test)] mod tests` blocks in crates с `[lib] test = false`

**Problem:**

When a workspace member (e.g., `pulse-app`) declares `[lib] test = false` in `Cargo.toml` as а workaround for runtime build-time issues (e.g., Windows WebView2 DLL load failure per session-learning 2026-05-13), any source-level `#[cfg(test)] mod tests { … }` blocks in that crate compile but NEVER run as nextest binaries. Authors writing those tests assume they run (the `mod tests` pattern is universal Rust idiom); wrap-session test reports show "N/N pass" without noting source-level tests are silently absent from the discovered set; the gap persists across multiple chunks without surfacing.

This actually happened on andromeda-pulse: chunks #69 / #70 / #71 each added substantial `#[cfg(test)] mod tests` blocks to `pulse-app/src/{drain,lifecycle,storm,baseline}_persistence.rs` (47 tests total across 4 files). All 47 were dead code — never executed, never caught regressions, never validated chunk acceptance criteria. The gap surfaced only at chunk #72 wrap session 107 when the new tests added в that chunk's scope were observed missing from nextest output, and audit revealed the prior tests were equally absent. Bug-finding cost was nontrivial: chunk #72 wrap discovered a real production OOM-on-corrupt-input bug in `baseline_persistence::migrate_legacy_inner` that the dead `migrate_legacy_failed_on_corrupt_bytes_preserves_legacy_file` test WOULD have caught at chunk #70 if it had actually run.

**Proposal:**

Add a "dead-test detection" pass к `/andromeda-wrap-session` Phase 2 (test run) OR `/andromeda-implement` Phase 2 (fix-loop entry). The pass scans each workspace member's `Cargo.toml` for `[lib] test = false` (or `[[bin]] test = false`) declarations; for each such crate, grep src/ for `#[cfg(test)]\s*mod\s+tests` occurrences; if any found, emit warning:

```
⚠ Dead source-level tests detected:
  - pulse-app/src/drain_persistence.rs:105 — `mod tests` in crate с [lib] test = false
    (will compile but NEVER run as nextest binary; move к pulse-app/tests/<file>.rs)
```

Posture: warning-not-fatal. User decides whether to migrate (recommended) OR accept the dead code (rare; might be intentional documentation-only).

**Design:**

- `/andromeda-wrap-session` Phase 2 §Step 4 (new) "Dead-test scan":
  1. Parse all workspace `Cargo.toml` files; collect crates с `[lib] test = false` OR `[[bin] test = false` declarations into a target list
  2. For each target crate, glob `src/**/*.rs`; grep each file for `^#\[cfg\(test\)\]\s*\n?\s*mod\s+tests`
  3. For each match, emit warning line into Phase 11 report under new "Dead-test warnings" subsection
  4. Surface count in commit message body Phase 10 under "Dead tests: {N} blocks in {M} files ({K} crates)"
  5. Continue к Phase 3 regardless. Dead-test hits do NOT block commit.
- Configuration knob: per-crate opt-out via new `Cargo.toml [package.metadata.andromeda]` block:
  ```toml
  [package.metadata.andromeda]
  allow-dead-source-tests = true  # disables wrap-session dead-test warnings for this crate
  ```
- Handles the rare "intentional documentation-only" case без forcing the user к delete legitimate-but-unrunnable code.

**Implementation cost:**

| File | Change | LOC est |
|---|---|---|
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 2 | New §Step 4 dead-test scan with Cargo.toml parsing + grep loop | ~50 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 10 | Commit message body line for dead-test count | ~5 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 11 | Report subsection for dead-test warnings | ~15 |
| `~/.claude/skills/andromeda-wrap-session/references/visual-references.md` | Dead-test warning rendering template | ~10 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 2 | Opt-out parsing for `[package.metadata.andromeda] allow-dead-source-tests` | ~15 |

**Total:** ~95 lines across 2 files. Low-medium-effort pipeline detection; high value for any project с workspace members carrying `[lib] test = false` workarounds (rare pattern but happens in Tauri / GUI runtime constraint situations).

**When to do:**

- File-and-defer per the P5 / P7 / P12 / P13 / P14 precedent. The first occurrence (this session 107) is a costly bug-finding event — the dead-test gap hid a real OOM vulnerability for 3 chunks. The fix would prevent the same gap from recurring on any future project adopting the same Tauri workaround pattern (or any analogous `[[bin]] test = false` situation).
- Defer-until-second-occurrence may not apply here — the cost of the FIRST miss already validated the proposal. But Andromeda improvement convention is "wait for two evidences before implementing"; sticking к the convention means waiting until another project hits the same shape (or until pulse-v0.2.0 next phase introduces another `[lib] test = false` situation).
- Author preference — lean toward filing-and-deferring. Workaround (manual `cargo nextest list -p {crate} | grep <test-name>` audit pre-commit; documented в `.claude/rules/testing.md` Session Addition 2026-05-20 session 107) works as a disciplined-author check; pipeline support via P15 lands когда there's clear two-project-occurrence evidence.

**Cross-references:**

- Triggering session: session 107 (this wrap; chunk #72 PII scrubber coverage extension wrap + dead-test cleanup discovered 47 dead tests across chunks #69/#70/#71 source-level `mod tests` blocks).
- Sibling proposal: **Proposal 13** — first-class sub-phase state for two-phase chunks (filed session 97). Both P13 + P15 are wrap-session enhancements closing detection gaps; P13 detects sub-phase progress, P15 detects dead test code. Different domains but same wrap-session-as-detection-gate framing.
- `.claude/rules/testing.md` Session Addition 2026-05-13 — documents the `[lib] test = false` Windows WebView2 workaround.
- `.claude/rules/testing.md` Session Addition 2026-05-20 session 107 (filed this same wrap) — documents migration discipline for moving dead source-level tests к integration tests + the visibility-bump pattern (private fns → pub с `#[doc(hidden)]` for integration access).

---

## Status: IMPLEMENTED — 2026-05-22 (session 116, commit pending) — Option (b) applied: Phase 10 step 4 SHA-fixup amend REMOVED + Phase 8 step 7 State H housekeeping codified (auto-heal pending OR orphan commit_sha on next wrap) + new-session visual-references.md Phase 6 State H severity = info for pending / warning for unreachable orphan landed via chunk #76 batch

### Proposal 16 — wrap-session Phase 10 step 4 SHA-fixup amend captures pre-amend SHA (chronic single-wrap-lag drift)

**Problem:**

wrap-session Phase 10 step 4 ("post-commit state.yaml SHA-fixup amend") attempts к update `state.yaml.last_completed_chunk.commit_sha` from the `"pending"` placeholder к the real short SHA of the wrap commit. The sequence:

1. Pre-wrap: state.yaml.commit_sha = `"pending"` (placeholder)
2. Make wrap commit → produces SHA `X`
3. Capture `X` via `git rev-parse --short HEAD`
4. Update state.yaml.commit_sha = `X`
5. `git commit --amend --no-edit` к fold state.yaml into the same commit → produces NEW SHA `Y` (different from `X` because amend changes content + SHA)

Result: state.yaml inside the final commit `Y` contains `commit_sha = X` (the pre-amend SHA), but `X` is now a DANGLING orphan commit (not reachable from HEAD = `Y`). The next /new-session detects State H drift; the next /wrap-session's "State H housekeeping" fixes it by setting commit_sha = `Y` (the now-reachable HEAD).

Evidence of recurrence:
- Session 105 wrap created orphan SHA `9459d14`; session 106 wrap "State H housekeeping (chunk #71 95a9619→2537e44)" cleaned it up
- Session 107 wrap created orphan SHA `ae62162`; session 108 wrap (this file's wrap) "State H housekeeping (chunk #72 ae62162→e08693e)" cleaned it up
- Verified: `git merge-base --is-ancestor ae62162 HEAD` returns non-zero (orphan); `git log -1 ae62162` shows the commit exists but unreachable from HEAD

This is a fundamental git constraint: a commit's SHA cannot be known until the commit exists, and amending changes the SHA. There's no way to capture the post-amend SHA without yet another amend (which would loop forever).

**Proposal:**

Three viable mitigation options; recommend Option (b) per minimal-surgery preference:

**Option (a) — Detect + label as known-stale:**
After Phase 10 step 4 amend, re-read state.yaml + add a comment block above commit_sha citing "SHA is pre-amend; actual post-amend SHA in git is unknown (chicken-and-egg). Next wrap auto-heals via State H housekeeping." Surfaces honest intent в audit trail. Cost: every commit_sha field carries an explanatory comment forever.

**Option (b) — Remove Phase 10 step 4 entirely; accept the lag:**
Skip the amend attempt; leave commit_sha = `"pending"` placeholder. New-session detects State H with severity info (downgraded from warning because the lag is now part of the documented pipeline). Next wrap-session's State H housekeeping path (which already exists per session 106 + 108 evidence) sets commit_sha = real HEAD SHA в that wrap's atomic state.yaml write. Cost: one-wrap-cycle lag в state.yaml accuracy (cosmetic; doesn't affect correctness because audit trail in marker files + amendment IDs remain unambiguous).

**Option (c) — Two-commit pattern (wrap commit then SHA-fixup commit):**
Phase 10 makes the wrap commit normally; Phase 10 step 4 creates a SEPARATE follow-up commit `chore(state-cursor): record chunk #N commit_sha {real-sha}`. Audit trail has two commits per wrap instead of one; commit_sha records the prior wrap commit's actual SHA. Cost: doubles wrap commit count (98 wraps → 196 commits if applied retroactively); violates the "one-commit-per-wrap" invariant documented в session-state-contract.md.

Recommended: **Option (b)**. Simplest; respects the git constraint; acknowledges the chronic lag as part of the documented pipeline rather than fighting it with imperfect mitigations. State H detection at /new-session continues к function (it already handles the existing pattern); the dashboard's existing remediation hint ("next wrap-session Phase 8 should auto-fix") becomes load-bearing rather than aspirational.

**Design:**

For Option (b):

1. Edit `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 10:
   - Remove step 4 ("Post-commit state.yaml SHA-fixup amend") entirely
   - Update step 3 commit composition: leave `commit_sha = "pending"` in state.yaml inside the wrap commit; document this как deliberate (audit trail preserved by amendment markers + commit subject lines)
   - Add Phase 8 step 6 (NEW): "State H housekeeping — if state.yaml.last_completed_chunk.commit_sha is `'pending'` OR points к а SHA not reachable from HEAD, update it к the most recent commit matching chunk progression pattern (`^chunk\\(\\d+\\):` OR `^feat\\(\\{module\\}\\):` against the last_completed_chunk's title)". This becomes the auto-heal path every wrap (currently only documented as "next wrap-session Phase 8 should auto-fix" but not explicitly wired — codify it).

2. Edit `~/.claude/skills/andromeda-new-session/references/visual-references.md` Phase 9:
   - Downgrade State H from warning severity к info severity when state.yaml.commit_sha = `"pending"` (this is the EXPECTED post-wrap state under Option b — not an anomaly)
   - Keep warning severity when state.yaml.commit_sha points к an unreachable non-pending SHA (this would be an unhealed previous-wrap leftover)

3. No spec-amendment-protocol.md changes needed (state.yaml schema unchanged; Phase 10 step 4 was procedural, not schema-defining).

**Implementation cost:**

~30 LOC across 2 skill files. Single Andromeda toolkit user-level edit (no project-side changes). Should land as part of chunk #76 "Andromeda pipeline meta-improvements" alongside P7 + P12 + P15 + P17-P18.

**When to do:**

After chunk #76 starts (Phase 6 v3 plan sequence). Could land standalone if user wants to clear the chronic State H housekeeping noise from session-after-session wrap commits earlier; the 1-wrap-lag is cosmetic so not urgent.

**Cross-references:**

- Triggering observation: session 108 /new-session detected State H carry-over from session 107 wrap (ae62162 orphan); resolved via State H housekeeping in this wrap (ae62162 → e08693e).
- Historical precedent: session 105/106 pair (orphan 95a9619 → 2537e44 fix); session 107 carried it forward; pattern visible в commit log по subject lines.
- Sibling proposal: **Proposal 15** — dead-test detection gate for wrap-session/implement (filed session 107). Both P15 + P16 are wrap-session meta-improvements; P15 fixes detection gap, P16 fixes mechanism flaw. Both target chunk #76 batch.
- `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 10 step 4 — the load-bearing mechanism being deprecated.
- `~/.claude/skills/andromeda-new-session/references/visual-references.md` Phase 9 State H rendering — needs severity downgrade for `"pending"` case.
- `state.yaml.last_completed_chunk.commit_sha` field — semantics shift from "real post-amend SHA" к "real HEAD-reachable SHA OR `'pending'` placeholder until next wrap heals".
- `~/.claude/skills/andromeda-wrap-session/references/visual-references.md` Phase 11 — current report sections; P15 adds new "Dead-test warnings" subsection.

---

## Status: IMPLEMENTED — 2026-05-22 (session 116, commit pending) — implement SKILL.md Phase 1 step 0 META detection (≥80% signature) + NEW Phase 1b sibling-skill orchestration с availability check / branch path + MUST NOT arch.md clause clarified with META EXCEPTION + Phase 3 META-chunk-orchestrated success variant + visual-references.md banners (Phase 1 META detection / Phase 1b orchestration / Phase 3 META success) + fix-loop-protocol.md META bypass note landed via chunk #76 batch

### Proposal 17 — `/andromeda-implement` META-chunk recognition + inline sibling-skill orchestration

**Problem:**

When `/andromeda-implement` encounters a META chunk (a chunk whose `plan.md` Implementation Steps consist entirely of `/andromeda-evolve --allow-arch-registry` or similar sibling-skill invocations rather than code edits), the current behavior is inconsistent с user expectations:

1. **First-attempt observation (this session):** /andromeda-implement Phase 1 read plan.md, recognized that each Implementation Step said "Invoke /andromeda-evolve --allow-arch-registry с descriptor X", classified the chunk как out-of-scope (per MUST NOT clause "Do not modify .andromeda/architecture.md") and surfaced а META-handoff variant requesting user to manually invoke /andromeda-evolve × N. **Correct per skill constraints** but suboptimal UX: user already approved the plan at /andromeda-phase Phase 6; surfacing only к invoke another skill manually feels like artificial friction.

2. **Second-attempt observation:** user explicitly removed `disable-model-invocation: true` from andromeda-evolve + andromeda-setup-project skill definitions, then re-invoked /andromeda-implement. This time, the orchestrator recognized that the relevant sibling skills were Skill-tool-invocable, executed inline orchestration (3 amendments + 3 propagations + standard gates) successfully. **Same chunk; different UX path; outcome identical.**

The skill currently doesn't have explicit guidance for META chunks. The constraint MUST NOT modify .andromeda/architecture.md is correctly enforced, but the implicit assumption "META chunks are out-of-scope" is wrong: META chunks ARE in-scope if the sibling skills are available + permitted к invoke.

**Proposal:**

Extend /andromeda-implement Phase 1 с а META-chunk detection step + orchestration policy:

1. **Phase 1 step 0 (NEW) — META chunk detection.** After reading plan.md but before Phase 2 (fix loop), scan `## Implementation Steps` for the signature pattern: each step's primary verb is "Invoke /andromeda-{skill} с args ..." (no Read / Edit / Write file references in the step body). If ≥80% of Implementation Steps match this pattern → classify chunk as META.

2. **META-chunk path:** when classified as META, route к а new Phase 1b "Sibling-skill orchestration":
   - Enumerate the sibling skills referenced in plan.md Implementation Steps (e.g., `/andromeda-evolve --allow-arch-registry`, `/andromeda-setup-project --delta`).
   - Check Skill-tool availability for each referenced skill (introspect the system-injected skill list).
   - **If ALL referenced skills are Skill-tool-invocable:** proceed with inline orchestration (invoke each per plan.md step sequence; capture artifacts; record outcomes). Skip the existing MUST NOT для architecture.md (sibling-skill writes к arch.md are authorized by their own discipline, not /implement's).
   - **If ANY referenced skill is NOT Skill-tool-invocable:** surface the current META-handoff variant ("user runs the following N skills manually...") — preserves existing behavior for skills not yet enabled.

3. **Update MUST NOT clause** about arch.md: clarify that the prohibition is on /implement DIRECTLY editing arch.md as а fix-loop delta; sibling-skill orchestration (where the sibling skill — e.g., /andromeda-evolve --allow-arch-registry — has its own arch-write discipline + Type 6 authorization) is permitted under the META-chunk path.

4. **Phase 2 fix-loop adaptation:** for META chunks, "tests" = standard chunk-gate baseline run AFTER all sibling-skill orchestrations complete (zero `.rs` changes mean tests preserve baseline trivially; the gate confirms no regressions induced by the orchestration). bindings.ts regen discipline applies if any default-features nextest fires during the gate run (per testing.md 2026-05-13 + 2026-05-17 entries).

5. **Phase 3 report variants:** add а "META-chunk-orchestrated" success variant alongside the existing default-success / Path A / Path A' / Path B / Stuck / Smoke-surfaced variants. Reports each sibling-skill invocation's outcome + final standard-gate status + amendment marker paths for wrap-session pickup.

**Design:**

Concrete edits:

1. Edit `~/.claude/skills/andromeda-implement/SKILL.md`:
   - Phase 1 — insert "step 0 META-chunk detection" before existing step 1
   - New Phase 1b "Sibling-skill orchestration" between Phase 1 + Phase 2 (conditional на META detection)
   - MUST NOT clause "Modify .andromeda/architecture.md as а delta amendment" — clarify scope: "directly via Edit/Write tool during fix loop OR Trigger 4 Path A. EXCEPTION: when chunk is classified as META AND plan.md Implementation Steps explicitly delegate к sibling-skill invocations carrying their own arch-write discipline (e.g., /andromeda-evolve --allow-arch-registry), inline orchestration via Skill tool is permitted; the sibling skill's own constraints + audit trail (amendment marker + state.yaml lifecycle) apply."
   - Phase 3 — add "META-chunk-orchestrated" success variant.

2. Edit `~/.claude/skills/andromeda-implement/references/visual-references.md`:
   - New banner template для Phase 1 META detection ("⊙ META chunk detected: orchestrating sibling skills inline").
   - New banner template для Phase 3 META success variant (lists each sibling-skill invocation outcome + amendment marker paths + standard-gate status).

3. Edit `~/.claude/skills/andromeda-implement/references/fix-loop-protocol.md`:
   - Add brief note that META chunks bypass the standard fix loop (no code edits; "fix" is verifying gates stay green after orchestration).

**Implementation cost:**

- ~80-120 LOC across 3 files in `~/.claude/skills/andromeda-implement/` (SKILL.md Phase 1 + Phase 1b + Phase 3 + visual-references.md banners + fix-loop-protocol.md note).
- Verification: re-run chunk #74 plan mentally against the new logic (should classify as META; orchestrate 3 evolve invocations + 3 setup-project --delta invocations + final gate; produce the same outcome this session achieved).
- Test scenario (future chunk #75/#76/#77 are also META; canonical test cases). Run /andromeda-implement against chunk #75 (Documentation consolidation — manual edits + setup-project --delta cascade) to verify the new path handles non-evolve META chunks too.

**When to do:**

Chunk #76 (Andromeda pipeline meta-improvements P7 + P12 + P15-P18) is the natural batch. P17 joins P15/P16/P18 (already filed) as session-meta-improvements landing together. Pre-condition: chunks #75 + earlier consolidation chunks must complete first к stabilize the consolidation Phase 6 baseline.

**Cross-references:**

- Triggering observation: this session 111 /andromeda-implement first-attempt (META-handoff surface) → user enabled skill invocation → second-attempt (inline orchestration). Both attempts in the same conversation; comparison evidence of the UX gap.
- Sibling proposals: P14 (Phase 2b smoke check protocol extension) — both modify /andromeda-implement Phase 2/2b behavior; P17 modifies Phase 1 + Phase 3. P15 (dead-test detection) — sibling wrap-session improvement, joins P17 в chunk #76 batch.
- `~/.claude/skills/andromeda-implement/SKILL.md` Phase 1 + Phase 3 — the load-bearing skill body sections to extend.
- `~/.claude/skills/andromeda-implement/references/visual-references.md` Phase 1 + Phase 3 — banner templates to add.
- Chunk #74 marker files (`.andromeda/runs/2026-05-21T12-08-11-spec-amendment-acknowledge-log-templates-and-corpus-schema/amendment.md` + 2 siblings) — proof of concept for the META-orchestration path; preserved as audit trail.

---

## Status: IMPLEMENTED — 2026-05-22 (session 116, commit pending) — integrity-protocol.md Part C D5 section-aware classification (substantive vs cosmetic per arch §Design Philosophy ↔ CLAUDE.md §Architecture comparison) propagated byte-identical across 3 skills (new-session canonical + wrap-session + setup-project copies verified via diff -q) + new-session visual-references.md Phase 7 D5 substantive render variant landed via chunk #76 batch

### Proposal 18 — `/andromeda-new-session` D5 severity classification should distinguish arch §Design Philosophy mtime drift (substantive) from other arch narrative mtime drift (cosmetic)

**Problem:**

`/andromeda-new-session` Phase 7 currently flags D5 drift (arch.md mtime > CLAUDE.md mtime) with a single severity=warning + uniform remediation_hint "Optional — chunk #75 narrative-cascade content does not flow into CLAUDE.md derived sections... Cosmetic mtime drift only." This framing is INCORRECT when the arch.md change touched §Design Philosophy (because CLAUDE.md `<!-- GENERATED:setup:architecture -->` derives directly from arch §Design Philosophy line 3 narrative — verified at session 114). When the change touched §Established Decisions / §Conventions / §Standard Contracts / etc. instead, the "cosmetic" framing is correct.

Result observed at session 114: dashboard recommended D5 as "optional" cosmetic remediation. User invoked `/andromeda-setup-project` full re-derive anyway, and Phase 0 surfaced substantive stale text at CLAUDE.md line 96 ("eight library crates" — should be "twelve" per arch §Design Philosophy line 3 post chunks #58/#60/#68 cascade). The dashboard misclassified the severity. The wrap-session preceding (session 113) had stated this drift was cosmetic and "will defer until chunk #76 P7+P12 systematizes Type 6 narrative-cascade detection" — but P7+P12 cover different cascade paths (arch narrative → all derived sections; Type 6 evolve marker pre-population). Neither directly addresses the §Design Philosophy → §Architecture derivation gap.

**Proposal:**

Extend `/andromeda-new-session` Phase 7 (`references/integrity-protocol.md` Part C D5 detection) with section-aware severity classification:

1. After detecting D5 (arch.md mtime > CLAUDE.md mtime), read arch.md §Design Philosophy first paragraph (default first 5 lines after the `## Design Philosophy` heading; or up to the next `## ` heading — whichever first).
2. Read CLAUDE.md `<!-- GENERATED:setup:architecture -->` block content (parse via section-markers regex).
3. **If §Design Philosophy paragraph is NOT a substring/derivation of the CLAUDE.md §Architecture block** (substantive divergence detected): downgrade D5 to severity=**substantive** with remediation_hint "**Substantive** — run `/andromeda-setup-project` (full re-derive) to materialize the updated §Design Philosophy paragraph into CLAUDE.md §Architecture. The `--delta` variant will NOT fix this because narrative is not amendment-tracked."
4. **Otherwise** (CLAUDE.md §Architecture already reflects current §Design Philosophy): keep severity=warning + current "cosmetic" remediation_hint.

Same logic applies to route.md mtime drift, but route does NOT derive into CLAUDE.md narrative — only the "Roadmap (N epochs / M chunks)" pointer-table row count cascades. Type 7 route-append amendments handle that via `setup-project --delta`. So route.md mtime drift D5 stays severity=warning + cosmetic by default; substantive escalation N/A unless future scope adds narrative derivation from route.md.

**Design:**

Concrete edits to `~/.claude/skills/andromeda-new-session/references/integrity-protocol.md` Part C D5 section:

```diff
 D5 — Plan-to-CLAUDE.md drift (mtime-based):
 - For each upstream (input + arch + 6 specialist plans + route): if mtime(upstream) > mtime(CLAUDE.md) → flag D5 "{upstream} regenerated since last setup-project"
 - **Amendment-aware classification (NEW v2)**: check state.yaml.spec_amendments.active...
+- **Section-aware classification (NEW P18)**: for arch.md D5 entries with NO amendment match:
+  - Read arch.md §Design Philosophy first paragraph (lines between `## Design Philosophy` heading and next `## ` heading)
+  - Read CLAUDE.md `<!-- GENERATED:setup:architecture -->` block content (parse via section-markers regex)
+  - If §Design Philosophy content is NOT a substring/derivation of the CLAUDE.md §Architecture block: severity=substantive (not warning); remediation_hint "**Substantive** — run /andromeda-setup-project (full re-derive) to materialize the updated §Design Philosophy paragraph into CLAUDE.md §Architecture. The --delta variant will NOT fix this because narrative is not amendment-tracked."
+  - Otherwise: severity=warning; remediation_hint "Optional — cosmetic mtime drift only. CLAUDE.md derived sections (Modules / Stack / pointer-table / §Architecture) are current."
```

Same conceptual edit to:
- `~/.claude/skills/andromeda-wrap-session/references/integrity-protocol.md` Part C D5 (part of the 6-contract cross-skill diff; must update in lock-step)
- `~/.claude/skills/andromeda-setup-project/references/integrity-protocol.md` Part C D5 (same)

Visual rendering at `~/.claude/skills/andromeda-new-session/references/visual-references.md` Phase 7:
- Add new render variant for severity=substantive: `⚠️ D5 (substantive — arch §Design Philosophy not reflected in CLAUDE.md §Architecture) — {description}. Strongly recommend: /andromeda-setup-project (full re-derive) this session.`

**Implementation cost:**

- ~40-60 LOC across the 3 integrity-protocol.md copies (must stay byte-identical per cross-skill diff Check 8) + visual-references.md update.
- Verification: re-run new-session against pre-setup-project state at session 114 (arch.md mtime > CLAUDE.md mtime with stale "eight library crates" in CLAUDE.md §Architecture) — confirm new logic correctly flags as substantive.

**When to do:**

Chunk #76 (Andromeda pipeline meta-improvements P7 + P12 + P15-P18) is the natural batch. P18 joins P7 (arch narrative cascade — currently scoped to all specialist sections; P18 narrows to the §Design Philosophy → §Architecture path specifically) + P12 (Type 6 evolve markers should pre-populate `expected_propagation: [CLAUDE.md]` when registry section appears in CLAUDE.md derived sections — orthogonal to P18 since P18 covers narrative not registry) — all three address narrative-cascade scope detection gaps that the current "cosmetic only" framing masks.

**Cross-references:**

- Triggering observation: this session 114 `/andromeda-new-session` dashboard labeled D5 as "Optional — cosmetic mtime drift only" while CLAUDE.md §Architecture line 96 contained substantive stale "eight library crates" text from chunks #58/#60/#68 cascade.
- Sibling proposals: P7 (arch narrative cascade — narrative changes to specialist sections should trigger cascade); P12 (Type 6 evolve markers should pre-populate expected_propagation for CLAUDE.md derived sections); both adjacent to P18's scope but cover different cascade paths.
- Session 114 Tier 3 session-learning at `.claude/docs/session-learnings.md` (2026-05-21 entry "CLAUDE.md §Architecture section DOES propagate arch.md §Design Philosophy narrative cascade") — empirical anchor.
- `~/.claude/skills/andromeda-new-session/references/integrity-protocol.md` Part C D5 — load-bearing reference section to extend.
- `~/.claude/skills/andromeda-wrap-session/references/integrity-protocol.md` + `~/.claude/skills/andromeda-setup-project/references/integrity-protocol.md` — must stay byte-identical per 6-contract cross-skill diff.
- CLAUDE.md §Architecture block at line 96 (current; verifies the cascade path).
- arch.md §Design Philosophy line 3 (the source of truth narrative line for the derivation).
- Skill tool availability semantics — depends on Claude Code harness allowing model-invocation per-skill (via removing `disable-model-invocation: true`); P17 assumes this is the project's preferred posture для Andromeda skill set.
- chunks #75 + #77 — future META chunks that will benefit from P17 implementation; canonical test cases post-implementation.

---

## Status: PROPOSED — 2026-05-22 (session 116)

### Proposal 19 — P16 Phase 8 step 7 State H housekeeping timing discriminator (heal previous-wrap leftover, NOT this-wrap pending)

**Problem:**

P16 Option (b) landed at chunk #76 session 116. Phase 8 step 3 advances `state.yaml.last_completed_chunk` (potentially к а new chunk number) с `commit_sha = "pending"`. Phase 8 step 7 then runs State H housekeeping: "If commit_sha == 'pending' OR points к unreachable orphan → heal к most recent commit matching chunk progression pattern с title token overlap ≥0.5".

**Subtle ordering issue surfaced this session:** step 7's "pending" check fires on commit_sha that was JUST set к "pending" by step 3 in the SAME wrap. The healing then tries к find а commit matching the new chunk's title — but the CURRENT wrap's commit hasn't been made yet (Phase 10 makes it later). So step 7 looks at git log + finds the PREVIOUS chunk's commit (or finds nothing matching the new chunk's title с ≥0.5 overlap, in which case it leaves "pending" — which is correct).

The token-overlap threshold (≥0.5) effectively GUARDS against incorrect healing because the new chunk's title won't match the previous chunk's commit subject. But this guard is implicit, not explicit design.

**Dogfood evidence (session 116):**
- This wrap: state.yaml.last_completed_chunk advanced 75 → 76; commit_sha = "pending"; step 7 looked at recent commits (21d5663 "chunk #76 route-append amendment archived", 73be075 "chunk #75 documentation consolidation"); no match с ≥0.5 token overlap к the new last_completed_chunk.title "Andromeda pipeline meta-improvements"; step 7 left commit_sha = "pending"; this is the correct outcome.
- But the LOGIC works by accident — the ≥0.5 token overlap check happens к prevent incorrect healing. A future chunk с simpler title (e.g., "P-019 fingerprinting") could spuriously match а recent commit and heal incorrectly.

**Proposal:**

Add an EXPLICIT discriminator к Phase 8 step 7: track whether step 3 just-updated `last_completed_chunk` in this wrap. If step 3 advanced last_completed_chunk in this wrap, step 7 should:

- **NOT heal** commit_sha = "pending" — that's intentional for this wrap's pending state.
- **STILL heal** unreachable orphan SHAs from previous wraps (a "pending" left by previous wrap that didn't get healed; OR а historical SHA that's no longer reachable).

Concrete edit: add а sub-step 7a before the heal logic:

```
7a. Determine whether step 3 advanced last_completed_chunk this wrap:
    - Read state.yaml BEFORE step 3 ran (capture а snapshot at Phase 8 entry).
    - Compare snapshot.last_completed_chunk.route_index vs current value.
    - If current > snapshot: step 3 advanced; this-wrap progressed а chunk;
      commit_sha = "pending" is the deliberate this-wrap pending state →
      SKIP step 7 heal entirely (leave commit_sha = "pending" для next wrap).
    - If current == snapshot: step 3 didn't progress а chunk; commit_sha
      should be the previous wrap's actual SHA OR а "pending" leftover from
      previous wrap → run step 7 heal logic per the existing P16 design.
```

This makes the timing discrimination explicit AND eliminates reliance on the implicit ≥0.5 token overlap guard.

**Design:**

1. Add the snapshot capture к Phase 8 step entry (the start of Phase 8, before step 1).
2. Add the discriminator к Phase 8 step 7 prologue.
3. Update the rationale prose to explain BOTH the previous-wrap-orphan-heal case AND the same-wrap-pending-skip case.

**Implementation cost:**

| File | Change | LOC est |
|---|---|---|
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 8 step entry + step 7 | Snapshot capture + 7a discriminator + rationale prose | ~25 |

**Total:** ~25 LOC в 1 file. Minimal-effort refinement к close the timing-subtlety gap.

**When to do:**

Next-soon. Confidence the pattern recurs: MEDIUM (depends on future chunks с simple title token sets that could spuriously match recent commits с ≥0.5 overlap). Current ≥0.5 threshold IS the implicit guard; tightening к explicit discriminator improves predictability.

Defer-acceptable IF user prefers minimal additional skill surgery this soon after P16 lands; manual heal at next wrap if а spurious match fires.

**Cross-references:**

- Triggering session: 116 (this wrap; first dogfood of P16). Empirical evidence in this session's wrap report.
- Sibling proposal: P16 (the implementation this refines) — IMPLEMENTED 2026-05-22 (session 116).
- `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 8 step 3 + step 7 — load-bearing sections to extend.
- Session 116 Tier 3 session-learning at `.claude/docs/session-learnings.md` (2026-05-22 entry "Self-bootstrap dogfooding paradox is one-skill-invocation-removed, not session-removed") — context for why dogfood is one-skill-invocation-removed.

---

## Status: PROPOSED — 2026-05-22 (session 117)

### Proposal 20 — Self-evolve: cross-session accumulation tracking + patch/refactor maturation gate

**Problem:**

The existing meta-improvement loop (wrap-session Phase 3 step 2 "Andromeda pipeline friction" scan + `docs/andromeda-improvements.md` proposal log + amendment cycle as apply mechanism) sees only the CURRENT session's conversation. It dedups against existing PROPOSED entries by title token overlap >0.6 but cannot detect the PATTERN that keeps generating proposals across sessions. Three concrete consequences observable in the project today:

1. **No accumulation tracking.** P15 (dead `mod tests`) shipped in session 116 as а patch; the structural cause ("pulse-app declares `[lib] test = false` for WebView2 workaround") was never raised because there's nowhere to track "this pattern keeps surfacing". 16 blocks across 16 files all sit awaiting individual remediation; the system can't ask "is this а recurring pattern that needs а structural fix rather than 16 patches?"
2. **No honest-healthy authoring.** When zero proposals file in а wrap, Phase 11 says nothing about meta-observation. Zero candidates is indistinguishable from didn't-look. The user can't audit the scan.
3. **No patch/refactor class distinction.** All entries in `andromeda-improvements.md` are shaped as single fixes. Chunk #76 batched 6 proposals (P7+P12+P15-P18) without anyone asking "do these share а structural cause?" — they might. The schema can't tell.

Specific observable matured pattern in the project today: `state.yaml.living_artifact_freshness.api_surface_deferred = true` for **22 consecutive wraps** (sessions 91-116). The state.yaml field literally cites the count. No patch has been filed because there's no patch to file — the structural cost (cargo +nightly public-api on 14 crates ≈ 7-14 min) exceeds the wrap budget (~3 min). The deferral IS the workflow. This is exactly the case the current meta-layer cannot recognize as actionable.

**Proposal:**

EVOLVE the existing meta-layer at 4 coordinates (no new skill; no parallel log; no separate cron):

1. **`wrap-session/references/curation-guide.md`** gains а "Maturation logic" subsection (Filter 6 parallel to existing 5 quality filters). Classifies each survivor as `noise | patch | refactor`. Wrap-session-local — out of the triangle's 6-contract byte-identity surface (preserves Invariant 7).

2. **`state.yaml`** gains `pipeline_observation_state` field (schema bump v2.1 → v2.2; migration step in wrap-session Phase 8). Persistent cross-session registers:
   - `recurring_patch[pattern_key]`: count, first/last observed session, related_proposals, evidence list, matured_at_session, resolved_in_chunk
   - `patch_cascade[area_key]`: same shape
   - `amendment_clustering[plan_path]`: same shape
   - `high_severity_recurrence[drift_id]`: same shape
   - `deferral_recurrence[artifact_path]`: same shape
   Justification for this one new field (only non-pure-transformation in the design): cross-session accumulation has nowhere to live in the existing schema. `state.yaml.drift_warnings[].first_observed_session_count` is per-drift not per-pipeline-pattern. `andromeda-improvements.md` is unstructured prose. State.yaml IS the cross-session state hub (Invariant 1); adding а field is purely additive and uses the existing schema-migration mechanism (Invariant 14).

3. **`wrap-session` Phase 3 step 2** body extends:
   - **2a** (existing): scan this session's conversation for friction candidates.
   - **2b** (NEW): for each candidate that passes Filters 1-5, classify per `curation-guide.md` §Maturation logic.
   - **2c** (NEW): update `state.yaml.pipeline_observation_state.registers.*` from session evidence + amendment archive + drift_warnings with first_observed_session_count > 7 + deferral counter increments.
   - **2d** (NEW): scan registers for newly-matured patterns; file а Refactor R{N} entry in `andromeda-improvements.md` (class: refactor) with accumulation evidence MANDATORY in entry body.

4. **`docs/andromeda-improvements.md`** schema extends: each entry gets `**Class:** patch | refactor`; refactor entries additionally require `**Accumulation evidence:**` field citing ≥3 patches OR matured-criterion-specific data. Existing P1-P19 backfill as `Class: patch` (one-line edit). New refactor entries use `### Refactor R{N} — {title}` heading parallel to `### Proposal P{N}`.

5. **`wrap-session` Phase 11** extends with а "Pipeline meta-observation" subsection rendering one of three modes:
   - **Mode P** (patch filed): lists patches + pattern_key + register touched
   - **Mode R** (refactor matured): lists refactor + accumulation evidence + scope class + routing
   - **Mode H** (honest healthy, evidence-backed): lists registers scanned с current counts + closest-to-maturing top 3 + explicit "nothing matured this wrap" conclusion. NOT а silent void — а demonstrated scan.

6. **`new-session` Phase 9** dashboard extends with а "Matured pipeline patterns" subsection reading `state.yaml.pipeline_observation_state.registers.*.matured_at_session != null` entries. Section omitted entirely when empty (preserves new-session read-only role per Invariant 19).

**Design:**

Maturation thresholds (defaults; project-tunable in `curation-guide.md`):

| Pattern | Threshold | Justification |
|---|---|---|
| recurring_patch | ≥3 distinct proposals sharing pattern_key | 3 is smallest count that's not coincidence |
| patch_cascade | ≥2 patches in same area within 5-wrap window | Cascade signature |
| amendment_clustering | ≥4 amendments to same plan within 5 wraps | Almost-every-wrap touch = instability |
| high_severity_recurrence | drift_id age > 7 wraps | Beyond stale-drift escalation (which fires at >3) |
| deferral_recurrence | ≥10 consecutive wraps with `*_deferred = true` | Deferral became the workflow; only refactor can change |

Refactor entry routing by scope class (uses existing skills — no new pathways):

| Scope class | Apply path | Invariant preservation |
|---|---|---|
| Specialist plan amendment | `/andromeda-evolve` + `--delta` | Invariant 2, 8 |
| Arch registry section | `/andromeda-evolve --allow-arch-registry` | Invariant 3 (narrow exception) |
| Route additive | `/andromeda-evolve --allow-route-append` | Invariant 4 (narrow exception) |
| Specialist plan re-derive | `/andromeda-{specialty}` greenfield re-run | Existing path |
| Route restructuring | `/andromeda-route` greenfield re-run | Existing path |
| Arch re-plan | `/andromeda-arch` greenfield re-run | Cascades via documented path |
| Cross-skill contract | Manual 3-way coordinated edit + `/andromeda-setup-project` Phase 8 md5sum verify | Invariant 7 |
| Cross-skill USER-level skill body | META-chunk via /andromeda-evolve --allow-route-append + /andromeda-phase + /andromeda-implement | Invariant 9 (P17 path) |

Anti-pattern safeguards:
- **Refactor without accumulation citation:** rejected; Mode H rendered instead ("tried to mature but evidence insufficient").
- **Refiling matured pattern:** `matured_at_session != null` + `resolved_in_chunk = null` → no refile; visible in closest-to-maturing list.
- **Premature IMPLEMENTED:** one-wrap-lag verification — Status PROPOSED → IMPLEMENTED only after next wrap confirms register actually cleared. Parallel to State H housekeeping (Invariant 17).

First application (would be R1 if self-evolve active today): `api_surface.md` 22-wrap deferral. Walkthrough — detect (read state.yaml; matured at count=22≥10) → track (register updated) → propose (Refactor R1, scope class: Cross-skill contract — extend integrity-protocol.md Part B per-crate iteration) → review (user reads in new-session dashboard) → apply (3-way byte-identical edit + setup-project Phase 8 verify) → verify (next wrap reconciles 1 crate at ~0.5min cost; deferral flag clears) → archive (R1 → IMPLEMENTED).

**Implementation cost:**

| File | Change | LOC est |
|---|---|---|
| `~/.claude/skills/andromeda-wrap-session/references/curation-guide.md` | NEW "Maturation logic" subsection (Filter 6 + thresholds + routing table + anti-patterns) | ~120 |
| `~/.claude/skills/andromeda-wrap-session/references/session-state-contract.md` Part B | ADD `pipeline_observation_state` schema | ~40 |
| `~/.claude/skills/andromeda-setup-project/references/session-state-contract.md` Part B | byte-identical copy of above | ~40 |
| `~/.claude/skills/andromeda-new-session/references/session-state-contract.md` Part B | byte-identical copy of above | ~40 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 3 step 2 | EXTEND с 2b/2c/2d sub-steps | ~30 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 8 | ADD v2.1→v2.2 migration step (seed `pipeline_observation_state` + backfill `deferral_recurrence['api_surface']` from existing state.yaml.api_surface_deferred_reason text) | ~25 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 11 | EXTEND report template с "Pipeline meta-observation" subsection (Modes P / R / H) | ~40 |
| `~/.claude/skills/andromeda-wrap-session/references/visual-references.md` | ADD Mode P / Mode R / Mode H banner templates | ~30 |
| `~/.claude/skills/andromeda-new-session/SKILL.md` Phase 9 | EXTEND dashboard с "Matured pipeline patterns" subsection (omitted-if-empty rendering) | ~20 |
| `~/.claude/skills/andromeda-new-session/references/visual-references.md` | ADD matured-patterns subsection template | ~15 |
| `docs/andromeda-improvements.md` (this project) | Backfill P1-P19 with `**Class:** patch` (one-line per entry; mechanical) | ~20 |

**Total:** ~420 LOC across 11 files (incl. 3-way byte-identical triangle copies × 3). META chunk implementable via P17 sibling-skill orchestration path. Setup-project Phase 8 md5sum verifies triangle byte-identity.

**When to do:**

After P19 (P16 timing discriminator) lands — they don't conflict but P19 is cheaper (~25 LOC, single file) and matures the State H story before this larger evolution. P20 itself is ~420 LOC across 11 files; substantial META chunk. Justification for the cost is direct: 22-wrap api-surface deferral is the existing matured pattern; without P20, no mechanism exists to surface it as а refactor candidate beyond ad-hoc human attention.

Defer-acceptable IF user prefers continuing per-wrap proposal filing without cross-session pattern tracking — the existing meta-layer continues to work for individual patches; only refactor-class observations are missed.

**Cross-references:**

- Triggering session: 117 (this wrap; self-evolve design experiment commissioned by user via two-step prompt). Empirical evidence: 22-wrap api-surface deferral preserved in state.yaml.living_artifact_freshness.api_surface_deferred_reason text since session 91.
- Step 1 schema (this session conversation): localized friction to (skill × phase × invariant × artifact) coordinates — the substrate this design assumes.
- Sibling proposals: P15 (dead mod tests detection — а patch that would be tracked in `recurring_patch['dead-mod-tests']` for future maturation if pattern recurs); P17 (META-chunk orchestration — the path P20 implementation uses); P19 (P16 timing discriminator — sequenced before P20 per "When to do" above).
- `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 3 step 2 + Phase 8 + Phase 11 — load-bearing sections to extend.
- `~/.claude/skills/andromeda-wrap-session/references/curation-guide.md` — new "Maturation logic" subsection home.
- `~/.claude/skills/andromeda-{setup-project,wrap-session,new-session}/references/session-state-contract.md` Part B — triangle byte-identity surface for the schema addition.
- `state.yaml.living_artifact_freshness.api_surface_deferred_reason` (this project) — the empirical anchor: 22 consecutive sessions cited verbatim.

---

## Status: PROPOSED — 2026-05-22 (session 119)

### Proposal 21 — `/andromeda-implement` first-class support for chunk-scoped manual specialist plan rewrites (v3 reconciliation path)

**Problem:**

Chunk #77 ("Specialist plan reconciliation (security + tests)" — pulse v0.2.0 FINAL Consolidation Phase 6 chunk) introduced the FIRST instance of the v3 "manual body rewrite" mechanism per `docs/v0_2_0/pulse-v0_2_0-route.md` §77 Mechanism note. The chunk explicitly declares "Specialist plan touches: security-plan (definitely — manual body rewrite of §Threat Model + §Data Protection + §Secret Management + §Anti-Pattern Logging), test-plan / .claude/rules/testing.md (definitely — manual §Pending coverage triggers update + materialize deferred tests)" in its canonical chunk description. The /implement skill's MUST NOT clause categorically forbids modifying these files except via Trigger 4 → Path A (spec-drift dialogue triggered by UNEXPECTED gap between specialist plan + impl). Chunk #77's rewrites are PLANNED — not drift; not surprise. P17 (META-chunk inline sibling-skill orchestration) covers а different case (Implementation Steps invoke sibling skills like /andromeda-evolve), but chunk #77's plan.md directly Edits the spec files — there's no sibling skill to orchestrate.

Concrete observation at chunk #77 implementation:
- /implement Phase 1 step 0 META detection looked for sibling-skill invocations OR USER-level skill body edits — neither matched (Steps 1-5 target `.andromeda/security-plan.md`; Step 6 targets `.claude/rules/testing.md`; project files but NOT skill files); chunk classified as `standard`.
- The standard Phase 1 path would fire the MUST NOT clause immediately on Step 1's Edit attempt.
- /implement had to surface the ambiguity via AskUserQuestion and the user approved a "chunk-scoped exception" branch (1 of 3 options: execute all 13 steps treating chunk plan as authoritative override of MUST NOT clause).
- The dialogue worked, but is friction that will recur on EVERY future v3 reconciliation chunk (specialist re-derivation is deferred к v3 per chunk #77 Mechanism note; reconciliation chunks are how v2 covers the gap until then).

**Proposal:**

Add а new recognition path in /implement Phase 1 step 0 (alongside the existing META-chunk classification per P17):

Detection signal — "Chunk-scoped manual specialist plan rewrite":
- plan.md `## Implementation Steps` include Edit/Write operations targeting `.andromeda/{security,design,test,obs,a11y,layout-templates}-plan.md` OR `.claude/rules/*.md` paths, AND
- plan.md `## Codebase touchpoints > Files к modify` list explicitly enumerates these spec/rule files (not silent extension), AND
- combined.md OR research.md (or referenced docs like `docs/v0_2_0/pulse-v0_2_0-route.md` §N) explicitly declares the chunk performs "manual body rewrite" of these plans within а declared "Specialist plan touches" metadata field.

Routing — new Phase 1c "Chunk-scoped spec rewrite orchestration":
- Applies the spec edits per plan with same audit-trail discipline as Trigger 4 → Path A (write amendment marker at `.andromeda/runs/{ISO}-spec-amendment-chunk-{N}-{slug}/amendment.md` capturing the rewrite scope + Decisions Log entry; append `state.yaml.spec_amendments.active` entry with `flag_used: --chunk-scoped-rewrite` + `chunk_index: N` + `noted_at: null` for lifecycle progression).
- DOES NOT prompt user — chunk-plan approval at /phase Phase 6 IS the authorization (chunk #77's plan.md was approved by user pre-implementation per /phase Phase 6 user review).
- Standard Phase 2 fix-loop applies post-orchestration (verify no regressions from the spec edits + the chunk's code work).

**Design:**

- Detection precedence: Phase 1 step 0 sub-step order: (1) META sibling-skill check per P17; (2) Chunk-scoped spec rewrite check per this proposal; (3) fall through к standard chunk classification. The two recognition paths are mutually exclusive in practice (META chunks invoke skills; v3 reconciliation chunks Edit directly) but order matters if а future chunk does both.
- Audit-trail equivalence: the amendment marker + state.yaml lifecycle preserves the spec-amendment-protocol.md contract. Wrap-session Phase 6 D5 amendment-aware classification + Phase 8 lifecycle progression apply unchanged. The only difference от Path A: no user dialogue (chunk-plan approval substitutes).
- D4 drift detection: chunk's "Specialist plan touches" metadata defines the within-scope plan list. Edits within this list = within-scope; edits outside = D4 drift fires (matches current discipline per route §77 Mechanism note).
- Composability с P17: а future chunk could BOTH invoke а sibling skill AND directly Edit а spec file. Phase 1b (META orchestration) + Phase 1c (spec-rewrite orchestration) run in sequence if both detected.
- Failure mode: if Phase 1c detects the signal but plan.md's Files-to-modify list is INCOMPLETE relative к the spec edits actually attempted (research.md drift), surface as Trigger 4 deferred ("Path A' fix impl OR Path B defer") — falling back к the existing dialogue.

**Implementation cost:**

| File | Change | LOC est |
|---|---|---|
| `~/.claude/skills/andromeda-implement/SKILL.md` Phase 1 step 0 | EXTEND META detection с new sub-step 0b "Chunk-scoped spec rewrite detection" + branch routing | ~25 |
| `~/.claude/skills/andromeda-implement/SKILL.md` Phase 1c | NEW phase "Chunk-scoped spec rewrite orchestration" (orchestration loop + amendment marker write + state.yaml.spec_amendments append) | ~60 |
| `~/.claude/skills/andromeda-implement/SKILL.md` constraints MUST NOT clause | UPDATE EXCEPTION list к include "Phase 1c chunk-scoped rewrite" alongside existing Trigger 4 → Path A | ~5 |
| `~/.claude/skills/andromeda-implement/references/visual-references.md` | ADD Phase 3 success variant "chunk-scoped-spec-rewrite-orchestrated" (parallel к existing "amendment-applied" / "META-chunk-orchestrated") | ~15 |
| `~/.claude/skills/andromeda-implement/references/spec-drift-protocol.md` | ADD section "Phase 1c vs Trigger 4 path A — when each applies" (decision tree) | ~30 |
| `docs/andromeda-improvements.md` | mark P21 IMPLEMENTED post-application | ~5 |

Total ~140 LOC across 5 files at user-level skill toolkit + 1 project file.

**When to do:**

When the NEXT v3 reconciliation chunk fires (likely chunk #78+ if pulse-v0_2_0 evolves toward Phase 7+ surfaces that need specialist re-touch; OR а future scope's reconciliation chunk). Filing now captures the friction while fresh; implementation pays off when the next dialogue would have fired.

Empirical anchor: chunk #77 took ~3min of dialogue ceremony (user question + option selection + acknowledgment) that the proposal removes. Across N future reconciliation chunks (estimated 2-5 across pulse-v0_2_0 remaining), saves 6-15min of friction + standardizes the audit trail (current dialogue path doesn't generate amendment markers; rewrites are visible only in chunk implementation commit body).

**Cross-references:**

- `docs/v0_2_0/pulse-v0_2_0-route.md` §77 Mechanism note — canonical declaration of the v3 manual-rewrite mechanism
- Chunk #77 implementation commit body (session 119) — first dogfood; established the "chunk-scoped exception" precedent via /implement Phase 1 user dialogue
- Proposal 17 (META-chunk inline sibling-skill orchestration) — parallel pattern for the different META case (sibling-skill invocation rather than direct spec Edit)
- spec-amendment-protocol.md Part C (amendment-aware D5 classification) — applies unchanged to amendments generated by Phase 1c
- session-learnings.md 2026-05-22 (session 119) — Tier 3 entry documenting the v3 chunk-scoped manual specialist plan rewrite path
- Invariant 1 (state.yaml is wrap-session's territory) + Invariant 14 (schema migration is Phase 8's one-time responsibility) + Invariant 7 (triangle byte-identity) + Invariant 19 (new-session read-only) — preserved by the design; documented in Step 1 schema this session.

---

## Status: PROPOSED — 2026-05-23 (session 129) — REFACTOR

### Refactor R1 — Per-crate incremental api-surface reconciliation (deprecate batch-reconcile pattern)

## Status: IMPLEMENTED — 2026-05-24 (session 135) — REFACTOR — verified cleared session 135

(Filed PROPOSED 2026-05-23 session 129; applied 2026-05-24 session 135 chunk #82; verification_condition `consecutive_count == 0` PASS at wrap-session 135 Phase 8 step 8 — required common-sense P22 override for matured_at_session preservation through step 4b.i. This is the FIRST refactor entry to traverse the entire self-evolve compounding loop DETECT → PROPOSE → APPLY → VERIFY end-to-end.)

**Class:** refactor

**Accumulation evidence:** (REQUIRED iff Class=refactor)
- Accumulator: `api_surface_deferral` in `state.yaml.pipeline_accumulators`
- Current count: 33 (threshold: 10)
- First observed: session 91
- Last observed: session 129
- Matured at: session 128 (the moment Phase 8 step 4a migration first detected past threshold per Modification 1 present-reality discipline)
- Evidence snapshot: "per-crate cargo +nightly public-api iteration across 14 crates exceeds wrap budget (7-14 min vs ~3 min); 32nd consecutive deferral per sessions 91-126 pattern; cumulative backlog ~150+ new pub items unaccounted-for since session 91 baseline; re-baseline EXPLICITLY warranted at next non-META wrap"

**Problem:** Current `integrity-protocol.md` Part B reconcile protocol treats api-surface.md as a single-batch artifact — wrap-session Phase 5 runs `cargo +nightly public-api --simplified --workspace` across all 14 workspace crates in one pass. Per-crate cost (~0.5min/crate) × 14 crates = 7-14min total, vs. the ~3min wrap budget. Result: 33 consecutive wraps have set `state.yaml.living_artifact_freshness.api_surface_deferred = true` and preserved the flag across wraps rather than reconciling. The artifact has gone stale by ~150+ new pub items since the session 91 baseline. No patch-shape exists because the cost is structural — patching "reconcile faster" requires a different reconcile strategy, which is structural change.

**Proposal:** Extend `integrity-protocol.md` Part B reconcile protocol to support per-crate incremental mode specifically for api-surface.md (dependency-tree.md unchanged — batch is fast enough). Mechanism:

1. Extend `living_artifact_freshness` schema (in `session-state-contract.md` Part B — also a shared contract; 3-way byte-identical edit) with a cursor field: `api_surface_next_crate: {crate_name}` (string; cycles round-robin through workspace crates; defaults to first crate alphabetically).
2. Extend `api-surface.md` LIVING block schema (in `integrity-protocol.md` Part A) with per-crate sub-block markers `<!-- LIVING:api-surface:crate-{name} start --> ... <!-- LIVING:api-surface:crate-{name} end -->`.
3. Phase 5 reconcile changes (in `wrap-session/SKILL.md` Phase 5):
   - Pop `api_surface_next_crate` from state.yaml.
   - Run `cargo +nightly public-api --simplified -p {crate}` (single crate; ~0.5min).
   - Replace the matching `<!-- LIVING:api-surface:crate-{name} -->` sub-block atomically.
   - Advance cursor to next crate (wrap to first after last).
   - Set `api_surface_reconciled_at` to current ISO; deprecate `api_surface_deferred` flag (no longer needed once per-crate mode active).
4. Cycle-completion detection: when cursor wraps back to starting crate, full cycle completed within ≤14 wraps; api-surface is current within that bound.

**Scope class:** Cross-skill contract

**Routing:** Manual 3-way coordinated edit to two of the 6 shared contracts:
- `~/.claude/skills/andromeda-{setup-project,wrap-session,new-session}/references/integrity-protocol.md` (Part A LIVING block schema + Part B reconcile protocol)
- `~/.claude/skills/andromeda-{setup-project,wrap-session,new-session}/references/session-state-contract.md` (Part B `api_surface_next_crate` cursor field)

After edits: `diff -q` verification (must return empty for both pairs of both contracts). Next `/andromeda-setup-project` Phase 8 md5sum gate verifies 6/6 byte-identical. Also requires update to `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 5 to consume the cursor.

**Implementation cost estimate:**

| File | Change | LOC est |
|---|---|---|
| `~/.claude/skills/andromeda-*/references/integrity-protocol.md` (×3 byte-identical) | Part A extend LIVING block schema with per-crate sub-block markers; Part B extend reconcile protocol with per-crate incremental mode + round-robin cursor + cycle-completion detection | ~80 × 3 = 240 |
| `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 5 | Update reconcile loop to pop cursor + reconcile one crate + advance cursor | ~20 |
| `~/.claude/skills/andromeda-*/references/session-state-contract.md` (×3 byte-identical) | Part B add `api_surface_next_crate` cursor field; document `api_surface_deferred` deprecation path | ~12 × 3 = 36 |
| `.andromeda/context/api-surface.md` (in pulse project) | One-time migration: restructure existing single LIVING block into per-crate sub-blocks (initial empty placeholders; Phase 5 fills over ≤14 wraps) | ~50 |

**Total:** ~346 LOC across 7 user-level skill files + 1 project file. Triangle byte-identity discipline required for integrity-protocol.md + session-state-contract.md (2 of 6 shared contracts).

**Verification gate (one-wrap-lag):**
- Resolution chunk: 82 (set by /andromeda-apply --confirm-applied at 2026-05-24T11:34:51Z)
- Verification condition: `consecutive_count == 0` (from A1 catalogue per Modification 3)
- Verified cleared at session: TBD (filled by Phase 8 step 8 when condition passes)
- Anti-pattern: do NOT mark IMPLEMENTED until verification condition passes; diagnostic remediation required for failed verifications

**When to do:**

NOW. The accumulator has been past threshold for 1 wrap by session 129 (matured at session 128 by Phase 8 step 4a migration; refactor filing fires same wrap as the maturation-detection-after-migration under the B5+B4 fix from session 129). With self-evolve infrastructure fixed via B5+B4, R1 fires automatically on the first wrap with the consumer step 4b running on a populated accumulator — exactly as the session 129 diagnostic trace predicted.

**Cross-references:**

- Related patches: NONE — this pattern has no patch-shape. The closest related pattern with patches is P15 (dead `mod tests`) which IS individually-patchable; api-surface is the rarer "structural-only" case.
- Accumulator state: `state.yaml.pipeline_accumulators.api_surface_deferral` (consecutive_count=33 at filing time; matured_at_session=128)
- Empirical anchor narrative: `state.yaml.living_artifact_freshness.api_surface_deferred_reason` (preserves cumulative deferral context across sessions 91-127; verbose narrative form; one-time read by Phase 8 step 4a migration at session 128)
- Triggering session: 129 (first wrap after B5+B4 self-evolve fix landed; expected first-dogfood case per session 129 diagnostic trace prediction)
- P20 (original self-evolve proposal — session 117; cited 22-wrap deferral as empirical anchor; now 33 wraps and the matured refactor candidate this proposal addresses)
- B5+B4 fix (session 129 wrap-session/SKILL.md edits) — the wrap-local fix that made R1 filing automatic on first activation; documented in session 129 handoff Key Decisions

### Resolution log

- 2026-05-24T11:34:51Z — Phased plan applied manually (HIGH-risk Cross-skill contract; 6-phase plan per `~/.claude/skills-applier-plans/2026-05-24T10-28-32Z-R1.md` dry-run reference); verified by /andromeda-apply --confirm-applied R1 --chunk-id 82. **Resolution chunk: 82.** 8 files touched (7 in skills repo: 3× integrity-protocol.md + 3× session-state-contract.md + 1× wrap-session SKILL.md; + 1× project api-surface.md restructured to 14 per-crate sub-blocks). 14 verification gates PASS (2× [MD5SUM_3WAY] preserved triangle byte-identity for both shared contracts; 4× [READ_AFTER_WRITE]; 8× [NO_COLLATERAL_DAMAGE]; 5× [§12_10_SANITY] sub-checks on wrap-session SKILL.md). [VERIFICATION_CONDITION_INFORMATIONAL] `consecutive_count == 0` evaluates FALSE post-confirm (current=38; expected — per-crate Phase 5 reconcile at next wrap-session will reset count). Awaiting wrap-session Phase 8 step 8 one-wrap-lag verification to transition PROPOSED → IMPLEMENTED. Rollback pointers (still valid): `git -C ~/.claude/skills/ reset --hard pre-R1-apply` + `git -C D:/dev/projects/andromeda-pulse/ restore .andromeda/context/api-surface.md`.

---

### Proposal P22 — Phase 8 step 4b.i clears matured_at_session BEFORE step 8 verification, breaking one-wrap-lag gate for ANY accumulator-driven refactor

## Status: IMPLEMENTED — 2026-05-24 (session 135) — Option 2 applied: 2 edits к `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 8 step 4b.i (conditional preserve when refactor in flight) + step 8 IF-PASS branch (deferred cleanup of matured_at_session post-transition). Skills repo commit base `076a01b` (R1 changes) + `pre-P22-apply` tag for rollback. [READ_AFTER_WRITE] + [NO_COLLATERAL_DAMAGE] (2 hunks / 1 file) + [§12_10_SANITY] 5/5 PASS. Manually-applied patch (no accumulator binding → no Phase 8 step 8 one-wrap-lag gate applies; transition is user-driven per patch convention).

**Problem:** wrap-session Phase 8's intra-phase ordering creates a verification-gap when the same wrap that completes a matured refactor's apply ALSO triggers the natural cycle clear that step 8 needs to observe:

1. Step 4b.i increment-trigger runs FIRST. For accumulators where Phase 5 cleared the deferred flag this wrap (`api_surface_deferred=false` for A1; analogous for future accumulators), the ELSE branch fires and clears: `consecutive_count=0` + `first_deferred_session=null` + `last_deferred_session=null` + **`matured_at_session=null`**.

2. Step 8 verification runs SECOND. Its WHERE clause requires `matured_at_session != null AND refactor_proposal_id != null AND resolved_in_chunk != null AND verified_cleared_at_session == null`. Step 4b.i just cleared `matured_at_session`, so step 8 SKIPS the entry.

3. Result: `verified_cleared_at_session` stays null forever; refactor never auto-transitions PROPOSED → IMPLEMENTED via the gate. User must manually transition with explicit "manual override" annotation — defeating the one-wrap-lag verification design.

**This is not an R1-specific edge case** — it's the general path. EVERY accumulator-driven refactor's implementation IS what triggers the cycle clear (e.g., A1's R1 reconcile clears api_surface_deferred). The bug fires every time, not as a corner case.

**Discovered:** session 135 wrap-session, when tracing through expected behavior of R1's verification before executing Phase 8.

**Proposed fix (one of three options):**

1. **Swap step ordering:** run step 8 BEFORE step 4b.i. Step 8 observes pre-reset state (matured_at_session preserved); on PASS, transitions refactor to IMPLEMENTED. Then step 4b.i performs the natural cycle clear.
2. **Conditional clear in step 4b.i:** preserve `matured_at_session` (and any other fields step 8 needs) when `resolved_in_chunk != null AND verified_cleared_at_session == null` (refactor in flight). Reset only count/dates. Step 8 fires correctly; final cleanup happens after step 8 transitions.
3. **Step 8 wider WHERE:** drop `matured_at_session != null` from step 8's WHERE clause (use `refactor_proposal_id != null` as the primary trigger; matured_at is implied by refactor existing). This handles cleared-during-this-wrap.

Option 2 is least disruptive (smallest protocol change; preserves all design intent including "cycle naturally clears" semantics).

**When to do:** NOW. Without this fix, no accumulator-driven refactor ever auto-transitions. R1 (currently in-flight at session 135) is the first dogfood case. To unblock R1's verification THIS WRAP, session 135 applies common-sense fix per Option 2 with explicit annotation: "preserved matured_at_session for in-flight refactor's verification per P22 proposal pending protocol fix". Future sessions get the same treatment until the protocol is updated.

**Cross-references:**
- Affected accumulator state: `state.yaml.pipeline_accumulators.api_surface_deferral` (R1 in flight at filing time)
- Affected SKILL.md: `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 8 step 4b.i + step 8
- Related: R1 (the first refactor to hit this; would have stalled without the workaround)
- Related: maintainer guide §9.3.2 one-wrap-lag verification design (the design intent that step 4b.i ordering breaks)

---

### Proposal P23 — andromeda-apply HIGH-risk plan should flag target files exceeding Read tool size cap

## Status: PROPOSED — 2026-05-24 (session 135)

**Problem:** /andromeda-apply's design assumes all target files can be Read by the Edit/Write tools (Edit + Write both require Read-before-Edit). Claude Code's Read tool errors on files exceeding ~25K tokens regardless of `limit:` argument. For HIGH-risk apply where the human follows the emitted phased plan, hitting an oversize target mid-phase forces tool-path improvisation (Bash heredoc with atomic `> .tmp; mv .tmp file` workaround).

**Encountered:** session 135 Phase 6 of R1 manual apply. Target `D:/dev/projects/andromeda-pulse/.andromeda/context/api-surface.md` was 7794 lines / 819KB pre-migration. Read tool errored with "File content (26327 tokens) exceeds maximum allowed tokens (25000)" even with `limit: 60`. Switched to Bash heredoc; worked but lost the safety guarantees of Edit/Write tool atomicity.

**Proposed fix:**

1. **Class detection extension (Phase 4):** check each target file's byte size via `wc -c`. For files >250KB (conservative threshold below the ~819KB encountered), flag with "non-skill-large" sub-class. This is in addition to existing class detection (greenfield / triangle maintainer / author / non-skill).
2. **Plan emission (Phase 5):** for non-skill-large targets, the emitted plan includes a note: "TOOL ADVISORY: this target exceeds Read tool's ~25K-token cap. Edit/Write tools will fail Read-before-Edit prerequisite. Use Bash heredoc + atomic rename pattern: `cat > {target}.tmp << 'EOF' ... EOF; mv {target}.tmp {target}`."
3. **Verification gate adjustment:** for non-skill-large targets, [READ_AFTER_WRITE] uses Grep (which works on large files) instead of Read.

**Side benefit:** detecting oversize targets early also informs the user whether the file fits the "non-skill" category as designed (project doc, modest size) or has accidentally accumulated content that should itself be refactored.

**When to do:** opportunistic. P22 is the urgent blocker (R1 needs the verification gate fix). P23 is a UX improvement for future refactors that touch oversize project markdown.

**Cross-references:**
- Encountered chunk/refactor: R1 Phase 6 (project api-surface.md migration)
- Affected SKILL.md: `~/.claude/skills/andromeda-apply/SKILL.md` Phase 4 (class detection) + Phase 5 (plan emission) + `~/.claude/skills/andromeda-apply/references/verification-gates.md` ([READ_AFTER_WRITE] gate implementation)
- Related: applier audit-trail file at `~/.claude/skills-applier-plans/2026-05-24T10-28-32Z-R1.md` Phase 6 deliverable did NOT mention the size constraint despite Phase 4's class detection running on api-surface.md (the cap-violation surfaced only at human-apply time when Read failed)
---

### Proposal P24 — wrap-session should auto-progress Type 6 amendments where expected_propagation is empty (trivially-empty cascade = effectively propagated)

## Status: PROPOSED — 2026-05-25 (session 145)

**Problem:** When а Type 6 arch-registry amendment (per spec-amendment-protocol.md Part D Architecture.md exception → Narrow exception) declares empty `## Expected downstream propagation` (no CLAUDE.md cascade needed; no Tier 2/3 cascade needed — additions land в а §Occupied Resources sub-section that no `GENERATED:setup:*` anchor derives from), the propagation lifecycle stalls indefinitely OR requires а full /andromeda-setup-project re-derive (over-cost для known no-op cascade) OR requires а manual `propagated_by_run` sentinel write (workaround). setup-project --delta is the canonical propagation path BUT its Trigger exact-match defense correctly refuses manually-authored markers — designed-in defense against fake-flag use. This creates а stuck-amendment pattern когда the sibling-skill (/andromeda-evolve) is not Skill-tool-invocable (disable-model-invocation: true) AND user authorizes manual replication of /evolve output.

**Encountered:** session 145 wrap (chunk #84 L4 LLM runtime swap implementation + Type 6 arch-registry amendment manually authored). Chain of constraints:
1. Plan-81 step 11 delegated arch-registry registration к `/andromeda-evolve --allow-arch-registry`
2. /implement skill correctly refused inline orchestration (chunk #84 was NOT META-classified per P17's ≥80% signature; 1/11 steps = 9%)
3. Skill tool refused /evolve invocation (disable-model-invocation: true posture)
4. User authorized manual replication via "proceed" — agent created marker file + arch.md edits + state.yaml entry with honest Trigger field documenting manual provenance
5. setup-project --delta correctly refused propagation per Trigger exact-match defense (per delta-rerun-protocol.md Architecture.md exception → Type 6 permit path step "Defense-in-depth")
6. setup-project full would be 100% no-op cascade (verified: 0 of 8 GENERATED:setup:* anchors derive from §Occupied Resources Environment variables OR §Architecture Registry Updates sub-sections) — pure compute waste
7. Outcome: user picked "skip setup-project + go straight к wrap-session"; wrap-session must handle unpropagated lifecycle

Total friction: 4 user prompts + 2 AskUserQuestion exchanges + ~3 tool-call rounds к navigate а true no-op cascade. The amendment IS effectively propagated (nothing к cascade); the lifecycle machinery doesn't recognize this state.

**Proposed fix:** wrap-session Phase 8 spec_amendments lifecycle progression adds а new auto-progress branch:

```python
# Existing branches preserved.
# NEW: trivially-empty cascade auto-progression
for entry в state.yaml.spec_amendments.active where propagated_by_run is null:
    marker = read_marker_file(entry.marker_path)
    if marker has Type 6 signature (flag_used: --allow-arch-registry):
        expected_prop = parse_expected_downstream_propagation(marker)
        if expected_prop is empty OR contains only "no CLAUDE.md cascade required" sentinel:
            # Trivially-empty cascade: nothing к propagate
            # Auto-set propagated_by_run к а wrap-session sentinel
            entry.propagated_by_run = (
                f".andromeda/runs/{current_wrap_iso}-wrap-session-auto-progress-"
                f"trivially-empty-cascade-verified-{session_count}/"
            )
            entry.noted_at = current_iso
            # Continue к archive branch (entry now has propagated_by_run set)
```

This auto-progresses Type 6 amendments where the marker explicitly declares no Tier 2/3 cascade required. The sentinel `propagated_by_run` value documents the auto-progression provenance for audit-trail clarity (preserves intent: amendment WAS propagated; cascade was just empty).

Defense preserved: only Type 6 amendments (flag_used: --allow-arch-registry) с marker-declared empty expected_propagation can auto-progress. Type 7 route-append amendments OR Type 6 с non-empty expected_propagation still require setup-project --delta (preserves the existing Trigger exact-match defense для cases where propagation MUST cascade).

Edge case: if marker is manually authored (e.g., this session's chain) AND Trigger doesn't match canonical /evolve signature, the wrap-session auto-progress branch STILL fires — но the audit trail в the marker's lifecycle status checkbox + state.yaml sentinel both document the manual provenance. setup-project --delta's strict Trigger check remains unchanged (it stops manual markers from triggering Tier 2/3 cascade; wrap-session's path is purely lifecycle-progression-only, no Tier 2/3 writes).

**Why this matters:** Type 6 arch-registry amendments are the most common amendment type post-MVP (every chunk that adds а workspace crate / TauRPC procedure / broadcast topic / env var triggers one). Many of these don't actually require Tier 2/3 cascade (env var additions, broadcast topic additions). The current lifecycle machinery treats all Type 6 amendments как requiring setup-project --delta propagation, but а subset are trivially-empty cascades that don't need it. Recognizing this auto-progresses lifecycle cleanly + saves compute + reduces stuck-amendment carryover.

**When to do:** moderate priority. The workaround (skip setup-project; wrap-session sentinel write OR carry-over к next session) works но adds friction per Type 6 amendment с empty cascade. Implementation cost is modest (~30-50 LOC в wrap-session SKILL.md Phase 8 + protocol update в spec-amendment-protocol.md Part D).

**Cross-references:**
- Encountered chunk/amendment: chunk #84 L4 LLM runtime swap + manual Type 6 amendment at `.andromeda/runs/2026-05-25T12-34-46-spec-amendment-acknowledge-chunk-84-llama-bin-paths/amendment.md` (session 145 wrap)
- Related precedent: P17 META-chunk inline orchestration (covers ≥80% META path); P24 covers the complementary path где а non-META chunk has а minority sibling-skill invocation step + user authorizes manual replication
- Affected SKILL.md: `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 8 spec_amendments lifecycle progression block
- Affected references: `references/spec-amendment-protocol.md` Part D Architecture.md exception → Type 6 permit path subsection (add trivially-empty-cascade auto-progress sub-clause)
- Affected referenced delta protocol: `~/.claude/skills/andromeda-setup-project/references/delta-rerun-protocol.md` (no change required; defense-in-depth Trigger check stays unchanged — setup-project --delta refuses manual markers correctly; wrap-session is the proper lifecycle resolution path для trivially-empty cascades)

---

### Proposal P25 — wrap-session prior-block supersede leaves live duplicate keys that YAML-last-win-shadow last_completed_chunk (silently defeats the State H heal)

## Status: PROPOSED — 2026-05-29 (session 160)

**Problem:** When wrap-session supersedes `state.yaml.last_completed_chunk` and preserves the prior block as commented narrative (the `# PRIOR last_completed_chunk:` pattern), an incomplete comment-out can leave some of the prior block's keys (`epoch` / `committed_at` / `commit_sha` / `commit_subject`) UNcommented at the same 2-space indent. Because they stay inside the `last_completed_chunk:` mapping, they become live YAML DUPLICATE KEYS — and YAML last-key-wins means the stale prior values silently shadow the current block's values. The State H housekeeping heal (Proposal 16) edits the FIRST `commit_sha:` line, but the duplicate lower in the mapping wins at parse time, so the heal is silently ineffective.

**Encountered:** session 160 wrap. `last_completed_chunk` correctly showed `route_index: 89` + chunk-89 `title` (those keys were NOT duplicated), but `python yaml.safe_load` resolved `commit_sha: 4489ae3` + `committed_at: 2026-05-25T18:45:37Z` + `commit_subject: "chunk(86): ..."` — chunk #86's metadata, stale since session 151. Root cause: the session-151 supersede commented `# PRIOR last_completed_chunk:` + `# route_index: 86` + `# title: ...` (3 lines) but left `epoch: 9` / `committed_at:` / `commit_sha: 4489ae3` / `commit_subject: "chunk(86)..."` (4 lines) uncommented. So for ~9 sessions (151 -> 159) every State H "heal" edited the first commit_sha line while the duplicate won at parse time — new-session dashboards reading `commit_sha` would have shown the stale 4489ae3. Fixed this wrap by completing the comment-out.

**Proposed fix (two complementary guards):**
1. **Phase 8 duplicate-key detection (primary):** after the atomic state.yaml write, parse with a duplicate-key-rejecting loader (a `yaml.SafeLoader` subclass overriding `construct_mapping` to raise on duplicate keys, OR a post-parse scan asserting exactly 1 non-comment occurrence each of `commit_sha`/`committed_at`/`commit_subject`/`route_index` under `last_completed_chunk`). Surface a fatal-class warning on any duplicate. Catches the class regardless of how the duplicate arose.
2. **Supersede discipline (preventive):** when superseding `last_completed_chunk`, DELETE the prior block (git history + amendment markers are the audit trail; prior chunk metadata is recoverable) rather than comment it. If narrative preservation is desired, comment EVERY line of the block — never a prefix.

**Why this matters:** `last_completed_chunk.commit_sha` is read by new-session State H detection + the dashboard "Last completed chunk" line + the State H heal itself. A silent duplicate-key shadow means all three operate on stale data while reporting success — exactly the failure mode the State H machinery (Proposal 16) was built to prevent, defeated one layer below. The bug is invisible to line-level edits (the heal "succeeds") and only surfaces under a full YAML parse.

**When to do:** moderate priority. Cheap to implement (~15-25 LOC: a duplicate-key scan in wrap-session Phase 8 + a one-line discipline note in the supersede step).

**Cross-references:**
- Encountered: session 160 wrap (chunk #90 route-append propagation META wrap); fix completed the session-151 comment-out at `state.yaml` last_completed_chunk block.
- Related: Proposal 16 (State H "pending" -> real-SHA heal — this bug silently defeated that heal's parse-time effect for ~9 sessions).
- Affected SKILL.md: `~/.claude/skills/andromeda-wrap-session/SKILL.md` Phase 8 step 3 (last_completed_chunk supersede) + step 7 (State H housekeeping — add duplicate-key guard).

---

### Proposal P26 — /andromeda-implement does not author an amendment record for PLANNED specialist-plan edits (route "Specialist plan touches: X"), only for Trigger-4 drift — so setup-project --delta later halts on empty active list

## Status: PROPOSED — 2026-05-29 (session 161)

**Mode:** P (patch)

**Problem:** A route chunk whose entry declares "Specialist plan touches: {plan}" is expected to edit that specialist plan during /andromeda-implement (e.g., chunk #90 route §90 "Specialist plan touches: design-system" → /implement Step 1 edited design-system.md §Motion). But /andromeda-implement's spec-drift-protocol.md only authors an amendment record (marker + state.yaml.spec_amendments.active append) for **Trigger-4 harness-fired drift** — NOT for a PLANNED specialist-plan edit that is part of the chunk's declared scope. The planned edit lands directly in the specialist plan (+ its Decisions Log entry) with NO amendment record. Then /andromeda-setup-project --delta (the documented propagation mechanism per the 2026-05-16 manual-edit→delta precedent) HALTS at Setup step 1 because state.yaml.spec_amendments.active is empty — there is no tracked amendment to propagate, even though a real specialist-plan amendment was just made + its Tier 2/3 distillations are stale.

**Encountered:** session 161 (chunk #90 "Halo formula refactor"). /implement Step 1 amended design-system.md §Motion (Halo breathing band + hue driver) per route §90's declared design-system touch + user Q2 authorization, but authored no amendment record. /setup-project --delta then could not run (empty active list). Resolution this session was the "formalize amendment first" path: retroactively wrote the marker + state.yaml.active entry (per session-144 manual-formalization precedent), THEN --delta propagated to design-summary.md + a11y.md + frontend.md + CLAUDE.md (grep-expansion caught 3 beyond the plan→file table's design-summary.md-only prediction).

**Proposed fix (pick one or combine):**
1. **/andromeda-implement authors the amendment for planned specialist-plan edits.** When a plan.md Implementation Step edits `.andromeda/{specialist}-plan.md`, /implement writes the amendment marker + state.yaml.active append (same Part A/B discipline as Trigger-4), Trigger = "chunk #X planned specialist-plan touch (route §X)". Makes planned-edit → --delta propagation seamless; no retroactive formalization.
2. **/andromeda-phase flags specialist-plan-edit steps** so /implement knows to author the amendment.
3. **setup-project --delta halt diagnostic gains a "formalize manual edit" branch** — the current halt recommends "full re-derive" but not "if you made a manual specialist-plan edit, formalize it as an amendment first".

**Why this matters:** FIRST Type 1-5 specialist-plan BODY amendment in this project (all 27+ prior amendments were Type 6 arch-registry or Type 7 route-append, authored by /andromeda-evolve). The gap was invisible until a chunk's route entry declared a specialist-plan touch that /implement executed as a planned edit. As more chunks touch specialist plans (design-system / a11y / etc.), this halt recurs. Fix 1 is cleanest (seamless propagation); Fix 3 is cheapest (diagnostic-only).

**When to do:** moderate priority. Fix 3 ~5 LOC; Fix 1 ~30-50 LOC + a spec-drift-protocol.md section.

**Cross-references:**
- Encountered: session 161 (chunk #90 implementation + setup-project --delta formalize-amendment path). Marker: `.andromeda/runs/2026-05-29T19-20-03-spec-amendment-halo-severity-motion-token/amendment.md`.
- Related: 2026-05-16 CLAUDE.md session-learning (manual specialist-plan edits + setup-project --delta propagation); session-144 precedent (manual amendment formalization under user authorization).
- Affected SKILL.md: `~/.claude/skills/andromeda-implement/references/spec-drift-protocol.md` (add planned-specialist-edit amendment-authoring path) + `~/.claude/skills/andromeda-setup-project/references/delta-rerun-protocol.md` Detection step 3 halt diagnostic (add formalize-manual-edit branch).
