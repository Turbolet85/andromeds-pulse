# Cascade step 2 — the sweep and its dispositions

The search: `cascade.py sweep --patterns-file cascade-patterns.toml`, eleven patterns derived from the pass's 36
amendments after the last body edit, over the seven masters, every `.andromeda/registries/**` file, the three
curation homes, the two judgment bases and the leaf bodies. Baseline `8936f976`. Every pattern's control fired on the
pre-pass text. The listing's own lines: `total (11 patterns) · 27 rows over 12 files` ·
`per class · new 1/1 · standing 15/5 · leaf 11/7 · curation 0/0 · base 0/0`.

What was looked for: the count of harness-written files under `logs/` (`two-files`), the artifact's file count
(`five-files`), the `ci-gates` frame line conditioned on the smoke step alone (`smoke-passes`), the settle verdict's
seven-member list (`seven-members`), `harness:settled` with one caller (`one-caller`), the cause "not named"
(`not-named`), the driver pair "in lockstep" (`lockstep`), "one unit binary shells out" (`one-shellout`), the `boot`
job's step sequence without the two new steps (`boot-job-seq`), every site naming `harness-settled.json`
(`settled-json`), every site naming `P-129` (`p129`). What was not looked for: sites that describe the boot job or
the harness-only class with none of those tokens; the detectors' own reads of each master are the cover for those.

## Master and key-file rows (16)

- `architecture.md:218` two-files · standing edited → amended after the sweep: "it also holds two harness-written
  files" now reads "harness-written files …, two of them since chunk 2026-10-09-…", followed by this pass's larger
  set.
- `security-plan.md:395` two-files · standing edited → no change: it is clause (b) of the 2026-10-09 chunk's own
  class entry, which lists that chunk's two files; this pass added, in the same clause, that the artifact holds
  further harness-written files since this chunk.
- `obs-plan.md:499` two-files · standing edited → no change at this match: "two harness-written files the product
  never reads" lists the two of the 2026-10-09 reading inside the sentence this pass extended with the three more.
- `architecture.md:218` five-files · standing edited → no change: "five files as read on `ci#37979648967`" is a
  dated reading, followed by this pass's reading on `ci#38019133294`.
- `obs-plan.md:499` five-files · standing edited → amended after the sweep: "the artifact holds five files" now reads
  "held five files, as read on `ci#37979648967`".
- `obs-plan.md:545` smoke-passes · standing edited → no change: the match is this pass's own sentence, which names
  the series step beside the smoke step.
- `architecture.md:258` lockstep · standing edited → no change: this pass's own text ("in lockstep but for the
  sh-only exit-witness arm of `boot`").
- `test-plan.md:145` lockstep · standing → no change: a true claim sharing the token (the L4 latency budget made
  gradable "in lockstep").
- `registries/contracts/test-plan/5-command-implementation.md:34` lockstep · standing → no change: the `cleanup`
  verb's ps1 mirror, which this chunk did not touch.
- `architecture.md:218` settled-json ×2 · standing edited → no change: the 2026-10-09 file and this pass's copies.
- `architecture.md:258` settled-json ×3 (two rows) · standing edited → no change: the settle verdict's twin, the
  series' kept files, the smoke's verdict as ordinal 1; all this pass's text or unchanged contract text.
- `security-plan.md:85` settled-json · standing edited → no change: this pass's own enumeration.
- `security-plan.md:395` settled-json ×2 · standing edited → no change: the 2026-10-09 clause and this pass's arm.
- `test-plan.md:496` settled-json ×2 · standing edited → no change: this pass's own text.
- `obs-plan.md:499` settled-json ×3 · standing edited → no change: this pass's own text.
- `registries/contracts/test-plan/5-command-implementation.md:5` p129 · new → no change: this pass's own sentence
  ("P-129 is not claimed").

Zero-row patterns, each with a fired control: `seven-members` (0 master rows: both stated lists now read eight),
`one-caller`, `not-named`, `one-shellout`, `boot-job-seq`. A statement about those patterns, not an absence proof.

## Leaf rows (11) — each leaf joins step 3's set and is re-derived, not patched by wording

- `.claude/rules/security.md:17` two-files, settled-json → re-derived (the Input paragraph gains the exit-witness
  arm, its classification PROVISIONAL).
- `.claude/docs/security-summary.md:43` two-files, settled-json → re-derived.
- `.claude/rules/observability.md:99` smoke-passes ×2 → re-derived (the frame line needs the smoke and the series).
- `.claude/docs/obs-summary.md:80` smoke-passes ×2 → re-derived.
- `.claude/rules/verification-harness.md:84` seven-members, settled-json → re-derived (eight members; the series
  verb; the boot verb's witness arm at `:22`).
- `.claude/rules/verification-harness.md:25` lockstep → no change: the `cleanup` bullet's ps1 mirror, untouched by
  this chunk.
- `CLAUDE.md:44` settled-json → re-derived (the path-variable warning's harness-only enumeration).
- `.claude/docs/commands.md:102` settled-json → re-derived (the settle verdict's members; the series verb).

Leaves added by provenance, with no row: `.claude/docs/tests-summary.md`, `.claude/docs/stack.md`,
`.claude/rules/testing.md` (the Framework lines), CLAUDE.md's overview and modules lines for `xtask/`.

## Curation homes and judgment bases

0 rows in the three curation homes and 0 in `playbook.md` and `drift-base.md`.
