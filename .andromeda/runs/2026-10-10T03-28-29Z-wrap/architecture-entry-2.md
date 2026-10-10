
## 2026-10-10-boot-smoke-s-self-end-named-from-a-run — the witness variables, the kept files and the stack row
**Section:** §Occupied Resources → Environment variables; §Occupied Resources → Filesystem locations; §Stack and Technologies
**Change:**
- Environment variables, new rows: `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (read by `agent-run.sh boot` alone; trim, regular-file check, fail closed) · `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` (set on the spawn line, read by the preloaded library inside the app's process, which removes it and `LD_PRELOAD` at load and checks nothing of the path) · `LD_PRELOAD` (set on the spawn line alone; the one place harness code runs inside the app's process) · `XDG_DATA_HOME` · `XDG_CACHE_HOME` (set by the series on each of its boots). `PATH` gained a second harness-only by-value reader (the series, to find `xvfb-run`); the `ANDROMEDA_PULSE_PIDFILE` and `_LOGFILE` rows say the series removes them per boot.
- Filesystem locations: the harness-written set under `logs/` was two files; now also `logs/exit-witness.jsonl`, `logs/boot-series.json` and `logs/series/boot-{ordinal}/`; the artifact's contents as read on `ci#38019133294`. New subpath `series/boot-{ordinal}/` (per-boot data dirs, not uploaded). New harness-only entry outside the data dir: `scripts/exit-witness.c` and its two build outputs; the three product-written exceptions unchanged.
- §Stack: a harness-only row for the C exit-witness library built by the host's `cc` (dev host GCC 16.2.1 20260810, as measured 2026-10-10; the runner's version not read); the xtask controls on the built library fail on a missing `cc`.
**Why:** Each is a resource this chunk introduced. The classification of the library that runs inside the app's process is security-plan's and is PROVISIONAL there.
**Kept:** the stack row was shown to the operator beside the escalation and applied on the operator's word (the pc overseer, 2026-10-10).
**Ref:** .andromeda/runs/2026-10-10T03-28-29Z-wrap/
