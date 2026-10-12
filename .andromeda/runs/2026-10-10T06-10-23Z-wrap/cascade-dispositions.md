# Cascade dispositions — 2026-10-10-boot-smoke-s-self-end-closed

Written after `cascade.py sweep` ran over every body edit of this pass (the patterns file beside this one; the trail
`cascade-2026-10-10-boot-smoke-s-self-end-closed.json`). Copied from the listing: **total (15 patterns) · 26 rows
over 7 files** · per class **new 4/3 · standing 13/5 · leaf 9/2 · curation 0/0 · base 0/0**. Baseline `95d17008`.

## What was looked for

The old wording of each amended passage and the retired mechanism's own phrasings, over the seven masters and every
file under `.andromeda/registries/`, the three curation homes, the two judgment bases and the leaf bodies:

- how a series boot fails as a self-end (`settle-ended`: "settle verdict read(s) `ended`");
- the measured limit and its mechanism (`limit-sentence`, `counted-other`, `no-label`, `takes-no-settle`,
  `under-reads`);
- the open status and its owner (`not-closed`, `closing-entry`, `not-claimed`);
- the falsified mechanism (`read-failed`, `errno-11`, `runner-build`);
- the series' pin count (`sixteen-pins`), the witness reader's sole caller (`sole-reader`), ordinal 1's one source
  (`ordinal-one`).

Each pattern's control fired on the pre-pass masters. Not looked for by the tool: "reads red on most runs" (0 hits
in the pre-pass masters and registries, so no control; searched by hand over `CLAUDE.md`, `.claude/docs`,
`.claude/rules`, the seven masters and the registries after the leaf edits: 0 files). Not swept: the route and the
handoff (P5 and P6 own them), the chunk folder, the sidecars.

## Zero-row patterns (control fired, 0 rows after the pass)

`not-closed` · `closing-entry` · `sole-reader` · `ordinal-one` · `not-claimed` · `runner-build`: each retired
wording is gone from the masters, the registries and the leaves.

## Rows, each read

Master and registry rows (17: new 4, standing 13):

- `architecture.md:259` `settle-ended` new @c7475 — this pass's own both-ways wording of the exit-code clause →
  amended, no further change.
- `test-plan.md:496` `settle-ended` standing edited @c1687 — "its settle verdict reads `ended`, or …": the first of
  the two ways → amended.
- `test-plan.md:509` `settle-ended` new — the build-failure bullet's both-ways parenthesis → amended.
- `obs-plan.md:545` `settle-ended` standing edited ×2 @c4010, 4342 — each read by window: the first is this pass's
  both-ways parenthesis ("a series boot that ended by itself fails the job: its settle verdict reads `ended`, or
  …"); the second is the dated reading of a failed smoke ("as measured on `ci#38010977166`, the settle verdict read
  `ended`"), true as it stands → amended · no change.
- `registries/contracts/architecture/ci-cd-approach.md:3` `settle-ended` standing edited @c1256 — the both-ways
  clause → amended.
- `architecture.md:259` `counted-other` standing edited @c9555 and `no-label` @c9578 — "Until chunk … was counted
  `other` with a null `exit_witness` (as measured on `ci#38019133294` …)": the dated reading before the change, kept
  on purpose → amended, no further change.
- `test-plan.md:134` `counted-other` @c4395, `no-label` @c4444, `under-reads` ×2 @c4283, 4706, `sixteen-pins`
  @c4009 — the row's earlier widening clause, put in the past tense ("carried 16 unit pins … at that chunk", "its
  verdict under-read", "was counted `other`"), and the Narrowed clause's "no longer under-reads" → amended, no
  further change.
- `obs-plan.md:396` `read-failed` new @c2120 ("no read failed on the X connection") and `errno-11` new @c1181 ("each
  records `errno` 11, a stale value") → this pass's corrections.
- `registries/contracts/test-plan/5-command-implementation.md:5` `read-failed` standing edited @c2460 ("No read
  failed") and `errno-11` ×2 @c2009, 2257 — the first is the dated reading of `ci#38019133294`'s witness lines
  ("reads `_exit(1)`, `errno` 11, on the main thread"), which is what those lines hold; the second is this pass's
  "The `errno` 11 those lines record is a stale value" → no change · amended.
- `obs-plan.md:266` `no-label` standing — "Counts only, no labels": the fingerprint-feed counters' cardinality
  rule, another subject sharing the words → no change.
- `test-plan.md:496` `takes-no-settle` standing edited @c1830 — "Such a boot takes no settle verdict (its `verdict`,
  `app_exit_record` and `windows_settled` stay null)": true as it stands, the series reads the two other members
  → amended.

Leaf rows (9 rows over 2 files), both leaves re-derived from the amended masters:

- `.claude/rules/verification-harness.md:85` (`settle-ended`, `limit-sentence`, `counted-other`, `no-label`,
  `takes-no-settle`) — the `harness:boot-series` row of the xtask drift checks → re-derived: both ways to
  `self-ended`, the before-ready read, ordinal 1, the dated reading before the change, the never-on-a-runner limit.
- `.claude/docs/tests-summary.md:88` (`settle-ended`, `limit-sentence`, `counted-other`, `no-label`) — the CI `boot`
  job bullet → re-derived: both ways, the closed cause with the three attempts, the series as a gating step whose
  self-end is a new reading, the limit, the `libX11.so.6` host need. Its sentence "Until the cause of the self-end
  is closed that job reads red on most runs" is gone.

## Leaves re-derived beyond the rows (by provenance, cascade step 3)

- `CLAUDE.md` — the `pulse-app` module line (the second statement of `main()`); the path-variables warning's
  exit-witness clause (the series as a second reader, the same PROVISIONAL item). 154 lines.
- `.claude/docs/stack.md` — a new bullet for the Xlib thread initialisation (architecture §Stack and Technologies).
- `.claude/docs/commands.md` — the `harness:boot-series` line (the before-ready count, ordinal 1).
- `.claude/docs/security-summary.md` — the exit-witness arm (the second reader, the same PROVISIONAL item).
- `.claude/rules/security.md` — the path-variables rule's exit-witness arm (the same).
- `.claude/rules/observability.md` — the perf-budget bullet (the both-steps-pass reading, `ci#38026637514`).
- `.claude/rules/testing.md` — `pulse-app/tests/*.rs` 102 → 103 files, dated 2026-10-10.
- Read and left: `.claude/docs/obs-summary.md` (it carries no line on the CI self-end or its owner; its
  process-end leaf paragraph is the `app.exit` leaf, unchanged) · `.claude/docs/gotchas.md`, `workflow.md` (0 hits
  for the process-end narrative or a red boot job).

## The binds

- test-plan §3 ↔ obs-plan §3: no harness verb, status shape or log format changed; the `boot` key's Readiness signal
  (test-plan) and §7 Process-end cause (obs-plan) now state the same closed cause with the same readings.
- a11y schema ↔ obs schema: untouched.
- Curation homes: 0 rows. Judgment bases: 0 rows.
