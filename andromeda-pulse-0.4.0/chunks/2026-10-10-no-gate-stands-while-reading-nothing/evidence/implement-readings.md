# Readings taken at /implement, beside the gate block

The gate block's own record is the gate trail in the implement run dir
(`.andromeda/runs/2026-10-10T09-32-50Z-implement/`). Its summary line, one run call on the final tree:

```
entries 34 · green 24 · red 0 · recorded 0 · timeout 0 · not-run 10
```

The ten not-run entries are the ten keyed `leg = 'operator'` (25 to 34). No entry was deferred, skipped or voided.

## What the entries the acceptance cites printed

- Scope guard against the chunk base (entry 3): exit 0, no output.
- Parsed workflow against the base (entry 6): `4 True True True coverage`, the four changed steps named
  `coverage/Enforce coverage thresholds | lint-test/cargo nextest run --workspace --profile perf-samples |
  lint-test/cargo xtask test (no-tests=pass at Foundation epoch) | mcp-test/cargo nextest run --features mcp-server`
  (base names).
- The verb over a data dir that does not exist (entry 11): exit 2,
  `::error::ci-gates: cannot-evaluate (no agent-latest.jsonl* file in the log dir)`.
- The verb over one seeded record (entry 12): exit 0,
  `ci-gates: zero-spans PASS (1 log records across 1 file(s))` and `ci-gates: zero-panic PASS`, nothing else.
- The quarantine script over a root with none of its search dirs (entry 13): exit 1, four
  `::error::quarantine-tracking-check: search dir {dir} is missing` lines.
- The quarantine check over the tree (entry 14): `quarantine-tracking-check: PASS (0 quarantine(s) across 312
  file(s))` (311 at P3, plus the new `xtask/src/ci_gates.rs`).
- Workspace suite (entry 21): `2908 tests run: 2908 passed, 0 skipped`, 128 binaries.
- Local pre-push check (entry 24): `"verdict": "green"`, `"reason": "all-stages-ok"`, six stages `ok: true`
  (`script-modes`, `source-lint`, `npm`, `clippy`, `test`, `ci-gates`), `head` the chunk base sha; its `test` stage
  ran 2908 tests under the fail-on-empty flag; its `ci-gates` stage read its seed and exited 0.
- Bindings close (entry 23): exit 0; the tracked bindings read unchanged after every run of the session.

## The count check

18 tests were added: 6 in `pulse-app/tests/quality_gate_workflow.rs`, 6 in `xtask/src/main.rs`
(`empty_input_tests`), 6 in `xtask/src/ci_gates.rs`. The suite ran 2908; `cargo xtask test` ran 2890 on
`ci#38038281709` (research.md). 2890 + 18 = 2908. A limit: the base count was not read on this host before the
edits; the 2890 is the runner's.

## Two readings no listed entry makes

Both run by hand on the final tree, each exit read from the bare command.

- **An empty selection through the verb.** `cargo xtask test -- -E 'test(zz_no_such_test_for_this_chunk)'`:
  `Starting 0 tests across 128 binaries (2908 tests … skipped …)`, `0 tests run: 0 passed, 2908 skipped`,
  `error: no tests to run`, the verb's exit 1. (A first spelling without the `--` separator was refused by the
  argument parser, exit 2, and ran nothing.)
- **The edited `perf-samples` line as the workflow spells it**, then its budget step.
  `cargo nextest run --workspace --profile perf-samples --no-tests=fail`: `1 test run: 1 passed, 0 skipped`, exit 0.
  `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot`: exit 0,

  ```
  perf-budget: graded 120 record(s) across 1 file(s)
  perf-budget: frame: cannot-evaluate: 0 samples, no adapter record in this log
  perf-budget: memory max 7680000 B <= 512000000 B (n=3, populated 3) PASS
  perf-budget: snapshot p99 64.2 ms <= 500 ms (n=50) PASS
  perf-budget: PASS
  ```

  The producer binds an ephemeral loopback port, not 4317 or 4318.

## Not read here

- The `mcp-test` line (`--features mcp-server … --no-tests=fail`) was not run on this host; the pull-request run
  reads it, on the runner's cargo-nextest 0.9.133.
- The coverage job was not run on this host; its thresholds step is read by the witness over constructed reports
  and by the pull-request run over the real one.
- `xtask/ci/quarantine-tracking-check.ps1` was parsed (0 parse errors) and not run.
- P-128's ref is not written: its acceptance is read on the pull-request run of the pushed commit, named in
  `operator-pass.md`, which the operator pass produces.
