# Pre-registration addendum 2 — G3 retired by the founder's hardware ruling

**Written (UTC):** 2026-10-05T08:35:02Z — before any further run. The confirmation's out dirs must carry a later timestamp.

**Base:** `preregistration.md` (2026-10-05T06:49:18Z) as amended by `preregistration-addendum.md`
(2026-10-05T07:23:28Z). Everything they state stands unless amended here.
**Authority:** founder ruling 2026-10-05, relayed by the overseer (founder-delegated): inputs#I6, quoted verbatim:

> «Да давай тогда делаем по простому с моделью анализ без модели чисто программно но в любом случае иметь возможность
> отдать результат агенту маркдауном или по мсп»

As the relay reads it: with a GPU (CUDA, or Metal on macOS) L4 runs the small model. Without one, L4 runs NO model and
the analysis is programmatic. In both modes the result can be handed to an agent as markdown or over MCP. This
retires CPU inference.

## What is recorded, and stays recorded

- `selection.md` stands as it read under the rule then in force: `pick: none`, decided by G3 (the CPU-only latency
  gate). It is not rewritten (inputs#I6 §1).
- Every CPU-only row stays in the table as recorded data.
- **Disclosure:** this amendment is written AFTER the selection data was seen. Its pick therefore has to earn the
  slot on FRESH generations (the confirmation below). A reading of the recorded selection alone never ships a model.

## The amendment

1. **G3 is retired** by the founder ruling. Its readings remain in `selection.md` as data.
2. **The rule is otherwise unchanged** (inputs#I6 §2). G1 and G2 apply verbatim, read from the `gb` legs. G1 keeps
   its CPU-only half, so a model that timed out on the CPU route is still a G1 failure. The Order is unchanged:
   survivors ascend by CPU-only-leg max `peak_rss_kib`, a tie going to the lower CPU-only max `elapsed_ms`. S is
   unchanged with B = 36.
3. **Re-application on the recorded data, written now, before the confirmation fires:**
   - G1: Qwen3.5-4B (8 CPU timeouts) and gemma-4-E4B (5 CPU timeouts) remain disqualified. Qwen3.5-2B and
     gemma-4-E2B pass.
   - G2: all pass.
   - Order: Qwen3.5-2B (CPU-only max RSS 2321320 KiB), then gemma-4-E2B (4667264 KiB). On the CUDA-route RSS basis
     the order is the same (1676632 vs 3494456 KiB).
   - S: the first survivor with selection rank1 ≥ 36 is **Qwen3.5-2B** (37/40). The overseer's recount from the
     runs.json files matches (inputs#I6 §2: rank1 37, 1.60 GiB RSS / 2.08 GiB VRAM).
   - **Pick under this addendum: `Qwen3.5-2B-Q4_K_M`**, subject to the confirmation.
4. **C — confirmation, amended threshold:** PASS iff fresh rank1 ≥ **36** (B, measured under the same argv shape in
   the same series). This replaces the base rule's 34, which was the predecessor's standing under a different argv
   (llama.cpp default sampling, `--json-schema-file`). On the founder relay's instruction (inputs#I6 §3).

## The confirmation runs (each fired once, after this write time)

Both on the CUDA route, `Qwen3.5-2B-Q4_K_M`, the `gb` arm, its authors' sampling, thinking off. The argv is as in
`preregistration-addendum.md`: `--arms gb --gbnf target/l4-decision-probe/l4-output.gbnf --sampling '--temp 0.7
--top-p 0.8 --top-k 20 --min-p 0 --presence-penalty 1.5 --chat-template-kwargs {"enable_thinking":false}'
--footprint`.

1. **Confirmation** (plan entry 22's slot): `--shapes S1,S2,S3,S4 --n 10 --min-rank1 36`. Out dir
   `target/l4-decision-probe/confirm-gb-{UTC}`. The probe's exit decides: 0 PASS, 1 FAIL, 2 measured nothing.
2. **Held-out** (plan entry 23's slot): `--shapes S5,S6 --n 10`, recorded only. Out dir
   `target/l4-decision-probe/confirm-heldout-gb-{UTC}`.

Plan entries 22 and 23 read `pick:` from `selection.md`, which stays `none`, so their guard would not fire. They are
therefore driven by hand in the form above, and that is recorded as a plan deviation, never by editing
`selection.md`.

## Ship (unchanged in this chunk, inputs#I6 §4)

- The product argv in this chunk stays `-c 8192 -rea off` on the existing `--json-schema-file` path with llama.cpp's
  sampling. Shipping the model, its sampling and a committed GBNF is the next route entry.
- PASS: Qwen3.5-2B is the confirmed L4 model choice, and the wrap mints entry A ("L4 runs Qwen3.5-2B with its
  authors' settings") and entry B ("Without a GPU, L4 analysis is programmatic"), both ahead of pre-push:linux.
- FAIL: Llama stays, and the FAIL is recorded as measured. Entry B is minted either way (the founder ruling stands
  independent of this series).
