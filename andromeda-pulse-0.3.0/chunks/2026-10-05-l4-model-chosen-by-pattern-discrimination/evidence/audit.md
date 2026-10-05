# The blind audit — plan Step 6

Labels and aggregates only. `audit-sample.md`, `audit-key.json` and `audit-verdicts.txt` stay under the series root in
the gitignored `target/`.

## The draw (plan entry 21, fired once by hand)

```text
audit: drew 96 of 960 rows · seed 418507 · sample audit-sample.md · key audit-key.json
```

- The seed is the pre-registered one, read from `preregistration.md` by the entry's own `sed`. exit 0.
- The sample was checked before hand-off: 96 sections, and 0 case-insensitive hits for any model or GGUF name
  (`llama|qwen|gemma|nemotron|nvidia|q4_k_m|gguf`). The model output shown per row is the extracted JSON object only,
  never the raw stdout (whose banner could name the model).
- Keyword cause labels in the sample: hit 13 · service_only 34 · word_only 3 · none 33 · not_scored 13.

## The overseer's blind read

The overseer (pc overseer, founder-delegated, 2026-10-05) read the 96 rows against the ground truth and wrote
`audit-verdicts.txt`: 96 verdict lines, 9 `disagree`.

## The grade (plan entry 22)

- First firing: `INCONCLUSIVE - audit-verdicts.txt line 2 unreadable` (exit 2). The verdict lines carried trailing
  `#` notes that the parser did not strip. Parser fixed, nothing else; see `preregistration-addendum.md`.
- Second firing, exit 0:

```text
audit: disagreement 9/96 · bar 10% · agree
```

Disposition: **agree** (9.4 %, not above the 10 % bar). No keyword fix, no re-grade. The table runs on the
pre-registered keyword sets.

## Findings — record-only (overseer, 2026-10-05), neither changes a label or the rule

1. **A1-cause is a lower bound.** In the A1 rows the audit disagreed with, the rank-1 hypothesis blames the new price
   cache client introduced 3 min ago in substance ("the cart-service's price cache client is not properly
   configured…"), but carries no pre-declared word (deploy, release, commit, change, rollout, rollback), so the
   keyword label reads `service_only`. The overseer attributes 4 of the 9 disagreements to this reading. The key maps 5
   of the 9 to A1 rows (the other four: 1 A7, 2 C1 enriched, 1 C3 enriched).
2. **A7 `word_only` via a substring.** The cause word `down` matches at the start of "downstream", so a statement
   naming the cue service X and a downstream dependency reads `word_only` with no failure word. Re-derived over the
   whole series: of the 21 A7 `word_only` rows, 1 matched only through `down`.
