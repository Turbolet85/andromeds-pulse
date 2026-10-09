# Pre-registration record — the third reading (the three captured prompts under the two remedies that met the bar)

- **Chunk:** `2026-10-07-l4-probe-reproduces-the-canary-history-miss`
- **Chunk base:** `48714f09c95eaa0a2461d9a5ee26c96591983bb0`
- **Written:** 2026-10-07T15:16:21Z, at a re-entry of /implement before the operator pass, after the remedy reading
  (`evidence/reading-remedy.md`, SELECTED `CO`) and before any generation of this reading.
- **Whose word:** the operator (the pc overseer), 2026-10-07, in this session's invocation (inputs#I27) and in the
  relay it names (inputs#I24, sha256 `02f6757d4d66c1ac44606b4de6b36b930549efb96a2f767aa5bc458ad76e9cb1`, section 2).
  The plan has no step for this reading; its rules are the relay's, fixed here.
- This file is never edited after the reading.
- No text of a captured prompt is in this file. The builder read the captured digests in its own tool calls
  (inputs#I19 section 5) through two scratch scripts that print positions, counts, ages, statuses, closed kind words
  and scope ids, and no title or fingerprint; what follows states derived facts in the builder's words.

## What the reading is, and what it is not

- **It selects nothing.** `CO` stays the selected candidate of the remedy reading whatever this reads. No bar, no
  guard, no known-positive rule, no verdict word and no exit code that reads as one. The probe runs with no verdict
  flag: it prints counts, exits 0 when an invocation completed and 2 (INCONCLUSIVE) when a precondition failed and
  nothing was measured.
- **What it is for** (inputs#I24 sections 1 and 2): the remedy reading read `CO` and `CX` on the second drive's
  prompt alone. The first and third drives' prompts were never read under either, and the captures are deleted when
  this chunk closes, so this is the only slot in which a real prompt can be read under an arm. The founder decides
  which remedy ships and gets these numbers with the question. Until he answers, nothing is committed.
- **What changes in the tree:** nothing but this folder's evidence files and this run's records. No product file and
  no probe file changes for this reading.

## The instrument

- The probe as the tree holds it, the tree the previous run's last gate block read 19 green, 0 red, 12 operator legs
  not run (ended 2026-10-07T14:38:08Z, after the last source edit at 14:35:21Z). This run reads the block again on
  that tree before the go; its summary is in the reading's record. Hashed at 2026-10-07T15:15:03Z:
  - `pulse-app/examples/l4_decision_probe.rs` `c68626507a4afc719c2653c7d16ead28b943f039311118b519c8bbbe61b83b47`
  - `pulse-app/examples/l4_decision_probe/replay.rs` `004c4660db44f73aecd7e72df8511709ca8387f4fbdf94534f695284859536f2`
  - `crates/triage/src/digest/retrieval.rs` `e15b4f7c31666edb982f33ca2e7c43140cf0c76c5bcfc9d6a31bb7f57dca473f`
  - `crates/triage/src/digest/assembler.rs` `c24d12f2037d860fb9b43397c919e5ef8c79050f04f7cf78eda9716b8898ded1`
- **The product change in the tree enters no cell.** The tree ships `CO` in the product's selection since Step 15. A
  replay never renders a digest: `shipped` passes the captured prompt's bytes unchanged, and `CO` and `CX` are the
  harness's block edits of those bytes (`replay::edit_block`), the edits the remedy reading used.
- The product's argv (`build_llama_cli_args`), the committed GBNF, the shipped model
  `gemma-4-E4B-it-Q4_K_M.gguf` on the CUDA route, the env file inputs#I9 snapshots (`cmp` against the live file:
  exit 0 at 15:15:03Z).
- **No probe change.** `--own-lines` describes the replay labelled `miss` (`l4_decision_probe.rs:2931-2940`), and
  each of the three prompts has its own set. So each prompt is read in its own invocation, under the label `miss`.
  **The label is a slot, not a fact:** in this reading `replay:miss` names the first drive's prompt in the first
  invocation, the third drive's in the second and the second drive's in the third. No captured drive missed live.
  Each invocation writes its own `runs.json`; the out dir's last path component (`d1`, `d3`, `d2`) and the sha256
  below say which prompt its rows belong to.

## The prompts

| drive | directory name | sha256 of `prompt.txt` | bytes | rendered corpus lines | read before under `shipped` (`both` of 20) |
|---|---|---|---|---|---|
| d1 | `20261007T123304.212607066.88021` | `1d953dfbf29bfca03a3c6d4ef30188630a7f2469faab76e21d554783666e22ec` | 7616 | 1 | 19 (replay reading), 17 (remedy reading) |
| d2 | `20261007T124239.786338234.251033` | `21c39d8cdf8e55191fb292168b8fb352a1ec61821cd2e882bbcc86ffa42409c8` | 7854 | 3 | 9 (replay reading), 11 (remedy reading) |
| d3 | `20261007T125156.990720047.348846` | `83b8a3ea7ccbcfbda4dfbc76f62ded32f7e6362d711d8ba9ad34bbb2ac63661e` | 8059 | 5 | 20 (replay reading), 18 (remedy reading) |

The three hashes were re-read at 2026-10-07T15:14:52Z and equal the ones both earlier pre-registrations name.

## The own lines, by position

| prompt | `--own-lines` | the block, newest first |
|---|---|---|
| d1 | `none` | one line: the sibling's retry storm |
| d2 | `2` | the sibling's retry storm, the triggering scope's own earlier storm, the sibling's again (as pre-registered for the remedy reading) |
| d3 | `2,5` | the sibling's retry storm, the triggering scope's own storm of the second drive, the sibling's error-rate spike, the sibling's retry storm of the second drive, the triggering scope's own storm of the first drive |

The rule is the remedy pre-registration's: the own lines are those of incidents scoped to the cue's `scope_id`, plus
any line the fingerprint arm keeps on its own. The reading the three sets rest on, none of it a line's title:

1. **Which incident each line is.** Conductor's record of each drive lists the incidents opened before the drive's
   own, by id and `opened_at`, and flags the ones carrying the scenario's fingerprint (inputs#I25 `:407-409`,
   inputs#I22 `:407-412`, inputs#I26 `:407-414`). A block is ordered by recency, and each line's rendered age equals
   the whole minutes between its incident's `opened_at` and the drive's own incident's:
   - d1: incident 1 (3 m). One earlier incident, one line.
   - d2: incidents 3, 2, 1 (3, 9, 12 m).
   - d3: incidents 6, 5, 4, 3, 2 (3, 9, 9, 12, 18 m). Incident 1 (21 m) is the sixth candidate the five-line cap
     cuts (inputs#I26 `:579`, 6 retrieval rows).
2. **Each incident's scope, from the capture itself.** An incident's `opened_at` is its creating prompt's assembly
   instant (`evidence/preregistration-replay.md`). For each of the seven incidents exactly one capture directory
   follows that instant, by 4 to 14 ms, and that prompt's first cue line carries the incident's kind and scope:
   - incidents 1, 3 and 6: a retry storm scoped to `conductor-canary`;
   - incident 4: an error-rate spike scoped to `conductor-canary`;
   - incidents 2, 5 and 7: a retry storm scoped to `conductor`. They are the three drives' own (7616, 7854 and
     8059 bytes, the three prompts above).
   So of the lines above, the triggering scope's are incident 2 (d2 line 2, d3 line 5) and incident 5 (d3 line 2).
3. **The fingerprint arm agrees.** In each prompt exactly one fingerprint of the block stands anywhere outside it,
   once, as the prompt's citable evidence id: on d2's line 2 and on d3's lines 2 and 5, the same fingerprint in
   both prompts. d1's line carries a fingerprint that stands nowhere outside its block. The other lines carry three
   further fingerprints, and d3's line 3 an empty one (the error-rate spike has no exception fingerprint).
   Conductor's flags say the same: incidents 2 and 5 carry the scenario's fingerprint, and 1, 3, 4 and 6 do not.

One derived fact about d3's line 2, stated because it bears on what `CX` leaves in that prompt: the line is the
triggering scope's own by its creating cue and by its fingerprint, and its title names the sibling's id and does not
name the triggering service apart from it. A title is authored by the model at the incident's creation. So on d3
`CX` keeps two lines, and one of them names the sibling.

## The cells

n = 40 in every generated cell. Eight cells, seven generated, 280 generations.

| prompt | arm | generated | composition (the dry run below) |
|---|---|---|---|
| d1 | `shipped` | yes | 7616 B, 1 corpus line |
| d1 | `CO` | **no: it takes `shipped`'s counts** | byte for byte `shipped`: a reorder of a one-line block |
| d1 | `CX` | yes | 7401 B, no corpus block (the header and its note go with the last line) |
| d3 | `shipped` | yes | 8059 B, 5 lines |
| d3 | `CO` | yes | 8059 B, 5 lines in the order 2, 5, 1, 3, 4 |
| d3 | `CX` | yes | 7732 B, lines 2 and 5 |
| d2 | `CO` | yes | 7854 B, 3 lines in the order 2, 1, 3 |
| d2 | `CX` | yes | 7611 B, line 2 |

- The relay counts 320 generations. The difference is the d1 cell under `CO`: by the relay's own rule an arm that
  composes byte-identical to `shipped` on a prompt takes `shipped`'s counts (inputs#I24 section 2), and by the plan
  a second sample of one distribution is not generated. The identity is the instrument's own comparison, not an
  argument (the identity probe below).
- So on d1 `CO` cannot differ from `shipped`, known before the reading. The relay names this as its hypothesis
  (inputs#I24 section 1); it is a consequence of the edit, and what the reading measures on d1 is `shipped` against
  `CX`.
- `shipped` is not read on d2 here: the operator names the cells, and d2 under `shipped` was read twice.

## What is counted, per cell

- **`both`**, **service misses** (`signal_only` plus `neither`), **`unparsed`**: the `identifies` label over the first
  hypothesis statement, graded for the scope `conductor`, as in every earlier reading of this chunk. An `unparsed`
  generation is counted apart and is not a miss.
- **`names_other`**: the generations whose first hypothesis statement names `conductor-canary`, by
  `--count-naming conductor-canary`, the detector the replay pre-registration fixed (its amendment 5). It reads no
  placement: a statement that names the sibling to rule it out counts.
- **How the report gives them:** one row per cell with this reading's counts alone. The earlier readings' counts for
  the same prompt and arm stand beside them and are not pooled: d1 and d3 under `shipped` (two readings of 20 each),
  d2 under `CO` (19 of 20, named 2) and under `CX` (20 of 20, named 0).
- No threshold is applied to any count. The words REPRODUCES, CLEAN, MET and HOLDS are not used of this reading.

## What it does not show, stated before it is read

- One sample of 40 per cell on three prompts. The same prompt under the same arm read 20 then 18 of 20 (d3) and 19
  then 17 (d1) in the two earlier readings; a difference of two or three in 40 between two cells is inside what one
  cell does on its own.
- A replay through the probe, not a live drive: the product's pipeline is not driven.
- Three prompts of one capture run. A prompt whose block holds other kinds, more own lines or no own line at all is
  not among them, except d1 for the last.

## The entries, as they are fired

Three shell names carry the capture directories, `L4_REPLAY_D1`, `L4_REPLAY_D2` and `L4_REPLAY_D3`, variables of
these legs alone as the plan's are. `<probe>` is `./target/debug/examples/l4_decision_probe`.

**The dry runs** (fired 2026-10-07T15:14:52Z on the built probe, the model variables unset; they spawn nothing, each
exit 0; `argv same` on every line):

```
dry-run: arm shipped replay:miss: prompt 7616 bytes (max 16384) · argv same · corpus lines 1
dry-run: arm CO replay:miss: prompt 7616 bytes (max 16384) · argv same · corpus lines 1
dry-run: arm CX replay:miss: prompt 7401 bytes (max 16384) · argv same · corpus lines 0
```
```
dry-run: arm shipped replay:miss: prompt 8059 bytes (max 16384) · argv same · corpus lines 5
dry-run: arm CO replay:miss: prompt 8059 bytes (max 16384) · argv same · corpus lines 5
dry-run: arm CX replay:miss: prompt 7732 bytes (max 16384) · argv same · corpus lines 2
```
```
dry-run: arm CO replay:miss: prompt 7854 bytes (max 16384) · argv same · corpus lines 3
dry-run: arm CX replay:miss: prompt 7611 bytes (max 16384) · argv same · corpus lines 1
```

In order: `--arms shipped,CO,CX --replay "miss=$L4_REPLAY_D1" --replay-scope conductor --own-lines none`, then
`--arms shipped,CO,CX --replay "miss=$L4_REPLAY_D3" --replay-scope conductor --own-lines 2,5`, then
`--arms CO,CX --replay "miss=$L4_REPLAY_D2" --replay-scope conductor --own-lines 2`, each with
`--n 40 --count-naming conductor-canary --dry-run`. The largest composition is 8059 B, under the 16,384 B bound.

**The identity probe** (fired 2026-10-07T15:15:03Z; a dry run in the remedy mode, used only for its comparison of
each candidate's composition with `shipped`'s on the prompt labelled `miss`; it spawns nothing, grades nothing and
prints no bar; exit 0). With the first drive's directory as `miss` and `--own-lines none`:

```
dry-run: arm CX replay:miss: prompt 7401 bytes (max 16384) · argv same · corpus lines 0
dry-run: arm CO: skipped · composes as shipped on replay:miss
```

Its form: `<probe> --arms shipped,CO,CX --replay "miss=$L4_REPLAY_D1,control=$L4_REPLAY_D2" --replay-scope conductor
--own-lines none --remedy-from target/l4-decision-probe/replay-20261007T131536Z --remedy-read-as d2 --n 40
--bar-sibling 38 --bar-ordinary 72 --dry-run`. The remedy mode needs a `control` label and a sizing row to start;
neither bears on the comparison, and its `remedy prompt:` line describes no reading.

**The slot precondition**, fired once by hand on the go:

`nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv,noheader && echo third-slot`
— atoms `exit 0`, `contains third-slot`, `lacks llama`.

**The reading**, fired once by hand on the go, bounded at 2700 s:

```
O="target/l4-decision-probe/third-$(date -u +%Y%m%dT%H%M%SZ)" && echo "series out $O" && unset ANDROMEDA_PULSE_HARDWARE_PROFILE ANDROMEDA_PULSE_L4_ALLOW_ROOT ANDROMEDA_PULSE_L4_DETERMINISTIC && . "$HOME/dev/projects/additional/pc-overseer/l4-env.sh" && cargo build -p pulse-app --example l4_decision_probe && ./target/debug/examples/l4_decision_probe --arms shipped,CX --replay "miss=$L4_REPLAY_D1" --replay-scope conductor --own-lines none --n 40 --count-naming conductor-canary --out "$O/d1" && ./target/debug/examples/l4_decision_probe --arms shipped,CO,CX --replay "miss=$L4_REPLAY_D3" --replay-scope conductor --own-lines 2,5 --n 40 --count-naming conductor-canary --out "$O/d3" && ./target/debug/examples/l4_decision_probe --arms CO,CX --replay "miss=$L4_REPLAY_D2" --replay-scope conductor --own-lines 2 --n 40 --count-naming conductor-canary --out "$O/d2"
```

- **Atoms recorded, none an exit:** the header line with
  `binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf` three times; an `identifies` line and a
  `second count replay:miss` line for every generated arm; `lacks INCONCLUSIVE`; three fresh `runs.json` of 80, 120
  and 80 rows.
- **Order:** d1, then d3, then d2: the two prompts never read under an arm first. The chain stops at an invocation
  that exits non-zero. What a completed invocation wrote stands and is reported; what did not run is reported as
  not run, and nothing is fired a second time.
- **Predicted:** 280 generations at the remedy reading's 6.5 s each, about 30 minutes, plus the build.
- **The bound** (inputs#I24 section 4): no go is asked after 18:25 local and the reading ends by 19:00 local. It is
  not launched when 45 minutes do not remain before 19:00.
- Never re-run, re-ordered or re-sized. After it: the table is reported and the run stops for the founder's answer
  (inputs#I24 section 3).
