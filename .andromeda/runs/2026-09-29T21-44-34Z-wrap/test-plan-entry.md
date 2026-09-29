
## 2026-09-29-ci-wall-time-and-round-trips — CI job rows, the pre-push verb, the boot end-status recorder
**Section:** §1 Test harness summary · §3 5-command implementation (`boot`, `status`) · §3 PID file · §3 Per-chunk gate discipline · §9 Pipeline structure · §1 trigger `harness-cleanup-verdict-and-boot-spawn-shell-coverage`
**Change:**
- §3 `status` / §1: the verdict JSON is `{verdict, pid, ended, log_file_basename, last_write_age_seconds, stale_after_seconds}`; `ended` is the `run/andromeda-pulse.exit` record (one line ≤ 48 printable ASCII, else no record), only when not `running-healthy`, null under ps1; arms and exit codes unchanged.
- §3 `boot` / PID file: the sh verb runs the app under a waiting subshell writing `run/andromeda-pulse.spawn` (the provisional pid boot uses; none in 5 s → exit 1) and `run/andromeda-pulse.exit`; the failure path prints `app ended: {record}` (was: the spawned pid IS the app, a `wait` named the signal); ps1 does not mirror it.
- §3: `cargo xtask pre-push:linux` added — dev-host local Linux pre-push check (WSL `Ubuntu` clone of HEAD + worktree, stages `script-modes` · `npm` · `clippy` · `test` · `ci-gates`, exit 0/1/2, `cannot-evaluate` never green, no port, no `pulse-app`; Node 24 from the Viola repo's distro install).
- §9: rows follow the seven jobs — `lint-test` (key `lint-test-{os}`, was `{os}-cargo`; `perf:slo-load` selects its test with `--workspace -E` and reuses the test build) · new `release` and `mcp-test` rows (restore-only) · `a11y` its own matrix job (was inside `lint-test-build`) · `boot` job hosting the smoke + `ci-gates` + `logs-boot-*` (key `boot-Linux`) · `supply-chain` restore-only `boot-Linux` (was `ubuntu-22.04-cargo`), its auditable build the Linux release build · `coverage` registry-only cache.
- §1 trigger: widened again — the failure branch reads the exit record; the wrapper has no committed test (`read_ended` is unit-pinned).
**Why:** the CI split and cache allocation cut the warm round to 25.5 min; the recorder names how a Linux app that dies after `boot: ready` ended, for the boot watch.
**Ref:** .andromeda/runs/2026-09-29T21-44-34Z-wrap/
