# Fan-out results — 2026-10-10-console-engine-entry-point

Seven doc-agents, one parallel batch. Each agent's whole instruction was its prompt file in this run dir
(`prompt-{doc}.md`: the amendment-flow template with its doc, report path, keyed-contracts line and detectors
substituted); each return was taken from the agent's own hand-back, unchanged — no preamble stripped, 0 HTML
entities after decoding, in every one. Dispositions were written at Validate.

## architecture

verdict: 18 proposal(s).

### architecture · proposal 1

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process / service identity"
    change: >-
      Add a bullet for the third product binary: "Console engine binary: `andromeda-pulse-engine` — the second `[[bin]]` of the `pulse-app` package (`pulse-app/Cargo.toml:19-23`, path `src/bin/andromeda-pulse-engine.rs`, with `default-run = "pulse-app"` at `pulse-app/Cargo.toml:8`); its `main` hands its arguments to `pulse_app::console::main`, which boots the two receivers, the buffer, the detectors and the corpus through the shared `pulse_app::engine_boot` with no window, no display and no model runner. The product binaries are three: `pulse-app` (the window app), `andromeda-pulse-engine` (the console program), `andromeda-pulse-mcp` (the stdio sidecar, unchanged). The credential service id `com.andromeda.pulse` is written in one place for both engine programs, `engine_boot::os_key_backend()` (`pulse-app/src/engine_boot.rs:323-326`). Under its data dir the console program writes only `logs/agent-latest.jsonl.{date}`, `run/andromeda-pulse.pid`, `run/workspace-key` and `corpus/corpus.db`. Not shown: the binary inside a distribution bundle, or run as a background service."
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Process / service identity: registered the console engine binary `andromeda-pulse-engine` (second `[[bin]]` of `pulse-app`); product binaries 2 → 3."
    rationale: >-
      The report's Changes → Symbols / APIs bullet "A second product binary, `andromeda-pulse-engine`" lands a new process identity; its Expected amendments bullet names this section and measures `andromeda-pulse-engine` at 0 lines in all seven masters and the registries (the section head is at architecture:206). Counts / qualifiers moved: "Product binaries 2 → 3; `[[bin]]` targets of `pulse-app` 1 → 2". The files it writes are the report's "Files the console program writes" bullet.
    basis: "pulse-app/Cargo.toml:19-23"
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5: the plan's expected amendment. The command grammar and its exit code are folded into this bullet (proposal 3)

