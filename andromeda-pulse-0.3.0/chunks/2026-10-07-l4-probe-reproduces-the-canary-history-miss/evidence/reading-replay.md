# The replay reading (plan Step 12) — fired once, as pre-registered

- **Verdict: REPRODUCED** (exit 0), on `d2`. The second drive's captured prompt, replayed verbatim under the shipped
  arm, read 11 service misses of 20. The prompt in the `miss` role (d3, the position that missed in the fourth and
  fifth series) read 0 misses of 20 and is CLEAN; the control (d1) read 1 of 20 and is CLEAN.
- By the pre-registration's amendment 4, the prompt that takes the `miss` role in the remedy reading is therefore
  `d2`: `miss` (d3) did not reproduce.
- **Fired:** once, 2026-10-07T13:15:36Z to 13:21:52Z (15:15 to 15:21 local, daytime), on the operator's go given in
  this session (inputs#I21). Never re-run, re-ordered or re-thresholded.
- **Tree:** the chunk base `48714f0` plus the probe's two files; the probe file hashed `0c0aa2d1…93cd` before the
  run. No file under `crates/` differs from the chunk base (`git diff --name-only 48714f0 -- crates`: 0 lines).
- **Gate block before the reading** (this run's gate tool, on that tree): 19 green, 0 red, 12 operator legs not run.
  Entry 5 collected 150 probe pins, entry 4 ran 492 tests, entry 17 ran 2787.
- **Series out dir:** `replay-20261007T131536Z` (under the gitignored `target/l4-decision-probe/`): `runs.json`
  (60 rows, sha256 `178c42839ecca8b2acddb7a74ce01499e3f2722ee3c7e957d1a228d9d63c878c`) and `l4-output.gbnf`.
- **Pre-registration:** `evidence/preregistration-replay.md`, written 2026-10-07T13:13:30Z, before any generation.
  Which captured invocation each label names, by directory name and sha256, is in that file.
- **No captured drive missed** (inputs#I20 `:274-276`). `miss` is a role in this reading, not a fact about the
  drive it names. No text of a captured prompt and no model text is in this file: the probe prints closed labels
  and counts.

## Entry 24 — the slot precondition (driven once by hand)

- **run:** `nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv,noheader && echo replay-slot`
- **exit:** 0 · **atoms:** `exit 0` held · `contains replay-slot` held · `lacks llama` held (0 lines carry `llama`)
  → **green**
- Read at 2026-10-07T13:15:31Z: two compute apps, neither a `llama-cli` — a desktop OSD (10 MiB) and a desktop
  application's GPU process (49 MiB); their full command lines are not copied here. GPU utilisation read 5 %, 960 of
  24576 MiB in use; no listener on 4317 or 4318; no `llama` or probe process in the host's process list. The
  operator's own slot read, given with the go, says the same (inputs#I21).

## Entry 25 — the reading (driven once by hand, bounded at its 1800 s)

- **run** (the plan's text with the pre-registration's amendment 7 applied: the third prompt and the second count):
  `O="target/l4-decision-probe/replay-$(date -u +%Y%m%dT%H%M%SZ)" && echo "series out $O" && unset ANDROMEDA_PULSE_HARDWARE_PROFILE ANDROMEDA_PULSE_L4_ALLOW_ROOT ANDROMEDA_PULSE_L4_DETERMINISTIC && . "$HOME/dev/projects/additional/pc-overseer/l4-env.sh" && cargo build -p pulse-app --example l4_decision_probe && ./target/debug/examples/l4_decision_probe --arms shipped --replay "miss=$L4_REPLAY_MISS,control=$L4_REPLAY_CONTROL,d2=$L4_REPLAY_D2" --replay-scope conductor --n 20 --reproduce-misses 2 --count-naming conductor-canary --out "$O"`
- **exit:** 0 (REPRODUCED). The entry asserts no exit; exit 0 and exit 1 are both a recorded verdict.
- **atoms, all held:** the header line with `binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf` ·
  `arm shipped: reproduce replay:miss` · `l4-decision-probe: reproduction verdict:` · `lacks INCONCLUSIVE` (0 hits in
  stdout and in stderr).
- **artifact:** `target/l4-decision-probe/` fresh — the out dir was minted by this run, `runs.json` written at its end.
- The env file sourced is the one inputs#I9 snapshots: `cmp` of the live file against the copy exited 0 before the
  run.
- The entry's own `cargo build` recompiled `triage` and the crates above it (5.41 s), as at the first reading: the
  env file sets the local tokenizer path that `crates/triage/build.rs` reads. The source tree is the one the gate
  block read green.
- stderr held the build's lines and 60 per-generation lines of closed labels, nothing else.

## The probe's stdout, as printed

```
series out target/l4-decision-probe/replay-20261007T131536Z
l4-decision-probe: sampling default
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf · arms shipped · shapes replay:miss,replay:control,replay:d2 · n 20 per shape
arm shipped: would_create 60/60 · decision 60/0/0 · severity_none 0 · resolution_summary 0
  arm shipped: per shape would_create replay:miss 20 replay:control 20 replay:d2 20 · failed 0 · first_keys schema_version>prompt_version>title=60 · distinct outputs replay:miss 20 replay:control 20 replay:d2 20 · wall 370s
  arm shipped: names_trigger rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 replay:miss 20 replay:control 20 replay:d2 20
  arm shipped: names_trigger_stem rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 replay:miss 20 replay:control 20 replay:d2 20
  arm shipped: identifies both 48/60 · service_only 0 · signal_only 12 · neither 0 · unparsed 0 · per shape both replay:miss 20 replay:control 19 replay:d2 9
  arm shipped: reproduce replay:miss CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce replay:control CLEAN · misses 1/20 · unparsed 0
  arm shipped: reproduce replay:d2 REPRODUCES · misses 11/20 · unparsed 0
  arm shipped: second count replay:miss · other scope named 6/20 · unparsed 0
  arm shipped: second count replay:control · other scope named 4/20 · unparsed 0
  arm shipped: second count replay:d2 · other scope named 15/20 · unparsed 0
  arm shipped: footprint thinking present 0/60 · elapsed_ms p50 6107 max 7751 · peak_rss_kib max 5320648 · peak_vram_mib max -
l4-decision-probe: reproduction verdict: REPRODUCED · replay:d2
```

## Per prompt (from `runs.json`, by the pre-registered rule, K = 2)

| label | drive | role | rendered corpus lines | n | both | service_only | signal_only | neither | unparsed | misses | reading | elapsed_ms p50 / max |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `miss` | d3 | candidate | 5 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6448 / 7751 |
| `control` | d1 | control | 1 | 20 | 19 | 0 | 1 | 0 | 0 | 1 | CLEAN | 5942 / 6435 |
| `d2` | d2 | candidate | 3 | 20 | 9 | 0 | 11 | 0 | 0 | 11 | REPRODUCES | 5958 / 6741 |

All 60 generations parsed, decided `surface` and would create an incident. The control does not reproduce, so the
difference against it is not voided.

## The second count (recorded beside the bar; it selected nothing)

The generations whose first hypothesis statement names `conductor-canary`, by the pre-registered detector, and how
they fall against the service label:

| label | drive | named | not named | unparsed | `both` and named | `both` and not named | service miss and named | service miss and not named |
|---|---|---|---|---|---|---|---|---|
| `miss` | d3 | 6 | 14 | 0 | 6 | 14 | 0 | 0 |
| `control` | d1 | 4 | 16 | 0 | 3 | 16 | 1 | 0 |
| `d2` | d2 | 15 | 5 | 0 | 4 | 5 | 11 | 0 |

## What the reading shows

- The real prompt of the second drive, as the product handed it to the model, makes the shipped prompt (`v2.6`) miss
  the service in 11 of 20 generations. That is a known-positive on a real prompt, at a rate well above the threshold.
- The real prompt of the third drive, the position that missed live in the fourth and fifth series, did not miss in
  20 generations. Its first hypothesis named `conductor` as a whole word every time.
- Every one of the 12 service misses in the reading (11 on d2, 1 on d1) also names the sibling: none names neither
  service.
- Among the 48 generations the service bar reads as `both`, 13 also name the sibling in the first hypothesis (6 of
  20 on d3, 3 of 19 on d1, 4 of 9 on d2). The bar does not see those. On d2 only 5 of 20 first hypotheses name
  `conductor` and do not mention the sibling.
- The three prompts differ only inside the digest (`evidence/section-diff.md`): d2 differs from d3 in the
  `OVERALL:` line and the corpus block (3 lines against 5), and from d1 in those and the services table.

## What it does not show

- It does not show why the third drive missed live in two series. No prompt of a missing drive was captured; this
  run's third drive was graded `Identified`, and its prompt read clean here. At n = 20 a clean prompt shows that 20
  generations did not miss on it.
- It does not show what in the second drive's digest carries the miss. The section rows say where the three prompts
  differ, not what the differing lines say; that is read at Step 13.
- It does not show that the live second drive missed: Conductor graded it `Identified` (inputs#I20 `:275`). One live
  generation of a prompt that misses 11 times in 20 passes about half the time.
- The second count does not read placement. A statement that names the sibling to rule it out counts as `named`.
- It does not grade a remedy: none has been read.

## Footprint, as a reading

- 60 generations in 370 s of arm wall time (sum of `elapsed_ms` 370,135; 6.2 s each). The entry measured 6 min 16 s
  from launch to exit, its build included; the plan predicted about 5 minutes for 40 generations.
- `thinking` present 0 of 60. `elapsed_ms` p50 6107, max 7751. Peak resident set 5,320,648 KiB. `peak_vram_mib`
  unread (`--footprint` was not passed, as the entry is written).
- After the run (read 2026-10-07T13:22:44Z): no `llama-cli` and no probe process in the host's process list; 0
  compute apps carry `llama`.
