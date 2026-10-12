# The operator pass (2026-10-10)

**Whose word.** The operator, 2026-10-10, given in the implement session after the P4 report and kept verbatim as
inputs#I3: "Run the operator pass, entries 20 to 24 in the plan order: hygiene, the pre-CI commit, the pre-push
check, the merge-base probe, the push, the CI read, the boot artifact. A red boot job is a new reading: stop and
report its per-boot verdicts and witness lines whole; fix nothing on top. Start no skill - the wrap is mine to
call." Run by the implement agent on that word, under no skill and with no run dir. Each entry's line and its
call's summary line are copied verbatim, without the header.

## Entry 20 — hygiene, before the commit

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`:

```
root . · case exact · planes rust, ts · base HEAD (no --marker)
hygiene: clean — read 45 (runs 39 · evidence 1 · inputs 5) · trails 15 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This file was written after the push and is not among the 45.

## The pre-CI commit

`4f519c94aba16da44948e8edef110d23d602c293` on `build/andromeda-pulse-0.4.0`, subject
`chore(2026-10-10-console-engine-entry-point): operator pre-CI commit`, parent `f31027a4` (the chunk base).
68 files changed. Before it the staged bindings held `"mcp":` once and no staged path lay outside `.andromeda/`,
`andromeda-pulse-0.4.0/`, `.claude/session-handoff.md` and `pulse-app/`. After it `git status --short` printed
nothing.

## Entry 19 again — the local pre-push check, on the committed tree

`gate.py run --plan … --entry 19`:

```
 19 integration green · exit 0 · 88.45s · 1575769 B → 19.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

From its log: `"verdict": "green"`, `"reason": "all-stages-ok"`, `"head": "4f519c94aba16da44948e8edef110d23d602c293"`,
`"tree": "c1d4b44ecca45197e5db8701d81536b6cc302d88"`, `"missing": []`; six stages, each `"ok": true` (script-modes,
source-lint, npm, clippy, test, ci-gates). Its test stage read `2976 tests run: 2976 passed, 0 skipped`. The tree
was clean after it.

## Entry 21 — the merge-base probe, directly before the push

`gate.py run --plan … --operator 21`:

```
operator entry 21 · history tripwire: none of 7 forms matched — not a clearance
 21 probe       green · exit 0 · 0.43s · 0 B → 21.log · history: unmoved · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

The remote `main` read `178ebac56e26…` directly after the probe; the build branch contains it.

## Entry 22 — the push

`gate.py run --plan … --operator 22`:

```
operator entry 22 · history tripwire: git
 22 probe       green · exit 0 · 2.43s · 135 B → 22.log · history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 f31027a4→4f519c94 · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

The move is the intended one: the remote branch read `4f519c94aba16da44948e8edef110d23d602c293` through
`git ls-remote` after the push, equal to the local `HEAD`. Pushed by 2026-10-10T13:45:15Z.

## Entry 23 — the CI read, after the push

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`:

```
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 59× over 1798 s
4f519c94aba1 verdict: green · checks 7/7 · wall 1790 s · runs ci#38056942758 completed/success secret-scan#38056942764 …
runs: ci#38056942758 pull_request completed/success · secret-scan#38056942764 pull_request completed/success
checks: a11y (ubuntu-22.04) · boot smoke (ubuntu-22.04) · coverage gate (line ≥75% / function ≥85%) · gitleaks
  lint / test (ubuntu-22.04) · mcp-server tests (ubuntu-22.04) · supply-chain (audit + deny + auditable)
```

The ci run id is **38056942758**. Read once more after the tool returned, at 2026-10-10T14:15:27Z:
`gh run view 38056942758` gave status `completed`, conclusion `success`, attempt 1, event `pull_request`, head sha
`4f519c94aba16da44948e8edef110d23d602c293`, and six jobs (name · conclusion · started · completed):

```
coverage gate (line ≥75% / function ≥85%) · success · 2026-10-10T13:45:25Z · 2026-10-10T14:15:15Z
mcp-server tests (ubuntu-22.04) · success · 2026-10-10T13:45:25Z · 2026-10-10T13:51:48Z
boot smoke (ubuntu-22.04) · success · 2026-10-10T13:45:25Z · 2026-10-10T14:01:23Z
supply-chain (audit + deny + auditable) · success · 2026-10-10T13:45:25Z · 2026-10-10T13:53:46Z
lint / test (ubuntu-22.04) · success · 2026-10-10T13:45:26Z · 2026-10-10T13:53:25Z
a11y (ubuntu-22.04) · success · 2026-10-10T13:45:25Z · 2026-10-10T13:50:03Z
```

The seventh check, `gitleaks`, is the `secret-scan` workflow's (`secret-scan#38056942764`, success). No run was
re-run. The boot job read green, so the pass did not stop on it.

