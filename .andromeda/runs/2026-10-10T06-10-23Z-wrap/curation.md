CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "A runner-only failure that follows the version of a system library is reproduced on the dev host by swapping that library in for one command (the runner image's package by URL, checksum-refused, unpacked under target/, named through LD_LIBRARY_PATH on the leg's command alone; ldd under the variable as the known positive, without it as the undo)" (confidence 0.8)
    Proof: the close leg (`harness:boot-series --count 8` under `LD_LIBRARY_PATH` naming `target/jammy-x11/lib`) read `self-ended`, 7 of 8, on the untouched tree (phase P5 baseline) and `all-settled`, 8 of 8, on the built change (/implement gate entry 11); `ldd` under the variable read 2 (entry 10) and 0 without it (entry 14). Record: `chunks/2026-10-10-boot-smoke-s-self-end-closed/evidence/swapped-library.md`. Signals: verified by measurement, a real gate red (+0.4) · specific technical detail (+0.2) · no other durable home (+0.2): the how lives in the chunk's evidence file alone; architecture and test-plan state the result, not the recipe.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred
  No-other-home: "A runner-only failure that follows the version of a system library is reproduced on the dev host by swapping that library in for one command"
  CLAUDE.md size: read at P7 from `health.py check`

Seven candidates read from this session and the report's Decisions & corrections:

1. The library-swap recipe — applied (above).
2. "A new reader or outlet of a PROVISIONAL harness record joins the same provisional item, not a second one" (the operator's word at the escalation card) — an additive facet of `rules/security.md` Session Additions 2026-10-10. Explicit ruling +0.4 · specific detail +0.2 = 0.6, exactly the threshold; the no-other-home signal does not apply (this wrap amended it into security-plan, three sites, and the rule file's body re-derives it). Rejected at the threshold; its home is the master.
3. "A test that loads a host library adds a host need to the workspace suite; state it where the suite's host needs are listed" — specific detail +0.2, one event. Rejected below the threshold; its home is test-plan §4 and the `lint-test` row, amended this wrap.
4. "Between the pre-CI commit and the push, a listed entry run through the gate tool with the run dir rewrites a committed trail and trips the push's clean-tree guard" — specific detail +0.2, could be task-specific −0.2, not measured (the trip was avoided, never observed). Rejected (task-specific).
5. "`git diff --quiet` does not see untracked files" — specific detail +0.2, one-off −0.3. Rejected (task-specific).
6. A guard refused a `cat` heredoc with a file target (one probe during the operator pass) — already curated (`rules/host-linux.md` → Transports). The guard refused the act and one re-issue through the Edit tool landed it: the guard held the rule, so this is not a new recurrence. The handoff's open `recurrence-despite-learning: host-linux.md, Transports` from prior wraps stays as it is.
7. The cwd guard refused a leading `cd` out of the project root (once, at this wrap's P1) — already curated (the host leaf's Session Additions 2026-10-05). Refused and re-issued once as a script file: the guard held the rule, not a new recurrence. The handoff's open entry from prior wraps stays as it is.

Citation rows routed here by the sweep (`citation-dispositions.md`), surfaced and not corrected, as at the last three wraps: `rules/observability.md:148` twice (bare numbers of another file, in a `## Session Additions` entry) and `docs/session-learnings.md:865` and `:1829` (historical coordinates of `pulse-app/src/main.rs`, stale before this chunk).
