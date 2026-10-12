# The live legs on this host (plan steps 8 to 10), 2026-10-09

Four boots of the release app under `xvfb-run`, each on a fresh data dir under `target/boot-smoke/`, each with the
receivers moved to 14317 and 14318 (4317 and 4318 were never bound) and `WAYLAND_DISPLAY` unset. Two were fired by
the gate tool as plan entries 8 and 9; two were driven by hand for what no entry measures. Host load (1 min) was
6.2 before the gate run, 8.9 before the hand legs and 6.6 after them: lighter than research's 22 to 33.

Paths are written from the repository root. Raw outputs are not kept; every figure was read from the leg's own
data dir or the gate tool's log.

## The gate tool's lines (run of 19:15Z, whole block, summary `green 18 · red 0 · not-run 6`)

```
  8 smoke       green · exit 0 · 11.49s · 684 B → 8.log · d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-green" && … (479 chars)
  9 smoke       green · exit 1 · 4.31s · 572 B → 9.log · d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-display-lo… (589 chars)
```

## Entry 8, the green leg — `target/boot-smoke/20261009T191536Z-green`

Printed, in this order: `boot: ready (PID=1880635, …)`, the settle verdict, the status verdict, `cleanup: clean`.

The settle verdict as printed, and byte-equal in `logs/harness-settled.json` (173 B):

```json
{
  "app_exit_record": "absent",
  "display": "reachable",
  "ended": null,
  "pid": 1880635,
  "session_bus": "reachable",
  "verdict": "settled",
  "windows_settled": 4
}
```

The status verdict read `"verdict": "running-healthy"`, pid 1880635, `"ended": null`.

