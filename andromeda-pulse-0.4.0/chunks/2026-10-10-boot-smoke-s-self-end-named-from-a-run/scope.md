# Scope — Boot smoke's self-end named from a run

**Marker:** `2026-10-10-boot-smoke-s-self-end-named-from-a-run` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:19`):** "Boot smoke's self-end named from a run — the job keeps what names who
ended the app; cause closed; equal source reads the same (P-129)", with one `CARRY:` block folded below.
**Chunk base:** `8936f976` (HEAD at take-up, equal to its upstream). Every diff-shaped probe of this chunk names this
sha, never HEAD.

## Intent
Requirement P-129 reads: "the CI boot smoke ends the same on equal source: the cause of the app ending by itself
shortly after ready on the runner is found and closed, and the job no longer reads red where nothing it builds
changed" (`matrix.py show --id P-129`, read at take-up). The `boot smoke (ubuntu-22.04)` job has read red on four
of the twelve pull-request runs of the build branch since `b3ac58a` (re-derived at P3; three of ten when the entry
was minted). The job's own instruments say how the app ended and that its display and session bus were still
there, and they do not say who ended it. This chunk makes a run keep what names who ended the app, names the cause
from such a run, closes it, and shows equal source reading the same.

The operator's direction for this chunk (inputs#I1, the pc overseer's phase directive; inputs#I2, the invocation
word): the cause is named from what a CI run keeps, never from a local reproduction; two questions go to research
as measurements; the chunk adds no retry, no soft-fail key and no timing pad; the plan card is printed at P5 and the
chunk stops there for the operator's `yes`.

## What the chunk builds
- **The boot job keeps what names who ended the app** (the entry). The freight lists what the job does not keep
  today: "any record of the call that ended it (no stack or trace at the exit), the exit statuses of the webview's
  own child processes, anything the kernel or the session logged at that instant". The chunk makes a run in which
  the app ends by itself keep a record that names the ending call and its caller.
  - **Which instrument records it** (inputs#I1 item 3a), answered at P3 by measurement (research.md "The
    instrument"): the runner image as it is carries no tracer (`strace`, `ltrace`, `gdb`, `bpftrace`, `perf` are not
    in the image's own list) and carries a C compiler. A small library loaded into the app process alone, which
    records the exit call with its caller frames, was built with that compiler's kind on the dev host and read on a
    known member of the silent `exit 1` class (the display stopped): it named `_exit(1)`, called from a static
    function of `libgdk-3.so.0` entered from Xlib's `_XIOError`, with the thread and `errno`, 2 of 2. On six green
    boots against six without it no timing difference was visible (the webview's first record at 0.869 s against
    0.911 s median, four windows settled at 5.173 s in both arms) and it wrote nothing. Its cost on the runner is
    not measured; the chunk's first run reads it on a green boot before a red one is trusted to it.
  - **Whether one run can hold several independent boots** (inputs#I1 item 3b), answered at P3 (research.md
    "Several boots in one run"): yes, inside the recorded workflow shape, as a step of the `boot` job that boots
    again on a fresh data dir under its own display server, sharing the job's one build. Booting again does change
    what is measured in two named ways: the webview keeps state under the user's data directory
    (`com.andromeda.pulse/` with `WebKitCache`, `CacheStorage`, `storage`), which a second boot finds unless it is
    given its own (measured: with `XDG_DATA_HOME` and `XDG_CACHE_HOME` moved the state lands there and the app
    settles); and the file cache is warm. Every recorded red was the first and only boot of a fresh runner. So a
    series records each boot's position, and whether later boots end at the first boot's rate is read from the
    series itself, not assumed.
  - **A design that waits for a natural red waits about three runs for one reading** (inputs#I1 item 1). Verified at
    P3: under the present step the `boot` job has six readings, two red (`cb8cc4dc`, `8936f976`). The plan states
    how many readings its design needs and where they come from.
- **The cause is named from a run** (the entry's title; P-129's acceptance: "named from evidence a run produced, not
  inferred"). Never from a local reproduction (inputs#I1 item 1). Verified at P3: the previous chunk's research
  holds "0 self-ends in 51 boots", and this chunk's thirteen further local boots of the unchanged sequence all
  settled. A local boot measures an instrument's cost and shape; it does not name the cause.
- **The cause is closed** (the entry). Naming and closing may be two steps (inputs#I1 item 4: "A change that names
  the cause and a change that closes it may be two steps; if the cause cannot be closed in this chunk, say so at P5
  and claim nothing of P-129 that was not shown"). Which of the two this plan is, is P4's to state.
- Added at P4, the operator's three rulings (inputs#I3):
  - **This plan names; the closing is the next entry.** A run can name the cause only after the operator's pre-CI
    commit, and past it no plan revision is offered. The pass stops at the first witnessed self-end and says what
    the job kept; closing is planned from that record as its own entry at the wrap. P-129 is not claimed here.
    When the first run holds no self-end, the `boot` job alone may be re-run on the same commit, "at most three
    times, to collect equal-source readings - every attempt is recorded with its number, none replaces another,
    and the chunk CI verdict stays the first attempt."
  - **The kept witness file is the app's exit record, routine harness evidence**: "Operator reading (pc overseer),
    recorded as mine and not the founder word: it holds less than the app log the job already uploads." Two
    conditions: each of the three ways a process ends with a code (a call through the linker, a direct `_exit`,
    `main` returning) gets a known-positive control, "and a self-end that leaves no witness line reads as its own
    named verdict, never as nothing"; and the plan says how the library stays out of the webview's child processes
    and proves it by a control.
  - **A gating series, eight boots a run.** "The defect stays visible until it is closed. Record each boot ordinal
    with its verdict: a self-end rate that differs between the first boot and the later ones is itself a reading
    to report, not noise." What the series does not reset between boots is named in the plan.
- **Equal source reads the same** (the entry; P-129). The acceptance names "a stated number of consecutive runs of
  equal source" reading green, counted with the cause closed. A pull-request run builds the branch merged with
  `main`, so a reading is of equal source only while both tips stand. Verified at P3: the logs of the runs on
  `8936f976` and `b7101914` carry `git.commit.sha` `9df2c7b57a2e` and `4a79cbbbf22d`, neither the pushed tip.
- **The records that describe the boot job and what it keeps are corrected at the wrap, not here.** Phase amends no
  spec source.

## The causal and measured claims the freight carries
Each was closed at P3 against the run's own artifact (`logs-boot-Linux` `11653875886` of `ci#38010977166`, read
record by record; research.md "The recorded CI runs").
- **Verified.** "what the job kept on this run: settle verdict ended, exit 1 recorded by the waiting wrapper as an
  exit and not a signal, no app.exit record, windows_settled 0, display reachable, session bus reachable, xvfb.log
  0 B, the application log 53 records long and ending 0.072 s after the webview's first call (ui-bridge.ready), no
  app.panic.fatal". Read: 53 records, `ui-bridge.ready` at 0.835 s, the last record at 0.907 s.
