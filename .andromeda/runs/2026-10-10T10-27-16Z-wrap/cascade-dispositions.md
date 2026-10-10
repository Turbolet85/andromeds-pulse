# Cascade dispositions — 2026-10-10-no-gate-stands-while-reading-nothing

The sweep ran twice. The first listing (`total (18 patterns) · 120 rows over 16 files`) found two restatements of
the perf-samples command without its flag (`obs-plan.md:559`, the CI/CD approach key file); both were folded into
this pass, the leaves were re-derived, and the sweep ran again. The counts below are the second listing's, copied:

```
total (18 patterns) · 126 rows over 17 files
per class · new 29/5 · standing 39/7 · leaf 50/9 · curation 8/2 · base 0/0
```

**The search.** Patterns in `cascade-patterns.toml`, each with its control fired on the pre-pass masters at
`7419496b`: `branch70` · `ge70` (the 70 % branch threshold, as words and as a table cell) · `no-tests` · `mcp-cmd`
· `perf-samples-cmd` (the nextest invocations) · `no-arm-req` · `pb-neutral` (the grader's callers and the line
`ci-gates` printed) · `boot-line` (the boot job's frame line) · `zero-span` · `exit-code-val` · `post-test` ·
`ci-greps` (the zero-span and panic enforcement sentences) · `hb-gap` (the gap check) · `trivially` · `quarantine`
· `ci-gates` (every mention of the verb) · `cov-gate` · `seed-log`. One pattern was dropped before the first run:
its control never fired on the pre-pass masters (the tool refused it), and `ci-greps` replaced it.
**Not looked for:** a restatement that names none of these words and none of the verb's name; the counts of tests
(no master states 2890, 2928 or a per-file count: 0 pre-pass hits each).

## Master and key-file rows — `new` (29 rows, 5 files): this pass's own text

Each was re-read after its write for a duplicate of the retired claim on the same line; none stands.
`architecture.md:34` (the `bash`/`awk` row) · `architecture.md:261` (the rewritten grader sentence and the two new
entries: `zero-span` ×4, `quarantine` ×7, `ci-greps` ×3, `seed-log`) · `test-plan.md:100`, `:135`, `:151`, `:195`,
`:491`, `:497`, `:502` · `obs-plan.md:556`, `:557`, `:558`, `:559`, `:579` · `registries/contracts/test-plan/
per-chunk-gate-discipline.md:32` · `registries/contracts/architecture/ci-cd-approach.md:3` (the flag) → **amended**.

## Master and key-file rows — `standing` (39 rows, 7 files)

- **amended in place** (`standing edited`): `bootstrap-phases.md:10` · `test-plan.md:493`, `:497`, `:510`, `:545` ·
  `obs-plan.md:499`, `:545`, `:557`, `:558`, `:559`, `:579` · `architecture.md:261` · the two key files' lines above.
  Where swept words still stand on an amended line they are history, written in the past tense: `obs-plan.md:559`
  ("before that chunk it graded … with no arm required and printed `perf-budget NEUTRAL`"), `obs-plan.md:545` and
  `test-plan.md:135` (the boot job's line "read").
- **no change, read:**
  - `test-plan.md:567` (`no-arm-req`) — `perf:load-profiles` applying the grader with no arm required: true.
  - `per-chunk-gate-discipline.md:20` (`no-arm-req`) — `check:ingest-progress`'s NEUTRAL arm compared with the
    grader's all-empty verdict: true, and not this chunk's subject.
  - `obs-plan.md:127`, `:291` (`post-test`) — the memory budget "asserted post-test" and the rationale's post-test
    `jq` analysis: neither names a CI step this chunk changed; `:291` already says the enforcement is `perf:budget`.
  - `obs-plan.md:548` (`trivially`) — the L4 latency grader's history.
  - `obs-plan.md:545` (`hb-gap`) — `perf:load-profiles`' run-window sentence: true.
  - `test-plan.md:512`, `:576`, `:579`, `:643`, `a11y-plan.md:564` (`quarantine`) — the flake-quarantine policy: true.
  - `test-plan.md:193`, `:501` (`cov-gate`) — the `coverage` job named by its key: true.
  - `ci-cd-approach.md:3` (`ci-gates` ×3) — the boot job's step order and "skipped when the smoke or the series
    fails": true.
  - `architecture.md:261` (`ci-gates`, the offsets outside the rewritten sentence) — the series before `ci-gates`,
    and `pre-push:linux`'s six stages: true.

## Leaf rows (50 rows, 9 files) — re-derived, then re-read

- **re-derived this pass:** `CLAUDE.md:67`, `:89` · `.claude/rules/testing.md:56` ·
  `.claude/docs/tests-summary.md:54`, `:72`, `:90` · `.claude/docs/workflow.md:43` ·
  `.claude/rules/verification-harness.md:66`, `:91`, `:97` · `.claude/rules/observability.md:96`, `:97`, `:99`, `:101`
  · `.claude/docs/obs-summary.md:66`, `:76`, `:77`, `:80` · `.claude/docs/commands.md:107` · `.claude/docs/stack.md:12`
  (the new row).
- **no change, read:** `observability.md:98`, `:102`, `obs-summary.md:78`, `:87` (the progress companion to the gap
  check; the script's NEUTRAL tolerance for an unrequired arm: true of the script) · `verification-harness.md:85`
  (the series before `ci-gates`) · `commands.md:104`, `tests-summary.md:94` (the six stages) · `testing.md:18`,
  `:59`, `:60`, `tests-summary.md:73`, `:115` (coverage scope; flake quarantine).

## Curation-home rows (8 rows, 2 files) — never edited by the cascade

- `.claude/rules/verification-harness.md:118` (`ci-gates` ×6, `ci-greps`) — the 2026-05-14 Session Addition says a
  fresh empty data dir makes `ci-gates` "return NEUTRAL on empty logs". This chunk made that false (exit 2,
  `cannot-evaluate`) → **P3, a correction in place**.
- `.claude/rules/verification-harness.md:124` — names the 2026-05-14 entry by its subject: no claim of its own →
  no change.
- `.claude/docs/session-learnings.md:269`, `:586`, `:1414`, `:2629`, `:2631` — other subjects sharing a word → no
  change.

## Judgment bases

0 rows in `playbook.md` and `drift-base.md`.
