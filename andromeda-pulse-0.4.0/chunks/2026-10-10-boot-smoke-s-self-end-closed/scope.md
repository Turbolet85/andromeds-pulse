# Scope — Boot smoke's self-end closed

**Marker:** `2026-10-10-boot-smoke-s-self-end-closed` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:21`):** "Boot smoke's self-end closed — every self-end counted with its label; the
cause behind the ending call closed; equal source reads the same (P-129)", with four `CARRY:` blocks folded below
(`route.py pins`: 541, 1178, 612 and 500 characters; no other freight on the line, no `BLOCKED-ON`).
**Chunk base:** `95d17008` (HEAD at take-up, equal to its upstream `origin/build/andromeda-pulse-0.4.0`). Every
diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
Requirement P-129 reads: "the CI boot smoke ends the same on equal source: the cause of the app ending by itself
shortly after ready on the runner is found and closed, and the job no longer reads red where nothing it builds
changed" (`matrix.py show --id P-129`, read at take-up). The chunk before this one made a run keep what names the
call that ended the app. This chunk repairs the one measured defect of that instrument, finds why the ending call
is reached, removes that cause with the smallest change, and shows equal source reading the same.

The operator's direction for this chunk (inputs#I1, the pc overseer's phase directive; inputs#I2, the invocation
word): the relay is folded as an input; a close is a removed cause shown by a variation, never a rarer symptom; the
chunk adds no retry, no soft-fail key and no filter; the instrument's record does not grow; if the cause cannot be
closed from this repository the chunk stops at P4; the plan card is printed at P5 and the chunk stops there for the
operator's `yes`.

## What the chunk builds
- **Every self-end is counted with its label** (the entry; its first `CARRY:`; inputs#I1 item 6: "The series'
  undercount is this entry's first step"). This is the chunk's FIRST step, ahead of the close. The entry states the
  defect as: a boot that ends before the boot verb reads ready takes no settle verdict, so `boot-series.json` gives
  it no label although its witness file holds the line.
  - The defect's reading, verified at P3 from both runs' kept files: `boot-series.json` of `ci#38019133294` reads
    `ended` 2, `other` 4, and of `ci#38022477393` `ended` 2, `other` 3; each of the seven `other` boots has `cycle`
    `boot-failed`, five null settle members and an `end` line in its own witness file (research.md, The series'
    undercount).
  - The repair sits in the series verb (`xtask/src/harness_series.rs`) and reads what such a boot leaves in its
    own data dir: the spawn record, the exit record and the witness file, through the readers the settle verdict
    already uses (`harness_status::{read_pid, end_file, read_ended}`, `harness_witness::label`). Neither of those
    two modules changes. `[premise-corrected: research.md "The readers exist and are reachable"; the take-up left
    open whether harness_witness.rs or harness_ready.rs would change, and neither does]`
  - The smoke's own boot has the same gap and is read the same way: a first boot that ends before ready writes no
    settle verdict, so the series lists no ordinal 1 for it. Not stated by the entry; read at P3 in
    `series_payload` and the workflow's smoke step (research.md).
  - An app still running at the readiness timeout is ended by the boot verb's own cleanup; that end is the
    harness's and is not counted as a self-end (research.md, A constraint on the repair).
