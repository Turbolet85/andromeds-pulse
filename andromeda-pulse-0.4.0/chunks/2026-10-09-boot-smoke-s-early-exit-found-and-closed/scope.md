# Scope — Boot smoke's early exit found and closed

**Marker:** `2026-10-09-boot-smoke-s-early-exit-found-and-closed` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:15`):** "Boot smoke's early exit found and closed — the cause of the app ending
itself after ready is named; equal source reads the same (P-129)", with two `CARRY:` blocks folded below.
**Chunk base:** `277d65d` (HEAD at take-up, equal to its upstream). Every diff-shaped probe of this chunk names this
sha, never HEAD.

## Intent
Requirement P-129 reads: "the CI boot smoke ends the same on equal source: the cause of the app ending by itself
shortly after ready on the runner is found and closed, and the job no longer reads red where nothing it builds
changed" (`matrix.py show --id P-129`, read at take-up). The `boot smoke (ubuntu-22.04)` job read red on two of
eight runs of 2026-10-09 on source the job does not distinguish. This chunk finds out why the app ends, from
evidence a run produced, closes that cause, and makes a run in which the app ends by itself say why in what the job
keeps. It also closes the readiness gap the same logs showed: `boot` reports ready before the OTLP receivers bind.

The operator's direction for this chunk (inputs#I1, the pc overseer's phase directive; inputs#I2, the invocation
word): the cause is named from evidence a run produced; the early exit is reproduced, and its rate measured, before
any approach is chosen; the plan card is printed at P5 and the chunk stops there for the operator's `yes`.

## What the chunk builds
- **The cause of the app ending itself after ready is named** (the entry; P-129's acceptance: "named from evidence a
  run produced, not inferred"). A fix aimed at a cause no run has shown is not this chunk's outcome (inputs#I1
  item 2).
- [premise-corrected: 0 self-ends in 51 local boots across three arms, research.md "The local reproduction"] **The
  early exit did not reproduce on this host.** The relay's "hypothesis: the early exit reproduces locally at some
  rate" (inputs#I1 item 3) was measured at P3: 21 boots of the CI sequence unchanged, 20 boots holding the app 15 s,
  10 boots holding it under an empty private session bus that gives the log the runner's shape. No boot ended by
  itself. Host load was 22 to 33 during the readings (inputs#I1 item 7). So the cause is named from a runner's run,
  and this chunk's first work is what makes a runner's run able to show it.
- **What the job keeps today cannot show the cause, so the job is made to keep what would** (inputs#I1 item 2: "the
  exit path that writes no record is itself the finding"). Verified at P3:
  - The uploaded artifact is the data dir's `logs/` alone. The wrapper's exit record under `run/` is not in it. The
    display server's own error output is discarded by `xvfb-run`'s default, and nothing reads whether the display
    is still there after the app ends.
  - A control on this host produced the reds' three facts exactly: with the virtual display stopped 5 s after
    ready, the app ended `exit 1`, wrote no `app.exit` record, and added nothing to `boot.log`, 2 of 2. A second
    control, the session bus stopped, ended the app by SIGTERM with a record, 3 of 3, which is not the reds'
    signature. Both are controls. Neither shows what happens on the runner.
  - So a run in which the app ends by itself must say, in what the job keeps: how the app ended, whether it left an
    `app.exit` record, whether its display was still reachable, whether its session bus was still reachable, and
    what the display server itself printed. P-129 asks for this as a standing property ("a run in which the app
    ends by itself says why in what the job keeps").
- **The smoke reads the app after its windows have settled, not at whatever instant `status` happens to run.**
  Verified at P3: the smoke stops the app within 1.40 s on the runner, four of eight CI logs never reach the
  webview's first call, and 21 of 21 local boots of the unchanged sequence were stopped before it. No recorded run
  shows the app alive more than 0.27 s after that call. The app already emits a record when its windows have
  settled (`app.boot.window.navigation`, once per window, 5 s after boot); the smoke waits for it, bounded, or for
  the app's end. With that, equal source reads the same on every run: either the app lives past the point where
  both reds ended, or it ends and the job says what it saw.
- **The cause is closed, and equal source reads the same** (the entry; P-129). The acceptance names "a stated number
  of consecutive runs of equal source" reading green. Ruled at P4 (inputs#I3): the number is three, made by the
  operator pass's own pushes, and it counts only once a cause is named from a run and closed. With no named cause
  P-129 is not claimed at this wrap whatever the count ("three greens pass an unfixed job about 42 percent of the
  time"); the chunk then delivers what makes the next red show its cause, the entry stays open, and the count runs
  on as runs come, to eleven consecutive greens.
- Added at P4: **this plan is the second kind.** No run has named a cause, so the plan closes none and claims no
  capability. It delivers the readiness repair, the settle read and the kept evidence, and it reads its first CI
  run. If that run names a cause, the pass stops and says so; closing it is planned from that evidence.
- **`boot` reports ready only after it confirms a TCP handshake on the bound ports** (the entry's second `CARRY:`;
  inputs#I1 item 4: "The harness contract's clause (a TCP handshake on the bound ports) stays the contract").
  Verified at P3: `scripts/agent-run.sh:105-116` prints `boot: ready` when `cargo xtask harness:status` succeeds,
  with no port probe; the status verdict takes no port as input.
  - Verified at P3: the Windows twin has the same gap (`scripts/agent-run.ps1:186-199`). The two scripts are
    recorded as one surface in lockstep, and no CI job and no dev host runs the Windows one. The readiness decision
    therefore moves into xtask, where it is pinned, and each script changes the one verb it polls; the Windows edit
    is mirrored and stated as unwitnessed.
- **`ci#37971913620` on `277d65d` is this chunk's first reading of the job: green** (inputs#I1 item 5). Read at P3:
  `boot smoke (ubuntu-22.04)` `success`, job `113960407205`; the app was stopped before the webview's first call
  (42 records, none webview-originated), so this green says nothing about whether the app would have stayed up.
  Every CI run this chunk reads is recorded with its boot-smoke verdict. A run is a reading and is never re-run for
  a green.
