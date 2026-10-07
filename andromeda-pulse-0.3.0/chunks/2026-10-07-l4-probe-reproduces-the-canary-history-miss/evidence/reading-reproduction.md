# The reproduction reading (plan Step 5) — fired once, as pre-registered

- **Verdict: NOT REPRODUCED** (exit 1). This is a finding, not a pass: the shipped prompt (`v2.6`) did not fail the
  service bar on any of the eight new shapes, so the probe still holds no known-positive for the canary-history miss.
  The chunk stops here for the founder. No candidate arm was built, no remedy reading ran, nothing was committed or
  pushed.
- **Fired:** once, 2026-10-07T10:44:47Z to 11:06:36Z (12:44 to 13:06 local, daytime), on the operator's go given in
  this session (inputs#I12). Never re-run, re-ordered or re-thresholded.
- **Tree:** the chunk base `48714f0` plus one modified file, `pulse-app/examples/l4_decision_probe.rs`, as Steps 2-4
  left it. No file under `crates/` differs from the chunk base (`git diff --name-only 48714f0 -- crates`: 0 lines).
  The probe holds no candidate arm: `--arms CR` is still an unknown arm (gate entry 8, red by design).
- **Gate block before the reading** (the implement run's gate tool): 17 green, 1 red (entry 8, the candidate dry-run,
  its planned baseline), 7 operator legs not run. Entry 5 collected 105 probe pins; entry 16 ran 2742 tests.
- **Series out dir:** `repro-20261007T104447Z` (under the gitignored `target/l4-decision-probe/`): `runs.json`
  (200 rows, sha256 `02921811…ff67`) and `l4-output.gbnf`.
- **Pre-registration:** `evidence/preregistration-reproduction.md`, written 2026-10-07T10:32:16Z, before any run.

## Entry 19 — the slot precondition (driven once by hand)

- **run:** `nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv,noheader`
- **exit:** 0 · **atoms:** `exit 0` held · `lacks llama` held (0 lines carry `llama`) → **green**
- Read at 2026-10-07T10:44:35Z: two compute apps, neither a `llama-cli` — a desktop OSD (10 MiB) and a desktop
  application's GPU process (49 MiB); their full command lines are not copied here. GPU utilisation read 0 %, 960 of
  24576 MiB in use; no listener on 4317 or 4318. The operator's own slot read at 12:43 local says the same
  (inputs#I12).

## Entry 20 — the reading (driven once by hand, bounded at its 3600 s)

- **run** (the plan's text, verbatim):
  `O="target/l4-decision-probe/repro-$(date -u +%Y%m%dT%H%M%SZ)" && echo "series out $O" && unset ANDROMEDA_PULSE_HARDWARE_PROFILE ANDROMEDA_PULSE_L4_ALLOW_ROOT ANDROMEDA_PULSE_L4_DETERMINISTIC && . "$HOME/dev/projects/additional/pc-overseer/l4-env.sh" && cargo build -p pulse-app --example l4_decision_probe && ./target/debug/examples/l4_decision_probe --arms shipped --shapes S7,S8,S9,S10,S11,S12,S13,S14,S15,S16 --n 20 --reproduce-misses 2 --out "$O"`
- **exit:** 1 (NOT REPRODUCED). The entry asserts no exit; exit 0 and exit 1 are both a recorded verdict.
- **atoms, all held:** the header line with `binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf` ·
  `arm shipped: reproduce S16` · `l4-decision-probe: reproduction verdict:` · `lacks INCONCLUSIVE` (0 hits in stdout
  and in stderr).
- **artifact:** `target/l4-decision-probe/` fresh — the out dir was minted by this run, `runs.json` written at its end.
- The env file sourced is the one inputs#I9 snapshots: `cmp` of the live file against the copy exited 0 before the
  run (sha256 `0bc0706d…be2b`).
- The entry's own `cargo build` recompiled `triage` and the crates above it (4.57 s): the env file sets the local
  tokenizer path that `crates/triage/build.rs` reads. The source tree is the one the gate block read green; the probe
  composes no prompt through the tokenizer.

## The probe's stdout, as printed

```
series out target/l4-decision-probe/repro-20261007T104447Z
l4-decision-probe: sampling default
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf · arms shipped · shapes S7,S8,S9,S10,S11,S12,S13,S14,S15,S16 · n 20 per shape
arm shipped: would_create 200/200 · decision 200/0/0 · severity_none 0 · resolution_summary 0
  arm shipped: per shape would_create S7 20 S8 20 S9 20 S10 20 S11 20 S12 20 S13 20 S14 20 S15 20 S16 20 · failed 0 · first_keys schema_version>prompt_version>title=200 · distinct outputs S7 20 S8 20 S9 20 S10 20 S11 20 S12 20 S13 20 S14 20 S15 20 S16 20 · wall 1305s
  arm shipped: names_trigger rank1 200/200 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S7 20 S8 20 S9 20 S10 20 S11 20 S12 20 S13 20 S14 20 S15 20 S16 20
  arm shipped: names_trigger_stem rank1 200/200 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S7 20 S8 20 S9 20 S10 20 S11 20 S12 20 S13 20 S14 20 S15 20 S16 20
  arm shipped: identifies both 199/200 · service_only 0 · signal_only 1 · neither 0 · unparsed 0 · per shape both S7 20 S8 20 S9 20 S10 20 S11 20 S12 20 S13 20 S14 20 S15 20 S16 19
  arm shipped: reproduce S7 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S8 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S9 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S10 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S11 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S12 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S13 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S14 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S15 CLEAN · misses 0/20 · unparsed 0
  arm shipped: reproduce S16 CLEAN · misses 1/20 · unparsed 0
  arm shipped: footprint thinking present 0/200 · elapsed_ms p50 6320 max 12246 · peak_rss_kib max 5320840 · peak_vram_mib max -
l4-decision-probe: reproduction verdict: NOT REPRODUCED · shapes none
```

## Per shape (from `runs.json`, by the pre-registered rule, K = 2)

| shape | what it varies against S8 | n | both | service_only | signal_only | neither | unparsed | misses | reading | elapsed_ms p50 / max |
|---|---|---|---|---|---|---|---|---|---|---|
| S7 (control) | no corpus block | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6279 / 8618 |
| S8 (control) | two sibling retry-storm lines | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6377 / 8970 |
| S9 | count | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6738 / 10865 |
| S10 | kind mix | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6682 / 12246 |
| S11 | position, against S10 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6691 / 9659 |
| S12 | fingerprint | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6112 / 6751 |
| S13 | title form | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6126 / 6573 |
| S14 | count, kind mix, fingerprint, title form | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6136 / 7284 |
| S15 | position, against S14 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | CLEAN | 6174 / 7247 |
| S16 | scope mix | 20 | 19 | 0 | 1 | 0 | 0 | 1 | CLEAN | 6237 / 7013 |

All 200 generations parsed, decided `surface` and would create an incident. Neither control reproduces, so no
single-property comparison is voided.

## What the reading shows

- Over S9-S16 the first hypothesis named the cue's `scope_id` (`conductor`) as a whole word in 159 of 160
  generations. No shape reached the pre-registered 2 misses of 20; every one of the eight reads CLEAN.
- The one miss is S16 run 12, labelled `signal_only`: it parsed, named a retry token, and did not name `conductor`
  as a whole word. S16 is the mixed-scope shape, the one the third drive's capture bounds. One miss of 20 is below
  the threshold and is not a reproduction.
- The shape the founder named (S14) read 20 of 20.

## What it does not show

- It does not show that count, kind mix, position, fingerprint, title form or scope mix have no influence on the miss.
  At n = 20 a clean shape shows that 20 generations did not miss on it.
- It does not show whether S16's single miss is the canary-history miss at a low rate or noise. This reading cannot
  tell the two apart, and it was pre-registered not to be re-run or re-thresholded.
- It does not show what the third drive's digest held. That digest stays unmeasured (cell-encrypted). Every new shape
  is synthetic and keeps S7's two services rows, S7's single cue, no active-incident reference and the probe's fixed
  project context. Whatever the real digest carried outside its corpus block was held at S7's values here, so it was
  not varied.
- It does not grade a remedy: none was built, by the plan's order.

## Footprint, as a reading

- 200 generations in 1305 s of arm wall time (sum of `elapsed_ms` 1,304,862; 6.5 s each). The plan predicted about
  21 minutes at 6.2 s each; the entry measured 21 min 49 s from launch to exit, its build included.
- `thinking` present 0 of 200. `elapsed_ms` p50 6320, max 12246. Peak resident set 5,320,840 KiB. `peak_vram_mib`
  unread (`--footprint` was not passed, as the entry is written).
- After the run: no `llama-cli` and no probe process in the host's process list; 0 compute apps carry `llama`.
