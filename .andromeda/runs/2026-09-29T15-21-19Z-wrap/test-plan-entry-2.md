
## 2026-09-29-p-025-hue-shift-observable-made-gradable — harness:status liveness; boot names how a failed app ended
**Section:** §1 Test harness requirements · §3 `boot` (Exit code) · §3 `status` (Command body; Exit code semantics) · §3 PID file → Lifecycle
**Change:**
- status: was derived from the pidfile + log-family mtime; now also a liveness probe of the pid (`ps -o stat=`, a zombie `Z` counts as dead; `tasklist` on Windows) — a dead pid is `not-running` whatever the log says. `classify(pid, alive, newest_log)` is pinned per arm incl. dead-pid and exited-child (was "7 unit pins").
- boot: on a failed poll the sh verb names how the app ended (signal name, exit status, or still running) and runs `bash "$0" cleanup`; `scripts/agent-run.sh` is git mode 100755 (at 100644 the branch died with exit 126).
**Why:** a CI boot smoke whose app panicked in 12 ms read `running-healthy`; fixed in-chunk on a founder ruling.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
