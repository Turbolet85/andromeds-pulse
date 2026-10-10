# The operator pass (2026-10-10)

**Whose word.** The operator, 2026-10-10, given in the implement session after the P4 report and kept verbatim as
inputs#I3: "Run the operator pass, entries 17 to 22 in the plan order: hygiene, the pre-CI commit, the pre-push
check, the merge-base probe, the push, the CI read, then the two artifacts. The cycle step reads a runner for the
first time: if it is red, or the boot job is, stop and report what the job kept whole - the cycle verdict, the
engine log check lines, the per-boot verdicts; fix nothing on top. Start no skill - the wrap is mine to call."
Run by the implement agent on that word, under no skill and with no run dir. Each entry's line and its call's
summary line are copied verbatim, without the header.

## Entry 17 — hygiene, before the commit

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`:

```
root . · case exact · planes rust, ts · base HEAD (no --marker)
hygiene: clean — read 45 (runs 38 · evidence 2 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This file was written after the push and is not among the 45.

## The pre-CI commit

`b32e979f307e1dd9f9c2c688a3faaf2d27a1695b` on `build/andromeda-pulse-0.4.0`, subject
`chore(2026-10-10-agent-harness-drives-the-console-engine): operator pre-CI commit`, parent `8394da4f` (the chunk
base). 63 files changed. Before it the staged bindings held `"mcp":` once and were identical to the chunk base's;
no staged path lay outside `.andromeda/`, `andromeda-pulse-0.4.0/`, `.claude/session-handoff.md` and the ten
source paths of research's two lists. `scripts/agent-run.sh` stayed mode 100755. After it `git status --short`
printed nothing.

## Entry 16 again — the local pre-push check, on the committed tree

`gate.py run --plan … --entry 16`:

```
 16 integration green · exit 0 · 88.69s · 1621142 B → 16.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

From its log: `"verdict": "green"`, `"reason": "all-stages-ok"`, `"head": "b32e979f307e1dd9f9c2c688a3faaf2d27a1695b"`,
`"tree": "0af99383332210a0d0f0d61014afeea4b2c730d9"`, `"missing": []`; six stages, each `"ok": true` (script-modes,
source-lint, npm, clippy, test, ci-gates). Its test stage read `3064 tests run: 3064 passed, 0 skipped`. The tree
was clean after it.

## Entry 18 — the merge-base probe, directly before the push

`gate.py run --plan … --operator 18`:

```
operator entry 18 · history tripwire: none of 7 forms matched — not a clearance
 18 probe       green · exit 0 · 0.4s · 0 B → 18.log · history: unmoved · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

The remote `main` read `178ebac56e26…` directly after the probe; the build branch contains it. The remote build
branch then still stood at `8394da4f`.

## Entry 19 — the push

`gate.py run --plan … --operator 19`:

```
operator entry 19 · history tripwire: git
 19 probe       green · exit 0 · 2.14s · 135 B → 19.log · history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 8394da4f→b32e979f · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

The move is the intended one: `git ls-remote origin refs/heads/build/andromeda-pulse-0.4.0` read
`b32e979f307e1dd9f9c2c688a3faaf2d27a1695b` after it, equal to the local `HEAD` (clock read after it:
2026-10-10T15:58:30Z). No other ref moved.

## Entry 20 — the CI read of the pushed tip

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`:

```
repo Turbolet85/andromeds-pulse (the push remote `origin`) · polled 57× over 1737 s
b32e979f307e verdict: green · checks 7/7 · wall 1721 s · runs secret-scan#38065768208 completed/success ci#38065768197 …
runs: secret-scan#38065768208 pull_request completed/success · ci#38065768197 pull_request completed/success
checks: a11y (ubuntu-22.04) · boot smoke (ubuntu-22.04) · coverage gate (line ≥75% / function ≥85%) · gitleaks
  lint / test (ubuntu-22.04) · mcp-server tests (ubuntu-22.04) · supply-chain (audit + deny + auditable)
```

**The chunk's CI verdict is `ci#38065768197` on `b32e979f`: green, 7 of 7, attempt 1.**

A jobs read after the run closed (clock: 2026-10-10T16:27:41Z): run `completed` / `success`, `run_attempt` 1,
event `pull_request`, head `b32e979f307e1dd9f9c2c688a3faaf2d27a1695b`; all six jobs of the `ci` workflow
`completed/success` (supply-chain, a11y, mcp-server tests, lint / test, boot smoke, coverage gate). In the boot
job (`114252977009`) every step read `success`, the two new ones among them:

- step 17 `Console engine cycle`, 16:10:50Z to 16:12:30Z (100 s), directly after `cargo xtask ci-gates`;
- step 19 `Upload engine logs artifact`, after `Upload boot logs artifact`.

Artifacts of the run: `logs-engine-Linux` (`11675214119`, 5482 B, expires 2026-10-24) and `logs-boot-Linux`
(`11675169187`, 39057 B, expires 2026-10-24).

## The cycle step on the runner — its first reading there (from the boot job's log)

The step's own output, read from the job log with colour escapes and line timestamps stripped. The boot verb's
three path lines are respelled with `{runner temp}`; nothing else is changed.

```
boot: ready (PID=15879, data_dir={runner temp}/andromeda-pulse-engine-data)
  OTLP gRPC:    127.0.0.1:24317
  OTLP HTTP:    127.0.0.1:24318
  Log file:     {runner temp}/andromeda-pulse-engine-data/logs/agent-latest.jsonl
{
  "buffer_ticks": 2,
  "connection_ticks": 2,
  "ingest_ticks": 2,
  "memory_samples_populated": 1,
  "pid": 15879,
  "program": "console",
  "verdict": "settled"
}
{
  "ended": null,
  "last_write_age_seconds": 0,
  "log_file_basename": "agent-latest.jsonl.2026-10-10",
  "pid": 15879,
  "program": "console",
  "stale_after_seconds": 60,
  "verdict": "running-healthy"
}
cleanup: clean
engine-log: family PASS (339 records across 1 file(s))
engine-log: program PASS (one app.boot.engine record, reading console)
engine-log: panic PASS (no app.panic.fatal record at ERROR)
engine-log: heartbeat-gap PASS (ingest.tick max gap 15000 ms (n=2); buffer.tick max gap 15001 ms (n=2); connection.tick max gap 15001 ms (n=2))
engine-log: progress PASS (2 buffer.tick record(s), largest rows_ingested 810)
engine-log: process-end PASS (one app.exit record, the last of the family)
engine-log: budget PASS (memory max 138240 B <= 512000000 B, n=2, populated 1)
engine-log: budget frame and snapshot not graded (a console engine has no producer for either)
engine-log: PASS
{
  "boot": 0,
  "check": 0,
  "cleanup": 0,
  "error_records": 0,
  "settled": 0,
  "status": 0,
  "verdict": "pass",
  "witness_file": "absent"
}
```

The step's environment held `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (the job exports it for every later step) and the
job's own `ANDROMEDA_PULSE_DATA_DIR`; the cycle ran on its own data dir and left no witness file. `cleanup: clean`
appears nine times in the job log: the smoke, seven series boots and the engine.

**Measured there, and named as unmeasured in the implement report:** the children's cleared environment does
re-fingerprint the build on the runner. The step's 100 s held a 25.20 s dev build before the boot (the injector
and what it needs), then, inside `boot engine`, a release build that recompiled `pulse-app` alone (44.61 s) and a
dev build that recompiled 14 workspace crates (11.51 s). The job's environment sets `CARGO_INCREMENTAL: 0`, which
the children do not carry. It cost time, not a verdict; it is not fixed here.

