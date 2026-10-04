# Fan-out results — 2026-10-04-l4-framing-measured-on-the-real-model

Seven Explore doc-agents, one parallel batch; prompts from `amendment-flow.md`, substituted and sent verbatim (19 detector slots = 19 `doc:` names over drift-base; `{contracts_line}` dropped — `registry.py contracts` read NOT MIGRATED for architecture · test-plan · obs-plan · a11y-plan; the other three carry no keyed contracts). No return was altered by stripping (each returned YAML plus `#` comment lines; the comments were dropped, no proposal content in them); no entity escapes present; no raw twin warranted.

## Verdicts
- architecture — 1 proposal (D-arch-decisions)
- security-plan — `proposals: []` (D-security-input · -auth · -deps · -logging: no new input surface, secret flow, dependency or scrub boundary; the 7185 B dry-run max equals the observed max §Input Validation's argv row already states)
- design-system — `proposals: []`
- layout-templates — `proposals: []`
- test-plan — `proposals: []` (the `l4-decision-probe-arg-parse-unit-coverage` row at `:144` stays an open gap this chunk did not change)
- obs-plan — `proposals: []` (§10 defect list untouched; 0 hits for the series tokens)
- a11y-plan — `proposals: []`

## architecture — parsed proposal
```yaml
- detector: D-arch-decisions
  severity: warning
  section: §Established Decisions → [Fault Identity — what makes two faults ONE fault]
  change: Replace the clause "whether it moves the model's rank-1 hypothesis is UNMEASURED: the pre-registered real-model series (...) is gated on a Linux llama.cpp CUDA binary" with the measured result — shipped FAIL rank1 30/40 (S1 9 · S2 6 · S3 5 · S4 10), nf 18/40 (S1 6 · S2 6 · S3 1 · S4 5); framing kept; shortfall in S2/S3 (no corpus match) owned by a 0.3.0 remedy entry; identity tuple, coalesce predicate, title grounding unchanged.
  sidecar: arch [Fault Identity] — the UNMEASURED trigger-framing clause replaced by the measured real-model verdict; the decision unchanged.
  rationale: report Changes → Counts / qualifiers moved (the measured series), Spec claims disproved (the arch clause), Expected amendments (this clause); no new stack/library/runtime. Whole-file sweep for UNMEASURED / min-rank1 / names-trigger / pre-registered / rank-1 / framing / decision_probe / 'CUDA binary' → only :72, so no dependents. D-arch-resources: no drift.
  basis: .andromeda/architecture.md:72
```

## Validate
- **D-arch-decisions (Fault Identity)** → **apply**.
  - Check 1: settled by the recorded direction — the plan's P5-approved `Expected amendments (wrap)` entry names this change itself ("replace the UNMEASURED clause with the recorded verdict … the `nf` distribution beside it; the identity tuple, the coalesce predicate and the title grounding are left unchanged"). The playbook rule "ACCURATELY correcting a doc claim that a MEASUREMENT disproved, where an impl half DOES exist but is too large to ride this chunk (it needs its own route entry)" also governs: the remedy is the route entry minted at P5. Routine. Not a boundary widening.
  - Re-derivation guard: the rationale and basis cite only the report and the doc. The applied text is re-derived from the report's fact and scoped to the measured boundary: the model, the binary, n = 10 per shape over S1–S4. It is not pasted from `change`.
  - Check 2: no other proposal. Check 3: consistent with the intent (the entry's acceptance is the recorded verdict, PASS or FAIL). Check 4: the absence of other sites is backed by the agent's whole-file sweep, and re-swept by the cascade (`cascade-dispositions.md`).
  - Check 5: the plan's two entries are the arch amendment (this proposal) and the remedy route entry (FAIL only; P5, not a spec amendment).
  - Check 6: three disproved claims, each disposed:
    - the CONTEXT hypothesis is recorded in the report, a chunk-artifact claim with no amendment owed, and the remedy entry's CONTEXT carries the measured shortfall;
    - the arch clause is this proposal;
    - the plan's acceptance-1 `--out` wording is recorded in the report, a chunk-artifact claim with no amendment owed.
- Escalations: 0.
