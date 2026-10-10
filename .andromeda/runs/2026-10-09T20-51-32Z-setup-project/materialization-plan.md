# Materialization plan — andromeda-pulse · re-run, upgrade form

Run dir: `.andromeda/runs/2026-10-09T20-51-32Z-setup-project/` · Development Style: agent-driven · host: linux ·
stack: rust.

The operator's words at invocation: a re-run between chunks, for registry U02 (the two guard entries in
`.claude/settings.json` swapped for the installed hook calls) and U49; at the card, `regenerate host-linux.md`; the
card shown before anything is written; nothing written beyond what those two entries name; stop before the commit.
No structural change was named, so this is the upgrade form: no upstream body was read.

## Tier 1 — CLAUDE.md
- `overview` · `modules` · `warnings` · `pointer-table` · `workflow` · `architecture`: not re-derived —
  cascade-maintained, each standing byte for byte.
- `imports`: the template's (U01 ok) — no Edit.
- `pointer-table`: U48 ok (4 of 4 indexes named) — no Edit.
- `deeper-topics`: compared with the `.claude/docs/` and `.claude/rules/` files that exist — no difference, no Edit.
- CLAUDE.md is not written (154 lines).

## Tier 2 — .claude/rules/
- `a11y.md` · `design-tokens.md` · `frontend.md` · `observability.md` · `security.md` · `testing.md` ·
  `verification-harness.md`: preserve.
- `host-linux.md`: regenerate (the operator's word at the card, U49) — through
  `upgrade.py apply --root . --id U04 --regenerate --run-dir {run_dir}`; the tool's dry-run read
  `50 → 47 lines` above `## Session Additions`, the 1 line below it byte-identical.
- absent: none.

## Tier 3 — .claude/docs/
- 5 core, 5 specialist summaries, 14 `services/` files, `session-learnings.md`,
  `andromeda-after-mvp-playbook.md`: preserve. Absent: none.

## Agent harness
- `scripts/agent-run.{sh,ps1}`: preserve; no fresh render made, nothing compared (upgrade form).

## Hooks
- U02 behind (`write inline · bash inline`): the two PreToolUse entries are replaced by the matrix's calls,
  `bash ~/.claude/skills/andromeda-tools/hooks/write-guard.sh` and
  `bash ~/.claude/skills/andromeda-tools/hooks/bash-guard.sh`; matcher, type and timeout unchanged.
- The PostToolUse rustfmt row equals the matrix's row byte for byte — not rewritten. The `env` block is present —
  not rewritten.
- Backup: `.claude/backup/settings.json.pre-setup-2026-10-09T20-51-32Z` (md5 `d1848088eb0334932d4f1035234a0e3e`).

## Code-graph pipeline
- U03 ok — nothing seeded, nothing proposed.

## Code reviewer
- `.claude/agents/code-reviewer.md`: preserve.

## Gitignore
- U07 ok, U06 ok, U05 ok — nothing written.

## Upgrade
Setup 5b: HEAD `8a89e3681c940bdfe21e9eed50cdfc0547e426df`; the dirty path set, all expected-transient bookkeeping:
- ` M .andromeda/friction-log.ndjson`
- ` M .andromeda/runs/2026-10-09T19-51-29Z-wrap/evolve-2026-10-09-boot-smoke-s-early-exit-found-and-closed.json`
- ` M .claude/session-handoff.md`

`route.py cursor`: `records 86 · complete 86 · pending 0 · gated 0` · `half-promote 0 of 3 stamped lines vs 86
master records`.

`upgrade.py detect --root .`, as printed:

```
upgrade v1.8 · 30b07c54
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write inline · bash inline
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
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
upgrade: for setup 1 (U02) · awaiting a door 0 · noted 3 (U09, U10, U36) · INDETERMINATE 0 · 17 detectors of 45 registry entries
```

U49 (class `operator`, no detector): the act is `regenerate host-linux.md` at the card. Its premise was read on this
host before the card: a `\\` pair reached bash intact, single-quoted and inside a quoted heredoc (one probe each;
this session, this host only).

`upgrade.py host --root .`: `host: linux` · `.claude/rules/host-linux.md · rendered for linux · 3153 B · 1 item(s)
below ## Session Additions` · `leaf_md5 628ee262627c9d17930715b09f62d83c`. No re-seed (the leaf is this host's).
