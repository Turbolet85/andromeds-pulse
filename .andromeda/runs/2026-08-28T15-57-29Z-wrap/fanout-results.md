# Fan-out results — 2026-08-27-incident-persist-vs-resolve-write-race

7 doc-agents, one per spec source. **4 proposals across 3 docs; 4 docs clean.**

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 1 — `D-arch-decisions` (warning) |
| security-plan | clean | `proposals: []` |
| design-system | clean | `proposals: []` |
| layout-templates | clean | `proposals: []` |
| test-plan | drift | 2 — `D-tests-coverage` primary + its `dependent-of` duplicate |
| obs-plan | drift | 1 — `D-obs-pii` (escalate severity) |
| a11y-plan | clean | `proposals: []` |

Raw twins kept for the three docs carrying proposals: `.raw-fanout-arch.md` · `.raw-fanout-test-plan.md` ·
`.raw-fanout-obs-plan.md`. The four clean returns are recorded here (this file is their audit artifact).

## Clean returns — the reasoning each gave

- **security-plan** — all four detectors clean. `D-security-input`: both new surfaces marked validation
  `n/a` with reason (bound `?N` parameters, no interpolation), matching §Input Validation.
  `D-security-auth`: no identity/session/token/key handling; corpus key, keyring backends and the
  workspace-key flow untouched. `D-security-deps`: Dependencies "none added · none bumped",
  `deny bans licenses sources` exit 0, and the audit between-point record matches the §Dependency Security
  standing-deferral pin, which already names session 52 as next. `D-security-logging`: no DDL and no
  scrub-coverage change — the five persisted client-controlled cells, the `span_events.exception_type`
  exclusion, the `metrics_points.labels` over-redaction failure mode and the unencrypted-ring-buffer
  residual are all untouched; `declined_count` is an aggregate count on an existing target.
  **Grep-confirmed: security-plan carries no allowlist enumeration**, so the leaf 3 → 4 count is not
  restated there.
- **design-system** — both Coverage rows carry `tokens n/a`; no UI element, no `hardcoded✗` flag, so
  §Color Palette / §Spacing / §Typography are unchallenged.
- **layout-templates** — no user-facing surface or region added; both new surfaces self-declare
  `a11y n/a` / `tokens n/a`; zero `pulse-app/ui/**` delta.
- **a11y-plan** — no interactive UI element, so §5–§7 owe nothing. On the schema bind: the obs change is
  scoped to §8 (allowlist leaves), while a11y's violation schema binds to obs **§6** (the
  `timestamp`/`level`/`target`/`message`/`fields` envelope), which is untouched — a new field on an
  unrelated target is not an envelope change. Grep: 0 hits for `triage.incident.persist` /
  `declined_count` in a11y-plan.

## Validation (orchestrator) — 6 checks

1. **Playbook** — all 4 routine, **0 escalations**:
   - `D-arch-decisions` → **2026-07-08 routine-APPLY** (accurate, this-chunk, inside an existing section).
   - `D-obs-pii` → **2026-08-23 routine-APPLY-BY-ACTUAL-CLASS**. Both load-bearing conditions hold:
     (a) the escalate condition is affirmatively ABSENT — the report measures the field bounded non-PII
     ("aggregate count only — no incident id, scope_id, workspace, payload, SQL text, or bound values");
     (b) the actual class has its own disposition — a doc-only count correction the impl already ships →
     **2026-08-14 routine-APPLY** (is there an impl half? NO). Cross-checked against the 2026-08-16
     D-obs-pii rule's two conditions: the leaf enumerates EVERY field the emit site emits (asserted by
     set-equality, both directions), and its guard lives in `pulse-app/tests/` where it actually runs.
   - `D-tests-coverage` ×2 → **2026-08-15 routine-APPLY-AS-MEASURED**: an impl half exists (the
     cross-process assertion) but is owned by the pre-existing named trigger
     `mcp-incident-read-back-cross-process-coverage`; the amendment records measured truth AND names that
     owner, so the APPLY-AS-MEASURED conditions hold rather than the HANDOFF route.
2. **Cross-contradiction** — none. The four proposals touch four distinct sections
   (arch §Established Decisions · obs §8 · test-plan §1 · test-plan §6); no opposing pair.
3. **Intent-consistency** — report matches the working-route entry + plan acceptance criteria: the outcome
   ("a corpus resolution stays resolved") is delivered, the entry's VERIFY clause is satisfied live, and the
   folded PREREQ is discharged in its between-point form. All 4 deviations carry justifications. No
   unjustified divergence.
4. **Absence needs evidence** — every absence claim cites its search: arch's "no write-arbitration decision
   exists" (its own greps, independently matching the report's); obs's "sole occurrence at line 534" (with
   the two nearby sites examined and excluded by reason); tests' §6 line-482 restatement; security's
   "no allowlist enumeration (grep-confirmed)". None inferred from a partial view.
5. **Expected-amendments reconciliation** — the plan's list is the coverage floor and did NOT under-run:
   `architecture.md` §Established Decisions → proposed by arch ✓ · `obs-plan.md` §8 → proposed by obs ✓ ·
   the new working-route tail entry → not a spec master, owned by P5 route-resolve ✓. The two test-plan
   proposals are ADDITIONAL genuine drift beyond the plan's floor.
6. **Disproved-claims disposition** — all 3 report entries end DISPOSED, none silent:
   (1) the route entry's "two writers" framing → recorded in the report + already premise-corrected in
   `scope.md`; the route entry is not a spec master, so no amendment is owed and the record IS the
   disposition. (2) plan step 5's edit surface → chunk-artifact claim, report entry is the disposition.
   (3) the chunk's own vacuous wait condition → recorded; it CONFIRMS rather than contradicts obs-plan §8's
   standing note, and the un-muting is owned by the "Diagnostics un-muting + harness-truth sweep" entry.
   The `dependent-of` group (tests) validates and applies atomically.

**Escalations: 0. HALT not required.**
