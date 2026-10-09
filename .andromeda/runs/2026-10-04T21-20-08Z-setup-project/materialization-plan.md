# Materialization plan — andromeda-pulse (re-run / upgrade)

Run: `2026-10-04T21-20-08Z-setup-project` · Development Style: **agent-driven** · host: **Linux (Omarchy)** — the
first setup run on this host; the previous one (`2026-09-28T21-35-29Z-setup-project`) ran on win32 / Git Bash.

Upstreams at P0: all 9 present (input.md · architecture.md · security-plan · design-system · layout-templates ·
test-plan · obs-plan · a11y-plan · master-route.md, ~905 KB together). **Read form, stated plainly:** the previous
checkpoint's full synthesis (at HEAD `83d40601`) was taken as the base, and this run read the WHOLE master delta
`83d40601..268ae86` (7 files, +171 / −116 lines). That is a deviation from the letter's "read all upstreams in full",
chosen because the delta is the only input that can move a materialization choice. What the delta carries: field-level
refinements inside existing sections — crate count words 12 → 14 (Occupied Resources was already 14), toolchain pin
1.95.0 / code floor ≥ 1.89, wasmtime 48.0.5, MCP tools 8 → 9 (`retrieve_incident_events`), the `[License]` decision,
the corpus-key lock-file carve-out (`XDG_RUNTIME_DIR`), `ANDROMEDA_PULSE_HARDWARE_PROFILE`,
`__NV_DISABLE_EXPLICIT_SYNC`, the seven-job ci.yml, new obs records (`app.exit` · `ui.webgpu.adapter` ·
`app.boot.render.posture` · `interpretation.incident.skipped`), new xtask verbs (`pre-push:linux` · `perf:budget` ·
`perf:frame-sample` · `smoke:*`), 17 new route records. **No new crate, surface, harness verb or rule class** — the
5-command harness contract is unchanged. Each of these already reached CLAUDE.md / the leaves through the wrap
cascade (checked: CLAUDE.md names 14 crates, 9 MCP tools, rustc 1.95.0, `render_posture`, the lock-file carve-out).

## Upgrade

- HEAD at 5b: `268ae86b6fd302de8a87ba27cb54e94f3ba8dc81`
- 5b path set (all expected-transient):
  `.andromeda/friction-log.ndjson` (M) · `.claude/session-handoff.md` (M) ·
  `.andromeda/runs/2026-10-04T21-03-56Z-wrap/evolve-2026-10-04-l4-interpretation-names-its-triggering-cue.json` (M)
- route cursor at 5b: `records 74 · complete 74 · pending 0 · gated 0` · `half-promote 0 of 74` · 0 `NOT DERIVED`
  (UNPARSED / INDETERMINATE abstentions on frozen working-route lines — pre-existing, not guard-bearing)
- `upgrade.py detect` listing (verbatim; raw twin `detect-p0.txt`):

```
upgrade v1.4 · 950c4e61
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write current · bash pre-cd
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
U14 · ok · hand · working-route markerless introducers · 3 markerless entries · 0 introducers behind markup or after th…
U35 · behind · hand · masters' logs + keyed contracts · behind: infra K, test K, obs K, a11y K · n/a: test L, obs L, a1…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
upgrade: for setup 1 (U02) · awaiting a door 1 (U35→0-pending wrap U35 door) · noted 4 (U04, U09, U10, U36) · INDETERMINATE 0 · 16 detectors of 35 registry entries
```

Row actions: **U02 → P5** (the only write) · U35 handed (card → a 0-pending /andromeda-wrap-session, U35 door) ·
U04 / U09 / U10 / U36 noted (card) · every other row `ok` → nothing. No U11 / U12 at P7.5 (both `ok`).

## Tier 1 — CLAUDE.md

Disposition: GENERATED blocks re-derived against the template + the delta → **no change**. 154 lines (≤ 200);
markers parse (8 `GENERATED:setup:*` blocks + `USER:session-learnings`); the one `@`-import
(`@.claude/session-handoff.md`) resolves and equals the template's (U01 ok).
- overview / modules / workflow / architecture — current (14 crates, toolchain 1.95.0, 9 MCP tools, the keyring
  feature set, the corpus-key lock, `render_posture`).
