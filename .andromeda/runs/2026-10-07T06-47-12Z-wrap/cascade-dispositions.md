# Cascade dispositions — the 2026-10-07T06-47-12Z wrap

Written from the `cascade.py sweep` listing of this pass (trail `cascade-{marker}.json`), after every body was
amended and before any sidecar entry landed. Baseline: `b5138e20`, the parent of the pre-CI commit.

## The search

19 patterns in `cascade-patterns.toml`, each with a control that fired on the pre-pass masters:
- the lineage: `v2\.5\b` · `v1\.4-(fallback|reflection|\*)`
- the instruction's wording and mechanism: `own words` · `separate first hypothesis` · `first hypothesis`
  (case-insensitive) · `TRIGGER_FRAMING_INSTRUCTION` · `cue_cause_label`
- the argv-prompt measurement: `7[,.]?575` · `2.16` · `observed maximum` · `8[,.]?809` · `are unmeasured` ·
  `MAX_PROMPT_BYTES|16[,.]?384`
- the probe's coverage row: `128 pins` · `61 collected pins` · `\bARMS\b` · `S1[-–]S6` ·
  `l4-decision-probe-arg-parse-unit-coverage` · `l4[_-]decision[_-]probe`

Swept: the seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and
the leaf bodies. Not looked for: the probe's shape ids as bare words (`S7`, `S8`), the arm names (`ns`, `L`, `LI`) and
the service names of the sibling shapes — none is a retired claim. A second, hand-run read of the leaves (CLAUDE.md,
`.claude/docs/**`, `.claude/rules/*`; pattern `[Ff]raming instruction|TRIGGER[:_ ]|prompt lineage|PROMPT_VERSION|argv
prompt|validate_prompt_bounded|MAX_PROMPT_BYTES|l4_decision_probe|first hypothesis|observed max|v2\.\d\b`, 11 lines)
confirmed what the sweep printed: no leaf carries the lineage, the instruction's obligation list, the observed
maximum or the probe's pin counts.

Every amended site sits on a line over 2,000 chars; each was read by offset window or whole before it was edited.

## Zero-row patterns (their controls fired)

`2.16` · `8,809` · `are unmeasured` · `128 pins` — each retired wording is gone from every master, registry file,
leaf, curation home and base. A statement about these four patterns.

## Rows, by pattern

- **lineage `v2.5`** — arch:73 `standing edited ×3`: `@c7016` is the amended clause naming v2.5 as the PRIOR lineage
  (this pass's text); `@c8919` and `@c10406` are dated accounts (the 2026-10-05 model-choice series ran "under v2.5";
  the new reading names "the v2.5 baseline `ns`") — true as dated, no change. security-plan:139 ×2 and :463 ×3 —
  dated measurement notes ("the 2026-10-05 v2.5 composition", "were unmeasured at v2.5", "A6 at v2.5"), this pass's
  text or true as dated, no change. test-plan:229 `new` — this pass's prior-step clause.
- **lineage `v1.4-*`** — arch:73 ×2 and test-plan:229: the prior-lineage clauses this pass wrote. No other site.
- **`own words` · `separate first hypothesis` · `first hypothesis` · `TRIGGER_FRAMING_INSTRUCTION`** — arch:73 and
  test-plan:229 only (amended lines: the three earlier obligations stand beside the new one; the later
  `first hypothesis` hits on arch:73 are the dated 2026-10-04 grader account and this pass's reading);
  test-plan:144 ×2 — this pass's `identifies` clause and the row's earlier `names_trigger` account, true as dated.
  No leaf, no curation home, no base.
- **`cue_cause_label`** — arch:73 ×2 (the title grounding, untouched; the TRIGGER line statement, amended);
  arch:239 (§Occupied Resources, the export's registry line — a true claim sharing the token, no change);
  test-plan:228 (the triage bullet: exactly one `TRIGGER: {cue_cause_label}` line keyed on the first cue — true,
  no change). Leaves: **CLAUDE.md:50 re-derived** (the Fault Identity warning now states the line carries the kind
  label only and that the service reaches the first hypothesis through the framing instruction);
  **`.claude/docs/services/triage.md:28` re-derived** (the same clause on the digest line); triage.md:32 (the
  export's description — true, no change).
- **`7,575`** — security-plan:139 and :463: this pass's text (the prior-maximum note). **`observed maximum`** —
  the same two sites, amended to 7,824 B. **`MAX_PROMPT_BYTES|16384`** — security-plan:139 ×2, :463, test-plan:144:
  the bound itself, unchanged by the chunk, no change.
- **`61 collected pins`** — test-plan:144 ×2: the two dated earlier clauses of the row (2026-10-05), true as dated;
  the new clause states 91. **`ARMS`** — test-plan:144 ×3: two dated earlier steps (14 → 16, 16 → 15) and this
  pass's 15 → 18. Curation rows `.claude/rules/security.md:167` and `.claude/rules/testing.md:284`: the word "ARMS"
  in an unrelated sense (a regex catalog's arms; a guard's two arms) — not the claim, no change. The 50
  case-variants are the common noun.
- **`S1–S6`** — test-plan:144: a dated earlier clause (the bound pin "now also asserts the shape set S1–S6", written
  at the 2026-10-04 rank-1 chunk); the new clause states S1–S8. Left as dated history, like the row's other steps.
- **`l4-decision-probe-arg-parse-unit-coverage` · `l4_decision_probe`** — test-plan:144 (the row, amended). Leaf
  `.claude/docs/services/interpretation.md:38` ("reuses the product argv builder, and its grammar file") — true,
  no change.

## Lateral binds

`test-plan §3 ↔ obs-plan §3` and the a11y ↔ obs violation schema: no harness command, status shape, log format or
schema was amended in this pass. Nothing to reconcile.

## Leaves, by changed source (re-computed, not scanned)

- **architecture.md** (§Established Decisions [Fault Identity]): CLAUDE.md `GENERATED:setup:warnings` — the Fault
  Identity bullet re-derived (above); the other generated blocks derive from sections this pass did not touch.
  `.claude/docs/services/triage.md` — the digest line re-derived (above). `.claude/docs/services/interpretation.md`
  states neither the instruction's obligations nor the lineage, before or after this pass: nothing to recompute.
  `conventions.md` and `stack.md` (the two leaves whose provenance header names architecture.md) distill §Conventions
  and §Stack, untouched.
- **security-plan.md** (§Input Validation row; §Security Anti-Patterns → Code Patterns): `security-summary.md:48` and
  `.claude/rules/security.md:17` state the one-OTLP-derived-operand rule and the bound without any measured figure —
  still current, no change. CLAUDE.md warnings: no line derives from the measurement note.
- **test-plan.md** (§1 trigger row; §4 interpretation crate): `tests-summary.md` and the body of
  `.claude/rules/testing.md` carry neither the probe's row nor the lineage — no change. The two `v2.x` hits in
  testing.md are inside `## Session Additions` (dated examples of a sweep hazard), preserved verbatim.
