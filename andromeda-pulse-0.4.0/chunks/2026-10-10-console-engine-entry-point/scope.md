# Scope — Console engine entry point

**Marker:** `2026-10-10-console-engine-entry-point` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:29`):** "Console engine entry point — one program boots ingest, buffer,
detectors, corpus with no display, driven by commands; log, identity, panic, heartbeat, process-end records kept
(P-086)"
**Chunk base:** `f31027a4` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
Today the engine exists only inside one application binary that is a Tauri process, with no headless mode (intent
F1, OBSERVED). This version makes it a console program that runs in the background on a Linux server and is driven
by commands, with nothing in it needing a display, a GPU or a desktop session (F1, EXPECT). This chunk is the first
that builds that form: one program that boots the engine's ingest, buffer, detectors and corpus with no display,
and keeps the engine's own records of itself. The window app stands beside it until Epoch 2 removes it.

## What the chunk builds (the entry's own words)
- **One program boots ingest, buffer, detectors, corpus with no display.** A second way to start the same engine,
  as a console program: the two OTLP receivers, the ring buffer, the detectors and the incident corpus come up with
  no window, no webview and no display server.
- **Driven by commands.** The program is started and told what to do from a command line; it has no interface of
  its own. `[premise-corrected: read at P3 in observability.rs and the harness rules]` The entry does not state
  which commands exist at this chunk. It needs one that runs the engine; it does not need one that ends it. The
  engine ends by a signal sent to its pid: the existing signal listener records the process end and re-raises the
  same signal, the harness's `cleanup` verb already ends the app that way, and a product command that found the
  running engine would have to read the pid file, whose place is `One place on a node`'s. So this chunk's
  commands are `run` and `version`; an unknown command is refused. Token, door and node commands are later
  entries'.
- **Log, identity, panic, heartbeat, process-end records kept.** The console program writes the same five records
  of itself that the window app writes today: its log sink, its identity at boot, its panic record, its heartbeat
  and exactly one record of how the process ended. The list is closed (inputs#I1 item 4).

## What the directive adds (inputs#I1 — each bullet closed at P3)
- **The size of the step is the first question** (inputs#I1 item 1). The directive's own reading, kept verbatim
  with its marker: "measured on 2026-10-08 at `18a872d`, stale by nine chunks — re-measure before leaning on any
  of it: the engine lives inside the Tauri process; `tauri` is a dependency of `crates/ingest`, `crates/triage`,
  `crates/ui-bridge` and `pulse-app`; two binaries exist (`pulse-app`, `andromeda-pulse-mcp`); `main.rs` has no
  headless switch; 18 of 42 files of `pulse-app/src` name tauri or taurpc."
  `[premise-corrected: re-measured at the chunk base f31027a4 (research, Patterns detected)]`
  - Holds: the engine lives inside the Tauri process, wired whole inside one `fn main()` with every task spawned
    from the Tauri setup closure; the product binaries are `pulse-app` and `andromeda-pulse-mcp`; `main.rs` has no
    headless switch.
  - Corrected: `tauri` is a dependency of `pulse-app` and, optionally, of `crates/ui-bridge`, behind a feature
    `xtask` already turns off. `crates/ingest` and `crates/triage` carry an optional `specta`, not `tauri`. The
    count is 18 of 43 files.
  - The cost: no crate's dependency on the window framework has to be cut for this chunk. None of the
    engine-wiring modules imports the window framework, and a binary that links the `pulse_app` library without
    building the window needs none of the window's shared libraries at load (0 of 103 test binaries; the window
    binary needs 6). The cost is in files: 6 modified and 9 new, all under `pulse-app/` (research, the two lists).
- **It fits one chunk** (inputs#I1 item 2). `[premise-corrected: measured at P3]` No cut is proposed. The step is
  one refactor (the engine boot lifted out of `main()` into one function), one new program that calls it, and
  their tests. Cutting `tauri` out of the crate's manifest is not needed for a console program that runs with no
  display, and it is what `Window retired` does to the whole crate; doing it now by moving the engine into a new
  crate would move most of the 25 files of `pulse-app/src` that do not name the window framework, with
  `observability.rs`, and add a seventeenth workspace member, for a state Epoch 2 reaches anyway.
- **The window app stays whole while it stands** (inputs#I1 item 3). Verified at P3 as a set of pins the change
  must keep: `main()`'s first two statements and its own runtime build are held by source text
  (`pulse-app/tests/unit_xlib_threads.rs`); the boot job reads an `app.boot.window.navigation` record for each of
  four windows and spawns `target/release/pulse-app` by path; the a11y job reads the mocked webview and not the
  engine. Until `Window retired` (`working-route.md:50`) the window app boots, and its boot job, its eight boots
  and its a11y job stay green. The plan says how one engine serves both programs until Epoch 2, and what forbids
  the two from drifting apart in between.
- **No local model in the console program** (inputs#I1 item 5). It does not start, load or wait for a model, and
  what the window app does with one today is not moved. Verified at P3: an incident is created only behind the
  model's output (`create_incident_from_l4_output`, reached through the interpretation subscriber alone), and the
  one existing way through that point with no model is the canned runner the app selects when
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` is set. What the console engine does at that point is a fork, brought at P4.

