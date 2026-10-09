# The reading (plan Step 11) — fired once, as pre-registered

- **Verdict: PASS.** Selection `shipped`. Regression guard HOLDS.
- **Fired:** once, 2026-10-07T05:43:42Z to 06:08:42Z (07:43 to 08:08 local, daytime), on the founder's live word
  relayed by the overseer (inputs#I9). Never re-run, re-ordered or re-thresholded.
- **Tree:** the chunk base `b5138e2` plus this chunk's four modified files, unchanged since every non-leg gate entry
  read green on it (17 of 17, the implement run's gate block).
- **Series out dir:** `service-20261007T054342Z` (under the gitignored `target/l4-decision-probe/`): `runs.json`
  (240 rows) and `l4-output.gbnf`.
- **Pre-registration:** `evidence/preregistration.md`, written 2026-10-06T23:55:48Z, before any run.

## Entry 18 — the slot precondition (driven once by hand)

- **run:** `nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv,noheader`
- **exit:** 0 · **atoms:** `exit 0` held · `lacks llama` held (0 lines carry `llama`) → **green**
- Read at 2026-10-07T05:43:31Z: two compute apps, neither a `llama-cli` — a desktop OSD (10 MiB) and a desktop
  application's GPU process (48 MiB); their full command lines are host paths and are not copied here. GPU
  utilisation read 1 %, 1205 of 24576 MiB in use.

## Entry 19 — the reading (driven once by hand, bounded at its 7200 s)

- **run** (the plan's text, verbatim):
  `O="target/l4-decision-probe/service-$(date -u +%Y%m%dT%H%M%SZ)" && echo "series out $O" && unset ANDROMEDA_PULSE_HARDWARE_PROFILE ANDROMEDA_PULSE_L4_ALLOW_ROOT ANDROMEDA_PULSE_L4_DETERMINISTIC && . "$HOME/dev/projects/additional/pc-overseer/l4-env.sh" && cargo build -p pulse-app --example l4_decision_probe && ./target/debug/examples/l4_decision_probe --arms shipped,ns,L,LI --shapes S1,S2,S3,S4,S7,S8 --n 10 --bar-sibling 19 --bar-ordinary 36 --out "$O"`
- **exit:** 0 (PASS). The entry asserts no exit; exit 0 and exit 1 are both a recorded verdict.
- **atoms, all held:** the header line with `binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf` ·
  `arm shipped: bar` · `arm LI: bar` · `l4-decision-probe: selection:` · `l4-decision-probe: service verdict:` ·
  `l4-decision-probe: regression guard:` · `lacks INCONCLUSIVE` (0 hits in stdout and in stderr).
- **artifact:** `target/l4-decision-probe/` fresh — the out dir was minted by this run.
- The env file sourced is the one inputs#I6 snapshots: its sha256 read `0bc0706d…be2b` before the run, equal to the
  manifest's.

## The probe's stdout, as printed

```
series out target/l4-decision-probe/service-20261007T054342Z
l4-decision-probe: sampling default
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model gemma-4-E4B-it-Q4_K_M.gguf · arms shipped,ns,L,LI · shapes S1,S2,S3,S4,S7,S8 · n 10 per shape
arm shipped: would_create 60/60 · decision 60/0/0 · severity_none 0 · resolution_summary 0
  arm shipped: per shape would_create S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · failed 0 · first_keys schema_version>prompt_version>title=60 · distinct outputs S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · wall 399s
  arm shipped: names_trigger rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm shipped: names_trigger_stem rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm shipped: identifies both 60/60 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm shipped: bar MET · sibling both 20/20 (min 19) · ordinary both 40/40 (min 36)
  arm shipped: footprint thinking present 0/60 · elapsed_ms p50 6411 max 10099 · peak_rss_kib max 5320776 · peak_vram_mib max -
arm ns: would_create 60/60 · decision 59/0/1 · severity_none 0 · resolution_summary 0
  arm ns: per shape would_create S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · failed 0 · first_keys schema_version>prompt_version>title=60 · distinct outputs S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · wall 362s
  arm ns: names_trigger rank1 57/60 · elsewhere 3 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 8 S3 10 S4 9 S7 10 S8 10
  arm ns: names_trigger_stem rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm ns: identifies both 58/60 · service_only 0 · signal_only 2 · neither 0 · unparsed 0 · per shape both S1 10 S2 10 S3 8 S4 10 S7 10 S8 10
  arm ns: bar MET · sibling both 20/20 (min 19) · ordinary both 38/40 (min 36)
  arm ns: footprint thinking present 0/60 · elapsed_ms p50 6029 max 7348 · peak_rss_kib max 5320744 · peak_vram_mib max -
arm L: would_create 60/60 · decision 60/0/0 · severity_none 0 · resolution_summary 0
  arm L: per shape would_create S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · failed 0 · first_keys schema_version>prompt_version>title=60 · distinct outputs S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · wall 363s
  arm L: names_trigger rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm L: names_trigger_stem rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm L: identifies both 60/60 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm L: bar MET · sibling both 20/20 (min 19) · ordinary both 40/40 (min 36)
  arm L: footprint thinking present 0/60 · elapsed_ms p50 5944 max 7301 · peak_rss_kib max 5320760 · peak_vram_mib max -
arm LI: would_create 60/60 · decision 60/0/0 · severity_none 0 · resolution_summary 0
  arm LI: per shape would_create S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · failed 0 · first_keys schema_version>prompt_version>title=60 · distinct outputs S1 10 S2 10 S3 10 S4 10 S7 10 S8 10 · wall 368s
  arm LI: names_trigger rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm LI: names_trigger_stem rank1 60/60 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm LI: identifies both 60/60 · service_only 0 · signal_only 0 · neither 0 · unparsed 0 · per shape both S1 10 S2 10 S3 10 S4 10 S7 10 S8 10
  arm LI: bar MET · sibling both 20/20 (min 19) · ordinary both 40/40 (min 36)
  arm LI: footprint thinking present 0/60 · elapsed_ms p50 6054 max 7178 · peak_rss_kib max 5320792 · peak_vram_mib max -
l4-decision-probe: selection: shipped · order shipped,L,LI
l4-decision-probe: service verdict: PASS · arm shipped
l4-decision-probe: regression guard: HOLDS · sibling shipped 20 vs ns 20 · ordinary shipped 40 vs ns 38
```

## Per-arm, per-shape table — `identifies` = `both`, of 10 generations each

| arm | S1 | S2 | S3 | S4 | ordinary (min 36) | S7 | S8 | sibling (min 19) | bar |
|---|---|---|---|---|---|---|---|---|---|
| `shipped` (the product) | 10 | 10 | 10 | 10 | **40/40** | 10 | 10 | **20/20** | MET |
| `ns` (baseline, v2.5) | 10 | 10 | 8 | 10 | **38/40** | 10 | 10 | **20/20** | MET |
| `L` (line only, harness) | 10 | 10 | 10 | 10 | **40/40** | 10 | 10 | **20/20** | MET |
| `LI` (both, harness) | 10 | 10 | 10 | 10 | **40/40** | 10 | 10 | **20/20** | MET |

Every label that was not `both`: 2 of 240, both `signal_only`, both in arm `ns` on S3 (runs 9 and 10). No
`service_only`, `neither` or `unparsed` label occurred in any arm; 240 of 240 generations parsed.

The table was recounted from `runs.json` independently of the printed lines (240 rows; per arm 60; the `both` counts
per shape, the two halves and the two non-`both` rows equal the lines above), and the 240 per-run stderr lines carry
`identifies both` 238 times and `identifies signal_only` twice, the same two runs.

## Selection, verdict, guard

- **Selection:** `shipped` — the first arm in the order `shipped`, `L`, `LI` whose bar reads MET.
- **Verdict:** PASS — the selection is `shipped`.
- **Regression guard:** HOLDS — sibling `shipped` 20 vs `ns` 20, ordinary `shipped` 40 vs `ns` 38
  (`evidence/regression-guard.md`).

## What this reading does and does not show

- It shows the shipped form meets the pre-registered bar on this instrument: 20 of 20 on the sibling shapes and 40 of
  40 on the ordinary ones.
- **The baseline arm also met the bar** (20/20 and 38/40). On the sibling half the sentence moved nothing that this
  reading could see: `ns` already read 20 of 20. The only difference between `shipped` and `ns` is 2 generations on
  S3, an ordinary shape. At n = 10 per shape that difference is small; this reading does not establish that the
  sentence is what makes the sibling case pass.
- **The d3 failure did not reproduce on the probe.** Under the pre-change render (`ns`) the first hypothesis named
  `conductor` as a whole word in 10 of 10 generations on S7 (no corpus match) and 10 of 10 on S8 (two corpus lines
  naming `Conductor-Canary`). So the corpus-route question stays open: S7 and S8 read the same, which is not evidence
  that the route has no influence, only that 20 generations on these two synthetic shapes did not show the miss that
  1 of 3 end-to-end drives showed.
- The probe is not the product end to end: its digests are synthetic, it boots no app and reads no corpus.
  Conductor's fifth series is that leg and is not replaced by this reading.
- The grader's stated limits ride with every count: negation is not read, a space-separated sibling passes, `retried`
  is not a token.
- Recorded only, outside the rule: `names_trigger` (the substring reading) read rank1 57 of 60 on `ns` (S2 8, S4 9)
  and 60 of 60 on the other three arms; one `ns` generation decided `watch`, every other generation `surface`.

## Footprint, as a reading (never a grade)

| arm | elapsed_ms p50 | elapsed_ms max | wall | peak_rss_kib max | thinking present |
|---|---|---|---|---|---|
| `shipped` | 6411 | 10099 | 399 s | 5320776 | 0/60 |
| `ns` | 6029 | 7348 | 362 s | 5320744 | 0/60 |
| `L` | 5944 | 7301 | 363 s | 5320760 | 0/60 |
| `LI` | 6054 | 7178 | 368 s | 5320792 | 0/60 |

Beside the 10000 ms gpu-primary budget (obs-plan §10): every arm's p50 is under it; the `shipped` arm's slowest single
generation read 10099 ms, the other three arms' maxima are under it. `elapsed_ms` is spawn to reap of one `llama-cli`
process, model load included, so it is not the budget's own sample. `peak_vram_mib` is unread (`-`): the entry does
not pass `--footprint`. Generation wall time summed over the four arms: 1492 s (24.9 min); predicted about 21 min. The
whole entry, build included, took 25 min 0 s.

The run wrote no model text: `runs.json` rows hold the 16 closed-label keys only (`arm`, `decision`, `elapsed_ms`,
`first_keys`, `identifies`, `is_resolution_summary`, `names_trigger`, `names_trigger_stem`, `output_hash`,
`peak_rss_kib`, `peak_vram_mib`, `run`, `severity`, `shape`, `thinking`, `would_create`).

After the run no `llama-cli` and no probe process remained (process list and `nvidia-smi` read at
2026-10-07T06:09:02Z).

Recorded 2026-10-07T06:09:02Z.
