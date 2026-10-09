# Report — 2026-08-30-agent-harness-teardown-truth

**Chunk:** Agent-harness teardown truth — `cleanup` terminates the APP and `boot`'s ceiling fits the
env-triggered relink, so harness exit codes stop lying
**Date:** 2026-08-30T21:40Z
**Commits:** none since last_wrap (this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)
- **Files:** `scripts/agent-run.sh` · `scripts/agent-run.ps1` · `.github/workflows/ci.yml` (+ chunk-folder
  artifacts `scope.md`/`research.md`/`plan.md`/`report.md`; phase + wrap run dirs). ZERO `.rs` / manifest /
  lockfile / webview delta.
- **Symbols / APIs:** no Rust or TS symbols. Harness CLI surface (test-plan §3's contract, sh+ps1 in
  lockstep): `boot` now PRE-BUILDS under its own exported env (`cargo build --bin pulse-app --release`, then
  `cargo build -p xtask` — the readiness poll runs `cargo xtask`, so a stale xtask would otherwise rebuild
  inside the timed window) and spawns `target/release/pulse-app[.exe]` BY PATH — the `cargo run` wrapper is
  GONE; the spawn pid is the app, and the app's own `write_pid_file` overwrites the default pidfile with its
  canonical pid (measured live: file held app pid 59828 after ready, not the script's `$!` 128125). `cleanup`
  now derives its verdict ONLY from independent probes (pid liveness via msys `kill -0` with a
  genuinely-probing PowerShell `Get-Process` fallback; TCP handshake on the RESOLVED
  `ANDROMEDA_PULSE_OTLP_{GRPC,HTTP}_PORT` at `127.0.0.1`) and emits ONE bounded stdout token —
  `cleanup: clean | app-survived | ports-lingering | no-pid-ports-accepting` — exit 0 ONLY on `clean`,
  exit 1 otherwise (was: stderr warning + unconditional `cleanup: done` exit 0). `status` / `run` / `logs`
  verbs byte-identical; the 5 command names unchanged. No new env vars; `ANDROMEDA_PULSE_PIDFILE`/`_LOGFILE`
  remain harness-only and unconsumed by the product (no product-binary change — `write_pid_file`'s single
  caller `main() @ pulse-app/src/main.rs:293` untouched).
- **Crates / modules:** none added/removed/changed.
- **Dependencies:** none.
- **Schema / config:** `.github/workflows/ci.yml` "Boot pulse-app smoke" step (Linux-only): removed
  `continue-on-error: true` — the step now GATES (operator-approved at phase P4). No env change: the
  workflow-level `ANDROMEDA_PULSE_DATA_DIR` (ci.yml:12) already gives the three invocations one shared dir,
  so the plan's step-level env addition executed as a reported NO-OP.
- **Spec-master edits:** none (this chunk's code edits touched no master; the owed re-alignments are this
  wrap's P2).
- **Counts / qualifiers moved:** none — verified.
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none.
- **Spec claims disproved by measurement:**
  1. **test-plan §3 PID file → Location** states a per-OS path set (`$XDG_RUNTIME_DIR/andromeda-pulse.pid` |
     `$TMPDIR/andromeda-pulse.pid` | `%LOCALAPPDATA%\andromeda-pulse\pid`, fallback
     `~/.andromeda-pulse/run/andromeda-pulse.pid`) that matches NEITHER the scripts (default
     `<data_dir>/run/andromeda-pulse.pid`), the app (`write_pid_file` → same), nor arch §Occupied Resources —
     stale v1 text (measured at research; `.andromeda/test-plan.md:263`). Disposition: P2 amendment (joins the
     owed §3 re-alignment).
  2. **obs-plan §3 Log file location** (lines 93 + 225) names per-platform dirs
     (`%APPDATA%\Andromeda Pulse\logs\` — space + caps; macOS `~/Library/Application Support/com.andromeda.pulse/logs/`;
     Linux `~/.local/share/com.andromeda.pulse/logs/`) that disagree with `resolve_data_dir()`
     (`%APPDATA%\andromeda-pulse`; XDG/HOME forms; macOS matches) and with arch §Filesystem locations.
     Disposition: P2 amendment (obs; Windows + Linux halves wrong, macOS half correct).
  3. **research.md/scope.md (THIS chunk's own artifacts)** claimed ci.yml's harness step exports no shared
     `ANDROMEDA_PULSE_DATA_DIR` — FALSIFIED at implement P1 Read-before-Edit (workflow-level env at
     ci.yml:11-12; the narrow basis was a step-block-only read). The `$$`-divergence defect is real but its
     victim is BARE LOCAL invocations (proven live by the bonus arm), not CI. Chunk-artifact class: no
     sanctioned writer at wrap — **recorded here, no amendment owed** (this bullet IS the disposition). The
     phase P4 CI-gating question carried the false detail; its ruling ground (continue-on-error swallowing
     every verdict — measured-true) survives, and the approved gating landed.
  4. **rules/verification-harness.md** (leaf) restates the same stale per-OS PID location set + the
     `cargo run --release` boot body + the AS-OPEN cleanup caveats — re-aligns via the test-plan amendment's
     CASCADE, not its own amendment.
- **Coverage of new surfaces:**
  - `agent-run cleanup verdict (sh+ps1)` → validation n/a (harness-only; pidfile content numeric-trimmed) ·
    instrumentation n/a (verdict = exit code + stdout token BY DESIGN — obs-plan §11; scripts are not obs
    emitters) · PII n/a (bounded tokens only; no payloads/paths in the verdict line) · tests: live RED/GREEN
    legs this session (no committed shell test — the standing shell-half-untested shape, test-plan §1
    trigger family) · a11y n/a · tokens n/a
  - `agent-run boot pre-build + direct spawn (sh+ps1)` → validation n/a · instrumentation n/a · PII n/a ·
    tests: live legs (boot 1.953s under the DEFAULT 10s ceiling) · a11y n/a · tokens n/a

## Deviations from intent
1. **ci.yml env half → NO-OP** (plan step 4): workflow-level env pre-satisfies it; see disproved-claim 3.
2. **xtask added to the pre-build phase** (both scripts): not in the plan's literal step; the readiness poll
   runs `cargo xtask harness:status`, so a stale xtask would rebuild INSIDE the timed window — the same
   mechanism the chunk fixes, absorbed the same way. In-scope of the rewritten boot arm.
3. **Two latent ps1 bugs fixed inside the touched arms**: (a) `Start-Process` rejects the SAME file for
   stdout+stderr — the old boot arm redirected both to `boot.log` and would throw; now separate
   `boot.log`/`boot.err.log` (+ build logs). (b) `Write-Error` under `$ErrorActionPreference='Stop'` THROWS —
   the old boot-timeout path died before `Invoke-Cleanup` ran; now `Write-Warning` + explicit cleanup +
   `exit 1`. The SAME `Write-Error` class survives in the UNTOUCHED `logs`/`default` arms (their
   `exit 1`/`exit 2` lines are unreachable; script still exits 1 via the throw) — out of scope, left as
   noted residue.
4. **Rust workspace gates deferred** (clippy `--all-features` + workspace nextest): zero compiled-source
   delta (scripts+yml only; no manifest/lockfile/build-script/codegen delta). Sanctioned
   source-delta-proportional deferral; origin marker `2026-08-30-agent-harness-teardown-truth`; the next
   Rust-touching chunk closes it (P5 pins the PREREQ).
5. **RED-leg prep**: pre-warmed the release build under the leg env + `HARNESS_STATUS_TIMEOUT=60` so the leg
   measured the CLEANUP defect rather than re-measuring the relink (whose RED evidence is the recorded
   2026-08-30-acl-rejection measurement); the defect reproduced exactly regardless.
6. **ps1 not live-driven**: parse-validated (`PSParser`, 1000 tokens) + logic-mirrored; its PowerShell
   primitives (`Get-Process`/`Stop-Process` by id) were exercised live via the sh legs' fallback path. A full
   ps1 boot cycle remains operator-runnable; recorded as an honest limit, not claimed.

## Decisions & corrections
- **Operator ruling (phase P4):** the CI smoke step GATES — `continue-on-error: true` dropped; ground: a
  truthful exit code CI ignores stays ignored (the swallowed-verdicts half of the premise, which survived
  the env-half correction).
- **Design decision (decisive leans per artifacts):** pre-build + direct-spawn-BY-PATH over
  cargo-run + bigger ceiling — dissolves the wrapper structurally (the pidfile identity defect's root)
  AND absorbs the relink mechanism-agnostically (the exact fingerprinting var remains unattributed;
  the fix does not depend on naming it). The 10s default ceiling is now honest: measured boot-to-ready
  1.953s on a warm target.
- **Identity insight (measured live):** with a shared default pidfile the app's `write_pid_file` OVERWRITES
  the script's provisional `$!` — file held app pid 59828 vs `$!` 128125 (msys pid space ≠ Windows pid
  space); cleanup treats FILE content as canonical and probes/kills across both spaces (msys `kill` first,
  PowerShell fallback — never `taskkill`/`tasklist /X` forms, never a `||`-manufactured answer).
- **Correction (narrow-basis, self-caught):** a CI-env claim derived from a step-block-only read was
  falsified by the workflow header — re-derive env claims from the WHOLE workflow file, not the step.
- **Verdict-shape decision:** bounded token line + 0/1 exits on the harness-CLI contract model
  (`harness:status` precedent); `no-pid-ports-accepting` makes the formerly-silent
  cannot-locate-the-run shape a loud exit-1.

## Outcome
All 9 acceptance criteria met; gates green; smoke ✓.
- **Gates run (implement P2, re-run P7):** `cargo fmt --check` 0 · `cargo xtask capability-widening-check` 0 ·
  `cargo xtask check:ingest-progress` 0 · `cargo xtask check:staged-artifacts` staged-clean 0 ·
  `cargo xtask capability-drift` 0 (LAST; folded staged assertion green). Deferred with origin (deviation 4):
  clippy + workspace nextest.
- **Live pair (measure-first):** RED at HEAD — boot registered the WRAPPER (msys 127966) in the caller-set
  pidfile while the app wrote 35288; `cleanup: done` exit 0 with BOTH ports accepting and the app ALIVE
  (Get-Process-verified precondition); orphan killed by specific pid. GREEN after — boot ready 1.953s under
  the DEFAULT 10s ceiling; `status` running-healthy on the app pid; engineered-survival arm
  `cleanup: ports-lingering` exit 1; true teardown `cleanup: clean` exit 0 with both ports refused and the
  app gone; bonus bare-shape arm (no exports, separate invocations) `cleanup: no-pid-ports-accepting` exit 1
  then env'd `clean` exit 0. Leg logs: 0 `"level":"ERROR"` · 0 `app.panic.fatal`.
- **Pin #22 — session-64 FULL-FORM interval point DISCHARGED at this wrap** (session count confirmed: 63 → 64
  at this wrap): `cargo audit` true exit 1 read directly, basis byte-identical
  (`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`) — the deferral
  continues; `cargo deny check advisories` exit 0, owned set re-enumerated FROM SCRATCH = EMPTY (0 distinct
  `RUSTSEC-` ids); `cargo deny check bans licenses sources` exit 0. Next interval point: session 67
  (65/66 between-points).
- **Matrix:** untouched — this chunk claims 0 capabilities (P-075 remains the version's one unclaimed cap).
