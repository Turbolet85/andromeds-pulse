# Codebase Research — 2026-10-10-engine-harness-proofs-completed

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 19 · **Hand readings on a spawned engine:** 5 runs (below)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — whole, its 25 Session Additions included;
  7 applied (2026-08-23 the `run` verb is the suite and one exported data dir per leg · 2026-08-28 a leg's window
  outlasts the threshold its verdict reads · 2026-08-28 each arm proves its own precondition · 2026-08-29 a
  harness's negative finding needs a second source · 2026-08-30 cross-invocation state is never keyed on `$$` ·
  2026-10-10 the arm-line shape of `check:engine-log` · 2026-10-10 a count is read from a tool's summary line).
  `.claude/rules/testing.md` — whole; applied: sibling-binary path resolution (2026-05-13), narrowing with `-E`
  under `--workspace`, mutation-applied-before-read (2026-08-17 and its 2026-10-04 extension), a clean skip is
  invisible to nextest's counts (2026-10-04).
- **Platform issues consulted:** none — no runner-only bullet (the one CI run read, `ci#38072937534`, is in
  progress, not red) and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's phase directive, six items (what each proof is read on; the
  pre-push sixth stage as the P4 question; other builders on this host; readings, not repairs; boundaries; the P5
  stop) · `inputs#I2` — the invocation directive naming it.

## Files inspected
- `xtask/src/pre_push.rs` (1-560) — the six stages (`Stage::ALL`, `:207-214`), the stage child set (`stage_env`,
  `:352-368`), the sixth stage (`:278-281`: `seed` then `cargo xtask ci-gates`), the seed (`SEED_LOG`, `:32-39`:
  one `app.boot.ready` record and two `ingest.tick` records 15 s apart; `seed`, `:520-525`), the header sentence
  "No stage starts pulse-app or binds a port" (`:14`), the nine reasons (`all-stages-ok` and eight stops), every
  stage failure being red (`Stop::red`, `:182`).
- `xtask/src/engine_cycle.rs` (1-470) — `drive` (`:217-301`), the three labels `refusal` returns (`:105-118`) and
  the four `drive` alone returns (`data-dir-unusable` `:219`, `:238`, `:239`; `home-unset` `:236`; `path-unset`
  `:237`; `injector-build-failed` `:258`), all before `boot` (`:261`); `child_env` (`:127-155`); `decide`
  (`:159-179`: an ERROR record or a witness file fails the cycle); the injector spawned straight after a ready
  boot (`:262-263`).
- `xtask/src/engine_log.rs` (1-470) — `evaluate` builds seven arms in order (`:156-164`): `family`, `program`,
  `panic`, `heartbeat-gap`, `progress`, `process-end`, `budget`; each arm's decision is its own function
  (`family_arm` `:179`, `program_arm` `:201`, `panic_arm` `:231`, `tick_gap` `:248` under `heartbeat_arm` `:285`,
  `progress_arm` `:298`, `process_end_arm` `:345`, `budget_arm` `:383`); `run_check` (`:416`) is the verb glue.
- `xtask/src/harness_ready.rs` (60-195) — `decide_ready` (`:156-164`): `ready` is the status verdict
  `running-healthy` and a TCP connection accepted on both resolved ports (`receiver_accepts`, `:177-180`); it does
  not read whose listener accepted. `resolve_port` (`:166-171`) takes any number but 0, so a privileged number
  the product rejects is still probed; an unparseable value is `cannot-evaluate`.
- `scripts/agent-run.sh` (whole, 343 lines) — the word arms (`:55-64`, usage `:38-49`, before `mkdir` `:66`);
  `boot`'s failure exits (witness library `:90-93`, engine build `:100-104`, xtask build `:107-111`, binary not
  found `:119-122`, no spawn record `:159-162`, readiness timeout `:186-208` with `app still running` `:192`,
  the receivers line `:193-197`, `app ended:` `:203`, the self-run cleanup `:207`); `cleanup`'s four tokens
  (`:293-302`) from `kill -0` and a TCP probe of both resolved ports; `logs`' three bases (`:321-331`), newest
  match within the first base that resolves, `tail -F`, exit 1 with a diagnostic when none resolves (`:335-336`).
