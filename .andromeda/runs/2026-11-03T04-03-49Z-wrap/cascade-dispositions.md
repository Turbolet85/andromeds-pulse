# Cascade dispositions — 2026-11-03-stdio-sidecar-retired

The search: `cascade.py sweep` over the ten apply writers' `## Patterns` (`apply-*.md`, ids a11y- · arch1- · arch2- ·
ds- · lt- · obs1- · obs2- · sec- · tp1- · tp2-) and main's `cascade-patterns.toml` (main-01 the retired double-gate
ban's wording, main-02 «Toggle MCP Server»), one call, baseline `f2bb258f` (the parent of the oldest pre-CI commit
`d6ee34f4`), after every body amendment of the pass landed (TX 43–69). Every pattern's control fired.
Listing: `total (142 patterns) · 34 rows over 15 files` · `per class · new 0/0 · standing 3/2 · leaf 20/11 ·
curation 7/3 · base 4/1`. The cascade.py sweep listing is the record (this run dir's `cascade-…json` trail).

## Master rows (standing 3/2)
- `architecture.md:202` arch1-18 fct-zyffqbcb — the workspace crate list naming `mcp-server`: no change — a true
  claim sharing the token (the crate stays a workspace member, the tools library the door serves).
- `obs-plan.md:96` tp1-17 fct-ohds4uw3 (edited) — "only `result_type` + `result_count` metadata": no change — true of
  the door's `door.call` record, which this pass's obs-plan 10 made the carrier; the line was amended this pass and
  holds no retired mechanism.
- `obs-plan.md:658` tp1-17 fct-id2evwg6 — the same phrase for LLM clients: no change — true of `door.call`.

## Leaf rows (leaf 20/11) — step 3's set
`.claude/docs/stack.md:21` · `.claude/docs/services/mcp-server.md:6`, `:63` · `.claude/rules/security.md:17`, `:42` ×2 ·
`.claude/docs/conventions.md:23`, `:51` · `.claude/docs/services/workspace-detector.md:4` · `.claude/docs/tests-summary.md:82`,
`:84` ×2, `:132`, `:145` · `.claude/docs/obs-summary.md:191` · `.claude/docs/gotchas.md:47` ·
`.claude/docs/services/ui-bridge.md:17` · `.claude/rules/testing.md:79`, `:95` · `.claude/docs/design-summary.md:84` —
re-derived: each leaf joins the `cascade.py leaves` set (`--add` where the pass mode does not already list it).

## Curation rows (curation 7/3) — preserve-verbatim, routed to P3
`.claude/rules/security.md:132`, `:176` ×2 · `.claude/rules/testing.md:214`, `:258` ×2 ·
`.claude/docs/session-learnings.md:633` — Session Additions / Tier 3 entries quoting the `--features mcp-server` regen
and the "no-mcp shape" bindings clobber: routed to P3 curation as in-place extensions (dated "[extended …: the
feature and the clobber left with the stdio sidecar]"); never edited by the cascade.

## Base rows (base 4/1) — the judgment base, propose → approve
`.andromeda/playbook.md:78`, `:122`, `:124` @c811, @c867 — the two gate-order rules around the bindings' "no-mcp
shape" and the `--features mcp-server` regen: their subject left with the feature (a default build is now the whole
procedure set). Routed to the propose → approve → append channel: proposed to the operator on the wrap card, never
a cascade edit; until the operator's word the rules stand as written.

## Leaves
Mode: pass (`cascade.py leaves`, 17 groups · 32 leaves, `--add` of step 2's leaf files); unmapped 2 — `.claude/rules/host-linux.md`
(the host's leaf, no master states it) and `.claude/docs/andromeda-after-mvp-playbook.md` (derives from no master): no comparer, no change.
`cascade.py leaves --apply` over 16 comparer files: `total · blocks 111 · applied 106 · preserved 4 · reshaped 1`.
- by hand: `.claude/rules/observability.md:58` (reshaped — a whole-line removal): the `app.boot.workspace_key` PII bullet removed (its record retired).
- declined, carried to the handoff: the 4 preserved rows — `gotchas.md:46` and `:51` (headings «… rmcp stdio reservation»), `services/mcp-server.md:1`
  (the title «MCP stdio Sidecar»), `rules/security.md:39` (heading «MCP feature double-gate») — headings the tool never edits; their bodies were rewritten;
  a heading rename is the next wrap's or a hand edit with the leaf read.
- not applied: `leaf-architecture+test-plan.md` (`.claude/docs/commands.md`) — `--apply` refused it (`read:` says 192 lines, the file holds 191);
  `render --respawn` refused both `--failed HEADER` (the check passed) and `--failed RULED --rows` (no judged check for a leaf), so no re-spawn
  exists for it; its findings are not written and `commands.md` is carried to the handoff for a re-derive.
- the decisions rest on the dry-run rows and their receipts (`receipts ok` for all 16 files); the comparer files were not each re-read whole by main
  in this window (context at ~89%) — a deviation from §Cascade step 3, named on the wrap card.
