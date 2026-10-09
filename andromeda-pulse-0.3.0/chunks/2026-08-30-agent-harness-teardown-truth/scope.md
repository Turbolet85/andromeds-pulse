# Scope — 2026-08-30-agent-harness-teardown-truth

**Chunk:** Agent-harness teardown truth
**Working-route intent (verbatim outcome):** `cleanup` terminates the APP and `boot`'s ceiling fits the
env-triggered relink, so harness exit codes stop lying.

**Provenance:** entry minted at the `2026-08-30-acl-rejection-logging` wrap's P2 escalation
(operator-approved Apply + mint owner). All coordinates the entry names were re-verified at P1 against the
artifacts themselves (per promotion.md's fold-as-hypotheses rule); verification notes inline below.

## What it builds

1. **Pidfile identity fix** — `scripts/agent-run.{sh,ps1}` stop treating the `cargo run` WRAPPER PID as the
   teardown target; the APP's own PID is what `cleanup` terminates. (Verified at P1: `agent-run.sh` boot does
   `cargo run … & DAEMON_PID=$!; echo "$DAEMON_PID" > "$PIDFILE"`; ps1 registers `Start-Process cargo`'s
   `$proc.Id` — both are the wrapper. The app already writes its own PID: `pulse-app/src/main.rs::write_pid_file`
   → `<data_dir>/run/andromeda-pulse.pid`, registered in arch §Occupied Resources.) The entry's SHAPE note:
   the scripts may simply prefer the app-written file. NO production-binary change is needed [verified at P3:
   `write_pid_file` has exactly ONE caller — `main()` at `pulse-app/src/main.rs:293` (graph query) — runs
   unconditionally at boot, and `resolve_data_dir` reads `ANDROMEDA_PULSE_DATA_DIR` first; MSYS converts
   path-shaped env values on native spawn (probe: `/tmp/probe-x` → `C:/Users/turbo/AppData/Local/Temp/probe-x`),
   so under an exported DATA_DIR the app's write lands in the SAME physical file the script reads].

2. **`cleanup` verdict independent of exit codes** — a cleanup that leaves the app child alive or
   `:4317`/`:4318` still accepting exits NON-ZERO. (Verified at P1: today both scripts print only a stderr
   warning on a still-accepting port and finish with exit 0 / `cleanup: done` — the measured wrong-reason-pass:
   `cleanup` exited 0 with both ports accepting and the app child ALIVE, measured at
   2026-08-30-acl-rejection-logging.) [verified at P3: the ps1 twin shares the identical warning-only shape
   (`Write-Warning` + `cleanup: done`, exit 0) and registers `$proc.Id` of the `cargo` wrapper from
   `Start-Process -PassThru`; `Stop-Process` kills only the named process — no cascade — so the ps1 defect is
   at least as severe; the measured run was the sh script.]

3. **`boot` ceiling fits the env-triggered relink** — the scripts' exported `ANDROMEDA_PULSE_*` env
   re-fingerprints the release build so `cargo run --release` performs a full thin-LTO relink of a WARM binary;
   measured at 2026-08-30-acl-rejection-logging: a 180s `HARNESS_STATUS_TIMEOUT` override died at SIGTERM
   mid-rustc, 900s succeeded. (Verified at P1: default is 10s in both scripts — `agent-run.sh:29`,
   `agent-run.ps1:26` — so a default boot cannot absorb any relink.) Entry's fix shape: write the APP's PID +
   either pre-build under the boot env or a default that absorbs the relink.

4. **Spec re-alignment once green** — test-plan §3 (cleanup Verification/Exit + PID lifecycle) and §1 (harness
   summary) plus `rules/verification-harness.md` move from AS-OPEN back to as-designed. (Verified at P1:
   test-plan lines 66/193/209/210/267 and verification-harness.md lines 22/25/59 all record the defect OPEN
   naming this entry as owner.) The re-alignment is owed at WRAP through the amendment channel — never a phase
   edit; the plan records it as a wrap obligation.

## PREREQ (folded from the entry — pin #22)

Re-check `cargo audit` (standing deferral since `2026-08-15-corpus-key-persistence`; ratified pin #22,
re-pinned here from `2026-08-30-dead-lib-src-test-migration`, origin preserved). **Session 64 is the next owed
FULL-FORM interval point — expected to fall on THIS chunk's wrap; an inserted wrap moves the point, so confirm
the session count at that wrap** (state.yaml `session_count` read 63 at this session's start). Full form:
probe `cargo audit` directly (expect true exit 1, basis `duplicate advisory ID: RUSTSEC-2026-0244` —
byte-identical or the deferral ENDS), run `cargo deny check advisories` and re-enumerate the owned set of
DISTINCT `RUSTSEC-` ids FROM SCRATCH (last enumeration: EMPTY), `bans licenses sources` exit 0. No
"Nth consecutive" ordinal. Rationale: the `2026-08-15-corpus-key-persistence` report.

## Boundaries

- **Touches:** `scripts/agent-run.sh` + `scripts/agent-run.ps1` (harness-only), plus the `.github/workflows/ci.yml`
  "Boot pulse-app smoke" step [added at P3 premise closure: that step runs boot/status/cleanup as THREE separate
  invocations with no exported `ANDROMEDA_PULSE_DATA_DIR`, so under the `$$`-minted default each verb resolves a
  DIFFERENT data dir — cleanup structurally kills nothing there — and `continue-on-error: true` hides every
  verdict]. NO xtask change [verified at P3: `harness_status` has one external consumer (the `xtask/src/main.rs:212`
  dispatch, graph query); its contract is complete for this fix — the cleanup verdict derives from script-side
  port/PID probes, not a new xtask arm].
- **Does NOT touch:** the production binary (the app-side `write_pid_file` already exists and is registered) —
  unless P3's Open Q1 falsifies the landing-path premise; TauRPC surface; capabilities; obs registry (the
  scripts are not obs-allowlist emitters). Zero workspace-dep delta expected.
- **Test surface:** the harness scripts have no committed test today [verified at P3: no test file references
  `agent-run`; test-plan §1's `harness-log-family-resolution-coverage` + `harness-encoding-relay-coverage`
  triggers record the same shell-half-untested shape as standing]; the chunk's proof is the
  measure-first live pair — a RED leg at HEAD reproducing the wrong-reason-pass (cleanup exit 0, app alive) and
  a GREEN leg after (cleanup non-zero on survival / zero on true teardown, ports confirmed released, boot
  within ceiling), per test-plan §3's direct-binary-smoke discipline and its RED-leg shape rules.

## Open questions — CLOSED at P3 (see research.md for evidence)

- **Q1 [premise-corrected: candidate (b) FALSIFIED by probe — MSYS converts path-shaped env values on native
  spawn, so script and app converge on one physical file whenever DATA_DIR is exported].** Candidate (c)
  cannot be the default-shape mechanism (the guard succeeds on converted paths; single unconditional call
  site). The measured wrapper-pid-at-cleanup is therefore a DIVERGENT-PIDFILE shape: either caller-set
  `ANDROMEDA_PULSE_PIDFILE` (the arch-registered threading pattern — candidate (a); the acl-rejection report
  records the outcome but not the env, so (a) is the consistent reconstruction, not a measurement) or the
  NEWLY-FOUND third shape: the default `DATA_DIR` embeds `$$`, so separate boot/cleanup invocations without an
  exported DATA_DIR resolve DIFFERENT dirs — cleanup reads an absent pidfile, kills nothing, and still exits 0
  (CI's exact invocation shape). ALL shapes converge on one fix: the pid that identifies the APP is the
  teardown target, and cleanup's verdict must not depend on the kill's own status.
- **Q2 CLOSED:** verdict from independent probes — PID liveness + TCP handshake on the RESOLVED ports at
  loopback; exit 0 only on pid-gone + both-ports-refusing; exit 1 with a bounded verdict token
  (`app-survived` / `ports-lingering` / `no-pid-ports-accepting`) otherwise; structured stdout line, not
  stderr prose.
- **Q3 CLOSED:** pre-build under the boot env, then spawn `target/release/pulse-app.exe` BY PATH — the relink
  (whichever env var fingerprints it; the exact var is unattributed in the record and the fix is deliberately
  mechanism-agnostic) is absorbed OUTSIDE the timed readiness window, and direct spawn dissolves the wrapper
  entirely so `$!`/`$proc.Id` IS the app. The 10s `HARNESS_STATUS_TIMEOUT` default becomes honest for a
  prebuilt binary; no ceiling raise needed.
- **Q4 CLOSED:** ps1 mirrors 1:1 under direct spawn (`Start-Process target\release\pulse-app.exe` → `$proc.Id`
  is the app's Windows pid, same identity space as the app-written file; `Stop-Process -Id` by app pid).
  sh-on-Windows kills the file-recorded Windows pid via the PowerShell fallback per the 2026-05-19/2026-08-23
  verification-harness rules (never `taskkill /F` under Git Bash).
