# Codebase Research — 2026-10-10-agent-harness-drives-the-console-engine

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read whole, Session Additions included) —
  5 additions applied: 2026-08-23 (`run` is the test suite, never a telemetry send; one exported data dir per leg;
  the producer built first and run by path), 2026-08-30 (a default data dir keyed on `$$` differs per invocation,
  so one exported `ANDROMEDA_PULSE_DATA_DIR` serves every verb of a cycle), 2026-08-28 (a leg's window must outlast
  the threshold its verdict reads), 2026-08-29 (a harness's negative finding needs a second source), 2026-05-14
  (a log dir reused across boots carries earlier records into a family read).
- **Platform issues consulted:** none — no runner-only bullet (the one run read at take-up was in progress, not
  red) and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's phase directive for this chunk (eight points) ·
  `inputs#I2` — the operator's three answers at P4 (the cut, C4's first half retired, the three harness readings
  classified routine).

## Files inspected
- `scripts/agent-run.sh` (full, 300 lines) — the five verbs. `boot` is the only verb that names a program
  (`cargo build --bin pulse-app --release` at :61, `APP_BIN="target/release/pulse-app"` at :77); it loads the exit
  witness onto the spawn line when `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` names a file (:48-55, :94-96). `status`
  (:169-177) shells `cargo xtask harness:status`; `cleanup` (:179-265) ends the pid the pid file holds with TERM,
  a 5 s liveness poll, then KILL, and takes its token from pid liveness and the two resolved ports; `logs`
  (:267-290) resolves the `agent-latest.jsonl*` family. None of the last three names a program.
- `scripts/agent-run.ps1` (the two program names, :133, :155) — the mirror; no job and no dev host runs it.
- `xtask/src/harness_status.rs` (:1-260) — `run()` takes no argument; `classify(pid, alive, newest_log)` is pure;
  the verdict JSON has six members; `read_ended` is the bounded exit-record reader.
- `xtask/src/harness_ready.rs` (outline) — `run_ready()` = the status verdict plus a TCP accept on both resolved
  ports; `run_settled` waits for an `app.boot.window.navigation` record for each of four labels (`WINDOW_LABELS`,
  :37), a condition a windowless program can never meet.
- `xtask/src/ci_gates.rs` (full) — two arms over the family (`evaluate`, :95): at least one record, no
  `app.panic.fatal` at ERROR; exit 0 / 1 / 2; an absent family is `CannotEvaluate`.
- `xtask/src/perf_budget.rs` (:1-330) — arms frame / memory / snapshot; `grade_arm` reads a memory arm whose
  samples are all zero as NEUTRAL (`populated 0 of n`), which a required arm turns into FAIL; `family_members`
  (:298) takes files by the `agent-latest.jsonl` prefix alone.
- `xtask/src/ingest_progress.rs` (full) and `xtask/src/main.rs` (:489-526) — `evaluate` returns `Neutral` when
  the stream holds no `buffer.tick`; the verb prints `check:ingest-progress: NEUTRAL — …` and exits 0.
- `xtask/ci/heartbeat-gap-check.sh` (full) — grades `ingest|buffer|viz|plugins.tick` (never `connection.tick`),
  exits 0 with a "trivially passes" line on no log and on no tick, and reads one tick per target as gap 0, PASS.
- `xtask/src/main.rs` (:606-770) — `invoke_heartbeat_check` (:614) is called from `run_perf_load_profiles`
  alone (:742).
- `xtask/src/pre_push.rs` (:20-40, :262-282, :519-524) — `SEED_LOG` holds three records (one `app.boot.ready`,
  two `ingest.tick` 15 s apart); the sixth stage writes it and runs `cargo xtask ci-gates`.
- `xtask/src/harness_series.rs` (:40-56) — the series' cycle script calls bare `boot`, `status`, `cleanup`.
- `pulse-app/src/console.rs` (full) — the grammar is `run` | `version`; `run` never returns and ends by signal.
- `pulse-app/src/engine_boot.rs` (full) — `init_process` writes the pid file (:206) for both programs;
  `emit_boot_record` (:415) writes one `app.boot.engine` record with `program` `window` | `console`, first thing in
  `start` (:478), before either receiver binds; a `None` port records `invalid_port` and starts no receiver
  (:852-875).
