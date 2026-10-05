# Pre-registration addendum 3 — Gemma 4 E2B as a record-only comparator in the confirmation slot

**Written (UTC):** 2026-10-05T08:35:46Z — before any gemma-4-E2B confirmation generation. Its out dirs must carry a later
timestamp.

**Base:** `preregistration.md` + `preregistration-addendum.md` + `preregistration-addendum-2.md` (08:35:02Z). All
of them stand unchanged.
**Authority:** overseer (founder-delegated), 2026-10-05, relayed at /implement P2 while the Qwen3.5-2B confirmation
was already running: "add Gemma 4 E2B to the confirmation slot as a RECORD-ONLY comparator (fresh n=40 S1-S4 plus
S5/S6, its authors' sampling, GBNF, -rea off), measured in the same slot as Qwen3.5-2B. The pick and its PASS rule
stay Qwen3.5-2B as written; Gemma's confirmation is recorded beside it so the founder can choose on fresh data."

**Timing disclosure:** addendum 2 was already stamped, and the Qwen3.5-2B confirmation had started
(`confirm-gb-20261005T083515Z`), before this instruction arrived, so this is a third addendum rather than an edit.
The Qwen3.5-2B runs are unaffected by it.

## What it adds

- **gemma-4-E2B-it-Q4_K_M, record-only.** It runs on the CUDA route, `gb` arm, its authors' sampling
  `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0`, `--grammar-file l4-output.gbnf`, `-rea off` (in the shared
  argv), `--footprint`. Two runs, each fired once, in this order:
  1. S1–S4 × 10 (`--shapes S1,S2,S3,S4 --n 10`), with **no** `--min-rank1`, so the probe prints no verdict and its
     exit 0 is a completed record. Out dir `target/l4-decision-probe/confirm-gemma-e2b-gb-{UTC}`.
  2. S5/S6 × 10 (`--shapes S5,S6 --n 10`). Out dir `target/l4-decision-probe/confirm-heldout-gemma-e2b-gb-{UTC}`.
- **The same slot:** the same session, host, route, prompt composition, GBNF and probe build as the Qwen3.5-2B
  confirmation. The two run strictly one after the other, never concurrently, so neither perturbs the other's
  latency or VRAM.
- **The pick and its PASS rule are unchanged.** The pick is Qwen3.5-2B, PASS iff its fresh rank1 ≥ 36
  (addendum 2). Gemma's readings decide nothing under the rule. They are reported beside Qwen's in one table
  (rank1, per shape, stem, held-out, RSS, VRAM, GPU latency) for the founder's choice.
