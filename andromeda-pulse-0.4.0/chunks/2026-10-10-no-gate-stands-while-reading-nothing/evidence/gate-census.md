# Gate census — every gating step of `ci#38042949735`, with the line that says what it read

The run: pull-request event, attempt 1, on the pushed tip `d99d2dcb` (merge commit `45ea78a7eb66`), six jobs, every
step of every job `success` (`gh api …/actions/runs/38042949735/jobs`, read once). Each line below was read from
that job's log, fetched whole with `--allow-escape-sequences` (lint-test 14762 lines, coverage 6230, mcp-test 8440,
a11y 3110, supply-chain 3118, boot 2960; none empty). `:{n}` is the line's number in that log. A runner path is
written `{runner}`.

In every job the preparation steps (harden runner, the data-dir export, checkout, the toolchain install, the cargo
cache, the tool installs, the system libraries, Node, `npm ci`, `npm run build`) read `success`; they are not
listed per job.

## lint-test — job 114186682610

| step | what it read |
|---|---|
| `cargo fmt --check` | printed nothing; exit 0 over the tree's sources |
| No Cyrillic in sources | `"files_scanned": 492` · `"verdict": "clean"` (:1601, :1604) |
| `cargo clippy … -D warnings` | checked the workspace; ``Finished `dev` profile … in 32.86s`` (:1637) |
| `cargo xtask typecheck` | `tsc --noEmit` printed nothing; exit 0 over the webview sources |
| `cargo xtask quarantine-tracking` | `quarantine-tracking-check: PASS (0 quarantine(s) across 312 file(s))` (:1670) |
| `cargo xtask capability-drift` | `capability-drift: clean (0 missing, 0 extra)` (:1686), its staged half `check:staged-artifacts: staged-clean (exit 0)` (:1704) |
| `cargo xtask check:staged-artifacts` | `"arm": "staged-clean"` · `"verdict": "green"` (:1720, :1733) |
| `cargo xtask capability-widening-check` | `capability-widening-check: clean (0 violations across 3 inspected)` (:1751) |
| `cargo xtask verify:capability-matrix` | `verify:capability-matrix: clean (60/60 capabilities, 0 violation(s))` (:1768) |
| `cargo xtask test` | `Starting 2908 tests across 128 binaries` (:1804) · `2908 tests run: 2908 passed, 0 skipped` (:10770) |
| `cargo xtask perf:slo-load` | `Starting 1 test across 1 binary` (:13698) · `1 test run: 1 passed, 0 skipped` (:13705) |
| `cargo nextest run --workspace --profile perf-samples --no-tests=fail` | `Starting 1 test across 1 binary` (:13725) · `1 test run: 1 passed, 0 skipped` (:13728) |
| `cargo xtask perf:budget … --require memory,snapshot` | `perf-budget: graded 120 record(s) across 1 file(s)` (:13744) · `memory max 7680000 B <= 512000000 B (n=3, populated 3) PASS` (:13746) · `snapshot p99 97.6 ms <= 500 ms (n=50) PASS` (:13747) · `perf-budget: PASS` (:13748); beside them `frame: cannot-evaluate: 0 samples, no adapter record in this log` (:13745), an arm the step does not require |
| Upload perf-samples logs | `With the provided path, there will be 1 file uploaded` (:13771); artifact `logs-perf-samples-Linux`, id 11666219570 |
| Upload capability-drift report | `… there will be 1 file uploaded` (:13805); artifact `capability-drift-Linux`, id 11667035184 |

## coverage — job 114186682727

| step | what it read |
|---|---|
| `cargo xtask test:coverage` | `Starting 2908 tests across 128 binaries` (:2305) · `2908 tests run: 2908 passed, 0 skipped` (:5215) · `Finished report saved to lcov.info` (:5217) |
| Enforce coverage thresholds | `Line:     33992/38252 = 88.9% (threshold 75%)` (:5252) · `Function: 3565/4055 = 87.9% (threshold 85%)` (:5253); no branch line |
| Upload coverage artifact | `… there will be 1 file uploaded` (:5276); artifact `coverage-linux`, id 11667222245 |

## mcp-test — job 114186682766

| step | what it read |
|---|---|
| `cargo nextest run --workspace --features mcp-server --profile ci --no-tests=fail` | `Starting 2946 tests across 129 binaries` (:1602) · `2946 tests run: 2946 passed, 0 skipped` (:4550) |

