# The operator pass (plan Step 17) — entries 29, 30, 31

Run on the operator's explicit word, given in the session that swapped the product change to `CX` (2026-10-07,
after that run's report: "Operator pass: go, on my explicit word. Run entry 29 (hygiene), entry 30 (the pre-CI
commit, then the push) and entry 31 (the CI read), then stop before the wrap."). Each entry was driven once by hand;
its `run` is written in the spelling the gate tool prints (the home as `~`).

The tree it ran on: the chunk base `48714f0` plus the chunk's edits, `CX` in the product
(`evidence/remedy-shipped.md`). The gate block before it: 19 green, 0 red, 12 operator legs not run, and the eight
tree-reading entries green again once the last evidence file was written (2026-10-07T18:36:57Z).

## The capture-text read, once more before the commit

- Fired by hand at 2026-10-07T18:38:16Z, the run `evidence/capture-text-read.md` records (three captured prompts, 9
  distinct corpus lines): 0 files, exit 1, both atoms held → **green**. No commit carries a captured line.

## Entry 29 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **exit:** 0 · **atoms:** `exit 0` held · `contains hygiene: clean` held → **green**
- **summary line, as printed:**
  `hygiene: clean — read 98 (runs 61 · evidence 15 · inputs 22) · trails 30 not read · copies 20 not read by P1 — 0 host paths kept · binary 0 not read by P1`
- Read at 2026-10-07T18:38:16Z. No committed capture needed a placeholder: no row was listed.
