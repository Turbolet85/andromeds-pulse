# The operator pass (plan entries 25 to 32), 2026-10-10

Run by the agent on the operator's word, given in this session after the implement report: "implement report read
against the tree (9 files; ci.yml reads 0 downloads, 0 soft-fail keys, 6 uploads each failing on no file).
Deviations accepted as recorded - no dead arm is kept for a plan sentence; the report says which form of the fourth
mutation reads red. Run the operator pass, entries 25 to 32 in the plan order: hygiene, the pre-CI commit, the
pre-push check on the committed tree, the push, the CI verdict, then the five reads by run id. A red boot job is now
a NEW reading: stop and report its per-boot verdicts and witness lines whole; fix nothing on top. Start no skill -
the wrap is mine to call." — the operator, 2026-10-10.

Each operator entry was driven once, by hand, in the spelling the plan lists. Raw outputs are not kept; the lines
quoted are the tools' own verdict lines.

## Entry 25 — hygiene, 2026-10-10T07:29:10Z

- Run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- Exit 0. `gate v1.13 · 3718c868` · `base HEAD (no --marker)`.
- Summary line: `hygiene: clean — read 43 (runs 36 · evidence 3 · inputs 4) · trails 13 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1`
- Atoms: `exit 0` held; `contains hygiene: clean` held. **Green.**
- This record was written after that read. The verb was read once more after it, as a check of this file and not
  as the entry; that reading is the first line of the next section.

Everything below this line was written after the build-branch push, so it is in neither the pre-CI commit's tree nor
the pushed tip's.
