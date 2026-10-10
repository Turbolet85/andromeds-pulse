# Curation — 2026-10-10-no-ci-step-reads-nothing

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (one candidate held for the route-resolve card — below)
  Tier 2 (.claude/rules/*):                   + testing.md: extension of the 2026-08-17 mutation-check entry
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred · 2 below threshold · 1 guard refusal that held its rule (not a recurrence)
  No-other-home: "a mutation that removes one line of a fix does not restore the defect where the fix holds the property in two places"
  Extended: T2/testing.md: "To DISCHARGE (not merely assert) an acceptance criterion … run a MUTATION CHECK" + "a one-line removal does not restore a defect a fix holds twice"
```

## Applied

- **Tier 2 · `.claude/rules/testing.md`, the 2026-08-17 mutation-check entry, extended in place** (`Extended 2026-10-10`):
  a mutation that removes one line of a fix does not restore the defect where the fix holds the property in two
  places; put the base form of the block back.
  Score: verified by measurement +0.4 (it falsified the plan's forecast for its fourth mutation) · specific technical
  detail with context +0.2 = 0.6, then the no-other-home signal +0.2 = 0.8. The fact is in no master, no route
  annotation and no ledger note.
  Proof: `andromeda-pulse-0.4.0/chunks/2026-10-10-no-ci-step-reads-nothing/evidence/mutation-checks.md`, sections 4a
  (the detector's exit line removed: 36 of 36 pass, the script still exits 1 on `ENOENT`) and 4b (the block put back
  as at `6df9e95c`: the pin red, `exit status: 0`).

## Held for the route-resolve card (the operator's wrap directive, inputs#I3 item 5b)

- **The pull-request merge is not the tip.** The local pre-push check builds the tip; the pull-request run builds
  the tip merged with `main`. Candidate rule, in the operator's wording: before a pre-CI push, show that the build
  branch contains `origin/main`, and if it does not, bring it to the operator before pushing.
  Score: an explicit operator correction +0.4 ("the red is this chunk own and its cause is the seam") · verified by a
  real gate failure +0.4 = 0.8.
  Where it would be read by an operator pass: a path-scoped rule file loads only when a matching file is touched,
  and an operator pass may touch none, so the homes that are always loaded are CLAUDE.md's `USER:session-learnings`
  (Tier 1, one sentence) and the two rule files with no `paths:` frontmatter. Its disposition is the operator's at
  the card; nothing is written before that word.
  Proof: `evidence/operator-pass.md`, "The cause: a seam between `main` and the build branch"
  (`git merge-tree --write-tree 3049966f origin/main` held the test twice, at `:377` and `:420`, the runner's lines;
  `ci#38034700885` red, `ci#38035474359` green on the equal tree with `main` an ancestor).

## Not applied

- *A leading `cd` into a subdirectory is refused by the Bash guard* — the guard held the standing host rule at the
  cost of one re-issue; not a recurrence, no entry.
- *`criterion` as a search word hits "acceptance criterion" and `wcag_criterion`* — a sweep hazard seen once; score
  0.2 (specific detail only). Below the threshold.
- *"No dead arm is kept for a plan sentence"* — the operator's acceptance of one deviation, said once; score 0.4.
  Below the threshold.
- *A pasted message is confirmed before it is acted on* — the session's own conduct, not a fact about this project.
  Task-specific.
- *An upload that fails on no file reds an already-red job a second time* — task-specific to this chunk's workflow
  and now stated in obs-plan §9 and test-plan §9.

## Surfaced, not corrected (the operator's standing word: it keeps riding the handoff)

Four citations in preserve-verbatim homes that the citation sweep listed: `.claude/rules/frontend.md:99`
(`xtask/src/main.rs:550`), `.claude/rules/security.md:160` (`xtask/src/main.rs:813`) and
`.claude/rules/observability.md:148` twice (`pulse-app/src/heartbeat.rs:1078`, `:1934`, both past the file's 424
lines). The sweep's map moves the first two by the 26 lines this chunk removed above them; read at the mapped
lines, neither shows what its sentence names, so each was stale before this chunk and a re-point would not make it
true.

## The held candidate, after the card

The operator's word: "a check that runs, not a sentence that is remembered" — the merge-base probe in test-plan §3,
on the operator leg directly before the push, and "no separate Tier-1 line unless the letters require one". The
letters do not require one, so no curation entry was written for it; the fact's home is the master and its two
leaves. Tier counts for this wrap: T1 0 · T2 1 (the extension) · T3 0.
