# Fan-out results — 2026-08-27-report-window-copy-affordance

7 doc-agents, one per spec source. All returned clean YAML; no return needed stripping beyond the
code-fence, so no raw twins are warranted (per amendment-flow §Fan-out).

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | **1 proposal** (D-arch-decisions) |
| security-plan | D-security-input · D-security-auth · D-security-deps · D-security-logging | `proposals: []` |
| design-system | D-design-tokens | `proposals: []` |
| layout-templates | D-layout-surface | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | **6 proposals** (1 primary + 5 `dependent-of`) |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-pii | `proposals: []` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` |

## Proposals

**arch / D-arch-decisions** — §Stack and Technologies → GUI verification harness (dev-only) Role cell:
assembled path 15 → 16 stages; `report-copy` enumerated between `report-window` and `investigate`; growth
lineage extended. Agent reports a single occurrence in arch, citing its own grep plus the precedent already
recorded at `architecture-amendments.md:319` ("arch §Stack is the only site in this document stating the
stage count").

**test-plan / D-tests-framework** — one primary + five `dependent-of` duplicate-occurrence proposals:
1. §6 Drivers per surface → desktop-webview row (primary)
2. §2 Agent-runnable invariants
3. §1 Critical paths → P5 Verification Signal
4. §6 Scenario P5 → Status ("fifteen" spelled)
5. §6 Scenario P5 → Full P5 coverage (lineage)
6. §9 CI Integration → E2E tests row (lineage; NOT-CI-wired ruling unchanged)

## Validate (orchestrator)

1. **Playbook** — all 7 match `playbook.md` "Drift proposal registering an ACCURATE this-chunk
   addition/correction WITHIN an already-documented doc structure" → **routine**. No escalate verdicts.
2. **Cross-contradiction** — arch and test-plan move the same count in the SAME direction (15 → 16); no
   opposing pair. PASS.
3. **Intent-consistency** — the report matches the chunk's working-route entry and its plan acceptance
   criteria; no divergence to classify. PASS.
4. **Absence needs evidence** — both "single/complete occurrence" claims independently corroborated by the
   orchestrator's own greps BEFORE reading the proposals: `15-stage|15 stages|FIFTEEN` returns arch **1**,
   test-plan **6**, rules/testing **2**, rules/verification-harness **2**. The 6 test-plan proposals map
   1:1 onto the 6 measured line hits (78 → §1 P5 · 161 → §2 · 414 → §6 driver row · 508 → §6 P5 Status ·
   516 → §6 full-coverage · 624 → §9 E2E row). Sweep complete. PASS.
5. **Expected-amendments reconciliation** — the plan listed TWO:
   - `architecture.md` §Stack Role cell → **PROPOSED** by the arch agent. Covered.
   - "Possibly `test-plan.md` §1 — a trigger row if the ACL-revoke discrimination remains unautomated
     beyond this chunk's RED-first proof" → **no detector proposed it; raised by the orchestrator and
     resolved NOT-OWED, substantiated by the report.** The condition ("remains unautomated") is not met:
     this chunk's discrimination proof was the REAL pre-existing defect rather than a hand-revoke cycle
     (RED at HEAD `copy_state:"error"` → GREEN `copy_state:"copied"`, same stage, both recorded), and the
     forward regression guard is code — reverting the grant reddens `report-copy` on the shipped leg, and
     `--no-inject --expect-absent report-copy` passed as the code-driven arm. The standing §1 trigger
     `webview-drive-mutation-arm-not-gated` (no shipped gate catches a left-revoked grant; webview-drive is
     not CI-wired) is untouched by this chunk — neither closed nor worsened — so no NEW trigger row is owed
     and the existing one needs no edit. Recorded rather than silently dropped, per the check's own rule.
6. **Disproved-claims disposition** — the report's bullet carries two entries, both DISPOSED:
   - `.claude/rules/testing.md` §Session Additions 2026-07-05 warm-re-embed recipe (measured false for the
     two release-preferring headful harnesses) → **routed to P3 curation**. It sits inside a
     preserve-verbatim `## Session Additions` block, which amendment-flow §Cascade forbids the cascade from
     editing and explicitly routes to curation "as an in-place extension of the stale entry".
   - The plan's step-6/7 sequencing inheriting the same premise → a CHUNK-ARTIFACT claim, whose durable home
     is this report (2026-08-26 ownership rule); no amendment owed, and the report says so inline.

**Escalations: 0.** All 7 proposals staged to apply.

## Apply + cascade (orchestrator)

**Applied: 8 amendments** (7 proposed + 1 orchestrator-raised).
- arch ×1 — §Stack Role cell, 15 → 16 + `report-copy` enumerated + lineage rung.
- test-plan ×6 — §1 P5 · §2 invariants · §6 driver row · §6 P5 Status · §6 full-coverage · §9 E2E row.
- test-plan ×1 **orchestrator-raised, playbook-routine** (pre-existing doc claim falsified by reality,
  impl already correct, completes in one artifact): the §6 P5 Status bullet's enumeration was still the
  13-stage list (missing `native-menu-suppressed`, `signpost-repeat`) and still called `widget-close`
  "the TERMINAL stage" — a claim retired at 2026-08-24. The count edit alone would have left the bullet
  self-contradictory ("sixteen stages" over a 13-id list), so the enumeration and the terminal sentence
  were completed to reality in the same pass.

Bodies verified BEFORE the sidecar entries were written: arch 1 site, test-plan 6 sites, **0 residual
`15`-form claims in any of the seven masters**, historical lineage rungs preserved.

**Cascade (single pass, fixed DAG):**
1. Bodies applied (above); sidecars appended to `architecture-amendments.md` + `test-plan-amendments.md`.
2. Lateral binds — `test-plan §3 ↔ obs-plan §3` untouched (this chunk amended §1/§2/§6/§9, not §3);
   `a11y-plan ↔ obs-plan` schema untouched. Cross-master verbatim sweep across all seven: **0 residual.**
   Judgment bases (`playbook.md`, `drift-base.md`): **0 hits.**
3. Leaves re-derived: `.claude/docs/stack.md` (arch §Stack leaf) · `.claude/docs/tests-summary.md` +
   `.claude/rules/testing.md` ×2 sites (test-plan leaves). CLAUDE.md needed no edit — it states no stage
   count, and `GENERATED:setup:warnings` derives from §Anti-Patterns, which this chunk did not amend.

**Preserve-verbatim routing (cascade must NOT edit):**
- `.claude/rules/verification-harness.md:117` §Session Additions — "the leg is now FIFTEEN stages" is a
  CURRENT-state claim gone stale → **routed to P3 curation** as an in-place extension of that entry.
- `.claude/rules/verification-harness.md:125` §Session Additions — "the re-homed 15-stage arm ran 183s
  PASS" is a HISTORICAL measurement of that chunk's own run, accurate as written → **left verbatim**.
  (The discrimination matters: only the first goes stale when the count moves.)

**Escalations: 0. Drift = 0 at P2 exit.**
