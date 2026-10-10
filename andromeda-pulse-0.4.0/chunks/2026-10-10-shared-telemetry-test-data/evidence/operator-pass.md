# The operator pass (2026-10-10)

**Whose word.** The operator, 2026-10-10, given in the implement session after the P4 report and kept verbatim as
inputs#I5: "If it reads green, go on to the operator pass, entries 23 to 26 in the plan order: hygiene, the pre-CI
commit, the pre-push check, the merge-base probe, the push, the CI read. If the boot job, the cycle step or a test
is red on the runner, stop and report what the job kept; fix nothing on top. Start no skill - the wrap is mine to
call." Run by the implement agent on that word, under no skill and with no run dir. Each entry's line and its
call's summary line are copied verbatim, without the header.

"It" is entry 4, re-fired green after the operator edit of the plan (`evidence/operator-edit-plan.md`).

## Entry 23 — hygiene, before the commit

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`:

```
root . · case exact · planes rust, ts · base HEAD (no --marker)
hygiene: clean — read 53 (runs 41 · evidence 5 · inputs 7) · trails 15 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This file was written after the push and is not among the 53.

## The pre-CI commit

`d9651126f00614c5c10825b292c17cfe4be94277` on `build/andromeda-pulse-0.4.0`, subject
`chore(2026-10-10-shared-telemetry-test-data): operator pre-CI commit`, parent `50216d7e` (the chunk base).
81 files changed. Before it: the staged set held no path outside `.andromeda/`, `andromeda-pulse-0.4.0/`,
`.claude/session-handoff.md` and the paths research's two lists cover (the root `Cargo.toml` and `Cargo.lock`,
three member manifests, ten source files modified, the four files of `crates/test-support/` and
`xtask/src/test_support_edges.rs` new); no staged deletion; the bindings file was not staged and its index copy
held `"mcp":` once; `scripts/agent-run.sh` stayed mode 100755. After it `git status --short` printed nothing.

## Entry 21 again, with its probe — the local pre-push check, on the committed tree

`gate.py run --plan … --only 21,22`:

```
 21 integration green · exit 0 · 108.0s · 1659424 B → 21.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
 22 probe       green · exit 0 · 0.0s · 0 B → 22.log · test ! -e target/pre-push/run/data/run/andromeda-pulse.pid… (162 chars)
entries 26 · green 2 · red 0 · recorded 0 · timeout 0 · not-run 24
```

From entry 21's log: `"verdict": "green"`, `"reason": "all-stages-ok"`,
`"head": "d9651126f00614c5c10825b292c17cfe4be94277"`, `"tree": "538150b3c03b0603f80527f7780fbaed6b3ae3d8"`,
`"missing": []`; six stages, each `"ok": true` (script-modes 2 ms, source-lint 398 ms, npm 65 072 ms, clippy
1 134 ms, test 21 644 ms, ci-gates 18 814 ms). Its test stage read `3132 tests run: 3132 passed, 0 skipped`, under
the stage's cleared environment: the port claim found `target/test-ports/` with no variable. Its sixth stage
printed `cleanup: clean`, `engine-log: PASS`, `ci-gates: zero-spans PASS (335 log records across 1 file(s))` and
`ci-gates: zero-panic PASS`. The tree was clean after it.

## Entry 24 — the merge-base probe, directly before the push

`gate.py run --plan … --operator 24`:

```
operator entry 24 · history tripwire: none of 7 forms matched — not a clearance
 24 probe       green · exit 0 · 0.5s · 0 B → 24.log · history: unmoved · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

The remote `main` read `178ebac56e26…` directly after the probe; the build branch contains it. The remote build
branch then still stood at `50216d7e`.

## Entry 25 — the push

`gate.py run --plan … --operator 25`:

```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 2.27s · 135 B → 25.log · history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 50216d7e→d9651126 · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

The move is the intended one: `git ls-remote origin refs/heads/build/andromeda-pulse-0.4.0` read
`d9651126f00614c5c10825b292c17cfe4be94277` after it, equal to the local `HEAD` (clock read after it:
2026-10-10T21:33:27Z). No other ref moved.

