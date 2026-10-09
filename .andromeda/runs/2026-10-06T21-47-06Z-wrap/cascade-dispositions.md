# Cascade dispositions — the 2026-10-06T21-47-06Z 0-pending wrap

Amendments of this pass (facts this wrap produced or measured; the overseer, founder-delegated, directed them taken
in this wrap, in its conversation, 2026-10-06):
- **A** — obs-plan §8, the muted-diagnostic backlog bullet: "the head entry" → "the entry" (this wrap minted a new
  entry ahead of "L4 generation records render unredacted").
- **B** — test-plan §1 `l4-latency-p99-ps1-run-coverage` and obs-plan §10's L4 gpu-primary row: `pwsh` is no longer
  absent on the dev host (`/usr/bin/pwsh`, `PowerShell 7.6.6`, measured at this wrap); only the `.ps1` run is owed.

Search: `cascade v1.1 · 43eb863f`, baseline `373b5760` (HEAD, after the U35 migration commit), patterns in
`cascade-patterns.toml` — `head-entry` (fixed, case-insensitive) · `redaction-owner` (the entry's title) ·
`pwsh-absent` (the four absence phrasings) · `founder-sudo` · `ps1-half` (the true claim that the half has not run,
swept to find every restatement and its stated reason) · `ps1-trigger` (the trigger's id). Every control fired on
the pre-pass masters. Swept: the seven masters, every `.andromeda/registries/**` file, the three curation homes,
the two judgment bases, the leaf bodies. NOT looked for: other mentions of `pwsh` that state no absence (the
Windows invocation discipline in `rules/observability.md` and `docs/obs-summary.md` — a true claim sharing the
token), and the handoff, which this wrap rewrites whole.

| Row | Disposition |
|---|---|
| `head-entry` · 0 rows | amended — the one pre-pass site (`obs-plan.md:464`) is the amended line; no other master, registry file, leaf, curation home or base calls it the head entry |
| `.andromeda/obs-plan.md:464` `redaction-owner` standing edited | amended (A) — the title stands, the positional word is gone |
| `founder-sudo` · 0 rows | amended — the one pre-pass site (`test-plan.md:145`) is the amended line |
| `.claude/rules/observability.md:99` `pwsh-absent` + `ps1-half` leaf | re-derived — the generated body (above `## Session Additions` at line 123) restated "no `pwsh` on the dev host"; now reads `pwsh` 7.6.6 present, only the run owed |
| `.claude/docs/obs-summary.md:82` `pwsh-absent` + `ps1-half` leaf | re-derived — the SLO table row restated "(no `pwsh`)"; now reads present, only the run owed |
| `.andromeda/test-plan.md:145` `ps1-half` + `ps1-trigger` standing edited | amended (B) — "has never run" stands (true); the absence and the sudo clause are gone |
| `.andromeda/obs-plan.md:548` `ps1-half` + `ps1-trigger` standing edited | amended (B) — "has not run" stands (true); the absence is gone |
| `ps1-trigger` leaf 0 | no change — no test-plan leaf (`docs/tests-summary.md`, `rules/testing.md`) carries the trigger, so none restates its reason |
| curation 0 · base 0 on every pattern | no change — no curation home or judgment base carries either retired wording |

Lateral binds: `test-plan §3 ↔ obs-plan §3` and the a11y↔obs schema are untouched (neither amendment is in a §3 key
file). The one cross-master pair this pass touches — test-plan §1's trigger and obs-plan §10's row citing it — was
amended on both sides in the same pass and reads consistently.

Also corrected in the same wrap, outside the masters: the working-route CARRY (d) on "pre-push:linux runs natively
on Linux" (route-resolve, a tail edit) states the same fact.
