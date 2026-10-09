# Pre-registration addendum 2 — Gemma 4 E4B, QAT q4_0 vs Q4_K_M, face to face

Written (addendum, UTC): 2026-10-05T12:24:20Z. Written before the QAT file's download and before any generation of
this leg. The leg's series root carries a later UTC stamp (`series.md`).

## Why

The founder's ruling, relayed by the overseer (inputs#I4): after the table recommended gemma-4-E4B, compare Google's
QAT q4_0 build against the regular Q4_K_M face to face before he picks. Record-only: the pre-registered recommendation
(`table.md`, `recommendation: gemma-4-E4B-it-Q4_K_M`) stands as it read. This leg decides nothing by itself; it
informs the founder's pick between the two files.

## The two files

| file (basename) | source | quant | integrity |
|---|---|---|---|
| `gemma-4-E4B-it-Q4_K_M.gguf` | the existing file, unchanged | Q4_K_M | its bytes as at the main series |
| `gemma-4-E4B_q4_0-it.gguf` | `google/gemma-4-E4B-it-qat-q4_0-gguf` at revision `4b4a2c1d584be7264f87aac328a1bc739ce81b6c`; apache-2.0, not gated (HF API, read 12:23Z) | QAT q4_0 | 5,154,941,280 B; LFS sha256 `676c35070db6dbe52f93e9c864ee0fba4eddea94b9c875d9cb10daff453fbaee`. The download is accepted only if its own sha256 equals this value |

The repo's `gemma-4-E4B-it-mmproj.gguf` (vision projector) is not downloaded: text only.

## The leg

- Both files, fresh, back to back in one slot, Q4_K_M first, into one new series root under the gitignored `target/`.
- The same pattern set as the main series: all 13 shapes, A and B in today's render, C in `today` and `enriched`:
  16 shape-renders × n = 10 = 160 generations per file, 320 in all.
- The same argv and settings as the main series' gemma rows: the `gb` arm (`--grammar-file l4-output.gbnf`),
  `-c 8192 -rea off`, CUDA route, the authors' sampling `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0` (gemma4 is the
  architecture of both files). The probe binary and `patterns.rs` are the ones the table graded (sha256
  `4d3e6f81…0ca9`); nothing is changed before this leg.
- Seed: the series passes no generation seed (llama-cli seeds each run itself), so "the pre-registered seed" names only
  the audit draw's seed, and no audit is re-run here (the grader was audited on this pattern set at 9/96, `agree`). It
  is therefore unused by this leg.

## What is reported

- The same grader, labels and table columns: A detect · A cause · B false alarm · C today detect and cause · C enriched
  detect and cause · no_reading · GPU p50 and max · over 5000 ms · peak VRAM · peak RSS, from `--table` over the new
  series root (the whole leg re-graded from its stored outputs).
- The two fresh rows side by side; the main series' Q4_K_M row beside its fresh one, read as a run-to-run noise
  reading, not a result.
- The post-hoc surface-or-watch reading per file, labelled not pre-registered, as before.
- A cause stays a lower bound (audit finding 1). The `rule:` and `recommendation:` lines the table prints over two
  files are recorded only as its output; the pre-registered recommendation is the main series' one.
- The new run dir is also copied to the founder's data directory (inputs#I4 §5).
