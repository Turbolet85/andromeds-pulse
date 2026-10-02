
## 2026-10-01-conductor-return — process-end witness form and its composition trigger
**Section:** §1 Pending coverage triggers · §3 Per-chunk gate discipline
**Change:**
- §1: new trigger `exit-hook-main-composition-coverage` — the `main.rs` composition (`hold_log_guard(init(..))` + `install_exit_hook()` + the Unix `install_signal_listener()`, and the `.build(ctx)` + `run_return` + `exit_after_event_loop(code)` tail that replaced `.run(ctx)`) has no committed test; the live event-loop exit is not inducible headless; the main-thread panic → `outside_event_loop` ordering is unmeasured. Owed: a seam lifting the composition out of `main.rs`, or a headful Tray-Quit leg.
- §3: the process-end witness form — re-exec children, each on its own TempDir, one class each, the parent reading the log family after the child ended; child-ran proof in-arm (the child's `app.boot.tracing.init` record — child stdout is discarded); a once-flag whose sink's only drain is a guard drop pinned by the emitter's return value through the exit code, never a file read; `cfg(unix)` arms run by `pre-push:linux` and CI lint-test Linux/macOS; the unloggable-by-construction ends out of reach.
**Why:** the lib functions are witnessed but `main`'s composition is not (same one-time-proof class as `discovery-observer-wiring-coverage`); the mutation check showed a file-reading arm stays green with the whole once-guard removed, so the form must say where such a flag is pinned.
**Ref:** .andromeda/runs/2026-10-02T12-54-57Z-wrap/
