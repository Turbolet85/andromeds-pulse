# Report — 2026-10-09-boot-smoke-s-early-exit-found-and-closed

**Chunk:** Boot smoke's early exit found and closed — the cause of the app ending itself after ready named from run evidence; boot ready waits for the receivers (P-129)
**Date:** 2026-10-09T19:53:51Z
**Commits:** `e2931127 chore(2026-10-09-boot-smoke-s-early-exit-found-and-closed): operator pre-CI commit` (the one commit since the chunk base `277d65db`; basis `git log --format='%h %s' 277d65db..HEAD`)

**What this chunk is, in one paragraph.** No run has named the cause of the early exit, so this chunk closes none
and claims no capability (the plan's "second kind", inputs#I3). It built three things: `boot` reports ready only
after both OTLP receivers accept a connection; the CI smoke reads the app after its windows have settled, or at its
end; and a run in which the app ends by itself says what the job saw of the exit record, the `app.exit` record, the
display and the session bus, and keeps the display server's own output. No product code changed.

## Changes (structured — detectors read this)
- **Files:** seven, all in research's two lists (basis: `gate.py scope` at this wrap, `changed 7 · listed 7`,
  base `277d65db`).
  - New: `xtask/src/harness_ready.rs` (660 lines).
  - Modified: `xtask/src/main.rs` · `xtask/src/harness_status.rs` · `scripts/agent-run.sh` ·
    `scripts/agent-run.ps1` · `.github/workflows/ci.yml` · `pulse-app/tests/quality_gate_workflow.rs`.
  - Nothing under `pulse-app/src`, `pulse-app/ui`, `crates`, `pulse-app/capabilities`, no manifest, no lockfile
    (basis: the plan's scope-guard entry, `git diff --name-only 277d65d… -- crates pulse-app xtask scripts …` with
    the seven files excluded: exit 0, no output, at implement).
