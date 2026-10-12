# Codebase Research — 2026-10-10-console-engine-entry-point

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 23
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (160 lines, 23 Session
  Additions; an index of headers and dated introducers, then three offset reads covering every line). Applied:
  the `boot` verb builds and spawns `target/release/pulse-app` by path; `harness:boot-series` binds the resolved
  OTLP ports, one boot at a time, and on the dev host they are moved off 4317 / 4318 first; one exported data dir
  per leg; readers resolve the `agent-latest.jsonl*` family; a leg's window outlasts the threshold its verdict
  reads; a harness's negative finding needs a second source.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's phase directive for this chunk (size of the step, what stays
  true while the window stands, the closed list, no local model, boundaries, P-086, the P5 stop)

## Files inspected
- `pulse-app/src/main.rs` (280-1622 whole, 1815-1934) — the whole engine is wired inside one `fn main()`
  (`:280-1582`): state is built at `:297-1116`, the Tauri builder at `:1118-1577`, and every engine task is spawned
  inside its `.setup` closure (`:1132-1575`). Nine private helpers sit beside it (`resolve_data_dir`,
  `resolve_port`, `resolve_grpc_port`, `resolve_http_port`, `resolve_retention_seconds`, `write_pid_file`,
  `publish_workspace_key_for_sidecar`, `resolve_mcp_sidecar_binary_path`, `init_buffer`). The `mod tests` at
  `:1624` holds 14 resolver tests and `emit_taurpc_bindings`, which builds its own router from hermetic inputs and
  does not touch the engine boot.
- `pulse-app/src/lib.rs` (full) — 42 public modules; engine wiring and window code share one library crate.
- `pulse-app/Cargo.toml` (full) — `[lib] test = false`, one `[[bin]] pulse-app`, no `default-run`; `tauri`,
  `taurpc`, three tauri plugins and `tauri-build` are plain dependencies of the crate.
- `Cargo.toml` (workspace head) and `crates/*/Cargo.toml` (dependency lines) — 16 members; `clap` is a workspace
  dependency used by `xtask` alone.
- `crates/ui-bridge/Cargo.toml`, `src/lib.rs`, `src/health.rs` (head and item outline) — `tauri`, `taurpc`,
  `specta` and `tokio` are optional behind the default-on feature `taurpc-runtime`; `Settings`, `HeartbeatState`,
  `BindStatus`, `record_start`, `register_heartbeat_state` and `current_health` are outside the gate.
- `pulse-app/src/heartbeat.rs` (1-125) — `spawn` takes ten arguments and returns five handles: `ingest.tick`,
  `buffer.tick`, `viz.tick`, `plugins.tick`, `connection.tick`.
- `pulse-app/src/observability.rs` (public outline, 2885-2916) — `init`, `hold_log_guard`, `install_exit_hook`,
  `install_signal_listener`, `record_exit`, `exit_after_event_loop`, `SERVICE_NAME`. The file names `tauri` in
  strings only (allowlist keys); it imports nothing from the window framework. `install_signal_listener` needs an
  entered tokio runtime and returns silently without one.
- `pulse-app/src/deterministic_inference.rs` (1-60) — the canned `L4Output` runner, selected in `main` when
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` is truthy.
- `pulse-app/src/window.rs` (86-102) — `emit_boot_spans` writes three window records (`app.boot.webview.init`,
  `app.boot.gpu.check`, `app.boot.tray.init`).
- `crates/corpus/src/keychain.rs` (104-162, 262-320) — the key comes from the OS credential store; when the store
  fails, the key is derived from `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` if it is set, with one WARN
  `corpus.keychain.fallback`; with neither, `Corpus::open` fails and `main` continues with no corpus (`:349-362`).
- `pulse-app/tests/unit_xlib_threads.rs` (40-80) — pins by source text that `main`'s first statement is the render
  posture, its second `xlib_threads::init();`, and that the tokio runtime is built in `main`'s body after both.
- `pulse-app/tests/unit_observability_allowlist_sweep.rs` (199-244) — the dead-test ratchet reads the top level of
  `pulse-app/src/` only and exempts `main.rs` by name; a file under `pulse-app/src/bin/` is not read by it.
- `pulse-app/tests/integration_exit_cause_record.rs` (1-60, test outline) — seven re-exec arms over the library's
  exit functions, the signal arm included (`exit_cause_sigterm_is_recorded_and_still_ends_by_signal`).
- `pulse-app/tests/unit_heartbeat_ticks.rs` (15-30, 440-500) — `spawn_returns_five_handles_and_aborts_cleanly`
  calls `heartbeat::spawn` and asserts five handles.
- `crates/ingest/examples/inject_demo.rs` (argument and endpoint lines) — the injector connects to
  `http://127.0.0.1:4317` and takes no port argument.
