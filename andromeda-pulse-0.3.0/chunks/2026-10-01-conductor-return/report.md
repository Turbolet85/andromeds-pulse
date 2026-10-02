# Report — 2026-10-01-conductor-return

**Chunk:** Conductor return — every non-zero pulse-app exit logs its cause; the external P-075 assert round verifies the last unclaimed capability
**Date:** 2026-10-02T13:00Z
**Commits:** `03ec944 chore(2026-10-01-conductor-return): operator pre-CI commit` (since last_wrap 2026-10-01T18:30:53Z; base `a2addb3`)

## Changes (structured — detectors read this)
- **Files:** `pulse-app/Cargo.toml` · `Cargo.lock` · `pulse-app/src/observability.rs` · `pulse-app/src/main.rs` · NEW
  `pulse-app/tests/unit_observability_allowlist_app_exit.rs` · NEW `pulse-app/tests/integration_exit_cause_record.rs` ·
  the chunk folder (`evidence/` red-probe, mutation-checks, round-request, round-binary, operator-pass) ·
  `andromeda-pulse-0.3.0/verification-matrix.json` (P-075) · `plan.md` (one `timeout` key, operator-directed — Deviations).
  Unchanged after the sweep: `pulse-app/tests/observability_pins.rs`, `pulse-app/tests/unit_observability_allowlist_sweep.rs`
  (neither enumerates the allowlist key set: `grep -n 'keys()\|registered' ` over both → no key-set enumeration).
