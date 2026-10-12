# Codebase Research — 2026-10-10-no-gate-stands-while-reading-nothing

## Scope
- **Depth:** deep on the gates, moderate on their pins · **Reads:** 16 files · **Globs/Greps:** 31
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, its Session Additions included
  (it loaded on the first `xtask` read); 2 additions applied: 2026-05-14 (`ci-gates` reads every
  `agent-latest.jsonl*` under the resolved data dir, so a probe names a fresh one) and 2026-06-29 (a log reader
  globs the date-suffixed family). No live leg is planned: the chunk boots no app and binds no port.
- **Platform issues consulted:**
  - `https://github.com/taiki-e/cargo-llvm-cov/issues/8` (fetched; closed): "0.6.8 (#356) added unstable
    `--branch` flag to enable branch coverage"; the feature "requires nightly-2024-03-16+".
  - `https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/coverage-options.html` (fetched): the flag
    takes `block`, `branch`, `condition`; `branch` is "In addition to block coverage, also enables branch coverage
    instrumentation"; the page is the Unstable Book's, a nightly-only flag.
  - `actions/upload-artifact` `action.yml` at the pinned sha `ea165f8d` (tags `v4`, `v4.6.2`; read through
    `gh api …/contents/action.yml?ref=…`): `if-no-files-found` is "The desired behavior if no files are found using
    the provided path"; `error` is "Fail the action with an error message". The rule is stated for the path input
    as a whole, not per member.
  - No failure signature was searched: the scope holds no runner-only red (Setup 5a read green).
- **External inputs:** `inputs#I1` — the operator's phase directive (every carried gate measured before planning;
  no third state; where a gate sits decides half; the branch arm's cause; P-128 claimed here; the merge-base probe;
  the founder halt; the P5 stop). `inputs#I2` — the operator's four answers at the P4 dialog (an empty coverage
  report fails and the branch arm is retired, PROVISIONAL for the founder; all five empty selections fail;
  `ci-gates` narrowed to its two reading arms; the quarantine check repaired, the a11y upload and the pre-push
  stage each keeping an owner and a stated reading).

## The run every reading below was taken on
`ci#38038281709` on `7419496b` (pull-request event, attempt 1, completed/success, 7 of 7 checks with
`secret-scan#38038281700`), jobs read through `gh api repos/Turbolet85/andromeds-pulse/actions/runs/38038281709/jobs`
(6 jobs) and each job's log fetched whole to a file with `--allow-escape-sequences` (coverage 6249 lines, lint-test
14669, boot 2950, mcp-test 8392, a11y 3103, supply-chain 3097; none empty). The runner's cargo steps ran the pinned
toolchain: "the toolchain '1.95.0-x86_64-unknown-linux-gnu' is currently in use (overridden by …
rust-toolchain.toml)" (coverage log :358, lint-test log :332). `ci.yml` at `7419496b` is the file this chunk edits.

## The carried gates, each measured two ways
"Full" is what the gate read on that run. "Empty or absent" is what it does over nothing, measured on the dev host
at `7419496b` unless a runner reading is named. A constructed input lived in the session scratchpad; nothing was
written in the tree.

### (1) The coverage thresholds step's zero arm — job `coverage`, `ci.yml:519-522`
- **Full:** the arm did not fire. `Line: 33992/38252 = 88.9% (threshold 75%)` · `Function: 3565/4055 = 87.9%
  (threshold 85%)` (coverage log :5234, :5236); the suite under it ran `2890 tests run: 2890 passed, 0 skipped`
  (:5192).
- **Empty or absent** (the step body cut from `ci.yml:508-533`, 26 lines, run with `bash` over constructed
  `lcov.info` files): an empty `lcov.info` → **exit 0**, `Coverage gate: 0 lines / 0 functions tracked (Foundation
  epoch — gate trivially passes; activates at chunk #15 first tests)`; no `lcov.info` at all → exit 1,
  `::error::lcov.info missing — coverage gate cannot evaluate`.
