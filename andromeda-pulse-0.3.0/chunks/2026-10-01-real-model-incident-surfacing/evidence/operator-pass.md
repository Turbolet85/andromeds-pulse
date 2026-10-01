# Operator pass (plan entries 25-28), on the overseer's word 2026-10-01

Fired in plan order by /implement on the operator's explicit instruction ("the OPERATOR PASS on
my word, in plan order"); each entry's exact `run`, its exit and its atoms.

## Entry 25 - `python -X utf8 {tools}/gate.py hygiene`
- exit 0; expect `exit 0` + `contains hygiene: clean` -> held.
- `hygiene: clean — read 31 (runs 26 · evidence 5) · trails 13 not read · binary 0 not read by P1`

## Entry 26 - `cargo xtask pre-push:linux`
- exit 0; expect `exit 0` + `contains "verdict": "green"` -> held.
- head `09d08091d392745c38062f0400c2b4bd32a06179` + working tree, synced tree `ac1f63d7421f9a70dbcbf2fd28911a07fc4c282f`.
- stages, all ok: script-modes 88 ms · **source-lint 7358 ms** · npm 18518 ms · clippy 11762 ms ·
  test 63183 ms · ci-gates 1693 ms; `reason: all-stages-ok`, `missing: []`.
- The new sixth stage ran `cargo xtask check:english-sources` in the distro clone and read clean.

## Pre-CI commit
`chore(2026-10-01-real-model-incident-surfacing): operator pre-CI commit` - the whole tree (`git add -A`).

## Entry 27 - push
See the push reading appended below after it runs.
