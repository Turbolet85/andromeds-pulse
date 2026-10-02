# Fan-out results — 2026-10-01-conductor-return

Seven doc-agents (Explore), one parallel batch, prompt sent verbatim from amendment-flow.md; `{contracts_line}` dropped
for all four keyed-contract docs (`registry.py contracts` → exit 3 `NOT MIGRATED` for architecture · test-plan · obs-plan
· a11y-plan). Returns were plain YAML; stripping removed only `#`-comment rationale lines (summarised per verdict below);
no HTML entities present (probe: `entities=0` by read); no raw twin warranted (no `proposals: []` return was changed by
stripping in substance, none failed the parse).

## Verdicts
- **architecture** — `proposals: []`. Stripped notes: D-arch-resources — no IPC/endpoint/event/port/env var/crate added;
  `app.exit` is an obs allowlist item, not an arch-registered resource class. D-arch-decisions — `libc` direct dep is
  below §Stack's grain (one technology per layer); no locked decision contradicted (tray policy, Self-Observation file
  sink, `anyhow` at `main.rs` intact).
- **security-plan** — 2 proposals (below). Stripped notes: D-security-input / -auth / -deps no drift (`libc` already in
  the lockfile, deny + audit green, no ban touches it).
- **design-system** — `proposals: []`. No new UI; all disproved claims are chunk-artifact claims; Tray "Quit" status
  (`design-system.md:328`) still holds.
- **layout-templates** — `proposals: []`. No new surface; Tray "Quit terminates the process" (`:280`, `:321`) and
  close-to-tray (`:256`, `:341`) still hold.
- **test-plan** — 2 proposals (below).
- **obs-plan** — 5 proposals (one primary + four `dependent-of`). Stripped notes: D-obs-stack no drift (`libc` is FFI, not
  a logger); D-obs-pii no drift (closed four fields; the leaf registration filed under D-obs-instrumentation);
  D-obs-defect-narrative no drift (§10 holds no exit narrative).
- **a11y-plan** — `proposals: []`. No interactive UI; no a11y/obs schema change (a new target + leaf does not change the
  record envelope).

## Proposals + dispositions

### security-plan
1. D-security-logging · §Security Anti-Patterns → Logging — add `app.exit` as a deliberate NO-SCRUB exact-leaf log
   boundary (one record per process end; the closed four fields; levels; no bare `app` key; the 0-path/0-canary witness;
   the unloggable-by-construction ends as its stated failure mode).
   **Disposition: APPLY** — playbook "Accurate this-chunk addition" (routine): `app.exit` is in the report's Changes
   (Schema / config); invariant holds. Check 5: the plan's expected amendment (security Logging) — matched. Check 4: the
   absence claim (0 `app.exit` / `atexit` hits) re-derived: `grep -c 'atexit\|app.exit' security-plan.md` → 0 (report).
2. D-security-logging · §Security Anti-Patterns → Universal — add the ban: never log from an `atexit` / signal handler on
   the exiting thread (glibc TLS destructors run first; a `LocalKey::with` panic in an `extern "C"` handler aborts) —
   hand off to a thread spawned at install; a signal handler that records re-raises the SAME signal with the default
   disposition.
   **Disposition: APPLY** — playbook "Accurate this-chunk addition" (routine): the mechanism is the report's (Symbols +
   Decisions); check 5: the plan's expected amendment (security Universal — the FFI surface) — matched. Not "Boundary
   widening": no data crosses a hardened boundary; this records a new process-lifecycle FFI surface and its ban.

### obs-plan
3. D-obs-instrumentation · §6 Log Coverage (`warn` row) — register `app.exit` (fields, levels INFO/WARN/ERROR by class,
   exactly once per process end, emitters, loggable vs unloggable ends).
   **Disposition: APPLY** — playbook D-obs-pii new-exact-leaf rule (routine; both load-bearing conditions hold: the leaf
   enumerates all four emitted fields, and the guard runs in `pulse-app/tests/`) + "Accurate this-chunk addition".
   Re-derived placement: the `warn` row, levels named per class (the row already registers a mixed INFO/WARN target,
   `app.boot.window.navigation`). Check 5: expected amendment obs §6 — matched.
