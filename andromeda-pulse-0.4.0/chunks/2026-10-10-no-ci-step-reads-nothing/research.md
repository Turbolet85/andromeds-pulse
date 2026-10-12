# Codebase Research — 2026-10-10-no-ci-step-reads-nothing

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, Session Additions included; 0
  additions applied to a leg (the chunk has no local live leg: its run-level witness is the operator pass's push and
  CI read). `.claude/rules/testing.md` — 2 additions applied to the research reads themselves: a job log is fetched to
  a file with `--allow-escape-sequences` and counted from the file (2026-06-05, extended 2026-10-09), and a claim
  about a CI log names the run it read.
- **Platform issues consulted:** `actions/upload-artifact` `action.yml` at the sha `ci.yml` pins
  (`ea165f8d65b6e75b540449e92b4886f43607fa02`, fetched through `gh api
  repos/actions/upload-artifact/contents/action.yml?ref=…`) → the `if-no-files-found` input reads "warn: Output a
  warning but do not fail the action · error: Fail the action with an error message · ignore: Do not output any
  warnings or errors, the action does not fail", default `warn`. No runner-only failure is in scope, so no issue
  tracker was searched.
- **External inputs:** `inputs#I1` — the operator's phase directive for this chunk: no third state; the fall of each
  comparison is a P4 dialog; the two riders settled here by lean; the guard carry decided in the plan; the plan card
  and a stop at P5. `inputs#I2` (snapped at P4) — the operator's four answers at the P4 dialog: criterion, coverage
  and the three empty uploads gone; the a11y download gone and its comparison kept on the tree baseline with a
  fails-when-absent pin; P-128 not claimed here.
- **Measured for the plan's mechanism (P4, on this host, node v26.8.2):** `node tests-a11y/regression-detector.mjs
  --baseline {absent} --current {a summary holding no tuple}` exits 0 today, printing `baseline not found at …;
  treating as empty baseline`; with one tuple in the current it exits 1; the tree baseline against itself exits 0.

## The newest run, read step by step
`ci#38031822696` on `6df9e95c` (pull-request event, run attempt 1, concluded `success`; `ci.py conclusion`: green, 7
of 7 checks, wall 1071 s). The workflow file is byte-identical at `4e61553b` and `6df9e95c`
(`git diff --quiet 4e61553b 6df9e95c -- .github/workflows/ci.yml`: exit 0), so `ci#38026637514` attempt 1 (on
`4e61553b`) is a second reading of the same workflow where one is named. Each job log was fetched to a file through
`gh api --allow-escape-sequences repos/{owner}/{repo}/actions/jobs/{id}/logs`; each file is non-empty (lint
2,495,881 B · a11y 412,006 B · boot 360,415 B · coverage 960,026 B).

| step (`ci.yml` at `6df9e95c`) | job | what the run shows |
|---|---|---|
| upload `logs-perf-samples-${{ runner.os }}` `:126-133` | lint-test | 1 file (`agent-latest.jsonl.2026-10-10`, 50,427 B); `perf:budget` graded 120 records before it |
| upload `logs-${{ runner.os }}` `:135-142` | lint-test | `No files were found with the provided path: …/andromeda-pulse-ci-data/logs/` |
| upload `nextest-${{ runner.os }}` `:144-151` | lint-test | `No files were found with the provided path: target/nextest/**/junit.xml` |
| upload `criterion-${{ runner.os }}` `:153-160` | lint-test | `No files were found with the provided path: target/criterion/` |
| download `criterion-${{ runner.os }}-base` `:162-168` | lint-test | `##[error]Unable to download artifact(s): Artifact not found for name: criterion-Linux-base` |
| `cargo xtask criterion-regression` `:170-171` | lint-test | `criterion-regression-check: current dir target/criterion not found; NEUTRAL (no criterion bench output yet)` |
| upload `capability-drift-${{ runner.os }}` `:173-180` | lint-test | 1 file (`report.json`, 2,789 B) |
| download `a11y-violations-base` `:293-299` | a11y | `##[error]Unable to download artifact(s): Artifact not found for name: a11y-violations-base` |
| `cargo xtask test:a11y` `:301-302` | a11y | `regression-detector: no new violations vs baseline (0/0)`; no `baseline not found` line in the log |
| upload `a11y-violations-${{ runner.os }}` `:304-314` | a11y | 5 files: three `logs/a11y-*` (`a11y-axe-core-results.jsonl`, `a11y-lighthouse-results.json`, `a11y-violations-summary.json`), `contrast-report.json`, `regression-set.json` |
| upload `playwright-a11y-report-${{ runner.os }}` `:316-323` | a11y | 1 file (`results.json`) |
| upload `logs-boot-${{ runner.os }}` `:412-419` | boot | 42 files |
| download `coverage-linux-base` `:581-587` | coverage | `##[error]Unable to download artifact(s): Artifact not found for name: coverage-linux-base` (also on `ci#38026637514` attempt 1) |
| `cargo xtask coverage-regression` `:589-590` | coverage | `coverage-regression-check: baseline target/lcov-baseline/lcov.info not found; NEUTRAL (first PR / new branch / local dev)` (also on `ci#38026637514` attempt 1) |
| upload `coverage-linux` `:592-599` | coverage | 1 file (`lcov.info`, 1,441,519 B) |

