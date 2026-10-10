# Scope — No CI step reads nothing

**Marker:** `2026-10-10-no-ci-step-reads-nothing` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:23`):** "No CI step reads nothing — every comparison has a producer for its baseline
or is gone; every upload finds its file or is gone (P-128)"
**Chunk base:** `6df9e95c` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
A CI step that compares against a baseline nobody produced, or uploads a file nobody wrote, passes while it reads
nothing. Two epochs of removals lean on these gates (the ruling that minted P-128, quoted in the ledger's notes for
`verification-matrix.json#P-128`). This chunk leaves every such step in one of two states: it reads something a
workflow produced, or it is gone. There is no third state: "kept, and neutral when nothing arrives" is the present
defect, not an option (inputs#I1 item 2).

## What the chunk builds
- **Every baseline comparison a CI job makes reads a baseline some workflow produced, or the workflow no longer makes
  the comparison.** Read at take-up, `.github/workflows/ci.yml` at `6df9e95c`:
  - three `download-artifact` steps, each pull-request-only and each `continue-on-error: true`: `:162-168`
    (`criterion-${{ runner.os }}-base`, job `lint-test`), `:293-299` (`a11y-violations-base`, job `a11y`),
    `:581-587` (`coverage-linux-base`, job `coverage`). The coordinates agree with inputs#I1 item 1 (`:164`, `:295`,
    `:583` are the three `uses:` lines).
  - their consumers: `cargo xtask criterion-regression --current target/criterion --baseline
    target/criterion-baseline` (`:170-171`), `cargo xtask coverage-regression --current lcov.info --baseline
    target/lcov-baseline/lcov.info` (`:589-590`), and the regression leg inside `cargo xtask test:a11y` (`:301-302`).
    The first two run on a push as well, where no download step runs at all.
  - Verified at P3 on the newest run, `ci#38031822696` on `6df9e95c` (pull-request event, green, 7 of 7 checks): each
    of the three downloads ends `Unable to download artifact(s): Artifact not found for name: …` (`criterion-Linux-base`,
    `a11y-violations-base`, `coverage-linux-base`), and no workflow uploads a name ending `-base` (the nine upload
    names of `ci.yml`, the three of `release.yml` and the two of `update-channels.yml`, read at P3). The entry's first
    `CARRY:` stands as measured on 2026-10-09 and again today.
  - Verified at P3, what each comparison does with nothing to read:
    - criterion: `criterion-regression-check: current dir target/criterion not found; NEUTRAL (no criterion bench
      output yet)`, exit 0. It has no baseline and no current input either: no step runs a bench, the workspace holds
      no bench target, no `criterion` dependency and no `bench` verb (research.md).
    - coverage: `coverage-regression-check: baseline target/lcov-baseline/lcov.info not found; NEUTRAL (first PR / new
      branch / local dev)`, exit 0, read on `ci#38031822696` and on `ci#38026637514` attempt 1.
    - [premise-corrected: the a11y comparison DOES read a baseline. `regression-detector.mjs:9` defaults to the file
      committed at `pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json` (tracked; regenerated at chunk #99,
      6 informational tuples), and the job log of `ci#38031822696` reads `regression-detector: no new violations vs
      baseline (0/0)` with no `baseline not found` line.] The download step reads nothing; the comparison reads a
      baseline held in the tree, which no workflow produced. The route's carry at `working-route.md:46` ("so its
      regression comparison reads no baseline") is false in that half; a report entry carries the correction, and
      which state the comparison is left in is the operator's fork at P4.
  - Verified at P3: the three `continue-on-error` keys of `ci.yml` are these three downloads' and no other step
    carries one (`grep -n 'continue-on-error' .github/workflows/ci.yml`: `:165`, `:296`, `:584`). A key that exists
    only so an absent baseline does not red the job goes with the download it shields (inputs#I1 item 2).
- **Every upload a CI job makes finds its file, or the workflow no longer makes the upload.** Read at take-up,
  `ci.yml` at `6df9e95c`: nine `upload-artifact` steps (`grep -c 'upload-artifact@' .github/workflows/ci.yml`: 9), every
  one `if: always()` with `if-no-files-found: ignore`.
  - Verified at P3 on `ci#38031822696` (job logs and the run's artifact list; the same six names on `ci#38026637514`):
    six find a file — `logs-perf-samples-Linux` (1 file), `capability-drift-Linux` (1), `a11y-violations-Linux` (5),
    `playwright-a11y-report-Linux` (1), `logs-boot-Linux` (42), `coverage-linux` (1) — and three find none, each
    printing `No files were found with the provided path`: `logs-Linux` (`:135-142`), `nextest-Linux` (`:144-151`),
    `criterion-Linux` (`:153-160`).
  - Verified at P3: the `a11y-violations` upload's three path members each match on that run (three `logs/a11y-*`
    files, `contrast-report.json`, `regression-set.json`). The action can fail a step only when no member matches at
    all; a single missing member is not something a key can red (research.md).
  - Verified at P3, why the three are empty: no step of `lint-test` writes a log family under the job's data dir (the
    job boots no app and its tests use their own temp dirs); nothing writes a JUnit file (`.config/nextest.toml` holds
    no `junit` key, and `cargo xtask test` asks nextest for `libtest-json`); nothing writes `target/criterion/`.
