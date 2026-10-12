# The swapped-library reading — 2026-10-10-boot-smoke-s-self-end-closed

The dev host's own Xlib (1.8.13) initialises its thread support when it loads, so the app's missing call cannot
show there. The reading below swaps in the runner's Xlib build for one command at a time. Two parts: what the phase
measured at the chunk base (`95d17008`, before any edit), and what /implement's gate block recorded on the built
change. Sources: `.andromeda/runs/2026-10-10T04-02-31Z-phase/p3-measurements.md` for the first part; the recorded
outputs of gate entries 9 to 14 of the run `.andromeda/runs/2026-10-10T04-51-45Z-implement/` for the second.

Every local run moved the receivers to 14317 and 14318, unset `WAYLAND_DISPLAY`, and wrote a fresh data dir under
`target/boot-smoke/` (ignored). Ports 4317 and 4318 were never bound.

## How the swap is made

- Two Ubuntu jammy packages are fetched by URL from `security.ubuntu.com`, pool `main/libx/libx11/`, and refused on
  a checksum mismatch:

  | file | sha256 |
  |---|---|
  | `libx11-6_1.7.5-1ubuntu0.3_amd64.deb` | `d382a3064ecd7576f24661a6d234adc737b854a9a0ec1d9f42cb4d007e77d547` |
  | `libx11-xcb1_1.7.5-1ubuntu0.3_amd64.deb` | `02c7db5d9f32e8c7c766d75ce9e0eb92f88d720c85da0fe506849b4a64f36806` |

- They are unpacked with `bsdtar` and their two libraries copied to one directory under a name the loader looks
  for: `libX11.so.6.4.0` as `libX11.so.6`, `libX11-xcb.so.1.0.0` as `libX11-xcb.so.1`.

  | library | sha256 |
  |---|---|
  | `libX11.so.6` | `34b7869a1ba702462a1aa862fc0afe5b2608a7d5a09552c07796802fcf0c684b` |
  | `libX11-xcb.so.1` | `2526f9d1ca41ce2df4750d2d083bb9d335aa4430955faf66435ec038ea88e473` |

- That directory is named through `LD_LIBRARY_PATH` on the one command that boots the app. Nothing is installed, no
  system path is written, and no file of the tree changes.
- At P3 the directory was in the session scratch directory, outside the project. In the gate block it is
  `target/jammy-x11/lib/` (ignored), written again by the fetch entry on every run.

## At the chunk base (phase P3 and P5)

One release binary built from `95d17008`, unchanged across the arms, under
`cargo xtask harness:boot-series --count 8`.

| arm | Xlib | what differs | settled | self-ended |
|---|---|---|---|---|
| A | jammy 1.7.5 | the exit witness loaded | 2 | 6 |
| A' | jammy 1.7.5 | nothing preloaded | 0 | 8 |
| D | jammy 1.7.5 | `WEBKIT_DISABLE_COMPOSITING_MODE=1` | 3 | 5 |
| C | jammy 1.7.5 | `XInitThreads()` called first in the app process, by a scratch preload never in the tree | 8 | 0 |
| B | host 1.8.13 | the exit witness loaded | 8 | 0 |

- Without the initialisation on 1.7.5: 19 of 24 boots self-ended (arms A, A', D). With it: 0 of 8 (arm C). On the
  host's 1.8.13: 0 of 8 (arm B).
- Arm A's six `end` lines are equal and are the runner's call chain: `_exit`, code 1, `errno` 11, main thread,
  `libX11.so.6` `_XIOError` +0x41393, `_XReply` +0x4660f, `XGetWindowProperty` +0x255a8, then
  `gdk_x11_screen_supports_net_wm_hint`, a signal emission, GLib's main loop and `gtk_main_iteration_do`. 14 lines in
  8 files, 0 beyond the shape.
- Every local self-end came after `boot: ready`; none before ready. The before-ready class was read on the runner
  only.
- **The debugger capture** (one ending, jammy Xlib, `gdb -batch`, `break _XIOError`): the stop is on the main
  thread, `_XIOError` from `_XReply` from `XGetWindowProperty` from `gdk_x11_screen_supports_net_wm_hint`.
  `xcb_connection_has_error` on that display's connection reads 0 and `errno` reads 11, so the connection is healthy
  and 11 is a stale value. One other thread is inside Xlib: `XNextEvent`, called from the app binary at the offset
  of `tao`'s device-event thread. 87 threads in all; those two are the only ones holding an Xlib, xcb, GDK or GTK
  frame in their top 14.
- **The close leg's own baseline** (phase P5, the gate block's close-leg command on the untouched tree, 46.8 s):
  exit 1, verdict `self-ended`, settled 1, ended 7, each ended boot `exit 1` with `exit_witness` `exit-call`.
- With that baseline the base reads 26 of 32 boots self-ended across four series under the jammy Xlib.
- **Undone at P3:** the variable lived on one command each. After the last arm `ldd target/release/pulse-app`
  resolved both libraries from `/usr/lib`, no `pulse-app`, `Xvfb` or `gdb` process remained, and ports 14317, 14318,
  4317 and 4318 held no listener.

## On the built change (/implement's gate block, 2026-10-10)

