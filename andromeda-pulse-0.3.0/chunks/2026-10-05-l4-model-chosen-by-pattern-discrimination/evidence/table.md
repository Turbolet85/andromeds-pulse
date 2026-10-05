# The quality-vs-footprint table — plan Steps 6 and 7

The whole series re-graded from its stored outputs by `--table` (plan entry 23, fired once by hand, exit 0), after
the audit read `agree`. The pre-registered rule produced the recommendation; the founder decides, and the decision is
the next entry's input.

## The table leg's output, verbatim

```text
table: Llama-3.2-3B-Instruct-Q4_K_M · A detect 91% (64/70) · A cause 6% (4/70) · B false_alarm 97% (29/30) · C today detect 93% (28/30) cause 0% (0/30) · C enriched detect 57% (17/30) cause 30% (9/30) · no_reading 0/160 · gpu_ms p50 2761 max 3423 · over_5000ms 0/160 · peak_vram_mib 3382 · peak_rss_kib 2373348
table: NVIDIA-Nemotron-3-Nano-4B-Q4_K_M · A detect 67% (47/70) · A cause 6% (4/70) · B false_alarm 43% (13/30) · C today detect 53% (16/30) cause 0% (0/30) · C enriched detect 40% (12/30) cause 37% (11/30) · no_reading 0/160 · gpu_ms p50 3378 max 4395 · over_5000ms 0/160 · peak_vram_mib 3284 · peak_rss_kib 3233940
table: Qwen3.5-2B-Q4_K_M · A detect 94% (66/70) · A cause 17% (12/70) · B false_alarm 90% (27/30) · C today detect 93% (28/30) cause 3% (1/30) · C enriched detect 100% (30/30) cause 27% (8/30) · no_reading 0/160 · gpu_ms p50 3498 max 4793 · over_5000ms 0/160 · peak_vram_mib 2128 · peak_rss_kib 1676740
table: Qwen3.5-4B-Q4_K_M · A detect 60% (42/70) · A cause 19% (13/70) · B false_alarm 0% (0/30) · C today detect 3% (1/30) cause 7% (2/30) · C enriched detect 97% (29/30) cause 50% (15/30) · no_reading 0/160 · gpu_ms p50 3396 max 8650 · over_5000ms 66/160 · peak_vram_mib 3716 · peak_rss_kib 3102636
table: gemma-4-E2B-it-Q4_K_M · A detect 19% (13/70) · A cause 3% (2/70) · B false_alarm 0% (0/29) · C today detect 0% (0/30) cause 0% (0/30) · C enriched detect 20% (6/30) cause 47% (14/30) · no_reading 1/160 · gpu_ms p50 3707 max 5043 · over_5000ms 1/160 · peak_vram_mib 2308 · peak_rss_kib 3494540
table: gemma-4-E4B-it-Q4_K_M · A detect 57% (40/70) · A cause 49% (34/70) · B false_alarm 0% (0/30) · C today detect 0% (0/30) cause 0% (0/30) · C enriched detect 10% (3/30) cause 57% (17/30) · no_reading 0/160 · gpu_ms p50 5346 max 7285 · over_5000ms 100/160 · peak_vram_mib 3898 · peak_rss_kib 5321236
table: regrade changed 0 of 960 row labels
rule: best A cause 49% · best B false_alarm 0% · qualifying gemma-4-E4B-it-Q4_K_M
recommendation: gemma-4-E4B-it-Q4_K_M
```

The re-grade from stored outputs reproduced every generation-time label (0 of 960 changed).

## The curve, read

Quality (A-cause, B false alarm) against footprint (peak VRAM; GPU latency), lightest VRAM first:

| model | peak VRAM MiB | peak RSS GB | GPU p50 / max ms | over 5000 ms | A detect | A cause | B false alarm | C today cause | C enriched cause |
|---|---|---|---|---|---|---|---|---|---|
| Qwen3.5-2B | 2128 | 1.68 | 3498 / 4793 | 0/160 | 94 % | 17 % | 90 % | 3 % | 27 % |
| gemma-4-E2B | 2308 | 3.49 | 3707 / 5043 | 1/160 | 19 % | 3 % | 0 % | 0 % | 47 % |
| Nemotron-3-Nano-4B | 3284 | 3.23 | 3378 / 4395 | 0/160 | 67 % | 6 % | 43 % | 0 % | 37 % |
| Llama-3.2-3B (shipped) | 3382 | 2.37 | 2761 / 3423 | 0/160 | 91 % | 6 % | 97 % | 0 % | 30 % |
| Qwen3.5-4B | 3716 | 3.10 | 3396 / 8650 | 66/160 | 60 % | 19 % | 0 % | 7 % | 50 % |
| gemma-4-E4B | 3898 | 5.32 | 5346 / 7285 | 100/160 | 57 % | 49 % | 0 % | 0 % | 57 % |

