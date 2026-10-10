# P3 measurements — 2026-10-10-boot-smoke-s-self-end-closed

Written by the phase orchestrator at P3, 2026-10-10 (04:05Z to 04:28Z). Every probe ran from the session scratch
directory, outside the tree; nothing in the tree was edited. Local data dirs are under `target/boot-smoke/` (ignored).
The receivers were moved to 14317 and 14318 in every local run; `WAYLAND_DISPLAY` was unset.

## 1. The two red runs, read from what each kept

| run | pushed tip | merge built | boots | settled | self-ended | of those, before ready |
|---|---|---|---|---|---|---|
| `ci#38019133294` attempt 1 | `925be35f` | `ab6a1ae6ef0d` | 8 | 1 (ordinal 6) | 7 | 4 (ordinals 3, 4, 5, 7) |
| `ci#38022477393` attempt 1 | `95d17008` | `71ce5340f2ad` | 8 | 3 (ordinals 1, 2, 6) | 5 | 3 (ordinals 3, 4, 5) |

- Sixteen witness files, 28 lines, read whole against the shape (`loaded`: kind, pid, comm; `end`: kind, pid, tid,
  comm, call, code, errno, frames of m, s, o): 0 lines beyond it. The twelve `end` lines are equal in every member
  but `pid` and `tid`, across both runs.
- `boot-series.json`: run 1 `ended` 2, `other` 4; run 2 `settled` 2, `ended` 2, `other` 3. Every `other` is a boot
  with `cycle` `boot-failed`, all five settle members null, and an `end` line in its own witness file.
- All 16 `xvfb.log` are 0 B. No `boot.log` of either run holds a GDK or GLib message beside the one accessibility-bus
  warning (`grep -l -i 'Fatal IO error\|Gdk-Message'` over the 16 files: 0).
- Artifacts: `logs-boot-Linux` `11657059728` and `11660085100` (expires 2026-10-24T04:15:04Z), downloaded to
  `target/boot-smoke/ci-38019133294-attempt-1/` and `target/boot-smoke/ci-38022477393-attempt-1/`. Job logs fetched
  with `gh api --allow-escape-sequences …/actions/jobs/{114115991438,114126248205}/logs` to files (353629 B, 355656 B).
- Run 2's other five jobs succeeded (`a11y`, `lint / test`, `coverage gate`, `supply-chain`, `mcp-server tests`).

## 2. The runner's library builds, read from Ubuntu's own packages

The job log names the image (`ubuntu-22.04`, version `20261004.315.1`) and the installed `libgtk-3-dev
3.24.33-1ubuntu2.2`, `libwebkit2gtk-4.1-0 2.50.4-0ubuntu0.22.04.1`, `xvfb 2:21.1.4-2ubuntu1.7~22.04.16`.

| package (jammy, amd64) | sha256 |
|---|---|
| `libx11-6_1.7.5-1ubuntu0.3_amd64.deb` (security.ubuntu.com `pool/main/libx/libx11/`) | `d382a3064ecd7576f24661a6d234adc737b854a9a0ec1d9f42cb4d007e77d547` |
| `libx11-xcb1_1.7.5-1ubuntu0.3_amd64.deb` (same pool) | `02c7db5d9f32e8c7c766d75ce9e0eb92f88d720c85da0fe506849b4a64f36806` |
| `libgtk-3-0_3.24.33-1ubuntu2.2_amd64.deb` (archive.ubuntu.com `pool/main/g/gtk+3.0/`) | `b22a6e211b0e79aeaf467773a6689abf96c3a524446528c8b54c80b5e165fe4a` |

- `libX11.so.6.4.0` of that package (sha256 `34b7869a…c684b`): `nm -D` gives `_XIOError` 0x41330, `_XReply` 0x46220,
  `XGetWindowProperty` 0x254a0. The witness frames are exact return addresses in it: 0x4660f follows
  `call _XIOError` at 0x4660a inside `_XReply`, and the next instructions are `xor %eax,%eax` and a jump (the
  function's "no reply and no X error" exit); 0x41393 follows the handler call inside `_XIOError`; 0x255a8 follows
  `call _XReply` inside `XGetWindowProperty`. The `1.7.5-1` build has other addresses (`_XIOError` 0x41230), so the
  runner runs `1.7.5-1ubuntu0.3`.
- `libgdk-3.so.0.2404.29` of the GTK package: the function ending at +0x76ccc is `gdk_x_io_error`
  (`gdkmain-x11.c:246`, strings read from the binary). It calls `g_log_structured_standard` with level `0x80`
  (`G_LOG_LEVEL_DEBUG`) and then `_exit(1)`. One branch only.
- `_Xglobal_lock` at load, read by a ten-line C program: the dev host's libX11 1.8.13 prints
  `thread-support-initialised-at-load`; the jammy 1.7.5 build prints `not-initialised-at-load`.

## 3. The reproduction on the dev host

The same release binary (built from `95d17008`, sha256 `4c9097178f97c171…`, unchanged across the arms), under
`cargo xtask harness:boot-series --count 8`, with `LD_LIBRARY_PATH` naming a scratch directory that holds the two
jammy libraries above (`ldd` confirms the app resolves `libX11.so.6` and `libX11-xcb.so.1` there and everything
else from the host).

| arm | Xlib | what differs | settled | self-ended | data dir |
|---|---|---|---|---|---|
| A | jammy 1.7.5 | the exit witness loaded | 2 | 6 | `20261010T041733Z-p3-jammy-x11` |
| A' | jammy 1.7.5 | nothing preloaded | 0 | 8 | `20261010T042216Z-p3-jammy-x11-bare` |
| D | jammy 1.7.5 | `WEBKIT_DISABLE_COMPOSITING_MODE=1` | 3 | 5 | `20261010T042649Z-p3-jammy-x11-nocompositing` |
| C | jammy 1.7.5 | `XInitThreads()` called first in the app process (the scratch preload below) | 8 | 0 | `20261010T042004Z-p3-jammy-x11-xinit` |
| B | host 1.8.13 | the exit witness loaded | 8 | 0 | `20261010T042258Z-p3-host-x11` |

- Arm A's six `end` lines are equal and read `_exit`, code 1, `errno` 11, main thread, `libX11.so.6` `_XIOError`
  +0x41393, `_XReply` +0x4660f, `XGetWindowProperty` +0x255a8, then `gdk_x11_screen_supports_net_wm_hint`, a signal
  emission, GLib's main loop and `gtk_main_iteration_do`: the runner's chain, with the host's GDK offsets. 14 lines
  in 8 files, 0 beyond the shape.
- Without Xlib thread initialisation on 1.7.5: 19 of 24 boots self-ended. With it: 0 of 8. On 1.8.13: 0 of 8.
- Every local self-end came after `boot: ready` (`cycle` `status-not-healthy`); none before ready. The before-ready
  class was read on the runner only.

### How the swap was made and undone

- **Made.** The two packages were downloaded with `curl` into the session scratch directory (outside the project),
  unpacked there with `bsdtar`, and `libX11.so.6.4.0` and `libX11-xcb.so.1.0.0` copied to one scratch directory as
  `libX11.so.6` and `libX11-xcb.so.1`. That directory was named through `LD_LIBRARY_PATH` on the series command of
  arms A, A', C and D alone (and through gdb's `set environment` for the capture of section 4). Nothing was
  installed, no system path was written, and no file of the tree was changed.
