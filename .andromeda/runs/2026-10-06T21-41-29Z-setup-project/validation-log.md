# Validation log — 2026-10-06T21-41-29Z-setup-project (re-run / upgrade U02)

Operator word at the Phase 7 card: **yes** (2026-10-06).

## Pre-flight
`pre-flight · ✓ · CLAUDE.md at ./CLAUDE.md` (`health v1.0 · 3c0f4685`; the tool's trail: `health-no-marker.json`).

## Checks 1–14

| Check | Status | Diagnostic |
|---|---|---|
| 1 CLAUDE.md size | ⚠ | 154/200 lines · T1 28.8 KB · 15 of 18 bullets over 600 B (+19.2 KB) — pre-existing; the operator promotes, setup does not |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one import, `@.claude/session-handoff.md`, exists |
| 4 rule files | ⚠ | 8 files · frontmatter 6/6 parsed · always-loaded 2 (97.6 KB) · security.md 90.2 KB, testing.md 183.1 KB, observability.md 62.8 KB past the read cap — pre-existing |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | architecture.md 2026-10-05 18:09 ≤ CLAUDE.md 2026-10-05 18:13 (local) |
| 7 session-handoff (hand) | ✓ | present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root .gitignore (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | `schema_version: 3` · `last_wrap` · `tree_db_refreshed_at` · `session_count` only |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 25 entries (threshold 5) |

## Upgrade re-detect (Phase 7.5 step 2)
`upgrade v1.5 · eeed2076` — the one row this run wrote:
`U02 · ok-uncommitted · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin pr…` → ✓.
No U11 / U12 apply (both `ok` at P0). Hand / noted rows are the card's: U35 `behind` (handed), U04 · U09 · U10 · U36 `noted`.

## Write check (this run's own, before the card)
The three hook entries in `.claude/settings.json` compared with `references/hooks-matrix.md` as parsed JSON: the write
guard and the Bash guard each equal their matrix fence (command bytes 388 / 3858); the PostToolUse rustfmt row equals
the matrix prologue; `env` and the top-level key set equal the backup's.

## Hook smoke (formatter · Bash guard · write guard · jq)
Each stored command piped its stdin JSON from the Bash tool's own shell (`jq` at `/usr/bin/jq`; `CLAUDE_PROJECT_DIR`
unset in that shell, so the guard fell back to the shell's directory, the project root).

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
12 ✓ / 2 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓ → **commit**.