- **Symbols / APIs:**
  - **Two new xtask verbs**, declared at `xtask/src/main.rs:53-65` and dispatched at `:273-274`:
    - `cargo xtask harness:ready` → `harness_ready::run_ready` (`xtask/src/harness_ready.rs:47-85`). One pretty-JSON
      object on stdout: `verdict` (`ready` · `not-ready` · `ended` · `cannot-evaluate`), `pid`, `ended` (the exit
      record, only when the verdict is `ended`), `otlp_grpc` and `otlp_http` (each `accepting` · `refusing`, `null`
      on `cannot-evaluate`). Exit 0 on `ready`, 1 on `not-ready` and `ended`, 2 on `cannot-evaluate`.
      `ready` requires the status verdict `running-healthy` AND a TCP connection accepted on `127.0.0.1` at each
      port resolved from `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `ANDROMEDA_PULSE_OTLP_HTTP_PORT` (defaults 4317 / 4318),
      each attempt bounded at one second. The pure decision is `decide_ready` (`:132-140`).
    - `cargo xtask harness:settled [--timeout-seconds N]` (default 30) → `harness_ready::run_settled` (`:87-122`).
      It polls every 200 ms until one arm holds, prints one pretty-JSON object and writes the same object to
      `logs/harness-settled.json` under the resolved data dir (`keep_verdict`, `:366-378`; written only when that
      `logs/` dir already exists). Arms: the pid is dead → `ended`, exit 1; the log family holds an
      `app.boot.window.navigation` record for each of the four window labels (`compact-widget`, `main`, `findings`,
      `report`) and the pid is alive → `settled`, exit 0; the timeout passes → `not-settled`, exit 1; no pid, an
      unprobeable pid, no resolvable data dir, or a `--timeout-seconds` below 8 → `cannot-evaluate`, exit 2. The
      pure decision is `decide_settled` (`:166-182`). Fields on every arm: `verdict`, `pid`, `ended`,
      `app_exit_record` (`present` · `absent`), `windows_settled` (0 to 4), `display` and `session_bus` (each
      `reachable` · `gone` · `unset` · `unknown`).
  - **Three system variables read by the harness tool, by presence, labels only:** `DISPLAY`,
    `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR` (`parse_display` `:271-292`, `parse_session_bus` `:294-315`).
    A local display (`:N`, `:N.S`) is probed as the X socket of that number; a `unix:path=` bus address, else the
    `bus` socket under the runtime dir, is probed by a Unix-socket connect. No value of any of the three is
    printed or written. **They are read by xtask only; the product binary's reads are unchanged.**
  - **`harness_status.rs` helpers made crate-visible, behaviour unchanged:** `resolve_paths`, `pid_alive`,
    `end_file`, `read_ended`, `read_pid`, `newest_family_member` became `pub(crate)`; `resolve_data_dir`
    (`xtask/src/harness_status.rs:139-141`) and `env_path` (`:132-137`) were extracted from `resolve_paths`.
    `harness:status` keeps its verdict object, its four arms and its exit codes; its 13 in-file pins pass
    unchanged. Remaining callers: `classify` and the probes now have two production callers each
    (`harness_status::run` and `harness_ready`).
  - No TauRPC procedure, no port, no product-consumed environment variable, no capability.
- **Crates / modules:** one module added, `xtask::harness_ready`. No crate added, removed or renamed.
- **Dependencies:** none added, none bumped. `Cargo.toml` and `Cargo.lock` untouched.
- **Schema / config:** none in the product. Two harness-written files are new under the data dir's `logs/`:
  `logs/harness-settled.json` (the settle verdict, written by `harness:settled`) and `logs/xvfb.log` (the display
  server's own error output, written by `xvfb-run -e` in the CI smoke step and in the plan's two live legs). The
  product never reads either. `run/` is still not uploaded.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - **How long the CI app lives before the smoke stops it.** Stated as 0.57 s to 1.40 s (`grep -E '1\.40 s|0\.57 s|1\.4 s'`:
    architecture 1 · security-plan 1 · test-plan 1 · obs-plan 1 · the test-plan key file
    `5-command-implementation.md` 1). Measured on `ci#37979648967`: 5.494 s from the app's first record to its
    `app.exit` (the smoke's own `cleanup`).
  - **How many CI boot-job logs hold `ui.webgpu.adapter`.** Stated as "four of seven" / some runs only
    (`grep -E 'some runs only|four of seven|three of seven'`: architecture 1 · security-plan 1 · test-plan 2 ·
    obs-plan 1). Under the new step the smoke reads past the adapter request on every run that reaches the settle
    verdict; measured on one run: 2 records, both `no_navigator_gpu`.
  - **Boot-smoke readings of 2026-10-09:** nine, two red (was eight, two red, at the plan). The ninth,
    `ci#37979648967`, is green.
  - **Workspace test count:** 2826 passed, 0 skipped at implement (`cargo nextest run --workspace --profile ci`);
    24 are new (20 in `xtask/src/harness_ready.rs`, 4 in `pulse-app/tests/quality_gate_workflow.rs`). No master
    states the total.
  - **xtask verbs:** two more (`harness:ready`, `harness:settled`).
- **Dev-tool versions:** none.
- **Harness / gate surface:**
  - **`scripts/agent-run.sh` `boot`** polls `cargo xtask harness:ready` in place of `harness:status` (same threaded
    `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE`, same `HARNESS_STATUS_TIMEOUT`, default 10 s). The ready lines are
    unchanged. On a failed poll with the app alive it prints the existing `app still running (pid N) but never
    reported healthy` line and, **only when the last readiness verdict held a `refusing` label**, one more:
    `the receivers never both accepted: OTLP gRPC 127.0.0.1:{port}, OTLP HTTP 127.0.0.1:{port}`. With the app gone
    the existing `app ended:` line stands. `status`, `cleanup`, `run`, `logs` are unchanged. Still five verbs.
    Git mode 100755 in the index.
  - **`scripts/agent-run.ps1` `boot`** mirrors it through a new `Invoke-Ready` function (`scripts/agent-run.ps1:46-55`)
    and the same conditional line. **Unwitnessed:** parsed by `pwsh` with 0 parse errors, never run; no CI job and
    no dev host runs the script.
  - **The CI boot job's smoke step** (`.github/workflows/ci.yml`, the step `Boot pulse-app smoke`, added lines
    `377-381` and `383-389`), and nothing else in the workflow: it creates the data dir's `logs/`; `xvfb-run` gains
    `-e "$ANDROMEDA_PULSE_DATA_DIR/logs/xvfb.log"`; inside the one `xvfb-run`, in order: `boot || exit 1`,
    `cargo xtask harness:settled; a=$?`, `status; b=$?`, `cleanup; c=$?`, `test "$a$b$c" = 000`. `set -e` is gone
    from the inner script, so `cleanup` runs whatever the two before it returned. No `uses:` line, permission,
    soft gate, retry, job edge or new step (basis: the plan's two workflow probes, green at implement).
  - **The boot job's uploaded artifact** `logs-boot-Linux` now holds five files (read on `ci#37979648967`):
    `agent-latest.jsonl.{date}`, `boot.log`, `build.log`, `harness-settled.json`, `xvfb.log`.
  - **Four new workflow pins** in `pulse-app/tests/quality_gate_workflow.rs` (`468-500`, `502-526`, `528-549`,
    `551-562`), with two helpers (`434-449`, `451-466`): the four invocations in order inside one `xvfb-run`;
    cleanup running whatever settled and status returned; the error file under `logs/` and the same dir uploaded;
    no soft-fail key on the step.
  - **A wording constraint found:** the pin `ci_workflow_runs_on_linux_only` refuses the token `windows` on any
    line of `ci.yml`, comments included. A step comment using the word in its other sense reddened it once at
    implement and was reworded.
- **Cross-project / external claims:**
  - **`ci#37979648967`** (GitHub Actions, repository `Turbolet85/andromeds-pulse`, event `pull_request`, run
    attempt 1) on the pushed tip `e29311270035`: `verdict: green · checks 7/7 · wall 1668 s`; all six jobs
    `success`; boot smoke `success` in 446 s; a11y `success` in 298 s. Read once, not re-run (basis:
    `evidence/operator-pass.md`, entries 21 to 24).
  - **What that run checked out, measured at this wrap** (inputs#I4 item 3): the boot job's own checkout lines
    read `fetch … +88d5ed3094ab…:refs/remotes/pull/40/merge`, `git checkout … refs/remotes/pull/40/merge` and
    `HEAD is now at 88d5ed3 Merge e2931127… into 178ebac5…`; the GitHub API reads the run's `head_sha` as
    `e2931127…` and commit `88d5ed30`'s parents as `178ebac5` (`main`'s tip) and `e2931127`. So a pull-request run
    of this branch builds the merge of the branch with `main`, and the application log's `git.commit.sha` names
    that merge commit, not the pushed tip. `main` is not an ancestor of the branch. **On this run the merged tree
    is equal to the pushed tip's:** `git diff --name-only e2931127 88d5ed30` prints 0 files. That equality holds
    only while `main` holds nothing the branch lacks.
  - `inputs.py verify` at this wrap: `6 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 4 · uncited 3 · unparsed 0`. No drift.
    - I1 · `../additional/pc-overseer/relays/pulse-phase-boot-smoke-early-exit-2026-10-09.md` · copy · unchanged.
    - I2 · message, the operator in the /andromeda-phase invocation · copy · n/a.
    - I3 · message, the operator's answers to the two P4 questions · copy · n/a.
    - I4 · `../additional/pc-overseer/relays/pulse-wrap-boot-smoke-early-exit-2026-10-09.md` · copy · unchanged.
      Snapped at this wrap; cited here as inputs#I4.
    - I5 · message, the operator's operator-pass directive after the implement report · copy · n/a. Snapped at
      this wrap; cited here as inputs#I5.
    - I6 · message, the operator in the /andromeda-wrap-session invocation · copy · n/a. Snapped at this wrap;
      cited here as inputs#I6.
    - I7 · message, the operator's word on the route-resolve card · copy · n/a. Snapped at this wrap's P5, after
      the report was first written; cited here as inputs#I7. It settles the owner of P-129's closing (a `WATCH:` to
      eleven green readings on the next markerless entry, its retirement by count putting P-129 to the operator),
      three `CARRY:` lines, the master record's description, and keeps the harness-only classification as not a
      widening, named as the pc overseer's own.
