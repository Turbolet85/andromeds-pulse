# Materialization plan — andromeda-pulse · re-run, upgrade form

Run dir: `.andromeda/runs/2026-10-10T01-50-13Z-setup-project/` (clock read `date -u`: 2026-10-10T01-50-13Z)
Development Style: agent-driven (`architecture.md:273`, `:308`) · stack fragment: `rust` (`architecture.md:14`, §Stack's
language row) · host: `linux` (`upgrade.py host`)
Form: **upgrade** — the invocation named no structural change. No upstream body was read: Setup 3's existence check,
the two `architecture.md` lines above, step 5b, `upgrade.py detect`, `CLAUDE.md` and the `.claude/` listing.

## Tier 1 — CLAUDE.md (154 lines, 18 `GENERATED:setup` marker lines, 1 `USER:*` section)
- `overview` — not re-derived — cascade-maintained (stands byte for byte)
- `modules` — not re-derived — cascade-maintained (stands byte for byte)
- `warnings` — not re-derived — cascade-maintained (10 bullets, not re-selected)
- `pointer-table` — not re-derived — cascade-maintained; no row edit (U48 `ok`, 4 of 4 indexes named)
- `workflow` — not re-derived — cascade-maintained (stands byte for byte)
- `architecture` — not re-derived — cascade-maintained (stands byte for byte)
- `imports` — the template's (U01 `ok`: one import line, `@.claude/session-handoff.md`)
- `deeper-topics` — recomputed from the files that exist: 5 summaries · 5 core · 14 `services/` · `session-learnings.md`
  and 8 rule files — every one already listed, none listed that is absent. No Edit.
  (`.claude/docs/andromeda-after-mvp-playbook.md` exists and is not in the block; it is a project-added doc the
  pointer table names, not one of setup's lists. It stands.)
- `USER:session-learnings` — preserved verbatim (18 bullets).

**Result: no Edit → CLAUDE.md is not written.** Backup taken per Setup 5:
`.claude/backup/CLAUDE.md.pre-setup-2026-10-10T01-50-13Z`.

## Tier 2 — `.claude/rules/` (8 present, each `preserve`)
`a11y.md` · `design-tokens.md` · `frontend.md` · `host-linux.md` · `observability.md` · `security.md` · `testing.md` ·
`verification-harness.md`. Nothing planned beyond them. Host leaf: U04 `ok` (rendered for this host, every template
line present above `## Session Additions`) — no re-seed, no sort (step 8a does not run).

## Tier 3 — `.claude/docs/` (each `preserve`)
Core 5: `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md`.
Summaries 5: `security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md`.
Services 14: `buffer` · `config-watcher` · `corpus` · `curation` · `ingest` · `interpretation` · `mcp-server` ·
`plugins` · `security` · `snapshot` · `triage` · `ui-bridge` · `viz` · `workspace-detector`.
Also present, wrap's or the project's: `session-learnings.md` · `andromeda-after-mvp-playbook.md`.
Absent (of what health checks 5 · 9 · 13 name): none.

## Agent harness (each `preserve`)
`scripts/agent-run.sh` (executable) · `scripts/agent-run.ps1` · `.claude/rules/verification-harness.md`. Upgrade form:
no fresh render, nothing compared.

## Code reviewer
`.claude/agents/code-reviewer.md` — `preserve` (rust).

## Hooks
U02 `ok` — write guard current · Bash guard current · PostToolUse on the stdin prologue. `settings.json` not written.
Formatter config: U05 `ok` (`rustfmt.toml` present).

## Gitignore · gitattributes
U07 `ok` (every base ignore decided by the root `.gitignore`, depth 2 included) · U06 `ok` (`* text=auto eol=lf`
carried). Neither written.

## Code-graph pipeline
Five files present (`code-graph.py` · `code-graph-views.sql` · `scip_pb2.py` · `requirements.txt` ·
`code-graph-cookbook.md`); nothing to seed. Planes by manifest: `rust`, `ts`.
**U03 `behind` — the drift rule's proposal:** `scripts/code-graph.py` is 27 template lines behind (health check 11:
`behind py 27 · sql 0 · cookbook 0`). The fresh copy is the fence of
`references/scripts-templates/code-graph.py.md` (407 lines against the present 387). The change, whole:
the rust plane's indexer is handed every Cargo feature through a transient `cache/rust/indexer.json` and
`--config-path` (removed when the indexer returns), with one retry on the default features when the all-features
index fails, said in the status line; the `argv` lambdas take a fifth argument. `code-graph-views.sql` and the
cookbook above its marker are current — no views rebuild follows from the views file.
Present copy backed up: `.claude/backup/code-graph.py.pre-setup-2026-10-10T01-50-13Z`. The replacement is written
only on the operator's word at the card.

## Seeded artifacts (all present, none written)
`.andromeda/state.yaml` (schema 3) · `.claude/session-handoff.md` · `.andromeda/drift-base.md` ·
`.andromeda/playbook.md` · `.claude/docs/session-learnings.md`.

## Dispositions
`preserve`: all 8 rule files, all 24 docs leaves, the reviewer, both harness scripts. `regenerate`: none named.

## Upgrade
HEAD at Setup 5b: `b71019145e72e56c7e7f3ff4a6f82f25551c9cd2`
Path set at Setup 5b (all expected-transient bookkeeping):
```
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-10T01-19-03Z-wrap/evolve-2026-10-09-pre-push-check-native-on-linux.json
 M .claude/session-handoff.md
```
`route.py cursor`: `records 87 · complete 87 · pending 0 · gated 0` · `half-promote 0 of 4 stamped lines`.

`upgrade.py detect --root .` — verbatim (the tool elides long facts with `…`):
```
upgrade v1.8 · 30b07c54
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · behind · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph…
U04 · ok · setup · .claude/rules/host-{os}.md · every template line present above `## Session Additions`
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh lacks `ensure_fresh_artif…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 77 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, obs K, a11y K · n/a: test L, obs L, a11y L, se…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
U48 · ok · setup · CLAUDE.md pointer rows · 4 of 4 indexes named
upgrade: for setup 1 (U03) · awaiting a door 0 · noted 3 (U09, U10, U36) · INDETERMINATE 0 · 17 detectors of 45 registry entries
```
Acting rows: U03 (setup — the drift rule's proposal). Noted, for the card: U09 (`workflow.md` lacks
`it never commits`), U10 (`agent-run.sh` has no `ensure_fresh_artifacts` hook), U36 (the five summaries' header line,
read on `security-summary.md`). Handed: none awaiting a door. No setup-class row INDETERMINATE.

## Consistency
Every block either stands or has a named row; no file is planned that the tree lacks; the one write outside the run
dir and the backups is `scripts/code-graph.py`, and it waits for the card.
