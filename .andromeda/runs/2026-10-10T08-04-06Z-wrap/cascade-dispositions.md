# Cascade sweep dispositions — 2026-10-10-no-ci-step-reads-nothing

Written after `cascade.py sweep` ran over this pass's bodies (`cascade v1.2 · 89172b3d`, baseline `6df9e95c`, the
pre-CI commit's parent). Counts copied from the listing: **total (15 patterns) · 57 rows over 16 files** ·
**per class · new 16/6 · standing 25/8 · leaf 15/7 · curation 1/1 · base 0/0**.

## What was searched

Fifteen patterns in `cascade-patterns.toml`, each with a control that fired on the pre-pass masters: the bench
claim by token and by phrasing (`criterion`, `xtask-bench`, `regr-detect`); the JUnit report (`junit`,
`test-reporter`); the base-branch baseline (`a11y-base`, `base-branch`, `artifact-cmp`); the test job's log artifact
(`logs-os`, `n-logs`, `logs-artifact`); the snapshot artifact (`snapshot-art`); the count of binaries that shell
out (`two-binaries`); the guard by name (`soft-fail-guard`); the detector and its baseline (`regr-detector`).

Not in the tool's set, because no control fires on a master: the two removed verbs and their scripts
(`coverage-regression|criterion-regression|regression-check`) and the detector's old wording (`treating as empty`).
Searched by hand over `CLAUDE.md`, `.claude/rules`, `.claude/docs`, `docs` and `scripts`: 0 hits. Also by hand over
the leaves: `logs-${{ runner`, `nextest-$`, `junit.xml`, `-n logs`, `a11y-violations-base`, `if-no-files-found`,
`Two unit binaries`, `shell out`: 0 hits; `continue-on-error`: one, `.claude/rules/security.md:79`, the audit step's
own shape, true. `.claude/docs/obs-summary.md` was read for `artifact|bench`: 0 hits.

Two patterns printed 0 rows after their control fired (`artifact-cmp`, `logs-artifact`): the three phrasings of each
no longer stand anywhere the tool reads.

## Master and key-file rows (41: new 16, standing 25)

- **new, 16 rows over 6 files** — this pass's own text, each read as written: `obs-plan.md:499`, `:509`, `:544`,
  `:559`; `test-plan.md:188` ×2, `:493`, `:514`, `:516`; `a11y-plan.md:495`; `architecture.md:33`; the key files
  `a11y-plan/ci-integration.md:3`, `:12` ×3 patterns; `test-plan/5-command-implementation.md:12`. Each names the
  retired thing to say it is gone, or names a thing the pass added. No change.
- **standing and edited, 16 rows** — amended lines where a swept token still stands, each re-read for a duplicate
  of the retired mechanism: `obs-plan.md:125` ×2, `:501` ×2, `:544`, `:545`, `:559`, `:500`, `:524`;
  `a11y-plan.md:566`; `test-plan.md:56`, `:493`, `:505` ×2; the key files `ci-integration.md:12`,
  `harness-wiring-conventions.md:6`. In each the token now sits in a sentence that states the absence or the tree
  baseline. `test-plan.md:505` holds `JUnit` four times and `obs-plan.md:501` `Criterion` twice: read whole, each
  occurrence is the negative statement or the vitest file. No change.
- **standing, not edited, 9 rows:**
  - `test-plan.md:55`, `:190` (`junit`) — the webview suite's `target/junit-ui.xml`, which vitest writes. True, no
    change.
  - `test-plan.md:494` (`soft-fail-guard`) — "the `cargo audit` entry of
    `ci_workflow_test_gates_no_continue_on_error`". True: `cargo audit` is one of the 14. No change.
  - `registries/a11y-plan-contracts.toml:44`, `:49` (`regr-detect`, `regr-detector`) — the labels "Per-PR regression
    detection" and "Regression baseline"; neither label was renamed. No change.
  - `architecture.md:220`, `security-plan.md:29` (`snapshot-art`) — the product's own snapshot markdown files under
    the data dir, another subject. No change.
  - `obs-plan.md:634`, `:636` (`snapshot-art`) — §11's two bans. Requirements, not descriptions; left as they stand
    and said at the route-resolve card (`fanout-results.md`, "Not amended").

## Curation row (1)

- `.claude/rules/testing.md:171` (`regr-detect`, at offset 1113 of 2395) — a 2026-05-14 Session Additions entry
  saying a later chunk's "perf-budget regression detection will reuse this generator shape". A dated statement about
  a fixture generator, in a preserve-verbatim home; it asserts no bench. No change.

## Leaf rows (15 over 7 files) — each leaf joined step 3's set and was re-derived from the amended master

- `.claude/rules/observability.md:90` (`criterion`, `xtask-bench`) — re-derived from obs-plan §2 / §5: perf:budget
  grades the budgets, no bench suite exists.
- `.claude/docs/commands.md:154` (`criterion`, `regr-detect`) — the `cargo bench` block replaced by the perf:budget
  command and the measured absence.
- `.claude/docs/tests-summary.md:59` (`criterion`, `xtask-bench`), `:85` (`junit` ×2, `test-reporter`) — re-derived
  from test-plan §9; two bullets added from the same section (no download, no soft-fail key, uploads fail on no
  file; `node` as a test-time requirement).
- `.claude/docs/workflow.md:44` (`criterion`), `:48` (`base-branch`) — re-derived.
- `.claude/rules/a11y.md:91` (`base-branch`, `regr-detector`) — re-derived from the a11y CI-integration key.
- `.claude/docs/a11y-summary.md:80`, `:88` (`base-branch`) — re-derived from a11y-plan §10 and the key's CI gate
  list.
- `.claude/rules/testing.md:18` (`junit` ×3) — the vitest JUnit file. True, no change.
- `.claude/docs/stack.md` — no row (the sweep holds no pattern for it); re-derived for architecture §Stack's new
  row by provenance (`Extracted from .andromeda/architecture.md Stack and Technologies`). `conventions.md`, the other
  arch leaf by provenance, derives from §Conventions, which this pass did not amend.

## Binds

- test-plan §3 ↔ obs-plan §3: the one §3 edit is the `run` → Output format label; obs-plan §3 states no JUnit
  format (`junit` rows: none in obs-plan or its key files). Consistent.
- a11y schema ↔ obs schema: no schema changed.

## After the route-resolve card

Three more body edits on the operator's word (`route-card.md`), each an addition that retires no wording, so no
pattern was added to the sweep: the merge-base probe paragraph in the test-plan key file `per-chunk-gate-discipline.md`;
an owner clause in obs-plan §9's Snapshot markdown row; an owner sentence in obs-plan §10 CI gates. Leaves that
restate the gate set, found by `grep -F 'chunk-gate-baseline-coverage'` over `.claude/rules` and `.claude/docs`
(`rules/testing.md` 2 hits, `docs/tests-summary.md` 1 hit, `workflow.md` and `commands.md` 0): the one bullet in each
of the two files that lists the set gained the probe. `registry.py check` on test-plan: 0 defects.
