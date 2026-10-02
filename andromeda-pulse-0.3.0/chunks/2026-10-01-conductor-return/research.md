# Codebase Research — 2026-10-01-conductor-return

## Scope
- **Depth:** deep (PREREQ: boot/exit path + its dependency stack) · moderate (P-075: Pulse surfaces + the companion repo's
  ledger, read-only) · **Reads:** 31 · **Globs/Greps:** 38
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full by structural extraction (154 lines,
  index by `grep -n '^#\|^- 20'`, spans 1-128 and 129-155), 21 Session Additions applied; `.claude/rules/testing.md`
  (304 lines) and `.claude/rules/observability.md` (148 lines) — both `paths:` cover `xtask/**/*.rs`; read in full by a
  delegated structural extraction (every span read, coverage recorded in the extraction), the facts used below cited by
  `{file}:{line}`; `.claude/rules/host-win32.md` (always loaded).
- **Platform issues consulted:** query "tauri app exits with code 1 xvfb-run ubuntu-22.04 webkit2gtk github actions no
  panic" → FETCHED `github.com/yw0nam/YUI/issues/729`: a Tauri v2 + WebKitGTK app under Xvfb with no window manager
  "exits with code 1 approximately one second after the render loop begins", "no stderr, no panic message, no
  backtrace even with `RUST_BACKTRACE=1`", 5/5 reproducible there, root cause NOT identified, env workarounds
  (`WEBKIT_DISABLE_COMPOSITING_MODE`, `LIBGL_ALWAYS_SOFTWARE`, `GALLIUM_DRIVER=llvmpipe`) did not resolve it, closed as
  not planned. FETCHED `github.com/tauri-apps/tauri/issues/15936` (open): a sibling diagnostic gap under Xvfb — a blank
  window with no diagnostic (process keeps running), fixed by `WEBKIT_DISABLE_COMPOSITING_MODE=1`; the reporter's ask is
  that Tauri "log one line" naming it. Query "GDK X11 Fatal IO error exit(1) xvfb" → forum threads only (an X client
  talking to a gone X server), no tracker entry matching this signature. Conclusion: the 69f0b93 signature (silent
  `exit 1` at window init under Xvfb) has a public twin with no known cause and no upstream fix; instrumentation is the
  available move, which is this PREREQ.

## Files inspected
- `pulse-app/src/main.rs` (278-300, 1555-1570) — `main()` builds/enters the tokio runtime (`.expect` at :288), calls
  `observability::init(&data_dir)` holding the guard as `let _guard` (:293), then the Tauri builder ends
  `.run(tauri::generate_context!()).expect("error while running tauri application")` (:1567-1568). No `RunEvent`
  callback, no `run_return`, no signal handling anywhere in `pulse-app/src` (grep `tokio::signal|ctrlc|SIGTERM|ctrl_c|
  RunEvent::|run_return` → 0 hits; the only `on_window_event` is :1113).
- `pulse-app/src/observability.rs` (2587-2712) — `install_panic_hook` (:2587) emits `app.panic.fatal {location,
  spantrace}` then chains the prior hook; `init` (:2622) builds `rolling::daily(logs_dir, "agent-latest.jsonl")` →
  `tracing_appender::non_blocking` (:2628) as the ONLY writer (one `fmt::layer().event_format(JsonWithDefaults…)
  .with_writer(non_blocking)`, :2637-2641 — no stderr layer), installs the panic hook (:2649), returns the
  `WorkerGuard`. `fs::create_dir_all(...).expect` at :2624 and the runtime `.expect` at main.rs:288 run BEFORE any sink
  exists. Leaves present: `app.boot.pid` (:606), `app.panic.fatal` (:1081), `metric.report.render_ms` (:2419).
- `pulse-app/src/tray.rs:271` — `app.exit(0)`, the only explicit exit in `pulse-app/src` (grep
  `process::exit|ExitCode|\.exit\(` over `pulse-app/src` → 1 hit).
- `pulse-app/Cargo.toml` — `[lib] pulse_app` (`src/lib.rs`, `test = false`), `[[bin]] pulse-app`, one `[[example]]`;
  no direct `libc` dependency; no `panic =` profile key in `Cargo.toml` or `pulse-app/Cargo.toml` (grep → 0).
- `scripts/agent-run.sh` (60-135) — the app runs as `"$APP_BIN" > "$DATA_DIR/logs/boot.log" 2>&1 &` under a waiting
  subshell that writes `exit $rc` / `signal N (NAME)` to `run/andromeda-pulse.exit`.
