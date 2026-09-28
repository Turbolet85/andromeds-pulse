# Materialization plan — andromeda-pulse (re-run / upgrade)

Run: `2026-09-28T21-35-29Z-setup-project` · Development Style: **agent-driven** · host: win32 (Git Bash over Windows)
Upstreams read at P0: input.md · architecture.md · security-plan · design-system · layout-templates · test-plan ·
obs-plan · a11y-plan · master-route.md (all present). Phases 1–6 act on THIS file only.

## Upgrade

- HEAD at 5b: `83d40601fdb3758ec82650a9d06a22f4b7c94387`
- 5b path set (all expected-transient or untracked run dirs):
  `.andromeda/friction-log.ndjson` (M) · `.claude/session-handoff.md` (M) · `.andromeda/code-metrics.ndjson` (??) ·
  `.andromeda/runs/2026-08-31T07-46-32Z-evolve-diagnose/**` (??) · `.andromeda/runs/2026-08-31T08-07-26Z-code-audit/**` (??)
- route cursor at 5b: `records 57 · complete 57 · pending 0` · `half-promote 0 of 57` · 0 `NOT DERIVED`
  (6 UNPARSED + 1 INDETERMINATE markerless-tail abstentions — not guard-bearing)
- `upgrade.py detect` listing (verbatim; raw twin `detect-p0.txt`):

```
upgrade v1.0 · c78f62d3
U01 · behind · setup · CLAUDE.md @imports block · 3 import lines — a known prior form (@.andromeda/architecture.md @.an…
U02 · behind · setup · .claude/settings.json hooks · write dead-env · bash absent · post 5 dead-env
U03 · behind · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph…
U04 · behind · setup · .claude/rules/host-win32.md · absent — setup P2 renders it on a Windows host
U05 · behind · setup · rustfmt.toml · Cargo.toml without rustfmt.toml — setup P5 writes it (the card names the reflow)
U06 · behind · setup · .gitattributes · .gitattributes absent
U07 · behind · setup · .gitignore base ignores · not ignored by the root .gitignore: __pycache__/ · zz/a/__pycache__/
U08 · behind · hand · .andromeda/playbook.md seed rules · 6 of 6 seed rules unmatched: Sequencing deferral · Not this c…
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh lacks `ensure_fresh_artif…
U11 · behind · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson MIXED · …
U12 · behind · setup · .andromeda/residuals.md header · header 0d2fe82e2c81fb02 — a known prior form (the setup upgrade…
U13 · behind · hand · .andromeda/{doc}-amendments.md entry form · 7 of 7 sidecars · 206 of 206 entries off-form — the c…
U14 · ok · hand · working-route markerless introducers · 1 markerless entries · 0 introducers behind markup or after th…
upgrade: for setup 9 (U01, U02, U03, U04, U05, U06, U07, U11, U12) · awaiting a door 2 (U08→0-pending wrap supersession, U13→0-pending wrap consolidation) · noted 2 (U09, U10) · INDETERMINATE 0 · 14 detectors of 29 registry entries
```

Row actions: U01 → P1 · U02 → P5 · U03 → P6 (proposal — operator accepts at P7) · U04 → P2 · U05 → P5 · U06 → P5 ·
U07 → P5 · U11 + U12 → P7.5 after "yes" · U08 / U13 handed (card) · U14 ok · U09 / U10 noted (card).

## Tier 1 — CLAUDE.md

Disposition: GENERATED blocks re-derived against the template; `USER:session-learnings` preserved verbatim.
The existing GENERATED content is current (cascade-maintained); P1 applies the template deltas as anchored edits:

- **overview / modules / workflow** — unchanged (14 library crates + pulse-app + xtask; key commands current).
- **warnings (top 10)** — kept, severity-ordered security > a11y > obs:
  1. OTLP loopback-only bind (security)
  2. DuckDB prepared statements (security)
  3. post-`prost` invariant checks (security)
  4. product-binary path env var canonicalization + the L4 narrowed exception (security)
  5. TauRPC `EXPECTED_PROCEDURES` pin + negative-default core-API grants (security)
  6. self-observation never dials own OTLP (obs / arch)
  7. NEVER log raw OTLP attrs / snapshot / clipboard / MCP bodies / full plugin paths / DuckDB params (security)
  8. serde `AppError` at the bridge (security)
  9. `prefers-reduced-motion` app-wide (a11y)
  10. Fault identity is DECIDED (arch Established Decision)
  **Rejected (audit):** "Pin every third-party GitHub Action by 40-char SHA" — path-scoped to `.github/workflows/`
  and carried verbatim by the unconditionally-loaded `.claude/rules/security.md` §Supply chain + CI (no loss).
  Fault Identity is KEPT (reversing the dry-run's proposal): with `architecture.md` de-imported by U01, Tier 1 is its
  only loaded carrier (testing.md mentions it only incidentally). Also rejected: rust-toolchain ≥1.85 (config-scoped,
  in security.md) · tonic duplicate (security.md) · MCP stdout ban + double-gate (path-scoped to mcp-server; in
  obs/security rules) · design Universal Bans (path-scoped to UI → design-tokens.md / frontend.md) · test
  anti-patterns (testing.md).
- **pointer-table** — keep the 17 rows; add the template's `Directory tree · resource registry` anchor to the
  Occupied-Resources row; architecture.md + master-route.md are named rows (not imported).
- **architecture** — primary-source line → "the pointer table's row — not imported; read explicitly where a step
  needs it".
- **imports (U01)** — `@.claude/session-handoff.md` only; maintainer note → template text.
- **deeper-topics** — rules list gains `host-win32.md` (Windows/MSYS host recipes).
- Budget: 156 → ~154 lines (≤200).

## Tier 2 — .claude/rules/

| file | disposition |
|---|---|
| security.md | preserve |
| testing.md | preserve |
| observability.md | preserve |
| a11y.md | preserve |
| verification-harness.md | preserve |
| frontend.md | preserve |
| design-tokens.md | preserve |
| host-win32.md | **render** (absent; generating host is Windows — U04) — template verbatim + empty `## Session Additions` |

migrations / api / events: not planned (no arch markers — no migration framework, no external REST API beyond
spec-fixed OTLP, events are in-process broadcast).

## Tier 3 — .claude/docs/

All present → preserve: stack · conventions · commands · gotchas · workflow (U09 noted: lacks `it never commits`) ·
security-summary · design-summary · tests-summary · obs-summary · a11y-summary · session-learnings ·
andromeda-after-mvp-playbook · services/{buffer, config-watcher, corpus, curation, ingest, interpretation,
mcp-server, plugins, security, snapshot, triage, ui-bridge, viz, workspace-detector}.md (14). Nothing regenerated.

## Agent harness

agent-driven → `scripts/agent-run.{sh,ps1}` present (233 / 205 lines, project-evolved through
2026-08-30-agent-harness-teardown-truth) → **preserve**. A fresh render differs (template 106 / 107 lines, pre-build
+ four-token cleanup verdict absent) → back up both to `.claude/backup/agent-run.{sh,ps1}.pre-setup-2026-09-28T21-35-29Z`,
card names the drift for manual merge (U10: no `ensure_fresh_artifacts` hook). verification-harness.md preserve.

## Hooks (U02)

Resolution: Rust primary (arch §Stack) → rustfmt file-scoped; clippy stays a GATE (no write-time row).
`.claude/settings.json`: back up, then
- PreToolUse `Edit|MultiEdit|Write` (dead-env write guard) → REPLACED by the matrix write guard
  `Edit|MultiEdit|Write|NotebookEdit` (stdin, backslash-normalised, exit 2).
- PreToolUse `Bash` guard → ADDED (6500-byte Windows cap + cat/tee heredoc ban).
- PostToolUse `Edit|MultiEdit|Write` (5 dead-env rows: rustfmt · workspace clippy --fix · 2 TS · tsc) → REPLACED by
  the stdin rustfmt row (`timeout 30`).
- TS rows: **operator's decision** — prettier is not a dependency of pulse-app/ui; eslint ^9 is (file-scoped row
  `npx --no-install eslint --fix "$f"` available); tsc has no file-scoped row. Not rendered unless named.
