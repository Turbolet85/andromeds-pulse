# Fan-out results — 2026-10-04-l4-interpretation-names-its-triggering-cue

Seven Explore doc-agents, one parallel batch. Prompts are in `prompt-{doc}.txt`: detector counts
2 / 4 / 2 / 2 / 3 / 4 / 2 = 19, equal to the `doc:` names in drift-base. The contracts line was dropped
for arch / test-plan / obs-plan / a11y-plan, because `registry.py contracts` read `NOT MIGRATED` (exit 3).
Returns were parsed after stripping the trailing `#` commentary each agent appended; entities 0 (no HTML
escapes in any value). No raw twin was warranted: each empty return carried only commentary, and every
non-empty return parsed.

## Verdicts
- arch — 1 proposal
- security-plan — 2 proposals (one `dependent-of`)
- design-system — `proposals: []` (stripped: notes that D-design-tokens has no UI element and that no
  status surface moved)
- layout-templates — `proposals: []` (stripped: notes that there is no surface/region and no status claim
  moved)
- test-plan — 3 proposals
- obs-plan — `proposals: []` (stripped: notes that there is no new op or log, and §10 is untouched)
- a11y-plan — `proposals: []` (stripped: notes that there is no interactive element and no schema change)

## Proposals + dispositions

### arch-1 · D-arch-decisions · warning
- section: §Established Decisions → [Fault Identity]
- change: replace "The model-authored symptom, timeline and ranked hypotheses are untouched (a prompt-side
  remedy for the model layer is its own route entry)" with the landed remedy: the TRIGGER line + the
  corpus framing note + `TRIGGER_FRAMING_INSTRUCTION`, lineage v2.4 / v1.3-fallback / v1.3-reflection,
  framing not identity, the rank-1 effect UNMEASURED and gated on a Linux llama.cpp CUDA binary
- basis: architecture.md:72
- **disposition: APPLY** — check 1, playbook "Accurate this-chunk addition" (routine; named in the
  report's Changes). Check 5: it matches the plan's expected arch entry.

### sec-1 · D-security-input · escalate
- section: §Input Validation → the L4 inference argv prompt row
- change: re-base the observed maximum 6,932 B → 7,185 B (the 2026-10-04 v2.4 probe composition, S4
  shipped, synthetic; a measurement note, never a bound); ~2.28×; headroom 9,199 B ≈ 9.0 KiB; the
  OTLP-derived content set unchanged
- basis: security-plan.md:139
- **disposition: APPLY (routine by actual class)** — check 1, playbook rule "ESCALATE-severity detector
  fires OUTSIDE its class" (2026-08-23). Clause (a) holds: the report affirms no unvalidated boundary
  (`validate_prompt_bounded`, `every_arm_and_shape_composes_within_the_production_bound`, the closed
  `cue_cause_label` vocabulary, the content set unchanged). Clause (b) holds: the actual class is a doc-only
  stale measurement (the 2026-08-14 doc-correction rule), AND the plan's P5-approved expected amendment names
  this change itself ("re-base … at BOTH sites in lockstep, and only if the figure moves"). Not a boundary
  widening, since no new input class crosses argv.

### sec-2 · D-security-input · escalate · dependent-of D-security-input
- section: §Security Anti-Patterns → Code Patterns (the L4 `-p` prompt exception)
- change: the mirror re-base 6,932 B → 7,185 B, ~2.28×, in lockstep
- basis: security-plan.md:461
- **disposition: APPLY** — validates with sec-1 as a `dependent-of` group (same checks).

### test-1 · D-tests-coverage · warning
- section: §1 → `l4-decision-probe-arg-parse-unit-coverage`
- change: a partial discharge (the row stays open). `test = true` + 8 pins (`names_trigger` closed set,
  `nf` count-checked removals, S4, the every-arm bound); generations still never a gate. "Zero tests /
  neither a test" retired. A5 is now the identity, so the content-equality check runs for no arm. Still
  owed: the flag parse (incl. `--min-rank1`, `nf`), the INCONCLUSIVE exit 2, `first_keys`.
- basis: test-plan.md:144
- **disposition: APPLY** — check 1 "Accurate this-chunk addition". Check 5: it matches the plan's §1
  entry. Check 6: it disposes both of the report's test-plan §1 disproved claims.

### test-2 · D-tests-coverage · warning
- section: §4 → the interpretation crate bullet
- change: lineage v2.4 / v1.3-fallback / v1.3-reflection; the `TRIGGER_FRAMING_INSTRUCTION` every-tier pin
  (mutation (c) RED on fallback); ASCII covered by the existing template pin
- basis: test-plan.md:385
- **disposition: APPLY** — check 1 "Accurate this-chunk addition". Check 5: it matches the plan's §4
  interpretation entry.

### test-3 · D-tests-coverage · warning
- section: §4 → the triage crate bullet
- change: the 5 `render_payload` pins for the TRIGGER line + the corpus framing note
- basis: test-plan.md:384
- **disposition: APPLY, with a re-derived mutation clause.** The proposal's "dropping the TRIGGER line reds
  the pins" is NOT what the report measured. Mutation (a) keyed the line on the LAST cue and reddened the
  first-cue pin; (b) dropped the note and reddened 2 pins. The applied text follows the report (Apply step 1
  re-derives, never pastes). Check 5: it matches the plan's §4 triage entry.

## Validate summary
- check 1 playbook: 6 apply (4 "Accurate this-chunk addition", 2 by the escalate-outside-class rule)
- check 2 cross-contradiction: none (each proposal edits a distinct site)
- check 3 intent-consistency: the report matches the intent. The scope record's 3 companions each serve a
  listed path and add no behaviour (`mod.rs` / `contract.rs` re-export the consts `assembler.rs` defines;
  `unit_inference_runtime.rs` moves 3 version pins with `schema.rs`). The A5 identity serves the plan's own
  every-arm pin. No widening.
- check 4 absence: test-1's "runs for no arm" rests on the identity return (`cut_start > cut_end` ⇒
  return) and on the implement-run panic `l4_decision_probe.rs:281` before it. It is evidenced, not inferred.
- check 5 expected amendments: all 6 plan entries are matched by a proposal (arch · test §4 interp · test §4
  triage · test §1 · security ×2 as one entry over two sites); nothing raised by the orchestrator.
- check 6 disproved claims: the two test-plan §1 claims → test-1. The three chunk-artifact claims (the
  research sweep count, the plan Step 1 lineage prediction, the plan Step 12(d) prediction) → recorded in the
  report, no amendment owed (research/plan are closed; no sanctioned writer).

Escalations: 0.