- The same step holds a second pass over nothing: each percentage prints `100.0` when its total is 0 (`:523-525`),
  so with the zero arm merely deleted a report tracking nothing would read 100 % on every line and still pass.
- **Where it sits:** a job that stays. No test in the tree pins the step's arms (`grep -n -E 'Enforce coverage|
  threshold' pulse-app/tests/*_workflow.rs`: 0 hits).

### (2) The same step's branch arm — `ci.yml:515-516`, `:524`, `:527`, `:531`; the job's name `:450`
- **Full:** `Branch: 0/0 = 100.0% (threshold 70%)` (coverage log :5235). The uploaded report (artifact
  `coverage-linux`, id 11664724073, `lcov.info`, 1441580 bytes) holds 144 `BRF:` records and 144 `BRH:` records,
  every one `BRF:0` / `BRH:0`, and **0 `BRDA:` lines**; its `LF`/`LH`/`FNF`/`FNH` sums are 38252 / 33992 / 4055 /
  3565, the numbers the step printed (counted in python over the downloaded artifact).
- **Why the report holds no branch count:** branch instrumentation is a nightly-only compiler feature
  (`-Z coverage-options=branch`; `cargo llvm-cov --branch` "(unstable)" in `cargo llvm-cov --help` of the host's
  0.9.1, and per the fetched issue above; the runner installs 0.8.5). The project pins `channel = "1.95.0"`
  (`rust-toolchain.toml`), the runner used it, and `run_cargo_llvm_cov` passes no `--branch`
  (`xtask/src/main.rs:462-472`). On the pin the tool writes the per-file summary records with a zero total and no
  per-branch line.
- **Empty or absent** (the same step body): a report with lines and functions and no `BRF`/`BRH` record →
  `Branch: 0/0 = 100.0%`, exit 0; `BRF:10`/`BRH:5` → `::error::branch coverage 50.0% < 70%`, exit 1;
  `BRF:10`/`BRH:8` → exit 0. The arm can fail, and nothing the pinned toolchain produces reaches it.
- **Cost of producing a count:** a toolchain the project does not pin, in the `coverage` job (arch §Stack names
  one channel, held by the xtask test `declared_floor_equals_the_pinned_channel`). The step that would change ran
  26 min 5 s on that run (`cargo xtask test:coverage`, 08:35:13 → 09:01:18). What the workspace's branch coverage
  reads under a nightly was **not measured**: it needs a full instrumented build on another toolchain, and the
  default-features test run under it rewrites the tracked bindings. A `nightly-2026-09-20` is installed on the dev
  host (`rustup toolchain list`), so the measurement is possible on the operator's word.
- **Nothing outside the workflow keys on the job's name:** `main` has no branch protection
  (`gh api …/branches/main/protection` → 404 "Branch not protected") and the repository has no ruleset
  (`gh api …/rulesets` → `[]`); `git grep` for the name and for a 70 % branch threshold outside the masters and
  the leaves finds only `ci.yml:450` and `:531`.

### (3) and the second `CARRY:` — the five `--no-tests=pass` literals
Sites (`command grep -rn -e '--no-tests' .github xtask/src scripts .config pulse-app/tests crates`: 7 hits, two of
them `about` strings): `ci.yml:199`; `xtask/src/main.rs:440` (`run_cargo_nextest`), `:469` (`run_cargo_llvm_cov`),
`:749` (`run_perf_slo_load`), `:780` (`run_perf_load_profiles`); the `about` strings `:78`, `:86`; the step's name
`ci.yml:111` ("no-tests=pass at Foundation epoch").
- **Full, per site:** `mcp-test` → `2928 tests run: 2928 passed, 0 skipped` (mcp log :4522); `cargo xtask test`
  → `2890 tests run` (lint-test log :10683); `cargo xtask test:coverage` → 2890 (above); `cargo xtask
  perf:slo-load` → `(1/1) pulse-app::perf_slo_10k_spans`, `1 test run: 1 passed` (lint-test log :13594);
  `perf:load-profiles` runs in no workflow (`grep -rn 'perf:load-profiles' .github/workflows/`: 0 hits) and its
  selection lists 4 tests on the dev host (`cargo nextest list -p pulse-app --test perf_load_profiles --profile
  load-profiles`).
