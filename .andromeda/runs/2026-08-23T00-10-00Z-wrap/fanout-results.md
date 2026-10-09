# Fan-out results — 2026-08-22-log-records-identity

7 doc-agents, one per spec source. **4 proposals** (arch ×1, test-plan ×3 incl. 1 `dependent-of`); 5 docs clean.
Raw twins kept for the two docs that carried proposals (`.raw-fanout-arch.md`, `.raw-fanout-test-plan.md`);
this file is the sanctioned audit artifact for the five clean returns.

| doc | verdict | detectors evaluated |
|---|---|---|
| arch | **1 proposal** (D-arch-decisions, warning) | D-arch-resources clean · D-arch-decisions VIOLATED |
| security-plan | clean | D-security-input · D-security-auth · D-security-deps |
| design-system | clean | D-design-tokens |
| layout-templates | clean | D-layout-surface |
| test-plan | **3 proposals** (D-tests-coverage ×3, warning) | D-tests-coverage VIOLATED · D-tests-framework clean · D-tests-obs-harness clean |
| obs-plan | clean | D-obs-instrumentation · D-obs-stack · D-obs-pii |
| a11y-plan | clean | D-a11y-surface · D-a11y-obs-schema |

## Clean returns — basis

- **security-plan** — returned bare `proposals: []`. No new external-input surface (the new `pub fn` is
  in-crate; no IPC/HTTP/deserialized struct), no identity/crypto/secret handling touched, and the report's
  Dependencies bullet is "none added, none bumped" so the §Dependency Security ban list has nothing to test.
- **design-system** — all three new surfaces carry `tokens n/a`; zero `hardcoded✗`. The only UI-adjacent
  file is the regenerated bindings artifact, which renders nothing. D-design-tokens had no occurrence to
  evaluate.
- **layout-templates** — no user-facing surface or region added; no TauRPC procedure, so no new data feed
  reaches a webview view; `seq` deliberately kept off `SELECT_LOGS`, so the Logs table gains no column.
  Correctly declined to treat the doc's own baseline prose (unbuilt-Halo notes, Findings/Report windows)
  as this chunk's changes.
- **obs-plan** — returned bare `proposals: []`. Matches the plan's expectation; disposed as
  **checked-no-change**, not skipped: the module-boundary mandate (§10) is SATISFIED — the append boundary
  emits at ERROR on `duckdb.append` with `reject_reason` (`consumer.rs:78-83`, live-verified this chunk).
  No tick field was minted, so no three-site amendment and no allowlist leaf were owed.
- **a11y-plan** — no interactive element added (all three surfaces `a11y n/a`); the a11y violation schema
  and obs §6 are both untouched. Keyed on the report's *affirmative* record that the obs schema was not
  changed rather than inferring from silence.

## Validate outcome

- **Check 1 (playbook)** — all 4 proposals matched existing rules as **routine**: the arch one under
  2026-08-14 *routine-APPLY* (doc-only fix, impl already correct — the `name` component was inaccurate at
  HEAD independent of this chunk); the three test-plan ones under 2026-07-08 *routine-APPLY* (accurate
  this-chunk additions within an existing structure). **Zero proposal-level escalations.**
- **Check 2 (cross-contradiction)** — none. test-plan P1 and P3 both edit §1 but different rows, in the
  same direction; P2 edits §4.
- **Check 3 (intent-consistency)** — aligned. The one deviation (bindings regen, outside touchpoints) is
  justified and recorded in the report.
- **Check 4 (absence needs evidence)** — both absence-claims re-verified by the orchestrator, not taken:
  arch's within-doc sweep (only `:85` carries the identity claim; `:84`/`:203` enumerate table names only)
  and test-plan's (`buffer-redaction-counter-unit-coverage` occurs at `:125` §1 and `:331` §4 — the latter
  is exactly what the `dependent-of` proposal covers).
- **Check 5 (expected-amendments floor)** — all three plan entries accounted for: arch §Primary key
  convention → proposed; test-plan §Pending coverage triggers → proposed; obs-plan NONE → checked-no-change.
  Floor met, no orchestrator-raised additions needed.
- **Check 6 (disproved-claims disposition)** — all 6 report entries disposed: #1 → D-arch-decisions ·
  #2 → **Tier-3 curation** (operator-decided) · #3 and #4 → already applied as scope corrections at phase
  P3 · #5 → D-tests-coverage P3 · #6 → checked-no-change.

## Orchestrator findings beyond the detectors

1. **Cross-master citation** — `.claude/docs/conventions.md:32` carried the retired arch wording verbatim.
   The arch agent's sweep was correct but scoped to arch; only the orchestrator's cross-master grep found
   it. Fixed in the cascade. Root cause: the amendment-flow cascade table lists only `stack.md` (+ CLAUDE.md
   `GENERATED:setup:*`) as arch's leaves, omitting `conventions.md`, which distills §Conventions verbatim.
   → new playbook rule: enumerate arch leaves by provenance header, not by a hardcoded list.
2. **Preserve-verbatim hit** — `CLAUDE.md:151` (`USER:session-learnings`, the 2026-08-21 external-relay
   entry) encodes the now-falsified "a SECOND table at `:185` the relay did not name" reading. The cascade
   must NEVER edit that home, so per the amendment-flow rule it **routes to P3 curation** as an in-place
   extension of the stale entry.
3. **Bindings recurrence (D2 class)** — `capability-drift` was RED at HEAD from commit `70344d5`; four
   existing rule entries describe this exact defect. Resolved WITH the operator as a two-layer fix: a
   playbook ORDERING rule now + a markerless route entry for the mechanical xtask staged-copy assertion.

## Applied

- `architecture.md` §Conventions → Primary key convention (body + sidecar)
- `test-plan.md` §1 Pending coverage triggers ×2 + §4 buffer bullet (body + sidecar)
- `.claude/docs/conventions.md` §Primary keys (leaf re-derivation)
- `.andromeda/playbook.md` — 2 new rules (14 → 16)

**Drift = 0**: 4 proposals applied · 0 escalations open · cascade closed (leaves: `conventions.md` only —
`stack.md` distills §Stack and carries no §Conventions content; CLAUDE.md `GENERATED` blocks restate no PK
claim; `tests-summary.md` / `rules/testing.md` trigger lists are curated subsets that omit the sibling
`buffer` trigger too, so neither was owed an update).
