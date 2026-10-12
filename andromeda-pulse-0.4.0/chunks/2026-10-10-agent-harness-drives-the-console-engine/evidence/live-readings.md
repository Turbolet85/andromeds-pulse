# Live readings — the gate block and one hand-driven smoke (dev host, 2026-10-10)

/implement run `.andromeda/runs/2026-10-10T15-24-06Z-implement/`. Every path below is relative to the
repository root; the boot verb's own lines print the data dir absolute and are respelled here.

## The gate block, fired whole twice through the gate tool

First firing (over the tree as P1 left it): `entries 22 · green 14 · red 2 (2,16) · recorded 0 · timeout 0 · not-run 6`.
Both reds were one cause, clippy's `manual_contains` on two of the new workflow pins in
`pulse-app/tests/quality_gate_workflow.rs` (entry 2 directly, entry 16 through its `clippy` stage:
`"reason": "stage-failed:clippy"`). Fixed in that file; entries 2 and 16 re-fired green; then the whole block:

```
  1 lint        green · exit 0 · 0.64s · 0 B → 1.2.log · cargo fmt --check
  2 lint        green · exit 0 · 0.57s · 72 B → 2.3.log · cargo clippy --workspace --all-targets --all-features -- -… (68 chars)
  3 probe       green · exit 0 · 0.01s · 0 B → 3.2.log · git diff --name-only 8394da4f2585e9ba7c5b2f197c4fa3e81f393… (520 chars)
  4 unit        green · exit 0 · 0.93s · 63357 B → 4.4.log · cargo nextest run --workspace --profile ci --no-tests=fail… (120 chars)
  5 smoke       green · exit 0 · 97.7s · 2195 B → 5.2.log · d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-series" &&… (290 chars)
  6 e2e         green · exit 0 · 18.57s · 1591 B → 6.2.log · mkdir -p target/exit-witness && cc -shared -fPIC -O2 -o ta… (226 chars)
  7 lint        green · exit 0 · 0.4s · 100 B → 7.2.log · cargo xtask check:english-sources
  8 probe       green · exit 0 · 0.29s · 394 B → 8.2.log · cargo xtask capability-widening-check
  9 probe       green · exit 0 · 0.29s · 99 B → 9.2.log · cargo xtask check:ingest-progress
 10 probe       green · exit 0 · 0.32s · 342 B → 10.2.log · cargo xtask check:staged-artifacts
 11 probe       green · exit 0 · 0.33s · 714 B → 11.2.log · cargo xtask capability-drift
 12 probe       green · exit 0 · 0.3s · 447 B → 12.2.log · cargo xtask verify:capability-matrix
 13 unit        green · exit 0 · 18.45s · 738523 B → 13.2.log · cargo nextest run --workspace --profile ci --no-tests=fail
 14 build       green · exit 0 · 2.74s · 493 B → 14.2.log · cargo nextest run -p pulse-app --features mcp-server --bin… (101 chars)
 15 probe       green · exit 0 · 0.01s · 0 B → 15.2.log · git diff --quiet 8394da4f2585e9ba7c5b2f197c4fa3e81f393d35 … (95 chars)
 16 integration green · exit 0 · 91.21s · 1621185 B → 16.3.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
 17 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ga… (71 chars)
 18 probe       not run — leg operator (the letter drives it) · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
 19 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
 20 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci… (95 chars)
 21 probe       not run — leg operator (the letter drives it) · d="target/boot-smoke/ci-<id>-attempt-1" && gh run download… (379 chars)
 22 probe       not run — leg operator (the letter drives it) · d="target/engine-cycle/ci-<id>-attempt-1" && gh run downlo… (196 chars)
entries 22 · green 16 · red 0 · recorded 0 · timeout 0 · not-run 6
```

Entry 13's own line in `13.2.log`: `Summary [  17.344s] 3064 tests run: 3064 passed, 0 skipped`. The new pins are
in it by name: 45 of `engine_log`, 21 of `engine_cycle`, the three port pins of `inject_demo` and the four
workflow pins of the engine cycle step and its upload. Entry 9 printed
`check:ingest-progress: NEUTRAL — no `buffer.tick` events in the log family — nothing to assert`, as before this
chunk; the verb is not changed here.

## Entry 6 — the engine cycle (second firing, `6.2.log`, the output whole)

Data dir `target/engine-cycle/20261010T155020Z`, ports 24317 and 24318, the witness library built and named in
the verb's environment.

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
boot: ready (PID=1296832, data_dir=target/engine-cycle/20261010T155020Z)
  OTLP gRPC:    127.0.0.1:24317
  OTLP HTTP:    127.0.0.1:24318
  Log file:     target/engine-cycle/20261010T155020Z/logs/agent-latest.jsonl
{
  "buffer_ticks": 2,
  "connection_ticks": 2,
  "ingest_ticks": 2,
  "memory_samples_populated": 1,
  "pid": 1296832,
  "program": "console",
  "verdict": "settled"
}
{
  "ended": null,
  "last_write_age_seconds": 0,
  "log_file_basename": "agent-latest.jsonl.2026-10-10",
  "pid": 1296832,
  "program": "console",
  "stale_after_seconds": 60,
  "verdict": "running-healthy"
}
cleanup: clean
engine-log: family PASS (335 records across 1 file(s))
engine-log: program PASS (one app.boot.engine record, reading console)
engine-log: panic PASS (no app.panic.fatal record at ERROR)
engine-log: heartbeat-gap PASS (ingest.tick max gap 15000 ms (n=2); buffer.tick max gap 15000 ms (n=2); connection.tick max gap 15000 ms (n=2))
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