The run's artifact list (`gh api …/actions/runs/38031822696/artifacts`) holds six names: `logs-boot-Linux`,
`capability-drift-Linux`, `logs-perf-samples-Linux`, `playwright-a11y-report-Linux`, `a11y-violations-Linux`,
`coverage-linux`. `ci#38026637514` holds the same six (its `logs-boot-Linux` three times, one per attempt). No
`logs-Linux`, `nextest-Linux` or `criterion-Linux` artifact exists on either.

Also read on that run, of a different shape (scope.md, "Read at take-up and at P3"):
- coverage thresholds: `Line: 33992/38252 = 88.9%` · `Branch: 0/0 = 100.0% (threshold 70%)` · `Function: 3565/4055 =
  87.9%`, the same three lines on `ci#38026637514` attempt 1. The branch arm passes over no branch count.
- boot job `ci-gates`: `zero-spans PASS (118 log records across 1 file(s))` · `heartbeat-gap-check: max gap 0ms in
  (threshold 45000ms) PASS` · `perf-budget: memory NEUTRAL — populated 0 of 1` · `snapshot NEUTRAL` · `perf-budget
  NEUTRAL`.

## Files inspected
- `.github/workflows/ci.yml` (full, 599 lines) — six jobs; 100 step blocks, 53 `run:` and 47 `uses:`; nine uploads,
  three downloads, three `continue-on-error` keys (`:165`, `:296`, `:584`); all 47 `uses:` lines carry a 40-hex ref
  and a version comment (`grep -E '^\s+uses:' | grep -v -c -E '@[0-9a-f]{40} # '`: 0).
- `pulse-app/tests/quality_gate_workflow.rs` (`:1-470`) — the workflow pins; `read_workflow` (`:18`) and
  `workflow_job_block` (`:79`) are the helpers a new pin reuses; the soft-fail guard (`:257-299`) splits on
  `      - name: ` and skips every block that holds none of 16 command substrings.
- `pulse-app/tests/a11y_perf_workflow.rs` (`:70-96`) — the two a11y upload pins assert a substring of the artifact
  name only (`a11y-violations-`, `playwright-a11y-report-`).
- `xtask/src/main.rs` (`:237-259`, `:344-349`, `:1077-1124`, `:1173-1220`) — the two verbs shell out to
  `xtask/ci/{coverage,criterion}-regression-check.{sh,ps1}` and return the script's exit.
- `xtask/ci/criterion-regression-check.sh` (full) — five exit-0 NEUTRAL arms: current dir absent, baseline dir
  absent, `jq` absent, no `estimates.json`, no bench with a matching baseline.
- `xtask/ci/coverage-regression-check.sh` (full) — two exit-0 NEUTRAL arms: current file absent, baseline absent.
- `pulse-app/ui/tests-a11y/regression-detector.mjs` (full) — default baseline is the tracked file
  `tests-a11y/baselines/a11y-violations-summary.json` (`:9`); an absent baseline is read as an empty one (`:63`, `:69`),
  so every current tuple would then count as new and the step would exit 1; it writes `regression-set.json` (`:95`).
