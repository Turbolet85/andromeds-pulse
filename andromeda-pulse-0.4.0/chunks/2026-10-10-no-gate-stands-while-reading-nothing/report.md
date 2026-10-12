# Report — 2026-10-10-no-gate-stands-while-reading-nothing

**Chunk:** No gate stands while reading nothing: coverage branch and empty-report arms, empty test selections, boot budget and heartbeat reads, zero-span check: each reads something or leaves (P-128)
**Date:** 2026-10-10
**Commits:** `d99d2dcb chore(2026-10-10-no-gate-stands-while-reading-nothing): operator pre-CI commit` (the operator pass's one commit, parent `7419496b`, the chunk base; `git log --format='%h %s' 7419496b..HEAD`: 1 line)

## Changes (structured — detectors read this)
- **Files:** six source files, `git diff --stat 7419496b -- .github pulse-app xtask`: 694 insertions, 185 deletions.
  - `.github/workflows/ci.yml` (542 → 537 lines, `wc -l` at the base and in the tree)
  - `xtask/src/ci_gates.rs` (new, 243 lines)
  - `xtask/src/main.rs` (1725 → 1763 lines)
  - `xtask/ci/quarantine-tracking-check.sh` (54 → 74 lines) and `xtask/ci/quarantine-tracking-check.ps1` (54 → 71 lines)
  - `pulse-app/tests/quality_gate_workflow.rs` (811 → 1007 lines)
  - Chunk evidence, not source: `evidence/{red-before,mutation-checks,implement-readings,operator-pass,gate-census}.md`.
- **Symbols / APIs:** no IPC method, endpoint, event, socket, port or environment variable is added or removed.
  - **New module `xtask::ci_gates`** (`xtask/src/ci_gates.rs`, declared at `xtask/src/main.rs:10`): `pub enum Verdict { CannotEvaluate, NoRecords, Panic { records, files, member, line }, Pass { records, files } }` (20-37); `Verdict::exit_code` (40-46: Pass 0 · NoRecords and Panic 1 · CannotEvaluate 2); `Verdict::lines` (48-75); `pub fn evaluate(log_dir: &Path) -> Verdict` (93-128); `pub fn run(log_dir: &Path) -> ExitCode` (130-141); six tests (143-243). It reads the family through the existing `perf_budget::family_members` and edits nothing in `perf_budget.rs`.
  - **`cargo xtask ci-gates` — contract changed.** Before: exit 0 with four NEUTRAL lines over no log family; a heartbeat-gap arm (shelling to `xtask/ci/heartbeat-gap-check.{sh,ps1}`) and a perf-budget arm (the in-process grader, no arm required) after the two reading arms; its panic line printed the log's full path and a 240-character preview of the record. Now: **no family member (log dir absent, or holding no `agent-latest.jsonl*` file) → exit 2**, one line `::error::ci-gates: cannot-evaluate (no agent-latest.jsonl* file in the log dir)`; members holding zero records → exit 1, `::error::ci-gates: zero-spans FAIL (log files present but contain no events)` (text unchanged); an `app.panic.fatal` record at level ERROR → exit 1, `ci-gates: zero-spans PASS (…)` then `::error::ci-gates: zero-panic FAIL: app.panic.fatal at {member file name}:{line}`; otherwise exit 0 with exactly `ci-gates: zero-spans PASS ({n} log records across {k} file(s))` and `ci-gates: zero-panic PASS` (both texts unchanged). The verb prints **no heartbeat line and no perf-budget or frame line**. A member that cannot be read is skipped with `ci-gates: skipping {file name} (unreadable)`. `::error::` lines go to stderr, the rest to stdout.
  - **`run_ci_gates`** (`xtask/src/main.rs:488-489`) is now a plain `fn … -> ExitCode` handing over to `ci_gates::run(&resolve_log_dir())`; one caller, `main` (`:309`). Remaining callers of what it stopped calling (`grep -rn` over `xtask/src/*.rs`, definitions excluded): `collect_log_files` 2 (`run_check_ingest_progress` `:497`, `run_perf_load_profiles` `:715`); `invoke_heartbeat_check` 1 (`run_perf_load_profiles` `:741`, now its sole caller); `perf_budget::family_members` 3 non-test (`ci_gates.rs:96`, `perf_budget.rs:341`, `perf_frame.rs:123`). The in-process grader keeps `perf:budget`, `perf:load-profiles` and `perf:frame-sample` as callers; `xtask/ci/heartbeat-gap-check.{sh,ps1}` and `xtask/src/perf_budget.rs` are byte-identical to the base (the scope-guard entry: no output).
  - **The four nextest argument lists are constants** read by a pin: `TEST_ARGS` (`xtask/src/main.rs:430-439`), `TEST_COVERAGE_ARGS` (`:463-473`), `PERF_SLO_LOAD_ARGS` (`:652-663`), `PERF_LOAD_PROFILES_ARGS` (in the added range `:688-701`; its head line read at `:688`). Each holds `--no-tests=fail` where the base held the pass-on-empty value. So `cargo xtask test`, `test:coverage`, `perf:slo-load` and `perf:load-profiles` exit non-zero over a selection that matches no test (read on the dev host: `cargo xtask test -- -E 'test(zz_no_such_test_for_this_chunk)'` → `0 tests run`, `error: no tests to run`, the verb's exit 1 — nextest's own exit is 4, the verb maps a failed status to 1).
  - **Three `about` strings** (`xtask/src/main.rs:79`, `:87`, `:150`): `test` and `test:coverage` name `--no-tests=fail`; `ci-gates` reads "Read the app's obs-log family (agent-latest.jsonl* under the resolved log dir): zero-spans (the family holds at least one record) and zero-panic (no app.panic.fatal record at ERROR). Exit 0 PASS, 1 FAIL, 2 cannot-evaluate (no log family to read)".
  - **`cargo xtask quarantine-tracking`** (the `.sh` and its `.ps1` mirror): a missing search dir → exit 1 with `::error::quarantine-tracking-check: search dir {dir} is missing` per missing dir (relative to the root); a scan of zero `.rs` files → exit 1 with `::error::quarantine-tracking-check: no .rs file under …`; zero `#[ignore]` → `quarantine-tracking-check: PASS (0 quarantine(s) across {N} file(s))`; tracked ones → `PASS ({k} quarantine(s) tracked via GitHub issue URL across {N} file(s))`. The word NEUTRAL is gone from both scripts. An untracked `#[ignore]` fails as before.
- **Crates / modules:** no workspace crate added or removed. One module added to the `xtask` binary crate (`ci_gates`): `ls xtask/src/*.rs` 22 → 23 files.
- **Dependencies:** none added, none bumped (`Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml` unchanged: the scope-guard entry).
- **Schema / config:** none. `.config/nextest.toml` unchanged.
- **Spec-master edits:** none by this chunk before this wrap (implement and the operator pass wrote no master).
- **Counts / qualifiers moved:**
  - Workspace tests, default features: **2890 → 2908** (`cargo xtask test` on `ci#38038281709` → on `ci#38042949735`; the dev host's gate entry read 2908). With `--features mcp-server`: **2928 → 2946** (the `mcp-test` job on the same two runs). 18 tests added.
  - `pulse-app/tests/quality_gate_workflow.rs`: **25 → 31** `#[test]` (`grep -c '^#\[test\]'`). `pulse-app/tests/*.rs` file count unchanged.
  - `xtask` binary tests: **304 → 316** (the dev host's `package(xtask)` selections: 310 with the six `empty_input_tests` before the unit was declared, 316 after).
  - **Coverage thresholds the workflow states and enforces: three → two** (line ≥ 75 %, function ≥ 85 %). The branch ≥ 70 % threshold is retired from `ci.yml`. The `coverage` job's name: `coverage gate (line ≥75% / branch ≥70% / function ≥85%)` → `coverage gate (line ≥75% / function ≥85%)`.
  - **Arms of `cargo xtask ci-gates`: four → two** (zero-spans, zero-panic); its exits `0 · 1` → `0 · 1 · 2`. Lines the pre-push `ci-gates` stage's verb prints over its seed: four kinds → two.
  - **Callers of the in-process perf grader with no arm required: two → one** (`perf:load-profiles`; `ci-gates` left).
  - **CI steps that run the heartbeat gap check: one → zero** (it ran inside `ci-gates` in the `boot` job).
  - `cargo nextest run` lines of `ci.yml` that spell an empty-selection flag: 1 of 2 (the pass value) → 2 of 2 (`--no-tests=fail`). `ci.yml` step count per job unchanged (the parsed-workflow entry: `4 True True True coverage`).
  - Files the quarantine check scans: 311 (P3) → 312 (the new `xtask/src/ci_gates.rs`).
- **Dev-tool versions:** none — cargo-nextest re-read at 0.9.146 on the dev host (research.md); the CI image's 0.9.133 was read taking `--no-tests=fail` on `ci#38042949735`; `pwsh` 7.6.6 on the dev host parsed the `.ps1` mirror (0 parse errors) and did not run it.
- **Harness / gate surface:**
  - `ci.yml`, job `coverage`, step `Enforce coverage thresholds` (inline `run:`): a report tracking 0 lines **or** 0 functions fails with `::error::lcov.info tracks {n} lines and {m} functions — coverage gate cannot pass a report that tracks nothing`, exit 1 (the base exited 0 with "gate trivially passes" when both were 0, and printed `100.0` over a zero total). The `BRF`/`BRH` sums, the branch percentage, its `Branch:` line and its threshold line are gone. The line and function thresholds stay 75 and 85; an absent `lcov.info` still fails.
  - `ci.yml`, job `mcp-test`: `cargo nextest run --workspace --features mcp-server --profile ci --no-tests=fail`. Job `lint-test`: `cargo nextest run --workspace --profile perf-samples --no-tests=fail` (the flag is new on this line); the step `cargo xtask test (no-tests=pass at Foundation epoch)` is renamed `cargo xtask test`.
  - `ci.yml` is otherwise unchanged: six jobs on `ubuntu-22.04`, no matrix, no `needs:`, `permissions: contents: read`, no `uses:` line added, the boot job's smoke, exit-witness, series and `cargo xtask ci-gates` steps equal to the base.
  - `cargo xtask ci-gates`, `quarantine-tracking`, `test`, `test:coverage`, `perf:slo-load`, `perf:load-profiles`: as under Symbols / APIs.
  - `cargo xtask pre-push:linux`: not edited. Its `test` stage runs `cargo xtask test`, which now fails on an empty selection (it ran 2908 tests). Its sixth stage, `ci-gates`, runs the narrowed verb over its own seed log and reads exit 0 with the two lines; the heartbeat and perf-budget reads its seed was written to drive no longer run (the seed's doc comment in `xtask/src/pre_push.rs` still says it "drives the heartbeat arm and the in-process grader's empty-arm reading": a file the plan forbade editing).
  - New pins (workflow self-lint and `xtask`): the thresholds witness, which cuts the step's script out of `ci.yml` by the step's name and runs it with `bash -e` over constructed `lcov.info` files (four tests, `pulse-app/tests/quality_gate_workflow.rs:757-829`); `coverage_job_reads_no_branch_count` (`:831-851`); `ci_workflow_nextest_runs_fail_on_an_empty_selection` (`:853-882`); `empty_input_tests` in `xtask/src/main.rs:1021-1134` (the argument-list pin and five quarantine pins running the `.sh` with `bash` over scratch roots); the unit's six tests (`xtask/src/ci_gates.rs:143-243`). **The workspace tests now need `bash` and `awk` on PATH**: a missing `bash` or `awk` fails the witness, a missing `bash` fails the quarantine pins; neither is skipped.
- **Cross-project / external claims:**
  - **`ci#38042949735`** (GitHub Actions, repository `Turbolet85/andromeds-pulse`): pull-request event, attempt 1, on the pushed tip `d99d2dcb5b33692558fe19729c32d8c2ed0568f8` (merge commit `45ea78a7eb66`), completed/success, read `verdict: green · checks 7/7 · wall 1785 s` beside `secret-scan#38042949743` (completed/success). The sha is the record: this wrap's commit adds to that tree. Its artifacts: exactly `a11y-violations-Linux`, `capability-drift-Linux`, `coverage-linux`, `logs-boot-Linux`, `logs-perf-samples-Linux`, `playwright-a11y-report-Linux`. Its readings are in `evidence/operator-pass.md` and `evidence/gate-census.md`.
  - `ci#38038281709` on `7419496b`: the base run research measured (the controls `6 9`, `1 1 6`, `0 1 3`).
  - `inputs.py verify` at this wrap (`inputs: 4 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 2 · unparsed 0`, read before this report existed):
    - `I1 · ../additional/pc-overseer/relays/pulse-phase-no-gate-reads-nothing-2026-10-10.md · copy no-repo · unchanged`
    - `I2 · message: the operator (the pc overseer), in this session at the P4 dialog, 2026-10-10 · copy message · n/a — a message has no live source`
    - `I3 · ../additional/pc-overseer/relays/pulse-wrap-no-gate-reads-nothing-2026-10-10.md · copy no-repo · unchanged` — snapped at this wrap; cited here as inputs#I3 (the operator's wrap directive).
    - `I4 · message: the operator (the pc overseer), typed into the session, 2026-10-10 · copy message · n/a` — snapped at this wrap; cited here as inputs#I4 (the deviations accepted, the word for the operator pass and the P-128 ref, the wrap's two stops).
- **Reverted / negative API facts:** none shipped and reverted. Four mutations (step 5) each put a base block back, were read red and were restored to the kept sha256; none is in the tree.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **test-plan, the 70 % branch threshold** (§4 Coverage target `test-plan.md:194`; §9 Quality gates row `:501`; §9 Build failure conditions `:510`; §10 Coverage thresholds table `:542`, `:544`; §10 Build failure conditions `:585`; `registries/contracts/test-plan/bootstrap-phases.md:10`). Measured: the arm read `Branch: 0/0 = 100.0%` on every run read; the `coverage-linux` artifact of `ci#38038281709` (id 11664724073) holds 144 `BRF:0` and 144 `BRH:0` records and no `BRDA:` line (research.md). Cause: branch instrumentation is a nightly-only compiler feature (`-Z coverage-options=branch`, `cargo llvm-cov --branch`), the project pins `1.95.0`, and `cargo xtask test:coverage` asks for no branch count. The threshold never read a number. It is retired on the operator's reading (the pc overseer, 2026-10-10; inputs#I2 item 1, inputs#I3 item 2), **PROVISIONAL, awaiting the founder's own word**. What a real branch count would need (plan.md, Constraints): branch instrumentation on the toolchain the project pins — today a nightly-only feature, so the arm returns when that feature is stable on the pinned channel or when the founder admits a second channel for this job — with a measured first reading before a threshold is stated (the workspace's branch coverage was never measured).
  2. **test-plan §9, the MCP-feature tests row** (`test-plan.md:492`) spells the pass-on-empty flag on the `mcp-test` line. The line spells `--no-tests=fail`; so does the perf-samples line, and the step is named `cargo xtask test`.
  3. **obs-plan §10 CI gates, the zero-span bullet** (`obs-plan.md:556`): "Build fails if `xtask test` produces zero spans in log file … enforced by `xtask test` step exit code validation". Measured (research.md (5)): no step of `lint-test` makes that check — `cargo xtask test` returns nextest's own status and reads no log, and the job's `perf:budget` step reads the perf-samples producer's log for its budget arms alone. The one check of the kind is the `zero-spans` arm of `cargo xtask ci-gates`, which counts log RECORDS of any target (not spans) in the `boot` job's log (110 records on `ci#38042949735`).
  4. **obs-plan §10 CI gates, the heartbeat bullet** (`obs-plan.md:558`): enforced "by post-test gap analysis … `xtask/ci/heartbeat-gap-check.sh`". Measured: before this chunk the only CI step making the gap check was `ci-gates` in the `boot` job, over a log holding one tick per target, which has no consecutive pair to subtract (research.md (4)); after it **no CI step makes the gap check**. The script stays, called by `perf:load-profiles` alone, which no workflow runs.
  5. **obs-plan §10, the perf-budget bullet** (`obs-plan.md:559`): "`ci-gates` grades the boot-smoke log with no arm required and prints `perf-budget NEUTRAL` over it", and the frame line it says the boot-smoke `ci-gates` step prints. After this chunk `ci-gates` grades no arm and prints no perf-budget or frame line (`ci#38042949735`: the boot-log entry read `1 1 0`). The `lint-test` budget step is unchanged and read `perf-budget: graded 120 record(s)`, `memory … PASS`, `snapshot … PASS`, `perf-budget: PASS`, and beside them `frame: cannot-evaluate: 0 samples, no adapter record in this log`.
  6. **obs-plan §10 Performance budgets, the frame row** (`obs-plan.md:545`, four `ci-gates` mentions): the boot job's `ci-gates` line `frame: cannot-evaluate: 0 samples, no WebGPU adapter (no_navigator_gpu)` as "the expected line on a run whose smoke step passes". After this chunk the boot job prints no such line on any run. The log still holds the `ui.webgpu.adapter` records (not re-read on this run; the boot artifact was not opened).
  7. **obs-plan §10 Load-profile constraints** (`obs-plan.md:579`): "`ci-gates` and `perf:load-profiles` grade with no arm required". Now `perf:load-profiles` alone.
  8. **obs-plan §9** (`obs-plan.md:499`): "the boot smoke's log, the one `ci-gates` reads in that job" stays true; what the verb reads of it is the record count and the panic read.
  9. **test-plan §1** (`test-plan.md:100`) and **§9 Boot smoke row** (`:496`, `:509`): the same boot-job frame line (`:100`), and `ci-gates` "over the log the smoke wrote" with no statement of what it reads or of its exit 2 (`:496`). Skipped-when-the-smoke-or-series-fails stays true.
  10. **architecture §Occupied Resources → xtask CLI surfaces** (`architecture.md:260`): "`cargo xtask ci-gates` and `perf:load-profiles` run the same grader in-process with no arm required (`ci-gates: perf-budget PASS|FAIL|NEUTRAL`; no log → `perf-budget NEUTRAL (no log file to grade)`)". After this chunk `ci-gates` runs no grader; its contract is exits 0 · 1 · 2 over two arms. The same line's `pre-push:linux` text (six stages, `ci-gates` last) stays true.
  11. **`registries/contracts/test-plan/per-chunk-gate-discipline.md:32`**: the pre-push `ci-gates` stage "over the data dir … holding only a seed log of non-metric records" stays true; the stage now reads two lines over it.
  12. **Chunk-artifact precision, no edit** (inputs#I2's option text for question 2): "A filtered cargo xtask test that matches nothing then exits 4 on the dev host too". Measured: nextest exits 4; the verb maps it to exit 1.
- **Expected amendments (from plan):** five entries. The searches ran over the seven masters and every file under `.andromeda/registries/` (`grep -c -i -E`, hits per file).
  1. *test-plan §4, §9 Quality gates row, §9 and §10 Build failure conditions, §10 Coverage thresholds, §3 Bootstrap phases item 8 — the branch threshold retired as never measured, PROVISIONAL, with what a real branch count would need; an empty report fails.* **Carried:** disproved-claim 1 and the Harness bullet's thresholds step. Search `70 ?%|≥ ?70|branch (cov|≥|thresh|count)|--branch`: test-plan 6 lines (`:194`, `:501`, `:510`, `:542`, `:544`, `:585`), `registries/contracts/test-plan/bootstrap-phases.md` 1 (`:10`), 0 in the other six masters and their key files.
  2. *test-plan §9 Pipeline structure (MCP-feature tests row, the lint-test row) — every nextest invocation spells `--no-tests=fail`; the step is named `cargo xtask test`.* **Carried:** disproved-claim 2. Search `no-tests`: test-plan 1 (`:492`), 0 elsewhere. No master or key file states the `lint-test` step's old name (search `no-tests=pass at Foundation|cargo xtask test \(`: 0 hits in all seven masters and every registry file).
  3. *test-plan §9 Boot smoke row (`:496`), §1 (`:100`), §3 Per-chunk gate discipline (the `pre-push:linux` paragraph) — `ci-gates` reads the record count and the panic read, exits 2 on an absent log, prints no heartbeat, perf-budget or frame line; the pre-push stage reads two lines over its seed.* **Carried:** disproved-claims 9 and 11. Search `ci-gates`: test-plan 3 lines (`:100`, `:496`, `:509`), `registries/contracts/test-plan/per-chunk-gate-discipline.md` 1 (`:32`).
  4. *obs-plan §10 CI gates (zero-span `:556`, heartbeat `:558`, perf-budget `:559`), §10 Performance budgets frame row (`:545`), §10 Load-profile constraints (`:579`), §9 (`:499`, `:508`).* **Carried:** disproved-claims 3 to 8. Searches: `ci-gates` obs-plan 4 lines (`:499`, `:545`, `:559`, `:579`); `zero spans|zero-span` obs-plan 1 (`:556`); `heartbeat-gap|gap check|tick gap` obs-plan 2 (`:545` — the `perf:load-profiles` window sentence, unchanged by this chunk — and `:558`). `obs-plan.md:508` was named by the plan and holds no `ci-gates` token; it is read by the obs detector, not asserted here.
  5. *architecture §Occupied Resources → xtask CLI surfaces (`:260`: the `ci-gates` contract; `quarantine-tracking`'s absent-input arm), §Infrastructure Patterns → CI/CD approach (the coverage job's name where the key file states it), §Stack and Technologies (a test-time row for `bash` and `awk`).* **Carried:** disproved-claim 10, the Symbols bullet's `quarantine-tracking` contract, the Counts bullet's job name, the Harness bullet's `bash`/`awk` need. Searches: `ci-gates` architecture 1 line (`:260`, three mentions), `registries/contracts/architecture/ci-cd-approach.md` 1 (`:3`, two mentions: the step order and "skipped when the smoke or the series fails", both still true); `coverage gate|coverage job` 0 in architecture and 0 in its key files (the job's name is stated in no architecture text: nothing to amend there); `quarantine` 0 in architecture; `\bawk\b|\bbash\b` architecture 1 (`:260`, unrelated), §Stack holds a test-time row for `node` (`architecture.md:33`) and none for `bash` or `awk`.
- **Coverage of new surfaces:**
  - `cargo xtask ci-gates` (narrowed verb, a dev and CI tool, not a product surface) → validation n/a (it reads the log dir the existing resolver gives it; no new variable) · instrumentation n/a (a harness verb prints its verdict; no hot path) · PII redacted✓ (closed labels, counts, a member's file name and a line number; the panic line holds neither the record's text nor a path, by its pin) · tests unit (6) and two gate entries · a11y n/a · tokens n/a
  - `Enforce coverage thresholds` step → validation n/a · instrumentation n/a · PII n/a · tests integ (the witness runs the step's script) · a11y n/a · tokens n/a
  - `xtask/ci/quarantine-tracking-check.sh` → validation n/a · instrumentation n/a · PII n/a (the untracked-`#[ignore]` line prints the file path it scanned, as before) · tests integ (5 pins run the script) · a11y n/a · tokens n/a; `.ps1` mirror → tests ✗ (parsed, not run; no pin reads it)
  - No UI element, no product hot-path operation, no external-input surface.

## Deviations from intent
- **The panic line's text.** The plan fixed its content (file name and line, no record text, no path) and noted "Text added under `xtask/src` stays ASCII"; the base line held a non-ASCII dash. Written `zero-panic FAIL: app.panic.fatal at {file name}:{line}`. The `zero-spans PASS` line still prints before it, as at the base (the plan did not say).
- **The unreadable-member arm.** The plan's arm list did not name it; the base printed the member's full path and the io error. Kept as a skip notice with the file name alone (the harness convention: no path beyond a basename).
- **The exit-2 line** is one `::error::` line on stderr; the plan asked for "one line that names what is missing by a basename".
- **Pins are 18 tests.** The witness is four tests with a third zero-total case (functions and no line); the quarantine pin is five tests.
- **`.ps1` mirror parsed, not run** — as planned; the parse was an addition.
- **Two readings beside the plan** (`evidence/implement-readings.md`): the empty selection through the verb, and the edited `perf-samples` line with its budget step, run by hand.
- All of the above were accepted by the operator as recorded (inputs#I4: "Deviations accepted as recorded").
- scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 6 · listed 6 · recorded 0 … · excluded 57`, read at this wrap against the base `7419496b`).

## Decisions & corrections
- **The operator's words** (inputs#I4): the deviations accepted; the operator pass run by the session on that word, entries 25 to 34 in block order, then the P-128 ref; "A red boot job is a new reading: stop and report its per-boot verdicts; fix nothing on top" (it was green); the wrap is the operator's to call.
- **The operator's wrap directive** (inputs#I3): the pass stands as recorded; the retired branch threshold is PROVISIONAL and its sentence says so with its cause and what a real count would need; the two kept readings are pinned as carries on their owners; three sightings go to the route-resolve card for disposition; a master sentence that promised a reading the version still needs is named at the card, not reworded away; three lines for the main overseer if they occur; stops at the card and before the flip and the commit.
- **P-128's ref was withheld at /implement and written after the pass.** Its acceptance is read on the pull-request run of the pushed commit; the ref names `ci#38042949735`.
- **Sweep hazards found:**
  - A job log echoes its step's script, so a token count over job logs counts the script's own `echo` lines beside the output (the `6 9` control held three lines of the coverage step's text). A token probe over job logs reads the step's source as well as its output.
  - A looped pin stops at its first failing case, so a red-before run reads only that case; the other cases were read directly.
  - A fixture that holds an `#[ignore]` line inside `xtask/src` or `pulse-app/tests` must not start a source line with it: the quarantine check greps line-anchored over those trees and would flag the test's own file. The pins write it inside a one-line string literal.
  - `cargo xtask test` takes nextest arguments only after `--`; without it the argument parser refuses `-E` (exit 2, nothing run), which reads like a failed empty selection and is not one.
  - A one-line removal did not restore the quarantine defect (two checks hold the property); the base block was put back. The rule of 2026-10-10 in `rules/testing.md` held.
- **Seen, not owned by this chunk** (for the route-resolve card, inputs#I3 item 4): `cargo xtask check:ingest-progress` prints `NEUTRAL — no buffer.tick events` and exits 0 (a standard gate entry, not a CI step); the `lint-test` budget step prints `frame: cannot-evaluate: 0 samples` beside its PASS (the frame arm not required there); the `perf:slo-load` verb's `about` string still claims post-test gates it does not run (`xtask/src/main.rs`, the `perf:slo-load` command's `about`); the pre-push seed's doc comment (above).

## Outcome
**Acceptance criteria, each against the diff and the run:**
- (P-128, a) **met** — `ci.yml` holds 0 download steps (`grep -c download-artifact`: 0); the `a11y` job is green with `regression-detector: no new violations vs baseline (0/0)`; no step of that job changed (the parsed-workflow entry).
- (P-128, b) **met** — the artifacts entry read exactly the six names; the six uploads found 1, 1, 5, 1, 42 and 1 files.
- (P-128, c) **met** — `verdict: green`; `6 0`; `1 1`; `1 1 0`; `1 1 3`; the census names each gating step with what it read. One line on the run says an arm read nothing while its step passed (`frame: cannot-evaluate: 0 samples` in the `lint-test` budget step); it is not one of the acceptance's tokens, the step's exit is decided by two required arms that each read samples, and it goes to the card.
- (P-128) each kept gate has a committed pin red on an empty or absent input — **met** (`evidence/red-before.md`: 8 of 12 red on the untouched gates; `evidence/mutation-checks.md`: 4 of 4).
- (P-128) outside the sentence, each with an owner and a stated reading — **met here, pinned at P5**: the a11y upload's three path members (5 files on `ci#38042949735`; which member each matched not read) → `Window's gates retired`; the pre-push `ci-gates` stage (one seed it wrote itself, two lines) → `Agent harness drives the console engine`.
- (tests) the standard gate set passes on the final tree, closed by the base-identity bindings probe — **met** (the block below; re-run at P7).
- (tests) the merge-base probe directly before the push, exit 0 — **met** (`evidence/operator-pass.md`, entry 26).
- (tests) the retired flag counts 0 in `ci.yml` and `xtask/src/main.rs`, and an empty selection exits non-zero by its pin — **met**.
- (tests) the coverage step fails on a report tracking nothing and states no branch threshold — **met** (the witness; the branch-token entry; the job-name entry).
- (arch) six jobs on `ubuntu-22.04`, no matrix, no `needs:`; four steps differ, none added or removed — **met** (`6 of 6`; `4 True True True coverage`).
- (arch) every changed verb arm maps an empty or absent input to exit 1 or 2 — **met** (exit 2; exit 1).
- (arch) no crate, variable, port, procedure or capability added; pre-push six stages green — **met**.
- (security) `permissions: contents: read` kept, no job-level `permissions:`, no token, no `uses:`, no toolchain line added; `rust-toolchain.toml` unchanged — **met** (the added-lines entry read 0).
- (security) the narrowed verb prints closed labels, counts, a file name and a line number — **met** (its pin).
- (obs) no `ci.yml` step grades a perf-budget arm it cannot read: `ci-gates` grades none; the `lint-test` budget step unchanged and green with `perf-budget: graded` — **met**, with the frame line above named.
- (obs) a step of a job that stays still fails on an `app.panic.fatal` record in the log it reads — **met** (the unit's panic pin; `ci-gates: zero-panic PASS` in the boot log).
- (a11y) no step of the `a11y` job changes; its two regression-baseline pins pass unedited — **met**.

**Gates** (the plan's 34 entries by `run`, in order; /implement's one run on the final tree: `entries 34 · green 24 · red 0 · recorded 0 · timeout 0 · not-run 10`; the operator entries from `evidence/operator-pass.md`):
- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `git diff --name-only 7419496b… -- crates pulse-app xtask …` (the scope guard) — green · exit 0, no output
- `python … print(' '.join(sorted(d['jobs'])))` — green · last line `a11y boot coverage lint-test mcp-test supply-chain`
- `python … 'of', len(d['jobs'])` — green · last line `6 of 6`
- `python … the parsed workflow against the base` — green · last line `4 True True True coverage`
- `git diff 7419496b… -- .github/workflows/ci.yml | grep -c -E '^\+.*(uses:|…)'` — green · exit 1, last line `0`
- `cat .github/workflows/ci.yml xtask/src/main.rs | grep -c -e` (the retired flag) — green · exit 1, last line `0`
- `grep -c -E 'BRF|BRH|branch_|Branch:|branch ≥' .github/workflows/ci.yml` — green · exit 1, last line `0`
- `python … ['jobs']['coverage']['name']` — green · last line `coverage gate (line ≥75% / function ≥85%)`
- `ANDROMEDA_PULSE_DATA_DIR="$(mktemp -d)/absent" cargo xtask ci-gates` — green · exit 2, contains `cannot-evaluate`, lacks NEUTRAL and PASS
- `d="$(mktemp -d)" && mkdir -p "$d/logs" && printf … cargo xtask ci-gates` — green · exit 0, both PASS lines, lacks heartbeat, perf-budget, NEUTRAL
- `bash xtask/ci/quarantine-tracking-check.sh "$(mktemp -d)"` — green · exit 1, lacks NEUTRAL and PASS
- `cargo xtask quarantine-tracking` — green · `quarantine-tracking-check: PASS (0 quarantine(s) across 312 file(s))`
- `cargo xtask check:english-sources` — green · `"verdict": "clean"`
- `cargo xtask capability-widening-check` — green · exit 0
- `cargo xtask check:ingest-progress` — green · exit 0 (it printed `NEUTRAL — no buffer.tick events in the log family`; named above)
- `cargo xtask check:staged-artifacts` — green · exit 0
- `cargo xtask capability-drift` — green · exit 0
- `cargo xtask verify:capability-matrix` — green · exit 0
- `cargo nextest run --workspace --profile ci` — green · `2908 tests run: 2908 passed, 0 skipped`
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green · exit 0
- `git diff --quiet 7419496b… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green · `"verdict": "green"`, `"reason": "all-stages-ok"`; again in the operator pass on the committed tree: green, `"head": "d99d2dcb…"`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`, fired as written) — `hygiene: clean`
- `m="$(git ls-remote origin refs/heads/main | cut -f1)" && … git merge-base --is-ancestor "$m" HEAD` (`operator`) — green · exit 0 · `history: unmoved`
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` (`operator`) — green · exit 0 · `history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 7419496b→d99d2dcb`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` (`operator`, fired as written) — exit 0 · `verdict: green · checks 7/7`
- `gh api "…/actions/runs/<id>/artifacts?per_page=100" …` (`operator`, `--id 38042949735`) — green · the six names
- `d="$(mktemp -d)" && for j in $(gh api "…/runs/<id>/jobs…") …` (`operator`) — green · last line `6 0`
- `f="$(mktemp)" && … startswith("coverage") …` (`operator`) — green · last line `1 1`
- `f="$(mktemp)" && … startswith("boot") …` (`operator`) — green · last line `1 1 0`
- `f="$(mktemp)" && … startswith("lint") …` (`operator`) — green · last line `1 1 3`
- `gh api "…/actions/runs/<id>/jobs?per_page=100" --jq '.jobs[] | …'` (`operator`, `expect = []`) — recorded · exit 0. **The outcome it read:** six jobs of `ci#38042949735`, each `success` (lint / test, coverage gate, mcp-server tests, a11y, supply-chain, boot smoke); the run is this chunk's own, on its pre-CI commit.
- No entry carries `defer`; none was skipped or voided. Smoke: skipped — no boot-path or UI-surface change.

**Watches:** none — the entry folded no `WATCH:`.

**Outcome basis:** the operator pass ran. The verdicts rest on its final state: one commit `d99d2dcb` from the base `7419496b`, and the final HEAD's run `ci#38042949735` recorded in `evidence/operator-pass.md` (the record P7 re-verifies). /implement's P4 report as given in this session is the basis for what only it holds (the red-before reading, the mutation checks, the two hand readings, the deviations); the operator's word between it and this report accepted the deviations and ordered the pass (inputs#I4). The implement conversation is present in this window; the evolve records were not read.

**Process hygiene** (implement's census, re-measured here against the host's process list): every `cargo`, `nextest`, `xtask`, gate-tool, pre-push and sample-producer process this chunk's runs started — terminated (none listed after /implement, none after the operator pass); the CI wait of entry 28 — terminated (it printed its verdict and exited 0); the gate-output monitor — terminated. Seen on the host and not this chunk's: two `ci.py conclusion` waits and a push-watch loop whose working directory is another project (`escher`), under other sessions — left alone. Ports 4317 and 4318 were not bound by any run.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 7419496b (the parent of the oldest pre-CI commit d99d2dcb) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 9 line(s) in 6 range(s)
added: 111 · 121 · 199 · 450 · 517-519 · 521-522
### pulse-app/tests/quality_gate_workflow.rs — added 201 line(s) in 3 range(s)
added: 6-11 · 450 · 690-883
- 693-713 @694 «fn workflow_run_script(content: &str, job: &str, step: &str) -> String {»
  - 698-701 «let start = lines»
  - 702-706 «assert_eq!(»
  - 707-712 «lines[start + 2..]»
- 715-725 «fn lcov_report(lines: Option<(u32, u32)>, functions: Option<(u32, u32)>) -> String {»
  - 717-719 «if let Some((hit, found)) = functions {»
  - 720-722 «if let Some((hit, found)) = lines {»
- 727-755 @730 «fn run_coverage_thresholds_step(report: Option<&str>) -> (Option<i32>, String) {»
  - 731-734 «let awk = std::process::Command::new("awk")»
  - 737-739 «if let Some(report) = report {»
  - 743-748 «let output = std::process::Command::new("bash")»
  - 749-753 «let printed = format!(»
- 757-781 @758 «fn coverage_thresholds_step_fails_on_a_report_tracking_nothing() {»
  - 759-763 «let cases = [»
  - 764-780 «for (case, report) in cases {»
- 783-797 @784 «fn coverage_thresholds_step_passes_a_report_over_both_thresholds() {»
  - 788-796 «for line in [»
- 799-819 @800 «fn coverage_thresholds_step_fails_a_report_under_either_threshold() {»
  - 801-810 «let cases = [»
  - 811-818 «for (report, error) in cases {»
- 821-829 @822 «fn coverage_thresholds_step_fails_without_a_report() {»
  - 825-828 «assert!(»
- 831-851 @834 «fn coverage_job_reads_no_branch_count() {»
  - 836-842 «for token in ["BRF", "BRH", "branch_", "Branch:"] {»
  - 843-846 «let name = coverage»
  - 847-850 «assert!(»
- 853-882 @854 «fn ci_workflow_nextest_runs_fail_on_an_empty_selection() {»
  - 856-861 «let runs: Vec<&str> = content»
  - 862-866 «assert!(»
  - 867-873 «for line in &runs {»
  - 874-881 «for (idx, line) in content.lines().enumerate() {»
### xtask/ci/quarantine-tracking-check.ps1 — added 27 line(s) in 9 range(s)
added: 1 · 3-5 · 11-14 · 17-26 · 29 · 31 · 44-48 · 50 · 68
- 18-23 «foreach ($dir in $searchDirs) {»
  - 19-22 «if (-not (Test-Path -LiteralPath (Join-Path $workspaceRoot $dir) -PathType Container)) {»
- 44-47 «if ($fileCount -eq 0) {»
### xtask/ci/quarantine-tracking-check.sh — added 36 line(s) in 8 range(s)
added: 2-11 · 20-23 · 26-37 · 40-41 · 44 · 47-51 · 53 · 71
- 27-32 «for dir in "${SEARCH_DIRS[@]}"; do»
  - 28-31 «if [ ! -d "$WORKSPACE_ROOT/$dir" ]; then»
- 33-35 «if [ "$missing" -ne 0 ]; then»
- 47-50 «if [ "$files" -eq 0 ]; then»
### xtask/src/ci_gates.rs — new file · 243 line(s)
- 20-37 @21 «pub enum Verdict {»
  - 26-32 @27 «Panic {»
  - 33-36 «Pass {»
- 39-76 «impl Verdict {»
  - 40-46 «pub fn exit_code(&self) -> u8 {»
  - 48-75 @49 «pub fn lines(&self) -> Vec<String> {»
- 78-83 «fn file_name(member: &Path) -> String {»
  - 79-82 «member»
- 85-91 «fn is_panic_at_error(raw: &str) -> bool {»
  - 86-88 «let Ok(record) = serde_json::from_str::<Value>(raw) else {»
- 93-128 @95 «pub fn evaluate(log_dir: &Path) -> Verdict {»
  - 97-99 «if members.is_empty() {»
  - 102-116 «for member in &members {»
  - 118-127 «match panic {»
- 130-141 @131 «pub fn run(log_dir: &Path) -> ExitCode {»
  - 133-139 «for line in verdict.lines() {»
- 143-243 @144 «mod tests {»
  - 151-155 «fn panic_record(level: &str) -> String {»
  - 157-161 «fn log_dir_holding(content: &str) -> tempfile::TempDir {»
  - 163-173 «fn assert_cannot_evaluate(verdict: &Verdict) {»
  - 175-179 @176 «fn an_absent_log_dir_cannot_be_evaluated() {»
  - 181-186 @182 «fn a_log_dir_with_no_family_member_cannot_be_evaluated() {»
  - 188-194 @189 «fn a_member_with_no_record_fails() {»
  - 196-208 @197 «fn one_record_passes_with_the_two_reading_lines_alone() {»
  - 210-228 @211 «fn a_panic_record_at_error_fails_by_file_name_and_line_alone() {»
  - 230-242 @231 «fn a_panic_target_record_below_error_passes() {»
### xtask/src/main.rs — added 178 line(s) in 16 range(s)
added: 10 · 79 · 87 · 150 · 309 · 430-440 · 446 · 463-474 · 477 · 488-489 · 652-664 · 667-668 · 673 · 688-701 · 706
       1021-1135
- 430-439 «const TEST_ARGS: [&str; 8] = [»
- 463-473 «const TEST_COVERAGE_ARGS: [&str; 9] = [»
- 652-663 «const PERF_SLO_LOAD_ARGS: [&str; 10] = [»
- 1021-1134 @1022 «mod empty_input_tests {»
  - 1025-1044 @1026 «fn every_nextest_argument_list_fails_on_an_empty_selection() {»
  - 1046-1047 «const QUARANTINE_SEARCH_DIRS: [&str; 4] =»
  - 1049-1058 «fn quarantine_root(sources: &[(&str, &str)]) -> tempfile::TempDir {»
  - 1060-1076 @1061 «fn quarantine_check(root: &Path) -> (Option<i32>, String) {»
  - 1078-1088 @1079 «fn quarantine_check_fails_when_no_search_dir_exists() {»
  - 1090-1097 @1091 «fn quarantine_check_fails_on_a_scan_of_no_source_file() {»
  - 1099-1109 @1100 «fn quarantine_check_says_how_many_files_it_scanned() {»
  - 1111-1121 @1112 «fn quarantine_check_fails_an_ignore_without_an_issue_url() {»
  - 1123-1133 @1124 «fn quarantine_check_passes_an_ignore_with_its_issue_url() {»