- **Empty** (measured on cargo-nextest 0.9.146, package `security`, filter `test(zz_no_such_test_for_p3_probe)`;
  the control with no filter ran 85 tests): with `--no-tests=pass` → **exit 0**; without it → exit 4,
  `error: no tests to run`. The runner installs 0.9.133 (lint-test log :604); its default on an empty selection
  was not read, so a repair spells the behaviour (`--no-tests=fail`) rather than leaning on a default.
- The sixth nextest step, `cargo nextest run --workspace --profile perf-samples` (`ci.yml:121`), carries no such
  flag and ran `1 test run`.
- **Where they sit:** four in jobs that stay; the fifth in a release-cadence verb no workflow runs. The
  `pre-push:linux` `test` stage runs `cargo xtask test` (`xtask/src/pre_push.rs:277`).

### (4) `cargo xtask ci-gates` — job `boot`, `ci.yml:363-364`; `run_ci_gates` at `xtask/src/main.rs:483-571`
- **Full** (boot log :1845-1852): `ci-gates: zero-spans PASS (112 log records across 1 file(s))` ·
  `ci-gates: zero-panic PASS` · `heartbeat-gap-check: max gap 0ms in  (threshold 45000ms) PASS` ·
  `ci-gates: perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter (no_navigator_gpu)` ·
  `… memory NEUTRAL — populated 0 of 1` · `… snapshot NEUTRAL — no metric.snapshot.token_count_ms record` ·
  `ci-gates: perf-budget NEUTRAL`.
- **What that log holds:** a runner boot log of the same step, kept on the host from `ci#38026637514` attempt 3
  (`target/boot-smoke/ci-38026637514-attempt-3/`, 110 records, first 06:02:58.837, last 06:03:04.637), holds
  exactly **one** record each of `ingest.tick`, `buffer.tick`, `viz.tick`, `plugins.tick`, **one**
  `metric.buffer.memory_bytes`, no `metric.snapshot.token_count_ms`, no frame record and two `ui.webgpu.adapter`
  records. `ci#38038281709`'s own boot artifact was not opened (it also carries the exit-witness files, which are
  read whole or not at all); its `ci-gates` lines are the same shape.
- **Arm by arm, over constructed data dirs** (`ANDROMEDA_PULSE_DATA_DIR={scratch dir} cargo xtask ci-gates`):

  | input | exit | what printed |
  |---|---|---|
  | no data dir · a `logs/` dir with no log file | **0** | four `NEUTRAL` lines |
  | a log file with zero lines | 1 | `zero-spans FAIL (log files present but contain no events)` |
  | one non-tick record (the pre-push seed's shape) | **0** | `zero-spans PASS (1 …)` · `zero-panic PASS` · heartbeat `no … .tick records found (INACTIVE state — … gate trivially passes)` · `perf-budget NEUTRAL` |
  | the kept runner log | **0** | the run's lines, `PASS (110 …)` |
  | the kept runner log + one `app.panic.fatal` ERROR record | 1 | `zero-panic FAIL — app.panic.fatal at …:111` |
  | two `buffer.tick` 50 s apart | 1 | `heartbeat-gap-check: max gap 50000ms in buffer.tick exceeds 45000ms threshold` |
  | one `metric.buffer.memory_bytes` of 600000000 | 1 | `memory max 600000000 B > 512000000 B (n=1, populated 1) FAIL` |

- **Reading:** on the boot job's log the heartbeat arm has no consecutive pair to subtract (one tick per target,
  the script prints the gap as 0 and an empty target name), the memory arm holds one zero sample, the snapshot and
  frame arms hold none. None of the three can fail on a boot of this length; each can fail on an input it never
  gets there. The zero-spans and zero-panic arms read the 112 records and fail on what they are for.