- `pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json` (full) — tracked, 6 informational
  `lighthouse-score` tuples, generated 2026-06-09 (`git log`: commits `b00f2d79` and `749778db`).
- `.config/nextest.toml` (section headers) — four profiles, no `junit` key (`grep -i junit`: 0 lines).
- `.github/workflows/release.yml`, `update-channels.yml`, `secret-scan.yml` (`on:` blocks and artifact steps) —
  triggers as scope.md states; no upload is named `-base`.
- `.andromeda/obs-plan.md` (`:499-501`, `:505-512`, `:520-527`, and every `criterion` line), `.andromeda/test-plan.md`
  (`:493`, `:505`), `.andromeda/registries/contracts/a11y-plan/ci-integration.md` (full),
  `.andromeda/registries/contracts/test-plan/5-command-implementation.md` (`:12`) — the sentences the wrap corrects.
- `andromeda-pulse-0.4.0/chunks/2026-10-09-ci-on-linux-alone/report.md` (`:105-126`) — the record the entry's carry
  cites; its Spec claims disproved 1 and 2 read as the carry says.

## Graph impact (from the code-graph query; rust plane)
- **run_criterion_regression** — 1 caller: `main` @ `xtask/src/main.rs:349`; it calls
  `invoke_criterion_regression_check` @ `xtask/src/main.rs:1215`. Removing the verb touches `main.rs` alone.
- **run_coverage_regression** — 1 caller: `main` @ `xtask/src/main.rs:345`; it calls
  `invoke_coverage_regression_check` @ `xtask/src/main.rs:1119`. Removing the verb touches `main.rs` alone.
- **CriterionRegression** / **CoverageRegression** — the two clap variants @ `xtask/src/main.rs:255` and `:240`; no
  other symbol of either name on the plane.
- **ci_workflow_test_gates_no_continue_on_error** @ `pulse-app/tests/quality_gate_workflow.rs:257` — a test fn, no
  callers; `read_workflow` is defined three times (one per test file that reads a workflow), so a new pin in
  `quality_gate_workflow.rs` reuses that file's own.
