# The operator pass (plan Step 13) — entries 20, 21, 22

Run on the overseer's go (founder-delegated, 2026-10-07; inputs#I11), after the reading and the regression guard's
disposition (HOLDS, nothing changed). Each entry was driven once by hand; its `run` is written in the spelling the
gate tool prints (the home as `~`).

## Entry 20 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **exit:** 0 · **atoms:** `exit 0` held · `contains hygiene: clean` held → **green**
- **summary line, as printed:**
  `hygiene: clean — read 53 (runs 37 · evidence 7 · inputs 9) · trails 14 not read · copies 7 not read by P1 — 0 host paths kept · binary 0 not read by P1`
- Read at 2026-10-07T06:16:36Z. No committed capture needed a placeholder: no row was listed.
