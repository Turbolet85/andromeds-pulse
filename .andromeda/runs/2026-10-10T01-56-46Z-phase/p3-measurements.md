# P3 measurements — 2026-10-10-boot-smoke-s-self-end-named-from-a-run

Read 2026-10-10, 02:06Z to 02:16Z. Local boots ran on the dev host under `xvfb-run -a -e {logs}/xvfb.log
--server-args="-screen 0 1280x1024x24"`, one fresh data dir per boot, the receivers on 14317 and 14318,
`WAYLAND_DISPLAY` unset. The release binary was rebuilt by the first boot (87 s) at `8936f976`. Host load is the
`load1` read before each boot.

## The recorded CI runs (eight boot artifacts, `logs-boot-Linux`, read record by record)

Seconds after the log's first record.

| run | tip | records | receivers bind | webview's first record | `ui-bridge.ready` | last record | last target | `app.exit` | settle verdict |
|---|---|---|---|---|---|---|---|---|---|
| ci#37924991598 | b3ac58a9 | 51 | 0.217 | 0.763 | 0.852 | 0.852 | ui.webgpu.adapter | none | (step predates it) |
| ci#37961031489 | f36a2ac3 | 48 | 0.348 | 0.913 | 0.920 | 0.926 | ui.webgpu.adapter | none | (step predates it) |
| ci#37979648967 | e2931127 | 116 | 0.143 | 0.440 | 0.445 | 5.494 | app.exit | sigterm | settled |
| ci#37986543962 | 8a89e368 | 110 | 0.678 | 1.390 | 1.426 | 6.180 | app.exit | sigterm | settled |
| ci#37990081874 | 8f655cae | 113 | 0.579 | 1.150 | 1.158 | 6.159 | app.exit | sigterm | settled |
| ci#38010977166 | cb8cc4dc | 53 | 0.189 | 0.829 | 0.835 | 0.907 | ui.webgpu.adapter | none | ended |
| ci#38014639286 | b7101914 | 118 | 0.220 | 0.565 | 0.603 | 5.624 | app.exit | sigterm | settled |
| ci#38014971549 | 8936f976 | 57 | 0.271 | 0.898 | 0.928 | 1.273 | triage.cue.tick | none | ended |

- The four red logs each hold both `ui.webgpu.adapter` records (`main` and `compact-widget`, `no_navigator_gpu`) and
  end 0 to 0.34 s after the second of them. None holds `app.exit`, `app.panic.fatal` or
  `app.boot.window.navigation`.
