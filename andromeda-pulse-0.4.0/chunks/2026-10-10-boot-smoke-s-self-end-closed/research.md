# Codebase Research — 2026-10-10-boot-smoke-s-self-end-closed

## Scope
- **Depth:** deep · **Reads:** 26 · **Globs/Greps:** 31
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, Session Additions included; its
  `boot`, `cleanup`, `harness:settled` and `harness:boot-series` rows and the 2026-08-23, 2026-08-28 and 2026-09-29
  additions applied. `.claude/rules/testing.md` — the measure-first, mutation-check and vacuity additions applied.
- **Platform issues consulted:**
  - `gh search issues --repo tauri-apps/tao` for `XInitThreads`, `Fatal IO error 11`, `device event thread X11
    crash`: 0 issues each. For `xvfb`: #74 and #1047. #1047 (fetched, open, 2025-01-15), "Entire process is
    terminated when xserver is killed": the reporter sees the whole process exit when the display server is
    stopped; it names no cause and no thread-initialisation call. Not this failure.
  - `gh search issues --repo tauri-apps/tauri` for `Fatal IO error 11`: 0. For `XInitThreads`: #2221 (closed), an
    updater dialog race; not this failure.
  - `gh search issues --repo actions/runner-images` for `Fatal IO error 11 xvfb`: 0 issues of this signature.
  - The runner image's own software list (`images/ubuntu/Ubuntu2204-Readme.md`, fetched): Ubuntu 22.04.5, image
    `20261004.315.1`, `xvfb 2:21.1.4-2ubuntu1.7~22.04.16`.
  - libX11's tracker (`gitlab.freedesktop.org/xorg/lib/libx11/issues/21`): the fetch returned an access-denied
    page; nothing was read from it. What the two Xlib builds do at load was measured instead (below).
- **External inputs:** `inputs#I1` — the pc overseer's phase directive (one more run to read, what a close is, what
  the chunk must not do, the research question, the `errno` hypothesis, the instrument bound, the stop at P4, the
  scope edge). `inputs#I2` — the invocation word (fold the relay; at P5 print the plan card and stop).
- **Measurement record:** `.andromeda/runs/2026-10-10T04-02-31Z-phase/p3-measurements.md` holds every reading
  below with its command, the package checksums, the probe's source and the data dirs.

## What research found

### The cause, named from the runs and shown by a variation
1. **The ending call is Xlib's "no reply arrived" exit, in the runner's own Xlib build.** The three `libX11.so.6`
   frames of every `end` line (+0x41393, +0x4660f, +0x255a8) are exact return addresses in Ubuntu's
   `libx11-6 1.7.5-1ubuntu0.3`: 0x4660f follows the `call _XIOError` inside `_XReply` whose next instructions
   return 0, which is the function's exit for a request that got neither a reply nor an X error
   (`objdump -d` on the package's `libX11.so.6.4.0`, sha256 `d382a306…7d547` for the package). The runner's
   library build, which the prior chunk listed as not read, is read.
2. **Both red runs hold the same line.** `ci#38019133294` (attempt 1, tip `925be35f`, merge `ab6a1ae6ef0d`): 7 of
   8 boots self-ended. `ci#38022477393` (attempt 1, tip `95d17008`, merge `71ce5340f2ad`): 5 of 8. Twelve `end`
   lines, equal in every member but `pid` and `tid`; 28 witness lines in 16 files read whole, 0 beyond the shape.
3. **It reproduces on the dev host once the runner's Xlib is swapped in.** The same release binary under
   `cargo xtask harness:boot-series --count 8`, with `LD_LIBRARY_PATH` naming the two jammy libraries: 6 of 8
   boots self-ended with the exit witness loaded, 8 of 8 with nothing preloaded, 5 of 8 with
   `WEBKIT_DISABLE_COMPOSITING_MODE=1`. The six witness lines are the runner's call chain (`_exit`, code 1, `errno`
   11, main thread, the same three Xlib offsets). With the host's own Xlib 1.8.13: 0 of 8.
4. **The variation.** Same binary, same jammy Xlib, one difference: `XInitThreads()` called first in the app
   process (a scratch preload, never in the tree). 0 of 8 self-ended, 8 settled with four windows each. Without it
   on that Xlib: 19 of 24.
5. **Why the two Xlib builds differ.** The host's libX11 1.8.13 has its thread support initialised when the
   library loads; the runner's 1.7.5 does not (`_Xglobal_lock` read at load by a ten-line program: set, not set).
   On 1.7.5 a program that uses Xlib from more than one thread has to call `XInitThreads()` before any other Xlib
   call.
