# Fan-out results — 2026-10-10-no-ci-step-reads-nothing

Seven doc-agents, one batch. Each return was taken from the agent's own hand-back by a script, not retyped; the entity probe read 0 (`&lt;` · `&gt;` · `&amp;`) on all seven. 17 proposals: architecture 1 · test-plan 8 · obs-plan 8.

## architecture

**Verdict:** 1 proposal (D-arch-decisions). D-arch-resources: no violation. Nothing stripped.

The return, as it arrived:

````yaml
proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies"
    change: "Add one row beside the two harness-only host-tool rows (the L4 grammar conversion check's Python 3 interpreter, the boot-smoke exit witness's `cc`): Layer `A11y regression-detector pin (test-only)` · Technology `Node — `node` on PATH on every host that runs the workspace tests, running the existing `pulse-app/ui/tests-a11y/regression-detector.mjs`; no npm or Cargo dependency, no manifest or lockfile change (read at v26.8.2, the dev host's default, and 24.21.0, the `mise` install the pre-push check runs under; the CI image's toolchain not changed by the chunk)` · Role `the test `a11y_regression_detector_fails_when_its_baseline_is_absent` in `pulse-app/tests/a11y_perf_workflow.rs` spawns `node` on the detector with an absent baseline and asserts a non-zero exit and a stderr holding `baseline not found`; a missing `node` FAILS the test, never skips it. Never in the shipped binary (chunk 2026-10-10-no-ci-step-reads-nothing)`."
    sidecar: "2026-10-10-no-ci-step-reads-nothing — §Stack and Technologies gains the test-only row for `node` on PATH as a host need of the workspace test run (the a11y regression-detector pin; a missing `node` fails, never skips)."
    rationale: "The report's Changes, under Counts / qualifiers moved, state `a new host need for the workspace test run: `node` on PATH (the detector pin runs it; a missing `node` fails the pin, it never skips)`, and under Symbols / APIs the new test `a11y_regression_detector_fails_when_its_baseline_is_absent` (`99-114` its sibling, `116-143` itself). §Stack is where arch registers each non-Cargo interpreter or compiler the workspace tests need on the host — the Python 3 row says `on every host that runs the workspace tests … a missing interpreter FAILS the test, never skips it`, the exit-witness row says the xtask tests `FAIL on a missing `cc`` — and it states no such need for Node: Node appears in arch only as the GUI verification harness's npm devDependencies and as `cargo xtask pre-push:linux`'s `node:{pin}` probe and stage PATH. Node itself is not a new runtime to the stack and no locked decision is contradicted; the drift is the unregistered tier (a Rust workspace test now spawns it), which is why this is a warning. The report's Expected amendments route the same fact to test-plan §9 Pipeline structure and mark architecture `not carried` for the CI/CD approach key only; that key file was read whole and needs no change. Swept arch and its four key files for every other claim the chunk could retire — `coverage-regression|criterion-regression|criterion|regression-check|download-artifact|if-no-files-found|junit|nextest-Linux|a11y-violations|regression-detector|lcov`: 0 hits in each; `base-branch|PR-only|vs base`: 0 relevant hits; `xtask/ci/` names only `l4-latency-p99` (stays) and the already-deleted `perf-slo-check`; the one `continue-on-error` mention is the boot smoke step's history, as the report says of `architecture.md:259`. No duplicate occurrence, so no dependent proposal. D-arch-resources: no violation — the report states no IPC method, endpoint, event, port, socket, env var, TauRPC procedure or workspace crate added or removed, and neither removed xtask verb was registered in §Occupied Resources."
    basis: "pulse-app/tests/a11y_perf_workflow.rs:122-130"
````

**Dispositions:**
- 1 (§Stack and Technologies, a row for `node`) → **apply**. Check 1: playbook *Accurate this-chunk addition* (routine): the host need is the report's (Counts / qualifiers moved), and the section already registers two test-only host tools in that shape (the Python 3 row, the `cc` row). Its `basis`, `a11y_perf_workflow.rs:122-130`, is a row of the report's New text section. Checks 2, 3, 4: nothing against it; the return's own sweep of arch and its four key files found no other occurrence.