## Decided at P4 (the operator's answer, the pc overseer, 2026-10-10 — inputs#I2)
- **The console engine follows the registered variable.** With `ANDROMEDA_PULSE_L4_DETERMINISTIC` set it seats the
  canned runner and an incident forms; with it unset it seats nothing: cues and digests only, no incident, and the
  boot record says why. One meaning for the variable, and fixture text never stands as a finding of an ordinary
  run.
- **The unset arm has an owner:** `Incident is the engine's own record` (`working-route.md:65`) gives the console
  engine an incident of its own; until then an ungated console run reports none.
- **Both arms are pinned:** set, an incident forms; unset, none forms and the boot record says why.

## What this chunk leaves for the next entries (inputs#I1 item 4 — named so none is assumed done)
- The five harness verbs targeting the console engine, their verdict arms, and the panic, heartbeat-gap,
  process-end and budget checks over its log: `Agent harness drives the console engine` (`working-route.md:31`).
- The door inside the engine's process and the end of the stdio sidecar: `Door inside the engine's process`
  (`working-route.md:35`).
- The gap-and-resume and external-resolve scenario legs: `Scenario legs driven against the console engine`
  (`working-route.md:37`).
- Operator-set locations for stores, log and pid: `One place on a node` (`working-route.md:39`).
- The end-to-end gate in CI: `Engine end-to-end gate reachable` (`working-route.md:43`).
- The detection baseline read through the door: `Detection baseline through the console engine`
  (`working-route.md:45`).
- What gate this chunk itself ends with, given that the harness verbs and the end-to-end gate are later entries':
  closed at P3. A test of the crate spawns the built console program with no display variable and no session bus,
  on its own data dir and on ports it picks, sends telemetry over the real receiver, and reads the program's own
  log after it has ended. It runs inside the workspace test run, which CI's `lint-test` job and the local pre-push
  check both run with no display, so the program is run by a check on every push with no workflow edit. A program
  that is built and never run by any check is not an end.

## Capability
- **P-086** (`verification-matrix.json#P-086`, `planned`, unclaimed). Acceptance: "The engine starts as a console
  program on a Linux server with no display, no GPU and no desktop session, runs in the background and answers its
  commands; every kind of finding 0.3.0 detected, one incident for a sustained storm and automatic resolution are
  each still observed on the same telemetry."
  `[premise-corrected: read at take-up over the working route]` The directive says "P-086 is carried by four
  entries" (inputs#I1 item 7); six entries name it: this one and `working-route.md:31`, `:37`, `:43`, `:45`, `:81`.
  Either count gives the same reading: this chunk advances P-086 and does not claim it. P5 shows what of its
  acceptance this chunk can show, and claims nothing that was not shown.

## Boundaries (inputs#I1 item 6)
- A listening socket other than the two OTLP receivers the app binds today, a new place on disk, a new credential,
  or anything reachable from another host halts for the founder's own word. The console program binds the same two
  loopback receivers and nothing more.
- Ports 4317 and 4318 are shared on this host: the operator is asked before any run that binds them.
- No surface is removed here. The window, the local model, the stdio sidecar, the bridge and their tests stand
  after this chunk as before it; Epochs 2 and 3 remove them.
- The spec masters are read-only at phase; what this chunk changes in them is the wrap's.

## Process items carried by the directive (inputs#I1 items 7 and 8)
- The plan lists the merge-base probe directly before its push entry.
- At P5: what `planlint` check 4 listed, what of P-086's acceptance this chunk can show, the plan card, and a stop
  for the operator's `yes`.

## Folded freight
- none — the entry carries no freight block (`route.py pins`: 33 blocks on the markerless tail, no row for
  `working-route.md:29`).

## Second fold source — the CI verdict read at Setup 5a
- One sha since the last wrap's flip: `f31027a4` (the wrap's own commit; the flip is in it). `ci#38052998981` on it
  read **in progress** at take-up (checks 7/7 listed, 6 running, the oldest the a11y job at 147 s);
  `secret-scan#38052998980` completed, success. At take-up and once more at P4 (1 running, the coverage gate at
  1322 s) the verdict was not yet available, and it was not folded or read as green then.
  **Read again at P5, after the run closed: `verdict: green`, checks 7/7, wall 1712 s** (`ci#38052998981`
  completed, success; attempt 1). No red and no `not green` is carried into this chunk.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py cursor`: no annotation-position line; `route.py pins`: no row
  for line 29).