- **Which way each comparison and each empty upload falls is the operator's technical fork, brought at P4 as one
  dialog** (inputs#I1 item 3): each with what a producer would cost and what the gate is worth to the version as it
  will stand (a console engine; the window, its `a11y` job and its `boot` job leave in Epoch 2). A producer that
  needs a new permission, a token, a new trigger or a third-party action halts for the founder's own word; the
  option says so and is not chosen here.
- **The two of the kind that ride other entries are settled here with the rest** (inputs#I1 item 4, the operator's
  lean "for the plan to test"): the `a11y-violations-base` download (`working-route.md:46`, first `CARRY:`) and the
  `logs-${{ runner.os }}` upload (`working-route.md:41`, its `CARRY:`). Tested at P3: no artifact, pin or later route
  entry needs either step to stay (research.md), so the lean holds for the download and the upload. Two things are
  said at P4 beside it: obs-plan §11's ban on losing telemetry artifacts is written against the `logs` upload, and the
  a11y comparison's tree-held baseline is not "a baseline that a workflow produced" in P-128's words.
- **What reads `ci.yml` in the tree is kept true.** Verified at P3 (`grep -rln 'ci\.yml' crates pulse-app/src
  pulse-app/tests xtask scripts .github`): two test files read the workflow's steps,
  `pulse-app/tests/quality_gate_workflow.rs` and `pulse-app/tests/a11y_perf_workflow.rs`; `xtask/src/pre_push.rs`
  reads only its `node-version` major (`:148`, `:576-580`). The pins on the steps this chunk decides:
  `ci_workflow_invokes_coverage_regression_check` (`:218`), `ci_workflow_downloads_coverage_baseline_artifact`
  (`:228`), `ci_workflow_invokes_criterion_regression_check` (`:378`),
  `ci_workflow_downloads_criterion_baseline_artifact` (`:388`), `ci_workflow_uploads_criterion_artifact_unchanged`
  (`:406`), `ci_workflow_uploads_logs_artifact_unchanged` (`:417`), and in the second file
  `ci_workflow_uploads_a11y_violations_artifact` (`:77`) and `ci_workflow_uploads_playwright_a11y_report_artifact`
  (`:88`). Each pin follows its step: a step that goes takes its pin with it, a step that stays keeps a pin that
  reads what makes it non-empty.
- **A witness that cannot pass vacuously.** The acceptance is read "on a CI run of the tree", so the chunk's proof is
  a run on its pre-CI commit, read step by step and artifact by artifact, beside a self-lint pin in the tree.
  Verified at P3: `actions/upload-artifact` at the pinned sha takes `if-no-files-found: error` ("Fail the action with
  an error message"), so a kept upload can be made to red the step when it finds nothing; the pin reads that key.
- **The self-lint guard that reads 16 of the workflow's run steps: the plan decides whether it rides here or gets
  its own requirement line** (the entry's second `CARRY:`; inputs#I1 item 5).
  - Re-derived at P3 (the guard's own split, `pulse-app/tests/quality_gate_workflow.rs:257-299`, run over `ci.yml` at
    `6df9e95c`): 100 step blocks, 53 of them `run:` steps; the guard reads 17 blocks, 16 `run:` steps and the
    `cargo deny` action step; 37 `run:` steps are unread, and none of those 37 carries a soft-fail key. The carry's
    16 and 53 stand. Its list of unread gates stands too: the Cyrillic lint, typecheck, staged-artifacts,
    capability-widening, capability-matrix, the mcp-server build, the Cranelift assertion, the npm supply-chain gate
    and the coverage thresholds.
  - "this is outside P-128's text, which is about comparisons and uploads" (the carry). If the guard rides here it
    is not folded into P-128's claim (inputs#I1 item 5).
- **The masters' sentences this entry's wrap corrects are listed in the plan as expected amendments, as measured**
  (inputs#I1 item 6). Phase amends no spec source. Verified at P3, the sites:
  - the entry's own two: obs-plan §9 Telemetry artifact handling, the Criterion bench JSON row (`obs-plan.md:501`) and
    §9 Pipeline integration, the `xtask bench` row (`:510`); test-plan §9 Test report format (`test-plan.md:505`) with
    its sibling, test-plan §3 `run` Output format
    (`.andromeda/registries/contracts/test-plan/5-command-implementation.md:12`).
  - the riders': obs-plan §9 Log file row (`:499`, the `logs-${{ runner.os }}` clause), §9 Pipeline integration
    (`:508`, `:509`, `:511`), §9 CI failure → artifact triage workflow (`:522-526`); a11y-plan §3 CI integration
    (`.andromeda/registries/contracts/a11y-plan/ci-integration.md:3`, `:12`), test-plan §9's A11y suite row
    (`test-plan.md:493`) and test-plan §1's a11y tier row (`:56`).
  - found at P3, not named by the entry: six more obs-plan sentences state a criterion bench under `xtask benches/`
    (`obs-plan.md:125`, `:148`, `:291`, `:544`, `:545`, `:559`), and no such directory, dependency or verb exists.

## Read at take-up and at P3, not stated by the entry
Places in `ci.yml` and its tools that pass while reading nothing, of a different shape from a baseline comparison or
an upload. P-128's requirement names comparisons and uploads; its acceptance closes "No gate stands while reading
nothing." Whether that sentence reaches these is the operator's at P4. They are not work of this chunk unless the
operator says so.
- the coverage thresholds step exits 0 when `lcov.info` tracks 0 lines and 0 functions (`ci.yml:565-568`);
- the same step's branch arm reads `Branch: 0/0 = 100.0% (threshold 70%)` on `ci#38031822696` and on
  `ci#38026637514` attempt 1: the uploaded `lcov.info` carries no branch count, so the 70 % branch gate passes over
  nothing while the job's name states it;
- `--no-tests=pass` on the mcp-server test step (`ci.yml:237`) and inside `cargo xtask test` and
  `cargo xtask test:coverage` (`xtask/src/main.rs:466`, `:495`);
- `cargo xtask ci-gates` in the `boot` job: `perf-budget NEUTRAL` (memory and snapshot arms read no sample; stated
  in obs-plan §10 as designed) and `heartbeat-gap-check: max gap 0ms in  (threshold 45000ms) PASS` over a boot that
  lives about five seconds;
- obs-plan §10's "build fails if `xtask test` produces zero spans" has no step in `lint-test`; the zero-span check
  runs only inside `ci-gates`, in the `boot` job;
- the two regression scripts' NEUTRAL arms (they go with their comparison if it goes) and the regression-detector's
  "baseline not found; treating as empty baseline" arm.

## Decided at P4 (the operator's answers, the pc overseer, 2026-10-10 — inputs#I2)
- **Criterion: gone whole.** The upload, the download with its soft-fail key, the comparison step, the
  `criterion-regression` verb with its two scripts, and their pins. No bench suite exists in the tree, as measured
  (inputs#I2 item 1).
- **The `logs-Linux` and `nextest-Linux` uploads: gone.** No JUnit producer is added: "an artifact nothing reads is
  the same defect as a step that reads nothing" (inputs#I2 item 1).
- **Coverage comparison: gone whole; the absolute thresholds stay** (inputs#I2 item 2). The download with its
  soft-fail key, the comparison step, the `coverage-regression` verb with its two scripts, and their pins.
- **A11y: the download and its soft-fail key go; the comparison stays, reading the baseline committed in the tree**
  (inputs#I2 item 3). The operator's reading: a baseline committed in the tree at the commit under test has a producer,
  the tree, and the comparison reads it. A pin holds that the comparison fails, and does not pass, when that file is
  absent.
- **The closing sentence reaches the five other gates, so P-128 is not claimed here** (inputs#I2 item 4). This chunk
  settles every comparison and every upload and claims nothing. At its wrap the entry that closes the sentence is
  minted first in the tail, carrying the five gates each with its measured reading, and P-128 is claimed there.
  Research is not reopened for them; what was measured is recorded.
- The six uploads that stay each fail their step when they find no file; the guard is widened in this chunk, outside
  any capability claim. Both are the plan's own decisions, shown on the P5 card.

## Capability
- **P-128 is this entry's capability, advanced and not claimed.** Its acceptance: "On a CI run of the tree, every
  comparison against a baseline reads a baseline that a workflow produced, or the workflow no longer makes that
  comparison; every upload uploads a file, or the workflow no longer makes that upload. No gate stands while reading
  nothing." This chunk makes the first two clauses true; the third stands false while the five gates above stand, so
  the cap stays pooled with a dated note (inputs#I2 item 4). The note carries the operator's concretization for the
  later claim: "produced by a workflow or committed in the tree", the acceptance's wording having been narrower than
  the requirement line (inputs#I2 item 3).
- P-128 is PROVISIONAL until the founder's own word (the ledger's notes; the handoff).

## Boundaries
- **"A CI job" is a job of a workflow that a push or a pull request of the tree triggers.** Verified at P3 (each
  file's `on:` block): `ci.yml` and `secret-scan.yml` run on `pull_request` and on a push to `main`;
  `secret-scan.yml` holds no artifact step; `release.yml` runs on a `v*` tag push and `update-channels.yml` on the
  completion of `release`. Those two hold five uploads with `if-no-files-found: ignore` and three downloads; they
  leave at `Desktop distribution retired` (`working-route.md:50`, P-114, P-127), whose card names the release
  environment and its secret names for the founder. They are outside this chunk; that reading is said at P4.
- **The `a11y` job and the `boot` job stay.** They leave at `Window's gates retired` (`working-route.md:46`, P-083).
  This chunk settles only the `a11y` job's baseline download, the state of its comparison, and the upload keys; the
  job's suites, the boot smoke, its series and the `logs-boot` upload's content are not this chunk's to change. The
  boot job's `Boot series (equal source)` step stays a gating step, and a self-end in it is a new reading, brought to
  the operator (the handoff; `working-route.md:46`, second `CARRY:`).
- **No producer that needs a new permission, a token, a new trigger or a third-party action without the founder's
  own word** (inputs#I1 item 3). The workflow's `permissions: contents: read` (`ci.yml:8-9`) stays.
- **No new requirement is minted by phase.** If the guard needs its own requirement line, the plan says so and the
  operator's requirement-add path mints it.
- No run binds 4317 or 4318 without the operator's word (the handoff's host note); the chunk's own edits need none.

## Folded freight (the entry's two blocks)
`route.py pins` lists two blocks on `working-route.md:23`, both `CARRY:` (722 and 871 characters); both are folded
above, whole.
- First `CARRY:` (the measured ground, the two riders, the two masters' sentences): folded under "What the chunk
  builds", first, second, fourth and last bullets.
- Second `CARRY:` (the self-lint guard): folded under "What the chunk builds", the guard bullet.
- No `PREREQ:`, no `WATCH:`, no `BLOCKED-ON:`.

## Second fold source — the CI verdict read at Setup 5a
The base is the last commit that flipped a master record, `6df9e95c`, which is HEAD: one sha, read through
`ci.py conclusion`.
- `6df9e95c7dd3` (HEAD, the last wrap's commit): read `in progress` at Setup (`ci#38031822696`), and read again at P3
  once it had closed: **green** · 7 of 7 checks · wall 1071 s (`ci#38031822696`, `secret-scan#38031822695`, both
  pull-request runs). It is the newest run, the one inputs#I1 item 1 asks every comparison and upload to be re-read
  on; the readings above are from it.
- No red and no `not green` was read, so nothing is dispositioned here.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py blocked`: 0 blocks on a pending, gated or markerless line).
