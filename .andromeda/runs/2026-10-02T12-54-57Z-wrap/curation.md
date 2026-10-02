# Curation — 2026-10-01-conductor-return

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + observability.md: "[correction] the app writes no stderr log stream — the 'Dual sink (app)' and 'stderr + file appender' generated claims are false at HEAD; read the agent-latest.jsonl* family" (correction — cap-exempt)
    Proof: `pulse-app/src/observability.rs::init` builds one `fmt` layer with `.with_writer(non_blocking)` and contains 0 `stderr` occurrences (grep this wrap); research.md §Files inspected (the chunk's own read, `observability.rs:2637-2641` at a2addb3); the plan routed this correction to the wrap's curation channel. Source fix owed in obs-plan §1/§3 step (2) — route CARRY.
  Tier 3 (.claude/docs/session-learnings.md): + "A numeric count grep over the specs matches every Ed25519" (confidence 0.8)
    Proof: `grep -rln '2551\|2,551\|2 551' .andromeda/*.md CLAUDE.md .claude/docs .claude/rules` → 9 files, every hit `Ed25519` (report.md Counts / qualifiers moved). Signals: verified by measurement +0.4 · specific technical detail +0.2 · no other durable home this wrap +0.2.
  Filters: 3 dup (the once-flag-by-return-value pin, the atexit no-log-on-exiting-thread ban, Windows process::exit = ExitProcess — each amended into a master this wrap) · 0 task-specific · 0 conflict · 0 deferred (cap)
  Below threshold: the re-exec `--no-capture` child-ran facet (0.6 — test-plan §3 now carries the in-arm proof); the flycheck-respawns-after-every-save facet (0.5 — the gate-time attribution was not measured)
  No-other-home: "A numeric count grep over the specs matches every Ed25519"
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B
Recurrence (→ handoff Deferred learnings): the targeted-nextest-timeout remedy from the prior wrap WAS applied (timeout = 3600) and the cold run still measured 3564.77 s — undersized under host contention; operator raised it to 5400 at this wrap.
