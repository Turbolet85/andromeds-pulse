# The operator edit of the plan, between runs (2026-10-10)

**Whose word.** The operator, 2026-10-10, given in the implement session after its P4 report and kept verbatim as
inputs#I5: "one correction of the plan between runs, made by you as my hands and recorded as the operator edit in
evidence: entry 4 git diff takes --src-prefix=a/ --dst-prefix=b/ so it reads the same on any host; the plan lines
that restate the command follow. Re-fire entry 4." The course was the operator's own answer to the question on
that entry (inputs#I4). The edits below were typed by the implement agent on that word; /implement edits no plan
on its own. Edited and re-fired 2026-10-10T21:30:27Z.

Why the entry could not read its subject as written is in `evidence/test-module-guard.md`.

## The correction — entry 4 pins the two diff prefixes

`plan.md:179`, the entry's `run`, at its head:

- before: `git diff -U0 50216d7ee49c9763d1d9304aa07d3c3f3b74c7e3 -- crates/buffer/src/consumer.rs …`
- after: `git diff -U0 --src-prefix=a/ --dst-prefix=b/ 50216d7ee49c9763d1d9304aa07d3c3f3b74c7e3 -- crates/buffer/src/consumer.rs …`

Nothing else of the entry changed: the four paths, the awk text, `role`, `env`, `expect`, `new`, `baseline` and
`note` stand as the plan's author wrote them.

## The one line that restates the command

`plan.md:339-342`, the acceptance criterion "(arch) No product line changes for the factories":

- before: "the test-module-only guard (`git diff -U0 50216d7e… -- … | awk …`) prints nothing"
- after: "the test-module-only guard (`git diff -U0 --src-prefix=a/ --dst-prefix=b/ 50216d7e… -- … | awk …`)
  prints nothing"

The criterion's lines were re-wrapped so that the command stands on one line; no other word of it changed. A grep
of `plan.md` for `git diff -U0` reads these two lines and no third.

## The block after the edit

`gate.py run --plan … --dry-run`: `entries 26 · green 0 · red 0 · recorded 0 · timeout 0 · not-run 26`, the block
parses.

## Entry 4 re-fired

`gate.py run --plan … --run-dir … --marker … --entry 4`, entry line and summary line, verbatim:

```
  4 probe       green · exit 0 · 0.01s · 0 B → 4.2.log · git diff -U0 --src-prefix=a/ --dst-prefix=b/ 50216d7ee49c9… (447 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

Exit 0 and no output, both atoms held. The first firing's red stands in the trail beside it (`4.log`, 25 B).

## Known-positive control of the edited form, in the entry's own diff shape

Run by hand (git and awk only, the entry's awk text unchanged): the edited `git diff` with the commit one chunk
older than the base, `514fbe57`, against the work tree. That diff holds the product hunks of the last chunk in
`xtask/src/harness_ready.rs` and `xtask/src/engine_cycle.rs`.

- exit 1 · 25 lines, every one opening with its file name: 21 of `xtask/src/harness_ready.rs`, 4 of
  `xtask/src/engine_cycle.rs`.

The same count the plan's own control reads over `514fbe57..50216d7e`, now read from a diff against the work tree,
the form the entry runs.

## Limits
- Entries 1 to 3 and 5 to 22 were not re-fired: their first firing stands, the edit touched no file they read.
- The entry's `baseline` text still describes its P5 reading (an empty diff, a commit-to-commit control). It is
  left as written; this file is the record of what the entry reads after the edit.
- research.md and scope.md are unchanged.
