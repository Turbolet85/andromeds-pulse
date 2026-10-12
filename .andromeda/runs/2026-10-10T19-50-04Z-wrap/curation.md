# Curation — 2026-10-10-engine-harness-proofs-completed

Scope: this window's conversation and the report's "Decisions & corrections". The wrap's first window (implement, the
operator pass, P1) was cleared before curation ran; what it held reaches curation only through the report.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A new test binary that spawns processes or takes ports is run repeatedly, together with its siblings, before its first green is trusted" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Corrections (in place, exempt from the cap):
    testing.md Session Additions, the 2026-10-10 quarantine SWEEP HAZARD entry: four search dirs → five
    verification-harness.md Session Additions, the 2026-06-29 log-family entry: the trigger it names has landed
  Extended: T2/observability.md: "COMPLEMENT to the 2026-05-07 `AllowList::for_target()` resolver-order entry" + "its two line numbers are of pulse-app/src/observability.rs as it stood on 2026-08-26"
  Filters: 2 dup · 3 task-specific or one-off · 0 conflict · 0 deferred · 4 rejected at exactly 0.6 or below (a master owns the fact)
  No-other-home: "A new test binary that spawns processes or takes ports is run repeatedly, together with its siblings, before its first green is trusted"
```

## Applied

- **T2 testing.md, new entry (2026-10-10).** Signals: verified by measurement, proven by a real failure (+0.4);
  specific technical detail with context (+0.2); reached no other durable home (+0.2: the masters carry the port-claim
  remedy and the one-in-seven count, not the directive to repeat a new concurrent suite before trusting it) = 0.8.
  Proof: report, Decisions & corrections — "The first firing of the gate block had already read green on the tree
  that carried the racy picker; the failure was found by running the port-using binaries repeatedly by hand, not by a
  listed gate", and "One of seven hand runs failed one arm … the same arm alone passed 60 of 60. After the claims: 14
  of 14 hand runs".
- **Correction, testing.md Session Additions (the quarantine sweep hazard).**
  Proof: report, Counts — "quarantine search dirs: 4 → 5 (both gain `xtask/tests`)"; the cascade sweep row
  `.claude/rules/testing.md:316 lint-roots curation`.
- **Correction, verification-harness.md Session Additions (the log-family trigger).**
  Proof: report, Outcome, The four pending rows — "`harness-log-family-resolution-coverage` — closed (three `logs`
  pins, one on a real engine's family)"; the cascade sweep row `.claude/rules/verification-harness.md:127`.
- **Extension, observability.md Session Additions (the 2026-08-21 entry).**
  Proof: the citation sweep's two `out-of-range` rows on line 149 (`citation-dispositions.md`); the tool binds the two
  naked numbers by position to `pulse-app/src/heartbeat.rs`, 435 lines; the registry the entry measures,
  `by_target.insert(`, stands in `pulse-app/src/observability.rs` (216 call sites, 2947 lines at this wrap). The two
  rows will print at every sweep while the digits stand: the entry is preserved verbatim, and only the reading is added.

## Rejected, with the filter

- An accepted loopback connection does not say whose listener accepted it — Filter 4, 0.6: amended into three
  masters and re-derived into the harness rule's body this wrap.
- A port bound to 0, read and released is handed to another test — Filter 4, 0.6: amended into test-plan §9 and
  architecture §Network ports.
- The cycle verb and the script resolve the engine under the compile-time workspace root's `target/release` —
  Filter 4, 0.6: test-plan §9's `lint-test` row carries it.
- A pin whose fixture fails for another reason stays green under a mutation of its own arm — Filter 4, 0.6: carried in
  test-plan §1 (the engine-log row's limits) and brought to the route card as found and not owned.
- A backgrounded `gate.py run` prints nothing until the block ends — Filter 1, duplicate of the 2026-10-10 harness
  entry on reading a verdict from a gate log by its summary.
- A job log holds each nextest `PASS` line twice — Filter 1, duplicate facet of the same entry and of testing.md's
  2026-10-10 job-log sweep hazard.
- The per-write `rustfmt` hook reflowed call sites a script had re-pointed — Filter 4: one-off, no emphasis.
- A count taken from a neighbouring count (nine `xvfb.log` files inferred from nine ready lines; eight by `find`) —
  Filter 2 and Filter 1: task-specific, and the 2026-08-21 Tier-1 entry on re-deriving a relayed measurement covers
  the class.
- The runner's boot record carries the pull-request merge commit, not the pushed tip — Filter 4: 0.2, one reading.
- Snap the operator's word before the pre-CI commit — Filter 4: 0.2, one event; the handoff's host note on the
  window between a pre-CI commit and its push covers the neighbouring hazard.

## Guard refusals — not a recurrence

The report records the Bash guard refusing a leading `cd` twice and a `cat` heredoc with a file target once in the
first window, each re-issued once in a form the guard passes. The guard held the rule each time, so neither is logged
as a new recurrence; the two `recurrence-despite-learning` lines the handoff already carries from earlier wraps stand
as they are.