- **Verified.** "boot.log holds the app's stdout and stderr whole (scripts/agent-run.sh:79 redirects both to it) and
  holds one line, the accessibility-bus warning, the same line the three green artifacts read hold". The same one
  line stands in all eight artifacts read at P3.
- **Verified, with its reading kept as a reading.** "the three green application logs each hold four
  app.boot.window.navigation records written 4.1 to 4.7 s after that run's ui-bridge.ready and this log holds none —
  its process ended about four seconds before the point a green run writes them, so their absence is the early end
  itself (measured at that chunk's wrap, from the four logs-boot-Linux artifacts)". Re-derived: 4.129 s, 4.382 s and
  4.696 s on the three greens, 4.606 s on a fourth (`ci#38014639286`).
- **Verified as stated, and it excludes less than it may seem.** "the display stopping, which the previous chunk's
  local control reproduced with display gone, is not what this run shows". The run's settle verdict reads `display
  reachable`: the display server was accepting connections after the app ended. That reading does not show that
  the app's own connection to it was sound. The control's ending call is Xlib's I/O error handler (research.md "The
  instrument"), which a client reaches when its connection fails, whether or not the server lives. No run has shown
  which call ended the app on the runner.
- **Verified, and moved on.** "the third red of twelve readings and the first one the job's own instruments read;
  the twelve readings are every run of the ci workflow created from b3ac58a's on ...; in each red the one failed
  job is boot smoke". Re-derived at P3 (`gh api …/workflows/ci.yml/runs`): fourteen runs now, twelve pull-request
  runs on the build branch with four red (`b3ac58a9`, `f36a2ac3`, `cb8cc4dc`, `8936f976`), one on
  `hotfix/p-120-audit-step` green, one push on `main` green; the one failed job of each red is `boot smoke
  (ubuntu-22.04)`.
- **Verified.** From the relay (inputs#I1 item 2), the same ground restated: "the process ended by an exit and not a
  signal, code 1, no `app.exit` and no `app.panic.fatal` record, display and session bus reachable, `boot.log`
  holding stdout and stderr whole with nothing new in it." It holds on `ci#38014971549` too.

## Capability
- **P-129 is this entry's one capability.** Its acceptance: "The cause of the early exit is named from evidence a
  run produced, not inferred; with it closed, the boot smoke reads green on a stated number of consecutive runs of
  equal source, and a run in which the app ends by itself says why in what the job keeps." Whether this chunk claims
  it is P4's and P5's reading. If the cause cannot be closed here, nothing of P-129 that was not shown is claimed
  (inputs#I1 item 4). P-129 is PROVISIONAL until the founder's own word (the ledger's notes; the handoff).

## Boundaries
- **No retry that turns a red into a green, no soft-fail key, no timing pad placed to make the symptom rarer before
  the cause is named** (inputs#I1 item 4: "each would be the smoothing this entry exists to end").
- **The boot job's own steps and what it uploads are this chunk's. "A new workflow trigger, a new permission, a
  secret, or an artifact that could carry more than the app's own log and exit record halts for the founder's own
  word"** (inputs#I1 item 5).
- **The window, its bridge and the boot job leave at `Window's gates retired` and `Window retired`; "this chunk
  repairs nothing for a surface beyond what naming and closing this cause needs"** (inputs#I1 item 6). Both entries
  stand in the route (`working-route.md:44`, `:46`).
- **No CI run is re-run for a green** (the handoff; the previous two chunks' standing rule). A run is a reading.
- **`No CI step reads nothing` is the next entry** (`working-route.md:21`). Its work is not this chunk's, even with
  `ci.yml` open.
- **Ports 4317 and 4318 are shared with conductor-builder** (the handoff's host note). Every local boot of this
  chunk moves the receivers off them through the two port variables, or the operator is asked first.
- **At P5 `planlint`'s listing of gate entries that read what the wrap itself writes is read row by row and each
  row is named in the card** (inputs#I1 item 7). **The plan card is printed at P5 and the chunk stops there for the
  operator's `yes`** (inputs#I1 item 7, inputs#I2).

## Folded freight (the entry's one block)
- carry: the boot-smoke watch recurred on ci#38010977166 (pushed tip cb8cc4dc, the merge it built 3a05394f; record:
  2026-10-09-pre-push-check-native-on-linux's report and its evidence/operator-pass.md): the third red of twelve
  readings and the first one the job's own instruments read; the twelve readings are every run of the ci workflow
  created from b3ac58a's on, read 2026-10-10 — ten pull-request runs on the build branch (red on b3ac58a9
  ci#37924991598, f36a2ac3 ci#37961031489 and cb8cc4dc; green on 0b61bfbe, 569604be, b3e58597, 277d65db, e2931127
  ci#37979648967, 8a89e368 ci#37986543962, 8f655cae ci#37990081874), one pull-request run on hotfix/p-120-audit-step
  (5b307e4c, green) and one push run on main (178ebac5, the merge of that hotfix, green); in each red the one failed
  job is boot smoke; what the job kept on this run: settle verdict ended, exit 1 recorded by the waiting wrapper as
  an exit and not a signal, no app.exit record, windows_settled 0, display reachable, session bus reachable,
  xvfb.log 0 B, the application log 53 records long and ending 0.072 s after the webview's first call
  (ui-bridge.ready), no app.panic.fatal; boot.log holds the app's stdout and stderr whole (scripts/agent-run.sh:79
  redirects both to it) and holds one line, the accessibility-bus warning, the same line the three green artifacts
  read hold; the three green application logs each hold four app.boot.window.navigation records written 4.1 to 4.7 s
  after that run's ui-bridge.ready and this log holds none — its process ended about four seconds before the point a
  green run writes them, so their absence is the early end itself (measured at that chunk's wrap, from the four
  logs-boot-Linux artifacts); the display stopping, which the previous chunk's local control reproduced with display
  gone, is not what this run shows; what the job still does not keep that would name who ended the process: any
  record of the call that ended it (no stack or trace at the exit), the exit statuses of the webview's own child
  processes, anything the kernel or the session logged at that instant; a pull-request run builds the branch merged
  with main, so a reading is of equal source only while both tips stand; the red runs' artifacts 11613618010 and
  11632850540 expire 2026-10-23 and 11653875886 expires 2026-10-24; the WATCH this replaces retired at the
  recurrence and does not go on counting; minted first in the tail on the operator's word (the pc overseer,
  2026-10-10, given in that chunk's wrap directive and again at its route-resolve card).
  **Absorbed as:** the chunk's subject, in "What the chunk builds" and "The causal and measured claims" above. The
  retired watch folds nothing: it does not go on counting, and this chunk owns the closing.

## Second fold source — the CI verdict read at Setup 5a
The last master flip is `b7101914`; two shas were read through `ci.py conclusion` (`b7101914` through HEAD). Both
read **verdict not yet available** at Setup (in progress at 2026-10-10T01:57Z) and were read again at P3 once the
`boot` job of each had ended.
- **`ci#38014971549` on `8936f976b129` (the chunk base): red, the one failed job `boot smoke (ubuntu-22.04)`**
  (job `114103109878`; `ci.py conclusion` at P3: `verdict: red · first-fail +622 s boot smoke (ubuntu-22.04)`). Its
  failing subject is this chunk's own, so it is folded here and closed against that run: settle verdict `ended`,
  `exit 1`, no `app.exit` record, `windows_settled 0`, display and session bus `reachable`, `xvfb.log` 0 B, the log
  57 records long with its last record 0.345 s after `ui-bridge.ready`. It is the fourth red and the second the
  job's instruments read. It is a runner's reading: that it does not reproduce on this host is its expected state.
  The commit it sits on changed one tool script outside every job's build (`scripts/code-graph.py`, the U03
  upgrade, with four files of its own run dir; `git show --stat 8936f976`).
- **`ci#38014639286` on `b71019145e72` (the wrap commit): green** (`ci.py conclusion` at P3: `verdict: green ·
  checks 7/7 · wall 1640 s`; the `boot` job `114102094003`: settle verdict `settled`, four windows, the app alive
  5.624 s).
Both are readings of the job and are recorded with their verdicts. Neither was re-run.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py pins`: its one block is a `CARRY:`; `route.py blocked`: no
  block on a pending, gated or markerless line).
