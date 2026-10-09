# L4 trigger-framing series — the pre-registered measurement

Chunk `2026-10-04-l4-framing-measured-on-the-real-model` · fired by /andromeda-implement run
`.andromeda/runs/2026-10-04T23-09-00Z-implement/` on the model slot granted at `inputs#I1` · leg env `inputs#I2`.
Bounded labels and counts only: no model text, no prompt text, no llama-cli stdout, no absolute host path.

## Pre-flight (Step 1, no model load) — 2026-10-04T23:09Z
- Shell exports of `ANDROMEDA_PULSE_*`: 0.
- GPU: RTX 3090, 971 / 24576 MiB in use, 0 % utilization; the only compute app was a desktop OSD (10 MiB).
- `test ! -e evidence/shipped` and `test ! -e evidence/nf`: both held (the `evidence/` dir did not exist).
- The dry-run entry (plan entry 5, green) printed, per arm × shape, prompt bytes against max 16384, schema 4626 B:
  - shipped S1–S4: 6984 / 6987 / 6991 / 7185 B
  - nf S1–S4: 6644 / 6647 / 6651 / 6766 B
  - Identical to research.md's P3 reading.

## Run 1 — `shipped` (plan entry 13, `leg = 'operator'`, fired ONCE)
- Run: the entry's `run` text byte for byte, driven from the repo root under `timeout 7200`.
- Started 2026-10-04T23:10:34Z · ended 23:12:26Z · wall 112 s (the probe's own `wall 111s`) · **exit 1** (FAIL verdict).
- Out dir printed: `series out target/l4-decision-probe/shipped-20261004T231034Z` (gitignored).
- Header: `l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model Llama-3.2-3B-Instruct-Q4_K_M.gguf · arms shipped · n 10 per shape`
- Per-arm lines, verbatim:
  - `arm shipped: would_create 40/40 · decision 36/0/4 · severity_none 0 · resolution_summary 0`
  - `arm shipped: per shape would_create S1 10 S2 10 S3 10 S4 10 · failed 0 · first_keys schema_version>prompt_version>title=40 · distinct outputs S1 10 S2 10 S3 10 S4 10 · wall 111s`
  - `arm shipped: names_trigger rank1 30/40 · elsewhere 5 · none 5 · unparsed 0 · per shape rank1 S1 9 S2 6 S3 5 S4 10`
- Verdict line, verbatim: **`l4-decision-probe: names-trigger verdict: FAIL · rank1 30/40`**
- Atoms: `contains l4-decision-probe: binary llama-cli (cuda, -ngl 99)` held · `contains arm shipped: names_trigger rank1`
  held · `contains l4-decision-probe: names-trigger verdict:` held · `lacks INCONCLUSIVE` held (0 occurrences).
- The series is spent. Not re-run; `--min-rank1`, `--n`, the arms and the shapes untouched.

## Run 2 — `nf` (plan entry 14, `leg = 'operator'`, fired ONCE, after `shipped`)
- Started 2026-10-04T23:12:42Z · ended 23:14:41Z · wall 119 s (the probe's own `wall 118s`) · **exit 0**.
- Out dir printed: `series out target/l4-decision-probe/nf-20261004T231242Z` (gitignored).
- Header: `l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model Llama-3.2-3B-Instruct-Q4_K_M.gguf · arms nf · n 10 per shape`
- Per-arm lines, verbatim:
  - `arm nf: would_create 39/40 · decision 26/0/14 · severity_none 1 · resolution_summary 0`
  - `arm nf: per shape would_create S1 9 S2 10 S3 10 S4 10 · failed 0 · first_keys schema_version>prompt_version>title=40 · distinct outputs S1 10 S2 10 S3 10 S4 10 · wall 118s`
  - `arm nf: names_trigger rank1 18/40 · elsewhere 10 · none 12 · unparsed 0 · per shape rank1 S1 6 S2 6 S3 1 S4 5`
- Record-only, unthresholded: no verdict line is printed and none is drawn.
- Atoms: `exit 0` held · `contains l4-decision-probe: binary llama-cli (cuda, -ngl 99)` held ·
  `contains arm nf: names_trigger rank1` held · `lacks INCONCLUSIVE` held (0 occurrences).

## The records
- `shipped-runs.json` and `nf-runs.json` are byte-for-byte copies of each run's `runs.json` (sha256 equal to the source):
  - shipped `446eb36d…fe68`, 40 rows
  - nf `26f97b47…68e9`, 40 rows
- Each row's keys: `arm` · `decision` · `first_keys` · `is_resolution_summary` · `names_trigger` · `output_hash` ·
  `run` · `severity` · `shape` · `would_create`. `schema-{arm}.json` (the product's L4 schema text) was not copied.

## Side by side — `names_trigger` (rank1 / elsewhere / none / unparsed), tallied from the two records

| shape | shipped | nf |
|---|---|---|
| S1 | 9 / 1 / 0 / 0 | 6 / 3 / 1 / 0 |
| S2 | 6 / 2 / 2 / 0 | 6 / 3 / 1 / 0 |
| S3 | 5 / 2 / 3 / 0 | 1 / 2 / 7 / 0 |
| S4 | 10 / 0 / 0 / 0 | 5 / 2 / 3 / 0 |
| total | 30 / 5 / 5 / 0 | 18 / 10 / 12 / 0 |

- Failed generations (timeout / non-zero exit / bad UTF-8 / unparsable, counted in the denominator): shipped 0 · nf 0.
- Decisions surface / dismiss / watch: shipped 36 / 0 / 4 · nf 26 / 0 / 14.

## The CONTEXT hypothesis reading
- Hypothesis (`[inferred]`, carried from the predecessor; from n = 1 recorded generation): "the d3 model restated a
  corpus-match title". Only S4 carries a corpus match, so its direct reading is S4's rank1 in each arm.
- **Direct reading: S4 rank1 — nf 5/10 · shipped 10/10.**
- Beside it, the non-corpus shapes: nf S1–S3 rank1 6 / 6 / 1 · shipped 9 / 6 / 5.
- What this sample shows, and its limits (n = 10 per shape and arm):
  - Without the framing, S4 (5/10) does not sit below the shapes that carry no corpus match (6 / 6 / 1). This sample
    does not show the corpus match depressing rank-1 retry naming. At n = 10 it cannot exclude a smaller effect.
  - With the framing, S4 reached 10/10. The shipped arm's shortfall against the threshold (30 against 36) lies in
    S2 (6/10) and S3 (5/10), shapes with no corpus match.
  - `names_trigger` grades whether the rank-1 statement names the retry. It does not detect a restated corpus-match
    title, so the mechanism itself is not observed here: only its predicted effect on S4's rank1 is.

## Routing witness
- Both runs printed `binary llama-cli (cuda, -ngl 99)` with the model basename `Llama-3.2-3B-Instruct-Q4_K_M.gguf`.
- The leg env sets no `ANDROMEDA_PULSE_HARDWARE_PROFILE`, and each run unset it, together with
  `ANDROMEDA_PULSE_L4_ALLOW_ROOT` and `ANDROMEDA_PULSE_L4_DETERMINISTIC`. So detection routed both runs, through
  `binary_target_for_profile`.
- **Attested by the probe header; not attested by self-observation.** The probe installs no tracing subscriber, so no
  `interpretation.*` record was emitted.

## Hygiene (plan entry 15, `leg = 'operator'`, driven by hand)
- `gate.py hygiene`: exit 0 · `hygiene: clean — read 35 (runs 28 · evidence 3 · inputs 4)` before this section was
  added; re-run after it (the final reading is in /implement's report).

## Process census after the series
- No `llama-cli` or `l4_decision_probe` process was left (process list read at 23:14Z). The GPU was back at
  971 MiB, 0 %.
