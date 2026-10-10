# Route-resolve card — 2026-10-10-console-engine-entry-point (stop 1 of 2)

State at the card: P1 report written · P2 applied (58 amendments in 4 masters, 4 sidecar entries, citations
0 re-pointed · 2 changed · 0 stretched, no `held` row, no `REFUSED id:` in any trail) · P3 one Tier-2 entry · P4
code-graph fresh (rust 10240n/49688e · ts 4164n/7755e). Nothing below is written to the route yet.

## A. Carries the directive names (inputs#I4 item 4 and 5) — proposed text, each on its owner
1. `Incident is the engine's own record` (working-route.md:65) —
   `CARRY: the console program seats no interpretation runner when ANDROMEDA_PULSE_L4_DETERMINISTIC is unset: cues and digests are produced, no incident forms, and app.boot.engine says so at WARN (interpretation none, reason deterministic_gate_unset); this entry gives the console engine an incident of its own, and the unset arm's test (integration_engine_boot, an_empty_seat_archives_the_digest_and_forms_no_incident) is re-pointed here (record: 2026-10-10-console-engine-entry-point)`
2. `Desktop distribution retired` (:52) —
   `CARRY: what tauri build does with the pulse-app package's second [[bin]] (andromeda-pulse-engine) is not measured — no step runs the release workflow; the bundler may place the console binary in the desktop bundles (record: 2026-10-10-console-engine-entry-point)`
3. `One place on a node` (:39) —
   `CARRY: the console program ends by SIGTERM or SIGINT only — no stop command exists, and finding a running engine needs the pid file, whose place is this entry's; both programs write run/andromeda-pulse.pid and run/workspace-key at the same paths (record: 2026-10-10-console-engine-entry-point)`
4. `Agent harness drives the console engine` (:31) —
   `CARRY: the console program's panic record and at-exit record are witnessed one level down, in re-exec children of engine_boot::init_process (integration_engine_boot), never on the console program itself, which is ended only by SIGTERM and SIGINT in its tests; the harness verbs that read run/andromeda-pulse.pid are not aimed at it although it writes that file (record: 2026-10-10-console-engine-entry-point)`
5. `Window retired` (:50) —
   `CARRY: the shared engine boot's second caller leaves here: pulse-app/tests/unit_engine_boot_seam.rs pins two entry points to one boot (its window half reads main.rs) and unit_xlib_threads pins main's first statements — when main.rs leaves, both pins lose their subject and are removed or re-based with it, and Program::Window, SeatKind::Model and spawn_window_ticks go the same way (record: 2026-10-10-console-engine-entry-point)`

## B. Found and not owned — each with a lean
6. Both programs default to the same data dir, the same two ports and the same pid file path, and nothing refuses a
   second engine on a data dir (the second records both binds failed, overwrites the pid file, opens the same
   corpus; not exercised by a test). Lean: fold into carry 3 on `One place on a node`.
7. Three readings the new boot's tests do not make (the rejected-port arm of `engine_boot::start`, a socket census,
   the lock file's directory) — recorded as test-plan §1 `engine-boot-rejected-port-and-socket-census-coverage`.
   Lean: a `CARRY:` on `Agent harness drives the console engine`, naming that row.
8. The dead-test ratchet reads the top level of `pulse-app/src/` only; `src/bin/` is outside it. Nothing is dead
   there (a `[[bin]]`'s tests run). Lean: drop.
9. On the runner the boot record carries `deployment.environment: dev` and the pull-request merge sha. Read, not
   changed, not this chunk's. Lean: drop.
10. Leaves were re-derived at the passages the 58 amendments feed, not recomputed whole (the 85 % context alarm);
    `docs/{security,tests}-summary.md`, `docs/commands.md`, `docs/gotchas.md`, `docs/services/*.md` were not opened.
    Lean: carried in the handoff as owed to the next wrap's cascade; no route pin.
11. Seven stale line citations in preserve-verbatim homes (`rules/observability.md:148`,
    `docs/session-learnings.md:877`, `:1596`, `:1841`, `:1861`). Lean: surfaced in the handoff, not corrected, as at
    the last wrap.

## C. Judgment base — one rule proposed (the P2 collision's discriminator)
12. `- pattern: Product command-line word registration — a detector flags a command word of a product binary absent
    from security-plan §Input Validation / §Threat Model Summary, AND the report shows the word is a closed member of
    a code-validated, unit-tested grammar that takes no value, no path and no network address.`
    `verdict: routine`
    `note: registry completeness, not a boundary widening — P-086 rules a console program driven by commands, so a
    command line is the ruled form (the operator, the pc overseer, 2026-10-10, at the console-engine-entry-point
    wrap). A command that takes a value, a path or a network address is a new question each time: that is the
    Boundary widening rule's, and escalates.`

## D. The P-086 ledger note (directive item 2; written at P7.3)
13. `2026-10-10 (chunk 2026-10-10-console-engine-entry-point; advanced, not claimed): the first half of the
    acceptance is shown — the engine starts as a console program (andromeda-pulse-engine run) with no display
    variable, no session bus, no GPU probe and no model runner, stays up, takes telemetry over its gRPC receiver and
    answers run and version (integration_console_engine, green in lint / test of ci#38056942758). Not shown, and
    carried by the entries that name P-086: runs in the background as a service; every kind of finding 0.3.0
    detected; one incident for a sustained storm; automatic resolution on the same telemetry (Agent harness drives
    the console engine · Scenario legs driven against the console engine · Engine end-to-end gate reachable ·
    Detection baseline through the console engine, and the route's later P-086 entry). On the operator's wrap
    directive (the pc overseer, 2026-10-10, relayed in a file).`

## E. Surfaced, no edit
14. Epoch 1 holds 18 entries (8 markerless, 9 complete, 1 pending) — over the ten at which growth is surfaced; no
    split proposed (your word at the last wrap).
15. No `gated` record, no `BLOCKED-ON`, no `PREREQ`, no `WATCH` on the tail; no gate was deferred.
16. For the main overseer: no `REFUSED id:` in any trail of this session; the citation sweep read `held 0`.
