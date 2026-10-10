# The local legs and the gate block — /implement, 2026-10-10

Written by /implement from the gate tool's recorded outputs (run dir `.andromeda/runs/2026-10-10T02-37-40Z-implement/`,
trail `gate-2026-10-10-boot-smoke-s-self-end-named-from-a-run.json`). Entry numbers are the plan's `## Test Commands`
order. Data dirs are under `target/boot-smoke/`, which is ignored. Every leg ran with the receivers on 14317 and
14318 and `WAYLAND_DISPLAY` unset.

## The block, three calls

| call | read at | summary line |
|---|---|---|
| the whole block | 02:50Z to 02:52Z | `entries 37 · green 20 · red 2 (3,17) · recorded 0 · timeout 0 · not-run 15` |
| `--only 3,17` | 02:53Z | `entries 37 · green 2 · red 0 · recorded 0 · timeout 0 · not-run 35` |
| the whole block again | 02:53Z to 02:57Z | `entries 37 · green 22 · red 0 · recorded 0 · timeout 0 · not-run 15` |

- **The two reds of the first call had one cause, and it was not the chunk's edits.** Step 1 of the plan runs the
  workspace suite once for the red reading, before the block. That run rewrote the tracked
  `pulse-app/ui/src/bindings/index.ts` to the default-features shape. So entry 3 (the scope guard) printed that one
  path, and entry 17 (`capability-drift`) read `drifted (3 missing, 0 extra)`, missing `mcp.start`, `mcp.status`,
  `mcp.stop`. The block's own regen (entry 20) put the file back and entry 21 read it equal to the chunk base in
  the same call. Nothing was edited between the calls.
- The 15 entries not run are the operator pass's (`leg = 'operator'`), entries 23 to 37.
- Host load: `load1` 3.5 and `load5` 10.9 at the start of the third call (another project's builds share this
  host). The first call started at `load1` about 2.

## Entry 10 — the green leg (data dir `…-witness-green`)

| reading | first call | third call |
|---|---|---|
| wall | 11.85 s | 98.05 s |
| `boot: ready` | yes | yes |
| settle verdict | `settled`, 4 windows, `exit_witness` `loaded` | `settled`, 4 windows, `exit_witness` `loaded` |
| status | `running-healthy` | `running-healthy` |
| cleanup | `cleanup: clean` | `cleanup: clean` |
| `loaded` lines in `logs/exit-witness.jsonl` | 1 | 1 |

- The witness file of the third call, whole: `{"kind":"loaded","pid":500893,"comm":"pulse-app"}`. One line, the
  app's own. The app settled with its webview's child processes running and none of them wrote a line.
- The third call's 98 s holds a rebuild: the boot verb's own pre-build compiled `pulse-app` again
  (`Finished release … in 1m 27s` in that data dir's `logs/build.log`), after the pre-push check of the first call
  had run on the shared `target/`. Why that check leaves `pulse-app` to rebuild is stated in the handoff and not
  measured here.

## Entry 11 — the control (data dir `…-witness-display-lost`)

The leg's display server is stopped by pid right after `boot: ready`. Both calls read the same:

- settle verdict `ended`, `ended` `exit 1`, `app_exit_record` `absent`, `windows_settled` 0, `display` `gone`,
  `session_bus` `reachable`, **`exit_witness` `exit-call`**; status `not-running`; `cleanup: clean`.
- The witness file of the third call, whole (two lines):

```json
{"kind":"loaded","pid":501438,"comm":"pulse-app"}
{"kind":"end","pid":501438,"tid":501523,"comm":"pulse-app","call":"_exit","code":1,"errno":11,"frames":[{"m":"exit-witness.so","s":"_exit","o":"0x1851"},{"m":"libgdk-3.so.0","s":"","o":"0x8ca2d"},{"m":"libX11.so.6","s":"_XIOError","o":"0x3fa4c"},{"m":"libX11.so.6","s":"XNextEvent","o":"0x2d379"},{"m":"pulse-app","s":"","o":"0x305a0aa"},{"m":"pulse-app","s":"","o":"0x3062fb4"},{"m":"pulse-app","s":"","o":"0x304c334"},{"m":"libc.so.6","s":"","o":"0x980a2"},{"m":"libc.so.6","s":"","o":"0x12080c"}]}
```

- So the job's kept record names the call (`_exit`, code 1, `errno` 11) and its caller (a static function of
  `libgdk-3.so.0` at +0x8ca2d, entered from Xlib's `_XIOError`).
- **One difference from the P3 reading, kept as a reading.** In both calls the one `end` line was written by a
  second thread (`tid` differs from `pid`), reaching the handler through `XNextEvent` called from the app binary.
  At P3, with the display stopped 2 s after ready, the main thread wrote the record through `XPending` in 2 of 2
  boots and the second thread in 1 of 2. Here the display is stopped right after ready and the second thread got
  there first, 2 of 2. Same call, same code, same handler.
- The leg carries no exit atom: `xvfb-run` returned 1 with its display server already gone
  (`kill: … No such process` from its own cleanup).
- This is the dev host's library set. It shows what the instrument reads; it does not show what ends the app on
  the runner.

## Entry 12 — the series, two boots (data dir `…-series`)

Both calls read the same:

- verdict `all-settled`, `boots` 2, `settled` 2, `ended` 0, `other` 0; exit 0; 21.05 s for the two boots.
- `per_boot`: ordinal 2 and ordinal 3, each `cycle` `complete`, `verdict` `settled`, `ended` null,
  `app_exit_record` `absent`, `windows_settled` 4, `exit_witness` `loaded`. No ordinal 1: no smoke ran in this data
  dir.
- Each boot's cycle printed `boot: ready`, the settle verdict, status `running-healthy`, `cleanup: clean` and its
  marker `series-cycle boot=0 settled=0 status=0 cleanup=0`.
- `logs/series/boot-2/` and `logs/series/boot-3/` each hold five files: `agent-latest.jsonl.2026-10-10`,
  `boot.log`, `exit-witness.jsonl`, `harness-settled.json`, `xvfb.log` (0 B in both). Neither holds a `run`.
- Each boot's own data dir (`series/boot-{ordinal}/`) holds `corpus`, `logs`, `run`, `xdg-cache`, `xdg-data`; it is
  not under `logs/`.
- Each boot's witness file holds one `loaded` line under its own pid (501846, 502370 in the third call).

## Driven once by hand — the boot verb's refusal of a library that is not there

Not an entry of the block; the plan ships this arm with no committed shell-level test (its implementation notes).
`ANDROMEDA_PULSE_EXIT_WITNESS_LIB` set to a path with no file behind it, padded with blanks, on a fresh data dir,
`bash scripts/agent-run.sh boot`: exit 1, stdout 0 B, stderr the one line `boot: exit witness library not found`,
and the data dir holds no file at all (no build log), so nothing was built or spawned.

## The other gates of the third call

`cargo fmt --check` green · clippy green · the four workflow probes green (six jobs; `6 of 6`; `['boot'] ['Boot
series (equal source)', 'Build the exit witness'] True True True True`; the added-line grep `0`) · the library
build green, artifact fresh · the targeted nextest green (the witness and series pins and the controls on the built
library) · `check:english-sources` clean · `capability-widening-check`, `check:ingest-progress`,
`check:staged-artifacts`, `capability-drift`, `verify:capability-matrix` green · the workspace nextest green
(14.8 s) · the bindings regen green · the base-identity bindings close exit 0 · `pre-push:linux` green,
`all-stages-ok`, 86.8 s.
