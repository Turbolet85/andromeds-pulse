# Materialization plan — andromeda-pulse (re-run / upgrade)

Run: `2026-10-07T20-59-36Z-setup-project` · Development Style: **agent-driven** (architecture.md §Cross-cutting
Patterns, line 270) · host: **linux** (`upgrade.py host`) — the third setup run on this host (the previous:
`2026-10-06T21-41-29Z-setup-project`, HEAD `5f77859`, committed as `1124148`).

**The operator's brief for this run:** produce the Phase 7 card with its Host leaf row and stop there — the card is
the founder's to approve, brought to him by dialog; change nothing beyond what the re-run itself applies. So this run
ends at the card: no P7.5, no P8, no P9 until his word comes back.

Upstreams at P0: all 9 present (input.md · architecture.md · security-plan · design-system · layout-templates ·
test-plan · obs-plan · a11y-plan · master-route.md, 884 KB together). **Read form, stated plainly:** the previous
checkpoint's synthesis (at HEAD `5f77859`) was taken as the base, and this run read the master delta
`5f77859..f18c631` in two parts — the same deviation from the letter's "read all upstreams in full" the two previous
runs made and stated, for the same reason: the delta is the only input that can move a materialization choice.
- **The registry migration (`373b576`, U35):** 4 masters lost 443 lines to one file per key under
  `.andromeda/registries/contracts/` (architecture §Infrastructure Patterns 4 keys · test-plan §3 7 keys · obs-plan §3
  8 keys · a11y-plan §3 10 keys, as `registry.py contracts` lists them). Read as a move by its stat and its record;
  the key files are byte-unchanged since (`git diff --stat 373b576..HEAD -- .andromeda/registries` is empty). Not
  re-read key by key.