What the plan asked to be measured (Implementation notes, "What the settle read waits for"):

- The cycle took 18.57 s on the second firing and 41.14 s on the first, which also linked the release engine
  binary for the first time on this host and compiled the injector.
- The settle read returned at the second tick of each target (counts 2, 2, 2), and the memory sample on that
  second `buffer.tick` was already populated (1 of 2): the prediction held, on both firings. No reading needed
  the third tick.
- Read from the kept data dir of the first firing (`target/engine-cycle/20261010T154418Z`): 335 records, levels
  INFO 134 · DEBUG 198 · WARN 3 · ERROR 0. The three WARN records are `app.boot.engine` (no interpretation
  seated), `corpus.keychain.fallback` (the passphrase path, no session bus in the child environment) and
  `app.exit`. The harness's exit record reads `signal 15 (TERM)`. `logs/` holds the log member, an empty
  `boot.log` and `build.log`; no `exit-witness.jsonl`.

## Entry 5 — the window smoke (second firing, `5.2.log`, its verdict)

Two boots of the release window app, each under its own virtual display, ports 14317 and 14318. Both status
reads carried `"program": "window"` and `"verdict": "running-healthy"` with the window program asked; both
cleanups printed `cleanup: clean`; both cycle lines read `series-cycle boot=0 settled=0 status=0 cleanup=0`.

```
{
  "boots": 2,
  "ended": 0,
  "other": 0,
  "per_boot": [
    {
      "app_exit_record": "absent",
      "cycle": "complete",
      "ended": null,
      "exit_witness": "unset",
      "ordinal": 2,
      "verdict": "settled",
      "windows_settled": 4
    },
    {
      "app_exit_record": "absent",
      "cycle": "complete",
      "ended": null,
      "exit_witness": "unset",
      "ordinal": 3,
      "verdict": "settled",
      "windows_settled": 4
    }
  ],
  "settled": 2,
  "verdict": "all-settled"
}
```

## Hand-driven smoke — a verb asked for the window app, pointed at a running console engine

Driven by hand at 2026-10-10T15:53:08Z (clock read before it; 15:53:11Z after it), outside the gate tool, each
step bounded at 300 s, under `env -i` with `HOME`, `PATH`, the data dir, the two ports and a passphrase made
for the run. Data dir `target/engine-cycle/20261010T155308Z-wrong-program`, ports 24317 and 24318, no feed.

```
## boot engine
boot: ready (PID=1363114, data_dir=target/engine-cycle/20261010T155308Z-wrong-program)
  OTLP gRPC:    127.0.0.1:24317
  OTLP HTTP:    127.0.0.1:24318
  Log file:     target/engine-cycle/20261010T155308Z-wrong-program/logs/agent-latest.jsonl
exit=0
## status (no word: the window app is asked)
{
  "ended": null,
  "last_write_age_seconds": 0,
  "log_file_basename": "agent-latest.jsonl.2026-10-10",
  "pid": 1363114,
  "program": "console",
  "stale_after_seconds": 60,
  "verdict": "wrong-program"
}
exit=1
## harness:ready --program window
{
  "ended": null,
  "otlp_grpc": "accepting",
  "otlp_http": "accepting",
  "pid": 1363114,
  "program": "console",
  "verdict": "wrong-program"
}
exit=1
## status engine
{
  "ended": null,
  "last_write_age_seconds": 0,
  "log_file_basename": "agent-latest.jsonl.2026-10-10",
  "pid": 1363114,
  "program": "console",
  "stale_after_seconds": 60,
  "verdict": "running-healthy"
}
exit=0
## cleanup
cleanup: clean
exit=0
## check:engine-log (a run of a few seconds with no feed)
engine-log: family PASS (38 records across 1 file(s))
engine-log: program PASS (one app.boot.engine record, reading console)
engine-log: panic PASS (no app.panic.fatal record at ERROR)
engine-log: heartbeat-gap cannot-evaluate (ingest.tick 1 record(s), two needed; buffer.tick 1 record(s), two needed; connection.tick 1 record(s), two needed)
engine-log: progress FAIL (no span landed, largest rows_ingested 0 over 1 buffer.tick record(s))
engine-log: process-end PASS (one app.exit record, the last of the family)
engine-log: budget FAIL (no memory sample to grade, populated 0 of 1)
engine-log: budget frame and snapshot not graded (a console engine has no producer for either)
engine-log: FAIL
exit=1
```

This is the live form of two things the pins hold on constructed input: a healthy console engine with both
receivers accepting reads `wrong-program`, exit 1, to a verb that asks for the window app; and a boot of a few
seconds with no feed cannot read PASS on the check.

## Process and port census (measured after each live step, `ps -eo pid,comm,args` and `ss -ltn`)

No `pulse-app`, `andromeda-pulse-engine`, `inject_demo` or `Xvfb` process, and nothing listening on 14317,
14318, 24317 or 24318, after the first whole firing, and again after the hand-driven smoke at 15:53:11Z. Ports
4317 and 4318 were never bound by this run.

## Limits

- One host, three engine boots and four window boots in all; every one of them healthy. No red was produced
  live except the two the hand-driven smoke asks for.
- The `wrong-program` reading in the other direction (a verb asked for the console engine, pointed at a
  running window app) was not driven live; it is pinned on constructed input.
- The exit-witness variable was set in the cycle's environment on both firings and no witness file appeared;
  the engine's process was not inspected for a preload while it ran.
- The cycle step of the `boot` CI job has not run: its first reading is the operator pass's.