- **Reverted / negative API facts:** none shipped and reverted. One planted and removed: `continue-on-error: true`
  on the smoke step, a mutation check of the soft-fail pin (`evidence/mutation-soft-fail.md`); the workflow's diff
  against the base holds one hunk, inside the smoke step.
- **Insufficient fixes (written, kept, not the remedy):** **the whole chunk, against P-129's closing clause.** The
  defect is the app ending by itself shortly after ready on a runner (two reds in nine readings). What the change
  resolved: `boot` no longer reports ready before the receivers bind; every run reads the app past the point
  where both reds ended; a run that ends says what the job saw. What it did not resolve: the cause is not named
  and not closed. Owner of the remainder: decided at this wrap's route-resolve card (inputs#I4 item 2, inputs#I6).
- **Spec claims disproved by measurement:**
  1. **"The shipped sh `boot` reports ready on the `running-healthy` verdict alone and confirms no handshake", an
     OPEN gap owned by this entry** — the test-plan key file `5-command-implementation.md` (Readiness signal) and
     its restating sites (`grep 'TCP handshake'`: architecture 2 · test-plan 1 · that key file 2). No longer true:
     ready is `harness:ready`'s verdict, which holds a handshake on both ports. Evidence: the timed leg (`boot`
     returned 0.296 s after the app's first record, 0.097 s after both bind records) and `ci#37979648967` (ready
     printed 0.596 s after the first record, 0.453 s after both bind records).
  2. **"The app lives 0.57 s to 1.40 s before `cleanup`" / "the smoke stops the app within 1.40 s on every run"**
     — the five sites named under Counts. On the new step the app lived 5.494 s on the runner and 5.514 s on the
     local green leg.
  3. **"The CI boot job's log holds `ui.webgpu.adapter` on some runs only … by how long the app lives"** and the
     paired statement that the `ci-gates` frame line reads `no adapter record in this log` where the smoke stopped
     the app before the adapter request (`grep 'no adapter record in this log'`: architecture 1 · test-plan 2 ·
     obs-plan 2; `grep 'before the webview'`: security-plan 1 · obs-plan 1). On `ci#37979648967` the log holds 2
     adapter records and the frame line read `frame: cannot-evaluate: 0 samples, no WebGPU adapter
     (no_navigator_gpu)`, the line the plan predicted. One run measured; the step's order makes it the expected
     line on every run that reaches the settle verdict.
  4. **The `boot` Command body's statement that the app's streams go to `/dev/null`** (`grep '/dev/null'`:
     architecture 1 · the key file 1). True of the waiting wrapper's own streams (`scripts/agent-run.sh:89` at the
     base); the app's stdout and stderr go to `logs/boot.log` (`:79` at the base). Read at phase P3 (research.md);
     the script lines did not change in meaning at this chunk.
  5. **Chunk-artifact claim, no edit owed:** research.md says no CI log has ever held `app.boot.window.navigation`.
     `ci#37979648967`'s log holds four. True when written.
