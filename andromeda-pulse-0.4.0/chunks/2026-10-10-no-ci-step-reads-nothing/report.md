# Report — 2026-10-10-no-ci-step-reads-nothing

**Chunk:** No CI step reads nothing: every comparison has a producer for its baseline or is gone; every upload finds
its file or is gone (P-128, advanced and not claimed)
**Date:** 2026-10-10
**Commits:** `3049966f` `chore(2026-10-10-no-ci-step-reads-nothing): operator pre-CI commit` · `2ec4d439`
`chore(2026-10-10-no-ci-step-reads-nothing): record main as merged, tree unchanged` (a merge commit; through it
`main`'s `5b307e4c` `fix(ci): cargo audit runs as a plain step` and its merge `178ebac5` are reachable from HEAD).
Basis: `git log --format='%h %s' 6df9e95c..HEAD`, 4 rows.

## Changes (structured — detectors read this)
- **Files:** nine, exactly research's list (`gate.py scope`: `clean — changed 9 · listed 9 · recorded 0`):
  `.github/workflows/ci.yml` · `pulse-app/tests/quality_gate_workflow.rs` · `pulse-app/tests/a11y_perf_workflow.rs` ·
  `pulse-app/ui/tests-a11y/regression-detector.mjs` · `xtask/src/main.rs` · and four deleted:
  `xtask/ci/coverage-regression-check.sh` · `xtask/ci/coverage-regression-check.ps1` ·
  `xtask/ci/criterion-regression-check.sh` · `xtask/ci/criterion-regression-check.ps1`. 179 insertions, 605 deletions
  (`git diff --stat 6df9e95c -- .github pulse-app xtask`). The merge commit changed no file (its tree is the pre-CI
  commit's, `b4614705`).
- **Symbols / APIs:**
  - Removed from `xtask/src/main.rs`: the clap variants `Cmd::CoverageRegression` (verb `coverage-regression`) and
    `Cmd::CriterionRegression` (verb `criterion-regression`), their two dispatch arms, and the four functions
    `invoke_coverage_regression_check`, `run_coverage_regression`, `invoke_criterion_regression_check`,
    `run_criterion_regression`. Each `run_…` had one caller, `main`; each `invoke_…` one caller, its `run_…`
    (research.md, the code-graph read). No other symbol of the file changed. `cargo xtask --help | grep -c -E
    'criterion-regression|coverage-regression'`: 0.
  - New in `pulse-app/tests/quality_gate_workflow.rs`, all test-only: the const `STEP_NAME_PREFIX` (`:276`, read by
    `grep -n`), `fn workflow_step_blocks` (`278-292`), `fn artifact_step_name` (`294-306`),
    `fn uploaded_artifact_names` (`308-330`), the test `ci_workflow_makes_no_download_without_a_producer` (`332-351`),
    the test `ci_workflow_uploads_fail_when_they_find_no_file` (its body `356-373`; its `fn` line `:354`, read by
    `grep -n`), and the guard's second arm inside `ci_workflow_test_gates_no_continue_on_error` (`265-273`).
  - Removed from that file: the tests `ci_workflow_invokes_coverage_regression_check`,
    `ci_workflow_downloads_coverage_baseline_artifact`, `ci_workflow_invokes_criterion_regression_check`,
    `ci_workflow_downloads_criterion_baseline_artifact`, `ci_workflow_uploads_criterion_artifact_unchanged`.
    `ci_workflow_uploads_logs_artifact_unchanged` keeps its name and its `logs-boot-${{ runner.os }}` assertion and
    lost its `logs-${{ runner.os }}` one.
  - New in `pulse-app/tests/a11y_perf_workflow.rs`: the const `A11Y_BASELINE` and the tests
    `a11y_regression_baseline_is_committed_in_the_tree` (`99-114`) and
    `a11y_regression_detector_fails_when_its_baseline_is_absent` (`116-143`).
  - No IPC method, endpoint, event, port, socket, env var or TauRPC procedure added or removed. The bindings are
    byte-identical to `6df9e95c` (`git diff --quiet 6df9e95c -- pulse-app/ui/src/bindings/index.ts`: exit 0).
- **Crates / modules:** none added or removed. `xtask` lost two verbs; `xtask/ci/` holds six scripts where it held
  ten (`ls xtask/ci`: `heartbeat-gap-check.{sh,ps1}`, `l4-latency-p99.{sh,ps1}`, `quarantine-tracking-check.{sh,ps1}`,
  and `fixtures/`).
- **Dependencies:** none added, none bumped. No manifest and no lockfile changed.
- **Schema / config:** `ci.yml`: on each of the six uploads that stay, `if-no-files-found: ignore` became
  `if-no-files-found: error` (added lines `132 · 141 · 267 · 276 · 372 · 541`). No upload's `name:`, `path:`, `if:` or
  retention changed. The a11y regression detector's contract changed: an absent baseline file now prints
  `regression-detector: baseline not found at {path}` and exits 1 (added lines `63-64 · 68`); before, it printed
  `…; treating as empty baseline` and compared against an empty set. A present baseline is read as before. No config
  key, no migration, no violation schema, no scrub shape.
- **Spec-master edits:** none before this wrap.
- **Counts / qualifiers moved** (basis for the workflow counts: `yaml.safe_load` over `git show {rev}:.github/workflows/ci.yml`):
  - `ci.yml` steps 100 → 92; `run:` steps 53 → 51; `uses:` steps 47 → 41; lines 599 → 542.
  - upload steps 9 → 6; download steps 3 → 0; `continue-on-error` keys 3 → 0; uploads carrying
    `if-no-files-found: error` 0 → 6.
  - the soft-fail guard's command list 16 → 14 entries (`cargo xtask coverage-regression` and
    `cargo xtask criterion-regression` left); the step blocks it reads: 17 of 100 → every block, 92 of 92.
  - the two workflow pin binaries: 37 tests → 36 (4 added, 5 deleted). Basis: the red-before run, taken with the
    four new tests written and none deleted, printed `Starting 41 tests across 2 binaries`; the run after step 4
    printed `Starting 36 tests across 2 binaries`. Workspace run: `2890 tests run: 2890 passed, 0 skipped`.
  - `xtask` verbs: two fewer. Docs stating either verb: `grep -E 'coverage-regression|criterion-regression'` over the
    seven masters and `.andromeda/registries/`: 0 hits.
  - a new host need for the workspace test run: `node` on PATH (the detector pin runs it; a missing `node` fails the
    pin, it never skips).
- **Dev-tool versions:** none — `node` re-read at v26.8.2 (the dev host's default) and 24.21.0 (the `mise` install
  the pre-push check runs under); the CI image's toolchain was not changed by this chunk.
- **Harness / gate surface:**
  - CI steps removed, eight, by name. Job `lint-test`: `Upload logs artifact`, `Upload nextest junit artifact`,
    `Upload criterion bench artifact (deferred; non-blocking)`, `Download base-branch criterion baseline (PR only)`,
    `cargo xtask criterion-regression (chunk #56 — mean.point_estimate +10% gate vs base branch)`. Job `a11y`:
    `Download base-branch a11y-violations baseline (PR only)`. Job `coverage`:
    `Download base-branch coverage baseline (PR only)`,
    `cargo xtask coverage-regression (chunk #55 — line/branch/function gate vs base branch)`. No step was added; no
    other step moved; the job set, the trigger block, the workflow `env` and `permissions: contents: read` are equal
    to the base (the gate entry reading `8 True True True`).
  - The six uploads that stay, by artifact name: `logs-perf-samples-${{ runner.os }}`,
    `capability-drift-${{ runner.os }}`, `a11y-violations-${{ runner.os }}`, `playwright-a11y-report-${{ runner.os }}`,
    `logs-boot-${{ runner.os }}`, `coverage-linux`. Each now fails its step when it finds no file. The `lint-test`
    job uploads no log family except the perf-samples one; the `boot` job's `logs-boot` upload is unchanged.
  - The a11y job downloads nothing. Its comparison (the regression leg of `cargo xtask test:a11y`) reads the baseline
    committed in the tree, `pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json`, and fails when that file
    is absent.
  - The coverage job's absolute thresholds step is unchanged; the comparison against a base-branch `lcov.info` is
    gone.
  - `xtask` verbs removed: `coverage-regression`, `criterion-regression`; their four scripts deleted.
  - Workflow self-lint: the guard `ci_workflow_test_gates_no_continue_on_error` refuses `continue-on-error` on every
    step block of `ci.yml` (its listed-gate arm keeps 14 commands, `cargo audit` among them); two new pins hold that
    every `actions/download-artifact` step of `ci.yml` names an artifact some `actions/upload-artifact` step under
    `.github/workflows/` uploads under the same name, and that every upload step of `ci.yml` carries
    `if-no-files-found: error`; two new pins hold the a11y baseline's presence and the detector's failing arm.
  - Not touched, by the plan's constraint and the operator's word (inputs#I2 item 4): the coverage thresholds step,
    every `--no-tests=pass`, `cargo xtask ci-gates`, the heartbeat script, the boot job's smoke, exit-witness and
    series steps and their pins, `release.yml`, `update-channels.yml`, `secret-scan.yml`.
- **Cross-project / external claims:**
  - **The verdict run:** `ci#38035474359` on `2ec4d439` (pull-request event, attempt 1): `success`, 7 of 7 checks
    with `secret-scan#38035474342`, wall 1023 s (`ci.py conclusion`). Six jobs, six `success`. Its artifact list is
    exactly `a11y-violations-Linux capability-drift-Linux coverage-linux logs-boot-Linux logs-perf-samples-Linux
    playwright-a11y-report-Linux`; across its six job logs the count of lines saying an upload found no file, a
    download found no artifact or a comparison script ran is 0 (`6 0`; control on `ci#38031822696`: `6 8`); the
    a11y log holds `regression-detector: no new violations vs baseline` once; the `lint / test` log holds
    `perf-budget: graded` once. Recorded in `evidence/operator-pass.md`.
  - **The red run:** `ci#38034700885` on the pre-CI commit `3049966f` (pull-request event, attempt 1; its logs name
    the merge commit `e1a76573`): `failure`. `lint / test`, `mcp-server tests` and `coverage gate` each failed on
    ``error[E0428]: the name `ci_workflow_audit_step_is_a_plain_run_step` is defined multiple times``
    (`pulse-app/tests/quality_gate_workflow.rs:420:1`, previous definition `:377`); `a11y`, `supply-chain` and
    `boot smoke` passed. On that run three uploads failed their step with `No files were found with the provided
    path` (`logs-perf-samples`, `capability-drift`, `coverage-linux`), each after its producer did not run.
  - **`main`:** `178ebac5` (local `origin/main` and the remote equal, read through `gh api …/branches/main`). Past
    the former merge base `60ef43c6` it holds `5b307e4c` and its merge `178ebac5`, two files
    (`.github/workflows/ci.yml` +1 −3, `pulse-app/tests/quality_gate_workflow.rs` +44); all 45 added lines are present
    in `3049966f` (three hunks whole). After the merge commit `git merge-base HEAD origin/main` reads `178ebac5`.
  - **The draft pull request #40** read `MERGEABLE` · `UNSTABLE` · draft at `2ec4d439` while its run was open; not
    re-read after the run closed.
  - Phase's readings stand as research.md states them: `ci#38031822696` on `6df9e95c` and `ci#38026637514` on
    `4e61553b`; `actions/upload-artifact`'s `action.yml` at the pinned sha
    (`if-no-files-found`: "error: Fail the action with an error message").
  - Inputs, from `inputs.py verify` (`inputs: 4 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered
    0 · unreachable 0 · n/a 2 · uncited 2 · unparsed 0`, fired before this report existed):
    - `I1 · ../additional/pc-overseer/relays/pulse-phase-no-ci-step-reads-nothing-2026-10-10.md · copy no-repo ·
      unchanged` — cited by scope, research and plan.
    - `I2 · message: the operator (the pc overseer), in this session at the P4 dialog, 2026-10-10 · copy message ·
      n/a — a message has no live source` — cited by scope, research and plan.
    - `I3 · ../additional/pc-overseer/relays/pulse-wrap-no-ci-step-reads-nothing-2026-10-10.md · copy no-repo ·
      unchanged` — snapped at this wrap; cited here as inputs#I3 (the operator's wrap directive: what the closing
      entry carries, the two carries consumed, the masters corrected as measured, both CI runs in the report, the
      pre-push and merge-ref gap for disposition, the two stops).
    - `I4 · message: the operator (the pc overseer), in this session, 2026-10-10 · copy message · n/a` — snapped at
      this wrap; cited here as inputs#I4 (the operator's words at the operator pass and at this wrap's invocation).
    No entry drifted, vanished or broke.
- **Reverted / negative API facts:**
  - A remedy for the red run was prepared in the working tree and dropped on the operator's word (inputs#I4, the
    second word), never committed: the two new pins and their helpers moved to the end of
    `quality_gate_workflow.rs`, and the guard's every-step arm folded into its one loop above the listed-gate arm.
    With it the in-memory merge with `main` equalled the working tree. The operator's reading: it stepped around the
    seam and left it open for the next edit there. The pins stand where the plan has them.
  - No JUnit producer was added for the `nextest-Linux` upload, no baseline source for the coverage comparison, and
    no bench for the criterion comparison (inputs#I2 items 1 and 2): each step left instead.
  - The detector's "an absent baseline is an empty baseline" arm is removed, not kept behind the new exit.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **obs-plan — a criterion bench under `xtask benches/`.** Stated at `obs-plan.md:125` (§1), `:148` (§2), `:291`
     (§5), `:501` (§9 Telemetry artifact handling, the Criterion bench JSON row), `:510` (§9 Pipeline integration,
     the `xtask bench` row), `:544` and `:545` (§10 Performance budgets) and `:559` (§10 CI gates, "the `xtask bench`
     half"). Measured by phase (research.md, "What a producer would cost"): no step runs a bench; `ls xtask/benches`
     finds no directory; `grep -rn criterion --include=Cargo.toml .` finds no dependency line; `xtask` has no `bench`
     verb; on `ci#38031822696` the `criterion-Linux` upload found no file, the `criterion-Linux-base` download found
     no artifact and the comparison printed `current dir target/criterion not found; NEUTRAL`. The upload, the
     download and the comparison are now gone. Search: `criterion` over the masters and registries — obs-plan 7 hits
     (the seven lines above but `:559`, which reads "Criterion"; `[Cc]riterion|xtask bench|benches/`: 8 lines),
     test-plan 1 hit (`:194`, "acceptance criterion", another word), a11y-plan 4 hits and two a11y key files 1 hit
     each (all `wcag_criterion` or "WCAG criterion", another word).
  2. **test-plan — a JUnit XML report from nextest, read by a reporter action.** Stated at `test-plan.md:505` (§9
     Test report format: "JUnit XML via `cargo nextest --message-format junit --output-file target/nextest/junit.xml`;
     agent reads via `dorny/test-reporter`") and in the §3 key file
     `.andromeda/registries/contracts/test-plan/5-command-implementation.md:12` (`run` → Output format). Measured by
     phase: `.config/nextest.toml` holds no `junit` key; `cargo xtask test` asks nextest for `libtest-json`; no
     workflow uses a reporter action; on `ci#38031822696` the `nextest-Linux` upload found no file. The upload is now
     gone. Search: `[Jj][Uu]nit|test-reporter` — test-plan 3 hits (`:505`; `:55` and `:190` state the webview suite's
     own `target/junit-ui.xml`, which vitest does write: no change), the key file 1 hit (`:12`).
  3. **obs-plan §9 Telemetry artifact handling, the Log file row** (`obs-plan.md:499`): "the `lint-test` job
     (`logs-${{ runner.os }}` …)". Measured: on `ci#38031822696` that upload printed `No files were found with the
     provided path` (no step of `lint-test` writes a log family under the job's data dir); the step is now gone. The
     row's `boot` half and its perf-samples half stand.
  4. **obs-plan §9 Pipeline integration** (`obs-plan.md:508`, `:509`, `:510`, `:511`): `xtask test` and
     `xtask test --release` produce a "CI artifact `logs/agent-latest.jsonl`"; `xtask bench` a criterion artifact;
     "Integration tests (P1-P7)" a "logs artifact + snapshot markdown artifact". Measured on the workflow: no step
     runs `xtask test --release` or `xtask bench`; the nine upload names of `ci.yml` at `6df9e95c` held no snapshot
     artifact and the one `lint-test` log upload found no file; the six names that stay are listed above.
  5. **obs-plan §9 CI failure → artifact triage workflow** (`obs-plan.md:522-526`, `:524`): "Agent downloads
     artifact via `gh run download <run_id> -n logs`". Measured: no artifact of a run is named `logs`; the two log
     artifacts are `logs-boot-Linux` and `logs-perf-samples-Linux`.
  6. **obs-plan §9 Telemetry artifact handling, the Snapshot markdown row** (`obs-plan.md:500`): "Snapshot markdown
     (on test failure) … uploaded as CI artifact". Found at this wrap while reading the rows beside it, not named by
     the plan: no upload step of `ci.yml`, at the base or now, has a snapshot path (the nine names at `6df9e95c`, the
     six now). Whether the version still needs that reading is the operator's at the card (inputs#I3 item 5).
  7. **a11y-plan — a base-branch baseline downloaded as an artifact.** Stated in the §3 key file
     `.andromeda/registries/contracts/a11y-plan/ci-integration.md:3` ("the PR-only `a11y-violations-base` download")
     and `:12` ("Download base-branch `a11y-violations-summary.json` via `actions/download-artifact@v4` (name
     `a11y-violations-base`)"), at `a11y-plan.md:466` ("PR comment with new violations vs base branch"), `:494` (§10,
     "Per-PR diff detection via artifact comparison") and `:565` ("vs base branch"), and at `test-plan.md:493` (§9
     Pipeline structure, the A11y suite row: "the PR-only `a11y-violations-base` download" and its host-needs cell).
     Measured: no workflow uploads that name; on `ci#38031822696` the download ended `Artifact not found`; the
     comparison read the baseline committed in the tree all along (`regression-detector.mjs:9`) and printed `no new
     violations vs baseline (0/0)`. The download is now gone; the comparison reads the tree baseline and fails when
     it is absent. The §3 key file `harness-wiring-conventions.md:6` (Regression baseline) already names the tree
     file and says nothing of an absent one. The "PR comment" of `a11y-plan.md:466` has no producer either, read at
     this wrap during Validate: `grep -n -i -E 'comment|pull-requests|issues:|github-script|GITHUB_TOKEN'
     .github/workflows/ci.yml` prints one line, a YAML comment about source comments (`:79`), and the workflow's
     permissions are `contents: read`; no step posts to a pull request.
  8. **The route's carry on `Window's gates retired`** (`working-route.md:46`, first `CARRY:`): "so its regression
     comparison reads no baseline". False in that half, as phase measured and scope.md records as
     premise-corrected: the download read nothing, the comparison read the baseline held in the tree. A route
     annotation, corrected at P5.
  9. **The plan's implementation note** (plan.md, "What the upload key proves, and its limit"): "No run of this
     chunk shows a step failing on an empty upload". The red run `ci#38034700885` shows three upload steps failing
     on no file. The note's limit holds in a narrower form: no run shows that arm with a producer that ran and wrote
     nothing. A chunk-artifact claim: this entry, no edit.
  10. **The plan's forecast for step 7's fourth mutation** (plan.md step 7 and the a11y acceptance line: red when
      "the detector's new exit" is removed). Measured green in that form and red when the block is put back in its
      base form (`evidence/mutation-checks.md` 4a, 4b). A chunk-artifact claim: this entry, no edit.
  11. **"A coverage baseline artifact" in a master** (inputs#I3 item 5 names it among the things the masters state).
      Searched: `coverage-regression|coverage regression|lcov-baseline|coverage-linux-base|coverage baseline|
      no-decrease` over the seven masters and `.andromeda/registries/`: 0 hits. No master states it; the claim lived
      in the workflow, the two deleted scripts and the deleted pins. Nothing to correct in a master.
- **Expected amendments (from plan):** nine entries.
  - *obs-plan §9 Telemetry artifact handling* — carried: Spec claims disproved 1 and 3, and Schema / config (each
    upload that stays fails its step on no file). Sites: `logs-\$\{\{ runner\.os \}\}`: obs-plan 1 hit in this
    section (`:499`); `[Cc]riterion`: `:501`; `if-no-files-found`: 0 hits in any master or key file (the key's
    behaviour is stated nowhere yet).
  - *obs-plan §9 Pipeline integration, and §9 CI failure → artifact triage workflow* — carried: Spec claims
    disproved 4 and 5. Sites: obs-plan `:508`, `:509`, `:510`, `:511` (read at the lines) and `-n logs`: 1 hit
    (`:524`).
  - *obs-plan §10 Performance budgets and CI gates, with §1, §2, §5* — carried: Spec claims disproved 1. Sites:
    `xtask bench|benches/`: obs-plan 7 hits (`:125`, `:148`, `:291`, `:501`, `:510`, `:544`, `:559`); `criterion`
    adds `:545`.
  - *test-plan §9 Test report format and §3 `run` → Output format* — carried: Spec claims disproved 2. Sites:
    test-plan `:505`; the key file `registries/contracts/test-plan/5-command-implementation.md:12` (counted for
    test-plan).
  - *test-plan §9 Pipeline structure and §1's a11y tier row* — carried: Spec claims disproved 7 (the A11y suite row,
    `test-plan.md:493`), Counts / qualifiers moved (the `node` host need for the workspace run; the guard reads
    every step) and Harness / gate surface. Sites: `a11y-violations-base|base-branch`: test-plan 1 hit (`:493`);
    `no_continue_on_error`: test-plan 1 hit (`:494`, the Supply chain row, "the `cargo audit` entry of
    `ci_workflow_test_gates_no_continue_on_error`" — still true: `cargo audit` is in the 14); `continue-on-error`:
    test-plan `:496` (the Boot smoke row, its own step pins: unchanged); `regression-detector`: test-plan `:56` (§1,
    "the regression-detector's zero-new-vs-baseline diff") and `:493`.
  - *a11y-plan §3 CI integration, §3 Harness wiring & conventions, §10 Standard+ invariants, §1's CI Integration
    list* — carried: Spec claims disproved 7 and Schema / config (an absent baseline is an error). Sites:
    `a11y-violations-base|base-branch|base branch|artifact comparison`: a11y-plan 3 hits (`:466`, `:494`, `:565`),
    the key file `ci-integration.md` 2 hits (`:3`, `:12`); `[Rr]egression baseline`: the key file
    `harness-wiring-conventions.md` 1 hit (`:6`) and its label in `registries/a11y-plan-contracts.toml:49`.
  - *architecture §Infrastructure Patterns → CI/CD approach* — not carried: the key file
    `registries/contracts/architecture/ci-cd-approach.md` states none of the eight removed steps (`criterion|
    regression|download|junit|baseline` over it: 0 hits; its `lint-test` clause names the perf-samples upload only,
    its `coverage` clause is the job's bare name). `architecture.md:259` holds `continue-on-error` once, about the
    boot smoke step's history: unchanged by this chunk.
  - *security-plan §Dependency Security → CI integration* — not carried, none expected: `security-plan.md:216`
    states the audit step's own shape pinned by `ci_workflow_audit_step_is_a_plain_run_step`, which stands; the
    guard keeps `cargo audit` in its list and gained an arm.
  - *Ledger: a dated note on `verification-matrix.json#P-128`* — written at P5 by phase, not at this wrap (plan.md,
    last implementation note); it rode the pre-CI commit. Not a P7.3 write.
- **Coverage of new surfaces** (no new external surface, hot-path operation or UI element; the changed surfaces are
  CI and test harness):
  - `regression-detector.mjs`, the absent-baseline exit → validation n/a · instrumentation n/a (a harness script;
    it prints one stderr line with the path it was given) · PII n/a · tests unit ✓
    (`a11y_regression_detector_fails_when_its_baseline_is_absent`, red before, red under the base-form mutation) ·
    a11y n/a (no rendered surface) · tokens n/a.
  - `ci.yml`, six uploads failing on no file → validation n/a · instrumentation n/a · PII n/a (no `path:` changed,
    so no artifact gained a member) · tests unit ✓ (`ci_workflow_uploads_fail_when_they_find_no_file`) and one
    runner reading of the failing arm (the red run) · a11y n/a · tokens n/a.
  - `ci.yml`, no download and no soft-fail key → tests unit ✓ (`ci_workflow_makes_no_download_without_a_producer`,
    the guard's every-step arm) · the rest n/a.

## Deviations from intent
- **Plan step 7, the fourth mutation.** As worded (the detector's new exit removed) it leaves the pin green: step 5
  was written without the empty-baseline read arm, so with the exit gone the read of the absent file throws and the
  script still exits 1. A second form, the block put back as it stands at `6df9e95c`, reads red. Both readings are
  in `evidence/mutation-checks.md`. Justification: keeping the arm would be dead code after the exit; accepted by
  the operator (inputs#I4, the first word: "no dead arm is kept for a plan sentence").
- **The detector pin asserts more than the plan asked.** Beside the non-zero exit it asserts stderr holds
  `baseline not found`, so a failure for another reason (a missing script) cannot pass it.
- **Mutation 1 turned the guard red as well as the download pin**, because the download step was restored whole,
  with its `continue-on-error` key.
- **The operator pass went past the plan's order.** The plan lists one pre-CI commit, one push and one CI read. The
  first run was red on the pull-request merge, so on the operator's word (inputs#I4) a merge commit followed
  (`2ec4d439`: `main` recorded as merged, strategy `ours`, the tree unchanged, no force and no rebase), then the
  pre-push check a third time, a second push and a second CI read. The acceptance line "on the pull-request run of
  the pre-CI commit … `verdict: green`" is met on the run of `2ec4d439`, whose tree is the pre-CI commit's; the
  pre-CI commit's own run is red and stays recorded. No run was re-run to obtain a green.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 9 · listed 9 · recorded 0
  (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 52`, base `6df9e95c`).

## Decisions & corrections
- **The operator, at the P4 dialog (inputs#I2):** criterion, the coverage comparison and the three empty uploads are
  gone whole; the a11y download is gone and its comparison stays on the tree baseline with a fails-when-absent pin;
  a baseline "committed in the tree at the commit under test has a producer, the tree"; P-128's closing sentence
  reaches five other gates, so the chunk claims nothing.
- **The operator, after the implement report (inputs#I4):** deviations accepted as recorded; "no dead arm is kept
  for a plan sentence".
- **The operator, after the red run (inputs#I4):** "the red is this chunk own and its cause is the seam, not the pin
  placement … Close the seam instead of stepping around it." `main` is recorded as merged with the tip's tree
  unchanged; the second run is the chunk's verdict and the red one stays recorded.
- **The operator's wrap directive (inputs#I3):** the closing entry is minted first in the tail and owns P-128's
  claim; the two consumed carries are struck; the masters are corrected as measured, and a sentence that promised a
  reading the version still needs is said at the card; the pre-push and merge-ref gap is presented for disposition
  there (the operator's lean: a rule where the operator pass reads it, not an entry).
- **A pasted message is not the operator's word until the operator says so.** The second word arrived as pasted
  text; the session ran only its read-only check, asked, and acted on the answer.
- **Found, not owned:** the local pre-push check builds the tip; the pull-request run builds the tip merged with
  `main`. They were equal at `6df9e95c` and at `2ec4d439` and parted at `3049966f`. With `main` an ancestor they are
  equal until `main` moves.
- **Seen while locating, not measured:** two further `--no-tests=pass` literals in `xtask/src/main.rs`, in
  `run_perf_slo_load` and `run_perf_load_profiles` (`evidence/other-gates.md`, gate 3).
- **Sweep hazards found this chunk:**
  - A branch that carries by hand a change `main` also holds merges clean only while nothing is edited beside those
    hunks: git folds two identical insertions, and stops folding them when one side's hunk grows. The instrument
    that shows it without moving a ref: `git merge-tree --write-tree {tip} origin/main`, its tree compared with the
    tip's; for an uncommitted tree, `git merge-file -p ours base theirs` per file, or a tree written through a
    temporary `GIT_INDEX_FILE` with `--merge-base`.
  - A mutation that removes one line of a fix does not restore the defect when the fix holds the property in two
    places; the mutation that discriminates puts the base form back.
  - `grep '/tmp/'` over an evidence file hits `target/tmp/…`, a repository-relative path; hygiene's own predicate
    does not (a POSIX form after a word character is no match).
  - `criterion` as a search word hits "acceptance criterion" and `wcag_criterion`; the bench sentences are found by
    `[Cc]riterion|xtask bench|benches/` and read at the lines.
  - A leading `cd` into a subdirectory is refused by the Bash guard; a census over `.andromeda/` runs from the root
    by a script given the paths.

## Outcome
**Acceptance criteria, each against the diff:**
- (tests) no `actions/download-artifact` step; the download pin holds a later one to a producer — **met**: the gate
  entry `last line 92 6 6 0 0`; the pin red before and red under mutation 1.
- (obs) six uploads, the six names, each `if-no-files-found: error` — **met**: the two gate entries; the pin red
  before and under mutation 2.
- (arch) against `6df9e95c` exactly eight steps gone, every staying step equal but for the upload key, job set,
  trigger block, `env` and permissions equal — **met**: `last line 8 True True True`.
- (arch) six jobs on `ubuntu-22.04`, no `strategy`, no `needs:` — **met**: `last line a11y boot coverage lint-test
  mcp-test supply-chain` and `6 of 6`.
- (security) no step carries `continue-on-error`; the guard reads every step and keeps `cargo audit`;
  `ci_workflow_audit_step_is_a_plain_run_step` passes; the guard red under mutation 3 on a step its list never read
  — **met**.
- (security) the `ci.yml` diff adds no action reference, permission, secret reference, token or condition — **met**:
  the grep of the diff's added lines reads `last line 0`; no upload `path:` changed.
- (a11y) the comparison reads a baseline that has a producer, the tree, and fails when it is absent — **met**, with
  the mutation read as stated under Deviations (red in the base form, green in the plan's wording).
- (tests) `cargo xtask` names neither verb and `xtask/ci` holds no `regression-check` script — **met**: two entries
  reading `last line 0`.
- (tests) the standard gate set, the bindings equal to `6df9e95c`, `pre-push:linux` green — **met** at implement,
  and `pre-push:linux` green again on `3049966f` and on `2ec4d439`.
- (tests) the pull-request run: `verdict: green`, exactly six artifacts, `6 0`, the a11y line, the perf line —
  **met on `ci#38035474359`** (the run of `2ec4d439`, the operator's word that it is the chunk's verdict); **not
  met on the pre-CI commit's own run**, `ci#38034700885`, red on the merge seam.
- (tests) the mutation readings and the red-before reading are in `evidence/` — **met**.
- (obs) `evidence/other-gates.md` holds the five gates — **met**.
- P-128: no capability claimed; the ledger shows 0 claimed by this chunk.

**Gates, by `run`, as /implement ran them once (the whole block, first run) and the operator pass drove them:**
- `cargo fmt --check` — green (exit 0).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green (exit 0).
- `git diff --name-only 6df9e95c… -- crates pulse-app xtask scripts docs … ':!…'` (the scope guard) — green (exit 0,
  no output).
- the five `python -X utf8 -c "import yaml; …"` probes — green: `a11y boot coverage lint-test mcp-test
  supply-chain` · `6 of 6` · `92 6 6 0 0` · the six upload names · `8 True True True` with the two named steps.
- `git diff 6df9e95c… -- .github/workflows/ci.yml | grep -c -E '^\+.*(uses:|permissions:|…)'` — green (exit 1,
  `0`).
- `ls xtask/ci | grep -c 'regression-check'` — green (exit 1, `0`).
- `cargo xtask --help | grep -c -E 'criterion-regression|coverage-regression'` — green (exit 1, `0`).
- `cargo xtask check:english-sources` — green (`"verdict": "clean"`).
- `cargo xtask capability-widening-check` · `cargo xtask check:ingest-progress` ·
  `cargo xtask check:staged-artifacts` · `cargo xtask capability-drift` · `cargo xtask verify:capability-matrix` —
  green, each exit 0.
- `cargo nextest run --workspace --profile ci` — green (2890 run, 2890 passed).
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green.
- `git diff --quiet 6df9e95c… -- pulse-app/ui/src/bindings/index.ts` — green (exit 0).
- `npm run lint --prefix pulse-app/ui` · `npm run typecheck --prefix pulse-app/ui` ·
  `npm run test --prefix pulse-app/ui` — green.
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green at implement (85 s); green
  by hand on `3049966f` and on `2ec4d439` (`"verdict": "green"`, `"reason": "all-stages-ok"`, six stages `ok`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (operator) — green: `hygiene: clean`.
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` (operator) — green
  twice: `6df9e95c..3049966f`, then `3049966f..2ec4d439`, a fast-forward.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` (operator) —
  **red** on `3049966f` (`verdict: red`, `lint / test` first, `ci#38034700885`), this chunk's own, cause the merge
  seam, closed by `2ec4d439`; **green** on `2ec4d439` (`verdict: green · checks 7/7`, `ci#38035474359`).
- `gh api "…/actions/runs/38035474359/artifacts?per_page=100" --jq …` (operator) — green: the six names.
- the six-job-log count (operator) — green: `6 0`.
- the a11y job's comparison line (operator) — green: `1`.
- the `lint / test` job's `perf-budget: graded` line (operator) — green: `1`.
- `gh api "…/actions/runs/38035474359/jobs?per_page=100" --jq …` (operator, `expect = []`) — recorded: six jobs, six
  `success`; whose: this chunk's verdict run.
- Smoke: skipped — no boot-path or UI-surface change; the plan lists no smoke entry.

**Watches:** none folded (scope.md: no `WATCH:`).

**Outcome basis:** the operator pass ran. The verdicts above rest on its final state: the commits `3049966f` and
`2ec4d439` (Setup's list) and the final HEAD's CI run `ci#38035474359`, recorded in `evidence/operator-pass.md`.
Implement's report (in this session's conversation) is the basis for the gate block's first run, the red-before
reading and the mutation checks; the operator's directives between implement and this report are inputs#I4 (what
each changed is under Deviations); the wrap directive is inputs#I3.

**Limits, each a limit:** no run shows a comparison failing for an absent baseline on a runner (the detector pin
holds that arm locally); the upload key's failing arm was read on a runner only where a producer did not run, never
where one ran and wrote nothing; the a11y upload has three path members and the key fails the step only when none
matches; the local pre-push check does not build the pull-request merge.

**Process hygiene:** implement's census read `none left` (no `pulse-app`, `cargo`, `nextest`, detector `node` or
`Xvfb`; nothing listening on 4317 or 4318). The operator pass started `cargo xtask pre-push:linux` twice and
`ci.py conclusion` twice; each exited. Re-measured at the end of the pass: one `ci.py conclusion --sha HEAD --name …`
process stood, started from the `andromeda-phase` skill path — not this session's (this session's `ci.py` calls
carried no `--name`), left alone. This wrap started `scripts/code-graph.py refresh` in the background; it exited 0.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 6df9e95c (the parent of the oldest pre-CI commit 3049966f) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 6 line(s) in 6 range(s)
added: 132 · 141 · 267 · 276 · 372 · 541
### pulse-app/tests/a11y_perf_workflow.rs — added 48 line(s) in 1 range(s)
added: 97-144
- 99-114 @101 «fn a11y_regression_baseline_is_committed_in_the_tree() {»
  - 103-104 «let text = std::fs::read_to_string(&path)»
  - 105-106 «let baseline: serde_json::Value = serde_json::from_str(&text)»
  - 107-113 «assert!(»
- 116-143 @117 «fn a11y_regression_detector_fails_when_its_baseline_is_absent() {»
  - 122-130 «let output = std::process::Command::new("node")»
  - 132-135 «assert!(»
  - 136-142 «assert!(»
### pulse-app/tests/quality_gate_workflow.rs — added 122 line(s) in 6 range(s)
added: 1-9 · 249 · 251 · 254 · 265-373 · 452
  - 265-273 @267 «for block in &blocks {»
- 278-292 @279 «fn workflow_step_blocks(content: &str) -> Vec<&str> {»
  - 280-283 «let starts: Vec<usize> = content»
  - 284-291 «starts»
- 294-306 @296 «fn artifact_step_name<'a>(block: &'a str, action: &str) -> Option<&'a str> {»
  - 298-300 «if !block.lines().any(|l| l.trim_start().starts_with(&uses)) {»
  - 301-304 «let name = block»
- 308-330 «fn uploaded_artifact_names() -> Vec<String> {»
  - 311-312 «let entries =»
  - 313-328 «for entry in entries {»
- 332-351 @333 «fn ci_workflow_makes_no_download_without_a_producer() {»
  - 335-338 «assert!(»
  - 340-350 «for block in workflow_step_blocks(&content) {»
  - 356-359 «let uploads: Vec<&str> = workflow_step_blocks(&content)»
  - 360-363 «assert!(»
  - 364-373 «for block in uploads {»
### pulse-app/ui/tests-a11y/regression-detector.mjs — added 3 line(s) in 2 range(s)
added: 63-64 · 68
### xtask/ci/coverage-regression-check.ps1 — deleted
### xtask/ci/coverage-regression-check.sh — deleted
### xtask/ci/criterion-regression-check.ps1 — deleted
### xtask/ci/criterion-regression-check.sh — deleted
### xtask/src/main.rs — nothing added (lines removed only)
