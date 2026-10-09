# The operator pass (plan Step 7) — entries 17 to 22

Run on the operator's word, given in this session after /implement's report (the operator, 2026-10-09):

> Disposition for hygiene: in .andromeda/runs/2026-10-09T14-05-04Z-phase/relay-1.md respell the home prefix of my
> path to a tilde - one anchored edit, nothing else in the line - and say so in evidence/operator-pass.md. Then run
> the operator pass, plan.md Step 7 in order: hygiene, the pre-CI commit, the push, the CI read, the three reads of
> the after-run; fill the After table. Start NO skill after it - the wrap waits for my word.

Each entry was driven once by hand; its `run` is written in the spelling the gate tool prints (the home as `~`).

The tree it ran on: the chunk base `0b61bfb` plus the chunk's edits (2 source files, 53 insertions, 88 deletions).
The gate block before it: 16 green, 0 red, 6 operator legs not run; the scope read `clean — changed 2 · listed 2`.

## The respell, before entry 17

- A preview hygiene read at the end of /implement printed `hygiene: refused 1 files — P1 1`, its one row
  `P1 .andromeda/runs/2026-10-09T14-05-04Z-phase/relay-1.md:1 ×1 · home`: the operator's invocation message, kept
  verbatim by the phase run, names the relay file by an absolute path under the home directory.
- On the word quoted above, one anchored edit of that file, line 1: the home prefix of that one path was replaced
  by `~`, so the path now opens `~/dev/projects/additional/pc-overseer/relays/`. Nothing else in the line or the
  file changed (13 bytes shorter). The file is untracked, so no diff against HEAD shows the edit; this record is
  its account.
- The verbatim copy of the same message, `inputs/I2-relay-1.md.txt`, was not touched: it is a manifest-listed copy
  and keeps the path as given (the hygiene line's `1 host paths kept`).
- The respell was made by hand, not by `gate.py respell`: the row's form was `home`, a path outside the
  repository, which that verb does not rewrite.

## Entry 17 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **exit:** 0 · **atoms:** `exit 0` held · `contains hygiene: clean` held → **green**
- **summary line, as printed:**
  `hygiene: clean — read 41 (runs 34 · evidence 3 · inputs 4) · trails 13 not read · copies 2 not read by P1 — 1 host paths kept · binary 0 not read by P1`
- Read at 2026-10-09T14:36:20Z, after the respell. No row was listed.
- Re-read after this record was first written, before the commit (2026-10-09T14:36:45Z): `hygiene: clean — read 42
  (runs 34 · evidence 4 · inputs 4)`; the scope read: `scope: clean — changed 2 · listed 2 · recorded 0`.
