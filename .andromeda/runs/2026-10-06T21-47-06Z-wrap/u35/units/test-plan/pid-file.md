### PID file

- **Location:** `<data_dir>/run/andromeda-pulse.pid` on every platform — `<data_dir>` is `resolve_data_dir()` (overridable via `ANDROMEDA_PULSE_DATA_DIR`), and the harness-only `ANDROMEDA_PULSE_PIDFILE` overrides the whole path. The single form the scripts, the app's `write_pid_file`, and arch §Occupied Resources all use (the former per-OS `$XDG_RUNTIME_DIR`/`$TMPDIR`/`%LOCALAPPDATA%` set was stale v1 text, measured false at chunk 2026-08-30-agent-harness-teardown-truth)
- **Lifecycle:** 
  - `boot` writes a provisional pid — since chunk 2026-09-29-ci-wall-time-and-round-trips the one the waiting wrapper records in `run/andromeda-pulse.spawn` — and the app's own `write_pid_file` overwrites the default pidfile with its canonical pid at boot (measured live earlier: file held app pid 59828, not the script's `$!` 128125 — msys pid space ≠ Windows pid space). Beside the pidfile the harness (never the product binary) also writes `run/andromeda-pulse.spawn` and `run/andromeda-pulse.exit`; the latter is `harness:status`'s `ended`
  - `status` reads the pidfile, probes that pid's liveness (a dead or zombie pid is `not-running`), and reads the log-family mtime via `cargo xtask harness:status`
  - `cleanup` terminates the pid the FILE holds (canonical), probing/killing across both pid spaces (msys `kill` first, PowerShell `Get-Process`/`Stop-Process` fallback — never `taskkill`/`tasklist` forms); the wrapper-pid defect is CLOSED (chunk 2026-08-30-agent-harness-teardown-truth)
- **Format:** single decimal PID on one line, no newline-stripping needed (portable)