### architecture · proposal 2

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process / service identity → Dev-build binary"
    change: >-
      Retire "The one CORRECT `target/{profile}/andromeda-pulse*` path is the sidecar below": the `pulse-app` package now declares two `[[bin]]` targets with `default-run = "pulse-app"`, so `target/{profile}/pulse-app` stays the window app's path (the one the harness builds and spawns) and the CORRECT `target/{profile}/andromeda-pulse*` paths are two — the console program `target/{profile}/andromeda-pulse-engine` and the sidecar `andromeda-pulse-mcp`; `target/{profile}/andromeda-pulse` itself still exists on no host. Both `pulse-app` binaries build with `cargo build -p pulse-app --bin pulse-app --bin andromeda-pulse-engine`.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Process / service identity → Dev-build binary: the correct `target/{profile}/andromeda-pulse*` paths are now two (the console engine and the sidecar); `pulse-app` carries two `[[bin]]` targets and `default-run`."
    rationale: >-
      Same claim as the primary, restated in the neighbouring bullet: it says the sidecar is the only correct `andromeda-pulse*` build path, which the report's "A second product binary" bullet makes false (`pulse-app/Cargo.toml:19-23`, `default-run` at `:8`); the report's Gates list the build of both bins and read `target/debug/andromeda-pulse-engine`. `scripts/agent-run.sh boot` still builds and spawns `pulse-app` alone (Changes → Harness / gate surface).
    basis: "pulse-app/Cargo.toml:8"
    dependent-of: D-arch-resources
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### architecture · proposal 3

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources (new bullet beside `xtask CLI surfaces`: the console program's command line)"
    change: >-
      Register the product command line `andromeda-pulse-engine <run|version>` (`pulse-app/src/console.rs`) as a formalized CLI contract: a closed two-word grammar — `run` boots the engine and stays up until SIGTERM or SIGINT, which the existing signal listener records as `app.exit` with class `signal` before re-raising the same signal (the program has no event loop and no `stop` command exists); `version` prints the one line `andromeda-pulse-engine {version}` and exits 0; no word, an unknown word, a second word or a word that is not Unicode prints `usage: andromeda-pulse-engine <run|version>` on stderr and exits 2 (`EXIT_USAGE`), creating nothing under the data dir and writing no log record. It adds no TauRPC procedure, IPC event, capability file, port or environment variable.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources: registered the console program's command line (`run` | `version`; usage on stderr and exit 2 for anything else; ends by SIGTERM / SIGINT; no `stop`)."
    rationale: >-
      The report's Changes → Symbols / APIs bullet "The console program's command grammar" lands a new external command surface with fixed words, output and exit codes (`parse` at `console.rs:25-42`, `main` at `:48-65`, `EXIT_USAGE` 2), and its "Process end" bullet fixes how the process ends; the registry holds every other CLI contract (the xtask verbs, `scripts/agent-run.{sh,ps1}`) under the formalized-CLI-contract rule and holds nothing for this one. "No IPC change" is the report's own bullet.
    basis: "pulse-app/src/console.rs:25-42"
```

**Disposition:** reject as a separate bullet — check 1: playbook `Registry over-reach` (a command name inside an already-registered category); its substance (the two words, usage and exit 2, the end by signal) is carried by proposal 1's bullet

### architecture · proposal 4

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → `run/andromeda-pulse.pid`"
    change: >-
      Replace "PID file written by production binary at boot — `pulse-app/src/main.rs`" with: the pid file is written at boot by BOTH engine programs, the window app and the console program `andromeda-pulse-engine`, through `engine_boot::write_pid_file` (`pulse-app/src/engine_boot.rs:199-241`, moved out of `main.rs` with behaviour unchanged and called by `engine_boot::init_process`, `:311-321`), at the same path — both programs default to the same data dir. The harness readers named in this entry (`harness:status` / `harness:ready` / `harness:settled`, `scripts/agent-run.{sh,ps1}`) are not aimed at the console program although it writes the file.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Filesystem locations: `run/andromeda-pulse.pid` is written by both engine programs through `engine_boot::write_pid_file`, no longer from `main.rs`."
    rationale: >-
      The report's "Files the console program writes" bullet says "The pid file and the workspace key are now written by BOTH programs, at the same paths", and its "Eight boot helpers moved out of `main.rs`" bullet lists `write_pid_file` (`:199-241`); the entry still names `pulse-app/src/main.rs` as the writer. Changes → Harness / gate surface: "the harness verbs that read `run/andromeda-pulse.pid` are not aimed at the console program although it writes that file". The Expected amendments bullet names this entry.
    basis: "pulse-app/src/engine_boot.rs:199-241"
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5

### architecture · proposal 5

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → `run/workspace-key`"
    change: >-
      Replace "published by the app at boot" with: published at boot by either engine program — the window app or the console program `andromeda-pulse-engine` — through `engine_boot::publish_workspace_key_for_sidecar` (`pulse-app/src/engine_boot.rs:243-268`, moved out of `main.rs`, behaviour unchanged) inside the shared `engine_boot::start`, at the same path; the atomic `.tmp`+rename, the canonicalize-and-confine guard, the 4096-byte read bound and the sidecar reader are unchanged.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Filesystem locations: `run/workspace-key` is published by both engine programs through `engine_boot::publish_workspace_key_for_sidecar`."
    rationale: >-
      The report's "Files the console program writes" bullet lists `run/workspace-key` among the four files the console program writes and says the workspace key is "now written by BOTH programs, at the same paths"; the moved helper is `publish_workspace_key_for_sidecar` (`:243-268`). The entry names "the app" as the one publisher. The Expected amendments bullet names this entry.
    basis: "pulse-app/src/engine_boot.rs:243-268"
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5

### architecture · proposal 6

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → Out-of-data-dir lock file (corpus-key creation)"
    change: >-
      Replace "Written by the app AND the `andromeda-pulse-mcp` sidecar, both through `Corpus::open`" with: written by the window app, the console program `andromeda-pulse-engine` and the `andromeda-pulse-mcp` sidecar, all through `Corpus::open` — the two engine programs from the shared `engine_boot::start`, with the service id of the one entry `(com.andromeda.pulse, corpus-key)` written once for both in `engine_boot::os_key_backend()` (`pulse-app/src/engine_boot.rs:323-326`). For the console program the file was read on a hand run; its placement outside the data dir is not asserted by a test.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Filesystem locations: the corpus-key lock file has a third writer, the console program, through the shared engine boot."
    rationale: >-
      Duplicate of the retired "two product processes" claim: the report's "Files the console program writes" bullet says "outside it, the registered corpus-key lock file", and "The one engine boot" bullet names `os_key_backend()` (`:323-326`) as "the one place the credential service id `com.andromeda.pulse` is written for both programs"; the report's Outcome says the lock file's placement outside the data dir was not asserted by the test.
    basis: "pulse-app/src/engine_boot.rs:323-326"
    dependent-of: D-arch-resources
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### architecture · proposal 7

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables → `ANDROMEDA_PULSE_L4_DETERMINISTIC`"
    change: >-
      Replace "selecting the deterministic L4 runner … at `pulse-app` boot" with two readers: (1) the window app, unchanged — truthy seats the canned runner, else the llama-cli runner; (2) the console program `andromeda-pulse-engine run` (`pulse-app/src/console.rs:78-83`), where truthy seats the canned runner (tier for the unknown hardware profile) and UNSET SEATS NOTHING — `engine_boot::start` then spawns no interpretation subscriber and no L4 heartbeat, the digest assembler's cadence subscriber and persister still run, cues and digests are produced, and no incident forms. The arm taken is reported once per `start` on `app.boot.engine` {`program` window|console, `interpretation` model|deterministic|none, `reason` window_runs_model|deterministic_gate_set|deterministic_gate_unset}, WARN when `interpretation` is `none`, INFO otherwise.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Environment variables: `ANDROMEDA_PULSE_L4_DETERMINISTIC` has a second reader, the console program, where unset seats no interpretation runner (no incident forms)."
    rationale: >-
      The report's Changes → Environment variables bullet: "`ANDROMEDA_PULSE_L4_DETERMINISTIC` has a second reader, the console program, where unset means 'seat nothing'"; "The interpretation seat" bullet gives the two arms (`console.rs:78-83`) and "The boot record `app.boot.engine`" bullet the labels (`engine_boot.rs:413-440`). The entry at architecture:246 names `pulse-app` boot as the one place the gate is read and says nothing of an unset arm that seats nothing. The Expected amendments bullet names this entry.
    basis: "pulse-app/src/console.rs:78-83"
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5; the unset arm was decided by the operator at P4 (inputs#I2)

### architecture · proposal 8

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables (reserved at arch level)"
    change: >-
      Add one roster line for the second engine program: the console program `andromeda-pulse-engine` reads, through the shared engine boot, `ANDROMEDA_PULSE_DATA_DIR` (and `XDG_CONFIG_HOME` / `HOME` for its default), `ANDROMEDA_PULSE_OTLP_GRPC_PORT`, `ANDROMEDA_PULSE_OTLP_HTTP_PORT`, `ANDROMEDA_PULSE_RETENTION_SECONDS`, `ANDROMEDA_PULSE_LOG_LEVEL` / `RUST_LOG`, `ANDROMEDA_PULSE_CORPUS_PASSPHRASE`, the system `XDG_RUNTIME_DIR` (the corpus-key lock dir), the variables `Thresholds::from_env` reads and `ANDROMEDA_PULSE_L4_DETERMINISTIC`; it reads no model path variable, no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, no `ANDROMEDA_PULSE_HARDWARE_PROFILE` and no `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` (pinned by source text in `pulse-app/tests/unit_engine_boot_seam.rs`), and the plugin host with its directory variable stays in the window app's `main`. No variable is new.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Environment variables: recorded which registered variables the console program reads and which it never reads; none new."
    rationale: >-
      The report's Changes → Environment variables bullet lists exactly this read set and never-read set for the new program ("none new"), and the registry is where a variable's readers are recorded; `EngineConfig::from_env` (`engine_boot.rs:366-375`) reads the two port variables and the retention variable through the moved resolvers.
    basis: "pulse-app/src/engine_boot.rs:366-375"
```

**Disposition:** reject — check 1: playbook `Registry over-reach` (the registry lists variables, not a per-program read roster); the one variable whose meaning differs per reader is amended by proposal 7, and the console program's read set lands in security-plan §Input Validation

### architecture · proposal 9

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables → `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`"
    change: >-
      Replace "which `pulse-app/src/main.rs` hands to" the baseline state with: `Thresholds::from_env()` is resolved once per boot inside the shared engine boot (`pulse-app/src/engine_boot.rs`, `start`), which builds the baseline state from it for BOTH programs — the window app and the console program — so `main.rs` no longer carries the hand-off; the bounded parse, the 3600 s default, the one-bound gating and the WARN on a non-default bound are unchanged.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Environment variables: `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` is resolved in the shared `engine_boot::start` for both programs, not in `main.rs`."
    rationale: >-
      Duplicate of the retired "`main.rs` is where the engine is wired" claim: the report's "The one engine boot" bullet says `start` (`engine_boot.rs:462-1253`) "builds the engine state" (the baseline state at `:590-595`), its "`main.rs`" bullet says `main` only builds routers, plugin host, `VizState`, window geometry and the Tauri builder from the returned handles, and its Environment variables bullet says the console program reads "the variables `Thresholds::from_env` reads" through the shared code; `unit_engine_boot_seam` pins that neither entry point wires an engine part.
    basis: "pulse-app/src/engine_boot.rs:590-595"
    dependent-of: D-arch-resources
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### architecture · proposal 10

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Network ports"
    change: >-
      Add to the two port lines: both engine programs bind these two receivers and nothing else — the window app and the console program `andromeda-pulse-engine`, each through the shared `engine_boot::start`, and both default to 4317 / 4318. The bind address is not configurable: the engine config holds two validated PORTS and `start` builds `127.0.0.1:{port}` itself (`loopback`, `pulse-app/src/engine_boot.rs:442-444`), so no caller can ask for a non-loopback bind; a rejected port variable records the bind as failed with `reason = "invalid_port"` and starts no receiver. No port is new.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Occupied Resources → Network ports: `:4317` / `:4318` have a second default binder, the console program; the loopback bind is fixed in `engine_boot::start`."
    rationale: >-
      The report's Changes → Ports / sockets bullet: "none new. The console program binds the same two loopback receivers at the ports the two registered variables name, and nothing else. Both programs default to 4317 / 4318 and to the same data dir"; "The bind address is not configurable" bullet gives the mechanism (`engine_boot.rs:442-444`). The registry lists the two ports with one occupant; that no third socket listens is by construction and was not measured from the process's socket list (report Outcome).
    basis: "pulse-app/src/engine_boot.rs:442-444"
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### architecture · proposal 11

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Project Intent → Product type"
    change: >-
      Replace "Not a web app, not a CLI, not a microservice fleet" with: a cross-platform desktop application (Windows / macOS / Linux) shipped as native bundles, and — since chunk 2026-10-10-console-engine-entry-point — a second program over the same engine, the console program `andromeda-pulse-engine`, driven by a closed two-word command line (`run` | `version`) with no window and no display. Not a web app, not a microservice fleet. What the console program is not yet: a background service, a bundled release artifact, or a place where an incident forms without the deterministic gate set.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Project Intent → Product type: the product is two programs over one engine (the desktop window app and the console program `andromeda-pulse-engine`); 'not a CLI' retired."
    rationale: >-
      The chunk built a command-line program, which the locked sentence excludes: the report's Changes → Symbols / APIs bullets "A second product binary" and "The console program's command grammar" (`pulse-app/src/console.rs:19-23` for `Command`), and its Expected amendments bullet names §Project Intent → Product type with the search `not a CLI` — architecture 1 line. The operator's wrap directive (inputs#I4) orders that the masters describe two programs and that no sentence reads as if the console engine already did what a later entry builds; the report's Outcome keeps P-086 `planned` with only its first half shown.
    basis: "pulse-app/src/console.rs:19-23"
```

**Disposition:** apply — check 1: no rule governs retiring a Project Intent sentence, and the operator's recorded direction settles it: the plan's P5-approved `Expected amendments (wrap)` entry names the change itself ("no longer 'not a CLI'"), and the wrap directive orders the masters to describe two programs (inputs#I4 item 3); check 5

### architecture · proposal 12

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Design Philosophy → Single-process modular monolith"
    change: >-
      Replace "fourteen library crates linked into the `pulse-app` Tauri binary … the OS only sees one process" with: each product program is one process — the fourteen library crates are wired by ONE shared engine boot, `pulse_app::engine_boot` (`init_process`, then `start`), which both entry points call: the Tauri window app (`pulse-app`, `main.rs`) and the console program (`andromeda-pulse-engine`, `console.rs`). Neither entry point wires an engine part itself, the shared boot and the console module name neither `tauri` nor `taurpc` (pinned by `pulse-app/tests/unit_engine_boot_seam.rs`), and every engine task is spawned with `tokio::spawn`, never through the window framework. Sixteen workspace members stands; `tauri` stays in `pulse-app`'s manifest. Channels and in-process latency as before.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Design Philosophy: the monolith is wired by one shared engine boot (`engine_boot::init_process` + `start`) called by two entry points, the window app and the console program; each program is one process."
    rationale: >-
      The same retired claim (one Tauri binary, one process) in the philosophy bullet: the report's "The one engine boot both programs call" bullet (`init_process` at `engine_boot.rs:311-321`, `start` at `:462-1253`), its Crates / modules bullet ("No workspace member is added (sixteen stands); no crate's dependency on the window framework is cut … `engine_boot.rs` and `console.rs` name neither `tauri` nor `taurpc` (pinned)"), and the Expected amendments bullet naming §Design Philosophy.
    basis: "pulse-app/src/engine_boot.rs:462-1253"
    dependent-of: D-arch-decisions
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5

### architecture · proposal 13

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Project Intent → Growth model"
    change: >-
      Replace "wired into one Tauri binary" with: wired by one shared engine boot into two programs of the `pulse-app` package — the Tauri window app and the console engine `andromeda-pulse-engine`; the crate count wording and the plugin extension layer are untouched by this chunk.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Project Intent → Growth model: the crates are wired into two programs over one engine boot, no longer 'one Tauri binary'."
    rationale: >-
      The same retired claim restated in Project Intent: the report's "A second product binary" bullet makes the `pulse-app` package two `[[bin]]` targets (`pulse-app/Cargo.toml:19-23`) and its Crates / modules bullet adds the modules `engine_boot` (`pulse-app/src/lib.rs:15`) and `console` (`lib.rs:7`) with no new workspace member.
    basis: "pulse-app/src/lib.rs:15"
    dependent-of: D-arch-decisions
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### architecture · proposal 14

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → Deployment model"
    change: >-
      Replace the runtime-topology line with: on the user's machine the engine runs as one of two programs — the Tauri process hosting all fourteen library crates and the embedded webview, or the console program `andromeda-pulse-engine run`, one process hosting the same engine with no webview, no window and no model runner, ended by SIGTERM or SIGINT — plus one optional `andromeda-pulse-mcp` sidecar process (conditions unchanged). Both engine programs default to the same data dir, the same two ports and the same pid file path. Not built: a background-service form of the console program, a `stop` command, a release bundle that carries it.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Infrastructure Patterns → Deployment model: runtime topology names two engine programs (the Tauri window app, the console program) and the optional sidecar."
    rationale: >-
      The keyed contract `.andromeda/registries/contracts/architecture/deployment-model.md` says "one Tauri process hosting all fourteen library crates and the embedded webview"; the report's "A second product binary", "Process end", "Ports / sockets" and "Files the console program writes" bullets describe the second program, and the Expected amendments bullet names this key. "Not built" is the report's Reverted / negative API facts bullet (a `stop` command) and its Outcome (the service half of P-086 not shown).
    basis: "pulse-app/src/bin/andromeda-pulse-engine.rs:1-3"
    dependent-of: D-arch-decisions
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5 (a keyed contract: its key file is edited)

### architecture · proposal 15

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: >-
      In the `pulse-app/` subtree, reword the crate comment to "binary crate: the Tauri window app and the console engine, both over the shared engine boot" and list under `src/`: `main.rs` (the window app's entry), `lib.rs`, `engine_boot.rs` (the one engine boot both programs call), `console.rs` (the console program's command grammar and its `run`), and `bin/andromeda-pulse-engine.rs` (the console binary's `main`, which only hands its arguments to `pulse_app::console::main`).
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Infrastructure Patterns → Project directory structure: `pulse-app/src/` gains `engine_boot.rs`, `console.rs` and `bin/andromeda-pulse-engine.rs`; `pulse-app/` is no longer described as the Tauri binary alone."
    rationale: >-
      The keyed contract `.andromeda/registries/contracts/architecture/project-directory-structure.md` shows `pulse-app/` as "Tauri binary crate that wires the workspace" with `src/main.rs` and `src/lib.rs` only; the report's Files bullet lists the three new source files (a new `src/bin/` directory among them), its Crates / modules bullet the two new library modules (`lib.rs:15`, `lib.rs:7`), and the Expected amendments bullet names this key.
    basis: "pulse-app/src/lib.rs:7"
    dependent-of: D-arch-decisions
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5 (a keyed contract: its key file is edited)

### architecture · proposal 16

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → Message broker row"
    change: >-
      Replace "N/A — single-process desktop app" with: N/A — each product program is a single process (the desktop window app; the console engine `andromeda-pulse-engine`), its parts joined by in-process tokio channels.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Stack → Message broker: N/A restated for two single-process programs."
    rationale: >-
      One of the two `single-process` lines the report's Expected amendments bullet counts in architecture under the "second program over the shared engine boot" amendment: the row describes the product as one desktop app, and the report's "A second product binary" bullet adds a program that is not a desktop app; the N/A itself stands (no dependency added — report Dependencies: "none added, none bumped").
    basis: "pulse-app/Cargo.toml:19-23"
    dependent-of: D-arch-decisions
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### architecture · proposal 17

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Mobile / Message Broker / Push Notifications] N/A"
    change: >-
      Replace "single-process desktop app on Windows/macOS/Linux only" with: each product program is a single process — the desktop window app on Windows / macOS / Linux and the console engine `andromeda-pulse-engine` — with no mobile target and no message broker; OS-native local notifications only.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Established Decisions → [Mobile / Message Broker / Push Notifications]: N/A restated for two single-process programs."
    rationale: >-
      The second of the two `single-process` lines the report's Expected amendments bullet counts in architecture; same retired wording as the Stack row, restated in the decision entry. The decision's N/A verdicts are not contradicted — the console program's `run` builds one tokio runtime of its own (`pulse-app/src/console.rs:68-71`) and adds no broker.
    basis: "pulse-app/src/console.rs:68-71"
    dependent-of: D-arch-decisions
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### architecture · proposal 18

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Cross-cutting Patterns → Test-time telemetry injection"
    change: >-
      Add after the TauRPC read-back sentence: the console program mounts no TauRPC router, so a test of it still injects over real OTLP (gRPC to the bound receiver) and reads state back from the program's own log records on a spawned run (stored span rows from its `duckdb.append` records) or from the `EngineHandles` that `engine_boot::start` returns in an in-process arm; the TauRPC query routers remain the read-back path for the window app.
    sidecar: "2026-10-10 · chunk 2026-10-10-console-engine-entry-point · §Cross-cutting Patterns → Test-time telemetry injection: read-back for the console program is its own log records or the shared boot's `EngineHandles`; injection stays real OTLP."
    rationale: >-
      The pattern assumes the one program with TauRPC routers ("Tests that need to inspect buffer state read it back through the TauRPC `traces.*` / `metrics.*` / `logs.*` query routers"); the report's Crates / modules bullet pins that `engine_boot.rs` and `console.rs` name neither `tauri` nor `taurpc`, and its Coverage of new surfaces rows put the console program's tests in `integration_console_engine` (a spawned binary; the appended-rows read at `pulse-app/tests/integration_console_engine.rs:377-381`) and `integration_engine_boot` (in-process arms; `:297-299`). Injection itself conforms — both send spans over gRPC.
    basis: "pulse-app/tests/integration_console_engine.rs:377-381"
    dependent-of: D-arch-decisions
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

## security-plan

verdict: 14 proposal(s).

### security-plan · proposal 1

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (the boundary table, and the section's opening `Boundary` sentence)"
    change: >-
      Add a row "Console program command line — `andromeda-pulse-engine <run|version>` (the second `[[bin]]` of the `pulse-app` package)": What to validate — a closed two-word grammar parsed to the closed `Command` enum by `pulse_app::console::parse`; `run` boots the engine and stays up until signalled, `version` prints one line (`andromeda-pulse-engine {version}`) and exits 0; no word, an unknown word, a second word or a word that is not Unicode prints `usage: andromeda-pulse-engine <run|version>` on stderr and exits 2, creating nothing (no data dir entry, and no log record by design); no `stop` command exists. How — `std` only, no new library; pinned by `unit_console_commands` (9 tests) and the spawned-binary arms of `integration_console_engine` (exit codes, an empty data dir). Also name the console program's command line among the boundaries listed in the section's opening sentence.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Input Validation: row added for the command line of the second product binary `andromeda-pulse-engine` (closed grammar `run` | `version`, anything else usage on stderr + exit 2, creating nothing), and the boundary named in the section's opening sentence.
    rationale: >-
      The report's Changes add a new external-input surface the section does not register: Symbols / APIs "A second product binary, `andromeda-pulse-engine`" and "The console program's command grammar" (closed, two words; exit 2 with usage on anything else, creating nothing), with the Coverage row "`andromeda-pulse-engine` command line → validation closed two-word grammar, exit 2 on anything else · instrumentation n/a (a refused command line writes no record by design) · tests unit (`unit_console_commands`, 9) + integration". The report's Expected amendments name this row ("§Input Validation (a row for the console program's command words ...)"). The code-side validation IS present per the report, so no unvalidated boundary was found; the drift is that §Input Validation carries no row for the surface, so it cannot yet be read as validated per that section. Carried at the detector's severity because it is a new product input surface: the report records no classification of it as a boundary widening and no ratification beyond the operator's acceptance (inputs#I3, inputs#I4), so the row should carry none unless the operator supplies it.
    basis: "pulse-app/src/console.rs:25-42 (`parse`); pulse-app/Cargo.toml:19-23 (the second `[[bin]]`); .andromeda/security-plan.md:121 (the section head, as the report states it)"
```

**Disposition:** ESCALATED and resolved — apply. Check 1: rule `New env var (or similar bounded config input) registration` (routine: a closed two-word grammar, code-validated and unit-tested) met rule `Boundary widening` (escalate: "a validated surface admits a new input class"). Brought to the operator at the P2 halt; word: "Not a widening (Recommended)" with the note "classification mine, dated today, with one more basis for the sidecar entry: the requirement itself says the engine is a console program driven by commands (P-086), and the founder approved that requirement with the route on 2026-10-09 - so a command line is the ruled form, and these two words are its first closed members. A later command that takes a value, a path or a network address is a new question each time." — the operator (the pc overseer), 2026-10-10, given here. The discriminator rule is proposed at the route-resolve card.

### security-plan · proposal 2

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the `CLI / env var inputs` row"
    change: >-
      In the row: (1) `l4_deterministic` now has TWO readers — the window app `pulse-app` (truthy seats the canned runner, else the llama-cli runner, as before) and the console program `andromeda-pulse-engine` (truthy seats the canned runner; unset seats NOTHING: no interpretation subscriber and no L4 heartbeats are spawned, cues and digests are still produced, no incident forms, and the boot says so on the WARN `app.boot.engine` record with `interpretation = none`, `reason = deterministic_gate_unset`); (2) through the shared `engine_boot` code the console program reads `ANDROMEDA_PULSE_DATA_DIR`, the two OTLP port variables, the retention variable, `ANDROMEDA_PULSE_LOG_LEVEL` / `RUST_LOG`, `ANDROMEDA_PULSE_CORPUS_PASSPHRASE`, the system `XDG_RUNTIME_DIR` and the variables `Thresholds::from_env` reads, under the rules this row already states (the resolvers moved, behaviour unchanged), and it reads none of the three L4 path variables, `ANDROMEDA_PULSE_L4_ALLOW_ROOT` or `ANDROMEDA_PULSE_HARDWARE_PROFILE` — so "the shipped binary" in the row's PRODUCT-CONSUMED path clause is the window app `pulse-app`; (3) the two port variables yield validated ports only — the bind host is not an input in either program: `engine_boot::start` builds `127.0.0.1:{port}` itself, and a rejected port records the bind failed with `reason = "invalid_port"` and starts no receiver.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Input Validation, `CLI / env var inputs` row: `ANDROMEDA_PULSE_L4_DETERMINISTIC` gains a second reader (the console program, where unset means no interpretation seat); the variables the console program reads through the shared engine boot and the L4 / hardware variables it never reads are stated; the OTLP bind host is recorded as not an input (ports only, loopback built by `engine_boot::start`).
    rationale: >-
      Report Changes → Environment variables: "none new. `ANDROMEDA_PULSE_L4_DETERMINISTIC` has a second reader, the console program, where unset means 'seat nothing'", followed by the list the console program reads through the shared code and the list it never reads ("pinned by source text in `unit_engine_boot_seam`", and "it constructs no `LlamaCliInference` and no `HardwareProfileDetector`"). Symbols / APIs → "The interpretation seat" and "The boot record `app.boot.engine`" give the unset arm's behaviour and its WARN; "The bind address is not configurable" gives the ports-only config. The row today describes the variable as a single bool ("default false") with one reader and names "the shipped binary" in the singular for the L4 path variables; with a second product binary both readings are incomplete. The report's Expected amendments name this row ("its 'CLI / env var inputs' row at :138 ... the variable's second reader").
    basis: ".andromeda/security-plan.md:138 (the row, as the report states it); pulse-app/src/console.rs:78-83 (the console seat); pulse-app/src/engine_boot.rs:442-444 (`loopback`); pulse-app/tests/unit_engine_boot_seam.rs:52-60 (the seven variables the console never reads)"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; the variable's second reader was decided by the operator at P4 (inputs#I2)

### security-plan · proposal 3

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → the `CLI / env var inputs` row (the `nv_disable_explicit_sync` clause)"
    change: >-
      Where the clause says the product SETS `__NV_DISABLE_EXPLICIT_SYNC` to `1` at "the first statement of `main()`", name the actor: the window app's `main()` in `pulse-app/src/main.rs`. The console program's `main` only hands its arguments to `pulse_app::console::main`, and the console files name none of the window entry's first statements.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Input Validation, `CLI / env var inputs` row: the `__NV_DISABLE_EXPLICIT_SYNC` presence-read and set are scoped to the window app's `main()`, now that the `pulse-app` package has a second `main`.
    rationale: >-
      With a second `[[bin]]` in the package, "`main()`" and "the product" no longer name one actor. Report Changes: `main.rs` — "its first two statements and its own runtime build are unchanged and still pinned by `unit_xlib_threads`"; the console bin's `main` "hands its arguments to `pulse_app::console::main`"; the seam pin's test `the_console_program_names_no_model_no_probe_and_no_window_start` and the deviation line "the window entry names the model, the probe and its first statements". Limit, stated plainly: the report does not name the variable or `apply_linux_default` in this connection — the scoping rests on the seam pin; one read of its `MODEL_AND_WINDOW_START` list settles whether the posture default is among the four.
    basis: "pulse-app/src/bin/andromeda-pulse-engine.rs:1-3; pulse-app/tests/unit_engine_boot_seam.rs:133-144 (the test), :43-51 (the list)"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 4

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Input Validation → the `Published workspace key` row"
    change: >-
      Replace "written by the app at boot" with: written at boot by whichever engine program runs — the window app `pulse-app` or the console program `andromeda-pulse-engine` — through the one shared `engine_boot::publish_workspace_key_for_sidecar`, at the same path `<data_dir>/run/workspace-key`; the write-side canonicalize-and-confine and the sidecar's bounded read are unchanged.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Input Validation, `Published workspace key` row: the writer is either engine program (window app or console program) through the shared engine boot, at the same path.
    rationale: >-
      Report Changes → "Files the console program writes": under its data dir it writes `run/workspace-key`, and "The pid file and the workspace key are now written by BOTH programs, at the same paths"; the helper `publish_workspace_key_for_sidecar` moved out of `main.rs` into the shared boot, "behaviour unchanged". The row names "the app" as the one writer. The report counts `workspace-key` on 3 lines of security-plan; this is one of the two that name the writer.
    basis: "pulse-app/src/engine_boot.rs:243-268"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 5

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Threat Model Summary → Attack surface → `CLI input (env vars + binary launch)`"
    change: >-
      Entry point: add the command line of the console program `andromeda-pulse-engine` (`run` | `version`), and say the system `XDG_RUNTIME_DIR` is read through `Corpus::open` by all three product binaries (`pulse-app`, `andromeda-pulse-engine`, `andromeda-pulse-mcp`), not "both product binaries". Trust boundary: add the closed two-word grammar — any other command line prints usage on stderr and exits 2, creating nothing (§Input Validation).
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Threat Model Summary, Attack surface, CLI input: the console program's command line added as an entry point with its closed-grammar trust boundary; `XDG_RUNTIME_DIR` readers restated as the three product binaries.
    rationale: >-
      Report Changes: "The product binaries are now three: `pulse-app` (the window app), `andromeda-pulse-engine` (the console program), `andromeda-pulse-mcp` (the stdio sidecar, unchanged)"; "The console program's command grammar"; and Environment variables lists the system `XDG_RUNTIME_DIR` (the corpus-key lock dir) among what the console program reads. The bullet's entry point names no command words, and its "read by both product binaries" counts two. The report's Expected amendments name §Threat Model Summary (security-plan:18).
    basis: "pulse-app/src/console.rs:25-42; .andromeda/security-plan.md:18 (the section head, as the report states it)"
    dependent-of: D-security-input
```

**Disposition:** apply with proposal 1 — its command-line entry point follows proposal 1's disposition; the three-binaries count is check 1 `Accurate this-chunk addition`

### security-plan · proposal 6

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Threat Model Summary → Attack surface → `CLI input (env vars + binary launch)` (the `__NV_DISABLE_EXPLICIT_SYNC` clauses of Entry point and Trust boundary)"
    change: >-
      Name the actor of the `__NV_DISABLE_EXPLICIT_SYNC` presence-read and set as the window app: "at the head of the window app's `main()` (`pulse-app/src/main.rs`)" in Entry point, and "the window app writes the process environment only before any thread exists" in Trust boundary.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Threat Model Summary, Attack surface, CLI input: the `__NV_DISABLE_EXPLICIT_SYNC` read and set scoped to the window app's `main()`.
    rationale: >-
      The same claim as the `nv_disable_explicit_sync` clause of the §Input Validation row, restated here with "`main()`" and "the product" as the actor; a second `main` now exists in the package (report: the console bin's `main` hands its arguments to `pulse_app::console::main`; `main.rs`'s first two statements unchanged and pinned by `unit_xlib_threads`; the seam pin holds the console files free of the window entry's first statements). Same limit as that proposal: the report does not name the variable in this connection.
    basis: "pulse-app/src/bin/andromeda-pulse-engine.rs:1-3; pulse-app/tests/unit_engine_boot_seam.rs:133-144"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 7

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Threat Model Summary → Attack surface → `filesystem reads (config + workspace detection)`"
    change: >-
      Replace "`<data_dir>/run/workspace-key`, written by the app and read by the sidecar" with: written by the window app or the console program `andromeda-pulse-engine` (whichever boots the engine, through the shared engine boot) and read by the sidecar.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Threat Model Summary, Attack surface, filesystem reads: the published workspace key's writer is either engine program.
    rationale: >-
      Report Changes → "Files the console program writes": "The pid file and the workspace key are now written by BOTH programs, at the same paths." The Entry point line names "the app" as the one writer — the second of the two security-plan lines that state the key's writer.
    basis: "pulse-app/src/engine_boot.rs:243-268"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 8

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Threat Model Summary → Infrastructure → Hosting"
    change: >-
      Replace "runs as a single Tauri process" with the roster of three product binaries: the window app `pulse-app` (the Tauri process), the console program `andromeda-pulse-engine` (the same engine boot — the two loopback receivers, the buffer, the detectors, the corpus — with no window framework, no webview, no TauRPC and no plugin host; driven by `run` | `version`; ended by SIGTERM or SIGINT), and the stdio sidecar `andromeda-pulse-mcp` (unchanged). Say nothing of how the console program is distributed or of it running as a background service: neither is built or measured.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Threat Model Summary, Infrastructure, Hosting: "a single Tauri process" replaced by the three product binaries (window app, console program over the shared engine boot, stdio sidecar).
    rationale: >-
      Report Changes: "The product binaries are now three"; "The one engine boot both programs call"; Crates / modules: "`engine_boot.rs` and `console.rs` name neither `tauri` nor `taurpc` (pinned)"; Environment variables: "the plugin host and its directory variable stay in `main`"; Process end: the console program "ends by SIGTERM or SIGINT through the existing listener". This is the process roster the report's Expected amendments name for §Threat Model Summary. The wording limit follows the report: `tauri build` with a second `[[bin]]` is unmeasured, and P-086 ("runs in the background" as a service) is not claimed (inputs#I4).
    basis: "pulse-app/Cargo.toml:19-23; pulse-app/src/engine_boot.rs:462-1253 (`start`)"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5: the plan's expected amendment (the process roster)

### security-plan · proposal 9

```yaml
  - detector: D-security-input
    severity: warning
    section: "document preamble → the `Stack adaptability` note (above §Threat Model Summary)"
    change: >-
      Extend "This plan is for a **local-first Tauri 2 desktop app**" to say the product is the Tauri 2 window app plus, since chunk 2026-10-10-console-engine-entry-point, a console program (`andromeda-pulse-engine`) over the same engine boot with no window framework; the adapted anti-pattern framing (no public API surface, no user auth, no cloud DB; loopback receivers dominant) is unchanged.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — preamble, Stack adaptability note: the plan's subject restated as the window app plus a console program over the same engine boot.
    rationale: >-
      The note states the plan's subject as one Tauri desktop app; the report's Changes add a second product program that names no window framework ("A second product binary"; "`engine_boot.rs` and `console.rs` name neither `tauri` nor `taurpc`"). The operator's wrap directive as the report records it: "the masters describe two programs" (inputs#I4). The preamble is not a `##` section; if it is outside what the orchestrator amends, the Hosting proposal carries the same fact.
    basis: "pulse-app/Cargo.toml:19-23"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 10

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Data Protection → At rest — per medium → `Persistent incident corpus`"
    change: >-
      In "concurrently starting processes (the app and the MCP sidecar, or several instances) converge on that one key", name all three: the window app, the console program `andromeda-pulse-engine` and the MCP sidecar, or several instances.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Data Protection, At rest, Persistent incident corpus: the processes that converge on the one corpus key now include the console program.
    rationale: >-
      Report Changes: `engine_boot::os_key_backend()` is "the one place the credential service id `com.andromeda.pulse` is written for both programs", and the console program writes `corpus/corpus.db` under its data dir and reads `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` and `XDG_RUNTIME_DIR` through the shared code — it opens the corpus through the same key flow. The parenthesis enumerates two process kinds. Library and flow are unchanged (no D-security-auth drift); only the roster is stale.
    basis: "pulse-app/src/engine_boot.rs:323-326"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 11

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Data Protection → At rest — per medium → `Corpus-key lock file`"
    change: >-
      Replace "written by the app and the MCP sidecar through `Corpus::open`" with: written by the window app, the console program `andromeda-pulse-engine` and the MCP sidecar through `Corpus::open`.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Data Protection, At rest, Corpus-key lock file: the console program added to the writers of the content-free lock file.
    rationale: >-
      Report Changes → "Files the console program writes": "outside it, the registered corpus-key lock file". The bullet names two writers. The report's Outcome adds a limit worth keeping out of the body: the lock file's placement outside the data dir was not asserted by the console test.
    basis: "pulse-app/src/engine_boot.rs:323-326"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 12

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Security Anti-Patterns → Input → the corpus-key lock dir carve-out (`XDG_RUNTIME_DIR`)"
    change: >-
      Replace "both shipped binaries read the system `XDG_RUNTIME_DIR` (Linux) through `Corpus::open`" with: all three product binaries (`pulse-app`, `andromeda-pulse-engine`, `andromeda-pulse-mcp`) read it through `Corpus::open`; the fail-closed rule is unchanged.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Security Anti-Patterns, Input, corpus-key lock dir carve-out: "both shipped binaries" restated as the three product binaries.
    rationale: >-
      The same count as the §Threat Model Summary CLI-input bullet's "both product binaries", restated here as "both shipped binaries". Report Changes: product binaries 2 → 3, and the console program reads the system `XDG_RUNTIME_DIR` (the corpus-key lock dir) through the shared code. "Product binaries" is the report's term; whether the console binary is shipped by the release workflow is unmeasured there.
    basis: "pulse-app/Cargo.toml:19-23"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 13

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Security Anti-Patterns → Input → `NARROWED EXCEPTION` (the three product-consumed L4 path vars)"
    change: >-
      Where the paragraph says the three L4 path variables "are read by the SHIPPED binary", name the reader: the window app `pulse-app` alone among the product binaries. Add that the console program `andromeda-pulse-engine` reads no model path variable and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, constructs no `LlamaCliInference` and emits no `interpretation.model.*` record (pinned by source text in `unit_engine_boot_seam`), so the scoped consequence — an environment-set path read as a model or executed as the inference binary — does not reach it.
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Security Anti-Patterns, Input, NARROWED EXCEPTION: the three L4 path variables and the allow-root are scoped to the window app; the console program reads none of them and builds no inference runner.
    rationale: >-
      Report Changes → Environment variables: the console program "reads no model path variable, no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, no `ANDROMEDA_PULSE_HARDWARE_PROFILE` ... (pinned by source text in `unit_engine_boot_seam`), and it constructs no `LlamaCliInference` and no `HardwareProfileDetector`"; Outcome: "no model variable, no runner, no probe; no `interpretation.model.*` record — MET". "The SHIPPED binary" in the singular no longer names one program, and the paragraph's accepted risk is narrower than a reader of three product binaries would assume.
    basis: "pulse-app/tests/unit_engine_boot_seam.rs:52-60"
    dependent-of: D-security-input
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### security-plan · proposal 14

```yaml
  - detector: D-security-logging
    severity: warning
    section: "§Security Anti-Patterns → Logging (a paragraph beside the `app.exit` NO-SCRUB boundary)"
    change: >-
      Add: **`app.boot.engine` is a deliberate NO-SCRUB log boundary** (chunk 2026-10-10-console-engine-entry-point): one product-originated record per engine boot — exactly once per `engine_boot::start`, from either program — behind its OWN exact allowlist leaf `{program, interpretation, reason}` beside `app.boot.render.posture`. Nothing client-controlled or free-text crosses it: three closed labels (`window`|`console` · `model`|`deterministic`|`none` · `window_runs_model`|`deterministic_gate_set`|`deterministic_gate_unset`), no path and no variable value; WARN when `interpretation` is `none` (its message names the gating variable by NAME only), INFO otherwise. Still no bare `app` key; pinned ×4 in `pulse-app/tests/unit_observability_allowlist_boot_engine.rs` (exact resolve, field-set equality both ways, the no-bare-`app` discriminator, emit-site capture for the four program and seat pairs; mutation-checked).
    sidecar: >-
      2026-10-10 (chunk 2026-10-10-console-engine-entry-point) — §Security Anti-Patterns, Logging: `app.boot.engine` recorded as a deliberate NO-SCRUB log boundary (exact leaf `{program, interpretation, reason}`, three closed labels, no bare `app` key).
    rationale: >-
      The report's Changes touch a log boundary: Schema / config — "One allowlist leaf added (`app.boot.engine`: `program`, `interpretation`, `reason`)"; Symbols / APIs → "The boot record `app.boot.engine`" (once per `start`, closed field values, WARN on `none`, "its own exact allowlist leaf beside `app.boot.render.posture` ...; no bare `app` key exists"); Coverage — "PII: exact leaf, closed labels" and the four tests. The section states that values reaching the log sink pass the scrubber and then records each exact leaf whose fields stay unredacted as a deliberate NO-SCRUB boundary (`ui.ipc.rejection`, `ui.webgpu.adapter`, `app.exit`); the new leaf is absent from that record. The scrub coverage set, the redaction counter and every stated failure mode are untouched by this chunk, and the two restating sites (§Threat Model Summary → Data classification sensitivity note; §Data Protection → At rest) carry no claim this retires. The `app.exit` paragraph stands as written: the console program ends by signal through the existing listener and no label was added to the closed sets.
    basis: "pulse-app/src/observability.rs:1014-1021 (the leaf); pulse-app/src/engine_boot.rs:413-440 (`emit_boot_record`)"
```

**Disposition:** reject — check 1: playbook rule of 2026-08-25 (a D-security-logging proposal re-stating the scrubber posture for a bounded first-party label record: routine-REJECT, ground (c)) — the record class predates this chunk (`app.boot.render.posture`, `tray.signpost.shown` have the same shape and no such paragraph); the leaf is registered in obs-plan §6 and §8, which is its home

## design-system

verdict: `proposals: []` — no drift proposed.

## layout-templates

verdict: `proposals: []` — no drift proposed.

## test-plan

verdict: 14 proposal(s).

### test-plan · proposal 1

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Pending coverage triggers → exit-hook-main-composition-coverage'
    change: >-
      Append to the row: "**Narrowed 2026-10-10-console-engine-entry-point:** the process-start half left `pulse-app/src/main.rs` — the log sink and its guard, the at-exit hook and the Unix signal listener are now `engine_boot::init_process`, the one function both programs call (the window app and the console program `andromeda-pulse-engine`) — and it is witnessed on real processes: two re-exec arms of `pulse-app/tests/integration_engine_boot.rs` call `init_process` in a child (`the_process_start_keeps_the_panic_record_and_one_exit_record`: a panic on a thread leaves one ERROR `app.panic.fatal` and one `app.exit` of class `outside_event_loop`; `the_process_start_records_a_sigterm_and_the_process_still_ends_by_it`), `pulse-app/tests/integration_console_engine.rs` reads exactly one `app.exit` as the last record of the spawned console program ended by SIGTERM and by SIGINT, and `unit_engine_boot_seam` (`each_entry_point_calls_the_two_boot_functions_once`, mutation-checked) reddens when an entry point stops calling it or wires a part itself. Still owed: the window's own tail in `main.rs` — `.build(ctx)` + `run_return` + `exit_after_event_loop(code)`, the window's alone and still not inducible headless; the MAIN-thread panic case (the witnessed panic is on a spawned thread); and the composition inside the window's own `main` end to end, which only the boot smoke runs." The row's opening sentence then reads the production composition as split: process start in `engine_boot::init_process` (witnessed), event-loop tail in `main.rs` (not).
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 `exit-hook-main-composition-coverage` narrowed: the process start moved from `main.rs` to `engine_boot::init_process` and is witnessed in re-exec children and on the spawned console program; the window's event-loop tail and the main-thread panic case stay owed.
    rationale: >-
      The row says the production composition `hold_log_guard(init(..))` + `install_exit_hook()` + `install_signal_listener()` sits in `pulse-app/src/main.rs` and that nothing committed fails if `main` drops an install. The report's Changes move that composition into `engine_boot::init_process` ("the log sink and its guard, the at-exit hook, the Unix signal listener, the start instant, the pid file, in that order"), its Coverage row lists "tests integration, two re-exec arms (panic then exit; SIGTERM)", its Outcome reads the panic record and one `app.exit` on a child that calls `init_process`, and its Expected amendments name this trigger as narrowed ("`init_process` is the function both programs call and its panic, at-exit and signal records are witnessed in re-exec children … Not shown: the composition inside the window's own `main` end to end"). `exit_after_event_loop` "stays the window's tail alone".
    basis: 'pulse-app/src/engine_boot.rs:311-321'
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 2

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Pending coverage triggers → discovery-observer-wiring-coverage'
    change: >-
      Re-word the row's site and append a narrowing: the P-027 composition (`vec![baseline, restart, discovery]`, built after `lifecycle_registry`) is no longer made by `pulse-app/src/main.rs` but by `engine_boot::start` (`pulse-app/src/engine_boot.rs:739-743`), the one boot both programs call; "**Narrowed 2026-10-10-console-engine-entry-point:** the seam the row asked for exists and the PRODUCTION composition is run per test run — `pulse-app/tests/integration_engine_boot.rs` (`both_receivers_accept_and_a_span_lands_and_lists_its_service`) boots `engine_boot::start` in process, sends a service over gRPC and reads it listed in the lifecycle registry by the first-sighting adapter, the last of the three observers; `unit_engine_boot_seam` holds both entry points to that one boot (neither wires an engine part). `cargo xtask smoke:discovery` is no longer its only proof. Still owed: a recorded discrimination of the ORDER on the production composition (no mutation of the order was recorded at that chunk), and the composition inside the window's own `main` end to end, which only the boot smoke runs."
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 `discovery-observer-wiring-coverage` narrowed: the composite is built in `engine_boot::start` and its three-observer fan-out is asserted in process by `integration_engine_boot`; the order's discrimination on the production composition stays owed.
    rationale: >-
      The row states "nothing committed pins the production composition in `main.rs` itself" and owes "a seam lifting the composite's construction out of `main.rs`". The report's Changes put the composition in `engine_boot::start` (New text lists the `CompositeSpanObserver` construction there), and its Expected amendments name this trigger as narrowed: "`start` is the production composition and its three-observer fan-out is asserted in process (a service sent over gRPC is listed in the lifecycle registry by the first-sighting adapter, the last of the three)". The report records two mutations only (the seam pin's and the allowlist guard's), so the order half is kept as owed.
    basis: 'pulse-app/tests/integration_engine_boot.rs:279-317'
    dependent-of: D-tests-coverage
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 3

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Pending coverage triggers → damper-shared-instance-wiring-coverage'
    change: >-
      Replace "their sole production caller (`pulse-app/src/main.rs`) carries no test" with: their sole production caller is `engine_boot::start` (`pulse-app/src/engine_boot.rs`, since chunk 2026-10-10-console-engine-entry-point; it spawns the interpretation subscriber and the two L4 heartbeats only behind a seated runner, and neither entry point wires an engine part) — `pulse-app/tests/integration_engine_boot.rs` runs that caller in process with a seated canned runner (`a_seated_canned_runner_turns_a_cue_bearing_digest_into_one_incident`) but asserts nothing about the damper instance, so a second construction would still leave every test green. Owed unchanged: an assertion that the gate and the heartbeat observe the same instance.
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 `damper-shared-instance-wiring-coverage`: the sole production caller is `engine_boot::start`, not `main.rs`; it now runs in process under a seated runner with no shared-instance assertion, the owed pin unchanged.
    rationale: >-
      A second site of the claim the primary retires (the production engine wiring sits in `main.rs`). The report's Changes: "`start` spawns the interpretation subscriber and the two L4 heartbeats only behind a seated runner", `main.rs`'s "setup closure keeps only window work and the window's two ticks", and the seam pin holds that no entry point wires an engine part. The four in-process arms the report lists assert incident formation and the boot record, none the damper instance.
    basis: 'pulse-app/tests/integration_engine_boot.rs:319-358'
    dependent-of: D-tests-coverage
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 4

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Pending coverage triggers → viz-read-connection-router-wiring-coverage'
    change: >-
      In the parenthesis "(production `main.rs:…` are their only other constructors)" drop the two line ranges written after `main.rs` and name the file alone: "(production `pulse-app/src/main.rs` holds their only other constructors — the routers are still built in `main`, since chunk 2026-10-10-console-engine-entry-point from the handles `engine_boot::start` returns; the file went from 2054 to 649 lines at that chunk, so the ranges this row carried name no line of it)".
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 `viz-read-connection-router-wiring-coverage`: the two `main.rs` line ranges of the router constructors removed (the file is 649 lines, was 2054); the file is named without a line.
    rationale: >-
      The row places the production wiring at two line ranges of `main.rs`, both above 649. The report's Changes: "`main.rs` (2054 → 649 lines, basis `wc -l`)" and `main` "builds routers, plugin host, `VizState`, window geometry and the Tauri builder from the returned handles" — so the routers stay in `main.rs` but neither range can exist; the report gives no new line for the three constructors, hence the file without a line. The citation sweep of this run lists no row for this site.
    dependent-of: D-tests-coverage
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 5

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Pending coverage triggers → render-posture-main-placement-coverage'
    change: >-
      Append: "**Anchor moved 2026-10-10-console-engine-entry-point:** `main()` no longer initialises the log sink itself — after its two first statements and its runtime build it calls `engine_boot::init_process` (the sink, its guard, the exit hook, the signal listener, the pid file), then `window::emit_boot_spans()` and `render_posture::emit_posture(..)` — so the still-owed pin reads: `emit_posture` follows the `engine_boot::init_process` call exactly once. The posture record stays the window's alone: `pulse-app/tests/integration_console_engine.rs` asserts the spawned console program writes no `app.boot.render.posture` record."
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 `render-posture-main-placement-coverage`: the owed order pin's anchor is the `engine_boot::init_process` call (the sink init moved into it); the console program is asserted to emit no posture record.
    rationale: >-
      The row's still-owed half is anchored on `observability::init` inside `main()`. The report's Changes: `main` "then calls `engine_boot::init_process`, its own `window::emit_boot_spans()` and `render_posture::emit_posture(..)`", and `init_process` holds "the log sink and its guard"; Deviations: "The spawned-program test also asserts no `app.boot.render.posture` record". The first-two-statements half stays pinned by `unit_xlib_threads` ("unchanged and still pinned").
    basis: 'pulse-app/src/engine_boot.rs:311-321'
    dependent-of: D-tests-coverage
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 6

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → Per-chunk gate discipline'
    change: >-
      In "**Boot-smoke gate (conditional)**" add `pulse-app/src/engine_boot.rs` to the boot/setup path list beside `pulse-app/src/main.rs` — since chunk 2026-10-10-console-engine-entry-point it holds the process start (`init_process`) and the whole engine composition (`start`) that both programs boot through, moved out of `main.rs` — and state the console entry's witness: a chunk touching `pulse-app/src/console.rs` or `pulse-app/src/bin/andromeda-pulse-engine.rs` takes `pulse-app/tests/integration_console_engine.rs` (the spawned console program, inside the workspace run) as its runtime gate, since the window smoke boots `pulse-app` alone.
    sidecar: >-
      2026-10-10-console-engine-entry-point — §3 Per-chunk gate discipline, Boot-smoke gate: `pulse-app/src/engine_boot.rs` joins the boot/setup path list; the console entry's runtime gate is `integration_console_engine`.
    rationale: >-
      The trigger list names `pulse-app/src/main.rs` as the boot path. The report's Changes move the boot out of it: `engine_boot.rs` is new at 1253 lines ("The one engine boot both programs call"), `main.rs` shrank 2054 → 649, and "the engine's tasks now start before the Tauri builder instead of inside its setup closure". With the list unchanged a later chunk editing only `engine_boot.rs` would owe no runtime smoke although it edits both programs' boot. The report also reads "`scripts/agent-run.sh boot` still builds and spawns `pulse-app` alone" and names the console program's per-push run as `integration_console_engine`.
    basis: 'pulse-app/src/engine_boot.rs:462-1253'
    dependent-of: D-tests-coverage
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 7

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Coverage scope (testable entities) → pulse-app (Tauri binary crate)'
    change: >-
      Re-word the row's reason: the `pulse-app` package builds two programs over one engine boot (`engine_boot::init_process` + `engine_boot::start`, which wires every engine part) — the window app `pulse-app`, which adds routers, plugin host, tray and webview on the returned handles, and the console program `andromeda-pulse-engine`; neither entry point wires an engine part (`pulse-app/tests/unit_engine_boot_seam.rs`, eight source-text pins). Drive: the window app via the Tauri test harness as before; the shared boot in process (`pulse-app/tests/integration_engine_boot.rs`); the console program as a spawned binary (`pulse-app/tests/integration_console_engine.rs`).
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 Coverage scope, `pulse-app` row: two programs over one shared engine boot; the wiring is `engine_boot::start`'s, pinned by `unit_engine_boot_seam`, and driven in process and as a spawned console program.
    rationale: >-
      The row says the binary crate "Wires all library crates and webview". The report's Changes: a second `[[bin]]` (`pulse-app/Cargo.toml:19-23`), "The one engine boot both programs call", and the seam pin under which "neither wires an engine part itself" — the same retired claim (the window entry point holds the wiring) restated in the entity table.
    basis: 'pulse-app/Cargo.toml:19-23'
    dependent-of: D-tests-coverage
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 8

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Surfaces under test'
    change: >-
      Add a row: **console engine program (`andromeda-pulse-engine <run|version>`)** | Driver: the built binary spawned by `std::process::Command` from `pulse-app/tests/integration_console_engine.rs` (4 tests) under a cleared environment with no display variable and no session bus, its own TempDir data dir and two picked loopback ports; the command grammar through `pulse_app::console::parse` / `main` in `pulse-app/tests/unit_console_commands.rs` (9) | Signal: the exit status (`version` exits 0 with one line `andromeda-pulse-engine {version}`; no word, an unknown word, a second word or a non-Unicode word exits 2 with `usage: andromeda-pulse-engine <run|version>` on stderr and an empty data dir; `run` ends by the signal sent), TCP accept on both ports and refusal after the end, and the program's own `agent-latest.jsonl*` family (`app.boot.engine` once, `ingest.tick` / `buffer.tick`, never `viz.tick` / `plugins.tick`, one `app.exit` last, no record on stdout or stderr) | Boundary: single-surface (no window, no IPC, no model runner; the canned runner only when `ANDROMEDA_PULSE_L4_DETERMINISTIC` is truthy) | Notes: third product binary beside `pulse-app` and `andromeda-pulse-mcp`, since chunk 2026-10-10-console-engine-entry-point; no `stop` command; no harness verb boots it; it runs inside `cargo nextest run --workspace` on every push.
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 Surfaces under test: row added for the console engine program `andromeda-pulse-engine` (spawned-binary driver, exit status + ports + log family as signal).
    rationale: >-
      The chunk adds a surface with its own driver and signal that the plan's inventory does not hold: the report's Changes list "A second product binary, `andromeda-pulse-engine`" and its closed command grammar, and Coverage of new surfaces lists "tests unit (`unit_console_commands`, 9) + integration (spawned binary, exit codes and empty data dir)" and "tests integration (`integration_console_engine`, 4)". `andromeda-pulse-engine` reads 0 lines in the seven masters (report, Expected amendments). The tests exist at the tier §2 asks; the drift is the inventory omitting the surface.
    basis: 'pulse-app/tests/integration_console_engine.rs:320-462'
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 9

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → Per-chunk gate discipline'
    change: >-
      Add, after "**Process-end witness form (integration tier)**": "**Spawned-program witness form (integration tier).** A claim about a product program's whole process — it boots with no display, both receivers accept, telemetry lands, its log is whole JSON, it ends by a signal leaving one `app.exit` — is witnessed by a `pulse-app/tests/integration_*.rs` test that spawns the BUILT product binary (first use: `andromeda-pulse-engine run`, `pulse-app/tests/integration_console_engine.rs`, chunk 2026-10-10-console-engine-entry-point): a cleared environment with no display variable and no session bus; its own TempDir data dir and two picked loopback ports passed through the two registered port variables; the pid from `Child::id()`, never a shell's background pid, and a still-running child killed on drop; readiness read from both ports accepting and records awaited by target under a bound, no sleep for a count; the end by SIGTERM or SIGINT with the signal read from the exit status; the log family read after the end — every line a whole record, exactly one `app.exit` and it is last, both ports refusing, no log record on stdout or stderr, the data dir's top-level entries within `corpus`, `logs`, `run`. It runs inside `cargo nextest run --workspace` (`cargo xtask test` in `lint / test`, and the pre-push `test` stage), so the console program is run on every push with no workflow step and no harness verb of its own." And in the Process-end witness form note that the re-exec form now also has two arms over the production process start: the children of `pulse-app/tests/integration_engine_boot.rs` call `engine_boot::init_process` (the sink, the hooks and the pid file together), not `observability::init` alone.
    sidecar: >-
      2026-10-10-console-engine-entry-point — §3 Per-chunk gate discipline: the spawned-program witness form recorded (first use `integration_console_engine`); the process-end form gains two re-exec arms over `engine_boot::init_process`.
    rationale: >-
      The report's Expected amendments name "§3 (the spawned-program witness as a form)". The form is new this chunk: the Coverage row "tests integration (`integration_console_engine`, 4)", the Harness / gate surface bullet ("the six new test binaries run inside the existing `cargo xtask test` step … both with no display variable and no session bus, so the console program is run by a check on every push with no workflow edit"), and Decisions ("The tests take the pid from `Child::id()` and kill a still-running child on drop", written after a hand run whose shell background pid named a subshell). §3 today names the re-exec form only, whose child "calls `observability::init`"; the new re-exec arms call `init_process`.
    basis: 'pulse-app/tests/integration_console_engine.rs:57-160'
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 10

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Pending coverage triggers'
    change: >-
      Add a row `engine-boot-rejected-port-and-socket-census-coverage`: "**NEW 2026-10-10-console-engine-entry-point.** `engine_boot::start` runs under four in-process arms (`pulse-app/tests/integration_engine_boot.rs`) and the console program under a spawned run (`pulse-app/tests/integration_console_engine.rs`), and three readings of the new boot are not committed. (1) The rejected-port arm: `EngineConfig` holds `Option<OtlpPort>` per receiver, and a `None` port records the bind as failed with `reason = "invalid_port"` and starts no receiver; the resolvers that return the rejection are pinned (`pulse-app/tests/unit_engine_boot_env.rs`, 14), but no test drives `start` or the console program with a rejected port, so nothing committed fails if that arm binds anyway. (2) That no third socket listens is by construction (the shared boot binds two) and was not measured from the process's socket list. (3) The corpus-key lock file's placement outside the data dir is not asserted by the spawned run. Owed: an in-process or spawned arm with a rejected port variable reading the failed-bind record and a refusing port; a socket census on the spawned run; an assertion on the lock file's directory."
    sidecar: >-
      2026-10-10-console-engine-entry-point — §1 Pending coverage triggers: new row `engine-boot-rejected-port-and-socket-census-coverage` (the `None`-port arm of `engine_boot::start`, the no-third-socket reading, the lock file's placement — none committed).
    rationale: >-
      A new path with no test at the integration tier §2 requires. The report's Changes describe the arm ("a `None` port (a rejected variable) records the bind as failed with `reason = "invalid_port"` and starts no receiver"); its Coverage row for `engine_boot::start` lists exactly four arms ("composition; seat `deterministic`; seat none; program `window`"), and the New text section, which lists every added test fn of the six new test files, names none with a rejected port. Readings (2) and (3) are the report's own Outcome limits ("that no THIRD socket listens is by construction … was not measured"; "the lock file's placement outside the data dir was not asserted by the test"). The two-engines-on-one-data-dir case ("Not exercised by any test") is left to the card, where the report sends it.
    basis: 'pulse-app/src/engine_boot.rs:356-363'
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; a new pending-coverage row recording three readings the chunk's own report states as not made

### test-plan · proposal 11

```yaml
  - detector: D-tests-framework
    severity: warning
    section: '§2 Test Strategy → Test directory + naming conventions → Directory pattern'
    change: >-
      Re-word the exemption: "**`main.rs` is exempt**: it is a `[[bin]]` target — the `pulse-app` package has two since chunk 2026-10-10-console-engine-entry-point, `src/main.rs` and `src/bin/andromeda-pulse-engine.rs` (three lines, no test) — whose tests genuinely execute; since that chunk it holds one, `emit_taurpc_bindings`: the 14 resolver tests left its `mod tests` for `pulse-app/tests/unit_engine_boot_env.rs` under the same names when the resolvers moved to `pulse_app::engine_boot` as `pub` + `#[doc(hidden)]` items."
    sidecar: >-
      2026-10-10-console-engine-entry-point — §2 Directory pattern: `main.rs` is one of two `[[bin]]` targets and keeps one test (`emit_taurpc_bindings`); the 14 resolver tests run from `pulse-app/tests/unit_engine_boot_env.rs`.
    rationale: >-
      The chunk's runner and framework match the plan (nextest, `cargo xtask test`, `--no-tests=fail`); what moved is the carrier the convention describes. §2 calls `main.rs` "the `[[bin]]` target, whose tests genuinely execute (e.g. `emit_taurpc_bindings`)". The report's Changes: "`[[bin]]` targets of `pulse-app` 1 → 2", and "The 14 resolver tests moved from `main.rs`'s `mod tests` (the `[[bin]]` test binary) to `pulse-app/tests/unit_engine_boot_env.rs` under the same 14 names; `main.rs`'s `mod tests` keeps one test, `emit_taurpc_bindings`". The ratchet's reach over `src/bin/` is not proposed here: the report lists it as found-and-not-owned, for the card.
    basis: 'pulse-app/Cargo.toml:19-23'
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 12

```yaml
  - detector: D-tests-framework
    severity: warning
    section: '§4 Unit Test Strategy → Conventions → Test file location (Rust)'
    change: >-
      In the bullet's closing parenthesis re-word "`main.rs` the `[[bin]]` exemption" to "`main.rs` a `[[bin]]` exemption holding one test, `emit_taurpc_bindings`, since chunk 2026-10-10-console-engine-entry-point (the package's second `[[bin]]`, `src/bin/andromeda-pulse-engine.rs`, carries none)".
    sidecar: >-
      2026-10-10-console-engine-entry-point — §4 Test file location: the `main.rs` exemption re-worded for two `[[bin]]` targets and the one test `main.rs` keeps.
    rationale: >-
      The same singular-`[[bin]]` claim as §2's Directory pattern, restated at the end of this bullet; same report facts ("`[[bin]]` targets of `pulse-app` 1 → 2"; `main.rs`'s `mod tests` keeps one test).
    basis: 'pulse-app/src/bin/andromeda-pulse-engine.rs:1-3'
    dependent-of: D-tests-framework
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 13

```yaml
  - detector: D-tests-framework
    severity: warning
    section: '§4 Unit Test Strategy → Conventions → Test file location (Rust)'
    change: >-
      Lead the bullet's dated count chain with the current reading: "109 top-level `pulse-app/tests/*.rs` files as of chunk 2026-10-10-console-engine-entry-point — the six new ones `unit_engine_boot_env.rs`, `unit_console_commands.rs`, `unit_observability_allowlist_boot_engine.rs`, `unit_engine_boot_seam.rs`, `integration_engine_boot.rs`, `integration_console_engine.rs` — 103 at its base", keeping the existing "102 … as of chunk 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings" chain behind it as history.
    sidecar: >-
      2026-10-10-console-engine-entry-point — §4 Test file location: `pulse-app/tests/*.rs` count 103 → 109 (six new test files named).
    rationale: >-
      The report's Counts / qualifiers moved: "`pulse-app/tests/*.rs` files 103 → 109 (basis: `ls pulse-app/tests/*.rs | wc -l` = 109; `git ls-tree --name-only f31027a4 pulse-app/tests/` = 103)", and its Expected amendments name "test-plan §2 and §4 (the `pulse-app/tests` count, 103 → 109)". The plan's own newest reading is 102 at an earlier chunk; the literal 103 stands only in the leaf `.claude/rules/testing.md:25`, which the cascade re-derives from this bullet. §2's Directory pattern states no count, so the count lands at this one site.
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### test-plan · proposal 14

```yaml
  - detector: D-tests-obs-harness
    severity: warning
    section: '§3 → PID file'
    change: >-
      Add to Lifecycle: "the writer is `write_pid_file` of the shared boot (`pulse_app::engine_boot`, called through `engine_boot::init_process`), and since chunk 2026-10-10-console-engine-entry-point BOTH product programs call it — the window app `pulse-app` and the console program `andromeda-pulse-engine run` — at the same path, so the file holds the pid of whichever program started last on that data dir. The harness verbs are not aimed at the console program although it writes the file: `boot` builds and spawns `pulse-app` alone, and `status` / `cleanup` read whatever pid the file holds." In Location read "the app's `write_pid_file`" as "the shared boot's `write_pid_file`".
    sidecar: >-
      2026-10-10-console-engine-entry-point — §3 PID file: second writer recorded (the console program, through the shared `engine_boot::write_pid_file`); the harness verbs stay aimed at `pulse-app` alone.
    rationale: >-
      No verb, verdict shape, status endpoint or log format changed ("Harness / gate surface: none changed"; "No IPC change"; the console program uses "the same log sink, identity fields"), so the three-way shape agreement with obs-plan §3 holds. One harness fact did move and §3 names one writer for it: the report's Changes say "The pid file and the workspace key are now written by BOTH programs, at the same paths", `write_pid_file` moved to `engine_boot`, and the Harness bullet reads "the harness verbs that read `run/andromeda-pulse.pid` are not aimed at the console program although it writes that file". obs-plan §3 Tracing init is due the console program's process-start order (report, Expected amendments), which includes the pid file step; test-plan §3 staying silent would leave the pid file's writer described on one side only.
    basis: 'pulse-app/src/engine_boot.rs:199-241'
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

## obs-plan

verdict: 17 proposal(s).

### obs-plan · proposal 1

```yaml
  - detector: D-obs-stack
    severity: warning
    section: "§3 → Tracing init"
    change: >-
      Init order names the process start both programs call: `engine_boot::init_process(data_dir)`, entered with the tokio runtime, does in this order the log sink and its guard, the at-exit hook, the Unix SIGTERM/SIGINT listener, the start instant and the pid file; the steps this label gave to `main` (parking the `WorkerGuard`, the at-exit hook, the signal listener) are that function's, and the window app (`pulse-app`) and the console program (`andromeda-pulse-engine run`) each call it once. Step (4) reads: `engine_boot::start` spawns the engine tasks and binds the two OTLP receivers — in the window app before the Tauri builder (was inside its setup closure), after `main`'s own window boot records and render posture; the window app alone then runs the Tauri app through `App::run_return` → `exit_after_event_loop`. The console program spawns no Tauri app and has no event loop: it ends by SIGTERM or SIGINT through the same listener (`app.exit` class `signal`, the signal re-raised), and it has the one file layer — no stderr or stdout log layer.
    sidecar: >-
      §3 Tracing init — was `main` parks the guard, installs the exit hook and the signal listener, then "Spawn Tauri app + bind OTLP receivers" with `run_return` → `exit_after_event_loop` as the end; now `engine_boot::init_process` is the process start both programs call, `engine_boot::start` binds the receivers before the window's Tauri builder, and the console program has no event loop and ends by signal.
    rationale: >-
      Report Changes → Symbols / APIs: "The one engine boot both programs call" (`init_process`: sink and guard, at-exit hook, signal listener, start instant, pid file, in that order, called with the runtime entered), "`main.rs`" (calls `init_process`, then its own window boot spans and posture, then `start`), "Order changes in the window app" (engine tasks start before the Tauri builder instead of inside its setup closure) and "Process end" (no event loop; `exit_after_event_loop` stays the window's tail alone). Expected amendments names this key. The stack itself is unchanged — Dependencies "none added, none bumped", and a stderr or stdout log layer in the console program is listed as deliberately not built — so the drift is the actor and the order, not the library. The report names the steps `init_process` performs, not the functions `hold_log_guard` / `install_exit_hook` / `install_signal_listener`; whether those three names still stand inside it is one read of the cited range.
    basis: pulse-app/src/engine_boot.rs:311-321
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5 (a keyed contract: its key file is edited)

### obs-plan · proposal 2

```yaml
  - detector: D-obs-stack
    severity: warning
    section: "§1 Obs Scope Summary → Observability harness specification → Tracing init (obs-plan:72)"
    change: >-
      The same restatement: "After init, `main` parks the returned `WorkerGuard` … installs the at-exit hook … the SIGTERM/SIGINT listener" becomes the process start `engine_boot::init_process`, called once by each program (sink and guard, at-exit hook, Unix signal listener, start instant, pid file, in that order); "(4) spawn Tauri app + bind OTLP receivers" becomes `engine_boot::start` binding the receivers, then — window app only — the Tauri app; the `run_return` → `exit_after_event_loop` sentence is the window app's, and the console program (`andromeda-pulse-engine run`) has no event loop and ends by SIGTERM or SIGINT through the listener with one `app.exit` of class `signal`.
    sidecar: >-
      §1 Tracing init — the `main`-parks-the-guard sentence and step (4) restated for two programs over `engine_boot::init_process` / `engine_boot::start`; the event-loop end is the window app's alone.
    rationale: >-
      Second occurrence of the claim the §3 → Tracing init change retires: the report's own search reads `install_signal_listener|install_exit_hook|hold_log_guard` in obs-plan on 1 line, and names `Tracing init` at obs-plan:72 beside the key file. A key-only apply leaves the body naming `main` as the actor and the event loop as the only end.
    basis: pulse-app/src/engine_boot.rs:311-321
    dependent-of: D-obs-stack
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 3

```yaml
  - detector: D-obs-stack
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry surfaces (the CLI row)"
    change: >-
      The row gains the second command-line entry point: the console program `andromeda-pulse-engine` (a second `[[bin]]` of `pulse-app`), grammar closed at `run` | `version`. `run` goes through `engine_boot::init_process` and `engine_boot::start`, so its records ride the same single file sink with the same identity fields, and no log record reaches stderr or stdout. `version` prints one line and exits 0; no word, an unknown word, a second word or a non-Unicode word prints the usage line on stderr and exits 2. Neither writes a record or creates the data dir (no sink exists at that point), so "structured `tracing::error!` on startup failure" does not cover a refused command line.
    sidecar: >-
      §1 Telemetry surfaces, CLI row — was one command-line entry whose startup failures log a structured error; now also the console program `andromeda-pulse-engine` (`run` | `version`): `run` logs through the shared process start, `version` and a refused command line write no record by design.
    rationale: >-
      Report Changes → "A second product binary" and "The console program's command grammar" (refusal prints usage on stderr, exits 2, "creating nothing"); Coverage row for the command line: "instrumentation n/a (a refused command line writes no record by design)"; Outcome: one file sink, every line whole JSON with the identity fields, no log record on stderr, stdout checked too. The row states the command-line surface as one binary and promises a structured error on startup failure; the second entry point's refusal path is the measured exception.
    basis: pulse-app/src/console.rs:48-65
    dependent-of: D-obs-stack
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 4

```yaml
  - detector: D-obs-stack
    severity: warning
    section: "§4 Span / Trace Coverage → Instrumentation per surface (the CLI / main binary row)"
    change: >-
      The row covers two entries: the window app's `main` and the console program's (`andromeda-pulse-engine`, whose `main` hands its arguments to `pulse_app::console::main`). Both call `engine_boot::init_process` then `engine_boot::start`, so the console program's `run` carries `app.boot.tracing.init`, `app.boot.pid`, the panic and process-end records (`app.panic.fatal`, `app.exit`), `app.boot.engine` and every record the shared engine wiring emits, and none of the window's boot records; `version` and a refused command line emit nothing.
    sidecar: >-
      §4 CLI / main binary row — was one entry (`main`); now two entries over the shared `engine_boot::init_process` / `engine_boot::start`, with the console program's `version` and refused command line emitting no record.
    rationale: >-
      The §4 site of the same surface the §1 CLI row describes. Report Coverage rows: `engine_boot::init_process` → `app.boot.tracing.init`, `app.boot.pid`, `app.panic.fatal`, `app.exit`; `engine_boot::start` → `app.boot.engine` plus every record the moved wiring already emitted; the command line → no record on refusal. The row's "Root span `app.boot` wraps main()" is not touched: the report measures nothing about it for either program.
    basis: pulse-app/src/console.rs:67-97
    dependent-of: D-obs-stack
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 5

```yaml
  - detector: D-obs-stack
    severity: warning
    section: "§9 CI Integration → Pipeline integration (the `xtask test` row)"
    change: >-
      "the job boots no app" becomes: the job boots no window app; since chunk 2026-10-10-console-engine-entry-point one test binary of the run, `integration_console_engine`, spawns the console program (`andromeda-pulse-engine run`) with no display variable and no session bus, and the program writes its log family under the data dir the test gives it. No step uploads that log (no workflow step changed at that chunk); the one app log a run keeps is still the `boot` job's.
    sidecar: >-
      §9 Pipeline integration, `xtask test` row — was "the job boots no app"; now it boots no window app, and a test of the run spawns the console program, whose log family lands under the test's own data dir and is uploaded by no step.
    rationale: >-
      Report Changes → Harness / gate surface: the six new test binaries run inside the existing `cargo xtask test` step of `lint / test`, "so the console program is run by a check on every push with no workflow edit"; Decisions: "two programs are run on every push — … the console by `integration_console_engine` in the workspace run". The row's present-tense "boots no app" is the single-program claim restated for CI. The row's dated reading on `ci#38031822696` stays as a reading of that run. Worded so that nothing reads as the engine's log being kept in CI: §9's Snapshot markdown row already names the route entry that owns that.
    basis: pulse-app/tests/integration_console_engine.rs:320-462
    dependent-of: D-obs-stack
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 6

```yaml
  - detector: D-obs-stack
    severity: warning
    section: "§9 CI Integration → CI failure → artifact triage workflow (the first bullet)"
    change: >-
      "a failed test in `lint-test` uploads no app log — that job writes none" becomes: a failed test in `lint-test` uploads no app log — the job writes none under an uploaded path; the console program's log family that `integration_console_engine` produces sits under the test's own data dir and no step uploads it, so a failure of that test leaves its log on the runner only.
    sidecar: >-
      §9 triage workflow — "that job writes none" narrowed: a test of the job now spawns the console program, whose log is written under the test's data dir and not uploaded.
    rationale: >-
      Same claim as the `xtask test` row, restated in the triage list. Report: `integration_console_engine` reads the spawned program's log family from the data dir it passed (Files the console program writes, "asserted in `integration_console_engine`"); Harness / gate surface: no CI step changed, so no upload was added.
    basis: pulse-app/tests/integration_console_engine.rs
    dependent-of: D-obs-stack
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 7

```yaml
  - detector: D-obs-stack
    severity: warning
    section: "§3 → Heartbeat ticks (the Stall detection label)"
    change: >-
      (a) LIVENESS reads "agent reads recent ticks via tail of the JSON file"; the words "or CLI `logs` command" go — no program has a `logs` command: the console program's grammar is closed at `run` | `version`, and any other word exits 2 with the usage line.
    sidecar: >-
      §3 Heartbeat ticks, Stall detection — "or CLI `logs` command" removed: the console program built at chunk 2026-10-10-console-engine-entry-point has a closed two-word grammar and refuses any other word.
    rationale: >-
      Report Changes → "The console program's command grammar": closed, two words, an unknown word prints usage and exits 2; "No `stop` command exists". The wording predates this chunk and named a command no program had; this chunk built the command line and closed its grammar, so the label now names a command the product refuses. One occurrence, in the key file only (0 in the obs-plan body). Separate claim from the tick-roster change on the same key.
    basis: pulse-app/src/console.rs:25-42
```

**Disposition:** reject — check 4 (absence needs evidence): the claim that no program has a `logs` command is false as to the harness, whose `scripts/agent-run.sh` carries a `logs` verb (the five-command harness: boot, run, status, cleanup, logs); the label names that verb, not a product command

### obs-plan · proposal 8

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 → Heartbeat ticks (the Tick interval label)"
    change: >-
      The 15 s ticks are spawned in two groups: the engine's three (`ingest.tick`, `buffer.tick`, `connection.tick`) by `heartbeat::spawn_engine_ticks`, called once from `engine_boot::start` and so emitted by both programs; the window's two (`viz.tick`, `plugins.tick`) by `heartbeat::spawn_window_ticks`, called once from the window app's setup closure. The window app emits all five; the console program (`andromeda-pulse-engine run`) emits the three and never `viz.tick` or `plugins.tick`, so in a log whose `app.boot.engine` record reads `program: console` their absence is by design and not a stall. `heartbeat::spawn`, the one function that returned five handles, no longer exists.
    sidecar: >-
      §3 Heartbeat ticks — was one roster "(ingest, buffer, viz, plugins)" for the app; now two groups per program: the engine's three ticks in both programs (`spawn_engine_ticks`), the window's two in the window app alone (`spawn_window_ticks`).
    rationale: >-
      Report Changes → "`heartbeat::spawn` no longer exists" (three handles and two handles, their one production caller each; "The window app still emits all five ticks; the console program emits three and never `viz.tick` or `plugins.tick`"); Outcome: two `ingest.tick` and two `buffer.tick` at most 45 s apart, no `viz.tick` or `plugins.tick`, on the spawned run. Expected amendments names this key. The claim retired here and at its restatements: a record only the window app emits is stated as every process's. Instrumentation itself is present on every new operation (Coverage rows all tick the instrumentation cell; nothing new is on a hot path). `connection.tick` is named because the split's two lists name it; the key had not listed it before this chunk and the report states no field set for it, so none is proposed — a pre-existing gap, not this chunk's.
    basis: pulse-app/src/engine_boot.rs:1219-1228
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes; check 5 (a keyed contract: its key file is edited)

### obs-plan · proposal 9

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Observability harness specification → Heartbeat ticks (obs-plan:101)"
    change: >-
      The tick list states which program emits which: `ingest.tick` and `buffer.tick` (with `connection.tick`) come from the shared engine boot and are in both programs' logs; `viz.tick` and `plugins.tick` are the window app's alone, and the console program never emits them. LIVENESS's "missing tick for >45s" reads the ticks the program emits.
    sidecar: >-
      §1 Heartbeat ticks — the four-target list now says which program emits which: the engine's ticks in both, `viz.tick` / `plugins.tick` in the window app alone.
    rationale: >-
      Restatement of the roster the §3 → Heartbeat ticks change corrects; the report names `Heartbeat ticks` at obs-plan:101 and reads `viz\.tick` on 2 obs-plan lines. The field lists and the two-signal stall text are unchanged (Coverage: "the five existing tick targets, unchanged fields").
    basis: pulse-app/src/heartbeat.rs:120-131
    dependent-of: D-obs-instrumentation
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 10

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Standard+ invariants → Heartbeat ticks (obs-plan:536)"
    change: >-
      "every long-running task (ingest, buffer, viz, plugins) emits tick every 15s" becomes: the engine's ticks (ingest, buffer, connection) every 15 s in both programs, and the window's (viz, plugins) in the window app; in the console program the absence of the window's two is by design, not a stall. The dated 2026-08-26 reading ("all four ticks emitting normally") stays as written.
    sidecar: >-
      §10 Standard+ invariants, Heartbeat ticks — the "every long-running task (ingest, buffer, viz, plugins)" roster made per program; the 2026-08-26 measurement untouched.
    rationale: >-
      Restatement of the same roster; the report names `Heartbeat ticks` at obs-plan:536. The progress invariant (`rows_ingested_delta`, `buffer.consumer.stalled`) is the engine's and holds for both programs unchanged — the spawned run read `rows_ingested_delta` rendered.
    basis: pulse-app/src/main.rs:379-382
    dependent-of: D-obs-instrumentation
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 11

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → CI gates (the heartbeat-stall bullet — its target list)"
    change: >-
      The stall rule's target list is per program: a >45 s gap between consecutive ticks of `ingest.tick` / `buffer.tick` in either program, and of `viz.tick` / `plugins.tick` in the window app only — a console program's log holds neither of the last two.
    sidecar: >-
      §10 CI gates, heartbeat bullet — "for any of `ingest.tick` / `buffer.tick` / `viz.tick` / `plugins.tick`" made per program: the last two are the window app's alone.
    rationale: >-
      The second of the 2 obs-plan lines the report reads for `viz\.tick`: the bullet lists the four targets as one process's. Roster only; the bullet's enforcement status is the separate D-obs-defect-narrative proposal on the same bullet.
    basis: pulse-app/tests/integration_console_engine.rs:410-420
    dependent-of: D-obs-instrumentation
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 12

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§6 Log Coverage → Log levels mapping (the `warn` row — `app.boot.render.posture` and `interpretation.model.allow_root`)"
    change: >-
      The two "EXACTLY ONCE per boot" records that only the window app emits say so: `app.boot.render.posture` is emitted once per window-app boot, by `main` after `engine_boot::init_process`, and a console program's log holds none (asserted on the spawned run); `interpretation.model.allow_root` is once per window-app boot — the console program reads no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, constructs no `LlamaCliInference` and writes no `interpretation.model.*` record.
    sidecar: >-
      §6 warn row — `app.boot.render.posture` and `interpretation.model.allow_root` were "EXACTLY ONCE per boot"; now once per window-app boot, with the console program emitting neither.
    rationale: >-
      Same retired claim as the tick roster, in the once-per-boot records: a record only the window app emits is stated as every boot's. Report Changes → "`main.rs`" (`render_posture::emit_posture(..)` is `main`'s own call) and "Environment variables" (the console program reads no allow-root variable and constructs no `LlamaCliInference`); Deviations → Test arms: the spawned-program test asserts no `app.boot.render.posture` record; Outcome: no `interpretation.model.*` record. Read as written, §6 would make a console boot's log look two records short.
    basis: pulse-app/tests/integration_console_engine.rs
    dependent-of: D-obs-instrumentation
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 13

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§8 PII Scrubbing → the two `interpretation.*` leaves of chunk 2026-08-26-l4-runtime-security-residuals (`interpretation.model.allow_root`)"
    change: >-
      "Fires exactly once per boot." becomes: fires exactly once per boot of the window app; the console program emits no `interpretation.model.*` record.
    sidecar: >-
      §8 `interpretation.model.allow_root` — "exactly once per boot" scoped to the window app; the console program emits no `interpretation.model.*` record.
    rationale: >-
      The §8 restatement of the once-per-boot frequency the §6 warn row carries for the same target. Report Outcome (inputs#I1 item 5): no model variable, no runner, no probe; no `interpretation.model.*` record — MET.
    basis: pulse-app/src/console.rs:78-83
    dependent-of: D-obs-instrumentation
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 14

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry triggers (the multi-platform-exporter-compat row)"
    change: >-
      The platform boot records of the row (`app.boot.webview.init`, `app.boot.gpu.check`, `app.boot.tray.init`) are the window app's — `main` emits them through its own `window::emit_boot_spans()`, not the shared boot — so "Assert all platform-specific spans present in expected platform's logs" reads: in the window app's log on that platform; the console program emits none of them.
    sidecar: >-
      §1 Telemetry triggers, multi-platform row — the platform boot records scoped to the window app; the console program emits none.
    rationale: >-
      Same retired claim in the boot-record form. Report Changes → "`main.rs`" (`main` calls its own `window::emit_boot_spans()`) and "Order changes" ("the three window boot records" stay in `main`'s order); the console entry is pinned to name no window start (`the_console_program_names_no_model_no_probe_and_no_window_start`). Limit: the report calls them "the three window boot records" without listing the targets, and their absence from the console log rests on that source pin — the log assertion on the spawned run names `app.boot.render.posture` only.
    basis: pulse-app/tests/unit_engine_boot_seam.rs:133-144
    dependent-of: D-obs-instrumentation
```

**Disposition:** apply — check 1: playbook `Accurate this-chunk addition` (routine); the named thing is in the report's Changes

### obs-plan · proposal 15

```yaml
  - detector: D-obs-pii
    severity: escalate
    section: "§8 PII Scrubbing → the exact-leaf list (a new entry beside `app.boot.render.posture`)"
    change: >-
      §8 gains `app.boot.engine`: `program`, `interpretation`, `reason` — ALL THREE fields the emit site (`engine_boot::emit_boot_record`) emits (chunk 2026-10-10-console-engine-entry-point), each a closed label: `program` `window` | `console`; `interpretation` `model` | `deterministic` | `none`; `reason` `window_runs_model` | `deterministic_gate_set` | `deterministic_gate_unset`. Never a path or a variable's value — the WARN's message names the variable `ANDROMEDA_PULSE_L4_DETERMINISTIC` by name only. An EXACT leaf beside `app.boot.render.posture` under the same no-bare-`app` invariant, so without it all three fields would redact. Asserted by `pulse-app/tests/unit_observability_allowlist_boot_engine.rs` (4 tests: exact-resolve, set equality both ways, the no-bare-`app` discriminator, an emit-site capture for the four program and seat pairs at each one's level; mutation-checked), under `pulse-app/tests/` per the `[lib] test = false` rule. Wire-read: once in each of the eight boot logs of `ci#38056942758`'s artifact, `"program":"window"` rendered, and once on the spawned console run.
    sidecar: >-
      §8 gains the exact leaf `app.boot.engine {program, interpretation, reason}` — three closed labels, every field the emit site emits, no bare `app` key, guarded ×4 in `pulse-app/tests/unit_observability_allowlist_boot_engine.rs` (mutation-checked).
    rationale: >-
      Actual class: registry completeness, not a PII hole — the escalate condition (raw user input or PII reaching the log) is affirmatively absent in the report. Report Changes → "The boot record `app.boot.engine`" and Schema / config ("One allowlist leaf added"); Coverage rows: `engine_boot::start` — "PII: three closed labels, no path, no variable value"; log target `app.boot.engine` — "exact leaf, closed labels", 4 tests, mutation-checked. Both conditions of the 2026-08-16 leaf-registration rule hold as the report states them: (a) the leaf enumerates every field the emit site emits (set equality both ways plus the emit-site capture), (b) the guard lives in `pulse-app/tests/` and ran (it is one of the six new test binaries of the green workspace run). The report's own search reads `app\.boot\.engine` on 0 lines everywhere, so §8 lists the default-deny allowlist one leaf short. The rest of the chunk's new logging touches no user data: no full data-dir path and no passphrase value in the spawned run's log, and the panic payload never reaches the log (canary asserted).
    basis: pulse-app/src/observability.rs:1014-1021
```

**Disposition:** apply — check 1: playbook rule of 2026-08-16 (a D-obs-pii registration of a new target with bounded non-PII fields: routine); both load-bearing conditions hold in the report — (a) the leaf holds every field the emit site emits (set equality both ways and an emit-site capture), (b) the guard is under `pulse-app/tests/` and ran; check 5

### obs-plan · proposal 16

```yaml
  - detector: D-obs-pii
    severity: escalate
    section: "§6 Log Coverage → Log levels mapping (the `warn` row — a new `app.boot.engine` entry)"
    change: >-
      The `warn` row gains `warn!(target: "app.boot.engine", program, interpretation, reason)` — EXACTLY ONCE per `engine_boot::start`, so once per boot of either program, off any hot path per §11: WARN when `interpretation` is `none` (the console program with `ANDROMEDA_PULSE_L4_DETERMINISTIC` unset seats no runner — cues and digests are produced and no incident forms), INFO for `model` and `deterministic`. It is the one record that names which program booted and what sits in the interpretation seat; all three fields are closed labels (§8).
    sidecar: >-
      §6 warn row gains `app.boot.engine {program, interpretation, reason}` — once per boot of either program, WARN when the interpretation seat is empty, INFO otherwise; registered dual-site with its §8 leaf.
    rationale: >-
      The duplicate-occurrence half of the §8 registration: a once-per-boot WARN is registered in both §6 and §8, and a §8-only apply leaves §6 without the record. Report Changes → "The boot record `app.boot.engine`" (emitted exactly once per `start`; WARN when `interpretation` is `none`, INFO otherwise) and "The interpretation seat" (with no seat, cues and digests are produced, no incident forms); Outcome: once per boot behind its own exact leaf — guard, in-process arms, spawned run, each of the eight CI boot logs. Same actual class as the primary: registry completeness, no PII.
    basis: pulse-app/src/engine_boot.rs:413-440
    dependent-of: D-obs-pii
```

**Disposition:** apply — the duplicate-occurrence half of proposal 15 (the same rule: a once-per-boot WARN is registered in §6 and §8); check 5

### obs-plan · proposal 17

```yaml
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → CI gates (the heartbeat-stall bullet — its enforcement status)"
    change: >-
      "since chunk 2026-10-10-no-gate-stands-while-reading-nothing NO CI step makes the gap check" is narrowed: since chunk 2026-10-10-console-engine-entry-point one test of the `lint / test` job's `cargo xtask test` step, `integration_console_engine`, spawns the console program and asserts two `ingest.tick` and two `buffer.tick` records at most 45 s apart (green on `ci#38056942758`) — one consecutive pair per target, the engine's two targets, the console program only. Still made by no CI step: the gap check over the window app's log, over `viz.tick` / `plugins.tick`, and over more than one pair per target (`heartbeat-gap-check` stays called by `perf:load-profiles` alone); the carry on the working route stands for that remainder.
    sidecar: >-
      §10 CI gates, heartbeat bullet — "NO CI step makes the gap check" narrowed: a test of the `lint / test` job now asserts one consecutive ≤ 45 s pair of `ingest.tick` and of `buffer.tick` on a spawned console program; the window app's ticks and the full gap check stay unenforced on CI and carried.
    rationale: >-
      A status the bullet states as current, changed by what the chunk measured with no new symbol behind it. Report Outcome: "(obs) two `ingest.tick` and two `buffer.tick` at most 45 s apart … no `viz.tick` or `plugins.tick` — MET", and `integration_console_engine` passes in `lint / test` of `ci#38056942758`; Harness / gate surface: the new test binaries run inside the existing `cargo xtask test` step on every push. The bullet's own account of why the former `ci-gates` arm could not fail — one tick per target, no consecutive pair — is the contrast: this test holds a pair. Proposed as a narrowing, not a discharge: the report does not say the working-route carry is closed, and the wrap directive's "four carries on their owning entries" are not listed in it, so the owner pointer is the operator's to confirm. No other §10 narrative is touched — "Spec claims disproved by measurement: none", and the four DuckDB defects and their lead-in tally are unaffected. The zero-spans bullet is left alone: its unmet reading is a generic gate over a test run, which this one test's record assertions do not make.
    basis: pulse-app/tests/integration_console_engine.rs:410-420
```

**Disposition:** reject — check 3 and the operator's recorded direction: the wrap directive says nothing is reworded to sound as if the console engine already did what a later entry builds (inputs#I4 item 3), and the heartbeat-gap check over the console program's log is `Agent harness drives the console engine`'s; one consecutive pair of ticks in a 16-second test run is not the gap check, and the fact that a test asserts that pair is carried by test-plan §1's new surface row

## a11y-plan

verdict: `proposals: []` — no drift proposed.

## Tally

Proposals 63: apply 57 · reject 5 · escalated 1. Rejected for a source the report does not carry: 0 — every coordinate a proposal cites is a line the report states or a row (or a span of rows) of its `New text, by line` section, checked against the section.

Check 2 (cross-contradiction): none — obs-plan proposals 11 and 17 edit one bullet, and 17 is rejected.
Check 5 (expected amendments): every entry of the plan's list is matched by a proposal (architecture 1, 4, 5, 7, 11, 12, 14, 15 · obs-plan 1, 2, 8, 15, 16 · security-plan 1, 2, 5, 8 · test-plan 1, 2, 9, 13); none was raised by the orchestrator.
Check 6 (disproved claims): the report lists none.
