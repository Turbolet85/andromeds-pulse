# Fan-out results — 2026-08-15-tier-1-incident-path-investigation

7 Explore doc-agents, one per spec source, run in one parallel batch against
`chunks/2026-08-15-tier-1-incident-path-investigation/report.md`.

| doc | verdict | proposals |
|---|---|---|
| arch | clean | `proposals: []` |
| security-plan | clean | `proposals: []` |
| design-system | clean | `proposals: []` |
| layout-templates | clean | `proposals: []` |
| a11y-plan | clean | `proposals: []` |
| **test-plan** | **2 proposals** | raw twin `.raw-fanout-test-plan.md` |
| **obs-plan** | **2 proposals** | raw twin `.raw-fanout-obs-plan.md` |

## Clean returns — the reasoning each gave

- **arch** — report's Symbols/APIs bullet is an explicit negative (no new fn / procedure / IPC /
  endpoint / port / env var); Dependencies none; Schema none. The three obs leaves are pre-existing
  emit sites un-muted via the allowlist, whose registry owner is obs-plan §8; the underlying
  `incidents.list_active` procedure is already registered at architecture.md §Occupied Resources.
- **security-plan** — the three leaves are outbound emit sites marked `validation n/a`, not input
  boundaries, so §Input Validation needs no row; the env vars used in the smoke are already
  enumerated there. No dependency added, so the §Dependency Security ban list has nothing to check.
  The keyless Python `sqlite3` read is an investigation method, and its result is exactly what
  §Data Protection predicts (plaintext table/column names, opaque BLOBs).
- **design-system** — invariant holds vacuously: no `.tsx` / `.css` / `pulse-app/ui` path in the
  Changes set, and all three new surfaces carry `tokens n/a`. No `hardcoded✗` anywhere.
- **layout-templates** — no new user-facing surface or region; the three Coverage entries are
  telemetry targets (`a11y n/a · tokens n/a`), not screens. Pre-existing doc gaps in that file are
  baseline state from earlier chunks, not drift introduced here.
- **a11y-plan** — no interactive UI element added; Schema/config none, and none of the three leaves
  is `a11y::assertion`, so the violation-schema ↔ obs-schema bind is untouched. Noted correctly that
  the malformed playwright `--prefix` line is the CHUNK PLAN's, not a11y-plan's — a11y-plan already
  carries the correct bare form.

## Validation outcome (main)

- Playbook: obs#1 + test#1 + test#2 → **routine-APPLY** (2026-07-08 / 2026-08-14 rules);
  obs#2 → **routine-APPLY-AS-MEASURED** (2026-08-15 rule) once its owner was named.
- Cross-contradiction: none — obs#1 and obs#2 are complementary by construction (obs#2 exists
  precisely so obs#1 does not leave §8 asserting a clean backlog).
- Intent-consistency: the report's divergence from intent (no fix; premise falsified) is JUSTIFIED
  by measurement, and intent was ALREADY amended at /andromeda-phase (scope.md premise-corrections
  at P3 + the P5 acceptance concretization). Nothing further to amend; the working-route entry is
  `[{marker}]`-frozen and stays as the historical record.
- Absence-needs-evidence: obs#2's five-target claim rests on a FULL redaction census over the whole
  log family (32,985 and 26,101 lines), not a partial view. ✓
- Expected-amendments reconciliation: the plan's floor was `obs-plan §8` — covered by obs#1. Its
  conditional "architecture.md and/or test-plan.md if a break is located" did not fire (no break);
  the two test-plan proposals are additional, detector-raised, and measured.
- Disproved-claims disposition (all three DISPOSED): (1) the working-route regression premise →
  scope.md premise-correction (applied at phase) + report + handoff; (2) the `cue.emit == 0`
  inference → scope.md closure + report + curation; (3) the LwwQueue CARRY → route-resolve P5.

## Escalation

One, resolved WITH the operator in a single round: obs#2 required naming the owner of the five
newly-measured muted targets, and the operator's wrap directive explicitly reserved that choice
(sweep entry vs extension) plus the owner of the `agent-run.sh status` pid defect. Both resolved to
a NEW dedicated "Diagnostics un-muting + harness-truth sweep" route entry, which P5 creates.

## Narrowing applied by main (validation check 4)

test#1 as proposed said pulse-app tests "must be run under `--workspace` … **never** with `-p`".
One measurement does not establish a universal, and `.claude/rules/testing.md` (a preserve-verbatim
curation home, 2026-05-31 session-153 entry) documents `cargo test --test X -p pulse-app` working.
Applied narrowed: match the scope of the run to the scope of the build. Left unnarrowed it would
have contradicted curated knowledge.
