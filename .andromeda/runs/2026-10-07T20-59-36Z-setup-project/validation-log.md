# Validation log — 2026-10-07T20-59-36Z-setup-project (re-run / upgrade U04)

The word at the Phase 7 card, relayed by the operator on 2026-10-08: **yes** — the founder approved the Host leaf card
as proposed (host 1 `learnings`, host 2 `keep`); **pointer rows** — the operator's own. Validation ran 2026-10-08.

## Pre-flight
`pre-flight · ✓ · CLAUDE.md at ./CLAUDE.md` (`health v1.0 · b1b10944`; the tool's trail: `health-no-marker.json`).

## Checks 1–14

| Check | Status | Diagnostic |
|---|---|---|
| 1 CLAUDE.md size | ✓ | 154/200 lines · T1 6.8 KB · 0 of 18 bullets over 600 B |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one import, `@.claude/session-handoff.md`, exists |
| 4 rule files | ⚠ | 8 files · frontmatter 6/6 parsed · always-loaded 2 (94.7 KB: security.md 91.6 KB + host-linux.md 3.1 KB) · security.md 91.6 KB, testing.md 183.1 KB, observability.md 62.9 KB past the read cap — pre-existing; the always-loaded total was 97.6 KB at the previous run |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | architecture.md 2026-10-07 22:44 ≤ CLAUDE.md 2026-10-08 07:07 (local) |
| 7 session-handoff (hand) | ✓ | present, 7617 B |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root .gitignore (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | `schema_version: 3` · `last_wrap` · `tree_db_refreshed_at` · `session_count` only |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 25 entries (threshold 5) |

## Upgrade re-detect (Phase 7.5 step 2)
`upgrade v1.6 · 814083ff` — the one row this run wrote:
`U04 · ok-uncommitted · setup · .claude/rules/host-{os}.md · every template line present above `## Session Additions`` → ✓.
No U11 / U12 apply (both `ok` at P0). Noted rows are the card's: U09 · U10 · U36. Nothing handed.

## The re-seed, as the tool reported it
`2 items · 1168 B found on disk where the sort says — keep 1 · learnings 1 · drop 0` · `7638 B → 3153 B every turn`.
After it, `host-win32.md` is named in no loaded file: CLAUDE.md, `.claude/rules/`, `.claude/docs/` whole,
`.claude/agents/` and `scripts/` were searched, 0 lines. `.claude/session-handoff.md` and the friction log still name
it (bookkeeping and a ledger, neither setup's).

## Hook smoke (formatter · Bash guard · write guard · jq)
Each stored command piped its stdin JSON from the Bash tool's own shell (`jq` at `/usr/bin/jq`; `CLAUDE_PROJECT_DIR`
unset in that shell, so the guard fell back to the shell's directory, the project root). `.claude/settings.json` is
byte-unchanged by this run (md5 `d1848088…`).

| Arm | Expected | Got |
|---|---|---|
| formatter on `.setup-validation-test.rs` (mis-formatted control) | exit 0, file rewritten | exit 0, rewritten; temp file removed |
| Bash guard: `cat > x.md <<'EOF'` | 2 | 2 |
| Bash guard: `python - <<'PY'` | 0 | 0 |
| Bash guard: `cd .andromeda && ls` | 2 | 2 |
| Bash guard: `cd . && ls` | 0 | 0 |
| Bash guard: `ls && cd .andromeda` | 2 | 2 |
| Bash guard: `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard: `src/x.rs` | 0 | 0 |
| write guard: `target\x.rs` | 2 | 2 |
| write guard: `C:\p\target\x.rs` | 2 | 2 |

Hook smoke ✓.

## Summary
13 ✓ / 1 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓ → **commit**.
