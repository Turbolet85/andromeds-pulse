# Fan-out results — 2026-08-29-app-registry-reconciliation

7 doc-agents, one parallel batch. 6 proposals across 3 docs; 4 docs clean.

| doc | verdict | detail |
|---|---|---|
| arch | **1 proposal** | `D-arch-decisions` — close the §Established Decisions [Corpus Write Arbitration] *Accepted residual*. Grepped for every restatement of the retired wording; confirmed single-site, so no `dependent-of` owed. `D-arch-resources` did not fire (report negates every registry category). |
| security-plan | clean | all 4 detectors clean. Observed but correctly NOT proposed: §Input Validation's MCP row and §Threat Model Summary's MCP vector enumerate 4 tools (`query_traces`/`query_metrics`/`query_logs`/`generate_snapshot`) while 8 have shipped since chunk #94 — a PRE-EXISTING gap this chunk did not cause, outside the Changes-only rule. Carried to the handoff. |
| design-system | clean | no UI surface touched; all Coverage rows `tokens n/a`, zero `hardcoded✗`. |
| layout-templates | clean | no new user-facing surface or region. |
| test-plan | **4 proposals** | `D-tests-coverage` ×3 (trigger narrowing + its §6 P3 `dependent-of`; plus a NEW coverage trigger — **REJECTED**, see below) · `D-tests-obs-harness` ×1 (register the `smoke:external-resolve` scenario leg). |
| obs-plan | **1 proposal** | `D-obs-pii` — §8 Incident-path diagnostic leaves, `triage.incident.persist` 4 → 5 fields. `D-obs-defect-narrative` correctly did NOT fire: the closed residual lives in arch, not obs §10, and §10's DuckDB narrative + lead-in tally are untouched by this chunk. |
| a11y-plan | clean | no interactive element; the leaf change lands in obs §8, not the §6 envelope, so the violation-schema bind is intact. |

## Validation

| # | Proposal | Playbook rule | Disposition |
|---|---|---|---|
| 1 | arch — close the Accepted residual | 2026-07-08 accurate-this-chunk-correction; 2026-08-14 doc-claim-falsified-impl-correct | routine-APPLY |
| 2 | obs-plan §8 leaf 4 → 5 | 2026-08-16 (D-obs-pii leaf registration — both conditions hold) + 2026-08-23 apply-by-actual-class | routine-APPLY |
| 3 | test-plan §1 trigger narrowing | 2026-07-08 | routine-APPLY |
| 4 | test-plan §6 P3 residual (`dependent-of` #3) | applies atomically with its primary | routine-APPLY |
| 5 | test-plan §3 scenario-leg registration | 2026-07-08; `smoke:gap-resume` precedent | routine-APPLY |
| 6 | test-plan §1 NEW `durable-incidents-adapter-and-wiring-coverage` trigger | 2026-06-28 / 2026-06-30 generalized over-reach; Validate check 4 (absence needs evidence) | **ESCALATED → REJECTED with the operator** |
| 7 | security-plan §Dependency Security — session-55 probe | raised by the ORCHESTRATOR under Validate check 5 (no detector proposed it); a fact this wrap measured first-hand | routine-APPLY |

### Escalation #6 — resolved with the operator

The proposal's premise — *"the trait's 4 pins live in triage against a test double, so re-constructing a
second `Arc` or reverting the wiring leaves every test green"* — is **falsified at HEAD**. This chunk
strengthened `pulse-app/tests/integration_incident_write_guard.rs::an_externally_resolved_row_is_not_reverted_by_the_next_persist_cycle`
to drive the **real** `CorpusIncidentPersistence` as the durable source over a real in-memory corpus and to
assert `registry.list_active(WS).is_empty()` — which fails if the adapter returns wrong ids. The adapter half
is therefore pinned by a committed test, not by a double.

What genuinely remains unpinned is narrow: that `main.rs` derives both trait views from the SAME `Arc` rather
than two. Two `Arc`s over the same `Arc<dyn CorpusWriter>` are behaviourally identical, so no test could
distinguish them today and there is no measured defect to guard.

**Operator decision:** reject the trigger as proposed; record the narrow residual as a handoff note rather
than a §1 row — a trigger carrying a measurably false absence claim trains readers to distrust the list
(the 2026-08-23 wrong-file-absence lesson).

## Expected-amendments reconciliation (Validate check 5)

The plan's `Expected amendments (wrap)` list is this chunk's coverage floor. All four entries covered:

| Plan entry | Covered by |
|---|---|
| obs-plan §8 leaf 4 → 5 | proposal 2 |
| test-plan §1 trigger discharged · §6 P3 residual · §3 register the leg | proposals 3 / 4 / 5 |
| arch [Corpus Write Arbitration] Accepted residual closed | proposal 1 |
| security-plan §Dependency Security — session-55 probe result | proposal 7 (orchestrator-raised) |

## Disproved-claims disposition (Validate check 6)

| Report entry | Disposition |
|---|---|
| The plan's `item_count` acceptance half does not discriminate | **DISPOSED in the report itself** — a CHUNK-ARTIFACT claim, whose sanctioned home per the 2026-08-26 rule is the report bullet plus the corrected leg. No amendment owed; none pending. Stated inline so it does not read as unresolved. |
| arch [Corpus Write Arbitration] Accepted residual now closed | **DISPOSED by proposal 1** (amendment applied). |

Cross-contradiction check: none — no two proposals edit the same section in opposing directions.
Intent-consistency: the report matches the working-route entry + plan acceptance criteria, with deviation 1
justified and carried.