- **The cause behind the ending call is found** (the entry; its third `CARRY:`; inputs#I1 items 1, 4, 5). Found at
  P3 and recorded in research.md with its measurements:
  - The app uses Xlib from two threads (GTK's main thread, and a device-event thread its windowing library `tao`
    starts on X11) and never initialises Xlib's thread support. The runner's Xlib (`libx11-6 1.7.5-1ubuntu0.3`)
    does not initialise it at load; the dev host's 1.8.13 does. On the runner's Xlib a reply is then lost inside
    the app, Xlib raises an I/O error on a healthy connection, and GDK's handler calls `_exit(1)`.
  - (a) "which side ended this one client's connection": none did. At the stop the connection reports no error; the
    app's own side lost the reply.
  - (b) "is the self-end a property of the runner a run lands on": no. Two runs (7 of 8 and 5 of 8 boots) and the
    dev host under the runner's Xlib (19 of 24) all hold it.
  - (c) "do boots 2 to 8 run the binary boot 1 ran": they run a second build of the same source; the series' first
    boot relinks (3 min 27 s and 3 min 36 s). Why the runner relinks is not measured.
  - (d) GDK's message (inputs#I1 item 4): the handler logs at debug level, which is never written; nothing is lost
    at `_exit`.
- **The cause is closed** (the entry). What a close is here (inputs#I1 item 2): "The cause is removed and a variation
  shows it was the cause: on equal source, the series with the change reads no self-end and the series without it
  reads them. A change after which the symptom is merely rarer, with no variation behind it, is not a close." The
  plan states before P5 how many boots, over how many runs, the close is read on, and what rate an unclosed job
  would pass that count at.
  - The variation behind the cause was read at P3 on equal source (one binary, the runner's Xlib): with Xlib's
    thread support initialised first in the app process 0 of 8 boots self-ended, without it 19 of 24.
  - Where the change lands. `[premise-corrected: research.md, The cause; the take-up listed the workflow's boot
    step, the harness and the app's start as unknowns]` The cause is in the app's start, so the cause can be
    closed from this repository. The places a change could land are the app's start (the call made first), the
    workflow (a runner image whose Xlib initialises its threads at load) or the harness (a preload inside the
    app's process). Put to the operator at P4 and ruled (inputs#I3): **the app's start**. "the call is the first
    thing main() does on Linux, before any thread exists and before any library that can reach Xlib; say what it
    does on a host where the library cannot be found (inert, never a failed start) and pin both arms; the module
    leaves with the window - name its leaving owner. The close is read on the runner: state how many boots over
    how many runs, and the rate an unclosed job would pass that count at, using the 7 of 8 run and the run on
    95d17008 as the without arm. The swapped-library reading on this host goes into evidence with how the swap was
    made and undone." Placed at the P5 review (inputs#I4): "the render-posture step stays the FIRST statement of
    main, word for word, and the XInitThreads call is the SECOND, still before any thread exists and before any
    library that can reach Xlib (my P4 note said first thing; I withdraw that word - what matters is before
    threads and before Xlib). So no amendment of that ratified wording and no reworded comments there".
- **Equal source reads the same** (the entry; P-129's acceptance: "with it closed, the boot smoke reads green on a
  stated number of consecutive runs of equal source"). A pull-request run builds the branch merged with `main`, so
  equal source across such runs means both tips unchanged (P-129's note of 2026-10-09).

## What the entry measured, folded and re-derived
- The second `CARRY:`, verified at P3 against the downloaded artifact of `ci#38019133294`: "seven of eight boots
  ended by themselves and one settled (ordinal 6); the seven end lines are equal in every member but the process
  and thread ids — call _exit, code 1, errno 11, on the main thread, from a static function of libgdk-3.so.0
  (+0x76ccc) entered from Xlib's _XIOError, reached from _XReply under XGetWindowProperty under
  gdk_x11_screen_supports_net_wm_hint, out of a GObject signal emission dispatched from GLib's main loop under
  gtk_main_iteration_do; ... all eight xvfb.log are 0 B; no ended boot holds app.exit and no boot holds
  app.panic.fatal". Re-read: eight files, 15 lines, 0 beyond the shape, the seven `end` lines equal. The static
  function at +0x76ccc is `gdk_x_io_error` in Ubuntu's `libgtk-3-0 3.24.33-1ubuntu2.2`.
- Its limits, as they stand after P3: why the reply is lost is measured as far as "the connection is healthy and
  a second thread is inside Xlib", and the step inside Xlib is not traced; the runner's Xlib and GDK builds are
  read; the rate under the witness against the rate without it is read on the dev host (6 of 8 against 8 of 8), so
  the library does not raise it; the difference between a run's first boot and its later boots is not attributed.
- inputs#I1 item 5, verified: "`hypothesis:` the `errno` 11 in the witness line may be a stale value". It is: no
  connection error exists at the stop.
- inputs#I1 item 1, verified: the wrap commit `95d17008` started a pull-request run, `ci#38022477393`.

## The CI verdict
- `95d17008` (the last wrap's flip and HEAD, one sha). At Setup: **CI 95d17008: verdict not yet available** (in
  progress). Read again at P3 once closed: **red**, `ci#38022477393` (pull_request, attempt 1, the merge it built
  `71ce5340f2ad`), failed job `boot smoke (ubuntu-22.04)` alone, first failure at +938 s, wall-clock 1059 s; its
  other five jobs succeeded; `secret-scan#38022477402` success.
- That red is this chunk's subject (the entry's fourth `CARRY:`: "until this entry closes the cause the boot job
  reads red on most runs; that red is read by its per-boot verdicts (logs/boot-series.json and each boot's
  exit-witness.jsonl in the logs-boot-Linux artifact) and is this entry's, never a later chunk's to fix"). Closed
  against THAT RUN, a runner-only observation: 5 of its 8 boots self-ended (ordinals 3, 4, 5 before ready with no
  label; 7 and 8 after ready, `ended`, `exit 1`, `exit-call`), the smoke's own boot settled, and its five `end`
  lines equal run 1's seven. Still failing there; the bullet keeps its coordinates.

## Boundaries
- **Must not** (inputs#I1 item 3): "No retry, no soft-fail key, no filter that reads a self-end as anything but red,
  and no product change whose only purpose is to keep the app alive past the ending call."
- **The instrument's record does not grow** (inputs#I1 item 6): "The witness record's classification is
  PROVISIONAL: its shape does not grow here. A new member, a new kept file or anything read from inside the app's
  process beyond the present record halts for the founder's own word." Counting a self-end with its label reads the
  files the job already keeps or leaves in the boot's data dir; it adds no member to the witness line and no kept
  file.
- **The stop at P4** (inputs#I1 item 7): "If research shows the cause cannot be closed from this repository (the
  runner image, the display server, a library the job only consumes): stop at P4 and bring what was measured. Where
  the boot job goes then is a route question for me, not a design for this plan." Research shows the cause can be
  closed from this repository, so the stop does not apply; where the close lands is still the operator's choice.
- **Scope edge** (inputs#I1 item 8): "The window, its bridge and the boot job leave in Epoch 2. The close is the
  smallest change that removes the cause; nothing is improved for a surface on its way out."
- **The standing stop on witness files** (the handoff): every kept witness file is read whole, and a line holding
  anything beyond its shape stops the reading.
- **Out of scope:** the boot job's removal and what was built for it (`Window's gates retired`, its own `CARRY:`);
  the self-lint's reach over `ci.yml`'s run steps (`No CI step reads nothing`); the `agent-run.ps1` edits (`Other
  operating systems retired from the code`); why the series' first boot relinks on the runner.

## Surfaces and contracts the chunk touches
- `cargo xtask harness:boot-series` and its verdict file `logs/boot-series.json` (architecture §Occupied Resources,
  the `harness:boot-series` row; test-plan §9 Boot smoke and its failure conditions; obs-plan §9 Log file row). The
  member sets and the closed labels are kept.
- The app's start (the operator's rulings, inputs#I3 and inputs#I4): `pulse-app/src/main.rs` and one new module
  beside it; no environment variable is read or set, no log record is added, and no dependency is added. The
  render-posture step stays `main`'s first statement and its module is not touched; the new call is the second
  statement, before any thread exists. The workflow is not touched.
- `scripts/agent-run.sh boot` is not touched. `[premise-corrected: research names no cause in how the display
  server or the app is started, and the counting repair needs no script change]`
- Security: the exit-witness arm (security-plan §Security Anti-Patterns → Input) is read, not widened; its
  classification stays PROVISIONAL, awaiting the founder's own word.
- Host: ports 4317/4318 are shared with conductor-builder; a local run of `harness:boot-series` binds the resolved
  ports, so every local run moves them to 14317 and 14318 first (the handoff).

## Capability
- **P-129** — "The boot smoke's verdict follows the tree, not the run" (planned, unclaimed, PROVISIONAL until the
  founder's own word). Acceptance: "The cause of the early exit is named from evidence a run produced, not inferred;
  with it closed, the boot smoke reads green on a stated number of consecutive runs of equal source, and a run in
  which the app ends by itself says why in what the job keeps." Whether this chunk can claim it is P4's to decide;
  the working route names P-129 on lines 15, 19, 21 and 46 (15 and 19 frozen, 46 the boot job's removal).