- `.github/workflows/ci.yml` (393-456) — the boot job: `cargo build --workspace --release --features mcp-server`, then one
  `xvfb-run -a --server-args="-screen 0 1280x1024x24"` spanning `agent-run.sh boot` · `status` · `cleanup`, then
  `cargo xtask ci-gates`, then the `logs-boot-${{ runner.os }}` upload.
- `pulse-app/tests/e2e_p003_panic_hook_propagation.rs` (1-80) — calls `observability::init(data_dir.path())`, holds the
  guard, panics in a tokio task, and asserts the canary is absent from the log — the in-process precedent.
- Dependency sources (registry, read-only): `tao-0.35.0/src/platform_impl/linux/event_loop.rs` (985-1160),
  `tauri-2.11.0/src/app.rs` (570-625, 1330-1460), `tauri-runtime-wry-2.11.0/src/lib.rs` (3240-3262, 4185-4200,
  4320-4380), `tracing-appender-0.2.5/src/non_blocking.rs` (225-300), `libc-0.2.186/src` (atexit by target).
- Companion repo (read-only, cited never copied), Conductor HEAD `1fe46a1`: `contracts/pulse-capabilities.toml:88`
  (`"P-075"`), `contracts/` listing (7 files), `conductor-0.3.0/intent.md:7,143`, `conductor-0.3.0/working-route.md`
  (epochs 4-5, :41-77), `conductor-0.3.0/verification-matrix.json` (v3-01..v3-11), `conductor-0.2.0/verification-
  matrix.json#v2-20`, `conductor-0.2.0/chunks/2026-08-31-p-075-assert-round/report.md` (1-6, 80-100, 165-180),
  `.andromeda/master-route.md:107`, `.andromeda/architecture.md:62,93` ([Read-Back Dependency Posture]).
- Pulse ledger: `andromeda-pulse-0.3.0/verification-matrix.json#P-075` (`matrix.py show --id P-075`),
  `andromeda-pulse-0.3.0/requirements.md:26`, `andromeda-pulse-0.3.0/route-archive.md:231,243`,
  `chunks/2026-10-01-real-model-incident-surfacing/evidence/operator-pass.md` (§Red 2).

## Graph impact (rust plane, fresh — `tree-query-2026-10-01-conductor-return.json`, 14 rows)
- **init** (`pulse-app/src/observability.rs`) — 3 callers: `main @ pulse-app/src/main.rs:293`,
  `p003_panic_propagates_to_receiver_panicked_broadcast_within_2s @ pulse-app/tests/e2e_p003_panic_hook_propagation.rs:51`,
  `perf_budget_samples_are_produced_by_the_real_pipeline @ pulse-app/tests/perf_budget_samples.rs:115`. A changed `init`
  signature reaches both tests; keeping it (`-> WorkerGuard`) leaves them untouched.
- **install_panic_hook** — 1 caller: `init @ pulse-app/src/observability.rs:2649`.
- **log_basename** — `write_pid_file @ pulse-app/src/main.rs:230,232,246`, `init @ observability.rs:2661`, and the pin
  `log_basename_returns_filename_only @ pulse-app/tests/observability_pins.rs:354,360,365`.
- **resolve_data_dir / write_pid_file / emit_boot_spans** — `main @ main.rs:292 / :295 / :296` (single callers).
- No crate-edge query was needed: every touched symbol is in `pulse-app` (the binary-boundary crate); no lib crate changes.

## Patterns detected
- **Exact-leaf bounded diagnostic record** (`interpretation.incident.skipped`, `ui.ipc.rejection`, `app.boot.window.
  navigation`): own exact allowlist leaf, closed-enum/integer fields, a `pulse-app/tests/unit_observability_allowlist_*.rs`
  guard asserting exact resolve + set equality both ways + banned fields + no-bare-`app` discriminator, mutation-checked
  (observability.md:61-71, :132, :148; obs-history 2026-08-24, 2026-08-30-acl-rejection-logging).
- **Cross-process claim via re-exec** — re-exec `std::env::current_exe()` with `--exact <test::path>`, state through env
  vars, the child writes to a temp sink, a skip-clean guard, verified once with `--no-capture` that the child ran
  (testing.md:260). This is the CI-runnable shape for "the record survives process termination".
- **Panic hook chaining** (`observability.rs:2592-2617`): take the prior hook, emit, call it — the sibling the exit path
  mirrors for ordering and sink.

## Conventions to follow
- New pulse-app tests live in `pulse-app/tests/*.rs`, collected by name (`cargo nextest list -p pulse-app | grep <name>`);
  the dead-test ratchet is flat zero for lib sources, `main.rs` exempt (testing.md:25, :185).
