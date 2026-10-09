# Prompt sizes without a model (plan Step 10)

Source: gate entry 7 of this run, the dry-run
`./target/debug/examples/l4_decision_probe --arms shipped,ns,L,LI --shapes S1,S2,S3,S4,S7,S8 --dry-run`
(the five L4 env vars unset), exit 0, 24 lines, no `INCONCLUSIVE`. It spawns nothing and needs no model. The bound is
`MAX_PROMPT_BYTES` = 16,384 B; every composition passed `validate_prompt_bounded` (a rejected one would have exited 2).

## Composed primary-tier prompt, bytes, per arm x shape

| arm | S1 | S2 | S3 | S4 | S7 | S8 | largest | headroom to 16,384 |
|---|---|---|---|---|---|---|---|---|
| `shipped` | 7453 | 7456 | 7460 | 7654 | 7367 | 7693 | **7693** (S8) | 8691 |
| `ns` | 7204 | 7207 | 7211 | 7405 | 7118 | 7444 | **7444** (S8) | 8940 |
| `L` | 7220 | 7226 | 7232 | 7421 | 7131 | 7457 | **7457** (S8) | 8927 |
| `LI` | 7469 | 7475 | 7481 | 7670 | 7380 | 7706 | **7706** (S8) | 8678 |

The frozen path (gate entry 8, `--arms shipped --shapes S1,S2,S3,S4,S5,S6 --dry-run`, exit 0): S5 7452, S6 7452; S1-S4 as
above. The embedded schema is 4626 B in every line.

## What the numbers say

- **The sentence adds 249 B to every composition**: `shipped` minus `ns` is 249 on all six shapes, and so is `LI` minus
  `L`. That is the pre-registered sentence (248 B) plus the one space ahead of it. The prediction (every composition
  grows by the sentence's length over v2.5) held, with the space counted.
- **`ns` reproduces the v2.5 composition**: `ns` S4 reads 7405 B, the figure the plan's baseline recorded for the
  shipped S4 prompt at the chunk base.
- **The line rewrite adds ` on {scope_id}`**: 16 B on S1 and S4 (`checkout-api`), 19 on S2, 21 on S3, 13 on S7 and S8
  (`conductor`); identical under `L` and `LI`.
- The largest composition of the reading is `LI` on S8, 7706 B, 47 % of the bound.

## The instruction and the three tiers on an empty digest

Measured once by a throwaway integration test calling the three builders with empty inputs (the file was deleted after
the run and is not in the tree):

| reading | bytes | at the chunk base |
|---|---|---|
| `TRIGGER_FRAMING_INSTRUCTION` | 787 | 538 (the plan's measurement) |
| primary tier, empty digest | 7011 | 6762 (derived: minus 249) |
| fallback tier, empty digest | 7200 | 6951 (derived: minus 249) |
| reflection tier, empty digest | 7549 | 7300 (derived: minus 249) |

The fallback tier's sanity pin (`fallback_prompt_total_bytes_bounded`) asserts under 8000 B on an empty digest: the
measured figure is 7200 B, 800 B under it. The "at the chunk base" column for the three tiers is arithmetic, not a
second measurement.

These are measurements of this tree, never a bound.

Recorded 2026-10-07T00:09:34Z.
