# Render-posture live legs — 2026-10-04-linux-launch-stays-up-on-nvidia-wayland

Plan entries 16-18 (`leg = 'operator'`), fired ONCE each by /implement on the operator-granted real-display slot
(overseer grant 2026-10-04: ports 4317/4318 re-measured free, no pulse-app running, the founder hands-off; the
PRESET leg ran inside the same slot under that grant). Each `run` was fired verbatim from a scratchpad copy of the
plan text, from the repository root, under `timeout` (180 s GREEN · 120 s PRESET); `REAL EXIT` was read from the
unpiped command.

**Host:** NVIDIA GA102 (RTX 3090) · nvidia-open 610.57.04 · Hyprland · webkit2gtk-4.1 2.52.6 · native Wayland
(`WAYLAND_DISPLAY=wayland-1`), as in research.md §Host measurement.
**Binary:** `target/debug/pulse-app`, mtime 2026-10-04T11:54:07Z, 310 160 176 bytes, rebuilt from the restored
source by the post-mutation gate re-run (entry 15); the binary carries the `app.boot.render.posture` target ×2 and
the lever name ×1 (byte count).

## Precondition (entry 16), before each leg
| before | ports free (`! ss … :4317|:4318`) | `WAYLAND_DISPLAY` | `pulse-app` running |
|---|---|---|---|
| GREEN | exit 0 | `wayland-1` | 0 |
| PRESET | exit 0 | `wayland-1` | 0 (post-GREEN census) |

## GREEN leg (entry 17) — lever UNSET in the launch env
Data dir `target/render-posture/green-20261004T115506Z` (fresh, created this run). Window 11:55:06Z → 11:56:08Z.
`REAL EXIT=0`. Printed lines:
```
ended=alive
gdk_error71=0
ports_after=0
posture=applied
errors=0
panics=0
```
Every `expect` atom holds (`exit 0` · `ended=alive` · `gdk_error71=0` · `posture=applied` · `errors=0` ·
`panics=0` · `ports_after=0`).
Second reads of the same log (907 records): the posture record is `INFO` with fields exactly `{lever:
"__NV_DISABLE_EXPLICIT_SYNC", posture: "applied"}`, no other field. `app.boot.window.navigation` carries
`navigated: true` ×4 (all four windows, matching the P3 surviving arms). `app.exit` ×1 (the TERM by pid inside the
run).

**RED→GREEN:** against research.md §Host measurement arms A1-A3 at base `ffb62f0` (default posture, 3/3
`ended=died` at ~2.0 s, exit 1, stderr `Error 71 (Protocol error)`), cited, not re-run. The in-process `set_var`
at the head of `main()` reaches NVIDIA's EGL the way the shell-exported variable did in arm C: the app stayed up
across the full 60 s window.

## PRESET leg (entry 18) — `__NV_DISABLE_EXPLICIT_SYNC=0` preset
Data dir `target/render-posture/preset-20261004T115622Z` (fresh, created this run). Window 11:56:22Z → 11:56:25Z.
`REAL EXIT=0`. Printed lines:
```
ended=died
gdk_error71=1
ports_after=0
posture=preset_honoured
panics=0
```
Every `expect` atom holds (`exit 0` · `ended=died` · `gdk_error71=1` · `posture=preset_honoured` · `panics=0` ·
`ports_after=0`).
Second reads (50 records, readable from a dying run as P3 measured): the posture record is `WARN` with fields
exactly `{lever: "__NV_DISABLE_EXPLICIT_SYNC", posture: "preset_honoured"}` — the preset value `0` is not on the
wire. stderr: `Gdk-Message: …: Error 71 (Protocol error) dispatching to Wayland display.` `app.exit` ×0 (the GDK
`_exit(1)` path, a stated `app.exit` blind spot). No window navigated before the death.

The preset was honoured, not overwritten, and the app died exactly as at the base, so the lever is the operative
variable. Nothing else in this chunk keeps the app up.

## Process census (after each leg, `ps -eo pid,comm,args` filtered for `pulse-app|WebKit`)
| after | pulse-app / WebKit processes | OTLP listeners |
|---|---|---|
| GREEN | none | 0 (`ports_after=0`) |
| PRESET | none | 0 (`ports_after=0`) |