6. **The app uses Xlib from two threads and never makes that call at start.** `tao 0.35.0` (the locked version)
   starts a thread at event-loop creation on X11 that opens its own display and loops on `XNextEvent`
   (`tao-0.35.0/src/platform_impl/linux/device.rs:15-32`, spawned at `event_loop.rs:263`). Its one
   `XInitThreads` call is in `x11/xdisplay.rs:48`, inside a constructor reached only through
   `platform/unix.rs:246`, not at start. `tao 0.35.3` and `0.37.1` have the same two sites, so a dependency bump
   does not change it. `grep -rn XInitThreads pulse-app/src crates xtask/src scripts`: 0 lines.

### The research questions
- **(a) Which side ended the connection: none did.** Under `gdb`, stopped at `_XIOError` on the main thread (the
  runner's chain), `xcb_connection_has_error` on that display's connection reads **0** and `errno` reads 11. The
  connection is healthy; the server went on serving it. Xlib raised an I/O error because its own bookkeeping held
  no reply for the request. At that instant one other thread was inside Xlib: `XNextEvent` called from the app
  binary at the offset of tao's device thread. So it is the app's own side: two threads in Xlib without Xlib's
  thread support. One capture of one ending; the variation in point 4 is what shows the cause. The step inside
  Xlib by which the reply is lost was not traced.
- **`hypothesis:` the `errno` 11 may be a stale value (inputs#I1 item 5): verified.** No connection error exists
  at the stop, so no system call failed there; 11 is left over from an earlier non-blocking read.
- **(b) Is it a property of the runner a run lands on: no.** Two runs on two runner instances both hold it (7 of 8,
  5 of 8), and so does the dev host under the runner's Xlib. It follows the library set and a race in the app's
  first second, not the machine. First boots, as the matrix's P-129 notes carry them and not re-derived here: 4
  of the twelve runs before the series ended by themselves; then run 1's did and run 2's did not (5 of 14).
- **(c) Do boots 2 to 8 run the binary boot 1 ran: no, they run a second build of the same source.** In both runs
  the series' first boot took 3 min 27 s and 3 min 36 s to reach ready, the length of the smoke's own release
  build (`Finished release … in 3m 45s` in boot 1's `build.log`), and each later cycle took at most 12 s. The five
  `pulse-app` frames of every `end` line have equal offsets in boot 1's file and the series' files. Why the runner
  relinks is not measured (no series boot keeps its `build.log`); on the dev host a shell-launched build after
  four series compiled nothing.
- **(d) GDK's message (inputs#I1 item 4): it is never written, and nothing is lost at `_exit`.** The runner's
  `gdk_x_io_error` (`gdkmain-x11.c:246`, read from Ubuntu's `libgtk-3-0 3.24.33-1ubuntu2.2`) logs at
  `G_LOG_LEVEL_DEBUG` and then calls `_exit(1)`. GLib's default writer drops debug messages unless
  `G_MESSAGES_DEBUG` names the domain, which nothing sets. The app's stdout and stderr both go to `logs/boot.log`
  (`scripts/agent-run.sh:96`), and no `boot.log` of the 16 boots holds a GDK or GLib line beside the
  accessibility-bus warning.
- **The limit "seven of eight under the instrument against four of twelve before it":** on the dev host the arm
  with nothing preloaded ended 8 of 8 and the arm with the witness 6 of 8, so the witness library does not raise
  the rate. The difference between a run's first boot and its later boots is not attributed.

### The series' undercount
- **Re-derived.** `boot-series.json` of run 1 reads `ended` 2, `other` 4; of run 2 `settled` 2, `ended` 2, `other`
  3. Each of the seven `other` boots has `cycle` `boot-failed`, five null settle members, and an `end` line in its
  own `exit-witness.jsonl`.
- **Mechanism.** `CYCLE_SCRIPT` runs `harness:settled` only when `boot` returned 0
  (`xtask/src/harness_series.rs:38-46`), so a boot that ends before ready has no `harness-settled.json`,
  `read_settle` returns `None` (`:144`), and `classify` counts it `Other` (`:261-267`).
- **What such a boot leaves on disk.** The boot verb's failure path prints how the app ended and runs its own
  `cleanup` (`scripts/agent-run.sh:139-161`). `cleanup` removes the pid file and nothing else of a data dir whose
  name does not match `*/agent-run-*` (`:229-231`, `:257-259`). So `run/andromeda-pulse.spawn` (the app pid),
  `run/andromeda-pulse.exit` (`exit 1`) and `logs/exit-witness.jsonl` remain in `series/boot-{ordinal}/`.
- **The readers exist and are reachable.** `harness_status::{read_pid, end_file, read_ended}` and
  `harness_witness::label(data_dir, pid, ended)` are `pub(crate)` (`harness_status.rs:204-217`,
  `harness_witness.rs:42`). `read_pid` takes any one-decimal-pid file, the spawn record included. The witness
  reader is the bounded one (64 lines of at most 4096 printable-ASCII bytes, three kinds, a `u32` pid, else
  `unreadable`); no second parser is needed.
- **A constraint on the repair.** When the app is still running at the readiness timeout, the boot verb's own
  `cleanup` ends it with `TERM` (then `KILL`), and the exit record then reads `signal 15 (TERM)` or
  `signal 9 (KILL)`. That end is the harness's, not the app's. The app re-raises the same signal and never turns
  it into an exit code (`rules/security.md`, the `app.exit` boundary), so an exit record of the form `exit N` on a
  boot that failed to reach ready is an end the app made itself.
- **The smoke's own boot has the same gap.** The workflow's smoke step runs `agent-run.sh boot || exit 1`, so a
  first boot that ends before ready writes no `harness-settled.json` and the series lists no ordinal 1 for it
  (`series_payload`, `:370-378`). Its spawn record, exit record and witness file are in the top data dir.
- **The labels fit the registered sets.** `ended` is the bounded exit record, `exit_witness` one of the six closed
  labels, `cycle` stays `boot-failed`. No member, label or kept file is added.

## Files inspected
- `xtask/src/harness_series.rs` (1-432 read, 433-755 indexed by test name) — the cycle script, `run_boot`,
  `classify`, `boot_entry`, `series_payload`, the 16 pins.
- `xtask/src/harness_ready.rs` (84-233) — `run_settled`, `decide_settled`, `wait_for_settle`, `read_exit_record`.
- `xtask/src/harness_witness.rs` (1-135) — `label`, `decide`, the bounded reader.
- `xtask/src/harness_status.rs` (signatures) — the `pub(crate)` helpers.
- `scripts/agent-run.sh` (20-269) — the boot verb's spawn line, its failure path, `cleanup`.
- `.github/workflows/ci.yml` (the `boot` job's steps, the `a11y` job's commands) — the smoke step, the series step;
  the `a11y` job runs `cargo xtask test:a11y` and has no caller of the boot verb (`grep -n agent-run` over its
  lines: 0).
- `pulse-app/tests/quality_gate_workflow.rs` (index of pins) — `:639` fixes the literal
  `run: cargo xtask harness:boot-series --count 7`.
- `pulse-app/src/main.rs` (278-279) — `main()` opens with `render_posture::apply_linux_default()`.
- `pulse-app/src/render_posture.rs` (signatures) — the one existing head-of-`main` Linux step.
- `pulse-app/Cargo.toml` — `libc` is a direct dependency; no X11 crate is.
- `Cargo.lock` — `tao 0.35.0`, `wry 0.55.0`; `x11-dl` and `x11` enter through tao, wry and `gdkx11`.
- The cargo registry's `tao-0.35.0`, `tao-0.35.3`, `tao-0.37.1` (`device.rs`, `event_loop.rs`, `x11/xdisplay.rs`,
  `platform/unix.rs`).
- Ubuntu's `libx11-6`, `libx11-xcb1` and `libgtk-3-0` jammy packages (symbols and disassembly).
- The two runs' kept files (16 witness files, 16 `boot.log`, 2 `boot-series.json`, the `build.log` of each smoke)
  and both `boot` job logs.
- The prior chunk's `report.md` (100-234), `evidence/local-legs.md`, `plan.md` (the three local legs' commands).

## Graph impact (from the code-graph query; `rust` plane, `db_state` fresh, 72 rows)
- **`classify` / `boot_entry` / `run_boot` / `read_settle` / `cycle_label` / `parse_cycle_marker`** — every caller
  is inside `xtask/src/harness_series.rs` (`run`, `run_boot`, `run_cycle`, `class_of`, `series_payload`, and seven
  of its tests). The repair has no caller outside the module.
- **`read_pid` / `end_file` / `read_ended`** — called from `harness_status::run`, `harness_ready::{run_ready,
  run_settled, wait_for_settle, read_exit_record}` and their tests. The series would be a new caller; their
  signatures do not change.
- **`harness_witness::label`** — one production caller, `harness_ready::run_settled` (`:112`). The series would be
  the second.
- **`harness_series::run`** — one caller, `xtask/src/main.rs::main`.

## Patterns detected
- **Pure decision, one pin per arm** (`xtask/src/harness_series.rs:261-298`, `harness_ready.rs:174-187`,
  `harness_witness.rs:60-86`): the verdict cores take plain values and are tested without a process.
- **A harness state file read through one bounded grammar** (`harness_status.rs:210`): the exit record is one line
  of at most 48 printable ASCII bytes, else no record.
- **A head-of-`main` Linux step taken before any thread exists and reported after the subscriber is up**
  (`pulse-app/src/main.rs:279`, `render_posture.rs:50-65`): the decision is carried as a value.
- **Local legs on moved ports** (the prior plan's three legs): `env -u WAYLAND_DISPLAY`, a fresh data dir under
  `target/boot-smoke/`, `ANDROMEDA_PULSE_OTLP_GRPC_PORT=14317`, `ANDROMEDA_PULSE_OTLP_HTTP_PORT=14318`,
  `cargo xtask harness:boot-series --count N`; the verdict's printed tokens are `"verdict": "all-settled"` and
  `"verdict": "self-ended"` (read from the recorded outputs of this phase's five local series and from
  `series_payload`).

## Conventions to follow
- **A series verdict change keeps the registered member sets** (`harness_series.rs:563-600`, the set-equality pin).
- **A workflow edit moves its pin in the same change** (`pulse-app/tests/quality_gate_workflow.rs:588-657`).
- **A pulse-app probe lives under `pulse-app/tests/`** (the crate sets `[lib] test = false`).
- **A local leg that needs the runner's Xlib fetches the package by URL and refuses it on a checksum mismatch**:
  `libx11-6_1.7.5-1ubuntu0.3_amd64.deb` sha256
  `d382a3064ecd7576f24661a6d234adc737b854a9a0ec1d9f42cb4d007e77d547`, `libx11-xcb1_1.7.5-1ubuntu0.3_amd64.deb`
  sha256 `02c7db5d9f32e8c7c766d75ce9e0eb92f88d720c85da0fe506849b4a64f36806`, unpacked under `target/` and named
  through `LD_LIBRARY_PATH` on the series command only.

## New files to create
- `pulse-app/src/xlib_threads.rs` — the one call that initialises Xlib's thread support, inert where the library is not found
- `pulse-app/tests/unit_xlib_threads.rs` — the pins on that call's arms and on its place in `main`

## Files to modify
- `xtask/src/harness_series.rs` — a boot that ended by itself before ready is counted with its exit record and witness label; the smoke's own boot likewise; pins
- `pulse-app/src/main.rs` — the call as the second statement of `main`, after the render-posture step
- `pulse-app/src/lib.rs` — the module declaration

## Open questions
- Where does the close land: in the app's start (`XInitThreads()` first), in the workflow (a runner image whose Xlib
  initialises its threads at load), or in the harness (a preload inside the app's process)? → blocks: plan-decision.
  **Answered at P4 by the operator (inputs#I3): the app's start**; the lists above were narrowed to that branch at
  P4. **Placed at the P5 review (inputs#I4):** the render-posture step stays the first statement of `main` and the
  call is the second, so `render_posture.rs` is not in the lists and its comments stand. Read whole at P5
  (`render_posture.rs:1-80`): `apply_linux_default` is `cfg!`, `std::env::var_os` and `std::env::set_var`; it
  reaches no Xlib and starts no thread, and its record is emitted later by `emit_posture` (`main.rs:302`). No test
  pins the order of `main`'s opening statements today (`grep -rn 'apply_linux_default\|first statement'
  pulse-app/tests`: the two hits are the render-posture unit test's import and call).
- On how many consecutive equal-source runs is the close read on the runner? → blocks: plan-decision. **Leaned at
  P4:** three, per the operator's ruling recorded in P-129's note of 2026-10-09 ("three consecutive green runs count
  only once a cause is named from a run and closed"), with the two red runs as the without arm (inputs#I3).
- **External inputs, added at P4 and P5:** `inputs#I3` — the operator's P4 ruling: the close lands in the app's
  start; inert where the library cannot be found, both arms pinned; the module leaves with the window; the close is
  read on the runner against the two red runs; the swapped-library reading goes into evidence with how the swap
  was made and undone. `inputs#I4` — the operator's P5 review: the render-posture step stays the first statement
  of `main`, the call is the second, before any thread and before Xlib; the "first thing" of I3 is withdrawn.