## Entry 26 — the CI read of the pushed tip

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`:

```
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 58× over 1766 s
d9651126f006 verdict: green · checks 7/7 · wall 1747 s · runs secret-scan#38088083692 completed/success ci#38088083705 …
runs: secret-scan#38088083692 pull_request completed/success · ci#38088083705 pull_request completed/success
checks: a11y (ubuntu-22.04) · boot smoke (ubuntu-22.04) · coverage gate (line ≥75% / function ≥85%) · gitleaks
  lint / test (ubuntu-22.04) · mcp-server tests (ubuntu-22.04) · supply-chain (audit + deny + auditable)
```

**The chunk's CI verdict is `ci#38088083705` on `d9651126`: green, 7 of 7, attempt 1.**

Nothing read red, so the stop did not apply and nothing was fixed on top.

A jobs read after the run closed (clock: 2026-10-10T22:03:10Z): run `completed` / `success`, `run_attempt` 1,
event `pull_request`, head `d9651126f00614c5c10825b292c17cfe4be94277`; all six jobs of the `ci` workflow
`completed/success` (lint / test `114318701241`, supply-chain `114318701365`, boot smoke `114318701396`, a11y
`114318701432`, mcp-server tests `114318701459`, coverage gate `114318701519`). In the boot job every step read
`success`: `Boot pulse-app smoke` (step 14), `Boot series (equal source)` (15), `cargo xtask ci-gates` (16),
`Console engine cycle` (17), the two uploads (18, 19).

Artifacts of the run: `logs-boot-Linux` (`11682901972`, 39106 B) and `logs-engine-Linux` (`11682966785`, 5535 B),
both expiring 2026-10-24. Neither was downloaded: the plan lists no artifact read.

## The chunk's tests on the runner (from the `lint / test` job's log)

Read from the job log fetched to a file (2 673 783 B, not empty) with colour escapes and line timestamps stripped;
every count is the log's own summary line or a count of `PASS` name lines halved (each stands twice in the log).

- `3132 tests run: 3132 passed, 0 skipped`; 0 lines of `FAIL`, `TIMEOUT`, `SIGABRT` or `SIGSEGV`.
- The new member `test-support`: 14 tests passed (the factories' ten, the port claim's four).
- `xtask::bin/xtask test_support_edges::`: 5 passed. The `default_built` test of `buffer`: passed.
- The four port-using binaries, each on the one shared claim: `pulse-app::integration_engine_boot` 6,
  `pulse-app::integration_console_engine` 7, `xtask::agent_run_shell` 19, `xtask::engine_cycle_refusals` 6.
- The two unit tests that took a claimed number, `receiver_probe_tells_a_listening_loopback_port_from_a_closed_one`
  and `a_listener_on_a_cycle_port_is_read_as_taken_and_a_closed_one_is_not`: passed.
- The two moved end-to-end binaries, `e2e_drain_template_assignment` and `e2e_pii_span_masking_stored_field`:
  one test each, passed.

The `coverage gate` job's thresholds step printed `Line: 35070/38688 = 90.6% (threshold 75%)` and
`Function: 3695/4129 = 89.5% (threshold 85%)`, the new member inside the measure; its instrumented run read
`3132 tests run: 3132 passed, 0 skipped`. The `gitleaks` check read `success` with the moved canary const in the
pushed diff. The `supply-chain` and `mcp-server tests` jobs read `success` (their logs were not read).

## Limits of this pass

- One CI run read, attempt 1, on the pre-CI commit. The wrap's own commit is not covered by it.
- The runner's pull-request run builds the tip merged with `main`; the local pre-push check read the tip alone.
- The port-using binaries ran once on the runner under the test job and once more under coverage. One green run
  does not bound a rare failure there; the dev-host count is seven of seven (`evidence/repeated-run.md`).
- The boot job's log and the two artifacts were not read beyond the step conclusions: this chunk touches no
  boot-path file and the plan lists no artifact read.
- This file and nothing else was written after the push; the tree carries it uncommitted for the wrap.