- **Expected amendments (from plan):**
  1. *test-plan §3 5-command implementation — the Readiness signal clause, the `boot` Command body's `/dev/null`
     sentence, the Timeout clause's measured boot-to-ready* — **carried**: Harness / gate surface (the boot verb)
     and Spec claims disproved 1 and 4; the measured boot-to-ready is in claim 1. Sites: `grep 'Readiness signal'`
     → the key file `registries/contracts/test-plan/5-command-implementation.md` 1 and the registry index
     `test-plan-contracts.toml` 1; `grep 'harness:status'` → test-plan 2 · its key files `status-endpoint-shape.md`
     1, `pid-file.md` 2, `per-chunk-gate-discipline.md` 1, `5-command-implementation.md` 2.
  2. *test-plan §9 CI Integration — the Boot smoke row's step sequence; test-plan §1 — the one-line `boot` summary
     and the pending trigger `harness-cleanup-verdict-and-boot-spawn-shell-coverage`* — **carried**: Harness / gate
     surface (the smoke step; the failure line and the ps1 edit with no committed shell-level test) and Coverage.
     Sites: `grep -E 'Boot smoke|boot smoke'` → test-plan 2 · the key file 2; `grep
     'harness-cleanup-verdict-and-boot-spawn-shell-coverage'` → test-plan 1 · the key file 2.
  3. *architecture §Occupied Resources — xtask CLI surfaces, Filesystem locations, Environment variables* —
     **carried**: Symbols / APIs (the two verbs, the three variables) and Schema / config (the two files). Sites:
     `grep 'harness:status'` → architecture 4; `grep -E 'andromeda-pulse\.(exit|spawn)'` → architecture 2; `grep
     'XDG_RUNTIME_DIR'` → architecture 1; `grep 'agent-run\.ps1'` → architecture 5; `grep -E 'Boot smoke|boot smoke'`
     → the architecture key file `ci-cd-approach.md` 1.
  4. *obs-plan §9 Telemetry artifact handling — what the boot job's artifact holds; obs-plan §10 — how long the CI
     app lives and what the frame line reads* — **carried**: Harness / gate surface (the artifact's five files),
     Counts, Spec claims disproved 2 and 3. Sites: `grep 'logs-boot'` → obs-plan 1 (test-plan 1); `grep
     'boot\.log'` → obs-plan 1; `grep -E 'Boot smoke|boot smoke'` → obs-plan 4; `grep 'no adapter record in this
     log'` → obs-plan 2.
  5. *security-plan §Security Anti-Patterns → Logging — the `ui.webgpu.adapter` paragraph's "some runs only"
     sentence; → Input — the two new harness-written files in the state-file class* — **carried**: Spec claims
     disproved 3 and Schema / config. Sites: `grep -E 'some runs only|four of seven'` → security-plan 1; `grep
     'before the webview'` → security-plan 1; `grep -E 'andromeda-pulse\.(exit|spawn)'` → security-plan 1; `grep
     'boot\.log'` → security-plan 1; `grep 'XDG_RUNTIME_DIR'` → security-plan 6.
  6. *The plan's note for the route card (not an amendment): the requirement stays open, the count runs to eleven,
     the two red artifacts expire 2026-10-23* — **not an amendment**; its fact is in Insufficient fixes and
     Outcome; disposed at route-resolve.
  7. *`matrix#P-129 notes` (acceptance 12: "P-129 stays pooled with a dated ledger note")* — **ledger-note — owner
     P7.3.** Phase P5 already wrote one dated note; this wrap's adds the reading of `ci#37979648967` and the
     closing's owner.
- **Coverage of new surfaces:**
  - `cargo xtask harness:ready` → validation mechanism✓ (a port that is not 1 to 65535 reads `cannot-evaluate`,
    pinned) · instrumentation n/a (a harness tool that prints a verdict; it logs nothing) · PII redacted✓ (a pid,
    the bounded exit record, two closed labels) · tests unit (8 pins) + live (ready and not-ready read against a
    running app) · a11y n/a · tokens n/a
  - `cargo xtask harness:settled` → validation mechanism✓ (the timeout floor, both address parsers, pinned per
    form) · instrumentation n/a · PII redacted✓ (closed labels, a pid, a count, the bounded exit record; no value
    of the three variables, no path; the field set pinned) · tests unit (12 pins) + live (`settled`, `ended`,
    `cannot-evaluate` read on this host; `settled` read on a runner; **`not-settled` pinned, never read live**) ·
    a11y n/a · tokens n/a
  - `logs/harness-settled.json` and `logs/xvfb.log` in the uploaded artifact → validation n/a · instrumentation
    n/a · PII redacted✓ for the verdict file; the display server's file read whole five times (four local, one
    runner): four empty, one holding keymap-compiler warnings, none holding telemetry · tests the workflow pin on
    the error-file path · a11y n/a · tokens n/a
  - `boot`'s new failure line (sh) → validation n/a · instrumentation n/a · PII n/a (two port numbers) · tests ✗
    (no committed shell-level test; driven once by hand, the never-ready leg; the arm where it stays silent was
    not driven) · a11y n/a · tokens n/a
  - `Invoke-Ready` and the failure line (ps1) → tests unrunnable-here (parsed, never run) · the rest n/a
  - The CI smoke step → validation n/a · instrumentation n/a · PII n/a · tests 4 workflow pins + one runner run ·
    a11y n/a · tokens n/a
  - The non-Unix branch of `unix_socket_accepts` (`xtask/src/harness_ready.rs:317-329`) → tests unrunnable-here
    (not compiled on this host; it returns "no such socket", which reads `unknown`)