- warnings (top 10) — unchanged from the 2026-09-28 selection, severity-ordered security > a11y > obs:
  OTLP loopback bind · DuckDB prepared statements · post-`prost` invariants · product-binary path env vars (+ L4
  narrowed exception, + the `XDG_RUNTIME_DIR` carve-out) · `EXPECTED_PROCEDURES` pin + negative-default core-API
  grants · self-observation never dials own OTLP · NEVER-log list · serde `AppError` · `prefers-reduced-motion` ·
  Fault identity is DECIDED. **Rejected (audit), unchanged:** SHA-pinned Actions (path-scoped, carried by
  security.md) · rust-toolchain floor (security.md) · tonic duplicate (security.md) · MCP stdout ban + double-gate
  (security / observability rules) · design Universal Bans (design-tokens.md / frontend.md) · test anti-patterns
  (testing.md). New candidate considered and rejected: the `atexit`/signal-handler TLS ban — path-scoped to
  `pulse-app/src/observability.rs`, carried by security.md §Logging.
- pointer-table (17 rows) / deeper-topics — current.

## Tier 2 — .claude/rules/

| file | disposition |
|---|---|
| security.md · testing.md · observability.md · a11y.md · verification-harness.md · frontend.md · design-tokens.md | preserve |
| host-win32.md | preserve (U04 noted: 6 template lines missing above `## Session Additions`). **Host note:** the generating host is now Linux; the step-6 rule keeps a project moved off Windows "inert" — but the file has no `paths:` frontmatter, so it loads every session and its opening line states "The Bash tool on this host is Git Bash (MSYS)". Named on the card for the operator; not rewritten here |

migrations / api / events: not planned (no arch markers — unchanged).

## Tier 3 — .claude/docs/

All present → preserve: 5 core (U09 noted on workflow.md) · 5 summaries (U36 noted: header line) ·
session-learnings · andromeda-after-mvp-playbook · services/ ×14. Nothing regenerated.

## Agent harness

agent-driven → `scripts/agent-run.sh` (273 lines) / `.ps1` (248) present, project-evolved → **preserve**. A fresh
render differs (template ~106 / ~107 lines; the project added the pre-build, the four-token cleanup verdict, the
spawn/exit recorder) → back up both to `.claude/backup/agent-run.{sh,ps1}.pre-setup-2026-10-04T21-20-08Z` (no
earlier backup survived the host move — `.claude/backup/` is gitignored); the card names the drift (U10).
verification-harness.md preserve.

## Hooks (U02)

Rust primary → rustfmt file-scoped; clippy stays a gate. `.claude/settings.json`: back up to
`.claude/backup/settings.json.pre-setup-2026-10-04T21-20-08Z`, then
- PreToolUse `Edit|MultiEdit|Write|NotebookEdit` write guard — already the matrix form (U02 "write current") →
  re-rendered identical.
- PreToolUse `Bash` guard — **REPLACED** by the matrix form: adds arm (c), the leading-`cd` guard (a first-word `cd`
  whose target resolves anywhere but the stdin `cwd` is blocked with exit 2 + remedy; `cd` into the cwd itself passes).
  The Windows 6500-byte arm stays (inert on Linux by its `uname` case).
- PostToolUse `Edit|MultiEdit|Write` rustfmt row — matrix form, unchanged.
- TS rows — still the operator's decision (prettier not a dependency; eslint ^9 is; tsc has no file-scoped row).
- `env` kept (`PYTHONUTF8=1`, `PYTHONIOENCODING=utf-8`).
Formatter config: `rustfmt.toml` present (U05 ok) → nothing written, no reflow step owed.

## Code reviewer

`.claude/agents/code-reviewer.md` present (rust, project-tailored) → preserve.

## Code-graph pipeline (U03 ok)

Planes rust + ts. All 5 files present; `code-graph.py` byte-current with its template, `code-graph-views.sql` current
(the 13 differing lines are the template's example-query fence, not SQL), cookbook current above its
`Project-specific query learnings` marker. Nothing proposed, no rebuild owed.

## Gitignore (U07 ok) / Gitattributes (U06 ok)

Both current → nothing appended.

## Seeds

state.yaml · session-handoff · drift-base · playbook · session-learnings · code-graph scripts — all present, nothing
seeded.

## Handed / noted

- U35 → a 0-pending /andromeda-wrap-session, the U35 door (masters' logs + keyed contracts: infra K, test K, obs K,
  a11y K behind)
- U04 / U09 / U10 / U36 noted (card)