4. (dependent-of 3) §8 PII Scrubbing leaf list — the `app.exit` exact leaf beside `app.boot.window.navigation`.
   **Disposition: APPLY** (atomic with 3). Check 5: expected amendment obs §8 — matched (dual-site).
5. (dependent-of 3) §7 Error Capture & Reporting → Error classes captured — add the process-end cause class.
   **Disposition: APPLY** (atomic with 3), re-derived: the main-thread-panic follow-on record stated as NOT measured
   (by construction only), per the report.
6. (dependent-of 3) §3 Observability Harness Contract → Tracing init (Init order) — extend the order with guard holding,
   the exit hook, the Unix signal listener and the `run_return` → `exit_after_event_loop` tail.
   **Disposition: APPLY** (atomic with 3). Bind check: test-plan §3 ↔ obs-plan §3 — the 5-command harness, status shape
   and log format are unchanged; the init-order steps are obs-side only → no one-sided change.
   **Kept:** the step-(2) "stderr layer + file appender" clause is NOT touched — a pre-existing drift (HEAD builds the
   file layer only; research.md §Files inspected) that the plan routes to the wrap's curation channel; not this
   chunk's (playbook "Not this chunk's drift" → its owned channel, route-resolve CARRY — see P5).
7. (dependent-of 3) §1 Obs Scope Summary → Tracing init — the same init-order extension (a restatement of §3).
   **Disposition: APPLY** (atomic with 3); same Kept note as 6.

### test-plan
8. D-tests-coverage · §1 Pending coverage triggers — new trigger `exit-hook-main-composition-coverage` (the `main.rs`
   composition — guard holding, hook + listener install, the `run_return` tail — has no committed test; the live
   event-loop exit is not inducible headless; the main-thread panic → `app.exit` ordering is unmeasured).
   **Disposition: APPLY** — playbook "Accurate this-chunk addition" (routine); check 5: expected amendment test §1 —
   matched.
9. D-tests-coverage · §3 Per-chunk gate discipline — record the process-end witness form (re-exec children, init-record
   child-ran proof, return-value pin for a once-flag whose only drain is a guard drop, `cfg(unix)` arms on the Linux
   legs, the unloggable ends it cannot reach).
   **Disposition: APPLY** — routine (accurate this-chunk addition); check 5: expected amendment test §3 — matched.

## Validate summary
- Check 1 (playbook): 9/9 routine — no escalation. Check 2 (cross-contradiction): none (no two proposals edit one
  section in opposing directions). Check 3 (intent-consistency): the report matches the working-route intent (PREREQ +
  P-075); scope record: none (`gate.py scope` clean, 0 recorded) — nothing to judge. Check 4 (absence needs evidence):
  the three absence claims (`app.exit`/`atexit` 0 in security-plan; `app.exit` 0 in obs-plan; no §3 witness form in
  test-plan) re-derived by grep in the report and at `obs-plan.md` (0 `app.exit` hits) — lines read by offset where
  over 2 000 chars (obs `:418` is 5110 c; the `app.boot.window.navigation` clause read by match window). Check 5
  (expected amendments): obs §6+§8 → 3/4 · security Logging+Universal → 1/2 · test §1+§3 → 8/9 · arch `libc` → not
  carried (arch enumerates no direct dependency; report) — floor met. Check 6 (disproved claims): all three are
  CHUNK-ARTIFACT claims (research.md Mechanism 3; plan Step 8 mutation line; plan Step 8 arm (b) premise) — DISPOSED:
  recorded in the report, no amendment owed (research/plan have no sanctioned writer at wrap); the durable lessons ride
  test-plan §3 (proposal 9), security Universal (proposal 2) and P3 curation.
- Escalations: 0.