## Entry 24 — the boot job's artifact

`gate.py run --plan … --operator 24 --id 38056942758`:

```
operator entry 24 · history tripwire: none of 7 forms matched — not a clearance
 24 probe       green · exit 0 · 1.34s · 3303 B → 24.log · history: unmoved · d="target/boot-smoke/ci-<id>-attempt-1" && gh run download… (379 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

Its log was read whole (3303 B). What it holds, in order:

- **The series verdict** (`boot-series.json`): `"boots": 7`, `"ended": 0`, `"other": 0`, `"settled": 7`,
  `"verdict": "all-settled"`. Its `per_boot` lists eight entries, ordinals 1 to 8: ordinal 1 is `"cycle": "smoke"`,
  ordinals 2 to 8 are `"cycle": "complete"`; every one reads `"verdict": "settled"`, `"windows_settled": 4`,
  `"ended": null`, `"app_exit_record": "absent"`, `"exit_witness": "loaded"`.
- **The smoke's settle verdict** (`harness-settled.json`): `"verdict": "settled"`, `"windows_settled": 4`,
  `"display": "reachable"`, `"session_bus": "reachable"`, `"ended": null`, `"app_exit_record": "absent"`,
  `"exit_witness": "loaded"`, `"pid": 7659`.
- **Every kept witness file, whole.** Eight files, one line each, and no other line:

```
== exit-witness.jsonl
{"kind":"loaded","pid":7659,"comm":"pulse-app"}
== series/boot-2/exit-witness.jsonl
{"kind":"loaded","pid":8597,"comm":"pulse-app"}
== series/boot-3/exit-witness.jsonl
{"kind":"loaded","pid":8914,"comm":"pulse-app"}
== series/boot-4/exit-witness.jsonl
{"kind":"loaded","pid":9235,"comm":"pulse-app"}
== series/boot-5/exit-witness.jsonl
{"kind":"loaded","pid":9556,"comm":"pulse-app"}
== series/boot-6/exit-witness.jsonl
{"kind":"loaded","pid":9873,"comm":"pulse-app"}
== series/boot-7/exit-witness.jsonl
{"kind":"loaded","pid":10195,"comm":"pulse-app"}
== series/boot-8/exit-witness.jsonl
{"kind":"loaded","pid":10513,"comm":"pulse-app"}
```

  (The `==` lines are shortened here to the path under the download dir.) Each line is the closed `loaded` shape
  (kind, pid, process name). No line holds anything beyond its shape, and there is no `end` line.
- **The window's boot record on the runner** (the smoke boot's log, first match):

```
{"fields":{"ci.run.id":"38056942758","deployment.environment":"dev","git.commit.sha":"e3d94ada21e32d41dd86dd81e227d4099769fc48","interpretation":"model","program":"window","reason":"window_runs_model","service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"engine boot","target":"app.boot.engine","timestamp":"2026-10-10T13:56:36.398Z"}
```

- **The display server's error output**: the entry's last command, `find … -name xvfb.log -size +0c`, printed
  nothing.

## Reads beside the entries (read-only, by the agent)

- Each of the eight kept app logs of the artifact (the smoke's and `series/boot-2` to `boot-8`) holds the
  `app.boot.engine` record exactly once (`grep -c` per file: 1, eight times). Eight `xvfb.log` files, 0 of them
  larger than 0 bytes.
- The `coverage gate` job's log (job 114227250099, fetched to a file, 977,987 B) prints
  `Line:     34599/38214 = 90.5% (threshold 75%)` and `Function: 3622/4056 = 89.3% (threshold 85%)`.
- The `lint / test` job's log (job 114227250413, fetched to a file, 2,556,830 B) reads
  `Starting 2976 tests across 135 binaries` and `Summary [  37.595s] 2976 tests run: 2976 passed, 0 skipped`, and
  holds a `PASS` line for each of the four tests of `pulse-app::integration_console_engine`, the spawned `run`
  arm at 16.896 s.

## Limits

One run was read, attempt 1, on the pre-CI commit `4f519c94`; the wrap's commit follows it. The pull-request run
builds the tip merged with `main` (the record's `git.commit.sha`, `e3d94ada…`, is that merge: its two parents read
`178ebac56e26` and `4f519c94aba1` through the commits API), while the local pre-push check read the tip alone. The coverage log prints totals only: that the lines of `engine_boot.rs` and
`console.rs` are inside the measure was not read per file. The job logs of `a11y`, `mcp-server tests`,
`supply-chain` and `boot smoke` were not opened; the boot job was read through its artifact. The `xvfb.log`
files were read for size alone. Every non-operator entry and the local pre-push check ran on the dev host alone.
