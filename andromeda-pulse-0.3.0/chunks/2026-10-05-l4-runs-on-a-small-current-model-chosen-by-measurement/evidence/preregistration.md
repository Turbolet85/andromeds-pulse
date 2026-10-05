# Pre-registration — L4 model selection series

**Written (UTC):** 2026-10-05T06:49:18Z — before any real-model generation of this chunk. Every leg's out dir
(`target/l4-decision-probe/{leg}-{UTC}`) must carry a later timestamp.

**Source:** `plan.md` §Implementation notes → Pre-registered rule, copied verbatim below (plan lines 557–580).

---

- **Pre-registered rule** (copied verbatim into `evidence/preregistration.md` at Step 7, before any real-model
  generation; the overseer ratified the shape at P4, founder-delegated, 2026-10-05):
  - **Models:** baseline `Llama-3.2-3B-Instruct-Q4_K_M`; candidates `Qwen3.5-2B-Q4_K_M` · `Qwen3.5-4B-Q4_K_M` ·
    `gemma-4-E2B-it-Q4_K_M` · `gemma-4-E4B-it-Q4_K_M`. All run under the shipped argv (`-c 8192 -rea off`), the frozen
    strict `names_trigger` grader, and the `shipped` arm (the v2.5 lineage).
  - **G1 — loads.** On the CUDA load leg (S1 × 2) AND on the CPU-only leg (S1–S4 × 2), no shipped row reads
    `spawn_failed` · `exit_failure` · `timeout` · `output_too_large` · `stdout_utf8_invalid`. A failing candidate is
    disqualified.
  - **G2 — no thinking.** Every shipped row of every leg reads `thinking` ≠ `present`. One `present` disqualifies.
    The `nr` arm is recorded only.
  - **G3 — CPU-only latency.** Every CPU-only-leg generation's `elapsed_ms` ≤ 30000, the cpu_primary budget
    (`xtask/ci/l4-latency-p99.sh:38`), with none timed out. Any over disqualifies.
  - **Order.** Survivors ascend by CPU-only-leg max `peak_rss_kib`; a tie goes to the lower CPU-only max `elapsed_ms`.
  - **S — selection.** B = the baseline's strict rank1/40 in the selection slot (same slot, same argv). The pick is
    the FIRST survivor in order whose selection rank1 ≥ B. None qualifies → `pick: none`.
  - **C — confirmation.** A fresh S1–S4 × 10 run on the pick: PASS iff strict rank1 ≥ 34 (the baseline's confirmed
    standing, scope CONTEXT 3). S5/S6 × 10 are recorded only.
  - **Ship.** PASS → the pick is the L4 model. FAIL or `pick: none` → Llama stays and the FAIL is recorded. Either
    way `-c 8192` and `-rea off` ship.
  - **The table.** A row per model, all five, whether disqualified or not (the overseer's requirement).
  - **Discipline.** Each slot is fired once. No re-run, no re-threshold, no arm or shape added after the first
    generation.
  - **The baseline.** If the baseline itself fails G1 or G2 under the new argv, that is a product regression of Step 1,
    not a candidate outcome: stop and report.
