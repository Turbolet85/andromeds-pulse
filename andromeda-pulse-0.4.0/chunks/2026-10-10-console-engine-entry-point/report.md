# Report — 2026-10-10-console-engine-entry-point

**Chunk:** one console program boots ingest, buffer, detectors, corpus with no display, driven by commands; its log, identity, panic, heartbeat, process-end records kept
**Date:** 2026-10-10
**Commits:** `4f519c94` chore(2026-10-10-console-engine-entry-point): operator pre-CI commit (the one commit since the last wrap; basis `git log --format='%h %s' f31027a4..HEAD`)

## Changes (structured — detectors read this)
- **Files:** 15, all under `pulse-app/` (basis: `gate.py scope` — changed 15 · listed 15, base `f31027a4`).
  Modified 6: `pulse-app/Cargo.toml`, `pulse-app/src/main.rs`, `pulse-app/src/lib.rs`, `pulse-app/src/heartbeat.rs`,
  `pulse-app/src/observability.rs`, `pulse-app/tests/unit_heartbeat_ticks.rs`. New 9:
  `pulse-app/src/engine_boot.rs`, `pulse-app/src/console.rs`, `pulse-app/src/bin/andromeda-pulse-engine.rs`,
  `pulse-app/tests/unit_engine_boot_env.rs`, `pulse-app/tests/unit_console_commands.rs`,
  `pulse-app/tests/unit_observability_allowlist_boot_engine.rs`, `pulse-app/tests/unit_engine_boot_seam.rs`,
  `pulse-app/tests/integration_engine_boot.rs`, `pulse-app/tests/integration_console_engine.rs`.
  No file of `crates/`, `xtask/`, `scripts/`, `.github/`, `docs/`, `pulse-app/ui/`, `pulse-app/capabilities/`
  changed, and `Cargo.lock` is unchanged (basis: the plan's scope-guard entry, exit 0, no output, against `f31027a4`).
- **Symbols / APIs:**
  - **A second product binary, `andromeda-pulse-engine`** — a second `[[bin]]` of the `pulse-app` package
    (`pulse-app/Cargo.toml:19-23`, path `src/bin/andromeda-pulse-engine.rs`), with `default-run = "pulse-app"`
    (`pulse-app/Cargo.toml:8`). Its `main` (`pulse-app/src/bin/andromeda-pulse-engine.rs:1-3`) hands its arguments to
    `pulse_app::console::main`. The product binaries are now three: `pulse-app` (the window app),
    `andromeda-pulse-engine` (the console program), `andromeda-pulse-mcp` (the stdio sidecar, unchanged).
  - **The console program's command grammar** (`pulse-app/src/console.rs`): closed, two words. `run` boots the engine
    and stays up until signalled; `version` prints one line (`andromeda-pulse-engine {version}`) and exits 0; no word,
    an unknown word, a second word or a word that is not Unicode prints
    `usage: andromeda-pulse-engine <run|version>` on stderr and exits 2, creating nothing. Public items:
    `Command` (`console.rs:19-23`), `parse` (`:25-42`), `version_line` (`:44-46`), `main` (`:48-65`), the private
    `run` (`:67-97`), consts `PROGRAM_NAME`, `USAGE`, `EXIT_USAGE` (2). No `stop` command exists.
  - **The one engine boot both programs call** (`pulse-app/src/engine_boot.rs`, new, 1253 lines):
    - `init_process(data_dir)` (`engine_boot.rs:311-321`): the log sink and its guard, the at-exit hook, the Unix
      signal listener, the start instant, the pid file, in that order. Called with the tokio runtime entered.
    - `start(config, inputs) -> EngineHandles` (`engine_boot.rs:462-1253`): builds the engine state and spawns every
      engine task with `tokio::spawn` (never through the window framework): the buffer consumer and retention, the two
      receivers, the connection poller, the baseline persist loop, the cue emitter, the restart and storm detector
      ticks, the cadence coordinator, the digest assembler's cadence subscriber and persister, the lifecycle
      heartbeat, the lifecycle / storm / incident persist loops, the auto-resolution loop, the config watcher and its
      fan-out, and the engine's three heartbeat ticks. It emits one boot record (below).
    - Inputs: `EngineConfig { data_dir, grpc_port: Option<OtlpPort>, http_port: Option<OtlpPort>, retention_seconds }`
      (`:356-363`) with `EngineConfig::from_env(data_dir)` (`:366-375`) reading the two port variables and the
      retention variable through the moved resolvers; `EngineInputs { program, key_backend, hardware_profile,
      interpretation }` (`:378-384`); `Program { Window, Console }` (`:328-333`); `SeatKind { Model, Deterministic }`
      (`:344-349`); `InterpretationSeat { runner, kind }` (`:351-354`); `os_key_backend()` (`:323-326`), the one place
      the credential service id `com.andromeda.pulse` is written for both programs.
    - **The bind address is not configurable**: the config holds two validated PORTS and `start` builds
      `127.0.0.1:{port}` itself (`loopback`, `engine_boot.rs:442-444`); a `None` port (a rejected variable) records the
      bind as failed with `reason = "invalid_port"` and starts no receiver, as `main` did before.
    - `EngineHandles` (`:386-411`): the state the window's routers, tray and two ticks read (heartbeat state, ingest
      state, bind status, buffer connection and state, broadcast senders, retention seconds, corpus reader and writer,
      lifecycle and incident registries and broadcasts, incident persistence, the workspace key, the drain miner, the
      degraded-mode handle, the reevaluator, the config handle slot and status) and, `#[doc(hidden)]`, the digest
      broadcast for tests. Dropping it stops nothing.
    - Eight boot helpers moved out of `main.rs`, behaviour unchanged, `pub` + `#[doc(hidden)]`:
      `resolve_retention_seconds` (`:101-129`), `resolve_grpc_port` (`:131-134`), `resolve_http_port` (`:136-139`),
      `resolve_port` (`:141-173`), `resolve_data_dir` (`:175-197`), `write_pid_file` (`:199-241`),
      `publish_workspace_key_for_sidecar` (`:243-268`), `init_buffer` (`:270-309`), with the consts
      `ENV_OTLP_GRPC_PORT`, `ENV_OTLP_HTTP_PORT`, `ENV_RETENTION_SECONDS`, `RETENTION_SECONDS_MIN` / `_MAX` /
      `_DEFAULT`. `resolve_mcp_sidecar_binary_path` and the test `emit_taurpc_bindings` stay in `main.rs`.
  - **The interpretation seat.** `start` spawns the interpretation subscriber and the two L4 heartbeats only behind a
    seated runner; with `interpretation: None` it spawns neither, and the digest assembler's cadence subscriber and
    persister still run — cues and digests are produced, no incident forms. The window app seats its runner exactly as
    before (`main.rs`, the `interpretation.model.load` record unchanged): the canned runner when
    `ANDROMEDA_PULSE_L4_DETERMINISTIC` is truthy, else the llama-cli runner. The console program seats the canned
    runner (tier for the unknown hardware profile) when the variable is truthy and NOTHING when it is unset
    (`console.rs:78-83`).
  - **The boot record `app.boot.engine`** — emitted exactly once per `start` by `emit_boot_record`
    (`engine_boot.rs:413-440`), fields `program` (`window` | `console`), `interpretation` (`model` | `deterministic` |
    `none`), `reason` (`window_runs_model` | `deterministic_gate_set` | `deterministic_gate_unset`); WARN when
    `interpretation` is `none` (its message says no incident forms until the engine keeps its own incident record and
    names the variable), INFO otherwise. Its own exact allowlist leaf beside `app.boot.render.posture`
    (`pulse-app/src/observability.rs:1014-1021`); no bare `app` key exists.
  - **`heartbeat::spawn` no longer exists.** It is split into `heartbeat::spawn_engine_ticks` (three handles:
    `ingest.tick`, `buffer.tick`, `connection.tick`; its new doc lines at `pulse-app/src/heartbeat.rs:84-85` and its
    head at `:87`, its one production caller `engine_boot::start` at `engine_boot.rs:1219-1228`) and
    `heartbeat::spawn_window_ticks` (two handles: `viz.tick`, `plugins.tick`; `heartbeat.rs:120-131`, its one
    production caller `main`'s setup closure at `main.rs:379-382`). The window app still emits all five ticks; the
    console program emits three and never `viz.tick` or `plugins.tick`. Callers of each: one production site and one
    test in `pulse-app/tests/unit_heartbeat_ticks.rs` (basis: `grep -rn 'spawn_engine_ticks\|spawn_window_ticks'
    pulse-app --include=*.rs` — 10 lines: 2 definitions, 2 production calls, 2 test calls, 1 import, 2 test names,
    1 entry of the seam pin's list).
  - **`main.rs`** (2054 → 649 lines, basis `wc -l`): its first two statements and its own runtime build are unchanged
    and still pinned by `unit_xlib_threads`; it then calls `engine_boot::init_process`, its own
    `window::emit_boot_spans()` and `render_posture::emit_posture(..)`, selects its runner, calls
    `engine_boot::start` with program `window`, and builds routers, plugin host, `VizState`, window geometry and the
    Tauri builder from the returned handles. Its setup closure keeps only window work and the window's two ticks.
  - **Order changes in the window app** (behavioural, in the log's record order): the engine's tasks now start before
    the Tauri builder instead of inside its setup closure; the model selection (`interpretation.hardware.*`,
    `interpretation.model.*` records) now precedes the engine state (corpus, buffer); the engine reads `Settings` once
    (the cadence config comes from the same boot read as the drain knobs) and `main` reads it once more for the
    widget. The records up to the engine boot keep their order (`app.boot.tracing.init`, `app.boot.pid`, the three
    window boot records, the render posture).
  - **Ports / sockets:** none new. The console program binds the same two loopback receivers at the ports the two
    registered variables name, and nothing else. Both programs default to 4317 / 4318 and to the same data dir.
  - **Environment variables:** none new. `ANDROMEDA_PULSE_L4_DETERMINISTIC` has a second reader, the console
    program, where unset means "seat nothing". Through the shared code the console program also reads
    `ANDROMEDA_PULSE_DATA_DIR` (and `XDG_CONFIG_HOME` / `HOME` for its default), the two OTLP port variables, the
    retention variable, `ANDROMEDA_PULSE_LOG_LEVEL` / `RUST_LOG`, `ANDROMEDA_PULSE_CORPUS_PASSPHRASE`, the system
    `XDG_RUNTIME_DIR` (the corpus-key lock dir) and the variables `Thresholds::from_env` reads. It reads no model
    path variable, no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, no `ANDROMEDA_PULSE_HARDWARE_PROFILE`, no
    `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` (pinned by source text in `unit_engine_boot_seam`), and it constructs no
    `LlamaCliInference` and no `HardwareProfileDetector`; the plugin host and its directory variable stay in `main`.
  - **Files the console program writes** (measured on a hand run and asserted in `integration_console_engine`): under
    its data dir only `logs/agent-latest.jsonl.{date}`, `run/andromeda-pulse.pid`, `run/workspace-key` and
    `corpus/corpus.db` (top-level entries a subset of `corpus`, `logs`, `run`); outside it, the registered corpus-key
    lock file. The pid file and the workspace key are now written by BOTH programs, at the same paths.
  - **Process end.** The console program has no event loop: it ends by SIGTERM or SIGINT through the existing
    listener, which records `app.exit` with class `signal` and re-raises the same signal. `exit_after_event_loop`
    stays the window's tail alone; no label was added to the closed `exit_class` / `signal` sets.
  - **No IPC change:** no TauRPC procedure, payload, event or capability file changed; the bindings equal the base
    (the plan's bindings close, exit 0).
- **Crates / modules:** the `pulse_app` library gains the modules `engine_boot` (`pulse-app/src/lib.rs:15`) and
  `console` (`lib.rs:7`). No workspace member is added (sixteen stands); no crate's dependency on the window
  framework is cut — `tauri` stays in `pulse-app`'s manifest. `engine_boot.rs` and `console.rs` name neither `tauri`
  nor `taurpc` (pinned).
- **Dependencies:** none added, none bumped; `Cargo.lock` unchanged.
- **Schema / config:** no schema, no config key. One allowlist leaf added (`app.boot.engine`: `program`,
  `interpretation`, `reason`).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `pulse-app/tests/*.rs` files 103 → 109 (basis: `ls pulse-app/tests/*.rs | wc -l` = 109; `git ls-tree --name-only
    f31027a4 pulse-app/tests/` = 103). Stated at `.claude/rules/testing.md:25` ("103 files as of 2026-10-10");
    `\b103\b` reads 0 lines in `test-plan.md` and 1 in the registries, which is the id `P-103`, not this count.
  - `pulse-app/src/*.rs` 43 → 45, plus one file under `src/bin/` (basis `ls`); 18 of the 45 name `tauri` or `taurpc`
    (basis `grep -l -E 'tauri|taurpc' pulse-app/src/*.rs | wc -l` = 18, unchanged from research's 18 of 43).
  - Product binaries 2 → 3; `[[bin]]` targets of `pulse-app` 1 → 2.
  - Heartbeat spawn: one function returning five handles → two functions returning three and two.
  - Workspace run: `2976 tests run: 2976 passed, 0 skipped` across 135 binaries, read on the dev host and in the
    `lint / test` job of `ci#38056942758`. The count on the base was not measured here.
  - Coverage on `ci#38056942758`: `Line: 34599/38214 = 90.5%`, `Function: 3622/4056 = 89.3%` (the last recorded
    reading was 88.9 % / 87.9 %).
  - The 14 resolver tests moved from `main.rs`'s `mod tests` (the `[[bin]]` test binary) to
    `pulse-app/tests/unit_engine_boot_env.rs` under the same 14 names; `main.rs`'s `mod tests` keeps one test,
    `emit_taurpc_bindings`.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none changed — no script, xtask verb, CI step or verdict shape. Read, not changed:
  the six new test binaries run inside the existing `cargo xtask test` step of `lint / test` and inside the local
  pre-push check's `test` stage, both with no display variable and no session bus, so the console program is run by a
  check on every push with no workflow edit; `scripts/agent-run.sh boot` still builds and spawns `pulse-app` alone;
  the harness verbs that read `run/andromeda-pulse.pid` are not aimed at the console program although it writes that
  file.
- **Cross-project / external claims:**
  - CI: `ci#38056942758` (event `pull_request`, attempt 1) measured the pre-CI commit
    `4f519c94aba16da44948e8edef110d23d602c293` merged with `main` (`178ebac56e26`; the merge is `e3d94ada…`),
    conclusion success, 7 of 7 checks with `secret-scan#38056942764`. This wrap's commit adds to that tree and is not
    covered by the run. Basis: `evidence/operator-pass.md`.
  - `I1 · ../additional/pc-overseer/relays/pulse-phase-console-engine-entry-point-2026-10-10.md · copy · unchanged`
    (cited by scope, research and plan).
  - `I2 · message: the operator (the pc overseer), in the session at P4, 2026-10-10 · copy · n/a — a message has no
    live source` (cited by scope and plan).
  - `I3 · message: the operator, in the implement session after the P4 report, 2026-10-10 · copy · n/a` — the word
    for the operator pass; cited by `evidence/operator-pass.md` and by this bullet (inputs#I3).
  - `I4 · ../additional/pc-overseer/relays/pulse-wrap-console-engine-entry-point-2026-10-10.md · copy · unchanged` —
    the operator's wrap directive, snapped at this wrap and cited by this report (inputs#I4).
  - No entry reads `drifted`, `vanished` or `broken`; no `UNPARSED` row.
- **Reverted / negative API facts:** nothing shipped and reverted. Deliberately not built (plan constraints): a
  `stop` command; a stderr or stdout log layer in the console program; a test-only switch that induces a panic in
  the product binary; a Cargo feature making `tauri` optional; a new workspace crate for the engine. Two mutations
  were applied and reverted during P2 (`evidence/mutation-checks.md`); the tree carries neither.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none — no spec, plan or contract assertion was measured false. The
  sentences this chunk makes stale by building a second program are listed under Expected amendments.
- **Expected amendments (from plan):**
  - architecture §Project Intent → Product type, §Design Philosophy, §Infrastructure Patterns → Deployment model and
    Project directory structure (a second program over the shared engine boot, no longer "not a CLI") — carried: the
    Symbols / APIs bullets "A second product binary", "The console program's command grammar" and "The one engine
    boot". Search: `not a CLI` — architecture 1 line, the six other masters 0, registries 0; `single-process` —
    architecture 2 lines; the two keyed contracts are `.andromeda/registries/contracts/architecture/deployment-model.md`
    and `…/project-directory-structure.md` (`grep -rlE 'Deployment model|Project directory structure'
    .andromeda/registries`).
  - architecture §Occupied Resources → Process / service identity (the binary `andromeda-pulse-engine`), Filesystem
    locations (the pid file and the workspace key written by both programs), Environment variables
    (`ANDROMEDA_PULSE_L4_DETERMINISTIC` read by the console program, with its unset meaning there) — carried: the
    Symbols / APIs bullets "A second product binary", "Files the console program writes", "Environment variables" and
    "The interpretation seat". Search: `andromeda-pulse-engine` — 0 lines in all seven masters and the registries;
    `Process / service identity` — architecture:206; `andromeda-pulse\.pid` — architecture 2 lines, registries 3;
    `workspace-key` — architecture 1, security-plan 3, test-plan 4, obs-plan 2; `ANDROMEDA_PULSE_L4_DETERMINISTIC` —
    architecture 2 lines (:202, :246), security-plan 1 (:138), test-plan 1 (:286), registries 3 (all in
    `contracts/test-plan/per-chunk-gate-discipline.md`).
  - obs-plan §3 Tracing init (the console program's order, with no event loop), §3 Heartbeat ticks (which ticks each
    program emits), §6 and §8 (`app.boot.engine`, both sites) — carried: the Symbols / APIs bullets "The one engine
    boot" (`init_process`), "Process end", "`heartbeat::spawn` no longer exists" and "The boot record". Search:
    `app\.boot\.engine` — 0 lines everywhere; its neighbour `app\.boot\.render\.posture` — obs-plan 2 lines (the §6
    and §8 sites), architecture 1, security-plan 1, test-plan 1; `viz\.tick` — obs-plan 2 lines; `Tracing init` —
    obs-plan:72 and the key file `contracts/obs-plan/tracing-init.md`; `Heartbeat ticks` — obs-plan:101, :536 and the
    key file `contracts/obs-plan/heartbeat-ticks.md`; `install_signal_listener|install_exit_hook|hold_log_guard` —
    obs-plan 1 line, test-plan 1, registries 1.
  - security-plan §Threat Model Summary (the process roster) and §Input Validation (a row for the console program's
    command words; the variable's second reader) — carried: "A second product binary", "The console program's command
    grammar", "Environment variables", "Ports / sockets". Search: `## Threat Model Summary` — security-plan:18;
    `## Input Validation` — security-plan:121, its "CLI / env var inputs" row at :138 naming
    `ANDROMEDA_PULSE_L4_DETERMINISTIC`.
  - test-plan §2 and §4 (the `pulse-app/tests` count, 103 → 109), §3 (the spawned-program witness as a form), §1
    Pending coverage triggers (`exit-hook-main-composition-coverage` and `discovery-observer-wiring-coverage`
    narrowed) — carried: Counts / qualifiers moved (the count and where it is stated) and the Coverage rows below.
    Search: `\b103\b` — test-plan 0 lines (the literal count is stated in the leaf `.claude/rules/testing.md:25`
    alone); `exit-hook-main-composition-coverage` — test-plan 2 lines; `discovery-observer-wiring-coverage` —
    test-plan 3 lines; `pulse-app/tests/\*\.rs` — test-plan:126 and five rows of §1. What narrows the two triggers:
    `init_process` is the function both programs call and its panic, at-exit and signal records are witnessed in
    re-exec children; `start` is the production composition and its three-observer fan-out is asserted in process
    (a service sent over gRPC is listed in the lifecycle registry by the first-sighting adapter, the last of the
    three). Not shown: the composition inside the window's own `main` end to end, which only the boot smoke runs.
- **Coverage of new surfaces:**
  - `andromeda-pulse-engine` command line (`run` | `version`) → validation closed two-word grammar, exit 2 on
    anything else✓ · instrumentation n/a (a refused command line writes no record by design) · PII n/a · tests unit
    (`unit_console_commands`, 9) + integration (spawned binary, exit codes and empty data dir)✓ · a11y n/a · tokens
    n/a
  - `andromeda-pulse-engine run` (the console program's process) → validation: the two port variables and the
    retention variable through the moved resolvers, the bind fixed to loopback✓ · instrumentation: the same log sink,
    identity fields, panic hook, three ticks, `app.exit`✓ · PII: no full data-dir path and no passphrase value in its
    log, asserted on a spawned run✓ · tests integration (`integration_console_engine`, 4)✓ · a11y n/a · tokens n/a
  - `engine_boot::init_process` → validation n/a · instrumentation: `app.boot.tracing.init`, `app.boot.pid`,
    `app.panic.fatal`, `app.exit`✓ · PII: the panic payload never reaches the log (canary asserted)✓ · tests
    integration, two re-exec arms (panic then exit; SIGTERM)✓ · a11y n/a · tokens n/a
  - `engine_boot::start` → validation n/a (typed inputs) · instrumentation: `app.boot.engine` plus every record the
    moved wiring already emitted✓ · PII: three closed labels, no path, no variable value✓ · tests integration, four
    in-process arms (composition; seat `deterministic`; seat none; program `window`)✓ · a11y n/a · tokens n/a
  - log target `app.boot.engine` → validation n/a · instrumentation is the record itself✓ · PII: exact leaf, closed
    labels✓ · tests unit (`unit_observability_allowlist_boot_engine`, 4: exact resolve, set equality both ways, the
    no-bare-`app` discriminator, emit-site capture for the four program and seat pairs; mutation-checked)✓ · a11y n/a
    · tokens n/a
  - `heartbeat::spawn_engine_ticks` / `spawn_window_ticks` → validation n/a · instrumentation: the five existing
    tick targets, unchanged fields✓ · PII n/a · tests unit (three-handle and two-handle tests) + the spawned run's
    tick assertions✓ · a11y n/a · tokens n/a
  - No UI element, no window surface and no rendered region was added or changed.

## Deviations from intent
- **The config holds two validated ports, not two addresses** (plan step 3 says "the two receiver addresses").
  `start` fixes the bind to `127.0.0.1`, so no caller can ask for a non-loopback bind; a test still passes its own
  ports. Accepted by the operator after the implement report: "a config that cannot ask for a non-loopback bind is
  the right form" (inputs#I3).
- **The split heartbeat functions are named `spawn_engine_ticks` and `spawn_window_ticks`**; the plan named none.
- **The seam pin is stricter than step 8.** Its closed list holds 17 parts — the plan's 16 plus
  `spawn_engine_ticks(` — and it adds two known-positive controls (the shared boot names every part; the window entry
  names the model, the probe and its first statements). For the console files it also forbids the three variables the
  plan's constraints name, the allow-root variable, and the window framework's name.
- **One settings load in the engine.** The cadence config comes from the same boot-time `Settings` read as the drain
  knobs (before, `main` read the file twice, at state build and in the setup closure).
- **Small additions:** `engine_boot::os_key_backend()`; with an empty seat the `digest.runtime.boot` INFO message is
  a new string that does not claim an interpretation subscriber.
- **Test arms.** The four in-process arms each boot a different program and seat pair so each boot record is told
  apart in the process-wide capture; the composition arm seats the canned runner labelled `model`. The
  spawned-program test also asserts no `app.boot.render.posture` record, stored span rows equal to spans sent, and
  the data dir's top-level entries within `corpus`, `logs`, `run` — three assertions the plan's step 10 does not list.
- **Gate order at P2.** The chunk's own entries (the first eight) were fired once with `--only` before the whole
  block; the whole block then ran once and is the record.
- **P3 was not driven by hand.** The plan's one smoke entry is the whole window smoke and is not a leg, so the gate
  tool fired it at P2 with its artifact read fresh; it was not run a second time.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 15 · listed 15, at implement P4 and again at this
  wrap's P1 against base `f31027a4`).

## Decisions & corrections
- The operator accepted the implement deviations as recorded and ordered the operator pass (inputs#I3); the wrap
  directive (inputs#I4) says: the pass stands; P-086 is not claimed and takes a dated note; the masters describe two
  programs and no sentence is worded as if the console engine already did what a later entry builds; four carries on
  their owning entries; the drift pin goes on `Window retired`; found-and-not-owned goes to the card.
- **What forbids the two entry points from drifting until the window leaves** (directive item 5): (1) both call the
  same two functions and neither wires an engine part itself — `unit_engine_boot_seam`, eight source-text pins,
  mutation-checked on the step-8 mutation; (2) `integration_engine_boot` runs the shared `start` in process for both
  seat arms and both program labels; (3) two programs are run on every push — the window by the boot job's smoke and
  its seven further boots, the console by `integration_console_engine` in the workspace run. The seam pin's subject
  is "two callers of one boot": when `Window retired` removes `main.rs`, the pin's window half and its
  `unit_xlib_threads` neighbour go with it.
- A hand run of the built console binary at implement backgrounded an and-list in a subshell, so the shell's
  last-background pid named the subshell and the first SIGTERM missed the program; it stayed up about 20 s on two
  scratch ports until its pid was read from its own pid file, checked against `ps`, and signalled. The tests take the
  pid from `Child::id()` and kill a still-running child on drop.
- Sweep hazards met: the seam pin matches call text with the opening parenthesis (`Corpus::open(`), because
  `main.rs`'s bindings test names `Corpus::open_in_memory(`; a CI job log fetched with
  `--allow-escape-sequences` carries colour escapes between `PASS` and the test name, so a plain `PASS .*name` grep
  over it reads 0 until the escapes are stripped; the coverage step prints `Line:` with five spaces before the
  figure.
- Found and not owned by this chunk, for the card:
  - Both programs default to the same data dir, the same two ports and the same pid file path, and nothing refuses
    a second engine on a data dir: started beside a running one, the second records both binds failed, overwrites
    the first's pid file and opens the same corpus. Not exercised by any test.
  - What `tauri build` does with a second `[[bin]]` in the crate is not measured; no step runs the release workflow.
  - The dead-test ratchet reads the top level of `pulse-app/src/` only; `src/bin/` is outside it. A test attribute
    in the bin file would run (a `[[bin]]` test binary), so nothing is dead there; the ratchet's reach is simply one
    directory.
  - In CI the boot record carries `deployment.environment: dev` and the merge sha; read, not changed, not this
    chunk's.

## Outcome
- **Acceptance criteria, each against the diff:**
  - (scope) the console program boots the two receivers, the buffer, the detectors and the corpus with no display
    variable and no session bus, and telemetry over gRPC lands — MET: `integration_console_engine` passes locally, in
    the pre-push `test` stage and in `lint / test` of `ci#38056942758`; five spans sent, five rows appended to
    `spans`, read from the program's own `duckdb.append` records.
  - (scope) driven by commands — MET: `run` stays up until signalled, `version` exits 0 with one line, any other
    command line exits 2 with usage and an empty data dir (`unit_console_commands`, `integration_console_engine`).
  - (obs) one file sink, every line whole JSON with the identity fields, no log record on stderr — MET (asserted on
    the spawned run; stdout checked too).
  - (obs) each end the chunk builds leaves exactly one `app.exit` as the last record, by SIGTERM and by SIGINT, the
    process still ending by that signal — MET (signals 15 and 2 read from the exit status).
  - (obs) the process-start function keeps the panic record — MET: a re-exec child that calls `init_process` and
    panics on a thread leaves one ERROR `app.panic.fatal` and one `app.exit` of class `outside_event_loop`.
  - (obs) two `ingest.tick` and two `buffer.tick` at most 45 s apart, `rows_ingested_delta` rendered, no `viz.tick`
    or `plugins.tick` — MET.
  - (obs) `app.boot.engine` once per boot behind its own exact leaf — MET (guard, in-process arms, spawned run; once
    in each of the eight boot logs of the CI artifact).
  - (operator, inputs#I2) both seat arms pinned; the console program reads the arm from the variable in both
    directions — MET.
  - (inputs#I1 item 5) no model variable, no runner, no probe; no `interpretation.model.*` record — MET.
  - (inputs#I1 item 3) neither entry point wires an engine part; the shared boot names no window framework; the pin
    reddens under the mutation of step 8 — MET (`evidence/mutation-checks.md`).
  - (arch) two listening sockets, both on `127.0.0.1`, at the two variables' ports, both refusing after the end —
    MET for the ports the test set (both accept, both refuse after the end); that no THIRD socket listens is by
    construction (the shared boot binds two) and was not measured from the process's socket list.
  - (arch, security) every file under the data dir at a registered subpath or the registered lock file; no full
    data-dir path, no passphrase value; one `corpus.keychain.fallback` — MET for the data dir (top-level entries
    within `corpus`, `logs`, `run`) and the log; the lock file's placement outside the data dir was not asserted by
    the test.
  - (inputs#I1 item 1) the console binary needs no window library at load — MET on the dev host's debug binaries:
    the window binary reads 6, the console binary 0 after its `libc` control reads 1.
  - (inputs#I1 item 3, tests) the window app still boots on the shared boot — MET: `unit_xlib_threads` green; the
    local series `all-settled`, `"program":"window"`; on `ci#38056942758` the boot job's series `all-settled` with 7
    settled, the smoke `settled`, the `a11y` job success with no step of it edited.
  - (tests) every entry of the fence exits per its `expect`; the bindings close exits 0; the 14 moved tests listed by
    name — MET (below).
  - (tests) CI green on the pushed tip, coverage ≥ 75 % line and ≥ 85 % function — MET: `ci#38056942758`, 90.5 % /
    89.3 %. "With the new code inside the measure" was not read per file: the coverage log prints totals only.
  - (P-086, advanced, not claimed) the first half only is shown: the engine starts as a console program with no
    display, no GPU probe and no desktop session, stays up and answers `run` and `version`. Not shown: "runs in the
    background" as a service, every kind of finding, one incident for a sustained storm, automatic resolution on real
    telemetry. P-086 stays `planned`, unclaimed; a dated ledger note records the half shown (ledger-note — owner
    P7.3, on the operator's directive, inputs#I4 item 2).
- **Gates** (implement's whole-block run, then the operator pass; each named by its `run` text):
  - `cargo fmt --check` — green · exit 0
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
  - `git diff --name-only f31027a4… -- crates pulse-app xtask scripts docs …` (the scope guard) — green · exit 0 ·
    no output
  - `cargo nextest run … -E 'binary(unit_engine_boot_env) + … + binary(unit_xlib_threads)'` — green · exit 0 · 67
    passed, 0 skipped
  - `cargo nextest run … -E 'binary(integration_engine_boot) + binary(integration_console_engine)'` — green · exit 0
    · 10 passed, 0 skipped
  - `cargo build -p pulse-app --bin pulse-app --bin andromeda-pulse-engine` — green · exit 0
  - `f=target/debug/pulse-app && … grep -c -i -E 'gtk|webkit|gdk|soup|javascriptcore'` — green · exit 0 · last line 6
  - `f=target/debug/andromeda-pulse-engine && …` — green · exit 1 · last line 0
  - `d="target/boot-smoke/$(date -u …)-series" && … cargo xtask harness:boot-series --count 2 && grep …` (the window
    smoke) — green · exit 0 · `all-settled`, ordinals 2 and 3, `"program":"window"`, artifact fresh
  - `cargo xtask check:english-sources` — green · `"verdict": "clean"`
  - `cargo xtask capability-widening-check` · `cargo xtask check:ingest-progress` · `cargo xtask
    check:staged-artifacts` · `cargo xtask capability-drift` · `cargo xtask verify:capability-matrix` — green · exit 0
    each
  - `cargo nextest run --workspace --profile ci --no-tests=fail` — green · exit 0 · 2976 passed, 0 skipped
  - `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
  - `git diff --quiet f31027a4… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
  - `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green at implement, and green
    again on the committed tree in the operator pass (`"verdict": "green"`, `"reason": "all-stages-ok"`, head
    `4f519c94`)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (operator) — `hygiene: clean — read 45`
  - `m="$(git ls-remote origin refs/heads/main | cut -f1)" && … git merge-base --is-ancestor "$m" HEAD` (operator)
    — green · exit 0
  - `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` (operator) —
    green · the one ref moved `f31027a4→4f519c94`
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` (operator) —
    `verdict: green · checks 7/7`, `ci#38056942758`
  - `d="target/boot-smoke/ci-<id>-attempt-1" && gh run download <id> …` (operator, id 38056942758) — green ·
    `all-settled`, `"settled": 7`, the smoke `settled`, eight witness files of one `loaded` line each, no `end` line,
    `"program":"window"`
  - No entry carries `defer`; nothing was skipped. Smoke: the window app's two-boot series, as above.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran. The gate verdicts rest on its final state: the one pass commit
  `4f519c94` and its run `ci#38056942758`, recorded in `evidence/operator-pass.md`. Implement's P4 report as given in
  this session's conversation stays the basis for what only it holds (the deviations, the mutation checks, the hand
  run); the operator's word after it changed nothing in the tree (inputs#I3). `evidence/operator-pass.md` was
  written after the push and rides this wrap's commit.
- **Process hygiene** (implement P4's census, re-measured at this wrap's P1 from `ps` and `ss`,
  2026-10-10T14:21:39Z: no app, console-program or display-server process; none of the four ports listening):
  - `andromeda-pulse-engine run`, pid 675583, ports 24317 / 24318 — started by implement, by hand — terminated (by
    SIGTERM to its own pid after the first signal missed it; gone from `ps`, both ports released).
  - the window app's two boots and their display servers (the smoke entry) — started by the gate tool — terminated
    (none in `ps`; neither 14317 nor 14318 listening).
  - the console program's children of the tests — started by the tests — terminated (none in `ps`).
  - Left on the host, ignored by git: `target/boot-smoke/{stamp}-series/` of the local smoke and
    `target/boot-smoke/ci-38056942758-attempt-1/`; a scratch data dir in the session scratchpad.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: f31027a4 (the parent of the oldest pre-CI commit 4f519c94) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### pulse-app/Cargo.toml — added 6 line(s) in 2 range(s)
added: 8 · 19-23
### pulse-app/src/bin/andromeda-pulse-engine.rs — new file · 3 line(s)
- 1-3 «fn main() -> std::process::ExitCode {»
### pulse-app/src/console.rs — new file · 97 line(s)
- 19-23 @20 «pub enum Command {»
- 25-42 @27 «pub fn parse<I, S>(args: I) -> Option<Command>»
  - 33-37 «let command = match args.next()?.as_ref().to_str() {»
  - 38-40 «if args.next().is_some() {»
- 44-46 «pub fn version_line() -> String {»
- 48-65 @49 «pub fn main<I, S>(args: I) -> ExitCode»
  - 54-64 «match parse(args) {»
- 67-97 «fn run() -> ExitCode {»
  - 68-71 «let runtime = tokio::runtime::Builder::new_multi_thread()»
  - 78-83 «let interpretation = deterministic_mode_enabled().then(|| InterpretationSeat {»
  - 84-92 «let _engine = engine_boot::start(»
### pulse-app/src/engine_boot.rs — new file · 1253 line(s)
- 20-23 «use buffer::{»
- 34-52 «use triage::contract::{»
- 54-57 «use ui_bridge::health::{»
- 101-129 @102 «pub fn resolve_retention_seconds() -> u64 {»
  - 103-106 «let raw = match env::var(ENV_RETENTION_SECONDS) {»
  - 107-118 «let parsed: u64 = match raw.parse::<u64>() {»
  - 119-127 «if !(RETENTION_SECONDS_MIN..=RETENTION_SECONDS_MAX).contains(&parsed) {»
- 131-134 @132 «pub fn resolve_grpc_port() -> Result<OtlpPort, IngestError> {»
- 136-139 @137 «pub fn resolve_http_port() -> Result<OtlpPort, IngestError> {»
- 141-173 @142 «pub fn resolve_port(env_var_name: &'static str, default: u16) -> Result<OtlpPort, IngestError> {»
  - 143-148 «let raw = match env::var(env_var_name) {»
  - 149-160 «let parsed: u16 = match raw.parse::<u16>() {»
  - 161-172 «match OtlpPort::try_from(parsed) {»
- 175-197 @176 «pub fn resolve_data_dir() -> PathBuf {»
  - 177-179 «if let Ok(p) = env::var("ANDROMEDA_PULSE_DATA_DIR") {»
  - 180-195 «if cfg!(target_os = "windows") {»
- 199-241 @200 «pub fn write_pid_file(data_dir: &Path) {»
  - 202-205 «if let Err(e) = fs::create_dir_all(&run_dir) {»
  - 207-213 «let canonical_data = match fs::canonicalize(data_dir) {»
  - 214-220 «let canonical_run = match fs::canonicalize(&run_dir) {»
  - 221-229 «if !canonical_run.starts_with(&canonical_data) {»
  - 231-234 «if let Err(e) = fs::write(&pid_path, pid.to_string()) {»
  - 235-240 «tracing::info!(»
- 243-268 @250 «pub fn publish_workspace_key_for_sidecar(data_dir: &Path, key: &str) {»
  - 251-267 «match workspace_detector::contract::publish_workspace_key(data_dir, key) {»
- 270-309 @271 «pub fn init_buffer(heartbeat_state: &Arc<HeartbeatState>) -> Option<Arc<Mutex<Connection>>> {»
  - 273-287 «let conn = match Connection::open_in_memory() {»
  - 288-299 «if create_schema(&conn).is_err() {»
  - 302-307 «tracing::info!(»
- 311-321 @314 «pub fn init_process(data_dir: &Path) {»
- 323-326 @324 «pub fn os_key_backend() -> Arc<dyn KeychainBackend> {»
- 328-333 @330 «pub enum Program {»
- 335-342 «impl Program {»
  - 336-341 «pub fn label(self) -> &'static str {»
- 344-349 @346 «pub enum SeatKind {»
- 351-354 «pub struct InterpretationSeat {»
- 356-363 @358 «pub struct EngineConfig {»
- 365-376 «impl EngineConfig {»
  - 366-375 @368 «pub fn from_env(data_dir: PathBuf) -> Self {»
- 378-384 «pub struct EngineInputs {»
- 386-411 @388 «pub struct EngineHandles {»
- 413-440 @415 «pub fn emit_boot_record(program: Program, seat: Option<SeatKind>) {»
  - 417-439 «match seat {»
- 442-444 «fn loopback(port: OtlpPort) -> SocketAddr {»
- 446-451 «fn unix_nanos_now() -> i64 {»
  - 447-450 «std::time::SystemTime::now()»
- 453-460 «fn fresh_storm_detector() -> RetryStormDetector {»
  - 454-459 «RetryStormDetector::new(»
- 462-1253 @464 «pub fn start(config: EngineConfig, inputs: EngineInputs) -> EngineHandles {»
  - 465-470 «let EngineConfig {»
  - 471-476 «let EngineInputs {»
  - 495-496 «let bind_status: Arc<dyn ReceiverBindStatus> =»
  - 502-512 «let corpus_arc: Option<Arc<Corpus>> = match Corpus::open(corpus_db_path.clone(), key_backend) {»
  - 513-516 @514 «let corpus_reader: Option<Arc<dyn CorpusReader>> = corpus_arc»
  - 517-519 «let corpus_writer: Option<Arc<dyn CorpusWriter>> = corpus_arc»
  - 521-532 @525 «if let Some(c) = corpus_arc.as_ref() {»
  - 534-539 @537 «if let Some(w) = corpus_writer.as_ref() {»
  - 541-544 «let baseline_persistence: Option<Arc<dyn BaselinePersistence>> =»
  - 546-557 @548 «if let Some(persistence) = baseline_persistence.as_ref() {»
  - 559-563 «let lifecycle_persistence: Option<Arc<dyn LifecyclePersistence>> =»
  - 564-566 «let storm_persistence: Option<Arc<dyn StormPersistence>> = corpus_writer»
  - 567-574 «if lifecycle_persistence.is_none() {»
  - 575-582 «if storm_persistence.is_none() {»
  - 590-595 «let baseline_state = Arc::new(bootstrap_state(»
  - 606-609 «let cadence_sql_runner: Option<Arc<dyn SqlQueryRunner>> = buffer_conn.as_ref().map(|conn| {»
  - 612-614 «let restart_detector = Arc::new(RestartDetector::new(»
  - 617-618 «let baseline_adapter: Arc<dyn SpanObserver> =»
  - 619-622 «let restart_adapter: Arc<dyn SpanObserver> = Arc::new(RestartObserverAdapter::new(»
  - 624-662 @626 «let storm_detector = {»
  - 663-667 «let fingerprint_observer: Option<Arc<dyn buffer::fingerprint::FingerprintObserver>> =»
  - 673-730 «let lifecycle_registry: Arc<dyn ServiceRegistry> = {»
  - 731-738 @734 «let discovery_adapter: Arc<dyn SpanObserver> = Arc::new(DiscoveryObserverAdapter::new(»
  - 739-743 «let span_observer: Arc<dyn SpanObserver> = Arc::new(CompositeSpanObserver::new(vec![»
  - 745-754 @749 «let (incident_workspace_key, digest_project_context) = {»
  - 757-762 @760 «let incident_adapter: Option<Arc<CorpusIncidentPersistence>> = corpus_writer»
  - 763-765 «let incident_persistence: Option<Arc<dyn IncidentPersistence>> = incident_adapter»
  - 766-768 «let incident_durable: Option<Arc<dyn DurableActiveIncidents>> = incident_adapter»
  - 769-783 «let restored_incidents: Vec<Incident> = incident_persistence»
  - 785-786 «let incident_registry: Arc<dyn IncidentRegistry> =»
  - 787-792 «tracing::info!(»
  - 794-802 @798 «let drain_persistence: Option<Arc<dyn buffer::DrainPersistence>> =»
  - 809-825 «match drain_miner.load_from_persistence() {»
  - 836-837 «let config_handle_slot: Arc<OnceLock<config_watcher::ConfigWatchHandle>> =»
  - 838-839 «let (config_settings_tx, config_settings_rx) =»
  - 840-841 «let (cadence_cfg_tx, cadence_cfg_rx) =»
  - 842-844 «let (lifecycle_thresh_tx, lifecycle_thresh_rx) = tokio::sync::watch::channel(»
  - 845-850 «let reevaluator: Arc<dyn RecentWindowReevaluator> = Arc::new(LiveReevaluator::new(»
  - 852-863 «let grpc_addr = match grpc_port {»
  - 864-875 «let http_addr = match http_port {»
  - 877-907 «match config_watcher::start_config_watcher(»
  - 909-945 «match buffer_conn.as_ref() {»
  - 946-983 «if let Some(grpc_addr) = grpc_addr {»
  - 984-1021 «if let Some(http_addr) = http_addr {»
  - 1022-1029 @1025 «tokio::spawn(connection::start_poller(»
  - 1030-1036 «if let Some(persistence) = baseline_persistence.as_ref() {»
  - 1037-1044 «tokio::spawn(start_emitter(»
  - 1045-1050 @1047 «tokio::spawn(start_restart_detector(»
  - 1051-1054 «tokio::spawn(start_storm_detector(»
  - 1055-1063 «let cadence_config = Arc::new(»
  - 1064-1071 «tracing::info!(»
  - 1072-1088 «if let Some(sql_runner) = cadence_sql_runner.as_ref() {»
  - 1095-1168 «if let (Some(sql_runner), Some(corpus_writer_handle)) =»
  - 1170-1177 «tokio::spawn(start_lifecycle_heartbeat(»
  - 1178-1182 «let corpus_basename: String = corpus_db_path»
  - 1183-1190 «if let Some(persistence) = lifecycle_persistence.as_ref() {»
  - 1191-1198 «if let Some(persistence) = storm_persistence.as_ref() {»
  - 1199-1218 @1202 «if let Some(persistence) = incident_persistence.as_ref() {»
  - 1219-1228 «let _engine_tick_handles = heartbeat::spawn_engine_ticks(»
  - 1230-1252 «EngineHandles {»
### pulse-app/src/heartbeat.rs — added 16 line(s) in 3 range(s)
added: 84-85 · 87 · 120-132
- 120-131 @122 «pub fn spawn_window_ticks(»
  - 127-130 «vec![»
### pulse-app/src/lib.rs — added 2 line(s) in 2 range(s)
added: 7 · 15
### pulse-app/src/main.rs — added 86 line(s) in 24 range(s)
added: 4 · 6 · 8 · 15-16 · 19-20 · 22 · 49 · 67 · 75-76 · 80-81 · 99 · 106-112 · 120-123 · 141-153 · 155-171 · 173-174
       176 · 179 · 182-185 · 187-189 · 192-194 · 209-210 · 379-382 · 396-405
  - 160-163 «let connection_impl = ConnectionApiImpl::new(»
  - 164-170 @166 «let storage_impl = engine»
  - 401-405 «use triage::contract::{»
### pulse-app/src/observability.rs — added 8 line(s) in 1 range(s)
added: 1014-1021
### pulse-app/tests/integration_console_engine.rs — new file · 536 line(s)
- 34-38 @35 «fn pick_port() -> u16 {»
- 40-42 «fn loopback(port: u16) -> SocketAddr {»
- 44-55 «fn cleared(dir: &Path) -> Command {»
  - 46-50 «command»
  - 51-53 «if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {»
- 57-64 «struct Running {»
- 66-73 «impl Drop for Running {»
  - 67-72 «fn drop(&mut self) {»
- 75-160 «impl Running {»
  - 76-102 «fn spawn(extra_env: &[(&str, &str)]) -> Self {»
  - 104-106 «fn still_running(&mut self) -> bool {»
  - 108-118 «async fn wait_for_both_ports(&mut self) {»
  - 120-134 @121 «async fn wait_for_records(&mut self, target: &str, count: usize, bound: Duration) {»
  - 136-141 «fn signal(&self, signal: libc::c_int) {»
  - 143-155 «async fn wait_for_end(&mut self) -> ExitStatus {»
  - 157-159 «fn stream(&self, name: &str) -> String {»
- 162-164 «fn accepts(port: u16) -> bool {»
- 166-192 «fn family_lines(dir: &Path) -> Vec<Value> {»
  - 168-170 «let Ok(entries) = std::fs::read_dir(dir.join("logs")) else {»
  - 171-178 «let mut paths: Vec<PathBuf> = entries»
  - 180-190 «for path in paths {»
- 194-199 «fn records<'a>(lines: &'a [Value], target: &str) -> Vec<&'a Value> {»
  - 195-198 «lines»
- 201-203 «fn target_of(line: &Value) -> &str {»
- 205-215 «fn strings_in(value: &Value, out: &mut Vec<String>) {»
  - 206-214 «match value {»
- 217-222 «fn now_ns() -> u64 {»
  - 218-221 «SystemTime::now()»
- 224-256 «fn spans_request(service: &str) -> ExportTraceServiceRequest {»
  - 226-236 «let spans = (0..SPAN_COUNT)»
  - 237-255 «ExportTraceServiceRequest {»
- 258-263 «fn timestamp(line: &Value) -> chrono::DateTime<chrono::Utc> {»
  - 260-262 «chrono::DateTime::parse_from_rfc3339(text)»
- 265-280 «fn assert_every_line_is_a_whole_record(lines: &[Value]) {»
  - 267-279 «for line in lines {»
- 282-295 «fn assert_the_one_exit_record_is_last(lines: &[Value], signal: &str) {»
  - 289-291 «for key in ["exit_class", "exit_code", "exit_code_known", "signal"] {»
- 297-301 «fn the_one_boot_record(lines: &[Value]) -> &Value {»
- 303-310 «fn assert_no_log_record_in(stream: &str, name: &str) {»
  - 304-309 «for line in stream.lines() {»
- 312-318 «fn entries(dir: &Path) -> BTreeSet<String> {»
  - 313-317 «std::fs::read_dir(dir)»
- 320-462 @322 «async fn run_boots_with_no_display_lands_telemetry_and_ends_by_sigterm_with_one_exit_record() {»
  - 329-333 «let pid_file = engine»
  - 334-338 «assert_eq!(»
  - 340-342 «let mut client = TraceServiceClient::connect(format!("http://{}", loopback(engine.grpc)))»
  - 343-346 «client»
  - 357-361 «assert_eq!(»
  - 377-381 «let appended: u64 = records(&lines, "duckdb.append")»
  - 382-385 «assert_eq!(»
  - 387-408 «for line in &lines {»
  - 410-420 «for target in ["ingest.tick", "buffer.tick"] {»
  - 421-426 «for tick in records(&lines, "buffer.tick") {»
  - 427-432 «assert!(»
  - 437-446 «for s in &strings {»
  - 451-454 «assert!(»
  - 458-461 «assert!(»
- 464-498 @466 «async fn run_with_the_deterministic_variable_set_seats_the_canned_runner_and_ends_by_sigint() {»
  - 471-473 «engine»
  - 477-481 «assert_eq!(»
  - 492-497 «assert!(»
- 500-514 @501 «fn version_prints_one_line_exits_zero_and_touches_no_data_dir() {»
  - 503-506 «let output = cleared(dir.path())»
  - 508-511 «assert_eq!(»
- 516-536 @517 «fn an_unknown_word_and_no_word_exit_two_with_usage_and_create_nothing() {»
  - 518-535 «for words in [&["stop"][..], &[][..], &["run", "extra"][..]] {»
### pulse-app/tests/integration_engine_boot.rs — new file · 608 line(s)
- 33-35 «use pulse_app::engine_boot::{»
- 37-40 «use triage::contract::{»
- 50-60 «fn global_capture() -> Captured {»
  - 52-59 «Arc::clone(CAPTURE.get_or_init(|| {»
- 62-64 «struct CapturingSubscriber {»
- 66-99 «impl tracing::Subscriber for CapturingSubscriber {»
  - 67-69 «fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {»
  - 70-72 «fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {»
  - 75-96 «fn event(&self, event: &tracing::Event<'_>) {»
- 101-120 @102 «fn boot_records(»
  - 108-119 «events»
- 124-129 «struct Booted {»
- 131-136 @132 «fn pick_port() -> OtlpPort {»
- 138-143 «fn canned_seat(kind: SeatKind) -> InterpretationSeat {»
  - 139-142 «InterpretationSeat {»
- 145-169 «fn boot(program: Program, interpretation: Option<InterpretationSeat>) -> Booted {»
  - 149-162 «let engine = engine_boot::start(»
  - 163-168 «Booted {»
- 171-181 @172 «async fn wait_until(what: &str, mut condition: impl FnMut() -> bool) {»
  - 174-180 «while !condition() {»
- 183-185 «async fn accepts(addr: SocketAddr) -> bool {»
- 187-196 «async fn wait_for_both_receivers(booted: &Booted) {»
  - 189-195 «while !(accepts(booted.grpc).await && accepts(booted.http).await) {»
- 198-203 «fn now_ns() -> u64 {»
  - 199-202 «SystemTime::now()»
- 205-234 «fn one_span_request(service: &str) -> ExportTraceServiceRequest {»
  - 207-233 «ExportTraceServiceRequest {»
- 236-262 @238 «fn digest_with_cue(workspace: &str) -> Digest {»
  - 239-261 «Digest {»
- 264-275 «fn archived_digests(engine: &EngineHandles) -> u64 {»
  - 265-274 «engine»
- 279-317 @280 «async fn both_receivers_accept_and_a_span_lands_and_lists_its_service() {»
  - 288-290 «let mut client = TraceServiceClient::connect(format!("http://{}", booted.grpc))»
  - 291-294 «client»
  - 297-299 «wait_until("the span advances rows_ingested", || {»
  - 304-309 «wait_until("the service is listed in the lifecycle registry", || {»
  - 312-316 «assert_eq!(»
- 319-358 @320 «async fn a_seated_canned_runner_turns_a_cue_bearing_digest_into_one_incident() {»
  - 327-329 «wait_until("both digest subscribers are listening", || {»
  - 331-335 «engine»
  - 339-341 «wait_until("one incident forms", || {»
  - 343-345 «wait_until("the digest reaches the archive", || {»
  - 348-357 «assert_eq!(»
- 360-399 @361 «async fn an_empty_seat_archives_the_digest_and_forms_no_incident() {»
  - 368-370 «wait_until("the digest persister is listening", || {»
  - 372-376 «engine»
  - 377-379 «wait_until("the digest reaches the archive", || {»
  - 382-386 «assert_eq!(»
  - 387-393 «assert!(»
  - 394-398 «assert_eq!(»
- 401-411 @402 «async fn the_window_program_with_a_seated_runner_reads_window_in_the_record() {»
  - 406-410 «assert_eq!(»
- 419-424 «struct Ended {»
- 426-433 «impl Ended {»
  - 427-432 «fn records(&self, target: &str) -> Vec<&Value> {»
- 435-441 @436 «fn child_arm(arm: &str) -> Option<PathBuf> {»
  - 437-439 «if std::env::var(ARM_ENV).ok().as_deref() != Some(arm) {»
- 443-468 «fn run_child(test: &str, arm: &str) -> Ended {»
  - 446-456 «let mut child = Command::new(exe)»
  - 460-465 «let ended = Ended {»
- 470-489 «fn family_lines(dir: &Path) -> Vec<Value> {»
  - 472-474 «let Ok(entries) = std::fs::read_dir(dir.join("logs")) else {»
  - 475-487 «for entry in entries.flatten() {»
- 491-508 «fn assert_child_ran_the_process_start(ended: &Ended) {»
  - 492-496 «assert_eq!(»
  - 497-501 «assert_eq!(»
  - 503-507 «assert_eq!(»
- 510-521 «fn the_last_record_is_the_one_exit_record(ended: &Ended) -> &Value {»
  - 516-519 «for key in ["exit_class", "exit_code", "exit_code_known", "signal"] {»
- 523-528 «fn entered_runtime() -> tokio::runtime::Runtime {»
  - 524-527 «tokio::runtime::Builder::new_multi_thread()»
- 530-571 @532 «fn the_process_start_keeps_the_panic_record_and_one_exit_record() {»
  - 534-544 «if let Some(dir) = child_arm(ARM) {»
  - 545-548 «let ended = run_child(»
  - 561-566 «let text = ended»
  - 567-570 «assert!(»
- 573-608 @575 «fn the_process_start_records_a_sigterm_and_the_process_still_ends_by_it() {»
  - 578-591 «if let Some(dir) = child_arm(ARM) {»
  - 592-595 «let ended = run_child(»
  - 596-600 «assert_eq!(»
### pulse-app/tests/unit_console_commands.rs — new file · 91 line(s)
- 12-15 @13 «fn code(exit: ExitCode) -> String {»
- 17-20 @18 «fn run_is_the_run_command() {»
- 22-25 @23 «fn version_is_the_version_command() {»
- 27-30 @28 «fn no_word_is_refused() {»
- 32-49 @33 «fn an_unknown_word_is_refused() {»
  - 34-48 «for word in [»
- 51-56 @52 «fn an_extra_word_is_refused() {»
- 58-67 @60 «fn a_word_that_is_not_unicode_is_refused() {»
  - 63-66 «assert_eq!(»
- 69-76 @70 «fn a_refused_command_line_exits_two() {»
- 78-85 @79 «fn version_exits_zero_and_names_the_program_and_its_version() {»
  - 80-83 «assert_eq!(»
- 87-91 @88 «fn the_usage_line_names_the_program_and_both_words() {»
### pulse-app/tests/unit_engine_boot_env.rs — new file · 194 line(s)
- 8-11 «use pulse_app::engine_boot::{»
- 20-29 @21 «fn resolve_port_unset_env_returns_default() {»
  - 23-25 «unsafe {»
- 31-42 @32 «fn resolve_port_unparseable_env_returns_invalid_port_err() {»
  - 34-36 «unsafe {»
  - 38-40 «unsafe {»
- 44-55 @45 «fn resolve_port_overflow_env_returns_invalid_port_err() {»
  - 47-49 «unsafe {»
  - 51-53 «unsafe {»
- 57-68 @58 «fn resolve_port_privileged_env_returns_invalid_port_err() {»
  - 60-62 «unsafe {»
  - 64-66 «unsafe {»
- 70-81 @71 «fn resolve_port_zero_env_returns_invalid_port_err() {»
  - 73-75 «unsafe {»
  - 77-79 «unsafe {»
- 83-95 @84 «fn resolve_port_valid_non_privileged_returns_ok() {»
  - 86-88 «unsafe {»
  - 90-92 «unsafe {»
- 97-109 @98 «fn resolve_port_spec_default_via_env_returns_ok() {»
  - 100-102 «unsafe {»
  - 104-106 «unsafe {»
- 116-122 @117 «fn resolve_retention_seconds_unset_returns_default() {»
  - 118-120 «unsafe {»
- 124-134 @125 «fn resolve_retention_seconds_unparseable_falls_back_to_default() {»
  - 126-128 «unsafe {»
  - 130-132 «unsafe {»
- 136-146 @137 «fn resolve_retention_seconds_below_min_falls_back_to_default() {»
  - 138-140 «unsafe {»
  - 142-144 «unsafe {»
- 148-158 @149 «fn resolve_retention_seconds_above_max_falls_back_to_default() {»
  - 150-152 «unsafe {»
  - 154-156 «unsafe {»
- 160-170 @161 «fn resolve_retention_seconds_in_range_returns_parsed_value() {»
  - 162-164 «unsafe {»
  - 166-168 «unsafe {»
- 172-182 @173 «fn resolve_retention_seconds_at_min_boundary_returns_min() {»
  - 174-176 «unsafe {»
  - 178-180 «unsafe {»
- 184-194 @185 «fn resolve_retention_seconds_at_max_boundary_returns_max() {»
  - 186-188 «unsafe {»
  - 190-192 «unsafe {»
### pulse-app/tests/unit_engine_boot_seam.rs — new file · 154 line(s)
- 19-39 @21 «const ENGINE_PARTS: [&str; 17] = [»
- 43-51 @46 «const MODEL_AND_WINDOW_START: [&str; 4] = [»
- 52-60 «const VARIABLES_THE_CONSOLE_NEVER_READS: [&str; 7] = [»
- 62-65 «fn source(name: &str) -> String {»
- 67-82 @68 «fn each_entry_point_calls_the_two_boot_functions_once() {»
  - 69-81 «for name in [WINDOW_ENTRY, CONSOLE_ENTRY] {»
- 84-90 @85 «fn the_console_bin_only_hands_its_arguments_to_the_console_module() {»
- 92-103 @93 «fn no_entry_point_wires_an_engine_part() {»
  - 94-102 «for name in [WINDOW_ENTRY, CONSOLE_ENTRY, CONSOLE_BIN] {»
- 105-113 @108 «fn the_shared_boot_wires_every_engine_part() {»
  - 110-112 «for part in ENGINE_PARTS {»
- 115-121 @116 «fn the_shared_boot_names_no_window_framework() {»
  - 118-120 «for word in WINDOW_FRAMEWORK {»
- 123-131 @124 «fn the_console_program_names_no_window_framework() {»
  - 125-130 «for name in [CONSOLE_ENTRY, CONSOLE_BIN] {»
- 133-144 @134 «fn the_console_program_names_no_model_no_probe_and_no_window_start() {»
  - 135-143 «for name in [CONSOLE_ENTRY, CONSOLE_BIN] {»
- 146-154 @149 «fn the_window_entry_names_the_model_the_probe_and_its_first_statements() {»
  - 151-153 «for word in MODEL_AND_WINDOW_START {»
### pulse-app/tests/unit_heartbeat_ticks.rs — added 17 line(s) in 4 range(s)
added: 25 · 454 · 469 · 479-492
  - 480-483 «for handle in handles {»
### pulse-app/tests/unit_observability_allowlist_boot_engine.rs — new file · 177 line(s)
- 21-24 @22 «fn the_emit_site_target_is_the_registered_one() {»
- 26-38 @27 «fn boot_engine_resolves_to_an_exact_leaf_with_exactly_its_fields() {»
  - 29-31 «let set = al»
  - 34-37 «assert_eq!(»
- 40-45 @41 «fn boot_engine_has_no_bare_app_fallback() {»
- 50-62 @52 «fn global_capture() -> Captured {»
  - 54-61 «Arc::clone(CAPTURE.get_or_init(|| {»
- 64-66 «struct CapturingSubscriber {»
- 68-102 «impl tracing::Subscriber for CapturingSubscriber {»
  - 69-71 «fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {»
  - 72-74 «fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {»
  - 77-99 «fn event(&self, event: &tracing::Event<'_>) {»
- 104-119 «fn emitted(program: Program, seat: Option<SeatKind>) -> (BTreeMap<String, String>, tracing::Level) {»
  - 109-116 «let mine: Vec<_> = events»
- 121-177 @122 «fn every_program_and_seat_emits_exactly_the_leaf_fields_at_its_level() {»
  - 123-128 «let leaf: BTreeSet<String> = AllowList::production()»
  - 131-176 «for (program, seat, level, labels) in [»