- The companion sweep (`grep -rn -E 'criterion-regression|coverage-regression|criterion_regression|
  coverage_regression|criterion-baseline|lcov-baseline|a11y-violations-base|coverage-linux-base|junit|
  if-no-files-found|continue-on-error'` over `crates pulse-app/src pulse-app/tests pulse-app/ui/tests-a11y xtask
  scripts .github .config`): hits outside `ci.yml` in `pulse-app/tests/quality_gate_workflow.rs` (the pins above),
  `xtask/src/main.rs` and the four `xtask/ci/*-regression-check.*` scripts (the verbs),
  `pulse-app/ui/vitest.config.mjs:13` (the webview suite's own JUnit file, `target/junit-ui.xml`: no change),
  `release.yml` and `update-channels.yml` (five `if-no-files-found: ignore` lines and `release.yml:183`'s
  `logs-${{ runner.os }}`: no change, outside the chunk). No test under `xtask/` names either verb; `xtask/ci/fixtures/`
  holds three `l4-latency-*` files only.

## Patterns detected
- **A pin reads the workflow file from the tree** (`pulse-app/tests/quality_gate_workflow.rs:18-22`, `:100-127`):
  `read_workflow()` then a line-wise or block-wise assertion; `ci_workflow_runs_on_linux_only` walks every line and is
  the shape of a whole-file ban.
- **A producer beside its grader and its upload** (`ci.yml:118-133`): the perf-samples producer, `perf:budget
  --require memory,snapshot`, then the upload. It is the one upload in the file whose input has a gate that fails when
  the producer wrote nothing.
- **A baseline held in the tree** (`pulse-app/ui/tests-a11y/regression-detector.mjs:9`): the a11y comparison's
  default; the download, had it ever found an artifact, would have overwritten that file in the checkout.
- **A step-scoped soft-fail pin** (`pulse-app/tests/quality_gate_workflow.rs:552`, `:647`): the boot smoke
  and series steps each have a pin banning `continue-on-error`, `retry` and `|| true` on that one step.

## Conventions to follow
- **Workflow pins live under `pulse-app/tests/`**, never in `pulse-app/src/` (`[lib] test = false`; test-plan §2), and
  are collected by name in the default workspace run.
- **A comment line in `ci.yml` may not hold the tokens `windows`, `macos`, `matrix.os` or `runner.os ==`**
  (`ci_workflow_runs_on_linux_only`, `quality_gate_workflow.rs:115`); a new step comment is worded without them.
- **A step's `name:` line is the split point** of every block-wise pin (`      - name: `, six spaces); a step this
  chunk edits keeps that indentation.
- **A mutation check proves a green-on-arrival pin discriminates** (`.claude/rules/testing.md`, 2026-08-17): the
  mutation is applied with the anchored Edit tool, confirmed present by a grep, then the pin is run.

## What a producer would cost (for the P4 dialog)
- **Criterion.** There is nothing to compare on either side: no step runs a bench; `ls xtask/benches` finds no
  directory; `grep -rn criterion --include=Cargo.toml .` finds no dependency line; `xtask/src/main.rs` has no `bench`
  verb. A producer is a bench suite, a dependency, a bench step and a baseline source. No entry of the 0.4.0 route
  names a bench (`grep -n -i -E 'criterion|bench' andromeda-pulse-0.4.0/working-route.md`: line 23 only, this entry).
  The snapshot p99 budget is enforced by `perf:budget --require memory,snapshot` over log samples, not by criterion
  (obs-plan §10, `obs-plan.md:544`).
- **Coverage.** The current side exists (`lcov.info`, uploaded as `coverage-linux`). A baseline source is one of:
  (a) the last push run on `main`, read across runs: `actions/download-artifact` needs a `run-id`, a token input and
  `actions: read`, which is a new permission and a token (the founder's own word, inputs#I1 item 3), and `main`
  carries the old workflow until the version lands (architecture §Infrastructure Patterns → CI/CD approach);
  (b) the Actions cache, last read 1,471,243,881 B over the 10 GB cap (same key file); (c) a file held in the tree,
  as the a11y baseline is: no permission, but every chunk that moves a percentage down for a stated reason must
  regenerate it, and Epochs 2 and 3 remove whole crates. Without the comparison the absolute thresholds step stays
  (`ci.yml:552-579`).
- **A11y.** The comparison has a baseline in the tree and reads it. The download has no producer in any workflow; a
  producer for the artifact name would be the same cross-run read as (a). The job leaves at `Window's gates retired`.
- **`nextest` JUnit upload.** A producer is two lines in `.config/nextest.toml` (a `[profile.ci.junit]` table with a
  `path`), which makes nextest write `target/nextest/ci/junit.xml` on the `--profile ci` run `cargo xtask test` makes
  (`xtask/src/main.rs:464-468`); no permission, no action. No step or tool reads a JUnit file today; test-plan §9's
  `dorny/test-reporter` is in no workflow.
- **`logs` upload in `lint-test`.** No step of the job writes a log family under its data dir. A producer is a step
  that boots the app there, which is the `boot` job's work and its `logs-boot` upload. The later entry `Engine
  end-to-end gate reachable` keeps the engine's log in CI and adds the upload that finds it
  (`working-route.md:41`).

## New files to create
- none

## Files to modify
- `.github/workflows/ci.yml` — eight steps leave; the six uploads that stay fail when they find no file
- `pulse-app/tests/quality_gate_workflow.rs` — five pins leave with their steps, one is re-pointed, the guard is widened, two pins are added
- `pulse-app/tests/a11y_perf_workflow.rs` — two pins added: the tree baseline is present, the comparison fails when it is absent
- `pulse-app/ui/tests-a11y/regression-detector.mjs` — an absent baseline ends the run with exit 1
- `xtask/src/main.rs` — the two comparison verbs leave
- `xtask/ci/criterion-regression-check.sh` — deleted
- `xtask/ci/criterion-regression-check.ps1` — deleted
- `xtask/ci/coverage-regression-check.sh` — deleted
- `xtask/ci/coverage-regression-check.ps1` — deleted

## Open questions
- none. The two questions research carried to P4 were answered by the operator at the P4 dialog (inputs#I2): which
  way each comparison and each empty upload falls, and whether P-128's closing sentence reaches the gates that are
  neither comparisons nor uploads (it does; P-128 is not claimed here).
