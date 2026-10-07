# The remedy reading (plan Step 14) — fired once, as pre-registered

- **Verdict: SELECTED, arm `CO`** (exit 0): the triggering scope's own corpus lines first, then the others, the
  order inside each group kept. The known-positive HELD, n = 20 on the remedy prompt.
- **Fired:** once, 2026-10-07T13:35:32Z to 14:29:26Z (15:35 to 16:29 local, daytime), on the operator's go given in
  this session (inputs#I23). Never re-run, re-ordered or re-thresholded.
- **Tree:** the chunk base `48714f0` plus the probe's two files (probe file `e9b409d7…b2a7`, module `db364e49…11dd`).
  No file under `crates/` differed from the chunk base when the reading ran (`git diff --name-only 48714f0 --
  crates`: 0 lines); the product change of Step 15 was written after it.
- **Gate block before the reading** (this run's gate tool, on that tree): 19 green, 0 red, 12 operator legs not run.
  Entry 5 collected 153 probe pins, entry 17 ran 2790.
- **Series out dir:** `remedy-20261007T133532Z` (under the gitignored `target/l4-decision-probe/`): `runs.json`
  (500 rows, sha256 `c4f439d5128625340b04767769222fedd8e7a27d87ba5a466d33a011c7833975`) and `l4-output.gbnf`.
- **Pre-registration:** `evidence/preregistration-remedy.md`, written 2026-10-07T13:31:14Z and amended
  2026-10-07T13:34:30Z on the operator's word, both before any generation.
- **The labels are roles.** In this reading `miss` names the second drive's captured prompt (the one that reproduced
  in the replay reading, read there as `d2`), `control` the first drive's, `d3` the third drive's, recorded only. No
  captured drive missed live. No text of a captured prompt and no model text is in this file.

## Entry 26 — the slot precondition (driven once by hand)

- **run:** `nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv,noheader && echo second-slot`
- **exit:** 0 · **atoms:** `exit 0` held · `contains second-slot` held · `lacks llama` held (0 lines carry `llama`)
  → **green**
- Read at 2026-10-07T13:35:28Z: two compute apps, neither a `llama-cli` — a desktop OSD (10 MiB) and a desktop
  application's GPU process (49 MiB). GPU utilisation read 3 %, 960 of 24576 MiB in use; no listener on 4317 or
  4318; no `llama` or probe process in the host's process list. The operator's own slot read, given with the go,
  says the same (inputs#I23).

## Entry 27 — the reading (driven once by hand, bounded at its 7200 s)

- **run** (the plan's text with the pre-registration's amendment 2 applied):
  `O="target/l4-decision-probe/remedy-$(date -u +%Y%m%dT%H%M%SZ)" && echo "series out $O" && unset ANDROMEDA_PULSE_HARDWARE_PROFILE ANDROMEDA_PULSE_L4_ALLOW_ROOT ANDROMEDA_PULSE_L4_DETERMINISTIC && . "$HOME/dev/projects/additional/pc-overseer/l4-env.sh" && cargo build -p pulse-app --example l4_decision_probe && ./target/debug/examples/l4_decision_probe --arms shipped,CR,CO,CC,CX --replay "miss=$L4_REPLAY_MISS,control=$L4_REPLAY_CONTROL,d3=$L4_REPLAY_D3" --replay-scope conductor --own-lines "$L4_REPLAY_OWN_LINES" --remedy-from target/l4-decision-probe/replay-20261007T131536Z --remedy-read-as d2 --n 20 --bar-sibling 38 --bar-ordinary 72 --count-naming conductor-canary --out "$O"`
- **exit:** 0 (SELECTED). The entry asserts no exit; exit 0 and exit 1 are both a recorded verdict.
- **atoms, all held:** the header line with `binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf` ·
  `l4-decision-probe: remedy prompt:` · `l4-decision-probe: known-positive:` · `l4-decision-probe: derived shape:` ·
  `l4-decision-probe: remedy verdict:` · `lacks INCONCLUSIVE` (0 hits in stdout and in stderr).
- **artifact:** `target/l4-decision-probe/` fresh — the out dir was minted by this run, `runs.json` written at its end.
- The three prompts hashed before the run as the pre-registration names them (`21c39d8c…09c8`, `1d953dfb…22ec`,
  `83b8a3ea…661e`); the env file compared equal to the copy inputs#I9 snapshots.
- The entry's own `cargo build` recompiled `triage` and the crates above it (4.32 s), as at both earlier readings.
- stderr held the build's lines and 500 per-generation lines of closed labels, nothing else.

## The probe's stdout, as printed (the verdict lines and each arm's counts)

The full stdout is 100 lines; 48 are quoted below: the header and the skip lines, each arm's `identifies` and
footprint lines, the bar and verdict lines, and the second-count lines of the cells this record reads. Left out:
each arm's `would_create` pair, `names_trigger` and `names_trigger_stem` lines (every generation would create an
incident and named the trigger at rank 1), and the second-count lines of the six guard shapes under `shipped`
(0 named each), of `S4` under `CR` (0 named) and of every cell an arm did not generate (`0/0`).

```
series out target/l4-decision-probe/remedy-20261007T133532Z
l4-decision-probe: sampling default
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf · arms shipped,CR,CO,CC,CX · shapes S1,S2,S3,S4,S7,S8,S17,replay:miss,replay:control,replay:d3 · n 20 per shape
l4-decision-probe: remedy prompt: replay:miss n 20 (replay reading 11/20 as replay:d2) · from replay-20261007T131536Z
  arm CO replay:control: skipped · own lines unknown
  arm CO replay:d3: skipped · own lines unknown
  arm CX replay:control: skipped · own lines unknown
  arm CX replay:d3: skipped · own lines unknown
  arm CR: composes as shipped on S1,S2,S3,S7 · takes its counts there
  arm CO: composes as shipped on S1,S2,S3,S4,S7,S8 · takes its counts there
  arm CC: composes as shipped on S1,S2,S3,S4,S7,S8 · takes its counts there
  arm CX: composes as shipped on S1,S2,S3,S7,S8 · takes its counts there
  arm shipped: identifies both 185/200 · service_only 0 · signal_only 15 · neither 0 · unparsed 0 · per shape both S1 20 S2 20 S3 20 S4 20 S7 20 S8 20 S17 19 replay:miss 11 replay:control 17 replay:d3 18
  arm shipped: second count S17 · other scope named 1/20 · unparsed 0
  arm shipped: second count replay:miss · other scope named 13/20 · unparsed 0
  arm shipped: second count replay:control · other scope named 3/20 · unparsed 0
  arm shipped: second count replay:d3 · other scope named 6/20 · unparsed 0
  arm shipped: footprint thinking present 0/200 · elapsed_ms p50 6137 max 24391 · peak_rss_kib max 5320656 · peak_vram_mib max -
  arm CR: identifies both 113/120 · service_only 0 · signal_only 7 · neither 0 · unparsed 0 · per shape both S1 0 S2 0 S3 0 S4 20 S7 0 S8 20 S17 20 replay:miss 16 replay:control 17 replay:d3 20
  arm CR: second count S8 · other scope named 2/20 · unparsed 0
  arm CR: second count S17 · other scope named 1/20 · unparsed 0
  arm CR: second count replay:miss · other scope named 7/20 · unparsed 0
  arm CR: second count replay:control · other scope named 3/20 · unparsed 0
  arm CR: second count replay:d3 · other scope named 3/20 · unparsed 0
  arm CR: footprint thinking present 0/120 · elapsed_ms p50 6145 max 15337 · peak_rss_kib max 5320672 · peak_vram_mib max -
  arm CO: identifies both 39/40 · service_only 0 · signal_only 1 · neither 0 · unparsed 0 · per shape both S1 0 S2 0 S3 0 S4 0 S7 0 S8 0 S17 20 replay:miss 19 replay:control 0 replay:d3 0
  arm CO: second count S17 · other scope named 2/20 · unparsed 0
  arm CO: second count replay:miss · other scope named 2/20 · unparsed 0
  arm CO: footprint thinking present 0/40 · elapsed_ms p50 6142 max 7132 · peak_rss_kib max 5320596 · peak_vram_mib max -
  arm CC: identifies both 70/80 · service_only 0 · signal_only 10 · neither 0 · unparsed 0 · per shape both S1 0 S2 0 S3 0 S4 0 S7 0 S8 0 S17 20 replay:miss 14 replay:control 17 replay:d3 19
  arm CC: second count S17 · other scope named 1/20 · unparsed 0
  arm CC: second count replay:miss · other scope named 10/20 · unparsed 0
  arm CC: second count replay:control · other scope named 3/20 · unparsed 0
  arm CC: second count replay:d3 · other scope named 6/20 · unparsed 0
  arm CC: footprint thinking present 0/80 · elapsed_ms p50 5859 max 30676 · peak_rss_kib max 5320688 · peak_vram_mib max -
  arm CX: identifies both 60/60 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both S1 0 S2 0 S3 0 S4 20 S7 0 S8 0 S17 20 replay:miss 20 replay:control 0 replay:d3 0
  arm CX: second count S4 · other scope named 0/20 · unparsed 0
  arm CX: second count S17 · other scope named 0/20 · unparsed 0
  arm CX: second count replay:miss · other scope named 0/20 · unparsed 0
  arm CX: footprint thinking present 0/60 · elapsed_ms p50 6125 max 7118 · peak_rss_kib max 5319264 · peak_vram_mib max -
  arm CR: remedy bar NOT MET · guard HOLDS · not both 4/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
  arm CO: remedy bar MET · guard HOLDS · not both 1/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
  arm CC: remedy bar NOT MET · guard HOLDS · not both 6/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
  arm CX: remedy bar MET · guard HOLDS · not both 0/20 (allowance 1) · sibling both 40/40 (min 38, shipped 40) · ordinary both 80/80 (min 72, shipped 80)
l4-decision-probe: known-positive: HELD · shipped misses 9/20 on replay:miss (min 3)
l4-decision-probe: remedy selection: CO · order CR,CO,CC,CX
l4-decision-probe: derived shape: S17 CLEAN · not read against a selection · shipped misses 1/20 · unparsed 0 · not both CR 0/20 CO 0/20 CC 0/20 CX 0/20
l4-decision-probe: remedy verdict: SELECTED · arm CO
```

## Per arm and prompt (from `runs.json`; n = 20 in every cell)

`named` is the second count: generations whose first hypothesis statement names `conductor-canary`. It selected
nothing.

| arm | prompt | both | misses | unparsed | named | of the `both`, also named |
|---|---|---|---|---|---|---|
| `shipped` | remedy prompt (d2) | 11 | 9 | 0 | 13 | 4 |
| `shipped` | control (d1) | 17 | 3 | 0 | 3 | 0 |
| `shipped` | d3 | 18 | 2 | 0 | 6 | 4 |
| `shipped` | `S17` | 19 | 1 | 0 | 1 | 0 |
| `CR` | remedy prompt (d2) | 16 | 4 | 0 | 7 | 3 |
| `CR` | control (d1) | 17 | 3 | 0 | 3 | 0 |
| `CR` | d3 | 20 | 0 | 0 | 3 | 3 |
| `CR` | `S17` | 20 | 0 | 0 | 1 | 1 |
| `CO` | remedy prompt (d2) | 19 | 1 | 0 | 2 | 1 |
| `CO` | control (d1) | skipped: own lines not pre-registered | | | | |
| `CO` | d3 | skipped: own lines not pre-registered | | | | |
| `CO` | `S17` | 20 | 0 | 0 | 2 | 2 |
| `CC` | remedy prompt (d2) | 14 | 6 | 0 | 10 | 4 |
| `CC` | control (d1) | 17 | 3 | 0 | 3 | 0 |
| `CC` | d3 | 19 | 1 | 0 | 6 | 5 |
| `CC` | `S17` | 20 | 0 | 0 | 1 | 1 |
| `CX` | remedy prompt (d2) | 20 | 0 | 0 | 0 | 0 |
| `CX` | control (d1) | skipped: own lines not pre-registered | | | | |
| `CX` | d3 | skipped: own lines not pre-registered | | | | |
| `CX` | `S17` | 20 | 0 | 0 | 0 | 0 |

Every service miss in the reading (33 of 33) also names the sibling. All 500 generations parsed.

## The guards (20 generations a shape; a candidate takes `shipped`'s counts where it composes identically)

| arm | `S7` + `S8` both | `S1`-`S4` both | generated on | guard |
|---|---|---|---|---|
| `shipped` | 40 / 40 | 80 / 80 | all six | the baseline |
| `CR` | 40 / 40 | 80 / 80 | `S4`, `S8` (20 and 20 `both`; 2 of 20 named on `S8`) | HOLDS |
| `CO` | 40 / 40 | 80 / 80 | none | HOLDS |
| `CC` | 40 / 40 | 80 / 80 | none | HOLDS |
| `CX` | 40 / 40 | 80 / 80 | `S4` (20 `both`) | HOLDS |

## The verdict, by the pre-registered rule

- **Known-positive HELD:** `shipped` read 9 service misses of 20 on the remedy prompt, against a minimum of 3
  (allowance 1 plus 2). The replay reading read 11 on the same prompt.
- **Bar** (not `both` within 1 of 20 on the remedy prompt, 38 of 40 on `S7` + `S8`, 72 of 80 on `S1`-`S4`):
  `CR` NOT MET (4), `CO` MET (1), `CC` NOT MET (6), `CX` MET (0). No guard tripped.
- **Selection:** in the order `CR`, `CO`, `CC`, `CX`, the first with its bar met and its guard holding is `CO`.
  `CX` also met the bar and reads cleaner (0 of 20, and 0 named), and the order, least change first, puts `CO`
  before it.

## The derived shape `S17`

- **CLEAN under `shipped`:** 1 service miss of 20. It did not reproduce, so it is not a known-positive, and the
  COVERED / NOT COVERED reading does not apply ("not read against a selection").
- Every candidate read 20 of 20 `both` on it.
- **The probe therefore keeps no durable known-positive.** The one the readings found is the second drive's captured
  prompt, which is deleted when the chunk closes. `S17` carries that prompt's services order, its active-incident
  count and its block's kinds, ages, statuses and fingerprint pattern, with the builder's own titles, rate, project
  and ids; it read 1 of 20 where the captured prompt read 9 and 11. What the captured prompt has and `S17` lacks is
  therefore among: the titles' wording, the exact values of the services rows, the project line and the citable id.
  The reading does not say which.

## What the reading shows

- On the real prompt that reproduces, putting the triggering scope's own line before the sibling's brings the
  service misses from 9 of 20 to 1 of 20, and the generations that name the sibling at all from 13 to 2. The remedy
  clears the bar and brings the second count down with it.
- Removing the other scopes' lines (`CX`) reads 0 misses and 0 named. Restating the cue after the block (`CR`) and
  keeping two lines (`CC`) do not meet the bar: 4 and 6 not `both`.
- No candidate moved a guard shape: every arm read every guard shape 20 of 20.

## What it does not show

- **The third drive's prompt under the selected arm.** `CO` was skipped on `d3`, its own lines not being
  pre-registered. Under `shipped` it read 2 misses of 20 here against 0 of 20 in the replay reading; under `CR` 0
  and under `CC` 1. Whether the reorder helps, leaves or disturbs that prompt is unmeasured.
- **The first drive's prompt under the selected arm**, for the same reason. Its block holds one line, the
  sibling's. If that line is not the triggering scope's own, the reorder changes nothing in that block; this
  reading did not pre-register that line's ownership, so it is stated as a consequence of the rule, not as a
  measurement. `shipped`, `CR` and `CC` each read 3 misses of 20 on it.
- **That the miss is gone.** One prompt at n = 20: 1 of 20 is within the allowance, not zero.
- **Stability across readings.** The same prompt under the same arm read 0 then 2 misses of 20 (d3) and 1 then 3
  (d1) in two readings an hour apart. At n = 20 a clean prompt shows that 20 generations did not miss on it.
- **A live drive.** Every count here is a replay of a captured prompt or a synthetic shape through the probe; the
  product's own pipeline was not driven.
- The second count reads no placement: a statement that names the sibling to rule it out counts as named.

## Footprint, as a reading

- 500 generations in 3227 s of arm wall time (1278 + 762 + 247 + 572 + 368), 6.5 s each; the entry measured 53 min
  54 s from launch to exit, its build included. The pre-registration predicted about 52 minutes.
- `thinking` present 0 of 500. `elapsed_ms` p50 5859 to 6145 by arm; three generations ran long (24391, 15337 and
  30676 ms), each parsed. Peak resident set 5,320,688 KiB. `peak_vram_mib` unread (`--footprint` was not passed).
- After the run (read 2026-10-07T14:30:00Z): no `llama-cli` and no probe process in the host's process list; 0
  compute apps carry `llama`.
