# Materialization plan — andromeda-pulse (re-run / upgrade)

Run: `2026-10-06T21-41-29Z-setup-project` · Development Style: **agent-driven** · host: **Linux (Omarchy)** — the
second setup run on this host (the previous: `2026-10-04T21-20-08Z-setup-project`, HEAD `268ae86`).

Upstreams at P0: all 9 present (input.md · architecture.md · security-plan · design-system · layout-templates ·
test-plan · obs-plan · a11y-plan · master-route.md, ~934 KB together). **Read form, stated plainly:** the previous
checkpoint's synthesis (at HEAD `268ae86`) was taken as the base, and this run read the WHOLE master delta
`268ae86..5f77859` (5 files, +40 / −27 lines; every changed word, 33.8 KB as a word-diff) — the same deviation from
the letter's "read all upstreams in full" the previous run made and stated, chosen for the same reason: the delta is
the only input that can move a materialization choice. input.md · design-system · layout-templates · a11y-plan are
byte-unchanged since that base. What the delta carries, all field-level refinements inside existing sections:
- arch: the declared `rust-version = "1.95"` equal to the pin (the xtask test `declared_floor_equals_the_pinned_channel`);
  L4 constrained by the committed GBNF `pulse-app/src/l4-output.gbnf` via `--grammar-file` (never `--json-schema-file`);
  the shipped model gemma-4-E4B-it-Q4_K_M with `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0`, `-c 8192`, `-rea off`;
  a new Stack row for the vendored test-only converter (`pulse-app/vendor/llama-cpp/`, Python 3 at test time); the CPU
  route founder-retired 2026-10-05 (still routed in code); THREE product-written locations outside the data dir (the
  per-spawn L4 grammar temp file joins the export sink and the corpus-key lock); prompt versions v2.5 / v1.4-*.
- security-plan: the argv's other operands are first-party; the vendored test-only channel sits outside every scanner;
  the grammar temp file path is never logged; the three-locations count; 1.85.0 named the Edition-2024 minimum, not
  the build floor.
- test-plan: `unit_l4_grammar.rs` (102 unit binaries), the Python 3 requirement on all three runners, the L4 probe pin
  counts, a new pending trigger `l4-latency-p99-ps1-run-coverage`, the corpus skip-arm lock-file test.
- obs-plan: one muted-backlog recurrence OPEN (`interpretation.constrained.generate`, owner = the working route's head
  entry); the gpu-primary L4 SLO row (p99 ≤ 10000 ms, nearest rank, dev-host grader).
- master-route: 6 new `complete` records (74 → 80).

**No new crate, surface, harness verb or rule class** — the 5-command harness contract is unchanged. Each item
already reached CLAUDE.md and the leaves through the wrap cascade (checked against CLAUDE.md as loaded: the overview
names rustc 1.95.0 / `rust-version = "1.95"`, gemma-4-E4B, `--grammar-file`, the vendored converter and Python 3; the
`corpus` module line counts three product-written locations; the path-env-var warning carries the `XDG_RUNTIME_DIR`
carve-out).

## Upgrade

- HEAD at 5b: `5f77859f8ebbb18fe01f6394a6f94bfc66b9ef34`
- 5b path set (all expected-transient):
  `.andromeda/friction-log.ndjson` (M) · `.claude/session-handoff.md` (M) ·
  `.andromeda/runs/2026-10-05T15-18-57Z-wrap/evolve-2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings.json` (M) ·
  `.andromeda/runs/2026-10-05T15-18-57Z-wrap/health-2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings.json` (??, an untracked `.andromeda/runs/**` path)
- route cursor at 5b: `records 80 · complete 80 · pending 0 · gated 0` · `half-promote 0 of 80` · 0 `NOT DERIVED`
  (UNPARSED / INDETERMINATE abstentions on frozen working-route lines 52–125 — pre-existing, not guard-bearing)
- `upgrade.py detect` listing (verbatim):

```
upgrade v1.5 · eeed2076
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write current · bash leading-cd
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · noted · setup · .claude/rules/host-win32.md · 6 template line(s) missing above `## Session Additions` — regenerat…
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh lacks `ensure_fresh_artif…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 4 markerless entries · 0 introducers behind markup or after th…
U35 · behind · hand · masters' logs + keyed contracts · behind: infra K, test K, obs K, a11y K · n/a: test L, obs L, a1…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
upgrade: for setup 1 (U02) · awaiting a door 1 (U35→0-pending wrap U35 door) · noted 4 (U04, U09, U10, U36) · INDETERMINATE 0 · 16 detectors of 36 registry entries
```

Row actions: **U02 → P5** (the only write) · U35 handed (card → a 0-pending /andromeda-wrap-session, the U35 door) ·
U04 / U09 / U10 / U36 noted (card) · every other row `ok` → nothing. No U11 / U12 at P7.5 (both `ok`).

## Tier 1 — CLAUDE.md

