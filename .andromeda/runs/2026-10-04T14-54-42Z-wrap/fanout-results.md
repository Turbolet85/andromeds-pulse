# Fan-out results — 2026-10-04-retry-storm-interpretation-names-its-cause

Seven Explore doc-agents ran in one parallel batch, prompts sent verbatim per `amendment-flow.md`. There were 19
detectors (arch 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 · obs-plan 4 · a11y-plan 2),
each naming one doc. No contracts line was sent: `registry.py contracts` exited 3 (NOT MIGRATED) for architecture,
test-plan, obs-plan and a11y-plan. Every return parsed cleanly. Stripping removed only `#` commentary lines (the
agents' reasoning, summarized below), and no entity or HTML escapes were present, so no raw twin was warranted.

## Verdicts

| doc | proposals | stripped commentary (substance) |
|---|---|---|
| architecture | 2 | — |
| security-plan | 0 | no new input surface; deps none; `security-plan.md:433` already states the three-path `scrubbed_l4_json` per-leaf scrub the chunk keeps; restating sites `:48`/`:166` generic and still true |
| design-system | 0 | no new UI; tokens n/a; no status claim touched (Halo signature / Motion / native / Self-Validation) |
| layout-templates | 0 | no surface added; Findings `:33` / Report `:34` entries make no title-format claim; no status claim touched |
| test-plan | 0 | both new paths tested at the mandated tier (`:175`, `:354`); runner on-spec (`:344`); harness unchanged; 0 title-format hits |
| obs-plan | 0 | no hot path or emit; leaves byte-unchanged (`:542-543`); §10 defect narrative (`:649-669`) does not describe this defect |
| a11y-plan | 0 | no interactive element; no schema change |

## architecture — parsed proposals and dispositions

1. **D-arch-resources** · warning · §Occupied Resources → `ANDROMEDA_PULSE_L4_DETERMINISTIC` (basis `architecture.md:236`)
   - change: record that the producer grounds the title with the cue's closed cause label before scrubbing, so
     `Incident.title` and the persisted L4 JSON title read `{Cause label}: {model title}` in every mode (creation,
     dedupe refresh, resolution), and a deterministic storm incident reads `Retry storm: Deterministic verification
     incident`. A deduped incident's title is not rewritten, and pre-build rows stay unprefixed.
   - **Disposition: APPLY.**
     - Check 1, playbook: it matches "Accurate this-chunk addition" (routine), and the plan's P5-approved Expected
       amendments (wrap) entry names this change itself.
     - Check 2: no contradiction.
     - Check 3: consistent with the intent (founder Q2), scope record none.
     - Check 4: the agent's 0-other-occurrence claim was re-derived by offset. A whole-file regex
       `(?i)\btitle|model-authored|incident text` gives 2 hits. `:72 @2747` "model-authored" concerns
       `L4Output.fingerprint`, a true claim, no change. `:195 @115` is the `investigate.run_action` DTO's `title`,
       a separate transient path not routed through the producer, no change. `splice.py summary`: 13 lines over
       2 000 chars, so the read was by offset.
     - Check 5: matches expected amendment 1.
     - Check 6: no disproved claims.
     - The applied text is re-derived from the report, not pasted.
2. **D-arch-decisions** · warning · §Established Decisions → [Fault Identity] (basis `architecture.md:72`)
   - change: add that the cue KIND, not only its fingerprint, now reaches the incident text as a closed cause label
     prefixed onto `Incident.title`, while the identity tuple, coalesce predicate, severity and evidence union are
     unchanged.
   - **Disposition: APPLY.**
     - Check 1, playbook: "Accurate this-chunk addition" (routine), plus the plan's Expected amendments entry naming
       the change.
     - Check 3: consistent with the intent.
     - Check 4: the same offset sweep as above.
     - Check 5: matches expected amendment 2.
     - The proposal's "coherent because kind is part of the identity key" is a design observation the report
       supports (identity `(kind, scope, scope_id)`), so it is kept as a scoped clause.

## Validate summary
- **Escalations:** 0.
- **Expected-amendments floor:** 2 of 2 matched by proposals.
- **Disproved claims:** none to dispose.
- **dependent-of groups:** none.
