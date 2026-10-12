# Fan-out results — 2026-10-10-agent-harness-drives-the-console-engine (wrap, second window)

Seven Explore doc-agents, one batch, the prompt of `references/amendment-flow.md` sent verbatim with repo-relative
paths. Detectors per prompt: architecture 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 ·
obs-plan 4 · a11y-plan 2 = 19, the count of `doc:` names over `.andromeda/drift-base.md`.
Each return below is the agent's hand-back text as it arrived, taken from its transcript by script (no hand copy).
Entity probe on every return: 0 before decoding, 0 after. No return failed the parse; no raw twin is kept
(the three empty returns parse as `proposals: []` with their YAML comments in place — stripping changed nothing).

Verdicts: architecture 24 · security-plan 6 · design-system 0 · layout-templates 0 · test-plan 19 · obs-plan 12 ·
a11y-plan 0 — 61 proposals. Rejected at Validate's opening rule (a coordinate the report does not carry): 0.
Dispositions: 60 apply · 0 reject · 1 escalate (obs-plan 12, resolved at the route-resolve card by the operator's
recorded direction, inputs#I4 item 7; its factual half applied with obs-plan 8).

Proposals are numbered per doc in the order returned (A1…A24, S1…S6, T1…T19, O1…O12).

## architecture

verdict: 24 proposals (D-arch-resources: 9 primary, 15 dependent; D-arch-decisions: none). Comments kept below.

```yaml
# D-arch-decisions: no drift. The report's Dependencies bullet reads "none added, none bumped" (Cargo.toml and Cargo.lock not in the diff; chrono already declared by xtask); no workspace member, runtime or library is new, and no locked decision in §Stack / §Established Decisions is contradicted.
# D-arch-resources: 9 primary occurrences + 15 dependent occurrences, all in architecture.md body sections or one keyed contract.
# Line numbers below are the report's own (Changes / New text, by line); architecture.md sites are named by section only.
proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → xtask CLI surfaces (dev/CI gates) — new entry `cargo xtask check:engine-log`
    change: >-
      Register `cargo xtask check:engine-log` (`xtask/src/engine_log.rs`; no argument, paths resolved as `harness:status` resolves them): it reads the family `agent-latest.jsonl*` in the directory of the resolved log base and the harness's exit record beside the resolved pid file through `read_ended`; all lines on stdout — one per arm, `engine-log: {arm} {PASS|FAIL|cannot-evaluate} ({detail})`, arms in order `family` · `program` · `panic` · `heartbeat-gap` · `progress` · `process-end` · `budget`, then `engine-log: budget frame and snapshot not graded (a console engine has no producer for either)`, then `engine-log: PASS` | `FAIL` | `cannot-evaluate`; exit 0 · 1 · 2, a FAIL on any arm outranking a cannot-evaluate. Arms: `family` (no member is cannot-evaluate and nothing else is graded; members with no record FAIL) and `panic` read `ci_gates::evaluate`'s verdict; `program` passes on exactly one `app.boot.engine` record reading `console` (`window` FAIL; none, an unknown label or more than one cannot-evaluate); `heartbeat-gap` grades `ingest.tick`, `buffer.tick`, `connection.tick` (a gap over 45 000 ms FAIL; fewer than two records or an unreadable timestamp cannot-evaluate) and never `viz.tick` or `plugins.tick`; `progress` is `ingest_progress::evaluate` (a non-recovery stall or a largest `rows_ingested` of 0 FAIL); `process-end` wants exactly one `app.exit` as the last record (none is FAIL `unloggable-end` when the exit record reads `signal 9 (KILL)`, else `end-not-recorded`); `budget` is `perf_budget::grade_arm(…, Arm::Memory)`, required, over 512 000 000 B / non-numeric / no sample / only zero samples FAIL. No line holds a record's text or a path. Caller: the last step of `harness:engine-cycle`. It adds no environment variable, port, TauRPC procedure or capability.
    sidecar: architecture §Occupied Resources gains the `check:engine-log` CLI contract (seven arms, exit 0/1/2) landed by chunk 2026-10-10-agent-harness-drives-the-console-engine.
    rationale: Report Changes → Counts "xtask verbs +3 … `check:engine-log` (`main.rs` 69-97)" and Harness / gate surface give the full contract; the registry's xtask CLI surfaces bullet names no `engine-log` verb (Expected amendments 7, "three new verbs"). An unregistered CLI contract is drift under the formalized-CLI-contract rule the bullet itself cites.
    basis: xtask/src/engine_log.rs:415-434

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → xtask CLI surfaces (dev/CI gates) — new entry `cargo xtask harness:engine-settled`
    change: >-
      Register `cargo xtask harness:engine-settled [--timeout-seconds N]` (default 60; below 20 reads `cannot-evaluate`; `xtask/src/engine_log.rs`): it polls every 500 ms until the log family's boot record reads `console`, the pid is alive, the family holds at least two records of each of `ingest.tick`, `buffer.tick`, `connection.tick` and one `metric.buffer.memory_bytes` sample with a non-zero `value`. Contract: one JSON object on stdout, seven members `{verdict, pid, program, ingest_ticks, buffer_ticks, connection_ticks, memory_samples_populated}`; `settled` exit 0 · `ended` 1 · `wrong-program` 1 · `not-settled` 1 · `cannot-evaluate` 2 (no pid, an unprobeable pid, the timeout floor). It writes no file and resolves its paths through `harness_status::resolve_paths`. Caller: `harness:engine-cycle`, between the injector's start and the status read. The window settle verb `harness:settled` is unchanged and has no console arm.
    sidecar: architecture §Occupied Resources gains the `harness:engine-settled` CLI contract (seven members, five verdicts) landed by chunk 2026-10-10-agent-harness-drives-the-console-engine.
    rationale: Report Changes → Counts "xtask verbs +3: `harness:engine-settled` …" and Harness / gate surface state the verb, its seven members and five verdicts; the registry names only `harness:settled`. Unregistered verb = drift.
    basis: xtask/src/engine_log.rs:565-578

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → xtask CLI surfaces (dev/CI gates) — new entry `cargo xtask harness:engine-cycle`
    change: >-
      Register `cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]` (`xtask/src/engine_cycle.rs`): defaults a fresh `target/engine-cycle/{UTC second, %Y%m%dT%H%M%SZ}` and ports 24317 / 24318. Refused before anything starts — verdict `cannot-evaluate`, exit 2, one stderr line `harness:engine-cycle: cannot-evaluate ({label})` — on `not-linux` · `shared-port` (4317 or 4318 in either flag) · `data-dir-holds-a-log-family`, also `home-unset`, `path-unset`, `data-dir-unusable`, `injector-build-failed`. Then in order: `cargo build -p ingest --example inject_demo` → `bash scripts/agent-run.sh boot engine` → `target/debug/examples/inject_demo --sustained --error-pct=0` by path → `cargo xtask harness:engine-settled` → the injector killed by its child handle → `agent-run.sh status engine` → `agent-run.sh cleanup` → `cargo xtask check:engine-log`; after a failed boot the settle and status verbs are skipped, cleanup and the check always run. Every child gets a CLEARED environment plus exactly `HOME` · `PATH` · `ANDROMEDA_PULSE_DATA_DIR` · `ANDROMEDA_PULSE_OTLP_GRPC_PORT` · `ANDROMEDA_PULSE_OTLP_HTTP_PORT` · `ANDROMEDA_PULSE_RETENTION_SECONDS` = 60 · `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (32 hex characters made per run, never printed or written) · `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` only when the verb's own environment holds it; no session bus, runtime dir or display variable. Children's stdout goes to the verb's stderr; stdout carries one JSON object of eight members `{verdict, boot, settled, status, cleanup, check, error_records, witness_file}` — `verdict` `pass` 0 · `fail` 1 · `cannot-evaluate` 2; each verb member its exit, null when it did not run, -1 for a child that could not start or was ended by a signal; `witness_file` `absent` | `present`. `fail`: any verb exit other than 0 or 2, an ERROR record in the family, or a witness file; `cannot-evaluate`: no failure and a verb exit of 2, a verb that did not run, or no cycle. The verdict holds no path and no environment value. Caller: the `Console engine cycle` step of ci.yml's `boot` job (`if: always()`, `--data-dir "$RUNNER_TEMP/andromeda-pulse-engine-data"`, directly after `cargo xtask ci-gates`), held by four workflow pins in `pulse-app/tests/quality_gate_workflow.rs`. It binds no port itself and adds no `ANDROMEDA_PULSE_*` name, TauRPC procedure or capability.
    sidecar: architecture §Occupied Resources gains the `harness:engine-cycle` CLI contract (refusals, pinned child environment, eight-member verdict, CI caller) landed by chunk 2026-10-10-agent-harness-drives-the-console-engine.
    rationale: Report Changes → Counts "xtask verbs +3: … `harness:engine-cycle`" plus Harness / gate surface (verb, CI step 366-374, workflow pins 695-792); the registry holds no `engine-cycle` entry. Unregistered verb with a CI caller = drift.
    basis: xtask/src/engine_cycle.rs:87-102

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → xtask CLI surfaces (dev/CI gates) — the `cargo xtask harness:status` entry
    change: >-
      The entry reads `cargo xtask harness:status [--program window|console]`: one verdict JSON object of SEVEN members `{verdict, pid, ended, program, log_file_basename, last_write_age_seconds, stale_after_seconds}`; `program` is the label of the LAST `app.boot.engine` record of the log family — `window`, `console`, or `unknown` (no such record, or a label that is neither). FIVE arms: `running-healthy` (exit 0) / `stale` (1) / `wrong-program` (1) / `not-running` (1) / `cannot-evaluate` (2); with `--program`, a `running-healthy` or `stale` run whose log records another program, or none, reads `wrong-program`, while `not-running` and `cannot-evaluate` stand whatever was asked; with no flag the verdict is as before plus the `program` member. The sentence "the four arms and their exit codes are unchanged" is retired. Script callers: `agent-run.sh status` runs it with `--program window`, `status engine` with `--program console`; `agent-run.ps1` passes no flag.
    sidecar: architecture `harness:status` contract — members 6 → 7 (`program`), arms 4 → 5 (`wrong-program`), new `--program window|console` flag (chunk 2026-10-10-agent-harness-drives-the-console-engine).
    rationale: Report Changes → Counts "`harness:status` verdict members 6 → 7 … verdict words 4 → 5 (`wrong-program`)" and Harness / gate surface; the registry still spells a six-member object and "four arms", and `wrong-program` appears nowhere in architecture (Expected amendments 7 site counts).
    basis: xtask/src/harness_status.rs:101-111

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → xtask CLI surfaces (dev/CI gates) — the `cargo xtask harness:ready` entry
    change: >-
      The entry reads `cargo xtask harness:ready [--program window|console]`: one JSON object of SIX members `{verdict, pid, ended, program, otlp_grpc, otlp_http}`; FIVE arms `ready` (exit 0) / `not-ready` (1) / `wrong-program` (1) / `ended` (1) / `cannot-evaluate` (2). It reads the status through `harness_status::for_program`; `wrong-program` holds whatever the receivers answer, and a dead pid is `ended` whatever was asked. Callers: the `boot` readiness poll of `agent-run.sh` (`--program window` for bare `boot`, `--program console` for `boot engine`); `agent-run.ps1` still calls it with no flag, which reads as before.
    sidecar: architecture `harness:ready` contract — members 5 → 6 (`program`), arms 4 → 5 (`wrong-program`), new `--program window|console` flag (chunk 2026-10-10-agent-harness-drives-the-console-engine).
    rationale: Report Changes → Counts "`harness:ready` verdict members 5 → 6 … its verdict words 4 → 5 (`wrong-program`)" and Symbols / APIs (`decide_ready` gains `wrong-program → wrong-program`); the registry still spells `{verdict, pid, ended, otlp_grpc, otlp_http}` and "four arms".
    basis: xtask/src/harness_ready.rs:89-104

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → xtask CLI surfaces (dev/CI gates) — the `scripts/agent-run.{sh,ps1}` entry
    change: >-
      Still five verbs, but in `agent-run.sh` `boot` and `status` take one optional word: none is the window app, `engine` is the console engine, any other word is usage, exit 2. `boot engine` pre-builds `cargo build --bin andromeda-pulse-engine --release` and xtask, spawns `target/release/andromeda-pulse-engine run` BY PATH under the same waiting wrapper (spawn record and exit record as for the window app), and polls `cargo xtask harness:ready --program console`; bare `boot` still builds and spawns `target/release/pulse-app` and polls `harness:ready --program window`; `status` runs `harness:status --program window`, `status engine` `--program console`. The exit-witness arm runs for the window program only: `boot engine` never reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, its spawn line sets no preload, and `logs/exit-witness.jsonl` is neither removed nor created on that path. `cleanup`, `logs` and `run` are unchanged. The pair is in lockstep but for TWO sh-only parts — the exit-witness arm of `boot` and the `engine` word (`agent-run.ps1` takes neither and calls the two xtask verbs with no flag). A second CI caller: `cargo xtask harness:engine-cycle` runs `boot engine`, `status engine` and `cleanup`.
    sidecar: architecture `agent-run` contract — `boot` / `status` gain the sh-only `engine` word (builds and spawns `target/release/andromeda-pulse-engine run`); lockstep exceptions 1 → 2 (chunk 2026-10-10-agent-harness-drives-the-console-engine).
    rationale: Report Changes → Symbols / APIs (`scripts/agent-run.sh`, `usage()` 38-49, the word read at 56-64): "the two scripts no longer take the same words: the `engine` word is sh-only, as the exit-witness arm is". The registry says `boot` builds `--bin pulse-app` and spawns `target/release/pulse-app[.exe]` with one sh-only exception.
    basis: scripts/agent-run.sh:56-64

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem locations — new bullet for the engine cycle's data dirs
    change: >-
      Add a harness-written, git-ignored location: `target/engine-cycle/{stamp}/` — one data dir per `cargo xtask harness:engine-cycle` run with no `--data-dir` (`{stamp}` the UTC second, `%Y%m%dT%H%M%SZ`), holding `logs/` (the engine's log family, `boot.log`, `build.log`), `run/` and `corpus/` as the engine and the boot verb write them; nothing is removed after a cycle, so the dirs accumulate and are the operator's to delete. Never product-written. In CI the cycle's data dir is `$RUNNER_TEMP/andromeda-pulse-engine-data`, a directory other than the job's own data dir, and its whole `logs/` rides the artifact `logs-engine-Linux` (`if-no-files-found: error`, `retention-days: 14`) — the log family plus `boot.log` and `build.log`.
    sidecar: architecture §Filesystem locations gains `target/engine-cycle/{stamp}/` and the `logs-engine-Linux` artifact (chunk 2026-10-10-agent-harness-drives-the-console-engine).
    rationale: Report Changes → Harness / gate surface "Filesystem locations (harness-written, git-ignored): `target/engine-cycle/{stamp}/` per cycle … nothing is removed after a cycle" and the CI upload step (384-393); architecture names `target/pre-push` and `target/exit-witness` but no `target/engine-cycle` (Expected amendments 7).
    basis: .github/workflows/ci.yml:384-393

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Network ports
    change: >-
      Add: `:24317` / `:24318` — harness-only defaults of `cargo xtask harness:engine-cycle` (`DEFAULT_GRPC_PORT` / `DEFAULT_HTTP_PORT`), the loopback ports it hands the console engine's two receivers for its own run through `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `_HTTP_PORT`; the verb refuses 4317 and 4318 in either flag (`shared-port`), so a cycle never binds the product defaults. No port is added to the product.
    sidecar: architecture §Network ports records the engine cycle's harness-only loopback defaults 24317 / 24318 (chunk 2026-10-10-agent-harness-drives-the-console-engine).
    rationale: Report Changes → Symbols / APIs "`DEFAULT_GRPC_PORT` 24317, `DEFAULT_HTTP_PORT` 24318" and Ports / sockets "The cycle verb's defaults move the engine's two loopback receivers to 24317 and 24318 for its own run and refuse 4317 and 4318"; neither number is in architecture.
    basis: xtask/src/engine_cycle.rs:104-118

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — `ANDROMEDA_PULSE_OTLP_GRPC_PORT`
    change: >-
      Add to the entry: a second reader outside the product since chunk 2026-10-10-agent-harness-drives-the-console-engine — the dev example `crates/ingest/examples/inject_demo` (`resolve_port`), never shipped: its endpoint is `http://127.0.0.1:{port}`, the host fixed, the port 4317 when the variable is unset; a set value that does not parse as a port from 1 to 65535, a blank one included, is refused with `inject_demo: … must be a port from 1 to 65535`, exit 2, nothing sent, the value never repeated. `cargo xtask harness:engine-cycle` SETs this variable and `ANDROMEDA_PULSE_OTLP_HTTP_PORT` in its children's environment.
    sidecar: architecture env registry — `ANDROMEDA_PULSE_OTLP_GRPC_PORT` gains the `inject_demo` reader and the cycle verb as a setter (chunk 2026-10-10-agent-harness-drives-the-console-engine).
    rationale: Report Changes → Symbols / APIs (`inject_demo` — `resolve_port` 302-314): "This is a second reader of that registered variable, a dev example, never shipped"; the registry entry reads only "override `:4317`" (Expected amendments 7).
    basis: crates/ingest/examples/inject_demo.rs:302-314

  - detector: D-arch-resources
    severity: warning
    section: §Stack and Technologies → GUI verification harness (dev-only) row — the injector's CLI contract
    change: >-
      Extend the injector's contract sentence: besides its arguments, `inject_demo` reads `ANDROMEDA_PULSE_OTLP_GRPC_PORT` for its endpoint port (`http://127.0.0.1:{port}`, 4317 when unset); a set value that is not a port from 1 to 65535, blank included, is REJECTED with exit 2 and nothing sent, never defaulted.
    sidecar: architecture §Stack injector contract extended 2026-10-10 — the endpoint port follows `ANDROMEDA_PULSE_OTLP_GRPC_PORT`.
    rationale: The §Stack row restates the injector's formalized CLI contract (arguments and exit-2 rejection); the report's Symbols / APIs adds an environment input and a new exit-2 arm to that same contract (Expected amendments 7, "the injector's contract").
    basis: crates/ingest/examples/inject_demo.rs:302-314
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Process / service identity — Dev-build binary bullet
    change: >-
      Retire "so `target/{profile}/pulse-app` stays the window app's path and the one the harness builds and spawns": `target/{profile}/pulse-app` stays the window app's path and the one bare `agent-run.sh boot` builds and spawns; since chunk 2026-10-10-agent-harness-drives-the-console-engine `agent-run.sh boot engine` builds `cargo build --bin andromeda-pulse-engine --release` and spawns `target/release/andromeda-pulse-engine run` by path, so the harness names two binaries, one per program.
    sidecar: architecture Dev-build binary bullet — the harness now builds and spawns `target/release/andromeda-pulse-engine` too (`boot engine`).
    rationale: The bullet asserts the window binary is "the one the harness builds and spawns"; report Symbols / APIs: "`boot engine` pre-builds `cargo build --bin andromeda-pulse-engine --release` … spawns `target/release/andromeda-pulse-engine run` by path" (Expected amendments 7, "the sentence naming the binary the harness spawns").
    basis: scripts/agent-run.sh:56-64
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem locations → Subpaths — `run/andromeda-pulse.pid`
    change: >-
      Retire "the harness readers named next are not aimed at the console program although it writes the file": since chunk 2026-10-10-agent-harness-drives-the-console-engine `harness:status` and `harness:ready` take `--program window|console` and name the program from the LAST `app.boot.engine` record of the log family (a run of the other program, or one with no boot record, reads `wrong-program`), and `cargo xtask harness:engine-settled` reads the pid file and probes its pid through the same `harness_status` helpers (`read_pid`, `pid_alive`); `agent-run.sh` reaches the console program through `boot engine` / `status engine`. `harness:settled` and `harness:boot-series` stay window-only.
    sidecar: architecture pid-file bullet — the harness readers are aimed at the console program too (`--program console`, `harness:engine-settled`).
    rationale: Same claim as the retired status/ready contract, restated in the pid-file bullet; report Symbols / APIs (`Program` 39-56, `program_of` 132-141) and Harness / gate surface show the readers now grade a console run.
    basis: xtask/src/harness_status.rs:132-141
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem locations → Subpaths — `run/andromeda-pulse.spawn` + `run/andromeda-pulse.exit`
    change: >-
      Add: the `agent-run.sh` waiting wrapper writes the same two records for the console engine under `boot engine` (the engine's pid, then its one-line end), and `cargo xtask check:engine-log` is a further reader of the exit record — through `harness_status::read_ended`, beside the resolved pid file — to tell a `process-end` FAIL of `unloggable-end` (`signal 9 (KILL)`) from `end-not-recorded`; only the closed label leaves the read.
    sidecar: architecture spawn/exit-record bullet — written for the console engine too; `check:engine-log` reads the exit record.
    rationale: The bullet enumerates who writes and reads the two records and says "the app pid"; report Symbols / APIs: "spawn record and exit record as for the window app" and Harness / gate surface: `check:engine-log` reads "the harness's exit record beside the resolved pid file through `read_ended`".
    basis: xtask/src/engine_log.rs:345-379
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem locations → Subpaths — `logs/` (the `logs/exit-witness.jsonl` sentence)
    change: >-
      Qualify "a stale one of the same data dir is removed by `agent-run.sh boot` before the spawn" to the window program: bare `boot` removes it; `boot engine` neither removes nor creates the witness file, and `harness:engine-cycle` reads a file at `logs/exit-witness.jsonl` in its data dir as a failure (`witness_file: present`).
    sidecar: architecture `logs/exit-witness.jsonl` sentence — the stale-file removal is the window boot's alone.
    rationale: Report Symbols / APIs: "the witness block runs for the window program only … the witness file is neither removed nor created on that path"; Harness / gate surface: `fail` on "a witness file". The sentence states the removal for `boot` without a program.
    basis: scripts/agent-run.sh:56-64
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`
    change: >-
      Retire "read solely by `scripts/agent-run.sh boot`": it is read by bare `agent-run.sh boot` (the window program) — `boot engine` never reads it — and, since chunk 2026-10-10-agent-harness-drives-the-console-engine, by value by `cargo xtask harness:engine-cycle`, only to pass it on into its children's cleared environment when the verb's own environment holds it; the verb prints and writes no value of it. The not-a-regular-file refusal before the pre-build is bare `boot`'s alone.
    sidecar: architecture env registry — `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` has a second, pass-through reader (`harness:engine-cycle`); `boot engine` does not read it.
    rationale: Report Env vars: the cycle verb reads "`ANDROMEDA_PULSE_EXIT_WITNESS_LIB` by value, only to pass it on"; Symbols / APIs: "`boot engine` never reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`". "read solely by" no longer holds.
    basis: xtask/src/engine_cycle.rs:124-155
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — `ANDROMEDA_PULSE_EXIT_WITNESS_FILE`
    change: >-
      Name the spawn line: SET (never exported) by bare `scripts/agent-run.sh boot` on the WINDOW app's spawn line; the console engine's spawn line under `boot engine` carries no such variable.
    sidecar: architecture env registry — `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` is set on the window app's spawn line only.
    rationale: The entry says `agent-run.sh boot` sets it "on the app's spawn line"; with two spawn lines the report states the engine's "sets no preload" and that the witness block runs for the window program only.
    basis: scripts/agent-run.sh:56-64
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — `LD_PRELOAD`
    change: >-
      Name the spawn line: SET harness-only by bare `scripts/agent-run.sh boot` on the WINDOW app's spawn line alone; the console engine's spawn line under `boot engine` sets no preload even when `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` names a regular file (read on three cycles with the variable set: `witness_file: absent`).
    sidecar: architecture env registry — `LD_PRELOAD` is set on the window app's spawn line only; `boot engine` sets none.
    rationale: Report Symbols / APIs: "the engine's spawn line sets no preload"; Outcome 6: "`witness_file`: `absent` with the library named". The entry's "the app's spawn line alone … when `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` names a regular file" would read as covering both programs.
    basis: scripts/agent-run.sh:56-64
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Stack and Technologies → Boot-smoke exit witness (harness-only, Linux) row
    change: >-
      Qualify the role cell: a shared library bare `scripts/agent-run.sh boot` preloads into the WINDOW app's spawn alone; `boot engine` spawns the console engine with no preload.
    sidecar: architecture §Stack exit-witness row — the preload is the window boot's alone.
    rationale: Same claim as the `LD_PRELOAD` / `_EXIT_WITNESS_LIB` entries, restated in the §Stack row ("`scripts/agent-run.sh boot` preloads into the app's spawn alone"); report: the witness arm "stays the window's alone" (Expected amendments 9; Reverted / negative API facts: "the exit witness on the engine's spawn line" not written).
    basis: scripts/agent-run.sh:56-64
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — `ANDROMEDA_PULSE_PIDFILE`
    change: >-
      Extend the `xtask/src/harness_status.rs` reader clause: it locates the PID file for both verdicts, for `harness:settled` and, since chunk 2026-10-10-agent-harness-drives-the-console-engine, through the same `resolve_paths` for `cargo xtask harness:engine-settled` and `cargo xtask check:engine-log` (which reads the exit record beside it); `cargo xtask harness:engine-cycle` does not carry the variable into its children's cleared environment, so a cycle's PID file resolves under its own data dir.
    sidecar: architecture env registry — `ANDROMEDA_PULSE_PIDFILE` is honoured by `harness:engine-settled` and `check:engine-log` through `resolve_paths`; absent from the cycle's child set.
    rationale: Report Symbols / APIs: `engine_log` calls `harness_status::{…, resolve_paths, read_ended, end_file, read_pid, pid_alive}`; Deviation 6: "the pid-file and log-file overrides are honoured"; the child set is "exactly" eight names without it. The entry enumerates the verbs the override serves and stops at `harness:settled`.
    basis: xtask/src/engine_log.rs:415-434
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — `ANDROMEDA_PULSE_LOGFILE`
    change: >-
      Extend the reader clause: the log family it names also yields the `program` member of `harness:status` / `harness:ready` (the last `app.boot.engine` record, `read_program`), and since chunk 2026-10-10-agent-harness-drives-the-console-engine `cargo xtask harness:engine-settled` and `cargo xtask check:engine-log` read the same resolved family (ticks, the memory sample, `app.exit`, `app.panic.fatal`); `cargo xtask harness:engine-cycle` does not carry the variable into its children, so a cycle's log family resolves under its own data dir.
    sidecar: architecture env registry — `ANDROMEDA_PULSE_LOGFILE`'s family now feeds the `program` member and the two engine verbs.
    rationale: The entry lists which verdict fields and verbs derive from the named log family; report Symbols / APIs: `read_program(log_base)` (143-149) "over the family through `smoke::read_jsonl_lines`", and `check:engine-log` "reads the family `agent-latest.jsonl*` in the directory of the resolved log base".
    basis: xtask/src/harness_status.rs:143-149
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — `HOME` · `PATH`
    change: >-
      Add a further harness-only reader since chunk 2026-10-10-agent-harness-drives-the-console-engine: `cargo xtask harness:engine-cycle` reads both from its own environment by value, only to build its children's cleared environment (`HOME` unset reads `cannot-evaluate` / `home-unset`, `PATH` unset `path-unset`, nothing started), and prints and writes neither.
    sidecar: architecture env registry — `HOME` · `PATH` gain `harness:engine-cycle` as a by-value harness reader.
    rationale: Report Env vars: "The cycle verb reads `HOME` and `PATH` from its own environment by value, only to build its children's environment … it prints and writes none of them"; the entry names `pre-push:linux` and, for `PATH`, `harness:boot-series` as the readers.
    basis: xtask/src/engine_cycle.rs:217-301
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Environment variables — harness-set values (beside the `PUPPETEER_CACHE_DIR` entry, which records what `pre-push:linux` SETs)
    change: >-
      Add: `cargo xtask harness:engine-cycle` SETs, into its children only, `ANDROMEDA_PULSE_DATA_DIR` (the cycle's data dir), `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `_HTTP_PORT` (24317 / 24318 unless flagged), `ANDROMEDA_PULSE_RETENTION_SECONDS` = 60 and `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` — 32 hex characters made per run, never printed or written, so the engine of a cycle takes the passphrase key and no child reaches a session bus or an OS credential store. No new `ANDROMEDA_PULSE_*` name.
    sidecar: architecture env registry records the five registered variables the engine cycle sets into its children (data dir, two ports, retention 60, a per-run corpus passphrase).
    rationale: Report Harness / gate surface: "Every child gets a cleared environment plus exactly: … `ANDROMEDA_PULSE_RETENTION_SECONDS` = 60, `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (32 hex characters made per run …)". The registry records the sibling verb's sets in the env section; a harness setter of the secret-class passphrase variable is unrecorded.
    basis: xtask/src/engine_cycle.rs:124-155
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: >-
      The `boot` job's step chain no longer ends at `cargo xtask ci-gates`: directly after it comes `Console engine cycle` — `if: always()`, a plain `run: cargo xtask harness:engine-cycle --data-dir "$RUNNER_TEMP/andromeda-pulse-engine-data"`, no soft-fail key, on a data dir other than the job's own — and, after `Upload boot logs artifact`, `Upload engine logs artifact` (`if: always()`, `logs-engine-${{ runner.os }}`, the cycle data dir's `logs/`, `if-no-files-found: error`, `retention-days: 14`); so the console engine's own log is graded on every run of the job whatever its earlier steps returned, the job has two uploads, and four more workflow tests in `pulse-app/tests/quality_gate_workflow.rs` pin the two steps. As measured on `ci#38065768197` (pull_request, sha `b32e979f`): green, checks 7/7, the cycle step `success`. `ci.yml` stays six jobs.
    sidecar: architecture CI/CD approach — the `boot` job gains `Console engine cycle` after `ci-gates` and the `logs-engine-Linux` upload (chunk 2026-10-10-agent-harness-drives-the-console-engine).
    rationale: The contract spells the `boot` job's sequence as "… → `Boot series (equal source)` … → `cargo xtask ci-gates`" and speaks of "the upload"; report Harness / gate surface (CI): steps at `ci.yml` 366-374 and 384-393, Counts "the `boot` job's named steps 15 → 17", "upload steps 6 → 7" (Expected amendments 8). Same fact as the new verb's CI caller, restated in the keyed contract.
    basis: .github/workflows/ci.yml:366-374
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → xtask CLI surfaces (dev/CI gates) — the `cargo xtask perf:budget` entry (its closing sentence on in-process callers)
    change: >-
      Where the entry says "`cargo xtask perf:load-profiles` alone runs the same grader in-process with no arm required", add the second in-process caller: `cargo xtask check:engine-log` grades the memory arm through `perf_budget::grade_arm(…, Arm::Memory)`, required, over the console engine's log, and prints that frame and snapshot are not graded; `cargo xtask ci-gates` still runs no grader.
    sidecar: architecture `perf:budget` entry — `check:engine-log` is a second in-process caller of the grader (memory arm, required).
    rationale: Report Symbols / APIs: `engine_log` calls `perf_budget::{read_family, grade_arm}`; Harness / gate surface `budget` arm. If "alone" is read as "the only in-process caller" the sentence is now false; if read as "by itself", this is a completeness note only — lowest-confidence item of the set, for the orchestrator to keep or drop.
    basis: xtask/src/engine_log.rs:381-413
    dependent-of: D-arch-resources
```

### architecture — dispositions (Validate)

Every coordinate cited is a row, an `@`-range or a span of the report's `New text, by line` section, or an added
range it prints (checked row by row: engine_log 415-434, 565-578, 345-379, 381-413; engine_cycle 87-102, 104-118,
124-155, 217-301; harness_status 101-111, 132-141, 143-149; harness_ready 89-104; agent-run.sh 56-64; inject_demo
302-314; ci.yml 366-374, 384-393).

- A1, A2, A3 (the three new verbs) — **apply**. Check 1: playbook `Accurate this-chunk addition`; the xtask CLI
  bullet registers every verb under the formalized-CLI-contract rule, so `Registry over-reach` does not govern (its
  own last sentence: a new resource of a kind the registry enumerates registers normally). Check 5: Expected
  amendments 7.
- A4, A5, A6 (`harness:status`, `harness:ready`, `agent-run` contracts) — **apply**. Check 1 as above; check 5: EA 7.
- A7 (`target/engine-cycle/`, the upload) — **apply**. Check 5: EA 7.
- A8 (ports 24317 / 24318) — **apply**. Check 1: `Accurate this-chunk addition`; the registry is of occupied
  resources and this host shares 4317 / 4318 with another project, so a harness default pair is a resource of the
  kind enumerated. Written as harness-only, not a product port.
- A9 (the gRPC port variable's second reader), A10 (the injector's contract, dependent) — **apply**. Check 1:
  `New env var … registration` does not fit (the name is not new); `Accurate this-chunk addition` governs: a reader
  added to a registered name, code-validated (`resolve_port`) and unit-tested (3 pins). Check 5: EA 7.
- A11 (the Dev-build binary sentence), A12 (the pid-file bullet), A13 (spawn and exit records) — **apply**.
  Dependents of the retired claim "the harness builds and spawns the window app alone". Check 5: EA 7.
- A14, A15, A16, A17, A18 (the exit-witness sentences: bare `boot` alone; the cycle verb's pass-through) —
  **apply**. Check 1: `Accurate this-chunk addition`; `Boundary widening` read by subject and found not to match:
  no library is loaded into any process by the cycle, the engine's spawn line carries no preload (three cycles,
  `witness_file: absent`), and no product code reads the variable. The arm's classification stays PROVISIONAL and
  is not touched; the new reader is named under it and brought to the card as a fact for that item.
- A19, A20, A21 (`_PIDFILE`, `_LOGFILE`, `HOME` · `PATH` readers) — **apply**. Dependents; report Env vars and
  Deviation 6.
- A22 (what the cycle sets into its children) — **apply**, re-derived to the report's set; the sentence about which
  key the engine then takes is written as what the child set implies, not as a measurement.
- A23 (`§Infrastructure Patterns → CI/CD approach`) — **apply** (the key file). Check 5: EA 8.
- A24 (`perf:budget`'s in-process callers) — **apply**. Read at the site: "`perf:load-profiles` alone runs the same
  grader in-process with no arm required" stays true as worded; the second in-process caller is added beside it.

## security-plan

verdict: 6 proposals (D-security-input: 2 primary, 4 dependent; D-security-auth, -deps, -logging: none).

```yaml
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the exit-witness arm, item (a))"
    change: "Item (a) should say that `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` is read and validated by bare `scripts/agent-run.sh boot` — the window program — alone among the script's paths (sh only): `boot engine` never reads it, the witness block runs for the window program only, the engine's spawn line sets no preload, and the witness file is neither removed nor created on that path; 'the one spawn command' becomes the window app's spawn command, since the script now holds a second spawn line, the engine's. One more harness member reads the variable: `cargo xtask harness:engine-cycle` reads it by value from its own environment only to pass it on to its children when present, prints and writes nothing of it, reports `logs/exit-witness.jsonl` of the cycle's data dir as the closed label `witness_file` = `absent` | `present`, and reads `fail` on `present`. The library still runs inside the window app's process only, never the console engine's."
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine — exit-witness arm (a): 'read by `agent-run.sh boot` alone' narrowed to bare `boot` (the window program); `boot engine` never reads the variable and sets no preload; `harness:engine-cycle` named as a by-value pass-through reader whose verdict fails on a witness file."
    rationale: "Report Changes → Symbols / APIs (`agent-run.sh`): `boot engine` never reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, the witness block runs for the window program only, the engine's spawn line sets no preload. Changes → Env vars: the cycle verb reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` by value, only to pass it on, and prints and writes none. Harness / gate surface (`harness:engine-cycle`): the variable is in the children's set only when the verb's own environment holds it; `fail` on a witness file. Expected amendments 9: 'the witness arm stays the window's alone'. The body's 'read by `scripts/agent-run.sh boot` alone' and 'the one spawn command' no longer hold as worded. Validation itself is unchanged and present; the drift is the reader claim, and it sits under the arm's PROVISIONAL classification, so it is raised at the detector's severity."
    basis: "xtask/src/engine_cycle.rs:124-155 (`child_env`; pinned at 452-464 and 523-526); scripts/agent-run.sh (the report gives no line for the witness block)"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the `CLI / env var inputs` row ('A fourth harness-only boundary, the exit-witness arm')"
    change: "The row's sentence '`ANDROMEDA_PULSE_EXIT_WITNESS_LIB` is read by `scripts/agent-run.sh boot` alone (sh only)' should say bare `boot` — the window program — alone among the script's paths, `boot engine` never reading it and setting no preload; 'loaded into the app's process on the one spawn line' becomes the window app's spawn line; and it should add that `cargo xtask harness:engine-cycle` reads the variable by value only to pass it on to its children when present, never printing or writing it."
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine — §Input Validation CLI / env row: the witness variable's reader claim narrowed to bare `boot` (window program); the cycle verb's by-value pass-through added."
    rationale: "Second occurrence of the claim retired by the primary: the same 'read by `agent-run.sh boot` alone' / 'the one spawn line' wording restated in §Input Validation. Report Changes → Symbols / APIs (`agent-run.sh`) and Env vars."
    basis: "xtask/src/engine_cycle.rs:124-155; scripts/agent-run.sh"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → CLI input (env vars + binary launch) → Trust boundary"
    change: "The sentence '`agent-run.sh boot` reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (sh only; trim + regular-file check, fail closed) and loads the named library into the app's process on the spawn line' should name bare `boot`, the window program, and the window app's spawn line; add that `boot engine` never reads the variable and its spawn line carries no preload, and that `harness:engine-cycle` passes the variable on by value to its children only when its own environment holds it."
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine — Threat Model CLI-input trust boundary: the witness read scoped to bare `boot` (window program); `boot engine` and the cycle verb's pass-through stated."
    rationale: "Third occurrence of the claim retired by the primary, restated without the word 'alone' but asserting the same mechanism (one `boot`, one spawn line). Report Changes → Symbols / APIs (`agent-run.sh`), Env vars; acceptance criterion 6 (the engine's spawn line carries no preload)."
    basis: "xtask/src/engine_cycle.rs:124-155; scripts/agent-run.sh"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the `CLI / env var inputs` row (the harness-only readers, after 'A fourth harness-only boundary')"
    change: "Add a fifth harness-only boundary (chunk `2026-10-10-agent-harness-drives-the-console-engine`; not product-consumed, no file under `pulse-app/src` changed): (1) `scripts/agent-run.sh` `boot` / `status` take one optional word from a closed set — none is the window app, `engine` the console engine, any other word prints usage and exits 2 before a directory is made (sh only; `agent-run.ps1` unchanged; no committed shell-level test); (2) `cargo xtask harness:status` / `harness:ready` take `--program window|console`, a clap value enum, and their `program` member is one of three closed labels (`window` · `console` · `unknown`) read from the last `app.boot.engine` record of the log family, a run of the other program or of none reading `wrong-program`; (3) `cargo xtask harness:engine-settled --timeout-seconds N` reads `cannot-evaluate` below 20, and `cargo xtask check:engine-log` takes no argument; both resolve their paths as `harness:status` does (pid-file and log-file overrides honoured), and no line of the check holds a record's text or a path; (4) `cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]` parses its ports as `u16` and is refused before anything starts — `cannot-evaluate`, exit 2, one closed label on stderr — on `not-linux`, `shared-port` (4317 or 4318 in either flag), `data-dir-holds-a-log-family`, `home-unset`, `path-unset`, `data-dir-unusable`, `injector-build-failed`; it reads `HOME` and `PATH` by value only to build its children's environment, printing and writing neither, and its eight-member verdict carries no path and no value of an environment variable; (5) the dev example `inject_demo` (never shipped) reads `ANDROMEDA_PULSE_OTLP_GRPC_PORT` through `resolve_port` — unset is 4317; a set value that is not a port from 1 to 65535, a blank one included, exits 2 with nothing sent and a message that never repeats the value; the host is fixed at `127.0.0.1`; `harness:ready` still reads a blank value as the default."
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine — §Input Validation CLI / env row: a fifth harness-only boundary registered (the `engine` word of `agent-run.sh`, `--program`, `harness:engine-settled`, `check:engine-log`, `harness:engine-cycle` and its refusals, `inject_demo`'s port read)."
    rationale: "The report adds six harness input surfaces and §Input Validation names none of them: its harness-only enumeration ends at 'A fourth harness-only boundary'. Each is validated per the report's Coverage of new surfaces (closed word set; clap value enum; the timeout floor; the three refusals and `u16` ports; `resolve_port`, exit 2) and Symbols / APIs + Harness / gate surface; Deviations 5 and 6 give the blank-value and path-resolution readings. No product external surface was added ('No UI element, no product external surface'). So no boundary is unvalidated; the drift is that the registry does not state the validation for surfaces the chunk added. Raised at the detector's severity."
    basis: "scripts/agent-run.sh:56-64 (usage 38-49); xtask/src/harness_status.rs:39-56; xtask/src/engine_cycle.rs:104-118; crates/ingest/examples/inject_demo.rs:302-314; xtask/src/engine_log.rs:1108-1126"
  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the harness-only class paragraph: boot-recorder state files and 'The same class since chunk …' members)"
    change: "Add a member 'The same class since chunk `2026-10-10-agent-harness-drives-the-console-engine`', classified routine on the operator's reading of 2026-10-10 (inputs#I2; the three harness readings the report carries: the cycle's data dir, the `logs-engine-Linux` upload, the injector's port read), stating: (a) writers and readers of the boot-recorder state files — `agent-run.sh boot engine` spawns `target/release/andromeda-pulse-engine run` by path under the same waiting wrapper and writes the spawn record and exit record as for the window app; bare `boot` polls `harness:ready --program window`, `boot engine` `--program console`; `cargo xtask check:engine-log` reads the exit record through `read_ended`, and it and `harness:engine-settled` resolve paths as `harness:status` does; (b) `cargo xtask harness:engine-cycle` (an xtask verb, never product-consumed, Linux alone) spawns every child with a CLEARED environment plus exactly `HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_OTLP_GRPC_PORT`, `ANDROMEDA_PULSE_OTLP_HTTP_PORT` (defaults 24317 / 24318; 4317 and 4318 refused), `ANDROMEDA_PULSE_RETENTION_SECONDS` = 60, `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (32 hex characters made per run from two `RandomState` hashers, never printed or written) and `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` only when its own environment holds it — no session bus, runtime dir or display variable, so the engine opens its corpus through the passphrase fallback; (c) it writes only its per-cycle data dir, default `target/engine-cycle/{UTC second}/` (`logs/` with the log family, `boot.log` and `build.log`; `run/`; `corpus/`), git-ignored, nothing removed after a cycle; the children's stdout goes to the verb's stderr, which passes through the boot verb's own `boot: ready (PID=…, data_dir=…)` lines; (d) the boot job's step `Console engine cycle` runs it on a data dir of its own and `Upload engine logs artifact` uploads that dir's whole `logs/` as `logs-engine-Linux` (retention 14 days, failing on no file, the upload action at the sha the workflow already pins): the engine's log family plus `boot.log` and `build.log`, and `build.log` holds cargo's `Compiling` lines with the runner's checkout paths, passing no scrubber. Open, stated as open: the operator's classification named 'the engine's own log family', and whether the class covers the two harness files awaits the operator's word."
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine — §Security Anti-Patterns → Input: harness-only class gains the console-engine harness (the cycle verb's cleared child environment and pinned set, `target/engine-cycle/`, the `logs-engine-Linux` upload with `boot.log` and `build.log`, new readers of the exit record); operator's reading of 2026-10-10, the two harness files' coverage left open."
    rationale: "Second occurrence of the retired claim: the class paragraph enumerates its members, their state-file readers ('`boot` polls `cargo xtask harness:ready`', the `read_ended` readers) and the harness-written files riding `logs-boot-Linux` as a closed list, and the report adds members to each. Report Changes → Symbols / APIs (`agent-run.sh`, `engine_log`, `engine_cycle`), Harness / gate surface (the cycle's child set, Filesystem locations, CI), Coverage of new surfaces ('PII raw✗ for `build.log` only'), Expected amendments 9 ('One fact for that classification that the question at P4 did not state: the upload holds `boot.log` and `build.log`'), Found and not owned 2. The passphrase handling matches §Secret Management (the opt-in fallback, by environment, never printed or written), so D-security-auth raises nothing; it is recorded here as part of the pinned child set."
    basis: "xtask/src/engine_cycle.rs:124-155 (pinned at 443-450 and 466-484; the passphrase 206-215); .github/workflows/ci.yml:366-374 and 384-393; pulse-app/tests/quality_gate_workflow.rs:695-792"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → CLI input (env vars + binary launch) → Trust boundary (the harness-only carve-out enumeration)"
    change: "The carve-out enumeration ('tool-locator vars, the `agent-run.{sh,ps1}` boot-recorder state files, the harness-written files under `logs/`, the xtask-only presence read …, and the `pre-push:linux` xtask verb's by-value read of `HOME` / `PATH` with its per-run area under `target/pre-push/run/`') should also name, since chunk `2026-10-10-agent-harness-drives-the-console-engine`: the `engine` word of `agent-run.sh boot` / `status` (sh only, a closed word set), the `--program window|console` flag of `harness:status` / `harness:ready`, the `harness:engine-cycle` xtask verb's by-value read of `HOME` / `PATH` with its per-cycle data dir under `target/engine-cycle/` and its cleared child environment, and the dev example `inject_demo`'s read of `ANDROMEDA_PULSE_OTLP_GRPC_PORT` — none product-consumed."
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine — Threat Model CLI-input trust boundary: harness-only carve-out enumeration gains the console-engine harness members."
    rationale: "Third occurrence: the Threat Model's trust boundary restates the harness-only carve-out as an enumerated list ending at the `pre-push:linux` verb and the exit-witness arm. Report Changes → Symbols / APIs, Env vars ('The cycle verb reads `HOME` and `PATH` from its own environment by value'), Harness / gate surface (Filesystem locations)."
    basis: "xtask/src/engine_cycle.rs:124-155; scripts/agent-run.sh:56-64; crates/ingest/examples/inject_demo.rs:302-314"
    dependent-of: D-security-input
```

### security-plan — dispositions (Validate)

Coordinates checked against `New text, by line`: engine_cycle 124-155, 206-215, 443-450, 452-464, 466-484, 523-526,
104-118; agent-run.sh 38-49, 56-64; harness_status 39-56 (a span of two rows); inject_demo 302-314; engine_log
1108-1126; ci.yml 366-374, 384-393; quality_gate_workflow 695-792 (a span).

All six carry the detector's `escalate` severity. Check 1: playbook `An ESCALATE-severity detector fires on a finding
OUTSIDE the class that severity exists to guard` governs — (a) the escalate condition is affirmatively absent in the
report (every new input reads `validation mechanism✓` in Coverage; "no product external surface"; no file under
`pulse-app/src` in the diff); (b) the actual class has its own disposition, `Accurate this-chunk addition`.
`Boundary widening` read by subject for each: the operator's answer at P4 (inputs#I2, answer 3) classes the three
harness readings routine on the operator's own reading, "none of the three runs inside the product process or adds a
product input"; the plan's Expected amendments 9 names the change itself.

- S1, S2, S3 (the witness variable's reader: bare `boot` alone; the cycle verb passes it on) — **apply**. The arm
  stays PROVISIONAL; nothing of its classification is rewritten.
- S4 (a fifth harness-only boundary in §Input Validation) — **apply**. The `engine` word has no committed shell
  test: recorded as such in the row and in test-plan §1's shell-coverage trigger (T17); owner pinned at
  route-resolve (playbook `Sequencing deferral`: ownership is a route annotation).
- S5 (the harness-only class gains the console-engine harness; the classification) — **apply**. Check 5: EA 9.
  Written as the pre-push member is: the operator's own reading, dated, not the founder's word. The two harness
  files in the upload are stated, with the class question open until the card (inputs#I4 item 7).
- S6 (the Threat Model's carve-out enumeration) — **apply**. Dependent of S4.

## design-system

verdict: proposals: [] — no UI element added, no status claim moved; comments kept below.

```yaml
proposals: []
# D-design-tokens: no drift. Report Coverage of new surfaces (report.md 259-283) flags every one of the seven new surfaces `tokens n/a`, none `hardcoded✗`, and closes with "No UI element, no product external surface, no hot-path operation was added." Files bullet (report.md 20-21) lists every file under `pulse-app/ui` and `pulse-app/src` as not touched.
# D-design-status-narrative: no drift. Spec-master edits: "none" (report.md 78). Spec claims disproved by measurement (report.md 203-212): one chunk-artifact claim (plan.md step 9, the boot job's release build) and "No master sentence was measured false by this chunk." The operator words recorded (inputs#I2, #I3, #I4; report.md 321-330) rule on deviations, the operator pass, harness-reading classification and wrap procedure; none touches the Halo State Pulse signature layer, the Motion high-impact moments, the desktop-native tray patterns or the Self-Validation checks. Expected amendments (report.md 213-216) name nine sites in test-plan, obs-plan, architecture and security-plan, and state design-system site counts read 0.
# Swept .andromeda/design-system.md (426 lines) for status claims the chunk could have moved (DEFERRED / unbuilt / owner pointers in Brand Identity, Motion, desktop-native, Self-Validation 3): the report's Changes bullets carry no fact about any of them, so each stands as the baseline.
```

### design-system — dispositions (Validate)

Nothing to dispose. Check 5: no Expected-amendments entry names this master.

## layout-templates

verdict: proposals: [] — no user-facing surface added, no status claim moved; comments kept below.

```yaml
proposals: []
# D-layout-surface: no drift. The report's Coverage bullet closes with "No UI element, no product external surface, no hot-path operation was added"; Files lists every path under pulse-app/src and pulse-app/ui as not touched. The new surfaces are harness verbs, a script command word, a dev example's port read and two CI steps, none of which is a user-facing surface or region for §Primary screens / the wireframes.
# D-layout-status-narrative: no drift. "Spec-master edits: none"; "Spec claims disproved by measurement" names one plan.md sentence only and states "No master sentence was measured false by this chunk"; Expected amendments name test-plan, obs-plan, architecture and security-plan (layout-templates site counts read 0, not named). The operator words recorded (inputs#I2, I3, I4) rule on deviations, the operator pass and the wrap, not on any surface status, signature placement or owner pointer this doc states. The doc's one overlapping token, the `:4318` / `:4317` empty-state hint in §Component — Empty / error state, stands: the report's Ports / sockets bullet says none added to the product, and 24317 / 24318 are the cycle verb's own-run defaults.
```

### layout-templates — dispositions (Validate)

Nothing to dispose. Check 5: no Expected-amendments entry names this master.

## test-plan

verdict: 19 proposals (D-tests-obs-harness 16, D-tests-coverage 3; D-tests-framework: none). Comments kept below.

```yaml
# test-plan drift pass — chunk 2026-10-10-agent-harness-drives-the-console-engine
# D-tests-framework: no drift. The report's runner is `cargo nextest run --workspace --profile ci` (plus the
#   narrowed `-E 'package(xtask) and test(/^(engine_log|engine_cycle)::/)'` form); new tests are co-located
#   `mod tests` in `xtask/src/*` and the `inject_demo` example, plus `pulse-app/tests/quality_gate_workflow.rs`.
#   Dependencies bullet: none added, none bumped.
# D-tests-obs-harness, cross-doc half: obs-plan §3's keys (read: heartbeat-ticks, and a grep of all eight) carry
#   no harness-verb, status-verdict or `agent-run` claim, and the log format is not changed by the report, so
#   test-plan §3 and obs-plan §3 do not contradict each other. The drift below is test-plan's own §3 / §1 / §9
#   text left behind by the harness change. obs-plan's own expected amendments (§3 graded tick set, §9, §10) are
#   the obs detector's, not proposed here.
# Line numbers below are source-file coordinates the report states; test-plan sites are named by section/row/label.
proposals:
  # ---------- claim A: the `harness:status` / `harness:ready` verdict shapes ----------
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation"
    change: >-
      Under `status` → Verdict JSON: the object has seven members — add `"program": "window" | "console" | "unknown"`
      (the label of the LAST `app.boot.engine` record of the log family; `unknown` when there is no such record or
      its label is neither) between `ended` and `log_file_basename` — and the `verdict` set gains `wrong-program`
      (`running-healthy | stale | wrong-program | not-running | cannot-evaluate`).
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: `harness:status` verdict 6 → 7 members (`program`), verdict words 4 → 5 (`wrong-program`)."
    rationale: >-
      Report Changes → Counts / qualifiers moved: "`harness:status` verdict members 6 → 7 (`program` added)" and
      "verdict words 4 → 5 (`wrong-program`)"; Harness / gate surface spells the seven members in order. The key file
      still shows six members and four words.
    basis: "xtask/src/harness_status.rs:611-634 (the_status_payload_carries_the_closed_member_set); payload at 101-111"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation"
    change: >-
      Under `status` → Exit code semantics: 0 `running-healthy` · 1 `stale` · 1 `wrong-program` · 1 `not-running` ·
      2 `cannot-evaluate`. With `--program`, a `running-healthy` or `stale` run whose log records another program, or
      none, reads `wrong-program`; `not-running` and `cannot-evaluate` stand whatever was asked; with no flag the
      verdict is as before plus the `program` member. `classify` keeps its signature (pure, pinned per arm); the
      program read is `for_program` / `graded_arm` over `program_of`, pinned. The closing sentence "The arms and
      exit codes are unchanged" is scoped to the `ended` chunk it describes or dropped.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: `status` exit semantics gain `wrong-program` (exit 1) and the `--program` grading rule."
    rationale: >-
      Report Harness / gate surface: "Verdicts `running-healthy` 0 · `stale` 1 · `wrong-program` 1 · `not-running` 1 ·
      `cannot-evaluate` 2" with the `--program` rule; Symbols / APIs: "`classify` keeps its signature; its 13 test
      sites did not move". The label lists four arms and ends "The arms and exit codes are unchanged".
    basis: "xtask/src/harness_status.rs:151-160 (graded_arm), 162-167 (for_program), pins 553-600"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation"
    change: >-
      Under `status` → Command body: the verb is `cargo xtask harness:status [--program window|console]`. The sh
      `status` verb takes one optional word — none runs `--program window`, `engine` runs `--program console`, any
      other word is usage, exit 2; `agent-run.ps1 status` still calls it with no flag and takes no `engine` word, so
      the two scripts no longer take the same words.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: sh `status` takes the optional word `engine` and passes `--program`; ps1 unchanged."
    rationale: >-
      Report Symbols / APIs (`scripts/agent-run.sh`): "`status` runs `harness:status --program window`, `status
      engine` `--program console`"; "any other word is usage, exit 2"; "`agent-run.ps1` is unchanged, so the two
      scripts no longer take the same words". The label says the verb is invoked identically by both scripts.
    basis: "scripts/agent-run.sh:38-49 (usage), 56-64 (the word read); xtask/src/harness_status.rs:39-56 (Program)"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation"
    change: >-
      Under `boot` → Readiness signal: the poll is `cargo xtask harness:ready --program window` (bare `boot`) or
      `--program console` (`boot engine`); the object has six members — `{verdict: ready | not-ready | wrong-program |
      ended | cannot-evaluate, pid, ended, program, otlp_grpc, otlp_http}` — exit 0 `ready` · 1 `not-ready`,
      `wrong-program` or `ended` · 2 `cannot-evaluate`; `wrong-program` whatever the receivers answer, and a dead pid
      is `ended` whatever was asked. `decide_ready` gains the arm `wrong-program → wrong-program`. The ps1 verb still
      polls with no flag, which reads as before.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: `harness:ready` verdict 5 → 6 members (`program`), verdict words 4 → 5 (`wrong-program`), `--program` flag."
    rationale: >-
      Report Counts / qualifiers moved: "`harness:ready` verdict members 5 → 6 (`program` added) … its verdict words
      4 → 5 (`wrong-program`)"; Harness / gate surface gives the six members and exits. The label still prints the
      five-member, four-word object.
    basis: "xtask/src/harness_ready.rs:490-522 (the_ready_payload_carries_the_closed_member_set); payload 89-104; pins 459-488"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§1 Test Scope Summary — Test harness requirements (the one-line summary)"
    change: >-
      One site, both retired claims: `boot` reads "(the window app `pulse-app`, or with the word `engine` — sh only —
      the console engine `andromeda-pulse-engine run`; …; ready is the `cargo xtask harness:ready --program
      window|console` verdict …)", and `status` reads "`cargo xtask harness:status [--program window|console]` —
      verdict JSON `{verdict, pid, ended, program, log_file_basename, last_write_age_seconds, stale_after_seconds}` …
      a run of the other program, or with no boot record, is `wrong-program` — exits 0/1/1/1/2".
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: §1 harness summary follows §3 — `boot`/`status` take `engine`; status object seven members, exits 0/1/1/1/2."
    rationale: >-
      The summary restates §3: it names `boot` as "Tauri `pulse-app`" and spells the six-member status object with
      "exits 0/1/1/2". Report Symbols / APIs and Harness / gate surface retire both (Expected amendments 2 names "the
      harness requirements summary").
    basis: "xtask/src/harness_status.rs:611-634; scripts/agent-run.sh:56-64"
    dependent-of: D-tests-obs-harness
  # ---------- claim B: "`boot` builds and spawns `pulse-app` alone / no harness verb boots the console engine" ----------
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation"
    change: >-
      Under `boot` → Command body: still five verbs, no sixth; `boot` (and `status`) take one optional word in the sh
      script — none is the window app, `engine` is the console engine, any other word is usage, exit 2, before a
      directory is made. `boot engine` pre-builds `cargo build --bin andromeda-pulse-engine --release` and xtask,
      spawns `target/release/andromeda-pulse-engine run` BY PATH under the same waiting wrapper (spawn record and exit
      record as for the window app) and polls `harness:ready --program console`. The exit-witness arm is the window
      program's alone: `boot engine` never reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, sets no preload, and neither
      removes nor creates the witness file. The `engine` word is sh-only, as the witness arm is; `agent-run.ps1` is
      unchanged. No committed shell-level test (the §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage`
      trigger); the word is read end to end by `cargo xtask harness:engine-cycle`.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: `boot engine` spawns `target/release/andromeda-pulse-engine run`; word set closed (usage exit 2); witness arm window-only; sh-only."
    rationale: >-
      Report Symbols / APIs (`scripts/agent-run.sh`): "still five verbs. `boot` and `status` take one optional word …
      `boot engine` pre-builds … spawns `target/release/andromeda-pulse-engine run` by path … `boot engine` never
      reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`". The label describes one program only (`cargo build --bin pulse-app
      --release`, `target/release/pulse-app[.exe]`).
    basis: "scripts/agent-run.sh:38-49, 56-64"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → PID file"
    change: >-
      Lifecycle, last bullet: replace "The harness verbs are not aimed at the console program although it writes the
      file: `boot` builds and spawns `pulse-app` alone, and `status` / `cleanup` read whatever pid the file holds"
      with: since chunk 2026-10-10-agent-harness-drives-the-console-engine the sh verbs are aimed at either program —
      `boot engine` builds and spawns `andromeda-pulse-engine run`; `status` / `boot`'s readiness poll ask for a
      program and read `wrong-program` when the log family's last `app.boot.engine` record names the other one or
      none; `cleanup` still terminates whatever pid the file holds. Lifecycle, first bullet: the `ended` record is
      also what `cargo xtask check:engine-log` reads (through `read_ended`) for its `process-end` arm.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: PID-file lifecycle — harness verbs now aim at the console program too; `wrong-program` distinguishes the two writers."
    rationale: >-
      Report Expected amendments 1 names "the PID file lifecycle's last bullet"; Symbols / APIs: `boot engine` spawns
      the engine, `program_of` "the LAST boot record of the family decides"; Harness / gate surface: `check:engine-log`
      reads "the harness's exit record beside the resolved pid file through `read_ended`".
    basis: "xtask/src/harness_status.rs:132-141 (program_of), 143-149 (read_program)"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§1 Test Scope Summary — Surfaces under test, row `console engine program (andromeda-pulse-engine run / version)`"
    change: >-
      Notes column: strike "no harness verb boots it"; say instead that since chunk
      2026-10-10-agent-harness-drives-the-console-engine `bash scripts/agent-run.sh boot engine` / `status engine`
      (sh only) boot and read it, and `cargo xtask harness:engine-cycle` runs it end to end (boot → injector feed →
      `harness:engine-settled` → status → cleanup → `check:engine-log`) on the dev host and as the `boot` job's
      `Console engine cycle` step on every push — beside the `cargo nextest run --workspace` spawned-binary test,
      which stands. Driver column: add the harness path as a second driver.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: console-engine surface row — a harness word and a CI cycle now boot it."
    rationale: >-
      The row says "no harness verb boots it". Report Symbols / APIs: `boot engine` exists; Harness / gate surface:
      `harness:engine-cycle` and the CI step; Expected amendments 2 names this row.
    basis: "scripts/agent-run.sh:56-64; .github/workflows/ci.yml:366-374"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      Under Spawned-program witness form: the two closing sentences no longer hold as written — replace "so the
      console program is run on every push with no workflow step and no harness verb of its own" and "since the
      window smoke boots `pulse-app` alone" with: the test still runs it on every push inside `cargo nextest run
      --workspace`; since chunk 2026-10-10-agent-harness-drives-the-console-engine it ALSO has a harness word (`boot
      engine` / `status engine`) and a workflow step (`Console engine cycle` in the `boot` job).
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: Spawned-program witness form — the console program now has a harness word and a workflow step."
    rationale: >-
      Same claim as the PID-file bullet, restated here as "no workflow step and no harness verb of its own". Report
      Harness / gate surface (CI): step `Console engine cycle`, `if: always()`, directly after `cargo xtask ci-gates`.
    basis: ".github/workflows/ci.yml:366-374; pulse-app/tests/quality_gate_workflow.rs:695-792"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      Under Direct-binary smoke variant, the parenthetical "the harness `boot` verb itself IS the direct-binary form …
      it pre-builds then launches `target/release/pulse-app[.exe]` by path": add that `boot engine` (sh only) is the
      same form over `target/release/andromeda-pulse-engine run`.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: direct-binary form — `boot` launches one of two binaries by path."
    rationale: >-
      Report Expected amendments 1 counts `target/release/pulse-app` at 0+2 (two key-file sites); this is the second.
      Report Symbols / APIs: the engine is spawned "by path under the same waiting wrapper".
    basis: "scripts/agent-run.sh:56-64"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      Add a labelled gate form, "Engine cycle gate form (runtime, console engine)": `cargo xtask harness:engine-cycle
      [--data-dir DIR] [--grpc-port N] [--http-port N]` — defaults a fresh `target/engine-cycle/{UTC stamp}` and ports
      24317 / 24318; refused before anything starts as `cannot-evaluate`, exit 2 (`not-linux` · `shared-port`, 4317 or
      4318 · `data-dir-holds-a-log-family`; also `home-unset`, `path-unset`, `data-dir-unusable`,
      `injector-build-failed`); it runs, in order, the injector build, `agent-run.sh boot engine`, `inject_demo
      --sustained --error-pct=0`, `harness:engine-settled`, `status engine`, `cleanup`, `check:engine-log` (after a
      failed boot the settle and status verbs are skipped; cleanup and the check always run), every child under a
      cleared environment with the pinned set and no session bus, runtime dir or display variable; one JSON object on
      stdout, eight members (`verdict` `pass` 0 · `fail` 1 · `cannot-evaluate` 2, `boot`, `settled`, `status`,
      `cleanup`, `check`, `error_records`, `witness_file`), the children's output on stderr. Gate reading: exit 0,
      `"verdict": "pass"`, `"witness_file": "absent"`, `"error_records": 0`, `engine-log: PASS`, `cleanup: clean`.
      Also name its two parts: `cargo xtask harness:engine-settled [--timeout-seconds N]` (default 60, below 20
      `cannot-evaluate`; seven members; `settled` 0 · `ended` 1 · `wrong-program` 1 · `not-settled` 1 ·
      `cannot-evaluate` 2) and `cargo xtask check:engine-log` (seven arms `family`, `program`, `panic`,
      `heartbeat-gap`, `progress`, `process-end`, `budget`; exit 0 · 1 · 2, a FAIL outranks a cannot-evaluate). NOT a
      member of the standard gate set; `check:ingest-progress` and `ci-gates` are unchanged, NEUTRAL arm included.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: new runtime gate form `harness:engine-cycle` (+ `harness:engine-settled`, `check:engine-log`) recorded in §3."
    rationale: >-
      Report Expected amendments 1: "the engine cycle as a runtime gate form"; Counts: "xtask verbs +3:
      `harness:engine-settled`, `harness:engine-cycle`, `check:engine-log`"; the cycle ran as a listed gate in Outcome
      → Gates. §3's gate discipline names every other xtask scenario / gate verb and names none of these three.
    basis: "xtask/src/main.rs:69-97; xtask/src/engine_cycle.rs:87-102, 217-301; xtask/src/engine_log.rs:145-166, 565-578"
  # ---------- claim C: the `boot` job's step chain / "no CI step makes the heartbeat gap check" ----------
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration — stage table, row `Boot smoke (harness)`"
    change: >-
      The job's chain gains two steps: `Console engine cycle` directly after `cargo xtask ci-gates` (`if: always()`,
      `run: cargo xtask harness:engine-cycle --data-dir "$RUNNER_TEMP/andromeda-pulse-engine-data"`, no soft-fail key,
      a data dir of its own — it names no `ANDROMEDA_PULSE_DATA_DIR` and does not share the job's) and `Upload engine
      logs artifact` after the boot-logs upload (`if: always()`, `logs-engine-${{ runner.os }}`, path `${{ runner.temp
      }}/andromeda-pulse-engine-data/logs/` — the log family plus the boot verb's `boot.log` and `build.log` —
      `if-no-files-found: error`, `retention-days: 14`). "no CI step makes the heartbeat gap check" becomes: `ci-gates`
      makes none and the window log has none; for the console engine the cycle's `check:engine-log` `heartbeat-gap`
      arm (a gap over 45 000 ms between consecutive `ingest.tick` / `buffer.tick` / `connection.tick` records) runs
      on every push. Pins: four more workflow tests in `quality_gate_workflow.rs`. As measured on `ci#38065768197`:
      the cycle step `success`, `logs-engine-Linux` 5482 B.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: `boot` job +2 steps (`Console engine cycle`, `Upload engine logs artifact`), named steps 15 → 17; heartbeat-gap now CI-read for the engine."
    rationale: >-
      Report Harness / gate surface (CI) and Workflow pins; Counts: "the `boot` job's named steps 15 → 17"; Expected
      amendments 3. The row's chain ends at the `logs-boot` upload and states "no CI step makes the heartbeat gap
      check", which the cycle's `heartbeat-gap` arm retires for the engine.
    basis: ".github/workflows/ci.yml:366-374, 384-393; pulse-app/tests/quality_gate_workflow.rs:695-792"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration — failure conditions, bullet `The Linux boot-smoke harness cycle fails`"
    change: >-
      Add: since chunk 2026-10-10-agent-harness-drives-the-console-engine the `boot` job also fails when `cargo xtask
      harness:engine-cycle` is non-zero — `fail` exit 1 (any verb exit other than 0 or 2, an ERROR record in the
      engine's log family, or a witness file) or `cannot-evaluate` exit 2 (a refusal, a verb exit of 2, a verb that
      did not run); the step runs `if: always()` whatever the smoke, the series and `ci-gates` returned, with no
      soft-fail key.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: boot-job failure conditions gain the engine cycle step."
    rationale: >-
      The bullet enumerates every way the `boot` job fails and stops at `ci-gates`. Report Harness / gate surface:
      the cycle's verdict mapping and the CI step's `if: always()` / "No soft-fail key".
    basis: ".github/workflows/ci.yml:366-374; xtask/src/engine_cycle.rs:157-179 (decide)"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration — failure conditions, bullet `An upload step of ci.yml finds no file`"
    change: >-
      "each of the six uploads" → "each of the seven uploads" (the seventh, `logs-engine-${{ runner.os }}`, since
      chunk 2026-10-10-agent-harness-drives-the-console-engine).
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: `ci.yml` uploads 6 → 7, each `if-no-files-found: error`."
    rationale: >-
      Report Counts / qualifiers moved: "`ci.yml` upload steps 6 → 7"; Harness / gate surface: the new upload carries
      `if-no-files-found: error`.
    basis: ".github/workflows/ci.yml:384-393"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§1 Test Scope Summary — Coverage triggers, row `performance-budget: WebGPU canvas throughput`"
    change: >-
      Qualify "`cargo xtask ci-gates` grades no perf arm, so the boot job prints no frame line on any run": still true
      of `ci-gates` and of the perf-budget `frame: cannot-evaluate …` line (the `lint-test` `perf:budget` step's
      alone), but since chunk 2026-10-10-agent-harness-drives-the-console-engine the boot job's `Console engine cycle`
      step grades the memory budget arm as required through `check:engine-log` and prints one line `engine-log:
      budget frame and snapshot not graded (a console engine has no producer for either)`.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: boot job now grades a required memory arm (engine cycle) and prints a frame-not-graded line; the perf `frame:` line stays lint-test's."
    rationale: >-
      Report Harness / gate surface (`check:engine-log`): the `budget` arm is `perf_budget::grade_arm(…, Arm::Memory)`,
      required, and the check prints the "frame and snapshot not graded" line; the check runs inside the `boot` job's
      cycle step. A reader of "no frame line on any run" grepping that job's log now finds a line naming frame.
      Lower confidence than the others: the sentence's own subject (`ci-gates`) is unchanged.
    basis: "xtask/src/engine_log.rs:381-413 (budget_arm), pin 1053-1071"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§1 Test Scope Summary — Pending coverage triggers, row `perf-slo-check-arm-coverage`"
    change: >-
      Same qualification on "since the later chunk `cargo xtask ci-gates` grades no perf arm, the boot job prints no
      frame line": add that the boot job's engine cycle step grades the memory arm for the console engine and prints
      the `engine-log: budget frame and snapshot not graded …` line (chunk
      2026-10-10-agent-harness-drives-the-console-engine).
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: `perf-slo-check-arm-coverage` row — same boot-job perf qualification as the §1 coverage-trigger row."
    rationale: >-
      Second occurrence of the same sentence (the audit-trail row restates the coverage-trigger row). Same report
      basis.
    basis: "xtask/src/engine_log.rs:381-413"
    dependent-of: D-tests-obs-harness
  # ---------- D-tests-coverage: new paths with no test at the tier §2 / the pending-trigger precedent asks ----------
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary — Pending coverage triggers, row `harness-cleanup-verdict-and-boot-spawn-shell-coverage`"
    change: >-
      Append "**Widened 2026-10-10-agent-harness-drives-the-console-engine:**" — the sh `boot` / `status` verbs' new
      optional word (`engine` → the console engine; any other word → usage, exit 2, before a directory is made) and
      the `boot engine` spawn arm ship with NO committed shell-level test: the happy path is read end to end by
      `cargo xtask harness:engine-cycle` (dev host twice, `ci#38065768197` once), the usage arm was driven by hand;
      `agent-run.ps1` takes no `engine` word. Pinned on the Rust side: the program read and the `wrong-program` arm
      (`harness_status` 11 pins, `harness_ready` 4). `wrong-program` was read live in one direction only (the window
      asked, a console engine running); the other direction is pinned on constructed input. Owed: a harness-level
      assertion per word arm (none / `engine` / other).
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: shell-coverage trigger widened — the `engine` word and its usage arm have no committed shell test."
    rationale: >-
      Report Coverage of new surfaces: "`boot engine` / `status engine` … driven by hand, no committed shell test — the
      second entry holds the shell-level pins"; Decisions → Found and not owned 5 (the reverse `wrong-program`
      direction "pinned on constructed input and was not driven live") and 7 (ps1). §2's unit tier and this row's own
      precedent ask a per-arm assertion for a harness bounded-input boundary.
    basis: "scripts/agent-run.sh:38-49, 56-64; xtask/src/harness_status.rs:553-600; xtask/src/harness_ready.rs:459-488"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary — Pending coverage triggers (new row, e.g. `engine-cycle-verb-glue-and-engine-log-mutation-coverage`)"
    change: >-
      New row, "**NEW 2026-10-10-agent-harness-drives-the-console-engine.**": the pure decisions of the three new
      verbs are unit-pinned (`engine_log` 45, `engine_cycle` 21) and the cycle is read e2e, but (1) the red reading
      of `check:engine-log`'s arms is against stubs that pass everything (55 of 66 red, then 66 of 66 green) — no
      mutation of the written code, so a check wrong in ONE arm is not shown red; (2) a red cycle has never been
      seen on a runner (a cycle refused before its boot writes no log, and the upload then fails on no file: two red
      steps for one cause); (3) the cycle's orchestration `drive` and the verb glue `run_check` / `run_settled` have
      no unit pin and are read only by the e2e cycle — the report lists refusal pins for `not-linux`, `shared-port`
      and `data-dir-holds-a-log-family`, and none named for `home-unset`, `path-unset`, `data-dir-unusable`,
      `injector-build-failed`. Owed: one mutation per arm of `check:engine-log`; a pin per remaining refusal label; a
      runner reading of a red cycle.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: new pending trigger — engine-log arms not mutation-checked; cycle glue and four refusal labels unpinned; red cycle unseen on a runner."
    rationale: >-
      Report Outcome 3: "The limit is stated there: red against stubs, no mutation of the written code"; Decisions →
      Found and not owned 3 and 4 ("the second entry runs one mutation per arm of `check:engine-log`"). Item (3) is
      inferred from the test names in the report's New text listing for `xtask/src/engine_cycle.rs` (pins 355-397
      cover three refusal labels) — the orchestrator should confirm it before applying; drop (3) if a pin exists.
      Precedent rows: `verify-capability-matrix-verb-glue-coverage`, `engine-boot-rejected-port-and-socket-census-coverage`
      (the latter is NOT discharged by this chunk, per the report).
    basis: "xtask/src/engine_cycle.rs:217-301 (drive), 104-118 (refusal), pins 355-397; xtask/src/engine_log.rs:415-434, 565-578"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary — Pending coverage triggers, row `exit-hook-main-composition-coverage`"
    change: >-
      Append a dated sentence (chunk 2026-10-10-agent-harness-drives-the-console-engine): the console program's own
      panic and at-exit records stay witnessed one level down, in the re-exec children of `engine_boot::init_process`
      (`pulse-app/tests/integration_engine_boot.rs`), and no trigger for a panic or an exit was built in the product;
      the harness side is built instead — `check:engine-log`'s `panic` arm and its `process-end` arm
      (`unloggable-end` on a `signal 9 (KILL)` exit record, else `end-not-recorded`) — pinned on constructed logs, so
      those FAIL readings are unit-tier only and not driven on a real console process.
    sidecar: "2026-10-10-agent-harness-drives-the-console-engine: console program's panic / at-exit witness stays in `init_process` children; no product trigger built; engine-log panic and process-end FAIL arms pinned on constructed logs."
    rationale: >-
      Report Expected amendments 2: "the pending row stating C4's first half … the console program's own panic and
      at-exit records stay witnessed in children of `engine_boot::init_process` and no trigger is built"; Reverted /
      negative API facts: not written, "a trigger in the product for a panic or an exit". The plan adds the reason
      (inputs#I2: the console program has no input that reaches a panic once its sink exists) — that sentence is the
      plan's, not the report's; the orchestrator decides whether to carry it.
    basis: "xtask/src/engine_log.rs:230-246 (panic_arm), 345-379 (process_end_arm)"
```

### test-plan — dispositions (Validate)

Coordinates checked against `New text, by line`: harness_status 101-111, 132-141, 143-149, 151-160, 162-167,
553-600 (a span), 611-634, 39-56 (a span); harness_ready 89-104, 459-488 (a span), 490-522; agent-run.sh 38-49,
56-64; main.rs 69-97 (an added range); engine_cycle 87-102, 104-118, 157-179, 217-301, 355-397 (a span); engine_log
145-166, 230-246, 345-379, 381-413, 415-434, 565-578, 1053-1071; ci.yml 366-374, 384-393; quality_gate_workflow
695-792.

- T1, T2, T3, T4, T6 (`§3 → 5-command implementation`: the status object and exits, the `status` and `boot`
  bodies, the readiness object) — **apply** (the key file). Check 1: `Accurate this-chunk addition`; check 5: EA 1.
- T5 (§1 harness requirements summary) — **apply**. Check 5: EA 2.
- T7 (`§3 → PID file`, lifecycle) — **apply** (the key file). Check 5: EA 1.
- T8 (§1 console engine surface row) — **apply**. Check 5: EA 2.
- T9, T10, T11 (`§3 → Per-chunk gate discipline`: the two stale sentences; the engine cycle gate form) — **apply**
  (the key file). Check 5: EA 1.
- T12, T13, T14 (§9: the boot job's chain, its failure conditions, six uploads → seven) — **apply**. Check 5: EA 3.
- T15, T16 (§1: "the boot job prints no frame line") — **apply**, as a qualification. Read at both sites: the
  sentence's subject is the perf-budget `frame:` line, which stays `lint-test`'s; the boot job's cycle step now
  prints one `engine-log: budget frame and snapshot not graded …` line and grades the memory arm.
- T17 (the shell-coverage pending row, widened) — **apply**. Check 4: the absence (no committed shell test) is the
  report's own Coverage row.
- T18 (a new pending row) — **apply**. Check 4: items 1 and 2 are the report's (Outcome 3; Found and not owned 3,
  4). Item 3 the detector inferred from the listing; verified by the orchestrator's own search before applying —
  `grep -n 'home-unset\|path-unset\|data-dir-unusable\|injector-build-failed\|fn drive\|run_check\|run_settled'
  xtask/src/engine_cycle.rs xtask/src/engine_log.rs`: seven hits, all in `drive` (219, 236, 237, 238, 239, 258) or the
  two definitions (`engine_log.rs` 416, 566); no test names any of the four labels. Its owner is pinned at
  route-resolve.
- T19 (the `exit-hook-main-composition-coverage` row: where the console program's panic and at-exit witness is
  made, and why no trigger is built) — **apply**. Check 5: EA 2; inputs#I2 answer 2 names the sentence; inputs#I4
  item 3. The reason is carried with its source (research.md 119-121, the search it records), not as a measurement
  of this wrap.

## obs-plan

verdict: 12 proposals (D-obs-defect-narrative 11, D-obs-pii 1; D-obs-instrumentation, D-obs-stack: none).

```yaml
# D-obs-instrumentation: no drift. Report Coverage: "No UI element, no product external surface, no hot-path operation was added"; every new surface reads instrumentation n/a; no file under pulse-app/src is in the diff.
# D-obs-stack: no drift. Report Dependencies: none added, none bumped; the new xtask modules print verdict lines to stdout and use chrono, which xtask already declares; no logger or OTel setup is added.
# D-obs-pii: one item, the report's own "PII raw✗ for build.log only" Coverage row (last proposal). The new verbs' lines and verdict objects read redacted✓ with pins; no product logging was added.
# D-obs-defect-narrative: violated. §10 CI gates states two CI readings as unmet and carried on the route, and perf:budget as the whole perf gate; the chunk built the engine reading of each. The DuckDB defect 1-4 narrative, its lead-in tally and its owner are not touched by this chunk.
# No obs-plan line number is cited below: sites are named by section and bullet; every source coordinate is one the report states.
proposals:
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → CI gates → the `Heartbeat tick stalls` bullet"
    change: >-
      Replace "NO CI step makes the gap check: its CI enforcement stands unmet and is carried on the working route" with: since chunk 2026-10-10-agent-harness-drives-the-console-engine the gap check is made in CI for a console engine's log, by the `heartbeat-gap` arm of `cargo xtask check:engine-log`, which the `boot` job's `Console engine cycle` step (`cargo xtask harness:engine-cycle`, `if: always()`, no soft-fail key, directly after `ci-gates`) runs last over the cycle's own log family — per target of `ingest.tick`, `buffer.tick`, `connection.tick`: a gap over 45 000 ms between two consecutive records FAIL; fewer than two records, or a record with no readable timestamp, cannot-evaluate and never PASS; `viz.tick` and `plugins.tick` are not read. The cycle's settle verb waits for two records of each of the three ticks, so the graded log holds a consecutive pair. No CI step makes the gap check over the window app's log: `ci-gates` is unchanged and has no heartbeat arm, so the window's `viz.tick` / `plugins.tick` (and its three engine ticks) stay ungraded in CI. Name `connection.tick` in the bullet's engine tick list. As measured on `ci#38065768197`: the step read `success` and the downloaded artifact graded again as `engine-log: PASS`.
    sidecar: "obs-plan §10 CI gates (heartbeat): the gap check is CI-enforced for a console engine log by check:engine-log's heartbeat-gap arm in the boot job's Console engine cycle step; unmet for the window log only (chunk 2026-10-10-agent-harness-drives-the-console-engine)."
    rationale: >-
      Report Changes → Harness / gate surface: the `heartbeat-gap` arm of `check:engine-log` and the CI step `Console engine cycle`; Counts: the boot job's named steps 15 → 17; Reverted / negative API facts: no heartbeat arm on `ci-gates`. Expected amendments 4 names this bullet ("a tick gap over 45 s fails, and the step runs on every push"). Outcome criteria 4 and 8 met. §10 states the enforcement as unmet with an owner pointer to the route; the chunk built the engine reading, so the status and the pointer are stale as written.
    basis: "xtask/src/engine_log.rs 248-293 (tick_gap, heartbeat_arm); pins 841-868 and 909-924; .github/workflows/ci.yml 366-374; pulse-app/tests/quality_gate_workflow.rs 695-718"
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → CI gates → the `Zero records in the app's log` bullet"
    change: >-
      The sentence "a TEST RUN that produces zero spans fails the build ... is made by no CI step and stands unmet ... carried on the working route" should now say: the engine reading is made since chunk 2026-10-10-agent-harness-drives-the-console-engine by `cargo xtask check:engine-log` in the `boot` job's `Console engine cycle` step — a log family whose members hold no record FAILs (`family` arm; no member is cannot-evaluate and nothing else is graded), and a cycle in which no span landed FAILs (`progress` arm: a largest `rows_ingested` of 0; no `buffer.tick`, or no numeric `rows_ingested` on any tick, is cannot-evaluate, never PASS). The test-run form stays made by no step: `cargo xtask test` still returns nextest's own status and reads no log. The `zero-spans` arm of `ci-gates` over the boot smoke's log is unchanged.
    sidecar: "obs-plan §10 CI gates (zero records): the zero-spans reading is made for the console engine by check:engine-log's family and progress arms on every push; the test-run form is still made by no step (chunk 2026-10-10-agent-harness-drives-the-console-engine)."
    rationale: >-
      Report Changes → Harness / gate surface: `family` and `progress` arms; "`cargo xtask check:ingest-progress` and `cargo xtask ci-gates` are unchanged". Expected amendments 4: "For the engine: a log with no record fails, a run in which no span landed fails". The bullet's "made by no CI step and stands unmet" plus its route pointer no longer match what the chunk built and measured green on `ci#38065768197`.
    basis: "xtask/src/engine_log.rs 179-197 (family_arm), 295-343 (progress_arm); pins 783-791 and 942-950"
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Standard+ invariants → the `Heartbeat ticks` bullet"
    change: >-
      After "asserted by `cargo xtask check:ingest-progress`" add: that verb prints NEUTRAL and exits 0 over a log family holding no `buffer.tick` (unchanged at chunk 2026-10-10-agent-harness-drives-the-console-engine), so it asserts nothing over a log with no tick; the required readings are `cargo xtask check:engine-log`'s over a console engine cycle — `heartbeat-gap` for the >45s absence (the three engine ticks, never `viz.tick` / `plugins.tick`) and `progress` for drain progress (a stall announcement that is not a recovery FAIL; no `buffer.tick` or no numeric `rows_ingested` cannot-evaluate; a largest `rows_ingested` of 0 FAIL).
    sidecar: "obs-plan §10 Standard+ Heartbeat ticks: names check:ingest-progress's NEUTRAL exit 0 over a log with no buffer.tick and the engine check's heartbeat-gap and progress arms as the required readings."
    rationale: >-
      Second §10 occurrence of the liveness / progress enforcement claim the primary retires. Report Outcome → Gates: `cargo xtask check:ingest-progress` exit 0, "it printed NEUTRAL, no `buffer.tick` in the default log family"; Changes: NEUTRAL arm unchanged, `progress` arm built; Decisions → Found and not owned 8: "the NEUTRAL line of the standard entry is named in the amended obs-plan §10 sentence".
    basis: "xtask/src/engine_log.rs 295-343; pins 952-971"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§3 → Heartbeat ticks"
    change: >-
      In `Stall detection`: (a) LIVENESS gains — for a log whose `app.boot.engine` record reads `console`, the >45s gap is graded by the `heartbeat-gap` arm of `cargo xtask check:engine-log` over `ingest.tick`, `buffer.tick` and `connection.tick` only (fewer than two records of a target, or an unreadable timestamp, is cannot-evaluate), and `viz.tick` / `plugins.tick` are never read; (b) PROGRESS gains — `cargo xtask check:ingest-progress` reads NEUTRAL, exit 0, over a family with no `buffer.tick`, and the reading that cannot be neutral is the `progress` arm of `check:engine-log`.
    sidecar: "obs-plan §3 Heartbeat ticks key: the graded tick set of a console log (three engine ticks, never viz/plugins) and the non-neutral progress reading are check:engine-log's."
    rationale: >-
      The key restates the stall signal and "asserted by `cargo xtask check:ingest-progress`" (report site count `check:ingest-progress` obs-plan 2+1, `viz.tick` 2+1, `connection.tick` 1+1). Expected amendments 6: "the `heartbeat-gap` arm grades `ingest.tick`, `buffer.tick`, `connection.tick` and never `viz.tick` or `plugins.tick`". Key file read whole: .andromeda/registries/contracts/obs-plan/heartbeat-ticks.md.
    basis: "xtask/src/engine_log.rs 285-293; pin 909-924"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → CI gates → the `Build fails if a perf-budget arm exceeds its SLO` bullet"
    change: >-
      "`perf:budget` is the whole perf gate" should now say: `perf:budget` (lint-test) and, since chunk 2026-10-10-agent-harness-drives-the-console-engine, the `budget` arm of `cargo xtask check:engine-log` in the `boot` job are the perf gates — the engine check calls the same grader (`perf_budget::grade_arm`, memory arm, required) over the console engine cycle's log: over 512 000 000 B, a non-numeric value, no sample, or only zero samples FAIL; it prints one line saying frame and snapshot are not graded (a console engine has no producer for either). `ci-gates` still grades no perf arm. The owner clause for readings of the engine on its node (`Engine memory measured`, P-090, and the load entries) stands: the chunk claims none of them, and the graded value is the row-count gauge, not RSS.
    sidecar: "obs-plan §10 CI gates (perf): check:engine-log's budget arm is a second CI caller of the perf grader, memory required over the console engine's log; perf:budget is no longer the whole perf gate."
    rationale: >-
      Report Changes → Symbols / APIs: `engine_log` calls `perf_budget::{read_family, grade_arm}`, neither changed; Harness / gate surface: the `budget` arm and the not-graded line; Expected amendments 4: "the engine check's required progress and memory arms". The bullet's exclusive claim is stale once a second step grades a perf arm on every push.
    basis: "xtask/src/engine_log.rs 381-413 (budget_arm); pins 1016-1023 and 1053-1071"
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Performance budgets → the `Buffer memory bounded` row"
    change: >-
      Beside "enforced on CI by `cargo xtask perf:budget --require memory,snapshot` (lint-test Linux)" add: and, over a console engine cycle's log, by the `budget` arm of `cargo xtask check:engine-log` in the `boot` job (same grader, memory required; no sample or only zero samples FAIL). "No gate bounds the app's RSS" stands.
    sidecar: "obs-plan §10 Buffer memory row: a second CI enforcement site, check:engine-log's budget arm over the console engine's log."
    rationale: "Same claim as the perf bullet (who enforces the memory budget on CI), restated in the budget table. Report Harness / gate surface: `budget` arm."
    basis: "xtask/src/engine_log.rs 381-413"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 → DuckDB Connection Isolation & Load-Profile Operational Constraints → `Load-profile constraints`"
    change: >-
      The caller set of the perf grader ("`perf:load-profiles` alone grades with no arm required ... a caller that EXPECTS samples names the arm with `perf:budget --require`") gains a third caller: `cargo xtask check:engine-log` requires the memory arm in-process through `perf_budget::grade_arm` and has no NEUTRAL reading — it states frame and snapshot as not graded on one line and never passes over an input it cannot grade.
    sidecar: "obs-plan §10 Load-profile constraints: check:engine-log named as a caller of the perf grader that requires memory without `--require`."
    rationale: "The paragraph enumerates who grades with which arms required; the chunk added a caller (report Symbols / APIs, `xtask::engine_log`; Outcome criterion 4)."
    basis: "xtask/src/engine_log.rs 381-413"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§9 CI Integration → Telemetry artifact handling → the `Log file` row"
    change: >-
      The row's "two uploads" becomes three log uploads: add the `boot` job's `logs-engine-${{ runner.os }}` — path `${{ runner.temp }}/andromeda-pulse-engine-data/logs/`, `if: always()`, `if-no-files-found: error`, `retention-days: 14`, placed after `Upload boot logs artifact`; it carries the console engine cycle's whole `logs/`: the log family `check:engine-log` grades plus the boot verb's `boot.log` and `build.log`. As read on `ci#38065768197`: `logs-engine-Linux` 5482 B beside `logs-boot-Linux` 39057 B. `ci.yml` holds seven upload steps.
    sidecar: "obs-plan §9 Log file row: the boot job's third log upload, logs-engine-Linux (the console engine cycle's logs/), added at chunk 2026-10-10-agent-harness-drives-the-console-engine."
    rationale: >-
      Occurrence of the retired claim in §9 (CI reads and keeps the boot smoke's log alone). Report Changes → Harness / gate surface (CI): step `Upload engine logs artifact`; Counts: `ci.yml` upload steps 6 → 7; Cross-project: the two artifacts of `ci#38065768197`. Expected amendments 5.
    basis: ".github/workflows/ci.yml 384-393; pulse-app/tests/quality_gate_workflow.rs 755-792"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§9 CI Integration → Pipeline integration → the closing paragraph (`The one app log a run keeps ...`)"
    change: >-
      "The one app log a run keeps is the `boot` job's (`logs-boot-Linux`)" should now say a run keeps two app logs, both from the `boot` job: the window boot smoke's (`logs-boot-Linux`) and the console engine cycle's (`logs-engine-Linux`).
    sidecar: "obs-plan §9: a run keeps two app logs (logs-boot-Linux, logs-engine-Linux), not one."
    rationale: "Falsified by the report's CI upload `logs-engine-Linux` (Harness / gate surface; artifact `11675214119` on `ci#38065768197`)."
    basis: ".github/workflows/ci.yml 384-393"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§9 CI Integration → CI failure → artifact triage workflow"
    change: >-
      Add the engine artifact to the first two bullets: the `boot` job runs the console engine cycle and its upload whatever the earlier steps returned (`if: always()` on both), and the upload fails its own step when it finds no file; the agent downloads it with `gh run download <run_id> -n logs-engine-Linux` and can grade it again with `cargo xtask check:engine-log`.
    sidecar: "obs-plan §9 triage: logs-engine-Linux named as the console engine's artifact and its download line."
    rationale: "The triage list names `logs-boot-Linux` as the artifact of a failed boot job; the chunk added a second one (report Harness / gate surface, CI; Outcome → Gates: the artifact downloaded and graded, exit 0)."
    basis: ".github/workflows/ci.yml 366-374 and 384-393"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§9 CI Integration → Telemetry artifact handling → the `Snapshot markdown (on test failure)` row"
    change: >-
      The owner clause "the route entry `Engine end-to-end gate reachable`, which keeps the engine's log in CI" should state that the engine's log is already kept in CI since chunk 2026-10-10-agent-harness-drives-the-console-engine (the `boot` job's `logs-engine-Linux`); the snapshot-on-failure bans still stand unmet under that entry, which builds the engine's own CI job.
    sidecar: "obs-plan §9 Snapshot row: the owner clause no longer reads as if the engine's log were not yet kept in CI."
    rationale: >-
      An owner pointer whose stated deliverable the chunk partly delivered: the report's CI upload keeps the engine's log on every push. The entry itself is not discharged — the report still names it as the one that builds the engine's CI job (Decisions → Found and not owned 1) — so only the clause's wording is stale.
    basis: ".github/workflows/ci.yml 384-393"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-pii
    severity: escalate
    section: "§9 CI Integration → Telemetry artifact handling → the `Log file` row"
    change: >-
      Where the row lists the `logs-engine-Linux` contents, state that `build.log` is cargo's own output and its `Compiling` lines carry the runner's checkout paths in full — a harness file, not the engine's allowlisted record, and outside §8's per-module allowlist — and that the class of the two harness files (`boot.log`, `build.log`) in that upload is the operator's word, not yet given; the log family itself is the allowlisted record.
    sidecar: "obs-plan §9 Log file row: logs-engine-Linux's build.log carries the runner's checkout paths raw; classification of the upload's two harness files is pending the operator."
    rationale: >-
      Report Coverage row for the CI step and upload: "PII raw✗ for `build.log` only (cargo's `Compiling` lines carry the runner's checkout paths; an artifact, not the repository; the log family itself is the engine's allowlisted record)". Expected amendments 9 and Decisions → Found and not owned 2: the operator's classification at P4 named "the engine's own log family" and did not state the two harness files; the lean is that the operator rules, and the upload path is narrowed in the second entry if the class does not cover them. What this is not: no user input, no OTLP payload and no product log line is involved, no product logging was added, and the report says the boot upload already has the same shape. Raised because the report marks the surface raw and the detector's severity is escalate; the uncontested upload facts are the separate warning proposal on the same row.
    basis: ".github/workflows/ci.yml 384-393"
```

### obs-plan — dispositions (Validate)

Coordinates checked against `New text, by line`: engine_log 179-197, 248-293 (a span), 285-293, 295-343, 381-413,
783-791, 841-868, 909-924, 942-950, 952-971 (a span), 1016-1023, 1053-1071; ci.yml 366-374, 384-393;
quality_gate_workflow 695-718, 755-792.

- O1 (§10 CI gates, heartbeat), O2 (§10 CI gates, zero records) — **apply**. Check 1: `Accurate this-chunk
  addition`; check 5: EA 4. Each keeps what is still unmet: no CI step grades the window log's gaps; the test-run
  form of the zero-spans reading is made by no step.
- O3 (§10 Standard+ Heartbeat ticks: the NEUTRAL reading named) — **apply**. Report Found and not owned 8.
- O4 (`§3 → Heartbeat ticks`) — **apply** (the key file). Check 5: EA 6.
- O5, O6, O7 (§10: the perf gate, the memory row, the grader's callers) — **apply**, re-derived: the clause "the
  graded value is the row-count gauge, not RSS" in O5's change line is the master's own standing text, not a fact
  of the report, and is not re-stated by the amendment.
- O8, O9, O10 (§9: the log upload, the app logs a run keeps, the triage list) — **apply**. Check 5: EA 5.
- O11 (§9 Snapshot row's owner clause) — **apply**: the engine's log is kept in CI since this chunk; the two bans
  stay unmet under the same owner.
- O12 (D-obs-pii, escalate: `build.log` carries the runner's checkout paths) — **escalate**. Check 1: no rule
  matches the classification itself; the escalate-severity rule's clause (a) holds (no user input, no OTLP payload,
  no product log line; the boot upload has the same shape), but the class of two harness files the operator's
  reading did not name is the operator's to say. The operator's recorded direction settles the routing: found and
  not owned goes to the route-resolve card with a lean (inputs#I4 item 7). The fact is applied with O8; the class is
  resolved at the card and written then.

## a11y-plan

verdict: proposals: [] — no interactive element added, neither schema changed; comments kept below.

```yaml
proposals: []
# D-a11y-surface: no drift. The report's Changes add no interactive UI element: "No UI element, no product external surface, no hot-path operation was added" (Coverage of new surfaces), every file under pulse-app/ui and pulse-app/src is "not touched, by plan" (Files), and each of the seven new-surface rows reads "a11y n/a". The ten changed paths are xtask, scripts/agent-run.sh, one dev example, ci.yml and one workflow test.
# D-a11y-obs-schema: no drift. The report changes neither schema: "Schema / config: no migration, no config key, no scrub or redaction shape". The new and widened JSON objects (harness:status 7 members, harness:ready 6, harness:engine-settled 7, harness:engine-cycle 8) are harness verdicts on stdout, not the a11y violation record or the obs structured-log record. Key file read: .andromeda/registries/contracts/a11y-plan/structured-violation-json-schema.md, unaffected.
# Checked and left alone (outside both detectors, and still true by the report): a11y-plan's "5-command discipline" (body §9 Pipeline integration; §3 -> Focus management test harness) and "`boot` command starts Tauri app" (§1 Focus Management Test Harness) hold, since agent-run.sh is "still five verbs" and bare `boot` is the window app. The two ci.yml ranges are both inside the `boot` job, so the `a11y` job is unchanged. The report's own site search reads 0 for a11y-plan and none of its nine Expected amendments names it.
# Both source files were read whole (report.md 787 lines, a11y-plan.md 581 lines).
```

### a11y-plan — dispositions (Validate)

Nothing to dispose. Check 5: no Expected-amendments entry names this master.

## Validate — the checks that are not per proposal

- Check 2 (cross-contradiction): none. The four masters describe the upload, the cycle and the two verdict objects
  in the same terms (A7 · S5 · T12 · O8; A4 · T1; A5 · T4).
- Check 3 (intent-consistency): the report's eight deviations were accepted by the operator as recorded
  (inputs#I3); `scope: clean`, no scope record.
- Check 5 (Expected amendments, nine entries): 1 → T1-T4, T6, T7, T9-T11 · 2 → T5, T8, T19 · 3 → T12-T14 ·
  4 → O1, O2, O3, O5 · 5 → O8-O10 · 6 → O4 · 7 → A1-A22, A24 · 8 → A23 · 9 → S1-S6. None left for the orchestrator
  to raise. No ledger-note entry in the plan; P-086's dated note is P7.3's (inputs#I4 item 5). No
  `citation-dispositions.md` row reads `claim false`.
- **Escalation resolved (obs-plan 12, D-obs-pii):** with the operator at the route-resolve card, 2026-10-10
  (inputs#I5, answer 2): "the class covers build.log and boot.log, my reading, same as the boot upload; write it
  so." Applied: security-plan §Security Anti-Patterns → Input item (d) and obs-plan §9's Log file row state it, each
  with a second sidecar entry (`security-plan-entry-2.md`, `obs-plan-entry-2.md`). No rule proposed: a
  classification is the operator's each time. Open escalations: 0.
- Check 6 (disproved claims): the one entry, `plan.md` step 9 ("the job's release build already builds the engine
  binary"), is a chunk-artifact claim: a report entry, no edit; its cost on the runner goes to the card (Found and
  not owned 1).
