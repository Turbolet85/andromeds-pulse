# The regression guard's disposition (plan Step 12; inputs#I8)

## The guard line, as the reading printed it

```
l4-decision-probe: regression guard: HOLDS · sibling shipped 20 vs ns 20 · ordinary shipped 40 vs ns 38
```

It is the last line of the one reading (`evidence/reading.md`, series out dir `service-20261007T054342Z`).

## The two pairs of counts (`identifies` = `both`)

| half | shapes | `shipped` | `ns` | `shipped` lower than `ns`? |
|---|---|---|---|---|
| sibling | S7 + S8, 20 generations per arm | 20 | 20 | no (equal; an equal count holds) |
| ordinary | S1-S4, 40 generations per arm | 40 | 38 | no (higher) |

The pre-registered rule: TRIPPED iff the `shipped` arm's `both` count is lower than the `ns` arm's on either half.
Neither half is lower, so the guard reads **HOLDS**. Both pairs were recounted from `runs.json` and equal the line.

## Disposition: HOLDS — nothing changes

- The sentence stays in `TRIGGER_FRAMING_INSTRUCTION`; the lineage stays `v2.6` / `v1.5-fallback` /
  `v1.5-reflection`; the Step 2 pin stays.
- No file was restored to the chunk base and no gate entry was re-run on account of the guard: the tree the reading
  ran on is the tree every non-leg entry read green on, and it has not been edited since.
- The reading was fired once and is not fired again.

The guard decides only whether the sentence stays. It is not a claim that the sentence caused the difference: on the
sibling half the two arms read the same, and the ordinary half differs by 2 of 40 generations.

Recorded 2026-10-07T06:09:02Z.
