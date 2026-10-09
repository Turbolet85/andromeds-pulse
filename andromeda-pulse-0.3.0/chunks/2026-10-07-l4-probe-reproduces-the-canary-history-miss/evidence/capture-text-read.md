# The capture-text read (plan Step 17, entry 28) — fired once, while the captures still exist

- **Verdict: green.** No file of the chunk folder, the run dirs, the probe or the digest module holds a corpus line
  of any of the three captured prompts.
- **Fired:** by hand on the final tree at 2026-10-07T14:38:43Z, and again at 14:40:36Z once every other evidence
  file and this run's last ledger records were written. Both read the same: 0 files, exit 1, the control 1. It is
  a probe, not a reading, and re-firing it is what covers the files written between the two.
- **run** (the plan's text with the third captured prompt added to the `cat`, as the readings read three):
  `grep -rlFf <(cat "$L4_REPLAY_MISS/prompt.txt" "$L4_REPLAY_CONTROL/prompt.txt" "$L4_REPLAY_D3/prompt.txt" | grep -E '^  - \[' | cut -c5- | sort -u) andromeda-pulse-0.3.0/chunks/2026-10-07-l4-probe-reproduces-the-canary-history-miss .andromeda/runs pulse-app/examples crates/triage/src | wc -l`
- **exit:** 1 · **atoms:** `exit 1` held · `last line 0` held → **green** (the 0 form of a grep count under
  pipefail).
- **What it searched for:** 9 distinct corpus lines, the 3, 1 and 5 the second, first and third drives' prompts
  render, each as one fixed string from its fingerprint bracket to its status.
- **Known-positive control**, run once beside it in the session scratchpad, outside the tree: a scratch file holding
  one captured corpus line, written by the shell and never printed, read by the same pattern list: 1 file counted,
  exit 0. The file and its directory were removed in the same call.

What the read does not cover: a captured line reworded, a title quoted without its bracket and tail, or text of a
prompt outside its corpus block. Those are held by how the evidence was written, from counts, hashes and closed
labels. `S17`'s titles are the builder's own sentence since the operator's ruling (inputs#I23).

The file this record sits in was written after the read; it holds no corpus line.

## Fired again after the third reading (2026-10-07T15:55:19Z)

- **Verdict: green.** The same run, on the tree with the third reading's files (`evidence/preregistration-third.md`,
  `evidence/reading-third.md`, four input snapshots, this run's dir): 0 files, exit 1, both atoms held.
- The same 9 distinct corpus lines. The known-positive control, run beside it as before: 1 file counted, the file
  and its directory removed in the same call.
- The third reading's builder read the three captured blocks, and four other captured prompts' cue lines, through
  two scratch scripts that print positions, counts, ages, statuses, kind words and scope ids. Neither printed a
  title or a fingerprint, and neither wrote a file. The session scratchpad was read by the same pattern list before
  the go: 0 files.
- This section was written after the read; it holds no corpus line.

## Fired again after the product change became `CX` (2026-10-07T18:36:19Z)

- **Verdict: green.** The same run, on the tree of that re-entry: the swapped selection and its pins, the probe's
  `nb` arm and product path, `evidence/product-path-equality.md`, `evidence/remedy-shipped.md`, the new sections of
  `evidence/mutation-checks.md` and `evidence/prompt-sizes.md`, four input snapshots and this run's dir. 0 files,
  exit 1, both atoms held.
- The same 9 distinct corpus lines. The known-positive control, run beside it as before: 1 file counted, exit 0,
  the file and its directory removed in the same call.
- This re-entry read the captures three ways, none of which wrote a line of one: the probe's dry runs (closed words,
  sizes and counts); one scratch script that printed, per captured prompt, its cue's kind word and scope id and a
  six-character hash of each fingerprint it carries, and no title and no fingerprint; and this read. The session
  scratchpad was read by the same pattern list in the same call: 0 files.
- The product path reads a captured line back into its fields inside the probe's process and compares bytes there.
  Nothing of it is printed but the line `evidence/product-path-equality.md` quotes.
- This section was written after the read; it holds no corpus line.