- **The records that describe the smoke and the boot verb are corrected at the wrap, not here.** Phase amends no
  spec source. Known sites: test-plan §3, 5-command implementation (the Readiness signal clause and its open gap,
  the `boot` Command body's statement that the wrapper's streams go to `/dev/null`, which is true of the wrapper
  and not of the app, whose output goes to `logs/boot.log`); test-plan §9, the Boot smoke row; architecture
  §Occupied Resources, xtask CLI surfaces and Filesystem locations; the obs-plan and security-plan sites that state
  how long the CI app lives.

## The two causal claims the freight carries
- [premise-corrected: no recorded run decides it; none shows the app alive more than 0.27 s after the webview's
  first call, research.md "The recorded CI runs"] From the entry's first `CARRY:`: "hypothesis: the app ends itself
  a few hundred milliseconds after the webview's first calls on the runner, and the job reads red when status comes
  late enough to see it; not shown: a cause, or that the end is the window's and not the engine's". It stays a
  hypothesis. The plan rests on no part of it: the settle wait reads every run past that point, whichever way the
  hypothesis falls.
- **The measured readings hold on re-read.** From the same block: "measured 2026-10-09 on the boot job's application
  logs of seven runs ...: red on ci#37924991598 (b3ac58a) and ci#37961031489 (f36a2ac), green on five; on both reds
  boot printed ready and status read the app ended, exit 1, within 0.7 s; the red logs hold no record the green
  logs lack, no app.panic.fatal and no app.exit, and boot.log holds the one accessibility-bus warning all seven
  hold". Verified at P3 on five downloaded artifacts and the red job's own log (`ci#37961031489`, job
  `113923604622`: ready at 16:52:59.36Z, `"ended": "exit 1"` at 16:53:00.03Z). The count is now eight readings, two
  red. The two red runs' `logs-boot-Linux` artifacts `11613618010` and `11632850540` expire 2026-10-23.

## Capability
- **P-129 is this entry's one capability, and this plan does not claim it** (inputs#I3). Its acceptance asks for a
  cause named from a run and closed; no run has named one. One clause of it is built and proven here without a
  claim: a run in which the app ends by itself says why in what the job keeps. P-129 stays in the pool with a dated
  note. P-129 is PROVISIONAL until the founder's own word (the ledger's note; the handoff).

