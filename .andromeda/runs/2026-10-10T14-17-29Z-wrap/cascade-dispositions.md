# Cascade step 2 — the sweep and its dispositions

The search: `cascade-patterns.toml`, 22 patterns derived after the last of this pass's 58 amendments — the retired
wordings (`not a CLI`, one / single Tauri process or binary, `single-process desktop app`, both shipped / product
binaries, written / published by the app, the app and the sidecar, production binary at boot, `main` parks / hands,
boots no app, that job writes none, the four-target tick roster, exactly once per boot, sole production caller, the
shipped binary, spawn Tauri app, at `pulse-app` boot, the `[[bin]]` target / exemption, first statement / head of
`main()`, the wiring-in-`main.rs` phrasings, the app's `write_pid_file`, wires all library crates), run over the
seven masters, every registry file, the curation homes, the judgment bases and the leaf bodies. Every pattern's
control fired on the pre-pass masters.

Counts, copied from the listing: `total (22 patterns) · 39 rows over 11 files` ·
`per class · new 3/2 · standing 18/4 · leaf 14/6 · curation 4/2 · base 0/0`.

Not looked for: the count `103` and the symbol `heartbeat::spawn` (0 lines in the pre-pass masters, so no control
could fire; the count's one site is the leaf `.claude/rules/testing.md`, re-derived below).

## new (3 rows, 2 files) — this pass's own text
- `.andromeda/architecture.md:211` main-hands · `:225` main-rs-site ·
  `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:46` main-rs-site → no change: the amended
  sentences themselves ("its `main` hands its arguments", "moved out of `main.rs`").

## standing (18 rows, 4 files)
- `.andromeda/test-plan.md:39` tauri-binary (the entity row's title) → amended: the title now reads "binary crate:
  the Tauri window app and the console engine".
- `.andromeda/architecture.md:225` writes-pid ("the app's `write_pid_file` overwrites") → amended: "the engine boot's
  `write_pid_file`".
- `.andromeda/security-plan.md:138` shipped-bin @c10799 (the three L4 path variables "all read by the shipped
  binary") → amended: "the shipped window app `pulse-app` alone (the console program reads none of them)".
- `.andromeda/architecture.md:31`, `:32`, `:33`, `:34`, `:245` shipped-bin ("Never in the shipped binary", "Not read
  by the shipped binary") → no change: a true claim sharing the token — a dev tool or a build input that reaches no
  product binary, whichever it is.
- `.andromeda/security-plan.md:396` ×3 and `:398` shipped-bin ("never reaches the shipped binary") → no change: the
  harness carve-out's own statements, true of every product binary.
- `.andromeda/obs-plan.md:145` tick-roster ("module boundaries (ingest, buffer, viz, plugins, snapshot)") → no
  change: a module list, not the tick roster.
- `.andromeda/obs-plan.md:473` once-per-boot · `.andromeda/test-plan.md:141` sole-caller → no change: this pass's
  amended lines, the token standing inside the new sentence ("once per boot of the window app"; "their sole
  production caller — `engine_boot::start`").
- `.andromeda/obs-plan.md:329` main-first ("the decision is taken at the head of `main()`") → no change: the line is
  amended to "once per window-app boot" and the clause is true of the window app's `main`.
- `.andromeda/test-plan.md:151` main-first ("requires the first statement of `main()`") → no change: the row is the
  window entry's pin (`unit_xlib_threads`), and its appended narrowing names the window.
- `.andromeda/test-plan.md:147` bin-target ("`main.rs` exempt: the `[[bin]]` target's tests genuinely execute") → no
  change: a discharged row's history, and true of `main.rs`, which is a `[[bin]]` target.
- `.andromeda/test-plan.md:149` ×2 and `:150` main-rs-site → no change: the two rows keep their dated original
  statement and each now ends with a `Narrowed 2026-10-10-console-engine-entry-point` clause that says where the
  composition sits.

## leaf (14 rows, 6 files) — re-derived at the passages the amended sections feed
- `CLAUDE.md:13`, `:98` tauri-binary → re-derived (the key-directories line, the Architecture paragraph), with the
  `pulse-app` module bullet, from architecture §Design Philosophy, §Occupied Resources and the directory-structure key.
- `.claude/docs/stack.md:50` single-proc → re-derived from the Stack row. `:10`, `:11`, `:12` shipped-bin → no
  change (the same true claims as architecture `:31`–`:34`).
- `.claude/rules/security.md:17` both-bins → re-derived ("all three product binaries"), and the L4 path sentence of
  the same line scoped to the window app. Its two shipped-bin matches → no change (the carve-out's statements).
- `.claude/rules/observability.md:67`, `:70` once-per-boot → re-derived ("once per window-app boot"); `:67`
  main-first → no change (true of the window app's `main`). The same file gained the `app.boot.engine` bullet, the
  process-start clause of the init-order bullet and the per-program tick sentence, from obs-plan §6, §8 and the two
  key files.
- `.claude/docs/obs-summary.md:17` main-hands → re-derived; `:135-136` main-first → no change.
- `.claude/rules/testing.md:25` bin-target → re-derived with the count (103 → 109).
- Limit, stated: each leaf was re-derived at the passages this pass's amended sections feed, located by this sweep
  and by reading the leaf's matching bullets. No leaf was recomputed whole from its master in this pass, and
  `.claude/docs/{security,tests}-summary.md`, `docs/commands.md`, `docs/gotchas.md` and `docs/services/*.md` were
  not opened. This is carried in the handoff.

## curation (4 rows, 2 files) — preserve-verbatim homes, never edited by the cascade
- `.claude/rules/testing.md:161`, `:165` tauri-binary · `:185` bin-target · `.claude/rules/security.md:118`
  main-rs-site → P3: each is a dated Session Additions entry about the crate's test layout or the bindings regen;
  none states a claim this pass retires as current (the `:118` entry's "production `main.rs` setup closure" names
  where `Router::into_handler()` runs, which is still `main.rs`).

## base (0 rows)
- none.
