# Slot 1 - the arm matrix (plan Steps 10-12)

Granted by the overseer 2026-10-01 ("slot 1 granted"), L4 env exported for the one command only.
Run on the untouched base: no Part 5 edit existed; render, prompt and argv byte-identical to HEAD
(dry-run beforehand: every arm composed, prompts 6644-6786 bytes against the 16384 cap).

Command: `l4_decision_probe --arms A0,A1,A2,A3,A4,A5 --n 10` (A0 first), exit 0.
Binary `llama-cli.exe` (b9305, cuda, `-ngl 99`) · model `Llama-3.2-3B-Instruct-Q4_K_M.gguf`.
Synthetic Tier1 retry-storm digests S1 (1 of 4 services at 100% errors), S2 (35%), S3 (12% + p99 4x).
Wall clock: A0 126 s · A1 133 s · A2 125 s · A3 122 s · A4 126 s · A5 129 s = 761 s (~12.7 min,
~4.2 s per generation including the per-spawn model load). 0 failed generations (no timeout, exit
failure or parse failure in 180). No probe or llama-cli process survived the run.

## Per-arm summary lines (verbatim)
```
arm A0: would_create 24/30 · decision 22/1/7 · severity_none 6 · resolution_summary 0
  arm A0: per shape would_create S1 8 S2 9 S3 7 · failed 0 · first_keys schema_version>prompt_version>decision=30 · distinct outputs S1 10 S2 10 S3 10 · wall 126s
arm A1: would_create 30/30 · decision 30/0/0 · severity_none 0 · resolution_summary 0
  arm A1: per shape would_create S1 10 S2 10 S3 10 · failed 0 · first_keys schema_version>prompt_version>decision=30 · distinct outputs S1 1 S2 1 S3 1 · wall 133s
arm A2: would_create 28/30 · decision 28/0/2 · severity_none 2 · resolution_summary 0
  arm A2: per shape would_create S1 10 S2 9 S3 9 · failed 0 · first_keys schema_version>prompt_version>decision=30 · distinct outputs S1 10 S2 10 S3 10 · wall 125s
arm A3: would_create 23/30 · decision 22/1/7 · severity_none 7 · resolution_summary 0
  arm A3: per shape would_create S1 8 S2 6 S3 9 · failed 0 · first_keys schema_version>prompt_version>decision=30 · distinct outputs S1 10 S2 10 S3 10 · wall 122s
arm A4: would_create 30/30 · decision 30/0/0 · severity_none 0 · resolution_summary 0
  arm A4: per shape would_create S1 10 S2 10 S3 10 · failed 0 · first_keys schema_version>prompt_version>decision=30 · distinct outputs S1 10 S2 10 S3 10 · wall 126s
arm A5: would_create 30/30 · decision 23/0/7 · severity_none 0 · resolution_summary 0
  arm A5: per shape would_create S1 10 S2 10 S3 10 · failed 0 · first_keys schema_version>prompt_version>title=30 · distinct outputs S1 10 S2 10 S3 10 · wall 129s
```

## Readings
- **Defect reproduced at HEAD:** r(A0) = 24 < 27. Measured on synthetic storm digests, n = 30.
- **What the no-incident outcome actually was (premise correction):** A0's 6 non-creating generations
  are 6 `severity: none` (1 of them also `decision: dismiss`); 0 `is_resolution_summary`. "The model
  dismissed the storm" describes 1 of 30 here; the dominant skip reason is `severity_none`, which the
  production log could not tell apart before this chunk's `interpretation.incident.skipped` record.
- **Seed / determinism:** at llama.cpp's defaults (`seed 0xFFFFFFFF`, `temp 0.80`) every run diverged
  (A0 distinct outputs 10/10 per shape), so the sentinel draws a fresh seed per spawn. Under A1
  (`--temp 0`) each shape produced ONE distinct output across its 10 runs, so A1's effective n is 3
  prompts (plan note), recorded as a fact.
- **Grammar key order (research E5):** the generated key order follows the schema's property order -
  `decision` is the 3rd key in A0-A4 (30/30) and moves after the analysis fields under A5
  (`schema_version>prompt_version>title`, 30/30). E5 holds: at HEAD the decision is generated before
  any analysis text.
- **A2 (the OVERALL line alone):** 28/30, a delta of r(A2) - r(A0) = +4. Reported beside the selection,
  never selected or rejected by it (Step 13 ships it regardless).

## Step 12 rule, applied as pre-registered
- Qualifiers (r >= 27 among A1, A3, A4, A5): **A1 (30) · A4 (30) · A5 (30)**; A3 (23) does not qualify.
- Selection order A1 -> A3 -> A5 -> A4: the first qualifier is **A1 (sampling, `--temp 0`)**.
- A1 adds constant argv members to the L4 subprocess: a playbook "Boundary widening" (verdict
  `escalate`). /implement stops for the founder's ratification before writing it.
