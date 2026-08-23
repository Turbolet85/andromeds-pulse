# Fan-out results — 2026-08-23-integration-ux-e2e-test

7 doc-agents, one per spec source, run in one parallel batch against
`andromeda-pulse-0.3.0/chunks/2026-08-23-integration-ux-e2e-test/report.md`.

## Verdicts

| doc | proposals | verdict |
|---|---|---|
| **arch** | 2 (2 primary) | drift — §Stack harness Role cell stale; `PULSE_INJECTOR` unregistered |
| **security-plan** | 0 | clean — bare `proposals: []` |
| **design-system** | 0 | clean — no UI element added; both Coverage rows `tokens {n/a}`, never `hardcoded✗` |
| **layout-templates** | 0 | clean — bare `proposals: []` |
| **test-plan** | 10 (2 primary + 8 dependent) | drift — the one-press claim restated at 5 sites; "no DOM selectors" at 3 |
| **obs-plan** | 0 | clean — no product hot path, no dependency, no logging added; the leg READS the obs log |
| **a11y-plan** | 4 (1 primary + 3 dependent) | drift — the phantom Traces selector restated at 4 sites |

**Totals:** 16 detector proposals · **+2 raised by the orchestrator at validate check 5** · 18 applied · **0 escalations** · drift = 0.

Raw twins kept for the three proposal-carrying docs (`.raw-fanout-{arch,test-plan,a11y-plan}.md`).
`design-system` and `obs-plan` returned valid YAML followed by explanatory prose (stripped); their
reasoning is summarised above and carried no proposals, so per `amendment-flow.md` they ride this
consolidated record rather than a twin. `security-plan` and `layout-templates` returned bare
`proposals: []` with nothing to strip.

## Validation (the 6 named checks)

1. **Playbook check** — every proposal matched an existing rule as *routine*; none escalated.
   - arch §Stack row · test-plan §6 driver row + Selector strategy · the 8 test-plan dependents →
     **2026-07-08 routine-APPLY** (accurate this-chunk change inside an existing structure).
   - arch `PULSE_INJECTOR` → **2026-06-28 env-var registration** + **2026-08-14 routine-APPLY**
     (see *Widening* below).
   - all 4 a11y proposals → **2026-08-15 / 2026-08-17 routine-APPLY-AS-MEASURED** (an impl half
     exists but the operator boundaried it to the A11y verification entry at the /phase P4 dialogue;
     all three conditions met — records the gap, states it plainly, names the owner).
2. **Cross-contradiction** — none. The four §6 edits and the two §1 a11y-row edits touch distinct
   subsections in the same direction.
3. **Intent-consistency** — no divergence. The report matches the chunk's working-route entry and
   its plan acceptance-criteria; all 4 deviations carry justifications.
4. **Absence needs evidence** — one absence claim was checked rather than accepted: test-plan's
   "`resolve_msedgedriver` is still the one boundary with no unit test". **Verified** — 2 occurrences
   in `xtask/src/webview_drive.rs`, 0 inside `#[cfg(test)]`. The claim holds; the gap survived the
   module tripling from 9 to 27 tests.
5. **Expected-amendments reconciliation** — the plan's `Expected amendments (wrap)` list is the
   chunk's coverage floor, and **two entries no detector proposed** were raised by the orchestrator:
   - **test-plan §9 CI Integration** — the E2E stage named the bare command form; now carries
     `[--expect-absent <STAGE>] [--no-inject]` plus the CI prerequisites (node on the runner, the
     injector prebuilt) and still states plainly that the leg is not CI-wired.
   - **test-plan §1 new trigger `traces-surface-a11y-semantics-absent`** — the plan expected a
     test-plan trigger for the a11y selector gap; the a11y detector amended a11y-plan but no
     detector proposed the test-plan side.
   Both are substantiated by the report → routine, per check 5's own rule.
6. **Disproved-claims disposition** — all three report entries end disposed:
   - a11y-plan §1 P1 phantom selector → 4 amendments (applied as measured).
   - test-plan §6 one-press claim → 10 amendments.
   - `intent.md` F15 `inject_demo` "currently untracked" → **already disposed via the
     scope-correction channel** (`scope.md` §2, recorded at /phase P1). `intent.md` is an immutable
     artifact and is never a spec-amendment target — the report says so explicitly and the wrap
     honoured it rather than routing it here.

## Widening (one judgment worth flagging)

The arch detector proposed registering **`PULSE_INJECTOR` alone**. Measurement first: all four
xtask→node relay names — `PULSE_BIN`, `PULSE_DATA_DIR`, `MSEDGEDRIVER_PATH`, `PULSE_INJECTOR` —
had **0 occurrences** in `architecture.md`. Registering one of four would have left the registry
lopsided and actively misleading about a claimed `PULSE_*` prefix where a collision is silent. The
whole set was registered as one entry instead (playbook **2026-08-14 routine-APPLY**: doc-only fix,
impl already correct), with the sidecar recording that three are retroactive completeness from
`2026-08-23-webview-self-verify`.

## Cascade

- **Cross-master citation sweep** (all 7 masters, for each retired wording): 0 hits outside the
  amended masters themselves.
- **Preserve-verbatim homes checked, not edited:** `rules/verification-harness.md:116` (inside
  `## Session Additions`) understates the leg as "presses a real production control" — routed to
  **P3 curation** as an in-place extension, per the cascade rule. `rules/a11y.md:117` (also Session
  Additions) is a general HTML-AAM fact, not the retired claim — left alone.
- **Leaves re-derived by provenance** (playbook 2026-08-22 — enumerate by the generator's
  provenance header, not the reference's table): **8 edits across 5 files** —
  `docs/stack.md` (1) · `docs/tests-summary.md` (2) · `docs/a11y-summary.md` (1) ·
  `rules/testing.md` (2, generated body) · `rules/a11y.md` (2, generated body).
- `CLAUDE.md` `GENERATED:setup:warnings` checked — no harness/a11y-selector claim in it, so no
  re-derivation was needed. CLAUDE.md remains 153/200.