- `pulse-app/src/engine_boot.rs` (128-175, 846-1030) — `resolve_port` (`:142-173`): an unparseable or
  out-of-range value is `Err`; `start` records a `None` port as `BindStatus::Failed("invalid_port")` and emits
  an ERROR `app.boot.otlp.{grpc,http}.bind` record with `reason = "invalid_port"` (`:852-875`); a bind that fails
  on a held port emits the same target at ERROR with the OS reason and the engine keeps running (`:971-980`,
  `:1009-1018`).
- `pulse-app/src/observability.rs` (622-625 and the http twin) — `app.boot.otlp.grpc.bind` and
  `app.boot.otlp.http.bind` each have an exact allowlist leaf `{bind_address, reason}`, so both fields render.
- `crates/corpus/src/keychain.rs` (100-345) — `resolve_lock_dir` (`:131-151`), `default_lock_dir` (`:179-181`),
  the lock file opened with mode `0o600` before any credential-store call (`:266-277`), never deleted.
- `pulse-app/tests/integration_console_engine.rs` (1-165 and its outline) — the spawned-program form
  (`Running::spawn(extra_env)`, `:76-102`): a cleared environment holding the data dir, two picked ports and the
  passphrase canary; no temp-dir variable and no runtime dir. Four arms; none names a rejected port, a socket
  list or the lock file.
- `pulse-app/tests/integration_engine_boot.rs` (outline) — four in-process arms, each with `grpc_port: Some(..)`
  and `http_port: Some(..)` (`:152-153`).
- `pulse-app/tests/quality_gate_workflow.rs` (790-900) and `xtask/src/main.rs` (925-1000) — the two committed
  precedents of a Rust test that runs a committed script with `bash` and asserts its exit code and its output.
- `xtask/src/source_lint.rs` (24-30), `xtask/ci/quarantine-tracking-check.sh` (19-24) and `.ps1` (11-14) — the
  two source lints' scan sets; both hold `xtask/src` and neither holds `xtask/tests`.