Disposition: GENERATED blocks re-derived against the template + the delta → **no change, not rewritten**. 154 lines
(≤ 200); 20 markers parse (the `GENERATED:setup` root + 8 `GENERATED:setup:*` blocks + `USER:session-learnings`); the
one `@`-import (`@.claude/session-handoff.md`) resolves and equals the template's (U01 ok). Backed up anyway (Setup
step 5): `.claude/backup/CLAUDE.md.pre-setup-2026-10-06T21-41-29Z`.
- overview / modules / workflow / architecture — current with the delta (see the check above).
- warnings (top 10) — unchanged from the 2026-10-04 selection, severity-ordered security > a11y > obs:
  OTLP loopback bind · DuckDB prepared statements · post-`prost` invariants · product-binary path env vars (+ the L4
  narrowed exception, + the `XDG_RUNTIME_DIR` carve-out) · `EXPECTED_PROCEDURES` pin + negative-default core-API
  grants · self-observation never dials own OTLP · the NEVER-log list · serde `AppError` · `prefers-reduced-motion` ·
  Fault identity is DECIDED. **Rejected (audit), unchanged:** SHA-pinned Actions · the rust-toolchain floor · the tonic
  duplicate (all carried by security.md) · MCP stdout ban + double-gate (security / observability rules) · design
  Universal Bans (design-tokens.md / frontend.md) · test anti-patterns (testing.md) · the `atexit` / signal-handler
  TLS ban (security.md §Logging). **New candidates from this delta, both rejected as path-scoped:** "the L4 argv passes
  `--grammar-file`, never `--json-schema-file`" (`pulse-app/src/llamacli_inference.rs`; already in the overview's
  stack line and security.md) · "the vendored converter is bumped only with the pinned llama.cpp tag" (security.md
  §Supply chain).
- pointer-table (25 rows by health check 14's count) / deeper-topics — current.
- **Observation, not this run's to write** (`USER:session-learnings` is wrap's): its second seed bullet still says a
  new TauRPC procedure needs "a `pulse-app/capabilities/` JSON entry", which the Critical Warnings block and
  security.md both state does not exist in this project. Named on the card.

## Tier 2 — .claude/rules/

| file | disposition |
|---|---|
| security.md · testing.md · observability.md · a11y.md · verification-harness.md · frontend.md · design-tokens.md | preserve |
| host-win32.md | preserve (U04 noted: 6 template lines missing above `## Session Additions`). The generating host is Linux; the file has no `paths:` frontmatter, so it loads every session and its opening line still says the Bash tool is Git Bash (MSYS). Named on the card; not rewritten here |

migrations / api / events: not planned (no arch markers — unchanged).

## Tier 3 — .claude/docs/

All present → preserve: 5 core (U09 noted on workflow.md) · 5 summaries (U36 noted: header line) ·
session-learnings · andromeda-after-mvp-playbook · services/ ×14. Nothing regenerated.

## Agent harness

agent-driven → `scripts/agent-run.sh` / `.ps1` present, project-evolved → **preserve**. A fresh render differs, as at
the previous run (the project added the pre-build, the four-token cleanup verdict, the spawn/exit recorder; U10
noted). Both scripts are byte-identical to the backups that run took (`.claude/backup/agent-run.{sh,ps1}.pre-setup-2026-10-04T21-20-08Z`,
md5 `692c09c2…` / `682913ea…`), so no second backup of the same bytes is written. verification-harness.md preserve.

## Hooks (U02)

Rust primary → rustfmt file-scoped; clippy stays a gate. `.claude/settings.json`: backed up to
`.claude/backup/settings.json.pre-setup-2026-10-06T21-41-29Z`, then
- PreToolUse `Edit|MultiEdit|Write|NotebookEdit` write guard — already the matrix form (U02 "write current") →
  re-rendered identical.
- PreToolUse `Bash` guard — **REPLACED** by the matrix form: arm (c) now judges a `cd` at EVERY top-level position
  (the start of the command, or directly after `&&`, `||`, `;` or a newline, a `{ …; }` group included), not only a
  leading one; a target that resolves to the session cwd OR the project root passes; a `cd` in a subshell, a pipeline
  or behind `then` / `do` / `else` is not judged. The Windows 6500-byte arm and the cat/tee heredoc arm are unchanged.
- PostToolUse `Edit|MultiEdit|Write` rustfmt row — matrix form, unchanged.
- TS rows — still the operator's decision (prettier is not a dependency; tsc has no file-scoped row).
- `env` kept (`PYTHONUTF8=1`, `PYTHONIOENCODING=utf-8`).
Formatter config: `rustfmt.toml` present (U05 ok) → nothing written, no reflow step owed.

## Code reviewer

`.claude/agents/code-reviewer.md` present (rust, project-tailored) → preserve.

## Code-graph pipeline (U03 ok)

Planes rust + ts. All files present; health check 11 reads `behind py 0 · sql 0 · cookbook 0`. Nothing proposed, no
rebuild owed.

## Gitignore (U07 ok) / Gitattributes (U06 ok)

Both current → nothing appended. Index line endings: 5787 `i/lf w/lf`, 0 `i/crlf`, 0 `i/mixed`.

## Seeds

state.yaml · session-handoff · drift-base · playbook · session-learnings · code-graph scripts — all present, nothing
seeded.

## Handed / noted

- U35 → a 0-pending /andromeda-wrap-session, the U35 door (masters' logs + keyed contracts: infra K, test K, obs K,
  a11y K behind)
- U04 / U09 / U10 / U36 noted (card)
