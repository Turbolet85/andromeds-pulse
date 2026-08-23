# Fan-out results — 2026-08-23-metrics-points-identity

7 Explore doc-agents, one per spec source, one parallel batch. Report-only inputs (no git, no codebase).

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 1 — §Conventions → Primary key convention |
| security-plan | clean | 0 (agent flagged that no detector covers §Security Anti-Patterns → Logging) |
| design-system | clean | 0 |
| layout-templates | clean | 0 |
| test-plan | drift | 6 — §1 trigger · §4 buffer bullet (dependent) · §3 item 6 · §7 list (dependent) · §5 driver (dependent) · §5 boundary row (dependent) |
| obs-plan | drift | 1 — §5 `redactions_applied` Counter row |
| a11y-plan | clean | 0 |

**Total proposed: 8** across 3 docs. **Orchestrator-raised: 2** (see Validate check 5).

## Validate — the six checks

1. **Playbook** — no rule matched any proposal as routine-REJECT; none of the six over-reach rules
   applies (every proposal concerns a fact THIS chunk changed or measured, not a pre-existing surface
   mis-attributed to it). All 8 staged routine. 2 items escalated by the orchestrator on other grounds
   (below), not by a playbook verdict.
2. **Cross-contradiction** — none. The 8 proposals touch 8 distinct sections across 3 docs.
3. **Intent-consistency** — the report diverges from the working-route entry in two ways (label half not
   delivered; redaction-fold fix added). Both are operator decisions taken at the phase P4 gate and were
   already written into `scope.md` §The design tension. Justified ⇒ intent was incomplete ⇒ already
   amended. No escalation.
4. **Absence needs evidence** — three proposals rest on absence claims; all three re-derived first-hand:
   - arch "single site" → verified: `metrics_points` at architecture.md:84/:203 and `metric_name` at :187
     carry no key wording. Confirmed.
   - obs "other two mentions still true" → verified: obs-plan.md:101 (tick field list) and :517 (`buffer`
     allowlist) assert only tick-membership and aggregate-only shape. Confirmed.
   - test-plan "MockMetricPoint / MockArrowBatch absent" → verified, **and the agent's claim CORRECTED**:
     it asserted "only the `MockTraceSpan` half of the mandate exists". Measured hit counts are
     `MockTraceSpan` **0**, `MockArrowBatch` **0**, `MockMetricPoint` **0** — item 6 is ENTIRELY
     unimplemented, not partially. The amendment text was rewritten accordingly before applying.
5. **Expected-amendments reconciliation** — the plan listed four. Two under-ran the detector set and were
   raised by the orchestrator:
   - **security-plan §Security Anti-Patterns → Logging** — NO proposal (no detector covers that section).
     Report substantiates it (Spec-claims item 2) ⇒ routine, raised and applied.
   - **test-plan §3 Direct-binary smoke variant** (GREEN-leg scoping of the 0-ERROR assertion) — NO
     proposal. Report substantiates it (the RED leg produced an ERROR by design) ⇒ routine, raised and
     applied.
   - arch §Occupied Resources ("may need the column noted") — **verified NOT needed**: that section
     enumerates table names, not columns. Recorded in the arch sidecar as a not-amended finding.
6. **Disproved-claims disposition** — all 4 report entries end disposed: arch:85 → arch proposal ·
   security-plan:429 → orchestrator-raised · obs-plan:356 → obs proposal · test-plan:270/:552 →
   test-plan group (operator-approved to all 4 sites + a new §1 trigger).

## Escalations — 2, both resolved WITH the operator

1. **Recurring uncovered drift class.** §Security Anti-Patterns → Logging needed amendment at THREE
   consecutive chunks (`2026-08-22-pii-scrubber-recall`, `2026-08-23-ingestion-scrub-coverage`, this one)
   and no detector covers it — each was caught only by the chunk plan's own expected-amendments list.
   → **Resolved: add `D-security-logging`** to `drift-base.md` (appended, with its rationale comment).
2. **Breadth of the builder-factory correction.** Four test-plan sites assert factories that do not exist,
   in a chunk that did not touch fixtures. → **Resolved: amend all 4 sites + mint a §1 pending trigger**
   `test-data-bootstrap-factories-unimplemented`, so zero false claims stand and the implementation work
   has an owner.

## Applied — 10 body edits, 4 sidecars, 1 detector

- `architecture.md` §Conventions → Primary key convention (1)
- `security-plan.md` §Security Anti-Patterns → Logging (1)
- `obs-plan.md` §5 Metric Coverage `redactions_applied` row (1)
- `test-plan.md` §1 trigger LANDED · §1 NEW trigger · §3 item 6 · §3 Direct-binary smoke · §4 buffer
  bullet · §5 driver · §5 boundary row · §7 list (7 edits + 1 new row)
- Sidecars appended: `architecture-` / `security-plan-` / `obs-plan-` / `test-plan-amendments.md`
- `drift-base.md`: `D-security-logging` appended (operator-approved)

## Cascade — closed

- **Lateral binds:** test-plan §3 ↔ obs-plan §3 — the smoke-variant edit touches the RED/GREEN assertion
  scoping, not the harness command set or log format; obs §3 unchanged, pair still consistent. a11y ↔ obs
  schema — the obs edit lands in §5 Metric Coverage, not §6 Log Coverage (line-range verified by the a11y
  agent); violation schema untouched.
- **Cross-master citation grep** (all seven masters + the three preserve-verbatim curation homes + the two
  judgment bases): `master-route.md:50` cites the retired LOUD wording but is IMMUTABLE historical record —
  correctly untouched. Both rules-file hits were confirmed to sit in GENERATED bodies (security.md:56 <
  Session Additions@87; testing.md:33 < @110), so they cascade rather than route to curation. No hit in
  `playbook.md` / `drift-base.md` / `docs/session-learnings.md` / CLAUDE.md `USER:session-learnings`.
- **Leaves re-derived (7):** `.claude/docs/conventions.md` · `.claude/rules/observability.md` ·
  `.claude/rules/security.md` · `.claude/rules/testing.md` · `.claude/docs/tests-summary.md` ·
  `.claude/docs/services/buffer.md` · `.claude/docs/services/viz.md`.
- **CLAUDE.md: no edit needed — verified, not assumed.** The `GENERATED:setup:warnings` block carries none
  of the retired wordings (all 11 warnings unrelated); the `security` module bullet names
  `metrics_points.metric_name` only as a scrubbed column (still true); `stack.md:40` enumerates reserved
  tables with no key wording.
- **Closure assertion:** a final grep for all five retired wordings across every current-truth artifact
  returns **0 stale claims**.

**drift = 0.**