- **The step is reached only behind two gating steps** that already need the log to hold records
  (`harness:settled` reads four `app.boot.window.navigation` records): an empty or absent log reddens the smoke
  first, and `ci-gates` is then skipped (test-plan §9; measured at an earlier chunk on `ci#38010977166`).
- **The same verb in `pre-push:linux`:** its sixth stage writes one seed record, `app.boot.ready`, into a fresh
  data dir and runs `cargo xtask ci-gates` over it (`xtask/src/pre_push.rs:32-34`, `:278-281`, `:519-525`). It is
  the fourth row of the table: green over a record the stage wrote itself. It is not a step of `ci.yml`.
- **Seen while probing, not a live defect:** `heartbeat-gap-check.sh` finds a tick by a compact-JSON pattern
  (`"target":"…"`). A record spelled with a space after the colon is not seen as a tick (two such ticks 50 s
  apart read `no … .tick records found … trivially passes`, exit 0). The app writes compact JSON (the kept runner
  log), so its own logs are read.
- **Where it sits:** a job that leaves at `Window's gates retired` (`working-route.md:48`). The engine-side
  owner is on the route: `Agent harness drives the console engine` (`working-route.md:31`) reads "panic,
  heartbeat-gap, process-end, budget checks grade its log (P-086)". The enforced budget gate today is
  `lint-test`'s `cargo xtask perf:budget … --require memory,snapshot`, which on this run read
  `graded 120 record(s) across 1 file(s)` · `memory max 7680000 B <= 512000000 B (n=3, populated 3) PASS` ·
  `snapshot p99 69.9 ms <= 500 ms (n=50) PASS` (lint-test log :13639-13643).

### (5) The zero-span build failure — obs-plan §10 (`obs-plan.md:556`)
- **Measured:** no step of `lint-test` makes the check (`ci.yml:23-142`, read whole); `run_cargo_nextest`
  returns nextest's own status and reads no log (`xtask/src/main.rs:429-452`). The one check of the kind is the
  `zero-spans` arm above, which counts **log records** of any target, not spans, and runs in the `boot` job.
- **What `lint-test` does read:** the `perf:budget` step fails on an absent log family (exit 2) and on a required
  arm with no sample; on this run it graded 120 records of the producer's log. No sentence of obs-plan names that
  as a zero-record check.
- **Where it sits:** the sentence is a master's; phase amends none.

### The third `CARRY:` — the `a11y-violations` upload's path members, `ci.yml:258-268`
- **Full:** `With the provided path, there will be 5 files uploaded` (a11y log :1755), artifact id 11664124403.
  The three members are `{data dir}/logs/a11y-*.json*`, `pulse-app/ui/dist/contrast-report.json` and
  `pulse-app/ui/tests-a11y/regression-set.json`.
- **Each member's producer is a link of one `&&` chain** inside `cargo xtask test:a11y` (a11y log :1702):
  `npm run verify:contrast` wrote the contrast report (`12 pairs evaluated`, :1708), the aggregator the summary
  under the data dir (`6 violation tuple(s) across 7 surfaces`, :1726), the regression detector
  `regression-set.json` (`no new violations vs baseline (0/0)`, :1727). A member is absent when its producer
  failed, which already reddens the job, or when a producer exits 0 without writing, which no run has shown.
- **One member missing while another matches:** not measured on a runner. The action's own text states the rule
  for the path input as a whole (above).
- **Where it sits:** a job that leaves at `Window's gates retired`. The upload is read back by no gate
  (a11y-plan §10: the comparison reads the tree baseline, no artifact).

## Found at P3, not stated by the entry
- **`cargo xtask quarantine-tracking`** (`lint-test`, `ci.yml:93-94`; `xtask/ci/quarantine-tracking-check.sh`)
  printed `quarantine-tracking-check: NEUTRAL (zero #[ignore] in source; gate establishes convention for future
  quarantines)` on the run (lint-test log :1637). Measured on the dev host: the tree → exit 0, the same line, over
  311 `.rs` files under its four search dirs (`find crates pulse-app/src pulse-app/tests xtask/src -name '*.rs' |
  wc -l`); a root where **none of the four search dirs exists → exit 0, the same line**; a constructed root with one
  `#[ignore]` and no issue URL → exit 1; one with the URL → exit 0 `PASS (1 quarantine(s) …)`. It reads the tree
  and can fail; it prints NEUTRAL, and it passes over an absent input.
