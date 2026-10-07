
## 2026-10-07-l4-probe-reproduces-the-canary-history-miss — the probe row at 157 pins; the triage selection pins
**Section:** §1 Pending coverage triggers, row `l4-decision-probe-arg-parse-unit-coverage` · §4 Unit Test Strategy, triage crate
**Change:**
- §1 row, a dated clause after the 91-pin clause: 157 collected pins (66 new, 29 of them in the second `#[path]` module `l4_decision_probe/replay.rs`), `ARMS` 18 → 23 (`nb` / `CR` / `CO` / `CC` / `CX`), the shape set S1–S17; the replay input and its validation; `--sections` and `--product-path`; the candidate arms as edits of the baseline arm `nb`; `CX` pinned byte-identical to `shipped` on every S shape; the 16 pins of a measured block moved to `nb`, because `shipped` now drops or narrows the block on S4 / S12 / S14 / S15 / S16; 76 scripted mutation checks, 61–64 of them for `CO` product pins that no longer exist; the probe keeps no durable known-positive.
- §1 row, STILL OWED: the "every flag but" list gains `--replay` / `--replay-scope` / `--own-lines` / `--reproduce-misses` / `--remedy-from` / `--remedy-read-as` / `--count-naming` / `--sections` / `--product-path`. The rest of the owed list is unchanged.
- §4 triage: the bullet named the render pins and no selection pin; it now carries the corpus selection under a triggering scope and its 5 in-crate pins (3 `select_corpus_matches_*`, 2 `assemble_*`), mutation checks 65–69, crate suite 497 passed.
**Why:** The probe and the triage crate gained collected pins and the row's counts were stale. Trap for later chunks: a product change to what `shipped` composes reddens every pin of a measured block at once; those pins move to the baseline arm, never to the new output.
**Kept:** The row's earlier dated clauses (8, 16, 29, 61 and 91 collected pins) as history, the S4 framed-corpus-match clause among them: that pin now composes under `nb`.
**Ref:** .andromeda/runs/2026-10-07T20-29-42Z-wrap/
