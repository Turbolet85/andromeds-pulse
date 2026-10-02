# Codebase Research — 2026-08-30-agent-harness-teardown-truth

## Scope
- **Depth:** moderate · **Reads:** 8 (both scripts full · `write_pid_file` + `resolve_data_dir` in
  `pulse-app/src/main.rs` · `xtask/src/harness_status.rs` full · ci.yml harness step · acl-rejection report
  §deviation-4 · testing.md fingerprint family · test-plan §3 PID block) · **Globs/Greps:** 6 · **Probes:** 1
  live env-conversion measurement

## Files inspected
- `scripts/agent-run.sh` (full) — boot writes `$!` (the `cargo run` WRAPPER pid) to `$PIDFILE` at spawn
  (`DAEMON_PID=$!; echo "$DAEMON_PID" > "$PIDFILE"`); cleanup kills `$(cat $PIDFILE)`, port check is a stderr
  WARNING with unconditional `cleanup: done` + exit 0; `DATA_DIR` default is `${TMPDIR:-/tmp}/agent-run-$$` —
  **`$$` differs per invocation**, so separate verbs share no state unless the caller exports
  `ANDROMEDA_PULSE_DATA_DIR`; boot-failure path calls `"$0" cleanup` then exit 1; `STATUS_TIMEOUT_SEC`
  default 10 (`HARNESS_STATUS_TIMEOUT` override); ports resolved from `ANDROMEDA_PULSE_OTLP_{GRPC,HTTP}_PORT`
  (defaults 4317/4318); port probe = `timeout 1 bash -c "cat < /dev/null > /dev/tcp/127.0.0.1/$port"`.
- `scripts/agent-run.ps1` (full) — identical shape: `Start-Process -FilePath 'cargo' … -PassThru` →
  `$proc.Id | Out-File $PidFile` (wrapper pid); `Invoke-Cleanup` `Stop-Process -Id` (no cascade on Windows —
  the app child survives a wrapper kill), port warning via `Write-Warning`, exit unconditionally 0;
  `$StatusTimeoutSec` default 10.
- `pulse-app/src/main.rs:182-243` — `resolve_data_dir()` reads `ANDROMEDA_PULSE_DATA_DIR` FIRST (then
  APPDATA→`andromeda-pulse` / macOS `com.andromeda.pulse` / XDG / HOME / temp);
  `write_pid_file(data_dir)` at `main()`:294: create `run/`, canonicalize both sides, confinement check
  (basename-only diagnostics on `app.boot.pid`), write `std::process::id()` to `run/andromeda-pulse.pid`.
  Runs unconditionally, EARLY in main — i.e. immediately post-relink when spawned via `cargo run`.
- `xtask/src/harness_status.rs` (full, 231 lines) — `resolve_paths()`: trim + fall back
  (`ANDROMEDA_PULSE_PIDFILE` → `<data_dir>/run/andromeda-pulse.pid`); `classify()` pure, 4 arms, 7 pins;
  `read_pid` rejects garbage; `newest_family_member` globs the rotated family. Contract complete for this
  chunk — no teardown arm needed.
- `.github/workflows/ci.yml:145-151` — "Boot pulse-app smoke" step: **Linux-only, `continue-on-error: true`**,
  runs `bash scripts/agent-run.sh boot` / `status` / `cleanup` as THREE separate invocations with NO exported
  `ANDROMEDA_PULSE_DATA_DIR` → each mints a different `agent-run-$$`; status/cleanup structurally read absent
  pidfiles; every verdict is swallowed by continue-on-error. The step runs after
  `cargo build --workspace --release` (line 135), so the binary is warm but built under a DIFFERENT env.
- `andromeda-pulse-0.3.0/chunks/2026-08-30-acl-rejection-logging/report.md:98-102` — the measured narrative:
  relink outran a 180s override (default NOT measured that session), 900s succeeded; cleanup exit 0, ports
  open, app alive, "pidfile names the wrapper"; the run's exact env (custom PIDFILE vs default) is NOT
  recorded — reconstruction, not measurement.
- `.claude/rules/testing.md` (fingerprint family) — "the session env-prefix is part of cargo's FINGERPRINT"
  (2026-08-30 facet); the exact var that re-fingerprints the release build under the boot env is
  UNATTRIBUTED in every record ("the script's exported env" is the whole attribution).
- `.andromeda/test-plan.md:261-267` — §3 PID-file block: the per-OS Location set
  (`$XDG_RUNTIME_DIR/…` | `$TMPDIR/…` | `%LOCALAPPDATA%\andromeda-pulse\pid`) matches NEITHER the scripts,
  the app, nor arch §Occupied Resources (`<data_dir>/run/andromeda-pulse.pid`) — stale v1 text; joins the
  already-owed §3 wrap re-alignment.

## Graph impact (trace: `tree-query-2026-08-30-agent-harness-teardown-truth.json`, rust plane)
- **`write_pid_file`** — exactly 1 caller: `main()` at `pulse-app/src/main.rs:293`. Zero cross-crate blast;
  no product change needed or permitted.