- **The other gating steps each read something on that run** (the census; lint-test, a11y and supply-chain logs
  segmented by their `##[group]Run` lines): `check:english-sources` `"files_scanned": 491`; `capability-drift`
  `clean (0 missing, 0 extra)`; `check:staged-artifacts` `staged-clean`; `capability-widening-check`
  `0 violations across 3 inspected`; `verify:capability-matrix` `60/60 capabilities`; the `perf-samples` producer
  `1 test run`; the a11y chain `41 passed`, `7 surfaces audited`, `7/7 URLs passed`; `cargo audit` `Loaded 1296
  security advisories`; `cargo deny` `bans ok, licenses ok, sources ok`; `check:npm-supply-chain` `"verdict":
  "green"`; `fmt`, `clippy`, `typecheck` and the two release builds compile or check the tree. `cargo tree -p
  wasmtime | grep -q "cranelift" || exit 1` fails on an empty stream by construction.
- **The first two clauses of P-128 hold on that run:** `ci.yml` holds 0 download steps (`grep -c
  'download-artifact'`: 0); the six uploads found 1, 1, 5, 1, 42 and 1 files (`logs-perf-samples-Linux`,
  `capability-drift-Linux`, `a11y-violations-Linux`, `playwright-a11y-report-Linux`, `logs-boot-Linux`,
  `coverage-linux`); the a11y comparison read the tree baseline.
