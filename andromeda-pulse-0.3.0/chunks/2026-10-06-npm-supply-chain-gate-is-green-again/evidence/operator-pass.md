# Operator pass — plan entries 26, 27, 28

Fired by the implementing agent on the overseer's word (founder-delegated, 2026-10-07, in the session): "Go: the
operator pass as planned: hygiene (26), the pre-CI commit and clean-tree push (27), the ci.py CI read (28). If a
CI job goes red on a runner or network fault with no link to this diff, re-run the failed job yourself once and
record both readings. Report the verdict and stop before the wrap." Each entry was driven once, by hand, in its
listed form.

## Entry 26 — hygiene (before the pre-CI commit)
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- fired: 2026-10-06T22:36:50Z · exit 0
- atoms: `exit 0` held · `contains hygiene: clean` held
- output, verbatim:

```
gate v1.11 · 0aca1113
root . · case exact · planes rust, ts · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P2 · P3 rust · P3 ts — each fired on its synthetic known positive
hygiene: clean — read 38 (runs 32 · evidence 3 · inputs 3) · trails 12 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

- This file was written after that read, so the read did not cover it.