The runner's cargo-nextest took the fail-on-empty flag as spelled; the step ran and exited 0 over 2946 tests.

## a11y — job 114186682773

| step | what it read |
|---|---|
| Install Playwright chromium | `success`; it installs, it gates nothing of the tree |
| `cargo xtask test:a11y` | `verify-contrast: 12 pairs evaluated, all critical pairs pass` (:1719) · `41 passed (18.9s)` (:1725) · `lighthouse-runner: 7 surfaces audited; all ≥90` (:1726) · `7/7 URLs passed` (:1736) · `aggregator: 6 violation tuple(s) across 7 surfaces` (:1737) · `regression-detector: no new violations vs baseline (0/0)` (:1738), the baseline the one committed in the tree |
| Upload a11y violations summary + regression set + contrast report | `… there will be 5 files uploaded` (:1766); artifact `a11y-violations-Linux`, id 11666424149. Three path members; which member each of the 5 files matched was not read (outside the sentence, owner `Window's gates retired`) |
| Upload Playwright a11y report | `… there will be 1 file uploaded` (:1800); artifact `playwright-a11y-report-Linux`, id 11666289198 |

## supply-chain — job 114186682774

| step | what it read |
|---|---|
| `cargo audit` | `Loaded 1296 security advisories` (:704) · `Scanning Cargo.lock for vulnerabilities (916 crate dependencies)` (:706); exit 0 (warnings listed, no vulnerability) |
| `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok` (:1007) |
| Cranelift-only WASM backend assertion | `cargo tree -p wasmtime \| grep -q "cranelift" \|\| exit 1` printed nothing of its own; exit 0. It fails on an empty stream by construction |
| `cargo xtask check:npm-supply-chain` | from its verdict object: license `"checked": 883`, `"violations": []` · advisory `"distinct": 2`, both excepted, `"unexcepted": []` · `"arm": "green-with-dispositions"` · `"verdict": "green"` (:1911) |
| `cargo auditable build --workspace --release` | built the workspace; ``Finished `release` profile [optimized] … in 4m 03s`` (:2088) |

## boot — job 114186682779

| step | what it read |
|---|---|
| `cargo build --workspace --release --features mcp-server` | built the workspace; ``Finished `release` profile [optimized] … in 4m 19s`` (:1528) |
| Build the exit witness | `cc -shared …` printed nothing; exit 0 (the library the smoke then reports `loaded`) |
| Boot pulse-app smoke | `boot: ready (PID=6993, …)` · the settle verdict `"verdict": "settled"`, `"windows_settled": 4`, `"exit_witness": "loaded"`, `"display": "reachable"`, `"session_bus": "reachable"` (:1577, :1578) · the status verdict `"verdict": "running-healthy"` (:1586) · `cleanup: clean` (:1588) |
| Boot series (equal source) | seven cycles, each `settled` with 4 windows, `running-healthy`, `cleanup: clean`; the verdict `"boots": 7`, `"settled": 7`, `"ended": 0`, `"other": 0`, `"verdict": "all-settled"` (:1780, :1857, :1858); eight per-boot entries, the smoke's own as ordinal 1 |
| `cargo xtask ci-gates` | `ci-gates: zero-spans PASS (110 log records across 1 file(s))` (:1875) · `ci-gates: zero-panic PASS` (:1876); two lines, no heartbeat or perf-budget line |
| Upload boot logs artifact | `… there will be 42 files uploaded` (:1900); artifact `logs-boot-Linux`, id 11666680950 (not opened) |

## Read over all six logs

- The four tokens of entry 30 (`NEUTRAL`, `trivially passes`, `Branch: `, the pass-on-empty flag): 0 lines, counted
  again over these fetched files.
- Steps that printed no line of their own, each still deciding its job by its exit over a non-empty input: `cargo
  fmt --check`, `tsc --noEmit`, the Cranelift assertion, the exit-witness build.
- One line that says an arm read nothing while its step passed: the `frame: cannot-evaluate: 0 samples` line of
  the `lint-test` budget step. The step's exit is decided by its two required arms, which each read samples
  (n=3 and n=50); the frame arm is graded on no runner (obs-plan §10). It is not one of the acceptance's tokens.
