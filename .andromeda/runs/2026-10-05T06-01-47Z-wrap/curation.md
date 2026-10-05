# Curation — 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-win32.md: "On the Linux host the Bash tool's working directory persists across calls and the cwd guard blocks only a LEADING `cd`, so a `cd` inside a loop or a compound command silently moves the session cwd for every later call — use absolute paths or a `( cd DIR && … )` subshell there."
    Proof: at this wrap's P2 fan-out prep, a `for d in …; do …; done` read preceded by `cd {run_dir} 2>/dev/null;` moved the session cwd to `.andromeda/runs/2026-10-05T06-01-47Z-wrap` (the harness's "Primary working directory" notice); the PreToolUse guard (`case "$c" in cd|cd[[:blank:]]*`) only tests the command's FIRST token. Routed by tiebreaker 2's exception: host-win32.md has no `paths:` frontmatter, so it is judged at Tier 1's one-sentence bar (≈ 300 B). Dedup: the generated body's "The working directory MAY persist … rely on neither" bullet is the nearest match; this candidate's facet (the guard's leading-only scope, the silent move) is absent from it, and a generated-body match takes a new Session Additions entry, never an in-place amend. Confidence 0.8 (measured +0.4 · specific detail +0.2 · no other durable home +0.2).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold
    - rejected at 0.6 (measured +0.4 · detail +0.2; no conditional signal): "a candidate chosen as the best of several single-run arms at n = 10 regresses on fresh confirmation (selection slot 37/40 → confirmation 34/40 on the same text)". Its durable home is the route: it rides the model-replacement entry's CONTEXT minted at P5 (that entry's selection series faces the same carry-over).
  No-other-home: "the Bash cwd guard blocks only a leading cd" (the host-win32.md entry above)
  CLAUDE.md size: see P7 health check 1
