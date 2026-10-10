# The operator pass (2026-10-10)

**Whose word.** The operator, 2026-10-10, given in the implement session after the P4 report and kept verbatim as
inputs#I4: "Run the operator pass, entries 20 to 25 in the plan order: hygiene, the pre-CI commit, the pre-push
check, the merge-base probe, the push, the CI read, the two artifact reads. If the boot job, the cycle step or a
new test is red on the runner, stop and report what the job kept whole; fix nothing on top. Start no skill - the
wrap is mine to call." Run by the implement agent on that word, under no skill and with no run dir. Each entry's
line and its call's summary line are copied verbatim, without the header.

Nothing read red, so the stop did not apply and nothing was fixed on top.

## Entry 20 — hygiene, before the commit

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`:

```
root . · case exact · planes rust, ts · base HEAD (no --marker)
hygiene: clean — read 49 (runs 39 · evidence 4 · inputs 6) · trails 16 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This file was written after the push and is not among the 49.

## The pre-CI commit

`d30b70375eac1c7d5b41524fcd56d359feaa0d60` on `build/andromeda-pulse-0.4.0`, subject
`chore(2026-10-10-engine-harness-proofs-completed): operator pre-CI commit`, parent `514fbe57` (the chunk base).
68 files changed. Before it the staged bindings held `"mcp":` once and were identical to the chunk base's; no
staged path lay outside `.andromeda/`, `andromeda-pulse-0.4.0/`, `.claude/session-handoff.md` and the twelve source
paths research's two lists cover (nine modified, three new under `xtask/tests/`); no staged deletion.
`scripts/agent-run.sh` stayed mode 100755. After it `git status --short` printed nothing.

## Entry 18 again, with its probe — the local pre-push check, on the committed tree

`gate.py run --plan … --only 18,19`:

```
 18 integration green · exit 0 · 110.44s · 1648992 B → 18.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
 19 probe       green · exit 0 · 0.0s · 0 B → 19.log · test ! -e target/pre-push/run/data/run/andromeda-pulse.pid… (162 chars)
entries 25 · green 2 · red 0 · recorded 0 · timeout 0 · not-run 23
```

From entry 18's log: `"verdict": "green"`, `"reason": "all-stages-ok"`,
`"head": "d30b70375eac1c7d5b41524fcd56d359feaa0d60"`, `"tree": "ece08c1570f58b92aac1b389b560f79644a83d0b"`,
`"missing": []`; six stages, each `"ok": true` (script-modes 2 ms, source-lint 451 ms, npm 66 689 ms, clippy
1 151 ms, test 22 285 ms, ci-gates 18 827 ms). Its test stage read `3112 tests run: 3112 passed, 0 skipped`. Its
sixth stage printed `cleanup: clean`, `engine-log: PASS`, `ci-gates: zero-spans PASS (335 log records across 1 file(s))`
and `ci-gates: zero-panic PASS`. The tree was clean after it.

## Entry 21 — the merge-base probe, directly before the push

`gate.py run --plan … --operator 21`:

```
operator entry 21 · history tripwire: none of 7 forms matched — not a clearance
 21 probe       green · exit 0 · 0.39s · 0 B → 21.log · history: unmoved · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

The remote `main` read `178ebac56e26…` directly after the probe; the build branch contains it. The remote build
branch then still stood at `514fbe57`.

## Entry 22 — the push

`gate.py run --plan … --operator 22`:

```
operator entry 22 · history tripwire: git
 22 probe       green · exit 0 · 2.26s · 135 B → 22.log · history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 514fbe57→d30b7037 · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

The move is the intended one: `git ls-remote origin refs/heads/build/andromeda-pulse-0.4.0` read
`d30b70375eac1c7d5b41524fcd56d359feaa0d60` after it, equal to the local `HEAD` (clock read after it:
2026-10-10T19:18:54Z). No other ref moved.

## Entry 23 — the CI read of the pushed tip

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`:

```
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 56× over 1705 s
d30b70375eac verdict: green · checks 7/7 · wall 1684 s · runs secret-scan#38079305747 completed/success ci#38079305603 …
runs: secret-scan#38079305747 pull_request completed/success · ci#38079305603 pull_request completed/success
checks: a11y (ubuntu-22.04) · boot smoke (ubuntu-22.04) · coverage gate (line ≥75% / function ≥85%) · gitleaks
  lint / test (ubuntu-22.04) · mcp-server tests (ubuntu-22.04) · supply-chain (audit + deny + auditable)
