# Scope — 2026-10-01-conductor-return

**Working entry (verbatim title + hint):** Conductor return — the external P-075 assert round runs and the version's
last unclaimed capability verifies.

**Chunk base:** `a2addb3` (W182) · version andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification (the
epoch's last markerless entry; it closes when this chunk completes — operator no-split ruling stands).

## What the chunk delivers
1. **PREREQ first** — every non-zero exit path of `pulse-app` logs its cause before the process exits (below).
2. **The P-075 claim** — the version's last unclaimed capability (`verification-matrix.json#P-075`, method
   `dynamic-external`, status `planned`, `chunk: null`; `matrix.py coverage` 21/22, unclaimed `[P-075]`). Its
   acceptance: "Conductor drives a deterministic incident and reads it back via MCP, asserting e2e content-fidelity
   plus P-025 <=2s / P-027 <=5s / P-037 <=2s / P-045 <=1s". The claiming chunk concretizes it (contract §Acceptance
   lifecycle), so this chunk owns the re-authoring the cap's notes defer to "whichever chunk finally claims the cap".
3. **The external assert round** — Conductor (a companion repo, read-only from here) drives the real Pulse binary and
   asserts; Pulse's side is the binary at a named sha, the evidence citation (cite Conductor's paths, never copy
   content — 2026-08-21 relay discipline), and the matrix `ref`.

## Folded freight (the entry's one block: PREREQ 543 chars — `route.py pins`)

### PREREQ — exit-cause logging (operator-directed at the 2026-10-01-real-model-incident-surfacing wrap, 2026-10-01)
- "every non-zero exit path of pulse-app logs its cause before exiting, so the next unexplained death names itself".
- "the Linux CI boot smoke's app ended `exit 1` 0.27 s after `boot: ready`, before any webview IPC, with no panic and
  no ERROR beyond the keyring line to say why (1 of 2 attempts at `69f0b93`: `ci#36893004900` job 110472991644; its
  single re-run on the same sha read `running-healthy`); cause unexplained, no confining mechanism identified".
  COORDINATES VERIFIED at take-up against the prior chunk's record
  (`chunks/2026-10-01-real-model-incident-surfacing/evidence/operator-pass.md` §Red 2): the failed job 110472991644,
  `"ended": "exit 1"` (the app's own code, not a signal), 40 records over 0.27 s, 0 `app.panic.fatal`, ERROR only
  `corpus.open.error {error_kind: KeyringUnavailable}`, no webview-originated record; re-run job 110496296816 read
  `running-healthy`.
- The exit is not one of pulse-app's own Rust exit sites — VERIFIED, and widened at P3 (research.md §Mechanism
  re-derivations 1): `pulse-app/src` has one explicit exit, `tray.rs:271` `app.exit(0)`; across the whole lockfile graph
  (916 packages swept) the only runtime non-zero path is tao's `process::exit(exit_code)`, whose code is 0 on every exit
  tauri-runtime-wry requests, and a panic exits 101. By elimination a bare `exit 1` with 0 `app.panic.fatal` is a NATIVE
  `exit()` (GTK/GDK/WebKitGTK/glib) — seen only by a hook that runs at exit. Public twin of the signature: YUI #729
  (silent exit 1 under Xvfb, cause unknown, closed not planned) — research.md §Scope. This classifies the candidate set;
  it does NOT explain the 69f0b93 death.
