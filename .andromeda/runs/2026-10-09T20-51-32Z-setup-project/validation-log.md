# Validation log — setup re-run, upgrade form (2026-10-09T20-51-32Z)

Stopped before Phase 9 on the operator's word: nothing is staged, nothing is committed.

## Pre-flight
`pre-flight · ✓ · CLAUDE.md at ./CLAUDE.md` (`health v1.0 · 1358832b`).

## Checks
| Check | Status | Diagnostic |
|---|---|---|
| 1 CLAUDE.md size | ✓ | 154/200 lines · T1 6.8 KB · 0 of 18 bullets over 600 B |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports valid (by hand) | ✓ | one import line, `CLAUDE.md:104` `@.claude/session-handoff.md`; the file exists |
| 4 rule frontmatter | ⚠ | 8 rule files · frontmatter 6/6 parsed · always-loaded 2 (95.9 KB); `observability.md` 63.9 KB, `security.md` 93.0 KB and `testing.md` 183.8 KB are past the read cap. Not this run's: `host-linux.md` reads 2.8 KB |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (by hand) | ✓ | `architecture.md` mtime 1791576360 ≤ `CLAUDE.md` mtime 1791576649 |
| 7 session-handoff.md (by hand) | ✓ | present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root .gitignore (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 · summaries 5/5 |
| 10 master-route | ✓ | present |
| 11 code-graph pipeline + seeds | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (by hand) | ✓ | parses · `schema_version: 3` · fields `last_wrap`, `tree_db_refreshed_at`, `session_count` only |
| 13 agent harness | ✓ | harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 25 entries |
| Hook smoke | ✓ | 10 arms as wanted, each command read from `.claude/settings.json` and run as stored with its input on stdin |
| Upgrade re-detect | ✓ | U02 `ok-uncommitted` (`write current · bash current`); U04 `ok`; 0 `INDETERMINATE` |

Summary: 13 ✓ / 1 ⚠ / 0 – / 0 ✗, plus the hook smoke ✓ and the re-detect ✓.

## Hook smoke, per arm
- Bash guard: a `cat` heredoc with a file target exit 2 · a python heredoc exit 0 · `cd .andromeda && ls` exit 2 ·
  `cd . && ls` exit 0 · `ls && cd .andromeda` exit 2 · `ls; ( cd .andromeda && ls )` exit 0.
- Write guard: `src/x.rs` exit 0 · `target\x.rs` exit 2 · `C:\p\target\x.rs` exit 2.
- Formatter: the PostToolUse rustfmt row, exit 0, the mis-formatted file changed. Run with the temp file in the
  session scratchpad instead of the project root, so nothing landed in the tree; `rustfmt.toml` was therefore not
  in reach of that one run, which reads the input path and not the edition.

## The writes
- `.claude/settings.json`: lines 14 and 24 (the two PreToolUse `command` strings); every other line identical to
  the backup by `diff`.
- `.claude/rules/host-linux.md`: three lines removed (the backslash-pair bullet under `## Encoding & heredocs`);
  the line below `## Session Additions` byte-identical, read back by the tool.

## Decision
Pre-flight pass, zero ✗: the run is commit-ready. The commit is the operator's: Phase 9 was not entered.
