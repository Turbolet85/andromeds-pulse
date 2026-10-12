# The five other gates — 2026-10-10-no-ci-step-reads-nothing

Plan step 8. Written by /implement on 2026-10-10 from `research.md` ("The newest run, read step by step") and
`scope.md` ("Read at take-up and at P3, not stated by the entry"). **No new measurement was made for this file and
none of the five was edited** (inputs#I2 item 4; the plan's Constraints). Each reading below is the one phase took,
on the run named beside it.

They are gates of the `ci` workflow that pass while reading nothing and are neither a baseline comparison nor an
artifact upload. P-128's acceptance closes "No gate stands while reading nothing"; the operator's answer is that the
sentence reaches these five, so P-128 is not claimed by this chunk. The entry that closes the sentence is minted at
this chunk's wrap and carries them.

The runs: `ci#38031822696` on `6df9e95c` (pull-request event, attempt 1, green, 7 of 7 checks) and
`ci#38026637514` attempt 1 on `4e61553b`; the workflow file is byte-identical at the two commits (research.md).

Coordinates are given twice: as phase read them at the chunk base `6df9e95c`, and where the same line stands in the
tree after this chunk's edits (found by `grep`, a locating read of the file, not a reading of any gate).

## 1. The coverage thresholds step passes when nothing is tracked

- **Where:** job `coverage`, step `Enforce coverage thresholds`. Base `ci.yml:565-568`; after this chunk
  `ci.yml:519-522`.
- **What it does:** when `lcov.info` tracks 0 lines and 0 functions the step prints `Coverage gate: 0 lines / 0
  functions tracked (…)` and exits 0.
- **Measured reading:** read from the step's text at take-up. The arm did not fire on either run: both tracked
  lines and functions (`Line: 33992/38252 = 88.9%`, `Function: 3565/4055 = 87.9%` on `ci#38031822696`, the same lines
  on `ci#38026637514` attempt 1).

## 2. The same step's branch arm passes over no branch count

- **Where:** the same step. After this chunk the line that prints it is `ci.yml:527`.
- **Measured reading:** `Branch: 0/0 = 100.0% (threshold 70%)` on `ci#38031822696` and on `ci#38026637514`
  attempt 1. The uploaded `lcov.info` carries no branch count, so the 70 % branch gate passes over nothing while the
  job's name states it (`coverage gate (line ≥75% / branch ≥70% / function ≥85%)`).

## 3. `--no-tests=pass`

- **Where:** job `mcp-test`, step `cargo nextest run --features mcp-server` (base `ci.yml:237`; after this chunk
  `ci.yml:199`), and inside `cargo xtask test` and `cargo xtask test:coverage` (base `xtask/src/main.rs:466` and
  `:495`; after this chunk `:440` in `run_cargo_nextest` and `:469` in `run_cargo_llvm_cov`).
- **What it does:** a run that selects no test passes.
- **Measured reading:** read from the step's and the two functions' text at take-up. No run was read on which a
  selection was empty.
- **Seen while locating, not measured:** `xtask/src/main.rs` holds the same literal twice more, at `:749` in
  `run_perf_slo_load` and `:780` in `run_perf_load_profiles` (base `:775`, `:806`). Phase did not name them; what
  either selects on a run was not read here.

## 4. `cargo xtask ci-gates` in the `boot` job

- **Where:** job `boot`, step `cargo xtask ci-gates` (after this chunk `ci.yml:363`).
- **Measured reading, `ci#38031822696`:** `zero-spans PASS (118 log records across 1 file(s))` ·
  `heartbeat-gap-check: max gap 0ms in  (threshold 45000ms) PASS` · `perf-budget: memory NEUTRAL — populated 0 of 1`
  · `snapshot NEUTRAL` · `perf-budget NEUTRAL`.
- **What reads nothing:** the perf-budget memory and snapshot arms read no sample (stated in obs-plan §10 as
  designed); the heartbeat gap check passes with a max gap of 0 ms over a boot that lives about five seconds, shorter
  than one 15 s tick interval.

## 5. The zero-span build failure has no step in `lint-test`

- **Where:** obs-plan §10, "Build fails if `xtask test` produces zero spans in log file".
- **Measured reading:** no step of the `lint-test` job makes that check; the zero-span check runs only inside
  `ci-gates`, in the `boot` job (the `zero-spans PASS` line above, read on `ci#38031822696`).

## Settled here, so not among the five

Scope's sixth line of the same list, the comparison tools' own neutral arms, is closed by this chunk: the two
regression scripts are deleted with their comparisons, and the regression detector's "baseline not found; treating
as empty baseline" arm is gone (an absent baseline ends the run with exit 1; `evidence/mutation-checks.md`, 4b).
