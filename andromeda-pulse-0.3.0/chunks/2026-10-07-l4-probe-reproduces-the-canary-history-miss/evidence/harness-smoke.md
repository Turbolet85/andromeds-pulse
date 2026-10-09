# Harness smoke of the replay path (no model, no GPU, no capture)

This is not a reading. It drove the probe's new command-line paths end to end over inputs this run made itself, so
that the two pre-registered readings, each fired once, do not meet the wiring for the first time. Nothing here was
read from a capture, and nothing here says anything about the model.

## The inputs

- **Two synthetic capture directories** under the gitignored `target/l4-decision-probe/synthetic-capture/`, in the
  operator's wrapper's form (`prompt.txt`, `argv.nul`): `miss` is the probe's own `shipped` composition of `S16`
  (8046 bytes, sha256 `edb1d356d78e01d5…`), `control` its composition of `S8` (7693 bytes, sha256
  `a7412217140a1149…`), each with the probe's own argv. A temporary test in `replay.rs` wrote them
  (`prompt.txt` written 2026-10-07T12:37:22Z); it was removed before any gate ran, and the file hashes in
  `mutation-checks.md` are of the tree without it.
- **A stub in place of `llama-cli`**: a shell script in the session scratchpad that prints one canned L4 output per
  call. It names the `scope_id` of the prompt's cue line, except on a prompt with three or more corpus lines and no
  restate line, where it names that `scope_id` with `-canary` appended (a service miss). The three locator variables
  pointed at it for these calls alone. It loads no model and uses no GPU.

## What ran, and what it printed

**Dry runs** (spawn nothing):

```
dry-run: arm shipped replay:miss: prompt 8046 bytes (max 16384) · argv same · corpus lines 5
dry-run: arm shipped replay:control: prompt 7693 bytes (max 16384) · argv same · corpus lines 2
dry-run: arm CR replay:miss: prompt 8175 bytes (max 16384) · argv same · corpus lines 5
dry-run: arm CR replay:control: prompt 7822 bytes (max 16384) · argv same · corpus lines 2
dry-run: arm CO replay:miss: prompt 8046 bytes (max 16384) · argv same · corpus lines 5
dry-run: arm CC replay:miss: prompt 7681 bytes (max 16384) · argv same · corpus lines 2
dry-run: arm CC replay:control: prompt 7693 bytes (max 16384) · argv same · corpus lines 2
dry-run: arm CX replay:miss: prompt 7683 bytes (max 16384) · argv same · corpus lines 2
dry-run: arm CO replay:control: skipped · own lines unknown
dry-run: arm CX replay:control: skipped · own lines unknown
```

(`--arms shipped,CR,CO,CC,CX`, both replays, `--replay-scope conductor --own-lines 2,5`.) Three refusals were read
the same way, each exit 2 before any spawn: a scope other than the cue line's
(`replay miss: the first cue line's scope_id is not --replay-scope`), a directory that holds no capture
(`replay miss: prompt.txt is missing or not a regular file`, over `pulse-app/examples`), and, before the fix below,
`replay:control: --own-lines names a line the block does not hold`. The refusal of a capture placed inside the work
tree was not driven here; its pin holds it.

**The section reader** over `miss`, `control`, `S14` and `S16`: `miss vs control` and `miss vs S14` each
`differs in digest.corpus-matches`; `miss vs S16: differs in none` (the synthetic `miss` is `S16`).

**The replay reading through the stub** (out dir `smoke-stub-replay`, 40 rows, written 2026-10-07T12:39:11Z):

```
l4-decision-probe: binary llama-cli-stub.sh (cuda, -ngl 99) · model llama-cli-stub.sh · arms shipped · shapes replay:miss,replay:control · n 20 per shape
  arm shipped: reproduce replay:miss REPRODUCES · misses 20/20 · unparsed 0
  arm shipped: reproduce replay:control CLEAN · misses 0/20 · unparsed 0
l4-decision-probe: reproduction verdict: REPRODUCED · replay:miss
```

Exit 0. A first call, with a canned output the product's bounded parse rejected, read both replays UNREAD and exited
2 (`INCONCLUSIVE - replay:miss unread`): the unparsed arm of the rule, reached by accident.

**The remedy reading through the stub** (`--remedy-from` that out dir; out dir `smoke-stub-remedy`, 340 rows, written
2026-10-07T12:40:00Z):

```
l4-decision-probe: remedy prompt: replay:miss n 20 (replay reading 20/20) · from smoke-stub-replay
  arm CO replay:control: skipped · own lines unknown
  arm CX replay:control: skipped · own lines unknown
  arm CR: composes as shipped on S1,S2,S3,S7 · takes its counts there
  arm CO: composes as shipped on S1,S2,S3,S4,S7,S8 · takes its counts there
  arm CC: composes as shipped on S1,S2,S3,S4,S7,S8 · takes its counts there
  arm CX: composes as shipped on S1,S2,S3,S7,S8 · takes its counts there
  arm CR: remedy bar MET · guard HOLDS · not both 0/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
  arm CO: remedy bar NOT MET · guard HOLDS · not both 20/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
  arm CC: remedy bar MET · guard HOLDS · not both 0/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
  arm CX: remedy bar MET · guard HOLDS · not both 0/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
l4-decision-probe: known-positive: HELD · shipped misses 20/20 on replay:miss (min 3)
l4-decision-probe: remedy selection: CR · order CR,CO,CC,CX
l4-decision-probe: derived shape: absent
l4-decision-probe: remedy verdict: SELECTED · arm CR
```

Exit 0. The per-arm summary lines between the plan and the verdict are left out here. 340 rows are the generations
the plan above leaves: `shipped` 160, `CR` 80 (both replays, `S4`, `S8`), `CO` 20, `CC` 40, `CX` 40 (the `miss`
replay and `S4`). The guard shapes a candidate did not generate read `shipped`'s counts in its line.

**The capture-text entry, rehearsed** with the two synthetic directories in place of a capture: over the chunk folder,
the run dirs, the probe and the digest module it printed `0` and exited 1 under `pipefail` (the entry's two atoms);
over a scratch directory holding one file with one of those corpus lines it printed `1` (the known-positive control
the entry's note asks for).

## What the smoke found

One defect, fixed in this run. `--own-lines` was applied to every replay, so positions read against the `miss` block
were out of range on the shorter `control` block and the whole call was INCONCLUSIVE before any spawn. The remedy
entry passes one `--own-lines` value beside two replays, so the remedy reading would have stopped there. The flag now
describes the replay labelled `miss`; any other replay's own lines are unknown, and `CO` and `CX` are skipped on it
(pinned in `the_replay_flags_parse_and_a_replay_brings_no_s_shape_of_its_own`, mutation 27).

## What it does not show

- Nothing about the model: every label above is the stub's.
- Nothing about a real capture: the loader was not run on one. Whether the product's recorded argv equals the probe's
  own is the capture precondition entry's to say.
- The paths the stub cannot reach: a known-positive that is lost, a guard that trips, n above 20 and the derived
  shape are held by the unit pins alone.