The release binary was built by entry 8 from the working tree (HEAD `95d17008` plus this chunk's five files; the
pre-push check's tree id for it is `e7dfce60ca67`), and built again by each leg's boot verb. Its sha256 after the
block opens `a82b5b4c36903baa`.

| entry | what it is | recorded |
|---|---|---|
| 9 | the fetch | exit 0 (0.33 s). `libx11-6.deb: OK`, `libx11-xcb1.deb: OK`, then the library's checksum `34b7869a…c684b` |
| 10 | `ldd` under the variable, the swap's known positive | exit 0, last line `2`: `libX11.so.6` and `libX11-xcb.so.1` resolve from `target/jammy-x11/lib/` |
| 11 | the close leg: eight boots under the jammy Xlib, the witness loaded | exit 0 (84.1 s). `"verdict": "all-settled"`, `"boots": 8`, `"settled": 8`, `"ended": 0`, `"other": 0` |
| 12 | the repair leg: a boot with no display, then a series of one in its data dir | exit 0 (21.5 s). See below |
| 13 | the host leg: two boots on the host's own Xlib | exit 0 (20.9 s). `"verdict": "all-settled"`, `"settled": 2`; each boot's five kept files listed, nothing of `run/` copied |
| 14 | `ldd` with no variable set, the swap read undone | exit 1, last line `0`: nothing resolves from the fetched directory |

- **The close leg, per boot.** Ordinals 2 to 9 each read `cycle` `complete`, `verdict` `settled`,
  `windows_settled` 4, `ended` null, `app_exit_record` `absent`, `exit_witness` `loaded`. Each cycle printed
  `series-cycle boot=0 settled=0 status=0 cleanup=0` and `cleanup: clean`. Data dir
  `target/boot-smoke/20261010T050056Z-series-jammy-x11`.
- **Read against the base.** Same command, same Xlib build, same host: 7 of 8 self-ended at the base (P5), 0 of 8
  with the call as the second statement of `main`. One series of eight on the change; the runner's reading is the
  operator pass's three attempts, not this.
- **The repair leg.** The no-display boot printed `boot: failed to reach ready state within 10s`,
  `app ended: exit 101`, `cleanup: clean`, then `boot-exit 1`. Its data dir kept `run/andromeda-pulse.spawn` and
  `run/andromeda-pulse.exit` (`exit 101`); the pid file was removed by the verb's cleanup. The series of one then
  listed it first:

  ```json
  {
    "app_exit_record": null,
    "cycle": "smoke",
    "ended": "exit 101",
    "exit_witness": "runtime-exit",
    "ordinal": 1,
    "verdict": null,
    "windows_settled": null
  }
  ```

  with `"boots": 1`, `"settled": 1`, `"ended": 0`, `"other": 0` and `"verdict": "all-settled"`: ordinal 1 is out of
  the counts, and the series' own boot (ordinal 2) settled. At the base the same command listed ordinal 2 alone.
  Data dir `target/boot-smoke/20261010T050220Z-no-display`. That boot's log holds a panic record by design (the
  app cannot open a display); no gate grades it.
- **The `ldd` lines themselves**, read once more after the block. Under the variable: `libX11.so.6 =>
  ./target/jammy-x11/lib/libX11.so.6` and `libX11-xcb.so.1 => ./target/jammy-x11/lib/libX11-xcb.so.1`. With no
  variable: both from `/usr/lib`.

## Witness files read in this chunk's local legs

Every `exit-witness.jsonl` under the three data dirs of this block was read whole against the line shapes (`loaded`:
kind, pid, comm; `end`: kind, pid, tid, comm, call, code, errno, frames of m, s, o; `runtime-exit`: kind, pid):

- 23 files (11 series boots, each as the boot wrote it and as the series kept it, and the no-display boot's one),
  24 lines, **0 lines beyond the shape**.
- 23 `loaded` lines and 1 `runtime-exit` line (the no-display boot: `loaded`, then `runtime-exit`, both for the pid
  its spawn record names). No `end` line in any file.

The files the phase read at P3 are counted in its own record: 16 files and 28 lines from the two red runs, 8 files
and 14 lines from arm A, 0 beyond the shape in each.

## How the swap is undone, and what was left

- The variable stands on the close leg's command and on the `ldd` entry before it, nowhere else. Entry 14 reads it
  gone.
- Measured after the block: no `pulse-app` or `Xvfb` process (`ps -eo pid,comm`: 0 rows); no listener on 14317,
  14318, 4317 or 4318 (`ss -ltnH`: 0 rows); `git status --short` lists this chunk's five source files and the
  chunk's and the runs' own records only.
- Left on the host, all under `target/` (ignored): `target/jammy-x11/` (the two packages, their unpacked trees and
  the two libraries), `target/exit-witness/exit-witness.so`, and the three data dirs named above.

## Limits

- One series of eight on the built change under the swapped library; the base read four. The change's reading on
  the runner is not in this file.
- The close leg's boots all reached ready, so none exercised the before-ready read; the repair leg drives that read
  on a real process for the smoke's own boot, and the fixture pins stand for a series boot (plan, Test Commands).
- That the app process itself mapped the fetched library during the close leg is read from `ldd` under the same
  variable and from the base's result under the same command, not from the running process's own map.
- The step inside Xlib by which the reply is lost was not traced.
