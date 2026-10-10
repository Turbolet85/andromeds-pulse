# The operator edit of the plan, between runs (2026-10-10)

**Whose word.** The operator, 2026-10-10, given in the implement session after its report and kept verbatim as
inputs#I9: "two corrections of the plan between runs, made by you as my hands and recorded as the operator edit
in evidence." The edits below were typed by the implement agent on that word; /implement edits no plan on its own.
Recorded 2026-10-10T11:45:45Z.

## Correction 1 — entry 5's name atom

The word: "plan.md entry 5: the name atom becomes contains lint-test/cargo xtask verify:capability-matrix (chunk -
the parsed name ends there."

`plan.md:212`, the third atom of `expect`:

- before: `'contains lint-test/cargo xtask verify:capability-matrix (chunk #99 — P-001..P-060 scenario mapping)'`
- after: `'contains lint-test/cargo xtask verify:capability-matrix (chunk'`

Why the old atom could not hold: the base step's name in `.github/workflows/ci.yml` at `279a477e` is an unquoted
YAML scalar, and YAML reads the text after a space and a hash as a comment. `yaml.safe_load` of the base file gives
the step the name `cargo xtask verify:capability-matrix (chunk`. The first firing read exit 0 and last line
`1 True True True -`, both held, and failed on this atom alone.

## Correction 2 — one rule for the guard word: P-076 is `part`

The word: "P-076 has pins that run under nextest, as P-078 has, so it is part, not none; correct the record, the
plan table row and the atom of the probe that counts guard states to what the record then reads."

- **The record**, `docs/capability-record.json`, entry P-076 and no other entry (compared through
  `python -m json.tool`, before against after):
  - `guard.state`: `none` → `part`
  - `guard.by`: `[]` → `["xtask/src/webview_drive.rs"]`
  - `guard.unrun`: `["cargo xtask webview-drive"]`, unchanged
  - `note`: reworded to "The pins in xtask/src/webview_drive.rs hold the drive's stage predicates and run under
    the workspace test job; the drive of the assembled app has no CI caller."
  - The pins that run: 54 tests under `xtask::bin/xtask webview_drive::` read PASS in the first firing of entry 16
    (the 0.3.0 ledger's ref counted 27 when it was written); `self_verify::` holds 8, the figure P-078's ref gives.
- **The plan table row**, `plan.md:137`: `P-076 [none], P-078 [part]` → `P-076 [part], P-078 [part]`.
- **The atom of the probe that counts guard states**, `plan.md:244`: `'last line 7 14 25 46 4 36'` →
  `'last line 6 15 25 46 4 36'`, which is what the record reads after the correction.

## The entries re-fired after the edits

One call, `gate.py run … --only 5,8,9,15,16,17,18`: the two edited entries (5, 9), the other readers of the changed
record (8, 15, and 16 for the pin that reads the committed record), and the bindings regen and close that follow a
workspace run (17, 18). Entry lines and the summary line, verbatim:

```
  5 probe       green · exit 0 · 0.07s · 73 B → 5.2.log · python -X utf8 -c "import subprocess, yaml; b = yaml.safe_… (797 chars)
  8 probe       green · exit 0 · 0.02s · 19 B → 8.2.log · python -X utf8 -c "import json; c = json.load(open('docs/c… (370 chars)
  9 probe       green · exit 0 · 0.02s · 16 B → 9.2.log · python -X utf8 -c "import json; c = json.load(open('docs/c… (456 chars)
 15 probe       green · exit 0 · 0.33s · 447 B → 15.2.log · cargo xtask verify:capability-matrix
 16 unit        green · exit 0 · 14.77s · 708805 B → 16.2.log · cargo nextest run --workspace --profile ci
 17 build       green · exit 0 · 2.91s · 513 B → 17.2.log · cargo nextest run -p pulse-app --features mcp-server --bin… (101 chars)
 18 probe       green · exit 0 · 0.01s · 0 B → 18.2.log · git diff --quiet 279a477e9e12a1e76bae3bab3c64e42db9949a8f … (95 chars)
entries 25 · green 7 · red 0 · recorded 0 · timeout 0 · not-run 18
```

What each printed, from its log:

- entry 5: `lint-test/cargo xtask verify:capability-matrix (chunk`, then `1 True True True -`
- entry 8: `82 82 True 36 46 0`
- entry 9: `6 15 25 46 4 36`
- entry 15: `verify:capability-matrix: clean (82 ids: 36 claimed, 46 retired, 0 violation(s))`
- entry 16: `Summary 2944 tests run: 2944 passed, 0 skipped`
- entry 17: `Summary 1 test run: 1 passed, 14 skipped`
- entry 18: no output; the bindings equal the chunk base

## The six follower lines — the operator's second word

The first word named the record, the table row and the atom; six lines of `plan.md` still stated the figures or
the membership as they stood before correction 2, and were reported as left. The operator's second word, the
operator, 2026-10-10, given in the same session and kept verbatim as inputs#I10: "The six plan lines that still
state the old figures or membership follow the same operator edit: bring each to 6 none, 15 part, 25 runs and move
P-076 out of the no-running-proof list (it joins the part list in the route pin owed), recorded in the same
evidence file; research.md stays as written and the report says it was superseded on this point." Edited
2026-10-10T11:47:33Z, before the pre-CI commit:

- `:150` — "Guard totals: none 7, part 14, runs 25" → "none 6, part 15, runs 25"
- `:247` — entry 9's `note`: "the guard states count 7 none, 14 part, 25 runs" → "6 none, 15 part, 25 runs"
- `:358` — the acceptance criterion: "the second probe prints `7 14 25 46 4 36`" → "`6 15 25 46 4 36`"
- `:438` — "7 / 14 / 25: counted from the same table's bracketed states" → "6 / 15 / 25: …"
- `:442-444` — "the figures are seven and fourteen (research, the corrected paragraph)" → "the figures are six and
  fifteen (research, the corrected paragraph, reads seven and fourteen; the operator edit of 2026-10-10 reads
  P-076 as a part)"; the sentence after it, that the `CARRY:` names all twenty-one, holds unchanged (6 + 15)
- `:445-447` — the route pin owed on `Window's gates retired`: P-076 leaves the list of ids with no running proof
  (now P-025, P-026, P-064, P-065, P-066, P-068) and joins the list of ids with a part of one, between P-071 and
  P-078

After the six edits no line of `plan.md` states 7, 14 or the old membership as current (a grep for the old figures
returns nothing), and the block still parses (`gate.py run --plan … --dry-run`: `entries 25`). No `expect` atom was
touched by them, so no entry was re-fired for them.

**research.md is superseded on this point and stays as written.** It reads P-076 as "no proof runs" in two places
(the line on the gates that drive the window, and the paragraph naming the seven ids with no running proof and the
fourteen with a part). The operator's one rule reads P-076 as a part; the record and the plan carry that reading.

## Limits

No pin reads the guard-state counts: the committed-record pin asserts the ids and the claimed and retired counts.
The workspace suite was re-fired because that pin reads the file the correction changed. Entries 1 to 4, 6, 7, 10
to 14 and 19 were not re-fired; their first firing stands and no file they read changed, except that the pre-push
check's test stage (entry 19) ran the committed-record pin over the record as it stood before the correction.