- **The rule's pick is the heaviest model.** Only gemma-4-E4B is within 10 points of the best A-cause (49 %; the
  next is Qwen3.5-4B at 19 %), and its B false alarm is 0 %. It is also the largest on VRAM and RSS and the slowest:
  p50 5346 ms, and 100 of 160 generations over the 5000 ms gpu-primary budget.
- **Two shapes of failure on the light side.** Llama-3.2-3B (the shipped model) and Qwen3.5-2B surface almost
  everything: A detect above 90 % but B false alarm 97 % and 90 %, so their detect is not discrimination. gemma-4-E2B
  is the opposite: quiet on B (0 %) and on most of A (19 %).
- **Qwen3.5-4B is the only model quiet on B (0 %) that still surfaces most of A (60 %)**, at A-cause 19 % and 66/160
  generations over 5000 ms.
- **A-cause is a lower bound** for every model (audit finding 1, `audit.md`): the A1 cause words miss a rank-1
  statement that blames the new client in substance.

## C-enriched vs C-today (inputs#I1 §6)

Cause on the C shapes rises in every model once the digest carries baselines and a short trend: from 0–7 % today to
27–57 % enriched (Llama 0→30, Nemotron 0→37, Qwen3.5-2B 3→27, Qwen3.5-4B 7→50, gemma-4-E2B 0→47, gemma-4-E4B 0→57).
Detect on C rises for the models quiet on B (Qwen3.5-4B 3→97 %, gemma-4-E2B 0→20 %, gemma-4-E4B 0→10 %); for the
models that surface nearly everything, today's C detect was already high and carries no discrimination. This is a
clear rise. It PROPOSES the separate entry "the digest carries baselines and a short trend", on the founder's word
only, never minted here.

## The cueless no-incident fact (scope §Founder-facing fact)

Every A shape but A7, and every C shape, is a cueless `tier3` digest. Today a cueless, non-reflection digest whose
generation says `surface` creates NO incident: the producer records `interpretation.incident.skipped
{skip_reason: no_cue}` and returns (`pulse-app/src/inference_runtime.rs:822-824`). So the A and C detect columns
measure what the MODEL catches; they become user-visible only after a product change. If the founder's pick rests on A
or C, "a cueless surface creates an incident" is a candidate follow-on entry, on the founder's word only.

## Post-hoc reading — NOT pre-registered, record-only (overseer, 2026-10-05)

Asked after the table, because the founder ruled that cueless findings become quiet observations: the share of
readable A and C rows whose decision is `surface` OR `watch`, per model. It feeds no label, no cell and no rule.

| model | A surface-or-watch | C today | C enriched |
|---|---|---|---|
| Llama-3.2-3B | 70/70 | 30/30 | 30/30 |
| Nemotron-3-Nano-4B | 63/70 | 25/30 | 30/30 |
| Qwen3.5-2B | 67/70 | 28/30 | 30/30 |
| Qwen3.5-4B | 46/70 | 2/30 | 30/30 |
| gemma-4-E2B | 32/70 | 0/30 | 30/30 |
| gemma-4-E4B | 70/70 | 10/30 | 30/30 |

Under this reading every model records every enriched C row, and gemma-4-E4B records every A row (70/70) while
staying silent on B (0 % surface). B's `watch` share is not shown: it was not asked.

## The founder's decision (2026-10-05)

After this table and the record-only QAT face-to-face (`qat-head-to-head.md`), the founder picked, verbatim:
«Давай брать обычную тогда» ("let's take the regular one, then"), relayed verbatim by the overseer
(founder-delegated), 2026-10-05.

- **Pick: Gemma 4 E4B, the unsloth `gemma-4-E4B-it-Q4_K_M.gguf`**, sha256 `85a896a0…ab87` (the bytes of this
  series), over Google's QAT `gemma-4-E4B_q4_0-it.gguf`. It is the model the pre-registered rule recommended.
- It is the input of the next working entry, "L4 runs the founder's pick with its authors' settings" (entry A), which
  ships it. This chunk changes no product model, argv or sampling: the shipped GGUF stays Llama-3.2-3B-Instruct-Q4_K_M
  until that entry.
