# Curation log — wrap of 2026-10-07-l4-probe-reproduces-the-canary-history-miss

Source: this wrap's conversation and the report's Decisions & corrections. The conversation of the last implement
run and of the operator pass was cleared before P2 (the operator's stop after P1); what it held beyond the report is
not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  no new entry · 15 bullets re-tiered (below)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "The corpus fingerprint arm is fed by the whole window, and no prompt shows its set"
                                              + "`gh run rerun --job` waits for the whole run to conclude"
  Filters: 1 dup (a recurrence, → handoff) · 2 task-specific · 0 conflict · 0 deferred · 2 below threshold
  Load-bearing: "`gh run rerun --job` waits for the whole run to conclude" → The Linux boot smoke is deterministic
  No-other-home: "The corpus fingerprint arm is fed by the whole window, and no prompt shows its set"
  Extended: T3/session-learnings.md: "A digest a live run archived is recoverable only with the corpus key" + "a line one word away from a captured line is still the capture's text"
  CLAUDE.md size: 154/200 · T1 6.8 KB, 0 over 600 B
```

## Applied

- **T3 · The corpus fingerprint arm is fed by the whole window, and no prompt shows its set.** Score 0.8: verified by
  measurement +0.4, specific technical detail +0.2, reached no other durable home +0.2 (the other signals total
  exactly 0.6; the next-entry signal did not fire).
  Proof: the report's Decisions & corrections, "A narrow basis, re-derived before it was relied on":
  `evidence/preregistration-third.md` read "the fingerprint arm agrees" from the one fingerprint a prompt cites; the
  Q3 query groups `span_events` exceptions by fingerprint, `HAVING COUNT(*) >= 2`, `LIMIT 10`, over all services
  (`crates/triage/src/baseline/sql.rs`, re-read at this wrap). The stale comment: the report's "Spec claims disproved
  by measurement", last item; `crates/triage/src/digest/retrieval.rs`, the doc paragraph opening "The fingerprint arm
  is correct by contract and currently STARVED" (re-read at this wrap).
- **T3 · `gh run rerun --job` waits for the whole run to conclude.** Score 0.8: proven by a real gate failure +0.4,
  specific technical detail +0.2, load-bearing for the next entry +0.2 (the first markerless entry's work is CI
  re-runs of the boot smoke).
  Proof: `evidence/operator-pass.md`: `ci#37668429742` attempt 1, the boot smoke failed while the coverage job sat
  78 min 26 s in a package install step; the failed job could not be re-run until the run was cancelled; attempt 2 of
  the same run id read 13 of 13.
- **Extended · T3 "A digest a live run archived is recoverable only with the corpus key".** The facet: a line one word
  away from a captured line is the capture's text; committed files carry derived facts only. Score 0.9: explicit user
  correction +0.4, "never" language +0.3, specific technical detail +0.2.
  Proof: the report's Decisions & corrections, "Operator's ruling on capture text" (inputs#I23); `S17`'s titles were
  reworded before its reading on that ruling.

## The Tier-1 re-tier (a directed restructure, not a learning; outside the max-3 cap)

Direction: the founder's pick, 2026-10-07 12:14 local, by dialog ("move the long lessons now"), relayed by the pc
overseer (the wrap relay §5). Fifteen `USER:session-learnings` bullets ran over 600 B (health check 1 before:
`T1 29.1 KB · 15 of 18 bullets over 600 B (+19.2 KB)`). Each now keeps a one-sentence lead with a pointer; its full
text stands verbatim in `.claude/docs/session-learnings.md` under a heading `Moved from Tier 1: {title}`. All fifteen
went to Tier 3: none is path-scoped (process and cross-crate lessons), and `rules/security.md` has no `paths:`, so
routing there would keep the text always loaded.
Proof: the move script asserted, per bullet, one matching line inside the block, a lead of at most 600 B, and after
the write that every replaced bullet's text is a byte-exact substring of the Tier-3 file; CLAUDE.md 57,864 B →
35,033 B, 15 lines changed, line count unchanged; both files `i/lf w/lf`.

| original bytes | lead bytes | Tier-3 title |
|---|---|---|
| 810 | 380 | the after-MVP evolution path |
| 848 | 358 | migration helpers preserve the source on failure |
| 1311 | 369 | bincode 1.3 deserialize on untrusted input |
| 1157 | 369 | scrubbed_clone with an injected closure |
| 1291 | 418 | bindings vs subprocess vs persistent server |
| 1951 | 380 | the hybrid render pattern |
| 2094 | 404 | verify the data producer exists |
| 1619 | 407 | orphan-graph mapping before a cleanup chunk |
| 1820 | 341 | the planned add that already exists |
| 1756 | 417 | research corrects intent |
| 5133 | 400 | external-relay measurements are verified at HEAD |
| 2848 | 365 | verify the consumer or render site exists |
| 1765 | 425 | durable text carries the caveat |
| 1504 | 352 | a disposition is not inherited |
| 2726 | 417 | a falsified premise has a writer or a report |

## Filtered

- task-specific: the capture-text guard greps whole rendered lines and misses a reworded line (the guard and the
  captures end with this chunk).
- task-specific: the mutation checks were run by a script (anchor matched once, run, restore, hashes compared), a
  one-off method of this chunk's 76 checks.
- below threshold (exactly 0.6, already amended into a master): a remedy that changes what `shipped` composes
  reddens every pin of a measured block at once, and the pins move to the baseline arm. It stands in test-plan §1
  and its sidecar entry.
- dup, a recurrence: the Bash guard refuses a leading `cd` out of the project. The host leaf's Session Additions
  entry of 2026-10-05 already states it; logged in the handoff as `recurrence-despite-learning`.
- below threshold: the Bash guard refuses a `cat` heredoc with a file target; the permission classifier returned no
  verdict three times once (a one-off mention). Host lessons go to Tier 3 by the relay's direction, and none passed
  the threshold.
