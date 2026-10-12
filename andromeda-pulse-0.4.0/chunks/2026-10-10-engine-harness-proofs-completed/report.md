# Report — 2026-10-10-engine-harness-proofs-completed

**Chunk:** rejected port, socket census, lock-file place read on a spawned engine; pre-push reads a real engine cycle; cleanup, boot, logs verbs pinned in shell
**Date:** 2026-10-10T19:52:10Z
**Commits:** `d30b7037 chore(2026-10-10-engine-harness-proofs-completed): operator pre-CI commit` (the one commit since
the chunk base `514fbe57`; basis: `git log --format='%h %s' 514fbe57..HEAD`)

**This report was written in the wrap's first window, which ends at P1** (inputs#I5 item 2, inputs#I6). The
implement conversation and the operator pass are in that window and are cleared before P2. What curation needs from
them is in "Decisions & corrections"; what the later phases owe on the operator's directive is in "Carried for the
second window" at the end of that section.

## Changes (structured — detectors read this)
- **Files:** 12, all inside research's two lists (basis: `gate.py scope`, base `514fbe57`: `changed 12 · listed 12 ·
  recorded 0`). Modified (9): `pulse-app/tests/integration_console_engine.rs`, `scripts/agent-run.sh`,
  `xtask/ci/quarantine-tracking-check.sh`, `xtask/ci/quarantine-tracking-check.ps1`, `xtask/src/engine_cycle.rs`,
  `xtask/src/harness_ready.rs`, `xtask/src/main.rs`, `xtask/src/pre_push.rs`, `xtask/src/source_lint.rs`. New (3):
  `xtask/tests/agent_run_shell.rs`, `xtask/tests/engine_cycle_refusals.rs`, `xtask/tests/common/mod.rs`. Untouched,
  each read by `git diff --quiet 514fbe57 --` at implement (exit 0): `xtask/src/engine_log.rs`,
  `pulse-app/ui/src/bindings/index.ts`, `scripts/agent-run.ps1`, `Cargo.toml`, `Cargo.lock`, `xtask/Cargo.toml`. No
  file under `pulse-app/src`, `crates`, `.github` or `pulse-app/capabilities`.
- **Symbols / APIs:** (every coordinate is copied from "New text, by line")
  - `xtask/src/harness_ready.rs` — `enum Receiver` (172-180; `Accepting` · `Foreign` · `Refusing`), `impl Receiver`
    (182-198: `read(accepted, bound_by_this_run)`, `label()`), `fn bound_by_this_run(lines, target, port)` (200-227).
    `decide_ready` now takes two `Receiver` values in place of two booleans; its sole caller is `run_ready`.
    `receiver_accepts` is `pub(crate)` now and has three callers: `run_ready`, `engine_cycle::drive`,
    `pre_push::read_engine_cycle`. `receiver_label` is removed. `run_ready` probes both ports, then reads the log
    family once (`smoke::read_jsonl_lines`) and takes the program from `harness_status::program_of`;
    `harness_status::read_program` keeps its one caller, `harness_status::run`. Two constants name the bind targets
    (`app.boot.otlp.grpc.bind`, `app.boot.otlp.http.bind`).
  - **The readiness decision, as built:** `ready` only when the status verdict is `running-healthy` and both
    receivers read `accepting`. A receiver reads `accepting` when a loopback connection is accepted on the resolved
    port AND the log family holds, after its last `app.boot.engine` record, an INFO record of that receiver's bind
    target whose `bind_address` is `127.0.0.1:` and that port; `foreign` when the connection is accepted and no such
    record stands (a failed-bind ERROR record, a record for another port, a record of the other receiver, a record
    before the last boot record, and a log with no boot record at all each read `foreign`); `refusing` when no
    connection is accepted. Verdict words, exits and the six members are unchanged. It holds for both programs.
  - `xtask/src/engine_cycle.rs` — `fn taken_port_refusal(grpc_accepting, http_accepting)` (122-127), called in
    `drive` (243-250) after the existing `refusal` and before the `HOME` read, so 4317 and 4318 are refused before
    any port is probed. New closed label `port-taken`: stderr `harness:engine-cycle: cannot-evaluate (port-taken)`,
    the verdict object with nulls, exit 2, nothing built or started, no data dir made.
  - `xtask/src/pre_push.rs` — `Stop` derives `Debug, PartialEq, Eq`; `Stage::run` returns `Result<(), Stop>` (was
    `Result<(), String>`; sole caller `drive`); `Stage::failed` (283-285); `struct CycleReading` (288-303);
    `fn decide_ci_gates` (305-320); `fn read_engine_cycle` (322-369); `fn end_cycle` (inside the added range
    282-384; its `cleanup` call is 375-383). `SEED_LOG` and `seed` are removed (`seed`'s sole caller was
    `Stage::run`). Two new closed reasons: `engine-port-taken`, `engine-cycle-unevaluated`.
  - `scripts/agent-run.sh` 196-200 — a second `case` in the readiness-failure branch: when the last readiness
    verdict held `"foreign"` the verb prints `  a receiver port accepts but this run's log holds no bound record for
    it: OTLP gRPC 127.0.0.1:{port}, OTLP HTTP 127.0.0.1:{port}`. The `refusing` line beside it is unchanged; both
    can print.
  - `xtask/src/main.rs` — three help strings changed (lines 67, 83, 291: `harness:ready`, `harness:engine-cycle`,
    `pre-push:linux`); `QUARANTINE_SEARCH_DIRS` is `[&str; 5]` (939-945).
  - `xtask/src/source_lint.rs` — `ROOTS` is `[&str; 6]` (lines 24, 30), gaining `xtask/tests`.
  - No new `ANDROMEDA_PULSE_*` name, no new xtask verb or flag, no TauRPC procedure, no capability, no engine
    command word, no product port.
  - Variables the new tests hand a child (test code only): `TMPDIR` and `XDG_RUNTIME_DIR` to the spawned engine in
    the lock-file arm alone; `HOME`, `PATH`, `TMPDIR`, the data dir, the two port variables, the passphrase variable
    and `HARNESS_STATUS_TIMEOUT` to the script; `LLVM_PROFILE_FILE` passed through when the test process holds it.
- **Crates / modules:** no workspace member added. The `xtask` package gains its first integration tests: two test
  binaries (`agent_run_shell`, `engine_cycle_refusals`) and a shared module `xtask/tests/common/mod.rs`
  (`#![allow(dead_code)]`, since each binary uses a part of it).
- **Dependencies:** none added or bumped (`Cargo.toml`, `Cargo.lock`, `xtask/Cargo.toml` unchanged). The new xtask
  tests use `tempfile` and `serde_json`, already normal dependencies of the package, and no `libc`.
- **Schema / config:** the `harness:ready` verdict's two receiver members take one of three closed labels
  (`accepting` · `foreign` · `refusing`), was two. No log record, allowlist leaf, redaction shape or config key
  changed.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** (docs stating each are the sites of "Expected amendments")
  - refusal labels of `harness:engine-cycle`: 7 → 8 (`port-taken`; basis: the eight literals in
    `xtask/src/engine_cycle.rs`).
  - reasons of `pre-push:linux`: 9 → 11 (`engine-port-taken`, `engine-cycle-unevaluated`).
  - receiver labels of `harness:ready`: 2 → 3 (`foreign`).
  - English-only lint roots: 5 → 6; quarantine search dirs: 4 → 5 (both gain `xtask/tests`). The quarantine check's
    PASS line read `0 quarantine(s) across 327 file(s)` at implement.
  - workspace tests under the `ci` profile: 3064 (the previous chunk's operator-pass record) → 3112 (this chunk's
    final tree and `ci#38079305603`): +48 — 20 xtask unit pins (`harness_ready` 8, `engine_cycle` 2, `pre_push` 8,
    `main` 1, `source_lint` 1), 25 xtask integration tests (19 + 6), 3 spawned arms.
  - arms of `pulse-app/tests/integration_console_engine.rs`: 4 → 7. `pulse-app/tests/*.rs`: 109, unchanged
    (`ls pulse-app/tests/*.rs | wc -l`).
  - xtask integration test binaries: 0 → 2.
  - `pre-push:linux` wall-clock on the warm dev host: 89 s on record (one reading, stage six over a seed) → 107.04,
    108.13, 109.07 and 110.44 s (four readings, the engine already built in release); its sixth stage 18.8 to 19.0 s.
    No cold release build was timed.
  - what stage six of `pre-push:linux` reads: a seed of three records → the log family of a real console engine
    cycle (335 records in each of the four readings).
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed. `pwsh` parsed the `.ps1` mirror
  (0 parse errors); its version was not re-read.