- `env` kept (`PYTHONUTF8=1`, `PYTHONIOENCODING=utf-8` already present).

## Formatter config (U05)

`rustfmt.toml` beside `Cargo.toml`: `edition = "2024"` (from `[workspace.package] edition`). Written into a tree
with Rust source → next chunk's FIRST step: `cargo fmt --all` with `cargo fmt --all -- --check` as a Test Command;
the following wrap pins it as a PREREQ on the next markerless entry.

## Code reviewer

`.claude/agents/code-reviewer.md` present (rust, project-tailored) → preserve.

## Code-graph pipeline (U03)

Planes: rust (root Cargo.toml) + ts (pulse-app/ui tsconfig). Per-file only-if-missing: all 5 present
(`code-graph.py`, `code-graph-views.sql`, `scip_pb2.py`, `requirements.txt`, `code-graph-cookbook.md`).
Drift rule: `code-graph.py` (55 diff lines), `code-graph-views.sql` (45), cookbook (above the
`Project-specific query learnings` marker) differ → back up + PROPOSE; fresh copies staged in this run dir as
`proposed/`; applied only on the operator's word. A replaced views.sql triggers a one-time rebuild per plane at the
next query. `scip_pb2.py` / `requirements.txt` stay.

## Gitignore (U07) / Gitattributes (U06)

- `.gitignore`: append `__pycache__/` (any depth) — the only base path U07 names unignored; the rust fragment's
  paths are already matched.
- `.gitattributes`: absent; index all-LF (3850 i/lf · 9 i/-text · 3 i/none · 0 i/crlf · 0 i/mixed) → write
  `* text=auto eol=lf` with its comment. Worktree: 171 w/crlf + friction-log w/mixed. Card names the operator
  re-checkout after the commit on an EMPTY full porcelain (bookkeeping committed first).

## Seeds

state.yaml · session-handoff · drift-base · playbook · session-learnings · code-graph scripts — all present, nothing
seeded.

## Handed / noted

- U08 → 0-pending /andromeda-wrap-session, judgment-base supersession (6 of 6 seed rules unmatched)
- U13 → 0-pending /andromeda-wrap-session, consolidation (7 sidecars · 206 entries off-form)
- U14 ok · U09 / U10 noted (card)
