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

_(Subsequent proposals appended below in chronological order. Each proposal has its own `## Status:` heading and `### Proposal N — title` subheading for navigability.)_
