CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   (two in-place extensions, below)
  Tier 3 (.claude/docs/session-learnings.md): + "raising `rust-version` raises clippy's MSRV and switches lints ON"
  Filters: 1 dup · 2 task-specific · 0 conflict · 1 below threshold · 0 deferred (→ handoff: 2 recurrence-despite-learning records)
  No-other-home: "a zero-is-healthy count gate `grep … | wc -l` under pipefail reads green only while its property is false"
  No-other-home: "a hand-written evidence transcript naming a temp-dir path trips the hygiene check"
  No-other-home: "raising rust-version raises clippy's MSRV and switches MSRV-gated lints ON"
  Extended: T2/testing.md: "2026-06-05 (session 178): BACKGROUND command exit code masked by `| tail`…" [extended 2026-08-30 pipefail clause] + "an empty grep exits 1 — a `grep … | wc -l` count gate expecting exit 0 is green only when false"
  Extended: T2/testing.md: "2026-09-30: A TEST INPUT ECHOED INTO A COMMITTED EVIDENCE LOG CAN TRIP THE RUN-DIR HYGIENE CHECK" + "a hand-written transcript of a temp-dir path trips it too; re-run hygiene after any later evidence edit"
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B

## Applied
1. Extension — testing.md 2026-06-05 entry (pipefail facet). Signals: verified by measurement +0.4 · specific technical
   detail +0.2 · no-other-home +0.2 (the report records it; no route annotation, master, playbook rule or matrix note
   carries it) = 0.8.
   Proof: plan gate 8 (`grep -rn 'incompatible_msrv' … | wc -l`, `expect = ['exit 0', 'last line 0']`) read
   `red · exit 0 ✗ (exit 1)` with `last line 0` ✓ in /implement's gate run; reproduced `bash -c` → exit 0 vs
   `bash -o pipefail -c` → exit 1 (implement run `.andromeda/runs/2026-10-04T21-49-56Z-implement`, gate trail entry 8).
2. Extension — testing.md 2026-09-30 entry (hand-written temp-path transcript). Signals: measured +0.4 · detail +0.2 ·
   no-other-home +0.2 = 0.8.
   Proof: the operator pass's first `gate.py hygiene` read `refused 1 files — P1 1` on
   `evidence/residue-removal.md:7` (`tmp` predicate, 3 hits on the pasted `ls` line); after a `{tmp}` placeholder,
   `hygiene: clean` (chunk `evidence/operator-pass.md` entry 24).
3. New Tier 3 — the MSRV raise switches lints on; the floor is the dependency-graph max. Signals: measured +0.4 (it
   changed the chunk's design: 20 fix sites, a 1.95 floor instead of 1.89) · detail +0.2 · no-other-home +0.2 (the
   masters now carry the 1.95 floor but not the lint-switching consequence or the probe form) = 0.8. Tier 3 by
   tiebreaker 2 (reference, multi-sentence).
   Proof: `CLIPPY_CONF_DIR` probe at P3 (research.md §Patterns detected: 20 sites, 13 files) reproduced exactly by
   `cargo clippy … --message-format=short` after the raise at /implement; `cargo metadata --offline` → 28 packages at
   1.95.0.

## Filtered
- "nextest captures a passing test's `[skip]` line, so a skip under `env -i` is an inference" — duplicate of testing.md
  2026-10-04 "A CLEAN-SKIP IS INVISIBLE TO NEXTEST'S COUNTS". The evidence record first stated the arm as observed,
  then was corrected against the stage-5 output → logged `recurrence-despite-learning` (handoff).
- "a stated process re-measure must be taken before it is written" — below threshold (one-off, self-caught; 0.4).
- "the skip-arm cleanup mirrors the success arm's `remove_file`" — task-specific (names private test fns).
- "the `/tmp` lock residue's origin was undetermined by name-recurrence" — task-specific.
- Pipefail facet also logged as `recurrence-despite-learning` (handoff): the handoff's standing open item "the
  `producer | grep -q` under pipefail plan-authoring CHECK" — a plan gate again mis-predicted a pipeline's exit under
  pipefail.
