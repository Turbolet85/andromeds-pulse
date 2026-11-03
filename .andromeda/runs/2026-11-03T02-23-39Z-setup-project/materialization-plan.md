# Materialization plan — andromeda-pulse · re-run, upgrade form

Invocation: re-run for U56 only (the overseer, founder-delegated, 2026-11-03). Development Style: agent-driven
(`fct-s6oev5of`). Stack fragment: rust. No upstream body read; no structural change named.

## Tier 1
- overview · modules · warnings (10 bullets) · pointer-table · workflow · architecture: not re-derived —
  cascade-maintained, each standing byte for byte. U48 n/a — no pointer-row edit.
- imports: the template's (U01 ok) — unchanged.
- deeper-topics: recomputed from the files that exist — the docs list (5 summaries, 5 core, 14 services,
  session-learnings) and the rules list (8 files) equal the block as it stands → no Edit; CLAUDE.md not written.

## Tier 2
Present, each `preserve`: a11y.md · design-tokens.md · frontend.md · host-linux.md · observability.md · security.md ·
testing.md · verification-harness.md. Host leaf: U04 `noted` (9 template lines missing above `## Session Additions`)
→ the card's Re-run row; regenerate on the operator's word only.

## Tier 3
Present, each `preserve`: stack · conventions · commands · gotchas · workflow (U09 noted) · the five summaries (U36
noted) · services/ (14) · session-learnings · andromeda-after-mvp-playbook. Absent: none.

## Agent harness
`scripts/agent-run.sh` · `scripts/agent-run.ps1` present → preserve (U10 noted); validate the five verbs.

## Hooks
U02 ok → `.claude/settings.json` not written.

## Code-graph pipeline
U03 ok (0 lines behind each) · `scip_pb2.py` · `requirements.txt` present → nothing seeded.

## Code reviewer
`.claude/agents/code-reviewer.md` present → preserve.

## Gitignore
U56 behind → append the base ignore `.andromeda/runs/*/prompts/` to the root `.gitignore` (idempotent by git's
matcher: the line goes in only because U56 reads the path unignored). U07 ok · U06 `.gitattributes` ok → unchanged.

## Seeds (Phase 6)
state.yaml · session-handoff.md · drift-base.md · playbook.md · session-learnings.md present → nothing seeded.

## Upgrade
HEAD at 5b: `26940f39f63ed519d813c14b512d05688f6fa97e`
Path set at 5b (all expected-transient or untracked run-dir paths):
- ` M .claude/session-handoff.md`
- `?? .andromeda/db/log/2026-11-03T02-16-07-session-2026-11-03.ndjson`
- `?? .andromeda/db/log/2026-11-03T02-20-22-2026-11-03-stdio-sidecar-retired.ndjson`
- `?? .andromeda/runs/2026-11-03T02-18-46Z-phase/` (9 files — the cancelled take-up's record, kept on the
  overseer's word)

```
upgrade v1.21 · 5e33236c
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-gra… · lines behind: code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0
U04 · noted · setup · .claude/rul… · 9 template line(s) missing above `## Session Additions` — regenerate on the operator's word
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts… · scripts/agent-run.sh lacks `ensure_fresh_artifacts`
U11 · ok · setup · code-metrics.ndjson line endings (and, until S1's deletion, the friction le… · code-metrics.ndjson LF
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · n/a · hand · .andromeda/{doc}-amendments.md entry form · no amendment sidecar yet
U14 · ok · hand · working-route markerless i… · 71 markerless entries · 0 introducers behind markup or after the em-dash
U35 · n/a · hand · masters' lo… · n/a: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security L, design L, layout L
U36 · noted · noted · .claude/docs/{security,design,te… · .claude/docs/security-summary.md lacks `cascade re-derives it`
U48 · n/a · setup · CLAUDE.md pointer rows · no keyed-contract index in the tree
U56 · behind · setup · .gitignore base ignore `.androm… · not ignored by the root .gitignore: .andromeda/runs/*/prompts/
U57 · ok · hand · the friction family in the store — `.andromeda/db/{moved, log/*.ndjson, reco… · friction: in the store
U58 · ok · hand · the spine in the store — `version` · `epoch` · `chunk` · `carry` · `capability`… · spine: in the store
U59 · ok · hand · a chunk's and a version's documents and files in the store — `doc` · `section` ·… · docs: in the store
U60 · ok · hand · the masters in the store — the seven `.andromeda/{doc}.md` bodies, their cate… · masters: in the store
upgrade: for setup 1 (U56) · awaiting a door 0 · noted 4 (U04, U09, U10, U36) · INDETERMINATE 0 · 22 detectors of 54 registry entries
```

Consistency: one setup-class row behind (U56), no INDETERMINATE; every other planned artifact preserved; no
handed follow-on (U08 ok, U13 n/a, U14 ok).
