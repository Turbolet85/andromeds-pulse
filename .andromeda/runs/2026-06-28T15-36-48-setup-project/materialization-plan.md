# Materialization Plan — andromeda-pulse (v2→v3 migration setup-project re-run)

**Date:** 2026-06-28 · **Mode:** re-run (CLAUDE.md has `GENERATED:setup` markers, 149 lines) · **Development Style:** agent-driven
**Context:** v2→v3 clean-cut forward migration. Commit (i) `090571b` seeded the route+state v3 half
(`master-route.md` + `andromeda-pulse-0.3.0/` + lean schema-3 `state.yaml` + `project.yaml` reconcile). This
run materializes the v3 OPERATIONAL LAYER + flips the CLAUDE.md @import, PRESERVING the mature curated
`.claude/` ecosystem.

## Decisive constraint — conservative re-run (preserve curation)
The v3 `rules-templates/` + `docs-templates/` are GENERIC skeletons (e.g. `rules-templates/security.md` =
generic Secrets / Credentials / Input-validation boilerplate; `docs-templates/*` = `{substitute from §X}`
placeholders). The project's `.claude/rules/` + `.claude/docs/` are RICH, pulse-specific, and curated across
186 sessions (e.g. `security.md`: loopback-OTLP / DuckDB-CVE prepared-statements / Tauri capability gating /
WASM sandbox / MCP double-gate + ~25 dated `## Session Additions`). Regenerating Tier 2/3 from the generic
templates would CLOBBER that curation. The migration spec (`pulse-v2-to-v3-migration-diff.md` §3) lists
`.claude/` rules+docs under **"STAYS — same structure; PRESERVING `USER:*` / `## Session Additions`."**
→ PRESERVE Tier 2/3 in place; do NOT regenerate from the generic templates. The migration value of this
re-run is the @import flip + the new v3 operational seeds, exactly as the diff frames setup-project's role.

## Tier 1 — CLAUDE.md (surgical, GENERATED:setup preserved-except)
- Flip @import: `@.andromeda/route.md` → `@.andromeda/master-route.md` (the load-bearing change).
- Maintainer-note comment: `route.md` → `master-route.md`; scopes→versions phrasing.
- §Where to Look: split the Roadmap row → `master-route.md` (forward) + `route.md` + `phases/` (v2 forensic);
  ADD the code-graph row (`.andromeda/cache/tree.db` via `scripts/code-graph.py`); annotate the two living-
  artifact rows as superseded-by-tree.db (deferred retire).
- PRESERVE verbatim: overview / modules / warnings / workflow / architecture GENERATED sections (current
  pulse truth) + `USER:session-learnings`.

## Tier 2 — .claude/rules/ — PRESERVE all 7 (no regeneration). Generic templates would regress.
## Tier 3 — .claude/docs/ — PRESERVE all (5 core + 5 summaries + 12 services + session-learnings).

## Seeds (the v3 operational layer — NEW, this run's real materialization)
- `.andromeda/drift-base.md` — 7-artifact drift-detector starter set.
- `.andromeda/playbook.md` — amendment-validation rules starter (Foundation-sequencing).
- `scripts/code-graph.py` + `code-graph-views.sql` + `scip_pb2.py` + `requirements.txt` +
  `code-graph-cookbook.md` — rust → `rust-analyzer scip` → DuckDB code-graph pipeline.
- `.andromeda/cache/` — gitignored code-graph DB home; `tree.db` built on first phase/wrap.
- `.gitignore`: + `.andromeda/cache/` + `scripts/__pycache__/`.

## Preserve (no change — already correct/curated)
- `.claude/agents/code-reviewer.md` (pulse-tuned; generic rust template would regress).
- `.claude/settings.json` (complete project-tuned hooks: generated-dir block + rustfmt/clippy/prettier/eslint).
- `scripts/agent-run.{sh,ps1}` (project-evolved; only-if-missing rule).
- `.claude/session-handoff.md` (v2-shaped; runtime-owned — the first v3 wrap overwrites it).
- `.andromeda/state.yaml` (lean schema-3, from commit i).
- `.andromeda/context/{api-surface,dependency-tree}.md` (deferred retire post-tree.db, per user Q4).

## @imports (CLAUDE.md): `architecture.md` / `master-route.md` / `session-handoff.md` (master-route = nav hub).
