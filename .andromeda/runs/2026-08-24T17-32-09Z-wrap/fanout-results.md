# Fan-out results — 2026-08-23-headful-leg-extension wrap

7 doc-agents, one batch. Proposals: 16 agent + 2 orchestrator-raised = 18 · applied 16 · rejected 2 · escalations 0.

## Per-doc verdicts

- **arch** — 1 proposal (D-arch-decisions): §Stack GUI-harness Role cell 7 → 13 stages; grep-verified sole occurrence in arch. D-arch-resources clean (no new registry resource; xtask-internal symbols; the driver rides existing grants). **APPLY** (2026-07-08 accurate-this-chunk rule + expected-amendments floor).
- **security-plan** — 2 proposals (D-security-logging primary + dependent). **REJECT, routine over-reach**: the §Anti-Patterns→Logging INTENDED-posture sentence governs "any **attribute value**" (the OTLP/user-data class) — `window_label` is a product-internal bounded label produced by the pre-existing `sanitize_window_label` pattern that has always ridden the log (`layout_mode_from`, `tray_visible`, `error_kind`, …) without `scrub_attribute`; and §Logging & Monitoring→Log format's "field redaction is applied at the subscriber layer" remains true — the AllowList default-deny IS the redaction and governs the new field via its exact leaf; call-site bounding is cardinality discipline (obs-plan §5), not redaction. The chunk added one more instance of an established mechanism; neither sentence's claim moved. Dependent rejected with primary (group rule). No residual handoff gap: bounded-label discipline is owned by obs-plan §5/§6, where the new leaf is being registered this same wrap.
- **design-system** — clean (`proposals: []`); no new UI element, `tokens n/a` throughout, mutations reverted.
- **layout-templates** — 2 proposals (D-layout-surface primary + dependent): trigger #4 "fires ONCE per session" → every-time; §Primary screens "the first window-close-to-tray signpost" → drop "first". Grep-verified the only 2 occurrences. **APPLY** (2026-08-14 doc-only rule — the impl half [stale comments] was fixed in-chunk, so nothing else carries the old form).
- **test-plan** — 10 proposals (D-tests-coverage; 2 primaries + 6 dependents + 2 standalone): the 7→13 count at 6 sites; the flake root-cause rewrite carrying the operator's boundary directive (measured under the automation environment; production exposure unmeasured; no confining mechanism identified; open half owned on the working route); widget-close obs-half strengthening at 2 sites; the msedgedriver-trigger colocated count 27 → 88 (which also corrects the REPORT's own "no doc states the number" claim — §1's trigger row stated 27; the report's Counts bullet was wrong on that sub-claim and the proposal's fix is accepted as the accurate form). **APPLY all 10.**
- **obs-plan** — 1 proposal (D-obs-pii, static severity escalate): register the exact `tray.signpost.shown` → `window_label` leaf in §8. **APPLY as routine** per the codified 2026-08-16 rule — both load-bearing conditions hold: (a) the leaf enumerates the emit site's complete field list (one field; the guard's complete-set test asserts exactly-one), (b) the guard lives under `pulse-app/tests/` and RAN (mutation-checked RED 3/3 → GREEN 3/3 this session).
- **a11y-plan** — clean (`proposals: []`), with a load-bearing aside the orchestrator picked up (below).

## Orchestrator-raised (validation checks 5 + 6)

- **A (a11y-plan §1 P1)**: the row's tail "…their re-pointing is the one half still owed" is discharged — this chunk re-pointed the leg's Traces selectors onto the accessible anchors (report §Changes; `trace-table-empty` testid retained where no accessible anchor ships). Routine-APPLY; cross-master duplicate of the test-plan §6 selector discharge.
- **B (test-plan §6 Selector strategy)**: "the leg's own selectors … should migrate off `data-testid`" → discharged with the retention note. Routine-APPLY (expected-amendments floor; no detector proposed it).

## Disproved-claims disposition (check 6)
1. layouts "fires ONCE per session" → matched by layouts proposals (applied).
2. test-plan §6 leg description + flake note → matched by tests #1/#7 (applied), boundary per the operator directive.
3. plan.md RED-arm polarity → not a spec master; disposed via the report's own record + routed to P3 curation (the polarity lesson).

## Escalations
None. All dispositions matched codified playbook rules; no unease.
