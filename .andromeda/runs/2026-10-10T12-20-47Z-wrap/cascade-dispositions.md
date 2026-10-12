# Cascade dispositions — 2026-10-10-capability-record-re-based

**The search.** `cascade.py sweep` over `cascade-patterns.toml`, baseline `279a477e`, after all four body amendments
were applied. Six patterns, each with its control firing on the pre-pass test plan (`test-plan.md:535`, `:533`):
`old-path` (`capability-verification-matrix`) · `verb` (`verify:capability-matrix`) · `matrix-name` (`capability
verification matrix`, any case) · `sixty-range` (`P-001.{1,3}P-060`) · `sixty-count` (`\b60 (P-|capabilit|P-entries)`)
· `extend` (`extend(s|ed)? the matrix`). They key the one retired claim of the pass: test-plan §9 said the gate
validates the old matrix file, all 60 ids, and that ids from P-061 on extend it. The two architecture amendments add
text and retire no claim, so no pattern was derived from them.

**The listing's counts, copied:** `total (6 patterns) · 28 rows over 7 files` · `per class · new 5/1 · standing 4/1 ·
leaf 14/3 · curation 5/2 · base 0/0`.

**Not looked for:** the old report twin's `verification_mode_counts` member and the old exit pair (0 / 1) — no master
or leaf stated either (the pre-pass site search found the gate named in one master line only). The masters' wording
about surfaces the record retires (the glow layer, corpus encryption, the training export) is not this pass's claim;
it belongs to the entries that remove those surfaces.

## Rows

**new (5 rows, 1 file) — this pass's own text, read after the write:**
- `.andromeda/architecture.md:171` `old-path` · `verb` — the new Standard Contracts bullet: names the old matrix file
  as superseded and the verb as the record's validator. True as written → no change.
- `.andromeda/architecture.md:263` `old-path` · `verb ×3` · `sixty-range` — the new registration on the xtask
  surfaces line (read by offset, `@c30643` to the line's end): the verb's name three times in its own contract, and
  the closing sentence saying what it read before this chunk. True as written → no change.

**standing (4 rows, 1 file):**
- `.andromeda/test-plan.md:536` `old-path` · `verb` · `sixty-range` (edited) — the amended paragraph. The old path
  and `P-001–P-060` stand once each, in its last sentence, which says both older records are superseded and read by
  no gate; the verb opens the paragraph. Re-read whole for a duplicate of the retired claim: none → amended, no
  further change.
- `.andromeda/test-plan.md:534` `matrix-name` — the paragraph's bold heading, "Capability verification matrix (CI
  step, runs after capability-drift)". The verb keeps its name and its place in the job; the heading names the
  gate, not its input → no change.

**leaf (14 rows, 3 files):**
- `.claude/rules/testing.md:58` (5 rows: `old-path` · `verb` · `matrix-name` · `sixty-range` · `extend`) — the
  Quality gates bullet restating the retired claim → re-derived from the amended paragraph.
- `.claude/rules/testing.md:107` (2 rows: `verb` · `sixty-range`) — the Running tests bullet, "P-001–P-060 scenario
  mapping" → re-derived.
- `.claude/docs/tests-summary.md:75` (5 rows: `old-path` · `verb` · `matrix-name` · `sixty-count` · `extend`) — the
  summary's restatement → re-derived.
- `.claude/docs/tests-summary.md:84` (1 row: `verb`) — the capability-gates chain, the verb tagged "(chunk #99)" →
  re-derived (the tag now says what the gate is).
- `.claude/docs/andromeda-after-mvp-playbook.md:330` (`sixty-count`) — describes `pulse-capability-spec.md` as "60
  P-XXX capabilities formal contract". A true claim about that 0.2 document, sharing the token → no change; marking
  the 0.2–0.3 documents as history is the route's (`Records say what the product is`).

**curation (5 rows, 2 files) — preserve-verbatim homes, never edited by the cascade:**
- `.claude/rules/security.md:160` (`verb` · `matrix-name` · `sixty-range`) — the 2026-06-11 Session Addition saying
  chunk #99 wired the "P-001–P-060 capability verification matrix gate" as a CI step. True of that chunk; the
  gate's input moved here → routed to P3 as an in-place extension of that entry.
- `.claude/docs/session-learnings.md:420` (`old-path`) — a learning about how the old matrix file is authored (one
  compact line per capability). Still true of that file, which stands unchanged → no change.
- `.claude/docs/session-learnings.md:1480` (`sixty-count`) — history of the 0.2 planning material ("capability-spec
  v2 60 P-XXX") → no change.

**base (0 rows):** neither judgment base states the claim.

## Leaves re-derived at step 3

From the amended test-plan paragraph: `.claude/rules/testing.md` (two bullets) and `.claude/docs/tests-summary.md`
(two bullets). From the two architecture additions, recomputed from the sections they derive from:
`CLAUDE.md` — the pointer-table row for §Standard Contracts (now naming the capability record's form) and the two
`xtask` lines (key directories, modules), which gain the verb; `.claude/docs/commands.md` — one line for the verb
beside its sibling gates. The tests summary does not enumerate the newer pending-coverage triggers (no line for the
two before this one), so the new trigger row adds none there. `CLAUDE.md` stays at 154 lines.

The citation sweep's four rows stand in `## Session Additions` of three rule files; they are P3's.
