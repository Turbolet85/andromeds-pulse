# Fan-out results — 2026-08-15-corpus-key-persistence

All 7 doc-agents returned on the first spawn; no re-spawns, no strip/parse failures.

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 5 (4 D-arch-resources incl. 1 dependent · 1 D-arch-decisions) |
| security-plan | drift | 10 (D-security-input 2 · D-security-auth 6 · D-security-deps 2) |
| design-system | clean | `proposals: []` — every Coverage `tokens` flag is n/a; no UI surface |
| layout-templates | clean | `proposals: []` — no user-facing surface or region added |
| test-plan | drift | 4 (D-tests-coverage, incl. 2 dependents) |
| obs-plan | drift | 3 (D-obs-pii 2 · D-obs-instrumentation 1) |
| a11y-plan | clean | `proposals: []` |

**Total 22 proposals · 22 applied · 0 rejected as false positives.**

Validation: all 22 matched a playbook rule (2026-06-28 env-var registration · 2026-07-08 routine-APPLY
register-current-truth · 2026-08-14 doc-only-fix APPLY). The 6 `escalate`-severity proposals
(D-security-input ×2, D-security-deps ×2, D-obs-pii ×2) were downgraded to routine by an explicit rule
match, not by unease. Two operator escalations were raised by the ORCHESTRATOR rather than by a detector
(the external-decay playbook rule + the advisory-findings disposition) and resolved in one dialogue round.

Orchestrator-raised beyond the detectors (Validate check 5 — the plan's Expected-amendments floor, and
operator directive 7): the muted-diagnostic backlog for `incidents.list_active.request` /
`triage.incident.persist` / `triage.incident.corpus_restore`, recorded in obs-plan §8 as measured-not-fixed
with its owner named.

Detector counts of note: the security detector independently found the SAME six restating sites the
2026-08-14 amendment had corrected — a clean round-trip of that chunk's own cascade.
