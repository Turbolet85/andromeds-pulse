
## 2026-10-04-l4-framing-measured-on-the-real-model — Fault Identity: the trigger framing measured, FAIL 30/40
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:** Was: the framing's effect on the model's rank-1 hypothesis was "UNMEASURED", with the pre-registered series "gated on a Linux llama.cpp CUDA binary", per "2026-10-04-l4-interpretation-names-its-triggering-cue — Fault Identity: the model layer is framed at prompt composition". Now the body says the effect FAILS the pre-registered bar, measured on Llama-3.2-3B-Instruct-Q4_K_M through llama-cli b9305 CUDA `-ngl 99`, routed by detection, n = 10 per shape over the four retry-storm shapes S1–S4 (S4 alone carries a corpus match).
- shipped `--min-rank1 36`: `FAIL · rank1 30/40` (S1 9 · S2 6 · S3 5 · S4 10; elsewhere 5 · none 5 · unparsed 0).
- no-framing `nf`: 18/40 (S1 6 · S2 6 · S3 1 · S4 5).
- Each arm ran once and was never re-run or re-thresholded.
- The framing stays shipped. The shortfall lies in S2 and S3, which carry no corpus match, and is owned by its own 0.3.0 route entry.
- The tuple, the coalesce predicate and the title grounding are unchanged.
**Why:** The block cleared and the pre-registered series ran. Under the P4 ruling (overseer, founder-delegated, 2026-10-04), a FAIL completes the measuring chunk, recorded as measured and never as passed. The remedy stays inside 0.3.0 under the founder ruling of 2026-10-02. Standing trap: S4 reading 10/10 means the corpus-match restatement hypothesis does not account for this FAIL. A remedy aimed only at corpus-match framing would miss the measured shortfall.
**Kept:** The framing itself, because it raised rank-1 retry naming from 18 to 30 of 40 and the corpus-match shape from 5 to 10 of 10. The probe's still-owed flag-parse pins (test-plan §1) are left to that row.
**Ref:** .andromeda/runs/2026-10-04T23-17-01Z-wrap/