- `xtask/Cargo.toml` — one `[[bin]]`, no `tests/` directory; `tempfile` and `serde_json` are normal dependencies.
- `.config/nextest.toml` — the `ci` profile ends a test at 180 s (`slow-timeout` 60 s, `terminate-after` 3).
- The chunk `2026-10-09-pre-push-check-native-on-linux`: `plan.md:20` ("no stage starts `pulse-app` or binds a
  port"), `plan.md:402` ("Do not bind a port or start `pulse-app` from the verb"), `report.md:85`.
- `verification-matrix.json#P-103` (`matrix.py show --id P-103`) — `verified`; its acceptance names the six
  stages in order and "one JSON verdict reading `green` / `all-stages-ok`"; it states nothing about ports or
  about what the `ci-gates` stage reads.

## Hand readings (P3, 2026-10-10, the release engine built at the chunk base tree; each n = 1)
Every engine was the reading script's own child on picked loopback ports, ended by SIGTERM by its pid; no engine
was left running (`ps`, 0 after each script).
- **Socket census** (the process's socket inodes from `/proc/{pid}/fd`, matched in `/proc/net/tcp`, `tcp6`,
  `udp`, `udp6` and `unix`): two TCP sockets, both `LISTEN`, both on `127.0.0.1`, at the two configured ports;
  two unnamed unix sockets, not listening (a connected pair); no UDP socket, no IPv6 socket; 0 socket inodes
  outside the five tables. The same on the run given a runtime dir.
- **Rejected port** (the gRPC variable set to a free port's digits followed by `/tcp`): one ERROR record
  `app.boot.otlp.grpc.bind` with `reason` rendered as `invalid_port` and no `bind_address`; the census read one
  TCP listener, the HTTP receiver's on `127.0.0.1`; a loopback connection to the digits' port was refused while
  the engine ran; the engine ran on and ended by SIGTERM.
- **Lock file's place.** With a temp-dir variable supplied and no runtime dir: the file
  `andromeda-pulse-corpus-key-37a1cf5bab3fb6a4.lock`, mode `0o600`, 0 bytes, is the supplied temp dir's only
  entry. With a runtime dir supplied: the file is that directory's only entry and the temp dir is empty. In
  neither is it under the data dir, whose top entries are `corpus`, `logs`, `run`. The lock file's name and the
  temp dir's path appear 0 times in the run's log family. The log family is one date-suffixed member
  (`agent-latest.jsonl.2026-10-10`), no bare name.
- **Where the file lands under the existing spawned form.** The spawned tests and the engine cycle hand the
  engine neither a temp-dir variable nor a runtime dir, so the rule yields the shared temp dir: `ls` on this host
  shows `/tmp/andromeda-pulse-corpus-key-37a1cf5bab3fb6a4.lock` (mode `0o600`, 0 bytes, dated 2026-10-10 15:26).
  security-plan §Data Protection records the shared-temp-dir case as a stated residual of the lock-dir rule
  (security extract, Constraints, "Lock file's place"); the directive names the same case as `One place on a
  node`'s (inputs#I1 item 4). Recorded, not repaired; brought at P5.
- **A held port.** With the gRPC port held by the script's own listener: the engine kept running and logged an
  ERROR `app.boot.otlp.grpc.bind` (`reason`: the OS's address-in-use text, `bind_address` rendered) and an ERROR
  `connection.state.transition`; `xtask harness:ready --program console` over that run exited 0 with
  `"verdict": "ready"`, `"otlp_grpc": "accepting"`: the accepted connection was the holder's, not the engine's.
  So today `agent-run.sh boot engine` would print ready on a held port and `harness:engine-cycle` would start its
  injector against the holder; the cycle would then read `fail` (an ERROR record, no span landed) without naming
  the cause. The window program's `boot` reads readiness through the same decision.
- **`cleanup`'s four tokens** (the script run by `bash` on scratch data dirs, the test's own child and listener):
  no pidfile and both ports refusing → `cleanup: clean`, exit 0; no pidfile and one port accepting →
  `cleanup: no-pid-ports-accepting`, exit 1; a pidfile naming a dead pid and one port accepting →
  `cleanup: ports-lingering`, exit 1; a pidfile naming a child its parent never reaps → `cleanup: app-survived`,
  exit 1 (the script's `kill -0` reads an unreaped child as alive; `harness:status` reads such a pid as dead); a
  pidfile naming a live child that is reaped → `cleanup: clean`, exit 0, the pidfile removed.

## Graph impact (rust plane; `tree-query-2026-10-10-engine-harness-proofs-completed.json`, 28 rows)
- **`drive`** (`xtask/src/engine_cycle.rs`) — 1 caller: `run` @ `xtask/src/engine_cycle.rs:89`. A refusal added
  before the boot changes no signature.
- **`refusal`** — 1 production caller: `drive` @ `xtask/src/engine_cycle.rs:225`; 4 test callers in the same file
  (`:358`, `:373`, `:383`, `:391`, `:394`). A fourth argument, if the taken-port refusal goes through it, moves
  those five call sites, all in the file.
- **`child_env`** — `drive` @ `:241` and the test helper `env_of` @ `:416`. Unchanged by this chunk.
- **`seed`** (`xtask/src/pre_push.rs`) — 1 caller: `Stage::run` @ `xtask/src/pre_push.rs:279`.
- **`drive`** (`xtask/src/pre_push.rs`) — 1 caller: `run` @ `xtask/src/pre_push.rs:101`.
- No caller outside the two files: the changes have no cross-file blast radius in xtask, and no crate depends on
  xtask.

## Patterns detected
- **A Rust test runs a committed script with `bash`** (`pulse-app/tests/quality_gate_workflow.rs:834-859`;
  `xtask/src/main.rs:954-969`): a scratch root, the exit code and the merged output asserted, a missing `bash`
  fails and never skips. No test framework for shell is in the tree.
- **The spawned-program form** (`pulse-app/tests/integration_console_engine.rs:44-102`): the built
  `andromeda-pulse-engine` under a cleared environment, extra variables passed per arm, the child killed on drop,
  readiness from ports accepting, records awaited by target under a bound, the log read after the end.
- **A refusal before anything starts** (`xtask/src/engine_cycle.rs:225-233`, `:91-93`): one closed label on
  stderr as `harness:engine-cycle: cannot-evaluate ({label})`, the verdict object printed with nulls, exit 2.
- **A re-executed child under a constructed environment** (`pulse-app/tests/integration_engine_boot.rs:436-468`;
  `crates/corpus/src/keychain.rs:580-597`): the in-tree way to read what a process does with a variable unset,
  without mutating the test process's own environment.

## Conventions to follow
- **`pulse-app` probes live in `pulse-app/tests/*.rs`** (its lib sources hold zero tests, held by a ratchet); a
  spawned reading of the engine goes into `integration_console_engine.rs`, where `CARGO_BIN_EXE_andromeda-pulse-engine`
  is set.
- **A test that needs the built `xtask` binary is an integration test of the xtask package**: Cargo builds a
  package's plain binary for a test run only when that package has an integration test, and
  `CARGO_BIN_EXE_xtask` is set for it alone. `xtask` has none today, so `target/debug/xtask` is whatever an
  earlier `cargo xtask` call left.
- **A sibling package's binary is found by path from the test binary's own profile directory**, never by a fixed
  `target/debug` (the coverage job builds into another target dir); in a `--workspace` run the engine binary is
  built because `pulse-app` has integration tests. A missing binary fails the test with the reason.
- **A narrowed nextest run uses `-E` under `--workspace`**, with `--no-tests=fail`.
- **Verdicts carry closed labels, no path and no environment value**; a new label joins a pinned set.

## What each owed proof is read on (inputs#I1 item 1)
- **Rejected port, socket census, lock file's directory** — a spawned engine (three new arms of the spawned
  form). No stand-in.
- **One mutation per arm of `check:engine-log`** — the written code mutated once per arm; read twice: the unit
  pins that redden, and the arm's own line when the mutated check grades the kept log of a real engine cycle run
  in this chunk. The pins' inputs are constructed logs; the cycle's log is real.
- **The four refusal labels of `drive`** — the built verb itself, run under a constructed environment or
  argument. A refusal is returned before anything starts, so no engine exists to read: a constructed
  environment is the only reading there is.
- **Shell pins of `cleanup`, `boot`, `logs` and the word arms** — the committed script run by `bash` against a
  real spawned engine, the built `xtask` answering `harness:ready` and `harness:status`. Stand-ins, named: a
  `cargo` on the test's PATH that skips the pre-build (or fails it, for the build-failure arm) and hands
  `cargo xtask` to the built binary; the test's own listener for the two port tokens; the test's own unreaped
  child for `app-survived`. No real process survives SIGKILL, so that token has no reading on an engine.

## New files to create
- `xtask/tests/` — the xtask package's first integration tests: the shell-level pins of `scripts/agent-run.sh` and the refusal-label pins of `harness:engine-cycle`, each run against the built binaries

## Files to modify
- `pulse-app/tests/integration_console_engine.rs` — three new spawned arms: the rejected port, the socket census, the lock file's directory
- `xtask/src/engine_cycle.rs` — a refusal for a port already taken, made before anything starts, and its pins
- `xtask/src/harness_ready.rs` — ready only over the run's own bound records, a third receiver label, and their pins
- `scripts/agent-run.sh` — the boot failure branch names a port that accepts but is not this run's
- `xtask/src/pre_push.rs` — the sixth stage runs a real engine cycle and then `ci-gates` over its log, with the header sentence, the reasons and the pins that state it
- `xtask/src/source_lint.rs` — the scan set gains `xtask/tests`
- `xtask/ci/quarantine-tracking-check.sh` — the search dirs gain `xtask/tests`
- `xtask/ci/quarantine-tracking-check.ps1` — the same one entry, the mirror
- `xtask/src/main.rs` — the quarantine pins' search-dir list gains `xtask/tests`

## Open questions
- none — the two plan-decision questions P3 carried were answered by the operator at P4 (inputs#I3): the sixth
  stage runs a real engine cycle and then `ci-gates` over its log (recorded PROVISIONAL, the founder's own word
  owed); the readiness verb is fixed at its source for both programs, which adds `xtask/src/harness_ready.rs`
  and `scripts/agent-run.sh` to the list above. Two facts read for that second answer after P3
  (`xtask/src/harness_ready.rs:50-87`, and the bound record as a hand-run engine wrote it): `run_ready` already
  resolves the log base and reads the family's last `app.boot.engine` record (`read_program`); a bound record is
  an INFO record of target `app.boot.otlp.grpc.bind` or `app.boot.otlp.http.bind` whose `bind_address` is
  `127.0.0.1:` and the port. The three clauses of the shell-coverage row this chunk does not build have owners
  on the route (the `.ps1` leg on `working-route.md:64`; the witness-arm branches and the boot series on `:50`).
