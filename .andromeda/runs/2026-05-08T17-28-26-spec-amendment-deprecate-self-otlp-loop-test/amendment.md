# Amendment Record — 2026-05-08T17-28-26-deprecate-self-otlp-loop-test

_Generated retroactively per spec-amendment-protocol.md Part A. Original
plan-edit landed in commit a7294d0 (2026-05-08T17:28:25Z) without a
synchronized marker file; this record closes the three-component contract
(Decisions Log entry + marker file + state.yaml registration)._

## Schema version: 1

## Identity

- **Amendment ID:** 2026-05-08T17-28-26-deprecate-self-otlp-loop-test
- **Trigger:** session 27 cross-plan rot reconciliation (out-of-band manual edit; not triggered by a chunk harness — flagged as D5 generic by wrap-session Phase 6 in handoff dated 2026-05-08T17:30:00Z)
- **Authority resolution:**
  - **Winning plan/tier:** obs-plan.md tier=Standard — `2026-05-02 Phase 3.5 pivot` eliminated the outbound OTel exporter from self-observation runtime; consequently no `ANDROMEDA_OBSERVER_URL`-shaped configuration surface exists
  - **Losing plan/concern:** test-plan.md §1 Test Scope Summary — coverage trigger `security-vector-coverage: No self-OTLP dialing` describes a negative test ("Configure product with OTLP exporter pointing to own `:4317` or `:4318`") that is now unimplementable because the configuration surface does not exist
  - **Rationale:** Skipping this trigger silently would lose the agent-driven invariant; documenting deprecation here keeps the audit trail intact. If a future scope re-introduces an outbound OTLP observer surface, this amendment should be revisited and the trigger un-deprecated.

## Plans amended

### Plan: `.andromeda/test-plan.md`
- **Sections:** §Test Decisions Log (append-only entry; §1 Coverage triggers table + §6 E2E P5 path bodies untouched — they retain DEPRECATED context)
- **Decisions Log entry:** "2026-05-08 — Deprecate self-OTLP-loop negative test (Section 1 coverage triggers + P5 critical path)"
- **Before → After:**
  - §1 trigger `security-vector-coverage: No self-OTLP dialing` semantics: implicit "test must exist asserting product refuses to dial own :4317/:4318" → explicit "trigger by-construction-satisfied per obs Phase 3.5 pivot; no exporter surface to misconfigure; mark DEPRECATED in next /andromeda-tests re-run"
  - §6 E2E P5 critical path "self-observation discipline" steps: implicit dependency on negative test → explicit "trigger no-op, /andromeda-implement should NOT generate dead test code"
  - test-plan.md Decisions Log: N entries → N+1 entries (append-only)

## Implementation files synced

- (N/A — amendment is deprecation of an unimplementable trigger; no test code was ever written for this trigger because the configuration surface it targets does not exist. The implementation-side P5 boot wiring per chunks #7 + #20 + #21 + #22 already enforces tracing-only self-observation by construction — no `OTEL_EXPORTER_OTLP_ENDPOINT` env var read, no `opentelemetry-otlp` crate in Cargo.lock.)

## Expected downstream propagation

(Hard-coded baseline per `delta-rerun-protocol.md` §plan→file mapping table,
plus project-specific overrides.)

- Tier 3: `.claude/docs/tests-summary.md` — refresh §1 Coverage triggers enumeration if it surfaces this trigger; mark DEPRECATED
- state.yaml: `plan_freshness.tests_mtime` — acknowledge mtime advance (already at 2026-05-08T16:47:59Z)
- CLAUDE.md: no change expected (CLAUDE.md does not enumerate test-plan §1 triggers)
- Tier 2: no change expected (`.claude/rules/testing.md` does not call out this specific trigger)
- Pending /andromeda-tests re-run: §1 Coverage triggers table should mark `security-vector-coverage: No self-OTLP dialing` as DEPRECATED with cross-reference to this amendment + obs-plan.md `2026-05-02 Phase 3.5 pivot`

## Lifecycle status

- [x] Applied 2026-05-08T17:28:25Z — via commit a7294d0 (chunk #27 commit; Decisions Log entry added out-of-band by user during cross-plan rot review)
- [ ] Noted (timestamp | null) — by next `/andromeda-wrap-session` (will be session 28)
- [x] Propagated 2026-05-08T18:43:08Z — by `/andromeda-setup-project --delta` (run-dir `.andromeda/runs/2026-05-08T18-43-08-setup-project-delta/`)
- [x] Archived 2026-05-08T20:15:15Z — by `/andromeda-wrap-session` (session 28)

(Retroactive marker note: see sibling marker `2026-05-08T17-28-25-reconcile-otel-stdout-references` for context on the retroactive marker creation.)

## Verification

- **Orphan grep:** `git grep -- "ANDROMEDA_OBSERVER_URL"` returned 0 matches in implementation code (only mentions in architecture.md §Established Decisions self-observation rule, which is documentation of the negative invariant, not a configuration variable). Self-observation is `tracing` ecosystem only, no outbound exporter envelope, no `OTEL_EXPORTER_OTLP_ENDPOINT` env var consumption.
  - Acceptable matches: architecture.md §Established Decisions [Self-Observation] — documents the negative invariant (the variable that must never exist as outbound observer config); test-plan.md §1 trigger description (the trigger this amendment deprecates).
  - Verified clean at: 2026-05-08T17:28:25Z (commit a7294d0)
  - Status: clean (deprecation amendment; no implementation code to remove because it never existed)

## Cross-references

- Specialist plan Decisions Log: `.andromeda/test-plan.md` §Test Decisions Log entry dated 2026-05-08 (line 890)
- Authority source: `.andromeda/obs-plan.md` §12 Obs Decisions Log entry dated 2026-05-02 ("User review (Phase 3.5, iteration 1) — pivot to tracing-only self-observation, drop OTel SDK from self-runtime")
- Related session-27 wave amendments (cross-plan rot reconciliation, all from commit a7294d0):
  - `2026-05-08T17-28-25-reconcile-otel-stdout-references` (security-plan.md — same Phase 3.5 pivot context, security-side cleanup)
  - `2026-05-08T17-28-27-document-pii-vector-test-gaps` (test-plan.md sibling, also from session 27 cross-plan rot wave)
  - `2026-05-08T17-28-29-document-capability-widening-test-gap` (test-plan.md sibling, also from session 27 cross-plan rot wave)
- Harness output: (N/A — no harness; out-of-band manual cross-plan review; trigger is by-construction-satisfied)
- state.yaml.spec_amendments[] entry: `2026-05-08T17-28-26Z-deprecate-self-otlp-loop-test` in active list
- session-handoff.md drift entry that this amendment retroactively closes: D5 ("test-plan.md regenerated since last setup-project — manual edit (2026-05-08 Decisions Log entry: deprecate self-OTLP-loop negative test per obs §3 pivot)") in `state.yaml.drift_warnings`
