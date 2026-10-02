# Mutation checks — 2026-10-01-conductor-return

One-shot controls (plan Steps 7-8), Windows dev host, operator slot #2a, 2026-10-02. Each mutation was applied with the
anchored Edit tool, the two new test targets rebuilt and run
(`cargo nextest run -p pulse-app --profile ci --no-fail-fast --test integration_exit_cause_record --test
unit_observability_allowlist_app_exit`), then restored. After the last restore, `pulse-app/src/observability.rs` carries
no mutation residue (grep for the mutation markers: 0) and its diff is the 221-line addition only.

| # | mutation | expected red | measured |
|---|---|---|---|
| 1 | `app.exit` leaf deleted | the exact-resolve pin | RED: `app_exit_resolves_to_an_exact_leaf`, the field-set and banned-field pins, and all four integration arms (fields render `<redacted>`). GREEN: `app_exit_has_no_bare_app_fallback` (correct — it guards the absence of a bare key) |
| 2 | leaf narrowed by one field (`signal` dropped) | the field-set equality pin | RED: `app_exit_leaf_carries_exactly_the_emitted_field_set` + the four integration arms. GREEN: `app_exit_resolves_to_an_exact_leaf` — the narrowed-leaf asymmetry (observability.md 2026-08-26): only the set pin carries this guard |
| 3 | flush neutralized (`flush_log_sink` returns before dropping the guard) | arm (a) or (c) | RED: both arm-(a) children (`…nonzero_code_is_carried_at_error`, `…zero_code_is_recorded_at_info`). GREEN: arm (c) — the at-exit hand-off waits for the reporter, which gives the worker time to write before the C runtime exits; arm (e) — the record happened to land in that run |
| 4 | once-guard neutralized | arm (e) | see below |

## Mutation 4 — the plan's arm (e) cannot discriminate the once-flag (measured); arm (e2) added

- First form (only the `swap` in `record_exit` neutralized): all green, (e) included. Two reasons, both structural:
  (i) on Windows the Step 4 tail ends in `std::process::exit` = `ExitProcess`, which never reaches the at-exit hook
  (red-probe.md), so no second emitter exists on this host; (ii) the handler's own `EXIT_RECORDED.load` fast path still
  returned early.
- Second form (the `swap` AND the handler's `load` neutralized) with an added arm (e2) — an event-loop record, then
  `libc::exit(3)`, which reaches the hook on every OS (on Unix this is exactly the Step 4 tail): still all green.
  Mechanism: the first `record_exit` FLUSHES, and the flush drops the only `WorkerGuard`, shutting the worker down, so a
  second record can never reach the file. "Exactly one `app.exit` in the file" therefore holds by construction of the
  single drain, independent of the flag. The flag guards a different property: two emitters that both write before
  either flushes (a concurrent race), which no file read can observe.
- Third form: (e2) also reads `record_exit`'s return value in the child (first call `true`, a second call must return
  `false`) and reports it through the exit code (9 = a second emitter let through). Under the second-form mutation:
  **RED** — `exit_cause_at_exit_hook_does_not_duplicate_a_record_before_a_native_exit`, `left: Some(9)` vs
  `right: Some(3)`. Restored: green.
- Consequence for the plan's mutation line "neutralize the once-flag → (e) reddens": it does not hold as written (a
  premise of the plan, not of the product). (e) stays as the literal Step 4 tail witness; (e2) carries the once-flag
  guard on every OS.

## The re-exec children ran (testing.md 2026-08-15)

Every arm asserts its child's `app.boot.tracing.init` record is present (exactly 1) before any other assertion, so a
child that returned early fails the arm. That replaces the one-time `--no-capture` read: the children's stdout is
discarded by design (`Stdio::null()`), so a `--no-capture` run would show nothing from them.