- **`harness_status`** — 1 external consumer: the `xtask/src/main.rs:212` dispatch (plus module decl at :11).
  The verdict contract is closed; this chunk does not touch it.
- The modify-set proper (`agent-run.sh` / `agent-run.ps1` / ci.yml) is SHELL/YAML — outside both graph planes;
  file-first research above covers it (recorded per the cookbook's skipped-plane rule: not `derived-without-graph`
  — the rust plane WAS queried for every adjacent Rust symbol).

## Patterns detected
- **Wrapper-pid registration** (`agent-run.sh` boot; `agent-run.ps1:~100`): both scripts record the `cargo`
  wrapper's pid, not the app's — the defect's mechanical root.
- **Same-file coincidence** (`agent-run.sh:27` vs `main.rs:211`): the script's default `$PIDFILE` IS the
  app's registered write target, so under exported-DATA_DIR defaults the app OVERWRITES the wrapper pid at
  boot and cleanup would kill the right process; the measured wrong-reason-pass therefore arose from a
  divergent-PIDFILE or no-export shape (see scope Q1 closure).
- **MSYS env conversion** (live probe): `ANDROMEDA_PULSE_DATA_DIR=/tmp/probe-x` reaches a native process as
  `C:/Users/turbo/AppData/Local/Temp/probe-x` — Git-Bash-vs-native path divergence for exported env values is
  NOT a real mechanism on this host.
- **Warning-not-verdict** (both scripts' cleanup): port residue → stderr text + exit 0 — the exact
  obs-plan §11 "unstructured stderr text" ban.
- **Truthful-status precedent** (`harness_status.rs`): pure `classify()` + bounded arms + JSON on stdout +
  per-arm pins — the shape Q2's cleanup verdict follows (script-side, bounded token line).

## Conventions to follow
- **Dev-binary path**: `target/release/pulse-app.exe` (never `target/{profile}/andromeda-pulse*`) — arch
  §Occupied Resources §Process/service identity.
- **Build outside the timed section, invoke BY PATH** — rules/testing.md 2026-08-23 (injector rule,
  same class).
- **Kill-by-specific-PID via PowerShell on Windows; never `taskkill /F` or `tasklist /FI` under Git Bash**
  (the `/X`-mangling class); liveness probes must not let a `||` fallback manufacture the answer —
  rules/verification-harness.md 2026-05-19 + 2026-08-23.
- **No naked sleep-as-sync** — bounded loops POLLING pid-liveness/port state (test-plan §11 E2E).
- **Port probes dial loopback at the RESOLVED ports** (scripts already resolve `ANDROMEDA_PULSE_OTLP_*_PORT`).

## New files to create
- (none)

## Files to modify
- `scripts/agent-run.sh` — boot: pre-build (`cargo build --bin pulse-app --release` under the exported env)
  + direct spawn by path + pidfile becomes the APP identity; cleanup: independent PID+port verdict, bounded
  token line, exit 0/1; missing-run-dir honesty (absent pidfile + ports accepting → exit 1).
- `scripts/agent-run.ps1` — 1:1 mirror of the above.
- `.github/workflows/ci.yml` — the "Boot pulse-app smoke" step: export one shared `ANDROMEDA_PULSE_DATA_DIR`
  for the three invocations; `continue-on-error` disposition is a plan fork (P4).
- (No caller threading: the scripts' only invokers are ci.yml (this list), operators, and chunk legs; the
  xtask `perf:load-profiles` NEUTRAL message mentions `agent-run boot` as advisory text only. The a11y chain
  does not consume the scripts — no reference under `pulse-app/ui/`.)

## Open questions
- **CI gating** (`continue-on-error` flip) → blocks: plan-decision — put to the operator at P4 (contestable:
  truthful-harness-should-gate vs observe-first-under-CI-flake-risk).
- (none else — scope Q1–Q4 closed at premise closure; the unattributed relink var is deliberately moot under
  the mechanism-agnostic pre-build fix, and the RED leg's reproduction recipe — divergent `ANDROMEDA_PULSE_PIDFILE`
  or no-export default — is an implementation-scope choice /implement verifies against its own precondition.)

## Notes for wrap (spec-side findings outside this chunk's edit surface)
- test-plan §3 PID-file Location per-OS set is stale vs code/arch (above) — fold into the owed §3 re-alignment.
- obs-plan lines 93/225 name `%APPDATA%\Andromeda Pulse\logs\` (space + caps) + macOS/Linux dirs that disagree
  with `resolve_data_dir()` (`%APPDATA%\andromeda-pulse`, XDG/HOME forms) and with arch §Filesystem locations —
  doc drift, moot for the harness (DATA_DIR always exported) but an obs amendment candidate.
- ci.yml's harness step blindness (`continue-on-error` + unshared DATA_DIR) predates this chunk — the step has
  never been able to verify anything cross-verb; record in the report as a measured-at-research finding.
