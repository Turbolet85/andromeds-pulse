# First-sweep read — the 2026-10-09T08-07-22Z 0-pending wrap

Nothing is written by this file's existence. `cites.py map` ran read-only at `18a872d` (`cites v1.3`, `base: blame — no
earlier sweep`); no `apply` was fired. `citation-contract.md` §What is swept says the 0-pending path runs no sweep, so
whether a first sweep is written at this wrap is the operator's word, asked on the card.

The tool's summary lines, as printed:

```
sites 85 (bare ":digits" 437) over 72 file(s) · unmoved 41 · moved 6 · stretched 5 · changed 6
out-of-range 2 · unresolved 16 · new 0 · repointed 0 · hand 0 · held 0 · INDETERMINATE 0 · listed, not written 14
the write, by blame: 6 citation(s) — the next 6 row(s), each with the line its new number names
```

## The block, each row read at its citing line and at the line its new number names

- `moved .andromeda/architecture.md:242 scripts/agent-run.sh:22 → 24 «export PYTHONIOENCODING=utf-8»` → refused: the
  sentence cites the line where the harness reads `ANDROMEDA_PULSE_PIDFILE`; line 24 is the `PYTHONIOENCODING` export
  and line 22 a comment. The read stands at `scripts/agent-run.sh:29` (`PIDFILE="${ANDROMEDA_PULSE_PIDFILE:-…}"`). The
  number was already stale when its citing line was last committed.
- `moved .andromeda/architecture.md:243 scripts/agent-run.sh:23 → 25 «»` → refused: the sentence cites the line where
  the harness reads `ANDROMEDA_PULSE_LOGFILE`; line 25 is blank and line 23 the `PYTHONUTF8` export. The read stands
  at `scripts/agent-run.sh:30` (`LOGFILE="${ANDROMEDA_PULSE_LOGFILE:-…}"`).
- `moved .andromeda/architecture.md:244 scripts/agent-run.sh:99 → 230 «if [ -z "${ANDROMEDA_PULSE_DATA_DIR_KEEP:-}" ] && …»`
  → holds: line 230 is the `ANDROMEDA_PULSE_DATA_DIR_KEEP` test guarding the tempdir removal.
- `moved .andromeda/architecture.md:244 scripts/agent-run.ps1:71 → 100 «if (-not $env:ANDROMEDA_PULSE_DATA_DIR_KEEP -and …»`
  → holds: line 100 is the same test in the PowerShell half.
- `moved .andromeda/test-plan.md:493 xtask/src/main.rs:174 → 207 «#[command(»` → refused: the sentence says the cited
  line "delegates to `npm run test:a11y --prefix pulse-app/ui`"; line 207 opens the `perf:slo-load` command attribute.
  The delegation stands at `xtask/src/main.rs:306` (`Cmd::TestA11y { extra } => run_npm_script("test:a11y", extra).await`);
  the command's declaration, whose `about` names the npm script, is at `:160-161`.
- `moved .andromeda/obs-plan.md:568 crates/buffer/src/retention.rs:72-86 → 69-83 «let sweep_conn: Arc<Mutex<Connection>> …»`
  → holds: lines 69-82 are the `sweep_conn` block that clones the connection (`try_clone()` at line 70), line 83 blank.

## Wrong numbers on the citing lines this read opened, with no row printed

- `.andromeda/architecture.md:242` `scripts/agent-run.ps1:15` — line 15 is a comment; the `ANDROMEDA_PULSE_PIDFILE`
  read is at `scripts/agent-run.ps1:24`.
- `.andromeda/architecture.md:243` `scripts/agent-run.ps1:16` — line 16 is a comment; the `ANDROMEDA_PULSE_LOGFILE`
  read is at `scripts/agent-run.ps1:25`.

Not re-pointed: no hand edit is made before the operator's word on the sweep.

## Against the relayed hypothesis

The pc overseer's relay (`pc-overseer/relays/pulse-wrap-0pending-close-2026-10-09.md` §7) carried the main overseer's
reading from a scratch clone at `18a872d`: three rows wrong before and after the map (`architecture.md:242`, `:243`,
`test-plan.md:493`) and the two `agent-run.ps1` neighbours wrong and unlisted. This read agrees on all five; it adds
where each fact stands now (`agent-run.sh:29` / `:30`, `agent-run.ps1:24` / `:25`, `xtask/src/main.rs:306`).

## Rows outside the block (not read here)

One master row a written sweep would owe a disposition on: `changed .andromeda/test-plan.md:221@c941
crates/buffer/src/schema.rs:320-323 → ?`. One master row listed and not written: `moved .andromeda/obs-plan.md:453@c680
observability.rs:1934 → 1971` (`suffix`). The leaf rows (19 across `.claude/rules/` and `.claude/docs/`), the route row
(`working-route.md:186`, an entry this wrap retires) and the 16 `unresolved` rows are in the trail,
`cites-no-marker.json`.
