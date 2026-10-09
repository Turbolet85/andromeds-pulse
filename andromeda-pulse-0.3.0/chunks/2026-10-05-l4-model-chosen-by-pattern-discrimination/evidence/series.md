# The pattern series — plan Step 6

Labels and aggregates only. The digests, the raw outputs, `runs.json` and the audit files stay under the gitignored
series root in `target/`.

## GPU precondition (plan entry 19, fired by hand at 2026-10-05T10:40:02Z)

- `test "$(ss -ltnH | grep -cE ':(4317|4318) ')" = 0 && nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv,noheader`
  → exit 0. `:4317` / `:4318` free.
- Compute processes listed: one, `voxtype-osd-gtk4`, 10 MiB: the founder's overlay, foreign, RECORDED per CARRY 2,
  never read as red. No `llama` process (the `lacks llama` atom holds). The footprint is read per child pid, so the
  overlay never enters a reading.
- conductor-builder's session was present and idle (inputs#I2).

## The series leg (plan entry 20, fired once by hand)

- Pre-registration stamp: `Written (UTC): 2026-10-05T10:39:23Z`.
- Leg start 2026-10-05T10:40:32Z · end 2026-10-05T11:41:44Z · exit 0 · 960 row lines on stderr.
- Series root: `target/l4-decision-probe/patterns-20261005T104032Z` (UTC stamp 10:40:32Z, after the pre-registration).
  One dir per model under it, each stamped by the same root.
- The probe binary was built outside the timed leg (entry 3, then a rebuild from the final source at 10:40:21Z).
  `l4-output.gbnf` read sha256 `7b5cc276…ac03`, equal to the predecessor's addendum. `l4-env.sh` was identical to
  inputs#I3.
- Every model printed `l4-decision-probe: gbnf l4-output.gbnf` and the binary line
  `binary llama-cli (cuda, -ngl 99) · model {basename} · arms gb · shapes A1,…,C3 · renders today,enriched · n 10 per shape-render`.

Per model, the leg's own summary lines, verbatim:

### Llama-3.2-3B-Instruct-Q4_K_M — sampling `--temp 0.6 --top-p 0.9 --top-k 40 --min-p 0`

```text
arm gb: decision 138/0/22 · no decision 0 · wall 438s
  arm gb: patterns A detect 64/70 cause 4/70 · B false_alarm 29/30 · C today detect 28/30 cause 0/30 · C enriched detect 17/30 cause 9/30 · no_reading 0/160
  arm gb: footprint thinking present 0/160 · elapsed_ms p50 2761 max 3423 · peak_rss_kib max 2373348 · peak_vram_mib max 3382
```

### Qwen3.5-2B-Q4_K_M — sampling `--temp 0.7 --top-p 0.8 --top-k 20 --min-p 0 --presence-penalty 1.5 --chat-template-kwargs {"enable_thinking":false}`

```text
arm gb: decision 151/7/2 · no decision 0 · wall 564s
  arm gb: patterns A detect 66/70 cause 12/70 · B false_alarm 27/30 · C today detect 28/30 cause 1/30 · C enriched detect 30/30 cause 8/30 · no_reading 0/160
  arm gb: footprint thinking present 0/160 · elapsed_ms p50 3498 max 4793 · peak_rss_kib max 1676740 · peak_vram_mib max 2128
```

### Qwen3.5-4B-Q4_K_M — sampling as Qwen3.5-2B

```text
arm gb: decision 72/82/6 · no decision 0 · wall 672s
  arm gb: patterns A detect 42/70 cause 13/70 · B false_alarm 0/30 · C today detect 1/30 cause 2/30 · C enriched detect 29/30 cause 15/30 · no_reading 0/160
  arm gb: footprint thinking present 0/160 · elapsed_ms p50 3396 max 8650 · peak_rss_kib max 3102636 · peak_vram_mib max 3716
```

### gemma-4-E2B-it-Q4_K_M — sampling `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0`

```text
arm gb: decision 19/96/44 · no decision 1 · wall 618s
  arm gb: patterns A detect 13/70 cause 2/70 · B false_alarm 0/29 · C today detect 0/30 cause 0/30 · C enriched detect 6/30 cause 14/30 · no_reading 1/160
  arm gb: footprint thinking present 0/160 · elapsed_ms p50 3707 max 5043 · peak_rss_kib max 3494540 · peak_vram_mib max 2308
```

### gemma-4-E4B-it-Q4_K_M — sampling as gemma-4-E2B

```text
arm gb: decision 43/50/67 · no decision 0 · wall 834s
  arm gb: patterns A detect 40/70 cause 34/70 · B false_alarm 0/30 · C today detect 0/30 cause 0/30 · C enriched detect 3/30 cause 17/30 · no_reading 0/160
  arm gb: footprint thinking present 0/160 · elapsed_ms p50 5346 max 7285 · peak_rss_kib max 5321236 · peak_vram_mib max 3898
```

### NVIDIA-Nemotron-3-Nano-4B-Q4_K_M — sampling `--temp 0.6 --top-p 0.95 --top-k 40 --min-p 0 --chat-template-kwargs {"enable_thinking":false}`

```text
arm gb: decision 88/17/55 · no decision 0 · wall 542s
  arm gb: patterns A detect 47/70 cause 4/70 · B false_alarm 13/30 · C today detect 16/30 cause 0/30 · C enriched detect 12/30 cause 11/30 · no_reading 0/160
  arm gb: footprint thinking present 0/160 · elapsed_ms p50 3378 max 4395 · peak_rss_kib max 3233940 · peak_vram_mib max 3284
```

## Readings recorded at the leg (not yet graded by the audit)

- Thinking present 0/960; no timeout, no exit failure; one `no_reading` row (gemma-4-E2B, B2 today run 2,
  `parse_failed` at 3594 ms: the `no decision` in its decision count), excluded from its B denominator (29).
- These are the keyword labels at generation time. The table re-grades the whole series from its stored outputs,
  after the blind audit (`audit.md`), and its numbers are the record of decision.

## Survivors after the leg

`ps` read no `llama-cli` and no probe process; `nvidia-smi` listed only the foreign overlay.
