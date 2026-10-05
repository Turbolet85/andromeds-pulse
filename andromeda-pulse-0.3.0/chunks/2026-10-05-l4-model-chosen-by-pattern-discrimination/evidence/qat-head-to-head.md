# Gemma 4 E4B — QAT q4_0 vs Q4_K_M, face to face (record-only)

Pre-registered in `preregistration-addendum-2.md` (12:24:20Z), on the founder's ruling relayed by the overseer
(inputs#I4). Labels and aggregates only; the series root stays under the gitignored `target/` and is copied to the
founder's data directory. It decides nothing: the pre-registered recommendation is the main series' one
(`table.md`).

## The files and the leg

- `gemma-4-E4B-it-Q4_K_M.gguf`: 4,977,171,584 B, sha256 `85a896a0…ab87`, equal to the predecessor's
  `candidates.sha256` (the bytes of the main series).
- `gemma-4-E4B_q4_0-it.gguf` (Google QAT): downloaded from `google/gemma-4-E4B-it-qat-q4_0-gguf` at revision
  `4b4a2c1d…`, 5,154,941,280 B, sha256 `676c35070db6dbe52f93e9c864ee0fba4eddea94b9c875d9cb10daff453fbaee`, equal
  to the LFS value read before the download. The vision projector was not downloaded.
- Precondition at 12:32:31Z: `:4317` / `:4318` free; compute processes: only the foreign `voxtype-osd-gtk4`
  (10 MiB), recorded per CARRY 2.
- The leg: series root `target/l4-decision-probe/qat-h2h-20261005T123237Z` (stamp after the addendum), start
  12:32:37Z, end 12:59:03Z, exit 0, 320 row lines. Both files back to back, Q4_K_M first, the main series' form:
  `gb` arm, `-c 8192 -rea off`, CUDA route (`-ngl 99`), sampling `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0`.
  `patterns.rs` was the table-graded `4d3e6f81…0ca9`. No survivors after the leg.
- Archived: the series root copied to `~/dev/data/l4-model-comparison-2026-10/qat-h2h-20261005T123237Z`,
  `diff -rq` identical.

## The table over the leg, verbatim

```text
table: gemma-4-E4B-it-Q4_K_M · A detect 57% (40/70) · A cause 47% (33/70) · B false_alarm 0% (0/30) · C today detect 0% (0/30) cause 0% (0/30) · C enriched detect 7% (2/30) cause 67% (20/30) · no_reading 0/160 · gpu_ms p50 5187 max 7531 · over_5000ms 93/160 · peak_vram_mib 3898 · peak_rss_kib 5321236
table: gemma-4-E4B_q4_0-it · A detect 51% (36/70) · A cause 41% (29/70) · B false_alarm 0% (0/30) · C today detect 0% (0/30) cause 0% (0/30) · C enriched detect 13% (4/30) cause 57% (17/30) · no_reading 0/160 · gpu_ms p50 4871 max 7005 · over_5000ms 62/160 · peak_vram_mib 3706 · peak_rss_kib 5494780
table: regrade changed 0 of 320 row labels
rule: best A cause 47% · best B false_alarm 0% · qualifying gemma-4-E4B_q4_0-it,gemma-4-E4B-it-Q4_K_M
```

The table also printed its rule's pick over these two files, `gemma-4-E4B_q4_0-it` (both qualify, and the QAT file
is the lighter on VRAM). Per the addendum that line is the tool's output over a two-file leg, recorded here and not
a recommendation; it is kept out of column 0 so that `table.md` stays the one recommendation record.

## Side by side

| | Q4_K_M (main series) | Q4_K_M (fresh) | QAT q4_0 (fresh) |
|---|---|---|---|
| A detect | 57 % (40/70) | 57 % (40/70) | 51 % (36/70) |
| A cause (a lower bound) | 49 % (34/70) | 47 % (33/70) | 41 % (29/70) |
| B false alarm | 0 % (0/30) | 0 % (0/30) | 0 % (0/30) |
| C today detect / cause | 0 % / 0 % | 0 % / 0 % | 0 % / 0 % |
| C enriched detect / cause | 10 % / 57 % | 7 % / 67 % | 13 % / 57 % |
| no_reading | 0/160 | 0/160 | 0/160 |
| GPU p50 / max ms | 5346 / 7285 | 5187 / 7531 | 4871 / 7005 |
| over 5000 ms | 100/160 | 93/160 | 62/160 |
| peak VRAM MiB | 3898 | 3898 | 3706 |
| peak RSS kB | 5,321,236 | 5,321,236 | 5,494,780 |
| file size | 4.98 GB | 4.98 GB | 5.15 GB |

## Reading

- **Run-to-run noise (Q4_K_M, main vs fresh):** A detect identical, A cause 49→47, C enriched cause 57→67, GPU p50
  5346→5187 ms, over-5000 100→93. At n = 10 per shape a cell moves by up to about 10 points between identical runs
  with the same file, so a difference of that size between the two files is not separable from noise here.
- **Quality:** QAT reads lower on every A and C-cause cell (A detect −6, A cause −6, C enriched cause −10 against
  the fresh Q4_K_M), each within that noise band; both are silent on healthy traffic (B 0 %) and both catch nothing
  on today's C render.
- **Footprint:** QAT is the faster (p50 −316 ms; over-5000 62 vs 93 of 160) and lighter on GPU memory (−192 MiB
  VRAM), but heavier on host memory (+173 MB peak RSS) and on disk (+178 MB). Neither file fits the 5000 ms
  gpu-primary budget for most generations.

## Post-hoc reading — NOT pre-registered, record-only

Share of readable A and C rows whose decision is `surface` OR `watch` (the founder's quiet-observation ruling); feeds
no label and no rule.

| file | A surface-or-watch | C today | C enriched |
|---|---|---|---|
| Q4_K_M (main series) | 70/70 | 10/30 | 30/30 |
| Q4_K_M (fresh) | 68/70 | 10/30 | 30/30 |
| QAT q4_0 (fresh) | 68/70 | 10/30 | 30/30 |