| Read from the leg's log family (139 records) | Instant (UTC) | After the first record |
|---|---|---|
| first record, `app.boot.tracing.init` | 19:15:41.166 | 0 |
| `app.boot.otlp.grpc.bind`, `app.boot.otlp.http.bind` (both INFO) | 19:15:41.378 | 0.212 s |
| `app.boot.window.navigation` × 4: `compact-widget`, `main`, `findings`, `report`, each `navigated: true` | 19:15:46.377 | 5.211 s |
| `app.exit` (WARN, the smoke's own `cleanup`; exit record `signal 15 (TERM)`) | 19:15:46.680 | 5.514 s |

- Levels: 130 INFO, 5 DEBUG, 4 WARN, **0 ERROR**. `app.panic.fatal`: **0**. This host has a keyring, so the log
  holds no `corpus.open.error`.
- `ui.webgpu.adapter`: 2 records, both `no_navigator_gpu`.
- `logs/xvfb.log`: **0 B, empty.**
- `logs/boot.log` (314 B): two MESA-EGL DRI3 warnings and the appindicator deprecation warning; nothing else.

## Entry 9, the control leg — `target/boot-smoke/20261009T191547Z-display-lost`

The leg stops this leg's own `Xvfb` by pid right after ready. The gate judged it by its `contains` atoms; its exit
was 1, as the plan says it would be. Printed: `boot: ready (PID=1881658, …)`, the settle verdict, `cleanup: clean`,
and one line from `xvfb-run` itself saying its display server was already gone (`kill: … No such process`).

The settle verdict as printed, and byte-equal in `logs/harness-settled.json` (170 B):

```json
{
  "app_exit_record": "absent",
  "display": "gone",
  "ended": "exit 1",
  "pid": 1881658,
  "session_bus": "reachable",
  "verdict": "ended",
  "windows_settled": 0
}
```

- The log family holds 29 records, the last at 19:15:51.696Z (`tonic::transport::server`), 0.248 s after the
  first. No `app.exit`, no `app.panic.fatal`, 0 ERROR. The wrapper's exit record reads `exit 1`.
- `logs/boot.log` (314 B): the same three warnings as the green leg. Nothing was added when the app ended.
- `logs/xvfb.log` (817 B, sha256 head `12dcabf0449b9ff8`): 13 lines, all from the X server's keymap compiler —
  "The XKEYBOARD keymap compiler (xkbcomp) reports:", ten `Warning:` lines about key symbols (`<FK23>`, `<FK24>`,
  four `Could not resolve keysym XF86…` lines), and "Errors from xkbcomp are not fatal to the X server".
- Before-reading (research's control on the untouched tree): `status` printed `"ended": "exit 1"` and nothing
  about the display. After: the job's kept file says the app ended `exit 1`, left no `app.exit`, and its display
  was gone.
- This is a control. It shows what a lost display looks like in what the job keeps on this host. It does not show
  that a runner's display goes away.

## Step 10 — the display server's output, read for telemetry

Four `logs/xvfb.log` files were read whole (the two above and the two hand legs below). Three are empty. One holds
the keymap compiler's warnings quoted above. **None holds anything of a watched service's telemetry**: no span,
metric or log content, no attribute, no service name, no path under a data dir. The stop of inputs#I3 did not fire.
The CI artifact's file is the operator pass's to read.

## Hand leg, timed — `target/boot-smoke/20261009T191736Z-timed`

Driven by hand because no entry records when `boot` returns.

| | Instant (UTC) | After the first record |
|---|---|---|
| first record | 19:17:40.743 | 0 |
| both `app.boot.otlp.*.bind` records | 19:17:40.942 | 0.199 s |
| `boot` returned, exit 0 | 19:17:41.039 | **0.296 s** |
| four navigation records | 19:17:45.942 | 5.199 s |

- `boot` reported ready 0.097 s after the receivers bound, inside the 10 s default window. On `ci#37964887106`
  the old form printed ready 12 ms after the first record, before the bind (the entry's second carry).
- `cargo xtask harness:ready` right after: `"verdict": "ready"`, both receivers `accepting`, exit 0.
- The same verb with the gRPC port variable pointed at 14319, where nothing listens: `"verdict": "not-ready"`,
  `"otlp_grpc": "refusing"`, `"otlp_http": "accepting"`, exit 1. The not-ready arm, read against a live app.
- `cargo xtask harness:settled --timeout-seconds 7`: `"verdict": "cannot-evaluate"`, exit 2 (a window that could
  never read `settled` is refused).
- `cargo xtask harness:settled`: `settled`, 4 windows, exit 0; `status` 0; `cleanup: clean`, 0.
- 141 records, 0 ERROR, 0 `app.panic.fatal`; `logs/xvfb.log` 0 B.

## Hand leg, never ready — `target/boot-smoke/20261009T191736Z-never-ready`

Driven by hand because the plan lists no entry for `boot`'s failure path. The gRPC port variable was set to 81, a
port the app refuses (`config.load.port_validation`, 1 record; `app.boot.otlp.grpc.bind` at ERROR), so the app
lived with its HTTP receiver alone and nothing listened on 81.

`boot` printed, then exited 1:

```
boot: failed to reach ready state within 10s
  Boot log: ./target/boot-smoke/20261009T191736Z-never-ready/logs/boot.log
  app still running (pid 1898696) but never reported healthy
  the receivers never both accepted: OTLP gRPC 127.0.0.1:81, OTLP HTTP 127.0.0.1:14318
cleanup: clean
```

- The app's log holds its four navigation records and ends on `app.exit` from `boot`'s own cleanup (exit record
  `signal 15 (TERM)`), so the app was alive for the whole window and `boot` still refused to call it ready.
- The line is printed only when the last readiness verdict held a `refusing` label; on a failed poll whose last
  verdict showed both receivers accepting (a stale log) it is not printed. That arm was not driven.

## What was left running

Read from the host's process list after each pair of legs (`ps -eo pid,ppid,etimes,comm`, filtered for
`pulse-app`, `Xvfb`, `xvfb-run`): no row, both times. 14317 and 14318 refused a connection both times; 81 refused
after the hand legs.