- `ci#38014971549` (this chunk's base, job `114103109878`, artifact `11656415762`, expires 2026-10-24):
  `harness-settled.json` reads `verdict ended`, `ended "exit 1"`, `app_exit_record absent`, `windows_settled 0`,
  `display reachable`, `session_bus reachable`; `xvfb.log` 0 B; `boot.log` one line, the accessibility-bus warning;
  the log's `git.commit.sha` reads `9df2c7b57a2e`.
- `ci#38014639286` (the wrap commit, job `114102094003`, artifact `11655810270`): `settled`, four windows, the app
  alive 5.624 s; `git.commit.sha` `4a79cbbbf22d`.
- A lead that did not hold: the webview's first record is later on the two reds first read (0.829 s, 0.898 s) than
  on two greens (0.440 s, 0.565 s), and two other greens read 1.150 s and 1.390 s. It separates nothing.
- Readings of the `boot` job under the settle step, in order: `e2931127` green, `8a89e368` green, `8f655cae` green,
  `cb8cc4dc` red, `b7101914` green, `8936f976` red. Two red in six.

## The probe (a research instrument, kept in the session scratchpad, not in the tree)

A C library of about 130 lines, built with `cc -shared -fPIC -O2`, loaded into the app process alone through
`LD_PRELOAD` on the app's spawn line. It interposes `_exit`, `_Exit`, `exit`, `quick_exit` and `abort`; at the call
it appends one JSON line to a file named by an environment variable: pid, thread id, process name, the call, its
code, `errno`, and up to 23 caller frames as module basename, exported symbol (empty for a static function) and
offset in the module.

### Green boots, alternating arms (the CI sequence: boot, settle, status, cleanup)

| arm | boots | settle verdict | boot verb wall, s | receivers bind, s (median) | webview's first record, s (median, range) | four windows settled, s (median) | lines the probe wrote | new lines in `boot.log` |
|---|---|---|---|---|---|---|---|---|
| plain | 6 | settled 6 of 6 | 1.026 to 1.035 (five; the first, 86.98, held the rebuild) | 0.173 | 0.911 (0.852 to 1.017) | 5.173 | n/a | 0 |
| probe | 6 | settled 6 of 6 | 1.021 to 1.086 | 0.174 | 0.869 (0.854 to 0.913) | 5.173 | 0 | 0 |

Load 2.2 to 5.9. Every boot ended on the smoke's SIGTERM (`signal 15 (TERM)`), `cleanup: clean`. A SIGTERM end is
no exit call, so the probe writes nothing on a green boot. No difference between the arms is visible at this n.

### Control: the virtual display stopped 2 s after ready (a known member of the silent `exit 1` class)

| arm | boots | settle verdict | exit record | `app.exit` | new line in `boot.log` | probe |
|---|---|---|---|---|---|---|
| probe | 2 | ended 2 of 2 | `exit 1` 2 of 2 | absent | none | 1 record on the main thread, 2 of 2; a second record on another thread, 1 of 2 |
| plain | 1 | ended | `exit 1` | absent | none | n/a |

The probe's main-thread record, boot 1, whole:

```json
{"kind":"end","pid":374465,"tid":374465,"comm":"pulse-app","call":"_exit","code":1,"errno":11,"frames":[{"m":"exit_witness.so","s":"_exit","o":"0x16a1"},{"m":"libgdk-3.so.0","s":"","o":"0x8ca2d"},{"m":"libX11.so.6","s":"_XIOError","o":"0x3fa4c"},{"m":"libX11.so.6","s":"_XEventsQueued","o":"0x44fbf"},{"m":"libX11.so.6","s":"XPending","o":"0x34208"},{"m":"libgdk-3.so.0","s":"","o":"0x812be"},{"m":"libglib-2.0.so.0","s":"","o":"0x637e2"},{"m":"libglib-2.0.so.0","s":"","o":"0x63cb3"},{"m":"libglib-2.0.so.0","s":"g_main_context_iteration","o":"0x64055"},{"m":"libgtk-3.so.0","s":"gtk_main_iteration_do","o":"0x1ead7f"},{"m":"pulse-app","s":"","o":"0x2a53c36"},{"m":"pulse-app","s":"","o":"0x2d308ad"},{"m":"pulse-app","s":"","o":"0x29654a3"},{"m":"pulse-app","s":"","o":"0x2965119"},{"m":"pulse-app","s":"","o":"0x3041414"},{"m":"pulse-app","s":"","o":"0x2db0c25"},{"m":"libc.so.6","s":"","o":"0x27781"},{"m":"libc.so.6","s":"__libc_start_main","o":"0x278b9"},{"m":"pulse-app","s":"","o":"0x18cafa5"}]}
```

The second record of the same boot (thread 374551): `_exit`, code 1, `errno` 11, called from `libgdk-3.so.0`
+0x8ca2d under `_XIOError` under `XNextEvent`, itself called from `pulse-app` +0x305a0aa.

- So on the control the probe names the call (`_exit(1)`), its caller (a static function of `libgdk-3.so.0`, entered
  from Xlib's `_XIOError`), the thread and `errno`, where the job's present instruments read only "exit 1, no record".
- The app holds a second thread that reads X events by itself (`XNextEvent` called from the app binary); it ends the
  process through the same handler.
- This is the dev host's library set (gtk3 3.24.52, libx11 1.8.13, glib2 2.88.3, webkit2gtk-4.1 2.52.6,
  `pacman -Q`). It shows what the instrument reads. It does not show what ends the app on the runner.

### Where the webview keeps state between boots

One probe boot with `XDG_DATA_HOME` and `XDG_CACHE_HOME` pointed at fresh directories: the app created
`com.andromeda.pulse/` under the data directory with `WebKitCache`, `CacheStorage`, `storage`, `mediakeys` and
`hsts-storage.sqlite`, and `gtk-3.0/compose` under the cache directory; it settled, 4 windows. Without the two
variables that state sits under the user's own data directory and outlives a boot, so a second boot in one job
starts with the first boot's webview state unless it is given its own.

## P4 control: the record's arms and the child-process boundary (read 2026-10-10T02:27Z, after the operator's P4 conditions)

The scratch probe was extended three ways: a `loaded` line written at load, a `runtime-exit` line written from an
exit handler it registers at load, and both of its variables removed from the process environment at load. Four
children of a few lines of C each, built with `cc -O0`, each ending with code 7, read under it:

| child ends by | lines written, in order | child's exit code |
|---|---|---|
| `exit(7)` called through the linker | `loaded`, `end` (call `exit`), `runtime-exit` | 7 |
| `_exit(7)` called directly | `loaded`, `end` (call `_exit`) | 7 |
| `main` returning 7 | `loaded`, `runtime-exit` | 7 |
| a direct `exit_group` system call | `loaded` | 7 |

- So each of the three ways a process ends with a code leaves its own line pattern, and the fourth (an end the
  library cannot see at all) leaves `loaded` alone: an end with no line is distinguishable from a library that was
  never there.
- Child boundary: a parent loaded with the probe forked and ran a child that prints whether it holds the two
  variables. The child printed `LD_PRELOAD=unset FILE=unset`, and the file held 1 `loaded` line, the parent's.
- `python3 -c "import sys; sys.exit(7)"` under the probe wrote no `end` line: the interpreter returns from `main`.
  It is not a control for the linker arm; a compiled child is.
- dev host glibc; the runner's C library was not read.

## Platform readings

- Runner image `ubuntu-22.04`, readme fetched 2026-10-10 (`actions/runner-images`, `images/ubuntu/Ubuntu2204-Readme.md`):
  image version `20261004.315.1`, kernel `6.8.0-1064-azure`. Not in the document: `strace`, `ltrace`, `gdb`, `lldb`,
  `bpftrace`, `perf`, `valgrind`. In it: `gcc 4:11.2.0-1ubuntu1`, `binutils 2.38-4ubuntu2.12`,
  `libunwind8 1.3.2-2build2.1`, `systemd-coredump 249.11-0ubuntu3.22`, `xvfb`, `dbus 1.12.20-2ubuntu4.1`.
- GTK source, branch `gtk-3-24`, `gdk/x11/gdkmain-x11.c`, fetched 2026-10-10: `gdk_x_io_error` logs through
  `g_debug` ("We g_debug() instead of g_warning(), because g_warning() could possibly be redirected to the log") and
  then calls `_exit (1)`; the file installs it with `XSetIOErrorHandler (gdk_x_io_error)`. A `g_debug` line is not
  printed unless `G_MESSAGES_DEBUG` names its domain, so this end is silent on stderr and the product's exit record
  cannot see `_exit`. The runner's GTK build was not read.
- Dev host tools: `gdb`, `eu-stack`, `gcore`, `cc`, `clang` present; `strace`, `ltrace`, `bpftrace`, `perf` absent
  (`command -v`).
