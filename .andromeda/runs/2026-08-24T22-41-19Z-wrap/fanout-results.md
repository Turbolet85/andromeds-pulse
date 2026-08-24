# Fan-out results — 2026-08-24-headful-mechanics-probe-race-disposition

7 doc-agents, one per spec source. 15 proposals returned.

| doc | proposals | verdict |
|---|---|---|
| design-system | 0 | clean — no product UI element added (report: "the chunk adds observation, not surface"); all 3 new surfaces `tokens n/a` |
| layout-templates | 0 | clean — `DRAG_REGION` binds the already-documented §Component Custom-titlebar drag region; the 4 windows already in §Primary screens |
| a11y-plan | 0 | clean — no interactive element added; the new obs record is a new leaf INSIDE the existing obs §6 envelope, not a schema change |
| architecture | 1 | **APPLY** — D-arch-decisions §Stack GUI-harness Role cell 13 → 15 stages |
| obs-plan | 2 | **APPLY** — D-obs-instrumentation §6 warn row + D-obs-pii §8 whitelist (the dual-site pair) |
| test-plan | 9 | **APPLY** — 1 primary + boundary correction + 2 §1 items + 5 duplicate-occurrence dependents |
| security-plan | 3 | **REJECT (routine over-reach)** — see below |

## Validation (the 5 named checks + disposition)

**1. Playbook check.**
- arch / tests stage-count + boundary + count re-base → 2026-07-08 rule (accurate this-chunk correction inside an existing structure) → routine-APPLY.
- obs §6 + §8 pair → 2026-08-16 rule (new self-observation target, bounded non-PII fields, exact leaf, guard test). Both load-bearing conditions verified: (a) the leaf enumerates ALL THREE fields the emit site emits; (b) the guard lives in `pulse-app/tests/`, where `[lib] test = false` lets it actually run. → routine-APPLY, no escalation despite D-obs-pii's escalate severity.
- security trio → 2026-06-28/06-30 generalized over-reach rule → routine-REJECT.

**2. Cross-contradiction.** None — no two proposals edit one section in opposing directions.

**3. Intent-consistency.** Report matches the chunk's working-route entry + plan acceptance criteria; the deviations carry justifications. Aligned.

**4. Absence needs evidence.** Each absence claim cites its sweep: arch enumerated the other `13` hits (tonic 0.13 · cudart64_13.dll · a decryption count); tests named 6 stating sites with line numbers; security evidenced why §Threat Model's column-coverage duplicate is NOT stale. Accepted.

**5. Expected-amendments reconciliation.** The plan's four entries are all covered (test-plan §6, arch §Stack, obs-plan §6+§8, test-plan §1). The detectors found FIVE ADDITIONAL restating sites the plan's list missed (test-plan §1 P5 row, §2 agent-runnable invariants, §6 P5 Status, §6 full-P5-coverage, §9 CI table) — the duplicate-occurrence sweep doing its job; the plan's list was the floor, not the ceiling.

**6. Disproved-claims disposition.** All 6 entries disposed: #4 (§6 boundary) and #5 (Actions/setWindowRect) by the tests proposals; #1/#2/#3 (route-CARRY premises) route to P5 route-resolve; #6 (P-064/P-065 "folded into P-076") routes to the P7 matrix premise-correction.

## Security trio — REJECTED, with evidence

The proposals claim this chunk introduces a first-party record class gated by bounded ENUMERATION rather than `scrub_attribute()`, making the section's coverage claim over-broad. Three facts defeat that premise:

1. **The class predates this chunk by one chunk.** `tray.signpost.shown` (obs-plan:526, shipped at `2026-08-23-headful-leg-extension`) is the identical shape — `window_label` produced by the same pre-existing `sanitize_window_label`, an exact leaf, a guard test under `pulse-app/tests/`, and no `scrub_attribute` anywhere. So this chunk did not introduce the class; the generalized over-reach rule applies.
2. **The gate sentence is already scoped and remains true.** security-plan:427 reads "any **attribute value** crossing into persistent storage OR the self-observation log sink passes through `scrub_attribute()`". A bounded internal window label is not an OTLP attribute value, so the sentence is not over-broad.
3. **The Log-format sentence is also still true.** "Field redaction is applied at the subscriber layer, not at log call sites" — the ALLOWLIST *is* the subscriber-layer redaction; `sanitize_window_label` bounds the value's cardinality, it does not perform the redaction.

The third proposal carried `dependent-of` the primary and is rejected with it (atomic group).

**This is recurrence #2 of the identical pair.** The previous wrap (`2026-08-23-headful-leg-extension`) rejected the same two security proposals against the same sections for the same stated reason ("`window_label` is a bounded internal label, not an 'attribute value'; the allowlist IS the subscriber-layer redaction"). Two consecutive wraps is the playbook's own stated recurrence bar for codifying a rule — proposed to the operator at this wrap.

No genuine pre-existing doc gap was identified in the rejected material, so nothing is handed off from it.