## Boundaries
- **The window is not removed or rewritten** (inputs#I1 item 6). It leaves at `Window's gates retired` and `Window
  retired` (Epoch 2). If the cause sits in the window alone, this chunk names it, closes it at the smallest place
  that makes equal source read the same, and says what of it leaves with the window anyway.
- **No product code changes under this plan as written.** The readiness verdict, the settle verdict and the kept
  evidence are harness and job work. A product change to close a named cause is planned when the cause is named.
- **A boundary widening halts for the founder's own word** (inputs#I1 item 8). None is planned. Ruled at P4
  (inputs#I3): the two files the job newly keeps are routine harness evidence, "the same class as boot.log,
  harness-written, never read by the product, registered at the wrap", with one stop: "If the display server output
  is seen to carry anything of a watched service telemetry, stop and say so."
- **No CI run is re-run for a green** (inputs#I1 item 5; the handoff).
- **`Pre-push check native on Linux` and `No CI step reads nothing` are the next two entries**
  (`working-route.md:17`, `:19`). Their work is not this chunk's, even with `ci.yml` or `scripts/agent-run.sh` open.
- **Ports 4317 and 4318 are shared with conductor-builder** (the handoff's host note). Every local boot of this
  chunk moves the receivers to 14317 and 14318 through the two port variables, so the shared ports are never bound.
- **The plan card is printed at P5 and the chunk stops there for the operator's `yes`** (inputs#I1 item 8,
  inputs#I2).

## Folded freight (the entry's two blocks)
- carry: measured 2026-10-09 on the boot job's application logs of seven runs (record:
  2026-10-09-supply-chain-job-same-on-push-and-pull-request's report, Outcome, watches): red on ci#37924991598
  (b3ac58a) and ci#37961031489 (f36a2ac), green on five; on both reds boot printed ready and status read the app
  ended, exit 1, within 0.7 s; the red logs hold no record the green logs lack, no app.panic.fatal and no app.exit,
  and boot.log holds the one accessibility-bus warning all seven hold; in the four logs that hold webview-originated
  records status read 0.23 s and 0.27 s after the first of them (app alive) and 0.49 s and 0.67 s after (app ended);
  the smoke stops the app within 1.40 s of its first record on every run, so no green log shows it alive longer;
  hypothesis: the app ends itself a few hundred milliseconds after the webview's first calls on the runner, and the
  job reads red when status comes late enough to see it; not shown: a cause, or that the end is the window's and not
  the engine's; the two red runs' logs-boot-Linux artifacts 11613618010 and 11632850540 expire 2026-10-23.
  **Absorbed as:** the chunk's subject, in "What the chunk builds" and "The two causal claims" above.
- carry: the harness contract's boot readiness signal says boot confirms a TCP handshake on the bound ports, and the
  shipped scripts/agent-run.sh reports ready on the harness:status verdict alone; on ci#37964887106 ready printed
  12 ms after the app's first record and the app was stopped having logged neither app.boot.otlp.grpc.bind nor a
  tick (measured 2026-10-09; test-plan §3, 5-command implementation, records the gap as open with this entry as
  owner); ready before the receivers bind is the engine's own matter too (the operator's word at the card,
  2026-10-09). **Absorbed as:** the readiness bullet above. The harness's readiness is this chunk's; what the
  engine itself reports before its receivers bind (the `ready` and `health` envelopes) is not changed here.

## Second fold source — the CI verdict read at Setup 5a
The last master flip is HEAD itself (`277d65d`), so one sha was read through `ci.py conclusion`:
- `277d65dbb51a`: **verdict not yet available** at Setup (in progress at 2026-10-09T18:18Z, `ci#37971913620`). It
  settled during this phase and was read again at P3: `verdict: green · checks 7/7 · wall 1190 s`, boot smoke
  `success`. Nothing is folded from it as a red. It is this chunk's first reading of the job, recorded above.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py pins`: its two blocks are both `CARRY:`).
