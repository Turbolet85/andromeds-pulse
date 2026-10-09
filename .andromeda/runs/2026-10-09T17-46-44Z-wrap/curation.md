CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a zero-is-healthy count piped from a producer that can refuse to print reads green over an empty stream" (an in-place extension)
    Proof: plan entries 40 and 41 of this chunk, driven once at the operator pass — `gh api …/actions/jobs/{id}/logs | grep -c …` printed `0` and exited 1 with the refusal on the error stream, so the presence count read red and the absence count read green by its atoms; the same log fetched with `--allow-escape-sequences` to a file read 1 and 0 (`evidence/operator-pass.md`, "Entries 40, 41 and 42"). The operator's directive asked for the form a later plan must write (inputs#I5 item 3). Signals: measured +0.4 · the operator's direction +0.4 · technical detail +0.2 = 1.0.
  Tier 3 (.claude/docs/session-learnings.md): + "A red CI read returns while the run is still open"
    Proof: entry 31 of this chunk — `ci.py conclusion --sha HEAD --wait 2700` returned red after 626 s with `run open: ci#37961031489 in_progress` and one job still running; its atom `contains pull_request completed/success` held on the secret-scan run's words; the run settled 16 minutes later, and entries 32–33 were read after a `gh run watch` (`evidence/operator-pass.md`, "Entry 31", "The host restart"). The run's `GIT_COMMIT_SHA` read `70042b8b`, not `f36a2ac`. Signals: measured +0.4 · technical detail +0.2 = 0.6 exactly, then load-bearing for the next entry +0.2 = 0.8.
  Filters: 0 dup · 2 task-specific or below the threshold · 0 conflict · 0 deferred
  Load-bearing: "A red CI read returns while the run is still open" → Boot smoke's early exit found and closed (the route's first markerless entry after this wrap's route-resolve, as `route.py markerless` prints it; its work is reading CI runs of the boot job, red ones among them. At P3, before the card, the first entry was "Pre-push check native on Linux")
  Extended: T2/testing.md: "BACKGROUND command exit code masked by `| tail`/`| head`/`| grep`" (the 2026-06-05 entry, its 2026-10-04 zero-is-healthy facet) + "a producer that refuses to print"
  CLAUDE.md size: read at P7 from `health.py check`

Rejected, with the filter:
  - "`cargo audit` is a prefix of `cargo auditable`: a fixed-string count of one counts the other" — Filter 4: technical detail +0.2 alone; it was read before the gate substring was added and changed nothing. It stands in the report's Decisions & corrections.
  - "a hotfix branch for `main` is made from the build branch's own diff, never typed twice" — Filter 4: measured +0.4 · detail +0.2 · could be task-specific −0.2 = 0.4. It stands in the plan and the report.

Not candidates:
  - What the boot smoke's log holds, and that `boot` prints ready before the OTLP bind: amended into four masters by this wrap's P2, or riding the route card.
  - The two `out-of-range` citation rows in `.claude/rules/observability.md:148` (`citation-dispositions.md`): a preserve-verbatim entry of 2026-08-21, extended 2026-08-26, quoting two line numbers of `pulse-app/src/observability.rs` as that day's measurement. Not corrected: the entry dates them, and one of the two keys they located was removed on 2026-08-30, as the same rule file says. They join the handoff's standing list of stale citations in preserve-verbatim homes.
  - `recurrence-despite-learning: host-linux.md, Transports` (a `cat` heredoc with a file target): the guard refused one such call during the operator pass and the records went through the Edit tool on the one re-issue — the guard held the rule, no recurrence is logged.