- "Every non-zero exit path" is enumerable — VERIFIED; the classes and how each ends (research.md §Mechanism
  re-derivations 2-5): (a) event-loop exit — tao `process::exit(code)`, no destructor runs, so `main`'s `_guard`
  (main.rs:293) never drops and the non-blocking worker is never flushed; a record emitted just before it races the
  worker (loss UNMEASURED — the RED leg's first measurement); `App::run_return` returns the code to Rust instead;
  (b) native `exit()` — runs `atexit` handlers (bound by `libc` 0.2.186 on unix and windows; the exit status is NOT
  available there — glibc `on_exit` is unbound); (c) panic — `app.panic.fatal` exists; main-thread unwinding drops the
  guard; (d) signals — no handler today; SIGTERM/SIGINT are hookable on Unix, SIGKILL and Windows `Stop-Process -Force`
  are unloggable by construction; (e) pre-sink failures (main.rs:288 runtime `.expect`, observability.rs:2624 logs-dir
  `.expect`) can reach only stderr / `boot.log` by construction. Tray Quit (`app.exit(0)`) and window close (hide to tray)
  are not non-zero exits.
- The witness can fail — VERIFIED shape: the re-exec form (testing.md:260 — `current_exe()` with `--exact`, state through
  env, the child writes to a temp sink, a skip-clean guard, `--no-capture` once to prove the child ran) induces each
  loggable class in a child and reads `agent-latest.jsonl*` after the child has ended — CI-runnable, no window. At base the
  cause record is ABSENT (no exit-cause target exists at HEAD), the block-shaped RED (testing.md:290). The Linux runner
  class stays unreproducible here; the CI boot smoke is the runner-side observer.
- Logging discipline applies — VERIFIED: own exact allowlist leaf (no bare `app` key, observability.md:65), closed-enum /
  integer / basename fields only, guard under `pulse-app/tests/`, dual-site registration (obs-plan §6 + §8) as an
  expected wrap amendment.

## The P-075 claim — what must hold at claim
- The P-025 clause — VERIFIED gradable, and graded: Conductor `v3-08` (`2026-09-29-hue-shift-budget-graded-hard`) drove one
  graded live leg at Pulse `4502d5d` (carrying `e98d838`) under `contracts/pulse-p025-measurement-contract.md`: PASS, worst
  684.98 ms <= 2000. The hue path is unchanged since `e98d838` (only `5fbf762` touched `telemetry.rs`, adding the WebGPU
  adapter procedure). The cap's 2026-08-22 over-claim note is resolved by concretizing the clause to that contracted
  quantity (constellation dot hue, never the deferred Halo glow).
- [premise-corrected: v2-20's P-027 PASS (2026-08-21) graded the tick-anchored `discovery_ms` that
  `2026-09-30-p-027-discovery-bound` measured to HIDE the wait (435 ms vs a true 15 219 ms), and every budget surface has
  moved since (`87fe658` `f2a131a` `5fbf762` `2f43cb8` `b2e4cb3`)] P-027 / P-037 (`metric.report.render_ms`, leaf at
  observability.rs:2419) / P-045 have Pulse-side surfaces, but the existing external evidence cannot back them on the
  current binary: a FRESH Conductor round against this chunk's binary is required. A Pulse-side measurement alone stays a
  hollow verified (operator decision 2026-08-21).
- Content fidelity under deterministic L4 — VERIFIED from Conductor's ledger (cited): the one payload-varying read-back
  value is the triggering cue's full-hex fingerprint in `retrieve_telemetry_slice.fingerprint_refs`; every L4-authored
  field is a fixture constant; `incident_events` reaches no MCP tool (a Pulse 0.4.0 residual candidate); runtime-STATE
  fidelity (`mark_incident_resolved` + active-set membership) is the other live-proven axis (Conductor
  `.andromeda/architecture.md:62,93`; `conductor-0.2.0/chunks/2026-08-31-p-075-assert-round`).
- [premise-corrected: Conductor HEAD `1fe46a1` has no route entry and no contract that runs a P-075 round —
  `intent.md:143` calls Pulse's P-075 bookkeeping "a decision in Pulse's ledger, not Conductor work"; its markerless tail
  is the in-flight third real-model series (`working-route.md:75`) and "Version close on measured evidence" (`:77`)]
  `contracts/pulse-capabilities.toml:88` lists `P-075` (verified). The round this entry names needs a Conductor-side owner
  the operator mints, against a Pulse sha this chunk names.
- The round is EXTERNAL and its timing is Conductor's — VERIFIED: the claim is reachable in this chunk only if the round's
  evidence exists before wrap (implement writes `ref` on green); otherwise the cap stays pooled with a `notes` line. A P4
  fork (research.md §Open questions).
- watch: the operator's directive states "Conductor runs its third real-model series in parallel and may launch the
  Pulse binaries" — a shared-host constraint (ports 4317/4318, the build dir), not scope.

## Setup CI verdict (Setup 5a, the last wrap's flip through HEAD)
- `a2addb3` (the wrap commit, = HEAD): **verdict not yet available** — `ci#36908445702` `in_progress` (11 running,
  oldest a11y windows-latest 218 s at read), `secret-scan#36908445677` completed/success. Not folded; re-read before
  any claim that leans on it.

## Boundaries
- Pulse-side only: Conductor's repo is read-only from here; its scenarios, assertions and series are Conductor's work.
- No new capability ids; no spec-source amendments from phase.
- Operator constraints (take-up directive, 2026-10-01): any Pulse build, `pulse-app` run or window is the operator's
  slot — STOP and ask with its length; disk 54 GB < 60 GB, so `cargo clean --profile dev` before heavy builds, with the
  rust-analyzer flycheck stopped by PID (parent `rust-analyzer.exe`; clippy's own `cargo check` child is not it).
