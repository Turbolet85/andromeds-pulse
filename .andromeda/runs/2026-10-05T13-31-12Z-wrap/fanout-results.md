# Fan-out results — 2026-10-05-l4-model-chosen-by-pattern-discrimination

Seven Explore doc-agents, one parallel batch, the verbatim amendment-flow prompt; detectors scoped from
`drift-base.md` (19 entries; per-doc counts arch 2 · security-plan 4 · design-system 2 · layout-templates 2 ·
test-plan 3 · obs-plan 4 · a11y-plan 2 = 19, equal to the `doc:` names). Keyed-contract renders: architecture,
test-plan, obs-plan, a11y-plan → `registry.py contracts` exit 3 NOT MIGRATED → the contracts line dropped. Every
return was YAML with trailing `#` commentary; stripping removed only commentary (each detector's no-drift reasoning).
No HTML entities in any return (entities=0). No raw twin: no `proposals: []` return changed in substance, none failed
the parse.

## Verdicts

- architecture — 2 proposals (stripped: no-drift notes for D-arch-resources and the stack half of D-arch-decisions;
  §Stack :30, :367, :232, :194 checked, no model named)
- security-plan — 2 proposals (stripped: no-drift notes for D-security-auth, D-security-deps, D-security-logging,
  and the new-surface half of D-security-input)
- design-system — `proposals: []` (stripped: no-drift notes; the Halo DEFERRED status sites untouched)
- layout-templates — `proposals: []` (stripped: no-drift notes)
- test-plan — 1 proposal (stripped: no-drift notes for D-tests-framework, D-tests-obs-harness)
- obs-plan — `proposals: []` (stripped: no-drift notes; 0 hits for Llama-3.2 / gemma / 7,405 / gpu-primary / MAX_PROMPT_BYTES / l4_decision_probe)
- a11y-plan — `proposals: []` (stripped: no-drift notes)

## Parsed proposals and dispositions

### architecture
1. D-arch-decisions · warning · §Established Decisions → [LLM Inference Runtime] → Model · retire the future tense
   "the model that ships is chosen by the pattern-discrimination route entry"; record the table's recommendation,
   the founder's pick of gemma-4-E4B-it-Q4_K_M over the QAT build, the shipped GGUF unchanged until the next entry,
   and the next entry's founder question (p50 above the 5 s budget). basis `architecture.md:71`.
   **apply** — check 1: playbook "Accurate this-chunk addition" (routine), and the plan's Expected amendment names
   the change itself; text re-derived from the report, not pasted.
2. D-arch-decisions · warning · [Fault Identity] tail · dependent-of D-arch-decisions · "the remainder is owned by the
   pattern-discrimination route entry, which chooses the model" → chosen; the remainder now with the entry that
   ships the pick. basis `architecture.md:72` (offset ~8219). **apply** — the primary's duplicate occurrence (check 1;
   the group applies atomically).

### security-plan
1. D-security-input · escalate · §Input Validation → L4 argv row · re-base the observed maximum 7,405 B → 7,575 B
   (A6 pattern composition, dev-probe `--dry-run`, primary builder only), ratio ~2.21× → ~2.16×, headroom 8,979 B →
   8,809 B; ceiling unchanged. basis `security-plan.md:139`. **apply** — check 1: playbook "An ESCALATE-severity
   detector fires on a finding OUTSIDE the class…" → routine-apply-by-actual-class; (a) the report affirms no
   unvalidated boundary (every new input validated by `parse_args_from` / the out-dir guard, unit-tested); (b) the
   actual class is the plan's named lockstep re-base (Expected amendments) under "Accurate this-chunk addition".
   Arithmetic re-derived: 16384 / 7575 = 2.163; 16384 − 7575 = 8809.
2. D-security-input · escalate · §Security Anti-Patterns → Code Patterns argv bullet · dependent-of D-security-input
   · the same re-base in lockstep. basis `security-plan.md:461`. **apply** (as 1; the group atomically).

### test-plan
1. D-tests-coverage · warning · §1 → `l4-decision-probe-arg-parse-unit-coverage` · 29 → 61 collected pins, the 32
   pattern pins named, mutation-checked (a)–(f) + (e'); the STILL OWED flag list narrowed to read the new flags as
   pinned. basis `test-plan.md:144`. **apply** — check 1: "Accurate this-chunk addition" + the plan's Expected
   amendment.

## Validate — the six checks

1. Playbook — 5 apply (above); 0 escalate.
2. Cross-contradiction — none (no two proposals on one section in opposing directions).
3. Intent-consistency — the report's deviations each carry a justification; the QAT face-to-face and the operator
   pass were operator-directed (quoted); scope record none (`gate.py scope` clean). No unjustified divergence.
4. Absence-needs-evidence — the agents' no-other-occurrence claims rest on greps the report carries
   (`grep -c '7,405' security-plan.md` = 2; `l4-decision-probe-arg-parse-unit-coverage` 1 in test-plan;
   `Llama-3.2-3B-Instruct-Q4_K_M` 2 in architecture); the arch :72 hit at offset ~6523 (the v2.5 framing series'
   measured model) is a true past measurement, no change — window read by offset (`cascade-dispositions.md`).
5. Expected-amendments reconciliation — all three plan entries proposed (arch Model · test-plan §1 · security-plan
   lockstep re-base); none raised by the orchestrator.
6. Disproved-claims disposition — (i) research.md's module-resolution convention: a chunk-artifact claim → recorded
   in the report, no amendment owed; routed to P3 curation as a sweep hazard. (ii) inputs#I4's "pre-registered seed":
   a relay claim → recorded in `preregistration-addendum-2.md` and the report, no amendment owed. (iii) the 7,405 B
   observed maximum → matched by security-plan proposals 1–2. All DISPOSED.

Escalations: 0. No HALT.