## Deviations from intent
- **The failure line is conditional.** Plan step 5 asks `boot` to name that the receivers never accepted whenever
  the poll fails with the app alive. The app can be alive and unready on a stale log with both receivers
  accepting, where that sentence would be false. The line is printed only when the last readiness verdict held a
  `refusing` label, and reads "the receivers never both accepted". Acceptance 2 is met in the arm it describes.
- **Cases the plan left open, settled in the code:** `harness:ready` reads `cannot-evaluate` on an unparseable
  port and `ended` only when a pid exists (a missing pidfile is `not-ready`); `harness:settled` reads
  `cannot-evaluate` on an absent or unprobeable pid; a `%`-escaped bus path reads `unknown` rather than being
  probed undecoded; the CI step ends `boot || exit 1`.
- **Two hand legs beyond the listed entries** (`evidence/live-legs.md`): a timed boot, for when `boot` returns,
  and a never-ready boot, for the failure line. The never-ready leg set the gRPC port variable to 81, which the
  app refuses to bind and does not replace with a default; 4317 and 4318 were never bound.
- **One read-only fetch at this wrap left a side effect that was undone:** fetching the merge commit with
  `--depth=1` marked the repository shallow at that commit; a second fetch with `--unshallow` removed the mark
  (`git rev-parse --is-shallow-repository` reads `false`, no `.git/shallow`). No ref moved.
- scope record: none — `gate.py scope` clean, 0 recorded (`changed 7 · listed 7 · recorded 0 · excluded 69`).

## Decisions & corrections
- **The operator's rulings this chunk carries** (inputs#I3, inputs#I4, inputs#I5, inputs#I6): no cause named → no
  claim, whatever the count; the count of consecutive green boot-smoke readings runs to eleven from `e2931127`;
  the two kept files are routine harness evidence, with a stop if the display output carries telemetry; the
  operator pass was run by the agent on the operator's word; the wrap stops at the route-resolve card and before
  the flip and the commit.
- **A pull-request CI run reads the merge of the branch with `main`.** Its log's `git.commit.sha` is the merge
  commit. "Equal source" across such runs means equal merged trees: the branch tip AND `main`'s tip both fixed.
- **Sweep hazard:** the token `windows` on any line of `ci.yml`, a comment included, reddens
  `ci_workflow_runs_on_linux_only`. Write "the app's four window records" or reword.
