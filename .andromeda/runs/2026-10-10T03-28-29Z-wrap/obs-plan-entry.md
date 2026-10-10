
## 2026-10-10-boot-smoke-s-self-end-named-from-a-run — the boot job's kept files, the frame line's condition, the measured unloggable end
**Section:** §9 CI Integration → Telemetry artifact handling (Log file row); §10 SLO Invariants → Performance budgets (WebGPU canvas frame row); §10 → CI gates (perf-budget bullet); §7 Error Capture → Error classes captured (Process-end cause)
**Change:**
- Log file row: was "the artifact holds five files"; now held five on `ci#37979648967` and, as read on `ci#38019133294`, also `exit-witness.jsonl`, `boot-series.json` and `series/boot-{2..8}/` (five files each, four where a boot took no settle verdict); `harness-settled.json` holds eight members, the eighth `exit_witness`; the series' data dirs are not uploaded.
- Frame row and CI gates bullet: the boot job's `ci-gates` line `… no WebGPU adapter (no_navigator_gpu)` was the expected line on every run whose smoke step passes; now on a run whose smoke step and `Boot series (equal source)` step both pass, and not printed when either fails.
- Process-end cause: the partition kept; added, as measured on `ci#38019133294`, that the CI boot job's self-end is `_exit(1)`, `errno` 11, from a static function of `libgdk-3.so.0` entered from Xlib's `_XIOError` (seven equal `end` lines), an unloggable end; that the harness's exit witness leaves one `end` line in `logs/exit-witness.jsonl` for such an end; and what is not measured (why the X connection's read failed; the runner's library build, the GTK source read recorded as a source read).
**Why:** The end with no `app.exit` is now named from a run. It is not closed: the cause's owner is the route's closing entry, and the witness is a harness surface, not a product record.
**Kept:** `app.exit` stays at four fields and gains no new record for this end.
**Ref:** .andromeda/runs/2026-10-10T03-28-29Z-wrap/
