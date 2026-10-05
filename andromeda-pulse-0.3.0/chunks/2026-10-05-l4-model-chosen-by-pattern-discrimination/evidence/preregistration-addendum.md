# Pre-registration addendum 1 — the audit-verdict parser (not a keyword fix)

Written (addendum, UTC): 2026-10-05T11:47:26Z. The order, as it happened: the series (10:40:32Z–11:41:44Z), the
overseer's verdict file (11:45:37Z), the refused first grade, the parser fix, the second grade and the table, then this
addendum. Writing it after the grade and the table is safe only because the change cannot move a label: the
byte-identity proof below shows everything the labels, the audit and the rule are computed by is untouched.

## What changed, and what did not

- The overseer's `audit-verdicts.txt` annotated each verdict line with a trailing `# …` note. The first
  `--audit-grade` refused it, `INCONCLUSIVE - audit-verdicts.txt line 2 unreadable`: `grade_audit` ignored only
  whole-line `#` comments. The sample's own instructions said "`#` comments are ignored", which covers a trailing one.
- Fix: `grade_audit` drops everything from the first `#` on a line before reading it. One pin was extended: an
  annotated copy of a verdict set (a `#` header line, a trailing note on every `agree`) grades the same as the plain
  one, `Ok((2, 20))`.
- `patterns.rs` sha256 moved from the pre-registered `d597e7ccef151ce4413cf46f2a0628c8d3a19ba41c289da7346bf57d1040229d`
  to `4d3e6f8155f5a71a43e1f0aa4cdc3b4d782ee2b991d13faf8fd5161f63ff0ca9`.
- Proof that nothing else moved: reversing exactly the two edits (the parser's two lines and the added pin) on the new
  file reproduces bytes hashing to `d597e7cc…229d`, the pre-registered value. So the shapes, their numbers, the ground
  truth, the cause-word sets, the scorer, the re-grade, the audit draw and the rule are byte-identical to the
  pre-registration. The generations were not touched; the overseer's file was not edited.
- After the fix: `test result: ok. 61 passed`; `--audit-grade` read `audit: disagreement 9/96 · bar 10% · agree`.