```

**The chunk's CI verdict is `ci#38079305603` on `d30b7037`: green, 7 of 7, attempt 1.**

A jobs read after the run closed (clock: 2026-10-10T19:47:44Z): run `completed` / `success`, `run_attempt` 1,
event `pull_request`, head `d30b70375eac1c7d5b41524fcd56d359feaa0d60`; all six jobs of the `ci` workflow
`completed/success` (a11y `114292749124`, lint / test `114292749263`, coverage gate `114292749265`, boot smoke
`114292749292`, supply-chain `114292749315`, mcp-server tests `114292749324`). In the boot job every step read
`success`: `Boot pulse-app smoke` (step 14), `Boot series (equal source)` (15), `cargo xtask ci-gates` (16),
`Console engine cycle` (17, 19:30:28Z to 19:31:55Z, 87 s), the two uploads (18, 19).

Artifacts of the run: `logs-boot-Linux` (`11680211947`, 38835 B) and `logs-engine-Linux` (`11679807527`, 5493 B),
both expiring 2026-10-24.

## The new tests on the runner (from the `lint / test` job's log)

Read from the job log fetched to a file with colour escapes and line timestamps stripped; every count is the
log's own summary line or a count of name lines halved (each `PASS` line stands twice in the log).

- `3112 tests run: 3112 passed, 0 skipped`; 0 lines of `FAIL`, `TIMEOUT`, `SIGABRT` or `SIGSEGV`.
- `xtask::agent_run_shell`: 19 tests passed. `xtask::engine_cycle_refusals`: 6 tests passed, the cycle on a real
  engine among them (`a_cycle_that_fails_on_a_real_engine_ends_the_engine_it_started`, 2.1 s): the runner's test
  job holds no release engine, and the pin took the engine built beside it.
- The three spawned arms passed (`a_rejected_grpc_port_…` 0.19 s, `the_engine_listens_on_its_two_loopback_receivers_and_on_nothing_else`
  0.22 s, `the_corpus_key_lock_file_sits_in_the_directory_the_environment_gives` 0.44 s): the census read the same
  on the runner as on the dev host, since the arm asserts it.
- `pre_push::tests::end_cycle_ends_the_process_the_stage_data_dir_s_pid_file_names` passed (1.07 s).

The `coverage gate` job's thresholds step printed `Line: 34620/38214 = 90.6% (threshold 75%)` and
`Function: 3623/4056 = 89.3% (threshold 85%)`; its instrumented run read `3112 tests run: 3112 passed, 0 skipped`.
The `mcp-server tests` job read `success` (its summary line was not read).

## The boot job under the changed readiness (from its log)

Nine `boot: ready (PID=…` lines: the smoke's boot, the series' seven and the engine cycle's. Seven
`series-cycle boot=0 settled=0 status=0 cleanup=0` lines, nine `cleanup: clean` lines,
`ci-gates: zero-spans PASS (111 log records across 1 file(s))` and `ci-gates: zero-panic PASS`. Every window boot
on the runner read ready over its own bound records.

The cycle step's own output, with the boot verb's data dir respelled `{runner temp}` and nothing else changed:

```
boot: ready (PID=15637, data_dir={runner temp}/andromeda-pulse-engine-data)
  OTLP gRPC:    127.0.0.1:24317
  OTLP HTTP:    127.0.0.1:24318
"verdict": "settled"            (two ticks of each engine target, memory_samples_populated 1)
"verdict": "running-healthy"    (program console)
cleanup: clean
engine-log: family PASS (335 records across 1 file(s))
engine-log: program PASS (one app.boot.engine record, reading console)
engine-log: panic PASS (no app.panic.fatal record at ERROR)
engine-log: heartbeat-gap PASS (ingest.tick max gap 15001 ms (n=2); buffer.tick max gap 15000 ms (n=2); connection.tick max gap 15000 ms (n=2))
engine-log: progress PASS (2 buffer.tick record(s), largest rows_ingested 810)
engine-log: process-end PASS (one app.exit record, the last of the family)
engine-log: budget PASS (memory max 138240 B <= 512000000 B, n=2, populated 1)
engine-log: budget frame and snapshot not graded (a console engine has no producer for either)
engine-log: PASS
{"boot": 0, "check": 0, "cleanup": 0, "error_records": 0, "settled": 0, "status": 0, "verdict": "pass", "witness_file": "absent"}
```

