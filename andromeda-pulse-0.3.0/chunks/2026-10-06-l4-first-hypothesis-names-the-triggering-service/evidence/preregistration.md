# Pre-registration record

- **Chunk:** `2026-10-06-l4-first-hypothesis-names-the-triggering-service`
- **Chunk base:** `b5138e208b9eea8742bfde78837279f346f988b1`
- **Written:** 2026-10-06T23:55:48Z, at /implement Step 1, before any other step and before any model run
- **Source:** `plan.md`, the section `## Pre-registration`, copied byte for byte below
  (sha256 of the copied section: `2a18f92ebc6c9cfc0f4152159f952f75c9ae5d8ee82bb68f8d0d2d98187d9ee7`)
- This file is never edited after the reading.

---

## Pre-registration (fixed before any run; /implement copies it to `evidence/preregistration.md` in Step 1)
- **Instrument:** `pulse-app/examples/l4_decision_probe.rs`, the tree as Step 8 leaves it; the real
  `build_primary_tier_prompt` and `render_payload`, production's argv, the shipped model
  `gemma-4-E4B-it-Q4_K_M.gguf` on the CUDA route (inputs#I6). n = 10 generations per arm per shape. Fired once; never
  re-run, re-ordered or re-thresholded.
- **The shipped sentence** (appended to `TRIGGER_FRAMING_INSTRUCTION`, one space after its last sentence, same line):
  `When the cue line under ATTENTION CUES carries a scope_id, the first hypothesis statement must name that scope_id value exactly as written there, and must not attribute the signal to anything else, including a service whose name merely contains it.`
- **Arms:**
  - `shipped` — the tree as it is: the instruction with the sentence, the trigger line as the kind label alone.
  - `ns` — `shipped` with the sentence removed (the v2.5 composition). The baseline; recorded, never in the rule.
  - `L` — `ns` with the digest's one trigger line rewritten to `TRIGGER: {cause label} on {scope_id}`. Line only.
  - `LI` — `shipped` with the same trigger-line rewrite. Both.
- **Shapes:** the four existing ordinary shapes S1, S2, S3, S4 (`l4_decision_probe.rs:266-314`) and two sibling shapes:
  - `S7` — a retry storm scoped to `conductor`; the services rows carry `conductor` and `conductor-canary`, both at a
    100 % error rate; no corpus match.
  - `S8` — S7 plus two corpus-match lines, rendered through the real `format_corpus_match_line`, for earlier
    retry-storm incidents scoped to `conductor-canary` whose titles name `Conductor-Canary` in the producer's
    `{Cause label}: {title}` form (one active, 3 minutes old; one resolved, 9 minutes old). This is the case d3 hit.
- **Label, per generation, over the FIRST hypothesis statement only, ASCII-lowercased** — the rule of inputs#I2:
  - service: the cue's `scope_id` occurs with neither neighbour in `[a-z0-9_-]`;
  - signal: some maximal run of ASCII letters equals `retry`, `retries` or `retrying`;
  - `both` · `service_only` · `signal_only` · `neither` · `unparsed` (no parse, or no first hypothesis).
  The rule's stated limits ride with it: negation is not read; a space-separated sibling passes; `retried` is not a token.
- **Bar, per arm:** MET iff `both` ≥ 19 of 20 over S7 + S8 AND `both` ≥ 36 of 40 over S1-S4. Denominators are fixed: an
  `unparsed` generation is not `both`.
- **Selection rule:** in the order `shipped`, `L`, `LI`, the first arm whose bar reads MET; `none` if none does.
- **Verdict:** PASS iff the selection is `shipped`. Otherwise NOT MET for the shipped product, and the selected arm (if
  any) is the candidate the founder is shown.
- **Regression guard** (inputs#I8): TRIPPED iff the `shipped` arm's `both` count is lower than the `ns` arm's on either
  half of the bar — over S7 + S8, or over S1-S4. HOLDS otherwise, an equal count included. TRIPPED → the sentence does
  not stay in the product: it and the lineage go back to v2.5 before the wrap, and the reading is recorded as read.
  A `shipped` arm that misses the bar while the guard HOLDS keeps its sentence, as in the 2026-10-04 slot-2 precedent.
- **What is recorded only:** the `ns` arm outside the guard, and through it the corpus-route question (S7 against S8
  under the pre-change render); every arm's per-shape counts; the footprint line.
