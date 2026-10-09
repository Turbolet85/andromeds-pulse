# Prompt sizes (plan Step 16) — every composition against the 16,384-byte bound

Read 2026-10-07T14:38:44Z from two dry runs of the probe built from the final tree (probe file `c6862650…3b47`,
`crates/triage/src/digest/retrieval.rs` `e15b4f7c…473f`): all 22 arms over `S1`-`S17` (374 lines), and `shipped`
with the four candidates over the three captured prompts (11 composed cells, 4 skipped for unknown own lines). A dry
run composes and spawns nothing. `MAX_PROMPT_BYTES` and the control-character whitelist are unchanged by this chunk.

## The product's composition (`shipped`, the tree as it is)

| source | largest prompt | where |
|---|---|---|
| synthetic shapes `S1`-`S17` | 8067 B | `S14` and `S15` (five corpus lines) |
| captured prompts | 8059 B | the third drive's (five corpus lines); the second drive's is 7854 B, the first's 7616 B |

Both are above the 7,824 B the plan carries as the recorded maximum, and 49 % of the bound. The captured figure is
the first size measured on a prompt the product itself composed in a live run: 8059 B at five rendered corpus lines,
the block's cap.

## Per arm

| arm | largest over `S1`-`S17` | largest over the captured prompts |
|---|---|---|
| `shipped`, `A0`, `A1`, `A2`, `A5`, `nr`, `R1` | 8067 B (`S14`, `S15`) | `shipped`: 8059 B (`d3`) |
| `A3` | 8128 B | not run on a replay |
| `A4` | 8197 B | not run on a replay |
| `nf` | 7179 B | not run on a replay |
| `R3`, `R1R3` | 8179 B | not run on a replay |
| `R2`, `R1R2` | 8189 B | not run on a replay |
| `R3R2` | 8301 B (`S14`, `S15`), the largest of all 385 cells | not run on a replay |
| `ns` | 7818 B | not run on a replay |
| `L` | 7831 B | not run on a replay |
| `LI` | 8080 B | not run on a replay |
| `CR` | 8196 B | 8188 B (`d3`) |
| `CO` | 8067 B | 7854 B (the second drive's, the one prompt with pre-registered own lines) |
| `CC` | 7709 B | 7739 B (`d3`) |
| `CX` | 8059 B (`S9`) | 7611 B (the second drive's) |

Every one of the 385 composed cells is within the bound; the largest, 8301 B, is 51 % of it.

## What the product change moved

The reorder changes no size: it moves lines inside the block. Against the dry run written before Step 13, every
arm composes `S1`-`S15` at the same size, and `S16` at the same size under every arm but `CC`, which keeps the first
two lines of a block whose order changed (7681 B before, 7683 B now). `S17` is new in this chunk: 7824 B under
`shipped`.

## Read again after the product change became `CX` (2026-10-07T18:31:29Z)

Everything above is the tree with `CO` in the product. The founder chose `CX` (inputs#I29); this section is the
tree of that re-entry (probe file `04082c4e…a466`, its module `5c145e27…5575`,
`crates/triage/src/digest/retrieval.rs` `16b6fd31…f6e4`). Two dry runs of the probe built from it: all 23 arms over
`S1`-`S17` (391 lines, exit 0), and `shipped` with the four candidates over each of the three captured prompts, its
own lines as the third reading pre-registered them (15 lines, each exit 0, read at 18:32:42Z).

### The product's composition (`shipped`)

| source | largest prompt | where |
|---|---|---|
| synthetic shapes `S1`-`S17` | 8059 B | `S9` (five lines, all carrying the cue's fingerprint, so all kept) |
| captured prompts | 8059 B | the third drive's, as captured (five corpus lines). A replay's `shipped` is the capture's bytes: the product at `f70be92`, before any remedy |

Under the cue's scope the product now composes a smaller or equal block on every shape. Against `nb`, the block
before the remedy:

| shape | `nb` | `shipped` | what the product drops |
|---|---|---|---|
| `S4` | 7654 B | 7453 B | its one line, another service's |
| `S12` | 7693 B | 7367 B | both lines (the sibling's, another fingerprint) |
| `S14`, `S15` | 8067 B | 7367 B | all five lines |
| `S16` | 8046 B | 7683 B | the sibling's three lines; its two conductor lines stay |
| `S17` | 7824 B | 7575 B | the sibling's two lines; the triggering service's own stays |
| the other eleven | the same | the same | nothing: no block, or lines that carry the cue's fingerprint |

`nb` composes every shape at the size the earlier sections record for `shipped` before any remedy (`S14` and `S15`
8067 B, `S17` 7824 B, `S4` 7654 B).

### Per arm

| arm | largest over `S1`-`S17` | largest over the captured prompts |
|---|---|---|
| `shipped`, `A0`, `A1`, `A2`, `A5`, `nr`, `R1` | 8059 B (`S9`) | `shipped`: 8059 B (`d3`) |
| `A3` | 8120 B | not run on a replay |
| `A4` | 8189 B | not run on a replay |
| `nf` | 7171 B | not run on a replay |
| `R3`, `R1R3` | 8171 B | not run on a replay |
| `R2`, `R1R2` | 8181 B | not run on a replay |
| `R3R2` | 8293 B (`S9`), the largest of all 391 cells | not run on a replay |
| `ns` | 7810 B | not run on a replay |
| `L` | 7823 B | not run on a replay |
| `LI` | 8072 B | not run on a replay |
| `nb` | 8067 B (`S14`, `S15`) | does not apply to a replay |
| `CR` | 8196 B (`S14`, `S15`) | 8188 B (`d3`); 7983 B (`d2`), 7745 B (`d1`) |
| `CO` | 8067 B (`S14`, `S15`) | 8059 B (`d3`); 7854 B (`d2`), 7616 B (`d1`, as `shipped`) |
| `CC` | 7709 B (`S15`) | 7739 B (`d3`); 7731 B (`d2`), 7616 B (`d1`, as `shipped`) |
| `CX` | 8059 B (`S9`) | 7732 B (`d3`); 7611 B (`d2`), 7401 B (`d1`) |

Every one of the 391 composed cells is within the 16,384 B bound; the largest, 8293 B, is 51 % of it. The largest
prompt the product now composes over the shapes is 8059 B, and the largest it composes from a captured block is the
`CX` row's 7732 B (`evidence/product-path-equality.md`). `MAX_PROMPT_BYTES` and the control-character whitelist are
unchanged.

The candidate arms are edits of `nb` since this re-entry, the block the remedy reading's arms edited: `CC` on `S16`
is 7681 B again, as before the reorder shipped, and `CO` composes every shape at `nb`'s size.
