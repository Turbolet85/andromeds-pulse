# Fan-out results — 2026-08-30-npm-advisory-coverage wrap

7 doc-agents, one per spec source, each against `chunks/2026-08-30-npm-advisory-coverage/report.md`
+ its drift-base detectors. Raw twins saved for docs whose returns needed stripping or carried
proposals (`.raw-fanout-{doc}.md`).

| doc | proposals | verdict |
|---|---|---|
| architecture.md | 1 (D-arch-resources) | register `cargo xtask check:npm-supply-chain` + `npm-policy.json` in §Occupied Resources; D-arch-decisions clean (zero Rust deps; wdio 9.31.5 inside the 9.x row; frontend tooling delegated per arch:305) |
| security-plan.md | 4 (D-security-deps group: primary + dev-only correction + 2 `dependent-of` roster sites at §Dependency Security build-failure roster and §Bootstrap dep-security-ci-gate) | D-security-input/auth/logging clean (policy parser validated + unit-pinned; SHA-pinned setup-node; PII n/a; the 3 §Logging restating sites verified in agreement) |
| design-system.md | 0 | clean — all 10 halo-deferral status sites verified untouched; 0 npm-subject hits in the doc |
| layout-templates.md | 0 | clean — 12 halo restating sites hold; near-miss noted and correctly rejected: line 17 "shadcn/ui (Radix UI primitives)" stack prose is adjacent to the chunk's radix-family denylist but is NOT a status claim the chunk moved (pre-existing; surfaced to the handoff as an observation) |
| test-plan.md | 2 (D-tests-coverage §9 supply-chain stage row; D-tests-framework §1:56 `--list` config correction — grep confirmed single occurrence) | D-tests-obs-harness clean (no harness/status/log-format change) |
| obs-plan.md | 5 (D-obs-stack group: web-vitals demoted to unimplemented target-state mandate at §3 Frontend bridge + §1 Justification + §1 instrumentation-scope table + §1 telemetry-surfaces table + §4 per-surface table) | D-obs-instrumentation/pii/defect-narrative clean (xtask tool n/a; PII n/a; §10 defect narratives untouched — zero Rust deps) |
| a11y-plan.md | 0 | clean — no interactive element added, schema untouched; the §3 pin amendment correctly noted as outside both invariants (raised by the orchestrator per validate check 5, the plan's Expected-amendments floor) |

## Validation (main)
- Playbook: 11 proposals routine-APPLY — arch (2026-07-08 this-chunk-accurate-addition) · security ×4
  (2026-08-23 APPLY-BY-ACTUAL-CLASS: escalate-severity detector outside its guarded class — the report
  affirmatively shows no banned/unvetted dep, deny gates green, zero Rust deps; actual classes: this-chunk
  recording + 2026-08-14 doc-only measured correction; the `dependent-of` pair rides the primary) ·
  test-plan ×2 (this-chunk row; 2026-08-14 doc-only correction) — plus the orchestrator-raised a11y-plan §3
  pin amendment (check-5 floor; operator P4 ruling + measured installs substantiate).
- ESCALATED: the obs-plan D-obs-stack group (5 sites) — accurate measured annotation, but the open
  web-vitals mandate has NO owning route entry; the 2026-08-28 playbook rule's carve-out ("escalate if no
  owner exists anywhere in the route") governs. Resolved WITH the operator (see below).
- Cross-contradiction: none. Intent-consistency: report deviations justified, consistent with the working
  entry + plan. Absence-needs-evidence: web-vitals absence cites the 0-hit three-way grep; test-plan
  single-occurrence cites the agent's grep (1 hit, :56); arch duplicate sweep cites 0 further sites.
- Check 6 (disproved-claims): (1) dev-only channel claim → security proposal 2 · (2) web-vitals → the
  escalated group + operator ruling · (3) playwright --list → test-plan §1:56 proposal + the chunk-artifact
  report entry. All three DISPOSED.

## Escalation resolution
The web-vitals escalation was resolved WITH the operator: **RETIRE the mandate** (chosen over
keep-and-own and annotate-unowned). Applied stronger than the proposals: the 5 obs-plan sites now name
the hand-rolled TauRPC `telemetry.frontend.*` bridge as the DECIDED mechanism, with web-vitals recorded
considered-and-retired (measured never installed; page-load-shaped metrics; no producer/consumer ever).
No route entry minted. No new playbook rule proposed — the 2026-08-28 rule's carve-out routed this
correctly; one instance of the retire-arm is below the recurrence bar.

## Apply ledger
13 master body edits (arch 1 · security 3 · test 2 · a11y 7 incl. the orchestrator-raised pin amendment's
5 restating sites + 1 straggler · obs 5) + 5 sidecar entries + 15 cascade leaf/CLAUDE.md edits
(obs-summary, stack, rules/frontend, rules/observability ×frontend-bridge, a11y-summary ×3, commands.md ×2,
conventions.md procedure-list `record_web_vital` retired, rules/security CI line, rules/verification-harness
xtask list, CLAUDE.md workflow supply-chain line, rules/a11y Lighthouse pin). Closure sweep: 0 stale
web-vitals-as-live claims, 0 old pins outside sidecars/history.

## Judgment-base hit (dispositioned, no edit)
`playbook.md:44` mentions web-vitals inside the 2026-06-30 preventDefault-rule's rationale note — a
HISTORICAL enumeration of obs §4's targets at that rule's codification date, not a current-truth claim;
the rule's verdict logic does not depend on the list. Per the cascade's judgment-base routing, no direct
edit; recorded here.

## Handoff observations (pre-existing, outside this chunk — not amended)
layout-templates:17 "Components shadcn/ui (Radix UI primitives)" + :145 "shadcn `Table`" describe a
component stack that is measurably not shipped (react-aria-components is installed; the radix family is
absent from the lockfile and now DENYLISTED by npm-policy.json per a11y-plan §11). Pre-existing prose
outside every detector's scope and this chunk's Changes — flagged for the doc owner's next touch-up.