## security-plan

**Verdict:** proposals: [] — four detectors, none violated. The return carried YAML comments after the list: per detector why it did not fire, a sweep of the doc for the retired claims (0 hits), four sites that mention the touched surface and still hold (`:216`, `:395`, `:103`, `:224`), and one observation outside the detectors (the Python 3 sentence at `:224` is not exclusive, so `node` does not stale it).

## design-system

**Verdict:** proposals: [] — two detectors, none violated. The return carried YAML comments: no new UI element, every coverage row `tokens n/a`; no disproved claim or ruling touches a design surface; a sweep of the doc for the retired claims, 0 restating occurrences.

## layout-templates

**Verdict:** proposals: [] — two detectors, none violated. The return carried YAML comments before the list: the report adds no UI surface; no disproved claim or ruling touches a surface this doc states.

## test-plan

**Verdict:** 8 proposals, all under D-tests-framework (1, 2, 6, 7 on its runner wording; 3, 4, 5, 8 attached to the nearest detector because the report's Expected amendments names those sites — the return says so). D-tests-coverage and D-tests-obs-harness: no drift. Nothing stripped but three leading comment lines.

The return, as it arrived:

````yaml
# D-tests-coverage: no drift. Every new path in the report's Changes carries a unit-tier pin (Coverage of new surfaces: the detector's absent-baseline exit, the six uploads' failing arm, the no-download / no-soft-fail guard); the rest of the chunk is removal.
# D-tests-obs-harness: no drift. The report changes no harness verb, status shape or log format; the one §3 key edit proposed below (run -> Output format) is not one-sided: `[Jj]unit|test-reporter` has 0 hits in obs-plan.md and in registries/contracts/obs-plan/.
# All eight proposals ride D-tests-framework. Only 1, 2, 6, 7 fit its §2/§4 runner wording squarely; 3, 4, 5, 8 are §9 / §1 CI-surface facts with no dedicated test-plan detector, attached to the nearest one because the report's Expected amendments names those sites.
proposals:
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Test report format"
    change: >-
      Test report format: the Rust suite writes no JUnit XML and no reporter action reads one — `cargo xtask test` asks nextest for `libtest-json`, `.config/nextest.toml` holds no `junit` key, no workflow uses `dorny/test-reporter`, and the `Upload nextest junit artifact` step left `ci.yml` at chunk 2026-10-10-no-ci-step-reads-nothing (it found no file on `ci#38031822696`); the agent reads the exit code and the nextest output. The webview suite's own `target/junit-ui.xml` (vitest) stands as §1 and §4 state it.
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §9 Test report format: the nextest JUnit XML + `dorny/test-reporter` sentence retired as measured (no `junit` key, `libtest-json` asked, no reporter action, the `nextest-Linux` upload found no file and its step is gone).
    rationale: >-
      Report, Spec claims disproved by measurement 2: the sentence at `test-plan.md:505` names a report file nothing writes and an action no workflow uses; Harness / gate surface lists `Upload nextest junit artifact` among the eight removed steps; Reverted / negative API facts: no JUnit producer was added. Sweep: `[Jj][Uu]nit|test-reporter` over test-plan reads 3 hits — `:505` (this one), and `:55` / `:190`, which state vitest's `target/junit-ui.xml` (written, per the report: no change, no proposal); the fourth site is the §3 key file, proposed next.
    basis: ".andromeda/test-plan.md:505"
  - detector: D-tests-framework
    severity: warning
    section: "§3 → 5-command implementation"
    change: >-
      `run` → Output format: nextest `--message-format libtest-json` (the form the same key's Command body already carries, teed to `target/test-run.log`) — no JUnit XML and no `target/nextest/junit.xml`; the rest of the label (the final-line regex) unchanged.
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §3 5-command implementation, `run` → Output format: JUnit XML retired as measured; the label now agrees with the key's own Command body (`libtest-json`).
    rationale: >-
      Same claim as the primary, restated in the keyed contract: report, Spec claims disproved 2 names `.andromeda/registries/contracts/test-plan/5-command-implementation.md:12` and Expected amendments lists it ("counted for test-plan"). Inside the key the Output format label also contradicts its own Command body, which asks for `libtest-json`. obs-plan §3 states no JUnit format (0 hits), so the edit leaves §3 ↔ obs-plan §3 in agreement.
    basis: ".andromeda/registries/contracts/test-plan/5-command-implementation.md:12"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline structure (the A11y suite row, Runs cell)"
    change: >-
      The `a11y` job's order reads `npm run build --prefix pulse-app/ui` → Playwright chromium → `cargo xtask test:a11y` with no download step (the PR-only `a11y-violations-base` download left at chunk 2026-10-10-no-ci-step-reads-nothing: no workflow uploads that name, and on `ci#38031822696` it ended `Artifact not found`); the regression leg reads the baseline committed in the tree, `pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json`, and on an absent file prints `regression-detector: baseline not found at {path}` and exits 1 — pinned by `a11y_regression_baseline_is_committed_in_the_tree` and `a11y_regression_detector_fails_when_its_baseline_is_absent` (`pulse-app/tests/a11y_perf_workflow.rs`); the two uploads (`a11y-violations-*` / `playwright-a11y-report-*`, `if: always()`) now carry `if-no-files-found: error`.
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §9 Pipeline structure, A11y suite row: the `a11y-violations-base` download removed from the job's order; the comparison named as reading the tree baseline and failing when it is absent; the row's two uploads fail on no file.
    rationale: >-
      Report, Spec claims disproved 7 (`test-plan.md:493`, "the PR-only `a11y-violations-base` download"): the download read nothing and is gone (Harness / gate surface: "The a11y job downloads nothing"); Schema / config: an absent baseline now exits 1 (`regression-detector.mjs` added lines `63-64 · 68`); the comparison read the tree baseline all along (`regression-detector.mjs:9`). Limit to keep with it: no run shows the absent-baseline arm on a runner — the pin holds it locally (Outcome, Limits).
    basis: ".andromeda/test-plan.md:493"
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline structure (the A11y suite row, host-needs / Cache cell)"
    change: >-
      The cell reads `node` + `npm ci` + `npm run build` (the harness serves `ui/dist`); the zero-new-tuple diff reads the baseline committed in the tree — nothing is downloaded, on a pull request or otherwise.
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §9 Pipeline structure, A11y suite row, host-needs cell: "a base-branch `a11y-violations-summary.json` is downloaded on PRs" retired; the diff reads the tree baseline.
    rationale: >-
      Second occurrence of the retired claim on the same row: report, Spec claims disproved 7 names "the A11y suite row … and its host-needs cell" at `test-plan.md:493`; Counts / qualifiers moved: download steps 3 → 0. Sweep `a11y-violations-base|base-branch` over test-plan: 1 line (`:493`), two cells — a single-cell apply would leave this one standing.
    basis: ".andromeda/test-plan.md:493"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§1 Test Scope Summary → Surfaces under test (the desktop-webview a11y / browser-driven tier row, Signal cell)"
    change: >-
      The Signal cell's "the regression-detector's zero-new-vs-baseline diff" names its baseline and its failing arm: the diff reads the baseline committed in the tree (`pulse-app/ui/tests-a11y/baselines/a11y-violations-summary.json`), and an absent baseline is exit 1 with `regression-detector: baseline not found at {path}`, never an empty baseline (since chunk 2026-10-10-no-ci-step-reads-nothing).
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §1 a11y tier row, Signal: the regression-detector's baseline named as the committed tree file; an absent baseline fails (exit 1) where it was read as empty.
    rationale: >-
      Not a false sentence — an incomplete one at the site the plan names: report, Expected amendments, "test-plan §9 Pipeline structure and §1's a11y tier row", site `regression-detector`: `test-plan.md:56`. The row's signal is the detector's exit code, and its contract changed (Schema / config; Reverted / negative API facts: the "absent baseline is an empty baseline" arm is removed, not kept). Decline if §1 is held to carry no per-arm detail; the §9 row then carries it alone.
    basis: ".andromeda/test-plan.md:56"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → Framework (Rust crates)"
    change: >-
      The count of unit binaries that shell out of Rust is three, not two: beside the `xtask` binary (`cc`) and `unit_l4_grammar.rs` (Python 3), since chunk 2026-10-10-no-ci-step-reads-nothing `pulse-app/tests/a11y_perf_workflow.rs` runs `node` on `pulse-app/ui/tests-a11y/regression-detector.mjs` in `a11y_regression_detector_fails_when_its_baseline_is_absent` and asserts a non-zero exit with `baseline not found` on stderr — a missing `node` FAILS the pin, never skips it; the binary that loads a host library without shelling out (`unit_xlib_threads.rs`) becomes the fourth named.
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §4 Framework (Rust crates): a third binary shells out of Rust — `a11y_perf_workflow.rs` runs `node` on the a11y regression detector; `node` on PATH is a host need of the workspace run, failing and never skipping.
    rationale: >-
      Report, Counts / qualifiers moved: "a new host need for the workspace test run: `node` on PATH (the detector pin runs it; a missing `node` fails the pin, it never skips)"; Symbols / APIs: the new test at `pulse-app/tests/a11y_perf_workflow.rs` `116-143`, its `Command::new("node")` block at `122-130`; Deviations: the pin also asserts stderr holds `baseline not found`. §4's Framework paragraph enumerates the binaries that shell out and counts them "Two" — the runner-side tool set the chunk's tests use is off that list (the report gives no test-plan line for this sentence).
    basis: "pulse-app/tests/a11y_perf_workflow.rs:122-130"
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline structure (the Lint + tests `lint-test` job row, Runs cell)"
    change: >-
      The row's list of what the workspace tests need on the runner (a C compiler, the host's `libX11.so.6`, a Python 3 interpreter) gains `node` on PATH since chunk 2026-10-10-no-ci-step-reads-nothing: `a11y_regression_detector_fails_when_its_baseline_is_absent` (`pulse-app/tests/a11y_perf_workflow.rs`) runs the runner's own `node` — a missing `node` fails the pin, never skips it — green on the verdict run `ci#38035474359` (six jobs, six `success`); the Node version of that run not read.
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §9 Pipeline structure, lint-test row: `node` added to the workspace tests' runner needs (the a11y detector pin), failing and never skipping; green on `ci#38035474359`.
    rationale: >-
      Second occurrence of the host-needs enumeration §4 carries: the lint-test row restates each need of the workspace run with its chunk and its CI witness, and omits the one this chunk added. Report, Counts / qualifiers moved (the `node` host need) and Expected amendments ("the `node` host need for the workspace run" carried into §9 Pipeline structure); Cross-project / external claims: `ci#38035474359` `success`, 7 of 7 checks; Dev-tool versions: the CI image's toolchain was not changed. The report does not say whether a setup step provides `node` in that job, so the amendment must not copy the row's "no setup step" wording for it (the report gives no test-plan line for this row).
    basis: "pulse-app/tests/a11y_perf_workflow.rs:116-143"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Build failure conditions"
    change: >-
      One bullet more, since chunk 2026-10-10-no-ci-step-reads-nothing: a job fails when one of the six upload steps of `ci.yml` finds no file (`if-no-files-found: error` on `logs-perf-samples-*`, `capability-drift-*`, `a11y-violations-*`, `playwright-a11y-report-*`, `logs-boot-*`, `coverage-linux`), and the `a11y` job fails when the regression baseline is absent from the tree; no step of `ci.yml` carries `continue-on-error` and none downloads an artifact — held by `ci_workflow_test_gates_no_continue_on_error` (every step block; its listed-gate arm keeps 14 commands, `cargo audit` among them), `ci_workflow_makes_no_download_without_a_producer` and `ci_workflow_uploads_fail_when_they_find_no_file` (`pulse-app/tests/quality_gate_workflow.rs`). Limits: the upload's failing arm was read on a runner only where the producer did not run (`ci#38034700885`, three uploads), never where one ran and wrote nothing; the a11y upload has three path members and fails only when none matches.
    sidecar: >-
      2026-10-10-no-ci-step-reads-nothing — §9 Build failure conditions: an upload that finds no file and an absent a11y baseline now fail their jobs; the workflow holds no download and no `continue-on-error`, each pinned in `quality_gate_workflow.rs`.
    rationale: >-
      The list enumerates what fails a build and omits two conditions this chunk added — an addition, no sentence of the list is false. Report, Schema / config (`ci.yml` added lines `132 · 141 · 267 · 276 · 372 · 541`), Harness / gate surface (the six names; "Workflow self-lint": the guard on every step block, the two new pins), Counts / qualifiers moved (download steps 3 → 0, `continue-on-error` keys 3 → 0, the guard's list 16 → 14, blocks read 92 of 92), Outcome → Limits. Expected amendments carries "the guard reads every step" and Harness / gate surface into test-plan §9; the Supply chain row's "the `cargo audit` entry of `ci_workflow_test_gates_no_continue_on_error`" (`test-plan.md:494`) and the Boot smoke row's own step pins (`test-plan.md:496`) stay true and take no edit. The pins: `pulse-app/tests/quality_gate_workflow.rs` `332-351` (download), `356-373` (upload body), `265-273` (the guard's every-step arm). Decline if the orchestrator homes the upload key in obs-plan §9 alone.
    basis: "pulse-app/tests/quality_gate_workflow.rs:265-373"
````

**Dispositions:**
- 1 (§9 Test report format) → **apply**. Check 1: playbook 2026-08-14 *doc claim a pre-existing reality falsifies, impl already correct* (routine), and a P5-approved Expected-amendments entry names the change; the operator's recorded direction, inputs#I3 item 5 ("Correct each as measured"). Check 6: report claim 2.
- 2 (§3 → 5-command implementation, `run` → Output format; dependent of 1) → **apply**, with 1. Read at the key file: the same key's Command body asks for `--message-format libtest-json`.
- 3 (§9 Pipeline structure, the A11y suite row, Runs cell) → **apply**. Check 1 as 1; Expected-amendments entry. Check 6: report claim 7.
- 4 (the same row, the host-needs cell; dependent of 3) → **apply**, with 3.
- 5 (§1, the desktop-webview a11y tier row, Signal cell; dependent of 3) → **apply**, kept to one clause. The plan's entry names this row. Check 1: *Accurate this-chunk addition*.
- 6 (§4 Unit Test Strategy → Framework (Rust crates): "Two unit binaries shell out of Rust") → **apply**. Not on the plan's list. Check 1: *Accurate this-chunk addition*; the count the paragraph states moved with this chunk's test. Read at `test-plan.md:188`.
- 7 (§9 Pipeline structure, the `lint-test` row, the workspace tests' runner needs; dependent of 6) → **apply**. The plan's entry names it. The row's "no setup step" wording is not copied for `node`: the job's `Setup Node` step provides it (plan.md, implementation notes; `ci.yml` read whole at implement).
- 8 (§9 Build failure conditions, one bullet) → **apply**. Check 1: *Accurate this-chunk addition*. Check 2: it states the upload key beside obs-plan §9, in the same direction. The two limits ride with it.

## obs-plan

**Verdict:** 8 proposals, one primary and seven `dependent-of`, all under D-obs-defect-narrative by its falsified-mechanism clause (the criterion bench). D-obs-instrumentation, D-obs-stack, D-obs-pii: no drift. The leading comments also name what the detectors do not reach: report claims 3, 4, 5, 6 and the unstated `if-no-files-found: error` behaviour, and that §11's CI bans restate claim 6.

The return, as it arrived:

````yaml
# D-obs-instrumentation: no drift — the report's Coverage states "no new external surface, hot-path operation or UI element"; every new symbol is test-only.
# D-obs-stack: no drift — Dependencies: "none added, none bumped"; no telemetry symbol changed; §3 key files tracing-init / logging-stack read, unaffected.
# D-obs-pii: no drift — the only new output is the a11y harness script's one stderr line carrying the baseline path it was given; no upload `path:` changed, so no artifact gained a member.
# D-obs-defect-narrative: the four numbered DuckDB defects, their lead-in tally and the owner pointer are NOT touched by this chunk (no proposal there). What IS falsified is a mechanism §10 states as current: the criterion bench. This is mechanism prose, not a numbered defect — the fit to the detector is by its "falsified mechanism" clause; the orchestrator may judge it.
# Not proposed, outside all four detectors (all §9 / §11, none a §10 narrative): report Spec claims disproved 3 (`logs-${{ runner.os }}` half of the Log file row, obs-plan.md:499), 4 (`:508`, `:509`, `:511`), 5 (`:522-526`, `-n logs` at `:524`), 6 (Snapshot markdown row `:500`, the operator's at the card), and the unstated `if-no-files-found: error` behaviour. They are the report's Expected amendments, not detector drift. §11 CI bans restate claim 6 ("log file + snapshot on failure … uploaded as CI artifacts", "NEVER skip snapshot generation on test failure") — relevant if claim 6 is applied.
proposals:
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Performance budgets (Snapshot generation row)"
    change: "Strike 'criterion 0.5 in `xtask benches/snapshot.rs` provides regression detection on stable estimators (median + slope)'; the row's assertion is the `metric.snapshot.token_count_ms` events graded by `cargo xtask perf:budget --require memory,snapshot` alone — no criterion bench, `xtask/benches` directory or `bench` verb exists (as measured at chunk 2026-10-10-no-ci-step-reads-nothing)."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §10 snapshot row — criterion bench mechanism retired as measured absent (no bench, no `xtask/benches`, no criterion dependency, no `bench` verb); perf:budget is the sole snapshot-p99 gate."
    rationale: "Report, Spec claims disproved 1 (obs-plan.md:544): no step runs a bench; `ls xtask/benches` finds no directory; `grep -rn criterion --include=Cargo.toml .` finds no dependency line; `xtask` has no `bench` verb; on ci#38031822696 the `criterion-Linux` upload found no file, the `criterion-Linux-base` download found no artifact and the comparison printed `current dir target/criterion not found; NEUTRAL`. §10 states the mechanism as current; it never ran. The upload, download, comparison, the `criterion-regression` verb and its two scripts are removed this chunk (Changes: Symbols / APIs, Harness / gate surface); no bench was added (Reverted / negative API facts)."
    basis: "xtask/src/main.rs (lines removed only: `Cmd::CriterionRegression`, `run_criterion_regression`, `invoke_criterion_regression_check`; `cargo xtask --help | grep -c -E 'criterion-regression|coverage-regression'`: 0); `ls xtask/benches`: no directory"
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Performance budgets (WebGPU canvas frame row)"
    change: "Strike 'offline criterion bench for stable hardware regression detection'; the frame budget's only grader is `cargo xtask perf:frame-sample` on the dev host — no criterion bench exists."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §10 frame row — 'offline criterion bench' clause retired as measured absent."
    rationale: "Report, Spec claims disproved 1 names obs-plan.md:545 among the sites of the same disproved claim (no bench, no `xtask/benches`, no criterion dependency line, no `bench` verb)."
    basis: "`grep -rn criterion --include=Cargo.toml .`: no dependency line (report, Spec claims disproved 1)"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → CI gates (perf-budget bullet)"
    change: "Strike the closing sentence 'Criterion regression detection on stable estimators (median + slope) is the `xtask bench` half.'; say instead that there is no `xtask bench` half — the `criterion-regression` verb, its CI step, the `criterion-Linux` upload and the base-branch criterion download were removed at chunk 2026-10-10-no-ci-step-reads-nothing, and `perf:budget` is the whole perf gate."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §10 CI gates — 'the `xtask bench` half' retired; criterion-regression verb, step, upload and download removed, no bench ever ran."
    rationale: "Report, Spec claims disproved 1 (obs-plan.md:559, 'the `xtask bench` half'); Harness / gate surface: CI steps removed include `Upload criterion bench artifact (deferred; non-blocking)`, `Download base-branch criterion baseline (PR only)` and `cargo xtask criterion-regression (chunk #56 …)`; `xtask` verbs removed: `criterion-regression`."
    basis: ".github/workflows/ci.yml (eight steps removed, three of them criterion; gate entry `8 True True True`); xtask/src/main.rs (nothing added, lines removed only)"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry triggers (perf-budget-instruments, snapshot token budget row)"
    change: "Strike 'Offline regression assertion via `criterion` 0.5 in `xtask benches/`: assert p99 generation latency ≤ 500ms for 25k token budget'; the p99 ≤ 500 ms assertion is the `perf:budget` snapshot arm over the emitted events (§10 snapshot row) — no criterion bench exists."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §1 perf-budget trigger — criterion offline-assertion sentence retired as measured absent; points at the §10 perf:budget arm."
    rationale: "Report, Spec claims disproved 1 names obs-plan.md:125 (§1) as a site of the disproved bench claim; a second occurrence of the mechanism the §10 primary retires."
    basis: "`ls xtask/benches`: no directory (report, Spec claims disproved 1)"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§2 Telemetry Strategy → Telemetry signal pyramid (Metrics row)"
    change: "Strike 'offline regression assertion via `criterion` 0.5 in `xtask benches/` for SLO-critical budgets'; SLO-critical budgets are asserted by `cargo xtask perf:budget` over the `metric.*` event stream (§10) — no criterion bench exists."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §2 Metrics row — criterion offline-assertion clause retired as measured absent."
    rationale: "Report, Spec claims disproved 1 names obs-plan.md:148 (§2) as a site of the disproved bench claim."
    basis: "`grep -rn criterion --include=Cargo.toml .`: no dependency line (report, Spec claims disproved 1)"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§5 Metric Coverage → 'Why `tracing` events instead of an OTel Meter' paragraph"
    change: "Strike 'For runtime SLO enforcement, `criterion` 0.5 in `xtask benches/` provides offline regression assertion with statistically-grounded thresholds'; SLO enforcement is `cargo xtask perf:budget` grading the event stream (§10) — still no OTel Meter dependency, and no criterion bench exists."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §5 OTel-Meter rationale — criterion sentence retired as measured absent; enforcement named as perf:budget."
    rationale: "Report, Spec claims disproved 1 names obs-plan.md:291 (§5) as a site of the disproved bench claim."
    basis: "`ls xtask/benches`: no directory; `xtask` has no `bench` verb (report, Spec claims disproved 1)"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§9 CI Integration → Telemetry artifact handling (Criterion bench JSON row)"
    change: "Remove the 'Criterion bench JSON (on perf-budget regression)' row: no `xtask bench` step and no criterion artifact exist — the `criterion-Linux` upload found no file on ci#38031822696 and was removed at chunk 2026-10-10-no-ci-step-reads-nothing."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §9 Telemetry artifact handling — Criterion bench JSON row removed; the upload never found a file and its step is gone."
    rationale: "Report, Spec claims disproved 1 (obs-plan.md:501, the Criterion bench JSON row); Counts / qualifiers moved: upload steps 9 → 6; the six artifact names that stay hold no criterion artifact (verdict run ci#38035474359: exactly six artifacts)."
    basis: ".github/workflows/ci.yml (step `Upload criterion bench artifact (deferred; non-blocking)` removed; six uploads stay)"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§9 CI Integration → Pipeline integration (`xtask bench` row)"
    change: "Remove the '`xtask bench` | criterion JSON benchmark results | CI artifact `target/criterion/`' row: no workflow step runs `xtask bench`, the verb does not exist, and no `target/criterion/` artifact is uploaded."
    sidecar: "2026-10-10-no-ci-step-reads-nothing: §9 Pipeline integration — `xtask bench` stage row removed as measured absent."
    rationale: "Report, Spec claims disproved 1 and 4 (obs-plan.md:510): 'no step runs `xtask test --release` or `xtask bench`'; `xtask` has no `bench` verb; the comparison printed `current dir target/criterion not found; NEUTRAL` and is now gone."
    basis: ".github/workflows/ci.yml (no `xtask bench` step; `run:` steps 53 → 51); xtask/src/main.rs (no `bench` verb)"
    dependent-of: D-obs-defect-narrative
````

**Dispositions:**
- 1 to 8 (the criterion bench at §10 snapshot row, §10 frame row, §10 CI gates, §1, §2, §5, §9 Telemetry artifact handling, §9 Pipeline integration) → **apply**, the group whole. Check 1: playbook 2026-08-14 *doc claim a pre-existing reality falsifies, impl already correct* (routine: no bench exists and none is owed by this chunk; inputs#I2 item 1), a P5-approved Expected-amendments entry names the change, and inputs#I3 item 5. Check 6: report claim 1. No `basis` cites a line the report does not carry.
  The applied text states the absence where a sentence promised regression detection, and does not drop it silently: what the masters promised and no step delivers is said at the route-resolve card (inputs#I3 item 5).

## a11y-plan

**Verdict:** proposals: [] — two detectors, none violated. The return carried YAML comments: the stale a11y-plan claims of report claim 7 sit outside both invariants; it read the six sites the report names and found the retired claim at each (`ci-integration.md:3`, `:12`, `a11y-plan.md:466`, `:494`, `:565`, `harness-wiring-conventions.md:6`), no seventh; §1's CI Integration block states no base-branch claim, so an edit there is an addition.

## Raised by the orchestrator (Validate checks 5 and 6)

Entries of the plan's Expected amendments list and of the report's Spec claims disproved that no detector proposed. Each is routine: the report substantiates it, and the operator's recorded direction (inputs#I3 item 5, "Correct each as measured") settles it.

- R1 · obs-plan §9 Telemetry artifact handling, the Log file row (`obs-plan.md:499`) → **apply**: the `lint-test` job's `logs-${{ runner.os }}` clause leaves; each upload that stays fails its step on no file. Report claim 3; Schema / config.
- R2 · obs-plan §9 Telemetry artifact handling, the Snapshot markdown row (`:500`) → **apply as measured**: no upload step has a snapshot path. Report claim 6, found at this wrap. The same claim as the Integration tests row of R3, so the two land together. Whether the version still needs the reading: the card.
- R3 · obs-plan §9 Pipeline integration, the `xtask test`, `xtask test --release` and Integration tests rows (`:508`, `:509`, `:511`) → **apply**. Report claim 4.
- R4 · obs-plan §9 CI failure → artifact triage workflow (`:522-526`) → **apply**: the artifact names as measured. Report claim 5.
- R5 · a11y-plan §3 → CI integration (the key file), Pipeline integration and Per-PR regression detection labels, and its CI gate list → **apply**. Report claim 7.
- R6 · a11y-plan §9 CI Integration, the E2E row (`a11y-plan.md:466`) → **apply**. Report claim 7, with the "PR comment" reading added to it at Validate.
- R7 · a11y-plan §10, Zero new violations per PR (`:494`) → **apply**. Report claim 7.
- R8 · a11y-plan §11, the per-PR diff ban (`:565`) → **apply**, the mechanism wording only ("vs base branch" → the baseline committed in the tree); the ban stands. Check 1: *Accurate this-chunk addition*, the reconcile-a-mechanism clause: the invariant holds.
- R9 · a11y-plan §3 → Harness wiring & conventions, Regression baseline (the key file) → **apply**: an absent baseline is an error. Schema / config.
- R10 · a11y-plan §1 CI Integration, the CI gate list (`:214-218`) → **apply**, one bullet (the regression leg; an absent baseline). The plan's entry names the list.

Not amended, and said at the card: obs-plan §11's two bans, "NEVER lose telemetry artifacts (log file + snapshot on failure) — uploaded as CI artifacts with retention" (`:634`) and "NEVER skip snapshot generation on test failure" (`:636`). They are requirements, not descriptions; the workflow does not meet them, and whether the version keeps them is the operator's.

Expected-amendments entries with no amendment: architecture §Infrastructure Patterns → CI/CD approach (the key file states none of the eight removed steps) and security-plan §Dependency Security → CI integration (none expected; the security return read `:216` and found it true). The ledger-note entry was phase's write.

Report claims 8, 9, 10, 11: 8 is a route annotation, corrected at P5; 9 and 10 are chunk-artifact claims, disposed by their report entries with no edit; 11 found nothing in a master.

Rejected for a source the report does not carry: 0. Escalated: 0 (no detector graded a proposal `escalate`, and no proposal widens a boundary).
