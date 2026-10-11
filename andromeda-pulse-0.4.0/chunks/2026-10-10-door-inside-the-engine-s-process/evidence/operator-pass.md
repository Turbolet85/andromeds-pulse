# The operator pass (2026-10-11 UTC)

**Whose word.** The operator, 2026-10-11, given in the implement session after the P4 report and kept verbatim as
inputs#I4: "Run the operator pass in the plan order: hygiene, the pre-CI commit, the pre-push check, the merge-base
probe, the push, the CI read; then write the P-126 ref with the run id. The step Door refuses another account reads
a runner for the first time: if it is red, skipped, or the boot job or a test is red, stop and report what the job
kept whole - the arm own lines, the door.refused records; fix nothing on top. Start no skill - the wrap is mine to
call." Run by the implement agent on that word, under no skill and with no run dir. Each entry's line and its call's
summary line are copied verbatim, without the header. The same message accepted the implement report's deviations as
recorded.

## Entry 23 — hygiene, before the commit

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`:

```
root . · case exact · planes rust, ts · base HEAD (no --marker)
hygiene: clean — read 55 (runs 45 · evidence 4 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This file was written after the push and is not among the 55.

## The pre-CI commit

`2e4ee5ec2e5dcb0293a55881942a4ff97d146317` on `build/andromeda-pulse-0.4.0`, subject
`chore(2026-10-10-door-inside-the-engine-s-process): operator pre-CI commit`, parent `303fa9a8` (the chunk base).
79 files changed. Before it: the staged set held no path outside `.andromeda/`, `andromeda-pulse-0.4.0/`,
`.claude/session-handoff.md` and the sixteen paths research's two lists name (twelve modified, four new); no staged
deletion; the bindings file was not staged and its index copy held `"mcp":` once; `scripts/agent-run.sh` stayed mode
100755; `Cargo.lock` was not in the set. After it `git status --short` printed nothing.

## Entry 21 again, with its probe — the local pre-push check, on the committed tree

`gate.py run --plan … --only 21,22`:

```
 21 integration green · exit 0 · 121.76s · 1682489 B → 21.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
 22 probe       green · exit 0 · 0.0s · 0 B → 22.log · test ! -e target/pre-push/run/data/run/andromeda-pulse.pid… (162 chars)
entries 26 · green 2 · red 0 · recorded 0 · timeout 0 · not-run 24
```

From entry 21's log: `"verdict": "green"`, `"reason": "all-stages-ok"`,
`"head": "2e4ee5ec2e5dcb0293a55881942a4ff97d146317"`, `"tree": "beda4dbf304a39a92a7886a465eb8a1627cfde4d"`,
`"missing": []`; six stages, each `"ok": true` (script-modes 2 ms, source-lint 404 ms, npm 65 650 ms, clippy
1 064 ms, test 35 373 ms, ci-gates 18 343 ms). Its test stage read `3174 tests run: 3174 passed, 0 skipped`. Its
sixth stage, a console engine with the door built in, printed `cleanup: clean`, `engine-log: PASS`,
`ci-gates: zero-spans PASS (329 log records across 1 file(s))` and `ci-gates: zero-panic PASS`. The tree was clean
after it.

## Entry 24 — the merge-base probe, directly before the push

`gate.py run --plan … --operator 24`:

```
operator entry 24 · history tripwire: none of 7 forms matched — not a clearance
 24 probe       green · exit 0 · 1.01s · 0 B → 24.log · history: unmoved · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

## Entry 25 — the push

`gate.py run --plan … --operator 25`:

```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 2.4s · 135 B → 25.log · history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 303fa9a8→2e4ee5ec · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

The one ref that moved is the build branch's remote-tracking ref, `303fa9a8 → 2e4ee5ec`: the intended move. After
it `HEAD` and `origin/build/andromeda-pulse-0.4.0` both read `2e4ee5ec2e5dcb0293a55881942a4ff97d146317`.

## Entry 26 — the CI read of the pushed tip

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`
(exit 0, returned 2026-10-11T00:44:23Z):

```
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 53× over 1611 s
2e4ee5ec2e5d verdict: green · checks 7/7 · wall 1591 s · runs secret-scan#38097928018 completed/success ci#38097928056 …
runs: secret-scan#38097928018 pull_request completed/success · ci#38097928056 pull_request completed/success
checks: a11y (ubuntu-22.04) · boot smoke (ubuntu-22.04) · coverage gate (line ≥75% / function ≥85%) · gitleaks
  lint / test (ubuntu-22.04) · mcp-server tests (ubuntu-22.04) · supply-chain (audit + deny + auditable)
```

A jobs read after the run closed (`gh api …/actions/runs/38097928056/jobs`): the run is `pull_request`, attempt 1,
`completed` / `success`, head `2e4ee5ec…`, pull request #40, built on the merge commit `6b553ea4de20`; all six jobs
`success` (a11y 114347625013 · boot smoke 114347625052 · lint / test 114347625062 · coverage 114347625090 ·
supply-chain 114347625109 · mcp-server tests 114347625166).

## The step `Door refuses another account` — a second account read for the first time

Job `lint / test` (114347625062): step 22 `cargo xtask test` success, step 23 `Door refuses another account`
success. The job's log was fetched with `gh api --allow-escape-sequences …/actions/jobs/114347625062/logs` to the
session's scratch dir (2 727 192 bytes, 16 272 lines after the colour escapes and the leading timestamps were
stripped; known-present control `cargo xtask verify:capability-matrix`: 2 lines). Every line of it holding the
arm's prefix, three: the step's own script line that greps for the sentence, and the arm's two lines. The step's
output around them, as the job kept it:

```
    Starting 1 test across 1 binary (4 tests and 142 binaries skipped)
        PASS [   0.513s] (1/1) pulse-app::integration_door another_account_is_refused_at_each_of_three_layers
  stdout ───

    running 1 test
    door other-account arm: readings as [nobody, root]: directory 0700 [Kernel(13), Closed], socket 0600 [Kernel(13), Closed], both opened [Closed, Closed]
    door other-account arm: directory refused, socket mode refused, account check refused
    test another_account_is_refused_at_each_of_three_layers ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.51s

     Summary [   0.514s] 1 test run: 1 passed, 4 skipped
```

What the readings say, layer by layer:
- **The directory, 0700.** `nobody`: the kernel refused the connect, errno 13. Root: the connect went through and
  the door closed it with no byte.
- **The socket, 0600, the directory opened.** `nobody`: the kernel refused the connect, errno 13. This is the mode
  rule research could not read from a local source, now measured on the runner's kernel. Root: through, and closed
  by the door with no byte.
- **Both opened.** `nobody` and root: each through, each closed by the door with no byte.

Root is not held by a file mode, as the implement report said it would not be; at every layer the door's account
check is what closed it. The arm ran the same way inside `cargo xtask test` (`PASS [ 1.115s] ( 854/3174)`).

**The `door.refused` records.** The arm passed, so its own assertions held on the runner: one `door.refused` record
(`reason` `other_account`, `peer` `other`, WARN, nothing redacted) per client that reached the door, which is four
by the readings above (root three times, `nobody` once); exactly one `door.call`, the owner's `query_logs`,
`outcome` `ok`; no client of another account got a byte; the owner's call returned the log record the test had sent
over OTLP. The records themselves are in no kept artifact: the arm's engine wrote them under a `TempDir` on the
runner and the arm read them there. Nothing of them can be quoted here beyond what the arm asserted.

## The other jobs, as read

- **boot smoke** (114347625052): `Boot pulse-app smoke`, `Boot series (equal source)` and `Console engine cycle`
  each success. From its log: `"verdict": "all-settled"`; the cycle's `cleanup: clean`, `engine-log: PASS`,
  `"error_records": 0`, `"verdict": "pass"`, `"witness_file": "absent"`. The cycle's engine has the door built in.
- **coverage** (114347625090): `Line:     35521/39153 = 90.7% (threshold 75%)`,
  `Function: 3750/4186 = 89.6% (threshold 85%)`.
- **lint / test**: the workspace run read `3174 tests run: 3174 passed, 0 skipped`.

## Artifacts of `ci#38097928056`

`logs-boot-Linux` 11687162979 · `logs-engine-Linux` 11686728619 · `capability-drift-Linux` 11687102486 ·
`logs-perf-samples-Linux` 11686558302 · `coverage-linux` 11686388506 · `playwright-a11y-report-Linux` 11686792711 ·
`a11y-violations-Linux` 11686762648; each expires 2026-10-25. None was downloaded.

## The P-126 ref

Written after the CI read with `matrix.py implement --id P-126`: `planned → implemented`, the ref naming the arm,
`ci#38097928056`, the tip `2e4ee5ec`, the merge commit and the step's two lines.

## Limits

- One CI run, attempt 1. The second account was read on one runner image, twice in that run (the workspace run and
  the step).
- The engine cycle's uploaded log (`logs-engine-Linux`) was not downloaded, so the door's boot record on the runner
  was not read; the cycle step's success and its printed verdict were.
- The mcp-server, a11y and supply-chain jobs were read by their conclusions alone.
- The tree is not clean after this pass: the ledger (`verification-matrix.json`), the ledger tool's trail in the
  implement run dir and this file changed after the push. They ride the wrap's commit.