- **Found, for an owner (inputs#I4 item 4):** `ci_workflow_test_gates_no_continue_on_error` stayed green under a
  soft-fail planted on the smoke step; its command list names no `agent-run.sh` verb. The new pin
  `ci_workflow_boot_smoke_carries_no_soft_fail` covers this one step. Whether the older guard's list is widened is
  not this chunk's.
- **A green-on-arrival pin proves nothing until mutated.** One of the four workflow pins could not read red before
  the edit; a planted key showed it discriminates.
- **A deterministic alive-but-never-ready app without a product change:** point one receiver's port variable at a
  privileged port. The app logs `config.load.port_validation`, does not start that receiver, does not fall back
  to the default port, and stays up.
- **`git fetch --depth=1 {sha}` into a full clone marks it shallow.** Fetch a single commit without `--depth`.
- **Recurred this session:** the Bash guard refused a leading `cd` out of the project root once (the read of the
  chunk's inputs); re-issued once in a subshell.
- **The runner has a session bus.** `session_bus` read `reachable` on `ci#37979648967` while its `boot.log` holds
  the accessibility-bus `ServiceUnknown` warning. Research's empty-bus arm assumed that shape; this is its first
  direct reading.

## Outcome
**Acceptance criteria, each re-asserted against the diff.**
1. *`boot` exits 0 only on `ready` (status `running-healthy` and a handshake on both ports); pinned per arm, red
   before and green after; the green leg's `boot` exits 0 inside 10 s with both bind records* — **met.** 8 pins,
   compile-red before the functions existed (`evidence/red-before-green.md`); the green leg and the runner both
   show the bind records before ready.
2. *On a failed poll with the app alive, `boot` names the receivers and runs `cleanup`; with the app gone it
   prints `app ended:`* — **met in the arm driven** (the never-ready leg: both lines, `cleanup: clean`, exit 1).
   The `app ended:` branch is unchanged in the diff and was not driven at this chunk. No committed test covers
   either.
3. *The green leg passes; its log holds four navigation records, 0 `app.panic.fatal`, 0 ERROR other than the
   keyring record* — **met.** `settled`, `running-healthy`, `cleanup: clean`; 0 ERROR, 0 panic (this host has a
   keyring).
4. *A run in which the app ends by itself says why in what the job keeps: the control prints `ended`, `"ended":
   "exit 1"`, `app_exit_record` `absent`, `display` `gone`, and the same object is in `logs/harness-settled.json`*
   — **met** on this host. It is a control, not a runner's run.
5. *`harness:status` keeps its object, arms and exit codes* — **met.** Its diff is visibility and one extraction;
   its 13 pins pass unchanged.
6. *The diff holds the seven listed files and no other; no product-consumed variable, port, procedure or
   capability; the two kept files sit under `logs/` and the product never reads them* — **met.** The three system
   variables are read by xtask alone.
7. *The verdict and its file carry closed labels, a pid, a count and the exit record only; both local legs'
   `xvfb.log` and the CI artifact's are read and none holds telemetry* — **met.** Five files read.
8. *`ci.yml` still holds six independent jobs on `ubuntu-22.04`; only the smoke step differs; no `uses:`,
   permission, soft gate, retry or job edge; the self-lint tests pass* — **met.**
9. *The `a11y` job is unchanged and its verdict is recorded beside the boot-smoke verdict* — **met.** a11y
   `success`, 298 s; boot smoke `success`, 446 s.
10. *The pushed tip's CI run is read whole and recorded with its boot-smoke verdict and settle verdict, whichever
    way it ends; not re-run* — **met.** `ci#37979648967`, green, `settled`.
11. *The workspace run returns 0, the standard gate set passes in order, the bindings close reads identical to
    the base* — **met at implement** (2826 passed); re-run at this wrap's light gate.
12. *No capability is claimed; P-129 stays pooled with a dated ledger note* — **met for the claim** (`matrix.py
    show --chunk`: claimed 0; P-129 `planned`, unclaimed). The note of this wrap is P7.3's.

**P-129 itself: not met, not claimed.** Its acceptance asks for a cause named from a run and closed, then green on
a stated number of consecutive runs. One clause is built: a run in which the app ends by itself says why in what
the job keeps.

**Gates** (the gate tool at implement, one call: `green 18 · red 0 · not-run 6`).
- `cargo fmt --check` — green, exit 0.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green, exit 0.
- `git diff --name-only 277d65d… -- crates pulse-app xtask …` (the scope guard) — green, exit 0, no output.
- `python … print(' '.join(sorted(d['jobs'])))` — green, last line `a11y boot coverage lint-test mcp-test supply-chain`.
- `python … 'of', len(d['jobs'])` — green, last line `6 of 6`.
- `python … b['jobs']['boot']['steps'] …` (the parsed workflow against the base) — green, last line
  `['boot'] ['Boot pulse-app smoke'] True True True True`.
- `git diff 277d65d… -- .github/workflows/ci.yml | grep -c -E '^\+.*(uses:|…)'` — green, exit 1, last line `0`.
- `d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-green" && …` (the green leg) — green, exit 0, all five
  `contains` atoms held.
- `d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-display-lost" && …` (the control) — green by its `contains`
  and `lacks` atoms; exit 1, as the plan says (no exit atom).
- `cargo xtask check:english-sources` — green.
- `cargo xtask capability-widening-check` — green.
- `cargo xtask check:ingest-progress` — green.
- `cargo xtask check:staged-artifacts` — green.
- `cargo xtask capability-drift` — green.
- `cargo xtask verify:capability-matrix` — green.
- `cargo nextest run --workspace --profile ci` — green, 2826 passed, 0 skipped.
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green.
- `git diff --quiet 277d65d… -- pulse-app/ui/src/bindings/index.ts` — green.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`; driven once in
  the pass: exit 0, `hygiene: clean`. Green.
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` — `leg =
  'operator'`; driven once: exit 0, `277d65db..e2931127`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` — `leg =
  'operator'`, report-only, `recorded`. **The outcome it read: `ci#37979648967` on `e29311270035`, `verdict: green
  · checks 7/7 · wall 1668 s`.** Disposition: this chunk's own run, green reading 1 of the count to eleven; it
  names no cause.
- `gh api "repos/Turbolet85/andromeds-pulse/actions/runs/<id>/jobs?per_page=100" …` — `leg = 'operator'`,
  report-only, `recorded`: six jobs, all `success`.
- `f="$(mktemp)" && gh api --allow-escape-sequences … /logs" …` — `leg = 'operator'`, report-only, `recorded`:
  `boot: ready`, the settle verdict `settled` with 4 windows and both labels `reachable`, status
  `running-healthy`, `cleanup: clean`.
- `gh run download <id> … -n logs-boot-Linux …` — `leg = 'operator'`, report-only, `recorded`: five files;
  `harness-settled.json` equal to the printed verdict; `xvfb.log` 0 B.
- **Smoke:** the plan's two smoke entries ran as gates; two hand legs beside them (`evidence/live-legs.md`).

**Watches:** none folded on this entry (its two blocks were `CARRY:`). The boot-smoke count this chunk starts:
1 green run [`ci#37979648967`], from `e2931127`.

**Outcome basis.** The operator pass ran, so the gate verdicts rest on its final state: the one commit `e2931127`
and that tip's CI run, recorded in `evidence/operator-pass.md`. Implement's P4 report, given in this session,
stays the basis for what only it holds (the red-before-green readings, the mutation check, the four local legs).
Between implement and this report: the operator's pass directive (inputs#I5), which changed no source. After it:
the wrap relay (inputs#I4) and the invocation directive (inputs#I6); post-implement artifacts
`evidence/operator-pass.md` and `target/boot-smoke/ci-37979648967/` (ignored).

**Process hygiene.**

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` and `Xvfb`, the green leg | this chunk's run (the gate tool) | terminated |
| `pulse-app` and `Xvfb`, the control leg | this chunk's run (the gate tool) | terminated (the leg stops `Xvfb`; the app ended `exit 1`) |
| `pulse-app` and `Xvfb`, the timed hand leg | this chunk's run (by hand) | terminated |
| `pulse-app` and `Xvfb`, the never-ready hand leg | this chunk's run (by hand) | terminated |
| the operator pass | — | none started on this host |

Re-measured at this wrap (`ps -eo pid,comm`, filtered for `pulse-app`, `Xvfb`, `xvfb-run`): no row; 14317 and
14318 refuse a connection.
## New text, by line
Generated by `cites.py added` (cites v1.3); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 277d65db (the parent of the oldest pre-CI commit e2931127) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 12 line(s) in 2 range(s)
added: 377-381 · 383-389
### pulse-app/tests/quality_gate_workflow.rs — added 130 line(s) in 1 range(s)
added: 434-563
- 434-449 @435 «fn boot_smoke_step_lines(content: &str) -> Vec<String> {»
  - 439-442 «let start = lines»
  - 443-448 «lines[start + 1..]»
- 451-466 «fn only_line_with(step: &[String], needle: &str) -> usize {»
  - 452-457 «let hits: Vec<usize> = step»
  - 458-464 «assert_eq!(»
- 468-500 @469 «fn ci_workflow_boot_smoke_reads_the_app_past_its_settle_inside_one_display() {»
  - 472-477 «assert!(»
  - 478-484 «let order = [»
  - 485-490 «assert!(»
  - 491-494 «let close = step»
  - 495-499 «assert!(»
- 502-526 @503 «fn ci_workflow_boot_smoke_runs_cleanup_whatever_settled_and_status_returned() {»
  - 508-515 «for (idx, capture) in [(settled, "; a=$?"), (status, "; b=$?"), (cleanup, "; c=$?")] {»
  - 517-520 «assert!(»
  - 521-525 «assert!(»
- 528-549 @529 «fn ci_workflow_boot_smoke_keeps_the_display_server_output_in_the_logs_artifact() {»
  - 533-538 «assert!(»
  - 540-543 «assert!(»
  - 544-548 «assert!(»
- 551-562 @552 «fn ci_workflow_boot_smoke_carries_no_soft_fail() {»
  - 554-561 «for banned in ["continue-on-error", "retry", "|| true"] {»
### scripts/agent-run.ps1 — added 17 line(s) in 4 range(s)
added: 45-56 · 140 · 202 · 217-219
- 46-55 «function Invoke-Ready {»
### scripts/agent-run.sh — added 15 line(s) in 5 range(s)
added: 8 · 53 · 101-106 · 109-110 · 126-130
### xtask/src/harness_ready.rs — new file · 660 line(s)
- 25-28 «use crate::harness_status::{»
- 47-85 «pub(crate) fn run_ready() -> Result<ExitCode> {»
  - 48-51 «let ports = (»
  - 52-81 «let payload = match (resolve_paths(), ports) {»
- 87-122 «pub(crate) fn run_settled(timeout_seconds: u64) -> Result<ExitCode> {»
  - 89-94 «let (verdict, pid) = match &paths {»
  - 95-99 @97 «let ended = paths»
  - 100-103 «let evidence = paths»
  - 106-109 «let session_bus = probe_label(&parse_session_bus(»
  - 111-118 «let text = serde_json::to_string_pretty(&settled_payload(»
- 124-130 «fn exit_status(verdict: &str) -> u8 {»
  - 125-129 «match verdict {»
- 132-140 @133 «fn decide_ready(status: &Verdict, grpc_accepting: bool, http_accepting: bool) -> &'static str {»
  - 134-139 «match status.arm {»
- 142-147 «fn resolve_port(raw: Option<&str>, default: u16) -> Option<u16> {»
  - 143-146 «match raw.map(str::trim) {»
- 149-151 «fn env_port(name: &str, default: u16) -> Option<u16> {»
- 153-156 «fn receiver_accepts(port: u16) -> bool {»
- 158-160 «fn receiver_label(accepting: bool) -> &'static str {»
- 162-164 «fn timeout_supports_verdict(timeout_seconds: u64) -> bool {»
- 166-182 @169 «fn decide_settled(»
  - 175-181 «match (pid, alive) {»
- 184-200 «fn wait_for_settle(»
  - 190-199 «loop {»
- 202-218 «fn read_exit_record(path: &Path, wait: bool) -> Option<String> {»
  - 203-208 «let deadline = Instant::now()»
  - 209-217 «loop {»
- 220-224 @221 «struct LogEvidence {»
- 226-251 «fn log_evidence(lines: &[String]) -> LogEvidence {»
  - 229-246 «for record in lines»
  - 247-250 «LogEvidence {»
- 253-257 «fn read_log_evidence(log_base: &Path) -> LogEvidence {»
  - 254-256 «crate::smoke::read_jsonl_lines(log_base)»
- 259-265 @261 «enum SocketTarget {»
- 267-269 «fn non_blank(raw: Option<&str>) -> Option<&str> {»
- 271-292 @273 «fn parse_display(raw: Option<&str>) -> SocketTarget {»
  - 274-276 «let Some(text) = non_blank(raw) else {»
  - 277-279 «let Some(local) = text.strip_prefix(':') else {»
  - 280-283 «let (number, screen) = match local.split_once('.') {»
  - 285-287 «if !digits(number) || !screen.is_none_or(digits) {»
  - 288-291 «match number.parse::<u32>() {»
- 294-315 @296 «fn parse_session_bus(address: Option<&str>, runtime_dir: Option<&str>) -> SocketTarget {»
  - 297-314 «match (non_blank(address), non_blank(runtime_dir)) {»
- 317-329 @319 «fn unix_socket_accepts(path: &Path) -> Option<bool> {»
  - 320-323 @321 «{»
  - 324-328 @325 «{»
- 331-337 «fn socket_label(probe: Option<bool>) -> &'static str {»
  - 332-336 «match probe {»
- 339-345 «fn probe_label(target: &SocketTarget) -> &'static str {»
  - 340-344 «match target {»
- 347-364 «fn settled_payload(»
  - 355-363 «json!({»
- 366-378 @368 «fn keep_verdict(text: &str) {»
  - 369-371 «let Some(logs) = resolve_data_dir()»
  - 372-374 «else {»
  - 375-377 «if std::fs::write(logs.join(KEPT_VERDICT_FILE), format!("{text}\n")).is_err() {»
- 380-660 @381 «mod tests {»
  - 384-390 «fn healthy() -> Verdict {»
  - 392-395 @393 «fn ready_requires_running_healthy_and_both_receivers_accepting() {»
  - 397-402 @398 «fn a_refusing_receiver_is_not_ready() {»
  - 404-412 @405 «fn a_dead_pid_is_ended_whatever_the_receivers_say() {»
  - 414-420 @415 «fn a_stale_log_or_an_absent_pidfile_is_not_ready() {»
  - 422-431 @423 «fn an_unresolvable_status_cannot_be_evaluated() {»
  - 433-441 @434 «fn exit_status_is_zero_only_for_a_passing_verdict() {»
  - 443-451 @444 «fn resolve_port_falls_back_to_the_default_and_rejects_garbage() {»
  - 453-462 @454 «fn receiver_probe_tells_a_listening_loopback_port_from_a_closed_one() {»
  - 464-471 @465 «fn a_dead_pid_is_ended_even_after_every_window_settled() {»
  - 473-479 @474 «fn four_settled_windows_and_a_live_pid_is_settled() {»
  - 481-488 @482 «fn a_live_pid_short_of_four_windows_polls_until_the_timeout() {»
  - 490-500 @491 «fn an_absent_or_unprobeable_pid_cannot_be_evaluated() {»
  - 502-507 @503 «fn a_window_shorter_than_the_settle_record_is_refused() {»
  - 509-518 «fn record(target: &str, window_label: Option<&str>) -> String {»
  - 520-533 @521 «fn log_evidence_counts_each_known_window_once() {»
  - 535-545 @536 «fn log_evidence_reads_all_four_windows_and_the_exit_record() {»
  - 547-563 @548 «fn display_parser_reads_a_local_display_only() {»
  - 565-593 @566 «fn session_bus_parser_reads_a_unix_path_or_the_runtime_dir_socket() {»
  - 595-614 @597 «fn socket_probe_tells_a_listening_unix_socket_from_a_dead_or_absent_one() {»
  - 616-621 @617 «fn an_unset_or_unreadable_target_is_labelled_without_a_probe() {»
  - 623-659 @624 «fn settled_payload_carries_the_closed_field_set() {»
### xtask/src/harness_status.rs — added 18 line(s) in 7 range(s)
added: 123-124 · 132-142 · 159 · 204 · 210 · 217 · 226
- 132-137 «fn env_path(name: &str) -> Option<PathBuf> {»
  - 133-136 «std::env::var(name).ok().and_then(|v| {»
- 139-141 «pub(crate) fn resolve_data_dir() -> Option<PathBuf> {»
### xtask/src/main.rs — added 16 line(s) in 3 range(s)
added: 13 · 53-65 · 273-274
  - 62-65 «HarnessSettled {»
