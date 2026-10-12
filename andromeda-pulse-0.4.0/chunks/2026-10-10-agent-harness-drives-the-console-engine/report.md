# Report — 2026-10-10-agent-harness-drives-the-console-engine

**Chunk:** the agent harness's five verbs drive the console engine and four checks grade its log
**Date:** 2026-10-10T16:49:04Z
**Commits:** `b32e979f chore(2026-10-10-agent-harness-drives-the-console-engine): operator pre-CI commit` (the one
commit since the chunk base `8394da4f`; `git log --format='%h %s' 8394da4f..HEAD`)

Every coordinate of added text below is copied from the last section, `## New text, by line`. Every diff-shaped
basis is `8394da4f` (the parent of the pre-CI commit) → the work tree.

## Changes (structured — detectors read this)

- **Files:** ten, all in research's two lists (`git diff --name-status 8394da4f -- crates pulse-app xtask scripts
  .github Cargo.toml Cargo.lock`: 8 `M`, 2 `A`, no lockfile or manifest).
  - new: `xtask/src/engine_log.rs` (1301 lines), `xtask/src/engine_cycle.rs` (650 lines);
  - modified: `scripts/agent-run.sh`, `xtask/src/harness_status.rs`, `xtask/src/harness_ready.rs`,
    `xtask/src/main.rs`, `xtask/src/pre_push.rs` (a doc comment, lines 30-31),
    `crates/ingest/examples/inject_demo.rs`, `.github/workflows/ci.yml`,
    `pulse-app/tests/quality_gate_workflow.rs`.
  - not touched, by plan: `scripts/agent-run.ps1`, `xtask/src/ci_gates.rs`, `xtask/src/harness_series.rs`,
    every file under `pulse-app/src`, every file under `pulse-app/ui`.
- **Symbols / APIs:**
  - **`scripts/agent-run.sh`** — still five verbs. `boot` and `status` take one optional word: none is the window
    app, `engine` is the console engine, any other word is usage, exit 2 (`usage()` 38-49; the word read at
    56-64). `boot engine` pre-builds `cargo build --bin andromeda-pulse-engine --release` and xtask, spawns
    `target/release/andromeda-pulse-engine run` by path under the same waiting wrapper (spawn record and exit
    record as for the window app), and polls `cargo xtask harness:ready --program console`. Bare `boot` polls
    `harness:ready --program window`; `status` runs `harness:status --program window`, `status engine`
    `--program console`. `boot engine` never reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`: the witness block runs for
    the window program only, the engine's spawn line sets no preload, and the witness file is neither removed
    nor created on that path. `cleanup`, `logs` and `run` are not edited. Git mode stays 100755
    (`git ls-files -s`). `agent-run.ps1` is unchanged, so the two scripts no longer take the same words: the
    `engine` word is sh-only, as the exit-witness arm is.
  - **`xtask::harness_status`** — `Program` (`Window` | `Console`, a clap value enum, 39-56) with `label()`
    (`window` | `console`, the boot record's own labels); `BOOT_ENGINE_TARGET` = `app.boot.engine`;
    `PROGRAM_UNKNOWN` = `unknown`; `boot_record_program(&Value)` (113-130); `program_of(&[String])` (132-141, the
    LAST boot record of the family decides); `read_program(log_base)` (143-149, over the family through
    `smoke::read_jsonl_lines`, so a file beside the family is not read); `graded_arm(arm, asked, program)`
    (151-160); `for_program(verdict, asked, program)` (162-167); `status_payload` (101-111). `run` now takes
    `Option<Program>`. `classify` keeps its signature; its 13 test sites did not move. Callers of `run`: `main()`
    alone.
  - **`xtask::harness_ready`** — `run_ready` takes `Option<Program>` and reads the status through
    `for_program`; `ready_payload` (89-104); `decide_ready` gains the arm `wrong-program → wrong-program`.
    `run_settled` and its eight-member verdict are not touched. Callers of `run_ready`: `main()` alone;
    `agent-run.ps1` still calls `cargo xtask harness:ready` with no flag, which reads as before.
  - **`xtask::engine_log`** (new) — `State` (`Pass` | `Fail` | `CannotEvaluate`, 56-79); `ArmReading`
    (95-119); `Report` with `overall()` and `lines()` (121-143); `evaluate(log_dir, ended)` (145-166); the seven
    arm functions (`family_arm` 179-197, `program_arm` 199-228, `panic_arm` 230-246, `tick_gap` 248-283 and
    `heartbeat_arm` 285-293, `progress_arm` 295-343, `process_end_arm` 345-379, `budget_arm` 381-413);
    `run_check()` (415-434); `SettleEvidence` (436-467), `settle_evidence` (469-495), `decide_engine_settled`
    (507-525), `settled_payload` (553-563), `run_settled(timeout_seconds)` (565-578). It calls
    `ci_gates::evaluate`, `perf_budget::{read_family, grade_arm}`, `ingest_progress::evaluate`,
    `harness_status::{boot_record_program, resolve_paths, read_ended, end_file, read_pid, pid_alive}` and
    `smoke::read_jsonl_lines`; none of those is changed.
  - **`xtask::engine_cycle`** (new) — `Options` (53-57), `run(&Options)` (87-102), `refusal` (104-118),
    `holds_log_family` (120-122), `child_env` (124-155), `decide` (157-179), `payload` (189-200), `drive`
    (217-301), `command` (303-314, a child under a cleared environment), `spawn_injector` (325-333);
    `DEFAULT_GRPC_PORT` 24317, `DEFAULT_HTTP_PORT` 24318.
  - **`inject_demo`** — `resolve_port(Option<&str>)` (302-314): the endpoint is `http://127.0.0.1:{port}`, the
    port from `ANDROMEDA_PULSE_OTLP_GRPC_PORT` (unset: 4317; a set value that does not parse as a port from 1 to
    65535, a blank one included: `inject_demo: … must be a port from 1 to 65535`, exit 2, nothing sent; the
    message never repeats the value). The host is fixed at `127.0.0.1`. This is a second reader of that
    registered variable, a dev example, never shipped.
  - **Ports / sockets:** none added to the product. The cycle verb's defaults move the engine's two loopback
    receivers to 24317 and 24318 for its own run and refuse 4317 and 4318.
  - **Env vars:** no new `ANDROMEDA_PULSE_*` name. The six names on added lines are each already in
    architecture.md (`git diff 8394da4f … | grep '^+' | grep -o 'ANDROMEDA_PULSE_[A-Z0-9_]+' | sort -u`, each
    counted in `.andromeda/architecture.md`: `_CORPUS_PASSPHRASE` 5, `_DATA_DIR` 5, `_EXIT_WITNESS_LIB` 3,
    `_OTLP_GRPC_PORT` 3, `_OTLP_HTTP_PORT` 3, `_RETENTION_SECONDS` 1). The cycle verb reads `HOME` and `PATH`
    from its own environment by value, only to build its children's environment, and
    `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` by value, only to pass it on; it prints and writes none of them.
- **Crates / modules:** two modules added to the `xtask` crate (`mod engine_cycle;`, `mod engine_log;`,
  `main.rs` 13-14). No workspace member added or removed.
- **Dependencies:** none added, none bumped; `Cargo.toml` and `Cargo.lock` are not in the diff. `engine_log`
  and `engine_cycle` use `chrono`, which `xtask` already declares.
- **Schema / config:** no migration, no config key, no scrub or redaction shape. Verdict shapes are under
  Harness / gate surface.
- **Spec-master edits:** none (this report precedes the wrap's reconcile).
- **Counts / qualifiers moved:**
  - `harness:status` verdict members 6 → 7 (`program` added; pinned by
    `the_status_payload_carries_the_closed_member_set`, `harness_status.rs` 611-634);
  - `harness:ready` verdict members 5 → 6 (`program` added; `the_ready_payload_carries_the_closed_member_set`,
    `harness_ready.rs` 490-522); its verdict words 4 → 5 (`wrong-program`);
  - `harness:status` verdict words 4 → 5 (`wrong-program`);
  - xtask verbs +3: `harness:engine-settled`, `harness:engine-cycle`, `check:engine-log` (`main.rs` 69-97);
  - `ci.yml` upload steps 6 → 7 (`grep -c 'uses: actions/upload-artifact@'` at the base and in the tree);
  - the `boot` job's named steps 15 → 17 (awk over the job block at the base and in the tree);
  - workspace tests 2976 → 3064, +88 (2976: the prior chunk's `evidence/operator-pass.md`; 3064:
    `Summary [  17.247s] 3064 tests run: 3064 passed, 0 skipped` in the pre-push log on `b32e979f`). The 88 are
    the tests this chunk wrote: `engine_log` 45, `engine_cycle` 21, `harness_status` 11, `harness_ready` 4,
    `inject_demo` 3, `quality_gate_workflow` 4;
  - CI jobs stay six; the pre-push check stays six stages; `pulse-app/tests` stays 109 files.
  - Docs stating the moved values: the leaves and masters that spell the status and readiness verdict objects
    (sites under Expected amendments 1 and 7).
- **Dev-tool versions:** none — no host tool was installed, upgraded or read changed.
- **Harness / gate surface:**
  - **`cargo xtask harness:status [--program window|console]`** — one JSON object, seven members: `verdict`,
    `pid`, `ended`, `program`, `log_file_basename`, `last_write_age_seconds`, `stale_after_seconds`. `program`
    is the label of the LAST `app.boot.engine` record of the log family: `window`, `console`, or `unknown`
    (no such record, or a label that is neither). Verdicts `running-healthy` 0 · `stale` 1 · `wrong-program` 1 ·
    `not-running` 1 · `cannot-evaluate` 2. With `--program`, a `running-healthy` or `stale` run whose log
    records another program, or none, reads `wrong-program`; `not-running` and `cannot-evaluate` stand whatever
    was asked. With no flag the verdict is as before plus the `program` member.
  - **`cargo xtask harness:ready [--program window|console]`** — six members: `verdict`, `pid`, `ended`,
    `program`, `otlp_grpc`, `otlp_http`. Verdicts `ready` 0 · `not-ready` 1 · `wrong-program` 1 · `ended` 1 ·
    `cannot-evaluate` 2. `wrong-program` whatever the receivers answer; a dead pid is `ended` whatever was asked.
  - **`cargo xtask harness:engine-settled [--timeout-seconds N]`** (default 60; below 20 is `cannot-evaluate`)
    — polls every 500 ms until the log family's boot record reads `console`, the pid is alive, the family holds
    at least two records of each of `ingest.tick`, `buffer.tick`, `connection.tick`, and one
    `metric.buffer.memory_bytes` sample with a non-zero `value`. Seven members: `verdict`, `pid`, `program`,
    `ingest_ticks`, `buffer_ticks`, `connection_ticks`, `memory_samples_populated`. Verdicts `settled` 0 ·
    `ended` 1 · `wrong-program` 1 · `not-settled` 1 · `cannot-evaluate` 2 (no pid, an unprobeable pid, the
    timeout floor). It writes no file.
  - **`cargo xtask check:engine-log`** — reads the family `agent-latest.jsonl*` in the directory of the
    resolved log base, and the harness's exit record beside the resolved pid file through `read_ended`. One
    line per arm, `engine-log: {arm} {PASS|FAIL|cannot-evaluate} ({detail})`, one line
    `engine-log: budget frame and snapshot not graded (a console engine has no producer for either)`, and a
    last line `engine-log: PASS` | `engine-log: FAIL` | `engine-log: cannot-evaluate`; exit 0 · 1 · 2; a FAIL
    on any arm outranks a cannot-evaluate. All lines go to stdout. Arms, in order:
    - `family` — no member: cannot-evaluate and nothing else is graded; members holding no record: FAIL
      (both read from `ci_gates::evaluate`'s verdict);
    - `program` — exactly one `app.boot.engine` record reading `console`: PASS; reading `window`: FAIL; none,
      a label that is neither, or more than one: cannot-evaluate;
    - `panic` — `ci_gates::evaluate`'s `Panic` verdict: FAIL, `app.panic.fatal at {member}:{line}`;
    - `heartbeat-gap` — per target of `ingest.tick`, `buffer.tick`, `connection.tick`: fewer than two records,
      or a record with no readable timestamp: cannot-evaluate; a gap over 45 000 ms between two consecutive
      records: FAIL; else PASS with the largest gap. `viz.tick` and `plugins.tick` are not read;
    - `progress` — `ingest_progress::evaluate`: a stall announcement that is not a recovery: FAIL; no
      `buffer.tick`: cannot-evaluate; no numeric `rows_ingested` on any tick: cannot-evaluate; a largest
      `rows_ingested` of 0: FAIL;
    - `process-end` — exactly one `app.exit` record, the last parsed record of the family: PASS; two or more,
      or a record after it: FAIL; none: FAIL as `unloggable-end` when the harness's exit record reads
      `signal 9 (KILL)`, else `end-not-recorded`;
    - `budget` — `perf_budget::grade_arm(…, Arm::Memory)`, required: over 512 000 000 B, a non-numeric value,
      no sample, or only zero samples: FAIL.
    `cargo xtask check:ingest-progress` and `cargo xtask ci-gates` are unchanged, NEUTRAL arm included.
  - **`cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]`** — defaults: a
    fresh `target/engine-cycle/{UTC second, %Y%m%dT%H%M%SZ}` and ports 24317 / 24318. Refused before anything
    starts, verdict `cannot-evaluate`, exit 2, one stderr line `harness:engine-cycle: cannot-evaluate
    ({label})`: `not-linux` · `shared-port` (4317 or 4318 in either flag) · `data-dir-holds-a-log-family`;
    also `home-unset`, `path-unset`, `data-dir-unusable`, `injector-build-failed`. Then, in order:
    `cargo build -p ingest --example inject_demo`; `bash scripts/agent-run.sh boot engine`;
    `target/debug/examples/inject_demo --sustained --error-pct=0` by path; `cargo xtask harness:engine-settled`;
    the injector killed by its child handle; `bash scripts/agent-run.sh status engine`;
    `bash scripts/agent-run.sh cleanup`; `cargo xtask check:engine-log`. After a failed boot the settle and
    status verbs are skipped; cleanup and the check always run. Every child gets a cleared environment plus
    exactly: `HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_OTLP_GRPC_PORT`,
    `ANDROMEDA_PULSE_OTLP_HTTP_PORT`, `ANDROMEDA_PULSE_RETENTION_SECONDS` = 60,
    `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (32 hex characters made per run from two `RandomState` hashers, never
    printed or written), and `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` only when the verb's own environment holds it.
    No session bus, runtime dir or display variable. The children's stdout is sent to the verb's stderr; the
    verb's stdout carries one JSON object of eight members: `verdict` (`pass` 0 · `fail` 1 ·
    `cannot-evaluate` 2), `boot`, `settled`, `status`, `cleanup`, `check` (each verb's exit, null when it did
    not run, -1 for a child that could not start or was ended by a signal), `error_records` (ERROR-level
    records in the family; null when no cycle ran), `witness_file` (`absent` | `present`,
    `logs/exit-witness.jsonl`). `fail`: any verb exit other than 0 or 2, an ERROR record, or a witness file.
    `cannot-evaluate`: no failure, and a verb exit of 2, a verb that did not run, or no cycle. Else `pass`.
  - **Filesystem locations (harness-written, git-ignored):** `target/engine-cycle/{stamp}/` per cycle, with
    `logs/` (the log family, `boot.log`, `build.log`), `run/` and `corpus/` as the engine and the boot verb
    write them; nothing is removed after a cycle.
  - **CI (`.github/workflows/ci.yml`, the `boot` job):** step `Console engine cycle` (366-374, `if: always()`,
    `run: cargo xtask harness:engine-cycle --data-dir "$RUNNER_TEMP/andromeda-pulse-engine-data"`) directly
    after `cargo xtask ci-gates`; step `Upload engine logs artifact` (384-393, `if: always()`,
    `name: logs-engine-${{ runner.os }}`, `path: ${{ runner.temp }}/andromeda-pulse-engine-data/logs/`,
    `if-no-files-found: error`, `retention-days: 14`, the upload action at the sha the workflow already pins)
    after `Upload boot logs artifact`. No soft-fail key. The upload carries the data dir's whole `logs/`: the
    log family plus the boot verb's `boot.log` and `build.log`.
  - **Workflow pins** (`pulse-app/tests/quality_gate_workflow.rs` 695-792): the cycle step sits after
    `ci-gates`, carries `if: always()` and that exact `run` line; it names no `ANDROMEDA_PULSE_DATA_DIR` and
    the job's own data dir is another directory; no `continue-on-error`, `retry` or `|| true`; the upload
    sits after the cycle with its four lines and is the one upload named `logs-engine-…`.
  - **`xtask/src/pre_push.rs` 30-31:** the seed's doc comment now reads "One boot record and two ticks: the
    `ci-gates` stage reads the record count and the panic read over it." The stage is unchanged.
- **Cross-project / external claims:**
  - **CI:** `ci#38065768197` (event `pull_request`, attempt 1) measured sha `b32e979f`: `verdict: green`,
    checks 7/7, wall 1721 s; `secret-scan#38065768208` success. The run builds the tip merged with `main`: the
    window boot record on the runner carries merge commit `2d098663…`. Jobs read after it closed, all
    `completed/success`. This wrap's own commit adds to that tree and is not covered by it.
  - **Artifacts of that run** (GitHub, repo `Turbolet85/andromeds-pulse`): `logs-engine-Linux` `11675214119`
    (5482 B) and `logs-boot-Linux` `11675169187` (39057 B), both expiring 2026-10-24; read in
    `evidence/operator-pass.md`.
  - **Inputs** (`inputs.py verify`, this wrap, `inputs: 4 entries — unchanged 2 · drifted 0 · vanished 0 ·
    broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 2 · unparsed 0`):
    - I1 · `../additional/pc-overseer/relays/pulse-phase-agent-harness-console-engine-2026-10-10.md` · copy ·
      unchanged (cited by scope, research and plan);
    - I2 · a message, the operator at P4 · copy · n/a, a message has no live source (cited);
    - I3 · a message, the operator after implement's P4 report · copy · n/a. Cited here: inputs#I3 is the
      word the operator pass ran on (`evidence/operator-pass.md` quotes it);
    - I4 · `../additional/pc-overseer/relays/pulse-wrap-agent-harness-console-engine-2026-10-10.md` · copy ·
      unchanged; snapped at this wrap and cited here: inputs#I4 is the wrap directive (ten items; its items 2,
      3, 4, 6 and 7 are route-resolve's and the cascade's, item 8 is why this report ends the first window).
- **Reverted / negative API facts:**
  - The decision bodies of both new modules were first written as stubs (every arm PASS, the cycle verdict
    `pass`) so that the pins could be read red; the stubs were replaced before any other gate ran and none is
    in the tree.
  - `worst()` in `engine_log.rs` was first written over the exit codes' order, which ranks cannot-evaluate
    above FAIL; replaced in the same edit burst by the explicit match (81-93). It never ran.
  - Not written, by a listed rejection each: a new variable to select the program; a sixth `agent-run` verb;
    heartbeat or budget arms on `ci-gates`; a console arm on `harness:settled`; a trigger in the product for a
    panic or an exit; the exit witness on the engine's spawn line; a change to `check:ingest-progress`'s
    NEUTRAL arm; a visibility change in `ci_gates.rs`.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `plan.md` step 9: "The job's release build already builds the engine binary." The job's release build is
    `cargo build --workspace --release --features mcp-server` under the job's environment; on
    `ci#38065768197` the cycle's `boot engine`, under the cycle's cleared environment, compiled `pulse-app`
    again in the release profile (44.61 s) and 14 workspace crates in the dev profile (11.51 s), after a
    25.20 s dev build of the injector. The binary existed; the step still paid a build. Basis: the artifact's
    `build.log` and the boot job's log, in `evidence/operator-pass.md`. A chunk-artifact claim: a report entry,
    no edit to `plan.md`.
  - No master sentence was measured false by this chunk. The master sentences this chunk makes stale are
    under Expected amendments.
- **Expected amendments (from plan):** nine entries. Site search: fixed-string line counts per master body and
  its key files under `.andromeda/registries/contracts/` (`body+keys`; the script
  `site_counts.py` of this wrap's scratchpad, run over the seven masters; a11y-plan, design-system and
  layout-templates read 0 unless named).
  1. **test-plan §3** — the `boot` and `status` bodies, the PID file lifecycle's last bullet, the engine
     cycle as a runtime gate form. **Carried**: Symbols / APIs (`agent-run.sh`) and Harness / gate surface.
     Sites: `agent-run` test-plan 4+6 · `harness:status` 2+6 · `harness:ready` 2+4 ·
     `target/release/pulse-app` 0+2 · `last_write_age_seconds` 1+1 · `stale_after_seconds` 1+1 · `otlp_grpc`
     0+2 · `wrong-program` 0 everywhere (a new word).
  2. **test-plan §1** — the console engine surface row, the harness requirements summary, the pending row
     stating C4's first half. **Carried**: `boot engine` and `status engine` exist (Symbols / APIs); the
     console program's own panic and at-exit records stay witnessed in children of
     `engine_boot::init_process` and no trigger is built (Reverted / negative API facts; inputs#I2;
     inputs#I4 item 3). Sites: `no harness verb` test-plan 1+1 · `integration_engine_boot` 6+1 ·
     `engine-boot-rejected-port` 1+0 (that pending row is NOT discharged here: the second entry holds it).
  3. **test-plan §9** — the boot job gains the cycle step and its upload. **Carried**: Harness / gate
     surface (CI). Sites: `Boot series` test-plan 2+0 · `boot-series` 3+0 · `logs-boot` 1+0 · `ci-gates` 4+1.
  4. **obs-plan §10** — the zero-records and heartbeat bullets of CI gates; the engine check's required
     progress and memory arms. **Carried**: Harness / gate surface (`check:engine-log`, the cycle's CI step).
     For the engine: a log with no record fails, a run in which no span landed fails, a tick gap over 45 s
     fails, and the step runs on every push. Sites: `heartbeat-gap` obs-plan 2+0 · `zero spans` 1+0 ·
     `check:ingest-progress` 2+1 · `ci-gates` 7+0.
  5. **obs-plan §9** — the `logs-engine` upload. **Carried**: Harness / gate surface (CI). Sites:
     `logs-boot` obs-plan 4+0.
  6. **obs-plan §3** — the graded tick set of a console log. **Carried**: the `heartbeat-gap` arm grades
     `ingest.tick`, `buffer.tick`, `connection.tick` and never `viz.tick` or `plugins.tick`. Sites:
     `viz.tick` obs-plan 2+1 · `connection.tick` 1+1.
  7. **architecture §Occupied Resources** — three new verbs, the changed `harness:status`, `harness:ready`
     and `agent-run` contracts, the sentence naming the binary the harness spawns, a second reader of the
     gRPC port variable, `target/engine-cycle/`, the injector's contract. **Carried**: Symbols / APIs and
     Harness / gate surface. Sites: `harness:status` architecture 4+0 · `harness:ready` 4+0 · `agent-run`
     10+0 · `target/release/pulse-app` 1+0 · `ANDROMEDA_PULSE_OTLP_GRPC_PORT` 3+0 · `inject_demo` 3+0 ·
     `target/pre-push` 2+0 (the sibling harness-written location) · `target/boot-smoke` 0 everywhere ·
     `last_write_age_seconds` 2+0 · `otlp_grpc` 3+0.
  8. **architecture §Infrastructure Patterns** — the boot job's two added steps. **Carried**: Harness / gate
     surface (CI). Sites: `Boot series` architecture 1+1 · `logs-boot` 1+0 · `ci-gates` 1+1.
  9. **security-plan §Security Anti-Patterns** — the three harness readings classified routine on the
     operator's reading of 2026-10-10 (inputs#I2); the witness arm stays the window's alone. **Carried**:
     the cycle's data dir, the `logs-engine-Linux` upload, the injector's port read (Symbols / APIs, Harness /
     gate surface), and `boot engine` never reading the witness variable. One fact for that classification
     that the question at P4 did not state: the upload holds `boot.log` and `build.log` beside the log family
     (Found and not owned, 2). Sites: `EXIT_WITNESS` security-plan 3+0 · `agent-run` 3+0 · `harness:ready`
     2+0 · `harness:status` 1+0 · `pre-push:linux` 3+0 · `target/pre-push` 2+0 · `logs-boot` 1+0 ·
     `ANDROMEDA_PULSE_OTLP_GRPC_PORT` 1+0.
  No ledger-note entry is in the plan's list. P-086's dated note is this wrap's by inputs#I4 item 5:
  `ledger-note — owner P7.3`.
- **Coverage of new surfaces:**
  - `boot engine` / `status engine` (a command word of a harness script) → validation mechanism✓ (a closed
    word set, any other word exits 2 before a directory is made; driven by hand, no committed shell test —
    the second entry holds the shell-level pins) · instrumentation n/a · PII n/a · tests e2e (the cycle, dev
    host and CI) · a11y n/a · tokens n/a
  - `harness:status --program` / `harness:ready --program` → validation mechanism✓ (clap value enum) ·
    instrumentation n/a (a verdict object on stdout) · PII redacted✓ (the member is one of three closed
    labels; member sets pinned) · tests unit (15) + e2e (the cycle; the hand-driven `wrong-program` read) ·
    a11y n/a · tokens n/a
  - `check:engine-log` → validation n/a (no argument; paths resolved like `harness:status`) ·
    instrumentation n/a · PII redacted✓ (`no_line_holds_a_record_s_text_or_a_path`, 1108-1126) · tests unit
    (the arm pins) + e2e (the cycle, and the CI artifact graded again on this host) · a11y n/a · tokens n/a
  - `harness:engine-settled` → validation mechanism✓ (the timeout floor) · instrumentation n/a · PII
    redacted✓ (labels and counts; member set pinned) · tests unit + e2e · a11y n/a · tokens n/a
  - `harness:engine-cycle` → validation mechanism✓ (the three refusals; ports parsed as `u16`) ·
    instrumentation n/a · PII redacted✓ (no path and no environment value in the verdict, pinned by
    `the_verdict_text_is_what_the_gate_entry_reads`, 620-635; the passphrase is neither printed nor written)
    · tests unit (21) + e2e (dev host twice, CI once) · a11y n/a · tokens n/a
  - `inject_demo`'s port variable → validation mechanism✓ (`resolve_port`, exit 2) · instrumentation n/a ·
    PII n/a · tests unit (3) + e2e (the cycle's feed on 24317) · a11y n/a · tokens n/a
  - CI step `Console engine cycle` + upload `logs-engine-Linux` → validation n/a · instrumentation n/a · PII
    raw✗ for `build.log` only (cargo's `Compiling` lines carry the runner's checkout paths; an artifact, not
    the repository; the log family itself is the engine's allowlisted record) · tests unit (4 workflow pins) +
    e2e (`ci#38065768197`) · a11y n/a · tokens n/a
  - No UI element, no product external surface, no hot-path operation was added.

## Deviations from intent

Each was in implement's P4 report and was accepted by the operator as recorded (inputs#I3: "Deviations accepted
as recorded; the panic predicate read through the public verdict is the right way round the scope guard").

1. **The panic predicate is shared through `ci_gates::evaluate`'s verdict, not by exposing the private
   function.** Plan step 4 says to share `ci_gates.rs:85`'s predicate; `ci_gates.rs` is outside research's
   lists and the plan's own scope-guard entry holds it unchanged. The `panic` and `family` arms read
   `ci_gates::Verdict`. Nothing is copied and the file is not edited.
2. **The cycle's verdict mapping.** Step 7 gives the pass condition and the three refusals, not which other
   outcome is `fail` and which `cannot-evaluate`. Settled as: a failure outranks a cannot-evaluate; a verb
   exit of 2 or a verb that did not run reads `cannot-evaluate`; a child that cannot start or is ended by a
   signal is exit -1 and a failure; a failed injector build, an unset `HOME` or `PATH` and an unusable data
   dir are `cannot-evaluate` with nothing started.
3. **The children's stdout goes to the verb's stderr.** Step 7 says the children's output passes through;
   the verdict object alone is on stdout (the `pre-push:linux` shape). The merged output holds every line the
   gate entry reads.
4. **Four cannot-evaluate readings the plan does not name** in the log check: a boot record naming no known
   program; a tick with no readable timestamp; buffer ticks with no numeric `rows_ingested`; and, for an
   empty family, the panic arm (FAIL stands on the family arm).
5. **The injector refuses a set-but-blank value**, as the product's own resolver does (`resolve_port` in
   `engine_boot.rs` parses without trimming); `harness:ready` reads a blank value as the default.
6. **`check:engine-log` and `harness:engine-settled` resolve their paths as `harness:status` does**
   (`resolve_paths`: the pid-file and log-file overrides are honoured), where step 4 says "under the resolved
   data dir". With no override the two are the same place.
7. **One pin's assertion was reworded after the red reading**:
   `a_complete_console_log_passes_every_arm_in_order` asserted that an arm's detail was not the stub's word
   and asserts that it is not empty. No pin was added, removed or weakened.
8. **One hand-driven smoke beyond the plan's step 10**: a console engine on 24317 / 24318 read by the
   window-asked verbs (`evidence/live-readings.md`).

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 10 · listed 10 · recorded 0
(companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 54`, base `8394da4f`, this wrap).

## Decisions & corrections

- **The operator's word on the implement report** (inputs#I3): deviations accepted as recorded; run the
  operator pass, entries 17 to 22; a red cycle step or boot job stops the pass with what the job kept
  reported whole; nothing fixed on top; no skill started. Nothing read red.
- **The wrap directive** (inputs#I4): the operator pass stands as recorded (item 1); the second entry is
  minted at this wrap directly after this chunk's line, its card at route-resolve (2); the five carries are
  struck, moved or amended, none left on the completed line (3); a home for the cycle step and its upload
  when the boot job leaves (4); P-086 not claimed, a dated note (5); the last wrap's cascade is finished here
  by recomputing every leaf the four masters feed from its master as it stands (6); found and not owned goes
  to the card with a lean (7); two windows, P1 only in the first (8); `REFUSED id:` and `held 3` reported if
  they occur (9); stops at the route-resolve card and before the flip and the commit (10).
- **The operator's word at the wrap's route-resolve card** (inputs#I5, added in the wrap's second window): the
  second entry is minted as worded and also takes the pending row opened at this wrap; the engine log upload's
  class covers `build.log` and `boot.log` (the operator's reading, as for the boot upload); ten master defects the
  leaf recompute found are named in one `CARRY:` on `Records say what the product is`; the unsourced leaf
  statements stay, listed; the service notes' file-layout lines are corrected here only under a window condition,
  else owed to the next wrap's cascade (they were not: `cascade-dispositions.md` in the wrap run dir); the two
  process departures of the leaf recompute stand as recorded.
- **Decided in this chunk, by the agent, within the plan:** the log check prints every line to stdout with
  no `::error::` prefix; the settle read polls at 500 ms; the passphrase is 32 hex characters; a refusal
  prints one closed label on stderr and adds no member to the verdict.
- **Found and not owned — for the card, each with a lean (inputs#I4 item 7):**
  1. **The cycle's cleared child environment re-fingerprints the build on the runner** (about 56 s inside
     `boot engine` plus 25 s for the injector; the step took 100 s there, 19 to 41 s on the dev host). One
     reading. The job sets `CARGO_INCREMENTAL: 0`, which the children do not carry; the plan pinned the
     child set, so this was not changed. Lean: carry on `Engine end-to-end gate reachable`, which builds the
     engine's CI job, to rule whether the build-affecting part of the job's environment joins the pinned set.
  2. **`logs-engine-Linux` carries `build.log` and an empty `boot.log` beside the log family**, and
     `build.log` holds the runner's checkout paths. The boot upload has the same shape. The operator's
     classification at P4 named "the engine's own log family". Lean: the operator says whether the class
     covers the two harness files as it does for the boot upload; if not, the upload path is narrowed in the
     second entry.
  3. **What the job keeps when the cycle is red has not been seen on a runner.** A cycle refused before its
     boot writes no log, and the upload then fails on no file: two red steps for one cause. Lean: accept; it
     is the designed reading (no gate stands while reading nothing).
  4. **No mutation check was run against the written code.** The red reading is against stubs that pass
     everything, so it shows each failing-input pin red against a check that grades nothing, not against a
     check wrong in one arm. Lean: the second entry runs one mutation per arm of `check:engine-log`.
  5. **The `wrong-program` reading in the other direction** (the console engine asked, a window app
     running) is pinned on constructed input and was not driven live. Lean: accept.
  6. **Cycle data dirs accumulate** under `target/engine-cycle/` (one per run, never removed). Lean: the
     operator's to delete, like the dated caches under `target/pre-push/`; a handoff line.
  7. **`agent-run.ps1` does not take the `engine` word**, so the two scripts no longer share one command
     grammar. By plan. Lean: owned by `Other operating systems retired from the code`; a `CARRY:` there if
     the card wants it pinned.
  8. **The standard gate entry `cargo xtask check:ingest-progress` still prints NEUTRAL** over a data dir
     with no tick (C3). The engine's progress reading is built in `check:engine-log`; the verb itself is
     unchanged by a listed rejection. Lean: C3 is struck as built; the NEUTRAL line of the standard entry is
     named in the amended obs-plan §10 sentence.
- **Sweep hazards met this chunk:**
  - A nextest run's merged log prints each `PASS` line twice in the gate tool's capture, so a
    `grep -c 'PASS .*{module}::'` reads double the test count (90 for 45 tests). Count from the `Summary`
    line, or halve.
  - Clippy's `manual_contains` fires on `step.iter().any(|line| *line == wanted)` over a `Vec<String>`; the
    workflow pins in `quality_gate_workflow.rs` that compare a whole line use `step.contains(&wanted)`.
  - The gate entry's atom `contains engine-log: PASS` cannot be met by an arm line, because every arm line
    has the arm's name between the prefix and the word; a new line of the check must keep that shape.
  - A grep for `"name"` or `"ok"` over the pre-push check's log matches the libtest-JSON test lines (850 kB
    of them), not only the verdict object; read the verdict from the log's tail.
- **Guard refusals met** (each re-issued once in a passing form): a `cat` heredoc with a file target
  (implement, the `main.rs` wiring, re-issued as Edit calls); a leading `cd` into `.andromeda/` (this wrap's
  site counts, re-issued as a scratchpad script).

## Outcome

**Acceptance criteria, each re-asserted against the diff:**

1. (tests) `boot engine` starts the engine by path and returns 0 on `ready` asked with `--program console`;
   `status engine` reads `running-healthy`; `cleanup` prints `cleanup: clean`; the cycle reads
   `"verdict": "pass"` on 24317 and 24318 — **met** (dev host twice, `evidence/live-readings.md`; the runner
   once, `evidence/operator-pass.md`).
2. (tests) A verb asked for one program and pointed at a run of the other reads `wrong-program`, exit 1, and
   a log with no boot record reads the same — **met** (`harness_status.rs` 553-600, `harness_ready.rs`
   459-488, in the workspace run; and live, `status` and `harness:ready --program window` against a running
   console engine).
3. (obs) Each arm of `check:engine-log` is pinned FAIL on a constructed log before the check is read green,
   both readings in `evidence/` — **met** (`evidence/red-before-green.md`: 55 of 66 failed over the stubs,
   then 66 of 66 passed). The limit is stated there: red against stubs, no mutation of the written code.
4. (obs) No arm reads PASS over an input it cannot grade; a `window` boot record fails; `viz.tick` and
   `plugins.tick` are never graded — **met** (pins at `engine_log.rs` 770-821, 880-924, 952-971).
5. (security) A SIGKILL end lands on `unloggable-end`, a missing end on `end-not-recorded`, both FAIL; no
   line of the check or of the verdict objects holds a path, a record's text or an environment value; each
   object's member set is pinned — **met** for the check's lines and for the four verdict objects (status 7,
   ready 6, engine-settled 7, cycle 8). Stated for precision: the cycle verb's stderr passes through the boot
   verb's own `boot: ready (PID=…, data_dir=…)` lines, which print the data dir; those lines are the boot
   verb's, unchanged by this chunk, and are neither a check line nor a verdict object.
6. (security) The engine's spawn line carries no preload (`"witness_file": "absent"` with the library
   named); the children get the pinned set and no session bus; the console program's command line is still
   `run` | `version` — **met** (three cycles with the variable set; `pulse-app/src` is not in the diff).
7. (arch) Code lands in `xtask/`, `scripts/agent-run.sh`, one dev example and tests; no new workspace
   member, no new `ANDROMEDA_PULSE_*` name, no product source changed; the scope guard prints nothing —
   **met** (the six names on added lines are all registered, Symbols / APIs; the diff's ten paths).
8. (tests) The `boot` job runs the cycle on every push whatever its earlier steps returned, with no
   soft-fail key, and uploads the engine's log family under its own name, failing on no file — **met**
   (four workflow pins; `ci#38065768197`, step 17 `success`; the artifact graded again here,
   `engine-log: PASS`). The upload also carries two harness files (Found and not owned, 2).
9. (a11y, tests) The window app's harness path stands — **met** (`all-settled` on two boots here and on
   seven series boots plus the smoke on the runner, every status read `"program": "window"`; the diff's two
   `ci.yml` ranges are both inside the `boot` job; no path under `pulse-app/ui` is in the diff).
10. (tests) The standard gate set passes in the listed order and the bindings close exits 0 — **met**.
11. P-086 is advanced, not claimed — **holds**: `matrix.py show --chunk {marker}` reads `claimed … : 0`.
    Shown: started by a harness verb with no display variable and no session bus, the engine runs detached
    from its starter, is found by its pid file, answers a state read, takes telemetry over its gRPC receiver
    and is ended by signal, and its own log is graded on every push. Not shown, carried by the four other
    entries that name the id: running in the background as a service; every kind of finding 0.3.0 detected;
    one incident for a sustained storm; automatic resolution.

No criterion is contradicted by the diff.

**Gates** (by `run`; the verdict is the final state: implement's second whole firing over the tree the pre-CI
commit then carried, entry by entry the same content as `b32e979f`, and the operator pass for the rest):

- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0 (red once at
  implement: `manual_contains` on two new workflow pins; fixed in `quality_gate_workflow.rs`)
- `git diff --name-only 8394da4f… -- crates pulse-app xtask scripts … (the scope guard)` — green · exit 0 · no
  output
- `cargo nextest run --workspace --profile ci --no-tests=fail -E 'package(xtask) and
  test(/^(engine_log|engine_cycle)::/)'` — green · exit 0 (66 passed; red first by design, over the stubs)
- `d="target/boot-smoke/…-series" && … cargo xtask harness:boot-series --count 2` — green · exit 0 ·
  `"verdict": "all-settled"`, ordinals 2 and 3; artifact fresh
- `mkdir -p target/exit-witness && cc … && ANDROMEDA_PULSE_EXIT_WITNESS_LIB=… cargo xtask
  harness:engine-cycle` — green · exit 0 · `"verdict": "pass"` · `"witness_file": "absent"` ·
  `"error_records": 0` · `engine-log: PASS` · `cleanup: clean`; artifact fresh
- `cargo xtask check:english-sources` — green · `"verdict": "clean"`
- `cargo xtask capability-widening-check` — green · exit 0
- `cargo xtask check:ingest-progress` — green · exit 0 (it printed NEUTRAL, no `buffer.tick` in the default
  log family; the entry asserts the exit alone)
- `cargo xtask check:staged-artifacts` — green · exit 0
- `cargo xtask capability-drift` — green · exit 0
- `cargo xtask verify:capability-matrix` — green · exit 0
- `cargo nextest run --workspace --profile ci --no-tests=fail` — green · exit 0 (3064 passed, 0 skipped)
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` —
  green · exit 0
- `git diff --quiet 8394da4f… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green · exit 0 ·
  `"verdict": "green"` · `"reason": "all-stages-ok"`, on `b32e979f` (tree `0af99383…`); red once at
  implement through its clippy stage, the same cause as above
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`) —
  `hygiene: clean`, fired as written before the pre-CI commit; recorded in `evidence/operator-pass.md`
- `m="$(git ls-remote origin refs/heads/main | cut -f1)" && … git merge-base --is-ancestor "$m" HEAD`
  (`operator`) — green · exit 0 · `history: unmoved`; remote `main` `178ebac5…` contained
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0`
  (`operator`) — green · exit 0 · `history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0
  8394da4f→b32e979f`, the intended move
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700`
  (`operator`) — `verdict: green` · checks 7/7 · `ci#38065768197` on `b32e979f`
- `d="target/boot-smoke/ci-<id>-attempt-1" && gh run download <id> … -n logs-boot-Linux …` (`operator`,
  id 38065768197) — green · exit 0 · every atom held (`all-settled`, `"settled": 7`, `"verdict": "settled"`,
  `"program":"window"`, no `"kind":"end"`, no `"verdict": "ended"`); eight witness files read whole, each one
  `loaded` line
- `d="target/engine-cycle/ci-<id>-attempt-1" && gh run download <id> … -n logs-engine-Linux … cargo xtask
  check:engine-log` (`operator`, id 38065768197) — green · exit 0 (the entry asserts the exit alone; its
  lines, ending `engine-log: PASS`, are in `evidence/operator-pass.md`)

No entry carries `defer`; none was skipped or voided; no `leg = 'live'` or `leg = 'round'` entry exists in
the block. The smoke: the window series and the engine cycle ran as gates, and one hand-driven smoke was
added (Deviations 8).

**Watches:** none — the plan folds no `watch:` bullet (`grep -c -i 'watch:' plan.md`: 0).

**Outcome basis:** the operator pass ran (one commit, `b32e979f`, pre-CI; no fix commit followed). The gate
verdicts above rest on its final state: `evidence/operator-pass.md` and the final HEAD's run
`ci#38065768197`. Implement's P4 report, given in this session's conversation, stays the basis for what only
it holds (the deviations, the first red and its fix, the hand smoke, the census). Between implement and this
report: the operator's word inputs#I3 (it accepted the deviations and ordered the pass; it changed no file)
and the wrap directive inputs#I4. Post-implement artifacts: `evidence/operator-pass.md` (untracked until this
wrap's commit), `target/boot-smoke/ci-38065768197-attempt-1/`, `target/engine-cycle/ci-38065768197-attempt-1/`.
The implement conversation is present in this window, so the evolve records were not read as a basis.

**Process hygiene** (implement P4's census, re-measured at this wrap with `ps -eo pid,comm,args`,
2026-10-10T16:49Z: no `pulse-app`, `andromeda-pulse-engine`, `inject_demo` or `Xvfb` process):

| Process | Started by | Final state |
|---|---|---|
| `andromeda-pulse-engine` ×2 (the cycle entry, two whole firings) | implement, through the gate tool | terminated (`cleanup: clean`; exit record `signal 15 (TERM)`) |
| `inject_demo` ×2 | the cycle verb | terminated by its child handle |
| `pulse-app` ×4 and `Xvfb` ×4 (the window smoke, two whole firings) | implement, through the gate tool | terminated (`cleanup: clean` ×4) |
| `andromeda-pulse-engine` ×1 (the hand-driven smoke) | implement, by hand | terminated (`cleanup: clean`) |
| the code-graph refresh | this wrap's Setup, in the background | finished or finishing; read at P4 |

The operator pass started no long-lived process: its two artifact entries download and grade files. Ports
4317 and 4318 were never bound by this chunk's runs.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 8394da4f (the parent of the oldest pre-CI commit b32e979f) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 19 line(s) in 2 range(s)
added: 366-374 · 384-393
### crates/ingest/examples/inject_demo.rs — added 60 line(s) in 6 range(s)
added: 42-49 · 299-315 · 330-338 · 341 · 345 · 580-603
- 302-314 @305 «fn resolve_port(raw: Option<&str>) -> Result<u16, String> {»
  - 306-313 «match raw {»
  - 331-337 «let port = match resolve_port(raw_port.as_deref()) {»
  - 581-584 @582 «fn the_port_is_4317_when_the_variable_is_unset() {»
  - 586-591 @587 «fn the_port_follows_the_registered_variable() {»
  - 593-603 @594 «fn a_set_value_that_is_no_port_is_refused_never_defaulted() {»
### pulse-app/tests/quality_gate_workflow.rs — added 108 line(s) in 2 range(s)
added: 8-12 · 691-793
- 695-718 @696 «fn ci_workflow_boot_job_runs_the_console_engine_cycle_after_ci_gates_whatever_they_returned() {»
  - 700-703 «assert!(»
  - 704-709 «assert!(»
  - 710-712 «let run = format!(»
  - 713-717 «assert!(»
- 720-740 @723 «fn ci_workflow_console_engine_cycle_runs_on_a_data_dir_of_its_own() {»
  - 726-733 «assert!(»
  - 735-739 «assert!(»
- 742-753 @743 «fn ci_workflow_console_engine_cycle_carries_no_soft_fail() {»
  - 745-752 «for banned in ["continue-on-error", "retry", "|| true"] {»
- 755-792 @756 «fn ci_workflow_uploads_the_engine_log_family_under_its_own_name() {»
  - 760-763 «assert!(»
  - 764-776 «for wanted in [»
  - 777-782 «assert!(»
  - 784-791 «assert_eq!(»
### scripts/agent-run.sh — added 74 line(s) in 16 range(s)
added: 8-10 · 12-13 · 38-65 · 70 · 75-76 · 78-94 · 100-101 · 116 · 120 · 129-132 · 136-138 · 169-172 · 177 · 218-220
       224 · 341
- 38-49 «usage() {»
- 56-64 «case "${1:-}" in»
  - 57-63 «boot|status)»
### xtask/src/engine_cycle.rs — new file · 650 line(s)
- 53-57 «pub(crate) struct Options {»
- 59-67 @61 «struct Exits {»
- 69-75 @70 «struct Outcome {»
- 77-85 «impl Outcome {»
  - 78-84 «fn not_run() -> Self {»
- 87-102 «pub(crate) fn run(options: &Options) -> Result<ExitCode> {»
  - 89-95 «let outcome = match drive(&root, options) {»
  - 97-100 «println!(»
- 104-118 @105 «fn refusal(linux: bool, grpc_port: u16, http_port: u16, holds_log: bool) -> Option<&'static str> {»
  - 106-110 «if !linux {»
  - 111-117 «{»
- 120-122 «fn holds_log_family(data_dir: &Path) -> bool {»
- 124-155 @127 «fn child_env(»
  - 135-146 «let fixed: [(&str, OsString); 7] = [»
  - 147-150 «let mut env: Vec<Var> = fixed»
  - 151-153 «if let Some(lib) = witness_lib {»
- 157-179 @159 «fn decide(outcome: &Outcome) -> &'static str {»
  - 160-166 «let exits = [»
  - 167-169 «let failed = exits.iter().flatten().any(|code| !matches!(code, 0 | 2))»
  - 170-171 «let unevaluated =»
  - 172-178 «if failed {»
- 181-187 «fn exit_status(verdict: &str) -> u8 {»
  - 182-186 «match verdict {»
- 189-200 «fn payload(verdict: &str, outcome: &Outcome) -> Value {»
  - 190-199 «json!({»
- 202-204 «fn default_data_dir_name(now: chrono::DateTime<chrono::Utc>) -> String {»
- 206-215 @207 «fn run_passphrase() -> String {»
  - 209-213 «let word = || {»
- 217-301 «fn drive(root: &Path, options: &Options) -> Result<Outcome, &'static str> {»
  - 218-224 «let data_dir = match &options.data_dir {»
  - 225-230 «let refused = refusal(»
  - 231-233 «if let Some(reason) = refused {»
  - 234-236 «let home = std::env::var_os("HOME")»
  - 241-248 «let env = child_env(»
  - 251-256 @253 «if run(»
  - 257-259 «{»
  - 262-276 «let (settled, status) = if boot == 0 {»
  - 281-285 «let error_records = read_family(&logs)»
  - 286-300 «Ok(Outcome {»
- 303-314 @305 «fn command(env: &[Var], dir: &Path, program: &str, args: &[&str]) -> Command {»
  - 307-312 «cmd.args(args)»
- 316-323 @318 «fn exit_of(cmd: &mut Command) -> i32 {»
  - 319-322 «cmd.status()»
- 325-333 «fn spawn_injector(env: &[Var], root: &Path) -> Option<Child> {»
  - 326-331 «let mut cmd = command(»
- 335-340 «fn workspace_root() -> PathBuf {»
  - 336-339 «Path::new(env!("CARGO_MANIFEST_DIR"))»
- 342-650 @343 «mod tests {»
  - 347-353 «const SESSION_VARIABLES: [&str; 5] = [»
  - 355-361 @356 «fn a_host_that_is_not_linux_is_refused() {»
  - 363-378 @364 «fn a_shared_port_in_either_place_is_refused() {»
  - 380-386 @381 «fn a_data_dir_that_already_holds_a_log_family_is_refused() {»
  - 388-397 @389 «fn the_default_ports_on_linux_over_a_fresh_data_dir_are_not_refused() {»
  - 399-413 @400 «fn a_log_family_is_read_under_the_data_dir_s_logs_alone() {»
  - 415-426 «fn env_of(witness_lib: Option<&OsStr>) -> BTreeMap<OsString, OsString> {»
  - 428-441 «fn fixed_set() -> BTreeMap<OsString, OsString> {»
  - 443-450 @444 «fn the_child_environment_is_exactly_the_fixed_set() {»
  - 452-464 @453 «fn the_witness_variable_is_passed_on_only_when_the_verb_s_own_environment_holds_it() {»
  - 466-484 @468 «fn a_child_sees_the_constructed_set_and_nothing_of_this_process() {»
  - 486-498 «fn ran(exits: [Option<i32>; 5], error_records: usize, witness_file: &'static str) -> Outcome {»
  - 502-505 @503 «fn five_zero_exits_no_error_record_and_no_witness_file_pass() {»
  - 507-516 @508 «fn any_verb_that_exits_one_fails() {»
  - 518-521 @519 «fn an_error_record_fails() {»
  - 523-526 @524 «fn a_witness_file_fails() {»
  - 528-532 @529 «fn a_failed_boot_fails_with_the_verbs_between_skipped() {»
  - 534-556 @535 «fn a_verb_that_could_not_evaluate_is_cannot_evaluate_unless_another_failed() {»
  - 558-563 @559 «fn a_cycle_that_did_not_run_cannot_be_evaluated() {»
  - 565-570 @566 «fn the_exit_follows_the_verdict() {»
  - 572-602 @573 «fn the_verdict_carries_the_closed_member_set() {»
  - 604-618 @605 «fn a_cycle_that_did_not_run_reports_nulls_and_no_witness_file() {»
  - 620-635 @621 «fn the_verdict_text_is_what_the_gate_entry_reads() {»
  - 637-641 @638 «fn the_default_data_dir_is_named_by_the_utc_second() {»
  - 643-649 @644 «fn the_passphrase_is_made_for_each_run() {»
### xtask/src/engine_log.rs — new file · 1301 line(s)
- 28-31 «use crate::harness_status::{»
- 56-61 @57 «pub(crate) enum State {»
- 63-79 «impl State {»
  - 64-70 «fn word(self) -> &'static str {»
  - 72-78 «pub(crate) fn exit_code(self) -> u8 {»
- 81-93 @83 «fn worst(states: impl IntoIterator<Item = State>) -> State {»
  - 85-91 «for state in states {»
- 95-100 @96 «pub(crate) struct ArmReading {»
- 102-119 «impl ArmReading {»
  - 103-109 «fn new(arm: &'static str, state: State, detail: impl Into<String>) -> Self {»
  - 111-118 «fn line(&self) -> String {»
- 121-124 @122 «pub(crate) struct Report {»
- 126-143 «impl Report {»
  - 127-129 «pub(crate) fn overall(&self) -> State {»
  - 131-142 @132 «pub(crate) fn lines(&self) -> Vec<String> {»
- 145-166 @147 «pub(crate) fn evaluate(log_dir: &Path, ended: Option<&str>) -> Report {»
  - 149-153 «if family == ci_gates::Verdict::CannotEvaluate {»
  - 155-165 «Report {»
- 168-170 «fn target_of(record: &Value) -> Option<&str> {»
- 172-177 «fn timestamp_ms(record: &Value) -> Option<i64> {»
  - 174-176 «chrono::DateTime::parse_from_rfc3339(raw)»
- 179-197 «fn family_arm(family: &ci_gates::Verdict) -> ArmReading {»
  - 181-196 «match family {»
- 199-228 @201 «fn program_arm(lines: &[Value]) -> ArmReading {»
  - 204-227 «match programs.as_slice() {»
- 230-246 @231 «fn panic_arm(family: &ci_gates::Verdict) -> ArmReading {»
  - 233-245 «match family {»
- 248-283 «fn tick_gap(lines: &[Value], target: &str) -> (State, String) {»
  - 249-253 «let stamps: Vec<Option<i64>> = lines»
  - 255-260 «if n < 2 {»
  - 261-266 «let Some(stamps) = stamps.into_iter().collect::<Option<Vec<i64>>>() else {»
  - 267-271 «let largest = stamps»
  - 272-282 «if largest > MAX_TICK_GAP_MS {»
- 285-293 «fn heartbeat_arm(lines: &[Value]) -> ArmReading {»
  - 286-289 «let readings: Vec<(State, String)> = ENGINE_TICKS»
- 295-343 @298 «fn progress_arm(lines: &[Value]) -> ArmReading {»
  - 300-342 «match ingest_progress::evaluate(lines) {»
- 345-379 «fn process_end_arm(lines: &[Value], ended: Option<&str>) -> ArmReading {»
  - 347-352 «let exits: Vec<usize> = lines»
  - 353-378 «match exits.as_slice() {»
- 381-413 @383 «fn budget_arm(lines: &[Value]) -> ArmReading {»
  - 387-412 «match result.state {»
- 415-434 @416 «pub(crate) fn run_check() -> ExitCode {»
  - 417-429 «let report = match resolve_paths() {»
  - 430-432 «for line in report.lines() {»
- 436-444 @438 «struct SettleEvidence {»
- 446-456 «impl Default for SettleEvidence {»
  - 447-455 «fn default() -> Self {»
- 458-467 «impl SettleEvidence {»
  - 459-466 @460 «fn gradeable(&self) -> bool {»
- 469-495 «fn settle_evidence(lines: &[String]) -> SettleEvidence {»
  - 471-493 «for record in lines»
- 497-501 «fn read_settle_evidence(log_base: &Path) -> SettleEvidence {»
  - 498-500 «crate::smoke::read_jsonl_lines(log_base)»
- 503-505 «fn timeout_supports_verdict(timeout_seconds: u64) -> bool {»
- 507-525 @509 «fn decide_engine_settled(»
  - 515-524 «match (pid, alive) {»
- 527-543 «fn wait_for_settle(»
  - 533-542 «loop {»
- 545-551 «fn settle_exit(verdict: &str) -> u8 {»
  - 546-550 «match verdict {»
- 553-563 «fn settled_payload(verdict: &str, pid: Option<u32>, evidence: &SettleEvidence) -> Value {»
  - 554-562 «json!({»
- 565-578 @566 «pub(crate) fn run_settled(timeout_seconds: u64) -> Result<ExitCode> {»
  - 567-572 «let (verdict, pid, evidence) = match resolve_paths() {»
  - 573-576 «println!(»
- 580-1301 @581 «mod tests {»
  - 591-597 «fn record(at_ms: i64, level: &str, target: &str, fields: Value) -> String {»
  - 599-606 «fn boot(program: &str) -> String {»
  - 608-610 «fn tick(at_ms: i64, target: &str) -> String {»
  - 612-619 «fn buffer_tick(at_ms: i64, rows: u64, delta: u64) -> String {»
  - 621-623 «fn memory(at_ms: i64, value: Value) -> String {»
  - 625-632 «fn exit_record(at_ms: i64) -> String {»
  - 634-641 «fn panic_record(at_ms: i64, level: &str) -> String {»
  - 643-650 «fn stalled(at_ms: i64, reason: &str) -> String {»
  - 652-667 @654 «fn complete() -> Vec<String> {»
  - 669-677 «fn without(lines: &[String], needle: &str) -> Vec<String> {»
  - 679-686 @680 «fn with_before_end(extra: &[String]) -> Vec<String> {»
  - 688-694 «fn family(lines: &[String]) -> tempfile::TempDir {»
  - 696-698 «fn graded(lines: &[String]) -> Report {»
  - 700-706 «fn reading<'a>(report: &'a Report, arm: &str) -> &'a ArmReading {»
  - 708-710 «fn state(report: &Report, arm: &str) -> State {»
  - 712-738 @713 «fn a_complete_console_log_passes_every_arm_in_order() {»
  - 740-752 @741 «fn an_unrelated_file_beside_the_family_changes_nothing() {»
  - 754-768 «fn assert_family_cannot_be_evaluated(report: &Report) {»
  - 770-774 @771 «fn an_absent_log_dir_cannot_be_evaluated() {»
  - 776-781 @777 «fn a_log_dir_with_no_family_member_cannot_be_evaluated() {»
  - 783-791 @784 «fn members_that_hold_no_record_fail() {»
  - 793-800 @794 «fn a_boot_record_that_reads_window_fails() {»
  - 802-807 @803 «fn a_log_with_no_boot_record_cannot_be_evaluated() {»
  - 809-814 @810 «fn two_boots_in_one_family_cannot_be_evaluated() {»
  - 816-821 @817 «fn a_boot_record_naming_no_known_program_cannot_be_evaluated() {»
  - 823-833 @824 «fn a_panic_record_at_error_fails_by_file_name_and_line() {»
  - 835-839 @836 «fn a_panic_target_record_below_error_passes() {»
  - 841-868 @842 «fn a_gap_over_45_seconds_between_two_ticks_of_one_target_fails_with_no_error_record() {»
  - 870-878 @871 «fn a_gap_of_exactly_45_seconds_passes() {»
  - 880-897 @881 «fn fewer_than_two_ticks_of_a_target_cannot_be_evaluated() {»
  - 899-907 @900 «fn a_tick_with_no_readable_timestamp_cannot_be_evaluated() {»
  - 909-924 @910 «fn viz_and_plugins_ticks_are_never_graded() {»
  - 926-934 @927 «fn a_stall_announcement_fails() {»
  - 936-940 @937 «fn a_recovery_announcement_passes() {»
  - 942-950 @943 «fn a_run_in_which_no_span_landed_fails() {»
  - 952-957 @953 «fn a_log_with_no_buffer_tick_cannot_be_evaluated_for_progress() {»
  - 959-971 @960 «fn buffer_ticks_with_no_numeric_row_count_cannot_be_evaluated_for_progress() {»
  - 973-981 @974 «fn a_run_ended_by_sigkill_with_no_exit_record_is_unloggable_end() {»
  - 983-992 @984 «fn a_run_with_no_exit_record_otherwise_is_end_not_recorded() {»
  - 994-999 @995 «fn two_exit_records_fail() {»
  - 1001-1008 @1002 «fn a_record_after_the_exit_record_fails() {»
  - 1010-1014 @1011 «fn a_kill_record_does_not_excuse_a_log_that_holds_its_end() {»
  - 1016-1023 @1017 «fn a_memory_sample_over_budget_fails() {»
  - 1025-1030 @1026 «fn a_memory_sample_at_the_budget_passes() {»
  - 1032-1037 @1033 «fn a_log_with_no_memory_sample_fails_the_budget() {»
  - 1039-1044 @1040 «fn only_zero_memory_samples_fail_the_budget() {»
  - 1046-1051 @1047 «fn a_non_numeric_memory_value_fails_the_budget() {»
  - 1053-1071 @1054 «fn one_line_says_frame_and_snapshot_are_not_graded() {»
  - 1073-1080 «fn report_of(states: &[State]) -> Report {»
  - 1082-1106 @1083 «fn a_fail_outranks_a_cannot_evaluate_and_the_exit_follows_the_verdict() {»
  - 1108-1126 @1109 «fn no_line_holds_a_record_s_text_or_a_path() {»
  - 1128-1136 «fn evidence(program: &'static str, ticks: [usize; 3], populated: usize) -> SettleEvidence {»
  - 1138-1140 «fn gradeable() -> SettleEvidence {»
  - 1142-1149 @1143 «fn a_settle_window_shorter_than_a_tick_interval_and_its_margin_is_refused() {»
  - 1151-1166 @1152 «fn a_live_console_run_with_a_gradeable_log_is_settled() {»
  - 1168-1178 @1169 «fn a_dead_pid_is_ended_whatever_the_log_holds() {»
  - 1180-1191 @1181 «fn a_log_whose_boot_record_reads_window_is_wrong_program() {»
  - 1193-1213 @1194 «fn a_log_short_of_a_reading_polls_until_the_timeout() {»
  - 1215-1225 @1216 «fn an_absent_or_unprobeable_pid_cannot_be_evaluated() {»
  - 1227-1240 @1228 «fn settle_evidence_counts_the_ticks_and_the_populated_memory_samples() {»
  - 1242-1254 @1243 «fn settle_evidence_reads_the_family_and_no_file_beside_it() {»
  - 1256-1263 @1257 «fn the_settle_exit_is_zero_for_settled_alone() {»
  - 1265-1291 @1266 «fn the_settle_payload_carries_the_closed_member_set() {»
  - 1293-1300 @1294 «fn the_console_label_is_the_one_the_settle_read_waits_for() {»
### xtask/src/harness_ready.rs — added 107 line(s) in 10 range(s)
added: 13-15 · 29-30 · 50 · 58-66 · 74-80 · 82 · 89-105 · 159 · 459-523 · 529
- 89-104 «fn ready_payload(»
  - 96-103 «json!({»
  - 459-466 @460 «fn a_status_of_the_wrong_program_is_wrong_program_whatever_the_receivers_say() {»
  - 468-474 @469 «fn the_asked_program_matching_the_log_is_ready_as_before() {»
  - 476-488 @477 «fn a_dead_pid_is_ended_whatever_program_was_asked() {»
  - 490-522 @491 «fn the_ready_payload_carries_the_closed_member_set() {»
### xtask/src/harness_status.rs — added 279 line(s) in 10 range(s)
added: 13-22 · 29 · 36-57 · 66-67 · 72-73 · 77 · 87 · 91 · 101-168 · 471-641
- 39-44 @41 «pub(crate) enum Program {»
- 46-56 «impl Program {»
  - 49-55 @50 «pub(crate) fn label(self) -> &'static str {»
- 101-111 «fn status_payload(verdict: &Verdict, ended: Option<String>, program: &str) -> Value {»
  - 102-110 «json!({»
- 113-130 @115 «pub(crate) fn boot_record_program(record: &Value) -> Option<&'static str> {»
  - 116-118 «if record.get("target").and_then(Value::as_str) != Some(BOOT_ENGINE_TARGET) {»
  - 119-122 «let label = record»
  - 123-129 «Some(»
- 132-141 @134 «pub(crate) fn program_of(lines: &[String]) -> &'static str {»
  - 135-140 «lines»
- 143-149 @145 «pub(crate) fn read_program(log_base: &Path) -> &'static str {»
  - 146-148 «crate::smoke::read_jsonl_lines(log_base)»
- 151-160 @154 «pub(crate) fn graded_arm(arm: &'static str, asked: Option<Program>, program: &str) -> &'static str {»
  - 155-159 «match (arm, asked) {»
- 162-167 «pub(crate) fn for_program(verdict: Verdict, asked: Option<Program>, program: &str) -> Verdict {»
  - 163-166 «Verdict {»
  - 471-479 «fn boot_record(program: &str) -> String {»
  - 481-500 @482 «fn the_program_is_the_last_boot_record_of_the_family() {»
  - 502-518 @503 «fn a_family_with_no_boot_record_reads_unknown() {»
  - 520-532 @521 «fn a_boot_record_naming_no_known_program_reads_unknown() {»
  - 534-551 @535 «fn a_file_beside_the_family_is_not_read_for_the_program() {»
  - 553-570 @554 «fn an_asked_program_that_the_log_does_not_record_is_wrong_program() {»
  - 572-582 @573 «fn the_asked_program_matching_the_log_keeps_the_arm() {»
  - 584-592 @585 «fn with_no_program_asked_the_arm_stands_whatever_the_log_records() {»
  - 594-600 @595 «fn a_run_that_is_not_there_is_not_graded_for_its_program() {»
  - 602-609 @603 «fn for_program_changes_the_arm_alone() {»
  - 611-634 @612 «fn the_status_payload_carries_the_closed_member_set() {»
  - 636-640 @637 «fn the_program_labels_are_the_boot_record_s_own() {»
### xtask/src/main.rs — added 53 line(s) in 5 range(s)
added: 13-14 · 57-64 · 67 · 69-97 · 300-312
  - 61-64 «HarnessStatus {»
  - 69-72 «HarnessReady {»
  - 77-80 «HarnessEngineSettled {»
  - 85-92 «HarnessEngineCycle {»
### xtask/src/pre_push.rs — added 2 line(s) in 1 range(s)
added: 30-31
