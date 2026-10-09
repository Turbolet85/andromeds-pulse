sweep: first sweep: "yes, as read. I re-read all six rows and the two unlisted ps1 numbers at 569604b and they stand as you say: write the three rows that hold, withhold the three refused, re-point by hand agent-run.sh:29 and :30, xtask/src/main.rs:306, agent-run.ps1:24 and :25" — the operator, 2026-10-09 · withheld 3

# First-sweep read — the 2026-10-09T15-11-06Z wrap of 2026-10-09-ci-on-linux-alone

Written before the ask. Nothing is written to a master by this file's existence: `cites.py apply --dry-run` exited 3
(`no earlier sweep`), then `cites.py map` ran read-only at `569604b` (`cites v1.3`, `base: blame — no earlier sweep;
each citing line's own last commit`). Line 1 was added on the operator's word, which moved no row: the three refused
rows are withheld and the three that hold are written, as read.

The tool's summary lines, as printed:

```
sites 84 (bare ":digits" 431) over 72 file(s) · unmoved 40 · moved 6 · stretched 5 · changed 6
out-of-range 2 · unresolved 17 · new 0 · repointed 0 · hand 0 · held 0 · INDETERMINATE 0 · listed, not written 13
the write, by blame: 6 citation(s) — the next 6 row(s), each with the line its new number names
```

## The block — each row read at its citing line and at the line its new number names

- `moved .andromeda/architecture.md:242 scripts/agent-run.sh:22 → 24 «export PYTHONIOENCODING=utf-8»` → refused: the
  sentence cites where the harness reads `ANDROMEDA_PULSE_PIDFILE`. Line 24 is the `PYTHONIOENCODING` export and line
  22 a comment line; the read stands at `scripts/agent-run.sh:29` (`PIDFILE="${ANDROMEDA_PULSE_PIDFILE:-…}"`). The
  number was already stale when its citing line was last committed.
- `moved .andromeda/architecture.md:243 scripts/agent-run.sh:23 → 25 «»` → refused: the sentence cites where the
  harness reads `ANDROMEDA_PULSE_LOGFILE`. Line 25 is blank and line 23 the `PYTHONUTF8` export; the read stands at
  `scripts/agent-run.sh:30` (`LOGFILE="${ANDROMEDA_PULSE_LOGFILE:-…}"`).
- `moved .andromeda/architecture.md:244 scripts/agent-run.sh:99 → 230 «if [ -z "${ANDROMEDA_PULSE_DATA_DIR_KEEP:-}" ] && …»`
  → holds: line 230 is the `ANDROMEDA_PULSE_DATA_DIR_KEEP` test that guards the tempdir removal.
- `moved .andromeda/architecture.md:244 scripts/agent-run.ps1:71 → 100 «if (-not $env:ANDROMEDA_PULSE_DATA_DIR_KEEP -and …»`
  → holds: line 100 is the same test in the PowerShell half.
- `moved .andromeda/test-plan.md:493 xtask/src/main.rs:174 → 207 «#[command(»` → refused: the sentence says the cited
  line "delegates to `npm run test:a11y --prefix pulse-app/ui`". Line 207 opens the `perf:slo-load` command
  attribute. The delegation stands at `xtask/src/main.rs:306`
  (`Cmd::TestA11y { extra } => run_npm_script("test:a11y", extra).await`); the command's declaration, whose `about`
  names the npm script, is at `:159-162`.
- `moved .andromeda/obs-plan.md:568 crates/buffer/src/retention.rs:72-86 → 69-83 «let sweep_conn: Arc<Mutex<Connection>> …»`
  → holds: lines 69-82 are the `sweep_conn` block that clones the connection (`try_clone()` at line 70); line 83 is
  blank.

Three rows refused, three hold. The write withholds the three refused rows unless the operator's word says otherwise.

## Wrong numbers on the citing lines this read opened, with no row printed

- `.andromeda/architecture.md:242` `scripts/agent-run.ps1:15` — line 15 is a comment; the `ANDROMEDA_PULSE_PIDFILE`
  read is at `scripts/agent-run.ps1:24`.
- `.andromeda/architecture.md:243` `scripts/agent-run.ps1:16` — line 16 is a comment; the `ANDROMEDA_PULSE_LOGFILE`
  read is at `scripts/agent-run.ps1:25`.

By the letter each is re-pointed by hand, the digits alone, to the line read, before the write; it then prints
`hand`. Not re-pointed yet: no hand edit is made before the operator's word.

## Against the earlier reading

`.andromeda/runs/2026-10-09T08-07-22Z-wrap/first-sweep-read.md` read the same six rows at `18a872d` and reached the
same six verdicts and the same two unlisted numbers. This read was made again at `569604b`, from the files; it
agrees on every line number (`agent-run.sh:29` / `:30`, `agent-run.ps1:24` / `:25`, `xtask/src/main.rs:306`). One
difference: that read placed the `test:a11y` declaration at `:160-161`; the attribute spans `:159-162`.

## Rows outside the block (dispositioned after the write, not here)

- Masters, 2 rows: `changed .andromeda/test-plan.md:221@c941 crates/buffer/src/schema.rs:320-323 → ?` (both ends
  rewritten) and `moved .andromeda/obs-plan.md:453@c680 observability.rs:1934 → 1971` (listed, not written: `suffix`).
- Leaves, 19 rows that are not `unresolved`, across `.claude/rules/` (7) and `.claude/docs/` (12).
- `unresolved`, 17 rows: 4 in masters (`test-plan.md:129`, `:136`; `a11y-plan.md:224`, `:424`), 12 in leaves, 1 in
  the route (`working-route.md:124`). `apply` prints their count, and no disposition is owed on them.