- **Symbols / APIs:** in `pulse-app/src/observability.rs`, all `#[doc(hidden)] pub` (reached by `pulse-app/tests/` because
  `[lib] test = false`): `enum ExitClass {EventLoop, OutsideEventLoop, Signal}` + `label()` (`event_loop` ·
  `outside_event_loop` · `signal`) · `enum ExitSignal {None, Sigterm, Sigint}` + `label()` (`none` · `sigterm` · `sigint`) ·
  `hold_log_guard(WorkerGuard)` (a process-wide `Mutex<Option<WorkerGuard>>` slot) · `flush_log_sink()` (takes and drops
  the guard — idempotent; `WorkerGuard::drop` is the sink's only drain) · `record_exit(class, Option<i32>, ExitSignal) ->
  bool` (process-wide once-flag `EXIT_RECORDED`, first emitter wins, emits then flushes) · `exit_after_event_loop(code) -> !`
  (record → flush → `std::process::exit(code)`, same code) · `install_exit_hook()` (once: spawns the
  `pulse-exit-reporter` thread, then `libc::atexit(on_process_exit)`; the handler hands off to the reporter thread and
  waits ≤ 2 s, because glibc runs the exiting thread's TLS destructors before `atexit` handlers and the formatter needs
  TLS) · `#[cfg(unix)] install_signal_listener()` (tokio SIGTERM/SIGINT streams on the entered runtime; records, then
  `libc::signal(SIG_DFL)` + `libc::raise` the same signal — the process still ends BY the signal; silently absent when no
  runtime or registration fails). `observability::init` signature UNCHANGED (`-> WorkerGuard`); its two test callers
  (`e2e_p003_panic_hook_propagation.rs`, `perf_budget_samples.rs`) untouched. `main.rs`: `hold_log_guard(init(..))` +
  `install_exit_hook()` + `#[cfg(unix)] install_signal_listener()` after `init`; the Tauri builder now ends
  `.build(ctx).expect(..)` + `app.run_return(|_, _| {})` + `exit_after_event_loop(code)` (was `.run(ctx).expect(..)`, which
  ends in tao's `process::exit` with the worker undrained). No TauRPC procedure, MCP tool, port, env var or capability
  added; `window::on_window_event` (hide-to-tray) untouched.
- **Crates / modules:** none added/removed; `pulse-app` only (binary-boundary crate), no library crate touched.
- **Dependencies:** `libc = "0.2.186"` added as a DIRECT `pulse-app` dependency (already in `Cargo.lock` at that one
  version transitively; the lockfile diff is pulse-app's dependency list gaining `"libc"`, 1 line). No version bump.
- **Schema / config:** NEW log target `app.exit` with its OWN exact allowlist leaf in `AllowList::production()` (beside
  `app.panic.fatal`) carrying exactly `{exit_class, exit_code, exit_code_known, signal}` — closed-enum labels, an `i32`
  (0 when unknown) and a `bool`; no path, payload or free text. Levels: INFO for `event_loop` with code 0 · WARN for
  `signal` · ERROR for `event_loop` non-zero and every `outside_event_loop`. Exactly one record per process end (the
  once-flag, and structurally the single drain — Decisions). No bare `app` key exists (pinned `is_none()`). Loggable ends:
  the event-loop exit (code carried), a C `exit()` incl. a native GTK/GDK/WebKitGTK one (`outside_event_loop`, code not
  known — `libc` binds no `on_exit`), a Rust `process::exit` on Unix (same class), SIGTERM/SIGINT on Unix. UNLOGGABLE by
  construction: SIGKILL · `_exit` · Windows `Stop-Process -Force` (TerminateProcess) · a Rust `std::process::exit` on
  WINDOWS (= `ExitProcess`, runs no `atexit` and stops other threads first — measured) · pre-sink failures (runtime
  `.expect` in `main`, logs-dir `.expect` in `init` — stderr / `boot.log` only). A main-thread panic keeps `app.panic.fatal`;
  whether it is followed by an `outside_event_loop` record is NOT measured (a test child cannot panic its main thread) —
  by construction main-thread unwinding returns from `main` → C runtime `exit` → the hook → one record after the panic
  record.
- **Spec-master edits:** none (implement authors none).
- **Counts / qualifiers moved:** none — verified. The workspace test count moved 2551 → 2560 (Windows host) / 2562
  (WSL Linux, + the two `cfg(unix)` arms), and no master or leaf states it: `grep -rln '2551\|2,551\|2 551'` over
  `.andromeda/*.md CLAUDE.md .claude/docs .claude/rules` → 9 files, every hit `Ed25519` (false positive — Decisions).
  `pre-push:linux` stage count unchanged (six).
- **Dev-tool versions:** none — rustc re-read at 1.95.0 (dev host).
- **Harness / gate surface:** none changed. Measured facts about existing gates: the targeted nextest entry
  (`-E 'test(/app_exit|exit_cause/)'`) ran 3564.77 s against its 3600 s timeout on the cold build after
  `cargo clean --profile dev` (with a concurrent rust-analyzer flycheck and a Conductor workspace build on the host), and
  ~31 min again on the incremental re-run (as did the `nextest list` entry) — operator directive: timeout raised to 5400
  (Deviations).
- **Cross-project / external claims:** (a) Conductor (`D:/dev/projects/conductor`), read-only, at
  `2a494804f6d91bb61718fc9a520cebc73b29af86` (checkout HEAD read = that sha): the P-075 round on Pulse S
  `03ec94481b0d6c3ba574626e7acb39e33fd40141` graded 6/6 PASS — `conductor-0.3.0/chunks/2026-10-02-p-075-assert-round-against-pulse/evidence/round-ledger.md`
  (read via `git show 2a494804:…`), assertion 1-2 booleans in `p075-leg.txt`; Conductor CI#36970919487 green
  (overseer-verified, not read here); the binary sha256 pair in its ledger equals `target/release/{pulse-app,andromeda-pulse-mcp}.exe`
  here (re-measured); the six test fns exist at that sha in `crates/conductor-run/tests/lifecycle_harvest.rs` (1-2) and
  `delegated_timing_harvest.rs` (3-6) (`git grep`). (b) Pulse CI on S: `ci#36964436269` completed/success +
  `secret-scan#36964436289` completed/success, 13/13 checks, sha `03ec944` (gate 24, `ci.py conclusion`). This wrap's
  commit adds to that tree (docs, ledger, run dirs, one plan `timeout` key; no source).
- **Reverted / negative API facts:** a TEMPORARY test target `pulse-app/tests/integration_exit_red_probe.rs` (the
  measure-first probe, base product + `libc`) was written, run once (slot #1) and deleted; its readings live in
  `evidence/red-probe.md`.
- **Insufficient fixes (written, kept, not the remedy):** the PREREQ instrumentation does NOT explain the `69f0b93` Linux
  CI boot-smoke `exit 1`; it makes the next such end name its class (`outside_event_loop` at ERROR for a native exit).
  Owner of the remainder: the next runner-side occurrence (the Linux boot job is the observer).
- **Spec claims disproved by measurement:**
  1. research.md §Mechanism re-derivations 3 — "Rust's `process::exit` calls the platform `exit`, which runs `atexit`
     handlers" — FALSE on Windows: `library/std/src/sys/exit.rs` maps `target_os = "windows"` to `ExitProcess`; the
     probe's `atexit` marker did not run under `std::process::exit(4)` and did under `libc::exit(1)`
     (`evidence/red-probe.md` §The decider). A CHUNK-ARTIFACT claim (research.md is never mutated): recorded here, no
     amendment owed (operator: "a research correction").
  2. plan.md Step 8 mutation line — "neutralize the once-flag → (e) reddens" — FALSE as written: with the whole
     once-guard neutralized, arm (e) and a new arm (e2) both stayed green, because the first `record_exit` flushes and
     the flush drops the only `WorkerGuard`, so no second record can reach the file; on Windows the Step 4 tail also never
     reaches the hook. The flag became discriminable only by reading `record_exit`'s return value in (e2)
     (`evidence/mutation-checks.md` §Mutation 4). A CHUNK-ARTIFACT claim: recorded here, no amendment owed.
  3. plan.md Step 8 arm (b) premise (a Rust `process::exit` recorded as `outside_event_loop` on every OS) — FALSE on
     Windows by 1. above; arm (b) is `cfg(unix)` and ran green on WSL (`pre-push:linux`) and CI. Recorded here.
- **Expected amendments (from plan):**
  - obs-plan §6 and §8 — `app.exit` target + its exact leaf, dual-site: CARRIED (Schema / config bullet). Sites:
    `grep -n 'app.boot.window.navigation' .andromeda/obs-plan.md` → 2 hits (`:418` §6 `warn` row, `:534` §8 leaf list) —
    the sibling `app.`-prefixed exact-leaf precedent; owner obs-plan.
  - security-plan §Security Anti-Patterns → Logging (bounded closed-enum record) + Universal (the new `atexit` / signal
    FFI surface): CARRIED (Schema / config + Symbols bullets). Sites: `grep -n '^### Logging\|^### Universal'
    .andromeda/security-plan.md` → `:426`, `:465`; `grep -c 'atexit\|app.exit' .andromeda/security-plan.md` → 0 (new).
  - test-plan §1 (a pending trigger for the `main.rs` composition of Step 4 — the `run_return` tail is a one-line call
    into the witnessed `exit_after_event_loop`; the live event-loop exit is not inducible without a click) + §3 (the
    exit-record witness form — the re-exec children, each asserting its init record): CARRIED (Symbols + Outcome). Sites:
    `grep -n '^### Pending coverage triggers\|^### Per-chunk gate discipline' .andromeda/test-plan.md` → `:111`, `:295`.
  - architecture §Occupied Resources (or Stack) — `libc` as a direct dependency, "if the registry enumerates direct
    dependencies": NOT CARRIED — `grep -c 'libc' .andromeda/architecture.md` → 1 (`libcuda`, unrelated);
    `grep -c 'bincode\|tauri-plugin-clipboard-manager\|pulldown' .andromeda/architecture.md` → 0 — arch enumerates no
    direct `pulse-app` dependency. Fact stated in Dependencies.
- **Coverage of new surfaces:**
  - `app.exit` record → validation n/a (closed enums + integer, no input) · instrumentation log✓ · PII redacted✓ (closed
    field set; integration arms assert 0 full data-dir paths + 0 planted canary across the child's whole log family) ·
    tests unit (leaf ×4) + integ (re-exec arms) · a11y n/a · tokens n/a
  - `install_exit_hook` (`libc::atexit` FFI + reporter thread) → validation n/a · instrumentation log✓ · PII n/a ·
    tests integ (arms (c), (e), (e2) on every OS; (b) on Unix) · a11y n/a · tokens n/a
  - `install_signal_listener` (Unix SIGTERM/SIGINT) → validation n/a · instrumentation log✓ · PII n/a · tests integ arm
    (d) — `unrunnable-here` on the Windows host, ran green in WSL (`pre-push:linux`) and on CI lint-test Linux/macOS ·
    a11y n/a · tokens n/a
  - `main.rs` `run_return` tail → validation n/a · instrumentation log✓ (via `exit_after_event_loop`) · PII n/a · tests
    the lib fn witnessed by arms (a)/(e); the `main` composition itself ✗ (not inducible headless — the test-plan §1
    trigger above) · a11y n/a · tokens n/a

## Deviations from intent
- **Arm (b) `cfg(unix)`** — measured: Windows `process::exit` runs no `atexit` (Spec claims disproved 1).
- **Arm (e2) added** to `integration_exit_cause_record.rs` (a listed file) — the plan's once-flag mutation could not
  redden (e) (Spec claims disproved 2); (e2) records then calls `libc::exit`, and reads `record_exit`'s return value;
  RED under the neutralized once-guard (`left: Some(9)`), green restored.
- **Child-ran proof** — the one-time `--no-capture` read replaced by an in-arm assertion that each child's
  `app.boot.tracing.init` record exists (child stdout is `Stdio::null()` by design; a `--no-capture` run shows nothing
  from children).
- **Flush mutation** reddened arm (a) only; (c) stays green (the hand-off wait lets the worker write) — the plan's
  "(a) or (c)" criterion is met by (a).
- **Plan `timeout` raised 3600 → 5400** on the targeted nextest entry — operator directive at this wrap ("Raise the
  3600 s gate timeout per the measured 3565 s run").
- **`incident_events` disposition** — the plan carried it as a NAMED GAP awaiting the founder's word. The overseer's
  first wrap directive (a named 0.4.0 candidate awaiting the founder) was SUPERSEDED mid-wrap by founder ruling
  2026-10-02 (relayed live by the overseer): everything planned for 0.3.0 lands in 0.3.0, nothing moves to 0.4.0. P5
  mints two markerless 0.3.0 entries: (1) incident events readable through MCP — a NEW read surface, on the founder's
  word — with P-075's content fidelity then covering the events and P-075 re-verified after it; (2) the retry-storm
  interpretation names its retry cause (measure first why Conductor d3 rank-1, 2026-10-01, read "Error Rate Spike"
  without retry, then fix). No 0.4.0 residual is written.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 6 · listed 6).

## Decisions & corrections
- Operator: the measure-first probe is accepted and "the Windows atexit reading" is "the decider" (slot #1); the design
  (reporter thread + atexit) was written after it.
- Operator slot discipline held throughout: every cargo build/test and the pulse-app boot ran in a granted slot;
  `cargo clean --profile dev` was the first act of slot #1 (66.5 GiB freed; D: 54 → 114 GB free); the rust-analyzer
  flycheck (`cargo check` children of this session's `rust-analyzer.exe`, pid 24484) was stopped by PID before every
  cargo run — it respawned after EVERY source save (5 respawns stopped during the mutation cycle) and its build-lock
  contention is the likely cause of the ~31 min targeted-gate recompiles.
- Operator: no release rebuild after S; Conductor's round ran on the gate-17 binaries (sha256 cross-checked).
- Operator (overseer): the narrowed P-075 content-fidelity claim stands as concretized at P5 for THIS chunk's verify.
- Founder ruling 2026-10-02 (relayed live by the overseer, superseding the overseer's 0.4.0-candidate directive):
  nothing planned for 0.3.0 is deferred; two 0.3.0 entries are minted at P5 — MCP-readable incident events (the
  founder's word for the new read surface; P-075 re-verified after it) and the retry-storm interpretation naming its
  retry cause (measure first).
- Sweep hazard: a count grep for `2551` matches every `Ed25519` — anchor numeric counts on a word boundary or the
  surrounding words (`'\b2551\b'` or `'2551 →'`).
- Mechanism lesson (P3 raw material): a log sink whose only drain is a guard DROP makes "exactly one record" hold
  structurally after the first flush — a mutation of a once-flag is then invisible to any file read; pin such a flag by
  its return value.
- Mechanism lesson: an `atexit` handler must not log on the exiting thread under glibc (TLS destructors run first; a
  `LocalKey::with` panic in an `extern "C"` handler aborts) — hand off to a thread spawned at install.

## Outcome
- (obs) each loggable class leaves exactly ONE `app.exit` after the child ended — MET: arms (a) ×2, (c), (e), (e2) on
  Windows; (b), (d) + all on WSL Linux; mutation-checked (evidence/mutation-checks.md).
- (tests) the signal arm ends BY signal 15 — MET on WSL (`pre-push:linux`) and CI lint-test Linux/macOS; not runnable on
  the Windows host.
- (tests) RED at base recorded — MET: arms (a)-(c) at base read 0 `app.exit` (children ran: init record 1 each);
  flush-race tally 0/20 landed (emit then `process::exit`), control 20/20 (guard dropped first) — evidence/red-probe.md.
- (obs) exact leaf, set equality both ways, no-bare-`app` discriminator, mutation-checked — MET.
- (security) 0 full paths, 0 canary, closed four fields — MET (asserted in every arm).
- (security) `cargo deny check bans licenses sources` and `cargo audit` as separate invocations — MET (green).
- (arch) no capability/bundle/webview/library-crate change; bindings byte-identical to `a2addb3` — MET (base-diff probe:
  no output; bindings `git diff --quiet` exit 0).
- (layouts) a zero-code event-loop end (Tray Quit shape) writes INFO — MET (arm (a) code-0 child); window close stays
  hide-to-tray by construction (`on_window_event` untouched).
- (tests) standard gate set in order, boot smoke 0 panic / 0 ERROR, CI green on the pushed sha — MET.
- **P-075** — MET: Conductor 6/6 PASS at `2a494804` on S; `ref` written (status implemented) on the overseer's word.
- Gates (implement P2 `.andromeda/runs/2026-10-01T20-09-57Z-implement`; re-run after (e2) for the touched entries):
  `cargo fmt --check` green · `cargo clippy --workspace --all-targets --all-features -- -D warnings` green ·
  `cargo nextest run --workspace --profile ci -E 'test(/app_exit|exit_cause/)'` green (9/9; 3564.77 s first run) ·
  `cargo nextest list … -E 'test(/app_exit|exit_cause/)'` green (contains both tokens; 9 named) · `git diff --name-only
  a2addb3… -- deny.toml pulse-app/capabilities pulse-app/tauri.conf.json pulse-app/ui crates` green (no output) ·
  `cargo deny check bans licenses sources` green · `cargo audit` green · `cargo xtask check:english-sources` green
  (`"verdict": "clean"`, 481 files) · `cargo xtask capability-widening-check` green · `cargo xtask check:ingest-progress`
  green · `cargo xtask check:staged-artifacts` green · `cargo xtask capability-drift` green · `cargo xtask
  verify:capability-matrix` green · `cargo nextest run --workspace --profile ci` green (2560/2560) · `cargo nextest run -p
  pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` green · `git diff --quiet a2addb3… --
  pulse-app/ui/src/bindings/index.ts` green · `cargo build --workspace --release --features mcp-server` green (645.82 s)
  · the `:4317` liveness probe green · the boot-health smoke green (`exit 1` / `last line 0`) · `gate.py hygiene`
  (leg operator) green — `hygiene: clean` · `cargo xtask pre-push:linux` (leg operator) green — `"verdict": "green"`,
  2562/2562 · `git rev-parse HEAD` (leg operator) → S `03ec944…` · the push (leg operator) exit 0 `a2addb3..03ec944` ·
  `ci.py conclusion --sha HEAD --wait 2400` (leg operator) green — `verdict: green`, `ci#36964436269` · the Conductor
  evidence grep (leg operator, `<id>` = S) exit 0, 3 hits. Operator-leg results: `evidence/operator-pass.md`.
- Smoke (boot-path changed): green — release `pulse-app.exe` booted by path on a fresh once-exported data dir, ready
  ~1 s, 0 `app.panic.fatal` / 0 ERROR over ~4.5k records, torn down by pid, 4317/4318 refused.
- watches: none folded.
- Outcome basis: the operator pass ran (pre-CI commit `03ec944`); its final HEAD's CI is `ci#36964436269` green on S,
  recorded in `evidence/operator-pass.md`; implement's conversation is present (this session).
- Process hygiene (implement P4 census + this wrap, re-measured): `cargo clean` — terminated; probe/gate cargo runs —
  terminated; release `pulse-app.exe` pid 42076 (this run) — terminated by pid, ports refused; rust-analyzer flycheck
  cargo trees (respawned by the IDE server, not by this run) — stopped by pid before each cargo run; Conductor's round
  processes (conductor-builder) — terminated per its ledger census. No `pulse-app` / `andromeda-pulse-mcp` process
  present at this wrap's read.
