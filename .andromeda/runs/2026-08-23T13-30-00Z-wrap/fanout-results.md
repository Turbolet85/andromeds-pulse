# Fan-out results — 2026-08-23-webview-self-verify

7 Explore doc-agents, one per spec source, one parallel batch. Report was the sole input
(`andromeda-pulse-0.3.0/chunks/2026-08-23-webview-self-verify/report.md`).

## Verdicts

| doc | proposals | outcome |
|---|---|---|
| arch | 2 | both APPLIED (D-arch-resources env var · D-arch-decisions §Stack row) |
| security-plan | 5 (all `escalate`) | 4 APPLIED after escalation · **1 REJECTED** on Validate check 4 |
| design-system | 0 | clean — `proposals: []` |
| layout-templates | 0 | clean — `proposals: []` |
| test-plan | 7 | all APPLIED (5 D-tests-framework incl. 4 dependents · 2 D-tests-coverage triggers) |
| obs-plan | 0 | clean — `proposals: []` |
| a11y-plan | 2 | both APPLIED (D-a11y-surface primary + 1 dependent) |

**Totals: 16 proposed · 15 applied · 1 rejected · 0 open.** Coverage sanity: 3 of 7 clean — well
inside the pass band. Every return parsed cleanly with no stripping required, so no `.raw-fanout-*`
twins were warranted; this file is the sanctioned audit artifact for all seven.

## Validation

1. **Playbook** — routine dispositions matched existing rules (2026-06-28 env-var registration;
   2026-07-08 accurate-addition-within-existing-structure; 2026-08-14 doc-only-fix; 2026-08-15
   APPLY-AS-MEASURED). The security carve-out had no covering rule and was structural → escalated.
2. **Cross-contradiction** — none. arch's §Stack row and test-plan's §6 driver row describe the same
   stack consistently; the security carve-out and the arch registration are complementary.
3. **Intent-consistency** — report aligns with the working-route entry and the plan's
   acceptance criteria. No divergence.
4. **Absence needs evidence** — **1 FAILURE.** The security detector proposed amending §Bootstrap
   phases on the premise that no npm Dependabot ecosystem exists. `.github/dependabot.yml`
   demonstrably configures three: `cargo` (`/`), `github-actions` (`/`), **`npm` (`/pulse-app/ui`)**.
   Proposal REJECTED; the accurate residue (npm gets update PRs but no advisory/license/ban
   scanning, a gap pre-existing at 27 devDeps) was applied instead as a narrowed note.
   Two other absence claims were independently verified and held: `"Minimize to tray"` = 0
   occurrences; `mocha` / `@wdio/*` = absent from `package.json`.
5. **Expected-amendments reconciliation** — the plan's floor named 3 (test-plan §2/§6 · a11y-plan
   §1 P5 · arch §Occupied Resources). All 3 were proposed by detectors; none had to be
   orchestrator-raised. Floor met, no under-run.
6. **Disproved-claims disposition** — all 4 report entries disposed: (1) a11y phantom → applied;
   (2) test-plan §2:148/§6:497 → applied; (3) `verification-harness.md:110` → routed to **P3
   curation** (rule file, not a spec master); (4) plan.md's Conductor-precedent premise → recorded
   in the report + routed to **P3 curation**.

## Escalations (4, all resolved WITH the operator)

| # | subject | resolution |
|---|---|---|
| E1 | Narrowing the categorical `ANDROMEDA_PULSE_*_PATH` canonicalize-and-confine ban (3 sites) | **Apply the carve-out**, tightly scoped to harness-only / not-production-consumed / existence-guarded vars |
| E2 | npm-channel dependency gap, partly false | **Reject the false half, apply-as-measured** the verified residue + name an owning route entry |
| E3 | RED mutation arm is manual (operator directive 1) | **Trigger row + CARRY** on the Staged-bindings assertion entry |
| E4 | test-plan §9 CI stage names a runner that does not run | **Apply-as-measured** — correct invocation + state it is not yet wired |

No new `playbook.md` rule was proposed: E1/E2/E4 each resolved under an EXISTING rule once the
operator settled the structural question, and no pattern recurred within this pass.

## Cascade

- **Step 2 (cross-master + curation-home grep)** — 4 retired-wording sweeps run. Real hits found and
  folded into this pass: `test-plan.md:45` and `:91` both quote the security ban over *all*
  `ANDROMEDA_PULSE_*_PATH` vars → qualified to product-binary. `obs-plan.md:124` cites the same
  Vector but scopes it to the product's config-load path → **checked and correctly left unchanged**.
  One hit landed in a preserve-verbatim home (`rules/testing.md:167`, inside `## Session Additions`)
  → routed to P3 curation, never cascade-edited.
- **Step 3 (leaf re-derivation, enumerated by provenance not by the table)** — arch's leaves resolved
  to `{stack.md, conventions.md}`; `conventions.md` carried no amended claim, `stack.md` gained the
  harness entry. Also re-derived: `CLAUDE.md` warnings block · `.claude/rules/security.md` ·
  `.claude/docs/security-summary.md` · `.claude/rules/testing.md` (body only) ·
  `.claude/docs/tests-summary.md` · `.claude/rules/a11y.md` · `.claude/docs/a11y-summary.md`.
  `USER:*` and every `## Session Additions` block preserved verbatim.

**Drift = 0 at exit.**