- Basenames only through `pulse_app::observability::log_basename` (security.md §Logging; main.rs:230-246 precedent).
- Edition 2024 FFI: `unsafe extern` blocks and `unsafe` blocks inside `unsafe fn` (security extract, toolchain bullet).
- ASCII-only source text — `cargo xtask check:english-sources` is a CI and pre-push stage (verification-harness.md:84).
- A gate's exit is read from the bare command, never through `| tail` (testing.md:230, :294; host-win32.md).

## Mechanism re-derivations (the plan's load-bearing equalities, verified at HEAD)
1. **The Rust graph has exactly one runtime path to a non-zero process exit, and it does not yield `exit 1` here.**
   Sweep of every `Cargo.lock` package's registry `src/` (916 packages, 891 dirs present; `tests/examples/benches/bin`
   and `build.rs` skipped) for `process::exit(|libc::exit(|libc::_exit(` — script
   `scratchpad/exit_sweep.py`, re-run after a CRLF false-zero (host-shell, recorded). Runtime hits in pulse-app's graph:
   `tao-0.35.0/.../linux/event_loop.rs:998` `process::exit(exit_code)` (and its macOS :202 / Windows :230 twins),
   `tauri-2.11.0/src/app.rs:578` (the `AppHandle::exit` fallback, same code), `tauri-plugin-updater-2.10.1/src/updater.rs:865`
   (`exit(0)`). The rest are build-time (`cc`, `tauri-build`, `tauri-codegen`, `tauri-plugin` build), docs (`csv`), test
   harnesses (`proptest`, `rusty-fork`), or other platforms (`objc2-*`). tao's code comes from `ControlFlow::ExitWithCode`;
   the only `ExitWithCode(1)` is a queued `Event::LoopDestroyed` (`event_loop.rs:1127`) with no Linux sender (grep
   `LoopDestroyed` over `tao-0.35.0/src` → only the loop's own handling + `event.rs` definitions); tauri-runtime-wry sets
   `ControlFlow::Exit` (code 0) on every exit request (`lib.rs:3205, :4329, :4372`). A panic exits 101. So a bare
   `exit 1` with 0 `app.panic.fatal` is, by elimination over the Rust graph, a NATIVE `exit()` (GTK/GDK/WebKitGTK/glib)
   — the class only an at-exit hook can see. This is a classification of the candidate set, NOT a cause of the 69f0b93
   death, which stays unexplained.
2. **No record emitted on the event-loop exit path is guaranteed to reach the file.** `Builder::run` → `App::run` →
   runtime `run` → tao `run` → `process::exit(exit_code)` (`tauri-2.11.0/src/app.rs:1338` docs: "the process is exited
   directly using std::process::exit"; tao :997-998). `process::exit` runs no destructors, so `main`'s `_guard`
   (main.rs:293) is never dropped. The file is written by a separate worker thread draining a channel
   (`non_blocking`, lossy by default, `DEFAULT_BUFFERED_LINES_LIMIT` 128 000, non_blocking.rs:67, :232); the ONLY flush
   handshake is `WorkerGuard::drop` (non_blocking.rs:283-300: `send_timeout(Shutdown, 100 ms)` then wait up to 1000 ms).
   An event enqueued immediately before `process::exit` therefore races the worker — whether it lands is UNMEASURED and
   is the RED leg's first measurement at /implement. `App::run_return` (app.rs:1397) returns the intended exit code
   instead of exiting, which gives Rust code a place to emit, drop the guard (flush), then exit with the same code.
3. **An at-exit hook sees both `process::exit` and a native `exit()`, but not the code.** Rust's `process::exit` calls the
   platform `exit`, which runs `atexit` handlers; a C `exit(1)` from GTK runs them too; `_exit`, a signal kill and
   `TerminateProcess` do not. `libc-0.2.186` binds `atexit` for `unix/mod.rs` and `windows/mod.rs`; glibc's `on_exit`
   (which passes the status) is NOT bound by the crate (grep `pub fn on_exit` → only haiku's `on_exit_thread`). Other
   threads are still alive while `atexit` handlers run, so a handler that drops a guard held in a static can drain the
   worker — the equality to MEASURE at /implement: an induced `libc::exit(1)` after `init` leaves exactly one cause record
   in `agent-latest.jsonl*` after the child has ended.
4. **Signals end the process with no record today.** No handler is installed (grep above). The harness `cleanup` sends
   TERM (verification-harness.md:25); the wrapper records `signal N (NAME)`. SIGKILL and Windows `Stop-Process -Force`
   (TerminateProcess) are unloggable by construction.
5. **Pre-sink failures cannot reach the file by construction**: the runtime `.expect` (main.rs:288) and the logs-dir
   `.expect` (observability.rs:2624) panic before the subscriber exists — the default hook prints to stderr, which the
   harness captures in `boot.log`.
6. **P-075 content fidelity has exactly one payload-varying read-back value under deterministic L4**: the triggering
   cue's full-hex fingerprint inside `retrieve_telemetry_slice.fingerprint_refs`, beside the constant `det-*` triple;
   every L4-authored field is a fixture constant; `incident_events` reaches no MCP tool (a Pulse 0.4.0 residual
   candidate); runtime-STATE fidelity (`mark_incident_resolved` + `query_incident_list` active-set membership) is the
   other live-proven axis (Conductor `.andromeda/architecture.md:62,93`; master-route.md:107). Pulse-side, the
   fingerprint threading is the [Fault Identity] invariant (CLAUDE.md; arch-history 2026-08-26-interpretation-brief-
   completeness).

## Premise corrections recorded at the closure
- **v2-20's P-027 PASS cannot back P-075.** Conductor graded P-027 on 2026-08-21 against Pulse's then-HEAD; Pulse's
  `2026-09-30-p-027-discovery-bound` later measured that the tick-anchored `discovery_ms` HID the wait (obs-history:
  435 ms reported against a true 15 219 ms) and re-anchored it. The budget surfaces also moved since (`git log
  --since=2026-08-21` over the emit sites: `87fe658`, `f2a131a`, `5fbf762`, `2f43cb8`, `b2e4cb3`). So the existing
  external evidence is stale for P-027 and of an older binary for P-037/P-045; a fresh round is required.
- **P-025 is now gradable and graded.** Conductor `v3-08` (verified, `2026-09-29-hue-shift-budget-graded-hard`): one
  graded live leg, PASS worst 684.98 ms <= 2000 at Pulse checkout `4502d5d` carrying `e98d838`, graded under the contract
  `contracts/pulse-p025-measurement-contract.md`. The hue path has not changed since `e98d838` (`git log e98d838..HEAD`
  over `constellation-types.ts`, `ConstellationCanvas.tsx`, `telemetry.rs` → only `5fbf762`, which added the WebGPU
  adapter procedure, not a hue change). The cap's P-025 over-claim note (2026-08-22) is therefore resolvable by the
  claiming chunk's concretization.
- **No Conductor route entry runs a P-075 round today.** Conductor HEAD `1fe46a1`: `intent.md:143` states Pulse's P-075
  bookkeeping "is a decision in Pulse's ledger, not Conductor work"; the working route's markerless tail is the in-flight
  third real-model series (`:75`, `BLOCKED-ON` Pulse's incident-surfacing fix) and "Version close on measured evidence"
  (`:77`). The round this entry names needs a Conductor-side owner the operator mints.

## New files to create
- `pulse-app/tests/unit_observability_allowlist_app_exit.rs` — the exit-cause leaf guard: exact resolve, set equality
  both ways, banned fields, no-bare-`app` discriminator, mutation-checked.
- `pulse-app/tests/integration_exit_cause_record.rs` — re-exec witness: each loggable exit class induced in a child after
  `init`, the parent reads the child's `agent-latest.jsonl*` after it has ended.

## Files to modify
- `pulse-app/src/main.rs` — the Tauri run ends through a code-returning path that emits the event-loop exit record and
  flushes before the process exits; the at-exit hook is installed after `init`.
- `pulse-app/src/observability.rs` — the exit-cause emit surface (closed exit-class enum, bounded fields), the guard
  holder the at-exit hook drains, the `atexit` registration, and the exact allowlist leaf.
- `pulse-app/Cargo.toml` — `libc` as a direct dependency at the lockfile's existing `0.2.186`.
- `Cargo.lock` — `pulse-app`'s dependency list gains `libc` (no new package version).
- `pulse-app/tests/observability_pins.rs` — sweep record target: any pin enumerating the allowlist's key set is updated
  for the new leaf (`no change` if none enumerates it — /implement's sweep decides).
- `pulse-app/tests/unit_observability_allowlist_sweep.rs` — the sweep leaf-equality guards gain the new target if they
  enumerate leaves (observability.md:73).

## Open questions
- Does this chunk wait for Conductor's fresh round and claim P-075, or land the PREREQ and leave P-075 pooled for a later
  entry? → blocks: plan-decision (P4 fork; the round has no Conductor route owner today).
- Which non-event-loop exit classes the PREREQ instruments: at-exit hook (native `exit()`), Unix SIGTERM/SIGINT, or both;
  signals re-raised to keep `signal N` semantics or not → blocks: plan-decision.
