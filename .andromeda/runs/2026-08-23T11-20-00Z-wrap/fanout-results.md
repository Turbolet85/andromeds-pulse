# Fan-out results — 2026-08-23-metrics-points-labels

7 doc-agents, one parallel batch. **9 proposals** across 4 docs; 3 docs clean.

| doc | verdict | proposals |
|---|---|---|
| architecture.md | drift | 1 (D-arch-resources) |
| security-plan.md | drift | 5 (D-security-logging — 2 primaries + 3 dependents) |
| design-system.md | **clean** | `proposals: []` |
| layout-templates.md | **clean** | `proposals: []` |
| test-plan.md | drift | 2 (D-tests-coverage — 1 primary + 1 dependent) |
| obs-plan.md | drift | 1 (D-obs-instrumentation) |
| a11y-plan.md | **clean** | `proposals: []` |

Raw twins written for the four proposal-carrying docs (`.raw-fanout-{doc}.md`). The three clean
returns are recorded here, which is their sanctioned audit artifact.

## Clean returns — the basis each cited

- **design-system** (`D-design-tokens`): both new surfaces carry `tokens n/a`; no `hardcoded✗` anywhere.
  Only webview files touched are three test fixtures + regenerated bindings — no component/render code.
  Noted (no detector binds it): design-system §Component Patterns → Loading / Empty States uses the literal
  "No metrics received yet" as its EmptyState example; the report's disproved-claim #1 discusses that surface
  only to falsify the working entry's NOTE, and asserts nothing wrong with the copy or tokens.
- **layout-templates** (`D-layout-surface`): no user-facing surface or region added; the only public change
  is a DTO field. Cited the report's measured basis that `MetricsChart.tsx:180-195` consumes only
  `ts_unix_nano` and `value`, so labels reach no rendered surface.
- **a11y-plan** (`D-a11y-surface`, `D-a11y-obs-schema`): no interactive element added; the report's
  Schema bullet carries the explicit negative "No violation-schema change." The owed obs amendment is §5
  (a metric-registry fact), not §6 (the structured-log schema the a11y violation JSON binds to), so the
  two stay matched.

## Orchestrator verification of the agents' absence claims (validate check 4)

Each "single-site" / "unrelated hit" claim was re-derived first-hand rather than taken:

- arch's "retired wording occurs only at §Conventions → Primary key convention" — **confirmed**;
  `grep 'attributes column' architecture.md` returns exactly line 85.
- The same wording DOES appear in `architecture-amendments.md` (a sidecar — append-only history, never
  edited) and `master-route.md` (the immutable record of the PREDECESSOR chunk, describing what that chunk
  did). Both correctly stay; neither is an amendment target.
- obs's "the other ALL FOUR hit is unrelated" — **confirmed**; `obs-plan.md:530` is
  `interpretation.incident.created` field-completeness, a different claim.
- security's three restating sites — **confirmed** at `security-plan.md:48` (§Threat Model sensitivity
  note), `:165` (§Data Protection → At rest), plus the count-correction paragraph in §Anti-Patterns →
  Logging alongside the primary at `:427`.
- Curation homes + judgment bases (`CLAUDE.md` USER block, `rules/*` Session Additions,
  `docs/session-learnings.md`, `playbook.md`, `drift-base.md`) — **no hits** for the retired wording, so
  nothing routes to the preserve-verbatim channel this wrap.

## Validation outcome

- **Playbook:** all 9 match the 2026-07-08 rule (accurate this-chunk addition/correction within an
  already-documented structure) → **routine-APPLY**. The three escalate-severity detectors
  (`D-security-input`, `D-security-deps`, `D-obs-pii`) all returned clean with cited bases.
- **Cross-contradiction:** none. Three security edits land in §Anti-Patterns → Logging but in different
  paragraphs and all move the same direction (four → five, plus disambiguation).
- **Intent-consistency:** the report's one divergence family (the working entry's NOTE falsified) is a
  JUSTIFIED divergence ⇒ intent was incomplete → already amended at phase P5 (`scope.md` §Premise
  correction). Disposed, not open.
- **Absence-needs-evidence:** every absence claim re-derived above.
- **Expected-amendments reconciliation (coverage floor):** all four plan entries covered — arch §Conventions
  (+ §Occupied Resources explicitly disposed as tables-not-columns, per the predecessor's ruling);
  security §Logging all three facts (count, label-KEY disposition, third scrub shape); obs §5 both halves
  (scope AND unit); test-plan §4. **No floor entry under-ran.**
- **Disproved-claims disposition:** both report entries disposed — #1 to `scope.md` (applied at phase P5),
  #2 to `plan.md` §Constraints & rejected approaches + routed to P3 curation.

**Escalations: 0.** No HALT required.

## A note on the fan-out transport

The security return carried HTML-escaped `&amp;` in one rationale (the same entity-escape class the
phase fan-out hit in 6 of 7 extracts). Decoded on read; it changed no proposal content.
