# The third reading — fired once, as pre-registered

- **It selects nothing and carries no verdict.** `CO` stays the selected candidate of the remedy reading
  (`evidence/reading-remedy.md`). This file gives counts for the founder's decision on which remedy ships
  (inputs#I24).
- **Fired:** once, 2026-10-07T15:23:23Z to 15:53:19Z (17:23 to 17:53 local, inside the bound of 19:00), on the
  operator's go given in this session (inputs#I28). Never re-run, re-ordered or re-sized.
- **Pre-registration:** `evidence/preregistration-third.md`, written 2026-10-07T15:16:21Z, before any generation; its
  sha256 when the reading launched was `2b49fb1beda85f04ee7e1cd9036a54c8ff301c4710e879400c5d09bd290485bd`.
- **Tree:** the chunk base `48714f0` plus the chunk's edits as the previous run left them, `CO` in the product. No
  source file changed for this reading (probe file `c6862650…3b47`, module `004c4660…36f2`, as the pre-registration
  names them, re-hashed at the slot read). A replay renders no digest, so the product change enters no cell.
- **Gate block before the go** (this run's gate tool, ended 2026-10-07T15:18:30Z, on that tree): 19 green, 0 red,
  12 operator legs not run.
- **Series out dir:** `third-20261007T152323Z` (under the gitignored `target/l4-decision-probe/`), one sub-directory
  an invocation, each with its `runs.json` and `l4-output.gbnf`:

  | sub-directory | prompt | rows | sha256 of `runs.json` |
  |---|---|---|---|
  | `d1` | the first drive's, `1d953dfb…22ec` | 80 | `909b2873e0009079d51a05653e5e35ff4ae23c170df6aee6535b2f983c202497` |
  | `d3` | the third drive's, `83b8a3ea…661e` | 120 | `758e851952852f5b20c9c5308a70c5b0129e66f93b6275dcbc50da24e66e15bb` |
  | `d2` | the second drive's, `21c39d8c…09c8` | 80 | `cdcae9e471d4b375ea1668a8412fc30bebeef94e6fd8874e988f8e27cc1ca526` |

- **The label `replay:miss` is a slot.** Every row of all three files carries it; the sub-directory says which prompt
  the rows belong to. No captured drive missed live. No text of a captured prompt and no model text is in this file.

## The slot precondition (driven once by hand)

- **run:** `nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv,noheader && echo third-slot`
- **exit:** 0 · **atoms:** `exit 0` held · `contains third-slot` held · `lacks llama` held (0 lines carry `llama`)
  → **green**
- Read at 2026-10-07T15:23:07Z: two compute apps, neither a `llama-cli` — a desktop OSD (10 MiB) and a desktop
  application's GPU process (49 MiB); their command lines are not copied here. GPU utilisation read 2 %, 960 of
  24576 MiB in use; no listener on 4317 or 4318; no `llama` or probe process in the host's process list. The
  operator's own slot read, given with the go, says the same (inputs#I28).

## The reading (driven once by hand, bounded at 2700 s)

- **run** (the pre-registration's text, unchanged; `L4_REPLAY_D1`, `L4_REPLAY_D2` and `L4_REPLAY_D3` the three
  capture directories):
  `O="target/l4-decision-probe/third-$(date -u +%Y%m%dT%H%M%SZ)" && echo "series out $O" && unset ANDROMEDA_PULSE_HARDWARE_PROFILE ANDROMEDA_PULSE_L4_ALLOW_ROOT ANDROMEDA_PULSE_L4_DETERMINISTIC && . "$HOME/dev/projects/additional/pc-overseer/l4-env.sh" && cargo build -p pulse-app --example l4_decision_probe && ./target/debug/examples/l4_decision_probe --arms shipped,CX --replay "miss=$L4_REPLAY_D1" --replay-scope conductor --own-lines none --n 40 --count-naming conductor-canary --out "$O/d1" && ./target/debug/examples/l4_decision_probe --arms shipped,CO,CX --replay "miss=$L4_REPLAY_D3" --replay-scope conductor --own-lines 2,5 --n 40 --count-naming conductor-canary --out "$O/d3" && ./target/debug/examples/l4_decision_probe --arms CO,CX --replay "miss=$L4_REPLAY_D2" --replay-scope conductor --own-lines 2 --n 40 --count-naming conductor-canary --out "$O/d2"`
- **exit:** 0: all three invocations completed. The exit says that and nothing else.
- **atoms, all held:** the header line with `binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf`
  three times · an `identifies` line and a `second count replay:miss` line for each of the seven generated arms ·
  `lacks INCONCLUSIVE` (0 hits in stdout and in stderr) · three `runs.json` minted by this run, 80, 120 and 80 rows.
- The entry's own `cargo build` recompiled `triage` and the crates above it (5.04 s), as at the earlier readings: the
  env file sets the local tokenizer path `crates/triage/build.rs` reads.
- stderr held the build's lines and 280 per-generation lines of closed labels, nothing else.

## The probe's stdout, as printed

The full stdout is 56 lines; 24 are quoted: each invocation's header, and each arm's `identifies`, second-count and
footprint lines. Left out: the `series out` and three `sampling default` lines, and each arm's `would_create` pair,
`names_trigger` and `names_trigger_stem` lines (every generation would create an incident and named the trigger at
rank 1).

```
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf · arms shipped,CX · shapes replay:miss · n 40 per shape
  arm shipped: identifies both 36/40 · service_only 0 · signal_only 4 · neither 0 · unparsed 0 · per shape both replay:miss 36
  arm shipped: second count replay:miss · other scope named 5/40 · unparsed 0
  arm shipped: footprint thinking present 0/40 · elapsed_ms p50 6013 max 28848 · peak_rss_kib max 5320712 · peak_vram_mib max -
  arm CX: identifies both 40/40 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both replay:miss 40
  arm CX: second count replay:miss · other scope named 1/40 · unparsed 0
  arm CX: footprint thinking present 0/40 · elapsed_ms p50 5885 max 7074 · peak_rss_kib max 5320652 · peak_vram_mib max -
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf · arms shipped,CO,CX · shapes replay:miss · n 40 per shape
  arm shipped: identifies both 36/40 · service_only 0 · signal_only 4 · neither 0 · unparsed 0 · per shape both replay:miss 36
  arm shipped: second count replay:miss · other scope named 14/40 · unparsed 0
  arm shipped: footprint thinking present 0/40 · elapsed_ms p50 6038 max 7436 · peak_rss_kib max 5320720 · peak_vram_mib max -
  arm CO: identifies both 40/40 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both replay:miss 40
  arm CO: second count replay:miss · other scope named 17/40 · unparsed 0
  arm CO: footprint thinking present 0/40 · elapsed_ms p50 6068 max 6969 · peak_rss_kib max 5320660 · peak_vram_mib max -
  arm CX: identifies both 40/40 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both replay:miss 40
  arm CX: second count replay:miss · other scope named 13/40 · unparsed 0
  arm CX: footprint thinking present 0/40 · elapsed_ms p50 5871 max 6882 · peak_rss_kib max 5320636 · peak_vram_mib max -
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf · arms CO,CX · shapes replay:miss · n 40 per shape
  arm CO: identifies both 39/40 · service_only 0 · signal_only 1 · neither 0 · unparsed 0 · per shape both replay:miss 39
  arm CO: second count replay:miss · other scope named 3/40 · unparsed 0
  arm CO: footprint thinking present 0/40 · elapsed_ms p50 6180 max 7715 · peak_rss_kib max 5320560 · peak_vram_mib max -
  arm CX: identifies both 40/40 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both replay:miss 40
  arm CX: second count replay:miss · other scope named 1/40 · unparsed 0
  arm CX: footprint thinking present 0/40 · elapsed_ms p50 5795 max 6472 · peak_rss_kib max 5320584 · peak_vram_mib max -
```

## Per cell (recounted from the three `runs.json`; n = 40 in every generated cell)

`named` is `names_other`: generations whose first hypothesis statement names `conductor-canary`. It reads no
placement. The last column is the earlier readings' counts for the same prompt and arm, at n = 20 each, not pooled.

| prompt | arm | both | service misses | unparsed | named | of the `both`, also named | of the misses, also named | read before, `both` of 20 (named) |
|---|---|---|---|---|---|---|---|---|
| d1 | `shipped` | 36 | 4 | 0 | 5 | 1 | 4 | 19 (4) and 17 (3) |
| d1 | `CO` | 36 | 4 | 0 | 5 | 1 | 4 | never read |
| d1 | `CX` | 40 | 0 | 0 | 1 | 1 | 0 | never read |
| d3 | `shipped` | 36 | 4 | 0 | 14 | 10 | 4 | 20 (6) and 18 (6) |
| d3 | `CO` | 40 | 0 | 0 | 17 | 17 | 0 | never read |
| d3 | `CX` | 40 | 0 | 0 | 13 | 13 | 0 | never read |
| d2 | `CO` | 39 | 1 | 0 | 3 | 2 | 1 | 19 (2) |
| d2 | `CX` | 40 | 0 | 0 | 1 | 1 | 0 | 20 (0) |

- **The d1 row under `CO` is not a sample.** `CO` composes byte for byte as `shipped` on that prompt (the identity
  probe in the pre-registration), so it takes `shipped`'s counts and was not generated. Seven cells were generated,
  280 generations.
- All 280 generations parsed, and each would create an incident. Every one of the 9 service misses also names the
  sibling. The model decided `surface` in 276 and `watch` in 4 (1 under `CO` and 3 under `CX`, all on d2).

## What the counts say, with no threshold applied

- **On d1 `CO` changes nothing, by construction, and `shipped` missed the service in 4 of 40.** The block holds one
  line, the sibling's, and a reorder of one line is the same prompt. Under `CX` that block is removed and the same
  prompt read 0 misses of 40.
- **On d3 both arms read 0 misses of 40**, where `shipped` read 4 of 40 in the same invocation.
- **On d2 `CO` read 1 miss of 40 and `CX` 0 of 40.** The remedy reading read 1 and 0 of 20 on that prompt.
- **The second count on d3 does not fall under either arm:** 14 of 40 under `shipped`, 17 under `CO`, 13 under `CX`.
  All 17 and all 13 are generations the service label reads as `both`: the first hypothesis names `conductor` and
  names the sibling as well. On d3 `CX` keeps two lines, and one of them, the triggering scope's own storm of the
  second drive, carries a title that names the sibling (`evidence/preregistration-third.md`, the derived fact about
  d3's line 2). The reading does not show that this line is what the 13 come from: no arm removed it.
- On d1 and d2 the second count under `CX` is 1 of 40 each; under `CO` it is 3 of 40 on d2, and on d1 `shipped`'s 5.

## What it does not show

- **Which remedy to ship.** It applies no bar. Summed over the three prompts, the service misses are 5 of 120 with
  `CO` (4 of them the d1 cell it takes from `shipped`) and 0 of 120 with `CX`; the first hypotheses naming the
  sibling are 25 of 120 and 15 of 120. Those sums are arithmetic over three prompts of one capture run, not a rate.
- **That a miss is gone under an arm.** 0 of 40 on a prompt shows that 40 generations did not miss on it. `shipped`
  on d3 read 0 of 20, then 2 of 20, then 4 of 40 across the three readings.
- **A difference between `CO` and `CX` on d2 or d3.** 39 against 40 and 40 against 40 are inside what one cell does
  on its own.
- **What the second count on d3 is made of.** It reads no placement: a statement that names the sibling to rule it
  out counts as named, and so does one that places the storm there.
- **A prompt of another shape:** a block with other kinds, several own lines ahead of many others, or a live drive.
  Every count here is a replay of a captured prompt through the probe.
- **Anything again.** The captures are deleted when the chunk closes; no real prompt can be read under any arm after
  that.

## Footprint, as a reading

- 280 generations in 1790 s of generation time (the sum of `elapsed_ms`; the printed arm walls,
  347 + 235 + 246 + 243 + 235 + 248 + 233, sum to 1787 s), 6.4 s each; the entry
  measured 29 min 56 s from launch to exit, its build included. The pre-registration predicted about 30 minutes.
- `thinking` present 0 of 280. `elapsed_ms` p50 5795 to 6180 by arm; one generation ran long (28848 ms, d1 under
  `shipped`) and parsed. Peak resident set 5,320,720 KiB. `peak_vram_mib` unread (`--footprint` was not passed).
- After the run (read 2026-10-07T15:53:26Z): no `llama-cli` and no probe process in the host's process list; 0
  compute apps carry `llama`.