- **Harness / gate surface:**
  - `cargo xtask harness:ready` — the decision above; one more label. Both scripts call it; `agent-run.ps1` is not
    edited and its readiness branch still matches `refusing` alone.
  - `scripts/agent-run.sh boot` (both programs) — one more failure line. Five verbs, the word set, `cleanup` and
    `logs` as they stood.
  - `cargo xtask harness:engine-cycle` — one more refusal before anything starts (`port-taken`).
  - `cargo xtask pre-push:linux` — stage six, in order: the stage data dir removed and created again; either cycle
    port (24317, 24318) accepting stops the run `cannot-evaluate` / `engine-port-taken` with nothing started;
    `cargo xtask harness:engine-cycle --data-dir {that dir}` under the stage child set; then both ports and the pid
    of that data dir's pid file are read: one still accepting or alive → the script's `cleanup` runs once on that
    data dir and the stage is red (`stage-failed:ci-gates`) whatever the cycle returned; else cycle exit 2 →
    `cannot-evaluate` / `engine-cycle-unevaluated`; exit 0 → `cargo xtask ci-gates` over the same data dir decides
    the stage; any other exit → red. Stage names, order and the six verdict members are unchanged; a stage that
    stops `cannot-evaluate` is listed `"ok": false`. **The check now starts `andromeda-pulse-engine` and binds
    loopback 24317 and 24318.** The stage child set is unchanged (`HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`); the
    one cleanup child also carries the two cycle port variables, so its probe never touches 4317 or 4318. The
    cycle's engine writes its corpus-key lock file to the temp dir (the cycle's child set holds no runtime dir).
    This change is the operator's reading (the pc overseer, 2026-10-10, inputs#I3), **PROVISIONAL, the founder's own
    word owed** (playbook pattern: Boundary widening).
  - `cargo xtask check:english-sources` and `cargo xtask quarantine-tracking` (the `.sh`; the `.ps1` mirror takes
    the same entry) read `xtask/tests`. A missing `xtask/tests` now fails both (a root absent is `cannot-evaluate`
    for the lint; a missing search dir fails the quarantine check).
  - The `ci` workflow is not edited. Its `lint / test`, `mcp-server tests` and `coverage gate` jobs now run the new
    tests, which need `bash`, `/proc`, an advisory file lock under `target/` and the engine built in the test
    profile (a `--workspace` run builds it).
  - Test-side surface: every port the three binaries `agent_run_shell`, `engine_cycle_refusals` and
    `integration_console_engine` use is claimed, not bound-to-0-and-released: a number from 20000 to 31999 (24317
    and 24318 left out), held by an advisory lock (`File::try_lock`) on a file under `target/test-ports/`
    (git-ignored, never unlinked: 1 350 empty files after implement's runs), shown bindable when claimed. This
    changes how the four existing spawned arms take their ports; what the engine is handed is unchanged in kind.
- **Cross-project / external claims:**
  - **CI:** `ci#38079305603` (event `pull_request`, attempt 1) measured `d30b70375eac1c7d5b41524fcd56d359feaa0d60`,
    the pre-CI commit: `verdict: green · checks 7/7`, all six jobs of the `ci` workflow `completed/success`,
    `secret-scan#38079305747` success. Repo `Turbolet85/andromeds-pulse`. The runner builds the tip merged with
    `main` (the boot record's `git.commit.sha` reads `b61f830d…`). This wrap's own commit is not covered by it.
    Record: `evidence/operator-pass.md`.
  - Artifacts of that run: `logs-boot-Linux` `11680211947`, `logs-engine-Linux` `11679807527`, both expiring
    2026-10-24; downloaded to `target/boot-smoke/ci-38079305603-attempt-1/` and
    `target/engine-cycle/ci-38079305603-attempt-1/` (git-ignored).
  - Inputs, from `inputs.py verify` at this wrap (`6 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 ·
    n/a 4 · uncited 3 · unparsed 0`):
    - I1 · `../additional/pc-overseer/relays/pulse-phase-engine-harness-proofs-2026-10-10.md` · copy · unchanged
    - I2 · the phase invocation message · copy · n/a (a message has no live source)
    - I3 · the two P4 answers · copy · n/a
    - I4 · the operator's word for the operator pass, given in the implement session · copy · n/a — printed
      `UNCITED` before this report; cited here and in `evidence/operator-pass.md` (inputs#I4)
    - I5 · `../additional/pc-overseer/relays/pulse-wrap-engine-harness-proofs-2026-10-10.md` · copy · unchanged —
      snapped at this wrap, cited here (inputs#I5)
    - I6 · the wrap invocation message · copy · n/a — snapped at this wrap, cited here (inputs#I6)
  - inputs#I5 item 3 says the handoff written at `514fbe57` names 19 groups of leaf file-layout lines. Re-derived
    from that handoff's own text (its "Owed to the next wrap's cascade" note): config-watcher 2, triage 1, ingest 3,
    buffer 1, snapshot 3, ui-bridge 2, viz 2, mcp-server 2, workspace-detector 3 = 19, in nine files.
- **Reverted / negative API facts:**
  - Port picking by bind-to-0, read and release was written into `xtask/tests/common/mod.rs` (`pick_port`, as the
    spawned suite already had it) and then replaced in both places by `claim_port` / `PortClaim`: on the tree that
    carried it, one of seven hand runs of the three port-using binaries failed (below, Decisions).
  - `harness:engine-cycle` gained no flag for a root or an engine path. The failing-cycle pin reaches the built
    engine through a stand-in `bash` on its PATH instead (below, Deviations).
  - Seven mutations of `xtask/src/engine_log.rs` and several one-line stand-ins were applied and removed at
    implement; none is in the tree (the file's blob equals the base's, `493d908e`; `grep -rn` for the stand-in mark
    over `xtask`, `scripts`, `pulse-app/tests`: 0 hits).
- **Insufficient fixes (written, kept, not the remedy):** the port claims remove collisions among the three
  binaries that use them. `pulse-app/tests/integration_engine_boot.rs` still picks by bind-and-release (its
  `pick_port`, four in-process arms) and is not in this chunk's lists; any other workspace test that binds port 0
  and holds it is outside the claimed range and cannot collide with a claim. Owner of the remainder: none named
  (for the card).
- **Spec claims disproved by measurement:** (both are statements of this chunk's `plan.md`, a chunk artifact: a
  report entry, no edit)
  - Plan step 6: both new xtask suites "resolve … the built engine by path from the test binary's own profile
    directory". Measured false for the failing-cycle arm: `harness:engine-cycle` runs `scripts/agent-run.sh` from
    the compile-time workspace root, and the script spawns `target/release/andromeda-pulse-engine` relative to its
    working directory, which a test-only job does not build. With the decision stubbed, the port-taken arm's cycle
    booted the release engine of the dev host's own tree (`evidence/red-readings.md`, reading 1).
  - Plan step 6: "Each never-ready arm asserts exit 1 and the self-run cleanup's `cleanup: clean`." Measured false
    for the foreign-listener arm: the test's listener still holds the port when the self-run cleanup probes it, and
    the script prints `cleanup: ports-lingering`. The other three never-ready arms read `clean`.
- **Expected amendments (from plan):** (search: `scratchpad/master_hits.py` at this wrap, one regex per row over
  the seven master bodies and every file under `.andromeda/registries/`, counting matching lines; registry files
  per master: architecture 5, test-plan 8, obs-plan 9, a11y-plan 11)
  - architecture §Occupied Resources → xtask CLI surfaces (the `pre-push:linux` entry, the `ci-gates` caller
    clause, the `harness:engine-cycle` refusal list, the `harness:ready` decision and third label, the
    `agent-run.sh` failure line; Boundary widening, PROVISIONAL) — **carried:** Harness / gate surface, Symbols /
    APIs. Sites: `pre-push:linux` architecture body 3; the no-port sentence
    (`starts no .pulse-app|No stage starts|Binds no port|binds no port|bind a port or start`) body 1;
    `seed log|a seed\b|seed record` body 1; `harness:engine-cycle` body 10 + 1 key file (1 hit);
    `injector-build-failed` body 1; `harness:ready` body 4; `never both accepted` body 1.
  - architecture §Occupied Resources → Network ports, → Filesystem locations, → Environment variables —
    **carried:** Coverage of new surfaces (the census, the lock file) and, for the key path a cycle's engine takes:
    the engine of a cycle and of the spawned suite is handed a passphrase variable and no session bus, so it takes
    the passphrase fallback after opening its lock file (the spawned suite's first arm asserts one
    `corpus.keychain.fallback` record; the lock-file arm waits on that record). Sites:
    `third socket|no other socket|socket census` architecture body 1; `andromeda-pulse-corpus-key-` body 1.
  - architecture §Stack and Technologies → Gate-script pins — **carried:** Crates / modules, Harness / gate surface.
    Site: `Gate-script pins` architecture body 1; `xtask/tests` has 0 hits in the seven masters and their key files.
  - test-plan §1 Pending coverage triggers, the four rows clause by clause — **carried:** Outcome, "The four
    pending rows". Sites: `engine-boot-rejected-port-and-socket-census-coverage` test-plan body 1;
    `harness-cleanup-verdict-and-boot-spawn-shell-coverage` test-plan body 1 + 1 key file (3 hits) and
    security-plan body 2; `harness-log-family-resolution-coverage` test-plan body 2;
    `engine-log-arm-mutation-and-engine-cycle-glue-coverage` test-plan body 1.
  - test-plan §3 (the `pre-push:linux` paragraph, the Engine cycle gate form's refusals, the `boot` readiness
    sentence and failure lines, the Spawned-program witness form's readings) and §2 / §9 (the `xtask/tests`
    directory and what a runner needs) — **carried:** Harness / gate surface, Coverage of new surfaces. Sites:
    `pre-push:linux` test-plan body 1 + 1 key file (2 hits), whose key file also holds the no-port sentence (1 hit)
    and the seed (1 hit); `Engine cycle gate form` body 2 + 2 key files (3 hits); `Spawned-program witness form`
    body 0 + 1 key file (1 hit); `harness:ready` body 2 + 2 key files (4 hits); `never both accepted` body 1 + 1 key
    file (2 hits); `injector-build-failed` body 1 + 1 key file (1 hit).
  - security-plan §Security Anti-Patterns → Input and §Input Validation (the `pre-push:linux` member, the closed
    refusal labels, the readiness verb's read of the log; Boundary widening, PROVISIONAL) — **carried:** Harness /
    gate surface. Sites: `pre-push:linux` security-plan body 3; `harness:engine-cycle` body 3;
    `injector-build-failed` body 1; `harness:ready` body 3; `andromeda-pulse-corpus-key-` body 1.
  - obs-plan §10 CI gates and Load-profile constraints (what the pre-push stage grades; the mutation readings per
    arm) — **carried:** Harness / gate surface and Outcome ("Mutations"). Sites: `check:engine-log` obs-plan body 8 +
    1 key file (1 hit); `harness:engine-cycle` body 1; `pre-push:linux` has 0 hits in obs-plan and its key files, so
    the obs-plan sites are its `check:engine-log` and cycle text.
  - matrix#P-103 notes — **ledger-note — owner P7.3.** Text as the plan carries it (plan.md, the last Expected
    amendments entry), dated 2026-10-10, PROVISIONAL; status, ref and acceptance untouched (inputs#I5 item 4).
- **Coverage of new surfaces:**
  - `harness:ready` reading `foreign` → validation ✓ (the port from `resolve_port`, the log read as JSON lines, a
    line that is no JSON skipped) · instrumentation n/a (a harness verdict; no record written) · PII n/a (labels
    only; no record text, address or path leaves the verb) · tests unit (8 pins) + integ (the foreign boot arm on a
    real console engine; the window program's healthy path in the boot series, local and CI) · a11y n/a · tokens n/a
  - `harness:engine-cycle` refusing `port-taken` → validation ✓ · instrumentation n/a · PII n/a (a closed label) ·
    tests unit (2) + integ (the built verb over the test's own listener, each port in turn) · a11y n/a · tokens n/a
  - `pre-push:linux` stage six → validation ✓ (fixed ports, a taken port refused before anything starts) ·
    instrumentation n/a · PII n/a (the verdict's six members carry no value or path; the cycle's stderr prints the
    data dir through the boot verb's ready line, as it did standalone) · tests unit (the decision, 7 pins; the
    teardown call against a real process, 1 pin) + e2e (the check itself, four green readings) · a11y n/a · tokens
    n/a. Not read live: a taken port on 24317 / 24318, and the stage's teardown after a cycle that left an engine.
  - `agent-run.sh boot` foreign line → tests integ (one arm) · every other flag n/a
  - the three spawned readings (rejected port, socket census, lock file's directory) → tests integ on a spawned
    engine, each read red first · PII ✓ (the lock arm asserts the lock file's name and the lock directory's path,
    as given and as resolved, appear 0 times in the run's log family) · the rest n/a
  - `xtask/tests/` (the shell-level and refusal pins) → tests integ (25) · lints ✓ (English-only and quarantine
    read the directory; one pin each) · the rest n/a
  - test-port claims (`target/test-ports/`) → validation ✓ (range, the two harness ports left out, a bind shown) ·
    tests: no pin of its own (it is exercised by every test that claims) · the rest n/a

## Deviations from intent
The operator read implement's report against the tree and accepted the deviations as recorded (inputs#I4).

- **A second stand-in, `bash`, in the failing-cycle pin** (`Scratch::stand_in_bash`, `xtask/tests/common/mod.rs`
  334-350). The plan named one stand-in, `cargo`. The cycle verb runs the script from the workspace root, where a
  test-only job holds no release engine; the stand-in moves the committed script into the scratch root that links
  the built engine, sets `HARNESS_STATUS_TIMEOUT=60` for it, and hands every other call to the real shell. The
  script is still the committed one, run by the real `bash`, on a real engine.
- **The foreign-listener boot arm pins `cleanup: ports-lingering`**, not `clean` (above, Spec claims disproved).
- **Port claims replace bind-and-release picking in three test binaries**, the four existing spawned arms
  included (above, Harness / gate surface; below, Decisions).
- **A log with no boot record reads `foreign`.** The plan says "after its last `app.boot.engine` record"; with no
  such record nothing stands after it. Pinned (`a_log_with_no_boot_record_holds_no_bound_record_of_this_run`).
- **Unplanned additions, all inside listed files:** the three help strings in `xtask/src/main.rs`, brought in line
  with the changed contracts; one pin per lint for the new directory; one pin that runs the stage's own teardown
  against a real process (`end_cycle_ends_the_process_the_stage_data_dir_s_pid_file_names`, added after the second
  firing of the gate block for the operator's condition "the stage ends what it started whichever way it ends, and
  that is pinned", inputs#I3); the stage's cleanup child carrying the two cycle port variables beyond the stage
  child set.
- **The rejected-port arm reads the refused connection on a claimed port**, not on a bound-and-released one.
- **Red before green, as taken.** New-behaviour pins read red against stand-ins holding the base decision (19 of
  484, then 1 of 484, then 484 of 484; the teardown pin 0 of 1, then 28 of 28 for its module). The 23 pins of the
  two xtask binaries that hold unchanged behaviour have no per-test red: their red reading is the plan's own, the
  gate entry that selects the binaries reading exit 94 at P5 where neither exists.
- **P3 smoke not re-driven by hand:** the window series and the engine cycle are listed entries and ran as gates.
- **The spawned arms print their readings** (`eprintln!`, shown under `--success-output immediate`): ports, the
  census, a bind record's level and reason, the lock file's name, size and mode. No path and no passphrase.

scope record: none — `gate.py scope` clean, 0 recorded (`changed 12 · listed 12 · recorded 0 · absorbed 3 ·
excluded 59`, base `514fbe57`).

## Decisions & corrections
**The operator's words in this window** (each kept verbatim as an input):
- inputs#I4, after implement's report: "Deviations accepted as recorded; the test-port claim files and the heartbeat
  pin that stayed green under an inverted comparison go to the wrap card as found and not owned." Then the operator
  pass, with: "If the boot job, the cycle step or a new test is red on the runner, stop and report what the job kept
  whole; fix nothing on top. Start no skill - the wrap is mine to call."
- inputs#I5 and inputs#I6: this wrap runs in two windows; P1 only in the first.
- No operator correction of the agent's work was given in this window, and no "from now on" convention.

**What was measured, and what was only reasoned** (raw material for curation; each line says which):
- *Measured.* A port bound to 0, read and released is handed to another test when many start together. On this
  host 20 000 binds to port 0 returned 10 667 distinct ports, every one odd, inside 32768 to 60999 (a uniform draw
  over about 14 100 numbers). The 32 tests of the three binaries made about 70 such draws at one instant. One of
  seven hand runs failed one arm (`no gRPC receiver was started`: a port the arm had picked and left unbound
  accepted a connection, while its census read the engine's one listener); the same arm alone passed 60 of 60.
  After the claims: 14 of 14 hand runs, two more firings of the gate block, the operator pass's pre-push check and
  one CI run, all green. That is a count, not an exclusion.
- *Measured.* The first firing of the gate block had already read green on the tree that carried the racy picker;
  the failure was found by running the port-using binaries repeatedly by hand, not by a listed gate.
- *Measured.* An accepted loopback connection does not say whose listener accepted it: with the test's listener on
  the gRPC port, `agent-run.sh boot engine` printed `boot: ready` at the base decision, and the built cycle fed the
  holder and ended `fail` on `progress` and `budget` without naming the cause (`evidence/red-readings.md`).
- *Measured.* `harness:engine-cycle` and the script resolve the engine under the compile-time workspace root's
  `target/release`; a pin of the built cycle verb that must run where only the test profile is built needs the
  script's working directory changed for it. On `ci#38079305603` the pin passed in the `lint / test` job (2.1 s).
- *Measured.* Six of the 45 `engine_log` pins also read the whole verdict, so any arm's FAIL reddens them; and
  `a_gap_over_45_seconds_between_two_ticks_of_one_target_fails_with_no_error_record` stayed green under the
  inverted gap comparison, because its log's two other targets carry ordinary gaps the inversion fails. Nine other
  pins reddened under that mutation. The operator sent this pin to the card as found and not owned (inputs#I4).
- *Measured.* The per-write `rustfmt` hook reflowed fourteen call sites a script had re-pointed, and the next
  anchored Edit missed once.
- *Measured.* The Bash guard refused a leading `cd` twice (into the chunk's `inputs/` at implement, into
  `.andromeda` at this wrap) and a `cat` heredoc with a file target once; each was re-issued once in a form the
  guard passes. Both are entries the host rule file already holds, and the last handoff lists both as recurring.
- *Measured.* A backgrounded `gate.py run` prints nothing until the block ends; progress was read from the
  per-entry log files in the tool's log dir, and the verdict from the summary line.
- *Measured, a count taken from a neighbouring count.* The operator-pass record first said "nine" `xvfb.log` files
  from the nine `boot: ready` lines; the artifact holds eight (the engine cycle has none). Counted with `find`
  and corrected before this report.
- *Measured.* The runner's boot record carries the pull-request merge commit as `git.commit.sha`, not the pushed
  tip; a search of the boot log for the tip's sha reads 0.
- *Reasoned, not measured.* The stand-in executables are written by a child shell so the test process never holds
  one open for writing (a write-then-exec in a threaded test process can meet "text file busy"). No such failure
  was seen; the tests ran under nextest, one test per process.
- *Reasoned, not measured.* A claim file is never unlinked: unlinking a lock file lets two later claimants each
  lock a different inode under one name. No collision was provoked to show it.
- *Procedure that held.* The operator's word for the pass was snapped before the pre-CI commit (the inputs
  manifest is tracked once that commit lands, and a later snap would dirty the tree the push guard reads); the
  operator-pass record was written after the push and left untracked for this wrap.
- **Sweep hazards met:** a job log holds each nextest `PASS` line twice (counts were halved, the summary line
  read); `ls target/boot-smoke | tail` sorts `ci-…` after the dated dirs, so a tail shows only the CI downloads.

**Found and not owned** (for the card; inputs#I4, inputs#I5 items 5 and 6):
- `target/test-ports/`: about 1 350 empty claim files after implement's runs, never removed by a test, removed by
  `cargo clean`. Owner: none.
- The heartbeat pin insensitive to the inverted comparison (above). Owner: none.
- `pulse-app/tests/integration_engine_boot.rs` still picks ports by bind-and-release. Owner: none.
- **The console engine stays up with a receiver that failed to bind** (measured at research, n = 1, and again in
  `evidence/red-readings.md`: one ERROR `app.boot.otlp.grpc.bind`, the engine ticking on, no exit). Lean: an
  engine with one receiver bound is degraded and may stay up, but nothing tells whoever started it; an engine with
  no receiver bound can do nothing and should end non-zero. It is a product behaviour. Requirement text it touches:
  P-086 ("the engine is a console program … driven by commands") and P-098 ("no data reached the engine" is one of
  three states told apart). Entry it would be pinned on: `One place on a node` (`working-route.md:41` at this
  wrap), which owns how the engine is started and ended on a node; the operator places it.
- **The corpus-key lock file lands in the shared temp dir** under the environment the existing spawned arms and
  the cycle's children hand the engine (read on this host after the gate block:
  `andromeda-pulse-corpus-key-37a1cf5bab3fb6a4.lock` in the system temp dir). security-plan records the case as a
  stated residual; the directive names it as `One place on a node`'s.
- **`cleanup` reads an unreaped child as alive** (`kill -0`), while `harness:status` reads such a pid as dead.
  Pinned as it stands (`cleanup_with_an_unreaped_child_in_the_pidfile_is_app_survived`). Owner: to be named at
  the card, or nothing owed with the reason.
- **`boot` with no spawn record exits 1 and leaves the engine it started running.** Pinned as it stands
  (`boot_with_no_spawn_record_exits_one_and_leaves_the_engine_it_started`; the pin ends that engine through the
  product's own pid file). Owner: to be named at the card, or nothing owed with the reason.

**Carried for the second window** (inputs#I5; none of it is done in this window):
- Item 3: the nine leaf files and 19 groups of file-layout lines the handoff at `514fbe57` names
  (`git show 514fbe57:.claude/session-handoff.md`, the note "Owed to the next wrap's cascade") are each corrected
  from a read of the source tree and named in this wrap's `cascade-dispositions.md`. Not carried a second time.
- Item 4: wherever a master or the check's own contract said no stage starts a program or binds a port, the
  amended sentence says what the stage does now, that it is the operator's reading (the pc overseer, 2026-10-10),
  and that the founder's own word is owed. P-103 gets its dated note; its acceptance is not re-worded.
- Item 7: the four new closed labels (`foreign`, `port-taken`, `engine-port-taken`, `engine-cycle-unevaluated`)
  and the readiness verdict's changed decision are registered where the masters list verdict words; the
  shell-coverage row stays open on its three clauses with their owners.
- Item 8, for the main overseer: no `REFUSED id:` was printed in any trail of the implement run or of this
  window's first steps (three evolve appends at implement landed on their first call). `held` at the citation
  sweep is the second window's to read.
- Item 9, stops: after P1 (this one); the route-resolve card; before the flip and the commit with
  `git status --short`.
- The handoff still lists four `recurrence-despite-learning` items and the stale citations it surfaced; this
  window adds two recurrences to the host-leaf pair (above).

## Outcome
**Acceptance criteria, each re-asserted against the diff:**
1. *Each owed clause has a committed test in the workspace run, read red before green* — met, with the stated
   limit on what "red" was for the 23 pins of unchanged behaviour (Deviations). The tests: the three spawned arms;
   `cleanup`'s four tokens (four pins, plus `clean` after a real boot); `boot`'s arms (ready; the engine build
   failing; the xtask build failing; the binary absent; no spawn record; never ready with the receivers line, with
   neither line, with the foreign line; killed during the poll); the word arms (bare `boot`; `status engine` and
   bare `status`; any other word); `logs` (a real engine's date-suffixed member; the variable's base first; none
   resolving); five refusal labels (`home-unset`, `path-unset`, `data-dir-unusable`, `injector-build-failed`,
   `port-taken`).
2. *`evidence/mutations.md` holds seven entries; the scope guard reads no output after* — met.
3. *Under each mutation the arm's reading over the real log differs; no arm reads PASS over an input it cannot
   grade* — met for the first half on one real log; the second half was read over that one input only, which all
   seven arms can grade.
4. *Listening sockets are exactly the two receivers on `127.0.0.1`; a rejected variable reads `invalid_port` and
   the census the other alone; 4317 and 4318 are bound by no test and no gate entry* — met.
5. *The lock file sits where the rule gives, zero bytes, mode `0o600`, nothing under the data dir, 0 times in the
   log* — met, in both environments.
6. *`harness:ready` reads `ready` only over the run's own bound records; over another listener `not-ready` with
   `foreign`, for either program; `boot` then exits 1 and prints the line* — met. For the window program the
   `foreign` reading is pinned at the decision and not read live; its live readings are the healthy path (six
   local boots, eight on the runner).
7. *The boot-series entry reads `all-settled`* — met (three firings; `7` of 7 on the runner).
8. *The engine-cycle entry reads `pass`, 0 ERROR records, no witness file, `engine-log: PASS`, `cleanup: clean`* —
   met (three firings; the runner's step).
9. *`pre-push:linux` green with six stages, the cycle's lines in stage six, the probe after it green* — met (four
   readings).
10. *A taken port is a refusal before anything starts; the stage's decision never reads it green; a failing cycle
    on a real engine ends that engine* — met.
11. *No verdict, report, kept log line or test transcript added carries a passphrase, an environment value or a
    full product-consumed path* — met. A failed assertion of the new xtask suites prints the script's output with
    the scratch path and the passphrase replaced; the spawned arms print a lock file's basename.
12. *The standard gate set passes on the final tree* — met.
13. *The CI read prints `verdict: green`, the run named; the coverage job's two thresholds hold* — met:
    `ci#38079305603`; line 90.6 %, function 89.3 %.
14. *No workspace member, `ANDROMEDA_PULSE_*` name, TauRPC procedure, capability, product port or engine command
    word; no product source in the diff* — met.

**The four pending rows** (test-plan §1), clause by clause:
- `engine-boot-rejected-port-and-socket-census-coverage` — all three owed clauses closed (a spawned arm each).
- `harness-log-family-resolution-coverage` — closed (three `logs` pins, one on a real engine's family).
- `engine-log-arm-mutation-and-engine-cycle-glue-coverage` — one mutation per arm: closed. A pin per remaining
  refusal label: closed (four, and the new fifth). `engine_cycle::drive` now runs under committed tests (five
  refusals and one whole failing cycle on a real engine). **Open:** a runner reading of a red cycle (none
  occurred); `engine_log::run_check` and `run_settled` still have no pin of their own beyond the cycle.
- `harness-cleanup-verdict-and-boot-spawn-shell-coverage` — (a) per `cleanup` arm: closed. (b) per `boot`
  failure-reason arm: closed. (f) per word arm: closed. **Open, with their owners on the route:** (c) the `.ps1`
  boot-cycle leg (`Other operating systems retired from the code`, `working-route.md:64`); (d) the exit-witness
  branches and (e) the boot series' `timed-out` path (`Window's gates retired`, `working-route.md:50`).

**Gates** (the plan's entries by `run`; "×3" is implement's three firings of the whole block, each on the tree as
it stood, the third on the final tree; verdicts are the gate tool's words):
- `cargo fmt --check` — green ×3
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green ×3
- `git diff --name-only 514fbe57… -- crates pulse-app xtask scripts …` (the scope guard) — green ×3 (exit 0, no
  output)
- `cargo nextest run --workspace --profile ci --no-tests=fail -E 'binary(agent_run_shell) | binary(engine_cycle_refusals)'`
  — green ×3 (25 passed)
- `cargo nextest run --workspace --profile ci --no-tests=fail -E 'binary(integration_console_engine)'` — green ×3
  (7 passed)
- `d="target/boot-smoke/…-series" && … cargo xtask harness:boot-series --count 2` — green ×3 (`all-settled`,
  ordinals 2 and 3)
- `mkdir -p target/exit-witness && cc … && … cargo xtask harness:engine-cycle` — green ×3 (`pass`, 0 ERROR
  records, witness absent, `engine-log: PASS`, `cleanup: clean`)
- `cargo xtask check:english-sources` — green ×3 · `cargo xtask quarantine-tracking` — green ×3 ·
  `cargo xtask capability-widening-check` — green ×3 · `cargo xtask check:ingest-progress` — green ×3 (its line
  reads NEUTRAL, exit 0, over a family with no `buffer.tick`) · `cargo xtask check:staged-artifacts` — green ×3 ·
  `cargo xtask capability-drift` — green ×3 · `cargo xtask verify:capability-matrix` — green ×3
- `cargo nextest run --workspace --profile ci --no-tests=fail` — green ×3 (3111, 3111, 3112 passed)
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green ×3
- `git diff --quiet 514fbe57… -- pulse-app/ui/src/bindings/index.ts` — green ×3
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green ×3, and green again on the
  committed tree in the operator pass (110.44 s, head `d30b7037`)
- `test ! -e target/pre-push/run/data/run/andromeda-pulse.pid && ! (…24317) && ! (…24318)` — green ×3, and again
  in the operator pass
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (`leg = 'operator'`) — fired as
  written in the operator pass: `hygiene: clean` (read 49)
- `m="$(git ls-remote origin refs/heads/main | cut -f1)" && … git merge-base --is-ancestor "$m" HEAD`
  (`operator`) — green; remote `main` `178ebac5…` is contained
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` (`operator`) —
  green; `history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 514fbe57→d30b7037`, the intended move
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` (`operator`)
  — fired as written: `verdict: green · checks 7/7`, `ci#38079305603`
- `d="target/boot-smoke/ci-<id>-attempt-1" && gh run download <id> … -n logs-boot-Linux …` (`operator`,
  `--id 38079305603`) — green: `all-settled`, `"settled": 7`, the smoke `settled`, `"program":"window"`, no `end`
  line; eight witness files of one `loaded` line each
- `d="target/engine-cycle/ci-<id>-attempt-1" && gh run download <id> … -n logs-engine-Linux … cargo xtask check:engine-log`
  (`operator`, `--id 38079305603`) — green (exit 0); its nine lines end `engine-log: PASS`
- No entry carries `defer`; none was skipped. Smoke: the window series and the engine cycle above; no product
  boot-path file is touched.

**Mutations:** seven of seven arms of `check:engine-log`, each one line, each reddening pins (6, 3, 7, 9, 7, 8, 8
of 45) and changing the arm's line over the kept log of this chunk's first engine cycle
(`target/engine-cycle/20261010T184357Z`); 45 of 45 and the nine original lines after the reverts
(`evidence/mutations.md`).

**Watches:** none — the plan folds no `watch:`.

**Outcome basis:** the operator pass ran. The verdicts rest on its final state: the one commit `d30b7037` and
`ci#38079305603` on it, recorded in `evidence/operator-pass.md`, which P7 re-verifies. Implement's report (given in
this window, accepted by inputs#I4) is the basis for what only it holds: the red readings, the mutations, the three
firings, the hand runs and the census. Evidence files: `red-readings.md`, `mutations.md`, `gate-readings.md`,
`spawned-readings.md`, `operator-pass.md`.

**Limits:**
- One CI run, attempt 1, on the pre-CI commit. Every boot on the runner was the healthy path: no red cycle, no
  taken harness port and no foreign listener was read there.
- The port-using suites ran once on a runner in `lint / test` and once under coverage; the `mcp-server tests`
  job's summary line was not read, only its `success`.
- No live reading of a taken port on 24317 or 24318; the stage's `engine-port-taken` wiring
  (`read_engine_cycle`'s probe) is pinned at the decision only.
- One mutation per arm, on its passing side, over one real log (n = 1).
- The `.ps1` mirror was parsed, not run.
- This wrap's commit will be covered by the wrap's light gate alone.

**Process hygiene** (implement's census, re-measured at this wrap from the host's process list):

| Process | Started by | Final state |
|---|---|---|
| Window app, two per series, each under its own virtual display | the boot-series entry, three firings | terminated |
| Console engine and injector | the engine-cycle entry and the pre-push check, three firings; the operator pass's pre-push check | terminated |
| Engines, listeners and children of the test suites | this run's nextest runs | terminated |
| `sleep 600` of the teardown pin | this run | terminated |

At this wrap: 0 processes matching `andromeda-pulse-engine`, `pulse-app`, `inject_demo`, `Xvfb`, `xvfb-run`,
`tail -F` or `sleep 600` (the builder's own terminal left out); ports 14317, 14318, 24317 and 24318 all refuse.
Left on the host, git-ignored: three dated dirs each under `target/engine-cycle/` and `target/boot-smoke/`, the two
CI downloads, `target/pre-push/run/`, `target/test-ports/`, and the lock file in the system temp dir.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 514fbe57 (the parent of the oldest pre-CI commit d30b7037) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### pulse-app/tests/integration_console_engine.rs — added 441 line(s) in 8 range(s)
added: 4 · 6-7 · 35-82 · 109 · 125-126 · 147 · 167-178 · 379-752
- 35-43 @40 «struct PortClaim {»
  - 51-55 «let claims = Path::new(env!("CARGO_MANIFEST_DIR"))»
  - 58-81 «for _ in 0..10_000 {»
  - 167-177 «async fn wait_for_port(&mut self, port: u16) {»
- 379-387 @383 «struct Socket {»
- 389-396 @393 «struct Census {»
- 398-416 @399 «impl Census {»
  - 400-406 «fn listeners(&self) -> BTreeSet<(&'static str, String)> {»
  - 408-415 @409 «fn at_rest(&self) -> bool {»
- 418-434 @420 «fn socket_inodes(pid: u32) -> BTreeSet<u64> {»
  - 421-433 «std::fs::read_dir(format!("/proc/{pid}/fd"))»
- 436-447 @438 «fn local_end(column: &str) -> String {»
  - 441-446 «match u32::from_str_radix(address, 16) {»
- 449-493 @452 «fn socket_census(pid: u32) -> Census {»
  - 454-455 «let table =»
  - 458-473 «for name in ["tcp", "tcp6", "udp", "udp6"] {»
  - 474-487 «for line in table("unix").lines().skip(1) {»
  - 489-492 «Census {»
- 495-507 @498 «async fn census_at_rest(pid: u32) -> Census {»
  - 500-506 «loop {»
- 509-522 @512 «fn assert_the_census_finds_a_listener_of_this_process() {»
  - 516-521 «assert!(»
- 524-544 @525 «fn assert_no_socket_beyond_the_listeners(census: &Census) {»
  - 526-539 «for socket in &census.sockets {»
  - 540-543 «assert_eq!(»
- 546-556 @547 «fn file_names_under(dir: &Path, out: &mut Vec<String>) {»
  - 548-555 «for entry in std::fs::read_dir(dir).expect("read a directory").flatten() {»
- 558-568 @560 «fn family_text(dir: &Path) -> String {»
  - 563-567 «names»
- 570-629 @572 «async fn a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_alone_listening()»
  - 583-585 «engine»
  - 589-593 «assert_eq!(»
  - 595-598 «assert!(»
  - 609-612 «assert_eq!(»
  - 613-616 «eprintln!(»
  - 617-620 «assert!(»
  - 624-627 «assert_eq!(»
- 631-655 @633 «async fn the_engine_listens_on_its_two_loopback_receivers_and_on_nothing_else() {»
  - 642-649 «assert_eq!(»
- 657-751 @659 «async fn the_corpus_key_lock_file_sits_in_the_directory_the_environment_gives() {»
  - 664-750 «for with_runtime_dir in [false, true] {»
### scripts/agent-run.sh — added 5 line(s) in 1 range(s)
added: 196-200
### xtask/ci/quarantine-tracking-check.ps1 — added 2 line(s) in 1 range(s)
added: 14-15
### xtask/ci/quarantine-tracking-check.sh — added 2 line(s) in 2 range(s)
added: 4 · 24
### xtask/src/engine_cycle.rs — added 42 line(s) in 5 range(s)
added: 15-16 · 36 · 122-128 · 243-250 · 416-439
- 122-127 @125 «fn taken_port_refusal(grpc_accepting: bool, http_accepting: bool) -> Option<&'static str> {»
  - 243-250 @245 «if let Some(reason) = taken_port_refusal(»
  - 416-426 @417 «fn a_port_that_accepts_in_either_place_is_refused_as_taken() {»
  - 428-438 @429 «fn a_listener_on_a_cycle_port_is_read_as_taken_and_a_closed_one_is_not() {»
### xtask/src/harness_ready.rs — added 261 line(s) in 23 range(s)
added: 5-9 · 32-34 · 40-41 · 64-69 · 79-86 · 96 · 172-228 · 230 · 235 · 251 · 490-493 · 498-627 · 637-640 · 646-649
       651-654 · 665-668 · 674-681 · 683-686 · 692-695 · 697-700 · 714-717 · 780 · 783
- 172-180 @176 «enum Receiver {»
- 182-198 «impl Receiver {»
  - 183-189 «fn read(accepted: bool, bound_by_this_run: bool) -> Self {»
  - 191-197 «fn label(self) -> &'static str {»
- 200-227 @204 «fn bound_by_this_run(lines: &[String], target: &str, port: u16) -> bool {»
  - 205-208 «let records: Vec<Value> = lines»
  - 209-210 «let is =»
  - 211-213 «let Some(boot) = records»
  - 214-216 «else {»
  - 218-226 «records[boot + 1..].iter().any(|record| {»
  - 515-518 «fn boot_line(program: &str) -> String {»
  - 520-527 «fn bind_line(target: &str, level: &str, port: u16) -> String {»
  - 529-536 @530 «fn own_log() -> Vec<String> {»
  - 538-544 @539 «fn read_both(lines: &[String]) -> (Receiver, Receiver) {»
  - 546-551 @547 «fn both_ports_accepting_over_the_run_s_own_bound_records_is_ready() {»
  - 553-564 @554 «fn a_held_port_with_no_bound_record_is_foreign_and_not_ready() {»
  - 566-576 @567 «fn a_failed_bind_record_of_the_same_target_does_not_count() {»
  - 578-592 @579 «fn a_bound_record_for_another_port_or_the_other_receiver_does_not_count() {»
  - 594-602 @595 «fn a_bound_record_before_the_last_boot_record_does_not_count() {»
  - 604-611 @605 «fn a_log_with_no_boot_record_holds_no_bound_record_of_this_run() {»
  - 613-619 @614 «fn a_port_that_refuses_is_refusing_whatever_the_log_holds() {»
### xtask/src/main.rs — added 22 line(s) in 5 range(s)
added: 67 · 83 · 291 · 939-945 · 1021-1032
  - 939-945 «const QUARANTINE_SEARCH_DIRS: [&str; 5] = [»
  - 1021-1031 @1022 «fn quarantine_check_reads_the_xtask_integration_tests() {»
### xtask/src/pre_push.rs — added 271 line(s) in 10 range(s)
added: 14-20 · 32-35 · 76 · 182 · 227-228 · 276 · 278 · 280 · 282-384 · 984-1133
  - 283-285 «fn failed(self) -> Stop {»
- 288-303 @290 «struct CycleReading {»
- 305-320 @308 «fn decide_ci_gates(reading: CycleReading) -> Result<(), Stop> {»
  - 309-311 «if reading.port_taken {»
  - 312-314 «if reading.port_accepting_after || reading.pid_alive_after {»
  - 315-319 «match (reading.cycle_exit, reading.gates_passed) {»
- 322-369 @324 «fn read_engine_cycle(host: &Host) -> CycleReading {»
  - 327-329 «if remove_tree(data).is_err() || fs::create_dir_all(data).is_err() {»
  - 330-334 «let accepting = || {»
  - 336-338 «if reading.port_taken {»
  - 339-349 «let exit = command(»
  - 356-357 «reading.pid_alive_after = read_pid(&data.join("run").join("andromeda-pulse.pid"))»
  - 358-367 «if reading.port_accepting_after || reading.pid_alive_after {»
  - 375-383 «cleanup»
  - 984-990 «fn passed_cycle() -> CycleReading {»
  - 992-994 «fn stage_six_red() -> Result<(), Stop> {»
  - 996-999 @997 «fn a_passed_cycle_with_passing_gates_and_nothing_left_running_is_green() {»
  - 1001-1017 @1002 «fn a_taken_cycle_port_cannot_be_evaluated_and_never_reads_green() {»
  - 1019-1032 @1020 «fn a_cycle_that_could_not_be_evaluated_stops_as_cannot_evaluate() {»
  - 1034-1048 @1035 «fn a_failed_cycle_is_red() {»
  - 1050-1063 @1051 «fn a_passed_cycle_whose_gates_failed_or_did_not_run_is_red() {»
  - 1065-1084 @1066 «fn a_cycle_that_left_a_port_accepting_or_its_engine_alive_is_red_whatever_it_returned() {»
  - 1086-1089 @1087 «fn the_stage_s_cycle_ports_are_the_harness_only_pair_and_never_the_shared_ones() {»
  - 1091-1132 @1093 «fn end_cycle_ends_the_process_the_stage_data_dir_s_pid_file_names() {»
### xtask/src/source_lint.rs — added 21 line(s) in 3 range(s)
added: 24 · 30 · 233-251
  - 233-250 @234 «fn english_sources_reads_the_xtask_integration_tests() {»
### xtask/tests/agent_run_shell.rs — new file · 590 line(s)
- 28-29 «const FOREIGN_LINE: &str =»
- 31-38 @33 «fn engine_scratch() -> Scratch {»
- 40-42 «fn boot_engine(scratch: &Scratch) -> Ran {»
- 44-56 @45 «fn booted(scratch: &Scratch) -> u32 {»
  - 48-52 «assert!(»
- 58-60 «fn cleanup(scratch: &Scratch) -> Ran {»
- 62-71 «fn assert_cleanup(scratch: &Scratch, token: &str, code: i32) {»
  - 64-69 «assert_eq!(»
- 73-77 «fn write_pidfile(scratch: &Scratch, pid: u32) {»
- 79-105 @81 «fn assert_never_ready(scratch: &Scratch, boot: &Ran, cleanup_token: &str) {»
  - 83-88 «assert!(»
  - 89-94 «assert_eq!(»
  - 95-99 «assert!(»
  - 101-104 «assert!(»
- 107-112 @108 «fn cleanup_with_no_pidfile_and_both_ports_refusing_is_clean_twice_over() {»
- 114-123 @115 «fn cleanup_with_no_pidfile_and_a_port_accepting_is_no_pid_ports_accepting() {»
  - 119-122 «assert!(»
- 125-143 @126 «fn cleanup_with_a_dead_pid_and_a_port_accepting_is_ports_lingering() {»
  - 128-131 «let mut child = Command::new(common::real_bash())»
  - 139-142 «assert!(»
- 145-161 @146 «fn cleanup_with_an_unreaped_child_in_the_pidfile_is_app_survived() {»
  - 148-151 «let mut child = Command::new(common::real_bash())»
  - 153-156 «assert!(»
- 163-194 @164 «fn boot_engine_reads_ready_and_cleanup_ends_the_engine_clean() {»
  - 167-170 «assert!(»
  - 171-175 «assert_eq!(»
  - 179-182 «assert!(»
  - 183-186 «assert!(»
  - 188-193 «assert!(»
- 196-211 @197 «fn boot_fails_when_the_engine_build_fails() {»
  - 203-208 «assert!(»
- 213-226 @214 «fn boot_fails_when_the_xtask_build_fails() {»
  - 220-224 «assert!(»
- 228-242 @229 «fn boot_fails_when_the_built_binary_is_absent() {»
  - 234-240 «assert!(»
- 244-259 @245 «fn boot_with_no_word_resolves_the_window_program() {»
  - 249-254 «assert!(»
  - 255-258 «assert_eq!(»
- 261-295 @262 «fn boot_with_no_spawn_record_exits_one_and_leaves_the_engine_it_started() {»
  - 265-269 «let boot = scratch.run(»
  - 271-276 «assert!(»
  - 281-284 «assert!(»
  - 286-289 «assert!(»
  - 291-294 «assert!(»
- 297-304 @299 «fn refusing_privileged_port() -> u16 {»
  - 300-303 «[1, 7, 9, 13, 19]»
- 306-335 @307 «fn boot_never_ready_over_a_refusing_receiver_prints_the_receivers_line() {»
  - 310-315 «let boot = scratch.run(»
  - 317-321 «assert!(»
  - 322-329 «assert!(»
  - 330-334 «assert!(»
- 337-357 @338 «fn boot_never_ready_over_an_unparseable_port_prints_neither_receiver_line() {»
  - 340-345 «let boot = scratch.run(»
  - 347-351 «assert!(»
  - 352-356 «assert!(»
- 359-393 @360 «fn boot_never_ready_over_another_listener_prints_the_foreign_line() {»
  - 363-367 «let boot = scratch.run(»
  - 371-375 «assert!(»
  - 376-383 «assert!(»
  - 384-388 «assert!(»
  - 389-392 «assert!(»
- 395-426 @396 «fn boot_names_how_the_engine_ended_when_it_is_killed_during_the_poll() {»
  - 398-405 «let child = scratch»
  - 406-409 «assert!(»
  - 416-420 «assert!(»
  - 421-425 «assert!(»
- 428-451 @429 «fn status_reads_the_program_asked_for_on_a_running_engine() {»
  - 435-440 «assert!(»
  - 444-448 «assert!(»
- 453-476 @454 «fn any_other_word_after_boot_or_status_is_usage_exit_two_and_no_data_dir() {»
  - 455-475 «for words in [»
- 478-483 @480 «struct Tail {»
- 485-524 «impl Tail {»
  - 486-503 «fn spawn(command: &mut Command) -> Self {»
  - 505-509 «fn first_line(&self) -> String {»
  - 511-523 @512 «fn end(mut self) {»
- 526-552 @527 «fn logs_prints_a_record_of_the_date_suffixed_member_a_real_engine_wrote() {»
  - 531-534 «let dated = |name: &String| {»
  - 535-538 «assert!(»
  - 542-543 «let record: serde_json::Value =»
  - 545-548 «assert_eq!(»
- 554-577 @555 «fn logs_reads_the_base_the_log_file_variable_names_before_the_data_dir() {»
  - 559-561 «for dir in [&other, &own] {»
  - 563-566 «std::fs::write(»
  - 570-574 «let tail = Tail::spawn(»
- 579-590 @580 «fn logs_with_no_base_resolving_says_so_and_exits_one() {»
  - 584-588 «assert!(»
### xtask/tests/common/mod.rs — new file · 420 line(s)
- 22-27 «pub fn repo_root() -> PathBuf {»
  - 23-26 «Path::new(env!("CARGO_MANIFEST_DIR"))»
- 29-33 «pub fn script() -> PathBuf {»
- 35-49 @36 «pub fn engine_bin() -> PathBuf {»
  - 38-42 «let engine = exe»
  - 43-47 «assert!(»
- 51-57 «pub fn real_bash() -> PathBuf {»
  - 53-56 «std::env::split_paths(&path)»
- 59-64 «fn require_proc() {»
  - 60-63 «assert!(»
- 66-74 @71 «pub struct PortClaim {»
- 80-85 «fn random_word() -> u64 {»
  - 82-84 «std::collections::hash_map::RandomState::new()»
- 87-113 «pub fn claim_port() -> PortClaim {»
  - 91-111 «for _ in 0..10_000 {»
- 115-117 «pub fn listen(port: u16) -> TcpListener {»
- 119-122 «pub fn accepts(port: u16) -> bool {»
- 124-136 @125 «pub fn wait_until(bound: Duration, mut condition: impl FnMut() -> bool) -> bool {»
  - 127-135 «loop {»
- 138-140 «pub fn read_pid(path: &Path) -> Option<u32> {»
- 142-147 @144 «pub fn pid_state(pid: u32) -> Option<char> {»
- 149-152 @150 «pub fn pid_ended(pid: u32) -> bool {»
- 154-157 «pub fn is_engine(pid: u32) -> bool {»
  - 155-156 «std::fs::read(format!("/proc/{pid}/cmdline"))»
- 159-177 @160 «pub fn group_members(pgid: u32) -> usize {»
  - 161-163 «let Ok(entries) = std::fs::read_dir("/proc") else {»
  - 164-176 «entries»
- 179-187 @181 «pub fn signal(signal: &str, target: &str) -> bool {»
  - 182-186 «Command::new(real_bash())»
- 189-203 @191 «fn write_executable(path: &Path, text: &str) {»
  - 192-201 «let written = Command::new(real_bash())»
- 205-207 «fn run_passphrase() -> String {»
- 209-220 @210 «pub enum Cargo {»
- 222-226 «pub struct Ran {»
- 228-236 «impl From<Output> for Ran {»
  - 229-235 «fn from(output: Output) -> Self {»
- 238-244 «pub struct Scratch {»
- 246-408 «impl Scratch {»
  - 247-261 «pub fn new() -> Self {»
  - 263-265 «pub fn path(&self, sub: &str) -> PathBuf {»
  - 267-269 «pub fn data(&self) -> PathBuf {»
  - 271-273 «pub fn run_file(&self, name: &str) -> PathBuf {»
  - 275-278 @276 «pub fn spawned_pid(&self) -> Option<u32> {»
  - 280-292 @281 «pub fn log_family(&self) -> Vec<String> {»
  - 294-301 @296 «pub fn link_engine(&self) {»
  - 303-325 «pub fn stand_in_cargo(&self, cargo: Cargo) {»
  - 327-332 @328 «pub fn cargo_calls(&self) -> Vec<String> {»
  - 334-350 @338 «pub fn stand_in_bash(&self) {»
  - 352-355 @353 «pub fn search_path(&self) -> String {»
  - 357-370 @359 «pub fn cleared(&self, program: impl AsRef<std::ffi::OsStr>) -> Command {»
  - 372-388 @375 «pub fn script(&self, args: &[&str]) -> Command {»
  - 390-392 «pub fn run(&self, command: &mut Command) -> Ran {»
  - 394-399 @396 «pub fn shown(&self, text: &str) -> String {»
  - 401-407 @402 «pub fn printed(&self, ran: &Ran) -> String {»
- 410-420 «impl Drop for Scratch {»
  - 411-419 «fn drop(&mut self) {»
### xtask/tests/engine_cycle_refusals.rs — new file · 191 line(s)
- 26-42 @28 «fn cycle(scratch: &Scratch, data_dir: &std::path::Path) -> Command {»
  - 30-37 «command»
  - 38-40 «if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {»
- 44-47 «fn verdict(scratch: &Scratch, ran: &Ran) -> Value {»
  - 45-46 «serde_json::from_str(&ran.stdout)»
- 49-70 @51 «fn assert_refused(scratch: &Scratch, ran: &Ran, label: &str) {»
  - 53-58 «assert!(»
  - 61-63 «for verb in VERBS {»
  - 66-69 «assert!(»
- 72-77 «fn assert_both_ports_refuse(scratch: &Scratch) {»
  - 73-76 «assert!(»
- 79-87 @80 «fn a_cycle_with_no_home_is_refused_as_home_unset() {»
- 89-97 @90 «fn a_cycle_with_no_path_is_refused_as_path_unset() {»
- 99-113 @100 «fn a_cycle_whose_data_dir_sits_under_a_regular_file_is_refused_as_data_dir_unusable() {»
  - 105-109 «let ran = scratch.run(»
- 115-131 @116 «fn a_cycle_whose_injector_build_fails_is_refused_as_injector_build_failed() {»
  - 119-123 «let ran = scratch.run(»
  - 125-129 «assert_eq!(»
- 133-155 @134 «fn a_cycle_over_a_port_another_listener_holds_is_refused_as_port_taken() {»
  - 135-154 «for held_is_grpc in [true, false] {»
- 157-191 @158 «fn a_cycle_that_fails_on_a_real_engine_ends_the_engine_it_started() {»
  - 163-167 «let ran = scratch.run(»
  - 175-179 «assert!(»
  - 181-184 «assert!(»
  - 186-189 «assert!(»
