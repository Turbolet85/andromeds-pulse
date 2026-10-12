# Curation log — 2026-10-09T15-11-06Z wrap of 2026-10-09-ci-on-linux-alone

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A thing found standing gets an owner at route-resolve; a mention in the report is not one"
                                              + "A verbatim relay kept in a run dir can fail the hygiene read at the pre-CI commit"
  Filters: 1 dup · 0 task-specific · 0 conflict · 0 deferred · 2 below the confidence threshold
```

## Applied

- Tier 3: "A thing found standing gets an owner at route-resolve; a mention in the report is not one" (confidence 0.7:
  an explicit operator correction +0.4, convention language +0.3).
  Proof: the operator's wrap directive, inputs#I3 item 3 ("present each for disposition, pre-place none; 'carried to
  the report as found' is not an owner"); plan.md's Implementation notes had written three such findings as "carried
  to the wrap's report as found". At this wrap's Validate four proposals on those findings were handed to route-resolve
  for lack of a named owner (`fanout-results.md`, O1 (b) to O4).
- Tier 3: "A verbatim relay kept in a run dir can fail the hygiene read at the pre-CI commit" (confidence 0.8: an
  explicit operator direction +0.4, proven by a real gate refusal +0.4, a specific technical detail +0.2, could be
  task-specific −0.2).
  Proof: the hygiene preview at the end of /implement printed `hygiene: refused 1 files — P1 1`, its row
  `P1 .andromeda/runs/2026-10-09T14-05-04Z-phase/relay-1.md:1 ×1 · home`; after the hand respell on the operator's
  word, entry 17 read `hygiene: clean` (`chunks/2026-10-09-ci-on-linux-alone/evidence/operator-pass.md`).

## Filtered

- duplicate: "one CI run is one reading — record it as measured, state no cause, re-run nothing" (the operator's word
  on the a11y job, inputs#I3 item 2). Already carried by CLAUDE.md's 2026-08-23 learning (a small-n result is never
  written as an exclusion) and by test-plan §10's zero-flakiness rule; no new facet.
- below threshold (0.3): "`git diff --quiet` cannot compare against an uncommitted intermediate; take a copy or a hash
  before a working-tree mutation" (measured +0.4, technical detail +0.2, a one-off −0.3).
- below threshold (0.2): "the masters say 'lint-test Linux' for the perf-budget gate's host, so a grep for other-system
  wording that includes `Linux` over-matches" (a sweep hazard, technical detail +0.2).
- No candidate landed exactly on 0.6.

## Not candidates

- The session's guard refusal and the mid-session change of a skill reference are telemetry of the tool (the friction
  ledger's), not session learnings.
- The operator's word to run the operator pass, the pre-CI commit and the push included, was given for this pass; it
  is not written as a standing permission.

## Rows the two sweeps routed here, and what curation did with them

- The citation sweep's 17 rows in preserve-verbatim homes (`citation-dispositions.md`, "curation home — P3"): 6 in
  rule files' `## Session Additions`, on 5 citing lines (`frontend.md:99`, `observability.md:134`, `:148` ×2,
  `security.md:159`, `testing.md:159`), and 11 in `docs/session-learnings.md`. Not corrected: each is a line
  number inside a dated learning, the rule each entry states does not turn on the number, and a blame-mapped number
  is not proof (3 of the 6 master rows mapped that way were refused on reading). They are surfaced in the handoff;
  correcting them is a read per row.
- The cascade sweep's curation rows: `testing.md:155`, `:206` and `session-learnings.md:231` state nothing about the
  CI's runner systems as current; `session-learnings.md:2461`, `:2567`, `:2589`, `:2591` are 2026-05-03 learnings
  that name the CI matrix runners of that date as context for a Windows dev-host toolchain. Left as dated: none is a
  rule this chunk's measurement makes false.
