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

## Status: PROPOSED — 2026-05-16 (session 70)

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