- **The three chunk wraps after it (`373b576..f18c631`):** 5 masters, +15 / −12 lines, every changed word read
  (22.9 KB as a word-diff). input.md · design-system · layout-templates · a11y-plan are unchanged in that range.
  - arch [Fault Identity]: the framing instruction obliges the first hypothesis to name the cue line's `scope_id`
    (prompt v2.6 / v1.5-fallback / v1.5-reflection); `select_corpus_matches` takes `triggering_scope` and, under a
    cue with a `scope_id`, keeps scope matches of that scope only (`CX`, the founder's choice); what the readings do
    not show stands beside them.
  - security-plan: prompt sizes re-measured (headroom ~2.03×); the dev probe's `--replay` as a harness-only second
    reader of the argv bound; the npm gate green after three advisories closed at source; the 72h window not in
    force before a first release (founder ruling 2026-10-07).
  - test-plan: the L4 probe's pin counts (91, then 157), five in-crate pins on the corpus selection, the npm gate
    running before `npm ci`; `pwsh` 7.6.6 present on this host, so only the `.ps1` grader run is owed.
  - obs-plan: the same `pwsh` reading; one owner reference re-pointed.
  - master-route: 3 new `complete` records (80 → 83).

**No new crate, surface, harness verb or rule class.** The 5-command harness contract is unchanged (its key file
`5-command-implementation.md` is one of the byte-unchanged key files). Each delta item already reached CLAUDE.md and
the leaves through the wrap cascade (checked against CLAUDE.md as loaded: the Fault Identity warning carries the
`TRIGGER:` kind-label-only ruling and the `CX` corpus selection; the security rule carries the `--replay` reader).

## Upgrade

- HEAD at 5b: `f18c631bd545c50466c6177c5ae658dc61f7d2e7` (branch `chore/migrate-pulse-to-v3`)
- 5b path set (all expected-transient bookkeeping):
  `.andromeda/friction-log.ndjson` (M) · `.claude/session-handoff.md` (M) ·
  `.andromeda/runs/2026-10-07T20-29-42Z-wrap/evolve-2026-10-07-l4-probe-reproduces-the-canary-history-miss.json` (M)
- route cursor at 5b: `records 83 · complete 83 · pending 0 · gated 0` · `half-promote 0 of 83` · 0 `NOT DERIVED`
  (UNPARSED / INDETERMINATE abstentions on frozen working-route lines 52–125 — pre-existing, not guard-bearing)
- `upgrade.py detect` listing (verbatim; the tool clips a row at its own width):

```
upgrade v1.6 · 814083ff
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · behind · setup · .claude/rules/host-{os}.md · host-win32.md was rendered for win32, this host is linux — the re-r…
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh lacks `ensure_fresh_artif…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 5 markerless entries · 0 introducers behind markup or after th…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, obs K, a11y K · n/a: test L, obs L, a11y L, se…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
upgrade: for setup 1 (U04) · awaiting a door 0 · noted 3 (U09, U10, U36) · INDETERMINATE 0 · 16 detectors of 40 registry entries
```

The clipped facts, read whole with the row clip widened (the same reading, nothing re-derived): U03 `lines behind:
code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0` · U04 `… the re-run re-seeds it: 2 Session
Additions to sort` · U10 lacks `ensure_fresh_artifacts` · U11 both ledgers LF · U36 `security-summary.md lacks
`cascade re-derives it``.

Row actions: **U04 → the card's Host leaf row, written at P7.5 only after the founder's word** · U09 / U10 / U36
noted (card) · every other row `ok` → nothing. No U11 / U12 at P7.5 (both `ok`). Nothing handed.

## Host leaf (U04) — the re-seed, proposed

`upgrade.py host --root .`: `.claude/rules/host-win32.md · rendered for win32, this host is linux · 7638 B · 2
item(s) below `## Session Additions`` · `leaf_md5 4be4626b14363aec1fc1b9dbd80f9005`.

| n | lines | bytes | to | rule | reason |
|---|---|---|---|---|---|
| 1 | 91-91 | 873 | `learnings` | 3 | mixed — a live lesson (the rust-analyzer flycheck against a `cargo clean` or a long build) whose 2026-10-01 extension names `rust-analyzer.exe` and `Get-CimInstance Win32_Process` |
| 2 | 92-92 | 295 | `keep` | 4 | this host's own mechanics, no old-host clause, within ~600 B, live as measured (a `cd` inside a loop, an `if` or a piped `while` passes the stored guard: exit 0); its "only a LEADING `cd`" clause is behind the 2026-10-06 guard |

Rule 1 (another rule file's class) fits neither: both are host/shell mechanics, the host leaf's own class by
tiebreaker 6. Rule 2 (`drop`) fits neither: no item names only the old host's tools.

The tool's dry-run (`apply --id U04 --sort host-reseed.json --dry-run`, nothing written):

```
upgrade v1.6 · 814083ff
U04: would re-seed .claude/rules/host-win32.md (rendered for win32) → .claude/rules/host-linux.md · 7638 B → 3153 B every turn
U04: 2 items · 1168 B — keep 1 · learnings 1 · drop 0
  .claude/docs/session-learnings.md: Tier 3 — read on demand · 388838 B · +5 line(s)
still naming host-win32.md: 1 line(s) — CLAUDE.md:125
```

**Above the cut (the sort does not cover it):** the body is replaced by the template's linux render — 89 lines /
6467 B → 50 lines / 2857 B (rendered read-only with the tool's `host_body`, shape problems none). The win32-tagged
lines go; so does the whole `## Long single-line files` section (3 bullets, 2156 B), which is in NO host section of
the current template — it entered the leaf through setup run `8b86529`, the template dropped it since, and its rules
stand in the pipeline's `line-write-contract.md`. In this project it stands nowhere else. Listed in
`host-reseed.md`; named on the card.

**`USER:session-learnings` bullets naming the old host:** 0 of 18.

**Files naming the old leaf:** `CLAUDE.md:125` (the `GENERATED:setup:deeper-topics` rule list — P7.5 re-renders it)
and, by rule name without `.md`, `CLAUDE.md:77` (the pointer-table row's rule-name list, also `GENERATED:setup` —
re-rendered with it). `.claude/session-handoff.md` and `.andromeda/friction-log.ndjson` name it too: bookkeeping and
a ledger, neither setup's.

## Tier 1 — CLAUDE.md

154 lines (≤ 200); 20 marker lines parse (the `GENERATED:setup` root + 8 `GENERATED:setup:*` blocks +
`USER:session-learnings`, 18 bullets preserved); the one `@`-import (`@.claude/session-handoff.md`) resolves and
equals the template's (U01 ok). Backed up (Setup step 5): `.claude/backup/CLAUDE.md.pre-setup-2026-10-07T20-59-36Z`
(md5 `6a66d3b1…`, equal to the file).

Disposition: GENERATED blocks re-derived against the template and the delta → **not rewritten before the card.**
- overview / modules / workflow / architecture / imports — current with the delta.
- warnings (top 10) — unchanged from the 2026-10-04 selection, severity-ordered security > a11y > obs: OTLP loopback
  bind · DuckDB prepared statements · post-`prost` invariants · product-binary path env vars (+ the L4 narrowed
  exception, + the `XDG_RUNTIME_DIR` carve-out) · `EXPECTED_PROCEDURES` pin + negative-default core-API grants ·
  self-observation never dials own OTLP · the NEVER-log list · serde `AppError` · `prefers-reduced-motion` · Fault
  identity is DECIDED. **Rejected (audit), unchanged:** SHA-pinned Actions · the rust-toolchain floor · the tonic
  duplicate (security.md) · MCP stdout ban + double-gate (security / observability rules) · design Universal Bans
  (design-tokens.md / frontend.md) · test anti-patterns (testing.md) · the `atexit` / signal-handler TLS ban
  (security.md §Logging) · `--grammar-file` never `--json-schema-file` · the vendored converter's bump rule. **New
  candidates from this delta, all rejected:** "the 72h advisory window binds from the first tagged release" (a
  process ruling inside security-plan §Dependency Security, not a per-file invariant) · "the dev probe's `--replay`
  prints and stores nothing of a prompt" (path-scoped: `pulse-app/examples/l4_decision_probe/`; in security.md) ·
  "the `CORPUS MATCHES:` block is selected per cue, never identity" (already inside the Fault Identity warning).
- deeper-topics — current until the re-seed; line 125 names `host-win32.md` and is re-rendered at P7.5 after "yes".
- **pointer-table — 4 rows are behind the template's U35 form; PROPOSED, not written (a stated deviation).** The
  v1.6 template renders, on a migrated project, the registry beside the section: `§Infrastructure Patterns →
  .andromeda/registries/architecture-contracts.toml, one file per key` and the same for `test-plan.md §3`,
  `obs-plan.md §3`, `a11y-plan.md §3`. The four rows (CLAUDE.md lines 60, 65, 68, 70) still cite the sections alone;
  each resolves through the section's stub line (the handoff carries this). No detector reads it (U35 is `ok`). The
  letter's P1 would write it before the card; this run does not, because the brief is "change nothing else" and the
  card goes to the founder for the Host leaf — it is on the card as the operator's own word (`pointer rows`), apart
  from the Host leaf row, and is written with the P7.5 CLAUDE.md re-render if that word is given.

## Tier 2 — .claude/rules/

| file | disposition |
|---|---|
| security.md · testing.md · observability.md · a11y.md · verification-harness.md · frontend.md · design-tokens.md | preserve |
| host-win32.md | **not touched before the card** — rendered for another host; its re-seed is P7.5's, after the word |
| host-linux.md | absent; written by `upgrade.py apply --id U04 --sort …` at P7.5 (never a hand render) |

migrations / api / events: not planned (no arch markers — unchanged). Nobody named a leaf to regenerate.

## Tier 3 — .claude/docs/

All present → preserve: 5 core (U09 noted on workflow.md) · 5 summaries (U36 noted: header line) ·
session-learnings · andromeda-after-mvp-playbook · services/ ×14. Nothing regenerated. `session-learnings.md` gains
one entry (+5 lines) at P7.5 if item 1 stays `learnings`.

## Agent harness

agent-driven → `scripts/agent-run.sh` / `.ps1` present, project-evolved → **preserve**. A fresh render differs, as at
the two previous runs (U10 noted). Both scripts are byte-identical to the backups of 2026-10-04 (md5 `692c09c2…` /
`682913ea…`), so no second backup of the same bytes is written. verification-harness.md preserve.

## Hooks (U02 ok)

`.claude/settings.json` md5 `d1848088…` equals the post-write hash in the previous run's manifest — the three
setup-rendered entries are the matrix form that run wrote, and `detect` reads them current (`write current · bash
current · PostToolUse on the stdin prologue`). Re-rendered identical → nothing written, no backup of the same bytes.
TS rows remain the operator's decision (prettier is not a dependency; tsc has no file-scoped row). `env` kept.
Formatter config: `rustfmt.toml` present (U05 ok) → nothing written, no reflow step owed.

## Code reviewer

`.claude/agents/code-reviewer.md` present (rust, project-tailored) → preserve.

## Code-graph pipeline (U03 ok)

Planes rust + ts. All five files present; U03 reads `code-graph.py 0 · code-graph-views.sql 0 ·
code-graph-cookbook.md 0` lines behind. Nothing proposed, no rebuild owed.

## Gitignore (U07 ok) / Gitattributes (U06 ok)

Both current → nothing appended. Index line endings: 6163 `i/lf w/lf`, 0 `i/crlf`, 0 `i/mixed` (5 `i/none`, 9
`-text`).

## Seeds

state.yaml · session-handoff · drift-base · playbook · session-learnings · code-graph scripts — all present, nothing
seeded.

## Handed / noted

- Nothing handed (`awaiting a door 0`).
- U09 / U10 / U36 noted (card).

## The card's word (added 2026-10-08T05:08:26Z — the run resumed)

The card was printed 2026-10-07 and the run stopped there, as briefed. The word came back through the operator on
2026-10-08:
- **yes** — the founder approved the Host leaf card as proposed (by dialog, the morning of 2026-10-08, relayed by the
  operator): host 1 `learnings`, host 2 `keep`.
- **pointer rows** — the operator's own technical word, apart from the Host leaf row: the four pointer-table rows
  take the registry form.

What P7.5 wrote on that word, and nothing else:
- `upgrade.py apply --id U04 --sort host-reseed.json --run-dir …` — the old leaf backed up, item 1 appended to
  `.claude/docs/session-learnings.md` (+5 lines), `.claude/rules/host-linux.md` written with item 2 kept byte for
  byte, `.claude/rules/host-win32.md` removed last. `2 items · 1168 B found on disk where the sort says`.
- **One change to the sort after the card, stated:** its `date` went from `2026-10-07` to `2026-10-08`. The tool
  takes the heading's date from the caller ("never this tool's clock"), the heading reads "Moved from the host
  leaf", and the move happened on 2026-10-08. The dispositions, the lines and the title are as the card showed them;
  a fresh dry-run printed the same numbers before the apply.
- CLAUDE.md, six lines inside `GENERATED:setup` blocks: the deeper-topics rule list names `host-linux.md` (line
  125) and the pointer table's rule-name list names `host-linux` (line 77); rows 60 / 65 / 68 / 70 carry the
  registry beside the section (`(keyed: .andromeda/registries/{architecture,test-plan,obs-plan,a11y-plan}-contracts.toml…)`,
  the template's U35 form in the spelling the sibling projects' renders use). 154 lines, unchanged.
- Re-detect: `U04 · ok-uncommitted · … every template line present above ## Session Additions`.
