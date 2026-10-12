# Validation log — setup-project re-run (upgrade form), 2026-10-10T01-50-13Z

## Pre-flight
`pre-flight · ✓ · CLAUDE.md at ./CLAUDE.md` (`health v1.0 · 1358832b`; trail `health-no-marker.json`).

## Checks
| Check | Status | Diagnostic |
|---|---|---|
| 1 CLAUDE.md size | ✓ | 154/200 lines · T1 6.8 KB · 0 of 18 bullets over 600 B |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one import line, `CLAUDE.md:104` `@.claude/session-handoff.md`; the file exists |
| 4 rule files | ⚠ | frontmatter 6/6 parsed · always-loaded 2 (96.9 KB: `security.md` 94.1 KB, `host-linux.md` 2.8 KB) · past the read cap: `observability.md` 64.1 KB · `security.md` 94.1 KB · `testing.md` 183.8 KB |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` mtime 2026-10-10 03:32:28 +0200, `CLAUDE.md` 03:36:23 +0200 — arch is the older |
| 7 session-handoff (hand) | ✓ | present, non-empty (9279 B) |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master-route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 (py read 27 before this run's write) |
| 12 state.yaml (hand) | ✓ | `schema_version: 3` and the three lean fields, four top-level keys, nothing else |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 25 entries (threshold 5) |
| Hook smoke | ✓ | formatter: exit 0 and the mis-formatted `.setup-validation-test.rs` was rewritten (temp file removed) · Bash guard: heredoc-to-file 2 · python heredoc 0 · `cd .andromeda && ls` 2 · `cd . && ls` 0 · `ls && cd .andromeda` 2 · `ls; ( cd .andromeda && ls )` 0 · write guard: `src/x.rs` 0 · `target\x.rs` 2 · `C:\p\target\x.rs` 2 · `jq` at `/usr/bin/jq` |
| Upgrade re-detect | ✓ | U03 `ok` after the write; summary `for setup 0 · awaiting a door 0 · noted 3 (U09, U10, U36) · INDETERMINATE 0` |

One probe reading corrected inside this run: the first pass over the six Bash-guard arms printed `exit 0` for all
six. The probe read `$?` after a command substitution on the same `echo` line, so it reported that substitution's
exit, not the guard's. Re-run with the exit code captured first, the six arms read 2 · 0 · 2 · 0 · 2 · 0 as specified;
the guard run directly printed both `Blocked:` messages with exit 2.

## Summary
13 ✓ / 1 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓

## Decision
Pre-flight passes and there is no ✗ → commit. The one ⚠ (check 4, rule-file sizes) is not this run's to remediate:
superseded Session Additions entries belong in Tier 3, and that curation is wrap's.