The two verdict objects between the boot lines and `cleanup: clean` are shortened to their verdict member here;
the last object is the cycle verdict whole, on one line.

## Entry 24 — the boot artifact

`gate.py run --plan … --operator 24 --id 38079305603`:

```
operator entry 24 · history tripwire: none of 7 forms matched — not a clearance
 24 probe       green · exit 0 · 1.28s · 3302 B → 24.log · history: unmoved · d="target/boot-smoke/ci-<id>-attempt-1" && gh run download… (379 chars)
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

From its log, read whole:

- The series verdict: `"boots": 7`, `"settled": 7`, `"ended": 0`, `"other": 0`, `"verdict": "all-settled"`; eight
  per-boot rows, ordinal 1 (`"cycle": "smoke"`) to 8, each `"verdict": "settled"`, `"windows_settled": 4`,
  `"ended": null`, `"app_exit_record": "absent"`, `"exit_witness": "loaded"`.
- The smoke's settle verdict: `"verdict": "settled"`, `"windows_settled": 4`, `"display": "reachable"`,
  `"session_bus": "reachable"`, `"exit_witness": "loaded"`, `"app_exit_record": "absent"`, `"ended": null`.
- Eight witness files, each one line and nothing else: `{"kind":"loaded","pid":…,"comm":"pulse-app"}` (pids 7334,
  8269, 8592, 8914, 9237, 9567, 9892, 10212). No `end` line, no line beyond the closed shape.
- The window boot record on the runner: `app.boot.engine`, INFO, `"program":"window"`,
  `"interpretation":"model"`, `"reason":"window_runs_model"`; its `git.commit.sha` reads `b61f830d…`, the
  pull-request merge commit, not the pushed tip.
- No `xvfb.log` of the eight holds a byte.

Kept under `target/boot-smoke/ci-38079305603-attempt-1/` (git-ignored): the smoke's log family, `boot.log`,
`build.log`, `boot-series.json`, `harness-settled.json`, `exit-witness.jsonl`, `xvfb.log`, and seven
`series/boot-{2..8}/` directories of five files each.

## Entry 25 — the engine artifact, graded again on this host

`gate.py run --plan … --operator 25 --id 38079305603`:

```
operator entry 25 · history tripwire: none of 7 forms matched — not a clearance
 25 probe       green · exit 0 · 1.47s · 675 B → 25.log · history: unmoved · d="target/engine-cycle/ci-<id>-attempt-1" && gh run downlo… (196 chars)
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

The entry asserts the exit alone; its log, whole:

```
engine-log: family PASS (335 records across 1 file(s))
engine-log: program PASS (one app.boot.engine record, reading console)
engine-log: panic PASS (no app.panic.fatal record at ERROR)
engine-log: heartbeat-gap PASS (ingest.tick max gap 15001 ms (n=2); buffer.tick max gap 15000 ms (n=2); connection.tick max gap 15000 ms (n=2))
engine-log: progress PASS (2 buffer.tick record(s), largest rows_ingested 810)
engine-log: process-end PASS (one app.exit record, the last of the family)
engine-log: budget PASS (memory max 138240 B <= 512000000 B, n=2, populated 1)
engine-log: budget frame and snapshot not graded (a console engine has no producer for either)
engine-log: PASS
```

The same nine lines the step printed on the runner. The artifact holds `agent-latest.jsonl.2026-10-10`,
`boot.log` and `build.log`, kept under `target/engine-cycle/ci-38079305603-attempt-1/logs/` (git-ignored). The
artifact carries no `run/` directory, so the `process-end` arm read the log's own exit record here, as it did
on the runner.

## Limits of this pass

- One CI run read, attempt 1, on the pre-CI commit. The wrap's own commit is not covered by it.
- The runner's pull-request run builds the tip merged with `main`; the local pre-push check read the tip alone.
- No red cycle, no taken harness port and no foreign listener was read on a runner: every boot there was the
  healthy path. The refusals and the `foreign` reading are pinned by the tests that passed on it.
- The new suites ran once on a runner (and once more under coverage). One green run does not bound a rare
  failure of the port-using tests there.