- `pulse-app/src/heartbeat.rs` (:80-232, :282-372) — the engine's three ticks every 15 s, the first at boot;
  each `buffer.tick` is followed by one `metric.buffer.memory_bytes` sample.
- `crates/buffer/src/retention.rs` (:18-125) — the memory gauge is set only by a retention sweep; the sweep
  cadence is `retention_seconds.max(60) / 6` and the first sweep runs one cadence after boot.
- `pulse-app/tests/integration_console_engine.rs` (full) — the spawned-program witness: cleared environment, own
  data dir, picked ports, a passphrase for the corpus, ended by SIGTERM and by SIGINT.
- `pulse-app/tests/integration_engine_boot.rs` (outline) — the two re-exec arms over `init_process` (:531, :574).
- `pulse-app/tests/quality_gate_workflow.rs` (:466-545 and the test list) — the pins a `ci.yml` edit must keep.
- `.github/workflows/ci.yml` (the job outline; `boot` job :279-372) — six jobs; `boot` builds the workspace in
  release, then smoke, series, `ci-gates`, one upload.
- `crates/ingest/examples/inject_demo.rs` (:1-60, the CLI lines) — its endpoint is the literal
  `http://127.0.0.1:4317` (:307); its `mod tests` is collected (`test = true` in `crates/ingest/Cargo.toml:74-77`).
- `.andromeda/registries/contracts/test-plan/5-command-implementation.md`, `pid-file.md`,
  `.andromeda/registries/contracts/obs-plan/heartbeat-ticks.md` — the three keyed contracts the chunk turns on.

## Graph impact (from the code-graph query; plane `rust`)
- **`run_ready`** — 1 caller, `main()` at `xtask/src/main.rs:264` — a parameter added to it threads through one
  call site (query 1 of `tree-query-2026-10-10-agent-harness-drives-the-console-engine.json`).
- **`harness_status::run`** — 1 caller, `main()` at `xtask/src/main.rs:264` (read in the file; the bare name
  `run` collides across modules, so it was not keyed in the query).
- **`classify`** (the one in `harness_status.rs`) — called by `harness_status::run` (:42), `run_ready`
  (`harness_ready.rs:55`) and 13 test sites in the two files. Its signature is left as it is: the program reading
  is a separate function, so no test site moves.
- **`family_members`** — 3 production callers (`ci_gates::evaluate`, `run_perf_budget`, `perf_frame`); reused
  unchanged by the new module.
- **Name check** — `engine_log`, `engine_cycle`, `engine_settled`: 0 rows in `symbol` (query 2 of the same trace);
  the names are free.

## Patterns detected
- **A pure decision pinned per arm, the probe pinned apart** (`harness_status.rs:83`, `harness_ready.rs:138`):
  the shape every new arm here takes.
- **Three exits, an absent input never a pass** (`ci_gates.rs:39-46`): 0 PASS · 1 FAIL · 2 cannot-evaluate.
- **A failure line of coordinates, never content** (`ci_gates.rs:68`): member file name and line number.
- **A window that cannot reach its verdict is refused up front** (`harness_ready.rs:167`,
  `timeout_supports_verdict`).
- **The log names its own program** (`engine_boot.rs:415-440`): the one in-log fact that tells a window boot's
  log from a console boot's. It is written before the receivers bind, so it is in the log before a readiness read
  can succeed.
- **A spawned child under a cleared environment** (`integration_console_engine.rs:44-55`,
  `pre_push.rs` stage children): no session bus, so no leg reaches the OS credential store; the corpus then needs
  `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` or it opens absent with an ERROR record.

## Conventions to follow
- **xtask verbs are formalized contracts** (`xtask/src/main.rs:47-260`, clap `name = "…"` with an `about`); a new
  verb declares its arms and exits in its module doc, like `ci_gates.rs:1-8`.
- **xtask tests are co-located `mod tests`** (xtask is not `pulse-app`; its unit binary runs under nextest).
- **Workflow pins live in `pulse-app/tests/quality_gate_workflow.rs`** and read `ci.yml` by lines; no line of
  `ci.yml`, a comment included, may hold the tokens `windows`, `macos`, `matrix.os` or `runner.os ==`
  (`ci_workflow_runs_on_linux_only`, :105).
