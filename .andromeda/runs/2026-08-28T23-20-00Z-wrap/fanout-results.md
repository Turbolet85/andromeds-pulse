# Fan-out results — 2026-08-28-duplicate-span-replay-fails-loudly

7 doc-agents, one parallel batch. 17 proposals returned; 1 self-raised by the orchestrator under Validate check 5.

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 6 (3 primary + 3 `dependent-of`) |
| security-plan | drift | 3 (2 at escalate severity, 1 warning) |
| test-plan | drift | 8 (6 primary + 2 `dependent-of`) |
| obs-plan | `proposals: []` | 0 — **missed the amendment its own §10 defect 4 owed**; raised by the orchestrator (check 5) |
| design-system | `proposals: []` | 0 — clean; no UI rendered, every Coverage `tokens` flag `n/a` |
| layout-templates | `proposals: []` | 0 — clean; no user-facing surface added |
| a11y-plan | `proposals: []` | 0 — clean; no interactive element, violation schema untouched |

## Duplicate-occurrence sweep — verified first-hand before applying
The agents' `dependent-of` claims were re-derived by the orchestrator rather than trusted:
- `1.10500.x` in `architecture.md` → **3 sites** (`:20` §Stack · `:49` §Established Decisions [Database] · `:351` §Inherited Defaults) — matches arch's claim. Post-apply: 0 stale.
- `byte-identical` in `architecture.md` → **2 sites** (`:42` §Stack row · `:240` §Occupied Resources `PULSE_INJECTOR`) — matches.
- `whole-batch` in `security-plan.md` → **1 site** (`:434`) — matches; no dependent owed.
- `same hang rationale` / `documented hang` in `test-plan.md` → both at `:353` — matches.

## Validation outcomes

**Check 1 (playbook):** 15 routine, 2 escalated.
**Check 2 (cross-contradiction):** none — no two proposals edit one section in opposing directions.
**Check 3 (intent-consistency):** aligned. The report's 5 deviations are each justified and operator-directed or structurally necessary.
**Check 4 (absence needs evidence):** **fired once** — see the rejected proposal below.
**Check 5 (expected-amendments reconciliation):** the plan listed 6 expected amendments; detectors covered 5. **obs-plan §10 defect 4 was proposed by no detector** — the known blind class (none of the three obs detectors covers a defect narrative in a spec body; the predecessor chunk recorded the same gap). Orchestrator-raised as routine: the report substantiates it with the measured red→green.
**Check 6 (disproved-claims disposition):** all 3 disposed — #1 → test-plan §4 ×2 sites (+ the `schema.rs:320–323` source comment routed to route-resolve, since wrap does not edit source); #2 → obs-plan §10 defect 4 + the `rules/observability.md` cascade leaf; #3 → test-plan §1 + §4.

## Escalations (2) — resolved WITH the operator

**1. security-plan proposal 3 — REJECTED on measurement.**
Proposed rewriting §Security Anti-Patterns → Logging (`:434`) to say the "whole-batch ERROR was the only signal" claim "never held on this project's Arrow appender path", generalizing this chunk's **spans**-path hang measurement.
Re-derived first-hand: at commit `7608217` duckdb was the **same 1.10502**, and chunk `2026-08-23-metrics-points-identity` measured *one `duckdb.append` ERROR* on a **metrics_points** PK collision. So at one library version, `metrics_points` errored while `spans` hung. The proposal's central claim is **false**; the sentence it targets is accurate for the path it describes.
Disposition: routine-REJECT (2026-06-30 generalized over-reach rule — inaccurate proposal). The real failure-mode change is carried by the obs-plan §10 defect-4 amendment instead. The per-table divergence is recorded as a finding in both obs-plan §10 and `rules/observability.md`, explicitly as **mechanism never established** and moot at 1.10505.

**2. test-plan proposal 7 — APPLIED WITH A CARVE-OUT.**
Proposed promoting arm A to gate-grade and retiring the whole red-by-design framing *including* the never-weaken instruction. Operator ruling: promote, but **RETAIN never-weaken/never-`--expect-absent`** — the reason it existed (arm A is the only on-demand reproduction of this defect class) survives the fix; the guard is what keeps a future red diagnosed rather than hidden. Retired only the now-false "exits 1 by design", the TRUE-POSITIVE framing (which described the old red), and the discharged owner pointer.

## Applied (17)

- **architecture.md** ×5 sites: `--replay` registered (flag set 3 → 4) · byte-identity → budget-identity at 2 sites · duckdb requirement-plus-resolved at 3 sites.
- **obs-plan.md** ×2: §10 defect 4 OPEN → CLOSED with the twice-corrected mechanism · §10 lead-in re-stated.
- **security-plan.md** ×2: `ureq` build-only carve-out + the 3 pruned stale skips · session-54 between-point discharge.
- **test-plan.md** ×8: arm A gate-grade (never-weaken retained) · observe-window gap closed · §1 trigger discharged + its example-target note corrected · §4 buffer pin list extended · duplicate-INSERT-hang rationale retired at both sites.

## Cascade
Cross-master grep for retired wording across all 7 masters + the 3 preserve-verbatim curation homes + the 2 judgment bases: **no cross-master citation fixes owed** (the `test-plan:131` `byte-identical` hit is about capability-JSON after a revoke cycle, and `:360` about an env-var default — neither cites the injector claim).
Leaves re-derived, enumerated by provenance header rather than the reference table: `docs/stack.md` (arch §Stack) · `docs/obs-summary.md` + `rules/observability.md` (obs-plan) · `docs/tests-summary.md` + `rules/verification-harness.md` (test-plan) · `rules/security.md` (security-plan). `docs/conventions.md` carries arch §Conventions, which this chunk did not amend — not re-derived. All `## Session Additions` blocks and `USER:*` sections preserved verbatim; both cascade hits (`rules/verification-harness.md:87`, `docs/tests-summary.md:88`) were confirmed to sit in generated bodies, not curation homes.
