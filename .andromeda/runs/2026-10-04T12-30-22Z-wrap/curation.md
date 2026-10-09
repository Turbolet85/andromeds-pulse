# Curation — 2026-10-04-linux-launch-stays-up-on-nvidia-wayland

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): extended (1 write)
  Extended: T3/session-learnings.md: "`grep` on the Linux dev host is ugrep, and a bounded-context pattern dies silently" + "`grep -c` on a binary file prints nothing — count byte strings with python"
    Proof: /implement P2, the post-mutation binary check — `grep -c 'app.boot.render.posture' target/debug/pulse-app` printed no line at all, while `python3 -c "…read().count(b'app.boot.render.posture')…"` over the same file printed `2` (and `1` for the lever name). Score 0.8 = measured +0.4 · specific detail +0.2 · no other durable home +0.2 (not on the route, not in a master, no playbook/matrix home; the report's Decisions bullet is not a durable home).
  No-other-home: "`grep -c` on a binary file prints nothing (ugrep)"
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred
    - dup: "edition-2024 `set_var` is sound only before the tokio runtime build — set launch env as `main()`'s first statement, emit its record after the sink exists" → carried by this wrap's master amendments (arch §Occupied Resources, CLAUDE.md `pulse-app` line re-derived) — a master home
    - dup: "honour a preset env value by presence only, never parse it" → carried by the arch / security-plan bodies amended this wrap
    - task-specific: "a workspace `cargo nextest` during a mutation check leaves the debug binary built from mutated source; rebuild before a live leg" → the mutated-binary state was inferred, not measured (the later `Compiling pulse-app` line does not establish it), −0.2 paraphrased-from-implicit; below threshold
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B