- **ASCII-only sources** (`cargo xtask check:english-sources` scans `xtask/src`).

## Measured facts the plan rests on
- **Which verbs already serve the console engine unchanged.** `status`, `cleanup`, `logs` and `harness:ready`
  read the pid file, the log family, the exit record and the two ports — all written the same way by either
  program through `engine_boot::init_process` and the boot wrapper. Only `boot` (it names the binary) and
  `harness:settled` (it waits for window records) are program-specific.
- **What `cleanup` does to the engine.** TERM first; the engine's signal listener writes `app.exit` and re-raises
  the same signal; the wrapper then records `signal 15 (TERM)`. A KILL escalation leaves no `app.exit` by
  construction and the wrapper records `signal 9 (KILL)`.
- **What a short engine run holds.** The first tick of each target is written at boot and the second 15 s later,
  so a run shorter than about 16 s holds one tick per target and no consecutive pair. With the default 600 s
  retention the first sweep is at 100 s; with `ANDROMEDA_PULSE_RETENTION_SECONDS=60` (the registered minimum) it
  is at 10 s, so the `buffer.tick` at 15 s carries a non-zero memory sample when spans landed before the sweep.
- **Which budget arms have a producer in a console run.** Memory: yes, one sample per `buffer.tick`. Snapshot:
  no (its one emitter is the snapshot generator, which a console run never calls). Frame: no (webview-originated).
- **The existing gap script cannot grade a console log as the plan needs.** It ignores `connection.tick`, passes
  on no tick and on a single tick, and reads no `app.boot.engine` record.
- **The exit witness would ride onto the engine.** `boot` loads it on whatever it spawns when the variable is
  set, and the CI `boot` job exports that variable for every later step of the job.
- **The injector cannot reach a leg on other ports.** Its endpoint is fixed at 4317.
- **No program-level trigger exists for a panic or a C `exit()` of the console program.** Its inputs are two
  command words, the registered variables and OTLP payloads; none reaches a panic or `exit()` short of a defect.
  Search: `grep -n 'panic!\|process::exit\|unwrap()\|expect(' pulse-app/src/console.rs
  pulse-app/src/bin/andromeda-pulse-engine.rs` finds one hit, `console.rs:71`, the runtime-build `expect` — it
  fires before the log sink exists, so it can leave no record, and no input reaches it.
- **The a11y job calls no harness verb.** `grep -n 'agent-run\|harness:' .github/workflows/ci.yml` finds the verbs
  in the `boot` job's steps alone; `cargo xtask test:a11y` writes nothing a family read takes (the family is
  matched by the `agent-latest.jsonl` prefix).
- **The release engine binary is not on the dev host** (`ls target/release/andromeda-pulse-engine`: absent);
  `target/release/pulse-app` is, so the release dependency set is warm and the first engine pre-build is a link.

## New files to create
- `xtask/src/engine_log.rs` — the check over the console engine's log family and the read that says when that log is gradeable
- `xtask/src/engine_cycle.rs` — one cycle of the verbs on the console engine, on its own data dir and ports, as an xtask verb

## Files to modify
- `scripts/agent-run.sh` — `boot` and `status` take the program word; the witness is never loaded onto the engine
- `xtask/src/harness_status.rs` — the verdict names the program its log records, and refuses the wrong one
- `xtask/src/harness_ready.rs` — the readiness verdict carries the same program reading
- `xtask/src/main.rs` — the program flag on the two harness verbs, the three new verbs, the module lines
- `xtask/src/pre_push.rs` — the seed's doc comment says what the seed holds and what the stage reads
- `crates/ingest/examples/inject_demo.rs` — the endpoint port comes from the registered gRPC port variable
- `.github/workflows/ci.yml` — the engine cycle step and its log upload in the `boot` job
- `pulse-app/tests/quality_gate_workflow.rs` — pins for the engine cycle step and its upload

## Open questions
- none — the three plan-decision questions research carried were answered at P4 (inputs#I2): the freight is cut
  in two and the second chunk's files left these lists; the console program's own panic and at-exit witness is
  retired with its reason, no trigger built; the three new harness readings are routine on the operator's
  reading. The cycle is an xtask verb, not a script: test-plan §3 places a check outside the five verbs as a
  `cargo xtask` verb (the `pre-push:linux` precedent), and a verb needs no new variable.