- `scripts/agent-run.sh` (build and spawn lines) — `boot` runs `cargo build --bin pulse-app --release` and spawns
  `target/release/pulse-app`.
- `.github/workflows/ci.yml` (job and step outline) — `lint-test` runs `cargo xtask test` with no `xvfb-run`; the
  only `xvfb-run` is the `boot` job's smoke; `supply-chain` and `boot` each build `--workspace --release`.
- `.config/nextest.toml` (full) — profile `ci`: `slow-timeout = { period = "60s", terminate-after = 3 }`.
- `andromeda-pulse-0.4.0/chunks/2026-10-10-boot-smoke-s-self-end-closed/plan.md` (131-464) — the recorded firing
  form of the window app's series on this host: ports 14317 / 14318, `env -u WAYLAND_DISPLAY`, a fresh data dir
  under `target/boot-smoke/`, the witness library, `cargo xtask harness:boot-series --count 2`.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **`spawn`** (`pulse-app/src/heartbeat.rs`) — 3 call sites: `main()` at `pulse-app/src/main.rs:1557`, and
  `pulse-app/tests/unit_heartbeat_ticks.rs:25` (the import) and `:474`
  (`spawn_returns_five_handles_and_aborts_cleanly`). A change of its signature touches one production caller and
  one test file.
- **The nine private helpers of `main.rs`** — every reference is inside `pulse-app/src/main.rs` (9 names, 24
  sites, 1 file; `resolve_port` 9 and `resolve_retention_seconds` 8 include their tests in `mod tests`). Moving
  them into the library moves their 14 tests out of `main.rs` too: a test left beside them in a library source
  never runs and reddens the ratchet.
- **Names** — `engine` is taken as a module name by `crates/plugins/src/engine.rs` and as a field of
  `PluginsApiImpl`; `Engine`, `EngineBoot`, `console`, `start_engine` and `boot_engine` have no symbol on the rust
  plane (2 rows returned, both for `engine`).

## Patterns detected
- **The window framework is a dependency of two crates, and one of them can already drop it.** `tauri` is a
  dependency of `pulse-app` and, optionally, of `crates/ui-bridge` (`grep -n -E '^(tauri|taurpc|specta)'` over the
  workspace and member manifests: `tauri` at `pulse-app/Cargo.toml:37` and `crates/ui-bridge/Cargo.toml:29`,
  nowhere else). `crates/ingest` and `crates/triage` carry an optional `specta` behind a feature that is also
  named `taurpc-runtime`; they do not depend on `tauri`. `xtask` already builds `ui-bridge` with
  `default-features = false`.
