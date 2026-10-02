# P3 warm boot smoke (2026-09-30, /andromeda-implement)

The founder's word at P4 ("Run it"). The host constraint on pulse-app / 4317 was released for this run.

- Build (warm): `cargo build -p pulse-app` (1m44s, exit 0) and `cargo build -p ingest --example inject_demo` (exit 0).
- Data dir = cwd = `%LOCALAPPDATA%/Temp/smoke-scrub/rm-20260930-065023`: fresh, `rm-YYYYMMDD-HHMMSS`-shaped, the measured naming shape.
- Launch: `target/debug/pulse-app.exe` by path, with `ANDROMEDA_PULSE_DATA_DIR` = that dir and `ANDROMEDA_PULSE_L4_DETERMINISTIC=1` in its own spawn env. Pidfile `run/andromeda-pulse.pid` = 64616.
- `:4317` was listening within one 2-second poll.
- Seed: `./target/debug/examples/inject_demo.exe` under `timeout 25`. rc=124 is the expected bound for the storm (41/600 batches, 1107 spans sent).

## Readings: `logs/agent-latest.jsonl.2026-09-30`, 14,222 lines, all written this run (fresh dir)

| check | reading |
|---|---|
| `app.panic.fatal` | 0 |
| `level: ERROR` | 0 |
| `level: WARN` | 1: `interpretation.model.allow_root` `confinement=unconfined` (the documented once-per-boot posture record) |
| `digest.assemble.request` | 3 |
| `interpretation.prompt.assemble` | 3 |
| `interpretation.incident.created` | 3 (first: `created=true`, `severity=error`, `priority_tier=autonomous`) |
| `app.boot.workspace_key` | `workspace_root_basename = rm-20260930-065023`: detection yielded the measured shape |
| `buffer.tick` (last) | `rows_ingested = 1323`, `redactions_applied = 0` |

`redactions_applied = 0` holds under a date-stamped cwd and data dir. That is consistent with the fix: the demo injector carries no card-shaped PII, and the workspace basename no longer trips the card arm. The counter only covers the buffer write sites, though, so this is context, not proof. By design no log field carries scrubbed text, so the digest and Report text equality rests on the Step 2–5 pins.

## Stop
- `Stop-Process -Id 64616 -Force`. After it the pid was gone, `:4317`/`:4318` were closed, and no `pulse-app`/`inject_demo` process remained.
- The backgrounded launch wrapper then reported exit 127. That is the forced kill of its child, not a launch failure: the app had run, written its pidfile and served the seed.