## Entry 21 — the `logs-boot-Linux` artifact, what the boot job kept of the window app

`gate.py run --plan … --operator 21 --id 38065768197`, into `target/boot-smoke/ci-38065768197-attempt-1/`:

```
operator entry 21 · history tripwire: none of 7 forms matched — not a clearance
 21 probe       green · exit 0 · 1.32s · 3303 B → 21.log · history: unmoved · d="target/boot-smoke/ci-<id>-attempt-1" && gh run download… (379 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

From its log, the series verdict (`boot-series.json`), per boot:

| ordinal | cycle | verdict | windows_settled | ended | app_exit_record | exit_witness |
|---|---|---|---|---|---|---|
| 1 | smoke | settled | 4 | null | absent | loaded |
| 2 | complete | settled | 4 | null | absent | loaded |
| 3 | complete | settled | 4 | null | absent | loaded |
| 4 | complete | settled | 4 | null | absent | loaded |
| 5 | complete | settled | 4 | null | absent | loaded |
| 6 | complete | settled | 4 | null | absent | loaded |
| 7 | complete | settled | 4 | null | absent | loaded |
| 8 | complete | settled | 4 | null | absent | loaded |

`"boots": 7`, `"settled": 7`, `"ended": 0`, `"other": 0`, `"verdict": "all-settled"`.

The smoke's settle verdict (`harness-settled.json`): `"verdict": "settled"`, `"windows_settled": 4`, `"pid": 7520`,
`"ended": null`, `"app_exit_record": "absent"`, `"display": "reachable"`, `"session_bus": "reachable"`,
`"exit_witness": "loaded"`. The seven per-boot settle verdicts read the same (settled, 4, loaded, reachable,
reachable).

Every kept witness file, read whole — eight files, one line each, every line of the `loaded` shape and nothing
beyond it:

```
exit-witness.jsonl                  {"kind":"loaded","pid":7520,"comm":"pulse-app"}
series/boot-2/exit-witness.jsonl    {"kind":"loaded","pid":8510,"comm":"pulse-app"}
series/boot-3/exit-witness.jsonl    {"kind":"loaded","pid":8833,"comm":"pulse-app"}
series/boot-4/exit-witness.jsonl    {"kind":"loaded","pid":9155,"comm":"pulse-app"}
series/boot-5/exit-witness.jsonl    {"kind":"loaded","pid":9477,"comm":"pulse-app"}
series/boot-6/exit-witness.jsonl    {"kind":"loaded","pid":9799,"comm":"pulse-app"}
series/boot-7/exit-witness.jsonl    {"kind":"loaded","pid":10122,"comm":"pulse-app"}
series/boot-8/exit-witness.jsonl    {"kind":"loaded","pid":10446,"comm":"pulse-app"}
```

The window's boot record on the runner (the smoke's log): target `app.boot.engine`, level INFO,
`"program":"window"`, `"interpretation":"model"`, `"reason":"window_runs_model"`, timestamp
`2026-10-10T16:07:24.453Z`. Each of the seven series logs holds exactly one such record, all `"program":"window"`:
with the window program asked by bare `boot` and `status`, all eight boots read ready and healthy. The record's
`git.commit.sha` reads `2d098663…`, the pull request's merge commit, not the pushed tip: the run builds the tip
merged with `main`. Every `xvfb.log` is empty (the entry's last read printed nothing).

## Entry 22 — the `logs-engine-Linux` artifact, graded again on this host

`gate.py run --plan … --operator 22 --id 38065768197`, into `target/engine-cycle/ci-38065768197-attempt-1/logs/`:

```
operator entry 22 · history tripwire: none of 7 forms matched — not a clearance
 22 probe       green · exit 0 · 2.31s · 675 B → 22.log · history: unmoved · d="target/engine-cycle/ci-<id>-attempt-1" && gh run downlo… (196 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

The entry asserts the exit alone; the check's lines, copied from its log:

```
engine-log: family PASS (339 records across 1 file(s))
engine-log: program PASS (one app.boot.engine record, reading console)
engine-log: panic PASS (no app.panic.fatal record at ERROR)
engine-log: heartbeat-gap PASS (ingest.tick max gap 15000 ms (n=2); buffer.tick max gap 15001 ms (n=2); connection.tick max gap 15001 ms (n=2))
engine-log: progress PASS (2 buffer.tick record(s), largest rows_ingested 810)
engine-log: process-end PASS (one app.exit record, the last of the family)
engine-log: budget PASS (memory max 138240 B <= 512000000 B, n=2, populated 1)
engine-log: budget frame and snapshot not graded (a console engine has no producer for either)
engine-log: PASS
```

The same nine lines the step printed on the runner. The artifact holds three files: the log member
`agent-latest.jsonl.2026-10-10` (96302 B; levels INFO 138 · DEBUG 198 · WARN 3 · ERROR 0, the three WARN records
being `app.boot.engine`, `corpus.keychain.fallback` and `app.exit`), an empty `boot.log` and the boot verb's
`build.log` (1985 B). No `exit-witness.jsonl`. With `logs/` alone downloaded there is no harness exit record
beside a pid file; the log holds its own end, so the process-end arm did not need one.

## Limits of this pass

- One run, attempt 1, on one sha (`b32e979f`). The wrap's own commit will be covered by its local gate until it
  is pushed and read.
- Only the boot job's log was opened. The other five jobs are read by their conclusion.
- The cycle step has one reading on a runner. A red of it has never been seen there, so what the job keeps when
  the cycle fails is not shown: the upload fails on no file, and a cycle refused before its boot writes no log.
- `logs-engine-Linux` carries `build.log` and `boot.log` beside the log family, because the step uploads the data
  dir's `logs/` whole, as the boot upload does. `build.log` holds the runner's checkout paths (cargo's own
  `Compiling` lines). It is kept in the artifact, not in this repository.
- The cost the cleared child environment adds on the runner (about 56 s of rebuild inside `boot engine`) is one
  reading.

## Left on the host by this pass (all under git-ignored `target/`)

`target/boot-smoke/ci-38065768197-attempt-1/` and `target/engine-cycle/ci-38065768197-attempt-1/`.
