# Curation log — the 2026-10-07T06-47-12Z wrap

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none)
  Tier 2 (.claude/rules/*):                   (none)
  Tier 3 (.claude/docs/session-learnings.md): + "A digest a live run archived is recoverable only with the corpus key"
                                              ~ extended "Measure a real-model decision defect with a pre-registered,
                                                one-factor arm matrix before choosing a fix"
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred
  Extended: T3/session-learnings.md: "2026-10-01 — Measure a real-model decision defect with a pre-registered,
            one-factor arm matrix before choosing a fix" + "the baseline arm must be able to miss the case the fix
            was minted for"
  CLAUDE.md size: 154/200 · T1 29.1 KB, 15 over 600 B
```

## Applied

- **Tier 3, new — "A digest a live run archived is recoverable only with the corpus key"** (confidence 1.0:
  verified by measurement +0.4, a specific technical detail with context +0.2, named for curation by the overseer in
  the wrap relay +0.5, capped).
  Proof: `evidence/post-hoc-d3-replay.md` of chunk `2026-10-06-l4-first-hypothesis-names-the-triggering-service` —
  the fourth series' corpus holds 51 `digest_archive` rows, each cell-encrypted; `grep -rn 'digest_archive'` over
  `crates` and `pulse-app/src` returns two INSERT sites and no SELECT; the throwaway get-only key read was refused by
  the session's permission layer before it ran; row 45 was identified from plaintext timestamps alone.
  Tier: reference context over several sentences (decision-tree step 3). `security.md` carries no `paths:` and loads
  every turn, so it is judged at Tier 1's one-sentence bar, which this fails; Tier 3 whole.

- **Tier 3, extended in place — the 2026-10-01 pre-registered arm-matrix entry** (the facet's own chain: verified by
  measurement +0.4, a specific technical detail +0.2, named for curation by the overseer +0.5, capped at 1.0).
  Proof: `evidence/reading.md` of the same chunk — the baseline arm `ns` read 20 of 20 on the sibling shapes S7 + S8
  and 38 of 40 on S1–S4, so every arm met the bar and the reading could not show whether the sentence fixes the case
  the chunk was minted for; the overseer's recount of `runs.json` matched (240 rows).
  Filter 1: the matched entry already says "measure the baseline FIRST on the untouched tree"; the facet it lacks is
  that the baseline must be able to FAIL on the new shapes, and what to pre-register when it cannot. Same tier, same
  location, so the write form is the in-place `Extended 2026-10-07` sentence.

## Filtered

- **dup (a defect record → recurrence, logged to the handoff):** "`grep` on the Linux dev host is ugrep and a
  bounded-repeat window dies silently" — matched Tier 3 "2026-10-04 — `grep` on the Linux dev host is ugrep, and a
  bounded-context pattern dies silently". It recurred at this wrap's P2 (a leaf probe with stderr suppressed printed
  nothing; caught on the empty output, re-run in python: 11 lines).
- **dup (a defect record → recurrence, logged to the handoff):** "a recursive `rm` inside a compound command is
  denied; plain `rm` of named files and an `rmdir` pass" — matched the host rule's "Compound commands & permissions"
  body.
- **task-specific:** the probe's R1 identity predicate, the grader's signature and the run-loop extraction — design
  detail of one file, recorded in the report's Deviations.

## Recurrences without a curation home (logged to the handoff's Deferred learnings)

- An evidence stamp typed as an estimate (three minutes ahead of the clock) against the host memory rule "timestamps
  come from the clock"; the stamp-ahead hook refused it.
- An absence claim made on a pattern too narrow to find the hit ("no master states the pin count", while test-plan §4
  stated `128 pins`) — matched Tier 3 "2026-10-02 — A numeric count grep over the specs matches every `Ed25519`"
  (its rule: anchor the probe and read the hits, never the tally); the test-plan detector caught it.