- **18 of 43 source files of `pulse-app/src` name tauri or taurpc** (`grep -l -E 'tauri|taurpc'
  pulse-app/src/*.rs | wc -l` → 18; `ls pulse-app/src/*.rs` → 43, `xlib_threads.rs` being the one added since the
  directive's count). None of the engine-wiring modules is among them: the persistence adapters, the observers,
  `heartbeat.rs`, `digest_runtime.rs`, `inference_runtime.rs`, `corpus_retrieval.rs`, `reevaluation.rs` and
  `cadence_runner.rs` import nothing from the window framework. `observability.rs` is on the list for its strings
  alone. Three helpers the engine boot calls live in router files that also define TauRPC routers:
  `connection_router::HeartbeatBindStatus`, `config_router::settings_to_cadence_config` /
  `settings_to_lifecycle_thresholds` / `config_watch_error_category`.
- **A binary that links the `pulse_app` library without building the window needs no window library at load.**
  Measured on this host: of the 103 test binaries built from `pulse-app/tests/*.rs` (debug profile, each links the
  library, the router tests included), 0 carry a `NEEDED` entry for GTK, GDK, WebKit, libsoup or JavaScriptCore
  (`readelf -d {binary} | grep NEEDED | grep -c -i -E 'gtk|webkit|gdk|soup|javascriptcore'` over all 103). Known
  positive: `target/debug/pulse-app` and `target/release/pulse-app` each read 6. This is a reading of test
  binaries; the console binary itself is read by a gate entry once it exists.
- **The engine boot has no seam.** Every engine task is spawned from the Tauri `.setup` closure through
  `tauri::async_runtime::spawn`, on the runtime `main` builds and hands to Tauri (`main.rs:290-295`).
  `heartbeat::spawn`, the digest runtime and the inference runtime already use plain `tokio::spawn` from that same
  closure, so the closure runs with the runtime entered and the spawns do not depend on Tauri.
- **An incident is created only behind the model's output.** The chain is cue → cadence coordinator → digest
  trigger → digest assembler → `DigestBroadcast` → `spawn_l4_inference_subscriber` → `llm_runner` →
  `create_incident_from_l4_output` (`main.rs:1416-1476`). No other production path creates an incident. The one
  existing way through it with no model is the canned runner of `deterministic_inference.rs`.
- **The process-end record needs no event loop.** The signal listener records `app.exit` with class `signal` from
  a runtime task, restores the default disposition and re-raises (`observability.rs:2891-2916`); the at-exit hook
  records `outside_event_loop`. `exit_after_event_loop` is the window's tail alone. A program that runs until it
  is signalled ends through the first, and the closed label set is unchanged.
- **A second binary in the same crate is spawnable from that crate's tests.** `env!("CARGO_BIN_EXE_{name}")` is
  available to an integration test of the crate that declares the `[[bin]]`, and cargo builds the binary for the
  test (rules/testing.md 2026-05-13). The sidecar tests, which spawn a sibling crate's binary, need a prebuild
  step; a binary declared by `pulse-app` does not.
- **The workspace test run in CI has no display.** `lint-test` runs `cargo xtask test` outside `xvfb-run`, and the
  local pre-push check's `test` stage runs under a cleared environment with no display variable and no session
  bus. A test that spawns the console program there witnesses a boot with no display on every push, with no
  workflow edit.
- **The injector cannot feed a program on other ports.** `inject_demo` is fixed to 4317. A check that keeps off
  the shared ports sends OTLP with a `tonic` client of its own (`tonic` and `prost` are dev-dependencies of
  `pulse-app` already).
- **Coverage margin.** The last recorded coverage step read `Line: 33992/38252 = 88.9%` and `Function: 3565/4055
  = 87.9%` against 75 % and 85 % (`chunks/2026-10-10-no-gate-stands-while-reading-nothing/evidence/gate-census.md:38`).
  A process ended by a re-raised signal writes no coverage profile, so the spawned program adds no covered line;
  an in-process test of the boot function does.

## Conventions to follow
- **Tests for `pulse-app` library code live in `pulse-app/tests/`**, reach internals through `pub` +
  `#[doc(hidden)]`, and are checked by name in the workspace listing (rules/testing.md 2026-05-20, 2026-08-29).
- **A new log target gets its own exact allowlist leaf, a guard under `pulse-app/tests/` asserting the field set
  by equality both ways, and a discriminator that no bare `app` key exists** (rules/observability.md; the
  `app.boot.render.posture` leaf and `pulse-app/tests/unit_observability_allowlist_render_posture.rs` are the
  nearest precedent).
- **A closed command-line contract rejects an unknown argument with exit 2** (`inject_demo`'s `parse`,
  `crates/ingest/examples/inject_demo.rs:210-221`; the xtask verbs' 0 · 1 · 2 exit form).
- **A cross-process claim is witnessed across a real process boundary, the log read after the child has ended,
  with the child-ran proof in the arm** (`pulse-app/tests/integration_exit_cause_record.rs:1-10`).
- **No `sleep` for synchronisation**: wait on a record in the log family or on a TCP accept, under a bound
  (rules/testing.md, Synchronization).
- **A diff-shaped probe names the chunk base `f31027a4`, never `HEAD`**, and a scope guard's pathspec lists the
  chunk's new files beside its modified ones (rules/testing.md 2026-10-04).

## New files to create
- `pulse-app/src/engine_boot.rs` — the one engine boot both programs call: the environment resolvers and boot helpers moved out of `main.rs`, the engine's state, its task spawns and its boot record
- `pulse-app/src/console.rs` — the console program's command grammar and its run function
- `pulse-app/src/bin/andromeda-pulse-engine.rs` — the console program's entry point
- `pulse-app/tests/unit_engine_boot_env.rs` — the 14 resolver tests moved out of `main.rs`
- `pulse-app/tests/unit_console_commands.rs` — pins of the command grammar and its exit codes
- `pulse-app/tests/unit_observability_allowlist_boot_engine.rs` — the guard of the boot record's allowlist leaf
- `pulse-app/tests/unit_engine_boot_seam.rs` — the pin that neither entry point wires the engine by itself
- `pulse-app/tests/integration_engine_boot.rs` — the boot function run in process on its own data dir, its composition asserted
- `pulse-app/tests/integration_console_engine.rs` — the built console program run with no display: receivers, records, telemetry landed, one process-end record

## Files to modify
- `pulse-app/src/main.rs` — the engine boot leaves for the shared function; the window shell, the routers and the bindings test stay; the resolver tests leave
- `pulse-app/src/lib.rs` — the two new modules
- `pulse-app/src/heartbeat.rs` — the engine's ticks and the window's ticks spawn apart
- `pulse-app/src/observability.rs` — the allowlist leaf of the boot record
- `pulse-app/Cargo.toml` — the second `[[bin]]` and `default-run`
- `pulse-app/tests/unit_heartbeat_ticks.rs` — the callers of the changed `heartbeat::spawn`

<!-- Not on the list, by reading: `Cargo.lock` (no dependency is added; the command grammar is two words and needs
no parser crate); `.github/workflows/ci.yml` (the witness runs inside the existing `cargo xtask test` step and the
two workspace release builds compile the new binary as they stand); `scripts/agent-run.sh` and `xtask/` (the
harness verbs are the next entry's); `pulse-app/capabilities/*.json`, `pulse-app/ui/**` and the bindings (no
procedure, no surface); `docs/capability-record.json` (no claimed entry names `main.rs`, `heartbeat.rs`, a resolver
test or the heartbeat test: `grep -n -E 'main\.rs|unit_heartbeat_ticks|resolve_port|resolve_retention|heartbeat\.rs'
docs/capability-record.json` → 0 lines; known positive: the same grep for `pulse-app/tests` matches). -->

## Open questions
- What the console engine does at the point where 0.3.0's flow called the model: it runs the canned runner only
  when `ANDROMEDA_PULSE_L4_DETERMINISTIC` is set and nothing otherwise, or it runs the canned runner always. Both
  start, load and wait for no model; they differ in whether an ungated console run creates incidents, and with
  what text. → blocks: plan-decision
- What `tauri build` does with a second `[[bin]]` in the crate is not measured (the bundler may place the console
  binary in the desktop bundles). The release workflow is not run by this chunk and leaves at
  `Desktop distribution retired`. A note, not a question: no step of this chunk waits on it.