- **Seen, not measured further:** the `cargo deny` action's log opens with `error: override toolchain
  '1.95.0-x86_64-unknown-linux-musl' is not installed` inside its container and still ends `bans ok, licenses ok,
  sources ok`. `xtask/ci/l4-latency-p99.{sh,ps1}` carry the same "INACTIVE … trivially passes" arms; no workflow
  runs them.

## Files inspected
- `.github/workflows/ci.yml` (full, 542 lines) — the six jobs, every step and key this chunk decides.
- `xtask/src/main.rs` (:420-580, :604-692, :694-841) — the two nextest wrappers, `run_ci_gates`, the log-dir
  resolver, the heartbeat shell-out, the two perf verbs.
- `xtask/src/perf_budget.rs` (:1-112 and its index of items) — the three arms, `Neutral`, `evaluate`, `arm_lines`,
  21 in-crate tests.
- `xtask/src/pre_push.rs` (:32-34, :262-292, :515-528) — the seed record and the `test` and `ci-gates` stages.
- `xtask/ci/heartbeat-gap-check.sh` (full) and `xtask/ci/quarantine-tracking-check.sh` (full).
- `.config/nextest.toml` (full) — the four profiles and their filters.
- `pulse-app/tests/quality_gate_workflow.rs` (:226-374, :440-462, its test index) and
  `pulse-app/tests/a11y_perf_workflow.rs` (:60-96) — the pins that read the edited steps.
- `rust-toolchain.toml` (full).
- `.andromeda/obs-plan.md` (:556-559) and the two later route entries (`working-route.md:31`, `:43`, `:48`, `:50`).
- The last chunk's `evidence/other-gates.md`, `scope.md` and `inputs/I2-relay-1.md.txt` (full).

## Graph impact (rust plane; trace `tree-query-2026-10-10-no-gate-stands-while-reading-nothing.json`, 14 rows)
- **`run_ci_gates`** — 1 caller: `main` @ `xtask/src/main.rs:308`. Changing its arms moves one verb.
- **`invoke_heartbeat_check`** — 2 callers: `run_ci_gates` @ `xtask/src/main.rs:542`, `run_perf_load_profiles` @
  `:818`. The script stays in use by the second whatever the first does.
- **`collect_log_files`** — 3 callers: `run_ci_gates` @ `:484`, `run_check_ingest_progress` @ `:578`,
  `run_perf_load_profiles` @ `:792`. An absent-log arm changed inside `run_ci_gates` leaves the other two as they
  are; one changed inside the helper would move all three.
- **`run_cargo_nextest`** @ `:266`, **`run_cargo_llvm_cov`** @ `:267`, **`run_perf_slo_load`** @ `:323`,
  **`run_perf_load_profiles`** @ `:326` — one caller each, `main`. The flag is an argument literal in each body.
- The `perf_budget` grader is shared by `perf:budget`, `ci-gates` and `perf:load-profiles`; its `Neutral` arm is
  what `perf:budget` turns into FAIL for a required arm and into exit 2 when nothing is required and nothing read
  (`xtask/src/perf_budget.rs:215-229`, `:364`).

## Patterns detected
- **A required arm is how an empty input fails** (`xtask/src/perf_budget.rs:215-229`): `evaluate(results,
  required)` reads a required `Neutral` arm as `Fail`; `lint-test` names `memory,snapshot`.
- **Three exits, cannot-evaluate never green** (`xtask/src/perf_budget.rs:330-364`; the registered form of
  `check:english-sources`, `check:staged-artifacts`, `check:npm-supply-chain`): 0 green · 1 red · 2 could not read.
- **A workflow step is pinned by a test that reads `ci.yml` as text** (`pulse-app/tests/quality_gate_workflow.rs`,
  25 tests by `grep -c '^#\[test\]'`): `ci_workflow_test_gates_no_continue_on_error` (`:231`) lists 14 gate commands, among them
  `cargo nextest run`, `cargo xtask test`, `cargo xtask ci-gates`, `cargo xtask perf:slo-load`,
  `cargo xtask quarantine-tracking`; `ci_workflow_invokes_ci_gates` (`:443`) and
  `ci_workflow_invokes_quarantine_tracking_check` (`:209`) pin a step's presence;
  `ci_workflow_boot_series_runs_after_the_smoke_whatever_it_returned_and_before_ci_gates` (`:651`) reads the
  `cargo xtask ci-gates` step as the series' right neighbour.
- **A witness that runs the tool** (`pulse-app/tests/a11y_perf_workflow.rs:97-`, the last chunk's absent-baseline
  pin): the test runs the real script over a constructed input and asserts a non-zero exit; a missing interpreter
  fails the test.
- **`xtask` pins its own arms in-crate** (`xtask/src/*.rs`, `#[cfg(test)]` modules, 22 files with tests;
  `xtask::bin/xtask` in the suite).

## Conventions to follow
- **One nextest selection form:** `--workspace` narrowed by `-E`, never `-p` (`xtask/src/main.rs:737-738`).
- **New `pulse-app` pins live under `pulse-app/tests/`** (`[lib] test = false`); a new file there moves a tracked
  count (test-plan §4).
- **A harness verb prints closed labels and counts**, no environment value and no path beyond a basename
  (security-plan §Security Anti-Patterns → Input; the `zero-panic FAIL` line today prints the log's full path and
  a 240-character preview of the record, `xtask/src/main.rs:519-522`, `:536`).
- **Workflow comments avoid the tokens `windows`, `macos`, `matrix.os`, `runner.os ==`**
  (`ci_workflow_runs_on_linux_only` reads every line).
- **Text added to `xtask` and the workflow stays ASCII where the source lint reads it** (`xtask/src` is one of its
  five roots; `ci.yml` is not).

## New files to create
- `xtask/src/ci_gates.rs` — the verb's arms and their verdict as a unit the crate's tests can call

## Files to modify
- `.github/workflows/ci.yml` — the coverage thresholds step and its job's name, the mcp-test nextest step, the lint-test test step's name
- `xtask/src/main.rs` — the four nextest argument lists, three about strings, `run_ci_gates` handing over to the new unit, two stale comments
- `xtask/ci/quarantine-tracking-check.sh` — the absent-input arm and the printed word
- `xtask/ci/quarantine-tracking-check.ps1` — the same arm, mirrored and not run on this host
- `pulse-app/tests/quality_gate_workflow.rs` — the pins of each edited step, and a witness that runs the thresholds step

## Open questions
- none — the fork was decided at P4 (inputs#I2), and the two lists above were narrowed to the decided set: the
  heartbeat scripts, `xtask/src/pre_push.rs` and `pulse-app/tests/a11y_perf_workflow.rs` are not written by this
  chunk.

## Every place that reads the two lines that leave `ci-gates` (asked at P4, inputs#I2 item 3)
Sweep: `git grep -n -I -E 'ci-gates|ci_gates|CiGates|heartbeat-gap|perf-budget NEUTRAL|perf-budget: ' -- xtask
pulse-app/src pulse-app/tests scripts .github crates` (83 hits by `wc -l`), then `grep -n 'ci-gates'` over the masters and the
key files, then a count over the leaves.
- **No program reads the verb's output.** `xtask/src/pre_push.rs:278-281` reads its exit alone (`succeeds`); no
  script and no test parses a `ci-gates:` line.
- **The pre-push `ci-gates` stage** reads exit 0 today over its seed (four lines: records, panic, heartbeat
  "trivially passes", `perf-budget NEUTRAL`). After: exit 0 over the same seed, two lines (records, panic).
- **Pins:** `ci_workflow_invokes_ci_gates` (`pulse-app/tests/quality_gate_workflow.rs:443-450`) reads the step's
  presence; its failure message names the four arms. `ci_workflow_test_gates_no_continue_on_error` (`:241`) and
  the series pin (`:651-658`) read the step's command text. All three read the same after; the message is
  reworded.
- **Text in the tree that names the arms:** the verb's `about` (`xtask/src/main.rs:149`), the pre-push verb's
  `about` (`:253`, names the stage only), a comment in `run_perf_slo_load` (`:734-736`, "post-test p99 / max
  gates fire via run_ci_gates()", false today), the `INACTIVE state` comment (`:486-487`).
- **Master sentences** (`grep -n 'ci-gates'`): `architecture.md:260`;
  `registries/contracts/architecture/ci-cd-approach.md:3`; `test-plan.md:100`, `:496`, `:509`;
  `registries/contracts/test-plan/per-chunk-gate-discipline.md:32`; `obs-plan.md:499`, `:545`, `:559`, `:579`.
  The heartbeat bullet (`obs-plan.md:558`) and the zero-span bullet (`:556`) name the checks without the verb.
- **Leaves** (re-derived by the wrap's cascade): `.claude/docs/commands.md` (1), `obs-summary.md` (1),
  `tests-summary.md` (2), `.claude/rules/observability.md` (1), `.claude/rules/verification-harness.md` (5).
- **`invoke_heartbeat_check` and the grader keep their other callers** (`perf:load-profiles`, `perf:budget`): the
  two check scripts and `xtask/src/perf_budget.rs` are not edited.

## Scope premise closure
Each `[inferred]` bullet of `scope.md`, read against the above.
- Gates (1), (3), (4), (5), their placement, and the second and third `CARRY:` — **verified** as measured above;
  the tags are dropped and each bullet now names its reading.
- Gate (2) — **corrected in one clause:** the report does carry branch records (144 `BRF:0` / `BRH:0`), a zero
  total and no per-branch line, where the freight said it carried "no branch count".
- "`xtask` carries its own tests of the `ci-gates` arms" — **falsified:** no test names `run_ci_gates`,
  `invoke_heartbeat_check`, the nextest argument lists or either check script (`grep -n` over `xtask/src/*.rs`
  finds only their definitions and the `main` dispatch); the grader's arms alone are pinned
  (`xtask/src/perf_budget.rs`, 21 tests), and the workflow tests pin a step's presence, never an arm.
- The witness, the masters' sentences and the re-read of the first two clauses — **verified** (the witness as an
  obligation the tests and obs extracts state; the sites as the extracts list them; the two clauses as re-read
  above).