- **Undone.** The variable lived on one command each, so nothing on the system holds it. Read after the last arm:
  `ldd target/release/pulse-app` resolves `libX11.so.6` and `libX11-xcb.so.1` to `/usr/lib`; no `pulse-app`, `Xvfb`
  or `gdb` process remained (`ps -eo pid,comm`: 0); ports 14317, 14318, 4317 and 4318 held no listener
  (`ss -ltnH`: 0); `git status --short` lists the phase's own files only. The scratch directory is the session's and
  goes with it.
- One further boot ran at P4 with no display at all (`DISPLAY=:987`, data dir
  `20261010T043526Z-p4-no-display`): the app ended before ready with exit record `exit 101`, its witness file holds
  `loaded` and `runtime-exit`, the pid file was removed by the verb's cleanup and the spawn and exit records
  remained; a series of one boot run in that data dir afterwards listed no ordinal 1.

### The scratch preload of arm C (never in the tree)

```c
#define _GNU_SOURCE
#include <X11/Xlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((constructor)) static void probe_init(void) {
    char comm[32] = {0};
    FILE *f = fopen("/proc/self/comm", "r");
    if (!f) return;
    if (!fgets(comm, sizeof comm, f)) { fclose(f); return; }
    fclose(f);
    comm[strcspn(comm, "\n")] = 0;
    if (strcmp(comm, "pulse-app") != 0) return;
    XInitThreads();
    unsetenv("LD_PRELOAD");
}
```

Built with `cc -shared -fPIC -O2 -o xinit.so xinit.c -lX11` and named through `LD_PRELOAD` on the series command;
it acts in the process named `pulse-app` alone and drops the variable there, so no child of the app loads it.
Arm C ran without the exit witness: the boot verb sets `LD_PRELOAD` to the witness on the spawn line, which would
replace it.

## 4. One ending caught under a debugger (dev host, jammy Xlib, first attempt)

`gdb -batch` on the release binary, `break _XIOError`, under `xvfb-run`, a fresh data dir:

- The stop is on the main thread: `_XIOError` from `_XReply` from `XGetWindowProperty` from
  `gdk_x11_screen_supports_net_wm_hint`.
- `xcb_connection_has_error(XGetXCBConnection(display))` at the stop: **0**. `errno`: 11.
- One other thread is inside Xlib: thread 72, `XNextEvent` → `_XReadEvents` → `xcb_wait_for_event` → `poll`, called
  from the app binary at offset 0x305a0aa (the same offset the dev-host control of the prior chunk recorded for its
  second-thread line). Among the linked crates only `tao-0.35.0/src/platform_impl/linux/device.rs` calls
  `XNextEvent` (`grep -rl XNextEvent` over tao 0.35.0, wry 0.55.0, tray-icon 0.23.1, muda 0.19.1: 1 file).
- 87 threads in all; threads 1 and 72 are the only two holding a libX11, libxcb, libgdk or libgtk frame in their
  top 14 (one capture, one ending).

## 5. The series' first boot relinks the app

- `ci#38019133294`: the smoke step ran 03:08:56Z to `boot: ready` at 03:12:54Z (its `build.log`: `Finished release
  … in 3m 45s`); the series' boot 2 ran 03:12:57Z to ready at 03:16:24Z (3 min 27 s); each later boot's cycle took at most 12 s.
- `ci#38022477393`: smoke 04:06:21Z to 04:10:34Z; boot 2 04:10:41Z to 04:14:17Z (3 min 36 s).
- The five `pulse-app` frames of every `end` line have equal offsets in boot 1's file and in the series' files, in
  both runs.
- On the dev host a shell-launched `cargo build --bin pulse-app --release` under the boot verb's exported variables,
  right after four series, compiled nothing (0.33 s, binary sha256 unchanged). Why the runner relinks is not
  measured: no series boot keeps its `build.log`.
