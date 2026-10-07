# Pre-registration record

- **Chunk:** `2026-10-07-l4-probe-reproduces-the-canary-history-miss`
- **Chunk base:** `48714f09c95eaa0a2461d9a5ee26c96591983bb0`
- **Written:** 2026-10-07T10:32:16Z, at /implement Step 1, before any other step and before any model run
- **Source:** `plan.md`, lines 30-71, the section copied byte for byte below
  (sha256 of the copied section: `fd6d371430afda9a96e71cfa464eb76a11075e3166644f9bcf89e2f2c6f8dfc4`)
- This file is never edited after the reading.

---

## Pre-registration — the reproduction reading (fixed before any run; /implement copies it to `evidence/preregistration-reproduction.md` in Step 1)
- **Instrument:** `pulse-app/examples/l4_decision_probe.rs` as Steps 2-4 leave it: the product's `render_payload`,
  `select_corpus_matches` and `build_primary_tier_prompt`, production's argv, the shipped model
  `gemma-4-E4B-it-Q4_K_M.gguf` on the CUDA route (inputs#I9). Fired once; never re-run, re-ordered or re-thresholded.
- **Arm:** `shipped` alone, the tree's prompt `v2.6` with no transform.
- **n:** 20 generations per shape; 10 shapes; 200 generations.
- **Controls:** `S7` (no corpus block) and `S8` (two sibling retry-storm lines), unchanged.
- **New shapes.** Each has S7's two service rows and S7's cue (a retry storm scoped to `conductor`). Each corpus line
  is a synthetic incident rendered through the real `format_corpus_match_line`; ages are 3, 9, 12, 15 and 18 minutes,
  newest first. "Sibling" means scoped to `conductor-canary`. Unless a row says otherwise a line carries the cue's
  fingerprint and a descriptive title, as S8's do.

  | shape | what it holds | differs from S8 in |
  |---|---|---|
  | `S9` | five sibling retry-storm lines; the newest active, four resolved | count |
  | `S10` | two sibling lines: a retry storm (active, 3 m), an error-rate spike (resolved, 9 m) | kind mix |
  | `S11` | S10's two lines with the ages swapped: the spike newest and active, the storm older and resolved | position, against S10 |
  | `S12` | S8's two lines carrying a fingerprint other than the cue's | fingerprint |
  | `S13` | S8's two lines with titles that read as a hypothesis statement naming the sibling | title form |
  | `S14` | five sibling lines, other fingerprint, statement titles: one retry storm (newest, active) and four error-rate spikes (one active, three resolved) | count, kind mix, fingerprint, title form |
  | `S15` | S14's five lines with the retry storm oldest and resolved, a spike newest and active | position, against S14 |
  | `S16` | five lines of mixed scope: a sibling retry storm (active, 3 m, other fingerprint), a `conductor` retry storm (resolved, 9 m, the cue's fingerprint), two sibling error-rate spikes (12 m active, 15 m resolved, other fingerprint), a `conductor` retry storm (resolved, 18 m, the cue's fingerprint); statement titles | scope mix |

  `S14` is the shape the founder named (inputs#I1 §2 item 1). `S16` is the shape the third drive's capture bounds
  (scope.md §Mechanism claims; inputs#I2 `:407-413`, `:422`, `:450-451`). Titles are synthetic ASCII; no title seen in
  a capture is copied.
- **Label, per generation:** the predecessor's `identifies`, unchanged, over the FIRST hypothesis statement only. A
  **service miss** is a generation labelled `signal_only` or `neither`: it parsed, and its first hypothesis does not
  name the cue's `scope_id` as a whole word. `unparsed` is counted apart and is never a miss.
- **Per shape:** REPRODUCES iff its service misses are 2 or more of 20 (the predecessor's 19-of-20 sibling minimum,
  applied to one shape). CLEAN iff its misses plus its `unparsed` rows are 1 or fewer. Otherwise UNREAD.
- **Verdict:**
  - REPRODUCED (exit 0) iff at least one of `S9`-`S16` reproduces.
  - NOT REPRODUCED (exit 1) iff every one of `S9`-`S16` reads CLEAN. This is a finding, never a pass: the chunk stops
    for the founder.
  - INCONCLUSIVE (exit 2) otherwise: no shape reproduces and at least one is UNREAD.
- **Controls in the verdict:** `S7` and `S8` are read by the same per-shape rule and printed. A control that
  reproduces is named in the report and voids the single-property comparisons made against it; it does not change the
  verdict.
- **What is recorded only:** every shape's label counts; which single-property shape reproduces, as the reading of
  which property carries the miss; the footprint line. At n = 20 a clean shape shows that 20 generations did not miss,
  not that the property has no influence.
