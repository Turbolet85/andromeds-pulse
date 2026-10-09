# Fan-out results — 2026-09-30-span-level-redaction

Seven Explore doc-agents, one parallel batch, prompts verbatim from amendment-flow.md (contracts line dropped: arch ·
tests · obs · a11y render `NOT MIGRATED`; security · design · layouts carry no keyed-contract section). Entity probe:
no `&lt;`/`&gt;`/`&amp;` in any return (entities=0).

## Verdicts
- architecture — 2 proposals (stripped: orchestrator notes — D-arch-resources no violation; Fault Identity arch:72 no derivation claim; arch:235 / :195 / :42 swept, unchanged)
- security-plan — 6 proposals (stripped: notes — D-security-input/auth/deps no drift; :436 labels and :446 Residual still accurate; training export not enumerated in security-plan)
- design-system — `proposals: []` (stripped: basis comments → `.raw-fanout-design-system.md`)
- layout-templates — `proposals: []` (stripped: basis comments → `.raw-fanout-layout-templates.md`)
- test-plan — 1 proposal (stripped: no-drift notes — consumer pins, §4 buffer :372 still holds, 0 `whole value`/`one placeholder` hits; framework and §3 bind untouched)
- obs-plan — 1 proposal (stripped: notes — D-obs-stack / D-obs-pii / D-obs-defect-narrative no drift)
- a11y-plan — `proposals: []` (stripped: comments → `.raw-fanout-a11y-plan.md`)

## Parsed proposals + dispositions

### architecture
- A1 · D-arch-decisions · warning · §Conventions → Primary key convention (arch:88) — the metric name is span-masked at ingestion (`mask_secret_spans`); two distinct names collide on one placeholder only when they differ solely inside a masked span; a single-token credential-shaped name still masks whole; `seq` stays the disambiguator.
  **Disposition: APPLY** — check 1: Boundary widening (escalate) → resolved by the founder's P4 ratification «Ок давай по типу правила» (recorded in plan.md Metadata / scope.md Boundaries); with "Accurate this-chunk addition" (routine) for the reconciliation itself. Check 5: plan expected amendment (PK convention) — covered.
- A2 · D-arch-decisions · warning · dependent-of D-arch-decisions · §Occupied Resources → Out-of-data-dir egress sink (training export) (arch:218) — egress scrub is `mask_secret_spans`; `interpretation` masked per JSON string leaf, non-JSON whole; `count_redactions` counts a field CONTAINING `[redacted:`.
  **Disposition: APPLY** — same group; the report's Schema bullet (training export) carries every fact.

### security-plan
- S1 · D-security-logging · warning · §Security Anti-Patterns → Logging, catalog paragraph (:428) — retire "Whole-value replacement is unchanged (P-048)" → the class-aware span extent; fail-closed; bounded fixpoint; whole-value only at `metrics_points.labels`; opening actor `mask_secret_spans`.
  **Disposition: APPLY** — Boundary widening resolved by the P4 ratification; check 5 covered (catalog paragraph).
- S2 · dependent-of · INTENDED posture (:430) — `mask_secret_spans` gated by `scrub_attribute`'s verdict; labels keep `scrub_attribute` whole-value. **APPLY.**
- S3 · dependent-of · MEASURED reality, incident-summary clause (:432) — per-STRING-LEAF masking, stays parseable; no collapse, no pending branch from a redaction. **APPLY** — check 5 covered (incident-summary sentence).
- S4 · dependent-of · identity columns + `metric_name` Residual (:434) — span-masked identity cells; Residual narrowed to names differing only inside a masked span. **APPLY** — check 5 covered.
- S5 · dependent-of · §Threat Model Summary → corpus Sensitivity note (:48) — actor `mask_secret_spans`. **APPLY** — check 5 covered.
- S6 · dependent-of · §Data Protection → At rest → Persistent incident corpus (:166) — actor `mask_secret_spans`. **APPLY** — the report forecast 0 change at :166 (no whole-value claim) but the site names `scrub_attribute` as the corpus-write mechanism, which the report's Symbols bullet retires; the detector's reading stands.

### test-plan
- T1 · D-tests-coverage · warning · §4 → What unit tests cover → security crate (:373) — record the `mask_secret_spans` coverage (31 pins + proptests, mutation-checked); crate suite 54 → 85; `scrub_attribute`/`ScrubbedValue` still asserted unchanged.
  **Disposition: APPLY** — "Accurate this-chunk addition" (routine); check 5 covered.

### obs-plan
- O1 · D-obs-instrumentation · warning · §5 Metric Coverage → `redactions_applied` row (:356) — state the scalar-cell unit (one increment per redacted VALUE however many spans were masked) beside the labels unit; the drain/template path now masks via `mask_secret_spans`.
  **Disposition: APPLY** — routine (accurate this-chunk addition); check 5 covered (obs §5 row; §1 :101 and §8 :521 carry no unit — no change). The collateral "scrub_attribute has one production caller" is not applied to obs-plan (a security fact; it lands in security-plan S1/S2).

## Validate checks
1. Playbook — 10/10 governed: Boundary widening (escalate) resolved by the founder's P4 live ratification «Ок давай по типу правила», relayed by the overseer, which the wrap directive asks to be quoted in every widening amendment; reconciliation itself "Accurate this-chunk addition" (routine). No re-derivation tell: every `basis` cites the doc itself or the report.
2. Cross-contradiction — none (S1–S6 one group; A1/A2 one group; no two proposals edit one section in opposing directions).
3. Intent-consistency — the report's deviations are justified (fixpoint: measured necessity; training-export leaf masking and the Cyrillic swap: the overseer's word; the scope record's one `mechanical` line serves step 8). No escalation.
4. Absence needs evidence — the report's [Fault Identity] "not carried" rests on `grep -c scope_id architecture.md` = 1 (arch:72, the cue tuple), read by the arch agent; arch:72 is a long line — its only `scope_id` hit is the `(kind, scope, scope_id)` tuple. Disposed.
5. Expected amendments — all 9 entries covered: security :428 (S1) · :432 (S3) · :434 (S4) · :436 (no change, labels whole-value stands) · :48 / :166 (S5 / S6) · arch Fault Identity (no claim to amend — stated) · arch :88 (A1) · test-plan :373 (T1) · obs :356 (O1).
6. Disproved claims — all four are CHUNK-ARTIFACT claims (plan.md idempotence note · Step 8 m1 prediction · Step 5 training-export · the `redactions_applied` probe's poll note): recorded in the report, no amendment owed (no writer at wrap). The pipefail-poll class routes to P3 curation; the Cyrillic-lint pair routes to P5 (PREREQ).

## Escalations
None open (the one escalate-class pattern is resolved by the recorded P4 ratification).
